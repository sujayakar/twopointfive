//! THE LIGHT UNDER A ROOF THAT IS NO LAMP'S AND NOT THE SUN'S OWN BEAM: the sky and the ground a room sees through its windows, its doors and the holes in its
//! walls, and all the light that comes round off its surfaces, the lamps', the sky's and the sun's where it falls inside. Under the open sky the hour has
//! its own sums (game_rules hours) and nothing here is asked.
//!
//! IT IS LIGHT AND NOT A NUMBER TUNED: from the middle of every square metre under a roof, at a standing man's height, RAYS lines go out every way alike. What a line
//! meets is what it brings: a surface (its own colour × the light in the metre before it, and the sun's where the sun stands on it), or, out by an
//! opening, the sky of the hour or the ground out there. A metre keeps the best of the meter's six faces, their mean and the face that looks up, and one more,
//! what leaves its own surfaces (the mean, with the lamps'), which is what the other metres' lines read. So light goes from room to room by the ways light goes,
//! and a step (a tenth of a second, after the smoke's) works a slice of the metres out again: a SWEEP is the whole plan, and every sweep is one more
//! bounce. Where the lamps are not those of the last step the sweep goes QUICK steps to a round for ROUNDS rounds, so the light off the walls
//! follows its lamp within a few seconds, most of it in the first; a new field is swept FIRST times whole in its first step.
//!
//! HOW LIGHT A PLACE IS, for eyes that are used to it: the light on a face that looks up, smoothed over REACH metres and never through a wall, a pane or a doorway, so that a
//! room has one level and the yard outside its door another (the picture's own place field, render/shaders/place_field.wgsl, by its own numbers).
//!
//! WHAT IS STATE: the five numbers a metre, the lamps' word, where the sweep is (bricks.rs O_GLOW_…: a part of the game's block). WHAT IS MADE AGAIN from the
//! box world as it stands, a table a world: which half metres have a roof, where each metre's lines end among the statics, and which metres are one place. A static that is
//! written makes them again round it (`fresh` compares the rows themselves, so who wrote them, and how often, is nothing to it): what a sweep works out
//! is of the statics and the leaves as they are when it comes by, whether the table was kept or made that moment. The leaves are asked at the sweep.
//!
//! WHAT IT LEAVES OUT, named: a lamp somebody carries (it moves faster than a sweep), a flash, the smoke, a lamp under the open sky seen from inside.

use crate::bricks::{LEN, MAX_GLOW, O_GLOW_A, O_GLOW_AT, O_GLOW_LAMPS, O_GLOW_LEFT, O_GLOW_MAX, O_GLOW_MEAN, O_GLOW_PLACE, O_GLOW_UP, O_RHO, SIDE};
use crate::light;
use crate::lines;
use crate::maths::{cos, exp, log, sin};
use crate::world::{js_max, js_min, world_read, World, ASK_HOST, CUT_PLANES, F_GLASS, K_CUT, K_ROT, MAX_BOXES, MAX_LEAVES, REC, R_C, R_COS, R_CUT, R_CUTN, R_H, R_KIND, R_SIN, WORLDS};
use prim::{floor, sqrt};

/// the sky of the hour or of the night, by luminance: where its one directional light stands and what that puts on a face turned to it, the dome's top and foot
#[derive(Clone, Copy)]
pub struct Sky {
    pub to_sun: [f64; 3],
    pub sun: f64,
    pub zenith: f64,
    pub horizon: f64,
}
/// what a step is told: how many steps a sweep takes, the sky, and the lamps: what the host says of the level's rows (light.rs Rows: the steady ones and
/// their steady selves) and how many lamps of nobody's fixture it wrote beside them (light.rs LAMPS)
pub struct K {
    pub sweep: f64,
    pub sky: Sky,
    pub lamps: usize,
    pub rows: Option<light::Rows>,
}
/// the height the lines go out from (between a standing man's chest and his head: the meter is the brighter of the two), a metre of the plan, and how many of those a square of the smoke grid is across
pub const HEIGHT: f64 = 1.4;
pub const METRE: f64 = 1.0;
const PER: usize = (SIDE / METRE) as usize;
/// the plan's half metres, for the roofs
const MAX_HALF: usize = MAX_GLOW * 4;

