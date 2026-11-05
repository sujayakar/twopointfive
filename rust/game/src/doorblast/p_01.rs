//! src/game/doorblast.ts lines 1 to 494
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---- what skeleton 3 still cannot do: one blocked step, with the TypeScript's own arguments

/// `FURN_BREAK.wood.land` (:295): FURN_BREAK came out as `pub const FURN_BREAK: bool = false`
fn seam_wood_land(g: &mut Game) -> crate::furnblast::BreakRowLand {
    todo!("needs: FURN_BREAK's rows (crate::furnblast::FURN_BREAK is a bool)")
}

/// the slabs' hand-over in rust/sim's memory (sim::slabs::addr(0), the same doubles the page's `slabMem()` reads); sim::slabs::par is private
fn slab_par() -> &'static mut [f64; sim::slabs::NPAR] {
    unsafe { &mut *(sim::slabs::addr(0) as *mut [f64; sim::slabs::NPAR]) }
}

/// Keeps a line of what the doors' blasts did. (doorblast.ts note)
pub fn note(g: &mut crate::game::Game, s: &str) {
    let line = format!("{:.2}s {}", g.time, s);
    let log = &mut g.m.doorblast.doorblast_log;
    log.push(line);
    if log.len() > 200 {
        log.remove(0);
    }
}

/// The slab's centre height with its foot (or its face) on the floor at this much tip. (doorblast.ts support)
pub fn support(g: &mut crate::game::Game, d: DoorId, tip: f64) -> f64 {
    (g.dr[d].height / 2.0) * m::cos(tip) + (g.dr[d].thick / 2.0) * m::sin(tip) + LIFT
}

/// The nearest static (or other door's leaf) along a horizontal ray. (doorblast.ts hitAt)
pub fn hit_at(g: &mut crate::game::Game, x: f64, y: f64, z: f64, ex: f64, ez: f64, tmax: f64) -> Option<crate::doorblast::HitAt> {
    crate::scene_collision::raycast(g, [x, y, z], [ex, 0.0, ez], tmax).map(|h| crate::doorblast::HitAt { t: h.t, n: h.n })
}

/// Runs `fn_` with the door's own box out of the dynamic list. (doorblast.ts withoutLeaf)
pub fn without_leaf<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, d: DoorId, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    let b = g.dr[d].box_;
    let all = core::mem::take(&mut g.col.dynamic_boxes);
    g.col.dynamic_boxes = all.iter().copied().filter(|x| *x != b).collect();
    let r = fn_(g);
    g.col.dynamic_boxes = all;
    r
}

/// Does a floor rectangle cut into a static standing in the heights [y0, y1]? (doorblast.ts overlaps)
pub fn overlaps(g: &mut crate::game::Game, x: f64, z: f64, a: f64, hu: f64, hn: f64, y0: f64, y1: f64) -> bool {
    let (ux, uz, su, sn) = (m::cos(a), m::sin(a), hu - 0.012, hn - 0.012);
    let ex = m::abs(ux) * hu + m::abs(uz) * hn;
    let ez = m::abs(uz) * hu + m::abs(ux) * hn;
    let mut ov = ov();
    let n = crate::scene_collision::gather(g, x - ex, z - ez, x + ex, z + ez, y0, y1, &mut ov);
    for i in 0..n as usize {
        let b = BoxId(ov[i] as u32);
        let (yaw, bc, bh) = (g.col.boxes[b].yaw, g.col.boxes[b].c, g.col.boxes[b].h);
        let (bx, bz, dx, dz) = (m::cos(yaw), -m::sin(yaw), bc[0] - x, bc[2] - z);
        let c = m::abs(ux * bx + uz * bz);
        let s = m::abs(ux * bz - uz * bx);
        if m::abs(dx * ux + dz * uz) > su + bh[0] * c + bh[2] * s {
            continue;
        }
        if m::abs(-dx * uz + dz * ux) > sn + bh[0] * s + bh[2] * c {
            continue;
        }
        if m::abs(dx * bx + dz * bz) > bh[0] + su * c + sn * s {
            continue;
        }
        if m::abs(-dx * bz + dz * bx) > bh[2] + su * s + sn * c {
            continue;
        }
        return true;
    }
    false
}

/// The nearest spot within maxR where that rectangle stands clear, by rings of sixteen bearings. (doorblast.ts freeSpot)
pub fn free_spot(g: &mut crate::game::Game, x: f64, z: f64, a: f64, hu: f64, hn: f64, y0: f64, y1: f64, max_r: f64) -> Option<[f64; 2]> {
    let mut r = 0.03;
    while r <= max_r + 1e-9 {
        for i in 0..16 {
            let th = a + (i as f64) * core::f64::consts::PI / 8.0;
            let px = x + m::cos(th) * r;
            let pz = z + m::sin(th) * r;
            if !overlaps(g, px, pz, a, hu, hn, y0, y1) {
                return Some([px, pz]);
            }
        }
        r += if r < 0.3 { 0.03 } else { 0.06 };
    }
    None
}

