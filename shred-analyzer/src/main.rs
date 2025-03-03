use {
    assert_matches::assert_matches, bincode::serialized_size, rand::{seq::SliceRandom, Rng}, 
    solana_entry::entry::Entry, 
    solana_ledger::shred::{verify_test_data_shred, Error, ProcessShredsStats, ReedSolomonCache, Shred, ShredData, ShredType, Shredder, DATA_SHREDS_PER_FEC_BLOCK}, 
    solana_sdk::{
        clock::Slot, hash::Hash, pubkey::Pubkey, signature::Keypair, signer::Signer, system_transaction
    }, std::{
        borrow::Borrow, collections::HashSet, fmt::Debug, fs::OpenOptions, io::Write, sync::{Arc, Mutex}
        
    }
};

fn main() {
    println!("Shred stream analyzer!");
    run_test_data_shredder(32543536, true, false);
    println!("done!");
}
pub fn run_test_data_shredder(slot: Slot, chained: bool, is_last_in_slot: bool) {
    let keypair = Arc::new(Keypair::new());

    // Test that parent cannot be > current slot
    assert_matches!(
        Shredder::new(slot, slot + 1, 0, 0),
        Err(Error::InvalidParentSlot { .. })
    );
    // Test that slot - parent cannot be > u16 MAX
    assert_matches!(
        Shredder::new(slot, slot - 1 - 0xffff, 0, 0),
        Err(Error::InvalidParentSlot { .. })
    );
    let parent_slot = slot - 5;
    let shredder = Shredder::new(slot, parent_slot, 0, 0).unwrap();
    let entries: Vec<_> = (0..5)
        .map(|_| {
            let keypair0 = Keypair::new();
            let keypair1 = Keypair::new();
            let tx0 =
                system_transaction::transfer(&keypair0, &keypair1.pubkey(), 1000, Hash::new_unique());
            Entry::new(&Hash::new_unique(), 1, vec![tx0])
        })
        .collect();
    // println!("origin_txs {:#?}", entries);
    let origin_txs_content = format!("origin_txs {:#?}", entries);
    let mut origin_txs_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("/root/project/origin_txs.txt").expect("oppen error");
    origin_txs_file.write_all(origin_txs_content.as_bytes()).expect("write error");
    origin_txs_file.flush().expect("flush error");
    println!("saved origin_txs to file");

    let size = serialized_size(&entries).unwrap() as usize;
    // Integer division to ensure we have enough shreds to fit all the data
    let data_buffer_size = ShredData::capacity(/*merkle_proof_size:*/ None).unwrap();
    let num_expected_data_shreds = (size + data_buffer_size - 1) / data_buffer_size;
    let num_expected_data_shreds = num_expected_data_shreds.max(if is_last_in_slot {
        DATA_SHREDS_PER_FEC_BLOCK
    } else {
        1
    });
    let num_expected_coding_shreds =
        get_erasure_batch_size(num_expected_data_shreds, is_last_in_slot)
            - num_expected_data_shreds;
    let start_index = 0;
    let (data_shreds, coding_shreds) = shredder.entries_to_shreds(
        &keypair,
        &entries,
        is_last_in_slot,
        // chained_merkle_root
        chained.then(|| Hash::new_from_array(rand::thread_rng().gen())),
        start_index, // next_shred_index
        start_index, // next_code_index
        true,        // merkle_variant
        &ReedSolomonCache::default(),
        &mut ProcessShredsStats::default(),
    );

    // println!("encoded {:#?}, {:#?}", data_shreds, coding_shreds);
    let encoded_shreds_content = format!("encoded {:#?}, {:#?}", data_shreds, coding_shreds);
    let mut encoded_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("/root/project/encoded.txt").expect("oppen error");
    encoded_file.write_all(encoded_shreds_content.as_bytes()).expect("write error");
    encoded_file.flush().expect("flush error");
    println!("saved encoded to file");

    let next_index = data_shreds.last().unwrap().index() + 1;
    assert_eq!(next_index as usize, num_expected_data_shreds);

    let mut data_shred_indexes = HashSet::new();
    let mut coding_shred_indexes = HashSet::new();
    for shred in data_shreds.iter() {
        assert_eq!(shred.shred_type(), ShredType::Data);
        let index = shred.index();
        let is_last = index as usize == num_expected_data_shreds - 1;
        verify_test_data_shred(
            shred,
            index,
            slot,
            parent_slot,
            &keypair.pubkey(),
            true, // verify
            is_last && is_last_in_slot,
            is_last,
        );
        assert!(!data_shred_indexes.contains(&index));
        data_shred_indexes.insert(index);
    }

    for shred in coding_shreds.iter() {
        let index = shred.index();
        assert_eq!(shred.shred_type(), ShredType::Code);
        verify_test_code_shred(shred, index, slot, &keypair.pubkey(), true);
        assert!(!coding_shred_indexes.contains(&index));
        coding_shred_indexes.insert(index);
    }

    for i in start_index..start_index + num_expected_data_shreds as u32 {
        assert!(data_shred_indexes.contains(&i));
    }

    for i in start_index..start_index + num_expected_coding_shreds as u32 {
        assert!(coding_shred_indexes.contains(&i));
    }

    assert_eq!(data_shred_indexes.len(), num_expected_data_shreds);
    assert_eq!(coding_shred_indexes.len(), num_expected_coding_shreds);

    // Test reassembly
    let deshred_payload = Shredder::deshred(&data_shreds).unwrap();
    // println!("binary data {:#?}", deshred_payload);
    let decoded_binary_content = format!("decoded binary {:#?}", deshred_payload);
    let mut decoded_binary_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("/root/project/decoded_binary.txt").expect("oppen error");
    decoded_binary_file.write_all(decoded_binary_content.as_bytes()).expect("write error");
    decoded_binary_file.flush().expect("flush error");
    println!("saved decoded_binary to file");

    let deshred_entries: Vec<Entry> = bincode::deserialize(&deshred_payload).unwrap();
    // println!("decoded {:#?}", deshred_entries);
    let decoded_serialized_content = format!("decoded serialized {:#?}", deshred_entries);
    let mut decoded_serialized_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("/root/project/decoded_serialized.txt").expect("oppen error");
    decoded_serialized_file.write_all(decoded_serialized_content.as_bytes()).expect("write error");
    decoded_serialized_file.flush().expect("flush error");
    println!("saved decoded_serialized to file");

    assert_eq!(entries, deshred_entries);
}
// Maps number of data shreds to the optimal erasure batch size which has the
// same recovery probabilities as a 32:32 erasure batch.
pub const ERASURE_BATCH_SIZE: [usize; 33] = [
    0, 18, 20, 22, 23, 25, 27, 28, 30, // 8
    32, 33, 35, 36, 38, 39, 41, 42, // 16
    43, 45, 46, 48, 49, 51, 52, 53, // 24
    55, 56, 58, 59, 60, 62, 63, 64, // 32
];
/// Maps number of data shreds in each batch to the erasure batch size.
pub fn get_erasure_batch_size(num_data_shreds: usize, is_last_in_slot: bool) -> usize {
    let erasure_batch_size = ERASURE_BATCH_SIZE
        .get(num_data_shreds)
        .copied()
        .unwrap_or(2 * num_data_shreds);
    if is_last_in_slot {
        erasure_batch_size.max(2 * DATA_SHREDS_PER_FEC_BLOCK)
    } else {
        erasure_batch_size
    }
}
pub fn verify_test_code_shred(shred: &Shred, index: u32, slot: Slot, pk: &Pubkey, verify: bool) {
    assert_matches!(shred.sanitize(), Ok(()));
    assert!(!shred.is_data());
    assert_eq!(shred.index(), index);
    assert_eq!(shred.slot(), slot);
    assert_eq!(verify, shred.verify(pk));
}