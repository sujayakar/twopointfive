//! THE FEEL OF A WALL CHARGE GOING OFF, THE PICTURE'S HALF: the fireball in the hole, a low ring of floor dust racing out from the wall on both faces, what lies loose thrown out by three pushes
//! that follow the ring, and dust and crumbs shaken off the ceiling a beat later. Particles only, none of it read by play. Where each goes is asked of the world as it stands, so the game
//! works it out, a stage at its time, and hands the particles over as the tick's (`render_decals thrown`); its queue is kept with the game, so a game put back goes on where it was.
use super::*;
use crate::game::Game;
use crate::m;
use crate::out_events::{OutEvent, OutParticlesEmit, OutParticlesImpulse};
use crate::render_decals::thrown;
use crate::render_particles::{EmitOpts, EmitShape, ImpulseOpts, PKind};
use crate::v3::{self, V3};
use alloc::vec::Vec;

const FRAME: f64 = 1.0 / 60.0;

#[derive(Clone, Copy, Debug)]
pub struct Way { from: V3, d: V3, k: usize }
#[derive(Clone, Debug)]
enum What { Roll(Vec<Way>, bool), Soot(usize, Vec<Way>, bool), Ring(u32, usize), Push(usize, usize), Ceil(V3) }
#[derive(Clone, Debug)]
struct Stage { at: f64, what: What }
/// the stages to come, in the order they were asked for; the faces their blasts have, each ring's sectors stopped by a wall kept on its face; and this half's own counter
#[derive(Clone, Debug, Default)]
pub struct Stages { queue: Vec<Stage>, faces: Vec<Vec<Face>>, serial: f64 }
impl crate::bytes::ToBytes for Stages { fn w(&self, _: &mut crate::bytes::W) {} }

fn stages(g: &mut Game) -> &mut Stages { let s: &mut Stages = g.feel.at(); if s.serial == 0.0 { s.serial = 1.0; } s }
fn draw(g: &mut Game) -> f64 { let s = stages(g); let n = s.serial; s.serial += 1.0; hash_u(n) / 4294967296.0 }
fn seed(g: &mut Game) -> f64 { let s = stages(g); let n = s.serial; s.serial += 1.0; hash_u(m::imul(crate::js::to_i32(n), crate::js::to_i32(2654435761.0)) as f64) }
/// the team's doing, as the blast is
fn ours(g: &Game, seat: &crate::seats::SeatRef) -> bool { let who = g.seat_id(seat); crate::fogofwar::fog_of_war_ours_id(g, Some(who)) }

/// one record, with a seed of this half's own
fn emit(g: &mut Game, kind: PKind, pos: V3, dir: V3, count: f64, o: EmitOpts) {
    let o = EmitOpts { seed: Some(seed(g)), ..o };
    let ours = ours(g, &crate::seats::here(g));
    if kind == PKind::Dust {
        if let Some((n, o)) = crate::particle_fx::dust_asked(g, count, o) { thrown(g, OutEvent::ParticlesEmit(OutParticlesEmit { kind, pos, dir, count: n, o, ours })); }
    } else { thrown(g, OutEvent::ParticlesEmit(OutParticlesEmit { kind, pos, dir, count, o, ours })); }
}
fn later(g: &mut Game, secs: f64, what: What) {
    let at = g.time + secs - FRAME / 2.0;
    stages(g).queue.push(Stage { at, what });
}

/// the fat slow sparks in the hole
fn sparks(g: &mut Game, ways: &[Way], open: bool, count: &[f64], speed: &[f64], life: f64, size: f64, jit: f64) {
    for w in ways {
        let i = if open { w.k } else { 0 };
        emit(g, PKind::Spark, w.from, v3::normalize([w.d[0], 0.15, w.d[2]]), count[i], EmitOpts { shape: Some(EmitShape::Cone), speed: speed[i], speed_jit: Some(0.7), spread: Some(1.25), life, life_jit: Some(0.5), size, size_jit: Some(0.5), pos_jit: Some(jit), ..EmitOpts::default() });
    }
}

