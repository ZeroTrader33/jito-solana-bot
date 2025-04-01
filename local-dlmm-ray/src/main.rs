#![allow(non_snake_case)]
// #![allow(dead_code)]
use core as core_;

// pub mod hash;
// pub mod jup_perps;
pub mod big_num;
pub mod full_math;

use std::{
    ffi::OsStr, fs, thread::{self, sleep}, time::Instant, usize
};

use {
    borsh::{BorshDeserialize, BorshSerialize},
    futures::stream::{FuturesUnordered, StreamExt},
    serde::{Deserialize, Serialize},
    solana_client::rpc_client::RpcClient,
    solana_faucet::faucet::run_local_faucet,
    solana_measure::{measure::Measure, measure_us},
    solana_program::instruction::{AccountMeta, Instruction},
    solana_runtime::{
        bank::{Bank, TransactionSimulationResult},
        bank_forks::BankForks,
        commitment::BlockCommitmentCache,
    },
    solana_sdk::{
        account::{Account, AccountSharedData, ReadableAccount},
        address_lookup_table::AddressLookupTableAccount,
        compute_budget::ComputeBudgetInstruction,
        fee_calculator::FeeRateGovernor,
        lamports,
        message::{AddressLoader, VersionedMessage::V0},
        program_pack::Pack,
        pubkey::Pubkey,
        rent::Rent,
        signature::{self, Keypair},
        signer::Signer,
        transaction::{MessageHash, SanitizedTransaction, VersionedTransaction},
    },
    solana_streamer::socket::SocketAddrSpace,
    solana_test_validator::{TestValidator, TestValidatorGenesis, UpgradeableProgramInfo},
    std::{
        borrow::BorrowMut,
        collections::{HashMap, HashSet},
        fmt,
        fs::{File, OpenOptions},
        io::{Read, Write},
        ops::{Div, Mul},
        path::PathBuf,
        str::FromStr,
        sync::{
            atomic::{AtomicPtr, AtomicUsize, Ordering},
            Arc,
        },
        u16, u64,
    },
    tokio::task::spawn_blocking,
};

use arrayref::{array_ref, array_refs};
use big_num::U128;
use full_math::MulDiv;
use num::Integer;
use solana_account_decoder::{UiAccountEncoding, UiDataSliceConfig};

use solana_client::{
    rpc_config::{
        RpcAccountInfoConfig, RpcProgramAccountsConfig, RpcSendTransactionConfig, RpcSimulateTransactionAccountsConfig, RpcSimulateTransactionConfig, RpcSimulateTransactionTokenAmountsConfig
    },
    rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType}, rpc_response::{RpcResult, RpcSimulateTransactionResult},
};
use solana_runtime::bank::{self, LoadAndExecuteTransactionsOutput};
use solana_runtime_transaction::runtime_transaction::RuntimeTransaction;
use solana_sdk::{
    account::WritableAccount, account_info::AccountInfo, address_lookup_table::state::AddressLookupTable, bpf_loader_upgradeable::UpgradeableLoaderState, commitment_config::CommitmentConfig, epoch_schedule::EpochSchedule, message::AccountKeys, program_option::COption, sysvar::{clock, Sysvar}, transaction::Transaction
};

// pub const RPC_ENDPOINT: &str = "http://127.0.0.1:8899";
pub const WSS_ENDPOINT: &str = "ws://127.0.0.1:8900";
pub const RPC_ENDPOINT: &str = "https://mainnet.helius-rpc.com/?api-key=4182c684-2ade-4428-8349-8c060c6d36ac";
pub const EDGES_FILE_PATH: &str = "/mnt/edges-solfi.json";
pub const POOL_OFFSET_AFTER_BIN: usize = 8 + 4 * 8 + 4 * 8 + 4 + 4 + 2;
pub const POOL_OFFSET_UNTIL_BIN: usize = 8 + 4 * 8 + 4 * 8 + 4;
pub const ORACLE_OFFSET: usize = 464;

const BASE_FACTOR_OFFSET: usize = 8;
const PROTOCOL_SHARE_OFFSET: usize = 8 + 2 * 4 + 4 * 4;
const VOLATILITY_ACCUMULATOR_OFFSET: usize = 8 + 4 * 8;
const VARIABLE_FEE_CONTROL_OFFSET: usize = 8 + 2 * 4;


pub const MAX_BIN_PER_ARRAY: usize = 70;
pub const PRICE_MULTIPLIER: u64 = 1_000_000_000;

pub const Q64: u128 = (u64::MAX as u128) + 1; // 2^64
pub const BIN_ARRAY: &[u8] = b"bin_array";

pub const DLMM_FEE_PRECISION: u64 = 1_000_000_000;
/// Maximum fee rate. 10%
pub const MAX_FEE_RATE: u64 = 100_000_000;

