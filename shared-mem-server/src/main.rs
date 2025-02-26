use std::time;
use std::{path::Path, thread::sleep};
use std::collections::HashMap;
use shared_mem::ShmManager;
use shared_memory::{Shmem, ShmemConf, ShmemError};
use solana_sdk::account::ReadableAccount;
use solana_sdk::{account::AccountSharedData, pubkey::Pubkey};
use raw_sync::{events::*, Timeout};
use rand::Rng;


pub fn main() {
    let mut shm_manager = ShmManager::create();
    
    println!("server started!");
    let mut pubkeys = Vec::new();
    for i in 0..10000 {
        let pubkey = Pubkey::new_unique();
        let account_shared_data = AccountSharedData::new(i as u64, i * 10, &pubkey);
        shm_manager.insert_account(&pubkey, &account_shared_data);
        pubkeys.push(pubkey.clone());
    }
    
    println!("added 10000 accounts");
    println!("run client in 10s");
    let delay = time::Duration::from_millis(10000);
    sleep(delay);
    loop {
        let random_idx = rand::thread_rng().gen_range(0..10000);
        let random_key = pubkeys.get(random_idx).unwrap();
        // let account_data = shm_manager.get_account_data(random_key);

        shm_manager.set_account_update_event(random_key);
        println!("notified {:#?}", random_key);
        let delay = time::Duration::from_millis(2000);
        sleep(delay);
    }
    
}