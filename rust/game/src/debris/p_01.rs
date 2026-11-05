//! src/game/debris.ts lines 1 to 450
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A kind's row of the table: hearing level and knock.   (debris.ts LANDING)
fn landing_of(kind: ChunkKind) -> LANDINGValue {
    landing().get(String::from(kind.as_str())).unwrap().clone()
}

fn sfx_of(l: &LANDINGValue) -> Vec<ChunkSfxItem> {
    l.sfx.iter().map(|s| ChunkSfxItem { name: s.name.clone(), rate: s.rate, gain: s.gain }).collect()
}

// ---------------------------------------------------------------- seams: what skeleton 3 still cannot do

/// g.audio.play(name, pos, gain, { rate }): `opts` of game_audio_like_play holds `crate::Opaque`, which cannot carry the rate.
fn seam_audio_play_rate(g: &mut Game, name: &str, pos: V3, gain: f64, rate: f64) { todo!("needs: game_audio_like_play's opts to carry a rate") }
/// settings.debrisCap: RenderSettings has no such field, though settings.ts:254 declares it.
fn seam_settings_debris_cap(g: &mut Game) -> Option<f64> { todo!("needs: RenderSettings.debris_cap") }

// ---------------------------------------------------------------- the arithmetic of a box's attitude

fn rot_of(g: &mut Game, b: BoxId) -> Quat {
    match g.col.boxes[b].rot {
        Some(r) => r,
        None => v3::quat::yaw(g.col.boxes[b].yaw),
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
fn bound_of(h: V3) -> f64 { m::hyp3(h[0], h[1], h[2]) }
fn mass_from(h: V3) -> f64 { m::min(6.0, m::max(0.1, h[0] * h[1] * h[2] * 8.0 * 600.0)) }

fn is_on(x: Option<bool>) -> bool { x == Some(true) }

/// 0‥1 off an integer (a hash, not a die).   (debris.ts hash01)
pub fn hash01(x: f64) -> f64 {
    let mut x = (js::to_i32(x) ^ (0x9E3779B9u32 as i32)) as u32;
    x = x.wrapping_mul(0x85EBCA6B);
    x ^= x >> 13;
    x = x.wrapping_mul(0xC2B2AE35);
    x ^= x >> 16;
    ((x >> 8) as f64) / 16777216.0
}

/// A new pool.   (debris.ts Debris.constructor)
pub fn debris_new(g: &mut crate::game::Game) -> crate::debris::Debris { crate::debris::Debris::blank() }

/// The pool's size on this tier; 64 without one.   (debris.ts Debris.cap)
pub fn debris_cap(g: &mut crate::game::Game) -> f64 {
    let c = seam_settings_debris_cap(g);
    m::max(4.0, m::floor(c.unwrap_or(64.0)))
}

/// Thrown and not yet down; a piece a boot slid is not counted.   (debris.ts Debris.moving)
pub fn debris_moving(g: &mut crate::game::Game) -> f64 {
    let mut n = 0.0;
    for &c in g.destr.debris.list.iter() {
        let ch = &g.tab.debris_chunk[c];
        if !ch.rest && !is_on(ch.foot) { n += 1.0; }
    }
    n
}

/// A chunk of `kind` thrown at `v` turning at `w`; None when the pool is full of chunks still moving.   (debris.ts Debris.spawn)
pub fn debris_spawn(g: &mut crate::game::Game, kind: crate::debris::ChunkKind, box_: BoxId, v: V3, w: V3, who: Option<f64>, level: Option<f64>) -> Option<DebrisChunkId> {
    let who = who.unwrap_or(crate::consts::NOBODY_ID);
    let level = level.unwrap_or(0.0);
    let mut v = v;
    if kind == ChunkKind::Shard {
        if let Some(r) = crate::shards::glass_shards_take(g, box_, v, w, who) { return Some(r); }
    }
    if !g.k.debris.dress { return debris_spawn_raw(g, kind, box_, v, w, Some(who), None); }
    if let Some(f) = g.destr.debris.fling { v = v3::add(v, f); }
    debris_thrown(g, kind, box_, v, level, who);
    debris_spawn_raw(g, kind, box_, v, w, Some(who), Some(true))
}

/// The simulation's half of a piece that is thrown: its one landing, put in the game's falls.   (debris.ts Debris.thrown)
pub fn debris_thrown(g: &mut crate::game::Game, kind: crate::debris::ChunkKind, box_: BoxId, v: V3, level: f64, who: f64) {
    let c = g.col.boxes[box_].c;
    let f = crate::falling::flight_of(g, c, v, G, crate::falling::FallWorld::StaticsAndProps);
    let down = m::sqrt(v[1] * v[1] + 2.0 * G * m::max(0.0, c[1] - f.pos[1]));
    if !(down > g.k.debris.min_speed) { return; }
    let l = crate::break_table::breaks_lands(g, kind, box_, down);
    let mut lv = level;
    if !js::truthy(lv) { lv = l.level; }
    if !js::truthy(lv) { lv = landing_of(kind).level; }
    let t = g.time + f.t;
    if crate::sim_fallshome::fall_store_schedule(g, t, f.pos[0], f.pos[1], f.pos[2], crate::sim_fallshome::FallSound::Empty, 0.0, 1.0, 0.0, who, crate::sim_fallshome::FALL_HEARD, lv, crate::sim_fallshome::FALL_MAXP, Some(crate::sim_fallshome::FALL_PIECES), Some(l.dust)) {
        g.destr.debris.stats.told += 1.0;
    }
}

/// The thrown pieces whose landing has come: the floor is told of each.   (debris.ts Debris.landings)
pub fn debris_landings(g: &mut crate::game::Game) {
    let time = g.time;
    let due = crate::sim_fallshome::fall_store_due(g, time, 0.0, 0.0, Some(crate::sim_fallshome::FALL_PIECES));
    for p in due {
        if p.heard {
            let (time, serial) = (g.time, g.serial());
            g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Prop, pos: p.pos, time, loud: false, level: Some(p.level), who: Some(p.who), serial, ..crate::game::GameEvent::zeroed() });
        }
        if p.dust > 0.0 {
            crate::destruction::destruction_cell_dust(g, p.pos, [0.0, 1.0, 0.0], p.dust, crate::scene_materials::Surf::Drywall);
        }
    }
}

