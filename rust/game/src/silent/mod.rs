pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{mate, cast_of, squad_silent, set_silent, silent_man, silent_pace, reached, urgent, silent_step, silent_dark, silent_hush, silent_ease, silent_no_boot, silent_pick, silent_boot, silent_no_cans, quiet_gun, own_hold, silent_holds, silent_on_man, silent_glass_holds, silent_tool, silent_compromised, rows, loud_word, seat, silent_tick, silent_summary};