/// A step that has met nothing yet. (doorblast.ts nothingMet)
pub fn nothing_met() -> crate::doorblast::SlabMet {
    crate::doorblast::SlabMet { landed: false, lx: 0.0, lz: 0.0, rested: false, nudged: 0.0, nudge_max: 0.0, laid: 0.0 }
}

/// The doors' side of a blast: every leaf within `tear_m` that it reaches comes off. (doorblast.ts doorBlast)
pub fn door_blast(g: &mut crate::game::Game, at: V3, tear_m: f64, who: f64) -> Vec<DoorId> {
    let mut out = Vec::new();
    if !(tear_m > 0.0) {
        return out;
    }
    g.stats.doorblast.doorblast_blasts += 1;
    for item in door_blast_reach(g, at, tear_m) {
        if tear_off(g, item.d, at, who, Some(item.k)) {
            out.push(item.d);
        }
    }
    out
}

/// The leaves a blast would take off, each with its strength there, asked without touching anything. (doorblast.ts doorBlastReach)
pub fn door_blast_reach(g: &mut crate::game::Game, at: V3, tear_m: f64) -> Vec<crate::doorblast::DoorBlastReachItem> {
    let mut out = Vec::new();
    if !(tear_m > 0.0) {
        return out;
    }
    let list = g.doors.list.clone();
    for d in list {
        if crate::doors::door_off(g, d) || g.dr[d].def.steel.unwrap_or(false) {
            continue;
        }
        let [hx, hz] = g.dr[d].def.hinge;
        let w = g.dr[d].def.width;
        let (px, pz) = (at[0] - hx, at[2] - hz);
        if px * px + pz * pz > m::pow(w + tear_m + 0.1, 2.0) {
            continue;
        }
        let u = crate::doors::door_dir(g, d);
        let t = v3::clamp(px * u[0] + pz * u[1], 0.0, w);
        let y = v3::clamp(at[1], 0.05, g.dr[d].height);
        let dist = m::max(0.0, m::hyp3(px - u[0] * t, at[1] - y, pz - u[1] * t) - g.dr[d].thick / 2.0);
        if dist > tear_m || !reaches(g, d, at) {
            continue;
        }
        out.push(crate::doorblast::DoorBlastReachItem { d, k: 1.0 - dist / tear_m });
    }
    out
}

/// Has the blast a line to the leaf that no wall is in (the leaves themselves are not walls to it). (doorblast.ts reaches)
pub fn reaches(g: &mut crate::game::Game, d: DoorId, at: V3) -> bool {
    let [hx, hz] = g.dr[d].def.hinge;
    let w = g.dr[d].def.width;
    let u = crate::doors::door_dir(g, d);
    let s = if u[0] * (at[2] - hz) - u[1] * (at[0] - hx) >= 0.0 { 1.0 } else { -1.0 };
    let off = g.dr[d].thick / 2.0 + 0.08;
    let height = g.dr[d].height;
    let from: V3 = [at[0], m::max(at[1], 0.15), at[2]];
    let all = core::mem::take(&mut g.col.dynamic_boxes);
    let mut open = false;
    'lines: for f in [0.5, 0.85, 0.15] {
        for y in [1.0, 0.3, 1.8] {
            let p: V3 = [hx + u[0] * w * f - u[1] * off * s, m::min(y, height - 0.1), hz + u[1] * w * f + u[0] * off * s];
            if v3::dist(from, p) < 0.12 || !crate::scene_collision::solid_blocked(g, from, p, None) {
                open = true;
                break 'lines;
            }
        }
    }
    g.col.dynamic_boxes = all;
    open
}

/// Sam's hands come off a leaf that has left its frame. (doorblast.ts handsOff)
pub fn hands_off(g: &mut crate::game::Game, d: DoorId) {
    let of = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
    for pl in crate::seats::men(g, &of) {
        if g.pl[pl].easing == Some(d) {
            g.pl[pl].easing = None;
        }
        if g.pl[pl].picking == Some(d) {
            g.pl[pl].picking = None;
            let c = g.pl[pl].ch;
            g.ch[c].anim.lockpick = false;
            g.pl[pl].speed_sm = 0.0;
        }
        if g.pl[pl].coding == Some(d) {
            g.pl[pl].coding = None;
            g.pl[pl].code_t = 0.0;
        }
        if g.pl[pl].wanding.as_ref().map(|w| w.door) == Some(d) {
            crate::wand::wand_drop(g, pl);
        }
    }
}

