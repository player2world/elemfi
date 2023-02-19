use crate::located::Located;
use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub mint: Pubkey,
    pub authority_bump: u8,
}

impl Escrow {
    pub const PREFIX: &'static [u8] = b"elemfi-escrow";
    pub const LEN: usize = 41; // 8+32+1

    pub const AUTHORITY_PREFIX: &'static [u8] = b"elemfi-escrow-authority";
}

pub trait EscrowAuthoritySeeds {
    fn authority_seeds<R, F: FnOnce(&[&[&[u8]]]) -> R>(&self, f: F) -> R;
}

impl<T> EscrowAuthoritySeeds for T
where
    T: Located<Escrow>,
{
    fn authority_seeds<R, F: FnOnce(&[&[&[u8]]]) -> R>(&self, f: F) -> R {
        f(&[&[
            Escrow::AUTHORITY_PREFIX,
            &self.as_ref().mint.key().to_bytes(),
            &[self.as_ref().authority_bump],
        ]])
    }
}
