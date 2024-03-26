use anchor_lang::prelude::*;

#[account]
pub struct TradeOffer {
    pub owner: Pubkey,
    pub offer_mint: Pubkey,
    pub offer_amount: u64,
    pub request_mint: Pubkey,
    pub request_amount: u64,
    // size = 8 + 32 + 32 + 8 + 32 + 8 = 120
}

impl TradeOffer {
    //pub const SIZE = 120;
    pub fn invariant() -> Result<()> {
        Ok(())
    }
}
