use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;
pub mod instructions;
pub mod state;

declare_id!("C4at2ajRabPt67goQuTvXpTKvb262j4MrHG4yYLExUNH");

#[program]
pub mod v1 {
    use super::*;

    pub fn create_offer(ctx: Context<CreateOffer>, args: CreateOfferArgs) -> Result<()> {
        CreateOffer::handle(ctx,args)
    }

    pub fn cancel_offer(ctx: Context<CancelOffer>) -> Result<()> {
        CancelOffer::handle(ctx)
    }

    pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
        TakeOffer::handle(ctx)
    }
}
