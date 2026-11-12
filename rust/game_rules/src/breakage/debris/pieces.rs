use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A kind's row of the table: hearing level and knock.
pub(super) fn landing_of(kind: ChunkKind) -> LANDINGValue {
    landing().of(kind.as_str()).unwrap().clone()
}

pub(super) fn sfx_of(l: &LANDINGValue) -> Vec<ChunkSfxItem> {
    l.sfx.iter().map(|s| ChunkSfxItem { name: s.name.clone(), rate: s.rate, gain: s.gain }).collect()
}

// ---------------------------------------------------------------- the arithmetic of a box's attitude

pub(super) fn rot_of(g: &Game, b: AnyBoxId) -> Quat {
    match g.tab[b].rot {
        Some(r) => r,
        None => v3::quat::yaw(g.tab[b].yaw),
    }
}
fn axes_of(q: Quat) -> [V3; 3] {
    [v3::quat::rotate(q, [1.0, 0.0, 0.0]), v3::quat::rotate(q, [0.0, 1.0, 0.0]), v3::quat::rotate(q, [0.0, 0.0, 1.0])]
}
fn support_of(r: Quat, h: V3, n: V3) -> f64 {
    let ax = axes_of(r);
    m::abs(v3::dot(ax[0], n)) * h[0] + m::abs(v3::dot(ax[1], n)) * h[1] + m::abs(v3::dot(ax[2], n)) * h[2]
}
fn face_tilt_of(q: Quat, h: V3, n: V3) -> f64 {
    let axes = axes_of(q);
    let (mut bi, mut bd) = (0usize, -1.0);
    for i in 0..3 {
        let d = m::abs(v3::dot(axes[i], n)) / m::max(0.01, h[i]);
        if d > bd { bd = d; bi = i; }
    }
    m::acos(m::min(1.0, m::abs(v3::dot(axes[bi], n))))
}
fn mass_from(h: V3) -> f64 { m::min(6.0, m::max(0.1, h[0] * h[1] * h[2] * 8.0 * 600.0)) }

/// 0‥1 off an integer (a hash, not a die).
pub fn hash01(x: f64) -> f64 {
    let mut x = (js::to_i32(x) ^ (0x9E3779B9u32 as i32)) as u32;
    x = x.wrapping_mul(0x85EBCA6B);
    x ^= x >> 13;
    x = x.wrapping_mul(0xC2B2AE35);
    x ^= x >> 16;
    ((x >> 8) as f64) / 16777216.0
}

/// A new pool.
pub(in crate::breakage) fn debris_new() -> crate::debris::Debris { crate::debris::Debris::blank() }

/// The pool's size.
pub const DEBRIS_CAP: f64 = 64.0;

/// Thrown and not yet down; a piece a boot slid is not counted.
pub(in crate::breakage) fn debris_moving(g: &crate::game::Game) -> f64 {
    let mut n = 0.0;
    for &c in g.destr.debris.list.iter() {
        let ch = &g.tab.debris_chunk[c];
        if !ch.rest && ch.foot != Some(true) { n += 1.0; }
    }
    n
}

/// A chunk of `kind` thrown at `v` turning at `w`; None when the pool is full of chunks still moving.
pub fn debris_spawn(g: &mut crate::game::Game, kind: crate::debris::ChunkKind, box_: BoxId, v: V3, w: V3, who: Option<f64>, level: Option<f64>) -> Option<DebrisChunkId> {
    let who = who.unwrap_or(crate::consts::NOBODY_ID);
    let level = level.unwrap_or(0.0);
    let mut v = v;
    if let Some(f) = g.destr.debris.fling { v = v3::add(v, f); }
    debris_thrown(g, kind, box_, v, level, who);
    debris_spawn_raw(g, kind, box_, v, w, Some(who))
}

/// The simulation's half of a piece that is thrown: its one landing, put in the game's falls.
fn debris_thrown(g: &mut crate::game::Game, kind: crate::debris::ChunkKind, box_: BoxId, v: V3, level: f64, who: f64) {
    let c = g.tab.boxes[box_].c;
    let f = crate::falling::flight_of(g, c, v, G, crate::falling::FallWorld::StaticsAndProps);
    let down = m::sqrt(v[1] * v[1] + 2.0 * G * m::max(0.0, c[1] - f.pos[1]));
    if !(down > g.k.debris.min_speed) { return; }
    let l = crate::break_table::breaks_lands(g, kind, box_.into(), down);
    let mut lv = level;
    if !js::truthy(lv) { lv = l.level; }
    if !js::truthy(lv) { lv = landing_of(kind).level; }
    let t = g.time + f.t;
    if crate::falls::schedule(&mut g.falls, crate::falls::FallKind::Pieces, crate::falls::Landing { t, pos: f.pos, sound: crate::falls::FallSound::Empty, gain: 0.0, rate: 1.0, amount: 0.0, who, told: crate::falls::FallTold::Always, level: lv, dust: l.dust }, crate::falls::MAX_WAITING as f64) {
        g.destr.debris.stats.told += 1.0;
    }
}

