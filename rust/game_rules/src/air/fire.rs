pub use ::game_types::fire::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js;
use crate::m;
use crate::v3::{self, V3};
use alloc::vec::Vec;

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

/// Integer hash to [0, 1).
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
    if v3::dist_xz(c, mouth.pos) > 0.05 || c[1] > mouth.pos[1] {
        return false;
    }
    p[1] <= mouth.pos[1] + 0.05 && v3::dist_xz(p, mouth.pos) <= r
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
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::FireCrack, Some(pos), Some(volume), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.7 }));
}

/// The steady row of a fire's lamp: what it is as the level hung it. Made the first time the lamp is a fire's.
fn steady_of(g: &mut Game, t: FixtureId) -> &mut crate::render_lights::RtLightSteady {
    let pos = g.tab.fixture[t].pos;
    let lamp = &mut g.tab.light[g.tab.fixture[t].light.unwrap()];
    let (radius, color, range) = (lamp.radius, lamp.color, lamp.range);
    lamp.steady.get_or_insert(crate::render_lights::RtLightSteady { intensity: 0.0, pos, radius, color: Some(color), range: Some(range), out: None })
}

/// Its lamp as a fire's: the width, the warmth and the reach of one, and for whoever asks after the place's steady light, how bright it is and whether it is out.
fn as_fire(g: &mut Game, t: FixtureId, intensity: f64, out: bool) {
    let s = steady_of(g, t);
    s.intensity = intensity;
    s.out = Some(out);
    let s = *s;
    let radius = m::max(s.radius, g.k.fire.size * mouth_of(g, t).r);
    let warm = [0, 1, 2].map(|n| g.k.fire.warm.get(n).copied().unwrap_or(1.0));
    let reach = g.k.fire.reach;
    let lamp = &mut g.tab.light[g.tab.fixture[t].light.unwrap()];
    let c = s.color.unwrap_or(lamp.color);
    lamp.radius = radius;
    lamp.color = [c[0] * warm[0], c[1] * warm[1], c[2] * warm[2]];
    lamp.range = s.range.unwrap_or(lamp.range) * reach;
}

/// A fire that is out, each frame of it. While the fog's picture still has it lit, and for the one frame before the fog has had its say, it is held; else its lamp is back as the level hung it.
fn out(g: &mut Game, t: FixtureId) {
    let l = g.tab.fixture[t].light.unwrap();
    let ghost = g.tab.light[l].fog_ghost.unwrap_or(0.0) > 0.0;
    let first = g.tab.light[l].steady.is_some_and(|st| !st.out.unwrap_or(false));
    if ghost || first { held(g, t); } else { plain(g, t); }
    crate::fixtures::light_home_feed_burning(g, l);
}

