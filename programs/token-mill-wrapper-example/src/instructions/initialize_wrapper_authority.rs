use anchor_lang::prelude::*;

use crate::state::WrapperAuthority;

#[derive(Accounts)]
pub struct InitializeWrapperAuthority<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + WrapperAuthority::INIT_SPACE,
        seeds = ["wrapper_authority".as_bytes()],
        bump
    )]
    pub wrapper_authority: Account<'info, WrapperAuthority>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeWrapperAuthority>) -> Result<()> {
    let market_authority = &mut ctx.accounts.wrapper_authority;

    market_authority.initialize(ctx.bumps.wrapper_authority);

    Ok(())
}