/// The thrown pieces whose landing has come: the floor is told of each.
pub(in crate::breakage) fn debris_landings(g: &mut crate::game::Game) {
    let time = g.time;
    let due = crate::falls::due(&mut g.falls, crate::falls::FallKind::Pieces, time, 0.0, 0.0);
    for p in due {
        if p.heard {
            let (time, serial) = (g.time, g.serial());
            g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Prop, pos: p.pos, time, loud: false, level: Some(p.level), who: Some(p.who), serial, ..crate::game::GameEvent::default() });
        }
        if p.dust > 0.0 {
            crate::destruction::destruction_cell_dust(g, p.pos, [0.0, 1.0, 0.0], p.dust, crate::scene_materials::Surf::Drywall);
        }
    }
}

/// spawn() without the glass door; `told` = the simulation has this piece's landing.
pub fn debris_spawn_raw(g: &mut crate::game::Game, kind: crate::debris::ChunkKind, box_: BoxId, v: V3, w: V3, who: Option<f64>) -> Option<DebrisChunkId> {
    let who = who.unwrap_or(crate::consts::NOBODY_ID);
    while (g.destr.debris.list.len() as f64) >= DEBRIS_CAP {
        let list = g.destr.debris.list.clone();
        let mut ok = |g: &mut Game, c: DebrisChunkId| evictable(g, c);
        let i = crate::break_table::breaks_pick(g, &list, &mut ok);
        if i < 0.0 { g.destr.debris.stats.refused += 1.0; return None; }
        let gone = g.destr.debris.list.remove(i as usize);
        g.destr.debris.stats.evicted += 1.0;
        crate::break_table::breaks_evicted(g, gone);
    }
    let serial = g.destr.debris.serial;
    g.destr.debris.serial += 1.0;
    let (c0, h0, yaw, rot0, albedo, stuff, does) = {
        let bx = &g.tab.boxes[box_];
        (bx.c, bx.h, bx.yaw, bx.rot, bx.albedo, bx.stuff.clone(), bx.does.clone())
    };
    let rot = match rot0 { Some(r) => r, None => v3::quat::yaw(yaw) };
    let word = crate::scene_boxes::word_of(crate::scene_boxes::WordOfB { stuff, does });
    let p = crate::scene_boxes::MakeBoxP {
        c: v3::copy(c0),
        h: v3::copy(h0),
        yaw: Some(yaw),
        rot: Some(rot),
        albedo: Some(v3::copy(albedo)),
        stuff: word.stuff,
        does: word.does,
        flags: Some(crate::scene_boxes::box_flag::DYNAMIC as f64),
        mkey: Some(crate::scene_boxes::mkey().debris + ((js::to_i32(serial) & 65535) as f64)),
        ..crate::scene_boxes::MakeBoxP::default()
    };
    let b = crate::scene_boxes::make_box(g, &p);
    let l = landing_of(kind);
    let id = g.tab.debris_chunk.push(Chunk {
        kind, box_: b.into(), v: v3::copy(v), w: v3::copy(w), rest: false, still_t: 0.0, age: 0.0, sounded: false,
        level: l.level, sfx: sfx_of(&l), serial, who, on: None, on_pose: 0.0,
        dress: Some(true),
        ..Chunk::default()
    });
    g.destr.debris.list.push(id);
    g.destr.debris.stats.spawned += 1.0;
    crate::break_table::breaks_born(g, id);
    Some(id)
}

/// Back to an empty pool; `serial` runs on.
pub(in crate::breakage) fn debris_clear(g: &mut crate::game::Game) {
    crate::falls::clear(&mut g.falls, crate::falls::FallKind::Pieces);
    let d = &mut g.destr.debris;
    d.fling = None;
    d.list.clear();
    d.impulses = 0.0;
    d.serial0 = d.serial;
    d.shoves = 0.0;
    d.scuff_at = -1.0;
    d.feet_stats = DebrisFeetStats { kicked: 0.0, off_man: 0.0, walled: 0.0, gave_up: 0.0 };
}

