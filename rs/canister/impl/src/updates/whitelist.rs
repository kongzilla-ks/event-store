use candid::Principal;
use ic_cdk::api::caller;
use ic_cdk_macros::update;
use ic_cdk::trap;

use crate::state;
use event_store_canister::{WhitelistAction, WhitelistedPrincipals};

#[update]
pub fn update_push_whitelist(principals: Vec<Principal>, action: WhitelistAction) {
    // Check if caller is an admin
    if !state::read(|s| s.is_caller_admin()) {
        trap("Caller is not authorized to modify whitelist");
    }

    state::mutate(|s| {
        for principal in principals {
            match action {
                WhitelistAction::Add => s.add_push_whitelist_principal(principal),
                WhitelistAction::Remove => s.remove_push_whitelist_principal(&principal),
            };
        }
    })
}

#[update]
pub fn update_read_whitelist(principals: Vec<Principal>, action: WhitelistAction) {
    // Check if caller is an admin
    if !state::read(|s| s.is_caller_admin()) {
        trap("Caller is not authorized to modify whitelist");
    }

    state::mutate(|s| {
        for principal in principals {
            match action {
                WhitelistAction::Add => s.add_read_whitelist_principal(principal),
                WhitelistAction::Remove => s.remove_read_whitelist_principal(&principal),
            };
        }
    })
}

#[update]
pub fn update_admin_whitelist(principals: Vec<Principal>, action: WhitelistAction) {
    // Check if caller is an admin
    if !state::read(|s| s.is_caller_admin()) {
        trap("Caller is not authorized to modify whitelist");
    }

    state::mutate(|s| {
        for principal in principals {
            match action {
                WhitelistAction::Add => s.add_admin_whitelist_principal(principal),
                WhitelistAction::Remove => s.remove_admin_whitelist_principal(&principal),
            };
        }
    })
} 