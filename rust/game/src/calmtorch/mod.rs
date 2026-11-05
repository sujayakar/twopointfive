pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{carried, place_light, work_lit, place_read, calm_torch_ruled, calm_torch_dark, click, calm_torch_of, calm_torch_summary, calm_torch_posed};