/// A WALL CHARGE WENT OFF, as the tick left the wall: the fireball now, and the rest of it to come
pub(in crate::breakage) fn feel_starts(g: &mut Game, wall: &crate::wallcharge::WallSpot) {
    if !live(g) { return; }
    let f = g.k.blast_feel.clone();
    { let s = stages(g); s.serial += 2.0; }   // (the report's two draws of this half's counter, so the seeds after them are the ones they would be)
    let open = holed(g, wall);
    let n: V3 = if wall.along_x { [0.0, 0.0, wall.side] } else { [wall.side, 0.0, 0.0] };
    let th = crate::wallcharge::thick(g, wall);
    // the ways out of the wall: back off the face it was set on, and, only through a wall that gave, into the room
    let mut faces = Vec::new();
    if open { let o = crate::wallcharge::wall_pt(g, wall, 0.0, -(th + 0.35), Some(0.0)); faces.push(Face { o, d: v3::scale(n, -1.0), mask: 0.0 }); }
    { let o = crate::wallcharge::wall_pt(g, wall, 0.0, 0.35, Some(0.0)); faces.push(Face { o, d: n, mask: 0.0 }); }
    // THE FIREBALL
    let h = g.k.wallcharge.height;
    let mut ways = Vec::new();
    if open { let from = crate::wallcharge::wall_pt(g, wall, 0.0, -(th + 0.14), Some(h)); ways.push(Way { from, d: v3::scale(n, -1.0), k: 0 }); }
    { let from = crate::wallcharge::wall_pt(g, wall, 0.0, 0.14, Some(h)); ways.push(Way { from, d: n, k: 1 }); }
    let b = &f.fire;
    sparks(g, &ways, open, &b.core.count, &b.core.speed, b.core.life, b.core.size, 0.2);
    { for w in &ways {
        let i = if open { w.k } else { 0 };
        let c = &b.chunks;
        emit(g, PKind::Debris, w.from, v3::normalize([w.d[0], 0.35, w.d[2]]), c.count[i], EmitOpts { shape: Some(EmitShape::Cone), speed: c.speed[i], speed_jit: Some(0.6), spread: Some(0.95), life: c.life, life_jit: Some(0.3), size: c.size, size_jit: Some(0.6), aspect: Some(1.7), pos_jit: Some(0.25), color: Some(c.color), ..EmitOpts::default() });
    } }
    later(g, b.roll.after * FRAME, What::Roll(ways.clone(), open));
    for i in 0..b.soot.len() { later(g, b.soot[i].after * FRAME, What::Soot(i, ways.clone(), open)); }
    // THE RING, from the frame after the blast's, and THE PUSHES
    let blast = { let s = stages(g); s.faces.push(faces.clone()); s.faces.len() - 1 };
    let r = &f.ring;
    let mut k = 0;
    while (k as f64) < r.stages { later(g, FRAME + k as f64 * r.every, What::Ring(k, blast)); k += 1; }
    for i in 0..f.push.len() { let t = f.push[i][0]; if t < FRAME / 2.0 { push(g, i, blast); } else { later(g, t, What::Push(i, blast)); } }
    // THE CEILING: spots over the room on each face the blast can see, each shaken a beat after the wave gets under it
    let c = &f.ceil;
    for fa in &faces {
        let mut j = 0.0;
        while j < c.spots {
            j += 1.0;
            let a = (draw(g) - 0.5) * 2.0 * 1.45;
            let r = 0.7 + (c.reach - 0.7) * m::sqrt(draw(g));
            let (cs, sn) = (m::cos(a), m::sin(a));
            let (x, z) = (fa.o[0] + (fa.d[0] * cs - fa.d[2] * sn) * r, fa.o[2] + (fa.d[2] * cs + fa.d[0] * sn) * r);
            if crate::scene_collision::solid_blocked(g, [fa.o[0], 1.4, fa.o[2]], [x, 1.4, z], None) { continue; }
            let Some(up) = crate::scene_collision::raycast(g, [x, 2.0, z], [0.0, 1.0, 0.0], 6.0) else { continue };   // (open sky: nothing overhead to shake)
            later(g, c.after + r / c.speed, What::Ceil([x, 2.0 + up.t - 0.06, z]));
        }
    }
}

fn push(g: &mut Game, i: usize, blast: usize) {
    if !crate::destruction::impulses_on(g) { return; }
    let (row, up) = (g.k.blast_feel.push[i], g.k.blast_feel.push_up);
    let faces = stages(g).faces[blast].clone();
    for f in &faces {
        let s = seed(g);
        thrown(g, OutEvent::ParticlesImpulse(OutParticlesImpulse { pos: [f.o[0], 0.25, f.o[2]], radius: row[1], strength: row[2], o: Some(ImpulseOpts { up: Some(up), bias: None, kinds: None, occlude: Some(true), eye: Some([f.o[0], 0.6, f.o[2]]), seed: Some(s) }) }));
    }
}

