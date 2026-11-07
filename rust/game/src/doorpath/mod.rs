pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{flat, gone, seg_seg, pt_seg, door_path_new, door_path_reset, door_path_guns, door_path_friend_on, door_path_after, door_path_leaf, door_path_leaves, door_path_free, door_path_open, door_path_round_leaves, door_path_way, door_path_lay, door_path_stand, door_path_fallen, door_path_move, door_path_total, door_path_point_at, door_path_advance, door_path_to_line, door_path_depth, door_path_may_go, door_path_sent, door_path_own, door_path_drop, door_path_tick, door_path_summary};
