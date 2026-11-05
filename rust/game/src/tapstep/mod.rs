pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{state_of, tap_held, tap_hears, tap_reset, back_of, spot_of, step_secs, lift, plant, tap_tick, tap_off};
