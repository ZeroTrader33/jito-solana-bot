use std::convert::TryInto;

use crate::bank_bot::SharedMemClientErr;
use super::*;

pub const NO_EXPLICIT_SQRT_PRICE_LIMIT: u128 = 0u128;

#[derive(PartialEq, Debug)]
pub struct SwapStepComputation {
    pub amount_in: u64,
    pub amount_out: u64,
    pub next_price: u128,
    pub fee_amount: u64,
}

pub fn compute_swap(
    amount_remaining: u64,
    fee_rate: u16,
    liquidity: u128,
    sqrt_price_current: u128,
    sqrt_price_target: u128,
    amount_specified_is_input: bool,
    a_to_b: bool,
) -> Result<SwapStepComputation, SharedMemClientErr> {
    let mut amount_fixed_delta = get_amount_fixed_delta(
        sqrt_price_current,
        sqrt_price_target,
        liquidity,
        amount_specified_is_input,
        a_to_b,
    )?;

    let mut amount_calc = amount_remaining;
    if amount_specified_is_input {
        amount_calc = checked_mul_div(
            amount_remaining as u128,
            FEE_RATE_MUL_VALUE - fee_rate as u128,
            FEE_RATE_MUL_VALUE,
        )?
        .try_into().unwrap();
    }

    let next_sqrt_price = if amount_calc >= amount_fixed_delta {
        sqrt_price_target
    } else {
        get_next_sqrt_price(
            sqrt_price_current,
            liquidity,
            amount_calc,
            amount_specified_is_input,
            a_to_b,
        )?
    };

    let is_max_swap = next_sqrt_price == sqrt_price_target;

    let amount_unfixed_delta = get_amount_unfixed_delta(
        sqrt_price_current,
        next_sqrt_price,
        liquidity,
        amount_specified_is_input,
        a_to_b,
    )?;

    // If the swap is not at the max, we need to readjust the amount of the fixed token we are using
    if !is_max_swap {
        amount_fixed_delta = get_amount_fixed_delta(
            sqrt_price_current,
            next_sqrt_price,
            liquidity,
            amount_specified_is_input,
            a_to_b,
        )?;
    }

    let (amount_in, mut amount_out) = if amount_specified_is_input {
        (amount_fixed_delta, amount_unfixed_delta)
    } else {
        (amount_unfixed_delta, amount_fixed_delta)
    };

    // Cap output amount if using output
    if !amount_specified_is_input && amount_out > amount_remaining {
        amount_out = amount_remaining;
    }

    let fee_amount = if amount_specified_is_input && !is_max_swap {
        amount_remaining - amount_in
    } else {
        checked_mul_div_round_up(
            amount_in as u128,
            fee_rate as u128,
            FEE_RATE_MUL_VALUE - fee_rate as u128,
        )?
        .try_into().unwrap()
    };

    Ok(SwapStepComputation {
        amount_in,
        amount_out,
        next_price: next_sqrt_price,
        fee_amount,
    })
}

fn get_amount_fixed_delta(
    sqrt_price_current: u128,
    sqrt_price_target: u128,
    liquidity: u128,
    amount_specified_is_input: bool,
    a_to_b: bool,
) -> Result<u64, SharedMemClientErr> {
    if a_to_b == amount_specified_is_input {
        get_amount_delta_a(
            sqrt_price_current,
            sqrt_price_target,
            liquidity,
            amount_specified_is_input,
        )
    } else {
        get_amount_delta_b(
            sqrt_price_current,
            sqrt_price_target,
            liquidity,
            amount_specified_is_input,
        )
    }
}

fn get_amount_unfixed_delta(
    sqrt_price_current: u128,
    sqrt_price_target: u128,
    liquidity: u128,
    amount_specified_is_input: bool,
    a_to_b: bool,
) -> Result<u64, SharedMemClientErr> {
    if a_to_b == amount_specified_is_input {
        get_amount_delta_b(
            sqrt_price_current,
            sqrt_price_target,
            liquidity,
            !amount_specified_is_input,
        )
    } else {
        get_amount_delta_a(
            sqrt_price_current,
            sqrt_price_target,
            liquidity,
            !amount_specified_is_input,
        )
    }
}
