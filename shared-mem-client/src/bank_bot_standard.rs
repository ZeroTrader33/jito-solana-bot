use {
    arrayref::{array_ref, array_refs}, borsh::{BorshDeserialize, BorshSerialize}, dashmap::DashMap, futures::{
      stream::{FuturesUnordered, StreamExt},
      TryFutureExt,
    }, hwloc::{CpuSet, Topology, CPUBIND_THREAD}, serde::{Deserialize, Serialize}, solana_client::{client_error::Result as ClientResult, rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig}, solana_measure::{measure::Measure, measure_us}, solana_sdk::{
        account::{AccountSharedData, ReadableAccount}, address_lookup_table::AddressLookupTableAccount, commitment_config::CommitmentLevel, compute_budget::ComputeBudgetInstruction, hash::Hash, instruction::{AccountMeta, Instruction}, message::{AddressLoader, VersionedMessage::V0}, pubkey::Pubkey, signature::{self, Keypair, Signature, Signer}, signer::SignerError, transaction::VersionedTransaction
    }, std::{
        collections::HashMap, fmt, fs::File, io::Read, str::FromStr, sync::{
            atomic::{AtomicBool, AtomicPtr, AtomicU64, AtomicUsize, Ordering}, Arc, Mutex, RwLock
        }, thread::spawn, time::{Duration, UNIX_EPOCH}
    }, tokio::task::spawn_blocking, crate::big_num::{U128, MulDiv}, crate::safe_math::SafeMath,
};




pub const ADMIN_KEYPAIR_PATH: &str = "/home/sol/project/mcard-contract/runner-rust/admin.json";
pub const PAIR_FILE_PATH: &str =
    "/home/sol/project/mcard-contract/json-data/standard/serialized-standard-pools.bin";
pub const LOOKUPTABLE_FILE_PATH: &str =
    "/home/sol/project/mcard-contract/json-data/standard/lookuptable.json";
pub const ROUTES_FILE_PATH: &str = "/home/sol/project/mcard-contract/json-data/standard/standard-routes.json";
pub const GROUP_FOLDER_PATH: &str = "/home/sol/project/mcard-contract/json-data/standard/groups/";
pub const POOL_ADDRESSES_FILE_PATH: &str = "/home/sol/project/mcard-contract/json-data/standard/standard-pool-addresses.json";

pub const RPC_ENDPOINT: &str = "http://127.0.0.1:8899";
pub const WSS_ENDPOINT: &str = "ws://127.0.0.1:8900";

pub const JITO_TIP_ACCOUNTS: [&str; 1] = ["96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5"];

pub const PAIRS_CNT: usize = 1300;
pub const ROUTES_CNT: usize = 368028;
pub const FEE_MULTIPLIER: u32 = 10000;
pub const PRICE_MULTIPLIER: u64 = 1_000_000_000;
pub const ALLOWED_AMOUNT_CHANGE: u64 = 999;
pub const ALLOWED_PRICE_CHANGE: u128 = 6; // 0.000001
pub const BLOCKHASH_UPDATE_PERIOD: u64 = 500;

pub const MAX_CU: u32 = 1_400_000_000;
pub const VALIDATOR_CPU_START: u32 = 9;
pub const VALIDATOR_CPU_END: i32 = 39;
pub const BOT_CPU_START: u32 = 40;
pub const BOT_CPU_END: i32 = 95;
const BOT_THREAD_CNT: usize = 50;

macro_rules! trace {
    ($($args: expr),*) => {
        // print!("TRACE: file: {}, line: {}", file!(), line!());
        // $(
        //     print!(", {}: {}", stringify!($args), $args);
        // )*
        // println!(""); // to get a new line at the end
    }
}
pub struct BankBot {
    pub tokio_runtime: tokio::runtime::Runtime,
    pub pairs: AtomicPtr<Vec<Pair>>,
    pub routes: AtomicPtr<Vec<Route>>,
    pub routes_state: DashMap<usize, RouteState>,

    pub lookuptables: AtomicPtr<Vec<AddressLookupTableAccount>>,
    pub groups: AtomicPtr<Vec<Vec<u32>>>,

    pub accounts_to_subscribe_vec: AtomicPtr<Vec<Pubkey>>,
    pub accounts_to_subscribe: AtomicPtr<HashMap<Pubkey, SubscribeAccount>>,


    pub last_blockhash_timestamp: AtomicU64,
    pub latest_blockhash: AtomicPtr<Option<Hash>>,
    pub is_running: AtomicBool,
    pub no_jito_validators: AtomicPtr<Vec<Pubkey>>,
    pub is_price_updated_initially: AtomicBool,
    pub jito_bundle_endpoint_idx: AtomicUsize,
    pub jito_tx_endpoint_idx: AtomicUsize,
    pub proxy_idx: AtomicUsize,
    pub bundle_sender: udp_proxy::BundleSender,

    pub is_processing_price_update: AtomicBool,

    pub payer: Keypair,
    pub rpc_client: Arc<RpcClient>,

}

impl Default for BankBot {
    fn default() -> Self {
        let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
          .worker_threads(BOT_THREAD_CNT)
          // .on_thread_start(move || renice_this_thread(rpc_niceness_adj).unwrap())
          .thread_name("processNotifyThread")
          .enable_all()
          .build()
          .expect("Runtime");
        let payer = signature::read_keypair_file(ADMIN_KEYPAIR_PATH).unwrap();
        let rpc_client = Arc::new(RpcClient::new(String::from(RPC_ENDPOINT)));
        BankBot {
            tokio_runtime: runtime,
            pairs: AtomicPtr::new(Box::into_raw(Box::new(Vec::new()))),
            routes: AtomicPtr::new(Box::into_raw(Box::new(Vec::new()))),
            routes_state: DashMap::new(),
            lookuptables: AtomicPtr::new(Box::into_raw(Box::new(Vec::new()))),
            groups: AtomicPtr::new(Box::into_raw(Box::new(Vec::new()))),
            accounts_to_subscribe_vec: AtomicPtr::new(Box::into_raw(Box::new(Vec::new()))),
            accounts_to_subscribe: AtomicPtr::new(Box::into_raw(Box::new(HashMap::new()))),
            is_processing_price_update: AtomicBool::new(false),

            last_blockhash_timestamp: AtomicU64::new(0),
            latest_blockhash: AtomicPtr::new(Box::into_raw(Box::new(None))),

            is_running: AtomicBool::new(false),
            no_jito_validators: AtomicPtr::new(Box::into_raw(Box::new(Vec::new()))),
            is_price_updated_initially: AtomicBool::new(false),
            jito_bundle_endpoint_idx: AtomicUsize::new(0),
            jito_tx_endpoint_idx: AtomicUsize::new(0),
            proxy_idx: AtomicUsize::new(0),
            payer,
            rpc_client,
            bundle_sender: udp_proxy::BundleSender::create(),
        }
    }
}

pub struct ArcBankBot {
    pub bank_bot: Arc<BankBot>,
}

impl ArcBankBot {

