#![allow(non_snake_case)]
// #![allow(dead_code)]

pub mod hash;
// pub mod jup_perps;

use std::{
    ffi::OsStr, fs, thread::{self, sleep}, time::Instant
};

use {
    borsh::{BorshDeserialize, BorshSerialize},
    futures::stream::{FuturesUnordered, StreamExt},
    serde::{Deserialize, Serialize},
    solana_client::rpc_client::RpcClient,
    solana_faucet::faucet::run_local_faucet,
    solana_measure::{measure::Measure, measure_us},
    solana_program::instruction::{AccountMeta, Instruction},
    solana_runtime::{
        bank::{Bank, TransactionSimulationResult},
        bank_forks::BankForks,
        commitment::BlockCommitmentCache,
    },
    solana_sdk::{
        account::{Account, AccountSharedData, ReadableAccount},
        address_lookup_table::AddressLookupTableAccount,
        compute_budget::ComputeBudgetInstruction,
        fee_calculator::FeeRateGovernor,
        lamports,
        message::{AddressLoader, VersionedMessage::V0},
        program_pack::Pack,
        pubkey::Pubkey,
        rent::Rent,
        signature::{self, Keypair},
        signer::Signer,
        transaction::{MessageHash, SanitizedTransaction, VersionedTransaction},
    },
    solana_streamer::socket::SocketAddrSpace,
    solana_test_validator::{TestValidator, TestValidatorGenesis, UpgradeableProgramInfo},
    std::{
        borrow::BorrowMut,
        collections::{HashMap, HashSet},
        fmt,
        fs::{File, OpenOptions},
        io::{Read, Write},
        ops::{Div, Mul},
        path::PathBuf,
        str::FromStr,
        sync::{
            atomic::{AtomicPtr, AtomicUsize, Ordering},
            Arc,
        },
        u16, u64,
    },
    tokio::task::spawn_blocking,
};

use arrayref::{array_ref, array_refs};
use solana_account_decoder::{UiAccountEncoding, UiDataSliceConfig};

use solana_client::{
    rpc_config::{
        RpcAccountInfoConfig, RpcProgramAccountsConfig, RpcSendTransactionConfig, RpcSimulateTransactionAccountsConfig, RpcSimulateTransactionConfig, RpcSimulateTransactionTokenAmountsConfig
    },
    rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType}, rpc_response::{RpcResult, RpcSimulateTransactionResult},
};
use solana_runtime::bank::{self, LoadAndExecuteTransactionsOutput};
use solana_runtime_transaction::runtime_transaction::RuntimeTransaction;
use solana_sdk::{
    account::WritableAccount, account_info::AccountInfo, address_lookup_table::state::AddressLookupTable, bpf_loader_upgradeable::UpgradeableLoaderState, commitment_config::CommitmentConfig, epoch_schedule::EpochSchedule, message::AccountKeys, program_option::COption, sysvar::{clock, Sysvar}, transaction::Transaction
};

// pub const RPC_ENDPOINT: &str = "http://127.0.0.1:8899";
pub const WSS_ENDPOINT: &str = "ws://127.0.0.1:8900";
pub const RPC_ENDPOINT: &str = "https://mainnet.helius-rpc.com/?api-key=4182c684-2ade-4428-8349-8c060c6d36ac";
pub const EDGES_FILE_PATH: &str = "/mnt/edges-solfi.json";

