use crate::{native_token::ID as NATIVE_TOKEN_ID, state::*};
use anchor_lang::{prelude::*, system_program};
use anchor_spl::token::{self, accessor::mint as token_mint, Token};

pub fn process_close_obligation<'a, 'b, 'c, 'info>(
    ctx: Context<'_, '_, '_, 'info, CloseObligation<'info>>,
) -> Result<()> {
    ctx.accounts.vault.pending_obligation_amount = ctx
        .accounts
        .vault
        .pending_obligation_amount
        .checked_sub(ctx.accounts.obligation.pending_amount)
        .unwrap();
    ctx.accounts.vault.emit_updated_event();
    ctx.accounts.obligation.emit_closed_event();

    ctx.accounts.vault.authority_seeds(|authority_seeds| {
        if ctx.accounts.vault.underlying_token == NATIVE_TOKEN_ID {
            let vault_native_account = &ctx.remaining_accounts[0];
            assert_eq!(vault_native_account.key(), ctx.accounts.vault_authority.key());
            system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: vault_native_account.clone(),
                        to: ctx.accounts.destination.to_account_info(),
                    },
                )
                .with_signer(authority_seeds),
                ctx.accounts.obligation.pending_amount,
            )
        } else {
            let vault_token_account = &ctx.remaining_accounts[0];
            assert_eq!(token_mint(vault_token_account)?, ctx.accounts.vault.underlying_token);
            let token_program = &ctx.remaining_accounts[1];
            assert_eq!(token_program.key(), Token::id());
            token::transfer(
                CpiContext::new(
                    token_program.clone(),
                    token::Transfer {
                        from: vault_token_account.clone(),
                        to: ctx.accounts.destination.to_account_info(),
                        authority: ctx.accounts.vault_authority.to_account_info(),
                    },
                )
                .with_signer(authority_seeds),
                ctx.accounts.obligation.pending_amount,
            )
        }
    })
}

#[derive(Accounts)]
#[instruction(created_ts: i64)]
pub struct CloseObligation<'info> {
    #[account(has_one = authority)]
    pub realm: Account<'info, Realm>,
    pub authority: Signer<'info>,

    #[account(mut, has_one = realm)]
    pub vault: Account<'info, Vault>,
    /// CHECK: OK
    #[account(seeds = [Vault::AUTHORITY_PREFIX, &vault.key().to_bytes()], bump = vault.authority_bump)]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(mut, has_one = rent_collector, close = rent_collector)]
    pub obligation: Account<'info, Obligation>,

    /// CHECK: OK
    #[account(mut)]
    pub destination: UncheckedAccount<'info>,

    /// CHECK: OK
    #[account(mut)]
    pub rent_collector: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
