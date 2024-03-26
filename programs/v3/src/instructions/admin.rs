use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::TradeOfferError;
use crate::events::AdminEvent;

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
    fn validate(&self, _args: &AdminArgs) -> Result<()> {
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
            AdminAction::SetAdmin(new_admin) => {
                global.admin = new_admin
            },
            AdminAction::SetFee(new_fee) => {
                global.fee = new_fee
            },
            AdminAction::SetTreasury(new_treasury) => {
                global.treasury = new_treasury
            }
            AdminAction::SetProgramState(new_program_state) => {
                global.program_state = new_program_state
            }
        }

        emit!(AdminEvent{
            admin: global.admin,
            treasury: global.treasury,
            state: global.program_state,
            fee: global.fee,
        });

        global.invariant()
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum AdminAction {
    SetAdmin(Pubkey),
    SetFee(u64),
    SetTreasury(Pubkey),
    SetProgramState(ProgramState),
}
