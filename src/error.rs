use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("Unauthorized - only {owner} can close bid")]
    Unauthorised {
        owner: String,
    },
    #[error("Unexpected error")]
    UnexpectedError {},
    #[error("Owner cannot bid")]
    BidError {},
    #[error("Bid not high enough")]
    AmountError {},
    #[error("Bid done with wrong coin")]
    DenomError {},
    #[error("Winner cannot withdraw funds!")]
    WithWinError {},
    #[error("Owner cannot withdraw funds!")]
    WithOwnError {},
    #[error("Bidding is stil live")]
    BidLiveError {},
}