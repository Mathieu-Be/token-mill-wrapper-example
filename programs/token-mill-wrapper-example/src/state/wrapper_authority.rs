use anchor_lang::prelude::*;

// Wrapper swap authority will sign the cpi call to the market and act as the swap authority
#[account]
#[derive(InitSpace)]
pub struct WrapperSwapAuthority {
    pub bump: u8,
}

impl WrapperSwapAuthority {
    pub fn initialize(&mut self, bump: u8) {
        self.bump = bump;
    }
}
