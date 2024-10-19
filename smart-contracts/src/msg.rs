use std::time::Duration;
use cosmwasm_std::{Addr, Uint128, Timestamp};

use cosmwasm_std::{Binary, Coin};
use pyth_sdk_cw::{
    Price,
    PriceIdentifier,
};

use cosmwasm_schema::{
    cw_serde,
    QueryResponses,
};

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct InstantiateMsg {
    pub price_feed_id:      PriceIdentifier,
    pub pyth_contract_addr: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateBet {
        amount : Uint128,
        price : Price,
        price_key : Addr
    },
    EnterBet {
        amount : Uint128,
        expiry : Timestamp
    },
    ClaimBet {
        palyer : Addr
    },
    CloseBet {
        player : Addr
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(FetchPriceResponse)]
    FetchPrice {},
    #[returns(Coin)]
    FetchUpdateFee { vaas: Vec<Binary> },
    #[returns(Duration)]
    FetchValidTimePeriod,
}

#[cw_serde]
pub struct FetchPriceResponse {
    pub current_price: Price,
    pub ema_price:     Price,
}