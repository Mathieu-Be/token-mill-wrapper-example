use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};
use token_mill::{
    cpi::accounts::PermissionedSwap,
    program::TokenMill,
    state::{Market, TokenMillConfig},
    SwapAmountType, SwapType,
};

use crate::state::WrapperAuthority;

#[event_cpi]
#[derive(Accounts)]
pub struct WrappedBuyWithExtraFee<'info> {
    pub config: Account<'info, TokenMillConfig>,

    #[account(mut)]
    pub market: AccountLoader<'info, Market>,

    pub market_authority: Account<'info, WrapperAuthority>,

    pub base_token_mint: InterfaceAccount<'info, Mint>,

    pub quote_token_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub market_base_token_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub market_quote_token_ata: InterfaceAccount<'info, TokenAccount>,

    // This ATA account will get the extra fee
    #[account(
        mut,
        associated_token::mint = base_token_mint,
        associated_token::authority = wrapper_authority,
        associated_token::token_program = base_token_program
    )]
    pub wrapper_authority_base_token_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub user_base_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub user_quote_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub protocol_quote_token_ata: InterfaceAccount<'info, TokenAccount>,

    // Wrapper authority will sign the cpi call to the market and act as the market authority
    pub wrapper_authority: Account<'info, WrapperAuthority>,

    pub user: Signer<'info>,

    pub token_mill_program: Program<'info, TokenMill>,

    pub base_token_program: Interface<'info, TokenInterface>,

    pub quote_token_program: Interface<'info, TokenInterface>,
}

pub fn handler(
    ctx: Context<WrappedBuyWithExtraFee>,
    swap_amount_type: SwapAmountType,
    amount: u64,
    other_amount_threshold: u64,
) -> Result<()> {
    let wrapper_authority_seeds: &[&[&[u8]]] = &[&[
        &b"wrapper_authority"[..],
        &[ctx.accounts.wrapper_authority.bump],
    ]];

    let context = CpiContext::new_with_signer(
        ctx.accounts.token_mill_program.to_account_info(),
        PermissionedSwap {
            config: ctx.accounts.config.to_account_info(),
            market: ctx.accounts.market.to_account_info(),
            market_authority: ctx.accounts.market_authority.to_account_info(),
            base_token_mint: ctx.accounts.base_token_mint.to_account_info(),
            quote_token_mint: ctx.accounts.quote_token_mint.to_account_info(),
            market_base_token_ata: ctx.accounts.market_base_token_ata.to_account_info(),
            market_quote_token_ata: ctx.accounts.market_quote_token_ata.to_account_info(),
            user_base_token_account: ctx.accounts.user_base_token_account.to_account_info(),
            user_quote_token_account: ctx.accounts.user_quote_token_account.to_account_info(),
            protocol_quote_token_ata: ctx.accounts.protocol_quote_token_ata.to_account_info(),
            referral_token_account: None,
            authority: ctx.accounts.wrapper_authority.to_account_info(),
            user: ctx.accounts.user.to_account_info(),
            base_token_program: ctx.accounts.base_token_program.to_account_info(),
            quote_token_program: ctx.accounts.quote_token_program.to_account_info(),
            event_authority: ctx.accounts.event_authority.to_account_info(),
            program: ctx.accounts.token_mill_program.to_account_info(),
        },
        wrapper_authority_seeds,
    );

    let (_, amount_out) = token_mill::cpi::permissioned_swap(
        context,
        SwapType::Buy,
        swap_amount_type,
        amount,
        other_amount_threshold,
    )?
    .get();

    // 2% extra fee
    let fee_amount = 200 * amount_out / 10_000;

    // Transfer the fee to the wrapper authority
    transfer_checked(
        CpiContext::new(
            ctx.accounts.base_token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.user_base_token_account.to_account_info(),
                mint: ctx.accounts.base_token_mint.to_account_info(),
                to: ctx
                    .accounts
                    .wrapper_authority_base_token_ata
                    .to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        fee_amount,
        ctx.accounts.base_token_mint.decimals,
    )?;

    Ok(())
}
