pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{switches_live, room_light, room_of_light, wall_ray, plate_on, plate_at, plate_near, fewest, build, wall_switches, geom_serial, wall_stands, light_switch_reset, switch_lit, plants, turn_secs, plant_broken, switch_let_go, switch_hand, flip_switch, hands_free, switch_near, switch_rows, switch_act, puts_back, light_switch_tick, switch_boxes};
