use cosmwasm_std::{QuerierWrapper, StdResult};

use crate::derivative::{DerivativeOracleHistoryOptions, DerivativeOracleInfo, DerivativeTradeHistoryOptions};
use crate::query::{
    DerivativeMarketMidPriceResponse, DerivativeMarketResponse, DerivativeMarketVolatilityResponse, DerivativeOracleVolatilityResponse,
    InjectiveQuery, InjectiveQueryWrapper, PerpetualMarketFundingResponse, PerpetualMarketInfoResponse, SpotMarketMidPriceResponse,
    SpotMarketResponse, SpotMarketVolatilityResponse, SubaccountDepositResponse, SubaccountEffectivePositionInMarketResponse,
    SubaccountPositionInMarketResponse, TraderDerivativeOrdersResponse, TraderSpotOrdersResponse,
};

use crate::route::InjectiveRoute;

pub struct InjectiveQuerier<'a> {
    querier: &'a QuerierWrapper<'a, InjectiveQueryWrapper>,
}

impl<'a> InjectiveQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<InjectiveQueryWrapper>) -> Self {
        InjectiveQuerier { querier }
    }

    pub fn query_subaccount_deposit<T: Into<String>>(&self, subaccount_id: T, denom: T) -> StdResult<SubaccountDepositResponse> {
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

    pub fn query_derivative_market<T: Into<String>>(&self, market_id: T) -> StdResult<DerivativeMarketResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::DerivativeMarket { market_id: market_id.into() },
        };

        let res: DerivativeMarketResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_spot_market<T: Into<String>>(&self, market_id: T) -> StdResult<SpotMarketResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::SpotMarket { market_id: market_id.into() },
        };

        let res: SpotMarketResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_effective_subaccount_position<T: Into<String>>(
        &self,
        market_id: T,
        subaccount_id: T,
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

    pub fn query_vanilla_subaccount_position<T: Into<String>>(
        &self,
        market_id: T,
        subaccount_id: T,
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

    pub fn query_trader_derivative_orders<T: Into<String>>(&self, market_id: T, subaccount_id: T) -> StdResult<TraderDerivativeOrdersResponse> {
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

    pub fn query_trader_transient_derivative_orders<T: Into<String>>(
        &self,
        market_id: T,
        subaccount_id: T,
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

    pub fn query_trader_spot_orders<T: Into<String>>(&self, market_id: T, subaccount_id: T) -> StdResult<TraderSpotOrdersResponse> {
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

    pub fn query_perpetual_market_info<T: Into<String>>(&self, market_id: T) -> StdResult<PerpetualMarketInfoResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::PerpetualMarketInfo { market_id: market_id.into() },
        };

        let res: PerpetualMarketInfoResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_perpetual_market_funding<T: Into<String>>(&self, market_id: T) -> StdResult<PerpetualMarketFundingResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::PerpetualMarketFunding { market_id: market_id.into() },
        };

        let res: PerpetualMarketFundingResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_derivative_market_volatility<T: Into<String>>(
        &self,
        market_id: T,
        trade_grouping_sec: u64,
        max_age: u64,
        include_raw_history: bool,
        include_metadata: bool,
    ) -> StdResult<DerivativeMarketVolatilityResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::DerivativeMarketVolatility {
                market_id: market_id.into(),
                trade_history_options: DerivativeTradeHistoryOptions {
                    trade_grouping_sec,
                    max_age,
                    include_raw_history,
                    include_metadata,
                },
            },
        };

        let res: DerivativeMarketVolatilityResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_derivative_oracle_volatility<T: Into<String>>(
        &self,
        symbol: T,
        scale_factor: u32,
        max_age: u64,
        include_raw_history: bool,
        include_metadata: bool,
    ) -> StdResult<DerivativeOracleVolatilityResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::DerivativeOracleVolatility {
                base_info: Some(DerivativeOracleInfo {
                    symbol: symbol.into(),
                    scale_factor,
                }),
                quote_info: None,
                oracle_history_options: DerivativeOracleHistoryOptions {
                    max_age,
                    include_raw_history,
                    include_metadata,
                },
            },
        };

        let res: DerivativeOracleVolatilityResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_spot_market_volatility<T: Into<String>>(&self, market_id: T, from: i64) -> StdResult<SpotMarketVolatilityResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::SpotMarketVolatility {
                market_id: market_id.into(),
                from,
            },
        };

        let res: SpotMarketVolatilityResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_derivative_market_mid_price<T: Into<String>>(&self, market_id: T) -> StdResult<DerivativeMarketMidPriceResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::DerivativeMarketMidPrice { market_id: market_id.into() },
        };

        let res: DerivativeMarketMidPriceResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_spot_market_mid_price<T: Into<String>>(&self, market_id: T) -> StdResult<SpotMarketMidPriceResponse> {
        let request = InjectiveQueryWrapper {
            route: InjectiveRoute::Exchange,
            query_data: InjectiveQuery::SpotMarketMidPrice { market_id: market_id.into() },
        };

        let res: SpotMarketMidPriceResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}
