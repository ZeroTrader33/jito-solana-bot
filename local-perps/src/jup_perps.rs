use std::str::FromStr;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

const DECIMAL_POS: usize = 104;
const IS_STABLE_POS: usize = 105;
const IMPACT_FEE_SCALAR_POS: usize = 151;
const FEES_START_POS: usize = 236;
const AUM_USD_POS: usize = 180;
const TARGET_RATIO_POS: usize = 151 + 55;
const FEE_RESERVES_POS: usize = 151 + 63;
const DOVE_PRICE_POS: usize = 73;
const CUSTODY_MINT_POS: usize = 40;
const AG_DOVE_PRICE_POS: usize = 168;
const PYTH_PRICE_POS: usize = 73;

const PYTH_PRICE_ACC_DATA_SIZE: usize = 134;
const DOVE_PRICE_ACC_DATA_SIZE: usize = 283;
const AG_DOVE_PRICE_ACC_DATA_SIZE: usize = 394;

const USD_DECIMALS: u64 = 1_000_000u64;
const PRICE_DECIMALS: u64 = 100_000_000u64;
const FEE_BPS_POWER: u64 = 10000;

const JUP_PERPS_PID: &str = "PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu";
const USDC_MINT_STR: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
const USDT_MINT_STR: &str = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB";
const WSOL_MINT_STR: &str = "So11111111111111111111111111111111111111112";
const WETH_MINT_STR: &str = "7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs";
const WBTC_MINT_STR: &str = "3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh";

const CUSTODY_INFOS: [[&str; 4]; 5] = [
    [
        //WSOL
        "7xS2gz2bTp3fwCC7knJvUWTEU9Tycczu6VhJYKgi1wdz",
        "39cWjvHrpHNz2SbXv6ME4NPhqBDBd4KsjUYv5JkHEAJU",
        "7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE",
        "BUvduFTd2sWFagCunBPLupG8fBTJqweLw9DuhruNFSCm",
    ],
    [
        //WETH
        "AQCGyheWPLeo6Qp9WpYS9m3Qj479t7R636N9ey1rEjEn",
        "5URYohbPy32nxK1t3jAHVNfdWY2xTubHiFvLrE3VhXEp",
        "42amVS4KgzR9rA28tkVYqVXjq9Qa8dcZQMbH5EYFX6XC",
        "Bgarxg65CEjN3kosjCW5Du3wEqvV3dpCGDR3a2HRQsYJ",
    ],
    [
        //WBTC
        "5Pv3gM9JrFFH883SWAhvJC9RPYmo8UNxuFtv5bMMALkm",
        "hUqAT1KQ7eW1i6Csp9CXYtpPfSAvi835V7wKi5fRfmC",
        "4cSM2e6rvbGQUFiJbqytoVMi5GgghSMr8LwVrT9VPSPo",
        "FgpXg2J3TzSs7w3WGYYE7aWePdrxBVLCXSxmAKnCZNtZ",
    ],
    [
        //USDC
        "G18jKKXQwBbrHeiK3C9MRXhkHsLHf7XgCSisykV46EZa",
        "A28T5pKtscnhDo6C1Sz786Tup88aTjt8uyKewjVvPrGk",
        "Dpw1EAVrSB1ibxiDQyTAW6Zip3J4Btk2x4SgApQCeFbX",
        "WzWUoCmtVv7eqAbU3BfKPU3fhLP6CXR8NCJH78UK9VS",
    ],
    [
        //USDT
        "4vkNeXiYEUizLdrpdPS1eC2mccyM4NUPRtERrk6ZETkk",
        "Fgc93D641F8N2d1xLjQ4jmShuD3GE3BsCXA56KBQbF5u",
        "HT2PLQBcG5EiCcNSaMHAjSgd9F98ecpATbk4Sk5oYuM",
        "Gex24YznvguMad1mBzTQ7a64U1CJy59gvsStQmNnnwAd",
    ],
];

