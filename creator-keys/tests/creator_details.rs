//! Tests for the `get_creator_details` read-only view method (#11).

use creator_keys::{CreatorKeysContract, CreatorKeysContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_get_creator_details_returns_registered_creator() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let handle = String::from_str(&env, "alice");
    client.register_creator(&creator, &handle);

    let details = client.get_creator_details(&creator);
    assert!(details.is_registered);
    assert_eq!(details.creator, creator);
    assert_eq!(details.handle, handle);
    assert_eq!(details.supply, 0);
}

#[test]
fn test_get_creator_details_unregistered_returns_defaults() {
    let env = Env::default();

    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    let unknown = Address::generate(&env);
    let details = client.get_creator_details(&unknown);

    assert!(!details.is_registered);
    assert_eq!(details.creator, unknown);
    assert_eq!(details.handle, String::from_str(&env, ""));
    assert_eq!(details.supply, 0);
}

#[test]
fn test_get_creator_details_supply_reflects_buys() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let creator = Address::generate(&env);
    let buyer = Address::generate(&env);

    client.set_key_price(&admin, &100_i128);
    client.register_creator(&creator, &String::from_str(&env, "bob"));

    client.buy_key(&creator, &buyer, &100_i128);
    client.buy_key(&creator, &buyer, &100_i128);

    let details = client.get_creator_details(&creator);
    assert!(details.is_registered);
    assert_eq!(details.supply, 2);
}

#[test]
fn test_get_creator_details_is_read_only() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    client.register_creator(&creator, &String::from_str(&env, "carol"));

    // Multiple calls should not mutate state
    let d1 = client.get_creator_details(&creator);
    let d2 = client.get_creator_details(&creator);
    assert_eq!(d1.supply, d2.supply);
    assert_eq!(d1.is_registered, d2.is_registered);
}
