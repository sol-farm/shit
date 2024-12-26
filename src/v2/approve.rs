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
            AccountMeta::new(self.transaction, false),
            AccountMeta::new(self.owner, true),
        ]
    }
}


impl ApproveTransaction {
    pub fn instruction(&self) -> Instruction {
        let pid: Pubkey = "msigmtwzgXJHj2ext4XJjCDmpbcMuufFb5cHuwg6Xdt".parse().unwrap();
        let ix_data: Vec<u8> = vec![69, 74, 217, 36, 115, 117, 97, 76];
        Instruction {
            program_id: pid,
            accounts: self.accounts.to_account_metas(),
            data: ix_data,
        }
    }
}
