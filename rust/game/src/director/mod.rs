pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{lifted_budget, manifest_of, lint_manifest, scan_showcase_verbs};
pub mod p_02;
pub use p_02::{director_inject_input, director_new, director_report, director_attach, director_programs, director_showing, director_lower_third, director_handed_over, director_can_start, director_running, director_lesson_runs, director_their_frags_off, director_objective, director_signal, director_signalled, director_chapters, director_summary, director_push, director_rows_of, director_start, director_stop, director_on_encounter_restart, director_tick, director_fire_triggers, director_start_beat, director_resume_all, director_resume, director_holds, director_fail_beat, director_begin_finish, director_tail, director_frame_shot, director_look_of, director_enter_show, director_exit_show, director_restore_weather, director_restore_chaos, director_hand_over, director_apply_systems, director_restore_systems, director_tick_drill, director_end_drill, director_show_man, director_tick_shown, director_tick_bursts, director_tick_tracks, director_tick_kicks, director_place_of, director_door_of, director_guard_of, director_plan_of, director_inside_volume, director_query_of};
pub mod p_03;
pub use p_03::{director_ctx_of, cond_label, near_of, sam_view_of, view_of, signal_view};
