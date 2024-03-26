use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::token;

declare_id!("HTnAwbxeHFSCYuGLXqjU7u4pnTBoM1C4U9gkRtiAdYsU");
//const TREASURY: Pubkey = pubkey!("95REKqaMNA1XVCfugsozD4cFnQENCUyu31PdY8FDxULa");

#[program]
pub mod v0 {
    use super::*;

    pub fn create_offer(ctx: Context<CreateOffer>, offer_amount: u64, request_amount: u64) -> Result<()> {
        let offer = &mut ctx.accounts.offer;

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
        )
    }

    pub fn cancel_offer(ctx: Context<CancelOffer>) -> Result<()> {
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

    pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
        let offer = &mut ctx.accounts.offer;

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
        )
        // done, close offer account
    }
}


#[derive(Accounts)]
pub struct CreateOffer<'info> {
    #[account(mut)]
    owner: Signer<'info>,
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
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}


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

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    taker: Signer<'info>,
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

#[account]
pub struct TradeOffer {
    pub owner: Pubkey,
    pub offer_mint: Pubkey,
    pub offer_amount: u64,
    pub request_mint: Pubkey,
    pub request_amount: u64,
    // size = 8 + 32 + 32 + 8 + 32 + 8 = 120
}
