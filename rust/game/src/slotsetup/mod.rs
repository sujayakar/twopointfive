pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{setup_gun, setup_rows, slot_setup_open, slot_setup_left, slot_setup_shut, load_keys, light_keys, slot_setup_takes, clocked, lapsed, load_steps, step_row, draw_press, slot_setup_keys};
