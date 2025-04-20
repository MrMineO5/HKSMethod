use crate::model::Couplings;
use crate::simulation::IntegrationResult;

pub mod print_consumer;
pub mod stability_consumer;
pub mod allowed_consumer;
pub mod threading_consumer;

pub trait ScanConsumer<const N: usize> {
    fn consume(&mut self, couplings: Couplings<N>, result: IntegrationResult);
}
