pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{b, sticky_visit, sticky_visit_set, sticky_issue, cam_aim_point, cam_solve, cam_vel, fire_cam, settle, update_stickies, flat, cam_lens, sticky_live, sticky_eyes, can_take, cam_near, hands_free, cam_to_take, sticky_rows, take_cam, sticky_flying, sticky_left};
