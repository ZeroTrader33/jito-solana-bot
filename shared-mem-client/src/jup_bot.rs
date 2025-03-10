use solana_faucet::faucet::run_local_faucet;
use solana_rpc_client_api::bundles::RpcBundleSimulationSummary;
use solana_runtime::bank::{Bank, TransactionSimulationResult};
use solana_runtime_transaction::{runtime_transaction::RuntimeTransaction, transaction_with_meta::TransactionWithMeta};
use solana_sdk::{account::WritableAccount, account::Account, fee_calculator::FeeRateGovernor, program_option::COption, program_pack::Pack, rent::Rent, transaction::{MessageHash, SanitizedTransaction}};
use solana_streamer::socket::SocketAddrSpace;
use solana_test_validator::{TestValidator, TestValidatorGenesis};

use {
    crate::{bank_bot::PRICE_MULTIPLIER, big_num::{MulDiv, UnsafeMathTrait, U128}, invariant::{self, invariant_math}, orca_whirlpool, safe_math::SafeMath, u128x128_math::Rounding, utils_math::{safe_mul_shr_cast, safe_shl_div_cast}}, anchor_lang::solana_program::system_instruction::SystemInstruction, arrayref::{array_ref, array_refs}, borsh::{BorshDeserialize, BorshSerialize}, dashmap::DashMap, futures::{
      stream::{FuturesUnordered, StreamExt},
      TryFutureExt,
    }, 
    // hwloc::{CpuSet, Topology, CPUBIND_THREAD}, 
    num_traits::Pow, rand::seq::SliceRandom, serde::{Deserialize, Serialize}, solana_account_decoder::{UiAccount, UiAccountData, UiAccountEncoding, UiDataSliceConfig}, solana_client::{client_error::Result as ClientResult, rpc_client::RpcClient, rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig, RpcSendTransactionConfig, RpcSimulateTransactionAccountsConfig, RpcSimulateTransactionConfig, RpcSimulateTransactionTokenAmountsConfig}, rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType}, rpc_response::{RpcResult, RpcSimulateTransactionResult}}, solana_measure::{measure::Measure, measure_us}, solana_rpc_client_api::bundles::{RpcSimulateBundleConfig, RpcSimulateBundleResult}, solana_sdk::{
        account::{AccountSharedData, ReadableAccount}, address_lookup_table::{self, AddressLookupTableAccount}, bundle::VersionedBundle, commitment_config::{CommitmentConfig, CommitmentLevel}, compute_budget::ComputeBudgetInstruction, hash::Hash, instruction::{AccountMeta, Instruction}, message::{AddressLoader, VersionedMessage::V0}, pubkey::Pubkey, signature::{self, Keypair, Signature, Signer}, signer::SignerError, system_instruction::transfer, transaction::VersionedTransaction
    }, std::{
        collections::HashMap, fmt, fs::{self, File, OpenOptions}, io::{Read, Write}, ops::{Add, Mul}, str::FromStr, sync::{
            atomic::{AtomicBool, AtomicPtr, AtomicU64, AtomicU8, AtomicUsize, Ordering}, Arc, Mutex, RwLock
        }, thread::{sleep, spawn}, time::{Duration, UNIX_EPOCH}
    }, tokio::task::spawn_blocking
};
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering as CmpOrdering;
use std::f64;
use rayon::prelude::*;
use solana_program::bpf_loader_upgradeable::UpgradeableLoaderState;

pub const RPC_ENDPOINT: &str = "http://127.0.0.1:8899";
pub const WSS_ENDPOINT: &str = "ws://127.0.0.1:8900";
pub const BLOCKHASH_UPDATE_PERIOD: u64 = 500;
pub const WSOL_MINT: &str = "So11111111111111111111111111111111111111112";
pub const RATE_VALUE: u64 = 1;
pub const FEE_MULTIPLIER: u32 = 10000;
pub const ALLOWED_AMOUNT_CHANGE: u64 = 999;

pub const JITO_TIP_ACCOUNTS: [&str; 8] = [
    "96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5",

    "HFqU5x63VTqvQss8hp11i4wVV8bD44PvwucfZ2bU7gRe",

    "Cw8CFyM9FkoMi7K7Crf6HNQqf4uEMzpKw6QNghXLvLkY",

    "ADaUMid9yfUytqMBgopwjb2DTLSokTSzL1zt6iGPaS49",

    "DfXygSm4jCyNCybVYYK6DwvWqjKee8pbDmJGcLWNDXjh",

    "ADuUkR4vqLUMWXxW9gh6D6L8pMSawimctcNZ5pGwDcEt",

    "DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiL2KRL",

    "3AVi9Tg9Uo68tJfuvoKvqKNWKkC5wPdSSdeBnizKZ6jT"
];
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
pub struct JupBot {
    pub is_running: AtomicBool,
    pub last_blockhash_timestamp: AtomicU64,
    pub latest_blockhash: AtomicPtr<Option<Hash>>,
    pub rpc_client: Arc<RpcClient>,
    pub edges_pool_to_id: AtomicPtr<HashMap<String, u32>>,
    pub edges: AtomicPtr<HashMap<u32, JupSwapEdge>>,
    // pub grouped_routes: AtomicPtr<HashMap<u32, Vec<(f64, Vec<u32>)>>>,
    // pub manager: AtomicPtr<ArbitrageManager>,
    pub mints: AtomicPtr<Vec<String>>,
    pub mints_base_pairs: AtomicPtr<HashMap<String, Vec<(u32, bool)>>>,
    pub chunk2_routes: AtomicPtr<HashMap<String, Vec<(String, String)>>>,
    pub payer: Keypair,
    pub tip_payer: Keypair,
    pub user: Keypair,
    pub bundle_sender: udp_proxy::BundleSender,
    pub process_pair_cnt: AtomicU8,

    pub test_validator: TestValidator
}
impl Default for JupBot {
    
    fn default() -> Self {
        let rpc_client = Arc::new(RpcClient::new_with_commitment(String::from(RPC_ENDPOINT), CommitmentConfig::processed()));
        Self {
            is_running: AtomicBool::new(false),
            last_blockhash_timestamp: AtomicU64::new(0),
            latest_blockhash: AtomicPtr::new(Box::into_raw(Box::new(None))),
            edges_pool_to_id: AtomicPtr::new(Box::into_raw(Box::new(HashMap::new()))),
            edges: AtomicPtr::new(Box::into_raw(Box::new(HashMap::new()))),
            // grouped_routes: AtomicPtr::new(Box::into_raw(Box::new(HashMap::new()))),
            // manager: AtomicPtr::new(Box::into_raw(Box::new(ArbitrageManager::new()))),
            mints: AtomicPtr::new(Box::into_raw(Box::new(Vec::new()))),
            mints_base_pairs: AtomicPtr::new(Box::into_raw(Box::new(HashMap::new()))),
            chunk2_routes: AtomicPtr::new(Box::into_raw(Box::new(HashMap::new()))),
            payer: signature::read_keypair_file("/mnt/wallet/payer.json").unwrap(),
            tip_payer: signature::read_keypair_file("/mnt/wallet/tip_payer.json").unwrap(),
            bundle_sender: udp_proxy::BundleSender::create(),
            process_pair_cnt: AtomicU8::new(0),
            test_validator: Self::setup_validator(&rpc_client),
            rpc_client,
            user: signature::read_keypair_file("/mnt/wallet/user.json").unwrap()
        }
    }
}


#[derive(Clone, Debug)]
struct ArbitrageRoute {
    route_id: usize,
    profit: f64, // Sorting key
    swap_pairs: Vec<(u32, bool)>, // (Swap pair, is_forward) → true = A->B, false = B->A
}

// Custom ordering for max-heap (higher profit comes first)
impl Ord for ArbitrageRoute {
    fn cmp(&self, other: &Self) -> CmpOrdering {
        other.profit.partial_cmp(&self.profit).unwrap()
    }
}

