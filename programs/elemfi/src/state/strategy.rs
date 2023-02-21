use crate::located::Located;
use anchor_lang::prelude::*;

#[account]
pub struct Strategy {
    pub realm: Pubkey,
    pub vault: Pubkey,
    pub authority: Pubkey,
    pub utilized_amount: u64,
    pub utilization_max_amount: u64,
}

impl Strategy {
    pub const PREFIX: &'static [u8] = b"elemfi-strategy";
    pub const LEN: usize = 120; // 8+32+32+32+8+8
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct StrategyUpdatedData {
    pub utilized_amount: u64,
    pub utilization_max_amount: u64,
}

#[event]
pub struct StrategyUpdatedEvent {
    pub pubkey: Pubkey,
    pub data: StrategyUpdatedData,
}

pub trait EmitStrategyUpdatedEvent {
    fn emit_updated_event(&self);
}

impl<T> EmitStrategyUpdatedEvent for T
where
    T: Located<Strategy>,
{
    fn emit_updated_event(&self) {
        emit!(StrategyUpdatedEvent {
            pubkey: self.key(),
            data: StrategyUpdatedData {
                utilized_amount: self.as_ref().utilized_amount,
                utilization_max_amount: self.as_ref().utilization_max_amount,
            }
        })
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct StrategyCreatedData {
    pub realm: Pubkey,
    pub vault: Pubkey,
    pub authority: Pubkey,
    pub utilized_amount: u64,
    pub utilization_max_amount: u64,
}

#[event]
pub struct StrategyCreatedEvent {
    pub pubkey: Pubkey,
    pub data: StrategyCreatedData,
}

pub trait EmitStrategyCreatedEvent {
    fn emit_created_event(&self);
}

impl<T> EmitStrategyCreatedEvent for T
where
    T: Located<Strategy>,
{
    fn emit_created_event(&self) {
        emit!(StrategyCreatedEvent {
            pubkey: self.key(),
            data: StrategyCreatedData {
                realm: self.as_ref().realm,
                vault: self.as_ref().vault,
                authority: self.as_ref().authority,
                utilized_amount: self.as_ref().utilized_amount,
                utilization_max_amount: self.as_ref().utilization_max_amount,
            }
        })
    }
}

#[event]
pub struct StrategyClosedEvent {
    pub pubkey: Pubkey,
}

pub trait EmitStrategyClosedEvent {
    fn emit_closed_event(&self);
}

impl<T> EmitStrategyClosedEvent for T
where
    T: Located<Strategy>,
{
    fn emit_closed_event(&self) {
        emit!(StrategyClosedEvent { pubkey: self.key() })
    }
}
