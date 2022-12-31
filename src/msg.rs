use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct InstantiateMsg {
    pub commodity: String,
    pub owner: Option<Addr>,
    pub commision: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    Bid {},
    Close {},
    Retract {
        receiver: Option<Addr>
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(HBidResp)]
    HighestBid {},
    #[returns(CommResp)]
    Commision {},
    #[returns(BidStateResp)]
    BiddingState {},
    #[returns(WinnerResp)]
    Winner {},
}

#[cw_serde]
pub struct HBidResp {
    pub value: Coin,
}

#[cw_serde]
pub struct CommResp {
    pub value: u64,
}

#[cw_serde]
pub struct BidStateResp {
    pub value: bool,
}

#[cw_serde]
pub struct WinnerResp {
    pub value: Addr,
}