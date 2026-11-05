pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{save_clear_line, mem_of, live_doing, chest_of, aim_of, flat, off_seg, sam_at_edge, leads, sam_moves, entering, shot_from, masked, spot_ok, find_spot, note, cue, end_it, doing, clear_line_live, clear_line_feet, cover_ok, clear_line_blocked, clear_line_summary, clear_line_of, bodies};
