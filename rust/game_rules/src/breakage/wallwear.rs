#![allow(unused_imports, unused_variables)]
#![allow(unused_variables)]
//! WALL WEAR: what a breakable wall or piece of cover SHOWS while it is being shot, before it gives. Look only: wallbreak counts a round into a cell and opens the cell when the count is
//! up; here every counted round's mark is sized by how far gone its cell is (a pock, a small hole in a ring of blown plaster, cracks out of it), the marks the cell already carries grow
//! with it, and on a thin partition the same mark shows on the far face, with a puff and chips thrown out of that side. It adds no mark: the round's own is made into this. A THROUGH mark
//! sits on the wall's mid-plane with a radius of hypot(half thickness, reach), and the shader paints the wall's two faces from it.

pub use ::game_types::wallwear::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};

#[allow(unused_imports)]
use crate::out_events::*;
use crate::render_decals::{mark_asked, Ask};

/// (src/game/wallwear.ts:58 WallWear.round)
pub fn wall_wear_round(g: &mut crate::game::Game, box_: BoxId, surf: crate::scene_materials::Surf, key: f64, counted: f64, frac: f64, hit_p: V3, dir: V3) { let e = OutWallWearRound { box_, surf, key, counted, frac, hit_p, dir }; let ours = g.fog_of_war.filing_ours; mark_asked(g, Ask::Round(e, ours)); }

/// (src/game/wallwear.ts:88 WallWear.reset)
pub fn wall_wear_reset(g: &mut crate::game::Game) { mark_asked(g, Ask::Reset); }

use crate::out_events::{OutEvent, OutParticlesEmit, OutWallWearRound};
use crate::render_decals::{mark_of, mark_set_mut, wear_mark, DecalKind, MarkSet};
use crate::render_particles::{EmitOpts, EmitShape, PKind, MAX_NEW_PER_FRAME};
use crate::scene_materials::Surf;

const PALE: V3 = [0.86, 0.85, 0.82];
/// the mark's material code (decals.wgsl wearMark): plaster · wood · torn board or cloth
fn mat_of(s: Surf) -> f64 { match s { Surf::Wood => 1.0, Surf::Cardboard | Surf::Fabric => 2.0, _ => 0.0 } }
/// strength → the record: param = look + material + strength; radius = the reach, or hypot(half thickness, reach) for a mark on the mid-plane
fn shape(d: &mut crate::render_decals::Mark, s: f64) {
    let k = wear_mark();
    let reach = k.reach0 + k.reach1 * s;
    d.param = k.look + d.wear_mat + s;
    d.radius = if d.wear_half > 0.0 { m::hyp2(d.wear_half, reach) } else { reach };
}

/// A round was counted on a cell of a breakable. `key` names the cell that was hit, `counted` the one the count went to (the same, but for a sill under a standing upper), `frac` that cell's count ÷ what it takes (≥ 1: it goes this frame).
pub fn wear_round(g: &mut Game, e: &OutWallWearRound, ours: bool) {
    if !g.destr.walls.wear.on { return; }
    let w = crate::wallwear::wear();
    if e.frac >= 1.0 { mark_set_mut(g).wear.cells.retain(|c| c.0 != e.counted); return; }   // the cell is gone this frame: what it throws is the break's own (a sill's list is kept: it stands)
    let b = g.tab.boxes[e.box_].clone();
    // the thin axis of the box is the one a through mark crosses
    let (cs, sn) = (m::cos(b.yaw), m::sin(b.yaw));
    let thin_x = b.h[0] <= b.h[2];
    let half = if thin_x { b.h[0] } else { b.h[2] };
    let axis = if thin_x { [cs, 0.0, -sn] } else { [sn, 0.0, cs] };
    let f = m::max(0.0, m::min(1.0, e.frac));
    let base = m::pow(f, w.curve);
    let strength = |seed: u32| m::min(0.999, base * (1.0 - w.vary * (seed as f64 / 4096.0)));
    let s = mark_set_mut(g);
    // every mark the cell carries grows with it
    if let Some(at) = s.wear.cells.iter().position(|c| c.0 == e.key) {
        let list = core::mem::take(&mut s.wear.cells[at].1);
        for &serial in list.iter() { if let Some(d) = mark_of(s, serial) { let v = strength(d.seed); if v > crate::js::rem(d.param, 1.0) + 1e-4 { shape(d, v); } } }
        s.wear.cells[at].1 = list;
    }
    // this round's own mark
    let Some(i) = own(s, e.hit_p) else { return };
    let (serial, n) = (s.list[i].serial, s.list[i].n);
    let through = m::abs(v3::dot(n, axis)) > 0.9 && half >= wear_mark().min_half && half <= wear_mark().max_half;
    let d = &mut s.list[i];
    if through { d.pos = v3::mad(d.pos, d.n, -half); }   // onto the mid-plane: both faces are then on its two planes
    d.kind = DecalKind::Damage;
    d.wear_half = if through { half } else { 0.0 };
    d.wear_mat = mat_of(e.surf);
    let v = strength(d.seed);
    shape(d, v);
    match s.wear.cells.iter().position(|c| c.0 == e.key) {
        Some(at) => s.wear.cells[at].1.push(serial),
        None => {
            if s.wear.cells.len() as f64 >= w.max_cells { s.wear.cells.remove(0); }
            s.wear.cells.push((e.key, alloc::vec![serial]));
        }
    }
    throw_out(g, &b, e.surf, e.hit_p, e.dir, n, half, through, base, ours);
}

