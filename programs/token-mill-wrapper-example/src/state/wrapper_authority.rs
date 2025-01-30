use anchor_lang::prelude::*;

// Wrapper authority will sign the cpi call to the market and act as the market authority
#[account]
#[derive(InitSpace)]
pub struct WrapperAuthority {
    pub bump: u8,
}

impl WrapperAuthority {
    pub fn initialize(&mut self, bump: u8) {
        self.bump = bump;
    }
}
