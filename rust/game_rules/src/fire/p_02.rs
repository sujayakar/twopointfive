//! src/game/fire.ts lines 320 to 439
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// bx.emissive = e (fire.ts:358): skeleton 5 keeps no row of a Box anywhere (`g.col.boxes` and `world.statics` are lists of BoxId, Tabs has no box table).
fn seam_box_set_emissive(g: &mut Game, bx: BoxId, e: V3) { g.tab.boxes[bx].emissive = e; }

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

/// The host's book of the fires, on a frame that moves the clock (where it has just been made). (fire.ts updateFire's `P`)
fn book(g: &mut Game) -> &mut FireShown { g.engine.host_books.as_mut().unwrap().fire.as_mut().unwrap() }

/// `arr[ix] = x` on a book's array: an index past its end leaves holes, written as `hole`, which every reader takes as the array's missing entry.
fn put_at(arr: &mut Vec<f64>, ix: usize, x: f64, hole: f64) {
    if arr.len() <= ix { arr.resize(ix + 1, hole); }
    arr[ix] = x;
}

/// A frame book's hole: `n` is at least 1 where these are read, so it never equals `n - 1` or `n`, as `undefined` does not.
const NO_FRAME: f64 = -1.0;

/// One fire's embers, as this frame files them: the closure's captures at fire.ts:369.
struct EmberFrom { mouth: MouthOf, flame_h: f64, e_up: f64, e_a: [f64; 4], heat: f64, si: f64 }

/// A throw of embers off one fire, by its own seed and not the particles' serial. (fire.ts:369 emberEmit)
fn ember_emit(g: &mut Game, f: &EmberFrom, n: f64, hard: f64, k: f64) {
    let fl = fire_look_smoke();
    let (ember_speed, ember_life, ember_size, ember_gain) = (g.k.fire.ember_speed, g.k.fire.ember_life, g.k.fire.ember_size, g.k.fire.ember_gain);
    let p = f.mouth.pos;
    let a = f.e_a;
    let (pos, dir, spread, speed_share, jit) = ([p[0] + a[0], p[1] + f.e_up, p[2] + a[1]], [a[2], 1.0, a[3]], fl.ember_spread + 0.6 * hard, fl.ember_speed, fl.ember_jit);
    let seed = (m::imul(js::to_i32(f.si + 1.0), 0x9E3779B1u32 as i32) ^ m::imul(js::to_i32(k + 1.0), 0x85EBCA77u32 as i32)) as u32 as f64;
    crate::render_particles::particles_emit(g, crate::render_particles::PKind::Spark, pos, dir, n, crate::render_particles::EmitOpts {
        shape: Some(crate::render_particles::EmitShape::Cone),
        spread: Some(spread),
        speed: ember_speed * speed_share * (1.0 + 1.6 * hard),
        speed_jit: Some(0.6),
        life: ember_life * (1.0 - 0.45 * hard),
        life_jit: Some(0.45),
        size: ember_size,
        size_jit: Some(0.4),
        pos_jit: Some(f.mouth.r * jit),
        ember: Some(m::min(0.99, ember_gain * (0.5 + 0.5 * f.heat))),
        seed: Some(seed),
        ..crate::render_particles::EmitOpts::zeroed()
    });
}

/// `n` embers off one fire, counted in the book. (fire.ts:371, :372, :375: `ember(n, hard, P.embers); P.embers += n`)
fn ember_throw(g: &mut Game, f: &EmberFrom, n: f64, hard: f64) {
    let k = book(g).embers;
    ember_emit(g, f, n, hard, k);
    book(g).embers += n;
}

/// `g.smoke.emit`: the real body, then the event for the host's solver.
fn smoke_emit(g: &mut Game, e: &crate::smoke_types::SmokeEmitter) {
    crate::smoke_types::smoke_system_like_emit(g, e);
    crate::out_events::push_smoke_system_like_emit(g, e);
}

