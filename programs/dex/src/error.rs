use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Invalid swap fee rate")]
    InvalidSwapFeeRate,
    #[msg("Invalid protocol fee rate")]
    InvalidProtocolFeeRate,
    #[msg("protocol ispaused")]
    Ispaused,
    #[msg("InvalidMint")]
    InvalidMint,
    #[msg("InvalidAmount")]
    InvalidAmount,
    #[msg("InsufficientBalance")]
    InsufficientBalance,

}
