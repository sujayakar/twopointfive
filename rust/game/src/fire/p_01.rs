//! src/game/fire.ts lines 1 to 320
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Whether a fire's pieces are filed as one column.   (fire.ts fits)
pub fn fits(g: &mut crate::game::Game) -> bool {
    let c = &g.m.render_frame.look_config;
    c.on && c.fire_smoke > 0.0 && c.fire_fit > 0.0
}

/// The flame's own middle `up` metres over its mouth: how far downwind (x, z) and its slope there.   (fire.ts axisAt)
pub fn axis_at(g: &mut crate::game::Game, up: f64, wind: [f64; 2]) -> [f64; 4] {
    axis_with(&g.k.fire, up, wind)
}
fn axis_with(k: &crate::knobs::FireK, up: f64, wind: [f64; 2]) -> [f64; 4] {
    let h = k.height * k.look.tall;
    let y = m::max(up, 0.0) / h;
    let capped = m::min(y, 1.5);
    let lean = k.look.lean;
    let b = lean * (capped * m::sqrt(capped) + 0.735 * m::max(y - 1.5, 0.0));
    let sl = lean * (if y < 1.5 { 1.5 * m::sqrt(capped) } else { 0.735 }) / h;
    [wind[0] * b, wind[1] * b, wind[0] * sl, wind[1] * sl]
}

/// A new row of the counts: its least and most start at the infinities, not at a blank's zeros.   (fire.ts fresh)
pub fn fresh() -> crate::fire::FireStats {
    crate::fire::FireStats { fires: 0.0, lit: 0.0, frames: 0.0, cracks: 0.0, lo: m::INF, hi: -m::INF, sum: 0.0, n: 0.0 }
}

/// The game's row of counts, made if it has none: the row itself, a caller writes through it.   (fire.ts stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::fire::FireStats {
    g.m.fire.states.get_or_insert_with(fresh)
}

/// The counts to read now and not to keep: a game the fire has not run in gets a blank row, and none is made for it.   (fire.ts fireStats)
pub fn fire_stats(g: &mut crate::game::Game) -> crate::fire::FireStats {
    match g.m.fire.states {
        Some(s) => s,
        None => fresh(),
    }
}

/// The host's book of the fires, for the panel's line and the tools: none before the first frame a fire burns in.   (fire.ts fireShown)
pub fn fire_shown(g: &mut crate::game::Game) -> Option<crate::fire::FireShown2> {
    let f = g.engine.host_books.as_ref()?.fire.as_ref()?;
    Some(crate::fire::FireShown2 {
        n: f.n,
        time: f.time,
        stir: f.stir.clone(),
        stir_at: f.stir_at.clone(),
        bed: f.bed.clone(),
        smoke: f.smoke.clone(),
        smokes: f.smokes,
        beds: f.beds,
        ember: f.ember.clone(),
        embers: f.embers,
        gas: f.gas.clone(),
        gas_mix: f.gas_mix.clone(),
        gases: f.gases,
        soot_mix: f.soot_mix.clone(),
        kept: f.kept,
    })
}

/// Integer hash to [0, 1). The TypeScript's signed and unsigned steps give these same 32 bits.   (fire.ts h01)
pub fn h01(x: f64) -> f64 {
    let mut x = (js::to_i32(x) as u32 ^ 0x2545F491).wrapping_mul(0x9E3779B1);
    x ^= x >> 15;
    x = x.wrapping_mul(0x85EBCA77);
    x ^= x >> 13;
    (x >> 8) as f64 / 16777216.0
}

/// Is this light target an open fire, by its name.   (fire.ts isFire)
pub fn is_fire(g: &mut crate::game::Game, t: FixtureId) -> bool {
    let row = &g.tab.sim_lightshome_fixture[t];
    if row.kind != crate::sim_lightshome::FixtureKind::Analytic || row.light.is_none() {
        return false;
    }
    for n in &g.k.fire.names {
        if row.name.starts_with(n.as_str()) {
            return true;
        }
    }
    false
}