/// The pool's boxes for the picture; a mate rides its chunk's pose.
pub(in crate::breakage) fn debris_boxes(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>) {
    let list = g.destr.debris.list.clone();
    for c in list {
        let (b, dress, mate) = { let ch = &g.tab.debris_chunk[c]; (ch.box_, ch.dress == Some(true), ch.mate) };
        if dress { g.tab.edit(b).dress = Some(true); }
        out.push(b);
        if let Some(mt) = mate {
            let (bc, by, br, bd) = { let bx = &g.tab[b]; (bx.c, bx.yaw, bx.rot, bx.dress) };
            let mut row = g.tab.edit(mt);
            row.c = bc;
            row.yaw = by;
            row.rot = br;
            row.dress = bd;
            out.push(mt.into());
        }
    }
}

/// An impulse over the pool: wakes, shoves and turns every chunk within reach; how many it moved.
pub(in crate::breakage) fn debris_impulse(g: &mut crate::game::Game, imp: crate::sheet_model::ImpulseLike, blocked: Option<&mut dyn FnMut(&mut crate::game::Game, V3) -> bool>, who: Option<f64>) -> f64 {
    if g.destr.debris.list.is_empty() { return 0.0; }
    let mut blocked = blocked;
    let mut n = 0.0;
    g.destr.debris.impulses += 1.0;
    let k = g.destr.debris.impulses;
    let di = debris_imp();
    let resp = crate::sheet_model::ImpulseResponse { gain: di.gain, up_k: di.up_k };
    let mut i = 0;
    while i < g.destr.debris.list.len() {
        let c = g.destr.debris.list[i];
        i += 1;
        if g.tab.debris_chunk[c].age <= 0.0 { continue; }
        let b = g.tab.debris_chunk[c].box_;
        let bc = g.tab[b].c;
        let kick = match crate::sheet_model::impulse_kick(imp, bc, resp, None) { Some(x) => x, None => continue };
        if kick.s < di.wake { continue; }
        if let Some(f) = blocked.as_mut() {
            if v3::dist(bc, imp.pos) > 0.5 && (*f)(g, bc) { continue; }
        }
        let tw = di.tumble * kick.s / m::max(0.08, bound_r(g, b));
        let ch = &mut g.tab.debris_chunk[c];
        if ch.rest { ch.rest = false; ch.on = None; ch.still_t = 0.0; }
        if ch.foot == Some(true) { ch.foot = Some(false); }
        let mut v = v3::add(ch.v, kick.dv);
        let sp = v3::len(v);
        if sp > di.v_max { v = v3::scale(v, di.v_max / sp); }
        ch.v = v;
        let serial = ch.serial;
        let h = |j: f64| hash01(serial * 977.0 + k * 131.0 + j) - 0.5;
        ch.w = v3::add(ch.w, [h(1.0) * tw, h(2.0) * tw * 0.5, h(3.0) * tw]);
        ch.sounded = false;
        if let Some(w) = who { ch.who = w; }
        crate::break_table::breaks_kicked(g, c, kick.s);
        n += 1.0;
    }
    n
}

/// The pool's frame: the moving fly or slide, the resting are checked for what is under them.
pub(in crate::breakage) fn debris_update(g: &mut crate::game::Game, dt: f64) {
    if g.destr.debris.list.is_empty() { return; }
    g.destr.debris.frame += 1.0;
    for i in (0..g.destr.debris.list.len()).rev() {
        let c = g.destr.debris.list[i];
        g.tab.debris_chunk[c].age += dt;
        if g.tab.debris_chunk[c].rest {
            let (b, on, on_pose, serial) = { let ch = &g.tab.debris_chunk[c]; (ch.box_, ch.on, ch.on_pose, ch.serial) };
            let bc = g.tab[b].c;
            if let Some(p) = on {
                // on furniture: it drops again once that piece has moved out from under it
                if pose_key(g, p) != on_pose {
                    let reach = support_along(g, b, [0.0, 1.0, 0.0]) + 0.12;
                    let under = crate::props::prop_system_raycast(g, [bc[0], bc[1] + 0.02, bc[2]], [0.0, -1.0, 0.0], reach);
                    let same = match &under { Some(u) => u.prop == p, None => false };
                    if same {
                        let k = pose_key(g, p);
                        g.tab.debris_chunk[c].on_pose = k;
                    } else {
                        let ch = &mut g.tab.debris_chunk[c];
                        ch.rest = false;
                        ch.on = None;
                        ch.still_t = 0.0;
                    }
                }
            } else if (js::to_i32(g.destr.debris.frame + serial) & 31) == 0 {
                // on a static: a staggered down-ray now and then catches the static going away under it
                debris_resettle(g, c);
            }
            continue;
        }
        let sub = 2;
        let h = dt / sub as f64;
        let mut s = 0;
        while s < sub && !g.tab.debris_chunk[c].rest {
            if g.tab.debris_chunk[c].foot == Some(true) { debris_slide(g, c, h); } else { debris_step(g, c, h); }
            s += 1;
        }
        let b = g.tab.debris_chunk[c].box_;
        let y = g.tab[b].c[1];
        if y < -1.0 || g.tab.debris_chunk[c].age > 600.0 {
            if i < g.destr.debris.list.len() { g.destr.debris.list.remove(i); }
            if g.tab.debris_chunk[c].glass.is_some() { crate::shards::glass_shards_gone(g, c); }
        }
    }
}

