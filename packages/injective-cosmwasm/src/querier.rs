use cosmwasm_std::{QuerierWrapper, StdResult};

use injective_math::FPDecimal;

use crate::oracle::{OracleHistoryOptions, OracleInfo};
use crate::query::{
    DerivativeMarketResponse, InjectiveQuery, InjectiveQueryWrapper, MarketMidPriceAndTOBResponse, MarketVolatilityResponse, OraclePriceResponse,
    OracleVolatilityResponse, PerpetualMarketFundingResponse, PerpetualMarketInfoResponse, PythPriceResponse, QueryAggregateVolumeResponse,
    QueryContractRegistrationInfoResponse, QueryDenomDecimalResponse, QueryDenomDecimalsResponse, SpotMarketResponse, SubaccountDepositResponse,
    SubaccountEffectivePositionInMarketResponse, SubaccountPositionInMarketResponse, TokenFactoryCreateDenomFeeResponse,
    TokenFactoryDenomSupplyResponse, TraderDerivativeOrdersResponse, TraderSpotOrdersResponse,
};
use crate::route::InjectiveRoute;
use crate::volatility::TradeHistoryOptions;
use crate::OracleType;
use crate::{MarketId, SubaccountId};

pub struct InjectiveQuerier<'a> {
    querier: &'a QuerierWrapper<'a, InjectiveQueryWrapper>,
}

