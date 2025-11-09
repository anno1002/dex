use anchor_lang::accounts::signer;
use anchor_lang::prelude::*;
use crate::state::Config_State;
use crate::constants::CONFIG_SEED;
use crate::error::ErrorCode;

fn check_config_params(swap_fee_rate: u16, protocol_fee_rate: u16) -> Result<()> {
    if swap_fee_rate > 1000 {
        return err!(ErrorCode::InvalidSwapFeeRate);
    }
    if protocol_fee_rate > 50 {
        return err!(ErrorCode::InvalidProtocolFeeRate);
    }
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeConfig <'info>{
    //手续费
    //协议费
    //协议费收款账户
    //管理员
    //是否暂停
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, 
        payer = payer, 
        space = Config_State::SIZE,
        seeds = [CONFIG_SEED], 
        bump)]
    pub config: Account<'info, Config_State>,
    
    /// CHECK: This account is used to receive protocol fees
    pub protocol_fee_account: AccountInfo<'info>,

    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,

}

pub fn initialize_config(
    ctx: Context<InitializeConfig>,
    _swap_fee_rate: u16,
    _protocol_fee_rate: u16,
) -> Result<()> {
    //检查参数合理性
    check_config_params(_swap_fee_rate, _protocol_fee_rate)?;

    let config = &mut ctx.accounts.config;

    config.swap_fee_rate = _swap_fee_rate;
    config.protocol_fee_rate = _protocol_fee_rate;
    config.protocol_fee_account = ctx.accounts.protocol_fee_account.key();
    config.admin = ctx.accounts.admin.key();
    config.is_paused = false;
    config.bump = ctx.bumps.config;

    msg!("Config initialized, swap fee rate: {}, protocol fee rate: {}",   
    config.swap_fee_rate, config.protocol_fee_rate);

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateBasicConfig <'info>{
    //手续费
    //协议费
    //协议费收款账户
    //管理员
    //是否暂停
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [CONFIG_SEED], 
        bump = config.bump,
        constraint = config.admin == admin.key() @ ErrorCode::Unauthorized
    )]
    pub config: Account<'info, Config_State>,

}

pub fn update_basic_config(
    ctx: Context<UpdateBasicConfig>,
    _swap_fee_rate: u16,
    _protocol_fee_rate: u16,
    _is_paused: bool,
) -> Result<()> {
    //检查参数合理性
    check_config_params(_swap_fee_rate, _protocol_fee_rate)?;

    let config = &mut ctx.accounts.config;

    config.swap_fee_rate = _swap_fee_rate;
    config.protocol_fee_rate = _protocol_fee_rate;
    config.is_paused = _is_paused;

    msg!("Config updated, swap fee rate: {}, protocol fee rate: {}, is paused: {}", 
    config.swap_fee_rate, config.protocol_fee_rate, config.is_paused);

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateProtocolAcc <'info>{
    //手续费
    //协议费
    //协议费收款账户
    //管理员
    //是否暂停
    pub admin: Signer<'info>,
    /// CHECK: This account is used to receive protocol fees
    pub new_protocol_fee_account: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [CONFIG_SEED], 
        bump = config.bump,
        constraint = config.admin == admin.key() @ ErrorCode::Unauthorized
    )]
    pub config: Account<'info, Config_State>,

}

pub fn update_protocol_account(
    ctx: Context<UpdateProtocolAcc>,
) -> Result<()> {

    let config = &mut ctx.accounts.config;
    let new_protocol_account = ctx.accounts.new_protocol_fee_account.key();
    config.protocol_fee_account = new_protocol_account;

    msg!("update new protocol account: {}", 
    new_protocol_account);

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateAdmin <'info>{
    //手续费
    //协议费
    //协议费收款账户
    //管理员
    //是否暂停
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: This account is used to receive protocol fees
    pub new_admin: Signer<'info>,
    #[account(
        mut,
        seeds = [CONFIG_SEED], 
        bump = config.bump,
        constraint = config.admin == admin.key() @ ErrorCode::Unauthorized
    )]
    pub config: Account<'info, Config_State>,

}

pub fn update_admin(
    ctx: Context<UpdateAdmin>,
) -> Result<()> {

    let config = &mut ctx.accounts.config;
    let new_admin = ctx.accounts.new_admin.key();
    config.protocol_fee_account = new_admin;

    msg!("update new admin: {}", 
    new_admin);

    Ok(())
}


