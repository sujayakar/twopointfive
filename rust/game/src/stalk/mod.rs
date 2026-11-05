pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{fresh, state_of, end, bearing_of, hands, free, shut, able, begin_m, out_m, stalk_begin_m, stalk_out_m, stalk_click_only, out_of, stalk_could, pressed, corner_gate, corner_man, duck, broke_duck, stalk_polish, stalk_step, dragged, into_keep, orbited, stalk_tell, stalk_wait, stalk_has, stalk_grab, stalk_key_can, stalk_key_took, stalk_on, stalk_reset, step_ease};