    pub fn get_current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }

    pub fn get_arc_clone(&self) -> Self {
        ArcBankBot {bank_bot: Arc::clone(&self.bank_bot)}
    }

    pub fn update_latest_blockhash(&self) {
        loop {
            let maxRetry = 5;
            let mut retry = 0;
            let last_blockhash_timestamp = self.bank_bot.last_blockhash_timestamp.load(Ordering::SeqCst);

            if last_blockhash_timestamp + BLOCKHASH_UPDATE_PERIOD < Self::get_current_timestamp() || last_blockhash_timestamp == 0
            {
                let mut hash_tmp: Option<Hash> = None;
                loop {
                    if let Ok(hash) = self.bank_bot.rpc_client.get_latest_blockhash() {
                        hash_tmp = Some(hash)
                    };
                    if hash_tmp.is_some() {
                        break;
                    }
                    retry += 1;
                    if retry > maxRetry {
                        break;
                    }
                }

                if hash_tmp.is_some() {
                    self.bank_bot.last_blockhash_timestamp.store(Self::get_current_timestamp(), Ordering::SeqCst);
                    self.bank_bot.latest_blockhash.store(Box::into_raw(Box::new(hash_tmp)), Ordering::SeqCst);
                }
                // println!("latest blockhash bot running, current-> {:#?}", hash_tmp);
            }
            std::thread::sleep(Duration::from_millis(200));
        }
    }

    pub fn account_update(&self, pubkey: &Pubkey, account_data: &[u8]) {        
        let is_running = self.bank_bot.is_running.load(Ordering::SeqCst);
        let is_price_update_initially = self.bank_bot.is_price_updated_initially.load(Ordering::SeqCst);

        if is_running && is_price_update_initially {
            println!("account_updated: {:#?}", pubkey);
            //   trace!();
            let accounts_to_subscribe_vec = unsafe{&* self.bank_bot.accounts_to_subscribe_vec.load(Ordering::SeqCst)};
            let accounts_to_subscribe = unsafe{&mut * self.bank_bot.accounts_to_subscribe.load(Ordering::SeqCst)};
            let account_subscribe = accounts_to_subscribe.get_mut(pubkey).unwrap();
            account_subscribe.account_data = account_data.to_vec();

            if accounts_to_subscribe_vec.contains(&pubkey) {
                let instance = self.get_arc_clone();
                std::thread::spawn(move || {
                    let pair_index = account_subscribe.pair_index.unwrap();
                    instance.build_pair_price(pair_index);

                    if instance.bank_bot.is_processing_price_update.load(Ordering::SeqCst) == false {
                        trace!();
                        instance.bank_bot.is_processing_price_update.store(true, Ordering::SeqCst);
                        trace!();
                        let instance_clone = instance.get_arc_clone();
                        std::thread::spawn(move|| {
                            let (_, elapsed) = measure_us!(instance_clone.process_pair_change(pair_index));
                            println!("process_pair_change time: {} us", elapsed);
                        }).join().expect("processing pair thread error");
                        instance.bank_bot.is_processing_price_update.store(false, Ordering::SeqCst);
                    }
                });
            }
        }
    }

    pub fn set_run_mode(&self, is_running: bool) {
      self.bank_bot.is_running.store(is_running, Ordering::SeqCst);
      if !is_running {
            self.bank_bot.is_processing_price_update.store(false, Ordering::SeqCst);
            self.bank_bot.is_price_updated_initially.store(false, Ordering::SeqCst);
            self.bank_bot.routes_state.clear();
            println!("bot stopped!");
      }
    }

    pub fn process_pair_change(&self, pair_idx: usize) {
        // calculate acc prices of a group;
        trace!();
        let group = unsafe{&* self.bank_bot.groups.load(Ordering::SeqCst)};
        let routes = unsafe { &*self.bank_bot.routes.load(Ordering::SeqCst) };
        let accounts_to_subscribe = unsafe { &* self.bank_bot.accounts_to_subscribe.load(Ordering::SeqCst)};

        let wsol_mint_pubkey =
            Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
        let usdc_mint_pubkey =
            Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
        let wsol_ata_pubkey =
            Pubkey::from_str("D398BcsjQ8tyuxGvfCHecgAanKWs8QWj2gDPYfup86vz").unwrap();
        let usdc_ata_pubkey =
            Pubkey::from_str("5n99AA4e888tVdtjvvS6J7ab4LPTLFqJXUL136p9hXWh").unwrap();
        let wsol_token_vault_amount = ArcBankBot::get_amount_from_token_account(&accounts_to_subscribe.get(&wsol_ata_pubkey).unwrap().account_data);
        let usdc_token_vault_amount = ArcBankBot::get_amount_from_token_account(&accounts_to_subscribe.get(&usdc_ata_pubkey).unwrap().account_data);

        let mut handles = Vec::new();
        let filtered_routes_by_pnl: Arc<Mutex<Vec<(usize, u64, u64)>>> = Arc::new(Mutex::new(Vec::new()));
        trace!();
        //   for route_idx in group[pair_idx].iter() {
        let route_len = group[pair_idx].len();
        let current_route_idx = Arc::new(AtomicUsize::new(0usize));

        for _ in 0..30 {
            let instance = ArcBankBot {bank_bot: Arc::clone(&self.bank_bot)};
            let filtered_routes_by_pnl_clone = Arc::clone(&filtered_routes_by_pnl);
            let current_route_idx_clone = Arc::clone(&current_route_idx);

            handles.push(std::thread::spawn(move || {
                loop {
                    trace!();
                    let route_idx = current_route_idx_clone.load(Ordering::SeqCst);
                    if route_idx >= route_len {
                        break;
                    } else {
                        current_route_idx_clone.store(route_idx+1, Ordering::SeqCst);
                    }
                    let acc_price = instance.get_acc_price(route_idx);
                    if acc_price > PRICE_MULTIPLIER as u128 {
                        let route = &routes[route_idx];
                        let (tx_fee, max_amount) = if route.route_start_mint == wsol_mint_pubkey {
                            (5140, wsol_token_vault_amount)
                        } else {
                            (800, usdc_token_vault_amount)
                        };
                        trace!();
                        let (input_amount, final_price, out_amount) = instance.determine_input_amount(10000, max_amount, route_idx);
                        if out_amount > input_amount + tx_fee {
                            let pnl = out_amount - input_amount - tx_fee;
                            let pnl_base_sol = if route.route_start_mint == wsol_mint_pubkey { pnl } else { pnl * 7 };

                            let present_route_state = RouteState {acc_price, pnl: pnl_base_sol};
                            if instance.bank_bot.routes_state.get(&route_idx).map_or(true, |v| !v.eq(&present_route_state)) {
                                let mut filtered_routes = filtered_routes_by_pnl_clone.lock().unwrap();
                                filtered_routes.push((route_idx, input_amount, pnl_base_sol));
                                instance.bank_bot.routes_state.insert(route_idx, present_route_state);
                            }
                            trace!();
                        }
                    }
                }
            }));
        }
        for h in handles {
            h.join().expect("acc price and pnl thread panic");
        }

        // let _filtered_routes_by_pnl = unsafe{&mut * filtered_routes_by_pnl.load(Ordering::SeqCst)};
        let mut _filtered_routes_by_pnl = filtered_routes_by_pnl.lock().unwrap();
        //   println!("filtered_routes_by_pnl len: {}", _filtered_routes_by_pnl.len());

        _filtered_routes_by_pnl.sort_by(|&(route_idx_a, _, pnl_a), &(route_idx_b, _, pnl_b)| (pnl_a as u128 * 10_000_000 / routes[route_idx_a].cu as u128).cmp(&(pnl_b as u128 * 10_000_000 / routes[route_idx_b].cu as u128)));

        // send transaction
        let runnable_routes = if _filtered_routes_by_pnl.len() < 10 {
            _filtered_routes_by_pnl.to_vec()
        } else {
            _filtered_routes_by_pnl[0..10].to_vec()
        };
        trace!();
        println!("total_route_cnt: {}, runnable_routes cnt: {:#?}", route_len, runnable_routes.len());

        let mut run_route_handles = Vec::new();
        
        for (route_idx, input_amount, _pnl) in runnable_routes {
            let instance = ArcBankBot{bank_bot: Arc::clone(&self.bank_bot)};
            run_route_handles.push(std::thread::spawn(move || {
                trace!();
            instance.run_route(route_idx, input_amount);
            }));
        }

        // let mut run_route_cnt = 0;
        // for h in run_route_handles {
        //     run_route_cnt += 1;
        //     h.join().expect("run route thread panic!");
        // }
    }

    pub fn run_route(&self, route_idx: usize, input_amount: u64) {
    //   println!("run_route started!");
    trace!();
      let routes = unsafe{&* self.bank_bot.routes.load(Ordering::SeqCst)};
      let mut route = routes[route_idx].clone();
      route.update_amount(input_amount);

      let cu_ix = ComputeBudgetInstruction::set_compute_unit_limit(route.cu);

      let trade_pay_jito_tip_params = TradePayJitoTipParams {
          usdc_sol_rate: 6700,
          percent: 5000,
      };
      trace!();
      let trade_pay_jito_tip_account_metas = vec![
          AccountMeta {
              pubkey: self.bank_bot.payer.pubkey(),
              is_signer: true,
              is_writable: true,
          },
          AccountMeta {
              pubkey: Pubkey::from_str("Byg11WnJ9Q53hzDFmeBH6sFgnpUFVJJsCvhZBTWkAQEL").unwrap(), // trade
              is_signer: false,
              is_writable: true,
          },
          AccountMeta {
            pubkey: Pubkey::from_str("G5vAeh1AQBYi4c4bCqhioV3AWjh7Gw2oAaUCDcdcWy3G").unwrap(), // trade_authority
            is_signer: false,
            is_writable: false,
          },
          AccountMeta {
              pubkey: Pubkey::from_str(JITO_TIP_ACCOUNTS[0]).unwrap(),
              is_signer: false,
              is_writable: true,
          },
          AccountMeta {
              pubkey: solana_program::system_program::ID,
              is_signer: false,
              is_writable: false,
          },
      ];
      trace!();
      let jito_tip_ix = Route::get_trade_pay_jito_tip_ix(
          &trade_pay_jito_tip_account_metas,
          &trade_pay_jito_tip_params,
      );

      let jito_tip_versioned_tx = self.get_versioned_tx(&[jito_tip_ix], &[]);
      let versioned_tx =
          self.get_versioned_tx(&[cu_ix, route.ix.clone()], &route.lookuptables);
    // let send_res = self.create_and_send_ix(&[cu_ix, route.ix.clone()], &route.lookuptables, false);
    
    // println!("send_res: {:#?}", send_res);
      if jito_tip_versioned_tx.is_ok() && versioned_tx.is_ok() {
        let versioned_tx = versioned_tx.unwrap();
          self.bank_bot
              .bundle_sender
              .send_bundle(&[versioned_tx.clone(), jito_tip_versioned_tx.unwrap()]);

        // self.bank_bot.rpc_client.send_transaction_with_config(
        //     &versioned_tx.clone(),
        //     RpcSendTransactionConfig {
        //         skip_preflight: false,
        //         preflight_commitment: Some(CommitmentLevel::Confirmed),
        //         ..RpcSendTransactionConfig::default()
        //     },
        // );
      } else {
        // println!("versioned_tx: {:#?}", versioned_tx);
      }
      trace!();
    //   println!("run_route ended!");
    }

    pub fn create_and_send_ix(
        &self,
        tx_instructions: &[Instruction],
        lookuptables: &[AddressLookupTableAccount],
        skip_preflight: bool,
    ) -> ClientResult<Signature> {
        trace!();
        let payer = &self.bank_bot.payer;
        let signers = vec![payer];
        let blockhash = unsafe{&* self.bank_bot.latest_blockhash.load(Ordering::SeqCst)};

        trace!();
        let versioned_message = V0(solana_sdk::message::v0::Message::try_compile(
            &self.bank_bot.payer.pubkey(),
            tx_instructions,
            lookuptables,
            blockhash.unwrap(),
        )
        .unwrap());

        let tx = solana_sdk::transaction::VersionedTransaction::try_new(versioned_message, &signers)
            .unwrap();
        trace!();
        self.bank_bot.rpc_client.send_transaction_with_config(
            &tx,
            RpcSendTransactionConfig {
                skip_preflight,
                preflight_commitment: Some(CommitmentLevel::Confirmed),
                ..RpcSendTransactionConfig::default()
            },
        )
    }

    pub fn get_versioned_tx(
        &self,
        tx_instructions: &[Instruction],
        lookuptables: &[AddressLookupTableAccount],
    ) -> std::result::Result<VersionedTransaction, SignerError> {
        let signers = vec![&self.bank_bot.payer];
        let blockhash = unsafe{ &* self.bank_bot.latest_blockhash.load(Ordering::SeqCst)};

        let versioned_message = V0(solana_sdk::message::v0::Message::try_compile(
            &self.bank_bot.payer.pubkey(),
            tx_instructions,
            lookuptables,
            blockhash.unwrap(),
        )
        .unwrap());
        VersionedTransaction::try_new(versioned_message, &signers)
    }

    pub fn create() -> Self {
      ArcBankBot {
        bank_bot: Arc::new(BankBot::default())
      }
    }

    pub async fn run(&self) {
      std::panic::set_hook(Box::new(|info| {
        println!("Caught a panic: {:?}", info);
      }));
      println!("standard shm server bank bot is started!");
      self.bank_bot.is_running.store(true, Ordering::SeqCst);
      self.read_from_files().await;
      trace!();
      self.clone_accounts_and_build_pair_prices_initially().await;
      println!("setup ended.");
      trace!();
    }

    pub fn set_validator_cpu() {
        Self::set_cpu_range_for_this_thread(VALIDATOR_CPU_START, VALIDATOR_CPU_END);
    }
    pub fn set_bot_cpu() {
        Self::set_cpu_range_for_this_thread(BOT_CPU_START, BOT_CPU_END);
    }
    pub fn set_cpu_range_for_this_thread(start: u32, end: i32) {
        let topo = Arc::new(Mutex::new(Topology::new()));
        let child_topo = topo.clone();
        let tid = unsafe { libc::pthread_self() };
        let mut locked_topo = child_topo.lock().unwrap();
        let before = locked_topo.get_cpubind_for_thread(tid, CPUBIND_THREAD);
        // println!("Thread {}: Before {:?}", i, before);
        let bind_to = CpuSet::from_range(start, end);

        locked_topo
            .set_cpubind_for_thread(tid, bind_to, CPUBIND_THREAD)
            .unwrap();

        let after = locked_topo.get_cpubind_for_thread(tid, CPUBIND_THREAD);
        // Mutex::unlock(locked_topo);

        println!("Cpu Set: Before {:?}, After {:?}", before, after);
    }

    pub async fn read_from_files(&self) {
        let pairs = unsafe { &mut *self.bank_bot.pairs.load(Ordering::SeqCst) };
        let lookuptables = unsafe { &mut *self.bank_bot.lookuptables.load(Ordering::SeqCst) };

        if let Ok(pairs_slice) = spawn_blocking(|| {
            let mut pairs_file = File::open(PAIR_FILE_PATH)?;
            let mut slice = Vec::new();
            pairs_file.read_to_end(&mut slice)?;
            Ok::<_, std::io::Error>(slice)
        })
        .await
        {
            for pair_chunk_slice in pairs_slice.unwrap().chunks(Pair::PAIR_SLICE_SIZE) {
                let pair = Pair::deserialize(pair_chunk_slice);
                pairs.push(pair);
            }
        }

        // if let Ok(acounts_to_subscribe_content) = spawn_blocking(|| {
        //     let mut accounts_to_subscribe_file = File::open(POOL_ADDRESSES_FILE_PATH)?;
        //     let mut content = String::new();
        //     accounts_to_subscribe_file.read_to_string(&mut content)?;
        //     Ok::<_, std::io::Error>(content)
        // })
        // .await
        // {
        //     match serde_json::from_str(&acounts_to_subscribe_content.unwrap()) {
        //         Ok(res) => {
        //             let accounts_to_subscribe_tmp: Vec<String> = res;
        //             let accounts_to_subscribe = accounts_to_subscribe_tmp.iter().map(|addr| Pubkey::from_str(addr).unwrap()).collect();
        //             self.bank_bot.accounts_to_subscribe_vec.store(Box::into_raw(Box::new(accounts_to_subscribe)), Ordering::SeqCst);
        //         }
        //         Err(_err) => println!("accounts_to_subscribe read error-> {:#?}", _err),
        //     };
        // };

        if let Ok(lookuptables_content) = spawn_blocking(|| {
            let mut lookuptables_file = File::open(LOOKUPTABLE_FILE_PATH)?;
            let mut content = String::new();
            lookuptables_file.read_to_string(&mut content)?;
            Ok::<_, std::io::Error>(content)
        })
        .await
        {
            match serde_json::from_str(&lookuptables_content.unwrap()) {
                Ok(res) => {
                    let lookuptables_tmp: Vec<LookupTable> = res;

                    for lookuptable in lookuptables_tmp {
                        lookuptables.push(AddressLookupTableAccount {
                            key: Pubkey::from_str(&lookuptable.lookuptable).unwrap(),
                            addresses: lookuptable
                                .keys
                                .iter()
                                .map(|addr| Pubkey::from_str(addr).unwrap())
                                .collect(),
                        });
                    }
                }
                Err(_err) => println!("lookuptables read error-> {:#?}", _err),
            };
        };

        if let Ok(routes_content) = spawn_blocking(|| {
            let mut routes_file = File::open(ROUTES_FILE_PATH)?;
            let mut content = String::new();
            routes_file.read_to_string(&mut content)?;
            Ok::<_, std::io::Error>(content)
        })
        .await
        {
            match serde_json::from_str(&routes_content.unwrap()) {
                Ok(res) => {
                    let routes_tmp: Vec<Vec<u32>> = res;

                    let mut route_futures = FuturesUnordered::new();

                    for (route_idx, route) in routes_tmp.into_iter().enumerate() {
                        let pairs = unsafe { &*self.bank_bot.pairs.load(Ordering::SeqCst) };
                        let lookuptables =
                            unsafe { &*self.bank_bot.lookuptables.load(Ordering::SeqCst) };
                        route_futures.push(tokio::spawn(async move {
                            let result = spawn_blocking(move || {
                              let route = Route::build_route(&route, pairs, lookuptables);
                              route
                            }).await;
                            (route_idx, result)
                        }));
                    }
                    let mut full_routes: Vec<(usize, Route)> = Vec::new();

                    let mut read_cnt = 0;
                    while let Some(result) = route_futures.next().await {
                      read_cnt += 1;
                      match result {
                          Ok((route_idx, Ok(route))) => {
                            full_routes.push((route_idx, route));
                          },
                          _ => {

                          }
                      };
                      if read_cnt % 1000 == 0 {
                        println!("{} route read!", read_cnt);
                      }
                    }

                    full_routes.sort_by_key(|(idx, _)| *idx);
                    let all_routes: Vec<Route> =
                        full_routes.into_iter().map(|(_, route)| route).collect();
                    // println!("all_routes[0]: {:#?}", all_routes[0]);
                    self.bank_bot
                        .routes
                        .store(Box::into_raw(Box::new(all_routes)), Ordering::SeqCst);
                }
                Err(_err) => println!("routes read error-> {:#?}", _err),
            };
        };

        let mut group_futures = FuturesUnordered::new();
        for group_file_idx in 0..PAIRS_CNT {
            let group_file_path =
                GROUP_FOLDER_PATH.to_owned() + &group_file_idx.to_string() + ".json";
            group_futures.push(tokio::spawn(async move {
                let content = spawn_blocking(move || {
                    let mut group_file = File::open(group_file_path)?;
                    let mut content = String::new();
                    group_file.read_to_string(&mut content)?;
                    Ok::<_, std::io::Error>(content)
                })
                .await;
                (group_file_idx, content)
            }));
        }
        let mut all_groups = Box::new(Vec::new());

        while let Some(result) = group_futures.next().await {
            match result {
                Ok((file_idx, Ok(content))) => {
                    if let Ok(res) = serde_json::from_str::<Vec<u32>>(&content.unwrap()) {
                        if file_idx % 100 == 0 {
                            println!("current read group index: {:?}", file_idx);
                        }
                        all_groups.push((file_idx, res));
                    }
                }
                Err(e) => println!("Error reading group file: {:?}", e),
                _ => println!("Error reading group file"),
            }
        }
        all_groups.sort_by_key(|(idx, _)| *idx);
        let sorted_groups: Vec<Vec<u32>> =
            all_groups.into_iter().map(|(_idx, group)| group).collect();
        self.bank_bot
            .groups
            .store(Box::into_raw(Box::new(sorted_groups)), Ordering::SeqCst);

        println!("successfully read files!");
    }

    pub async fn clone_accounts_and_build_pair_prices_initially(&self) {

      let pairs = unsafe{&* self.bank_bot.pairs.load(Ordering::SeqCst)};
      let accounts_to_subscribe_vec = unsafe{&mut * self.bank_bot.accounts_to_subscribe_vec.load(Ordering::SeqCst)};
      let accounts_to_subscribe = unsafe{&mut *self.bank_bot.accounts_to_subscribe.load(Ordering::SeqCst)};
      trace!();

      //clone user vaults initially
      let wsol_ata_pubkey = Pubkey::from_str("D398BcsjQ8tyuxGvfCHecgAanKWs8QWj2gDPYfup86vz").unwrap();
      let usdc_ata_pubkey = Pubkey::from_str("5n99AA4e888tVdtjvvS6J7ab4LPTLFqJXUL136p9hXWh").unwrap();
      accounts_to_subscribe.insert(wsol_ata_pubkey, SubscribeAccount{account_data: self.bank_bot.rpc_client.get_account_data(&wsol_ata_pubkey).unwrap_or(Vec::new()), pair_index: None});
      accounts_to_subscribe.insert(usdc_ata_pubkey, SubscribeAccount{account_data: self.bank_bot.rpc_client.get_account_data(&usdc_ata_pubkey).unwrap_or(Vec::new()), pair_index: None});

      for (pair_idx, pair) in pairs.iter().enumerate() {
        let accounts = pair.get_all_accounts_for_subscribe();
        for (pubkey, is_subscribe_for_price_calc) in accounts {
            let account_data = match self.bank_bot.rpc_client.get_account_data(&pubkey) {
                Ok(data) => data,
                Err(_) => Vec::new()
            };
          if !accounts_to_subscribe_vec.contains(&pubkey) && is_subscribe_for_price_calc == true {    
            accounts_to_subscribe_vec.push(pubkey);
          }
          accounts_to_subscribe.insert(pubkey, SubscribeAccount{account_data, pair_index: if is_subscribe_for_price_calc {Some(pair_idx)} else {None}});
        }
      }

      let mut build_price_futures = FuturesUnordered::new();

      for i in 0..pairs.len() {
        let instance = ArcBankBot {
          bank_bot: Arc::clone(&self.bank_bot)
        };

        build_price_futures.push(tokio::spawn(async move {
          let _ = spawn_blocking(move || {
            instance.build_pair_price(i);
          }).await;
        }))
      }

      let mut build_price_cnt = 0;
      while let Some(result) = build_price_futures.next().await {
        build_price_cnt += 1;
      }
    //   println!("total build pair price cnt: {}", build_price_cnt);
      self.bank_bot.is_price_updated_initially.store(true, Ordering::SeqCst);
    //   println!("clone_accounts_and_build_pair_prices_initially done!");
    trace!();
    }

    pub fn build_pair_price(&self, pair_idx: usize) {
        //
        let pairs = unsafe { &mut *self.bank_bot.pairs.load(Ordering::SeqCst) };
        let pair = &mut pairs[pair_idx];
        trace!();
        let accounts_info = unsafe {&* self.bank_bot.accounts_to_subscribe.load(Ordering::SeqCst)};
        // println!("build_pair_price pair#{}: {:#?}",pair_idx, pair);
        match pair.swap_type {
            SwapType::BonkSwap => {
                const POOL_OFFSET_UNTIL_RESERVE: usize = 8 + 32 * 6;
                const POOL_OFFSET_AFTER_RESERVE: usize = 8 + 32 * 6 + 8 + 8;

                if let Some(pool_state_account_info) = accounts_info.get(&pair.pool_state) {
                    let pool_state_data = pool_state_account_info.account_data.to_vec();
                    let input = array_ref![pool_state_data, 0, POOL_OFFSET_AFTER_RESERVE];
                    let (_, reserve_x_slice, reserve_y_slice) =
                        array_refs![input, POOL_OFFSET_UNTIL_RESERVE, 8, 8];
                    let reserve_x = u64::from_le_bytes(*reserve_x_slice);
                    let reserve_y = u64::from_le_bytes(*reserve_y_slice);

                    let price: u128 = if reserve_x > 0 {
                        (reserve_y as u128)
                            .checked_mul(PRICE_MULTIPLIER as u128)
                            .unwrap()
                            .checked_div(reserve_x as u128)
                            .unwrap()
                    } else {
                        // msg!("zero coin vault {:#?}", coin_vault_account_info.key());
                        0u128
                    };

                    let price_reverse = if reserve_y > 0 {
                        (reserve_x as u128)
                            .checked_mul(PRICE_MULTIPLIER as u128)
                            .unwrap()
                            .checked_div(reserve_y as u128)
                            .unwrap()
                    } else {
                        // msg!("zero pc vault {:#?}", coin_vault_account_info.key());
                        0u128
                    };

                    // println!("Bonkswap Price: {:#?}", Price {
                    //   price_a_to_b: price,
                    //   price_b_to_a: price_reverse,
                    //   vault_a_amount: reserve_x,
                    //   vault_b_amount: reserve_y,
                    // });
                    pair.price = Some(Price {
                        price_a_to_b: price,
                        price_b_to_a: price_reverse,
                        vault_a_amount: Some(reserve_x),
                        vault_b_amount: Some(reserve_y),
                        active_tick_id: None,
                        pool_token_mint_a: None,
                        liquidity: None,
                        sqrt_price_x64: None,

                        config_denominator: None,
                        std_spread: None,
                        std_spread_buffer: None,
                        spread_coef: None,
                        base_decimal_value: None,
                        price_buffer_coin: None,
                    });
                }
            },
            SwapType::MeteoraPools => {
                const A_LP_MINT_IDX: usize = 8;
                const B_LP_MINT_IDX: usize = 9;

                const A_LP_VAULT_IDX: usize = 10;
                const B_LP_VAULT_IDX: usize = 11;

                let a_vault_account_info = accounts_info.get(&pair.pool_vault_a);
                let b_vault_account_info = accounts_info.get(&pair.pool_vault_b);
                let a_lp_mint_account_info =
                    accounts_info.get(&pair.accounts_vec_a_to_b[A_LP_MINT_IDX].pubkey);
                let b_lp_mint_account_info =
                    accounts_info.get(&pair.accounts_vec_a_to_b[B_LP_MINT_IDX].pubkey);
                let a_lp_vault_account_info =
                    accounts_info.get(&pair.accounts_vec_a_to_b[A_LP_VAULT_IDX].pubkey);
                let b_lp_vault_account_info =
                    accounts_info.get(&pair.accounts_vec_a_to_b[B_LP_VAULT_IDX].pubkey);

                if a_vault_account_info.is_some()
                    && b_vault_account_info.is_some()
                    && a_lp_mint_account_info.is_some()
                    && b_lp_mint_account_info.is_some()
                    && a_lp_vault_account_info.is_some()
                    && b_lp_vault_account_info.is_some()
                {
                    let now = std::time::SystemTime::now();
                    let current_time = now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
                    let a_supply = ArcBankBot::get_supply_from_mint(
                        &a_lp_mint_account_info.unwrap().account_data,
                    );
                    let b_supply = ArcBankBot::get_supply_from_mint(
                        &b_lp_mint_account_info.unwrap().account_data,
                    );
                    let a_lp_vault_amount = ArcBankBot::get_amount_from_token_account(
                        &a_lp_vault_account_info.unwrap().account_data,
                    );
                    let b_lp_vault_amount = ArcBankBot::get_amount_from_token_account(
                        &b_lp_vault_account_info.unwrap().account_data,
                    );
                    let a_withdrawable_amount =
                        ArcBankBot::get_meteora_pools_vault_withdrawable_amount(
                            &a_vault_account_info.unwrap().account_data,
                            current_time,
                        );
                    let b_withdrawable_amount =
                        ArcBankBot::get_meteora_pools_vault_withdrawable_amount(
                            &b_vault_account_info.unwrap().account_data,
                            current_time,
                        );

                    if !(a_withdrawable_amount == 0 || b_withdrawable_amount == 0) {
                        let swap_source_amount = u128::from(a_lp_vault_amount)
                            .checked_mul(u128::from(a_withdrawable_amount))
                            .unwrap()
                            .checked_div(u128::from(a_supply))
                            .unwrap() as u64;
                        let swap_dst_amount = u128::from(b_lp_vault_amount)
                            .checked_mul(u128::from(b_withdrawable_amount))
                            .unwrap()
                            .checked_div(u128::from(b_supply))
                            .unwrap() as u64;

                        let price: u128 = if swap_source_amount == 0 {
                            0u128
                        } else {
                            u128::from(swap_dst_amount)
                                .checked_mul(u128::from(PRICE_MULTIPLIER))
                                .unwrap()
                                .checked_div(u128::from(swap_source_amount))
                                .unwrap()
                        };
                        let price_reverse: u128 = if swap_dst_amount == 0 {
                            0u128
                        } else {
                            u128::from(swap_source_amount)
                                .checked_mul(u128::from(PRICE_MULTIPLIER))
                                .unwrap()
                                .checked_div(u128::from(swap_dst_amount))
                                .unwrap()
                        };
                        // println!("Meteorapools Price: {:#?}", Price {
                        //   price_a_to_b: price,
                        //   price_b_to_a: price_reverse,
                        //   vault_a_amount: swap_source_amount,
                        //   vault_b_amount: swap_dst_amount,
                        // });

                        pair.price = Some(Price {
                            price_a_to_b: price,
                            price_b_to_a: price_reverse,
                            vault_a_amount: Some(swap_source_amount),
                            vault_b_amount: Some(swap_dst_amount),
                            active_tick_id: None,
                            pool_token_mint_a: None,
                            liquidity: None,
                            sqrt_price_x64: None,

                            config_denominator: None,
                            std_spread: None,
                            std_spread_buffer: None,
                            spread_coef: None,
                            base_decimal_value: None,
                            price_buffer_coin: None,
                        });
                    }
                }
            },
            SwapType::RaydiumPoolV4 | SwapType::OrcaSwapV2 | SwapType::TokenSwap | SwapType::CropperFinance | SwapType::DooarSwap | SwapType::DexlabSwap | SwapType::SarosAmm | SwapType::Fluxbeam | SwapType::AldrinAmm | SwapType::AldrinAmmV2 | SwapType::PenguinFinance | SwapType::StepFinance | SwapType::RaydiumCpmm => {
                let coin_vault_account_info = accounts_info.get(&pair.pool_vault_a);
                let pc_vault_account_info = accounts_info.get(&pair.pool_vault_b);
                // println!("coin_vault_account_info: {:#?}", coin_vault_account_info);
                // println!("pc_vault_account_info: {:#?}", pc_vault_account_info);
                if coin_vault_account_info.is_some() && pc_vault_account_info.is_some() {
                    let coin_vault_amount = ArcBankBot::get_amount_from_token_account(
                        &coin_vault_account_info.unwrap().account_data,
                    );
                    let pc_vault_amount = ArcBankBot::get_amount_from_token_account(
                        &pc_vault_account_info.unwrap().account_data,
                    );

                    let price: u128 = if coin_vault_amount > 0 {
                        (pc_vault_amount as u128)
                            .checked_mul(PRICE_MULTIPLIER as u128)
                            .unwrap()
                            .checked_div(coin_vault_amount as u128)
                            .unwrap()
                    } else {
                        // msg!("zero coin vault {:#?}", coin_vault_account_info.key());
                        0u128
                    };

                    let price_reverse = if pc_vault_amount > 0 {
                        (coin_vault_amount as u128)
                            .checked_mul(PRICE_MULTIPLIER as u128)
                            .unwrap()
                            .checked_div(pc_vault_amount as u128)
                            .unwrap()
                    } else {
                        // msg!("zero pc vault {:#?}", coin_vault_account_info.key());
                        0u128
                    };

                    // println!("Standard Price: {:#?}", Price {
                    //   price_a_to_b: price,
                    //   price_b_to_a: price_reverse,
                    //   vault_a_amount: coin_vault_amount,
                    //   vault_b_amount: pc_vault_amount,
                    // });

                    pair.price = Some(Price {
                        price_a_to_b: price,
                        price_b_to_a: price_reverse,
                        vault_a_amount: Some(coin_vault_amount),
                        vault_b_amount: Some(pc_vault_amount),
                        active_tick_id: None,
                        pool_token_mint_a: None,
                        liquidity: None,
                        sqrt_price_x64: None,

                        config_denominator: None,
                        std_spread: None,
                        std_spread_buffer: None,
                        spread_coef: None,
                        base_decimal_value: None,
                        price_buffer_coin: None,
                    });
                }
            }
            SwapType::RaydiumConcentrated => {
                const POOL_OFFSET_UNTIL_PRICE: usize = 8 + 1 + 32 * 7 + 1 + 1 + 2 + 16 + 16 + 4;
                const POOL_OFFSET_UNTIL_LIQUIDITY: usize = 8 + 1 + 32 * 7 + 1 + 1 + 2;
                const TOKEN_MINT_A_OFFSET: usize = 8 + 1 + 32 * 2;

                if let Some(pool_state_account_info) = accounts_info.get(&pair.pool_state) {
                    let pool_state_data = &pool_state_account_info.account_data;
                    let input = array_ref![pool_state_data, 0, POOL_OFFSET_UNTIL_PRICE];
                    let (_, _liquidity_slice, sqrt_price_slice, tick_slice) =
                        array_refs![input, POOL_OFFSET_UNTIL_LIQUIDITY, 16, 16, 4];

                    let sqrt_price_x64 = u128::from_le_bytes(*sqrt_price_slice);
                    let liquidity = u128::from_le_bytes(*_liquidity_slice);
                    let active_tick_id = i32::from_le_bytes(*tick_slice);
                    let price = Self::get_price_from_sqrt_x64(sqrt_price_x64);
                    let price_reverse = (PRICE_MULTIPLIER as u128)
                        .checked_mul(PRICE_MULTIPLIER as u128)
                        .unwrap()
                        .checked_div(price)
                        .unwrap();
                    let token_a_mint_slice = array_ref![pool_state_data, TOKEN_MINT_A_OFFSET, 32];

                    pair.price = Some(Price {
                        price_a_to_b: price,
                        price_b_to_a: price_reverse,
                        vault_a_amount: None,
                        vault_b_amount: None,
                        active_tick_id: Some(active_tick_id),
                        pool_token_mint_a: Some(Pubkey::new_from_array(*token_a_mint_slice)),
                        liquidity: Some(liquidity),
                        sqrt_price_x64: Some(sqrt_price_x64),

                        config_denominator: None,
                        std_spread: None,
                        std_spread_buffer: None,
                        spread_coef: None,
                        base_decimal_value: None,
                        price_buffer_coin: None,
                    });
                }
            },
            SwapType::OrcaWhirlpool | SwapType::CropperWhirlpool => {
                const POOL_OFFSET_UNTIL_PRICE: usize = 8 + 32 + 1 + 2 + 2 + 2 + 2 + 16 + 16 + 4;
                const POOL_OFFSET_UNTIL_LIQUIDITY: usize = 8 + 32 + 1 + 2 + 2 + 2 + 2;
                const TOKEN_MINT_A_OFFSET: usize = 8 + 32 + 1 + 2 + 2 + 2 + 2 + 16 + 16 + 4 + 8 + 8;

                if let Some(pool_state_account_info) = accounts_info.get(&pair.pool_state) {
                    let pool_state_data = &pool_state_account_info.account_data;
                    let input = array_ref![pool_state_data, 0, POOL_OFFSET_UNTIL_PRICE];
                    let (_, _liquidity_slice, sqrt_price_slice, tick_slice) =
                        array_refs![input, POOL_OFFSET_UNTIL_LIQUIDITY, 16, 16, 4];

                    let sqrt_price_x64 = u128::from_le_bytes(*sqrt_price_slice);
                    let liquidity = u128::from_le_bytes(*_liquidity_slice);
                    let active_tick_id = i32::from_le_bytes(*tick_slice);
                    let price = Self::get_price_from_sqrt_x64(sqrt_price_x64);
                    let price_reverse = (PRICE_MULTIPLIER as u128)
                        .checked_mul(PRICE_MULTIPLIER as u128)
                        .unwrap()
                        .checked_div(price)
                        .unwrap();
                    let token_a_mint_slice = array_ref![pool_state_data, TOKEN_MINT_A_OFFSET, 32];

                    pair.price = Some(Price {
                        price_a_to_b: price,
                        price_b_to_a: price_reverse,
                        vault_a_amount: None,
                        vault_b_amount: None,
                        active_tick_id: Some(active_tick_id),
                        pool_token_mint_a: Some(Pubkey::new_from_array(*token_a_mint_slice)),
                        liquidity: Some(liquidity),
                        sqrt_price_x64: Some(sqrt_price_x64),

                        config_denominator: None,
                        std_spread: None,
                        std_spread_buffer: None,
                        spread_coef: None,
                        base_decimal_value: None,
                        price_buffer_coin: None,
                    });
                }
            },
            SwapType::InvariantSwap => {
                const POOL_OFFSET_UNTIL_LIQUIDITY: usize = 8 + 32 * 4 + 16 + 2 + 16 + 16;
                const POOL_OFFSET_UNTIL_PRICE: usize = POOL_OFFSET_UNTIL_LIQUIDITY + 16 + 16 + 4;
                const TOKEN_MINT_A_OFFSET: usize = 8;

                if let Some(pool_state_account_info) = accounts_info.get(&pair.pool_state) {
                    let pool_state_data = &pool_state_account_info.account_data;
                    let input = array_ref![pool_state_data, 0, POOL_OFFSET_UNTIL_PRICE];
                    let (_, _liquidity_slice, sqrt_price_slice, tick_slice) =
                        array_refs![input, POOL_OFFSET_UNTIL_LIQUIDITY, 16, 16, 4];

                    let sqrt_price_x64 = u128::from_le_bytes(*sqrt_price_slice);
                    let liquidity = u128::from_le_bytes(*_liquidity_slice);
                    let active_tick_id = i32::from_le_bytes(*tick_slice);
                    let price = Self::get_price_from_invariant_sqrt(sqrt_price_x64);
                    let price_reverse = (PRICE_MULTIPLIER as u128)
                        .checked_mul(PRICE_MULTIPLIER as u128)
                        .unwrap()
                        .checked_div(price)
                        .unwrap();
                    let token_a_mint_slice = array_ref![pool_state_data, TOKEN_MINT_A_OFFSET, 32];

                    pair.price = Some(Price {
                        price_a_to_b: price,
                        price_b_to_a: price_reverse,
                        vault_a_amount: None,
                        vault_b_amount: None,
                        active_tick_id: Some(active_tick_id),
                        pool_token_mint_a: Some(Pubkey::new_from_array(*token_a_mint_slice)),
                        liquidity: Some(liquidity),
                        sqrt_price_x64: Some(sqrt_price_x64),

                        config_denominator: None,
                        std_spread: None,
                        std_spread_buffer: None,
                        spread_coef: None,
                        base_decimal_value: None,
                        price_buffer_coin: None,
                    });
                }
            },
            SwapType::CremaFinance => {
                const POOL_OFFSET_AFTER_TICK: usize = 8 + 5 * 32 + 3 * 2 + 2 * 16 + 4;
                const POOL_OFFSET_UNTIL_LIQUIDITY: usize = 8 + 5 * 32 + 3 * 2;
                const TOKEN_MINT_A_OFFSET: usize = 8 + 32;

                if let Some(pool_state_account_info) = accounts_info.get(&pair.pool_state) {
                    let pool_state_data = &pool_state_account_info.account_data;

                    let input: &[u8; 210] = array_ref![pool_state_data, 0, POOL_OFFSET_AFTER_TICK];
                    let (_, _liquidity_slice, sqrt_price_slice, tick_slice) =
                        array_refs![input, POOL_OFFSET_UNTIL_LIQUIDITY, 16, 16, 4];

                    let sqrt_price_x64 = u128::from_le_bytes(*sqrt_price_slice);
                    let liquidity = u128::from_le_bytes(*_liquidity_slice);
                    let active_tick_id = i32::from_le_bytes(*tick_slice);

                    let price = Self::get_price_from_sqrt_x64(sqrt_price_x64);

                    let price_reverse = (PRICE_MULTIPLIER as u128)
                        .checked_mul(PRICE_MULTIPLIER as u128)
                        .unwrap()
                        .checked_div(price)
                        .unwrap();
                    let token_a_mint_slice = array_ref![pool_state_data, TOKEN_MINT_A_OFFSET, 32];

                    pair.price = Some(Price {
                        price_a_to_b: price,
                        price_b_to_a: price_reverse,
                        vault_a_amount: None,
                        vault_b_amount: None,
                        active_tick_id: Some(active_tick_id),
                        pool_token_mint_a: Some(Pubkey::new_from_array(*token_a_mint_slice)),
                        liquidity: Some(liquidity),
                        sqrt_price_x64: Some(sqrt_price_x64),

                        config_denominator: None,
                        std_spread: None,
                        std_spread_buffer: None,
                        spread_coef: None,
                        base_decimal_value: None,
                        price_buffer_coin: None,
                    });
                }
            },
            SwapType::MeteoraDlmm => {
                pub const FEE_PRECISION: u64 = 1_000_000_000;
                const POOL_OFFSET_AFTER_BIN: usize = 8 + 4 * 8 + 4 * 8 + 4 + 4 + 2;
                const POOL_OFFSET_UNTIL_BIN: usize = 8 + 4 * 8 + 4 * 8 + 4;
                const BASE_FACTOR_OFFSET: usize = 8;
                const PROTOCOL_SHARE_OFFSET: usize = 8 + 2 * 4 + 4 * 4;
                const VOLATILITY_ACCUMULATOR_OFFSET: usize = 8 + 4 * 8;
                const VARIABLE_FEE_CONTROL_OFFSET: usize = 8 + 2 * 4;
                const TOKEN_MINT_A_OFFSET: usize = 8 + 4 * 8 + 4 * 8 + 4 + 4 + 2 + 1 + 1 + 2 + 1 + 1;

                if let Some(pool_state_account_info) = accounts_info.get(&pair.pool_state) {
                    let pool_state_data = &pool_state_account_info.account_data;

                    let input = array_ref![pool_state_data, 0, POOL_OFFSET_AFTER_BIN];
                    let (_, active_id_slice, bin_step_slice) = array_refs![input, POOL_OFFSET_UNTIL_BIN, 4, 2];

                    let active_id = i32::from_le_bytes(*active_id_slice);
                    let bin_step = u16::from_le_bytes(*bin_step_slice);
                    if let Ok(price_x64) = Self::get_price_from_id(active_id, bin_step) {
                        let price = Self::get_price_from_x64(price_x64);
                        let price_reverse = (PRICE_MULTIPLIER as u128)
                            .checked_mul(PRICE_MULTIPLIER as u128)
                            .unwrap()
                            .checked_div(price)
                            .unwrap();

                        let volatility_accumulator_slice = array_ref![pool_state_data, VOLATILITY_ACCUMULATOR_OFFSET, 4];
                        let variable_fee_control_slice = array_ref![pool_state_data, VARIABLE_FEE_CONTROL_OFFSET, 4];
                        let base_factor_slice = array_ref![pool_state_data, BASE_FACTOR_OFFSET, 2];
                
                        let volatility_accumulator = u32::from_le_bytes(*volatility_accumulator_slice);
                        let variable_fee_control = u32::from_le_bytes(*variable_fee_control_slice);
                        let base_factor = u16::from_le_bytes(*base_factor_slice);

                        let dlmm_fee = Self::get_total_fee(volatility_accumulator, variable_fee_control, base_factor, bin_step).unwrap();
                        let fee_percent = dlmm_fee
                            .checked_mul(FEE_MULTIPLIER as u128)
                            .unwrap()
                            .checked_div(FEE_PRECISION as u128)
                            .unwrap() as u64;
                        pair.fee = fee_percent as u32;
                        let token_a_mint_slice = array_ref![pool_state_data, TOKEN_MINT_A_OFFSET, 32];
                        let protocol_share_slice = array_ref![pool_state_data, PROTOCOL_SHARE_OFFSET, 2];
                        let protocol_share = u16::from_le_bytes(*protocol_share_slice);

                        pair.price = Some(Price {
                            price_a_to_b: price,
                            price_b_to_a: price_reverse,
                            vault_a_amount: None,
                            vault_b_amount: None,
                            active_tick_id: Some(active_id),
                            pool_token_mint_a: Some(Pubkey::new_from_array(*token_a_mint_slice)),
                            liquidity: None,
                            sqrt_price_x64: Some(price_x64),

                            config_denominator: Some(protocol_share as u64),
                            std_spread: Some(volatility_accumulator as u64),
                            std_spread_buffer: Some(variable_fee_control as u64),
                            spread_coef: Some(base_factor as u64),
                            base_decimal_value: Some(bin_step as u64),
                            price_buffer_coin: Some(active_id as i64),
                        });
                    }
                }
            },
            _ => {
              
            }
        };
        trace!();
        // println!("build_pair_price done!");
    }

    pub fn get_total_fee(volatility_accumulator: u32, variable_fee_control: u32, base_factor: u16, bin_step: u16) -> Result<u128, SharedMemClientErr> {
        pub const MAX_FEE_RATE: u64 = 100_000_000;
        let total_fee_rate = Self::get_base_fee(base_factor, bin_step)?.safe_add(Self::get_variable_fee(volatility_accumulator, variable_fee_control, bin_step)?)?;
        let total_fee_rate_cap = std::cmp::min(total_fee_rate, MAX_FEE_RATE.into());
        Ok(total_fee_rate_cap)
    }

    pub fn get_base_fee(base_factor: u16, bin_step: u16) -> Result<u128, SharedMemClientErr> {
        Ok(u128::from(base_factor)
            .safe_mul(bin_step.into())?
            // Make it to be the same as FEE_PRECISION defined for ceil_div later on.
            .safe_mul(10u128)?)
    }

    pub fn get_variable_fee(volatility_accumulator: u32, variable_fee_control: u32, bin_step: u16) -> Result<u128, SharedMemClientErr> {
        Self::compute_variable_fee(volatility_accumulator, variable_fee_control, bin_step)
    }

    pub fn compute_variable_fee(volatility_accumulator: u32, variable_fee_control: u32, bin_step: u16) -> Result<u128, SharedMemClientErr> {
        if variable_fee_control > 0 {
            let volatility_accumulator: u128 = volatility_accumulator.into();
            let bin_step: u128 = bin_step.into();
            let variable_fee_control: u128 = variable_fee_control.into();
    
            let square_vfa_bin = volatility_accumulator
                .safe_mul(bin_step)?
                .checked_pow(2)
                .ok_or(SharedMemClientErr::MathOverflowError("MathOverflow".to_string()))?;
    
            // Variable fee control, volatility accumulator, bin step are in basis point unit (10_000)
            // This is 1e20. Which > 1e9. Scale down it to 1e9 unit and ceiling the remaining.
            let v_fee = variable_fee_control.safe_mul(square_vfa_bin)?;
    
            let scaled_v_fee = v_fee.safe_add(99_999_999_999)?.safe_div(100_000_000_000)?;
            return Ok(scaled_v_fee);
        }
    
        Ok(0)
    }

    pub fn get_price_from_id(active_id: i32, bin_step: u16) -> Result<u128, SharedMemClientErr> {
        pub const SCALE_OFFSET: u8 = 64;
        pub const ONE: u128 = 1u128 << SCALE_OFFSET;
        pub const BASIS_POINT_MAX: i32 = 10000;

        // Make bin_step into Q64x64, and divided by BASIS_POINT_MAX. If bin_step = 1, we get 0.0001 in Q64x64
        let bps = u128::from(bin_step)
            .safe_shl(SCALE_OFFSET.into())?
            .safe_div(BASIS_POINT_MAX as u128)?;
        // Add 1 to bps, we get 1.0001 in Q64.64
        let base = ONE.safe_add(bps)?;
        Self::pow(base, active_id).ok_or_else(|| SharedMemClientErr::MathOverflowError("MathOverflow".to_string()))
    }

    pub fn pow(base: u128, exp: i32) -> Option<u128> {
        pub const SCALE_OFFSET: u8 = 64;
        const MAX_EXPONENTIAL: u32 = 0x80000; // 1048576
        pub const ONE: u128 = 1u128 << SCALE_OFFSET;

        // If exponent is negative. We will invert the result later by 1 / base^exp.abs()
        let mut invert = exp.is_negative();
    
        // When exponential is 0, result will always be 1
        if exp == 0 {
            return Some(1u128 << 64);
        }
    
        // Make the exponential positive. Which will compute the result later by 1 / base^exp
        let exp: u32 = if invert { exp.abs() as u32 } else { exp as u32 };
    
        // No point to continue the calculation as it will overflow the maximum value Q64.64 can support
        if exp >= MAX_EXPONENTIAL {
            return None;
        }
    
        let mut squared_base = base;
        let mut result = ONE;
    
        // When multiply the base twice, the number of bits double from 128 -> 256, which overflow.
        // The trick here is to inverse the calculation, which make the upper 64 bits (number bits) to be 0s.
        // For example:
        // let base = 1.001, exp = 5
        // let neg = 1 / (1.001 ^ 5)
        // Inverse the neg: 1 / neg
        // By using a calculator, you will find out that 1.001^5 == 1 / (1 / 1.001^5)
        if squared_base >= result {
            // This inverse the base: 1 / base
            squared_base = u128::MAX.checked_div(squared_base)?;
            // If exponent is negative, the above already inverted the result. Therefore, at the end of the function, we do not need to invert again.
            invert = !invert;
        }
    
        // The following code is equivalent to looping through each binary value of the exponential.
        // As explained in MAX_EXPONENTIAL, 19 exponential bits are enough to covert the full bin price.
        // Therefore, there will be 19 if statements, which similar to the following pseudo code.
        /*
            let mut result = 1;
            while exponential > 0 {
                if exponential & 1 > 0 {
                    result *= base;
                }
                base *= base;
                exponential >>= 1;
            }
        */
    
        // From right to left
        // squared_base = 1 * base^1
        // 1st bit is 1
        if exp & 0x1 > 0 {
            result = (result.checked_mul(squared_base)?) >> SCALE_OFFSET
        }
    
        // squared_base = base^2
        squared_base = (squared_base.checked_mul(squared_base)?) >> SCALE_OFFSET;
        // 2nd bit is 1
        if exp & 0x2 > 0 {
            result = (result.checked_mul(squared_base)?) >> SCALE_OFFSET
        }
    
        // Example:
        // If the base is 1.001, exponential is 3. Binary form of 3 is ..0011. The last 2 1's bit fulfill the above 2 bitwise condition.
        // The result will be 1 * base^1 * base^2 == base^3. The process continues until reach the 20th bit
    
        squared_base = (squared_base.checked_mul(squared_base)?) >> SCALE_OFFSET;
        if exp & 0x4 > 0 {
            result = (result.checked_mul(squared_base)?) >> SCALE_OFFSET
        }
    
        squared_base = (squared_base.checked_mul(squared_base)?) >> SCALE_OFFSET;
        if exp & 0x8 > 0 {
            result = (result.checked_mul(squared_base)?) >> SCALE_OFFSET
        }
    
        squared_base = (squared_base.checked_mul(squared_base)?) >> SCALE_OFFSET;
        if exp & 0x10 > 0 {
            result = (result.checked_mul(squared_base)?) >> SCALE_OFFSET
        }
    
        squared_base = (squared_base.checked_mul(squared_base)?) >> SCALE_OFFSET;
        if exp & 0x20 > 0 {
            result = (result.checked_mul(squared_base)?) >> SCALE_OFFSET
        }
    
        squared_base = (squared_base.checked_mul(squared_base)?) >> SCALE_OFFSET;
        if exp & 0x40 > 0 {
            result = (result.checked_mul(squared_base)?) >> SCALE_OFFSET
        }
    
        squared_base = (squared_base.checked_mul(squared_base)?) >> SCALE_OFFSET;
        if exp & 0x80 > 0 {
            result = (result.checked_mul(squared_base)?) >> SCALE_OFFSET
        }
    
        squared_base = (squared_base.checked_mul(squared_base)?) >> SCALE_OFFSET;
        if exp & 0x100 > 0 {
            result = (result.checked_mul(squared_base)?) >> SCALE_OFFSET
        }
    
        squared_base = (squared_base.checked_mul(squared_base)?) >> SCALE_OFFSET;
        if exp & 0x200 > 0 {
            result = (result.checked_mul(squared_base)?) >> SCALE_OFFSET
        }
    
        squared_base = (squared_base.checked_mul(squared_base)?) >> SCALE_OFFSET;
        if exp & 0x400 > 0 {
            result = (result.checked_mul(squared_base)?) >> SCALE_OFFSET
        }
    
        squared_base = (squared_base.checked_mul(squared_base)?) >> SCALE_OFFSET;
        if exp & 0x800 > 0 {
            result = (result.checked_mul(squared_base)?) >> SCALE_OFFSET
        }
    
        squared_base = (squared_base.checked_mul(squared_base)?) >> SCALE_OFFSET;
        if exp & 0x1000 > 0 {
            result = (result.checked_mul(squared_base)?) >> SCALE_OFFSET
        }
    
        squared_base = (squared_base.checked_mul(squared_base)?) >> SCALE_OFFSET;
        if exp & 0x2000 > 0 {
            result = (result.checked_mul(squared_base)?) >> SCALE_OFFSET
        }
    
        squared_base = (squared_base.checked_mul(squared_base)?) >> SCALE_OFFSET;
        if exp & 0x4000 > 0 {
            result = (result.checked_mul(squared_base)?) >> SCALE_OFFSET
        }
    
        squared_base = (squared_base.checked_mul(squared_base)?) >> SCALE_OFFSET;
        if exp & 0x8000 > 0 {
            result = (result.checked_mul(squared_base)?) >> SCALE_OFFSET
        }
    
        squared_base = (squared_base.checked_mul(squared_base)?) >> SCALE_OFFSET;
        if exp & 0x10000 > 0 {
            result = (result.checked_mul(squared_base)?) >> SCALE_OFFSET
        }
    
        squared_base = (squared_base.checked_mul(squared_base)?) >> SCALE_OFFSET;
        if exp & 0x20000 > 0 {
            result = (result.checked_mul(squared_base)?) >> SCALE_OFFSET
        }
    
        squared_base = (squared_base.checked_mul(squared_base)?) >> SCALE_OFFSET;
        if exp & 0x40000 > 0 {
            result = (result.checked_mul(squared_base)?) >> SCALE_OFFSET
        }
    
        // Stop here as the next is 20th bit, which > MAX_EXPONENTIAL
        if result == 0 {
            return None;
        }
    
        if invert {
            result = u128::MAX.checked_div(result)?;
        }
    
        Some(result)
    }
    
    pub fn get_price_from_x64(price_x64: u128) -> u128 {
        pub const Q64: u128 = (u64::MAX as u128) + 1; // 2^64
        U128::from(price_x64)
            .mul_div_floor(
                U128::from(PRICE_MULTIPLIER),
                U128::from(Q64),
            )
            .unwrap()
            .as_u128()
    }

    pub fn get_price_from_sqrt_x64(sqrt_price_x64: u128) -> u128 {
        pub const Q64: u128 = (u64::MAX as u128) + 1; // 2^64
        pub const RESOLUTION: u8 = 64;
        let price = U128::from(sqrt_price_x64)
            .mul_div_floor(U128::from(sqrt_price_x64), U128::from(Q64))
            .unwrap()
            .mul_div_floor(
                U128::from(PRICE_MULTIPLIER),
                U128::from(Q64),
            )
            .unwrap()
            .as_u128();
        return price;
    }

    pub fn get_price_from_invariant_sqrt(sqrt_price: u128) -> u128 {
        pub const INVARIANT_PRICE_MULTIPLIER: u128 = 1_000000_000000_000000_000000;
        U128::from(sqrt_price)
            .mul_div_floor(U128::from(sqrt_price), U128::from(INVARIANT_PRICE_MULTIPLIER))
            .unwrap()
            .mul_div_floor(U128::from(PRICE_MULTIPLIER), U128::from(INVARIANT_PRICE_MULTIPLIER))
            .unwrap()
            .as_u128()
    }

    pub fn get_supply_from_mint(mint_account_info: &[u8]) -> u64 {
        let src = array_ref![mint_account_info, 0, 82];
        let (_, supply, _, _, _) = array_refs![src, 36, 8, 1, 1, 36];
        let supply = u64::from_le_bytes(*supply);
        supply
    }

    pub fn get_amount_from_token_account(token_account_info: &[u8]) -> u64 {
        let input = array_ref![token_account_info, 0, 72];
        let (_, amount_slice) = array_refs![input, 64, 8];

        let vault_amount = u64::from_le_bytes(*amount_slice);

        vault_amount
    }

    pub fn get_meteora_pools_vault_withdrawable_amount(
        data: &[u8],
        current_time: u64,
    ) -> u64 {
        const PROFIT_TRACKER_OFFSET: usize = 8 + 3 + 8 + 32 * 4 + 32 * 30 + 32 * 3;
        pub const LOCKED_PROFIT_DEGRADATION_DENOMINATOR: u128 = 1_000_000_000_000;

        let total_amount_slice = array_ref![data, 8 + 3, 8];
        let last_updated_locked_profit_slice = array_ref![data, PROFIT_TRACKER_OFFSET, 8];
        let last_report_slice = array_ref![data, PROFIT_TRACKER_OFFSET + 8, 8];
        let locked_profit_degradation_slice = array_ref![data, PROFIT_TRACKER_OFFSET + 8 + 8, 8];

        let total_amount = u64::from_le_bytes(*total_amount_slice);
        let last_updated_locked_profit = u64::from_le_bytes(*last_updated_locked_profit_slice);
        let last_report = u64::from_le_bytes(*last_report_slice);
        let locked_profit_degradation = u64::from_le_bytes(*locked_profit_degradation_slice);

        let duration = u128::from(current_time.checked_sub(last_report).unwrap());
        let locked_profit_degradation = u128::from(locked_profit_degradation);
        let locked_fund_ratio = duration.checked_mul(locked_profit_degradation).unwrap();

        let locked_profit = if locked_fund_ratio < LOCKED_PROFIT_DEGRADATION_DENOMINATOR {
            u128::from(last_updated_locked_profit)
                .checked_mul(LOCKED_PROFIT_DEGRADATION_DENOMINATOR - locked_fund_ratio)
                .unwrap()
                .checked_div(LOCKED_PROFIT_DEGRADATION_DENOMINATOR)
                .unwrap() as u64
        } else {
            0u64
        };

        let unlocked_amount = total_amount.checked_sub(locked_profit).unwrap();
        unlocked_amount
    }

    pub fn calc_amount_out(
        &self,
        pair_idx: usize,
        input_amount: u64,
        from_mint: Pubkey,
    ) -> (u64, u128, u128) {
        trace!();
        let pairs = unsafe { &*self.bank_bot.pairs.load(Ordering::SeqCst) };
        let pair = &pairs[pair_idx];

        if pair.price.as_ref().is_none() {
            // println!("calc_amount_out: None pair-> pair_idx: {}", pair_idx);
            return (0, 0, 0);
        }

        match pair.swap_type {
            SwapType::RaydiumPoolV4 | SwapType::OrcaSwapV2 | SwapType::TokenSwap | SwapType::CropperFinance | SwapType::DooarSwap | SwapType::DexlabSwap | SwapType::SarosAmm | SwapType::Fluxbeam | SwapType::AldrinAmm | SwapType::AldrinAmmV2 | SwapType::PenguinFinance | SwapType::StepFinance | SwapType::RaydiumCpmm | SwapType::BonkSwap | SwapType::MeteoraPools => {
                let vault_a_amount = pair.price.as_ref().unwrap().vault_a_amount;
                let vault_b_amount = pair.price.as_ref().unwrap().vault_b_amount;
        
                if vault_a_amount.unwrap() == 0 || vault_b_amount.unwrap() == 0 {
                    // println!("calc_amount_out: zero amount pair-> pair_idx: {}", pair_idx);
                    return (0, 0, 0);
                }
        
                let mut _out_amount = 0u64;
                let mut _next_vault_a_amount = 0u64;
                let mut _next_vault_b_amount = 0u64;
                let mut _next_price = 0u128;
                let mut _next_price_reversed = 0u128;
        
                let fee_percent = pair.fee;
        
                if from_mint == pair.mint_a {
                    _next_vault_a_amount = vault_a_amount.unwrap()
                        .checked_add(
                            (input_amount as u128)
                                .checked_mul(fee_percent as u128)
                                .unwrap()
                                .checked_div(FEE_MULTIPLIER as u128)
                                .unwrap() as u64,
                        )
                        .unwrap();
        
                    _next_vault_b_amount = (vault_a_amount.unwrap() as u128)
                        .checked_mul(vault_b_amount.unwrap() as u128)
                        .unwrap()
                        .checked_div(_next_vault_a_amount as u128)
                        .unwrap() as u64;
        
                    _out_amount = vault_b_amount.unwrap().checked_sub(_next_vault_b_amount).unwrap();
        
                    let next_vault_a_amount_no_fee = vault_a_amount.unwrap().checked_add(input_amount).unwrap();
        
                    _next_price = (_next_vault_b_amount as u128)
                        .checked_mul(PRICE_MULTIPLIER as u128)
                        .unwrap()
                        .checked_div(next_vault_a_amount_no_fee as u128)
                        .unwrap() as u128;
                    _next_price_reversed = (next_vault_a_amount_no_fee as u128)
                        .checked_mul(PRICE_MULTIPLIER as u128)
                        .unwrap()
                        .checked_div(_next_vault_b_amount as u128)
                        .unwrap() as u128;
                } else {
                    _next_vault_b_amount = vault_b_amount.unwrap()
                        .checked_add(
                            (input_amount as u128)
                                .checked_mul(fee_percent as u128)
                                .unwrap()
                                .checked_div(FEE_MULTIPLIER as u128)
                                .unwrap() as u64,
                        )
                        .unwrap();
                    _next_vault_a_amount = (vault_b_amount.unwrap() as u128)
                        .checked_mul(vault_a_amount.unwrap() as u128)
                        .unwrap()
                        .checked_div(_next_vault_b_amount as u128)
                        .unwrap() as u64;
        
                    _out_amount = vault_a_amount.unwrap().checked_sub(_next_vault_a_amount).unwrap();
        
                    let next_vault_b_amount_no_fee = vault_b_amount.unwrap().checked_add(input_amount).unwrap();
                    _next_price = (_next_vault_a_amount as u128)
                        .checked_mul(PRICE_MULTIPLIER as u128)
                        .unwrap()
                        .checked_div(next_vault_b_amount_no_fee as u128)
                        .unwrap();
                    _next_price_reversed = (next_vault_b_amount_no_fee as u128)
                        .checked_mul(PRICE_MULTIPLIER as u128)
                        .unwrap()
                        .checked_div(_next_vault_a_amount as u128)
                        .unwrap();
                }
                trace!();
                (_out_amount, _next_price, _next_price_reversed)
            },
            SwapType::MeteoraDlmm =>{
                let price = pair.price.as_ref().unwrap();
                let protocol_share = price.config_denominator.unwrap() as u16;
                let volatility_accumulator = price.std_spread.unwrap() as u32;
                let variable_fee_control = price.std_spread_buffer.unwrap() as u32;
                let base_factor = price.spread_coef.unwrap() as u16;
                let bin_step = price.base_decimal_value.unwrap() as u16;
                let active_id = price.price_buffer_coin.unwrap() as i32;
                (0, 0, 0)
            },
            _ => {
                (0, 0, 0)
            }
        }

        
    }

    pub fn get_acc_price(&self, route_idx: usize) -> u128 {
        let routes = unsafe { &*self.bank_bot.routes.load(Ordering::SeqCst) };
        let pairs = unsafe { &*self.bank_bot.pairs.load(Ordering::SeqCst) };
        let route = &routes[route_idx];

        let tokens = vec![
            pairs[route.route[0] as usize].mint_a,
            pairs[route.route[0] as usize].mint_b,
            pairs[route.route[1] as usize].mint_a,
            pairs[route.route[1] as usize].mint_b,
        ];
        let route_start_mint = Route::find_first_mint(&tokens).unwrap();

        let mut acc_price: u128 = PRICE_MULTIPLIER as u128;
        let mut from_mint = route_start_mint;
        trace!();
        for pair_idx in route.route.iter() {
            trace!();
            let pair = &pairs[*pair_idx as usize];
            let fee = pair.fee;
            if pair.price.as_ref().is_none() || pair.price.as_ref().is_none() {
                // println!("get_acc_price: None pair-> pair_idx: {}", pair_idx);
                acc_price = 0;
                break;
            }
            let price_a_to_b = pair.price.as_ref().unwrap().price_a_to_b;
            let price_b_to_a = pair.price.as_ref().unwrap().price_b_to_a;
            if price_a_to_b == 0 || price_b_to_a == 0 {
                // println!("get_acc_price: zero pair price-> pair_idx: {}", pair_idx);
                acc_price = 0;
                break;
            }

            if pair.mint_a == from_mint {
                acc_price = if price_a_to_b > price_b_to_a {
                    acc_price
                        .checked_mul(fee as u128)
                        .unwrap()
                        .checked_div(FEE_MULTIPLIER as u128)
                        .unwrap()
                        .checked_mul(price_a_to_b)
                        .unwrap()
                        .checked_div(PRICE_MULTIPLIER as u128)
                        .unwrap()
                } else {
                    acc_price
                        .checked_mul(fee as u128)
                        .unwrap()
                        .checked_div(FEE_MULTIPLIER as u128)
                        .unwrap()
                        .checked_mul(PRICE_MULTIPLIER as u128)
                        .unwrap()
                        .checked_div(price_b_to_a)
                        .unwrap()
                };
                from_mint = pair.mint_b;
            } else {
                acc_price = if price_a_to_b > price_b_to_a {
                    acc_price
                        .checked_mul(fee as u128)
                        .unwrap()
                        .checked_div(FEE_MULTIPLIER as u128)
                        .unwrap()
                        .checked_mul(PRICE_MULTIPLIER as u128)
                        .unwrap()
                        .checked_div(price_a_to_b)
                        .unwrap()
                } else {
                    acc_price
                        .checked_mul(fee as u128)
                        .unwrap()
                        .checked_div(FEE_MULTIPLIER as u128)
                        .unwrap()
                        .checked_mul(price_b_to_a as u128)
                        .unwrap()
                        .checked_div(PRICE_MULTIPLIER as u128)
                        .unwrap()
                };
                from_mint = pair.mint_a;
                trace!();
            }
        }
        // println!("acc_price: {}", acc_price);
        trace!();
        acc_price
    }

    pub fn calc_next_price(&self, route_idx: usize, input_amount: u64) -> (u128, u64) {
        let routes = unsafe { &*self.bank_bot.routes.load(Ordering::SeqCst) };
        let pairs = unsafe { &*self.bank_bot.pairs.load(Ordering::SeqCst) };
        let route = &routes[route_idx];
        trace!();

        let mut mid_amount = input_amount;
        let mut acc_price = PRICE_MULTIPLIER as u128;
        let mut from_mint = route.route_start_mint;

        for pair_idx in route.route.iter() {
            trace!();
            let pair = &pairs[*pair_idx as usize];
            let fee = pair.fee;
            let (out_amount, next_price, next_price_reversed) =
                self.calc_amount_out(*pair_idx as usize, mid_amount, from_mint);
            if next_price == 0 && next_price_reversed == 0 {
                trace!();
                acc_price = u128::MAX;
                break;
            } else {
                trace!();
                acc_price = if next_price > next_price_reversed {
                    acc_price
                        .checked_mul(fee as u128)
                        .unwrap()
                        .checked_div(FEE_MULTIPLIER as u128)
                        .unwrap()
                        .checked_mul(next_price)
                        .unwrap()
                        .checked_div(PRICE_MULTIPLIER as u128)
                        .unwrap()
                } else {
                    acc_price
                        .checked_mul(fee as u128)
                        .unwrap()
                        .checked_div(FEE_MULTIPLIER as u128)
                        .unwrap()
                        .checked_mul(PRICE_MULTIPLIER as u128)
                        .unwrap()
                        .checked_div(next_price_reversed)
                        .unwrap()
                };
            }
            mid_amount = out_amount;
            from_mint = if pair.mint_a == from_mint {
                pair.mint_b
            } else {
                pair.mint_a
            };
        }
        trace!();
        (acc_price, mid_amount)
    }

    pub fn determine_input_amount(
        &self,
        min_input: u64,
        max_input: u64,
        route_idx: usize,
    ) -> (u64, u128, u64) {
        trace!();
        // println!("determine_input_amount start");
        // println!("min_input: {}, max_input: {}, route_idx: {}", min_input, max_input, route_idx);
        let mut _next_price = 0u128;
        let mut min_amount = min_input;
        let mut bound_amount = min_input.checked_mul(10).unwrap();
        trace!();
        loop {
            trace!();
            (_next_price, _) = self.calc_next_price(route_idx, bound_amount);

            if _next_price < PRICE_MULTIPLIER as u128 || bound_amount >= max_input {
                break;
            }
            min_amount = bound_amount;
            bound_amount = bound_amount.checked_mul(10).unwrap();
        }

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
        let mut out_amount: u64 = 0;

        // println!("min")
        loop {
            trace!();
            mid_amount = min_amount + (max_amount - min_amount) / 2;
            
            if max_amount - min_amount < allowed_amount_range {
                break;
            }

            (_next_price, out_amount) = self.calc_next_price(route_idx, mid_amount);

            if _next_price >= PRICE_MULTIPLIER as u128 {
                let price_range = _next_price - PRICE_MULTIPLIER as u128;
                if price_range <= ALLOWED_PRICE_CHANGE {
                    break;
                }
                min_amount = mid_amount;
            } else {
                max_amount = mid_amount;
            }
        }
        trace!();
        // println!("determine_input_amount end");
        // println!("(mid_amount: {}, _next_price: {}, out_amount: {}, pnl: {})", mid_amount, _next_price, out_amount, if mid_amount > out_amount {mid_amount - out_amount} else {out_amount - mid_amount});
        (mid_amount, _next_price, out_amount)
    }
}

