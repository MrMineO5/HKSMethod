use std::fs::File;
use std::io::{BufWriter, Write};
use std::{env, thread};
use master_research_project::models::main_model::MainModel;
use master_research_project::scanner::consumer::breaking_scale_consumer::BreakingScaleConsumer;
use master_research_project::scanner::multi_threaded_scanner::MultiThreadedScanner;
use master_research_project::simulation;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Setting up...");
    let coupling_ranges = [
        (0.425, 0.425),
        (0.3, 0.3),
        (-0.3, -0.3),
        (0.1, 0.1),
        (0.0, 0.0),
        (-0.5, 0.5),
        (-0.5, 0.5),
    ];

    let num_samples = args[1].parse::<u64>().unwrap();
    let num_threads = thread::available_parallelism()
        .expect("Failed to get available parallelism")
        .get();

    let consumer: BreakingScaleConsumer<7, 400, 400> = BreakingScaleConsumer::new(coupling_ranges, 5, 6);

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

    let (image, min, max) = scanner.consumer.render();

    // Create out directory
    std::fs::create_dir_all("out").expect("Failed to create output directory");

    let filename = format!("out/scale_{}_{}.png", 5, 6);
    image.save_to_png(&filename).unwrap();
    let filename = format!("out/scale_{}_{}.txt", 5, 6);
    let file = File::create(filename).unwrap();
    let mut writer = BufWriter::new(file);
    write!(writer, "{} {}", min, max).unwrap();

    println!("Done!");
}
