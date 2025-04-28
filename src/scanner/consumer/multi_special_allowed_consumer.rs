use crate::model::Couplings;
use crate::scanner::consumer::ScanConsumer;
use crate::scanner::scanner::CouplingRanges;
use crate::simulation::IntegrationResult;
use crate::util::image::{boolean_layer, color_layer, Image, Layer};
use crate::util::stability::{FinalStabilityResult, StabilityResult};

const VEV_EPSILON: f64 = 1E-12;

#[derive(Clone)]
pub struct MultiSpecialAllowedConsumer<const N: usize, const NX: usize, const NY: usize> {
    broken_allowed: Box<Layer<u32, NX, NY>>,
    broken_disallowed: Box<Layer<bool, NX, NY>>,
    index_x: usize,
    index_y: usize,
}
impl<const N: usize, const NX: usize, const NY: usize> MultiSpecialAllowedConsumer<N, NX, NY> {
    pub fn new(ranges: CouplingRanges<N>, index_x: usize, index_y: usize) -> Self {
        let broken_allowed = Box::new(color_layer(ranges[index_x], ranges[index_y]));
        let broken_disallowed = Box::new(boolean_layer(ranges[index_x], ranges[index_y]));
        
        Self {
            broken_allowed,
            broken_disallowed,
            index_x,
            index_y,
        }
    }

    pub fn render(&self) -> Image<NX, NY> {
        let mut image = Image::new();
        image.draw_color_layer(&self.broken_allowed);
        image.draw_boolean_layer(&self.broken_disallowed, 0xFF0000);
        image
    }
}
impl<const N: usize, const NX: usize, const NY: usize> ScanConsumer<N> for MultiSpecialAllowedConsumer<N, NX, NY> {
    fn consume(&mut self, couplings: Couplings<N>, result: IntegrationResult) {
        if let IntegrationResult::Unbroken = result {
            return;
        }

        let couplings_ref = &couplings.couplings;

        match result {
            IntegrationResult::Broken(_, result) => {
                match result {
                    FinalStabilityResult::UnstableAllowed(stability_result) => {
                        match stability_result {
                            StabilityResult::Violated1(vev1) | StabilityResult::Violated2(vev1, _) => {
                                // We ignore the second vev in this case because it always consists of a +-
                                if vev1[0].abs() < VEV_EPSILON {
                                    self.broken_allowed.write(couplings_ref[self.index_x], couplings_ref[self.index_y], 0xFFFF00);
                                } else if vev1[1].abs() < VEV_EPSILON {
                                    self.broken_allowed.write(couplings_ref[self.index_x], couplings_ref[self.index_y], 0x0000FF);
                                } else if vev1[2].abs() < VEV_EPSILON {
                                    self.broken_allowed.write(couplings_ref[self.index_x], couplings_ref[self.index_y], 0x00FFFF);
                                } else if (vev1[1] - vev1[2]).abs() < VEV_EPSILON {
                                    self.broken_allowed.write(couplings_ref[self.index_x], couplings_ref[self.index_y], 0x000000);
                                } else {
                                    self.broken_allowed.write(couplings_ref[self.index_x], couplings_ref[self.index_y], 0x00FF00);
                                }
                            }
                            StabilityResult::Stable => {
                                panic!("Found stable stability result in broken integration result")
                            }
                            StabilityResult::ViolatedReqInit => {
                                panic!("Found stability violation with large polynomial root gap along RG flow")
                            }
                        };
                    }
                    FinalStabilityResult::UnstableDisallowed(_) => {
                        self.broken_disallowed.write(couplings_ref[self.index_x], couplings_ref[self.index_y], true);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    fn merge(&mut self, other: Self) {
        self.broken_allowed.merge(&other.broken_allowed);
        self.broken_disallowed.merge(&other.broken_disallowed);
    }
}
