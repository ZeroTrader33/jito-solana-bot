use std::sync::atomic::Ordering;
use std::{path::Path, sync::Arc};
use std::str::FromStr;
use jup_bot::JupBot;
use shared_mem::ShmManager;
use shared_memory::{Shmem, ShmemConf, ShmemError};
use solana_sdk::{account::AccountSharedData, pubkey::Pubkey};
use raw_sync::{events::*, Timeout};
use rand::Rng;

pub mod big_num;
pub mod bank_bot;
pub mod orca_whirlpool;
pub mod invariant;
// pub mod bank_bot_standard;
pub mod hash;
pub mod safe_math;
pub mod utils_math;
pub mod u64x64_math;
pub mod u128x128_math;
pub mod perps_bot;
pub mod jup_bot;

// #[tokio::main]
// pub async fn main() {
// }

// use bank_bot_standard::{
//     ArcBankBot,
//     BankBot
// };
use bank_bot::{
    ArcBankBot,
    BankBot
};

#[tokio::main]
pub async fn main() {
    // run_bot_v1().await;
    // run_bot_v2().await;
    // run_perps_bot();
    run_jup_bot();
}
pub fn run_jup_bot() {
    println!("jup bot started!");
    std::panic::set_hook(Box::new(|info| {
        println!("Caught a panic: {:?}", info);
      }));

    let jup_bot = Arc::new(jup_bot::JupBot::default());
    jup_bot.setup();

    let jup_bot_clone = Arc::clone(&jup_bot);
    let handle_blockhash = std::thread::spawn(move || {
        jup_bot_clone.update_latest_blockhash();
    });
    let handle_update = std::thread::spawn(move || {
        listen_account_update_jup(&jup_bot);
    });
    handle_update.join().expect("account update thread panicked!");
}
pub fn run_perps_bot() {
    println!("perps bot started!");
}
pub async fn run_bot_v2() {
    let bank_bot = Arc::new(ArcBankBot { bank_bot: Arc::new(BankBot::default())});
    bank_bot.setup_v2().await;
    
    let bank_bot_clone1 = Arc::clone(&bank_bot);
    let handle1 = std::thread::spawn(move || {
        bank_bot_clone1.update_latest_blockhash();
    });

    let bank_bot_clone2 = Arc::clone(&bank_bot);
    let handle2 = std::thread::spawn(move || {
        listen_account_update_v2(&bank_bot_clone2);
    });

    // handle1.join().expect("blockhash thread panicked!");
    handle2.join().expect("account update thread panicked!");

    // bank_bot.run_v2().await;
}
pub async fn run_bot_v1() {
    let bank_bot = Arc::new(ArcBankBot { bank_bot: Arc::new(BankBot::default())});
    bank_bot.run().await;
    
    let bank_bot_clone1 = Arc::clone(&bank_bot);
    let handle1 = std::thread::spawn(move || {
        bank_bot_clone1.update_latest_blockhash();
    });

    let bank_bot_clone2 = Arc::clone(&bank_bot);
    let handle2 = std::thread::spawn(move || {
        listen_account_update(&bank_bot_clone2);
    });

    // handle1.join().expect("blockhash thread panicked!");
    handle2.join().expect("account update thread panicked!");
}
pub fn listen_account_update(bot: &Arc<ArcBankBot>) {
    println!("start to listen.");
    let shm_manager = ShmManager::open();
    let mut prev_pubkey = Pubkey::default();
    let wsol_ata_pubkey = Pubkey::from_str("Gm6TTgZXjATYcpBxkmJnCZbzeeoJ7TMJKwDLMaLFSpE1").unwrap();
    let usdc_ata_pubkey = Pubkey::from_str("BChF15Y7PAwEhWNCZxfa6AujRXZiHVTecsbH5CuV1jzD").unwrap();
    loop {
        let pubkey = shm_manager.recv_account_update_event();
        if prev_pubkey.ne(&pubkey) {
            prev_pubkey = pubkey;
            
    
            let is_running = bot.bank_bot.is_running.load(Ordering::SeqCst);
            let is_price_update_initially = bot.bank_bot.is_price_updated_initially.load(Ordering::SeqCst);

            if is_running && is_price_update_initially {
                let accounts_to_subscribe_vec = unsafe{&* bot.bank_bot.accounts_to_subscribe_vec.load(Ordering::SeqCst)};
                if accounts_to_subscribe_vec.contains(&prev_pubkey) && shm_manager.has_key(&prev_pubkey) {
                    // println!("pubkey: {:#?}", prev_pubkey);
                    let account_data = shm_manager.get_account_data(&prev_pubkey);
                    bot.account_update(&prev_pubkey, account_data);
                }
                else if wsol_ata_pubkey.eq(&prev_pubkey) || usdc_ata_pubkey.eq(&prev_pubkey) {
                    let account_data = shm_manager.get_account_data(&prev_pubkey);
                    let accounts_to_subscribe = unsafe{&mut * bot.bank_bot.accounts_to_subscribe.load(Ordering::SeqCst)};
                    let account_subscribe = accounts_to_subscribe.get_mut(&prev_pubkey).unwrap();
                    account_subscribe.account_data = account_data.to_vec();
                }
                
            }
            
        }
        // println!("updated pubkey {:#?},  account len {}",pubkey,  shm_manager.get_account_data(&pubkey).len());
        // let account_shared_data = accounts.get(&pubkey).expect("account doesn't exist");
        // println!("received update for {:#?}, space: {:#?}", pubkey, account_shared_data.capacity());
    }
}
pub fn listen_account_update_v2(bot: &Arc<ArcBankBot>) {
    println!("start to listen.");
    let shm_manager = ShmManager::open();
    let mut prev_pubkey = Pubkey::default();
    let wsol_ata_pubkey = Pubkey::from_str("Gm6TTgZXjATYcpBxkmJnCZbzeeoJ7TMJKwDLMaLFSpE1").unwrap();
    let usdc_ata_pubkey = Pubkey::from_str("BChF15Y7PAwEhWNCZxfa6AujRXZiHVTecsbH5CuV1jzD").unwrap();
    loop {
        let pubkey = shm_manager.recv_account_update_event();
        if prev_pubkey.ne(&pubkey) {
            prev_pubkey = pubkey;
            
    
            let is_running = bot.bank_bot.is_running.load(Ordering::SeqCst);
            let is_price_update_initially = bot.bank_bot.is_price_updated_initially.load(Ordering::SeqCst);

            if is_running && is_price_update_initially {
                let accounts_to_subscribe_vec = unsafe{&* bot.bank_bot.accounts_to_subscribe_vec.load(Ordering::SeqCst)};
                if accounts_to_subscribe_vec.contains(&prev_pubkey) && shm_manager.has_key(&prev_pubkey) {
                    // println!("pubkey: {:#?}", prev_pubkey);
                    let account_data = shm_manager.get_account_data(&prev_pubkey);
                    bot.account_update_v2(&prev_pubkey, account_data);
                }
                else if wsol_ata_pubkey.eq(&prev_pubkey) || usdc_ata_pubkey.eq(&prev_pubkey) {
                    let account_data = shm_manager.get_account_data(&prev_pubkey);
                    let accounts_to_subscribe = unsafe{&mut * bot.bank_bot.accounts_to_subscribe.load(Ordering::SeqCst)};
                    let account_subscribe = accounts_to_subscribe.get_mut(&prev_pubkey).unwrap();
                    account_subscribe.account_data = account_data.to_vec();
                }
                
            }
            
        }
    }
}
pub fn listen_account_update_jup(bot: &Arc<JupBot>) {
    println!("start to listen.");
    let shm_manager = ShmManager::open();
    let mut prev_pubkey = Pubkey::default();
    // let bot_clone = bot.clone();
    // std::thread::spawn(move || {
    //     JupBot::run_iterate_edges(&bot_clone);
    // });

    loop {
        let pubkey = shm_manager.recv_account_update_event();
        if prev_pubkey.ne(&pubkey) {
            prev_pubkey = pubkey;
            // println!("pubkey: {:#?}", prev_pubkey);

            JupBot::account_update(bot, &prev_pubkey, false);
        }
    }
}