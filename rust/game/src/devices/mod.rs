pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{camera_rt_new, laser_rt_new, devices_note, devices_new, devices_pad_of, devices_overhear_said, devices_pad_near, devices_reset, alive, sensing, lens_gain, lens_dir, sweep_yaw, update_devices, update_camera, camera_suspect, camera_alarm, update_laser, body_breaks, laser_trip, device_down, send_to_look, control, chron_device, learn_code, overhear, spell, pad_door_name, pad_state, start_coding, deny_pad, stop_coding, update_coding, device_boxes, device_boxes_raw, seg_seg_closest};
