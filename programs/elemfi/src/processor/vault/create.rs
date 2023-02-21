use crate::{
    native_token::{DECIMALS as NATIVE_TOKEN_DECIMALS, ID as NATIVE_TOKEN_ID},
    state::*,
};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

pub fn process_create_vault(
    ctx: Context<CreateVault>,
    collateral_max_supply: u64,
    collateral_min_amount: u64,
    collateral_max_amount: u64,
    underlying_liquidity: u64,
    escrow_collection: Option<Pubkey>,
) -> Result<()> {
    let underlying_token = &ctx.remaining_accounts[0];
    let token_decimals = if underlying_token.key() == NATIVE_TOKEN_ID {
        NATIVE_TOKEN_DECIMALS
    } else {
        let mut data: &[u8] = &underlying_token.try_borrow_data()?;
        let mint = Mint::try_deserialize(&mut data)?;
        mint.decimals
    };
    assert_eq!(ctx.accounts.collateral_token.decimals, token_decimals);

    ctx.accounts.vault.set_inner(Vault {
        realm: ctx.accounts.realm.key(),
        authority_bump: *ctx.bumps.get("vault_authority").unwrap(),
        token_decimals,
        collateral_token: ctx.accounts.collateral_token.key(),
        collateral_supply: ctx.accounts.collateral_token.supply,
        collateral_max_supply,
        collateral_min_amount,
        collateral_max_amount,
        underlying_token: underlying_token.key(),
        underlying_liquidity,
        pending_obligation_amount: 0,
        pending_obligations: 0,
        escrow_collection,
    });
    Ok(())
}

impl<'info> CreateVault<'info> {
    pub fn validate(ctx: &Context<CreateVault>) -> Result<()> {
        assert_eq!(
            ctx.accounts.collateral_token.mint_authority.unwrap(),
            ctx.accounts.vault_authority.key()
        );
        assert!(ctx.accounts.collateral_token.freeze_authority.is_none());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(has_one = authority)]
    pub realm: Account<'info, Realm>,
    pub authority: Signer<'info>,

    #[account(zero, rent_exempt = enforce)]
    pub vault: Account<'info, Vault>,
    /// CHECK: OK
    #[account(seeds = [Vault::AUTHORITY_PREFIX, &vault.key().to_bytes()], bump)]
    pub vault_authority: UncheckedAccount<'info>,

    pub collateral_token: Account<'info, Mint>,
}
