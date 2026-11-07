pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{note, seated_of, state_of, arc, close_has_seat, close_seated, close_windows, window_at, clear_between, close_subject, close_at, close_lethal, close_rmb, close_bare, close_live, close_blow, close_kind_of, close_rows, sneak_hint, note_best, close_civ_gate, close_civ_offer, close_ready, close_mode, mode_now, close_tick, close_near_k, close_station, close_noise, close_took, clip_reach, label_of, close_marks, calm, close_seat, close_unseat};
pub mod p_02;
pub use p_02::{close_seat_tick, close_reset, close_summary};
