//! THE BRING-UP'S OWN PART of wet: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A man's own bookkeeping, as wet.ts memOf makes it the first time he is asked for.
fn mem_at(g: &mut Game, c: CharId) -> crate::wet::WetMem {
    if !g.m.wet.mem.has(c) {
        g.m.wet.mem.set(c, crate::wet::WetMem { sky_t: -1.0, open: true, prints_left: 0.0, last_out_wet_t: -1e9, drip_acc: 0.0 });
    }
    *g.m.wet.mem.get(c).unwrap()
}

/// how high his head is, as the rule reads it (wet.ts headOf)
fn head_at(g: &Game, c: CharId) -> f64 {
    let ch = &g.ch[c];
    let head = ch.bones.get(crate::character::Bone::Head).map(|b| b[1]);
    if ch.ragdoll.is_some() { m::max(0.3, head.unwrap_or(0.3)) } else { head.unwrap_or(ch.pos[1] + 1.6) }
}

/// Once a tick, after the weather has written this tick's rain: soak / dry everybody. The rule is rust/sim's `weather::wet_step`, a man's coat handed in and
/// back. A man's coat is the picture's, but the call is the game's: it stands the rule's copy of the box world (`rule_open`), asks the roofs and may cast the
/// sky's rays, and what it leaves on the copy the next rule finds. Sam's drips and prints after it (samAftermath) tell the host; agePrints has nothing to age
/// while `decals_add` answers no decal. (src/game/wet.ts:96 updateWetness)
pub fn update_wetness(g: &mut crate::game::Game, dt: f64) {
    if dt <= 0.0 { return; }
    crate::weather::weather_roofs_now(g);
    if !g.m.rustknobs.rust_knobs.tick { crate::rustknobs::knobs_to_rust(g); } // WX_KNOBS.here
    crate::scene_collision::rule(g, &mut |g: &mut Game| {
        crate::scene_collision::rule_open(g);
        let (w, set, ask) = (g.col.mirror.slot as usize, g.col.mirror.rule_set as i32, g.col.mirror.rule_panes as u32);
        // everyCharacter: Sam (every seat's man), then the guards, living or not
        let mut all: Vec<CharId> = crate::seats::men(g).into_iter().map(|p| g.pl[p].ch).collect();
        all.extend(g.guards.iter().map(|gd| g.gd[*gd].ch));
        for c in all {
            let mem = mem_at(g, c);
            let head = head_at(g, c);
            let ch = &g.ch[c];
            let mut coat = [0.0f64; ::sim::weather::M_LEN];
            coat[::sim::weather::M_WET] = ch.wet;
            coat[::sim::weather::M_BEADS] = ch.beads;
            coat[::sim::weather::M_RAIN] = ch.wet_rain;
            coat[::sim::weather::M_LINE] = ch.wet_line;
            coat[::sim::weather::M_SKY_T] = mem.sky_t;
            coat[::sim::weather::M_OPEN] = if mem.open { 1.0 } else { 0.0 };
            ::sim::weather::wet_step(::sim::weather::of(g.sim), w, set, ask, g.time, dt, ch.pos[0], ch.pos[2], head, &mut coat);
            if coat[::sim::weather::M_ASKED] != 0.0 { g.stats.wet.sky_tests += 1; }
            let ch = &mut g.ch[c];
            ch.wet = coat[::sim::weather::M_WET];
            ch.beads = coat[::sim::weather::M_BEADS];
            ch.wet_rain = coat[::sim::weather::M_RAIN];
            ch.wet_line = coat[::sim::weather::M_LINE];
            let m = g.m.wet.mem.get_mut(c).unwrap();
            m.sky_t = coat[::sim::weather::M_SKY_T];
            m.open = coat[::sim::weather::M_OPEN] != 0.0;
        }
    });
    crate::seats::each_seat(g, &mut |v: &mut Game| sam_aftermath(v, dt));
    age_prints(g);
}

