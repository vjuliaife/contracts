use soroban_sdk::{contracttype, Address, String};

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
    /// Insurance fund USDC balance (#135).
    InsuranceFund,
    /// Whether a project default claim has been paid out (#135).
    InsuranceClaimed(u32),
    /// Lifetime USDC deposited by an investor — used in portfolio analytics (#132).
    TotalDeposited(Address),
    /// Optional management fee in basis points, admin-set, hard-capped (#7).
    ManagementFeeBps,
    /// Recipient address for management fee transfers (#7).
    ManagementFeeRecipient,
    /// Whether secondary market trading of HBS is active (#126).
    TradingEnabled,
    /// Index of the oldest unprocessed redemption queue entry (#3).
    QueueHead,
    /// Next free index in the redemption queue (#3).
    QueueTail,
    /// A queued redemption claim by index (#3).
    QueueEntry(u64),
}

/// Metadata returned for DEX listing and secondary market integration (#126).
#[contracttype]
#[derive(Clone, Debug)]
pub struct HBSTokenInfo {
    /// Human-readable token name.
    pub name: String,
    /// Ticker symbol.
    pub symbol: String,
    /// Number of decimal places (7 for USDC-parity denominations).
    pub decimals: u32,
    /// Whether the admin has enabled secondary trading.
    pub trading_enabled: bool,
}

/// A pending withdrawal claim created when vault liquidity is insufficient (#3).
/// Shares are burned immediately at enqueue; this records the fixed USDC owed.
#[contracttype]
pub struct QueuedClaim {
    /// Address that will receive the USDC when liquidity is available.
    pub from: Address,
    /// USDC amount owed, fixed at the share price when the withdrawal was requested.
    pub usdc_owed: i128,
}

/// On-chain portfolio snapshot for a single investor (#132).
#[contracttype]
pub struct PortfolioInfo {
    /// HBS shares currently held.
    pub shares: i128,
    /// Current USDC redemption value of those shares.
    pub usdc_value: i128,
    /// Unclaimed yield in USDC.
    pub claimable_yield: i128,
    /// Shares as a fraction of total supply, in basis points (0-10 000).
    pub share_of_pool_bps: i128,
    /// Lifetime USDC deposited by this investor.
    pub total_deposited: i128,
}
