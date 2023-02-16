use cosmwasm_std::{Addr, Storage};
use cw_multi_test::AddressGenerator;
use injective_cosmwasm::addr_to_bech32;
use rand::OsRng;
use secp256k1::Secp256k1;
use std::fmt::Write;
use std::u8;

const ADDRESS_LENGTH: usize = 40;
const ADDRESS_BYTES: usize = ADDRESS_LENGTH / 2;
const KECCAK_OUTPUT_BYTES: usize = 32;
const ADDRESS_BYTE_INDEX: usize = KECCAK_OUTPUT_BYTES - ADDRESS_BYTES;

#[derive(Default)]
pub struct InjectiveAddressGenerator();

impl AddressGenerator for InjectiveAddressGenerator {
    fn next_address(&self, _: &mut dyn Storage) -> Addr {
        generate_inj_address()
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
    fn next_address(&self, storage: &mut dyn Storage) -> Addr {
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

        generated_address
    }
}

pub fn generate_inj_address() -> Addr {
    let secp256k1 = Secp256k1::new();
    let mut rng = OsRng::new().expect("failed to create new random number generator");
    let (_, public_key) = secp256k1.generate_keypair(&mut rng).expect("failed to generate key pair");

    let public_key_array = &public_key.serialize_vec(&secp256k1, false)[1..];
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
