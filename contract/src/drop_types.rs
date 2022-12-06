use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Promise};

use crate::{ft, token};

#[derive(BorshSerialize, BorshDeserialize, PartialEq)]
pub enum DropType {
    NEAR {
        funder: AccountId,
        tokens: u128,
    },
    FT {
        funder: AccountId,
        tokens: u128,
        ft_contract: AccountId,
    },
}

pub trait Dropper {
    fn claim_for(&self, account_id: AccountId) -> Promise;
    fn promise_to_resolve_claim(self, created: bool) -> Promise;
}

impl Dropper for DropType {
    fn claim_for(&self, account_id: AccountId) -> Promise {
        match self {
            DropType::NEAR {
                funder: _,
                tokens
            } => token::claim_near(account_id, *tokens),
            DropType::FT {
                funder: _,
                tokens,
                ft_contract,
            } => ft::claim_ft(account_id, ft_contract.clone(), *tokens),
        }
    }

    fn promise_to_resolve_claim(self, created: bool) -> Promise {
        match self {
            DropType::NEAR{ funder, tokens } => token::promise_to_resolve_claim(created, funder, tokens),
            DropType::FT { .. } => panic!("Not implemented"),
        }
    }
}
