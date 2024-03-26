
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::token;
use crate::state::*;


#[derive(Accounts)]
pub struct CancelOffer<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"tradeoffer", owner.key().as_ref()],
        has_one = owner,
        has_one = offer_mint,
        close = owner,
        bump,
    )]
    offer: Account<'info, TradeOffer>,
    offer_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = offer_mint,
        associated_token::authority = owner,
    )]
    owner_offer_token: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = offer_mint,
        token::authority = offer,
        close = owner
    )]
    offer_escrow: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}


impl CancelOffer<'_> {
    pub fn handle(ctx: Context<Self>) -> Result<()> {
        let offer = &mut ctx.accounts.offer;

        // transfer from escrow back to owner
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.offer_escrow.to_account_info(),
                    to: ctx.accounts.owner_offer_token.to_account_info(),
                    authority: offer.to_account_info(),
                },
                &[&[b"tradeoffer", offer.key().as_ref(), &[ctx.bumps.offer]],]
                ), offer.offer_amount
        )
    }
}
