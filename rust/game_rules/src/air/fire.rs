pub use ::game_types::fire::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, vec::Vec};

/// The flame's own middle `up` metres over its mouth: how far downwind (x, z) and its slope there.
fn axis_at(g: &crate::game::Game, up: f64, wind: [f64; 2]) -> [f64; 4] {
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

/// A new row of the counts: its least and most start at the infinities, not at a blank's zeros.
fn fresh() -> crate::fire::FireStats {
    crate::fire::FireStats { lo: m::INF, hi: -m::INF }
}

/// The game's row of counts, made if it has none: the row itself, a caller writes through it.
fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::fire::FireStats {
    g.m.fire.states.get_or_insert_with(fresh)
}

/// Integer hash to [0, 1). The TypeScript's signed and unsigned steps give these same 32 bits.
fn h01(x: f64) -> f64 {
    let mut x = (js::to_i32(x) as u32 ^ 0x2545F491).wrapping_mul(0x9E3779B1);
    x ^= x >> 15;
    x = x.wrapping_mul(0x85EBCA77);
    x ^= x >> 13;
    (x >> 8) as f64 / 16777216.0
}

/// Is this light target an open fire, by its name.
pub fn is_fire(g: &crate::game::Game, t: FixtureId) -> bool {
    let row = &g.tab.fixture[t];
    if row.kind != crate::fixtures::FixtureKind::Analytic || row.light.is_none() {
        return false;
    }
    for n in &g.k.fire.names {
        if row.name.starts_with(n.as_str()) {
            return true;
        }
    }
    false
}

/// A fire's own seed, 0 to 1, off where it stands.
pub fn fire_seed(g: &crate::game::Game, t: FixtureId) -> f64 {
    let p = g.tab.fixture[t].pos;
    seed_at(p)
}
fn seed_at(p: V3) -> f64 {
    h01(m::round(p[0] * 100.0) * 7919.0 + m::round(p[2] * 100.0) * 104729.0 + 17.0)
}

/// The cracks of the wood near `time`, the latest first, none older than `back` s.
fn fire_pops(g: &crate::game::Game, seed: f64, time: f64, back: f64) -> Vec<crate::fire::FirePopsItem> {
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

/// The light's flicker at `time`: what the authored strength is multiplied by.
pub fn fire_flicker(g: &crate::game::Game, seed: f64, time: f64) -> f64 {
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
fn static_at(g: &Game, ix: f64) -> Option<BoxId> {
    if !(ix >= 0.0) || m::floor(ix) != ix {
        return None;
    }
    g.engine.world.statics.get(ix as usize).copied()
}

/// Where the fire's mouth is and how wide: the glowing box hung on the lamp, or a hand under the lamp.
pub fn mouth_of(g: &crate::game::Game, t: FixtureId) -> crate::fire::MouthOf {
    let first = g.tab.fixture[t].boxes.first().copied();
    let b = match first {
        Some(i) => static_at(g, i),
        None => None,
    };
    let mouth = g.k.fire.mouth;
    if let Some(b) = b {
        let (c, h) = { let x = &g.tab.boxes[b]; (x.c, x.h) };
        return crate::fire::MouthOf { pos: [c[0], c[1] + h[1], c[2]], r: m::min(h[0], h[2]) * mouth, lid: m::min(h[0], h[2]) };
    }
    let p = g.tab.fixture[t].pos;
    crate::fire::MouthOf { pos: [p[0], p[1] - 0.16, p[2]], r: 0.19 * mouth, lid: 0.0 }
}

/// How hot a fire burns, 0 to 1: its lamp's share of its authored strength.
pub fn heat_of(g: &crate::game::Game, t: FixtureId) -> f64 {
    m::max(0.0, m::min(1.0, g.tab.fixture[t].factor))
}

/// Whether it burns at all.
pub fn burns(g: &crate::game::Game, t: FixtureId) -> bool {
    if !(heat_of(g, t) > 0.001) {
        return false;
    }
    let l = g.tab.fixture[t].light.unwrap();
    g.tab.light[l].enabled
}

/// The wind at a fire's flame: as it blows, how hard, and what leans the flame (capped).
fn wind_at(g: &crate::game::Game, mouth: V3) -> crate::fire::WindAt {
    let (height, wind_max) = (g.k.fire.height, g.k.fire.wind_max);
    let wv = crate::wind::wind_at(g, [mouth[0], mouth[1] + height * 0.6, mouth[2]], None);
    let wl = m::hyp2(wv[0], wv[2]);
    let wk = if wl > wind_max { wind_max / wl } else { 1.0 };
    crate::fire::WindAt { wv, wl, wind: [wv[0] * wk, wv[2] * wk] }
}

/// The same of the game as it stands, for whoever only reads it.
pub fn flame_wind(g: &crate::game::Game, mouth: V3) -> crate::fire::WindAt {
    let (height, wind_max) = (g.k.fire.height, g.k.fire.wind_max);
    let wv = crate::wind::wind_sum(g, [mouth[0], mouth[1] + height * 0.6, mouth[2]]);
    let wl = m::hyp2(wv[0], wv[2]);
    let wk = if wl > wind_max { wind_max / wl } else { 1.0 };
    crate::fire::WindAt { wv, wl, wind: [wv[0] * wk, wv[2] * wk] }
}

/// Whether a round or a blast leaves `t` burning: to the fixture rules such a fire is no fixture at all.
pub fn fire_holds(g: &crate::game::Game, t: FixtureId) -> bool {
    is_fire(g, t)
}

/// Did a round that stopped at `p`, in static `ix`, stop in this fire: its own box, or the drum under its mouth.
pub fn fire_struck(g: &crate::game::Game, t: FixtureId, p: V3, ix: f64) -> bool {
    if g.tab.fixture[t].boxes.contains(&ix) {
        return true;
    }
    let mouth = mouth_of(g, t);
    let b = static_at(g, ix);
    let r = m::max(mouth.lid * 1.7, 0.3);
    let Some(b) = b else { return false };
    let c = g.tab.boxes[b].c;
    if m::hyp2(c[0] - mouth.pos[0], c[2] - mouth.pos[2]) > 0.05 || c[1] > mouth.pos[1] {
        return false;
    }
    p[1] <= mouth.pos[1] + 0.05 && m::hyp2(p[0] - mouth.pos[0], p[2] - mouth.pos[2]) <= r
}

/// A hit on a burning fire: a crack now and a throw of embers on its next frame, by the host's book.
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
    crate::audio_iface::game_audio_like_play(g, "fireCrack", Some(pos), Some(volume), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.7 }));
}

