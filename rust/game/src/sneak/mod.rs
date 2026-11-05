pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{clamp, save_sneak, leaf_moved, flat, note, sneak_live, mate, sneaking, sneak_low, sneak_post, sneak_step, sneak_coming, sneak_word, eyes_for, cpu_lit, lit_at, eye_for, stand_in, read, exposure, exposure_of, eye_rate, eye_gains, lit_seen, cross_cost, lit_cell, sneak_walk};
pub mod p_02;
pub use p_02::{track, sneak_tick, sneak_follow, sneak_way, cue, sneak_summary};
