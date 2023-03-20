use anyhow::{bail, Result as AnyResult};
use cosmwasm_std::{testing::MockApi, Addr, Api, Binary, BlockInfo, Coin, CustomQuery, Empty, MemoryStorage, Querier, Storage};
use cosmwasm_std::{to_binary, StdError};
use cw_multi_test::{AddressGenerator, App};
use cw_multi_test::{AppResponse, BankKeeper, BasicAppBuilder, CosmosRouter, DistributionKeeper, Module, Router, StakeKeeper, WasmKeeper};
use injective_cosmwasm::{InjectiveMsgWrapper, InjectiveQueryWrapper};

use std::{
    cell::{Ref, RefCell},
    marker::PhantomData,
    ops::Deref,
    rc::Rc,
    u8,
};

use crate::InjectiveAddressGenerator;

fn no_init<BankT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT>(
    _: &mut Router<BankT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT>,
    _: &dyn Api,
    _: &mut dyn Storage,
) {
}

pub type MockedInjectiveApp = App<
    BankKeeper,
    MockApi,
    MemoryStorage,
    CustomInjectiveHandler,
    WasmKeeper<InjectiveMsgWrapper, InjectiveQueryWrapper>,
    StakeKeeper,
    DistributionKeeper,
>;

#[derive(Clone)]
pub struct InitialBalance {
    pub amounts: Vec<Coin>,
    pub address: Addr,
}

pub struct CachingCustomHandlerState<CustomModule, Msg, Query> {
    pub execs: Rc<RefCell<Vec<Msg>>>,
    pub queries: Rc<RefCell<Vec<Query>>>,
    _p: PhantomData<CustomModule>,
}

impl<CustomModule, Msg, Query> CachingCustomHandlerState<CustomModule, Msg, Query>
where
    CustomModule: Module,
    CustomModule::ExecT: Clone + 'static,
    CustomModule::QueryT: CustomQuery + 'static,
{
    pub fn execs(&self) -> impl Deref<Target = [Msg]> + '_ {
        Ref::map(self.execs.borrow(), Vec::as_slice)
    }

    pub fn queries(&self) -> impl Deref<Target = [Query]> + '_ {
        Ref::map(self.queries.borrow(), Vec::as_slice)
    }

    pub fn reset(&self) {
        self.execs.borrow_mut().clear();
        self.queries.borrow_mut().clear();
    }
}

impl Default for CachingCustomHandlerState<CustomInjectiveHandler, InjectiveMsgWrapper, InjectiveQueryWrapper> {
    fn default() -> Self {
        Self {
            execs: Rc::new(RefCell::new(vec![])),
            queries: Rc::new(RefCell::new(vec![])),
            _p: PhantomData,
        }
    }
}

pub type ExecuteResponse = Result<Option<Binary>, anyhow::Error>;
pub type QueryResponse = Result<Binary, anyhow::Error>;

pub struct ExecuteResponseContainer {
    response: Option<ExecuteResponse>,
}

impl ExecuteResponseContainer {
    pub fn with_ok_response<T: serde::ser::Serialize + Sized>(payload: &T) -> Self {
        ExecuteResponseContainer {
            response: Some(Ok(Some(to_binary(payload).unwrap()))),
        }
    }

    pub fn with_error(error: anyhow::Error) -> Self {
        ExecuteResponseContainer { response: Some(Err(error)) }
    }

    pub fn empty() -> Self {
        ExecuteResponseContainer { response: None }
    }

    pub fn is_empty(&self) -> bool {
        self.response.is_none()
    }
}

pub struct QueryResponseContainer {
    response: Option<QueryResponse>,
}

impl QueryResponseContainer {
    pub fn with_ok_response<T: serde::ser::Serialize + Sized>(payload: &T) -> Self {
        QueryResponseContainer {
            response: Some(Ok(to_binary(payload).unwrap())),
        }
    }

    pub fn with_error(error: anyhow::Error) -> Self {
        QueryResponseContainer { response: Some(Err(error)) }
    }

    pub fn empty() -> Self {
        QueryResponseContainer { response: None }
    }

