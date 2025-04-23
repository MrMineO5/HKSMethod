use std::f64::EPSILON;
use crate::model::Couplings;
use crate::scanner::consumer::ScanConsumer;
use crate::scanner::scanner::CouplingRanges;
use crate::simulation::IntegrationResult;
use crate::util::image::{boolean_layer, count_layer, Image, Layer};
use crate::util::stability::{FinalStabilityResult, StabilityResult};

const VEV_EPSILON: f64 = 1E-12;

#[derive(Clone)]
pub struct SpecialAllowedConsumer<const N: usize, const NX: usize, const NY: usize> {
    broken_allowed: Vec<Layer<bool, NX, NY>>,
    broken_super: Vec<Layer<bool, NX, NY>>,
}
impl<const N: usize, const NX: usize, const NY: usize> SpecialAllowedConsumer<N, NX, NY> {
    pub fn new(ranges: CouplingRanges<N>) -> Self {
        let mut matrix = vec![boolean_layer(ranges[0], ranges[0]); N*N];
        for i in 0..N {
            for j in 0..N {
                let range_x = ranges[i];
                let range_y = ranges[j];
                matrix[i * N + j] = boolean_layer(range_x, range_y);
            }
        }

        let broken_allowed = matrix.clone();
        let broken_super = matrix;

        Self {
            broken_allowed,
            broken_super,
        }
    }

    pub fn render(&self) -> Vec<Vec<Image<NX, NY>>> {
        let mut images = vec![vec![Image::new(); N]; N];
        for i in 0..N {
            for j in 0..N {
                images[i][j].draw_boolean_layer(&self.broken_allowed[i*N+j], 0x00FF00);
                images[i][j].draw_boolean_layer(&self.broken_super[i*N+j], 0x0000FF);
            }
        }
        images
    }
}
impl<const N: usize, const NX: usize, const NY: usize> ScanConsumer<N> for SpecialAllowedConsumer<N, NX, NY> {
    fn consume(&mut self, couplings: Couplings<N>, result: IntegrationResult) {
        if let IntegrationResult::Unbroken = result {
            return;
        }

        let couplings_ref = &couplings.couplings;

        match result {
            IntegrationResult::Broken(_, result) => {
                match result {
                    FinalStabilityResult::UnstableAllowed(stability_result) => {
                        let supergroup = match stability_result {
                            StabilityResult::Violated1(vev1) | StabilityResult::Violated2(vev1, _) => {
                                vev1[0].abs() < VEV_EPSILON || 
                                    vev1[1].abs() < VEV_EPSILON || 
                                    vev1[2].abs() < VEV_EPSILON ||
                                    (vev1[1] - vev1[2]).abs() < VEV_EPSILON
                            }
                            StabilityResult::Stable => {
                                panic!("Found stable stability result in broken integration result")
                            }
                            StabilityResult::ViolatedReqInit => {
                                panic!("Found stability violation with large polynomial root gap along RG flow")
                            }
                        };
                        
                        if supergroup {
                            for i in 0..N {
                                for j in 0..N {
                                    let index = i * N + j;
                                    self.broken_super[index].write(couplings_ref[i], couplings_ref[j], true);
                                }
                            }
                        } else {
                            for i in 0..N {
                                for j in 0..N {
                                    let index = i * N + j;
                                    self.broken_allowed[index].write(couplings_ref[i], couplings_ref[j], true);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    fn merge(&mut self, other: Self) {
        for i in 0..N {
            for j in 0..N {
                let index = i * N + j;
                self.broken_allowed[index].merge(&other.broken_allowed[index]);
                self.broken_super[index].merge(&other.broken_super[index]);
            }
        }
    }
}
