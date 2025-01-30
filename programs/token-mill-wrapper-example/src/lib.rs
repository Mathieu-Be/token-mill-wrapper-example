#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

mod instructions;
mod state;

use instructions::*;

use token_mill::{SwapAmountType, SwapType};

declare_id!("GyXseHdafYkKMCQmS6dJN1KNWe3Zok9pSGm7wjiYK9ob");

#[program]
pub mod token_mill_wrapper_example {
    use super::*;

    pub fn initialize_wrapper_authority(ctx: Context<InitializeWrapperAuthority>) -> Result<()> {
        instructions::initialize_wrapper_authority::handler(ctx)
    }

    pub fn simple_wrapped_swap(
        ctx: Context<SimpleWrappedSwap>,
        swap_type: SwapType,
        swap_amount_type: SwapAmountType,
        amount: u64,
        other_amount_threshold: u64,
    ) -> Result<()> {
        instructions::simple_wrapped_swap::handler(
            ctx,
            swap_type,
            swap_amount_type,
            amount,
            other_amount_threshold,
        )
    }

    pub fn graduate(ctx: Context<Graduate>) -> Result<()> {
        instructions::graduate::handler(ctx)
    }
}
