const EPSILON: f64 = 1e-15;

pub fn solve_linear(a0: f64, a1: f64) -> f64 {
    if a1.abs() < EPSILON {
        return f64::NAN;
    }
    -a0 / a1
}

pub fn solve_quadratic(a0: f64, a1: f64, a2: f64) -> (f64, f64) {
    if a2.abs() < EPSILON {
        return (solve_linear(a0, a1), f64::NAN);
    }

    let d = a1 * a1 - 4.0 * a2 * a0;
    if d < 0.0 {
        return (f64::NAN, f64::NAN);
    }
    let sqrt_d = d.sqrt();
    let x1 = (-a1 - sqrt_d) / (2.0 * a2);
    let x2 = (-a1 + sqrt_d) / (2.0 * a2);
    (x1, x2)
}

pub fn solve_cubic(a0: f64, a1: f64, a2: f64, a3: f64) -> (f64, f64, f64) {
    if a3.abs() < EPSILON {
        let (x1, x2) = solve_quadratic(a0, a1, a2);
        return (x1, x2, f64::NAN);
    }

    let p = (3. * a3 * a1 - a2 * a2) / (3. * a3 * a3);
    let q = (2. * a2 * a2 * a2 - 9. * a3 * a2 * a1 + 27. * a3 * a3 * a0) / (27. * a3 * a3 * a3);
    let d = 4. * p.powi(3) + 27. * q.powi(2);

    if p.abs() < EPSILON {
        return (-q.cbrt(), f64::NAN, f64::NAN);
    }

    let shift = -a2 / (3. * a3);
    if d <= 0.0 {
        let prefac = 2.0 * (-p / 3.).sqrt();
        let theta = (1. / 3.) * (3. * q / (2. * p) * (-3. / p).sqrt()).acos();
        let x1 = prefac * theta.cos() + shift;
        let x2 = prefac * (theta - std::f64::consts::TAU / 3.).cos() + shift;
        let x3 = prefac * (theta - 2.0 * std::f64::consts::TAU / 3.).cos() + shift;
        (x1, x2, x3)
    } else {
        let t0 = if p < 0. {
            -2. * q.signum()
                * (-p / 3.).sqrt()
                * ((1. / 3.) * (-3. * q.abs() / (2. * p) * (-3. / p).sqrt()).acosh()).cosh()
        } else {
            -2. * (p / 3.).sqrt()
                * ((1. / 3.) * (3. * q / (2. * p) * (3. / p).sqrt()).asinh()).sinh()
        };
        let x1 = t0 + shift;
        (x1, f64::NAN, f64::NAN)
    }
}

pub fn solve_quartic(a0: f64, a1: f64, a2: f64, a3: f64, a4: f64) -> (f64, f64, f64, f64) {
    if a4.abs() < EPSILON {
        let (x1, x2, x3) = solve_cubic(a0, a1, a2, a3);
        return (x1, x2, x3, f64::NAN);
    }

    let a = -3. * a3 * a3 / (8. * a4 * a4) + a2 / a4;
    let b = a3 * a3 * a3 / (8. * a4 * a4 * a4) - a3 * a2 / (2. * a4 * a4) + a1 / a4;
    let c = -3. * a3 * a3 * a3 * a3 / (256. * a4 * a4 * a4 * a4)
        + a3 * a3 * a2 / (16. * a4 * a4 * a4)
        - a3 * a1 / (4. * a4 * a4)
        + a0 / a4;

    let shift = -a3 / (4. * a4);

    if b.abs() < EPSILON {
        let (x1, x2) = solve_quadratic(c, a, 1.);
        return if x2.is_nan() {
            if x1.is_nan() || x1 < 0. {
                (f64::NAN, f64::NAN, f64::NAN, f64::NAN)
            } else {
                (-x1.sqrt() + shift, x1.sqrt() + shift, f64::NAN, f64::NAN)
            }
        } else {
            if x1 < 0. {
                (f64::NAN, f64::NAN, f64::NAN, f64::NAN)
            } else if x2 < 0. {
                (-x1.sqrt() + shift, x1.sqrt() + shift, f64::NAN, f64::NAN)
            } else {
                (
                    -x2.sqrt() + shift,
                    -x1.sqrt() + shift,
                    x1.sqrt() + shift,
                    x2.sqrt() + shift,
                )
            }
        };
    }

    let (y1, _y2, _y3) = solve_cubic(a * c - 1. / 4. * b * b, -2. * c, -a, 2.);

    let s1 = (2. * y1 - a).sqrt();

    if s1.is_nan() {
        return (f64::NAN, f64::NAN, f64::NAN, f64::NAN);
    }

    let s2 = (-2. * y1 - a + 2. * b / s1).sqrt();
    let s3 = (-2. * y1 - a - 2. * b / s1).sqrt();

    let x1 = 0.5 * (-s1 + s2) + shift;
    let x2 = 0.5 * (-s1 - s2) + shift;
    let x3 = 0.5 * (s1 + s3) + shift;
    let x4 = 0.5 * (s1 - s3) + shift;
    
    if s1.is_infinite() {
        (f64::NAN, f64::NAN, f64::NAN, f64::NAN)
    } else if s2.is_nan() {
        if s3.is_nan() {
            (f64::NAN, f64::NAN, f64::NAN, f64::NAN)
        } else {
            (x4, x3, f64::NAN, f64::NAN)
        }
    } else if s3.is_nan() {
        (x2, x1, f64::NAN, f64::NAN)
    } else {
        if x1 < x4 {
            (x2, x1, x4, x3)
        } else {
            (x2, x4, x1, x3)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_quartic() {
        let coeffs = [
            0.3364831008218681,
            -0.9868269363515232,
            -0.6980929773084139,
            -1.006931810296128,
            0.0000007415144538114316
        ];
        let (x1, x2, x3, x4) = solve_quartic(coeffs[0], coeffs[1], coeffs[2], coeffs[3], coeffs[4]);

        println!("Roots: {:?}, {:?}, {:?}, {:?}", x1, x2, x3, x4);

        for root in [x1, x2, x3, x4] {
            if root.is_nan() {
                continue;
            }
            let value = coeffs[0]
                + coeffs[1] * root
                + coeffs[2] * root.powi(2)
                + coeffs[3] * root.powi(3)
                + coeffs[4] * root.powi(4);
            assert!(
                value.abs() < 1e-10,
                "Root {} does not satisfy the equation: {}",
                root,
                value
            );
        }
    }

    #[test]
    fn test_solve_cubic() {
        let coeffs = [
            -18.086502522062556,
            7.650661100021841,
            6.775250896636531,
            2.0,
        ];
        let (x1, x2, x3) = solve_cubic(coeffs[0], coeffs[1], coeffs[2], coeffs[3]);

        println!("Roots: {:?}, {:?}, {:?}", x1, x2, x3);

        for root in [x1, x2, x3] {
            if root.is_nan() {
                continue;
            }
            let value =
                coeffs[0] + coeffs[1] * root + coeffs[2] * root.powi(2) + coeffs[3] * root.powi(3);
            assert!(
                value.abs() < 1e-10,
                "Root {} does not satisfy the equation: {}",
                root,
                value
            );
        }
    }
}
