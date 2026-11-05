pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{civ_spots, with_dice, log, join, spawn_civilians, reset_civilians, civilian_compromise, make_civilian, spawn_hostage, spawn_target, civilian_yield, civilian_bolt, civilian_sees, civilian_tie};
pub mod p_02;
pub use p_02::{hostage_say, forget_meter, civilians_on_floor, civilians_shown, civilian_bodies, civilian_door_contacts, floor_of, later, run_pending, civ_say, on_screen_head, in_view, pick, away, report_pool, no_security, fill, control, scream, update_civilians, set_state, update_one, step_sound, eye_of, see, hear, startle_or_worse, bodies, civ_visibility, task_at, civilian_pushes_props, civilian_seated, work, end_task, to_work, to_startled, startled, to_alarmed, alarmed, released, path_metres, tellable, pick_target, to_flee, flee, first_hand, to_report};
pub mod p_03;
pub use p_03::{prime_guard, report, deliver_report, to_phone, phone, phone_call, to_gone, corner_spot, to_cower, hostage_lead, civilian_led_to, civilian_off, follow, cower, civilian_grabbed, civilian_answer, spell_words, civilian_sam_line, civilian_comply, civilian_down, civ_point_in_view, staff_calm, guards_notice, body_in_view_of, found_down, hear_scream, chatty_enough, greetings, evacuation_check, save_civ_tune, load_civ_tune, set_civilians, civilians_summary, civ_panic, check_civ_spots};