impl PartialOrd for ArbitrageRoute {
    fn partial_cmp(&self, other: &Self) -> Option<CmpOrdering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ArbitrageRoute {
    fn eq(&self, other: &Self) -> bool {
        self.route_id == other.route_id
    }
}

impl Eq for ArbitrageRoute {}

/// **Indexed Priority Queue (IPQ) for Efficient Updates**
struct IndexedPriorityQueue {
    heap: BinaryHeap<ArbitrageRoute>,
    index_map: HashMap<usize, usize>, // Maps route_id → heap_index
}

impl IndexedPriorityQueue {
    fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            index_map: HashMap::new(),
        }
    }

    fn insert(&mut self, route: ArbitrageRoute) {
        self.index_map.insert(route.route_id, self.heap.len()); // Track index in heap
        self.heap.push(route);
    }

    /// **Update Route Profit Based on Direction**
    fn update_route_profit(&mut self, route_id: usize, rate_log_change: f64) {
        if let Some(&heap_index) = self.index_map.get(&route_id) {
            let mut route_vec: Vec<_> = self.heap.clone().into_sorted_vec();
            if let Some(route) = route_vec.get_mut(heap_index) {
                route.profit += rate_log_change; // Apply price change
            }
            self.heap = BinaryHeap::from(route_vec);
        }
    }

    fn get_top_routes(&self, k: usize) -> Vec<ArbitrageRoute> {
        self.heap.clone().into_sorted_vec().into_iter().take(k).collect()
    }
}
struct ArbitrageManager {
    swap_pair_heaps: HashMap<u32, IndexedPriorityQueue>, // Maps swap pair → IndexedHeap
}

impl ArbitrageManager {
    fn new() -> Self {
        Self {
            swap_pair_heaps: HashMap::new(),
        }
    }

    fn add_route(&mut self, route: ArbitrageRoute) {
        for swap_pair in &route.swap_pairs {
            self.swap_pair_heaps
                .entry(swap_pair.0)
                .or_insert_with(IndexedPriorityQueue::new)
                .insert(route.clone());
        }
    }

    /// **Update a Swap Pair with Directional Multipliers**
    fn update_swap_pair(&mut self, swap_pair: &u32, rate_log_change_ab: f64, rate_log_change_ba: f64) {
        if let Some(heap) = self.swap_pair_heaps.get_mut(swap_pair) {
            for route in heap.heap.clone().into_sorted_vec() {
                let mut new_profit = route.profit;
                for (sp, is_forward) in &route.swap_pairs {
                    if sp == swap_pair {
                        if *is_forward {
                            new_profit += rate_log_change_ab; // Apply A->B multiplier
                        } else {
                            new_profit += rate_log_change_ba; // Apply B->A multiplier
                        }
                    }
                }
                heap.update_route_profit(route.route_id, new_profit - route.profit);
            }
            println!("Updated routes for swap pair: {}", swap_pair);
        }
    }

    fn get_top_routes(&self, swap_pair: &u32, k: usize) -> Vec<ArbitrageRoute> {
        self.swap_pair_heaps
            .get(swap_pair)
            .map(|heap| heap.get_top_routes(k))
            .unwrap_or_else(Vec::new)
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TradeJupSwapParams {
    pub initial_amount: u64,
    pub exp_pnl: u64,
    pub amount_start_index: u32,
    pub percent: u64, // percent of initial_amount
    pub bundle_index: u8, // 0 - first & register, 1 - first & not register, 2 - mid, 3 - last, 4 - last & check pnl
    pub ix_data: Vec<u8>
}
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
    cu_consumed_b_a: u64
}
impl JupSwapEdge {
    