#[derive(BorshSerialize, BorshDeserialize)]
struct PerpsSwapParams {
    pub amount_in: u64,
    pub minimum_out: u64,
}
fn get_custody_info_idx(mint: &Pubkey) -> usize {
    match mint.to_string().as_ref() {
        "So11111111111111111111111111111111111111112" => 0usize,
        "7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs" => 1usize,
        "3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh" => 2usize,
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v" => 3usize,
        "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB" => 4usize,
        _ => 0usize,
    }
}
fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{namespace}:{name}");

    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(&crate::hash::hash(preimage.as_bytes()).to_bytes()[..8]);
    sighash
}

fn get_perps_dynamic_fee(
    pool_data: &[u8],
    custody_data: &[u8],
    token_price: u64,
    usd_amount: u64,
    in_decimals: u64,
    is_in: bool,
    in_is_stable: bool,
    out_is_stable: bool,
) -> u64 {

    let is_stable_swap = in_is_stable && out_is_stable;

    let base_fee_pos = if !is_stable_swap {
        FEES_START_POS + 24
    } else {
        FEES_START_POS + 40
    };
    let tax_pos = if !is_stable_swap {
        FEES_START_POS + 32
    } else {
        FEES_START_POS + 48
    };
    let multi_pos = if !is_stable_swap {
        FEES_START_POS
    } else {
        FEES_START_POS + 8
    };

    let sub_vector_swap_fee: [u8; 8] = pool_data[base_fee_pos..(base_fee_pos + 8)]
        .try_into()
        .unwrap();
    let swap_base_fee_bps = u64::from_le_bytes(sub_vector_swap_fee);

    // println!("swap_base_fee_bps {}", swap_base_fee_bps);

    let sub_vector_tax_fee: [u8; 8] = pool_data[tax_pos..(tax_pos + 8)].try_into().unwrap();
    let tax_bps = u64::from_le_bytes(sub_vector_tax_fee);
    // println!("tax_bps {}", tax_bps);

    let sub_vector_multi: [u8; 8] = pool_data[multi_pos..(multi_pos + 8)].try_into().unwrap();
    let multi = u64::from_le_bytes(sub_vector_multi);
    // for i in 0..pool_data.len() - 32 {
    //     let sub_vector_aum_usd: [u8; 32] = pool_data[i..(i + 32)]
    //         .try_into()
    //         .unwrap();
    //     let pubkey = Pubkey::new_from_array(sub_vector_aum_usd);
    //     // let pool_aum_usd = u128::from_le_bytes(sub_vector_aum_usd);
    //     println!("pubkey {}: {}",i, pubkey);
    // }
    // println!("multi {}", multi);
    let sub_vector_aum_usd: [u8; 16] = pool_data[AUM_USD_POS..(AUM_USD_POS + 16)]
        .try_into()
        .unwrap();
    let pool_aum_usd = u128::from_le_bytes(sub_vector_aum_usd);
    // println!("pool_aum_usd {}", pool_aum_usd);

    let target_ratio_vec: [u8; 8] = custody_data[TARGET_RATIO_POS..(TARGET_RATIO_POS + 8)]
        .try_into()
        .unwrap();
    let target_ratio_bps = u64::from_le_bytes(target_ratio_vec);
    println!("target_ratio_bps {}", target_ratio_bps);

    let owned_vec: [u8; 8] = custody_data[(FEE_RESERVES_POS + 8)..(FEE_RESERVES_POS + 16)]
        .try_into()
        .unwrap();
    let owned = u64::from_le_bytes(owned_vec);
    println!("owned {}", owned);

    let custody_aum_usd = (owned as u128)
        .checked_mul(token_price as u128)
        .unwrap()
        .checked_div(PRICE_DECIMALS as u128)
        .unwrap()
        .checked_mul(USD_DECIMALS as u128)
        .unwrap()
        .checked_div(in_decimals as u128)
        .unwrap();
    // println!("in_decimals {}, token_price {}", in_decimals, token_price);
    println!("custody_aum_usd {}, pool_aum_usd {}", custody_aum_usd, pool_aum_usd);
    // println!("custody_aum_usd {}, pool_aum_usd {}, target_ratio_bps {}", custody_aum_usd, pool_aum_usd, target_ratio_bps);
    // let current_ratio_bps = (custody_aum_usd as u128).checked_mul(FEE_BPS_POWER as u128).unwrap().checked_div(pool_aum_usd).unwrap() as u64;
    let target_usd_amount = pool_aum_usd
        .checked_mul(target_ratio_bps as u128)
        .unwrap()
        .checked_div(FEE_BPS_POWER as u128)
        .unwrap();
    println!("target_usd_amount {}", target_usd_amount);
    // println!("input usd_amount {}", usd_amount);
    let initial_diff: i128 = (target_usd_amount as i128) - (custody_aum_usd as i128);
    
    let dynamic_fee_bps = {
        let final_diff = if is_in {
            initial_diff - (usd_amount as u128).checked_mul(multi as u128).unwrap() as i128
        } else {
            initial_diff + (usd_amount as u128).checked_mul(multi as u128).unwrap() as i128
        };
        println!("initial_diff {}, final_diff {}, initial_diff.abs() < final_diff.abs() {}", initial_diff, final_diff, initial_diff.abs() < final_diff.abs());
        if initial_diff.abs() < final_diff.abs() {
            let dynamic_delta_mul = (tax_bps as u128)
                .checked_mul(initial_diff.abs().checked_add(final_diff.abs()).unwrap() as u128)
                .unwrap();
            // let dynamic_delta = dynamic_delta_mul
            //     .checked_div(2 * target_usd_amount)
            //     .unwrap() as u64;
            let dynamic_delta = if (is_in && !in_is_stable) || (!is_in && !out_is_stable) {
                dynamic_delta_mul
                    .div_ceil(2 * target_usd_amount) as u64
            } else {
                dynamic_delta_mul
                    .checked_div(2 * target_usd_amount)
                    .unwrap() as u64
            };
            swap_base_fee_bps
                .checked_add(
                    dynamic_delta
                )
                .unwrap()
        } else {
            let dynamic_delta_mul = (tax_bps as u128)
                .checked_mul(initial_diff.abs() as u128)
                .unwrap();
            let dynamic_delta = if (is_in && !in_is_stable) || (!is_in && !out_is_stable) {
                dynamic_delta_mul
                    .div_ceil(target_usd_amount) as u64
            } else {
                dynamic_delta_mul
                    .checked_div(target_usd_amount)
                    .unwrap() as u64
            };
            if swap_base_fee_bps > dynamic_delta {
                swap_base_fee_bps.checked_sub(dynamic_delta).unwrap()
            } else {
                0u64
            }
        }
    };
    
    // println!("dynamic_fee_bps {}", dynamic_fee_bps);
    dynamic_fee_bps
}
fn get_oracle_price(price_account_data: &[u8]) -> u64 {
    let price = if price_account_data.len() == DOVE_PRICE_ACC_DATA_SIZE {
        let sub_vector_dove_in: [u8; 8] = price_account_data[DOVE_PRICE_POS..(DOVE_PRICE_POS + 8)]
            .try_into()
            .unwrap();
        u64::from_le_bytes(sub_vector_dove_in)
    } else if price_account_data.len() == PYTH_PRICE_ACC_DATA_SIZE {
        let sub_vector_dove_in: [u8; 8] = price_account_data[PYTH_PRICE_POS..(PYTH_PRICE_POS + 8)]
            .try_into()
            .unwrap();
        u64::from_le_bytes(sub_vector_dove_in)
    }
    else if price_account_data.len() == AG_DOVE_PRICE_ACC_DATA_SIZE {
        let sub_vector_dove_in: [u8; 8] = price_account_data[AG_DOVE_PRICE_POS..(AG_DOVE_PRICE_POS + 8)]
            .try_into()
            .unwrap();
        u64::from_le_bytes(sub_vector_dove_in)
    } else {
        println!("price_account_data.len() {}", price_account_data.len());
        0
    };
    price
}
fn check_pool_mint(custody_data: &[u8], mint: &Pubkey) -> bool {
    let pool_mint_bytes: [u8; 32] = custody_data[CUSTODY_MINT_POS..(CUSTODY_MINT_POS + 32)]
        .try_into()
        .unwrap();
    let pool_mint = Pubkey::new_from_array(pool_mint_bytes);
    pool_mint.eq(mint)
}
fn get_impact_scalar(custody_data: &[u8]) -> u64 {
    let bytes: [u8; 8] = custody_data[IMPACT_FEE_SCALAR_POS..(IMPACT_FEE_SCALAR_POS + 8)]
        .try_into()
        .unwrap();
    u64::from_le_bytes(bytes)
}
fn get_price_impact_fee_bps(trade_size_usdc: u64, fee_scalar: u64) -> u64 {
    (trade_size_usdc as u128)
        .checked_mul(FEE_BPS_POWER as u128).unwrap()
        .checked_div(fee_scalar as u128).unwrap() as u64
}
pub fn generate_perps_ix(
    mint_from: &Pubkey,
    mint_to: &Pubkey,
    owner: &Pubkey,
    ata_from: &Pubkey,
    ata_to: &Pubkey,
    amount_in: u64,
) -> Instruction {
    let custody_info_idx_from = get_custody_info_idx(mint_from);
    let custody_info_idx_to = get_custody_info_idx(mint_to);
    let program_id = Pubkey::from_str(JUP_PERPS_PID).unwrap();
    let discriminator = sighash("global", "swap2");
    let params = PerpsSwapParams {
        amount_in,
        minimum_out: 0,
    };
    let custody_info_from = CUSTODY_INFOS[custody_info_idx_from];
    let custody_info_to = CUSTODY_INFOS[custody_info_idx_to];
    let main_acc_meta = vec![
        AccountMeta {
            pubkey: owner.clone(), // signer
            is_signer: true,
            is_writable: true,
        },
        AccountMeta {
            pubkey: ata_from.clone(),
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: ata_to.clone(), 
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: Pubkey::from_str("AVzP2GeRmqGphJsMxWoqjpUifPpCret7LqWhD8NWQK49").unwrap(),
            is_signer: false,
            is_writable: false,
        },
        AccountMeta {
            pubkey: Pubkey::from_str("H4ND9aYttUVLFmNypZqLjZ52FYiGvdEB45GmwNoKEjTj").unwrap(),
            is_signer: false,
            is_writable: false,
        },
        AccountMeta {
            pubkey: Pubkey::from_str("5BUwFW4nRbftYTDMbgxykoFWqWHPzahFSNAaaaJtVKsq").unwrap(),
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: Pubkey::from_str(custody_info_from[0]).unwrap(),
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: Pubkey::from_str(custody_info_from[1]).unwrap(),
            is_signer: false,
            is_writable: false,
        },
        AccountMeta {
            pubkey: Pubkey::from_str(custody_info_from[2]).unwrap(),
            is_signer: false,
            is_writable: false,
        },
        AccountMeta {
            pubkey: Pubkey::from_str(custody_info_from[3]).unwrap(),
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: Pubkey::from_str(custody_info_to[0]).unwrap(),
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: Pubkey::from_str(custody_info_to[1]).unwrap(),
            is_signer: false,
            is_writable: false,
        },
        AccountMeta {
            pubkey: Pubkey::from_str(custody_info_to[2]).unwrap(),
            is_signer: false,
            is_writable: false,
        },
        AccountMeta {
            pubkey: Pubkey::from_str(custody_info_to[3]).unwrap(),
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
            is_signer: false,
            is_writable: false,
        },
        AccountMeta {
            pubkey: Pubkey::from_str("37hJBDnntwqhGbK7L6M1bLyvccj4u55CCUiLPdYkiqBN").unwrap(),
            is_signer: false,
            is_writable: false,
        },
        AccountMeta {
            pubkey: Pubkey::from_str("PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu").unwrap(),
            is_signer: false,
            is_writable: false,
        },
    ];
    Instruction::new_with_borsh(program_id, &(discriminator, params), main_acc_meta)
}

