use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct StakeConfig {
    pub points_per_stake: u8,
    pub max_stake: u8,
    pub freeze_period: u32,
    pub rewards_bump:u8,
    pub bump: u8,
}

impl Space for StakeConfig {
    const INIT_SPACE: usize = ANCHOR_DISC +  U8_L * 4 + U32_L * 1;
}