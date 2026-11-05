//! src/game/doors.ts lines 689 to 1040
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The hearing level of a key's clack; only `doors_use` reads it.   (doors.ts Doors.KEY_LEVEL)
const KEY_LEVEL: f64 = 0.12;

/// `doorMem()[at]`: one double of the step's hand-over, where `at` is an index as `door_io` gives them, a byte address / 8.   (doors.ts:694, 722)
fn mem(at: f64) -> f64 {
    // SAFETY: every `at` here is a table's start from door_io + a column x the table's height + a row within what the step was handed: inside rust/sim's hand-over, a static of doubles.
    unsafe { core::ptr::read((at as usize * 8) as *const f64) }
}

// ---- the module's numbers for a word: its place in doorshome.ts DOOR_SOUNDS and STRIKE_TIERS

/// `DOOR_SOUNDS[n]` (doorshome.ts:26): the enum's own numbering is not the module's promise, the list's order is.
fn sound_at(n: f64) -> crate::doors::DoorSound {
    match n as i32 {
        0 => DoorSound::Creak,
        1 => DoorSound::Bang,
        2 => DoorSound::Latch,
        3 => DoorSound::Bash,
        4 => DoorSound::Rattle,
        5 => DoorSound::Pick,
        6 => DoorSound::Unlock,
        7 => DoorSound::Kick,
        8 => DoorSound::Clang,
        9 => DoorSound::Thud,
        _ => unreachable!("no door sound has this place"),
    }
}

/// `STRIKE_TIERS[n]` (doorshome.ts:28): open 0, slam 1, boot 2. The Rust enum declares them in another order.
fn strike_tier_at(n: f64) -> crate::doors::DoorStrikeTier {
    match n as i32 {
        0 => DoorStrikeTier::Open,
        1 => DoorStrikeTier::Slam,
        2 => DoorStrikeTier::Boot,
        _ => unreachable!("no strike tier has this place"),
    }
}

/// `m.slice(from, to)`: the doubles read out now, before a listener can ask the module anything.
fn mem_copy(from: f64, to: f64) -> Vec<f64> {
    let mut out = Vec::new();
    let mut at = from;
    while at < to {
        out.push(mem(at));
        at += 1.0;
    }
    out
}

