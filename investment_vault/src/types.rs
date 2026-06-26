use soroban_sdk::{contracttype, Address};

#[contracttype]
pub enum VaultKey {
    UsdcSac,
    Registry,
    TotalInvestments,
    ProjectInvestment(u32),
    /// Global yield-per-share accumulator, scaled by YIELD_SCALE (#125).
    YieldPerShareAccum,
    /// Per-shareholder checkpoint: yield-per-share value at last claim (#125).
    YieldDebt(Address),
}
