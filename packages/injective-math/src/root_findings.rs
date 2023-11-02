use crate::fp_decimal::FPDecimal;
use crate::utils::round;

pub fn newton<Func, DFunc>(f: Func, fd: DFunc, mut x0: FPDecimal, abs_error: FPDecimal, max_iter: usize) -> Result<FPDecimal, FPDecimal>
where
    Func: Fn(FPDecimal) -> FPDecimal,
    DFunc: Fn(FPDecimal) -> FPDecimal,
{
    // WARN: No complex number support
    let mut x1 = x0;
    for _ in 0..max_iter {
        x0 -= f(x1) / fd(x1);
        if (x0 - x1).abs() < abs_error {
            return Ok(x0);
        }
        x1 = x0;
    }
    Err(x0)
}

pub fn discrete_newton<Func>(
    func: Func,
    delta: FPDecimal,
    x0: FPDecimal,
    obj: FPDecimal,
    abs_error: FPDecimal,
    max_iter: usize,
) -> Result<FPDecimal, FPDecimal>
where
    Func: Fn(FPDecimal) -> FPDecimal,
{
    //https://www.mathworks.com/matlabcentral/fileexchange/68170-discrete-newton-newton-s-method-for-discrete-functions
    // WARN: No complex number support
    let y0 = func(x0);
    let mut xn = x0;
    let mut yn = y0;

    let x1 = x0 + delta;
    let y1 = func(x1);

    let mut m = (y1 - y0) / (x1 - x0);

    let mut xr;
    let mut yr;
    let mut mr;

    let mut xl;
    let mut yl;
    let mut ml;

    let mut dx;

    for _ in 0..max_iter {
        dx = (obj - yn) / m;
        dx = delta * round(dx / delta, FPDecimal::ONE);

        let xk = xn;
        xn += dx;
        // let yk = yn;
        yn = func(xn);

        if xk != xn {
            xr = xn + delta;
            xl = xn - delta;

            yr = func(xr);
            yl = func(xl);

            mr = (yr - yn) / delta;
            ml = (yn - yl) / delta;

            m = (mr + ml) / FPDecimal::TWO;
        } else {
            return Ok(round(xn, abs_error));
        }
    }
    Err(round(xn, abs_error))
}

#[allow(dead_code)]
fn halleys<Func, DFunc, DDFunc>(
    f: Func,
    fd: DFunc,
    fdd: DDFunc,
    mut x0: FPDecimal,
    abs_error: FPDecimal,
    max_iter: usize,
) -> Result<FPDecimal, FPDecimal>
where
    Func: Fn(FPDecimal) -> FPDecimal,
    DFunc: Fn(FPDecimal) -> FPDecimal,
    DDFunc: Fn(FPDecimal) -> FPDecimal,
{
    let mut x1 = x0;
    for _ in 0..max_iter {
        let fxn = f(x1);
        let dfxn = fd(x1);
        let ddfxn = fdd(x1);
        x0 -= FPDecimal::TWO * (fxn * dfxn) / (FPDecimal::TWO * dfxn * dfxn - fxn * ddfxn);
        if (x0 - x1).abs() < abs_error {
            return Ok(x0);
        }
        x1 = x0;
    }
    Err(x0)
}

// TODO: add a discrete halley's method

#[cfg(test)]
mod tests {
    use crate::root_findings::*;
    use crate::FPDecimal;

    #[test]
    fn test_discrete_newton_1() {
        let x0 = FPDecimal::ONE;
        fn func(x0: FPDecimal) -> FPDecimal {
            x0.ln()
        }
        let delta = FPDecimal::ONE;
        let obj = FPDecimal::must_from_str("3.23");
        let max_iter = 300;
        let abs_error = FPDecimal::ONE;
        let output = discrete_newton(func, delta, x0, obj, abs_error, max_iter).unwrap();
        let target = FPDecimal::must_from_str("25");
        assert_eq!(output, target);
    }

    #[test]
    fn test_discrete_newton_2() {
        let x0 = FPDecimal::TEN;
        fn func(x0: FPDecimal) -> FPDecimal {
            x0.pow(FPDecimal::TWO).unwrap()
        }
        let delta = FPDecimal::must_from_str("0.25");
        let obj = FPDecimal::FIVE;
        let max_iter = 300;
        let abs_error = FPDecimal::must_from_str("0.01");
        let output = discrete_newton(func, delta, x0, obj, abs_error, max_iter).unwrap();
        let target = FPDecimal::must_from_str("2.25");
        assert_eq!(output, target);
    }

