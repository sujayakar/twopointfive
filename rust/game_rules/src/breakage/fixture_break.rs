#![allow(unused_imports, unused_variables)]
//! A FIXTURE THAT BROKE: its tray swinging down by one end and tearing out, its diffuser and glass on the floor, the torn
//! lead's glow. "Picture only: nothing play asks about" by its rays; but every piece is a KEYED box of the frame's list (MKEY.test + 0x5000 + …), and
//! the fog of war keeps a record of every keyed loose thing of that list and caps its records, the frame's room is budgeted with them in it, and the
//! look reads the box world (a ray down, the furniture's footprints, sight lines to where it will lie) and this frame's hearing events. So the lane
//! runs here, under its DRESS row, as it runs in the TypeScript.
//!
//! THE PARTICLES (the lead's sparks, the tube's glass, the splinters a landing throws) go to the host's pool where it has one (`gpu_fx`, the
//! TypeScript's own `if (P)`), each with a seed of the fixture's own (`seed_of`, which counts on `Rec.emits`); a host with none gets none and the
//! count stays, as there.
//! The module's state is `g.m.fixture_break.state` (the TypeScript's SideMap by the game); `by` (a Map by the target) is the record's own `t`.

pub use ::game_types::fixture_break::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

use crate::v3::quat;

/// integer hash → [0, 1): the module's own dice, keyed by the fixture's place
fn h01(x: f64) -> f64 {
    let mut x = m::imul(js::to_i32(x) ^ 0x68E31DA4, 0x9E3779B1u32 as i32) as u32;
    x ^= x >> 15;
    x = m::imul(x as i32, 0x85EBCA77u32 as i32) as u32;
    x ^= x >> 13;
    (x >> 8) as f64 / 16777216.0
}

fn static_box(g: &Game, t: FixtureId) -> Option<BoxId> { let i = *g.tab.fixture[t].boxes.first()?; g.engine.world.statics.get(i as usize).copied() }

/// 1 = a ceiling panel (the full look), 2 = a lamp with a light of its own (sparks only), 0 = not this lane's.
fn kind_of(g: &crate::game::Game, t: FixtureId) -> f64 {
    use crate::fixtures::FixtureKind as K;
    let (interactive, kind, light, n_boxes) = { let f = &g.tab.fixture[t]; (f.interactive, f.kind, f.light, f.boxes.len()) };
    if !interactive { return 0.0; }
    if kind == K::Analytic { return if light.is_some() { 2.0 } else { 0.0 }; }
    if kind != K::Emissive || n_boxes == 0 { return 0.0; }
    let Some(b) = static_box(g, t) else { return 0.0; };
    let b = &g.tab.boxes[b];
    let (long, short) = (m::max(b.h[0], b.h[2]), m::min(b.h[0], b.h[2]));
    if js::to_i32(b.flags) & js::to_i32(crate::scene_boxes::box_flag().no_primary) != 0 && b.h[1] <= 0.06 && b.c[1] > 2.2 && long >= 0.25 && long <= 1.2 && short >= 0.1 { 1.0 } else { 0.0 }
}

/// The top of whatever lies under (x, z): the level's statics by one ray down, the furniture by its boxes' footprints.
fn floor_at(g: &mut crate::game::Game, x: f64, z: f64, from: f64) -> f64 {
    let mut y = 0.0;
    if let Some(hit) = crate::scene_collision::raycast(g, [x, from, z], down(), from + 0.5) { if hit.index < 1e6 { y = m::max(0.0, from - hit.t); } }   // (a door leaf's top edge is no floor)
    g.m.fixture_break.floor.on = None;
    let no_shadow = js::to_i32(crate::scene_boxes::box_flag().no_shadow);
    for pi in 0..g.props.props.len() {
        let p = g.props.props[pi];
        for bi in 0..g.tab.props_prop[p].boxes.len() {
            let id = g.tab.props_prop[p].boxes[bi]; let b = &g.tab.boxes[id];
            if js::to_i32(b.flags) & no_shadow != 0 { continue; }
            let top = b.c[1] + b.h[1]; if top <= y || top > from { continue; }
            let (dx, dz, cs, sn) = (x - b.c[0], z - b.c[2], m::cos(b.yaw), m::sin(b.yaw)); let (lx, lz) = (dx * cs - dz * sn, dx * sn + dz * cs);
            if m::abs(lx) <= b.h[0] && m::abs(lz) <= b.h[2] { y = top; g.m.fixture_break.floor.on = Some(id); }
        }
    }
    y
}

