use crate::constants::{PI_4_2, PI_4_4, PI_4_6};
use crate::stability::FinalStabilityResult;

#[derive(Debug)]
pub struct BetaFunctionValue {
    pub b1: f64,
    pub b2: f64,
    pub b3: f64,
}
impl BetaFunctionValue {
    pub fn compute(&self) -> f64 {
        self.b1 / PI_4_2 + self.b2 / PI_4_4 + self.b3 / PI_4_6
    }
}

pub struct Couplings<const N: usize> {
    pub couplings: [f64; N],
}

pub struct TimeStep<const N: usize> {
    pub log_scale: f64,
    pub couplings: Couplings<N>,
}

pub trait Model<const N: usize> {
    fn beta_function(&self, couplings: &Couplings<N>) -> [BetaFunctionValue; N];
    fn stability_condition(&self, couplings: &Couplings<N>) -> FinalStabilityResult;
}
