pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{note, live, nv_hand_free, kit_nv, kit_nv_eye, hands_off, kit_in_hand, kit_sling_front, kit_sling_rest, kit_closeness, kit_tick, kit_click, kit_lock_max, kit_show, kit_gun_up, kit_reload, kit_summary, kit_reload_note};
