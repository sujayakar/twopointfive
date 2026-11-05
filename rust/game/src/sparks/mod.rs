pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod b_01;
pub use b_01::{sparks_new, sparks_slot, sparks_spawn, sparks_h, sparks_emit_cone, sparks_emit_burst, sparks_clear, sparks_update, sparks_boxes};
