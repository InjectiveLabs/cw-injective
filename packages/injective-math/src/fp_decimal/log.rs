use crate::fp_decimal::error::FPDecimalError;
/// Logarithmic functions for FPDecimal
use crate::fp_decimal::{FPDecimal, U256};

impl FPDecimal {
    pub(crate) fn log_e(self) -> Option<FPDecimal> {
        let e = FPDecimal::E;
        if self == FPDecimal::ONE {
            return Some(FPDecimal::ZERO);
        }
        if self == FPDecimal::E {
            return Some(FPDecimal::ONE);
        }
        if self == e * e {
            return Some(FPDecimal::TWO);
        }
        if self == e * e * e {
            return Some(FPDecimal::THREE);
        }
        if self == e * e * e * e {
            return Some(FPDecimal::FOUR);
        }
        if self == e * e * e * e * e {
            return Some(FPDecimal::FIVE);
        }
        if self == e * e * e * e * e * e {
            return Some(FPDecimal::SIX);
        }
        if self == e * e * e * e * e * e * e {
            return Some(FPDecimal::SEVEN);
        }
        if self == e * e * e * e * e * e * e * e {
            return Some(FPDecimal::EIGHT);
        }
        if self == e * e * e * e * e * e * e * e * e {
            return Some(FPDecimal::NINE);
        }
        if self == e * e * e * e * e * e * e * e * e * e {
            return Some(FPDecimal::TEN);
        }
        if self == e * e * e * e * e * e * e * e * e * e * e {
            return Some(FPDecimal::ELEVEN);
        }

        if self == FPDecimal::ONE / FPDecimal::E {
            return Some(-FPDecimal::ONE);
        }
        if self == FPDecimal::ONE / (e * e) {
            return Some(-FPDecimal::TWO);
        }
        if self == FPDecimal::ONE / (e * e * e) {
            return Some(-FPDecimal::THREE);
        }
        if self == FPDecimal::ONE / (e * e * e * e) {
            return Some(-FPDecimal::FOUR);
        }
        if self == FPDecimal::ONE / (e * e * e * e * e) {
            return Some(-FPDecimal::FIVE);
        }
        if self == FPDecimal::ONE / (e * e * e * e * e * e) {
            return Some(-FPDecimal::SIX);
        }
        if self == FPDecimal::ONE / (e * e * e * e * e * e * e) {
            return Some(-FPDecimal::SEVEN);
        }
        if self == FPDecimal::ONE / (e * e * e * e * e * e * e * e) {
            return Some(-FPDecimal::EIGHT);
        }
        if self == FPDecimal::ONE / (e * e * e * e * e * e * e * e * e) {
            return Some(-FPDecimal::NINE);
        }
        if self == FPDecimal::ONE / (e * e * e * e * e * e * e * e * e * e) {
            return Some(-FPDecimal::TEN);
        }
        if self == FPDecimal::ONE / (e * e * e * e * e * e * e * e * e * e * e) {
            return Some(-FPDecimal::ELEVEN);
        }
        None
    }

