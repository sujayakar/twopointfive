pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{senses, net_asks, squads_where, squad_drill};
pub mod p_02;
pub use p_02::{squad_drill_words, squad_skill, flat, pick, alive, grown, room_of, note, along, path_len, squad_leg_of, squad_word_of, squad_say, squads_let_go, squads_stage, squads_stop, squads_live, squad_of, squad_may_wedge, squad_rally, def_of, fit, squad_look_since, in_squad_rooms, home, tell, team_known, way_round, team_known_how, round_ctx, way_round2, event, goers, mate, stay_key, net_k, own_k, may_go, decide, door_at, contact};
pub mod p_03;
pub use p_03::{follow, moved_on, back, look_out, lost_room_has, check_dice, check_beat, rc_of, went_silent, on_net, heard_fire, bang_t, walk, radio_check, looker, turn_back, settle, looks, noticed, glimpse, call_quiet};
pub mod p_04;
pub use p_04::{key_up, call_about, calls_cut, cut_ask, owed_of, calls_hung, calls_tick, squad_calls, net_wary, seen_to, net_say, waits_on, net_send, net_check, squads_report, squads_summary, squad_men, squads_tick};
pub mod p_05;
pub use p_05::{squad_ways_round};
