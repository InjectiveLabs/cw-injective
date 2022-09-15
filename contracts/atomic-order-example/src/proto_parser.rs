use cosmwasm_std::Binary;
use cw_utils::ParseReplyError;
//
// /// Base128 varint decoding.
// /// The remaining of the data is kept in the data parameter.
// pub fn parse_protobuf_varint(data: &mut Vec<u8>, field_number: u8) -> Result<usize, ParseReplyError> {
//     let data_len = data.len();
//     let mut len: u64 = 0;
//     let mut i = 0;
//     while i < VARINT_MAX_BYTES {
//         if data_len == i {
//             return Err(ParseReplyError::ParseFailure(format!(
//                 "failed to decode Protobuf message: field #{}: varint data too short",
//                 field_number
//             )));
//         }
//         len += ((data[i] & 0x7f) as u64) << (i * 7);
//         if data[i] & 0x80 == 0 {
//             break;
//         }
//         i += 1;
//     }
//     if i == VARINT_MAX_BYTES {
//         return Err(ParseReplyError::ParseFailure(format!(
//             "failed to decode Protobuf message: field #{}: varint data too long",
//             field_number
//         )));
//     }
//     *data = data[i + 1..].to_owned();
//
//     Ok(len as usize) // Gently fall back to the arch's max addressable size
// }
//
// /// Helper function to parse length-prefixed protobuf fields.
// /// The remaining of the data is kept in the data parameter.
// fn parse_protobuf_length_prefixed(
//     data: &mut Vec<u8>,
//     field_number: u8,
// ) -> Result<Vec<u8>, ParseReplyError> {
//     if data.is_empty() {
//         return Ok(vec![]);
//     };
//     let mut rest_1 = data.split_off(1);
//     let wire_type = data[0] & 0b11;
//     let field = data[0] >> 3;
//
//     if field != field_number {
//         return Err(ParseReplyError::ParseFailure(format!(
//             "failed to decode Protobuf message: invalid field #{} for field #{}",
//             field, field_number
//         )));
//     }
//     if wire_type != WIRE_TYPE_LENGTH_DELIMITED {
//         return Err(ParseReplyError::ParseFailure(format!(
//             "failed to decode Protobuf message: field #{}: invalid wire type {}",
//             field_number, wire_type
//         )));
//     }
//
//     let len = parse_protobuf_varint(&mut rest_1, field_number)?;
//     if rest_1.len() < len {
//         return Err(ParseReplyError::ParseFailure(format!(
//             "failed to decode Protobuf message: field #{}: message too short",
//             field_number
//         )));
//     }
//     *data = rest_1.split_off(len);
//
//     Ok(rest_1)
// }
//
// pub fn parse_protobuf_string(data: &mut Vec<u8>, field_number: u8) -> Result<String, ParseReplyError> {
//     let str_field = parse_protobuf_length_prefixed(data, field_number)?;
//     Ok(String::from_utf8(str_field)?)
// }
//
// pub fn parse_protobuf_bytes(
//     data: &mut Vec<u8>,
//     field_number: u8,
// ) -> Result<Option<Binary>, ParseReplyError> {
//     let bytes_field = parse_protobuf_length_prefixed(data, field_number)?;
//     if bytes_field.is_empty() {
//         Ok(None)
//     } else {
//         Ok(Some(Binary(bytes_field)))
//     }
// }
