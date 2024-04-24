use cosmwasm_std::{to_json_binary, Binary, to_json_vec, ContractResult, StdError, StdResult, SystemResult, QuerierWrapper};
use injective_cosmwasm::{InjectiveQueryWrapper};
use crate::msg::QueryStargateResponse;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine as _;

pub fn handle_query_stargate(querier: &QuerierWrapper<InjectiveQueryWrapper>, path: String, query_request: String) -> StdResult<Binary> {

    let data = Binary::from_base64(&query_request)?;
    let request = &cosmwasm_std::QueryRequest::<cosmwasm_std::Empty>::Stargate { path, data };

    let raw = to_json_vec(request).map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
    })?;

    let value = match querier.raw_query (&raw) {
        SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
            "Querier system error: {}",
            system_err
        ))),
        SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(format!(
            "Querier contract error: {}",
            contract_err
        ))),
        SystemResult::Ok(ContractResult::Ok(value)) => Ok(value),
    }?;


    to_json_binary(&QueryStargateResponse { value: String::from(decoded_value)? })
}