#[derive(Debug)]
pub struct Pair {
    pub price: Option<Price>,
    pub swap_type: SwapType,
    pub pool: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub fee: u32,
    pub data_index_a_to_b: u8,
    pub data_index_b_to_a: u8,
    pub accounts_vec_a_to_b: [AccountMeta; 25],
    pub accounts_vec_b_to_a: [AccountMeta; 25],
    pub user_vault_a: Pubkey,
    pub user_vault_b: Pubkey,
    pub pool_state: Pubkey,
    pub pool_vault_a: Pubkey,
    pub pool_vault_b: Pubkey,
    pub mint_a_decimals: u8,
    pub mint_b_decimals: u8,
    pub mint_a_token_program: Pubkey,
    pub mint_b_token_program: Pubkey,
}

impl Pair {
    pub const PAIR_SLICE_SIZE: usize = 2029; //1+32+32+32+4+1+1+34×50+32+32+32+32+32+1+1+32+32
                                             // swap_type + pool + mint_a + mint_b + fee + data_index_a_to_b, data_index_b_to_a, accounts_vec_a_to_b, accounts_vec_b_to_a, user_vault_a, user_vault_b, pool_state, pool_vault_a, pool_vault_b, mint_a_decimals, mint_b_decimals, mint_a_token_program, mint_b_token_program

