use crate::model::Couplings;
use crate::scanner::consumer::ScanConsumer;
use crate::scanner::scanner::CouplingRanges;
use crate::simulation::IntegrationResult;
use crate::util::image::{count_layer, Image, Layer};
use crate::util::stability::FinalStabilityResult;

#[derive(Clone)]
pub struct AllowedConsumer<const N: usize, const NX: usize, const NY: usize> {
    broken_allowed: Vec<Layer<i64, NX, NY>>,
    broken_disallowed: Vec<Layer<i64, NX, NY>>,
}
impl<const N: usize, const NX: usize, const NY: usize> AllowedConsumer<N, NX, NY> {
    pub fn new(ranges: CouplingRanges<N>) -> Self {
        let mut matrix = vec![count_layer(ranges[0], ranges[0]); N*N];
        for i in 0..N {
            for j in 0..N {
                let range_x = ranges[i];
                let range_y = ranges[j];
                matrix[i * N + j] = count_layer(range_x, range_y);
            }
        }

        let broken_allowed = matrix.clone();
        let broken_disallowed = matrix;

        Self {
            broken_allowed,
            broken_disallowed,
        }
    }

    pub fn render(&self) -> Vec<Vec<Image<NX, NY>>> {
        let mut images = vec![vec![Image::new(); N]; N];
        for i in 0..N {
            for j in 0..N {
                images[i][j].draw_count_layers(&self.broken_allowed[i*N+j], &self.broken_disallowed[i*N+j], 0x00FF00, 0xFFFF00, 0xFF0000);
            }
        }
        images
    }
}
impl<const N: usize, const NX: usize, const NY: usize> ScanConsumer<N> for AllowedConsumer<N, NX, NY> {
    fn consume(&mut self, couplings: Couplings<N>, result: IntegrationResult) {
        if let IntegrationResult::Unbroken = result {
            return;
        }

        let couplings_ref = &couplings.couplings;

        match result {
            IntegrationResult::Broken(_, result) => {
                match result {
                    FinalStabilityResult::UnstableAllowed(_) => {
                        for i in 0..N {
                            for j in 0..N {
                                let index = i * N + j;
                                self.broken_allowed[index].write(couplings_ref[i], couplings_ref[j], 1);
                            }
                        }
                    }
                    FinalStabilityResult::UnstableDisallowed(_) => {
                        for i in 0..N {
                            for j in 0..N {
                                let index = i * N + j;
                                self.broken_disallowed[index].write(couplings_ref[i], couplings_ref[j], 1);
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
                self.broken_disallowed[index].merge(&other.broken_disallowed[index]);
            }
        }
    }
}
