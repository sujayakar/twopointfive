//! THE WEATHER THAT PLAY READS — src/game/weather.ts's rules over a table of its own, a part of the game's block: what is under a roof, the rain's bed at a place and
//! what it does to an ear (`hear_mask`), the fog and the dust along a line (`air_trans`), the storm's own die with what is thrown off it, thunder on its way, how wet
//! the lot is, and how wet a man is. Statement for statement. Who is asked and who draws it stays TypeScript: `sound.rs` and `smoke.rs` answer the box world and the
//! smoke and know nothing of weather, and the PRODUCT of their answer with this one is taken by whoever asks.
//!
//! WHAT IS NOT HERE, AND WHY: rain, snow and wet as they are drawn (rainFx.ts, snowFx.ts, wetGround.ts and the rest: dressing, reading this); the kinds' tables
//! (weatherKinds.ts: authored numbers handed in when a kind is set); the lot's soaked box (a Box of the host's, which a round's material still reads); a man's
//! place, head and coat (fields of TypeScript objects, handed over as a call's numbers until the men's columns land); the hour (hours.ts: last, on the lamps' table).
use crate::air::Air;
use crate::knobs::{k, AIRDUST_DIFFUSE, AIRDUST_RECHECK_SECS, AIRDUST_SETTLE, AIRDUST_STEP, SNOWBOG_BANK, SNOWBOG_OLD, SNOWBOG_PACKED, SNOWBOG_TOP, WET_BEAD_RUNOFF, WET_DRY_PER_SEC, WET_SOAK_PER_SEC, WX_RAIN_K, WX_ROOF_BED, WX_THUNDER_K, WX_THUNDER_LOUD, WX_THUNDER_SECS};
use crate::lines;
use crate::maths::{exp, hypot2, hypot3, pow};
use crate::state::state;
use crate::world::{clamp, js_abs, js_ceil, js_max, js_min};
use core::mem::size_of;

/// EVERY KNOB THE WEATHER'S RULES READ (here, air.rs, snow.rs): what a question asked outside a tick has handed over first (game/rustknobs.ts WX_KNOBS; tools/qa/weather-ident.ts reads
/// these files and is red on a `k(..)` that is not named here)
pub const READS: [usize; 16] = [AIRDUST_DIFFUSE, AIRDUST_RECHECK_SECS, AIRDUST_SETTLE, AIRDUST_STEP, SNOWBOG_BANK, SNOWBOG_OLD, SNOWBOG_PACKED, SNOWBOG_TOP, WET_BEAD_RUNOFF, WET_DRY_PER_SEC, WET_SOAK_PER_SEC, WX_RAIN_K, WX_ROOF_BED, WX_THUNDER_K, WX_THUNDER_LOUD, WX_THUNDER_SECS];

/// the roofs a table holds (roofsOf's four biggest: the level's own, or the office's two)
pub const ROOFS: usize = crate::items::MAX_ROOFS;
/// thunder on its way at once. A stroke comes at least four seconds after the last and its sound is at most thirteen on its way, so play has four at most; a
/// hand on the panel's button may ask more, and the oldest then goes unheard
pub const THUNDERS: usize = 16;
/// doubles a thunder: when it is due, how far off, the bearing, how loud
pub const TH: usize = 4;

#[repr(C)]
pub struct Weather {
    /// a kind is picked: the weather acts on the simulation (Weather.simLive)
    pub live: f64,
    pub storm: f64,
    pub rain_rate: f64,
    /// the rain falling on the game this tick (0 with the storm off or held for the day): what the frame draws, and what soaks a man
    pub rain_now: f64,
    /// how wet the lot is
    pub wet: f64,
    /// game time until which a thunderclap is drowning the small noises
    pub thunder_until: f64,
    pub next_flash_at: f64,
    pub last_bark_t: f64,
    pub fog_on: f64,
    pub fog_sigma: f64,
    pub fog_h: f64,
    pub fog_leak: f64,
    pub roofs_n: f64,
    pub roofs: [f64; ROOFS * 4],
    pub thunders_n: f64,
    pub thunders: [f64; THUNDERS * TH],
    /// the storm's own die: never the game's, whose stream a storm must not shift
    pub dice: u32,
    pub spare: u32,
    pub spare2: f64,
    pub air: Air,
}
// (a whole number of sixteens: the pools after it start on a 16-byte boundary, state.rs)
const _: () = assert!(size_of::<Weather>() == 8 * 13 + 8 * ROOFS * 4 + 8 + 8 * THUNDERS * TH + 8 + 8 + size_of::<Air>() && size_of::<Weather>() % 16 == 0);

