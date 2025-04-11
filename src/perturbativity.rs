use crate::model::BetaFunctionValue;

const PI_4_2: f64 = std::f64::consts::PI * std::f64::consts::PI * 16.0;
const PI_4_4: f64 = PI_4_2 * PI_4_2;

pub fn check_perturbativity<const N: usize>(funs: [BetaFunctionValue; N], threshold: f64) -> bool {
    let mut sum_b1 = 0.;
    let mut sum_b2 = 0.;
    for i in 0..N {
        sum_b1 += funs[i].b1;
        sum_b2 += funs[i].b2;
    }
    sum_b1 / PI_4_2 < threshold * sum_b1 / PI_4_4
}