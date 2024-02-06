use crate::Event;
use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct PushEventsArgs {
    pub events: Vec<Event>,
}
