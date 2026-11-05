pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{in_sight, contact_behind, auto_entries, auto_pick, auto_quick, auto_hint, show_secs_of};
