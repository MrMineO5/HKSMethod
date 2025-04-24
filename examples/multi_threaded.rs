use std::{env, thread};
use MasterResearchProject::models::main_model::MainModel;
use MasterResearchProject::scanner::consumer::allowed_consumer::AllowedConsumer;
use MasterResearchProject::scanner::consumer::special_allowed_consumer::SpecialAllowedConsumer;
use MasterResearchProject::scanner::multi_threaded_scanner::MultiThreadedScanner;
use MasterResearchProject::scanner::scanner::Scanner;
use MasterResearchProject::simulation;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Setting up...");
    let coupling_ranges = [
        (0.425, 0.425),
        // (-1., 1.),
        // (-1., 1.),
        // (-1., 1.),
        // (-1., 1.),
        // (-1., 1.),
        // (-1., 1.),
        // (-0.5, 0.5),
        // (-0.5, 0.5),
        // (-0.5, 0.5),
        // (-0.5, 0.5),
        // (-0.5, 0.5),
        // (-0.5, 0.5),
        (0.3, 0.3),
        (-0.3, -0.3),
        (0.1, 0.1),
        (0.0, 0.0),
        (-0.5, 0.5),
        (-0.5, 0.5),
    ];

    let num_samples = args[1].parse::<u64>().unwrap();
    // let num_samples = 10000;
    let num_threads = thread::available_parallelism()
        .expect("Failed to get available parallelism")
        .get();

    let consumer: SpecialAllowedConsumer<7, 400, 400> = SpecialAllowedConsumer::new(coupling_ranges);

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

    let images = scanner.consumer.render();

    // Create out directory
    std::fs::create_dir_all("out").expect("Failed to create output directory");

    for i in 0..images.len() {
        for j in 0..images[i].len() {
            let filename = format!("out/allowed_{}_{}.png", i, j);
            images[i][j].save_to_png(&filename).unwrap();
        }
    }

    println!("Done!");
}
