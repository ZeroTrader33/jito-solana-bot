use std::{collections::HashMap, str::FromStr, sync::Arc, thread::sleep};

use arrayref::array_ref;
use solana_client::{rpc_config::{RpcBlockConfig, RpcSendTransactionConfig}, tpu_client::{self, TpuClientConfig}};
use solana_rpc_client::rpc_client;
use solana_sdk::{clock::Slot, commitment_config::{CommitmentConfig, CommitmentLevel}, compute_budget::ComputeBudgetInstruction, hash::hash, instruction::{AccountMeta, Instruction}, pubkey::Pubkey, rent, signature::{self, Keypair}, signer::Signer, timing::timestamp, vote::{instruction::CreateVoteAccountConfig, state::{VoteInit, VoteStateUpdate}}};
use solana_transaction_status::{EncodedTransaction, UiMessage};

pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{namespace}:{name}");

    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(&hash(preimage.as_bytes()).to_bytes()[..8]);
    sighash
}
fn main() {
    let mut cnt = 0;
    loop {
        cnt += 1;
        print!("{} ", cnt);
        compact_vote_update();
        let ten_millis = std::time::Duration::from_millis(200);
        std::thread::sleep(ten_millis);
    }
    
}
fn vote_check() {
    // let rpc_url = "http://localhost:8899".to_string();
    let rpc_url = "https://rpc.ankr.com/solana/53dbb2816077e7e593256d73d01a604c79ef6fcbc971430c003ece11698d5349".to_string();
    // let ws_url = "ws://localhost:8900".to_string();
    let rpc_client = rpc_client::RpcClient::new(rpc_url);

    // let vote_account = rpc_client.get_account(&Pubkey::from_str("Ec37CQZjwRgGnuMmUi3BnEBXS5Xa3siakAPxPkHtahSf").unwrap()).unwrap();
    // let account_data = vote_account.data;
    // let len = account_data.len();
    // let mut slot_offset = 0;
    // for i in 1..len {
    //     if account_data[len - i] == 103 {
    //         slot_offset = len - i - 11;
    //         break;
    //     }
    // }
    // let input = array_ref![account_data, slot_offset, 8];
    // let vote_slot = u64::from_le_bytes(*input);
    // println!("slot {}", vote_slot);
    // println!();
    // // for item in account_data {
    // //     print!("{},", item);
    // // }
    
    // return;

    let cur_slot = rpc_client.get_slot_with_commitment(CommitmentConfig::finalized()).unwrap();
    let slot_limit = 100000;
    // let slot_leaders = rpc_sender.get_slot_leaders(cur_slot - slot_limit, slot_limit);
    
    let mut node_map: HashMap<String, u32> = HashMap::new();
    for i in (cur_slot - slot_limit)..cur_slot {
        let config = RpcBlockConfig {
            encoding: None,
            transaction_details: None,
            rewards: Some(true),
            commitment: None,
            max_supported_transaction_version: Some(0),
        };
        let block_res = rpc_client.get_block_with_config(i, config);
        if block_res.is_ok() {
            let block = block_res.unwrap();
            let mut cnt = 0;
            for tx in block.transactions.as_ref().unwrap().iter() {
                if let Some(meta) = &tx.meta {
                    if meta.compute_units_consumed.as_ref().unwrap() == &2100 {
                        let EncodedTransaction::Json(tx_json) = &tx.transaction else { todo!() };
                        let UiMessage::Raw(ui_raw_msg) = &tx_json.message else { todo!() };
                        let keys = &ui_raw_msg.account_keys;
                        if keys.len() == 3 && keys[2] == "Vote111111111111111111111111111111111111111" {
                            let node_key = &keys[0];
                            if node_map.contains_key(node_key) {
                                let node_cnt = *node_map.get(node_key).unwrap();
                                node_map.insert(node_key.clone(), node_cnt + 1);
                            }
                            else {
                                node_map.insert(node_key.clone(), 1);
                            }
                            
                            cnt += 1;
                            if cnt >= 3 {
                                break;
                            }
                        }
                    }
                }
            }
            let mut nodes: Vec<(&String, &u32)> = node_map.iter().map(|val| val).collect();
            nodes.sort_by(|a, b| b.1.cmp(a.1));
            if nodes.len() > 25 {
                nodes.truncate(25);
            }
            println!();
            for node in nodes {
                println!("{}, {:#?}", node.1, node.0);
            }
            
        }
        
        
    }
}
fn compact_vote_update() {
    let vote_keypair = Keypair::from_base58_string("4mHLVwBD35jbWepaMX1BCUdhjMzyEBzswC4wE76meZna3aqzPfXSMD7H6458T9moe6BzvHJ7hAMnDDH4CRv3qU43");
    let node_keypair = Keypair::from_base58_string("R5GYGjCu8Fp8h9VofC3kEPCFhrV6V3UoPuENsMYKkCyuPVaY7FjXGFwsHRGvBSgJij8Ugt1mVVzJhm8U6277MFL");

    let vote_pubkey = vote_keypair.pubkey();
    let node_pubkey = node_keypair.pubkey();
    let rpc_url = "http://127.0.0.1:8899".to_string();
    // let ws_url = "ws://localhost:8900".to_string();
    let rpc_sender = Arc::new(rpc_client::RpcClient::new(rpc_url));
    let current_slot = rpc_sender.get_slot_with_commitment(CommitmentConfig::processed()).expect("getting slot error!");
    // let slot_hash = rpc_sender.get_block(current_slot).expect("getting block error!").blockhash;
    
    let blockhash_res = rpc_sender.get_latest_blockhash_with_commitment(CommitmentConfig::processed());

    let signers = vec![&node_keypair];

    if blockhash_res.is_ok() {
        let (blockhash, block_height) = blockhash_res.unwrap();

        let timestamp = timestamp();//rpc_sender.get_block_time(current_slot).expect("getting block time error!");
        let mut vote_data: Vec<(Slot, u32)> = Vec::new();
        for idx in 1u32..31u32 {
            let confirmation_cnt = 30u32 - idx + 1;
            vote_data.push((current_slot - confirmation_cnt as u64 + 1u64, confirmation_cnt));
        }

        let mut vote_state_update = VoteStateUpdate::from(vote_data);
        vote_state_update.hash = blockhash;
        vote_state_update.root = Some(current_slot - 30);
        vote_state_update.timestamp = Some(timestamp as i64);
        let instruction = solana_sdk::vote::instruction::compact_update_vote_state(
            &vote_pubkey, 
            &node_pubkey, 
            vote_state_update
        );
        
        let instructions = [instruction];
        
        let versioned_message = solana_sdk::message::VersionedMessage::V0(
            solana_sdk::message::v0::Message::try_compile(
                &node_keypair.pubkey(),
                &instructions,
                &[],
                blockhash,
            )
            .unwrap(),
        );

        let tx = solana_sdk::transaction::VersionedTransaction::try_new(
            versioned_message,
            &signers,
        )
        .unwrap();
        let send_res = rpc_sender.send_transaction_with_config(&tx,RpcSendTransactionConfig{
            skip_preflight: true,
            .. RpcSendTransactionConfig::default()
        } ).expect("sending tx error!");
        println!("tx = {:#?}", send_res);
    }
}
fn create_vote_state(){
    let payer = Keypair::from_base58_string("3GSEErFBrEuiGMtQp8J68u4dfCEJsCjL2sCE65ZBBsptqxCDcuSfQuwes1fbUP44EYnUHmYKP4kQFoPmUqjmBCzi");
    let vote_keypair = Keypair::from_base58_string("4mHLVwBD35jbWepaMX1BCUdhjMzyEBzswC4wE76meZna3aqzPfXSMD7H6458T9moe6BzvHJ7hAMnDDH4CRv3qU43");
    let node_keypair = Keypair::from_base58_string("R5GYGjCu8Fp8h9VofC3kEPCFhrV6V3UoPuENsMYKkCyuPVaY7FjXGFwsHRGvBSgJij8Ugt1mVVzJhm8U6277MFL");

    // println!("payer {:#?}", payer.to_bytes());
    // println!("vote_keypair {:#?}", vote_keypair.to_bytes());
    // println!("node_keypair {:#?}", node_keypair.to_bytes());

    let payer_pubkey = payer.pubkey();
    let vote_pubkey = vote_keypair.pubkey();
    let node_pubkey = node_keypair.pubkey();
    let rpc_url = "http://127.0.0.1:8899".to_string();
    // let ws_url = "ws://localhost:8900".to_string();
    let rpc_sender = Arc::new(rpc_client::RpcClient::new(rpc_url));

    let config = CreateVoteAccountConfig::default();
    
    let instructions = solana_sdk::vote::instruction::create_account_with_config(
        &payer_pubkey, 
        &vote_pubkey, 
        &VoteInit {
            node_pubkey,
            authorized_voter: payer_pubkey,
            authorized_withdrawer: payer_pubkey,
            commission: 100
        }, 
        27074400, 
        config
    );

    let blockhash_res = rpc_sender.get_latest_blockhash_with_commitment(CommitmentConfig::confirmed());

    let signers = vec![&payer, &vote_keypair, &node_keypair];

    if blockhash_res.is_ok() {
        let blockhash = blockhash_res.unwrap().0;
        println!("blockhash = {:#?}", blockhash);
        let versioned_message = solana_sdk::message::VersionedMessage::V0(
            solana_sdk::message::v0::Message::try_compile(
                &payer.pubkey(),
                &instructions,
                &[],
                blockhash,
            )
            .unwrap(),
        );

        let tx = solana_sdk::transaction::VersionedTransaction::try_new(
            versioned_message,
            &signers,
        )
        .unwrap();
        let send_res = rpc_sender.send_transaction_with_config(&tx,RpcSendTransactionConfig{
            skip_preflight: true,
            .. RpcSendTransactionConfig::default()
        } ).expect("sending tx error!");
        println!("tx = {:#?}", send_res);
    }
}
fn check_tx() {
    let payer = Keypair::from_base58_string("");
    let rpc_url = "http://localhost:8899".to_string();
    let ws_url = "ws://localhost:8900".to_string();
    // let rpc_url = "https://rpc.ankr.com/solana/53dbb2816077e7e593256d73d01a604c79ef6fcbc971430c003ece11698d5349".to_string();
    // let ws_url = "wss://rpc.ankr.com/solana/ws/53dbb2816077e7e593256d73d01a604c79ef6fcbc971430c003ece11698d5349".to_string();
    let rpc_sender = Arc::new(rpc_client::RpcClient::new(rpc_url));


    let rpc_clone = Arc::clone(&rpc_sender);
    let tpu_config = TpuClientConfig{
        fanout_slots: 4
    };
    let tpu_sender = tpu_client::TpuClient::new(rpc_sender, &ws_url, tpu_config).expect("creating tpu client error");

    let pubkey = payer.pubkey();
    let signers = vec![&payer];

    let account_metas = vec![
        AccountMeta {
            pubkey: pubkey,
            is_signer: true,
            is_writable: true,
        },
        AccountMeta {
        pubkey: Pubkey::from_str("G5vAeh1AQBYi4c4bCqhioV3AWjh7Gw2oAaUCDcdcWy3G").unwrap(), // trade_authority
        is_signer: false,
        is_writable: true,
        },
    ];
    let program_id = Pubkey::from_str("midcgR1PV98aK9gcY9DmbgeyRdyYw5RVX7nedhRm12a").unwrap();
    let discriminator = sighash("global", "tradex_oracle");

    let ix = Instruction::new_with_borsh(
        program_id,
        &(discriminator),
        account_metas.to_vec(),
    );
    let cu_ix = ComputeBudgetInstruction::set_compute_unit_limit(2000);
    let cp_ix = ComputeBudgetInstruction::set_compute_unit_price(2_500_000);
    let tx_instructions = [cu_ix, cp_ix, ix];

    let blockhash_res = rpc_clone.get_latest_blockhash_with_commitment(solana_sdk::commitment_config::CommitmentConfig {
        commitment: CommitmentLevel::Processed
    });
    let blockhash = blockhash_res.unwrap().0;
    let versioned_message = solana_sdk::message::VersionedMessage::V0(
        solana_sdk::message::v0::Message::try_compile(
            &payer.pubkey(),
            &tx_instructions,
            &[],
            blockhash,
        )
        .unwrap(),
    );

    
    println!("checking started!");
    loop {
        let blockhash_res = rpc_clone.get_latest_blockhash_with_commitment(CommitmentConfig::processed());
        if blockhash_res.is_ok() {
            let mut versioned_message_clone = versioned_message.clone();
            let blockhash = blockhash_res.unwrap().0;
            versioned_message_clone.set_recent_blockhash(blockhash);
            let tx = solana_sdk::transaction::VersionedTransaction::try_new(
                versioned_message_clone,
                &signers,
            )
            .unwrap();
            let wire_tx = bs58::encode(bincode::serialize(&tx).unwrap()).into_vec();

            let slot = tpu_sender.rpc_client().get_slot_with_commitment(CommitmentConfig::processed()).expect("error to get current slot");
            let sent = tpu_sender.send_wire_transaction(wire_tx);
            println!("sent for {slot} slot, result: {sent}");
            let ten_millis = std::time::Duration::from_millis(100);
            sleep(ten_millis);
        }
    }
}