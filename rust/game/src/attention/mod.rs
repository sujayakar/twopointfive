pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{mem, log, attention_lines, line, attention_of, off_cos, outside, attention_off_deg, attention_side_k, attention_aside, attention_ear_k, attention_noise_k, attention_step, attention_told, attention_again, attention_heard, attention_lend, able, secs_for, end, attention_tick, tell, attention_walk, attention_dwell, attention_marks, attention_way_store_get, attention_way_store_set, attention_way, attention_summary};
