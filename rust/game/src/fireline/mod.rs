pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{live, fire_line_tick, lane_of, in_shape, in_lane, along, cross_at, door_at, in_doorway, walk_of, in_enemy_line, way_round, end, hold, fire_line_feet, fire_line_live};
