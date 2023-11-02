use std::cmp::Ordering;
use std::str::FromStr;

use crate::fp_decimal::error::FPDecimalError;
/// Exponential functions for FPDecimal
use crate::fp_decimal::{FPDecimal, U256};

impl FPDecimal {
    #[allow(clippy::many_single_char_names)]
    pub fn exp_taylor_expansion(a: FPDecimal, b: FPDecimal) -> FPDecimal {
        //a^b n+1 terms taylor expansion
        let base = a.ln() * b;
        let mut numerator = base;
        let mut denominator = FPDecimal::ONE;

        let denominator_parts = vec![
            FPDecimal::TWO,
            FPDecimal::THREE,
            FPDecimal::FOUR,
            FPDecimal::FIVE,
            FPDecimal::SIX,
            FPDecimal::SEVEN,
            FPDecimal::EIGHT,
            FPDecimal::NINE,
            FPDecimal::TEN,
            FPDecimal::ELEVEN,
            FPDecimal::must_from_str("12"),
            FPDecimal::must_from_str("13"),
            FPDecimal::must_from_str("14"),
            FPDecimal::must_from_str("15"),
            FPDecimal::must_from_str("16"),
            FPDecimal::must_from_str("17"),
            FPDecimal::must_from_str("18"),
            FPDecimal::must_from_str("19"),
            FPDecimal::must_from_str("20"),
            FPDecimal::must_from_str("21"),
            FPDecimal::must_from_str("22"),
            FPDecimal::must_from_str("23"),
            FPDecimal::must_from_str("24"),
            FPDecimal::must_from_str("25"),
        ];

        denominator_parts
            .iter()
            .map(|part| {
                numerator *= base;
                denominator *= *part;
                numerator / denominator
            })
            .collect::<Vec<FPDecimal>>()
            .iter()
            .sum::<FPDecimal>()
            + FPDecimal::ONE
            + a.ln() * b
    }
    // e^(a)

