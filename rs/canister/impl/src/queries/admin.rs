use candid::Principal;
use ic_cdk_macros::query;

use crate::state;

#[query]
pub fn get_admin_whitelist() -> Vec<Principal> {
    state::read(|s| {
        // Check if caller is an admin
        if !s.is_caller_admin() {
            return vec![];
        }
        
        // Return the admin whitelist
        s.whitelisted_principals().push
    })
} 