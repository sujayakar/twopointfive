pub use ::game_types::wallpen::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js;
use crate::m;
use crate::v3::{self, V3};
use alloc::{vec, vec::Vec};

// ---------------------------------------------------------------- small helpers

/// the row `list[i]` reads in a list of `len`: None past its end and for a negative or fractional `i`
fn row_of(i: f64, len: usize) -> Option<usize> {
    let u = i as usize;
    if i >= 0.0 && u as f64 == i && u < len { Some(u) } else { None }
}
/// `g.col.boxes[index]`
fn static_at(g: &Game, index: f64) -> Option<BoxId> { row_of(index, g.col.boxes.len()).map(|u| g.col.boxes[u]) }
/// `g.col.dynamicBoxes[i]`
fn leaf_at(g: &Game, i: f64) -> Option<BoxId> { row_of(i, g.col.dynamic_boxes.len()).map(|u| g.col.dynamic_boxes[u]) }
/// `WALLPEN.thinMax[stuff]`
fn thin_max(g: &Game, stuff: ThinStuff) -> f64 { *g.k.wallpen.thin_max.get(stuff).unwrap() }
/// a ray's answer as a leg of the flight holds it
fn hit_of(h: crate::scene_collision::Raycast) -> crate::glasspen::RoundPathSt { crate::glasspen::RoundPathSt { t: h.t, index: h.index, n: h.n } }
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

// ---------------------------------------------------------------- the module's functions

/// The thin level axis of a box in the world.
fn thin_n(g: &crate::game::Game, b: BoxId) -> V3 {
    let bx = &g.tab.boxes[b];
    thin_n_of(bx.h, bx.yaw)
}

/// The outward face of `b` that `p` lies on.
fn face_n(g: &crate::game::Game, b: BoxId, p: V3) -> V3 {
    let bx = &g.tab.boxes[b];
    face_n_of(bx.c, bx.h, bx.yaw, p)
}

/// Ceiling over the point; no interior cells built = everywhere.
fn roofed(g: &crate::game::Game, x: f64, z: f64) -> bool {
    let Some(i) = &g.reach_interior else { return true };
    let w = crate::scene_world::world();
    let (cx, cz) = (m::floor(x - w.min[0]), m::floor(z - w.min[2]));
    if !(cx >= 0.0 && cz >= 0.0 && cx < w.grid_w && cz < w.grid_h) { return false; }
    i.get((cz * w.grid_w + cx) as usize) == Some(&1)
}

