#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::Address as _,
    token::StellarAssetClient,
    Address, Env,
};

struct TestSetup {
    env: Env,
    admin: Address,
    vault_client: InvestmentVaultClient<'static>,
    usdc_sac: Address,
    registry: Address,
}

fn setup() -> TestSetup {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let registry = Address::generate(&env);

    // Create mock USDC Stellar Asset Contract
    let usdc_admin = Address::generate(&env);
    let usdc_sac = env.register_stellar_asset_contract_v2(usdc_admin.clone()).address();

    let contract_id = env.register(InvestmentVault, ());
    let vault_client = InvestmentVaultClient::new(&env, &contract_id);
    vault_client.initialize(&admin, &usdc_sac, &registry);

    TestSetup {
        env,
        admin,
        vault_client,
        usdc_sac,
        registry,
    }
}

fn mint_usdc(env: &Env, usdc_sac: &Address, to: &Address, amount: i128) {
    let asset_client = StellarAssetClient::new(env, usdc_sac);
    asset_client.mint(to, &amount);
}

#[test]
fn test_first_deposit_mints_1_to_1_shares() {
    let s = setup();
    let investor = Address::generate(&s.env);
    mint_usdc(&s.env, &s.usdc_sac, &investor, 1_000_0000000i128);

    let shares = s.vault_client.deposit(&investor, &1_000_0000000i128);

    assert_eq!(shares, 1_000_0000000i128); // 1:1 on first deposit
    assert_eq!(s.vault_client.balance(&investor), 1_000_0000000i128);
    assert_eq!(s.vault_client.total_supply(), 1_000_0000000i128);
}

#[test]
fn test_deposit_proportional_after_first() {
    let s = setup();
    let investor1 = Address::generate(&s.env);
    let investor2 = Address::generate(&s.env);
    mint_usdc(&s.env, &s.usdc_sac, &investor1, 1_000_0000000i128);
    mint_usdc(&s.env, &s.usdc_sac, &investor2, 1_000_0000000i128);

    s.vault_client.deposit(&investor1, &1_000_0000000i128);
    let shares2 = s.vault_client.deposit(&investor2, &1_000_0000000i128);

    // Same USDC amount, same total assets → same shares
    assert_eq!(shares2, 1_000_0000000i128);
}

#[test]
fn test_withdraw_returns_usdc() {
    let s = setup();
    let investor = Address::generate(&s.env);
    mint_usdc(&s.env, &s.usdc_sac, &investor, 1_000_0000000i128);

    let shares = s.vault_client.deposit(&investor, &1_000_0000000i128);
    let returned = s.vault_client.withdraw(&investor, &shares);

    assert_eq!(returned, 1_000_0000000i128);
    assert_eq!(s.vault_client.balance(&investor), 0);
}

#[test]
fn test_total_assets_after_deposit() {
    let s = setup();
    let investor = Address::generate(&s.env);
    mint_usdc(&s.env, &s.usdc_sac, &investor, 500_0000000i128);
    s.vault_client.deposit(&investor, &500_0000000i128);
    assert_eq!(s.vault_client.total_assets(), 500_0000000i128);
}

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(InvestmentVault, ());
    let client = InvestmentVaultClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let usdc = Address::generate(&env);
    let registry = Address::generate(&env);
    client.initialize(&admin, &usdc, &registry);
}
