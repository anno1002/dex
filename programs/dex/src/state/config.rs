use anchor_lang::prelude::*;
use crate::constants::DISCRIMINATOR_SIZE;

#[account]
pub struct Config_State {
    pub swap_fee_rate: u16,
    pub protocol_fee_rate: u16,
    pub protocol_fee_account: Pubkey,
    pub admin: Pubkey,
    pub is_paused: bool,
    pub bump: u8,
}

impl Config_State {
    //账户大小
    pub const SIZE: usize = DISCRIMINATOR_SIZE + 2 + 2 + 32 + 32 + 1 + 1;
}