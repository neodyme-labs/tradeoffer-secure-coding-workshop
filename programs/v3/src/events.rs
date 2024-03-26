use anchor_lang::prelude::*;
use crate::state::*;

#[event]
pub struct AdminEvent {
    pub admin: Pubkey,
    pub treasury: Pubkey,
    pub state: ProgramState,
    pub fee: u64,
}


#[event]
pub struct OfferEvent {
    pub offer: Pubkey,
    pub event: OfferEventType,
    pub offer_mint: Pubkey,
    pub request_mint: Pubkey,
    pub offer_amount: u64,
    pub request_amount: u64,
}


#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum OfferEventType {
    Create,
    Cancel,
    Take
}