pub(super) const UP: V3 = [0.0, 1.0, 0.0];

/// A box put at that height.
pub(super) fn box_at_y(g: &mut Game, b: AnyBoxId, y: f64) { g.tab.edit(b).c[1] = y; }

fn box_add_c(g: &mut Game, b: AnyBoxId, d: V3) {
    let mut c = g.tab[b].c;
    c[0] += d[0];
    c[1] += d[1];
    c[2] += d[2];
    g.tab.edit(b).c = c;
}

/// A thrown chunk for `h` seconds: ballistic flight with a swept contact.
fn debris_step(g: &mut Game, c: DebrisChunkId, h: f64) {
    let b = g.tab.debris_chunk[c].box_;
    g.tab.debris_chunk[c].v[1] -= G * h;
    spins(g, c, b, h);
    let mv = v3::scale(g.tab.debris_chunk[c].v, h);
    let len = v3::len(mv);
    if len < 1e-7 { return debris_settle_tick(g, c, h, None); }
    let mut landed = false;
    match meets(g, b, v3::scale(mv, 1.0 / len), len) {
        None => box_add_c(g, b, mv),
        Some((by, hit)) => {
            box_add_c(g, b, by);
            landed = bounces(g, c, b, &hit);
            if landed && still(g, c) { return debris_sleep(g, c, hit.n, hit.thing); }
            box_add_c(g, b, [hit.n[0] * 0.002, hit.n[1] * 0.002, hit.n[2] * 0.002]);
        }
    }
    if on_the_floor(g, c, b) && still(g, c) { return debris_sleep(g, c, UP, None); }
    debris_settle_tick(g, c, h, if landed { Some(UP) } else { None });
}

/// Turned by its spin.
fn spins(g: &mut Game, c: DebrisChunkId, b: AnyBoxId, h: f64) {
    let w = g.tab.debris_chunk[c].w;
    let wl = v3::len(w);
    if !(wl > 1e-5) { return; }
    let a = wl * h;
    let s = m::sin(a / 2.0) / wl;
    let r = rot_of(g, b);
    g.tab.edit(b).rot = Some(v3::quat::normalize(v3::quat::mul([w[0] * s, w[1] * s, w[2] * s, m::cos(a / 2.0)], r)));
}

/// The swept contact: the centre ray, met `support` short of the surface. How far it goes to it, and what it is.
fn meets(g: &Game, b: AnyBoxId, dir: V3, len: f64) -> Option<(V3, crate::props::LevelRay)> {
    let reach = len + bound_r(g, b) * 1.05;
    let hit = crate::props::level_ray(g, g.tab[b].c, dir, reach)?;
    let back = support_along(g, b, hit.n) / m::max(0.35, -v3::dot(dir, hit.n));
    if !(hit.t - back <= len) { return None; }
    let t = m::max(0.0, hit.t - back);
    Some(([dir[0] * t, dir[1] * t, dir[2] * t], hit))
}

