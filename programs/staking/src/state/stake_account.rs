use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub last_update: i64,
    pub bump: u8,
}

impl Space for StakeAccount {
    const INIT_SPACE: usize = ANCHOR_DISC + PUBKEY_L * 2 + I64_L * 1 + U8_L * 1;
}