use crate::model::BetaFunctionValue;
use crate::util::constants::{PI_4_2, PI_4_4};

pub fn check_perturbativity<const N: usize>(funs: [BetaFunctionValue; N], threshold: f64) -> bool {
    let mut sum_b1 = 0.;
    let mut sum_b2 = 0.;
    for i in 0..N {
        sum_b1 += funs[i].b1.abs();
        sum_b2 += funs[i].b2.abs();
    }
    sum_b2 / PI_4_4 < threshold * sum_b1 / PI_4_2
}