use cosmwasm_std::Uint128;
use injective_test_tube::{
    injective_std::types::cosmos::{
        bank::v1beta1::{MsgSend, QueryBalanceRequest},
        base::v1beta1::Coin,
    },
    Account, Bank, InjectiveTestApp, SigningAccount,
};
use std::str::FromStr;

pub fn send(bank: &Bank<InjectiveTestApp>, amount: &str, denom: &str, from: &SigningAccount, to: &SigningAccount) {
    bank.send(
        MsgSend {
            from_address: from.address(),
            to_address: to.address(),
            amount: vec![Coin {
                amount: amount.to_string(),
                denom: denom.to_string(),
            }],
        },
        from,
    )
    .unwrap();
}

pub fn query_balance(bank: &Bank<InjectiveTestApp>, address: String, denom: String) -> Uint128 {
    let response = bank.query_balance(&QueryBalanceRequest { address, denom }).unwrap();

    match response.balance {
        Some(balance) => Uint128::from_str(&balance.amount).unwrap(),
        None => Uint128::zero(),
    }
}
