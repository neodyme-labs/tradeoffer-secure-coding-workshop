use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;
pub mod instructions;
pub mod state;
pub mod error;

declare_id!("D5AoRK9vLoeNhtmV8Brbj9pau1KAJA3Br8oeBQ2bjyML");

#[program]
pub mod v2 {
    use super::*;

    pub fn create_offer(ctx: Context<CreateOffer>, args: CreateOfferArgs) -> Result<()> {
        CreateOffer::handle(ctx, args)
    }

    pub fn cancel_offer(ctx: Context<CancelOffer>) -> Result<()> {
        CancelOffer::handle(ctx)
    }

    pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
        TakeOffer::handle(ctx)
    }
    pub fn initialize(ctx: Context<Initialize>, args: InitializeArgs) -> Result<()> {
        Initialize::handle(ctx, args)
    }
    pub fn admin(ctx: Context<Admin>, args: AdminArgs) -> Result<()> {
        Admin::handle(ctx, args)
    }
}