    pub fn is_empty(&self) -> bool {
        self.response.is_none()
    }
}

pub type ExecuteAssertion<Msg> = fn(message: &Msg);
pub type QueryAssertion<Query> = fn(query: &Query);

pub struct ExecuteAssertionContainer<Msg> {
    pub assertion: Option<ExecuteAssertion<Msg>>,
}

impl<Msg> ExecuteAssertionContainer<Msg> {
    pub fn new(assertion: ExecuteAssertion<Msg>) -> Self {
        ExecuteAssertionContainer { assertion: Some(assertion) }
    }

    pub fn empty() -> Self {
        ExecuteAssertionContainer { assertion: None }
    }

    pub fn is_empty(&self) -> bool {
        self.assertion.is_none()
    }
}

pub struct QueryAssertionContainer<Query> {
    pub assertion: Option<QueryAssertion<Query>>,
}

impl<Query> QueryAssertionContainer<Query> {
    pub fn new(assertion: QueryAssertion<Query>) -> Self {
        QueryAssertionContainer { assertion: Some(assertion) }
    }

    pub fn empty() -> Self {
        QueryAssertionContainer { assertion: None }
    }

    pub fn is_empty(&self) -> bool {
        self.assertion.is_none()
    }
}

pub struct CustomInjectiveHandlerAssertions<Msg, Query> {
    pub executes: Vec<ExecuteAssertionContainer<Msg>>,
    pub queries: Vec<QueryAssertionContainer<Query>>,
}

impl<InjectiveMsgWrapper, InjectiveQueryWrapper> Default for CustomInjectiveHandlerAssertions<InjectiveMsgWrapper, InjectiveQueryWrapper> {
    fn default() -> Self {
        Self {
            executes: vec![],
            queries: vec![],
        }
    }
}

#[derive(Default)]
pub struct CustomInjectiveHandlerResponses {
    pub executes: Vec<ExecuteResponseContainer>,
    pub queries: Vec<QueryResponseContainer>,
}

#[derive(Default)]
pub struct CustomInjectiveHandler {
    pub state: CachingCustomHandlerState<CustomInjectiveHandler, InjectiveMsgWrapper, InjectiveQueryWrapper>,
    pub responses: CustomInjectiveHandlerResponses,
    pub assertions: CustomInjectiveHandlerAssertions<InjectiveMsgWrapper, InjectiveQueryWrapper>,
    pub enable_debug: bool,
}

impl Module for CustomInjectiveHandler {
    type ExecT = InjectiveMsgWrapper;
    type QueryT = InjectiveQueryWrapper;
    type SudoT = Empty;

