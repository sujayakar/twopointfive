pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{wrap, note, seg_rect, rect_dist, wall_react_new, wall_react_live, wall_react_fight, wall_react_def_of, wall_react_reset, wall_react_tick, wall_react_take, wall_react_kept, wall_react_sweep, wall_react_upsert, wall_react_unflood, wall_react_reflood, wall_react_head, wall_react_face_of, wall_react_sees, wall_react_ops_know, wall_react_man_knows, wall_react_heard, wall_react_notice, wall_react_drop_opening, wall_react_drop_openings, wall_react_book, wall_react_evidence, wall_react_through, wall_react_look, wall_react_posts, wall_react_char_of, wall_react_threat_for, wall_react_things, wall_react_asks, wall_react_beyond, wall_react_open_gone, wall_react_step_out, wall_react_open_through, wall_react_known, wall_react_spot, wall_react_run_for, wall_react_reconsider, wall_react_reconsider1, wall_react_shift, wall_react_op_move, wall_react_replan, wall_react_summary};
pub mod p_02;
pub use p_02::{wall_react_of, wall_react_tick_2, wall_react_reset_2, wall_react_look_2, wall_react_replan_2};
