pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{roofs_of, open_ground, sky_spots, sky_over, sky_over_most, own_floor, stroke_mean, weather_enabled, weather_set_enabled, weather_kind, weather_set_kind, weather_roofs, weather_set_roofs, weather_fog, weather_set_fog, weather_thunder_until, weather_set_thunder_until, weather_cloud, weather_storm, weather_set_storm, weather_rain_rate, weather_set_rain_rate, weather_wet, weather_set_wet, weather_thunders, weather_set_thunders_are, weather_thunder, weather_next_flash_at, weather_set_next_flash_at, weather_die, weather_set_die, weather_last_bark_t, weather_set_last_bark_t, weather_dry_paint, weather_new, weather_level_kind, weather_own_night, weather_own_storm, weather_office_fit, weather_sync_roofs, weather_level_roofs, weather_own_roofs, weather_set_kind_2, weather_refuses, weather_roofs_now, weather_under_roof, weather_rain_bed, weather_hear_mask, weather_air_trans, weather_seed, weather_snapshot, weather_restore, weather_set_enabled_2, weather_strike, weather_thunder_due, weather_update, weather_gap, weather_set_rain_now, weather_apply_flash, weather_end_flash, weather_play_thunder, weather_ground_wet, weather_set_wet_2, weather_teardown, weather_summary};
pub mod p_02;
pub use p_02::{dir_of, zero, hear_path, loud_len, stroke_light, air_trans};