#[derive(Debug, Clone, Copy)]
pub struct SwapCase {
    pub index: u8,
    pub amount_in: u64,
    pub atob: bool
}
pub const MAX_SWAPS: usize = 30;
pub const SWAP_SIZE_PER_TX: usize = 8;
pub const BUNDLE_SIZE: usize = 4;

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
pub const DLMM_POOLS: [[&str; 6]; 30] = [
    [
      "5rCf1DM8LjKTw4YqhnoLcngyZYeNnQqztScTogYHAS6",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "EYj9xKw6ZszwpyNibHY7JD5o3QgTVrSdcBp1fMJhrR9o",
      "CoaxzEh8p5YyGLcj36Eo3cUThVJxeKCs7qvLAGDYwBcz"
    ],
    [
      "BGm1tav58oGcsQJehL9WXBFXF7D27vZsKefj4xJKD5Y",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "DwZz4S1Z1LBXomzmncQRVKCYhjCqSAMQ6RPKbUAadr7H",
      "4N22J4vW2juHocTntJNmXywSonYjkndCwahjZ2cYLDgb"
    ],
    [
      "BVRbyLjjfSBcoyiYFuxbgKYnWuiFaF9CSXEa5vdSZ9Hh",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "FMzVsENjscefpAtUJYBUTeJAYaKNfFQBHjTZE1AQRFYY",
      "7du3jFJK4rhf9JnZSQmhr6qPkgdQyJ88528qyxpYPPtL"
    ],
    [
      "CgqwPLSFfht89pF5RSKGUUMFj5zRxoUt4861w2SkXaqY",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "4jTcZiooRV5Z5sb29xroJEu1FnPJbadmZcnEbXFsUuXi",
      "AsAhzfi3u9gihEYzV3zq6VnkfvQaM7yF13wnsTfAk936"
    ],
    [
      "5XRqv7LCoC5FhWKk5JN8n4kCrJs3e4KH1XsYzKeMd5Nt",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "EN1RTvqZ3BpLmpJVXqpMb6Sc2w8ncbA5imsTQmQtRCZg",
      "BsLY7Qxh8NM61MDj6DK1UWdSprJfTEBPnp6Lc9iw2Gmw"
    ],
    [
      "sZxb9vrxJBpFiJBogovhfkYqfapVzveLEU4TmzWv4GN",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "CN8k1PtzJz2mGGdf5puwV14Dh1skMRkT42sSvhesf3nT",
      "4FkX872Wbo6NK7eNEmMnDRMJnwbu6tQsE6utb5fbDbzv"
    ],
    [
      "Cu8Zg65pw6anL3khVnwvjR9drchnszTdkwE3gtw1uHhU",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "CHPY3W3pDzPH3vb9E2Y1BvcNCGG1otrvmSUya1eY9YvL",
      "2fekvNccKSRYcjKrf398kPSiJfqzrbsWi2JFGQz3wkPf"
    ],
    [
      "2sf5NYcY4zUPXUSmG6f66mskb24t5F8S11pC1Nz5nQT3",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "4CwYbt9a8LdFB32BpcxME88bcDMoxcK7ubGCLVBYbJ1N",
      "3abHCpu2sMD4A9N7NabgF7FbXQNEFMgjpa8G2XBKgrzs"
    ],
    [
      "HTvjzsfX3yU6BUodCjZ5vZkUrAxMDTrBs3CJaq43ashR",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "H7j5NPopj3tQvDg4N8CxwtYciTn3e8AEV6wSVrxpyDUc",
      "HbYjRzx7teCxqW3unpXBEcNHhfVZvW2vW9MQ99TkizWt"
    ],
    [
      "9oCwYQJYCWHeXnodFCxdqY1kV4uBPo5P7yWGkNJ2Wjj3",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "G9HhRByXgHzA3hQUy5LARE8xxk13gQyYzgFToLqR6zG5",
      "2vRxenZynWMyV7UFLLw1zvjEocqq1bTneaYDQi4KK7xx"
    ],
    [
      "5BKxfWMbmYBAEWvyPZS9esPducUba9GqyMjtLCfbaqyF",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "2SE8APEF3E9o172RQ7AH2tsYLa4ymruQhBWnKkcpaHcH",
      "BqQwcLb8h6tjfWdkQvq3vGjnS8sM3wgsmnErMESrUmk9"
    ],
    [
      "FbkX1h2YTs171cEMa4GrV7XbAiQt5zSmV2CjfYWxXJDP",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "C1SWdxKJA8GZZ9xgGYEcyNrdw47afu8wB9KynimvkJAn",
      "E5mt7S6oncRjuHtRxwgEdiv3AQ5aAMu1RThUm25yyNBm"
    ],
    [
      "8gJ7UWboMeQ6z6AQwFP3cAZwSYG8udVS2UesyCbH79r7",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "chM5ZB1uPZxvJJAK4D1Z4KHAYjWKvwuQTy6fFAeWQ1T",
      "FGFaiYjXTVuLsKvzn6ueckraNTeqUGHeYqrQPQCpd7kH"
    ],
    [
      "FoSDw2L5DmTuQTFe55gWPDXf88euaxAEKFre74CnvQbX",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "Cvf6b3eKjZbYMzwQyFsLeMjx9Hpo7itEJ6tvrnPRmGLn",
      "GEarArxgKL19JxihPHmsSQdWajZWqyitsgf7oUht61Nb"
    ],
    [
      "81MPQqJY58rgT83sy99MkRHs2g3dyy6uWKHD24twV62F",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "7ixaquirw9k3VNkxJ5zpbx9GTAbAvUrrNeZovw7TBqyu",
      "2JwKVhEeJZn8bHsHVK1rFuATHni3wDft3UFAsrKKMKfP"
    ],
    [
      "4YVLUZGEhsjfsWuxRbo6h18vL297HYRHTrLVE8bwpyCW",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "5Ys4iNr3MVhXYdtoHtCjcYvMq34MjnkFynaxNihy71M4",
      "2GHtKmEEEX2vwqD3btyNUUibhE3DvowojCpLH178t7Pk"
    ],
    [
      "627kqiAtYNE4FFKtUcnR9nmDqqCZnQue2ALYRQtnziLR",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "GptzjzMFdtKZBXQSsM4cfxDHzdyZYvKo4A9unjNZS7e2",
      "71Hjqj7G11361GquQ4oqwnpuUeQzDynzo3gRC86WAa5w"
    ],
    [
      "DGSu6KJrHrXMXyXCzGMqanu8g7EuSyniDw3mPR3B9BGg",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "3GK495pN2osAdPXE337oLKHsVrYd1bvAFURAh41yk3To",
      "FWHZxG274ahvsRYgpf82wfXHuzC8CxXTEhAqCviybH4h"
    ],
    [
      "EdEU6L2yLbXC78fdpruEFWubK7AQUGSjTNummi5zJRyY",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "BcPUJ5DVy8rdm5jcX9bcQXtx36s7dhcz75en6Cdi8qRU",
      "AbYGijTSxS2YsRbrWT9GzrfGNhM7AWH3qcLjBeYxdtQW"
    ],
    [
      "1jw5fDodwGEGBVqNXsx2eqiLgNmgMDEeXWSbrTreLCM",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "GLFpCS3jPrQ2y2yCyQWb4Uiz3aEeDSQVqQcaM4rhGxUa",
      "DyDE7RLGZStDSMxBVp4RMRnNWmN5LNLTwXWajCKGpURx"
    ],
    [
      "9fjLVCpbHRHtGNyVGiuiq2jitSCc3vzyScAdYRtsRUe6",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "J2n9gnZrBSSEQRozEK2w74k42kTi19JMVo5cwX5vwSLY",
      "E13QJrZyy9Rgfvdf3pooXjSkBuKH9e4jZ3P5ZAWGhCX"
    ],
    [
      "3msVd34R5KxonDzyNSV5nT19UtUeJ2RF1NaQhvVPNLxL",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "8kR2HTHzPtTJuzpFZ8jtGCQ9TpahPaWbZfTNRs2GJdxq",
      "EeThDNkUuNhJFHYqR3yTB6wzcj1hrubgVQuvSSGjNt4W"
    ],
    [
      "6YuRXMMF4W8zo216CP1A7iiE627GdDaitCRS2iXEdM2q",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "Do86A4P6J2j4BFQHXcVd8cH5pbVZoeXhKncs5AtaNfuc",
      "yQaQddrNTyHcwT2yQrnhXMe25qjcneATWeEtpE1mTTT"
    ],
    [
      "5BoHD7DAHsGCA9D8eUkkZTgWTrnpad6G18MnTq3cUS86",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "7m67zNLQ7FtrUetLgD9DxaACVmwtA8rX16PfG4CKJjBg",
      "JDgcfNtP5ivGRdWdyoiPxnSGCCu6U9FjKNKARjcS6R1Y"
    ],
    [
      "En2cZdXy71xDqqYRmkA6NdZpSZvVPgsomAspL6Nz7wvB",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "dY5cups8EYbDSrNL8EpJuPiXX2rbDNRuJXaZatSpW5N",
      "CQxdPWr2Xbe1uq5sFDoTyBAXujZexhEPuNxpYSyPNQwm"
    ],
    [
      "2L47gbUzGsnJZqRzfny4b3VWvKBnMRTMeNH3uzFrj6ko",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "RBNV3esFPQ4jAkzW9hzxtZj34vJ9Snzrm6arXLs1mYZ",
      "5E84baKQQCFrBRVc9aViHQ49ZTmfQPmGWqXfPY9v8ZNt"
    ],
    [
      "8M5rjeDQKW4w4rmWFQLTqCYVuA1rMe9Z2QQ2SZResD9M",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "2LizoD7noGRmTFszBRj3P2GzfARqdGwhpimQthWTGbhn",
      "DHqUM3nbZtvuv7tm4rcAbbWSMX99hnPQvEmDQ2EyGfg3"
    ],
    [
      "8M6hydDhfZJyaVcsstFUoGD4qae9VUdPrDCr1WXJM9hj",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "HMGvcxSSvVLzyEYBhcLi1NmiUmCoMKwpRYehNpW2BVqn",
      "ALkE7UJJ2FUYDj7P4ENExcUmA91vRLzPBdeZg4RHM2xw"
    ],
    [
      "DJJzSUx5gEqDJhTyE8tvZtkkHG7R7v6EoyZXN32aAsXF",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "7b1HaBcaVt8ZjUqNdoDMrqHWugtj1h9jzuvCY74xeojy",
      "2zWBCsqELMDsny3ox89AsM4wEqvbcH2y33NtwaHVyERn"
    ],
    [
      "9Q1njS4j8svdjCnGd2xJn7RAkqrJ2vqjaPs3sXRZ6UR7",
      "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
      "So11111111111111111111111111111111111111112",
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "C3616CMpgcLv6KXbF2WyDKjL46x9m1PTWqHbE2eUhcsv",
      "2ph6cAuB9grBQT4mntSJB3JeBcHzCUX1mx4wBon1nnuN"
    ]
];
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
    cu_consumed_b_a: u64,
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
    // pub fn get_native_ix(&self, atob: bool) -> Instruction {

    // }
}
pub fn main() {
    // Runner::print_fetchable_accounts();
    // Runner::analyze_dlmm_bin();
    // Runner::parse_dlmm_pools_bins();
    Runner::simulate_dlmm_ray();
    
    // let runner = Runner::setup_validator();
    // runner.simulate_edges();
}

