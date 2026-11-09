//! FLOOR MESS: what a blast and a broken thing LEAVE once the dust has gone out of the air. FILM where a burst's pall stood, a pale film of settled dust on the floor: a HEART at the
//! burst and LOBES round it, each fitted to the room (none laid across something standing between it and the burst, each reaching only as far as the nearest thing standing between waist
//! and head height, so the dust goes out through a doorway or the hole the blast made, never under a wall that held). The tops standing in a film take their own. RUBBLE: a chunk the pool
//! takes for its slot leaves the crushed mark of itself where it lay. Each is a mark laid only for the picture: where it goes and how it looks is a hash of where it is.
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js;
use crate::m;
use crate::render_decals::{lay, mark_set, DecalKind, DecalOpts};
use crate::scene_boxes::{box_flag, Box as WorldBox};
use crate::v3::{self, quat, V3};
use alloc::vec::Vec;

const DIRS: [[f64; 2]; 8] = [[1.0, 0.0], [0.7071, 0.7071], [0.0, 1.0], [-0.7071, 0.7071], [-1.0, 0.0], [-0.7071, -0.7071], [0.0, -1.0], [0.7071, -0.7071]];
const UP: V3 = [0.0, 1.0, 0.0];
const DOWN: V3 = [0.0, -1.0, 0.0];
/// statics a fit or a film asks about at most
const NEAR: usize = 192;
fn on(g: &Game) -> bool { g.decals.enabled && crate::destruction::chaos_of(g) > 0.0 }
/// 0‥1 off a place and a number: the same mark in the same place every run
fn h(x: f64, z: f64, k: f64) -> f64 {
    crate::debris::hash01((m::imul(js::to_i32(m::round(x * 64.0)), 73856093) ^ m::imul(js::to_i32(m::round(z * 64.0)), 19349663) ^ m::imul(js::to_i32(k + 1.0), js::to_i32(2654435761.0))) as f64)
}
fn look_of(d: &crate::render_decals::Mark) -> f64 { if d.kind == DecalKind::Damage { m::floor(d.param) } else { -1.0 } }
/// may a top take a film: lit geometry that is not a glowing fixture
fn takes(b: &WorldBox) -> bool {
    let f = box_flag();
    js::to_u32(b.flags) & js::to_u32(f.no_primary + f.no_shadow) == 0 && b.emissive[0] + b.emissive[1] + b.emissive[2] <= 0.0
}
/// how far a film centred here may reach: to the nearest thing STANDING (any static whose span meets the band from waist to head height), measured to the box itself, and `want` at most. 0 = the centre is inside one.
fn fit(g: &mut Game, x: f64, z: f64, want: f64) -> f64 {
    let k = g.k.chaos.mess;
    let mut near = alloc::vec![0i32; NEAR];
    let n = crate::scene_collision::gather(g, x - want, z - want, x + want, z + want, k.stand_y, k.clear_y, &mut near);
    let mut r = want;
    for i in 0..n as usize {
        let Some(&id) = g.col.boxes.get(near[i] as usize) else { continue };
        let b = &g.tab.boxes[id];
        if b.rot.is_some() { continue; }
        let (dx, dz, cs, sn) = (x - b.c[0], z - b.c[2], m::cos(b.yaw), m::sin(b.yaw));
        let (ex, ez) = (m::abs(cs * dx - sn * dz) - b.h[0], m::abs(sn * dx + cs * dz) - b.h[2]);
        if ex <= 0.0 && ez <= 0.0 { return 0.0; }
        r = m::min(r, m::hyp2(m::max(0.0, ex), m::max(0.0, ez)) + k.under);
    }
    r
}
/// one film, unless one already lies here
fn film(g: &mut Game, x: f64, z: f64, y: f64, rad: f64, s: f64, on: Option<BoxId>, top: bool, k: f64, heart: bool) {
    let (crowd, look) = (g.k.chaos.mess.crowd, crate::floormess::mess_look().film);
    if mark_set(g).is_some_and(|set| set.list.iter().any(|d| look_of(d) == look && m::abs(d.pos[1] - y) < 0.04 && m::hyp2(d.pos[0] - x, d.pos[2] - z) < crowd * m::min(rad, d.radius))) { return; }
    let a = h(x, z, k + 31.0) * m::PI * 2.0;
    let seed = (js::to_i32(m::floor(h(x, z, k) * 2048.0)) & 0x7ff) | if heart && !top { 0x800 } else { 0 };   // (the look seed's top bit: decals.wgsl draws the soot on a heart alone)
    lay(g, DecalKind::Damage, [x, y, z], UP, Some(rad), &DecalOpts { param: Some(look + m::min(0.999, m::max(0.05, s))), look: Some(true), seed: Some(seed as f64), tangent: Some([m::cos(a), 0.0, m::sin(a)]), attach: on, ..Default::default() });
}
/// ONE film: on the floor at (x, z) reaching `want` at most, and on the tops that stand in it. `k` tells this patch from its burst's others; `heart` = the burst's own middle (the one film that carries the soot).
fn patch(g: &mut Game, x: f64, z: f64, want: f64, strength: f64, k: f64, heart: bool) {
    let K = g.k.chaos.mess;
    let r = fit(g, x, z, want);
    if r < K.min_radius { return; }
    film(g, x, z, 0.0, r, strength, None, false, k, heart);   // (the level's floor: a rug lying on it is inside the mark's own plane tolerance)
    // the tops that stand in it: a mark's plane takes every surface at its height, so one film a height dusts every top of that height inside the reach; the nearest three heights
    let mut near = alloc::vec![0i32; NEAR];
    let n = crate::scene_collision::gather(g, x - r, z - r, x + r, z + r, K.top.lo, K.top.hi, &mut near);
    let mut hs: Vec<(f64, f64)> = Vec::new();
    for i in 0..n as usize {
        let Some(&id) = g.col.boxes.get(near[i] as usize) else { continue };
        let b = &g.tab.boxes[id];
        if b.rot.is_some() || !takes(b) || m::min(b.h[0], b.h[2]) < 0.12 { continue; }
        let (y, d) = (b.c[1] + b.h[1], m::hyp2(b.c[0] - x, b.c[2] - z) - m::max(b.h[0], b.h[2]));
        if y < K.top.lo || y > K.top.hi || d > r * 0.8 { continue; }
        let q = m::round(y * 50.0) / 50.0;
        match hs.iter_mut().find(|e| m::abs(e.0 - q) < 0.03) { Some(e) => e.1 = m::min(e.1, d), None => hs.push((q, d)) }
    }
    hs.sort_by(|a, b| a.1.total_cmp(&b.1).then(a.0.total_cmp(&b.0)));
    for &(y, _) in hs.iter().take(3) { film(g, x, z, y, r, strength * K.top.strength, None, true, k, heart); }
    // …and the prop standing under its middle (a prop's rides the prop)
    if let Some(ph) = crate::props::prop_system_raycast(g, [x, K.clear_y, z], DOWN, K.clear_y - K.top.lo) {
        if ph.n[1] > 0.9 && takes(&g.tab.boxes[ph.box_]) {
            let (y, half) = (K.clear_y - ph.t, m::max(g.tab.boxes[ph.box_].h[0], g.tab.boxes[ph.box_].h[2]));
            if y <= K.top.hi && half > 0.15 { film(g, x, z, y, m::min(r, half * 1.15), strength * K.top.strength, Some(ph.box_), true, k, heart); }
        }
    }
}