/// What a fixture lying from (x, z) along (ux, uz) rests on: the highest of three samples down its length, and whether that is flat bare floor.
fn rest_on(g: &mut crate::game::Game, x: f64, z: f64, ux: f64, uz: f64, l: f64, from: f64) -> crate::fixture_break::RestOn {
    let (mut y, mut lo, mut prop) = (0.0, 1e9, false);
    for a in rest_at() { let f = floor_at(g, x + ux * l * a, z + uz * l * a, from); if g.m.fixture_break.floor.on.is_some() { prop = true; } if f > y { y = f; } if f < lo { lo = f; } }
    RestOn { y, flat: y - lo < 0.05 && !prop }
}

/// Where the blast or the round that just killed it came from, if this frame's events say.
fn cause_of(g: &crate::game::Game, at: V3) -> Option<V3> {
    use crate::director::EventKind as E;
    let n = g.events.len(); let (mut best, mut bd): (Option<V3>, f64) = (None, 12.0);
    let mut i = n as i64 - 1;
    while i >= 0 && i >= n as i64 - 48 {
        let e = &g.events[i as usize]; i -= 1;
        if g.time - e.time > 0.3 { continue; }
        if e.kind != E::Bang && e.kind != E::Shot && e.kind != E::GuardShot { continue; }
        let d = v3::dist(e.pos, at); if d < bd { bd = d; best = Some(e.pos); }
    }
    best
}

fn state(g: &mut Game) -> &mut State { g.m.fixture_break.state.as_mut().unwrap() }

