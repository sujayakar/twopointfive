pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{reset_wound_stats, seg_dist, hit_zone, round_does, wound_down, lay, finish_wounded, wounded_men};
