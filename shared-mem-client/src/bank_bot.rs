use {
    crate::{big_num::{MulDiv, UnsafeMathTrait, U128}, invariant::{self, invariant_math}, orca_whirlpool, safe_math::SafeMath, u128x128_math::Rounding, utils_math::{safe_mul_shr_cast, safe_shl_div_cast}}, anchor_lang::solana_program::system_instruction::SystemInstruction, arrayref::{array_ref, array_refs}, borsh::{BorshDeserialize, BorshSerialize}, dashmap::DashMap, futures::{
      stream::{FuturesUnordered, StreamExt},
      TryFutureExt,
    }, 
    // hwloc::{CpuSet, Topology, CPUBIND_THREAD}, 
    num_traits::Pow, rand::seq::SliceRandom, serde::{Deserialize, Serialize}, solana_account_decoder::{UiAccount, UiAccountData, UiAccountEncoding, UiDataSliceConfig}, solana_client::{client_error::Result as ClientResult, rpc_client::RpcClient, rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig, RpcSendTransactionConfig, RpcSimulateTransactionAccountsConfig, RpcSimulateTransactionConfig}, rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType}, rpc_response::{RpcResult, RpcSimulateTransactionResult}}, solana_measure::{measure::Measure, measure_us}, solana_rpc_client_api::bundles::{RpcSimulateBundleConfig, RpcSimulateBundleResult}, solana_sdk::{
        account::{AccountSharedData, ReadableAccount}, address_lookup_table::{self, AddressLookupTableAccount}, bundle::VersionedBundle, commitment_config::{CommitmentConfig, CommitmentLevel}, compute_budget::ComputeBudgetInstruction, hash::Hash, instruction::{AccountMeta, Instruction}, message::{AddressLoader, VersionedMessage::V0}, pubkey::Pubkey, signature::{self, Keypair, Signature, Signer}, signer::SignerError, system_instruction::transfer, transaction::VersionedTransaction
    }, std::{
        collections::HashMap, fmt, fs::{File, OpenOptions, self}, io::{Read, Write}, ops::{Add, Mul}, str::FromStr, sync::{
            atomic::{AtomicBool, AtomicPtr, AtomicU64, AtomicU8, AtomicUsize, Ordering}, Arc, Mutex, RwLock
        }, thread::{sleep, spawn}, time::{Duration, UNIX_EPOCH}
    }, tokio::task::spawn_blocking
};

pub const DIRECT_SEND: bool = false;

pub const MAX_PNL:u64 = 1_000_000_000;
pub const PNL_THRESHOLD:u64 = 100_000;
pub const TIP_PERCENT:u64 = 4500;
pub const PRIORITY_FEE_PERCENT_BUNDLE:u64 = 10;
pub const PRIORITY_FEE_PERCENT_DIRECT:u64 = 5100;
pub const SOL_USDC_RATE_MULTIPLIER: u64 = 1000;
pub const SOL_USDC_RATE: u64 = 5200;

pub const CLMM_SWAP_TYPES: [SwapType; 1] = [
    // SwapType::RaydiumConcentrated,
    // SwapType::OrcaWhirlpool,
    // SwapType::CropperWhirlpool,
    // SwapType::InvariantSwap,
    // SwapType::CremaFinance,
    SwapType::MeteoraDlmm,
];
pub const CLMM_TICK_START_POS: [(usize, usize); 6] = [(14, 3), (8, 3), (8, 3), (11, 13), (15, 3), (16, 3)];

const PREFIX_FINAL: &str = "Program log: xswap-log-final:";
pub const STACK_ELAPSED_MILLIS: u128 = 10;

pub const VOTE_ACCOUNTS: [&str; 25] = [
    "5eJQDSbgTZSEmH3zSWDEdAKgjavUUn9BkouCFNLz1x93",
    "Ec37CQZjwRgGnuMmUi3BnEBXS5Xa3siakAPxPkHtahSf",
    "Hgv6JZhwTDyNywaANyzQqfX9ScMSdFzqKtGubFbf77AJ",
    "2ZThwAmKij9QCs6ifEVnhE52Kp1aYRsq6iHQJQvwAytq",
    "4T799AaK9YT7zBtVqYZEnCY5ihUF5XaEYemwr5EnozoQ",
    "HsCdVYYZAVSKhykpJdrmNKN15ePR8WugGXMbkJb8xdsU",
    "36MVUhntTiTY7nsLyoCdRj4wbs2rvw2nPEZiM5XCkJLb",
    "GuPnsaM3j4ojKe2KJpcaAWNmhk5n2kaJxc8ZKkowdxrw",
    "6X8ZbfHrRgmLuPBRAsxDMjUv49yAt9T4M9W8WxXU7cQf",
    "5NbEJGaA9J3TL7ZkwFxwAmeNLsUd6K3miPDjcmesE1av",
    "B88zXQkusXcsMn3e28ZCjZHWAAYv9ryBHXZj3EwcsSPm",
    "BUS3ixL7siraZR1tLhtquN4useHGRT87m5NJHLuMSSMp",
    "4Nx9YfQSPe7Xv9Fwzg7T2Sy5wJGagKTkunpJwkMyciNx",
    "4ZxDeRyvhXi7Bc23qPFkeZptzE8J54eimL8phVPN5AXa",
    "SNPWYSmpVdmFyzEm2bgQw88qZQc2bCXoH4s51SFvgsV",
    "jUP5hCf2fGJEz8F2j2gACezxFYCNE9zo1eTMsRQjRK9",
    "8EHeAESs87EejfEMi6u1ndexywxGMAanV7QsUpDiseWP",
    "8ACcAUtFEEYRYGJ5CPPBco8NvbwhjZKsgv7Fma64QDPM",
    "6tZcqt31FzJv6DhnNeS8xBCH66gvcL5ACrfqyqxQvgJg",
    "CviQXDMPZPVAZ5qgVGN2gSBSD9GqqwRt7VZidoHMpArr",
    "ACyyUk2AF8XCkyjVKL6kMxUHqaud5wrXo44jawohn3qT",
    "DKnZkpsqtS4kpFP5zWudrg8w9eCUHixJ2QCRtwwxh8Me",
    "UVxJgkAkEiu5wqpR8JkBsF6yV3CPMfPivk8nUpCDKw6",
    "46Q2KgTuA1dieZrdxpNBz56ujCxz3mZmwTQY7iNgbuxc",
    "EsnouLyTd4WcFyBtdL6XFWnVRnzuZj7ZPfYpawdsiW1n"
];
pub const RAYDIUM_CLMM: &str = "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK";
pub const ORCA_WHIRLPOOL: &str = "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc";
pub const CROPPER_WHIRLPOOL: &str = "H8W3ctz92svYg6mkn1UtGfu2aQr2fnUFHM1RhScEtQDt";
pub const INVARIANT_SWAP: &str = "HyaB3W9q6XdA5xwpU4XnSZV94htfmbmqJXZcEbRaJutt";
pub const CREMA_FINANCE: &str = "CLMM9tUoggJu2wagPkkqs9eFG4BWhVBZWkP1qv3Sp7tR";
pub const METEORA_DLMM: &str = "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo";

// pub const ADMIN_KEYPAIR_PATH: &str = "/home/sol/project/mcard-contract/runner-rust/admin.json";
pub const PAIR_FILE_PATH: &str =
    "/home/sol/project/mcard-contract/json-data/non-stable/serialized-non-stable-pools.bin";
pub const PAIRS_V2_FILE_PATH: &str = "/root/project/pairs_2024_12_05/serialized-non-stable-pools.bin";
pub const ROUTES_V2_FILE_PATH: &str = "/root/project/pairs_2024_12_05/routes-sol-x.json";
pub const LOOKUPTABLE_V2_FILE_PATH: &str =
    "/root/project/pairs_2024_12_05/v2-jup-lookups.json";

pub const LOOKUPTABLE_FILE_PATH: &str =
    "/home/sol/project/mcard-contract/json-data/standard/lookuptable.json";
pub const ROUTES_FILE_PATH: &str = "/home/sol/project/mcard-contract/json-data/routes-full/";
pub const GROUP_FOLDER_PATH: &str = "/home/sol/project/mcard-contract/json-data/groups-v2-full/";
pub const POOL_ADDRESSES_FILE_PATH: &str = "/home/sol/project/mcard-contract/json-data/non-stable/non-stable-pool-addresses-origin.json";
pub const TOTAL_ADDRESSES_TO_SUB_FILE_PATH: &str = "/home/sol/project/mcard-contract/json-data/non-stable/non-stable-pool-addresses.json";

pub const TICK_FILE_PATH: &str = "/root/project/pairs_2024_12_05/tick-array.json";
pub const ROUTE_BIN_FILE_PATH1: &str = "/mnt/ledger/route-bin-files/";
pub const ROUTE_BIN_FILE_PATH2: &str = "/mnt/ledger/route-bin-files2/";
pub const ROUTE_BIN_FILE_PATH3: &str = "/mnt/ledger/route-bin-files3/";
pub const ROUTE_BIN_FILE_PATH4: &str = "/mnt/ledger/route-bin-files4/";

// pub const PAIR_FILE_PATH: &str =
//     "/home/sol/project/mcard-contract/json-data/standard/serialized-standard-pools.bin";
// pub const LOOKUPTABLE_FILE_PATH: &str =
//     "/home/sol/project/mcard-contract/json-data/standard/lookuptable.json";
// pub const ROUTES_FILE_PATH: &str = "/home/sol/project/mcard-contract/json-data/standard/standard-routes.json";
// pub const GROUP_FOLDER_PATH: &str = "/home/sol/project/mcard-contract/json-data/standard/groups/";
// pub const POOL_ADDRESSES_FILE_PATH: &str = "/home/sol/project/mcard-contract/json-data/standard/standard-pool-addresses.json";

pub const RPC_ENDPOINT: &str = "http://127.0.0.1:8899";
pub const WSS_ENDPOINT: &str = "ws://127.0.0.1:8900";
pub const DIED_POOLS: [&str; 1] = [
    "DbE17qTz61Ryj2DfZNiUm7Dcdxp5Vf7LTZQvfueaUNTB"
];
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
pub const TICK_RPC_ENDPOINTS: [&str; 2007] = [
    "https://mainnet.helius-rpc.com/?api-key=99afd7d2-82de-4bcf-bf62-63e550f2cb5f",
    "https://mainnet.helius-rpc.com/?api-key=757dd266-b5d5-4d48-9db9-410dabcc0e1b",
    "https://mainnet.helius-rpc.com/?api-key=5f1aaced-0223-4a1e-8572-c66b4b87ca33",
    "https://mainnet.helius-rpc.com/?api-key=64a20088-b8d7-4d73-b77c-f2a4f14e9c2e",
    "https://mainnet.helius-rpc.com/?api-key=6f0fbc41-a8fc-484e-a4b8-8abff4557858",
    "https://mainnet.helius-rpc.com/?api-key=6904585b-8b5d-433c-b11e-4854a30663ad",
    "https://mainnet.helius-rpc.com/?api-key=a73bc059-76a3-4a47-a123-c7f380f684f6",
    "https://mainnet.helius-rpc.com/?api-key=a033ad18-1a76-4c44-8d25-47dd6895989d",
    "https://mainnet.helius-rpc.com/?api-key=c7fdde09-8a48-4f7e-bd99-1eabc63c0d04",
    "https://mainnet.helius-rpc.com/?api-key=cafd36e9-9ca0-4b86-b124-ea96463a283c",
    "https://mainnet.helius-rpc.com/?api-key=902645f8-92f7-434f-90f5-7cce91e9597c",
    "https://mainnet.helius-rpc.com/?api-key=4c71cbc7-cac1-4703-ad0c-801f08195f5e",
    "https://mainnet.helius-rpc.com/?api-key=ea311a03-c44f-4797-af9c-a6e8b3008843",
    "https://mainnet.helius-rpc.com/?api-key=6ff7cae5-0574-485b-83f2-da1993d4ff02",
    "https://mainnet.helius-rpc.com/?api-key=4868e780-1dd8-4ad5-baa5-ae842769c5e0",
    "https://mainnet.helius-rpc.com/?api-key=0cb44597-a3f8-4caa-b8dc-fac0a411d4ec",
    "https://mainnet.helius-rpc.com/?api-key=64a0fa9e-9f23-4dc0-b988-51a1f7052858",
    "https://mainnet.helius-rpc.com/?api-key=54db2d1c-e384-4ddb-9e2e-aa021e9dac54",
    "https://mainnet.helius-rpc.com/?api-key=18c6955b-ccff-4bb1-b0bb-f055a11c38e0",
    "https://mainnet.helius-rpc.com/?api-key=ec2c1a03-f6c8-4038-a532-c3b630a50589",
    "https://mainnet.helius-rpc.com/?api-key=2f497073-5827-41b7-9ea4-981c00ba9f1d",
    "https://mainnet.helius-rpc.com/?api-key=f94c5d60-a2dc-4c61-8fe9-baa6f8a2da2d",
    "https://mainnet.helius-rpc.com/?api-key=0b1df2c2-bcdc-41f5-ab1f-73e44df828e4",
    "https://mainnet.helius-rpc.com/?api-key=5e567d6e-a655-4f6a-9368-9041f9712dbd",
    "https://mainnet.helius-rpc.com/?api-key=4908c6ee-a7f5-4bbb-a6d2-1a3be69b48b1",
    "https://mainnet.helius-rpc.com/?api-key=04969cd9-735d-48ea-8d8a-7fc95fbd6aa0",
    "https://mainnet.helius-rpc.com/?api-key=d6a2063a-bae9-4639-8114-16c35e38e59e",
    "https://mainnet.helius-rpc.com/?api-key=bb295e4e-0b38-47bf-b22f-1001ba46f66d",
    "https://mainnet.helius-rpc.com/?api-key=9caf3414-d87a-4b8d-8c84-c9b9e2555e32",
    "https://mainnet.helius-rpc.com/?api-key=ece0f521-844a-4a4b-a9fa-a23e437e7d0c",
    "https://mainnet.helius-rpc.com/?api-key=4fdd6b8d-12d8-4ed8-a12d-45f1166b6aa3",
    "https://mainnet.helius-rpc.com/?api-key=1d5e15db-ef79-4eab-8f04-253d40204377",
    "https://mainnet.helius-rpc.com/?api-key=6890e722-8705-4fab-930f-2045e02ec844",
    "https://mainnet.helius-rpc.com/?api-key=3f968b88-883a-48ac-b978-f7d5334aedaf",
    "https://mainnet.helius-rpc.com/?api-key=011bf631-a520-4055-872f-03cc0f8fd54e",
    "https://mainnet.helius-rpc.com/?api-key=cd28265a-f522-4bb2-b9fe-f692ce5a7651",
    "https://mainnet.helius-rpc.com/?api-key=bf51a246-5913-469c-87e0-fa0f034d428f",
    "https://mainnet.helius-rpc.com/?api-key=149931be-b29a-492a-b54c-fa6f829f862f",
    "https://mainnet.helius-rpc.com/?api-key=ba614d3b-93d1-4d69-93f5-fba4b606a4e9",
    "https://mainnet.helius-rpc.com/?api-key=fead835c-f352-4980-8214-21cbdd242a21",
    "https://mainnet.helius-rpc.com/?api-key=d55e222b-2f7f-4d3d-a976-17ac1ac5c782",
    "https://mainnet.helius-rpc.com/?api-key=2d526693-77f0-4085-a8ef-729cf1f3030e",
    "https://mainnet.helius-rpc.com/?api-key=fc702bd5-2013-431b-838c-3227742b9679",
    "https://mainnet.helius-rpc.com/?api-key=eb4e16d2-8240-4ab8-9cdc-3ffe2532a809",
    "https://mainnet.helius-rpc.com/?api-key=6c816777-68a3-4eb2-9848-6f795dcf033e",
    "https://mainnet.helius-rpc.com/?api-key=c21514aa-a4eb-4e69-9241-e7719da0d355",
    "https://mainnet.helius-rpc.com/?api-key=ef24589b-1845-4916-aecf-727ffd2a5c82",
    "https://mainnet.helius-rpc.com/?api-key=6c74db77-4cd3-442f-8b2f-662df3deddf6",
    "https://mainnet.helius-rpc.com/?api-key=913ebdcf-4a36-4b0b-9d05-4e49f4b67c4f",
    "https://mainnet.helius-rpc.com/?api-key=ec7d045c-5235-4ceb-9500-c3e105348848",
    "https://mainnet.helius-rpc.com/?api-key=66b11e6e-6e48-425c-90ff-4e058a834955",
    "https://mainnet.helius-rpc.com/?api-key=9c890e1f-121a-43df-9f92-f7320e5dc064",
    "https://mainnet.helius-rpc.com/?api-key=eaeae4e2-ffc1-468f-84a4-d138c4781509",
    "https://mainnet.helius-rpc.com/?api-key=31f5b35d-f40f-4a9b-8f14-d67205796c74",
    "https://mainnet.helius-rpc.com/?api-key=2f800f1a-a334-42a3-8b52-5d7c0957b0c5",
    "https://mainnet.helius-rpc.com/?api-key=a247c67d-dbf4-428d-840a-8b0f3f7eb1f6",
    "https://mainnet.helius-rpc.com/?api-key=0e93a05f-1235-4629-8a21-64c41e938575",
    "https://mainnet.helius-rpc.com/?api-key=606517e2-15fa-4889-85e7-af32be346caa",
    "https://mainnet.helius-rpc.com/?api-key=ef40771f-f7a5-4fca-a02f-09a2e46aeefa",
    "https://mainnet.helius-rpc.com/?api-key=9149eaae-2f1f-4f12-b67a-557b25593bb5",
    "https://mainnet.helius-rpc.com/?api-key=26d5c233-1d80-4ff6-a266-b59a98dbe8d8",
    "https://mainnet.helius-rpc.com/?api-key=f4a10480-821f-4146-a179-2848144da17d",
    "https://mainnet.helius-rpc.com/?api-key=263bd809-6644-4971-af91-51010d5e0140",
    "https://mainnet.helius-rpc.com/?api-key=9fc51110-f42a-4560-bb11-b13bb1a59df4",
    "https://mainnet.helius-rpc.com/?api-key=d61a7eb5-5e8b-448b-b4cb-63f4515fb02f",
    "https://mainnet.helius-rpc.com/?api-key=90d526bc-b7a8-4d50-85f5-8af082548b21",
    "https://mainnet.helius-rpc.com/?api-key=e675c2d0-bd7a-4a21-893a-05ec21dbf172",
    "https://mainnet.helius-rpc.com/?api-key=1b3175b5-6f99-4c3d-8b47-be9a70a7f46a",
    "https://mainnet.helius-rpc.com/?api-key=6d7d5843-7842-4b2b-88d9-b9bbbc7a97b2",
    "https://mainnet.helius-rpc.com/?api-key=f7477f94-217d-47dc-8dd1-96493387aa51",
    "https://mainnet.helius-rpc.com/?api-key=fc625997-dd41-4941-aad4-bd185fab2229",
    "https://mainnet.helius-rpc.com/?api-key=f06b9d08-d0ad-46cb-9f38-31eee75fd4b2",
    "https://mainnet.helius-rpc.com/?api-key=59256b65-5255-4eb9-992e-88a1f8823fb0",
    "https://mainnet.helius-rpc.com/?api-key=f4beaf1f-5bf1-4dfc-b5f5-89d71d2664b3",
    "https://mainnet.helius-rpc.com/?api-key=ccdbee18-7c3d-47e1-8813-a9a1eabb5613",
    "https://mainnet.helius-rpc.com/?api-key=01492c75-3027-4805-b6b3-f03c53a9c372",
    "https://mainnet.helius-rpc.com/?api-key=2c93ae59-1ca3-4f4f-94f1-19ab181d2c6c",
    "https://mainnet.helius-rpc.com/?api-key=0c5c7a55-c255-4f11-91d7-9e88d03ff48c",
    "https://mainnet.helius-rpc.com/?api-key=ecc6332d-9f94-44ea-9c50-80252016a0d9",
    "https://mainnet.helius-rpc.com/?api-key=18e755d1-1774-4f7d-b8b1-2fd250f622ae",
    "https://mainnet.helius-rpc.com/?api-key=55d3797f-51d9-4734-99a2-902d987e1d1d",
    "https://mainnet.helius-rpc.com/?api-key=87e91f7b-520d-42a4-b59c-0c84540234b7",
    "https://mainnet.helius-rpc.com/?api-key=9b992571-6840-488c-a26e-0cff61eefc38",
    "https://mainnet.helius-rpc.com/?api-key=99c8bcac-954a-4d5e-a6b6-ea86b0b41eda",
    "https://mainnet.helius-rpc.com/?api-key=08347d9b-fe0d-4118-a071-a79c18d2f25a",
    "https://mainnet.helius-rpc.com/?api-key=316c3f3f-e0ea-405d-a398-d22d206151d8",
    "https://mainnet.helius-rpc.com/?api-key=0c946c9e-8e08-4e2c-b0fc-4b4558667326",
    "https://mainnet.helius-rpc.com/?api-key=9f408700-5efe-400c-bc5c-f5e680e9acf7",
    "https://mainnet.helius-rpc.com/?api-key=3a4ffedb-8d95-4880-9642-b72365a4f991",
    "https://mainnet.helius-rpc.com/?api-key=6ff66a4e-fcdb-48a1-aee5-aef6e2ef00dc",
    "https://mainnet.helius-rpc.com/?api-key=5bce6a89-9240-4ba0-96c4-bbe44b83117d",
    "https://mainnet.helius-rpc.com/?api-key=519e0d9e-2835-4e8c-93e9-03d55fc8f33d",
    "https://mainnet.helius-rpc.com/?api-key=392e891c-02ab-4378-be91-7596f0b2b053",
    "https://mainnet.helius-rpc.com/?api-key=e68c0218-5808-43b1-9c71-7264d524252a",
    "https://mainnet.helius-rpc.com/?api-key=a661d7ed-2bdb-4143-82aa-5857322c4048",
    "https://mainnet.helius-rpc.com/?api-key=592a245e-0680-47bf-a6ff-0aad711a4f5a",
    "https://mainnet.helius-rpc.com/?api-key=374740ba-b7b5-4eb0-a019-d2314604d156",
    "https://mainnet.helius-rpc.com/?api-key=d4b9cc6e-fcba-4dcb-884b-ed5b60ac0aee",
    "https://mainnet.helius-rpc.com/?api-key=21f7c03a-2728-43cd-8960-d2c53f3e474d",
    "https://mainnet.helius-rpc.com/?api-key=e6a48a8f-f0a5-41b6-8ea5-2f81419aeb0c",
    "https://mainnet.helius-rpc.com/?api-key=d9931b89-a0fe-48c8-9fc8-306fb204908c",
    "https://mainnet.helius-rpc.com/?api-key=71302e85-cba4-4cc7-b926-428ef03117cb",
    "https://mainnet.helius-rpc.com/?api-key=6939e150-5834-4802-80e7-00dc046cce0e",
    "https://mainnet.helius-rpc.com/?api-key=473ae27a-92df-4c5b-9b22-0574e544abd3",
    "https://mainnet.helius-rpc.com/?api-key=8c2219f8-b1a2-457f-8ff0-8193feef11f1",
    "https://mainnet.helius-rpc.com/?api-key=571846be-d533-4617-8a8c-c9df06693704",
    "https://mainnet.helius-rpc.com/?api-key=6c574989-c308-43fe-8359-cfe3e600ebf4",
    "https://mainnet.helius-rpc.com/?api-key=f8f67ee1-f4b9-4474-b3fe-cfdc76b21aa3",
    "https://mainnet.helius-rpc.com/?api-key=b34d4821-414b-4985-ae94-4e9d760cd799",
    "https://mainnet.helius-rpc.com/?api-key=f3ac8021-6b68-4c65-a6bd-9f89359b1153",
    "https://mainnet.helius-rpc.com/?api-key=61fc094a-41b2-49ab-b693-931db1a0671f",
    "https://mainnet.helius-rpc.com/?api-key=4ff397a3-171d-47f3-8333-5ad2d0052c9b",
    "https://mainnet.helius-rpc.com/?api-key=a016d37e-017f-4bd6-b8ad-d622282389c8",
    "https://mainnet.helius-rpc.com/?api-key=da2b90de-ab84-410e-aa17-66c570e221a9",
    "https://mainnet.helius-rpc.com/?api-key=38614566-bc32-43bd-9ec5-c67c2ce1b9a6",
    "https://mainnet.helius-rpc.com/?api-key=bf25365f-0712-46d4-9ca0-35e898afdd5a",
    "https://mainnet.helius-rpc.com/?api-key=18c5b514-c8be-4116-a9f0-f87981732ef3",
    "https://mainnet.helius-rpc.com/?api-key=dec5d167-c322-4a4a-bce3-f01c56387ad6",
    "https://mainnet.helius-rpc.com/?api-key=e09d0939-ca41-4912-b161-0c8ee081bf54",
    "https://mainnet.helius-rpc.com/?api-key=d4fc0157-6adb-4086-91af-bb1d5f5b99ae",
    "https://mainnet.helius-rpc.com/?api-key=95d5e24f-569c-4ace-a36c-e02bd8196090",
    "https://mainnet.helius-rpc.com/?api-key=cd3e226e-7fce-4ce2-967e-4f175b17469d",
    "https://mainnet.helius-rpc.com/?api-key=25387ff7-074f-4897-b44d-aaa8fb4e4fd9",
    "https://mainnet.helius-rpc.com/?api-key=a3d83957-e5be-4c4b-bcbe-220be42c514e",
    "https://mainnet.helius-rpc.com/?api-key=c7575a58-8868-44d6-8cf2-2b55f63490a9",
    "https://mainnet.helius-rpc.com/?api-key=a3a1dbb5-e4ab-4891-b0ba-6368d55a14bd",
    "https://mainnet.helius-rpc.com/?api-key=1cecf4a3-939b-4738-a739-7eb4dd3338f5",
    "https://mainnet.helius-rpc.com/?api-key=338ce22f-5daa-42bf-ad0a-fbd37a4f8ab1",
    "https://mainnet.helius-rpc.com/?api-key=743091d3-381f-4ad6-ad28-9bfb2b190d15",
    "https://mainnet.helius-rpc.com/?api-key=76e09f46-4c36-4b89-9abd-c9d621c96e0b",
    "https://mainnet.helius-rpc.com/?api-key=a62772ed-e624-42f6-b18b-072b5e79a4f4",
    "https://mainnet.helius-rpc.com/?api-key=722c4968-a8b7-45a9-870b-8605c34637a2",
    "https://mainnet.helius-rpc.com/?api-key=89631d6d-3f6f-477b-b6fe-4c89062a84fb",
    "https://mainnet.helius-rpc.com/?api-key=f6847db3-4b62-4b93-83a4-070fc490d7f2",
    "https://mainnet.helius-rpc.com/?api-key=fd94aaa6-25ae-4916-b4c6-5208d5f3f7a0",
    "https://mainnet.helius-rpc.com/?api-key=983b9cb1-b624-4ff8-a1bc-1eb1b655a1b6",
    "https://mainnet.helius-rpc.com/?api-key=d40a9182-9e0c-4534-97e2-aa1f2e727fd1",
    "https://mainnet.helius-rpc.com/?api-key=7e8bbe59-cea3-4175-852b-ef01c3af8125",
    "https://mainnet.helius-rpc.com/?api-key=a2010a51-c457-4682-b364-742380205178",
    "https://mainnet.helius-rpc.com/?api-key=0292ff58-36bb-48e4-a4d2-2b97fa92beb7",
    "https://mainnet.helius-rpc.com/?api-key=20ce0c93-f2d0-4ee7-80e9-c04a01efa3a8",
    "https://mainnet.helius-rpc.com/?api-key=dd59e78d-13a1-4a9a-b2fc-492031000b11",
    "https://mainnet.helius-rpc.com/?api-key=78823eb2-ba99-4064-9f5f-59920630d1fd",
    "https://mainnet.helius-rpc.com/?api-key=41976bcb-cc15-4433-a829-fb4954e287c9",
    "https://mainnet.helius-rpc.com/?api-key=c0655382-a692-49c1-93ff-ac975d7b3a48",
    "https://mainnet.helius-rpc.com/?api-key=873e826a-8558-473c-b8f1-faebc7c14738",
    "https://mainnet.helius-rpc.com/?api-key=4103b361-5755-474c-9e9b-cba4bb9477c4",
    "https://mainnet.helius-rpc.com/?api-key=49c2b861-4ddc-4f02-85b2-287a1708bde9",
    "https://mainnet.helius-rpc.com/?api-key=1eb81db1-40e8-4396-97ce-aa8bb75c9c55",
    "https://mainnet.helius-rpc.com/?api-key=8eb9ff80-6054-4ec2-9d3a-869c7be1628e",
    "https://mainnet.helius-rpc.com/?api-key=d6b2b538-bcaa-4c5b-9ecb-0672949a772d",
    "https://mainnet.helius-rpc.com/?api-key=46161efd-b5df-450a-8746-85f81ddf48a7",
    "https://mainnet.helius-rpc.com/?api-key=ed8a8f07-76c0-45b1-b7eb-40ccc93528bb",
    "https://mainnet.helius-rpc.com/?api-key=86c86e2d-e752-4a13-abeb-e7411be41b9a",
    "https://mainnet.helius-rpc.com/?api-key=40491372-26fd-454d-9d42-df525f7a504d",
    "https://mainnet.helius-rpc.com/?api-key=311399c5-a84e-4a37-9da5-c637a567a93c",
    "https://mainnet.helius-rpc.com/?api-key=cdd6b6af-fa68-492b-8b05-f555e95925ed",
    "https://mainnet.helius-rpc.com/?api-key=51f19c5e-dc82-466d-a845-70c5532e107f",
    "https://mainnet.helius-rpc.com/?api-key=ab11f7a4-7186-4a07-bb82-0aa0ebb5c590",
    "https://mainnet.helius-rpc.com/?api-key=fa91e279-9123-4924-a952-ed71a47b2ea3",
    "https://mainnet.helius-rpc.com/?api-key=3da85ef8-89f2-45e9-bde7-ef3c6944e458",
    "https://mainnet.helius-rpc.com/?api-key=123a7eb6-9ff6-48b0-a136-e1f3473d502a",
    "https://mainnet.helius-rpc.com/?api-key=428267d4-bb23-4adc-805b-42bc64ef6a84",
    "https://mainnet.helius-rpc.com/?api-key=f309e03f-0253-4581-bc22-d643155a170c",
    "https://mainnet.helius-rpc.com/?api-key=7f8480ec-acdf-469b-8069-ab97ff8fe285",
    "https://mainnet.helius-rpc.com/?api-key=3779fd41-8a6f-4329-ae63-9986a1ccfe5b",
    "https://mainnet.helius-rpc.com/?api-key=a207a35e-9bb2-4957-901b-6e6c324ee4b2",
    "https://mainnet.helius-rpc.com/?api-key=4201244c-c960-4b6e-a02a-dc4a4bbe4003",
    "https://mainnet.helius-rpc.com/?api-key=aa81735d-2503-4831-a920-5552a141ecea",
    "https://mainnet.helius-rpc.com/?api-key=79534021-f174-4218-abd0-0c3fcc2f7313",
    "https://mainnet.helius-rpc.com/?api-key=4367ccd4-a59c-4253-bef7-dde48e09bac3",
    "https://mainnet.helius-rpc.com/?api-key=96ac2b48-e1a4-4a86-937a-79e4fcae615b",
    "https://mainnet.helius-rpc.com/?api-key=a76abef4-6006-4f03-8f98-93e1b791face",
    "https://mainnet.helius-rpc.com/?api-key=85d92daa-7af2-41b9-9b00-3cc6aea416ab",
    "https://mainnet.helius-rpc.com/?api-key=9878876e-f8e8-4718-9f70-056fe52cc1c9",
    "https://mainnet.helius-rpc.com/?api-key=b320c4c6-bb0f-4391-b3aa-b505f5bddd5f",
    "https://mainnet.helius-rpc.com/?api-key=30c761f8-b2cf-4042-8fac-0465ff507cfa",
    "https://mainnet.helius-rpc.com/?api-key=ebc87fe7-448f-46f3-84a5-ade65a5048b9",
    "https://mainnet.helius-rpc.com/?api-key=0986aece-f05b-4f14-a21b-e713175d27d6",
    "https://mainnet.helius-rpc.com/?api-key=71210f7a-3833-4b1b-b1e5-3ff99a61e383",
    "https://mainnet.helius-rpc.com/?api-key=7bf98cdc-5ab3-470d-8956-b204c0b72e53",
    "https://mainnet.helius-rpc.com/?api-key=119ffed3-d6e9-4609-8fe8-f4a7d3b0a87b",
    "https://mainnet.helius-rpc.com/?api-key=a389095a-38e2-4da6-a884-82e5375cd501",
    "https://mainnet.helius-rpc.com/?api-key=d05097e3-c7c3-4b2e-a3c5-0e742f7229f2",
    "https://mainnet.helius-rpc.com/?api-key=4c12b401-e704-4fad-a18d-a8897006df41",
    "https://mainnet.helius-rpc.com/?api-key=62e3c023-1795-4974-98c9-38a37719f0e9",
    "https://mainnet.helius-rpc.com/?api-key=b0a1c873-2448-4ef1-a0f2-044baa53d1ae",
    "https://mainnet.helius-rpc.com/?api-key=09a6ffc6-152d-4298-8646-b0385c4d4d28",
    "https://mainnet.helius-rpc.com/?api-key=3fca0397-c24e-4dc2-b795-2b32dc22f17b",
    "https://mainnet.helius-rpc.com/?api-key=68550337-eeaa-4b3b-8b5a-f6975c949d45",
    "https://mainnet.helius-rpc.com/?api-key=ecf85585-9b3a-40b7-9f50-6af077a77ea5",
    "https://mainnet.helius-rpc.com/?api-key=141700c9-b6c2-4d7e-9b71-7deaa498d71a",
    "https://mainnet.helius-rpc.com/?api-key=20966fb1-e999-47e1-b31d-23bad10f975d",
    "https://mainnet.helius-rpc.com/?api-key=0125d0b3-120a-442f-8ae9-887172651ab5",
    "https://mainnet.helius-rpc.com/?api-key=49139784-63e0-4458-bf7e-317c602b771c",
    "https://mainnet.helius-rpc.com/?api-key=8f225ac2-9d57-4746-9d6c-0da0f57c1b27",
    "https://mainnet.helius-rpc.com/?api-key=e6576e0c-5095-4ad1-ac07-1e10d6044671",
    "https://mainnet.helius-rpc.com/?api-key=d6dabb72-b2c0-41c7-8a1e-2baa19ffde6c",
    "https://mainnet.helius-rpc.com/?api-key=47cb9e9a-dbab-439c-a434-77ba7990c24e",
    "https://mainnet.helius-rpc.com/?api-key=2a867e1f-1d58-4120-bda1-a93aa17044dd",
    "https://mainnet.helius-rpc.com/?api-key=5fbbfcbd-1c71-4528-ac0a-e7e1344f6f52",
    "https://mainnet.helius-rpc.com/?api-key=6dfb0bc9-81eb-49be-9eb5-e50a1e0ab7d3",
    "https://mainnet.helius-rpc.com/?api-key=3e80b9d6-9a7b-40de-b0c2-36bab8b744ef",
    "https://mainnet.helius-rpc.com/?api-key=b324ab08-d4d2-4e34-bd77-4c3384416ad1",
    "https://mainnet.helius-rpc.com/?api-key=23094ac6-5f17-4a96-966d-a398256bae8e",
    "https://mainnet.helius-rpc.com/?api-key=c7640fc6-0152-49a5-afb6-3865d36a211c",
    "https://mainnet.helius-rpc.com/?api-key=a10f9873-1e84-434a-a8e9-e6da0ee4ac61",
    "https://mainnet.helius-rpc.com/?api-key=d510d88e-70a3-41bc-b3c5-f6297cf37e2e",
    "https://mainnet.helius-rpc.com/?api-key=7c169f2c-b159-42e5-83e9-1bfe7c521380",
    "https://mainnet.helius-rpc.com/?api-key=e035e47c-0084-4301-b90e-bc7f6cad1b3d",
    "https://mainnet.helius-rpc.com/?api-key=39d3854c-c39d-4ecf-9edc-b803cd0d1f87",
    "https://mainnet.helius-rpc.com/?api-key=85d5cc15-097d-4f83-8ff4-f9282a0279e2",
    "https://mainnet.helius-rpc.com/?api-key=22d0b0c7-d366-48b1-a727-c65e5b9c9f9c",
    "https://mainnet.helius-rpc.com/?api-key=cce4d862-d5e3-485c-9581-bbdeb45ca766",
    "https://mainnet.helius-rpc.com/?api-key=b14beb6e-e65b-44fd-8c9c-b0ee60a06af6",
    "https://mainnet.helius-rpc.com/?api-key=30063d86-d7c2-4d70-b100-45995801b5c8",
    "https://mainnet.helius-rpc.com/?api-key=6ab7132e-680b-494f-b5b5-68b43a213bcd",
    "https://mainnet.helius-rpc.com/?api-key=504dc5bf-d899-4535-93ee-7f357dfaf6e5",
    "https://mainnet.helius-rpc.com/?api-key=f92b0252-1026-4644-a3a3-9b77af79c2a3",
    "https://mainnet.helius-rpc.com/?api-key=56bb0a3e-e8aa-4402-9894-aef7c50afaca",
    "https://mainnet.helius-rpc.com/?api-key=50d63a25-7cb5-4bc2-ac43-1d71078a49e8",
    "https://mainnet.helius-rpc.com/?api-key=eb761ee8-b022-4507-9823-6bc6a90ed9fc",
    "https://mainnet.helius-rpc.com/?api-key=5a256f35-080d-4743-84f9-318bb64d358b",
    "https://mainnet.helius-rpc.com/?api-key=8e14f5b1-3d93-4ee1-84f1-ec71f0e14ce8",
    "https://mainnet.helius-rpc.com/?api-key=90e6dfe9-27bc-4b71-9d66-d1cb149c7846",
    "https://mainnet.helius-rpc.com/?api-key=e753cc40-b479-4923-80f7-335428b3828e",
    "https://mainnet.helius-rpc.com/?api-key=ed4be0a1-0303-4736-8092-c387946730fb",
    "https://mainnet.helius-rpc.com/?api-key=d584efff-0216-4981-8bc1-d056aca805f8",
    "https://mainnet.helius-rpc.com/?api-key=c9f28fbb-3e5f-44b5-8c91-11d0d257639b",
    "https://mainnet.helius-rpc.com/?api-key=dfd7afa0-e677-408c-88eb-bb2303e09aef",
    "https://mainnet.helius-rpc.com/?api-key=940c98d1-02f0-4fa0-9bd0-c5357c608d92",
    "https://mainnet.helius-rpc.com/?api-key=44fc661d-1f12-4bb2-99ad-bb99c36f9273",
    "https://mainnet.helius-rpc.com/?api-key=ff1a07e9-71a2-4ea6-9024-0accbbfdba23",
    "https://mainnet.helius-rpc.com/?api-key=4bb76d08-37de-429b-b91a-8eadd0d89871",
    "https://mainnet.helius-rpc.com/?api-key=a2751d41-17b0-4e36-9914-0a5b2fba386a",
    "https://mainnet.helius-rpc.com/?api-key=4fe7192b-19c9-40fd-9172-fd2974975ae4",
    "https://mainnet.helius-rpc.com/?api-key=529f27ee-9203-44b4-a3ac-b1986d3fc623",
    "https://mainnet.helius-rpc.com/?api-key=f55cc9f3-b918-4daf-bee1-137e612599bc",
    "https://mainnet.helius-rpc.com/?api-key=d3c6c037-49eb-4999-89b2-7859631e3c59",
    "https://mainnet.helius-rpc.com/?api-key=07ebbf22-b993-41cf-b117-3a3eb7abf11a",
    "https://mainnet.helius-rpc.com/?api-key=d9db7e80-9bd0-42bf-a1d4-86efeab0ca6b",
    "https://mainnet.helius-rpc.com/?api-key=661d85e2-eb6c-4ba5-91c2-5048d9c74cd1",
    "https://mainnet.helius-rpc.com/?api-key=d297ce9f-8ec9-4c60-9b5f-ad5cb78fd37b",
    "https://mainnet.helius-rpc.com/?api-key=dcc65efc-499e-406b-8572-1b09035f9a7d",
    "https://mainnet.helius-rpc.com/?api-key=7d5e2389-3b98-4c24-ad72-1909c5cd8ea9",
    "https://mainnet.helius-rpc.com/?api-key=e0f4b757-c67b-4020-b63b-4daa296d660c",
    "https://mainnet.helius-rpc.com/?api-key=189cce89-0e49-44b4-bbb1-fe69b0786fb5",
    "https://mainnet.helius-rpc.com/?api-key=cf0084a4-a8f6-41e6-bf20-749f272d2d6b",
    "https://mainnet.helius-rpc.com/?api-key=b166059d-41fc-45e8-b025-83bd1b06eade",
    "https://mainnet.helius-rpc.com/?api-key=2c2f8b4f-6d1a-4d36-b090-ce257bfd7ba0",
    "https://mainnet.helius-rpc.com/?api-key=ddb6d1a8-28b2-4733-8b26-70705db1b846",
    "https://mainnet.helius-rpc.com/?api-key=e8941e2e-0425-4915-868e-f6471879c94e",
    "https://mainnet.helius-rpc.com/?api-key=d89edc7b-cebe-4ddc-9a59-29b01fbdbd7f",
    "https://mainnet.helius-rpc.com/?api-key=82fab3e3-fcb7-45a6-96a0-91c9b3373609",
    "https://mainnet.helius-rpc.com/?api-key=93a194a6-65c7-4b24-ad36-98c8ad4ef7c0",
    "https://mainnet.helius-rpc.com/?api-key=d3c195c2-221a-4e0e-87a2-6e91ab0c8ba2",
    "https://mainnet.helius-rpc.com/?api-key=2459b6d2-6b22-4a6e-b60f-0011880bf2ea",
    "https://mainnet.helius-rpc.com/?api-key=db7d8517-257e-4fb5-b590-0f05800dda2c",
    "https://mainnet.helius-rpc.com/?api-key=436a8943-c687-4a21-b03b-230e0443d163",
    "https://mainnet.helius-rpc.com/?api-key=43e1641d-b723-4c6b-95a4-b7ebee79c8de",
    "https://mainnet.helius-rpc.com/?api-key=a354489f-db15-493d-94d8-821376c47449",
    "https://mainnet.helius-rpc.com/?api-key=60a75693-90ad-424e-a0a2-2ca90a140d94",
    "https://mainnet.helius-rpc.com/?api-key=5e2f61e9-d782-40c9-b33f-b066aa193a2e",
    "https://mainnet.helius-rpc.com/?api-key=d86cfc7e-0b0a-46c3-a4a6-223d1749458c",
    "https://mainnet.helius-rpc.com/?api-key=a8884a13-ec0e-4bc7-8e69-25fa7df354d6",
    "https://mainnet.helius-rpc.com/?api-key=89cf7a1a-9d05-4573-94a5-5be6ce16aa45",
    "https://mainnet.helius-rpc.com/?api-key=075001d4-9599-4e0d-a495-c6dcfeb8f15e",
    "https://mainnet.helius-rpc.com/?api-key=28a78f78-6ec2-46dc-a663-bc1efdb4f8eb",
    "https://mainnet.helius-rpc.com/?api-key=40600701-adb2-4412-98a4-cf69affceffb",
    "https://mainnet.helius-rpc.com/?api-key=3e314595-d758-4c03-9440-590c15b822dc",
    "https://mainnet.helius-rpc.com/?api-key=8f7c49b7-a3d6-4b06-9a02-50094cb6fa82",
    "https://mainnet.helius-rpc.com/?api-key=0cae3f75-b4de-4b00-9fc5-d2ae747cd2e4",
    "https://mainnet.helius-rpc.com/?api-key=d41783d3-ad51-4a3a-97bf-f2cceca6fd65",
    "https://mainnet.helius-rpc.com/?api-key=fc79ae91-8320-444c-b508-a3a31e68ffd6",
    "https://mainnet.helius-rpc.com/?api-key=eaaf132d-c2e8-41ea-9305-e490283c17a3",
    "https://mainnet.helius-rpc.com/?api-key=1252261a-564c-4427-b264-6ad98ad01ea4",
    "https://mainnet.helius-rpc.com/?api-key=28277113-fdcf-43ac-b165-efcd44484d5e",
    "https://mainnet.helius-rpc.com/?api-key=02b54a34-70aa-4ef1-b001-73966ec87cf4",
    "https://mainnet.helius-rpc.com/?api-key=d03cdacf-5d51-447a-a536-16b6a582f618",
    "https://mainnet.helius-rpc.com/?api-key=d69e7a34-89a9-426d-a458-a04cb15fd72a",
    "https://mainnet.helius-rpc.com/?api-key=59a1b6ff-f7b7-4c39-9fba-4dfefa431946",
    "https://mainnet.helius-rpc.com/?api-key=230027fe-b1e3-49c5-a731-7f00044b47e5",
    "https://mainnet.helius-rpc.com/?api-key=175acfbd-89bb-4531-939a-be9378349b39",
    "https://mainnet.helius-rpc.com/?api-key=921d1a73-d0c0-498e-96e9-8271968b11a3",
    "https://mainnet.helius-rpc.com/?api-key=f4402dcb-8c52-4cd6-913a-dd087aa97699",
    "https://mainnet.helius-rpc.com/?api-key=3bcde019-bdbd-4945-80da-9b98553c6b24",
    "https://mainnet.helius-rpc.com/?api-key=398942ef-e20e-4964-a530-982f1c953b89",
    "https://mainnet.helius-rpc.com/?api-key=f30450ac-04c5-4b6f-98c0-5a397f5a594c",
    "https://mainnet.helius-rpc.com/?api-key=12f06eab-0774-4271-b3dc-a086270a21f7",
    "https://mainnet.helius-rpc.com/?api-key=ad6e59f3-3ee6-4ee8-a624-a9a4b957af5d",
    "https://mainnet.helius-rpc.com/?api-key=7722f97b-7495-4ad4-b8fa-9e339bbf69dd",
    "https://mainnet.helius-rpc.com/?api-key=afce6df6-17a4-4f3c-b458-807e97f10da7",
    "https://mainnet.helius-rpc.com/?api-key=50cc7652-4234-4882-b724-bccf723c5979",
    "https://mainnet.helius-rpc.com/?api-key=d8c5588b-ee6a-44c2-93a7-a86802297264",
    "https://mainnet.helius-rpc.com/?api-key=00fd1bd8-a30b-4324-90e5-c1e7c266c794",
    "https://mainnet.helius-rpc.com/?api-key=d02a4e00-348e-4ea2-be54-882a4fb7c9f0",
    "https://mainnet.helius-rpc.com/?api-key=c7178992-d1cc-4eca-bf2b-6f6a28552ff7",
    "https://mainnet.helius-rpc.com/?api-key=e440d508-5c9b-47cb-97da-ab7107b40c14",
    "https://mainnet.helius-rpc.com/?api-key=3d6f8049-faef-4701-a44a-92c5b39ccf8f",
    "https://mainnet.helius-rpc.com/?api-key=217dfe13-5c9b-47dd-8b86-cbefc79becb8",
    "https://mainnet.helius-rpc.com/?api-key=bdc776fa-aa20-47d9-97af-bba6c06989ff",
    "https://mainnet.helius-rpc.com/?api-key=b6c84cec-ddd4-4195-b669-c5e75aabb092",
    "https://mainnet.helius-rpc.com/?api-key=67275e89-a06f-4753-a6cd-7d9ee3db6bd1",
    "https://mainnet.helius-rpc.com/?api-key=cbf56cc9-0b01-45a3-8844-290b498dcd53",
    "https://mainnet.helius-rpc.com/?api-key=f528d774-ddde-4b85-9ede-ec20d6f68753",
    "https://mainnet.helius-rpc.com/?api-key=d05ff78d-4ab1-4a0b-944e-9134f4a2617f",
    "https://mainnet.helius-rpc.com/?api-key=c7b18138-c9ee-447b-8833-5979623fd3e9",
    "https://mainnet.helius-rpc.com/?api-key=afdf6ad3-1ea2-457b-8fd3-f9deae6b1fa6",
    "https://mainnet.helius-rpc.com/?api-key=750b9abe-f891-4ffb-8190-496304659bbc",
    "https://mainnet.helius-rpc.com/?api-key=3ec5d5f3-84d1-4d03-8db4-76ae635731d1",
    "https://mainnet.helius-rpc.com/?api-key=906c0f87-4019-4770-8700-bdf8ecb0e4f6",
    "https://mainnet.helius-rpc.com/?api-key=8ca278c4-4beb-483a-a275-1a9105b6fa6d",
    "https://mainnet.helius-rpc.com/?api-key=ea459094-5fcb-471a-837d-f35114a328b3",
    "https://mainnet.helius-rpc.com/?api-key=4b234614-f85e-40cd-83a1-bed0b870f13a",
    "https://mainnet.helius-rpc.com/?api-key=86719f11-88dc-4fc1-a280-be54530db420",
    "https://mainnet.helius-rpc.com/?api-key=235926e9-21a5-43bf-9859-d339eb6a70eb",
    "https://mainnet.helius-rpc.com/?api-key=87e2f5e0-5734-40a5-8530-9059c61f7afa",
    "https://mainnet.helius-rpc.com/?api-key=cb8cc547-bb1a-41cb-b930-a3565ac03d17",
    "https://mainnet.helius-rpc.com/?api-key=1531bee4-68ce-49ba-8f62-d7a22777f088",
    "https://mainnet.helius-rpc.com/?api-key=d3d7a37b-b6bb-4046-8a3f-a72cc61602df",
    "https://mainnet.helius-rpc.com/?api-key=2275e009-576a-4038-b925-cda864ded1dc",
    "https://mainnet.helius-rpc.com/?api-key=d8926643-224d-4a3e-8fe2-c7cd23ccc972",
    "https://mainnet.helius-rpc.com/?api-key=f5368ff7-9295-455b-bfbe-73ed38a59466",
    "https://mainnet.helius-rpc.com/?api-key=f5ffcde0-c477-42af-8b9c-068ea434ba04",
    "https://mainnet.helius-rpc.com/?api-key=f62120f8-28cd-4f15-b635-7114007c1984",
    "https://mainnet.helius-rpc.com/?api-key=4e015027-1fb9-44d2-bef9-ec4a9d12a86a",
    "https://mainnet.helius-rpc.com/?api-key=ccaacca7-e250-485b-8fbc-de0a01e5c153",
    "https://mainnet.helius-rpc.com/?api-key=326c4c6f-8519-4c6f-9aa0-f50990b8abcd",
    "https://mainnet.helius-rpc.com/?api-key=f407de65-f8f4-4d22-9a26-ccaafc449580",
    "https://mainnet.helius-rpc.com/?api-key=fa67b96f-808b-4e44-8c4b-b71c070262d9",
    "https://mainnet.helius-rpc.com/?api-key=eb283e8f-3f7a-43d5-b131-bb2c6e4613c1",
    "https://mainnet.helius-rpc.com/?api-key=b04fce8b-54d7-4d0e-bdf4-2264c38687d4",
    "https://mainnet.helius-rpc.com/?api-key=4d78ee42-93f3-400d-8869-238d2585ec13",
    "https://mainnet.helius-rpc.com/?api-key=a5ce775d-943d-475f-b4fb-ff1fa54864f5",
    "https://mainnet.helius-rpc.com/?api-key=cd87ea58-576e-4d5e-a84b-b504291412d2",
    "https://mainnet.helius-rpc.com/?api-key=d47e6913-2ce5-46bf-a44d-749da853e19b",
    "https://mainnet.helius-rpc.com/?api-key=005277a0-a940-4edb-b47c-9f524cea4c5a",
    "https://mainnet.helius-rpc.com/?api-key=0fb3f3c7-73cd-49c5-b736-78120998b612",
    "https://mainnet.helius-rpc.com/?api-key=dcd8446a-0da0-4880-a546-2f4a2a058b4e",
    "https://mainnet.helius-rpc.com/?api-key=ce396a86-43fb-4958-804d-cb9adff04145",
    "https://mainnet.helius-rpc.com/?api-key=a652fc3f-871a-4a16-a010-2cf7e8acce81",
    "https://mainnet.helius-rpc.com/?api-key=6e2d0d54-2dbb-4d2b-807a-f7a4f2f3ba3d",
    "https://mainnet.helius-rpc.com/?api-key=ceb1db45-c8c3-4ecd-a771-7162aa79cc2f",
    "https://mainnet.helius-rpc.com/?api-key=bdb828e2-298a-40cf-bc8e-e507327e7222",
    "https://mainnet.helius-rpc.com/?api-key=410c4b29-d045-4d3a-9d3c-c804844dae6f",
    "https://mainnet.helius-rpc.com/?api-key=353c2955-cf12-4182-b51a-d0c6fd8c6b5c",
    "https://mainnet.helius-rpc.com/?api-key=6245fc88-7f7d-4421-b9a9-4d06f92e287d",
    "https://mainnet.helius-rpc.com/?api-key=a9f815dd-488c-489c-842f-9775af10e553",
    "https://mainnet.helius-rpc.com/?api-key=f7e5aca3-d017-40f3-914e-162f183b1191",
    "https://mainnet.helius-rpc.com/?api-key=38a3adca-ba6c-4c59-a5e6-c23063d7ef5e",
    "https://mainnet.helius-rpc.com/?api-key=f35b86ca-4735-4e21-a806-4617f30e6b06",
    "https://mainnet.helius-rpc.com/?api-key=a8b603c0-65ee-4d0c-bc95-ebe8808dd5cf",
    "https://mainnet.helius-rpc.com/?api-key=8f786a51-1ed3-413b-b213-60fa7ba31f63",
    "https://mainnet.helius-rpc.com/?api-key=605bb9f7-a5a5-496d-ad6b-19f3a0d062d1",
    "https://mainnet.helius-rpc.com/?api-key=4f7cdcbd-22f0-4668-9a63-c2e6579f39e1",
    "https://mainnet.helius-rpc.com/?api-key=5befb40f-2ec8-43cb-8a12-5e697e2406fd",
    "https://mainnet.helius-rpc.com/?api-key=f4f7b9e6-e4c8-49b5-99f7-6f31295b91b2",
    "https://mainnet.helius-rpc.com/?api-key=b82d5720-a2db-41d4-8ca0-0cb1d7665541",
    "https://mainnet.helius-rpc.com/?api-key=167fea26-2e78-420a-897e-51be77e410fa",
    "https://mainnet.helius-rpc.com/?api-key=01216afc-dff5-48a1-943a-fd788cc7d02e",
    "https://mainnet.helius-rpc.com/?api-key=dc0196b4-0621-4722-bc3c-6450e5a65b9b",
    "https://mainnet.helius-rpc.com/?api-key=1493088c-baac-4567-b12e-5df4d623b76e",
    "https://mainnet.helius-rpc.com/?api-key=af31ea18-8930-4632-91ba-9e76e8670e2f",
    "https://mainnet.helius-rpc.com/?api-key=4ab9e287-25b2-45e5-a6da-d77b3f83b66f",
    "https://mainnet.helius-rpc.com/?api-key=4e90f794-dd81-447f-b07c-57c6a685b6be",
    "https://mainnet.helius-rpc.com/?api-key=7fee19c4-0569-4ee4-9f6d-216276a02be5",
    "https://mainnet.helius-rpc.com/?api-key=217cc0ca-1bae-4e6d-9d8b-5000775e2da5",
    "https://mainnet.helius-rpc.com/?api-key=24ecbfd1-8662-4098-bd48-67b26ae60945",
    "https://mainnet.helius-rpc.com/?api-key=567f905c-9271-45b2-b461-30f8dc67c95f",
    "https://mainnet.helius-rpc.com/?api-key=64a368be-ba0f-4cdb-93bb-2c768ee8b2f3",
    "https://mainnet.helius-rpc.com/?api-key=62407405-1603-43ce-8987-e7a605995a7e",
    "https://mainnet.helius-rpc.com/?api-key=97faa0e1-ed4f-4d4e-b253-c3682dae599e",
    "https://mainnet.helius-rpc.com/?api-key=e2ca9672-09bd-4bda-8588-ec6755e9f4f5",
    "https://mainnet.helius-rpc.com/?api-key=213b9814-6a62-4e67-a007-5fdf285b89e1",
    "https://mainnet.helius-rpc.com/?api-key=f957324a-086f-465e-8210-a244dba33b2a",
    "https://mainnet.helius-rpc.com/?api-key=13f63523-4518-4238-bbc8-592dd1541504",
    "https://mainnet.helius-rpc.com/?api-key=f4a911b6-d2fe-4663-a775-72d09cdf1a05",
    "https://mainnet.helius-rpc.com/?api-key=32cf22f7-61bf-488f-b11d-2b7e5029b05f",
    "https://mainnet.helius-rpc.com/?api-key=a79f88ba-8848-4777-8234-71fc1d1a91cc",
    "https://mainnet.helius-rpc.com/?api-key=7485c170-ec74-4819-bf8c-c3bc619140de",
    "https://mainnet.helius-rpc.com/?api-key=79135585-10ee-46c1-b1dd-5a8f38244d42",
    "https://mainnet.helius-rpc.com/?api-key=dbf347fa-9229-4c14-96c0-7323a931e46e",
    "https://mainnet.helius-rpc.com/?api-key=a637807b-7099-4037-874b-d83348e441df",
    "https://mainnet.helius-rpc.com/?api-key=f4848266-ae6d-4e93-8181-5774f02bcde3",
    "https://mainnet.helius-rpc.com/?api-key=fb163931-fddb-46c4-8492-ae643d0425d2",
    "https://mainnet.helius-rpc.com/?api-key=6df59233-5815-47b0-abaa-947fe2f8c9f4",
    "https://mainnet.helius-rpc.com/?api-key=1e5b45a8-42e7-4d68-a750-b0c2eb56bd04",
    "https://mainnet.helius-rpc.com/?api-key=fb159973-5723-4110-ba8c-11a8772851f4",
    "https://mainnet.helius-rpc.com/?api-key=a65dc3af-6cfe-4e8c-9825-87dcbbd2abda",
    "https://mainnet.helius-rpc.com/?api-key=c795cf10-fa9a-4901-ad31-d8430df462ab",
    "https://mainnet.helius-rpc.com/?api-key=6cf0e077-909e-4c66-9009-a51db248c554",
    "https://mainnet.helius-rpc.com/?api-key=cbf55e23-6e32-4234-9a37-863e3db6fdb2",
    "https://mainnet.helius-rpc.com/?api-key=05da265c-beeb-4933-b5ee-3342bd86fc56",
    "https://mainnet.helius-rpc.com/?api-key=8f5778e8-c58c-40da-aa16-70152b585e86",
    "https://mainnet.helius-rpc.com/?api-key=490c07e6-eda5-4667-92bd-be64edfc9207",
    "https://mainnet.helius-rpc.com/?api-key=004f2629-fbd3-4461-856e-11d891cc25b3",
    "https://mainnet.helius-rpc.com/?api-key=436e315e-fc4a-4aa5-9ba8-3aadf97bb51f",
    "https://mainnet.helius-rpc.com/?api-key=7e2f4b0d-b720-4bf1-9155-98b239339de2",
    "https://mainnet.helius-rpc.com/?api-key=3eb26036-c5fe-4932-82fb-642220413a6c",
    "https://mainnet.helius-rpc.com/?api-key=821f003f-bc99-4b78-a4b9-b0bb50a6047e",
    "https://mainnet.helius-rpc.com/?api-key=81832c58-6b89-4be5-bf9e-c1a3762343c8",
    "https://mainnet.helius-rpc.com/?api-key=8d266a2e-3b08-49d7-b092-c1a437d59346",
    "https://mainnet.helius-rpc.com/?api-key=8c513ade-3d12-4950-be96-4681eef6d19c",
    "https://mainnet.helius-rpc.com/?api-key=ab51f5f6-db2f-4852-a3c4-9da0e20ef2d2",
    "https://mainnet.helius-rpc.com/?api-key=90f67b4f-de2e-43aa-9648-89dde88f1a04",
    "https://mainnet.helius-rpc.com/?api-key=b275c06a-ef95-4db3-b4be-609ca262e058",
    "https://mainnet.helius-rpc.com/?api-key=6111bee3-dc16-421a-901d-18a7182648ff",
    "https://mainnet.helius-rpc.com/?api-key=aac42d6d-71eb-47d5-8408-2a9173d3263e",
    "https://mainnet.helius-rpc.com/?api-key=3071e72c-2fea-47b7-ae61-c59e6d6ba8bc",
    "https://mainnet.helius-rpc.com/?api-key=3533ce7e-2e43-4aae-a383-e1e6df8f88cc",
    "https://mainnet.helius-rpc.com/?api-key=b7c221f1-0a6d-4f14-adb6-1cac85fc3f19",
    "https://mainnet.helius-rpc.com/?api-key=92baaf01-df50-4149-9625-e7ba3a5ffe7e",
    "https://mainnet.helius-rpc.com/?api-key=84d2cd69-6632-44ed-a487-0e4c16bd3447",
    "https://mainnet.helius-rpc.com/?api-key=7e07411c-b784-4472-9ed0-8a8da3bcca57",
    "https://mainnet.helius-rpc.com/?api-key=000f2639-73c8-4a1e-9db9-94cadbe4b0d2",
    "https://mainnet.helius-rpc.com/?api-key=bf302603-9f31-4e98-ab8c-065876794b8b",
    "https://mainnet.helius-rpc.com/?api-key=bc54d21b-dc79-4d0e-85e0-cc536c326442",
    "https://mainnet.helius-rpc.com/?api-key=5d04d5d0-2370-4c55-a8f1-31fbb5709dc8",
    "https://mainnet.helius-rpc.com/?api-key=3325416a-66ad-4e53-9345-803ff9e9be2a",
    "https://mainnet.helius-rpc.com/?api-key=7875b404-2f81-47de-846e-5137bc8f2b8c",
    "https://mainnet.helius-rpc.com/?api-key=24c1b7ef-05dd-4473-b9cf-ab7d15d67825",
    "https://mainnet.helius-rpc.com/?api-key=22e9b1b5-b013-4333-a0ea-abd709f11670",
    "https://mainnet.helius-rpc.com/?api-key=40a25bfe-80c0-4ba5-8337-5aaa59fab3e1",
    "https://mainnet.helius-rpc.com/?api-key=ec56d5e3-ed61-4221-a754-5218afb043d6",
    "https://mainnet.helius-rpc.com/?api-key=6d73e8f0-970f-48c8-b700-21507289c311",
    "https://mainnet.helius-rpc.com/?api-key=5b134783-8cd3-41e3-9a6a-a36d8e689203",
    "https://mainnet.helius-rpc.com/?api-key=95904abb-2c8f-4438-aabd-20cccb1567af",
    "https://mainnet.helius-rpc.com/?api-key=065175f8-4ad2-4dfc-9217-f4f7ec8baf34",
    "https://mainnet.helius-rpc.com/?api-key=688f8739-20f2-4527-b67a-b9b26594f8bd",
    "https://mainnet.helius-rpc.com/?api-key=08d920a8-e056-47c0-be41-fab731616d8a",
    "https://mainnet.helius-rpc.com/?api-key=c8a0a755-4393-440d-b6da-a7f0baead7b2",
    "https://mainnet.helius-rpc.com/?api-key=5b01c903-7c0a-463b-816e-af48bf300ce8",
    "https://mainnet.helius-rpc.com/?api-key=f8eed3f2-8589-45c1-a669-47d344ea20cf",
    "https://mainnet.helius-rpc.com/?api-key=58c2f2ce-c122-48a4-820a-2d2164308fa3",
    "https://mainnet.helius-rpc.com/?api-key=e98f11e9-0955-4f1b-a48e-d3e3deb1fe90",
    "https://mainnet.helius-rpc.com/?api-key=53b37143-4961-4c56-8aba-69f8f80bc21a",
    "https://mainnet.helius-rpc.com/?api-key=cdef5694-d7b8-4e8f-97ad-36d94ed818ac",
    "https://mainnet.helius-rpc.com/?api-key=bc86c635-8bac-4f24-ac64-ed8e3359b346",
    "https://mainnet.helius-rpc.com/?api-key=16384545-933b-4d51-95b8-25b4f708749b",
    "https://mainnet.helius-rpc.com/?api-key=9bfed113-aaf2-4c99-ae01-5b0cd611712f",
    "https://mainnet.helius-rpc.com/?api-key=8328f816-f736-4cae-bfe2-345ca346f311",
    "https://mainnet.helius-rpc.com/?api-key=7914678a-b206-4db6-89d1-dec0f9486e84",
    "https://mainnet.helius-rpc.com/?api-key=84481063-1eca-43af-98d5-74e344cbbda5",
    "https://mainnet.helius-rpc.com/?api-key=088aab95-4c32-4454-addb-29986a50e2a7",
    "https://mainnet.helius-rpc.com/?api-key=c2abfe3c-3e0e-4d19-8f40-0bd436f8d463",
    "https://mainnet.helius-rpc.com/?api-key=23ef6f2d-1f6b-41b2-b9ac-09cb190f8d89",
    "https://mainnet.helius-rpc.com/?api-key=ab9acee5-0b02-4f1a-99e1-b2d0be53b930",
    "https://mainnet.helius-rpc.com/?api-key=368c9d4c-5c0a-4fa2-a698-746157486950",
    "https://mainnet.helius-rpc.com/?api-key=9f0aa9ca-81df-492f-8e6a-de254703d12a",
    "https://mainnet.helius-rpc.com/?api-key=d4a34f17-9e1e-4f21-b6a9-a6f17b9c6625",
    "https://mainnet.helius-rpc.com/?api-key=a97a81bd-f387-436b-a90b-c8c6720dd5d9",
    "https://mainnet.helius-rpc.com/?api-key=1bd51c44-ffe9-4dc0-b9e2-59eae0c69659",
    "https://mainnet.helius-rpc.com/?api-key=28d9d19b-d7c2-431a-b438-533dd7e95153",
    "https://mainnet.helius-rpc.com/?api-key=fd7941fb-3233-4dfb-915d-9850f2434313",
    "https://mainnet.helius-rpc.com/?api-key=3d140567-5214-4cc5-b6ba-33172a7a42f1",
    "https://mainnet.helius-rpc.com/?api-key=025b6477-6834-4806-8ffc-4a9c0fd33ed7",
    "https://mainnet.helius-rpc.com/?api-key=adcd519c-be44-4c6f-bded-93d0419692da",
    "https://mainnet.helius-rpc.com/?api-key=68b25dd4-912a-4612-a0a2-a1e8ba5bb218",
    "https://mainnet.helius-rpc.com/?api-key=1a8eed9c-65ca-4312-9180-79dab2b80199",
    "https://mainnet.helius-rpc.com/?api-key=1c3fbf58-8a06-4e4f-af0e-bf48c34032f0",
    "https://mainnet.helius-rpc.com/?api-key=29e21acd-0146-43a1-bc85-06a24d4ed604",
    "https://mainnet.helius-rpc.com/?api-key=fb2c9f8c-4f16-442e-a886-76547cfdd3f7",
    "https://mainnet.helius-rpc.com/?api-key=f6bb0ef5-f835-48da-b86c-9d2312df3d1f",
    "https://mainnet.helius-rpc.com/?api-key=9da39367-3079-490d-9ba7-eb9143641db8",
    "https://mainnet.helius-rpc.com/?api-key=20b5be46-1f3f-4177-8c32-3e25b117d999",
    "https://mainnet.helius-rpc.com/?api-key=b0f9e2d4-639b-4d25-849c-e379bb249ed4",
    "https://mainnet.helius-rpc.com/?api-key=ad933300-fa95-4306-a7d2-c8240ff0e202",
    "https://mainnet.helius-rpc.com/?api-key=0a640082-6971-4a8d-aab6-6787a6336e14",
    "https://mainnet.helius-rpc.com/?api-key=35f1e0a7-e82c-4bd6-b16b-417e612195ba",
    "https://mainnet.helius-rpc.com/?api-key=434f8519-d7d6-4bad-9c0c-a60bf0ae2513",
    "https://mainnet.helius-rpc.com/?api-key=e5fe4f01-efb1-4574-8c56-bcb33522547a",
    "https://mainnet.helius-rpc.com/?api-key=74b8d778-2cb8-42a6-8914-e90e251ae6f2",
    "https://mainnet.helius-rpc.com/?api-key=a1f3d347-44e6-4d0c-aa3c-8825543147ab",
    "https://mainnet.helius-rpc.com/?api-key=0f117fa0-925c-4104-92eb-0fdfd72921ba",
    "https://mainnet.helius-rpc.com/?api-key=16928d8b-c896-49d3-9d1a-5309adb8b734",
    "https://mainnet.helius-rpc.com/?api-key=0e0bfa8e-f286-49df-8fc1-d9b2d0bda476",
    "https://mainnet.helius-rpc.com/?api-key=09809fe5-dd35-4af1-93fa-a261a2563de2",
    "https://mainnet.helius-rpc.com/?api-key=b3414597-6e0b-4fb7-a478-4151c5f5c2ff",
    "https://mainnet.helius-rpc.com/?api-key=6ac7f5af-84c1-4041-8ab3-5eab327ce17e",
    "https://mainnet.helius-rpc.com/?api-key=9c0575d4-8d51-4d5c-a4f6-9e5cd5c1cb06",
    "https://mainnet.helius-rpc.com/?api-key=89b5080d-06e5-4ad4-8808-b3274d584f52",
    "https://mainnet.helius-rpc.com/?api-key=6d678a74-e10a-4193-a501-2e384422ae12",
    "https://mainnet.helius-rpc.com/?api-key=61d35078-ff96-4ff2-b98a-42fe2f200f67",
    "https://mainnet.helius-rpc.com/?api-key=fb79b745-b4e3-4126-baa6-acd9481b3c05",
    "https://mainnet.helius-rpc.com/?api-key=9a043ac5-3ba2-4f62-98bb-8df7fc371dd8",
    "https://mainnet.helius-rpc.com/?api-key=01973ce7-6d2b-4c32-88ab-3aaa95c1f39b",
    "https://mainnet.helius-rpc.com/?api-key=a6a25205-2809-46d8-bca6-0c245d39a8f1",
    "https://mainnet.helius-rpc.com/?api-key=beb2e4e6-9f87-46cb-9d6b-2779283e0a4f",
    "https://mainnet.helius-rpc.com/?api-key=fb1ae8d8-a834-4453-a9f2-bce3963881f3",
    "https://mainnet.helius-rpc.com/?api-key=f8ec98aa-be62-487b-b117-9118b5a93f6f",
    "https://mainnet.helius-rpc.com/?api-key=7575c11a-a9e6-4823-9341-a6acc3d6828d",
    "https://mainnet.helius-rpc.com/?api-key=627ba253-7282-42e7-855b-fabd916d525f",
    "https://mainnet.helius-rpc.com/?api-key=e236ce44-53b9-4235-a85e-569eb99d4e9f",
    "https://mainnet.helius-rpc.com/?api-key=0d0da387-61b8-4eab-82e3-afda54e34cb6",
    "https://mainnet.helius-rpc.com/?api-key=96157876-e05c-49bc-bd41-a39f347a17d1",
    "https://mainnet.helius-rpc.com/?api-key=33c209f1-de9a-4404-a9c2-dd49ca79224c",
    "https://mainnet.helius-rpc.com/?api-key=08dd459e-221a-4b46-85eb-85531f745c99",
    "https://mainnet.helius-rpc.com/?api-key=7877242d-593d-43c5-bf25-f612e8fb2a60",
    "https://mainnet.helius-rpc.com/?api-key=7bb68ef6-d55c-4997-8a6a-da40361f35fb",
    "https://mainnet.helius-rpc.com/?api-key=466dfb69-25e9-4505-82b0-223829498844",
    "https://mainnet.helius-rpc.com/?api-key=ae4756a3-8b52-4713-b141-fa7ab91b3756",
    "https://mainnet.helius-rpc.com/?api-key=9d96ded3-eb55-4c5a-baaa-3e7479098026",
    "https://mainnet.helius-rpc.com/?api-key=680d5bc8-d4a7-4c8c-8da9-be15b6821787",
    "https://mainnet.helius-rpc.com/?api-key=de15dab8-83e2-4201-9d01-dc246c1bca33",
    "https://mainnet.helius-rpc.com/?api-key=15e79c97-ebb3-48c5-807e-a968e724544e",
    "https://mainnet.helius-rpc.com/?api-key=b5233b27-5e9a-493c-87b5-f761b6e65e5c",
    "https://mainnet.helius-rpc.com/?api-key=b8768265-374f-4a17-8056-e8eca8d32e9e",
    "https://mainnet.helius-rpc.com/?api-key=8028aaca-e92c-4c39-bb66-51f90a03b21a",
    "https://mainnet.helius-rpc.com/?api-key=7a782504-6265-4382-851c-0c31b907210b",
    "https://mainnet.helius-rpc.com/?api-key=7a622e47-0ab0-421e-a745-09300020f176",
    "https://mainnet.helius-rpc.com/?api-key=afba30df-9aea-4f6d-8f6d-8c6df5fcf7b6",
    "https://mainnet.helius-rpc.com/?api-key=99a5bc16-ffd6-451a-bf4c-668894e65676",
    "https://mainnet.helius-rpc.com/?api-key=f884a5bc-9d99-4b5f-b1d5-17137579177f",
    "https://mainnet.helius-rpc.com/?api-key=59c7dec4-bad4-4fb0-94f4-a1509755f0ba",
    "https://mainnet.helius-rpc.com/?api-key=b9b6d62d-42b8-4fde-bb3a-a8f646693e39",
    "https://mainnet.helius-rpc.com/?api-key=2d09e4a6-1dd9-4438-909c-e4a120c54b73",
    "https://mainnet.helius-rpc.com/?api-key=1073d2ad-c418-42d4-b4cc-1bb3ed55934e",
    "https://mainnet.helius-rpc.com/?api-key=9f0b273c-c78e-48c5-98d0-3845dd1e555a",
    "https://mainnet.helius-rpc.com/?api-key=4278e786-439e-4a98-9588-9c154f9fb092",
    "https://mainnet.helius-rpc.com/?api-key=7ecd6fdc-7bc1-4594-8457-621b1459dadc",
    "https://mainnet.helius-rpc.com/?api-key=573265a5-2d11-49e6-a4be-18a1c3a002f7",
    "https://mainnet.helius-rpc.com/?api-key=afef2c91-914f-4e36-90a4-d7fc71cc5705",
    "https://mainnet.helius-rpc.com/?api-key=f379ffff-bb04-4750-9f3b-20415eb77d94",
    "https://mainnet.helius-rpc.com/?api-key=3e4b0543-945f-40a3-a735-fcca20c85ea3",
    "https://mainnet.helius-rpc.com/?api-key=787b52e9-656f-4ba8-8847-acdae58ee6c4",
    "https://mainnet.helius-rpc.com/?api-key=bce3f314-91fa-47ab-a28a-5c1ad76598ce",
    "https://mainnet.helius-rpc.com/?api-key=723f89e4-7fda-4bed-934d-dee4c736a483",
    "https://mainnet.helius-rpc.com/?api-key=e701534b-d3d5-4221-bd0d-797fa381d6cd",
    "https://mainnet.helius-rpc.com/?api-key=075c6404-712a-4907-92dc-a378cd163100",
    "https://mainnet.helius-rpc.com/?api-key=583bdc71-2385-4839-8332-711a48ad489e",
    "https://mainnet.helius-rpc.com/?api-key=f49e36b6-068d-4fab-9d3b-58ae29ea3e4d",
    "https://mainnet.helius-rpc.com/?api-key=2e65c51b-bf67-412f-ae63-2e2f68f8fb88",
    "https://mainnet.helius-rpc.com/?api-key=35463ac5-15e9-4851-858d-3c2d0d8c68a6",
    "https://mainnet.helius-rpc.com/?api-key=b82cb56e-d769-4bd8-a4b1-c7476f0fe23d",
    "https://mainnet.helius-rpc.com/?api-key=6d0c1bec-a1fa-44da-af0d-e65b724fa02c",
    "https://mainnet.helius-rpc.com/?api-key=07ed81a3-297c-4410-834e-b257aa0f1c54",
    "https://mainnet.helius-rpc.com/?api-key=84d3d0f2-1f58-45ee-ad90-8a0d6a8bb0db",
    "https://mainnet.helius-rpc.com/?api-key=65241b52-b360-4db8-92d5-53d43986fbef",
    "https://mainnet.helius-rpc.com/?api-key=e1e5ceb2-8b3a-496f-9e72-895f3cf79bfe",
    "https://mainnet.helius-rpc.com/?api-key=4a3b82c4-b5bb-4a5b-96f1-d88ae4441f80",
    "https://mainnet.helius-rpc.com/?api-key=4aa8e01b-53b1-4c12-b51f-27dd28eaf884",
    "https://mainnet.helius-rpc.com/?api-key=d09a1a45-8f0e-4ba0-a370-4e2fe1055f3c",
    "https://mainnet.helius-rpc.com/?api-key=a40f9a3c-a655-4fd8-99cf-524aa6e65152",
    "https://mainnet.helius-rpc.com/?api-key=9b77d1e8-6339-42af-a922-5e19a57bfc45",
    "https://mainnet.helius-rpc.com/?api-key=73d2b8eb-91a0-4582-a1f2-194eeb614e46",
    "https://mainnet.helius-rpc.com/?api-key=e5c24ef0-45c3-45a4-a034-db51c8d0fbd4",
    "https://mainnet.helius-rpc.com/?api-key=a1964625-e273-40e2-a548-658beaf5c328",
    "https://mainnet.helius-rpc.com/?api-key=8a4a4782-d825-4887-b79c-8cb6aaf510b6",
    "https://mainnet.helius-rpc.com/?api-key=8db62b1e-e4d5-4e29-98b0-d966154d0e01",
    "https://mainnet.helius-rpc.com/?api-key=c8f953a9-8694-47c7-9e25-0c9893cfbb63",
    "https://mainnet.helius-rpc.com/?api-key=cafe9b76-aa15-47d8-b60e-e9a24d645bb1",
    "https://mainnet.helius-rpc.com/?api-key=f94e442d-e374-4776-8cfb-45ca21558741",
    "https://mainnet.helius-rpc.com/?api-key=e0a0b944-7400-416d-8ac4-90f2743d5447",
    "https://mainnet.helius-rpc.com/?api-key=ce63895f-6ec5-438b-a1af-4c4bef47da3c",
    "https://mainnet.helius-rpc.com/?api-key=ac586350-d1c9-4be0-a9a1-baddc724665c",
    "https://mainnet.helius-rpc.com/?api-key=37f846a0-a2a6-40a3-a744-5efa1a6ed73f",
    "https://mainnet.helius-rpc.com/?api-key=98fdcabd-29cd-46d6-bb70-0de5062c7c1a",
    "https://mainnet.helius-rpc.com/?api-key=315f6401-5170-4e8b-9ef1-b45514bfa667",
    "https://mainnet.helius-rpc.com/?api-key=ea0e6a1c-354f-4e1c-a400-0f244bee59df",
    "https://mainnet.helius-rpc.com/?api-key=a10bc382-a57d-4595-8424-93f51b73c386",
    "https://mainnet.helius-rpc.com/?api-key=42667646-0ca6-43ee-9ac8-6e4e89231137",
    "https://mainnet.helius-rpc.com/?api-key=39436d3a-5560-481f-8357-40d4906ac68e",
    "https://mainnet.helius-rpc.com/?api-key=d56f83cf-1bfc-472b-8b49-1dd8ff43ab91",
    "https://mainnet.helius-rpc.com/?api-key=02b9a9f2-9156-480c-b602-2ad6f1538142",
    "https://mainnet.helius-rpc.com/?api-key=393c39be-0894-4c28-ae25-c85dc8e4573c",
    "https://mainnet.helius-rpc.com/?api-key=79899a97-d0f9-4a62-bcf7-49218afde6ee",
    "https://mainnet.helius-rpc.com/?api-key=bc7a45ea-5bcc-4db9-94d5-f1eee53b34a9",
    "https://mainnet.helius-rpc.com/?api-key=104221e1-2af7-4f11-bbf9-da98130bd2c1",
    "https://mainnet.helius-rpc.com/?api-key=bd06af3f-35fe-4208-b14d-b82a40fc55aa",
    "https://mainnet.helius-rpc.com/?api-key=0a91b90a-8f6f-45c0-813b-8b8c4f3f8f2a",
    "https://mainnet.helius-rpc.com/?api-key=5294e79f-92c9-40b4-8588-a3b9426334e7",
    "https://mainnet.helius-rpc.com/?api-key=3daf4183-4b2a-4a51-b247-cf7f73f76b9e",
    "https://mainnet.helius-rpc.com/?api-key=1ce4225e-137a-4293-93bc-1fe48ac8210a",
    "https://mainnet.helius-rpc.com/?api-key=8b76b9c9-4bcf-49bb-9068-add20389437d",
    "https://mainnet.helius-rpc.com/?api-key=1381622d-9d62-4a21-92b7-c6a6c7b665c0",
    "https://mainnet.helius-rpc.com/?api-key=49f75bff-1a82-4cdb-90ea-ce6fdc7ce206",
    "https://mainnet.helius-rpc.com/?api-key=a58619dd-f96a-4e1c-99f9-f102b65ce618",
    "https://mainnet.helius-rpc.com/?api-key=071d817a-eeb8-40df-9a69-dd9ac65c0ac9",
    "https://mainnet.helius-rpc.com/?api-key=53523913-852d-4907-9ae8-8f5589240c72",
    "https://mainnet.helius-rpc.com/?api-key=87c83e9d-4931-44d2-b21d-91a41e78e4c7",
    "https://mainnet.helius-rpc.com/?api-key=0cc86d6b-ddc1-4c12-bb9f-a72ac3e9f29e",
    "https://mainnet.helius-rpc.com/?api-key=1f746325-29a1-4564-a885-d8b004f03512",
    "https://mainnet.helius-rpc.com/?api-key=5f431e6f-d356-4c8c-a73b-535ad89f5056",
    "https://mainnet.helius-rpc.com/?api-key=83ed25cb-cb9c-473b-b9dc-910297f0293f",
    "https://mainnet.helius-rpc.com/?api-key=8947f65f-1a35-4d23-b38b-cb3684c06193",
    "https://mainnet.helius-rpc.com/?api-key=41d3e4f2-4c6c-46aa-bc46-1b42dd6260d6",
    "https://mainnet.helius-rpc.com/?api-key=4c7aa984-7fdf-429a-85ee-c37927ca3da4",
    "https://mainnet.helius-rpc.com/?api-key=3fab0d68-9c6b-4c77-89e8-5f9cd33ca6e3",
    "https://mainnet.helius-rpc.com/?api-key=165365a6-a647-4471-96ff-87186c608b07",
    "https://mainnet.helius-rpc.com/?api-key=31291475-3fce-4710-9625-b370f8c0b1e5",
    "https://mainnet.helius-rpc.com/?api-key=5c98c2f3-c166-4c42-ac3d-ea24fb79f6f4",
    "https://mainnet.helius-rpc.com/?api-key=b8e133bd-cfd2-4b03-9e66-a1519f77bc5d",
    "https://mainnet.helius-rpc.com/?api-key=796d47d3-4420-44e5-b60e-ab49fe67ec64",
    "https://mainnet.helius-rpc.com/?api-key=6d98fbf0-e736-43df-9fc2-e44a0ea0eb10",
    "https://mainnet.helius-rpc.com/?api-key=f2fd6651-2d77-4c0f-b26e-1d98ada6a1b0",
    "https://mainnet.helius-rpc.com/?api-key=b3d67b9f-ea05-4ceb-b6a9-08409e5babd9",
    "https://mainnet.helius-rpc.com/?api-key=c53d751d-4bf0-40fc-a029-113359ee7374",
    "https://mainnet.helius-rpc.com/?api-key=a2c2b987-597c-4e95-8134-c31a3fd72ca2",
    "https://mainnet.helius-rpc.com/?api-key=8dcda10b-738b-4b97-8b8f-d09580cdd66e",
    "https://mainnet.helius-rpc.com/?api-key=25014e45-42a3-4983-a985-d2d56a527540",
    "https://mainnet.helius-rpc.com/?api-key=59019792-4fa3-4f5f-bd4a-ac504d0079cc",
    "https://mainnet.helius-rpc.com/?api-key=5421fdea-dce7-4b60-afd1-c2401f50f48e",
    "https://mainnet.helius-rpc.com/?api-key=d7ba244e-0f4f-408c-bfc2-04e8f943d9e1",
    "https://mainnet.helius-rpc.com/?api-key=497c60ed-ba0b-4efe-9b0b-3d59b922f870",
    "https://mainnet.helius-rpc.com/?api-key=c8d3d235-83e9-426e-9dc5-a458952895ad",
    "https://mainnet.helius-rpc.com/?api-key=3a81ff4a-95bd-4dea-937e-31bdf7a6f495",
    "https://mainnet.helius-rpc.com/?api-key=90a66bfd-a40a-428b-bed5-fde4f9118d31",
    "https://mainnet.helius-rpc.com/?api-key=4c46b620-a27f-4502-89df-8ab33fee4de0",
    "https://mainnet.helius-rpc.com/?api-key=92780969-32d2-432e-9cf3-b016e90b050f",
    "https://mainnet.helius-rpc.com/?api-key=2708ecd8-1033-4d6a-aba4-0dbd0faddee3",
    "https://mainnet.helius-rpc.com/?api-key=a17a84b5-2e9c-4c8d-9032-f4d9685fb24a",
    "https://mainnet.helius-rpc.com/?api-key=3cb4055b-a09c-4ca7-a5f9-369b872494de",
    "https://mainnet.helius-rpc.com/?api-key=44137ce1-ba47-406b-80ce-909d77759d4b",
    "https://mainnet.helius-rpc.com/?api-key=410a2863-452b-48dd-a3cd-df9581edadea",
    "https://mainnet.helius-rpc.com/?api-key=2c2a965e-4e47-40ff-8ddf-156221a7f618",
    "https://mainnet.helius-rpc.com/?api-key=4e446ce5-df82-497a-8812-1f0d30b47501",
    "https://mainnet.helius-rpc.com/?api-key=b52bedde-4bdd-4841-a23a-1e663b5abd5e",
    "https://mainnet.helius-rpc.com/?api-key=d66e5e0d-3c49-4b75-8ea0-80d16bf6bebb",
    "https://mainnet.helius-rpc.com/?api-key=d95f3bda-0d47-40f4-b1e9-e23e53933acb",
    "https://mainnet.helius-rpc.com/?api-key=ac7dcf6b-7ab8-47e2-9c7e-2fe02e10de29",
    "https://mainnet.helius-rpc.com/?api-key=88a35858-dce8-4e15-b171-6de8374ea586",
    "https://mainnet.helius-rpc.com/?api-key=ecddf5c5-f285-4f84-bea4-79386430fe29",
    "https://mainnet.helius-rpc.com/?api-key=5896d9bb-73bf-45aa-9ef9-f3846b73b58b",
    "https://mainnet.helius-rpc.com/?api-key=32522248-99dc-41a0-b5dd-46604f45e42d",
    "https://mainnet.helius-rpc.com/?api-key=c02c2df3-da72-45f9-86e9-db068f3eca40",
    "https://mainnet.helius-rpc.com/?api-key=c48360b7-5851-4554-870b-0e5f3aab13f5",
    "https://mainnet.helius-rpc.com/?api-key=5c4eee2c-8877-4b43-8e1b-5dd3cfd19dd3",
    "https://mainnet.helius-rpc.com/?api-key=78bd8a32-aa3d-4b5c-87c5-4d1838b3df0c",
    "https://mainnet.helius-rpc.com/?api-key=86e7c471-6403-4aa8-bd01-217503f325b4",
    "https://mainnet.helius-rpc.com/?api-key=72fba976-3e7c-4734-b150-c30edbdf50fe",
    "https://mainnet.helius-rpc.com/?api-key=868aa054-b394-453b-ae7b-d28c153eaffa",
    "https://mainnet.helius-rpc.com/?api-key=c3f45b51-6021-4496-aedd-061fd04b0039",
    "https://mainnet.helius-rpc.com/?api-key=a79cabd2-517c-49b0-b602-b7a2f1ba1c1b",
    "https://mainnet.helius-rpc.com/?api-key=21f03c7e-7370-4704-8963-8cb0e4022121",
    "https://mainnet.helius-rpc.com/?api-key=4a4ccb4a-661b-45ec-8490-fa8002c549fb",
    "https://mainnet.helius-rpc.com/?api-key=bb34268c-a3a2-4e1e-bb29-073f266e9c6e",
    "https://mainnet.helius-rpc.com/?api-key=fa1a8660-cabf-4471-811e-0589751eb5e2",
    "https://mainnet.helius-rpc.com/?api-key=fcd91f8c-7a3a-490c-b69b-faa18999f729",
    "https://mainnet.helius-rpc.com/?api-key=903b544e-554b-401e-b9a9-860b1df1e4bf",
    "https://mainnet.helius-rpc.com/?api-key=bb7d28b5-6567-4ea0-957b-a0022c8e12c8",
    "https://mainnet.helius-rpc.com/?api-key=33649d4c-00df-4db6-8609-31008c921113",
    "https://mainnet.helius-rpc.com/?api-key=f9cd3e87-afe3-4242-89cd-4af328733419",
    "https://mainnet.helius-rpc.com/?api-key=458f4c07-3a53-419f-9f18-6c74ed990076",
    "https://mainnet.helius-rpc.com/?api-key=5fe13db4-03ea-49ef-b8c6-2235c81a16c1",
    "https://mainnet.helius-rpc.com/?api-key=080e838d-a0d9-492c-a016-f3f3f92fc888",
    "https://mainnet.helius-rpc.com/?api-key=ffa39cfe-b4b8-4825-9c9f-b95abca29750",
    "https://mainnet.helius-rpc.com/?api-key=9765321f-4956-4222-bc84-c17b2b0d2be4",
    "https://mainnet.helius-rpc.com/?api-key=c428c5b6-bb03-4bd7-8509-026ce24a383b",
    "https://mainnet.helius-rpc.com/?api-key=85ef6189-c961-4afe-b804-541be1040a7e",
    "https://mainnet.helius-rpc.com/?api-key=9d73fd60-abd2-4cb1-ac7f-3aba09177287",
    "https://mainnet.helius-rpc.com/?api-key=bfb1a69f-1fe3-46b3-a9d8-4e1f3594030e",
    "https://mainnet.helius-rpc.com/?api-key=8236bd35-3e31-435c-a2a2-45f07901afa7",
    "https://mainnet.helius-rpc.com/?api-key=42da36e4-1732-43f8-9bdf-0547ffbb41a2",
    "https://mainnet.helius-rpc.com/?api-key=e53aba7a-cfcd-4b7f-a5be-018aabbc3344",
    "https://mainnet.helius-rpc.com/?api-key=966566a4-0849-4ecb-88fb-f410a2e3fd8d",
    "https://mainnet.helius-rpc.com/?api-key=b03a6c8b-e09e-420a-bf00-211c074e9701",
    "https://mainnet.helius-rpc.com/?api-key=6706ba9d-1223-4d25-b067-e29769db8da7",
    "https://mainnet.helius-rpc.com/?api-key=6fd3f7e1-fc25-4b6f-95f7-fa87cae5b2e5",
    "https://mainnet.helius-rpc.com/?api-key=75fff1b2-d472-4beb-8de0-1f765e4431cc",
    "https://mainnet.helius-rpc.com/?api-key=80a47052-eb11-4cb3-9a93-42be9e080fbf",
    "https://mainnet.helius-rpc.com/?api-key=5477f320-4642-4b00-9023-5022876888ba",
    "https://mainnet.helius-rpc.com/?api-key=8444c24f-d771-4486-96ed-ccd91459bd54",
    "https://mainnet.helius-rpc.com/?api-key=b8a0795a-1df0-4656-8f7c-23634132fd1b",
    "https://mainnet.helius-rpc.com/?api-key=a43315f4-6baf-4e0b-b8ce-538cd807b228",
    "https://mainnet.helius-rpc.com/?api-key=a9d2ada2-26cb-44e0-b0df-ba049610301e",
    "https://mainnet.helius-rpc.com/?api-key=7c5d03e2-ac9a-4cbb-9f7e-7657d1211301",
    "https://mainnet.helius-rpc.com/?api-key=e2ef9f78-d0da-4814-a3f5-c665d451d135",
    "https://mainnet.helius-rpc.com/?api-key=63d0bfbd-eeb3-48de-a710-1331edcec56b",
    "https://mainnet.helius-rpc.com/?api-key=71fd6ac3-b0d2-44a6-885a-ab65b42fd4e3",
    "https://mainnet.helius-rpc.com/?api-key=2e6e0acc-288b-442b-a62e-2f2b73fb74ab",
    "https://mainnet.helius-rpc.com/?api-key=7e1789a5-ec36-41f7-a893-e24796eb97ce",
    "https://mainnet.helius-rpc.com/?api-key=bc54da85-afcb-425c-9ee7-e6cc8759555d",
    "https://mainnet.helius-rpc.com/?api-key=3bc33e63-f730-48ea-ae2a-fe61a590a6b5",
    "https://mainnet.helius-rpc.com/?api-key=158e9420-fc43-4181-a60a-68cf44373a41",
    "https://mainnet.helius-rpc.com/?api-key=00b620b5-4ec5-49a2-a965-97419817cca3",
    "https://mainnet.helius-rpc.com/?api-key=c51929a5-291c-4523-8c80-c6a8ed6378e8",
    "https://mainnet.helius-rpc.com/?api-key=37705ab1-4252-4686-9f41-5b347a314dc1",
    "https://mainnet.helius-rpc.com/?api-key=2787a436-68c2-4fb7-b6d3-442ef066e9a7",
    "https://mainnet.helius-rpc.com/?api-key=6e5c5f1b-c119-4eb2-9296-577623f26a26",
    "https://mainnet.helius-rpc.com/?api-key=e2b0463c-94c7-40ff-8130-c7a5dd5c97d2",
    "https://mainnet.helius-rpc.com/?api-key=bae8253e-6208-442a-a3a9-968129815e8f",
    "https://mainnet.helius-rpc.com/?api-key=2dfe6c9b-c0a6-4d02-9d29-828d6fbd8981",
    "https://mainnet.helius-rpc.com/?api-key=71a1243b-4ef0-4242-8ea1-16fe06c7654e",
    "https://mainnet.helius-rpc.com/?api-key=2c94a129-cec9-47c4-8fa2-e7b74116fe42",
    "https://mainnet.helius-rpc.com/?api-key=ec5fdb55-9aef-46f4-8b0c-a7374e7bf556",
    "https://mainnet.helius-rpc.com/?api-key=6cb20f97-8559-4129-9bdb-e445772f865a",
    "https://mainnet.helius-rpc.com/?api-key=8ba720e7-4543-4371-bff5-1dc78c8430df",
    "https://mainnet.helius-rpc.com/?api-key=eb7d5272-39f6-4f08-b3c3-a513f010b7ba",
    "https://mainnet.helius-rpc.com/?api-key=83b5ec48-cb73-49dd-8168-c835c58071ea",
    "https://mainnet.helius-rpc.com/?api-key=c5feed9f-dc4c-45f6-aca7-1b346f0a6b54",
    "https://mainnet.helius-rpc.com/?api-key=6980f9f6-2fb3-4b81-b620-dbba18f876ad",
    "https://mainnet.helius-rpc.com/?api-key=2ac966d8-e20d-43b5-a2aa-4934308e0b76",
    "https://mainnet.helius-rpc.com/?api-key=62211948-0678-4000-9db2-56aaef1b482e",
    "https://mainnet.helius-rpc.com/?api-key=462499ad-119a-4e4e-9530-c720a5336e6e",
    "https://mainnet.helius-rpc.com/?api-key=bac0150b-408e-44d3-a740-a6fb369bd5f2",
    "https://mainnet.helius-rpc.com/?api-key=b86de01c-ab51-46a2-aaf8-f95230bd5939",
    "https://mainnet.helius-rpc.com/?api-key=08c4be25-fbe0-49a9-9360-15e886e393de",
    "https://mainnet.helius-rpc.com/?api-key=57e8e427-d47e-45ae-8e1a-ea8e63e25935",
    "https://mainnet.helius-rpc.com/?api-key=3b47c5e8-dab6-4cb5-b67e-a356054a7e43",
    "https://mainnet.helius-rpc.com/?api-key=ca1f792f-b32e-492a-aa7c-2803d8307ea8",
    "https://mainnet.helius-rpc.com/?api-key=1a440f6d-3c7c-4032-afe9-e184848e8d2d",
    "https://mainnet.helius-rpc.com/?api-key=d03640ee-92f4-43c1-ae24-c3e44e1287e9",
    "https://mainnet.helius-rpc.com/?api-key=5db92d1c-d4bf-4de7-804f-61888b36f48d",
    "https://mainnet.helius-rpc.com/?api-key=4cca52e6-2908-4abb-8901-6d788ec48aa7",
    "https://mainnet.helius-rpc.com/?api-key=2c359cc8-c49c-46e7-a77d-bde6935a2ede",
    "https://mainnet.helius-rpc.com/?api-key=13f677ae-4643-4e60-870e-8e26a33901b3",
    "https://mainnet.helius-rpc.com/?api-key=29d74abd-ff15-45da-a564-8609a8333794",
    "https://mainnet.helius-rpc.com/?api-key=2c07afd7-3b56-4e40-9896-c97bf0e46592",
    "https://mainnet.helius-rpc.com/?api-key=9dd56499-5874-4273-beb5-9a3e677acbd5",
    "https://mainnet.helius-rpc.com/?api-key=1ab8ac52-f4f3-4f21-979b-8bf78551affd",
    "https://mainnet.helius-rpc.com/?api-key=5ebe4270-6d4f-4857-bad7-622766bc5abb",
    "https://mainnet.helius-rpc.com/?api-key=18e4b2ae-4ae5-4899-b442-dc4356872df6",
    "https://mainnet.helius-rpc.com/?api-key=c79625bd-adf1-4781-bf8a-4c2ec32777be",
    "https://mainnet.helius-rpc.com/?api-key=f3351ab5-e6c5-4162-988b-fdcb73d602a1",
    "https://mainnet.helius-rpc.com/?api-key=543a09f7-228a-4f63-8423-fe5f88b910b6",
    "https://mainnet.helius-rpc.com/?api-key=225fb719-967c-489e-a2fd-ec7b44f3c26d",
    "https://mainnet.helius-rpc.com/?api-key=9a7824ef-b304-4304-97de-d5465d51a06c",
    "https://mainnet.helius-rpc.com/?api-key=22e8ff9f-1037-4849-a9ee-cfd460cab511",
    "https://mainnet.helius-rpc.com/?api-key=cd08450e-c5ff-407a-b88f-d14e5b580a56",
    "https://mainnet.helius-rpc.com/?api-key=ab6c6418-ea56-432d-9dd8-ffa50704594d",
    "https://mainnet.helius-rpc.com/?api-key=fa068456-5c4d-44ae-89db-1e3da4e4192e",
    "https://mainnet.helius-rpc.com/?api-key=4bdcfd91-edb7-447f-a8d8-73308ab0acb2",
    "https://mainnet.helius-rpc.com/?api-key=3894636d-2e96-4289-9464-1291c29ebd0a",
    "https://mainnet.helius-rpc.com/?api-key=178dbb93-146c-4bb1-b1e6-f1bff0f7072f",
    "https://mainnet.helius-rpc.com/?api-key=e8f8642d-140b-4e05-add4-2bfaa00e4cc2",
    "https://mainnet.helius-rpc.com/?api-key=0d18134e-afc4-4f67-a804-2952c8f23566",
    "https://mainnet.helius-rpc.com/?api-key=e3de9d96-6e08-49ab-8ab1-2ef3c1bb2412",
    "https://mainnet.helius-rpc.com/?api-key=5e376bd6-fbb2-4d9c-8bdf-a3631e708a56",
    "https://mainnet.helius-rpc.com/?api-key=9849f71b-0d8f-4ebe-84ae-ac0b478f0e9c",
    "https://mainnet.helius-rpc.com/?api-key=7a7ab50e-676c-423f-bb99-ded25cc935b1",
    "https://mainnet.helius-rpc.com/?api-key=11eeb204-b59d-44fb-a198-284b75d569e9",
    "https://mainnet.helius-rpc.com/?api-key=093ecc5b-b80c-41a5-a61c-3b5e9716cdfb",
    "https://mainnet.helius-rpc.com/?api-key=348e9551-4a8e-4b25-8f3f-c36873371291",
    "https://mainnet.helius-rpc.com/?api-key=a0df130c-da22-4f33-8a3c-36491bab185e",
    "https://mainnet.helius-rpc.com/?api-key=b5b3062e-e4bf-4de9-8939-7a5d890a7812",
    "https://mainnet.helius-rpc.com/?api-key=ebb72696-c363-4225-bcda-89bba350125f",
    "https://mainnet.helius-rpc.com/?api-key=f6c563b4-d743-4ce9-b502-27a39fa10bcf",
    "https://mainnet.helius-rpc.com/?api-key=a7e80d79-ee1b-409f-8aac-6eff777be5da",
    "https://mainnet.helius-rpc.com/?api-key=9256b277-b106-4099-bbe5-650814906b40",
    "https://mainnet.helius-rpc.com/?api-key=48adb9d2-b069-4787-b81c-4ca1bfa285af",
    "https://mainnet.helius-rpc.com/?api-key=81df4aee-0fec-46e6-a6d8-15c160715991",
    "https://mainnet.helius-rpc.com/?api-key=68c7ae7e-b97d-4e0b-b6b5-f719ffa9e8ba",
    "https://mainnet.helius-rpc.com/?api-key=33a97e87-a60f-4712-9490-876bae0f276c",
    "https://mainnet.helius-rpc.com/?api-key=e5a992f6-2128-4f75-a080-ce13db8add68",
    "https://mainnet.helius-rpc.com/?api-key=84a335dd-a638-47e5-804e-f5b0c0177d5c",
    "https://mainnet.helius-rpc.com/?api-key=b5decaaf-3c57-4cc1-b56c-4ecdf66ffdd2",
    "https://mainnet.helius-rpc.com/?api-key=363bebe6-c48b-4c2e-8599-ce75c476859e",
    "https://mainnet.helius-rpc.com/?api-key=011cc9af-f93c-4960-9e8b-429c4ee51564",
    "https://mainnet.helius-rpc.com/?api-key=57cd2186-6872-4804-b081-d860949a559b",
    "https://mainnet.helius-rpc.com/?api-key=518d1abd-b842-4b6f-a4aa-4bb977a3e6ac",
    "https://mainnet.helius-rpc.com/?api-key=2b509f2e-177d-44c9-9a6b-e2888993c129",
    "https://mainnet.helius-rpc.com/?api-key=94abd8e4-775c-4b0b-afed-0718703bd350",
    "https://mainnet.helius-rpc.com/?api-key=9b3fc0e0-a54c-4005-9549-3272e2fe6cba",
    "https://mainnet.helius-rpc.com/?api-key=a47d116f-23ce-4cda-a2c3-90caba20c249",
    "https://mainnet.helius-rpc.com/?api-key=17bd7582-3e63-475e-a8df-484e71221217",
    "https://mainnet.helius-rpc.com/?api-key=a5792a5d-63f4-4684-8fda-3468719cbcd1",
    "https://mainnet.helius-rpc.com/?api-key=f1585b4e-cd41-458d-be55-69bbc2f5ca62",
    "https://mainnet.helius-rpc.com/?api-key=8d33c811-5f53-456a-b098-50ba8e71ecef",
    "https://mainnet.helius-rpc.com/?api-key=ac12dcff-204d-4b27-8b98-15e13df359d8",
    "https://mainnet.helius-rpc.com/?api-key=8e626bfe-8aa9-416b-a92e-aa493579558b",
    "https://mainnet.helius-rpc.com/?api-key=df4d258c-41e0-4a52-8649-ede536014aa6",
    "https://mainnet.helius-rpc.com/?api-key=f43f1523-370e-4b09-a022-ce65bea9f3a0",
    "https://mainnet.helius-rpc.com/?api-key=d7f1165b-b954-4250-a84c-3755b7a1dee2",
    "https://mainnet.helius-rpc.com/?api-key=a44474df-c828-4ae1-8295-f263220f043d",
    "https://mainnet.helius-rpc.com/?api-key=0eae65ad-526c-4622-b0a5-607529c19520",
    "https://mainnet.helius-rpc.com/?api-key=37a07bc5-0fdf-4eed-a6b8-fb980ade2007",
    "https://mainnet.helius-rpc.com/?api-key=07f426ea-612f-4112-a047-eec3aa525d07",
    "https://mainnet.helius-rpc.com/?api-key=83adfe2d-47b3-41c4-a3ce-03e8ed1ecb97",
    "https://mainnet.helius-rpc.com/?api-key=14eca26c-b777-4d59-8243-936759cbbed0",
    "https://mainnet.helius-rpc.com/?api-key=64ed06e3-8cd4-4637-a1bd-80a9b713a25f",
    "https://mainnet.helius-rpc.com/?api-key=d5074366-a9f8-432d-a7a8-b26817dc1992",
    "https://mainnet.helius-rpc.com/?api-key=cf9861b5-8f62-4451-87ad-b32da56f1fbe",
    "https://mainnet.helius-rpc.com/?api-key=a945ea7b-0412-425a-98fe-24de0428e5d4",
    "https://mainnet.helius-rpc.com/?api-key=e4d7cd01-4921-479d-80fe-99556eed883b",
    "https://mainnet.helius-rpc.com/?api-key=e0773ebf-28bd-4d07-9240-ee0b85e661d4",
    "https://mainnet.helius-rpc.com/?api-key=11b327d6-b770-42c9-9351-ea894c4f70c2",
    "https://mainnet.helius-rpc.com/?api-key=3acb9ff4-0c14-4484-b443-17824e246d77",
    "https://mainnet.helius-rpc.com/?api-key=ab2ef346-be1a-41c6-90f0-bdf4d7413a23",
    "https://mainnet.helius-rpc.com/?api-key=c88b758b-e743-4dac-8708-002e8d8433ec",
    "https://mainnet.helius-rpc.com/?api-key=abfb24ab-4f5d-4a94-9665-5b5e114feafe",
    "https://mainnet.helius-rpc.com/?api-key=1e32ee4b-d9c8-463e-a9db-9e229e6900a2",
    "https://mainnet.helius-rpc.com/?api-key=b9022e35-4b2a-4599-b55e-cb7b9e9a9799",
    "https://mainnet.helius-rpc.com/?api-key=0d490fde-7ac7-4289-ab4e-86541e99857c",
    "https://mainnet.helius-rpc.com/?api-key=c309c52e-61e5-486d-899a-f4263f8aaa64",
    "https://mainnet.helius-rpc.com/?api-key=21659f02-4cca-432d-8b54-40961e94831f",
    "https://mainnet.helius-rpc.com/?api-key=3a68d80b-ef65-4ca2-84f2-2f12fc35450c",
    "https://mainnet.helius-rpc.com/?api-key=71965970-9ef6-4fa7-96c8-42db1549ac22",
    "https://mainnet.helius-rpc.com/?api-key=a7f7595e-0212-4c94-b5b4-1c63417ead53",
    "https://mainnet.helius-rpc.com/?api-key=ad2e47fb-d69e-4222-83cb-b2f61245db5f",
    "https://mainnet.helius-rpc.com/?api-key=8a7bbbe1-27cd-4291-948c-4c828009f2e7",
    "https://mainnet.helius-rpc.com/?api-key=34dcf853-8a3e-4ff4-8e74-8ab7475c3d5b",
    "https://mainnet.helius-rpc.com/?api-key=d60d3833-14f7-4809-b271-f8a74d58b968",
    "https://mainnet.helius-rpc.com/?api-key=e0cfbdd2-f637-42d5-b06f-3e7a697d5114",
    "https://mainnet.helius-rpc.com/?api-key=2444f22b-d8c8-4b02-bbc0-826456bddf64",
    "https://mainnet.helius-rpc.com/?api-key=49f9cf67-53a8-49ed-af4b-1634971cd413",
    "https://mainnet.helius-rpc.com/?api-key=fd2618ad-5ead-4d91-b5a6-fbe8832ff861",
    "https://mainnet.helius-rpc.com/?api-key=abf14e94-0756-47b5-bd90-3a3938ed4aa6",
    "https://mainnet.helius-rpc.com/?api-key=a368bc07-05e6-4bfa-8c70-c85a72db8195",
    "https://mainnet.helius-rpc.com/?api-key=9ccfde8c-b98a-4a35-b782-c445474f8e0f",
    "https://mainnet.helius-rpc.com/?api-key=fd137f24-e98a-4ca0-ad14-60a4f9725ea9",
    "https://mainnet.helius-rpc.com/?api-key=3ce2f31d-03a3-4b11-ae2c-68ebdb78ac65",
    "https://mainnet.helius-rpc.com/?api-key=d8d56920-1017-4d5b-9d12-870264477f52",
    "https://mainnet.helius-rpc.com/?api-key=8104a4c0-1da3-4a1c-94e5-ccc71ab77b30",
    "https://mainnet.helius-rpc.com/?api-key=b0839344-8284-480c-91c3-3e3544ecf1a1",
    "https://mainnet.helius-rpc.com/?api-key=23110253-5434-4cb0-9411-26d9f3b2a52d",
    "https://mainnet.helius-rpc.com/?api-key=7d746971-6ca5-4269-b85f-c1d508df167b",
    "https://mainnet.helius-rpc.com/?api-key=0bf921a8-42d6-4c21-89d8-5700bd2f654a",
    "https://mainnet.helius-rpc.com/?api-key=549952fb-3363-4466-a519-f6cad077eb98",
    "https://mainnet.helius-rpc.com/?api-key=66601c85-0f22-4af6-aac3-dfde3aab7673",
    "https://mainnet.helius-rpc.com/?api-key=618cf758-d9fc-4b0f-8637-823954aeacb6",
    "https://mainnet.helius-rpc.com/?api-key=c628c0b3-8562-4411-97fc-c6723df2d16a",
    "https://mainnet.helius-rpc.com/?api-key=5c178dd2-2d39-40ac-87fe-a3e39ad06a18",
    "https://mainnet.helius-rpc.com/?api-key=0f614869-f5b4-4c08-b170-6e02c56efdf6",
    "https://mainnet.helius-rpc.com/?api-key=b90867f4-3cf7-45bf-a020-f0d250b3f82e",
    "https://mainnet.helius-rpc.com/?api-key=ca612ac3-80bb-4887-8e17-374d87e346a6",
    "https://mainnet.helius-rpc.com/?api-key=bb8d16ba-d21b-4d04-b351-4756c081eeda",
    "https://mainnet.helius-rpc.com/?api-key=935bc1d2-801e-429c-8b4b-52fefae94162",
    "https://mainnet.helius-rpc.com/?api-key=621cdd38-5226-4029-9212-c2e32b1940ea",
    "https://mainnet.helius-rpc.com/?api-key=93594d29-7337-4d38-bb89-4d5af1d80783",
    "https://mainnet.helius-rpc.com/?api-key=a65095a5-58e7-4f9c-b6a9-a9a2d6c0acc6",
    "https://mainnet.helius-rpc.com/?api-key=a1139d44-925e-4ec2-8e1e-723c89a3963a",
    "https://mainnet.helius-rpc.com/?api-key=8278ddf7-9c16-4cbb-88c5-038e744ec5c0",
    "https://mainnet.helius-rpc.com/?api-key=b1c0af6e-744a-4dcf-8e84-359e667938ff",
    "https://mainnet.helius-rpc.com/?api-key=a10c4a86-bb0b-4e77-8517-bc4c8915b392",
    "https://mainnet.helius-rpc.com/?api-key=472da66a-bb55-40c5-a3da-7a2ccc7c8d89",
    "https://mainnet.helius-rpc.com/?api-key=4770efeb-54d3-4fee-b42e-c8ae6bfd3764",
    "https://mainnet.helius-rpc.com/?api-key=e0e880f0-2dad-4066-a9d1-20cad86a322d",
    "https://mainnet.helius-rpc.com/?api-key=7b56791b-d8e5-4084-ba3a-f709c10dfd7a",
    "https://mainnet.helius-rpc.com/?api-key=49447cc1-8b3f-4b7b-b572-5aa869d6755a",
    "https://mainnet.helius-rpc.com/?api-key=68cec17a-18b9-435b-ab4f-0380995d16bb",
    "https://mainnet.helius-rpc.com/?api-key=6e4ec45b-b2b6-4341-8d72-822d6764e1eb",
    "https://mainnet.helius-rpc.com/?api-key=a510c49a-0a08-40ca-bb70-d875f7bf387d",
    "https://mainnet.helius-rpc.com/?api-key=8e098d8d-d69e-455c-b012-7cc24ff6f0dd",
    "https://mainnet.helius-rpc.com/?api-key=45228438-d995-45c8-818f-80293c968555",
    "https://mainnet.helius-rpc.com/?api-key=351b0501-ec92-4c06-8238-1bac8d66259c",
    "https://mainnet.helius-rpc.com/?api-key=6a00f446-8b3e-44fc-9aa9-a83f6c4fd7d3",
    "https://mainnet.helius-rpc.com/?api-key=5af963d3-b0e3-4596-95e6-2e22f7bc03c1",
    "https://mainnet.helius-rpc.com/?api-key=3a37fdd6-6991-4f56-b6a5-c90f06d4e839",
    "https://mainnet.helius-rpc.com/?api-key=8c3a65eb-060d-40f1-ba62-8ce72168b3dc",
    "https://mainnet.helius-rpc.com/?api-key=bf7d0e7d-d919-4b06-ba1f-3636566981a0",
    "https://mainnet.helius-rpc.com/?api-key=66d9fa36-fa52-4c74-924e-a54c6dcd9788",
    "https://mainnet.helius-rpc.com/?api-key=96900924-d271-4b30-adf5-a98068be1547",
    "https://mainnet.helius-rpc.com/?api-key=aa9a1dc5-e357-4980-bfc2-dc6291a7df85",
    "https://mainnet.helius-rpc.com/?api-key=94ec864c-5a9f-47d2-803b-23be7277095c",
    "https://mainnet.helius-rpc.com/?api-key=a2e5c7cb-665e-479a-bbf2-f60d6e4a2b3b",
    "https://mainnet.helius-rpc.com/?api-key=a05b3c8f-5cef-4c85-8f03-c06ac9630ff3",
    "https://mainnet.helius-rpc.com/?api-key=8e1d7ff9-a147-478e-901f-762b352d0aad",
    "https://mainnet.helius-rpc.com/?api-key=6411affe-f5fe-458e-87dd-c0efc66be089",
    "https://mainnet.helius-rpc.com/?api-key=1cf762b3-c753-435d-9198-8cf921869f10",
    "https://mainnet.helius-rpc.com/?api-key=3d95b1aa-e121-46d7-bcca-dea59d00cf99",
    "https://mainnet.helius-rpc.com/?api-key=7ffa13f7-5e3d-4904-9e7e-7e74dce211e5",
    "https://mainnet.helius-rpc.com/?api-key=25567ed4-f813-4e9d-99a8-363cf93ba5e5",
    "https://mainnet.helius-rpc.com/?api-key=01fa322d-28eb-43cf-a2cc-c9392b29739a",
    "https://mainnet.helius-rpc.com/?api-key=5c9f34ce-aaa9-4ce3-be57-0c4e90de6992",
    "https://mainnet.helius-rpc.com/?api-key=22e67cef-8287-4563-b123-d385ef06a6db",
    "https://mainnet.helius-rpc.com/?api-key=a04c0864-8086-4ded-a5ff-966b78e1de0c",
    "https://mainnet.helius-rpc.com/?api-key=ddef6eaa-1ed2-456f-b72f-f9f6faa618db",
    "https://mainnet.helius-rpc.com/?api-key=85c48d58-c04e-4636-b054-e784bc6590bc",
    "https://mainnet.helius-rpc.com/?api-key=e83b6999-2179-4b76-9888-ab4f2a523f0b",
    "https://mainnet.helius-rpc.com/?api-key=6784abee-377f-40ab-9fdb-4d512315e1d9",
    "https://mainnet.helius-rpc.com/?api-key=bd3b47c6-003c-4d11-b706-0937d0918464",
    "https://mainnet.helius-rpc.com/?api-key=e1f57500-d30e-49be-8f21-d18e6514c945",
    "https://mainnet.helius-rpc.com/?api-key=de0604df-3d91-4ba1-bdb7-0e3f1af5351c",
    "https://mainnet.helius-rpc.com/?api-key=3dd15c80-c8e6-4ac3-ac3e-3133c6389a25",
    "https://mainnet.helius-rpc.com/?api-key=20909d12-1f8e-4967-9538-e526cb6a1db9",
    "https://mainnet.helius-rpc.com/?api-key=e80ea2e5-b715-4081-b869-56505afa7d92",
    "https://mainnet.helius-rpc.com/?api-key=c90580a6-6160-4732-8f31-9322b9cbe0bb",
    "https://mainnet.helius-rpc.com/?api-key=1553ff82-7484-4cda-9d01-15eed682f6bd",
    "https://mainnet.helius-rpc.com/?api-key=bae233e2-28e3-4437-a2c4-194d16c6cbc1",
    "https://mainnet.helius-rpc.com/?api-key=18bb548c-c705-4ea3-ace1-79ac5dd67faa",
    "https://mainnet.helius-rpc.com/?api-key=ff169c15-000a-4f66-a555-2b20058921b0",
    "https://mainnet.helius-rpc.com/?api-key=897ae99b-e20d-431f-b8ca-364e77347841",
    "https://mainnet.helius-rpc.com/?api-key=40bc9092-bb6d-463e-9893-282c7dfbb51e",
    "https://mainnet.helius-rpc.com/?api-key=e40ba640-b9f6-43cc-94ca-56c86c4ba255",
    "https://mainnet.helius-rpc.com/?api-key=a80c7924-ddb8-4708-94ca-341420392510",
    "https://mainnet.helius-rpc.com/?api-key=c0e28352-8222-4db9-8015-c645a5dbec2a",
    "https://mainnet.helius-rpc.com/?api-key=9c47aa2b-a959-48f3-b0bb-19197399ebdd",
    "https://mainnet.helius-rpc.com/?api-key=8840d980-a78a-40c7-badb-1bbb76f345a8",
    "https://mainnet.helius-rpc.com/?api-key=8f0e1e15-0884-428d-8e8c-b63bef419afb",
    "https://mainnet.helius-rpc.com/?api-key=714764ab-e507-45c3-9d9c-ee9faba15969",
    "https://mainnet.helius-rpc.com/?api-key=531d9d63-29e8-4422-b2db-ad30be5832e5",
    "https://mainnet.helius-rpc.com/?api-key=2ab8061b-13ab-4380-b949-d23b61ccdf8c",
    "https://mainnet.helius-rpc.com/?api-key=053e484e-b13e-4630-8b55-d73f08b223c8",
    "https://mainnet.helius-rpc.com/?api-key=ddd76fd4-8a11-4256-b65c-b6a9b5ca791d",
    "https://mainnet.helius-rpc.com/?api-key=4f8994f6-067d-4ab1-a269-ee83ee89e7ee",
    "https://mainnet.helius-rpc.com/?api-key=26739b70-3367-42e7-982d-ffd1734277d3",
    "https://mainnet.helius-rpc.com/?api-key=ed900847-c172-4f88-ba04-09010f63956c",
    "https://mainnet.helius-rpc.com/?api-key=fd3cc290-1e49-434a-8734-cab8b6ac8543",
    "https://mainnet.helius-rpc.com/?api-key=4f80841b-f941-4728-ad40-9bc21e4eb446",
    "https://mainnet.helius-rpc.com/?api-key=7c2891f4-4eb0-4311-ae75-2f52db7d346b",
    "https://mainnet.helius-rpc.com/?api-key=0e1e4cbc-2f93-42f6-beda-8020960ea513",
    "https://mainnet.helius-rpc.com/?api-key=c3840a38-dac0-446c-a2a1-55577c7c56d3",
    "https://mainnet.helius-rpc.com/?api-key=58392a61-d966-43ba-84b3-0f9dfd202efc",
    "https://mainnet.helius-rpc.com/?api-key=38bb1394-345c-4107-bc25-4feb49fb136a",
    "https://mainnet.helius-rpc.com/?api-key=aa69ff89-1cb8-4fa1-b38d-fab05c28962f",
    "https://mainnet.helius-rpc.com/?api-key=7db9e2fd-6a31-4b78-aba2-0c3e018f7d15",
    "https://mainnet.helius-rpc.com/?api-key=65da2f11-2139-452f-9794-3bb41dcf650b",
    "https://mainnet.helius-rpc.com/?api-key=ce9a4edf-4104-4020-8add-d9a85e9ed643",
    "https://mainnet.helius-rpc.com/?api-key=69693adf-35a5-4d29-a850-9dc12be47d55",
    "https://mainnet.helius-rpc.com/?api-key=f3b7586e-c925-4b04-9fca-1aca7c06d4f2",
    "https://mainnet.helius-rpc.com/?api-key=29336a2a-ee49-4e6a-af6e-9c7f9dcc305c",
    "https://mainnet.helius-rpc.com/?api-key=ad5a6395-c0bf-45fb-bf96-646815eed100",
    "https://mainnet.helius-rpc.com/?api-key=9a4bf358-9c14-4ebf-9480-31d2b3d9e272",
    "https://mainnet.helius-rpc.com/?api-key=e16d5f85-9f0e-41eb-943c-e24b59cbea40",
    "https://mainnet.helius-rpc.com/?api-key=2e7c278f-ff29-4ddc-9731-a27764f7247d",
    "https://mainnet.helius-rpc.com/?api-key=c4908b25-68dd-44ec-81f9-0151ff32aace",
    "https://mainnet.helius-rpc.com/?api-key=3bad78a5-aa2b-4a66-be6b-675e444cd068",
    "https://mainnet.helius-rpc.com/?api-key=6d290c84-ef0d-40ef-afc5-b62d22354bec",
    "https://mainnet.helius-rpc.com/?api-key=0009353c-7ac8-43b4-9caa-50cf2cf11637",
    "https://mainnet.helius-rpc.com/?api-key=98c975ba-1e53-4e7f-8399-7af48f280ba6",
    "https://mainnet.helius-rpc.com/?api-key=6ed0a6ef-f105-4b8e-899d-0d6991e47d9d",
    "https://mainnet.helius-rpc.com/?api-key=03cdaa6c-ff88-49af-9827-21281740bc5c",
    "https://mainnet.helius-rpc.com/?api-key=70916925-977a-462c-be1b-876f674c3d24",
    "https://mainnet.helius-rpc.com/?api-key=62dca298-2e6a-45b8-b1e1-98b37f6da47e",
    "https://mainnet.helius-rpc.com/?api-key=ad97cab2-a705-42f2-be6a-34e09971a3da",
    "https://mainnet.helius-rpc.com/?api-key=dd5e174c-ba07-4c6d-9411-01ceabf184ef",
    "https://mainnet.helius-rpc.com/?api-key=3da6727e-7a0e-4186-91e3-215c739c9604",
    "https://mainnet.helius-rpc.com/?api-key=2bc536e4-bf63-43f8-a98f-12727414a396",
    "https://mainnet.helius-rpc.com/?api-key=c0e76909-2819-459c-a800-c23f4efaace2",
    "https://mainnet.helius-rpc.com/?api-key=8985d57b-579f-4466-bcc6-e9b01e0252be",
    "https://mainnet.helius-rpc.com/?api-key=9c9c878d-8843-4336-975c-8e9f4896ebd9",
    "https://mainnet.helius-rpc.com/?api-key=491ba0d7-76be-4373-9e70-e2ea304c60bc",
    "https://mainnet.helius-rpc.com/?api-key=0184be6f-01ec-4d06-8daf-94f9d7d83de1",
    "https://mainnet.helius-rpc.com/?api-key=e8402b50-9594-4665-b9ea-48b5374a5063",
    "https://mainnet.helius-rpc.com/?api-key=7960a8ac-ff0e-4353-8407-1b747bcecfbe",
    "https://mainnet.helius-rpc.com/?api-key=03fbaf69-5407-4b86-a234-fbf228120123",
    "https://mainnet.helius-rpc.com/?api-key=8c1cbf5e-d3cd-4c39-aeae-37a0e2764d40",
    "https://mainnet.helius-rpc.com/?api-key=ff195396-436d-4838-98f4-5fc9548aa435",
    "https://mainnet.helius-rpc.com/?api-key=0170f8e7-26d9-4b0c-a357-a10f9fb48902",
    "https://mainnet.helius-rpc.com/?api-key=26b21ba0-9008-4c50-85a0-ac6a72e9f64e",
    "https://mainnet.helius-rpc.com/?api-key=42ae365c-9cb8-473c-ba40-f5105e9d5ed2",
    "https://mainnet.helius-rpc.com/?api-key=3418f2cd-aa1f-4ec3-b647-d2197e665649",
    "https://mainnet.helius-rpc.com/?api-key=d77edd15-35e4-404d-a2af-842ed6571c95",
    "https://mainnet.helius-rpc.com/?api-key=4e126255-e8ed-4550-b71b-cd85b69091f7",
    "https://mainnet.helius-rpc.com/?api-key=1e3a82f3-5c4d-483d-baf6-03bf0e36264d",
    "https://mainnet.helius-rpc.com/?api-key=d2fd556c-2a4b-41b7-b4bc-99f6fbf528f9",
    "https://mainnet.helius-rpc.com/?api-key=4331ba51-4650-478e-994f-c57da42b6d01",
    "https://mainnet.helius-rpc.com/?api-key=ec78664d-4755-4c75-b109-65b310e5efaa",
    "https://mainnet.helius-rpc.com/?api-key=5a0bf8f6-02d6-413a-aa49-3d39a18eec93",
    "https://mainnet.helius-rpc.com/?api-key=49f7dac3-5ccd-46ed-b263-b10f2c10fad0",
    "https://mainnet.helius-rpc.com/?api-key=23345b08-5bcc-4c23-815e-83349b887c90",
    "https://mainnet.helius-rpc.com/?api-key=ac01022a-6c93-46a8-a5d2-8c12f49a62b6",
    "https://mainnet.helius-rpc.com/?api-key=75dc26ec-833d-42ed-96cc-8c54a2fe204e",
    "https://mainnet.helius-rpc.com/?api-key=7dbde49d-40f7-46b8-8de9-ad0677bb0d1a",
    "https://mainnet.helius-rpc.com/?api-key=3c8ccca8-3cca-4892-8730-a51fe1a0cd9c",
    "https://mainnet.helius-rpc.com/?api-key=7020494d-74f3-4b8e-a8a8-be6ba12ee1d5",
    "https://mainnet.helius-rpc.com/?api-key=25cf9579-e442-47c5-bcfd-95b16d2c0732",
    "https://mainnet.helius-rpc.com/?api-key=56ce7347-51af-4271-8409-89b368bb82d1",
    "https://mainnet.helius-rpc.com/?api-key=74e37ad3-cb54-46c3-98fe-47bcc6cd87c7",
    "https://mainnet.helius-rpc.com/?api-key=5d0ad73e-d083-489d-b394-36afc9836c17",
    "https://mainnet.helius-rpc.com/?api-key=1bf6b1ab-d60a-4972-96aa-cdefac27c19f",
    "https://mainnet.helius-rpc.com/?api-key=1b4e4ea3-fe2c-44fc-bea7-79ed5a94ffcf",
    "https://mainnet.helius-rpc.com/?api-key=a29cf5a3-3f88-4cd4-aecf-f2c0a8ea2706",
    "https://mainnet.helius-rpc.com/?api-key=e8bd57e2-2d2b-4532-b8ca-0b730a822454",
    "https://mainnet.helius-rpc.com/?api-key=1c217c8e-aa09-4e20-bd14-dfe26f14e346",
    "https://mainnet.helius-rpc.com/?api-key=e29eb629-2be2-4b92-9e12-c243ab7964f0",
    "https://mainnet.helius-rpc.com/?api-key=4639f99b-0b71-4df1-be8c-2f9be1a42724",
    "https://mainnet.helius-rpc.com/?api-key=cc76d626-19f1-4b85-8a0b-44a86d23b5f4",
    "https://mainnet.helius-rpc.com/?api-key=8eb8e6e2-df5b-4fef-a5f6-215300f1eb54",
    "https://mainnet.helius-rpc.com/?api-key=ca6179cc-6d9e-4195-b2db-e91a52e1aaf9",
    "https://mainnet.helius-rpc.com/?api-key=66bccc03-88ec-4b3c-ad72-72a53a4daf78",
    "https://mainnet.helius-rpc.com/?api-key=179d44b1-e65d-4a71-97f4-ac0a16b9ec55",
    "https://mainnet.helius-rpc.com/?api-key=d934d435-cdfc-4f66-9e5d-c09eb84f4689",
    "https://mainnet.helius-rpc.com/?api-key=b026de02-b6ec-4fa7-8cc3-f6f8827a5699",
    "https://mainnet.helius-rpc.com/?api-key=55c193eb-b38b-48d8-9563-114d95f94c81",
    "https://mainnet.helius-rpc.com/?api-key=5cbd61d8-15b3-4d4f-b3a8-6de44a17a12f",
    "https://mainnet.helius-rpc.com/?api-key=3ecf775c-9834-4d04-bd8d-d2529d517358",
    "https://mainnet.helius-rpc.com/?api-key=4f60704f-0b87-4ced-af5c-65fca5828823",
    "https://mainnet.helius-rpc.com/?api-key=009ec486-0c26-4c60-802f-ff927cfd440a",
    "https://mainnet.helius-rpc.com/?api-key=41da901d-acac-4be1-9754-199fb8e9d6ce",
    "https://mainnet.helius-rpc.com/?api-key=dc25441a-8ab4-42be-87df-5cbcde4fc849",
    "https://mainnet.helius-rpc.com/?api-key=3faae40f-7e87-428e-9180-66b372617479",
    "https://mainnet.helius-rpc.com/?api-key=ebea550a-3a03-4e08-9e2b-133612d03c27",
    "https://mainnet.helius-rpc.com/?api-key=753762bc-bdab-4582-8d53-15661e68d1a0",
    "https://mainnet.helius-rpc.com/?api-key=d84a5571-ad3a-4ea1-832a-3af245a872e7",
    "https://mainnet.helius-rpc.com/?api-key=fd28987d-b917-45c8-bfcd-29c65c0c3feb",
    "https://mainnet.helius-rpc.com/?api-key=edcd8c12-c006-4512-ac9e-f471ee89a60d",
    "https://mainnet.helius-rpc.com/?api-key=e923c21e-5e8b-475e-9784-5c642734c774",
    "https://mainnet.helius-rpc.com/?api-key=78549850-1c9f-4362-bbe2-2ec2042bd5fe",
    "https://mainnet.helius-rpc.com/?api-key=35f91d28-7c13-455c-9ac7-2488c8279217",
    "https://mainnet.helius-rpc.com/?api-key=65d6175d-3724-4824-b747-ec69386ef004",
    "https://mainnet.helius-rpc.com/?api-key=b0aafbdc-5fbb-4ef0-812e-16aac7691bcb",
    "https://mainnet.helius-rpc.com/?api-key=14e844d4-e99c-46ab-ae41-6d23e3f3993d",
    "https://mainnet.helius-rpc.com/?api-key=1104c75e-f47b-4e5d-9e24-fa9c12679d06",
    "https://mainnet.helius-rpc.com/?api-key=439fab1b-2204-4f48-a48b-af690a5870bf",
    "https://mainnet.helius-rpc.com/?api-key=7afdb6c5-d7a7-4b34-844d-29fac0156d05",
    "https://mainnet.helius-rpc.com/?api-key=670c3bec-10fe-4090-ab6f-503d5a43ec51",
    "https://mainnet.helius-rpc.com/?api-key=22786980-13ec-4fb2-92a8-a69b4e5a09d4",
    "https://mainnet.helius-rpc.com/?api-key=9f42feab-5253-489b-98d0-0b85fe167e4b",
    "https://mainnet.helius-rpc.com/?api-key=bfdda611-d20a-449a-bef6-6b6264a10ae3",
    "https://mainnet.helius-rpc.com/?api-key=766f44be-c7c2-42de-9533-f15b6714c505",
    "https://mainnet.helius-rpc.com/?api-key=f0e75132-7df7-47c8-8125-fb986249b5bc",
    "https://mainnet.helius-rpc.com/?api-key=19436271-c953-4448-aa7e-a1834e8fa750",
    "https://mainnet.helius-rpc.com/?api-key=022003a9-ec3f-45ce-b2cf-ca93469d8a78",
    "https://mainnet.helius-rpc.com/?api-key=9ad18ee7-211a-43d2-9c86-8ad7913f6116",
    "https://mainnet.helius-rpc.com/?api-key=f0ef862e-6d83-4428-b3c1-4d788864f73c",
    "https://mainnet.helius-rpc.com/?api-key=e10c5958-68de-4f07-95a2-72033c4250ce",
    "https://mainnet.helius-rpc.com/?api-key=766939ac-8f99-46b5-9f9e-25e8b427a9b5",
    "https://mainnet.helius-rpc.com/?api-key=1510151a-7263-415a-8794-d21f5f334ee3",
    "https://mainnet.helius-rpc.com/?api-key=b2d25289-a0a4-4284-bae3-53cd5b8ad21c",
    "https://mainnet.helius-rpc.com/?api-key=bd0cf1d6-ba28-49db-aaca-f6af3a9b08bb",
    "https://mainnet.helius-rpc.com/?api-key=b9d3d49e-4af5-4f80-af4c-aa5a115e3215",
    "https://mainnet.helius-rpc.com/?api-key=10116e69-1563-4c9f-9e52-a76d09e3fd0a",
    "https://mainnet.helius-rpc.com/?api-key=904fc984-12ef-46a4-a1b5-5d2c70914ff6",
    "https://mainnet.helius-rpc.com/?api-key=d274bae9-b70e-4858-8710-8512b8e2d88f",
    "https://mainnet.helius-rpc.com/?api-key=92f169a2-778c-4580-a096-773a09f02dd9",
    "https://mainnet.helius-rpc.com/?api-key=08252155-c838-4e53-8532-e60c6b4292f1",
    "https://mainnet.helius-rpc.com/?api-key=7d51ce3e-e309-4c5f-a6d0-6fdf1b87edfe",
    "https://mainnet.helius-rpc.com/?api-key=13333d70-5eba-4caa-b133-6f1bc15fcbf4",
    "https://mainnet.helius-rpc.com/?api-key=b76f2d76-f167-4172-b346-b4936d767d31",
    "https://mainnet.helius-rpc.com/?api-key=acbc6eae-ecde-4f83-aa49-7dc312a85bd0",
    "https://mainnet.helius-rpc.com/?api-key=f1d6189f-cee0-4cfb-9ee0-289d2642953f",
    "https://mainnet.helius-rpc.com/?api-key=84e155d6-cdf4-4604-b101-1ef88f3e5e23",
    "https://mainnet.helius-rpc.com/?api-key=a708bcba-ce74-4c51-9ef9-f72783e5a41f",
    "https://mainnet.helius-rpc.com/?api-key=4319e134-33d4-4403-a74d-a0261d8ac346",
    "https://mainnet.helius-rpc.com/?api-key=4c0d4873-b86a-4276-a7b1-92c3372a3eae",
    "https://mainnet.helius-rpc.com/?api-key=d5e37a68-9f1e-413a-a9d5-9cf86069e163",
    "https://mainnet.helius-rpc.com/?api-key=1ff4bf6c-ff5d-4f74-ba64-56d4b905d59e",
    "https://mainnet.helius-rpc.com/?api-key=90102bbe-d1c7-48de-87b0-3778410fc23f",
    "https://mainnet.helius-rpc.com/?api-key=005cbeac-6dea-4be1-a6f5-cc10450ed3d4",
    "https://mainnet.helius-rpc.com/?api-key=9d6d8582-cc9f-4bba-be92-07a45717c5b4",
    "https://mainnet.helius-rpc.com/?api-key=655d95af-64ba-45e5-a978-002995b3fbcf",
    "https://mainnet.helius-rpc.com/?api-key=9a06b1b6-fbd3-4586-8d56-871a289d5b22",
    "https://mainnet.helius-rpc.com/?api-key=d8c1c0f0-322c-41b4-b46c-9e7b64621789",
    "https://mainnet.helius-rpc.com/?api-key=cc710a5b-de2a-437e-87ea-130a320cf6bc",
    "https://mainnet.helius-rpc.com/?api-key=ca08c8d7-6917-4e9b-b5ad-2a87341b8238",
    "https://mainnet.helius-rpc.com/?api-key=c8df1c81-71a1-443b-9487-56dd9869ca10",
    "https://mainnet.helius-rpc.com/?api-key=7f737790-6049-4da1-9dfe-a651054776b4",
    "https://mainnet.helius-rpc.com/?api-key=df8b6969-eb62-4c3c-b0c7-87f5bafc6f5f",
    "https://mainnet.helius-rpc.com/?api-key=ec64b721-4be9-48a9-ba17-4c910bc88a35",
    "https://mainnet.helius-rpc.com/?api-key=8f1e1fcf-3844-46e1-badd-d51085389483",
    "https://mainnet.helius-rpc.com/?api-key=36b4dd00-c4ec-4965-b271-0947490bef51",
    "https://mainnet.helius-rpc.com/?api-key=bc0b4ee6-a93c-403e-ab65-4237936729d8",
    "https://mainnet.helius-rpc.com/?api-key=7cc7cba8-8d23-41b4-9a62-a26ee722d4db",
    "https://mainnet.helius-rpc.com/?api-key=02f4deb2-993f-4a3c-8a06-f7d60c1e63cf",
    "https://mainnet.helius-rpc.com/?api-key=396d8ab2-23e0-430d-a931-ce2d48deb8d7",
    "https://mainnet.helius-rpc.com/?api-key=9cc15b24-f5b6-4a4c-be2b-51de6f277960",
    "https://mainnet.helius-rpc.com/?api-key=88e0faa6-c60c-49a8-861d-38d77829f666",
    "https://mainnet.helius-rpc.com/?api-key=7e0b12cf-dcf7-4a21-8c33-58019319da53",
    "https://mainnet.helius-rpc.com/?api-key=c38b2933-dec0-4843-8491-5c2fce8a0531",
    "https://mainnet.helius-rpc.com/?api-key=3281f3ac-82ef-4cb2-8bee-a4497a54f6d4",
    "https://mainnet.helius-rpc.com/?api-key=3f141aa7-2e3b-4237-a726-ad1186c97ca6",
    "https://mainnet.helius-rpc.com/?api-key=95cafb46-91b4-4099-9b55-0fdad8e02442",
    "https://mainnet.helius-rpc.com/?api-key=e6018184-3789-4212-a3fe-9b48a1a728dd",
    "https://mainnet.helius-rpc.com/?api-key=53c7f171-4101-4d96-8153-2c237a13ad34",
    "https://mainnet.helius-rpc.com/?api-key=271f6c3d-6350-4798-81cb-dbd128b95b38",
    "https://mainnet.helius-rpc.com/?api-key=01f2e9c4-e75b-46fd-a38c-3f3ea25b444c",
    "https://mainnet.helius-rpc.com/?api-key=b2fa14ad-b10f-447c-872c-172a825a15c0",
    "https://mainnet.helius-rpc.com/?api-key=70c3a3ae-36d1-468e-b8b1-a0b626bb3d7a",
    "https://mainnet.helius-rpc.com/?api-key=abc87f8d-0d67-49af-9378-44b3b6a3ce8a",
    "https://mainnet.helius-rpc.com/?api-key=1308adc9-e071-42dd-a6ee-cd78f67fb477",
    "https://mainnet.helius-rpc.com/?api-key=edbf13cc-061e-4e68-a879-542146683f5c",
    "https://mainnet.helius-rpc.com/?api-key=a956f358-21ee-4926-b259-95f9f5c69a0a",
    "https://mainnet.helius-rpc.com/?api-key=707e23e3-dc49-447a-984b-b181a880b7cd",
    "https://mainnet.helius-rpc.com/?api-key=9b7870ea-89c4-4d73-b392-851720e82318",
    "https://mainnet.helius-rpc.com/?api-key=c7470c41-dfba-44ca-a798-4d5150af79d2",
    "https://mainnet.helius-rpc.com/?api-key=5ebfa538-8c66-49a7-b5c1-9e6eaf3d9cda",
    "https://mainnet.helius-rpc.com/?api-key=5dab74f8-955b-4b18-9860-75b81910be78",
    "https://mainnet.helius-rpc.com/?api-key=7763c2ef-544a-4077-a788-803f6d29d6fe",
    "https://mainnet.helius-rpc.com/?api-key=48d21f7e-30ac-4a2b-b729-f403fa8e63f4",
    "https://mainnet.helius-rpc.com/?api-key=518da640-4e42-45d6-93dc-d6317bece499",
    "https://mainnet.helius-rpc.com/?api-key=ea689039-1f99-402a-ba26-bf6161226243",
    "https://mainnet.helius-rpc.com/?api-key=474c6663-b445-4a8e-b842-90b21dfed8d9",
    "https://mainnet.helius-rpc.com/?api-key=5d651180-5384-40f8-af98-99abaa16ee24",
    "https://mainnet.helius-rpc.com/?api-key=aa51c9bf-9393-4733-99d4-47db8329ace0",
    "https://mainnet.helius-rpc.com/?api-key=0b26e2dd-1004-451c-915a-bbc93076ff5d",
    "https://mainnet.helius-rpc.com/?api-key=291314f9-49cb-4e00-9fbc-fc0ed0ef5b47",
    "https://mainnet.helius-rpc.com/?api-key=0330848b-dbe1-4271-9130-00804be53a07",
    "https://mainnet.helius-rpc.com/?api-key=973586c5-96e0-4b10-99d6-5f9e65fa080e",
    "https://mainnet.helius-rpc.com/?api-key=8a5457e2-7ce1-49e4-83a3-ab55cebe7624",
    "https://mainnet.helius-rpc.com/?api-key=dc8d8cae-13f3-4b19-a909-1ec4ac4b8137",
    "https://mainnet.helius-rpc.com/?api-key=49f6bb0a-ae5e-44b3-abef-80dbca7abcef",
    "https://mainnet.helius-rpc.com/?api-key=a71bea72-4a59-4a7c-8deb-56a3f3e3ed4a",
    "https://mainnet.helius-rpc.com/?api-key=0d3ba929-c2a5-4af8-8d11-f585918be814",
    "https://mainnet.helius-rpc.com/?api-key=37db39c7-d966-435a-aec1-419d84a97825",
    "https://mainnet.helius-rpc.com/?api-key=ca4d4e82-9556-403a-9e5c-ca72f554a8b4",
    "https://mainnet.helius-rpc.com/?api-key=3cd9446d-c1e1-4d5b-be45-f5de83ef9952",
    "https://mainnet.helius-rpc.com/?api-key=df53e87c-3f5e-44c9-80c4-2602a8ee5283",
    "https://mainnet.helius-rpc.com/?api-key=ec7a3196-6d1f-488b-8740-5b7481ad808c",
    "https://mainnet.helius-rpc.com/?api-key=dec804cf-0ce5-42ae-9207-375694e8d406",
    "https://mainnet.helius-rpc.com/?api-key=8684dc95-d4c0-46ec-878b-c57fa122a98d",
    "https://mainnet.helius-rpc.com/?api-key=ad731a8f-046c-48aa-95d7-c8a1131cda20",
    "https://mainnet.helius-rpc.com/?api-key=609a666b-c220-491c-829a-0f5e0487ce12",
    "https://mainnet.helius-rpc.com/?api-key=b0d3a6ee-dab4-492f-9ebb-a6d82bb5210c",
    "https://mainnet.helius-rpc.com/?api-key=13c2e5a6-f580-4069-9cc6-8ac7e73d7774",
    "https://mainnet.helius-rpc.com/?api-key=0baa2ed8-946e-49ca-8ffa-5122aff8dbe9",
    "https://mainnet.helius-rpc.com/?api-key=0959679c-d249-4d63-aea4-6f199e25e066",
    "https://mainnet.helius-rpc.com/?api-key=a7bcddc5-4fc6-4bc0-907f-4c3c83790633",
    "https://mainnet.helius-rpc.com/?api-key=85ac2ba6-0ca8-47f7-9fb3-fbe7979a896e",
    "https://mainnet.helius-rpc.com/?api-key=049919a7-b8e6-40cd-8cc9-7021b02f84c5",
    "https://mainnet.helius-rpc.com/?api-key=48efd91b-06ba-4f96-a123-7c08ab773b6d",
    "https://mainnet.helius-rpc.com/?api-key=ae09f129-4e87-4ad1-a66c-66017919b6b9",
    "https://mainnet.helius-rpc.com/?api-key=82bc7368-3418-47f4-907a-6e7e8ec7a0a0",
    "https://mainnet.helius-rpc.com/?api-key=99180771-91dc-4a09-b05a-8e1c8d89136f",
    "https://mainnet.helius-rpc.com/?api-key=2d1ba14d-a692-46e4-82e3-29d1bf773cf8",
    "https://mainnet.helius-rpc.com/?api-key=7f725812-cf5b-425c-aa38-c2a352bd4d41",
    "https://mainnet.helius-rpc.com/?api-key=a2c77665-0924-4d06-9f58-af10acb1664e",
    "https://mainnet.helius-rpc.com/?api-key=fb1c18af-df95-455e-8523-d67337032747",
    "https://mainnet.helius-rpc.com/?api-key=06b1ea32-5850-481d-bcb3-bdf65f70420a",
    "https://mainnet.helius-rpc.com/?api-key=e1e507c9-fcbc-485a-99e5-a118a74be6e7",
    "https://mainnet.helius-rpc.com/?api-key=228d70e8-d0cd-42a2-a273-41fb1e33f434",
    "https://mainnet.helius-rpc.com/?api-key=b4913495-1656-487f-8b3a-e36f6e7cb513",
    "https://mainnet.helius-rpc.com/?api-key=ca38b3cb-4904-4418-ba7b-3c98013bb48e",
    "https://mainnet.helius-rpc.com/?api-key=77090156-24fc-4589-8564-3ff28b25ac2a",
    "https://mainnet.helius-rpc.com/?api-key=5d435722-ba76-4258-9e7a-8370b5c222fa",
    "https://mainnet.helius-rpc.com/?api-key=af9a8f83-7a09-40ca-94c9-13b6c5a17cd8",
    "https://mainnet.helius-rpc.com/?api-key=4243eca0-e65a-424a-9e8f-c59fc84a2a3e",
    "https://mainnet.helius-rpc.com/?api-key=d507dbb7-43c3-4a97-b960-9252b78b1aa1",
    "https://mainnet.helius-rpc.com/?api-key=0357f1d1-9eab-421e-84bc-b19234ec6c3e",
    "https://mainnet.helius-rpc.com/?api-key=626116d2-2c77-4f86-8729-56f8778eb3ed",
    "https://mainnet.helius-rpc.com/?api-key=66941176-1621-47a8-a408-967648945a37",
    "https://mainnet.helius-rpc.com/?api-key=1a5714ea-2998-45a3-bf6a-09bc2322d59d",
    "https://mainnet.helius-rpc.com/?api-key=34ae02f1-bc3c-4f3a-90ec-bd77606f8c1b",
    "https://mainnet.helius-rpc.com/?api-key=ae2ad1cd-bcd2-4d90-83fc-ec846d21d783",
    "https://mainnet.helius-rpc.com/?api-key=434a0f8a-a382-4628-9eb2-af27a055fa46",
    "https://mainnet.helius-rpc.com/?api-key=9ca7a76d-04df-4637-aad8-7b721ab113e1",
    "https://mainnet.helius-rpc.com/?api-key=0d1139e4-774e-448e-a84a-d9139ef7f53a",
    "https://mainnet.helius-rpc.com/?api-key=029b5924-e921-4160-b311-e75d4c6ccff5",
    "https://mainnet.helius-rpc.com/?api-key=0c994faf-aa63-440c-b662-f09693ed8d16",
    "https://mainnet.helius-rpc.com/?api-key=2b7fdd21-7278-4bc6-8097-2f1a940909e3",
    "https://mainnet.helius-rpc.com/?api-key=678c4961-3fbb-4765-a9e0-37636a106235",
    "https://mainnet.helius-rpc.com/?api-key=25542f22-0511-4157-9c86-0ce61f5abc8a",
    "https://mainnet.helius-rpc.com/?api-key=880723de-fa69-48c9-9155-0a97fbc924c1",
    "https://mainnet.helius-rpc.com/?api-key=9946d797-ea10-492a-89b7-00844b5caa7d",
    "https://mainnet.helius-rpc.com/?api-key=7552c3cc-700b-4fba-93e2-78aff57ed49f",
    "https://mainnet.helius-rpc.com/?api-key=df0f0cde-ebc6-421f-9b8c-2259024440da",
    "https://mainnet.helius-rpc.com/?api-key=fb3f70ab-849f-4567-b04a-0614f5431fa2",
    "https://mainnet.helius-rpc.com/?api-key=4a288cec-2606-434c-a1a3-2156ee7c0fbf",
    "https://mainnet.helius-rpc.com/?api-key=bfd9cf44-df82-4130-9be5-aa5f485083a3",
    "https://mainnet.helius-rpc.com/?api-key=dfcd6433-c46b-41e8-ad55-607ff391962b",
    "https://mainnet.helius-rpc.com/?api-key=af8d9b96-cf89-4ffb-864e-b736c7b58c61",
    "https://mainnet.helius-rpc.com/?api-key=12e95f30-1ef6-41aa-bc65-63f7ed4e4c37",
    "https://mainnet.helius-rpc.com/?api-key=37e2cb7a-04ef-4096-82b1-c6cb92bdca3c",
    "https://mainnet.helius-rpc.com/?api-key=97bfea0e-c726-448e-88cd-1f9d4e9e896e",
    "https://mainnet.helius-rpc.com/?api-key=0056c3d8-ce0c-4411-aeea-1eab14193c42",
    "https://mainnet.helius-rpc.com/?api-key=2985eb24-7c9f-499c-9ee1-b966252963a2",
    "https://mainnet.helius-rpc.com/?api-key=65d88fb3-419a-4865-ba85-7fe771a64cc1",
    "https://mainnet.helius-rpc.com/?api-key=d5279991-a90e-4d4c-9772-2cdc304d6838",
    "https://mainnet.helius-rpc.com/?api-key=34c2237b-bedb-4363-a626-5606cf8de063",
    "https://mainnet.helius-rpc.com/?api-key=4f97d8a3-5704-46ad-bbd9-5afdc241c146",
    "https://mainnet.helius-rpc.com/?api-key=76197ffe-f6d0-4ed5-80ea-0454b511f06c",
    "https://mainnet.helius-rpc.com/?api-key=2b56b97e-59d6-4f9b-8152-c2cad0d6b18b",
    "https://mainnet.helius-rpc.com/?api-key=b2396eda-0b4d-4898-b910-aa34c17e49fc",
    "https://mainnet.helius-rpc.com/?api-key=16d72390-689e-4eaf-b8aa-9e7aeb3d121c",
    "https://mainnet.helius-rpc.com/?api-key=3058b7d8-2af2-4c63-9bc3-a46f2cac6d7d",
    "https://mainnet.helius-rpc.com/?api-key=2379c6ec-7db2-45a5-97f5-3ea3b51f6f32",
    "https://mainnet.helius-rpc.com/?api-key=2dbf6a85-ec94-4c06-beaf-75601b57a86a",
    "https://mainnet.helius-rpc.com/?api-key=c7eed19c-72d7-4ccd-ab39-5f4f1aab1cee",
    "https://mainnet.helius-rpc.com/?api-key=facf76e3-4a5b-4dea-bb0c-4ef87df26f92",
    "https://mainnet.helius-rpc.com/?api-key=86c0b6fa-5547-4d42-a51e-fa311ac86fdd",
    "https://mainnet.helius-rpc.com/?api-key=a8c9730e-907c-4910-893a-e7a1c1004154",
    "https://mainnet.helius-rpc.com/?api-key=cd5e5aac-43e6-4358-a9e5-eec924677c20",
    "https://mainnet.helius-rpc.com/?api-key=a65332a0-4d85-42bf-a7c9-4608d5022bd7",
    "https://mainnet.helius-rpc.com/?api-key=ed789ca7-2cd5-4094-b423-b0a8f52ff86f",
    "https://mainnet.helius-rpc.com/?api-key=b063b4aa-367a-4e31-af95-8ad79c39ad80",
    "https://mainnet.helius-rpc.com/?api-key=2a9b22bb-bbfb-4440-a5f9-4f58434c42ab",
    "https://mainnet.helius-rpc.com/?api-key=0246f67a-1d2a-4fdc-9ef9-756ba2cb6650",
    "https://mainnet.helius-rpc.com/?api-key=7895c55f-a758-4db7-b631-6d94d753a5bc",
    "https://mainnet.helius-rpc.com/?api-key=7c602353-1212-4089-9d3c-a3a872c4ba18",
    "https://mainnet.helius-rpc.com/?api-key=f9e678d9-5379-4ffb-91aa-12b47fb977d1",
    "https://mainnet.helius-rpc.com/?api-key=6ffc7722-00ee-4aa2-99fe-30ac06c52812",
    "https://mainnet.helius-rpc.com/?api-key=1923dcde-450c-4c58-9295-b5cf7a8c9ffe",
    "https://mainnet.helius-rpc.com/?api-key=d0ff69be-eaf2-4525-9fdb-429c163a8af6",
    "https://mainnet.helius-rpc.com/?api-key=ed9d095d-9063-4352-b522-68bcc169875d",
    "https://mainnet.helius-rpc.com/?api-key=ff2911a4-a93d-43b1-b240-0b20e8be378b",
    "https://mainnet.helius-rpc.com/?api-key=949268cc-0c9e-46dd-9c6f-0c8c67d59c50",
    "https://mainnet.helius-rpc.com/?api-key=e881b968-9c8f-48c5-96cd-12df93c09fbb",
    "https://mainnet.helius-rpc.com/?api-key=5ce92725-90d3-4593-87c9-bd67dc38096c",
    "https://mainnet.helius-rpc.com/?api-key=3a47af11-8c52-4eb2-8250-d0e0fab5e24f",
    "https://mainnet.helius-rpc.com/?api-key=3bee75cc-ef63-4d49-aa03-0d9059d67973",
    "https://mainnet.helius-rpc.com/?api-key=302eafe8-5a59-460c-a0ca-b2c1f191712d",
    "https://mainnet.helius-rpc.com/?api-key=60c885c8-4751-4c26-a9ae-6abb4cb262a6",
    "https://mainnet.helius-rpc.com/?api-key=2f138cf7-0db0-49b2-93b6-9f3408e64b93",
    "https://mainnet.helius-rpc.com/?api-key=08c93fa0-a85a-480a-b1cf-0bd0d5bf4dee",
    "https://mainnet.helius-rpc.com/?api-key=b515ee02-1f74-451c-9f63-2142ee5e697d",
    "https://mainnet.helius-rpc.com/?api-key=5b840cf4-cadd-4afd-8cdb-ec0caae085f4",
    "https://mainnet.helius-rpc.com/?api-key=5cf390e4-31b0-45fd-ad71-574c552a9d02",
    "https://mainnet.helius-rpc.com/?api-key=fbe5d09b-586d-4a37-a07c-ad34536ae844",
    "https://mainnet.helius-rpc.com/?api-key=49379087-355f-475a-9eb3-c808cba9887c",
    "https://mainnet.helius-rpc.com/?api-key=145795bc-69c2-4b5d-8105-1f2086259e04",
    "https://mainnet.helius-rpc.com/?api-key=d7926457-5c44-44da-a0b1-425e8af807b7",
    "https://mainnet.helius-rpc.com/?api-key=cd358740-0ac1-4d6b-ad6c-a0615034fa75",
    "https://mainnet.helius-rpc.com/?api-key=14039113-3fbb-4adf-be87-49749cb3baa5",
    "https://mainnet.helius-rpc.com/?api-key=5f232ceb-c10f-47b2-8f11-ab786fd4b749",
    "https://mainnet.helius-rpc.com/?api-key=07c9aefb-331e-40ef-8598-99d1613e0353",
    "https://mainnet.helius-rpc.com/?api-key=ce435b02-f512-4845-b423-65370431c34a",
    "https://mainnet.helius-rpc.com/?api-key=7a851f39-3b38-4c9e-941f-1237145bc41d",
    "https://mainnet.helius-rpc.com/?api-key=18ef61f9-a3e1-458d-be70-a7c3a76dbbc1",
    "https://mainnet.helius-rpc.com/?api-key=b7bbaf4b-e8cb-417e-8f90-8da38884fe15",
    "https://mainnet.helius-rpc.com/?api-key=16003dc6-9d27-492a-938d-99be81110b41",
    "https://mainnet.helius-rpc.com/?api-key=168618cb-24ba-418c-9ebc-79f502d69c1d",
    "https://mainnet.helius-rpc.com/?api-key=484d72fd-0903-4350-8cc6-6ce65639a3e4",
    "https://mainnet.helius-rpc.com/?api-key=dd6f7f29-d001-4f6e-8ff4-cf128488035a",
    "https://mainnet.helius-rpc.com/?api-key=5e4d9eec-31aa-4114-b9a6-08c3b71bfa48",
    "https://mainnet.helius-rpc.com/?api-key=2518b9b4-2a92-4031-b16b-2f75ba7c6824",
    "https://mainnet.helius-rpc.com/?api-key=7faa226e-0ac4-43ab-92f8-3458f339c191",
    "https://mainnet.helius-rpc.com/?api-key=9e26a3ed-e6d6-461e-ba1e-64f37c05ba1b",
    "https://mainnet.helius-rpc.com/?api-key=40b999e1-62d2-4af4-85b2-33e38f247482",
    "https://mainnet.helius-rpc.com/?api-key=192e7a41-7798-4d54-b17a-ce7bb0494245",
    "https://mainnet.helius-rpc.com/?api-key=144f1969-76f0-41fa-b084-36d695d7f7f1",
    "https://mainnet.helius-rpc.com/?api-key=8b04d710-a3d8-4bd2-8956-d9e95be6e2e9",
    "https://mainnet.helius-rpc.com/?api-key=4d7f2a5d-79d8-495b-97e5-819fec74aa00",
    "https://mainnet.helius-rpc.com/?api-key=d1f25d14-de88-4a26-b95a-bfb91e51f6d8",
    "https://mainnet.helius-rpc.com/?api-key=e5552f2e-8113-4ca5-879f-3cf7df67384d",
    "https://mainnet.helius-rpc.com/?api-key=d90483d1-1d8a-45fc-8e38-a3653729f195",
    "https://mainnet.helius-rpc.com/?api-key=a6d65f2b-361e-40ed-96fe-2c7b9992159f",
    "https://mainnet.helius-rpc.com/?api-key=78370b6c-ee0c-4640-81da-57e14e1df779",
    "https://mainnet.helius-rpc.com/?api-key=4701068a-376a-4a9a-8bf9-d20cbc8f66f2",
    "https://mainnet.helius-rpc.com/?api-key=706a9497-5a8a-47bb-ac40-717b397c6ff5",
    "https://mainnet.helius-rpc.com/?api-key=bf5ac68f-8f54-4ef7-9fab-581a7e9b2126",
    "https://mainnet.helius-rpc.com/?api-key=4a2ced11-7333-4ff9-a63e-c46e10a19854",
    "https://mainnet.helius-rpc.com/?api-key=6029c948-3822-4bf7-81c5-24375334ca27",
    "https://mainnet.helius-rpc.com/?api-key=efc2ba0a-0afa-4a02-9756-879ef7770323",
    "https://mainnet.helius-rpc.com/?api-key=911ad79c-83ec-4ba9-8848-25ab1317e1d3",
    "https://mainnet.helius-rpc.com/?api-key=5d98a583-707a-4761-abb1-e4170ec6cffa",
    "https://mainnet.helius-rpc.com/?api-key=7161ceae-9da7-4d89-8a5b-910f70acdb16",
    "https://mainnet.helius-rpc.com/?api-key=f4fe813e-7b9c-4f07-b390-a05fd726f49f",
    "https://mainnet.helius-rpc.com/?api-key=9c327ae5-28cb-4a9f-af87-9753281a6ae6",
    "https://mainnet.helius-rpc.com/?api-key=e4731239-d82e-49a6-ae9d-7323db5108f8",
    "https://mainnet.helius-rpc.com/?api-key=a7de2838-77b1-40db-b5c1-2b60d4af3ece",
    "https://mainnet.helius-rpc.com/?api-key=bf224788-d72f-4877-a441-28be1a2d92fe",
    "https://mainnet.helius-rpc.com/?api-key=07408c19-67ac-4600-b6a3-81f230d62ec5",
    "https://mainnet.helius-rpc.com/?api-key=39bf6ade-c234-4054-bcc8-9bdcead2071c",
    "https://mainnet.helius-rpc.com/?api-key=298f4f62-0efe-44de-8b6c-c2fd4d30268d",
    "https://mainnet.helius-rpc.com/?api-key=82f2d988-8cb3-4efe-81e3-c44a33593f2e",
    "https://mainnet.helius-rpc.com/?api-key=e12ceed1-1b98-45f0-afb7-637bcf112f5d",
    "https://mainnet.helius-rpc.com/?api-key=205bd8ad-b410-40f8-b8d3-76713d9f01f7",
    "https://mainnet.helius-rpc.com/?api-key=240b499d-3870-46ff-bfa8-14d882afdc74",
    "https://mainnet.helius-rpc.com/?api-key=9ad26b66-873b-4b1a-ad71-3c3805994057",
    "https://mainnet.helius-rpc.com/?api-key=964f3a88-0a51-4ec4-a39d-1a769344c4dd",
    "https://mainnet.helius-rpc.com/?api-key=d1fab21b-8a44-40fe-b911-8cb31e7cc8c8",
    "https://mainnet.helius-rpc.com/?api-key=3a451935-3bb8-4ff0-add5-a05e2873e37d",
    "https://mainnet.helius-rpc.com/?api-key=bc4387da-bfd3-45c1-872f-e91d9dab4bd1",
    "https://mainnet.helius-rpc.com/?api-key=f3f0b86b-97dc-4b82-861f-c7fd6e66f6ba",
    "https://mainnet.helius-rpc.com/?api-key=7cc00490-6110-41c7-8dce-d76538ba282e",
    "https://mainnet.helius-rpc.com/?api-key=8ddb3fc5-ba59-4e36-bb46-a6e1df9a79f2",
    "https://mainnet.helius-rpc.com/?api-key=e204207f-5276-4a05-aed5-fa17e8eff84e",
    "https://mainnet.helius-rpc.com/?api-key=5a24d432-afb6-4a0f-a232-ef09ec7687d4",
    "https://mainnet.helius-rpc.com/?api-key=bec78fce-0716-45bc-9309-f333c9d8aa55",
    "https://mainnet.helius-rpc.com/?api-key=a71a08a5-bdda-4997-b388-b920061ada3e",
    "https://mainnet.helius-rpc.com/?api-key=51c257b8-b84e-490e-8fe7-c36e4c4068ab",
    "https://mainnet.helius-rpc.com/?api-key=20a2b710-fa8e-4b19-a732-8bb51bf722cd",
    "https://mainnet.helius-rpc.com/?api-key=e9e86e48-f127-4099-b409-75391ee6a84d",
    "https://mainnet.helius-rpc.com/?api-key=2153038d-33fa-4b49-8324-342b6544e8b9",
    "https://mainnet.helius-rpc.com/?api-key=607ed37d-a830-4be4-a29e-6c4bc7c93b49",
    "https://mainnet.helius-rpc.com/?api-key=e8ace68e-4db8-4783-be6f-279cb9d7a654",
    "https://mainnet.helius-rpc.com/?api-key=de8b2bbb-8eb2-473c-9ca0-4644dabd9c9d",
    "https://mainnet.helius-rpc.com/?api-key=d6588ff6-26c8-4e2c-b91d-72c9587e649a",
    "https://mainnet.helius-rpc.com/?api-key=904da5a2-c23d-41a3-bfcd-8b378bdb8dcc",
    "https://mainnet.helius-rpc.com/?api-key=3489fdb8-1771-4a9b-ad2b-7b9b3823153d",
    "https://mainnet.helius-rpc.com/?api-key=cc7b6059-c61e-4f5c-bf2c-13f360abc07d",
    "https://mainnet.helius-rpc.com/?api-key=c12b3c63-1b82-4ea8-a89b-9f814d3e32fd",
    "https://mainnet.helius-rpc.com/?api-key=6cdd44d6-1444-4174-b136-797000666c53",
    "https://mainnet.helius-rpc.com/?api-key=bfe86b5f-bb73-4e27-918a-1aefc3b36921",
    "https://mainnet.helius-rpc.com/?api-key=c6b5bf83-cfb4-48b7-a16d-307e6a3304e8",
    "https://mainnet.helius-rpc.com/?api-key=34d5d216-85cc-4e6e-9884-33d930bd5b7e",
    "https://mainnet.helius-rpc.com/?api-key=53c3b4aa-8bdd-4ecd-8e7e-bce5fc974cbe",
    "https://mainnet.helius-rpc.com/?api-key=67d9fee5-e209-4216-abef-f640108bae06",
    "https://mainnet.helius-rpc.com/?api-key=e421cf09-7ca1-4dfa-afd6-e5502a899fa0",
    "https://mainnet.helius-rpc.com/?api-key=2f83795a-3384-4e53-af12-7c868e650da2",
    "https://mainnet.helius-rpc.com/?api-key=e1c180c1-e710-4228-b3f1-9cdbd28ea93b",
    "https://mainnet.helius-rpc.com/?api-key=5f1c49f7-4adb-4963-9a68-1bcbad818942",
    "https://mainnet.helius-rpc.com/?api-key=c148eb71-919d-4ae6-bfef-4bddb53e4deb",
    "https://mainnet.helius-rpc.com/?api-key=9ba0250c-96ba-4142-9605-9003aa0e4004",
    "https://mainnet.helius-rpc.com/?api-key=734e3b13-4467-4e48-9131-71d3ae7cd454",
    "https://mainnet.helius-rpc.com/?api-key=d6ddaf5f-4cd5-41bf-b7a9-150183b4b39c",
    "https://mainnet.helius-rpc.com/?api-key=42acdaad-69e8-4d47-aa34-1880eadb4712",
    "https://mainnet.helius-rpc.com/?api-key=0af99b56-2b8e-435f-a0be-d2ddaf5e77f8",
    "https://mainnet.helius-rpc.com/?api-key=e3949896-1265-4894-a5a3-33a27fb1d4db",
    "https://mainnet.helius-rpc.com/?api-key=0b89944c-d577-42d7-ae90-cd09e7c6de06",
    "https://mainnet.helius-rpc.com/?api-key=c42c384d-a867-4b7e-a12b-c8d3167e136a",
    "https://mainnet.helius-rpc.com/?api-key=ebb8f620-f81e-4f4b-a291-f56c9f9fa7a0",
    "https://mainnet.helius-rpc.com/?api-key=d25fcabd-c9a9-46d3-bd2e-3e046d0c7d3e",
    "https://mainnet.helius-rpc.com/?api-key=24c2bf83-c37e-4b6b-b1b8-afa7ca9dea39",
    "https://mainnet.helius-rpc.com/?api-key=675abeb6-dd55-4ae1-a5ce-5e5a3deef704",
    "https://mainnet.helius-rpc.com/?api-key=341438c0-4d02-4656-bcc9-2af7bddf07a6",
    "https://mainnet.helius-rpc.com/?api-key=c8b3a21a-49f4-4384-bb29-21b8687a43f5",
    "https://mainnet.helius-rpc.com/?api-key=8cace7b2-7bd8-46b0-9924-8486fd655c24",
    "https://mainnet.helius-rpc.com/?api-key=9edbe73a-89be-4289-af0a-d5b801c716c6",
    "https://mainnet.helius-rpc.com/?api-key=9fa7a922-6eec-4523-98f2-1df2b21b53a9",
    "https://mainnet.helius-rpc.com/?api-key=3f3ea99a-23a0-437b-9653-262f187ddd2b",
    "https://mainnet.helius-rpc.com/?api-key=db4b2ecf-a35f-4640-b29f-a9c5a115ffb7",
    "https://mainnet.helius-rpc.com/?api-key=4e81905b-cd86-4d6d-925a-b33e4c5738ff",
    "https://mainnet.helius-rpc.com/?api-key=970dba5a-59d5-49e0-9ca9-51bf243d1edc",
    "https://mainnet.helius-rpc.com/?api-key=a0897c4a-00a1-4806-a18e-bbe430c87aad",
    "https://mainnet.helius-rpc.com/?api-key=01bfc83e-a096-4a3b-a8f1-2237627ff5ed",
    "https://mainnet.helius-rpc.com/?api-key=df95c5ba-cc43-4c59-b6ea-85a83fec041c",
    "https://mainnet.helius-rpc.com/?api-key=7a93cc4f-e403-48cb-b59b-0933da260886",
    "https://mainnet.helius-rpc.com/?api-key=44e68d52-8a65-4663-90a6-d977a025e870",
    "https://mainnet.helius-rpc.com/?api-key=d5b1c774-372e-4dc9-b3f6-08822951c7f2",
    "https://mainnet.helius-rpc.com/?api-key=177c57ff-7854-4cf6-9a98-d3db992e07ed",
    "https://mainnet.helius-rpc.com/?api-key=61a356fc-a41d-4833-a44a-24e9bb298fa9",
    "https://mainnet.helius-rpc.com/?api-key=57b6515d-13bb-4e07-82cf-0e089a180e4b",
    "https://mainnet.helius-rpc.com/?api-key=d6d45d7d-c0f6-44da-b041-7f4f9677e06f",
    "https://mainnet.helius-rpc.com/?api-key=17781a41-30ab-44d8-9a35-63fa4469be3e",
    "https://mainnet.helius-rpc.com/?api-key=4a8f44e8-5a99-4b2c-b628-a719ca79710d",
    "https://mainnet.helius-rpc.com/?api-key=e9297b04-8ae5-454f-9f6e-b64733699149",
    "https://mainnet.helius-rpc.com/?api-key=8f670af7-4038-4f46-aea5-4090073dcdca",
    "https://mainnet.helius-rpc.com/?api-key=dd963908-82d6-4f6d-bafc-79284578ecaf",
    "https://mainnet.helius-rpc.com/?api-key=f1d988a7-ac85-47af-9b66-8f4e6740c993",
    "https://mainnet.helius-rpc.com/?api-key=d99cab36-7db0-4b6a-bd1c-05bc50f8445f",
    "https://mainnet.helius-rpc.com/?api-key=07949382-cdaf-455d-aeab-7f8f558eeb66",
    "https://mainnet.helius-rpc.com/?api-key=7f55a065-412a-45e1-ab21-1d10d81d3a5e",
    "https://mainnet.helius-rpc.com/?api-key=bf82de81-f61e-414c-b7f5-1cd167b41f50",
    "https://mainnet.helius-rpc.com/?api-key=2e1705ad-3d2d-44dc-96c3-fe190957381a",
    "https://mainnet.helius-rpc.com/?api-key=3f911de0-a01c-494c-8763-e08bab999265",
    "https://mainnet.helius-rpc.com/?api-key=da99bdd3-6191-4426-97aa-6bd6546f5bfb",
    "https://mainnet.helius-rpc.com/?api-key=fb20d1a6-b757-49ee-b2df-b04bec793564",
    "https://mainnet.helius-rpc.com/?api-key=e247ea37-5754-4297-81a8-034763222a1e",
    "https://mainnet.helius-rpc.com/?api-key=b8ca776f-6dab-4838-8e36-d1b273c42f3f",
    "https://mainnet.helius-rpc.com/?api-key=df6c0881-da9e-42f1-9634-639d336279ff",
    "https://mainnet.helius-rpc.com/?api-key=eabccd45-ab15-4e8b-a2c7-579cd2fb4371",
    "https://mainnet.helius-rpc.com/?api-key=ad876366-421f-417a-8131-dc211cdef551",
    "https://mainnet.helius-rpc.com/?api-key=c2b582be-a240-48bc-9cba-53217c5729fa",
    "https://mainnet.helius-rpc.com/?api-key=561654d8-4008-447b-8925-cbdc79bbaacf",
    "https://mainnet.helius-rpc.com/?api-key=fcce332a-85b9-42e6-8b6b-20229549a02f",
    "https://mainnet.helius-rpc.com/?api-key=5c8d8db1-9e8c-4465-951e-38ba596c942f",
    "https://mainnet.helius-rpc.com/?api-key=2bfaadbc-1ce0-40a4-8cf0-6fecc754f3bb",
    "https://mainnet.helius-rpc.com/?api-key=7de8e79f-6d11-4760-b9ee-ff1445409153",
    "https://mainnet.helius-rpc.com/?api-key=e5cb021a-adfc-4969-b25d-930838c59bb3",
    "https://mainnet.helius-rpc.com/?api-key=72bea6c9-8b40-4ad2-b3bc-079bf2b88db4",
    "https://mainnet.helius-rpc.com/?api-key=399719ee-b417-4207-a9f0-622660ea14e9",
    "https://mainnet.helius-rpc.com/?api-key=6ffc8061-176b-4a16-83ec-12d880dd4b73",
    "https://mainnet.helius-rpc.com/?api-key=8882590f-a035-4de9-925c-d2283ab1a2b2",
    "https://mainnet.helius-rpc.com/?api-key=dc465dba-1cfe-485f-8b3f-e8eb5ab893b2",
    "https://mainnet.helius-rpc.com/?api-key=a17947c2-0486-477d-b5d7-8ceb367f3503",
    "https://mainnet.helius-rpc.com/?api-key=bda03fa9-21b5-4ff9-9b0e-3c6a406837e5",
    "https://mainnet.helius-rpc.com/?api-key=161fa442-46b1-4003-8b69-53116b691358",
    "https://mainnet.helius-rpc.com/?api-key=5209ab80-926e-4d30-bee8-e288145ee272",
    "https://mainnet.helius-rpc.com/?api-key=1c972e09-16a3-40cc-9d1b-0c54e72616fd",
    "https://mainnet.helius-rpc.com/?api-key=94b95223-e7bf-4cd7-9c7b-3d4f65f7f2da",
    "https://mainnet.helius-rpc.com/?api-key=3f5ace14-04d0-4fdd-9496-ec3d14cdb2b6",
    "https://mainnet.helius-rpc.com/?api-key=81cef3f6-3c9e-43e0-9553-9293db60eade",
    "https://mainnet.helius-rpc.com/?api-key=25be7aa8-304a-43f9-9cbf-3bc2b1f396e0",
    "https://mainnet.helius-rpc.com/?api-key=d3f7918b-c230-4d28-99d7-726185f74441",
    "https://mainnet.helius-rpc.com/?api-key=7e68f3cb-61b4-4170-bb76-f589f71e4691",
    "https://mainnet.helius-rpc.com/?api-key=dcb74be7-a117-4e7f-a73d-746bf5a80daa",
    "https://mainnet.helius-rpc.com/?api-key=00447ca4-ba07-4d2b-bd77-1c4d984e3607",
    "https://mainnet.helius-rpc.com/?api-key=aed7df64-ae26-4289-92ad-f3b0703149db",
    "https://mainnet.helius-rpc.com/?api-key=c5ea5b22-77a9-4854-9fa6-58700b3cf8fe",
    "https://mainnet.helius-rpc.com/?api-key=1b7323c9-4ed7-4ff6-b315-0634901710c2",
    "https://mainnet.helius-rpc.com/?api-key=2ca76edb-5f8e-494e-abe2-77dea1b4efcb",
    "https://mainnet.helius-rpc.com/?api-key=d172fc1c-a2f3-4183-a959-e354b48dfd6c",
    "https://mainnet.helius-rpc.com/?api-key=17156375-576f-42f1-ab49-15a392ac51dd",
    "https://mainnet.helius-rpc.com/?api-key=e3c14045-877c-46e4-95dc-0677bf753b94",
    "https://mainnet.helius-rpc.com/?api-key=1b10449c-7ef2-4208-97d1-b0e9458fb8ce",
    "https://mainnet.helius-rpc.com/?api-key=3f7d183a-e5db-4fd2-afca-32ed936b2349",
    "https://mainnet.helius-rpc.com/?api-key=3a0c69b6-eeac-4bf7-bd6b-d56decca309a",
    "https://mainnet.helius-rpc.com/?api-key=bd1fee2f-4755-4d8c-a94b-f41e1d12b2ed",
    "https://mainnet.helius-rpc.com/?api-key=e4b6ff06-7af8-4d31-b6ff-b3ff070195dc",
    "https://mainnet.helius-rpc.com/?api-key=68ca9ce3-36b3-4e3d-8a26-defdd8d2e0b8",
    "https://mainnet.helius-rpc.com/?api-key=c1ec0cd6-eab4-4af3-bf91-c3b69bb1317c",
    "https://mainnet.helius-rpc.com/?api-key=aa7e381e-7482-4851-a8d1-16cb7df6700f",
    "https://mainnet.helius-rpc.com/?api-key=0957f4fa-fe47-4808-9808-1ce6f873794d",
    "https://mainnet.helius-rpc.com/?api-key=82a708a8-8e3a-4036-b2dd-25acfac0cc1f",
    "https://mainnet.helius-rpc.com/?api-key=0510529e-5fc4-4550-8b16-875616cf438d",
    "https://mainnet.helius-rpc.com/?api-key=046ead6e-8391-465a-baf9-28248501fdd9",
    "https://mainnet.helius-rpc.com/?api-key=2879901c-b0ca-4d3f-8dfc-f2ebaa68462c",
    "https://mainnet.helius-rpc.com/?api-key=7c5f59f5-2ac4-4a7c-b755-d8e807fd9b28",
    "https://mainnet.helius-rpc.com/?api-key=1c678ce9-a732-4064-9458-5f624dd3fb73",
    "https://mainnet.helius-rpc.com/?api-key=61b9cae7-34ba-4d89-a1d2-fe5ddcab5c22",
    "https://mainnet.helius-rpc.com/?api-key=6c84d3df-b416-42fa-bedb-ed8b30d5567d",
    "https://mainnet.helius-rpc.com/?api-key=2b282a80-d7db-4855-ad86-5263042b5fc5",
    "https://mainnet.helius-rpc.com/?api-key=9d0a0cc3-871e-4ef9-ab82-b27045612eb1",
    "https://mainnet.helius-rpc.com/?api-key=caac8997-3060-4e30-9bcc-462bc5584c69",
    "https://mainnet.helius-rpc.com/?api-key=8b4c63c3-1688-4cf4-9928-4ac9211d510d",
    "https://mainnet.helius-rpc.com/?api-key=61fc36a3-7bb7-4761-886f-269d4a6a42eb",
    "https://mainnet.helius-rpc.com/?api-key=9fb9e0d1-dee9-4168-b536-2a8602bd220a",
    "https://mainnet.helius-rpc.com/?api-key=168a4704-ac10-46d9-aab6-0324937381d2",
    "https://mainnet.helius-rpc.com/?api-key=3d67b97b-6217-4ecb-bce6-863c016c4ec0",
    "https://mainnet.helius-rpc.com/?api-key=21ac6a48-6783-4ef8-9c5b-f901a54085be",
    "https://mainnet.helius-rpc.com/?api-key=e2822d85-29d2-470e-93a1-538849c14e83",
    "https://mainnet.helius-rpc.com/?api-key=36b4f696-0c42-4a1f-8175-5af8b9485283",
    "https://mainnet.helius-rpc.com/?api-key=fcb0aadf-b0e1-4cb9-98c0-d7241713fc29",
    "https://mainnet.helius-rpc.com/?api-key=bf567283-d33a-4565-8116-7470fe355c83",
    "https://mainnet.helius-rpc.com/?api-key=1ebe1ad1-cdaf-4a2d-a172-bf002e54f578",
    "https://mainnet.helius-rpc.com/?api-key=12d68f68-24a9-43e8-9486-0f2484f6835d",
    "https://mainnet.helius-rpc.com/?api-key=bc0c4d92-3442-447d-9bad-87b6aa13e84c",
    "https://mainnet.helius-rpc.com/?api-key=4f55bce1-099a-4edb-aa79-c935a1b37d1b",
    "https://mainnet.helius-rpc.com/?api-key=0fdeb225-55c7-47cf-b7b0-449dd9f055f9",
    "https://mainnet.helius-rpc.com/?api-key=170824db-2f76-43a8-baf4-0a81582d91c9",
    "https://mainnet.helius-rpc.com/?api-key=5f9834af-2c39-4ded-840d-23508b0e29ec",
    "https://mainnet.helius-rpc.com/?api-key=4fab7184-869b-46a8-958b-cc31f4fd5a10",
    "https://mainnet.helius-rpc.com/?api-key=e629d11e-3185-46ef-9c48-a4b6f3e92467",
    "https://mainnet.helius-rpc.com/?api-key=1dbe4bd7-2a6c-4f61-bdab-220d5ce90466",
    "https://mainnet.helius-rpc.com/?api-key=65e7422f-1c03-4742-8006-72b17e9df7a5",
    "https://mainnet.helius-rpc.com/?api-key=91f715d8-0bc9-4bc9-af66-c44afc2082fc",
    "https://mainnet.helius-rpc.com/?api-key=d2739382-3988-44f3-a4bb-e4a9ca3262d9",
    "https://mainnet.helius-rpc.com/?api-key=c6690f9c-2018-4f5a-96be-d5457e902d5d",
    "https://mainnet.helius-rpc.com/?api-key=4bb867e3-593a-42fc-96f2-a53076042782",
    "https://mainnet.helius-rpc.com/?api-key=688501bf-af4a-4868-a68f-f0c887f35a55",
    "https://mainnet.helius-rpc.com/?api-key=fed72c66-9f50-4d52-b6c2-e6d8951be573",
    "https://mainnet.helius-rpc.com/?api-key=03697046-b1e2-4c1c-86a2-93d68517f4c8",
    "https://mainnet.helius-rpc.com/?api-key=f65e7901-7a86-4228-8a5a-4a5dd3a051af",
    "https://mainnet.helius-rpc.com/?api-key=9c2312d5-4821-455d-949d-a218ff18d87b",
    "https://mainnet.helius-rpc.com/?api-key=29af1f2a-64a8-432c-9761-30b3eb121ccd",
    "https://mainnet.helius-rpc.com/?api-key=1038034e-996c-426f-a8ae-e3f775e5bcee",
    "https://mainnet.helius-rpc.com/?api-key=e89a658f-48b5-4536-8473-fb19a0fb35d2",
    "https://mainnet.helius-rpc.com/?api-key=2793bb5a-964c-4287-96e5-a173fe8d2cc9",
    "https://mainnet.helius-rpc.com/?api-key=7e79ec2e-c5f7-4ff9-83ea-a5904f63b058",
    "https://mainnet.helius-rpc.com/?api-key=ccb32c14-3267-4ba2-8ff7-d5ee26b28c15",
    "https://mainnet.helius-rpc.com/?api-key=469561f7-7a6a-4d91-9f19-63b7b743c487",
    "https://mainnet.helius-rpc.com/?api-key=260fd6c4-3180-4b74-881b-491e71c26c84",
    "https://mainnet.helius-rpc.com/?api-key=cf3c6e19-2fda-4005-857a-26a2edb673b9",
    "https://mainnet.helius-rpc.com/?api-key=9dcb406d-be5e-48af-b4b6-0acb22765f95",
    "https://mainnet.helius-rpc.com/?api-key=eed6473c-76b8-4e09-a562-c985db0224ba",
    "https://mainnet.helius-rpc.com/?api-key=ec9c86db-35e3-4464-866b-dbae7d3d67d3",
    "https://mainnet.helius-rpc.com/?api-key=49f6a0a7-1538-4535-8d98-089fe18538a4",
    "https://mainnet.helius-rpc.com/?api-key=df6c15f9-52e5-44d2-8391-29b9a0dc101f",
    "https://mainnet.helius-rpc.com/?api-key=be288382-83a0-4a5c-b98d-595a6788f9c8",
    "https://mainnet.helius-rpc.com/?api-key=dd072657-123f-4ada-a70b-2845d7d38aa4",
    "https://mainnet.helius-rpc.com/?api-key=c43b37fa-6724-4da5-9dd2-4a2caa086579",
    "https://mainnet.helius-rpc.com/?api-key=6ac8cd24-d62d-4712-88a2-18ab7e5fe0b4",
    "https://mainnet.helius-rpc.com/?api-key=6a00f59c-a276-47ae-8576-824abe3dc785",
    "https://mainnet.helius-rpc.com/?api-key=1b146a53-c0ac-48d9-abeb-4ce7813939fb",
    "https://mainnet.helius-rpc.com/?api-key=81b78365-76cb-4621-8308-76f77cbbf620",
    "https://mainnet.helius-rpc.com/?api-key=5690a638-d717-43f1-8048-efb3e85f578c",
    "https://mainnet.helius-rpc.com/?api-key=789bb7ce-9494-40d4-ac46-954318ca9d94",
    "https://mainnet.helius-rpc.com/?api-key=3e3678b8-2b3c-4a21-b7c4-66277c5ce878",
    "https://mainnet.helius-rpc.com/?api-key=8de1015f-ce10-4343-9d98-53d5c9865468",
    "https://mainnet.helius-rpc.com/?api-key=773fa3e0-4b5d-4fed-9262-7c09384bbe72",
    "https://mainnet.helius-rpc.com/?api-key=57b3dc0c-8a77-4437-a8ef-75fdf8f9fa05",
    "https://mainnet.helius-rpc.com/?api-key=a025ae50-6c2e-48fe-bed9-44739b4cf5bc",
    "https://mainnet.helius-rpc.com/?api-key=afc007a8-25ae-45b4-8aa1-010c3a7d791e",
    "https://mainnet.helius-rpc.com/?api-key=8251ce61-1299-453a-b1b6-f137062a5056",
    "https://mainnet.helius-rpc.com/?api-key=ca5da5ea-4b55-4608-89e9-430838d74e46",
    "https://mainnet.helius-rpc.com/?api-key=7f866d92-21bd-446d-adb9-650654166f6e",
    "https://mainnet.helius-rpc.com/?api-key=fd8a6c09-6830-4ce5-952c-fe8fc158b502",
    "https://mainnet.helius-rpc.com/?api-key=e0a50af8-989a-44b3-8243-3cad237e25c1",
    "https://mainnet.helius-rpc.com/?api-key=5be3c175-d835-475a-b28d-582fe2320bfe",
    "https://mainnet.helius-rpc.com/?api-key=ada0f2c1-f7d4-4a27-898c-262b7c1d0009",
    "https://mainnet.helius-rpc.com/?api-key=fa083a86-87e4-4803-8e18-d91da3b4e9a0",
    "https://mainnet.helius-rpc.com/?api-key=a88a6215-baba-4edd-a835-cd6b9103193a",
    "https://mainnet.helius-rpc.com/?api-key=1115abb7-9f0d-498b-93f7-3916392f36d5",
    "https://mainnet.helius-rpc.com/?api-key=edb42d0b-6f58-4e23-9551-d7ed48803071",
    "https://mainnet.helius-rpc.com/?api-key=25549313-80c9-4208-ab17-a35c48aa9210",
    "https://mainnet.helius-rpc.com/?api-key=f9c0c509-5193-429e-8c47-0cf481d07abf",
    "https://mainnet.helius-rpc.com/?api-key=3332539c-7185-4814-af85-d92dc3531056",
    "https://mainnet.helius-rpc.com/?api-key=b3f6323f-c594-4a10-a2db-c31b4314b249",
    "https://mainnet.helius-rpc.com/?api-key=642eaa98-2e90-44d3-8888-6a24b3eacb3a",
    "https://mainnet.helius-rpc.com/?api-key=38846514-8f82-4460-a6b5-6ea20da9359b",
    "https://mainnet.helius-rpc.com/?api-key=7587b1c9-a6ad-4665-9520-7fcbb93e189d",
    "https://mainnet.helius-rpc.com/?api-key=627e18f6-a24e-412c-b9be-61fe1d3046b5",
    "https://mainnet.helius-rpc.com/?api-key=6b707a03-fd34-4990-8f2b-36a5604ea3bb",
    "https://mainnet.helius-rpc.com/?api-key=a12241cf-9400-45fe-b22d-4939001c1d0a",
    "https://mainnet.helius-rpc.com/?api-key=bb067504-bbcd-4e55-bdb1-6fab17eddc70",
    "https://mainnet.helius-rpc.com/?api-key=8e9ea85f-c3d8-419b-9156-1acac4b305e7",
    "https://mainnet.helius-rpc.com/?api-key=955dcbd9-734f-40a4-99e6-9c633143cd36",
    "https://mainnet.helius-rpc.com/?api-key=3cc6cf18-b318-4c95-864f-2bd33dc5a01a",
    "https://mainnet.helius-rpc.com/?api-key=52d77f49-8b7d-4371-8e28-f3f8c4cc936e",
    "https://mainnet.helius-rpc.com/?api-key=6995838a-55bd-4afd-8bf4-1e3aba3e5d0c",
    "https://mainnet.helius-rpc.com/?api-key=8c23ff5a-4591-4a2b-83e5-41098b0a6ca6",
    "https://mainnet.helius-rpc.com/?api-key=8c193cc4-ea9a-4269-84eb-fd981cc01823",
    "https://mainnet.helius-rpc.com/?api-key=e8635ee7-9f2b-4d54-824b-70ecef849fdc",
    "https://mainnet.helius-rpc.com/?api-key=e130fa85-f251-4be5-8a93-359046cf89d4",
    "https://mainnet.helius-rpc.com/?api-key=f93527d6-4e3d-42f0-8d1d-45e504cb27c2",
    "https://mainnet.helius-rpc.com/?api-key=22b01c37-72d3-481e-94e5-b018206c6eac",
    "https://mainnet.helius-rpc.com/?api-key=a6204f81-2fc5-4e03-b238-3ba84e57cfd5",
    "https://mainnet.helius-rpc.com/?api-key=f8fd8595-9409-45f5-a1ee-59999ba9b7fd",
    "https://mainnet.helius-rpc.com/?api-key=2cf0ba49-7a74-420b-bb49-8eb11fb75c59",
    "https://mainnet.helius-rpc.com/?api-key=a0edc78f-3705-48db-ab4a-f1448c696556",
    "https://mainnet.helius-rpc.com/?api-key=3e30090e-4f28-4000-a8b5-157082ff02f2",
    "https://mainnet.helius-rpc.com/?api-key=d8c9cced-d586-4b65-adeb-5914d9c53378",
    "https://mainnet.helius-rpc.com/?api-key=47e4616a-5df0-43fe-9de8-17675a98ddaf",
    "https://mainnet.helius-rpc.com/?api-key=8fa89698-2eef-4d38-b7d8-090ea6bf3f9f",
    "https://mainnet.helius-rpc.com/?api-key=c093fae3-75e6-42db-9296-ed0ec3a1e0a5",
    "https://mainnet.helius-rpc.com/?api-key=e587f2ab-1486-4cf6-bc04-a3e5f22e3294",
    "https://mainnet.helius-rpc.com/?api-key=dea33ece-f119-49c2-983b-32067376d110",
    "https://mainnet.helius-rpc.com/?api-key=2032e1a3-9e74-4fb6-8914-dfb78eb9fbd1",
    "https://mainnet.helius-rpc.com/?api-key=23a49412-c858-4b45-968d-d1c6a93c9e87",
    "https://mainnet.helius-rpc.com/?api-key=9bf2a9da-43a5-4a20-8133-c547beb0a120",
    "https://mainnet.helius-rpc.com/?api-key=21c85f4f-ce83-4f41-8f90-dadf6cd6300f",
    "https://mainnet.helius-rpc.com/?api-key=7ae29825-7d77-445c-8a9c-775098f902b3",
    "https://mainnet.helius-rpc.com/?api-key=81503c7b-948e-4dac-82ac-0b03e596093e",
    "https://mainnet.helius-rpc.com/?api-key=81fb0b8b-ed64-4560-ae62-0ed8dd76247b",
    "https://mainnet.helius-rpc.com/?api-key=5451b8f6-46cc-40a6-ad7b-530ecddeae42",
    "https://mainnet.helius-rpc.com/?api-key=c997c2c9-3100-4457-b9e3-04164f296f0f",
    "https://mainnet.helius-rpc.com/?api-key=40a4d806-d191-4335-9f09-a60e2e9d4f24",
    "https://mainnet.helius-rpc.com/?api-key=27da2907-0908-4564-b8e1-60df4c444061",
    "https://mainnet.helius-rpc.com/?api-key=6edec6b8-aee9-4976-a4f4-5e60411a1b82",
    "https://mainnet.helius-rpc.com/?api-key=796872b9-5354-453f-b438-31eb92a73e6f",
    "https://mainnet.helius-rpc.com/?api-key=4c61f2cc-a257-47ac-9eab-22fb14f356fa",
    "https://mainnet.helius-rpc.com/?api-key=59000001-e29f-4668-89ee-4e9b73fec094",
    "https://mainnet.helius-rpc.com/?api-key=0c449991-4cae-4da8-a680-8d8ad6208b06",
    "https://mainnet.helius-rpc.com/?api-key=a3d83b08-bfca-4e9a-b43c-4f952cc41372",
    "https://mainnet.helius-rpc.com/?api-key=6a706e2f-6e1a-43a3-8c37-5c3e9a20ed7f",
    "https://mainnet.helius-rpc.com/?api-key=1474e8dd-ff95-4d6e-9baf-042652761cf1",
    "https://mainnet.helius-rpc.com/?api-key=1e194e2d-2de3-4323-8a47-9bdb39ed050d",
    "https://mainnet.helius-rpc.com/?api-key=2ef723ba-c251-4b4c-a424-58fa26f617a3",
    "https://mainnet.helius-rpc.com/?api-key=a2ecdf56-7e2f-4b2c-a77e-3f957e89d130",
    "https://mainnet.helius-rpc.com/?api-key=5febd4e3-9e9c-40ad-adf3-972b20b9a8ed",
    "https://mainnet.helius-rpc.com/?api-key=cc5d4397-c288-4625-828b-bb3fbc98cf57",
    "https://mainnet.helius-rpc.com/?api-key=46404429-88a1-47fa-9b5e-9274173c4901",
    "https://mainnet.helius-rpc.com/?api-key=4ae14ea3-5e7e-4455-bc61-f5c3766b2970",
    "https://mainnet.helius-rpc.com/?api-key=121a3735-cc41-477f-8c8e-45cd1f61c035",
    "https://mainnet.helius-rpc.com/?api-key=1fd6d74c-10de-4d29-b27b-0ed15ff0373b",
    "https://mainnet.helius-rpc.com/?api-key=099df3d2-c276-4f87-92b2-7108ebf5a952",
    "https://mainnet.helius-rpc.com/?api-key=2fe604e1-2ea6-44b6-b74e-d054ea0d56dc",
    "https://mainnet.helius-rpc.com/?api-key=d31cc785-bdbb-4652-a544-020613d8f53e",
    "https://mainnet.helius-rpc.com/?api-key=d96d90e1-7aab-4d36-b463-b8f7c72f71bc",
    "https://mainnet.helius-rpc.com/?api-key=d326e11c-ac24-453d-b4d3-340bd90569a2",
    "https://mainnet.helius-rpc.com/?api-key=94d0901f-c9b6-48f4-b35b-c8898308612c",
    "https://mainnet.helius-rpc.com/?api-key=110cd19b-3c77-4dc2-9dff-442c365250b8",
    "https://mainnet.helius-rpc.com/?api-key=1d0a44de-5ffd-44ca-9302-fec95c71c662",
    "https://mainnet.helius-rpc.com/?api-key=8ce6b87d-fe0a-47ca-8720-21cb49312cb0",
    "https://mainnet.helius-rpc.com/?api-key=b6e4aa6e-1a4e-4f94-83f0-65f671e65eb6",
    "https://mainnet.helius-rpc.com/?api-key=5fc6691c-fcb2-446c-8a1f-58d25e21f0d3",
    "https://mainnet.helius-rpc.com/?api-key=411597a6-3f97-4b2d-8125-40171d1202e2",
    "https://mainnet.helius-rpc.com/?api-key=cddb29b3-1536-45f6-b128-e7ddeaf91c95",
    "https://mainnet.helius-rpc.com/?api-key=1012ea12-40b0-44cf-8d29-2acc82cb9020",
    "https://mainnet.helius-rpc.com/?api-key=6effa890-b039-4233-92a6-ec4ec9d0322d",
    "https://mainnet.helius-rpc.com/?api-key=367cd29c-1852-4599-8e3f-508d7c7e811f",
    "https://mainnet.helius-rpc.com/?api-key=b2b53e93-82b8-4b44-b2d4-697b683c5421",
    "https://mainnet.helius-rpc.com/?api-key=9c70b59b-ef27-4858-876f-ca064c704696",
    "https://mainnet.helius-rpc.com/?api-key=8aea1568-0b92-49f7-9707-2d41afd31142",
    "https://mainnet.helius-rpc.com/?api-key=6e31009c-1510-4675-b6d0-9f82cb3c519e",
    "https://mainnet.helius-rpc.com/?api-key=bbd788ca-02e6-4d58-807f-3a1d0a95b158",
    "https://mainnet.helius-rpc.com/?api-key=93140fce-7a1a-4146-b1af-3d20b6ae8688",
    "https://mainnet.helius-rpc.com/?api-key=60777f59-64de-4c04-a6ed-3c985daab0ff",
    "https://mainnet.helius-rpc.com/?api-key=b10bc338-0887-46f1-ad64-771e754f2d00",
    "https://mainnet.helius-rpc.com/?api-key=c4aa918d-9fa0-4775-a54e-f5188e3199ca",
    "https://mainnet.helius-rpc.com/?api-key=6ac39c5d-69c1-47d1-a969-5ce04a2b5afb",
    "https://mainnet.helius-rpc.com/?api-key=f220c8cc-6f6a-41fa-91c6-9186380c8936",
    "https://mainnet.helius-rpc.com/?api-key=bf20b70a-d0fe-42ce-a3b8-7d40e23cfbe2",
    "https://mainnet.helius-rpc.com/?api-key=54520fd7-683d-4a1e-964c-156016cc4cb7",
    "https://mainnet.helius-rpc.com/?api-key=3c9ae231-536e-4219-8df3-b7a374066bbe",
    "https://mainnet.helius-rpc.com/?api-key=03bc8e91-e46b-4737-b11a-94adc9d0cf19",
    "https://mainnet.helius-rpc.com/?api-key=2586f78c-bd01-4ef7-a62e-9afc3802d348",
    "https://mainnet.helius-rpc.com/?api-key=ef92bb3b-9aa5-4138-9731-d0b77fc405da",
    "https://mainnet.helius-rpc.com/?api-key=946ee4ef-efa8-4d25-97a8-c76606132343",
    "https://mainnet.helius-rpc.com/?api-key=2218a3be-c3c5-4e24-a36a-db109f0b9a06",
    "https://mainnet.helius-rpc.com/?api-key=b618e623-a57a-4944-83d1-1b991cb253c8",
    "https://mainnet.helius-rpc.com/?api-key=454a3c2e-ffd9-4a2d-adef-08c36ee6b625",
    "https://mainnet.helius-rpc.com/?api-key=d50252de-c0d4-4a93-b327-60968c86f5a9",
    "https://mainnet.helius-rpc.com/?api-key=135ec4d6-934a-4843-94a6-5d44138037be",
    "https://mainnet.helius-rpc.com/?api-key=27ff414d-60eb-4ba3-a10b-ee760b71b243",
    "https://mainnet.helius-rpc.com/?api-key=d133f6d7-9f69-45b3-933e-d642eea8fda4",
    "https://mainnet.helius-rpc.com/?api-key=fff569a8-3491-4c04-8402-48265e722db4",
    "https://mainnet.helius-rpc.com/?api-key=d705ba72-ee7a-4395-bc54-ac9dc0c7ed71",
    "https://mainnet.helius-rpc.com/?api-key=b30961bb-bc2a-4cf3-a625-a581cfa0bfc0",
    "https://mainnet.helius-rpc.com/?api-key=6a1f7dff-2ac8-42a0-bc42-defca58ed56c",
    "https://mainnet.helius-rpc.com/?api-key=202309e7-9051-46f3-94a6-bf61577448d3",
    "https://mainnet.helius-rpc.com/?api-key=6adce490-fe98-43b5-aeb8-4845c7bb6bd8",
    "https://mainnet.helius-rpc.com/?api-key=6a63d6a7-f6f7-4f4b-9155-be2ecc753f0a",
    "https://mainnet.helius-rpc.com/?api-key=3606d836-3a8c-4caa-9555-e033386a2476",
    "https://mainnet.helius-rpc.com/?api-key=86482600-13a8-4fd5-b055-42e703011c9d",
    "https://mainnet.helius-rpc.com/?api-key=e13f91ba-c608-4a80-8bb9-4daf63ae4dfb",
    "https://mainnet.helius-rpc.com/?api-key=1fcbb4dd-a2e2-4867-96b3-1fd458112b7f",
    "https://mainnet.helius-rpc.com/?api-key=8f24ad41-d07b-4321-bc6d-25054f5cd395",
    "https://mainnet.helius-rpc.com/?api-key=4cd08b4f-9108-4cbd-9a5d-d49f9006ccb8",
    "https://mainnet.helius-rpc.com/?api-key=f83f7d15-953d-4f1b-bfc8-763f0ca4451b",
    "https://mainnet.helius-rpc.com/?api-key=5fba49bf-68da-498c-9913-341e83555801",
    "https://mainnet.helius-rpc.com/?api-key=a2ab304b-b532-4b3b-b77b-b4c6f0abf594",
    "https://mainnet.helius-rpc.com/?api-key=da8edf00-aeaf-4337-8ecd-037f13834d8f",
    "https://mainnet.helius-rpc.com/?api-key=758314f9-645a-4537-be3c-52da673ffdc1",
    "https://mainnet.helius-rpc.com/?api-key=87c4647f-9042-4ede-a4de-5a2f6b0796d1",
    "https://mainnet.helius-rpc.com/?api-key=686c9168-0314-41a4-805c-f5ad155c5e3a",
    "https://mainnet.helius-rpc.com/?api-key=d71c782c-7d36-4ff6-9b70-07ce0abd5147",
    "https://mainnet.helius-rpc.com/?api-key=31e59077-faf4-4c26-9c51-ba8cf97b2c6b",
    "https://mainnet.helius-rpc.com/?api-key=49a3b0ae-3dad-4da2-b683-fcaddb89c60a",
    "https://mainnet.helius-rpc.com/?api-key=131f6a02-90b9-44ad-be4e-7868063569e3",
    "https://mainnet.helius-rpc.com/?api-key=4a938814-ba45-4426-8676-b7dba1958609",
    "https://mainnet.helius-rpc.com/?api-key=6635d867-42c1-4932-9107-345062a6b0eb",
    "https://mainnet.helius-rpc.com/?api-key=a2a2ca1d-5bb7-4abc-b536-780ff0f42930",
    "https://mainnet.helius-rpc.com/?api-key=18caa4c8-55e4-47dd-8373-0a1b64cf02cd",
    "https://mainnet.helius-rpc.com/?api-key=6ecf3a38-a926-431d-a2b3-e1f99ac48be0",
    "https://mainnet.helius-rpc.com/?api-key=33dbdb7f-3825-4341-9c00-03e87692dd5e",
    "https://mainnet.helius-rpc.com/?api-key=d0bd9b9a-3adc-4660-8706-90b02dc5495d",
    "https://mainnet.helius-rpc.com/?api-key=3bc47b9d-3265-484c-8980-ac584ea006a8",
    "https://mainnet.helius-rpc.com/?api-key=a332eb8e-89dc-48f8-bdef-cabc1b424330",
    "https://mainnet.helius-rpc.com/?api-key=ebdba72a-8370-4165-b2eb-d3350d6c7a28",
    "https://mainnet.helius-rpc.com/?api-key=3825c61f-47da-46bb-8330-d64fa36d8625",
    "https://mainnet.helius-rpc.com/?api-key=b3042d96-12ae-47d0-8dfa-c9c0475cf6cc",
    "https://mainnet.helius-rpc.com/?api-key=edb865f5-ce94-4aef-b44b-66c3bdfb3997",
    "https://mainnet.helius-rpc.com/?api-key=d048d86c-6c35-48cc-97f6-2aa923ca3004",
    "https://mainnet.helius-rpc.com/?api-key=59fe02f8-e707-4470-9ba6-f68f44b87df2",
    "https://mainnet.helius-rpc.com/?api-key=d2283a04-85eb-471e-8228-f0284a9e5394",
    "https://mainnet.helius-rpc.com/?api-key=79a796f6-c379-4538-b05f-bf6803889b4c",
    "https://mainnet.helius-rpc.com/?api-key=ed9fdb09-05cd-4b5f-b91b-6c0739e3b833",
    "https://mainnet.helius-rpc.com/?api-key=489676a2-2466-479c-b919-1815701adbca",
    "https://mainnet.helius-rpc.com/?api-key=a0c8a805-ab9c-41c2-a3ff-c7ad16611584",
    "https://mainnet.helius-rpc.com/?api-key=a68a335d-48d7-4969-bf7d-486f5e858c08",
    "https://mainnet.helius-rpc.com/?api-key=8c7efe9c-deb3-4329-a41c-66a0ce0de17d",
    "https://mainnet.helius-rpc.com/?api-key=fd06eb35-57f3-470c-b186-4d700b22738d",
    "https://mainnet.helius-rpc.com/?api-key=74069adf-83a4-49d7-8b82-f1e7fe264173",
    "https://mainnet.helius-rpc.com/?api-key=b94e39ad-cd07-49e4-bcdd-c84f0824075c",
    "https://mainnet.helius-rpc.com/?api-key=4548d8fd-9d0c-4156-91c2-24ff8a3d5553",
    "https://mainnet.helius-rpc.com/?api-key=2d4c69ef-2b02-433f-b84e-7d33dd0d8458",
    "https://mainnet.helius-rpc.com/?api-key=eda451d7-f06b-4e91-90e0-5a2391f78821",
    "https://mainnet.helius-rpc.com/?api-key=8ceb28d5-3f6a-43c0-8cc5-016a022b65bc",
    "https://mainnet.helius-rpc.com/?api-key=03a59214-43ca-4e1e-bdf5-773939927861",
    "https://mainnet.helius-rpc.com/?api-key=c30374f4-bdfa-4d13-bfea-f7caf9053f40",
    "https://mainnet.helius-rpc.com/?api-key=259cc8a7-2ab9-4f6c-97d1-c6a1fb28a008",
    "https://mainnet.helius-rpc.com/?api-key=48b71a52-fe43-4bd0-938d-67bdeaf713b3",
    "https://mainnet.helius-rpc.com/?api-key=677c99d5-bb11-4adc-bf76-39fda3844f16",
    "https://mainnet.helius-rpc.com/?api-key=6ed63c2d-a8d8-427f-8885-7c68c497d2a5",
    "https://mainnet.helius-rpc.com/?api-key=7608436c-a7d3-4cc7-bce7-c6cd152d8052",
    "https://mainnet.helius-rpc.com/?api-key=e52bfa4c-b657-48ea-a1ba-8731e976240e",
    "https://mainnet.helius-rpc.com/?api-key=ddc502d8-c223-4acb-9be2-b974591ec202",
    "https://mainnet.helius-rpc.com/?api-key=c96521a1-6895-47f5-a572-d3aeff98e932",
    "https://mainnet.helius-rpc.com/?api-key=3f3b7c83-8b5d-44a9-8699-b586ba1cc194",
    "https://mainnet.helius-rpc.com/?api-key=8bc3d134-ae1d-4253-bb27-a8edbfd0c1d5",
    "https://mainnet.helius-rpc.com/?api-key=066b3e78-68b9-4b54-87d0-a631bac01bdf",
    "https://mainnet.helius-rpc.com/?api-key=e9b560c3-5c3b-4e52-8e91-36ea9bed5bb0",
    "https://mainnet.helius-rpc.com/?api-key=786171ee-11fa-4d9e-a7b6-2764b8132c07",
    "https://mainnet.helius-rpc.com/?api-key=026a0100-0b4a-45c3-909c-ea3c5e6f8b5e",
    "https://mainnet.helius-rpc.com/?api-key=d874e534-ab30-47b3-8549-045da1429980",
    "https://mainnet.helius-rpc.com/?api-key=9c78511a-d497-48b2-b5b7-e6f9cde24a5e",
    "https://mainnet.helius-rpc.com/?api-key=de1abc57-c658-4161-a58c-3e1e709c4df9",
    "https://mainnet.helius-rpc.com/?api-key=7f435ed5-d5c7-4021-ac7f-b30558dbb800",
    "https://mainnet.helius-rpc.com/?api-key=060508bb-212a-4179-ba87-26fc73a109af",
    "https://mainnet.helius-rpc.com/?api-key=39821d55-8a7d-4b07-adcd-49bcee98c940",
    "https://mainnet.helius-rpc.com/?api-key=03259198-5f2b-410e-b451-ef4826951059",
    "https://mainnet.helius-rpc.com/?api-key=089a980d-ea8f-45ce-9865-bc1d9180a766",
    "https://mainnet.helius-rpc.com/?api-key=64e38153-aa0b-4207-9773-3b66f63a7cc9",
    "https://mainnet.helius-rpc.com/?api-key=c17d1a48-05e2-4831-8052-a21fd042ecf8",
    "https://mainnet.helius-rpc.com/?api-key=876e2ffe-8051-4073-bc37-68b81af1fa59",
    "https://mainnet.helius-rpc.com/?api-key=fc8ffbcf-6ddf-4047-8762-fe3ac98c4c7b",
    "https://mainnet.helius-rpc.com/?api-key=11f88c8c-b89f-48a6-891b-9c697018003b",
    "https://mainnet.helius-rpc.com/?api-key=3dbbddb7-fc22-40f5-bd4c-242b29a5ff72",
    "https://mainnet.helius-rpc.com/?api-key=487e2cc0-2a85-4540-b20b-12fb5e46c815",
    "https://mainnet.helius-rpc.com/?api-key=70c3924e-3cdc-4772-adbf-326bc640a59a",
    "https://mainnet.helius-rpc.com/?api-key=7414663b-3735-427e-a8a4-188f544b0911",
    "https://mainnet.helius-rpc.com/?api-key=da012bbf-2639-4a49-b45c-0330009ccd61",
    "https://mainnet.helius-rpc.com/?api-key=be710e9e-3c7e-4763-93c5-0667057b3f19",
    "https://mainnet.helius-rpc.com/?api-key=3b7e26b5-a32a-4b6a-9846-abcd03c280cb",
    "https://mainnet.helius-rpc.com/?api-key=c445c294-305c-4e59-b838-c0c3e6de4e17",
    "https://mainnet.helius-rpc.com/?api-key=69bee3ad-c1d7-4158-9ced-1dd138ea3e3b",
    "https://mainnet.helius-rpc.com/?api-key=675a6c1d-4866-4827-8e8e-c0b97b51f3e4",
    "https://mainnet.helius-rpc.com/?api-key=ca92db37-728a-4af9-ad0d-526012b4e300",
    "https://mainnet.helius-rpc.com/?api-key=d6be4eda-c275-4390-a2f5-ffc600b61a0f",
    "https://mainnet.helius-rpc.com/?api-key=283e0efc-dd3c-4160-b082-ab60bdb3df8a",
    "https://mainnet.helius-rpc.com/?api-key=61e84b8f-dd7c-4f80-9f04-7348ba99f9f1",
    "https://mainnet.helius-rpc.com/?api-key=9f66eaa3-b2c2-4d36-bbbb-8a175c21582b",
    "https://mainnet.helius-rpc.com/?api-key=706b6b70-d0a0-4cee-9322-55745a70f95f",
    "https://mainnet.helius-rpc.com/?api-key=eaf1a085-8bab-4707-988e-57451f56d443",
    "https://mainnet.helius-rpc.com/?api-key=7a5679d0-6b9b-4b4d-822a-b8257d4a365a",
    "https://mainnet.helius-rpc.com/?api-key=ab98f898-2cb6-4179-9e7c-42a6fc04dcbe",
    "https://mainnet.helius-rpc.com/?api-key=9d228a0d-62dd-44b3-b378-1b5d3bcccf95",
    "https://mainnet.helius-rpc.com/?api-key=30fe20a6-eb89-4df2-9a41-90b31834c81f",
    "https://mainnet.helius-rpc.com/?api-key=68d51970-ae96-4a62-92e6-9cd3f17218d6",
    "https://mainnet.helius-rpc.com/?api-key=5e4fccf4-dc69-4559-ba05-20e06581ed33",
    "https://mainnet.helius-rpc.com/?api-key=60aaeb81-a426-4a36-8dc5-2ea29d7f0a7d",
    "https://mainnet.helius-rpc.com/?api-key=92e7ff2f-b4ee-4d49-a0ea-3ed4fa19ae2b",
    "https://mainnet.helius-rpc.com/?api-key=af7eb979-6ae0-4825-a022-0aacad3bf2b1",
    "https://mainnet.helius-rpc.com/?api-key=582c7f21-d70d-46bc-a3ae-b761c0f4e57e",
    "https://mainnet.helius-rpc.com/?api-key=43a6e69b-7b38-475a-b134-eb1c1edf4285",
    "https://mainnet.helius-rpc.com/?api-key=133ea86d-ff3d-4180-8ad7-f81cf9bf1da4",
    "https://mainnet.helius-rpc.com/?api-key=5007ab9e-e074-4c7a-8cde-a9d9410ab798",
    "https://mainnet.helius-rpc.com/?api-key=cfa32e34-f681-4bac-bad1-1cc53320f535",
    "https://mainnet.helius-rpc.com/?api-key=705f57b1-e57b-40aa-82bf-53f1f0089579",
    "https://mainnet.helius-rpc.com/?api-key=5e4057d6-e096-481a-8949-02a8ed138d59",
    "https://mainnet.helius-rpc.com/?api-key=8769c000-5a57-4dfc-b682-81064246509f",
    "https://mainnet.helius-rpc.com/?api-key=f61a618b-dfd0-4893-aaaf-01f69ef9eec5",
    "https://mainnet.helius-rpc.com/?api-key=f5282f51-3782-4a19-a1b9-d085bde06b55",
    "https://mainnet.helius-rpc.com/?api-key=83f8c247-ba50-4553-b3ee-1ea0517d6cf2",
    "https://mainnet.helius-rpc.com/?api-key=60216759-44b0-4b12-8be6-1204fc486689",
    "https://mainnet.helius-rpc.com/?api-key=ba3ea3fd-f470-43c3-a34f-d0b3a6ef996a",
    "https://mainnet.helius-rpc.com/?api-key=c69a1aba-3321-45fc-8f92-bc131b354255",
    "https://mainnet.helius-rpc.com/?api-key=0c27515b-cb5f-4183-ae32-515c2279b315",
    "https://mainnet.helius-rpc.com/?api-key=e01d8d11-5aaa-4915-8c53-31dbf32acf2e",
    "https://mainnet.helius-rpc.com/?api-key=bd402434-1245-4bb5-ae55-5b8dbe6bbd5b",
    "https://mainnet.helius-rpc.com/?api-key=1a28d8a2-0644-40d2-9d95-8469cbb01c55",
    "https://mainnet.helius-rpc.com/?api-key=431a582b-b604-4b02-86e7-e6a14c87cccc",
    "https://mainnet.helius-rpc.com/?api-key=8375a914-5272-4312-8c39-cfeaf174d619",
    "https://mainnet.helius-rpc.com/?api-key=78960687-3128-44bd-b416-b28e2128298d",
    "https://mainnet.helius-rpc.com/?api-key=6975a660-c337-4a91-a44e-b5c84dd520b0",
    "https://mainnet.helius-rpc.com/?api-key=4866166e-e59c-48ae-9349-d0d1ce7f149e",
    "https://mainnet.helius-rpc.com/?api-key=0a9f755a-76e3-4ebc-b694-f38956e6ff6a",
    "https://mainnet.helius-rpc.com/?api-key=7d0107c0-d6d9-4777-a937-66922f872c83",
    "https://mainnet.helius-rpc.com/?api-key=718e279c-255e-4453-8935-7fd7718a8ccc",
    "https://mainnet.helius-rpc.com/?api-key=2d2954d3-f4c5-400d-ab78-4010f2a822ad",
    "https://mainnet.helius-rpc.com/?api-key=b9dfbe91-7210-46d9-b72e-faa984331eef",
    "https://mainnet.helius-rpc.com/?api-key=71e2b5b8-5ac0-4e29-8a90-0ce83d854aa3",
    "https://mainnet.helius-rpc.com/?api-key=b83f1ad4-07e6-4941-8896-34127e0c6dd5",
    "https://mainnet.helius-rpc.com/?api-key=3f3a42bc-94c6-4cbc-adc1-5a9f0594d14b",
    "https://mainnet.helius-rpc.com/?api-key=5c997afa-b399-4352-8d74-88bded47b626",
    "https://mainnet.helius-rpc.com/?api-key=31ffb944-c5ac-4936-b7b3-d27f164ae47d",
    "https://mainnet.helius-rpc.com/?api-key=13bc640b-7ea3-46be-b3c1-349868cdb25e",
    "https://mainnet.helius-rpc.com/?api-key=20259325-1f73-4adb-ae08-d1c795b622b8",
    "https://mainnet.helius-rpc.com/?api-key=334907c3-2166-4da7-a729-204b57a612e4",
    "https://mainnet.helius-rpc.com/?api-key=3fa3ee7b-85b0-4d57-b77e-fcfdbdf88c11",
    "https://mainnet.helius-rpc.com/?api-key=99a51b81-bdb2-4bee-a9ef-ef8cd8f55ecb",
    "https://mainnet.helius-rpc.com/?api-key=3a05ad5f-aa1b-4b87-b321-560681d0d3ef",
    "https://mainnet.helius-rpc.com/?api-key=b93203d0-39d2-42c0-9c59-3d81cd3ee7b1",
    "https://mainnet.helius-rpc.com/?api-key=2484ace3-3a76-47a2-81cd-e1308ab1ee00",
    "https://mainnet.helius-rpc.com/?api-key=d3ee546f-d876-4466-b15c-f5c9eb738333",
    "https://mainnet.helius-rpc.com/?api-key=be454a1b-b545-4067-bc20-d77340fe108f",
    "https://mainnet.helius-rpc.com/?api-key=b876d8b2-b4db-4b78-8bf9-5f5900ee18c4",
    "https://mainnet.helius-rpc.com/?api-key=3add6133-2c3b-433d-8195-e88bbf98666b",
    "https://mainnet.helius-rpc.com/?api-key=f8b526e2-afe5-4b67-9db0-a95d4878ec46",
    "https://mainnet.helius-rpc.com/?api-key=a2f34f6b-0abb-4627-b4e6-d853ecf7ff95",
    "https://mainnet.helius-rpc.com/?api-key=88c8d27e-4bf5-4f83-ae9c-bce8c091e3cc",
    "https://mainnet.helius-rpc.com/?api-key=530e1276-1702-4adb-8233-2057c8b7ba54",
    "https://mainnet.helius-rpc.com/?api-key=e1cfbe93-3b49-4f50-9a57-d87c2c15bc4e",
    "https://mainnet.helius-rpc.com/?api-key=d0b98544-bd05-482c-9897-a4da63122431",
    "https://mainnet.helius-rpc.com/?api-key=4c2163c6-b6f5-454d-8ad9-ea50252b8de0",
    "https://mainnet.helius-rpc.com/?api-key=801de64f-e9d0-4f05-9f61-f921ee112a00",
    "https://mainnet.helius-rpc.com/?api-key=455e8d4a-7293-4661-81d5-86939eeb98fc",
    "https://mainnet.helius-rpc.com/?api-key=fdb67f57-37d4-4bb2-9562-57814076d27f",
    "https://mainnet.helius-rpc.com/?api-key=b841a9cf-2870-4abb-a69e-a8c31b5c904c",
    "https://mainnet.helius-rpc.com/?api-key=983dbb27-84af-46ce-9528-00f67dd709b3",
    "https://mainnet.helius-rpc.com/?api-key=b83dd5dc-bf90-4e6d-97c9-5585f401a8a7",
    "https://mainnet.helius-rpc.com/?api-key=7dc03911-9749-49a3-82fa-b9af416dedb4",
    "https://mainnet.helius-rpc.com/?api-key=192c31c0-fc99-45e8-9bab-c23765d43409",
    "https://mainnet.helius-rpc.com/?api-key=cfef4095-35c5-4d04-9479-efeffec9c34e",
    "https://mainnet.helius-rpc.com/?api-key=34aa1aed-47b4-417a-94b8-fa65d7611200",
    "https://mainnet.helius-rpc.com/?api-key=4eab5f62-6b28-4272-afcc-d84319b5d665",
    "https://mainnet.helius-rpc.com/?api-key=f43f4efa-39af-45b1-83be-e057c27dbc2a",
    "https://mainnet.helius-rpc.com/?api-key=47bc2c23-de57-4dfe-a9bd-10a0a6d424bd",
    "https://mainnet.helius-rpc.com/?api-key=c8eb1ad2-03ab-4fbf-bb41-a999bff5334e",
    "https://mainnet.helius-rpc.com/?api-key=969fa75b-8211-4c0c-bee2-b70c73eea182",
    "https://mainnet.helius-rpc.com/?api-key=39515ecc-36eb-4ec0-ab14-41de4c857701",
    "https://mainnet.helius-rpc.com/?api-key=144c8c8c-7158-4543-a5e5-2491f8b83f55",
    "https://mainnet.helius-rpc.com/?api-key=f3bc3b44-53ec-413d-adf1-1f2a0fbc676e",
    "https://mainnet.helius-rpc.com/?api-key=13d816ef-d83d-45c0-a907-6fdeb60e4513",
    "https://mainnet.helius-rpc.com/?api-key=4457670c-1864-4222-a441-b87e6887856b",
    "https://mainnet.helius-rpc.com/?api-key=7d2ecb29-5f04-439a-9d49-dcaf86a0c30f",
    "https://mainnet.helius-rpc.com/?api-key=6565f72e-1740-47cd-b898-720148b40b86",
    "https://mainnet.helius-rpc.com/?api-key=0d6a39a0-1d81-44b7-a8bd-364853db0034",
    "https://mainnet.helius-rpc.com/?api-key=5c8e5813-55ac-4bec-8457-89eacde5fccb",
    "https://mainnet.helius-rpc.com/?api-key=568e1810-270c-49d6-9bdb-253c9f86b488",
    "https://mainnet.helius-rpc.com/?api-key=c77a73b8-d111-4c33-9969-3971fc1d726e",
    "https://mainnet.helius-rpc.com/?api-key=21ec8588-3001-4115-b211-035e53f3a5cb",
    "https://mainnet.helius-rpc.com/?api-key=c0fb4e0a-c083-4fae-93b8-b6ae0fe94f16",
    "https://mainnet.helius-rpc.com/?api-key=e677df1f-13b3-43d2-bb53-b6ba51e5f09c",
    "https://mainnet.helius-rpc.com/?api-key=72590e1e-d411-4a27-9cb8-4e457b1daf74",
    "https://mainnet.helius-rpc.com/?api-key=1d51bcbb-9348-4e46-bfc7-b2f0240a24ba",
    "https://mainnet.helius-rpc.com/?api-key=db551617-4356-4477-9ecb-89dc652e1e85",
    "https://mainnet.helius-rpc.com/?api-key=14c1d7e6-f66e-4e71-862c-a0cd9791e007",
    "https://mainnet.helius-rpc.com/?api-key=b9eda869-ad49-4b3a-afd3-38bff9227fc1",
    "https://mainnet.helius-rpc.com/?api-key=5f1a56ee-8bba-4df7-b92f-e0cfe42700c8",
    "https://mainnet.helius-rpc.com/?api-key=08b69ecc-f9b3-4093-a3ef-5878cb237bdc",
    "https://mainnet.helius-rpc.com/?api-key=527f6dc2-c6e9-4ba2-b412-d847a2c4c0df",
    "https://mainnet.helius-rpc.com/?api-key=7e1515df-e76f-4cce-bd3f-049e55a74208",
    "https://mainnet.helius-rpc.com/?api-key=3d2e47e9-39ac-4116-bd52-af4c5d4651f4",
    "https://mainnet.helius-rpc.com/?api-key=55bb1e70-bd35-4428-9261-c0aa782edf55",
    "https://mainnet.helius-rpc.com/?api-key=69b024dd-67be-48c7-aae8-bf65419f756c",
    "https://mainnet.helius-rpc.com/?api-key=359ebc79-e893-4ef8-adf3-9597762e6ada",
    "https://mainnet.helius-rpc.com/?api-key=9c4f4bf7-095d-47c0-aba9-0b6718a112f5",
    "https://mainnet.helius-rpc.com/?api-key=24a24a4e-b0f6-4076-815a-b77fe9613721",
    "https://mainnet.helius-rpc.com/?api-key=48876868-629e-4197-bd21-c152c7f15135",
    "https://mainnet.helius-rpc.com/?api-key=68f20d12-9d05-4ee3-aa63-cfaf1a3980c3",
    "https://mainnet.helius-rpc.com/?api-key=1f1214d1-4f9a-4082-b591-6baef8c58209",
    "https://mainnet.helius-rpc.com/?api-key=aad67173-1d18-4ef3-9e36-2823b2de53a7",
    "https://mainnet.helius-rpc.com/?api-key=5d6ad3fb-7c87-4102-a463-a24dd9f4563a",
    "https://mainnet.helius-rpc.com/?api-key=8f2b222b-7e0c-458d-8158-a51a19d91921",
    "https://mainnet.helius-rpc.com/?api-key=0ac530b6-85b9-4646-9325-b9a3b1b6de67",
    "https://mainnet.helius-rpc.com/?api-key=e3e33489-22e6-4e2d-8db5-81f527205f07",
    "https://mainnet.helius-rpc.com/?api-key=55f2dede-b215-48cc-96ef-47678258667b",
    "https://mainnet.helius-rpc.com/?api-key=8abe8e61-84a0-4355-9832-2c9c291dbbd5",
    "https://mainnet.helius-rpc.com/?api-key=0610f0e3-80f7-4467-8aac-abc0fc30b9c5",
    "https://mainnet.helius-rpc.com/?api-key=3cb80030-ee29-4a49-a785-4e0592af5732",
    "https://mainnet.helius-rpc.com/?api-key=46a87ea8-762e-4e66-9fa5-0a30a77ab5d1",
    "https://mainnet.helius-rpc.com/?api-key=bd236e16-d5aa-44a9-9adc-c34330c4534c",
    "https://mainnet.helius-rpc.com/?api-key=1f0b9319-b879-43e0-a33f-e2aa39eee47a",
    "https://mainnet.helius-rpc.com/?api-key=54a7d0e6-e367-48ff-8095-73e531d2ab79",
    "https://mainnet.helius-rpc.com/?api-key=038ce518-4c8e-43da-8a92-25e8fab2c803",
    "https://mainnet.helius-rpc.com/?api-key=784f63ac-a336-4a76-8fc6-35bf5d467450",
    "https://mainnet.helius-rpc.com/?api-key=20449c42-ff8e-43cb-a432-57460236dff3",
    "https://mainnet.helius-rpc.com/?api-key=85f7594e-ca08-4a6f-b043-45b2d8c59f9d",
    "https://mainnet.helius-rpc.com/?api-key=77363663-0901-493a-a990-b7ba5d7c79b0",
    "https://mainnet.helius-rpc.com/?api-key=ebf3071e-272d-486e-b662-54b0f0b2e070",
    "https://mainnet.helius-rpc.com/?api-key=e4ba00b5-3f8a-4af5-ba17-9979ca835e73",
    "https://mainnet.helius-rpc.com/?api-key=06f38861-d555-44d9-8679-88f8e2b37afd",
    "https://mainnet.helius-rpc.com/?api-key=340331f6-fa41-402d-9d11-195887513299",
    "https://mainnet.helius-rpc.com/?api-key=20280871-dd2c-4044-8a7b-182472c8514e",
    "https://mainnet.helius-rpc.com/?api-key=d110d567-26de-4b25-8a56-d76e379641af",
    "https://mainnet.helius-rpc.com/?api-key=fe47c62f-8cff-4e4b-b696-f3098a6b911f",
    "https://mainnet.helius-rpc.com/?api-key=76263f6a-2196-4bd4-8c34-e94db86ac02e",
    "https://mainnet.helius-rpc.com/?api-key=60f4b72e-0b5b-4fbc-845e-dc262277a658",
    "https://mainnet.helius-rpc.com/?api-key=e7c1bc4a-1c66-4264-80f8-1d724231a433",
    "https://mainnet.helius-rpc.com/?api-key=7fc69d47-a078-4d03-bd76-14fc280eccc5",
    "https://mainnet.helius-rpc.com/?api-key=7070e096-2d35-4ec7-b392-c047a38e0921",
    "https://mainnet.helius-rpc.com/?api-key=0a0b9803-b70b-46c5-a05d-4d2d77d6ff9c",
    "https://mainnet.helius-rpc.com/?api-key=85be96d4-8f0d-4127-9b26-4c731394a569",
    "https://mainnet.helius-rpc.com/?api-key=3f35dcf0-4174-44f8-8e85-d99297416143",
    "https://mainnet.helius-rpc.com/?api-key=840e9e09-bc5a-40cc-b937-1f9ce37f195e",
    "https://mainnet.helius-rpc.com/?api-key=b3a8fd98-a331-4dae-8e94-5edd5c5f2c64",
    "https://mainnet.helius-rpc.com/?api-key=cdabf720-4ea4-4258-ab6b-0adaeae10f86",
    "https://mainnet.helius-rpc.com/?api-key=b97ca24c-7e35-44af-a89b-280a2d4d0f76",
    "https://mainnet.helius-rpc.com/?api-key=1aa53d12-1c0f-419f-9567-d3efe15300e7",
    "https://mainnet.helius-rpc.com/?api-key=b8b389fb-32c9-4be7-bb43-8ebba4afa584",
    "https://mainnet.helius-rpc.com/?api-key=31fb38da-8d7d-4d3e-855e-7b01bac75208",
    "https://mainnet.helius-rpc.com/?api-key=c21d613f-02e6-494b-99dd-18b96232a9e0",
    "https://mainnet.helius-rpc.com/?api-key=475af474-491c-4523-867a-0ef577af1e27",
    "https://mainnet.helius-rpc.com/?api-key=39bb7817-eb52-4c60-aa8b-7f04d09f23c9",
    "https://mainnet.helius-rpc.com/?api-key=5f927578-674b-4cef-ade2-eac73c7ab02c",
    "https://mainnet.helius-rpc.com/?api-key=b4f7d23d-aff3-4c74-8a67-8c8c50bfc3fd",
    "https://mainnet.helius-rpc.com/?api-key=9d9f02ef-7cde-4cc5-beac-c4756772d177",
    "https://mainnet.helius-rpc.com/?api-key=81897f5a-befd-4afb-abd6-31169aabb51b",
    "https://mainnet.helius-rpc.com/?api-key=fb98549e-510b-432c-8e78-7fecde4b3f09",
    "https://mainnet.helius-rpc.com/?api-key=7ba570b2-54c9-445b-8f6d-c5d0ef7378db",
    "https://mainnet.helius-rpc.com/?api-key=a51ec3e9-fefa-4c6e-b9af-a703206cbac4",
    "https://mainnet.helius-rpc.com/?api-key=22fafd57-a92f-4934-8931-b6af2ef0ffdb",
    "https://mainnet.helius-rpc.com/?api-key=040b24ef-b64e-4a85-9ae0-8aab4609236d",
    "https://mainnet.helius-rpc.com/?api-key=c43059f9-a8cd-4d33-abe7-e7818796b655",
    "https://mainnet.helius-rpc.com/?api-key=beeac38f-7c34-43be-9955-26d4f9a6b696",
    "https://mainnet.helius-rpc.com/?api-key=269657a8-f6f9-478d-bdea-b10f84adc824",
    "https://mainnet.helius-rpc.com/?api-key=a69bcf17-e304-422d-be10-953c44d4a7ac",
    "https://mainnet.helius-rpc.com/?api-key=dde622cd-b990-4985-bbce-64fc24d87008",
    "https://mainnet.helius-rpc.com/?api-key=ed6525ae-c514-4828-938c-2a26e5784c0e",
    "https://mainnet.helius-rpc.com/?api-key=5212e178-87c4-46d4-b55f-feee76b98e51",
    "https://mainnet.helius-rpc.com/?api-key=e9bd8bfa-34a1-49de-9cd7-201a8e96f33c",
    "https://mainnet.helius-rpc.com/?api-key=da6c58c1-db6a-4415-badf-f5ca32af8260",
    "https://mainnet.helius-rpc.com/?api-key=9ceb9da5-ff3e-4403-8502-e186a9ac0e26",
    "https://mainnet.helius-rpc.com/?api-key=95368b45-2396-4e29-a287-ccb18c47d33a",
    "https://mainnet.helius-rpc.com/?api-key=a054a0c1-e8bc-4260-bab3-4775286a351a",
    "https://mainnet.helius-rpc.com/?api-key=cd7a4e9a-531b-4aea-9977-cf2ce3499c93",
    "https://mainnet.helius-rpc.com/?api-key=7e9f9722-b9f2-4400-bd51-2d94937e5274",
    "https://mainnet.helius-rpc.com/?api-key=c111ea8f-6884-4775-8b4c-b044410ef192",
    "https://mainnet.helius-rpc.com/?api-key=fccb948a-88e2-4821-b9a3-43c7f9e3ef2b",
    "https://mainnet.helius-rpc.com/?api-key=6b06bb18-0cb8-453e-93dd-c270c2ef0764",
    "https://mainnet.helius-rpc.com/?api-key=fd2565ac-1f2d-420a-a914-0ca60f22b864",
    "https://mainnet.helius-rpc.com/?api-key=55f49bf8-94c3-4697-a8af-428c3be55887",
    "https://mainnet.helius-rpc.com/?api-key=ab8483f1-04ae-4d63-8a53-f182fad16dae",
    "https://mainnet.helius-rpc.com/?api-key=aa674440-fcc8-43f3-ab5e-b0be5e47ca7f",
    "https://mainnet.helius-rpc.com/?api-key=281b5673-3d04-424a-9d35-ae368ad65779",
    "https://mainnet.helius-rpc.com/?api-key=0653fa3d-800c-4e47-83c9-be75a3297768",
    "https://mainnet.helius-rpc.com/?api-key=186a43e1-a825-4897-aa70-f12b0b29b584",
    "https://mainnet.helius-rpc.com/?api-key=e2b089ce-44a2-4f04-a28b-d190386eabf7",
    "https://mainnet.helius-rpc.com/?api-key=a103fad4-ed0a-4dbe-bc78-2325b9cc2813",
    "https://mainnet.helius-rpc.com/?api-key=edea7954-641c-4040-82b6-7d65a9081c7e",
    "https://mainnet.helius-rpc.com/?api-key=1ab7d07f-b34e-4e6b-8695-5eec5797976b",
    "https://mainnet.helius-rpc.com/?api-key=49668913-7679-441c-9832-a9b1381df5d3",
    "https://mainnet.helius-rpc.com/?api-key=20d5ef52-86ee-40ff-a222-7de711b27c58",
    "https://mainnet.helius-rpc.com/?api-key=1ebb537f-9ad4-4c0b-982a-e4ccaa846d22",
    "https://mainnet.helius-rpc.com/?api-key=e1ed7d3e-34ef-4a75-a786-fc32158a16ab",
    "https://mainnet.helius-rpc.com/?api-key=e654e337-b4d5-466e-8f60-735b5a9fa988",
    "https://mainnet.helius-rpc.com/?api-key=1db860f2-52aa-4da2-a716-fad2ffbc1a8f",
    "https://mainnet.helius-rpc.com/?api-key=b5946c05-52f5-4eea-bbc6-3d833dc83a0d",
    "https://mainnet.helius-rpc.com/?api-key=df064f5c-d8b6-4172-a284-60d335508b62",
    "https://mainnet.helius-rpc.com/?api-key=8d85a89e-8718-4018-ae12-0b85c1bb2e51",
    "https://mainnet.helius-rpc.com/?api-key=23aab6c4-141a-4f67-9716-f329dff40676",
    "https://mainnet.helius-rpc.com/?api-key=d8ffd0c8-0a9c-409e-ae78-061a8f271005",
    "https://mainnet.helius-rpc.com/?api-key=e4d59051-1583-4e09-ae86-8a9fb578a297",
    "https://mainnet.helius-rpc.com/?api-key=5e9ae659-3e6c-4190-8c17-ba470ffb2801",
    "https://mainnet.helius-rpc.com/?api-key=55a08f36-7877-45bd-8bd0-26f95f560719",
    "https://mainnet.helius-rpc.com/?api-key=f8dc42cb-8835-4e4c-a913-c39a206d7719",
    "https://mainnet.helius-rpc.com/?api-key=685c4905-dc68-4075-b136-ec88a3f67453",
    "https://mainnet.helius-rpc.com/?api-key=e67079da-9dd8-4966-9cdc-6b5f8b2ef945",
    "https://mainnet.helius-rpc.com/?api-key=1a805d1a-4d4d-49e3-b748-8f5dfb95b9b3",
    "https://mainnet.helius-rpc.com/?api-key=52e3fb3e-f92b-476d-ae23-c6fdc58f67c8",
    "https://mainnet.helius-rpc.com/?api-key=44386077-f005-41cf-98d0-98032de66657",
    "https://mainnet.helius-rpc.com/?api-key=75b1ca73-82b9-4348-9163-f6ac58af0c6f",
    "https://mainnet.helius-rpc.com/?api-key=576b3687-da99-4eaa-b4c7-dc251d59f126",
    "https://mainnet.helius-rpc.com/?api-key=610a027d-2dfa-4800-8df7-3f9a1d5e8be2",
    "https://mainnet.helius-rpc.com/?api-key=71bc4bf6-6fa2-46c0-89e1-2d6eca0fe5bb",
    "https://mainnet.helius-rpc.com/?api-key=15a1c26f-2216-4d02-a479-740c9c429ce6",
    "https://mainnet.helius-rpc.com/?api-key=e5b2bffe-e12d-44a8-9ca5-fc00d71d28ea",
    "https://mainnet.helius-rpc.com/?api-key=ba06b6be-2454-4ce1-865b-38166968eca4",
    "https://mainnet.helius-rpc.com/?api-key=ca98a15d-de76-4001-9a2a-dba66ea8590e",
    "https://mainnet.helius-rpc.com/?api-key=c6ba7552-a070-4751-8929-ba6193eb2294",
    "https://mainnet.helius-rpc.com/?api-key=8b6b4dd9-8c5b-426c-9538-ff91647bd8fe",
    "https://mainnet.helius-rpc.com/?api-key=08f1e098-ad72-43c5-b04f-6fc64f8ec3db",
    "https://mainnet.helius-rpc.com/?api-key=a338e567-ed8d-48a4-a239-32e8bb333fdb",
    "https://mainnet.helius-rpc.com/?api-key=cebc51ee-88c0-4ae5-88d1-5e09d537eaaa",
    "https://mainnet.helius-rpc.com/?api-key=d0d11e2d-5995-4478-81b5-9cac7a79af93",
    "https://mainnet.helius-rpc.com/?api-key=71c38fea-9274-4652-82b3-6bd63811bf95",
    "https://mainnet.helius-rpc.com/?api-key=b44f178a-2991-406c-85e2-ade65bc361ae",
    "https://mainnet.helius-rpc.com/?api-key=8c644bcc-71c4-492b-b7a9-b3cb260a2f5d",
    "https://mainnet.helius-rpc.com/?api-key=a8ba3874-d967-43c2-962e-d8879e8c9c12",
    "https://mainnet.helius-rpc.com/?api-key=a47e2add-1583-485b-85a7-6e019895ad99",
    "https://mainnet.helius-rpc.com/?api-key=44fd2da1-dd04-4d77-b09e-88692782b9f3",
    "https://mainnet.helius-rpc.com/?api-key=07097db8-ec7b-4cd3-81d0-abb901e8645d",
    "https://mainnet.helius-rpc.com/?api-key=b3bf4300-04bc-4875-af7c-afdee375661b",
    "https://mainnet.helius-rpc.com/?api-key=c404aabb-cd34-4d95-87bf-b1de897a1c0a",
    "https://mainnet.helius-rpc.com/?api-key=a7d5f296-b126-454a-9a8e-72c1dad17b35",
    "https://mainnet.helius-rpc.com/?api-key=17a57c87-ef1b-4ec9-9de7-7fd880684ecb",
    "https://mainnet.helius-rpc.com/?api-key=a1301e60-6f95-4e60-830a-1208340cc99a",
    "https://mainnet.helius-rpc.com/?api-key=53af3e49-357f-4dcb-9c96-8e3f14039056",
    "https://mainnet.helius-rpc.com/?api-key=ad91cf0d-b277-4317-b14d-b12264932244",
    "https://mainnet.helius-rpc.com/?api-key=12c2f0dc-6864-4df5-80e1-7abede3eec72",
    "https://mainnet.helius-rpc.com/?api-key=a4aedf93-1114-4171-a5cc-ebea8038a960",
    "https://mainnet.helius-rpc.com/?api-key=fa355804-374b-4860-9df3-6deb3347603e",
    "https://mainnet.helius-rpc.com/?api-key=4f31edd6-465b-4cbc-b746-950ce269ff98",
    "https://mainnet.helius-rpc.com/?api-key=3d4dbb5b-2900-4614-be72-a7ed4e38d641",
    "https://mainnet.helius-rpc.com/?api-key=526cee46-ae8d-443b-bbcd-59799c592fd9",
    "https://mainnet.helius-rpc.com/?api-key=6d1d0e15-f4a6-4e69-83c5-4605c6e44f45",
    "https://mainnet.helius-rpc.com/?api-key=70e8aa5f-a6cc-4150-9da5-f8a2aa063e50",
    "https://mainnet.helius-rpc.com/?api-key=58436fb0-9b4c-4df3-ba82-cc2ca62e9a2f",
    "https://mainnet.helius-rpc.com/?api-key=d8f9e779-1a8c-422a-bccc-a8b1989ff40a",
    "https://mainnet.helius-rpc.com/?api-key=fd2efdfc-0251-495c-abee-df9db97b7752",
    "https://mainnet.helius-rpc.com/?api-key=696a86d6-d3c2-45fe-bead-b1e4d7516ef9",
    "https://mainnet.helius-rpc.com/?api-key=99302864-191f-4472-97bf-499b6745a899",
    "https://mainnet.helius-rpc.com/?api-key=1fde5e59-b771-4db5-87f9-97c0e1c2feb4",
    "https://mainnet.helius-rpc.com/?api-key=4a9cd9e9-8eca-4ac8-b80b-e7e2feae391d",
    "https://mainnet.helius-rpc.com/?api-key=ce2a079c-f8c1-4b30-96ad-9abcb0453028",
    "https://mainnet.helius-rpc.com/?api-key=7e732217-3e71-4a4a-99c0-42e831126b4d",
    "https://mainnet.helius-rpc.com/?api-key=e40a857d-14f7-4984-a2ca-7e42eef0bfaa",
    "https://mainnet.helius-rpc.com/?api-key=cf201b98-3d0c-45c8-a78d-23bef05b6a08",
    "https://mainnet.helius-rpc.com/?api-key=334c42cc-73f3-4e7e-b907-e3091a39802e",
    "https://mainnet.helius-rpc.com/?api-key=ede2a34f-7ddf-442b-af6c-7083e855f2d0",
    "https://mainnet.helius-rpc.com/?api-key=9f41b3eb-ff78-46c0-8f71-83c9441e4e3c",
    "https://mainnet.helius-rpc.com/?api-key=6113a4d0-b0ae-4820-bf16-6e14fb838a19",
    "https://mainnet.helius-rpc.com/?api-key=464bcd47-0a5f-4064-a954-7ac735772f41",
    "https://mainnet.helius-rpc.com/?api-key=3f9301f4-19f3-4b8b-9f75-73967dd0a73c",
    "https://mainnet.helius-rpc.com/?api-key=21a92100-b309-4cdd-9510-460db47f7698",
    "https://mainnet.helius-rpc.com/?api-key=d2368c6c-0d44-4f7f-bc11-f595d628534c",
    "https://mainnet.helius-rpc.com/?api-key=f8a0f0eb-692f-46d7-82a9-2e99994222e6",
    "https://mainnet.helius-rpc.com/?api-key=63f8e319-5a85-4497-b04b-a181d71f75cb",
    "https://mainnet.helius-rpc.com/?api-key=fff15a0e-045b-4eb6-9eb0-6d9ecc432526",
    "https://mainnet.helius-rpc.com/?api-key=a0c376f4-43bb-4c73-9aaa-b494a46d4d33",
    "https://mainnet.helius-rpc.com/?api-key=813c5e06-7682-4f67-8e08-d27ffe2e4b83",
    "https://mainnet.helius-rpc.com/?api-key=97b1058c-a454-4836-92d9-f2f1422e94cb",
    "https://mainnet.helius-rpc.com/?api-key=501137ad-4f96-4821-b860-ebb225b3effe",
    "https://mainnet.helius-rpc.com/?api-key=cf4bb216-ba0d-46c5-a30e-5a84797f0a14",
    "https://mainnet.helius-rpc.com/?api-key=157657ce-ce34-46ce-915b-cc46e43afa56",
    "https://mainnet.helius-rpc.com/?api-key=12063f84-fa97-4c98-a4a3-076172eea217",
    "https://mainnet.helius-rpc.com/?api-key=49259a9b-f809-4ba1-a94f-766f9f5d65ef",
    "https://mainnet.helius-rpc.com/?api-key=d65025c0-0792-4f6b-92eb-c71320c2ba5d",
    "https://mainnet.helius-rpc.com/?api-key=68a7894d-c127-48b1-9a18-d663e528fcf2",
    "https://mainnet.helius-rpc.com/?api-key=1587daaa-0de7-47e6-93a0-d43f4b3633e8",
    "https://mainnet.helius-rpc.com/?api-key=03266c0d-2b86-4914-bb09-a24c7f815e06",
    "https://mainnet.helius-rpc.com/?api-key=e82609d9-f092-41a1-afb7-67d9afb29fba",
    "https://mainnet.helius-rpc.com/?api-key=bac60915-e4cd-46a2-b5fe-d955ef170147",
    "https://mainnet.helius-rpc.com/?api-key=98aa0dae-99cc-4a9c-8360-8b0c52df6e14",
    "https://mainnet.helius-rpc.com/?api-key=129a9208-db2a-4817-9d8e-78164ef5e103",
    "https://mainnet.helius-rpc.com/?api-key=83b51d1b-9866-49c0-a042-df77258b1559",
    "https://mainnet.helius-rpc.com/?api-key=ea89953f-e4fc-488b-bf37-a63dabd81618",
    "https://mainnet.helius-rpc.com/?api-key=12372e63-9028-45c9-b0ce-58549084f878",
    "https://mainnet.helius-rpc.com/?api-key=0789b3f7-8daf-415d-b399-3eadf31c46bb",
    "https://mainnet.helius-rpc.com/?api-key=7fbede7d-c560-4b0a-b016-93df70870641",
    "https://mainnet.helius-rpc.com/?api-key=6d030462-3e41-4aed-981e-f794a50045ef",
    "https://mainnet.helius-rpc.com/?api-key=2fdb9572-af5f-419d-9461-50bd18054520",
    "https://mainnet.helius-rpc.com/?api-key=c0858215-c1ae-49d1-a185-0cad35a5ee5f",
    "https://mainnet.helius-rpc.com/?api-key=4deb032a-4792-432d-9466-0fd4b4dfd2bf",
    "https://mainnet.helius-rpc.com/?api-key=92366b35-aac4-4d22-a047-586efa7136a5",
    "https://mainnet.helius-rpc.com/?api-key=8bb07512-4e96-466c-b5c6-d510a899e916",
    "https://mainnet.helius-rpc.com/?api-key=0e950154-f367-4475-b98a-8b8e5238f614",
    "https://mainnet.helius-rpc.com/?api-key=e5768a28-e77d-4927-b79a-34b784774538",
    "https://mainnet.helius-rpc.com/?api-key=52ea859d-197a-4ea3-82cb-7796da85ff29",
    "https://mainnet.helius-rpc.com/?api-key=cd078980-1036-45aa-99eb-3cb5315a13bc",
    "https://mainnet.helius-rpc.com/?api-key=2a2b104f-be0a-4fe1-a07d-e1d38dc93f86",
    "https://mainnet.helius-rpc.com/?api-key=7aab9900-78fb-4cb6-b923-b421e170fe18",
    "https://mainnet.helius-rpc.com/?api-key=d7cb154b-a972-408f-9de2-7108473c8f6b",
    "https://mainnet.helius-rpc.com/?api-key=973e9648-6f77-4be1-85da-2b38626cf98a",
    "https://mainnet.helius-rpc.com/?api-key=a507d876-75e1-4c0b-9b41-1ab4ae2508c5",
    "https://mainnet.helius-rpc.com/?api-key=df8546f4-0e91-4d56-906e-494b19fc6081",
    "https://mainnet.helius-rpc.com/?api-key=9be7c5f6-65ae-4bd9-ad5e-730e828a0b85",
    "https://mainnet.helius-rpc.com/?api-key=be2ae6be-5c99-4032-b289-297f1df86914",
    "https://mainnet.helius-rpc.com/?api-key=d4d416a0-5fbd-4fe4-8336-0378a8c64d96",
    "https://mainnet.helius-rpc.com/?api-key=4536337b-420d-4b78-97a5-125c1c557a5e",
    "https://mainnet.helius-rpc.com/?api-key=7ddad0b5-e337-4a82-a4ca-2e1c55a6f175",
    "https://mainnet.helius-rpc.com/?api-key=4413672a-45f2-4b92-8fc4-3ce7bd8b307e",
    "https://mainnet.helius-rpc.com/?api-key=239096c7-2184-4f86-86ba-a40c24b0f535",
    "https://mainnet.helius-rpc.com/?api-key=dd869339-bb09-43ff-814c-86a948f5ff2f",
    "https://mainnet.helius-rpc.com/?api-key=a70d65d6-c638-4f64-8469-32e5d6b5d44c",
    "https://mainnet.helius-rpc.com/?api-key=1b84331e-d814-4930-9707-853a8d0e27b7",
    "https://mainnet.helius-rpc.com/?api-key=c0800339-d8ae-495e-a6cc-1427eb7346e1",
    "https://mainnet.helius-rpc.com/?api-key=7f428baf-09db-4c04-8f23-b6afa2f89e15",
    "https://mainnet.helius-rpc.com/?api-key=05a6f0fb-76c6-4a43-89fe-f0d261c93952",
    "https://mainnet.helius-rpc.com/?api-key=2f4b99a1-3481-4b62-9ee9-4f9cfd4b96a2",
    "https://mainnet.helius-rpc.com/?api-key=5e3f2a5f-8dd7-4f9a-8820-549c2c3da762",
    "https://mainnet.helius-rpc.com/?api-key=65655761-b3db-4016-9c64-a460c70ec47e",
    "https://mainnet.helius-rpc.com/?api-key=358724c2-2c31-44f6-82b3-ee8cbe5941dc",
    "https://mainnet.helius-rpc.com/?api-key=4a609381-e425-4dc9-9e60-9fb4d55bee08",
    "https://mainnet.helius-rpc.com/?api-key=399cfced-c7dd-43f7-9f90-d10b123d6885",
    "https://mainnet.helius-rpc.com/?api-key=7887e499-bd23-409c-91bf-d04bb5a8333e",
    "https://mainnet.helius-rpc.com/?api-key=d9dbcbb2-040e-489f-8896-88240eae8c1e",
    "https://mainnet.helius-rpc.com/?api-key=e4b63b4d-1fc5-4e0b-8f55-b44aeb5232bc",
    "https://mainnet.helius-rpc.com/?api-key=17b6a906-053e-4477-b4c4-05d79ac47d10",
    "https://mainnet.helius-rpc.com/?api-key=6365d28a-ab5f-4fc1-b78a-99fd2f1f8cd9",
    "https://mainnet.helius-rpc.com/?api-key=0cccbd7d-f49e-434c-b54c-c9a51bab0f0b",
    "https://mainnet.helius-rpc.com/?api-key=b0835905-ab18-47c4-9270-c54a93122975",
    "https://mainnet.helius-rpc.com/?api-key=d63c33dc-d1df-4359-bbb9-912f79d445eb",
    "https://mainnet.helius-rpc.com/?api-key=63303cd0-bf2a-4151-b5da-07dd6505b712",
    "https://mainnet.helius-rpc.com/?api-key=41e37c65-2bfc-41b7-ae32-15f85b3a2dbe",
    "https://mainnet.helius-rpc.com/?api-key=29516388-c365-4e59-b679-d279e23aab21",
    "https://mainnet.helius-rpc.com/?api-key=f20cb3ba-34a9-46dc-b434-8c14bc37c7e4",
    "https://mainnet.helius-rpc.com/?api-key=3cfc9fac-b39d-41bb-af9b-2623c7d8ac7d",
    "https://mainnet.helius-rpc.com/?api-key=1e92dace-44b4-418a-b661-50dce316b80b",
    "https://mainnet.helius-rpc.com/?api-key=db23ef8f-2823-40aa-aee5-1ff5547c9e43",
    "https://mainnet.helius-rpc.com/?api-key=9265d6ec-959b-419b-bb9e-7fe342cee3a0",
    "https://mainnet.helius-rpc.com/?api-key=b2dd02d2-0661-44bb-b63c-80de81ef2f3a",
    "https://mainnet.helius-rpc.com/?api-key=d73b64b5-639e-4c2f-8eeb-b78b0ff6c10a",
    "https://mainnet.helius-rpc.com/?api-key=48245fc9-13bd-4032-8b56-0d2c67780472",
    "https://mainnet.helius-rpc.com/?api-key=c65bafb2-a825-4024-a049-a0d3151a9c8e",
    "https://mainnet.helius-rpc.com/?api-key=d948935c-37dd-4718-973a-b4f7c93dea87",
    "https://mainnet.helius-rpc.com/?api-key=9f735b56-f8df-4049-9b8e-44fa51e418e2",
    "https://mainnet.helius-rpc.com/?api-key=7b27197c-1ce5-4e10-861f-a90fce1ffb85",
    "https://mainnet.helius-rpc.com/?api-key=71ed1cf3-e6ce-4fa0-8389-1078e70429d3",
    "https://mainnet.helius-rpc.com/?api-key=75c5f84d-4551-438f-814e-65aedc1b9718",
    "https://mainnet.helius-rpc.com/?api-key=caf60599-2ff8-4ae6-b312-547889a616ef",
    "https://mainnet.helius-rpc.com/?api-key=72a72d1b-9086-4f73-830f-d3cb62d1aad9",
    "https://mainnet.helius-rpc.com/?api-key=26810cbb-a8e4-4b6a-b67d-9e62d29d82c5",
    "https://mainnet.helius-rpc.com/?api-key=2af252ca-5799-4299-98e8-1656d105981b",
    "https://mainnet.helius-rpc.com/?api-key=b5b21147-1200-4121-9a34-770aaa4f2750",
    "https://mainnet.helius-rpc.com/?api-key=50cf480b-4893-40ea-8476-e1b543430cd0",
    "https://mainnet.helius-rpc.com/?api-key=4ba5804a-8fc5-4329-8108-6ca04f892990",
    "https://mainnet.helius-rpc.com/?api-key=25408825-50b3-418d-9f7e-874b4c76adbc",
    "https://mainnet.helius-rpc.com/?api-key=91eac3ca-2890-4726-9104-8070a3ae8111",
    "https://mainnet.helius-rpc.com/?api-key=f8e7f405-e0d0-4eda-896d-e499cfe606bf",
    "https://mainnet.helius-rpc.com/?api-key=0910e002-d4a3-4a27-8458-8a0d00aaa42b",
    "https://mainnet.helius-rpc.com/?api-key=859cbd05-cf3e-4c5a-b146-6cbf5adcb054",
    "https://mainnet.helius-rpc.com/?api-key=0c2ae587-3f47-4dc6-ae91-120cf94d27eb",
    "https://mainnet.helius-rpc.com/?api-key=55066b34-7911-478c-b3fa-88154fbf17bb",
    "https://mainnet.helius-rpc.com/?api-key=9e1e10f0-2d51-4b4b-a509-a3e9136a3af8",
    "https://mainnet.helius-rpc.com/?api-key=24e2495a-ed15-476f-8119-ae7941563202",
    "https://mainnet.helius-rpc.com/?api-key=85e89c11-49b6-4bde-a0c1-6c7d2e27bfeb",
    "https://mainnet.helius-rpc.com/?api-key=7564e3b0-a1d3-41ca-badf-7d6af697db0b",
    "https://mainnet.helius-rpc.com/?api-key=c888dfff-8140-4975-979a-1f584b735438",
    "https://mainnet.helius-rpc.com/?api-key=7021ff20-3192-4afc-9bff-37ed2c3e7a97",
    "https://mainnet.helius-rpc.com/?api-key=8f2accae-257f-4fce-a77f-c156222df14d",
    "https://mainnet.helius-rpc.com/?api-key=be1a583a-d37c-482e-983f-f41a48e6e13a",
    "https://mainnet.helius-rpc.com/?api-key=4182c684-2ade-4428-8349-8c060c6d36ac",
    "https://mainnet.helius-rpc.com/?api-key=108a9dac-035e-490a-abb2-83e2b52592d8",
];
pub const PAIRS_CNT: usize = 2840;
// pub const ROUTES_CNT: usize = 368028;
// pub const PAIRS_CNT: usize = 1300;
// pub const ROUTES_CNT: usize = 368028;
pub const FEE_MULTIPLIER: u32 = 10000;
pub const PRICE_DELTA_MULTIPLIER: u128 = 1_000_000;
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


pub struct CurveParam {
    amount_in: u64,
    last_price: u128,
    base_decimal_val: u64,
    confidence: u64,
    coin_balance: u64,
    pc_balance: u64,
    is_a_to_b: bool,
    price_buffer_coin: i64,
    price_buffer_pc: i64,
    config_denominator: u64,
    regression_target: u64,
    spread_coefficient: u64,
    std_spread: u64,
    std_spread_buffer: u64,
}
pub fn standard_curve(param: &CurveParam) -> (u64, u128) {
    let mut _amount_out = 0u64;
    let mut _next_price = 0u128;
    let mut _price_delta_f64: f64 = 0f64;

    if param.is_a_to_b {
        // let cur_price = param.last_price;
        let cur_price = param
            .last_price
            .checked_mul((param.price_buffer_coin + param.config_denominator as i64) as u128)
            .unwrap()
            .checked_div(param.config_denominator as u128)
            .unwrap();

        let cur_coin_balance = param.coin_balance;
        let base: f64 =
            (cur_coin_balance + param.amount_in) as f64 / param.regression_target as f64 - 1f64;

        _price_delta_f64 = if base < 0f64 {
            0f64
        } else {
            base.pow(param.spread_coefficient as f64 / param.config_denominator as f64)
                .mul(param.std_spread as f64 / param.config_denominator as f64)
                .add(param.std_spread_buffer as f64 / param.config_denominator as f64)
        };
        if _price_delta_f64 > 1f64 {
            _price_delta_f64 = 0.99f64;
        }
        let price_delta: u128 =
            ((1f64 - _price_delta_f64) * param.config_denominator as f64).floor() as u128;
        let swap_price = cur_price
            .checked_mul(price_delta)
            .unwrap()
            .checked_div(param.config_denominator as u128)
            .unwrap()
            - param.confidence as u128;

        _amount_out = (param.amount_in as u128)
            .checked_mul(swap_price)
            .unwrap()
            .checked_div(param.base_decimal_val as u128)
            .unwrap() as u64; //floor

        if _amount_out > param.pc_balance {
            _amount_out = param.pc_balance - 1;
        }
        _next_price = swap_price
            .checked_mul((param.price_buffer_coin + param.config_denominator as i64) as u128)
            .unwrap()
            .checked_div(param.config_denominator as u128)
            .unwrap();
    } else {
        // let cur_price = param.last_price;
        let cur_price = param
            .last_price
            .checked_mul((param.price_buffer_pc + param.config_denominator as i64) as u128)
            .unwrap()
            .checked_div(param.config_denominator as u128)
            .unwrap();
        let cur_coin_balance = param.coin_balance;
        let reg_target_amount = if param.regression_target * 2 < cur_coin_balance {
            0u64
        } else {
            cur_price
                .checked_mul((param.regression_target * 2 - cur_coin_balance) as u128)
                .unwrap()
                .checked_div(param.base_decimal_val as u128)
                .unwrap() as u64
        };
        let base: f64 = (reg_target_amount + param.amount_in) as f64
            / ((param.regression_target as u128)
                .checked_mul(cur_price)
                .unwrap()
                .checked_div(param.base_decimal_val as u128)
                .unwrap()) as f64
            - 1f64;

        _price_delta_f64 = if base < 0f64 {
            0f64
        } else {
            base.pow(param.spread_coefficient as f64 / param.config_denominator as f64)
                .mul(param.std_spread as f64 / param.config_denominator as f64)
                .add(param.std_spread_buffer as f64 / param.config_denominator as f64)
        };
        if _price_delta_f64 > 1f64 {
            _price_delta_f64 = 0.99f64;
        }

        let price_delta: u128 =
            ((1f64 + _price_delta_f64) * param.config_denominator as f64).floor() as u128;

        let swap_price = cur_price
            .checked_mul(price_delta)
            .unwrap()
            .checked_div(param.config_denominator as u128)
            .unwrap()
            .checked_sub(param.confidence as u128)
            .unwrap();

        _amount_out = (param.amount_in as u128)
            .checked_mul(param.base_decimal_val as u128)
            .unwrap()
            .checked_div(swap_price)
            .unwrap() as u64;

        if _amount_out > param.coin_balance {
            _amount_out = param.coin_balance - 1;
        }
        _next_price = swap_price
            .checked_mul((param.price_buffer_coin + param.config_denominator as i64) as u128)
            .unwrap()
            .checked_div(param.config_denominator as u128)
            .unwrap();
    }
    (_amount_out, _next_price)
}

pub struct BankBot {
    pub tokio_runtime: tokio::runtime::Runtime,
    pub pairs: AtomicPtr<Vec<Pair>>,
    pub pairs_v2: AtomicPtr<Vec<PairV2>>,
    pub routes: AtomicPtr<Vec<Route>>,
    pub routes_v2: AtomicPtr<Vec<RouteV2>>,
    pub routes_state: DashMap<usize, RouteState>,

    pub lookuptables: AtomicPtr<Vec<AddressLookupTableAccount>>,
    pub groups: AtomicPtr<Vec<Vec<u32>>>,

    pub ticks: AtomicPtr<Vec<TickInfo>>,

    pub accounts_to_subscribe_vec: AtomicPtr<Vec<Pubkey>>,
    pub accounts_to_subscribe: AtomicPtr<HashMap<Pubkey, SubscribeAccount>>,


    pub last_blockhash_timestamp: AtomicU64,
    pub latest_blockhash: AtomicPtr<Option<Hash>>,
    pub is_running: AtomicBool,
    pub is_bundle_allowed: AtomicBool,
    pub no_jito_validators: AtomicPtr<Vec<Pubkey>>,
    pub is_price_updated_initially: AtomicBool,
    pub jito_bundle_endpoint_idx: AtomicUsize,
    pub jito_tx_endpoint_idx: AtomicUsize,
    pub proxy_idx: AtomicUsize,
    pub bundle_sender: udp_proxy::BundleSender,

    pub process_pair_cnt: AtomicU8,

    pub payer: Keypair,
    pub tip_payer: Keypair,
    pub rpc_client: Arc<RpcClient>,
    pub tick_rpc_client_index: AtomicUsize,

    
    pub pair_change_stack: AtomicPtr<PairChangeStack>,
    pub check_ixs: [Instruction; 3],
    pub check_payer: Keypair
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
        let payer = signature::read_keypair_file("/mnt/wallet/payer.json").unwrap();
        let tip_payer = signature::read_keypair_file("/mnt/wallet/tip_payer.json").unwrap();
        let rpc_client = Arc::new(RpcClient::new_with_commitment(String::from(RPC_ENDPOINT), CommitmentConfig::processed()));

        let check_payer = signature::read_keypair_file("/mnt/wallet/user.json").unwrap();
        // let pubkey = check_payer.pubkey();

        let account_metas = vec![
            AccountMeta {
                pubkey: Pubkey::from_str("FdSEVgY72ztgZEeNvGtWw1wdNvSaLkNSgCPM5qtkck2S").unwrap(), // pda
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
            pubkey: Pubkey::from_str("D6k1znFSoG8Am73BBeY1JLpiDfQyqefTmdXoHQnZef7d").unwrap(), // trade_authority
            is_signer: false,
            is_writable: true,
            },
        ];
        let program_id = Pubkey::from_str("botHDy47CNugroED1sxHfYeAiXwDKbgaa1WUeBvqnej").unwrap();
        let discriminator = ArcBankBot::sighash("global", "tradex_oracle");

        let ix = Instruction::new_with_borsh(
            program_id,
            &(discriminator),
            account_metas.to_vec(),
        );
        let cu_ix = ComputeBudgetInstruction::set_compute_unit_limit(2000);
        let cp_ix = ComputeBudgetInstruction::set_compute_unit_price(2_500_000);
        let check_ixs = [cu_ix, cp_ix, ix];

        BankBot {
            tokio_runtime: runtime,
            pairs: AtomicPtr::new(Box::into_raw(Box::new(Vec::new()))),
            pairs_v2: AtomicPtr::new(Box::into_raw(Box::new(Vec::new()))),
            routes: AtomicPtr::new(Box::into_raw(Box::new(Vec::new()))),
            routes_v2: AtomicPtr::new(Box::into_raw(Box::new(Vec::new()))),
            routes_state: DashMap::new(),
            lookuptables: AtomicPtr::new(Box::into_raw(Box::new(Vec::new()))),
            groups: AtomicPtr::new(Box::into_raw(Box::new(Vec::new()))),
            ticks: AtomicPtr::new(Box::into_raw(Box::new(Vec::new()))),
            accounts_to_subscribe_vec: AtomicPtr::new(Box::into_raw(Box::new(Vec::new()))),
            accounts_to_subscribe: AtomicPtr::new(Box::into_raw(Box::new(HashMap::new()))),
            process_pair_cnt: AtomicU8::new(0),

            last_blockhash_timestamp: AtomicU64::new(0),
            latest_blockhash: AtomicPtr::new(Box::into_raw(Box::new(None))),

            is_running: AtomicBool::new(false),
            is_bundle_allowed: AtomicBool::new(false),
            no_jito_validators: AtomicPtr::new(Box::into_raw(Box::new(Vec::new()))),
            is_price_updated_initially: AtomicBool::new(false),
            jito_bundle_endpoint_idx: AtomicUsize::new(0),
            jito_tx_endpoint_idx: AtomicUsize::new(0),
            proxy_idx: AtomicUsize::new(0),
            payer,
            tip_payer,
            rpc_client,
            bundle_sender: udp_proxy::BundleSender::create(),
            tick_rpc_client_index: AtomicUsize::new(0),

            pair_change_stack: AtomicPtr::new(Box::into_raw(Box::new(PairChangeStack::default()))),
            check_ixs,
            check_payer
        }
    }
}

impl BankBot {
    pub fn create_and_send_ix(
        &self,
        tx_instructions: &[Instruction],
        lookuptables: &[AddressLookupTableAccount],
        skip_preflight: bool,
        payer: &Keypair
    ) -> ClientResult<Signature> {
        let signers = vec![payer];
        let blockhash = unsafe{&* self.latest_blockhash.load(Ordering::SeqCst)};

        let versioned_message = V0(solana_sdk::message::v0::Message::try_compile(
            &payer.pubkey(),
            tx_instructions,
            lookuptables,
            blockhash.unwrap(),
        )
        .unwrap());

        let tx = solana_sdk::transaction::VersionedTransaction::try_new(versioned_message, &signers)
            .unwrap();
        self.rpc_client.send_transaction_with_config(
            &tx,
            RpcSendTransactionConfig {
                skip_preflight,
                preflight_commitment: Some(CommitmentLevel::Processed),
                ..RpcSendTransactionConfig::default()
            },
        )
    }
    pub fn send_tx(
        &self,
        tx: &VersionedTransaction,
        skip_preflight: bool
    ) -> ClientResult<Signature> {
        self.rpc_client.send_transaction_with_config(
            tx,
            RpcSendTransactionConfig {
                skip_preflight,
                preflight_commitment: Some(CommitmentLevel::Processed),
                ..RpcSendTransactionConfig::default()
            },
        )
    }
    pub fn create_and_send_ix_with_rpc(
        &self,
        tx_instructions: &[Instruction],
        lookuptables: &[AddressLookupTableAccount],
        skip_preflight: bool,
        rpc_client: RpcClient
    ) -> ClientResult<Signature> {
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
        rpc_client.send_transaction_with_config(
            &tx,
            RpcSendTransactionConfig {
                skip_preflight,
                preflight_commitment: Some(CommitmentLevel::Processed),
                ..RpcSendTransactionConfig::default()
            },
        )
    }
    pub fn simulate_bundle(&self, versioned_txs: &[VersionedTransaction]) -> RpcResult<RpcSimulateBundleResult> {
        let mut configs = vec![];
        let len = versioned_txs.len();
        for _i in 0..len {
            configs.push(None);
        }
        self.rpc_client.simulate_bundle_with_config(&versioned_txs, RpcSimulateBundleConfig {
            skip_sig_verify: true, 
            replace_recent_blockhash: false,
            pre_execution_accounts_configs: configs.clone(),
            post_execution_accounts_configs: configs.clone(),
            ..RpcSimulateBundleConfig::default()
        })
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
    pub async fn run_tick_bot_in_background(instance: &Arc<Self>) {
        loop {
            if !instance.is_running.load(Ordering::SeqCst) {
                break;
            }
            println!("tick bot cycle started!");
            let pairs = unsafe { &*instance.pairs.load(Ordering::SeqCst) };
            let clmm_pairs: Vec<&Pair> = pairs
                .iter()
                .filter_map(|p| {
                    if CLMM_SWAP_TYPES.contains(&p.swap_type) {
                        Some(p)
                    } else {
                        None
                    }
                })
                .collect();
            let total_clmm_len = clmm_pairs.len();
            let mut tick_arrays: Box<Vec<TickInfo>> =
                Box::new(Vec::with_capacity(total_clmm_len));
            let mut dlmm_ticks: Vec<String> = Vec::new();
            let mut progress = 0;
            for pair in clmm_pairs {
                progress += 1;
                if !instance.is_running.load(Ordering::SeqCst) {
                    break;
                }
                let pool_pubkey = &pair.pool;
                println!("processing pool {}/{}, {:#?}", progress, total_clmm_len, pool_pubkey);
                loop {
                    let tick_rpc_client_index =
                        instance.tick_rpc_client_index.load(Ordering::SeqCst);
                    let tick_rpc_client = RpcClient::new(String::from(
                        TICK_RPC_ENDPOINTS[tick_rpc_client_index % 2007],
                    ));
                    instance
                        .tick_rpc_client_index
                        .store((tick_rpc_client_index + 1) % 2007, Ordering::SeqCst);

                    let tick_info: Option<TickInfo> = match pair.swap_type
                    {
                        SwapType::RaydiumConcentrated => {
                            BankBot::fetch_raydium_ticks(instance, pool_pubkey, &tick_rpc_client)
                        }
                        SwapType::OrcaWhirlpool => BankBot::fetch_orca_whirlpool_ticks(
                            instance,
                            pool_pubkey,
                            &tick_rpc_client,
                        ),
                        SwapType::CropperWhirlpool => BankBot::fetch_cropper_whirlpool_ticks(
                            instance,
                            pool_pubkey,
                            &tick_rpc_client,
                        ),
                        SwapType::InvariantSwap => {
                            BankBot::fetch_invariant_ticks(instance, pool_pubkey, &tick_rpc_client)
                        }
                        SwapType::CremaFinance => {
                            BankBot::fetch_crema_ticks(instance, pool_pubkey, &tick_rpc_client)
                        }
                        SwapType::MeteoraDlmm => {
                            let tick_info = BankBot::fetch_meteora_dlmm_ticks(
                            instance,
                            pool_pubkey,
                            &tick_rpc_client);
                            if tick_info.is_some() {
                                let new_dlmm_ticks:Vec<String> = tick_info.as_ref().unwrap().ticks.iter().map(|tick| tick.address.clone()).collect();
                                dlmm_ticks.extend(new_dlmm_ticks);
                            }
                            tick_info
                        },
                        _ => None,
                    };
                    if tick_info.is_some() && !tick_info.clone().unwrap().ticks.is_empty() {
                        // println!("get tick info from onchain, swapType: {:#?}, pool: {:#?}, tick_info: {:#?}", pair.swapType, pair.pool, tick_info.clone().unwrap());
                        // println!("get tick info from onchain, swapType: {:#?}, pool: {:#?}", pair.swapType, pair.pool);
                        tick_arrays.push(tick_info.unwrap());

                        break;
                    } else {
                        if pool_pubkey
                            .to_string()
                            .eq("yEuKWog8fBZ25JBRYrXs6igynxxiythd5zSsfUTSWwa") ||
                            pool_pubkey
                            .to_string()
                            .eq("FPbp7mWLzqsjUDRY1ujfh5dNaSR7nzym7rV8P8GhbjcZ")||
                            pool_pubkey
                            .to_string()
                            .eq("7GZXejXCyJ3R78d71328wqMWT7Ejwu7pxxWRWvHSeVfb")
                        {
                            // for now this has no ticks-> invariant
                            break;
                        }
                        println!(
                            "retrying to get ticks-> pair.swapType: {:#?}, pool_pubkey: {:#?}",
                            pair.swap_type, pool_pubkey
                        );
                    }
                }
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            }
            if !instance.is_running.load(Ordering::SeqCst) {
                break;
            }
            let all_tick_addresses: Vec<&String> = tick_arrays
                .iter()
                .flat_map(|tick| tick.ticks.iter().map(|t| &t.address))
                .collect();
            let missed_tick_addresses: Vec<&String> = all_tick_addresses
                .iter()
                .filter_map(|&addr| {
                    if BankBot::is_exist_in_lookuptable(instance, addr) {
                        None
                    } else {
                        Some(addr)
                    }
                })
                .collect();
            if !missed_tick_addresses.is_empty() {
                println!("inserting missed {} ticks to lookup table", missed_tick_addresses.len());
                BankBot::insert_address_to_lookuptable(instance, &missed_tick_addresses).await;
            }
            // read origin pool addresses
            let mut total_addresses_to_subscribe: Vec<String> = Vec::new();
            if let Ok(acounts_to_subscribe_content) = spawn_blocking(|| {
                let mut accounts_to_subscribe_file = File::open(POOL_ADDRESSES_FILE_PATH)?;
                let mut content = String::new();
                accounts_to_subscribe_file.read_to_string(&mut content)?;
                Ok::<_, std::io::Error>(content)
            })
            .await
            {
                match serde_json::from_str(&acounts_to_subscribe_content.unwrap()) {
                    Ok(res) => {
                        total_addresses_to_subscribe = res;
                    }
                    Err(_err) => println!("accounts_to_subscribe read error-> {:#?}", _err),
                };
            };
            total_addresses_to_subscribe.extend(dlmm_ticks);

            // save bin arrays separately
            let serialized_dlmm_content = serde_json::to_string(&*total_addresses_to_subscribe).unwrap();
            if let Err(err) = spawn_blocking(move || {
                let mut bin_arrays_file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(TOTAL_ADDRESSES_TO_SUB_FILE_PATH)?;
                bin_arrays_file.write_all(serialized_dlmm_content.as_bytes())?;
                bin_arrays_file.flush()?;
                println!("saved result bin arrays to file");
                Ok::<_, std::io::Error>(())
            })
            .await
            {
                println!("Error writing ticks to file: {:#?}", err);
            }

            println!("saved dlmm bin arrays");

            // save all ticks
            let serialized_content = serde_json::to_string(&*tick_arrays).unwrap();
            instance
                .ticks
                .store(Box::into_raw(tick_arrays), Ordering::SeqCst);
            println!("saving result ticks to file");
            if let Err(err) = spawn_blocking(move || {
                let mut ticks_file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(TICK_FILE_PATH)?;
                ticks_file.write_all(serialized_content.as_bytes())?;
                ticks_file.flush()?;
                println!("saved result ticks to file");
                Ok::<_, std::io::Error>(())
            })
            .await
            {
                println!("Error writing ticks to file: {:#?}", err);
            }
            // Sleep to avoid tight loop
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }
    }
    pub async fn run_tick_bot_in_background_v2(instance: &Arc<Self>) {
        loop {
            if !instance.is_running.load(Ordering::SeqCst) {
                break;
            }
            println!("tick bot cycle started!");
            let pairs = unsafe { &*instance.pairs_v2.load(Ordering::SeqCst) };
            let clmm_pairs: Vec<&PairV2> = pairs
                .iter()
                .filter_map(|p| {
                    if CLMM_SWAP_TYPES.contains(&p.swap_type) {
                        Some(p)
                    } else {
                        None
                    }
                })
                .collect();
            let total_clmm_len = clmm_pairs.len();
            let mut tick_arrays: Box<Vec<TickInfo>> =
                Box::new(Vec::with_capacity(total_clmm_len));
            let mut dlmm_ticks: Vec<String> = Vec::new();
            let mut progress = 0;
            
            for pair in clmm_pairs {
                progress += 1;
                if !instance.is_running.load(Ordering::SeqCst) {
                    break;
                }
                let pool_pubkey = &pair.pool;
                let mut retrying_cnt = 0;
                println!("getting ticks of pool {}/{}, {:#?}", progress, total_clmm_len, pool_pubkey);
                loop {
                    let tick_rpc_client_index =
                        instance.tick_rpc_client_index.load(Ordering::SeqCst);
                    let tick_rpc_client = RpcClient::new(String::from(
                        TICK_RPC_ENDPOINTS[tick_rpc_client_index % 2007],
                    ));
                    instance
                        .tick_rpc_client_index
                        .store((tick_rpc_client_index + 1) % 2007, Ordering::SeqCst);

                    let tick_info: Option<TickInfo> = match pair.swap_type
                    {
                        SwapType::RaydiumConcentrated => {
                            BankBot::fetch_raydium_ticks(instance, pool_pubkey, &tick_rpc_client)
                        }
                        SwapType::OrcaWhirlpool => BankBot::fetch_orca_whirlpool_ticks(
                            instance,
                            pool_pubkey,
                            &tick_rpc_client,
                        ),
                        SwapType::CropperWhirlpool => BankBot::fetch_cropper_whirlpool_ticks(
                            instance,
                            pool_pubkey,
                            &tick_rpc_client,
                        ),
                        SwapType::InvariantSwap => {
                            BankBot::fetch_invariant_ticks(instance, pool_pubkey, &tick_rpc_client)
                        }
                        SwapType::CremaFinance => {
                            BankBot::fetch_crema_ticks(instance, pool_pubkey, &tick_rpc_client)
                        }
                        SwapType::MeteoraDlmm => {
                            let tick_info = BankBot::fetch_meteora_dlmm_ticks(
                            instance,
                            pool_pubkey,
                            &tick_rpc_client);
                            if tick_info.is_some() {
                                let new_dlmm_ticks:Vec<String> = tick_info.as_ref().unwrap().ticks.iter().map(|tick| tick.address.clone()).collect();
                                dlmm_ticks.extend(new_dlmm_ticks);
                            }
                            tick_info
                        },
                        _ => None,
                    };
                    if tick_info.is_some() && !tick_info.clone().unwrap().ticks.is_empty() {
                        // println!("get tick info from onchain, swapType: {:#?}, pool: {:#?}, tick_info: {:#?}", pair.swapType, pair.pool, tick_info.clone().unwrap());
                        // println!("get tick info from onchain, swapType: {:#?}, pool: {:#?}", pair.swapType, pair.pool);
                        tick_arrays.push(tick_info.unwrap());

                        break;
                    } else {
                        if retrying_cnt > 3 {
                            
                            println!(
                                "failed to get ticks-> pair.swapType: {:#?}, pool_pubkey: {:#?}",
                                pair.swap_type, pool_pubkey
                            );
                            break;
                        }
                        else {
                            retrying_cnt += 1;
                        }
                        
                        // if pool_pubkey
                        //     .to_string()
                        //     .eq("yEuKWog8fBZ25JBRYrXs6igynxxiythd5zSsfUTSWwa") ||
                        //     pool_pubkey
                        //     .to_string()
                        //     .eq("FPbp7mWLzqsjUDRY1ujfh5dNaSR7nzym7rV8P8GhbjcZ")||
                        //     pool_pubkey
                        //     .to_string()
                        //     .eq("7GZXejXCyJ3R78d71328wqMWT7Ejwu7pxxWRWvHSeVfb")
                        // {
                        //     // for now this has no ticks-> invariant
                        //     break;
                        // }
                        // println!(
                        //     "retrying to get ticks-> pair.swapType: {:#?}, pool_pubkey: {:#?}",
                        //     pair.swap_type, pool_pubkey
                        // );
                    }
                }
                // tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            }
            if !instance.is_running.load(Ordering::SeqCst) {
                break;
            }
            // let all_tick_addresses: Vec<&String> = tick_arrays
            //     .iter()
            //     .flat_map(|tick| tick.ticks.iter().map(|t| &t.address))
            //     .collect();
            // let missed_tick_addresses: Vec<&String> = all_tick_addresses
            //     .iter()
            //     .filter_map(|&addr| {
            //         if BankBot::is_exist_in_lookuptable(instance, addr) {
            //             None
            //         } else {
            //             Some(addr)
            //         }
            //     })
            //     .collect();
            // if !missed_tick_addresses.is_empty() {
            //     println!("inserting missed {} ticks to lookup table", missed_tick_addresses.len());
            //     BankBot::insert_address_to_lookuptable(instance, &missed_tick_addresses).await;
            // }
            // read origin pool addresses
            // let mut total_addresses_to_subscribe: Vec<String> = Vec::new();
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
            //             total_addresses_to_subscribe = res;
            //         }
            //         Err(_err) => println!("accounts_to_subscribe read error-> {:#?}", _err),
            //     };
            // };
            // total_addresses_to_subscribe.extend(dlmm_ticks);

            // save bin arrays separately
            // let serialized_dlmm_content = serde_json::to_string(&*total_addresses_to_subscribe).unwrap();
            // if let Err(err) = spawn_blocking(move || {
            //     let mut bin_arrays_file = OpenOptions::new()
            //         .write(true)
            //         .truncate(true)
            //         .open(TOTAL_ADDRESSES_TO_SUB_FILE_PATH)?;
            //     bin_arrays_file.write_all(serialized_dlmm_content.as_bytes())?;
            //     bin_arrays_file.flush()?;
            //     println!("saved result bin arrays to file");
            //     Ok::<_, std::io::Error>(())
            // })
            // .await
            // {
            //     println!("Error writing ticks to file: {:#?}", err);
            // }

            // println!("saved dlmm bin arrays");

            // save all ticks
            let serialized_content = serde_json::to_string(&*tick_arrays).unwrap();
            instance
                .ticks
                .store(Box::into_raw(tick_arrays), Ordering::SeqCst);
            println!("saving result ticks to file");
            if let Err(err) = spawn_blocking(move || {
                let mut ticks_file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(TICK_FILE_PATH)?;
                ticks_file.write_all(serialized_content.as_bytes())?;
                ticks_file.flush()?;
                println!("saved result ticks to file");
                Ok::<_, std::io::Error>(())
            })
            .await
            {
                println!("Error writing ticks to file: {:#?}", err);
            }
            // Sleep to avoid tight loop
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }
    }

    pub fn is_exist_in_lookuptable(instance: &Arc<Self>, address: &String) -> bool {
        let lookuptables = unsafe { &*instance.lookuptables.load(Ordering::SeqCst) };
        let lookup = lookuptables
            .iter()
            .find(|&lookup| lookup.addresses.contains(&Pubkey::from_str(address).unwrap()));
        lookup.is_some()
    }
    
    pub fn fetch_raydium_ticks(
        instance: &Arc<Self>,
        pool: &Pubkey,
        tick_rpc_client: &RpcClient,
    ) -> Option<TickInfo> {
        let accounts_to_subscribe = unsafe{&* instance.accounts_to_subscribe.load(Ordering::SeqCst)};
        let pool_account_data = match accounts_to_subscribe.get(pool) {
            Some(account) => &account.account_data,
            None => return None,
        };
        // let pool_account_data = match BankBot::get_account_data(instance, pool) {
        //     Some(data) => data,
        //     None => return None,
        // };
        let input = array_ref![pool_account_data.as_slice(), 0, 237];
        let (_, pool_mint_a_slice, pool_mint_b_slice, _, tick_space_slice) =
            array_refs![input, 73, 32, 32, 98, 2];
        let raydium_clmm_program_id = Pubkey::from_str(RAYDIUM_CLMM).unwrap();
        let tick_array_accounts = tick_rpc_client
            .get_program_accounts_with_config(
                &raydium_clmm_program_id,
                RpcProgramAccountsConfig {
                    filters: Some(vec![
                        RpcFilterType::DataSize(10240),
                        RpcFilterType::Memcmp(Memcmp::new(
                            8,
                            MemcmpEncodedBytes::Base58(pool.to_string()),
                        )),
                    ]),
                    account_config: RpcAccountInfoConfig {
                        encoding: Some(UiAccountEncoding::Base64),
                        data_slice: Some(UiDataSliceConfig {
                            offset: 40,
                            length: 4,
                        }),
                        commitment: None,
                        min_context_slot: None,
                    },
                    with_context: None,
                    sort_results: None
                },
            )
            .ok();
        let tick_arrays = tick_array_accounts
            .map(|accounts| {
                accounts
                    .into_iter()
                    .map(|(addr, acc)| {
                        let tick_index_slice = array_ref![acc.data.as_slice(), 0, 4];
                        let tick_index = i32::from_le_bytes(*tick_index_slice);
                        Tick {
                            address: addr.to_string(),
                            index: tick_index,
                        }
                    })
                    .collect::<Vec<Tick>>()
            })
            .unwrap_or_default();
        Some(TickInfo {
            pool: pool.to_string(),
            pool_mint_a: Pubkey::new_from_array(*pool_mint_a_slice).to_string(),
            pool_mint_b: Pubkey::new_from_array(*pool_mint_b_slice).to_string(),
            tick_spacing: u16::from_le_bytes(*tick_space_slice) as i32,
            ticks: tick_arrays,
        })
    }

    pub fn fetch_orca_whirlpool_ticks(
        instance: &Arc<Self>,
        pool: &Pubkey,
        tick_rpc_client: &RpcClient,
    ) -> Option<TickInfo> {
        // Fetch pool account data
        let accounts_to_subscribe = unsafe{&* instance.accounts_to_subscribe.load(Ordering::SeqCst)};
        let pool_account_data = match accounts_to_subscribe.get(pool) {
            Some(account) => &account.account_data,
            None => return None,
        };
        let input = array_ref![pool_account_data.as_slice(), 0, 245];
        let (_, tick_space_slice, _, pool_mint_a_slice, _, pool_mint_b_slice, _) =
            array_refs![input, 41, 2, 58, 32, 48, 32, 32];

        let orca_whirlpool_program_id = Pubkey::from_str(ORCA_WHIRLPOOL).unwrap();

        // Fetch tick array accounts
        let tick_array_accounts = tick_rpc_client
            .get_program_accounts_with_config(
                &orca_whirlpool_program_id,
                RpcProgramAccountsConfig {
                    filters: Some(vec![
                        RpcFilterType::DataSize(9988),
                        RpcFilterType::Memcmp(Memcmp::new(
                            9956,
                            MemcmpEncodedBytes::Base58(pool.to_string()),
                        )),
                    ]),
                    account_config: RpcAccountInfoConfig {
                        encoding: Some(UiAccountEncoding::Base64),
                        data_slice: Some(UiDataSliceConfig {
                            offset: 8,
                            length: 4,
                        }),
                        commitment: None,
                        min_context_slot: None,
                    },
                    with_context: None,
                    sort_results: None
                },
            )
            .ok();
        // Parse tick data if available, else return an empty list
        let tick_arrays = tick_array_accounts
            .map(|accounts| {
                accounts
                    .into_iter()
                    .map(|(addr, acc)| {
                        let tick_index_slice = array_ref![acc.data.as_slice(), 0, 4];
                        let tick_index = i32::from_le_bytes(*tick_index_slice);
                        Tick {
                            address: addr.to_string(),
                            index: tick_index,
                        }
                    })
                    .collect::<Vec<Tick>>()
            })
            .unwrap_or_default();
        // Construct TickInfo
        Some(TickInfo {
            pool: pool.to_string(),
            pool_mint_a: Pubkey::new_from_array(*pool_mint_a_slice).to_string(),
            pool_mint_b: Pubkey::new_from_array(*pool_mint_b_slice).to_string(),
            tick_spacing: u16::from_le_bytes(*tick_space_slice) as i32,
            ticks: tick_arrays,
        })
    }

    pub fn fetch_cropper_whirlpool_ticks(
        instance: &Arc<Self>,
        pool: &Pubkey,
        tick_rpc_client: &RpcClient,
    ) -> Option<TickInfo> {
        // Fetch pool account data
        let accounts_to_subscribe = unsafe{&* instance.accounts_to_subscribe.load(Ordering::SeqCst)};
        let pool_account_data = match accounts_to_subscribe.get(pool) {
            Some(account) => &account.account_data,
            None => return None,
        };

        let input = array_ref![pool_account_data.as_slice(), 0, 245];
        let (_, tick_space_slice, _, pool_mint_a_slice, _, pool_mint_b_slice, _) =
            array_refs![input, 41, 2, 58, 32, 48, 32, 32];

        let cropper_whirlpool_program_id = Pubkey::from_str(CROPPER_WHIRLPOOL).unwrap();
        // Fetch tick array accounts
        let tick_array_accounts = tick_rpc_client
            .get_program_accounts_with_config(
                &cropper_whirlpool_program_id,
                RpcProgramAccountsConfig {
                    filters: Some(vec![
                        RpcFilterType::DataSize(9988),
                        RpcFilterType::Memcmp(Memcmp::new(
                            9956,
                            MemcmpEncodedBytes::Base58(pool.to_string()),
                        )),
                    ]),
                    account_config: RpcAccountInfoConfig {
                        encoding: Some(UiAccountEncoding::Base64),
                        data_slice: Some(UiDataSliceConfig {
                            offset: 8,
                            length: 4,
                        }),
                        commitment: None,
                        min_context_slot: None,
                    },
                    with_context: None,
                    sort_results: None
                },
            )
            .ok();
        // Parse tick data if available, else return an empty list
        let tick_arrays = tick_array_accounts
            .map(|accounts| {
                accounts
                    .into_iter()
                    .map(|(addr, acc)| {
                        let tick_index_slice = array_ref![acc.data.as_slice(), 0, 4];
                        let tick_index = i32::from_le_bytes(*tick_index_slice);
                        Tick {
                            address: addr.to_string(),
                            index: tick_index,
                        }
                    })
                    .collect::<Vec<Tick>>()
            })
            .unwrap_or_default();
        // Construct TickInfo
        Some(TickInfo {
            pool: pool.to_string(),
            pool_mint_a: Pubkey::new_from_array(*pool_mint_a_slice).to_string(),
            pool_mint_b: Pubkey::new_from_array(*pool_mint_b_slice).to_string(),
            tick_spacing: u16::from_le_bytes(*tick_space_slice) as i32,
            ticks: tick_arrays,
        })
    }

    pub fn fetch_invariant_ticks(
        instance: &Arc<Self>,
        pool: &Pubkey,
        tick_rpc_client: &RpcClient,
    ) -> Option<TickInfo> {
        // Fetch pool account data
        let accounts_to_subscribe = unsafe{&* instance.accounts_to_subscribe.load(Ordering::SeqCst)};
        let pool_account_data = match accounts_to_subscribe.get(pool) {
            Some(account) => &account.account_data,
            None => return None,
        };

        let input = array_ref![pool_account_data.as_slice(), 0, 154];
        let (_, pool_mint_a_slice, pool_mint_b_slice, _, tick_space_slice) =
            array_refs![input, 8, 32, 32, 80, 2];

        let invariant_swap_program_id = Pubkey::from_str(INVARIANT_SWAP).unwrap();
        // Fetch tick array accounts
        let tick_array_accounts = tick_rpc_client
            .get_program_accounts_with_config(
                &invariant_swap_program_id,
                RpcProgramAccountsConfig {
                    filters: Some(vec![
                        RpcFilterType::DataSize(150),
                        RpcFilterType::Memcmp(Memcmp::new(
                            8,
                            MemcmpEncodedBytes::Base58(pool.to_string()),
                        )),
                    ]),
                    account_config: RpcAccountInfoConfig {
                        encoding: Some(UiAccountEncoding::Base64),
                        data_slice: Some(UiDataSliceConfig {
                            offset: 40,
                            length: 4,
                        }),
                        commitment: None,
                        min_context_slot: None,
                    },
                    with_context: None,
                    sort_results: None
                },
            )
            .ok();
        // Parse tick data if available, else return an empty list
        let tick_arrays = tick_array_accounts
            .map(|accounts| {
                accounts
                    .into_iter()
                    .map(|(addr, acc)| {
                        let tick_index_slice = array_ref![acc.data.as_slice(), 0, 4];
                        let tick_index = i32::from_le_bytes(*tick_index_slice);
                        Tick {
                            address: addr.to_string(),
                            index: tick_index,
                        }
                    })
                    .collect::<Vec<Tick>>()
            })
            .unwrap_or_default();
        // Construct TickInfo
        Some(TickInfo {
            pool: pool.to_string(),
            pool_mint_a: Pubkey::new_from_array(*pool_mint_a_slice).to_string(),
            pool_mint_b: Pubkey::new_from_array(*pool_mint_b_slice).to_string(),
            tick_spacing: u16::from_le_bytes(*tick_space_slice) as i32,
            ticks: tick_arrays,
        })
    }

    pub fn fetch_crema_ticks(
        instance: &Arc<Self>,
        pool: &Pubkey,
        tick_rpc_client: &RpcClient,
    ) -> Option<TickInfo> {
        // Fetch pool account data
        let accounts_to_subscribe = unsafe{&* instance.accounts_to_subscribe.load(Ordering::SeqCst)};
        let pool_account_data = match accounts_to_subscribe.get(pool) {
            Some(account) => &account.account_data,
            None => return None,
        };

        let input = array_ref![pool_account_data.as_slice(), 0, 170];
        let (_, pool_mint_a_slice, pool_mint_b_slice, _, tick_space_slice) =
            array_refs![input, 40, 32, 32, 64, 2];

        let crema_finance_program_id = Pubkey::from_str(CREMA_FINANCE).unwrap();
        // Fetch tick array accounts
        let tick_array_accounts = tick_rpc_client
            .get_program_accounts_with_config(
                &crema_finance_program_id,
                RpcProgramAccountsConfig {
                    filters: Some(vec![
                        RpcFilterType::DataSize(8556),
                        RpcFilterType::Memcmp(Memcmp::new(
                            12,
                            MemcmpEncodedBytes::Base58(pool.to_string()),
                        )),
                    ]),
                    account_config: RpcAccountInfoConfig {
                        encoding: Some(UiAccountEncoding::Base64),
                        data_slice: Some(UiDataSliceConfig {
                            offset: 8,
                            length: 2,
                        }),
                        commitment: None,
                        min_context_slot: None,
                    },
                    with_context: None,
                    sort_results: None
                },
            )
            .ok();
        // Parse tick data if available, else return an empty list
        let tick_arrays = tick_array_accounts
            .map(|accounts| {
                accounts
                    .into_iter()
                    .map(|(addr, acc)| {
                        let tick_index_slice = array_ref![acc.data.as_slice(), 0, 2];
                        let tick_index = u16::from_le_bytes(*tick_index_slice) as i32;
                        Tick {
                            address: addr.to_string(),
                            index: tick_index,
                        }
                    })
                    .collect::<Vec<Tick>>()
            })
            .unwrap_or_default();
        // Construct TickInfo
        Some(TickInfo {
            pool: pool.to_string(),
            pool_mint_a: Pubkey::new_from_array(*pool_mint_a_slice).to_string(),
            pool_mint_b: Pubkey::new_from_array(*pool_mint_b_slice).to_string(),
            tick_spacing: u16::from_le_bytes(*tick_space_slice) as i32,
            ticks: tick_arrays,
        })
    }

    pub fn fetch_meteora_dlmm_ticks(
        instance: &Arc<Self>,
        pool: &Pubkey,
        tick_rpc_client: &RpcClient,
    ) -> Option<TickInfo> {
        // Fetch pool account data
        let accounts_to_subscribe = unsafe{&* instance.accounts_to_subscribe.load(Ordering::SeqCst)};
        let pool_account_data = match accounts_to_subscribe.get(pool) {
            Some(account) => &account.account_data,
            None => return None,
        };

        let input = array_ref![pool_account_data.as_slice(), 0, 152];
        let (_, tick_space_slice, _, pool_mint_a_slice, pool_mint_b_slice) =
            array_refs![input, 80, 2, 6, 32, 32];

        let meteora_dlmm_program_id = Pubkey::from_str(METEORA_DLMM).unwrap();
        // Fetch tick array accounts
        let tick_array_accounts = tick_rpc_client
            .get_program_accounts_with_config(
                &meteora_dlmm_program_id,
                RpcProgramAccountsConfig {
                    filters: Some(vec![
                        RpcFilterType::DataSize(10136),
                        RpcFilterType::Memcmp(Memcmp::new(
                            24,
                            MemcmpEncodedBytes::Base58(pool.to_string()),
                        )),
                    ]),
                    account_config: RpcAccountInfoConfig {
                        encoding: Some(UiAccountEncoding::Base64),
                        data_slice: Some(UiDataSliceConfig {
                            offset: 8,
                            length: 8,
                        }),
                        commitment: None,
                        min_context_slot: None,
                    },
                    with_context: None,
                    sort_results: None
                },
            )
            .ok();
        // Parse tick data if available, else return an empty list
        let tick_arrays = tick_array_accounts
            .map(|accounts| {
                accounts
                    .into_iter()
                    .map(|(addr, acc)| {
                        let tick_index_slice = array_ref![acc.data.as_slice(), 0, 8];
                        let tick_index = i64::from_le_bytes(*tick_index_slice) as i32;
                        Tick {
                            address: addr.to_string(),
                            index: tick_index,
                        }
                    })
                    .collect::<Vec<Tick>>()
            })
            .unwrap_or_default();
        // Construct TickInfo
        Some(TickInfo {
            pool: pool.to_string(),
            pool_mint_a: Pubkey::new_from_array(*pool_mint_a_slice).to_string(),
            pool_mint_b: Pubkey::new_from_array(*pool_mint_b_slice).to_string(),
            tick_spacing: u16::from_le_bytes(*tick_space_slice) as i32,
            ticks: tick_arrays,
        })
    }

    pub async fn insert_address_to_lookuptable(instance: &Arc<Self>, addresses: &[&String]) {
        println!("inserting started!...");
        let cu_ix = ComputeBudgetInstruction::set_compute_unit_limit(2_000);
        let cu_price_ix = ComputeBudgetInstruction::set_compute_unit_price(10);
        let lookuptables = unsafe { &mut *instance.lookuptables.load(Ordering::SeqCst) };
        // trace!();
        let mut current_address_index: usize = 0;
        while current_address_index < addresses.len() {
            let lookuptable_len = lookuptables.len();
            let mut last_lookuptable_keys_length = lookuptables[lookuptable_len - 1].addresses.len();
            while last_lookuptable_keys_length < 255 {
                if current_address_index >= addresses.len() {
                    return;
                }

                let keys_to_extent_last_index = if last_lookuptable_keys_length + 28 > 255 {
                    let last_index = 255 - last_lookuptable_keys_length + current_address_index;
                    if last_index > addresses.len() {
                        addresses.len()
                    } else {
                        last_index
                    }
                } else if current_address_index + 28 > addresses.len() {
                    addresses.len()
                } else {
                    current_address_index + 28
                };
                let keys_to_extend: Vec<Pubkey> = addresses
                    [current_address_index..keys_to_extent_last_index]
                    .iter()
                    .map(|&s| Pubkey::from_str(s).unwrap())
                    .collect();
                // println!("keys_to_extend: {:#?}", keys_to_extend);
                let extend_instruction = address_lookup_table::instruction::extend_lookup_table(
                    lookuptables[lookuptable_len - 1].key,
                    instance.payer.pubkey(),
                    Some(instance.payer.pubkey()),
                    keys_to_extend.clone(),
                );

                loop {
                    let tick_rpc_client_index =
                        instance.tick_rpc_client_index.load(Ordering::SeqCst);
                    let tick_rpc_client: RpcClient = RpcClient::new(String::from(
                        TICK_RPC_ENDPOINTS[tick_rpc_client_index % 2007],
                    ));
                    instance
                        .tick_rpc_client_index
                        .store((tick_rpc_client_index + 1) % 2007, Ordering::SeqCst);

                    if let Ok(_sig) = instance.create_and_send_ix_with_rpc(
                        &Vec::from([
                            cu_ix.clone(),
                            cu_price_ix.clone(),
                            extend_instruction.clone(),
                        ]),
                        &Vec::new(),
                        true,
                        tick_rpc_client
                    ) {
                        println!("here sent the extend tx-> {:#?}", _sig);
                        break;
                    } else {
                        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                    }
                }

                let waiting_start_time = ArcBankBot::get_current_timestamp();
                let mut lookuptable_data = instance
                    .rpc_client
                    .get_account_data(
                        &lookuptables[lookuptable_len - 1].key,
                    )
                    .unwrap();
                let mut lookuptable_info =
                    address_lookup_table::state::AddressLookupTable::deserialize(&lookuptable_data)
                        .unwrap();

                while lookuptable_info.addresses.len()
                    <= lookuptables[lookuptable_len - 1].addresses.len()
                {
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                    lookuptable_data = instance
                        .rpc_client
                        .get_account_data(
                            &lookuptables[lookuptable_len - 1].key,
                        )
                        .unwrap();
                    lookuptable_info =
                        address_lookup_table::state::AddressLookupTable::deserialize(
                            &lookuptable_data,
                        )
                        .unwrap();
                    if ArcBankBot::get_current_timestamp() - waiting_start_time > 5 * 60 * 1000 {
                        println!("failed to send and confirm extending lookuptable tx");
                        return;
                    }
                }

                println!(
                    "successfully extend lookuptable!, lookuptable={:#?}",
                    lookuptables[lookuptable_len - 1].key
                );

                // save
                lookuptables[lookuptable_len - 1]
                    .addresses
                    .extend(keys_to_extend.iter());

                if let Err(err) = spawn_blocking({
                    let lookuptables = lookuptables.to_vec();
                    move || {
                        let mut lookuptables_file = OpenOptions::new()
                            .write(true)
                            .truncate(true)
                            .open(LOOKUPTABLE_FILE_PATH)?;
                        let lookups: Vec<LookupTable> = lookuptables.iter().map(|lookup| LookupTable {
                            lookuptable: lookup.key.to_string(),
                            keys: lookup.addresses.iter().map(|address| address.to_string()).collect()
                        }).collect();
                        let content = serde_json::to_string(&lookups)?;

                        lookuptables_file.write_all(content.as_bytes())?;
                        lookuptables_file.flush()?;
                        Ok::<_, std::io::Error>(())
                    }
                })
                .await
                {
                    println!("Error writing lookuptables to file: {:#?}", err);
                }

                last_lookuptable_keys_length += keys_to_extend.len();
                current_address_index += keys_to_extend.len();
            }

            let slot = instance.rpc_client.get_slot().unwrap();
            let (lookuptable_ix, lookuptable_address) =
                address_lookup_table::instruction::create_lookup_table_signed(
                    instance.payer.pubkey(),
                    instance.payer.pubkey(),
                    slot,
                );

            loop {
                let tick_rpc_client_index =
                        instance.tick_rpc_client_index.load(Ordering::SeqCst);
                let tick_rpc_client: RpcClient = RpcClient::new(String::from(
                    TICK_RPC_ENDPOINTS[tick_rpc_client_index % 2007],
                ));
                instance
                    .tick_rpc_client_index
                    .store((tick_rpc_client_index + 1) % 2007, Ordering::SeqCst);

                if let Ok(_sig) = BankBot::create_and_send_ix_with_rpc(
                    instance,
                    &Vec::from([cu_ix.clone(), cu_price_ix.clone(), lookuptable_ix.clone()]),
                    &Vec::new(),
                    true,
                    tick_rpc_client
                ) {
                    println!("here sent the create tx-> {:#?}", _sig);
                    break;
                } else {
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                }
            }

            let waiting_start_time = ArcBankBot::get_current_timestamp();
            let mut lookuptable_data = match instance.rpc_client.get_account(&lookuptable_address) {
                Ok(acc) => Some(acc),
                Err(_err) => None,
            };

            while lookuptable_data.is_none() {
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                lookuptable_data = match instance.rpc_client.get_account(&lookuptable_address) {
                    Ok(acc) => Some(acc),
                    Err(_err) => None,
                };
                if ArcBankBot::get_current_timestamp() - waiting_start_time > 5 * 60 * 1000 {
                    println!("failed to send and confirm creating lookuptable tx");
                    return;
                }
            }

            println!(
                "successfully created lookuptable!, lookuptable={:#?}",
                lookuptable_address
            );

            lookuptables.push(AddressLookupTableAccount {
                key: lookuptable_address,
                addresses: Vec::new(),
            });

            if let Err(err) = spawn_blocking({
                let lookuptables = lookuptables.to_vec();
                move || {
                    let mut lookuptables_file = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(LOOKUPTABLE_FILE_PATH)?;
                    let lookups: Vec<LookupTable> = lookuptables.iter().map(|lookup| LookupTable {
                        lookuptable: lookup.key.to_string(),
                        keys: lookup.addresses.iter().map(|address| address.to_string()).collect()
                    }).collect();
                    let content = serde_json::to_string(&lookups)?;

                    lookuptables_file.write_all(content.as_bytes())?;
                    lookuptables_file.flush()?;
                    Ok::<_, std::io::Error>(())
                }
            })
            .await
            {
                println!("Error writing lookuptables to file: {:#?}", err);
            }
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct FinalPriceLog {
    INIT_PRICE: u128,
    LAST_PRICE: u128,
    TARG_PRICE: u128,
    AMOUNT__IN: u64,
    P___N____L: u64,
    START_MINT: String,
}
#[derive(Clone)]
pub struct PairChangeItem {
    pub last_time: std::time::Instant,
    pub pair_idx: usize,
    pub delta: u128
}
pub struct PairChangeStack {
    pub items: Vec<PairChangeItem>,
    pub processed_pairs: Vec<usize>,
    pub draining: AtomicBool
}
impl PairChangeStack {
    pub fn default() -> Self {
        Self {
            items: Vec::new(),
            draining: AtomicBool::new(false),
            processed_pairs: Vec::new()
        }
    }
}
pub struct ArcBankBot {
    pub bank_bot: Arc<BankBot>
}

impl ArcBankBot {
    pub fn push_pair_change(&self, pair_idx: usize, delta: u128) {
        if delta > 0 {
            let stack = unsafe{&mut * self.bank_bot.pair_change_stack.load(Ordering::SeqCst)};
        
            stack.items.push(PairChangeItem {
                last_time: std::time::Instant::now(),
                pair_idx,
                delta
            });
            if !stack.draining.load(Ordering::SeqCst) {
                stack.draining.store(true, Ordering::SeqCst);
                for i in 0..stack.items.len() {
                    if stack.items[i].last_time.elapsed().as_millis() < STACK_ELAPSED_MILLIS {
                        // remove old items
                        if i > 0 {
                            stack.items.drain(0..i);
                        }
                        break;
                    }
                }
                stack.draining.store(false, Ordering::SeqCst);
            }
            
        }
        
    }
    pub fn pop_pair_change(&self) -> usize {
        let stack_origin = unsafe{&mut * self.bank_bot.pair_change_stack.load(Ordering::SeqCst)};
        let stack_items = stack_origin.items.clone();
        let stack_len = stack_items.len();
        if stack_items.len() == 0 {
            return usize::MAX;
        }
        let mut max_i = usize::MAX;
        let mut max_delta = 0;
        for i in 0..stack_len {
            let item = &stack_items[i];
            if /*max_delta < item.delta &&*/ !stack_origin.processed_pairs.contains(&item.pair_idx) {
                max_delta = item.delta;
                max_i = i;
            }
        }
        if max_i == usize::MAX {
            return usize::MAX;
        }
        let pair_idx = stack_items[max_i].pair_idx;
        // let delta = stack_items[max_i].delta;

        let processed_pairs_len = stack_origin.processed_pairs.len();
        if processed_pairs_len > 10 {
            stack_origin.processed_pairs.drain(0..(processed_pairs_len - 10));
        }
        stack_origin.processed_pairs.push(pair_idx);
        // println!("pair_idx {}, delta {}, stack size {}", pair_idx, delta, stack_len);
        pair_idx
    }
    pub fn get_pnl_from_sim_logs(logs: &Vec<String>) -> u64 {
        let mut log_final: &str = "";

        for msg in logs.iter() {
            if msg.contains(PREFIX_FINAL) {
                log_final = msg;
            }
        }
        let final_log = &mut log_final.to_string()[PREFIX_FINAL.len()..];

        let final_result =
            Self::decode_final_json_log(final_log)
                .unwrap();

        let final_pnl = if final_result.START_MINT.eq("S") {
            final_result.P___N____L
        } else {
            final_result.P___N____L * 5
        };
        final_pnl
    }
    pub fn decode_final_json_log(log: &mut str) -> Option<FinalPriceLog> {
        let log_bytes = unsafe { log.as_bytes_mut() };

        for byte in log_bytes.iter_mut() {
            *byte = ((*byte as u16 + 256 - 10) % 256) as u8;
        }

        let log_str = String::from_utf8_lossy(log_bytes)
            .replace("[", "{")
            .replace("]", "}");
        serde_json::from_str::<FinalPriceLog>(&log_str).ok()
    }
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
            let is_running = self.bank_bot.is_running.load(Ordering::SeqCst);
            if is_running {
                let max_retry = 5;
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
                        if retry > max_retry {
                            break;
                        }
                    }
    
                    if hash_tmp.is_some() {
                        self.bank_bot.last_blockhash_timestamp.store(Self::get_current_timestamp(), Ordering::SeqCst);
                        self.bank_bot.latest_blockhash.store(Box::into_raw(Box::new(hash_tmp)), Ordering::SeqCst);
                    }
                    // println!("latest blockhash bot running, current-> {:#?}", hash_tmp);
                }
            }
            
            std::thread::sleep(Duration::from_millis(200));
        }
    }

    pub fn account_update(&self, pubkey: &Pubkey, account_data: &[u8]) { 
        let accounts_to_subscribe = unsafe{&mut * self.bank_bot.accounts_to_subscribe.load(Ordering::SeqCst)};
        let account_subscribe = accounts_to_subscribe.get_mut(pubkey).unwrap();
        account_subscribe.account_data = account_data.to_vec();

        let instance = self.get_arc_clone();
        std::thread::spawn(move || {
            let pair_index_res: Option<usize> = account_subscribe.pair_index;
            if pair_index_res.is_none() {
                return;
            }
            let pair_index = account_subscribe.pair_index.unwrap();
            let delta_price = instance.build_pair_price(pair_index);
            if delta_price < 5 {
                return;
            }
            // println!("delta_price {}", delta_price);
            // instance.push_pair_change(pair_index, delta_price);

            
                // let pairs = unsafe { &*instance.bank_bot.pairs.load(Ordering::SeqCst) };
                // let pair = &pairs[pair_index];
                // if pair.swap_type == SwapType::MeteoraDlmm {
                    let rpc_health = instance.bank_bot.rpc_client.get_health();
                    let process_cnt = instance.bank_bot.process_pair_cnt.load(Ordering::SeqCst);
                    let max_process_cnt = if DIRECT_SEND {10} else {3};
                    if  process_cnt < max_process_cnt && rpc_health.is_ok() {
                        instance.bank_bot.process_pair_cnt.store(process_cnt + 1, Ordering::SeqCst);
                        let instance_clone = instance.get_arc_clone();

                        // let top_pair_index = instance.pop_pair_change();
                        let top_pair_index = pair_index;
                        if top_pair_index < usize::MAX {
                            std::thread::spawn(move|| {
                                let ((routes_len, runnable_cnt, run_cnt), elapsed) = measure_us!(instance_clone.process_pair_change(top_pair_index));
                                
                                // if run_cnt > 0 {
                                //     println!("run time: {} us, total {}, runnable {}, run {}", elapsed, routes_len, runnable_cnt, run_cnt);
                                // }
                            }).join().expect("processing pair thread error");
                        }
                        let process_cnt = instance.bank_bot.process_pair_cnt.load(Ordering::SeqCst);
                        if process_cnt > 0 {
                            instance.bank_bot.process_pair_cnt.store(process_cnt - 1, Ordering::SeqCst);
                        }
                    }
                // }
            
        });
    }
    pub fn account_update_v2(&self, pubkey: &Pubkey, account_data: &[u8]) { 
        let accounts_to_subscribe = unsafe{&mut * self.bank_bot.accounts_to_subscribe.load(Ordering::SeqCst)};
        let account_subscribe = accounts_to_subscribe.get_mut(pubkey).unwrap();
        account_subscribe.account_data = account_data.to_vec();

        let instance = self.get_arc_clone();
        std::thread::spawn(move || {
            
            let pair_index_res: Option<usize> = account_subscribe.pair_index;
            if pair_index_res.is_none() {
                return;
            }
            let pair_index = account_subscribe.pair_index.unwrap();

            // println!("account_update_v2");
            // instance.build_pair_price_v2(pair_index);
            let (_, elapsed_price_calc) = measure_us!(instance.build_pair_price_v2(pair_index));


            let process_cnt = instance.bank_bot.process_pair_cnt.load(Ordering::SeqCst);
            if  process_cnt > 0 {
                return;
            }
            instance.bank_bot.process_pair_cnt.store(process_cnt + 1, Ordering::SeqCst);
            let (run_cnt, elapsed_process_route) = measure_us!(instance.process_routes_v2(pair_index));

            let process_cnt = instance.bank_bot.process_pair_cnt.load(Ordering::SeqCst);
            if  process_cnt > 0 {
                instance.bank_bot.process_pair_cnt.store(process_cnt - 1, Ordering::SeqCst);
            }
            if run_cnt > 0 {
                println!("process time: {:?}, {:?}, {:?}ms", elapsed_price_calc / 1000, elapsed_process_route / 1000, (elapsed_price_calc + elapsed_process_route) / 1000);
            }
            
        });
    }

    pub fn set_run_mode(&self, is_running: bool) {
      self.bank_bot.is_running.store(is_running, Ordering::SeqCst);
      if !is_running {
            self.bank_bot.process_pair_cnt.store(0, Ordering::SeqCst);
            self.bank_bot.is_price_updated_initially.store(false, Ordering::SeqCst);
            self.bank_bot.routes_state.clear();
            println!("bot stopped!");
      }
    }
    pub fn filter_routes_engine(&self, routes_by_pnl: &mut Vec<(usize, u64, u64)>, main_pair_idx: u32) -> Vec<(usize, u64, u64)> {
        let routes = unsafe { &*self.bank_bot.routes.load(Ordering::SeqCst) };

        routes_by_pnl.sort_by(|&(_route_idx_a, _, pnl_a), &(_route_idx_b, _, pnl_b)| pnl_b.cmp(&pnl_a));
        // _filtered_routes_by_pnl.sort_by(|&(route_idx_a, _, pnl_a), &(route_idx_b, _, pnl_b)| (
        //     pnl_a as u128 * 10_000_000 / routes[route_idx_a].cu as u128).cmp(&(pnl_b as u128 * 10_000_000 / routes[route_idx_b].cu as u128)
        // ));
        routes_by_pnl.dedup_by(|a, b| a.2.eq(&b.2));
        
        let mut pair_idxes: Vec<u32> = Vec::new();
        routes_by_pnl.retain(|&route_item| {
            let route = &routes[route_item.0];
            let mut remained = true;
            let mut not_included_list = Vec::new();
            for pair_idx in route.route.iter() {
                if pair_idxes.contains(pair_idx) {
                    remained = false;
                }
                else if *pair_idx != main_pair_idx {
                    not_included_list.push(*pair_idx);
                }
            }
            if remained {
                pair_idxes.extend(not_included_list.iter());
            }
            remained
        });
        routes_by_pnl.to_vec()
    }
    pub fn filter_routes_engine_v2(&self, routes_by_pnl: &mut Vec<(usize, u64, u64)>) -> Vec<(usize, u64, u64)> {
        routes_by_pnl.sort_by(|&(_route_idx_a, _, pnl_a), &(_route_idx_b, _, pnl_b)| pnl_b.cmp(&pnl_a));
        // _filtered_routes_by_pnl.sort_by(|&(route_idx_a, _, pnl_a), &(route_idx_b, _, pnl_b)| (
        //     pnl_a as u128 * 10_000_000 / routes[route_idx_a].cu as u128).cmp(&(pnl_b as u128 * 10_000_000 / routes[route_idx_b].cu as u128)
        // ));
        routes_by_pnl.dedup_by(|a, b| a.2.eq(&b.2));
        
        routes_by_pnl.to_vec()
    }
    pub fn process_routes_v2(&self, pair_idx: usize)  -> i32{
        let routes = unsafe { &*self.bank_bot.routes_v2.load(Ordering::SeqCst) };
        let accounts_to_subscribe = unsafe { &* self.bank_bot.accounts_to_subscribe.load(Ordering::SeqCst)};
        let wsol_ata_pubkey =
            Pubkey::from_str("Gm6TTgZXjATYcpBxkmJnCZbzeeoJ7TMJKwDLMaLFSpE1").unwrap();
        let usdc_ata_pubkey =
            Pubkey::from_str("BChF15Y7PAwEhWNCZxfa6AujRXZiHVTecsbH5CuV1jzD").unwrap();
        // let wsol_token_vault_amount = ArcBankBot::get_amount_from_token_account(&accounts_to_subscribe.get(&wsol_ata_pubkey).unwrap().account_data);
        let wsol_token_vault_amount = 1_000_000_000_000;
        let usdc_token_vault_amount = ArcBankBot::get_amount_from_token_account(&accounts_to_subscribe.get(&usdc_ata_pubkey).unwrap().account_data);

        let mut handles = Vec::new();
        let filtered_routes_by_pnl: Arc<Mutex<Vec<(usize, u64, u64)>>> = Arc::new(Mutex::new(Vec::new()));

        let route_len = routes.len();
        let current_route_idx = Arc::new(AtomicUsize::new(0usize));

        for _ in 0..9 {
            let instance = ArcBankBot {bank_bot: Arc::clone(&self.bank_bot)};
            let filtered_routes_by_pnl_clone = Arc::clone(&filtered_routes_by_pnl);
            let current_route_idx_clone = Arc::clone(&current_route_idx);

            handles.push(std::thread::spawn(move || {
                loop {
                    let route_idx = current_route_idx_clone.load(Ordering::SeqCst);
                    if route_idx >= route_len {
                        break;
                    } else {
                        current_route_idx_clone.store(route_idx+1, Ordering::SeqCst);
                    }
                    let acc_price = instance.get_acc_price_v2(route_idx);
                    // println!("acc price {}", acc_price);
                    if acc_price > PRICE_MULTIPLIER as u128 {
                        let route = &routes[route_idx];
                        if !route.route.contains(&(pair_idx as u32)) {
                            continue;
                        }
                        let (tx_fee, max_amount) = if route.route_start_mint {
                            (5140, wsol_token_vault_amount)
                        } else {
                            (1100, usdc_token_vault_amount)
                        };
                        let (input_amount, _final_price, out_amount) = instance.determine_input_amount_v2(10000, max_amount, route_idx);
                        if out_amount > input_amount {
                            let pnl = out_amount - input_amount;
                            let pnl_base_sol = if route.route_start_mint { pnl } else { pnl * SOL_USDC_RATE / SOL_USDC_RATE_MULTIPLIER  };

                            let present_route_state = RouteState {acc_price, pnl: pnl_base_sol};
                            if instance.bank_bot.routes_state.get(&route_idx).map_or(true, |v| !v.eq(&present_route_state)) {
                                let mut filtered_routes = filtered_routes_by_pnl_clone.lock().unwrap();
                                filtered_routes.push((route_idx, input_amount, pnl_base_sol));
                                instance.bank_bot.routes_state.insert(route_idx, present_route_state);
                            }
                        }
                    }
                }
            }));
        }
        for h in handles {
            h.join().expect("acc price and pnl thread panic");
        }

        let mut _filtered_routes_by_pnl = filtered_routes_by_pnl.lock().unwrap();

        let mut runnable_routes: Vec<(usize, u64, u64)> = self.filter_routes_engine_v2(&mut _filtered_routes_by_pnl);

        let mut run_cnt = 0;
        

        // let mut run_route_handles = Vec::new();
        // if runnable_routes.len() > 0 {
        //     runnable_routes.remove(0);
        // }
        for (route_idx, input_amount, _pnl) in runnable_routes {
            
            
            let instance = ArcBankBot{bank_bot: Arc::clone(&self.bank_bot)};
            // for SimulateAndBundle
            // run_route_handles.push(std::thread::spawn(move || {
            //     let sent = instance.run_route_v2(route_idx, input_amount, _pnl);
            // }));

            if instance.run_route_v2(route_idx, input_amount, _pnl) {
                run_cnt += 1;
                break;
            }
        }
        run_cnt
    }
    pub fn process_pair_change(&self, pair_idx: usize) -> (usize, usize, usize) {
        // calculate acc prices of a group;
        trace!();
        let group = unsafe{&* self.bank_bot.groups.load(Ordering::SeqCst)};
        let routes = unsafe { &*self.bank_bot.routes.load(Ordering::SeqCst) };
        let accounts_to_subscribe = unsafe { &* self.bank_bot.accounts_to_subscribe.load(Ordering::SeqCst)};

        let wsol_ata_pubkey =
            Pubkey::from_str("Gm6TTgZXjATYcpBxkmJnCZbzeeoJ7TMJKwDLMaLFSpE1").unwrap();
        let usdc_ata_pubkey =
            Pubkey::from_str("BChF15Y7PAwEhWNCZxfa6AujRXZiHVTecsbH5CuV1jzD").unwrap();
        let wsol_token_vault_amount = ArcBankBot::get_amount_from_token_account(&accounts_to_subscribe.get(&wsol_ata_pubkey).unwrap().account_data);
        let usdc_token_vault_amount = ArcBankBot::get_amount_from_token_account(&accounts_to_subscribe.get(&usdc_ata_pubkey).unwrap().account_data);

        let mut handles = Vec::new();
        let filtered_routes_by_pnl: Arc<Mutex<Vec<(usize, u64, u64)>>> = Arc::new(Mutex::new(Vec::new()));
        trace!();
        //   for route_idx in group[pair_idx].iter() {
        let route_len = group[pair_idx].len();
        let current_route_idx = Arc::new(AtomicUsize::new(0usize));

        for _ in 0..40 {
            let instance = ArcBankBot {bank_bot: Arc::clone(&self.bank_bot)};
            let filtered_routes_by_pnl_clone = Arc::clone(&filtered_routes_by_pnl);
            let current_route_idx_clone = Arc::clone(&current_route_idx);

            handles.push(std::thread::spawn(move || {
                let groups = unsafe{&* instance.bank_bot.groups.load(Ordering::SeqCst)};
                let cur_group = &groups[pair_idx];
                loop {
                    trace!();
                    let idx_in_group = current_route_idx_clone.load(Ordering::SeqCst);
                    if idx_in_group >= route_len {
                        break;
                    } else {
                        current_route_idx_clone.store(idx_in_group+1, Ordering::SeqCst);
                    }
                    let route_idx = cur_group[idx_in_group] as usize;
                    let acc_price = instance.get_acc_price(route_idx);
                    if acc_price > PRICE_MULTIPLIER as u128 {
                        let route = &routes[route_idx];
                        let (tx_fee, max_amount) = if route.route_start_mint {
                            (5140, wsol_token_vault_amount)
                        } else {
                            (1100, usdc_token_vault_amount)
                        };
                        trace!();
                        let (input_amount, _final_price, out_amount) = instance.determine_input_amount(10000, max_amount, route_idx);
                        if out_amount > input_amount {
                            let pnl = out_amount - input_amount;
                            let pnl_base_sol = if route.route_start_mint { pnl } else { pnl * SOL_USDC_RATE / SOL_USDC_RATE_MULTIPLIER  };

                            let present_route_state = RouteState {acc_price, pnl: pnl_base_sol};
                            if instance.bank_bot.routes_state.get(&route_idx).map_or(true, |v| !v.eq(&present_route_state)) {
                                let mut filtered_routes = filtered_routes_by_pnl_clone.lock().unwrap();
                                filtered_routes.push((route_idx, input_amount, pnl_base_sol));
                                instance.bank_bot.routes_state.insert(route_idx, present_route_state);
                            }
                        }
                    }
                }
            }));
        }
        for h in handles {
            h.join().expect("acc price and pnl thread panic");
        }

        let mut _filtered_routes_by_pnl = filtered_routes_by_pnl.lock().unwrap();

        let runnable_routes: Vec<(usize, u64, u64)> = self.filter_routes_engine(&mut _filtered_routes_by_pnl, pair_idx as u32);
        // let runnable_routes: Vec<(usize, u64, u64)> = if _filtered_routes_by_pnl.len() < 50 {
        //     _filtered_routes_by_pnl.to_vec()
        // } else {
        //     _filtered_routes_by_pnl.choose_multiple(&mut rand::thread_rng(), 50).cloned().collect()
        // };
        // println!("runnable_routes {}", runnable_routes.len());
        let mut run_cnt = 0;
        

        let mut run_route_handles = Vec::new();
        
        for (route_idx, input_amount, _pnl) in runnable_routes {
            run_cnt += 1;
            
            let instance = ArcBankBot{bank_bot: Arc::clone(&self.bank_bot)};
            if DIRECT_SEND {
                // for SimulateAndDirectTx
                if instance.run_route(route_idx, input_amount, _pnl) {
                    break;
                }
            }
            else {
                // for SimulateAndBundle
                run_route_handles.push(std::thread::spawn(move || {
                    instance.run_route(route_idx, input_amount, _pnl);
                }));
            }
        }
        (route_len, run_cnt, run_cnt)
    }
    pub fn run_route_v2(&self, route_idx: usize, input_amount: u64, exp_pnl: u64) -> bool {
        let mut sent = false;
        let routes = unsafe{&* self.bank_bot.routes_v2.load(Ordering::SeqCst)};
        let pairs = unsafe{&* self.bank_bot.pairs_v2.load(Ordering::SeqCst)};
      let mut route = routes[route_idx].clone();
      route.update_amount(input_amount);

      let trade_pay_jito_tip_account_metas = vec![
          AccountMeta {
              pubkey: self.bank_bot.tip_payer.pubkey(),
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
              pubkey: self.bank_bot.tip_payer.pubkey(),
              is_signer: false,
              is_writable: true,
          },
          AccountMeta {
              pubkey: solana_program::system_program::ID,
              is_signer: false,
              is_writable: false,
          },
      ];

      

        let wsol_ata_pubkey: Pubkey = Pubkey::from_str("Gm6TTgZXjATYcpBxkmJnCZbzeeoJ7TMJKwDLMaLFSpE1").unwrap();
        let usdc_ata_pubkey: Pubkey = Pubkey::from_str("BChF15Y7PAwEhWNCZxfa6AujRXZiHVTecsbH5CuV1jzD").unwrap();
        let ata_pubkey = if route.route_start_mint {wsol_ata_pubkey} else {usdc_ata_pubkey};

        // simulate tx
        let accounts: Option<RpcSimulateTransactionAccountsConfig> = Some(RpcSimulateTransactionAccountsConfig {
            encoding: Some(UiAccountEncoding::JsonParsed),
            addresses: vec![ata_pubkey.to_string()]
        });
        let cu_ix = ComputeBudgetInstruction::set_compute_unit_limit(1_400_000);

        // let _ = self.bank_bot.create_and_send_ix(&[cu_ix, route.instruction.clone()], &route.lookuptables, true, &self.bank_bot.payer);
        // println!("sent");
        // return;

        let sim_rpc_res = self.bank_bot.create_and_simulate_ix(&[cu_ix, route.instruction.clone()], &route.lookuptables, accounts);
        if sim_rpc_res.is_err() {
            // println!("rpc error");
            // println!("rpc error {:#?}", sim_rpc_res.err());
        }
        else {
            let sim_res = sim_rpc_res.unwrap();
            if sim_res.value.err.is_none() {
                let accounts_to_subscribe = unsafe{&*self.bank_bot.accounts_to_subscribe.load(Ordering::SeqCst)};
                let amount_before = ArcBankBot::get_amount_from_token_account(&accounts_to_subscribe.get(&ata_pubkey).unwrap().account_data);
                let decimals = if route.route_start_mint {1_000_000_000f64} else {1_000_000f64};
                let amount_after = Self::get_ui_amount(&sim_res.value.accounts, 0, decimals);
                // println!("{}, {} -> {}", route.route_start_mint, amount_before, amount_after);
                if amount_after > amount_before {
                    let mut sim_pnl = if route.route_start_mint { amount_after - amount_before } else { (amount_after - amount_before) * SOL_USDC_RATE / SOL_USDC_RATE_MULTIPLIER };
                
                    // if  sim_pnl > 3000 {
                    if sim_pnl > PNL_THRESHOLD {
                        let slot = self.bank_bot.rpc_client.get_max_shred_insert_slot().unwrap_or(0);
                        let sim_cu = sim_res.value.units_consumed.unwrap() + 3000;
                        // static tipping
                        let tip_percent = 4500;
                        let mut tip_amount = sim_pnl.checked_mul(tip_percent).unwrap().checked_div(FEE_MULTIPLIER as u64).unwrap();


                        //temp
                        // route.update_amount(input_amount / 10);
                        // sim_pnl = sim_pnl / 10;
                        // tip_amount = 100_000_000_000 + (slot % 1000);
                        
                        // let micro_lamports: u64 = sim_pnl * 1_000_000 * PRIORITY_FEE_PERCENT_BUNDLE / sim_cu / FEE_MULTIPLIER as u64;
                        // let micro_lamports: u64 = 1_000_000 * 100 / sim_cu;
                        // let new_cp_ix = ComputeBudgetInstruction::set_compute_unit_price(micro_lamports);
                        
                        // send bundle
                        let new_cu_ix = ComputeBudgetInstruction::set_compute_unit_limit(sim_cu as u32);



                        let versioned_tx = self.get_versioned_tx(&[new_cu_ix.clone(), route.instruction.clone()], &route.lookuptables);
                        if versioned_tx.is_ok() {
                            // let versioned_tx = versioned_tx.unwrap();

                            // send check tx
                            // let _ = self.bank_bot.create_and_send_ix(&self.bank_bot.check_ixs, &[], true, &self.bank_bot.check_payer);

                            // send bundle
                            
                            // let start = std::time::SystemTime::now();
                            // let since_the_epoch = start
                            //     .duration_since(UNIX_EPOCH)
                            //     .expect("Time went backwards");
                            // let in_secs = since_the_epoch.as_secs();
                            // // println!("in_secs {}", in_secs);
                            // let tip_percent = if in_secs < 1734936015 {
                            //     TIP_PERCENT * 100
                            // }
                            // else {
                            //     TIP_PERCENT
                            // };
                            

                            // if !self.bank_bot.is_bundle_allowed.load(Ordering::SeqCst) {
                            //     self.bank_bot.is_bundle_allowed.store(true, Ordering::SeqCst);
                            // }
                            // else {
                            //     return;
                            // }
                            let pair0 = &pairs[route.route[0] as usize];
                            let pair1 = &pairs[route.route[1] as usize];

                            let same_bundle_cnt = 1;
                            let dynamic_tip = false;
                            
                            // for i in 0..same_bundle_cnt {
                                // let odd = if dynamic_tip {slot % same_bundle_cnt as u64} else {100u64};
                                let odd = 100u64;
                                let new_cu_ix_i = ComputeBudgetInstruction::set_compute_unit_limit(sim_cu as u32 + 100u32);
                                let versioned_swap_tx = self.get_versioned_tx(&[new_cu_ix_i.clone(), route.instruction.clone()], &route.lookuptables).expect("versioned_swap_tx error");
                                
                                let tip_params = TradePayJitoTipParams {
                                    usdc_sol_rate: SOL_USDC_RATE as u16,
                                    percent: 10,
                                    odd,
                                    same_bundle_cnt: same_bundle_cnt as u8,
                                    sim_pnl: sim_pnl * 99 / 100
                                };
                                let jito_tip_ix = Route::get_trade_pay_jito_tip_ix(
                                    &trade_pay_jito_tip_account_metas,
                                    &tip_params,
                                );
                                let new_cu_ix_tip = ComputeBudgetInstruction::set_compute_unit_limit(11000);
                                let jito_tip_tx_res = self.get_tip_versioned_tx(&[new_cu_ix_tip, jito_tip_ix.clone()], &[]);
                                let jito_tip_static_ix = transfer(
                                    &self.bank_bot.tip_payer.pubkey(), 
                                    &Pubkey::from_str(JITO_TIP_ACCOUNTS[5]).unwrap(), 
                                    tip_amount
                                );
                                let new_cu_ix_static_tip = ComputeBudgetInstruction::set_compute_unit_limit(500 + 11000);
                                let jito_tip_static_tx_res = self.get_tip_versioned_tx(&[new_cu_ix_static_tip.clone(), jito_tip_ix.clone(), jito_tip_static_ix.clone()], &[]);
                                if jito_tip_static_tx_res.is_ok() && jito_tip_tx_res.is_ok() {
                                    let jito_tip_ix_versioned = jito_tip_tx_res.unwrap();
                                    let jito_tip_static_ix_versioned = jito_tip_static_tx_res.unwrap();

                                    // self.bank_bot.bundle_sender.send_bundle(&[jito_tip_tx]);
                                    // let _ = self.bank_bot.create_and_send_ix(&[new_cu_ix_i.clone(), new_cp_ix.clone(), route.instruction.clone()], &route.lookuptables, true, &self.bank_bot.payer);
                                    // let _ = self.bank_bot.create_and_send_ix(&[new_cu_ix_static_tip, jito_tip_ix, jito_tip_static_ix], &route.lookuptables, true, &self.bank_bot.payer);
                                    if dynamic_tip {
                                        self.bank_bot.bundle_sender.send_bundle(&[versioned_swap_tx.clone(), jito_tip_ix_versioned], false);
                                    }
                                    else {
                                        self.bank_bot.bundle_sender.send_bundle(&[ versioned_swap_tx.clone(), jito_tip_static_ix_versioned.clone()], true);

                                        
                                        let jito_tip_static_ix_temp = transfer(
                                            &self.bank_bot.tip_payer.pubkey(), 
                                            &Pubkey::from_str(JITO_TIP_ACCOUNTS[5]).unwrap(), 
                                            (slot % 1000) + 1000
                                        );
                                        let jito_tip_static_ix_temp2 = transfer(
                                            &self.bank_bot.tip_payer.pubkey(), 
                                            &pair0.pool, 
                                            1
                                        );
                                        let jito_tip_static_ix_temp3 = transfer(
                                            &self.bank_bot.tip_payer.pubkey(), 
                                            &pair1.pool, 
                                            1
                                        );
                                        let new_cu_ix_static_tip_temp = ComputeBudgetInstruction::set_compute_unit_limit(700);
                                        let jito_tip_static_tx_res_temp = self.get_tip_versioned_tx(&[new_cu_ix_static_tip_temp, jito_tip_static_ix_temp2, jito_tip_static_ix_temp3, jito_tip_static_ix_temp], &[]);
                                        let jito_tip_static_ix_versioned_temp = jito_tip_static_tx_res_temp.unwrap();
                                        self.bank_bot.bundle_sender.send_bundle(&[jito_tip_static_ix_versioned_temp], true);

                                        sent = true;
                                    }
                                    
                                    
                                    // let _ = self.bank_bot.send_tx(&jito_tip_tx, true);
                                    // println!("send_tx sent");

                                    // simulate bundle
                                    // let sim_bundle_res = self.bank_bot.simulate_bundle(&[versioned_swap_tx, jito_tip_static_ix_versioned]);
                                    // if sim_bundle_res.is_ok() {
                                    //     println!("sim bundle res {:#?}", sim_bundle_res.unwrap().value.transaction_results);
                                    // }
                                    // else {
                                    //     println!("sim bundle err {:#?}", sim_bundle_res.err());
                                    // }
                                    
                                }
                                else {
                                    println!("jito_tip_tx_res err {:#?}", jito_tip_tx_res.err());
                                }

                                
                            // }

                            
                            println!("bundle sent! route: {}, sim_pnl {}, input {}, slot {}", route.route.iter().map(|a|a.to_string()).collect::<Vec<String>>().join("->"), sim_pnl as f64 / 1_000_000_000f64, input_amount as f64 / 1_000_000_000f64, slot);
                            // println!("route.instruction {:#?}", route.instruction);
                            println!("pair0 {:#?} -> pair1 {:#?}", pair0.pool, pair1.pool);
                            // let ten_millis = std::time::Duration::from_millis(400);
                            // std::thread::sleep(ten_millis);
                        }
                    }
                }
                
                
            }
            else {
                // if pairs[route.route[0] as usize].swap_type == SwapType::MeteoraDlmm || pairs[route.route[1] as usize].swap_type == SwapType::MeteoraDlmm {
                    // println!("simulation error for MeteoraDlmm");
                    // println!("route.instruction: {:#?}", route.instruction.clone());
                    let logs = sim_res.value.logs.unwrap_or(vec![]);
                    let mut out_of_memory = false;
                    for log in logs.iter() {
                        if log.contains("OutOfMemory") {
                            out_of_memory = true;
                            break;
                        }
                    }
                    if !out_of_memory {
                        // println!("simulation error logs: {:#?}", logs);
                    }
                    
                // }
                
            }
        }
        return sent;

    }
    pub fn run_route(&self, route_idx: usize, input_amount: u64, exp_pnl: u64) -> bool {
        let mut sent = false;
      let routes = unsafe{&* self.bank_bot.routes.load(Ordering::SeqCst)};
      let mut route = routes[route_idx].clone();
      route.update_amount(input_amount);
      
        //   let cu_ix = ComputeBudgetInstruction::set_compute_unit_limit(1400000);

    //   let slot = self.bank_bot.rpc_client.get_cu().unwrap_or(0u64);

      let trade_pay_jito_tip_params_0 = TradePayJitoTipParams {
          usdc_sol_rate: SOL_USDC_RATE as u16,
          percent: TIP_PERCENT,
          odd: 0,
          same_bundle_cnt: 5,
          sim_pnl: 0u64
      };
      let trade_pay_jito_tip_params_1 = TradePayJitoTipParams {
        usdc_sol_rate: SOL_USDC_RATE as u16,
        percent: TIP_PERCENT,
        odd: 1,
        same_bundle_cnt: 5,
        sim_pnl: 0u64
        };
      let mut trade_pay_jito_tip_account_metas = vec![
          AccountMeta {
              pubkey: self.bank_bot.payer.pubkey(),
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
    //   for remaining_key in VOTE_ACCOUNTS {
    //     trade_pay_jito_tip_account_metas.push(AccountMeta {
    //         pubkey: Pubkey::from_str(remaining_key).unwrap(),
    //         is_signer: false,
    //         is_writable: false,
    //     });
    //   }
      let jito_tip_ix_0 = Route::get_trade_pay_jito_tip_ix(
          &trade_pay_jito_tip_account_metas,
          &trade_pay_jito_tip_params_0,
      );
      let jito_tip_ix_1 = Route::get_trade_pay_jito_tip_ix(
        &trade_pay_jito_tip_account_metas,
        &trade_pay_jito_tip_params_1,
    );

      let jito_tip_versioned_tx_0 = self.get_versioned_tx(&[jito_tip_ix_0.clone()], &[]);
      let jito_tip_versioned_tx_1 = self.get_versioned_tx(&[jito_tip_ix_1.clone()], &[]);

      let total_lookuptables: &Vec<AddressLookupTableAccount> = unsafe { &*self.bank_bot.lookuptables.load(Ordering::SeqCst) };
      

      let pairs = unsafe { &*self.bank_bot.pairs.load(Ordering::SeqCst) };
        
      let (ix, lookuptable_idxs) = route.get_instruction_with_ticks(total_lookuptables, pairs);
      let lookuptables: Vec<AddressLookupTableAccount> = lookuptable_idxs.iter().map(|idx| total_lookuptables[*idx as usize].clone() ).collect();

      

        // send tx

        // let tick_rpc_client_index = self.bank_bot.tick_rpc_client_index.load(Ordering::SeqCst);
        // let tick_rpc_client: RpcClient = RpcClient::new(String::from(
        //     TICK_RPC_ENDPOINTS[tick_rpc_client_index % 2007],
        // ));
        // self.bank_bot.tick_rpc_client_index.store((tick_rpc_client_index + 1) % 2007, Ordering::SeqCst);
        
        // let send_res = self.bank_bot.create_and_send_ix_with_rpc(&[cu_ix, ix], &lookuptables, true, tick_rpc_client);
        // if send_res.is_err() {
        //     println!("send_res: {:#?}", send_res);
        // }
        // else {
        //     println!("success tx sent!");
        // }
        #[derive(PartialEq)]
        enum SendOption {
            SimulateAndBundle,
            SimulateAndDirectTx,
            DirectSendBundle,
            DirectSendTx,
            DirectSendTxWithRpc
        }
        let send_option = if DIRECT_SEND {SendOption::SimulateAndDirectTx}  else {SendOption::SimulateAndBundle};

        if send_option == SendOption::DirectSendBundle {
            let sim_cu = route.cu;
            let new_cu_ix = ComputeBudgetInstruction::set_compute_unit_limit(sim_cu as u32);

            // send bundle
            let versioned_tx = self.get_versioned_tx(&[new_cu_ix, ix], &lookuptables);
            if jito_tip_versioned_tx_0.is_ok() && versioned_tx.is_ok() {
                let versioned_tx = versioned_tx.unwrap();
                self.bank_bot
                    .bundle_sender
                    .send_bundle(&[versioned_tx.clone(), jito_tip_versioned_tx_0.unwrap()], false);
                println!("bundle sent! route: {}, exp_pnl: {}", route.route.iter().map(|a|a.to_string()).collect::<Vec<String>>().join("->"), exp_pnl);
            }
        }
        else if send_option == SendOption::SimulateAndBundle {
            let wsol_ata_pubkey: Pubkey = Pubkey::from_str("Gm6TTgZXjATYcpBxkmJnCZbzeeoJ7TMJKwDLMaLFSpE1").unwrap();
            let usdc_ata_pubkey: Pubkey = Pubkey::from_str("BChF15Y7PAwEhWNCZxfa6AujRXZiHVTecsbH5CuV1jzD").unwrap();
            let ata_pubkey = if route.route_start_mint {wsol_ata_pubkey} else {usdc_ata_pubkey};
    
            // simulate tx
            let accounts: Option<RpcSimulateTransactionAccountsConfig> = Some(RpcSimulateTransactionAccountsConfig {
                encoding: Some(UiAccountEncoding::JsonParsed),
                addresses: vec![ata_pubkey.to_string()]
            });
    
            // let cu_ix = ComputeBudgetInstruction::set_compute_unit_limit(1_400_000);
            // let sim_rpc_res2 = self.bank_bot.create_and_simulate_ix(&[cu_ix, jito_tip_ix.clone()], &[], None);
            // println!("simulation logs: {:#?}", sim_rpc_res2.unwrap().value.logs.unwrap_or(vec![]));
            // return true;

            let cu_ix = ComputeBudgetInstruction::set_compute_unit_limit(1_400_000);
            let sim_rpc_res = self.bank_bot.create_and_simulate_ix(&[cu_ix, ix.clone()], &lookuptables, accounts);
            if sim_rpc_res.is_err() {
                println!("rpc error");
                // println!("rpc error {:#?}", sim_rpc_res.err());
            }
            else {
                let sim_res = sim_rpc_res.unwrap();
                if sim_res.value.err.is_none() {
    
                    let accounts_to_subscribe = unsafe{&*self.bank_bot.accounts_to_subscribe.load(Ordering::SeqCst)};
                    let amount_before = ArcBankBot::get_amount_from_token_account(&accounts_to_subscribe.get(&ata_pubkey).unwrap().account_data);
                    let decimals = if route.route_start_mint {1_000_000_000f64} else {1_000_000f64};
                    let amount_after = Self::get_ui_amount(&sim_res.value.accounts, 0, decimals);
                    // println!("{}, {} -> {}", route.route_start_mint, amount_before, amount_after);
                    if amount_after > amount_before {
                        let sim_pnl = if route.route_start_mint { amount_after - amount_before } else { (amount_after - amount_before) * SOL_USDC_RATE / SOL_USDC_RATE_MULTIPLIER };
                    
                        if sim_pnl > PNL_THRESHOLD {
                            let sim_cu = sim_res.value.units_consumed.unwrap() + 1000;
                            let new_cu_ix = ComputeBudgetInstruction::set_compute_unit_limit(sim_cu as u32);
                            let micro_lamports: u64 = sim_pnl * 1_000_000 * PRIORITY_FEE_PERCENT_BUNDLE / sim_cu / FEE_MULTIPLIER as u64;
                            let new_cp_ix = ComputeBudgetInstruction::set_compute_unit_price(micro_lamports);
                            // send bundle
                            let versioned_tx = self.get_versioned_tx(&[new_cu_ix, new_cp_ix, ix], &lookuptables);
                            if jito_tip_versioned_tx_0.is_ok() && versioned_tx.is_ok() {
                                let versioned_tx = versioned_tx.unwrap();

                                // send check tx
                                // let _ = self.bank_bot.create_and_send_ix(&self.bank_bot.check_ixs, &[], true, &self.bank_bot.check_payer);

                                // send bundle
                                self.bank_bot.bundle_sender.send_bundle(&[versioned_tx.clone(), jito_tip_versioned_tx_0.unwrap()], false);
                                self.bank_bot.bundle_sender.send_bundle(&[versioned_tx.clone(), jito_tip_versioned_tx_1.unwrap()], false);

                                // send check tx
                                // let _ = self.bank_bot.create_and_send_ix(&self.bank_bot.check_ixs, &[], true, &self.bank_bot.check_payer);

                                // simulate bundle
                                // let sim_bundle_res = self.bank_bot.simulate_bundle(&[versioned_tx.clone(), jito_tip_versioned_tx_0.unwrap()]);
                                // if sim_bundle_res.is_ok() {
                                //     println!("sim bundle res {:#?}", sim_bundle_res.unwrap().value.transaction_results);
                                // }
                                // else {
                                //     println!("sim bundle err res {:#?}", sim_bundle_res.err());
                                // }
                                
                                

                                // println!("bundle sent! route: {}, exp_pnl: {}, sim_pnl {}", route.route.iter().map(|a|pairs[*a as usize].swap_type.to_string()).collect::<Vec<String>>().join("->"), exp_pnl, sim_pnl);
                                println!("bundle sent! route: {}, exp_pnl: {}, sim_pnl {}", route.route.iter().map(|a|a.to_string()).collect::<Vec<String>>().join("->"), exp_pnl, sim_pnl);
                            }
                        }
                    }
                    
                    
                }
                else {
                    // println!("simulation error");
                    // println!("simulation error logs: {:#?}", sim_res.value.logs.unwrap_or(vec![]));
                }
            }
        }
        else if send_option == SendOption::SimulateAndDirectTx {
            let wsol_ata_pubkey: Pubkey = Pubkey::from_str("Gm6TTgZXjATYcpBxkmJnCZbzeeoJ7TMJKwDLMaLFSpE1").unwrap();
            let usdc_ata_pubkey: Pubkey = Pubkey::from_str("BChF15Y7PAwEhWNCZxfa6AujRXZiHVTecsbH5CuV1jzD").unwrap();
            let ata_pubkey = if route.route_start_mint {wsol_ata_pubkey} else {usdc_ata_pubkey};
    
            // simulate tx
            let accounts: Option<RpcSimulateTransactionAccountsConfig> = Some(RpcSimulateTransactionAccountsConfig {
                encoding: Some(UiAccountEncoding::JsonParsed),
                addresses: vec![ata_pubkey.to_string()]
            });
    
            let cu_ix = ComputeBudgetInstruction::set_compute_unit_limit(1_400_000);
            let sim_rpc_res = self.bank_bot.create_and_simulate_ix(&[cu_ix, ix.clone()], &lookuptables, accounts);
            if sim_rpc_res.is_err() {
                println!("rpc error");
                // println!("rpc error {:#?}", sim_rpc_res.err());
            }
            else {
                let sim_res = sim_rpc_res.unwrap();
                if sim_res.value.err.is_none() {
    
                    let accounts_to_subscribe = unsafe{&*self.bank_bot.accounts_to_subscribe.load(Ordering::SeqCst)};
                    let amount_before = ArcBankBot::get_amount_from_token_account(&accounts_to_subscribe.get(&ata_pubkey).unwrap().account_data);
                    let decimals = if route.route_start_mint {1_000_000_000f64} else {1_000_000f64};
                    let amount_after = Self::get_ui_amount(&sim_res.value.accounts, 0, decimals);
                    // println!("{}, {} -> {}", route.route_start_mint, amount_before, amount_after);
                    if amount_after > amount_before {
                        let sim_pnl = if route.route_start_mint { amount_after - amount_before } else { (amount_after - amount_before) * SOL_USDC_RATE / SOL_USDC_RATE_MULTIPLIER };
                    
                        if sim_pnl > PNL_THRESHOLD /*&& sim_pnl < MAX_PNL*/ {
                            let sim_cu = sim_res.value.units_consumed.unwrap() + 1000;
                            let new_cu_ix = ComputeBudgetInstruction::set_compute_unit_limit(sim_cu as u32);
                            // let micro_lamports: u64 = sim_pnl * 1_000_000 * PRIORITY_FEE_PERCENT_DIRECT / sim_cu / FEE_MULTIPLIER as u64;
                            let micro_lamports: u64 = 5_000 * 1_000_000 / sim_cu;
                            let new_cp_ix = ComputeBudgetInstruction::set_compute_unit_price(micro_lamports);
                            let send_res = self.bank_bot.create_and_send_ix(&[new_cu_ix, new_cp_ix, ix], &lookuptables, true, &self.bank_bot.payer);
                            sent = true;
                            if send_res.is_ok() {
                                println!("tx sent! route: {}, exp_pnl: {}, sim_pnl {}", route.route.iter().map(|a|a.to_string()).collect::<Vec<String>>().join("->"), exp_pnl, sim_pnl);
                            }
                            else {
                                println!("tx sending error!");
                            }
                        }
                    }
                    
                    
                }
                else {
                    // println!("simulation error");
                    // println!("simulation error logs: {:#?}", sim_res.value.logs.unwrap_or(vec![]));
                }
            }
        }
        else if send_option == SendOption::DirectSendTx {
            // send tx
            // let micro_lamports: u64 = 100;
            // let new_cp_ix = ComputeBudgetInstruction::set_compute_unit_price(micro_lamports);

            // let tick_rpc_client_index = self.bank_bot.tick_rpc_client_index.load(Ordering::SeqCst);
            // let tick_rpc_client = RpcClient::new(String::from(
            //     TICK_RPC_ENDPOINTS[tick_rpc_client_index % 2007],
            // ));
            // self.bank_bot
            //     .tick_rpc_client_index
            //     .store((tick_rpc_client_index + 1) % 2007, Ordering::SeqCst);
            // let send_res = self.bank_bot.create_and_send_ix_with_rpc(&[new_cu_ix, new_cp_ix, ix], &lookuptables, false, tick_rpc_client);
            // let send_res = self.bank_bot.create_and_send_ix(&[new_cu_ix, new_cp_ix, ix], &lookuptables, false);
            // if send_res.is_err() {
            //     println!("send_res: {:#?}", send_res);
            // }
            // else {
            //     println!("tx sent! route idx: {}, exp_pnl: {}", route_idx, exp_pnl);
            // }
        }
        else if send_option == SendOption::DirectSendTxWithRpc {
            // send tx
            // let micro_lamports: u64 = 100;
            // let new_cp_ix = ComputeBudgetInstruction::set_compute_unit_price(micro_lamports);

            // let tick_rpc_client_index = self.bank_bot.tick_rpc_client_index.load(Ordering::SeqCst);
            // let tick_rpc_client = RpcClient::new(String::from(
            //     TICK_RPC_ENDPOINTS[tick_rpc_client_index % 2007],
            // ));
            // self.bank_bot
            //     .tick_rpc_client_index
            //     .store((tick_rpc_client_index + 1) % 2007, Ordering::SeqCst);
            // let send_res = self.bank_bot.create_and_send_ix_with_rpc(&[new_cu_ix, new_cp_ix, ix], &lookuptables, false, tick_rpc_client);
            // let send_res = self.bank_bot.create_and_send_ix(&[new_cu_ix, new_cp_ix, ix], &lookuptables, false);
            // if send_res.is_err() {
            //     println!("send_res: {:#?}", send_res);
            // }
            // else {
            //     println!("tx sent! route idx: {}, exp_pnl: {}", route_idx, exp_pnl);
            // }
        }
        sent
        
        
    }
    pub fn get_ui_amount(accounts: &Option<Vec<Option<UiAccount>>>, index: usize, decimals: f64) -> u64 {
        if accounts.as_ref().unwrap().len() > index {
            if let UiAccountData::Json(json_data) = &accounts.as_ref().unwrap().get(index).unwrap().as_ref().unwrap().data {
                let ui_amount_f64 = json_data.parsed["info"]["tokenAmount"]["uiAmount"].as_f64().unwrap();
                let ui_amount: u64 = (ui_amount_f64 * decimals) as u64;
                return ui_amount;
            }
        }
        
        0u64
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
    pub fn get_tip_versioned_tx(
        &self,
        tx_instructions: &[Instruction],
        lookuptables: &[AddressLookupTableAccount],
    ) -> std::result::Result<VersionedTransaction, SignerError> {
        let signers = vec![&self.bank_bot.tip_payer];
        let blockhash = unsafe{ &* self.bank_bot.latest_blockhash.load(Ordering::SeqCst)};

        let versioned_message = V0(solana_sdk::message::v0::Message::try_compile(
            &self.bank_bot.tip_payer.pubkey(),
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
    pub async fn run_v2(&self) {
        std::panic::set_hook(Box::new(|info| {
          println!("Caught a panic: {:?}", info);
        }));
        println!("run_v2 started!");
        let pairs = unsafe{&* self.bank_bot.pairs_v2.load(Ordering::SeqCst)};
        let mut loops_cnt = 0;
        loop {
            loops_cnt += 1;
            println!("loops_cnt: {:?}", loops_cnt);
            let is_price_update_initially = self.bank_bot.is_price_updated_initially.load(Ordering::SeqCst);
            let pair_len = pairs.len();
            for pair_index in 0..pair_len {
                let is_running = self.bank_bot.is_running.load(Ordering::SeqCst);
                if is_running && is_price_update_initially {
                    let instance = self.get_arc_clone();
                    let (run_cnt, elapsed_process_route) = measure_us!(instance.process_routes_v2(pair_index));
                    if run_cnt > 0 {
                        println!("process time: {:?}ms", elapsed_process_route / 1000);
                    }
                }
                let ten_millis = std::time::Duration::from_millis(10);
                std::thread::sleep(ten_millis);
            }
        }
    }
    pub async fn setup_v2(&self) {
        std::panic::set_hook(Box::new(|info| {
          println!("Caught a panic: {:?}", info);
        }));
        println!("standard shm server bank bot is started!");
        self.bank_bot.is_running.store(true, Ordering::SeqCst);
        self.read_from_files_v2().await;
        self.clone_accounts_and_build_pair_prices_initially_v2().await;

        // let instance_clone = Arc::clone(&self.bank_bot);
        // self.bank_bot.tokio_runtime.spawn(async move {
        //     // trace!();
        //     BankBot::run_tick_bot_in_background_v2(&instance_clone).await;
        //     // trace!();
        // });
        
        println!("setup_v2 finished!");
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
      

      let instance_clone = Arc::clone(&self.bank_bot);
      self.bank_bot.tokio_runtime.spawn(async move {
        // trace!();
        BankBot::run_tick_bot_in_background(&instance_clone).await;
        // trace!();
      });
      println!("setup ended.");
    }

    // pub fn set_validator_cpu() {
    //     Self::set_cpu_range_for_this_thread(VALIDATOR_CPU_START, VALIDATOR_CPU_END);
    // }
    // pub fn set_bot_cpu() {
    //     Self::set_cpu_range_for_this_thread(BOT_CPU_START, BOT_CPU_END);
    // }
    // pub fn set_cpu_range_for_this_thread(start: u32, end: i32) {
    //     let topo = Arc::new(Mutex::new(Topology::new()));
    //     let child_topo = topo.clone();
    //     let tid = unsafe { libc::pthread_self() };
    //     let mut locked_topo = child_topo.lock().unwrap();
    //     let before = locked_topo.get_cpubind_for_thread(tid, CPUBIND_THREAD);
    //     // println!("Thread {}: Before {:?}", i, before);
    //     let bind_to = CpuSet::from_range(start, end);

    //     locked_topo
    //         .set_cpubind_for_thread(tid, bind_to, CPUBIND_THREAD)
    //         .unwrap();

    //     let after = locked_topo.get_cpubind_for_thread(tid, CPUBIND_THREAD);
    //     // Mutex::unlock(locked_topo);

    //     println!("Cpu Set: Before {:?}, After {:?}", before, after);
    // }
    pub async fn read_from_files_v2(&self) {
        let total_lookuptables = unsafe { &mut *self.bank_bot.lookuptables.load(Ordering::SeqCst) };
        let mut pair_lookups = Vec::new();
        // reading pairs v2
        let pairs = unsafe { &mut *self.bank_bot.pairs_v2.load(Ordering::SeqCst) };
        pairs.clear();
        println!("reading pairs...");
        if let Ok(pairs_slice) = spawn_blocking(|| {
            let mut pairs_file = File::open(PAIRS_V2_FILE_PATH)?;
            let mut slice = Vec::new();
            pairs_file.read_to_end(&mut slice)?;
            Ok::<_, std::io::Error>(slice)
        })
        .await
        {
            for pair_chunk_slice in pairs_slice.unwrap().chunks(PairV2::PAIR_SLICE_SIZE) {
                let pair = PairV2::deserialize(pair_chunk_slice);
                pair_lookups.push(pair.lookup.clone());
                pairs.push(pair);
            }
        }

        println!("reading lookuptables...");
        if let Ok(lookuptables_content) = spawn_blocking(|| {
            let mut lookuptables_file = File::open(LOOKUPTABLE_V2_FILE_PATH)?;
            let mut content = String::new();
            lookuptables_file.read_to_string(&mut content)?;
            Ok::<_, std::io::Error>(content)
        })
        .await
        {
            match serde_json::from_str(&lookuptables_content.unwrap()) {
                Ok(res) => {
                    let lookuptables_tmp: Vec<LookupTableV2> = res;

                    for lookuptable in lookuptables_tmp {
                        let key = Pubkey::from_str(&lookuptable.lookup).unwrap();
                        if pair_lookups.contains(&key) {
                            total_lookuptables.push(AddressLookupTableAccount {
                                key,
                                addresses: lookuptable
                                    .addresses
                                    .iter()
                                    .map(|addr| Pubkey::from_str(addr).unwrap())
                                    .collect(),
                            });
                        }
                    }
                }
                Err(_err) => println!("lookuptables read error-> {:#?}", _err),
            };
        };

        let routes = unsafe { &mut *self.bank_bot.routes_v2.load(Ordering::SeqCst) };
        routes.clear();
        println!("reading routes...");
        if let Ok(routes_content) = spawn_blocking(|| {
            let mut routes_file = File::open(ROUTES_V2_FILE_PATH)?;
            let mut content = String::new();
            routes_file.read_to_string(&mut content)?;
            Ok::<_, std::io::Error>(content)
        })
        .await
        {
            match serde_json::from_str(&routes_content.unwrap()) {
                Ok(res) => {
                    let routes_tmp: Vec<Vec<u32>> = res;

                    for route in routes_tmp {
                        let mut lookuptables = Vec::new();
                        for pair_idx in route.iter() {
                            let pair = &pairs[*pair_idx as usize];
                            let lookup_key = pair.lookup.clone();
                            for lookup_in_total in total_lookuptables.iter() {
                                if lookup_in_total.key.eq(&lookup_key) {
                                    lookuptables.push(lookup_in_total.clone());
                                }
                            }
                            
                        }
                        let instruction_opt = RouteV2::get_v2_instruction(&route, pairs);
                        if instruction_opt.is_some() {
                            routes.push(RouteV2{
                                route,
                                instruction: instruction_opt.unwrap(),
                                lookuptables,
                                route_start_mint: true
                            });
                        }
                        
                    }
                }
                Err(_err) => println!("routes read error-> {:#?}", _err),
            };
        };

        // read ticks
        println!("reading ticks...");
        if let Ok(ticks_content) = spawn_blocking(|| {
            let mut ticks_file = File::open(TICK_FILE_PATH)?;
            let mut content = String::new();
            ticks_file.read_to_string(&mut content)?;
            Ok::<_, std::io::Error>(content)
        })
        .await
        {
            match serde_json::from_str(&ticks_content.unwrap()) {
                Ok(res) => self.bank_bot
                    .ticks
                    .store(Box::into_raw(Box::new(res)), Ordering::SeqCst),
                Err(_err) => println!("ticks read error-> {:#?}", _err),
            };
        };
        
        println!("successfully read files!");

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


        let mut full_routes: Vec<Vec<u32>> = Vec::new();
        let mut read_cnt = 0;

        for routes_file_idx in 0..811 {
            // println!("reading {} routes file", routes_file_idx);
            let routes_file_path = ROUTES_FILE_PATH.to_string() + &routes_file_idx.to_string() + ".json";
            if let Ok(routes_content) = spawn_blocking(move || {
                let mut routes_file = File::open(&routes_file_path)?;
                let mut content = String::new();
                routes_file.read_to_string(&mut content)?;
                Ok::<_, std::io::Error>(content)
            })
            .await
            {
                match serde_json::from_str(&routes_content.unwrap()) {
                    Ok(res) => {
                        let mut routes_tmp: Vec<Vec<u32>> = res;
                        full_routes.append(&mut routes_tmp);
                        
                        
                        // let routes_tmp: Vec<Vec<u32>> = res;
    
                        // let mut route_futures = FuturesUnordered::new();
    
                        // for (route_idx, route) in routes_tmp.into_iter().enumerate() {
                        //     // let pairs = unsafe { &*self.bank_bot.pairs.load(Ordering::SeqCst) };
                        //     // let lookuptables = unsafe { &*self.bank_bot.lookuptables.load(Ordering::SeqCst) };

                        //     route_futures.push(tokio::spawn(async move {
                        //         let result = spawn_blocking(move || {

                        //             // let file_path = ROUTE_BIN_FILE_PATH.to_owned() + &route_idx.to_string() + ".bin";
                        //             // // read from file
                        //             // let mut route_file = File::open(&file_path).expect("open route file error");
                        //             // let mut slice = Vec::new();
                        //             // route_file.read_to_end(&mut slice).expect("reading route file error");
                        //             // let route = Route::deserialize(&mut slice.as_slice()).expect("deserializing route error");
                                  
                        //             // write to file
                        //             // let route = Route::build_route(&route, pairs, lookuptables);
                        //             // let mut buffer: Vec<u8> = Vec::new();
                        //             // route.serialize(&mut buffer).expect("serialize error");
                        //             // let mut route_file = OpenOptions::new()
                        //             //     .create(true)
                        //             //     .write(true)
                        //             //     .truncate(true)
                        //             //     .open(file_path).expect("write route file error");
                        //             // route_file.write_all(&buffer).expect("write route file error");
                        //             // route_file.flush().expect("write route file error");



                        //             route
                        //         }).await;

                        //         (route_idx, result)
                        //     }));
                        // }
                        
    
                        
                        // while let Some(result) = route_futures.next().await {
                        //   read_cnt += 1;
                        //   match result {
                        //       Ok((route_idx, Ok(route))) => {
                        //         full_routes.push((route_idx, route));
                        //       },
                        //       _ => {
    
                        //       }
                        //   };
                        //   if read_cnt % 10000 == 0 {
                        //     println!("{} route read!", read_cnt);
                        //   }
                        // }
    
                        
                    }
                    Err(_err) => println!("routes read error-> {:#?}", _err),
                };
            };
        }
        
        println!("reading route bins...");
        

        let all_routes_indexed: Arc<Mutex<Vec<(usize, Route)>>> = Arc::new(Mutex::new(Vec::new()));
        let mut handles = Vec::new();
        let route_len = full_routes.len();
        println!("full routes len {}", route_len);
        let current_route_idx = Arc::new(AtomicUsize::new(0usize));

        // let full_routes_arc = Arc::new(full_routes);
        for _ in 0..50 {
            // let instance = ArcBankBot {bank_bot: Arc::clone(&self.bank_bot)};
            // let full_routes_clone = Arc::clone(&full_routes_arc);
            let all_routes_indexed_clone = Arc::clone(&all_routes_indexed);
            let current_route_idx_clone = Arc::clone(&current_route_idx);
            handles.push(std::thread::spawn(move || {
                loop {
                    let route_chunk_idx = current_route_idx_clone.load(Ordering::SeqCst);
                    current_route_idx_clone.store(route_chunk_idx+1, Ordering::SeqCst);
    
                    // let pairs = unsafe { &*instance.bank_bot.pairs.load(Ordering::SeqCst) };
                    // let lookuptables = unsafe { &*instance.bank_bot.lookuptables.load(Ordering::SeqCst) };

                    let chunk = 20_000;
                    let start_route_idx = route_chunk_idx * chunk as usize;
                    let end_route_idx = (route_chunk_idx + 1) * chunk as usize;
                    // println!("processing cnt {}", start_route_idx);
                    for route_idx in start_route_idx..end_route_idx {
                        if route_idx >= route_len {
                            return;
                        }
                        let file_path = if route_idx == 39718552 || route_idx == 39718937 {
                            ROUTE_BIN_FILE_PATH4.to_owned() + &route_idx.to_string() + ".bin"
                        } else if route_chunk_idx < 1000 {
                            ROUTE_BIN_FILE_PATH1.to_owned() + &route_idx.to_string() + ".bin"
                        } else if route_chunk_idx < 2000 {
                            ROUTE_BIN_FILE_PATH2.to_owned() + &route_idx.to_string() + ".bin"
                        } else {
                            ROUTE_BIN_FILE_PATH3.to_owned() + &route_idx.to_string() + ".bin"
                        };
                        
                        // read from file
                        
                        let route_file_res = File::open(&file_path);
                        if route_file_res.is_err() {
                            println!("route {}.bin open error! {:#?}", route_idx, route_file_res.err());
                            let mut all_routes_indexed_temp = all_routes_indexed_clone.lock().unwrap();
                            all_routes_indexed_temp.push((route_idx, Route::default()));
                        }
                        else {
                            let mut route_file = route_file_res.unwrap();
                            let mut slice = Vec::new();
                            route_file.read_to_end(&mut slice).expect("reading route file error");
                            let route = Route::deserialize(&mut slice.as_slice()).expect("deserializing route error");
                            let mut all_routes_indexed_temp = all_routes_indexed_clone.lock().unwrap();
                            all_routes_indexed_temp.push((route_idx, route));
                        }
    
                        // write to file
                        // let route_path = &full_routes_clone[route_idx];
                        // let route = Route::build_route(route_path, pairs, lookuptables);
                        // let mut buffer: Vec<u8> = Vec::new();
                        // let route_serialized_res = route.serialize(&mut buffer);
                        // if route_serialized_res.is_ok() {
                        //     let route_file_res = OpenOptions::new()
                        //         .create(true)
                        //         .write(true)
                        //         .truncate(true)
                        //         .open(file_path);
                        //     if route_file_res.is_ok() {
                        //         let mut route_file = route_file_res.unwrap();
                        //         route_file.write_all(&buffer).expect("write route file error");
                        //         route_file.flush().expect("write route file error");
                        //     }
                        //     else {
                        //         println!("creating route file error {:#?}", route_file_res.err());
                        //     }
                        // }
                        // else {
                        //     println!("serializing route error");
                        // }
                        
                    }
                }
            }));
        }
        for h in handles {
            h.join().expect("reading bin files thread panic");
        }
        println!("processing routes done!");
        // loop {
        //     let ten_millis = std::time::Duration::from_millis(10000);
        //     std::thread::sleep(ten_millis);
        // }

        let mut all_routes_indexed_temp = all_routes_indexed.lock().unwrap();
        all_routes_indexed_temp.sort_by_key(|(idx, _)|*idx);
        let all_routes: Vec<Route> = all_routes_indexed_temp.clone().into_iter().map(|(_, route)| route).collect();
        self.bank_bot
            .routes
            .store(Box::into_raw(Box::new(all_routes)), Ordering::SeqCst);


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
                        // if file_idx % 100 == 0 {
                        //     println!("current read group index: {:?}", file_idx);
                        // }
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


        // read ticks
        if let Ok(ticks_content) = spawn_blocking(|| {
            let mut ticks_file = File::open(TICK_FILE_PATH)?;
            let mut content = String::new();
            ticks_file.read_to_string(&mut content)?;
            Ok::<_, std::io::Error>(content)
        })
        .await
        {
            match serde_json::from_str(&ticks_content.unwrap()) {
                Ok(res) => self.bank_bot
                    .ticks
                    .store(Box::into_raw(Box::new(res)), Ordering::SeqCst),
                Err(_err) => println!("ticks read error-> {:#?}", _err),
            };
        };

        println!("successfully read files!");
    }

    pub async fn clone_accounts_and_build_pair_prices_initially(&self) {

      let pairs = unsafe{&* self.bank_bot.pairs.load(Ordering::SeqCst)};
      let accounts_to_subscribe_vec = unsafe{&mut * self.bank_bot.accounts_to_subscribe_vec.load(Ordering::SeqCst)};
      let accounts_to_subscribe = unsafe{&mut *self.bank_bot.accounts_to_subscribe.load(Ordering::SeqCst)};
      trace!();

      //clone user vaults initially
      let wsol_ata_pubkey = Pubkey::from_str("Gm6TTgZXjATYcpBxkmJnCZbzeeoJ7TMJKwDLMaLFSpE1").unwrap();
      let usdc_ata_pubkey = Pubkey::from_str("BChF15Y7PAwEhWNCZxfa6AujRXZiHVTecsbH5CuV1jzD").unwrap();
      accounts_to_subscribe.insert(wsol_ata_pubkey, SubscribeAccount{account_data: self.bank_bot.rpc_client.get_account_data(&wsol_ata_pubkey).unwrap_or(Vec::new()), pair_index: None});
      accounts_to_subscribe.insert(usdc_ata_pubkey, SubscribeAccount{account_data: self.bank_bot.rpc_client.get_account_data(&usdc_ata_pubkey).unwrap_or(Vec::new()), pair_index: None});

      for (pair_idx, pair) in pairs.iter().enumerate() {
        // if DIED_POOLS.contains(&pair.pool.to_string().as_str()) {
        //     continue;
        // }
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

      let mut total_addresses_file = File::open(TOTAL_ADDRESSES_TO_SUB_FILE_PATH).unwrap();
    let mut total_addresses_content = String::new();
    total_addresses_file.read_to_string(&mut total_addresses_content).unwrap();
    let total_addresses_str: Vec<String> = serde_json::from_str(&total_addresses_content).unwrap();
    let total_addresses: Vec<Pubkey> = total_addresses_str.iter().map(|p| Pubkey::from_str(p).unwrap()).collect();
    for pubkey in total_addresses {
        if !accounts_to_subscribe.contains_key(&pubkey) {
            let account_data = match self.bank_bot.rpc_client.get_account_data(&pubkey) {
                Ok(data) => data,
                Err(_) => Vec::new()
            };
            accounts_to_subscribe.insert(pubkey, SubscribeAccount{account_data, pair_index: None});
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

      let mut _build_price_cnt = 0;
      while let Some(_result) = build_price_futures.next().await {
        _build_price_cnt += 1;
      }
    //   println!("total build pair price cnt: {}", build_price_cnt);
      self.bank_bot.is_price_updated_initially.store(true, Ordering::SeqCst);
    //   println!("clone_accounts_and_build_pair_prices_initially done!");
    trace!();
    }
    pub async fn clone_accounts_and_build_pair_prices_initially_v2(&self) {

        let pairs = unsafe{&* self.bank_bot.pairs_v2.load(Ordering::SeqCst)};
        let accounts_to_subscribe_vec = unsafe{&mut * self.bank_bot.accounts_to_subscribe_vec.load(Ordering::SeqCst)};
        let accounts_to_subscribe = unsafe{&mut *self.bank_bot.accounts_to_subscribe.load(Ordering::SeqCst)};
        trace!();
  
        //clone user vaults initially
        let wsol_ata_pubkey = Pubkey::from_str("Gm6TTgZXjATYcpBxkmJnCZbzeeoJ7TMJKwDLMaLFSpE1").unwrap();
        let usdc_ata_pubkey = Pubkey::from_str("BChF15Y7PAwEhWNCZxfa6AujRXZiHVTecsbH5CuV1jzD").unwrap();
        accounts_to_subscribe.insert(wsol_ata_pubkey, SubscribeAccount{account_data: self.bank_bot.rpc_client.get_account_data(&wsol_ata_pubkey).unwrap_or(Vec::new()), pair_index: None});
        accounts_to_subscribe.insert(usdc_ata_pubkey, SubscribeAccount{account_data: self.bank_bot.rpc_client.get_account_data(&usdc_ata_pubkey).unwrap_or(Vec::new()), pair_index: None});
  
        for (pair_idx, pair) in pairs.iter().enumerate() {
          // if DIED_POOLS.contains(&pair.pool.to_string().as_str()) {
          //     continue;
          // }
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
                // println!("initial price");
              instance.build_pair_price_v2(i);
            }).await;
          }))
        }
  
        let mut _build_price_cnt = 0;
        while let Some(_result) = build_price_futures.next().await {
          _build_price_cnt += 1;
        }
      //   println!("total build pair price cnt: {}", build_price_cnt);
        self.bank_bot.is_price_updated_initially.store(true, Ordering::SeqCst);
      //   println!("clone_accounts_and_build_pair_prices_initially done!");
      trace!();
      }
    pub fn build_pair_price(&self, pair_idx: usize) -> u128 {
        let ticks = unsafe {&mut * self.bank_bot.ticks.load(Ordering::SeqCst)};
        let pairs = unsafe { &mut *self.bank_bot.pairs.load(Ordering::SeqCst) };
        let pair = &mut pairs[pair_idx];
        // println!("build_pair_price pair#{}: {:#?}, {:#?}",pair_idx, pair.swap_type, pair.pool);
        let accounts_info = unsafe {&* self.bank_bot.accounts_to_subscribe.load(Ordering::SeqCst)};
        
        let mut _delta_price: u128 = 0;
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
                    if pair.price.is_some() && price > 0 {
                        _delta_price = (pair.price.as_ref().unwrap().price_a_to_b.max(price) - pair.price.as_ref().unwrap().price_a_to_b.min(price))
                            .checked_mul(PRICE_DELTA_MULTIPLIER).unwrap_or(u128::MAX)
                            .checked_div(price).unwrap();
                    }
                    pair.price = Some(Price {
                        ticks_a_to_b: None,
                        ticks_b_to_a: None,
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
                        price_buffer_pc: None,
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
                        if pair.price.is_some() && price > 0 {
                            _delta_price = (pair.price.as_ref().unwrap().price_a_to_b.max(price) - pair.price.as_ref().unwrap().price_a_to_b.min(price))
                                .checked_mul(PRICE_DELTA_MULTIPLIER).unwrap_or(u128::MAX)
                                .checked_div(price).unwrap();
                        }
                        pair.price = Some(Price {
                            ticks_a_to_b: None,
                            ticks_b_to_a: None,

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
                            price_buffer_pc: None,
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
                    if pair.price.is_some() && price > 0 {
                        _delta_price = (pair.price.as_ref().unwrap().price_a_to_b.max(price) - pair.price.as_ref().unwrap().price_a_to_b.min(price))
                            .checked_mul(PRICE_DELTA_MULTIPLIER).unwrap_or(u128::MAX)
                            .checked_div(price).unwrap();
                    }
                    pair.price = Some(Price {
                        ticks_a_to_b: None,
                        ticks_b_to_a: None,
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
                        price_buffer_pc: None,
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

                    let tick_opt = ticks.iter().find(|t| t.pool.eq(&pair.pool.to_string()));
                    let ticks_a_to_b;
                    let ticks_b_to_a;
                    if tick_opt.is_none() {
                        println!("pair.pool is none {}", pair.pool.to_string());
                        ticks_a_to_b = None;
                        ticks_b_to_a = None;
                    }
                    else {
                        let tick = tick_opt.unwrap();
                        ticks_a_to_b = Some(tick.get_tick_arrays(&pair.swap_type, active_tick_id, true));
                        ticks_b_to_a = Some(tick.get_tick_arrays(&pair.swap_type, active_tick_id, false));
                    }
                    if pair.price.is_some() && price > 0 {
                        _delta_price = (pair.price.as_ref().unwrap().price_a_to_b.max(price) - pair.price.as_ref().unwrap().price_a_to_b.min(price))
                            .checked_mul(PRICE_DELTA_MULTIPLIER).unwrap_or(u128::MAX)
                            .checked_div(price).unwrap();
                    }
                    pair.price = Some(Price {
                        ticks_a_to_b,
                        ticks_b_to_a,
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
                        price_buffer_pc: None,
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

                    let tick_opt = ticks.iter().find(|t| t.pool.eq(&pair.pool.to_string()));
                    let ticks_a_to_b;
                    let ticks_b_to_a;
                    if tick_opt.is_none() {
                        println!("pair.pool is none {}", pair.pool.to_string());
                        ticks_a_to_b = None;
                        ticks_b_to_a = None;
                    }
                    else {
                        let tick = tick_opt.unwrap();
                        ticks_a_to_b = Some(tick.get_tick_arrays(&pair.swap_type, active_tick_id, true));
                        ticks_b_to_a = Some(tick.get_tick_arrays(&pair.swap_type, active_tick_id, false));
                    }
                    if pair.price.is_some() && price > 0 {
                        _delta_price = (pair.price.as_ref().unwrap().price_a_to_b.max(price) - pair.price.as_ref().unwrap().price_a_to_b.min(price))
                            .checked_mul(PRICE_DELTA_MULTIPLIER).unwrap_or(u128::MAX)
                            .checked_div(price).unwrap();
                    }
                    pair.price = Some(Price {
                        ticks_a_to_b,
                        ticks_b_to_a,
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
                        price_buffer_pc: None,
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

                    let tick_opt = ticks.iter().find(|t| t.pool.eq(&pair.pool.to_string()));
                    let ticks_a_to_b;
                    let ticks_b_to_a;
                    if tick_opt.is_none() {
                        println!("pair.pool is none {}", pair.pool.to_string());
                        ticks_a_to_b = None;
                        ticks_b_to_a = None;
                    }
                    else {
                        let tick = tick_opt.unwrap();
                        ticks_a_to_b = Some(tick.get_tick_arrays(&pair.swap_type, active_tick_id, true));
                        ticks_b_to_a = Some(tick.get_tick_arrays(&pair.swap_type, active_tick_id, false));
                    }
                    if pair.price.is_some() && price > 0 {
                        _delta_price = (pair.price.as_ref().unwrap().price_a_to_b.max(price) - pair.price.as_ref().unwrap().price_a_to_b.min(price))
                            .checked_mul(PRICE_DELTA_MULTIPLIER).unwrap_or(u128::MAX)
                            .checked_div(price).unwrap();
                    }
                    pair.price = Some(Price {
                        ticks_a_to_b,
                        ticks_b_to_a,
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
                        price_buffer_pc: None,
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

                    let tick_opt = ticks.iter().find(|t| t.pool.eq(&pair.pool.to_string()));
                    let ticks_a_to_b;
                    let ticks_b_to_a;
                    if tick_opt.is_none() {
                        println!("pair.pool is none {}", pair.pool.to_string());
                        ticks_a_to_b = None;
                        ticks_b_to_a = None;
                    }
                    else {
                        let tick = tick_opt.unwrap();
                        ticks_a_to_b = Some(tick.get_tick_arrays(&pair.swap_type, active_tick_id, true));
                        ticks_b_to_a = Some(tick.get_tick_arrays(&pair.swap_type, active_tick_id, false));
                    }
                    if pair.price.is_some() && price > 0 {
                        _delta_price = (pair.price.as_ref().unwrap().price_a_to_b.max(price) - pair.price.as_ref().unwrap().price_a_to_b.min(price))
                            .checked_mul(PRICE_DELTA_MULTIPLIER).unwrap_or(u128::MAX)
                            .checked_div(price).unwrap();
                    }
                    pair.price = Some(Price {
                        ticks_a_to_b,
                        ticks_b_to_a,
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
                        price_buffer_pc: None,
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
                        pair.fee = FEE_MULTIPLIER - fee_percent as u32;
                        let token_a_mint_slice = array_ref![pool_state_data, TOKEN_MINT_A_OFFSET, 32];
                        let protocol_share_slice = array_ref![pool_state_data, PROTOCOL_SHARE_OFFSET, 2];
                        let protocol_share = u16::from_le_bytes(*protocol_share_slice);

                        let tick_opt = ticks.iter().find(|t| t.pool.eq(&pair.pool.to_string()));
                        let ticks_a_to_b;
                        let ticks_b_to_a;
                        if tick_opt.is_none() {
                            println!("pair.pool is none {}", pair.pool.to_string());
                            ticks_a_to_b = None;
                            ticks_b_to_a = None;
                        }
                        else {
                            let tick = tick_opt.unwrap();
                            // println!("dlmm active id {}", active_id);
                            ticks_a_to_b = Some(tick.get_tick_arrays(&pair.swap_type, active_id, true));
                            ticks_b_to_a = Some(tick.get_tick_arrays(&pair.swap_type, active_id, false));
                        }
                        if pair.price.is_some() && price > 0 {
                            _delta_price = (pair.price.as_ref().unwrap().price_a_to_b.max(price) - pair.price.as_ref().unwrap().price_a_to_b.min(price))
                                .checked_mul(PRICE_DELTA_MULTIPLIER).unwrap_or(u128::MAX)
                                .checked_div(price).unwrap();
                        }
                        pair.price = Some(Price {
                            ticks_a_to_b,
                            ticks_b_to_a,
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
                            price_buffer_pc: None,
                        });
                    }
                }
            },
            SwapType::LifinitySwapV2 => {
                const POOL_OFFSET_UNTIL_CONFIG: usize = 8 + 32 * 3 + 8 * 2 + 6 + 32 * 10 + 8 * 8 + 1 + 8;
                const TOKEN_A_MINT_OFFSET: usize = 8 + 32 * 3 + 8 * 2 + 6 + 32 * 4;
                const LAST_PRICE_OFFSET: usize = POOL_OFFSET_UNTIL_CONFIG;
                const BASE_DECIMALS_OFFSET: usize = 8 + 32 * 3 + 8 * 2 + 5;
                const CONFIG_DENOMINATOR_OFFSET: usize = POOL_OFFSET_UNTIL_CONFIG + 8 * 2;
                const REG_TARGET_OFFSET: usize = POOL_OFFSET_UNTIL_CONFIG + 8 * 7;
                const STD_SPREAD_OFFSET: usize = REG_TARGET_OFFSET + 8 * 8;
                const STD_SPREAD_BUFFER_OFFSET: usize = STD_SPREAD_OFFSET + 8;
                const SPREAD_COEF_OFFSET: usize = STD_SPREAD_BUFFER_OFFSET + 8;
                const PRICE_BUFFER_COIN_OFFSET: usize = SPREAD_COEF_OFFSET + 8;
                const PRICE_BUFFER_PC_OFFSET: usize = PRICE_BUFFER_COIN_OFFSET + 8;

                if let Some(pool_state_account_info) = accounts_info.get(&pair.pool_state) {
                    let pool_state_data = &pool_state_account_info.account_data;
                    let config_denominator_slice = array_ref![pool_state_data, CONFIG_DENOMINATOR_OFFSET, 8];
                    let config_denominator = u64::from_le_bytes(*config_denominator_slice);
                    let base_decimals_slice = array_ref![pool_state_data, BASE_DECIMALS_OFFSET, 1];
                    let base_decimal: u8 = base_decimals_slice[0];
                    // msg!("base_decimal {}", base_decimal);
                    let base_decimal_value = 10u64.checked_pow(base_decimal as u32).unwrap();

                    let last_price_slice = array_ref![pool_state_data, LAST_PRICE_OFFSET, 8];
                    let last_price = u64::from_le_bytes(*last_price_slice);

                    // msg!("price {}", last_price);
                    // msg!("PRICE_MULTIPLIER {}", PRICE_MULTIPLIER);
                    // msg!("base_decimal_value {}", base_decimal_value);
                    let price: u128 = (last_price as u128)
                        .checked_mul(PRICE_MULTIPLIER as u128)
                        .unwrap()
                        .checked_div(base_decimal_value as u128)
                        .unwrap();
                    let price_reverse = (PRICE_MULTIPLIER as u128)
                        .checked_mul(PRICE_MULTIPLIER as u128)
                        .unwrap()
                        .checked_div(price as u128)
                        .unwrap();
                    let token_a_mint_slice = array_ref![pool_state_data, TOKEN_A_MINT_OFFSET, 32];

                    let coin_vault_account_info = accounts_info.get(&pair.pool_vault_a);
                    let pc_vault_account_info = accounts_info.get(&pair.pool_vault_b);
                    let coin_vault_amount = ArcBankBot::get_amount_from_token_account(
                        &coin_vault_account_info.unwrap().account_data,
                    );
                    let pc_vault_amount = ArcBankBot::get_amount_from_token_account(
                        &pc_vault_account_info.unwrap().account_data,
                    );

                    let regret_target_slice = array_ref![pool_state_data, REG_TARGET_OFFSET, 8];
                    let std_spread_slice = array_ref![pool_state_data, STD_SPREAD_OFFSET, 8];
                    let std_spread_buffer_slice = array_ref![pool_state_data, STD_SPREAD_BUFFER_OFFSET, 8];
                    let spread_coef_slice = array_ref![pool_state_data, SPREAD_COEF_OFFSET, 8];
                    let price_buffer_coin_slice = array_ref![pool_state_data, PRICE_BUFFER_COIN_OFFSET, 8];
                    let price_buffer_pc_slice = array_ref![pool_state_data, PRICE_BUFFER_PC_OFFSET, 8];

                    let regret_target = u64::from_le_bytes(*regret_target_slice);
                    let std_spread = u64::from_le_bytes(*std_spread_slice);
                    let std_spread_buffer = u64::from_le_bytes(*std_spread_buffer_slice);
                    let spread_coef = u64::from_le_bytes(*spread_coef_slice);
                    let price_buffer_coin = i64::from_le_bytes(*price_buffer_coin_slice);
                    let price_buffer_pc = i64::from_le_bytes(*price_buffer_pc_slice);
                    
                    if pair.price.is_some() && price > 0 {
                        _delta_price = (pair.price.as_ref().unwrap().price_a_to_b.max(price) - pair.price.as_ref().unwrap().price_a_to_b.min(price))
                            .checked_mul(PRICE_DELTA_MULTIPLIER).unwrap_or(u128::MAX)
                            .checked_div(price).unwrap();
                    }
                    pair.price = Some(Price {
                        ticks_a_to_b: None,
                        ticks_b_to_a: None,
                        price_a_to_b: price,
                        price_b_to_a: price_reverse,
                        vault_a_amount: Some(coin_vault_amount),
                        vault_b_amount: Some(pc_vault_amount),
                        active_tick_id: None,
                        pool_token_mint_a: Some(Pubkey::new_from_array(*token_a_mint_slice)),
                        liquidity: Some(regret_target as u128),
                        sqrt_price_x64: Some(last_price as u128),

                        config_denominator: Some(config_denominator),
                        std_spread: Some(std_spread),
                        std_spread_buffer: Some(std_spread_buffer),
                        spread_coef: Some(spread_coef),
                        base_decimal_value: Some(base_decimal_value),
                        price_buffer_coin: Some(price_buffer_coin),
                        price_buffer_pc: Some(price_buffer_pc),
                    });
                }
            }
            _ => {
              _delta_price = 0;
            }
        };
        _delta_price
        // println!("build_pair_price done!");
    }
    pub fn build_pair_price_v2(&self, pair_idx: usize) -> u128 {
        let ticks = unsafe {&mut * self.bank_bot.ticks.load(Ordering::SeqCst)};
        let pairs = unsafe { &mut *self.bank_bot.pairs_v2.load(Ordering::SeqCst) };
        let pair = &mut pairs[pair_idx];
        // println!("build_pair_price pair#{}: {:#?}, {:#?}",pair_idx, pair.swap_type, pair.pool);
        let accounts_info = unsafe {&* self.bank_bot.accounts_to_subscribe.load(Ordering::SeqCst)};
        
        let mut _delta_price: u128 = 0;
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
                    if pair.price.is_some() && price > 0 {
                        _delta_price = (pair.price.as_ref().unwrap().price_a_to_b.max(price) - pair.price.as_ref().unwrap().price_a_to_b.min(price))
                            .checked_mul(PRICE_DELTA_MULTIPLIER).unwrap_or(u128::MAX)
                            .checked_div(price).unwrap();
                    }
                    pair.price = Some(Price {
                        ticks_a_to_b: None,
                        ticks_b_to_a: None,
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
                        price_buffer_pc: None,
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
                    let a_withdrawable_amount = 0u64;
                        // ArcBankBot::get_meteora_pools_vault_withdrawable_amount(
                        //     &a_vault_account_info.unwrap().account_data,
                        //     current_time,
                        // );
                    let b_withdrawable_amount = 0u64;
                        // ArcBankBot::get_meteora_pools_vault_withdrawable_amount(
                        //     &b_vault_account_info.unwrap().account_data,
                        //     current_time,
                        // );

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
                        if pair.price.is_some() && price > 0 {
                            _delta_price = (pair.price.as_ref().unwrap().price_a_to_b.max(price) - pair.price.as_ref().unwrap().price_a_to_b.min(price))
                                .checked_mul(PRICE_DELTA_MULTIPLIER).unwrap_or(u128::MAX)
                                .checked_div(price).unwrap();
                        }
                        pair.price = Some(Price {
                            ticks_a_to_b: None,
                            ticks_b_to_a: None,

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
                            price_buffer_pc: None,
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
                    if pair.price.is_some() && price > 0 {
                        _delta_price = (pair.price.as_ref().unwrap().price_a_to_b.max(price) - pair.price.as_ref().unwrap().price_a_to_b.min(price))
                            .checked_mul(PRICE_DELTA_MULTIPLIER).unwrap_or(u128::MAX)
                            .checked_div(price).unwrap();
                    }
                    pair.price = Some(Price {
                        ticks_a_to_b: None,
                        ticks_b_to_a: None,
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
                        price_buffer_pc: None,
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

                    let tick_opt = ticks.iter().find(|t| t.pool.eq(&pair.pool.to_string()));
                    let ticks_a_to_b;
                    let ticks_b_to_a;
                    if tick_opt.is_none() {
                        // println!("pair.pool is none {}", pair.pool.to_string());
                        ticks_a_to_b = None;
                        ticks_b_to_a = None;
                    }
                    else {
                        let tick = tick_opt.unwrap();
                        ticks_a_to_b = Some(tick.get_tick_arrays(&pair.swap_type, active_tick_id, true));
                        ticks_b_to_a = Some(tick.get_tick_arrays(&pair.swap_type, active_tick_id, false));
                    }
                    if pair.price.is_some() && price > 0 {
                        _delta_price = (pair.price.as_ref().unwrap().price_a_to_b.max(price) - pair.price.as_ref().unwrap().price_a_to_b.min(price))
                            .checked_mul(PRICE_DELTA_MULTIPLIER).unwrap_or(u128::MAX)
                            .checked_div(price).unwrap();
                    }
                    pair.price = Some(Price {
                        ticks_a_to_b,
                        ticks_b_to_a,
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
                        price_buffer_pc: None,
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
                        .unwrap_or(0);
                    let token_a_mint_slice = array_ref![pool_state_data, TOKEN_MINT_A_OFFSET, 32];

                    let tick_opt = ticks.iter().find(|t| t.pool.eq(&pair.pool.to_string()));
                    let ticks_a_to_b;
                    let ticks_b_to_a;
                    if tick_opt.is_none() {
                        // println!("pair.pool is none {}", pair.pool.to_string());
                        ticks_a_to_b = None;
                        ticks_b_to_a = None;
                    }
                    else {
                        let tick = tick_opt.unwrap();
                        ticks_a_to_b = Some(tick.get_tick_arrays(&pair.swap_type, active_tick_id, true));
                        ticks_b_to_a = Some(tick.get_tick_arrays(&pair.swap_type, active_tick_id, false));
                    }
                    if pair.price.is_some() && price > 0 {
                        _delta_price = (pair.price.as_ref().unwrap().price_a_to_b.max(price) - pair.price.as_ref().unwrap().price_a_to_b.min(price))
                            .checked_mul(PRICE_DELTA_MULTIPLIER).unwrap_or(u128::MAX)
                            .checked_div(price).unwrap();
                    }
                    pair.price = Some(Price {
                        ticks_a_to_b,
                        ticks_b_to_a,
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
                        price_buffer_pc: None,
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

                    let tick_opt = ticks.iter().find(|t| t.pool.eq(&pair.pool.to_string()));
                    let ticks_a_to_b;
                    let ticks_b_to_a;
                    if tick_opt.is_none() {
                        // println!("pair.pool is none {}", pair.pool.to_string());
                        ticks_a_to_b = None;
                        ticks_b_to_a = None;
                    }
                    else {
                        let tick = tick_opt.unwrap();
                        ticks_a_to_b = Some(tick.get_tick_arrays(&pair.swap_type, active_tick_id, true));
                        ticks_b_to_a = Some(tick.get_tick_arrays(&pair.swap_type, active_tick_id, false));
                    }
                    if pair.price.is_some() && price > 0 {
                        _delta_price = (pair.price.as_ref().unwrap().price_a_to_b.max(price) - pair.price.as_ref().unwrap().price_a_to_b.min(price))
                            .checked_mul(PRICE_DELTA_MULTIPLIER).unwrap_or(u128::MAX)
                            .checked_div(price).unwrap();
                    }
                    pair.price = Some(Price {
                        ticks_a_to_b,
                        ticks_b_to_a,
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
                        price_buffer_pc: None,
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

                    let tick_opt = ticks.iter().find(|t| t.pool.eq(&pair.pool.to_string()));
                    let ticks_a_to_b;
                    let ticks_b_to_a;
                    if tick_opt.is_none() {
                        // println!("pair.pool is none {}", pair.pool.to_string());
                        ticks_a_to_b = None;
                        ticks_b_to_a = None;
                    }
                    else {
                        let tick = tick_opt.unwrap();
                        ticks_a_to_b = Some(tick.get_tick_arrays(&pair.swap_type, active_tick_id, true));
                        ticks_b_to_a = Some(tick.get_tick_arrays(&pair.swap_type, active_tick_id, false));
                    }
                    if pair.price.is_some() && price > 0 {
                        _delta_price = (pair.price.as_ref().unwrap().price_a_to_b.max(price) - pair.price.as_ref().unwrap().price_a_to_b.min(price))
                            .checked_mul(PRICE_DELTA_MULTIPLIER).unwrap_or(u128::MAX)
                            .checked_div(price).unwrap();
                    }
                    pair.price = Some(Price {
                        ticks_a_to_b,
                        ticks_b_to_a,
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
                        price_buffer_pc: None,
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
                        pair.fee = FEE_MULTIPLIER - fee_percent as u32;
                        let token_a_mint_slice = array_ref![pool_state_data, TOKEN_MINT_A_OFFSET, 32];
                        let protocol_share_slice = array_ref![pool_state_data, PROTOCOL_SHARE_OFFSET, 2];
                        let protocol_share = u16::from_le_bytes(*protocol_share_slice);

                        // let tick_opt = ticks.iter().find(|t| t.pool.eq(&pair.pool.to_string()));
                        // let ticks_a_to_b;
                        // let ticks_b_to_a;
                        // if tick_opt.is_none() {
                        //     println!("pair.pool is none {}", pair.pool.to_string());
                        //     ticks_a_to_b = None;
                        //     ticks_b_to_a = None;
                        // }
                        // else {
                        //     let tick = tick_opt.unwrap();
                        //     // println!("dlmm active id {}", active_id);
                        //     ticks_a_to_b = Some(tick.get_tick_arrays(&pair.swap_type, active_id, true));
                        //     ticks_b_to_a = Some(tick.get_tick_arrays(&pair.swap_type, active_id, false));
                        // }

                        // get dlmm binarrays from account vec
                        let mut ticks_a_to_b_vec = Vec::new();
                        let max_len = if pair.accounts_vec_a_to_b.len() < 18 {pair.accounts_vec_a_to_b.len()} else {18};
                        for i in 16..max_len {
                            ticks_a_to_b_vec.push(pair.accounts_vec_a_to_b[i].pubkey);
                        }
                        let ticks_a_to_b = Some(ticks_a_to_b_vec);

                        let mut ticks_b_to_a_vec = Vec::new();
                        let max_len = if pair.accounts_vec_b_to_a.len() < 18 {pair.accounts_vec_b_to_a.len()} else {18};
                        for i in 16..max_len {
                            ticks_b_to_a_vec.push(pair.accounts_vec_b_to_a[i].pubkey);
                        }
                        let ticks_b_to_a = Some(ticks_b_to_a_vec);

                        if pair.price.is_some() && price > 0 {
                            _delta_price = (pair.price.as_ref().unwrap().price_a_to_b.max(price) - pair.price.as_ref().unwrap().price_a_to_b.min(price))
                                .checked_mul(PRICE_DELTA_MULTIPLIER).unwrap_or(u128::MAX)
                                .checked_div(price).unwrap();
                        }
                        println!("dlmm price {}, {}", price, price_reverse);
                        pair.price = Some(Price {
                            ticks_a_to_b,
                            ticks_b_to_a,
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
                            price_buffer_pc: None,
                        });
                    }
                }
            },
            SwapType::LifinitySwapV2 => {
                const POOL_OFFSET_UNTIL_CONFIG: usize = 8 + 32 * 3 + 8 * 2 + 6 + 32 * 10 + 8 * 8 + 1 + 8;
                const TOKEN_A_MINT_OFFSET: usize = 8 + 32 * 3 + 8 * 2 + 6 + 32 * 4;
                const LAST_PRICE_OFFSET: usize = POOL_OFFSET_UNTIL_CONFIG;
                const BASE_DECIMALS_OFFSET: usize = 8 + 32 * 3 + 8 * 2 + 5;
                const CONFIG_DENOMINATOR_OFFSET: usize = POOL_OFFSET_UNTIL_CONFIG + 8 * 2;
                const REG_TARGET_OFFSET: usize = POOL_OFFSET_UNTIL_CONFIG + 8 * 7;
                const STD_SPREAD_OFFSET: usize = REG_TARGET_OFFSET + 8 * 8;
                const STD_SPREAD_BUFFER_OFFSET: usize = STD_SPREAD_OFFSET + 8;
                const SPREAD_COEF_OFFSET: usize = STD_SPREAD_BUFFER_OFFSET + 8;
                const PRICE_BUFFER_COIN_OFFSET: usize = SPREAD_COEF_OFFSET + 8;
                const PRICE_BUFFER_PC_OFFSET: usize = PRICE_BUFFER_COIN_OFFSET + 8;

                if let Some(pool_state_account_info) = accounts_info.get(&pair.pool_state) {
                    let pool_state_data = &pool_state_account_info.account_data;
                    let config_denominator_slice = array_ref![pool_state_data, CONFIG_DENOMINATOR_OFFSET, 8];
                    let config_denominator = u64::from_le_bytes(*config_denominator_slice);
                    let base_decimals_slice = array_ref![pool_state_data, BASE_DECIMALS_OFFSET, 1];
                    let base_decimal: u8 = base_decimals_slice[0];
                    // msg!("base_decimal {}", base_decimal);
                    let base_decimal_value = 10u64.checked_pow(base_decimal as u32).unwrap();

                    let last_price_slice = array_ref![pool_state_data, LAST_PRICE_OFFSET, 8];
                    let last_price = u64::from_le_bytes(*last_price_slice);

                    // msg!("price {}", last_price);
                    // msg!("PRICE_MULTIPLIER {}", PRICE_MULTIPLIER);
                    // msg!("base_decimal_value {}", base_decimal_value);
                    let price: u128 = (last_price as u128)
                        .checked_mul(PRICE_MULTIPLIER as u128)
                        .unwrap()
                        .checked_div(base_decimal_value as u128)
                        .unwrap();
                    let price_reverse = (PRICE_MULTIPLIER as u128)
                        .checked_mul(PRICE_MULTIPLIER as u128)
                        .unwrap()
                        .checked_div(price as u128)
                        .unwrap();
                    let token_a_mint_slice = array_ref![pool_state_data, TOKEN_A_MINT_OFFSET, 32];

                    let coin_vault_account_info = accounts_info.get(&pair.pool_vault_a);
                    let pc_vault_account_info = accounts_info.get(&pair.pool_vault_b);
                    let coin_vault_amount = ArcBankBot::get_amount_from_token_account(
                        &coin_vault_account_info.unwrap().account_data,
                    );
                    let pc_vault_amount = ArcBankBot::get_amount_from_token_account(
                        &pc_vault_account_info.unwrap().account_data,
                    );

                    let regret_target_slice = array_ref![pool_state_data, REG_TARGET_OFFSET, 8];
                    let std_spread_slice = array_ref![pool_state_data, STD_SPREAD_OFFSET, 8];
                    let std_spread_buffer_slice = array_ref![pool_state_data, STD_SPREAD_BUFFER_OFFSET, 8];
                    let spread_coef_slice = array_ref![pool_state_data, SPREAD_COEF_OFFSET, 8];
                    let price_buffer_coin_slice = array_ref![pool_state_data, PRICE_BUFFER_COIN_OFFSET, 8];
                    let price_buffer_pc_slice = array_ref![pool_state_data, PRICE_BUFFER_PC_OFFSET, 8];

                    let regret_target = u64::from_le_bytes(*regret_target_slice);
                    let std_spread = u64::from_le_bytes(*std_spread_slice);
                    let std_spread_buffer = u64::from_le_bytes(*std_spread_buffer_slice);
                    let spread_coef = u64::from_le_bytes(*spread_coef_slice);
                    let price_buffer_coin = i64::from_le_bytes(*price_buffer_coin_slice);
                    let price_buffer_pc = i64::from_le_bytes(*price_buffer_pc_slice);
                    
                    if pair.price.is_some() && price > 0 {
                        _delta_price = (pair.price.as_ref().unwrap().price_a_to_b.max(price) - pair.price.as_ref().unwrap().price_a_to_b.min(price))
                            .checked_mul(PRICE_DELTA_MULTIPLIER).unwrap_or(u128::MAX)
                            .checked_div(price).unwrap();
                    }
                    pair.price = Some(Price {
                        ticks_a_to_b: None,
                        ticks_b_to_a: None,
                        price_a_to_b: price,
                        price_b_to_a: price_reverse,
                        vault_a_amount: Some(coin_vault_amount),
                        vault_b_amount: Some(pc_vault_amount),
                        active_tick_id: None,
                        pool_token_mint_a: Some(Pubkey::new_from_array(*token_a_mint_slice)),
                        liquidity: Some(regret_target as u128),
                        sqrt_price_x64: Some(last_price as u128),

                        config_denominator: Some(config_denominator),
                        std_spread: Some(std_spread),
                        std_spread_buffer: Some(std_spread_buffer),
                        spread_coef: Some(spread_coef),
                        base_decimal_value: Some(base_decimal_value),
                        price_buffer_coin: Some(price_buffer_coin),
                        price_buffer_pc: Some(price_buffer_pc),
                    });
                }
            }
            _ => {
              _delta_price = 0;
            }
        };
        _delta_price
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
            .safe_mul(bin_step as u128)?
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
        let price = U128::from(sqrt_price_x64)
            .mul_div_floor(U128::from(sqrt_price_x64), U128::from(Q64))
            .unwrap_or(U128::from(0))
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

        let unlocked_amount = total_amount.checked_sub(locked_profit).unwrap_or(0u64);
        unlocked_amount
    }

    pub fn calc_amount_out(
        &self,
        pair_idx: usize,
        input_amount: u64,
        from_mint: Pubkey,
        stacked_swap_steps: &mut [u64; 400],
        route_index: usize
    ) -> (u64, u128, u128) {
        

        let pairs = unsafe { &*self.bank_bot.pairs.load(Ordering::SeqCst) };
        let accounts = unsafe { &* self.bank_bot.accounts_to_subscribe.load(Ordering::SeqCst)};
        let pair = &pairs[pair_idx];

        // let swap_types: Vec<SwapType> = vec![
        //     SwapType::RaydiumConcentrated, 
        //     SwapType::OrcaWhirlpool, 
        //     SwapType::CropperWhirlpool, 
        //     SwapType::InvariantSwap, 
        //     SwapType::CremaFinance, 
        //     SwapType::MeteoraDlmm,
        // ];
        // if swap_types.contains(&pair.swap_type) {
        //     println!("swap type : {:#?}", pair.swap_type);
        // }

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
                        .unwrap_or(u128::MAX) as u128;
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
                        .unwrap_or(u128::MAX) as u128;
                }
                trace!();
                (_out_amount, _next_price, _next_price_reversed)
            },
            SwapType::MeteoraDlmm =>{
                // let mut log_msg: String = pair.pool.to_string();
                let price = pair.price.as_ref().unwrap();
                let protocol_share = price.config_denominator.unwrap() as u16;
                let volatility_accumulator = price.std_spread.unwrap() as u32;
                let variable_fee_control = price.std_spread_buffer.unwrap() as u32;
                let base_factor = price.spread_coef.unwrap() as u16;
                let bin_step = price.base_decimal_value.unwrap() as u16;
                let active_id = price.price_buffer_coin.unwrap() as i32;

                // log_msg.push_str(&(" active_id:".to_owned() + active_id.to_string().as_str()));

                let lb_pair_info = LbPairInfo{
                    protocol_share,
                    volatility_accumulator, 
                    variable_fee_control, 
                    base_factor, 
                    bin_step,
                    active_id
                };

                let (bin_array_pubkeys, zero_for_one) = if from_mint == pair.mint_a {
                    (pair.price.as_ref().unwrap().ticks_a_to_b.as_ref().unwrap(), true)
                } else {
                    (pair.price.as_ref().unwrap().ticks_b_to_a.as_ref().unwrap(), false)
                };
                // if pair.price.as_ref().unwrap().ticks_a_to_b.is_some() || pair.price.as_ref().unwrap().ticks_b_to_a.is_some() {
                //     println!("pool: {:#?}, bin_array_pubkeys.len() {}, ticks_a_to_b {:#?}, ticks_b_to_a {:#?}", pair.pool, bin_array_pubkeys.len(), pair.price.as_ref().unwrap().ticks_a_to_b, pair.price.as_ref().unwrap().ticks_b_to_a);
                // }
                // else {
                //     println!("ticks_a_to_b and ticks_b_to_a are none, pool: {:#?}", pair.pool);
                // }

                if bin_array_pubkeys.len() == 0 {
                    // log_msg.push_str(" returned (0,0,0)");
                    // println!("{}", log_msg);
                    // println!("bin_array_pubkeys.len() == 0");
                    return (0, 0, 0)
                };
                // println!("bin_array_pubkeys {}", bin_array_pubkeys.len());
                let mut bin_array_accs:Vec<&Vec<u8>> = Vec::new();
                for bin_array_key in bin_array_pubkeys {
                    match accounts.get(bin_array_key) {
                        Some(bin_account) => {
                            
                            bin_array_accs.push(&bin_account.account_data)
                        },
                        None => {
                            println!("bin array doesn't exist {:#?}", bin_array_key);
                        }
                    }
                }
                // println!("bin_array_accs.len() {:#?}", bin_array_accs.len());
                if bin_array_accs.len() == 0 {
                    // log_msg.push_str(" returned (0,0,0)");
                    // println!("{}", log_msg);
                    return (0, 0, 0)
                }
                // println!("calculating next price of dlmm... {}", bin_array_accs.len());
                let price_info = Self::get_next_price_from_input(
                    input_amount, 
                    price.sqrt_price_x64.unwrap(), 
                    zero_for_one,
                    &lb_pair_info, 
                    &bin_array_accs,
                    stacked_swap_steps,
                    route_index
                ).unwrap_or(PriceInfo {
                    price: 0u128,
                    actual_amount_in: 0u64,
                    actual_amount_out: 0u64,
                    fee: 0u64,
                    protocol_fee: 0u64,
                    bin_id: 0i32
                });

                let next_price = Self::get_price_from_x64(price_info.price);

                // println!("amount_out {}, next_price {}", price_info.actual_amount_out, next_price);
                if zero_for_one {
                    (price_info.actual_amount_out, next_price, 0)
                }
                else {
                    (price_info.actual_amount_out, 0, next_price)
                }
            },
            SwapType::RaydiumConcentrated | SwapType::CremaFinance => {
                let price = pair.price.as_ref().unwrap();

                let amount_in = (input_amount as u128)
                    .checked_mul(pair.fee as u128)
                    .unwrap()
                    .checked_div(FEE_MULTIPLIER as u128)
                    .unwrap()
                    as u64;
                
                let zero_for_one = price.pool_token_mint_a.unwrap() == from_mint;
                let sqrt_price_x64 = price.sqrt_price_x64.unwrap();
                let liquidity = price.liquidity.unwrap();
                let next_sqrt_price_x64 = if liquidity > 0 && sqrt_price_x64 > 0 {
                    Self::get_next_sqrt_price_from_input(sqrt_price_x64, liquidity, amount_in, zero_for_one)
                } else {
                    // println!("liquidity is zero or sqrt_price_x64 is zero");
                    return (0,0,0);
                };
                let next_price = Self::get_price_from_sqrt_x64(next_sqrt_price_x64);
                let amount_out = if zero_for_one {
                    Self::get_delta_amount_1_unsigned(sqrt_price_x64, next_sqrt_price_x64, liquidity, false).unwrap()
                }
                else {
                    Self::get_delta_amount_0_unsigned(sqrt_price_x64, next_sqrt_price_x64, liquidity, false).unwrap()
                };

                if zero_for_one {
                    (amount_out, next_price, 0)
                }
                else {
                    (amount_out, 0, next_price)
                }
            },
            SwapType::OrcaWhirlpool | SwapType::CropperWhirlpool => {
                let price = pair.price.as_ref().unwrap();

                let amount_in = (input_amount as u128)
                    .checked_mul(pair.fee as u128)
                    .unwrap()
                    .checked_div(FEE_MULTIPLIER as u128)
                    .unwrap()
                    as u64;
                
                let zero_for_one = price.pool_token_mint_a.unwrap() == from_mint;
                let sqrt_price_x64 = price.sqrt_price_x64.unwrap();
                let liquidity = price.liquidity.unwrap();
                let next_sqrt_price_x64 = match orca_whirlpool::get_next_sqrt_price(sqrt_price_x64, liquidity, amount_in, true, zero_for_one) {
                    Ok(p)=>p,
                    Err(err) => {
                        // println!("orca_whirlpool::get_next_sqrt_price error {:#?}", err);
                        0
                    }
                };
                let next_price = Self::get_price_from_sqrt_x64(next_sqrt_price_x64);
                let amount_out = if zero_for_one {
                    orca_whirlpool::get_amount_delta_b(sqrt_price_x64, next_sqrt_price_x64, liquidity, false).expect("get_amount_delta_b error")
                }
                else {
                    orca_whirlpool::get_amount_delta_a(sqrt_price_x64, next_sqrt_price_x64, liquidity, false).expect("get_amount_delta_a error")
                };

                if zero_for_one {
                    (amount_out, next_price, 0)
                }
                else {
                    (amount_out, 0, next_price)
                }
            },
            SwapType::InvariantSwap => {
                let price = pair.price.as_ref().unwrap();

                let amount_in_val = (input_amount as u128)
                    .checked_mul(pair.fee as u128)
                    .unwrap()
                    .checked_div(FEE_MULTIPLIER as u128)
                    .unwrap()
                    as u64;
                let zero_for_one = price.pool_token_mint_a.unwrap() == from_mint;
                let sqrt_price_val = price.sqrt_price_x64.unwrap();
                let sqrt_price = invariant::decimals::Price {
                    v: sqrt_price_val
                };
                
                let liquidity_val = price.liquidity.unwrap();
                let liquidity = invariant::decimals::Liquidity{
                    v: liquidity_val
                };
                let amount_in = invariant::decimals::TokenAmount(amount_in_val);

                let next_sqrt_price = if liquidity.v > 0 && sqrt_price.v > 0 {
                    invariant_math::get_next_sqrt_price_from_input(sqrt_price, liquidity, amount_in, zero_for_one)
                } else {
                    // println!("liquidity is zero or sqrt_price_x64 is zero");
                    return (0,0,0);
                };
                let next_price = Self::get_price_from_sqrt_x64(next_sqrt_price.v);
                let amount_out = if zero_for_one {
                    invariant_math::get_delta_y(sqrt_price, next_sqrt_price, liquidity, false).unwrap_or_default().0
                }
                else {
                    invariant_math::get_delta_x(sqrt_price, next_sqrt_price, liquidity, false).unwrap_or_default().0
                };

                if zero_for_one {
                    (amount_out, next_price, 0)
                }
                else {
                    (amount_out, 0, next_price)
                }
            }
            SwapType::LifinitySwapV2 => {
                let price = pair.price.as_ref().unwrap();
                // substract fee amount from input amount
                let amount_in = (input_amount as u128)
                    .checked_mul(pair.fee as u128)
                    .unwrap()
                    .checked_div(FEE_MULTIPLIER as u128)
                    .unwrap()
                    as u64;
                let zero_for_one = price.pool_token_mint_a.unwrap() == from_mint;
                let param = CurveParam {
                    amount_in,
                    last_price: price.sqrt_price_x64.unwrap(),
                    base_decimal_val: price.base_decimal_value.unwrap(),
                    confidence: 0,
                    coin_balance: price.vault_a_amount.unwrap(),
                    pc_balance: price.vault_b_amount.unwrap(),
                    is_a_to_b: zero_for_one,
                    price_buffer_coin: price.price_buffer_coin.unwrap(),
                    price_buffer_pc: price.price_buffer_pc.unwrap(),
                    config_denominator: price.config_denominator.unwrap(),
                    regression_target: price.liquidity.unwrap() as u64,
                    spread_coefficient: price.spread_coef.unwrap(),
                    std_spread: price.std_spread.unwrap(),
                    std_spread_buffer: price.std_spread_buffer.unwrap(),
                };
                let (amount_out, next_price) = standard_curve(&param);

                let next_price: u128 = (next_price as u128)
                    .checked_mul(PRICE_MULTIPLIER as u128)
                    .unwrap()
                    .checked_div(price.base_decimal_value.unwrap() as u128)
                    .unwrap();
                if zero_for_one {
                    (amount_out, next_price, 0)
                }
                else {
                    (amount_out, 0, next_price)
                }
            }
            _ => {
                (0, 0, 0)
            }
        }
    }
    pub fn calc_amount_out_v2(
        &self,
        pair_idx: usize,
        input_amount: u64,
        from_mint: Pubkey,
        stacked_swap_steps: &mut [u64; 400],
        route_index: usize
    ) -> (u64, u128, u128) {
        

        let pairs = unsafe { &*self.bank_bot.pairs_v2.load(Ordering::SeqCst) };
        let accounts = unsafe { &* self.bank_bot.accounts_to_subscribe.load(Ordering::SeqCst)};
        let pair = &pairs[pair_idx];

        // let swap_types: Vec<SwapType> = vec![
        //     SwapType::RaydiumConcentrated, 
        //     SwapType::OrcaWhirlpool, 
        //     SwapType::CropperWhirlpool, 
        //     SwapType::InvariantSwap, 
        //     SwapType::CremaFinance, 
        //     SwapType::MeteoraDlmm,
        // ];
        // if swap_types.contains(&pair.swap_type) {
        //     println!("swap type : {:#?}", pair.swap_type);
        // }

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
                        .unwrap_or(u128::MAX) as u128;
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
                        .unwrap_or(u128::MAX) as u128;
                }
                trace!();
                (_out_amount, _next_price, _next_price_reversed)
            },
            SwapType::MeteoraDlmm =>{
                // let mut log_msg: String = pair.pool.to_string();
                let price = pair.price.as_ref().unwrap();
                let protocol_share = price.config_denominator.unwrap() as u16;
                let volatility_accumulator = price.std_spread.unwrap() as u32;
                let variable_fee_control = price.std_spread_buffer.unwrap() as u32;
                let base_factor = price.spread_coef.unwrap() as u16;
                let bin_step = price.base_decimal_value.unwrap() as u16;
                let active_id = price.price_buffer_coin.unwrap() as i32;

                // log_msg.push_str(&(" active_id:".to_owned() + active_id.to_string().as_str()));

                let lb_pair_info = LbPairInfo{
                    protocol_share,
                    volatility_accumulator, 
                    variable_fee_control, 
                    base_factor, 
                    bin_step,
                    active_id
                };

                if pair.price.as_ref().unwrap().ticks_a_to_b.is_none() || pair.price.as_ref().unwrap().ticks_b_to_a.is_none() {
                    return (0, 0, 0)
                }
                let (bin_array_pubkeys, zero_for_one) = if from_mint == pair.mint_a {
                    (pair.price.as_ref().unwrap().ticks_a_to_b.as_ref().unwrap(), true)
                } else {
                    (pair.price.as_ref().unwrap().ticks_b_to_a.as_ref().unwrap(), false)
                };
                // if pair.price.as_ref().unwrap().ticks_a_to_b.is_some() || pair.price.as_ref().unwrap().ticks_b_to_a.is_some() {
                //     println!("pool: {:#?}, bin_array_pubkeys.len() {}, ticks_a_to_b {:#?}, ticks_b_to_a {:#?}", pair.pool, bin_array_pubkeys.len(), pair.price.as_ref().unwrap().ticks_a_to_b, pair.price.as_ref().unwrap().ticks_b_to_a);
                // }
                // else {
                //     println!("ticks_a_to_b and ticks_b_to_a are none, pool: {:#?}", pair.pool);
                // }

                if bin_array_pubkeys.len() == 0 {
                    // log_msg.push_str(" returned (0,0,0)");
                    // println!("{}", log_msg);
                    // println!("bin_array_pubkeys.len() == 0");
                    return (0, 0, 0)
                };
                // println!("bin_array_pubkeys {}", bin_array_pubkeys.len());
                let mut bin_array_accs:Vec<&Vec<u8>> = Vec::new();
                for bin_array_key in bin_array_pubkeys {
                    match accounts.get(bin_array_key) {
                        Some(bin_account) => {
                            
                            bin_array_accs.push(&bin_account.account_data)
                        },
                        None => {
                            // println!("bin array doesn't exist {:#?}", bin_array_key);
                        }
                    }
                }
                // println!("bin_array_accs.len() {:#?}", bin_array_accs.len());
                if bin_array_accs.len() == 0 {
                    // log_msg.push_str(" returned (0,0,0)");
                    // println!("{}", log_msg);
                    return (0, 0, 0)
                }
                // println!("calculating next price of dlmm... {}", bin_array_accs.len());
                let price_info = Self::get_next_price_from_input(
                    input_amount, 
                    price.sqrt_price_x64.unwrap(), 
                    zero_for_one,
                    &lb_pair_info, 
                    &bin_array_accs,
                    stacked_swap_steps,
                    route_index
                ).unwrap_or(PriceInfo {
                    price: 0u128,
                    actual_amount_in: 0u64,
                    actual_amount_out: 0u64,
                    fee: 0u64,
                    protocol_fee: 0u64,
                    bin_id: 0i32
                });

                let next_price = Self::get_price_from_x64(price_info.price);

                // println!("amount_out {}, next_price {}", price_info.actual_amount_out, next_price);
                if zero_for_one {
                    (price_info.actual_amount_out, next_price, 0)
                }
                else {
                    (price_info.actual_amount_out, 0, next_price)
                }
            },
            SwapType::RaydiumConcentrated | SwapType::CremaFinance => {
                let price = pair.price.as_ref().unwrap();

                let amount_in = (input_amount as u128)
                    .checked_mul(pair.fee as u128)
                    .unwrap()
                    .checked_div(FEE_MULTIPLIER as u128)
                    .unwrap()
                    as u64;
                
                let zero_for_one = price.pool_token_mint_a.unwrap() == from_mint;
                let sqrt_price_x64 = price.sqrt_price_x64.unwrap();
                let liquidity = price.liquidity.unwrap();
                let next_sqrt_price_x64 = if liquidity > 0 && sqrt_price_x64 > 0 {
                    Self::get_next_sqrt_price_from_input(sqrt_price_x64, liquidity, amount_in, zero_for_one)
                } else {
                    // println!("liquidity is zero or sqrt_price_x64 is zero");
                    return (0,0,0);
                };
                let next_price = Self::get_price_from_sqrt_x64(next_sqrt_price_x64);
                let amount_out = if zero_for_one {
                    Self::get_delta_amount_1_unsigned(sqrt_price_x64, next_sqrt_price_x64, liquidity, false).unwrap()
                }
                else {
                    Self::get_delta_amount_0_unsigned(sqrt_price_x64, next_sqrt_price_x64, liquidity, false).unwrap()
                };

                if zero_for_one {
                    (amount_out, next_price, 0)
                }
                else {
                    (amount_out, 0, next_price)
                }
            },
            SwapType::OrcaWhirlpool | SwapType::CropperWhirlpool => {
                let price = pair.price.as_ref().unwrap();

                let amount_in = (input_amount as u128)
                    .checked_mul(pair.fee as u128)
                    .unwrap()
                    .checked_div(FEE_MULTIPLIER as u128)
                    .unwrap()
                    as u64;
                
                let zero_for_one = price.pool_token_mint_a.unwrap() == from_mint;
                let sqrt_price_x64 = price.sqrt_price_x64.unwrap();
                let liquidity = price.liquidity.unwrap();
                let next_sqrt_price_x64 = match orca_whirlpool::get_next_sqrt_price(sqrt_price_x64, liquidity, amount_in, true, zero_for_one) {
                    Ok(p)=>p,
                    Err(err) => {
                        // println!("orca_whirlpool::get_next_sqrt_price error {:#?}", err);
                        0
                    }
                };
                let next_price = Self::get_price_from_sqrt_x64(next_sqrt_price_x64);
                let amount_out = if zero_for_one {
                    orca_whirlpool::get_amount_delta_b(sqrt_price_x64, next_sqrt_price_x64, liquidity, false).expect("get_amount_delta_b error")
                }
                else {
                    orca_whirlpool::get_amount_delta_a(sqrt_price_x64, next_sqrt_price_x64, liquidity, false).expect("get_amount_delta_a error")
                };

                if zero_for_one {
                    (amount_out, next_price, 0)
                }
                else {
                    (amount_out, 0, next_price)
                }
            },
            SwapType::InvariantSwap => {
                let price = pair.price.as_ref().unwrap();

                let amount_in_val = (input_amount as u128)
                    .checked_mul(pair.fee as u128)
                    .unwrap()
                    .checked_div(FEE_MULTIPLIER as u128)
                    .unwrap()
                    as u64;
                let zero_for_one = price.pool_token_mint_a.unwrap() == from_mint;
                let sqrt_price_val = price.sqrt_price_x64.unwrap();
                let sqrt_price = invariant::decimals::Price {
                    v: sqrt_price_val
                };
                
                let liquidity_val = price.liquidity.unwrap();
                let liquidity = invariant::decimals::Liquidity{
                    v: liquidity_val
                };
                let amount_in = invariant::decimals::TokenAmount(amount_in_val);

                let next_sqrt_price = if liquidity.v > 0 && sqrt_price.v > 0 {
                    invariant_math::get_next_sqrt_price_from_input(sqrt_price, liquidity, amount_in, zero_for_one)
                } else {
                    // println!("liquidity is zero or sqrt_price_x64 is zero");
                    return (0,0,0);
                };
                let next_price = Self::get_price_from_sqrt_x64(next_sqrt_price.v);
                let amount_out = if zero_for_one {
                    invariant_math::get_delta_y(sqrt_price, next_sqrt_price, liquidity, false).unwrap_or_default().0
                }
                else {
                    invariant_math::get_delta_x(sqrt_price, next_sqrt_price, liquidity, false).unwrap_or_default().0
                };

                if zero_for_one {
                    (amount_out, next_price, 0)
                }
                else {
                    (amount_out, 0, next_price)
                }
            }
            SwapType::LifinitySwapV2 => {
                let price = pair.price.as_ref().unwrap();
                // substract fee amount from input amount
                let amount_in = (input_amount as u128)
                    .checked_mul(pair.fee as u128)
                    .unwrap()
                    .checked_div(FEE_MULTIPLIER as u128)
                    .unwrap()
                    as u64;
                let zero_for_one = price.pool_token_mint_a.unwrap() == from_mint;
                let param = CurveParam {
                    amount_in,
                    last_price: price.sqrt_price_x64.unwrap(),
                    base_decimal_val: price.base_decimal_value.unwrap(),
                    confidence: 0,
                    coin_balance: price.vault_a_amount.unwrap(),
                    pc_balance: price.vault_b_amount.unwrap(),
                    is_a_to_b: zero_for_one,
                    price_buffer_coin: price.price_buffer_coin.unwrap(),
                    price_buffer_pc: price.price_buffer_pc.unwrap(),
                    config_denominator: price.config_denominator.unwrap(),
                    regression_target: price.liquidity.unwrap() as u64,
                    spread_coefficient: price.spread_coef.unwrap(),
                    std_spread: price.std_spread.unwrap(),
                    std_spread_buffer: price.std_spread_buffer.unwrap(),
                };
                let (amount_out, next_price) = standard_curve(&param);

                let next_price: u128 = (next_price as u128)
                    .checked_mul(PRICE_MULTIPLIER as u128)
                    .unwrap()
                    .checked_div(price.base_decimal_value.unwrap() as u128)
                    .unwrap();
                if zero_for_one {
                    (amount_out, next_price, 0)
                }
                else {
                    (amount_out, 0, next_price)
                }
            }
            _ => {
                (0, 0, 0)
            }
        }
    }

    pub fn get_delta_amount_0_unsigned(
        mut sqrt_ratio_a_x64: u128,
        mut sqrt_ratio_b_x64: u128,
        liquidity: u128,
        round_up: bool,
    ) -> Result<u64, SharedMemClientErr> {
        pub const RESOLUTION: u8 = 64;
        // sqrt_ratio_a_x64 should hold the smaller value
        if sqrt_ratio_a_x64 > sqrt_ratio_b_x64 {
            std::mem::swap(&mut sqrt_ratio_a_x64, &mut sqrt_ratio_b_x64);
        };
    
        let numerator_1 = U128::from(liquidity) << RESOLUTION;
        let numerator_2 = U128::from(sqrt_ratio_b_x64 - sqrt_ratio_a_x64);
    
        assert!(sqrt_ratio_a_x64 > 0);
    
        let result = if round_up {
            U128::div_rounding_up(
                numerator_1
                    .mul_div_ceil(numerator_2, U128::from(sqrt_ratio_b_x64))
                    .unwrap(),
                    U128::from(sqrt_ratio_a_x64),
            )
        } else {
            numerator_1
                .mul_div_floor(numerator_2, U128::from(sqrt_ratio_b_x64))
                .unwrap()
                / U128::from(sqrt_ratio_a_x64)
        };
        if result > U128::from(u64::MAX) {
            return Err(SharedMemClientErr::MathOverflowError("MathOverflowError".to_string()));
        }
        return Ok(result.as_u64());
    }

    pub fn get_delta_amount_1_unsigned(
        mut sqrt_ratio_a_x64: u128,
        mut sqrt_ratio_b_x64: u128,
        liquidity: u128,
        round_up: bool,
    ) -> Result<u64, SharedMemClientErr> {
        pub const Q64: u128 = (u64::MAX as u128) + 1; // 2^64
        // sqrt_ratio_a_x64 should hold the smaller value
        if sqrt_ratio_a_x64 > sqrt_ratio_b_x64 {
            std::mem::swap(&mut sqrt_ratio_a_x64, &mut sqrt_ratio_b_x64);
        };
    
        let result = if round_up {
            U128::from(liquidity).mul_div_ceil(
                U128::from(sqrt_ratio_b_x64 - sqrt_ratio_a_x64),
                U128::from(Q64),
            )
        } else {
            U128::from(liquidity).mul_div_floor(
                U128::from(sqrt_ratio_b_x64 - sqrt_ratio_a_x64),
                U128::from(Q64),
            )
        }
        .unwrap();
        if result > U128::from(u64::MAX) {
            return Err(SharedMemClientErr::MathOverflowError("Mathoverflow".to_string()));
        }
        return Ok(result.as_u64());
    }

    pub fn get_next_sqrt_price_from_amount_0_rounding_up(
        sqrt_price_x64: u128,
        liquidity: u128,
        amount: u64,
        add: bool,
    ) -> u128 {
        pub const RESOLUTION: u8 = 64;
        if amount == 0 {
            return sqrt_price_x64;
        };
        let numerator_1 = liquidity << RESOLUTION;
    
        if add {
            if let Some(product) = sqrt_price_x64.checked_mul(amount as u128) {
                let denominator = numerator_1 + product;
                if denominator >= numerator_1 {
                    return U128::from(numerator_1)
                        .mul_div_ceil(U128::from(sqrt_price_x64), U128::from(denominator))
                        .unwrap()
                        .as_u128();
                };
            }
    
            U128::div_rounding_up(
                U128::from(numerator_1),
                U128::from((numerator_1 / sqrt_price_x64)
                    .checked_add(amount as u128)
                    .unwrap()),
            )
            .as_u128()
        } else {
            let product = sqrt_price_x64.checked_mul(amount as u128).unwrap();
            let denominator = numerator_1.checked_sub(u128::from(product)).unwrap();
            U128::from(numerator_1)
                .mul_div_ceil(U128::from(sqrt_price_x64), U128::from(denominator))
                .unwrap()
                .as_u128()
        }
    }

    pub fn get_next_sqrt_price_from_amount_1_rounding_down(
        sqrt_price_x64: u128,
        liquidity: u128,
        amount: u64,
        add: bool,
    ) -> u128 {
        pub const RESOLUTION: u8 = 64;
        if add {
            let quotient = (u128::from(amount) << RESOLUTION) / liquidity;
            sqrt_price_x64.checked_add(quotient).unwrap()
        } else {
            let quotient = (u128::from(amount) << RESOLUTION).div_ceil(liquidity);
            sqrt_price_x64.checked_sub(quotient).unwrap()
        }
    }
    
    /// Gets the next sqrt price given an input amount of token_0 or token_1
    /// Throws if price or liquidity are 0, or if the next price is out of bounds
    pub fn get_next_sqrt_price_from_input(
        sqrt_price_x64: u128,
        liquidity: u128,
        amount_in: u64,
        zero_for_one: bool,
    ) -> u128 {
        assert!(sqrt_price_x64 > 0);
        assert!(liquidity > 0);
    
        // round to make sure that we don't pass the target price
        if zero_for_one {
            Self::get_next_sqrt_price_from_amount_0_rounding_up(sqrt_price_x64, liquidity, amount_in, true)
        } else {
            Self::get_next_sqrt_price_from_amount_1_rounding_down(sqrt_price_x64, liquidity, amount_in, true)
        }
    }
    
    /// Gets the next sqrt price given an output amount of token0 or token1
    ///
    /// Throws if price or liquidity are 0 or the next price is out of bounds
    ///
    pub fn get_next_sqrt_price_from_output(
        sqrt_price_x64: u128,
        liquidity: u128,
        amount_out: u64,
        zero_for_one: bool,
    ) -> u128 {
        assert!(sqrt_price_x64 > 0);
        assert!(liquidity > 0);
    
        if zero_for_one {
            Self::get_next_sqrt_price_from_amount_1_rounding_down(
                sqrt_price_x64,
                liquidity,
                amount_out,
                false,
            )
        } else {
            Self::get_next_sqrt_price_from_amount_0_rounding_up(sqrt_price_x64, liquidity, amount_out, false)
        }
    }
    

    pub fn get_next_price_from_input(amount_in: u64, price: u128, swap_for_y: bool, lb_pair_info: &LbPairInfo, bin_array_accs: &Vec<&Vec<u8>>, stacked_swap_steps: &mut [u64; 400], route_index: usize) -> Result<PriceInfo, SharedMemClientErr> {
        pub const METEORA_DLMM_SWAP_STEP_CNT: usize = 400;
        pub const MAX_BIN_ID: i32 = 443636;
        pub const MIN_BIN_ID: i32 = -443636;

        let mut amount_in_left = amount_in;
        let mut active_id = lb_pair_info.active_id;
        let mut actual_amount_out = 0u64;
        // let mut fee_amount = 0u64;
        // let mut protocol_fee_amount = 0u64;
    
        let mut i = route_index * (METEORA_DLMM_SWAP_STEP_CNT / 4);
    
        loop {
            if i >= (route_index + 1) * (METEORA_DLMM_SWAP_STEP_CNT / 4) || stacked_swap_steps[i] == 0 {
                break;
            }
            if stacked_swap_steps[i] > amount_in_left {
                if i > 0 {
                    amount_in_left -= stacked_swap_steps[i - 2];
                    actual_amount_out = stacked_swap_steps[i -1];
                } 
                break;
            }
    
            if swap_for_y {
                active_id -= 1;
            }
            else {
                active_id += 1;
            }
            i += 2;
        }
    
        while amount_in_left > 0 {
            if active_id >= MAX_BIN_ID - 1 || active_id <= MIN_BIN_ID + 1 {
                break;
            }
            if Self::is_bin_id_within(bin_array_accs, active_id)? {
                let bin_info = Self::get_bin_info_within(bin_array_accs, active_id)?;
                let swap_result = Self::bin_swap(
                    amount_in_left, 
                    price, 
                    swap_for_y, 
                    lb_pair_info.protocol_share, 
                    bin_info.amount_x, 
                    bin_info.amount_y, 
                    lb_pair_info.volatility_accumulator, 
                    lb_pair_info.variable_fee_control, 
                    lb_pair_info.base_factor, 
                    lb_pair_info.bin_step
                )?;
                if swap_result.amount_in_with_fees > 0 {
                    amount_in_left = amount_in_left.checked_sub(swap_result.amount_in_with_fees).unwrap();
                    actual_amount_out = actual_amount_out.checked_add(swap_result.amount_out).unwrap();
                    // fee_amount = fee_amount.checked_add(swap_result.fee).unwrap();
                    // protocol_fee_amount = protocol_fee_amount.checked_add(swap_result.protocol_fee).unwrap();
    
                    if amount_in_left > 0 {
                        let swap_step_index = if lb_pair_info.active_id > active_id { (lb_pair_info.active_id - active_id) as usize } else { (active_id -  lb_pair_info.active_id) as usize };
                        if swap_step_index < METEORA_DLMM_SWAP_STEP_CNT / 8 {
                            let swap_step_index = swap_step_index * 2 + route_index * (METEORA_DLMM_SWAP_STEP_CNT / 4);
                            stacked_swap_steps[swap_step_index] = if swap_step_index == 0 { swap_result.amount_in_with_fees } else { stacked_swap_steps[swap_step_index - 2] + swap_result.amount_in_with_fees };
                            stacked_swap_steps[swap_step_index + 1] = actual_amount_out;
                        }
                        
                    }
                }
            }
            else {
                if swap_for_y {
                    active_id += 1;
                }
                else {
                    active_id -= 1;
                }
                break;
            }
            if amount_in_left > 0 {
                if swap_for_y {
                    active_id -= 1;
                }
                else {
                    active_id += 1;
                }
            }
        }
        let final_amount_in = amount_in - amount_in_left;
        let end_price = Self::get_price_from_id(active_id, lb_pair_info.bin_step)?;
        Ok(PriceInfo{
            price: end_price,
            actual_amount_in: final_amount_in,
            actual_amount_out,
            fee: 0,
            protocol_fee: 0,
            bin_id: active_id
        })
    }

    pub fn bin_swap(
        amount_in: u64,
        price: u128,
        swap_for_y: bool, 
        protocol_share: u16,
        amount_x: u64,
        amount_y: u64, 
        volatility_accumulator: u32, 
        variable_fee_control: u32, 
        base_factor: u16, 
        bin_step: u16
    ) -> Result<SwapResult, SharedMemClientErr> {
        // Get maximum out token amount can be swapped out from the bin.
        let max_amount_out = Self::get_max_amount_out(swap_for_y, amount_x, amount_y);
        // Get maximum in token amount needed to swap out all of the opposite token from the bin.
        let mut max_amount_in = Self::get_max_amount_in(price, swap_for_y, amount_x, amount_y)?;
    
        // The fee was deducted from the amount_in if the swap will not move the active bin. So, the amount_in include fees
        // When the amount_in > max_amount_in, it will swap finish all the current bin token X/Y based on the swap direction.
        // However, max_amount_in is amount that required to swap finish the current bin without fee
        // Therefore, we need find max_amount_in_include_fees, where max_amount_in_include_fees - fee = max_amount_in
        let max_fee = Self::compute_fee(max_amount_in, volatility_accumulator, variable_fee_control, base_factor, bin_step)?;
        max_amount_in = max_amount_in.safe_add(max_fee)?;
    
        // If the in token amount > maximum token amount needed to swap out all of the opposite token from the bin.
        let (amount_in_with_fees, amount_out, fee, protocol_fee) = if amount_in > max_amount_in {
            (
                max_amount_in,
                max_amount_out,
                max_fee,
                Self::compute_protocol_fee(max_fee, protocol_share)?,
            )
        } else {
            // TODO: User possible to bypass fee by swapping small amount ? User do a "normal" swap by just bundling all small swap that bypass fee ?
            let fee = Self::compute_fee_from_amount(amount_in, volatility_accumulator, variable_fee_control, base_factor, bin_step)?;
            let amount_in_after_fee = amount_in.safe_sub(fee)?;
            let amount_out = Self::bin_get_amount_out(amount_in_after_fee, price, swap_for_y)?;
            (
                amount_in,
                std::cmp::min(amount_out, max_amount_out),
                fee,
                Self::compute_protocol_fee(fee, protocol_share)?,
            )
        };
    
        // let host_fee = match host_fee_bps {
        //     Some(bps) => protocol_fee
        //         .safe_mul(bps.into())?
        //         .safe_div(BASIS_POINT_MAX as u64)?,
        //     None => 0,
        // };
    
        // let protocol_fee_after_host_fee = protocol_fee.safe_sub(host_fee)?;
    
        // Exclude fee and protocol fee. Protocol fee already part of fee. User need to claim the fee later.
        let amount_into_bin = amount_in_with_fees.safe_sub(fee)?;
    
        let mut new_amount_x = amount_x;
        let mut new_amount_y = amount_y;
        if swap_for_y {
            new_amount_x = new_amount_x.safe_add(amount_into_bin)?;
            new_amount_y = new_amount_y.safe_sub(amount_out)?;
        } else {
            new_amount_y = new_amount_y.safe_add(amount_into_bin)?;
            new_amount_x = new_amount_x.safe_sub(amount_out)?;
        }
    
        Ok(SwapResult {
            amount_in_with_fees,
            amount_out,
            fee,
            protocol_fee,
            new_amount_x,
            new_amount_y,
        })
    }

    pub fn compute_fee_from_amount(amount_with_fees: u64, volatility_accumulator: u32, variable_fee_control: u32, base_factor: u16, bin_step: u16) -> Result<u64, SharedMemClientErr> {
        pub const FEE_PRECISION: u64 = 1000000000;

        // total_fee_rate 1e9 unit
        let total_fee_rate = Self::get_total_fee(volatility_accumulator, variable_fee_control, base_factor, bin_step)?;
        // Ceil division
        let fee_amount = u128::from(amount_with_fees)
            .safe_mul(total_fee_rate)?
            .safe_add((FEE_PRECISION - 1).into())?;
        let scaled_down_fee = fee_amount.safe_div(FEE_PRECISION.into())?;
    
        Ok(scaled_down_fee
            .try_into()
            .map_err(|_| SharedMemClientErr::TypeCastFailed("TypeCastFailed".to_string()))?)
    }

    pub fn compute_protocol_fee(fee_amount: u64, protocol_share: u16) -> Result<u64, SharedMemClientErr> {
        pub const BASIS_POINT_MAX: i32 = 10000;
        let protocol_fee = u128::from(fee_amount)
            .safe_mul(protocol_share.into())?
            .safe_div(BASIS_POINT_MAX as u128)?;
    
        Ok(protocol_fee
            .try_into()
            .map_err(|_| SharedMemClientErr::TypeCastFailed("TypeCastFailed".to_string()))?)
    }

    pub fn compute_fee(amount: u64, volatility_accumulator: u32, variable_fee_control: u32, base_factor: u16, bin_step: u16) -> Result<u64, SharedMemClientErr> {
        pub const FEE_PRECISION: u64 = 1000000000;
        let total_fee_rate = Self::get_total_fee(volatility_accumulator, variable_fee_control, base_factor, bin_step)?;
        let denominator = u128::from(FEE_PRECISION).safe_sub(total_fee_rate)?;
    
        // Ceil division
        let fee = u128::from(amount)
            .safe_mul(total_fee_rate)?
            .safe_add(denominator)?
            .safe_sub(1)?;
    
        let scaled_down_fee = fee.safe_div(denominator)?;
    
        Ok(scaled_down_fee
            .try_into()
            .map_err(|_| SharedMemClientErr::TypeCastFailed("TypeCastFailed".to_string()))?)
    }

    pub fn get_max_amount_out(swap_for_y: bool, amount_x: u64, amount_y: u64) -> u64 {
        if swap_for_y {
            amount_y
        } else {
            amount_x
        }
    }

    pub fn get_max_amount_in(price: u128, swap_for_y: bool, amount_x: u64, amount_y: u64) -> Result<u64, SharedMemClientErr> {
        pub const SCALE_OFFSET: u8 = 64;

        if swap_for_y {
            // (amount_y << SCALE_OFFSET) / price
            // Convert amount_y into Q64x0, if not the result will always in 0 as price is in Q64x64
            // Division between same Q number format cancel out, result in integer
            // amount_y / price = amount_in_token_x (integer [Rounding::Up])
            safe_shl_div_cast(amount_y.into(), price, SCALE_OFFSET, Rounding::Up)
        } else {
            // (Q64x64(price) * Q64x0(amount_x)) >> SCALE_OFFSET
            // price * amount_x = amount_in_token_y (Q64x64)
            // amount_in_token_y >> SCALE_OFFSET (convert it back to integer form [Rounding::Up])
            safe_mul_shr_cast(amount_x.into(), price, SCALE_OFFSET, Rounding::Up)
        }
    }

    pub fn bin_get_amount_out(amount_in: u64, price: u128, swap_for_y: bool) -> Result<u64, SharedMemClientErr> {
        pub const SCALE_OFFSET: u8 = 64;
        if swap_for_y {
            // (Q64x64(price) * Q64x0(amount_in)) >> SCALE_OFFSET
            // price * amount_in = amount_out_token_y (Q64x64)
            // amount_out_in_token_y >> SCALE_OFFSET (convert it back to integer form, with some loss of precision [Rounding::Down])
            safe_mul_shr_cast(price, amount_in.into(), SCALE_OFFSET, Rounding::Down)
        } else {
            // (amount_in << SCALE_OFFSET) / price
            // Convert amount_in into Q64x0, if not the result will always in 0 as price is in Q64x64
            // Division between same Q number format cancel out, result in integer
            // amount_in / price = amount_out_token_x (integer [Rounding::Down])
            safe_shl_div_cast(amount_in.into(), price, SCALE_OFFSET, Rounding::Down)
        }
    }

    pub fn get_bin_info_within<'info>(bin_array_accs: &Vec<&Vec<u8>>, bin_id: i32) -> Result<BinInfo, SharedMemClientErr> {
        for account_info in bin_array_accs.iter() {
            if Self::is_bin_id(account_info, bin_id)? {
                return Self::get_bin_info(account_info, bin_id);
            }
        }
        Ok(BinInfo{
            amount_x: 0u64,
            amount_y: 0u64,
            price: 0u128
        })
    }

    pub fn is_bin_id_within<'info>(bin_array_accs: &Vec<&Vec<u8>>, bin_id: i32) -> Result<bool, SharedMemClientErr> {
        for account_info in bin_array_accs.iter() {
            if Self::is_bin_id(account_info, bin_id)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn get_bin_info<'info>(bin_array_acc_info: &Vec<u8>, bin_id: i32) -> Result<BinInfo, SharedMemClientErr> {
        pub const BINS_POS: usize = 56;
        pub const BIN_SIZE: usize = 144;

        let index = Self::get_bin_array_index(bin_array_acc_info);
        let bin_index = Self::get_bin_index_in_array(index as i32, bin_id)?;
        let bin_start_pos = BINS_POS + BIN_SIZE * bin_index;
    
        let input = array_ref![bin_array_acc_info.as_slice(), bin_start_pos, 32];
        let (amount_x_slice, amount_y_slice, price_slice) = array_refs![input, 8, 8, 16];
        let amount_x = u64::from_le_bytes(*amount_x_slice);
        let amount_y = u64::from_le_bytes(*amount_y_slice);
        let price = u128::from_le_bytes(*price_slice);
        Ok(BinInfo{
            amount_x,
            amount_y,
            price
        })
    }

    pub fn is_bin_id(bin_array_acc_info: &Vec<u8>, bin_id: i32) -> Result<bool, SharedMemClientErr> {
        let index = Self::get_bin_array_index(bin_array_acc_info);
        let (lower_bin_id, upper_bin_id) = Self::get_bin_array_lower_upper_bin_id(index as i32)?;
        Ok(bin_id >= lower_bin_id && bin_id <= upper_bin_id)
    }

    pub fn get_bin_array_lower_upper_bin_id(index: i32) -> Result<(i32, i32), SharedMemClientErr> {
        pub const MAX_BIN_PER_ARRAY: usize = 70;
        let lower_bin_id = index.safe_mul(MAX_BIN_PER_ARRAY as i32)?;
        let upper_bin_id = lower_bin_id
            .safe_add(MAX_BIN_PER_ARRAY as i32)?
            .safe_sub(1)?;
    
        Ok((lower_bin_id, upper_bin_id))
    }
    pub fn get_bin_array_index(bin_array_acc_info: &Vec<u8>) -> i64 {
        pub const BIN_ARRAY_INDEX_POS: usize = 8;
        
        let input = array_ref![bin_array_acc_info.as_slice(), 0, BIN_ARRAY_INDEX_POS + 8usize];
        let (_, index_slice) = array_refs![input, BIN_ARRAY_INDEX_POS, 8];
        
        let bin_index = i64::from_le_bytes(*index_slice);
        bin_index
    }

    fn get_bin_index_in_array(index: i32, bin_id: i32) -> Result<usize, SharedMemClientErr> {
        pub const MAX_BIN_PER_ARRAY: usize = 70;
        let (lower_bin_id, upper_bin_id) = Self::get_bin_array_lower_upper_bin_id(index as i32)?;
    
        let index = if bin_id.is_positive() {
            // When bin id is positive, the index is ascending
            bin_id.safe_sub(lower_bin_id)?
        } else {
            // When bin id is negative, the index is descending. Eg: bin id -1 will be located at last index of the bin array
            ((MAX_BIN_PER_ARRAY as i32).safe_sub(upper_bin_id.safe_sub(bin_id)?)?).safe_sub(1)?
        };
    
        if index >= 0 && index < MAX_BIN_PER_ARRAY as i32 {
            Ok(index as usize)
        } else {
            Err(SharedMemClientErr::InvalidBinId("InvalidedBinId".to_string()))
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
        acc_price
    }
    pub fn get_acc_price_v2(&self, route_idx: usize) -> u128 {
        let routes = unsafe { &*self.bank_bot.routes_v2.load(Ordering::SeqCst) };
        let pairs = unsafe { &*self.bank_bot.pairs_v2.load(Ordering::SeqCst) };
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
        acc_price
    }

    pub fn calc_next_price(&self, route_idx: usize, input_amount: u64) -> (u128, u64) {
        let routes = unsafe { &*self.bank_bot.routes.load(Ordering::SeqCst) };
        let pairs = unsafe { &*self.bank_bot.pairs.load(Ordering::SeqCst) };
        let route = &routes[route_idx];
        trace!();

        let usdc_mint = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
        let wsol_mint = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();

        let mut mid_amount = input_amount;
        let mut acc_price = PRICE_MULTIPLIER as u128;
        let mut from_mint = if route.route_start_mint {wsol_mint} else {usdc_mint};

        for (route_index, pair_idx) in route.route.iter().enumerate() {
            trace!();
            let pair = &pairs[*pair_idx as usize];
            let mut stacked_swap_steps = [0u64; 400];
            let fee = pair.fee;
            let (out_amount, next_price, next_price_reversed) =
                self.calc_amount_out(*pair_idx as usize, mid_amount, from_mint, &mut stacked_swap_steps, route_index);
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
                        .unwrap_or(0)
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
    pub fn calc_next_price_v2(&self, route_idx: usize, input_amount: u64) -> (u128, u64) {
        let routes = unsafe { &*self.bank_bot.routes_v2.load(Ordering::SeqCst) };
        let pairs = unsafe { &*self.bank_bot.pairs_v2.load(Ordering::SeqCst) };
        let route = &routes[route_idx];
        trace!();

        let usdc_mint = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
        let wsol_mint = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();

        let mut mid_amount = input_amount;
        let mut acc_price = PRICE_MULTIPLIER as u128;
        let mut from_mint = if route.route_start_mint {wsol_mint} else {usdc_mint};

        for (route_index, pair_idx) in route.route.iter().enumerate() {
            trace!();
            let pair = &pairs[*pair_idx as usize];
            let mut stacked_swap_steps = [0u64; 400];
            let fee = pair.fee;
            let (out_amount, next_price, next_price_reversed) =
                self.calc_amount_out_v2(*pair_idx as usize, mid_amount, from_mint, &mut stacked_swap_steps, route_index);
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
                        .unwrap_or(0)
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

        let ignore_unit = 1000;
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
    pub fn determine_input_amount_v2(
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
            (_next_price, _) = self.calc_next_price_v2(route_idx, bound_amount);

            if _next_price < PRICE_MULTIPLIER as u128 || bound_amount >= max_input {
                break;
            }
            min_amount = bound_amount;
            bound_amount = bound_amount.checked_mul(10).unwrap();
        }

        let ignore_unit = 1000;
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

            (_next_price, out_amount) = self.calc_next_price_v2(route_idx, mid_amount);

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
    pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
        let preimage = format!("{namespace}:{name}");

        let mut sighash = [0u8; 8];
        sighash.copy_from_slice(&crate::hash::hash(preimage.as_bytes()).to_bytes()[..8]);
        sighash
    }

    pub fn get_trade_swap_ix(accounts: &Vec<AccountMeta>, params: &TradeSwapParams) -> Instruction {
        // trace!();
        let program_id = Pubkey::from_str("botHDy47CNugroED1sxHfYeAiXwDKbgaa1WUeBvqnej").unwrap();
        let discriminator = Self::sighash("global", "xswap");

        Instruction::new_with_borsh(program_id, &(discriminator, params), accounts.to_vec())
    }

    pub fn get_trade_pay_jito_tip_ix(
        accounts: &Vec<AccountMeta>,
        params: &TradePayJitoTipParams,
    ) -> Instruction {
        let program_id = Pubkey::from_str("botHDy47CNugroED1sxHfYeAiXwDKbgaa1WUeBvqnej").unwrap();
        let discriminator = Self::sighash("global", "tradex_pay_jito_tip");

        Instruction::new_with_borsh(
            program_id,
            &(discriminator, params),
            accounts.to_vec(),
        )
    }

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
        
        let pool_state_change_types: Vec<SwapType> = vec![
            SwapType::RaydiumConcentrated, 
            SwapType::OrcaWhirlpool, 
            SwapType::CropperWhirlpool, 
            SwapType::InvariantSwap, 
            SwapType::CremaFinance, 
            SwapType::MeteoraDlmm,

            SwapType::BonkSwap
        ];
        // let usdc_mint = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
        // let wsol_mint = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
        // let measure_price_change = if (self.mint_a.eq(&wsol_mint) && self.mint_b.eq(&usdc_mint)) || (self.mint_b.eq(&wsol_mint) && self.mint_a.eq(&usdc_mint)) {
        //     false
        // }
        // else {
        //     true
        // };
        let measure_price_change = true;

        if pool_state_change_types.contains(&self.swap_type) {
            vec![
                (self.pool_state, measure_price_change)
            ]
        } else if self.swap_type == SwapType::MeteoraPools {
            vec![
                (self.accounts_vec_a_to_b[8].pubkey, false),
                (self.accounts_vec_a_to_b[9].pubkey, false),
                (self.accounts_vec_a_to_b[10].pubkey, false),
                (self.accounts_vec_a_to_b[11].pubkey, false),
                (self.pool_vault_a, measure_price_change),
                (self.pool_vault_b, false)
            ]
        } 
        else {
            vec![
                (self.pool_vault_a, measure_price_change),
                (self.pool_vault_b, false)
            ]
        }
    }
}

#[derive(Debug)]
pub struct PairV2 {
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
    pub lookup: Pubkey,
}

impl PairV2 {
    pub const PAIR_SLICE_SIZE: usize = 2061; //1+32+32+32+4+1+1+34×50+32+32+32+32+32+1+1+32+32+32
                                             // swap_type + pool + mint_a + mint_b + fee + data_index_a_to_b, data_index_b_to_a, accounts_vec_a_to_b, accounts_vec_b_to_a, user_vault_a, user_vault_b, pool_state, pool_vault_a, pool_vault_b, mint_a_decimals, mint_b_decimals, mint_a_token_program, mint_b_token_program, lookup

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
            lookup_slice,
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

        PairV2 {
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
            lookup: Pubkey::new_from_array(*lookup_slice),
        }
    }
    pub fn get_all_accounts_for_subscribe(&self) -> Vec<(Pubkey, bool)> {
        
        let pool_state_change_types: Vec<SwapType> = vec![
            SwapType::RaydiumConcentrated, 
            SwapType::OrcaWhirlpool, 
            SwapType::CropperWhirlpool, 
            SwapType::InvariantSwap, 
            SwapType::CremaFinance, 
            SwapType::MeteoraDlmm,

            SwapType::BonkSwap
        ];
        // let usdc_mint = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
        // let wsol_mint = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
        // let measure_price_change = if (self.mint_a.eq(&wsol_mint) && self.mint_b.eq(&usdc_mint)) || (self.mint_b.eq(&wsol_mint) && self.mint_a.eq(&usdc_mint)) {
        //     false
        // }
        // else {
        //     true
        // };
        let measure_price_change = true;

        if pool_state_change_types.contains(&self.swap_type) {
            vec![
                (self.pool_state, measure_price_change)
            ]
        } else if self.swap_type == SwapType::MeteoraPools {
            vec![
                (self.accounts_vec_a_to_b[8].pubkey, false),
                (self.accounts_vec_a_to_b[9].pubkey, false),
                (self.accounts_vec_a_to_b[10].pubkey, false),
                (self.accounts_vec_a_to_b[11].pubkey, false),
                (self.pool_vault_a, measure_price_change),
                (self.pool_vault_b, false)
            ]
        } 
        else {
            vec![
                (self.pool_vault_a, measure_price_change),
                (self.pool_vault_b, false)
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

            SwapType::LifinitySwapV2 => 100000,

            SwapType::RaydiumConcentrated => 100000,
            SwapType::CremaFinance => 100000,
            SwapType::OrcaWhirlpool => 60000,
            SwapType::CropperWhirlpool => 60000,
            SwapType::InvariantSwap => 150000,
            SwapType::MeteoraDlmm => 80000,

            _ => 0,
        }
    }
}

#[derive(Debug)]
pub struct SubscribeAccount {
    pub account_data: Vec<u8>,
    pub pair_index: Option<usize>
}

#[derive(Debug)]
pub struct Price {
    pub price_a_to_b: u128,
    pub price_b_to_a: u128,
    pub vault_a_amount: Option<u64>,
    pub vault_b_amount: Option<u64>,

    // ticks here
    pub ticks_a_to_b: Option<Vec<Pubkey>>,
    pub ticks_b_to_a: Option<Vec<Pubkey>>,

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
    pub price_buffer_pc: Option<i64>,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct RouteState {
    pub acc_price: u128,
    pub pnl: u64,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct IxZipItem {
    pub lookup_idx: u16,
    pub address_idx: u8,
    pub is_signer: bool,
    pub is_writable: bool,
}
#[derive(Clone, Debug)]
pub struct RouteV2 {
    pub route: Vec<u32>,
    pub instruction: Instruction,
    pub lookuptables: Vec<AddressLookupTableAccount>,
    pub route_start_mint: bool,
}
impl RouteV2 {
    pub fn update_amount(&mut self, amount: u64) {
        let amount_start_index: usize = 8 + 1 * 7;
        self.instruction.data[amount_start_index..amount_start_index + 8]
            .copy_from_slice(&amount.to_le_bytes());
    }
    pub fn get_v2_instruction(route: &Vec<u32>, pairs: &Vec<PairV2>) -> Option<Instruction> {
        let wsol_mint_pubkey =
            Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
        let usdc_mint_pubkey =
            Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
        let wsol_ata_pubkey =
            Pubkey::from_str("Gm6TTgZXjATYcpBxkmJnCZbzeeoJ7TMJKwDLMaLFSpE1").unwrap();
        let usdc_ata_pubkey =
            Pubkey::from_str("BChF15Y7PAwEhWNCZxfa6AujRXZiHVTecsbH5CuV1jzD").unwrap();
        let ata_program_pubkey =
            Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap();
        let pda_signer_pubkey =
            Pubkey::from_str("D6k1znFSoG8Am73BBeY1JLpiDfQyqefTmdXoHQnZef7d").unwrap();

            let tokens = vec![
            pairs[route[0] as usize].mint_a,
            pairs[route[0] as usize].mint_b,
            pairs[route[1] as usize].mint_a,
            pairs[route[1] as usize].mint_b,
        ];
        // if Route::find_first_mint(&tokens).is_none() {
        //     println!("{:#?}", tokens);
        // }
        let route_start_mint = Route::find_first_mint(&tokens).unwrap();

        let mut route_pairs: Vec<&PairV2> = Vec::new();

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

        let pda_signer_index_res = account_metas.iter().position(|meta| meta.pubkey == pda_signer_pubkey);
        if pda_signer_index_res.is_none() {
            println!("failed to find pda signer when reading routes");
            return None;
        }
        let pda_signer_index = pda_signer_index_res.unwrap();
        account_metas[pda_signer_index].is_signer = false;

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
        Some(ix)
            
    }
}
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct Route {
    pub route: Vec<u32>,
    pub route_start_mint: bool,
    pub ix_data: Vec<u8>,
    pub ix_zip_items: Vec<IxZipItem>,
    pub original_accounts_cnt: usize,
    pub cu: u32,
    pub lookuptables: Vec<u16>,
}

impl Route {
    pub fn default() -> Self {
        Self {
            route: Vec::new(),
            route_start_mint: true,
            ix_data: Vec::new(),
            ix_zip_items: Vec::new(),
            original_accounts_cnt: 0,
            cu: 0,
            lookuptables: Vec::new()
        }
    }
    pub fn get_instruction_with_ticks(&self, total_lookuptables: &Vec<AddressLookupTableAccount>, pairs: &Vec<Pair>) -> (Instruction, Vec<u16>) {
        let program_id = Pubkey::from_str("botHDy47CNugroED1sxHfYeAiXwDKbgaa1WUeBvqnej").unwrap();
        let mut accounts = Vec::new();
        for item in self.ix_zip_items.iter() {
            if item.lookup_idx == u16::MAX {
                if item.address_idx == 0 {
                    let pubkey = Pubkey::from_str("HAw3XK6uRMXPrjvLjjZFW2PQRKDGJPQhxdxBL3wFrUyu").unwrap();
                    accounts.push(AccountMeta {
                        pubkey,
                        is_signer: item.is_signer,
                        is_writable: item.is_writable
                    });
                }
                else if item.address_idx == 1 {
                    let pubkey = Pubkey::from_str("6ZjmH3cRhwNcLk87ovRLnXYpWMEedhcYqKuuxCscVND4").unwrap();
                    accounts.push(AccountMeta {
                        pubkey,
                        is_signer: item.is_signer,
                        is_writable: item.is_writable
                    });
                }
            }
            else {
                let pubkey = &total_lookuptables[item.lookup_idx as usize].addresses[item.address_idx as usize];
                accounts.push(AccountMeta {
                    pubkey: pubkey.clone(),
                    is_signer: item.is_signer,
                    is_writable: item.is_writable
                });
            }
            
        }
        let mut ix = Instruction {
            program_id,
            accounts,
            data: self.ix_data.clone()
        };

        
        let usdc_mint = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
        let wsol_mint = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();

        let mut from_mint = if self.route_start_mint {wsol_mint} else {usdc_mint};
        let pair_cnt = self.route.len();

        
        for (pair_pos, pair_idx) in self.route.iter().enumerate() {
            let pair = &pairs[*pair_idx as usize];
            if let Some(clmm_index) = CLMM_SWAP_TYPES.iter().position(|c| c.eq(&pair.swap_type)) {
                if pair.price.is_some() {
                    let (account_index_pos, ticks_len) = CLMM_TICK_START_POS[clmm_index];
                    let (ticks_opt, is_a_to_b, from_mint_next) = if from_mint == pair.mint_a {
                        (&pair.price.as_ref().unwrap().ticks_a_to_b, true, pair.mint_b)
                    } else {
                        (&pair.price.as_ref().unwrap().ticks_b_to_a, false, pair.mint_a)
                    };
                    from_mint = from_mint_next;
                    if let Some(ticks) = ticks_opt {
                        // ix.accounts.truncate(self.original_accounts_cnt);
                        // let mut tick_pos = (ix.accounts.len() - 5) as u8;
                        const ROUTES_START_POS: usize = 8 + 7 + 8 + 4;
                        const TRADE_SWAP_ELEMENT_LEN: usize = 59;

                        let start_pos: usize = ROUTES_START_POS + TRADE_SWAP_ELEMENT_LEN * pair_pos;
                        let end_pos: usize = if pair_pos == pair_cnt - 1 { ix.data.len() } else { ROUTES_START_POS + TRADE_SWAP_ELEMENT_LEN * (pair_pos + 1) };
                        let serialized_route_slice = &mut ix.data[start_pos..end_pos];
                        
                        let tick_cnt = ticks_len.min(ticks.len());
                        let mut prev_account_index = 0;
                        let mut skipped_ticks = 0;
                        for tick_idx in 0..tick_cnt {
                            let account_index_in_vec = if is_a_to_b {
                                9 + account_index_pos + tick_idx
                            } else {
                                34 + account_index_pos + tick_idx
                            };
                            let account_index = serialized_route_slice[account_index_in_vec];
                            if account_index == prev_account_index || account_index == 0 {
                                skipped_ticks += 1;
                                // let new_account_index = (ix.accounts.len() - 5) as u8;
                                // serialized_route_slice[account_index_in_vec] = new_account_index;
                                // ix.accounts.push(AccountMeta {pubkey: ticks[tick_idx].clone(), is_signer: false, is_writable: true});

                                // println!("{:#?}, {}: {:#?} -> {:#?} , atob:{}, type: {:#?}", account_index_pos + tick_idx, new_account_index, ix.accounts[(new_account_index + 5) as usize].pubkey, ticks[tick_idx], is_a_to_b, pair.swap_type);
                            }
                            else {
                                // println!("{:#?}, {}: {:#?} -> {:#?} , atob:{}, type: {:#?}", account_index_pos + tick_idx, new_account_index, ix.accounts[(new_account_index + 5) as usize].pubkey, ticks[tick_idx], is_a_to_b, pair.swap_type);
                                ix.accounts[(account_index + 5) as usize].pubkey = ticks[tick_idx - skipped_ticks].clone();
                                ix.accounts[(account_index + 5) as usize].is_writable = true;
                                prev_account_index = account_index;
                            }
                        }
                        
                    }
                    else {
                        println!("ticks doesn't exist for pool {}", pair.pool.to_string());
                    }
                }
                
            }
        }

        // update lookuptables
        let lookuptables = Self::get_lookups(total_lookuptables, &ix.accounts);
        (ix, lookuptables)

    }
    pub fn zip_items_from(ix: &Instruction, total_lookuptables: &Vec<AddressLookupTableAccount>) -> Vec<IxZipItem> {

        let mut items = Vec::new();
        for meta in ix.accounts.iter() {
            let origin_len = items.len();
            for lookup_idx in 0..total_lookuptables.len() {
                for address_idx in 0..total_lookuptables[lookup_idx].addresses.len() {
                    if total_lookuptables[lookup_idx].addresses[address_idx].eq(&meta.pubkey) {
                        items.push(IxZipItem {
                            lookup_idx: lookup_idx as u16,
                            address_idx: address_idx as u8,
                            is_signer: meta.is_signer,
                            is_writable: meta.is_writable
                        });
                    }
                }
            }
            let items_len = items.len();
            if items_len == origin_len {
                if meta.pubkey.to_string().contains("HAw3XK6uRMXPrjvLjjZFW2PQRKDGJPQhxdxBL3wFrUyu") {
                    items.push(IxZipItem {
                        lookup_idx: u16::MAX,
                        address_idx: 0,
                        is_signer: true,
                        is_writable: true
                    });
                }
                else if meta.pubkey.to_string().contains("6ZjmH3cRhwNcLk87ovRLnXYpWMEedhcYqKuuxCscVND4") {
                    items.push(IxZipItem {
                        lookup_idx: u16::MAX,
                        address_idx: 1,
                        is_signer: false,
                        is_writable: true
                    });
                }
                else {
                    println!("this address is not in the lookup {:#?}", meta.pubkey);
                }
                
            }
        }
        items
    }
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

    pub fn build_route( // will build route without tick
        route: &Vec<u32>,
        pairs: &Vec<Pair>,
        total_lookuptables: &Vec<AddressLookupTableAccount>,
    ) -> Self {
        let wsol_mint_pubkey =
            Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
        let usdc_mint_pubkey =
            Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
        let wsol_ata_pubkey =
            Pubkey::from_str("Gm6TTgZXjATYcpBxkmJnCZbzeeoJ7TMJKwDLMaLFSpE1").unwrap();
        let usdc_ata_pubkey =
            Pubkey::from_str("BChF15Y7PAwEhWNCZxfa6AujRXZiHVTecsbH5CuV1jzD").unwrap();
        let ata_program_pubkey =
            Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap();
        let pda_signer_pubkey =
            Pubkey::from_str("D6k1znFSoG8Am73BBeY1JLpiDfQyqefTmdXoHQnZef7d").unwrap();

        let mut cu: u32 = 30000; // default anchor cu

        let tokens = vec![
            pairs[route[0] as usize].mint_a,
            pairs[route[0] as usize].mint_b,
            pairs[route[1] as usize].mint_a,
            pairs[route[1] as usize].mint_b,
        ];
        // if Route::find_first_mint(&tokens).is_none() {
        //     println!("{:#?}", tokens);
        // }
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

        let lookuptables = Route::get_lookups(total_lookuptables, &account_metas);

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

        let original_accounts_cnt = main_acc_meta.len();

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

        Route {
            route: route.clone(),
            route_start_mint: route_start_mint == wsol_mint_pubkey,
            ix_zip_items: Route::zip_items_from(&ix, total_lookuptables),
            ix_data: ix.data,
            original_accounts_cnt,
            cu,
            lookuptables,
        }
    }

    pub fn set_account_index_from_serialize(serialized: &mut Vec<u8>, pair_cnt: usize, pair_idx: usize, account_index_pos: usize, account_index: u8, is_a_to_b: bool) {
        const ROUTES_START_POS: usize = 8 + 7 + 8;
        const TRADE_SWAP_ELEMENT_LEN: usize = 59;

        let start_pos = ROUTES_START_POS + TRADE_SWAP_ELEMENT_LEN * pair_idx;
        let end_pos = if pair_idx == pair_cnt - 1 { serialized.len() } else { ROUTES_START_POS + TRADE_SWAP_ELEMENT_LEN * (pair_idx + 1) };

        let serialized_route_slice = &mut serialized[start_pos..end_pos];

        if is_a_to_b {
            serialized_route_slice[9 + account_index_pos] = account_index;
        } else {
            serialized_route_slice[34 + account_index_pos] = account_index;
        }
    }


    pub fn update_amount(&mut self, amount: u64) {
        let amount_start_index: usize = 8 + 1 * 7;
        self.ix_data[amount_start_index..amount_start_index + 8]
            .copy_from_slice(&amount.to_le_bytes());
    }

    pub fn get_lookups(
        lookuptables: &Vec<AddressLookupTableAccount>,
        account_metas: &[AccountMeta],
    ) -> Vec<u16> {
        let mut lookups: Vec<u16> = vec![0]; // default lookup

        for account_meta in account_metas {
            for lookuptable in 0..lookuptables.len() {
                if lookuptables[lookuptable].addresses.contains(&account_meta.pubkey) {
                    lookups.push(lookuptable as u16);
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
        let program_id = Pubkey::from_str("botHDy47CNugroED1sxHfYeAiXwDKbgaa1WUeBvqnej").unwrap();
        let discriminator = Route::sighash("global", "xswap");

        Instruction::new_with_borsh(program_id, &(discriminator, params), accounts.to_vec())
    }

    pub fn get_trade_pay_jito_tip_ix(
        accounts: &Vec<AccountMeta>,
        params: &TradePayJitoTipParams,
    ) -> Instruction {
        let program_id = Pubkey::from_str("botHDy47CNugroED1sxHfYeAiXwDKbgaa1WUeBvqnej").unwrap();
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
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct LookupTableV2 {
    pub lookup: String,
    pub addresses: Vec<String>,
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
    pub odd: u64,
    pub same_bundle_cnt: u8,
    pub sim_pnl: u64
}

#[derive(Debug)]
pub enum SharedMemClientErr {
    MathOverflowError(String),
    InvalidBinId(String),
    TypeCastFailed(String),
    LiquidityOverflow(String),
    LiquidityUnderflow(String),
    DivideByZero(String),
    MulDivOverflow(String),
    MultiplicationOverflow(String),
    LiquidityTooHigh(String),
    MultiplicationShiftRightOverflow(String),
    TokenMaxExceeded(String),
    TokenMinSubceeded(String),
    SqrtPriceOutOfBounds(String),
    NumberDownCastError(String),
    NumberCastError(String)
}

impl fmt::Display for SharedMemClientErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SharedMemClientErr::MathOverflowError(ref msg) => write!(f, "Mathoverflow Error: {}", msg),
            SharedMemClientErr::InvalidBinId(ref msg) => write!(f, "InvalidBinId Error: {}", msg),
            SharedMemClientErr::TypeCastFailed(ref msg) => write!(f, "TypeCastFailed Error: {}", msg),
            SharedMemClientErr::LiquidityOverflow(ref msg) => write!(f, "LiquidityOverflow Error: {}", msg),
            SharedMemClientErr::LiquidityUnderflow(ref msg) => write!(f, "LiquidityUnderflow Error: {}", msg),
            SharedMemClientErr::DivideByZero(ref msg) => write!(f, "DivideByZero Error: {}", msg),
            SharedMemClientErr::MulDivOverflow(ref msg) => write!(f, "MulDivOverflow Error: {}", msg),
            SharedMemClientErr::MultiplicationOverflow(ref msg) => write!(f, "MultiplicationOverflow Error: {}", msg),
            SharedMemClientErr::LiquidityTooHigh(ref msg) => write!(f, "LiquidityTooHigh Error: {}", msg),
            SharedMemClientErr::MultiplicationShiftRightOverflow(ref msg) => write!(f, "MultiplicationShiftRightOverflow Error: {}", msg),
            SharedMemClientErr::TokenMaxExceeded(ref msg) => write!(f, "TokenMaxExceeded Error: {}", msg),
            SharedMemClientErr::TokenMinSubceeded(ref msg) => write!(f, "TokenMinSubceeded Error: {}", msg),
            SharedMemClientErr::SqrtPriceOutOfBounds(ref msg) => write!(f, "SqrtPriceOutOfBounds Error: {}", msg),
            SharedMemClientErr::NumberDownCastError(ref msg) => write!(f, "NumberDownCastError Error: {}", msg),
            SharedMemClientErr::NumberCastError(ref msg) => write!(f, "NumberCastError Error: {}", msg),
        }
    }
}

impl std::error::Error for SharedMemClientErr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            SharedMemClientErr::MathOverflowError(_) => None,
            SharedMemClientErr::InvalidBinId(_) => None,
            SharedMemClientErr::TypeCastFailed(_) => None,
            SharedMemClientErr::LiquidityOverflow(_) => None,
            SharedMemClientErr::LiquidityUnderflow(_) => None,
            SharedMemClientErr::DivideByZero(_) => None,
            SharedMemClientErr::MulDivOverflow(_) => None,
            SharedMemClientErr::MultiplicationOverflow(_) => None,
            SharedMemClientErr::LiquidityTooHigh(_) => None,
            SharedMemClientErr::MultiplicationShiftRightOverflow(_) => None,
            SharedMemClientErr::TokenMaxExceeded(_) => None,
            SharedMemClientErr::TokenMinSubceeded(_) => None,
            SharedMemClientErr::SqrtPriceOutOfBounds(_) => None,
            SharedMemClientErr::NumberDownCastError(_) => None,
            SharedMemClientErr::NumberCastError(_) => None,
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct TickInfo {
    pub pool: String,
    pub pool_mint_a: String,
    pub pool_mint_b: String,
    pub tick_spacing: i32,
    pub ticks: Vec<Tick>,
}

impl TickInfo {
    pub const RAYDIUM_TICK_ARRAY_SIZE: i32 = 60;
    pub const WHIRLPOOL_TICK_ARRAY_SIZE: i32 = 88;
    pub const CREMA_FINANCE_TICK_ARRAY_SIZE: i32 = 64;
    pub const CREMA_FINANCE_MIN_TICK_INDEX: i32 = -443636;
    pub const METEORA_DLMM_BIN_ARRAY_SIZE: i32 = 70;

    pub fn get_tick_arrays(
        &self,
        swap_type: &SwapType,
        active_tick_id: i32,
        a_to_b: bool,
    ) -> Vec<Pubkey> {
        match swap_type {
            SwapType::RaydiumConcentrated => self
                .get_raydium_tick_arrays(active_tick_id, a_to_b),
            SwapType::OrcaWhirlpool | SwapType::CropperWhirlpool => self
                .get_whirlpool_tick_arrays(active_tick_id, a_to_b),
            SwapType::InvariantSwap => self
                .get_invariant_tick_arrays(active_tick_id, a_to_b),
            SwapType::CremaFinance => self
                .get_crema_finance_tick_arrays(active_tick_id, a_to_b),
            SwapType::MeteoraDlmm => self
                .get_meteora_dlmm_bin_arrays(active_tick_id, a_to_b),
            _ => Vec::new(),
        }
    }

    pub fn get_raydium_tick_arrays(&self, active_tick_id: i32, a_to_b: bool) -> Vec<Pubkey> {
        let mut tick_arrays: Vec<Pubkey> = Vec::new();
        let mut tick_index =
            TickInfo::get_raydium_clmm_tick_start_index(active_tick_id, self.tick_spacing);

        let mut sorted_ticks = self.ticks.clone();
        if sorted_ticks.is_empty() {
            return Vec::new();
        }
        if a_to_b {
            sorted_ticks.sort_by(|a, b| b.index.cmp(&a.index));
            let mut i = 0;
            while i < sorted_ticks.len() && sorted_ticks[i].index > tick_index {
                i += 1;
            }

            let end_index = if i + 3 > sorted_ticks.len() {
                sorted_ticks.len()
            } else {
                i + 3
            };
            tick_arrays = sorted_ticks[i..end_index]
                .iter()
                .map(|t| Pubkey::from_str(&t.address).unwrap())
                .collect();
        } else {
            sorted_ticks.sort_by(|a, b| a.index.cmp(&b.index));
            let mut i = 0;
            while i < sorted_ticks.len() && sorted_ticks[i].index < tick_index {
                i += 1;
            }

            let end_index = if i + 3 > sorted_ticks.len() {
                sorted_ticks.len()
            } else {
                i + 3
            };
            tick_arrays = sorted_ticks[i..end_index]
                .iter()
                .map(|t| Pubkey::from_str(&t.address).unwrap())
                .collect();
        }
        let (tick_array_bitmap, _) = Pubkey::find_program_address(
            &[
                "pool_tick_array_bitmap_extension".as_bytes(),
                Pubkey::from_str(&self.pool).unwrap().as_ref(),
            ],
            &Pubkey::from_str("CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK").unwrap(), // raydium clmm program id
        );
        tick_arrays.insert(0, tick_array_bitmap);
        tick_arrays
    }

    pub fn get_whirlpool_tick_arrays(&self, active_tick_id: i32, a_to_b: bool) -> Vec<Pubkey> {
        let mut tick_arrays: Vec<Pubkey> = Vec::new();
        let tick_index =
            TickInfo::get_whirlpool_tick_start_index(active_tick_id, self.tick_spacing);
        let mut sorted_ticks = self.ticks.clone();
        if sorted_ticks.is_empty() {
            return Vec::new();
        }
        if a_to_b {
            sorted_ticks.sort_by(|a, b| b.index.cmp(&a.index));
            let mut i = 0;
            while i < sorted_ticks.len() && sorted_ticks[i].index > tick_index {
                i += 1;
            }

            let end_index = if i + 3 > sorted_ticks.len() {
                sorted_ticks.len()
            } else {
                i + 3
            };
            tick_arrays = sorted_ticks[i..end_index]
                .iter()
                .map(|t| Pubkey::from_str(&t.address).unwrap())
                .collect();
        } else {
            sorted_ticks.sort_by(|a, b| a.index.cmp(&b.index));
            let mut i = 0;
            while i < sorted_ticks.len() && sorted_ticks[i].index < tick_index {
                i += 1;
            }

            let end_index = if i + 3 > sorted_ticks.len() {
                sorted_ticks.len()
            } else {
                i + 3
            };
            tick_arrays = sorted_ticks[i..end_index]
                .iter()
                .map(|t| Pubkey::from_str(&t.address).unwrap())
                .collect();
        }

        if tick_arrays.len() == 1 {
            tick_arrays.push(tick_arrays[0]);
            tick_arrays.push(tick_arrays[0]);
        } else if tick_arrays.len() == 2 {
            tick_arrays.push(tick_arrays[1])
        }
        
        tick_arrays
    }

    pub fn get_invariant_tick_arrays(&self, active_tick_id: i32, a_to_b: bool) -> Vec<Pubkey> {
        let mut _tick_arrays: Vec<Pubkey> = Vec::new();
        let mut sorted_ticks = self.ticks.clone();
        if sorted_ticks.is_empty() {
            return Vec::new();
        }

        if a_to_b {
            sorted_ticks.sort_by(|a, b| b.index.cmp(&a.index));
            let mut i = 0;
            while i < sorted_ticks.len() && sorted_ticks[i].index > active_tick_id {
                i += 1;
            }

            let end_index = if i + 3 > sorted_ticks.len() {
                sorted_ticks.len()
            } else {
                i + 3
            };
            _tick_arrays = sorted_ticks[i..end_index]
                .iter()
                .map(|t| Pubkey::from_str(&t.address).unwrap())
                .collect();
        } else {
            sorted_ticks.sort_by(|a, b| a.index.cmp(&b.index));
            let mut i = 0;
            while i < sorted_ticks.len() && sorted_ticks[i].index < active_tick_id {
                i += 1;
            }

            let end_index = if i + 3 > sorted_ticks.len() {
                sorted_ticks.len()
            } else {
                i + 3
            };
            _tick_arrays = sorted_ticks[i..end_index]
                .iter()
                .map(|t| Pubkey::from_str(&t.address).unwrap())
                .collect();
        }

        _tick_arrays
    }

    pub fn get_crema_finance_tick_arrays(&self, active_tick_id: i32, a_to_b: bool) -> Vec<Pubkey> {
        let mut _tick_arrays: Vec<Pubkey> = Vec::new();
        let mut sorted_ticks = self.ticks.clone();
        if sorted_ticks.is_empty() {
            return Vec::new();
        }
        let tick_index =
            TickInfo::get_crema_finance_tick_start_index(active_tick_id, self.tick_spacing);

        if a_to_b {
            sorted_ticks.sort_by(|a, b| b.index.cmp(&a.index));
            let mut i = 0;
            while i < sorted_ticks.len() && sorted_ticks[i].index > tick_index {
                i += 1;
            }

            let end_index = if i + 3 > sorted_ticks.len() {
                sorted_ticks.len()
            } else {
                i + 3
            };
            _tick_arrays = sorted_ticks[i..end_index]
                .iter()
                .map(|t| Pubkey::from_str(&t.address).unwrap())
                .collect();
        } else {
            sorted_ticks.sort_by(|a, b| a.index.cmp(&b.index));
            let mut i = 0;
            while i < sorted_ticks.len() && sorted_ticks[i].index < tick_index {
                i += 1;
            }

            let end_index = if i + 3 > sorted_ticks.len() {
                sorted_ticks.len()
            } else {
                i + 3
            };
            _tick_arrays = sorted_ticks[i..end_index]
                .iter()
                .map(|t| Pubkey::from_str(&t.address).unwrap())
                .collect();
        }

        _tick_arrays
    }

    pub fn get_meteora_dlmm_bin_arrays(&self, active_tick_id: i32, a_to_b: bool) -> Vec<Pubkey> {
        // let mut log_msg = self.pool.clone();
        // log_msg.push_str(&(" active_id:".to_owned() + active_tick_id.to_string().as_str()));

        let cur_bin_array_index = TickInfo::get_meteora_dlmm_bin_array_index(active_tick_id);
        // log_msg.push_str(&(" cur_bin_array_index:".to_owned() + cur_bin_array_index.to_string().as_str()));
        let mut bin_arrays: Vec<Pubkey> = Vec::new();
        let mut sorted_bins = self.ticks.clone();
        if sorted_bins.is_empty() {
            // log_msg.push_str(" sorted_bins.is_empty");
            // println!("{}", log_msg);
            return Vec::new();
        }

        if a_to_b {
            sorted_bins.sort_by(|a, b| b.index.cmp(&a.index));
            
            let mut i = 0;
            while i < sorted_bins.len() && sorted_bins[i].index > cur_bin_array_index {
                i += 1;
            }
            if i < sorted_bins.len() {
                let end_index = if i + 3 > sorted_bins.len() {
                    sorted_bins.len()
                } else {
                    i + 3
                };
                bin_arrays = sorted_bins[i..end_index]
                    .iter()
                    .map(|t| {
                        // println!("atob:{}, bin_array {}, {}",a_to_b, t.index, t.address);
                        Pubkey::from_str(&t.address).unwrap()
                    }
                    )
                    .collect();
            }
        } else {
            sorted_bins.sort_by(|a, b| a.index.cmp(&b.index));
            // println!("sorted bins {:#?}", sorted_bins);
            let mut i = 0;
            while i < sorted_bins.len() && sorted_bins[i].index < cur_bin_array_index {
                i += 1;
            }
            if i < sorted_bins.len() {
                let end_index = if i + 3 > sorted_bins.len() {
                    sorted_bins.len()
                } else {
                    i + 3
                };
                bin_arrays = sorted_bins[i..end_index]
                    .iter()
                    .map(|t| {
                        // println!("atob:{}, bin_array {}, {}",a_to_b, t.index, t.address);
                        Pubkey::from_str(&t.address).unwrap()
                    })
                    .collect();
            }
        }
        // println!("dlmm bin_arrays: {:#?}", bin_arrays);
        bin_arrays
    }

    pub fn get_raydium_clmm_tick_start_index(tick_index: i32, tick_spacing: i32) -> i32 {
        if tick_spacing > 0 {
            let ticks_in_array = tick_spacing * TickInfo::RAYDIUM_TICK_ARRAY_SIZE;
            let mut start = tick_index / ticks_in_array;
            if tick_index < 0 && tick_index % ticks_in_array != 0 {
                start = start - 1
            }
            start * ticks_in_array
        }
        else {
            tick_index
        }
        
    }

    pub fn get_whirlpool_tick_start_index(tick_index: i32, tick_spacing: i32) -> i32 {
        let ticks_in_array = tick_spacing * TickInfo::WHIRLPOOL_TICK_ARRAY_SIZE;
        let mut start = tick_index / ticks_in_array;
        if tick_index < 0 && tick_index % ticks_in_array != 0 {
            start = start - 1
        }
        start * ticks_in_array
    }

    pub fn get_crema_finance_tick_start_index(tick_index: i32, tick_spacing: i32) -> i32 {
        let min_tick_index = TickInfo::CREMA_FINANCE_MIN_TICK_INDEX
            + i32::abs(TickInfo::CREMA_FINANCE_MIN_TICK_INDEX) % tick_spacing;
        
        let array_spacing = tick_spacing * TickInfo::CREMA_FINANCE_TICK_ARRAY_SIZE;
        min_tick_index + (((tick_index - min_tick_index) / array_spacing ) * array_spacing )
    }

    pub fn get_meteora_dlmm_bin_array_index(bin_id: i32) -> i32 {
        if bin_id >= 0 {
            bin_id / TickInfo::METEORA_DLMM_BIN_ARRAY_SIZE
        }
        else {
            bin_id / TickInfo::METEORA_DLMM_BIN_ARRAY_SIZE - 1
        }
        
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Tick {
    pub address: String,
    pub index: i32,
}

#[derive(Debug)]
pub struct LbPairInfo {
    pub protocol_share: u16,
    pub volatility_accumulator: u32, 
    pub variable_fee_control: u32, 
    pub base_factor: u16, 
    pub bin_step: u16,
    pub active_id: i32
}
#[derive(Debug)]
pub struct PriceInfo {
    pub price: u128,
    pub actual_amount_in: u64,
    pub actual_amount_out: u64,
    pub fee: u64,
    pub protocol_fee: u64,
    pub bin_id: i32
}

#[derive(Debug)]
pub struct BinInfo {
    pub amount_x: u64,
    pub amount_y: u64,
    pub price: u128,
}

#[derive(Debug)]
pub struct SwapResult {
    /// Amount of token swap into the bin
    pub amount_in_with_fees: u64,
    /// Amount of token swap out from the bin
    pub amount_out: u64,
    /// Swap fee, includes protocol fee
    pub fee: u64,
    /// Part of fee
    pub protocol_fee: u64,
    /// new amount x
    pub new_amount_x: u64,
    /// new amount y
    pub new_amount_y: u64
}