    pub fn update_amount(&mut self, amount: u64, atob: bool) {
        let position = self.amount_index as usize;
        let replace_vec = amount.to_le_bytes().to_vec();
        let buf = if atob {
            &mut self.swap_ix_a_to_b.data[position..]
        } else {
            &mut self.swap_ix_a_to_b.data[position..]
        };
        let len = replace_vec.len().min(buf.len());
        buf[..len].copy_from_slice(&replace_vec[..len]);
    }
    pub fn update_rate_ab(&mut self, jup_bot: &JupBot) {
        let accounts: Option<RpcSimulateTransactionAccountsConfig> = Some(RpcSimulateTransactionAccountsConfig {
            encoding: Some(UiAccountEncoding::JsonParsed),
            addresses: vec![self.ata_b.clone()]
        });
        let price = self.price_a;
        let initial_amount = price * RATE_VALUE;
        let percent = 10000;
        let bundle_index = 0;
        let exp_pnl = 0;
        let ix = jup_bot.get_ix_by_edge(self, true, initial_amount, percent, bundle_index, exp_pnl);
        
        let token_amounts = vec![(self.ata_a.clone(), initial_amount), (self.ata_b.clone(), 0u64)];
        let amounts: Option<RpcSimulateTransactionTokenAmountsConfig> = Some(RpcSimulateTransactionTokenAmountsConfig {
            token_amounts
        });
        let cu_ix = ComputeBudgetInstruction::set_compute_unit_limit(1_400_000);
        let sim_rpc_res = jup_bot.create_and_simulate_ix_with_amounts(&[cu_ix, ix], &[], accounts, amounts);
        if sim_rpc_res.is_ok() {
            let sim_res = sim_rpc_res.unwrap();
            let units_consumed_opt = sim_res.value.units_consumed;
            let units_consumed = if units_consumed_opt.is_some() {units_consumed_opt.unwrap()} else {0u64};
            if sim_res.value.err.is_none() && sim_res.value.accounts.is_some() {
                let in_amount = price;
                let out_amount = JupBot::get_simulated_amount(&sim_res.value.accounts, 0);
                if in_amount > 0 && out_amount > 0 {
                    let rate: u128 = (out_amount as u128).checked_mul(PRICE_MULTIPLIER as u128).unwrap().checked_div(in_amount as u128).unwrap();
                    let rate_log: f64 = ((out_amount as f64) / (in_amount as f64)).ln();
                    self.rate_a_b = rate;
                    self.rate_a_b_log = rate_log;
                    self.cu_consumed_a_b = units_consumed + 10000;
                }
                else {
                    println!("in_amount or out_amount is zero");
                }
            }
            else {
                self.rate_a_b_log = f64::MIN;
                // println!("sim_res {:#?}", sim_res);
                // println!("pool {}", self.pool);
                // println!("pool {:X?}", swap_ix.data.clone());
            }
        }
        else {
            self.rate_a_b_log = f64::MIN;
            // println!("rpc error {:#?}", sim_rpc_res.err());
        }
    }
    pub fn update_rate_ba(&mut self, jup_bot: &JupBot) {
        let addresses = vec![self.ata_a.clone()];
        let accounts: Option<RpcSimulateTransactionAccountsConfig> = Some(RpcSimulateTransactionAccountsConfig {
            encoding: Some(UiAccountEncoding::JsonParsed),
            addresses
        });
        let price = self.price_b;
        let initial_amount = price * RATE_VALUE;
        let percent = 10000;
        let bundle_index = 0;
        let exp_pnl = 0;
        let ix = jup_bot.get_ix_by_edge(self, false, initial_amount, percent, bundle_index, exp_pnl);

        let token_amounts = vec![(self.ata_b.clone(), initial_amount), (self.ata_a.clone(), 0u64)];
        // println!("token_amounts {:#?}", token_amounts);
        let amounts: Option<RpcSimulateTransactionTokenAmountsConfig> = Some(RpcSimulateTransactionTokenAmountsConfig {
            token_amounts
        });
        let cu_ix = ComputeBudgetInstruction::set_compute_unit_limit(1_400_000);
        let sim_rpc_res = jup_bot.create_and_simulate_ix_with_amounts(&[cu_ix, ix], &[], accounts, amounts);
        if sim_rpc_res.is_ok() {
            let sim_res = sim_rpc_res.unwrap();
            let units_consumed_opt = sim_res.value.units_consumed;
            let units_consumed = if units_consumed_opt.is_some() {units_consumed_opt.unwrap()} else {0u64};
            if sim_res.value.err.is_none() && sim_res.value.accounts.is_some() {
                let in_amount = price;
                let out_amount = JupBot::get_simulated_amount(&sim_res.value.accounts, 0);
                if in_amount > 0 && out_amount > 0 {
                    let rate: u128 = (out_amount as u128).checked_mul(PRICE_MULTIPLIER as u128).unwrap().checked_div(in_amount as u128).unwrap();
                    let rate_log: f64 = ((out_amount as f64) / (in_amount as f64)).ln();
                    self.rate_b_a = rate;
                    self.rate_b_a_log = rate_log;
                    self.cu_consumed_b_a = units_consumed + 10000;
                }
                else {
                    println!("in_amount or out_amount is zero");
                }
            }
            else {
                self.rate_b_a_log = f64::MIN;
                // println!("sim_res {:#?}", sim_res);
                // println!("pool {}", self.pool);
                // println!("pool {:X?}", swap_ix.data.clone());
            }
        }
        else {
            self.rate_b_a_log = f64::MIN;
            // println!("rpc error {:#?}", sim_rpc_res.err());
        }
        

    }
    pub fn calc_amount_out_ab(&mut self, jup_bot: &JupBot, value: u64, bank: &Arc<Bank>) -> (u64, u64) {

        let input_amount = self.price_a * value;
        self.update_amount(input_amount, true);

        jup_bot.update_token_amount(&Pubkey::from_str(&self.ata_a).unwrap(), input_amount, bank);
        jup_bot.update_token_amount(&Pubkey::from_str(&self.ata_b).unwrap(), 0, bank);

        let ix = self.swap_ix_a_to_b.clone();
        let sim_res = jup_bot.create_and_simulate_ix_svm(&[ix.clone()], &[], bank);
        if sim_res.result.is_err() {
            println!("error {:#?}", sim_res.result.err());
            println!("error {:#?}", sim_res.logs);

            return (input_amount, 0);
        }
        for post_account in sim_res.post_simulation_accounts.iter() {
            if post_account.0.to_string().eq(&self.ata_b) {
                let token_amount = jup_bot.get_amount_from_token_account(post_account.1.data());
                return (input_amount, token_amount);
            }
        }
        (input_amount, 0)
    }
    pub fn calc_amount_out_ba(&mut self, jup_bot: &JupBot, value: u64, bank: &Arc<Bank>) -> (u64, u64) {

        let input_amount = self.price_b * value;
        self.update_amount(input_amount, false);

        jup_bot.update_token_amount(&Pubkey::from_str(&self.ata_b).unwrap(), input_amount, bank);
        jup_bot.update_token_amount(&Pubkey::from_str(&self.ata_a).unwrap(), 0, bank);

        let ix = self.swap_ix_b_to_a.clone();
        let sim_res = jup_bot.create_and_simulate_ix_svm(&[ix.clone()], &[], bank);
        if sim_res.result.is_err() {
            println!("error {:#?}", sim_res.result.err());
            println!("error {:#?}", sim_res.logs);

            return (input_amount, 0);
        }
        for post_account in sim_res.post_simulation_accounts.iter() {
            if post_account.0.to_string().eq(&self.ata_a) {
                let token_amount = jup_bot.get_amount_from_token_account(post_account.1.data());
                return (input_amount, token_amount);
            }
        }
        (input_amount, 0)
    }
    pub fn generate_inout_table(&mut self, jup_bot: &JupBot) {
        let bank = jup_bot.test_validator
            .bank_forks()
            .as_ref()
            .read()
            .unwrap()
            .working_bank();

        let mut pubkeys = vec![];
        for account_meta in self.swap_ix_a_to_b.accounts.iter() {
            if !pubkeys.contains(&account_meta.pubkey) {
                pubkeys.push(account_meta.pubkey);
            }
        }
        for account_meta in self.swap_ix_b_to_a.accounts.iter() {
            if !pubkeys.contains(&account_meta.pubkey) {
                pubkeys.push(account_meta.pubkey);
            }
        }

        for pubkey in pubkeys.iter() {
            let account_res = jup_bot.rpc_client.get_account(pubkey);
            if account_res.is_ok() {
                let account = account_res.unwrap();
                bank.unfreeze_for_ledger_tool();
                bank.store_account(&pubkey, &AccountSharedData::from(account.clone()));
                // if account.executable {
                //     if let Ok(UpgradeableLoaderState::Program {
                //         programdata_address,
                //     }) = account.deserialize_data()
                //     {
                //         let meta_res = jup_bot.rpc_client.get_account(&programdata_address);
                //         if meta_res.is_ok() {
                //             let meta = meta_res.unwrap();
                //             bank.store_account(&programdata_address, &AccountSharedData::from(meta));
                //         }
                //     }
                // }
            }
        }
        // construct ata
        let mint_a_key = Pubkey::from_str(&self.mint_a).unwrap();
        let mint_a_res = jup_bot.rpc_client.get_account(&mint_a_key);
        if mint_a_res.is_ok() {
            let mint_a = mint_a_res.unwrap();
            let account_data = JupBot::get_token_acount(mint_a_key, jup_bot.user.pubkey(), 0, mint_a.owner);
            bank.unfreeze_for_ledger_tool();
            bank.store_account(&Pubkey::from_str(&self.ata_a).unwrap(), &account_data);
        }
        

        let mint_b_key = Pubkey::from_str(&self.mint_b).unwrap();
        let mint_b_res = jup_bot.rpc_client.get_account(&mint_b_key);
        if mint_b_res.is_ok() {
            let mint_b = mint_b_res.unwrap();
            let account_data = JupBot::get_token_acount(mint_b_key, jup_bot.user.pubkey(), 0, mint_b.owner);
            bank.unfreeze_for_ledger_tool();
            bank.store_account(&Pubkey::from_str(&self.ata_b).unwrap(), &account_data);
        }
        
        
        bank.unfreeze_for_ledger_tool();
        bank.store_account(
            &jup_bot.user.pubkey(),
            &AccountSharedData::new(
                1_000_000_000_000_000,
                0,
                &Pubkey::from_str("11111111111111111111111111111111").unwrap(),
            ),
        );

        let max_usd_to_send = JupBot::get_max_usd();
        let min_usd_to_send = 1u64;
        let step = 3;
        let mut table_ab = vec![(0u64, 0u64)];
        let mut table_ba = vec![(0u64, 0u64)];
        for usd_to_send in min_usd_to_send..(max_usd_to_send/step) {
            // println!("calc_amount_out - {} / {}", usd_to_send, max_usd_to_send);
            let inout_ab = self.calc_amount_out_ab(jup_bot, usd_to_send * step, &bank);
            let inout_ba = self.calc_amount_out_ba(jup_bot, usd_to_send * step, &bank);
            table_ab.push(inout_ab);
            table_ba.push(inout_ba);
        }

        // let serialized_content = serde_json::to_string(&*table_ab).unwrap();
        // println!("saving result to file");
        // let file_path = format!("/mnt/table/{}-ab.json", self.pool);
        // let mut inout_file = OpenOptions::new()
        //     .create(true)
        //     .write(true)
        //     .truncate(true)
        //     .open(file_path)
        //     .expect("error when creating file");
        // inout_file.write_all(serialized_content.as_bytes()).expect("error to write");
        // inout_file.flush().expect("error when fluxh");

        // let serialized_content = serde_json::to_string(&*table_ba).unwrap();
        // println!("saving result to file");
        // let file_path = format!("/mnt/table/{}-ba.json", self.pool);
        // let mut inout_file = OpenOptions::new()
        //     .create(true)
        //     .write(true)
        //     .truncate(true)
        //     .open(file_path)
        //     .expect("error when creating file");
        // inout_file.write_all(serialized_content.as_bytes()).expect("error to write");
        // inout_file.flush().expect("error when fluxh");
        // println!("saved result to file");

    }
    
    pub fn get_trade_jupswap_ix(accounts: &Vec<AccountMeta>, params: &TradeJupSwapParams) -> Instruction {
        // trace!();
        let program_id = Pubkey::from_str("botHDy47CNugroED1sxHfYeAiXwDKbgaa1WUeBvqnej").unwrap();
        let discriminator = Self::sighash("global", "jupswap");

        Instruction::new_with_borsh(program_id, &(discriminator, params), accounts.to_vec())
    }
    pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
        let preimage = format!("{namespace}:{name}");

