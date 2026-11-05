pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{morale_lines, bark, flat, chest_of, head_of, ours, rescued, reacts, mind, in_the_dark, has_mate, band_of, mix_now, training_of, leader_of, folds, nerved, stand, roused, standing, dread, may_break, rung_now, morale_shock, morale_man_down, saw_leader_fall, morale_blast, morale_reset, feed, begin, end, seen_by, thrown, clear_run, away_from, back_goal, side_goal, round, run_dry, bait, pose, morale_tick};
pub mod p_02;
pub use p_02::{step, drive, morale_script, morale_owns, dread_of, shaken_of, rung_of, morale_of, morale_summary, morale_roster, kept, morale_keep, morale_from_query, morale_release, n3};
