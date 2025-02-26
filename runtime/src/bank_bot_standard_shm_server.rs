use std::{fs::File, io::Read, str::FromStr, sync::{atomic::{AtomicBool, AtomicPtr, Ordering}, Arc, RwLock}};

use shared_mem::ShmManager;
use solana_sdk::{account::AccountSharedData, pubkey::Pubkey};

use crate::{bank::Bank, bank_forks::BankForks, commitment::BlockCommitmentCache};

// pub const POOL_ADDRESSES_FILE_PATH: &str = "/home/sol/project/mcard-contract/json-data/standard/standard-pool-addresses.json";
pub const POOL_ADDRESSES_FILE_PATH: &str = "/root/project/pairs_2024_12_05/non-stable-pool-addresses.json";

pub struct BankBotStandardShmServer {
  pub is_running: AtomicBool,
  pub pubkeys_to_subscribe: AtomicPtr<Vec<Pubkey>>,
  pub bank_forks: AtomicPtr<Option<Arc<RwLock<BankForks>>>>,
  pub block_commitment_cache: AtomicPtr<Option<Arc<RwLock<BlockCommitmentCache>>>>,
  pub shm_manager: AtomicPtr<ShmManager>
}

impl Default for BankBotStandardShmServer {
  fn default() -> Self {
    BankBotStandardShmServer {
      is_running: AtomicBool::new(false),
      pubkeys_to_subscribe: AtomicPtr::new(Box::into_raw(Box::new(Vec::new()))),
      bank_forks: AtomicPtr::new(Box::into_raw(Box::new(None))),
      block_commitment_cache: AtomicPtr::new(Box::into_raw(Box::new(None))),
      shm_manager: AtomicPtr::new(Box::into_raw(Box::new(ShmManager::create())))
    }
  }
}

pub struct ArcBankBotStandardShmServer {
  pub bank_bot: Arc<BankBotStandardShmServer>,
}

impl ArcBankBotStandardShmServer {
  pub fn default() -> Self {
    ArcBankBotStandardShmServer {
      bank_bot: Arc::new(BankBotStandardShmServer::default()),
    }
  }

  pub fn set_run_mode(&self, is_running: bool) {
    if !is_running {
      self.bank_bot.is_running.store(is_running, Ordering::SeqCst);
      println!("standard shm bot stopped!");
    } else {
      println!("standard shm bot started!");
    }
  }

  pub fn account_update(&self, pubkey: &Pubkey, account_data: &AccountSharedData) {        
    let is_running = self.bank_bot.is_running.load(Ordering::SeqCst);
    let pubkeys_to_subscribe = unsafe{&* self.bank_bot.pubkeys_to_subscribe.load(Ordering::SeqCst)};
    let shm_manager = unsafe{&mut * self.bank_bot.shm_manager.load(Ordering::SeqCst)};

    if is_running && pubkeys_to_subscribe.contains(pubkey) {
      // println!("{:#?} sending", pubkey);
      shm_manager.insert_account(pubkey, account_data);
      // println!("{:#?} inserted", pubkey);
      shm_manager.set_account_update_event(pubkey);
      // println!("{:#?} sent", pubkey);
    }
  }

  pub fn run(&self) {
    println!("reading files...");
    self.read_from_file();
    println!("creating shm manager...");
    self.create_shm_and_set_initialy();
    self.bank_bot.is_running.store(true, Ordering::SeqCst);
    println!("bot server started!");
  }

  pub fn create_shm_and_set_initialy(&self) {
    let bank = self.bank();
    let pool_addresses = unsafe {&* self.bank_bot.pubkeys_to_subscribe.load(Ordering::SeqCst) };
    let mut shm_manager = ShmManager::create();
    
    for pubkey in pool_addresses {
      let account_data = match bank.get_account_with_fixed_root(pubkey) {
        Some(data) => data,
        None => AccountSharedData::default()
      };
      shm_manager.insert_account(pubkey, &account_data);
      // shm_manager.set_account_update_event(pubkey);
    }
    self.bank_bot.shm_manager.store(Box::into_raw(Box::new(shm_manager)), Ordering::SeqCst);
  }

  pub fn read_from_file(&self) {
    let mut pool_addresses_file = File::open(POOL_ADDRESSES_FILE_PATH).unwrap();
    let mut content = String::new();
    pool_addresses_file.read_to_string(&mut content).unwrap();
    let pool_addresses_str: Vec<String> = serde_json::from_str(&content).unwrap();
    let mut pool_addresses: Vec<Pubkey> = pool_addresses_str.iter().map(|p| Pubkey::from_str(p).unwrap()).collect();

    let wsol_ata_pubkey = Pubkey::from_str("D398BcsjQ8tyuxGvfCHecgAanKWs8QWj2gDPYfup86vz").unwrap();
    let usdc_ata_pubkey = Pubkey::from_str("5n99AA4e888tVdtjvvS6J7ab4LPTLFqJXUL136p9hXWh").unwrap();
    pool_addresses.push(wsol_ata_pubkey);
    pool_addresses.push(usdc_ata_pubkey);

    self.bank_bot.pubkeys_to_subscribe.store(Box::into_raw(Box::new(pool_addresses)), Ordering::SeqCst);
  }

  pub fn working_bank(&self) -> Arc<Bank> {
    let bank_forks = unsafe { &*self.bank_bot.bank_forks.load(Ordering::SeqCst) };
    bank_forks.as_ref().unwrap().read().unwrap().working_bank()
  }

  pub fn bank(&self) -> Arc<Bank> {
    let bank_forks = unsafe { &*self.bank_bot.bank_forks.load(Ordering::SeqCst) };
    let block_commitment_cache =
        unsafe { &*self.bank_bot.block_commitment_cache.load(Ordering::SeqCst) };

    // println!("getting bank_forks!");
    let bank_forks = bank_forks.as_ref().unwrap().read().unwrap();
    let block_commitment_cache = block_commitment_cache.as_ref().unwrap();
    let slot = block_commitment_cache.read().unwrap().slot();
    // println!("getting bank_forks is done!");
    bank_forks.get(slot).unwrap_or_else(|| {
        // We log a warning instead of returning an error, because all known error cases
        // are due to known bugs that should be fixed instead.
        //
        // The slot may not be found as a result of a known bug in snapshot creation, where
        // the bank at the given slot was not included in the snapshot.
        // Also, it may occur after an old bank has been purged from BankForks and a new
        // BlockCommitmentCache has not yet arrived. To make this case impossible,
        // BlockCommitmentCache should hold an `Arc<Bank>` everywhere it currently holds
        // a slot.
        //
        // For more information, see https://github.com/solana-labs/solana/issues/11078
        println!("Bank not found at slot: {:?}", slot);
        bank_forks.root_bank()
    })
  }

  pub fn set_bank_forks(&self, bank_forks: Arc<RwLock<BankForks>>) {
    self.bank_bot
        .bank_forks
        .store(Box::into_raw(Box::new(Some(bank_forks))), Ordering::SeqCst);
  }
  pub fn set_block_commitment_cache(
      &self,
      block_commitment_cache: Arc<RwLock<BlockCommitmentCache>>,
  ) {
      // println!("dyn-bot: set_bank_forks called");
      self.bank_bot.block_commitment_cache.store(
          Box::into_raw(Box::new(Some(block_commitment_cache))),
          Ordering::SeqCst,
      );
  }
}