use crate::model::Couplings;
use crate::scanner::consumer::ScanConsumer;
use crate::scanner::scanner::CouplingRanges;
use crate::simulation::IntegrationResult;
use crate::util::image::{average_layer, Image, Layer};
use crate::util::stability::FinalStabilityResult;

#[derive(Clone)]
pub struct BreakingScaleConsumer<const N: usize, const NX: usize, const NY: usize> {
    breaking_scale: Box<Layer<(f64, u64), NX, NY>>,
    index_x: usize,
    index_y: usize,
}
impl<const N: usize, const NX: usize, const NY: usize> BreakingScaleConsumer<N, NX, NY> {
    pub fn new(ranges: CouplingRanges<N>, index_x: usize, index_y: usize) -> Self {
        Self {
            breaking_scale: Box::new(average_layer(ranges[index_x], ranges[index_y])),
            index_x,
            index_y,
        }
    }

    pub fn render(&self) -> (Image<NX, NY>, f64, f64) {
        let mut image = Image::new();
        let (min, max) = image.draw_gradient_layer(&self.breaking_scale, 0x00FF00, 0x0000FF);
        (image, min, max)
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
                        self.breaking_scale.write(couplings_ref[self.index_x], couplings_ref[self.index_y], (log_scale, 1));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    fn merge(&mut self, other: Self) {
        self.breaking_scale.merge(&other.breaking_scale);
    }
}
