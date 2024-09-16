use crate::{mocks::MOCK_QUOTE_DECIMALS, utils::human_to_dec};

use injective_test_tube::{
    injective_std::types::{
        cosmos::base::v1beta1::Coin as BaseCoin,
        injective::{insurance::v1beta1::MsgCreateInsuranceFund, oracle::v1beta1::OracleType},
    },
    Account, InjectiveTestApp, Insurance, Module, SigningAccount,
};

pub fn launch_insurance_fund(
    app: &InjectiveTestApp,
    signer: &SigningAccount,
    ticker: &str,
    quote: &str,
    oracle_base: &str,
    oracle_quote: &str,
    oracle_type: OracleType,
) {
    let insurance = Insurance::new(app);

    insurance
        .create_insurance_fund(
            MsgCreateInsuranceFund {
                sender: signer.address(),
                ticker: ticker.to_string(),
                quote_denom: quote.to_string(),
                oracle_base: oracle_base.to_string(),
                oracle_quote: oracle_quote.to_string(),
                oracle_type: oracle_type as i32,
                expiry: -1i64,
                initial_deposit: Some(BaseCoin {
                    amount: human_to_dec("1_000", MOCK_QUOTE_DECIMALS).to_string(),
                    denom: quote.to_string(),
                }),
            },
            signer,
        )
        .unwrap();
}
