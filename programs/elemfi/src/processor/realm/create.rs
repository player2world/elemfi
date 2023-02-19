use crate::state::*;
use anchor_lang::prelude::*;

pub fn process_create_realm(
    ctx: Context<CreateRealm>,
    delegator: Pubkey,
    approver: Pubkey,
    escrow_collection: Option<Pubkey>,
) -> Result<()> {
    ctx.accounts.realm.set_inner(Realm {
        authority: ctx.accounts.authority.key(),
        delegator,
        approver,
        escrow_collection,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct CreateRealm<'info> {
    #[account(zero, rent_exempt = enforce)]
    pub realm: Account<'info, Realm>,
    pub authority: Signer<'info>,
}