const PI: f64 = 3.141592653589793;
/// lines a metre sends out
pub const RAYS: usize = 128;
/// as far as a line is followed (m)
const FAR: f64 = 60.0;
/// the height nothing of a level stands over (m)
const TOP: f64 = 10.0;
/// a roof is what stands over a place higher than this (m)
const ROOF: f64 = 2.0;
/// what the ground outside sends back of the light on it, as an opening shows it (the picture's own guess: scene/openings.ts LOOK_OPENINGS.ground), and what
/// a floor with no box of its own does
const GROUND: f64 = 0.5;
const BARE: f64 = 0.25;
/// what a leaf sends back, and how near one has to hang to be asked (m)
const LEAF: f64 = 0.3;
const NEAR: f64 = 12.0;
const MOST: usize = 24;
/// whole sweeps in a new field's first step · steps to a round once the lamps have changed, and how many such rounds
const FIRST: usize = 4;
/// HOW LIGHT A PLACE IS: how far it is smoothed (m; scene/placePlan.ts PLACE.reach), the solve's over-relaxation (the best for a pull this hard over cells a metre wide: the
/// picture's 1.93 is its 0.4 m cells'), its rounds a step (a change is nine tenths in within one) and in a new field's first; no reading under
/// this is told from the dark, and what a metre that holds none counts for
const REACH: f64 = 8.0;
const OMEGA: f64 = 1.84;
const LEVELS: [usize; 2] = [12, 160];
const NOTHING: f64 = 1e-6;
const DARK: f64 = 0.02;
/// A DOORWAY: a way through no wider than this (four whole cells of the picture's, PLACE.doorCells: 1.6 m to just under 2), that is wider by this within a metre on both sides (a passage that stays narrow is one place); in steps of a quarter metre
const DOORWAY: i32 = 7;
const WIDER: i32 = 2;
/// as far as the question whether two metres are one place asks (m)
const JOIN: f32 = 2.6;
pub const QUICK: f64 = 8.0;
const ROUNDS: f64 = 6.0;

/// WHERE A LINE ENDS: how far (cm), what it meets (the metre before the surface, or one of the words below), how much that sends back (of 255), and the
/// sun's cosine there (of 255; 0 = it is in shade)
#[repr(C)]
#[derive(Clone, Copy)]
struct Ray {
    cm: u16,
    to: u16,
    back: u8,
    sun: u8,
}
const TO_SKY: u16 = 0xffff;
const TO_GROUND: u16 = 0xfffe;
/// a surface under the open sky, seen out by an opening over the horizon
const TO_OUTSIDE: u16 = 0xfffd;
/// nothing: a surface in front of no metre's middle
const TO_DARK: u16 = 0xfffc;
const _: () = assert!(MAX_GLOW < TO_DARK as usize);

/// A WORLD'S TABLE, made from its statics
struct Cast {
    rays: [[Ray; RAYS]; MAX_GLOW],
    /// is a metre's row of the statics as they stand · what its lines cross of the plan (x0 z0 x1 z1), the sun's ways from where they end with them
    cast: [bool; MAX_GLOW],
    reach: [[f32; 4]; MAX_GLOW],
    /// 0 not asked · 1 the open sky · 2 a roof
    roof: [u8; MAX_HALF],
    /// is a metre one place with the next along x (1) and the next along z (2) · 4 = it was asked
    joined: [u8; MAX_GLOW],
    /// whose it is: the plan in metres, the sun's place, the world's count of writes when the rows were last compared
    nx: usize,
    nz: usize,
    to_sun: [f64; 3],
    wrote: u32,
    /// the statics it was made from: how many, a word of each one's row, what each takes of the plan
    boxes: usize,
    word: [u64; MAX_BOXES],
    at: [[f32; 4]; MAX_BOXES],
}
const NO_RAY: Ray = Ray { cm: 0, to: 0, back: 0, sun: 0 };
const NO_CAST: Cast = Cast {
    rays: [[NO_RAY; RAYS]; MAX_GLOW],
    cast: [false; MAX_GLOW],
    reach: [[0.0; 4]; MAX_GLOW],
    roof: [0; MAX_HALF],
    joined: [0; MAX_GLOW],
    nx: 0,
    nz: 0,
    to_sun: [0.0; 3],
    wrote: 0,
    boxes: 0,
    word: [0; MAX_BOXES],
    at: [[0.0; 4]; MAX_BOXES],
};
static mut CASTS: [Cast; WORLDS] = [NO_CAST; WORLDS];
#[inline(always)]
fn cast_of(w: usize) -> &'static mut Cast {
    unsafe { &mut (*core::ptr::addr_of_mut!(CASTS))[if w < WORLDS { w } else { 0 }] }
}

