use crate::model::Couplings;
use crate::models::main_model::MainModel;
use crate::scanner::scanner::Scanner;
use crate::simulation::Integrator;

mod model;
mod models;
mod simulation;
mod util;
mod scanner;

fn main() {
    let model = MainModel;

    let mut scanner = Scanner::new(
        [
            (0.425, 0.425),
            (0.1, 0.2),
            (-0.2, -0.3),
            (0.0, 0.2),
            (0.0, 0.2),
            (0.0, 0.2),
            (0.0, 0.2)
        ],
        simulation::IntegrationParameters {
            initial_scale: 1.22E19_f64.ln(),
            final_scale: 1.0E11_f64.ln(),
            num_steps: 1000000,
        },
        Box::new(model)
    );

    scanner.scan(100)
}
