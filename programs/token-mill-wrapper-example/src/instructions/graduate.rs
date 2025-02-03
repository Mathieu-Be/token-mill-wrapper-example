use anchor_lang::{prelude::*, solana_program::native_token::sol_to_lamports};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use token_mill::{
    cpi::accounts::FreeMarket,
    program::TokenMill,
    state::{Market, SwapAuthorityBadge},
};

use crate::state::WrapperSwapAuthority;

#[event_cpi]
#[derive(Accounts)]
pub struct Graduate<'info> {
    #[account(
        mut,
        has_one = quote_token_mint
    )]
    pub market: AccountLoader<'info, Market>,

    pub swap_authority_badge: Account<'info, SwapAuthorityBadge>,

    pub quote_token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = quote_token_mint,
        associated_token::authority = market,
        associated_token::token_program = quote_token_program
    )]
    pub market_quote_token_ata: InterfaceAccount<'info, TokenAccount>,

    // Wrapper authority will sign the cpi call to the market and act as the swap authority
    pub wrapper_swap_authority: Account<'info, WrapperSwapAuthority>,

    pub signer: Signer<'info>,

    pub token_mill_program: Program<'info, TokenMill>,

    pub quote_token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<Graduate>) -> Result<()> {
    // Here we check that the market reached the required market cap
    // TBD: Make sure this is the right value you want to use
    assert!(ctx.accounts.market_quote_token_ata.amount >= sol_to_lamports(69_000.0));

    let wrapper_authority_seeds: &[&[&[u8]]] = &[&[
        &b"wrapper_swap_authority"[..],
        &[ctx.accounts.wrapper_swap_authority.bump],
    ]];

    let context = CpiContext::new_with_signer(
        ctx.accounts.token_mill_program.to_account_info(),
        FreeMarket {
            market: ctx.accounts.market.to_account_info(),
            swap_authority_badge: ctx.accounts.swap_authority_badge.to_account_info(),
            swap_authority: ctx.accounts.wrapper_swap_authority.to_account_info(),
            event_authority: ctx.accounts.event_authority.to_account_info(),
            program: ctx.accounts.token_mill_program.to_account_info(),
        },
        wrapper_authority_seeds,
    );

    token_mill::cpi::free_market(context)?;

    Ok(())
}
