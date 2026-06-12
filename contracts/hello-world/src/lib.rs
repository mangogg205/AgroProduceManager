#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, Map};

#[contract]
pub struct AgroProduceContract;

#[contractimpl]
impl AgroProduceContract {
    /// Initialize empty storage for produce records
    pub fn init(env: Env) {
        let produce: Map<Symbol, Symbol> = Map::new(&env);
        env.storage().instance().set(&Symbol::short("produce"), &produce);
    }

    /// Add or update a produce record
    /// Example: set_produce("carrot01", "Carrot - 100kg - Organic")
    pub fn set_produce(env: Env, produce_id: Symbol, details: Symbol) {
        let mut produce: Map<Symbol, Symbol> = env
            .storage()
            .instance()
            .get(&Symbol::short("produce"))
            .unwrap_or(Map::new(&env));

        produce.set(produce_id.clone(), details);
        env.storage().instance().set(&Symbol::short("produce"), &produce);
    }

    /// Retrieve produce details by ID
    pub fn get_produce(env: Env, produce_id: Symbol) -> Option<Symbol> {
        let produce: Map<Symbol, Symbol> = env
            .storage()
            .instance()
            .get(&Symbol::short("produce"))
            .unwrap_or(Map::new(&env));

        produce.get(produce_id)
    }
}