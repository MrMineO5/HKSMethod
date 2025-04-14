use crate::model::Couplings;
use crate::models::main_model::MainModel;
use crate::simulation::Integrator;

mod constants;
mod model;
mod models;
mod perturbativity;
mod polysolver;
mod simulation;
mod stability;

fn main() {
    let model = MainModel;

    let mut simulation = Integrator::new(
        simulation::IntegrationParameters {
            initial_scale: 1.22E19_f64.ln(),
            final_scale: 1.0E11_f64.ln(),
            num_steps: 1000000,
        },
        Box::new(model),
        Couplings {
            couplings: [0.425, 0.2, -0.3, 0.1, 0.1, 0.1, 0.1],
        },
    );

    // for n in 0..10000000 {
    simulation.perform_full_integration();
    // }
}