/// The frame lets go of a leaf: its lock is gone, and `who` is the last to have touched it. (doorblast.ts :242-243, :287-288)
fn frame_lets_go(g: &mut Game, d: DoorId, who: f64) {
    if !crate::doors::door_lock_broken(g, d) {
        crate::doors::door_set_noticed(g, d, false);
    }
    crate::doors::door_set_locked(g, d, false);
    crate::doors::door_set_lock_broken(g, d, true);
    crate::doors::door_set_pick(g, d, 0.0);
    crate::doors::door_set_picking(g, d, false);
    crate::doors::door_set_rattle_t(g, d, 0.0);
    crate::doors::door_set_last_who(g, d, who);
}

/// A door's state once its leaf is off: wide open and quiet. (doorblast.ts :252, :317)
fn leaf_is_off(g: &mut Game, d: DoorId, fly: f64) {
    crate::doors::door_set_off(g, d, true);
    crate::doors::door_set_angle(g, d, fly * OFF_ANGLE);
    crate::doors::door_set_vel(g, d, 0.0);
    crate::doors::door_set_latched(g, d, false);
    crate::doors::door_set_closing(g, d, false);
    crate::doors::door_set_passers(g, d, 0.0);
    crate::doors::door_set_quiet_t(g, d, 10.0);
    crate::doors::door_set_since_bash(g, d, 10.0);
    crate::doors::door_set_creak_armed(g, d, false);
    crate::doors::door_set_shot(g, d, 0.0);
}

/// The leaf comes off, thrown out of its frame away from `from`. False, and nothing done, for steel or one already off. (doorblast.ts tearOff)
pub fn tear_off(g: &mut crate::game::Game, d: DoorId, from: V3, who: f64, k: Option<f64>) -> bool {
    let k = k.unwrap_or(1.0);
    if crate::doors::door_off(g, d) || g.dr[d].def.steel.unwrap_or(false) {
        return false;
    }
    let [hx, hz] = g.dr[d].def.hinge;
    let w = g.dr[d].def.width;
    let a = g.dr[d].def.closed_dir + crate::doors::door_angle(g, d);
    let (ux, uz) = (m::cos(a), m::sin(a));
    let cx = hx + ux * (w * 0.5 + 0.01);
    let cz = hz + uz * (w * 0.5 + 0.01);
    let face = if ux * (from[2] - hz) - uz * (from[0] - hx) >= 0.0 { 1.0 } else { -1.0 };
    let fly: f64 = -face;
    let (nx, nz) = (-uz * fly, ux * fly);
    frame_lets_go(g, d, who);
    let pos = crate::doors::door_pos(g, d);
    crate::doors::doors_sound(g, d, crate::doors::DoorSound::Kick, pos, who, 1.0);
    let mut ahead: f64 = 9.0;
    without_leaf(g, d, &mut |g: &mut Game| {
        for f in [0.0, -0.4, 0.4] {
            if let Some(h) = hit_at(g, cx + ux * w * f + nx * 0.06, 0.2, cz + uz * w * f + nz * 0.06, nx, nz, 9.0) {
                ahead = m::min(ahead, h.t + 0.06);
            }
        }
    });
    let back = ahead < g.dr[d].height + g.k.doorblast.room;
    let (mut dx, mut dz) = (nx, nz);
    if !back {
        let (rx, rz) = (cx - from[0], cz - from[2]);
        let rl = m::hyp2(rx, rz);
        if rl > 0.05 {
            dx += 0.35 * rx / rl;
            dz += 0.35 * rz / rl;
            let l = m::hyp2(dx, dz);
            dx /= l;
            dz /= l;
        }
    }
    let speed = g.k.doorblast.speed;
    let sp = v3::lerp(speed[0], speed[1], v3::clamp(k, 0.0, 1.0)) * (if back { 0.7 } else { 1.0 });
    leaf_is_off(g, d, fly);
    let (up, spin, tip) = (g.k.doorblast.up, g.k.doorblast.spin, g.k.doorblast.tip);
    let p: V3 = [cx, support(g, d, 0.0), cz];
    let ang = crate::doors::door_angle(g, d);
    crate::doors::door_set_slab(
        g,
        d,
        Some(crate::doors::SlabSeen {
            p,
            v: [dx * sp, up, dz * sp],
            a,
            spin: if back { 0.0 } else { fly * spin },
            tip: 0.0,
            tip_v: if back { 0.0 } else { tip },
            side: if back { -fly } else { fly },
            rest: false,
            landed: false,
            who,
            t: 0.0,
            ang,
            bits: None,
            wedged: None,
        }),
    );
    hands_off(g, d);
    g.stats.doorblast.doorblast_torn += 1;
    if back {
        g.stats.doorblast.doorblast_back += 1;
    }
    let said = if back { format!("no room ahead ({:.2} m): falls back", ahead) } else { format!("{:.2} m ahead", ahead) };
    let line = format!("{} off its hinges · k {:.2} · {}", g.dr[d].def.name, k, said);
    note(g, &line);
    pose_slab(g, d);
    crate::doors::doors_torn(g, d, who, None);
    true
}

