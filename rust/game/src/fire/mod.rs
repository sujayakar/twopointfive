pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{fits, axis_at, fresh, state_of, fire_stats, fire_shown, h01, is_fire, fire_seed, fire_pops, fire_flicker, mouth_of, heat_of, burns, wind_at, look_row, fire_holds, fire_struck, fire_stir, held, plain};
pub mod p_02;
pub use p_02::{update_fire, fire_draws, fire_summary};
