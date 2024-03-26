use anchor_lang::prelude::*;
use crate::state::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeArgs {
    pub admin: Pubkey,
    pub treasury: Pubkey,
    pub fee: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 84,
        seeds = [b"global"],
        bump
    )]
    global: Account<'info, Global>, 
    system_program: Program<'info, System>,
}

impl Initialize<'_> {
    fn validate(&self, _args: &InitializeArgs) -> Result<()> {
        // fee and accs checked in invariant
        Ok(())
    }

    #[access_control(ctx.accounts.validate(&args))]
    pub fn handle(ctx: Context<Self>, args: InitializeArgs) -> Result<()> {
        let InitializeArgs { admin, treasury, fee } = args;
        let global = &mut ctx.accounts.global;

        global.admin = admin;
        global.treasury = treasury;
        global.fee = fee;
        global.program_state = ProgramState::Running;

        global.invariant()
    }
}
