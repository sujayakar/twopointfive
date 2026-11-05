pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{clamp01, ws_take_secs, ws_press_near, ws_ease, ws_hit, ws_corner_angle, ws_seen_deg};
