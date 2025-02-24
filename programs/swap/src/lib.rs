pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("HvQMhtZGZDB1Fo3cumnqdaSHxXbmboUsxA7uEQMdxqNy");

#[program]
pub mod swap {
    use super::*;

    pub fn initialize_new(context: Context<Initialize>) -> Result<()> {
        instructions::initialize::initialize(context)
    }

    pub fn make_offer(
        context: Context<MakeOffer>,
        id: u64,
        token_a_offered_amount: u64,
        token_b_wanted_amount: u64,
    ) -> Result<()> {
        instructions::make_offer::send_offered_tokens_to_vault(&context, token_a_offered_amount)?;
        instructions::make_offer::save_offer(context, id, token_b_wanted_amount)
    }

    pub fn take_offer(context: Context<TakeOffer>) -> Result<()> {
        instructions::take_offer::send_wanted_tokens_to_maker(&context)
    }

    pub fn add_liquidity(context: Context<DepositSpl>, amount: u64) -> Result<()> {
        instructions::vault_management::deposit_spl(context, amount)
    }

    pub fn withdraw_liquidity(context: Context<WithdrawSpl>, amount: u64) -> Result<()> {
        instructions::vault_management::withdraw_spl(context, amount)
    }
}
