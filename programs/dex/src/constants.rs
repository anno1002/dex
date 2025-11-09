use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";
#[constant]
pub const CONFIG_SEED: &[u8] = b"config";
#[constant]
pub const POOL_SEED: &[u8] = b"pool";
#[constant]
pub const VAULT_SEED: &[u8] = b"vault";

pub const DISCRIMINATOR_SIZE: usize = 8;