/// The rounds' work a leaf stands: DOORSHOT.toll times plasterboard's. (doorblast.ts leafTough)
pub fn leaf_tough(g: &mut crate::game::Game) -> f64 {
    let drywall = g.k.walls.tough.get(String::from("drywall")).copied().unwrap_or(6.0);
    let wood = g.k.walls.tough.get(String::from("wood")).copied().unwrap_or(9.0);
    m::min(drywall, wood) * m::max(0.1, g.k.doorshot.toll)
}

/// A round met a leaf (dynamic box `index − 1e6`), worth `w` of a wall cell's toll. (doorblast.ts leafRound)
pub fn leaf_round(g: &mut crate::game::Game, index: f64, who: f64, hit_p: V3, dir: V3, w: f64) {
    let i = index - 1e6;
    if !(i >= 0.0) || (i as usize) as f64 != i || i as usize >= g.col.dynamic_boxes.len() {
        return;
    }
    let b = g.col.dynamic_boxes[i as usize];
    let d = match g.doors.list.iter().copied().find(|x| g.dr[*x].box_ == b) {
        Some(d) => d,
        None => return,
    };
    if crate::doors::door_off(g, d) || g.dr[d].def.steel.unwrap_or(false) {
        return;
    }
    let was = crate::doors::door_shot(g, d);
    let chaos = crate::destruction::chaos_of(g);
    crate::doors::door_set_shot(g, d, was + w / (2.0 * m::max(1e-3, chaos)));
    g.stats.doorblast.doorshot_rounds += 1;
    let share = crate::doors::door_shot(g, d) / leaf_tough(g);
    if share >= 1.0 {
        splinter(g, d, dir, who, Some(hit_p));
    }
}

/// `R(k)` of splinter: the hash of (seed + k). (doorblast.ts :292)
fn r_of(seed: f64, k: f64) -> f64 {
    crate::debris::hash01((seed + k) * 2654435761.0)
}

/// The cuts of a leaf's strips or lengths, uneven by `uneven`. (doorblast.ts :293)
fn cuts(seed: f64, n: f64, salt: f64, uneven: f64) -> Vec<f64> {
    let mut c = vec![-1.0];
    let mut j = 1.0;
    while j < n {
        c.push(-1.0 + 2.0 * (j + (r_of(seed, salt + j) - 0.5) * uneven) / n);
        j += 1.0;
    }
    c.push(1.0);
    c
}