/// spawn() without the glass door; `told` = the simulation has this piece's landing.   (debris.ts Debris.spawnRaw)
pub fn debris_spawn_raw(g: &mut crate::game::Game, kind: crate::debris::ChunkKind, box_: BoxId, v: V3, w: V3, who: Option<f64>, told: Option<bool>) -> Option<DebrisChunkId> {
    let who = who.unwrap_or(crate::consts::NOBODY_ID);
    let told = told.unwrap_or(false);
    let dress = g.k.debris.dress && (told || kind != ChunkKind::Shard || g.k.shards.by_fall);
    if dress && !g.m.dress.dress.debris { g.destr.debris.stats.refused += 1.0; return None; }
    while (g.destr.debris.list.len() as f64) >= debris_cap(g) {
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
        let bx = &g.col.boxes[box_];
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
        flags: Some(crate::scene_boxes::box_flag().dynamic),
        name: Some(String::from("debris")),
        mkey: Some(crate::scene_boxes::mkey().debris + ((js::to_i32(serial) & 65535) as f64)),
        ..crate::scene_boxes::MakeBoxP::zeroed()
    };
    let b = crate::scene_boxes::make_box(g, &p);
    let l = landing_of(kind);
    let id = g.tab.debris_chunk.push(Chunk {
        kind, box_: b, v: v3::copy(v), w: v3::copy(w), rest: false, still_t: 0.0, age: 0.0, sounded: false,
        level: l.level, sfx: sfx_of(&l), serial, who, on: None, on_pose: 0.0,
        dress: if dress { Some(true) } else { None },
        ..Chunk::zeroed()
    });
    g.destr.debris.list.push(id);
    g.destr.debris.stats.spawned += 1.0;
    crate::break_table::breaks_born(g, id);
    Some(id)
}

/// Back to an empty pool; `serial` runs on.   (debris.ts Debris.clear)
pub fn debris_clear(g: &mut crate::game::Game) {
    if g.k.debris.dress || js::truthy(crate::sim_fallshome::fall_store_waiting_in(g, crate::sim_fallshome::FALL_PIECES)) {
        crate::sim_fallshome::fall_store_clear(g, Some(false), Some(crate::sim_fallshome::FALL_PIECES));
    }
    let d = &mut g.destr.debris;
    d.fling = None;
    d.list.clear();
    d.impulses = 0.0;
    d.serial0 = d.serial;
    d.shoves = 0.0;
    d.scuff_at = -1.0;
    d.feet_stats = DebrisFeetStats { kicked: 0.0, off_man: 0.0, walled: 0.0, gave_up: 0.0 };
}

/// The pool's boxes for the picture; a mate rides its chunk's pose.   (debris.ts Debris.boxes)
pub fn debris_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {
    let list = g.destr.debris.list.clone();
    for c in list {
        let (b, dress, mate) = { let ch = &g.tab.debris_chunk[c]; (ch.box_, is_on(ch.dress), ch.mate) };
        if dress { g.col.boxes[b].dress = Some(true); }
        out.push(b);
        if let Some(mt) = mate {
            let (bc, by, br, bd) = { let bx = &g.col.boxes[b]; (bx.c, bx.yaw, bx.rot, bx.dress) };
            let row = &mut g.col.boxes[mt];
            row.c = bc;
            row.yaw = by;
            row.rot = br;
            row.dress = bd;
            out.push(mt);
        }
    }
}

