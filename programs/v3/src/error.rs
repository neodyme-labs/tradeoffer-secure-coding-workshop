use anchor_lang::prelude::*;

#[error_code]
pub enum TradeOfferError {
    #[msg("Something went wrong")]
    GenericError,
    #[msg("Can't use the same account as admin and treasury")]
    AuthoritySeparation,
    #[msg("Fees are capped at 10k lamports")]
    ExcessiveFees,
    #[msg("Program is sunset. This action is unavailable")]
    Sunset,
    #[msg("This function is currently disabled.")]
    FunctionDisabled,
}
