use crate::located::Located;
use anchor_lang::prelude::*;

#[account]
pub struct Obligation {
    pub realm: Pubkey,
    pub vault: Pubkey,
    pub rent_collector: Pubkey,
    pub destination: Pubkey,
    pub burnt_amount: u64,
    pub pending_amount: u64,
    pub created_ts: u32,
}

impl Obligation {
    pub const PREFIX: &'static [u8] = b"elemfi-obligation";
    pub const LEN: usize = 156; // 8+32+32+32+32+8+8+4
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ObligationCreatedData {
    pub realm: Pubkey,
    pub vault: Pubkey,
    pub rent_collector: Pubkey,
    pub destination: Pubkey,
    pub burnt_amount: u64,
    pub pending_amount: u64,
    pub created_ts: u32,
}

#[event]
pub struct ObligationCreatedEvent {
    pub pubkey: Pubkey,
    pub data: ObligationCreatedData,
}

pub trait EmitObligationCreatedEvent {
    fn emit_created_event(&self);
}

impl<T> EmitObligationCreatedEvent for T
where
    T: Located<Obligation>,
{
    fn emit_created_event(&self) {
        emit!(ObligationCreatedEvent {
            pubkey: self.key(),
            data: ObligationCreatedData {
                realm: self.as_ref().realm,
                vault: self.as_ref().vault,
                rent_collector: self.as_ref().rent_collector,
                destination: self.as_ref().destination,
                burnt_amount: self.as_ref().burnt_amount,
                pending_amount: self.as_ref().pending_amount,
                created_ts: self.as_ref().created_ts,
            }
        })
    }
}

#[event]
pub struct ObligationClosedEvent {
    pub pubkey: Pubkey,
}

pub trait EmitObligationClosedEvent {
    fn emit_closed_event(&self);
}

impl<T> EmitObligationClosedEvent for T
where
    T: Located<Obligation>,
{
    fn emit_closed_event(&self) {
        emit!(ObligationClosedEvent { pubkey: self.key() })
    }
}