/// Into a surface: furniture takes its share, and it comes off slower, laid toward its flattest face by a floor. True: it landed on one.
fn bounces(g: &mut Game, c: DebrisChunkId, b: AnyBoxId, hit: &crate::props::LevelRay) -> bool {
    let n = hit.n;
    let vn = v3::dot(g.tab.debris_chunk[c].v, n);
    if !(vn < 0.0) { return false; }
    if let Some(p) = hit.thing.filter(|_| g.tab.debris_chunk[c].dress != Some(true)) {
        let j = v3::scale(n, vn * 0.4 * mass_of(g, b));
        crate::props::prop_system_apply_impulse(g, p, g.tab[b].c, j, 0.0, None);
    }
    let floor = n[1] > 0.6;
    let v_n = v3::scale(n, vn);
    let v_t = v3::sub(g.tab.debris_chunk[c].v, v_n);
    let (keep, back) = match &g.tab.debris_chunk[c].contact {
        Some(x) => (if floor { x.keep_floor } else { x.keep_wall }, -x.rest),
        None => (if floor { 0.55 } else { 0.8 }, -0.22),
    };
    g.tab.debris_chunk[c].v = v3::add(v3::scale(v_t, keep), v3::scale(v_n, back));
    // a floor touch turns it toward lying on the face nearest down, and trades some of the knock into a wobble
    if floor {
        lay_flat(g, b, n, 0.45);
        let kick = v3::scale(v3::cross(n, v_t), 2.0 / m::max(0.08, bound_r(g, b)));
        let ch = &mut g.tab.debris_chunk[c];
        ch.w = v3::add(v3::scale(ch.w, 0.45), kick);
    } else {
        let ch = &mut g.tab.debris_chunk[c];
        ch.w = v3::scale(ch.w, 0.7);
    }
    lands(g, c, -vn);
    let ch = &mut g.tab.debris_chunk[c];
    if floor && m::abs(v3::dot(ch.v, n)) < 0.5 && v3::len(ch.v) < SLEEP_V * 2.0 {
        ch.v = v3::scale(ch.v, 0.5);
        ch.w = v3::scale(ch.w, 0.5);
    }
    floor
}

/// The world's floor as a last resort: is it on that.
fn on_the_floor(g: &mut Game, c: DebrisChunkId, b: AnyBoxId) -> bool {
    let sup = support_along(g, b, UP);
    if !(g.tab[b].c[1] < sup) { return false; }
    box_at_y(g, b, sup + 0.001);
    let ch = &mut g.tab.debris_chunk[c];
    let down = -ch.v[1];
    if down > 0.0 {
        ch.v[1] *= -0.2;
        ch.v[0] *= 0.6;
        ch.v[2] *= 0.6;
        lay_flat(g, b, UP, 0.5);
        let ch = &mut g.tab.debris_chunk[c];
        ch.w = v3::scale(ch.w, 0.5);
        lands(g, c, down);
    }
    true
}

/// Its first hard touch is told.
fn lands(g: &mut Game, c: DebrisChunkId, speed: f64) {
    if !g.tab.debris_chunk[c].sounded && speed > 1.0 { debris_land(g, c, speed); }
}

/// As slow as a piece sleeps at.
fn still(g: &Game, c: DebrisChunkId) -> bool {
    let ch = &g.tab.debris_chunk[c];
    v3::len(ch.v) < SLEEP_V && v3::len(ch.w) < SLEEP_W
}

/// Nearly still for SLEEP_T while touching a floor: asleep.
fn debris_settle_tick(g: &mut crate::game::Game, c_2: DebrisChunkId, h: f64, floor_n: Option<V3>) {
    let c = c_2;
    let (lv, lw) = { let ch = &g.tab.debris_chunk[c]; (v3::len(ch.v), v3::len(ch.w)) };
    match floor_n {
        Some(n) if lv < SLEEP_V * 1.6 && lw < SLEEP_W * 1.6 => {
            g.tab.debris_chunk[c].still_t += h;
            if g.tab.debris_chunk[c].still_t > SLEEP_T { debris_sleep(g, c, n, None); }
        }
        _ => {
            let ch = &mut g.tab.debris_chunk[c];
            ch.still_t = m::max(0.0, ch.still_t - h);
        }
    }
}

/// Lays it flat and puts it to rest on what is under it; still far off its face, another settling turn instead.
pub(super) fn debris_sleep(g: &mut crate::game::Game, c_2: DebrisChunkId, n: V3, on: Option<PropId>) {
    let c = c_2;
    let b = g.tab.debris_chunk[c].box_;
    if face_tilt(g, b, n) > 0.35 {
        lay_flat(g, b, n, 0.45);
        g.tab.debris_chunk[c].still_t = 0.0;
        return;
    }
    lay_flat(g, b, n, 1.0);
    {   // (a sheet does not stand on its edge; but one that going down would put through a wall it is clear of leans on that wall)
        let clear = !debris_foot_in(g, b);
        let (r0, y0) = (g.tab[b].rot, g.tab[b].yaw);
        lay_sheet(g, b, n);
        if clear && debris_foot_in(g, b) { g.tab.edit(b).rot = r0; g.tab.edit(b).yaw = y0; }
    }
    let on_pose = match on { Some(p) => pose_key(g, p), None => 0.0 };
    {
        let ch = &mut g.tab.debris_chunk[c];
        ch.rest = true;
        ch.v = [0.0, 0.0, 0.0];
        ch.w = [0.0, 0.0, 0.0];
        ch.on = on;
        ch.on_pose = on_pose;
        if ch.foot == Some(true) { ch.foot = Some(false); }
    }
    // sit exactly on the surface under it, and on whatever already lies there
    let (seat, thing) = debris_seat(g, c, false);
    if let Some(y) = seat { box_at_y(g, b, y); }
    if let Some(p) = thing {
        let k = pose_key(g, p);
        let ch = &mut g.tab.debris_chunk[c];
        ch.on = Some(p);
        ch.on_pose = k;
    }
    if g.tab.debris_chunk[c].glass.is_some() { crate::shards::glass_shards_rested(g, c); }
}