/// An impulse over the pool: wakes, shoves and turns every chunk within reach; how many it moved.   (debris.ts Debris.impulse)
pub fn debris_impulse(g: &mut crate::game::Game, imp: crate::sheet_model::ImpulseLike, blocked: Option<&mut dyn FnMut(&mut crate::game::Game, V3) -> bool>, who: Option<f64>) -> f64 {
    if g.destr.debris.list.is_empty() { return 0.0; }
    let mut blocked = blocked;
    let mut n = 0.0;
    g.destr.debris.impulses += 1.0;
    let k = g.destr.debris.impulses;
    let di = debris_imp();
    let resp = crate::sheet_model::ImpulseResponse { gain: di.gain, up_k: di.up_k, v_max: di.v_max, wake: di.wake };
    let mut i = 0;
    while i < g.destr.debris.list.len() {
        let c = g.destr.debris.list[i];
        i += 1;
        if g.tab.debris_chunk[c].age <= 0.0 { continue; }
        let b = g.tab.debris_chunk[c].box_;
        let bc = g.col.boxes[b].c;
        let kick = match crate::sheet_model::impulse_kick(imp, bc, resp, None) { Some(x) => x, None => continue };
        if kick.s < di.wake { continue; }
        if let Some(f) = blocked.as_mut() {
            if v3::dist(bc, imp.pos) > 0.5 && (*f)(g, bc) { continue; }
        }
        let tw = di.tumble * kick.s / m::max(0.08, bound_r(g, b));
        let ch = &mut g.tab.debris_chunk[c];
        if ch.rest { ch.rest = false; ch.on = None; ch.still_t = 0.0; }
        if is_on(ch.foot) { ch.foot = Some(false); }
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
    g.destr.debris.stats.kicked += n;
    n
}

/// The best man a piece is in the way of: footGap's answer, the man, his speed.   (debris.ts Debris.feet `best`)
#[derive(Clone, Copy)]
struct FootBest { hit: DebrisFootGap, m: FootMan, sp: f64 }

/// Men's boots on pieces at rest: shoved ahead and aside, or out of a man they lie in; the shoves made.   (debris.ts Debris.feet)
pub fn debris_feet(g: &mut crate::game::Game, men: &[crate::debris::FootMan], k: crate::debris::FootKnobs, list: Option<&[DebrisChunkId]>, book: Option<&mut crate::debris::FootBook>) -> f64 {
    let list: Vec<DebrisChunkId> = match list { Some(l) => l.to_vec(), None => g.destr.debris.list.clone() };
    if list.is_empty() || men.is_empty() { return 0.0; }
    let mut book = book;
    let mut n = 0.0;
    g.destr.debris.foot_mu = k.slide;
    let mut s_ = match &book {
        Some(b) => DebrisFeetStats { kicked: b.stats.kicked, off_man: b.stats.off_man, walled: b.stats.walled, gave_up: b.stats.gave_up },
        None => g.destr.debris.feet_stats,
    };
    for &c in list.iter() {
        if n >= k.per_frame { break; }
        let (rest, foot, age, on, glass, b, foot_at, foot_n, foot_side) = {
            let ch = &g.tab.debris_chunk[c];
            (ch.rest, is_on(ch.foot), ch.age, ch.on, ch.glass.is_some(), ch.box_, ch.foot_at, ch.foot_n, ch.foot_side)
        };
        if !(rest || foot) || age <= 0.0 || on.is_some() || glass { continue; }
        let since = match foot_at { None => m::INF, Some(at) => age - at };
        if since < (if foot_n.unwrap_or(0.0) >= k.give_up { k.give_up_secs } else { k.cooldown }) { continue; }
        let bh = g.col.boxes[b].h;
        let bc = g.col.boxes[b].c;
        let mass = bh[0] * bh[1] * bh[2] * 8.0 * 600.0;
        if mass > k.heavy { continue; }
        let far = bound_of(bh) + k.radius + k.reach + 0.05;
        let mut near = false;
        for mm in men.iter() {
            let d = far + m::hyp2(mm.vel[0], mm.vel[2]) * k.lead;
            if m::abs(mm.pos[0] - bc[0]) < d && m::abs(mm.pos[2] - bc[2]) < d { near = true; break; }
        }
        if !near { continue; }
        // its footprint on the floor: a rectangle along its most level long axis
        let q = rot_of(g, b);
        let a = axes_of(q);
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
        let (bottom, top) = (bc[1] - hy, bc[1] + hy);
        // the men it lies in, or, in nobody, the walking man whose leading foot it is most in the way of
        let (mut ox, mut oz, mut in_n) = (0.0, 0.0, 0.0);
        let mut best: Option<FootBest> = None;
        for mm in men.iter() {
            let fy = mm.pos[1];
            if bottom > fy + k.floor || top < fy + 0.01 { continue; }
            let here = debris_foot_gap(g, b, mm.pos[0], mm.pos[2], d1x, d1z, e1, e2, k.radius);
            if here.gap < -k.inside { ox -= here.dx * here.gap; oz -= here.dz * here.gap; in_n += 1.0; continue; }
            let sp = m::hyp2(mm.vel[0], mm.vel[2]);
            if js::truthy(in_n) || sp <= k.walk { continue; }
            let hit = debris_foot_gap(g, b, mm.pos[0] + mm.vel[0] * k.lead, mm.pos[2] + mm.vel[2] * k.lead, d1x, d1z, e1, e2, k.radius);
            if hit.gap >= k.reach || (mm.vel[0] * hit.dx + mm.vel[2] * hit.dz) <= 0.2 * sp { continue; }
            let better = match &best { None => true, Some(x) => hit.gap < x.hit.gap };
            if better { best = Some(FootBest { hit, m: *mm, sp }); }
        }
        if !js::truthy(in_n) && best.is_none() { continue; }
        let kk = match book.as_mut() {
            Some(bk) => { bk.shoves += 1.0; bk.shoves }
            None => { g.destr.debris.shoves += 1.0; g.destr.debris.shoves }
        };
        let h = |j: f64| hash01(kk * 149.0 + j);
        let fn_now = if since > 2.0 { 1.0 } else { foot_n.unwrap_or(0.0) + 1.0 };
        {
            let ch = &mut g.tab.debris_chunk[c];
            ch.foot_at = Some(ch.age);
            ch.foot_n = Some(fn_now);
        }
        if fn_now == k.give_up { s_.gave_up += 1.0; }
        let heft = 1.0 / (1.0 + mass / 4.0);
        let (mut dx, mut dz, mut s);
        let (mut lx, mut lz) = (0.0, 0.0);
        if js::truthy(in_n) {
            let ol = m::hyp2(ox, oz);
            if ol < 0.02 { s_.walled += 1.0; continue; }
            dx = ox / ol;
            dz = oz / ol;
            s = k.off * (0.6 + 0.4 * heft);
        } else {
            // ahead of him and aside: off his line to the side it already lies on (hashed when it lies dead ahead)
            let bb = best.unwrap();
            let (vx, vz) = (bb.m.vel[0] / bb.sp, bb.m.vel[2] / bb.sp);
            let toward = vx * bb.hit.dx + vz * bb.hit.dz;
            let cr = vx * (bc[2] - bb.m.pos[2]) - vz * (bc[0] - bb.m.pos[0]);
            let side = match foot_side {
                Some(fs) if since <= 2.0 && js::truthy(fs) => fs,
                _ => if m::abs(cr) > 0.03 { m::sign(cr) } else if h(4.0) < 0.5 { -1.0 } else { 1.0 },
            };
            g.tab.debris_chunk[c].foot_side = Some(side);
            dx = bb.hit.dx - vz * side * k.aside;
            dz = bb.hit.dz + vx * side * k.aside;
            let dn = m::hyp2(dx, dz);
            dx /= dn;
            dz /= dn;
            lx = bb.hit.lx;
            lz = bb.hit.lz;
            s = m::min(k.v_max, k.base + k.gain * bb.sp * m::max(0.3, toward)) * heft * (0.8 + 0.4 * h(5.0));
        }
        // not into a wall: along it, or (square on) not at all; the try still counts
        let reach = m::abs(dx * d1x + dz * d1z) * e1 + m::abs(dx * d2x + dz * d2z) * e2 + 0.1;
        let wall = crate::scene_collision::raycast(g, [bc[0], m::max(bc[1], bottom + 0.03), bc[2]], [dx, 0.0, dz], reach);
        if let Some(wl) = wall {
            let dn = dx * wl.n[0] + dz * wl.n[2];
            let (mut tx, mut tz) = (dx - wl.n[0] * dn, dz - wl.n[2] * dn);
            let tl = m::hyp2(tx, tz);
            if tl < 0.25 { s_.walled += 1.0; continue; }
            tx /= tl;
            tz /= tl;
            dx = tx;
            dz = tz;
            s *= tl;
        }
        let bound = bound_of(bh);
        let (first_sfx, has_sfx) = {
            let ch = &mut g.tab.debris_chunk[c];
            if ch.rest { ch.rest = false; ch.still_t = 0.0; }
            ch.foot = Some(true);
            ch.v = [dx * s, 0.0, dz * s];
            // a boot on one end turns it about its middle; a touch of its own so two planks do not slide as one
            let wy = m::max(-5.0, m::min(5.0, k.spin * (lz * dx - lx * dz) * s / m::max(0.01, e1 * e1 + e2 * e2)));
            ch.w = [0.0, wy + (h(3.0) - 0.5) * 0.6 * s / m::max(0.08, bound), 0.0];
            (ch.sfx.first().cloned(), !ch.sfx.is_empty())
        };
        if k.scuff > 0.0 && has_sfx && g.time - g.destr.debris.scuff_at > 0.12 {
            // a scuff for the ear only (no hearing event: his steps already carry his noise)
            g.destr.debris.scuff_at = g.time;
            let f = first_sfx.unwrap();
            let gain = m::min(1.0, k.scuff * f.gain * (0.6 + 0.6 * s / m::max(0.1, k.v_max)));
            seam_audio_play_rate(g, &f.name, v3::copy(bc), gain, f.rate * (1.15 + 0.2 * h(7.0)));
        }
        if js::truthy(in_n) { s_.off_man += 1.0; } else { s_.kicked += 1.0; }
        n += 1.0;
    }
    match book {
        Some(bk) => bk.stats = FootBookStats { kicked: s_.kicked, off_man: s_.off_man, walled: s_.walled, gave_up: s_.gave_up },
        None => g.destr.debris.feet_stats = s_,
    }
    n
}

/// A piece outside the pool, for the feet alone: a chunk record round its box, at rest.   (debris.ts Debris.loose)
pub fn debris_loose(g: &mut crate::game::Game, box_: BoxId, lift: Option<f64>) -> DebrisChunkId {
    let l = landing_of(ChunkKind::Chunk);
    g.tab.debris_chunk.push(Chunk {
        kind: ChunkKind::Chunk, box_, lift: Some(lift.unwrap_or(0.0)), v: [0.0, 0.0, 0.0], w: [0.0, 0.0, 0.0], rest: true, still_t: 0.0, age: 0.0,
        sounded: true, level: l.level, sfx: sfx_of(&l), serial: 0.0, who: crate::consts::NOBODY_ID, on: None, on_pose: 0.0,
        ..Chunk::zeroed()
    })
}

/// A loose piece's frame: older by `dt`, and slid on if a boot set it going.   (debris.ts Debris.footStep)
pub fn debris_foot_step(g: &mut crate::game::Game, c_2: DebrisChunkId, dt: f64) {
    let c = c_2;
    g.tab.debris_chunk[c].age += dt;
    if g.tab.debris_chunk[c].rest { return; }
    if !is_on(g.tab.debris_chunk[c].foot) { g.tab.debris_chunk[c].rest = true; return; }
    let sub = 2;
    let h = dt / sub as f64;
    let mut s = 0;
    while s < sub && !g.tab.debris_chunk[c].rest { debris_slide(g, c, h); s += 1; }
}

/// A man's circle against a piece's footprint: the gap, the way out, the boot's lever.   (debris.ts Debris.footGap)
pub fn debris_foot_gap(g: &mut crate::game::Game, b: BoxId, px: f64, pz: f64, d1x: f64, d1z: f64, e1: f64, e2: f64, radius: f64) -> crate::debris::DebrisFootGap {
    let bc = g.col.boxes[b].c;
    let (d2x, d2z) = (-d1z, d1x);
    let (rx, rz) = (px - bc[0], pz - bc[2]);
    let u = rx * d1x + rz * d1z;
    let v = rx * d2x + rz * d2z;
    let cu = m::max(-e1, m::min(e1, u));
    let cv = m::max(-e2, m::min(e2, v));
    if cu == u && cv == v {
        // his middle stands over it: out by the nearer side, away from him
        let pu = e1 - m::abs(u);
        let pv = e2 - m::abs(v);
        if pu < pv {
            let s = if u > 0.0 { -1.0 } else { 1.0 };
            return DebrisFootGap { gap: -(pu + radius), dx: d1x * s, dz: d1z * s, lx: 0.0, lz: 0.0 };
        }
        let s = if v > 0.0 { -1.0 } else { 1.0 };
        return DebrisFootGap { gap: -(pv + radius), dx: d2x * s, dz: d2z * s, lx: 0.0, lz: 0.0 };
    }
    let (ou, ov) = (u - cu, v - cv);
    let d = m::hyp2(ou, ov);
    DebrisFootGap { gap: d - radius, dx: -(ou * d1x + ov * d2x) / d, dz: -(ou * d1z + ov * d2z) / d, lx: cu * d1x + cv * d2x, lz: cu * d1z + cv * d2z }
}

/// slide's `back`: the piece where it was.   (debris.ts Debris.slide)
fn slide_back(g: &mut Game, b: BoxId, c0: V3, rot0: Option<Quat>, yaw0: f64) {
    g.col.boxes[b].c = c0;
    g.col.boxes[b].rot = rot0;
    g.col.boxes[b].yaw = yaw0;
}

/// A boot's piece for `h` seconds: a slider on the floor it lay on.   (debris.ts Debris.slide)
pub fn debris_slide(g: &mut crate::game::Game, c_2: DebrisChunkId, h: f64) {
    let c = c_2;
    let b = g.tab.debris_chunk[c].box_;
    let up: V3 = [0.0, 1.0, 0.0];
    let v0 = g.tab.debris_chunk[c].v;
    let sp = m::hyp2(v0[0], v0[2]);
    if sp < SLEEP_V * 0.25 && m::abs(g.tab.debris_chunk[c].w[1]) < 0.5 {
        // stopped: asleep, laid flat, unless lying down would put a corner of it into a wall it is clear of
        let clear = !debris_foot_in(g, b);
        let r0 = g.col.boxes[b].rot;
        let y0 = g.col.boxes[b].c[1];
        let w0 = g.col.boxes[b].yaw;
        debris_sleep(g, c, up, None);
        if clear && debris_foot_in(g, b) {
            g.col.boxes[b].rot = r0;
            g.col.boxes[b].yaw = w0;
            let mut bc = g.col.boxes[b].c;
            bc[1] = y0;
            g.col.boxes[b].c = bc;
            let ch = &mut g.tab.debris_chunk[c];
            ch.rest = true;
            ch.foot = Some(false);
            ch.v = [0.0, 0.0, 0.0];
            ch.w = [0.0, 0.0, 0.0];
        }
        return;
    }
    let c0 = g.col.boxes[b].c;
    let rot0 = g.col.boxes[b].rot;
    let yaw0 = g.col.boxes[b].yaw;
    let in0 = debris_foot_in(g, b);
    if face_tilt(g, b, up) > 1e-3 { lay_flat(g, b, up, 0.2); }
    if m::abs(g.tab.debris_chunk[c].w[1]) > 1e-4 {
        let a = g.tab.debris_chunk[c].w[1] * h;
        let r = rot_of(g, b);
        g.col.boxes[b].rot = Some(v3::quat::normalize(v3::quat::mul([0.0, m::sin(a / 2.0), 0.0, m::cos(a / 2.0)], r)));
        g.tab.debris_chunk[c].w[1] *= m::max(0.0, 1.0 - 5.0 * h);
    }
    let sup = support_along(g, b, up);
    if sp > 1e-4 {
        let v = g.tab.debris_chunk[c].v;
        let (dx, dz) = (v[0] / sp, v[2] / sp);
        let d = sp * h;
        let ahead = support_along(g, b, [dx, 0.0, dz]);
        let bc = g.col.boxes[b].c;
        let from: V3 = [bc[0], m::max(bc[1], bc[1] - sup + 0.05), bc[2]];
        let st = crate::scene_collision::raycast(g, from, [dx, 0.0, dz], d + ahead);
        let ph = crate::props::prop_system_raycast(g, from, [dx, 0.0, dz], match &st { Some(s) => s.t, None => d + ahead });
        let hit: Option<(f64, V3)> = match (&ph, &st) {
            (Some(p), _) => Some((p.t, p.n)),
            (None, Some(s)) => Some((s.t, s.n)),
            (None, None) => None,
        };
        let mut nc = bc;
        if let Some((ht, hn)) = hit {
            // up to it, then along it with what is left of its speed
            let t = m::max(0.0, ht - ahead);
            nc[0] += dx * t;
            nc[2] += dz * t;
            g.col.boxes[b].c = nc;
            let ch = &mut g.tab.debris_chunk[c];
            let vn = ch.v[0] * hn[0] + ch.v[2] * hn[2];
            if vn < 0.0 {
                ch.v[0] = (ch.v[0] - hn[0] * vn * 1.1) * 0.7;
                ch.v[2] = (ch.v[2] - hn[2] * vn * 1.1) * 0.7;
            }
        } else {
            nc[0] += dx * d;
            nc[2] += dz * d;
            g.col.boxes[b].c = nc;
        }
    }
    let bc = g.col.boxes[b].c;
    let top: V3 = [bc[0], c0[1] + 0.05, bc[2]];
    let fl = crate::scene_collision::raycast(g, top, [0.0, -1.0, 0.0], 0.05 + sup + 0.3);
    let fl = match fl {
        Some(f) => f,
        None => {
            // the floor ends here: it stops where it was
            slide_back(g, b, c0, rot0, yaw0);
            let ch = &mut g.tab.debris_chunk[c];
            ch.v = [0.0, 0.0, 0.0];
            ch.w = [0.0, 0.0, 0.0];
            return;
        }
    };
    let mut nc = bc;
    nc[1] = top[1] - fl.t + sup + 0.0015 + g.tab.debris_chunk[c].lift.unwrap_or(0.0);
    g.col.boxes[b].c = nc;
    if !in0 && debris_foot_in(g, b) {
        slide_back(g, b, c0, rot0, yaw0);
        let ch = &mut g.tab.debris_chunk[c];
        ch.w[1] = 0.0;
        ch.v[0] *= 0.5;
        ch.v[2] *= 0.5;
    }
    let mu = g.destr.debris.foot_mu;
    let ch = &mut g.tab.debris_chunk[c];
    let s2 = m::hyp2(ch.v[0], ch.v[2]);
    let keep = if s2 < 1e-4 { 0.0 } else { m::max(0.0, 1.0 - mu * G * h / s2) };
    ch.v = [ch.v[0] * keep, 0.0, ch.v[2] * keep];
}

/// Does a piece's footprint reach into a static? Level rays from its middle to its four corners.   (debris.ts Debris.footIn)
pub fn debris_foot_in(g: &mut crate::game::Game, b: BoxId) -> bool {
    let q = rot_of(g, b);
    let bh = g.col.boxes[b].h;
    let bc = g.col.boxes[b].c;
    let a = axes_of(q);
    let mut iu = 0usize;
    for i in 1..3 { if m::abs(a[i][1]) > m::abs(a[iu][1]) { iu = i; } }
    let (i1, i2) = ((iu + 1) % 3, (iu + 2) % 3);
    let il = if bh[i1] >= bh[i2] { i1 } else { i2 };
    let (mut d1x, mut d1z) = (a[il][0], a[il][2]);
    let dl = m::hyp2(d1x, d1z);
    if dl < 1e-4 { d1x = 1.0; d1z = 0.0; } else { d1x /= dl; d1z /= dl; }
    let (mut e1, mut e2, mut hy) = (0.0, 0.0, 0.0);
    for i in 0..3 {
        e1 += m::abs(a[i][0] * d1x + a[i][2] * d1z) * bh[i];
        e2 += m::abs(-a[i][0] * d1z + a[i][2] * d1x) * bh[i];
        hy += m::abs(a[i][1]) * bh[i];
    }
    let l = m::hyp2(e1, e2);
    if l < 0.06 { return false; }
    let o: V3 = [bc[0], m::max(bc[1], bc[1] - hy + 0.03), bc[2]];
    for (su, sv) in [(1.0, 1.0), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)] {
        let x = (d1x * e1 * su - d1z * e2 * sv) / l;
        let z = (d1z * e1 * su + d1x * e2 * sv) / l;
        if crate::scene_collision::raycast(g, o, [x, 0.0, z], l).is_some() { return true; }
    }
    false
}

