use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::token;
use crate::state::*;
use crate::error::TradeOfferError;
use crate::events::{OfferEvent,OfferEventType};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateOfferArgs {
    pub offer_amount: u64,
    pub request_amount: u64,
}

#[derive(Accounts)]
pub struct CreateOffer<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account( seeds = [b"global"],
               has_one = treasury, 
               bump)]
    global: Account<'info, Global>, 
    #[account(
        init,
        payer = owner,
        space = 120,
        seeds = [b"tradeoffer", owner.key().as_ref()],
        bump
    )]
    offer: Account<'info, TradeOffer>, 
    #[account(mut)]
    owner_offer_token: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = offer_mint,
        associated_token::authority = offer,
    )]
    offer_escrow: Account<'info, TokenAccount>,
    offer_mint: Account<'info, Mint>,
    request_mint: Account<'info, Mint>,
    /// CHECK: checked in global
    #[account(mut)]
    treasury: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

impl CreateOffer<'_> {
    fn validate(&self, _args: &CreateOfferArgs) -> Result<()> {
        let Self {
            global,  ..
        } = self;

        // no need to check input amounts.

        msg!("ok");
        if !global.can_create_offer() {
            err!(TradeOfferError::FunctionDisabled)
        } else { Ok(()) }
    }

    #[access_control(ctx.accounts.validate(&args))]
    pub fn handle(ctx: Context<Self>, args: CreateOfferArgs) -> Result<()> {
        let offer = &mut ctx.accounts.offer;
        let global =  &ctx.accounts.global;
        let CreateOfferArgs { offer_amount, request_amount } = args;

        offer.owner = ctx.accounts.owner.key();
        offer.offer_mint = ctx.accounts.offer_mint.key();
        offer.offer_amount = offer_amount;
        offer.request_mint = ctx.accounts.request_mint.key();
        offer.request_amount = request_amount;

        // transfer from owner to escrow
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.owner_offer_token.to_account_info(),
                    to: ctx.accounts.offer_escrow.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                }), offer_amount
        )?;

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer { 
                    from: ctx.accounts.owner.to_account_info(), 
                    to: ctx.accounts.treasury.to_account_info() 
                }), 
            global.fee
        )?;
        emit!(OfferEvent{
            offer: offer.key(),
            event: OfferEventType::Create,
            offer_mint: offer.offer_mint,
            request_mint: offer.request_mint,
            offer_amount: offer.offer_amount,
            request_amount: offer.request_amount,
        });
        global.invariant()
    }
}
