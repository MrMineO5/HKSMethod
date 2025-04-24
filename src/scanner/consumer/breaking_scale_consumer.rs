use crate::model::Couplings;
use crate::scanner::consumer::ScanConsumer;
use crate::scanner::scanner::CouplingRanges;
use crate::simulation::IntegrationResult;
use crate::util::image::{average_layer, Image, Layer};
use crate::util::stability::FinalStabilityResult;

#[derive(Clone)]
pub struct BreakingScaleConsumer<const N: usize, const NX: usize, const NY: usize> {
    breaking_scale: Vec<Layer<(f64, u64), NX, NY>>,
}
impl<const N: usize, const NX: usize, const NY: usize> BreakingScaleConsumer<N, NX, NY> {
    pub fn new(ranges: CouplingRanges<N>) -> Self {
        let mut matrix = vec![average_layer(ranges[0], ranges[0]); N*N];
        for i in 0..N {
            for j in 0..N {
                let range_x = ranges[i];
                let range_y = ranges[j];
                matrix[i * N + j] = average_layer(range_x, range_y);
            }
        }

        let breaking_scale = matrix;

        Self {
            breaking_scale
        }
    }

    pub fn render(&self) -> Vec<Vec<Image<NX, NY>>> {
        let mut images = vec![vec![Image::new(); N]; N];
        for i in 0..N {
            for j in 0..N {
                images[i][j].draw_gradient_layer(&self.breaking_scale[i*N+j], 0x00FF00, 0x0000FF);
            }
        }
        images
    }
}
impl<const N: usize, const NX: usize, const NY: usize> ScanConsumer<N> for BreakingScaleConsumer<N, NX, NY> {
    fn consume(&mut self, couplings: Couplings<N>, result: IntegrationResult) {
        if let IntegrationResult::Unbroken = result {
            return;
        }

        let couplings_ref = &couplings.couplings;

        match result {
            IntegrationResult::Broken(log_scale, result) => {
                match result {
                    FinalStabilityResult::UnstableAllowed(_) => {
                        for i in 0..N {
                            for j in 0..N {
                                let index = i * N + j;
                                self.breaking_scale[index].write(couplings_ref[i], couplings_ref[j], (log_scale, 1));
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
                self.breaking_scale[index].merge(&other.breaking_scale[index])
            }
        }
    }
}