/// The stuff of static `index` if a round goes through such a thing at all, by its paint and where it stands.
fn stuff_of(g: &crate::game::Game, index: f64, b: BoxId) -> Option<crate::wallpen::ThinStuff> {
    let mask = crate::scene_boxes::box_flag::GLASS as i32 | crate::scene_boxes::box_flag::NO_PRIMARY as i32 | crate::scene_boxes::box_flag::NO_SHADOW as i32 | crate::scene_boxes::box_flag::DYNAMIC as i32;
    let bx = &g.tab.boxes[b];
    let (c, h, em, word) = (bx.c, bx.h, bx.emissive, bx.does);
    if any_flag(bx.flags, mask) || bx.rot.is_some() || bx.metal.unwrap_or(false) || em[0] + em[1] + em[2] > 0.0 { return None; }
    let w = crate::scene_world::world();
    if c[0] < w.min[0] || c[0] > w.max[0] || c[2] < w.min[2] || c[2] > w.max[2] || crate::destruction::destruction_item_of(g, index).is_some() { return None; }
    for &t in &g.targets {
        if g.tab.fixture[t].boxes.contains(&index) { return None; }
    }
    let does = crate::scene_materials::does_of(word);
    match does.surf {
        crate::scene_materials::Surf::Wood => return Some(ThinStuff::Wood),
        crate::scene_materials::Surf::Cardboard => return Some(ThinStuff::Cardboard),
        crate::scene_materials::Surf::Fabric => return Some(ThinStuff::Fabric),
        crate::scene_materials::Surf::Foliage => return Some(ThinStuff::Foliage),
        crate::scene_materials::Surf::Drywall => {}
        _ => return None,
    }
    if !does.board { return None; }
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

/// The box behind an index a raycast reports (>= 1e6: a door leaf), and its stuff.
fn thing_at(g: &crate::game::Game, index: f64) -> Option<crate::wallpen::ThingAt> {
    if index >= 1e6 {
        let b = leaf_at(g, index - 1e6)?;
        let bx = &g.tab.boxes[b];
        return if !bx.metal.unwrap_or(false) && bx.rot.is_none() { Some(ThingAt { b, stuff: ThinStuff::Wood }) } else { None };
    }
    let b = static_at(g, index)?;
    let stuff = stuff_of(g, index, b)?;
    Some(ThingAt { b, stuff })
}

/// What thing `index` takes from a round of energy `e` at `at` flying `d`; None = the round is spent in it.
fn thin_toll(g: &crate::game::Game, index: f64, at: V3, d: V3, e: f64) -> Option<crate::wallpen::ThinToll> {
    let t = thing_at(g, index)?;
    let b = t.b;
    let h = g.tab.boxes[b].h;
    if m::min(h[0], h[2]) * 2.0 > thin_max(g, t.stuff) { return None; }
    let cut = g.tab.boxes[b].cut.as_ref().map_or(false, |c| c.len() > 0);
    if cut && !crate::scene_boxes::shape_cut(g, b.into()) { return None; }
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
    Some(ThinToll { take, e_out, thru })
}

/// What a round leaving a thing at `p` along `d` is born inside: -1 = open air, else a static's index or a leaf's.
fn born_in(g: &crate::game::Game, p: V3, d: V3, self_: f64) -> f64 {
    let mut scratch = embed_scratch();
    let n = crate::scene_collision::gather(g, p[0] - 0.02, p[2] - 0.02, p[0] + 0.02, p[2] + 0.02, p[1] - 0.02, p[1] + 0.02, &mut scratch);
    let (no_shadow, glass) = (crate::scene_boxes::box_flag::NO_SHADOW as i32, crate::scene_boxes::box_flag::GLASS as i32);
    let mut i = 0;
    while (i as f64) < n {
        let at = scratch[i] as usize;
        let bi = scratch[i] as f64;
        i += 1;
        if bi == self_ { continue; }
        let b = g.col.boxes[at];
        let flags = g.tab.boxes[b].flags;
        if any_flag(flags, no_shadow) { continue; }
        let Some(s) = crate::scene_collision::line_box(g, p, d, b) else { continue };
        if s.depth < 0.0 || s.t1 <= 0.0 { continue; }
        if any_flag(flags, glass) && crate::glassbreak::glass_break_rule_open(g, bi, p) { continue; }
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

/// The turn: a hashed scatter inside the cone the toll opens, unit vector out.
fn deflect(g: &crate::game::Game, b: BoxId, at: V3, d: V3, take: f64, e_in: f64) -> V3 {
    let a = g.k.wallpen.deflect_deg * m::PI / 180.0 * v3::clamp(take / m::max(e_in, 1e-6), 0.0, 1.0);
    if a < 1e-6 { return d; }
    scatter(crate::glass::hit_seed(g, b, at), a, d)
}

/// One thin thing met at `t` along the leg `o` + `d`, or None: the round is spent in it.
fn crossing(g: &crate::game::Game, index: f64, o: V3, d: V3, t: f64, n: V3, e: f64) -> Option<crate::wallpen::WallCross> {
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
        e_out: toll.e_out, thru: toll.thru,
        index, t, n_in, n_out, exit, at, dir_in: d, dir_out, into,
    })
}

/// The flight of a round over panes and thin things; None = it crosses nothing. Pure: nothing is filed until commitWalls.
pub(super) fn thin_path(g: &crate::game::Game, from: V3, dir: V3, range: f64, energy: f64, first: Option<Option<crate::glasspen::RoundPathSt>>, nth0: Option<f64>) -> Option<crate::wallpen::ThinPath> {
    let nth0 = nth0.unwrap_or(0.0);
    let mut hit: Option<crate::glasspen::RoundPathSt> = match first {
        Some(f) => f,
        None => crate::scene_collision::raycast(g, from, dir, range).map(hit_of),
    };
    if hit.is_none() { return None; }
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
            x = crate::glasspen::pane_crossing(g, h.index, o, d, h.t, e, nth0 + cross.len() as f64);
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
            Some(crate::glasspen::RoundPathSt { t: 0.0, index: into, n: v3::scale(d, -1.0) })
        } else {
            crate::scene_collision::raycast(g, o, d, range - t0).map(hit_of)
        };
    }
}

