use cosmwasm_std::{Addr, Storage};
use cw_multi_test::AddressGenerator;
use injective_cosmwasm::addr_to_bech32;
use secp256k1::{rand, PublicKey, Secp256k1, SecretKey};
use std::fmt::Write;

const ADDRESS_LENGTH: usize = 40;
const ADDRESS_BYTES: usize = ADDRESS_LENGTH / 2;
const KECCAK_OUTPUT_BYTES: usize = 32;
const ADDRESS_BYTE_INDEX: usize = KECCAK_OUTPUT_BYTES - ADDRESS_BYTES;

#[derive(Default)]
pub struct InjectiveAddressGenerator();

impl AddressGenerator for InjectiveAddressGenerator {
    fn contract_address(
        &self,
        _api: &dyn cosmwasm_std::Api,
        _storage: &mut dyn Storage,
        _code_id: u64,
        _instance_id: u64,
    ) -> Result<Addr, cosmwasm_std::StdError> {
        Ok(generate_inj_address())
    }

    fn predictable_contract_address(
        &self,
        api: &dyn cosmwasm_std::Api,
        _storage: &mut dyn Storage,
        _code_id: u64,
        _instance_id: u64,
        checksum: &[u8],
        creator: &cosmwasm_std::CanonicalAddr,
        salt: &[u8],
    ) -> Result<Addr, cosmwasm_std::StdError> {
        let canonical_addr = cosmwasm_std::instantiate2_address(checksum, creator, salt)?;
        api.addr_humanize(&canonical_addr)
    }
}

pub struct StorageAwareInjectiveAddressGenerator {
    key: String,
}

impl Default for StorageAwareInjectiveAddressGenerator {
    fn default() -> Self {
        Self {
            key: "generated_addresses".to_string(),
        }
    }
}

impl AddressGenerator for StorageAwareInjectiveAddressGenerator {
    fn contract_address(
        &self,
        _api: &dyn cosmwasm_std::Api,
        storage: &mut dyn Storage,
        _code_id: u64,
        _instance_id: u64,
    ) -> Result<Addr, cosmwasm_std::StdError> {
        let generated_address = generate_inj_address();
        let key = self.key.as_bytes();
        let stored = storage.get(key);

        match stored {
            Some(value) => {
                let as_string = String::from_utf8_lossy(&value);
                let mut split = as_string.split(',').collect::<Vec<&str>>();
                split.push(generated_address.as_str());
                let joined_as_string = split.join(",");
                storage.set(key, joined_as_string.as_bytes())
            }
            None => {
                let value = generated_address.as_str().as_bytes();
                storage.set(key, value);
            }
        }

        Ok(generated_address)
    }
}

pub fn generate_inj_address() -> Addr {
    let secp256k1 = Secp256k1::new();

    let secret_key = SecretKey::new(&mut rand::thread_rng());

    let public_key = PublicKey::from_secret_key(&secp256k1, &secret_key);

    let public_key_array = &public_key.serialize()[1..];

    let keccak = tiny_keccak::keccak256(public_key_array);

    let address_short = to_hex_string(&keccak[ADDRESS_BYTE_INDEX..], 40); // get rid of the constant 0x04 byte
    let full_address = format!("0x{address_short}");

    let inj_address = addr_to_bech32(full_address);

    Addr::unchecked(inj_address)
}

fn to_hex_string(slice: &[u8], expected_string_size: usize) -> String {
    let mut result = String::with_capacity(expected_string_size);

    for &byte in slice {
        write!(&mut result, "{byte:02x}").expect("Unable to format the public key.");
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_generate_inj_address() {
        // Generate an Injective address
        let generated_address = generate_inj_address();

        // Ensure the generated address is not empty
        assert!(!generated_address.to_string().is_empty(), "Generated address should not be empty");

        // Ensure the generated address starts with the Injective prefix (e.g., "inj")
        assert!(generated_address.as_str().starts_with("inj"), "Generated address should start with 'inj'");

        println!("generated address: {:?}", generated_address);

        // Ensure the address matches a valid bech32 format
        let bech32_regex = Regex::new(r"^inj[0-9a-z]{39}$").unwrap();
        assert!(
            bech32_regex.is_match(generated_address.as_str()),
            "Generated address does not match valid bech32 format"
        );

        // Ensure each generated address is unique (you can extend this for more iterations)
        let another_generated_address = generate_inj_address();
        assert_ne!(generated_address, another_generated_address, "Generated addresses should be unique");
    }
}
