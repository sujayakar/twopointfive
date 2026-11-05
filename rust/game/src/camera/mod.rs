pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{follow_camera_set_pin, follow_camera_update, follow_camera_step, follow_camera_frame, follow_camera_focus, follow_camera_kick_in, follow_camera_kick_out, follow_camera_draw_in, follow_camera_draw_out, follow_camera_rebuild, follow_camera_auto_yaw, follow_camera_ease_yaw, follow_camera_zoom, follow_camera_rotate, follow_camera_project, follow_camera_ray, follow_camera_ground_point, follow_camera_planar_basis};