/// Held: the pool the picture remembers stays a fire's.
fn held(g: &mut crate::game::Game, t: FixtureId) {
    let intensity = m::max(1e-4, g.tab.fixture[t].base_intensity * g.k.fire.bright);
    as_fire(g, t, intensity, true);
    let p = g.tab.fixture[t].pos;
    let lift = g.k.fire.lift;
    g.tab.light[g.tab.fixture[t].light.unwrap()].pos = [p[0], p[1] + lift, p[2]];
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

#[cfg(test)]
mod tests {
    // (The arithmetic goes by the knob table through the private `*_with` helpers: fire_pops and the rest only lend them `g.k.fire`.)
    use super::*;

    #[test]
    fn the_seed_and_h01_are_as_pinned() {
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
    fn the_cracks_are_as_pinned() {
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
    fn the_flicker_is_as_pinned() {
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
    fn the_flame_axis_is_as_pinned() {
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

/// The frame's clock.
struct Clock {
    time: f64,
    was: f64,
    dt: f64,
}

/// One fire burning this frame.
struct Blaze {
    t: FixtureId,
    /// Which of the level's fires it is.
    ix: usize,
    seed: f64,
    si: f64,
    heat: f64,
    flicker: f64,
    mouth: MouthOf,
    wind: [f64; 2],
    flame_h: f64,
}

/// `g.smoke.emit`: the real body, then the event for the host's solver.
fn smoke_emit(g: &mut Game, e: &crate::smoke_types::SmokeEmitter) {
    crate::smoke_types::smoke_system_like_emit(g, e);
    crate::out_events::push_smoke_system_like_emit(g, e);
}

/// Once a frame behind the lamps' paint: each fire's lamp flickered, moved and fed to its row, its coals, its cracks, and what the host shows and plays of it.
pub fn update_fire(g: &mut Game, dt: f64) {
    let fires: Vec<FixtureId> = g.targets.iter().copied().filter(|t| is_fire(g, *t)).collect();
    if fires.is_empty() { return; }
    state_of(g);   // (the row made)
    let clock = Clock { time: g.time, was: g.time - m::max(dt, 0.0), dt };
    let moved = dt > 0.0;
    if moved { opens_book(g, &clock); }
    let snow = g.m.render_frame.look_config.fire_snow > 0.0;
    let mut jets: Vec<crate::smoke_types::WindGpuSource> = Vec::new();
    for (ix, &t) in fires.iter().enumerate() {
        if !burns(g, t) {
            out(g, t);
            continue;
        }
        let b = Blaze::of(g, ix, t, clock.time);
        b.lamp(g, clock.time);
        b.coals(g);
        if !moved { continue; }
        let s = state_of(g);
        s.lo = m::min(s.lo, b.flicker);
        s.hi = m::max(s.hi, b.flicker);
        if snow { jets.push(b.hot_air(g)); }
        b.embers(g, &clock);
        b.cracks(g, &clock);
        b.smoke(g, &clock);
    }
    if !moved { return; }
    if let Some(stir) = book(g).stir.as_mut() { stir.clear(); }
    crate::render_particles::particles_set_heat(g, &jets);
}

/// The host's book, which fire_stir reads, on to this frame: a fresh one when there is none or the clock went back on it.
fn opens_book(g: &mut Game, clock: &Clock) {
    let books = g.engine.host_books.get_or_insert_with(crate::host_api::HostBooks::default);
    if books.fire.as_ref().map_or(true, |p| clock.was < p.time - 1e-9) {
        books.fire = Some(FireShown { n: 0.0, time: 0.0, stir: None, stir_at: None, smoke: Vec::new(), ember: Vec::new(), embers: 0.0 });
    }
    let p = books.fire.as_mut().unwrap();
    p.n += 1.0;
    p.time = clock.time;
}

impl Blaze {
    fn of(g: &Game, ix: usize, t: FixtureId, time: f64) -> Blaze {
        let seed = fire_seed(g, t);
        let mouth = mouth_of(g, t);
        Blaze { t, ix, seed, si: m::floor(seed * 65535.0), heat: heat_of(g, t), flicker: fire_flicker(g, seed, time), mouth, wind: flame_wind(g, mouth.pos).wind, flame_h: g.k.fire.height * g.k.fire.look.tall }
    }

    /// The light: the authored lamp x the flicker, its place wandering with the flames; a reader of the place's steady light takes it for the authored lamp.
    fn lamp(&self, g: &mut Game, time: f64) {
        let f = &g.tab.fixture[self.t];
        let (l, hung, intensity) = (f.light.unwrap(), f.pos, f.base_intensity * f.factor * g.k.fire.bright);
        as_fire(g, self.t, intensity, false);
        let (k, si) = (&g.k.fire, self.si);
        let x = wander_across(hung[0], k.wander, 0.83, h01(si + 5.0), 2.9, h01(si + 6.0), time);
        let z = wander_across(hung[2], k.wander, 0.71, h01(si + 7.0), 3.3, h01(si + 8.0), time);
        let y = wander_height(hung[1], k.lift, k.wander_up, self.flicker, k.depth);
        g.tab.light[l].intensity = intensity * self.flicker;
        g.tab.light[l].pos = [x, y, z];
        crate::fixtures::light_home_feed(g, l);
    }

    /// The coals, at this share of the glow updateTargets painted.
    fn coals(&self, g: &mut Game) {
        let f = &g.tab.fixture[self.t];
        let share = coal_share(f.factor, g.k.fire.coals, g.k.fire.coal_swing, self.flicker);
        let glow = f.base_emissive;
        for bi in f.boxes.clone() {
            if let Some(&b) = g.engine.world.statics.get(bi as usize) { g.tab.boxes.edit(b).emissive = [glow[0] * share, glow[1] * share, glow[2] * share]; }
        }
    }

    /// Its hot air, as the falling snow meets it.
    fn hot_air(&self, g: &Game) -> crate::smoke_types::WindGpuSource {
        let w = g.k.fire.plume.snow;
        let a = axis_at(g, self.flame_h * 0.5, self.wind);
        let n = m::hyp3(a[2], 1.0, a[3]);
        crate::smoke_types::WindGpuSource {
            kind: crate::wind::WindSourceKind::Jet,
            pos: self.mouth.pos,
            dir: [a[2] / n, 1.0 / n, a[3] / n],
            speed: w.speed * self.heat,
            reach: w.reach,
            tan_spread: w.spread,
            r0: self.mouth.r * w.mouth,
            d_max: w.top,
            bound: crate::smoke_types::WindGpuSourceBound { c: self.mouth.pos, r: w.top },
            heat: Some(true),
            ..crate::smoke_types::WindGpuSource::default()
        }
    }

    /// The embers: a steady few off the flame and a throw when it is stirred, of every fire on every host; the picture leaves out what is far from where it looks.
    fn embers(&self, g: &mut Game, clock: &Clock) {
        let (embers, life, crack) = (g.k.fire.embers, g.k.fire.ember_life, g.k.fire.ember_crack);
        if !(embers > 0.0) { return; }
        let stirred = book(g).stir.as_ref().is_some_and(|v| v.contains(&(self.ix as f64)));
        if stirred && crack > 0.0 { self.throw(g, m::round(crack * 2.0), 1.0); }
        let owe = book(g).ember.get(self.ix).copied().unwrap_or(0.0) + embers / m::max(life, 0.1) * self.heat * clock.dt;
        let n = m::floor(owe);
        put_at(&mut book(g).ember, self.ix, owe - n, 0.0);
        if n > 0.0 { self.throw(g, n, 0.0); }
    }

    /// The cracks that fell in this frame, for the ear and (a throw of embers) for the eye.
    fn cracks(&self, g: &mut Game, clock: &Clock) {
        let crack = g.k.fire.ember_crack;
        for p in fire_pops(g, self.seed, clock.time, 1.5) {
            if !(p.at > clock.was && p.at <= clock.time) { continue; }
            let gain = g.k.fire.crack * (0.5 + 0.5 * p.hard) * self.heat;
            let opts = crate::audio_iface::PlayOpts::Rate { rate: 0.8 + 0.5 * p.seed };
            crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::FireCrack, Some(self.mouth.pos), Some(gain), Some(&opts));
            if g.k.fire.embers > 0.0 && crack > 0.0 { self.throw(g, m::round(crack * (0.4 + 0.6 * p.hard)), p.hard); }
        }
    }

    /// `n` embers off it, counted in the book, by its own seed and not the particles' serial.
    fn throw(&self, g: &mut Game, n: f64, hard: f64) {
        let fl = g.k.fire.plume;
        let nth = book(g).embers;
        let k = &g.k.fire;
        let up = self.flame_h * fl.ember_up;
        let a = axis_at(g, up, self.wind);
        let p = self.mouth.pos;
        let seed = (m::imul(js::to_i32(self.si + 1.0), 0x9E3779B1u32 as i32) ^ m::imul(js::to_i32(nth + 1.0), 0x85EBCA77u32 as i32)) as u32 as f64;
        let o = crate::render_particles::EmitOpts {
            shape: Some(crate::render_particles::EmitShape::Cone),
            spread: Some(fl.ember_spread + 0.6 * hard),
            speed: k.ember_speed * fl.ember_speed * (1.0 + 1.6 * hard),
            speed_jit: Some(0.6),
            life: k.ember_life * (1.0 - 0.45 * hard),
            life_jit: Some(0.45),
            size: k.ember_size,
            size_jit: Some(0.4),
            pos_jit: Some(self.mouth.r * fl.ember_jit),
            ember: Some(m::min(0.99, k.ember_gain * (0.5 + 0.5 * self.heat))),
            seed: Some(seed),
            ..crate::render_particles::EmitOpts::default()
        };
        crate::render_particles::particles_emit(g, crate::render_particles::PKind::Spark, [p[0] + a[0], p[1] + up, p[2] + a[1]], [a[2], 1.0, a[3]], n, o);
        book(g).embers += n;
    }

    /// The smoke: a short source on the solver, filed afresh every `every` s, and the puffs, a short burst of thicker, hotter gas a few a second.
    fn smoke(&self, g: &mut Game, clock: &Clock) {
        let p = book(g);
        let n = p.n;
        let goes_on = p.smoke.get(self.ix).copied() == Some(n - 1.0);
        put_at(&mut p.smoke, self.ix, n, NO_FRAME);
        let due = |every: f64| !goes_on || m::floor(clock.time / every) != m::floor(clock.was / every);
        if due(g.k.fire.smoke.every) {
            let e = self.gas(g);
            smoke_emit(g, &e);
        }
        let every = g.k.fire.plume.puff_every;
        if due(every) {
            let e = self.puff(g, m::floor(clock.time / every));
            smoke_emit(g, &e);
        }
    }

    /// The steady source.
    fn gas(&self, g: &Game) -> crate::smoke_types::SmokeEmitter {
        let (b, l) = (&g.k.fire.smoke, g.k.fire.plume);
        let p = self.mouth.pos;
        let up = g.k.fire.height * l.up;
        let a = axis_at(g, up, self.wind);
        crate::smoke_types::SmokeEmitter {
            pos: [p[0] + a[0], p[1] + up, p[2] + a[1]],
            dir: [a[2], 1.0, a[3]],
            speed: b.speed,
            radius: l.radius,
            density: b.density * self.heat * l.density,
            temperature: b.temp * self.heat,
            ttl: b.every,
            push: Some(b.push),
            jitter: Some(b.jitter),
            ..soot(b.cells, l.soot)
        }
    }

    /// The `nth` puff.
    fn puff(&self, g: &Game, nth: f64) -> crate::smoke_types::SmokeEmitter {
        let l = g.k.fire.plume;
        let luck = |j: f64| h01(self.si + nth * 31.0 + j);
        let ox = (luck(11.0) - 0.5) * 2.0 * l.puff_off;
        let oz = (luck(12.0) - 0.5) * 2.0 * l.puff_off;
        let k = 0.7 + 0.6 * luck(13.0);
        let p = self.mouth.pos;
        let up = g.k.fire.height * l.puff_up;
        let a = axis_at(g, up, self.wind);
        crate::smoke_types::SmokeEmitter {
            pos: [p[0] + ox + a[0], p[1] + up, p[2] + oz + a[1]],
            dir: [a[2] + ox, 1.0, a[3] + oz],
            speed: l.puff_speed * k,
            radius: l.puff_radius,
            density: l.puff_density * self.heat * k,
            temperature: l.puff_temp * self.heat * k,
            ttl: l.puff_secs,
            push: Some(l.puff_push),
            expand: Some(l.puff_expand),
            attack: Some(0.06),
            jitter: Some(0.9),
            ..soot(g.k.fire.smoke.cells, l.soot)
        }
    }
}

/// What both its sources are: a cloud on the coarse grid, the picture's, a fire's.
fn soot(cells: f64, tint: V3) -> crate::smoke_types::SmokeEmitter {
    crate::smoke_types::SmokeEmitter {
        min_voxels: Some(cells),
        age: 0.0,
        kind: crate::smoke_types::SmokeEmitterKind::Cloud,
        lattice: Some(crate::smoke_types::SmokeLattice::Coarse),
        look: Some(true),
        fire: Some(true),
        tint: Some(tint),
        ..crate::smoke_types::SmokeEmitter::default()
    }
}

#[cfg(test)]
mod tests_2 {
    use super::*;
    // The pinned numbers, at the table's wander 0.14, wander_up 0.1, lift 0.5, depth 0.4, coals 0.26, coal_swing 0.6.
    #[test]
    fn lamp_wanders_across_on_the_pinned_bits() {
        assert_eq!(wander_across(3.25, 0.14, 0.83, 0.1937, 2.9, 0.7711, 123.456).to_bits(), 0x400aeb1a98f0afd6);
        assert_eq!(wander_across(-7.5, 0.14, 0.71, 0.5021, 3.3, 0.0313, 987.654321).to_bits(), 0xc01e8a3e54d176b0);
    }
    #[test]
    fn lamp_height_is_capped_at_the_swing_on_the_pinned_bits() {
        assert_eq!(wander_height(1.13, 0.5, 0.1, 1.07, 0.4).to_bits(), 0x3ffa5c28f5c28f5c);
        assert_eq!(wander_height(1.13, 0.5, 0.1, 1.6, 0.4).to_bits(), 0x3ffbae147ae147ae);   // a crack's kick, past the swing
        assert_eq!(wander_height(1.13, 0.5, 0.1, 0.6, 0.4).to_bits(), 0x3ff87ae147ae147a);
        assert_eq!(wander_height(1.13, 0.5, 0.1, 0.81, 0.4).to_bits(), 0x3ff951eb851eb852);
    }
    #[test]
    fn coals_breathe_on_the_pinned_bits() {
        assert_eq!(coal_share(1.0, 0.26, 0.6, 1.31).to_bits(), 0x3fd3bc2b94d94079);
        assert_eq!(coal_share(0.37, 0.26, 0.6, 0.83).to_bits(), 0x3fb61d7f6cc7a978);
    }
}
