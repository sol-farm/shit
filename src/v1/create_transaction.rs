use solana_sdk::{instruction::{AccountMeta, Instruction}, pubkey::Pubkey, sysvar};
use super::TransactionAccount;

pub struct CreateTransaction {
    pub data: CreateTransactionData,
    pub accounts: CreateTransactionAccounts
}

#[derive(borsh::BorshSerialize, borsh::BorshDeserialize, Clone)]
pub struct CreateTransactionData {
    pub pid: Pubkey,
    pub accounts: Vec<TransactionAccount>,
    pub data: Vec<u8>,
}

pub struct CreateTransactionAccounts {
    pub multisig: Pubkey,
    pub transaction: Pubkey,
    pub proposer: Pubkey,
}

impl CreateTransactionAccounts {
    pub fn to_account_metas(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new_readonly(self.multisig, false),
            AccountMeta::new(self.transaction, true),
            AccountMeta::new(self.proposer, false),
            AccountMeta::new(sysvar::rent::id(), false),
        ]
    }
}


impl CreateTransaction {
    pub fn instruction(&self) -> Instruction {
        let pid: Pubkey = "A9HAbnCwoD6f2NkZobKFf6buJoN9gUVVvX5PoUnDHS6u".parse().unwrap();
        let mut ix_data: Vec<u8> = vec![146, 7, 163, 135, 102, 44, 106, 250];
        let accounts = self.accounts.to_account_metas();
        ix_data.append(&mut borsh::to_vec(&self.data).unwrap());
        Instruction {
            program_id: pid,
            accounts,
            data: ix_data,
        }
    }
}

impl From<Instruction> for CreateTransactionData {
    fn from(value: Instruction) -> Self {
        Self {
            pid: value.program_id,
            accounts: value.accounts.into_iter().map(|acct| TransactionAccount {
                pubkey: acct.pubkey,
                is_signer: acct.is_signer,
                is_writable: acct.is_writable
            }).collect(),
            data: value.data,
        }
    }
}