/// the storm's numbers (weather.ts): seconds between strokes, the lot's soak, the speed of sound, a stroke's own peak
const SPEED_OF_SOUND: f64 = 343.0;
const FLASH_PEAK: f64 = 18.0;
const FLASH_MEAN_GAP: f64 = 11.0;
const WET_SECS: f64 = 20.0;
const ROOF_PAD: f64 = 0.06;
const PI: f64 = 3.141592653589793;

// ---- under a roof
/// Weather.underRoof over the footprints in force
#[inline]
pub fn under_roof(w: &Weather, x: f64, z: f64) -> bool {
    under(&w.roofs, w.roofs_n as usize, x, z)
}
/// …over any table of footprints (inclusive edges, as weatherKinds.ts inRect): the thrown things' walk hands its own over
#[inline]
pub fn under(roofs: &[f64], n: usize, x: f64, z: f64) -> bool {
    let mut i = 0;
    while i < n {
        let q = i * 4;
        if x >= roofs[q] && z >= roofs[q + 1] && x <= roofs[q + 2] && z <= roofs[q + 3] {
            return true;
        }
        i += 1;
    }
    false
}

/// A LEVEL'S ROOFS, AS THEY ARE MADE (roofsOf): the ceiling slabs among its boxes, biggest first, at most four. The host hands the boxes over one at a time; a slab
/// past the fourth biggest is no roof, which the sort-and-cut it was gave the same way (equal areas stay in the order they came).
pub struct RoofList {
    /// the slabs handed over that are roofs at all
    pub seen: usize,
    pub n: usize,
    pub r: [f64; ROOFS * 4],
    pub a: [f64; ROOFS],
}
static mut ROOF_LIST: RoofList = RoofList { seen: 0, n: 0, r: [0.0; ROOFS * 4], a: [0.0; ROOFS] };
fn roof_list() -> &'static mut RoofList {
    unsafe { &mut *core::ptr::addr_of_mut!(ROOF_LIST) }
}
pub fn roofs_begin() {
    let l = roof_list();
    l.n = 0;
    l.seen = 0;
}
/// one box: its centre and half size, and the ceiling's height
#[allow(clippy::too_many_arguments)]
pub fn roofs_box(cx: f64, cy: f64, cz: f64, hx: f64, hy: f64, hz: f64, ceiling: f64) {
    let (y0, y1) = (cy - hy, cy + hy);
    if !(y0 >= ceiling - 0.25 && y1 <= ceiling + 0.45) || hx < 0.45 || hz < 0.45 {
        return;
    }
    let r = [cx - hx - ROOF_PAD, cz - hz - ROOF_PAD, cx + hx + ROOF_PAD, cz + hz + ROOF_PAD];
    let area = (r[2] - r[0]) * (r[3] - r[1]);
    let l = roof_list();
    l.seen += 1;
    let mut at = l.n;
    while at > 0 && l.a[at - 1] < area {
        at -= 1;
    }
    if at >= ROOFS {
        return;
    }
    let last = if l.n < ROOFS { l.n } else { ROOFS - 1 };
    let mut i = last;
    while i > at {
        l.a[i] = l.a[i - 1];
        let mut c = 0;
        while c < 4 {
            l.r[i * 4 + c] = l.r[(i - 1) * 4 + c];
            c += 1;
        }
        i -= 1;
    }
    l.a[at] = area;
    let mut c = 0;
    while c < 4 {
        l.r[at * 4 + c] = r[c];
        c += 1;
    }
    if l.n < ROOFS {
        l.n += 1;
    }
}
/// how many slabs were roofs at all, table or no table (a level with more than the table holds is refused as it loads: levelfit.ts)
pub fn roofs_seen() -> usize {
    roof_list().seen
}
/// how many roofs the boxes made; they go to `out`
pub fn roofs_end(out: &mut [f64]) -> usize {
    let l = roof_list();
    out[..l.n * 4].copy_from_slice(&l.r[..l.n * 4]);
    l.n
}

