pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{kahan2, kahan3, kahan4, scaled2, scaled3, scaled4, native2, native3, native4, sample, same, agrees, hyp_exact, kahan_n};
