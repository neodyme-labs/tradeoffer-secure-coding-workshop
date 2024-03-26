use anchor_lang::prelude::*;
use crate::error::TradeOfferError;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
pub enum ProgramState {
    /// Fully Operational
    Running,
    /// Fully Stopped, Except for Admin
    Stopped,
    /// No more CreateOffer. Only Cancel And Take
    WithdrawOnly,
    /// No more CreateOffer or Take. Only Cancel.
    CancelOnly,
    /// No more CreateOffer. Also, ProgramState Locked forever.
    Sunset,
}


#[account]
pub struct Global {
    pub admin: Pubkey,
    pub treasury: Pubkey,
    pub fee: u64,
    pub program_state: ProgramState,
    // size = 8 + 32 + 32 + 8  + State(1) = 80
}

impl Global {
    //pub const SIZE = 120;
    pub fn invariant(&self) -> Result<()> {
        require_gt!(10000, self.fee, TradeOfferError::ExcessiveFees);
        require_keys_neq!(self.treasury, self.admin, TradeOfferError::AuthoritySeparation);
        Ok(())
    }

    pub fn can_create_offer(&self) -> bool {
        match self.program_state {
            ProgramState::Running => true,
            _ => false
        }
    }

    pub fn can_cancel_offer(&self) -> bool {
        match self.program_state {
            ProgramState::Running | ProgramState::WithdrawOnly | ProgramState::CancelOnly | ProgramState::Sunset => true,
            _ => false
        }
    }

    pub fn can_take_offer(&self) -> bool {
        match self.program_state {
            ProgramState::Running | ProgramState::WithdrawOnly | ProgramState::Sunset => true,
            _ => false
        }
    }
}