/// The pool's frame: the moving fly or slide, the resting are checked for what is under them.   (debris.ts Debris.update)
pub fn debris_update(g: &mut crate::game::Game, dt: f64) {
    if g.destr.debris.list.is_empty() { return; }
    g.destr.debris.frame += 1.0;
    let mut moving = 0.0;
    for i in (0..g.destr.debris.list.len()).rev() {
        let c = g.destr.debris.list[i];
        g.tab.debris_chunk[c].age += dt;
        if g.tab.debris_chunk[c].rest {
            let (b, on, on_pose, serial) = { let ch = &g.tab.debris_chunk[c]; (ch.box_, ch.on, ch.on_pose, ch.serial) };
            let bc = g.col.boxes[b].c;
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
                let sup = support_along(g, b, [0.0, 1.0, 0.0]);
                if bc[1] > sup + 0.02 && crate::scene_collision::raycast(g, [bc[0], bc[1] + 0.02, bc[2]], [0.0, -1.0, 0.0], sup + 0.08).is_none() {
                    let ch = &mut g.tab.debris_chunk[c];
                    ch.rest = false;
                    ch.still_t = 0.0;
                }
            }
            continue;
        }
        if !is_on(g.tab.debris_chunk[c].foot) { moving += 1.0; }
        let sub = 2;
        let h = dt / sub as f64;
        let mut s = 0;
        while s < sub && !g.tab.debris_chunk[c].rest {
            if is_on(g.tab.debris_chunk[c].foot) { debris_slide(g, c, h); } else { debris_step(g, c, h); }
            s += 1;
        }
        let b = g.tab.debris_chunk[c].box_;
        let y = g.col.boxes[b].c[1];
        if y < -1.0 || g.tab.debris_chunk[c].age > 600.0 {
            if i < g.destr.debris.list.len() { g.destr.debris.list.remove(i); }
            if g.tab.debris_chunk[c].glass.is_some() { crate::shards::glass_shards_gone(g, c); }
        }
    }
    let live = g.destr.debris.list.len() as f64;
    let st = &mut g.destr.debris.stats;
    st.peak_live = m::max(st.peak_live, live);
    st.peak_moving = m::max(st.peak_moving, moving);
}