/// THE WAYS: each line's direction, and what it counts for on each of the six faces (its cosine × its share of the sphere; the shares of a face sum to π)
struct Ways {
    d: [[f64; 3]; RAYS],
    c: [[f64; 6]; RAYS],
    made: bool,
}
static mut WAYS: Ways = Ways { d: [[0.0; 3]; RAYS], c: [[0.0; 6]; RAYS], made: false };
fn ways() -> &'static Ways {
    let w = unsafe { &mut *core::ptr::addr_of_mut!(WAYS) };
    if !w.made {
        let mut sum = [0.0; 6];
        for i in 0..RAYS {
            // (a spiral over the sphere, each turn the golden angle on from the last: as even as so many can lie)
            let y = 1.0 - (i as f64 + 0.5) * 2.0 / RAYS as f64;
            let (r, a) = (sqrt(1.0 - y * y), i as f64 * 2.399963229728653);
            let d = [cos(a) * r, y, sin(a) * r];
            w.d[i] = d;
            for k in 0..6 {
                let v = if k & 1 == 0 { d[k / 2] } else { -d[k / 2] };
                w.c[i][k] = if v > 0.0 { v } else { 0.0 };
                sum[k] += w.c[i][k];
            }
        }
        for c in w.c.iter_mut() {
            for k in 0..6 {
                c[k] *= PI / sum[k];
            }
        }
        w.made = true;
    }
    w
}

/// how many metres the last step worked out, and how many of those had their lines cast (what it cost: no state, nothing of the game reads it)
static mut SWEPT: [usize; 2] = [0; 2];
pub fn swept() -> [usize; 2] {
    unsafe { SWEPT }
}

/// the plan's rectangle round a static's row
fn plan_of(r: &[f64]) -> [f32; 4] {
    let abs = |v: f64| if v < 0.0 { -v } else { v };
    let (x, z) = if r[R_KIND] as u32 & K_ROT != 0 {
        let e = r[R_H] + r[R_H + 1] + r[R_H + 2];
        (e, e)
    } else {
        let (c, s) = (abs(r[R_COS]), abs(r[R_SIN]));
        (c * r[R_H] + s * r[R_H + 2], s * r[R_H] + c * r[R_H + 2])
    };
    [(r[R_C] - x) as f32, (r[R_C + 2] - z) as f32, (r[R_C] + x) as f32, (r[R_C + 2] + z) as f32]
}
fn over(a: &[f32; 4], b: &[f32; 4]) -> bool {
    a[0] <= b[2] && b[0] <= a[2] && a[1] <= b[3] && b[1] <= a[3]
}
/// a word of static `i`'s row: its sixteen numbers, its planes, its flags and what it sends back
fn word_of(o: &World, i: usize) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325 ^ ((o.flags[i] as u64) << 8 | o.back[i] as u64);
    let r = &o.rec[i * REC..(i + 1) * REC];
    let planes = if r[R_KIND] as u32 & K_CUT != 0 { &o.cut[r[R_CUT] as usize * CUT_PLANES * 4..][..js_min(r[R_CUTN], CUT_PLANES as f64) as usize * 4] } else { &[] };
    for v in r.iter().chain(planes) {
        h = (h ^ v.to_bits()).wrapping_mul(0x9e3779b97f4a7c15);
        h ^= h >> 29;
    }
    h
}
/// THE TABLE IS OF THE STATICS AS THEY STAND: on another plan or under another sun all of it is made again; where rows differ from those it was made
/// from, what crosses them
fn fresh(w: usize, nx: usize, nz: usize, to_sun: Option<[f64; 3]>) -> &'static mut Cast {
    let (c, o) = (cast_of(w), world_read(w));
    // (a world emptied and filled again is another, for what was asked of it while it was empty)
    if c.nx != nx || c.nz != nz || (c.boxes == 0 && c.wrote != o.wrote) {
        c.nx = nx;
        c.nz = nz;
        c.roof = [0; MAX_HALF];
        c.joined = [0; MAX_GLOW];
        c.cast = [false; MAX_GLOW];
        c.boxes = 0;
        c.wrote = o.wrote.wrapping_sub(1);
    }
    if let Some(s) = to_sun {
        if c.to_sun != s {
            c.to_sun = s;
            c.cast = [false; MAX_GLOW];
        }
    }
    if c.wrote != o.wrote {
        c.wrote = o.wrote;
        let n = js_min(o.boxes as f64, MAX_BOXES as f64) as usize;
        for i in 0..if n > c.boxes { n } else { c.boxes } {
            let (had, has) = (i < c.boxes, i < n);
            let word = if has { word_of(o, i) } else { 0 };
            if had && has && word == c.word[i] {
                continue;
            }
            if had {
                let r = c.at[i];
                stale(c, &r);
            }
            if has {
                let r = plan_of(&o.rec[i * REC..(i + 1) * REC]);
                c.at[i] = r;
                c.word[i] = word;
                stale(c, &r);
            }
        }
        c.boxes = n;
    }
    c
}
/// what crosses this rectangle of the plan is to be made again
fn stale(c: &mut Cast, r: &[f32; 4]) {
    for m in 0..c.nx * c.nz {
        if c.cast[m] && over(&c.reach[m], r) {
            c.cast[m] = false;
        }
        let (x, z) = ((m % c.nx) as f32 + 0.5, (m / c.nx) as f32 + 0.5);
        if over(&[x - JOIN, z - JOIN, x + JOIN, z + JOIN], r) {
            c.joined[m] = 0;
        }
    }
    let lim = |v: f32, n: usize| -> usize { js_min(js_max(floor(v as f64 * 2.0), 0.0), n as f64 * 2.0 - 1.0) as usize };
    for z in lim(r[1], c.nz)..=lim(r[3], c.nz) {
        for x in lim(r[0], c.nx)..=lim(r[2], c.nx) {
            c.roof[z * c.nx * 2 + x] = 0;
        }
    }
}

