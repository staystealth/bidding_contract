use cosmwasm_std::{Addr, StdResult, Coin};
use cw_multi_test::{App, Executor, ContractWrapper};

use crate::{msg::{InstantiateMsg, ExecuteMsg, QueryMsg, HBidResp, CommResp, BidStateResp, WinnerResp}, execute, instantiate, query, error::ContractError};

#[cfg(test)]
mod tests;

pub struct BiddingContract(Addr);

impl BiddingContract {
    pub fn addr(&self) -> &Addr {
        &self.0
    }

    pub fn store_code(app: &mut App) -> u64 {
        let contract = ContractWrapper::new(execute, instantiate, query);
        app.store_code(Box::new(contract))
    }
    
    #[track_caller]
    pub fn instantiate(
        app: &mut App, 
        code_id: u64, 
        sender: &Addr, 
        _admin: Option<&Addr>,
        label: &str,
        commodity: String,
        owner: Option<Addr>,
        commision: u64,
    ) -> StdResult<BiddingContract> {
        app.instantiate_contract(
            code_id, 
            sender.clone(), 
            &InstantiateMsg { commodity, owner, commision }, 
            &[], 
            label, 
            None,
        )
        .map_err(|err| err.downcast().unwrap())
        .map(BiddingContract)
    }
    
    #[track_caller]
    pub fn bid(
        &self, 
        app: &mut App, 
        sender: &Addr, 
        funds: &[Coin]
    ) -> Result<(), ContractError> {
        app.execute_contract(
            sender.clone(), 
            self.0.clone(), 
            &ExecuteMsg::Bid {}, 
            funds)
            .map_err(|err|err.downcast::<ContractError>().unwrap())?;

        Ok(())
    }

    #[track_caller]
    pub fn close(
        &self,
        app: &mut App,
        sender: &Addr,
    ) -> Result<(), ContractError> {
        app.execute_contract(
            sender.clone(), 
            self.0.clone(), 
            &ExecuteMsg::Close {}, 
            &[])
            .map_err(|err| err.downcast::<ContractError>().unwrap())?;
        Ok(())
    }

    #[track_caller]
    pub fn retract(
        &self, 
        app: &mut App, 
        sender: &Addr, 
        addr: Option<Addr>, 
    ) -> Result<(), ContractError> {
        app
            .execute_contract(
                sender.clone(), 
                self.0.clone(), 
                &ExecuteMsg::Retract { receiver: addr }, 
                &[],
            ).map_err(|err| err.downcast::<ContractError>().unwrap())?;
            

        Ok(())
    }

    #[track_caller]
    pub fn query_value_hbid(&self, app: &App) -> StdResult<HBidResp> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::HighestBid {})
    }
    #[track_caller]
    pub fn query_value_comm(&self, app: &App) -> StdResult<CommResp> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::Commision {})
    }
    #[track_caller]
    pub fn query_value_bidd(&self, app: &App) -> StdResult<BidStateResp> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::BiddingState {})
    }
    #[track_caller]
    pub fn query_value_winner(&self, app: &App) -> StdResult<WinnerResp> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::Winner {})
    }

}