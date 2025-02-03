use anchor_lang::prelude::*;

use crate::state::WrapperSwapAuthority;

#[derive(Accounts)]
pub struct InitializeWrapperAuthority<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + WrapperSwapAuthority::INIT_SPACE,
        seeds = ["wrapper_swap_authority".as_bytes()],
        bump
    )]
    pub wrapper_swap_authority: Account<'info, WrapperSwapAuthority>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeWrapperAuthority>) -> Result<()> {
    let market_authority = &mut ctx.accounts.wrapper_swap_authority;

    market_authority.initialize(ctx.bumps.wrapper_swap_authority);

    Ok(())
}
