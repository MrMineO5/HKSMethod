use std::{env, thread};
use master_research_project::models::main_model::MainModel;
use master_research_project::scanner::consumer::multi_special_allowed_consumer::MultiSpecialAllowedConsumer;
use master_research_project::scanner::multi_threaded_scanner::MultiThreadedScanner;
use master_research_project::simulation;

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

    // Iterate through all unique combinations of indices
    for index_x in 1..7 {
        for index_y in (index_x + 1)..7 {
            println!("Processing plane with indices {} and {}", index_x, index_y);
            
            let mut coupling_ranges = [(0.0, 0.0); 7];
            coupling_ranges[0] = (0.425, 0.425);
            
            for i in 1..7 {
                if i == index_x || i == index_y {
                    coupling_ranges[i] = (-0.5, 0.5);
                } else {
                    coupling_ranges[i] = (point[i - 1], point[i - 1]);
                }
            }

            let consumer: MultiSpecialAllowedConsumer<7, 400, 400> = 
                MultiSpecialAllowedConsumer::new(coupling_ranges, index_x, index_y);

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

            let image = scanner.consumer.render();
            let filename = format!("out/multi_special_allowed_{}_{}.png", index_x, index_y);
            image.save_to_png(&filename).unwrap();
            
            println!("Completed plane {}-{}", index_x, index_y);
        }
    }

    println!("All planes processed successfully!");
} 