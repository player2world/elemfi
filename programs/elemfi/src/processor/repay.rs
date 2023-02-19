use crate::{native_token::ID as NATIVE_TOKEN_ID, state::*};
use anchor_lang::{prelude::*, system_program};
use anchor_spl::token::{
    self,
    accessor::{authority as token_account_owner, mint as token_mint},
    Token,
};

pub fn process_repay<'a, 'b, 'c, 'info>(ctx: Context<'_, '_, '_, 'info, Repay<'info>>, amount: u64) -> Result<()> {
    ctx.accounts.strategy.utilized_amount = ctx.accounts.strategy.utilized_amount.checked_sub(amount).unwrap();
    ctx.accounts.strategy.emit_updated_event();

    if ctx.accounts.vault.underlying_token == NATIVE_TOKEN_ID {
        let repayer_native_account = &ctx.remaining_accounts[0];
        let vault_native_account = &ctx.remaining_accounts[1];
        let system_program = &ctx.remaining_accounts[2];
        assert_eq!(vault_native_account.key(), ctx.accounts.vault_authority.key());
        assert_eq!(system_program.key(), System::id());
        system_program::transfer(
            CpiContext::new(
                system_program.clone(),
                system_program::Transfer {
                    from: repayer_native_account.clone(),
                    to: vault_native_account.clone(),
                },
            ),
            amount,
        )
    } else {
        let repayer_token_account = &ctx.remaining_accounts[0];
        let vault_token_account = &ctx.remaining_accounts[1];
        let token_program = &ctx.remaining_accounts[2];
        assert_eq!(token_mint(repayer_token_account)?, ctx.accounts.vault.underlying_token);
        assert_eq!(
            token_account_owner(vault_token_account)?,
            ctx.accounts.vault_authority.key()
        );
        assert_eq!(token_program.key(), Token::id());
        token::transfer(
            CpiContext::new(
                token_program.clone(),
                token::Transfer {
                    from: repayer_token_account.clone(),
                    to: vault_token_account.clone(),
                    authority: ctx.accounts.repayer.to_account_info(),
                },
            ),
            amount,
        )
    }
}

#[derive(Accounts)]
pub struct Repay<'info> {
    pub realm: Account<'info, Realm>,

    #[account(has_one = realm)]
    pub vault: Account<'info, Vault>,
    /// CHECK: OK
    #[account(seeds = [Vault::AUTHORITY_PREFIX, &vault.key().to_bytes()], bump = vault.authority_bump)]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(mut, has_one = vault)]
    pub strategy: Account<'info, Strategy>,

    // repayer can be anyone
    pub repayer: Signer<'info>,
}