/// The leaf goes in pieces the way a round travelling `dir` sends it; the lock rail stays as its box. (doorblast.ts splinter)
pub fn splinter(g: &mut crate::game::Game, d: DoorId, dir: V3, who: f64, at: Option<V3>) -> bool {
    if crate::doors::door_off(g, d) || g.dr[d].def.steel.unwrap_or(false) {
        return false;
    }
    let kk = g.k.doorshot.clone();
    let [hx, hz] = g.dr[d].def.hinge;
    let (w_, h_, t_) = (g.dr[d].def.width, g.dr[d].height, g.dr[d].thick);
    let a = g.dr[d].def.closed_dir + crate::doors::door_angle(g, d);
    let (ux, uz) = (m::cos(a), m::sin(a));
    let cx = hx + ux * (w_ * 0.5 + 0.01);
    let cz = hz + uz * (w_ * 0.5 + 0.01);
    let hw = g.dr[d].leaf_h[0];
    let leaf = g.dr[d].box_;
    let yaw = g.col.boxes[leaf].yaw;
    let fly: f64 = if dir[2] * ux - dir[0] * uz >= 0.0 { 1.0 } else { -1.0 };
    let (nx, nz) = (-uz * fly, ux * fly);
    let (mut dx, mut dz) = (dir[0], dir[2]);
    {
        let l = m::hyp2(dx, dz);
        if l < 0.2 {
            dx = nx;
            dz = nz;
        } else {
            dx = 0.6 * dx / l + 0.4 * nx;
            dz = 0.6 * dz / l + 0.4 * nz;
            let mm = m::hyp2(dx, dz);
            dx /= mm;
            dz /= mm;
        }
    }
    frame_lets_go(g, d, who);
    let pos = crate::doors::door_pos(g, d);
    crate::doors::doors_sound(g, d, crate::doors::DoorSound::Kick, pos, who, kk.level);
    // the cuts: strips along the grain, each to uneven lengths; the piece the handles are on is the lock rail
    let struck = match at {
        Some(at) => m::floor(((at[0] - hx) * ux + (at[2] - hz) * uz) * 16.0) * 7.0 + m::floor(at[1] * 16.0) * 31.0,
        None => 0.0,
    };
    let door_ix = g.doors.list.iter().position(|x| *x == d).map_or(-1.0, |i| i as f64);
    let seed = door_ix * 977.0 + 131.0 + struck;
    let xs = cuts(seed, m::max(1.0, kk.strips), 10.0, kk.uneven);
    let handle_x = (w_ - 0.14) - (w_ * 0.5 + 0.01);
    let handle_y = 1.0 - (h_ / 2.0 + 0.01);
    let mut left = kk.chunks;
    let mut rail: Option<[f64; 4]> = None;
    let mut rail_x = 0.0;
    let mut k = 100.0;
    let land = seam_wood_land(g);
    for i in 0..xs.len() - 1 {
        let ys = cuts(seed, m::max(1.0, kk.lengths), 20.0 + (i as f64) * 8.0, kk.uneven);
        let lx = (xs[i] + xs[i + 1]) / 2.0 * hw;
        let sw = (xs[i + 1] - xs[i]) / 2.0 * hw;
        for j in 0..ys.len() - 1 {
            let ly = (ys[j] + ys[j + 1]) / 2.0 * h_ / 2.0;
            let sl = (ys[j + 1] - ys[j]) / 2.0 * h_ / 2.0;
            if rail.is_none() && m::abs(handle_x - lx) <= sw && m::abs(handle_y - ly) <= sl {
                let mx = m::max(0.0, sw - 0.065);
                let my = m::max(0.0, sl - 0.03);
                rail = Some([sw, sl, v3::clamp(handle_x - lx, -mx, mx), v3::clamp(handle_y - ly, -my, my)]);
                rail_x = lx;
                continue;
            }
            let r1 = r_of(seed, k);
            k += 1.0;
            let r2 = r_of(seed, k);
            k += 1.0;
            let r3 = r_of(seed, k);
            k += 1.0;
            let sp = v3::lerp(kk.fly[0], kk.fly[1], r1);
            let mut sd = (r2 - 0.5) * 2.0 * kk.side;
            if sd * lx > 0.0 && m::abs(lx) > hw * 0.4 {
                sd *= -0.5;
            }
            let top = fly * v3::lerp(kk.topple[0], kk.topple[1], r2);
            let v: V3 = [dx * sp + ux * sd, v3::lerp(kk.up[0], kk.up[1], r3), dz * sp + uz * sd];
            let om: V3 = [(r1 - 0.5) * kk.spin + ux * top, (r2 - 0.5) * 0.6 * kk.spin, (r3 - 0.5) * kk.spin + uz * top];
            let take = left > 0.0;
            left -= 1.0;
            let got = if take {
                let c: V3 = [cx + ux * lx, h_ / 2.0 + 0.01 + ly, cz + uz * lx];
                let he: V3 = [sw, sl, t_ / 2.0];
                let rot = v3::quat::mul(v3::quat::axis_angle([ux, 0.0, uz], fly * v3::lerp(kk.tilt[0], kk.tilt[1], r3)), v3::quat::yaw(yaw));
                let f = crate::scene_boxes::copy_box(g, leaf);
                let piece = &mut g.col.boxes[f];
                piece.c = c;
                piece.h = he;
                piece.yaw = yaw;
                piece.rot = Some(rot);
                piece.marks = None;
                crate::debris::debris_spawn(g, crate::debris::ChunkKind::Chunk, f, v, om, Some(who), Some(land.level))
            } else {
                None
            };
            match got {
                Some(c) => {
                    g.stats.doorblast.doorshot_chunks += 1;
                    g.tab.debris_chunk[c].level = land.level;
                    g.tab.debris_chunk[c].sfx = land.sfx.iter().map(|s| crate::debris::ChunkSfxItem { name: s.name.clone(), rate: s.rate, gain: s.gain }).collect();
                    crate::break_table::breaks_retag(g, c, crate::break_table::Mat::Wood);
                    crate::break_table::breaks_dress(g, c);
                }
                None => g.stats.doorblast.doorshot_demoted += 1,
            }
        }
    }
    // what is left: the lock rail, lying where it dropped a little the round's way, clear of every static or laid in its own doorway
    let bits = rail.unwrap_or([hw / m::max(1.0, kk.strips), h_ / 2.0 / m::max(1.0, kk.lengths), 0.0, 0.0]);
    let (y0, y1) = (0.06, 0.3);
    let mut ra = a + (r_of(seed, 7.0) - 0.5) * 0.8;
    let mut px = cx + ux * rail_x + dx * 0.3;
    let mut pz = cz + uz * rail_x + dz * 0.3;
    if overlaps(g, px, pz, ra, bits[0], bits[1], y0, y1) {
        match free_spot(g, px, pz, ra, bits[0], bits[1], y0, y1, 0.9) {
            Some(to) => {
                px = to[0];
                pz = to[1];
            }
            None => {
                let fc = g.dr[d].frame_centre;
                px = fc[0];
                pz = fc[2];
                ra = g.dr[d].def.closed_dir;
            }
        }
    }
    leaf_is_off(g, d, fly);
    let ang = crate::doors::door_angle(g, d);
    crate::doors::door_set_slab(
        g,
        d,
        Some(crate::doors::SlabSeen {
            p: [px, t_ / 2.0 + LIFT, pz],
            v: [0.0, 0.0, 0.0],
            a: ra,
            spin: 0.0,
            tip: HALF_PI,
            tip_v: 0.0,
            side: fly,
            rest: true,
            landed: true,
            who,
            t: 0.0,
            ang,
            bits: Some(bits),
            wedged: None,
        }),
    );
    hands_off(g, d);
    g.stats.doorblast.doorshot_splintered += 1;
    let line = format!("{} splintered by gunfire · {} pieces so far", g.dr[d].def.name, g.stats.doorblast.doorshot_chunks);
    note(g, &line);
    pose_slab(g, d);
    // a body asleep against the leaf slides down now that it is gone
    let mut chars = vec![g.pl[g.player].ch];
    for o in g.guards.iter() {
        chars.push(g.gd[*o].ch);
    }
    for o in crate::civilians::civilians_shown(g) {
        chars.push(g.gd[o].ch);
    }
    for ch in chars {
        let rag = match g.ch[ch].ragdoll {
            Some(rag) if g.tab.ragdoll[rag].sleeping => rag,
            _ => continue,
        };
        let h = g.ch[ch].bones.get(crate::character::Bone::Hips).copied().unwrap_or(g.ch[ch].pos);
        if m::hyp2(h[0] - hx, h[2] - hz) < w_ + 1.2 || m::hyp2(h[0] - px, h[2] - pz) < bits[1] + 0.8 {
            crate::ragdoll::ragdoll_wake(g, rag);
        }
    }
    crate::doors::doors_torn(g, d, who, Some(crate::doors::DoorsTornCause::Fire));
    true
}