    fn two_pow(exponent: FPDecimal) -> Option<FPDecimal> {
        if exponent == FPDecimal::ONE {
            return Some(FPDecimal::TWO);
        }
        if exponent == FPDecimal::TWO {
            return Some(FPDecimal::FOUR);
        }
        if exponent == FPDecimal::THREE {
            return Some(FPDecimal::EIGHT);
        }
        if exponent == FPDecimal::FOUR {
            return Some(FPDecimal::from(16u128));
        }
        if exponent == FPDecimal::FIVE {
            return Some(FPDecimal::from(32u128));
        }
        if exponent == FPDecimal::SIX {
            return Some(FPDecimal::from(64u128));
        }
        if exponent == FPDecimal::SEVEN {
            return Some(FPDecimal::from(128u128));
        }
        if exponent == FPDecimal::EIGHT {
            return Some(FPDecimal::from(256u128));
        }
        if exponent == FPDecimal::NINE {
            return Some(FPDecimal::from(512u128));
        }
        if exponent == FPDecimal::TEN {
            return Some(FPDecimal::from(1024u128));
        }
        if exponent == FPDecimal::ELEVEN {
            return Some(FPDecimal::from(2048u128));
        }
        if exponent == FPDecimal::from(12u128) {
            return Some(FPDecimal::from(4096u128));
        }
        if exponent == FPDecimal::from(13u128) {
            return Some(FPDecimal::from(8192u128));
        }
        if exponent == FPDecimal::from(14u128) {
            return Some(FPDecimal::from(16384u128));
        }
        if exponent == FPDecimal::from(15u128) {
            return Some(FPDecimal::from(32768u128));
        }

        if FPDecimal::ONE.log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::ONE);
        }
        if FPDecimal::TWO.log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::TWO);
        }
        if FPDecimal::THREE.log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::THREE);
        }
        if FPDecimal::FOUR.log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::FOUR);
        }
        if FPDecimal::FIVE.log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::FIVE);
        }
        if FPDecimal::SIX.log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::SIX);
        }
        if FPDecimal::SEVEN.log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::SEVEN);
        }
        if FPDecimal::EIGHT.log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::EIGHT);
        }
        if FPDecimal::NINE.log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::NINE);
        }
        if FPDecimal::TEN.log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::TEN);
        }
        if FPDecimal::ELEVEN.log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::ELEVEN);
        }
        if FPDecimal::from(12u128).log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::from(12u128));
        }
        if FPDecimal::from(13u128).log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::from(13u128));
        }
        if FPDecimal::from(14u128).log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::from(14u128));
        }
        if FPDecimal::from(15u128).log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::from(15u128));
        }

        if (FPDecimal::ONE / FPDecimal::TWO).log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::ONE / FPDecimal::TWO);
        }
        if (FPDecimal::ONE / FPDecimal::THREE).log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::ONE / FPDecimal::THREE);
        }
        if (FPDecimal::ONE / FPDecimal::FOUR).log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::ONE / FPDecimal::FOUR);
        }
        if (FPDecimal::ONE / FPDecimal::FIVE).log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::ONE / FPDecimal::FIVE);
        }
        if (FPDecimal::ONE / FPDecimal::SIX).log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::ONE / FPDecimal::SIX);
        }
        if (FPDecimal::ONE / FPDecimal::SEVEN).log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::ONE / FPDecimal::SEVEN);
        }
        if (FPDecimal::ONE / FPDecimal::EIGHT).log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::ONE / FPDecimal::EIGHT);
        }
        if (FPDecimal::ONE / FPDecimal::NINE).log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::ONE / FPDecimal::NINE);
        }
        if (FPDecimal::ONE / FPDecimal::TEN).log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::ONE / FPDecimal::TEN);
        }
        if (FPDecimal::ONE / FPDecimal::ELEVEN).log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::ONE / FPDecimal::ELEVEN);
        }
        if (FPDecimal::ONE / FPDecimal::from(12u128)).log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::ONE / FPDecimal::from(12u128));
        }
        if (FPDecimal::ONE / FPDecimal::from(13u128)).log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::ONE / FPDecimal::from(13u128));
        }
        if (FPDecimal::ONE / FPDecimal::from(14u128)).log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::ONE / FPDecimal::from(14u128));
        }
        if (FPDecimal::ONE / FPDecimal::from(15u128)).log2().is_some_and(|x| x == exponent) {
            return Some(FPDecimal::ONE / FPDecimal::from(15u128));
        }

        if exponent == -FPDecimal::TWO {
            return Some(FPDecimal::ONE / FPDecimal::FOUR);
        }
        if exponent == -FPDecimal::THREE {
            return Some(FPDecimal::ONE / FPDecimal::EIGHT);
        }
        if exponent == -FPDecimal::FOUR {
            return Some(FPDecimal::ONE / FPDecimal::from(16u128));
        }
        if exponent == -FPDecimal::FIVE {
            return Some(FPDecimal::ONE / FPDecimal::from(32u128));
        }
        if exponent == -FPDecimal::SIX {
            return Some(FPDecimal::ONE / FPDecimal::from(64u128));
        }
        if exponent == -FPDecimal::SEVEN {
            return Some(FPDecimal::ONE / FPDecimal::from(128u128));
        }
        if exponent == -FPDecimal::EIGHT {
            return Some(FPDecimal::ONE / FPDecimal::from(256u128));
        }
        if exponent == -FPDecimal::NINE {
            return Some(FPDecimal::ONE / FPDecimal::from(512u128));
        }
        if exponent == -FPDecimal::TEN {
            return Some(FPDecimal::ONE / FPDecimal::from(1024u128));
        }
        if exponent == -FPDecimal::ELEVEN {
            return Some(FPDecimal::ONE / FPDecimal::from(2048u128));
        }
        if exponent == -FPDecimal::from(12u128) {
            return Some(FPDecimal::ONE / FPDecimal::from(4096u128));
        }
        if exponent == -FPDecimal::from(13u128) {
            return Some(FPDecimal::ONE / FPDecimal::from(8192u128));
        }
        if exponent == -FPDecimal::from(14u128) {
            return Some(FPDecimal::ONE / FPDecimal::from(16384u128));
        }
        if exponent == -FPDecimal::from(15u128) {
            return Some(FPDecimal::ONE / FPDecimal::from(32768u128));
        }

        if FPDecimal::ONE.log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::ONE);
        }
        if FPDecimal::TWO.log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::ONE / FPDecimal::TWO);
        }
        if FPDecimal::THREE.log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::ONE / FPDecimal::THREE);
        }
        if FPDecimal::FOUR.log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::ONE / FPDecimal::FOUR);
        }
        if FPDecimal::FIVE.log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::ONE / FPDecimal::FIVE);
        }
        if FPDecimal::SIX.log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::ONE / FPDecimal::SIX);
        }
        if FPDecimal::SEVEN.log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::ONE / FPDecimal::SEVEN);
        }
        if FPDecimal::EIGHT.log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::ONE / FPDecimal::EIGHT);
        }
        if FPDecimal::NINE.log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::ONE / FPDecimal::NINE);
        }
        if FPDecimal::TEN.log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::ONE / FPDecimal::TEN);
        }
        if FPDecimal::ELEVEN.log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::ONE / FPDecimal::ELEVEN);
        }
        if FPDecimal::from(12u128).log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::ONE / FPDecimal::from(12u128));
        }
        if FPDecimal::from(13u128).log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::ONE / FPDecimal::from(13u128));
        }
        if FPDecimal::from(14u128).log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::ONE / FPDecimal::from(14u128));
        }
        if FPDecimal::from(15u128).log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::ONE / FPDecimal::from(15u128));
        }

        if (FPDecimal::ONE / FPDecimal::TWO).log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::TWO);
        }
        if (FPDecimal::ONE / FPDecimal::THREE).log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::THREE);
        }
        if (FPDecimal::ONE / FPDecimal::FOUR).log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::FOUR);
        }
        if (FPDecimal::ONE / FPDecimal::FIVE).log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::FIVE);
        }
        if (FPDecimal::ONE / FPDecimal::SIX).log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::SIX);
        }
        if (FPDecimal::ONE / FPDecimal::SEVEN).log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::SEVEN);
        }
        if (FPDecimal::ONE / FPDecimal::EIGHT).log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::EIGHT);
        }
        if (FPDecimal::ONE / FPDecimal::NINE).log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::NINE);
        }
        if (FPDecimal::ONE / FPDecimal::TEN).log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::TEN);
        }
        if (FPDecimal::ONE / FPDecimal::ELEVEN).log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::ELEVEN);
        }
        if (FPDecimal::ONE / FPDecimal::from(12u128)).log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::from(12u128));
        }
        if (FPDecimal::ONE / FPDecimal::from(13u128)).log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::from(13u128));
        }
        if (FPDecimal::ONE / FPDecimal::from(14u128)).log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::from(14u128));
        }
        if (FPDecimal::ONE / FPDecimal::from(15u128)).log2().is_some_and(|x| x == -exponent) {
            return Some(FPDecimal::from(15u128));
        }

        None
    }

    fn e_pow(exponent: FPDecimal) -> Option<FPDecimal> {
        if exponent == FPDecimal::ONE {
            return Some(FPDecimal::E);
        }
        if exponent == FPDecimal::ONE.ln() {
            return Some(FPDecimal::ONE);
        }
        if exponent == FPDecimal::TWO.ln() {
            return Some(FPDecimal::TWO);
        }
        if exponent == FPDecimal::THREE.ln() {
            return Some(FPDecimal::THREE);
        }
        if exponent == FPDecimal::FOUR.ln() {
            return Some(FPDecimal::FOUR);
        }
        if exponent == FPDecimal::FIVE.ln() {
            return Some(FPDecimal::FIVE);
        }
        if exponent == FPDecimal::SIX.ln() {
            return Some(FPDecimal::SIX);
        }
        if exponent == FPDecimal::SEVEN.ln() {
            return Some(FPDecimal::SEVEN);
        }
        if exponent == FPDecimal::EIGHT.ln() {
            return Some(FPDecimal::EIGHT);
        }
        if exponent == FPDecimal::NINE.ln() {
            return Some(FPDecimal::NINE);
        }
        if exponent == FPDecimal::TEN.ln() {
            return Some(FPDecimal::TEN);
        }
        if exponent == FPDecimal::ELEVEN.ln() {
            return Some(FPDecimal::ELEVEN);
        }
        if exponent == FPDecimal::from(12u128).ln() {
            return Some(FPDecimal::from(12u128));
        }
        if exponent == FPDecimal::from(13u128).ln() {
            return Some(FPDecimal::from(13u128));
        }
        if exponent == FPDecimal::from(14u128).ln() {
            return Some(FPDecimal::from(14u128));
        }
        if exponent == FPDecimal::from(15u128).ln() {
            return Some(FPDecimal::from(15u128));
        }

        if exponent == (FPDecimal::ONE / FPDecimal::TWO).ln() {
            return Some(FPDecimal::ONE / FPDecimal::TWO);
        }
        if exponent == (FPDecimal::ONE / FPDecimal::THREE).ln() {
            return Some(FPDecimal::ONE / FPDecimal::THREE);
        }
        if exponent == (FPDecimal::ONE / FPDecimal::FOUR).ln() {
            return Some(FPDecimal::ONE / FPDecimal::FOUR);
        }
        if exponent == (FPDecimal::ONE / FPDecimal::FIVE).ln() {
            return Some(FPDecimal::ONE / FPDecimal::FIVE);
        }
        if exponent == (FPDecimal::ONE / FPDecimal::SIX).ln() {
            return Some(FPDecimal::ONE / FPDecimal::SIX);
        }
        if exponent == (FPDecimal::ONE / FPDecimal::SEVEN).ln() {
            return Some(FPDecimal::ONE / FPDecimal::SEVEN);
        }
        if exponent == (FPDecimal::ONE / FPDecimal::EIGHT).ln() {
            return Some(FPDecimal::ONE / FPDecimal::EIGHT);
        }
        if exponent == (FPDecimal::ONE / FPDecimal::NINE).ln() {
            return Some(FPDecimal::ONE / FPDecimal::NINE);
        }
        if exponent == (FPDecimal::ONE / FPDecimal::TEN).ln() {
            return Some(FPDecimal::ONE / FPDecimal::TEN);
        }
        if exponent == (FPDecimal::ONE / FPDecimal::ELEVEN).ln() {
            return Some(FPDecimal::ONE / FPDecimal::ELEVEN);
        }
        if exponent == (FPDecimal::ONE / FPDecimal::from(12u128)).ln() {
            return Some(FPDecimal::ONE / FPDecimal::from(12u128));
        }
        if exponent == (FPDecimal::ONE / FPDecimal::from(13u128)).ln() {
            return Some(FPDecimal::ONE / FPDecimal::from(13u128));
        }
        if exponent == (FPDecimal::ONE / FPDecimal::from(14u128)).ln() {
            return Some(FPDecimal::ONE / FPDecimal::from(14u128));
        }
        if exponent == (FPDecimal::ONE / FPDecimal::from(15u128)).ln() {
            return Some(FPDecimal::ONE / FPDecimal::from(15u128));
        }

        if exponent == -FPDecimal::ONE {
            return Some(FPDecimal::ONE / FPDecimal::E);
        }
        if exponent == -FPDecimal::ONE.ln() {
            return Some(FPDecimal::ONE);
        }
        if exponent == FPDecimal::TWO.ln() {
            return Some(FPDecimal::ONE / FPDecimal::TWO);
        }
        if exponent == -FPDecimal::THREE.ln() {
            return Some(FPDecimal::ONE / FPDecimal::THREE);
        }
        if exponent == -FPDecimal::FOUR.ln() {
            return Some(FPDecimal::ONE / FPDecimal::FOUR);
        }
        if exponent == -FPDecimal::FIVE.ln() {
            return Some(FPDecimal::ONE / FPDecimal::FIVE);
        }
        if exponent == -FPDecimal::SIX.ln() {
            return Some(FPDecimal::ONE / FPDecimal::SIX);
        }
        if exponent == -FPDecimal::SEVEN.ln() {
            return Some(FPDecimal::ONE / FPDecimal::SEVEN);
        }
        if exponent == -FPDecimal::EIGHT.ln() {
            return Some(FPDecimal::ONE / FPDecimal::EIGHT);
        }
        if exponent == -FPDecimal::NINE.ln() {
            return Some(FPDecimal::ONE / FPDecimal::NINE);
        }
        if exponent == -FPDecimal::TEN.ln() {
            return Some(FPDecimal::ONE / FPDecimal::TEN);
        }
        if exponent == -FPDecimal::ELEVEN.ln() {
            return Some(FPDecimal::ONE / FPDecimal::ELEVEN);
        }
        if exponent == -FPDecimal::from(12u128).ln() {
            return Some(FPDecimal::ONE / FPDecimal::from(12u128));
        }
        if exponent == -FPDecimal::from(13u128).ln() {
            return Some(FPDecimal::ONE / FPDecimal::from(13u128));
        }
        if exponent == -FPDecimal::from(14u128).ln() {
            return Some(FPDecimal::ONE / FPDecimal::from(14u128));
        }
        if exponent == -FPDecimal::from(15u128).ln() {
            return Some(FPDecimal::ONE / FPDecimal::from(15u128));
        }

        if exponent == -(FPDecimal::ONE / FPDecimal::TWO).ln() {
            return Some(FPDecimal::TWO);
        }
        if exponent == -(FPDecimal::ONE / FPDecimal::THREE).ln() {
            return Some(FPDecimal::THREE);
        }
        if exponent == -(FPDecimal::ONE / FPDecimal::FOUR).ln() {
            return Some(FPDecimal::FOUR);
        }
        if exponent == -(FPDecimal::ONE / FPDecimal::FIVE).ln() {
            return Some(FPDecimal::FIVE);
        }
        if exponent == -(FPDecimal::ONE / FPDecimal::SIX).ln() {
            return Some(FPDecimal::SIX);
        }
        if exponent == -(FPDecimal::ONE / FPDecimal::SEVEN).ln() {
            return Some(FPDecimal::SEVEN);
        }
        if exponent == -(FPDecimal::ONE / FPDecimal::EIGHT).ln() {
            return Some(FPDecimal::EIGHT);
        }
        if exponent == -(FPDecimal::ONE / FPDecimal::NINE).ln() {
            return Some(FPDecimal::NINE);
        }
        if exponent == -(FPDecimal::ONE / FPDecimal::TEN).ln() {
            return Some(FPDecimal::TEN);
        }
        if exponent == -(FPDecimal::ONE / FPDecimal::ELEVEN).ln() {
            return Some(FPDecimal::ELEVEN);
        }
        if exponent == -(FPDecimal::ONE / FPDecimal::from(12u128)).ln() {
            return Some(FPDecimal::from(12u128));
        }
        if exponent == -(FPDecimal::ONE / FPDecimal::from(13u128)).ln() {
            return Some(FPDecimal::from(13u128));
        }
        if exponent == -(FPDecimal::ONE / FPDecimal::from(14u128)).ln() {
            return Some(FPDecimal::from(14u128));
        }
        if exponent == -(FPDecimal::ONE / FPDecimal::from(15u128)).ln() {
            return Some(FPDecimal::from(15u128));
        }
        None
    }

    fn three_pow(exponent: FPDecimal) -> Option<FPDecimal> {
        if exponent == FPDecimal::ONE {
            return Some(FPDecimal::THREE);
        }
        if exponent == FPDecimal::TWO {
            return Some(FPDecimal::NINE);
        }
        if exponent == FPDecimal::THREE {
            return Some(FPDecimal::from(27u128));
        }
        if exponent == FPDecimal::FOUR {
            return Some(FPDecimal::from(81u128));
        }
        if exponent == FPDecimal::FIVE {
            return Some(FPDecimal::from(243u128));
        }
        if exponent == FPDecimal::SIX {
            return Some(FPDecimal::from(729u128));
        }
        if exponent == FPDecimal::SEVEN {
            return Some(FPDecimal::from(2187u128));
        }
        if exponent == FPDecimal::EIGHT {
            return Some(FPDecimal::from(6561u128));
        }
        if exponent == FPDecimal::NINE {
            return Some(FPDecimal::from(19683u128));
        }
        if exponent == FPDecimal::TEN {
            return Some(FPDecimal::from(59049u128));
        }
        if exponent == FPDecimal::ELEVEN {
            return Some(FPDecimal::from(177147u128));
        }
        if exponent == FPDecimal::from(12u128) {
            return Some(FPDecimal::from(531441u128));
        }
        if exponent == FPDecimal::from(13u128) {
            return Some(FPDecimal::from(1594323u128));
        }
        if exponent == FPDecimal::from(14u128) {
            return Some(FPDecimal::from(4782969u128));
        }
        if exponent == FPDecimal::from(15u128) {
            return Some(FPDecimal::from(14348907u128));
        }

        if exponent == -FPDecimal::ONE {
            return Some(FPDecimal::ONE / FPDecimal::THREE);
        }
        if exponent == -FPDecimal::TWO {
            return Some(FPDecimal::ONE / FPDecimal::NINE);
        }
        if exponent == -FPDecimal::THREE {
            return Some(FPDecimal::ONE / FPDecimal::from(27u128));
        }
        if exponent == -FPDecimal::FOUR {
            return Some(FPDecimal::ONE / FPDecimal::from(81u128));
        }
        if exponent == -FPDecimal::FIVE {
            return Some(FPDecimal::ONE / FPDecimal::from(243u128));
        }
        if exponent == -FPDecimal::SIX {
            return Some(FPDecimal::ONE / FPDecimal::from(729u128));
        }
        if exponent == -FPDecimal::SEVEN {
            return Some(FPDecimal::ONE / FPDecimal::from(2187u128));
        }
        if exponent == -FPDecimal::EIGHT {
            return Some(FPDecimal::ONE / FPDecimal::from(6561u128));
        }
        if exponent == -FPDecimal::NINE {
            return Some(FPDecimal::ONE / FPDecimal::from(19683u128));
        }
        if exponent == -FPDecimal::TEN {
            return Some(FPDecimal::ONE / FPDecimal::from(59049u128));
        }
        if exponent == -FPDecimal::ELEVEN {
            return Some(FPDecimal::ONE / FPDecimal::from(177147u128));
        }
        if exponent == -FPDecimal::from(12u128) {
            return Some(FPDecimal::ONE / FPDecimal::from(531441u128));
        }
        if exponent == -FPDecimal::from(13u128) {
            return Some(FPDecimal::ONE / FPDecimal::from(1594323u128));
        }
        if exponent == -FPDecimal::from(14u128) {
            return Some(FPDecimal::ONE / FPDecimal::from(4782969u128));
        }
        if exponent == -FPDecimal::from(15u128) {
            return Some(FPDecimal::ONE / FPDecimal::from(14348907u128));
        }
        None
    }

    fn five_pow(exponent: FPDecimal) -> Option<FPDecimal> {
        if exponent == FPDecimal::ONE {
            return Some(FPDecimal::FIVE);
        }
        if exponent == FPDecimal::TWO {
            return Some(FPDecimal::from(25u128));
        }
        if exponent == FPDecimal::THREE {
            return Some(FPDecimal::from(125u128));
        }
        if exponent == FPDecimal::FOUR {
            return Some(FPDecimal::from(625u128));
        }
        if exponent == FPDecimal::FIVE {
            return Some(FPDecimal::from(3125u128));
        }
        if exponent == FPDecimal::SIX {
            return Some(FPDecimal::from(15625u128));
        }
        if exponent == FPDecimal::SEVEN {
            return Some(FPDecimal::from(78125u128));
        }
        if exponent == FPDecimal::EIGHT {
            return Some(FPDecimal::from(390625u128));
        }
        if exponent == FPDecimal::NINE {
            return Some(FPDecimal::from(1953125u128));
        }
        if exponent == FPDecimal::TEN {
            return Some(FPDecimal::from(9765625u128));
        }
        if exponent == FPDecimal::ELEVEN {
            return Some(FPDecimal::from(48828125u128));
        }
        if exponent == FPDecimal::from(12u128) {
            return Some(FPDecimal::from(244140625u128));
        }
        if exponent == FPDecimal::from(13u128) {
            return Some(FPDecimal::from(1220703125u128));
        }
        if exponent == FPDecimal::from(14u128) {
            return Some(FPDecimal::from(6103515625u128));
        }
        if exponent == FPDecimal::from(15u128) {
            return Some(FPDecimal::from(30517578125u128));
        }

        if exponent == -FPDecimal::ONE {
            return Some(FPDecimal::ONE / FPDecimal::FIVE);
        }
        if exponent == -FPDecimal::TWO {
            return Some(FPDecimal::ONE / FPDecimal::from(25u128));
        }
        if exponent == -FPDecimal::THREE {
            return Some(FPDecimal::ONE / FPDecimal::from(125u128));
        }
        if exponent == -FPDecimal::FOUR {
            return Some(FPDecimal::ONE / FPDecimal::from(625u128));
        }
        if exponent == -FPDecimal::FIVE {
            return Some(FPDecimal::ONE / FPDecimal::from(3125u128));
        }
        if exponent == -FPDecimal::SIX {
            return Some(FPDecimal::ONE / FPDecimal::from(15625u128));
        }
        if exponent == -FPDecimal::SEVEN {
            return Some(FPDecimal::ONE / FPDecimal::from(78125u128));
        }
        if exponent == -FPDecimal::EIGHT {
            return Some(FPDecimal::ONE / FPDecimal::from(390625u128));
        }
        if exponent == -FPDecimal::NINE {
            return Some(FPDecimal::ONE / FPDecimal::from(1953125u128));
        }
        if exponent == -FPDecimal::TEN {
            return Some(FPDecimal::ONE / FPDecimal::from(9765625u128));
        }
        if exponent == -FPDecimal::ELEVEN {
            return Some(FPDecimal::ONE / FPDecimal::from(48828125u128));
        }
        if exponent == -FPDecimal::from(12u128) {
            return Some(FPDecimal::ONE / FPDecimal::from(244140625u128));
        }
        if exponent == -FPDecimal::from(13u128) {
            return Some(FPDecimal::ONE / FPDecimal::from(1220703125u128));
        }
        if exponent == -FPDecimal::from(14u128) {
            return Some(FPDecimal::ONE / FPDecimal::from(6103515625u128));
        }
        if exponent == -FPDecimal::from(15u128) {
            return Some(FPDecimal::ONE / FPDecimal::from(30517578125u128));
        }
        None
    }

    fn seven_pow(exponent: FPDecimal) -> Option<FPDecimal> {
        if exponent == FPDecimal::ONE {
            return Some(FPDecimal::SEVEN);
        }
        if exponent == FPDecimal::TWO {
            return Some(FPDecimal::from(49u128));
        }
        if exponent == FPDecimal::THREE {
            return Some(FPDecimal::from(343u128));
        }
        if exponent == FPDecimal::FOUR {
            return Some(FPDecimal::from(2401u128));
        }
        if exponent == FPDecimal::FIVE {
            return Some(FPDecimal::from(16807u128));
        }
        if exponent == FPDecimal::SIX {
            return Some(FPDecimal::from(117649u128));
        }
        if exponent == FPDecimal::SEVEN {
            return Some(FPDecimal::from(823543u128));
        }
        if exponent == FPDecimal::EIGHT {
            return Some(FPDecimal::from(5764801u128));
        }
        if exponent == FPDecimal::NINE {
            return Some(FPDecimal::from(40353607u128));
        }
        if exponent == FPDecimal::TEN {
            return Some(FPDecimal::from(282475249u128));
        }
        if exponent == FPDecimal::ELEVEN {
            return Some(FPDecimal::from(1977326743u128));
        }
        if exponent == FPDecimal::from(12u128) {
            return Some(FPDecimal::from(13841287201u128));
        }
        if exponent == FPDecimal::from(13u128) {
            return Some(FPDecimal::from(96889010407u128));
        }
        if exponent == FPDecimal::from(14u128) {
            return Some(FPDecimal::from(678223072849u128));
        }
        if exponent == FPDecimal::from(15u128) {
            return Some(FPDecimal::from(4747561509943u128));
        }
        if exponent == -FPDecimal::ONE {
            return Some(FPDecimal::ONE / FPDecimal::SEVEN);
        }
        if exponent == -FPDecimal::TWO {
            return Some(FPDecimal::ONE / FPDecimal::from(49u128));
        }
        if exponent == -FPDecimal::THREE {
            return Some(FPDecimal::ONE / FPDecimal::from(343u128));
        }
        if exponent == -FPDecimal::FOUR {
            return Some(FPDecimal::ONE / FPDecimal::from(2401u128));
        }
        if exponent == -FPDecimal::FIVE {
            return Some(FPDecimal::ONE / FPDecimal::from(16807u128));
        }
        if exponent == -FPDecimal::SIX {
            return Some(FPDecimal::ONE / FPDecimal::from(117649u128));
        }
        if exponent == -FPDecimal::SEVEN {
            return Some(FPDecimal::ONE / FPDecimal::from(823543u128));
        }
        if exponent == -FPDecimal::EIGHT {
            return Some(FPDecimal::ONE / FPDecimal::from(5764801u128));
        }
        if exponent == -FPDecimal::NINE {
            return Some(FPDecimal::ONE / FPDecimal::from(40353607u128));
        }
        if exponent == -FPDecimal::TEN {
            return Some(FPDecimal::ONE / FPDecimal::from(282475249u128));
        }
        if exponent == -FPDecimal::ELEVEN {
            return Some(FPDecimal::ONE / FPDecimal::from(1977326743u128));
        }
        if exponent == -FPDecimal::from(12u128) {
            return Some(FPDecimal::ONE / FPDecimal::from(13841287201u128));
        }
        if exponent == -FPDecimal::from(13u128) {
            return Some(FPDecimal::ONE / FPDecimal::from(96889010407u128));
        }
        if exponent == -FPDecimal::from(14u128) {
            return Some(FPDecimal::ONE / FPDecimal::from(678223072849u128));
        }
        if exponent == -FPDecimal::from(15u128) {
            return Some(FPDecimal::ONE / FPDecimal::from(4747561509943u128));
        }

        None
    }

    fn ten_pow(exponent: FPDecimal) -> Option<FPDecimal> {
        if exponent == FPDecimal::ONE {
            return Some(FPDecimal::from(10u128));
        }
        if exponent == FPDecimal::TWO {
            return Some(FPDecimal::from(100u128));
        }
        if exponent == FPDecimal::THREE {
            return Some(FPDecimal::from(1000u128));
        }
        if exponent == FPDecimal::FOUR {
            return Some(FPDecimal::from(10000u128));
        }
        if exponent == FPDecimal::FIVE {
            return Some(FPDecimal::from(100000u128));
        }
        if exponent == FPDecimal::SIX {
            return Some(FPDecimal::from(1000000u128));
        }
        if exponent == FPDecimal::SEVEN {
            return Some(FPDecimal::from(10000000u128));
        }
        if exponent == FPDecimal::EIGHT {
            return Some(FPDecimal::from(100000000u128));
        }
        if exponent == FPDecimal::NINE {
            return Some(FPDecimal::from(1000000000u128));
        }
        if exponent == FPDecimal::TEN {
            return Some(FPDecimal::from(10000000000u128));
        }
        if exponent == FPDecimal::ELEVEN {
            return Some(FPDecimal::from(100000000000u128));
        }
        if exponent == FPDecimal::from(12u128) {
            return Some(FPDecimal::from(1000000000000u128));
        }
        if exponent == FPDecimal::from(13u128) {
            return Some(FPDecimal::from(10000000000000u128));
        }
        if exponent == FPDecimal::from(14u128) {
            return Some(FPDecimal::from(100000000000000u128));
        }
        if exponent == FPDecimal::from(15u128) {
            return Some(FPDecimal::from(1000000000000000u128));
        }
        if exponent == FPDecimal::from(16u128) {
            return Some(FPDecimal::from(10000000000000000u128));
        }
        if exponent == FPDecimal::from(17u128) {
            return Some(FPDecimal::from(100000000000000000u128));
        }
        if exponent == FPDecimal::from(18u128) {
            return Some(FPDecimal::from(1000000000000000000u128));
        }
        if exponent == FPDecimal::from(19u128) {
            return Some(FPDecimal::from(10000000000000000000u128));
        }
        if exponent == FPDecimal::from(20u128) {
            return Some(FPDecimal::from(100000000000000000000u128));
        }
        if exponent == FPDecimal::NEGATIVE_ONE {
            return Some(FPDecimal::from_str("0.1").unwrap());
        }
        if exponent == FPDecimal::from_str("-2").unwrap() {
            return Some(FPDecimal::from_str("0.01").unwrap());
        }
        if exponent == FPDecimal::from_str("-3").unwrap() {
            return Some(FPDecimal::from_str("0.001").unwrap());
        }
        if exponent == FPDecimal::from_str("-4").unwrap() {
            return Some(FPDecimal::from_str("0.0001").unwrap());
        }
        if exponent == FPDecimal::from_str("-5").unwrap() {
            return Some(FPDecimal::from_str("0.00001").unwrap());
        }
        if exponent == FPDecimal::from_str("-6").unwrap() {
            return Some(FPDecimal::from_str("0.000001").unwrap());
        }
        if exponent == FPDecimal::from_str("-7").unwrap() {
            return Some(FPDecimal::from_str("0.0000001").unwrap());
        }
        if exponent == FPDecimal::from_str("-8").unwrap() {
            return Some(FPDecimal::from_str("0.00000001").unwrap());
        }
        if exponent == FPDecimal::from_str("-9").unwrap() {
            return Some(FPDecimal::from_str("0.000000001").unwrap());
        }
        if exponent == FPDecimal::from_str("-10").unwrap() {
            return Some(FPDecimal::from_str("0.0000000001").unwrap());
        }
        if exponent == FPDecimal::from_str("-11").unwrap() {
            return Some(FPDecimal::from_str("0.00000000001").unwrap());
        }
        if exponent == FPDecimal::from_str("-12").unwrap() {
            return Some(FPDecimal::from_str("0.000000000001").unwrap());
        }
        if exponent == FPDecimal::from_str("-13").unwrap() {
            return Some(FPDecimal::from_str("0.0000000000001").unwrap());
        }
        if exponent == FPDecimal::from_str("-14").unwrap() {
            return Some(FPDecimal::from_str("0.00000000000001").unwrap());
        }
        if exponent == FPDecimal::from_str("-15").unwrap() {
            return Some(FPDecimal::from_str("0.000000000000001").unwrap());
        }
        if exponent == FPDecimal::from_str("-16").unwrap() {
            return Some(FPDecimal::from_str("0.0000000000000001").unwrap());
        }
        if exponent == FPDecimal::from_str("-17").unwrap() {
            return Some(FPDecimal::from_str("0.00000000000000001").unwrap());
        }
        if exponent == FPDecimal::from_str("-18").unwrap() {
            return Some(FPDecimal::from_str("0.000000000000000001").unwrap());
        }
        if exponent < FPDecimal::from_str("-18").unwrap() {
            return Some(FPDecimal::ZERO);
        }
        if exponent == FPDecimal::from(21u128) {
            return Some(FPDecimal::from(1000000000000000000000u128));
        }
        if exponent == FPDecimal::from(22u128) {
            return Some(FPDecimal::from(10000000000000000000000u128));
        }
        if exponent == FPDecimal::from(23u128) {
            return Some(FPDecimal::from(100000000000000000000000u128));
        }
        if exponent == FPDecimal::from(24u128) {
            return Some(FPDecimal::from(1000000000000000000000000u128));
        }
        if exponent == FPDecimal::from(25u128) {
            return Some(FPDecimal::from(10000000000000000000000000u128));
        }
        if exponent == FPDecimal::from(26u128) {
            return Some(FPDecimal::from(100000000000000000000000000u128));
        }
        if exponent == FPDecimal::from(27u128) {
            return Some(FPDecimal::from(1000000000000000000000000000u128));
        }
        if exponent == FPDecimal::from(28u128) {
            return Some(FPDecimal::from(10000000000000000000000000000u128));
        }
        if exponent == FPDecimal::from(29u128) {
            return Some(FPDecimal::from(100000000000000000000000000000u128));
        }
        if exponent == FPDecimal::from(30u128) {
            return Some(FPDecimal::from(1000000000000000000000000000000u128));
        }
        if exponent == FPDecimal::from(31u128) {
            return Some(FPDecimal::from(10000000000000000000000000000000u128));
        }
        if exponent == FPDecimal::from(32u128) {
            return Some(FPDecimal::from(100000000000000000000000000000000u128));
        }
        if exponent == FPDecimal::from(33u128) {
            return Some(FPDecimal::from(1000000000000000000000000000000000u128));
        }
        if exponent == FPDecimal::from(34u128) {
            return Some(FPDecimal::from(10000000000000000000000000000000000u128));
        }
        if exponent == FPDecimal::from(35u128) {
            return Some(FPDecimal::from(100000000000000000000000000000000000u128));
        }
        if exponent == FPDecimal::from(36u128) {
            return Some(FPDecimal::from(1000000000000000000000000000000000000u128));
        }
        if exponent == FPDecimal::from(37u128) {
            return Some(FPDecimal::from(10000000000000000000000000000000000000u128));
        }
        if exponent == FPDecimal::from(38u128) {
            return Some(FPDecimal::from(100000000000000000000000000000000000000u128));
        }
        if exponent == FPDecimal::from(39u128) {
            return Some(FPDecimal::from_str("1000000000000000000000000000000000000000").unwrap());
        }
        if exponent == FPDecimal::from(40u128) {
            return Some(FPDecimal::from_str("10000000000000000000000000000000000000000").unwrap());
        }
        if exponent == FPDecimal::from(41u128) {
            return Some(FPDecimal::from_str("100000000000000000000000000000000000000000").unwrap());
        }
        if exponent == FPDecimal::from(42u128) {
            return Some(FPDecimal::from_str("1000000000000000000000000000000000000000000").unwrap());
        }
        if exponent == FPDecimal::from(43u128) {
            return Some(FPDecimal::from_str("10000000000000000000000000000000000000000000").unwrap());
        }
        if exponent == FPDecimal::from(44u128) {
            return Some(FPDecimal::from_str("100000000000000000000000000000000000000000000").unwrap());
        }
        if exponent == FPDecimal::from(45u128) {
            return Some(FPDecimal::from_str("1000000000000000000000000000000000000000000000").unwrap());
        }
        if exponent == FPDecimal::from(46u128) {
            return Some(FPDecimal::from_str("10000000000000000000000000000000000000000000000").unwrap());
        }
        if exponent == FPDecimal::from(47u128) {
            return Some(FPDecimal::from_str("100000000000000000000000000000000000000000000000").unwrap());
        }
        if exponent == FPDecimal::from(48u128) {
            return Some(FPDecimal::from_str("1000000000000000000000000000000000000000000000000").unwrap());
        }
        if exponent == FPDecimal::from(49u128) {
            return Some(FPDecimal::from_str("10000000000000000000000000000000000000000000000000").unwrap());
        }
        if exponent == FPDecimal::from(50u128) {
            return Some(FPDecimal::from_str("100000000000000000000000000000000000000000000000000").unwrap());
        }
        if exponent == FPDecimal::from(51u128) {
            return Some(FPDecimal::from_str("1000000000000000000000000000000000000000000000000000").unwrap());
        }
        if exponent == FPDecimal::from(52u128) {
            return Some(FPDecimal::from_str("10000000000000000000000000000000000000000000000000000").unwrap());
        }
        if exponent == FPDecimal::from(53u128) {
            return Some(FPDecimal::from_str("100000000000000000000000000000000000000000000000000000").unwrap());
        }
        if exponent == FPDecimal::from(54u128) {
            return Some(FPDecimal::from_str("1000000000000000000000000000000000000000000000000000000").unwrap());
        }
        if exponent == FPDecimal::from(55u128) {
            return Some(FPDecimal::from_str("10000000000000000000000000000000000000000000000000000000").unwrap());
        }
        if exponent == FPDecimal::from(56u128) {
            return Some(FPDecimal::from_str("100000000000000000000000000000000000000000000000000000000").unwrap());
        }
        if exponent == FPDecimal::from(57u128) {
            return Some(FPDecimal::from_str("1000000000000000000000000000000000000000000000000000000000").unwrap());
        }
        if exponent == FPDecimal::from(58u128) {
            return Some(FPDecimal::from_str("10000000000000000000000000000000000000000000000000000000000").unwrap());
        }
        if exponent == FPDecimal::from(59u128) {
            return Some(FPDecimal::from_str("100000000000000000000000000000000000000000000000000000000000").unwrap());
        }
        None
    }

    fn eleven_pow(exponent: FPDecimal) -> Option<FPDecimal> {
        if exponent == FPDecimal::ONE {
            return Some(FPDecimal::ELEVEN);
        }
        if exponent == FPDecimal::TWO {
            return Some(FPDecimal::from(121u128));
        }
        if exponent == FPDecimal::THREE {
            return Some(FPDecimal::from(1331u128));
        }
        if exponent == FPDecimal::FOUR {
            return Some(FPDecimal::from(14641u128));
        }
        if exponent == FPDecimal::FIVE {
            return Some(FPDecimal::from(161051u128));
        }
        if exponent == FPDecimal::SIX {
            return Some(FPDecimal::from(1771561u128));
        }
        if exponent == FPDecimal::SEVEN {
            return Some(FPDecimal::from(19487171u128));
        }
        if exponent == FPDecimal::EIGHT {
            return Some(FPDecimal::from(214358881u128));
        }
        if exponent == FPDecimal::NINE {
            return Some(FPDecimal::from(2357947691u128));
        }
        if exponent == FPDecimal::TEN {
            return Some(FPDecimal::from(25937424601u128));
        }
        if exponent == FPDecimal::ELEVEN {
            return Some(FPDecimal::from(285311670611u128));
        }
        if exponent == FPDecimal::from(12u128) {
            return Some(FPDecimal::from(3138428376721u128));
        }
        if exponent == FPDecimal::from(13u128) {
            return Some(FPDecimal::from(34522712143931u128));
        }
        if exponent == FPDecimal::from(14u128) {
            return Some(FPDecimal::from(379749833583241u128));
        }
        if exponent == FPDecimal::from(15u128) {
            return Some(FPDecimal::from(4177248169415651u128));
        }
        if exponent == -FPDecimal::ONE {
            return Some(FPDecimal::ONE / FPDecimal::ELEVEN);
        }
        if exponent == -FPDecimal::TWO {
            return Some(FPDecimal::ONE / FPDecimal::from(121u128));
        }
        if exponent == -FPDecimal::THREE {
            return Some(FPDecimal::ONE / FPDecimal::from(1331u128));
        }
        if exponent == -FPDecimal::FOUR {
            return Some(FPDecimal::ONE / FPDecimal::from(14641u128));
        }
        if exponent == -FPDecimal::FIVE {
            return Some(FPDecimal::ONE / FPDecimal::from(161051u128));
        }
        if exponent == -FPDecimal::SIX {
            return Some(FPDecimal::ONE / FPDecimal::from(1771561u128));
        }
        if exponent == -FPDecimal::SEVEN {
            return Some(FPDecimal::ONE / FPDecimal::from(19487171u128));
        }
        if exponent == -FPDecimal::EIGHT {
            return Some(FPDecimal::ONE / FPDecimal::from(214358881u128));
        }
        if exponent == -FPDecimal::NINE {
            return Some(FPDecimal::ONE / FPDecimal::from(2357947691u128));
        }
        if exponent == -FPDecimal::TEN {
            return Some(FPDecimal::ONE / FPDecimal::from(25937424601u128));
        }
        if exponent == -FPDecimal::ELEVEN {
            return Some(FPDecimal::ONE / FPDecimal::from(285311670611u128));
        }
        if exponent == -FPDecimal::from(12u128) {
            return Some(FPDecimal::ONE / FPDecimal::from(3138428376721u128));
        }
        if exponent == -FPDecimal::from(13u128) {
            return Some(FPDecimal::ONE / FPDecimal::from(34522712143931u128));
        }
        if exponent == -FPDecimal::from(14u128) {
            return Some(FPDecimal::ONE / FPDecimal::from(379749833583241u128));
        }
        if exponent == -FPDecimal::from(15u128) {
            return Some(FPDecimal::ONE / FPDecimal::from(4177248169415651u128));
        }

        None
    }

    pub fn pow(self, exponent: FPDecimal) -> Result<FPDecimal, FPDecimalError> {
        if self.is_zero() {
            match exponent.cmp(&FPDecimal::ZERO) {
                Ordering::Greater => return Ok(self),
                Ordering::Equal => return Ok(FPDecimal::ONE),
                Ordering::Less => return Err(FPDecimalError::NotSupported("Not supported".to_owned())),
            }
        }
        if self > FPDecimal::ZERO && exponent == FPDecimal::ZERO {
            return Ok(FPDecimal::ONE);
        }
        // NOTE: x^(1/3) won't be precise
        if exponent == FPDecimal::ONE / FPDecimal::TWO {
            return self.sqrt();
        }
        fn common_const_checks(base: FPDecimal, exponent: FPDecimal) -> Option<FPDecimal> {
            if base == FPDecimal::ONE {
                return Some(FPDecimal::ONE);
            }
            // type
            type BaseFunction<'a> = (&'a dyn Fn(FPDecimal) -> Option<FPDecimal>, FPDecimal);

            let basic_check: [BaseFunction; 7] = [
                (&FPDecimal::two_pow, FPDecimal::TWO),
                (&FPDecimal::e_pow, FPDecimal::E),
                (&FPDecimal::three_pow, FPDecimal::THREE),
                (&FPDecimal::five_pow, FPDecimal::FIVE),
                (&FPDecimal::seven_pow, FPDecimal::SEVEN),
                (&FPDecimal::ten_pow, FPDecimal::TEN),
                (&FPDecimal::eleven_pow, FPDecimal::ELEVEN),
            ];

            for (exp_fn, basic_case) in basic_check {
                if base == basic_case {
                    return exp_fn(exponent);
                }
            }
            None
        }

        if self < FPDecimal::ZERO {
            if exponent.is_int() {
                if exponent % FPDecimal::TWO == FPDecimal::ONE {
                    if let Some(value) = common_const_checks(-self, exponent) {
                        return Ok(-value);
                    }
                } else if exponent % FPDecimal::TWO == FPDecimal::ZERO {
                    if let Some(value) = common_const_checks(-self, exponent) {
                        return Ok(value);
                    }
                }
                return Ok(FPDecimal::exp(exponent * (-self).ln()));
            } else {
                return Err(FPDecimalError::NotSupported("No complex number".to_owned()));
            }
        }
        if exponent.abs() == FPDecimal::ONE / FPDecimal::TWO {
            if exponent > FPDecimal::ZERO {
                return self.sqrt();
            } else {
                return Ok(FPDecimal::ONE / self.sqrt().unwrap());
            }
        }

        fn common_checks(exponent: FPDecimal) -> Option<FPDecimal> {
            if FPDecimal::TWO.ln() == exponent {
                return Some(FPDecimal::TWO);
            }
            if FPDecimal::THREE.ln() == exponent {
                return Some(FPDecimal::THREE);
            }
            if FPDecimal::FIVE.ln() == exponent {
                return Some(FPDecimal::FIVE);
            }
            if FPDecimal::SEVEN.ln() == exponent {
                return Some(FPDecimal::SEVEN);
            }
            if FPDecimal::TEN.ln() == exponent {
                return Some(FPDecimal::TEN);
            }
            if FPDecimal::ELEVEN.ln() == exponent {
                return Some(FPDecimal::ELEVEN);
            }
            None
        }

        type BaseCheckFunction<'a> = (&'a dyn Fn(FPDecimal) -> Option<FPDecimal>, FPDecimal);

        if let Some(value) = common_const_checks(self, exponent) {
            return Ok(value);
        }

        match common_checks(exponent) {
            Some(value) => Ok(value),
            None => {
                let base_checks: Vec<BaseCheckFunction> = vec![
                    (&FPDecimal::log_e, FPDecimal::E),
                    (&FPDecimal::log2, FPDecimal::TWO),
                    (&FPDecimal::log3, FPDecimal::THREE),
                    (&FPDecimal::log5, FPDecimal::FIVE),
                    (&FPDecimal::log7, FPDecimal::SEVEN),
                    (&FPDecimal::log10, FPDecimal::TEN),
                    (&FPDecimal::log11, FPDecimal::ELEVEN),
                ];
                for (log_fn, divisor) in base_checks {
                    if let Some(value) = log_fn(exponent) {
                        if self == divisor {
                            return Ok(value);
                        }
                    }
                    if let Some(value) = log_fn(self) {
                        if FPDecimal::ONE / value == exponent {
                            return Ok(divisor);
                        }
                    }
                }
                Ok(FPDecimal::exp(exponent * self.ln()))
            }
        }
    }

    pub fn exp(a: FPDecimal) -> FPDecimal {
        // this throws underflow with a sufficiently large negative exponent
        // short circuit and just return 0 above a certain threshold
        // otherwise if there is a long enough delay between updates on a cluster
        // the penalty function will be bricked
        if a.sign == 0 && a.num >= FPDecimal::from(45i128).num {
            return FPDecimal::ZERO;
        }
        let mut x = a.num;
        let mut r = FPDecimal::ONE;
        while x >= U256([10, 0, 0, 0]) * FPDecimal::ONE.num {
            x -= U256([10, 0, 0, 0]) * FPDecimal::ONE.num;
            r = FPDecimal::_mul(r, FPDecimal::E_10);
        }
        if x == FPDecimal::ONE.num {
            let val = FPDecimal::_mul(r, FPDecimal::E);
            if a.sign == 0 {
                return FPDecimal::reciprocal(val);
            }
            return val;
        } else if x == FPDecimal::ZERO.num {
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
            tr += d;
        }
        let val = FPDecimal::_mul(FPDecimal { num: tr, sign: 1 }, r);
        if a.sign == 0 {
            return FPDecimal::reciprocal(val);
        }
        val
    }

    pub fn sqrt(self) -> Result<FPDecimal, FPDecimalError> {
        const MAX_ITERATIONS: i64 = 300;

        if self < FPDecimal::ZERO {
            return Err(FPDecimalError::NotSupported("No complex number".to_owned()));
            // return None;
        }

        if self.is_zero() {
            return Ok(FPDecimal::ZERO);
        }

        // Start with an arbitrary number as the first guess
        let mut r = self / FPDecimal::TWO;
        let mut l = r + FPDecimal::ONE;

        // Keep going while the difference is larger than the tolerance
        let mut c = 0i64;
        while (l != r) && (c < MAX_ITERATIONS) {
            l = r;
            r = (r + self / r) / FPDecimal::TWO;
            c += 1;
        }

        Ok(r)
    }
    // pub fn sqrt(self) -> FPDecimal {
    //     match FPDecimal::_sqrt(self) {
    //         Some(value) => value,
    //         None => panic!("Undefined behavior"),
    //     }
    // }
}

