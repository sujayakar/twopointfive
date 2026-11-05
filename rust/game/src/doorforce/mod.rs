pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{live, state_of, man_of, flat, log, defender, shut, behind, door_between, door_hiding, learn, door_force_hear, door_force_own_boot, door_force_tick, drop, round_for, door_force, kick_step, door_force_belief, door_force_on, door_force_saw, door_force_reset, door_force_summary};