impl<'a> InjectiveQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<InjectiveQueryWrapper>) -> Self {
        InjectiveQuerier { querier }
    }

    pub fn query_subaccount_deposit<T: Into<SubaccountId> + Clone, P: Into<String> + Clone>(
        &self,
        subaccount_id: &'a T,
        denom: &'a P,
    ) -> StdResult<SubaccountDepositResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::SubaccountDeposit {
                subaccount_id: subaccount_id.clone().into(),
                denom: denom.clone().into(),
            },
        };

        let res: SubaccountDepositResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_derivative_market<T: Into<MarketId> + Clone>(&self, market_id: &'a T) -> StdResult<DerivativeMarketResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::DerivativeMarket {
                market_id: market_id.clone().into(),
            },
        };

        let res: DerivativeMarketResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_spot_market<T: Into<MarketId> + Clone>(&self, market_id: &'a T) -> StdResult<SpotMarketResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::SpotMarket {
                market_id: market_id.clone().into(),
            },
        };

        let res: SpotMarketResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_effective_subaccount_position<T: Into<MarketId> + Clone, P: Into<SubaccountId> + Clone>(
        &self,
        market_id: &'a T,
        subaccount_id: &'a P,
    ) -> StdResult<SubaccountEffectivePositionInMarketResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::SubaccountEffectivePositionInMarket {
                market_id: market_id.clone().into(),
                subaccount_id: subaccount_id.clone().into(),
            },
        };

        let res: SubaccountEffectivePositionInMarketResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_vanilla_subaccount_position<T: Into<MarketId> + Clone, P: Into<SubaccountId> + Clone>(
        &self,
        market_id: &'a T,
        subaccount_id: &'a P,
    ) -> StdResult<SubaccountPositionInMarketResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::SubaccountPositionInMarket {
                market_id: market_id.clone().into(),
                subaccount_id: subaccount_id.clone().into(),
            },
        };

        let res: SubaccountPositionInMarketResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_trader_derivative_orders<T: Into<MarketId> + Clone, P: Into<SubaccountId> + Clone>(
        &self,
        market_id: &'a T,
        subaccount_id: &'a P,
    ) -> StdResult<TraderDerivativeOrdersResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::TraderDerivativeOrders {
                market_id: market_id.clone().into(),
                subaccount_id: subaccount_id.clone().into(),
            },
        };

        let res: TraderDerivativeOrdersResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_trader_transient_spot_orders<T: Into<MarketId> + Clone, P: Into<SubaccountId> + Clone>(
        &self,
        market_id: &'a T,
        subaccount_id: &'a P,
    ) -> StdResult<TraderSpotOrdersResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::TraderTransientSpotOrders {
                market_id: market_id.clone().into(),
                subaccount_id: subaccount_id.clone().into(),
            },
        };

        let res: TraderSpotOrdersResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_trader_transient_derivative_orders<T: Into<MarketId> + Clone, P: Into<SubaccountId> + Clone>(
        &self,
        market_id: &'a T,
        subaccount_id: &'a P,
    ) -> StdResult<TraderDerivativeOrdersResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::TraderTransientDerivativeOrders {
                market_id: market_id.clone().into(),
                subaccount_id: subaccount_id.clone().into(),
            },
        };

        let res: TraderDerivativeOrdersResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_trader_spot_orders<T: Into<MarketId> + Clone, P: Into<SubaccountId> + Clone>(
        &self,
        market_id: &'a T,
        subaccount_id: &'a P,
    ) -> StdResult<TraderSpotOrdersResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::TraderSpotOrders {
                market_id: market_id.clone().into(),
                subaccount_id: subaccount_id.clone().into(),
            },
        };

        let res: TraderSpotOrdersResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_spot_orders_to_cancel_up_to_amount<T: Into<MarketId> + Clone, P: Into<SubaccountId> + Clone>(
        &self,
        market_id: &'a T,
        subaccount_id: &'a P,
        base_amount: FPDecimal,
        quote_amount: FPDecimal,
        strategy: i32,
        reference_price: Option<FPDecimal>,
    ) -> StdResult<TraderSpotOrdersResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::TraderSpotOrdersToCancelUpToAmount {
                market_id: market_id.clone().into(),
                subaccount_id: subaccount_id.clone().into(),
                base_amount,
                quote_amount,
                strategy,
                reference_price,
            },
        };

        let res: TraderSpotOrdersResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_derivative_orders_to_cancel_up_to_amount<T: Into<MarketId> + Clone, P: Into<SubaccountId> + Clone>(
        &self,
        market_id: &'a T,
        subaccount_id: &'a P,
        quote_amount: FPDecimal,
        strategy: i32,
        reference_price: Option<FPDecimal>,
    ) -> StdResult<TraderDerivativeOrdersResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::TraderDerivativeOrdersToCancelUpToAmount {
                market_id: market_id.clone().into(),
                subaccount_id: subaccount_id.clone().into(),
                quote_amount,
                strategy,
                reference_price,
            },
        };

        let res: TraderDerivativeOrdersResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_perpetual_market_info<T: Into<MarketId> + Clone>(&self, market_id: &'a T) -> StdResult<PerpetualMarketInfoResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::PerpetualMarketInfo {
                market_id: market_id.clone().into(),
            },
        };

        let res: PerpetualMarketInfoResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_perpetual_market_funding<T: Into<MarketId> + Clone>(&self, market_id: &'a T) -> StdResult<PerpetualMarketFundingResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::PerpetualMarketFunding {
                market_id: market_id.clone().into(),
            },
        };

        let res: PerpetualMarketFundingResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_market_volatility<T: Into<MarketId> + Clone>(
        &self,
        market_id: &'a T,
        trade_grouping_sec: u64,
        max_age: u64,
        include_raw_history: bool,
        include_metadata: bool,
    ) -> StdResult<MarketVolatilityResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::MarketVolatility {
                market_id: market_id.clone().into(),
                trade_history_options: TradeHistoryOptions {
                    trade_grouping_sec,
                    max_age,
                    include_raw_history,
                    include_metadata,
                },
            },
        };

        let res: MarketVolatilityResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_derivative_market_mid_price_and_tob<T: Into<MarketId> + Clone>(&self, market_id: &'a T) -> StdResult<MarketMidPriceAndTOBResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::DerivativeMarketMidPriceAndTob {
                market_id: market_id.clone().into(),
            },
        };

        let res: MarketMidPriceAndTOBResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_aggregate_market_volume<T: Into<MarketId> + Clone>(&self, market_id: &'a T) -> StdResult<QueryAggregateVolumeResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::AggregateMarketVolume {
                market_id: market_id.clone().into(),
            },
        };

        let res: QueryAggregateVolumeResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_aggregate_account_volume<T: Into<String> + Clone>(&self, account_id: &'a T) -> StdResult<QueryAggregateVolumeResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::AggregateAccountVolume {
                account: account_id.clone().into(),
            },
        };

        let res: QueryAggregateVolumeResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_denom_decimal<T: Into<String> + Clone>(&self, denom: &'a T) -> StdResult<QueryDenomDecimalResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::DenomDecimal { denom: denom.clone().into() },
        };

        let res: QueryDenomDecimalResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_denom_decimals<T: Into<Vec<String>> + Clone>(&self, denoms: &'a T) -> StdResult<QueryDenomDecimalsResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::DenomDecimals {
                denoms: denoms.clone().into(),
            },
        };

        let res: QueryDenomDecimalsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_spot_market_mid_price_and_tob<T: Into<MarketId> + Clone>(&self, market_id: &'a T) -> StdResult<MarketMidPriceAndTOBResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::SpotMarketMidPriceAndTob {
                market_id: market_id.clone().into(),
            },
        };

        let res: MarketMidPriceAndTOBResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_oracle_volatility(
        &self,
        base_info: &'a Option<OracleInfo>,
        quote_info: &'a Option<OracleInfo>,
        max_age: u64,
        include_raw_history: bool,
        include_metadata: bool,
    ) -> StdResult<OracleVolatilityResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Oracle,
            query_data: InjectiveQuery::OracleVolatility {
                base_info: base_info.clone(),
                quote_info: quote_info.clone(),
                oracle_history_options: Some(OracleHistoryOptions {
                    max_age,
                    include_raw_history,
                    include_metadata,
                }),
            },
        };

        let res: OracleVolatilityResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_oracle_price(&self, oracle_type: &'a OracleType, base: &str, quote: &str) -> StdResult<OraclePriceResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Oracle,
            query_data: InjectiveQuery::OraclePrice {
                oracle_type: *oracle_type,
                base: base.into(),
                quote: quote.into(),
            },
        };

        let res: OraclePriceResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_pyth_price(&self, price_id: &str) -> StdResult<PythPriceResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Oracle,
            query_data: InjectiveQuery::PythPrice {
                price_id: price_id.to_string(),
            },
        };

        let res: PythPriceResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_token_factory_denom_total_supply<T: Into<String> + Clone>(&self, denom: &'a T) -> StdResult<TokenFactoryDenomSupplyResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Tokenfactory,
            query_data: InjectiveQuery::TokenFactoryDenomTotalSupply { denom: denom.clone().into() },
        };

        let res: TokenFactoryDenomSupplyResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_token_factory_creation_fee(&self) -> StdResult<TokenFactoryCreateDenomFeeResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Tokenfactory,
            query_data: InjectiveQuery::TokenFactoryDenomCreationFee {},
        };

        let res: TokenFactoryCreateDenomFeeResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_contract_registration_info<A: Into<String> + Clone>(
        &self,
        contract_address: &'a A,
    ) -> StdResult<QueryContractRegistrationInfoResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Wasmx,
            query_data: InjectiveQuery::WasmxRegisteredContractInfo {
                contract_address: contract_address.clone().into(),
            },
        };

        let res: QueryContractRegistrationInfoResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}
