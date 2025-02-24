use anchor_lang::prelude::*;

use crate::Initial;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,payer = deployer,space = 8+89, seeds = [b"initial".as_ref()], bump)]
    pub initial: Account<'info, Initial>,

    #[account(mut)]
    pub deployer: Signer<'info>,
    #[account(mut,seeds = [b"vault".as_ref()],bump)]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.initial.admin = ctx.accounts.deployer.key();
    ctx.accounts.initial.vault = ctx.accounts.vault.key();

    Ok(())
}

