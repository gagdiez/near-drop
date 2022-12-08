use constants::ACCESS_KEY_ALLOWANCE;
use drop_types::DropType;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PublicKey};

mod claim;
mod constants;
mod drop_types;
mod ft;
mod token;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    DropForPublicKey,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub top_level_account: AccountId,
    pub drop_for_key: UnorderedMap<PublicKey, DropType>,
}

#[near_bindgen]
impl Contract {
    #[init]
    #[private]
    pub fn new(top_level_account: AccountId) -> Self {
        Self {
            top_level_account,
            drop_for_key: UnorderedMap::new(StorageKey::DropForPublicKey),
        }
    }

    #[payable]
    pub fn create_near_drop(&mut self, public_key: PublicKey, tokens: U128) -> Promise {
        let funder = env::predecessor_account_id();
        let drop = token::create_near_drop(funder, tokens);
        self.store_drop_and_key(public_key, drop)
    }

    #[payable]
    pub fn create_ft_drop(
        &mut self,
        public_key: PublicKey,
        tokens: U128,
        ft_contract: AccountId,
    ) -> Promise {
        let funder = env::predecessor_account_id();
        let drop = ft::create_ft_drop(funder, ft_contract, tokens);
        self.store_drop_and_key(public_key, drop)
    }

    fn store_drop_and_key(&mut self, public_key: PublicKey, drop: DropType) -> Promise {
        self.drop_for_key.insert(&public_key, &drop);

        // Add key so it can be used to call `claim_for` and `create_account_and_claim`
        Promise::new(env::current_account_id()).add_access_key(
            public_key,
            ACCESS_KEY_ALLOWANCE,
            env::current_account_id(),
            "claim_for,create_account_and_claim".to_string(),
        )
    }
}
