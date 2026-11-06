pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod b_01;
pub use b_01::{live, near, burst_at, run_of, flash_of, lum, nudge, warm_of, blast_look_tick, blast_look_tick_raw, charge_boxes, afterglow, door_blast, burst_of, emit, wall_blast, stun_sparks, blast_sparks};
