use std::sync::{
  atomic::{AtomicBool, AtomicPtr, Ordering},
  Arc, RwLock,
};

use crate::{
  bank_bot_standard_shm_server::ArcBankBotStandardShmServer, bank_forks::BankForks, commitment::BlockCommitmentCache
};
use solana_accounts_db::{
  account_storage::meta::StoredAccountMeta,
  accounts_update_notifier_interface::AccountsUpdateNotifierInterface,
};
use solana_sdk::{
  account::AccountSharedData, clock::Slot, pubkey::Pubkey, transaction::SanitizedTransaction,
};

#[derive(Debug)]
pub struct BotManager {
  pub is_running: AtomicBool,
  pub arc_bot: AtomicPtr<ArcBankBotStandardShmServer>,
}
impl BotManager {
  pub fn check_running(instance: &Arc<BotManager>) -> bool {
      instance.is_running.load(Ordering::SeqCst)
  }
  pub fn default() -> Self {
      BotManager {
          is_running: AtomicBool::new(false),
          arc_bot: AtomicPtr::new(Box::into_raw(Box::new(ArcBankBotStandardShmServer::default()))),
      }
  }
  pub fn os_string(origin: &str) -> std::ffi::OsString {
      let mut string = std::ffi::OsString::with_capacity(origin.len());
      string.push(origin);
      string
  }
  
  pub fn run_bot(
      instance: &Arc<BotManager>,
      bank_forks: Arc<RwLock<BankForks>>,
      block_commitment_cache: Arc<RwLock<BlockCommitmentCache>>,
  ) {
      if !BotManager::check_running(instance) {
          let arc_bot = unsafe { &*instance.arc_bot.load(Ordering::SeqCst) };
          arc_bot.set_bank_forks(bank_forks.clone());
          arc_bot.set_block_commitment_cache(block_commitment_cache.clone());

          let arc_bot_clone = ArcBankBotStandardShmServer {bank_bot: Arc::clone(&arc_bot.bank_bot)};

          std::thread::spawn(move || {
            arc_bot_clone.run();
          });

          instance.is_running.store(true, Ordering::SeqCst);
      }
  }
  pub fn stop_bot(instance: &Arc<BotManager>) {
      if BotManager::check_running(instance) {
          let arc_bot = unsafe { &*instance.arc_bot.load(Ordering::SeqCst) };
          arc_bot.set_run_mode(false);
          instance.is_running.store(false, Ordering::SeqCst);
      }
  }
  pub fn notify_account_update_self(&self, pubkey: &Pubkey, account_data: &AccountSharedData) {
      let is_running = self.is_running.load(Ordering::SeqCst);
      if is_running {
          let arc_bot = unsafe { &*self.arc_bot.load(Ordering::SeqCst) };
          arc_bot.account_update(pubkey, account_data);
      }
  }
}
impl AccountsUpdateNotifierInterface for BotManager {
  /// Notified when an account is updated at runtime, due to transaction activities

  fn notify_account_update(
      &self,
      _slot: Slot,
      account: &AccountSharedData,
      _txn: &Option<&SanitizedTransaction>,
      pubkey: &Pubkey,
      _write_version: u64,
  ) {
      self.notify_account_update_self(pubkey, account);
  }

  /// Notified when the AccountsDb is initialized at start when restored
  /// from a snapshot.
  fn notify_account_restore_from_snapshot(&self, _slot: Slot, _account: &StoredAccountMeta) {
      // println!("notify_account_restore_from_snapshot {:#?}", account.pubkey());
  }

  /// Notified when all accounts have been notified when restoring from a snapshot.
  fn notify_end_of_restore_from_snapshot(&self) {
      // println!("notify_end_of_restore_from_snapshot");
  }
  fn snapshot_notifications_enabled(&self) -> bool {
    true
}
}