/// Once a frame behind the lamps' paint: each fire's lamp flickered, moved and fed to its row, its coals, its cracks, and what the host shows and plays of it. (fire.ts updateFire)
pub fn update_fire(g: &mut Game, dt: f64) {
    let mut any = false;
    let mut i = 0;
    while i < g.targets.len() {
        let t = g.targets[i];
        if is_fire(g, t) { any = true; break; }
        i += 1;
    }
    if !any {
        return;
    }
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
                ember: Vec::new(), embers: 0.0, soot_mix: None,
                serial: 0,   // the picture's to deal, never g.serial(): a load rewinds that and the book stays. Only the bed's voice compares it, and that is not here.
            });
        }
        let p = books.fire.as_mut().unwrap();
        p.n += 1.0;
        p.time = time;
    }
    let focus = match g.engine.view_focus {
        Some(f) => f,
        None => { let h = crate::seats::host_man(g); let c = g.pl[h].ch; g.ch[c].pos }
    };
    let mut fires = 0.0;
    let mut lit = 0.0;
    let look = g.m.render_frame.look_config;
    let mut jets: Vec<crate::smoke_types::WindGpuSource> = Vec::new();
    let snow_lift = look.fire_snow > 0.0;
    if moved { state_of(g).frames += 1.0; }
    let mut i = 0;
    while i < g.targets.len() {
        let t = g.targets[i];
        i += 1;
        if !is_fire(g, t) { continue; }
        let ix = fires as usize;
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
                seam_box_set_emissive(g, bx, [glow[0] * ck, glow[1] * ck, glow[2] * ck]);
            }
        }
        if moved {
            let s = state_of(g);
            s.lo = m::min(s.lo, fl);
            s.hi = m::max(s.hi, fl);
            s.sum += fl;
            s.n += 1.0;
        }
        let crate::fire::WindAt { wv, wl, wind } = wind_at(g, mouth.pos);
        let d = m::hyp2(mouth.pos[0] - focus[0], mouth.pos[2] - focus[2]);
        // the embers: for a fire near the view, a steady few off the flame and a throw with each crack
        let (height, ember_r, embers, ember_life, ember_crack) = (g.k.fire.height, g.k.fire.ember_r, g.k.fire.embers, g.k.fire.ember_life, g.k.fire.ember_crack);
        let flame_h = height * g.k.fire.look.tall;
        let px = moved && embers > 0.0 && d < ember_r;
        let fl_smoke = fire_look_smoke();
        if moved && snow_lift && d < ember_r {
            // the falling snow: this fire's hot air
            let w = fl_smoke.snow;
            let a = axis_at(g, flame_h * 0.5, wind);
            let n = m::hyp3(a[2], 1.0, a[3]);
            jets.push(crate::smoke_types::WindGpuSource {
                kind: crate::smoke_types::WindGpuSourceKind::Jet,
                name: String::from("fire"),
                pos: mouth.pos,
                dir: [a[2] / n, 1.0 / n, a[3] / n],
                speed: w.speed * heat,
                reach: w.reach,
                tan_spread: w.spread,
                r0: mouth.r * w.mouth,
                d_max: w.top,
                bound: crate::smoke_types::WindGpuSourceBound { c: mouth.pos, r: w.top },
                heat: Some(true),
                ..crate::smoke_types::WindGpuSource::zeroed()
            });
        }
        let e_up = flame_h * fl_smoke.ember_up;
        let e_a = axis_at(g, e_up, wind);
        let from = EmberFrom { mouth, flame_h, e_up, e_a, heat, si };
        if moved && px && ember_crack > 0.0 && book(g).stir.as_ref().map_or(false, |v| v.contains(&(ix as f64))) {
            ember_throw(g, &from, m::round(ember_crack * 2.0), 1.0);
        }
        if px {
            let owe = book(g).ember.get(ix).copied().unwrap_or(0.0) + embers / m::max(ember_life, 0.1) * heat * dt;
            let n = m::floor(owe);
            put_at(&mut book(g).ember, ix, owe - n, 0.0);
            if n > 0.0 { ember_throw(g, &from, n, 0.0); }
        }
        // the cracks: the ones that fell in this frame, for the ear and (a throw of embers) for the eye
        if moved {
            for p in fire_pops(g, seed, time, 1.5) {
                if p.at > was && p.at <= time {
                    state_of(g).cracks += 1.0;
                    let gain = g.k.fire.crack * (0.5 + 0.5 * p.hard) * heat;
                    let opts = crate::audio_iface::PlayOpts::Rate { rate: 0.8 + 0.5 * p.seed };
                    crate::audio_iface::game_audio_like_play(g, "fireCrack", Some(mouth.pos), Some(gain), Some(&opts));
                    if px && ember_crack > 0.0 {
                        ember_throw(g, &from, m::round(ember_crack * (0.4 + 0.6 * p.hard)), p.hard);
                    }
                }
            }
        }
        // the bed: asked for when the fire starts to burn and handed on every `bed_secs`. The closure of fire.ts:379 (`follow`) has no arm in PlayOpts: notes, ## Out
        if moved {
            let (bed_secs, bed_gain) = (g.k.fire.bed_secs, g.k.fire.bed);
            let slot = m::floor(time / bed_secs);
            let p = book(g);
            let n = p.n;
            let goes = p.bed.get(ix).copied() == Some(n - 1.0);
            put_at(&mut p.bed, ix, n, NO_FRAME);
            if slot != m::floor(was / bed_secs) || !goes {
                p.beds += 1.0;
                crate::audio_iface::game_audio_like_play(g, "fireBed", Some(mouth.pos), Some(bed_gain), None);
            }
        }
        // the smoke: a short source on the solver, filed afresh every `every` s while the fire is near the view
        let b = g.k.fire.smoke;
        if moved && d < b.view_r {
            let slot = m::floor(time / b.every);
            let p = book(g);
            let n = p.n;
            let goes = p.smoke.get(ix).copied() == Some(n - 1.0);
            put_at(&mut p.smoke, ix, n, NO_FRAME);
            if slot != m::floor(was / b.every) || !goes {
                p.smokes += 1.0;
                let l = fl_smoke;
                let a = axis_at(g, height * l.up, wind);
                smoke_emit(g, &crate::smoke_types::SmokeEmitter {
                    pos: [mouth.pos[0] + a[0], mouth.pos[1] + height * l.up, mouth.pos[2] + a[1]],
                    dir: [a[2], 1.0, a[3]],
                    speed: b.speed,
                    radius: l.radius,
                    min_voxels: Some(b.cells),
                    density: b.density * heat * l.density,
                    temperature: b.temp * heat,
                    ttl: b.every,
                    age: 0.0,
                    kind: crate::smoke_types::SmokeEmitterKind::Cloud,
                    push: Some(b.push),
                    jitter: Some(b.jitter),
                    lattice: Some(crate::smoke_types::SmokeEmitterLattice::Coarse),
                    look: Some(true), fire: Some(true),
                    tint: Some(l.soot),
                    ..crate::smoke_types::SmokeEmitter::zeroed()
                });
            }
            // the look prototype: the fire puffs, a short burst of thicker, hotter gas, a few a second
            let l = fl_smoke;
            let ps = m::floor(time / l.puff_every);
            if ps != m::floor(was / l.puff_every) || !goes {
                let ox = (h01(si + ps * 31.0 + 11.0) - 0.5) * 2.0 * l.puff_off;
                let oz = (h01(si + ps * 31.0 + 12.0) - 0.5) * 2.0 * l.puff_off;
                let k = 0.7 + 0.6 * h01(si + ps * 31.0 + 13.0);
                let a = axis_at(g, height * l.puff_up, wind);
                smoke_emit(g, &crate::smoke_types::SmokeEmitter {
                    pos: [mouth.pos[0] + ox + a[0], mouth.pos[1] + height * l.puff_up, mouth.pos[2] + oz + a[1]],
                    dir: [a[2] + ox, 1.0, a[3] + oz],
                    speed: l.puff_speed * k,
                    radius: l.puff_radius,
                    min_voxels: Some(b.cells),
                    density: l.puff_density * heat * k,
                    temperature: l.puff_temp * heat * k,
                    ttl: l.puff_secs,
                    age: 0.0,
                    kind: crate::smoke_types::SmokeEmitterKind::Cloud,
                    push: Some(l.puff_push),
                    expand: Some(l.puff_expand),
                    attack: Some(0.06),
                    jitter: Some(0.9),
                    lattice: Some(crate::smoke_types::SmokeEmitterLattice::Coarse),
                    look: Some(true), fire: Some(true),
                    tint: Some(l.soot),
                    ..crate::smoke_types::SmokeEmitter::zeroed()
                });
            }
        }
    }
    if moved {
        if let Some(stir) = g.engine.host_books.as_mut().and_then(|b| b.fire.as_mut()).and_then(|p| p.stir.as_mut()) {
            stir.clear();
        }
        crate::render_particles::particles_set_heat(g, &jets);
    }
    let s = state_of(g);
    s.fires = fires;
    s.lit = lit;
}


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
