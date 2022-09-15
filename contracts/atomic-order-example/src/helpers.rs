use std::borrow::BorrowMut;

use cosmwasm_std::{Addr, BlockInfo, ContractInfo, CosmosMsg, CustomQuery, Env, Querier, QuerierWrapper, Binary, Reply, StdResult, SubMsg, Timestamp, to_binary, TransactionInfo, WasmMsg, WasmQuery};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use injective_cosmwasm::InjectiveMsgWrapper;
use injective_math::FPDecimal;
// use crate::contract::TEST_CONTRACT_ADDR;
use cw_utils::parse_reply_instantiate_data;


use crate::msg::{ExecuteMsg, GetCountResponse, QueryMsg};

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
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

    /// Get Count
    pub fn count<Q, T, CQ>(&self, querier: &Q) -> StdResult<GetCountResponse>
        where
            Q: Querier,
            T: Into<String>,
            CQ: CustomQuery,
    {
        let msg = QueryMsg::GetCount {};
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&msg)?,
        }
            .into();
        let res: GetCountResponse = QuerierWrapper::<CQ>::new(querier).query(&query)?;
        Ok(res)
    }
}


pub fn i32_to_dec(source: i32) -> FPDecimal {
    FPDecimal::from(i128::from(source))
}



pub fn get_message_data(response: &Vec<SubMsg<InjectiveMsgWrapper>>, position: usize) -> &InjectiveMsgWrapper {
    let sth = match &response.get(position).unwrap().msg {
        CosmosMsg::Custom(msg) => msg,
        _ => panic!("No wrapped message found"),
    };
    sth
}