    fn execute<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _sender: Addr,
        msg: Self::ExecT,
    ) -> AnyResult<AppResponse> {
        let mut exec_calls_count = self.state.execs.borrow().len();

        if !self.assertions.executes.is_empty()
            && exec_calls_count < self.assertions.executes.len()
            && !self.assertions.executes[exec_calls_count].is_empty()
        {
            self.assertions.executes[exec_calls_count].assertion.unwrap()(&msg);
        }

        self.state.execs.borrow_mut().push(msg.clone());
        exec_calls_count += 1;

        if self.enable_debug {
            println!("[{exec_calls_count}] Execute message: {msg:?}");
        }

        if self.responses.executes.is_empty()
            || exec_calls_count > self.responses.executes.len()
            || self.responses.executes[exec_calls_count - 1].is_empty()
        {
            return Ok(AppResponse::default());
        }

        let stored_result = self.responses.executes.get(exec_calls_count - 1).unwrap().response.as_ref().unwrap();

        // In order to implement the trait that method has to receive &self and neither Result nor Binary implements Copy
        // and that's the reason why I'm manually copying the underlying [u8] in order to return owned data
        match &stored_result {
            Ok(optional_data) => match &optional_data {
                Some(binary) => Ok(AppResponse {
                    events: vec![],
                    data: Some(copy_binary(binary)),
                }),
                &None => Ok(AppResponse::default()),
            },
            Err(e) => Err(anyhow::Error::new(StdError::generic_err(e.to_string()))),
        }
    }

    fn query(&self, _api: &dyn Api, _storage: &dyn Storage, _querier: &dyn Querier, _block: &BlockInfo, request: Self::QueryT) -> AnyResult<Binary> {
        let mut query_calls_count = self.state.queries.borrow().len();

        if !self.assertions.queries.is_empty()
            && query_calls_count < self.assertions.queries.len()
            && !self.assertions.queries[query_calls_count].is_empty()
        {
            self.assertions.queries[query_calls_count].assertion.unwrap()(&request);
        }

        self.state.queries.borrow_mut().push(request.clone());
        query_calls_count += 1;

        if self.enable_debug {
            println!("[{query_calls_count}] Query request: {request:?}");
        }

        if self.responses.queries.is_empty()
            || query_calls_count > self.responses.queries.len()
            || self.responses.queries[query_calls_count - 1].is_empty()
        {
            Ok(Binary::default())
        } else {
            let stored_result = self.responses.queries.get(query_calls_count - 1).unwrap().response.as_ref().unwrap();

            // In order to implement the trait that method has to receive &self and neither Result nor Binary implements Copy
            // and that's the reason why I'm manually copying the underlying [u8] in order to return owned data
            match &stored_result {
                Ok(optional_data) => Ok(copy_binary(optional_data)),
                Err(e) => Err(anyhow::Error::new(StdError::generic_err(e.to_string()))),
            }
        }
    }

    fn sudo<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        msg: Self::SudoT,
    ) -> AnyResult<AppResponse> {
        bail!("Unexpected sudo msg {:?}", msg)
    }
}

pub fn mock_custom_injective_chain_app(
    initial_balances: Vec<InitialBalance>,
    execute_responses: Vec<ExecuteResponseContainer>,
    query_responses: Vec<QueryResponseContainer>,
    execute_assertions: Vec<ExecuteAssertionContainer<InjectiveMsgWrapper>>,
    query_assertions: Vec<QueryAssertionContainer<InjectiveQueryWrapper>>,
    address_generator: Option<impl AddressGenerator + 'static>,
    enable_debug: bool,
) -> MockedInjectiveApp {
    let inj_handler = CustomInjectiveHandler {
        responses: CustomInjectiveHandlerResponses {
            executes: execute_responses,
            queries: query_responses,
        },
        assertions: CustomInjectiveHandlerAssertions {
            executes: execute_assertions,
            queries: query_assertions,
        },
        enable_debug,
        ..Default::default()
    };

    let inj_wasm_keeper = match address_generator {
        Some(generator) => WasmKeeper::<InjectiveMsgWrapper, InjectiveQueryWrapper>::new_with_custom_address_generator(generator),
        None => WasmKeeper::<InjectiveMsgWrapper, InjectiveQueryWrapper>::new_with_custom_address_generator(InjectiveAddressGenerator()),
    };

    BasicAppBuilder::new()
        .with_custom(inj_handler)
        .with_wasm::<CustomInjectiveHandler, WasmKeeper<InjectiveMsgWrapper, InjectiveQueryWrapper>>(inj_wasm_keeper)
        .build(|router, _, storage| {
            initial_balances.into_iter().for_each(|balance| {
                router
                    .bank
                    .init_balance(storage, &balance.address, balance.amounts)
                    .expect("balances added")
            })
        })
}

pub fn mock_default_injective_chain_app() -> MockedInjectiveApp {
    let inj_wasm_keeper = WasmKeeper::<InjectiveMsgWrapper, InjectiveQueryWrapper>::new_with_custom_address_generator(InjectiveAddressGenerator());

    let inj_handler = CustomInjectiveHandler::default();

    BasicAppBuilder::new()
        .with_custom(inj_handler)
        .with_wasm::<CustomInjectiveHandler, WasmKeeper<InjectiveMsgWrapper, InjectiveQueryWrapper>>(inj_wasm_keeper)
        .build(no_init)
}

fn copy_binary(binary: &Binary) -> Binary {
    let mut c: Vec<u8> = vec![0; binary.0.len()];
    c.clone_from_slice(&binary.0);
    Binary(c)
}
