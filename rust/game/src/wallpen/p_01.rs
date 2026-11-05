//! src/game/wallpen.ts lines 1 to 286
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- small helpers

/// the row `list[i]` reads in a list of `len`: None where the TypeScript reads undefined, a negative or fractional `i` included
fn row_of(i: f64, len: usize) -> Option<usize> {
    let u = i as usize;
    if i >= 0.0 && u as f64 == i && u < len { Some(u) } else { None }
}
/// `g.col.boxes[index]`
fn static_at(g: &Game, index: f64) -> Option<BoxId> { row_of(index, g.col.boxes.len()).map(BoxId::from_ix) }
/// `g.col.dynamicBoxes[i]`
fn leaf_at(g: &Game, i: f64) -> Option<BoxId> { row_of(i, g.col.dynamic_boxes.len()).map(|u| g.col.dynamic_boxes[u]) }
/// `g.col.glassOpen && g.col.glassOpen(bi, p)`. The one rule hung is glassbreak's (glassbreak.ts:213): open where a piece has fallen, or where the burst's view in hand sees no glass
fn pane_open(g: &mut Game, bi: f64, p: V3) -> bool {
    if crate::scene_collision::glass_open(g).is_none() { return false; }
    if crate::glassbreak::glass_break_open_at(g, bi, p) { return true; }
    match g.glass_break.also.clone() {
        None => false,
        Some(crate::glassbreak::GlassBreakAlso::Blown) => crate::glassbreak::glass_break_going_at(g, bi, p),
        Some(crate::glassbreak::GlassBreakAlso::Wary { from, size }) => crate::glassbreak::glass_break_would_give(g, bi, from, Some(size), None),
        Some(crate::glassbreak::GlassBreakAlso::Counted { from, size }) => crate::glassbreak::glass_break_would_give(g, bi, from, Some(size), Some(p)),
    }
}
/// `WALLPEN.thinMax[stuff]`
fn thin_max(g: &Game, stuff: ThinStuff) -> f64 { *g.k.wallpen.thin_max.get(stuff).unwrap() }
/// a ray's answer as a leg of the flight holds it
fn hit_of(h: crate::scene_collision::Raycast) -> StaticHit2 { StaticHit2 { t: h.t, index: h.index, n: h.n } }
/// `flags & mask`, as a condition
fn any_flag(flags: f64, mask: i32) -> bool { js::to_i32(flags) & mask != 0 }
/// `Math.sign(x) || 1`
fn sign_or_one(x: f64) -> f64 { js::or(m::sign(x), 1.0) }

/// the arithmetic of thinN, over a box's numbers
fn thin_n_of(h: V3, yaw: f64) -> V3 {
    let (cs, sn) = (m::cos(yaw), m::sin(yaw));
    if h[0] < h[2] { [cs, 0.0, -sn] } else { [sn, 0.0, cs] }
}
/// the arithmetic of faceN, over a box's numbers
fn face_n_of(c: V3, h: V3, yaw: f64, p: V3) -> V3 {
    let (cs, sn, dx, dy, dz) = (m::cos(yaw), m::sin(yaw), p[0] - c[0], p[1] - c[1], p[2] - c[2]);
    let (lx, lz) = (dx * cs - dz * sn, dx * sn + dz * cs);
    let (ax, ay, az) = (m::abs(lx) / h[0], m::abs(dy) / h[1], m::abs(lz) / h[2]);
    if ay >= ax && ay >= az { return [0.0, sign_or_one(dy), 0.0]; }
    if ax >= az {
        [cs * sign_or_one(lx), 0.0, -sn * sign_or_one(lx)]
    } else {
        [sn * sign_or_one(lz), 0.0, cs * sign_or_one(lz)]
    }
}
/// the hash deflect draws its two numbers from, off the box's hit seed
fn scatter_hash(seed: f64) -> u32 {
    let s = js::to_u32(seed);
    js::to_u32(m::imul((s ^ (s >> 15)) as i32, 0x2C1B3C6D) as f64 + 0x297A2D39u32 as f64)
}
/// the turn of deflect once the cone's half-angle `a` is known: `d` scattered inside it by `seed`
fn scatter(seed: f64, a: f64, d: V3) -> V3 {
    let s = scatter_hash(seed);
    let (u1, u2) = ((s & 0xffff) as f64 / 65535.0, (s >> 16) as f64 / 65535.0);
    let up: V3 = if m::abs(d[1]) < 0.95 { [0.0, 1.0, 0.0] } else { [1.0, 0.0, 0.0] };
    let e1 = v3::normalize(v3::cross(up, d));
    let e2 = v3::cross(d, e1);
    let (r, phi) = (m::tan(a * m::sqrt(u1)), u2 * m::PI * 2.0);
    let (c, s) = (m::cos(phi), m::sin(phi));
    v3::normalize([
        d[0] + (e1[0] * c + e2[0] * s) * r,
        d[1] + (e1[1] * c + e2[1] * s) * r,
        d[2] + (e1[2] * c + e2[2] * s) * r,
    ])
}

