use crate::model::{BetaFunctionValue, Couplings, Model};
use crate::stability::StabilityResult;

pub struct ToyModel;
impl Model<3> for ToyModel {
    fn beta_function(&self, couplings: &Couplings<3>) -> [BetaFunctionValue; 3] {
        f64::powi()
    }

    fn stability_condition(&self, couplings: &Couplings<3>) -> StabilityResult {
        todo!()
    }
}