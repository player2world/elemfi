pub mod located;
pub mod processor;
pub mod state;

use anchor_lang::prelude::*;
use processor::*;

#[cfg(feature = "development")]
declare_id!("5mYaD6wtPDpS4psJ7Nc69zkGeKcRaJhGrSSCX6uUaw3A");
#[cfg(feature = "default")]
declare_id!("E1eMFiZrCBjA2KqpTSbysK56aShTgU9TLmh4wXLmv8hS");

pub mod native_token {
    crate::declare_id!("So11111111111111111111111111111111111111112");
    pub const DECIMALS: u8 = 9;
}

#[program]
pub mod elemfi {
    use super::*;

    pub fn create_realm(ctx: Context<CreateRealm>, delegator: Pubkey, approver: Pubkey) -> Result<()> {
        process_create_realm(ctx, delegator, approver)
    }

    #[access_control(CreateVault::validate(&ctx))]
    pub fn create_vault(
        ctx: Context<CreateVault>,
        collateral_max_supply: u64,
        collateral_min_amount: u64,
        collateral_max_amount: u64,
        underlying_liquidity: u64,
        escrow_collection: Option<Pubkey>,
    ) -> Result<()> {
        process_create_vault(
            ctx,
            collateral_max_supply,
            collateral_min_amount,
            collateral_max_amount,
            underlying_liquidity,
            escrow_collection,
        )
    }

    pub fn create_strategy(
        ctx: Context<CreateStrategy>,
        authority: Pubkey,
        utilized_amount: u64,
        utilization_max_amount: u64,
    ) -> Result<()> {
        process_create_strategy(ctx, authority, utilized_amount, utilization_max_amount)
    }

    pub fn close_strategy(ctx: Context<CloseStrategy>) -> Result<()> {
        process_close_strategy(ctx)
    }

    pub fn borrow<'a, 'b, 'c, 'info>(ctx: Context<'_, '_, '_, 'info, Borrow<'info>>, amount: u64) -> Result<()> {
        process_borrow(ctx, amount)
    }

    pub fn repay<'a, 'b, 'c, 'info>(ctx: Context<'_, '_, '_, 'info, Repay<'info>>, amount: u64) -> Result<()> {
        process_repay(ctx, amount)
    }

    pub fn close_obligation<'a, 'b, 'c, 'info>(ctx: Context<'_, '_, '_, 'info, CloseObligation<'info>>) -> Result<()> {
        process_close_obligation(ctx)
    }

    pub fn create_obligation(ctx: Context<CreateObligation>, amount: u64, created_ts: i64) -> Result<()> {
        process_create_obligation(ctx, amount, created_ts, &[])
    }

    #[access_control(Deposit::validate(&ctx))]
    pub fn deposit<'a, 'b, 'c, 'info>(ctx: Context<'_, '_, '_, 'info, Deposit<'info>>, amount: u64) -> Result<()> {
        process_deposit(ctx, amount, &[])
    }

    #[access_control(CreateEscrow::validate(&ctx))]
    pub fn create_escrow(ctx: Context<CreateEscrow>) -> Result<()> {
        process_create_escrow(ctx)
    }

    pub fn escrow_create_obligation<'a, 'b, 'c, 'info>(
        ctx: Context<'_, '_, '_, 'info, EscrowCreateObligation<'info>>,
        amount: u64,
        created_ts: i64,
    ) -> Result<()> {
        process_escrow_create_obligation(ctx, amount, created_ts)
    }

    #[access_control(EscrowDeposit::validate(&ctx))]
    pub fn escrow_deposit<'a, 'b, 'c, 'info>(
        ctx: Context<'_, '_, '_, 'info, EscrowDeposit<'info>>,
        amount: u64,
    ) -> Result<()> {
        process_escrow_deposit(ctx, amount)
    }
}