/// A HEART and its LOBES round (x, z). `face` (a unit x, z) turns the lobes into a fan that way (a charge's side of its wall), else they ring it.
pub fn spread(g: &mut crate::game::Game, x: f64, z: f64, s: crate::floormess::Spread, size: f64, face: Option<[f64; 2]>) {
    let clear_y = g.k.chaos.mess.clear_y;
    let stand_y = g.k.chaos.mess.stand_y;
    // a heart hard against something standing moves off it first: half its reach of room, where there is room
    let (mut cx, mut cz) = (x, z);
    let room = s.core * size * 0.5;
    for _ in 0..2 { for w in DIRS {
        let Some(hit) = crate::scene_collision::raycast(g, [cx, clear_y, cz], [w[0], 0.0, w[1]], room) else { continue };
        let (nx, nz) = (cx - w[0] * (room - hit.t), cz - w[1] * (room - hit.t));
        if !crate::scene_collision::solid_blocked(g, [cx, clear_y, cz], [nx, clear_y, nz], None) { cx = nx; cz = nz; }
    } }
    patch(g, cx, cz, s.core * size, s.strength, 0.0, true);
    let (a0, arc) = match face { Some(f) => (m::atan2(f[0], f[1]), m::PI * 0.9), None => (h(x, z, 97.0) * m::PI * 2.0, m::PI * 2.0) };
    let mut k = 0.0;
    while k < s.lobes {
        let (j, j2) = (h(x, z, k * 2.0 + 1.0), h(x, z, k * 2.0 + 2.0));
        let a = a0 + (if face.is_some() { ((k + 0.5) / s.lobes - 0.5) * arc } else { k * arc / s.lobes }) + (j - 0.5) * 0.6 * arc / s.lobes;
        let d = s.reach * size * (0.75 + 0.5 * j2);
        let (px, pz) = (cx + m::sin(a) * d, cz + m::cos(a) * d);
        k += 1.0;
        if crate::scene_collision::solid_blocked(g, [cx, clear_y, cz], [px, clear_y, pz], None) || crate::scene_collision::solid_blocked(g, [cx, stand_y, cz], [px, stand_y, pz], None) { continue; }   // (something stands between: the dust did not go there)
        patch(g, px, pz, s.lobe * size * (0.8 + 0.4 * j), s.strength * (0.6 + 0.3 * j2), k, false);
    }
}

