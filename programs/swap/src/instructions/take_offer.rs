use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, TransferChecked},
};

use crate::Offer;

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    pub taker_token_account_b: Account<'info, TokenAccount>,

    #[account(mut,seeds = [b"vault".as_ref()],bump)]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        associated_token::mint = token_mint_b,
        associated_token::authority = vault,
        associated_token::token_program = token_program
    )]
    pub vault_ata: Account<'info, TokenAccount>,
    pub token_mint_b: Account<'info, Mint>,

    #[account(mut)]
    pub offer: Account<'info, Offer>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn send_wanted_tokens_to_maker(context: &Context<TakeOffer>) -> Result<()> {
    let accounts = TransferChecked {
        from: context.accounts.vault_ata.to_account_info(),
        to: context.accounts.taker_token_account_b.to_account_info(),
        authority: context.accounts.vault.to_account_info(),
        mint: context.accounts.token_mint_b.to_account_info(),
    };

    let bump = &[context.bumps.vault];
    let seeds: &[&[u8]] = &[b"vault".as_ref(), bump];
    let signer_seeds = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        context.accounts.token_program.to_account_info(),
        accounts,
        signer_seeds,
    );

    transfer_checked(cpi_ctx, context.accounts.offer.token_b_wanted_amount, context.accounts.token_mint_b.decimals)?;

    Ok(())
}
    