// ---------------------------------------------------------------- the skeleton's functions

/// The one gate: no showcase running. (wallpen.ts wallPenLive)
pub fn wall_pen_live(g: &mut crate::game::Game) -> bool { !g.touring }

/// The thin level axis of a box in the world. (wallpen.ts thinN)
pub fn thin_n(g: &mut crate::game::Game, b: BoxId) -> V3 { thin_n_of(g.col.boxes[b].h, g.col.boxes[b].yaw) }

/// The outward face of `b` that `p` lies on. (wallpen.ts faceN)
pub fn face_n(g: &mut crate::game::Game, b: BoxId, p: V3) -> V3 { face_n_of(g.col.boxes[b].c, g.col.boxes[b].h, g.col.boxes[b].yaw, p) }

/// Ceiling over the point; no interior cells built = everywhere. (wallpen.ts roofed)
pub fn roofed(g: &mut crate::game::Game, x: f64, z: f64) -> bool {
    let Some(i) = &g.reach_interior else { return true };
    let w = crate::scene_world::world();
    let (cx, cz) = (m::floor(x - w.min[0]), m::floor(z - w.min[2]));
    if !(cx >= 0.0 && cz >= 0.0 && cx < w.grid_w && cz < w.grid_h) { return false; }
    i.get((cz * w.grid_w + cx) as usize) == Some(&1)
}

/// The stuff of static `index` if a round goes through such a thing at all, by its paint and where it stands. (wallpen.ts stuffOf)
pub fn stuff_of(g: &mut crate::game::Game, index: f64, b: BoxId) -> Option<crate::wallpen::ThinStuff> {
    let bf = crate::scene_boxes::box_flag();
    let mask = js::to_i32(bf.glass) | js::to_i32(bf.no_primary) | js::to_i32(bf.no_shadow) | js::to_i32(bf.dynamic);
    let bx = &g.col.boxes[b];
    let (c, h, em) = (bx.c, bx.h, bx.emissive);
    if any_flag(bx.flags, mask) || bx.rot.is_some() || bx.metal.unwrap_or(false) || em[0] + em[1] + em[2] > 0.0 { return None; }
    let word = bx.does.map(|w| String::from(w.as_str()));
    let w = crate::scene_world::world();
    if c[0] < w.min[0] || c[0] > w.max[0] || c[2] < w.min[2] || c[2] > w.max[2] || crate::destruction::destruction_item_of(g, index).is_some() { return None; }
    for &t in &g.targets {
        if g.tab.sim_lightshome_fixture[t].boxes.contains(&index) { return None; }
    }
    let does = crate::scene_materials::does_of(g, Some(&crate::scene_materials::DoesOfB { does: word }));
    match does.surf {
        crate::scene_materials::Surf::Wood => return Some(ThinStuff::Wood),
        crate::scene_materials::Surf::Cardboard => return Some(ThinStuff::Cardboard),
        crate::scene_materials::Surf::Fabric => return Some(ThinStuff::Fabric),
        crate::scene_materials::Surf::Foliage => return Some(ThinStuff::Foliage),
        crate::scene_materials::Surf::Drywall => {}
        _ => return None,
    }
    if does.board != Some(true) { return None; }
    let part = does.wall == Some(crate::scene_materials::DoesWall::Screen);
    let (y0, y1) = (c[1] - h[1], c[1] + h[1]);
    let full = y0 <= 0.1 && y1 >= g.k.walls.wall_min_top;
    if full && does.wall.is_none() { return None; }
    if !full && part && m::min(h[0], h[2]) * 2.0 <= thin_max(g, ThinStuff::Fabric) { return Some(ThinStuff::Fabric); }
    let n = thin_n(g, b);
    let l = m::max(h[0], h[2]);
    let ax: V3 = [-n[2], 0.0, n[0]];
    let mut in2 = 0;
    for u in [-0.6, 0.0, 0.6] {
        let (px, pz) = (c[0] + ax[0] * u * l, c[2] + ax[2] * u * l);
        if roofed(g, px + n[0] * 0.6, pz + n[2] * 0.6) && roofed(g, px - n[0] * 0.6, pz - n[2] * 0.6) { in2 += 1; }
    }
    if in2 * 2 > 3 { Some(ThinStuff::Drywall) } else { None }
}