/// the first static a line meets, panes of glass let by: how far, its face's normal, which
fn first(w: usize, at: [f64; 3], d: [f64; 3], tmax: f64) -> Option<(f64, [f64; 3], usize)> {
    let (mut gone, mut out) = (0.0, [0.0; 4]);
    for _ in 0..8 {
        let a = [at[0] + d[0] * gone, at[1] + d[1] * gone, at[2] + d[2] * gone];
        let left = tmax - gone;
        if !(left > 0.0) {
            return None;
        }
        let bi = lines::ray(w, a[0], a[1], a[2], d[0], d[1], d[2], left, -1, false, &mut out);
        if bi < 0 {
            return None;
        }
        if world_read(w).flags[bi as usize] & F_GLASS == 0 {
            return Some((gone + out[0], [out[1], out[2], out[3]], bi as usize));
        }
        gone += out[0] + 0.02;
    }
    None
}
fn roof(c: &mut Cast, w: usize, x: f64, z: f64) -> bool {
    let (jx, jz) = (floor(x * 2.0), floor(z * 2.0));
    if !(jx >= 0.0 && jz >= 0.0 && jx < c.nx as f64 * 2.0 && jz < c.nz as f64 * 2.0) {
        return false;
    }
    let i = jz as usize * c.nx * 2 + jx as usize;
    if c.roof[i] == 0 {
        c.roof[i] = if first(w, [jx * 0.5 + 0.25, 30.0, jz * 0.5 + 0.25], [0.0, -1.0, 0.0], 30.0 - ROOF).is_some() { 2 } else { 1 };
    }
    c.roof[i] == 2
}
/// THE METRE A POINT READS: the nearest whose middle it has a clear line to past the statics, each way (its own, the one it leans to, the other, the
/// corner's); `good` says whether a metre will do at all
fn metre_of(w: usize, nx: usize, nz: usize, p: [f64; 3], mut good: impl FnMut(usize) -> bool) -> Option<usize> {
    let (ix, iz) = (floor(p[0]), floor(p[2]));
    let (ax, az) = (p[0] - ix - 0.5, p[2] - iz - 0.5);
    let (sx, sz) = (if ax < 0.0 { -1.0 } else { 1.0 }, if az < 0.0 { -1.0 } else { 1.0 });
    let x_first = ax * ax >= az * az;
    for k in 0..4 {
        let dx = if k == 3 || k == (if x_first { 1 } else { 2 }) { sx } else { 0.0 };
        let dz = if k == 3 || k == (if x_first { 2 } else { 1 }) { sz } else { 0.0 };
        let (jx, jz) = (ix + dx, iz + dz);
        if !(jx >= 0.0 && jz >= 0.0 && jx < nx as f64 && jz < nz as f64) {
            continue;
        }
        let m = jz as usize * nx + jx as usize;
        let c = [jx + 0.5, HEIGHT, jz + 0.5];
        let clear = |a: [f64; 3], b: [f64; 3]| lines::sight_line(w, -1, F_GLASS, a[0], a[1], a[2], b[0], b[1], b[2]) == Some(false);
        if good(m) && clear(p, c) && clear(c, p) {
            return Some(m);
        }
    }
    None
}

