use std::{env, thread};
use hks_method::models::main_model::MainModel;
use hks_method::scanner::consumer::special_allowed_consumer::SpecialAllowedConsumer;
use hks_method::scanner::multi_threaded_scanner::MultiThreadedScanner;
use hks_method::simulation;

fn main() {
    let args: Vec<String> = env::args().collect();

    let point = [0.3, -0.3, 0.1, 0.0, 0.0, 0.0];
    let index_x = 5usize;
    let index_y = 6usize;

    println!("Setting up...");
    let mut coupling_ranges = [(0.0, 0.0); 7];
    coupling_ranges[0] = (0.425, 0.425);
    for i in 1..7 {
        if i == index_x || i == index_y {
            coupling_ranges[i] = (-0.5, 0.5);
        } else {
            coupling_ranges[i] = (point[i - 1], point[i - 1]);
        }
    }

    let num_samples = args[1].parse::<u64>().unwrap();
    let num_threads = thread::available_parallelism()
        .expect("Failed to get available parallelism")
        .get();

    let consumer: SpecialAllowedConsumer<7, 400, 400> = SpecialAllowedConsumer::new(coupling_ranges, index_x, index_y);

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

    // Create out directory
    std::fs::create_dir_all("out").expect("Failed to create output directory");

    let filename = format!("out/special_allowed_{}_{}.png", index_x, index_y);
    image.save_to_png(&filename).unwrap();

    println!("Done!");
}