#[cfg(test)]
mod tests {

    use crate::fp_decimal::error::FPDecimalError;
    use crate::fp_decimal::U256;
    use crate::FPDecimal;
    use std::str::FromStr;

    #[test]
    fn test_3_pow_2_point_3() {
        // a^x = e^(xln(a))
        // 3^2.3 = e(2.3ln(3))
        assert_eq!(
            FPDecimal::exp_taylor_expansion(FPDecimal::THREE, FPDecimal::must_from_str("2.3")),
            // 12.513502532843181622
            FPDecimal::must_from_str("12.513502532843184097")
        );
    }

    #[test]
    fn test_exp() {
        assert_eq!(FPDecimal::exp(FPDecimal::ONE), FPDecimal::E);
    }
    #[test]
    fn test_exp_x_greater_than_neg_one() {
        assert_eq!(
            FPDecimal::exp(FPDecimal::must_from_str("-0.0001")),
            FPDecimal::must_from_str("0.999900004999833338")
        );

        assert_eq!(
            FPDecimal::exp(FPDecimal::must_from_str("-0.001")),
            FPDecimal::must_from_str("0.999000499833374993")
        );

        assert_eq!(
            FPDecimal::exp(FPDecimal::must_from_str("-0.01")),
            FPDecimal::must_from_str("0.990049833749168057")
        );

        assert_eq!(
            FPDecimal::exp(FPDecimal::must_from_str("-0.1")),
            FPDecimal::must_from_str("0.904837418035959577")
        );

        assert_eq!(
            FPDecimal::exp(FPDecimal::must_from_str("-0.3")),
            FPDecimal::must_from_str("0.740818220681717868")
        );

        assert_eq!(
            FPDecimal::exp(FPDecimal::must_from_str("-0.5")),
            FPDecimal::must_from_str("0.606530659712633426")
        );

        assert_eq!(
            FPDecimal::exp(FPDecimal::must_from_str("-0.79")),
            FPDecimal::must_from_str("0.453844795282355824")
        );

        assert_eq!(
            FPDecimal::exp(FPDecimal::must_from_str("-0.89")),
            FPDecimal::must_from_str("0.410655752752345489")
        );
        assert_eq!(
            FPDecimal::exp(FPDecimal::must_from_str("-0.9")),
            FPDecimal::must_from_str("0.406569659740599113")
        );
        assert_eq!(
            FPDecimal::exp(FPDecimal::must_from_str("-0.99")),
            FPDecimal::must_from_str("0.371576691022045691")
        );
    }