/// Out but still lit in the fog's picture: the remembered pool stays a fire's.
fn held(g: &mut crate::game::Game, t: FixtureId) {
    let l = g.tab.fixture[t].light.unwrap();
    let tp = g.tab.fixture[t].pos;
    if g.tab.light[l].steady.is_none() {
        let lt = &mut g.tab.light[l];
        lt.steady = Some(crate::render_lights::RtLightSteady { intensity: 0.0, pos: [tp[0], tp[1], tp[2]], radius: lt.radius, color: Some(lt.color), range: Some(lt.range), out: None });
    }
    let (s_radius, s_color, s_range) = {
        let s = g.tab.light[l].steady.as_ref().unwrap();
        (s.radius, s.color, s.range)
    };
    let c0 = match s_color {
        Some(c) => c,
        None => g.tab.light[l].color,
    };
    let warm = |i: usize, g: &Game| g.k.fire.warm.get(i).copied().unwrap_or(1.0);
    let w = [warm(0, g), warm(1, g), warm(2, g)];
    let intensity = m::max(1e-4, g.tab.fixture[t].base_intensity * g.k.fire.bright);
    g.tab.light[l].steady.as_mut().unwrap().intensity = intensity;
    let radius = m::max(s_radius, g.k.fire.size * mouth_of(g, t).r);
    let (lift, reach) = (g.k.fire.lift, g.k.fire.reach);
    let lt = &mut g.tab.light[l];
    lt.radius = radius;
    lt.pos = [tp[0], tp[1] + lift, tp[2]];
    lt.color = [c0[0] * w[0], c0[1] * w[1], c0[2] * w[2]];
    let range = match s_range {
        Some(r) => r,
        None => lt.range,
    };
    lt.range = range * reach;
}