    pub fn deserialize(buf: &[u8]) -> Self {
        let (
            swap_type,
            pool_slice,
            mint_a_slice,
            mint_b_slice,
            fee_slice,
            data_index_a_to_b,
            data_index_b_to_a,
            accounts_vec_a_to_b_slice,
            accounts_vec_b_to_a_slice,
            user_vault_a_slice,
            user_vault_b_slice,
            pool_state_slice,
            pool_vault_a_slice,
            pool_vault_b_slice,
            mint_a_decimals,
            mint_b_decimals,
            mint_a_token_program_slice,
            mint_b_token_program_slice,
        ) = array_refs![
            buf.try_into().unwrap(),
            1,
            32,
            32,
            32,
            4,
            1,
            1,
            850,
            850,
            32,
            32,
            32,
            32,
            32,
            1,
            1,
            32,
            32
        ];

        let mut accounts_vec_a_to_b: Vec<AccountMeta> = Vec::with_capacity(25);
        for account_meta_slice in accounts_vec_a_to_b_slice.chunks(34) {
            let (pubkey, is_signer, is_writable) =
                array_refs![account_meta_slice.try_into().unwrap(), 32, 1, 1];
            accounts_vec_a_to_b.push(AccountMeta {
                pubkey: Pubkey::new_from_array(*pubkey),
                is_signer: is_signer[0] == 1,
                is_writable: is_writable[0] == 1,
            });
        }

        let mut accounts_vec_b_to_a: Vec<AccountMeta> = Vec::with_capacity(25);
        for account_meta_slice in accounts_vec_b_to_a_slice.chunks(34) {
            let (pubkey, is_signer, is_writable) =
                array_refs![account_meta_slice.try_into().unwrap(), 32, 1, 1];
            accounts_vec_b_to_a.push(AccountMeta {
                pubkey: Pubkey::new_from_array(*pubkey),
                is_signer: is_signer[0] == 1,
                is_writable: is_writable[0] == 1,
            });
        }

        Pair {
            price: None,
            swap_type: SwapType::try_from(swap_type[0]).unwrap(),
            pool: Pubkey::new_from_array(*pool_slice),
            mint_a: Pubkey::new_from_array(*mint_a_slice),
            mint_b: Pubkey::new_from_array(*mint_b_slice),
            fee: FEE_MULTIPLIER - u32::from_le_bytes(*fee_slice),
            data_index_a_to_b: data_index_a_to_b[0],
            data_index_b_to_a: data_index_b_to_a[0],
            accounts_vec_a_to_b: accounts_vec_a_to_b.try_into().unwrap(),
            accounts_vec_b_to_a: accounts_vec_b_to_a.try_into().unwrap(),
            user_vault_a: Pubkey::new_from_array(*user_vault_a_slice),
            user_vault_b: Pubkey::new_from_array(*user_vault_b_slice),
            pool_state: Pubkey::new_from_array(*pool_state_slice),
            pool_vault_a: Pubkey::new_from_array(*pool_vault_a_slice),
            pool_vault_b: Pubkey::new_from_array(*pool_vault_b_slice),
            mint_a_decimals: mint_a_decimals[0],
            mint_b_decimals: mint_b_decimals[0],
            mint_a_token_program: Pubkey::new_from_array(*mint_a_token_program_slice),
            mint_b_token_program: Pubkey::new_from_array(*mint_b_token_program_slice),
        }
    }