    #[test]
    fn test_exp_x_smaller_than_neg_one() {
        assert_eq!(FPDecimal::exp(-FPDecimal::ONE), FPDecimal::ONE / FPDecimal::E);
        assert_eq!(FPDecimal::exp(-FPDecimal::TWO), FPDecimal::must_from_str("0.135335283236612692"));
        assert_eq!(
            FPDecimal::exp(-FPDecimal::THREE),
            FPDecimal::ONE / (FPDecimal::E * FPDecimal::E * FPDecimal::E)
        );
        assert_eq!(
            FPDecimal::exp(-FPDecimal::FOUR),
            FPDecimal::ONE / (FPDecimal::E * FPDecimal::E * FPDecimal::E * FPDecimal::E)
        );
    }

    #[test]
    fn test_exp_x_smaller_than_one() {
        assert_eq!(FPDecimal::exp(FPDecimal::ONE), FPDecimal::E);
        assert_eq!(
            FPDecimal::exp(FPDecimal::must_from_str("0.79")),
            FPDecimal::must_from_str("2.203396426255936650")
        );

        assert_eq!(
            FPDecimal::exp(FPDecimal::must_from_str("0.89")),
            FPDecimal::must_from_str("2.435129651289874518")
        );
        assert_eq!(
            FPDecimal::exp(FPDecimal::must_from_str("0.9")),
            FPDecimal::must_from_str("2.459603111156949656")
        );
        assert_eq!(
            FPDecimal::exp(FPDecimal::must_from_str("0.99")),
            FPDecimal::must_from_str("2.691234472349262282")
        );
    }

