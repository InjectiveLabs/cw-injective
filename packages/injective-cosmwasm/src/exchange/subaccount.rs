use std::str::FromStr;

use cosmwasm_std::{Addr, Deps, StdResult};
use subtle_encoding::bech32;

use ethereum_types::H160;

use crate::{exchange::types::SubaccountId, InjectiveQueryWrapper};

pub fn get_default_subaccount_id_for_checked_address(addr: &Addr) -> SubaccountId {
    checked_address_to_subaccount_id(addr, 0)
}

pub fn checked_address_to_subaccount_id(addr: &Addr, nonce: u32) -> SubaccountId {
    let address_str = bech32_to_hex(addr);
    let hex_nonce = format!("{nonce:08x}");
    let nonce_str = left_pad_with_zeroes(hex_nonce, 24);

    SubaccountId::new(format!("{address_str}{nonce_str}")).expect("failed to create subaccount_id")
}

pub fn is_default_subaccount(subaccount_id: &SubaccountId) -> bool {
    let subaccount_id_str = subaccount_id.as_str();
    let hex_nonce = &subaccount_id_str[subaccount_id_str.len() - 24..subaccount_id_str.len()];

    hex_nonce == "000000000000000000000000"
}

fn left_pad_with_zeroes(mut input: String, length: usize) -> String {
    while input.len() < length {
        input = "0".to_string() + &input;
    }
    input
}

pub fn bech32_to_hex(addr: &Addr) -> String {
    let decoded_bytes = bech32::decode(addr.as_str()).unwrap().1;
    let decoded_h160 = H160::from_slice(&decoded_bytes);
    let decoded_string = format!("{decoded_h160:?}");
    decoded_string
}

pub fn addr_to_bech32(addr: String) -> String {
    let encoded_bytes = H160::from_str(&addr[2..addr.len()]).unwrap();
    bech32::encode("inj", encoded_bytes)
}

pub fn subaccount_id_to_ethereum_address(subaccount_id: &SubaccountId) -> String {
    let subaccount_id_str = subaccount_id.as_str();
    subaccount_id_str[0..subaccount_id_str.len() - 24].to_string()
}

pub fn subaccount_id_to_injective_address(subaccount_id: &SubaccountId, deps: &Deps<InjectiveQueryWrapper>) -> StdResult<Addr> {
    let ethereum_address = subaccount_id_to_ethereum_address(subaccount_id);
    deps.api.addr_validate(addr_to_bech32(ethereum_address).as_str())
}

pub fn subaccount_id_to_unchecked_injective_address(subaccount_id: &SubaccountId) -> String {
    let ethereum_address = subaccount_id_to_ethereum_address(subaccount_id);
    addr_to_bech32(ethereum_address)
}

#[cfg(test)]
mod tests {
    use crate::{
        exchange::subaccount::{
            bech32_to_hex, checked_address_to_subaccount_id, get_default_subaccount_id_for_checked_address, subaccount_id_to_injective_address,
            SubaccountId,
        },
        mock_dependencies,
    };
    use cosmwasm_std::Addr;

    #[test]
    fn bech32_to_hex_test() {
        let decoded_string = bech32_to_hex(&Addr::unchecked("inj1khsfhyavadcvzug67pufytaz2cq36ljkrsr0nv"));
        assert_eq!(decoded_string, "0xB5e09b93aCEb70C1711aF078922fA256011D7e56".to_lowercase());
    }

    #[test]
    fn checked_address_to_subaccount_id_test() {
        let subaccount_id = checked_address_to_subaccount_id(&Addr::unchecked("inj1khsfhyavadcvzug67pufytaz2cq36ljkrsr0nv"), 69);
        assert_eq!(
            subaccount_id.as_str(),
            "0xb5e09b93aceb70c1711af078922fa256011d7e56000000000000000000000045"
        );

        assert_eq!(
            get_default_subaccount_id_for_checked_address(&Addr::unchecked("inj1khsfhyavadcvzug67pufytaz2cq36ljkrsr0nv")).as_str(),
            "0xb5e09b93aceb70c1711af078922fa256011d7e56000000000000000000000000"
        );
    }

    #[test]
    fn subaccount_id_to_address_test() {
        let subaccount_id = "0xb5e09b93aceb70c1711af078922fa256011d7e56000000000000000000000000";
        let address = subaccount_id_to_injective_address(
            &SubaccountId::new(subaccount_id.to_string()).expect("failed to create subaccount_id"),
            &mock_dependencies().as_ref(),
        );

        assert_eq!(address.unwrap(), "inj1khsfhyavadcvzug67pufytaz2cq36ljkrsr0nv");
    }
}