/// A BURST went off at `at`; `size` 1 = the hand grenade's.
pub fn lay_burst(g: &mut Game, at: V3, size: Option<f64>) {
    let K = g.k.chaos.mess;
    if !on(g) || at[1] > K.roof_y { return; }   // (a burst on a roof leaves the room under it alone)
    let b = K.burst;
    spread(g, at[0], at[2], crate::floormess::Spread { core: b.core, lobes: b.lobes, reach: b.reach, lobe: b.lobe, strength: b.strength }, m::max(0.6, m::min(1.6, size.unwrap_or(1.0))), None);
}

/// A WALL CHARGE went off: `p` on the wall's face at the charge, `n` off that face toward the charge's side, the wall `thick` m, `through` whether it opened a way (the far side's film goes only then).
pub fn lay_wall(g: &mut Game, p: V3, n: V3, thick: f64, through: bool) {
    if !on(g) { return; }
    let w = g.k.chaos.mess.wall;
    spread(g, p[0] + n[0] * w.out, p[2] + n[2] * w.out, crate::floormess::Spread { core: w.near.core, lobes: w.near.lobes, reach: w.near.reach, lobe: w.near.lobe, strength: w.near.strength }, 1.0, Some([n[0], n[2]]));
    if through { spread(g, p[0] - n[0] * (thick + w.out), p[2] - n[2] * (thick + w.out), crate::floormess::Spread { core: w.far.core, lobes: w.far.lobes, reach: w.far.reach, lobe: w.far.lobe, strength: w.far.strength }, 1.0, Some([-n[0], -n[2]])); }
}

