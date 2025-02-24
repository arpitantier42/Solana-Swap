use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Initial {
    pub admin: Pubkey,
    pub vault: Pubkey
}