        let mut sighash = [0u8; 8];
        sighash.copy_from_slice(&crate::hash::hash(preimage.as_bytes()).to_bytes()[..8]);
        sighash
    }
}
impl JupBot {
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
    pub fn generate_inout_table(this: &Arc<JupBot>) {
        let bot_edges = unsafe{&mut * this.edges.load(Ordering::SeqCst)};
        let total_len = bot_edges.len();
        for (index, (_edge_id, edge)) in bot_edges.iter_mut().enumerate() {
            let time = std::time::Instant::now();
            edge.generate_inout_table(this);
            // std::thread::sleep(Duration::from_millis(20000000));
            println!("{} / {} , edge gen time {}ns",index, total_len, time.elapsed().as_nanos());
            
        }
    }
    pub fn setup_validator(rpc_client: &Arc<RpcClient>) -> TestValidator {
        // let mint_keypair = Keypair::new();
        // let mint_pubkey = mint_keypair.pubkey();
        // let faucet_addr = run_local_faucet(mint_keypair, None);
        // let rpc_client = RpcClient::new_with_commitment(String::from(RPC_ENDPOINT), CommitmentConfig::processed());
        let ledger_path = "/mnt/ledger";
        let _ = fs::remove_dir_all(ledger_path);
        let _ = fs::create_dir_all(ledger_path);

        let mut test_validator_genesis = TestValidatorGenesis::default();
        test_validator_genesis.ledger_path(std::path::PathBuf::from_str(ledger_path).unwrap());
        test_validator_genesis.max_genesis_archive_unpacked_size = Some(u64::MAX);
        test_validator_genesis.fee_rate_governor(FeeRateGovernor::new(0, 0));
        test_validator_genesis.rent(Rent {
            lamports_per_byte_year: 1,
            exemption_threshold: 1.0,
            ..Rent::default()
        });
        // test_validator_genesis.faucet_addr(Some(faucet_addr));

        let test_validator_genesis = test_validator_genesis.clone_upgradeable_programs(
            GENESIS_PROGRAMS
                .iter()
                .map(|program| Pubkey::from_str(program).unwrap())
                .into_iter(),
            &rpc_client,
        ).expect("error clone programs");

        let (test_validator, _mint_keypair) = test_validator_genesis
            // .rpc_port(9901)
            .start();
        println!("test validator launched!");
        // let bank_forks = test_validator.bank_forks();
        // let bank = bank_forks.as_ref().read().unwrap().working_bank();
        // println!("Loaded Programs:");
        // println!("Program: {} | loaded: {}", "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4", bank.get_account(&Pubkey::from_str("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4").unwrap()).is_some());
        test_validator
    }
    pub fn setup(&self) {
        self.setup_files();
    }
    pub fn setup_files(&self) {
        // reading edges
        let edges_file_name = format!("/mnt/edges-solfi.json");
        // let edges_file_name = format!("/root/project/pairs_2024_12_05/jupEdges.json");
        let edges_data = fs::read_to_string(edges_file_name).expect("Failed to read JSON file");
        let edges: Vec<JupSwapEdge> = serde_json::from_str(&edges_data).expect("Failed to parse JSON");
        let bot_edges = unsafe{&mut * self.edges.load(Ordering::SeqCst)};
        let edges_pool_to_id = unsafe{&mut * self.edges_pool_to_id.load(Ordering::SeqCst)};
        let mints = unsafe{&mut * self.mints.load(Ordering::SeqCst)};
        let mints_base_pairs = unsafe{&mut * self.mints_base_pairs.load(Ordering::SeqCst)};
        for edge in edges.iter() {
            bot_edges.insert(edge.id, edge.clone());
            edges_pool_to_id.insert(edge.pool.clone(), edge.id);
            if !mints.contains(&edge.mint_a) {
                mints.push(edge.mint_a.clone());
            }
            if !mints.contains(&edge.mint_b) {
                mints.push(edge.mint_b.clone());
            }
            let key_ab = edge.mint_a.clone() + &edge.mint_b;
            let key_ba = edge.mint_b.clone() + &edge.mint_a;
            if mints_base_pairs.contains_key(&key_ab) {
                mints_base_pairs.get_mut(&key_ab).unwrap().push((edge.id, true));
            }
            else {
                mints_base_pairs.insert(key_ab, vec![(edge.id, true)]);
            }
            if mints_base_pairs.contains_key(&key_ba) {
                mints_base_pairs.get_mut(&key_ba).unwrap().push((edge.id, false));
            }
            else {
                mints_base_pairs.insert(key_ba, vec![(edge.id, false)]);
            }
        }
        println!("edges loaded! mints_base_pairs len = {}, mints len = {}", mints_base_pairs.len(), mints.len());

        /*
        // construct chunk routes
        let chunk2_routes = unsafe{&mut * self.chunk2_routes.load(Ordering::SeqCst)};
        let mints_len = mints.len();
        for (index, mint_1) in mints.iter().enumerate() {
            println!("making chunk2_routes for mint: {}, {} / {} ", mint_1, index, mints_len);
            for mint_3 in mints.iter() {
                for mint_2 in mints.iter() {
                    if (mint_1.eq(mint_2) && mint_1.eq(mint_3)) ||
                        (mint_1.eq(mint_2) && mint_2.eq(mint_3)) || 
                        mint_2.eq("So11111111111111111111111111111111111111112") || 
                        (mint_1.eq(mint_3) && mint_1.eq("So11111111111111111111111111111111111111112")){
                            continue;
                    }
                    let key = mint_1.clone() + mint_3;
                    let key_left = mint_1.clone() + mint_2;
                    let key_right = mint_2.clone() + mint_3;
                    if  mints_base_pairs.contains_key(&key_left) && mints_base_pairs.contains_key(&key_right) {
                        if chunk2_routes.contains_key(&key) {
                            chunk2_routes.get_mut(&key).unwrap().push((key_left, key_right));
                        }
                        else {
                            chunk2_routes.insert(key, vec![(key_left, key_right)]);
                        }
                    }
                    
                }
            }
        }

        println!("chunk2_routes done! chunk2_routes len = {}", chunk2_routes.len());
        */

         self.update_latest_blockhash_once();

         /*
         // update all rates of edges
         let edges_len = bot_edges.len();
        for (index, bot_edge) in bot_edges.iter_mut() {
            println!("updating edge rates {} / {}", index, edges_len);

            let (_, elapsed_ab) = measure_us!(bot_edge.update_rate_ab(self));
            let new_rate_log_ab = bot_edge.rate_a_b_log;

            let (_, elapsed_ba) = measure_us!(bot_edge.update_rate_ba(self));
            let new_rate_log_ba = bot_edge.rate_b_a_log;

            println!("rate changed {}, {}", new_rate_log_ab, new_rate_log_ba);

            // let (_, elapsed_routes) = measure_us!(manager.update_swap_pair(&bot_edge.id, new_rate_log_ab, new_rate_log_ba));
            let elapsed_routes = 0;
            println!("setup: pair's routes updated. {}ms delayed, pool = {}", (elapsed_ab + elapsed_ba + elapsed_routes) / 1000, bot_edge.pool);

        }
        println!("all rates updated!");
         */

        self.is_running.store(true, Ordering::SeqCst);
    }
    pub fn get_max_usd() -> u64 {
        3000u64
    }
    pub fn get_amount_from_token_account(&self, token_account_info: &[u8]) -> u64 {
        let input = array_ref![token_account_info, 0, 72];
        let (_, amount_slice) = array_refs![input, 64, 8];

        let vault_amount = u64::from_le_bytes(*amount_slice);

        vault_amount
    }
    pub fn update_token_amount(&self, key: &Pubkey, amount: u64, bank: &Arc<Bank>) {
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
    pub fn bank(&self) -> Arc<Bank> {
        let bank_forks = self.test_validator.bank_forks();
        let new_bank = bank_forks.as_ref().read().unwrap().working_bank();
        // let new_slot = bank.slot()+1;
        // let new_bank = Arc::new(Bank::new_from_parent(bank, &Pubkey::default(), new_slot));
        new_bank.unfreeze_for_ledger_tool();
        while new_bank.freeze_started() {
            let ten_millis = std::time::Duration::from_millis(10);
            std::thread::sleep(ten_millis);
        }
        new_bank
    }
    pub fn create_and_simulate_ix_svm(
        &self,
        txInstructions: &[Instruction],
        lookuptables: &[AddressLookupTableAccount],
        bank: &Arc<Bank>,
    ) -> TransactionSimulationResult {
        let signers = vec![&self.user];
        // let bank = Self::bank(instance);
        let blockhash = bank.last_blockhash();

        // println!("lookuptables: {:#?}", lookuptables.iter().map(|l| l.key).collect::<Vec<_>>());

        let versionedMessage = V0(solana_sdk::message::v0::Message::try_compile(
            &self.user.pubkey(),
            txInstructions,
            lookuptables,
            blockhash,
        )
        .unwrap());
        let tx = VersionedTransaction::try_new(versionedMessage, &signers).unwrap();
        Self::simulate_transaction_svm(tx, bank.clone())
    }
    fn sanitize_transaction(
        transaction: VersionedTransaction,
        address_loader: impl AddressLoader,
    ) -> RuntimeTransaction<SanitizedTransaction> {
        
        RuntimeTransaction::try_create(transaction, MessageHash::Compute, None, address_loader, &HashSet::new())
            .map_err(|err| println!("invalid transaction: {err}"))
            .unwrap()
    }
    pub fn simulate_transaction_svm(
        transaction: VersionedTransaction,
        bank: Arc<Bank>,
    ) -> TransactionSimulationResult {
        let sanitized_tx = Self::sanitize_transaction(transaction, &*bank);

        // let start = Instant::now();
        let tx_res = bank.simulate_transaction_unchecked(&sanitized_tx, false);
        // let elapsed = start.elapsed().as_nanos();
        // println!("svm elapsed {}ns", elapsed);

        tx_res
    }
    pub fn run_top_routes(this: &Arc<JupBot>, edge_id: u32, atob: bool) ->bool {
        let mut bundle_sent = false;
        let initial_route = this.get_top_route(edge_id, atob);
        // let expanded_route = this.expand_route_all(edge_id, &initial_route);
        // let expanded_route = this.expand_route_all(edge_id, &expanded_route);
        let top_route = initial_route;

        // let print_str1 = initial_route.iter().map(|a|a.0.to_string()).collect::<Vec<String>>().join("->");
        // let print_str2 = top_route.iter().map(|a|a.0.to_string()).collect::<Vec<String>>().join("->");
        // println!("expanded_route len {}", expanded_route.len());
        // return true;

        let rate_log = this.get_route_rate(&top_route);

        if rate_log > 0.0001f64 && top_route.len() > 2 {
            // println!("rate_log: {}", rate_log);
            
            let initial_amount = 1000000u64;
            let percent = 10000u64;
            let exp_pnl = 0u64;

            let ixs = this.get_versioned_txs_by_route(&top_route, initial_amount, percent, exp_pnl, false, vec![]);
            // return true;
            if ixs.len() <= 5 && ixs.len() >= 2 {
                // determine initial_amount and exp_pnl
                let min_input = 400_000u64;
                let max_input = 20_000_000_000u64;

                let mut prev_pnl = 0u64;
                let mut min_amount = min_input;
                let mut bound_amount = min_input.checked_mul(10).unwrap();
                let mut cus: Vec<u64> = vec![];
                let sim_result: Arc<AtomicPtr<Vec<(u64, Vec<u64>, u64)>>> = Arc::new(AtomicPtr::new(Box::into_raw(Box::new(Vec::new()))));
                /*
                let mut thread_handles = Vec::new();
                loop {
                    let this_clone = this.clone();
                    let first_sim_result_clone = sim_result.clone();
                    let top_route_clone = top_route.clone();
                    let handle = std::thread::spawn(move || {
                        let first_sim_result_vec = unsafe{&mut * first_sim_result_clone.load(Ordering::SeqCst)};
                        let result = this_clone.get_simulated_pnl(&top_route_clone, bound_amount, percent, exp_pnl);
                        first_sim_result_vec.push((result.0, result.1, bound_amount));
                    });
                    thread_handles.push(handle);

                    // println!("1loop sim_pnl {}", sim_pnl);
                    if bound_amount >= max_input {
                        break;
                    }
                    
                    bound_amount = bound_amount.checked_mul(10).unwrap();
                }
                for handle in thread_handles {
                    handle.join().expect("getting pnl thread1 error");
                }
                let sim_result_vec = unsafe{&mut * sim_result.load(Ordering::SeqCst)};
                let (sim_pnl, sim_cus, bound_amount_max) = sim_result_vec.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap();
                cus = sim_cus.clone();
                prev_pnl = *sim_pnl;
                bound_amount = *bound_amount_max;
                min_amount = bound_amount;

                let ignore_unit = 100;
                let allowed_amount_range = if min_amount.checked_div(ignore_unit).unwrap() == 0 {
                    ALLOWED_AMOUNT_CHANGE
                } else {
                    min_amount.checked_div(ignore_unit).unwrap()
                };

                let mut max_amount = if bound_amount >= max_input {
                    max_input
                } else {
                    bound_amount
                };
                let mut mid_amount: u64;
                prev_pnl = 0;

                let sim_result: Arc<AtomicPtr<Vec<(u64, Vec<u64>, u64)>>> = Arc::new(AtomicPtr::new(Box::into_raw(Box::new(Vec::new()))));
                let mut thread_handles = Vec::new();

                let mut loop_i = 0;
                loop {
                    mid_amount = min_amount + (max_amount - min_amount) * loop_i / ignore_unit;

                    let this_clone = this.clone();
                    let sim_result_clone = sim_result.clone();
                    let top_route_clone = top_route.clone();
                    let handle = std::thread::spawn(move || {
                        let sim_result_vec_thread = unsafe{&mut * sim_result_clone.load(Ordering::SeqCst)};
                        let result = this_clone.get_simulated_pnl(&top_route_clone, mid_amount, percent, exp_pnl);
                        sim_result_vec_thread.push((result.0, result.1, mid_amount));
                    });
                    thread_handles.push(handle);

                    loop_i += 1;

                    if loop_i >= 100 {
                        break;
                    }
                }
                for handle in thread_handles {
                    handle.join().expect("getting pnl thread2 error");
                }
                let sim_result_vec = unsafe{&mut * sim_result.load(Ordering::SeqCst)};
                let (sim_pnl, sim_cus, mid_amount_max) = sim_result_vec.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap();
                cus = sim_cus.clone();
                prev_pnl = *sim_pnl;
                mid_amount = *mid_amount_max;
                min_amount = mid_amount;
                */
                // let determined_input = min_amount;
                // let exp_pnl = prev_pnl * 85 / 100;
                let determined_input = 100_000_000;
                let exp_pnl = 100_000;
                
                // return;
                if exp_pnl > 10000 {
                    // println!("determined {}, exp pnl {}, cus {:#?}", determined_input, exp_pnl, cus);
                    let final_ixs = this.get_versioned_txs_by_route(&top_route, determined_input, percent, exp_pnl, true, cus);
                
                    //send bundle
                    this.bundle_sender.send_bundle(&final_ixs, true);
                    bundle_sent = true;
                    println!("bundle sent! route: {}, sim_pnl {}, input {}, rate_log {}", top_route.iter().map(|a|a.0.to_string()).collect::<Vec<String>>().join("->"), exp_pnl as f64 / 1_000_000_000f64, determined_input as f64 / 1_000_000_000f64, rate_log);
                }
                
            }
            else {
                println!("bundle txs count is between 2 and 5! current count = {}", ixs.len());
            }
            

        }
        bundle_sent
    }
    pub fn get_simulated_pnl(&self, top_route: &Vec<(u32, bool)>, in_amount: u64, percent: u64, exp_pnl: u64) -> (u64, Vec<u64>) {
        let mut old_amount = 0u64;
        let mut new_amount = 0u64;
        let mut cus: Vec<u64> = vec![];
        let cur_ixs = self.get_versioned_txs_by_route(top_route, in_amount, percent, exp_pnl, false, vec![]);
        let sim_bundle_res = self.simulate_bundle(&cur_ixs, "Gm6TTgZXjATYcpBxkmJnCZbzeeoJ7TMJKwDLMaLFSpE1");
        if sim_bundle_res.is_ok() {
            let sim_bundle = sim_bundle_res.unwrap();
            let tx_results = &sim_bundle.value.transaction_results;
            
            // println!("sim_bundle.value.summary {:#?}", sim_bundle.value.summary);
            if tx_results.len() == 0 {
                // println!("tx_results.len() == 0 ");
                return (0u64, cus);
            }
            let pre_execution_accounts_res = tx_results.get(0).unwrap().pre_execution_accounts.as_ref().unwrap();
            let post_execution_accounts_res = tx_results.get(tx_results.len() - 1).unwrap().post_execution_accounts.as_ref().unwrap();
            let pre_account_data = &pre_execution_accounts_res[0].data.decode().unwrap();
            let post_account_data = &post_execution_accounts_res[0].data.decode().unwrap();
            old_amount = u64::from_le_bytes(pre_account_data[64..72].try_into().unwrap());
            new_amount = u64::from_le_bytes(post_account_data[64..72].try_into().unwrap());
            cus = tx_results.iter().map(|f| f.units_consumed.unwrap()).collect();
        }
        else {
            // println!("sim_bundle_res err {:#?}", sim_bundle_res.err());
        }
        let mut sim_pnl = 0u64;
        if new_amount > old_amount {
            sim_pnl = new_amount - old_amount;
        }
        (sim_pnl, cus)
    }
    pub fn simulate_bundle(&self, versioned_txs: &[VersionedTransaction], measure_address: &str) -> RpcResult<RpcSimulateBundleResult> {
        let mut configs = vec![];
        let len = versioned_txs.len();
        for _i in 0..len {
            configs.push(Some(RpcSimulateTransactionAccountsConfig{
                encoding: Some(UiAccountEncoding::JsonParsed),
                addresses: vec![measure_address.to_string()]
            }));
        }
        self.rpc_client.simulate_bundle_with_config(&versioned_txs, RpcSimulateBundleConfig {
            skip_sig_verify: true, 
            replace_recent_blockhash: false,
            pre_execution_accounts_configs: configs.clone(),
            post_execution_accounts_configs: configs.clone(),
            ..RpcSimulateBundleConfig::default()
        })
    }
    pub fn get_versioned_txs_by_route(&self, route: &Vec<(u32, bool)>, initial_amount: u64, percent: u64, exp_pnl: u64, is_tip: bool, cus: Vec<u64>) -> Vec<VersionedTransaction> {
        let bot_edges = unsafe{&mut * self.edges.load(Ordering::SeqCst)};
        let mut versioned_ixs: Vec<VersionedTransaction> = Vec::new();
        for (index, pair) in route.iter().enumerate() {
            let edge = bot_edges.get(&pair.0).unwrap();
            let bundle_index = if index == 0 {
                0 // first & register
            } else if index == route.len() - 1 {
                4 // last & check pnl
            } else {
                2 // mid
            };
            let mut ixs = Vec::new();
            // let mut cu = if pair.1 {edge.cu_consumed_a_b} else {edge.cu_consumed_b_a};
            // if is_tip && bundle_index == 4{
            //     cu += 500;
            // }
            let mut cu = if (cus.len() > 1 && cus.len() - 1 > index) {cus[index]} else {1_400_000};
            cu += 1000;
            let cu_ix = ComputeBudgetInstruction::set_compute_unit_limit(cu as u32);
            ixs.push(cu_ix);
            let ix = self.get_ix_by_edge(edge, pair.1, initial_amount, percent, bundle_index, exp_pnl);
            ixs.push(ix);

            if is_tip && bundle_index == 4 {
                let tip_percent = 7500;
                let tip_amount = exp_pnl.checked_mul(tip_percent).unwrap().checked_div(FEE_MULTIPLIER as u64).unwrap();
                let jito_tip_static_ix = transfer(
                    &self.payer.pubkey(), 
                    &Pubkey::from_str(JITO_TIP_ACCOUNTS[5]).unwrap(), 
                    tip_amount
                );
                ixs.push(jito_tip_static_ix);
            }
            let versioned_swap_tx = self.get_versioned_tx(&ixs, &[]).expect("versioned_swap_tx error");
            versioned_ixs.push(versioned_swap_tx);
        }
        versioned_ixs
    }
    pub fn get_versioned_tx(
        &self,
        tx_instructions: &[Instruction],
        lookuptables: &[AddressLookupTableAccount],
    ) -> std::result::Result<VersionedTransaction, SignerError> {
        let signers = vec![&self.payer];
        let blockhash = unsafe{ &* self.latest_blockhash.load(Ordering::SeqCst)};

        let versioned_message = V0(solana_sdk::message::v0::Message::try_compile(
            &self.payer.pubkey(),
            tx_instructions,
            lookuptables,
            blockhash.unwrap(),
        )
        .unwrap());
        VersionedTransaction::try_new(versioned_message, &signers)
    }
    pub fn replace_exp_pnl(instruction: &mut Instruction, exp_pnl: u64) {
        let position = 16usize;
        let replace_vec = exp_pnl.to_le_bytes().to_vec();
        let buf = &mut instruction.data[position..];
        let len = replace_vec.len().min(buf.len());
        buf[..len].copy_from_slice(&replace_vec[..len]);
    }
    pub fn replace_initial_amount(instruction: &mut Instruction, new_amount: u64) {
        let position = 8usize;
        let replace_vec = new_amount.to_le_bytes().to_vec();
        let buf = &mut instruction.data[position..];
        let len = replace_vec.len().min(buf.len());
        buf[..len].copy_from_slice(&replace_vec[..len]);
    }
    pub fn get_ix_by_edge(&self, edge: &JupSwapEdge, atob: bool, initial_amount: u64, percent: u64, bundle_index: u8, exp_pnl: u64) -> Instruction {
        let start_mint = if atob {Pubkey::from_str(&edge.mint_a).unwrap()} else {Pubkey::from_str(&edge.mint_b).unwrap()};
        let ata = if atob {Pubkey::from_str(&edge.ata_a).unwrap()} else {Pubkey::from_str(&edge.ata_b).unwrap()};
        
        let swap_ix = if atob {&edge.swap_ix_a_to_b} else {&edge.swap_ix_b_to_a};
        let mut main_acc_meta = vec![
            AccountMeta {
                pubkey: Pubkey::from_str("HAw3XK6uRMXPrjvLjjZFW2PQRKDGJPQhxdxBL3wFrUyu").unwrap(), // signer
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: Pubkey::from_str("6ZjmH3cRhwNcLk87ovRLnXYpWMEedhcYqKuuxCscVND4").unwrap(), // trade
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: Pubkey::from_str("D6k1znFSoG8Am73BBeY1JLpiDfQyqefTmdXoHQnZef7d").unwrap(), // trade_authority
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: start_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: ata, // start mint ata
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: Pubkey::from_str("Gm6TTgZXjATYcpBxkmJnCZbzeeoJ7TMJKwDLMaLFSpE1").unwrap(), // wsol ata
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: swap_ix.program_id.clone(), // jupiter program
                is_signer: false,
                is_writable: false,
            },
        ];
        
        main_acc_meta.extend(swap_ix.accounts.clone().into_iter());
        
        let ix = JupSwapEdge::get_trade_jupswap_ix(
            &main_acc_meta,
            &TradeJupSwapParams {
                initial_amount,
                exp_pnl,
                amount_start_index: edge.amount_index,
                percent,
                bundle_index,
                ix_data: swap_ix.data.clone(),
            },
        );
        ix
    }
    pub fn get_route_rate(&self, route: &Vec<(u32, bool)>) -> f64 {
        let bot_edges = unsafe{&mut * self.edges.load(Ordering::SeqCst)};
        let mut rate_log_sum = 0f64;
        // let mut result = "".to_string();
        for pair in route.iter() {
            let edge = bot_edges.get_mut(&pair.0).unwrap();
            if pair.1 {
                rate_log_sum += edge.rate_a_b_log;
                // result = result +  &edge.mint_a[0..5] + ", " + &edge.mint_b[0..5] + " -> ";
            } else {
                rate_log_sum += edge.rate_b_a_log;
                // result = result + &edge.mint_b[0..5] + ", " + &edge.mint_a[0..5] + " -> ";
            }
        }
        // println!("sum: {}, path: {}", rate_log_sum, result);
        rate_log_sum
    }
    pub fn get_top_route(&self, edge_id: u32, atob: bool) -> Vec<(u32, bool)> {
        let bot_edges = unsafe{&mut * self.edges.load(Ordering::SeqCst)};
        let mut cur_route: Vec<(u32, bool)> = Vec::new();
        if  bot_edges.contains_key(&edge_id) {
            let bot_edge = bot_edges.get_mut(&edge_id).unwrap();
            cur_route.push((edge_id, atob));

            // expand to left for sol
            self.expand_route_to_sol(&mut cur_route, edge_id, bot_edge, true, atob);

            // expand to right for sol
            self.expand_route_to_sol(&mut cur_route, edge_id, bot_edge, false, atob);

            
        }
        cur_route
    }
    pub fn expand_route_all(&self, edge_id: u32, route: &Vec<(u32, bool)>) -> Vec<(u32, bool)> {
        let bot_edges = unsafe{&mut * self.edges.load(Ordering::SeqCst)};
        let mut except_edge_ids: Vec<u32> = route.iter().map(|pair| pair.0).collect();
        let mut new_route: Vec<(u32, bool)> = Vec::new();
        for (index, pair) in route.iter().enumerate() {
            if pair.0.eq(&edge_id) {
                new_route.push(pair.clone());
                continue;
            }
            let edge = bot_edges.get(&pair.0).unwrap();
            let edge_rate = if pair.1 {edge.rate_a_b_log} else {edge.rate_b_a_log};
            let mint_from = if pair.1 {&edge.mint_a} else {&edge.mint_b};
            let mint_to = if pair.1 {&edge.mint_b} else {&edge.mint_a};

            let (expand_rate, expand_left, expand_right) = self.expand_edge(mint_from, mint_to, &except_edge_ids);
            if expand_rate != f64::MIN && expand_rate > edge_rate {
                except_edge_ids.push(expand_left.0);
                except_edge_ids.push(expand_right.0);
                new_route.push(expand_left);
                new_route.push(expand_right);
            }
            else {
                new_route.push(pair.clone());
            }
            if index < route.len() - 1 {
                let (expand_rate, expand_left, expand_right) = self.expand_edge(mint_to, mint_to, &except_edge_ids);
                if expand_rate != f64::MIN && expand_rate > edge_rate {
                    except_edge_ids.push(expand_left.0);
                    except_edge_ids.push(expand_right.0);
                    new_route.push(expand_left);
                    new_route.push(expand_right);
                }
            }
        }
        new_route
    }
    pub fn expand_edge(&self, mint_from: &String, mint_to: &String, except_edge_ids: &Vec<u32>) -> (f64, (u32, bool), (u32, bool)) {
        let bot_edges = unsafe{&mut * self.edges.load(Ordering::SeqCst)};
        let chunk2_routes = unsafe{&mut * self.chunk2_routes.load(Ordering::SeqCst)};
        let mints_base_pairs = unsafe{&mut * self.mints_base_pairs.load(Ordering::SeqCst)};
        let chunk2_key = mint_from.clone() + mint_to;
        if chunk2_routes.contains_key(&chunk2_key) {
            let chunk2_pairs = chunk2_routes.get(&chunk2_key).unwrap();
            let rate_logs: Vec<(f64, (u32, bool), (u32, bool))> = chunk2_pairs.iter().map(|chunk2_pair| {
                let pairs_left = mints_base_pairs.get(&chunk2_pair.0).unwrap();
                let pairs_right = mints_base_pairs.get(&chunk2_pair.1).unwrap();
                let max_left = pairs_left.iter().max_by(|x, y| {
                    let edge_x = bot_edges.get(&x.0).unwrap();
                    let edge_y = bot_edges.get(&y.0).unwrap();
                    let rate_log_x = if x.1 {edge_x.rate_a_b_log} else {edge_x.rate_b_a_log};
                    let rate_log_y = if y.1 {edge_y.rate_a_b_log} else {edge_y.rate_b_a_log};
                    rate_log_x.partial_cmp(&rate_log_y).unwrap()
                }).unwrap();

                let max_right = pairs_right.iter().max_by(|x, y| {
                    let edge_x = bot_edges.get(&x.0).unwrap();
                    let edge_y = bot_edges.get(&y.0).unwrap();
                    let rate_log_x = if x.1 {edge_x.rate_a_b_log} else {edge_x.rate_b_a_log};
                    let rate_log_y = if y.1 {edge_y.rate_a_b_log} else {edge_y.rate_b_a_log};
                    rate_log_x.partial_cmp(&rate_log_y).unwrap()
                }).unwrap();

                let max_left_edge = bot_edges.get(&max_left.0).unwrap();
                let max_right_edge = bot_edges.get(&max_right.0).unwrap();
                let max_left_rate = if max_left.1 {max_left_edge.rate_a_b_log} else {max_left_edge.rate_b_a_log};
                let max_right_rate = if max_right.1 {max_right_edge.rate_a_b_log} else {max_right_edge.rate_b_a_log};
                (max_left_rate + max_right_rate, max_left.clone(), max_right.clone())

            }).collect();
            let max_expand = rate_logs.iter().max_by(|x, y| {
                x.0.partial_cmp(&y.0).unwrap()
            }).unwrap();
            if !except_edge_ids.contains(&max_expand.1.0) &&  !except_edge_ids.contains(&max_expand.2.0) {
                return max_expand.clone();
            }
            
        }
        (f64::MIN, (0, true), (0, true))
    }
    pub fn expand_route_to_sol(&self, cur_route: &mut Vec<(u32, bool)>, edge_id: u32, bot_edge: &JupSwapEdge, is_left: bool, a_to_b: bool) {
        let bot_edges = unsafe{&mut * self.edges.load(Ordering::SeqCst)};
        let mints_base_pairs = unsafe{&mut * self.mints_base_pairs.load(Ordering::SeqCst)};
        let mut max_pair = &(0u32, true);
        let mut max_rate_log = f64::MIN;
        let mut mints_base_key;
        if (is_left && a_to_b && bot_edge.mint_a.ne(WSOL_MINT)) || (is_left && !a_to_b && bot_edge.mint_b.ne(WSOL_MINT)) {
            mints_base_key = WSOL_MINT.to_string() + if a_to_b {&bot_edge.mint_a} else {&bot_edge.mint_b};
        } else if (!is_left && a_to_b && bot_edge.mint_b.ne(WSOL_MINT)) || (!is_left && !a_to_b && bot_edge.mint_a.ne(WSOL_MINT)) {
            mints_base_key = if a_to_b {bot_edge.mint_b.clone()} else {bot_edge.mint_a.clone()} + WSOL_MINT;
        } else {
            return;
        }

        let pairs_opt = mints_base_pairs.get(&mints_base_key);
        if pairs_opt.is_some(){
            let pairs = pairs_opt.unwrap();
            if pairs.len() > 0 {
                for pair in pairs {
                    if pair.0 != edge_id {
                        let cur_edge_opt = bot_edges.get(&pair.0);
                        if cur_edge_opt.is_some() {
                            let cur_edge = cur_edge_opt.unwrap();
                            let cur_rate = if pair.1 {cur_edge.rate_a_b_log} else {cur_edge.rate_b_a_log};
                            if cur_rate > max_rate_log {
                                max_rate_log = cur_rate;
                                max_pair = pair;
                            }
                        }
                    }
                }
                
            }
            
        }

        if max_rate_log > f64::MIN {
            if is_left {
                cur_route.insert(0, max_pair.clone());
            }
            else {
                cur_route.push(max_pair.clone());
            }
        }
    }
    pub fn get_current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
    pub fn update_latest_blockhash(&self) {
        loop {
            let is_running = self.is_running.load(Ordering::SeqCst);
            if is_running {
                self.update_latest_blockhash_once();
            }
            
            std::thread::sleep(Duration::from_millis(200));
        }
    }
    pub fn update_latest_blockhash_once(&self) {
        let max_retry = 5;
        let mut retry = 0;
        let last_blockhash_timestamp = self.last_blockhash_timestamp.load(Ordering::SeqCst);

        if last_blockhash_timestamp + BLOCKHASH_UPDATE_PERIOD < Self::get_current_timestamp() || last_blockhash_timestamp == 0
        {
            let mut hash_tmp: Option<Hash> = None;
            loop {
                if let Ok(hash) = self.rpc_client.get_latest_blockhash() {
                    hash_tmp = Some(hash)
                };
                if hash_tmp.is_some() {
                    break;
                }
                retry += 1;
                if retry > max_retry {
                    break;
                }
            }

            if hash_tmp.is_some() {
                self.last_blockhash_timestamp.store(Self::get_current_timestamp(), Ordering::SeqCst);
                self.latest_blockhash.store(Box::into_raw(Box::new(hash_tmp)), Ordering::SeqCst);
                // println!("latest_blockhash {:#?}", hash_tmp);
            }
        }
    }
    pub fn create_and_simulate_ix(
        &self,
        tx_instructions: &[Instruction],
        lookuptables: &[AddressLookupTableAccount],
        accounts: Option<RpcSimulateTransactionAccountsConfig>,
    ) -> RpcResult<RpcSimulateTransactionResult> {
        let payer = &self.payer;
        let signers = vec![payer];
        let blockhash = unsafe{&* self.latest_blockhash.load(Ordering::SeqCst)};
        

        let versioned_message = V0(solana_sdk::message::v0::Message::try_compile(
            &self.payer.pubkey(),
            tx_instructions,
            lookuptables,
            blockhash.unwrap(),
        )
        .unwrap());

        let tx = solana_sdk::transaction::VersionedTransaction::try_new(versioned_message, &signers)
            .unwrap();
        self.rpc_client.simulate_transaction_with_config(
            &tx,
            RpcSimulateTransactionConfig {
                sig_verify: false,
                replace_recent_blockhash: false,
                commitment: Some(CommitmentConfig::processed()),
                inner_instructions: true,
                accounts,
                ..RpcSimulateTransactionConfig::default()
            },
        )
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
        let blockhash = unsafe{&* self.latest_blockhash.load(Ordering::SeqCst)};
        

        let versioned_message = V0(solana_sdk::message::v0::Message::try_compile(
            &self.payer.pubkey(),
            tx_instructions,
            lookuptables,
            blockhash.unwrap(),
        )
        .unwrap());

        let tx = solana_sdk::transaction::VersionedTransaction::try_new(versioned_message, &signers)
            .unwrap();
        self.rpc_client.simulate_transaction_with_config(
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
    pub fn account_update(this: &Arc<JupBot>, pubkey: &Pubkey, rate_only: bool) { 
        let bot_edges = unsafe{&mut * this.edges.load(Ordering::SeqCst)};
        let edges_pool_to_id = unsafe{&* this.edges_pool_to_id.load(Ordering::SeqCst)};

        let is_running = this.is_running.load(Ordering::SeqCst);
        let last_blockhash_timestamp = this.last_blockhash_timestamp.load(Ordering::SeqCst);

        // let manager = unsafe{&mut * self.manager.load(Ordering::SeqCst)};

        if is_running && last_blockhash_timestamp > 0 {
            if let Some(edge_id) = edges_pool_to_id.get(&pubkey.to_string()) {
                if  bot_edges.contains_key(edge_id) {
                    // println!("{} edge updated!", *edge_id);
                    let bot_edge = bot_edges.get_mut(edge_id).unwrap();
                    // let (_, elapsed) = measure_us!(edge.update_rate(self, true));
                    // println!("elapsed update_rate {}ns", elapsed);
                    
                    // println!("updating pool {:#?}", bot_edge.pool);

                    let old_rate_log_ab = bot_edge.rate_a_b_log;
                    let (_, elapsed_ab) = measure_us!(bot_edge.update_rate_ab(this));
                    let new_rate_log_ab = bot_edge.rate_a_b_log;
                    let rate_log_change_ab = new_rate_log_ab - old_rate_log_ab;

                    let old_rate_log_ba = bot_edge.rate_b_a_log;
                    let (_, elapsed_ba) = measure_us!(bot_edge.update_rate_ba(this));
                    let new_rate_log_ba = bot_edge.rate_b_a_log;
                    let rate_log_change_ba = new_rate_log_ba - old_rate_log_ba;
                    
                    if !rate_only && (rate_log_change_ab != 0f64 || rate_log_change_ba != 0f64) {
                        let atob = rate_log_change_ab > 0f64 || rate_log_change_ba < 0f64;

                        let process_cnt = this.process_pair_cnt.load(Ordering::SeqCst);
                        if process_cnt < 1 {
                            let rpc_health = this.rpc_client.get_health();
                            if rpc_health.is_ok() {
                                this.process_pair_cnt.store(process_cnt + 1, Ordering::SeqCst);
                                let this_clone = this.clone();
                                std::thread::spawn(move || {
                                    let (bundle_sent, elapsed) = measure_us!(JupBot::run_top_routes(&this_clone, *edge_id, atob));
                                    let process_cnt = this_clone.process_pair_cnt.load(Ordering::SeqCst);
                                    if  process_cnt > 0 {
                                        this_clone.process_pair_cnt.store(process_cnt - 1, Ordering::SeqCst);
                                    }
                                    if bundle_sent {
                                        println!("{}ms, {}ns , ab:{}ns, ba:{}ns, elapsed!", elapsed / 1000, elapsed, elapsed_ab, elapsed_ba);
                                    }
                                    
                                });

                            }
                            else {
                                println!("rpc_health error");
                            }
                        }
                        
                    }

                    // println!("account_update: rate changed {}, {}", rate_log_change_ab, rate_log_change_ba);

                    // let (_, elapsed_routes) = measure_us!(manager.update_swap_pair(&bot_edge.id, rate_log_change_ab, rate_log_change_ba));
                    // let elapsed_routes = 0;
                    // println!("account_update: pair's routes updated. {}ms delayed, pool = {}", (_elapsed_ab + _elapsed_ba + elapsed_routes) / 1000, bot_edge.pool);
                }
                
            }
        }
    }
    pub fn run_iterate_edges(this: &Arc<JupBot>) {
        let bot_edges = unsafe{&mut * this.edges.load(Ordering::SeqCst)};
        let edge_ids: Vec<&u32> = bot_edges.keys().collect();
        let mut iters = 0;
        loop {
            for edge_id in edge_ids.iter() {
                JupBot::edge_update(this, **edge_id, true);
                JupBot::edge_update(this, **edge_id, false);
                // let ten_millis = std::time::Duration::from_millis(100);
                // std::thread::sleep(ten_millis);
            }
            iters += 1;
            println!("iterated {}!", iters);
        }
    }
    pub fn edge_update(this: &Arc<JupBot>, edge_id: u32, atob: bool) { 
        let is_running = this.is_running.load(Ordering::SeqCst);
        let last_blockhash_timestamp = this.last_blockhash_timestamp.load(Ordering::SeqCst);

        if is_running && last_blockhash_timestamp > 0 {
            let bot_edges = unsafe{&mut * this.edges.load(Ordering::SeqCst)};
            if  bot_edges.contains_key(&edge_id) {
                let process_cnt = this.process_pair_cnt.load(Ordering::SeqCst);
                if process_cnt < 1 {
                    let rpc_health = this.rpc_client.get_health();
                    if rpc_health.is_ok() {
                        this.process_pair_cnt.store(process_cnt + 1, Ordering::SeqCst);
                        let (bundle_sent, elapsed) = measure_us!(JupBot::run_top_routes(this, edge_id, atob));
                        let process_cnt = this.process_pair_cnt.load(Ordering::SeqCst);
                        if  process_cnt > 0 {
                            this.process_pair_cnt.store(process_cnt - 1, Ordering::SeqCst);
                        }
                        if bundle_sent {
                            println!("{}ms elapsed!", elapsed / 1000);
                        }

                    }
                }
            }
        }
    }
}