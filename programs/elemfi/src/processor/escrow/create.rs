use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::{metadata::MetadataAccount, token::TokenAccount};

pub fn process_create_escrow(ctx: Context<CreateEscrow>) -> Result<()> {
    ctx.accounts.escrow.set_inner(Escrow {
        mint: ctx.accounts.nft_token_account.mint,
        authority_bump: *ctx.bumps.get("escrow_authority").unwrap(),
    });
    Ok(())
}

impl<'info> CreateEscrow<'info> {
    pub fn validate(ctx: &Context<CreateEscrow>) -> Result<()> {
        assert_eq!(ctx.accounts.nft_token_account.amount, 1);
        assert_eq!(ctx.accounts.nft_token_account.mint, ctx.accounts.nft_metadata.mint);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(
        init, rent_exempt = enforce,
        seeds = [Escrow::PREFIX, &nft_token_account.mint.to_bytes()],
        bump,
        payer = payer,
        space = Escrow::LEN,
    )]
    pub escrow: Account<'info, Escrow>,
    /// CHECK: OK
    #[account(seeds = [Escrow::AUTHORITY_PREFIX, &nft_token_account.mint.to_bytes()], bump)]
    pub escrow_authority: UncheckedAccount<'info>,

    pub nft_token_account: Account<'info, TokenAccount>,
    pub nft_metadata: Account<'info, MetadataAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