fn box_add_c(g: &mut Game, b: BoxId, d: V3) {
    let mut c = g.col.boxes[b].c;
    c[0] += d[0];
    c[1] += d[1];
    c[2] += d[2];
    g.col.boxes[b].c = c;
}

/// A thrown chunk for `h` seconds: ballistic flight with a swept contact.   (debris.ts Debris.step)
pub fn debris_step(g: &mut crate::game::Game, c_2: DebrisChunkId, h: f64) {
    let c = c_2;
    let b = g.tab.debris_chunk[c].box_;
    g.tab.debris_chunk[c].v[1] -= G * h;
    // turn
    let w0 = g.tab.debris_chunk[c].w;
    let wl = v3::len(w0);
    if wl > 1e-5 {
        let a = wl * h;
        let s = m::sin(a / 2.0) / wl;
        let r = rot_of(g, b);
        g.col.boxes[b].rot = Some(v3::quat::normalize(v3::quat::mul([w0[0] * s, w0[1] * s, w0[2] * s, m::cos(a / 2.0)], r)));
    }
    // move with a swept contact: the centre ray, met `support` short of the surface
    let mv = v3::scale(g.tab.debris_chunk[c].v, h);
    let len = v3::len(mv);
    if len < 1e-7 { debris_settle_tick(g, c, h, None); return; }
    let dir = v3::scale(mv, 1.0 / len);
    let reach = len + bound_r(g, b) * 1.05;
    let bc = g.col.boxes[b].c;
    let st = crate::scene_collision::raycast(g, bc, dir, reach);
    let ph = crate::props::prop_system_raycast(g, bc, dir, match &st { Some(s) => s.t, None => reach });
    let hit: Option<(f64, V3, Option<PropId>)> = match (&ph, &st) {
        (Some(p), _) => Some((p.t, p.n, Some(p.prop))),
        (None, Some(s)) => Some((s.t, s.n, None)),
        (None, None) => None,
    };
    let mut landed = false;
    if let Some((ht, n, hit_prop)) = hit {
        let back = support_along(g, b, n) / m::max(0.35, -v3::dot(dir, n));
        if ht - back <= len {
            let t_hit = m::max(0.0, ht - back);
            box_add_c(g, b, [dir[0] * t_hit, dir[1] * t_hit, dir[2] * t_hit]);
            let vn = v3::dot(g.tab.debris_chunk[c].v, n);
            if vn < 0.0 {
                let speed = -vn;
                if let Some(p) = hit_prop {
                    if !is_on(g.tab.debris_chunk[c].dress) {
                        let at = g.col.boxes[b].c;
                        let j = v3::scale(n, vn * 0.4 * mass_of(g, b));
                        crate::props::prop_system_apply_impulse(g, p, at, j, 0.0, None);
                    }
                }
                let v_n = v3::scale(n, vn);
                let v_t = v3::sub(g.tab.debris_chunk[c].v, v_n);
                let ct = g.tab.debris_chunk[c].contact;
                let keep = if n[1] > 0.6 {
                    match &ct { Some(x) => x.keep_floor, None => 0.55 }
                } else {
                    match &ct { Some(x) => x.keep_wall, None => 0.8 }
                };
                let back_k = match &ct { Some(x) => -x.rest, None => -0.22 };
                g.tab.debris_chunk[c].v = v3::add(v3::scale(v_t, keep), v3::scale(v_n, back_k));
                // a floor touch turns it toward lying on the face nearest down, and trades some of the knock into a wobble
                if n[1] > 0.6 {
                    lay_flat(g, b, n, 0.45);
                    let kick = v3::cross(n, v_t);
                    let br = bound_r(g, b);
                    let ch = &mut g.tab.debris_chunk[c];
                    ch.w = v3::add(v3::scale(ch.w, 0.45), v3::scale(kick, 2.0 / m::max(0.08, br)));
                } else {
                    let ch = &mut g.tab.debris_chunk[c];
                    ch.w = v3::scale(ch.w, 0.7);
                }
                if !g.tab.debris_chunk[c].sounded && speed > 1.0 { debris_land(g, c, speed); }
                landed = n[1] > 0.6;
                {
                    let ch = &mut g.tab.debris_chunk[c];
                    if landed && m::abs(v3::dot(ch.v, n)) < 0.5 && v3::len(ch.v) < SLEEP_V * 2.0 {
                        ch.v = v3::scale(ch.v, 0.5);
                        ch.w = v3::scale(ch.w, 0.5);
                    }
                }
                let (lv, lw) = { let ch = &g.tab.debris_chunk[c]; (v3::len(ch.v), v3::len(ch.w)) };
                if landed && lv < SLEEP_V && lw < SLEEP_W { debris_sleep(g, c, n, hit_prop); return; }
            }
            box_add_c(g, b, [n[0] * 0.002, n[1] * 0.002, n[2] * 0.002]);
        } else {
            box_add_c(g, b, mv);
        }
    } else {
        box_add_c(g, b, mv);
    }
    // the world's floor as a last resort
    let sup = support_along(g, b, [0.0, 1.0, 0.0]);
    let bc = g.col.boxes[b].c;
    if bc[1] < sup {
        let mut nc = bc;
        nc[1] = sup + 0.001;
        g.col.boxes[b].c = nc;
        if g.tab.debris_chunk[c].v[1] < 0.0 {
            let sp = -g.tab.debris_chunk[c].v[1];
            {
                let ch = &mut g.tab.debris_chunk[c];
                ch.v[1] *= -0.2;
                ch.v[0] *= 0.6;
                ch.v[2] *= 0.6;
            }
            lay_flat(g, b, [0.0, 1.0, 0.0], 0.5);
            {
                let ch = &mut g.tab.debris_chunk[c];
                ch.w = v3::scale(ch.w, 0.5);
            }
            if !g.tab.debris_chunk[c].sounded && sp > 1.0 { debris_land(g, c, sp); }
        }
        let (lv, lw) = { let ch = &g.tab.debris_chunk[c]; (v3::len(ch.v), v3::len(ch.w)) };
        if lv < SLEEP_V && lw < SLEEP_W { debris_sleep(g, c, [0.0, 1.0, 0.0], None); return; }
    }
    debris_settle_tick(g, c, h, if landed { Some([0.0, 1.0, 0.0]) } else { None });
}

