use crate::located::Located;
use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub realm: Pubkey,
    pub authority_bump: u8,
    pub token_decimals: u8,
    pub collateral_token: Pubkey,
    pub collateral_supply: u64,
    pub collateral_max_supply: u64,
    pub collateral_min_amount: u64,
    pub collateral_max_amount: u64,
    pub underlying_token: Pubkey,
    pub underlying_liquidity: u64,
    pub pending_obligation_amount: u64,
    pub pending_obligations: u32,
    /// escrow NFT collection
    pub escrow_collection: Option<Pubkey>,
}

impl Vault {
    pub const AUTHORITY_PREFIX: &'static [u8] = b"elemfi-vault-authority";

    pub fn calc_amount_collateral_given_underlying(&self, amount: u64) -> u64 {
        if self.collateral_supply == 0 {
            amount
        } else {
            u64::try_from(
                (amount as u128)
                    .checked_mul(self.collateral_supply as u128)
                    .unwrap()
                    .checked_div(self.underlying_liquidity as u128)
                    .unwrap(),
            )
            .unwrap()
        }
    }

    pub fn calc_amount_underlying_given_collateral(&self, amount: u64) -> u64 {
        u64::try_from(
            (amount as u128)
                .checked_mul(self.underlying_liquidity as u128)
                .unwrap()
                .checked_div(self.collateral_supply as u128)
                .unwrap(),
        )
        .unwrap()
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct VaultUpdatedData {
    pub collateral_supply: u64,
    pub collateral_max_supply: u64,
    pub underlying_liquidity: u64,
    pub pending_obligation_amount: u64,
    pub pending_obligations: u32,
}

#[event]
pub struct VaultUpdatedEvent {
    pub pubkey: Pubkey,
    pub data: VaultUpdatedData,
}

pub trait EmitVaultUpdatedEvent {
    fn emit_updated_event(&self);
}

impl<T> EmitVaultUpdatedEvent for T
where
    T: Located<Vault>,
{
    fn emit_updated_event(&self) {
        emit!(VaultUpdatedEvent {
            pubkey: self.key(),
            data: VaultUpdatedData {
                collateral_supply: self.as_ref().collateral_supply,
                collateral_max_supply: self.as_ref().collateral_max_supply,
                underlying_liquidity: self.as_ref().underlying_liquidity,
                pending_obligation_amount: self.as_ref().pending_obligation_amount,
                pending_obligations: self.as_ref().pending_obligations,
            }
        })
    }
}

pub trait VaultAuthoritySeeds {
    fn authority_seeds<R, F: FnOnce(&[&[&[u8]]]) -> R>(&self, f: F) -> R;
}

impl<T> VaultAuthoritySeeds for T
where
    T: Located<Vault>,
{
    fn authority_seeds<R, F: FnOnce(&[&[&[u8]]]) -> R>(&self, f: F) -> R {
        f(&[&[
            Vault::AUTHORITY_PREFIX,
            &self.key().to_bytes(),
            &[self.as_ref().authority_bump],
        ]])
    }
}

#[test]
pub fn err_calc_amount_collateral_given_underlying() {
    let amount = u64::MAX;
    let collateral_supply = 10u64;
    let underlying_liquidity = 5u64;
    let amount_in_u128 = (amount as u128)
        .checked_mul(collateral_supply as u128)
        .unwrap()
        .checked_div(underlying_liquidity as u128)
        .unwrap();
    match u64::try_from(amount_in_u128) {
        Ok(_) => {
            assert!(false)
        }
        Err(_) => {
            assert!(true)
        }
    }
}

#[test]
pub fn ok_calc_amount_collateral_given_underlying() {
    let amount = u64::MAX;
    let collateral_supply = 5u64;
    let underlying_liquidity = 10u64;
    let amount_in_u128 = (amount as u128)
        .checked_mul(collateral_supply as u128)
        .unwrap()
        .checked_div(underlying_liquidity as u128)
        .unwrap();
    match u64::try_from(amount_in_u128) {
        Ok(_) => {
            assert!(true)
        }
        Err(_) => {
            assert!(false)
        }
    }
}
