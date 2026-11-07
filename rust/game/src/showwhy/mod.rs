pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{blank, fresh, state_of, flat, show_why_stats, show_why_log, reset_show_why, note, speaking, say, sign, masked, mate_nearer, line_side, line_said_by, waits, stands_free, man_ahead, stacks, show_why_tick, owe, show_why_of, show_why_summary};
