use solana_sdk::{instruction::{AccountMeta, Instruction}, pubkey::Pubkey, sysvar};
use super::TransactionAccount;

pub struct ApproveTransaction {
    pub accounts: ApproveAccounts
}


pub struct ApproveAccounts {
    pub multisig: Pubkey,
    pub transaction: Pubkey,
    pub owner: Pubkey,
}

impl ApproveAccounts {
    pub fn to_account_metas(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new_readonly(self.multisig, false),
            AccountMeta::new_readonly(self.transaction, false),
            AccountMeta::new(self.owner, true),
        ]
    }
}


impl ApproveTransaction {
    pub fn instruction(&self) -> Instruction {
        let pid: Pubkey = "A9HAbnCwoD6f2NkZobKFf6buJoN9gUVVvX5PoUnDHS6u".parse().unwrap();
        let ix_data: Vec<u8> = vec![24, 220, 188, 211, 27, 15, 195, 245];
        Instruction {
            program_id: pid,
            accounts: self.accounts.to_account_metas(),
            data: ix_data,
        }
    }
}