pub const GENESIS_PROGRAMS: [&str; 36] = [
    "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4",
    "SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe",
    "ZERor4xhbUycZ6gb9ntrhqscUcZmAbQDjEAtCf4hbZY",
    "2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c",
    "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",
    "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P",
    "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP",
    "AMM55ShdkoGRB5jVYPjWziwk8m5MpwyDgsMWHaMSQWH6",
    "BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p",
    "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK",
    "CLMM9tUoggJu2wagPkkqs9eFG4BWhVBZWkP1qv3Sp7tR",
    "CTMAxxk34HjKWxQ3QLZK1HpaLXmBveao3ESePXbiyfzh",
    "CURVGoZn8zycx6FXwwevgBTB2gVvdbGTEpvMJDbgs2t4",
    "Dooar9JkhdZ7J3LHN3A7YCuoGRUggXhQaG4kijfLGU2j",
    "DSwpgjMvXhtGn6BsbqmacdBZyfLj6jSWf3HJpdJtmg6N",
    "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB",
    "FLUXubRmkEi2q6K3Y9kBPg9248ggaZVsoSFhtJHSrm1X",
    "GFXsSL5sSaDfNFQUYsHekbWBW1TsFdjDYzACh62tEHxn",
    "H8W3ctz92svYg6mkn1UtGfu2aQr2fnUFHM1RhScEtQDt",
    "HyaB3W9q6XdA5xwpU4XnSZV94htfmbmqJXZcEbRaJutt",
    "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
    "MERLuDFBMmsHnsBPZw2sDQZHvXFMwp8EdjudcU2HKky",
    "MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA",
    "opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb",
    "PSwapMdSai8tjrEXcxFeQth87xC4rRsa4VA5mhGhXkP",
    "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX",
    "SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr",
    "SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ",
    "SSwpMgqNDsyV7mAgN9ady4bDVu5ySjmmXejXvy2vLt1",
    "SwaPpA9LAaLfeLi3a68M4DjnLqgtticKg6CnyNwgAC8",
    "treaf4wWBBty3fHdyBpo35Mz84M8k3heKXmjmi9vFt5",
    "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc",
    "24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi",
    "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C",
    "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin",
    "5quBtoiQqxF9Jv6KYKctB59NT3gtJD2Y65kdnB1Uev3h",
];
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JupSwapEdge {
    id: u32,
    pool: String,
    swap_label: String,
    mint_a: String,
    mint_b: String,
    swap_ix_a_to_b: Instruction,
    swap_ix_b_to_a: Instruction,
    fee_mint: String,
    amount_index: u32,
    ata_a: String,
    ata_b: String,
    lookuptables: Vec<String>,
    rate_a_b: u128,
    rate_a_b_log: f64,
    rate_b_a: u128,
    rate_b_a_log: f64,
    decimals_a: u64,
    decimals_b: u64,
    price_a: u64, // amount for 1 USDC
    price_b: u64, // amount for 1 USDC
    cu_consumed_a_b: u64,
    cu_consumed_b_a: u64,
}
impl JupSwapEdge {
    pub fn update_amount(&mut self, amount: u64, atob: bool) {
        let position = self.amount_index as usize;
        let replace_vec = amount.to_le_bytes().to_vec();
        let buf = if atob {
            &mut self.swap_ix_a_to_b.data[position..]
        } else {
            &mut self.swap_ix_b_to_a.data[position..]
        };
        let len = replace_vec.len().min(buf.len());
        buf[..len].copy_from_slice(&replace_vec[..len]);
    }
    // pub fn get_native_ix(&self, atob: bool) -> Instruction {

    // }
}
pub fn main() {
    // Runner::print_fetchable_accounts();
    let runner = Runner::setup_validator();
    runner.simulate_edges();
}

pub struct Runner {
    pub validator: AtomicPtr<TestValidator>,
    pub payer: Keypair,
    pub edges: AtomicPtr<HashMap<String, JupSwapEdge>>,
    pub slot: u64
}

impl Runner {
    pub fn simulate_perps(&self) {
        
    }
    pub fn simulate_edges(&self) {
        let bot_edges = unsafe { &mut *self.edges.load(Ordering::SeqCst) };

        for (pool, edge) in bot_edges.iter_mut() {
            let mut inout_table: Vec<(u64, u64)> = Vec::new();
            for value in 1..2u64 {
                let input_amount = edge.price_a * value; // 1$ worth of amount
                let out_amount = self.simulate_edge(edge, input_amount);
                inout_table.push((input_amount, out_amount));
            }

        }
    }
    pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
        let preimage = format!("{namespace}:{name}");

