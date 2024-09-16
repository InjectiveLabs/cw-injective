use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use prost::Message;

pub fn encode_proto_message<T: Message>(msg: T) -> String {
    let mut buf = vec![];
    T::encode(&msg, &mut buf).unwrap();
    BASE64_STANDARD.encode(&buf)
}
