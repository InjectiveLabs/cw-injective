use cosmwasm_std::{OverflowError, OverflowOperation};
use std::cmp::Ordering;
use std::str::FromStr;

/// Exponential functions for FPDecimal
use crate::fp_decimal::{FPDecimal, U256};

impl FPDecimal {
    #[allow(clippy::many_single_char_names)]
    pub fn _exp_taylor_expansion(a: FPDecimal, b: FPDecimal, n: u128) -> FPDecimal {
        //a^b n+1 terms taylor expansion
        assert!(n <= 13u128);
        if n == 0 {
            FPDecimal::ONE
        } else {
            let base = a.ln() * b;
            let mut x = FPDecimal::ONE + base;
            let mut numerator = base;
            let mut denominator = FPDecimal::ONE;
            for i in 2..n + 1 {
                numerator *= base;
                denominator *= FPDecimal::from(i);
                x += numerator / denominator;
            }
            x
        }
    }
    // e^(a)
    pub fn _exp(a: FPDecimal) -> FPDecimal {
        // this throws underflow with a sufficiently large negative exponent
        // short circuit and just return 0 above a certain threshold
        // otherwise if there is a long enough delay between updates on a cluster
        // the penalty function will be bricked
        if a.sign == 0 && a.num >= FPDecimal::from(45i128).num {
            return FPDecimal::zero();
        }
        let mut x = a.num;
        let mut r = FPDecimal::ONE;
        while x >= U256([10, 0, 0, 0]) * FPDecimal::ONE.num {
            x = x - U256([10, 0, 0, 0]) * FPDecimal::ONE.num;
            r = FPDecimal::_mul(r, FPDecimal::E_10);
        }
        if x == FPDecimal::ONE.num {
            let val = FPDecimal::_mul(r, FPDecimal::E);
            if a.sign == 0 {
                return FPDecimal::reciprocal(val);
            }
            return val;
        } else if x == FPDecimal::zero().num {
            let val = r;
            if a.sign == 0 {
                return FPDecimal::reciprocal(val);
            }
            return val;
        }
        let mut tr = FPDecimal::ONE.num;
        let mut d = tr;
        for i in 1..((2 * FPDecimal::DIGITS + 1) as u64) {
            d = (d * x) / (FPDecimal::ONE.num * U256([i, 0, 0, 0]));
            tr = tr + d;
        }
        let val = FPDecimal::_mul(FPDecimal { num: tr, sign: 1 }, r);
        if a.sign == 0 {
            return FPDecimal::reciprocal(val);
        }
        val
    }

    // a^(0.5)
    pub fn _sqrt(a: FPDecimal) -> Option<FPDecimal> {
        const MAX_ITERATIONS: i64 = 300;

        if a < FPDecimal::zero() {
            return None;
        }

        if a.is_zero() {
            return Some(FPDecimal::zero());
        }

        // Start with an arbitrary number as the first guess
        let mut r = a / FPDecimal::TWO;
        let mut l = r + FPDecimal::one();

        // Keep going while the difference is larger than the tolerance
        let mut c = 0i64;
        while (l != r) && (c < MAX_ITERATIONS) {
            l = r;
            r = (r + a / r) / FPDecimal::TWO;
            c += 1;
        }

        Some(r)
    }
    pub fn sqrt(self) -> FPDecimal {
        match FPDecimal::_sqrt(self) {
            Some(value) => value,
            None => panic!("Undefined behavior"),
        }
    }

