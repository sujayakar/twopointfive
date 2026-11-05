pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{squad40_here, tube_up, squad40_left, squad40_brought, squad40_of, squad40_word, squad40_stage, bodies, on_it, gun_on, shot_of, caught, plan, fire, turn_tick, row, notes, squad40_tick, squad40_plans, squad40_summary};