pub fn get_accounts_to_fetch(
    mint_from: &Pubkey,
    mint_to: &Pubkey
) -> Vec<Pubkey> {
    let custody_info_idx_from = get_custody_info_idx(mint_from);
    let custody_info_idx_to = get_custody_info_idx(mint_to);
    let custody_info_from = CUSTODY_INFOS[custody_info_idx_from];
    let custody_info_to = CUSTODY_INFOS[custody_info_idx_to];

    vec![
        Pubkey::from_str("5BUwFW4nRbftYTDMbgxykoFWqWHPzahFSNAaaaJtVKsq").unwrap(),
        Pubkey::from_str(custody_info_from[0]).unwrap(),
        Pubkey::from_str(custody_info_from[1]).unwrap(),
        Pubkey::from_str(custody_info_from[2]).unwrap(),
        Pubkey::from_str(custody_info_to[0]).unwrap(),
        Pubkey::from_str(custody_info_to[1]).unwrap(),
        Pubkey::from_str(custody_info_to[2]).unwrap(),
    ]
}
pub fn get_all_perps_accounts() -> Vec<Pubkey> {
    let mut all = vec![
        Pubkey::from_str("AVzP2GeRmqGphJsMxWoqjpUifPpCret7LqWhD8NWQK49").unwrap(),
        Pubkey::from_str("H4ND9aYttUVLFmNypZqLjZ52FYiGvdEB45GmwNoKEjTj").unwrap(),
        Pubkey::from_str("5BUwFW4nRbftYTDMbgxykoFWqWHPzahFSNAaaaJtVKsq").unwrap(),
        Pubkey::from_str("37hJBDnntwqhGbK7L6M1bLyvccj4u55CCUiLPdYkiqBN").unwrap(),
    ];
    for accounts in CUSTODY_INFOS {
        for account in accounts {
            let key = Pubkey::from_str(account).unwrap();
            all.push(key);
        }
    }
    all
}
pub fn get_perps_swap_out(
    pool_data: &[u8],
    custody_data_in: &[u8],
    dove_data_in: &[u8],
    pyth_data_in: &[u8],
    custody_data_out: &[u8],
    dove_data_out: &[u8],
    pyth_data_out: &[u8],
    in_amount: u64,
) -> u64 {
    // println!("swap spread in {}", get_swap_spread(custody_data_in));
    // println!("swap spread out {}", get_swap_spread(custody_data_out));
    // for i in 0..(dove_data_in.len()-8) {
    //     let sub_vector_pyth_in: [u8; 8] = dove_data_in[i..(i + 8)]
    //     .try_into()
    //     .unwrap();
    //     let pyth_in_price = u64::from_le_bytes(sub_vector_pyth_in);
    //     println!("dove {} - {:?}", i, pyth_in_price);
    // }
    // let sub_vector_dove_in: [u8; 8] = dove_data_in[DOVE_PRICE_POS..(DOVE_PRICE_POS + 8)]
    //     .try_into()
    //     .unwrap();
    // let dove_in_price = u64::from_le_bytes(sub_vector_dove_in);
    let dove_in_price = get_oracle_price(dove_data_in);
    // for i in 0..126 {
    //     let sub_vector_pyth_in: [u8; 8] = pyth_data_in[i..(i + 8)]
    //     .try_into()
    //     .unwrap();
    //     let pyth_in_price = u64::from_le_bytes(sub_vector_pyth_in);
    //     println!("pyth {} - {:?}", i, pyth_in_price);
    // }
    
    // println!("dove_in_price {:?}", dove_in_price);

    // let sub_vector_pyth_in: [u8; 8] = pyth_data_in[PYTH_PRICE_POS..(PYTH_PRICE_POS + 8)]
    //     .try_into()
    //     .unwrap();
    // let pyth_in_price = u64::from_le_bytes(sub_vector_pyth_in);
    let pyth_in_price = get_oracle_price(pyth_data_in);
    // println!("pyth_in_price {:?}", pyth_in_price);

    // let sub_vector_dove_out: [u8; 8] = dove_data_out[DOVE_PRICE_POS..(DOVE_PRICE_POS + 8)]
    //     .try_into()
    //     .unwrap();
    // let dove_out_price = u64::from_le_bytes(sub_vector_dove_out);
    let dove_out_price = get_oracle_price(dove_data_out);
    // println!("dove_out_price {:?}", dove_out_price);

    // let sub_vector_pyth_out: [u8; 8] = pyth_data_out[PYTH_PRICE_POS..(PYTH_PRICE_POS + 8)]
    //     .try_into()
    //     .unwrap();
    // let pyth_out_price = u64::from_le_bytes(sub_vector_pyth_out);
    let pyth_out_price = get_oracle_price(pyth_data_out);
    // println!("pyth_out_price {:?}", pyth_out_price);

    let token_in_price_min: u64 = dove_in_price; //.min(pyth_in_price);
    let token_in_price_max: u64 = dove_in_price; //.max(pyth_in_price);
    let token_out_price_min: u64 = dove_out_price; //.min(pyth_out_price);
    let token_out_price_max: u64 = dove_out_price; //dove_out_price.max(pyth_out_price);

    let in_decimals = 10u64.pow(custody_data_in[DECIMAL_POS] as u32);
    let out_decimals = 10u64.pow(custody_data_out[DECIMAL_POS] as u32);

    // if usdt, swap usd amount  = in_amount
    let usdc_mint = Pubkey::from_str(USDC_MINT_STR).unwrap();
    let usdt_mint = Pubkey::from_str(USDT_MINT_STR).unwrap();
    let wsol_mint = Pubkey::from_str(WSOL_MINT_STR).unwrap();
    let weth_mint = Pubkey::from_str(WETH_MINT_STR).unwrap();
    let wbtc_mint = Pubkey::from_str(WBTC_MINT_STR).unwrap();
    let in_is_usdt = check_pool_mint(custody_data_in, &usdt_mint);
    let out_is_usdc = check_pool_mint(custody_data_out, &usdc_mint);
    let out_is_usdt = check_pool_mint(custody_data_out, &usdt_mint);

    let out_is_wsol = check_pool_mint(custody_data_out, &wsol_mint);
    let out_is_weth = check_pool_mint(custody_data_out, &weth_mint);
    let out_is_wbtc = check_pool_mint(custody_data_out, &wbtc_mint);

    let swap_usd_amount = if in_is_usdt {
        in_amount
    } else {
        (in_amount as u128)
        .checked_mul(token_in_price_min as u128)
        .unwrap()
        .checked_div(PRICE_DECIMALS as u128)
        .unwrap()
        .checked_mul(USD_DECIMALS as u128)
        .unwrap()
        .checked_div(in_decimals as u128)
        .unwrap() as u64
    };
    let in_is_stable = if custody_data_in[IS_STABLE_POS] > 0 { true } else { false };
    let out_is_stable = if custody_data_out[IS_STABLE_POS] > 0 { true } else { false };


    let swap_fee_bps_in = get_perps_dynamic_fee(
        pool_data,
        custody_data_in,
        token_in_price_min,
        swap_usd_amount,
        in_decimals,
        true,
        in_is_stable,
        out_is_stable
    );
    // println!("swap_fee_bps_in {}", swap_fee_bps_in);
    let swap_fee_bps_out = get_perps_dynamic_fee(
        pool_data,
        custody_data_out,
        token_out_price_max,
        swap_usd_amount,
        out_decimals,
        false,
        in_is_stable,
        out_is_stable
    );
    println!("dove_in_price {}, pyth_in_price {}, dove_out_price {}, pyth_out_price {}", dove_in_price, pyth_in_price, dove_out_price, pyth_out_price);
    println!("swap_fee_bps_in {}, swap_fee_bps_out {}", swap_fee_bps_in, swap_fee_bps_out);
    let fee_bps = swap_fee_bps_in.max(swap_fee_bps_out);
    // if is_stable && fee_bps < 2 {
    //     fee_bps = 2;
    // }
    // if !is_stable && fee_bps < 8 {
    //     fee_bps = 8;
    // }
    // println!("");
    // println!("swap_usd_amount {}, swap_fee_bps_in {}, swap_fee_bps_out {}, fee_bps {}", swap_usd_amount, swap_fee_bps_in, swap_fee_bps_out, fee_bps);
    // println!("token_in_price_min {}, token_in_price_max {}, token_out_price_min {}, token_out_price_max {}", token_in_price_min, token_in_price_max, token_out_price_min, token_out_price_max);
    

    // println!("final fee_bps {}", fee_bps);
    // if !out_is_stable {
    //     let impact_fee_scalar = get_impact_scalar(custody_data_out);
    //     let impact_fee_bps = get_price_impact_fee_bps(swap_usd_amount, impact_fee_scalar);
    //     println!("impact_fee_bps {}", impact_fee_bps);
    // }
    

    let fee = swap_usd_amount
        .checked_mul(fee_bps)
        .unwrap()
        .checked_div(FEE_BPS_POWER)
        .unwrap();

    
    let swap_usd_amount_no_fee = swap_usd_amount - fee;


    println!("sdk_swap_usd_amount {}, fee_bps {}, swap_usd_amount_no_fee {}", swap_usd_amount, fee_bps, swap_usd_amount_no_fee);
    let amount_out = if out_is_usdc { // out is usdc only
        swap_usd_amount_no_fee
    } else if out_is_usdt {
        (swap_usd_amount_no_fee as u128)
            .checked_mul(PRICE_DECIMALS as u128)
            .unwrap()
            .checked_div(token_out_price_max as u128)
            .unwrap() as u64
    } else {
        (swap_usd_amount_no_fee as u128)
            .checked_mul(out_decimals as u128)
            .unwrap()
            .checked_div(USD_DECIMALS as u128)
            .unwrap()
            .checked_mul(PRICE_DECIMALS as u128)
            .unwrap()
            .checked_div(token_out_price_max as u128)
            .unwrap() as u64
    };
    
    // fee adjustment
    let final_amount_out = amount_out;
    // let final_amount_out = if out_is_wsol {
    //     (amount_out as u128)
    //         .checked_mul(PRICE_DECIMALS as u128)
    //         .unwrap()
    //         .checked_div((PRICE_DECIMALS + 50_000u64) as u128)
    //         .unwrap() as u64
    // } else if out_is_weth {
    //     (amount_out as u128)
    //         .checked_mul(PRICE_DECIMALS as u128)
    //         .unwrap()
    //         .checked_div((PRICE_DECIMALS + 10_000u64) as u128)
    //         .unwrap() as u64
    // } else {
    //     amount_out
    // };

    // let sub_vector_protocol_share: [u8; 8] = pool_data[(FEES_START_POS + 64)..(FEES_START_POS + 72)]
    //     .try_into()
    //     .unwrap();
    // let protocol_share_bps = u64::from_le_bytes(sub_vector_protocol_share);
    // println!("protocol_share_bps {}", protocol_share_bps);

    
    // let protocol_fee = fee
    //     .checked_mul(protocol_share_bps)
    //     .unwrap()
    //     .checked_div(FEE_BPS_POWER)
    //     .unwrap();
    // let amount_out_no_fee = amount_out;// - fee;
    // println!("amount_out_no_fee {}", amount_out_no_fee);
    final_amount_out

}