    #[test]
    fn test_discrete_newton_3() {
        let x0 = FPDecimal::TEN;
        fn func(x0: FPDecimal) -> FPDecimal {
            -x0.pow(FPDecimal::TWO).unwrap() + FPDecimal::TWO * x0 - FPDecimal::ONE
        }
        let delta = FPDecimal::ONE / FPDecimal::THREE;
        let obj = FPDecimal::must_from_str("-15");
        let max_iter = 300;
        let abs_error = FPDecimal::ONE;
        let output = discrete_newton(func, delta, x0, obj, abs_error, max_iter).unwrap();
        let target = FPDecimal::FIVE;
        assert_eq!(output, target);
    }

    #[test]
    fn test_newton_1() {
        fn f(x0: FPDecimal) -> FPDecimal {
            // x0 * x0 * FPDecimal::THREE
            x0 * x0 * x0 - x0 * x0 - FPDecimal::ONE
        }
        fn fd(x0: FPDecimal) -> FPDecimal {
            // FPDecimal::SIX * x0
            FPDecimal::THREE * x0 * x0 - FPDecimal::TWO * x0
        }
        let x0 = FPDecimal::ONE;
        let max_iter = 30;
        let abs_error = FPDecimal::must_from_str("0.00000000001");
        let target = FPDecimal::must_from_str("1.465571231876768027");
        let output = newton(f, fd, x0, abs_error, max_iter).unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn test_newton_2() {
        fn f(x0: FPDecimal) -> FPDecimal {
            x0 * x0 * x0 - x0 * x0 + FPDecimal::TWO
        }
        fn fd(x0: FPDecimal) -> FPDecimal {
            FPDecimal::THREE * x0 * x0 - FPDecimal::TWO * x0
        }
        let x0 = -FPDecimal::must_from_str("20");
        let max_iter = 30;
        let abs_error = FPDecimal::must_from_str("0.00000000001");
        let target = -FPDecimal::ONE;
        let output = newton(f, fd, x0, abs_error, max_iter).unwrap();
        assert_eq!(round(output, FPDecimal::must_from_str("0.0001")), target);
    }

    #[test]
    fn test_halleys_1() {
        fn f(x0: FPDecimal) -> FPDecimal {
            x0 * x0 * x0 - x0 * x0 + FPDecimal::TWO
        }
        fn fd(x0: FPDecimal) -> FPDecimal {
            FPDecimal::THREE * x0 * x0 - FPDecimal::TWO * x0
        }
        fn fdd(x0: FPDecimal) -> FPDecimal {
            FPDecimal::SIX * x0 - FPDecimal::TWO
        }
        let x0 = -FPDecimal::must_from_str("20");
        let max_iter = 30;
        let abs_error = FPDecimal::must_from_str("0.00000000001");
        let target = -FPDecimal::ONE;
        let output = halleys(f, fd, fdd, x0, abs_error, max_iter).unwrap();
        assert_eq!(round(output, FPDecimal::must_from_str("0.0001")), target);
    }

    #[test]
    fn test_halleys_2() {
        fn f(x0: FPDecimal) -> FPDecimal {
            // x0 * x0 * FPDecimal::THREE
            x0 * x0 * x0 - x0 * x0 - FPDecimal::ONE
        }
        fn fd(x0: FPDecimal) -> FPDecimal {
            // FPDecimal::SIX * x0
            FPDecimal::THREE * x0 * x0 - FPDecimal::TWO * x0
        }
        fn fdd(x0: FPDecimal) -> FPDecimal {
            FPDecimal::SIX * x0 - FPDecimal::TWO
        }

        let x0 = FPDecimal::ONE;
        let max_iter = 30;
        let abs_error = FPDecimal::must_from_str("0.00000000001");
        let target = FPDecimal::must_from_str("1.465571231876768026");
        let output = halleys(f, fd, fdd, x0, abs_error, max_iter).unwrap();
        assert_eq!(output, target);
    }
}