// ---- what the weather answers to hearing
/// Weather.rainBed: the rain's level at a listener, what the mixer plays there and what drowns small noises
#[inline]
pub fn rain_bed(w: &Weather, x: f64, z: f64) -> f64 {
    if w.rain_rate > 0.001 {
        (0.35 + 0.65 * w.rain_rate) * (if under_roof(w, x, z) { k(WX_ROOF_BED) } else { 1.0 })
    } else {
        0.0
    }
}
/// Weather.hearMask: the factor (at least 1) a sound's path is stretched by for a listener at (x, z). A number of the ear alone
pub fn hear_mask(w: &Weather, time: f64, x: f64, z: f64) -> f64 {
    if w.live == 0.0 {
        return 1.0;
    }
    let mut m = 1.0 + k(WX_RAIN_K) * rain_bed(w, x, z);
    if time < w.thunder_until {
        m += k(WX_THUNDER_K) * (if under_roof(w, x, z) { 0.6 } else { 1.0 });
    }
    m
}

// ---- what the weather answers to sight
/// weatherKinds.ts fogTau: the fog's optical depth along a → b, before any roof
pub fn fog_tau(sigma: f64, h: f64, a: [f64; 3], b: [f64; 3]) -> f64 {
    let l = hypot3(b[0] - a[0], b[1] - a[1], b[2] - a[2]);
    if !(l > 0.0) {
        return 0.0;
    }
    if !(h > 0.0) {
        return sigma * l;
    }
    let ya = js_max(a[1], 0.0) / h;
    let ks = (js_max(b[1], 0.0) - js_max(a[1], 0.0)) / h;
    sigma * exp(-ya) * l * (if js_abs(ks) < 1e-3 { 1.0 - 0.5 * ks } else { (1.0 - exp(-ks)) / ks })
}
/// Weather.airTrans: the fog's transmittance along a → b, the share of it standing there taken along the line (the dust where it actually is while the field
/// moves, else the roof's leak at four points). Exactly 1 with no kind picked or no fog in it.
pub fn air_trans(w: &Weather, a: [f64; 3], b: [f64; 3]) -> f64 {
    if w.fog_on == 0.0 || w.live == 0.0 {
        return 1.0;
    }
    let (sigma, h) = (w.fog_sigma, w.fog_h);
    let mut leak = 0.0;
    if w.air.on != 0.0 {
        let n = js_min(24.0, js_max(4.0, js_ceil(hypot2(b[0] - a[0], b[2] - a[2]) / 0.75)));
        let mut i = 0.0;
        while i < n {
            let t = (i + 0.5) / n;
            leak += w.air.share_at(a[0] + (b[0] - a[0]) * t, a[2] + (b[2] - a[2]) * t);
            i += 1.0;
        }
        return exp(-fog_tau(sigma, h, a, b) * leak / n);
    }
    let mut i = 0.0;
    while i < 4.0 {
        let t = (i + 0.5) / 4.0;
        leak += if under_roof(w, a[0] + (b[0] - a[0]) * t, a[2] + (b[2] - a[2]) * t) { w.fog_leak } else { 1.0 };
        i += 1.0;
    }
    exp(-fog_tau(sigma, h, a, b) * leak / 4.0)
}

