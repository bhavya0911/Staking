use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct UserAccount {
    pub points: u32,
    pub amount_staked: u8,
    pub bump: u8,
}

impl Space for UserAccount {
    const INIT_SPACE: usize = ANCHOR_DISC + U8_L * 2 + U32_L * 1;
}