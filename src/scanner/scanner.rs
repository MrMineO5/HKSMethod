use crate::model::{Couplings, Model};
use crate::simulation::{IntegrationParameters, Integrator};

pub struct Scanner<const N: usize> {
    coupling_ranges: [(f64, f64); N],
    integrator: Integrator<N>,
}
impl<const N: usize> Scanner<N> {
    pub fn new(
        coupling_ranges: [(f64, f64); N],
        params: IntegrationParameters,
        model: Box<dyn Model<N>>,
    ) -> Self {
        Self {
            coupling_ranges,
            integrator: Integrator::new(params, model, generate_couplings(coupling_ranges)),
        }
    }

    pub fn scan(&mut self, num_samples: usize) {
        for _ in 0..num_samples {
            let couplings = generate_couplings(self.coupling_ranges);
            self.integrator.reset(couplings);
            self.integrator.perform_full_integration();
        }
    }
}

fn generate_couplings<const N: usize>(
    coupling_ranges: [(f64, f64); N],
) -> Couplings<N> {
    let mut couplings = [0.0; N];
    for i in 0..N {
        let (min, max) = coupling_ranges[i];
        couplings[i] = rand::random::<f64>() * (max - min) + min;
    }
    Couplings { couplings }
}
