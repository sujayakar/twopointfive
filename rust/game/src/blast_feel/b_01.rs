//! THE FEEL OF A WALL CHARGE GOING OFF (src/game/blastFeel.ts): what of it the tick itself decides or writes.
//!
//! TRANSLATED: the charges that stand set remembered for the next frame's blast (`seen`); `wall_report`, which combat's stun burst asks before it plays
//! the can's report (true = this burst is a wall charge's, and its own report is played in the can's place); the report's sounds; the kick written to
//! the seat's camera for the render (`kick_of`), a field of a camera the game holds. All of it under the DRESS row `blastFeel`, as game.ts calls the tick.
//!
//! LEFT TO THE PICTURE'S SIDE (particles and the particles' impulses, which a Rust game has none of): the fireball, the dust ring, the pushes, the
//! ceiling's dust. A host that draws them needs to be told the blast (an out event for `wall_report`: asked of the generator).
//!
//! STAND-IN: THE REPORT IS PLAYED AT ONCE. The TypeScript holds each of the report's sounds back by the time sound takes from the blast to the player's
//! head (at most 0.25 s; the thump a frame and the patter 0.1 s after that) on a queue of closures (`later`). The generated queue row holds no data
//! (`StateQueueItem.run` is a closure's place), so there is nowhere to keep "which sound, how loud" until its time: the sounds go out as the blast's own
//! tick's out events. Belongs: a queue row that says what to run (asked of the generator), or the host's audio taking a delay.
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The kick's strength as this machine last left it: the host's storage. Here only "asked". (src/game/blastFeel.ts:84 loadFeel)
pub fn load_feel(g: &mut crate::game::Game) { g.m.blast_feel.loaded = true; }

/// this view's state, made on first ask (src/game/blastFeel.ts:93 stateOf)
fn state_mut(g: &mut Game) -> &mut State {
    let key = crate::seats::view_key(g);
    g.m.blast_feel.state.get_or_insert_with(key, || State { seen: Vec::new(), queue: Vec::new(), kicks: Vec::new(), serial: 1.0, last: -1.0, list: None })
}

/// This module's own dice: a counter of its own through a hash. (src/game/blastFeel.ts:95 hashU)
pub fn hash_u(n: f64) -> f64 {
    let mut x = (m::imul(js::to_i32(n) ^ 0x51ed270b, 0x85ebca6bu32 as i32)) as u32;
    x ^= x >> 15;
    x = m::imul(x as i32, 0xc2b2ae35u32 as i32) as u32;
    x ^= x >> 13;
    x as f64
}
/// (src/game/blastFeel.ts:96 dice)
pub fn dice(s: &mut crate::blast_feel::State) -> f64 { let n = s.serial; s.serial += 1.0; hash_u(n) / 4294967296.0 }

