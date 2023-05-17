use cosmwasm_std::{Addr, CosmosMsg, StdError, StdResult, SubMsg, to_binary, WasmMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use injective_cosmwasm::{InjectiveMsgWrapper, SpotMarket};
use injective_math::FPDecimal;

use crate::msg::ExecuteMsg;

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }
}

pub fn i32_to_dec(source: i32) -> FPDecimal {
    FPDecimal::from(i128::from(source))
}

pub fn get_message_data(
    response: &[SubMsg<InjectiveMsgWrapper>],
    position: usize,
) -> &InjectiveMsgWrapper {
    let sth = match &response.get(position).unwrap().msg {
        CosmosMsg::Custom(msg) => msg,
        _ => panic!("No wrapped message found"),
    };
    sth
}

pub fn counter_denom<'a>(market:&'a SpotMarket, denom: &str) -> StdResult<&'a str> {
    if market.quote_denom == denom {
        Ok(&market.base_denom)
    } else if market.base_denom == denom {
        Ok(&market.quote_denom)
    } else {
        Err(StdError::generic_err("Denom must be either base or quote denom of this market!"))
    }
}

pub fn dec_scale_factor() -> FPDecimal {
    FPDecimal::from(1000000000000000000_i128)
}
