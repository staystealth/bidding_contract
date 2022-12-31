use cosmwasm_std::{entry_point, StdResult, Response, MessageInfo, Env, DepsMut, Binary, Deps, to_binary};
use msg::{InstantiateMsg, ExecuteMsg::{Bid, self, Close, Retract}};
use error::ContractError;

mod contract;
pub mod msg;
mod state;
pub mod error;
#[cfg(any(test, feature = "tests"))]
pub mod multitest;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, info, msg)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        Bid {} => contract::exec::bid(deps, info).map_err(ContractError::from),
        Close {} => contract::exec::close(deps, info).map_err(ContractError::from),
        Retract { receiver } => contract::exec::retract(deps, info, receiver).map_err(ContractError::from),
    }
}

#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: msg::QueryMsg,
) -> StdResult<Binary> {
    use msg::QueryMsg::*;

    match msg {
        HighestBid {} => to_binary(&contract::query::hbid(deps)?),
        Commision {} => to_binary(&contract::query::comm(deps)?),
        BiddingState {} => to_binary(&contract::query::biddstate(deps)?),
        Winner {} => to_binary(&contract::query::winner(deps)?), 
    }
}