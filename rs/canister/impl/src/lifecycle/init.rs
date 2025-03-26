use crate::state;
use crate::state::State;
use event_store_canister::InitArgs;
use ic_cdk::init;
use std::time::Duration;

#[init]
fn init(args: InitArgs) {
    // Clone the whitelist to avoid moving it
    let whitelist = args.push_events_whitelist.clone();
    
    let mut state = State::new(
        args.push_events_whitelist.into_iter().collect(),
        args.read_events_whitelist.into_iter().collect(),
        args.time_granularity,
    );
    
    // All principals in push_events_whitelist are considered admins
    for principal in whitelist.iter() {
        state.add_admin_principal(*principal);
    }

    state::init(state);

    ic_cdk_timers::set_timer(Duration::ZERO, || {
        ic_cdk::spawn(async {
            let salt: [u8; 32] = ic_cdk::api::management_canister::main::raw_rand()
                .await
                .unwrap()
                .0
                .try_into()
                .unwrap();

            state::mutate(|s| s.set_salt(salt));
        })
    });
}
