pub mod create_transaction;
pub mod approve;

use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};


pub const ID: Pubkey = solana_sdk::pubkey!("A9HAbnCwoD6f2NkZobKFf6buJoN9gUVVvX5PoUnDHS6u");





#[derive(borsh::BorshSerialize, borsh::BorshDeserialize, Clone)]
pub struct TransactionAccount {
    pubkey: Pubkey,
    is_signer: bool,
    is_writable: bool,
}


impl From<TransactionAccount> for AccountMeta {
    fn from(account: TransactionAccount) -> AccountMeta {
        match account.is_writable {
            false => AccountMeta::new_readonly(account.pubkey, account.is_signer),
            true => AccountMeta::new(account.pubkey, account.is_signer),
        }
    }
}