/// DESTRUCTION.on && FEEL.on && chaosOf(g) > 0 (src/game/blastFeel.ts:98 live)
pub fn live(g: &mut crate::game::Game) -> bool { g.m.blast_feel.feel.on && crate::destruction::chaos_of(g) > 0.0 }
/// (src/game/blastFeel.ts:99 thick)
pub fn thick(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> f64 { let h = g.tab.boxes[w.box_].h; 2.0 * (if w.along_x { h[2] } else { h[0] }) }
/// (src/game/blastFeel.ts:100 near)
pub fn near(a: V3, b: V3, m: f64) -> bool { m::abs(a[0] - b[0]) + m::abs(a[1] - b[1]) + m::abs(a[2] - b[2]) < m }
/// Where a wall charge's borrowed stun burst stands. (src/game/blastFeel.ts:102 burstAt)
pub fn burst_at(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> V3 { let y = 0.5 + crate::effects::bang().height; crate::wallcharge::wall_pt(g, w, 0.0, 0.15, Some(y)) }
/// Is the wall open at the charge (the blast made its hole)? One segment through it at the slab's height. (src/game/blastFeel.ts:104 holed)
pub fn holed(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> bool {
    let (h, th) = (g.k.wallcharge.height, thick(g, w));
    let (a, b) = (crate::wallcharge::wall_pt(g, w, 0.0, 0.12, Some(h)), crate::wallcharge::wall_pt(g, w, 0.0, -(th + 0.12), Some(h)));
    !crate::scene_collision::solid_blocked(g, a, b, None)
}

/// A stage let go now or queued. The queue's rows hold no data (the head of this file): a stage with nothing of the tick's in it is not kept at all.
/// (src/game/blastFeel.ts:105 later)
pub fn later(g: &mut crate::game::Game, pl: PlayerId, secs: f64, run: &mut dyn FnMut(&mut crate::game::Game)) { run(g); }

/// Combat's stun burst, where it plays the can's report: true = this burst is a wall charge's and its own report is played (the can's is not); false =
/// not this module's (the switch off, the DRESS row down, a thrown can, a door charge). (src/game/blastFeel.ts:109 wallReport)
pub fn wall_report(g: &mut crate::game::Game, at: V3) -> bool {
    crate::seats::first_of(g, &mut |v: &mut Game| if wall_report_of(v, at) { Some(true) } else { None }).is_some()   // (whichever seat's charge it is: seats.ts)
}

/// (src/game/blastFeel.ts:110 wallReportOf)
pub fn wall_report_of(g: &mut crate::game::Game, at: V3) -> bool {
    if !live(g) { return false; }
    let key = crate::seats::view_key(g);
    let Some(seen) = g.m.blast_feel.state.get(key).map(|s| s.seen.clone()) else { return false; };
    let set = g.wall_charge.set.clone();
    let mut found: Option<usize> = None;
    for (i, c) in seen.iter().enumerate() {
        if set.contains(c) { continue; }   // (a charge is its row: the id is the TypeScript's object)
        let wall = g.tab.wallcharge_set_wall_charge[*c].wall.clone();
        let p = burst_at(g, &wall);
        if near(p, at, 0.06) { found = Some(i); break; }
    }
    let Some(i) = found else { return false; };
    let wall = g.tab.wallcharge_set_wall_charge[seen[i]].wall.clone();
    state_mut(g).seen.remove(i);
    // (the TypeScript asks whether the wall gave here, for the ways out of it: the fireball's and the ring's, the picture's. The ask is kept: it is a
    //  query of the box world like any other.)
    let _open = holed(g, &wall);
    let pl = g.player;
    report(g, pl, at);
    let t0 = g.time;
    state_mut(g).kicks.push(Kick { t0, at });
    true
}

/// game.ts update, once a frame: the charges that stand set remembered for next frame's blast, the kick written to the camera. A wall charge and every
/// later stage of its blast are the team's doing: the fog's begin / done round it. (src/game/blastFeel.ts:129 blastFeelTick)
pub fn blast_feel_tick(g: &mut crate::game::Game, cam: CameraId) {
    let who = g.ch[g.pl[g.player].ch].id;
    let was = crate::fogofwar::fog_of_war_begin(g, Some(who));
    blast_feel_tick_raw(g, cam);
    crate::fogofwar::fog_of_war_done(g, was);
}

/// (src/game/blastFeel.ts:130 blastFeelTickRaw)
pub fn blast_feel_tick_raw(g: &mut crate::game::Game, cam: CameraId) {
    let key = crate::seats::view_key(g);
    // (`g.wallCharge` is always there in the Rust game)
    if !live(g) {
        if let Some(s) = g.m.blast_feel.state.get_mut(key) { s.seen.clear(); s.queue.clear(); s.kicks.clear(); }
        if g.tab.camera[cam].kick.is_some() { g.tab.camera[cam].kick = None; }
        return;
    }
    let set = g.wall_charge.set.clone();
    if !g.m.blast_feel.state.has(key) && set.is_empty() { if g.tab.camera[cam].kick.is_some() { g.tab.camera[cam].kick = None; } return; }   // (a kick left on the camera by another Game: not this one's to keep)
    if !g.m.blast_feel.loaded { load_feel(g); }
    let time = g.time;
    {
        let s = state_mut(g);
        // (the clock went back, a load: nothing of the old encounter's blast plays into this one. The TypeScript also asks whether the charges' list is a
        //  NEW ARRAY, a restart whose clock runs on; a Rust list has no identity to ask, and a restart here is a game made again, with no state of this module)
        if time < s.last { s.queue.clear(); s.kicks.clear(); s.seen.clear(); }
        s.last = time;
        s.seen = set;
        // (the queue: nothing of the tick's is ever on it, the head of this file)
    }
    let k = kick_of(g, key, cam);
    g.tab.camera[cam].kick = k;
}

/// THE KICK this frame: every live blast's snap away from it, ringing down: metres to add to the look-at for the render (None: none).
/// (src/game/blastFeel.ts:146 kickOf)
pub fn kick_of(g: &mut crate::game::Game, pl: PlayerId, cam: CameraId) -> Option<V3> {
    let f = g.m.blast_feel.feel.clone(); let time = g.time;
    let (target, forward) = { let c = &g.tab.camera[cam]; (c.target, c.forward) };
    let s = g.m.blast_feel.state.get_mut(pl)?;
    if s.kicks.is_empty() { return None; }
    let (mut x, mut y, mut z, mut any) = (0.0, 0.0, 0.0, false);
    let mut i = s.kicks.len();
    while i > 0 {
        i -= 1;
        let k = s.kicks[i]; let age = time - k.t0;
        if age > f.kick_secs || age < 0.0 { s.kicks.remove(i); continue; }
        let (mut dx, mut dz) = (target[0] - k.at[0], target[2] - k.at[2]); let d = m::hyp2(dx, dz);
        if d < 0.5 { dx = forward[0]; dz = forward[2]; }   // (looking straight at it: thrown back along the view)
        let l = js::or(m::hyp2(dx, dz), 1.0); dx /= l; dz /= l;
        let far = m::max(0.0, 1.0 - d / f.kick_reach);
        let a = f.kick_m * f.kick * far * m::sqrt(far) * m::exp(-age / f.kick_tau) * m::min(1.0, (f.kick_secs - age) / 0.08);
        if !(a > 1e-4) { continue; }
        let w = 2.0 * m::PI * f.kick_hz * age; let (a1, a2, a3) = (a * m::cos(w), a * 0.45 * m::sin(1.37 * w), a * 0.3 * m::sin(0.81 * w + 1.0));
        x += dx * a1 - dz * a2; z += dz * a1 + dx * a2; y += a3; any = true;
    }
    if any { Some([x, y, z]) } else { None }
}

/// THE REPORT: crack, thump and patter from the blast, the crack thinner the further it has come; close to it, a second ring in the ears.
/// (src/game/blastFeel.ts:162 report)
pub fn report(g: &mut crate::game::Game, pl: PlayerId, at: V3) {
    let k = g.m.blast_feel.feel.sound;
    let c = g.pl[pl].ch;
    let ear = match g.ch[c].bones.get(crate::character::Bone::Head) { Some(p) => *p, None => v3::add(g.ch[c].pos, [0.0, 1.6, 0.0]) };
    let d = v3::dist(ear, at);
    let _late = m::min(k.late_max, d / k.c);   // (STAND-IN, the head of this file: the report is played at once, not `late` seconds on)
    let dull = 1.0 / (1.0 + (d / k.dull_m) * (d / k.dull_m));
    let carry = 1.0 + k.carry * m::min(1.0, m::max(0.0, (d - k.carry_from) / k.carry_over));
    let (r2, r3) = { let s = state_mut(g); let r2 = 0.96 + 0.06 * dice(s); let r3 = 0.94 + 0.12 * dice(s); (r2, r3) };
    // (the crack passes NO rate: the mixer draws its own)
    crate::out_events::push_wall_feel(g, at);   // (the picture's half of the feel: the fireball, the dust ring, the pushes, the ceiling's dust)
    crate::audio_iface::game_audio_like_play(g, "wallCrack", Some(at), Some(k.crack * (0.25 + 0.75 * dull) * carry), None);
    crate::audio_iface::game_audio_like_play(g, "wallThump", Some(at), Some(k.thump * carry), Some(&crate::audio_iface::PlayOpts::Rate { rate: r2 }));
    crate::audio_iface::game_audio_like_play(g, "wallDebris", Some(at), Some(k.debris * (0.4 + 0.6 * dull) * carry), Some(&crate::audio_iface::PlayOpts::Rate { rate: r3 }));
    if !g.pl[pl].down && d < k.ring_m { crate::audio_iface::game_audio_like_play(g, "earRing", None, Some(k.ring * (1.0 - d / k.ring_m)), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.9 })); }
}

/// BOUNDARY NO-OP (the picture's: a particle record; a Rust game has no particles): (src/game/blastFeel.ts:175 emit)
pub fn emit(g: &mut crate::game::Game, pl: PlayerId, kind: crate::render_particles::PKind, pos: V3, dir: V3, count: f64, o: crate::render_particles::EmitOpts) {}

/// BOUNDARY NO-OP (the picture's: particles): (src/game/blastFeel.ts:181 fireball)
pub fn fireball(g: &mut crate::game::Game, pl: PlayerId, w: &crate::wallcharge::WallSpot, open: bool, n: V3) {}

/// BOUNDARY NO-OP (the picture's: particles): (src/game/blastFeel.ts:196 ringStage)
pub fn ring_stage(g: &mut crate::game::Game, pl: PlayerId, k: f64, faces: &[crate::blast_feel::Face]) {}

/// BOUNDARY NO-OP (the picture's: particles): (src/game/blastFeel.ts:213 ceiling)
pub fn ceiling(g: &mut crate::game::Game, pl: PlayerId, faces: &[crate::blast_feel::Face]) {}
