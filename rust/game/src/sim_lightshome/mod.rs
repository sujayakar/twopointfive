pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{columns, fixture_new, fixture_on, fixture_broken, fixture_disabled_until, fixture_factor, fixture_hand_off, fixture_authored, fixture_base_intensity, fixture_stagger, mains_new, mains_active, mains_permanent, mains_until, mains_since, mains_restored_at, mains_fix_t, camera_machine_new, camera_machine_yaw, camera_machine_phase, camera_machine_seen, camera_machine_state, camera_machine_reported_at, camera_machine_last_seen, camera_machine_saw_this_frame, camera_machine_dead_called, camera_machine_put, laser_machine_new, laser_machine_last_trip_at, laser_machine_trips, laser_machine_cut, laser_machine_dead_called, laser_machine_held, laser_machine_put, light_home_new, light_home_add, light_home_camera, light_home_laser, light_home_stand, light_home_feed, light_home_feed_burning, light_home_in, light_home_flip, light_home_step, light_home_paint, light_home_camera_pan, light_home_camera_see, light_home_laser_open, light_home_laser_close};