/// a metre's lines, followed among the statics
fn cast(c: &mut Cast, w: usize, m: usize) {
    let (nx, nz) = (c.nx, c.nz);
    let a = [(m % nx) as f64 + 0.5, HEIGHT, (m / nx) as f64 + 0.5];
    let to_sun = c.to_sun;
    let mut box_ = [a[0], a[2], a[0], a[2]];
    let mut see = |x: f64, z: f64| {
        box_ = [js_min(box_[0], x), js_min(box_[1], z), js_max(box_[2], x), js_max(box_[3], z)];
    };
    // (nothing stands higher over a level than this: a line or the sun's way is followed on the plan to there)
    let to_top = |y: f64, up: f64| if up > 0.05 { js_min(FAR, js_max(0.0, TOP - y) / up) } else { FAR };
    for i in 0..RAYS {
        let d = ways().d[i];
        let hit = first(w, a, d, FAR);
        let down = if d[1] < -1e-6 { a[1] / -d[1] } else { FAR };
        // (the floor is a box in most places; where it is not, it is bare)
        let (far, normal, back) = match hit {
            Some((t, n, bi)) if t < down + 0.05 => (t, n, world_read(w).back[bi]),
            _ => (down, [0.0, 1.0, 0.0], (BARE * 255.0) as u8),
        };
        let mut out = None;
        let mut t = 0.1;
        while t < far - 0.02 {
            if !roof(c, w, a[0] + d[0] * t, a[2] + d[2] * t) {
                out = Some(t);
                break;
            }
            t += 0.25;
        }
        let end = if let Some(t) = out { t } else { far };
        let seen = if out.is_some() && d[1] > 0.0 { js_max(end, to_top(a[1], d[1])) } else { end };
        see(a[0] + d[0] * seen, a[2] + d[2] * seen);
        let sun_at = |h: [f64; 3], n: [f64; 3]| -> u8 {
            let ns = n[0] * to_sun[0] + n[1] * to_sun[1] + n[2] * to_sun[2];
            if to_sun[1] > 0.0 && ns > 0.0 && first(w, [h[0] + n[0] * 0.02, h[1] + n[1] * 0.02, h[2] + n[2] * 0.02], to_sun, FAR).is_none() {
                js_max(1.0, js_min(255.0, ns * 255.0)) as u8
            } else {
                0
            }
        };
        let h = [a[0] + d[0] * far, a[1] + d[1] * far, a[2] + d[2] * far];
        let (to, sun) = if out.is_some() || far >= FAR {
            if d[1] <= 0.0 {
                (TO_GROUND, 0)
            } else if far >= FAR {
                (TO_SKY, 0)
            } else {
                let run = to_top(h[1], to_sun[1]);
                see(h[0] + to_sun[0] * run, h[2] + to_sun[2] * run);
                (TO_OUTSIDE, sun_at(h, normal))
            }
        } else {
            let q = [h[0] + normal[0] * 0.25, js_max(0.3, js_min(2.4, h[1] + normal[1] * 0.25)), h[2] + normal[2] * 0.25];
            match metre_of(w, nx, nz, q, |_| true) {
                // (the metre before it is out under the sky: it is an eave's, or a wall's beside an opening)
                Some(k) if !roof(c, w, (k % nx) as f64 + 0.5, (k / nx) as f64 + 0.5) => {
                    let run = to_top(h[1], to_sun[1]);
                    see(h[0] + to_sun[0] * run, h[2] + to_sun[2] * run);
                    (TO_OUTSIDE, sun_at(h, normal))
                }
                Some(k) => {
                    let run = to_top(h[1], to_sun[1]);
                    see(h[0] + to_sun[0] * run, h[2] + to_sun[2] * run);
                    (k as u16, sun_at(h, normal))
                }
                None => (TO_DARK, 0),
            }
        };
        c.rays[m][i] = Ray { cm: js_min(65535.0, end * 100.0) as u16, to, back, sun };
    }
    c.reach[m] = [box_[0] as f32 - 0.6, box_[1] as f32 - 0.6, box_[2] as f32 + 0.6, box_[3] as f32 + 0.6];
    c.cast[m] = true;
}

