use soroban_sdk::{contracttype, Address, String};

/// Certification state for a green project (#130).
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum CertificationStatus {
    None      = 0,
    Pending   = 1,
    Certified = 2,
    Revoked   = 3,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct ProjectData {
    pub owner: Address,
    pub uri: String,
    pub credit_quality: u32,
    pub green_impact: u32,
    /// Unix timestamp (seconds) after which the project is considered mature (#127).
    /// 0 means no maturity date set.
    pub maturity_date: u64,
    /// Third-party certification state (#130).
    pub certification_status: CertificationStatus,
}

#[contracttype]
pub enum DataKey {
    Whitelister,
    ProjectCounter,
    Project(u32),
    Whitelist(Address),
}