/// The round got as far as `t_hit`: every thin thing it crossed before that is filed, its marks, chips and the wall engine's count.
pub(super) fn commit_walls(g: &mut crate::game::Game, walls: &[crate::wallpen::WallCross], t_hit: f64, shooter: CharId, def: &crate::weapons::WeaponDef, lead: bool, third: bool, pel: Option<&mut crate::shotgun::Shell>) {
    let mut pel = pel;
    for x in walls {
        if x.t > t_hit + 1e-6 { break; }
        g.stats.wallpen.crossed += 1;
        let s = crate::marks::static_surface(g, x.index);
        let (n_out, out) = (x.n_out, v3::mad(x.at, x.dir_in, x.thru));
        crate::marks::bullet_mark(g, out, n_out, s, def);
        crate::marks::bullet_mark(g, x.at, x.n_in, s, def);
        crate::particle_fx::impact_fx(g, x.at, x.dir_in, crate::combat::RoundMet { n: x.n_in, index: Some(x.index), prop: None, box_: None }, def);
        crate::particle_fx::impact_fx(g, out, v3::scale(x.dir_out, -1.0), crate::combat::RoundMet { n: n_out, index: Some(x.index), prop: None, box_: None }, def);
        if lead { crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::BulletImpact, Some(x.at), Some(0.6), None); }
        if x.index >= 1e6 {
            if let Some(p) = pel.as_deref_mut() { crate::shotgun::pellet_end(p, x.index, x.t, x.at); }
        }
        if third {
            crate::destruction::destruction_round(g, shooter, x.at, x.dir_in, &crate::combat::RoundMet { n: x.n_in, index: Some(x.index), prop: None, box_: None }, def);
        }
    }
}

/// Would a round of `energy` from `from` get to `to`: Clear, a ThinLine through thin things, or None. Reads only.
pub fn thin_line(g: &crate::game::Game, from: V3, to: V3, energy: f64, short: Option<f64>, first: Option<crate::wallpen::ThinLineFirst>) -> Option<crate::wallpen::ThinLine2> {
    let short = short.unwrap_or(0.3);
    let d = v3::sub(to, from);
    let l = v3::len(d);
    let reach = l - short;
    if reach <= 1e-3 { return Some(ThinLine2::Clear); }
    let dir = v3::scale(d, 1.0 / l);
    let hit = match first {
        Some(f) => Some(crate::glasspen::RoundPathSt { t: f.t, index: f.index, n: f.n }),
        None => crate::scene_collision::raycast(g, from, dir, reach).map(hit_of),
    };
    let Some(hit) = hit else { return Some(ThinLine2::Clear) };
    let p = thin_path(g, from, dir, reach, energy, Some(Some(hit)), None)?;
    if p.st.is_some() || p.walls.is_empty() { return None; }
    Some(ThinLine2::ThinLine(ThinLine { panes: p.cross.len() as f64, walls: p.walls }))
}

/// The hardest round any gun in the game fires.
pub fn top_energy(g: &crate::game::Game) -> f64 {
    let k = &g.k.glasspen;
    let mut all: Vec<f64> = vec![k.default_energy];
    all.extend(k.energy.values().copied());
    all.extend(k.by_class.values().copied());
    m::max_n(&all)
}

/// How far a round flies from `from` along `dir`: to the first thing that stops it.
pub fn flight_len(g: &crate::game::Game, from: V3, dir: V3, range: f64, energy: f64) -> f64 {
    let Some(first) = crate::scene_collision::raycast(g, from, dir, range) else { return range };
    let first_t = first.t;
    match thin_path(g, from, dir, range, energy, Some(Some(hit_of(first))), None) {
        Some(p) => p.len,
        None => first_t,
    }
}

/// The round stopped in something a weaker or shallower round would have gone through: counted.

#[cfg(test)]
mod tests {
    use super::*;

    fn near(a: V3, b: V3) -> bool { v3::dist(a, b) < 1e-12 }

