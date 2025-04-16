use crate::model::Couplings;
use crate::scanner::consumer::ScanConsumer;
use crate::simulation::IntegrationResult;

pub struct PrintConsumer<const N: usize>;
impl<const N: usize> ScanConsumer<N> for PrintConsumer<N> {
    fn consume(&mut self, couplings: Couplings<N>, result: IntegrationResult) {
        print!("Couplings: {:?} => ", couplings.couplings);
        match result {
            IntegrationResult::Unbroken => println!("Unbroken"),
            IntegrationResult::InitiallyUnstable => println!("Initially Unstable"),
            IntegrationResult::PerturbativityViolated(scale) => {
                println!("Perturbativity Violated at scale {}", scale)
            }
            IntegrationResult::Broken(scale, stability_result) => {
                println!("Broken at scale {}: {:?}", scale, stability_result)
            }
        }
    }
}
