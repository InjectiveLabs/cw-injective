use crate::msg::{ExecuteMsg, QueryMsg};
use crate::utils::{ExchangeType, Setup};
use cosmwasm_std::{instantiate2_address, Addr, Binary, CanonicalAddr, HexBinary, StdError};

use injective_test_tube::{Module, Wasm};

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_instantiate2() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);

    let code_id = env.code_id;

    let query_msg = QueryMsg::TestInstantiate2 { code_id };
    let address_from_query: Addr = wasm.query(&env.contract_address, &query_msg).unwrap();

    println!("address_from_query: {}", address_from_query);

    let res = wasm.execute(
        &env.contract_address,
        &ExecuteMsg::TestInstantiate2 { code_id },
        &[],
        &env.users[0].account,
    );
    assert!(res.is_ok(), "Execution failed with error: {:?}", res.unwrap_err());

    println!(
        "address_from_msg {}",
        res.unwrap()
            .events
            .iter()
            .find(|e| e.attributes.iter().any(|a| a.key == "contract_address"))
            .unwrap()
            .attributes
            .iter()
            .find(|a| a.key == "contract_address")
            .unwrap()
            .value
    );

    let canonical_addr = get_instantiate_address_from_outside();
    println!("canonical_addr: {}", canonical_addr.0);
}

// inj16qyc46tal0343nul269ugnf03e5d7z48mp3vznxcuyzh6tkus0msm6dkzm
// inj16qyc46tal0343nul269ugnf03e5d7z48jnd2cq

pub fn get_instantiate_address_from_outside() -> CanonicalAddr {
    let canonical_creator = CanonicalAddr(Binary::from_base64("reSl9YA6Q5g1xjY5Wo1kje5Xsvw=").unwrap());
    let checksum = HexBinary::from_hex("9c2b3da4b434cbf991d9d9c3955d6c3e1afdc375ebd3b0cf3d634cabed54d2a1").unwrap();
    let salt = b"123";

    println!("---------------------------------");
    println!("---------------------------------");
    println!("AAAAAAAA");
    println!("---------------------------------");
    println!("---------------------------------");

    instantiate2_address(&checksum, &canonical_creator, salt)
        .map_err(|_| StdError::generic_err("Could not calculate addr"))
        .unwrap()
}

// CanonicalAddr(Binary(517d8c45b121943bc383edfa91a644856f6459cc0dc6c37270eb334d0e4a7cc6))

// actually created in contract: 0AmK6X3741jPn1aLxE0vjmjfCqc=
// from calculation: CrH8f+WJbpxjYGy7Tx8NCMSInI1wiL0Uv7du0EBG4kA=