/// The lamp back as the level hung it, and nothing said to the steady readers.
fn plain(g: &mut crate::game::Game, t: FixtureId) {
    let l = g.tab.fixture[t].light.unwrap();
    let tp = g.tab.fixture[t].pos;
    let lt = &mut g.tab.light[l];
    lt.pos = [tp[0], tp[1], tp[2]];
    let Some(s) = lt.steady.take() else { return };
    lt.radius = s.radius;
    if let Some(range) = s.range {
        lt.range = range;
    }
    if let Some(c0) = s.color {
        lt.color = c0;
    }
}

// ---- the one seam: what skeleton 5 still cannot do (notes.md, ## Skeleton) ----


#[cfg(test)]
mod tests {
    // (The arithmetic goes by the knob table through the private `*_with` helpers: fire_pops and the rest only lend them `g.k.fire`.)
    // The bits were printed by the TypeScript under bun with EXACT.on, from src/game/fire.ts of this tree.
    use super::*;

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
        let k = crate::knobs::FireK::page();
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
        let k = crate::knobs::FireK::page();
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
        let k = crate::knobs::FireK::page();
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

    /// a lamp on a fixture, with no box hung on it
    fn stand() -> (crate::game::Game, FixtureId, LightId) {
        let mut g = crate::game::Game::blank();
        let l = g.tab.light.push(crate::render_lights::RtLight::default());
        let mut f = crate::fixtures::Fixture::blank();
        f.pos = [1.0, 2.0, 3.0];
        f.light = Some(l);
        let t = g.tab.fixture.push(f);
        (g, t, l)
    }

    #[test]
    fn with_no_box_the_mouth_is_a_hand_under_the_lamp() {
        let (mut g, t, _) = stand();
        let mouth = g.k.fire.mouth;
        let a = mouth_of(&mut g, t);
        assert_eq!((a.pos, a.r, a.lid), ([1.0, 2.0 - 0.16, 3.0], 0.19 * mouth, 0.0));
    }

    #[test]
    fn a_round_strikes_the_fire_in_its_own_box_and_in_no_static_it_is_not() {
        let (mut g, t, _) = stand();
        // past the list's end, a fractional index: no static, no fire
        assert!(!fire_struck(&mut g, t, [1.0, 2.0, 3.0], 9.0));
        assert!(!fire_struck(&mut g, t, [1.0, 2.0, 3.0], 0.5));
        // its own box, wherever the round stopped (decided by the index, no row read)
        g.tab.fixture[t].boxes.push(0.0);
        assert!(fire_struck(&mut g, t, [50.0, 50.0, 50.0], 0.0));
    }

    #[test]
    fn plain_puts_the_lamp_back_and_ends_the_steady_row() {
        let (mut g, t, l) = stand();
        let lt = &mut g.tab.light[l];
        (lt.pos, lt.radius, lt.range, lt.color) = ([9.0; 3], 9.0, 9.0, [9.0; 3]);
        // no steady row: only the place moves
        plain(&mut g, t);
        let lt = &mut g.tab.light[l];
        assert_eq!((lt.pos, lt.radius, lt.range, lt.color), ([1.0, 2.0, 3.0], 9.0, 9.0, [9.0; 3]));
        lt.steady = Some(crate::render_lights::RtLightSteady { intensity: 0.5, pos: [0.0; 3], radius: 1.5, color: Some([0.1, 0.2, 0.3]), range: Some(7.0), out: None });
        plain(&mut g, t);
        let lt = &g.tab.light[l];
        assert_eq!((lt.radius, lt.range, lt.color), (1.5, 7.0, [0.1, 0.2, 0.3]));
        assert!(lt.steady.is_none());
        // a row that names no range and no colour leaves the lamp's own
        let lt = &mut g.tab.light[l];
        lt.steady = Some(crate::render_lights::RtLightSteady { intensity: 0.5, pos: [0.0; 3], radius: 2.5, color: None, range: None, out: None });
        plain(&mut g, t);
        let lt = &g.tab.light[l];
        assert_eq!((lt.radius, lt.range, lt.color), (2.5, 7.0, [0.1, 0.2, 0.3]));
    }
}

