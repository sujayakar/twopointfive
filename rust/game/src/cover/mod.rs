pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{gaps_of};
pub mod p_02;
pub use p_02::{cover_new, cover_reset, cover_of, cover_kept_job, cover_ready, cover_build, cover_spare, cover_free, cover_read_rear, cover_walk_only, cover_walk_door, cover_eye_of, cover_sight_from, cover_line, cover_sight, cover_tick, cover_deal, cover_step_deal, cover_prune_steps, cover_step_spot, cover_step_of, cover_step_halt, cover_note, cover_drop, cover_release, cover_turn_from, cover_way_of, cover_turning, cover_turn_out, cover_unturn, cover_unkeep, cover_kept_of, cover_drop_all, cover_pose_for, cover_name_of, cover_has, cover_on_of, cover_words, cover_tail, cover_facing, cover_summary};
