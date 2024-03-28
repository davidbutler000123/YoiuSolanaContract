use crate::{constants::*, error::*, states::*};
use anchor_lang::prelude::*;
use solana_client::rpc_client::RpcClient;
use std::str::FromStr;

use solana_program::borsh0_10;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};
use solana_program::stake::state::StakeState;

use crate::states::UserState;

//entrypoint!(query);

#[derive(Accounts)]
pub struct Validator<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [USER_STATE_SEED, user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + core::mem::size_of::<UserState>()
    )]
    pub user_state: Box<Account<'info, UserState>>,
    
    pub system_program: Program<'info, System>,
}

pub fn query(
    ctx: Context<Validator>,
    validator: &str
) -> Result<()> {
    // Assuming the first account is the stake account you want to query
    let rpc = RpcClient::new(validator);
    let stake_account_info = &ctx.accounts[rpc];

    // Deserialize stake account data
    let stake_state: StakeState = borsh0_10::try_from_slice_unchecked(&stake_account_info.data.borrow())?;

    // Extract staked amount depending on stake state
    let staked_amount = match stake_state {
        StakeState::Initialized(meta) => {
            // Handle initialized but not delegated stake
            0
        },
        StakeState::Stake(meta, stake) => {
            // Here, `stake.delegation.stake` contains the staked amount
            stake.delegation.stake
        },
        _ => return Ok(()),
    };

    ctx.accounts.user_state.tier = match staked_amount {
        0..=999 => 5,
        1000..=5_999 => 4,
        6_000..=29_999 => 3,
        30_000..=79_999 => 2,
        _ => 1
    };

    Ok(())
}
