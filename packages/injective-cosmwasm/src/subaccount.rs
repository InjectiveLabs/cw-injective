use cosmwasm_std::Addr;
use subtle_encoding::bech32;

use ethereum_types::H160;

pub fn default_subaccount_id(addr: &Addr) -> String {
    address_to_subaccount_id(addr, 0)
}

// TODO: consider converting nonce to proper hex value, so e.g. 15 -> f
pub fn address_to_subaccount_id(addr: &Addr, nonce: u32) -> String {
    let address_str = bech32_to_hex(addr);
    let nonce_str = left_pad_with_zeroes(nonce, 24);

    format!("{}{}", address_str, nonce_str)
}

fn left_pad_with_zeroes(input: u32, length: usize) -> String {
    let mut padded_input = input.to_string();
    while padded_input.len() < length {
        padded_input = "0".to_string() + &padded_input;
    }
    padded_input
}

pub fn bech32_to_hex(addr: &Addr) -> String {
    let decoded_bytes = bech32::decode(addr.as_str()).unwrap().1;
    let decoded_h160 = H160::from_slice(&decoded_bytes);
    let decoded_string = format!("{:?}", decoded_h160);
    decoded_string
}

#[cfg(test)]
mod tests {
    use crate::subaccount::{address_to_subaccount_id, bech32_to_hex, default_subaccount_id};
    use cosmwasm_std::Addr;

    #[test]
    fn bech32_to_hex_test() {
        let decoded_string = bech32_to_hex(&Addr::unchecked("inj1khsfhyavadcvzug67pufytaz2cq36ljkrsr0nv"));
        println!("{}", decoded_string);
        assert_eq!(decoded_string, "0xB5e09b93aCEb70C1711aF078922fA256011D7e56".to_lowercase());
    }

    #[test]
    fn address_to_subaccount_id_test() {
        let subaccount_id = address_to_subaccount_id(&Addr::unchecked("inj1khsfhyavadcvzug67pufytaz2cq36ljkrsr0nv"), 69);
        println!("{}", subaccount_id);
        assert_eq!(subaccount_id, "0xb5e09b93aceb70c1711af078922fa256011d7e56000000000000000000000069");

        println!("{}", subaccount_id);
        assert_eq!(
            default_subaccount_id(&Addr::unchecked("inj1khsfhyavadcvzug67pufytaz2cq36ljkrsr0nv")),
            "0xb5e09b93aceb70c1711af078922fa256011d7e56000000000000000000000000"
        );
    }
}
