use std::str::FromStr;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

const DECIMAL_POS: usize = 104;
const IS_STABLE_POS: usize = 105;
const FEES_START_POS: usize = 236;
const AUM_USD_POS: usize = 280;
const TARGET_RATIO_POS: usize = 151 + 55;
const FEE_RESERVES_POS: usize = 151 + 63;
const DOVE_PRICE_POS: usize = 168;
const PYTH_PRICE_POS: usize = 168;

const USD_DECIMALS: u64 = 1_000_000u64;
const PRICE_DECIMALS: u64 = 100_000_000u64;
const FEE_BPS_POWER: u64 = 10000;

const JUP_PERPS_PID: &str = "PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu";

const CUSTODY_INFOS: [[&str; 4]; 5] = [
    [
        //WSOL
        "7xS2gz2bTp3fwCC7knJvUWTEU9Tycczu6VhJYKgi1wdz",
        "FYq2BWQ1V5P1WFBqr3qB2Kb5yHVvSv7upzKodgQE5zXh",
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
        "6Jp2xZUTWdDD2ZyUPRzeMdc6AFQ5K3pFgZxk2EijfjnM",
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
    in_out: bool,
) -> u64 {
    let is_stable = if custody_data[IS_STABLE_POS] > 0 {
        true
    } else {
        false
    };

    let base_fee_pos = if is_stable {
        FEES_START_POS + 24
    } else {
        FEES_START_POS + 40
    };
    let tax_pos = if is_stable {
        FEES_START_POS + 32
    } else {
        FEES_START_POS + 48
    };
    let multi_pos = if is_stable {
        FEES_START_POS
    } else {
        FEES_START_POS + 16
    };

    let sub_vector_swap_fee: [u8; 8] = pool_data[base_fee_pos..(base_fee_pos + 8)]
        .try_into()
        .unwrap();
    let swap_base_fee_bps = u64::from_le_bytes(sub_vector_swap_fee);

    let sub_vector_tax_fee: [u8; 8] = pool_data[tax_pos..(tax_pos + 8)].try_into().unwrap();
    let tax_bps = u64::from_le_bytes(sub_vector_tax_fee);

    let sub_vector_multi: [u8; 8] = pool_data[multi_pos..(multi_pos + 8)].try_into().unwrap();
    let multi = u64::from_le_bytes(sub_vector_multi);

    let sub_vector_aum_usd: [u8; 16] = pool_data[AUM_USD_POS..(AUM_USD_POS + 16)]
        .try_into()
        .unwrap();
    let pool_aum_usd = u128::from_le_bytes(sub_vector_aum_usd);

    let target_ratio_vec: [u8; 8] = custody_data[TARGET_RATIO_POS..(TARGET_RATIO_POS + 8)]
        .try_into()
        .unwrap();
    let target_ratio_bps = u64::from_le_bytes(target_ratio_vec);

    let owned_vec: [u8; 8] = custody_data[(FEE_RESERVES_POS + 8)..(FEE_RESERVES_POS + 16)]
        .try_into()
        .unwrap();
    let owned = u64::from_le_bytes(owned_vec);

    let custody_aum_usd = (owned as u128)
        .checked_mul(token_price as u128)
        .unwrap()
        .checked_div(PRICE_DECIMALS as u128)
        .unwrap()
        .checked_mul(USD_DECIMALS as u128)
        .unwrap()
        .checked_div(in_decimals as u128)
        .unwrap();

    // let current_ratio_bps = (custody_aum_usd as u128).checked_mul(FEE_BPS_POWER as u128).unwrap().checked_div(pool_aum_usd).unwrap() as u64;
    let target_usd_amount = pool_aum_usd
        .checked_mul(FEE_BPS_POWER as u128)
        .unwrap()
        .checked_div(target_ratio_bps as u128)
        .unwrap();

    let initial_diff: i128 = (target_usd_amount as i128) - (custody_aum_usd as i128);
    let final_diff: i128 = if in_out {
        initial_diff + (usd_amount as u128).checked_mul(multi as u128).unwrap() as i128
    } else {
        initial_diff - (usd_amount as u128).checked_mul(multi as u128).unwrap() as i128
    };

    let dynamic_fee_bps = if initial_diff.abs() > final_diff.abs() {
        swap_base_fee_bps
            .checked_add(
                (tax_bps as u128)
                    .checked_mul(initial_diff.abs().checked_add(final_diff.abs()).unwrap() as u128)
                    .unwrap()
                    .checked_div(2u128)
                    .unwrap()
                    .checked_div(target_usd_amount)
                    .unwrap() as u64,
            )
            .unwrap()
    } else {
        let temp_fee = (tax_bps as u128)
            .checked_mul(initial_diff.abs() as u128)
            .unwrap()
            .checked_div(target_usd_amount)
            .unwrap() as u64;
        if swap_base_fee_bps > temp_fee {
            swap_base_fee_bps.checked_sub(temp_fee).unwrap()
        } else {
            0u64
        }
    };
    dynamic_fee_bps
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
            pubkey: ata_from.clone(), // trade
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: ata_to.clone(), // trade_authority
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
    let sub_vector_dove_in: [u8; 8] = dove_data_in[DOVE_PRICE_POS..(DOVE_PRICE_POS + 8)]
        .try_into()
        .unwrap();
    let dove_in_price = u64::from_le_bytes(sub_vector_dove_in);

    let sub_vector_pyth_in: [u8; 8] = pyth_data_in[PYTH_PRICE_POS..(PYTH_PRICE_POS + 8)]
        .try_into()
        .unwrap();
    let pyth_in_price = u64::from_le_bytes(sub_vector_pyth_in);

    let sub_vector_dove_out: [u8; 8] = dove_data_out[DOVE_PRICE_POS..(DOVE_PRICE_POS + 8)]
        .try_into()
        .unwrap();
    let dove_out_price = u64::from_le_bytes(sub_vector_dove_out);

    let sub_vector_pyth_out: [u8; 8] = pyth_data_out[PYTH_PRICE_POS..(PYTH_PRICE_POS + 8)]
        .try_into()
        .unwrap();
    let pyth_out_price = u64::from_le_bytes(sub_vector_pyth_out);

    let token_in_price: u64 = dove_in_price.min(pyth_in_price);
    let token_out_price: u64 = dove_out_price.max(pyth_out_price);

    let in_decimals = 10u64.pow(custody_data_in[DECIMAL_POS] as u32);
    let out_decimals = 10u64.pow(custody_data_out[DECIMAL_POS] as u32);

    let swap_usd_amount = (in_amount as u128)
        .checked_mul(token_in_price as u128)
        .unwrap()
        .checked_div(PRICE_DECIMALS as u128)
        .unwrap()
        .checked_mul(USD_DECIMALS as u128)
        .unwrap()
        .checked_div(in_decimals as u128)
        .unwrap() as u64;

    let swap_fee_bps_in = get_perps_dynamic_fee(
        pool_data,
        custody_data_in,
        token_in_price,
        swap_usd_amount,
        in_decimals,
        true,
    );
    let swap_fee_bps_out = get_perps_dynamic_fee(
        pool_data,
        custody_data_out,
        token_out_price,
        swap_usd_amount,
        in_decimals,
        false,
    );
    let fee_bps = swap_fee_bps_in.max(swap_fee_bps_out);
    let fee = swap_usd_amount
        .checked_mul(fee_bps)
        .unwrap()
        .checked_div(FEE_BPS_POWER)
        .unwrap();

    let usd_out_amount = swap_usd_amount - fee;
    let out_is_stable = if custody_data_out[IS_STABLE_POS] > 0 {
        true
    } else {
        false
    };
    if out_is_stable {
        usd_out_amount
    } else {
        (usd_out_amount as u128)
            .checked_mul(PRICE_DECIMALS as u128)
            .unwrap()
            .checked_div(token_out_price as u128)
            .unwrap()
            .checked_mul(out_decimals as u128)
            .unwrap()
            .checked_div(USD_DECIMALS as u128)
            .unwrap() as u64
    }
}
