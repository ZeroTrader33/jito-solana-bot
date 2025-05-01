#![allow(non_snake_case)]
// #![allow(dead_code)]

pub mod hash;
pub mod jup_perps;

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
use jup_perps::{generate_perps_ix, get_perps_swap_out};
use solana_account_decoder::{UiAccount, UiAccountData, UiAccountEncoding, UiDataSliceConfig};

use solana_client::{
    rpc_config::{
        RpcAccountInfoConfig, RpcProgramAccountsConfig, RpcSendTransactionConfig, RpcSimulateTransactionAccountsConfig, RpcSimulateTransactionConfig, RpcSimulateTransactionTokenAmountsConfig
    },
    rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType}, rpc_response::{RpcResult, RpcSimulateTransactionResult},
};
use solana_runtime::bank::{self, LoadAndExecuteTransactionsOutput};
use solana_runtime_transaction::runtime_transaction::RuntimeTransaction;
use solana_sdk::{
    account::WritableAccount, account_info::AccountInfo, address_lookup_table::state::AddressLookupTable, bpf_loader_upgradeable::UpgradeableLoaderState, commitment_config::CommitmentConfig, message::AccountKeys, program_option::COption, sysvar::{clock, Sysvar}
};

// pub const RPC_ENDPOINT: &str = "http://127.0.0.1:8899";
// pub const WSS_ENDPOINT: &str = "ws://127.0.0.1:8900";
pub const RPC_ENDPOINT: &str = "https://mainnet.helius-rpc.com/?api-key=4182c684-2ade-4428-8349-8c060c6d36ac";
pub const EDGES_FILE_PATH: &str = "/mnt/edges-perps.json";
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
}
pub fn main() {
    let runner = Runner::setup_validator();
    // runner.simulate_edges();
    runner.simulate_perps();
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerpsTestCustody {
    pub name: String,
    pub key: Pubkey,
    pub mint: Pubkey,
    pub ata: Pubkey,
    pub initial_amount: u64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerpsTestCase {
    pub name: String,
    pub custody_a: PerpsTestCustody,
    pub custody_b: PerpsTestCustody
}
pub struct Runner {
    pub validator: AtomicPtr<TestValidator>,
    pub payer: Keypair,
    pub edges: AtomicPtr<HashMap<String, JupSwapEdge>>,
    pub slot: u64
}

impl Runner {
    pub fn simulate_perps(&self) {
        let mut test_custodies = Vec::new();
        // owner = DjTUqcUc9evT4LYEFF7UuFcMS8XeVNu88rJ2xqH6xi9Z
        let wsol_mint = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
        let wsol_ata = Pubkey::from_str("Fp2WbxZw4iwekiFjBzi1qPa3RXT4KH78bNhkgjB31qAj").unwrap();
        let wsol_custody = Pubkey::from_str("7xS2gz2bTp3fwCC7knJvUWTEU9Tycczu6VhJYKgi1wdz").unwrap();
        test_custodies.push(PerpsTestCustody{name: "WSOL".to_string(), key: wsol_custody, mint: wsol_mint, ata: wsol_ata, initial_amount: 1_000_000_000});

        let usdc_mint = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
        let usdc_ata = Pubkey::from_str("6Lge6DLXcAcbZ3D5PuXpKCWqidhQTYinXZ8ftjt6c3pm").unwrap();
        let usdc_custody = Pubkey::from_str("G18jKKXQwBbrHeiK3C9MRXhkHsLHf7XgCSisykV46EZa").unwrap();
        test_custodies.push(PerpsTestCustody{name: "USDC".to_string(),key: usdc_custody, mint: usdc_mint, ata: usdc_ata, initial_amount: 1_000_000_000});

        let weth_mint = Pubkey::from_str("7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs").unwrap();
        let weth_ata = Pubkey::from_str("7wLxgMeo5PeVcfwW1pzZckTCCgyb5yogRpVjh4vPJKyY").unwrap();
        let weth_custody = Pubkey::from_str("AQCGyheWPLeo6Qp9WpYS9m3Qj479t7R636N9ey1rEjEn").unwrap();
        test_custodies.push(PerpsTestCustody{name: "WETH".to_string(),key: weth_custody, mint: weth_mint, ata: weth_ata, initial_amount: 100_000_000});

        let wbtc_mint = Pubkey::from_str("3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh").unwrap();
        let wbtc_ata = Pubkey::from_str("Ayq1AVRoCAXDgCSTh8wxdf7u5nDTM83KCVbhPsJ8n6k").unwrap();
        let wbtc_custody = Pubkey::from_str("5Pv3gM9JrFFH883SWAhvJC9RPYmo8UNxuFtv5bMMALkm").unwrap();
        test_custodies.push(PerpsTestCustody{name: "WBTC".to_string(),key: wbtc_custody, mint: wbtc_mint, ata: wbtc_ata, initial_amount: 1000_000});

        let usdt_mint = Pubkey::from_str("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB").unwrap();
        let usdt_ata = Pubkey::from_str("BsTtAVZqroHaRf2h7QnGkeUEgU15PPwvCSi4kiKWGYdL").unwrap();
        let usdt_custody = Pubkey::from_str("4vkNeXiYEUizLdrpdPS1eC2mccyM4NUPRtERrk6ZETkk").unwrap();
        test_custodies.push(PerpsTestCustody{name: "USDT".to_string(), key: usdt_custody, mint: usdt_mint, ata: usdt_ata, initial_amount: 1_000_000_000});

        let mut test_cases: HashMap<String, PerpsTestCase> = HashMap::new();
        for test_custody_a in test_custodies.iter() {
            for test_custody_b in test_custodies.iter() {
                if test_custody_a.mint.ne(&test_custody_b.mint) {
                    let name = test_custody_a.name.clone() + "-" + &test_custody_b.name;
                    test_cases.insert(name.clone(), PerpsTestCase {name: name.clone(), custody_a: test_custody_a.clone(), custody_b: test_custody_b.clone()});
                }
            }
        }
        // construct atas
        let bank = self.bank().clone();

        let test_step = 1000u64;
        let test_count = 1;
        // for test_custody_a in test_custodies.iter() {
        //     bank.unfreeze_for_ledger_tool();
        //     bank.store_account(&test_custody_a.ata, &Self::get_token_acount(test_custody_a.mint, self.payer.pubkey(), test_custody_a.initial_amount, spl_token::id()));
        // }
        let simulate_balances = false;
        if simulate_balances {
            let pool_aum_usd: u128 = 2_000_000_000_000_000;
            let wsol_owned = 5_000_000_000_000_000u64;
            let wsol_target = 5000u64;
            let wsol_price = 20_000_000_000u64;
            let usdc_owned = 500_000_000_000_000u64;
            let usdc_target = 2500u64;
            let usdc_price = 100_000_000u64;
    
            // change pool aum usd
            {
                let key = Pubkey::from_str("5BUwFW4nRbftYTDMbgxykoFWqWHPzahFSNAaaaJtVKsq").unwrap();
                let mut account = bank.get_account(&key).unwrap();
                {
                    let data = account.borrow_mut().data_as_mut_slice();
                    let replace_bytes = pool_aum_usd.to_le_bytes();
                    let buf = &mut data[180..];
                    let len = 16;
                    buf[..len].copy_from_slice(&replace_bytes[..len]);
                    bank.store_account(&key, &account);
                }
            }
            //wsol aum usd
            {
                let key = Pubkey::from_str("7xS2gz2bTp3fwCC7knJvUWTEU9Tycczu6VhJYKgi1wdz").unwrap();
                let mut account = bank.get_account(&key).unwrap();
                {
                    let data = account.borrow_mut().data_as_mut_slice();
                    let replace_bytes = wsol_owned.to_le_bytes();
                    let buf = &mut data[222..];
                    let len = 8;
                    buf[..len].copy_from_slice(&replace_bytes[..len]);
    
                    let replace_bytes = wsol_target.to_le_bytes();
                    let buf = &mut data[206..];
                    let len = 8;
                    buf[..len].copy_from_slice(&replace_bytes[..len]);
    
                    bank.store_account(&key, &account);
                }
            }
            { //dove price change
                let key = Pubkey::from_str("39cWjvHrpHNz2SbXv6ME4NPhqBDBd4KsjUYv5JkHEAJU").unwrap();
                let mut account = bank.get_account(&key).unwrap();
                {
                    let data = account.borrow_mut().data_as_mut_slice();
                    let replace_bytes = wsol_price.to_le_bytes();
                    let buf = &mut data[73..];
                    let len = 8;
                    buf[..len].copy_from_slice(&replace_bytes[..len]);
                    bank.store_account(&key, &account);
                }
            }
            { //dove price change
                let key = Pubkey::from_str("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE").unwrap();
                let mut account = bank.get_account(&key).unwrap();
                {
                    let data = account.borrow_mut().data_as_mut_slice();
                    let replace_bytes = wsol_price.to_le_bytes();
                    let buf = &mut data[73..];
                    let len = 8;
                    buf[..len].copy_from_slice(&replace_bytes[..len]);
                    bank.store_account(&key, &account);
                }
            }
    
            //usdc aum usd
            {
                let key = Pubkey::from_str("G18jKKXQwBbrHeiK3C9MRXhkHsLHf7XgCSisykV46EZa").unwrap();
                let mut account = bank.get_account(&key).unwrap();
                {
                    let data = account.borrow_mut().data_as_mut_slice();
                    let replace_bytes = usdc_owned.to_le_bytes();
                    let buf = &mut data[222..];
                    let len = 8;
                    buf[..len].copy_from_slice(&replace_bytes[..len]);
    
                    let replace_bytes = usdc_target.to_le_bytes();
                    let buf = &mut data[206..];
                    let len = 8;
                    buf[..len].copy_from_slice(&replace_bytes[..len]);
    
                    bank.store_account(&key, &account);
                }
            }
            { //dove price change
                let key = Pubkey::from_str("A28T5pKtscnhDo6C1Sz786Tup88aTjt8uyKewjVvPrGk").unwrap();
                let mut account = bank.get_account(&key).unwrap();
                {
                    let data = account.borrow_mut().data_as_mut_slice();
                    let replace_bytes = usdc_price.to_le_bytes();
                    let buf = &mut data[73..];
                    let len = 8;
                    buf[..len].copy_from_slice(&replace_bytes[..len]);
                    bank.store_account(&key, &account);
                }
            }
            { //dove price change
                let key = Pubkey::from_str("Dpw1EAVrSB1ibxiDQyTAW6Zip3J4Btk2x4SgApQCeFbX").unwrap();
                let mut account = bank.get_account(&key).unwrap();
                {
                    let data = account.borrow_mut().data_as_mut_slice();
                    let replace_bytes = usdc_price.to_le_bytes();
                    let buf = &mut data[73..];
                    let len = 8;
                    buf[..len].copy_from_slice(&replace_bytes[..len]);
                    bank.store_account(&key, &account);
                }
            }
    
        }
        
        let matched_cases = vec![
            "USDC-USDT".to_owned(),
            "USDT-USDC".to_owned(),
            "WETH-WBTC".to_owned(),
            "WETH-USDC".to_owned(),
            "WETH-USDT".to_owned(),
            "WSOL-WETH".to_owned()
        ];

        let mut test_result = Vec::new();
        for (test_case_name, test_case) in test_cases.iter() {
            if !matched_cases.contains(&test_case_name) {
                continue;
            }
            let mut test_case_result = Vec::new();
            let initial_amount = test_case.custody_a.initial_amount;
            bank.unfreeze_for_ledger_tool();
            bank.store_account(&test_case.custody_a.ata, &Self::get_token_acount(test_case.custody_a.mint, self.payer.pubkey(), test_case.custody_a.initial_amount, spl_token::id()));
            bank.store_account(&test_case.custody_b.ata, &Self::get_token_acount(test_case.custody_b.mint, self.payer.pubkey(), 0, spl_token::id()));

            for test_i in 0..test_count {
                let input_amount = initial_amount + test_i * test_step;
                self.update_token_amount(&test_case.custody_a.ata, input_amount);
                self.update_token_amount(&test_case.custody_b.ata, 0);

                let ix = generate_perps_ix(
                    &test_case.custody_a.mint, 
                    &test_case.custody_b.mint, 
                    &self.payer.pubkey(), 
                    &test_case.custody_a.ata, 
                    &test_case.custody_b.ata, 
                    input_amount
                );
                bank.set_sysvar_for_tests(&clock::Clock {
                    slot: 329488571,
                    epoch_start_timestamp: 1743005206,
                    epoch: 762,
                    leader_schedule_epoch: 763,
                    unix_timestamp: 1743067798
                });
                bank.unfreeze_for_ledger_tool();
                let sim_res = self.create_and_simulate_ix(&[ix.clone()], &[], &bank, false);

                if sim_res.result.is_err() {
                    println!("error {:#?}", sim_res.result.err());
                    println!("logs {:#?}", sim_res.logs);
                }
                else {
                    // println!("logs {:#?}", sim_res.logs);
                    for log in sim_res.logs.iter() {
                        if log.contains("swap_usd_amount") {
                            println!("{}", log);
                        }
                    }
                }
                let mut amount_out = 0;
                for (post_account_key, post_account_data) in sim_res.post_simulation_accounts.iter() {
                    if post_account_key.eq(&test_case.custody_b.ata) {
                        let token_amount = self.get_amount_from_token_account(post_account_data.data());
                        amount_out = token_amount;
                    }
                }
                let pool_data_account = bank.get_account(&ix.accounts[5].pubkey).unwrap();
                let pool_data = pool_data_account.data();
                let custody_data_in_account = bank.get_account(&ix.accounts[6].pubkey).unwrap();
                let custody_data_in = custody_data_in_account.data();
                let dove_data_in_account = bank.get_account(&ix.accounts[7].pubkey).unwrap();
                let dove_data_in = dove_data_in_account.data();
                let pyth_data_in_account = bank.get_account(&ix.accounts[8].pubkey).unwrap();
                let pyth_data_in = pyth_data_in_account.data();
                let custody_data_out_account = bank.get_account(&ix.accounts[10].pubkey).unwrap();
                let custody_data_out = custody_data_out_account.data();
                let dove_data_out_account = bank.get_account(&ix.accounts[11].pubkey).unwrap();
                let dove_data_out = dove_data_out_account.data();
                let pyth_data_out_account = bank.get_account(&ix.accounts[12].pubkey).unwrap();
                let pyth_data_out = pyth_data_out_account.data();
                let sdk_out: u64 = get_perps_swap_out(pool_data, custody_data_in, dove_data_in, pyth_data_in, custody_data_out, dove_data_out, pyth_data_out, input_amount);

                test_case_result.push((input_amount, amount_out, sdk_out));
                println!("{}: in {}, out: {}, sdk out: {}, error {}", test_case_name, input_amount, amount_out, sdk_out, 1f64 - (sdk_out as f64) / (amount_out as f64));
                println!("");
            }
            test_result.push((test_case.name.clone(), test_case_result));
        }
        Self::save_result_as_file("/mnt/local-perps-result.json".to_string(), test_result);

    }


    pub fn simulate_edges(&self) {
        let bot_edges = unsafe { &mut *self.edges.load(Ordering::SeqCst) };

        for (pool, edge) in bot_edges.iter_mut() {
            let mut inout_table: Vec<(u64, u64, u64)> = Vec::new();
            for value in 1..2u64 {
                let atob = true;
                let input_amount = if atob {edge.price_a} else {edge.price_b} * value; // 1$ worth of amount
                // let start = Instant::now();
                let out = self.simulate_edge(edge, input_amount, atob);
                inout_table.push((input_amount, out.0, out.1));
                // let elapsed = start.elapsed().as_nanos();
                // let (out_amount, elapsed) = measure_us!(self.simulate_edge(edge, input_amount));
                // if out_amount > 0 {
                //     println!(
                //         "pool: {}, in: {}, out: {}, elapsed {}ns",
                //         pool, input_amount, out_amount, elapsed
                //     );
                // } else {
                //     println!("pool is not found {}", pool);
                // }
            }

            let serialized_content = serde_json::to_string(&*inout_table).unwrap();
            println!("saving result to file");
            let mut inout_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open("/mnt/perps-test-svm.json")
                .expect("error when creating file");
            inout_file.write_all(serialized_content.as_bytes()).expect("error to write");
            inout_file.flush().expect("error when fluxh");
            println!("saved result to file");

        }
    }
    pub fn save_result_as_file<T: Serialize>(file_path: String, result: T) {
        let serialized_content = serde_json::to_string(&result).unwrap();
        println!("saving result to file");
        let mut inout_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&file_path)
            .expect("error when creating file");
        inout_file.write_all(serialized_content.as_bytes()).expect("error to write");
        inout_file.flush().expect("error when fluxh");
        println!("saved result to file");
    }
    pub fn simulate_edge(&self, edge: &mut JupSwapEdge, input_amount: u64, atob: bool) -> (u64, u64) {
        // let input_amount = 500000;
        edge.update_amount(input_amount, atob);

        self.update_token_amount(&Pubkey::from_str(if atob {&edge.ata_a} else {&edge.ata_b}).unwrap(), input_amount);
        self.update_token_amount(&Pubkey::from_str(if atob {&edge.ata_b} else {&edge.ata_a}).unwrap(), 0);
        // let decoded = base64::decode(input).expect("error");

        // let ix = if atob {edge.swap_ix_a_to_b.clone()} else {edge.swap_ix_b_to_a.clone()};
        let ix = if atob {
            generate_perps_ix(
            &Pubkey::from_str(&edge.mint_a).unwrap(), 
            &Pubkey::from_str(&edge.mint_b).unwrap(), 
            &Pubkey::from_str("DjTUqcUc9evT4LYEFF7UuFcMS8XeVNu88rJ2xqH6xi9Z").unwrap(), 
            &Pubkey::from_str(&edge.ata_a).unwrap(), 
            &Pubkey::from_str(&edge.ata_b).unwrap(), 
            input_amount
            )
        }else {
            generate_perps_ix(
                &Pubkey::from_str(&edge.mint_b).unwrap(), 
                &Pubkey::from_str(&edge.mint_a).unwrap(), 
                &Pubkey::from_str("DjTUqcUc9evT4LYEFF7UuFcMS8XeVNu88rJ2xqH6xi9Z").unwrap(), 
                &Pubkey::from_str(&edge.ata_b).unwrap(), 
                &Pubkey::from_str(&edge.ata_a).unwrap(), 
                input_amount
            )
        };
        let bank = self.bank().clone();

        // println!("bank slot {}", bank.slot());
        // println!("bank epoch {}", bank.clock().epoch);
        // println!("bank epoch_start_timestamp {}", bank.clock().epoch_start_timestamp);
        // println!("bank leader_schedule_epoch {}", bank.clock().leader_schedule_epoch);
        // println!("bank unix_timestamp {}", bank.clock().unix_timestamp);
        
        bank.set_sysvar_for_tests(&clock::Clock {
            slot: 326516638,
            epoch_start_timestamp: 1741740548,
            epoch: 755,
            leader_schedule_epoch: 756,
            unix_timestamp: 1741881755
        });
        // if true {
        //     let sim_res = self.create_and_simulate_ix_with_amounts(&[ix.clone()], &[], None, None).unwrap();
        //     println!("logs {:#?}", sim_res.value.logs) ;
        //     return 0;
        // }

        
        // println!("edge {:#?}", edge);
        
        
        let pool_key = Pubkey::from_str("5BUwFW4nRbftYTDMbgxykoFWqWHPzahFSNAaaaJtVKsq").unwrap();
        let mut pool_account = bank.get_account(&pool_key).unwrap();
        let pool_data = pool_account.borrow_mut().data_as_mut_slice();

        let aum_usd_pos = 280;
        let fees_start_pos = 236;
        let sub_vector_swap_fee: [u8; 8] = pool_data[(fees_start_pos + 24)..(fees_start_pos + 32)].try_into().unwrap();
        let swap_fee = u64::from_le_bytes(sub_vector_swap_fee);
        println!("swap_base_fee {}", swap_fee);

        let sub_vector_aum_usd: [u8; 16] = pool_data[aum_usd_pos..(aum_usd_pos + 16)].try_into().unwrap();
        let pool_aum_usd = u128::from_le_bytes(sub_vector_aum_usd);
        println!("pool_aum_usd {}", pool_aum_usd);


        // println!("ix.accounts {:#?}", ix.accounts);

        let custody_key_in = Pubkey::from_str("7xS2gz2bTp3fwCC7knJvUWTEU9Tycczu6VhJYKgi1wdz").unwrap();
        let mut custody_account_in = bank.get_account(&custody_key_in).unwrap();
        let custody_data_in = custody_account_in.borrow_mut().data_as_mut_slice();

        let dove_key_in = Pubkey::from_str("39cWjvHrpHNz2SbXv6ME4NPhqBDBd4KsjUYv5JkHEAJU").unwrap();
        let mut dove_account_in = bank.get_account(&dove_key_in).unwrap();
        // {
        //     let dove_data_in = dove_account_in.borrow_mut().data_as_mut_slice();
        //     // for i in 0..(pyth_data_in.len() - 8) {
        //     //     let buf: &[u8;8] = &pyth_data_in[i..(i+8)].try_into().expect("msg");
        //     //     println!("{}: {}", i, u64::from_le_bytes(buf.clone()));
        //     // }
        //     let replace_bytes = 12432460196u64.to_le_bytes();
        //     let buf = &mut dove_data_in[73..];
        //     let len = 8;
        //     buf[..len].copy_from_slice(&replace_bytes[..len]);
        //     bank.store_account(&dove_key_in, &dove_account_in);
        // }
        let dove_data_in = dove_account_in.borrow_mut().data_as_mut_slice();

        let pyth_key_in = Pubkey::from_str("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE").unwrap();
        let mut pyth_account_in = bank.get_account(&pyth_key_in).unwrap();
        // {
        //     let pyth_data_in = pyth_account_in.borrow_mut().data_as_mut_slice();
        //     // for i in 0..(pyth_data_in.len() - 8) {
        //     //     let buf: &[u8;8] = &pyth_data_in[i..(i+8)].try_into().expect("msg");
        //     //     println!("{}: {}", i, u64::from_le_bytes(buf.clone()));
        //     // }
        //     let replace_bytes = 12433460196u64.to_le_bytes();
        //     let buf = &mut pyth_data_in[73..];
        //     let len = 8;
        //     buf[..len].copy_from_slice(&replace_bytes[..len]);
        //     bank.store_account(&pyth_key_in, &pyth_account_in);
        // }
        let pyth_data_in = pyth_account_in.borrow_mut().data_as_mut_slice();

        let custody_key_out = Pubkey::from_str("G18jKKXQwBbrHeiK3C9MRXhkHsLHf7XgCSisykV46EZa").unwrap();
        let mut custody_account_out = bank.get_account(&custody_key_out).unwrap();
        let custody_data_out = custody_account_out.borrow_mut().data_as_mut_slice();

        let dove_key_out = Pubkey::from_str("A28T5pKtscnhDo6C1Sz786Tup88aTjt8uyKewjVvPrGk").unwrap();
        let mut dove_account_out = bank.get_account(&dove_key_out).unwrap();
        
        let dove_data_out = dove_account_out.borrow_mut().data_as_mut_slice();

        let pyth_key_out = Pubkey::from_str("Dpw1EAVrSB1ibxiDQyTAW6Zip3J4Btk2x4SgApQCeFbX").unwrap();
        let mut pyth_account_out = bank.get_account(&pyth_key_out).unwrap();
        // {
        //     let pyth_data_out = pyth_account_out.borrow_mut().data_as_mut_slice();
        //     // for i in 0..(pyth_data_in.len() - 8) {
        //     //     let buf: &[u8;8] = &pyth_data_in[i..(i+8)].try_into().expect("msg");
        //     //     println!("{}: {}", i, u64::from_le_bytes(buf.clone()));
        //     // }
        //     let replace_bytes = 12431460196u64.to_le_bytes();
        //     let buf = &mut pyth_data_out[73..];
        //     let len = 8;
        //     buf[..len].copy_from_slice(&replace_bytes[..len]);
        // }
        let pyth_data_out = pyth_account_out.borrow_mut().data_as_mut_slice();

        let target_ratio_pos = 151 + 55;
        let fee_reserves_pos = 151 + 63;
        let target_ratio_vec: [u8; 8] = custody_data_in[target_ratio_pos..(target_ratio_pos + 8)].try_into().unwrap();
        let target_ratio = u64::from_le_bytes(target_ratio_vec);

        // let custody_vault_key = Pubkey::from_str("BUvduFTd2sWFagCunBPLupG8fBTJqweLw9DuhruNFSCm").unwrap();
        // let mut custody_vault_account = bank.get_account(&custody_vault_key).unwrap();
        // let custody_vault_data = custody_vault_account.borrow_mut().data_as_mut_slice();
        // let vault_amount = self.get_amount_from_token_account(custody_vault_data);
        // let token_price = 12428209281u64;
        // let custody_aum_usd = (vault_amount as u128)
        //     .checked_mul(token_price as u128)
        //     .unwrap()
        //     .checked_div(100_000_000)
        //     .unwrap()
        //     .checked_mul(1_000_000)
        //     .unwrap()
        //     .checked_div(1_000_000_000)
        //     .unwrap();


        let owned_vec: [u8; 8] = custody_data_in[(fee_reserves_pos + 8)..(fee_reserves_pos + 16)].try_into().unwrap();
        let owned = u64::from_le_bytes(owned_vec);
        let token_price = 12428209281u64;
        let custody_aum_usd = (owned as u128)
            .checked_mul(token_price as u128)
            .unwrap()
            .checked_div(100_000_000)
            .unwrap()
            .checked_mul(1_000_000)
            .unwrap()
            .checked_div(1_000_000_000)
            .unwrap();

        println!("custody_aum_usd {}", custody_aum_usd);

        let current_weightage = (custody_aum_usd as u128) * 10000 / pool_aum_usd;
        println!("cur_weightage {}, target_weightage {}", current_weightage, target_ratio);

        let new_target_ratio = 4716u64 - 100;
        let new_target_ratio_vec = new_target_ratio.to_le_bytes();
        let buf = &mut custody_data_in[target_ratio_pos..];
        let len = 8;
        // buf[..len].copy_from_slice(&new_target_ratio_vec[..len]);

        // bank.store_account(&custody_key_in, &mut custody_account_in);

        
        // let accounts: Option<RpcSimulateTransactionAccountsConfig> = Some(RpcSimulateTransactionAccountsConfig {
        //     encoding: Some(UiAccountEncoding::JsonParsed),
        //     addresses: vec![edge.ata_b.clone()]
        // });
        // let token_amounts = vec![(edge.ata_a.clone(), input_amount), (edge.ata_b.clone(), 0u64)];
        // let amounts: Option<RpcSimulateTransactionTokenAmountsConfig> = Some(RpcSimulateTransactionTokenAmountsConfig {
        //     token_amounts
        // });
        // let sim_res = self.create_and_simulate_ix_with_amounts(&[ix.clone()], &[], accounts, amounts).unwrap();
        // if sim_res.value.err.is_none() && sim_res.value.accounts.is_some() {
        //     let out_amount = Self::get_simulated_amount(&sim_res.value.accounts, 0);
        //     if input_amount > 0 && out_amount > 0 {
                
        //         let sdk_out: u64 = get_perps_swap_out(pool_data, custody_data_in, dove_data_in, pyth_data_in, custody_data_out, dove_data_out, pyth_data_out, input_amount);
        //         println!("in_amount {}, out_amount {}, sdk_out {}", input_amount, out_amount, sdk_out);
        //     }
        //     else {
        //         println!("in_amount or out_amount is zero");
        //     }
        // }
        // else {
        //     println!("sim_res.value.err {:#?}", sim_res.value.err) ;
        // }
        // // println!("logs {:#?}", sim_res.value.logs) ;
        // return 0;


        let sim_res = self.create_and_simulate_ix(&[ix.clone()], &[], &bank, false);
        
        // println!("edge.ata_a account {:#?}", spl_token::state::Account::unpack(bank.get_account(&Pubkey::from_str("CTaDZW2LhvHPRnA9JWcZF8R5y2mpkV2RcHAXyEoKLbzp").unwrap()).unwrap().data()).unwrap());
        // println!("edge.ata_b account {:#?}", spl_token::state::Account::unpack(bank.get_account(&Pubkey::from_str("JHVJLsPsbzNW8JP8cPYmrwfzD2M9aHXdFHSjeeCDERu").unwrap()).unwrap().data()).unwrap());
        // for account in ix.accounts.iter() {
        //     let acc = bank.get_account(&account.pubkey);
        //     if acc.is_some() {
        //         let acc_result = acc.unwrap();
        //         let acc_data = acc_result.data();
        //         println!("{} - {:#?}", account.pubkey, acc_data.len());
        //     }
        //     else {
        //         println!("{} - {:#?}", account.pubkey, false);
        //     }
        // }
        if sim_res.result.is_err() {
            println!("error {:#?}", sim_res.result.err());
            // println!("ix {:#?}", ix);
            println!("error {:#?}", sim_res.logs);
            // println!("inner_instructions {:#?}", sim_res.inner_instructions);
            // if edge.pool.contains("5guD4Uz462GT4Y4gEuqyGsHZ59JGxFN4a3rF6KWguMcJ") {
            // println!("5guD4Uz462GT4Y4gEuqyGsHZ59JGxFN4a3rF6KWguMcJ edge {:#?}", edge);
            // println!("5guD4Uz462GT4Y4gEuqyGsHZ59JGxFN4a3rF6KWguMcJ ix {:#?}", ix);
            // println!("edge ata a {:#?}", edge.ata_a);
            // println!("edge ata b {:#?}", edge.ata_b);
            // for ix_key in ix.accounts.iter() {
            //     println!("key {:#?} account {:#?}", ix_key.pubkey, bank.get_account(&ix_key.pubkey).unwrap_or_default());
            // }
            // println!("edge.ata_a account {:#?}", spl_token::state::Account::unpack(bank.get_account(&Pubkey::from_str(&edge.ata_a).unwrap()).unwrap().data()).unwrap());
            // println!("edge.ata_b account {:#?}", spl_token::state::Account::unpack(bank.get_account(&Pubkey::from_str(&edge.ata_b).unwrap()).unwrap().data()).unwrap());
            // println!("edge.ata_b account {:#?}", bank.get_account(&Pubkey::from_str(&edge.ata_b).unwrap()).unwrap_or_default());
            // }

            return (0, 0);
        }
        else {
            println!("logs {:#?}", sim_res.logs);
        }

        for post_account in sim_res.post_simulation_accounts.iter() {
            if post_account.0.to_string().eq(if atob {&edge.ata_b} else {&edge.ata_a}) {
                let token_amount = self.get_amount_from_token_account(post_account.1.data());
                
                let sdk_out = if atob{
                    let sdk_out: u64 = get_perps_swap_out(pool_data, custody_data_in, dove_data_in, pyth_data_in, custody_data_out, dove_data_out, pyth_data_out, input_amount);
                    println!("in_amount {}, out_amount {}, sdk_out {}", input_amount, token_amount, sdk_out);
                    sdk_out
                }
                else {
                    let sdk_out: u64 = get_perps_swap_out(pool_data, custody_data_out, dove_data_out, pyth_data_out, custody_data_in, dove_data_in, pyth_data_in, input_amount);
                    println!("in_amount {}, out_amount {}, sdk_out {}", input_amount, token_amount, sdk_out);
                    sdk_out
                };
                return (token_amount, sdk_out);
            }
        }
        (0, 0)
    }
    pub fn get_simulated_amount(accounts: &Option<Vec<Option<UiAccount>>>, index: usize) -> u64 {
        if accounts.as_ref().unwrap().len() > index {
            if let UiAccountData::Json(json_data) = &accounts.as_ref().unwrap().get(index).unwrap().as_ref().unwrap().data {
                let amount_str = json_data.parsed["info"]["tokenAmount"]["amount"].as_str().unwrap();
                let amount: u64 = u64::from_str(amount_str).unwrap_or_default();
                return amount;
            }
        }
        
        0u64
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
        bank.unfreeze_for_ledger_tool();
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
        
        let validator = unsafe { &*self.validator.load(Ordering::SeqCst) };
        let rpc_client = RpcClient::new_with_commitment(
            String::from(validator.rpc_url()),
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

        // let test_validator_genesis = test_validator_genesis.add_accounts_from_directories(vec!["/mnt/local-accounts-perps/account"]).expect("error adding json accounts");
        
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
        let matched_files = fs::read_dir("/mnt/local-accounts-perps/program")
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
        let test_validator_genesis = test_validator_genesis.add_accounts_from_directories(vec!["/mnt/local-accounts-perps/account"]).expect("error adding json accounts");
        
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
