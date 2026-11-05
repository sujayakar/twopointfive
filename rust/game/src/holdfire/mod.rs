pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{state_of, lead_of, mate, living, hold_fire_found, working, hold_fire_lifted, lift, hold_fire_shot, hold_fire_order, hold_fire_silent, hold_fire_loud, hold_fire_drill, held, holds, hold_fire_holds, hold_fire_on_man, hold_fire_glass, hold_fire_glass_held, hold_fire_reset, hold_fire_summary};
