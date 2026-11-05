//! WRITTEN BY tools/sim/men.ts FROM src/sim/men/schema.ts — never by hand (tools/qa/men-check.ts is red on a difference). A man's columns, as a rule here reads them.
//! A man is a row: `state(g).men.<column>[row]`. The host writes the columns through src/sim/men/cols.ts, in these same bytes, with no copy.
use core::mem::{offset_of, size_of};

/// rows of each column (src/sim/men/schema.ts MEN_ROWS)
pub const ROWS: usize = 64;
/// the hash of the layout: src/sim/men/cols.ts MEN_SCHEMA is the same number
pub const SCHEMA: u32 = 0xb8124aa1;

#[repr(C)]
pub struct Men {
    pub spare: [u32; 4],
    pub pos_x: [f64; ROWS],
    pub pos_y: [f64; ROWS],
    pub pos_z: [f64; ROWS],
    pub vel_x: [f64; ROWS],
    pub vel_y: [f64; ROWS],
    pub vel_z: [f64; ROWS],
    pub body_yaw: [f64; ROWS],
    pub aim_yaw: [f64; ROWS],
    pub aim_pitch: [f64; ROWS],
    pub radius: [f64; ROWS],
    pub step_phase: [f64; ROWS],
    pub speed: [f64; ROWS],
    pub last_pos_x: [f64; ROWS],
    pub last_pos_y: [f64; ROWS],
    pub last_pos_z: [f64; ROWS],
    pub speed_sm: [f64; ROWS],
    pub hits_left: [f64; ROWS],
    pub fire_cd: [f64; ROWS],
    pub noise: [f64; ROWS],
    pub visibility: [f64; ROWS],
    pub irradiance: [f64; ROWS],
    pub awareness: [f64; ROWS],
    pub last_seen_t: [f64; ROWS],
    pub sight_pos_x: [f64; ROWS],
    pub sight_pos_y: [f64; ROWS],
    pub sight_pos_z: [f64; ROWS],
    pub sight_vel_x: [f64; ROWS],
    pub sight_vel_y: [f64; ROWS],
    pub sight_vel_z: [f64; ROWS],
    pub sight_t: [f64; ROWS],
    pub react_t: [f64; ROWS],
    pub heard_up_to: [f64; ROWS],
    pub wp: [f64; ROWS],
    pub wait: [f64; ROWS],
    pub path_i: [f64; ROWS],
    pub repath_t: [f64; ROWS],
    pub path_goal_x: [f64; ROWS],
    pub path_goal_y: [f64; ROWS],
    pub path_goal_z: [f64; ROWS],
    pub sup_v: [f64; ROWS],
    pub aim_track: [f64; ROWS],
    pub id: [f64; ROWS],
    pub step_travel: [f64; ROWS],
    pub step_owed: [f64; ROWS],
    pub step_at: [f64; ROWS],
    pub since: [f64; ROWS],
    pub hurt_at: [f64; ROWS],
    pub stagger_until: [f64; ROWS],
    pub dazzled_until: [f64; ROWS],
    pub muzzle_ahead: [f64; ROWS],
    pub died_at: [f64; ROWS],
    pub killer_id: [f64; ROWS],
    pub missed_at: [f64; ROWS],
    pub muzzle_clear_t: [f64; ROWS],
    pub muzzle_eval_t: [f64; ROWS],
    pub line_bark_t: [f64; ROWS],
    pub line_in_t: [f64; ROWS],
    pub gun_lost_at: [f64; ROWS],
    pub torch_lost_at: [f64; ROWS],
    pub light_dead_until: [f64; ROWS],
    pub door_down_until: [f64; ROWS],
    pub reload_t: [f64; ROWS],
    pub last_fire_t: [f64; ROWS],
    pub friend_t: [f64; ROWS],
    pub hit_flash: [f64; ROWS],
    pub door_bash_at: [f64; ROWS],
    pub door_bash_lean: [f64; ROWS],
    pub door_rip_until: [f64; ROWS],
    pub door_head: [f64; ROWS],
    pub door_head_t: [f64; ROWS],
    pub drag_yaw: [f64; ROWS],
    pub drag_noise_t: [f64; ROWS],
    pub drag_stuck_t: [f64; ROWS],
    pub equip_t: [f64; ROWS],
    pub pace: [f64; ROWS],
    pub pace_shown_t: [f64; ROWS],
    pub nv_amount: [f64; ROWS],
    pub dazzled_at_x: [f64; ROWS],
    pub dazzled_at_y: [f64; ROWS],
    pub dazzled_at_z: [f64; ROWS],
    pub led_t: [f64; ROWS],
    pub fix_t: [f64; ROWS],
    pub fix_ref_x: [f64; ROWS],
    pub fix_ref_y: [f64; ROWS],
    pub fix_ref_z: [f64; ROWS],
    pub search_t: [f64; ROWS],
    pub smoke_trans: [f64; ROWS],
    pub saw_body_t: [f64; ROWS],
    pub alert_best: [f64; ROWS],
    pub alert_stall_t: [f64; ROWS],
    pub alert_ref_x: [f64; ROWS],
    pub alert_ref_y: [f64; ROWS],
    pub alert_ref_z: [f64; ROWS],
    pub chase_t: [f64; ROWS],
    pub shot_blocked_t: [f64; ROWS],
    pub leader_seen_t: [f64; ROWS],
    pub leader_seen_pos_x: [f64; ROWS],
    pub leader_seen_pos_y: [f64; ROWS],
    pub leader_seen_pos_z: [f64; ROWS],
    pub leader_still_t: [f64; ROWS],
    pub body_visits: [f64; ROWS],
    pub body_visit_t: [f64; ROWS],
    pub body_fan: [f64; ROWS],
    pub sweep: [f64; ROWS],
    pub beam_dir_x: [f64; ROWS],
    pub beam_dir_y: [f64; ROWS],
    pub beam_dir_z: [f64; ROWS],
    pub look_phase: [f64; ROWS],
    pub route_i: [f64; ROWS],
    pub follow_t: [f64; ROWS],
    pub follow_goal_x: [f64; ROWS],
    pub follow_goal_y: [f64; ROWS],
    pub follow_goal_z: [f64; ROWS],
    pub stuck_t: [f64; ROWS],
    pub stuck_n: [f64; ROWS],
    pub stall_ref_x: [f64; ROWS],
    pub stall_ref_y: [f64; ROWS],
    pub stall_ref_z: [f64; ROWS],
    pub stall_t: [f64; ROWS],
    pub line_side: [f64; ROWS],
    pub line_gain: [f64; ROWS],
    pub line_step_m: [f64; ROWS],
    pub line_stuck_t: [f64; ROWS],
    pub pie_last_t: [f64; ROWS],
    pub pie_prev_x: [f64; ROWS],
    pub pie_prev_y: [f64; ROWS],
    pub pie_prev_z: [f64; ROWS],
    pub pie_prev_t: [f64; ROWS],
    pub canisters: [f64; ROWS],
    pub flashbangs: [f64; ROWS],
    pub frags: [f64; ROWS],
    pub shoved_t: [f64; ROWS],
    pub kit_owed: [f64; ROWS],
    pub code_t: [f64; ROWS],
    pub hand_wait: [f64; ROWS],
    pub ease_stall_t: [f64; ROWS],
    pub ease_best: [f64; ROWS],
    pub cam_seq: [f64; ROWS],
    pub sup_kick: [f64; ROWS],
    pub sup_src_x: [f64; ROWS],
    pub sup_src_y: [f64; ROWS],
    pub sup_src_z: [f64; ROWS],
    pub sup_src_id: [f64; ROWS],
    pub sup_src_t: [f64; ROWS],
    pub sup_last_add: [f64; ROWS],
    pub sup_flinch_at: [f64; ROWS],
    pub sup_pinned_at: [f64; ROWS],
    pub sup_band: [f64; ROWS],
    pub sup_down_t: [f64; ROWS],
    pub sup_pinned_t: [f64; ROWS],
    pub sup_cover_x: [f64; ROWS],
    pub sup_cover_y: [f64; ROWS],
    pub sup_cover_z: [f64; ROWS],
    pub sup_displaced_at: [f64; ROWS],
    pub sup_displaces: [f64; ROWS],
    pub sup_cover_until: [f64; ROWS],
    pub sup_bark_t: [f64; ROWS],
    pub sup_pending_t: [f64; ROWS],
    pub aim_last_bearing: [f64; ROWS],
    pub aim_bearing_rate: [f64; ROWS],
    pub aim_speed_sm: [f64; ROWS],
    pub aim_move_sm: [f64; ROWS],
    pub aim_left_t: [f64; ROWS],
    pub aim_left_track: [f64; ROWS],
    pub fall_ahead: [f64; ROWS],
    pub gen: [u32; ROWS],
    pub truths0: [u32; ROWS],
    pub state: [u32; ROWS],
    pub team: [u32; ROWS],
    pub foe: [u32; ROWS],
    pub slot: [u32; ROWS],
    pub owner: [u32; ROWS],
    pub rank: [u32; ROWS],
    pub reason: [u32; ROWS],
    pub down_kind: [u32; ROWS],
    pub down_by: [u32; ROWS],
    pub truths1: [u32; ROWS],
    pub leader: [u32; ROWS],
    pub throw_kind: [u32; ROWS],
    pub held_kind: [u32; ROWS],
    pub big_kit: [u32; ROWS],
}
const _: () = assert!(size_of::<Men>() == 88592);

