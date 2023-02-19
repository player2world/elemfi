use crate::state::*;
use anchor_lang::prelude::*;

pub fn process_close_strategy(ctx: Context<CloseStrategy>) -> Result<()> {
    ctx.accounts.strategy.emit_closed_event();
    Ok(())
}

#[derive(Accounts)]
pub struct CloseStrategy<'info> {
    #[account(has_one = approver)]
    pub realm: Account<'info, Realm>,
    pub approver: Signer<'info>,

    #[account(has_one = realm)]
    pub vault: Account<'info, Vault>,

    #[account(mut, has_one = vault, close = rent_collector)]
    pub strategy: Account<'info, Strategy>,

    /// CHECK: OK
    #[account(mut)]
    pub rent_collector: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
