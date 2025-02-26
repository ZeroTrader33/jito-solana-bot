use crate::bank_bot::SharedMemClientErr;

// Adds a signed liquidity delta to a given integer liquidity amount.
// Errors on overflow or underflow.
pub fn add_liquidity_delta(liquidity: u128, delta: i128) -> Result<u128, SharedMemClientErr> {
    if delta == 0 {
        return Ok(liquidity);
    }
    if delta > 0 {
        liquidity
            .checked_add(delta as u128)
            .ok_or(SharedMemClientErr::LiquidityOverflow("SharedMemClientErr".to_string()))
    } else {
        liquidity
            .checked_sub(delta.unsigned_abs())
            .ok_or(SharedMemClientErr::LiquidityUnderflow("SharedMemClientErr".to_string()))
    }
}

// Converts an unsigned liquidity amount to a signed liquidity delta
pub fn convert_to_liquidity_delta(
    liquidity_amount: u128,
    positive: bool,
) -> Result<i128, SharedMemClientErr> {
    if liquidity_amount > i128::MAX as u128 {
        // The liquidity_amount is converted to a liquidity_delta that is represented as an i128
        // By doing this conversion we lose the most significant bit in the u128
        // Here we enforce a max value of i128::MAX on the u128 to prevent loss of data.
        return Err(SharedMemClientErr::LiquidityTooHigh("SharedMemClientErr".to_string()));
    }
    Ok(if positive {
        liquidity_amount as i128
    } else {
        -(liquidity_amount as i128)
    })
}
