//! WRITTEN BY tools/sim/men.ts FROM src/sim/men/schema.ts — never by hand (tools/qa/men-check.ts is red on a difference). A man's columns, as a rule here reads them.
//! A man is a row: `state(g).men.<column>[row]`. The host writes the columns through src/sim/men/cols.ts, in these same bytes, with no copy.
use core::mem::{offset_of, size_of};

/// rows of each column (src/sim/men/schema.ts MEN_ROWS)
pub const ROWS: usize = 256;
/// the hash of the layout: src/sim/men/cols.ts MEN_SCHEMA is the same number
pub const SCHEMA: u32 = 0xe38df8ec;

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
    pub gen: [u32; ROWS],
    pub truths0: [u32; ROWS],
    pub state: [u32; ROWS],
    pub owner: [u32; ROWS],
    pub rank: [u32; ROWS],
    pub reason: [u32; ROWS],
    pub down_kind: [u32; ROWS],
    pub down_by: [u32; ROWS],
    pub truths1: [u32; ROWS],
}
const _: () = assert!(size_of::<Men>() == 119824);

/// where each column is in the part, in bytes, in the order of the schema's layout (the host checks these against its own)
pub const COLS: [usize; 63] = [offset_of!(Men, pos_x), offset_of!(Men, pos_y), offset_of!(Men, pos_z), offset_of!(Men, vel_x), offset_of!(Men, vel_y), offset_of!(Men, vel_z), offset_of!(Men, body_yaw), offset_of!(Men, aim_yaw), offset_of!(Men, aim_pitch), offset_of!(Men, radius), offset_of!(Men, step_phase), offset_of!(Men, speed), offset_of!(Men, last_pos_x), offset_of!(Men, last_pos_y), offset_of!(Men, last_pos_z), offset_of!(Men, speed_sm), offset_of!(Men, hits_left), offset_of!(Men, fire_cd), offset_of!(Men, noise), offset_of!(Men, step_travel), offset_of!(Men, step_owed), offset_of!(Men, step_at), offset_of!(Men, since), offset_of!(Men, hurt_at), offset_of!(Men, stagger_until), offset_of!(Men, dazzled_until), offset_of!(Men, muzzle_ahead), offset_of!(Men, died_at), offset_of!(Men, killer_id), offset_of!(Men, missed_at), offset_of!(Men, muzzle_clear_t), offset_of!(Men, muzzle_eval_t), offset_of!(Men, line_bark_t), offset_of!(Men, line_in_t), offset_of!(Men, gun_lost_at), offset_of!(Men, torch_lost_at), offset_of!(Men, light_dead_until), offset_of!(Men, door_down_until), offset_of!(Men, reload_t), offset_of!(Men, last_fire_t), offset_of!(Men, friend_t), offset_of!(Men, hit_flash), offset_of!(Men, door_bash_at), offset_of!(Men, door_bash_lean), offset_of!(Men, door_rip_until), offset_of!(Men, door_head), offset_of!(Men, door_head_t), offset_of!(Men, drag_yaw), offset_of!(Men, drag_noise_t), offset_of!(Men, drag_stuck_t), offset_of!(Men, equip_t), offset_of!(Men, pace), offset_of!(Men, pace_shown_t), offset_of!(Men, nv_amount), offset_of!(Men, gen), offset_of!(Men, truths0), offset_of!(Men, state), offset_of!(Men, owner), offset_of!(Men, rank), offset_of!(Men, reason), offset_of!(Men, down_kind), offset_of!(Men, down_by), offset_of!(Men, truths1)];
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
    w(m.gen[r]);
    w(m.truths0[r]);
    w(m.state[r]);
    w(m.owner[r]);
    w(m.rank[r]);
    w(m.reason[r]);
    w(m.down_kind[r]);
    w(m.down_by[r]);
    w(m.truths1[r]);
    h
}