/// A fire's own seed, 0 to 1, off where it stands.   (fire.ts fireSeed)
pub fn fire_seed(g: &mut crate::game::Game, t: FixtureId) -> f64 {
    let p = g.tab.sim_lightshome_fixture[t].pos;
    seed_at(p)
}
fn seed_at(p: V3) -> f64 {
    h01(m::round(p[0] * 100.0) * 7919.0 + m::round(p[2] * 100.0) * 104729.0 + 17.0)
}

/// The cracks of the wood near `time`, the latest first, none older than `back` s.   (fire.ts firePops)
pub fn fire_pops(g: &mut crate::game::Game, seed: f64, time: f64, back: f64) -> Vec<crate::fire::FirePopsItem> {
    pops_with(&g.k.fire, seed, time, back)
}
fn pops_with(fk: &crate::knobs::FireK, seed: f64, time: f64, back: f64) -> Vec<crate::fire::FirePopsItem> {
    let (pop_slot, pop_rate) = (fk.pop_slot, fk.pop_rate);
    let mut out = Vec::new();
    let s = m::floor(seed * 65535.0);
    let k1 = m::floor(time / pop_slot);
    let mut k = k1;
    while k >= 0.0 && (k1 - k) * pop_slot < back + pop_slot {
        let this_k = k;
        k -= 1.0;
        let a = h01(this_k * 2654435.0 + s * 31.0 + 1.0);
        if a >= pop_rate * pop_slot {
            continue;
        }
        let at = (this_k + h01(this_k * 97.0 + s + 2.0)) * pop_slot;
        if at > time || time - at > back {
            continue;
        }
        out.push(crate::fire::FirePopsItem {
            at,
            hard: 0.35 + 0.65 * h01(this_k * 193.0 + s + 3.0),
            seed: h01(this_k * 389.0 + s + 4.0),
        });
    }
    out
}

/// The light's flicker at `time`: what the authored strength is multiplied by.   (fire.ts fireFlicker)
pub fn fire_flicker(g: &mut crate::game::Game, seed: f64, time: f64) -> f64 {
    flicker_with(&g.k.fire, seed, time)
}
fn flicker_with(fk: &crate::knobs::FireK, seed: f64, time: f64) -> f64 {
    let (mut s, mut w) = (0.0, 0.0);
    let si = m::floor(seed * 65535.0);
    for i in 0..fk.rates.len() {
        let weight = fk.weights[i];
        let n = i as f64;
        let x = fk.rates[i] * time + h01(si + n * 71.0);
        let k = m::floor(x);
        let u = x - k;
        let e = u * u * (3.0 - 2.0 * u);
        let c = si + n * 7919.0;
        s += weight * (2.0 * (h01(c + k * 131.0) * (1.0 - e) + h01(c + (k + 1.0) * 131.0) * e) - 1.0);
        w += weight * weight;
    }
    let mut f = 1.0 + fk.depth * s / m::max(m::sqrt(w), 1e-6);
    for p in pops_with(fk, seed, time, fk.kick_secs * 4.0) {
        f += fk.kick * p.hard * m::exp(-(time - p.at) / fk.kick_secs);
    }
    m::max(fk.least, m::min(fk.most, f))
}

/// `statics[ix]`: none past the list's end, as an array read gives undefined there.
fn static_at(g: &mut Game, ix: f64) -> Option<BoxId> {
    if !(ix >= 0.0) || m::floor(ix) != ix {
        return None;
    }
    g.engine.world.statics.get(ix as usize).copied()
}

