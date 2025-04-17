use std::sync::mpsc::Sender;
use indicatif::ProgressBar;
use crate::model::{Couplings, Model};
use crate::scanner::consumer::ScanConsumer;
use crate::simulation::{IntegrationParameters, IntegrationResult, Integrator};

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

    pub fn scan(&mut self, num_samples: u64, consumer: &mut dyn ScanConsumer<N>, sender: Sender<u64>) {
        let mut i = 0;
        while i < num_samples {
            let couplings = generate_couplings(self.coupling_ranges);
            self.integrator.reset(&couplings);
            let res = self.integrator.perform_full_integration();
            if let IntegrationResult::Invalid = res {
                continue;
            }
            consumer.consume(couplings, res);
            i += 1;
            
            if i % 1000 == 0 {
                sender.send(1000).expect("Failed to send progress");
            }
        }
        sender.send(num_samples % 1000).expect("Failed to send progress");
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
