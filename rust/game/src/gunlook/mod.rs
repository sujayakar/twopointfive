pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{gun_look_on, gun_shells_on, gun_own, voice_of, flame_of, under, long_model, pistol_model, gun_look_of};
