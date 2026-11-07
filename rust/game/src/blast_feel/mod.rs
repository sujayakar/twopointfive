pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod b_01;
pub use b_01::{load_feel, hash_u, dice, live, thick, near, burst_at, holed, later, wall_report, wall_report_of, blast_feel_tick, blast_feel_tick_raw, kick_of, report, emit, fireball, ring_stage, ceiling};