// ---- the storm's die, and what is thrown off it
/// weather.ts rng32's next: mulberry32 over the storm's own word
#[inline]
pub fn rnd(w: &mut Weather) -> f64 {
    let a = w.dice.wrapping_add(0x6D2B79F5);
    w.dice = a;
    let mut t = (a ^ (a >> 15)).wrapping_mul(1 | a);
    t = t.wrapping_add((t ^ (t >> 7)).wrapping_mul(61 | t)) ^ t;
    ((t ^ (t >> 14)) as f64) / 4294967296.0
}
/// Weather.gap: seconds to the next stroke (the first of a night comes sooner)
pub fn gap(w: &mut Weather, first: bool) -> f64 {
    if first {
        return 6.0 + 6.0 * rnd(w);
    }
    let mean = clamp(FLASH_MEAN_GAP / js_max(w.storm, 0.05), 8.0, 25.0);
    mean * (0.5 + rnd(w))
}
/// Weather.strike's dice and its thunder. What is given is a number, what is not is NaN and thrown for, in the order the rule has always thrown them. Into `out`:
/// bearing, elevation (rad), distance (km), peak, the second pulse's height, whether there is a third (0 / 1), and if so its start and height.
#[allow(clippy::too_many_arguments)]
pub fn strike(w: &mut Weather, time: f64, bearing: f64, elev_deg: f64, dist_km: f64, peak: f64, out: &mut [f64]) {
    let bearing = if bearing == bearing { bearing } else { rnd(w) * PI * 2.0 };
    let elev = (if elev_deg == elev_deg { elev_deg } else { 24.0 + rnd(w) * 30.0 }) * PI / 180.0;
    let dist = if dist_km == dist_km { dist_km } else { 0.3 + pow(rnd(w), 1.3) * 3.7 * (1.15 - 0.5 * w.storm) };
    let peak = if peak == peak { peak } else { FLASH_PEAK * (0.55 + 0.9 * rnd(w)) * clamp(1.25 - dist * 0.18, 0.45, 1.2) };
    let amp2 = 0.35 + 0.45 * rnd(w);
    let third = rnd(w) < 0.3;
    out[0] = bearing;
    out[1] = elev;
    out[2] = dist;
    out[3] = peak;
    out[4] = amp2;
    out[5] = if third { 1.0 } else { 0.0 };
    if third {
        out[6] = 0.2 + 0.05 * rnd(w);
        out[7] = 0.25 + 0.25 * rnd(w);
    }
    thunder_push(w, time + dist * 1000.0 / SPEED_OF_SOUND, dist, bearing, clamp(1.15 - dist * 0.22, 0.25, 1.0));
}
/// a thunder on its way (the oldest makes room where a hand has asked for more than the table holds)
pub fn thunder_push(w: &mut Weather, at: f64, dist: f64, bearing: f64, loud: f64) {
    let mut n = w.thunders_n as usize;
    if n >= THUNDERS {
        w.thunders.copy_within(TH..THUNDERS * TH, 0);
        n = THUNDERS - 1;
    }
    let q = n * TH;
    w.thunders[q] = at;
    w.thunders[q + 1] = dist;
    w.thunders[q + 2] = bearing;
    w.thunders[q + 3] = loud;
    w.thunders_n = (n + 1) as f64;
}
/// the storm's clock as a tick asks it (Weather.update): the first stroke's time set, and whether a stroke is due now. The host fires it (`strike`) and asks `next`.
pub fn stroke_due(w: &mut Weather, time: f64) -> bool {
    if w.next_flash_at < 0.0 {
        w.next_flash_at = time + gap(w, true);
    }
    w.storm > 0.01 && time >= w.next_flash_at
}
/// Weather.update's thunder and playThunder's dice, one thunder: is thunder `i` due. If it is it leaves the list, its clap may drown the small noises, and four
/// dice are thrown whoever is listening (the schedule after it never hangs on where the camera was). Into `out`: how far off, the bearing, how loud, the pitch's
/// die, the three of a remark, and when it was due.
pub fn thunder_due(w: &mut Weather, i: usize, time: f64, out: &mut [f64]) -> bool {
    let q = i * TH;
    if !(time >= w.thunders[q]) {
        return false;
    }
    let (at, dist, bearing, loud) = (w.thunders[q], w.thunders[q + 1], w.thunders[q + 2], w.thunders[q + 3]);
    let n = w.thunders_n as usize;
    w.thunders.copy_within((i + 1) * TH..n * TH, q);
    w.thunders_n = (n - 1) as f64;
    out[0] = dist;
    out[1] = bearing;
    out[2] = loud;
    out[3] = rnd(w);
    if w.live != 0.0 && loud > k(WX_THUNDER_LOUD) {
        w.thunder_until = time + k(WX_THUNDER_SECS);
    }
    out[4] = rnd(w);
    out[5] = rnd(w);
    out[6] = rnd(w);
    out[7] = at;
    true
}
/// weather.ts strokeMean: the mean of the stroke's envelope over [a, b], its `n` pulses (t0, amp, tau) in `pulses`
pub fn stroke_mean(pulses: &[f64], n: usize, a: f64, b: f64) -> f64 {
    if b <= a {
        let mut e = 0.0;
        for i in 0..n {
            let (t0, amp, tau) = (pulses[i * 3], pulses[i * 3 + 1], pulses[i * 3 + 2]);
            if a >= t0 {
                e += amp * exp(-(a - t0) / tau);
            }
        }
        return e;
    }
    let mut e = 0.0;
    for i in 0..n {
        let (t0, amp, tau) = (pulses[i * 3], pulses[i * 3 + 1], pulses[i * 3 + 2]);
        let lo = js_max(a, t0);
        if b <= lo {
            continue;
        }
        e += amp * tau * (exp(-(lo - t0) / tau) - exp(-(b - t0) / tau));
    }
    e / (b - a)
}
/// Weather.update's lot: how wet it is now, easing toward the rain. → is it wet enough to be soaked
pub fn lot_step(w: &mut Weather, dt: f64) -> bool {
    let raining = w.rain_rate > 0.001;
    w.wet = clamp(w.wet + (if raining { dt } else { -dt }) / WET_SECS, 0.0, 1.0);
    w.wet > 0.02
}