/// Nearly still for SLEEP_T while touching a floor: asleep.   (debris.ts Debris.settleTick)
pub fn debris_settle_tick(g: &mut crate::game::Game, c_2: DebrisChunkId, h: f64, floor_n: Option<V3>) {
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

/// Lays it flat and puts it to rest on what is under it; still far off its face, another settling turn instead.   (debris.ts Debris.sleep)
pub fn debris_sleep(g: &mut crate::game::Game, c_2: DebrisChunkId, n: V3, on: Option<PropId>) {
    let c = c_2;
    let b = g.tab.debris_chunk[c].box_;
    if face_tilt(g, b, n) > 0.35 {
        lay_flat(g, b, n, 0.45);
        g.tab.debris_chunk[c].still_t = 0.0;
        return;
    }
    lay_flat(g, b, n, 1.0);
    let on_pose = match on { Some(p) => pose_key(g, p), None => 0.0 };
    {
        let ch = &mut g.tab.debris_chunk[c];
        ch.rest = true;
        ch.v = [0.0, 0.0, 0.0];
        ch.w = [0.0, 0.0, 0.0];
        ch.on = on;
        ch.on_pose = on_pose;
        if is_on(ch.foot) { ch.foot = Some(false); }
    }
    // sit exactly on the surface under it
    let sup = support_along(g, b, [0.0, 1.0, 0.0]);
    let bc = g.col.boxes[b].c;
    let from: V3 = [bc[0], bc[1] + 0.05, bc[2]];
    let st = crate::scene_collision::raycast(g, from, [0.0, -1.0, 0.0], sup + 0.4);
    let ph = crate::props::prop_system_raycast(g, from, [0.0, -1.0, 0.0], match &st { Some(s) => s.t, None => sup + 0.4 });
    let t = match (&ph, &st) {
        (Some(p), _) => Some(p.t),
        (None, Some(s)) => Some(s.t),
        (None, None) => None,
    };
    if let Some(t) = t {
        let mut nc = bc;
        nc[1] = from[1] - t + sup + 0.0015 + g.tab.debris_chunk[c].lift.unwrap_or(0.0);
        g.col.boxes[b].c = nc;
    }
    if let Some(p) = ph {
        let k = pose_key(g, p.prop);
        let ch = &mut g.tab.debris_chunk[c];
        ch.on = Some(p.prop);
        ch.on_pose = k;
    }
    if g.tab.debris_chunk[c].glass.is_some() { crate::shards::glass_shards_rested(g, c); }
}

/// A chunk's first hard touch: its knock, and the floor told unless it is dressing.   (debris.ts Debris.land)
pub fn debris_land(g: &mut crate::game::Game, c_2: DebrisChunkId, speed: f64) {
    let c = c_2;
    g.tab.debris_chunk[c].sounded = true;
    g.destr.debris.stats.landed += 1.0;
    crate::break_table::breaks_landed(g, c, speed);
    if g.tab.debris_chunk[c].glass.is_some() && crate::shards::glass_shards_landed(g, c, speed) { return; }
    let b = g.tab.debris_chunk[c].box_;
    let p = v3::copy(g.col.boxes[b].c);
    let (sfx, serial) = { let ch = &g.tab.debris_chunk[c]; (ch.sfx.clone(), ch.serial) };
    for s in sfx.iter() {
        let gain = m::min(1.0, s.gain * (0.6 + speed * 0.15));
        seam_audio_play_rate(g, &s.name, p, gain, s.rate * (0.94 + 0.12 * hash01(serial * 31.0)));
    }
    if !is_on(g.tab.debris_chunk[c].dress) {
        let (time, level, who, ev) = (g.time, g.tab.debris_chunk[c].level, g.tab.debris_chunk[c].who, g.serial());
        g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Prop, pos: p, time, loud: false, level: Some(level), who: Some(who), serial: ev, ..crate::game::GameEvent::zeroed() });
    }
}