// The lid's and the drum's ways read a Box row, so they wait for the table (seam_box). Never compiled: `g.tab.boxes` is the name other parts' notes give.
// To run them: drop the cfg when seam_box's body changes.
#[cfg(any())]
mod tests_need_box_rows {
    use super::*;

    /// a lamp on a fixture, its box the world's first static
    fn stand() -> (crate::game::Game, FixtureId) {
        let mut g = crate::game::Game::blank();
        let l = g.tab.light.push(crate::render_lights::RtLight::default());
        let mut f = crate::fixtures::Fixture::blank();
        f.pos = [1.0, 2.0, 3.0];
        f.light = Some(l);
        let mut b = crate::scene_boxes::Box::default();
        b.c = [1.0, 2.0, 3.0];
        b.h = [0.5, 0.1, 0.25];
        let id = g.tab.boxes.push(b);
        g.engine.world.statics.push(id);
        f.boxes.push(0.0);
        let t = g.tab.fixture.push(f);
        (g, t)
    }

    #[test]
    fn the_mouth_is_the_lid_of_its_box() {
        let (mut g, t) = stand();
        let mouth = g.k.fire.mouth;
        let a = mouth_of(&g, t);
        assert_eq!((a.pos, a.r, a.lid), ([1.0, 2.0 + 0.1, 3.0], 0.25 * mouth, 0.25));
    }

    #[test]
    fn a_round_strikes_the_drum_under_its_mouth() {
        let (mut g, t) = stand();
        let mut drum = crate::scene_boxes::Box::default();
        drum.c = [1.0, 1.0, 3.0];
        let id = g.tab.boxes.push(drum);
        g.engine.world.statics.push(id);
        assert!(fire_struck(&g, t, [1.2, 1.5, 3.0], 1.0));
        assert!(!fire_struck(&g, t, [1.0, 2.2, 3.0], 1.0)); // above the mouth
        assert!(!fire_struck(&g, t, [1.5, 1.5, 3.0], 1.0)); // outside the wall
        g.tab.boxes.edit(id).c = [2.0, 1.0, 3.0];
        assert!(!fire_struck(&g, t, [1.2, 1.5, 3.0], 1.0)); // off the axis
    }
}

/// A lamp's place across: where the level hung it, wandering by two sines.
fn wander_across(hung: f64, wander: f64, rate_a: f64, phase_a: f64, rate_b: f64, phase_b: f64, time: f64) -> f64 {
    hung + wander * m::sin(TAU * (rate_a * time + phase_a)) * m::sin(TAU * (rate_b * time + phase_b))
}
/// A lamp's height: in the body of the flame, up with the flicker by no more than the table says.
fn wander_height(hung: f64, lift: f64, wander_up: f64, flicker: f64, depth: f64) -> f64 {
    hung + lift + wander_up * m::max(-1.0, m::min(1.0, (flicker - 1.0) / m::max(depth, 1e-6)))
}
/// The coals' share of the authored glow, breathing with the light.
fn coal_share(factor: f64, coals: f64, coal_swing: f64, flicker: f64) -> f64 {
    factor * coals * (1.0 + coal_swing * (flicker - 1.0))
}

/// The host's book of the fires, on a frame that moves the clock (where it has just been made).
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

/// A throw of embers off one fire, by its own seed and not the particles' serial.
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
        ..crate::render_particles::EmitOpts::default()
    });
}

/// `n` embers off one fire, counted in the book.
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

