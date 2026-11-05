pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{frag_stats_reset, hash01, through_glass, burst_blocked, burst_line, exposure, without_leaves, leaves_for, exposed_to, walls_for, flat, same_side, down_asked, lying, lying_shows, blast};
pub mod p_02;
pub use p_02::{blast_raw, detonate_frag, frag_fuse, books_of, frag_reset, come_down, roll_on, rest_of_throw, rest_y, frag_rest_guess, rest_of, shoulders, open_leaves, leaf_dist, leg_by_leaf, flee_spot, open_way, in_mouth, flee_by_door};
pub mod p_03;
pub use p_03::{taken_by, flee_mates, frag_tick, frag_flee, frag_running, own_bang, own_noise, sam_cue, frag_marks, frag_step_off, defenders_of, deal_carriers, frag_carriers, frag_throw_tick, aim_spots, wind_up, frag_in_fist, arm_round, free, frag_ready, frag_throw_at, frag_winding, frag_away, lands, hostage_near, own_clear};
