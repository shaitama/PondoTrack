#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Symbol, String,
};

#[contracttype]
#[derive(Clone)]
pub struct Project {
    pub project_id: u32,
    pub name: String,
    pub contractor: Address,
    pub allocated_amount: i128,
    pub released_amount: i128,
    pub status: Symbol,
}

#[contracttype]
pub enum DataKey {
    Project(u32),
    Admin,
}

#[contract]
pub struct BudgetTrackerContract;

#[contractimpl]
impl BudgetTrackerContract {

    // Initialize contract admin
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    // Create a new LGU project allocation
    pub fn create_project(
        env: Env,
        admin: Address,
        project_id: u32,
        name: String,
        contractor: Address,
        allocated_amount: i128,
    ) {
        admin.require_auth();

        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();

        if admin != stored_admin {
            panic!("Unauthorized");
        }

        let project = Project {
            project_id,
            name,
            contractor,
            allocated_amount,
            released_amount: 0,
            status: symbol_short!("CREATED"),
        };

        env.storage()
            .persistent()
            .set(&DataKey::Project(project_id), &project);
    }

    // Release milestone payment
    pub fn release_payment(
        env: Env,
        admin: Address,
        project_id: u32,
        amount: i128,
    ) {
        admin.require_auth();

        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();

        if admin != stored_admin {
            panic!("Unauthorized");
        }

        let key = DataKey::Project(project_id);

        let mut project: Project = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap();

        if project.released_amount + amount > project.allocated_amount {
            panic!("Insufficient allocation");
        }

        project.released_amount += amount;
        project.status = symbol_short!("PAID");

        env.storage().persistent().set(&key, &project);
    }

    // Get project details
    pub fn get_project(env: Env, project_id: u32) -> Project {
        env.storage()
            .persistent()
            .get(&DataKey::Project(project_id))
            .unwrap()
    }
}