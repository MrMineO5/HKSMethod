use std::marker::PhantomData;
use crate::model::{Couplings, Model};
use crate::scanner::consumer::ScanConsumer;
use crate::simulation::{IntegrationParameters, IntegrationResult, Integrator};
use std::sync::mpsc::Sender;

pub type CouplingRanges<const N: usize> = [(f64, f64); N];

pub struct Scanner<T: ScanConsumer<N>, const N: usize> {
    coupling_ranges: CouplingRanges<N>,
    integrator: Integrator<N>,
    
    consumer: PhantomData<T>
}
impl<T: ScanConsumer<N>, const N: usize> Scanner<T, N> {
    pub fn new(
        coupling_ranges: CouplingRanges<N>,
        params: IntegrationParameters,
        model: Box<dyn Model<N>>,
    ) -> Self {
        Self {
            coupling_ranges,
            integrator: Integrator::new(params, model, generate_couplings(coupling_ranges)),
            
            consumer: PhantomData,
        }
    }

    pub fn scan(
        &mut self,
        num_samples: u64,
        consumer: &mut T,
        sender: Sender<u64>,
    ) {
        let mut i = 0;
        let mut invalids = 0;
        while i < num_samples {
            let couplings = generate_couplings(self.coupling_ranges);
            self.integrator.reset(&couplings);
            let res = self.integrator.perform_full_integration();
            if let IntegrationResult::Invalid = res {
                invalids += 1;
                continue;
            }
            consumer.consume(couplings, res);
            i += 1;

            if i % 1000 == 0 {
                sender.send(1000).expect("Failed to send progress");
            }
        }
        sender
            .send(num_samples % 1000)
            .expect("Failed to send progress");
        
        println!("Invalids: {}", invalids);
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