/// A fixture found broken: its record, its hanging boxes and its loose pieces.
fn add(g: &mut crate::game::Game, t: FixtureId, kind: f64, born: f64) -> crate::fixture_break::Rec {
    let f = g.m.fixture_break.fixlook.clone();
    let sb = if kind == 1.0 { static_box(g, t) } else { None };
    let c: V3 = match sb { Some(b) => g.tab.boxes[b].c, None => g.tab.fixture[t].pos };
    let seed = (js::to_i32(m::round(c[0] * 100.0) * 73856093.0) ^ js::to_i32(m::round(c[2] * 100.0) * 19349663.0) ^ js::to_i32(m::round(c[1] * 100.0) * 83492791.0)) as f64;
    let scan = state(g).scan;
    let mut r = Rec { t, born, seed, seen: scan, hinge: c, uh: [1.0, 0.0, 0.0], l: 0.0, w: 0.0, th: 0.0, om: 0.0, rest: 0.0, still: false, hang: Vec::new(), pieces: Vec::new(),
        pop: 0.0, last_slot: -1.0, glass_out: false, tip: c, emits: 0.0, phase: 0.0, fall_at: f64::INFINITY, h0: c, th0: 0.0, t0: 0.0, lie: c, uh0: [1.0, 0.0, 0.0], skew: 0.0, lay: 0.0 };
    let beat = born > -1e8;
    if !beat { r.pop = f.pops.len() as f64; r.glass_out = true; }
    let Some(sb) = sb else {   // (a lamp has no line of its own: its sparks lean a bearing of its own, not the world's +x)
        let a = h01(seed + 7.0) * 2.0 * m::PI; r.uh = [m::cos(a), 0.0, m::sin(a)];  return r;
    };
    let (bh, byaw) = { let b = &g.tab.boxes[sb]; (b.h, b.yaw) };
    let q = quat::yaw(byaw); let long = bh[0] >= bh[2]; let u = quat::rotate(q, if long { [1.0, 0.0, 0.0] } else { [0.0, 0.0, 1.0] });
    let hanging = state(g).recs.iter().filter(|x| !x.hang.is_empty()).count() as f64;
    if hanging >= f.max_panels { r.uh = v3::scale(u, if h01(seed + 7.0) < 0.5 { -1.0 } else { 1.0 }); return r; }   
    let l = 2.0 * (if long { bh[0] } else { bh[2] }); let w = 2.0 * (if long { bh[2] } else { bh[0] }); r.l = l; r.w = w;
    let from = if beat { cause_of(g, c) } else { None }; let dn = match from { Some(p) => v3::dot(u, v3::sub(c, p)), None => 0.0 };
    let side = if m::abs(dn) > 0.05 { m::sign(dn) } else if h01(seed) < 0.5 { -1.0 } else { 1.0 };   // the hinge is the end AWAY from what hit it
    r.hinge = [c[0] + u[0] * side * l / 2.0, c[1] - 0.02, c[2] + u[2] * side * l / 2.0]; r.uh = v3::scale(u, -side);
    r.rest = (f.rest_deg[0] + (f.rest_deg[1] - f.rest_deg[0]) * h01(seed + 1.0)) * m::PI / 180.0;
    r.h0 = r.hinge; r.lie = r.hinge; r.uh0 = r.uh; r.fall_at = f.hang_secs * (0.85 + 0.3 * h01(seed + 3.0));
    if r.fall_at.is_finite() {   // where it comes to lie: under where it hung, slid to one side and skewed, face down on the floor or on what stands there
        let w0: V3 = [-r.uh[2], 0.0, r.uh[0]]; let s0 = if h01(seed + 4.0) < 0.5 { -1.0 } else { 1.0 }; let sk0 = (h01(seed + 5.0) - 0.5) * 2.0 * f.skew;
        let eye: V3 = [c[0], 1.4, c[2]]; let half = w / 2.0 + 0.1; let mut rank = -1;
        for k in [s0, -s0, 0.0] { for sk in [sk0, -sk0, 0.0] {
            if rank == 3 { break; }
            let sl = k * m::max(f.slide, w * 0.6); let (x, z) = (r.hinge[0] + w0[0] * sl, r.hinge[2] + w0[2] * sl);
            let (cs, sn) = (m::cos(sk), m::sin(sk)); let (ux, uz) = (r.uh[0] * cs - r.uh[2] * sn, r.uh[0] * sn + r.uh[2] * cs);   // (land()'s own turn)
            let mut clear = true;
            for a in [0.0, 1.0] { for e in [-half, half] { if clear && crate::scene_collision::solid_blocked(g, eye, [x + ux * l * a - uz * e, 1.4, z + uz * l * a + ux * e], None) { clear = false; } } }
            let fr = rest_on(g, x, z, ux, uz, l, c[1] - 0.1); let qk = if clear { if fr.flat { 3 } else { 2 } } else { 0 };
            if qk > rank || (qk == 0 && rank == 0) { rank = qk; r.skew = sk; r.lie = [x, m::min(fr.y, c[1] - 0.3) + 0.046, z]; }
        } }
    }
    if beat { r.th = 0.02; r.om = f.kick * (0.85 + 0.3 * h01(seed + 2.0)); }
    else if r.fall_at.is_finite() { r.phase = 2.0; r.th = 0.0; r.still = true; r.lay = 1.0; land(&mut r, 1.0); }
    else { r.th = r.rest; r.still = true; }
    let key = { let s = state(g); let k = crate::scene_boxes::mkey().test + 20480.0 + ((js::to_i32(s.keys) & 0xff) << 5) as f64; s.keys += 1.0; k };
    let mut part = 0.0;
    let mut mk = |g: &mut Game, h: V3, albedo: V3, rough: Option<f64>| -> FrameBoxId {
        let word = crate::scene_stuff::word_for(g, Some(&albedo), None, None);
        let id = crate::scene_boxes::box_kept(g, crate::scene_boxes::make_box_value(&crate::scene_boxes::MakeBoxP { c, h, albedo: Some(albedo), stuff: word.stuff, does: word.does,
            flags: Some(LOOK), rot: Some(quat::ident()), name: Some(String::from("fixtureLook")), mkey: Some(key + part), rough, ..Default::default() }));
        part += 1.0; id
    };
    r.hang = vec![mk(g, [l / 2.0, 0.02, w / 2.0], f.tray, None), mk(g, [l * 0.15, 0.03, m::min(0.06, w * 0.22)], f.cover, None), mk(g, [l / 2.0 - 0.015, 0.003, w / 2.0 - 0.02], f.face, None),
        mk(g, [l / 2.0 - 0.07, 0.014, 0.014], f.tube, None), mk(g, [l * 0.15, 0.014, 0.014], f.tube, None), mk(g, f.lead, f.soot, None)];
    // (belt and braces: were a ray ever to meet it, its glow would still count for nothing)
    { let x = &mut g.tab.frame_boxes[r.hang[5]]; x.flags = (js::to_i32(x.flags) | js::to_i32(crate::scene_boxes::box_flag().area_lit)) as f64; }
    let wv: V3 = [-r.uh0[2], 0.0, r.uh0[0]]; let y0 = c[1] - 0.06;
    let (h0, uh0) = (r.h0, r.uh0);
    let mut piece = |g: &mut Game, h: V3, a: f64, s: f64, albedo: V3, glass: bool, k: f64| -> Piece {
        let hs = |n: f64| h01(seed + 97.0 * k + n);
        let box_ = mk(g, h, albedo, Some(if glass { 0.15 } else { 0.3 }));
        let p: V3 = [h0[0] + uh0[0] * a + wv[0] * s, y0, h0[2] + uh0[2] * a + wv[2] * s];
        let mut v: V3 = [uh0[0] * (0.2 + 0.7 * hs(1.0)) + wv[0] * (hs(2.0) - 0.5) * (if glass { 2.6 } else { 0.8 }), 0.0, uh0[2] * (0.2 + 0.7 * hs(1.0)) + wv[2] * (hs(2.0) - 0.5) * (if glass { 2.6 } else { 0.8 })];
        if glass { v[0] += uh0[0] * (hs(6.0) - 0.4) * 1.6; v[2] += uh0[2] * (hs(6.0) - 0.4) * 1.6; }
        let yaw = byaw + (hs(3.0) - 0.5) * (if glass { 6.28 } else { 0.9 }); let q0 = quat::yaw(yaw);
        let axis = v3::normalize([wv[0] + (hs(4.0) - 0.5) * 0.8, (hs(5.0) - 0.5) * 0.4, wv[2] + (hs(4.0) - 0.5) * 0.8]);
        let fall = m::sqrt(2.0 * y0 / 9.8); let floor_y = floor_at(g, p[0] + v[0] * fall, p[2] + v[2] * fall, y0); let on = g.m.fixture_break.floor.on;
        { let b = &mut g.tab.frame_boxes[box_]; b.c = p; b.yaw = yaw; b.rot = Some(q0); }
        let (on_x, on_z) = match on { Some(o) => (g.tab.boxes[o].c[0], g.tab.boxes[o].c[2]), None => (0.0, 0.0) };
        let mut pc = Piece { box_, v, axis, rate: (if glass { 9.0 } else { 3.2 }) * (0.6 + 0.8 * hs(7.0)), ang: 0.0, q0, floor_y, yaw, delay: if glass { 0.02 + 0.2 * hs(8.0) } else { k * 0.11 },
            rest: false, bounced: false, glass, on, on_x, on_z };
        if !beat { let tf = m::sqrt(2.0 * m::max(0.0, y0 - floor_y) / 9.8); g.tab.frame_boxes[box_].c = [p[0] + v[0] * tf, floor_y + h[1] + 0.002, p[2] + v[2] * tf]; pc.rest = true; }
        pc
    };
    let p0 = piece(g, [l * 0.3, 0.005, w * 0.46], l * 0.36, 0.0, f.diffuser, false, 0.0); r.pieces.push(p0);
    let p1 = piece(g, [l * 0.17, 0.005, w * 0.44], l * 0.8, 0.02, f.diffuser, false, 1.0); r.pieces.push(p1);
    let mut k = 0.0;
    while k < f.shards {
        let hs = |n: f64| h01(seed + 977.0 * (k + 1.0) + n);
        let p = piece(g, [0.018 + 0.035 * hs(1.0), 0.003, 0.008 + 0.02 * hs(2.0)], l * (0.15 + 0.7 * hs(3.0)), (hs(4.0) - 0.5) * w * 0.5, f.shard, true, k + 2.0);
        r.pieces.push(p);
        k += 1.0;
    }
    pose(g, &mut r);
    r
}