// ---- how wet a man is
const SKY_TOP: f64 = 9.0;
const SKY_RING: f64 = 0.35;
const WET_SKY_SECS: f64 = 0.25;
const SKY_OFFSETS: [[f64; 2]; 5] = [[0.0, 0.0], [SKY_RING, 0.0], [-SKY_RING, 0.0], [0.0, SKY_RING], [0.0, -SKY_RING]];

/// wet.ts openSky: is a man under the open sky: outside every footprint, and most of what is over him and a forearm round him is sky. The answer holds a quarter of
/// a second a man (`sky_t`, `open`: his, handed in and back). `head`: how high his head is, as the host reads it. → (open, this call asked the rays)
#[allow(clippy::too_many_arguments)]
pub fn open_sky(wx: &Weather, wo: usize, set: i32, ask: u32, time: f64, x: f64, z: f64, head: f64, sky_t: &mut f64, open: &mut f64) -> (bool, bool) {
    if under_roof(wx, x, z) {
        return (false, false);
    }
    let mut asked = false;
    if time - *sky_t >= WET_SKY_SECS || *sky_t < 0.0 || time < *sky_t {
        *sky_t = time;
        let mut covered = 0;
        for o in SKY_OFFSETS.iter() {
            if lines::solid_line(wo, set, ask, x + o[0], head + 0.15, z + o[1], x + o[0], SKY_TOP, z + o[1]) == Some(true) {
                covered += 1;
            }
        }
        *open = if covered < 3 { 1.0 } else { 0.0 };
        asked = true;
    }
    (*open != 0.0, asked)
}
/// the numbers of one man's coat, in and out of `wet_step`: wet, beads, the rate the drops on him came at, how far up him it reaches, and his sky's memory
pub const M_WET: usize = 0;
pub const M_BEADS: usize = 1;
pub const M_RAIN: usize = 2;
pub const M_LINE: usize = 3;
pub const M_SKY_T: usize = 4;
pub const M_OPEN: usize = 5;
/// …out: this step asked the rays
pub const M_ASKED: usize = 6;
pub const M_LEN: usize = 7;
/// wet.ts updateWetness, one man. `m` is his coat as above.
#[allow(clippy::too_many_arguments)]
pub fn wet_step(wx: &Weather, wo: usize, set: i32, ask: u32, time: f64, dt: f64, x: f64, z: f64, head: f64, m: &mut [f64]) {
    let rain = js_max(0.0, wx.rain_now);
    let (mut sky_t, mut open) = (m[M_SKY_T], m[M_OPEN]);
    let mut asked = false;
    let wetting = rain > 0.001 && {
        let (o, a) = open_sky(wx, wo, set, ask, time, x, z, head, &mut sky_t, &mut open);
        asked = a;
        o
    };
    m[M_SKY_T] = sky_t;
    m[M_OPEN] = open;
    m[M_ASKED] = if asked { 1.0 } else { 0.0 };
    let (mut wet, mut beads, mut rate, mut line) = (m[M_WET], m[M_BEADS], m[M_RAIN], m[M_LINE]);
    let runoff = js_max(k(WET_BEAD_RUNOFF), 0.1);
    if wetting {
        line = f64::INFINITY;
        let soak = k(WET_SOAK_PER_SEC);
        wet = clamp(wet + soak * (0.4 + 0.6 * rain) * dt, 0.0, 1.0);
        let beads_to = clamp(0.35 + 0.65 * rain, 0.0, 1.0);
        beads = if beads < beads_to { js_min(beads_to, beads + 3.0 * soak * (0.4 + 0.6 * rain) * dt) } else { js_max(beads_to, beads - dt / runoff) };
        rate = rate + (rain - rate) * js_min(1.0, dt * 2.0);
    } else {
        wet = clamp(wet - k(WET_DRY_PER_SEC) * dt, 0.0, 1.0);
        beads = clamp(beads - dt / runoff, 0.0, 1.0);
    }
    if wet <= 0.0 && beads <= 0.0 {
        rate = 0.0;
        line = f64::INFINITY;
    }
    m[M_WET] = wet;
    m[M_BEADS] = beads;
    m[M_RAIN] = rate;
    m[M_LINE] = line;
}
// ---- the C face's helpers: the block of slot `g`
#[inline(always)]
pub fn of(g: usize) -> &'static mut Weather {
    &mut state(g).weather
}
/// the columns the host reads and writes in place, by name (`weather_col`)
pub fn col(i: u32) -> usize {
    use core::mem::offset_of;
    match i {
        0 => offset_of!(Weather, live),
        1 => offset_of!(Weather, storm),
        2 => offset_of!(Weather, rain_rate),
        3 => offset_of!(Weather, rain_now),
        4 => offset_of!(Weather, wet),
        5 => offset_of!(Weather, thunder_until),
        6 => offset_of!(Weather, next_flash_at),
        7 => offset_of!(Weather, last_bark_t),
        8 => offset_of!(Weather, fog_on),
        9 => offset_of!(Weather, fog_sigma),
        10 => offset_of!(Weather, fog_h),
        11 => offset_of!(Weather, fog_leak),
        12 => offset_of!(Weather, roofs_n),
        13 => offset_of!(Weather, roofs),
        14 => offset_of!(Weather, thunders_n),
        15 => offset_of!(Weather, thunders),
        16 => offset_of!(Weather, dice),
        20 => offset_of!(Weather, air) + offset_of!(Air, on),
        21 => offset_of!(Weather, air) + offset_of!(Air, leak),
        22 => offset_of!(Weather, air) + offset_of!(Air, acc),
        23 => offset_of!(Weather, air) + offset_of!(Air, cur),
        24 => offset_of!(Weather, air) + offset_of!(Air, n_cells),
        25 => offset_of!(Weather, air) + offset_of!(Air, n_edges),
        26 => offset_of!(Weather, air) + offset_of!(Air, s),
        30 => offset_of!(Weather, air) + offset_of!(Air, stat),
        _ => 0,
    }
}