        let mut sighash = [0u8; 8];
        sighash.copy_from_slice(&crate::hash::hash(preimage.as_bytes()).to_bytes()[..8]);
        sighash
    }pub fn simulate_edge(&self, edge: &mut JupSwapEdge, input_amount: u64) -> u64 {

        let input_amount = 100000000;
        let atob = false;
        edge.update_amount(input_amount, atob);

        self.update_token_amount(&Pubkey::from_str(if atob {&edge.ata_a} else {&edge.ata_b}).unwrap(), input_amount);
        self.update_token_amount(&Pubkey::from_str(if atob {&edge.ata_b} else {&edge.ata_a}).unwrap(), 0);
        // println!("input {:#?}",input_amount.to_le_bytes());

        let ix = if atob {edge.swap_ix_a_to_b.clone()} else {edge.swap_ix_b_to_a.clone()};
        let bank = self.bank().clone();

        // run jup ix
        // let sim_res = self.create_and_simulate_ix(&[ix.clone()], &[], &bank, true);
        // if sim_res.result.is_err() {
        //     // println!("error {:#?}", sim_res.result.err());
        //     println!("error {:#?}", sim_res.logs);
        //     println!("error {:#?}", ix.accounts);
        //     return 0;
        // }
        // else {
        //     println!("inner_instructions {:#?}", sim_res.inner_instructions);
        //     println!("logs {:#?}", sim_res.logs);
        //     for (key, account) in sim_res.post_simulation_accounts.iter() {
        //         if key.eq(&Pubkey::from_str(&edge.ata_b).unwrap()) {
        //             let amount_out = self.get_amount_from_token_account(account.data());
        //             println!("input_amount {}, amount_out {}", input_amount, amount_out);
        //         }
        //     }
        // }
        // return 0;


        // get solfi ix
        let jup_tx = self.create_ix(&[ix.clone()], &[], &bank);
        // println!("{:#?}", jup_tx.message.static_account_keys());
        let input_bytes = input_amount.to_le_bytes();
        let solfi_pid_idx = if atob {7} else {8};
        let solfi_accounts_idx = if atob {[0, 3, 4, 1, 5, 2, 9, 8]} else {[0, 3, 4, 1, 5, 2, 10, 9]};
        let solfi_ix_data = [
            7, 
            input_bytes[0], 
            input_bytes[1], 
            input_bytes[2], 
            input_bytes[3], 
            input_bytes[4], 
            input_bytes[5], 
            input_bytes[6], 
            input_bytes[7], 
            0,0,0,0,0,0,0,0, 
            if atob {0} else {1}
        ];
        let mut hashmap_jup_accounts: HashMap<Pubkey, AccountMeta> = HashMap::new();
        for account_meta in ix.accounts.iter() {
            hashmap_jup_accounts.insert(account_meta.pubkey.clone(), account_meta.clone());
        }

        let jup_static_keys = jup_tx.message.static_account_keys();

        let solfi_pid = jup_static_keys[solfi_pid_idx].clone();
        let mut solfi_accounts: Vec<AccountMeta> = Vec::new();
        for idx in solfi_accounts_idx {
            solfi_accounts.push(hashmap_jup_accounts.get(&jup_static_keys[idx]).expect("getting account meta error").clone());
        }

        let account = bank.get_account(&Pubkey::from_str("CTaDZW2LhvHPRnA9JWcZF8R5y2mpkV2RcHAXyEoKLbzp").unwrap()).unwrap();
        let wsol_amount = self.get_amount_from_token_account(account.data());
        let account = bank.get_account(&Pubkey::from_str("JHVJLsPsbzNW8JP8cPYmrwfzD2M9aHXdFHSjeeCDERu").unwrap()).unwrap();
        let usdc_amount = self.get_amount_from_token_account(account.data());

        println!("wsol_amount {}, usdc_amount {}, slot {}", wsol_amount, usdc_amount, bank.slot());

        
        
        // println!("{:#?}", solfi_accounts);
        let ix = Instruction::new_with_bytes(
            solfi_pid, 
            &solfi_ix_data, 
            solfi_accounts
        );

        // run solfi ix
        let sim_res = self.create_and_simulate_ix(&[ix.clone()], &[], &bank, false);
        if sim_res.result.is_err() {
            println!("error {:#?}", sim_res.result.err());
            println!("logs {:#?}", sim_res.logs);
            return 0;
        }
        else {
            println!("logs {:#?}", sim_res.logs);
            for (key, account) in sim_res.post_simulation_accounts.iter() {
                if key.eq(&Pubkey::from_str(if atob {&edge.ata_b} else {&edge.ata_a}).unwrap()) {
                    let amount_out = self.get_amount_from_token_account(account.data());
                    println!("input_amount {}, amount_out {}", input_amount, amount_out);
                }
            }
        }
        
        // let validator = unsafe { &*self.validator.load(Ordering::SeqCst) };
        // let test_rpc_client = RpcClient::new_with_commitment(
        //     String::from(validator.rpc_url()),
        //     CommitmentConfig::processed(),
        // );
        // let sim_res = self.create_and_simulate_ix_rpc(&[ix.clone()], &[], &test_rpc_client, true).expect("simulate error");
        // if sim_res.value.err.is_some() {
        //     println!("error {:#?}", sim_res.value.err);
        //     return 0;
        // }
        // else {
        //     println!("inner_instructions {:#?}", sim_res.value.inner_instructions);
        //     println!("logs {:#?}", sim_res.value.logs);
        // }
        0
    }
    pub fn update_token_amount(&self, key: &Pubkey, amount: u64) {
        let bank = self.bank();
        let shared_data_res = bank.get_account(key);
        if shared_data_res.is_none() {
            println!("account not found! {:#?}", key);
            return;
        }
        let mut shared_data = shared_data_res.unwrap();
        let position = 64 as usize;
        let replace_vec = amount.to_le_bytes().to_vec();
        let data = shared_data.data_as_mut_slice();
        let buf = &mut data[position..];
        let len = replace_vec.len().min(buf.len());
        buf[..len].copy_from_slice(&replace_vec[..len]);
        bank.store_account(key, &shared_data);
    }
    pub fn get_amount_from_token_account(&self, token_account_info: &[u8]) -> u64 {
        let input = array_ref![token_account_info, 0, 72];
        let (_, amount_slice) = array_refs![input, 64, 8];

        let vault_amount = u64::from_le_bytes(*amount_slice);

        vault_amount
    }
    pub fn create_and_simulate_ix_with_amounts(
        &self,
        tx_instructions: &[Instruction],
        lookuptables: &[AddressLookupTableAccount],
        accounts: Option<RpcSimulateTransactionAccountsConfig>,
        token_amounts: Option<RpcSimulateTransactionTokenAmountsConfig>,
    ) -> RpcResult<RpcSimulateTransactionResult> {
        let payer = &self.payer;
        let signers = vec![payer];
        let rpc_client = RpcClient::new_with_commitment(
            String::from(RPC_ENDPOINT),
            CommitmentConfig::confirmed(),
        );
        let blockhash = rpc_client.get_latest_blockhash().unwrap();

        let versioned_message = V0(solana_sdk::message::v0::Message::try_compile(
            &self.payer.pubkey(),
            tx_instructions,
            lookuptables,
            blockhash,
        )
        .unwrap());

        let tx = solana_sdk::transaction::VersionedTransaction::try_new(versioned_message, &signers)
            .unwrap();
        
        rpc_client.simulate_transaction_with_config(
            &tx,
            RpcSimulateTransactionConfig {
                sig_verify: false,
                replace_recent_blockhash: false,
                commitment: Some(CommitmentConfig::processed()),
                inner_instructions: true,
                accounts,
                amounts: token_amounts,
                ..RpcSimulateTransactionConfig::default()
            },
        )
    }
    pub fn print_fetchable_accounts() {
        let edges_file_name = EDGES_FILE_PATH.to_string();
        let edges_data = fs::read_to_string(edges_file_name).expect("Failed to read JSON file");
        let edges: Vec<JupSwapEdge> =
            serde_json::from_str(&edges_data).expect("Failed to parse JSON");
        let mut bot_edges: HashMap<String, JupSwapEdge> = HashMap::new();
        for edge in edges.iter() {
            bot_edges.insert(edge.pool.clone(), edge.clone());
        }
        println!("{} edges loaded!", bot_edges.len());

        let rpc_client = RpcClient::new_with_commitment(
            String::from(RPC_ENDPOINT),
            CommitmentConfig::confirmed(),
        );

        let mut accounts: Vec<String> = Vec::new();
        let mut shared_datas: Vec<(Pubkey, AccountSharedData)> = Vec::new();
        let mut programs: Vec<String> = Vec::new();
        for (_pool, edge) in bot_edges.iter() {
            for account in edge.swap_ix_a_to_b.accounts.iter() {
                if !accounts.contains(&account.pubkey.to_string()) {
                    // check if it is program
                    let account_res = rpc_client.get_account(&account.pubkey);
                    if account_res.is_ok() {
                        let account_data = account_res.unwrap();
                        if account_data.executable {
                            if account.pubkey.to_string().ne("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA") {
                                programs.push(account.pubkey.to_string());
                                // let p_data = AccountSharedData::from(account_data);
                                // if let Ok(UpgradeableLoaderState::Program {
                                //     programdata_address,
                                // }) = p_data.deserialize_data()
                                // {
                                //     programs.push(programdata_address.to_string());
                                // }
                            }
                            
                        } else {
                            accounts.push(account.pubkey.to_string());
                            shared_datas
                                .push((account.pubkey, AccountSharedData::from(account_data)));
                        }
                    } else {
                        // println!("getting account error {:#?}", account_res.err());
                    }
                }
            }
        }

        println!(
            "accounts len: {}, programs len: {}",
            accounts.len(),
            programs.len()
        );


        for account_address in accounts.iter() {
            println!("solana account -u m --output json-compact --output-file /mnt/local-accounts-solfi/account/{}.json {}", account_address, account_address);
        }
        for account_address in programs.iter() {
            println!("solana program dump {} /mnt/local-accounts-solfi/program/{}.so", account_address, account_address);
        }
    }
    pub fn setup_validator() -> Self {
        // loading edges
        let edges_file_name = EDGES_FILE_PATH.to_string();
        let edges_data = fs::read_to_string(edges_file_name).expect("Failed to read JSON file");
        let edges: Vec<JupSwapEdge> =
            serde_json::from_str(&edges_data).expect("Failed to parse JSON");
        let edges_atomic: AtomicPtr<HashMap<String, JupSwapEdge>> =
            AtomicPtr::new(Box::into_raw(Box::new(HashMap::new())));
        let bot_edges: &mut HashMap<String, JupSwapEdge> = unsafe { &mut *edges_atomic.load(Ordering::SeqCst) };
        for edge in edges.iter() {
            bot_edges.insert(edge.pool.clone(), edge.clone());
        }
        println!("{} edges loaded!", bot_edges.len());

        let rpc_client = RpcClient::new_with_commitment(
            String::from(RPC_ENDPOINT),
            CommitmentConfig::confirmed(),
        );

        let payer = signature::read_keypair_file("/mnt/wallet/user.json").unwrap();
        println!("payer {:#?}", payer.pubkey());

        let mint_keypair = Keypair::new();
        let mint_pubkey = mint_keypair.pubkey();
        let faucet_addr = run_local_faucet(mint_keypair, None);
        

        let ledger_path = "/mnt/local-ledger";
        let _ = fs::remove_dir_all(ledger_path);
        let _ = fs::create_dir_all(ledger_path);

        let mut test_validator_genesis = TestValidatorGenesis::default();
        test_validator_genesis.ledger_path(PathBuf::from_str(ledger_path).unwrap());
        test_validator_genesis.max_genesis_archive_unpacked_size = Some(u64::MAX);
        test_validator_genesis.fee_rate_governor(FeeRateGovernor::new(0, 0));
        test_validator_genesis.rent(Rent {
            lamports_per_byte_year: 1,
            exemption_threshold: 1.0,
            ..Rent::default()
        });
        test_validator_genesis.faucet_addr(Some(faucet_addr));

        let test_validator_genesis = test_validator_genesis.add_accounts_from_directories(vec!["/mnt/local-accounts-solfi/account"]).expect("error adding json accounts");
        
        // let accounts = test_validator_genesis.accounts.clone();
        let parse_program_path = |program: &str| {
            let program_path = PathBuf::from(program);
            if !program_path.exists() {
                println!(
                    "Error: program file does not exist: {}",
                    program_path.display()
                );
                std::process::exit(1);
            }
            program_path
        };
        let mut program_files: HashSet<String> = HashSet::new();
        let matched_files = fs::read_dir("/mnt/local-accounts-solfi/program")
            .expect("program file error")
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| path.is_file() && path.extension() == Some(OsStr::new("so")))
            .map(|path| String::from(path.to_string_lossy()));

        program_files.extend(matched_files);

        let mut upgradeable_programs_to_load = vec![];
        for program in program_files.iter() {
            let program_path = parse_program_path(program);
            println!("loading program {}", program_path.file_stem().expect("error getting file name").to_str().unwrap());
            upgradeable_programs_to_load.push(UpgradeableProgramInfo {
                program_id: Pubkey::from_str(program_path.file_stem().expect("error getting file name").to_str().unwrap()).unwrap(),
                loader: solana_sdk::bpf_loader_upgradeable::id(),
                upgrade_authority: Pubkey::default(),
                program_path,
            });
        }
        
        let test_validator_genesis = test_validator_genesis.add_upgradeable_programs_with_path(&upgradeable_programs_to_load);
        let test_validator_genesis = test_validator_genesis.add_accounts_from_directories(vec!["/mnt/local-accounts-solfi/account"]).expect("error adding json accounts");
        
        let test_validator = test_validator_genesis
            .start_with_mint_address(mint_pubkey, SocketAddrSpace::Unspecified)
            .expect("validator start failed");
        // println!("test validator rpc url {}", test_validator.rpc_url());
        // let test_rpc_client = RpcClient::new_with_commitment(
        //     String::from(test_validator.rpc_url()),
        //     CommitmentConfig::confirmed(),
        // );
        // println!("account {:#?}", test_rpc_client.get_account(&Pubkey::from_str("SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe").unwrap()));
        
        let bank_forks = test_validator.bank_forks();
        let bank = bank_forks
            .read()
            .unwrap()
            .working_bank();

        bank.unfreeze_for_ledger_tool();
        while bank.freeze_started() {
            let ten_millis = std::time::Duration::from_millis(100);
            thread::sleep(ten_millis);
        }
        // for (key, data) in accounts.iter() {
        //     bank.store_account(key, data);
        // }

        bank.store_account(
            &payer.pubkey(),
            &AccountSharedData::new(
                1_000_000_000_000_000,
                0,
                &Pubkey::from_str("11111111111111111111111111111111").unwrap(),
            ),
        );

        println!("setup validator successfully");

        Runner {
            validator: AtomicPtr::new(Box::into_raw(Box::new(test_validator))),
            payer,
            edges: edges_atomic,
            slot: 0
        }
    }
    pub fn get_token_acount(
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        program_id: Pubkey,
    ) -> AccountSharedData {
        if program_id.eq(&spl_token_2022::id()) {
            Self::get_token_account_2022(mint, owner, amount)
        } else {
            Self::get_token_account_old(mint, owner, amount)
        }
    }
    pub fn get_token_account_2022(mint: Pubkey, owner: Pubkey, amount: u64) -> AccountSharedData {
        let mut account_data = vec![0; spl_token_2022::state::Account::get_packed_len()];
        let token_account = spl_token_2022::state::Account {
            mint: mint,
            owner,
            delegate: COption::None,
            amount,
            state: spl_token_2022::state::AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::Some(owner),
        };
        spl_token_2022::state::Account::pack(token_account, &mut account_data).unwrap();
        AccountSharedData::from(Account {
            lamports: 2_158_000,
            data: account_data.to_vec(),
            owner: spl_token_2022::id(),
            ..Account::default()
        })
    }
    pub fn get_token_account_old(mint: Pubkey, owner: Pubkey, amount: u64) -> AccountSharedData {
        let mut account_data = vec![0; spl_token::state::Account::get_packed_len()];
        let token_account = spl_token::state::Account {
            mint: mint,
            owner,
            delegate: COption::None,
            amount,
            state: spl_token::state::AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::Some(owner),
        };
        spl_token::state::Account::pack(token_account, &mut account_data).unwrap();
        AccountSharedData::from(Account {
            lamports: 2_040_000,
            data: account_data.to_vec(),
            owner: spl_token::id(),
            ..Account::default()
        })
    }
    pub fn is_program_account(pubkey: &str) -> bool {
        GENESIS_PROGRAMS.contains(&pubkey)
    }
    pub fn get_multiple_accounts_data(&self, pubkeys: &[Pubkey]) -> Vec<Option<Vec<u8>>> {
        let mut accounts_info: Vec<Option<Vec<u8>>> = Vec::new();
        for pubkey in pubkeys.iter() {
            accounts_info.push(self.get_account_data(pubkey));
        }
        accounts_info
    }

    pub fn get_account_data(&self, pubkey: &Pubkey) -> Option<Vec<u8>> {
        let bank = self.bank();
        match bank.get_account_with_fixed_root(pubkey) {
            Some(account) => Some(account.data().to_vec()),
            None => None,
        }
    }

    pub fn bank(&self) -> Arc<Bank> {
        let validator = unsafe { &*self.validator.load(Ordering::SeqCst) };
        let bank_forks = validator.bank_forks();
        let new_bank = bank_forks.as_ref().read().unwrap().working_bank();

        // new_bank.set_sysvar_for_tests(
        //     &clock::Clock {
        //         slot: self.slot,
        //         ..clock::Clock::default()
        //     }
        // );
        // get latest slot
        // let rpc_client = RpcClient::new_with_commitment(
        //     String::from(RPC_ENDPOINT),
        //     CommitmentConfig::processed(),
        // );
        // let new_slot = rpc_client.get_slot().expect("error getting latest slot");
        // let new_slot = bank.slot()+1;
        // let new_bank = Arc::new(Bank::new_from_parent(bank, &Pubkey::default(), new_slot));


        new_bank.unfreeze_for_ledger_tool();
        while new_bank.freeze_started() {
            let ten_millis = std::time::Duration::from_millis(100);
            thread::sleep(ten_millis);
        }
        new_bank
    }

    pub fn create_and_simulate_ix_rpc(
        &self,
        txInstructions: &[Instruction],
        lookuptables: &[AddressLookupTableAccount],
        rpc_client: &RpcClient,
        inner_instructions: bool
    ) -> RpcResult<RpcSimulateTransactionResult> {
        let signers = vec![&self.payer];
        // let bank = Self::bank(instance);
        let blockhash = rpc_client.get_latest_blockhash().expect("blockhash error");

        // println!("lookuptables: {:#?}", lookuptables.iter().map(|l| l.key).collect::<Vec<_>>());

        let versionedMessage = V0(solana_sdk::message::v0::Message::try_compile(
            &self.payer.pubkey(),
            txInstructions,
            lookuptables,
            blockhash,
        )
        .unwrap());
        let tx = VersionedTransaction::try_new(versionedMessage, &signers).unwrap();
        rpc_client.simulate_transaction_with_config(&tx, RpcSimulateTransactionConfig{
            inner_instructions,
            ..RpcSimulateTransactionConfig::default()
        })
    }
    pub fn create_and_simulate_ix(
        &self,
        txInstructions: &[Instruction],
        lookuptables: &[AddressLookupTableAccount],
        bank: &Arc<Bank>,
        cpi_recording: bool
    ) -> TransactionSimulationResult {
        let signers = vec![&self.payer];
        // let bank = Self::bank(instance);
        let blockhash = bank.last_blockhash();

        // println!("lookuptables: {:#?}", lookuptables.iter().map(|l| l.key).collect::<Vec<_>>());

        let versionedMessage = V0(solana_sdk::message::v0::Message::try_compile(
            &self.payer.pubkey(),
            txInstructions,
            lookuptables,
            blockhash,
        )
        .unwrap());
        let tx = VersionedTransaction::try_new(versionedMessage, &signers).unwrap();
        Runner::simulate_transaction(tx, bank.clone(), cpi_recording)
    }
    pub fn create_ix(
        &self,
        txInstructions: &[Instruction],
        lookuptables: &[AddressLookupTableAccount],
        bank: &Arc<Bank>,
    ) -> VersionedTransaction {
        let signers = vec![&self.payer];
        // let bank = Self::bank(instance);
        let blockhash = bank.last_blockhash();

        // println!("lookuptables: {:#?}", lookuptables.iter().map(|l| l.key).collect::<Vec<_>>());

        let versionedMessage = V0(solana_sdk::message::v0::Message::try_compile(
            &self.payer.pubkey(),
            txInstructions,
            lookuptables,
            blockhash,
        )
        .unwrap());
        let tx = VersionedTransaction::try_new(versionedMessage, &signers).unwrap();
        tx
    }


    fn sanitize_transaction(
        transaction: VersionedTransaction,
        address_loader: impl AddressLoader,
    ) -> RuntimeTransaction<SanitizedTransaction> {
        RuntimeTransaction::try_create(transaction, MessageHash::Compute, None, address_loader, &HashSet::new())
            .map_err(|err| println!("invalid transaction: {err}"))
            .unwrap()
    }

    pub fn simulate_transaction(
        transaction: VersionedTransaction,
        bank: Arc<Bank>,
        cpi_recording: bool
    ) -> TransactionSimulationResult {
        let sanitized_tx = Self::sanitize_transaction(transaction, &*bank);

        let start = Instant::now();
        let tx_res = bank.simulate_transaction_unchecked(&sanitized_tx, cpi_recording);
        let elapsed = start.elapsed().as_nanos();
        // println!("svm elapsed {}ns", elapsed);

        tx_res
    }

    

    
}