/// Where the fire's mouth is and how wide: the glowing box hung on the lamp, or a hand under the lamp.   (fire.ts mouthOf)
pub fn mouth_of(g: &mut crate::game::Game, t: FixtureId) -> crate::fire::MouthOf {
    let first = g.tab.sim_lightshome_fixture[t].boxes.first().copied();
    let b = match first {
        Some(i) => static_at(g, i),
        None => None,
    };
    let mouth = g.k.fire.mouth;
    if let Some(b) = b {
        let (c, h) = (seam_box_c(g, b), seam_box_h(g, b));
        return crate::fire::MouthOf { pos: [c[0], c[1] + h[1], c[2]], r: m::min(h[0], h[2]) * mouth, lid: m::min(h[0], h[2]) };
    }
    let p = g.tab.sim_lightshome_fixture[t].pos;
    crate::fire::MouthOf { pos: [p[0], p[1] - 0.16, p[2]], r: 0.19 * mouth, lid: 0.0 }
}

/// How hot a fire burns, 0 to 1: its lamp's share of its authored strength.   (fire.ts heatOf)
pub fn heat_of(g: &mut crate::game::Game, t: FixtureId) -> f64 {
    m::max(0.0, m::min(1.0, crate::sim_lightshome::fixture_factor(g, t)))
}

/// Whether it burns at all.   (fire.ts burns)
pub fn burns(g: &mut crate::game::Game, t: FixtureId) -> bool {
    if !(heat_of(g, t) > 0.001) {
        return false;
    }
    let l = g.tab.sim_lightshome_fixture[t].light.unwrap();
    seam_light_enabled(g, l)
}

/// The wind at a fire's flame: as it blows, how hard, and what leans the flame (capped).   (fire.ts windAt)
pub fn wind_at(g: &mut crate::game::Game, mouth: V3) -> crate::fire::WindAt {
    let (height, wind_max) = (g.k.fire.height, g.k.fire.wind_max);
    let wv = crate::wind::wind_at(g, [mouth[0], mouth[1] + height * 0.6, mouth[2]], None);
    let wl = m::hyp2(wv[0], wv[2]);
    let wk = if wl > wind_max { wind_max / wl } else { 1.0 };
    crate::fire::WindAt { wv, wl, wind: [wv[0] * wk, wv[2] * wk] }
}

/// The look's numbers as the renderer takes them, fire.wgsl FireLook's sixteen in its order.   (fire.ts lookRow)
pub fn look_row(g: &mut crate::game::Game) -> Vec<f64> {
    let fk = &g.k.fire;
    let l = &fk.look;
    vec![l.gain, l.core, l.sigma, l.speed, l.tall, l.torn, l.plume, l.shimmer, l.soot, l.breathe, fk.gas.ref_, fk.gas.lo, l.react, l.lean, l.steps, 0.0]
}

/// Whether a round or a blast leaves `t` burning: to the fixture rules such a fire is no fixture at all.   (fire.ts fireHolds)
pub fn fire_holds(g: &mut crate::game::Game, t: FixtureId) -> bool {
    is_fire(g, t)
}

/// Did a round that stopped at `p`, in static `ix`, stop in this fire: its own box, or the drum under its mouth.   (fire.ts fireStruck)
pub fn fire_struck(g: &mut crate::game::Game, t: FixtureId, p: V3, ix: f64) -> bool {
    if g.tab.sim_lightshome_fixture[t].boxes.contains(&ix) {
        return true;
    }
    let mouth = mouth_of(g, t);
    let b = static_at(g, ix);
    let r = m::max(mouth.lid * 1.7, 0.3);
    let Some(b) = b else { return false };
    let c = seam_box_c(g, b);
    if m::hyp2(c[0] - mouth.pos[0], c[2] - mouth.pos[2]) > 0.05 || c[1] > mouth.pos[1] {
        return false;
    }
    p[1] <= mouth.pos[1] + 0.05 && m::hyp2(p[0] - mouth.pos[0], p[2] - mouth.pos[2]) <= r
}

