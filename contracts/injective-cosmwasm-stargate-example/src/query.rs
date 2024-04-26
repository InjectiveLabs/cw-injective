use crate::msg::QueryStargateResponse;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine as _;
use cosmwasm_std::{to_json_binary, to_json_vec, Binary, ContractResult, QuerierWrapper, QueryRequest, StdError, StdResult, SystemResult};
use injective_cosmwasm::InjectiveQueryWrapper;

pub fn handle_query_stargate(querier: &QuerierWrapper<InjectiveQueryWrapper>, path: String, query_request: String) -> StdResult<Binary> {
    let data = Binary::from_base64(&query_request)?;
    let request = &QueryRequest::<InjectiveQueryWrapper>::Stargate { path, data };
    let raw = to_json_vec(request).map_err(|serialize_err| StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err)))?;

    let value = match querier.raw_query(&raw) {
        SystemResult::Err(system_err) => Err(StdError::generic_err(format!("Querier system error: {}", system_err))),
        SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(format!("Querier contract error: {}", contract_err))),
        SystemResult::Ok(ContractResult::Ok(value)) => Ok(value),
    }?
    .to_string();

    let decoded_value = BASE64_STANDARD
        .decode(value)
        .map_err(|_| StdError::generic_err("Decoding base64 value"))?;
    to_json_binary(&QueryStargateResponse {
        value: String::from_utf8(decoded_value)?,
    })
}
