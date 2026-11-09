#![allow(unused_imports, unused_variables)]
pub use ::game_types::sparks::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::vec::Vec;

// SPARKS ARE NOT THE PICTURE'S ALONE (room-clear-four tick 1621: the stun grenade's burst, detonate_stun_raw < spawn_burst < sparks_emit_burst, a stub).
// triage.json had sparks.ts as dress, "unsure": effects.ts:262-266 hangs a smoke trail on each spark of a burst by `tracker(i)`, and the CPU smoke is what the senses
// read; the frame's box list takes a box a live spark (game.ts:1820, no DRESS row). So the whole class is translated: round one's three no-ops (clear, update, boxes)
// would have left every trail standing at the burst point. `tracker` itself is effects' Track::Spark { slot, gen_ } (p_effects_01 track_at): its stub stays.

/// a free slot, else the spark nearest its end   (sparks.ts Sparks.slot)
pub fn sparks_slot(g: &crate::game::Game) -> f64 {
    let n = m::min(js::to_i32(g.sparks.params.max_live) as f64, g.sparks.pool.len() as f64);
    let (mut worst, mut wf) = (0usize, -1.0);
    let mut i = 0usize;
    while (i as f64) < n {
        let s = &g.sparks.pool[i];
        if !s.alive { return i as f64; }
        let fr = s.age / m::max(s.life, 1e-3);
        if fr > wf { wf = fr; worst = i; }
        i += 1;
    }
    worst as f64
}
/// (sparks.ts Sparks.spawn)
pub fn sparks_spawn(g: &mut crate::game::Game, at: V3, vel: V3, life: f64) -> f64 {
    let i = sparks_slot(g);
    let s = &mut g.sparks.pool[i as usize];
    s.pos = v3::copy(at); s.vel = vel; s.age = 0.0; s.life = life; s.alive = true; s.gen_ += 1.0;
    i
}
/// deterministic per-spark hash in [0,1)   (sparks.ts Sparks.h)
pub fn sparks_h(i: f64, k: f64, seed: f64) -> f64 {
    let t = m::sin((i + 1.0) * 12.9898 + k * 78.233 + seed * 53.17) * 43758.5453;
    t - m::floor(t)
}
/// a ground burst: the upward hemisphere narrowed by `spread`, turned toward vertical by `lift`   (sparks.ts Sparks.emitBurst)
pub fn sparks_emit_burst(g: &mut crate::game::Game, at: V3, o: crate::sparks::SparksEmitBurstO, seed: f64) -> Vec<f64> {
    let mut out = Vec::new();
    let mut i = 0.0;
    while i < o.count {
        let theta = sparks_h(i, 1.0, seed) * core::f64::consts::PI * 2.0;
        let mut cy = m::pow(sparks_h(i, 2.0, seed), 1.0 + o.spread * 3.0) * 0.9 + 0.05;
        cy = cy + (1.0 - cy) * o.lift;
        let r = m::sqrt(m::max(0.0, 1.0 - cy * cy));
        let sp = o.speed * (0.45 + 0.55 * sparks_h(i, 3.0, seed));
        let vel: V3 = [m::cos(theta) * r * sp, cy * sp, m::sin(theta) * r * sp];
        let life = o.life * (0.5 + 0.7 * sparks_h(i, 4.0, seed));
        out.push(sparks_spawn(g, at, vel, life));
        i += 1.0;
    }
    out
}
/// (sparks.ts Sparks.clear)
pub fn sparks_clear(g: &mut crate::game::Game) {
    for s in g.sparks.pool.iter_mut() { s.alive = false; }
    g.sparks.live = 0.0;
}
/// the sparks fly, meet the walls and the floor, and die   (sparks.ts Sparks.update; the game always hands its own collision)
pub fn sparks_update(g: &mut crate::game::Game, dt: f64) {
    let p = g.sparks.params;
    let decay = m::exp(-p.drag * dt);
    let mut live = 0.0;
    for i in 0..g.sparks.pool.len() {
        let mut s = g.sparks.pool[i];
        if !s.alive { continue; }
        s.age += dt;
        if s.age >= s.life { s.alive = false; g.sparks.pool[i] = s; continue; }
        s.vel[1] -= p.gravity * dt; s.vel[0] *= decay; s.vel[1] *= decay; s.vel[2] *= decay;
        let mv = v3::scale(s.vel, dt);
        let len = v3::len(mv);
        let hit = if p.collide && len > 0.02 { crate::scene_collision::raycast(g, s.pos, v3::scale(mv, 1.0 / len), len + 0.01) } else { None };
        if let Some(hit) = hit {
            s.pos = v3::mad(s.pos, mv, m::max(0.0, hit.t - 0.006) / len);
            let vn = v3::dot(s.vel, hit.n);
            if vn < 0.0 { let v_n = v3::scale(hit.n, vn); s.vel = v3::add(v3::scale(v3::sub(s.vel, v_n), 0.7), v3::scale(v_n, -0.25)); }
        } else { s.pos = v3::add(s.pos, mv); }
        if s.pos[1] < 0.01 { s.pos[1] = 0.01; if s.vel[1] < 0.0 { s.vel[1] *= -0.25; s.vel[0] *= 0.7; s.vel[2] *= 0.7; } }
        g.sparks.pool[i] = s;
        live += 1.0;
    }
    g.sparks.live = live;
}
/// one thin emissive box a live spark, along its travel   (sparks.ts Sparks.boxes)
pub fn sparks_boxes(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>) {
    let p = g.sparks.params;
    let t = p.thickness * 0.5;
    let bf = crate::scene_boxes::box_flag();
    for i in 0..g.sparks.pool.len() {
        let s = g.sparks.pool[i];
        if !s.alive { continue; }
        let u = s.age / m::max(s.life, 1e-3);
        let f = if u < p.hold { 1.0 } else { m::max(0.0, 1.0 - (u - p.hold) / m::max(1e-3, 1.0 - p.hold)) };
        let e = f * f * p.brightness * p.emissive;
        if e < 0.02 * p.emissive { continue; }
        let emissive: V3 = [e, e * (0.35 + 0.5 * f), e * (0.08 + 0.25 * f * f)];
        let mut tail = v3::mad(s.pos, s.vel, -p.shutter);
        if tail[1] < 0.005 { tail = [tail[0], 0.005, tail[2]]; }
        let d = v3::sub(s.pos, tail);
        let l = v3::len(d);
        let mid = v3::lerp3(s.pos, tail, 0.5);
        let dir: V3 = if l > 1e-5 { [d[0] / l, d[1] / l, d[2] / l] } else { [0.0, 1.0, 0.0] };
        let b = crate::scene_boxes::make_box_frame(g, &crate::scene_boxes::MakeBoxP { c: mid, h: [t, t, m::max(p.min_streak * 0.5, l * 0.5)], yaw: Some(v3::yaw_of(d)), rot: Some(v3::quat::from_to([0.0, 0.0, 1.0], dir)),
            albedo: Some([0.04, 0.03, 0.02]), emissive: Some(emissive), flags: Some((js::to_i32(bf.dynamic) | js::to_i32(bf.no_shadow)) as f64), ..Default::default() });
        out.push(b.into());
    }
}