/// How high a thing may stand off its surface and still hold up a piece that merely overlaps it (m). Anything higher holds up only a piece whose middle is over it.
const HAND: f64 = 0.12;

/// How high above a surface a thing may lie and still be of that surface's pile (m): a thing higher up than this is on something else, and a piece goes under it.
const PILE: f64 = 0.3;

/// WHERE A PIECE LIES: its centre's height on the surface under it, or on the top of what already lies on that surface over its footprint, so that no two pieces lie
/// through each other. None = nothing under it. What it found under it, if that is a prop. A piece just landing (it has come through whatever lies there) goes on top of
/// the pile; a piece already `seated` asks only about what is beneath it.
pub(super) fn debris_seat(g: &Game, c: DebrisChunkId, seated: bool) -> (Option<f64>, Option<PropId>) {
    let b = g.tab.debris_chunk[c].box_;
    let sup = support_along(g, b, [0.0, 1.0, 0.0]);
    let bc = g.tab[b].c;
    let from: V3 = [bc[0], bc[1] + 0.05, bc[2]];
    let reach = sup + (if seated { 1.0 } else { 0.4 });   // (seated, it may lie on a pile well over the surface)
    let Some(under) = crate::props::level_ray(g, from, [0.0, -1.0, 0.0], reach) else { return (None, None) };
    let surface = from[1] - under.t;
    let top = pile_top(g, b, surface, seated, true);
    (Some(top + sup + 0.0015 + g.tab.debris_chunk[c].lift.unwrap_or(0.0)), under.thing)
}

/// The top of what lies on `surface` over box b's footprint; the surface itself where nothing does. `rubble`: the wall's rubble counts as well as the pool's pieces.
fn pile_top(g: &Game, b: AnyBoxId, surface: f64, seated: bool, rubble: bool) -> f64 {
    let mine = footprint(g, b);
    let (bc, sup) = (g.tab[b].c, mine.hy);
    let mut top = surface;
    let lies: Vec<AnyBoxId> = g.destr.debris.list.iter().map(|&o| &g.tab.debris_chunk[o]).filter(|o| o.rest && o.glass.is_none() && o.box_ != b).map(|o| o.box_)
        .chain(g.destr.edge_look.rubble.iter().filter(|r| rubble && r.t >= r.t_2 && AnyBoxId::Frame(r.box_) != b).map(|r| AnyBoxId::Frame(r.box_))).collect();
    for o in lies {
        let oc = g.tab[o].c;
        let (ob, oh) = (v3::len(g.tab[o].h), v3::len(g.tab[b].h));
        if m::abs(oc[0] - bc[0]) > ob + oh || m::abs(oc[2] - bc[2]) > ob + oh { continue; }
        let theirs = footprint(g, o);
        if oc[1] - theirs.hy > surface + PILE || !footprints_meet(bc, &mine, oc, &theirs) { continue; }
        let high = oc[1] + theirs.hy;
        if seated && high > bc[1] - sup + 0.005 { continue; }
        if high > surface + HAND && !footprint_has(oc, &theirs, bc) { continue; }
        top = m::max(top, high);
    }
    top
}

/// Where the rubble of a wall that has just come down lies: at `y` as it was dealt, or on the pool's pieces if some already lie there.
pub(in crate::breakage) fn debris_rubble_height(g: &crate::game::Game, b: AnyBoxId, y: f64) -> f64 {
    if g.destr.debris.list.is_empty() { return y; }
    let sup = support_along(g, b, [0.0, 1.0, 0.0]);
    m::max(y, pile_top(g, b, y - sup, false, false) + sup + 0.0015)
}