/// The box behind an index a raycast reports (>= 1e6: a door leaf), and its stuff. (wallpen.ts thingAt)
pub fn thing_at(g: &mut crate::game::Game, index: f64) -> Option<crate::wallpen::ThingAt> {
    if index >= 1e6 {
        let b = leaf_at(g, index - 1e6)?;
        let bx = &g.col.boxes[b];
        let is_door = bx.name.as_deref().unwrap_or("").starts_with("door_");
        return if !bx.metal.unwrap_or(false) && bx.rot.is_none() && is_door { Some(ThingAt { b, stuff: ThinStuff::Wood }) } else { None };
    }
    let b = static_at(g, index)?;
    let stuff = stuff_of(g, index, b)?;
    Some(ThingAt { b, stuff })
}

/// What static (or leaf) `index` is to a round: its stuff and how thick it stands. (wallpen.ts thinThing)
pub fn thin_thing(g: &mut crate::game::Game, index: f64) -> Option<crate::wallpen::ThinThing> {
    let t = thing_at(g, index)?;
    let h = g.col.boxes[t.b].h;
    let thick = m::min(h[0], h[2]) * 2.0;
    if thick <= thin_max(g, t.stuff) { Some(ThinThing { stuff: t.stuff, thick }) } else { None }
}

/// What thing `index` takes from a round of energy `e` at `at` flying `d`; None = the round is spent in it. (wallpen.ts thinToll)
pub fn thin_toll(g: &mut crate::game::Game, index: f64, at: V3, d: V3, e: f64) -> Option<crate::wallpen::ThinToll> {
    let t = thing_at(g, index)?;
    let b = t.b;
    let h = g.col.boxes[b].h;
    if m::min(h[0], h[2]) * 2.0 > thin_max(g, t.stuff) { return None; }
    let cut = g.col.boxes[b].cut.as_ref().map_or(false, |c| c.len() > 0);
    if cut && crate::scene_cut::cut_boxes_on(g) && !crate::scene_boxes::shape_cut(g, b) { return None; }
    let s = crate::scene_collision::line_box(g, at, d, b)?;
    if s.t1 <= 0.0 { return None; }
    let thru = s.t1;
    if thru > *g.k.wallpen.max_thru.get(t.stuff).unwrap() { return None; }
    let take = *g.k.wallpen.base.get(t.stuff).unwrap() + *g.k.wallpen.per_m.get(t.stuff).unwrap() * thru;
    let e_out = e - take;
    if e_out < g.k.wallpen.min_energy { return None; }
    if thru > 4e-4 {
        let in0 = v3::mad(at, d, 2e-4);
        if born_in(g, in0, d, index) >= 0.0 { return None; }
        if let Some(inner) = crate::scene_collision::raycast(g, in0, d, thru - 4e-4) {
            if inner.index != index { return None; }
        }
    }
    Some(ThinToll { stuff: t.stuff, take, e_out, thru })
}