/// IS A METRE ONE PLACE WITH THE NEXT, along x or along z: the way between them past the statics, panes among them, and no doorway
fn joined(w: usize, x: f64, z: f64, along_x: bool) -> bool {
    let clear = |a: [f64; 3], b: [f64; 3]| {
        let see = |a: [f64; 3], b: [f64; 3]| lines::sight_line(w, -1, 0, a[0], a[1], a[2], b[0], b[1], b[2]) == Some(false);
        see(a, b) && see(b, a)
    };
    let p = |on: f64, off: f64| if along_x { [x + on, HEIGHT, z + off] } else { [x + off, HEIGHT, z + on] };
    // (where the way between the two is free: the nearest to the middle, and the run of it)
    let free = |k: i32| clear(p(0.0, k as f64 * 0.25), p(1.0, k as f64 * 0.25));
    let Some(mid) = [0, 1, -1].into_iter().find(|&k| free(k)) else { return false };
    let run = |f: &dyn Fn(i32) -> bool| 1 + (1..=8).take_while(|&k| f(mid + k)).count() as i32 + (1..=8).take_while(|&k| f(mid - k)).count() as i32;
    let wide = run(&free);
    if wide > DOORWAY {
        return true;
    }
    let off = mid as f64 * 0.25;
    let opens = |from: f64, way: f64| {
        [0.0, 0.5, 1.0].into_iter().any(|d| {
            let on = from + way * d;
            (d == 0.0 || clear(p(from, off), p(on, off))) && run(&|k| clear(p(on, off), p(on, k as f64 * 0.25))) >= wide + WIDER
        })
    };
    !(opens(0.0, -1.0) && opens(1.0, 1.0))
}
/// THE LEVEL OF EVERY PLACE, some rounds of it: each metre under a roof is pulled to its own reading and, hard, to the metres it is one place with. `open` = the open air's
fn levels(c: &mut Cast, w: usize, set: i32, st: &mut [f64; LEN], rounds: usize, open: f64) {
    let (nx, nz) = (c.nx, c.nz);
    for m in 0..nx * nz {
        if c.joined[m] == 0 {
            let (x, z) = ((m % nx) as f64 + 0.5, (m / nx) as f64 + 0.5);
            c.joined[m] = 4 | if m % nx + 1 < nx && joined(w, x, z, true) { 1 } else { 0 } | if m / nx + 1 < nz && joined(w, x, z, false) { 2 } else { 0 };
        }
    }
    let mut way = c.joined;
    // (a leaf across a way that is no doorway's: a gate, a shutter)
    let o = world_read(w);
    if set >= 0 {
        for i in 0..js_min(o.leaves[set as usize] as f64, MAX_LEAVES as f64) as usize {
            let at = (set as usize * MAX_LEAVES + i) * REC;
            let r = &o.leaf[at..at + REC];
            let b = plan_of(r);
            let lim = |v: f32, n: usize| js_min(js_max(floor(v as f64), 0.0), n as f64 - 1.0) as usize;
            for z in lim(b[1] - 1.0, nz)..=lim(b[3] + 1.0, nz) {
                for x in lim(b[0] - 1.0, nx)..=lim(b[2] + 1.0, nx) {
                    let m = z * nx + x;
                    for (bit, d) in [(1, [1.0, 0.0, 0.0]), (2, [0.0, 0.0, 1.0])] {
                        if way[m] & bit != 0 && lines::ray_box_n(&o.cut, x as f64 + 0.5, HEIGHT, z as f64 + 0.5, d[0], d[1], d[2], r, 1.0).is_some() {
                            way[m] &= !bit;
                        }
                    }
                }
            }
        }
    }
    let open = log(js_max(open, NOTHING));
    let pull = REACH * REACH;
    let mut read = [0.0; MAX_GLOW];
    let first = rounds == LEVELS[1];
    for m in 0..nx * nz {
        read[m] = if roof(c, w, (m % nx) as f64 + 0.5, (m / nx) as f64 + 0.5) { log(js_max(st[O_GLOW_UP + m], NOTHING)) } else { open };
        if first {
            st[O_GLOW_PLACE + m] = read[m];
        }
    }
    for _ in 0..rounds {
        for m in 0..nx * nz {
            let (x, z) = (m % nx, m / nx);
            let weight = if st[O_GLOW_UP + m] > NOTHING || read[m] == open { 1.0 } else { DARK };
            let (mut sum, mut den) = (weight * read[m], weight);
            let mut hand = |n: usize| {
                sum += pull * st[O_GLOW_PLACE + n];
                den += pull;
            };
            if way[m] & 1 != 0 { hand(m + 1); }
            if way[m] & 2 != 0 { hand(m + nx); }
            if x > 0 && way[m - 1] & 1 != 0 { hand(m - 1); }
            if z > 0 && way[m - nx] & 2 != 0 { hand(m - nx); }
            let was = st[O_GLOW_PLACE + m];
            st[O_GLOW_PLACE + m] = was + OMEGA * (sum / den - was);
        }
    }
}

