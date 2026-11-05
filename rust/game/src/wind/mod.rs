pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{shape, save_wind, reset_wind_knobs, wind_of, air_out, jet_vel, in_rect, storm_vel, storm_vel_roofs, vnoise, vent_due, wind_for_particles, wind_new, wind_door_source, wind_reroof, wind_sync_field, wind_field_roofs, wind_storm_share, wind_share_under, wind_storm_drift, wind_blows, wind_storm_wind, wind_update, wind_at, wind_by_name, wind_fresh, wind_describe, wind_summary};