    #[test]
    fn test_exp_x_greater_than_one() {
        assert_eq!(FPDecimal::exp(FPDecimal::ONE), FPDecimal::E);
        assert_eq!(
            FPDecimal::exp(FPDecimal::must_from_str("1.0001")),
            FPDecimal::must_from_str("2.718553670233753334")
        );

        assert_eq!(
            FPDecimal::exp(FPDecimal::must_from_str("1.001")),
            FPDecimal::must_from_str("2.721001469881578756")
        );
        assert_eq!(
            FPDecimal::exp(FPDecimal::must_from_str("1.01")),
            FPDecimal::must_from_str("2.745601015016916484")
        );
        assert_eq!(
            FPDecimal::exp(FPDecimal::must_from_str("1.1")),
            FPDecimal::must_from_str("3.004166023946433102")
        );
        assert_eq!(
            FPDecimal::exp(FPDecimal::must_from_str("1.2")),
            FPDecimal::must_from_str("3.320116922736547481")
        );

        assert_eq!(FPDecimal::exp(FPDecimal::TWO), FPDecimal::must_from_str("7.389056098930650216"));
        assert_eq!(FPDecimal::exp(FPDecimal::THREE), FPDecimal::must_from_str("20.085536923187667729"));
        assert_eq!(FPDecimal::exp(FPDecimal::FOUR), FPDecimal::must_from_str("54.598150033144239058"));
        assert_eq!(FPDecimal::exp(FPDecimal::FIVE), FPDecimal::must_from_str("148.413159102576603394"));
        assert_eq!(FPDecimal::exp(FPDecimal::SIX), FPDecimal::must_from_str("403.428793492735117251"));
        assert_eq!(FPDecimal::exp(FPDecimal::SEVEN), FPDecimal::must_from_str("1096.633158428456948182"));
        assert_eq!(FPDecimal::exp(FPDecimal::EIGHT), FPDecimal::must_from_str("2980.957987041489775723"));
    }

