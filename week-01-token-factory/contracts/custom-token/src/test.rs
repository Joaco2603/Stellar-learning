#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn mint_increases_balance() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let id = env.register(CustomToken, ());
    let client = CustomTokenClient::new(&env, &id);
    client
        .initialize(
            &admin,
            &String::from_str(&env, "Learning Coin"),
            &String::from_str(&env, "LEARN"),
            &7u32,
        )
        .unwrap();
    client.mint(&admin, &user, &1000).unwrap();
    assert_eq!(client.balance_of(&user), 1000);
}

#[test]
#[ignore = "Reto 9: implementar transfer y quitar ignore"]
fn transfer_moves_balance() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let id = env.register(CustomToken, ());
    let client = CustomTokenClient::new(&env, &id);
    client
        .initialize(
            &admin,
            &String::from_str(&env, "Learning Coin"),
            &String::from_str(&env, "LEARN"),
            &7u32,
        )
        .unwrap();
    client.mint(&admin, &alice, &500).unwrap();
    client.transfer(&alice, &bob, &200).unwrap();
    assert_eq!(client.balance_of(&alice), 300);
    assert_eq!(client.balance_of(&bob), 200);
}
