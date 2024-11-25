use anchor_lang::prelude::*;

use crate::errors::RouterError;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct BondingCurve {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub complete: bool,
}

pub fn try_deserialize<T: AnchorDeserialize>(data: &[u8]) -> Result<T> {
    let deserialized: T = T::try_from_slice(data).map_err(|_| error!(RouterError::InvalidData))?; // Handle deserialization error
    Ok(deserialized)
}
