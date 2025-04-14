use crate::model::{BetaFunctionValue, Couplings, Model};
use crate::util::stability::{stab2vev, FinalStabilityResult, StabilityResult};

pub struct ToyModel;
impl Model<3> for ToyModel {
    fn beta_function(&self, couplings: &Couplings<3>) -> [BetaFunctionValue; 3] {
        let g = couplings.couplings[0];
        let l1 = couplings.couplings[1];
        let l2 = couplings.couplings[2];

        let g2 = g.powi(2);
        let g4 = g.powi(4);
        let g6 = g.powi(6);

        let l1_2 = l1.powi(2);
        let l1_3 = l1.powi(3);

        let l2_2 = l2.powi(2);
        let l2_3 = l2.powi(3);

        [
            BetaFunctionValue {
                b1: -24.0 * g.powi(3),
                b2: (-697.0/2.0) * g.powi(5),
                b3: (-291217.0 / 96.0) * g.powi(7)
                    + 640.0 * g.powi(5) * l1
                    - 3008.0 * g.powi(3) * l1_2
                    + 272.0 * g.powi(5) * l2
                    - 3296.0 * g.powi(3) * l1 * l2
                    - 1532.0 * g.powi(3) * l2_2
            },
            BetaFunctionValue {
                b1: (27./4.) * g4 - 96. * g2 * l1 + 424. * l1_2 + 412. * l1*l2 + (279./2.) * l2_2,
                b2: 72.0 * g6
                    - 360.0 * g4 * l1
                    + 25600.0 * g2 * l1_2
                    - 28608.0 * l1_3
                    + 648.0 * g4 * l2
                    + 26368.0 * g2 * l1 * l2
                    - 36256.0 * l1_2 * l2
                    + 8172.0 * g2 * l2_2
                    - 21052.0 * l1 * l2_2
                    - 4572.0 * l2_3,
                b3: 0.0
            },
            BetaFunctionValue {
                b1: -3.0 * g4 - 96.0 * g2 * l2 + 96.0 * l1 * l2 - 4.0 * l2_2,
                b2: -32.0 * g6
                    - 96.0 * g4 * l1
                    - 1632.0 * g4 * l2
                    + 3072.0 * g2 * l1 * l2
                    - 19648.0 * l1_2 * l2
                    - 304.0 * g2 * l2_2
                    - 16096.0 * l1 * l2_2
                    - 3660.0 * l2_3,
                b3: 0.0
            },
        ]
    }

    fn stability_condition(&self, couplings: &Couplings<3>) -> FinalStabilityResult {
        let [_g, l1, l2] = couplings.couplings;

        match stab2vev(
            9. * l1 + 21.0 / 4.0 * l2,
            12. * l1 + 9. * l2,
            4. * l1 + 2. * l2
        ) {
            StabilityResult::Stable => {}
            other => return FinalStabilityResult::UnstableAllowed(other),
        };

        match stab2vev(
            16. * l1 + 10. * l2,
            8. * l1 + 6. * l2,
            l1 + l2 / 4.
        ) {
            StabilityResult::Stable => {}
            other => return FinalStabilityResult::UnstableDisallowed(other),
        };

        match stab2vev(
            l1 + l2 / 4.,
            2. * l1 + 3.0 / 2.0 * l2,
            l1 + l2 / 4.
        ) {
            StabilityResult::Stable => {}
            other => return FinalStabilityResult::UnstableDisallowed(other),
        };

        FinalStabilityResult::Stable
    }
}