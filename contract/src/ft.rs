use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk::{env, near_bindgen, AccountId, GasWeight, Promise, PromiseOrValue, PublicKey};

use crate::constants::*;
use crate::DropType;
use crate::{Contract, ContractExt};

const FT_STORAGE: u128 = (ACC_STORAGE*2 + 128) * env::STORAGE_PRICE_PER_BYTE;
const FT_REGISTER: u128 = 12500000000000000000000;

pub fn create_ft_drop(funder: AccountId, ft_contract: AccountId, tokens: U128) -> DropType {
    let attached = env::attached_deposit();
    let required =
        CREATE_ACCOUNT_FEE + ACCESS_KEY_ALLOWANCE + ACCESS_KEY_STORAGE + FT_STORAGE;

    assert!(attached == required, "Please attach exactly {required} yN");

    DropType::FT {
        funder,
        ft_contract,
        tokens: tokens.0,
    }
}

pub fn claim_ft(account_id: AccountId, ft_contract: AccountId, tokens: u128) -> Promise {
    let deposit_args = json!({ "account_id": account_id })
        .to_string()
        .into_bytes()
        .to_vec();
    let transfer_args = json!({"receiver_id": account_id, "amount": U128(tokens)})
        .to_string()
        .into_bytes()
        .to_vec();

    Promise::new(ft_contract)
        .function_call_weight(
            "storage_deposit".to_string(),
            deposit_args,
            FT_REGISTER,
            MIN_GAS_FOR_STORAGE_DEPOSIT,
            GasWeight(0),
        )
        .function_call_weight(
            "ft_transfer".to_string(),
            transfer_args,
            1,
            MIN_GAS_FOR_FT_TRANSFER,
            GasWeight(0),
        )
}


#[near_bindgen]
impl Contract {
    // Fund an existing drop
    pub fn ft_on_transfer(
        &mut self,
        _sender_id: AccountId,
        amount: U128,
        msg: PublicKey,
    ) -> PromiseOrValue<U128> {
        // Make sure the drop exists
        if let DropType::FT {
            funder,
            ft_contract,
            tokens,
        } = self.drop_for_key.get(&msg).expect("Missing Key")
        {
            let predecessor = env::predecessor_account_id();
            assert!(
                ft_contract == predecessor,
                "Wrong FTs, expected {ft_contract}, got {predecessor}"
            );

            // Update and insert again
            self.drop_for_key.insert(
                &msg,
                &DropType::FT {
                    funder,
                    ft_contract,
                    tokens: tokens + amount.0,
                },
            )
        } else {
            panic!("Not an FT drop")
        };

        // We do not return any tokens
        PromiseOrValue::Value(U128(0))
    }
}