/// the six faces of the meter under the sweep's lamps at a point (None: a line of it is the host's to judge)
fn lamps_on(w: usize, set: i32, k: &K, p: [f64; 3]) -> Option<[f64; 6]> {
    let (mut acc, mut at) = ([0.0; 6], 0usize);
    if light::estimate(w, set, k.lamps, p, &light::NO_SHADE, false, k.rows, true, &mut at, &mut acc) == ASK_HOST {
        return None;
    }
    Some(acc)
}
fn mean(v: &[f64; 6]) -> f64 {
    (v[0] + v[1] + v[2] + v[3] + v[4] + v[5]) / 6.0
}

/// ONE METRE WORKED OUT: what its lines bring of the metres as they are
fn work(c: &mut Cast, w: usize, set: i32, st: &mut [f64; LEN], m: usize, k: &K) {
    if !c.cast[m] {
        cast(c, w, m);
        unsafe { SWEPT[1] += 1 };
    }
    let a = [(m % c.nx) as f64 + 0.5, HEIGHT, (m / c.nx) as f64 + 0.5];
    // the leaves that hang near enough to be in a line's way
    let o = world_read(w);
    let (mut near, mut n) = ([0usize; MOST], 0);
    if set >= 0 {
        for i in 0..js_min(o.leaves[set as usize] as f64, MAX_LEAVES as f64) as usize {
            let at = (set as usize * MAX_LEAVES + i) * REC;
            let (dx, dz) = (o.leaf[at + R_C] - a[0], o.leaf[at + R_C + 2] - a[2]);
            if dx * dx + dz * dz < NEAR * NEAR && n < MOST {
                near[n] = at;
                n += 1;
            }
        }
    }
    let shut = |p: [f64; 3], d: [f64; 3], tmax: f64| near[..n].iter().any(|&at| lines::ray_box_n(&o.cut, p[0], p[1], p[2], d[0], d[1], d[2], &o.leaf[at..at + REC], tmax).is_some());
    let s = &k.sky;
    let open = open_air(s) + s.sun * js_max(0.0, s.to_sun[1]);
    let w_ = ways();
    let mut f = [0.0; 6];
    for (i, r) in c.rays[m].iter().enumerate() {
        let d = w_.d[i];
        let t = r.cm as f64 / 100.0;
        let back = r.back as f64 / 255.0;
        // (a line that goes out is followed a leaf's width on: the leaf hangs in the opening's frame or stands open before it)
        let l = if n > 0 && shut(a, d, if r.to >= TO_OUTSIDE { t + 1.0 } else { t }) {
            LEAF / PI * st[O_GLOW_A + m]
        } else {
            match r.to {
                TO_SKY => s.horizon + (s.zenith - s.horizon) * js_min(1.0, d[1] * 2.5),
                TO_GROUND => GROUND / PI * open,
                TO_OUTSIDE => back / PI * (s.sun * r.sun as f64 / 255.0 + PI * 0.5 * s.horizon),
                TO_DARK => 0.0,
                to => {
                    let h = [a[0] + d[0] * (t - 0.02), a[1] + d[1] * (t - 0.02), a[2] + d[2] * (t - 0.02)];
                    let sun = if r.sun > 0 && s.sun > 0.0 && !(n > 0 && shut(h, s.to_sun, NEAR)) { s.sun * r.sun as f64 / 255.0 } else { 0.0 };
                    back / PI * (sun + st[O_GLOW_A + to as usize])
                }
            }
        };
        if l > 0.0 {
            for (f, c) in f.iter_mut().zip(&w_.c[i]) {
                *f += l * c;
            }
        }
    }
    let avg = mean(&f);
    st[O_GLOW_MEAN + m] = avg;
    st[O_GLOW_UP + m] = f[2];
    st[O_GLOW_MAX + m] = js_max(js_max(js_max(f[0], f[1]), js_max(f[2], f[3])), js_max(f[4], f[5]));
    if let Some(lamps) = lamps_on(w, set, k, a) {
        st[O_GLOW_A + m] = avg + mean(&lamps);
    }
}

