pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{clamp, state_of, lead, mate, on_wall, sam_sneaks, squad_found, unseen_live, cue, unseen_man, entering, off_wall, low_now, unseen_low, unseen_follow, unseen_post, unseen_wall_slot, unseen_wall_lost, wall_slot, unseen_pace, unseen_step, unseen_dark, unseen_hush, unseen_ease, unseen_holds, unseen_glass, unseen_glass_held, unseen_word, unseen_summary};
