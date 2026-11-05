pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{hour_label, hour_clock, hour_words, hour_live, hour_id_of, init_hour, hour_from_query, hour_href, hour_of, hour_sun, is_hour_sun, in_estimate, sky_lamp_of, sun_strength, sun_given, set_sun_strength, cloud_sun, cloud_soft, greyed, sky_share, cloud_light, cloud_total, under_weather, hour_weather_changed, sun_to_cloud, lum, to_sun_of, apply_hour, put_in, take_out, sun_reach, lamps_out, hour_after_reset, hour_world_changed, casters_of, rays, sky_lum, box_of, reach_of, static_blocked, open_rays, hour_light, moon_light, moon_forget, sky_light, hour_light_at, hour_summary};
