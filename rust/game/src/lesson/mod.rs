pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{stalk_line, bare_line, lower, lesson_token, token_line, lesson_token_faults, lesson_line, passes, after, num, str, rect, vec, inside, who, on_stack, downed, partner_of, band, band_check, of_check, free_in, test_fault};
pub mod p_02;
pub use p_02::{lesson_new, lesson_data, lesson_at, lesson_index, lesson_step, lesson_finished, lesson_tick, lesson_settling, lesson_advance, lesson_fresh, lesson_count, lesson_snapshot, lesson_restore, lesson_over, lesson_objective, lesson_summary, lesson_note, line_of, step_line, check_lesson};
