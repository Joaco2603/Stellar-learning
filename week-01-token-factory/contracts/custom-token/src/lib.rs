#![no_std]

mod errors;
mod storage;

use errors::TokenError;
use soroban_sdk::{contract, contractimpl, Address, Env, String};
use storage::{DataKey, TokenMetadata};

#[contract]
pub struct CustomToken;

#[contractimpl]
impl CustomToken {
    pub fn initialize(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
        decimals: u32,
    ) -> Result<(), TokenError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(TokenError::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(
            &DataKey::Metadata,
            &TokenMetadata {
                name,
                symbol,
                decimals,
            },
        );
        Ok(())
    }

    pub fn mint(env: Env, caller: Address, to: Address, amount: i128) -> Result<(), TokenError> {
        caller.require_auth();
        storage::require_admin(&env, &caller)?;
        if amount <= 0 {
            return Err(TokenError::InvalidAmount);
        }
        let new_balance = storage::balance(&env, &to) + amount;
        storage::set_balance(&env, &to, new_balance);
        Ok(())
    }

    pub fn balance_of(env: Env, owner: Address) -> i128 {
        storage::balance(&env, &owner)
    }

    /// Reto 9: transfer con `from.require_auth()` y chequeo de balance.
    pub fn transfer(
        _env: Env,
        _from: Address,
        _to: Address,
        _amount: i128,
    ) -> Result<(), TokenError> {
        Err(TokenError::NotInitialized)
    }
}

mod test;
