use soroban_sdk::{contracttype, Address, Env, String};

use super::errors::TokenError;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Metadata,
    Balance(Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
}

pub fn require_admin(env: &Env, caller: &Address) -> Result<(), TokenError> {
    let admin: Address = env
        .storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(TokenError::NotInitialized)?;
    if admin != *caller {
        return Err(TokenError::Unauthorized);
    }
    Ok(())
}

pub fn balance(env: &Env, owner: &Address) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::Balance(owner.clone()))
        .unwrap_or(0)
}

pub fn set_balance(env: &Env, owner: &Address, amount: i128) {
    env.storage()
        .persistent()
        .set(&DataKey::Balance(owner.clone()), &amount);
}
