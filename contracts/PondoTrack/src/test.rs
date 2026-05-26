#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::{BudgetTrackerContract, BudgetTrackerContractClient};

#[test]
fn test_happy_path() {
    let env = Env::default();

    let contract_id = env.register(BudgetTrackerContract, ());
    let client = BudgetTrackerContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let contractor = Address::generate(&env);

    client.initialize(&admin);

    client.create_project(
        &admin,
        &1,
        &String::from_str(&env, "Road Repair"),
        &contractor,
        &1000,
    );

    client.release_payment(&admin, &1, &500);

    let project = client.get_project(&1);

    assert_eq!(project.released_amount, 500);
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn test_unauthorized() {
    let env = Env::default();

    let contract_id = env.register(BudgetTrackerContract, ());
    let client = BudgetTrackerContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let fake_admin = Address::generate(&env);
    let contractor = Address::generate(&env);

    client.initialize(&admin);

    client.create_project(
        &fake_admin,
        &1,
        &String::from_str(&env, "Bridge"),
        &contractor,
        &1000,
    );
}

#[test]
fn test_state_verification() {
    let env = Env::default();

    let contract_id = env.register(BudgetTrackerContract, ());
    let client = BudgetTrackerContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let contractor = Address::generate(&env);

    client.initialize(&admin);

    client.create_project(
        &admin,
        &2,
        &String::from_str(&env, "Drainage"),
        &contractor,
        &2000,
    );

    let project = client.get_project(&2);

    assert_eq!(project.allocated_amount, 2000);
}

#[test]
#[should_panic(expected = "Insufficient allocation")]
fn test_over_release() {
    let env = Env::default();

    let contract_id = env.register(BudgetTrackerContract, ());
    let client = BudgetTrackerContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let contractor = Address::generate(&env);

    client.initialize(&admin);

    client.create_project(
        &admin,
        &3,
        &String::from_str(&env, "Flood Wall"),
        &contractor,
        &1000,
    );

    client.release_payment(&admin, &3, &2000);
}

#[test]
fn test_multiple_payments() {
    let env = Env::default();

    let contract_id = env.register(BudgetTrackerContract, ());
    let client = BudgetTrackerContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let contractor = Address::generate(&env);

    client.initialize(&admin);

    client.create_project(
        &admin,
        &4,
        &String::from_str(&env, "School Roof"),
        &contractor,
        &3000,
    );

    client.release_payment(&admin, &4, &1000);
    client.release_payment(&admin, &4, &500);

    let project = client.get_project(&4);

    assert_eq!(project.released_amount, 1500);
}