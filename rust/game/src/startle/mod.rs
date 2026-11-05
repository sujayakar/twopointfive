pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{blank, give_startle, startle_sight, startle_gun_down, startle_torch, startled, startle_left, startle_running, startle_script, choose, runs};
