use crate::model::Couplings;
use crate::scanner::consumer::ScanConsumer;
use crate::scanner::scanner::CouplingRanges;
use crate::simulation::IntegrationResult;
use crate::util::image::{boolean_layer, Image, Layer};

#[derive(Clone)]
pub struct StabilityConsumer<const N: usize, const NX: usize, const NY: usize> {
    perturbativity_violated: Vec<Layer<bool, NX, NY>>,
    stability_violated: Vec<Layer<bool, NX, NY>>,
    broken: Vec<Layer<bool, NX, NY>>,
}
impl<const N: usize, const NX: usize, const NY: usize> StabilityConsumer<N, NX, NY> {
    pub fn new(ranges: CouplingRanges<N>) -> Self {
        let mut matrix = vec![boolean_layer(ranges[0], ranges[0]); N*N];
        for i in 0..N {
            for j in 0..N {
                let range_x = ranges[i];
                let range_y = ranges[j];
                matrix[i * N + j] = boolean_layer(range_x, range_y);
            }
        }

        let perturbativity_violated = matrix.clone();
        let stability_violated = matrix.clone();
        let broken = matrix;

        Self {
            perturbativity_violated,
            stability_violated,
            broken
        }
    }

    pub fn render(&self) -> Vec<Vec<Image<NX, NY>>> {
        let mut images = vec![vec![Image::new(); N]; N];
        for i in 0..N {
            for j in 0..N {
                images[i][j].draw_boolean_layer(&self.stability_violated[i*N+j], 0xFF0000);
                images[i][j].draw_boolean_layer(&self.perturbativity_violated[i*N+j], 0xFFFFFF);
                images[i][j].draw_boolean_layer(&self.broken[i*N+j], 0x00FF00);
            }
        }
        images
    }
}
impl<const N: usize, const NX: usize, const NY: usize> ScanConsumer<N> for StabilityConsumer<N, NX, NY> {
    fn consume(&mut self, couplings: Couplings<N>, result: IntegrationResult) {
        if let IntegrationResult::Unbroken = result {
            return;
        }

        let couplings_ref = &couplings.couplings;

        match result {
            IntegrationResult::PerturbativityViolated(_) => {
                for i in 0..N {
                    for j in 0..N {
                        let index = i * N + j;
                        self.perturbativity_violated[index].write(couplings_ref[i], couplings_ref[j], true);
                    }
                }
            }
            IntegrationResult::Broken(_, _) => {
                for i in 0..N {
                    for j in 0..N {
                        let index = i * N + j;
                        self.broken[index].write(couplings_ref[i], couplings_ref[j], true);
                    }
                }
            }
            IntegrationResult::InitiallyUnstable => {
                for i in 0..N {
                    for j in 0..N {
                        let index = i * N + j;
                        self.stability_violated[index].write(couplings_ref[i], couplings_ref[j], true);
                    }
                }
            }
            _ => {}
        }
    }
    fn merge(&mut self, other: Self) {
        for i in 0..N {
            for j in 0..N {
                let index = i * N + j;
                self.perturbativity_violated[index].merge(&other.perturbativity_violated[index]);
                self.stability_violated[index].merge(&other.stability_violated[index]);
                self.broken[index].merge(&other.broken[index]);
            }
        }
    }
}
