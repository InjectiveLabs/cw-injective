use cosmwasm_std::{QuerierWrapper, StdResult};

use injective_math::FPDecimal;

use crate::oracle::{OracleHistoryOptions, OracleInfo};
use crate::query::{
    DerivativeMarketMidPriceAndTOBResponse, DerivativeMarketResponse, InjectiveQuery, InjectiveQueryWrapper, MarketVolatilityResponse,
    OraclePriceResponse, OracleVolatilityResponse, PerpetualMarketFundingResponse, PerpetualMarketInfoResponse, SpotMarketMidPriceAndTOBResponse,
    SpotMarketResponse, SubaccountDepositResponse, SubaccountEffectivePositionInMarketResponse, SubaccountPositionInMarketResponse,
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

    pub fn query_subaccount_deposit<T: Into<SubaccountId>, P: Into<String>>(
        &self,
        subaccount_id: T,
        denom: P,
    ) -> StdResult<SubaccountDepositResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::SubaccountDeposit {
                subaccount_id: subaccount_id.into(),
                denom: denom.into(),
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

    pub fn query_spot_market<T: Into<MarketId>>(&self, market_id: T) -> StdResult<SpotMarketResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::SpotMarket { market_id: market_id.into() },
        };

        let res: SpotMarketResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_effective_subaccount_position<T: Into<MarketId>, P: Into<SubaccountId>>(
        &self,
        market_id: T,
        subaccount_id: P,
    ) -> StdResult<SubaccountEffectivePositionInMarketResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::SubaccountEffectivePositionInMarket {
                market_id: market_id.into(),
                subaccount_id: subaccount_id.into(),
            },
        };

        let res: SubaccountEffectivePositionInMarketResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_vanilla_subaccount_position<T: Into<MarketId>, P: Into<SubaccountId>>(
        &self,
        market_id: T,
        subaccount_id: P,
    ) -> StdResult<SubaccountPositionInMarketResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::SubaccountPositionInMarket {
                market_id: market_id.into(),
                subaccount_id: subaccount_id.into(),
            },
        };

        let res: SubaccountPositionInMarketResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_trader_derivative_orders<T: Into<MarketId>, P: Into<SubaccountId>>(
        &self,
        market_id: T,
        subaccount_id: P,
    ) -> StdResult<TraderDerivativeOrdersResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::TraderDerivativeOrders {
                market_id: market_id.into(),
                subaccount_id: subaccount_id.into(),
            },
        };

        let res: TraderDerivativeOrdersResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_trader_transient_spot_orders<T: Into<MarketId>, P: Into<SubaccountId>>(
        &self,
        market_id: T,
        subaccount_id: P,
    ) -> StdResult<TraderSpotOrdersResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::TraderTransientSpotOrders {
                market_id: market_id.into(),
                subaccount_id: subaccount_id.into(),
            },
        };

        let res: TraderSpotOrdersResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_trader_transient_derivative_orders<T: Into<MarketId>, P: Into<SubaccountId>>(
        &self,
        market_id: T,
        subaccount_id: P,
    ) -> StdResult<TraderDerivativeOrdersResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::TraderTransientDerivativeOrders {
                market_id: market_id.into(),
                subaccount_id: subaccount_id.into(),
            },
        };

        let res: TraderDerivativeOrdersResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_trader_spot_orders<T: Into<MarketId>, P: Into<SubaccountId>>(
        &self,
        market_id: T,
        subaccount_id: P,
    ) -> StdResult<TraderSpotOrdersResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::TraderSpotOrders {
                market_id: market_id.into(),
                subaccount_id: subaccount_id.into(),
            },
        };

        let res: TraderSpotOrdersResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_spot_orders_to_cancel_up_to_amount<T: Into<MarketId>, P: Into<SubaccountId>>(
        &self,
        market_id: T,
        subaccount_id: P,
        base_amount: FPDecimal,
        quote_amount: FPDecimal,
        strategy: i32,
        reference_price: Option<FPDecimal>,
    ) -> StdResult<TraderSpotOrdersResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::TraderSpotOrdersToCancelUpToAmount {
                market_id: market_id.into(),
                subaccount_id: subaccount_id.into(),
                base_amount,
                quote_amount,
                strategy,
                reference_price,
            },
        };

        let res: TraderSpotOrdersResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_derivative_orders_to_cancel_up_to_amount<T: Into<MarketId>, P: Into<SubaccountId>>(
        &self,
        market_id: T,
        subaccount_id: P,
        quote_amount: FPDecimal,
        strategy: i32,
        reference_price: Option<FPDecimal>,
    ) -> StdResult<TraderDerivativeOrdersResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::TraderDerivativeOrdersToCancelUpToAmount {
                market_id: market_id.into(),
                subaccount_id: subaccount_id.into(),
                quote_amount,
                strategy,
                reference_price,
            },
        };

        let res: TraderDerivativeOrdersResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_perpetual_market_info<T: Into<MarketId>>(&self, market_id: T) -> StdResult<PerpetualMarketInfoResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::PerpetualMarketInfo { market_id: market_id.into() },
        };

        let res: PerpetualMarketInfoResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_perpetual_market_funding<T: Into<MarketId>>(&self, market_id: T) -> StdResult<PerpetualMarketFundingResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::PerpetualMarketFunding { market_id: market_id.into() },
        };

        let res: PerpetualMarketFundingResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_market_volatility<T: Into<MarketId>>(
        &self,
        market_id: T,
        trade_grouping_sec: u64,
        max_age: u64,
        include_raw_history: bool,
        include_metadata: bool,
    ) -> StdResult<MarketVolatilityResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::MarketVolatility {
                market_id: market_id.into(),
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

    pub fn query_derivative_market_mid_price_and_tob<T: Into<MarketId>>(&self, market_id: T) -> StdResult<DerivativeMarketMidPriceAndTOBResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::DerivativeMarketMidPriceAndTob { market_id: market_id.into() },
        };

        let res: DerivativeMarketMidPriceAndTOBResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_spot_market_mid_price_and_tob<T: Into<MarketId>>(&self, market_id: T) -> StdResult<SpotMarketMidPriceAndTOBResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::SpotMarketMidPriceAndTob { market_id: market_id.into() },
        };

        let res: SpotMarketMidPriceAndTOBResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_oracle_volatility(
        &self,
        base_info: Option<OracleInfo>,
        quote_info: Option<OracleInfo>,
        max_age: u64,
        include_raw_history: bool,
        include_metadata: bool,
    ) -> StdResult<OracleVolatilityResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Oracle,
            query_data: InjectiveQuery::OracleVolatility {
                base_info,
                quote_info,
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

    pub fn query_oracle_price(&self, oracle_type: OracleType, base: String, quote: String) -> StdResult<OraclePriceResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Oracle,
            query_data: InjectiveQuery::OraclePrice { oracle_type, base, quote },
        };

        let res: OraclePriceResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_token_factory_denom_total_supply<T: Into<String>>(&self, denom: T) -> StdResult<TokenFactoryDenomSupplyResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Tokenfactory,
            query_data: InjectiveQuery::TokenFactoryDenomTotalSupply { denom: denom.into() },
        };

        let res: TokenFactoryDenomSupplyResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}
