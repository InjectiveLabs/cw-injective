use crate::{msg::WrappedPosition, utils::div_dec};
use cosmwasm_std::Decimal256 as Decimal;

pub fn inv_imbalance_deriv(
    position: &Option<WrappedPosition>,
    inv_val: Decimal,
) -> (Decimal, bool) {
    match position {
        None => (Decimal::zero(), true),
        Some(position) => {
            let position_value = position.margin;
            let inv_imbalance = div_dec(position_value, inv_val);
            (inv_imbalance, position.is_long)
        }
    }
}