/// What the step did taken back: each contact's end and what a leaf did to him, the tallies, each door's noises and then its box.   (doors.ts Doors.takeBack)
pub fn doors_take_back(g: &mut crate::game::Game, contacts: &mut Vec<crate::doors::DoorContact>, ev: f64) {
    let (dk, dp, dout, de) = (crate::sim_doorshome::dk(), crate::sim_doorshome::dp(), crate::sim_doorshome::do_(), crate::sim_doorshome::de());
    let io = crate::sim_doorshome::door_io(g);
    let n = g.doors.list.len();
    let nc = contacts.len();
    let cap = crate::sim_doorshome::DOOR_CAP;
    let kk = io.max_contacts;
    let mut o = io.con;
    for c in 0..nc {
        let k = &mut contacts[c];
        k.pos[0] = mem(o + dk.x * kk);
        k.pos[2] = mem(o + dk.z * kk);
        let by = mem(o + dk.blocked_by * kk);
        if by >= 0.0 {
            k.blocked_by = Some(g.doors.list[by as usize]);
        }
        if k.run.is_some() {
            let b = mem(o + dk.bashed * kk);
            let r = mem(o + dk.ripped * kk);
            if b >= 0.0 {
                k.bashed = Some(g.doors.list[b as usize]);
                k.bashed_at = Some(mem(o + dk.bashed_at * kk));
            }
            if r >= 0.0 {
                k.ripped = Some(g.doors.list[r as usize]);
            }
        }
        let s = mem(o + dk.struck * kk);
        if s >= 0.0 {
            k.struck = Some(crate::doors::DoorStrike {
                door: g.doors.list[s as usize],
                by: mem(o + dk.struck_by * kk),
                tier: strike_tier_at(mem(o + dk.struck_tier * kk)),
                floor: mem(o + dk.struck_floor * kk) != 0.0,
                dir: [mem(o + dk.struck_dx * kk), mem(o + dk.struck_dz * kk)],
            });
        }
        if mem(o + dk.moved * kk) != 0.0 {
            k.moved = Some(true);
        }
        if mem(o + dk.man_asked * kk) != 0.0 {
            k.man = Some(mem(o + dk.man * kk));
        }
        o += 1.0;
    }
    let p = io.par;
    g.doors.solids.carried += mem(p + dp.carried);
    g.doors.solids.stopped += mem(p + dp.stopped);
    g.doors.solids.passed += mem(p + dp.passed);
    g.doors.solids.reached += mem(p + dp.reached);
    g.doors.solids.held += mem(p + dp.held);
    g.doors.solids.aside += mem(p + dp.aside);
    g.doors.rounds += mem(p + dp.rounds);
    g.m.sim_doorshome.doors_stepped.there += 1.0;
    g.m.sim_doorshome.doors_stepped.turned += mem(p + dp.turned);
    g.m.sim_doorshome.doors_stepped.noises += ev;
    if ev == 0.0 {
        for i in 0..n {
            doors_follow(g, i as f64);
        }
        return;
    }
    // Everything the step left is read out first: a listener may ask the module, which copies the leaves from the boxes over the rows the step placed.
    let e_all = mem_copy(io.ev, io.ev + ev * crate::sim_doorshome::DOOR_EV);
    let out = mem_copy(io.out, io.out + crate::sim_doorshome::DOOR_OUTS * cap);
    let at_out = |col: f64, i: usize| (col * cap) as usize + i;
    let mut rows: Vec<Option<Vec<f64>>> = Vec::with_capacity(n);
    for i in 0..n {
        rows.push(if out[at_out(dout.placed, i)] != 0.0 { Some(crate::scene_box_mirror::box_mirror_leaf_row(g, i as f64)) } else { None });
    }
    crate::scene_box_mirror::box_mirror_leaves_turned(g);
    let stride = crate::sim_doorshome::DOOR_EV as usize;
    let noises = ev as usize;
    let at_e = |e: usize, col: f64| e + col as usize;
    let mut at = 0usize;
    for i in 0..n {
        while at < noises && e_all[at_e(at * stride, de.door)] == i as f64 {
            let e = at * stride;
            let d = g.doors.list[i];
            if e_all[at_e(e, de.fresh)] != 0.0 {
                door_kept_pos(g, d, [e_all[at_e(e, de.px)], 1.0, e_all[at_e(e, de.pz)]]);
            }
            let pos = door_kept_at(g, d);
            doors_sound(g, d, sound_at(e_all[at_e(e, de.sound)]), pos, e_all[at_e(e, de.who)], e_all[at_e(e, de.level)]);
            at += 1;
        }
        if let Some(r) = &rows[i] {
            let d = g.doors.list[i];
            let b = g.dr[d].box_;
            let yaw = crate::scene_box_mirror::box_mirror_follow_row(g, b, r);
            doors_handles(g, d, yaw, out[at_out(dout.h1x, i)], out[at_out(dout.h1z, i)], out[at_out(dout.h2x, i)], out[at_out(dout.h2z, i)]);
        }
    }
    crate::scene_box_mirror::box_mirror_leaves_turned(g);
}

/// Door i's box and handles made what the step placed.   (doors.ts Doors.follow)
pub fn doors_follow(g: &mut crate::game::Game, i: f64) {
    let dout = crate::sim_doorshome::do_();
    let io = crate::sim_doorshome::door_io(g);
    let o = io.out + i;
    let cap = crate::sim_doorshome::DOOR_CAP;
    if mem(o + dout.placed * cap) == 0.0 {
        return;
    }
    let d = g.doors.list[i as usize];
    let b = g.dr[d].box_;
    let yaw = crate::scene_box_mirror::box_mirror_follow_leaf(g, i, b);
    let (x1, z1, x2, z2) = (mem(o + dout.h1x * cap), mem(o + dout.h1z * cap), mem(o + dout.h2x * cap), mem(o + dout.h2z * cap));
    doors_handles(g, d, yaw, x1, z1, x2, z2);
}

/// A leaf's two handle boxes placed.   (doors.ts Doors.handles)
pub fn doors_handles(g: &mut crate::game::Game, d: DoorId, yaw: f64, x1: f64, z1: f64, x2: f64, z2: f64) {
    let (h0, h1) = (g.dr[d].handle_boxes[0], g.dr[d].handle_boxes[1]);
    g.col.boxes[h0].c = [x1, 1.0, z1];
    g.col.boxes[h0].yaw = yaw;
    g.col.boxes[h1].c = [x2, 1.0, z2];
    g.col.boxes[h1].yaw = yaw;
}

/// The doors' step: the twin's one call. The hand-over must already be written.   (doors.ts Doors.updateHere)
pub fn doors_update_here(g: &mut crate::game::Game, dt: f64, contacts: &[crate::doors::DoorContact]) {
    let w = g.col.mirror.slot as usize;
    let _ = sim::doors::update(g.sim, w);
}

