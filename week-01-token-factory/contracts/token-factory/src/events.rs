use soroban_sdk::{Address, Env, String, Symbol};

pub(crate) fn asset_registered(
    env: &Env,
    asset_code: String,
    issuer: Address,
    kind: Symbol,
    contract_id: Address,
) {
    let topics = (Symbol::new(env, "asset_reg"), asset_code);
    env.events().publish(topics, (issuer, kind, contract_id));
}
