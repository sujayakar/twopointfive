//! THE BRING-UP'S OWN PART of blast_look: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// (blastLook.ts:69 CHARGE_LOOK: a table nothing writes; metres)
fn charge_look() -> CHARGELOOK {
    CHARGELOOK {
        frame_half: 0.3, frame_lo: 0.5, frame_hi: 1.6, strip: 0.045, proud: 0.012,
        strip_albedo: [0.62, 0.46, 0.1],
        lamp: 0.04, blink_every: 1.1, blink_on: 0.35, lamp_on: [70.0, 5.0, 2.5], lamp_off: [0.008, 0.0008, 0.0005],
        lamp_spill: 0.0025, led_spill: 0.02,
        tape: 0.07, wire_tape: 0.04,
        wire: 0.018, wire_px: 1.4, wire_max: 0.05, run: [0.4, 0.2, 0.6], short: 0.36, wire_albedo: [0.85, 0.36, 0.05],
        reel: [0.05, 0.035, 0.08], reel_albedo: [0.1, 0.11, 0.08],
        led: 0.013, led_on: [7.0, 3.6, 0.4],
    }
}
/// (blastLook.ts:118 WALL_FLASH.color)
const WALL_FLASH_COLOR: V3 = [1.0, 0.62, 0.25];

/// DESTRUCTION.on && BLAST.on && chaosOf(g) > 0   (blastLook.ts:155 live)
pub fn live(g: &mut crate::game::Game) -> bool { g.k.destruction.on && blast().on && crate::destruction::chaos_of(g) > 0.0 }

/// (blastLook.ts:157 near)
pub fn near(a: V3, b: V3, m: f64) -> bool { m::abs(a[0] - b[0]) + m::abs(a[1] - b[1]) + m::abs(a[2] - b[2]) < m }

