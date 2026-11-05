pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{gpu_fx, load_dust, save_dust, reset_dust, apply_dust_look, emit_dust, surf_at, said_surf, solid_glass, hit_look, impact_fx, muzzle_sparks_fx, bang_fx, kick_dust_fx, fixture_shatter_fx, lens_shatter_fx, breaker_spark_fx, lock_splinter_fx, clack_fx, ember_fx, casing_of, state_of, casing_fx, update_particle_fx, clear_particle_fx, pending_tinks, spark_sink, test_burst};
