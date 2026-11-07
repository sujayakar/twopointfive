pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{fresh, note, live, tube_vis, tube_sig, is_mate, loud_t, nvg_place_light, read_place, nvg_tick, nvg_dark, nvg_worn, nvg_down, nvg_black, nvg_sight, nvg_state, nvg_summary};