/// The hanging fixture's boxes at the swing's angle `th` (0 = flat under the ceiling, the free end coming down as it grows).
fn pose(g: &mut crate::game::Game, r: &mut crate::fixture_break::Rec) {
    if r.hang.is_empty() { return; }
    let (cs, sn, uh) = (m::cos(r.th), m::sin(r.th), r.uh);
    let d: V3 = [uh[0] * cs, -sn, uh[2] * cs];            // hinge → free end
    let n: V3 = [-uh[0] * sn, -cs, -uh[2] * sn];          // the face's normal (straight down while it was up)
    let up: V3 = [-n[0], -n[1], -n[2]]; let z = v3::cross(d, up); let q = quat::from_basis(d, up, z);
    let (l, w) = (r.l, r.w); let yaw = m::atan2(-d[2], d[0]);
    let hinge = r.hinge;
    let mut at = |i: usize, a: f64, off: f64, s: f64| {
        let x = &mut g.tab.frame_boxes[r.hang[i]];
        x.c = [hinge[0] + d[0] * a + n[0] * off + z[0] * s, hinge[1] + d[1] * a + n[1] * off + z[1] * s, hinge[2] + d[2] * a + n[2] * off + z[2] * s]; x.rot = Some(q); x.yaw = yaw;
    };
    at(0, l / 2.0, -0.02, 0.0); at(1, l * 0.3, -0.07, 0.0); at(2, l / 2.0, 0.003, 0.0); at(3, l * (0.62 - 0.06 * r.lay), 0.03, w * 0.2 + (w * 0.3 + 0.07) * r.lay); at(4, 0.07 + l * 0.15, 0.03, -w * 0.2); at(5, l + 0.012, -0.005, -w * 0.2);
    r.tip = g.tab.frame_boxes[r.hang[5]].c;
}

