pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{pulls, map_of, flat, chair_face, live, calm, put, drop, dark, left_lit, left_off, left_tick, blank, walk_len, seated, span, doze_lines, screen_seat, find_chair, doze_patrol, up, wake, doze_deaf, doze_tick, face_light, lamp_on, doze_in_chair, doze_asleep, doze_dark, doze_at, doze_ear, doze_woke, doze_breath, doze_mark, doze_of, doze_summary, doze_reset};
