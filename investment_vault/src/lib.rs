#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String};

mod events;
mod token;
mod types;

pub use types::VaultKey;

#[contract]
pub struct InvestmentVault;

#[contractimpl]
impl InvestmentVault {
    pub fn initialize(env: Env, admin: Address, usdc_sac: Address, registry: Address) {
        if env.storage().instance().has(&VaultKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&VaultKey::Admin, &admin);
        env.storage().instance().set(&VaultKey::UsdcSac, &usdc_sac);
        env.storage().instance().set(&VaultKey::Registry, &registry);
        env.storage().persistent().set(&VaultKey::TotalShares, &0i128);
        env.storage().persistent().set(&VaultKey::TotalInvestments, &0i128);
    }

    // SEP-41 token interface
    pub fn balance(env: Env, account: Address) -> i128 {
        token::balance(&env, &account)
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        token::transfer(&env, &from, &to, amount);
    }

    pub fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();
        token::approve(&env, &from, &spender, amount, expiration_ledger);
    }

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        token::allowance(&env, &from, &spender)
    }

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();
        token::transfer_from(&env, &spender, &from, &to, amount);
    }

    pub fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();
        token::burn(&env, &from, amount);
    }

    pub fn decimals(_env: Env) -> u32 {
        token::decimals()
    }

    pub fn name(env: Env) -> String {
        token::name(&env)
    }

    pub fn symbol(env: Env) -> String {
        token::symbol(&env)
    }

    pub fn total_supply(env: Env) -> i128 {
        token::total_shares(&env)
    }

    pub fn total_assets(env: Env) -> i128 {
        let usdc_sac: Address = env.storage().instance().get(&VaultKey::UsdcSac).unwrap();
        let liquid = soroban_sdk::token::TokenClient::new(&env, &usdc_sac)
            .balance(&env.current_contract_address());
        let investments: i128 = env
            .storage()
            .persistent()
            .get(&VaultKey::TotalInvestments)
            .unwrap_or(0);
        liquid + investments
    }

    pub fn convert_to_shares(env: Env, usdc_amount: i128) -> i128 {
        let total_assets = Self::total_assets(env.clone());
        let total_shares = token::total_shares(&env);
        if total_shares == 0 {
            usdc_amount
        } else {
            usdc_amount * total_shares / total_assets
        }
    }

    pub fn convert_to_assets(env: Env, shares_amount: i128) -> i128 {
        let total_assets = Self::total_assets(env.clone());
        let total_shares = token::total_shares(&env);
        if total_shares == 0 {
            0
        } else {
            shares_amount * total_assets / total_shares
        }
    }

    pub fn deposit(env: Env, from: Address, usdc_amount: i128) -> i128 {
        from.require_auth();
        if usdc_amount <= 0 {
            panic!("deposit must be positive");
        }

        // Compute shares BEFORE the transfer so total_assets reflects pre-deposit state
        let shares = Self::convert_to_shares(env.clone(), usdc_amount);

        let usdc_sac: Address = env.storage().instance().get(&VaultKey::UsdcSac).unwrap();
        soroban_sdk::token::TokenClient::new(&env, &usdc_sac)
            .transfer(&from, &env.current_contract_address(), &usdc_amount);

        token::mint(&env, &from, shares);

        events::deposit(&env, &from, usdc_amount, shares);

        shares
    }

    pub fn withdraw(env: Env, from: Address, shares_amount: i128) -> i128 {
        from.require_auth();
        if shares_amount <= 0 {
            panic!("shares must be positive");
        }

        let usdc_returned = Self::convert_to_assets(env.clone(), shares_amount);

        let usdc_sac: Address = env.storage().instance().get(&VaultKey::UsdcSac).unwrap();
        let liquid = soroban_sdk::token::TokenClient::new(&env, &usdc_sac)
            .balance(&env.current_contract_address());

        if usdc_returned > liquid {
            panic!("insufficient liquid USDC");
        }

        token::burn(&env, &from, shares_amount);
        soroban_sdk::token::TokenClient::new(&env, &usdc_sac)
            .transfer(&env.current_contract_address(), &from, &usdc_returned);

        events::withdraw(&env, &from, shares_amount, usdc_returned);

        usdc_returned
    }
}

#[cfg(test)]
mod test;
