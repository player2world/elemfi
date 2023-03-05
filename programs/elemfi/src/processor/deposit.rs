use crate::{native_token::ID as NATIVE_TOKEN_ID, state::*};
use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    metadata::MetadataAccount,
    token::{
        self,
        accessor::{amount as token_balance, authority as token_account_owner, mint as token_mint},
        mint_to, Mint, MintTo, Token, TokenAccount,
    },
};

pub fn process_deposit<'a, 'b, 'c, 'info>(
    ctx: Context<'_, '_, '_, 'info, Deposit<'info>>,
    amount: u64,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    if ctx.accounts.vault.underlying_token == NATIVE_TOKEN_ID {
        let underlying_native_account = &ctx.remaining_accounts[0];
        assert!(underlying_native_account.lamports() >= amount);
        let vault_native_account = &ctx.remaining_accounts[1];
        assert_eq!(vault_native_account.key(), ctx.accounts.vault_authority.key());
        let system_program = &ctx.remaining_accounts[2];
        assert_eq!(system_program.key(), System::id());
        system_program::transfer(
            CpiContext::new(
                system_program.clone(),
                system_program::Transfer {
                    from: underlying_native_account.clone(),
                    to: vault_native_account.clone(),
                },
            )
            .with_signer(signer_seeds),
            amount,
        )?;
    } else {
        let underlying_token_account = &ctx.remaining_accounts[0];
        assert!(token_balance(underlying_token_account)? >= amount);
        assert_eq!(
            token_mint(underlying_token_account)?,
            ctx.accounts.vault.underlying_token
        );
        let vault_token_account = &ctx.remaining_accounts[1];
        assert_eq!(
            token_account_owner(vault_token_account)?,
            ctx.accounts.vault_authority.key()
        );
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: underlying_token_account.clone(),
                    to: vault_token_account.clone(),
                    authority: ctx.accounts.underlying_token_owner.to_account_info(),
                },
            )
            .with_signer(signer_seeds),
            amount,
        )?;
    }

    let mint_amount = ctx.accounts.vault.calc_amount_collateral_given_underlying(amount);
    let post_collateral_amount = ctx
        .accounts
        .collateral_token_account
        .amount
        .checked_add(mint_amount)
        .unwrap();

    msg!("Post Collateral Amount: {}", post_collateral_amount);
    msg!("Collateral Min Amount: {}", ctx.accounts.vault.collateral_min_amount);
    msg!("Collateral Max Amount: {}", ctx.accounts.vault.collateral_max_amount);

    assert!(post_collateral_amount >= ctx.accounts.vault.collateral_min_amount);
    assert!(post_collateral_amount <= ctx.accounts.vault.collateral_max_amount);
    ctx.accounts.vault.underlying_liquidity = ctx.accounts.vault.underlying_liquidity.checked_add(amount).unwrap();
    ctx.accounts.vault.collateral_supply = ctx.accounts.vault.collateral_supply.checked_add(mint_amount).unwrap();
    assert!(ctx.accounts.vault.collateral_supply <= ctx.accounts.vault.collateral_max_supply);
    ctx.accounts.vault.emit_updated_event();

    ctx.accounts.vault.authority_seeds(|authority_seeds| {
        mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.collateral_token.to_account_info(),
                    to: ctx.accounts.collateral_token_account.to_account_info(),
                    authority: ctx.accounts.vault_authority.to_account_info(),
                },
            )
            .with_signer(authority_seeds),
            mint_amount,
        )
    })
}

pub fn process_escrow_deposit<'a, 'b, 'c, 'info>(
    ctx: Context<'_, '_, '_, 'info, EscrowDeposit<'info>>,
    amount: u64,
) -> Result<()> {
    if ctx.accounts.deposit.vault.underlying_token == NATIVE_TOKEN_ID {
        let underlying_native_account = &ctx.remaining_accounts[0];
        assert!(underlying_native_account.lamports() >= amount);
        let escrow_native_account = &ctx.remaining_accounts[1];
        assert_eq!(
            escrow_native_account.key(),
            ctx.accounts.deposit.underlying_token_owner.key()
        );
        let system_program = &ctx.remaining_accounts[4];
        assert_eq!(system_program.key(), System::id());
        system_program::transfer(
            CpiContext::new(
                system_program.clone(),
                system_program::Transfer {
                    from: underlying_native_account.clone(),
                    to: escrow_native_account.clone(),
                },
            ),
            amount,
        )?;
    } else {
        let underlying_token_account = &ctx.remaining_accounts[0];
        assert!(token_balance(underlying_token_account)? >= amount);
        assert_eq!(
            token_mint(underlying_token_account)?,
            ctx.accounts.deposit.vault.underlying_token
        );
        let escrow_token_account = &ctx.remaining_accounts[1];
        assert_eq!(
            token_account_owner(escrow_token_account)?,
            ctx.accounts.deposit.underlying_token_owner.key()
        );
        token::transfer(
            CpiContext::new(
                ctx.accounts.deposit.token_program.to_account_info(),
                token::Transfer {
                    from: underlying_token_account.clone(),
                    to: escrow_token_account.clone(),
                    authority: ctx.accounts.nft_owner.to_account_info(),
                },
            ),
            amount,
        )?;
    }
    ctx.accounts.escrow.authority_seeds(|authority_seeds| {
        process_deposit(
            Context::new(
                &crate::ID,
                &mut ctx.accounts.deposit,
                &ctx.remaining_accounts[2..],
                ctx.bumps.clone(),
            ),
            amount,
            authority_seeds,
        )
    })
}

impl<'info> Deposit<'info> {
    pub fn validate(ctx: &Context<Deposit>) -> Result<()> {
        assert!(ctx.accounts.vault.escrow_collection.is_none());
        Ok(())
    }
}

impl<'info> EscrowDeposit<'info> {
    pub fn validate(ctx: &Context<EscrowDeposit>) -> Result<()> {
        assert_eq!(
            ctx.accounts.nft_metadata.collection.as_ref().unwrap().key,
            ctx.accounts.deposit.vault.escrow_collection.unwrap()
        );
        assert_eq!(ctx.accounts.nft_account.owner, ctx.accounts.nft_owner.key());
        assert_eq!(ctx.accounts.nft_account.amount, 1);
        assert_eq!(ctx.accounts.nft_account.mint, ctx.accounts.nft_metadata.mint);
        assert_eq!(ctx.accounts.nft_account.mint, ctx.accounts.escrow.mint);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub realm: Account<'info, Realm>,

    #[account(mut, has_one = realm, has_one = collateral_token)]
    pub vault: Account<'info, Vault>,
    /// CHECK: OK
    #[account(seeds = [Vault::AUTHORITY_PREFIX, &vault.key().to_bytes()], bump = vault.authority_bump)]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub collateral_token: Account<'info, Mint>,
    /// CHECK: OK
    #[account(mut, constraint = collateral_token_account.owner == underlying_token_owner.key())]
    pub collateral_token_account: Account<'info, TokenAccount>,
    pub underlying_token_owner: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct EscrowDeposit<'info> {
    pub deposit: Deposit<'info>,

    pub escrow: Account<'info, Escrow>,

    pub nft_owner: Signer<'info>,
    pub nft_account: Box<Account<'info, TokenAccount>>,
    pub nft_metadata: Box<Account<'info, MetadataAccount>>,
}
