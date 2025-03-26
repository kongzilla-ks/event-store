use candid::Principal;
use crate::{WhitelistAction, WhitelistApi};

// Dummy implementations that just panic
pub fn update_push_whitelist(principals: Vec<Principal>, action: WhitelistAction) {
    unimplemented!("API definition only")
}

pub fn update_read_whitelist(principals: Vec<Principal>, action: WhitelistAction) {
    unimplemented!("API definition only")
}

pub fn update_admin_whitelist(principals: Vec<Principal>, action: WhitelistAction) {
    unimplemented!("API definition only")
}
