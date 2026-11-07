pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{is_weather_kind, snow_level, snow_level_of_query, is_any_kind, weather_title, weather_words, q, pack_fog, fog_glow_word, unpack_fog, fog_tau, pack_rect, unpack_rect, in_rect, weather_of_search, init_weather, weather_search};