/// this round's bullet mark: the newest world-fixed bullet among the last few marks (the lane may have laid a dent or a shred after it), laid after the last one taken, within its own radius of the hit
fn own(s: &mut MarkSet, hit: V3) -> Option<usize> {
    let n = s.list.len();
    for i in (n.saturating_sub(3)..n).rev() {
        let d = &s.list[i];
        if d.kind != DecalKind::Bullet || d.rides.is_some() || d.serial <= s.wear.taken { continue; }
        if v3::dist(d.pos, hit) < 0.045 { s.wear.taken = d.serial; return Some(i); }   // (a bullet mark's centre is clamped in from a face's edge by its radius, at most 3.1 cm)
    }
    None
}

/// what the wear throws goes to the picture alone, as a tick's particles (particles.ts emit's count)
fn throw(g: &mut Game, kind: PKind, pos: V3, dir: V3, count: f64, o: EmitOpts, ours: bool) -> f64 {
    let n = m::min(m::max(0.0, m::floor(count)), MAX_NEW_PER_FRAME);
    if n > 0.0 { crate::render_decals::thrown(g, OutEvent::ParticlesEmit(OutParticlesEmit { kind, pos, dir, count, o, ours })); }
    n
}
/// what the far face throws (through marks), and a little more off the near face as the cell goes: particles only, nothing without a system of them
fn throw_out(g: &mut Game, b: &crate::scene_boxes::Box, surf: Surf, hit_p: V3, dir: V3, n: V3, half: f64, through: bool, s: f64, ours: bool) {
    let (wood, soft) = (surf == Surf::Wood, surf == Surf::Fabric || surf == Surf::Cardboard);
    let chip = if surf == Surf::Drywall { v3::lerp3(b.albedo, PALE, 0.55) } else if wood { [0.66, 0.52, 0.34] } else { v3::lerp3(b.albedo, [0.8, 0.78, 0.72], if soft { 0.4 } else { 0.1 }) };
    let dust = if surf == Surf::Drywall { [0.82, 0.8, 0.76] } else if wood { [0.62, 0.54, 0.42] } else { v3::lerp3(b.albedo, [0.7, 0.68, 0.64], 0.6) };
    let w = crate::wallwear::wear();
    let mut room = w.max_births;
    if through {
        // out of the far face, along the round (kept off the wall plane so nothing is born inside it)
        let out = v3::normalize(v3::add(v3::scale(n, -1.0), v3::scale(dir, 0.6)));
        let at = v3::mad(hit_p, n, -(2.0 * half + 0.03));
        let o = EmitOpts { shape: Some(EmitShape::Puff), speed: 0.35 + 0.5 * s, speed_jit: Some(0.5), lift: Some(0.2), life: 1.1 + 0.6 * s, life_jit: Some(0.3), size: 0.09 + 0.08 * s, size_jit: Some(0.35), pos_jit: Some(0.04 + 0.05 * s), vadd: Some(v3::scale(out, 0.3 + 0.4 * s)), color: Some(dust), ..EmitOpts::default() };
        let puffs = match crate::particle_fx::dust_asked(g, m::min(room, 1.0 + m::round(2.0 * s)), o) { Some((c, o)) => throw(g, PKind::Dust, at, out, c, o, ours), None => 0.0 };
        room = m::max(0.0, room - puffs);
        let chips = m::min(room, if s < w.exit_chips_from { 1.0 } else { m::round(3.0 + 8.0 * s) });
        room -= chips;
        if chips > 0.0 { throw(g, PKind::Debris, at, out, chips, EmitOpts { speed: 1.2 + 2.2 * s, speed_jit: Some(0.7), spread: Some(0.7 + 0.3 * s), life: 14.0, life_jit: Some(0.3), size: if wood { 0.006 } else { 0.007 + 0.004 * s }, size_jit: Some(0.6), aspect: Some(if wood { 3.0 } else { 1.0 }), color: Some(chip), ..EmitOpts::default() }, ours); }
    }
    // near face: a cell most of the way gone sheds a few bigger flakes that fall
    let flakes = m::min(room, if s > 0.45 { m::round(5.0 * s) } else { 0.0 });
    if flakes > 0.0 { throw(g, PKind::Debris, v3::mad(hit_p, n, 0.02), v3::normalize(v3::add(n, [0.0, -0.4, 0.0])), flakes, EmitOpts { speed: 0.9, speed_jit: Some(0.7), spread: Some(1.1), life: 14.0, life_jit: Some(0.3), size: if wood { 0.007 } else { 0.012 }, size_jit: Some(0.6), aspect: Some(if wood { 3.0 } else { 1.4 }), color: Some(chip), ..EmitOpts::default() }, ours); }
}