/// The fixture `s` (0‥1) of the way from where it hung to where it lies: the hinge end comes down (a free fall) and across, the long axis skews.
fn land(r: &mut crate::fixture_break::Rec, s: f64) {
    r.hinge = [r.h0[0] + (r.lie[0] - r.h0[0]) * s, r.h0[1] + (r.lie[1] - r.h0[1]) * s * s, r.h0[2] + (r.lie[2] - r.h0[2]) * s];
    let a = r.skew * s; let (cs, sn) = (m::cos(a), m::sin(a)); r.uh = [r.uh0[0] * cs - r.uh0[2] * sn, 0.0, r.uh0[0] * sn + r.uh0[2] * cs];
}

/// The arc at the torn lead `a` seconds after the light died: 0‥1, and whether a pop or a spit STARTS inside this frame (→ sparks).
fn glow_at(g: &crate::game::Game, r: &mut crate::fixture_break::Rec, a: f64, out: &mut crate::fixture_break::GlowAtOut) -> f64 {
    let f = g.m.fixture_break.fixlook.clone(); let mut gl = 0.0; out.sparks = 0.0;
    for k in 0..f.pops.len() {
        let tk = f.pops[k] * (0.85 + 0.3 * h01(r.seed + 11.0 * k as f64 + 5.0)); if a < tk { break; }
        gl += f.pop_gain[k] * m::exp(-(a - tk) / f.pop_tau);
        if k as f64 >= r.pop { r.pop = k as f64 + 1.0; out.sparks += f.pop_sparks[k]; }
    }
    if a > f.spit_from && a < f.spit_secs {
        let slot = m::floor(a / 0.11); let h = h01(r.seed + slot * 31.0 + 7.0); let p = f.spit_rate * m::exp(-a / f.spit_tau);
        if h < p { gl += f.spit_gain * (0.5 + h01(r.seed + slot * 17.0 + 3.0)); if slot != r.last_slot && h < p * 0.4 { out.sparks += 4.0; } }
        r.last_slot = slot;
    }
    gl
}

