use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Map, Item};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Bid {
    pub addr: Option<Addr>,
    pub bid: Coin,
}

pub const TOTAL_BIDS: Map<&Addr, Coin> = Map::new("total bids");
pub const OWNER: Item<Addr> = Item::new("owner");
pub const HIGHEST_BID: Item<Bid> = Item::new("bid");
pub const BIDDING: Item<bool> = Item::new("bidding state");
pub const WINNER: Item<Addr> = Item::new("winner");
pub const COMMISION: Item<u64> = Item::new("commision");