/// The panel's and QA's line.   (debris.ts Debris.summary)
pub fn debris_summary(g: &mut crate::game::Game) -> String {
    let (n, cap, moving) = (g.destr.debris.list.len(), debris_cap(g), debris_moving(g));
    let s = g.destr.debris.stats;
    alloc::format!("debris {}/{} ({} moving) · spawned {} · refused {} · evicted {} · landings {} · peak {}/{} moving", n, cap, moving, s.spawned, s.refused, s.evicted, s.landed, s.peak_live, s.peak_moving)
}

/// The box's half extent along world direction n.   (debris.ts supportAlong)
pub fn support_along(g: &mut crate::game::Game, b: BoxId, n: V3) -> f64 {
    let r = rot_of(g, b);
    support_of(r, g.col.boxes[b].h, n)
}

/// A slot the pool may take: at rest, or one only a boot has set sliding.   (debris.ts evictable)
pub fn evictable(g: &mut crate::game::Game, c_2: DebrisChunkId) -> bool {
    let ch = &g.tab.debris_chunk[c_2];
    ch.rest || ch.foot == Some(true)
}

/// (debris.ts boundR)
pub fn bound_r(g: &mut crate::game::Game, b: BoxId) -> f64 { bound_of(g.col.boxes[b].h) }

