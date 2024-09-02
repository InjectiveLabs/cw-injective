use injective_test_tube::{
    injective_std::{
        shim::Any,
        types::{
            cosmos::{
                base::v1beta1::Coin as BaseCoin,
                gov::{v1::MsgVote, v1beta1::MsgSubmitProposal as MsgSubmitProposalV1Beta1},
            },
            injective::oracle::v1beta1::{
                GrantPriceFeederPrivilegeProposal, MsgRelayPriceFeedPrice, QueryOraclePriceRequest, QueryOraclePriceResponse,
            },
        },
    },
    Account, Gov, InjectiveTestApp, Module, Oracle, SigningAccount,
};
use prost::Message;
use std::str::FromStr;

pub fn launch_price_feed_oracle(
    app: &InjectiveTestApp,
    signer: &SigningAccount,
    validator: &SigningAccount,
    base: &str,
    quote: &str,
    dec_price: String,
) {
    let gov = Gov::new(app);
    let oracle = Oracle::new(app);

    let mut buf = vec![];
    GrantPriceFeederPrivilegeProposal::encode(
        &GrantPriceFeederPrivilegeProposal {
            title: "test-proposal".to_string(),
            description: "test-proposal".to_string(),
            base: base.to_string(),
            quote: quote.to_string(),
            relayers: vec![signer.address()],
        },
        &mut buf,
    )
    .unwrap();

    let res = gov
        .submit_proposal_v1beta1(
            MsgSubmitProposalV1Beta1 {
                content: Some(Any {
                    type_url: "/injective.oracle.v1beta1.GrantPriceFeederPrivilegeProposal".to_string(),
                    value: buf,
                }),
                initial_deposit: vec![BaseCoin {
                    amount: "100000000000000000000".to_string(),
                    denom: "inj".to_string(),
                }],
                proposer: validator.address(),
            },
            validator,
        )
        .unwrap();

    let proposal_id = res.events.iter().find(|e| e.ty == "submit_proposal").unwrap().attributes[0]
        .value
        .to_owned();

    gov.vote(
        MsgVote {
            proposal_id: u64::from_str(&proposal_id).unwrap(),
            voter: validator.address(),
            option: 1i32,
            metadata: "".to_string(),
        },
        validator,
    )
    .unwrap();

    // NOTE: increase the block time in order to move past the voting period
    app.increase_time(10u64);

    oracle
        .relay_price_feed(
            MsgRelayPriceFeedPrice {
                sender: signer.address(),
                base: vec![base.to_string()],
                quote: vec![quote.to_string()],
                price: vec![dec_price], // 1.2@18dp
            },
            signer,
        )
        .unwrap();
}

pub fn relay_price_feed_price(oracle: &Oracle<InjectiveTestApp>, relayer: &SigningAccount, base_denom: &str, quote_denom: &str, price: &str) {
    oracle
        .relay_price_feed(
            MsgRelayPriceFeedPrice {
                sender: relayer.address(),
                base: vec![base_denom.to_string()],
                quote: vec![quote_denom.to_string()],
                price: vec![price.to_string()],
            },
            relayer,
        )
        .unwrap();
}

pub fn query_oracle_mark_price(app: &InjectiveTestApp, base_denom: &str, quote_denom: &str) -> QueryOraclePriceResponse {
    let oracle = Oracle::new(app);

    oracle
        .query_oracle_price(&QueryOraclePriceRequest {
            oracle_type: 2,
            base: base_denom.to_string(),
            quote: quote_denom.to_string(),
            scaling_options: None,
        })
        .unwrap()
}
