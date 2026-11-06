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
/// sky's rays, and what it leaves on the copy the next rule finds. Sam's drips and prints after it (samAftermath, agePrints) are presentation only and stay
/// no-ops below. (src/game/wet.ts:96 updateWetness)
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

/// Sam indoors after the rain: his print budget and his drip clock. The drops themselves are GPU particles (`drip` below: nothing with no particle
/// system), but the ask of the sky is the rule's and moves his sky's memory, which the next soak reads. (src/game/wet.ts:115 samAftermath)
pub fn sam_aftermath(g: &mut crate::game::Game, dt: f64) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    mem_at(g, c);
    let raining = m::max(0.0, ::sim::weather::of(g.sim).rain_now) > 0.001 && open_sky_of(g, c);
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

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wet.ts:133 drip)
pub fn drip(g: &mut crate::game::Game, c_2: CharId) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wet.ts:143 wetFootfall)
pub fn wet_footfall(g: &mut crate::game::Game, left_hint: bool) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wet.ts:162 agePrints)
pub fn age_prints(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wet.ts:179 clearWetPrints)
pub fn clear_wet_prints(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wet.ts:183 wetInstance)
pub fn wet_instance(g: &mut crate::game::Game, c_2: CharId, on: bool, out: &mut Vec<f32>, o: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wet.ts:190 wetLineRow)
pub fn wet_line_row(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<f32>, o: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wet.ts:195 wetKnobs)
pub fn wet_knobs(g: &mut crate::game::Game, out: &mut Vec<f32>, o: f64) {}

/// BOUNDARY: the host's debug panel and `__wet.summary()` write this line (coats, prints, drips) at the swap; a headless run has no panel, so no line.   (src/game/wet.ts:204 wetSummary)
pub fn wet_summary(g: &mut crate::game::Game) -> String { String::new() }