/// A nominal mass off the volume, clamped.   (debris.ts massOf)
pub fn mass_of(g: &mut crate::game::Game, b: BoxId) -> f64 { mass_from(g.col.boxes[b].h) }

/// Radians between n and the box axis layFlat would settle onto it.   (debris.ts faceTilt)
pub fn face_tilt(g: &mut crate::game::Game, b: BoxId, n: V3) -> f64 {
    let q = rot_of(g, b);
    face_tilt_of(q, g.col.boxes[b].h, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash01_bits() {
        for (x, bits) in TS_HASH01 { assert_eq!(hash01(x).to_bits(), bits, "hash01({x})"); }
    }
    #[test]
    fn support_and_tilt_bits() {
        let q = v3::quat::normalize([0.1, 0.7, -0.3, 0.6]);
        let h = [0.3, 0.015, 0.3];
        let n = v3::normalize([0.2, 1.0, -0.1]);
        assert_eq!(support_of(q, h, n).to_bits(), TS_SUPPORT);
        assert_eq!(face_tilt_of(q, h, n).to_bits(), TS_TILT);
        assert_eq!(bound_of(h).to_bits(), TS_BOUND);
        assert_eq!(mass_from([0.1, 0.02, 0.1]).to_bits(), TS_MASS);
        assert_eq!(mass_from(h), 6.0);
    }
    // the TypeScript's own bits: printed by bun with EXACT on, from src/game/debris.ts
    const TS_HASH01: [(f64, u64); 10] = [
        (0.0, 0x3fbd62ba00000000), (1.0, 0x3fcf96d000000000), (31.0, 0x3fe511d080000000), (977.0, 0x3fe9af60e0000000), (149005.0, 0x3fe267c7a0000000),
        (-7.0, 0x3fe9bf4100000000), (4294967296.5, 0x3fbd62ba00000000), (6740467557.0, 0x3f9ea43400000000), (1000000000000000.0, 0x3fc6ef8800000000), (2.5, 0x3f97dd7000000000),
    ];
    const TS_SUPPORT: u64 = 0x3fc7788d954357fe;
    const TS_TILT: u64 = 0x3fdc4623a88db349;
    const TS_BOUND: u64 = 0x3fdb2b7c50dfcd02;
    const TS_MASS: u64 = 0x3feeb851eb851eb9;
}