/// A hit on a burning fire: a crack now and a throw of embers on its next frame, by the host's book.   (fire.ts fireStir)
pub fn fire_stir(g: &mut crate::game::Game, t: FixtureId) {
    let booked = matches!(&g.engine.host_books, Some(b) if b.fire.is_some());
    if !booked || !burns(g, t) {
        return;
    }
    let mut ix = 0usize;
    for i in 0..g.targets.len() {
        let x = g.targets[i];
        if x == t {
            break;
        }
        if is_fire(g, x) {
            ix += 1;
        }
    }
    let (now, gap) = (g.time, g.k.fire.stir_gap);
    let book = g.engine.host_books.as_mut().unwrap().fire.as_mut().unwrap();
    let at = book.stir_at.get_or_insert_with(Vec::new);
    if now - at.get(ix).copied().unwrap_or(-1e9) < gap {
        return;
    }
    // (the array's holes read as `?? -1e9`: a hole is written as that number)
    if at.len() <= ix {
        at.resize(ix + 1, -1e9);
    }
    at[ix] = now;
    book.stir.get_or_insert_with(Vec::new).push(ix as f64);
    let pos = mouth_of(g, t).pos;
    let volume = g.k.fire.crack * heat_of(g, t);
    seam_audio_play(g, "fireCrack", pos, volume, 0.7);
}

/// Out but still lit in the fog's picture: the remembered pool stays a fire's.   (fire.ts held)
pub fn held(g: &mut crate::game::Game, t: FixtureId) {
    let l = g.tab.sim_lightshome_fixture[t].light.unwrap();
    let tp = g.tab.sim_lightshome_fixture[t].pos;
    if !seam_steady_is_some(g, l) {
        let (radius, color, range) = (seam_light_radius(g, l), seam_light_color(g, l), seam_light_range(g, l));
        seam_steady_make(g, l, 0.0, [tp[0], tp[1], tp[2]], radius, color, range);
    }
    let c0 = match seam_steady_color(g, l) {
        Some(c) => c,
        None => seam_light_color(g, l),
    };
    let warm = |i: usize, g: &Game| g.k.fire.warm.get(i).copied().unwrap_or(1.0);
    let w = [warm(0, g), warm(1, g), warm(2, g)];
    let intensity = m::max(1e-4, crate::sim_lightshome::fixture_base_intensity(g, t) * g.k.fire.bright);
    seam_steady_set_intensity(g, l, intensity);
    let radius = m::max(seam_steady_radius(g, l), g.k.fire.size * mouth_of(g, t).r);
    seam_light_set_radius(g, l, radius);
    let lift = g.k.fire.lift;
    seam_light_set_pos(g, l, [tp[0], tp[1] + lift, tp[2]]);
    seam_light_set_color(g, l, [c0[0] * w[0], c0[1] * w[1], c0[2] * w[2]]);
    let range = match seam_steady_range(g, l) {
        Some(r) => r,
        None => seam_light_range(g, l),
    };
    let reach = g.k.fire.reach;
    seam_light_set_range(g, l, range * reach);
}

/// The lamp back as the level hung it, and nothing said to the steady readers.   (fire.ts plain)
pub fn plain(g: &mut crate::game::Game, t: FixtureId) {
    let l = g.tab.sim_lightshome_fixture[t].light.unwrap();
    let tp = g.tab.sim_lightshome_fixture[t].pos;
    seam_light_set_pos(g, l, [tp[0], tp[1], tp[2]]);
    if !seam_steady_is_some(g, l) {
        return;
    }
    let radius = seam_steady_radius(g, l);
    seam_light_set_radius(g, l, radius);
    if let Some(range) = seam_steady_range(g, l) {
        seam_light_set_range(g, l, range);
    }
    if let Some(c0) = seam_steady_color(g, l) {
        seam_light_set_color(g, l, [c0[0], c0[1], c0[2]]);
    }
    seam_steady_clear(g, l);
}

// ---- seams: what skeleton 3 still cannot reach (notes.md, ## Skeleton) ----