    pub(crate) fn log2(self) -> Option<FPDecimal> {
        if self == FPDecimal::ONE {
            return Some(FPDecimal::ZERO);
        }
        if self == FPDecimal::TWO {
            return Some(FPDecimal::ONE);
        }
        if self == FPDecimal::FOUR {
            return Some(FPDecimal::TWO);
        }
        if self == FPDecimal::EIGHT {
            return Some(FPDecimal::THREE);
        }
        if self == FPDecimal::from(16u128) {
            return Some(FPDecimal::FOUR);
        }
        if self == FPDecimal::from(32u128) {
            return Some(FPDecimal::FIVE);
        }
        if self == FPDecimal::from(64u128) {
            return Some(FPDecimal::SIX);
        }
        if self == FPDecimal::from(128u128) {
            return Some(FPDecimal::SEVEN);
        }
        if self == FPDecimal::from(256u128) {
            return Some(FPDecimal::EIGHT);
        }
        if self == FPDecimal::from(512u128) {
            return Some(FPDecimal::NINE);
        }
        if self == FPDecimal::from(1024u128) {
            return Some(FPDecimal::TEN);
        }
        if self == FPDecimal::from(2048u128) {
            return Some(FPDecimal::ELEVEN);
        }
        if self == FPDecimal::from(4096u128) {
            return Some(FPDecimal::from(12u128));
        }
        if self == FPDecimal::from(8192u128) {
            return Some(FPDecimal::from(13u128));
        }
        if self == FPDecimal::from(16384u128) {
            return Some(FPDecimal::from(14u128));
        }
        if self == FPDecimal::from(32768u128) {
            return Some(FPDecimal::from(15u128));
        }

        if self == FPDecimal::ONE / FPDecimal::TWO {
            return Some(-FPDecimal::ONE);
        }
        if self == FPDecimal::ONE / FPDecimal::FOUR {
            return Some(-FPDecimal::TWO);
        }
        if self == FPDecimal::ONE / FPDecimal::EIGHT {
            return Some(-FPDecimal::THREE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(16u128) {
            return Some(-FPDecimal::FOUR);
        }
        if self == FPDecimal::ONE / FPDecimal::from(32u128) {
            return Some(-FPDecimal::FIVE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(64u128) {
            return Some(-FPDecimal::SIX);
        }
        if self == FPDecimal::ONE / FPDecimal::from(128u128) {
            return Some(-FPDecimal::SEVEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(256u128) {
            return Some(-FPDecimal::EIGHT);
        }
        if self == FPDecimal::ONE / FPDecimal::from(512u128) {
            return Some(-FPDecimal::NINE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(1024u128) {
            return Some(-FPDecimal::TEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(2048u128) {
            return Some(-FPDecimal::ELEVEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(4096u128) {
            return Some(-FPDecimal::from(12u128));
        }
        if self == FPDecimal::ONE / FPDecimal::from(8192u128) {
            return Some(-FPDecimal::from(13u128));
        }
        if self == FPDecimal::ONE / FPDecimal::from(16384u128) {
            return Some(-FPDecimal::from(14u128));
        }
        if self == FPDecimal::ONE / FPDecimal::from(32768u128) {
            return Some(-FPDecimal::from(15u128));
        }
        None
    }

    pub(crate) fn log3(self) -> Option<FPDecimal> {
        if self == FPDecimal::ONE {
            return Some(FPDecimal::ZERO);
        }
        if self == FPDecimal::THREE {
            return Some(FPDecimal::ONE);
        }
        if self == FPDecimal::NINE {
            return Some(FPDecimal::TWO);
        }
        if self == FPDecimal::from(27u128) {
            return Some(FPDecimal::THREE);
        }
        if self == FPDecimal::from(81u128) {
            return Some(FPDecimal::FOUR);
        }
        if self == FPDecimal::from(243u128) {
            return Some(FPDecimal::FIVE);
        }
        if self == FPDecimal::from(729u128) {
            return Some(FPDecimal::SIX);
        }
        if self == FPDecimal::from(2187u128) {
            return Some(FPDecimal::SEVEN);
        }
        if self == FPDecimal::from(6561u128) {
            return Some(FPDecimal::EIGHT);
        }
        if self == FPDecimal::from(19683u128) {
            return Some(FPDecimal::NINE);
        }
        if self == FPDecimal::from(59049u128) {
            return Some(FPDecimal::TEN);
        }
        if self == FPDecimal::from(177147u128) {
            return Some(FPDecimal::ELEVEN);
        }
        if self == FPDecimal::from(531441u128) {
            return Some(FPDecimal::from(12u128));
        }
        if self == FPDecimal::from(531441u128) {
            return Some(FPDecimal::from(13u128));
        }
        if self == FPDecimal::from(4782969u128) {
            return Some(FPDecimal::from(14u128));
        }
        if self == FPDecimal::from(14348907u128) {
            return Some(FPDecimal::from(15u128));
        }

        if self == FPDecimal::ONE / FPDecimal::THREE {
            return Some(-FPDecimal::ONE);
        }
        if self == FPDecimal::ONE / FPDecimal::NINE {
            return Some(-FPDecimal::TWO);
        }
        if self == FPDecimal::ONE / FPDecimal::from(27u128) {
            return Some(-FPDecimal::THREE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(81u128) {
            return Some(-FPDecimal::FOUR);
        }
        if self == FPDecimal::ONE / FPDecimal::from(243u128) {
            return Some(-FPDecimal::FIVE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(729u128) {
            return Some(-FPDecimal::SIX);
        }
        if self == FPDecimal::ONE / FPDecimal::from(2187u128) {
            return Some(-FPDecimal::SEVEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(6561u128) {
            return Some(-FPDecimal::EIGHT);
        }
        if self == FPDecimal::ONE / FPDecimal::from(19683u128) {
            return Some(-FPDecimal::NINE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(59049u128) {
            return Some(-FPDecimal::TEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(177147u128) {
            return Some(-FPDecimal::ELEVEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(531441u128) {
            return Some(-FPDecimal::from(12u128));
        }
        if self == FPDecimal::ONE / FPDecimal::from(531441u128) {
            return Some(-FPDecimal::from(13u128));
        }
        if self == FPDecimal::ONE / FPDecimal::from(4782969u128) {
            return Some(-FPDecimal::from(14u128));
        }
        if self == FPDecimal::ONE / FPDecimal::from(14348907u128) {
            return Some(-FPDecimal::from(15u128));
        }

        None
    }

    pub(crate) fn log5(self) -> Option<FPDecimal> {
        if self == FPDecimal::ONE {
            return Some(FPDecimal::ZERO);
        }
        if self == FPDecimal::FIVE {
            return Some(FPDecimal::ONE);
        }
        if self == FPDecimal::from(25u128) {
            return Some(FPDecimal::TWO);
        }
        if self == FPDecimal::from(125u128) {
            return Some(FPDecimal::THREE);
        }
        if self == FPDecimal::from(625u128) {
            return Some(FPDecimal::FOUR);
        }
        if self == FPDecimal::from(3125u128) {
            return Some(FPDecimal::FIVE);
        }
        if self == FPDecimal::from(15625u128) {
            return Some(FPDecimal::SIX);
        }
        if self == FPDecimal::from(78125u128) {
            return Some(FPDecimal::SEVEN);
        }
        if self == FPDecimal::from(3906251u128) {
            return Some(FPDecimal::EIGHT);
        }
        if self == FPDecimal::from(1953125u128) {
            return Some(FPDecimal::NINE);
        }
        if self == FPDecimal::from(9765625u128) {
            return Some(FPDecimal::TEN);
        }
        if self == FPDecimal::from(48828125u128) {
            return Some(FPDecimal::ELEVEN);
        }
        if self == FPDecimal::from(244140625u128) {
            return Some(FPDecimal::from(12u128));
        }
        if self == FPDecimal::from(1220703125u128) {
            return Some(FPDecimal::from(13u128));
        }
        if self == FPDecimal::from(6103515625u128) {
            return Some(FPDecimal::from(14u128));
        }
        if self == FPDecimal::from(30517578125u128) {
            return Some(FPDecimal::from(15u128));
        }

        if self == FPDecimal::ONE / FPDecimal::FIVE {
            return Some(-FPDecimal::ONE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(25u128) {
            return Some(-FPDecimal::TWO);
        }
        if self == FPDecimal::ONE / FPDecimal::from(125u128) {
            return Some(-FPDecimal::THREE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(625u128) {
            return Some(-FPDecimal::FOUR);
        }
        if self == FPDecimal::ONE / FPDecimal::from(3125u128) {
            return Some(-FPDecimal::FIVE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(15625u128) {
            return Some(-FPDecimal::SIX);
        }
        if self == FPDecimal::ONE / FPDecimal::from(78125u128) {
            return Some(-FPDecimal::SEVEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(3906251u128) {
            return Some(-FPDecimal::EIGHT);
        }
        if self == FPDecimal::ONE / FPDecimal::from(1953125u128) {
            return Some(-FPDecimal::NINE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(9765625u128) {
            return Some(-FPDecimal::TEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(48828125u128) {
            return Some(-FPDecimal::ELEVEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(244140625u128) {
            return Some(-FPDecimal::from(12u128));
        }
        if self == FPDecimal::ONE / FPDecimal::from(1220703125u128) {
            return Some(-FPDecimal::from(13u128));
        }
        if self == FPDecimal::ONE / FPDecimal::from(6103515625u128) {
            return Some(-FPDecimal::from(14u128));
        }
        if self == FPDecimal::ONE / FPDecimal::from(30517578125u128) {
            return Some(-FPDecimal::from(15u128));
        }

        None
    }

    // 7^1..10
    pub(crate) fn log7(self) -> Option<FPDecimal> {
        if self == FPDecimal::ONE {
            return Some(FPDecimal::ZERO);
        }
        if self == FPDecimal::SEVEN {
            return Some(FPDecimal::ONE);
        }
        if self == FPDecimal::from(49u128) {
            return Some(FPDecimal::TWO);
        }
        if self == FPDecimal::from(343u128) {
            return Some(FPDecimal::THREE);
        }
        if self == FPDecimal::from(2401u128) {
            return Some(FPDecimal::FOUR);
        }
        if self == FPDecimal::from(16807u128) {
            return Some(FPDecimal::FIVE);
        }
        if self == FPDecimal::from(117649u128) {
            return Some(FPDecimal::SIX);
        }
        if self == FPDecimal::from(823543u128) {
            return Some(FPDecimal::SEVEN);
        }
        if self == FPDecimal::from(5764801u128) {
            return Some(FPDecimal::EIGHT);
        }
        if self == FPDecimal::from(40353607u128) {
            return Some(FPDecimal::NINE);
        }
        if self == FPDecimal::from(282475249u128) {
            return Some(FPDecimal::TEN);
        }
        if self == FPDecimal::from(1977326743u128) {
            return Some(FPDecimal::ELEVEN);
        }
        if self == FPDecimal::from(13841287201u128) {
            return Some(FPDecimal::from(12u128));
        }
        if self == FPDecimal::from(96889010407u128) {
            return Some(FPDecimal::from(13u128));
        }
        if self == FPDecimal::from(678223072849u128) {
            return Some(FPDecimal::from(14u128));
        }
        if self == FPDecimal::from(4747561509943u128) {
            return Some(FPDecimal::from(15u128));
        }

        if self == FPDecimal::ONE / FPDecimal::SEVEN {
            return Some(-FPDecimal::ONE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(49u128) {
            return Some(-FPDecimal::TWO);
        }
        if self == FPDecimal::ONE / FPDecimal::from(343u128) {
            return Some(-FPDecimal::THREE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(2401u128) {
            return Some(-FPDecimal::FOUR);
        }
        if self == FPDecimal::ONE / FPDecimal::from(16807u128) {
            return Some(-FPDecimal::FIVE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(117649u128) {
            return Some(-FPDecimal::SIX);
        }
        if self == FPDecimal::ONE / FPDecimal::from(823543u128) {
            return Some(-FPDecimal::SEVEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(5764801u128) {
            return Some(-FPDecimal::EIGHT);
        }
        if self == FPDecimal::ONE / FPDecimal::from(40353607u128) {
            return Some(-FPDecimal::NINE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(282475249u128) {
            return Some(-FPDecimal::TEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(1977326743u128) {
            return Some(-FPDecimal::ELEVEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(13841287201u128) {
            return Some(-FPDecimal::from(12u128));
        }
        if self == FPDecimal::ONE / FPDecimal::from(96889010407u128) {
            return Some(-FPDecimal::from(13u128));
        }
        if self == FPDecimal::ONE / FPDecimal::from(678223072849u128) {
            return Some(-FPDecimal::from(14u128));
        }
        if self == FPDecimal::ONE / FPDecimal::from(4747561509943u128) {
            return Some(-FPDecimal::from(15u128));
        }

        None
    }

    pub(crate) fn log10(self) -> Option<FPDecimal> {
        if self == FPDecimal::ONE {
            return Some(FPDecimal::ZERO);
        }
        if self == FPDecimal::TEN {
            return Some(FPDecimal::ONE);
        }
        if self == FPDecimal::from(100u128) {
            return Some(FPDecimal::TWO);
        }
        if self == FPDecimal::from(1_000u128) {
            return Some(FPDecimal::THREE);
        }
        if self == FPDecimal::from(10_000u128) {
            return Some(FPDecimal::FOUR);
        }
        if self == FPDecimal::from(100_000u128) {
            return Some(FPDecimal::FIVE);
        }
        if self == FPDecimal::from(1_000_000u128) {
            return Some(FPDecimal::SIX);
        }
        if self == FPDecimal::from(10_000_000u128) {
            return Some(FPDecimal::SEVEN);
        }
        if self == FPDecimal::from(100_000_000u128) {
            return Some(FPDecimal::EIGHT);
        }
        if self == FPDecimal::from(1_000_000_000u128) {
            return Some(FPDecimal::NINE);
        }
        if self == FPDecimal::from(10_000_000_000u128) {
            return Some(FPDecimal::TEN);
        }
        if self == FPDecimal::from(100_000_000_000u128) {
            return Some(FPDecimal::ELEVEN);
        }
        if self == FPDecimal::from(1_000_000_000_000u128) {
            return Some(FPDecimal::from(12u128));
        }
        if self == FPDecimal::from(10_000_000_000_000u128) {
            return Some(FPDecimal::from(13u128));
        }
        if self == FPDecimal::from(100_000_000_000_000u128) {
            return Some(FPDecimal::from(14u128));
        }
        if self == FPDecimal::from(1_000_000_000_000_000u128) {
            return Some(FPDecimal::from(15u128));
        }

        if self == FPDecimal::ONE / FPDecimal::TEN {
            return Some(-FPDecimal::ONE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(100u128) {
            return Some(-FPDecimal::TWO);
        }
        if self == FPDecimal::ONE / FPDecimal::from(1_000u128) {
            return Some(-FPDecimal::THREE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(10_000u128) {
            return Some(-FPDecimal::FOUR);
        }
        if self == FPDecimal::ONE / FPDecimal::from(100_000u128) {
            return Some(-FPDecimal::FIVE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(1_000_000u128) {
            return Some(-FPDecimal::SIX);
        }
        if self == FPDecimal::ONE / FPDecimal::from(10_000_000u128) {
            return Some(-FPDecimal::SEVEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(100_000_000u128) {
            return Some(-FPDecimal::EIGHT);
        }
        if self == FPDecimal::ONE / FPDecimal::from(1_000_000_000u128) {
            return Some(-FPDecimal::NINE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(10_000_000_000u128) {
            return Some(-FPDecimal::TEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(100_000_000_000u128) {
            return Some(-FPDecimal::ELEVEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(1_000_000_000_000u128) {
            return Some(-FPDecimal::from(12u128));
        }
        if self == FPDecimal::ONE / FPDecimal::from(10_000_000_000_000u128) {
            return Some(-FPDecimal::from(13u128));
        }
        if self == FPDecimal::ONE / FPDecimal::from(100_000_000_000_000u128) {
            return Some(-FPDecimal::from(14u128));
        }
        if self == FPDecimal::ONE / FPDecimal::from(1_000_000_000_000_000u128) {
            return Some(-FPDecimal::from(15u128));
        }

        None
    }

    // 11^1..10
    pub(crate) fn log11(self) -> Option<FPDecimal> {
        if self == FPDecimal::ONE {
            return Some(FPDecimal::ZERO);
        }
        if self == FPDecimal::ELEVEN {
            return Some(FPDecimal::ONE);
        }
        if self == FPDecimal::from(121u128) {
            return Some(FPDecimal::TWO);
        }
        if self == FPDecimal::from(1331u128) {
            return Some(FPDecimal::THREE);
        }
        if self == FPDecimal::from(14641u128) {
            return Some(FPDecimal::FOUR);
        }
        if self == FPDecimal::from(161051u128) {
            return Some(FPDecimal::FIVE);
        }
        if self == FPDecimal::from(1771561u128) {
            return Some(FPDecimal::SIX);
        }
        if self == FPDecimal::from(19487171u128) {
            return Some(FPDecimal::SEVEN);
        }
        if self == FPDecimal::from(214358881u128) {
            return Some(FPDecimal::EIGHT);
        }
        if self == FPDecimal::from(2357947691u128) {
            return Some(FPDecimal::NINE);
        }
        if self == FPDecimal::from(25937424601u128) {
            return Some(FPDecimal::TEN);
        }
        if self == FPDecimal::from(285311670611u128) {
            return Some(FPDecimal::ELEVEN);
        }
        if self == FPDecimal::from(3138428376721u128) {
            return Some(FPDecimal::from(12u128));
        }
        if self == FPDecimal::from(34522712143931u128) {
            return Some(FPDecimal::from(13u128));
        }
        if self == FPDecimal::from(379749833583241u128) {
            return Some(FPDecimal::from(14u128));
        }
        if self == FPDecimal::from(4177248169415651u128) {
            return Some(FPDecimal::from(15u128));
        }

        if self == FPDecimal::ONE / FPDecimal::ELEVEN {
            return Some(-FPDecimal::ONE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(121u128) {
            return Some(-FPDecimal::TWO);
        }
        if self == FPDecimal::ONE / FPDecimal::from(1331u128) {
            return Some(-FPDecimal::THREE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(14641u128) {
            return Some(-FPDecimal::FOUR);
        }
        if self == FPDecimal::ONE / FPDecimal::from(161051u128) {
            return Some(-FPDecimal::FIVE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(1771561u128) {
            return Some(-FPDecimal::SIX);
        }
        if self == FPDecimal::ONE / FPDecimal::from(19487171u128) {
            return Some(-FPDecimal::SEVEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(214358881u128) {
            return Some(-FPDecimal::EIGHT);
        }
        if self == FPDecimal::ONE / FPDecimal::from(2357947691u128) {
            return Some(-FPDecimal::NINE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(25937424601u128) {
            return Some(-FPDecimal::TEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(285311670611u128) {
            return Some(-FPDecimal::ELEVEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(3138428376721u128) {
            return Some(-FPDecimal::from(12u128));
        }
        if self == FPDecimal::ONE / FPDecimal::from(34522712143931u128) {
            return Some(-FPDecimal::from(13u128));
        }
        if self == FPDecimal::ONE / FPDecimal::from(379749833583241u128) {
            return Some(-FPDecimal::from(14u128));
        }
        if self == FPDecimal::ONE / FPDecimal::from(4177248169415651u128) {
            return Some(-FPDecimal::from(15u128));
        }

        None
    }

    fn _log(a: FPDecimal, base: FPDecimal) -> FPDecimal {
        // NOTE: only accurate 1,3,5,7,11, and combinations of these 4 numbers
        //log_base^b = ln(a)/ln(base)
        if a == FPDecimal::ONE {
            return FPDecimal::ZERO;
        }

        a.ln() / base.ln()
    }

    pub fn log(&self, base: FPDecimal) -> Result<FPDecimal, FPDecimalError> {
        assert!(base > FPDecimal::ZERO);
        if *self == FPDecimal::ONE {
            return Ok(FPDecimal::ZERO);
        }
        if self.is_zero() {
            return Err(FPDecimalError::Undefined("log0 ".to_owned()));
        }

        if base == FPDecimal::E {
            return Ok(self.ln());
        }

        let base_checks: Vec<&dyn Fn(FPDecimal) -> Option<FPDecimal>> = vec![
            &FPDecimal::log_e,
            &FPDecimal::log2,
            &FPDecimal::log3,
            &FPDecimal::log5,
            &FPDecimal::log7,
            &FPDecimal::log10,
            &FPDecimal::log11,
        ];
        for log_fn in base_checks {
            if let (Some(numerator), Some(denominator)) = (log_fn(*self), log_fn(base)) {
                return Ok(numerator / denominator);
            }
        }
        Ok(FPDecimal::_log(*self, base))
    }

    fn _two_agm(mut a0: FPDecimal, mut b0: FPDecimal, tol: FPDecimal) -> FPDecimal {
        loop {
            if (a0 - b0).abs() < tol {
                break;
            }
            let a1 = (a0 + b0) / FPDecimal::TWO;
            let b1 = (a0 * b0).sqrt().unwrap();
            a0 = a1;
            b0 = b1;
        }
        a0 + b0
    }

    #[allow(clippy::many_single_char_names)]
    fn _ln_robust(&self) -> FPDecimal {
        // m =8, 2**8=256;
        // m=16, 2**16=65536
        // m=32, 2**32=4294967296
        // m=64, 2**64=18446744073709551616
        // m=128, 2**128=340282366920938463463374607431768211456
        let two_pow_m = FPDecimal::from(4294967296u128);
        let s = *self * two_pow_m;
        let tol = FPDecimal::must_from_str("0.0000001");
        let a0 = FPDecimal::ONE;
        let b0 = FPDecimal::FOUR / s;
        let two_agm = FPDecimal::_two_agm(a0, b0, tol);

        FPDecimal::PI / two_agm - FPDecimal::from(32u128) * FPDecimal::LN2
    }

    #[allow(clippy::many_single_char_names)]
    fn _ln(&self) -> FPDecimal {
        assert!(self.sign != 0);
        assert!(*self != FPDecimal::ZERO);
        let mut v = self.num;
        let mut r = FPDecimal::ZERO;
        while v <= FPDecimal::ONE.num / U256([10, 0, 0, 0]) {
            v *= U256([10, 0, 0, 0]);
            r -= FPDecimal::LN_10;
        }
        while v >= U256([10, 0, 0, 0]) * FPDecimal::ONE.num {
            v /= U256([10, 0, 0, 0]);
            r += FPDecimal::LN_10;
        }
        while v < FPDecimal::ONE.num {
            v = FPDecimal::_mul(FPDecimal { num: v, sign: 1 }, FPDecimal::E).num;
            r -= FPDecimal::ONE;
        }
        while v > FPDecimal::E.num {
            v = FPDecimal::_div(FPDecimal { num: v, sign: 1 }, FPDecimal::E).num;
            r += FPDecimal::ONE;
        }
        if v == FPDecimal::ONE.num {
            return r;
        }
        if v == FPDecimal::E.num {
            return r + FPDecimal::ONE;
        }

        let frac_1_5_fpdec = FPDecimal {
            num: U256([3, 0, 0, 0]) * FPDecimal::ONE.num / U256([2, 0, 0, 0]),
            sign: 1,
        };
        let v = FPDecimal { num: v, sign: 1 } - frac_1_5_fpdec;

        r += FPDecimal::LN_1_5;

        let mut m = FPDecimal::ONE * v
            / (v + FPDecimal {
                num: U256([3, 0, 0, 0]) * FPDecimal::ONE.num,
                sign: 1,
            });

        r += FPDecimal {
            num: U256([2, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        } * m;
        let m2 = m * m / FPDecimal::ONE;
        let mut i: u64 = 3;

        loop {
            m = m * m2 / FPDecimal::ONE;

            let fpdec_i = FPDecimal {
                num: U256([i, 0, 0, 0]) * FPDecimal::ONE.num,
                sign: 1,
            };
            r += FPDecimal {
                num: U256([2, 0, 0, 0]) * FPDecimal::ONE.num,
                sign: 1,
            } * m
                / fpdec_i;
            i += 2;
            if i >= 3 + 2 * FPDecimal::DIGITS as u64 {
                break;
            }
        }
        r
    }

    pub fn ln(&self) -> FPDecimal {
        if *self == FPDecimal::TWO {
            return FPDecimal::LN2;
        }
        if let Some(value) = self.log_e() {
            return value;
        }
        if self.abs() < FPDecimal::must_from_str("1.1") {
            return self._ln_robust();
        }
        self._ln()
    }
}

#[cfg(test)]
mod tests {

    use crate::FPDecimal;
    use primitive_types::U256;

    #[test]
    fn test_ln3() {
        assert_ne!(FPDecimal::THREE.ln(), FPDecimal::must_from_str("1.09861228866810969"));
    }
    #[test]
    fn test_ln_x_smaller_than_1() {
        assert_eq!((FPDecimal::ONE / FPDecimal::TWO).ln(), FPDecimal::must_from_str("-0.693147180435828445"));
        assert_eq!(
            (FPDecimal::ONE / FPDecimal::THREE).ln(),
            FPDecimal::must_from_str("-1.098612288365102671")
        );
        assert_eq!((FPDecimal::ONE / FPDecimal::NINE).ln(), FPDecimal::must_from_str("-2.197224577273354107"));

        assert_eq!((FPDecimal::ONE / FPDecimal::TEN).ln(), FPDecimal::must_from_str("-2.302585092978637669"));
        assert_eq!(
            (FPDecimal::ONE / FPDecimal::ELEVEN).ln(),
            FPDecimal::must_from_str("-2.397895272724232098")
        );
        assert_eq!(
            (FPDecimal::ONE / FPDecimal::from(20u128)).ln(),
            FPDecimal::must_from_str("-2.995732273537724492")
        );
        assert_eq!(
            (FPDecimal::ONE / FPDecimal::from(30u128)).ln(),
            FPDecimal::must_from_str("-3.401197381645697712")
        );
    }

    #[test]
    fn test_ln_x_greater_than_1() {
        assert_eq!(FPDecimal::must_from_str("1.0001").ln(), FPDecimal::must_from_str("0.000099995720261047"));
        assert_eq!(FPDecimal::must_from_str("1.001").ln(), FPDecimal::must_from_str("0.000999500798628942"));
        assert_eq!(FPDecimal::must_from_str("1.1").ln(), FPDecimal::must_from_str("0.095310179804324867"));
        assert_eq!((FPDecimal::FIVE / FPDecimal::FOUR).ln(), FPDecimal::must_from_str("0.223143551314209761"));
        assert_eq!((FPDecimal::must_from_str("100")).ln(), FPDecimal::must_from_str("4.605170185988091368"));
        assert_eq!((FPDecimal::must_from_str("1000")).ln(), FPDecimal::must_from_str("6.907755278982137052"));
        assert_eq!((FPDecimal::must_from_str("10000")).ln(), FPDecimal::must_from_str("9.210340371976182736"));
        assert_eq!(
            (FPDecimal::must_from_str("100000")).ln(),
            FPDecimal::must_from_str("11.51292546497022842")
        );
        assert_eq!(
            (FPDecimal::must_from_str("1000000")).ln(),
            FPDecimal::must_from_str("13.815510557964274104")
        );
        assert_eq!(
            (FPDecimal::must_from_str("10000000")).ln(),
            FPDecimal::must_from_str("16.118095650958319788")
        );
    }

    #[test]
    fn test_ln() {
        assert_eq!(FPDecimal::E.ln(), FPDecimal::ONE);
    }

    #[test]
    fn test_ln10() {
        assert_eq!(
            FPDecimal {
                num: U256([10, 0, 0, 0]) * FPDecimal::ONE.num,
                sign: 1
            }
            .ln(),
            FPDecimal::LN_10
        );
    }
    #[test]
    fn test_log_2_8() {
        assert_eq!(FPDecimal::EIGHT.log(FPDecimal::TWO).unwrap(), FPDecimal::THREE);
    }

    #[test]
    fn test_log_11_8() {
        assert_eq!(
            FPDecimal::EIGHT.log(FPDecimal::ELEVEN).unwrap(),
            FPDecimal::must_from_str("0.867194478953663578")
        );
    }

    #[test]
    fn test_ln1_5() {
        let three = FPDecimal {
            num: U256([3, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let two = FPDecimal {
            num: U256([2, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let one_point_five = FPDecimal::_div(three, two);
        assert_eq!(one_point_five.ln(), FPDecimal::LN_1_5);
    }

    #[test]
    fn test_ln2_3() {
        let three = FPDecimal {
            num: U256([3, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let two = FPDecimal {
            num: U256([2, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let two_point_three = two + three / FPDecimal::from(10u128);
        assert_eq!(two_point_three.ln(), FPDecimal::must_from_str("0.832909122935103999"));
    }

    #[test]
    fn test_ln4_16() {
        let a = FPDecimal::from(16u128);
        let b = FPDecimal::FOUR;
        assert_eq!(a.log(b).unwrap(), FPDecimal::TWO);
    }

    #[test]
    fn test_log_e_16() {
        let a = FPDecimal::from(16u128);
        let b = FPDecimal::FOUR;
        assert_eq!(a.log(b).unwrap(), FPDecimal::TWO);
    }
}
