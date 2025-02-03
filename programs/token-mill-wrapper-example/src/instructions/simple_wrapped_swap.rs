use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use token_mill::{
    cpi::accounts::PermissionedSwap,
    program::TokenMill,
    state::{Market, SwapAuthorityBadge, TokenMillConfig},
    SwapAmountType, SwapType,
};

use crate::state::WrapperSwapAuthority;

#[event_cpi]
#[derive(Accounts)]
pub struct SimpleWrappedSwap<'info> {
    pub config: Account<'info, TokenMillConfig>,

    #[account(mut)]
    pub market: AccountLoader<'info, Market>,

    pub swap_authority_badge: Account<'info, SwapAuthorityBadge>,

    pub base_token_mint: InterfaceAccount<'info, Mint>,

    pub quote_token_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub market_base_token_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub market_quote_token_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub user_base_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub user_quote_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub protocol_quote_token_ata: InterfaceAccount<'info, TokenAccount>,

    // Wrapper authority will sign the cpi call to the market and act as the swap authority
    pub wrapper_authority: Account<'info, WrapperSwapAuthority>,

    pub user: Signer<'info>,

    pub token_mill_program: Program<'info, TokenMill>,

    pub base_token_program: Interface<'info, TokenInterface>,

    pub quote_token_program: Interface<'info, TokenInterface>,
}

pub fn handler(
    ctx: Context<SimpleWrappedSwap>,
    swap_type: SwapType,
    swap_amount_type: SwapAmountType,
    amount: u64,
    other_amount_threshold: u64,
) -> Result<()> {
    let wrapper_authority_seeds: &[&[&[u8]]] = &[&[
        &b"wrapper_swap_authority"[..],
        &[ctx.accounts.wrapper_authority.bump],
    ]];

    let context = CpiContext::new_with_signer(
        ctx.accounts.token_mill_program.to_account_info(),
        PermissionedSwap {
            config: ctx.accounts.config.to_account_info(),
            market: ctx.accounts.market.to_account_info(),
            swap_authority_badge: ctx.accounts.swap_authority_badge.to_account_info(),
            base_token_mint: ctx.accounts.base_token_mint.to_account_info(),
            quote_token_mint: ctx.accounts.quote_token_mint.to_account_info(),
            market_base_token_ata: ctx.accounts.market_base_token_ata.to_account_info(),
            market_quote_token_ata: ctx.accounts.market_quote_token_ata.to_account_info(),
            user_base_token_account: ctx.accounts.user_base_token_account.to_account_info(),
            user_quote_token_account: ctx.accounts.user_quote_token_account.to_account_info(),
            protocol_quote_token_ata: ctx.accounts.protocol_quote_token_ata.to_account_info(),
            referral_token_account: None,
            swap_authority: ctx.accounts.wrapper_authority.to_account_info(),
            user: ctx.accounts.user.to_account_info(),
            base_token_program: ctx.accounts.base_token_program.to_account_info(),
            quote_token_program: ctx.accounts.quote_token_program.to_account_info(),
            event_authority: ctx.accounts.event_authority.to_account_info(),
            program: ctx.accounts.token_mill_program.to_account_info(),
        },
        wrapper_authority_seeds,
    );

    token_mill::cpi::permissioned_swap(
        context,
        swap_type,
        swap_amount_type,
        amount,
        other_amount_threshold,
    )?;

    Ok(())
}
