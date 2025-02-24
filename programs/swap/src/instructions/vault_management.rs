use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer as SplTransfer};
use anchor_spl::token_interface::{transfer_checked, TransferChecked};
use anchor_spl::{associated_token::AssociatedToken, token::Mint};

use crate::Initial;

#[derive(Accounts)]
pub struct DepositSpl<'info> {
    #[account(mut, constraint=depositer.key==&initial.admin)]
    pub depositer: Signer<'info>,
    #[account(mut)]
    pub depositer_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_ata: Account<'info, TokenAccount>,
    pub initial: Account<'info, Initial>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawSpl<'info> {
    #[account(mut,constraint=admin.key==&initial.admin)]
    pub admin: Signer<'info>,
    #[account(mut,seeds = [b"vault".as_ref()],bump)]
    pub vault: SystemAccount<'info>,
    #[account(mut,associated_token::mint = token,associated_token::authority = vault)]
    pub vault_ata: Account<'info, TokenAccount>,
    #[account(mut, associated_token::mint = token,associated_token::authority = admin)]
    pub admin_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub initial: Account<'info, Initial>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum AmountError {
    #[msg("amount must be greater than zero")]
    InvalidAmount,

    #[msg("insufficient balance")]
    InsufficientBalance,
}

pub fn deposit_spl(ctx: Context<DepositSpl>, amount: u64) -> Result<()> {
    require!(amount > 0, AmountError::InvalidAmount);

    let destination = &ctx.accounts.vault_ata;
    let source = &ctx.accounts.depositer_ata;
    let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.depositer;

    let cpi_accounts = SplTransfer {
        from: source.to_account_info().clone(),
        to: destination.to_account_info().clone(),
        authority: authority.to_account_info().clone(),
    };

    let cpi_program = token_program.to_account_info();
    token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

    Ok(())
}

pub fn withdraw_spl(ctx: Context<WithdrawSpl>, amount: u64) -> Result<()> {
    let pda_balance = ctx.accounts.vault_ata.amount;
    require!(pda_balance >= amount, AmountError::InsufficientBalance);
    require!(amount > 0, AmountError::InvalidAmount);

    let accounts = TransferChecked {
        from: ctx.accounts.vault_ata.to_account_info(),
        to: ctx.accounts.admin_ata.to_account_info(),
        authority: ctx.accounts.vault.to_account_info(),
        mint: ctx.accounts.token.to_account_info(),
    };

    let bump = &[ctx.bumps.vault];
    let seeds: &[&[u8]] = &[b"pda".as_ref(), bump];
    let signer_seeds = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        accounts,
        signer_seeds,
    );

    transfer_checked(cpi_ctx, amount, ctx.accounts.token.decimals)?;
    Ok(())
}
