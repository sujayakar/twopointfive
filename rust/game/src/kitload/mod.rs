pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{guns_of, gun_load, add, load_share, load_of, load_ref, snap, load_worn, load_pace_k, load_seen_k, load_heard_k, load_wear, load_man, load_on, load_pace_on, load_sprint_on, sam_load_pace, load_pace, load_gait, load_seen, load_heard, load_cross};