    pub fn get_all_accounts_for_subscribe(&self) -> Vec<(Pubkey, bool)> {
        if self.swap_type == SwapType::BonkSwap {
            vec![
                (self.pool_state, true)
            ]
        } else if self.swap_type == SwapType::MeteoraPools {
            vec![
                (self.accounts_vec_a_to_b[8].pubkey, false),
                (self.accounts_vec_a_to_b[9].pubkey, false),
                (self.accounts_vec_a_to_b[10].pubkey, false),
                (self.accounts_vec_a_to_b[11].pubkey, false),
                (self.pool_vault_a, true),
                (self.pool_vault_b, true)
            ]
        } else {
            vec![
                (self.pool_vault_a, true),
                (self.pool_vault_b, true)
            ]
        }
    }
}

#[derive(Copy, Clone, PartialEq, BorshSerialize, BorshDeserialize, Debug)]
pub enum SwapType {
    None,

    // sprint1
    RaydiumPoolV4,
    OrcaSwapV2,
    TokenSwap,
    CropperFinance,
    DooarSwap,
    DexlabSwap,
    SarosAmm,
    BonkSwap,
    Fluxbeam,
    AldrinAmm,
    AldrinAmmV2,
    PenguinFinance,
    StepFinance,
    RaydiumCpmm,