/// A piece at rest on a static or a pile asks now and then what it lies on: gone a little way, it settles there; further, it falls.
pub(in crate::breakage) fn debris_resettle(g: &mut crate::game::Game, c: DebrisChunkId) {
    let b = g.tab.debris_chunk[c].box_;
    let sup = support_along(g, b, [0.0, 1.0, 0.0]);
    let bc = g.tab[b].c;
    if bc[1] <= sup + 0.02 { return; }
    match debris_seat(g, c, true).0 {
        Some(y) if bc[1] - y < 0.1 => box_at_y(g, b, y),
        _ => {
            let ch = &mut g.tab.debris_chunk[c];
            ch.rest = false;
            ch.still_t = 0.0;
        }
    }
}

/// A box's plan on the floor: a rectangle along its most level long axis, half extents e1 and e2, and its half height.
#[derive(Clone, Copy)]
pub(super) struct Footprint { pub d1x: f64, pub d1z: f64, pub e1: f64, pub e2: f64, pub hy: f64 }
pub(super) fn footprint(g: &Game, b: AnyBoxId) -> Footprint {
    let bh = g.tab[b].h;
    let a = axes_of(rot_of(g, b));
    let mut iu = 0usize;
    for i in 1..3 { if m::abs(a[i][1]) > m::abs(a[iu][1]) { iu = i; } }
    let (i1, i2) = ((iu + 1) % 3, (iu + 2) % 3);
    let il = if bh[i1] >= bh[i2] { i1 } else { i2 };
    let (mut d1x, mut d1z) = (a[il][0], a[il][2]);
    let dl = m::hyp2(d1x, d1z);
    if dl < 1e-4 { d1x = 1.0; d1z = 0.0; } else { d1x /= dl; d1z /= dl; }
    let (d2x, d2z) = (-d1z, d1x);
    let (mut e1, mut e2, mut hy) = (0.0, 0.0, 0.0);
    for i in 0..3 {
        e1 += m::abs(a[i][0] * d1x + a[i][2] * d1z) * bh[i];
        e2 += m::abs(a[i][0] * d2x + a[i][2] * d2z) * bh[i];
        hy += m::abs(a[i][1]) * bh[i];
    }
    Footprint { d1x, d1z, e1, e2, hy }
}
/// Does a piece's footprint reach into a static? Level rays from its middle to its four corners.
pub(super) fn debris_foot_in(g: &Game, b: AnyBoxId) -> bool {
    let Footprint { d1x, d1z, e1, e2, hy } = footprint(g, b);
    let l = m::hyp2(e1, e2);
    if l < 0.06 { return false; }
    let bc = g.tab[b].c;
    let o: V3 = [bc[0], m::max(bc[1], bc[1] - hy + 0.03), bc[2]];
    [(1.0, 1.0), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)].into_iter().any(|(su, sv)| {
        let x = (d1x * e1 * su - d1z * e2 * sv) / l;
        let z = (d1z * e1 * su + d1x * e2 * sv) / l;
        crate::scene_collision::raycast(g, o, [x, 0.0, z], l).is_some()
    })
}
/// Does a plan cover a point?
fn footprint_has(ac: V3, a: &Footprint, p: V3) -> bool {
    let (dx, dz) = (p[0] - ac[0], p[2] - ac[2]);
    m::abs(dx * a.d1x + dz * a.d1z) <= a.e1 && m::abs(-dx * a.d1z + dz * a.d1x) <= a.e2
}
/// Do two plans overlap? The separating axes of two rectangles.
fn footprints_meet(ac: V3, a: &Footprint, bc: V3, b: &Footprint) -> bool {
    let (dx, dz) = (bc[0] - ac[0], bc[2] - ac[2]);
    let reach = |x: f64, z: f64, f: &Footprint| m::abs(x * f.d1x + z * f.d1z) * f.e1 + m::abs(-x * f.d1z + z * f.d1x) * f.e2;
    for (x, z) in [(a.d1x, a.d1z), (-a.d1z, a.d1x), (b.d1x, b.d1z), (-b.d1z, b.d1x)] {
        if m::abs(dx * x + dz * z) >= reach(x, z, a) + reach(x, z, b) { return false; }
    }
    true
}