/// Once a frame: every loose slab still moving is stepped and posed, in the order of their doors. (doorblast.ts doorSlabs)
pub fn door_slabs(g: &mut crate::game::Game, dt: f64) {
    let n = crate::sim_slabshome::slab_store_fly(g);
    if n == 0.0 {
        return;
    }
    let mut ids: Vec<f64> = Vec::new();
    let mut r = 0.0;
    while r < n {
        ids.push(crate::sim_slabshome::slab_store_door_at(g, r));
        r += 1.0;
    }
    for i in ids {
        let d = g.doors.list[i as usize];
        if !crate::doors::door_off(g, d) {
            continue;
        }
        fly_slab(g, d, dt);
        pose_slab(g, d);
    }
}

/// One slab's step, and what it met said in one place: the tallies, the slam for the floor to hear, the log line. (doorblast.ts flySlab)
pub fn fly_slab(g: &mut crate::game::Game, d: DoorId, dt: f64) {
    let i = g.dr[d].i;
    let r = crate::sim_slabshome::slab_store_row_of(g, i);
    if r < 0.0 {
        return;
    }
    let met = match slab_there(g, d) {
        Some(w) => step_there(g, d, r, dt, crate::doorblast::StepThereW { but: w.but, ask: w.ask }),
        None => {
            g.m.sim_slabshome.slabs_stepped.here += 1.0;
            step_here(g, d, r, dt)
        }
    };
    let s = crate::doors::door_slab(g, d).unwrap();
    if s.rest {
        crate::sim_slabshome::slab_store_rest(g, i);
    }
    let a = &mut g.stats.doorblast;
    a.doorblast_nudged += met.nudged as i32;
    a.doorblast_nudge_max = m::max(a.doorblast_nudge_max as f64, met.nudge_max) as i32;
    a.doorblast_laid += met.laid as i32;
    if met.landed {
        g.stats.doorblast.doorblast_landed += 1;
        let at: V3 = [met.lx, 0.1, met.lz];
        let level = g.k.doorblast.land_level;
        let who = s.who;
        without_leaf(g, d, &mut |g: &mut Game| crate::doors::doors_sound(g, d, crate::doors::DoorSound::Bang, at, who, level));
    }
    if met.rested {
        let s = crate::doors::door_slab(g, d).unwrap();
        let line = format!("{} slab at rest ({:.2}, {:.2}) after {:.2} s", g.dr[d].def.name, s.p[0], s.p[2], s.t);
        note(g, &line);
    }
}