/// What a round leaving a thing at `p` along `d` is born inside: -1 = open air, else a static's index or a leaf's. (wallpen.ts bornIn)
pub fn born_in(g: &mut crate::game::Game, p: V3, d: V3, self_: f64) -> f64 {
    let mut scratch = embed_scratch();
    let n = crate::scene_collision::gather(g, p[0] - 0.02, p[2] - 0.02, p[0] + 0.02, p[2] + 0.02, p[1] - 0.02, p[1] + 0.02, &mut scratch);
    let bf = crate::scene_boxes::box_flag();
    let (no_shadow, glass) = (js::to_i32(bf.no_shadow), js::to_i32(bf.glass));
    let mut i = 0;
    while (i as f64) < n {
        let bi = scratch[i] as f64;
        i += 1;
        if bi == self_ { continue; }
        let b = BoxId::from_ix(bi as usize);
        let flags = g.col.boxes[b].flags;
        if any_flag(flags, no_shadow) { continue; }
        let Some(s) = crate::scene_collision::line_box(g, p, d, b) else { continue };
        if s.depth < 0.0 || s.t1 <= 0.0 { continue; }
        if any_flag(flags, glass) && pane_open(g, bi, p) { continue; }
        return bi;
    }
    let mut i = 0;
    while i < g.col.dynamic_boxes.len() {
        let at = 1e6 + i as f64;
        if at != self_ {
            let b = g.col.dynamic_boxes[i];
            if let Some(s) = crate::scene_collision::line_box(g, p, d, b) {
                if s.depth >= 0.0 && s.t1 > 0.0 { return at; }
            }
        }
        i += 1;
    }
    -1.0
}

/// The turn: a hashed scatter inside the cone the toll opens, unit vector out. (wallpen.ts deflect)
pub fn deflect(g: &mut crate::game::Game, b: BoxId, at: V3, d: V3, take: f64, e_in: f64) -> V3 {
    let a = g.k.wallpen.deflect_deg * m::PI / 180.0 * v3::clamp(take / m::max(e_in, 1e-6), 0.0, 1.0);
    if a < 1e-6 { return d; }
    scatter(crate::glass::hit_seed(g, b, at), a, d)
}

/// One thin thing met at `t` along the leg `o` + `d`, or None: the round is spent in it. (wallpen.ts crossing)
pub fn crossing(g: &mut crate::game::Game, index: f64, o: V3, d: V3, t: f64, n: V3, e: f64) -> Option<crate::wallpen::WallCross> {
    let at = v3::mad(o, d, t);
    let toll = thin_toll(g, index, at, d, e)?;
    let exit = v3::mad(at, d, toll.thru + EXIT_EPS);
    // (a toll was answered, so the box is there)
    let b = if index >= 1e6 { leaf_at(g, index - 1e6) } else { static_at(g, index) }.unwrap();
    let n_in = if t > 0.0 { n } else { face_n(g, b, at) };
    let n_out = face_n(g, b, v3::mad(at, d, toll.thru));
    let dir_out = deflect(g, b, at, d, toll.take, e);
    let into = born_in(g, exit, d, index);
    Some(WallCross {
        stuff: toll.stuff, take: toll.take, e_out: toll.e_out, thru: toll.thru,
        index, t, n_in, n_out, exit, at, dir_in: d, dir_out, e_in: e, into,
    })
}

