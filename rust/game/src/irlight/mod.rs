pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{ir_flags, kit_of, lamp, pose, dark_at, fan_dark, skey, forget, fan_darks, shown_dark, drop, burn, sam_on, out, near_k, ir_near_of, ir_light_tick, ir_light_at, ir_on_man, ir_shows_at, ir_carried, ir_light_state, ir_light_summary};
