use injective_test_tube::{
    injective_std::{
        shim::{Any, Timestamp},
        types::cosmos::{
            authz::v1beta1::{GenericAuthorization, Grant, MsgGrant, MsgRevoke, MsgRevokeResponse},
            bank::v1beta1::SendAuthorization,
            base::v1beta1::Coin as BaseCoin,
        },
    },
    Account, Authz, ExecuteResponse, InjectiveTestApp, Module, Runner, SigningAccount,
};
use prost::Message;

pub fn create_generic_authorization(app: &InjectiveTestApp, granter: &SigningAccount, grantee: String, msg: String, expiration: Option<Timestamp>) {
    let authz = Authz::new(app);

    let mut buf = vec![];
    GenericAuthorization::encode(&GenericAuthorization { msg }, &mut buf).unwrap();

    authz
        .grant(
            MsgGrant {
                granter: granter.address(),
                grantee,
                grant: Some(Grant {
                    authorization: Some(Any {
                        type_url: GenericAuthorization::TYPE_URL.to_string(),
                        value: buf.clone(),
                    }),
                    expiration,
                }),
            },
            granter,
        )
        .unwrap();
}

pub fn revoke_authorization(app: &InjectiveTestApp, granter: &SigningAccount, grantee: String, msg_type_url: String) {
    let _res: ExecuteResponse<MsgRevokeResponse> = app
        .execute_multiple(
            &[(
                MsgRevoke {
                    granter: granter.address(),
                    grantee,
                    msg_type_url,
                },
                MsgRevoke::TYPE_URL,
            )],
            granter,
        )
        .unwrap();
}

pub fn create_send_authorization(app: &InjectiveTestApp, granter: &SigningAccount, grantee: String, amount: BaseCoin, expiration: Option<Timestamp>) {
    let authz = Authz::new(app);

    let mut buf = vec![];
    SendAuthorization::encode(
        &SendAuthorization {
            spend_limit: vec![amount],
            allow_list: vec![],
        },
        &mut buf,
    )
    .unwrap();

    authz
        .grant(
            MsgGrant {
                granter: granter.address(),
                grantee,
                grant: Some(Grant {
                    authorization: Some(Any {
                        type_url: SendAuthorization::TYPE_URL.to_string(),
                        value: buf.clone(),
                    }),
                    expiration,
                }),
            },
            granter,
        )
        .unwrap();
}

pub fn execute_generic_authorizations(app: &InjectiveTestApp, granter: &SigningAccount, grantee: String, msgs: Vec<String>) {
    for msg in msgs {
        create_generic_authorization(app, granter, grantee.clone(), msg, None);
    }
}
