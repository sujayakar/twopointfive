pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{runner_lines, bark, flat, chest_of, head_of, ours, spoken, reacts, mate_of, foe_at, foes, by_picture, known, close_foe, ripe, lost, start, runner_break, rect_of, outside, line_to, flood, route, path_len, pick_mate, way_barred, drop, to_out, to_low, plan_out, tell, pose, runner_tick, step, gone, drive, runner_script, runner_owns, runner_fled, runner_bare, runner_nobody, runner_mute, runner_of, runner_summary};