/// is `c` standing under the open sky: the module's rule, his sky's memory handed in and back (it holds a quarter of a second a man). (src/game/wet.ts:75 openSky)
fn open_sky_of(g: &mut Game, c: CharId) -> bool {
    let mem = mem_at(g, c);
    crate::weather::weather_roofs_now(g);
    crate::scene_collision::rule(g, &mut |g: &mut Game| {
        crate::scene_collision::rule_open(g);
        let (w, set, ask) = (g.col.mirror.slot as usize, g.col.mirror.rule_set as i32, g.col.mirror.rule_panes as u32);
        let head = head_at(g, c);
        let (mut sky_t, mut open) = (mem.sky_t, if mem.open { 1.0 } else { 0.0 });
        let (x, z) = (g.ch[c].pos[0], g.ch[c].pos[2]);
        let (is_open, asked) = ::sim::weather::open_sky(::sim::weather::of(g.sim), w, set, ask, g.time, x, z, head, &mut sky_t, &mut open);
        if asked { g.stats.wet.sky_tests += 1; }
        let m = g.m.wet.mem.get_mut(c).unwrap();
        m.sky_t = sky_t;
        m.open = open != 0.0;
        is_open
    })
}

/// Is `c` under the open sky, asked from outside the tick (a panel's line, a check). The same ask as the tick's, so it moves his sky's memory as the
/// TypeScript's did. (src/game/wet.ts:75 openSky)
pub fn open_sky(g: &mut Game, c_2: CharId) -> bool { open_sky_of(g, c_2) }

/// Is `p` under one of the roofs the rain is cut by: the weather's own footprints in force. (src/game/wet.ts:64 underRoofRect; its arm for a caller with
/// no game, the office's two fixed rectangles, has no caller here: a Rust game always has its weather)
pub fn under_roof_rect(g: &mut Game, p: V3) -> bool { crate::weather::weather_under_roof(g, p[0], p[2]) }

/// Sam indoors after the rain: his print budget and his drip clock. The drops themselves are GPU particles (`drip` below: nothing with no particle
/// system), but the ask of the sky is the rule's and moves his sky's memory, which the next soak reads. (src/game/wet.ts:115 samAftermath)
pub fn sam_aftermath(g: &mut crate::game::Game, dt: f64) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    mem_at(g, c);
    let raining = m::max(0.0, ::sim::weather::of_read(g.sim).rain_now) > 0.001 && open_sky_of(g, c);
    if raining {
        if g.ch[c].wet > 0.5 {
            let (prints, now) = (g.k.wet.prints, g.time);
            let m = g.m.wet.mem.get_mut(c).unwrap();
            m.prints_left = prints;
            m.last_out_wet_t = now;
        }
        return;
    }
    let (beads, wet, down, now, secs) = (g.ch[c].beads, g.ch[c].wet, g.pl[pl].down, g.time, g.k.wet.drip_secs);
    let last = g.m.wet.mem.get(c).unwrap().last_out_wet_t;
    if beads > 0.02 && wet > 0.3 && now - last < secs && !down {
        let mut acc = g.m.wet.mem.get(c).unwrap().drip_acc + dt * (2.0 + 10.0 * beads);
        while acc >= 1.0 { acc -= 1.0; drip(g, c); }
        g.m.wet.mem.get_mut(c).unwrap().drip_acc = acc;
    } else {
        g.m.wet.mem.get_mut(c).unwrap().drip_acc = 0.0;
    }
}

/// where a drop leaves him, dealt in turn (wet.ts DRIP_FROM)
const DRIP_FROM: [crate::character::Bone; 5] = [crate::character::Bone::HandL, crate::character::Bone::HandR, crate::character::Bone::Head, crate::character::Bone::Hips, crate::character::Bone::Hips];

/// One drop off him: from a hand, the chin or the hem of the jacket. With no particle system nothing moves, as in the TypeScript. (src/game/wet.ts:129 drip)
pub fn drip(g: &mut crate::game::Game, c_2: CharId) {
    if !crate::particle_fx::gpu_fx(g) { return; }
    let n = g.m.wet.drip_n;
    g.m.wet.drip_n = n + 1.0;
    let k = DRIP_FROM[js::rem(n, 5.0) as usize];
    let Some(b) = g.ch[c_2].bones.get(k).copied() else { return; };
    let j = g.m.wet.drip_n * 0.618034;
    let down = match k { crate::character::Bone::Head => -0.12, crate::character::Bone::Hips => -0.05, _ => -0.06 };
    let off: V3 = [m::cos(j * 6.2831853) * 0.09, down, m::sin(j * 6.2831853) * 0.09];
    let (vel, id) = (g.ch[c_2].vel, g.ch[c_2].id);
    crate::render_particles::particles_emit(g, crate::render_particles::PKind::Debris, v3::add(b, off), [0.0, -1.0, 0.0], 1.0, crate::render_particles::EmitOpts {
        speed: 0.4,
        speed_jit: Some(0.5),
        spread: Some(0.15),
        life: 0.55,
        life_jit: Some(0.3),
        size: 0.0045,
        size_jit: Some(0.3),
        vadd: Some(vel),
        owner: Some(id),
        color: Some([0.42, 0.47, 0.55]),
        ..crate::render_particles::EmitOpts::zeroed()
    });
    g.stats.wet.drips += 1;
}

