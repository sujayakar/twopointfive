#![allow(unused_imports, unused_variables)]
//! THE BRING-UP'S OWN PART of blast_look: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)

pub use ::game_types::blast_look::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{vec, vec::Vec};

fn charge_look() -> CHARGELOOK {
    CHARGELOOK {
        frame_half: 0.3, frame_lo: 0.5, frame_hi: 1.6, strip: 0.045, proud: 0.012,
        strip_albedo: [0.62, 0.46, 0.1],
        lamp: 0.04, blink_every: 1.1, blink_on: 0.35, lamp_on: [70.0, 5.0, 2.5], lamp_off: [0.008, 0.0008, 0.0005],
        lamp_spill: 0.0025, led_spill: 0.02,
        tape: 0.07, wire_tape: 0.04,
        wire: 0.018, wire_px: 1.4, run: [0.4, 0.2, 0.6], short: 0.36, wire_albedo: [0.85, 0.36, 0.05],
        reel: [0.05, 0.035, 0.08], reel_albedo: [0.1, 0.11, 0.08],
        led: 0.013, led_on: [7.0, 3.6, 0.4],
    }
}
const WALL_FLASH_COLOR: V3 = [1.0, 0.62, 0.25];

/// DESTRUCTION.on && BLAST.on && chaosOf(g) > 0
pub fn live(g: &mut crate::game::Game) -> bool { blast().on && crate::destruction::chaos_of(g) > 0.0 }

pub fn near(a: V3, b: V3, m: f64) -> bool { m::abs(a[0] - b[0]) + m::abs(a[1] - b[1]) + m::abs(a[2] - b[2]) < m }