/// The flight of a round over panes and thin things; None = it crosses nothing. Pure: nothing is filed until commitWalls. (wallpen.ts thinPath)
pub fn thin_path(g: &mut crate::game::Game, from: V3, dir: V3, range: f64, energy: f64, first: Option<crate::wallpen::StaticHit2>, nth0: Option<f64>) -> Option<crate::wallpen::ThinPath> {
    let nth0 = nth0.unwrap_or(0.0);
    let mut hit: Option<StaticHit2> = match first {
        Some(f) => Some(f),
        None => crate::scene_collision::raycast(g, from, dir, range).map(hit_of),
    };
    if hit.is_none() { return None; }
    let glass = crate::glasspen::glass_pen_live(g);
    let mut segs: Option<Vec<crate::glasspen::RoundSeg>> = None;
    let mut cross: Vec<crate::glasspen::PaneCross> = Vec::new();
    let mut walls: Vec<WallCross> = Vec::new();
    let (mut o, mut d, mut t0, mut e) = (from, dir, 0.0, energy);
    loop {
        let Some(h) = hit else {
            let mut segs = segs.unwrap();
            segs.push(crate::glasspen::RoundSeg { o, d, t0, len: range - t0, e });
            return Some(ThinPath { segs, cross, walls, st: None, len: range });
        };
        let mut x: Option<crate::glasspen::PaneCross> = None;
        let mut w: Option<WallCross> = None;
        let sheet = static_at(g, h.index);
        if h.index < 1e6 && crate::glass::is_sheet(g, sheet) {
            if glass { x = crate::glasspen::pane_crossing(g, h.index, o, d, h.t, e, nth0 + cross.len() as f64); }
        } else if (walls.len() as f64) < g.k.wallpen.max_things {
            w = crossing(g, h.index, o, d, h.t, h.n, e);
        }
        if x.is_none() && w.is_none() {
            let mut segs = segs?;
            segs.push(crate::glasspen::RoundSeg { o, d, t0, len: h.t, e });
            return Some(ThinPath { segs, cross, walls, st: Some(crate::glasspen::RoundPathSt { t: t0 + h.t, index: h.index, n: h.n }), len: t0 + h.t });
        }
        segs.get_or_insert_with(Vec::new).push(crate::glasspen::RoundSeg { o, d, t0, len: h.t, e });
        let into;
        if let Some(mut x) = x {
            x.t = t0 + h.t;
            t0 = x.t + x.thru;
            o = x.exit;
            d = x.dir_out;
            e = x.e_out;
            into = x.into;
            cross.push(x);
        } else {
            let mut w = w.unwrap();
            w.t = t0 + h.t;
            t0 = w.t + w.thru + EXIT_EPS;
            o = w.exit;
            d = w.dir_out;
            e = w.e_out;
            into = w.into;
            walls.push(w);
        }
        if t0 >= range { return Some(ThinPath { segs: segs.unwrap(), cross, walls, st: None, len: range }); }
        hit = if into >= 0.0 {
            Some(StaticHit2 { t: 0.0, index: into, n: v3::scale(d, -1.0) })
        } else {
            crate::scene_collision::raycast(g, o, d, range - t0).map(hit_of)
        };
    }
}

/// The round got as far as `t_hit`: every thin thing it crossed before that is filed, its marks, chips and the wall engine's count. (wallpen.ts commitWalls)
pub fn commit_walls(g: &mut crate::game::Game, walls: &[crate::wallpen::WallCross], t_hit: f64, shooter: CharId, def: &crate::weapons::WeaponDef, lead: bool, third: bool, pel: Option<&mut crate::shotgun::Shell>) {
    let mut pel = pel;
    for x in walls {
        if x.t > t_hit + 1e-6 { break; }
        g.stats.wallpen.crossed += 1;
        if x.index >= 1e6 { g.stats.wallpen.leaves += 1; }
        let s = crate::marks::static_surface(g, x.index);
        let (n_out, out) = (x.n_out, v3::mad(x.at, x.dir_in, x.thru));
        crate::marks::bullet_mark(g, out, n_out, s, def);
        crate::marks::bullet_mark(g, x.at, x.n_in, s, def);
        crate::particle_fx::impact_fx(g, x.at, x.dir_in, crate::particle_fx::ImpactFxHit { n: x.n_in, index: Some(x.index), prop: None, box_: None }, def);
        crate::particle_fx::impact_fx(g, out, v3::scale(x.dir_out, -1.0), crate::particle_fx::ImpactFxHit { n: n_out, index: Some(x.index), prop: None, box_: None }, def);
        if lead { crate::audio_iface::game_audio_like_play(g, "bulletImpact", Some(x.at), Some(0.6), None); }
        if x.index >= 1e6 {
            if let Some(p) = pel.as_deref_mut() { crate::shotgun::pellet_end(p, x.index, x.t, x.at); }
        }
        if third {
            crate::destruction::destruction_round(g, shooter, x.at, x.dir_in, &mut crate::destruction::DestructionRoundHit { n: x.n_in, index: Some(x.index), prop: None, box_: None }, def);
        }
        let line = format!(
            "{:.1}s {} {} ({}, {:.0} cm of path) crossed: energy {:.2} \u{2192} {:.2} (who {})",
            g.time, if x.index >= 1e6 { "leaf" } else { "static" }, js::rem(x.index, 1e6), x.stuff.as_str(), x.thru * 100.0, x.e_in, x.e_out, g.ch[shooter].id,
        );
        g.m.wallpen.log.push(line);
        if g.m.wallpen.log.len() > 24 { g.m.wallpen.log.remove(0); }
    }
}

