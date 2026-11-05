pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod b_01;
pub use b_01::{tell_mess, dirtied, clear_snow_tracks, stamp_mark, form_snow_tracks, press, press_messy, press_crater, crater_into, press_groove, press_clods, update_snow_tracks, find_unseen, mark, lay, press_of};
