use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::token;
use crate::state::*;
use crate::error::TradeOfferError;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AdminArgs {
    action: AdminAction
}

#[derive(Accounts)]
pub struct Admin<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"global"],
        has_one = admin,
        bump
    )]
    global: Account<'info, Global>, 
}

impl Admin<'_> {
    fn validate(&self, args: &AdminArgs) -> Result<()> {
        // Sunset? Program locked
        match self.global.program_state {
            ProgramState::Sunset => err!(TradeOfferError::Sunset),
            _ => Ok(())
        }
    }

    #[access_control(ctx.accounts.validate(&args))]
    pub fn handle(ctx: Context<Self>, args: AdminArgs) -> Result<()> {
        let AdminArgs { action } = args;
        let global = &mut ctx.accounts.global;

        match action {
            AdminAction::setAdmin(new_admin) => {
                global.admin = new_admin
            },
            AdminAction::setFee(new_fee) => {
                global.fee = new_fee
            },
            AdminAction::setTreasury(new_treasury) => {
                global.treasury = new_treasury
            }
            AdminAction::setProgramState(new_program_state) => {
                global.program_state = new_program_state
            }
        }

        global.invariant()
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum AdminAction {
    setAdmin(Pubkey),
    setFee(u64),
    setTreasury(Pubkey),
    setProgramState(ProgramState),
}
