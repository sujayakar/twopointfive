pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{he40_stats_reset, arc_length, arc_armed, inside_at, shell_man, shell_bodies, shell_solve, preview_too_close, shell_aim_point, fire_shell, log_end, burst, bounce_off, dud, update_shells, dud_hits_character, shell_boxes};