/// `b.c` of a box of the world (fire.ts:271, 295)
fn seam_box_c(g: &mut Game, b: BoxId) -> V3 { todo!("needs: Box.c of a BoxId") }
/// `b.h` of a box of the world (fire.ts:271)
fn seam_box_h(g: &mut Game, b: BoxId) -> V3 { todo!("needs: Box.h of a BoxId") }
/// `l.enabled` (fire.ts:277)
fn seam_light_enabled(g: &mut Game, l: LightId) -> bool { todo!("needs: RtLight.enabled of a LightId") }
/// `l.radius` (fire.ts:310)
fn seam_light_radius(g: &mut Game, l: LightId) -> f64 { todo!("needs: RtLight.radius of a LightId") }
/// `l.radius = x` (fire.ts:312, 316)
fn seam_light_set_radius(g: &mut Game, l: LightId, x: f64) { todo!("needs: RtLight.radius of a LightId") }
/// `l.color` (fire.ts:310, 311)
fn seam_light_color(g: &mut Game, l: LightId) -> V3 { todo!("needs: RtLight.color of a LightId") }
/// `l.color[0..3] = x` (fire.ts:313, 316)
fn seam_light_set_color(g: &mut Game, l: LightId, x: V3) { todo!("needs: RtLight.color of a LightId") }
/// `l.range` (fire.ts:310, 313)
fn seam_light_range(g: &mut Game, l: LightId) -> f64 { todo!("needs: RtLight.range of a LightId") }
/// `l.range = x` (fire.ts:313, 316)
fn seam_light_set_range(g: &mut Game, l: LightId, x: f64) { todo!("needs: RtLight.range of a LightId") }
/// `l.pos[0..3] = x` (fire.ts:312, 316)
fn seam_light_set_pos(g: &mut Game, l: LightId, x: V3) { todo!("needs: RtLight.pos of a LightId") }
/// `l.steady` is there (fire.ts:310, 316)
fn seam_steady_is_some(g: &mut Game, l: LightId) -> bool { todo!("needs: RtLight.steady of a LightId") }
/// `l.steady = { intensity, pos, radius, color, range }` (fire.ts:310)
fn seam_steady_make(g: &mut Game, l: LightId, intensity: f64, pos: V3, radius: f64, color: V3, range: f64) { todo!("needs: RtLight.steady of a LightId") }
/// `l.steady.intensity = x` (fire.ts:312)
fn seam_steady_set_intensity(g: &mut Game, l: LightId, x: f64) { todo!("needs: RtLight.steady of a LightId") }
/// `l.steady.radius` (fire.ts:312, 316)
fn seam_steady_radius(g: &mut Game, l: LightId) -> f64 { todo!("needs: RtLight.steady of a LightId") }
/// `l.steady.color` (fire.ts:311, 316)
fn seam_steady_color(g: &mut Game, l: LightId) -> Option<V3> { todo!("needs: RtLight.steady of a LightId") }
/// `l.steady.range` (fire.ts:313, 316)
fn seam_steady_range(g: &mut Game, l: LightId) -> Option<f64> { todo!("needs: RtLight.steady of a LightId") }
/// `l.steady = undefined` (fire.ts:316)
fn seam_steady_clear(g: &mut Game, l: LightId) { todo!("needs: RtLight.steady of a LightId") }
/// `g.audio.play(name, pos, volume, { rate })` (fire.ts:303): the skeleton's `opts` are `JsMap<String, Opaque>`, which cannot hold the rate
fn seam_audio_play(g: &mut Game, name: &str, pos: V3, volume: f64, rate: f64) { todo!("needs: GameAudioLike.play's opts to carry a number") }

#[cfg(test)]
mod tests {
    // (The arithmetic goes by the knob table through the private `*_with` helpers: fire_pops and the rest only lend them `g.k.fire`.)
    // The bits were printed by the TypeScript under bun with EXACT.on, from src/game/fire.ts of this tree.
    use super::*;
    use crate::knobs::page_fire;

