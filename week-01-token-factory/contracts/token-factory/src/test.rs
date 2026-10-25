#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn setup() -> (Env, Address, TokenFactoryClient) {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let contract_id = env.register(TokenFactory, ());
    let client = TokenFactoryClient::new(&env, &contract_id);
    client.initialize(&admin).unwrap();
    (env, admin, client)
}

#[test]
fn initialize_sets_admin() {
    let (_env, admin, client) = setup();
    assert_eq!(client.admin(), Ok(admin));
}

#[test]
fn double_initialize_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let contract_id = env.register(TokenFactory, ());
    let client = TokenFactoryClient::new(&env, &contract_id);
    client.initialize(&admin).unwrap();
    assert_eq!(client.initialize(&admin), Err(Ok(FactoryError::AlreadyInitialized)));
}

#[test]
fn register_classic_as_admin_succeeds() {
    let (env, admin, client) = setup();
    let issuer = Address::generate(&env);
    let code = String::from_str(&env, "USDC");
    client.register_classic(&admin, &code, &issuer).unwrap();
    let record = client.get_asset(&code).unwrap();
    assert_eq!(record.issuer, issuer);
    assert_eq!(record.kind, storage::AssetKind::Classic);
}

#[test]
#[ignore = "Reto 2: implementar require_admin y quitar ignore"]
fn non_admin_cannot_register() {
    let (env, _admin, client) = setup();
    let attacker = Address::generate(&env);
    let issuer = Address::generate(&env);
    let code = String::from_str(&env, "HACK");
    let result = client.try_register_classic(&attacker, &code, &issuer);
    assert_eq!(result, Err(Ok(FactoryError::Unauthorized)));
}
