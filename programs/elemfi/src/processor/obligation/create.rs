use crate::{native_token::ID as NATIVE_TOKEN_ID, state::*};
use anchor_lang::{prelude::*, solana_program::clock};
use anchor_spl::{
    metadata::MetadataAccount,
    token::{accessor::mint as token_mint, burn, Burn, Token, TokenAccount},
};

pub fn process_create_obligation(
    ctx: Context<CreateObligation>,
    amount: u64,
    created_ts: i64,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    assert!(created_ts > clock::Clock::get()?.unix_timestamp);

    let destination = &ctx.remaining_accounts[0];
    if ctx.accounts.vault.underlying_token == NATIVE_TOKEN_ID {
        assert_eq!(*destination.owner, ctx.accounts.system_program.key());
    } else {
        assert_eq!(*destination.owner, ctx.accounts.token_program.key());
        assert_eq!(token_mint(destination)?, ctx.accounts.vault.underlying_token);
    }

    let pending_amount = ctx.accounts.vault.calc_amount_underlying_given_collateral(amount);
    ctx.accounts.obligation.set_inner(Obligation {
        realm: ctx.accounts.realm.key(),
        vault: ctx.accounts.vault.key(),
        rent_collector: ctx.accounts.payer.key(),
        destination: destination.key(),
        burnt_amount: amount,
        pending_amount,
        created_ts,
    });

    ctx.accounts.vault.pending_obligation_amount = ctx
        .accounts
        .vault
        .pending_obligation_amount
        .checked_add(pending_amount)
        .unwrap();
    ctx.accounts.vault.underlying_liquidity = ctx
        .accounts
        .vault
        .underlying_liquidity
        .checked_sub(pending_amount)
        .unwrap();
    ctx.accounts.vault.collateral_supply = ctx.accounts.vault.collateral_supply.checked_sub(amount).unwrap();
    ctx.accounts.vault.emit_updated_event();
    ctx.accounts.obligation.emit_created_event();

    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.collateral_token.to_account_info(),
                from: ctx.accounts.collateral_token_account.to_account_info(),
                authority: ctx.accounts.collateral_owner.to_account_info(),
            },
        )
        .with_signer(signer_seeds),
        amount,
    )
}

pub fn process_escrow_create_obligation<'a, 'b, 'c, 'info>(
    ctx: Context<'_, '_, '_, 'info, EscrowCreateObligation<'info>>,
    amount: u64,
    created_ts: i64,
) -> Result<()> {
    ctx.accounts.escrow.authority_seeds(|authority_seeds| {
        process_create_obligation(
            Context::new(
                &crate::ID,
                &mut ctx.accounts.create_obligation,
                &ctx.remaining_accounts,
                ctx.bumps.clone(),
            ),
            amount,
            created_ts,
            authority_seeds,
        )
    })
}

impl<'info> EscrowCreateObligation<'info> {
    pub fn validate(ctx: &Context<EscrowCreateObligation>) -> Result<()> {
        assert_eq!(
            ctx.accounts.nft_metadata.collection.as_ref().unwrap().key,
            ctx.accounts.create_obligation.realm.escrow_collection.unwrap()
        );
        assert_eq!(ctx.accounts.nft_token_account.owner, ctx.accounts.nft_owner.key());
        assert_eq!(ctx.accounts.nft_token_account.amount, 1);
        assert_eq!(ctx.accounts.nft_token_account.mint, ctx.accounts.nft_metadata.mint);
        assert_eq!(ctx.accounts.nft_token_account.mint, ctx.accounts.escrow.mint);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(created_ts: i64)]
pub struct CreateObligation<'info> {
    pub realm: Account<'info, Realm>,

    #[account(mut, has_one = realm, has_one = collateral_token)]
    pub vault: Account<'info, Vault>,
    /// CHECK: OK
    #[account(seeds = [Vault::AUTHORITY_PREFIX, &vault.key().to_bytes()], bump = vault.authority_bump)]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(
        init, rent_exempt = enforce,
        seeds = [Obligation::PREFIX, &vault.key().to_bytes(), &created_ts.to_le_bytes()],
        bump,
        payer = payer,
        space = Obligation::LEN,
    )]
    pub obligation: Account<'info, Obligation>,

    pub collateral_owner: Signer<'info>,
    /// CHECK: OK
    #[account(mut)]
    pub collateral_token: UncheckedAccount<'info>,
    /// CHECK: OK
    pub collateral_token_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EscrowCreateObligation<'info> {
    pub create_obligation: CreateObligation<'info>,

    pub escrow: Account<'info, Escrow>,

    pub nft_owner: Signer<'info>,
    pub nft_token_account: Box<Account<'info, TokenAccount>>,
    pub nft_metadata: Box<Account<'info, MetadataAccount>>,
}