    /// A wall's facing is along its thin side, whichever side that is and however the wall is turned: a round's toll is counted across it.
    #[test]
    fn a_walls_thin_axis_is_across_it() {
        assert!(near(thin_n_of([0.05, 1.2, 2.0], 0.0), [1.0, 0.0, 0.0]));
        assert!(near(thin_n_of([2.0, 1.2, 0.05], 0.0), [0.0, 0.0, 1.0]));
        for yaw in [0.7, 2.1] {
            for h in [[0.05, 1.2, 2.0], [2.0, 1.2, 0.05]] {
                let n = thin_n_of(h, yaw);
                assert!((v3::len(n) - 1.0).abs() < 1e-12 && n[1] == 0.0);
                // a turned box's axis: its face is reached along this normal by its half-thickness
                let thin = if h[0] < h[2] { h[0] } else { h[2] };
                let on_face = [n[0] * thin, 0.0, n[2] * thin];
                assert!(near(face_n_of([0.0; 3], [h[0], 1.2, h[2]], yaw, on_face), n), "the thin face's own normal");
            }
        }
    }

    /// The face a point is on is the one it is nearest by the box's own proportions, pointing out of the box.
    #[test]
    fn a_face_points_out_of_the_box_on_the_side_the_point_is() {
        let (c, h, yaw) = ([1.0, 1.5, -2.0], [0.06, 1.5, 2.0], 0.7);
        let (cs, sn) = (m::cos(yaw), m::sin(yaw));
        let (x_axis, z_axis): (V3, V3) = ([cs, 0.0, -sn], [sn, 0.0, cs]);
        let on = |x: f64, y: f64, z: f64| -> V3 { [c[0] + x_axis[0] * x + z_axis[0] * z, c[1] + y, c[2] + x_axis[2] * x + z_axis[2] * z] };
        assert_eq!(face_n_of(c, h, yaw, on(0.0, 1.5, 0.5)), [0.0, 1.0, 0.0], "on the top");
        assert_eq!(face_n_of(c, h, yaw, on(0.0, -1.5, 0.5)), [0.0, -1.0, 0.0], "under it");
        assert!(near(face_n_of(c, h, yaw, on(0.06, 0.2, 0.5)), x_axis));
        assert!(near(face_n_of(c, h, yaw, on(-0.06, 0.2, 0.5)), v3::scale(x_axis, -1.0)));
        assert!(near(face_n_of(c, h, yaw, on(0.01, 0.2, 2.0)), z_axis));
        assert!(near(face_n_of(c, h, yaw, on(0.01, 0.2, -2.0)), v3::scale(z_axis, -1.0)));
        assert_eq!(face_n_of(c, h, yaw, c), [0.0, 1.0, 0.0], "dead in the middle it still answers, upward");
    }

    fn turn_of(a: V3, b: V3) -> f64 { m::acos(m::min(1.0, v3::dot(a, b))) }

    /// A round through a thin wall comes out inside the cone the toll opens and no wider, and the same hole turns it the same way every time, on any heading.
    #[test]
    fn a_round_is_turned_inside_its_cone_by_the_hole() {
        let a = 4.0 * m::PI / 180.0;
        for d in [[0.6, 0.0, 0.8], [0.0, 0.98, 0.1989974874213241], [1.0, 0.0, 0.0]] {
            let mut ways = Vec::new();
            for seed in [0.0, 1.0, 123456789.0, 2654435769.0, 4294967295.0] {
                let out = scatter(seed, a, d);
                assert!((v3::len(out) - 1.0).abs() < 1e-12);
                assert!(turn_of(out, d) <= a + 1e-9, "turned {} by seed {}", turn_of(out, d), seed);
                assert_eq!(out, scatter(seed, a, d));
                ways.push(out);
            }
            for i in 1..ways.len() { assert!(!near(ways[i], ways[0]), "two holes turned it the same way"); }
            assert!(near(scatter(77.0, 0.0, d), d), "no toll, no turn");
        }
    }

    /// A row off the list is no row, whichever way it is off: a static or a leaf a ray can't name is never read.
    #[test]
    fn a_row_off_the_list_is_none() {
        assert_eq!(row_of(1.0, 2), Some(1));
        assert!(row_of(-1.0, 2).is_none());
        assert!(row_of(2.0, 2).is_none());
        assert!(row_of(0.5, 2).is_none());
        assert!(row_of(f64::NAN, 2).is_none());
    }
}
