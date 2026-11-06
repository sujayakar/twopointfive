pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod b_01;
pub use b_01::{thin_note, thin_half, thin_fit, thin_air};