/// The seed of the fixture's next emit (its own count, hashed with its place).
fn seed_of(r: &mut crate::fixture_break::Rec) -> f64 {
    let e = r.emits; r.emits += 1.0;
    let x = m::imul(js::to_i32((js::to_i32(r.seed) ^ 0x51ED270B) as f64 + e * 7919.0), 0x9E3779B1u32 as i32) as u32;
    if x == 0 { 1.0 } else { x as f64 }
}

/// Once a frame (game.ts, with the other dynamic boxes): notice fixtures that broke or were repaired, move what is moving, hand the frame the look's
/// boxes.
pub fn fixture_break_boxes(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>, dt: f64) {
    let f = g.m.fixture_break.fixlook.clone();
    if !f.on {
        if let Some(s) = g.m.fixture_break.state.as_mut() { let gone = core::mem::take(&mut s.recs); s.last_t = -1.0; for r in gone.iter() { let_go(g, r); } }
        return;
    }
    if g.m.fixture_break.state.is_none() { g.m.fixture_break.state = Some(State { recs: Vec::new(), prev: JsMap::new(), kind: JsMap::new(), last_t: -1.0, scan: 0.0, keys: 0.0 }); }
    let now = g.time;
    let running = { let s = state(g); s.last_t >= 0.0 && now >= s.last_t && now - s.last_t < 0.5 };   // (the clock ran on from the last frame: a target broken since then BROKE; after a load or a restart it is just found broken)
    if !running && !state(g).recs.is_empty() { let gone = core::mem::take(&mut state(g).recs); for r in gone.iter() { let_go(g, r); } }   // (the clock jumped: whatever is broken now is found broken, hanging at rest)
    { let s = state(g); s.last_t = now; s.scan += 1.0; }
    let scan = state(g).scan;
    for t in g.targets.clone() {
        let k = match state(g).kind.get(t).copied() { Some(k) => k, None => { let k = kind_of(g, t); state(g).kind.set(t, k); k } };
        if k == 0.0 { continue; }
        let broken = g.tab.fixture[t].broken;
        let was = state(g).prev.get(t).copied(); if was != Some(broken) { state(g).prev.set(t, broken); }
        if let Some(i) = state(g).recs.iter().position(|r| r.t == t) { if broken { state(g).recs[i].seen = scan; } continue; }
        if broken { let n = add(g, t, k, if running && was == Some(false) { now } else { -1e9 }); state(g).recs.push(n); }
    }
    if state(g).recs.is_empty() { return; }
    // repaired (a restart, the panel's 'repair lights') or no longer a target: gone
    if state(g).recs.iter().any(|r| r.seen != scan) {
        let all = core::mem::take(&mut state(g).recs);
        let (keep, gone): (Vec<Rec>, Vec<Rec>) = all.into_iter().partition(|r| r.seen == scan);
        state(g).recs = keep; for r in gone.iter() { let_go(g, r); }
    }
    for ri in 0..state(g).recs.len() {
        let mut r = state(g).recs[ri].clone();
        let a = now - r.born;
        let hang = !r.hang.is_empty();
        if hang && r.phase == 2.0 && ((js::to_i32(scan) + (js::to_i32(r.seed) & 63)) & 63) == 0 {   // about once a second: is what it lies on still there?
            let y = rest_on(g, r.hinge[0], r.hinge[2], r.uh[0], r.uh[2], r.l, r.lie[1] + 0.05).y + 0.046;
            if y < r.lie[1] - 0.03 { r.h0 = r.hinge; r.uh0 = r.uh; r.skew = 0.0; r.th0 = 0.0; r.t0 = a; r.lie = [r.hinge[0], y, r.hinge[2]]; r.phase = 1.0; r.still = false; }
        }
        if hang && r.phase == 0.0 && a >= r.fall_at { r.phase = 1.0; r.th0 = r.th; r.t0 = a; r.still = false; }   // the other end tears out
        if hang && r.phase == 1.0 {   // coming down: the hinge end in free fall, the tray flattening out as it goes, flat as it lands
            let tt = m::sqrt(2.0 * m::max(0.1, r.h0[1] - r.lie[1]) / 9.8); let s = m::min(1.0, m::max(0.0, a - r.t0) / tt);
            r.th = r.th0 * (1.0 - s); if s > r.lay { r.lay = s; } land(&mut r, s);
            if s >= 1.0 { r.phase = 2.0; r.still = true;
                { let seed = seed_of(&mut r); crate::render_particles::particles_emit(g, PKind::Debris, [r.hinge[0] + r.uh[0] * r.l / 2.0, r.lie[1], r.hinge[2] + r.uh[2] * r.l / 2.0], [0.0, 1.0, 0.0], 10.0,
                    EmitOpts { speed: 1.2, speed_jit: Some(0.7), spread: Some(1.4), life: 25.0, life_jit: Some(0.3), size: 0.012, size_jit: Some(0.6), aspect: Some(1.6), pos_jit: Some(m::min(0.3, r.l * 0.3)), color: Some(GLASS), seed: Some(seed), ..EmitOpts::default() }); } }
            pose(g, &mut r);
        } else if hang && !r.still {   // the swing: a damped spring about the rest angle, the ceiling its stop
            let h = m::min(dt, 1.0 / 30.0); r.om += (-f.swing_k * (r.th - r.rest) - f.swing_c * r.om) * h; r.th += r.om * h;
            if r.th < 0.0 { r.th = 0.0; r.om = m::abs(r.om) * 0.3; } if r.th > 1.45 { r.th = 1.45; r.om = -m::abs(r.om) * 0.3; }
            if a > 3.0 && m::abs(r.th - r.rest) < 0.004 && m::abs(r.om) < 0.02 { r.th = r.rest; r.still = true; }
            pose(g, &mut r);
        }
        if a < f.spit_secs {
            let mut sp = GlowAtOut { sparks: 0.0 };
            let gl = glow_at(g, &mut r, m::max(0.0, a), &mut sp); let warm = m::min(1.0, m::max(0.0, a) / 1.5);
            if hang { let e = f.glow_tip * gl; g.tab.frame_boxes[r.hang[5]].emissive = [e * (0.8 + 0.2 * warm), e * (0.9 - 0.25 * warm), e * (1.0 - 0.8 * warm)]; }
            // (every emit carries a seed of the fixture's own, so the particles' serial is never advanced by the look)
            if sp.sparks > 0.0 {
                // out past its free end and down from where it hangs, up off where it lies
                let d: V3 = if r.phase == 2.0 { [r.uh[0] * f.spark_aside, f.spark_up, r.uh[2] * f.spark_aside] } else if hang { [r.uh[0] * f.spark_aside, -f.spark_down, r.uh[2] * f.spark_aside] } else { [r.uh[0] * f.spark_aside * 0.5, -1.0, r.uh[2] * f.spark_aside * 0.5] };
                let seed = seed_of(&mut r);
                crate::render_particles::particles_emit(g, PKind::Spark, r.tip, v3::normalize(d), sp.sparks,
                    EmitOpts { speed: f.spark.speed, speed_jit: Some(f.spark.speed_jit), spread: Some(f.spark.spread), life: f.spark.life, life_jit: Some(f.spark.life_jit), size: f.spark.size, seed: Some(seed), ..EmitOpts::default() });
            }
            if !r.glass_out && a >= f.glass_at && hang { r.glass_out = true;   // born well under the ceiling, in two lots along the fixture
                for k in [0.3, 0.7] { let seed = seed_of(&mut r);
                    crate::render_particles::particles_emit(g, PKind::Debris, [r.hinge[0] + r.uh[0] * r.l * k, r.hinge[1] - 0.3, r.hinge[2] + r.uh[2] * r.l * k], [0.0, -1.0, 0.0], (js::to_i32(f.glass) >> 1) as f64,
                        EmitOpts { speed: 1.3, speed_jit: Some(0.8), spread: Some(1.25), life: 25.0, life_jit: Some(0.3), size: f.glass_size, size_jit: Some(0.6), aspect: Some(1.7), pos_jit: Some(m::min(0.2, r.l * 0.2)), color: Some(GLASS), seed: Some(seed), ..EmitOpts::default() }); }
            }
        } else if hang && g.tab.frame_boxes[r.hang[5]].emissive[0] > 0.0 { g.tab.frame_boxes[r.hang[5]].emissive = [0.0, 0.0, 0.0]; }
        for pi in 0..r.pieces.len() {
            let mut p = r.pieces[pi];
            if let (true, Some(on)) = (p.rest, p.on) {   // what it lay on was moved away: down to what is under it now
                let oc = g.tab.boxes[on].c;
                if oc[0] != p.on_x || oc[2] != p.on_z { let bc = g.tab.frame_boxes[p.box_].c; p.on = None; p.rest = false; p.bounced = true; p.floor_y = floor_at(g, bc[0], bc[2], bc[1] - 0.05); p.v = [0.0, 0.0, 0.0]; p.rate = 0.0; }
            }
            if !p.rest {
                if a < p.delay { r.pieces[pi] = p; continue; }   // (still in the fixture: not drawn yet)
                let h = m::min(dt, 1.0 / 30.0);
                p.v[1] -= 9.8 * h;
                let b = &mut g.tab.frame_boxes[p.box_];
                b.c = [b.c[0] + p.v[0] * h, b.c[1] + p.v[1] * h, b.c[2] + p.v[2] * h]; p.ang += p.rate * h; b.rot = Some(quat::mul(quat::axis_angle(p.axis, p.ang), p.q0));
                let lie = p.floor_y + b.h[1] + 0.002;
                if b.c[1] <= lie + (if p.bounced { 0.0 } else { m::max(b.h[0], b.h[2]) * 0.5 }) {   // down: one dead bounce for a diffuser piece, then flat where it stopped
                    if !p.bounced && !p.glass { p.bounced = true; p.v = [p.v[0] * 0.4, 1.1, p.v[2] * 0.4]; p.rate *= 0.5; b.c[1] = lie + m::max(b.h[0], b.h[2]) * 0.5;
                        let at: V3 = [b.c[0], p.floor_y + 0.03, b.c[2]];
                        { let seed = seed_of(&mut r); crate::render_particles::particles_emit(g, PKind::Debris, at, [0.0, 1.0, 0.0], 8.0,
                            EmitOpts { speed: 1.4, speed_jit: Some(0.7), spread: Some(1.3), life: 25.0, life_jit: Some(0.3), size: 0.012, size_jit: Some(0.6), aspect: Some(1.6), pos_jit: Some(0.15), color: Some(GLASS), seed: Some(seed), ..EmitOpts::default() }); } }
                    else { p.rest = true; b.c[1] = lie; b.rot = Some(quat::yaw(p.yaw + p.ang * 0.3)); b.yaw = p.yaw + p.ang * 0.3; }
                }
            }
            r.pieces[pi] = p;
            out.push(p.box_.into());
        }
        for &b in r.hang.iter() { out.push(b.into()); }
        state(g).recs[ri] = r;
    }
}

/// the tube's glass
const GLASS: V3 = [0.88, 0.92, 0.92];
use crate::render_particles::{EmitOpts, PKind};
/// a record dropped (the fixture repaired, the clock jumped): its boxes are let go, as the TypeScript drops the objects
fn let_go(g: &mut Game, r: &Rec) { for &b in r.hang.iter() { crate::scene_boxes::box_let_go(g, b); } for p in r.pieces.iter() { crate::scene_boxes::box_let_go(g, p.box_); } }
