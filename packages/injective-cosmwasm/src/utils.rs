use cosmwasm_std::{instantiate2_address, CanonicalAddr, Instantiate2AddressError};

pub const INJECTIVE_ADDRESS_LENGTH: usize = 20;

pub fn instantiate2_address_inj(checksum: &[u8], creator: &CanonicalAddr, salt: &[u8]) -> Result<CanonicalAddr, Instantiate2AddressError> {
    Ok(instantiate2_address(checksum, creator, salt)?[..INJECTIVE_ADDRESS_LENGTH].into())
}