    // sprint1 - extra
    MeteoraPools,
    LifinitySwapV2,

    // sprint2 concentrated whirlpools
    RaydiumConcentrated,
    OrcaWhirlpool,
    CropperWhirlpool,
    InvariantSwap,
    CremaFinance,
    MeteoraDlmm,

    //sprint3 we don't swap with stable but use it for bridge
    RaydiumPoolV4Stable,
    OrcaSwapV2Stable,
    OrcaSwapV2Offset,
    TokenSwapStable,
    TokenSwapOffset,
    BonkSwapStable,
    AldrinAmmV2Stable,
    MeteoraPoolsStable,
    SaberStableSwap,
    MercurialStableSwap,

    //sprint4 unimplemented
    PumpFun,
    HeliumTreasury,
    OpenbookV2,
    MarginFiV2,
    Kamino,
    KaminoLending,
    SanctumRouter,
    GooseFxV2,
}

impl Default for SwapType {
    fn default() -> Self {
        Self::None
    }
}
impl fmt::Display for SwapType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SwapType::None => write!(f, "None"),
            SwapType::RaydiumConcentrated => write!(f, "RaydiumConcentrated"),
            SwapType::OrcaWhirlpool => write!(f, "OrcaWhirlpool"),
            SwapType::CropperWhirlpool => write!(f, "CropperWhirlpool"),
            SwapType::RaydiumPoolV4 => write!(f, "RaydiumPoolV4"),
            SwapType::OrcaSwapV2 => write!(f, "OrcaSwapV2"),
            SwapType::MeteoraDlmm => write!(f, "MeteoraDlmm"),
            SwapType::MeteoraPools => write!(f, "MeteoraPools"),
            SwapType::LifinitySwapV2 => write!(f, "LifinitySwapV2"),
            SwapType::InvariantSwap => write!(f, "InvariantSwap"),
            SwapType::MercurialStableSwap => write!(f, "MercurialStableSwap"),
            SwapType::DooarSwap => write!(f, "DooarSwap"),
            SwapType::DexlabSwap => write!(f, "DexlabSwap"),
            SwapType::SarosAmm => write!(f, "SarosAmm"),
            SwapType::BonkSwap => write!(f, "BonkSwap"),
            SwapType::Fluxbeam => write!(f, "Fluxbeam"),
            SwapType::GooseFxV2 => write!(f, "GooseFxV2"),
            SwapType::CremaFinance => write!(f, "CremaFinance"),
            SwapType::AldrinAmmV2 => write!(f, "AldrinAmmV2"),
            SwapType::AldrinAmm => write!(f, "AldrinAmm"),
            SwapType::CropperFinance => write!(f, "CropperFinance"),
            SwapType::TokenSwap => write!(f, "TokenSwap"),
            SwapType::PenguinFinance => write!(f, "PenguinFinance"),
            SwapType::StepFinance => write!(f, "StepFinance"),
            SwapType::SaberStableSwap => write!(f, "SaberStableSwap"),
            SwapType::PumpFun => write!(f, "PumpFun"),
            SwapType::HeliumTreasury => write!(f, "HeliumTreasury"),
            SwapType::OpenbookV2 => write!(f, "OpenbookV2"),
            SwapType::MarginFiV2 => write!(f, "MarginFiV2"),
            SwapType::Kamino => write!(f, "Kamino"),
            SwapType::KaminoLending => write!(f, "KaminoLending"),
            SwapType::SanctumRouter => write!(f, "SanctumRouter"),
            SwapType::RaydiumCpmm => write!(f, "RaydiumCpmm"),
            SwapType::RaydiumPoolV4Stable => write!(f, "RaydiumPoolV4Stable"),
            SwapType::OrcaSwapV2Stable => write!(f, "OrcaSwapV2Stable"),
            SwapType::OrcaSwapV2Offset => write!(f, "OrcaSwapV2Offset"),
            SwapType::TokenSwapStable => write!(f, "TokenSwapStable"),
            SwapType::TokenSwapOffset => write!(f, "TokenSwapOffset"),
            SwapType::BonkSwapStable => write!(f, "BonkSwapStable"),
            SwapType::AldrinAmmV2Stable => write!(f, "AldrinAmmV2Stable"),
            SwapType::MeteoraPoolsStable => write!(f, "MeteoraPoolsStable"),
        }
    }
}