/// Hold-to-crack: ease a near-closed leaf open silently, up to `max_angle`.   (doors.ts Doors.crack)
pub fn doors_crack(g: &mut crate::game::Game, d: DoorId, from: V3, who: f64, dt: f64, keys: Option<&JsSet<String>>, rate: Option<f64>, max_angle: Option<f64>) {
    let rate = rate.unwrap_or(0.55);
    let max_angle = max_angle.unwrap_or(0.6);
    if door_off(g, d) || doors_locked_out(g, d, from, keys) {
        return;
    }
    let [hx, hz] = g.dr[d].def.hinge;
    let u = door_dir(g, d);
    let (px, pz) = (from[0] - hx, from[2] - hz);
    let perp = u[0] * pz - u[1] * px;
    let sgn = door_swing(g, d);
    if (if perp >= 0.0 { 1.0 } else { -1.0 }) == door_swing(g, d) {
        let pull = door_pull_t(g, d);
        door_set_pull_t(g, d, m::max(pull, 0.3));
    }
    door_set_latched(g, d, false);
    door_set_closing(g, d, false);
    door_set_vel(g, d, 0.0);
    door_set_quiet_t(g, d, -1.5);
    door_set_last_who(g, d, who);
    let a = door_angle(g, d);
    if m::abs(a) < max_angle {
        let to = a + sgn * rate * dt;
        let (lo, hi) = (door_min_a(g, d), door_max_a(g, d));
        door_set_angle(g, d, v3::clamp(to, lo, hi));
    }
}

/// A tap on F: closed, it swings open away from the user; open, it is pulled to; locked against him, the rattle or his key's clack.   (doors.ts Doors.use)
pub fn doors_use(g: &mut crate::game::Game, d: DoorId, from: V3, who: f64, keys: Option<&JsSet<String>>) {
    if door_off(g, d) {
        return;
    }
    if doors_locked_out(g, d, from, keys) {
        doors_rattle(g, d, who, None);
        return;
    }
    if doors_key_opens(g, d, from, keys) {
        let pos = door_pos(g, d);
        doors_sound(g, d, DoorSound::Unlock, pos, who, KEY_LEVEL);
    }
    let [hx, hz] = g.dr[d].def.hinge;
    let u = door_dir(g, d);
    let (px, pz) = (from[0] - hx, from[2] - hz);
    let perp = u[0] * pz - u[1] * px;
    if door_is_closed(g, d) {
        let sgn = if perp >= 0.0 { -1.0 } else { 1.0 };
        door_set_latched(g, d, false);
        door_set_closing(g, d, false);
        let v = sgn * (if sgn != door_swing(g, d) { PULL_VEL } else { 3.2 });
        door_set_vel(g, d, v);
        door_set_quiet_t(g, d, -1.0);
        let pos = door_pos(g, d);
        doors_sound(g, d, DoorSound::Creak, pos, who, 0.2);
    } else {
        doors_pull_to(g, d, who);
    }
    door_set_last_who(g, d, who);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_noise_is_named_by_the_modules_own_numbers() {
        assert_eq!(sound_at(sim::doors::S_CREAK), DoorSound::Creak);
        assert_eq!(sound_at(sim::doors::S_BANG), DoorSound::Bang);
        assert_eq!(sound_at(sim::doors::S_LATCH), DoorSound::Latch);
        assert_eq!(sound_at(sim::doors::S_BASH), DoorSound::Bash);
        assert_eq!(sound_at(sim::doors::S_RATTLE), DoorSound::Rattle);
        assert_eq!(sound_at(9.0), DoorSound::Thud);
    }

    #[test]
    fn a_strike_keeps_the_lists_order_not_the_enums() {
        assert_eq!(strike_tier_at(0.0), DoorStrikeTier::Open);
        assert_eq!(strike_tier_at(1.0), DoorStrikeTier::Slam);
        assert_eq!(strike_tier_at(2.0), DoorStrikeTier::Boot);
    }

    #[test]
    fn the_hand_over_is_read_by_an_address_over_eight() {
        let row = [1.5_f64, -2.25, 0.125];
        let at = row.as_ptr() as usize as f64 / 8.0;
        assert_eq!(mem(at).to_bits(), 1.5_f64.to_bits());
        assert_eq!(mem(at + 2.0).to_bits(), 0.125_f64.to_bits());
    }
}