/// The row read out, stepped, written back. (doorblast.ts stepHere)
pub fn step_here(g: &mut crate::game::Game, d: DoorId, r: f64, dt: f64) -> crate::doorblast::SlabMet {
    let mut s = crate::sim_slabshome::slab_store_load(g, r);
    let mut met = nothing_met();
    step_slab(g, d, &mut s, dt, &mut met);
    crate::sim_slabshome::slab_store_save(g, r, s);
    met
}

/// Can this slab's step run in the module: the world standing with the game's own leaves in it, which of them is the slab's own box (−1: none), and the pane's rule; None: a pane rule the module cannot judge by. (doorblast.ts slabThere)
pub fn slab_there(g: &mut crate::game::Game, d: DoorId) -> Option<crate::doorblast::SlabThere> {
    let dyn_ = g.col.dynamic_boxes.clone();
    crate::scene_collision::rules_world(g, &dyn_);
    let ask = crate::scene_collision::rules_panes(g);
    if ask < 0.0 {
        return None;
    }
    let b = g.dr[d].box_;
    let but = dyn_.iter().position(|x| *x == b).map_or(-1.0, |i| i as f64);
    Some(crate::doorblast::SlabThere { but, ask })
}

/// The step in the module: the door's facts and the rules' numbers handed over, the step taken, what it met read back. (doorblast.ts stepThere)
pub fn step_there(g: &mut crate::game::Game, d: DoorId, r: f64, dt: f64, w: crate::doorblast::StepThereW) -> crate::doorblast::SlabMet {
    let q = slab_par();
    q[sim::slabs::P_DT] = dt;
    q[sim::slabs::P_ROW] = r;
    q[sim::slabs::P_HEIGHT] = g.dr[d].height;
    q[sim::slabs::P_THICK] = g.dr[d].thick;
    q[sim::slabs::P_HU] = g.dr[d].leaf_h[0];
    q[sim::slabs::P_CLOSED] = g.dr[d].def.closed_dir;
    q[sim::slabs::P_FCX] = g.dr[d].frame_centre[0];
    q[sim::slabs::P_FCZ] = g.dr[d].frame_centre[2];
    q[sim::slabs::P_BUT] = w.but;
    q[sim::slabs::P_ASK] = w.ask;
    q[sim::slabs::P_SCRAPE] = g.k.doorblast.scrape;
    q[sim::slabs::P_SLAM] = g.k.doorblast.slam;
    q[sim::slabs::P_SLIDE] = g.k.doorblast.slide;
    q[sim::slabs::P_BOUNCE] = g.k.doorblast.bounce;
    q[sim::slabs::P_TIP] = g.k.doorblast.tip;
    let a = sim::slabs::step(g.sim, g.col.mirror.slot as usize);
    let c = &mut g.m.sim_slabshome.slabs_stepped;
    c.rays += q[sim::slabs::P_RAYS];
    c.gathers += q[sim::slabs::P_GATHERS];
    c.substeps += q[sim::slabs::P_SUBSTEPS];
    if a == sim::slabs::HOST {
        c.host += 1.0;
        panic!("a slab's step met a ray that is no number");
    }
    if a != sim::slabs::STEPPED {
        panic!("the Rust module was asked to step row {} of the slabs, which is no slab in the air", r);
    }
    c.there += 1.0;
    if q[sim::slabs::P_LANDED] != 0.0 {
        c.landed += 1.0;
    }
    if q[sim::slabs::P_RESTED] != 0.0 {
        c.rested += 1.0;
    }
    crate::doorblast::SlabMet {
        landed: q[sim::slabs::P_LANDED] != 0.0,
        lx: q[sim::slabs::P_LAND_X],
        lz: q[sim::slabs::P_LAND_Z],
        rested: q[sim::slabs::P_RESTED] != 0.0,
        nudged: q[sim::slabs::P_NUDGED],
        nudge_max: q[sim::slabs::P_NUDGE_MAX],
        laid: q[sim::slabs::P_LAID],
    }
}

/// TWIN of sim::slabs::step: `s` put in the door's row, stepped there and read back, what it met added to `t`. (doorblast.ts stepSlab)
pub fn step_slab(g: &mut crate::game::Game, d: DoorId, s: &mut crate::sim_slabshome::SlabNow, dt: f64, t: &mut crate::doorblast::SlabMet) {
    if s.rest {
        return;
    }
    let i = g.dr[d].i;
    let r = crate::sim_slabshome::slab_store_row_of(g, i);
    crate::sim_slabshome::slab_store_save(g, r, *s);
    let w = match slab_there(g, d) {
        Some(w) => crate::doorblast::StepThereW { but: w.but, ask: w.ask },
        None => panic!("a slab's step met a pane's rule the module cannot judge by: no glass stands behind it"),
    };
    let met = step_there(g, d, r, dt, w);
    *s = crate::sim_slabshome::slab_store_load(g, r);
    if met.landed {
        t.landed = true;
        t.lx = met.lx;
        t.lz = met.lz;
    }
    t.rested = t.rested || met.rested;
    t.nudged += met.nudged;
    t.nudge_max = m::max(t.nudge_max, met.nudge_max);
    t.laid += met.laid;
}

