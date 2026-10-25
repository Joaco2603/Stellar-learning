#![no_std]

mod errors;
mod events;
mod storage;

use errors::FactoryError;
use soroban_sdk::{contract, contractimpl, Address, Env, String, Symbol};
use storage::{AssetKind, AssetRecord, DataKey};

#[contract]
pub struct TokenFactory;

#[contractimpl]
impl TokenFactory {
    /// Reto 1: completar tests de doble inicialización.
    pub fn initialize(env: Env, admin: Address) -> Result<(), FactoryError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(FactoryError::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        Ok(())
    }

    pub fn admin(env: Env) -> Result<Address, FactoryError> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(FactoryError::NotInitialized)
    }

    /// Reto 3: registrar un activo clásico (metadata + issuer).
    /// Reto 4: emitir evento `asset_registered`.
    pub fn register_classic(
        env: Env,
        caller: Address,
        asset_code: String,
        issuer: Address,
    ) -> Result<(), FactoryError> {
        caller.require_auth();
        storage::require_admin(&env, &caller)?;

        if env
            .storage()
            .persistent()
            .has(&DataKey::Asset(asset_code.clone()))
        {
            return Err(FactoryError::AssetAlreadyRegistered);
        }

        let record = AssetRecord {
            issuer,
            kind: AssetKind::Classic,
            // Placeholder hasta que enlaces SAC o contrato custom (Reto 6).
            contract_id: env.current_contract_address(),
        };
        storage::write_asset(&env, &asset_code, &record);
        events::asset_registered(
            &env,
            asset_code,
            record.issuer.clone(),
            Symbol::new(&env, "classic"),
            record.contract_id,
        );
        Ok(())
    }

    /// Reto 5: implementar y testear.
    pub fn get_asset(env: Env, asset_code: String) -> Result<AssetRecord, FactoryError> {
        storage::read_asset(&env, &asset_code)
    }

    /// Reto 7: solo admin; kind = Sac | CustomContract.
    pub fn register_contract_asset(
        _env: Env,
        _caller: Address,
        _asset_code: String,
        _issuer: Address,
        _kind: AssetKind,
        _contract_id: Address,
    ) -> Result<(), FactoryError> {
        Err(FactoryError::NotInitialized)
    }
}

mod test;
