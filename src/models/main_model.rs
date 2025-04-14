use crate::model::{BetaFunctionValue, Couplings, Model};
use crate::stability::{stab3vev, FinalStabilityResult, StabilityResult};

pub struct MainMode;
impl Model<7> for MainMode {
    fn beta_function(&self, couplings: &Couplings<7>) -> [BetaFunctionValue; 7] {
        let [g, l1, l2, l6, l7, l8, l9] = couplings.couplings;

        let g2 = g.powi(2);
        let g3 = g.powi(3);
        let g4 = g.powi(4);
        let g5 = g.powi(5);

        let l8_2 = l8.powi(2);
        let l9_2 = l9.powi(2);

        [
            BetaFunctionValue {
                b1: (-70.0 / 3.0) * g3,
                b2: (-1757.0 / 6.0) * g5,
                b3: (1077557.0 / 1728.0) * g.powi(7) + 640.0 * g5 * l1 - 3008.0 * g3 * l1.powi(2)
                    + 272.0 * g5 * l2
                    - 3296.0 * g3 * l1 * l2
                    - 1532.0 * g3 * l2.powi(2)
                    + 180.0 * g5 * l6
                    - 1088.0 * g3 * l6.powi(2)
                    + 132.0 * g5 * l7
                    - 640.0 * g3 * l6 * l7
                    - 1280.0 * g3 * l7.powi(2)
                    + 256.0 * g5 * l8
                    - 3488.0 * g3 * l8_2
                    + 512.0 * g5 * l9
                    - 1744.0 * g3 * l8 * l9
                    - 6322.0 * g3 * l9_2,
            },
            BetaFunctionValue {
                b1: (27.0 / 4.0) * g4 - 96.0 * g2 * l1
                    + 424.0 * l1.powi(2)
                    + 412.0 * l1 * l2
                    + (279.0 / 2.0) * l2.powi(2)
                    + 256.0 * l8_2
                    + 128.0 * l8 * l9,
                b2: 51.0 * g.powi(6) - (728.0 / 3.0) * g4 * l1 + 25600.0 * g2 * l1.powi(2)
                    - 28608.0 * l1.powi(3)
                    + 648.0 * g4 * l2
                    + 26368.0 * g2 * l1 * l2
                    - 36256.0 * l1.powi(2) * l2
                    + 8172.0 * g2 * l2.powi(2)
                    - 21052.0 * l1 * l2.powi(2)
                    - 4572.0 * l2.powi(3)
                    + 640.0 * g4 * l8
                    + 11520.0 * g2 * l8_2
                    - 10240.0 * l1 * l8_2
                    - 8192.0 * l8.powi(3)
                    + 120.0 * g4 * l9
                    + 5760.0 * g2 * l8 * l9
                    - 5120.0 * l1 * l8 * l9
                    - 6144.0 * l8_2 * l9
                    + 576.0 * g2 * l9_2
                    - 3712.0 * l1 * l9_2
                    - 5952.0 * l2 * l9_2
                    - 14848.0 * l8 * l9_2
                    - 1536.0 * l9.powi(3),
                b3: 0.,
            },
            BetaFunctionValue {
                b1: -3.0 * g4 - 96.0 * g2 * l2 + 96.0 * l1 * l2 - 4.0 * l2.powi(2) + 64.0 * l9_2,
                b2: (-68.0 / 3.0) * g.powi(6) - 96.0 * g4 * l1 - (4544.0 / 3.0) * g4 * l2
                    + 3072.0 * g2 * l1 * l2
                    - 19648.0 * l1.powi(2) * l2
                    - 304.0 * g2 * l2.powi(2)
                    - 16096.0 * l1 * l2.powi(2)
                    - 3660.0 * l2.powi(3)
                    - 10240.0 * l2 * l8_2
                    + 160.0 * g4 * l9
                    - 5120.0 * l2 * l8 * l9
                    + 576.0 * g2 * l9_2
                    - 2048.0 * l1 * l9_2
                    + 5248.0 * l2 * l9_2
                    - 4096.0 * l8 * l9_2
                    - 8704.0 * l9.powi(3),
                b3: 0.,
            },
            BetaFunctionValue {
                b1: (315.0 / 128.0) * g4 - (135.0 / 2.0) * g2 * l6
                    + 320.0 * l6.powi(2)
                    + 160.0 * l6 * l7
                    + 80.0 * l7.powi(2)
                    + 360.0 * l8_2
                    + 180.0 * l8 * l9
                    + (105.0 / 2.0) * l9_2,
                b2: (45925.0 / 512.0) * g.powi(6) - (26475.0 / 32.0) * g4 * l6
                    + 13320.0 * g2 * l6.powi(2)
                    - 21120.0 * l6.powi(3)
                    + (1665.0 / 8.0) * g4 * l7
                    + 7200.0 * g2 * l6 * l7
                    - 14080.0 * l6.powi(2) * l7
                    + 2160.0 * g2 * l7.powi(2)
                    - 20480.0 * l6 * l7.powi(2)
                    - 6400.0 * l7.powi(3)
                    + 900.0 * g4 * l8
                    + 23040.0 * g2 * l8_2
                    - 14400.0 * l6 * l8_2
                    - 11520.0 * l8.powi(3)
                    + 300.0 * g4 * l9
                    + 11520.0 * g2 * l8 * l9
                    - 7200.0 * l6 * l8 * l9
                    - 8640.0 * l8_2 * l9
                    + 2280.0 * g2 * l9_2
                    - 6900.0 * l6 * l9_2
                    - 3840.0 * l7 * l9_2
                    - 24240.0 * l8 * l9_2
                    - 9300.0 * l9.powi(3),
                b3: 0.,
            },
            BetaFunctionValue {
                b1: (9.0 / 8.0) * g4 - (135.0 / 2.0) * g2 * l7 + 96.0 * l6 * l7 + 24.0 * l9_2,
                b2: (479.0 / 32.0) * g.powi(6) + 36.0 * g4 * l6 - (44511.0 / 32.0) * g4 * l7
                    + 2160.0 * g2 * l6 * l7
                    - 15488.0 * l6.powi(2) * l7
                    + 576.0 * g2 * l7.powi(2)
                    - 6400.0 * l6 * l7.powi(2)
                    + 6400.0 * l7.powi(3)
                    - 14400.0 * l7 * l8_2
                    + 60.0 * g4 * l9
                    - 7200.0 * l7 * l8 * l9
                    + 672.0 * g2 * l9_2
                    - 768.0 * l6 * l9_2
                    + 3084.0 * l7 * l9_2
                    - 1536.0 * l8 * l9_2
                    - 3264.0 * l9.powi(3),
                b3: 0.,
            },
            BetaFunctionValue {
                b1: (9.0 / 8.0) * g4 - (327.0 / 4.0) * g2 * l8
                    + 376.0 * l1 * l8
                    + 206.0 * l2 * l8
                    + 272.0 * l6 * l8
                    + 80.0 * l7 * l8
                    + 32.0 * l8_2
                    + 90.0 * l1 * l9
                    + (93.0 / 2.0) * l2 * l9
                    + 64.0 * l6 * l9
                    + 16.0 * l7 * l9
                    + 24.0 * l9_2,
                b2: (5287.0 / 64.0) * g.powi(6)
                    + 465.0 * g4 * l1
                    + (1005.0 / 4.0) * g4 * l2
                    + 335.0 * g4 * l6
                    + 95.0 * g4 * l7
                    - (129865.0 / 192.0) * g4 * l8
                    + 24064.0 * g2 * l1 * l8
                    - 7520.0 * l1.powi(2) * l8
                    + 13184.0 * g2 * l2 * l8
                    - 8240.0 * l1 * l2 * l8
                    - 3830.0 * l2.powi(2) * l8
                    + 12240.0 * g2 * l6 * l8
                    - 5440.0 * l6.powi(2) * l8
                    + 3600.0 * g2 * l7 * l8
                    - 3200.0 * l6 * l7 * l8
                    - 6400.0 * l7.powi(2) * l8
                    + 436.0 * g2 * l8_2
                    - 18048.0 * l1 * l8_2
                    - 9888.0 * l2 * l8_2
                    - 13056.0 * l6 * l8_2
                    - 3840.0 * l7 * l8_2
                    - 2976.0 * l8.powi(3)
                    + (477.0 / 2.0) * g4 * l9
                    + 5904.0 * g2 * l1 * l9
                    - 1440.0 * l1.powi(2) * l9
                    + 3156.0 * g2 * l2 * l9
                    - 1488.0 * l1 * l2 * l9
                    - 930.0 * l2.powi(2) * l9
                    + 3024.0 * g2 * l6 * l9
                    - 1024.0 * l6.powi(2) * l9
                    + 864.0 * g2 * l7 * l9
                    - 512.0 * l6 * l7 * l9
                    - 1792.0 * l7.powi(2) * l9
                    - 288.0 * g2 * l8 * l9
                    - 5760.0 * l1 * l8 * l9
                    - 2976.0 * l2 * l8 * l9
                    - 4096.0 * l6 * l8 * l9
                    - 1024.0 * l7 * l8 * l9
                    - 1232.0 * l8_2 * l9
                    + 471.0 * g2 * l9_2
                    - 11016.0 * l1 * l9_2
                    - 6114.0 * l2 * l9_2
                    - 8000.0 * l6 * l9_2
                    - 2432.0 * l7 * l9_2
                    - 5618.0 * l8 * l9_2
                    - 5760.0 * l9.powi(3),
                b3: 0.,
            },
            BetaFunctionValue {
                b1: (3.0 / 2.0) * g4 - (327.0 / 4.0) * g2 * l9
                    + 16.0 * l1 * l9
                    + 20.0 * l2 * l9
                    + 16.0 * l6 * l9
                    + 16.0 * l7 * l9
                    + 64.0 * l8 * l9
                    + 136.0 * l9_2,
                b2: (103.0 / 48.0) * g.powi(6)
                    + 20.0 * g4 * l1
                    + 25.0 * g4 * l2
                    + 20.0 * g4 * l6
                    + 20.0 * g4 * l7
                    + 8.0 * g4 * l8
                    - (301897.0 / 192.0) * g4 * l9
                    + 448.0 * g2 * l1 * l9
                    - 1760.0 * l1.powi(2) * l9
                    + 560.0 * g2 * l2 * l9
                    - 2288.0 * l1 * l2 * l9
                    - 110.0 * l2.powi(2) * l9
                    + 144.0 * g2 * l6 * l9
                    - 1344.0 * l6.powi(2) * l9
                    + 144.0 * g2 * l7 * l9
                    - 1152.0 * l6 * l7 * l9
                    + 768.0 * l7.powi(2) * l9
                    + 2024.0 * g2 * l8 * l9
                    - 13056.0 * l1 * l8 * l9
                    - 7872.0 * l2 * l8 * l9
                    - 9728.0 * l6 * l8 * l9
                    - 3584.0 * l7 * l8 * l9
                    - 4000.0 * l8_2 * l9
                    + 5309.0 * g2 * l9_2
                    - 6144.0 * l1 * l9_2
                    - 5568.0 * l2 * l9_2
                    - 5312.0 * l6 * l9_2
                    - 3776.0 * l7 * l9_2
                    - 7760.0 * l8 * l9_2
                    + 2414.0 * l9.powi(3),
                b3: 0.,
            },
        ]
    }