/// where a wall charge's borrowed stun burst stands   (blastLook.ts:159 burstAt)
pub fn burst_at(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> V3 {
    let y = 0.5 + crate::effects::bang().height;
    crate::wallcharge::wall_pt(g, w, 0.0, 0.15, Some(y))
}

/// game.ts update, once a frame after the frame's other boxes: the set charges drawn, a blown one's flash warmed.
/// NOT a boundary no-op, though it is "look only": a set charge's thirteen boxes are on the FRAME'S LIST (which the fog of war, the listing of the box
/// world and every table numbered by reach count), and the flash it warms is a light of the engine's list, which the light estimate reads.
/// (blastLook.ts:162 blastLookTick)
pub fn blast_look_tick(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {
    let who = g.ch[g.pl[g.player].ch].id;
    let was = crate::fogofwar::fog_of_war_begin(g, Some(who));
    blast_look_tick_raw(g, out);
    crate::fogofwar::fog_of_war_done(g, was);
}

/// (blastLook.ts:163 blastLookTickRaw)
pub fn blast_look_tick_raw(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {
    let key = crate::seats::view_key(g);
    // (`g.wallCharge` is always there in the Rust game)
    if !live(g) {
        if let Some(s) = g.m.blast_look.state.get_mut(key) { s.seen.clear(); s.queue.clear(); }
        return;
    }
    let set = g.wall_charge.set.clone();
    if !g.m.blast_look.state.has(key) {
        if set.is_empty() { return; }
        g.m.blast_look.state.set(key, State { seen: Vec::new(), queue: Vec::new(), serial: 1.0, first: false, last: -1.0 });
    }
    let time = g.time;
    {
        let st = g.m.blast_look.state.get_mut(key).unwrap();
        if time < st.last { st.queue.clear(); }   // (the clock went back: a load / a restart)
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

/// how much of the firing line's run fits: the whole of it over clear floor, the short one where something stands in the way   (blastLook.ts:184 runOf)
pub fn run_of(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> f64 {
    let k = charge_look();
    let [r0, jog, r1] = k.run;
    let a = crate::wallcharge::wall_pt(g, w, 0.035, 0.05, Some(0.12));
    let b = crate::wallcharge::wall_pt(g, w, 0.035 + jog, r0 + r1 + 2.0 * k.reel[2] + 0.05, Some(0.12));
    if crate::scene_collision::solid_blocked(g, a, b, None) { k.short } else { 1.0 }
}

/// the frame of strip, the blinking lamp, the firing line and its reel: thirteen boxes of the frame   (blastLook.ts:190 chargeBoxes)
pub fn charge_boxes(g: &mut crate::game::Game, s: crate::blast_look::Seen, out: &mut Vec<BoxId>) {
    let k = charge_look();
    let w = g.tab.wallcharge_set_wall_charge[s.c].wall.clone();
    let h = g.k.wallcharge.height;
    let bf = crate::scene_boxes::box_flag();
    let flags = (js::to_i32(bf.dynamic) | js::to_i32(bf.no_shadow)) as f64;
    let yaw = if w.along_x { 0.0 } else { core::f64::consts::PI / 2.0 };
    fn bx(g: &mut Game, out: &mut Vec<BoxId>, yaw: f64, flags: f64, c: V3, h: V3, albedo: V3, emissive: V3) {
        let b = crate::scene_boxes::make_box_frame(g, &crate::scene_boxes::MakeBoxP { c, h, yaw: Some(yaw), albedo: Some(albedo), emissive: Some(emissive), flags: Some(flags), ..Default::default() });
        out.push(b);
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
    let ww = m::min(k.wire_max, m::max(k.wire, k.wire_px * g.m_per_px)) / 2.0;
    let drop = h - 0.11;
    let r0 = k.run[0] * s.run;
    let jog = k.run[1] * s.run;
    let r1 = k.run[2] * s.run;
    let a0 = 0.035;
    let amber = spill(k.wire_albedo, k.led_on, k.led_spill);
    let c = wp(g, a0, 0.008, drop / 2.0); bx(g, out, yaw, flags, c, [ww, drop / 2.0, 0.008], k.wire_albedo, glow(k.wire_albedo, k.wire_tape, k.lamp_spill / 2.0));
    let c = wp(g, a0, r0 / 2.0, 0.008); bx(g, out, yaw, flags, c, [ww, 0.008, r0 / 2.0], k.wire_albedo, line);
    let c = wp(g, a0 + jog / 2.0, r0, 0.008); bx(g, out, yaw, flags, c, [jog / 2.0 + ww, 0.008, ww], k.wire_albedo, line);
    let c = wp(g, a0 + jog, r0 + r1 / 2.0, 0.008); bx(g, out, yaw, flags, c, [ww, 0.008, r1 / 2.0], k.wire_albedo, v3::add(line, amber));
    let c = wp(g, a0 + jog, r0 + r1 + k.reel[2], k.reel[1]); bx(g, out, yaw, flags, c, k.reel, k.reel_albedo, spill(k.reel_albedo, k.led_on, k.led_spill));
    let c = wp(g, a0 + jog, r0 + r1 + k.reel[2], 2.0 * k.reel[1] + k.led); bx(g, out, yaw, flags, c, [k.led, k.led, k.led], [0.4, 0.3, 0.1], k.led_on);
}

/// the stun charge's flash a wall charge borrowed (detonateStun lit it this frame where spawnBurst put it); None: none burning there   (blastLook.ts:219 flashOf)
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

/// the light meter's own sum, in its own order   (blastLook.ts:225 LUM)
pub fn lum(r: f64, g_2: f64, b: f64) -> f64 { 0.2126 * r + 0.7152 * g_2 + 0.0722 * b }

/// one representable number up or down (x > 0)   (blastLook.ts:229 nudge)
pub fn nudge(_g: &mut crate::game::Game, x: f64, up: bool) -> f64 { let bits = x.to_bits(); f64::from_bits(if up { bits.wrapping_add(1) } else { bits.wrapping_sub(1) }) }

/// WALL_FLASH's hue at `c`'s luminance, the meter's sum kept TO THE BIT: red and blue scaled, green solved for, then green (failing that, blue) walked
/// one representable number at a time until the sum lands; a colour it cannot land stays as it was.   (blastLook.ts:230 warmOf)
pub fn warm_of(g: &mut crate::game::Game, c_2: V3) -> V3 {
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

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastLook.ts:239 afterglow)
pub fn afterglow(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastLook.ts:280 doorBlast)
pub fn door_blast(g: &mut crate::game::Game, pl: PlayerId, d: crate::blast_look::DoorBlastD) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastLook.ts:287 burstOf)
pub fn burst_of(g: &mut crate::game::Game, pl: PlayerId, at: V3, r: crate::blast_look::STUNSPARKSFountain, k: crate::blast_look::BurstOfK) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastLook.ts:293 emit)
pub fn emit(g: &mut crate::game::Game, pl: PlayerId, kind: crate::render_particles::PKind, pos: V3, dir: V3, count: f64, o: crate::render_particles::EmitOpts) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastLook.ts:302 wallBlast)
pub fn wall_blast(g: &mut crate::game::Game, pl: PlayerId, w: &crate::wallcharge::WallSpot) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastLook.ts:320 stunSparks)
pub fn stun_sparks(g: &mut crate::game::Game, pl: PlayerId, at: V3) {}

/// BOUNDARY NO-OP (dress, "LOOK ONLY": false = not this module's burst, the caller's own sparks stand)   (blastLook.ts:246 blastSparks)
pub fn blast_sparks(g: &mut crate::game::Game, at: V3) -> bool { false }