    pub fn checked_positive_pow(self, exponent: FPDecimal) -> Result<FPDecimal, OverflowError> {
        {
            // This uses the exponentiation by squaring algorithm:
            // https://en.wikipedia.org/wiki/Exponentiation_by_squaring#Basic_method

            if self == FPDecimal::zero() {
                return Ok(FPDecimal::zero());
            }
            if self > FPDecimal::zero() && exponent == FPDecimal::zero() {
                return Ok(FPDecimal::one());
            }

            if exponent > FPDecimal::from(60u128) {
                return Err(OverflowError::new(OverflowOperation::Pow, self.to_string(), exponent.to_string()));
            }

            if self == FPDecimal::E {
                if exponent == FPDecimal::ONE.ln() {
                    return Ok(FPDecimal::ONE);
                }
                if exponent == FPDecimal::TWO.ln() {
                    return Ok(FPDecimal::TWO);
                }
                if exponent == FPDecimal::THREE.ln() {
                    return Ok(FPDecimal::THREE);
                }
                if exponent == FPDecimal::FOUR.ln() {
                    return Ok(FPDecimal::FOUR);
                }
                if exponent == FPDecimal::FIVE.ln() {
                    return Ok(FPDecimal::FIVE);
                }
                if exponent == FPDecimal::SIX.ln() {
                    return Ok(FPDecimal::SIX);
                }
                if exponent == FPDecimal::SEVEN.ln() {
                    return Ok(FPDecimal::SEVEN);
                }
                if exponent == FPDecimal::EIGHT.ln() {
                    return Ok(FPDecimal::EIGHT);
                }
                if exponent == FPDecimal::NINE.ln() {
                    return Ok(FPDecimal::NINE);
                }
                if exponent == FPDecimal::TEN.ln() {
                    return Ok(FPDecimal::TEN);
                }
                if exponent == (FPDecimal::ONE / FPDecimal::TWO).ln() {
                    return Ok(FPDecimal::ONE / FPDecimal::TWO);
                }
                if exponent == (FPDecimal::ONE / FPDecimal::THREE).ln() {
                    return Ok(FPDecimal::ONE / FPDecimal::THREE);
                }
                if exponent == (FPDecimal::ONE / FPDecimal::FOUR).ln() {
                    return Ok(FPDecimal::ONE / FPDecimal::FOUR);
                }
                if exponent == (FPDecimal::ONE / FPDecimal::FIVE).ln() {
                    return Ok(FPDecimal::ONE / FPDecimal::FIVE);
                }
                if exponent == (FPDecimal::ONE / FPDecimal::SIX).ln() {
                    return Ok(FPDecimal::ONE / FPDecimal::SIX);
                }
                if exponent == (FPDecimal::ONE / FPDecimal::SEVEN).ln() {
                    return Ok(FPDecimal::ONE / FPDecimal::SEVEN);
                }
                if exponent == (FPDecimal::ONE / FPDecimal::EIGHT).ln() {
                    return Ok(FPDecimal::ONE / FPDecimal::EIGHT);
                }
                if exponent == (FPDecimal::ONE / FPDecimal::NINE).ln() {
                    return Ok(FPDecimal::ONE / FPDecimal::NINE);
                }
                if exponent == (FPDecimal::ONE / FPDecimal::TEN).ln() {
                    return Ok(FPDecimal::ONE / FPDecimal::TEN);
                }
            }

            if self == FPDecimal::from(10u128) {
                if exponent == FPDecimal::one() {
                    return Ok(FPDecimal::from(10u128));
                }
                if exponent == FPDecimal::TWO {
                    return Ok(FPDecimal::from(100u128));
                }
                if exponent == FPDecimal::THREE {
                    return Ok(FPDecimal::from(1000u128));
                }
                if exponent == FPDecimal::FOUR {
                    return Ok(FPDecimal::from(10000u128));
                }
                if exponent == FPDecimal::FIVE {
                    return Ok(FPDecimal::from(100000u128));
                }
                if exponent == FPDecimal::SIX {
                    return Ok(FPDecimal::from(1000000u128));
                }
                if exponent == FPDecimal::SEVEN {
                    return Ok(FPDecimal::from(10000000u128));
                }
                if exponent == FPDecimal::EIGHT {
                    return Ok(FPDecimal::from(100000000u128));
                }
                if exponent == FPDecimal::NINE {
                    return Ok(FPDecimal::from(1000000000u128));
                }
                if exponent == FPDecimal::TEN {
                    return Ok(FPDecimal::from(10000000000u128));
                }
                if exponent == FPDecimal::from(11u128) {
                    return Ok(FPDecimal::from(100000000000u128));
                }
                if exponent == FPDecimal::from(12u128) {
                    return Ok(FPDecimal::from(1000000000000u128));
                }
                if exponent == FPDecimal::from(13u128) {
                    return Ok(FPDecimal::from(10000000000000u128));
                }
                if exponent == FPDecimal::from(14u128) {
                    return Ok(FPDecimal::from(100000000000000u128));
                }
                if exponent == FPDecimal::from(15u128) {
                    return Ok(FPDecimal::from(1000000000000000u128));
                }
                if exponent == FPDecimal::from(16u128) {
                    return Ok(FPDecimal::from(10000000000000000u128));
                }
                if exponent == FPDecimal::from(17u128) {
                    return Ok(FPDecimal::from(100000000000000000u128));
                }
                if exponent == FPDecimal::from(18u128) {
                    return Ok(FPDecimal::from(1000000000000000000u128));
                }
                if exponent == FPDecimal::from(19u128) {
                    return Ok(FPDecimal::from(10000000000000000000u128));
                }
                if exponent == FPDecimal::from(20u128) {
                    return Ok(FPDecimal::from(100000000000000000000u128));
                }
                if exponent == FPDecimal::NEGATIVE_ONE {
                    return Ok(FPDecimal::from_str("0.1").unwrap());
                }
                if exponent == FPDecimal::from_str("-2").unwrap() {
                    return Ok(FPDecimal::from_str("0.01").unwrap());
                }
                if exponent == FPDecimal::from_str("-3").unwrap() {
                    return Ok(FPDecimal::from_str("0.001").unwrap());
                }
                if exponent == FPDecimal::from_str("-4").unwrap() {
                    return Ok(FPDecimal::from_str("0.0001").unwrap());
                }
                if exponent == FPDecimal::from_str("-5").unwrap() {
                    return Ok(FPDecimal::from_str("0.00001").unwrap());
                }
                if exponent == FPDecimal::from_str("-6").unwrap() {
                    return Ok(FPDecimal::from_str("0.000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-7").unwrap() {
                    return Ok(FPDecimal::from_str("0.0000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-8").unwrap() {
                    return Ok(FPDecimal::from_str("0.00000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-9").unwrap() {
                    return Ok(FPDecimal::from_str("0.000000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-10").unwrap() {
                    return Ok(FPDecimal::from_str("0.0000000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-11").unwrap() {
                    return Ok(FPDecimal::from_str("0.00000000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-12").unwrap() {
                    return Ok(FPDecimal::from_str("0.000000000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-13").unwrap() {
                    return Ok(FPDecimal::from_str("0.0000000000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-14").unwrap() {
                    return Ok(FPDecimal::from_str("0.00000000000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-15").unwrap() {
                    return Ok(FPDecimal::from_str("0.000000000000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-16").unwrap() {
                    return Ok(FPDecimal::from_str("0.0000000000000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-17").unwrap() {
                    return Ok(FPDecimal::from_str("0.00000000000000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-18").unwrap() {
                    return Ok(FPDecimal::from_str("0.000000000000000001").unwrap());
                }
                if exponent < FPDecimal::from_str("-18").unwrap() {
                    return Ok(FPDecimal::zero());
                }
                if exponent == FPDecimal::from(21u128) {
                    return Ok(FPDecimal::from(1000000000000000000000u128));
                }
                if exponent == FPDecimal::from(22u128) {
                    return Ok(FPDecimal::from(10000000000000000000000u128));
                }
                if exponent == FPDecimal::from(23u128) {
                    return Ok(FPDecimal::from(100000000000000000000000u128));
                }
                if exponent == FPDecimal::from(24u128) {
                    return Ok(FPDecimal::from(1000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(25u128) {
                    return Ok(FPDecimal::from(10000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(26u128) {
                    return Ok(FPDecimal::from(100000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(27u128) {
                    return Ok(FPDecimal::from(1000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(28u128) {
                    return Ok(FPDecimal::from(10000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(29u128) {
                    return Ok(FPDecimal::from(100000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(30u128) {
                    return Ok(FPDecimal::from(1000000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(31u128) {
                    return Ok(FPDecimal::from(10000000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(32u128) {
                    return Ok(FPDecimal::from(100000000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(33u128) {
                    return Ok(FPDecimal::from(1000000000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(34u128) {
                    return Ok(FPDecimal::from(10000000000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(35u128) {
                    return Ok(FPDecimal::from(100000000000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(36u128) {
                    return Ok(FPDecimal::from(1000000000000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(37u128) {
                    return Ok(FPDecimal::from(10000000000000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(38u128) {
                    return Ok(FPDecimal::from(100000000000000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(39u128) {
                    return Ok(FPDecimal::from_str("1000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(40u128) {
                    return Ok(FPDecimal::from_str("10000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(41u128) {
                    return Ok(FPDecimal::from_str("100000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(42u128) {
                    return Ok(FPDecimal::from_str("1000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(43u128) {
                    return Ok(FPDecimal::from_str("10000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(44u128) {
                    return Ok(FPDecimal::from_str("100000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(45u128) {
                    return Ok(FPDecimal::from_str("1000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(46u128) {
                    return Ok(FPDecimal::from_str("10000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(47u128) {
                    return Ok(FPDecimal::from_str("100000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(48u128) {
                    return Ok(FPDecimal::from_str("1000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(49u128) {
                    return Ok(FPDecimal::from_str("10000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(50u128) {
                    return Ok(FPDecimal::from_str("100000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(51u128) {
                    return Ok(FPDecimal::from_str("1000000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(52u128) {
                    return Ok(FPDecimal::from_str("10000000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(53u128) {
                    return Ok(FPDecimal::from_str("100000000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(54u128) {
                    return Ok(FPDecimal::from_str("1000000000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(55u128) {
                    return Ok(FPDecimal::from_str("10000000000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(56u128) {
                    return Ok(FPDecimal::from_str("100000000000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(57u128) {
                    return Ok(FPDecimal::from_str("1000000000000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(58u128) {
                    return Ok(FPDecimal::from_str("10000000000000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(59u128) {
                    return Ok(FPDecimal::from_str("100000000000000000000000000000000000000000000000000000000000").unwrap());
                }
            }

            fn compute_exponentiation(base: FPDecimal, mut exponent: FPDecimal) -> Result<FPDecimal, OverflowError> {
                // base^exponent
                // NOTE: only accurate for 1,3,5,7,11, and combinations of these numbers
                // 14 terms taylor expansion provides a good enough approximation
                const N_TERMS: u128 = 13;
                match exponent.cmp(&FPDecimal::ZERO) {
                    Ordering::Equal => Ok(FPDecimal::one()),
                    // Negative exponent
                    Ordering::Less => {
                        exponent = -exponent;
                        match exponent.cmp(&(FPDecimal::ONE)) {
                            Ordering::Equal => Ok(FPDecimal::ONE / base),
                            Ordering::Less => compute_negative_exponent_less_one(base, exponent, N_TERMS),
                            Ordering::Greater => compute_negative_exponent_greater_one(base, exponent, N_TERMS),
                        }
                    }
                    // Positive exponent
                    Ordering::Greater => match exponent.cmp(&FPDecimal::ONE) {
                        Ordering::Equal => Ok(base),
                        Ordering::Less => compute_positive_exponent_less_one(base, exponent, N_TERMS),
                        Ordering::Greater => compute_positive_exponent_greater_one(base, exponent, N_TERMS),
                    },
                }
            }

            fn compute_negative_exponent_greater_one(mut base: FPDecimal, exponent: FPDecimal, n_terms: u128) -> Result<FPDecimal, OverflowError> {
                let mut int_b = exponent.int();
                let rem_b = exponent - int_b;
                let mut float_exp = FPDecimal::ONE;
                if rem_b != FPDecimal::ZERO {
                    float_exp = FPDecimal::_exp_taylor_expansion(FPDecimal::ONE / base, rem_b, n_terms);
                }
                let mut tmp_a = FPDecimal::ONE;
                while int_b > FPDecimal::one() {
                    if int_b.num % FPDecimal::TWO.num == FPDecimal::ONE.num {
                        tmp_a = base * tmp_a;
                        int_b -= FPDecimal::ONE;
                    }
                    base = base * base;
                    int_b /= FPDecimal::TWO;
                }
                base *= tmp_a;
                Ok(FPDecimal::ONE / base * float_exp)
            }

            fn negative_exponent_check_basic_log(
                mut base: FPDecimal,
                exponent: FPDecimal,
                reciprocal: FPDecimal,
                log_base: FPDecimal,
            ) -> Option<FPDecimal> {
                let mut temp_exponent_reciprocal = FPDecimal::reciprocal(exponent).int();

                let abs_difference: FPDecimal = FPDecimal::must_from_str("0.0000001");
                // reciprocal is odd
                if ((FPDecimal::reciprocal(exponent) % FPDecimal::TWO).int() - FPDecimal::ONE).abs() <= abs_difference {
                    while temp_exponent_reciprocal > FPDecimal::ONE {
                        base /= log_base;
                        temp_exponent_reciprocal -= FPDecimal::ONE;
                    }
                    return Some(FPDecimal::ONE / base);
                }
                // reciprocal is even
                if reciprocal % FPDecimal::TWO == FPDecimal::ZERO {
                    while temp_exponent_reciprocal > FPDecimal::ONE {
                        base = base.sqrt();
                        temp_exponent_reciprocal /= FPDecimal::TWO;
                    }
                    return Some(FPDecimal::ONE / base);
                }
                None
            }

            type BaseCheckFunction<'a> = (&'a dyn Fn(FPDecimal) -> Option<FPDecimal>, FPDecimal);

            fn compute_negative_exponent_less_one(base: FPDecimal, exponent: FPDecimal, n_terms: u128) -> Result<FPDecimal, OverflowError> {
                // NOTE: only accurate for 1,3,5,7,11, and combinations of these numbers
                let abs_error = FPDecimal::must_from_str("0.00000000000000001");
                let reciprocal = if (FPDecimal::reciprocal(exponent) - FPDecimal::reciprocal(exponent).int()).abs() < abs_error {
                    FPDecimal::reciprocal(exponent).int()
                } else {
                    FPDecimal::reciprocal(exponent)
                };

                let base_checks: Vec<BaseCheckFunction> = vec![
                    (&FPDecimal::_log_e, FPDecimal::E),
                    (&FPDecimal::_log2, FPDecimal::TWO),
                    (&FPDecimal::_log3, FPDecimal::THREE),
                    (&FPDecimal::_log5, FPDecimal::FIVE),
                    (&FPDecimal::_log7, FPDecimal::SEVEN),
                    (&FPDecimal::_log10, FPDecimal::TEN),
                    (&FPDecimal::_log11, FPDecimal::from(11u128)),
                ];

                for (log_fn, divisor) in base_checks {
                    if log_fn(base).is_some() {
                        if log_fn(base).unwrap().abs() >= reciprocal {
                            if let Some(value) = negative_exponent_check_basic_log(base, exponent, reciprocal, divisor) {
                                return Ok(value);
                            }
                        } else {
                            let multiplier = log_fn(base).unwrap();
                            return Ok(multiplier * divisor.ln() / reciprocal);
                        }
                    }
                }

                Ok(FPDecimal::_exp_taylor_expansion(FPDecimal::ONE / base, exponent, n_terms))
            }

            fn positive_exponent_check_basic_log(
                mut base: FPDecimal,
                exponent: FPDecimal,
                reciprocal: FPDecimal,
                log_base: FPDecimal,
            ) -> Option<FPDecimal> {
                let mut temp_b = FPDecimal::reciprocal(exponent).int();
                let abs_difference: FPDecimal = FPDecimal::must_from_str("0.0000001");

                // odd
                if ((reciprocal % FPDecimal::TWO).int() - FPDecimal::ONE).abs() <= abs_difference {
                    while temp_b > FPDecimal::ONE {
                        base /= log_base;
                        temp_b -= FPDecimal::ONE;
                    }
                    return Some(base);
                };

                // even
                if reciprocal % FPDecimal::TWO == FPDecimal::ZERO {
                    while temp_b > FPDecimal::ONE {
                        base = base.sqrt();
                        temp_b /= FPDecimal::TWO;
                    }
                    return Some(base);
                };
                None
            }

            fn compute_positive_exponent_less_one(base: FPDecimal, exponent: FPDecimal, n_terms: u128) -> Result<FPDecimal, OverflowError> {
                // taylor expansion approximation of exponentiation computation with float number exponent
                // NOTE: only accurate for 1,3,5,7,11, and combinations of these numbers
                let abs_error = FPDecimal::must_from_str("0.00000000000000001");
                let reciprocal = if (FPDecimal::reciprocal(exponent) - FPDecimal::reciprocal(exponent).int()).abs() < abs_error {
                    FPDecimal::reciprocal(exponent).int()
                } else {
                    FPDecimal::reciprocal(exponent)
                };
                let base_checks: Vec<BaseCheckFunction> = vec![
                    (&FPDecimal::_log_e, FPDecimal::E),
                    (&FPDecimal::_log2, FPDecimal::TWO),
                    (&FPDecimal::_log3, FPDecimal::THREE),
                    (&FPDecimal::_log5, FPDecimal::FIVE),
                    (&FPDecimal::_log7, FPDecimal::SEVEN),
                    (&FPDecimal::_log10, FPDecimal::TEN),
                    (&FPDecimal::_log11, FPDecimal::from(11u128)),
                ];

                for (log_fn, divisor) in base_checks {
                    if log_fn(base).is_some() && log_fn(base).unwrap().abs() >= reciprocal {
                        if let Some(value) = positive_exponent_check_basic_log(base, exponent, reciprocal, divisor) {
                            return Ok(value);
                        }
                    }
                }

                Ok(FPDecimal::_exp_taylor_expansion(base, exponent, n_terms))
            }

            fn compute_integer_exponentiation(mut base: FPDecimal, mut exponent: FPDecimal) -> FPDecimal {
                let mut temp_base = FPDecimal::ONE;

                while exponent > FPDecimal::one() {
                    if exponent.num % FPDecimal::TWO.num == FPDecimal::ONE.num {
                        temp_base = base * temp_base;
                        exponent -= FPDecimal::ONE;
                    }

                    base = base * base;
                    exponent /= FPDecimal::TWO;
                }

                base * temp_base
            }

            fn compute_positive_exponent_greater_one(mut base: FPDecimal, exponent: FPDecimal, n_terms: u128) -> Result<FPDecimal, OverflowError> {
                let integer_part_of_exponent = exponent.int();
                let fractional_part_of_exponent = exponent - integer_part_of_exponent;

                let fractional_exponentiation = if fractional_part_of_exponent != FPDecimal::ZERO {
                    FPDecimal::_exp_taylor_expansion(base, fractional_part_of_exponent, n_terms)
                } else {
                    FPDecimal::ONE
                };

                base = compute_integer_exponentiation(base, integer_part_of_exponent);

                Ok(base * fractional_exponentiation)
            }

            compute_exponentiation(self, exponent).map_err(|_| OverflowError {
                operation: OverflowOperation::Pow,
                operand1: self.to_string(),
                operand2: exponent.to_string(),
            })
        }
    }

    pub fn checked_negative_pow(self, exponent: FPDecimal) -> Result<FPDecimal, OverflowError> {
        {
            // This uses the exponentiation by squaring algorithm:
            // https://en.wikipedia.org/wiki/Exponentiation_by_squaring#Basic_method

            if self == FPDecimal::zero() {
                return Ok(FPDecimal::zero());
            }
            if self.is_negative() && exponent == FPDecimal::zero() {
                return Ok(FPDecimal::NEGATIVE_ONE);
            }
            if exponent > FPDecimal::from(60u128) {
                return Err(OverflowError::new(OverflowOperation::Pow, self.to_string(), exponent.to_string()));
            }
            if self == -FPDecimal::E {
                let e = FPDecimal::E;
                if exponent == FPDecimal::ONE {
                    return Ok(-e);
                }
                if exponent == FPDecimal::TWO {
                    return Ok(e * e);
                }
                if exponent == FPDecimal::THREE {
                    return Ok(-e * e * e);
                }
                if exponent == FPDecimal::FOUR {
                    return Ok(e * e * e * e);
                }
                if exponent == FPDecimal::FIVE {
                    return Ok(-e * e * e * e * e);
                }
                if exponent == FPDecimal::SIX {
                    return Ok(e * e * e * e * e * e);
                }
                if exponent == FPDecimal::SEVEN {
                    return Ok(-e * e * e * e * e * e * e);
                }
                if exponent == FPDecimal::EIGHT {
                    return Ok(e * e * e * e * e * e * e * e);
                }
                if exponent == FPDecimal::NINE {
                    return Ok(-e * e * e * e * e * e * e * e * e);
                }
                if exponent == FPDecimal::TEN {
                    return Ok(e * e * e * e * e * e * e * e * e * e);
                }
                if exponent == -FPDecimal::TWO {
                    return Ok((FPDecimal::ONE) / (e * e));
                }
                if exponent == -FPDecimal::THREE {
                    return Ok(-(FPDecimal::ONE) / (e * e * e));
                }
                if exponent == -FPDecimal::FOUR {
                    return Ok((FPDecimal::ONE) / (e * e * e * e));
                }
                if exponent == -FPDecimal::FIVE {
                    return Ok(-(FPDecimal::ONE) / (e * e * e * e * e));
                }
                if exponent == -FPDecimal::SIX {
                    return Ok((FPDecimal::ONE) / (e * e * e * e * e * e));
                }
                if exponent == -FPDecimal::SEVEN {
                    return Ok(-(FPDecimal::ONE) / (e * e * e * e * e * e * e));
                }
                if exponent == -FPDecimal::EIGHT {
                    return Ok((FPDecimal::ONE) / (e * e * e * e * e * e * e * e));
                }
                if exponent == -FPDecimal::NINE {
                    return Ok(-(FPDecimal::ONE) / (e * e * e * e * e * e * e * e * e));
                }
                if exponent == -FPDecimal::TEN {
                    return Ok((FPDecimal::ONE) / (e * e * e * e * e * e * e * e * e * e));
                }
            }

            if self == -FPDecimal::from(10u128) {
                if exponent == FPDecimal::ONE {
                    return Ok(-FPDecimal::from(10u128));
                }
                if exponent == FPDecimal::TWO {
                    return Ok(FPDecimal::from(100u128));
                }
                if exponent == FPDecimal::THREE {
                    return Ok(-FPDecimal::from(1000u128));
                }
                if exponent == FPDecimal::FOUR {
                    return Ok(-FPDecimal::from(10000u128));
                }
                if exponent == FPDecimal::FIVE {
                    return Ok(-FPDecimal::from(100000u128));
                }
                if exponent == FPDecimal::SIX {
                    return Ok(FPDecimal::from(1000000u128));
                }
                if exponent == FPDecimal::SEVEN {
                    return Ok(-FPDecimal::from(10000000u128));
                }
                if exponent == FPDecimal::EIGHT {
                    return Ok(FPDecimal::from(100000000u128));
                }
                if exponent == FPDecimal::NINE {
                    return Ok(-FPDecimal::from(1000000000u128));
                }
                if exponent == FPDecimal::from(10u128) {
                    return Ok(FPDecimal::from(10000000000u128));
                }
                if exponent == FPDecimal::from(11u128) {
                    return Ok(-FPDecimal::from(100000000000u128));
                }
                if exponent == FPDecimal::from(12u128) {
                    return Ok(FPDecimal::from(1000000000000u128));
                }
                if exponent == FPDecimal::from(13u128) {
                    return Ok(-FPDecimal::from(10000000000000u128));
                }
                if exponent == FPDecimal::from(14u128) {
                    return Ok(FPDecimal::from(100000000000000u128));
                }
                if exponent == FPDecimal::from(15u128) {
                    return Ok(-FPDecimal::from(1000000000000000u128));
                }
                if exponent == FPDecimal::from(16u128) {
                    return Ok(FPDecimal::from(10000000000000000u128));
                }
                if exponent == FPDecimal::from(17u128) {
                    return Ok(-FPDecimal::from(100000000000000000u128));
                }
                if exponent == FPDecimal::from(18u128) {
                    return Ok(FPDecimal::from(1000000000000000000u128));
                }
                if exponent == FPDecimal::from(19u128) {
                    return Ok(-FPDecimal::from(10000000000000000000u128));
                }
                if exponent == FPDecimal::from(20u128) {
                    return Ok(FPDecimal::from(100000000000000000000u128));
                }
                if exponent == FPDecimal::NEGATIVE_ONE {
                    return Ok(-FPDecimal::from_str("0.1").unwrap());
                }
                if exponent == FPDecimal::from_str("-2").unwrap() {
                    return Ok(FPDecimal::from_str("0.01").unwrap());
                }
                if exponent == FPDecimal::from_str("-3").unwrap() {
                    return Ok(-FPDecimal::from_str("0.001").unwrap());
                }
                if exponent == FPDecimal::from_str("-4").unwrap() {
                    return Ok(FPDecimal::from_str("0.0001").unwrap());
                }
                if exponent == FPDecimal::from_str("-5").unwrap() {
                    return Ok(-FPDecimal::from_str("0.00001").unwrap());
                }
                if exponent == FPDecimal::from_str("-6").unwrap() {
                    return Ok(FPDecimal::from_str("0.000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-7").unwrap() {
                    return Ok(-FPDecimal::from_str("0.0000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-8").unwrap() {
                    return Ok(FPDecimal::from_str("0.00000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-9").unwrap() {
                    return Ok(-FPDecimal::from_str("0.000000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-10").unwrap() {
                    return Ok(FPDecimal::from_str("0.0000000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-11").unwrap() {
                    return Ok(-FPDecimal::from_str("0.00000000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-12").unwrap() {
                    return Ok(FPDecimal::from_str("0.000000000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-13").unwrap() {
                    return Ok(-FPDecimal::from_str("0.0000000000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-14").unwrap() {
                    return Ok(FPDecimal::from_str("0.00000000000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-15").unwrap() {
                    return Ok(-FPDecimal::from_str("0.000000000000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-16").unwrap() {
                    return Ok(FPDecimal::from_str("0.0000000000000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-17").unwrap() {
                    return Ok(-FPDecimal::from_str("0.00000000000000001").unwrap());
                }
                if exponent == FPDecimal::from_str("-18").unwrap() {
                    return Ok(FPDecimal::from_str("0.000000000000000001").unwrap());
                }
                if exponent < FPDecimal::from_str("-18").unwrap() {
                    return Ok(FPDecimal::zero());
                }

                if exponent == FPDecimal::from(21u128) {
                    return Ok(-FPDecimal::from(1000000000000000000000u128));
                }
                if exponent == FPDecimal::from(22u128) {
                    return Ok(-FPDecimal::from(10000000000000000000000u128));
                }
                if exponent == FPDecimal::from(23u128) {
                    return Ok(-FPDecimal::from(100000000000000000000000u128));
                }
                if exponent == FPDecimal::from(24u128) {
                    return Ok(FPDecimal::from(1000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(25u128) {
                    return Ok(-FPDecimal::from(10000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(26u128) {
                    return Ok(FPDecimal::from(100000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(27u128) {
                    return Ok(-FPDecimal::from(1000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(28u128) {
                    return Ok(FPDecimal::from(10000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(29u128) {
                    return Ok(-FPDecimal::from(100000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(30u128) {
                    return Ok(FPDecimal::from(1000000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(31u128) {
                    return Ok(-FPDecimal::from(10000000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(32u128) {
                    return Ok(FPDecimal::from(100000000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(33u128) {
                    return Ok(-FPDecimal::from(1000000000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(34u128) {
                    return Ok(FPDecimal::from(10000000000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(35u128) {
                    return Ok(-FPDecimal::from(100000000000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(36u128) {
                    return Ok(FPDecimal::from(1000000000000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(37u128) {
                    return Ok(-FPDecimal::from(10000000000000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(38u128) {
                    return Ok(FPDecimal::from(100000000000000000000000000000000000000u128));
                }
                if exponent == FPDecimal::from(39u128) {
                    return Ok(-FPDecimal::from_str("1000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(40u128) {
                    return Ok(FPDecimal::from_str("10000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(41u128) {
                    return Ok(-FPDecimal::from_str("100000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(42u128) {
                    return Ok(FPDecimal::from_str("1000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(43u128) {
                    return Ok(-FPDecimal::from_str("10000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(44u128) {
                    return Ok(FPDecimal::from_str("100000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(45u128) {
                    return Ok(-FPDecimal::from_str("1000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(46u128) {
                    return Ok(FPDecimal::from_str("10000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(47u128) {
                    return Ok(-FPDecimal::from_str("100000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(48u128) {
                    return Ok(FPDecimal::from_str("1000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(49u128) {
                    return Ok(-FPDecimal::from_str("10000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(50u128) {
                    return Ok(FPDecimal::from_str("100000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(51u128) {
                    return Ok(-FPDecimal::from_str("1000000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(52u128) {
                    return Ok(FPDecimal::from_str("10000000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(53u128) {
                    return Ok(-FPDecimal::from_str("100000000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(54u128) {
                    return Ok(FPDecimal::from_str("1000000000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(55u128) {
                    return Ok(-FPDecimal::from_str("10000000000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(56u128) {
                    return Ok(FPDecimal::from_str("100000000000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(57u128) {
                    return Ok(-FPDecimal::from_str("1000000000000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(58u128) {
                    return Ok(FPDecimal::from_str("10000000000000000000000000000000000000000000000000000000000").unwrap());
                }
                if exponent == FPDecimal::from(59u128) {
                    return Ok(-FPDecimal::from_str("100000000000000000000000000000000000000000000000000000000000").unwrap());
                }
            }

            fn compute_exponentiation(base: FPDecimal, mut exponent: FPDecimal) -> Result<FPDecimal, OverflowError> {
                // a^b
                // 14 terms taylor expansion provides a good enough approximation
                const N_TERMS: u128 = 13;
                match exponent.cmp(&FPDecimal::ZERO) {
                    Ordering::Equal => Ok(FPDecimal::one()),
                    Ordering::Less => {
                        exponent = -exponent;
                        match exponent.cmp(&(FPDecimal::ONE)) {
                            Ordering::Equal => Ok(FPDecimal::ONE / base),
                            Ordering::Less => compute_negative_exponent_less_one(FPDecimal::ONE / base, exponent, N_TERMS),
                            Ordering::Greater => compute_negative_exponent_greater_one(FPDecimal::ONE / base, exponent, N_TERMS),
                        }
                    }
                    Ordering::Greater => match exponent.cmp(&FPDecimal::ONE) {
                        Ordering::Equal => Ok(base),
                        Ordering::Less => compute_positive_exponent_less_one(base, exponent, N_TERMS),
                        Ordering::Greater => compute_positive_exponent_greater_one(base, exponent),
                    },
                }
            }

            type BaseCheckFunction<'a> = (&'a dyn Fn(FPDecimal) -> Option<FPDecimal>, FPDecimal);

            fn compute_negative_exponent_less_one(mut base: FPDecimal, exponent: FPDecimal, n_terms: u128) -> Result<FPDecimal, OverflowError> {
                // NOTE: only accurate for 1,3,5,7,11, and combinations of these numbers
                base = -base;

                let base_checks: Vec<BaseCheckFunction> = vec![
                    (&FPDecimal::_log2, FPDecimal::TWO),
                    (&FPDecimal::_log_e, FPDecimal::E),
                    (&FPDecimal::_log3, FPDecimal::THREE),
                    (&FPDecimal::_log5, FPDecimal::FIVE),
                    (&FPDecimal::_log7, FPDecimal::SEVEN),
                    (&FPDecimal::_log10, FPDecimal::TEN),
                ];

                for (log_fn, divisor) in base_checks {
                    if log_fn(base).is_some() && check_conditions_and_return_negative(base, divisor, &exponent) {
                        return Ok(-base);
                    }
                }

                // Handling log11 case separately
                if (base)._log11().is_some() && check_conditions_and_return_negative(base, FPDecimal::from(11u128), &exponent) {
                    return Ok(-FPDecimal::ONE / base);
                }

                Ok(FPDecimal::_exp_taylor_expansion(FPDecimal::ONE / base, exponent, n_terms))
            }

            fn check_conditions_and_return_negative(mut base: FPDecimal, divisor: FPDecimal, exponent: &FPDecimal) -> bool {
                let abs_error = FPDecimal::must_from_str("0.00000000000000001");
                let reciprocal = if (FPDecimal::reciprocal(*exponent) - FPDecimal::reciprocal(*exponent).int()).abs() < abs_error {
                    FPDecimal::reciprocal(*exponent).int()
                } else {
                    FPDecimal::reciprocal(*exponent)
                };

                if reciprocal % FPDecimal::TWO == FPDecimal::ZERO {
                    panic!("No complex number");
                };

                if ((FPDecimal::reciprocal(*exponent) % FPDecimal::TWO).int() - FPDecimal::ONE).abs() <= FPDecimal::must_from_str("0.000001") {
                    let mut tmp_b = FPDecimal::reciprocal(*exponent).int();
                    while tmp_b > FPDecimal::ONE {
                        base /= divisor;
                        tmp_b -= FPDecimal::ONE;
                    }
                    true
                } else {
                    false
                }
            }

            fn compute_negative_exponent_greater_one(mut base: FPDecimal, exponent: FPDecimal, n_terms: u128) -> Result<FPDecimal, OverflowError> {
                let integer_part_of_exponent = exponent.int();
                let fractional_part_of_exponent = exponent - integer_part_of_exponent;

                let fractional_exponentiation = if fractional_part_of_exponent != FPDecimal::ZERO {
                    FPDecimal::_exp_taylor_expansion(FPDecimal::ONE / base, fractional_part_of_exponent, n_terms)
                } else {
                    FPDecimal::ONE
                };
                base = compute_integer_exponentiation(base, integer_part_of_exponent);
                Ok(FPDecimal::ONE / base * fractional_exponentiation)
            }

            fn compute_positive_exponent_less_one(mut base: FPDecimal, exponent: FPDecimal, n_terms: u128) -> Result<FPDecimal, OverflowError> {
                // taylor expansion approximation of exponentiation computation with float number exponent
                // NOTE: only accurate for 1,3,5,7,11, and combinations of these numbers
                base = -base;

                // Define a list of base-check functions and their corresponding values
                let base_checks: Vec<BaseCheckFunction> = vec![
                    (&FPDecimal::_log2, FPDecimal::TWO),
                    (&FPDecimal::_log_e, FPDecimal::E),
                    (&FPDecimal::_log3, FPDecimal::THREE),
                    (&FPDecimal::_log5, FPDecimal::FIVE),
                    (&FPDecimal::_log7, FPDecimal::SEVEN),
                    (&FPDecimal::_log10, FPDecimal::TEN),
                    (&FPDecimal::_log11, FPDecimal::from(11u128)),
                ];

                for (log_fn, divisor) in &base_checks {
                    if log_fn(base).is_some() && check_conditions_and_return(&mut base, divisor, &exponent) {
                        return Ok(-base);
                    }
                }

                Ok(FPDecimal::_exp_taylor_expansion(base, exponent, n_terms))
            }

            fn check_conditions_and_return(base: &mut FPDecimal, divisor: &FPDecimal, exponent: &FPDecimal) -> bool {
                if FPDecimal::reciprocal(*exponent) % FPDecimal::TWO == FPDecimal::ZERO {
                    panic!("No complex number");
                };

                if ((FPDecimal::reciprocal(*exponent) % FPDecimal::TWO).int() - FPDecimal::ONE).abs() <= FPDecimal::must_from_str("0.000001") {
                    let mut tmp_b = FPDecimal::reciprocal(*exponent).int();
                    while tmp_b > FPDecimal::ONE {
                        *base /= *divisor;
                        tmp_b -= FPDecimal::ONE;
                    }
                    true
                } else {
                    false
                }
            }

            fn compute_integer_exponentiation(mut base: FPDecimal, mut exponent: FPDecimal) -> FPDecimal {
                let mut temp_base = FPDecimal::ONE;
                while exponent > FPDecimal::one() {
                    if exponent.num % FPDecimal::TWO.num == FPDecimal::ONE.num {
                        temp_base = base * temp_base;
                        exponent -= FPDecimal::ONE;
                    }
                    base = base * base;
                    exponent /= FPDecimal::TWO;
                }
                base * temp_base
            }

            fn compute_positive_exponent_greater_one(base: FPDecimal, exponent: FPDecimal) -> Result<FPDecimal, OverflowError> {
                let integer_part_of_exponent = exponent.int();
                let fractional_part_of_exponent = exponent - integer_part_of_exponent;
                if fractional_part_of_exponent != FPDecimal::ZERO {
                    panic!("No complex number");
                }
                Ok(compute_integer_exponentiation(base, integer_part_of_exponent))
            }

            compute_exponentiation(self, exponent).map_err(|_| OverflowError {
                operation: OverflowOperation::Pow,
                operand1: self.to_string(),
                operand2: exponent.to_string(),
            })
        }
    }
    pub fn pow(self, exponent: FPDecimal) -> FPDecimal {
        // fn pow(self, exp: FPDecimal) -> Self {
        if self >= FPDecimal::ZERO {
            match self.checked_positive_pow(exponent) {
                Ok(value) => value,
                Err(_) => panic!("Multiplication overflow"),
            }
        } else {
            match self.checked_negative_pow(exponent) {
                Ok(value) => value,
                Err(_) => panic!("Multiplication overflow"),
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::FPDecimal;
    use bigint::U256;
    use std::str::FromStr;

    #[test]
    fn test_exp() {
        assert_eq!(FPDecimal::_exp(FPDecimal::ONE), FPDecimal::E);
    }

    #[test]
    fn test_exp0() {
        assert_eq!(FPDecimal::_exp(FPDecimal::zero()), FPDecimal::ONE);
    }

    #[test]
    fn test_exp10() {
        assert_eq!(
            FPDecimal::_exp(FPDecimal {
                num: U256([10, 0, 0, 0]) * FPDecimal::ONE.num,
                sign: 1
            }),
            FPDecimal::E_10
        );
    }

    #[test]
    fn test_pow_zero() {
        FPDecimal::pow(FPDecimal::zero(), FPDecimal::one().div(2i128));
        assert_eq!(FPDecimal::ZERO.pow(FPDecimal::ONE), FPDecimal::ZERO);
    }

    #[test]
    fn test_4_pow_0_5() {
        assert_eq!(FPDecimal::pow(FPDecimal::FOUR, FPDecimal::must_from_str("0.5")), FPDecimal::TWO);
    }

    #[test]
    fn test_128_pow_0_5() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(128u128), FPDecimal::must_from_str("0.5")),
            FPDecimal::must_from_str("11.313708498984760390")
        );
    }

    #[test]
    fn test_128_pow_1_7() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(128u128), FPDecimal::ONE / FPDecimal::SEVEN),
            FPDecimal::TWO
        );
    }

    #[test]
    fn test_9_pow_0_5() {
        assert_eq!(FPDecimal::pow(FPDecimal::NINE, FPDecimal::must_from_str("0.5")), FPDecimal::THREE);
    }

    #[test]
    fn test_27_pow_0_5() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(27u128), FPDecimal::must_from_str("0.5")),
            FPDecimal::must_from_str("5.196152422706631880")
        );
    }
    #[test]
    fn test_27_pow_1_over_3() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(27u128), FPDecimal::ONE / FPDecimal::THREE),
            FPDecimal::THREE
        );
    }

    #[test]
    fn test_81_pow_0_25() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(81u128), FPDecimal::ONE / FPDecimal::FOUR),
            FPDecimal::THREE
        );
    }

    #[test]
    fn test_81_pow_0_5() {
        assert_eq!(FPDecimal::pow(FPDecimal::from(81u128), FPDecimal::ONE / FPDecimal::TWO), FPDecimal::NINE);
    }

    #[test]
    fn test_25_pow_0_5() {
        assert_eq!(FPDecimal::pow(FPDecimal::from(25u128), FPDecimal::must_from_str("0.5")), FPDecimal::FIVE);
    }

    #[test]
    fn test_125_pow_0_5() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(125u128), FPDecimal::must_from_str("0.5")),
            FPDecimal::must_from_str("11.180339887498948482")
        );
    }
    #[test]
    fn test_125_pow_1_over_3() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(125u128), FPDecimal::ONE / FPDecimal::THREE),
            FPDecimal::FIVE
        );
    }

    #[test]
    fn test_625_pow_0_25() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(625u128), FPDecimal::ONE / FPDecimal::FOUR),
            FPDecimal::FIVE
        );
    }

    #[test]
    fn test_49_pow_0_5() {
        assert_eq!(FPDecimal::pow(FPDecimal::from(49u128), FPDecimal::must_from_str("0.5")), FPDecimal::SEVEN);
    }

    #[test]
    fn test_343_pow_0_5() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(343u128), FPDecimal::must_from_str("0.5")),
            FPDecimal::must_from_str("18.520259177452134133")
        );
    }
    #[test]
    fn test_343_pow_1_over_3() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(343u128), FPDecimal::ONE / FPDecimal::THREE),
            FPDecimal::SEVEN
        );
    }

    #[test]
    fn test_2401_pow_0_25() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(2401u128), FPDecimal::ONE / FPDecimal::FOUR),
            FPDecimal::SEVEN
        );
    }

    #[test]
    fn test_121_pow_0_5() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(121u128), FPDecimal::must_from_str("0.5")),
            FPDecimal::from(11u128)
        );
    }

    #[test]
    fn test_1331_pow_0_5() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(1331u128), FPDecimal::must_from_str("0.5")),
            FPDecimal::must_from_str("36.482872693909398340")
        );
    }
    #[test]
    fn test_1331_pow_1_over_3() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(1331u128), FPDecimal::ONE / FPDecimal::THREE),
            FPDecimal::from(11u128)
        );
    }

    #[test]
    fn test_14641_pow_0_25() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(14641u128), FPDecimal::ONE / FPDecimal::FOUR),
            FPDecimal::from(11u128)
        );
    }

    #[test]
    fn test_pow_exp() {
        assert_eq!(FPDecimal::E.pow(FPDecimal::ONE), FPDecimal::E);
    }

    #[test]
    fn test_pow_exp0() {
        assert_eq!(FPDecimal::E.pow(FPDecimal::zero()), FPDecimal::ONE);
    }

    #[test]
    fn test_pow_exp10() {
        assert_eq!(
            FPDecimal::E.pow(FPDecimal {
                num: U256([10, 0, 0, 0]) * FPDecimal::ONE.num,
                sign: 1
            }),
            FPDecimal::E_10
        );
    }

    #[test]
    fn test_pow_zero_2() {
        FPDecimal::ZERO.pow(FPDecimal::one().div(2i128));
    }

    #[test]
    fn test_square_root() {
        let inputs: Vec<i128> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 16, 25, -1];

        let expected: Vec<Option<FPDecimal>> = vec![
            Some(FPDecimal::zero()),
            Some(FPDecimal::one()),
            Some(FPDecimal::from_str("1.414213562373095048").unwrap()),
            Some(FPDecimal::from_str("1.732050807568877293").unwrap()),
            Some(FPDecimal::TWO),
            Some(FPDecimal::from_str("2.236067977499789696").unwrap()),
            Some(FPDecimal::from_str("2.449489742783178098").unwrap()),
            Some(FPDecimal::from_str("2.645751311064590590").unwrap()),
            Some(FPDecimal::from_str("2.828427124746190097").unwrap()),
            Some(FPDecimal::THREE),
            Some(FPDecimal::from_str("3.162277660168379331").unwrap()),
            Some(FPDecimal::FOUR),
            Some(FPDecimal::FIVE),
            None,
        ];

        for (ix, el) in inputs.iter().enumerate() {
            let result = FPDecimal::_sqrt(FPDecimal::from(*el));

            assert_eq!(result, expected[ix]);
        }
    }

    #[test]
    fn test_pow_10_positive() {
        let base = FPDecimal::from(10u128);
        assert_eq!(base.pow(FPDecimal::from_str("6").unwrap()), FPDecimal::from_str("1000000").unwrap());
    }

    #[test]
    fn test_pow_10_max() {
        let base = FPDecimal::from(10u128);
        assert_eq!(
            base.pow(FPDecimal::from_str("59").unwrap()),
            FPDecimal::from_str("100000000000000000000000000000000000000000000000000000000000").unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn test_pow_10_overflow() {
        let base = FPDecimal::from(10u128);
        base.pow(FPDecimal::from_str("60").unwrap());
    }

    #[test]
    fn test_pow_10_negative() {
        let base = FPDecimal::from(10u128);
        assert_eq!(base.pow(FPDecimal::from_str("-3").unwrap()), FPDecimal::from_str("0.001").unwrap());
    }

    #[test]
    fn test_e_pow_negative() {
        let base = FPDecimal::E;
        assert_eq!(
            base.pow(FPDecimal::from_str("-3").unwrap()),
            FPDecimal::from_str("0.049787068367863943").unwrap()
        );
    }

    #[test]
    fn test_e_pow_decimal() {
        let base = FPDecimal::E;
        assert_eq!(
            base.pow(FPDecimal::from_str("0.5").unwrap()),
            FPDecimal::from_str("1.648721270700127416").unwrap()
        );
    }

    #[test]
    fn test_pow_10_min() {
        let base = FPDecimal::from(10u128);
        assert_eq!(
            base.pow(FPDecimal::from_str("-18").unwrap()),
            FPDecimal::from_str("0.000000000000000001").unwrap()
        );
    }

    #[test]
    fn test_pow_10_underflow() {
        let base = FPDecimal::from(10u128);
        assert_eq!(base.pow(FPDecimal::from_str("-19").unwrap()), FPDecimal::zero());
    }

    #[test]
    fn test_checked_2_pow_2() {
        let base = FPDecimal::from(2u128);

        let result = FPDecimal::checked_positive_pow(base, FPDecimal::from(2u128)).unwrap();
        assert_eq!(result, FPDecimal::from(4u128));
    }

    #[test]
    fn test_2_3_pow_1_4() {
        let base = FPDecimal::must_from_str("2.3");
        let exponent = FPDecimal::must_from_str("1.4");
        let result = FPDecimal::checked_positive_pow(base, exponent).unwrap();

        assert_eq!(result, FPDecimal::must_from_str("3.209363953267971924"));
    }

    #[test]
    fn test_2_3_pow_3_7() {
        let base = FPDecimal::must_from_str("2.3");
        let exponent = FPDecimal::must_from_str("3.7");
        let result = FPDecimal::checked_positive_pow(base, exponent).unwrap();
        assert_eq!(result, FPDecimal::must_from_str("21.796812747431110477"));
    }

    #[test]
    fn test_2_3_pow_neg_1_4() {
        let base = FPDecimal::must_from_str("2.3");
        let exponent = FPDecimal::must_from_str("-1.4");
        let result = FPDecimal::checked_positive_pow(base, exponent).unwrap();
        assert_eq!(result, FPDecimal::must_from_str("0.311588219522980069"));
    }

    #[test]
    fn test_2_3_pow_neg_3_7() {
        let base = FPDecimal::must_from_str("2.3");
        let exponent = FPDecimal::must_from_str("-3.7");
        let result = FPDecimal::checked_positive_pow(base, exponent).unwrap();
        assert_eq!(result, FPDecimal::must_from_str("0.045878267230507924"));
    }

    #[test]
    fn test_2_3_pow_0_4() {
        let base = FPDecimal::must_from_str("2.3");
        let exponent = FPDecimal::must_from_str("0.4");
        let result = FPDecimal::checked_positive_pow(base, exponent).unwrap();
        assert_eq!(result, FPDecimal::must_from_str("1.395375631855639967"));
    }

    #[test]
    fn test_2_3_pow_neg_0_4() {
        let base = FPDecimal::must_from_str("2.3");
        let exponent = FPDecimal::must_from_str("-0.4");
        let result = FPDecimal::checked_positive_pow(base, exponent).unwrap();
        assert_eq!(result, FPDecimal::must_from_str("0.716652904902854162"));
    }

    #[test]
    fn test_1_over_16_pow_neg_0_5() {
        let base = FPDecimal::ONE / FPDecimal::from(16u128);
        let exponent = FPDecimal::must_from_str("-0.5");

        let result = FPDecimal::checked_positive_pow(base, exponent).unwrap();
        assert_eq!(result, FPDecimal::FOUR);
    }

    #[test]
    fn test_1_over_16_pow_0_5() {
        let base = FPDecimal::ONE / FPDecimal::from(16u128);
        let exponent = FPDecimal::must_from_str("0.5");

        let result = FPDecimal::checked_positive_pow(base, exponent).unwrap();
        assert_eq!(result, FPDecimal::ONE / FPDecimal::FOUR);
    }

    #[test]
    fn test_100_pow_neg_1_over_2() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(100u128), FPDecimal::must_from_str("-0.5")),
            FPDecimal::must_from_str("0.1")
        );
    }

    #[test]
    fn test_1000_pow_1_over_3() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(1000u128), FPDecimal::ONE / FPDecimal::THREE),
            FPDecimal::TEN
        );
    }

    #[test]
    fn test_neg_1000_pow_1_over_3() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::must_from_str("-1000.0"), FPDecimal::ONE / FPDecimal::THREE),
            -FPDecimal::TEN
        );
    }

    #[test]
    fn test_neg_10_pow_3() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::must_from_str("-10"), FPDecimal::THREE),
            -FPDecimal::TEN.pow(FPDecimal::THREE)
        );
    }

    #[test]
    #[should_panic]
    fn test_neg_1000_pow_1_over_4() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::must_from_str("-1000.0"), FPDecimal::ONE / FPDecimal::FOUR),
            -FPDecimal::TEN
        );
    }

    #[test]
    fn test_exp_log_2() {
        assert_eq!(FPDecimal::E.pow(FPDecimal::must_from_str("2.0").ln()), FPDecimal::must_from_str("2.0"));
    }
    #[test]
    fn test_25_pow_0_11111() {
        let power = FPDecimal::ONE / FPDecimal::from(9 as u128);
        let result: FPDecimal = FPDecimal::must_from_str("25.0").ln() * power;
        let dampen: FPDecimal = FPDecimal::E.pow(result);
        assert_eq!(dampen, FPDecimal::must_from_str("1.429969148308728731"));
    }
    #[test]
    fn test_25_pow_0_11111_decimal_lib() {
        let x = FPDecimal::ONE / FPDecimal::from(9 as u128);
        let a: FPDecimal = FPDecimal::must_from_str("25.0");
        let result: FPDecimal = a.pow(x);
        assert_eq!(result, FPDecimal::must_from_str("1.429969148308728731"));
    }
    // #[test]
    // fn test_negative_25_pow_0_11111_decimal_lib() {
    //     let x = FPDecimal::ONE / FPDecimal::from(9 as u128);
    //     let a: FPDecimal = FPDecimal::must_from_str("-25.0");
    //     let result: FPDecimal = a.pow(x);
    //     assert_eq!(result, FPDecimal::must_from_str("1.429969148308728731"));
    // }
}
