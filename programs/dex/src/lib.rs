pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("E57Y9dLtceTEwW4yHuEn7JifhHygLWw9x3dPCkmXeCeP");

#[program]
pub mod dex {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        swap_fee_rate: u16,
        protocol_fee_rate: u16,
    ) -> Result<()> {
        instructions::config::initialize_config(
            ctx,
            swap_fee_rate,
            protocol_fee_rate,
        )
    }

    pub fn update_basic_config(
        ctx: Context<UpdateBasicConfig>,
        swap_fee_rate: u16,
        protocol_fee_rate: u16,
        is_paused: bool
    ) -> Result<()> {
        instructions::config::update_basic_config(
            ctx,
            swap_fee_rate,
            protocol_fee_rate,
            is_paused
        )
    }

    pub fn update_protocol_account(
        ctx: Context<UpdateProtocolAcc>,
    ) -> Result<()> {
        instructions::config::update_protocol_account(
            ctx
        )
    }

    pub fn update_admin(
        ctx: Context<UpdateAdmin>,
    ) -> Result<()> {
        instructions::config::update_admin(
            ctx
        )
    }
}