pub struct Runner {
    pub validator: AtomicPtr<TestValidator>,
    pub payer: Keypair,
    pub edges: AtomicPtr<HashMap<String, JupSwapEdge>>,
    pub slot: u64
}

impl Runner {
    pub fn simulate_edges(&self) {
        let bot_edges = unsafe { &mut *self.edges.load(Ordering::SeqCst) };

        for (pool, edge) in bot_edges.iter_mut() {
            let mut inout_table: Vec<(u64, u64)> = Vec::new();
            for value in 1..2u64 {
                let input_amount = edge.price_a * value; // 1$ worth of amount
                let out_amount = self.simulate_edge(edge, input_amount);
                inout_table.push((input_amount, out_amount));
            }

        }
    }
    pub fn simulate_edge(&self, edge: &mut JupSwapEdge, input_amount: u64) -> u64 {
        let atob = true;
        edge.update_amount(input_amount, atob);

        self.update_token_amount(&Pubkey::from_str(&edge.ata_a).unwrap(), input_amount);
        self.update_token_amount(&Pubkey::from_str(&edge.ata_b).unwrap(), 0);
        // println!("input {:#?}",input_amount.to_le_bytes());

        let ix = edge.swap_ix_a_to_b.clone();
        let bank = self.bank().clone();

        // run jup ix
        // let sim_res = self.create_and_simulate_ix(&[ix.clone()], &[], &bank, true);
        // if sim_res.result.is_err() {
        //     // println!("error {:#?}", sim_res.result.err());
        //     println!("error {:#?}", sim_res.logs);
        //     return 0;
        // }
        // else {
        //     println!("inner_instructions {:#?}", sim_res.inner_instructions);
        //     println!("logs {:#?}", sim_res.logs);
        // }



        // get solfi ix
        let jup_tx = self.create_ix(&[ix.clone()], &[], &bank);
        let input_bytes = input_amount.to_le_bytes();
        let solfi_pid_idx = 7;
        let solfi_accounts_idx = [0, 3, 4, 1, 5, 2, 9, 8];
        let solfi_ix_data = [
            7, 
            input_bytes[0], 
            input_bytes[1], 
            input_bytes[2], 
            input_bytes[3], 
            input_bytes[4], 
            input_bytes[5], 
            input_bytes[6], 
            input_bytes[7], 
            0,0,0,0,0,0,0,0, 
            if atob {0} else {1}
        ];
        let mut hashmap_jup_accounts: HashMap<Pubkey, AccountMeta> = HashMap::new();
        for account_meta in ix.accounts.iter() {
            hashmap_jup_accounts.insert(account_meta.pubkey.clone(), account_meta.clone());
        }

        let jup_static_keys = jup_tx.message.static_account_keys();

        let solfi_pid = jup_static_keys[solfi_pid_idx].clone();
        let mut solfi_accounts: Vec<AccountMeta> = Vec::new();
        for idx in solfi_accounts_idx {
            solfi_accounts.push(hashmap_jup_accounts.get(&jup_static_keys[idx]).expect("getting account meta error").clone());
        }

        let solfi_ix = Instruction::new_with_bytes(
            solfi_pid, 
            &solfi_ix_data, 
            solfi_accounts
        );
        // run jup ix
        let sim_res = self.create_and_simulate_ix(&[solfi_ix.clone()], &[], &bank, false);
        if sim_res.result.is_err() {
            println!("error {:#?}", sim_res.result.err());
            println!("logs {:#?}", sim_res.logs);
            return 0;
        }
        else {
            println!("logs {:#?}", sim_res.logs);
        }
        


        // let validator = unsafe { &*self.validator.load(Ordering::SeqCst) };
        // let test_rpc_client = RpcClient::new_with_commitment(
        //     String::from(validator.rpc_url()),
        //     CommitmentConfig::processed(),
        // );
        // let sim_res = self.create_and_simulate_ix_rpc(&[ix.clone()], &[], &test_rpc_client, true).expect("simulate error");
        // if sim_res.value.err.is_some() {
        //     println!("error {:#?}", sim_res.value.err);
        //     return 0;
        // }
        // else {
        //     println!("inner_instructions {:#?}", sim_res.value.inner_instructions);
        //     println!("logs {:#?}", sim_res.value.logs);
        // }
        0
    }
    pub fn update_token_amount(&self, key: &Pubkey, amount: u64) {
        let bank = self.bank();
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
        bank.store_account(key, &shared_data);
    }
    pub fn get_amount_from_token_account(token_account_data: &[u8]) -> u64 {
        let input = array_ref![token_account_data, 0, 72];
        let (_, amount_slice) = array_refs![input, 64, 8];

        let vault_amount = u64::from_le_bytes(*amount_slice);

        vault_amount
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
        let rpc_client = RpcClient::new_with_commitment(
            String::from(RPC_ENDPOINT),
            CommitmentConfig::confirmed(),
        );
        let blockhash = rpc_client.get_latest_blockhash().unwrap();

        let versioned_message = V0(solana_sdk::message::v0::Message::try_compile(
            &self.payer.pubkey(),
            tx_instructions,
            lookuptables,
            blockhash,
        )
        .unwrap());

        let tx = solana_sdk::transaction::VersionedTransaction::try_new(versioned_message, &signers)
            .unwrap();
        
        rpc_client.simulate_transaction_with_config(
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
    pub fn print_fetchable_accounts() {
        let edges_file_name = EDGES_FILE_PATH.to_string();
        let edges_data = fs::read_to_string(edges_file_name).expect("Failed to read JSON file");
        let edges: Vec<JupSwapEdge> =
            serde_json::from_str(&edges_data).expect("Failed to parse JSON");
        let mut bot_edges: HashMap<String, JupSwapEdge> = HashMap::new();
        for edge in edges.iter() {
            bot_edges.insert(edge.pool.clone(), edge.clone());
        }
        println!("{} edges loaded!", bot_edges.len());

        let rpc_client = RpcClient::new_with_commitment(
            String::from(RPC_ENDPOINT),
            CommitmentConfig::confirmed(),
        );

        let mut accounts: Vec<String> = Vec::new();
        let mut shared_datas: Vec<(Pubkey, AccountSharedData)> = Vec::new();
        let mut programs: Vec<String> = Vec::new();
        for (_pool, edge) in bot_edges.iter() {
            for account in edge.swap_ix_a_to_b.accounts.iter() {
                if !accounts.contains(&account.pubkey.to_string()) {
                    // check if it is program
                    let account_res = rpc_client.get_account(&account.pubkey);
                    if account_res.is_ok() {
                        let account_data = account_res.unwrap();
                        if account_data.executable {
                            if account.pubkey.to_string().ne("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA") {
                                programs.push(account.pubkey.to_string());
                                // let p_data = AccountSharedData::from(account_data);
                                // if let Ok(UpgradeableLoaderState::Program {
                                //     programdata_address,
                                // }) = p_data.deserialize_data()
                                // {
                                //     programs.push(programdata_address.to_string());
                                // }
                            }
                            
                        } else {
                            accounts.push(account.pubkey.to_string());
                            shared_datas
                                .push((account.pubkey, AccountSharedData::from(account_data)));
                        }
                    } else {
                        // println!("getting account error {:#?}", account_res.err());
                    }
                }
            }
        }

        println!(
            "accounts len: {}, programs len: {}",
            accounts.len(),
            programs.len()
        );


        for account_address in accounts.iter() {
            println!("solana account -u m --output json-compact --output-file /mnt/local-accounts-solfi/account/{}.json {}", account_address, account_address);
        }
        for account_address in programs.iter() {
            println!("solana program dump {} /mnt/local-accounts-solfi/program/{}.so", account_address, account_address);
        }
    }
    pub fn bin_id_to_bin_array_index(bin_id: i32) -> i32 {
        let (idx, rem) = bin_id.div_rem(&(MAX_BIN_PER_ARRAY as i32));

        if bin_id.is_negative() && rem != 0 {
            idx - 1
        } else {
            idx
        }
    }
    pub fn simulate_dlmm_ray() {

    }


    pub fn calculate_dlmm_ray_amount(raydium_pool: &[u8], ray_coin_vault: &[u8], ray_pc_vault: &[u8], dlmm_pool: &[u8], dlmm_bin_arrays: &Vec<Vec<u8>>, max_amount: u64) -> Option<Vec<SwapCase>> {
        let coin_vault_amount = Self::get_amount_from_token_account(ray_coin_vault);
        let pc_vault_amount = Self::get_amount_from_token_account(ray_pc_vault);
        let cur_ray_price = Self::get_ray_price(coin_vault_amount, pc_vault_amount);

        let max_pos = dlmm_bin_arrays.len() * 70;
        let mut bin_datas = Vec::new();
        let mut x_pos = usize::MAX;
        for (bin_array_idx, bin_array_data) in dlmm_bin_arrays.iter().enumerate() {
            for i in 0..70 {
                let (amount_x, amount_y, bin_price) = Self::get_bin_info(bin_array_data, i);
                if amount_x > 0 && x_pos == usize::MAX {
                    x_pos = bin_array_idx * 70 + i;
                };
                bin_datas.push((amount_x, amount_y, bin_price));
            }
        };
        let y_pos = if bin_datas[x_pos].1 == 0 {
            if x_pos > 0 {
                x_pos -1
            }
            else {
                return None;
            }
        }
        else {
            x_pos
        };
        let acc_price_dlmm_ray = bin_datas[x_pos].2
            .checked_mul(PRICE_MULTIPLIER as u128)
            .unwrap()
            .checked_div(cur_ray_price)
            .unwrap();
        if acc_price_dlmm_ray > PRICE_MULTIPLIER as u128 {
            for pos in x_pos..max_pos {

            }
        }
        else {
            let acc_price_ray_dlmm = cur_ray_price
            .checked_mul(PRICE_MULTIPLIER as u128)
            .unwrap()
            .checked_div(bin_datas[y_pos].2)
            .unwrap();
            if acc_price_ray_dlmm > PRICE_MULTIPLIER as u128 {
                
            }
        }
        None
    }
    pub fn get_ray_price(
        coin_vault_amount: u64,
        pc_vault_amount: u64
    ) -> u128 {
        if coin_vault_amount > 0 {
            (pc_vault_amount as u128)
            .checked_mul(PRICE_MULTIPLIER as u128)
            .unwrap()
            .checked_div(coin_vault_amount as u128)
            .unwrap()
        }
        else {
            // msg!("zero coin vault {:#?}", coin_vault_account_info.key());
            0u128
        }
    }






    pub fn parse_dlmm_pools_bins() {
        let rpc_client = RpcClient::new_with_commitment(
            String::from(RPC_ENDPOINT),
            CommitmentConfig::confirmed(),
        );

        let dlmm_pid = Pubkey::from_str("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo").unwrap();
        let mut bin_array_datas = Vec::new();
        for (pool_idx, pool_info) in DLMM_POOLS.iter().enumerate() {
            let pool_str = pool_info[0];
            let lb_pair = Pubkey::from_str(pool_str).unwrap();
            let pool_account = rpc_client.get_account(&lb_pair).unwrap();
            
            let pool_state_data = pool_account.data;
            let input = array_ref![pool_state_data, 0, POOL_OFFSET_AFTER_BIN];
            let (_, active_id_slice, bin_step_slice) = array_refs![input, POOL_OFFSET_UNTIL_BIN, 4, 2];

            let active_id = i32::from_le_bytes(*active_id_slice);
            let bin_step = u16::from_le_bytes(*bin_step_slice);
            let oracle_key_bytes = array_ref![pool_state_data, ORACLE_OFFSET, 32];
            let oracle_key = Pubkey::new_from_array(*oracle_key_bytes);

            let bin_array_index = Self::bin_id_to_bin_array_index(active_id) as i64;
            println!("bin array idx {} - {}, bin_step: {}", pool_idx, bin_array_index, bin_step);
            let bin_array_seeds = [
                BIN_ARRAY,
                lb_pair.as_ref(),
                &bin_array_index.to_le_bytes()
            ];
            let (bin_array_key, _) = Pubkey::find_program_address(&bin_array_seeds, &dlmm_pid);
            let bin_array_account = rpc_client.get_account(&bin_array_key).unwrap();
            bin_array_datas.push(bin_array_account.data);
        }

        
        let mut swaps: [[SwapCase; SWAP_SIZE_PER_TX]; BUNDLE_SIZE] = [[SwapCase {
            index: u8::MAX,
            amount_in: 0,
            atob: true
        }; SWAP_SIZE_PER_TX]; BUNDLE_SIZE];
        Self::calculate_swap_plan(&mut swaps, &bin_array_datas, 1000_000_000);

        for sub_swaps in swaps.iter() {
            for swap in sub_swaps {
                if swap.index < u8::MAX {
                    for i in 0..70 {
                        let (amount_x, amount_y, bin_price) = Self::get_bin_info(&bin_array_datas[swap.index as usize], i);
                        println!("{}=> amount_x: {}, amount_y: {}, bin_price: {}", swap.index, amount_x, amount_y, bin_price);
                    }
                    println!("swap {:#?}", swap);
                }
            }
        }
        // println!("swaps {:#?}", swaps);

    }
    
    pub fn calculate_swap_plan(swaps: &mut [[SwapCase; SWAP_SIZE_PER_TX]; BUNDLE_SIZE], bin_array_datas: &Vec<Vec<u8>>, max_amount: u64) {
        let mut bin_data_vec = Vec::new();
        for (bin_array_idx, data) in bin_array_datas.iter().enumerate() {
            let total_fee = Self::get_total_fee_from_pool(data);
            let mut bin_data = [(0u64, 0u64, 0u128); 70];
            let mut active_idx = usize::MAX;
            let mut active_price = 0;
            for i in 0..70 {
                let (amount_x, amount_y, bin_price) = Self::get_bin_info(data, i);
                bin_data[i].0 = amount_x;
                bin_data[i].1 = amount_y;
                bin_data[i].2 = bin_price;
                if amount_x > 0 && active_idx == usize::MAX {
                    active_idx = i;
                    active_price = bin_price;
                }
            }
            bin_data_vec.push((active_price, active_idx, bin_data, bin_array_idx, total_fee));
        }
        bin_data_vec.sort_by(|a, b| a.0.cmp(&b.0));
        
        let mut start_pos = 0;
        let mut end_pos = bin_data_vec.len() - 1;
        let mut total_amount = 0;

        loop {
            if bin_data_vec[start_pos].0 == 0 {
                start_pos += 1;
                continue;
            }
            if bin_data_vec[end_pos].0 == 0 {
                end_pos -= 1;
                continue;
            }
            if start_pos >= end_pos {
                break;
            }
            let high_bin_array = &bin_data_vec[end_pos];
            let high_amount = high_bin_array.2[high_bin_array.1].0;
            let high_price_x64 = high_bin_array.0;

            let low_bin_array = &bin_data_vec[start_pos];
            let low_amount = low_bin_array.2[low_bin_array.1].0;
            let low_price_x64 = low_bin_array.0;

            if high_price_x64 <= low_price_x64 {
                break;
            }
            if total_amount >= max_amount {
                break;
            }
            if high_amount == 0 {
                Self::advance_bin_data_vec(&mut bin_data_vec, end_pos, true);
                continue;
            }
            if low_amount == 0 {
                Self::advance_bin_data_vec(&mut bin_data_vec, start_pos, false);
                continue;
            }
            let low_amount_to_high = Self::bin_swap_out(&mut bin_data_vec, start_pos, true);
            if high_amount > low_amount_to_high {
                let swap_amount = if low_amount + total_amount <= max_amount {
                    low_amount
                } else {
                    max_amount - total_amount
                };
                let swap_amount_to_high = Self::bin_swap_out_with_amount(&mut bin_data_vec, start_pos, true, swap_amount);
                Self::advance_bin_data_vec_with_amount(&mut bin_data_vec, start_pos, true, swap_amount);
                Self::advance_bin_data_vec_with_amount(&mut bin_data_vec, end_pos, false, swap_amount_to_high);
                // register this swap
                Self::register_swap(swaps, &mut bin_data_vec, start_pos, true, swap_amount);
                Self::register_swap(swaps, &mut bin_data_vec, end_pos, false, swap_amount_to_high);

                total_amount += swap_amount;
            }
            else {
                let low_amount_expected = Self::bin_swap_exact_out(&mut bin_data_vec, start_pos, false, high_amount);
                let swap_amount = if low_amount_expected + total_amount <= max_amount {
                    low_amount_expected
                } else {
                    max_amount - total_amount
                };
                let swap_amount_to_high = Self::bin_swap_out_with_amount(&mut bin_data_vec, start_pos, true, swap_amount);
                Self::advance_bin_data_vec_with_amount(&mut bin_data_vec, start_pos, true, swap_amount);
                Self::advance_bin_data_vec_with_amount(&mut bin_data_vec, end_pos, false, swap_amount_to_high);
                // register this swap
                Self::register_swap(swaps, &mut bin_data_vec, start_pos, true, swap_amount);
                Self::register_swap(swaps, &mut bin_data_vec, end_pos, false, swap_amount_to_high);

                total_amount += swap_amount;
            }
        }
    }
    pub fn register_swap(swaps: &mut [[SwapCase; SWAP_SIZE_PER_TX]; BUNDLE_SIZE], bin_data_vec: &mut Vec<(u128, usize, [(u64, u64, u128); 70], usize, u128)>, pos: usize, to_high: bool, amount: u64) {
        let swap_index = bin_data_vec[pos].3;
        let bundle_idx = swap_index / SWAP_SIZE_PER_TX;
        let tx_idx = swap_index % SWAP_SIZE_PER_TX;
        swaps[bundle_idx][tx_idx].amount_in += amount;
        swaps[bundle_idx][tx_idx].atob = !to_high;
        swaps[bundle_idx][tx_idx].index = swap_index as u8;
    }
    pub fn advance_bin_data_vec(bin_data_vec: &mut Vec<(u128, usize, [(u64, u64, u128); 70], usize, u128)>, pos: usize, to_high: bool) {
        let bin_idx = bin_data_vec[pos].1;
        if bin_idx > 0 && to_high {
            let new_bin_idx = bin_idx - 1;
            let bin_price_x64 = bin_data_vec[pos].2[new_bin_idx].2;
            bin_data_vec[pos].1 = new_bin_idx;
            bin_data_vec[pos].0 = Self::get_price_from_x64(bin_price_x64);
        }
        else if bin_idx < 69 && !to_high {
            let new_bin_idx = bin_idx + 1;
            let bin_price_x64 = bin_data_vec[pos].2[new_bin_idx].2;
            bin_data_vec[pos].1 = new_bin_idx;
            bin_data_vec[pos].0 = Self::get_price_from_x64(bin_price_x64);
        }
        else {
            bin_data_vec[pos].2[bin_idx].0 = 0;
            bin_data_vec[pos].2[bin_idx].1 = 0;
        }
    }
    pub fn advance_bin_data_vec_with_amount(bin_data_vec: &mut Vec<(u128, usize, [(u64, u64, u128); 70], usize, u128)>, pos: usize, to_high: bool, amount: u64) {
        let bin_idx = bin_data_vec[pos].1;
        if to_high {
            if amount >= bin_data_vec[pos].2[bin_idx].1 {
                Self::advance_bin_data_vec(bin_data_vec, pos, to_high);
            }
            else {
                bin_data_vec[pos].2[bin_idx].1 -= amount;
            }
        }
        else {
            if amount >= bin_data_vec[pos].2[bin_idx].0 {
                Self::advance_bin_data_vec(bin_data_vec, pos, to_high);
            }
            else {
                bin_data_vec[pos].2[bin_idx].0 -= amount;
            }
        }
        
    }
    pub fn bin_swap_exact_out(bin_data_vec: &mut Vec<(u128, usize, [(u64, u64, u128); 70], usize, u128)>, pos: usize, to_high: bool, amount_out: u64) -> u64 {
        Self::bin_swap_out_with_amount(bin_data_vec, pos, !to_high, amount_out)
    }
    pub fn bin_swap_out(bin_data_vec: &mut Vec<(u128, usize, [(u64, u64, u128); 70], usize, u128)>, pos: usize, to_high: bool) -> u64 {
        let low_bin_array = &bin_data_vec[pos];
        let low_price = low_bin_array.0;
        let amount_out = if to_high {
            let low_amount = low_bin_array.2[low_bin_array.1].1;
            (low_amount as u128)
                .checked_mul(PRICE_MULTIPLIER as u128)
                .unwrap()
                .checked_div(low_price)
                .expect(&format!("error. low_price = {}", low_price))
                .checked_mul(1_000_000_000)
                .unwrap()
                .checked_div(1_000_000)
                .unwrap()
        }
        else {
            let low_amount = low_bin_array.2[low_bin_array.1].0;
            (low_amount as u128)
                .checked_mul(low_price)
                .unwrap()
                .checked_div(PRICE_MULTIPLIER as u128)
                .unwrap()
                .checked_mul(1_000_000)
                .unwrap()
                .checked_div(1_000_000_000)
                .unwrap()
        };
        amount_out as u64
        
    }
    pub fn bin_swap_out_with_amount(bin_data_vec: &mut Vec<(u128, usize, [(u64, u64, u128); 70], usize, u128)>, pos: usize, to_high: bool, amount: u64) -> u64 {
        let low_bin_array = &bin_data_vec[pos];
        let low_price = low_bin_array.0;
        let amount_out = if to_high {
            let low_amount = low_bin_array.2[low_bin_array.1].1.min(amount);
            let fee: u64 = Self::compute_fee_by_rate(low_amount, low_bin_array.4);
            let swap_amount = low_amount - fee;
            println!("to_high {}, low amount {}, fee {}, fee rate {}", to_high, low_amount, fee, low_bin_array.4);
            (swap_amount as u128)
                .checked_mul(PRICE_MULTIPLIER as u128)
                .unwrap()
                .checked_div(low_price)
                .unwrap()
        }
        else {
            let low_amount = low_bin_array.2[low_bin_array.1].0.min(amount);
            let fee: u64 = Self::compute_fee_by_rate(low_amount, low_bin_array.4);
            let swap_amount = low_amount - fee;
            println!("to_high {}, low amount {}, fee {}, fee rate {}", to_high, low_amount, fee, low_bin_array.4);
            (swap_amount as u128)
                .checked_mul(low_price)
                .unwrap()
                .checked_div(PRICE_MULTIPLIER as u128)
                .unwrap()
        };
        println!("to_high {}, amount_out {}", to_high, amount_out);
        amount_out as u64
        
    }
    pub fn get_bin_info(data: &[u8], i: usize) -> (u64, u64, u128) {
        let amount_x_start = 56 + i * 144;
        let amount_y_start = 56 + i * 144 + 8;
        let bin_price_start = 56 + i * 144 + 8 + 8;
        let amount_x_slice = array_ref![data, amount_x_start, 8];
        let amount_y_slice = array_ref![data, amount_y_start, 8];
        let price_slice = array_ref![data, bin_price_start, 16];
        let amount_x = u64::from_le_bytes(amount_x_slice.clone());
        let amount_y = u64::from_le_bytes(amount_y_slice.clone());
        let price_x64 = u128::from_le_bytes(price_slice.clone());
        let price = Self::get_price_from_x64(price_x64);
        (amount_x, amount_y, price)
    }
    pub fn get_total_fee_from_pool(pool_state_data: &[u8]) -> u128 {
        let input = array_ref![pool_state_data, 0, POOL_OFFSET_AFTER_BIN];
        let (_, _, bin_step_slice) = array_refs![input, POOL_OFFSET_UNTIL_BIN, 4, 2];
        let bin_step = u16::from_le_bytes(*bin_step_slice);
        
        let volatility_accumulator_slice = array_ref![pool_state_data, VOLATILITY_ACCUMULATOR_OFFSET, 4];
        let variable_fee_control_slice = array_ref![pool_state_data, VARIABLE_FEE_CONTROL_OFFSET, 4];
        let base_factor_slice = array_ref![pool_state_data, BASE_FACTOR_OFFSET, 2];

        let volatility_accumulator = u32::from_le_bytes(*volatility_accumulator_slice);
        let variable_fee_control = u32::from_le_bytes(*variable_fee_control_slice);
        let base_factor = u16::from_le_bytes(*base_factor_slice);
        Self::get_total_fee(volatility_accumulator, variable_fee_control, base_factor, bin_step)
    }
    
    pub fn get_price_from_x64(price_x64: u128) -> u128 {
        U128::from(price_x64)
            .mul_div_floor(
                U128::from(PRICE_MULTIPLIER),
                U128::from(Q64),
            )
            .unwrap()
            .as_u128()
    }
    
    /// Compute fee from amount, where fee is part of the amount. The result is ceil-ed.
    pub fn compute_fee_from_amount(amount_with_fees: u64, volatility_accumulator: u32, variable_fee_control: u32, base_factor: u16, bin_step: u16) -> u64 {
        // total_fee_rate 1e9 unit
        let total_fee_rate = Self::get_total_fee(volatility_accumulator, variable_fee_control, base_factor, bin_step);
        // Ceil division
        let fee_amount = u128::from(amount_with_fees)
            .checked_mul(total_fee_rate).unwrap()
            .checked_add((DLMM_FEE_PRECISION - 1).into()).unwrap();
        let scaled_down_fee = fee_amount.checked_div(DLMM_FEE_PRECISION.into()).unwrap();
        scaled_down_fee as u64
    }
    /// Compute fee from amount, where fee is part of the amount. The result is ceil-ed.
    pub fn compute_fee_by_rate(amount_with_fees: u64, total_fee_rate: u128) -> u64 {
        // Ceil division
        let fee_amount = u128::from(amount_with_fees)
            .checked_mul(total_fee_rate).unwrap()
            .checked_add((DLMM_FEE_PRECISION - 1).into()).unwrap();
        let scaled_down_fee = fee_amount.checked_div(DLMM_FEE_PRECISION.into()).unwrap();
        scaled_down_fee as u64
    }
    /// Total fee rate = base_fee_rate + variable_fee_rate
    pub fn get_total_fee(volatility_accumulator: u32, variable_fee_control: u32, base_factor: u16, bin_step: u16) -> u128 {
        let total_fee_rate = Self::get_base_fee(base_factor, bin_step)
            .checked_add(
                Self::get_variable_fee(volatility_accumulator, variable_fee_control, bin_step)
            )
            .unwrap();
        let total_fee_rate_cap = std::cmp::min(total_fee_rate, MAX_FEE_RATE.into());
        total_fee_rate_cap
    }
    /// Base fee rate = Base fee factor * bin step. This is in 1e9 unit.
    pub fn get_base_fee(base_factor: u16, bin_step: u16) -> u128 {
        u128::from(base_factor)
            .checked_mul(bin_step.into())
            .unwrap()
            // Make it to be the same as FEE_PRECISION defined for ceil_div later on.
            .checked_mul(10u128)
            .unwrap()
    }
    /// Variable fee rate = variable_fee_control * (variable_fee_accumulator * bin_step) ^ 2
    pub fn get_variable_fee(volatility_accumulator: u32, variable_fee_control: u32, bin_step: u16) -> u128 {
        Self::compute_variable_fee(volatility_accumulator, variable_fee_control, bin_step)
    }
    /// Variable fee rate = variable fee factor * (volatility_accumulator * bin_step)^2
    pub fn compute_variable_fee(volatility_accumulator: u32, variable_fee_control: u32, bin_step: u16) -> u128 {
        if variable_fee_control > 0 {
            let volatility_accumulator: u128 = volatility_accumulator.into();
            let bin_step: u128 = bin_step.into();
            let variable_fee_control: u128 = variable_fee_control.into();

            let square_vfa_bin = volatility_accumulator
                .checked_mul(bin_step).unwrap()
                .checked_pow(2)
                .expect("pow error");

            // Variable fee control, volatility accumulator, bin step are in basis point unit (10_000)
            // This is 1e20. Which > 1e9. Scale down it to 1e9 unit and ceiling the remaining.
            let v_fee = variable_fee_control.checked_mul(square_vfa_bin).unwrap();

            let scaled_v_fee = v_fee
                .checked_add(99_999_999_999)
                .unwrap()
                .checked_div(100_000_000_000)
                .unwrap();
            return scaled_v_fee;
        }

        0
    }
    
    
    
    pub fn analyze_dlmm_bin() {
        let mint_keypair = Keypair::new();
        let mint_pubkey = mint_keypair.pubkey();
        let faucet_addr = run_local_faucet(mint_keypair, None);
        

        let ledger_path = "/mnt/local-ledger";
        let _ = fs::remove_dir_all(ledger_path);
        let _ = fs::create_dir_all(ledger_path);

        let mut test_validator_genesis = TestValidatorGenesis::default();
        test_validator_genesis.ledger_path(PathBuf::from_str(ledger_path).unwrap());
        test_validator_genesis.max_genesis_archive_unpacked_size = Some(u64::MAX);
        test_validator_genesis.fee_rate_governor(FeeRateGovernor::new(0, 0));
        test_validator_genesis.rent(Rent {
            lamports_per_byte_year: 1,
            exemption_threshold: 1.0,
            ..Rent::default()
        });
        test_validator_genesis.faucet_addr(Some(faucet_addr));

        let test_validator_genesis = test_validator_genesis.add_accounts_from_directories(vec!["/mnt/onchain-bot-accounts"]).expect("error adding json accounts");
        
        let account = test_validator_genesis.accounts.get(
            &Pubkey::from_str("3G3mGvCHG8aAQnwoV2Fyaxj41vuFwbYmuUMys4Uu7ug1").unwrap()
        ).unwrap();
        let data = account.data();
        for i in 0..70 {
            let amount_x_start = 56 + i * 144;
            let amount_y_start = 56 + i * 144 + 8;
            let amount_x_slice = array_ref![data, amount_x_start, 8];
            let amount_y_slice = array_ref![data, amount_y_start, 8];
            let amount_x = u64::from_le_bytes(amount_x_slice.clone());
            let amount_y = u64::from_le_bytes(amount_y_slice.clone());
            if amount_x > 0 || amount_y > 0 {
                println!("i {},x {}, y {}", i, amount_x, amount_y);
            }
        }

        let account = test_validator_genesis.accounts.get(
            &Pubkey::from_str("28WnLxAM6rpjMToCVqaDys7dpiRmVUeQus1pvzGVR4G2").unwrap()
        ).unwrap();
        let data = account.data();
        for i in 0..70 {
            let amount_x_start = 56 + i * 144;
            let amount_y_start = 56 + i * 144 + 8;
            let amount_x_slice = array_ref![data, amount_x_start, 8];
            let amount_y_slice = array_ref![data, amount_y_start, 8];
            let amount_x = u64::from_le_bytes(amount_x_slice.clone());
            let amount_y = u64::from_le_bytes(amount_y_slice.clone());
            if amount_x > 0 || amount_y > 0 {
                println!("i {},x {}, y {}", i, amount_x, amount_y);
            }
        }

    }
    pub fn setup_validator() -> Self {
        // loading edges
        let edges_file_name = EDGES_FILE_PATH.to_string();
        let edges_data = fs::read_to_string(edges_file_name).expect("Failed to read JSON file");
        let edges: Vec<JupSwapEdge> =
            serde_json::from_str(&edges_data).expect("Failed to parse JSON");
        let edges_atomic: AtomicPtr<HashMap<String, JupSwapEdge>> =
            AtomicPtr::new(Box::into_raw(Box::new(HashMap::new())));
        let bot_edges: &mut HashMap<String, JupSwapEdge> = unsafe { &mut *edges_atomic.load(Ordering::SeqCst) };
        for edge in edges.iter() {
            bot_edges.insert(edge.pool.clone(), edge.clone());
        }
        println!("{} edges loaded!", bot_edges.len());

        let rpc_client = RpcClient::new_with_commitment(
            String::from(RPC_ENDPOINT),
            CommitmentConfig::confirmed(),
        );

        let payer = signature::read_keypair_file("/mnt/wallet/user.json").unwrap();
        println!("payer {:#?}", payer.pubkey());

        let mint_keypair = Keypair::new();
        let mint_pubkey = mint_keypair.pubkey();
        let faucet_addr = run_local_faucet(mint_keypair, None);
        

        let ledger_path = "/mnt/local-ledger";
        let _ = fs::remove_dir_all(ledger_path);
        let _ = fs::create_dir_all(ledger_path);

        let mut test_validator_genesis = TestValidatorGenesis::default();
        test_validator_genesis.ledger_path(PathBuf::from_str(ledger_path).unwrap());
        test_validator_genesis.max_genesis_archive_unpacked_size = Some(u64::MAX);
        test_validator_genesis.fee_rate_governor(FeeRateGovernor::new(0, 0));
        test_validator_genesis.rent(Rent {
            lamports_per_byte_year: 1,
            exemption_threshold: 1.0,
            ..Rent::default()
        });
        test_validator_genesis.faucet_addr(Some(faucet_addr));

        let test_validator_genesis = test_validator_genesis.add_accounts_from_directories(vec!["/mnt/local-accounts-solfi/account"]).expect("error adding json accounts");
        
        // let accounts = test_validator_genesis.accounts.clone();
        let parse_program_path = |program: &str| {
            let program_path = PathBuf::from(program);
            if !program_path.exists() {
                println!(
                    "Error: program file does not exist: {}",
                    program_path.display()
                );
                std::process::exit(1);
            }
            program_path
        };
        let mut program_files: HashSet<String> = HashSet::new();
        let matched_files = fs::read_dir("/mnt/local-accounts-solfi/program")
            .expect("program file error")
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| path.is_file() && path.extension() == Some(OsStr::new("so")))
            .map(|path| String::from(path.to_string_lossy()));

        program_files.extend(matched_files);

        let mut upgradeable_programs_to_load = vec![];
        for program in program_files.iter() {
            let program_path = parse_program_path(program);
            println!("loading program {}", program_path.file_stem().expect("error getting file name").to_str().unwrap());
            upgradeable_programs_to_load.push(UpgradeableProgramInfo {
                program_id: Pubkey::from_str(program_path.file_stem().expect("error getting file name").to_str().unwrap()).unwrap(),
                loader: solana_sdk::bpf_loader_upgradeable::id(),
                upgrade_authority: Pubkey::default(),
                program_path,
            });
        }
        
        let test_validator_genesis = test_validator_genesis.add_upgradeable_programs_with_path(&upgradeable_programs_to_load);
        let test_validator_genesis = test_validator_genesis.add_accounts_from_directories(vec!["/mnt/local-accounts-solfi/account"]).expect("error adding json accounts");
        
        let test_validator = test_validator_genesis
            .start_with_mint_address(mint_pubkey, SocketAddrSpace::Unspecified)
            .expect("validator start failed");
        // println!("test validator rpc url {}", test_validator.rpc_url());
        // let test_rpc_client = RpcClient::new_with_commitment(
        //     String::from(test_validator.rpc_url()),
        //     CommitmentConfig::confirmed(),
        // );
        // println!("account {:#?}", test_rpc_client.get_account(&Pubkey::from_str("SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe").unwrap()));
        
        let bank_forks = test_validator.bank_forks();
        let bank = bank_forks
            .read()
            .unwrap()
            .working_bank();

        bank.unfreeze_for_ledger_tool();
        while bank.freeze_started() {
            let ten_millis = std::time::Duration::from_millis(100);
            thread::sleep(ten_millis);
        }
        // for (key, data) in accounts.iter() {
        //     bank.store_account(key, data);
        // }

        bank.store_account(
            &payer.pubkey(),
            &AccountSharedData::new(
                1_000_000_000_000_000,
                0,
                &Pubkey::from_str("11111111111111111111111111111111").unwrap(),
            ),
        );

        println!("setup validator successfully");

        Runner {
            validator: AtomicPtr::new(Box::into_raw(Box::new(test_validator))),
            payer,
            edges: edges_atomic,
            slot: 0
        }
    }
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
    pub fn is_program_account(pubkey: &str) -> bool {
        GENESIS_PROGRAMS.contains(&pubkey)
    }
    pub fn get_multiple_accounts_data(&self, pubkeys: &[Pubkey]) -> Vec<Option<Vec<u8>>> {
        let mut accounts_info: Vec<Option<Vec<u8>>> = Vec::new();
        for pubkey in pubkeys.iter() {
            accounts_info.push(self.get_account_data(pubkey));
        }
        accounts_info
    }

    pub fn get_account_data(&self, pubkey: &Pubkey) -> Option<Vec<u8>> {
        let bank = self.bank();
        match bank.get_account_with_fixed_root(pubkey) {
            Some(account) => Some(account.data().to_vec()),
            None => None,
        }
    }

    pub fn bank(&self) -> Arc<Bank> {
        let validator = unsafe { &*self.validator.load(Ordering::SeqCst) };
        let bank_forks = validator.bank_forks();
        let new_bank = bank_forks.as_ref().read().unwrap().working_bank();

        // new_bank.set_sysvar_for_tests(
        //     &clock::Clock {
        //         slot: self.slot,
        //         ..clock::Clock::default()
        //     }
        // );
        // get latest slot
        // let rpc_client = RpcClient::new_with_commitment(
        //     String::from(RPC_ENDPOINT),
        //     CommitmentConfig::processed(),
        // );
        // let new_slot = rpc_client.get_slot().expect("error getting latest slot");
        // let new_slot = bank.slot()+1;
        // let new_bank = Arc::new(Bank::new_from_parent(bank, &Pubkey::default(), new_slot));


        new_bank.unfreeze_for_ledger_tool();
        while new_bank.freeze_started() {
            let ten_millis = std::time::Duration::from_millis(100);
            thread::sleep(ten_millis);
        }
        new_bank
    }

    pub fn create_and_simulate_ix_rpc(
        &self,
        txInstructions: &[Instruction],
        lookuptables: &[AddressLookupTableAccount],
        rpc_client: &RpcClient,
        inner_instructions: bool
    ) -> RpcResult<RpcSimulateTransactionResult> {
        let signers = vec![&self.payer];
        // let bank = Self::bank(instance);
        let blockhash = rpc_client.get_latest_blockhash().expect("blockhash error");

        // println!("lookuptables: {:#?}", lookuptables.iter().map(|l| l.key).collect::<Vec<_>>());

        let versionedMessage = V0(solana_sdk::message::v0::Message::try_compile(
            &self.payer.pubkey(),
            txInstructions,
            lookuptables,
            blockhash,
        )
        .unwrap());
        let tx = VersionedTransaction::try_new(versionedMessage, &signers).unwrap();
        rpc_client.simulate_transaction_with_config(&tx, RpcSimulateTransactionConfig{
            inner_instructions,
            ..RpcSimulateTransactionConfig::default()
        })
    }
    pub fn create_and_simulate_ix(
        &self,
        txInstructions: &[Instruction],
        lookuptables: &[AddressLookupTableAccount],
        bank: &Arc<Bank>,
        cpi_recording: bool
    ) -> TransactionSimulationResult {
        let signers = vec![&self.payer];
        // let bank = Self::bank(instance);
        let blockhash = bank.last_blockhash();

        // println!("lookuptables: {:#?}", lookuptables.iter().map(|l| l.key).collect::<Vec<_>>());

        let versionedMessage = V0(solana_sdk::message::v0::Message::try_compile(
            &self.payer.pubkey(),
            txInstructions,
            lookuptables,
            blockhash,
        )
        .unwrap());
        let tx = VersionedTransaction::try_new(versionedMessage, &signers).unwrap();
        Runner::simulate_transaction(tx, bank.clone(), cpi_recording)
    }
    pub fn create_ix(
        &self,
        txInstructions: &[Instruction],
        lookuptables: &[AddressLookupTableAccount],
        bank: &Arc<Bank>,
    ) -> VersionedTransaction {
        let signers = vec![&self.payer];
        // let bank = Self::bank(instance);
        let blockhash = bank.last_blockhash();

        // println!("lookuptables: {:#?}", lookuptables.iter().map(|l| l.key).collect::<Vec<_>>());

        let versionedMessage = V0(solana_sdk::message::v0::Message::try_compile(
            &self.payer.pubkey(),
            txInstructions,
            lookuptables,
            blockhash,
        )
        .unwrap());
        let tx = VersionedTransaction::try_new(versionedMessage, &signers).unwrap();
        tx
    }


    fn sanitize_transaction(
        transaction: VersionedTransaction,
        address_loader: impl AddressLoader,
    ) -> RuntimeTransaction<SanitizedTransaction> {
        RuntimeTransaction::try_create(transaction, MessageHash::Compute, None, address_loader, &HashSet::new())
            .map_err(|err| println!("invalid transaction: {err}"))
            .unwrap()
    }

    pub fn simulate_transaction(
        transaction: VersionedTransaction,
        bank: Arc<Bank>,
        cpi_recording: bool
    ) -> TransactionSimulationResult {
        let sanitized_tx = Self::sanitize_transaction(transaction, &*bank);

        let start = Instant::now();
        let tx_res = bank.simulate_transaction_unchecked(&sanitized_tx, cpi_recording);
        let elapsed = start.elapsed().as_nanos();
        // println!("svm elapsed {}ns", elapsed);

        tx_res
    }

    

    
}