impl TryFrom<u8> for SwapType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SwapType::None),
            1 => Ok(SwapType::RaydiumPoolV4),
            2 => Ok(SwapType::OrcaSwapV2),
            3 => Ok(SwapType::TokenSwap),
            4 => Ok(SwapType::CropperFinance),
            5 => Ok(SwapType::DooarSwap),
            6 => Ok(SwapType::DexlabSwap),
            7 => Ok(SwapType::SarosAmm),
            8 => Ok(SwapType::BonkSwap),
            9 => Ok(SwapType::Fluxbeam),
            10 => Ok(SwapType::AldrinAmm),
            11 => Ok(SwapType::AldrinAmmV2),
            12 => Ok(SwapType::PenguinFinance),
            13 => Ok(SwapType::StepFinance),
            14 => Ok(SwapType::RaydiumCpmm),
            15 => Ok(SwapType::MeteoraPools),
            16 => Ok(SwapType::LifinitySwapV2),
            17 => Ok(SwapType::RaydiumConcentrated),
            18 => Ok(SwapType::OrcaWhirlpool),
            19 => Ok(SwapType::CropperWhirlpool),
            20 => Ok(SwapType::InvariantSwap),
            21 => Ok(SwapType::CremaFinance),
            22 => Ok(SwapType::MeteoraDlmm),
            23 => Ok(SwapType::RaydiumPoolV4Stable),
            24 => Ok(SwapType::OrcaSwapV2Stable),
            25 => Ok(SwapType::OrcaSwapV2Offset),
            26 => Ok(SwapType::TokenSwapStable),
            27 => Ok(SwapType::TokenSwapOffset),
            28 => Ok(SwapType::BonkSwapStable),
            29 => Ok(SwapType::AldrinAmmV2Stable),
            30 => Ok(SwapType::MeteoraPoolsStable),
            31 => Ok(SwapType::SaberStableSwap),
            32 => Ok(SwapType::MercurialStableSwap),
            33 => Ok(SwapType::PumpFun),
            34 => Ok(SwapType::HeliumTreasury),
            35 => Ok(SwapType::OpenbookV2),
            36 => Ok(SwapType::MarginFiV2),
            37 => Ok(SwapType::Kamino),
            38 => Ok(SwapType::KaminoLending),
            39 => Ok(SwapType::SanctumRouter),
            40 => Ok(SwapType::GooseFxV2),
            _ => Err("Invalid value for SwapType"),
        }
    }
}

impl SwapType {
    pub fn get_general_cu(&self) -> u32 {
        match self {
            SwapType::RaydiumPoolV4 => 65000,
            SwapType::OrcaSwapV2 => 75000,
            SwapType::TokenSwap => 75000,
            SwapType::CropperFinance => 55000,
            SwapType::DooarSwap => 70000,
            SwapType::DexlabSwap => 90000,
            SwapType::SarosAmm => 70000,
            SwapType::BonkSwap => 100000,
            SwapType::Fluxbeam => 85000,
            SwapType::AldrinAmm => 75000,
            SwapType::AldrinAmmV2 => 75000,
            SwapType::PenguinFinance => 75000,
            SwapType::StepFinance => 120000,
            SwapType::RaydiumCpmm => 60000,
            SwapType::MeteoraPools => 150000,
            _ => 0,
        }
    }
}

#[derive(Debug)]
pub struct SubscribeAccount {
    account_data: Vec<u8>,
    pair_index: Option<usize>
}

#[derive(Debug)]
pub struct Price {
    pub price_a_to_b: u128,
    pub price_b_to_a: u128,
    pub vault_a_amount: Option<u64>,
    pub vault_b_amount: Option<u64>,

    // below for clmm and dlmm
    pub active_tick_id: Option<i32>,
    pub pool_token_mint_a: Option<Pubkey>,
    pub liquidity: Option<u128>,
    pub sqrt_price_x64: Option<u128>,

