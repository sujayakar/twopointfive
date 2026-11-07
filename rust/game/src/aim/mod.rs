pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{aim_state_clear, move_cost, lamp_cost, feed_feet, feet_of, feed_target, track_guard, settle_time, aim_ready, feet_settled, guard_acquired, guard_terms, guard_round, hands_cone, aim_brake, note_bore, bore_fouled, track_player, player_aim, sam_aim, save_aim, load_aim, reset_aim, aim_summary};
