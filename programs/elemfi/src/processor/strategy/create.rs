use crate::state::*;
use anchor_lang::prelude::*;

pub fn process_create_strategy(
    ctx: Context<CreateStrategy>,
    authority: Pubkey,
    utilized_amount: u64,
    utilization_max_amount: u64,
) -> Result<()> {
    ctx.accounts.strategy.set_inner(Strategy {
        realm: ctx.accounts.realm.key(),
        vault: ctx.accounts.vault.key(),
        authority,
        utilized_amount,
        utilization_max_amount,
    });
    ctx.accounts.strategy.emit_created_event();
    Ok(())
}

#[derive(Accounts)]
#[instruction(authority: Pubkey)]
pub struct CreateStrategy<'info> {
    #[account(has_one = approver)]
    pub realm: Account<'info, Realm>,
    pub approver: Signer<'info>,

    #[account(has_one = realm)]
    pub vault: Account<'info, Vault>,

    #[account(
        init, rent_exempt = enforce,
        seeds = [Strategy::PREFIX, &vault.key().to_bytes(), &authority.to_bytes()],
        bump,
        payer = payer,
        space = Strategy::LEN,
    )]
    pub strategy: Account<'info, Strategy>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