    fn stability_condition(&self, couplings: &Couplings<7>) -> FinalStabilityResult {
        let [_g, l1, l2, l6, _l7, l8, l9] = couplings.couplings;

        match stab3vev(
            4. * l6,
            12. * l8 + 9. * l9,
            -12. * l9,
            8. * l8 + 4. * l9,
            9. * l1 + (21. / 4.) * l2,
            12. * l1 + 9. * l2,
            4. * l1 + 2. * l2,
        ) {
            StabilityResult::Stable => {}
            other => return FinalStabilityResult::UnstableAllowed(other),
        };

        match stab3vev(
            4. * l6,
            12. * l8 + 9. * l9,
            12. * l9,
            8. * l8 + 4. * l9,
            9. * l1 + (21. / 4.) * l2,
            12. * l1 + 9. * l2,
            4. * l1 + 2. * l2,
        ) {
            StabilityResult::Stable => {}
            other => return FinalStabilityResult::UnstableAllowed(other),
        };

        match stab3vev(
            4. * l6,
            16. * l8 + 16. * l9,
            -8. * l9,
            4. * l8 + l9,
            16. * l1 + 10. * l2,
            8. * l1 + 6. * l2,
            l1 + l2 / 4.,
        ) {
            StabilityResult::Stable => {}
            other => return FinalStabilityResult::UnstableDisallowed(other),
        };

        match stab3vev(
            4. * l6,
            16. * l8 + 16. * l9,
            8. * l9,
            4. * l8 + l9,
            16. * l1 + 10. * l2,
            8. * l1 + 6. * l2,
            l1 + l2 / 4.,
        ) {
            StabilityResult::Stable => {}
            other => return FinalStabilityResult::UnstableDisallowed(other),
        };

        FinalStabilityResult::Stable
    }
}
