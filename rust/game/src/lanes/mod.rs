pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{fresh, s, flat, mine, fit, live, eye_of, cell_at, in_mask, knows, close_fight, pushing, lane_e, roused, flanking, up_id, def_of, note, wedge, remask, mask_of, gun_on, lane_at, lane_on, cost, sees, become_known, read_rounds, lanes_tick};
pub mod p_02;
pub use p_02::{held_at, back_out, round, stun, bangs, guns_manned, spot_seen, net_sees, stepper, flank_tick, flank_go, pin_start, pin_step, winding, frag_out, guns_mid, frag_place, frag_step, peek_start, peek_step, mouth_of, price_of, lane_penalty};
pub mod p_03;
pub use p_03::{lane_gate, along, plant_short, ahead, lane_holds, lane_throws, lane_warns, lane_door, lane_drill, lane_covers, lane_exposed, lane_reload, lanes_reset, lanes_stats, lanes_summary, lanes_list, lanes_knows};
