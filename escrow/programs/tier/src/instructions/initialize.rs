use crate::{constants::*, error::*, states::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [GLOBAL_STATE_SEED],
        bump,
        payer = authority,
        space = 8 + core::mem::size_of::<GlobalState>()
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    #[account(
        init_if_needed,
        seeds = [POOL_SEED],
        bump,
        token::mint = yoiu_token_mint,
        token::authority = global_state,
        payer = authority
    )]
    pub pool: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        seeds = [DAO_TREASURY_SEED],
        bump,
        token::mint = yoiu_token_mint,
        token::authority = global_state,
        payer = authority
    )]
    pub dao_treasury: Box<Account<'info, TokenAccount>>,

    pub yoiu_token_mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Initialize<'info> {
    pub fn validate(&self) -> Result<()> {
        if self.global_state.is_initialized == 1 {
            require!(
                self.global_state.authority.eq(&self.authority.key()),
                StakingError::NotAllowedAuthority
            )
        }
        Ok(())
    }
}

/// Initialize Staking Program for the first time
/// to init global state with some data for validation
///
#[access_control(ctx.accounts.validate())]
pub fn handle(
    ctx: Context<Initialize>,
    new_authority: Pubkey,
    treasury: Pubkey,
    tier_grades: [u16; 10],
    available_tier: u8,
) -> Result<()> {
    let accts = ctx.accounts;
    accts.global_state.is_initialized = 1;
    accts.global_state.authority = new_authority;
    accts.global_state.treasury = treasury;
    accts.global_state.yoiu_token_mint = accts.yoiu_token_mint.key();
    accts.global_state.tier_grades = tier_grades;
    // avaiable tier means max tier. tier starts from zero
    accts.global_state.available_tier = available_tier;
    Ok(())
}