/// The men's gate: the rule live. (wallpen.ts wallMenLive)
pub fn wall_men_live(g: &mut crate::game::Game) -> bool { wall_pen_live(g) }

/// Would a round of `energy` from `from` get to `to`: Clear, a ThinLine through thin things, or None. Reads only. (wallpen.ts thinLine)
pub fn thin_line(g: &mut crate::game::Game, from: V3, to: V3, energy: f64, short: Option<f64>, first: Option<crate::wallpen::ThinLineFirst>) -> Option<crate::wallpen::ThinLine2> {
    let short = short.unwrap_or(0.3);
    if !wall_pen_live(g) { return None; }
    let d = v3::sub(to, from);
    let l = v3::len(d);
    let reach = l - short;
    if reach <= 1e-3 { return Some(ThinLine2::Clear); }
    let dir = v3::scale(d, 1.0 / l);
    let hit = match first {
        Some(f) => Some(StaticHit2 { t: f.t, index: f.index, n: f.n }),
        None => crate::scene_collision::raycast(g, from, dir, reach).map(hit_of),
    };
    let Some(hit) = hit else { return Some(ThinLine2::Clear) };
    g.stats.wallpen.asked += 1;
    let p = thin_path(g, from, dir, reach, energy, Some(hit), None)?;
    if p.st.is_some() || p.walls.is_empty() { return None; }
    g.stats.wallpen.through += 1;
    let e = p.segs[p.segs.len() - 1].e;
    Some(ThinLine2::ThinLine(ThinLine { panes: p.cross.len() as f64, e, walls: p.walls }))
}

/// The hardest round any gun in the game fires. (wallpen.ts topEnergy)
pub fn top_energy(g: &mut crate::game::Game) -> f64 {
    let k = &g.k.glasspen;
    let mut all: Vec<f64> = vec![k.default_energy];
    all.extend(k.energy.values().copied());
    all.extend(k.by_class.values().copied());
    m::max_n(&all)
}

/// How far a round flies from `from` along `dir`: to the first thing that stops it. (wallpen.ts flightLen)
pub fn flight_len(g: &mut crate::game::Game, from: V3, dir: V3, range: f64, energy: f64) -> f64 {
    let Some(first) = crate::scene_collision::raycast(g, from, dir, range) else { return range };
    if !wall_pen_live(g) { return first.t; }
    let hit = StaticHit2 { t: first.t, index: first.index, n: first.n };
    match thin_path(g, from, dir, range, energy, Some(hit), None) {
        Some(p) => p.len,
        None => first.t,
    }
}

/// The round stopped in something a weaker or shallower round would have gone through: counted. (wallpen.ts spentInWall)
pub fn spent_in_wall(g: &mut crate::game::Game, index: f64) {
    if thing_at(g, index).is_some() { g.stats.wallpen.spent += 1; }
}

/// Panel / console line. (wallpen.ts wallPenSummary)
pub fn wall_pen_summary(g: &mut crate::game::Game) -> String {
    let state = if wall_pen_live(g) { "ON" } else { "on \u{2014} inert while a showcase runs" };
    let st = &g.stats.wallpen;
    let last = match g.m.wallpen.log.last() {
        Some(l) => format!(" \u{b7} last: {}", l),
        None => String::new(),
    };
    format!(
        "rounds through thin walls and doors {} \u{25b8} {} crossed ({} door leaves) \u{b7} {} spent in one{}",
        state, st.crossed, st.leaves, st.spent, last,
    )
}

