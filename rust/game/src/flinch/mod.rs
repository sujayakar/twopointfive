pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{staff, fight_man, run_on, flinch_inside, flinch_round, free, flinch_scale, green, mate_by, flinch_tick, flinch_under};
