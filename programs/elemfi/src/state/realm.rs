use anchor_lang::prelude::*;

#[account]
pub struct Realm {
    /// authority
    pub authority: Pubkey,
    /// offchain bot
    pub delegator: Pubkey,
    /// approver has power to add new router
    pub approver: Pubkey,
    /// escrow NFT collection
    pub escrow_collection: Option<Pubkey>,
}
