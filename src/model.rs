use crate::stability::StabilityResult;

pub struct BetaFunctionValue {
    pub b1: f64,
    pub b2: f64
}

pub struct Couplings<const N: usize> {
    pub couplings: [f64; N]
}

trait Model<const N: usize> {
    fn beta_function(&self, couplings: &Couplings<N>) -> [BetaFunctionValue; N];
    fn stability_condition(&self, couplings: &Couplings<N>) -> StabilityResult;
}
