use crate::model::{Couplings, Model, TimeStep};
use crate::perturbativity::check_perturbativity;
use crate::stability::StabilityResult;

pub struct IntegrationParameters {
    pub initial_scale: f64,
    pub final_scale: f64,
    pub num_steps: usize
}

enum IntegrationStepResult {
    Continue,
    Stability(StabilityResult),
    Perturbativity
}

pub struct Integrator<const N: usize> {
    pub params: IntegrationParameters,
    pub model: Box<dyn Model<N>>,
    pub time_step: TimeStep<N>
}
impl<const N: usize> Integrator<N> {
    fn perform_integration_step(&mut self) -> IntegrationStepResult {
        let beta_functions = self.model.beta_function(&self.time_step.couplings);

        let step_size = (self.params.final_scale - self.params.initial_scale) / self.params.num_steps as f64;

        // Update the couplings based on the beta functions
        for i in 0..N {
            self.time_step.couplings.couplings[i] += beta_functions[i].compute() * step_size;
        }
        self.time_step.log_scale += step_size;

        let stability_result = self.model.stability_condition(&self.time_step.couplings);
        let StabilityResult::Stable = stability_result else {
            return IntegrationStepResult::Stability(stability_result)
        };

        if !check_perturbativity(beta_functions, 0.1) {
            return IntegrationStepResult::Perturbativity;
        }

        IntegrationStepResult::Continue
    }
}
