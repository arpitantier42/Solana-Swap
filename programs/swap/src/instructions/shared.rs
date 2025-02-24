use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer as SplTransfer};

use anchor_spl::token_interface::TokenInterface;

pub fn transfer_tokens<'info>(
    from: &Account<'info, TokenAccount>,
    to: &Account<'info, TokenAccount>,
    amount: &u64,
    authority: &Signer<'info>,
    token_program: &Interface<'info, TokenInterface>,
) -> Result<()> {
    let transfer_accounts_options = SplTransfer {
        from: from.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };
    token::transfer(
        CpiContext::new(token_program.to_account_info(), transfer_accounts_options),
        amount.clone(),
    )?;
    Ok(())
}