/// where a wall charge's borrowed stun burst stands
pub fn burst_at(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> V3 {
    let y = 0.5 + crate::effects::bang().height;
    crate::wallcharge::wall_pt(g, w, 0.0, 0.15, Some(y))
}

/// game.ts update, once a frame after the frame's other boxes: the set charges drawn, a blown one's flash warmed.
/// NOT a boundary no-op, though it is "look only": a set charge's thirteen boxes are on the FRAME'S LIST (which the fog of war, the listing of the box
/// world and every table numbered by reach count), and the flash it warms is a light of the engine's list, which the light estimate reads.
pub fn blast_look_tick(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>) {
    let who = g.ch[g.pl[g.player].ch].id;
    let was = crate::fogofwar::fog_of_war_begin(g, Some(who));
    blast_look_tick_raw(g, out);
    crate::fogofwar::fog_of_war_done(g, was);
}

pub fn blast_look_tick_raw(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>) {
    let key = crate::seats::view_key(g, &crate::seats::here(g));
    // (`g.wallCharge` is always there in the Rust game)
    if !live(g) {
        if let Some(s) = g.m.blast_look.state.get_mut(key) { s.seen.clear();  }
        return;
    }
    let set = g.wall_charge.set.clone();
    if !g.m.blast_look.state.has(key) {
        if set.is_empty() { return; }
        g.m.blast_look.state.set(key, State { seen: Vec::new(), serial: 1.0, first: false, last: -1.0 });
    }
    let time = g.time;
    {
        let st = g.m.blast_look.state.get_mut(key).unwrap();
        if time < st.last {  }   // (the clock went back: a load / a restart)
        st.last = time;
    }
    // charges gone from the list since last frame: BLOWN (the flash detonateStun lit for it is burning at its stun point) or given back (no flash)
    let mut i = g.m.blast_look.state.get(key).unwrap().seen.len();
    while i > 0 {
        i -= 1;
        let s = g.m.blast_look.state.get(key).unwrap().seen[i];
        if set.contains(&s.c) { continue; }   // (a charge is its row: the id is the TypeScript's object)
        g.m.blast_look.state.get_mut(key).unwrap().seen.remove(i);
        let wall = g.tab.wallcharge_set_wall_charge[s.c].wall.clone();
        let l = match flash_of(g, &wall) { Some(l) => l, None => continue };
        let c = g.tab.light[l].color;
        g.tab.light[l].color = warm_of(g, c);
        // (BLAST.glow is a constant false: the afterglow's light is never queued. The queue's other rows are a blast's later particle records, pushed by
        //  wallBlast from the particle lane, which this game does not run: nothing is ever on the queue here.)
    }
    for c in set {
        let found = g.m.blast_look.state.get(key).unwrap().seen.iter().copied().find(|x| x.c == c);
        let s = match found {
            Some(s) => s,
            None => {
                let (t, wall) = { let r = &g.tab.wallcharge_set_wall_charge[c]; (r.t, r.wall.clone()) };
                let s = Seen { c, t0: t, run: run_of(g, &wall) };
                g.m.blast_look.state.get_mut(key).unwrap().seen.push(s);
                s
            }
        };
        charge_boxes(g, s, out);
    }
}

/// how much of the firing line's run fits: the whole of it over clear floor, the short one where something stands in the way
pub fn run_of(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> f64 {
    let k = charge_look();
    let [r0, jog, r1] = k.run;
    let a = crate::wallcharge::wall_pt(g, w, 0.035, 0.05, Some(0.12));
    let b = crate::wallcharge::wall_pt(g, w, 0.035 + jog, r0 + r1 + 2.0 * k.reel[2] + 0.05, Some(0.12));
    if crate::scene_collision::solid_blocked(g, a, b, None) { k.short } else { 1.0 }
}

/// the frame of strip, the blinking lamp, the firing line and its reel: thirteen boxes of the frame
pub fn charge_boxes(g: &mut crate::game::Game, s: crate::blast_look::Seen, out: &mut Vec<AnyBoxId>) {
    let k = charge_look();
    let w = g.tab.wallcharge_set_wall_charge[s.c].wall.clone();
    let h = g.k.wallcharge.height;
    let bf = crate::scene_boxes::box_flag();
    let flags = (js::to_i32(bf.dynamic) | js::to_i32(bf.no_shadow)) as f64;
    let yaw = if w.along_x { 0.0 } else { core::f64::consts::PI / 2.0 };
    fn bx(g: &mut Game, out: &mut Vec<AnyBoxId>, yaw: f64, flags: f64, c: V3, h: V3, albedo: V3, emissive: V3) {
        let b = crate::scene_boxes::make_box_frame(g, &crate::scene_boxes::MakeBoxP { c, h, yaw: Some(yaw), albedo: Some(albedo), emissive: Some(emissive), flags: Some(flags), ..Default::default() });
        out.push(b.into());
    }
    /// a piece of the firing line: never thinner than is seen
    fn wire(g: &mut Game, out: &mut Vec<AnyBoxId>, k: &CHARGELOOK, yaw: f64, flags: f64, c: V3, h: V3, emissive: V3) {
        bx(g, out, yaw, flags, c, h, k.wire_albedo, emissive);
        crate::thinline::thin_note(g, out[out.len() - 1], k.wire / 2.0, k.wire_px / 2.0, emissive[0], emissive[1], emissive[2]);
    }
    let spill = |albedo: V3, lamp: V3, kk: f64| -> V3 { [albedo[0] * lamp[0] * kk, albedo[1] * lamp[1] * kk, albedo[2] * lamp[2] * kk] };
    // the lamp on the slab's head: dark, and lit for blinkOn of every blinkEvery since it was set
    let lit = ((g.time - s.t0) % k.blink_every + k.blink_every) % k.blink_every < k.blink_on;
    let glow = |albedo: V3, tape: f64, lamp: f64| -> V3 { let e = v3::scale(albedo, tape); if lit { v3::add(e, spill(albedo, k.lamp_on, lamp)) } else { e } };
    let red = glow(k.strip_albedo, k.tape, k.lamp_spill);
    let line = v3::scale(k.wire_albedo, k.wire_tape);
    // the frame (half extents: along the wall, up, off the face: the slab's own convention)
    let ws = k.strip / 2.0;
    let mid = (k.frame_lo + k.frame_hi) / 2.0;
    let tall = (k.frame_hi - k.frame_lo) / 2.0;
    let wp = |g: &mut Game, du: f64, off: f64, y: f64| crate::wallcharge::wall_pt(g, &w, du, off, Some(y));
    for side in [-1.0, 1.0] { let c = wp(g, side * k.frame_half, k.proud, mid); bx(g, out, yaw, flags, c, [ws, tall + ws, k.proud], k.strip_albedo, red); }
    for y in [k.frame_lo, k.frame_hi] { let c = wp(g, 0.0, k.proud, y); bx(g, out, yaw, flags, c, [k.frame_half - ws, ws, k.proud], k.strip_albedo, red); }
    let c = wp(g, 0.0, 0.05, h + 0.075);
    bx(g, out, yaw, flags, c, [k.lamp, k.lamp, 0.012], [0.5, 0.06, 0.04], if lit { k.lamp_on } else { k.lamp_off });
    // the firing line: down the wall from the slab, out across the floor with one jog, to the reel and its lamp
    let ww = k.wire / 2.0;
    let drop = h - 0.11;
    let r0 = k.run[0] * s.run;
    let jog = k.run[1] * s.run;
    let r1 = k.run[2] * s.run;
    let a0 = 0.035;
    let amber = spill(k.wire_albedo, k.led_on, k.led_spill);
    let c = wp(g, a0, 0.008, drop / 2.0); wire(g, out, &k, yaw, flags, c, [ww, drop / 2.0, 0.008], glow(k.wire_albedo, k.wire_tape, k.lamp_spill / 2.0));
    let c = wp(g, a0, r0 / 2.0, 0.008); wire(g, out, &k, yaw, flags, c, [ww, 0.008, r0 / 2.0], line);
    let c = wp(g, a0 + jog / 2.0, r0, 0.008); wire(g, out, &k, yaw, flags, c, [jog / 2.0 + ww, 0.008, ww], line);
    let c = wp(g, a0 + jog, r0 + r1 / 2.0, 0.008); wire(g, out, &k, yaw, flags, c, [ww, 0.008, r1 / 2.0], v3::add(line, amber));
    let c = wp(g, a0 + jog, r0 + r1 + k.reel[2], k.reel[1]); bx(g, out, yaw, flags, c, k.reel, k.reel_albedo, spill(k.reel_albedo, k.led_on, k.led_spill));
    let c = wp(g, a0 + jog, r0 + r1 + k.reel[2], 2.0 * k.reel[1] + k.led); bx(g, out, yaw, flags, c, [k.led, k.led, k.led], [0.4, 0.3, 0.1], k.led_on);
}

/// the stun charge's flash a wall charge borrowed (detonateStun lit it this frame where spawnBurst put it); None: none burning there
pub fn flash_of(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> Option<LightId> {
    let b = crate::effects::bang();
    let mut at = burst_at(g, w);
    at[1] += b.light_lift;
    let n = g.engine.lights.lights.len();
    for i in (0..n).rev() {
        let id = g.engine.lights.lights[i];
        let l = &g.tab.light[id];
        if l.name == "flash" && l.ttl == b.light_duration && near(l.pos, at, 0.06) { return Some(id); }
    }
    None
}

/// the light meter's own sum, in its own order
pub fn lum(r: f64, g_2: f64, b: f64) -> f64 { 0.2126 * r + 0.7152 * g_2 + 0.0722 * b }

/// one representable number up or down (x > 0)
pub fn nudge(_g: &crate::game::Game, x: f64, up: bool) -> f64 { let bits = x.to_bits(); f64::from_bits(if up { bits.wrapping_add(1) } else { bits.wrapping_sub(1) }) }

/// WALL_FLASH's hue at `c`'s luminance, the meter's sum kept TO THE BIT: red and blue scaled, green solved for, then green (failing that, blue) walked
/// one representable number at a time until the sum lands; a colour it cannot land stays as it was.
pub fn warm_of(g: &crate::game::Game, c_2: V3) -> V3 {
    let t = WALL_FLASH_COLOR;
    let l0 = lum(c_2[0], c_2[1], c_2[2]);
    let k = l0 / lum(t[0], t[1], t[2]);
    let r = t[0] * k;
    let mut b = t[2] * k;
    let mut gr = (l0 - 0.2126 * r - 0.0722 * b) / 0.7152;
    if !(l0 > 0.0 && gr > 0.0 && b > 0.0) { return c_2; }
    let mut i = 0;
    while i < 48 && lum(r, gr, b) != l0 { gr = nudge(g, gr, lum(r, gr, b) < l0); i += 1; }
    let mut i = 0;
    while i < 48 && lum(r, gr, b) != l0 { b = nudge(g, b, lum(r, gr, b) < l0); i += 1; }
    if lum(r, gr, b) == l0 { [r, gr, b] } else { c_2 }
}

// ---------------------------------------------------------------- THE FRAGMENTS OF A BURST
// Particle records, computed from the game's state (which charge blew, how a can lies, where a door's leaf went) and handed to the host's pool as out
// events (`render_particles::particles_emit`). Reached only where the host has a pool (particleFx `gpu_fx`): `bang_fx` asks `blast_sparks` first.

/// this view's state, made on first ask
fn state_mut(g: &mut Game) -> &mut State {
    let key = crate::seats::view_key(g, &crate::seats::here(g));
    g.m.blast_look.state.get_or_insert_with(key, || State { seen: Vec::new(), serial: 1.0, first: false, last: -1.0 })
}

/// this module's own dice: a counter of its own through a hash
pub fn hash_u(n: f64) -> f64 {
    let mut x = m::imul(js::to_i32(n) ^ (0x9e3779b9u32 as i32), 0x85ebca6bu32 as i32) as u32;
    x ^= x >> 15;
    x = m::imul(x as i32, 0xc2b2ae35u32 as i32) as u32;
    x ^= x >> 13;
    x as f64
}

pub fn thick(g: &crate::game::Game, w: &crate::wallcharge::WallSpot) -> f64 { let h = g.tab.boxes[w.box_].h; 2.0 * (if w.along_x { h[2] } else { h[0] }) }

/// is the wall open at the charge (the blast made its hole)? one segment through it at the slab's height
pub fn holed(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> bool {
    let (h, th) = (g.k.wallcharge.height, thick(g, w));
    let (a, b) = (crate::wallcharge::wall_pt(g, w, 0.0, 0.12, Some(h)), crate::wallcharge::wall_pt(g, w, 0.0, -(th + 0.12), Some(h)));
    !crate::scene_collision::solid_blocked(g, a, b, None)
}

/// the authored stun sparks as they stand, over what they were when the tables were set: count, speed, life, size
pub fn scale(g: &mut crate::game::Game) -> crate::blast_look::BurstOfK {
    let (r, pfx, sp) = (ref_(), g.m.particle_fx.pfx, crate::effects::sparks());
    BurstOfK { n: pfx.bang_sparks / r.sparks, v: sp.speed / r.speed, l: sp.life / r.life, s: pfx.bang_spark_size / r.size }
}

/// the 'flash' item a burst stands over (`born`: the one born this very frame, else an older one)
pub fn flash_under(g: &mut crate::game::Game, at: V3, born: bool) -> bool {
    use crate::throwables as t;
    let h = crate::effects::bang().height;
    let mut i = 0;
    while i < g.items.items.len() {
        let it = g.items.items[i];
        i += 1;
        if t::item_kind(g, it) != t::ItemKind::Flash || (t::item_age(g, it) == 0.0) != born { continue; }
        let p = t::item_pos(g, it);
        if near([p[0], p[1] + h, p[2]], at, 0.02) { return true; }
    }
    false
}

/// is this burst a thrown stun can's (its item is older than this frame)?
pub fn can_at(g: &mut crate::game::Game, at: V3) -> bool { flash_under(g, at, false) }

/// Is this burst a door charge's (a 'flash' item born this frame, 0.2 m off the door's frame line and 0.7 of its width along it)? The way the blast goes,
/// where the room's share starts (ahead of the leaf the charge threw in), where the rest starts. None: not one, or a leaf flung in on its hinges.
pub fn door_charge_at(g: &mut crate::game::Game, at: V3, dt: f64) -> Option<crate::blast_look::DoorChargeAt> {
    if !flash_under(g, at, true) { return None; }
    let k = door_sparks();
    let mut i = 0;
    while i < g.doors.list.len() {
        let d = g.doors.list[i];
        i += 1;
        let w = g.dr[d].def.width;
        let along = crate::doortools::along_of(g, d, at);
        let off = crate::doortools::off_of(g, d, at, 1.0);
        if along < w * 0.55 || along > w * 0.85 || m::abs(off) < 0.16 || m::abs(off) > 0.26 { continue; }
        let sg = m::sign(off);
        let th = g.dr[d].def.closed_dir;
        let into: V3 = [-m::sin(th) * sg, 0.0, m::cos(th) * sg];
        let slab = if crate::doors::door_off(g, d) { crate::doors::door_slab(g, d) } else { None };
        let Some(sl) = slab else {
            let still = m::abs(crate::doors::door_angle(g, d)) < 0.05 && m::abs(crate::doors::door_vel(g, d)) < 0.01;
            return if still { Some(DoorChargeAt { into, from: at, rest: at }) } else { None };
        };
        let gone = (sl.p[0] - at[0]) * into[0] + (sl.p[2] - at[2]) * into[2];
        let fly = m::max(0.0, sl.v[0] * into[0] + sl.v[2] * into[2]);
        let th_leaf = g.dr[d].thick;
        return Some(DoorChargeAt {
            into,
            from: v3::mad(at, into, m::max(0.0, gone + th_leaf / 2.0 + k.ahead + fly * dt * k.lead)),
            rest: if fly > 0.0 { v3::mad(at, into, -2.0 * m::abs(off)) } else { at },
        });
    }
    None
}

/// particleFx's bangFx, in place of its one spark record: true = the fragments were thrown here (a wall charge's two ways out of the wall, a door
/// charge's the way the blast goes, a thrown can's along its axis); false = not this module's, and the caller throws its own.
pub fn blast_sparks(g: &mut crate::game::Game, at: V3) -> bool {
    crate::seats::first_of(g, &mut |v: &mut crate::game::Game| if blast_sparks_of(v, at) { Some(true) } else { None }).is_some()   // (whichever seat's charge it is)
}
fn blast_sparks_of(g: &mut crate::game::Game, at: V3) -> bool {
    if !live(g) { return false; }
    let key = crate::seats::view_key(g, &crate::seats::here(g));
    let seen = state_mut(g).seen.clone();
    let set = g.wall_charge.set.clone();
    let mut wall: Option<crate::wallcharge::WallSpot> = None;
    for x in seen.iter() {
        if set.contains(&x.c) { continue; }
        let w = g.tab.wallcharge_set_wall_charge[x.c].wall.clone();
        let p = burst_at(g, &w);
        if near(p, at, 0.06) { wall = Some(w); break; }
    }
    let b = blast();
    // STAND-IN: the frame step doorChargeAt leads the leaf by is the host's last (`engine.lastDt`), which is not on the bridge's engine list: the
    // fixed step's 1/60 stands for it. Belongs in the bridge's host half.
    let door = if wall.is_none() && b.door { door_charge_at(g, at, m::min(0.1, 1.0 / 60.0)) } else { None };
    if wall.is_none() && door.is_none() && !(b.stun && can_at(g, at)) { return false; }
    state_mut(g).first = true;
    if let Some(w) = wall { wall_blast(g, key, &w); }
    else if let Some(d) = door { door_blast(g, key, DoorBlastD { into: d.into, from: d.from, rest: d.rest }); }
    else { stun_sparks(g, key, at); }
    state_mut(g).first = false;
    true
}

/// A door charge's fragments: a flat jet and a low skitter into the room, a share back at the squad's wall, then what is left of the stun's fountain
/// and its embers.
pub fn door_blast(g: &mut crate::game::Game, pl: PlayerId, d: crate::blast_look::DoorBlastD) {
    use crate::render_particles::{EmitOpts, EmitShape, PKind};
    let (kk, k, n) = (door_sparks(), scale(g), d.into);
    // (count, speed, jit, cone, tilt, life, lifeJit, size, sizeJit), which way, from where
    let fans: [((f64, f64, f64, f64, f64, f64, f64, f64, f64), f64, V3); 3] = [
        ((kk.jet.count, kk.jet.speed, kk.jet.jit, kk.jet.cone, kk.jet.tilt, kk.jet.life, kk.jet.life_jit, kk.jet.size, kk.jet.size_jit), 1.0, d.from),
        ((kk.skitter.count, kk.skitter.speed, kk.skitter.jit, kk.skitter.cone, kk.skitter.tilt, kk.skitter.life, kk.skitter.life_jit, kk.skitter.size, kk.skitter.size_jit), 1.0, d.from),
        ((kk.back.count, kk.back.speed, kk.back.jit, kk.back.cone, kk.back.tilt, kk.back.life, kk.back.life_jit, kk.back.size, kk.back.size_jit), -1.0, d.rest),
    ];
    for (f, way, from) in fans {
        emit(g, pl, PKind::Spark, from, v3::normalize([n[0] * way, f.4, n[2] * way]), m::round(f.0 * k.n), EmitOpts {
            shape: Some(EmitShape::Cone), speed: f.1 * k.v, speed_jit: Some(f.2), spread: Some(f.3), life: f.5 * k.l, life_jit: Some(f.6), size: f.7 * k.s, size_jit: Some(f.8), ..EmitOpts::default()
        });
    }
    let st = stun_sparks_table();
    burst_of(g, pl, d.rest, STUNSPARKSFountain { count: kk.fountain, ..st.fountain }, k);
    burst_of(g, pl, d.rest, embers_as_fountain(&st.embers), k);
}

/// the stun's embers as one all-round record's numbers (the two tables hold the same nine)
fn embers_as_fountain(e: &crate::blast_look::STUNSPARKSEmbers) -> crate::blast_look::STUNSPARKSFountain {
    STUNSPARKSFountain { count: e.count, speed: e.speed, jit: e.jit, spread: e.spread, lift: e.lift, life: e.life, life_jit: e.life_jit, size: e.size, size_jit: e.size_jit }
}

/// one all-round record of the stun's (its fountain, its embers)
pub fn burst_of(g: &mut crate::game::Game, pl: PlayerId, at: V3, r: crate::blast_look::STUNSPARKSFountain, k: crate::blast_look::BurstOfK) {
    use crate::render_particles::{EmitOpts, EmitShape, PKind};
    emit(g, pl, PKind::Spark, at, [0.0, 1.0, 0.0], m::round(r.count * k.n), EmitOpts {
        shape: Some(EmitShape::Burst), speed: r.speed * k.v, speed_jit: Some(r.jit), spread: Some(r.spread), lift: Some(r.lift), life: r.life * k.l, life_jit: Some(r.life_jit), size: r.size * k.s, size_jit: Some(r.size_jit), ..EmitOpts::default()
    });
}

/// One record. The first of a burst brings no seed: the pool deals it one off its own serial, the one step the fountain's record took. Every later one
/// brings a seed off this module's counter. `first` goes down once a record was taken (the pool's answer: render_particles `particles_emit`).
pub fn emit(g: &mut crate::game::Game, pl: PlayerId, kind: crate::render_particles::PKind, pos: V3, dir: V3, count: f64, o: crate::render_particles::EmitOpts) {
    let mut o = o;
    let first = g.m.blast_look.state.get(pl).map_or(false, |s| s.first);
    if !first {
        if let Some(s) = g.m.blast_look.state.get_mut(pl) { let n = s.serial; s.serial += 1.0; o.seed = Some(hash_u(n * 2654435761.0)); }
    }
    let got = if kind == crate::render_particles::PKind::Dust { crate::particle_fx::emit_dust(g, pos, dir, count, o) } else { crate::render_particles::particles_emit(g, kind, pos, dir, count, o) };
    if got > 0.0 { if let Some(s) = g.m.blast_look.state.get_mut(pl) { s.first = false; } }
}

/// A wall charge's blast, out of the wall's two faces (into the room only through a wall that gave): the jet, the core, the wall's dust, then the
/// shower, the wall's chips and the embers.
/// STAND-IN: THE THREE LATER FANS GO OUT WITH THE FIRST. The TypeScript holds the shower and the chips back a frame and the embers two, on a queue of
/// closures. There is no such queue here. They are thrown at once, in the order the queue would have run them (so each record's seed is the one it would have
/// had): one or two frames early. Belongs: a queue row that says which fan, of which wall, and when.
pub fn wall_blast(g: &mut crate::game::Game, pl: PlayerId, w: &crate::wallcharge::WallSpot) {
    use crate::render_particles::{EmitOpts, EmitShape, PKind};
    let (b, h, open, k) = (wall_blast_table(), g.k.wallcharge.height, holed(g, w), scale(g));
    let one = BurstOfK { n: 1.0, v: 1.0, l: 1.0, s: 1.0 };
    let n: V3 = if w.along_x { [0.0, 0.0, w.side] } else { [w.side, 0.0, 0.0] };
    // the two ways out: [into the room, only through a wall that gave; back off the face it was set on]
    let mut ways: Vec<(V3, V3, usize)> = vec![(crate::wallcharge::wall_pt(g, w, 0.0, 0.1, Some(h)), n, 1)];
    if open { let t = thick(g, w); ways.insert(0, (crate::wallcharge::wall_pt(g, w, 0.0, -(t + 0.1), Some(h)), v3::scale(n, -1.0), 0)); }
    let pale = v3::lerp3(g.tab.boxes[w.box_].albedo, [0.86, 0.85, 0.82], 0.45);
    struct F<'a> { count: &'a [f64], speed: &'a [f64], jit: f64, cone: &'a [f64], tilt: f64, life: f64, life_jit: f64, size: f64, size_jit: f64, aspect: Option<f64> }
    let mut throw = |g: &mut Game, kind: PKind, f: F, color: Option<V3>| {
        let kk = if kind == PKind::Spark { k } else { one };
        for way in ways.iter() {
            let i = if open { way.2 } else { 0 };   // (a wall that held: everything it had comes back off the near face)
            emit(g, pl, kind, way.0, v3::normalize([way.1[0], f.tilt, way.1[2]]), m::round(f.count[i] * kk.n), EmitOpts {
                shape: Some(EmitShape::Cone), speed: f.speed[i] * kk.v, speed_jit: Some(f.jit), spread: Some(f.cone[i]), life: f.life * kk.l, life_jit: Some(f.life_jit), size: f.size * kk.s, size_jit: Some(f.size_jit),
                pos_jit: Some(0.06), color, aspect: f.aspect, ..EmitOpts::default()
            });
        }
    };
    throw(g, PKind::Spark, F { count: &b.jet.count, speed: &b.jet.speed, jit: b.jet.jit, cone: &b.jet.cone, tilt: b.jet.tilt, life: b.jet.life, life_jit: b.jet.life_jit, size: b.jet.size, size_jit: b.jet.size_jit, aspect: None }, None);
    throw(g, PKind::Spark, F { count: &b.core.count, speed: &b.core.speed, jit: b.core.jit, cone: &b.core.cone, tilt: b.core.tilt, life: b.core.life, life_jit: b.core.life_jit, size: b.core.size, size_jit: b.core.size_jit, aspect: None }, None);
    throw(g, PKind::Dust, F { count: &b.dust.count, speed: &b.dust.speed, jit: b.dust.jit, cone: &b.dust.cone, tilt: b.dust.tilt, life: b.dust.life, life_jit: b.dust.life_jit, size: b.dust.size, size_jit: b.dust.size_jit, aspect: None }, Some(v3::scale(pale, 0.85)));
    // (the queue's order: the shower and the chips a frame on, the embers two)
    throw(g, PKind::Spark, F { count: &b.shower.count, speed: &b.shower.speed, jit: b.shower.jit, cone: &b.shower.cone, tilt: b.shower.tilt, life: b.shower.life, life_jit: b.shower.life_jit, size: b.shower.size, size_jit: b.shower.size_jit, aspect: None }, None);
    throw(g, PKind::Debris, F { count: &b.chips.count, speed: &b.chips.speed, jit: b.chips.jit, cone: &b.chips.cone, tilt: b.chips.tilt, life: b.chips.life, life_jit: b.chips.life_jit, size: b.chips.size, size_jit: b.chips.size_jit, aspect: Some(b.chips.aspect) }, Some(pale));
    throw(g, PKind::Spark, F { count: &b.embers.count, speed: &b.embers.speed, jit: b.embers.jit, cone: &b.embers.cone, tilt: b.embers.tilt, life: b.embers.life, life_jit: b.embers.life_jit, size: b.embers.size, size_jit: b.embers.size_jit, aspect: None }, None);
}

/// A thrown stun can's fragments: two unequal lobes along the can's own axis (effects.ts spawnBurst's draws for this burst's seed: the jets of smoke
/// go the same two ways), a low fan all round, a small fountain, embers.
pub fn stun_sparks(g: &mut crate::game::Game, pl: PlayerId, at: V3) {
    use crate::render_particles::{EmitOpts, EmitShape, PKind};
    let (kk, b, k) = (stun_sparks_table(), crate::effects::bang(), scale(g));
    let seed = g.fx_seed - 1.0;
    let rnd = |i: f64| { let t = m::sin(seed * 37.31 + i * 91.7) * 43758.5453; t - m::floor(t) };
    let yaw = rnd(1.0) * m::PI * 2.0;
    let bias = (rnd(3.0) - 0.5) * 2.0 * b.asymmetry;
    let l = kk.lobes;
    for side in [1.0, -1.0] {
        let n = l.count * k.n * m::max(0.25, 1.0 + side * bias);
        for (j, share, fast) in [(-1.0, 0.3, 0.85), (0.0, 0.4, 1.0), (1.0, 0.3, 0.9)] {
            let a = yaw + j * l.fan;
            emit(g, pl, PKind::Spark, at, v3::normalize([m::sin(a) * side, l.rise, m::cos(a) * side]), m::round(n * share), EmitOpts {
                shape: Some(EmitShape::Cone), speed: l.speed * fast * k.v, speed_jit: Some(l.jit), spread: Some(l.cone), life: l.life * k.l, life_jit: Some(l.life_jit), size: l.size * k.s, size_jit: Some(l.size_jit), ..EmitOpts::default()
            });
        }
    }
    let f = kk.fan;
    emit(g, pl, PKind::Spark, at, [0.0, 1.0, 0.0], m::round(f.count * k.n), EmitOpts {
        shape: Some(EmitShape::Ring), radius: Some(0.03), speed: f.speed * k.v, speed_jit: Some(f.jit), lift: Some(f.lift), spread: Some(f.spread), life: f.life * k.l, life_jit: Some(f.life_jit), size: f.size * k.s, size_jit: Some(f.size_jit), ..EmitOpts::default()
    });
    burst_of(g, pl, at, kk.fountain, k);
    burst_of(g, pl, at, embers_as_fountain(&kk.embers), k);
}
