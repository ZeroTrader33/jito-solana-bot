use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::{path::Path, sync::atomic::AtomicBool};
use std::collections::HashMap;
use dashmap::DashMap;
use shared_memory::{Shmem, ShmemConf, ShmemError};
use solana_sdk::account::ReadableAccount;
use solana_sdk::program_memory::{sol_memcpy, sol_memset};
use solana_sdk::{account::AccountSharedData, pubkey::Pubkey};
use raw_sync::{events::*, Timeout};

pub struct AccountMetaInfo {
    index: u64,
    len: u32
}
pub struct ShmManager {
    shm_accounts: HashMap<Pubkey, AccountMetaInfo>,
    shm_account_update: Shmem,
    shm_pubkey: Shmem,
    shm_pubkeys: Shmem,
    shm_account_data: Shmem,
}
impl ShmManager {
    pub const SHM_PUBKEYS_LINK: &str = "/root/SHM_PUBKEYS_LINK";
    pub const SHM_PUBKEYS_SIZE: usize = 4 + 44 * 20_000;
    pub const SHM_ACCOUNT_UPDATE_LINK: &str = "/root/SHM_ACCOUNT_UPDATE_LINK";
    pub const SHM_ACCOUNT_UPDATE_SIZE: usize = 1024;
    pub const SHM_PUBKEY_LINK: &str = "/root/SHM_PUBKEY_LINK";
    pub const SHM_PUBKEY_SIZE: usize = 32;
    pub const SHM_ACCOUNT_DATA_LINK: &str = "/root/SHM_ACCOUNT_DATA_LINK";
    pub const SHM_ACCOUNT_DATA_SIZE: usize = 15 * 1024 * 1024 * 1024;

