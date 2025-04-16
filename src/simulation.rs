use crate::model::{Couplings, Model, TimeStep};
use crate::util::perturbativity::check_perturbativity;
use crate::util::stability::FinalStabilityResult;

pub struct IntegrationParameters {
    pub initial_scale: f64,
    pub final_scale: f64,
    pub num_steps: usize,
}

enum IntegrationStepResult {
    Continue,
    Stability(FinalStabilityResult),
    Perturbativity,
}

pub enum IntegrationResult {
    Unbroken,
    InitiallyUnstable,
    PerturbativityViolated(f64),
    Broken(f64, FinalStabilityResult)
}

pub struct Integrator<const N: usize> {
    pub params: IntegrationParameters,
    pub model: Box<dyn Model<N>>,
    pub time_step: TimeStep<N>,
}
impl<const N: usize> Integrator<N> {
    pub fn new(
        params: IntegrationParameters,
        model: Box<dyn Model<N>>,
        initial_couplings: Couplings<N>,
    ) -> Self {
        let initial_scale = params.initial_scale;
        Self {
            params,
            model,
            time_step: TimeStep {
                log_scale: initial_scale,
                couplings: initial_couplings,
            },
        }
    }

    fn perform_integration_step(&mut self) -> IntegrationStepResult {
        let beta_functions = self.model.beta_function(&self.time_step.couplings);

        let step_size =
            (self.params.final_scale - self.params.initial_scale) / self.params.num_steps as f64;

        // Update the couplings based on the beta functions
        for i in 0..N {
            self.time_step.couplings.couplings[i] += beta_functions[i].compute() * step_size;
        }
        self.time_step.log_scale += step_size;

        let stability_result = self.model.stability_condition(&self.time_step.couplings);
        let FinalStabilityResult::Stable = stability_result else {
            return IntegrationStepResult::Stability(stability_result);
        };

        if !check_perturbativity(beta_functions, 0.1) {
            return IntegrationStepResult::Perturbativity;
        }

        IntegrationStepResult::Continue
    }

    pub fn perform_full_integration(&mut self) -> IntegrationResult {
        for i in 0..self.params.num_steps {
            match self.perform_integration_step() {
                IntegrationStepResult::Continue => {}
                IntegrationStepResult::Stability(result) => {
                    return if i == 0 {
                        IntegrationResult::InitiallyUnstable
                    } else {
                        IntegrationResult::Broken(self.time_step.log_scale, result)
                    };
                }
                IntegrationStepResult::Perturbativity => {
                    return IntegrationResult::PerturbativityViolated(self.time_step.log_scale);
                }
            }
        }
        IntegrationResult::Unbroken
    }
    
    pub fn reset(&mut self, initial_couplings: &Couplings<N>) {
        self.time_step.couplings.couplings = initial_couplings.couplings;
        self.time_step.log_scale = self.params.initial_scale;
    }
}