/// A wall or a piece of cover OPENED over a rect for a reason that is not a blast's: a small film at its foot, each side of its long line.
pub fn lay_opened(g: &mut Game, rect: [f64; 4]) {
    if !on(g) { return; }
    let f = g.k.chaos.mess.foot;
    let [x0, z0, x1, z1] = rect;
    let (cx, cz, along_x) = ((x0 + x1) / 2.0, (z0 + z1) / 2.0, x1 - x0 >= z1 - z0);
    let (len, half) = (if along_x { x1 - x0 } else { z1 - z0 }, (if along_x { z1 - z0 } else { x1 - x0 }) / 2.0);
    for s in [1.0, -1.0] {
        let (px, pz) = (cx + if along_x { 0.0 } else { s * (half + f.out) }, cz + if along_x { s * (half + f.out) } else { 0.0 });
        patch(g, px, pz, f.radius + f.per_metre * len, f.strength, if s > 0.0 { 3.0 } else { 4.0 }, false);
    }
}

/// The debris pool took a resting chunk's slot: the crushed mark of it where it lay.
pub fn lay_evicted(g: &mut Game, c: &crate::out_events::OutMessEvicted) {
    if !on(g) { return; }
    let R = g.k.chaos.mess.rubble;
    let long = m::max(c.h[0], m::max(c.h[1], c.h[2]));
    if long < R.least { return; }
    let (x, z, rad) = (c.c[0], c.c[2], m::max(R.min, m::min(R.max, long * R.grow)));
    // what it lay on: the level under it (a chunk on a desk marks the desk), a prop's top rides the prop
    let from = [x, c.c[1] + 0.02, z];
    let st = crate::scene_collision::raycast(g, from, DOWN, c.c[1] + 0.3);
    let ph = crate::props::prop_system_raycast(g, from, DOWN, c.c[1] + 0.3);
    let (mut y, mut on_box) = (0.0, None);
    match (&ph, &st) {
        (Some(p), _) if st.as_ref().map_or(true, |s| p.t < s.t) => { if p.n[1] < 0.9 || !takes(&g.tab.boxes[p.box_]) { return; } y = from[1] - p.t; on_box = Some(p.box_); }
        (_, Some(s)) if s.index < 1e6 && s.n[1] > 0.9 => y = from[1] - s.t,
        _ => if c.c[1] > 0.6 { return; },   // (nothing under it that a look down finds, and it is not on the floor: no mark in the air)
    }
    if y > c.c[1] + 0.02 || c.c[1] - y > long + 0.25 { return; }
    let looks = crate::floormess::mess_look();
    // (a kind is no stuff of these: what it is made of is said by its `mat` or it is grit)
    let look = if c.mat.is_some_and(|m| matches!(m, crate::break_table::Mat::Wood | crate::break_table::Mat::Cardboard | crate::break_table::Mat::Fabric | crate::break_table::Mat::Paper | crate::break_table::Mat::Carpet | crate::break_table::Mat::Plant)) { looks.splinter } else { looks.grit };
    if mark_set(g).is_some_and(|s| s.list.iter().any(|d| { let l = look_of(d); (l == looks.grit || l == looks.splinter) && m::abs(d.pos[1] - y) < 0.04 && m::hyp2(d.pos[0] - x, d.pos[2] - z) < R.crowd * m::min(rad, d.radius) })) { return; }
    // long along the way the piece lay: its longest axis as it came to rest, laid flat
    let il = if c.h[0] >= c.h[1] && c.h[0] >= c.h[2] { 0 } else if c.h[1] >= c.h[2] { 1 } else { 2 };
    let ax: V3 = core::array::from_fn(|i| if i == il { 1.0 } else { 0.0 });
    let wa = crate::render_decals::to_world([0.0; 3], c.yaw, c.rot, ax, false);
    let size = m::min(1.0, long / 0.3);
    let tangent = if m::hyp2(wa[0], wa[2]) > 0.2 { [wa[0], 0.0, wa[2]] } else { [1.0, 0.0, 0.0] };
    lay(g, DecalKind::Damage, [x, y, z], UP, Some(rad), &DecalOpts { param: Some(look + m::min(0.999, R.strength * (0.6 + 0.4 * size))), look: Some(true), seed: Some(m::floor(h(x, z, 7.0) * 4096.0)), tangent: Some(tangent), attach: on_box, ..Default::default() });
}
