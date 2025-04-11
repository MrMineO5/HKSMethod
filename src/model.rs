use crate::constants::{PI_4_2, PI_4_4};
use crate::stability::StabilityResult;

pub struct BetaFunctionValue {
    pub b1: f64,
    pub b2: f64
}
impl BetaFunctionValue {
    pub fn compute(&self) -> f64 {
        self.b1 / PI_4_2 + self.b2 / PI_4_4
    }
}

pub struct Couplings<const N: usize> {
    pub couplings: [f64; N]
}

pub struct TimeStep<const N: usize> {
    pub log_scale: f64,
    pub couplings: Couplings<N>,
}

pub trait Model<const N: usize> {
    fn beta_function(&self, couplings: &Couplings<N>) -> [BetaFunctionValue; N];
    fn stability_condition(&self, couplings: &Couplings<N>) -> StabilityResult;
}
