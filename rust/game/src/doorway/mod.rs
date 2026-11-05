pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{state_of, flat, ours, rel, in_opening, in_sweep, doorway_stand, doorway_lane, doorway_lane_price, doorway_off_lane, behind_leaf, rounds_leaves, leaf_stalled, doorway_file, door_ahead, friends, step_to, spot_free, jamb_spot, through, doorway_gate};
pub mod p_02;
pub use p_02::{off_threshold, doorway_in_way, doorway_held_by, doorway_held, doorway_clear_near};