    #[test]
    fn the_seed_and_h01_match_the_typescript() {
        let cases: [(f64, f64, u64); 5] = [
            (1.25, -3.5, 0x3feae7da80000000),
            (0.0, 0.0, 0x3fe3af6980000000),
            (12.34, 56.78, 0x3fefe5fda0000000),
            (-101.5, 7.25, 0x3fd59d4200000000),
            (1e6, -2e6, 0x3fcf8bb100000000),
        ];
        for (x, z, bits) in cases {
            assert_eq!(seed_at([x, 0.0, z]).to_bits(), bits, "{x} {z}");
        }
    }

    #[test]
    fn the_cracks_match_the_typescript() {
        let k = page_fire();
        let one = |seed, time, back| {
            let v = pops_with(&k, seed, time, back);
            assert_eq!(v.len(), 1);
            [v[0].at.to_bits(), v[0].hard.to_bits(), v[0].seed.to_bits()]
        };
        assert_eq!(one(0.3, 10.0, 1.5), [0x402249dd98666667, 0x3fe529cb4199999a, 0x3fe4d61aa0000000]);
        assert_eq!(one(0.77, 123.456, 0.48), [0x405edd1c8cc00000, 0x3fe5ad0b76666666, 0x3fddc94000000000]);
        assert_eq!(one(0.123, 3.0, 1.5), [0x40062414b999999a, 0x3fe00d9ccb333333, 0x3fdbcca500000000]);
        assert!(pops_with(&k, 0.5, 0.0, 1.0).is_empty());
    }

    #[test]
    fn the_flicker_matches_the_typescript() {
        let k = page_fire();
        let cases: [(f64, f64, u64); 5] = [
            (0.3, 10.0, 0x3ff035f24fcffb2c),
            (0.77, 123.456, 0x3ff86c2289bbfc56),
            (0.123, 3.14159, 0x3fef6965b539ab36),
            (0.9, 987.654, 0x3fec5b7473df9ec5),
            (0.5, 0.0, 0x3fe66072c62b87e0),
        ];
        for (seed, time, bits) in cases {
            assert_eq!(flicker_with(&k, seed, time).to_bits(), bits, "{seed} {time}");
        }
    }

    #[test]
    fn the_flame_axis_matches_the_typescript() {
        let k = page_fire();
        let cases: [(f64, [f64; 2], [u64; 4]); 3] = [
            (0.5, [1.5, -0.75], [0x3fb7c74a5a234cee, 0xbfa7c74a5a234cee, 0x3fd1d577c39a79b3, 0xbfc1d577c39a79b3]),
            (3.0, [2.5, 1.1], [0x3ff605fcfd04d500, 0x3fe3616dfd606985, 0x3fd58752997b9b3e, 0x3fc2f1f6c48140f0]),
            (-1.0, [1.0, 1.0], [0; 4]),
        ];
        for (up, wind, bits) in cases {
            assert_eq!(axis_with(&k, up, wind).map(f64::to_bits), bits, "{up}");
        }
    }

    #[test]
    fn a_new_row_of_counts_starts_at_the_infinities() {
        let s = fresh();
        assert_eq!((s.lo, s.hi), (m::INF, -m::INF));
    }

    #[test]
    fn a_write_through_state_of_lands_in_the_row_and_a_reader_makes_none() {
        let mut g = crate::game::Game::blank();
        // a reader gets a blank row and leaves none behind
        assert_eq!(fire_stats(&mut g).lo, m::INF);
        assert!(g.m.fire.states.is_none());
        state_of(&mut g).frames = 3.0;
        state_of(&mut g).cracks += 1.0;
        let s = fire_stats(&mut g);
        assert_eq!((s.frames, s.cracks, s.lo), (3.0, 1.0, m::INF));
    }
}
