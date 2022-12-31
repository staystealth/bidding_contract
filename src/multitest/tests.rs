use cosmwasm_std::{Empty, Addr, coins, Coin};
use cw_multi_test::{ContractWrapper, Contract, App};

use crate::{execute, instantiate, query, error::ContractError};

use super::BiddingContract;

fn bidding_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}

const ATOM: &str = "atom";

#[test]
fn simple_bidding() {
    let sender1 = Addr::unchecked("sender1");
    let sender2 = Addr::unchecked("sender2");
    let owner = Addr::unchecked("owner");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender1, coins(30,ATOM))
            .unwrap();
        router
            .bank
            .init_balance(storage, &sender2, coins(30,ATOM))
            .unwrap();
    });

    let contract_id = app.store_code(bidding_contract());
    let contract = BiddingContract::instantiate(
        &mut app, 
        contract_id, 
        &owner, 
        None, 
        "bidding contract", 
        "bar of gold".into(), 
        None, 
        10
    ).unwrap();

    contract
        .bid(&mut app, &sender1, &coins(10, ATOM))
        .unwrap();
    
    assert_eq!(
        app.wrap().query_all_balances(&sender1).unwrap(), 
        coins(20, ATOM)
    );

    contract
        .bid(&mut app, &sender2, &coins(15, ATOM))
        .unwrap();

    assert_eq!(
        app.wrap().query_all_balances(sender2).unwrap(), 
        coins(15, ATOM)
    );

    contract
        .close(&mut app, &owner)
        .unwrap();


    assert_eq!(app.wrap().query_all_balances(contract.addr()).unwrap(), coins(10, ATOM));
    
    //check for proper retract with commision applied
    contract.retract(&mut app, &sender1, None).unwrap();
    assert_eq!(
        app.wrap().query_all_balances(&sender1).unwrap(), 
        coins(29, ATOM)
    );
    
    //check for proper winner
    let resp = contract.query_value_winner(&app).unwrap();
    assert_eq!(resp.value, Addr::unchecked("sender2"));
    
}

#[test]
fn sender_cant_close() {
    let sender1 = Addr::unchecked("sender1");
    let sender2 = Addr::unchecked("sender2");
    let owner = Addr::unchecked("owner");
    
    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender1, coins(30,ATOM))
            .unwrap();
        router
            .bank
            .init_balance(storage, &sender2, coins(30,ATOM))
            .unwrap();
    });

    let contract_id = app.store_code(bidding_contract());
    let contract = BiddingContract::instantiate(
        &mut app, 
        contract_id, 
        &owner, 
        None, 
        "bidding contract", 
        "bar of gold".into(), 
        None, 
        2
    ).unwrap();

    contract
        .bid(&mut app, &sender1, &coins(2, ATOM))
    .unwrap();
    
    contract
        .bid(&mut app, &sender2, &coins(5, ATOM))
        .unwrap();

    let err = contract
        .close(&mut app, &sender1)
        .unwrap_err();

    assert_eq!(err, ContractError::Unauthorised { owner: owner.into() });
}

#[test]
fn owner_cant_bid() {
    let sender1 = Addr::unchecked("sender1");
    let owner = Addr::unchecked("owner");
    
    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender1, coins(30,ATOM))
            .unwrap();
        router
            .bank
            .init_balance(storage, &owner, coins(30,ATOM))
            .unwrap();
    });

    let contract_id = app.store_code(bidding_contract());
    let contract = BiddingContract::instantiate(
        &mut app, 
        contract_id, 
        &owner, 
        None, 
        "bidding contract", 
        "bar of gold".into(), 
        None, 
        2
    ).unwrap();

    let err = contract
        .bid(&mut app, &owner, &coins(2, ATOM))
    .unwrap_err();

    assert_eq!(err, ContractError::BidError {});
}

#[test]
fn query_values() {
    let sender1 = Addr::unchecked("sender1");
    let sender2 = Addr::unchecked("sender2");
    let owner = Addr::unchecked("owner");
    
    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender1, coins(30,ATOM))
            .unwrap();
        router
            .bank
            .init_balance(storage, &sender2, coins(30,ATOM))
            .unwrap();
    });

    let contract_id = app.store_code(bidding_contract());
    let contract = BiddingContract::instantiate(
        &mut app, 
        contract_id, 
        &owner, 
        None, 
        "bidding contract", 
        "bar of gold".into(), 
        None, 
        10
    ).unwrap();

    contract
        .bid(&mut app, &sender1, &coins(10, ATOM))
    .unwrap();

    let resp = contract.query_value_comm(&app).unwrap();
    assert_eq!(resp.value, 10);

    let resp = contract.query_value_bidd(&app).unwrap();
    assert_eq!(resp.value, true);

    let resp = contract.query_value_hbid(&app).unwrap();
    assert_eq!(resp.value, Coin::new(10, ATOM));

    contract
        .bid(&mut app, &sender2, &coins(15, ATOM))
    .unwrap();

    let resp = contract.query_value_hbid(&app).unwrap();
    assert_eq!(resp.value, Coin::new(15, ATOM));

    contract
        .close(&mut app, &owner)
        .unwrap();

    let resp = contract.query_value_bidd(&app).unwrap();
    assert_eq!(resp.value, false);

    let resp = contract.query_value_winner(&app).unwrap();
    assert_eq!(resp.value, Addr::unchecked("sender2"));

    assert_eq!(app.wrap().query_all_balances(contract.addr()).unwrap(), coins(10, ATOM));

}