/// Once a frame behind the lamps' paint: each fire's lamp flickered, moved and fed to its row, its coals, its cracks, and what the host shows and plays of it.
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
                n: 0.0, time: 0.0, stir: None, stir_at: None, smoke: Vec::new(),
                ember: Vec::new(), embers: 0.0,
            });
        }
        let p = books.fire.as_mut().unwrap();
        p.n += 1.0;
        p.time = time;
    }
    let mut fires = 0.0;
    let look = g.m.render_frame.look_config;
    let mut jets: Vec<crate::smoke_types::WindGpuSource> = Vec::new();
    let snow_lift = look.fire_snow > 0.0;
    let mut i = 0;
    while i < g.targets.len() {
        let t = g.targets[i];
        i += 1;
        if !is_fire(g, t) { continue; }
        let ix = fires as usize;
        fires += 1.0;
        let l = g.tab.fixture[t].light.unwrap();
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
            crate::fixtures::light_home_feed_burning(g, l);
            continue;
        }
        let seed = fire_seed(g, t);
        let fl = fire_flicker(g, seed, time);
        let mouth = mouth_of(g, t);
        let si = m::floor(seed * 65535.0);
        let hung = g.tab.fixture[t].pos;
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
        let intensity = g.tab.fixture[t].base_intensity * g.tab.fixture[t].factor * g.k.fire.bright;
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
        crate::fixtures::light_home_feed(g, l);
        // the coals, at this share of the glow updateTargets painted
        let ck = coal_share(g.tab.fixture[t].factor, g.k.fire.coals, g.k.fire.coal_swing, fl);
        let glow = g.tab.fixture[t].base_emissive;
        let mut b = 0;
        while b < g.tab.fixture[t].boxes.len() {
            let bi = g.tab.fixture[t].boxes[b];
            b += 1;
            if let Some(&bx) = g.engine.world.statics.get(bi as usize) {
                g.tab.boxes.edit(bx).emissive = [glow[0] * ck, glow[1] * ck, glow[2] * ck];
            }
        }
        if moved {
            let s = state_of(g);
            s.lo = m::min(s.lo, fl);
            s.hi = m::max(s.hi, fl);
        }
        let crate::fire::WindAt { wv, wl, wind } = wind_at(g, mouth.pos);
        // the embers: a steady few off the flame and a throw with each crack, of every fire on every host; the picture leaves out what is far from where it looks
        let (height, embers, ember_life, ember_crack) = (g.k.fire.height, g.k.fire.embers, g.k.fire.ember_life, g.k.fire.ember_crack);
        let flame_h = height * g.k.fire.look.tall;
        let px = moved && embers > 0.0;
        let fl_smoke = fire_look_smoke();
        if moved && snow_lift {
            // the falling snow: this fire's hot air
            let w = fl_smoke.snow;
            let a = axis_at(g, flame_h * 0.5, wind);
            let n = m::hyp3(a[2], 1.0, a[3]);
            jets.push(crate::smoke_types::WindGpuSource {
                kind: crate::wind::WindSourceKind::Jet,
                pos: mouth.pos,
                dir: [a[2] / n, 1.0 / n, a[3] / n],
                speed: w.speed * heat,
                reach: w.reach,
                tan_spread: w.spread,
                r0: mouth.r * w.mouth,
                d_max: w.top,
                bound: crate::smoke_types::WindGpuSourceBound { c: mouth.pos, r: w.top },
                heat: Some(true),
                ..crate::smoke_types::WindGpuSource::default()
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
                    let gain = g.k.fire.crack * (0.5 + 0.5 * p.hard) * heat;
                    let opts = crate::audio_iface::PlayOpts::Rate { rate: 0.8 + 0.5 * p.seed };
                    crate::audio_iface::game_audio_like_play(g, "fireCrack", Some(mouth.pos), Some(gain), Some(&opts));
                    if px && ember_crack > 0.0 {
                        ember_throw(g, &from, m::round(ember_crack * (0.4 + 0.6 * p.hard)), p.hard);
                    }
                }
            }
        }
        // the smoke: a short source on the solver, filed afresh every `every` s
        let b = g.k.fire.smoke;
        if moved {
            let slot = m::floor(time / b.every);
            let p = book(g);
            let n = p.n;
            let goes = p.smoke.get(ix).copied() == Some(n - 1.0);
            put_at(&mut p.smoke, ix, n, NO_FRAME);
            if slot != m::floor(was / b.every) || !goes {
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
                    lattice: Some(crate::smoke_types::SmokeLattice::Coarse),
                    look: Some(true), fire: Some(true),
                    tint: Some(l.soot),
                    ..crate::smoke_types::SmokeEmitter::default()
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
                    lattice: Some(crate::smoke_types::SmokeLattice::Coarse),
                    look: Some(true), fire: Some(true),
                    tint: Some(l.soot),
                    ..crate::smoke_types::SmokeEmitter::default()
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
    state_of(g);
}

#[cfg(test)]
mod tests_2 {
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