    #[test]
    fn test_exp0() {
        assert_eq!(FPDecimal::exp(FPDecimal::ZERO), FPDecimal::ONE);
    }

    #[test]
    fn test_exp10() {
        assert_eq!(FPDecimal::exp(FPDecimal::TEN), FPDecimal::E_10);
    }
    #[test]
    fn test_pow_neg() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::ZERO, -FPDecimal::ONE.div(2i128)).unwrap_err(),
            FPDecimalError::NotSupported("Not supported".to_owned())
        );
    }

    #[test]
    fn test_pow_zero() {
        assert_eq!(FPDecimal::ZERO.pow(FPDecimal::ONE).unwrap(), FPDecimal::ZERO);
    }

    #[test]
    fn test_4_pow_0_5() {
        assert_eq!(FPDecimal::pow(FPDecimal::FOUR, FPDecimal::must_from_str("0.5")).unwrap(), FPDecimal::TWO);
    }

    #[test]
    fn test_128_pow_0_5() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(128u128), FPDecimal::must_from_str("0.5")).unwrap(),
            FPDecimal::must_from_str("11.313708498984760390")
        );
    }

    #[test]
    fn test_128_pow_1_7() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(128u128), FPDecimal::ONE / FPDecimal::SEVEN).unwrap(),
            FPDecimal::TWO
        );
    }

    #[test]
    fn test_9_pow_0_5() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::NINE, FPDecimal::must_from_str("0.5")).unwrap(),
            FPDecimal::THREE
        );
    }

    #[test]
    fn test_27_pow_0_5() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(27u128), FPDecimal::must_from_str("0.5")).unwrap(),
            FPDecimal::must_from_str("5.196152422706631880")
        );
    }
    #[test]
    fn test_27_pow_1_over_3() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(27u128), FPDecimal::ONE / FPDecimal::THREE).unwrap(),
            FPDecimal::THREE
        );
    }

    #[test]
    fn test_81_pow_0_25() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(81u128), FPDecimal::ONE / FPDecimal::FOUR).unwrap(),
            FPDecimal::THREE
        );
    }

    #[test]
    fn test_81_pow_0_5() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(81u128), FPDecimal::ONE / FPDecimal::TWO).unwrap(),
            FPDecimal::NINE
        );
    }

    #[test]
    fn test_25_pow_0_5() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(25u128), FPDecimal::must_from_str("0.5")).unwrap(),
            FPDecimal::FIVE
        );
    }

    #[test]
    fn test_125_pow_0_5() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(125u128), FPDecimal::must_from_str("0.5")).unwrap(),
            FPDecimal::must_from_str("11.180339887498948482")
        );
    }
    #[test]
    fn test_125_pow_1_over_3() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(125u128), FPDecimal::ONE / FPDecimal::THREE).unwrap(),
            FPDecimal::FIVE
        );
    }

    #[test]
    fn test_625_pow_0_25() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(625u128), FPDecimal::ONE / FPDecimal::FOUR).unwrap(),
            FPDecimal::FIVE
        );
    }

    #[test]
    fn test_49_pow_0_5() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(49u128), FPDecimal::must_from_str("0.5")).unwrap(),
            FPDecimal::SEVEN
        );
    }

    #[test]
    fn test_343_pow_0_5() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(343u128), FPDecimal::must_from_str("0.5")).unwrap(),
            FPDecimal::must_from_str("18.520259177452134133")
        );
    }
    #[test]
    fn test_343_pow_1_over_3() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(343u128), FPDecimal::ONE / FPDecimal::THREE).unwrap(),
            FPDecimal::SEVEN
        );
    }

    #[test]
    fn test_2401_pow_0_25() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(2401u128), FPDecimal::ONE / FPDecimal::FOUR).unwrap(),
            FPDecimal::SEVEN
        );
    }

    #[test]
    fn test_121_pow_0_5() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(121u128), FPDecimal::must_from_str("0.5")).unwrap(),
            FPDecimal::ELEVEN
        );
    }

    #[test]
    fn test_1331_pow_0_5() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(1331u128), FPDecimal::must_from_str("0.5")).unwrap(),
            FPDecimal::must_from_str("36.48287269390939834")
        );
    }
    #[test]
    fn test_1331_pow_1_over_3() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(1331u128), FPDecimal::ONE / FPDecimal::THREE).unwrap(),
            FPDecimal::ELEVEN
        );
    }

    #[test]
    fn test_14641_pow_0_25() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(14641u128), FPDecimal::ONE / FPDecimal::FOUR).unwrap(),
            FPDecimal::ELEVEN
        );
    }

    #[test]
    fn test_2_pow_1() {
        assert_eq!(FPDecimal::TWO.pow(FPDecimal::ONE).unwrap(), FPDecimal::TWO);
    }

    #[test]
    fn test_3_pow_1() {
        assert_eq!(FPDecimal::THREE.pow(FPDecimal::ONE).unwrap(), FPDecimal::THREE);
    }

    #[test]
    fn test_pow_exp_1() {
        assert_eq!(FPDecimal::E.pow(FPDecimal::ONE).unwrap(), FPDecimal::E);
    }

    #[test]
    fn test_pow_exp_0() {
        assert_eq!(FPDecimal::E.pow(FPDecimal::ZERO).unwrap(), FPDecimal::ONE);
    }

    #[test]
    fn test_pow_exp_10() {
        assert_eq!(
            FPDecimal::E
                .pow(FPDecimal {
                    num: U256([10, 0, 0, 0]) * FPDecimal::ONE.num,
                    sign: 1
                })
                .unwrap(),
            FPDecimal::E_10
        );
    }

    #[test]
    fn test_pow_zero_2() {
        assert_eq!(FPDecimal::ZERO.pow(FPDecimal::ONE.div(2i128)).unwrap(), FPDecimal::ZERO);
    }

    #[test]
    fn test_square_root() {
        let inputs: Vec<i128> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 16, 25, -1];

        let expected = vec![
            Ok(FPDecimal::ZERO),
            Ok(FPDecimal::ONE),
            Ok(FPDecimal::from_str("1.414213562373095048").unwrap()),
            Ok(FPDecimal::from_str("1.732050807568877293").unwrap()),
            Ok(FPDecimal::TWO),
            Ok(FPDecimal::from_str("2.236067977499789696").unwrap()),
            Ok(FPDecimal::from_str("2.449489742783178098").unwrap()),
            Ok(FPDecimal::from_str("2.645751311064590590").unwrap()),
            Ok(FPDecimal::from_str("2.828427124746190097").unwrap()),
            Ok(FPDecimal::THREE),
            Ok(FPDecimal::from_str("3.162277660168379331").unwrap()),
            Ok(FPDecimal::FOUR),
            Ok(FPDecimal::FIVE),
            Err(FPDecimalError::NotSupported("No complex number".to_owned())),
        ];

        for (ix, el) in inputs.iter().enumerate() {
            let result = FPDecimal::from(*el).sqrt();

            assert_eq!(result, expected[ix]);
        }
    }

    #[test]
    fn test_pow_10_positive() {
        let base = FPDecimal::from(10u128);
        assert_eq!(base.pow(FPDecimal::must_from_str("6")).unwrap(), FPDecimal::must_from_str("1000000"));
    }

    #[test]
    fn test_pow_10_max() {
        let base = FPDecimal::from(10u128);
        assert_eq!(
            base.pow(FPDecimal::must_from_str("59")).unwrap(),
            FPDecimal::from_str("100000000000000000000000000000000000000000000000000000000000").unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn test_pow_10_overflow() {
        let base = FPDecimal::from(10u128);
        base.pow(FPDecimal::must_from_str("60")).unwrap();
    }

    #[test]
    fn test_pow_10_neg_3() {
        let base = FPDecimal::from(10u128);
        assert_eq!(base.pow(FPDecimal::must_from_str("-3")).unwrap(), FPDecimal::must_from_str("0.001"));
    }

    #[test]
    fn test_e_pow_neg_3() {
        let base = FPDecimal::E;
        assert_eq!(
            base.pow(FPDecimal::must_from_str("-3")).unwrap(),
            FPDecimal::must_from_str("0.049787068367863943")
        );
    }

    #[test]
    fn test_e_pow_0_5() {
        assert_eq!(
            FPDecimal::E.pow(FPDecimal::must_from_str("0.5")).unwrap(),
            // 1.6487212707001281469
            FPDecimal::must_from_str("1.648721270700128146")
        );
    }

    #[test]
    fn test_pow_10_min() {
        let base = FPDecimal::from(10u128);
        assert_eq!(
            base.pow(FPDecimal::must_from_str("-18")).unwrap(),
            FPDecimal::must_from_str("0.000000000000000001")
        );
    }

    #[test]
    fn test_pow_10_underflow() {
        let base = FPDecimal::from(10u128);
        assert_eq!(base.pow(FPDecimal::must_from_str("-19")).unwrap(), FPDecimal::ZERO);
    }

    #[test]
    fn test_checked_2_pow_2() {
        let base = FPDecimal::from(2u128);

        let result = FPDecimal::pow(base, FPDecimal::from(2u128)).unwrap();
        assert_eq!(result, FPDecimal::from(4u128));
    }

    #[test]
    fn test_2_3_pow_1_4() {
        let base = FPDecimal::must_from_str("2.3");
        let exponent = FPDecimal::must_from_str("1.4");
        // let result_1 = FPDecimal::checked_positive_pow(base, exponent).unwrap();
        let result_2 = FPDecimal::pow(base, exponent).unwrap();
        assert_eq!(result_2, FPDecimal::must_from_str("3.209363953267971906"));
    }

    #[test]
    fn test_2_3_pow_3_7() {
        let base = FPDecimal::must_from_str("2.3");
        let exponent = FPDecimal::must_from_str("3.7");
        let result_2 = FPDecimal::pow(base, exponent).unwrap();
        //21.796812747431183828
        assert_eq!(result_2, FPDecimal::must_from_str("21.796812747431186181"));
    }

    #[test]
    fn test_2_3_pow_neg_1_4() {
        let base = FPDecimal::must_from_str("2.3");
        let exponent = FPDecimal::must_from_str("-1.4");
        // let result_1 = FPDecimal::checked_positive_pow(base, exponent).unwrap();
        let result_2 = FPDecimal::pow(base, exponent).unwrap();
        // 0.31158821952298012815
        assert_eq!(result_2, FPDecimal::must_from_str("0.311588219522980075"));
        // assert_eq!(result_1, FPDecimal::must_from_str("0.311588219522980069"));
    }

    #[test]
    fn test_2_3_pow_neg_3_7() {
        let base = FPDecimal::must_from_str("2.3");
        let exponent = FPDecimal::must_from_str("-3.7");
        // let result_1 = FPDecimal::checked_positive_pow(base, exponent).unwrap();
        let result_2 = FPDecimal::pow(base, exponent).unwrap();
        // 0.045878267230508407006
        assert_eq!(result_2, FPDecimal::must_from_str("0.045878267230508402"));
        // assert_eq!(result_1, FPDecimal::must_from_str("0.045878267230507924"));
    }

    #[test]
    fn test_2_3_pow_0_4() {
        let base = FPDecimal::must_from_str("2.3");
        let exponent = FPDecimal::must_from_str("0.4");
        let result_2 = FPDecimal::pow(base, exponent).unwrap();
        assert_eq!(result_2, FPDecimal::must_from_str("1.395375631855639968"));
    }

    #[test]
    fn test_2_3_pow_neg_0_4() {
        let base = FPDecimal::must_from_str("2.3");
        let exponent = FPDecimal::must_from_str("-0.4");
        let result_2 = FPDecimal::pow(base, exponent).unwrap();
        // 0.71665290490285417314
        assert_eq!(result_2, FPDecimal::must_from_str("0.716652904902854170"));
    }

    #[test]
    fn test_1_over_16_pow_neg_0_5() {
        let base = FPDecimal::ONE / FPDecimal::from(16u128);
        let exponent = FPDecimal::must_from_str("-0.5");

        let result = FPDecimal::pow(base, exponent).unwrap();
        assert_eq!(result, FPDecimal::FOUR);
    }

    #[test]
    fn test_1_over_16_pow_0_5() {
        let base = FPDecimal::ONE / FPDecimal::from(16u128);
        let exponent = FPDecimal::must_from_str("0.5");

        // let result = FPDecimal::checked_positive_pow(base, exponent).unwrap();
        let result = FPDecimal::pow(base, exponent).unwrap();
        assert_eq!(result, FPDecimal::ONE / FPDecimal::FOUR);
    }

    #[test]
    fn test_100_pow_neg_1_over_2() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(100u128), FPDecimal::must_from_str("-0.5")).unwrap(),
            FPDecimal::must_from_str("0.1")
        );
    }

    #[test]
    fn test_1000_pow_1_over_3() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::from(1000u128), FPDecimal::ONE / FPDecimal::THREE).unwrap(),
            FPDecimal::TEN
        );
    }

    #[test]
    fn test_neg_1000_pow_1_over_3() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::must_from_str("-1000.0"), FPDecimal::ONE / FPDecimal::THREE).unwrap_err(),
            FPDecimalError::NotSupported("No complex number".to_owned())
        );
    }

    #[test]
    fn test_neg_10_pow_3() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::must_from_str("-10"), FPDecimal::THREE).unwrap(),
            -FPDecimal::TEN.pow(FPDecimal::THREE).unwrap()
        );
    }

    #[test]
    fn test_neg_10_pow_4() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::must_from_str("-10"), FPDecimal::FOUR),
            FPDecimal::TEN.pow(FPDecimal::FOUR)
        );
    }

    #[test]
    fn test_neg_10_pow_2_3() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::must_from_str("-10"), FPDecimal::must_from_str("2.3")).unwrap_err(),
            FPDecimalError::NotSupported("No complex number".to_owned())
        );
    }

    #[test]
    #[should_panic]
    fn test_neg_1000_pow_1_over_4() {
        assert_eq!(
            FPDecimal::pow(FPDecimal::must_from_str("-1000.0"), FPDecimal::ONE / FPDecimal::FOUR).unwrap(),
            -FPDecimal::TEN
        );
    }

    #[test]
    fn test_exp_log_2() {
        // assert_eq!(FPDecimal::E.pow(FPDecimal::TWO.ln()), FPDecimal::must_from_str("2.0"));
        assert_eq!(FPDecimal::E.pow(FPDecimal::TWO.ln()).unwrap(), FPDecimal::must_from_str("2.0"));
    }

    #[test]
    fn test_sqrt() {
        assert_eq!(FPDecimal::FOUR.sqrt().unwrap(), FPDecimal::TWO);
        assert_eq!(FPDecimal::from(16u128).sqrt().unwrap(), FPDecimal::FOUR);
        assert_eq!(FPDecimal::ONE.sqrt().unwrap(), FPDecimal::ONE);
        assert_eq!(FPDecimal::NINE.sqrt().unwrap(), FPDecimal::THREE);
        assert_eq!(FPDecimal::from(81u128).sqrt().unwrap(), FPDecimal::NINE);
    }
    #[test]
    fn test_1_power_n() {
        assert_eq!(FPDecimal::ONE, FPDecimal::ONE.pow(FPDecimal::ONE).unwrap());
        assert_eq!(FPDecimal::ONE, FPDecimal::ONE.pow(FPDecimal::TWO).unwrap());
        assert_eq!(FPDecimal::ONE, FPDecimal::ONE.pow(FPDecimal::THREE).unwrap());
        assert_eq!(FPDecimal::ONE, FPDecimal::ONE.pow(FPDecimal::FOUR).unwrap());
    }

    // NOTE: its ok we ignore these two unit tests. they should not be the goal of this crate
    // TODO: add a lookup table for this in future
    /*
    #[test]
    fn test_ln_e_pow_1_5() {
        let base = FPDecimal::E * FPDecimal::E.sqrt();
        assert_eq!(base.ln(), FPDecimal::must_from_str("1.5"));
    }

    #[test]
    fn test_ln_e_pow_minus_1_5() {
        let base = FPDecimal::ONE / (FPDecimal::E.sqrt() * FPDecimal::E);
        assert_eq!(base.ln(), FPDecimal::must_from_str("-1.5"));
    }
    #[test]
    fn test_log_1_5_2_2_5() {
        let base = FPDecimal::must_from_str("1.5");
        let exp = FPDecimal::must_from_str("2.25");
        assert_eq!(exp.log(base), FPDecimal::TWO);
    }

    #[test]
    fn test_25_pow_0_11111() {
        let power = FPDecimal::ONE / FPDecimal::from(9_u128);
        let result: FPDecimal = FPDecimal::must_from_str("25.0").ln() * power;
        let dampen: FPDecimal = FPDecimal::E.pow(result);
        //                                           1.4299640339921836144
        assert_eq!(dampen, FPDecimal::must_from_str("1.429969148308728731"));
    }
    #[test]
    fn test_25_pow_0_11111_decimal_lib() {
        let x = FPDecimal::ONE / FPDecimal::from(9_u128);
        let a: FPDecimal = FPDecimal::must_from_str("25.0");
        let result: FPDecimal = a.pow(x);
        assert_eq!(result, FPDecimal::must_from_str("1.429969148308728731"));
    }
    */
}
