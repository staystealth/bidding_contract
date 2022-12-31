use cosmwasm_std::{DepsMut, MessageInfo, StdResult, Response, Coin};
use cw2::set_contract_version;

use crate::{msg::InstantiateMsg, state::{OWNER, HIGHEST_BID, BIDDING, Bid, COMMISION}};

const ATOM: &str = "atom";
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate(deps: DepsMut, info: MessageInfo, msg: InstantiateMsg) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    
    if let Some(owner) = msg.owner {
        OWNER.save(deps.storage, &owner)?;
    } else {
        OWNER.save(deps.storage, &info.sender)?;
    };

    HIGHEST_BID.save(deps.storage, &Bid {
        addr: None,
        bid: Coin::new(0, ATOM),
    })?;
    BIDDING.save(deps.storage, &true)?;
    COMMISION.save(deps.storage, &msg.commision)?;
    Ok(Response::new())
}

pub mod query {
    use cosmwasm_std::{StdResult, Deps};

    use crate::{msg::{HBidResp, CommResp, WinnerResp, BidStateResp}, state::{HIGHEST_BID, COMMISION, BIDDING, WINNER}};

    pub fn hbid(deps: Deps) -> StdResult<HBidResp> {
        let value = HIGHEST_BID.load(deps.storage)?.bid;
        Ok(HBidResp { value: value })
    }
    
    pub fn comm(deps: Deps) -> StdResult<CommResp> {
        let value = COMMISION.load(deps.storage)?;
        Ok(CommResp { value: value })
    }

    pub fn biddstate(deps: Deps) -> StdResult<BidStateResp> {
        let value = BIDDING.load(deps.storage)?;
        Ok(BidStateResp { value: value })
    }

    pub fn winner(deps: Deps) -> StdResult<WinnerResp> {
        let value = WINNER.load(deps.storage)?;
        Ok(WinnerResp { value: value })
    }
}

pub mod exec {
    use cosmwasm_std::{DepsMut, MessageInfo, Response, Coin, BankMsg, coins, Addr, Decimal};

    use crate::{ state::{OWNER, HIGHEST_BID, TOTAL_BIDS, BIDDING, Bid, WINNER, COMMISION}, error::ContractError};

    use super::ATOM;

    pub fn bid(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        let owner = OWNER.load(deps.storage)?;
        //owner cannot send a bid
        if &info.sender == &owner {
            return Err(ContractError::BidError {});
        }

        // check if proper coin is send (ATOM in this case)
        let denom = info.funds.get(0).unwrap();
        if denom.denom != ATOM {
            return Err(ContractError::DenomError {})
        }

        let highest_bid = HIGHEST_BID.may_load(deps.storage)?.unwrap_or_default();
        let prev_bids = TOTAL_BIDS.may_load(deps.storage, &info.sender)?.unwrap_or_default();

        let current_funds = info.funds.get(0).unwrap();
        let total_funds = prev_bids.amount + current_funds.amount;

        // check if total bids(prev funds + current funds) are higher than highest bid 
        if total_funds > highest_bid.bid.amount {
            HIGHEST_BID.save(deps.storage, &Bid {
                addr: Some(info.sender.clone()),
                bid: Coin::new(u128::from(total_funds), ATOM),
            })?;
            TOTAL_BIDS.save(deps.storage, &info.sender, &Coin::new(u128::from(total_funds), ATOM))?;

            let res: Response = Response::new()
                .add_attribute("action", "bid")
                .add_attribute("bidder", info.sender.as_str());
            
            Ok(res)
        } else {
            return Err(ContractError::AmountError {});
        }
    }
    pub fn close(deps: DepsMut, info: MessageInfo)-> Result<Response, ContractError> {
        let owner = OWNER.load(deps.storage)?;
        if &info.sender != &owner {
            return Err(ContractError::Unauthorised { owner: owner.into() })
        } else {
            let highest_bid = HIGHEST_BID.load(deps.storage)?;
            let funds = coins(u128::from(highest_bid.bid.amount), ATOM);
            let bank_msg = BankMsg::Send { to_address: owner.into(), amount: funds, };

            BIDDING.save(deps.storage, &false)?;
            WINNER.save(deps.storage, &highest_bid.addr.unwrap())?;
            let resp: Response = Response::new()
                .add_message(bank_msg)
                .add_attribute("action", "close")
                .add_attribute("sender", info.sender.as_str());
        
            Ok(resp)
        }        
    }
    pub fn retract(deps: DepsMut, info: MessageInfo, addr: Option<Addr>) -> Result<Response, ContractError> {
        let owner = OWNER.load(deps.storage)?;
        let winner = WINNER.load(deps.storage)?;
        let bidding_state = BIDDING.load(deps.storage)?;

        if &info.sender == &owner {
            return Err(ContractError::WithOwnError {})
        }

        if &info.sender != &winner && bidding_state == false {
            let total_bids = TOTAL_BIDS.load(deps.storage, &info.sender)?;
            let commision = COMMISION.load(deps.storage)?;
            let comm = Decimal::percent(100) - Decimal::percent(commision);
            let funds = total_bids.amount * comm;
            let mut with_addr = &info.sender;
            
            if let Some(receiver) = addr {
                with_addr = &receiver;
                let bank_msg = BankMsg::Send { to_address: with_addr.into(), amount: coins(u128::from(funds), ATOM), };
                
                    let resp: Response = Response::new()
                            .add_message(bank_msg)
                            .add_attribute("action", "retract")
                            .add_attribute("sender", info.sender.as_str());
                    Ok(resp)
                } else {
                    let bank_msg = BankMsg::Send { to_address: with_addr.into(), amount: coins(u128::from(funds), ATOM), };
                
                    let resp: Response = Response::new()
                            .add_message(bank_msg)
                            .add_attribute("action", "retract")
                            .add_attribute("sender", info.sender.as_str());
                    Ok(resp)
                }
        }
        else {
            return Err(ContractError::WithWinError {})
        }
    }
}