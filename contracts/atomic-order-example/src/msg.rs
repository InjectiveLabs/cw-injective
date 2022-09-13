use cosmwasm_std::Reply;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    // Market Id
    // MARKET_ID="0xfbd55f13641acbb6e69d7b59eb335dabe2ecbfea136082ce2eedaba8a0c917a3"
    pub market_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Swap { quantity: String, price: String },
    Reset { count: i32 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetCountResponse {
    pub count: i32,
}
//
// pub fn parse_reply_instantiate_data(
//     msg: Reply,
// ) -> Result<MsgInstantiateContractResponse, ParseReplyError> {
//     let data = msg
//         .result
//         .into_result()
//         .map_err(ParseReplyError::SubMsgFailure)?
//         .data
//         .ok_or_else(|| ParseReplyError::ParseFailure("Missing reply data".to_owned()))?;
//     parse_instantiate_response_data(&data.0)
// }
//
//
// pub fn parse_instantiate_response_data(
//     data: &[u8],
// ) -> Result<MsgInstantiateContractResponse, ParseReplyError> {
//     // Manual protobuf decoding
//     let mut data = data.to_vec();
//     // Parse contract addr
//     let contract_addr = parse_protobuf_string(&mut data, 1)?;
//
//     // Parse (optional) data
//     let data = parse_protobuf_bytes(&mut data, 2)?;
//
//     Ok(MsgInstantiateContractResponse {
//         contract_address: contract_addr,
//         data,
//     })
// }
