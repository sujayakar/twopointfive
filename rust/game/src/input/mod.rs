pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{input_detached, input_new, input_mods_up, input_mod_up, input_focus_game, input_down, input_hit, input_lmb, input_rmb, input_lmb_hit, input_rmb_hit, input_mmb, input_mmb_hit, input_end_frame, input_end_tick};
