pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{launch_vel, ring_solve, arc_solve, ring_aim_point, fire_ring, update_rings, bounce_off, spend, clack, ring_hits_character, stagger_guard, stagger_lines, ring_boxes, ring_summary};
