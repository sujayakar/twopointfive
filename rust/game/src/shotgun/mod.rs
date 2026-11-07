pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{pellet_drops, pellet_end, fire_shell, carries_breacher, lock_point, breach_spot};
