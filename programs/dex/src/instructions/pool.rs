use anchor_lang::accounts::signer;
use anchor_lang::prelude::*;
use crate::check_not_paused;
use crate::state::{Config_State,Pool_State};
use crate::constants::{CONFIG_SEED,POOL_SEED,VAULT_SEED};
use crate::error::ErrorCode;
use anchor_spl::token::{Mint,TokenAccount,Token,transfer,Transfer};


#[derive(Accounts)]
pub struct InitializePool <'info>{
    #[account(mut)]
    pub payer: Signer<'info>,

    //配置账户
    #[account(
        mut,
        seeds = [CONFIG_SEED], 
        bump = config.bump,
    )]
    pub config: Account<'info, Config_State>,

    #[account(
        init,
        payer = payer,
        space = Pool_State::SIZE,
        seeds = [
            POOL_SEED,
            token_x_mint.key().as_ref(),
            token_y_mint.key().as_ref(),
        ],
        bump
    )]
    pub pool: Account<'info,Pool_State>,

    pub token_x_mint: Account<'info,Mint>,
    pub token_y_mint: Account<'info,Mint>,

    #[account(
        init,
        payer = payer,
        seeds = [
            VAULT_SEED,
            pool.key().as_ref(),
            token_x_mint.key().as_ref(),
        ],
        bump,
        token::mint = token_x_mint,
        token::authority = pool,
    )]
    pub token_x_vault:Account<'info,TokenAccount>,

    #[account(
        init,
        payer = payer,
        seeds = [
            VAULT_SEED,
            pool.key().as_ref(),
            token_y_mint.key().as_ref(),
        ],
        bump,
        token::mint = token_y_mint,
        token::authority = pool,
    )]
    pub token_y_vault:Account<'info,TokenAccount>,

    #[account(mut)]
    pub user_token_x:Account<'info,TokenAccount>,
    #[account(mut)]
    pub user_token_y:Account<'info,TokenAccount>,

    pub system_program: Program<'info,System>,

    pub token_program: Program<'info,Token>,
}

pub fn initialize_pool(
    ctx: Context<InitializePool>,
    initial_token_x: u64,
    initial_token_y: u64
) -> Result<()> {
    let config = &ctx.accounts.config;
    let token_x_mint = &ctx.accounts.token_x_mint;
    let token_y_mint = &ctx.accounts.token_y_mint;
    let user_token_x = &ctx.accounts.user_token_x;
    let user_token_y = &ctx.accounts.user_token_y;
    let pool = &mut ctx.accounts.pool;
    let token_x_vault = &ctx.accounts.token_x_vault;
    let token_y_vault = &ctx.accounts.token_y_vault;
    let token_program = &ctx.accounts.token_program;

    //检查协议是否暂停
    check_not_paused(config)?;
    //检查代币 x 和 y 是否相同
    if token_x_mint.key() == token_y_mint.key(){
        return err!(ErrorCode::InvalidMint);
    }
    //检查代币是否大于零
    if initial_token_x == 0 || initial_token_y == 0{
        return err!(ErrorCode::InvalidAmount);
    }
    //检查用户的余额是否大于流动性
    if user_token_x.amount < initial_token_x || user_token_y.amount < initial_token_y{
        return err!(ErrorCode::InsufficientBalance);
    }
    //初始化池子账户
    pool.token_x_mint = token_x_mint.key();
    pool.token_y_mint = token_y_mint.key();
    pool.token_x_vault = token_x_vault.key();
    pool.token_y_vault = token_y_vault.key();
    pool.bump = ctx.bumps.pool;
    //转移流动性
    let cpi_account_x = Transfer{
        from: user_token_x.to_account_info(),
        to: token_x_vault.to_account_info(),
        authority: user_token_x.to_account_info(),
    };
    let cpi_program = token_program.to_account_info();
    let cpi_context_x = CpiContext::new(cpi_program, cpi_account_x);
    transfer(cpi_context_x, initial_token_x);

    let cpi_account_y = Transfer{
        from: user_token_y.to_account_info(),
        to: token_y_vault.to_account_info(),
        authority: user_token_y.to_account_info(),
    };
    let cpi_program = token_program.to_account_info();
    let cpi_context_y = CpiContext::new(cpi_program, cpi_account_y);
    transfer(cpi_context_y, initial_token_y);
    //TODO LP代币

    msg!("流动性初始化完毕，pool:{},initial_x:{},initial_y:{}",pool.key(),initial_token_x,initial_token_y);
    Ok(())
}



