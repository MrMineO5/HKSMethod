use crate::model::Couplings;
use crate::scanner::consumer::ScanConsumer;
use crate::simulation::IntegrationResult;
use std::sync::mpsc::Sender;

#[derive(Clone)]
pub struct SendConsumer<const N: usize> {
    pub sender: Sender<(Couplings<N>, IntegrationResult)>,
}
impl<const N: usize> ScanConsumer<N> for SendConsumer<N> {
    fn consume(&mut self, couplings: Couplings<N>, result: IntegrationResult) {
        self.sender.send((couplings, result)).unwrap();
    }

    fn merge(&mut self, other: Self) {
        // No need to merge, all data was already sent to the channel
    }
}