    pub fn open_shm(flink: &str) -> Shmem {
        ShmemConf::new().flink(flink).open().expect("can't open accounts shared memory")
    }
    pub fn create_shm(flink: &str, space_size: usize) -> Shmem {
        ShmemConf::new().size(space_size).flink(flink).force_create_flink().create().expect("can't create accounts shared memory")
    }
    pub fn create() -> Self {
        let shm_pubkeys = Self::create_shm(Self::SHM_PUBKEYS_LINK, Self::SHM_PUBKEYS_SIZE);
        let shm_account_update = Self::create_shm(Self::SHM_ACCOUNT_UPDATE_LINK, Self::SHM_ACCOUNT_UPDATE_SIZE);
        let data: &mut [u8; Self::SHM_PUBKEYS_SIZE] = unsafe { &mut *shm_pubkeys.as_ptr().cast() };
        data[0] = 0;data[1] = 0;data[2] = 0;data[3] = 0;
        // let (_evt, _used_bytes) = unsafe { Event::new(shm_account_update.as_ptr(), true).expect("getting event failed!") };
        Self {
            shm_account_data: Self::create_shm(Self::SHM_ACCOUNT_DATA_LINK, Self::SHM_ACCOUNT_DATA_SIZE),
            shm_accounts: HashMap::new(),
            shm_account_update,
            shm_pubkey: Self::create_shm(Self::SHM_PUBKEY_LINK, Self::SHM_PUBKEY_SIZE),
            shm_pubkeys,
        }
    }
    pub fn close(shm_mang: Self) {
        drop(shm_mang.shm_accounts);
        drop(shm_mang.shm_account_update);
        drop(shm_mang.shm_pubkey);
        drop(shm_mang.shm_pubkeys);
    }
    pub fn open() -> Self {
        let shm_accounts = HashMap::new();
        let shm_pubkeys = Self::open_shm(Self::SHM_PUBKEYS_LINK);

        let mut shm_mng = Self {
            shm_accounts,
            shm_account_data: Self::open_shm(Self::SHM_ACCOUNT_DATA_LINK),
            shm_account_update: Self::open_shm(Self::SHM_ACCOUNT_UPDATE_LINK),
            shm_pubkey: Self::open_shm(Self::SHM_PUBKEY_LINK),
            shm_pubkeys,
        };
        shm_mng.open_all_accounts();
        shm_mng
    }
    pub fn open_all_accounts(&mut self) {
        let data: &[u8; Self::SHM_PUBKEYS_SIZE] = unsafe { &*self.shm_pubkeys.as_ptr().cast() };
        let cnt = u32::from_le_bytes(data[0..4].try_into().unwrap()) as usize;
        println!("opening {} accounts", cnt);
        for idx in 0..cnt {
            let start_idx: usize = 4 + idx * 44;
            let pubkey_bytes: [u8; 32] = data[start_idx..(start_idx + 32)].try_into().unwrap();
            let index = u64::from_le_bytes(data[start_idx + 32..start_idx + 40].try_into().unwrap());
            let len = u32::from_le_bytes(data[start_idx + 40..(start_idx + 44)].try_into().unwrap());
            let pubkey = Pubkey::new_from_array(pubkey_bytes);
            self.shm_accounts.insert(pubkey, AccountMetaInfo{ index, len});
        }
        println!("opened {} accounts", cnt);
    }
    pub fn has_key(&self, pubkey: &Pubkey) -> bool {
        self.shm_accounts.contains_key(pubkey)
    }
    pub fn get_account_data(&self, pubkey: &Pubkey) -> &[u8] {
        let info = self.shm_accounts.get(pubkey).unwrap();
        let data: &[u8; Self::SHM_ACCOUNT_DATA_SIZE] = unsafe { &*self.shm_account_data.as_ptr().cast() };
        &data[info.index as usize..info.index as usize + info.len as usize]
    }
    // server side insert
    pub fn insert_account(&mut self, pubkey: &Pubkey, account_shared_data: &AccountSharedData) {
        let data: &mut [u8; Self::SHM_ACCOUNT_DATA_SIZE] = unsafe { &mut *self.shm_account_data.as_ptr().cast() };
        
        let data_len = account_shared_data.data().len();
        
        if self.shm_accounts.contains_key(pubkey)  {
            let info = self.shm_accounts.get(pubkey).unwrap();
            if info.len as usize == data_len {
                sol_memcpy(&mut data[info.index as usize..info.index as usize + data_len], account_shared_data.data(), data_len);
            }
        }
        else {
            let stored_len = u64::from_le_bytes(data[0..8].try_into().unwrap());
            sol_memcpy(&mut data[stored_len as usize..stored_len as usize + data_len], account_shared_data.data(), data_len);
            sol_memcpy(&mut data[0..8], &(stored_len + data_len as u64).to_le_bytes(), 8);
            self.shm_accounts.insert(pubkey.clone(), AccountMetaInfo{ index: stored_len, len: data_len as u32});

            self.add_pubkey(pubkey, stored_len, data_len as u32);
        }
    }
    pub fn add_pubkey(&self, pubkey: &Pubkey, index: u64, len: u32) {
        let data: &mut [u8; Self::SHM_PUBKEYS_SIZE] = unsafe { &mut *self.shm_pubkeys.as_ptr().cast() };
        let cnt = u32::from_le_bytes(data[0..4].try_into().unwrap()) + 1;
        sol_memcpy(data, &cnt.to_le_bytes(), 4);
        let start_idx = (4 + (cnt - 1) * 44) as usize;
        sol_memcpy(&mut data[start_idx..start_idx + 32], &pubkey.to_bytes(), 32);
        sol_memcpy(&mut data[start_idx + 32..start_idx + 40], &index.to_le_bytes(), 8);
        sol_memcpy(&mut data[start_idx + 40..start_idx + 44], &len.to_le_bytes(), 4);
    }


    pub fn set_account_update_event(&self, pubkey: &Pubkey) {
        // let (evt, _used_bytes) = unsafe { Event::from_existing(self.shm_account_update.as_ptr()).expect("getting event failed!") };
        let evt_pubkey: &mut [u8; 32] = unsafe { &mut *self.shm_pubkey.as_ptr().cast() };
        *evt_pubkey = pubkey.to_bytes();
        // evt.set(EventState::Signaled).expect("event setting for Signaled failed!");
        
    }
    pub fn recv_account_update_event(&self) -> Pubkey {
        // let (evt, _used_bytes) = unsafe { Event::from_existing(self.shm_account_update.as_ptr()).expect("getting event failed!") };
        // evt.wait(Timeout::Infinite).expect("event waiting failed!");
        let pubkey: &[u8; 32] = unsafe { &*self.shm_pubkey.as_ptr().cast() };
        let new_pubkey = Pubkey::new_from_array(*pubkey);
        // evt.set(EventState::Clear).expect("event setting for Clear failed!");
        new_pubkey
    }
}