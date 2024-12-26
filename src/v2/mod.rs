pub mod create_transaction;
pub mod approve;
pub mod execute;

use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};


pub const ID: Pubkey = solana_sdk::pubkey!("msigmtwzgXJHj2ext4XJjCDmpbcMuufFb5cHuwg6Xdt");





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

#[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct MultisigTx {
    __discriminator: [u8; 8],
    // The multisig account this transaction belongs to.
    multisig: Pubkey,
    // Target program to execute against.
    pub program_id: Pubkey,
    // Accounts requried for the transaction.
    pub accounts: Vec<TransactionAccount>,
    // Instruction data for the transaction.
    data: Vec<u8>,
    // signers[index] is true iff multisig.owners[index] signed the transaction.
    signers: Vec<bool>,
    // Boolean ensuring one time execution.
    did_execute: bool,
    // Owner set sequence number.
    owner_set_seqno: u32,
}