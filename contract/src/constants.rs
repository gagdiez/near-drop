use near_sdk::Gas;

// Bytes used to store common data types
pub const PK_STORAGE: u128 = 32; // PublicKey
pub const ACC_STORAGE: u128 = 32 + 64; // AccountId

/*
    minimum amount of storage required to store an access key on the contract
    1_330_000_000_000_000_000_000 Simple linkdrop: 0.00133 $NEAR
    2_420_000_000_000_000_000_000 NFT Linkdrop: 0.00242 $NEAR
*/
pub const ACCESS_KEY_STORAGE: u128 = 1_000_000_000_000_000_000_000; // 0.001 N

// Allowance for the access key to cover GAS fees when the account is claimed.
// This amount will not be "reserved" on the contract but must be available when GAS is burnt using the access key.
pub const ACCESS_KEY_ALLOWANCE: u128 = 20_000_000_000_000_000_000_000; // 0.02 N (200 TGas)

// Cost of creating a new account with longest possible name
pub const CREATE_ACCOUNT_FEE: u128 = 1_840_000_000_000_000_000_000; // 0.00184 N

// Minimum GAS for callback. Any unspent GAS will be added according to the weights)
pub const CREATE_CALLBACK_GAS: Gas = Gas(55_000_000_000_000); // 55 TGas
pub const CLAIM_CALLBACK_GAS: Gas = Gas(5_000_000_000_000); // 5 TGas

// Actual amount of GAS to attach when creating a new account. No unspent GAS will be attached on top of this (weight of 0)
pub const GAS_FOR_CREATE_ACCOUNT: Gas = Gas(28_000_000_000_000); // 28 TGas

// FT
pub const MIN_GAS_FOR_STORAGE_DEPOSIT: Gas = Gas(5_000_000_000_000); // 5 TGas
pub const MIN_GAS_FOR_FT_TRANSFER: Gas = Gas(5_000_000_000_000); // 5 TGas