/// The slab's box (and the two handles on it) where the slab is: tilted while it topples, then an ordinary yaw-only box lying flat. (doorblast.ts poseSlab)
pub fn pose_slab(g: &mut crate::game::Game, d: DoorId) {
    let i = g.dr[d].i;
    let r = crate::sim_slabshome::slab_store_row_of(g, i);
    if r < 0.0 {
        return;
    }
    let s = crate::sim_slabshome::slab_store_load(g, r);
    let b = g.dr[d].box_;
    let (w, t_, height) = (g.dr[d].def.width, g.dr[d].thick, g.dr[d].height);
    let (ux, uz) = (m::cos(s.a), m::sin(s.a));
    let lx = (w - 0.14) - (w * 0.5 + 0.01);
    let ly = 1.0 - (height / 2.0 + 0.01);
    let lz = t_ / 2.0 + 0.03;
    let handles = g.dr[d].handle_boxes.clone();
    let seal = crate::scene_boxes::box_flag().sound_seal;
    g.col.boxes[b].flags = (js::to_i32(g.col.boxes[b].flags) & !js::to_i32(seal)) as f64;
    g.col.boxes[b].yaw = -s.a;
    crate::scene_box_mirror::leaf_wrote(g);
    if s.landed {
        let hw = match s.bits { Some(bits) => bits[0], None => g.dr[d].leaf_h[0] };
        let hl = match s.bits { Some(bits) => bits[1], None => height / 2.0 };
        let leaf = &mut g.col.boxes[b];
        leaf.rot = None;
        leaf.marks = None;
        leaf.h = [hw, t_ / 2.0, hl];
        leaf.c = [s.p[0], t_ / 2.0 + LIFT, s.p[2]];
        let (tx, tz) = (-uz * s.side, ux * s.side);
        let kx = match s.bits { Some(bits) => bits[2], None => lx };
        let ky = match s.bits { Some(bits) => bits[3], None => ly };
        for hb in handles {
            let handle = &mut g.col.boxes[hb];
            handle.rot = None;
            handle.yaw = -s.a;
            handle.c = [s.p[0] + ux * kx + tx * ky, t_ + LIFT + 0.03, s.p[2] + uz * kx + tz * ky];
        }
        return;
    }
    let q = v3::quat::mul(v3::quat::axis_angle([ux, 0.0, uz], s.side * s.tip), v3::quat::yaw(-s.a));
    let c: V3 = [s.p[0], s.p[1], s.p[2]];
    let leaf_h = g.dr[d].leaf_h;
    let leaf = &mut g.col.boxes[b];
    leaf.rot = Some(q);
    leaf.h = leaf_h;
    leaf.c = c;
    for k in 0..2 {
        let z = if k == 0 { lz } else { -lz };
        let handle = &mut g.col.boxes[handles[k]];
        handle.rot = Some(q);
        handle.yaw = -s.a;
        handle.c = v3::add(c, v3::quat::rotate(q, [lx, ly, z]));
    }
}

/// A slab run on to where it lands and nothing of it kept: at most eight seconds of its step on its own row, with no memory of being wedged; its row and the tallies are put back. (doorblast.ts runOn)
pub fn run_on(g: &mut crate::game::Game, d: DoorId) -> crate::sim_slabshome::SlabNow {
    let i = g.dr[d].i;
    let r = crate::sim_slabshome::slab_store_row_of(g, i);
    let s = crate::sim_slabshome::slab_store_load(g, r);
    if s.rest {
        return s;
    }
    crate::scene_collision::window(g, &mut |g: &mut Game| {
        let was = crate::sim_slabshome::slab_store_load(g, r);
        let tally = g.m.sim_slabshome.slabs_stepped.clone();
        let mut s = was;
        s.wedged = Some(0.0);
        match slab_there(g, d) {
            Some(w) => {
                crate::sim_slabshome::slab_store_save(g, r, s);
                let w = crate::doorblast::StepThereW { but: w.but, ask: w.ask };
                let mut i = 0;
                while i < 480 && !s.rest {
                    step_there(g, d, r, 1.0 / 60.0, w);
                    s = crate::sim_slabshome::slab_store_load(g, r);
                    i += 1;
                }
            }
            None => {
                let mut quiet = nothing_met();
                let mut i = 0;
                while i < 480 && !s.rest {
                    step_slab(g, d, &mut s, 1.0 / 60.0, &mut quiet);
                    i += 1;
                }
            }
        }
        crate::sim_slabshome::slab_store_save(g, r, was);
        g.m.sim_slabshome.slabs_stepped = tally.clone();
        s
    })
}
