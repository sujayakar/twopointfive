pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod b_01;
pub use b_01::{hash_u, live, thick, near, burst_at, blast_look_tick, blast_look_tick_raw, run_of, charge_boxes, holed, flash_of, lum, nudge, warm_of, afterglow, blast_sparks, flash_under, can_at, door_charge_at, door_blast, burst_of, emit, scale, wall_blast, stun_sparks};
