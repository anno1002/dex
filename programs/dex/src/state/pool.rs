use anchor_lang::prelude::*;
use crate::constants::DISCRIMINATOR_SIZE;

#[account]
pub struct Pool_State {
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub token_x_vault: Pubkey,
    pub token_y_vault: Pubkey,
    pub bump: u8
}

impl Pool_State {
    //账户大小
    pub const SIZE: usize = DISCRIMINATOR_SIZE +32 + 32 + 32 + 32 + 1;
}