/// A chunk's first hard touch: its knock, and the floor told unless it is dressing.
fn debris_land(g: &mut crate::game::Game, c_2: DebrisChunkId, speed: f64) {
    let c = c_2;
    g.tab.debris_chunk[c].sounded = true;
    g.destr.debris.stats.landed += 1.0;
    crate::break_table::breaks_landed(g, c, speed);
    if g.tab.debris_chunk[c].glass.is_some() && crate::shards::glass_shards_landed(g, c, speed) { return; }
    let b = g.tab.debris_chunk[c].box_;
    let p = v3::copy(g.tab[b].c);
    let (sfx, serial) = { let ch = &g.tab.debris_chunk[c]; (ch.sfx.clone(), ch.serial) };
    for s in sfx.iter() {
        let gain = m::min(1.0, s.gain * (0.6 + speed * 0.15));
        crate::audio_iface::game_audio_like_play(g, &s.name, Some(p), Some(gain), Some(&crate::audio_iface::PlayOpts::Rate { rate: s.rate * (0.94 + 0.12 * hash01(serial * 31.0)) }));
    }
    if g.tab.debris_chunk[c].dress != Some(true) {
        let (time, level, who, ev) = (g.time, g.tab.debris_chunk[c].level, g.tab.debris_chunk[c].who, g.serial());
        g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Prop, pos: p, time, loud: false, level: Some(level), who: Some(who), serial: ev, ..crate::game::GameEvent::default() });
    }
}

/// The box's half extent along world direction n.
pub(in crate::breakage) fn support_along(g: &crate::game::Game, b: AnyBoxId, n: V3) -> f64 {
    let r = rot_of(g, b);
    support_of(r, g.tab[b].h, n)
}

/// A slot the pool may take: at rest, or one only a boot has set sliding.
fn evictable(g: &crate::game::Game, c_2: DebrisChunkId) -> bool {
    let ch = &g.tab.debris_chunk[c_2];
    ch.rest || ch.foot == Some(true)
}

fn bound_r(g: &crate::game::Game, b: AnyBoxId) -> f64 { v3::len(g.tab[b].h) }

/// A nominal mass off the volume, clamped.
fn mass_of(g: &crate::game::Game, b: AnyBoxId) -> f64 { mass_from(g.tab[b].h) }

/// Radians between n and the box axis layFlat would settle onto it.
pub(super) fn face_tilt(g: &crate::game::Game, b: AnyBoxId, n: V3) -> f64 {
    let q = rot_of(g, b);
    face_tilt_of(q, g.tab[b].h, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every piece's dealt numbers come off this. A failure means numbers out of their range, all one way, or a piece of a different shape on two machines.
    #[test]
    fn a_hash_lands_in_the_unit_range_and_one_whole_word_has_one_answer() {
        let mut sum = 0.0;
        for i in 0..2000 {
            let h = hash01(i as f64);
            assert!((0.0..1.0).contains(&h), "{i}: {h}");
            sum += h;
        }
        assert!((sum / 2000.0 - 0.5).abs() < 0.03, "{}", sum / 2000.0);
        assert_ne!(hash01(1.0), hash01(2.0));
        assert_eq!(hash01(2.5), hash01(2.0));
    }

    const SLAB: V3 = [0.3, 0.015, 0.2];
    const STILL: Quat = [0.0, 0.0, 0.0, 1.0];
    fn about_x(a: f64) -> Quat { [m::sin(a / 2.0), 0.0, 0.0, m::cos(a / 2.0)] }
    fn near(a: f64, b: f64) { assert!((a - b).abs() < 1e-12, "{a} {b}"); }

    /// A failure means a piece sinks into the floor or hangs over it.
    #[test]
    fn a_box_stands_off_a_surface_by_its_half_size_along_the_normal() {
        near(support_of(STILL, SLAB, UP), 0.015);
        near(support_of(v3::quat::yaw(0.9), SLAB, UP), 0.015);    // turned on the spot
        near(support_of(about_x(m::PI / 2.0), SLAB, UP), 0.2);    // up on its edge
        let a = 0.5;
        near(support_of(about_x(a), SLAB, UP), 0.015 * m::cos(a) + 0.2 * m::sin(a));
    }

    #[test]
    fn a_slab_s_tilt_is_read_off_its_face() {
        near(face_tilt_of(STILL, SLAB, UP), 0.0);
        near(face_tilt_of(about_x(0.3), SLAB, UP), 0.3);
        assert!(face_tilt_of(about_x(m::PI), SLAB, UP) < 1e-7, "upside down is flat");
        near(face_tilt_of(about_x(1.4), SLAB, UP), 1.4);    // nearly on edge: still its face
    }

    #[test]
    fn a_piece_s_mass_goes_by_its_volume_between_two_limits() {
        near(mass_from([0.1, 0.02, 0.1]), 0.96);
        assert_eq!(mass_from([0.3, 0.015, 0.3]), 6.0);
        assert_eq!(mass_from([0.01, 0.01, 0.01]), 0.1);
    }
}
