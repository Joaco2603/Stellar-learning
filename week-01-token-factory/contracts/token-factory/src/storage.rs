use soroban_sdk::{contracttype, Address, Env, String, Symbol};

use super::errors::FactoryError;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    Asset(String),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AssetKind {
    Classic,
    Sac,
    CustomContract,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetRecord {
    pub issuer: Address,
    pub kind: AssetKind,
    pub contract_id: Address,
}

pub fn require_admin(env: &Env, caller: &Address) -> Result<(), FactoryError> {
    let admin: Address = env
        .storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(FactoryError::NotInitialized)?;
    if admin != *caller {
        return Err(FactoryError::Unauthorized);
    }
    Ok(())
}

pub fn read_asset(env: &Env, code: &String) -> Result<AssetRecord, FactoryError> {
    env.storage()
        .persistent()
        .get(&DataKey::Asset(code.clone()))
        .ok_or(FactoryError::AssetNotFound)
}

pub fn write_asset(env: &Env, code: &String, record: &AssetRecord) {
    env.storage().persistent().set(&DataKey::Asset(code.clone()), record);
}
