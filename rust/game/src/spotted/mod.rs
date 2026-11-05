pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{flat, eye_of, in_line, lit_at, ours, has_them, live, spotted_mode, spotted_by, spotted_holds, danger_now, spotted_keen, spotted_feet, set, cue, ring, in_sams_line, hide, shot_line, cover, on_him, hands, spotted_take, spotted_gripped, fresh, spotted_tick, spotted_word, spotted_summary, spotted_of, save_spotted};
