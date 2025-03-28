use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

mod lifecycle;
mod queries;
mod updates;

pub use lifecycle::*;
pub use queries::*;
pub use updates::*;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum WhitelistAction {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
}

pub type Milliseconds = u64;
pub type TimestampMillis = u64;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct IdempotentEvent {
    pub idempotency_key: u128,
    pub name: String,
    pub timestamp: TimestampMillis,
    pub user: Option<Anonymizable>,
    pub source: Option<Anonymizable>,
    #[serde(with = "serde_bytes")]
    pub payload: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct IndexedEvent {
    pub index: u64,
    pub name: String,
    pub timestamp: TimestampMillis,
    pub user: Option<String>,
    pub source: Option<String>,
    #[serde(with = "serde_bytes")]
    pub payload: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum Anonymizable {
    Public(String),
    Anonymize(String),
}

impl Anonymizable {
    pub fn new(value: String, anonymize: bool) -> Anonymizable {
        if anonymize {
            Anonymizable::Anonymize(value)
        } else {
            Anonymizable::Public(value)
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Anonymizable::Public(s) => s,
            Anonymizable::Anonymize(s) => s,
        }
    }

    pub fn is_public(&self) -> bool {
        matches!(self, Anonymizable::Public(_))
    }
}

pub trait WhitelistApi {
    fn update_push_whitelist(principals: Vec<Principal>, action: WhitelistAction);
    fn update_read_whitelist(principals: Vec<Principal>, action: WhitelistAction);
    fn update_admin_whitelist(principals: Vec<Principal>, action: WhitelistAction);
}
