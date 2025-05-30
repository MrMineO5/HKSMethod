use std::fs::File;
use std::io::{BufWriter, Write};
use std::{env, thread};
use hks_method::models::main_model::MainModel;
use hks_method::scanner::consumer::breaking_scale_consumer::BreakingScaleConsumer;
use hks_method::scanner::multi_threaded_scanner::MultiThreadedScanner;
use hks_method::simulation;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_samples = args[1].parse::<u64>().unwrap();
    
    let point = [0.3, -0.3, 0.1, 0.0, 0.1, -0.05];
    let num_threads = thread::available_parallelism()
        .expect("Failed to get available parallelism")
        .get();

    println!("Setting up...");
    
    // Create out directory
    std::fs::create_dir_all("out").expect("Failed to create output directory");

    // First pass: Collect all scanners and find global min/max
    let mut scanners = Vec::new();
    let mut global_min = f64::INFINITY;
    let mut global_max = f64::NEG_INFINITY;

    // First pass to compute all planes and find global min/max
    for index_x in 1..7 {
        for index_y in (index_x + 1)..7 {
            println!("Processing scale gradient for plane with indices {} and {}", index_x, index_y);
            
            let mut coupling_ranges = [(0.0, 0.0); 7];
            coupling_ranges[0] = (0.425, 0.425);
            
            for i in 1..7 {
                if i == index_x || i == index_y {
                    coupling_ranges[i] = (-0.5, 0.5);
                } else {
                    coupling_ranges[i] = (point[i - 1], point[i - 1]);
                }
            }

            let consumer: BreakingScaleConsumer<7, 400, 400> = 
                BreakingScaleConsumer::new(coupling_ranges, index_x, index_y);

            let mut scanner = MultiThreadedScanner::new(
                coupling_ranges,
                simulation::IntegrationParameters {
                    initial_scale: 1.22E19_f64.ln(),
                    final_scale: 1.0E11_f64.ln(),
                    num_steps: 1000000,
                },
                MainModel,
                consumer
            );

            scanner.scan(num_threads, num_samples);
            
            let (_, min, max) = scanner.consumer.render();
            global_min = global_min.min(min);
            global_max = global_max.max(max);
            
            scanners.push(scanner);
            
            println!("Completed scanning plane {}-{}", index_x, index_y);
        }
    }

    // Second pass: Render all planes with global min/max
    println!("Rendering all planes with global min={} max={}", global_min, global_max);
    
    let mut plane_index = 0;
    for index_x in 1..7 {
        for index_y in (index_x + 1)..7 {
            let scanner = &scanners[plane_index];
            let image = scanner.consumer.render_with_range(global_min, global_max);
            
            let filename = format!("out/scale_{}_{}.png", index_x, index_y);
            image.save_to_png(&filename).unwrap();
            
            let filename = format!("out/scale_{}_{}.txt", index_x, index_y);
            let file = File::create(filename).unwrap();
            let mut writer = BufWriter::new(file);
            write!(writer, "{} {}", global_min, global_max).unwrap();
            
            plane_index += 1;
            println!("Rendered plane {}-{}", index_x, index_y);
        }
    }

    println!("All scale gradient planes processed successfully!");
} 