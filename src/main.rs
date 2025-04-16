use std::thread;
use crate::models::main_model::MainModel;
use crate::scanner::consumer::stability_consumer::StabilityConsumer;
use crate::scanner::scanner::Scanner;
use std::time::Instant;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use crate::model::Couplings;
use crate::scanner::consumer::print_consumer::PrintConsumer;
use crate::scanner::consumer::ScanConsumer;
use crate::scanner::consumer::threading_consumer::SendConsumer;
use crate::simulation::IntegrationResult;

mod model;
mod models;
mod scanner;
mod simulation;
mod util;

fn main() {
    println!("Setting up...");
    let coupling_ranges = [
        (0.425, 0.425),
        (-2., 2.),
        (-2., 2.),
        (-2., 2.),
        (-2., 2.),
        (-2., 2.),
        (-2., 2.),
    ];
    
    let num_threads = thread::available_parallelism().expect("Failed to get available parallelism").get() as u64;
    let num_samples = 10000000000u64;
    let total_samples = num_threads * num_samples;

    println!("Starting {} simulation threads with {} samples each", num_threads, num_samples);

    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
        .unwrap()
        .progress_chars("##-");
    
    let join_handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let pb = m.add(ProgressBar::new(num_samples));
            pb.set_style(sty.clone());
            let model = MainModel;
            thread::spawn(move || {
                let mut send_consumer: StabilityConsumer<7, 400, 400> = StabilityConsumer::new(coupling_ranges);
                
                let mut scanner = Scanner::new(
                    coupling_ranges,
                    simulation::IntegrationParameters {
                        initial_scale: 1.22E19_f64.ln(),
                        final_scale: 1.0E11_f64.ln(),
                        num_steps: 1000000,
                    },
                    Box::new(model),
                );

                scanner.scan(num_samples, &mut send_consumer, pb);
                
                send_consumer
            })
        })
        .collect();

    println!("Simulating...");

    let mut merge_consumer: StabilityConsumer<7, 400, 400> = StabilityConsumer::new(coupling_ranges);

    for handle in join_handles {
        let consumer = handle.join().unwrap();
        merge_consumer.merge(consumer);
    }
    
    let images = merge_consumer.render();
    
    for i in 0..images.len() {
        for j in 0..images[i].len() {
            let filename = format!("out/stability_{}_{}.png", i, j);
            images[i][j].save_to_png(&filename).unwrap();
        }
    }

    println!("Done!");
}
