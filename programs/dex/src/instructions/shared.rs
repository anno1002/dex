use anchor_lang::prelude::*;
use crate::state::Config_State;
use crate::error::ErrorCode;

pub fn check_not_paused(
    config: &Account<Config_State>
) -> Result<()>{
    if config.is_paused{
        return err!(ErrorCode::Ispaused);
    }
    Ok(())
}