use indicatif::ProgressBar;
use crate::model::{Couplings, Model};
use crate::scanner::consumer::ScanConsumer;
use crate::simulation::{IntegrationParameters, Integrator};

pub type CouplingRanges<const N: usize> = [(f64, f64); N];

pub struct Scanner<const N: usize> {
    coupling_ranges: CouplingRanges<N>,
    integrator: Integrator<N>,
}
impl<const N: usize> Scanner<N> {
    pub fn new(
        coupling_ranges: CouplingRanges<N>,
        params: IntegrationParameters,
        model: Box<dyn Model<N>>,
    ) -> Self {
        Self {
            coupling_ranges,
            integrator: Integrator::new(params, model, generate_couplings(coupling_ranges)),
        }
    }

    pub fn scan(&mut self, num_samples: u64, consumer: &mut dyn ScanConsumer<N>, pb: ProgressBar) {
        for i in 0..num_samples {
            let couplings = generate_couplings(self.coupling_ranges);
            self.integrator.reset(&couplings);
            let res = self.integrator.perform_full_integration();
            consumer.consume(couplings, res);
            
            if i % 1000 == 0 {
                pb.inc(1000);
            }
        }
        pb.finish_and_clear();
    }
}

fn generate_couplings<const N: usize>(coupling_ranges: CouplingRanges<N>) -> Couplings<N> {
    let mut couplings = [0.0; N];
    for i in 0..N {
        let (min, max) = coupling_ranges[i];
        couplings[i] = rand::random::<f64>() * (max - min) + min;
    }
    Couplings { couplings }
}
