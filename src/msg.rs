/// A TWAP implementation for the FIN order book DEX, based on the Uniswap V2 implementation
/// https://docs.uniswap.org/protocol/V2/concepts/core-concepts/oracles
///
///
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal, Timestamp};

#[cw_serde]
pub struct InstantiateMsg {
    owner: Addr,
    // blocks where min(v[].timestamp) - block.timestamp > max_age will be pruned
    max_age: Timestamp,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// This will append a new CumulativePrice to the history of each registered pair
    /// ie timestamp: block.time, value: values[-1].value + ((block.time - values[-1].timestamp) * mid-market price from FIN)
    Run {},

    /// Add a FIN pair
    Register { addr: Addr },

    UpdateConfig {
        owner: Option<Addr>,
        max_age: Option<Timestamp>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Query the historic cumulative prices, so that a TWAP can be calculated for an arbitrary range.
    /// Eg timestamp: T-24h, offset: 0, to get sequential values for 24h ago to calculate a spot price 24h ago
    /// or T=now, offset=24h to get a 24 moving window TWAP value
    /// or T=date_trunc(now, 'day'), offset=24h to get yesterday's price
    #[returns(PriceResponse)]
    Price {
        pair: Addr,
        timestamp: Timestamp,
        offset: Timestamp,
    },
    #[returns(PairsResponse)]
    Pairs {
        /// Optionally supply the offset for the current price calculation
        offset: Option<Timestamp>,
    },
}

#[cw_serde]
pub struct PriceResponse {
    /// The TWAP computed value for this query
    /// I.E. (raw[1].value - raw[0].value) / (raw[1].timestamp - raw[0].timestamp)
    price: Decimal,
    /// The raw cumulative values that were used to calcualte this price
    raw: [CumulativePriceResponse; 2],
}

#[cw_serde]
pub struct CumulativePriceResponse {
    timestamp: Timestamp,
    value: Decimal,
}

#[cw_serde]
pub struct PairsResponse {
    pairs: Vec<PairResponse>,
}

#[cw_serde]
pub struct PairResponse {
    addr: Addr,
    // Current spot price
    price: PriceResponse,
}