    // meteora dlmm
    pub config_denominator: Option<u64>,
    pub std_spread: Option<u64>,
    pub std_spread_buffer: Option<u64>,
    pub spread_coef: Option<u64>,
    pub base_decimal_value: Option<u64>,
    pub price_buffer_coin: Option<i64>,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct RouteState {
    pub acc_price: u128,
    pub pnl: u64,
}

#[derive(Clone, Debug)]
pub struct Route {
    pub route: Vec<u32>,
    pub route_start_mint: Pubkey,
    pub ix: Instruction,
    pub cu: u32,
    pub lookuptables: Vec<AddressLookupTableAccount>,
}

impl Route {

    pub fn remove_trailing_duplicates(vec: Vec<AccountMeta>) -> Vec<AccountMeta> {
        if vec.is_empty() {
            return vec;
        }
    
        let last_unique_pos = vec.iter()
            .rposition(|x| *x != vec[vec.len() - 1])
            .map(|pos| pos + 1) // to keep the last unique item itself
            .unwrap_or(0);
    
        vec.into_iter().take(last_unique_pos).collect()
    }

    pub fn build_route(
        route: &Vec<u32>,
        pairs: &Vec<Pair>,
        lookuptables: &Vec<AddressLookupTableAccount>,
    ) -> Self {
        let wsol_mint_pubkey =
            Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
        let usdc_mint_pubkey =
            Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
        let wsol_ata_pubkey =
            Pubkey::from_str("D398BcsjQ8tyuxGvfCHecgAanKWs8QWj2gDPYfup86vz").unwrap();
        let usdc_ata_pubkey =
            Pubkey::from_str("5n99AA4e888tVdtjvvS6J7ab4LPTLFqJXUL136p9hXWh").unwrap();
        let ata_program_pubkey =
            Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap();
        let pda_signer_pubkey =
            Pubkey::from_str("G5vAeh1AQBYi4c4bCqhioV3AWjh7Gw2oAaUCDcdcWy3G").unwrap();

        let mut cu: u32 = 30000; // default anchor cu

        let tokens = vec![
            pairs[route[0] as usize].mint_a,
            pairs[route[0] as usize].mint_b,
            pairs[route[1] as usize].mint_a,
            pairs[route[1] as usize].mint_b,
        ];
        let route_start_mint = Route::find_first_mint(&tokens).unwrap();

        let mut route_pairs: Vec<&Pair> = Vec::new();

        for pair_idx in route {
            let pair = &pairs[*pair_idx as usize];
            route_pairs.push(pair);
        }

        let mut account_metas: Vec<AccountMeta> = Vec::new();

        account_metas.extend_from_slice(&[
            AccountMeta {
                pubkey: wsol_mint_pubkey,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: usdc_mint_pubkey,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: wsol_ata_pubkey,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: usdc_ata_pubkey,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: solana_program::system_program::ID,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: spl_token::ID,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: ata_program_pubkey,
                is_signer: false,
                is_writable: false,
            },
        ]);

        for pair in route_pairs.iter() {
            cu += pair.swap_type.get_general_cu();

            let mint_a_acc_meta = AccountMeta {
                pubkey: pair.mint_a,
                is_signer: false,
                is_writable: false,
            };
            let mint_b_acc_meta = AccountMeta {
                pubkey: pair.mint_b,
                is_signer: false,
                is_writable: false,
            };
            
            if account_metas.iter().find(|meta| meta.pubkey == pair.mint_a).is_none() {
                account_metas.push(mint_a_acc_meta);
            }
            if account_metas.iter().find(|meta| meta.pubkey == pair.mint_b).is_none() {
                account_metas.push(mint_b_acc_meta);
            }

            let accounts_vec_a_to_b = Route::remove_trailing_duplicates(pair.accounts_vec_a_to_b.to_vec());
            let accounts_vec_b_to_a = Route::remove_trailing_duplicates(pair.accounts_vec_b_to_a.to_vec());

            for meta in accounts_vec_a_to_b.iter() {
                if account_metas.iter().find(|m| m.pubkey == meta.pubkey).is_none() {
                    account_metas.push(meta.clone());
                }
            }

            for meta in accounts_vec_b_to_a.iter() {
                if account_metas.iter().find(|m| m.pubkey == meta.pubkey).is_none() {
                    account_metas.push(meta.clone());
                }
            }
        }

        let pda_signer_index = account_metas.iter().position(|meta| meta.pubkey == pda_signer_pubkey).unwrap();
        account_metas[pda_signer_index].is_signer = false;

        let lookuptables = Route::get_lookups(lookuptables, &account_metas);

        let route_data: Vec<TradeSwapElement> = route_pairs
            .iter()
            .map(|p| {
                let accounts_vec_a_to_b = Route::remove_trailing_duplicates(p.accounts_vec_a_to_b.to_vec());
                let accounts_vec_b_to_a = Route::remove_trailing_duplicates(p.accounts_vec_b_to_a.to_vec());
                let mut accounts_index_vec_a_to_b: Vec<u8> = accounts_vec_a_to_b
                    .iter()
                    .map(|m| {
                        account_metas
                            .iter()
                            .position(|t| m.pubkey == t.pubkey)
                            .unwrap() as u8
                    })
                    .collect();
                accounts_index_vec_a_to_b.resize(25, 0u8);

                let mut accounts_index_vec_b_to_a: Vec<u8> = accounts_vec_b_to_a
                    .iter()
                    .map(|m| {
                        account_metas
                            .iter()
                            .position(|t| m.pubkey == t.pubkey)
                            .unwrap() as u8
                    })
                    .collect();
                accounts_index_vec_b_to_a.resize(25, 0u8);

                TradeSwapElement {
                    swap_type: p.swap_type,
                    data_index_a_to_b: p.data_index_a_to_b,
                    data_index_b_to_a: p.data_index_b_to_a,
                    mint_a_index: account_metas
                        .iter()
                        .position(|m| m.pubkey == p.mint_a)
                        .unwrap() as u8,
                    mint_b_index: account_metas
                        .iter()
                        .position(|m| m.pubkey == p.mint_b)
                        .unwrap() as u8,
                    user_vault_a_index: account_metas
                        .iter()
                        .position(|m| m.pubkey == p.user_vault_a)
                        .unwrap() as u8,
                    user_vault_b_index: account_metas
                        .iter()
                        .position(|m| m.pubkey == p.user_vault_b)
                        .unwrap() as u8,
                    fee_percent: p.fee as u16,
                    accounts_index_vec_a_to_b: accounts_index_vec_a_to_b
                        .as_slice()
                        .try_into()
                        .unwrap(),
                    accounts_index_vec_b_to_a: accounts_index_vec_b_to_a
                        .as_slice()
                        .try_into()
                        .unwrap(),
                }
            })
            .collect();
        let start_mint_index = account_metas
            .iter()
            .position(|m| m.pubkey == route_start_mint)
            .unwrap();
        let sol_mint_index = account_metas
            .iter()
            .position(|m| m.pubkey == wsol_mint_pubkey)
            .unwrap();
        let usdc_mint_index = account_metas
            .iter()
            .position(|m| m.pubkey == usdc_mint_pubkey)
            .unwrap();
        let sol_ata_index = account_metas
            .iter()
            .position(|m| m.pubkey == wsol_ata_pubkey)
            .unwrap();
        let usdc_ata_index = account_metas
            .iter()
            .position(|m| m.pubkey == usdc_ata_pubkey)
            .unwrap();
        let system_program_index = account_metas
            .iter()
            .position(|m| m.pubkey == solana_program::system_program::ID)
            .unwrap();
        let token_program_index = account_metas
            .iter()
            .position(|m| m.pubkey == spl_token::ID)
            .unwrap();

        let mut main_acc_meta = vec![
            AccountMeta {
                pubkey: Pubkey::from_str("H4EpRFY8KRabsm1Ktso3Zf6rognHYANnMh3iLhnSnyaW").unwrap(), // signer
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: Pubkey::from_str("Byg11WnJ9Q53hzDFmeBH6sFgnpUFVJJsCvhZBTWkAQEL").unwrap(), // trade
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: Pubkey::from_str("G5vAeh1AQBYi4c4bCqhioV3AWjh7Gw2oAaUCDcdcWy3G").unwrap(), // trade_authority
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: route_start_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: if route_start_mint == wsol_mint_pubkey {
                    wsol_ata_pubkey
                } else {
                    usdc_ata_pubkey
                },
                is_signer: false,
                is_writable: true,
            },
        ];

        main_acc_meta.extend(account_metas.into_iter());

        let ix = Route::get_trade_swap_ix(
            &main_acc_meta,
            &TradeSwapParams {
                start_mint_index: start_mint_index as u8,
                sol_mint_index: sol_mint_index as u8,
                usdc_mint_index: usdc_mint_index as u8,
                sol_vault_index: sol_ata_index as u8,
                usdc_vault_index: usdc_ata_index as u8,
                system_program_index: system_program_index as u8,
                token_program_index: token_program_index as u8,
                initial_amount: 0,
                routes: route_data.clone(),
            },
        );

        // println!("ix: {:#?}", ix);

        Route {
            route: route.clone(),
            route_start_mint,
            ix,
            cu,
            lookuptables,
        }
    }

    pub fn update_amount(&mut self, amount: u64) {
        let amount_start_index: usize = 8 + 1 * 7;
        self.ix.data[amount_start_index..amount_start_index + 8]
            .copy_from_slice(&amount.to_le_bytes());
    }

    pub fn get_lookups(
        lookuptables: &Vec<AddressLookupTableAccount>,
        account_metas: &Vec<AccountMeta>,
    ) -> Vec<AddressLookupTableAccount> {
        let mut lookups: Vec<AddressLookupTableAccount> = vec![lookuptables[0].clone()]; // default lookup

        for account_meta in account_metas {
            if let Some(lookup) = lookuptables
                .iter()
                .find(|l| l.addresses.contains(&account_meta.pubkey))
            {
                if !lookups.contains(lookup) {
                    lookups.push(lookup.clone());
                }
            }
        }
        lookups
    }

    pub fn find_first_mint(tokens: &[Pubkey]) -> Option<Pubkey> {
        let usdc_mint = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
        let wsol_mint = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();

        if (tokens[0].eq(&usdc_mint) && !tokens[1].eq(&wsol_mint))
            || (tokens[1].eq(&usdc_mint) && !tokens[0].eq(&wsol_mint))
        {
            Some(usdc_mint)
        } else if (tokens[0].eq(&wsol_mint) && !tokens[1].eq(&usdc_mint))
            || (tokens[1].eq(&wsol_mint) && !tokens[0].eq(&usdc_mint))
        {
            Some(wsol_mint)
        } else if tokens[2].eq(&usdc_mint) || tokens[3].eq(&usdc_mint) {
            Some(wsol_mint)
        } else if tokens[2].eq(&wsol_mint) || tokens[3].eq(&wsol_mint) {
            Some(usdc_mint)
        } else {
            None
        }
    }

    pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
        let preimage = format!("{namespace}:{name}");

        let mut sighash = [0u8; 8];
        sighash.copy_from_slice(&crate::hash::hash(preimage.as_bytes()).to_bytes()[..8]);
        sighash
    }

    pub fn get_trade_swap_ix(accounts: &Vec<AccountMeta>, params: &TradeSwapParams) -> Instruction {
        // trace!();
        let program_id = Pubkey::from_str("midcgR1PV98aK9gcY9DmbgeyRdyYw5RVX7nedhRm12a").unwrap();
        let discriminator = Route::sighash("global", "xswap");

        Instruction::new_with_borsh(program_id, &(discriminator, params), accounts.to_vec())
    }

    pub fn get_trade_pay_jito_tip_ix(
        accounts: &Vec<AccountMeta>,
        params: &TradePayJitoTipParams,
    ) -> Instruction {
        let program_id = Pubkey::from_str("midcgR1PV98aK9gcY9DmbgeyRdyYw5RVX7nedhRm12a").unwrap();
        let discriminator = Route::sighash("global", "tradex_pay_jito_tip");

        Instruction::new_with_borsh(
            program_id,
            &(discriminator, params),
            accounts.to_vec(),
        )
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct LookupTable {
    pub lookuptable: String,
    pub keys: Vec<String>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TradeSwapParams {
    pub start_mint_index: u8,
    pub usdc_mint_index: u8,
    pub sol_mint_index: u8,
    pub usdc_vault_index: u8,
    pub sol_vault_index: u8,
    pub system_program_index: u8,
    pub token_program_index: u8,
    pub initial_amount: u64,
    pub routes: Vec<TradeSwapElement>,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct TradeSwapElement {
    pub swap_type: SwapType,
    pub data_index_a_to_b: u8,
    pub data_index_b_to_a: u8,
    pub mint_a_index: u8,
    pub mint_b_index: u8,
    pub user_vault_a_index: u8,
    pub user_vault_b_index: u8,
    pub fee_percent: u16,
    pub accounts_index_vec_a_to_b: [u8; 25],
    pub accounts_index_vec_b_to_a: [u8; 25],
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TradePayJitoTipParams {
    pub usdc_sol_rate: u16,
    pub percent: u64,
}

#[derive(Debug)]
pub enum SharedMemClientErr {
    MathOverflowError(String)
}

impl fmt::Display for SharedMemClientErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SharedMemClientErr::MathOverflowError(ref msg) => write!(f, "Mathoverflow Error: {}", msg),
        }
    }
}

impl std::error::Error for SharedMemClientErr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            SharedMemClientErr::MathOverflowError(_) => None,
        }
    }
}