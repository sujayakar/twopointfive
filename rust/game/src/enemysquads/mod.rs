pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{senses, net_asks, squads_where, squad_drill, squad_drill_words};
pub mod p_02;
pub use p_02::{squad_skill, flat, pick, alive, grown, room_of, note, along, path_len, squads_stage, squads_stop, squads_live, squad_of, squad_owns, squad_defers, squad_may_wedge, squad_stays_in, squad_rally, def_of, fit, squad_look_since, squad_busy, squad_holds, home, tell, team_known, way_round, team_known_how, round_ctx, way_round2, event, goers, mate, stay_key, net_k, own_k, may_go, decide, door_at, contact, follow, moved_on};
pub mod p_03;
pub use p_03::{back, look_out, lost_room_has, check_dice, check_beat, rc_of, went_silent, on_net, heard_fire, bang_t, walk, radio_check, looker, turn_back, settle, looks, noticed, glimpse, call_quiet, key_up, call_about};
pub mod p_04;
pub use p_04::{calls_cut, cut_ask, owed_of, calls_hung, calls_tick, squad_calls, net_wary, seen_to, net_say, waits_on, net_send, net_check, squads_report, squads_summary, squad_men, squads_tick, squad_ways_round};