/// A footfall of Sam's: if he has come in wet, a print under the foot that landed, fainter each step. The foot ahead along his motion wins over `left_hint`
/// whenever he is moving. (src/game/wet.ts:139 wetFootfall)
pub fn wet_footfall(g: &mut crate::game::Game, left_hint: bool) {
    use crate::character::Bone;
    let pl = g.player;
    let c = g.pl[pl].ch;
    let prints_left = mem_at(g, c).prints_left;
    if prints_left <= 0.0 || g.ch[c].wet < 0.15 { return; }
    if m::max(0.0, ::sim::weather::of_read(g.sim).rain_now) > 0.001 && open_sky_of(g, c) { return; } // still out in it: the ground is wetter than his soles
    let bone = |g: &Game, b: Bone| g.ch[c].bones.get(b).copied();
    let mut left = left_hint;
    if let (Some(fl), Some(fr)) = (bone(g, Bone::FootL), bone(g, Bone::FootR)) {
        let vel = g.ch[c].vel;
        if m::abs(vel[0]) + m::abs(vel[2]) > 0.2 { left = (fl[0] - fr[0]) * vel[0] + (fl[2] - fr[2]) * vel[2] > 0.0; }
    }
    let (Some(foot), Some(toe)) = (bone(g, if left { Bone::FootL } else { Bone::FootR }), bone(g, if left { Bone::ToeL } else { Bone::ToeR })) else { return; };
    let k = (prints_left / m::max(1.0, g.k.wet.prints)) * v3::clamp(g.ch[c].wet * 1.3, 0.0, 1.0); // strength: the budget left × how wet he still is
    g.m.wet.mem.get_mut(c).unwrap().prints_left = prints_left - 1.0;
    let dir = v3::normalize([toe[0] - foot[0], 0.0, toe[2] - foot[2]]);
    let mut centre: V3 = [foot[0] * 0.45 + toe[0] * 0.55, 0.005, foot[2] * 0.45 + toe[2] * 0.55];
    let hit = crate::scene_collision::raycast(g, [centre[0], 0.4, centre[2]], [0.0, -1.0, 0.0], 0.8);
    let y = match hit { Some(h) => 0.4 - h.t, None => 0.0 };
    centre[1] = y + 0.003;
    // seed bit 0 = which foot (the sole is mirrored), the rest = look
    let seed = (if left { 0 } else { 1 }) | (js::to_i32(g.stats.wet.prints as f64 * 37.0) & 0xffe);
    let added = crate::render_decals::decals_add(g, crate::render_decals::DecalKind::Wetprint, centre, [0.0, 1.0, 0.0], Some(0.17), Some(crate::render_decals::DecalOpts {
        tangent: Some(dir),
        param: Some(k),
        look: Some(true),
        seed: Some(seed as f64),
        ..crate::render_decals::DecalOpts::zeroed()
    }));
    let Some(d) = crate::look_decals::look_decal(g, added.as_ref()) else { return; };
    let t0 = g.time;
    g.m.wet.prints.get_or_insert_with(|| crate::wet::WetModPrintsValue { list: Vec::new(), age_t: 0.0 }).list.push(crate::wet::PrintRec { d, t0, k0: k });
    g.stats.wet.prints += 1;
}

/// BOUNDARY NO-OP: the list it ages is empty while `decals_add` answers no decal, and the TypeScript with an empty list does nothing. (src/game/wet.ts:158 agePrints)
pub fn age_prints(g: &crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wet.ts:179 clearWetPrints)
pub fn clear_wet_prints(g: &crate::game::Game) {}

/// BOUNDARY: the host's debug panel and `__wet.summary()` write this line (coats, prints, drips) at the swap; a headless run has no panel, so no line.   (src/game/wet.ts:204 wetSummary)
pub fn wet_summary(g: &crate::game::Game) -> String { String::new() }
