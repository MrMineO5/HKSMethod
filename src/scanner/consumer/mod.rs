use crate::model::Couplings;
use crate::simulation::IntegrationResult;

pub mod print_consumer;
pub mod stability_consumer;
pub mod allowed_consumer;
pub mod threading_consumer;
pub mod special_allowed_consumer;
pub mod multi_special_allowed_consumer;
pub mod breaking_scale_consumer;

pub trait ScanConsumer<const N: usize>: Clone {
    fn consume(&mut self, couplings: Couplings<N>, result: IntegrationResult);
    fn merge(&mut self, other: Self);
}
