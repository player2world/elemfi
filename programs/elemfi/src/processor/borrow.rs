use crate::{native_token::ID as NATIVE_TOKEN_ID, state::*};
use anchor_lang::{prelude::*, system_program};
use anchor_spl::token::{
    self,
    accessor::{authority as token_account_owner, mint as token_mint},
    Token,
};

pub fn process_borrow<'a, 'b, 'c, 'info>(ctx: Context<'_, '_, '_, 'info, Borrow<'info>>, amount: u64) -> Result<()> {
    ctx.accounts.strategy.utilized_amount = ctx.accounts.strategy.utilized_amount.checked_add(amount).unwrap();
    assert!(ctx.accounts.strategy.utilized_amount <= ctx.accounts.strategy.utilization_max_amount);
    ctx.accounts.strategy.emit_updated_event();

    ctx.accounts.vault.authority_seeds(|authority_seeds| {
        if ctx.accounts.vault.underlying_token == NATIVE_TOKEN_ID {
            let strategy_native_account = &ctx.remaining_accounts[0];
            let vault_native_account = &ctx.remaining_accounts[1];
            let system_program = &ctx.remaining_accounts[2];
            assert_eq!(strategy_native_account.key(), ctx.accounts.strategy.authority);
            assert_eq!(system_program.key(), System::id());
            system_program::transfer(
                CpiContext::new(
                    system_program.clone(),
                    system_program::Transfer {
                        from: vault_native_account.clone(),
                        to: strategy_native_account.clone(),
                    },
                )
                .with_signer(authority_seeds),
                amount,
            )
        } else {
            let strategy_token_account = &ctx.remaining_accounts[0];
            let vault_token_account = &ctx.remaining_accounts[1];
            let token_program = &ctx.remaining_accounts[2];
            assert_eq!(
                token_account_owner(strategy_token_account)?,
                ctx.accounts.strategy.authority
            );
            assert_eq!(token_mint(vault_token_account)?, ctx.accounts.vault.underlying_token);
            assert_eq!(token_program.key(), Token::id());
            token::transfer(
                CpiContext::new(
                    token_program.clone(),
                    token::Transfer {
                        from: vault_token_account.clone(),
                        to: strategy_token_account.clone(),
                        authority: ctx.accounts.vault_authority.to_account_info(),
                    },
                )
                .with_signer(authority_seeds),
                amount,
            )
        }
    })
}

#[derive(Accounts)]
pub struct Borrow<'info> {
    #[account(has_one = authority)]
    pub realm: Account<'info, Realm>,
    pub authority: Signer<'info>,

    #[account(has_one = realm)]
    pub vault: Account<'info, Vault>,
    /// CHECK: OK
    #[account(seeds = [Vault::AUTHORITY_PREFIX, &vault.key().to_bytes()], bump = vault.authority_bump)]
    pub vault_authority: UncheckedAccount<'info>,

    // borrower is strategy authority
    #[account(mut, has_one = vault)]
    pub strategy: Account<'info, Strategy>,
}
