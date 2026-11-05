pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{whistle_lines, mem, glance_toward, whistle_mem, blower, whistle_ready_in, whistle_key, try_whistle, mouth_of, can_answer, resolve, hear_whistle, pick, whistle_soft, whistle_patrol_tick, save_whistle, load_whistle, reset_whistle, whistle_summary};
