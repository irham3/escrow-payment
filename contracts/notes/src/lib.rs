#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    token, Address, Env, Symbol,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Buyer,
    Seller,
    Arbiter,
    Token,
    Amount,
    Status,
}

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum EscrowStatus {
    Pending = 0,
    Funded = 1,
    Released = 2,
    Refunded = 3,
    Disputed = 4,
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {

    pub fn initialize(
        env: Env,
        buyer: Address,
        seller: Address,
        arbiter: Address,
        token: Address,
        amount: i128,
    ) {
        if env.storage().instance().has(&DataKey::Status) {
            panic!("Contract already initialized");
        }

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        env.storage().instance().set(&DataKey::Buyer, &buyer);
        env.storage().instance().set(&DataKey::Seller, &seller);
        env.storage().instance().set(&DataKey::Arbiter, &arbiter);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Amount, &amount);
        env.storage().instance().set(&DataKey::Status, &EscrowStatus::Pending);

        env.events().publish(
            (Symbol::new(&env, "escrow"), Symbol::new(&env, "init")),
            (buyer, seller, amount),
        );
    }

    pub fn deposit(env: Env, from: Address) {
        from.require_auth();

        let buyer: Address = env.storage().instance().get(&DataKey::Buyer).unwrap();
        let status: EscrowStatus = env.storage().instance().get(&DataKey::Status).unwrap();

        if from != buyer {
            panic!("Only buyer can deposit");
        }
        if status != EscrowStatus::Pending {
            panic!("Deposit only allowed in Pending state");
        }

        let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let amount: i128 = env.storage().instance().get(&DataKey::Amount).unwrap();

        let client = token::Client::new(&env, &token_addr);
        client.transfer(&from, &env.current_contract_address(), &amount);

        env.storage().instance().set(&DataKey::Status, &EscrowStatus::Funded);

        env.events().publish(
            (Symbol::new(&env, "escrow"), Symbol::new(&env, "funded")),
            amount,
        );
    }

    pub fn release(env: Env, caller: Address) {
        caller.require_auth();

        let buyer: Address = env.storage().instance().get(&DataKey::Buyer).unwrap();
        let seller: Address = env.storage().instance().get(&DataKey::Seller).unwrap();
        let arbiter: Address = env.storage().instance().get(&DataKey::Arbiter).unwrap();
        let status: EscrowStatus = env.storage().instance().get(&DataKey::Status).unwrap();

        if caller != buyer && caller != arbiter {
            panic!("Only buyer or arbiter can release");
        }
        
        if status != EscrowStatus::Funded && status != EscrowStatus::Disputed {
            panic!("Release only allowed when Funded or Disputed");
        }

        let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let amount: i128 = env.storage().instance().get(&DataKey::Amount).unwrap();

        let client = token::Client::new(&env, &token_addr);
        client.transfer(&env.current_contract_address(), &seller, &amount);

        env.storage().instance().set(&DataKey::Status, &EscrowStatus::Released);

        env.events().publish(
            (Symbol::new(&env, "escrow"), Symbol::new(&env, "released")),
            seller,
        );
    }

    pub fn refund(env: Env, caller: Address) {
        caller.require_auth();

        let buyer: Address = env.storage().instance().get(&DataKey::Buyer).unwrap();
        let seller: Address = env.storage().instance().get(&DataKey::Seller).unwrap();
        let arbiter: Address = env.storage().instance().get(&DataKey::Arbiter).unwrap();
        let status: EscrowStatus = env.storage().instance().get(&DataKey::Status).unwrap();

        if caller != seller && caller != arbiter {
            panic!("Only seller or arbiter can refund");
        }

        if status != EscrowStatus::Funded && status != EscrowStatus::Disputed {
            panic!("Refund only allowed when Funded or Disputed");
        }

        let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let amount: i128 = env.storage().instance().get(&DataKey::Amount).unwrap();

        let client = token::Client::new(&env, &token_addr);
        client.transfer(&env.current_contract_address(), &buyer, &amount);

        env.storage().instance().set(&DataKey::Status, &EscrowStatus::Refunded);

        env.events().publish(
            (Symbol::new(&env, "escrow"), Symbol::new(&env, "refunded")),
            buyer,
        );
    }

    pub fn dispute(env: Env, caller: Address) {
        caller.require_auth();

        let buyer: Address = env.storage().instance().get(&DataKey::Buyer).unwrap();
        let seller: Address = env.storage().instance().get(&DataKey::Seller).unwrap();
        let status: EscrowStatus = env.storage().instance().get(&DataKey::Status).unwrap();

        if caller != buyer && caller != seller {
            panic!("Only buyer or seller can raise dispute");
        }
        if status != EscrowStatus::Funded {
            panic!("Dispute only allowed when Funded");
        }

        env.storage().instance().set(&DataKey::Status, &EscrowStatus::Disputed);

        env.events().publish(
            (Symbol::new(&env, "escrow"), Symbol::new(&env, "disputed")),
            caller,
        );
    }

    pub fn get_status(env: Env) -> EscrowStatus {
        env.storage().instance().get(&DataKey::Status).unwrap_or(EscrowStatus::Pending)
    }

    pub fn get_amount(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Amount).unwrap_or(0)
    }
}