/// A STEP WITH THE LIGHT MODEL DOWN: a field that was swept is no field (its doors and lamps would be old ones the day the model is raised again)
pub fn down(st: &mut [f64; LEN]) {
    if st[O_GLOW_LAMPS] != 0.0 {
        for v in st[O_GLOW_MAX..O_RHO].iter_mut() {
            *v = 0.0;
        }
    }
}
/// ONE STEP over a plan of `nbx` by `nbz` squares of the smoke grid, two metres to a square's side
pub fn step(w: usize, set: i32, st: &mut [f64; LEN], nbx: usize, nbz: usize, k: &K) {
    let (nx, nz) = (nbx * PER, nbz * PER);
    let n = nx * nz;
    let real = |v: f64| v - v == 0.0 && v >= 0.0;
    if n == 0 || n > MAX_GLOW || !(k.sweep >= 1.0) || !(real(k.sky.sun) && real(k.sky.zenith) && real(k.sky.horizon)) {
        return;
    }
    let c = fresh(w, nx, nz, Some(k.sky.to_sun));
    let lamps = light::word(k.lamps, k.rows);
    let new = st[O_GLOW_LAMPS] == 0.0;
    if st[O_GLOW_LAMPS] != lamps {
        st[O_GLOW_LEFT] = n as f64 * ROUNDS;
    }
    st[O_GLOW_LAMPS] = lamps;
    let pace = if st[O_GLOW_LEFT] > 0.0 && QUICK < k.sweep { QUICK } else { k.sweep };
    let per = if new { n * FIRST } else { (-floor(-(n as f64) / pace)) as usize };
    let mut i = if st[O_GLOW_AT] >= 0.0 && st[O_GLOW_AT] < n as f64 { st[O_GLOW_AT] as usize } else { 0 };
    st[O_GLOW_LEFT] = if new { 0.0 } else { js_max(0.0, st[O_GLOW_LEFT] - per as f64) };
    st[O_GLOW_AT] = ((i + per) % n) as f64;
    unsafe { SWEPT = [per, 0] };
    for _ in 0..per {
        if i >= n {
            i = 0;
        }
        if roof(c, w, (i % nx) as f64 + 0.5, (i / nx) as f64 + 0.5) {
            work(c, w, set, st, i, k);
        }
        i += 1;
    }
    levels(c, w, set, st, LEVELS[new as usize], open_air(&k.sky));
}
/// what is said of a metre: the mean of the six faces, the best of them, and how light the place it is in is
#[derive(Clone, Copy, Debug)]
pub struct Field {
    pub mean: f64,
    pub best: f64,
    pub place: f64,
}
/// how light the open air is: the sky's light on the ground (the sun's own beam is no place's level, as a lamp's is none)
pub fn open_air(s: &Sky) -> f64 {
    PI * (0.947 * s.zenith + 0.053 * s.horizon)
}
/// THE FIELD AT A POINT UNDER A ROOF, of the metre it reads (None: it is under the open sky, or no metre
/// under a roof shows it its middle, or the field was never swept)
pub fn at(w: usize, st: &[f64; LEN], nbx: usize, nbz: usize, p: [f64; 3]) -> Option<Field> {
    let (nx, nz) = (nbx * PER, nbz * PER);
    if nx * nz == 0 || nx * nz > MAX_GLOW || st[O_GLOW_LAMPS] == 0.0 {
        return None;
    }
    let c = fresh(w, nx, nz, None);
    if !roof(c, w, p[0], p[2]) {
        return None;
    }
    let m = metre_of(w, nx, nz, p, |m| roof(c, w, (m % nx) as f64 + 0.5, (m / nx) as f64 + 0.5))?;
    Some(Field { mean: st[O_GLOW_MEAN + m], best: st[O_GLOW_MAX + m], place: exp(st[O_GLOW_PLACE + m]) })
}
