pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{state_of, smooth, wrap, free, sam_look_noticed, reach, pick, sam_look_man, slip_of, slips, sam_slip_man, slip_now, head_of, sam_look_step};