/// one stage of THE DUST RING: on each face, a puff or three in every sector the wave can still reach (a sector a wall stops stays stopped)
fn ring(g: &mut Game, k: u32, blast: usize) {
    let r = g.k.blast_feel.ring;
    let (at, fr) = (r.r0 + k as f64 * r.step, 1.0 - k as f64 / r.stages);
    let n = stages(g).faces[blast].len();
    for fi in 0..n {
        let mut j = 0.0;
        while j < r.sectors {
            let sector = j as u32;
            j += 1.0;
            let fa = stages(g).faces[blast][fi];
            if crate::js::to_u32(fa.mask) & 1 << sector != 0 { continue; }
            let a = (sector as f64 / (r.sectors - 1.0) - 0.5) * 2.0 * r.arc;
            let (c, s) = (m::cos(a), m::sin(a));
            let d0 = [fa.d[0] * c - fa.d[2] * s, 0.0, fa.d[2] * c + fa.d[0] * s];
            if crate::scene_collision::solid_blocked(g, [fa.o[0], 0.3, fa.o[2]], [fa.o[0] + d0[0] * at, 0.3, fa.o[2] + d0[2] * at], None) {
                let f = &mut stages(g).faces[blast][fi];
                f.mask = (crate::js::to_u32(f.mask) | 1 << sector) as f64;
                continue;
            }
            // (each puff off its sector's line and its stage's radius by up to half a cell: a front, not a lattice of countable patches)
            let a2 = a + (draw(g) - 0.5) * 0.9 * 2.0 * r.arc / (r.sectors - 1.0);
            let r2 = at + (draw(g) - 0.5) * 0.9 * r.step;
            let (c2, s2) = (m::cos(a2), m::sin(a2));
            let dir = [fa.d[0] * c2 - fa.d[2] * s2, 0.0, fa.d[2] * c2 + fa.d[0] * s2];
            let p = [fa.o[0] + dir[0] * r2, r.y, fa.o[2] + dir[2] * r2];
            if crate::scene_collision::solid_blocked(g, [fa.o[0], 0.3, fa.o[2]], [p[0], 0.3, p[2]], None) { continue; }
            emit(g, PKind::Dust, p, dir, if k < 4 { 4.0 } else { 3.0 }, EmitOpts { shape: Some(EmitShape::Cone), speed: r.speed * (0.5 + 0.5 * fr), speed_jit: Some(0.4), spread: Some(0.3), life: r.life, life_jit: Some(0.3), size: r.size * (1.5 - 0.5 * fr), size_jit: Some(0.3), pos_jit: Some(m::min(0.1, r.y * 0.7)), color: Some(r.color), ..EmitOpts::default() });
        }
    }
}

/// THE TICK'S LAST STEP: the stages whose time has come
pub(in crate::breakage) fn feel_stages(g: &mut Game) {
    if g.feel.get::<Stages>().map_or(true, |s| s.queue.is_empty()) { return; }
    if !live(g) { stages(g).queue.clear(); return; }
    let time = g.time;
    let f = g.k.blast_feel.clone();
    let mut i = 0;
    while i < stages(g).queue.len() {
        if stages(g).queue[i].at > time { i += 1; continue; }
        let what = stages(g).queue.remove(i).what;
        match what {
            What::Roll(ways, open) => sparks(g, &ways, open, &f.fire.roll.count, &f.fire.roll.speed, f.fire.roll.life, f.fire.roll.size, 0.28),
            What::Soot(k, ways, open) => {
                let s = &f.fire.soot[k];
                for w in &ways {
                    let n = if open { w.k } else { 0 };
                    emit(g, PKind::Dust, v3::mad(w.from, w.d, 0.15), v3::normalize([w.d[0], 0.5, w.d[2]]), s.count[n], EmitOpts { shape: Some(EmitShape::Puff), speed: s.speed, speed_jit: Some(0.5), lift: Some(0.5), life: s.life, life_jit: Some(0.3), size: s.size, size_jit: Some(0.3), pos_jit: Some(0.22), vadd: Some(v3::scale(w.d, s.speed * 0.6)), color: Some(f.fire.soot_color), ..EmitOpts::default() });
                }
            },
            What::Ring(k, blast) => ring(g, k, blast),
            What::Push(k, blast) => push(g, k, blast),
            What::Ceil(at) => {
                let c = &f.ceil;
                emit(g, PKind::Dust, at, [0.0, -1.0, 0.0], c.puffs, EmitOpts { shape: Some(EmitShape::Puff), speed: 0.9, speed_jit: Some(0.5), lift: Some(-0.6), life: 1.7, life_jit: Some(0.3), size: 0.13, size_jit: Some(0.35), pos_jit: Some(0.12), color: Some(c.color), ..EmitOpts::default() });
                emit(g, PKind::Debris, at, [0.0, -1.0, 0.0], c.crumbs, EmitOpts { shape: Some(EmitShape::Cone), speed: 0.8, speed_jit: Some(0.8), spread: Some(0.9), life: 14.0, life_jit: Some(0.3), size: 0.012, size_jit: Some(0.6), pos_jit: Some(0.1), color: Some(c.color), ..EmitOpts::default() });
            },
        }
    }
    let s = stages(g);
    if s.queue.is_empty() { s.faces.clear(); }
}

/// the level is put back: nothing of the old encounter's blast plays into this one
pub(in crate::breakage) fn feel_cleared(g: &mut Game) { if let Some(s) = g.feel.get::<Stages>() { if !s.queue.is_empty() { let s = stages(g); s.queue.clear(); s.faces.clear(); } } }