/// where each column is in the part, in bytes, in the order of the schema's layout (the host checks these against its own)
pub const COLS: [usize; 181] = [offset_of!(Men, pos_x), offset_of!(Men, pos_y), offset_of!(Men, pos_z), offset_of!(Men, vel_x), offset_of!(Men, vel_y), offset_of!(Men, vel_z), offset_of!(Men, body_yaw), offset_of!(Men, aim_yaw), offset_of!(Men, aim_pitch), offset_of!(Men, radius), offset_of!(Men, step_phase), offset_of!(Men, speed), offset_of!(Men, last_pos_x), offset_of!(Men, last_pos_y), offset_of!(Men, last_pos_z), offset_of!(Men, speed_sm), offset_of!(Men, hits_left), offset_of!(Men, fire_cd), offset_of!(Men, noise), offset_of!(Men, visibility), offset_of!(Men, irradiance), offset_of!(Men, awareness), offset_of!(Men, last_seen_t), offset_of!(Men, sight_pos_x), offset_of!(Men, sight_pos_y), offset_of!(Men, sight_pos_z), offset_of!(Men, sight_vel_x), offset_of!(Men, sight_vel_y), offset_of!(Men, sight_vel_z), offset_of!(Men, sight_t), offset_of!(Men, react_t), offset_of!(Men, heard_up_to), offset_of!(Men, wp), offset_of!(Men, wait), offset_of!(Men, path_i), offset_of!(Men, repath_t), offset_of!(Men, path_goal_x), offset_of!(Men, path_goal_y), offset_of!(Men, path_goal_z), offset_of!(Men, sup_v), offset_of!(Men, aim_track), offset_of!(Men, id), offset_of!(Men, step_travel), offset_of!(Men, step_owed), offset_of!(Men, step_at), offset_of!(Men, since), offset_of!(Men, hurt_at), offset_of!(Men, stagger_until), offset_of!(Men, dazzled_until), offset_of!(Men, muzzle_ahead), offset_of!(Men, died_at), offset_of!(Men, killer_id), offset_of!(Men, missed_at), offset_of!(Men, muzzle_clear_t), offset_of!(Men, muzzle_eval_t), offset_of!(Men, line_bark_t), offset_of!(Men, line_in_t), offset_of!(Men, gun_lost_at), offset_of!(Men, torch_lost_at), offset_of!(Men, light_dead_until), offset_of!(Men, door_down_until), offset_of!(Men, reload_t), offset_of!(Men, last_fire_t), offset_of!(Men, friend_t), offset_of!(Men, hit_flash), offset_of!(Men, door_bash_at), offset_of!(Men, door_bash_lean), offset_of!(Men, door_rip_until), offset_of!(Men, door_head), offset_of!(Men, door_head_t), offset_of!(Men, drag_yaw), offset_of!(Men, drag_noise_t), offset_of!(Men, drag_stuck_t), offset_of!(Men, equip_t), offset_of!(Men, pace), offset_of!(Men, pace_shown_t), offset_of!(Men, nv_amount), offset_of!(Men, dazzled_at_x), offset_of!(Men, dazzled_at_y), offset_of!(Men, dazzled_at_z), offset_of!(Men, led_t), offset_of!(Men, fix_t), offset_of!(Men, fix_ref_x), offset_of!(Men, fix_ref_y), offset_of!(Men, fix_ref_z), offset_of!(Men, search_t), offset_of!(Men, smoke_trans), offset_of!(Men, saw_body_t), offset_of!(Men, alert_best), offset_of!(Men, alert_stall_t), offset_of!(Men, alert_ref_x), offset_of!(Men, alert_ref_y), offset_of!(Men, alert_ref_z), offset_of!(Men, chase_t), offset_of!(Men, shot_blocked_t), offset_of!(Men, leader_seen_t), offset_of!(Men, leader_seen_pos_x), offset_of!(Men, leader_seen_pos_y), offset_of!(Men, leader_seen_pos_z), offset_of!(Men, leader_still_t), offset_of!(Men, body_visits), offset_of!(Men, body_visit_t), offset_of!(Men, body_fan), offset_of!(Men, sweep), offset_of!(Men, beam_dir_x), offset_of!(Men, beam_dir_y), offset_of!(Men, beam_dir_z), offset_of!(Men, look_phase), offset_of!(Men, route_i), offset_of!(Men, follow_t), offset_of!(Men, follow_goal_x), offset_of!(Men, follow_goal_y), offset_of!(Men, follow_goal_z), offset_of!(Men, stuck_t), offset_of!(Men, stuck_n), offset_of!(Men, stall_ref_x), offset_of!(Men, stall_ref_y), offset_of!(Men, stall_ref_z), offset_of!(Men, stall_t), offset_of!(Men, line_side), offset_of!(Men, line_gain), offset_of!(Men, line_step_m), offset_of!(Men, line_stuck_t), offset_of!(Men, pie_last_t), offset_of!(Men, pie_prev_x), offset_of!(Men, pie_prev_y), offset_of!(Men, pie_prev_z), offset_of!(Men, pie_prev_t), offset_of!(Men, canisters), offset_of!(Men, flashbangs), offset_of!(Men, frags), offset_of!(Men, shoved_t), offset_of!(Men, kit_owed), offset_of!(Men, code_t), offset_of!(Men, hand_wait), offset_of!(Men, ease_stall_t), offset_of!(Men, ease_best), offset_of!(Men, cam_seq), offset_of!(Men, sup_kick), offset_of!(Men, sup_src_x), offset_of!(Men, sup_src_y), offset_of!(Men, sup_src_z), offset_of!(Men, sup_src_id), offset_of!(Men, sup_src_t), offset_of!(Men, sup_last_add), offset_of!(Men, sup_flinch_at), offset_of!(Men, sup_pinned_at), offset_of!(Men, sup_band), offset_of!(Men, sup_down_t), offset_of!(Men, sup_pinned_t), offset_of!(Men, sup_cover_x), offset_of!(Men, sup_cover_y), offset_of!(Men, sup_cover_z), offset_of!(Men, sup_displaced_at), offset_of!(Men, sup_displaces), offset_of!(Men, sup_cover_until), offset_of!(Men, sup_bark_t), offset_of!(Men, sup_pending_t), offset_of!(Men, aim_last_bearing), offset_of!(Men, aim_bearing_rate), offset_of!(Men, aim_speed_sm), offset_of!(Men, aim_move_sm), offset_of!(Men, aim_left_t), offset_of!(Men, aim_left_track), offset_of!(Men, fall_ahead), offset_of!(Men, gen), offset_of!(Men, truths0), offset_of!(Men, state), offset_of!(Men, team), offset_of!(Men, foe), offset_of!(Men, slot), offset_of!(Men, owner), offset_of!(Men, rank), offset_of!(Men, reason), offset_of!(Men, down_kind), offset_of!(Men, down_by), offset_of!(Men, truths1), offset_of!(Men, leader), offset_of!(Men, throw_kind), offset_of!(Men, held_kind), offset_of!(Men, big_kit)];
// every column is whole vectors from a 16-byte boundary: the part starts on one (state.rs), the head is 16 bytes, the rows a multiple of four
const _: () = assert!(ROWS % 4 == 0);
const _: () = { let mut i = 0; while i < COLS.len() { assert!(COLS[i] % 16 == 0); i += 1; } };

