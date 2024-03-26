use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::token;


use crate::state::*;
use crate::error::TradeOfferError;

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    taker: Signer<'info>,
    #[account( seeds = [b"global"], bump)]
    global: Account<'info, Global>, 
    #[account(
        mut,
        seeds = [b"tradeoffer", offer.owner.key().as_ref()],
        has_one = offer_mint,
        has_one = request_mint,
        has_one = owner,
        close = owner,
        bump
    )]
    offer: Account<'info, TradeOffer>,
    offer_mint: Account<'info, Mint>,
    #[account(
        mut,
        token::mint = offer_mint,
        token::authority = offer,
        close = owner,
    )]
    offer_escrow: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = offer_mint,
        associated_token::authority = taker,
    )]
    taker_offer_token: Account<'info, TokenAccount>,
    request_mint: Account<'info, Mint>,
    #[account(
        mut,
        token::authority = taker,
        token::mint = request_mint
    )]
    taker_request_token: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = request_mint,
        associated_token::authority = owner,
    )]
    owner_request_token: Account<'info, TokenAccount>,
    /// CHECK: is ok
    #[account(mut)]
    owner: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

impl TakeOffer<'_> {
    fn validate(&self) -> Result<()> {
        let Self {
            taker, global, offer, ..
        } = self;

        if !global.can_take_offer() {
            err!(TradeOfferError::FunctionDisabled)
        } else { Ok(()) }
    }

    #[access_control(ctx.accounts.validate())]
    pub fn handle(ctx: Context<Self>) -> Result<()> {
        let offer = &mut ctx.accounts.offer;
        let global = &ctx.accounts.global;

        // transfer requested tokens from taker to owner
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.taker_request_token.to_account_info(),
                    to: ctx.accounts.owner_request_token.to_account_info(),
                    authority: ctx.accounts.taker.to_account_info(),
                }), offer.request_amount
        )?;

        // transfer offered tokens from escrow to taker
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.offer_escrow.to_account_info(),
                    to: ctx.accounts.taker_offer_token.to_account_info(),
                    authority: offer.to_account_info(),
                },
                &[&[b"tradeoffer", &offer.key().as_ref(), &[ctx.bumps.offer]],]
                ), offer.offer_amount
        )?;
        global.invariant()
        // done, close offer account

    }

}
