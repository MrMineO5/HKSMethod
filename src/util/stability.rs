use crate::util::polysolver::{solve_quadratic, solve_quartic};

const THRESHOLD: f64 = 1e-10;

#[derive(Debug)]
pub enum FinalStabilityResult {
    Stable,
    UnstableAllowed(StabilityResult),
    UnstableDisallowed(StabilityResult)
}

#[derive(Debug)]
pub enum StabilityResult {
    Stable,
    Violated1([f64; 3]),
    Violated2([f64; 3], [f64; 3])
}

pub fn stab2vev(a0: f64, a1: f64, a2: f64) -> StabilityResult {
    if a0 <= 0. {
        StabilityResult::Violated1([1., 0., 0.])
    } else if a2 <= 0. {
        StabilityResult::Violated1([0., 1., 0.])
    } else if a1 + 2. * (a0*a2).sqrt() <= 0. {
        StabilityResult::Violated2([a2.powf(0.25), a0.powf(0.25), 0.0], [a2.powf(0.25), -a0.powf(0.25), 0.0])
    } else {
        StabilityResult::Stable
    }
}

pub fn stab3vev(alpha: f64, b0: f64, b1: f64, b2: f64, c0: f64, c1: f64, c2: f64) -> StabilityResult {
    if alpha <= 0. {
        return StabilityResult::Violated1([1., 0., 0.]);
    }
    if c0 <= 0. {
        return StabilityResult::Violated1([0., 1., 0.]);
    }
    if c2 <= 0. {
        return StabilityResult::Violated1([0., 0., 1.]);
    }
    if c1 + 2. * (c0*c2).sqrt() <= 0. {
        return StabilityResult::Violated2(
            [0., c2.powf(0.25), c0.powf(0.25)],
            [0., c2.powf(0.25), -c0.powf(0.25)]
        );
    }

    let a0 = -b0 * b0 + 4. * alpha * c0;
    let a1 = -2. * b0 * b1;
    let a2 = -b1 * b1 - 2. * b0 * b2 + 4. * alpha * c1;
    let a3 = -2. * b1 * b2;
    let a4 = -b2 * b2 + 4. * alpha * c2;

    let (d1, d2, d3, d4) = solve_quartic(a0, a1, a2, a3, a4);
    let (beta1, beta2) = solve_quadratic(b0, b1, b2);

    if d1.is_nan() && !(beta1.is_nan() && b2 <= 0.) {
        return StabilityResult::Stable;
    } else if beta1.is_nan() { // No real roots
        if b2 > 0. {
            return StabilityResult::Stable;
        }
    } else {
        if d3.is_nan() {
            if b2 > 0. {
                if a4 > 0. {
                    if beta2 < d1 || beta1 > d2 {
                        return StabilityResult::Stable;
                    }
                } else {
                    if beta1 > d1 && beta2 < d2 {
                        return StabilityResult::Stable;
                    }
                }
            } else {
                if a4 > 0. {
                    if beta2 < d1 && beta1 > d2 {
                        return StabilityResult::Stable;
                    }
                }
            }
        } else {
            if b2 > 0. {
                if a4 > 0. {
                    if beta2 < d1 || beta1 > d4 || (beta1 > d2 && beta2 < d3) {
                        return StabilityResult::Stable;
                    }
                } else {
                    if (beta1 > d1 && beta2 < d2) || (beta1 > d3 && beta2 < d4) {
                        return StabilityResult::Stable;
                    }
                }
            } else {
                if a4 > 0. {
                    if beta1 < d1 || beta2 > d4 {
                        return StabilityResult::Stable;
                    }
                }
            }
        }
    }

    if a0 <= 0. {
        return StabilityResult::Violated2(
            [c0.powf(0.25), alpha.powf(0.25), 0.],
            [c0.powf(0.25), -alpha.powf(0.25), 0.]
        )
    }

    if a4 <= 0. {
        return StabilityResult::Violated2(
            [c0.powf(0.25), 0., alpha.powf(0.25)],
            [c0.powf(0.25), 0., -alpha.powf(0.25)]
        )
    }

    println!("Roots: d1: {}, d2: {}, d3: {}, d4: {}", d1, d2, d3, d4);

    let repeated = if (d1 - d2).abs() < THRESHOLD {
        (d1 + d2) / 2.
    } else if (d2 - d3).abs() < THRESHOLD {
        (d2 + d3) / 2.
    } else if (d3 - d4).abs() < THRESHOLD {
        (d3 + d4) / 2.
    } else {
        panic!("No repeated roots found");
    };

    let val = ((c0 + c1 * repeated.powi(2) + c2 * repeated.powi(4)) / alpha).powf(0.25);

    StabilityResult::Violated2(
        [val, 1., repeated],
        [-val, 1., repeated]
    )
}