#[no_mangle]
pub extern "C" fn men_schema() -> u32 { SCHEMA }
/// column `i`'s place in the part in bytes, or the part's rows (`i` = the number of columns) · 0 past that
#[no_mangle]
pub extern "C" fn men_col(i: u32) -> usize { let i = i as usize; if i < COLS.len() { COLS[i] } else if i == COLS.len() { ROWS } else { 0 } }
/// EVERY NUMBER OF A MAN'S ROW AS THIS MODULE READS IT, folded (FNV-1a over the words, the columns in their order): what tools/qa/men-check.ts holds to the TypeScript's accessors
#[no_mangle]
pub extern "C" fn men_fold(g: u32, row: u32) -> u32 {
    let m = &crate::state::state(g as usize).men;
    let r = row as usize % ROWS;
    let mut h: u32 = 0x811c9dc5;
    let mut w = |v: u32| { h = (h ^ v).wrapping_mul(0x01000193); };
    let b = m.pos_x[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.pos_y[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.pos_z[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.vel_x[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.vel_y[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.vel_z[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.body_yaw[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.aim_yaw[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.aim_pitch[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.radius[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.step_phase[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.speed[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.last_pos_x[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.last_pos_y[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.last_pos_z[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.speed_sm[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.hits_left[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.fire_cd[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.noise[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.visibility[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.irradiance[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.awareness[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.last_seen_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sight_pos_x[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sight_pos_y[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sight_pos_z[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sight_vel_x[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sight_vel_y[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sight_vel_z[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sight_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.react_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.heard_up_to[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.wp[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.wait[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.path_i[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.repath_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.path_goal_x[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.path_goal_y[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.path_goal_z[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sup_v[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.aim_track[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.id[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.step_travel[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.step_owed[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.step_at[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.since[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.hurt_at[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.stagger_until[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.dazzled_until[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.muzzle_ahead[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.died_at[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.killer_id[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.missed_at[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.muzzle_clear_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.muzzle_eval_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.line_bark_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.line_in_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.gun_lost_at[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.torch_lost_at[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.light_dead_until[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.door_down_until[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.reload_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.last_fire_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.friend_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.hit_flash[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.door_bash_at[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.door_bash_lean[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.door_rip_until[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.door_head[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.door_head_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.drag_yaw[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.drag_noise_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.drag_stuck_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.equip_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.pace[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.pace_shown_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.nv_amount[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.dazzled_at_x[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.dazzled_at_y[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.dazzled_at_z[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.led_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.fix_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.fix_ref_x[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.fix_ref_y[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.fix_ref_z[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.search_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.smoke_trans[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.saw_body_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.alert_best[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.alert_stall_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.alert_ref_x[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.alert_ref_y[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.alert_ref_z[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.chase_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.shot_blocked_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.leader_seen_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.leader_seen_pos_x[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.leader_seen_pos_y[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.leader_seen_pos_z[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.leader_still_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.body_visits[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.body_visit_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.body_fan[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sweep[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.beam_dir_x[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.beam_dir_y[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.beam_dir_z[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.look_phase[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.route_i[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.follow_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.follow_goal_x[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.follow_goal_y[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.follow_goal_z[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.stuck_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.stuck_n[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.stall_ref_x[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.stall_ref_y[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.stall_ref_z[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.stall_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.line_side[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.line_gain[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.line_step_m[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.line_stuck_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.pie_last_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.pie_prev_x[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.pie_prev_y[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.pie_prev_z[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.pie_prev_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.canisters[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.flashbangs[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.frags[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.shoved_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.kit_owed[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.code_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.hand_wait[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.ease_stall_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.ease_best[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.cam_seq[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sup_kick[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sup_src_x[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sup_src_y[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sup_src_z[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sup_src_id[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sup_src_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sup_last_add[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sup_flinch_at[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sup_pinned_at[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sup_band[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sup_down_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sup_pinned_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sup_cover_x[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sup_cover_y[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sup_cover_z[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sup_displaced_at[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sup_displaces[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sup_cover_until[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sup_bark_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.sup_pending_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.aim_last_bearing[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.aim_bearing_rate[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.aim_speed_sm[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.aim_move_sm[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.aim_left_t[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.aim_left_track[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    let b = m.fall_ahead[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    w(m.gen[r]);
    w(m.truths0[r]);
    w(m.state[r]);
    w(m.team[r]);
    w(m.foe[r]);
    w(m.slot[r]);
    w(m.owner[r]);
    w(m.rank[r]);
    w(m.reason[r]);
    w(m.down_kind[r]);
    w(m.down_by[r]);
    w(m.truths1[r]);
    w(m.leader[r]);
    w(m.throw_kind[r]);
    w(m.held_kind[r]);
    w(m.big_kit[r]);
    h
}
