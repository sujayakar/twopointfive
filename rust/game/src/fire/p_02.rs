//! src/game/fire.ts lines 320 to 439
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// g.audio.play(name, at, gain, { rate }) (fire.ts:374): game_audio_like_play's opts are a map of `Opaque`, an empty struct, which cannot hold the rate.
fn seam_audio_play(g: &mut Game, name: &str, at: V3, gain: f64, rate: f64) { todo!("needs: game_audio_like_play opts that can hold a rate") }

/// A lamp's place across: where the level hung it, wandering by two sines. (fire.ts:354, 355)
fn wander_across(hung: f64, wander: f64, rate_a: f64, phase_a: f64, rate_b: f64, phase_b: f64, time: f64) -> f64 {
    hung + wander * m::sin(TAU * (rate_a * time + phase_a)) * m::sin(TAU * (rate_b * time + phase_b))
}
/// A lamp's height: in the body of the flame, up with the flicker by no more than the table says. (fire.ts:356)
fn wander_height(hung: f64, lift: f64, wander_up: f64, flicker: f64, depth: f64) -> f64 {
    hung + lift + wander_up * m::max(-1.0, m::min(1.0, (flicker - 1.0) / m::max(depth, 1e-6)))
}
/// The coals' share of the authored glow, breathing with the light. (fire.ts:358)
fn coal_share(factor: f64, coals: f64, coal_swing: f64, flicker: f64) -> f64 {
    factor * coals * (1.0 + coal_swing * (flicker - 1.0))
}

/// Once a frame behind the lamps' paint: each fire's lamp flickered, moved and fed to its row, its coals, its cracks. (fire.ts updateFire)
pub fn update_fire(g: &mut Game, dt: f64) {
    let mut any = false;
    let mut i = 0;
    while i < g.targets.len() {
        let t = g.targets[i];
        if is_fire(g, t) { any = true; break; }
        i += 1;
    }
    if !any { return; }
    state_of(g);   // makes the row; each write below gets it again, since calls that take g lie between them
    let time = g.time;
    let was = time - m::max(dt, 0.0);
    let moved = dt > 0.0;
    if moved {
        // the host's book, which fire_stir reads: a fresh one when there is none or the clock went back on it
        let stale = match g.engine.host_books.as_ref().and_then(|b| b.fire.as_ref()) {
            Some(p) => was < p.time - 1e-9,
            None => true,
        };
        let books = g.engine.host_books.get_or_insert_with(crate::host_api::HostBooks::default);
        if stale {
            books.fire = Some(FireShown {
                n: 0.0, time: 0.0, stir: None, stir_at: None, bed: Vec::new(), smoke: Vec::new(), smokes: 0.0, beds: 0.0,
                ember: Vec::new(), embers: 0.0, gas: Vec::new(), gas_mix: Vec::new(), gases: 0.0, soot_mix: None, kept: None,
                serial: 0,   // the picture's to deal, never g.serial(): a load rewinds that and the book stays. Only the bed's voice compares it, and that is not here.
            });
        }
        let p = books.fire.as_mut().unwrap();
        p.n += 1.0;
        p.time = time;
    }
    let mut fires = 0.0;
    let mut lit = 0.0;
    if moved { state_of(g).frames += 1.0; }
    let mut i = 0;
    while i < g.targets.len() {
        let t = g.targets[i];
        i += 1;
        if !is_fire(g, t) { continue; }
        fires += 1.0;
        let l = g.tab.sim_lightshome_fixture[t].light.unwrap();
        let heat = heat_of(g, t);
        if !burns(g, t) {
            // out: while the fog's picture has it lit it is held as a fire's lamp, and for the one frame before the fog has had its say
            let ghost = g.tab.light[l].fog_ghost.unwrap_or(0.0) > 0.0;
            let first = match g.tab.light[l].steady { Some(st) => !st.out.unwrap_or(false), None => false };
            if ghost || first {
                held(g, t);
                g.tab.light[l].steady.as_mut().unwrap().out = Some(true);
            } else {
                plain(g, t);
            }
            crate::sim_lightshome::light_home_feed_burning(g, l);
            continue;
        }
        lit += 1.0;
        let seed = fire_seed(g, t);
        let fl = fire_flicker(g, seed, time);
        let mouth = mouth_of(g, t);
        let si = m::floor(seed * 65535.0);
        let hung = g.tab.sim_lightshome_fixture[t].pos;
        // the light: the authored lamp x the flicker, its place wandering with the flames; a reader of the place's steady light takes it for the authored lamp
        if g.tab.light[l].steady.is_none() {
            let lamp = &mut g.tab.light[l];
            lamp.steady = Some(crate::render_lights::RtLightSteady {
                intensity: 0.0, pos: hung, radius: lamp.radius, color: Some(lamp.color), range: Some(lamp.range), out: None,
            });
        }
        g.tab.light[l].steady.as_mut().unwrap().out = Some(false);
        let radius = m::max(g.tab.light[l].steady.unwrap().radius, g.k.fire.size * mouth.r);
        g.tab.light[l].radius = radius;
        let intensity = crate::sim_lightshome::fixture_base_intensity(g, t) * crate::sim_lightshome::fixture_factor(g, t) * g.k.fire.bright;
        g.tab.light[l].steady.as_mut().unwrap().intensity = intensity;
        let st = g.tab.light[l].steady.unwrap();
        let c0 = match st.color { Some(c) => c, None => g.tab.light[l].color };
        let warm = |n: usize| g.k.fire.warm.get(n).copied().unwrap_or(1.0);
        let color = [c0[0] * warm(0), c0[1] * warm(1), c0[2] * warm(2)];
        g.tab.light[l].color = color;
        let range = match st.range { Some(r) => r, None => g.tab.light[l].range };
        g.tab.light[l].range = range * g.k.fire.reach;
        g.tab.light[l].intensity = st.intensity * fl;
        let wander = g.k.fire.wander;
        let x = wander_across(hung[0], wander, 0.83, h01(si + 5.0), 2.9, h01(si + 6.0), time);
        let z = wander_across(hung[2], wander, 0.71, h01(si + 7.0), 3.3, h01(si + 8.0), time);
        let y = wander_height(hung[1], g.k.fire.lift, g.k.fire.wander_up, fl, g.k.fire.depth);
        g.tab.light[l].pos = [x, y, z];
        crate::sim_lightshome::light_home_feed(g, l);
        // the coals, at this share of the glow updateTargets painted
        let ck = coal_share(crate::sim_lightshome::fixture_factor(g, t), g.k.fire.coals, g.k.fire.coal_swing, fl);
        let glow = g.tab.sim_lightshome_fixture[t].base_emissive;
        let mut b = 0;
        while b < g.tab.sim_lightshome_fixture[t].boxes.len() {
            let bi = g.tab.sim_lightshome_fixture[t].boxes[b];
            b += 1;
            if let Some(&bx) = g.engine.world.statics.get(bi as usize) {
                g.col.boxes[bx].emissive = [glow[0] * ck, glow[1] * ck, glow[2] * ck];
            }
        }
        if moved {
            let s = state_of(g);
            s.lo = m::min(s.lo, fl);
            s.hi = m::max(s.hi, fl);
            s.sum += fl;
            s.n += 1.0;
        }
        // the cracks: the ones that fell in this frame, for the ear
        if moved {
            for p in fire_pops(g, seed, time, 1.5) {
                if p.at > was && p.at <= time {
                    state_of(g).cracks += 1.0;
                    let gain = g.k.fire.crack * (0.5 + 0.5 * p.hard) * heat;
                    seam_audio_play(g, "fireCrack", mouth.pos, gain, 0.8 + 0.5 * p.seed);
                }
            }
        }
    }
    if moved {
        if let Some(stir) = g.engine.host_books.as_mut().and_then(|b| b.fire.as_mut()).and_then(|p| p.stir.as_mut()) {
            stir.clear();
        }
    }
    let s = state_of(g);
    s.fires = fires;
    s.lit = lit;
}

