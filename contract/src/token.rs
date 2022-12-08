use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, Promise, PromiseError};

use crate::constants::*;
use crate::drop_types::Dropper;
use crate::{Contract, ContractExt, DropType};

// Amount needed to store the NEAR{account,u128} struct in the contract
const NEAR_STORAGE: u128 = (PK_STORAGE + ACC_STORAGE + 128) * env::STORAGE_PRICE_PER_BYTE;

#[derive(BorshSerialize, BorshDeserialize, PartialEq)]
pub struct TokenDrop {
    funder: AccountId,
    tokens: u128,
}

impl Dropper for TokenDrop {
    fn promise_for_claiming(&self, account_id: AccountId) -> Promise {
        Promise::new(account_id).transfer(self.tokens)
    }

    fn promise_to_resolve_claim(&self, created: bool) -> Promise {
        Contract::ext(env::current_account_id())
            .with_static_gas(CLAIM_CALLBACK_GAS)
            .with_unused_gas_weight(0)
            .resolve_near_claim(created, self.funder.clone(), self.tokens)
    }
}

pub fn create_near_drop(funder: AccountId, tokens: U128) -> DropType {
    assert!(tokens.0 >= 1, "Give at least 1 yN");

    let attached = env::attached_deposit();
    let required =
        tokens.0 + CREATE_ACCOUNT_FEE + ACCESS_KEY_ALLOWANCE + ACCESS_KEY_STORAGE + NEAR_STORAGE;

    assert!(attached >= required, "Please attach at least {required} yN");

    let extra = attached - required;
    if extra > 0 {
        // refund the user, we don't need that money
        Promise::new(env::predecessor_account_id()).transfer(extra);
    }

    DropType::NEAR(TokenDrop {
        funder,
        tokens: tokens.0,
    })
}

#[near_bindgen]
impl Contract {
    pub fn resolve_near_claim(
        created: bool,
        funder: AccountId,
        tokens: u128,
        #[callback_result] result: Result<(), PromiseError>,
    ) -> bool {
        let mut to_refund = ACCESS_KEY_STORAGE + NEAR_STORAGE;

        if !created {
            to_refund += CREATE_ACCOUNT_FEE;
        }

        if result.is_err() {
            to_refund += tokens
        }

        // Return the money
        Promise::new(funder).transfer(to_refund);
        true
    }
}
