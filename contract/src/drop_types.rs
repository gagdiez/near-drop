use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Promise};

use crate::ft::FTDrop;
use crate::token::TokenDrop;

#[derive(BorshSerialize, BorshDeserialize, PartialEq)]
pub enum DropType {
    NEAR(TokenDrop),
    FT(FTDrop),
}

pub trait Dropper {
    fn promise_for_claiming(&self, account_id: AccountId) -> Promise;
    fn promise_to_resolve_claim(&self, created: bool) -> Promise;
}

impl Dropper for DropType {
    fn promise_for_claiming(&self, account_id: AccountId) -> Promise {
        match self {
            DropType::NEAR(tkdrop) => tkdrop.promise_for_claiming(account_id),
            DropType::FT(ftdrop) => ftdrop.promise_for_claiming(account_id),
        }
    }

    fn promise_to_resolve_claim(&self, created: bool) -> Promise {
        match self {
            DropType::NEAR(tkdrop) => tkdrop.promise_to_resolve_claim(created),
            DropType::FT(_) => panic!("Not implemented"),
        }
    }
}