/// The fires the picture draws. Picture only, behind DRESS.fires at game.ts:1836: not translated (conventions 6), the stub stays. (fire.ts fireDraws)
pub fn fire_draws(g: &mut crate::game::Game) { todo!("picture only: stays on the TypeScript dressing side") }

/// One line for the panel and the probes. Host only: not translated (conventions 6), the stub stays. (fire.ts fireSummary)
pub fn fire_summary(g: &mut crate::game::Game) -> String { todo!("host only: the panel and the probes") }

#[cfg(test)]
mod tests {
    use super::*;
    // Each number is the TypeScript's own expression under bun with EXACT.on, the page's table: wander 0.14, wanderUp 0.1, lift 0.5, depth 0.4, coals 0.26, coalSwing 0.6.
    #[test]
    fn lamp_wanders_across_on_the_typescript_bits() {
        assert_eq!(wander_across(3.25, 0.14, 0.83, 0.1937, 2.9, 0.7711, 123.456).to_bits(), 0x400aeb1a98f0afd6);
        assert_eq!(wander_across(-7.5, 0.14, 0.71, 0.5021, 3.3, 0.0313, 987.654321).to_bits(), 0xc01e8a3e54d176b0);
    }
    #[test]
    fn lamp_height_is_capped_at_the_swing_on_the_typescript_bits() {
        assert_eq!(wander_height(1.13, 0.5, 0.1, 1.07, 0.4).to_bits(), 0x3ffa5c28f5c28f5c);
        assert_eq!(wander_height(1.13, 0.5, 0.1, 1.6, 0.4).to_bits(), 0x3ffbae147ae147ae);   // a crack's kick, past the swing
        assert_eq!(wander_height(1.13, 0.5, 0.1, 0.6, 0.4).to_bits(), 0x3ff87ae147ae147a);
        assert_eq!(wander_height(1.13, 0.5, 0.1, 0.81, 0.4).to_bits(), 0x3ff951eb851eb852);
    }
    #[test]
    fn coals_breathe_on_the_typescript_bits() {
        assert_eq!(coal_share(1.0, 0.26, 0.6, 1.31).to_bits(), 0x3fd3bc2b94d94079);
        assert_eq!(coal_share(0.37, 0.26, 0.6, 0.83).to_bits(), 0x3fb61d7f6cc7a978);
    }
}
