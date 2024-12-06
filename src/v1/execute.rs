use solana_sdk::{instruction::{AccountMeta, Instruction}, pubkey::Pubkey, sysvar};
use super::{MultisigTx, TransactionAccount};

pub struct ExecuteTransaction {
    pub accounts: ExecuteAccounts
}


pub struct ExecuteAccounts {
    pub multisig: Pubkey,
    pub multisig_signer: Pubkey,
    pub transaction: Pubkey,
}

impl ExecuteAccounts {
    pub fn to_account_metas(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new_readonly(self.multisig, false),
            AccountMeta::new_readonly(self.multisig_signer, false),
            AccountMeta::new(self.transaction, false),
        ]
    }
}


impl ExecuteTransaction {
    pub fn instruction(&self, tx: MultisigTx) -> Instruction {
        let pid: Pubkey = "A9HAbnCwoD6f2NkZobKFf6buJoN9gUVVvX5PoUnDHS6u".parse().unwrap();
        let ix_data: Vec<u8> = vec![38, 78, 61, 126, 27, 240, 156, 161];
        let mut accounts = self.accounts.to_account_metas();
        accounts.push(AccountMeta::new_readonly(tx.program_id, false));
        accounts.append(&mut tx.accounts.into_iter().map(From::from).collect());
        accounts.iter_mut().for_each(|acct| {
            acct.is_signer = false;
        });
        Instruction {
            program_id: pid,
            accounts,
            data: ix_data,
        }
    }
}
