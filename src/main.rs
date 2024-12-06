use borsh::BorshDeserialize;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcSimulateTransactionConfig};
use solana_sdk::{bpf_loader_upgradeable, instruction::Instruction, signature::Keypair, signer::Signer, system_instruction, transaction::{Transaction, VersionedTransaction}};
use clap::{App, Arg, SubCommand};
use base64::{engine::general_purpose::STANDARD, Engine};
use solana_remote_wallet::{locator::Locator, remote_keypair::generate_remote_keypair, remote_wallet};
use solana_clap_utils::keypair::signer_from_path;
use v1::{approve::{ApproveAccounts, ApproveTransaction}, create_transaction::{CreateTransaction, CreateTransactionAccounts, CreateTransactionData}, execute::{ExecuteAccounts, ExecuteTransaction}, MultisigTx};
pub mod v1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("template-cli")
    .version("0.0.1")
    .author("solfarm")
    .about("template cli for rust projects")
    .arg(
        Arg::with_name("rpc-url")
            .long("rpc-url")
            .takes_value(true),
    )
    .arg(
        Arg::with_name("keypair")
        .long("keypair")
        .value_name("KEYPAIR")
        .help("specifies the keypair to use for signing transactions")
        .required(false)
    )
    .subcommand(
        SubCommand::with_name("set-buffer-auth")
        .arg(
            Arg::with_name("buffer")
            .long("buffer")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("multisig")
            .long("multisig")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("current-auth")
            .long("current-auth")
            .help("current authority")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("new-auth")
            .long("new-auth")
            .help("new authority")
            .takes_value(true)
        )
    )
    .subcommand(
        SubCommand::with_name("approve")
        .arg(
            Arg::with_name("multisig")
            .long("multisig")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("transaction")
            .long("transaction")
            .takes_value(true)
        )
    )
    .subcommand(
        SubCommand::with_name("execute")
        .arg(
            Arg::with_name("multisig")
            .long("multisig")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("multisig-signer")
            .long("multisig-signer")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("transaction")
            .long("transaction")
            .takes_value(true)
        )
    )
    .subcommand(
        SubCommand::with_name("set-upgrade-auth")
        .arg(
            Arg::with_name("program")
            .long("program")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("multisig")
            .long("multisig")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("current-auth")
            .long("current-auth")
            .help("current authority")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("new-auth")
            .long("new-auth")
            .help("new authority")
            .takes_value(true)
        )
    )
    .get_matches();
    let kp = matches.value_of("keypair").unwrap();
    let rpc_url = matches.value_of("rpc-url").unwrap();
    let rpc_client = RpcClient::new(rpc_url.to_string());

    let mut wallet_manager = remote_wallet::maybe_wallet_manager().unwrap();
    let signer = signer_from_path(&matches, kp, kp, &mut wallet_manager).unwrap();

    match matches.subcommand() {
        ("set-buffer-auth", Some(sba)) => {
            let buffer = sba.value_of("buffer").unwrap();
            let multisig = sba.value_of("multisig").unwrap();
            let current_auth = sba.value_of("current-auth").unwrap();
            let new_auth = sba.value_of("new-auth").unwrap();
            let tx_account = Keypair::new();
            let tx_data: CreateTransactionData = From::from(bpf_loader_upgradeable::set_buffer_authority(
                &buffer.parse().unwrap(),
                &current_auth.parse().unwrap(),
                &new_auth.parse().unwrap(),
            ));
            let tx_accounts = CreateTransactionAccounts {
                multisig: multisig.parse().unwrap(),
                transaction: tx_account.pubkey(),
                proposer: signer.pubkey(),
            };
            let ix = CreateTransaction {
                data: tx_data,
                accounts: tx_accounts,
            }.instruction();
            let mut tx = Transaction::new_with_payer(
                &[
                    system_instruction::create_account(
                        &signer.pubkey(),
                        &tx_account.pubkey(),
                        rpc_client.get_minimum_balance_for_rent_exemption(1_000).await?,
                        1_000,
                        &v1::ID
                    ),
                    ix,
                ],
                Some(&signer.pubkey())
            );
            let hash = rpc_client.get_latest_blockhash().await?;
            tx.partial_sign(
                &vec![
                    &tx_account,
                ],
                hash,
            );
            tx.partial_sign(&vec![signer], hash);
            println!("transaction {}", tx_account.pubkey());
            match rpc_client.send_and_confirm_transaction(&tx).await {
                Ok(sig) => println!("sent tx {sig}"),
                Err(err) => {
                    println!("failed to send tx {err:#?}")
                }
            }
        }
        ("set-upgrade-auth", Some(sba)) => {
            let program = sba.value_of("program").unwrap();
            let multisig = sba.value_of("multisig").unwrap();
            let current_auth = sba.value_of("current-auth").unwrap();
            let new_auth = sba.value_of("new-auth").unwrap();
            let tx_account = Keypair::new();
            let tx_data: CreateTransactionData = From::from(bpf_loader_upgradeable::set_upgrade_authority(
                &program.parse().unwrap(),
                &current_auth.parse().unwrap(),
                Some(&new_auth.parse().unwrap()),
            ));
            let tx_accounts = CreateTransactionAccounts {
                multisig: multisig.parse().unwrap(),
                transaction: tx_account.pubkey(),
                proposer: signer.pubkey(),
            };
            let ix = CreateTransaction {
                data: tx_data,
                accounts: tx_accounts,
            }.instruction();
            let mut tx = Transaction::new_with_payer(
                &[
                    system_instruction::create_account(
                        &signer.pubkey(),
                        &tx_account.pubkey(),
                        rpc_client.get_minimum_balance_for_rent_exemption(1_000).await?,
                        1_000,
                        &v1::ID
                    ),
                    ix,
                ],
                Some(&signer.pubkey())
            );
            let hash = rpc_client.get_latest_blockhash().await?;
            tx.partial_sign(
                &vec![
                    &tx_account,
                ],
                hash,
            );
            tx.partial_sign(&vec![signer], hash);
            println!("transaction {}", tx_account.pubkey());
            match rpc_client.send_and_confirm_transaction(&tx).await {
                Ok(sig) => println!("sent tx {sig}"),
                Err(err) => {
                    println!("failed to send tx {err:#?}")
                }
            }
        }
        ("approve", Some(ap)) => {
            let multisig= ap.value_of("multisig").unwrap();
            let transaction = ap.value_of("transaction").unwrap();
            let mut tx = Transaction::new_with_payer(
                &[ApproveTransaction {
                accounts: ApproveAccounts {
                    multisig: multisig.parse().unwrap(),
                    transaction: transaction.parse().unwrap(),
                    owner: signer.pubkey(),
                    },
                }.instruction()
                ],
                Some(&signer.pubkey())
            );
            tx.sign(&vec![
                signer,
            ], rpc_client.get_latest_blockhash().await?);
            match rpc_client.send_and_confirm_transaction_with_spinner(&tx).await {
                Ok(sig) => println!("sent tx {sig}"),
                Err(err) => {
                    println!("failed to send tx {err:#?}")
                }
            }
        }
        ("execute", Some(et)) => {
            
            let multisig= et.value_of("multisig").unwrap();
            let multisig_signer= et.value_of("multisig-signer").unwrap();
            let transaction = et.value_of("transaction").unwrap();


            let tx = MultisigTx::deserialize(&mut &rpc_client.get_account_data(&transaction.parse().unwrap()).await?[..]).unwrap();

            let mut tx = Transaction::new_with_payer(
                &[ExecuteTransaction {
                accounts: ExecuteAccounts {
                    multisig: multisig.parse().unwrap(),
                    multisig_signer: multisig_signer.parse().unwrap(),
                    transaction: transaction.parse().unwrap(),
                    },
                }.instruction(tx)
                ],
                Some(&signer.pubkey())
            );
            tx.sign(&vec![
                signer,
            ], rpc_client.get_latest_blockhash().await?);
            match rpc_client.send_and_confirm_transaction_with_spinner(&tx).await {
                Ok(sig) => println!("sent tx {sig}"),
                Err(err) => {
                    println!("failed to send tx {err:#?}")
                }
            }
        }
        _ => ()
    }
    Ok(())
}