/// QA: the last crossings filed, oldest first. (wallpen.ts wallPenLog)
pub fn wall_pen_log(g: &mut crate::game::Game) -> Vec<String> { g.m.wallpen.log.clone() }

#[cfg(test)]
mod tests {
    use super::*;

    fn bits(v: V3) -> [u64; 3] { [v[0].to_bits(), v[1].to_bits(), v[2].to_bits()] }

    // the expected bits are the TypeScript's own lines run under bun with EXACT.on (see notes.md)
    #[test]
    fn thin_axis_bits() {
        assert_eq!(bits(thin_n_of([0.05, 1.2, 2.0], 0.7)), TS_THIN_A);
        assert_eq!(bits(thin_n_of([2.0, 1.2, 0.05], 2.1)), TS_THIN_B);
    }

    #[test]
    fn face_bits() {
        let (c, h, yaw) = ([1.0, 1.5, -2.0], [0.06, 1.5, 2.0], 0.7);
        assert_eq!(bits(face_n_of(c, h, yaw, [1.0467, 1.1, -1.9633])), TS_FACE_A);
        assert_eq!(bits(face_n_of(c, h, yaw, [1.2, 2.9999, -2.3])), TS_FACE_B);
        assert_eq!(bits(face_n_of(c, h, yaw, [2.3, 1.1, -0.5])), TS_FACE_C);
        // exactly at the centre: sign 0 || 1
        assert_eq!(bits(face_n_of(c, h, yaw, c)), TS_FACE_D);
    }

    #[test]
    fn scatter_hash_bits() {
        assert_eq!(scatter_hash(0.0), TS_HASH[0]);
        assert_eq!(scatter_hash(1.0), TS_HASH[1]);
        assert_eq!(scatter_hash(2654435769.0), TS_HASH[2]);
        assert_eq!(scatter_hash(4294967295.0), TS_HASH[3]);
    }

    #[test]
    fn scatter_bits() {
        let a = 4.0 * m::PI / 180.0 * v3::clamp(0.225 / m::max(1.0, 1e-6), 0.0, 1.0);
        assert_eq!(bits(scatter(2654435769.0, a, [0.6, 0.0, 0.8])), TS_SCATTER_A);
        assert_eq!(bits(scatter(123456789.0, a, [0.0, 0.98, 0.1989974874213241])), TS_SCATTER_B);
    }

    #[test]
    fn row_of_is_undefined_off_the_list() {
        assert_eq!(row_of(1.0, 2), Some(1));
        assert!(row_of(-1.0, 2).is_none());
        assert!(row_of(2.0, 2).is_none());
        assert!(row_of(0.5, 2).is_none());
        assert!(row_of(f64::NAN, 2).is_none());
    }

    const TS_THIN_A: [u64; 3] = [0x3fe87996529f9d92, 0x0000000000000000, 0xbfe49d6e694619b8];
    const TS_THIN_B: [u64; 3] = [0x3feb9f693feb72fb, 0x0000000000000000, 0xbfe027b304989ec9];
    const TS_FACE_A: [u64; 3] = [0x0000000000000000, 0xbff0000000000000, 0x0000000000000000];
    const TS_FACE_B: [u64; 3] = [0x3fe87996529f9d92, 0x0000000000000000, 0xbfe49d6e694619b8];
    const TS_FACE_C: [u64; 3] = [0x3fe49d6e694619b8, 0x0000000000000000, 0x3fe87996529f9d92];
    const TS_FACE_D: [u64; 3] = [0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000];
    const TS_HASH: [u32; 4] = [695872825, 1435855270, 2426490308, 2963287353];
    const TS_SCATTER_A: [u64; 3] = [0x3fe2fef08aaa411d, 0xbf6c27014300eab3, 0x3fe9c06959cfcda1];
    const TS_SCATTER_B: [u64; 3] = [0x3f8508225d1585a0, 0x3fef5ed532f62798, 0x3fc93b1da2a72655];
}
