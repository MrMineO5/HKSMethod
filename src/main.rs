use crate::model::Couplings;
use crate::models::toy_model::ToyModel;
use crate::simulation::Integrator;

mod model;
mod models;
mod perturbativity;
mod polysolver;
mod stability;
mod simulation;
mod constants;

fn main() {
    let model = ToyModel;

    let mut simulation = Integrator::new(
        simulation::IntegrationParameters {
            initial_scale: 1.22E19_f64.ln(),
            final_scale: 1.0E11_f64.ln(),
            num_steps: 1000000,
        },
        Box::new(model),
        Couplings {
            couplings: [0.425, 0.2, -0.3]
        },
    );

    simulation.perform_full_integration();
}
