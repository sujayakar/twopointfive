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
//! follows its lamp within a few seconds, most of it in the first; a new field is swept whole in each of its first FIRST steps.
//!
//! HOW LIGHT A PLACE IS, for eyes that are used to it: the light on a face that looks up, smoothed over REACH metres and never through a wall, a pane or a doorway, so that a
//! room has one level and the yard outside its door another (the picture's own place field, render/shaders/place_field.wgsl, by its own numbers).
//!
//! WHAT IS STATE: the five numbers a metre, the lamps' word, where the sweep is (bricks.rs O_GLOW_…: a part of the game's block). WHAT IS MADE AGAIN from the
//! box world as it stands, a table a world: which half metres have a roof, where each metre's lines end among the statics, and which metres are one place. A static that is
//! written is marked on the plan (`fresh` compares the rows themselves, so who wrote them, and how often, is nothing to it), and the lines that pass a mark are
//! followed again, those and no more: what a sweep works out is of the statics and the leaves as they are when it comes by, whether the table was kept or made that
//! moment. Between steps some of what the next ones want is made ahead (`ahead`), so that a blown wall costs no tick the whole of its room's lines. The leaves are
//! asked at the sweep.
//!
//! WHAT IT LEAVES OUT, named: a lamp somebody carries (it moves faster than a sweep), a flash, the smoke, a lamp under the open sky seen from inside.

use game_types::knobs::Knobs;
use crate::bricks::{LEN, MAX_GLOW, O_GLOW_A, O_GLOW_AT, O_GLOW_LAMPS, O_GLOW_LEFT, O_GLOW_MAX, O_GLOW_MEAN, O_GLOW_PLACE, O_GLOW_UP, O_RHO, SIDE};
use crate::light;
use crate::lines;
use crate::maths::{cos, exp, log, sin};
use crate::state::GAMES;
use crate::world::{js_max, js_min, World, ASK_HOST, F_GLASS, K_ROT, MAX_BOXES, REC, R_C, R_COS, R_H, R_KIND, R_SIN, Seen};
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
/// their steady selves) and the lamps of nobody's fixture beside them (light.rs LAMP)
pub struct K<'a> {
    pub sweep: f64,
    pub sky: Sky,
    pub lamps: &'a [[f64; light::LAMP]],
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
/// what the ground outside sends back of the light on it, as an opening shows it, and what
/// a floor with no box of its own does
const GROUND: f64 = 0.5;
const BARE: f64 = 0.25;
/// what a leaf sends back, and how near one has to hang to be asked (m)
const LEAF: f64 = 0.3;
const NEAR: f64 = 12.0;
const MOST: usize = 24;
/// a new field's first steps, a whole sweep each · steps to a round once the lamps have changed, and how many such rounds
const FIRST: usize = 4;
/// HOW LIGHT A PLACE IS: how far it is smoothed, the solve's over-relaxation (the best for a pull this hard over cells a metre wide: the
/// picture's 1.93 is its 0.4 m cells'), its rounds a step (a change is nine tenths in within one) and in each of a new field's first; no reading under
/// this is told from the dark, and what a metre that holds none counts for
const REACH: f64 = 8.0;
const OMEGA: f64 = 1.84;
const LEVELS: [usize; 2] = [12, 40];
const NOTHING: f64 = 1e-6;
const DARK: f64 = 0.02;
/// A DOORWAY: a way through no wider than this (four whole cells of the picture's, PLACE.doorCells: 1.6 m to just under 2), that is wider by this within a metre on both sides (a passage that stays narrow is one place); in steps of a quarter metre
const DOORWAY: i32 = 7;
const WIDER: i32 = 2;
/// as far as the question whether two metres are one place asks (m)
const JOIN: f32 = 2.6;
pub const QUICK: f64 = 8.0;
const ROUNDS: f64 = 6.0;

/// WHERE A LINE ENDS: how far (cm), what it meets (the metre before the surface, or one of the words below), whose surface (what that sends back is its box's to say, as it
/// is painted: its place in the list and one, 0 = bare floor), the sun's cosine there (of 255; 0 = it is in shade), and how far the sun's way was followed from
/// there (half metres and one, 0 = it was not, WHOLE = to the top), with OWN where the metre it reads is the one it ends in
#[repr(C)]
#[derive(Clone, Copy)]
struct Ray {
    cm: u16,
    to: u16,
    of: u16,
    sun: u8,
    way: u8,
}
const _: () = assert!(MAX_BOXES < 0xffff);
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
    /// how many times the statics were found changed, and the last of those that a metre's lines are of (0 = they were never followed)
    now: u32,
    traced: [u32; MAX_GLOW],
    /// the last change that touched a half metre of the plan (a path that passes it), the last that did and stood under the lines' start all of it (a line that rises
    /// never meets it), and one near a metre's middle (where a line may end)
    half: [u32; MAX_HALF],
    low: [u32; MAX_HALF],
    near: [u32; MAX_GLOW],
    /// the change every metre under a roof is made for
    made: u32,
    /// 0 not asked · 1 the open sky · 2 a roof
    roof: [u8; MAX_HALF],
    /// is a metre one place with the next along x (1) and the next along z (2) · 4 = it was asked
    joined: [u8; MAX_GLOW],
    /// whose it is: the plan in metres, the sun's place, the world's count of writes when the rows were last compared
    nx: usize,
    nz: usize,
    to_sun: [f64; 3],
    /// the statics it was made from, and what each takes of the plan
    seen: Seen,
}
// (all zeros: a table that is not would be in the module's file)
const NO_RAY: Ray = Ray { cm: 0, to: 0, of: 0, sun: 0, way: 0 };
const NO_CAST: Cast = Cast {
    rays: [[NO_RAY; RAYS]; MAX_GLOW],
    now: 0,
    traced: [0; MAX_GLOW],
    half: [0; MAX_HALF],
    low: [0; MAX_HALF],
    near: [0; MAX_GLOW],
    made: 0,
    roof: [0; MAX_HALF],
    joined: [0; MAX_GLOW],
    nx: 0,
    nz: 0,
    to_sun: [0.0; 3],
    seen: Seen::NONE,
};
static mut CASTS: [Cast; GAMES] = [NO_CAST; GAMES];
#[inline(always)]
fn cast_of(g: usize) -> &'static mut Cast {
    unsafe { &mut (*core::ptr::addr_of_mut!(CASTS))[g] }
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
/// how far from a static a path's samples (PACE apart) and a half metre's own middle may be, and a line's end from what is asked round it: the metres round it
/// and the ways between
const BY: f32 = 0.55;
const ROUND: f32 = 2.3;
const PACE: f64 = 0.5;
const OWN: u8 = 128;
const WHOLE: u8 = 127;
/// …and round its row, with how high it stands
fn round_of(r: &[f64]) -> [f64; 5] {
    let p = plan_of(r);
    let up = if r[R_KIND] as u32 & K_ROT != 0 { r[R_H] + r[R_H + 1] + r[R_H + 2] } else { r[R_H + 1] };
    [p[0] as f64, p[1] as f64, p[2] as f64, p[3] as f64, r[R_C + 1] + up]
}
/// THE TABLE IS OF THE STATICS AS THEY STAND: on another plan or under another sun all of it is made again; where rows differ from those it was made
/// from, what crosses them
fn fresh(g: usize, w: &World, nx: usize, nz: usize, to_sun: Option<[f64; 3]>) -> &'static mut Cast {
    let c = cast_of(g);
    // (a world emptied and filled again is another, for what was asked of it while it was empty)
    if c.nx != nx || c.nz != nz || c.seen.empty_of(w) {
        c.nx = nx;
        c.nz = nz;
        c.roof = [0; MAX_HALF];
        c.joined = [0; MAX_GLOW];
        c.traced = [0; MAX_GLOW];
        c.now += 1;
        c.seen.forget();
    }
    if let Some(s) = to_sun {
        if c.to_sun != s {
            c.to_sun = s;
            c.traced = [0; MAX_GLOW];
            c.now += 1;
        }
    }
    let Cast { seen, joined, roof, half, low, near, now, nx, nz, .. } = c;
    let mut first = true;
    seen.look(w, round_of, |r| {
        *now += first as u32;
        first = false;
        let touched = if r[4] < HEIGHT - 0.01 { &mut *low } else { &mut *half };
        stale(joined, roof, touched, near, *now, *nx, *nz, &[r[0] as f32, r[1] as f32, r[2] as f32, r[3] as f32]);
    });
    c
}
/// what crosses this rectangle of the plan is marked, or is to be made again
#[allow(clippy::too_many_arguments)]
fn stale(joined: &mut [u8; MAX_GLOW], roof: &mut [u8; MAX_HALF], touched: &mut [u32; MAX_HALF], near: &mut [u32; MAX_GLOW], now: u32, nx: usize, nz: usize, r: &[f32; 4]) {
    for m in 0..nx * nz {
        let (x, z) = ((m % nx) as f32 + 0.5, (m / nx) as f32 + 0.5);
        if over(&[x - JOIN, z - JOIN, x + JOIN, z + JOIN], r) {
            joined[m] = 0;
        }
    }
    let cells = |r: [f32; 4], per: f64, mut f: &mut dyn FnMut(usize)| {
        let lim = |v: f32, n: usize| -> usize { js_min(js_max(floor(v as f64 * per), 0.0), n as f64 * per - 1.0) as usize };
        for z in lim(r[1], nz)..=lim(r[3], nz) {
            for x in lim(r[0], nx)..=lim(r[2], nx) {
                (&mut f)(z * nx * per as usize + x);
            }
        }
    };
    let wide = |by: f32| [r[0] - by, r[1] - by, r[2] + by, r[3] + by];
    cells(*r, 2.0, &mut |i| roof[i] = 0);
    cells(wide(BY), 2.0, &mut |i| touched[i] = now);
    cells(wide(ROUND), 1.0, &mut |m| near[m] = now);
}

/// the first static a line meets, panes of glass let by: how far, its face's normal, which
fn first(w: &World, at: [f64; 3], d: [f64; 3], tmax: f64, tune: &Knobs) -> Option<(f64, [f64; 3], usize)> {
    let (mut gone, mut out) = (0.0, [0.0; 4]);
    for _ in 0..8 {
        let a = [at[0] + d[0] * gone, at[1] + d[1] * gone, at[2] + d[2] * gone];
        let left = tmax - gone;
        if !(left > 0.0) {
            return None;
        }
        let bi = lines::ray(w, a[0], a[1], a[2], d[0], d[1], d[2], left, false, false, &mut out, tune);
        if bi < 0 {
            return None;
        }
        if w.flags[bi as usize] & F_GLASS == 0 {
            return Some((gone + out[0], [out[1], out[2], out[3]], bi as usize));
        }
        gone += out[0] + 0.02;
    }
    None
}
fn roof(c: &mut Cast, w: &World, x: f64, z: f64, tune: &Knobs) -> bool {
    let (jx, jz) = (floor(x * 2.0), floor(z * 2.0));
    if !(jx >= 0.0 && jz >= 0.0 && jx < c.nx as f64 * 2.0 && jz < c.nz as f64 * 2.0) {
        return false;
    }
    let i = jz as usize * c.nx * 2 + jx as usize;
    if c.roof[i] == 0 {
        c.roof[i] = if first(w, [jx * 0.5 + 0.25, 30.0, jz * 0.5 + 0.25], [0.0, -1.0, 0.0], 30.0 - ROOF, tune).is_some() { 2 } else { 1 };
    }
    c.roof[i] == 2
}
/// THE METRE A POINT READS: the nearest whose middle it has a clear line to past the statics, each way (its own, the one it leans to, the other, the
/// corner's); `good` says whether a metre will do at all
fn metre_of(w: &World, nx: usize, nz: usize, p: [f64; 3], mut good: impl FnMut(usize) -> bool, tune: &Knobs) -> Option<usize> {
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
        let clear = |a: [f64; 3], b: [f64; 3]| lines::sight_line(w, false, F_GLASS, a[0], a[1], a[2], b[0], b[1], b[2], tune) == Some(false);
        if good(m) && clear(p, c) && clear(c, p) {
            return Some(m);
        }
    }
    None
}

/// nothing stands higher over a level than this: a line or the sun's way is followed on the plan to there
fn to_top(y: f64, up: f64) -> f64 {
    if up > 0.05 { js_min(FAR, js_max(0.0, TOP - y) / up) } else { FAR }
}
/// A METRE'S LINES OF THE STATICS AS THEY STAND: all of them the first time, then those that pass something that has changed since. → how many were followed
fn cast(c: &mut Cast, w: &World, m: usize, tune: &Knobs) -> usize {
    let (since, mut n) = (c.traced[m], 0);
    if since != c.now {
        for i in 0..RAYS {
            if since == 0 || changed(c, m, i, since) {
                c.rays[m][i] = trace(c, w, m, i, tune);
                n += 1;
            }
        }
        c.traced[m] = c.now;
    }
    n
}
/// whether anything line `i` of metre `m` hangs on has changed since: the way it went, what stands round its end, and the sun's way from there
fn changed(c: &Cast, m: usize, i: usize, since: u32) -> bool {
    let (r, d) = (&c.rays[m][i], ways().d[i]);
    let a = [(m % c.nx) as f64 + 0.5, HEIGHT, (m / c.nx) as f64 + 0.5];
    let cell = |v: f64, per: f64, n: usize| js_min(js_max(floor(v * per), 0.0), n as f64 * per - 1.0) as usize;
    let passed = |x: f64, z: f64| {
        let k = cell(z, 2.0, c.nz) * c.nx * 2 + cell(x, 2.0, c.nx);
        c.half[k] > since || (d[1] < 0.0 && c.low[k] > since)
    };
    let along = |x: f64, z: f64, dx: f64, dz: f64, far: f64| {
        let mut t = 0.0;
        while t < far {
            if passed(x + dx * t, z + dz * t) {
                return true;
            }
            t += PACE;
        }
        passed(x + dx * far, z + dz * far)
    };
    // (a length is kept in whole centimetres, rounded down)
    let end = r.cm as f64 / 100.0 + 0.01;
    // (one that went out and met something is followed again whatever it was: where it ended out there is not kept)
    if r.to == TO_OUTSIDE && r.way & WHOLE > 0 && d[1] > 0.0 {
        return true;
    }
    let far = if r.to >= TO_OUTSIDE && d[1] > 0.0 { js_max(end, to_top(a[1], d[1])) } else { end };
    if along(a[0], a[2], d[0], d[2], far) {
        return true;
    }
    if r.to == TO_SKY || r.to == TO_GROUND {
        return false;
    }
    let h = [a[0] + d[0] * end, a[1] + d[1] * end, a[2] + d[2] * end];
    // (the metre it reads, and the way there: its own, or any of those round it)
    if r.way & OWN != 0 {
        let (k, way) = (r.to as usize, |v: f64, at: usize| at as f64 + 0.5 - v);
        if along(h[0], h[2], way(h[0], k % c.nx), way(h[2], k / c.nx), 1.0) {
            return true;
        }
    } else if c.near[cell(h[2], 1.0, c.nz) * c.nx + cell(h[0], 1.0, c.nx)] > since {
        return true;
    }
    let sun = r.way & WHOLE;
    sun > 0 && along(h[0], h[2], c.to_sun[0], c.to_sun[2], if sun == WHOLE { to_top(h[1], c.to_sun[1]) } else { sun as f64 * 0.5 })
}
/// one line of a metre's, followed among the statics
fn trace(c: &mut Cast, w: &World, m: usize, i: usize, tune: &Knobs) -> Ray {
    let (nx, nz) = (c.nx, c.nz);
    let a = [(m % nx) as f64 + 0.5, HEIGHT, (m / nx) as f64 + 0.5];
    let to_sun = c.to_sun;
    let d = ways().d[i];
    let hit = first(w, a, d, FAR, tune);
    let down = if d[1] < -1e-6 { a[1] / -d[1] } else { FAR };
    // (the floor is a box in most places; where it is not, it is bare)
    let (far, normal, of) = match hit {
        Some((t, n, bi)) if t < down + 0.05 => (t, n, bi as u16 + 1),
        _ => (down, [0.0, 1.0, 0.0], 0),
    };
    let mut out = None;
    let mut t = 0.1;
    while t < far - 0.02 {
        if !roof(c, w, a[0] + d[0] * t, a[2] + d[2] * t, tune) {
            out = Some(t);
            break;
        }
        t += 0.25;
    }
    let end = if let Some(t) = out { t } else { far };
    // the sun's cosine, and how far its way was followed
    let sun_at = |h: [f64; 3], n: [f64; 3]| -> (u8, u8) {
        let ns = n[0] * to_sun[0] + n[1] * to_sun[1] + n[2] * to_sun[2];
        if !(to_sun[1] > 0.0 && ns > 0.0) {
            return (0, 0);
        }
        match first(w, [h[0] + n[0] * 0.02, h[1] + n[1] * 0.02, h[2] + n[2] * 0.02], to_sun, FAR, tune) {
            Some((t, _, _)) => (0, js_min(WHOLE as f64, t * 2.0 + 2.0) as u8),
            None => (js_max(1.0, js_min(255.0, ns * 255.0)) as u8, WHOLE),
        }
    };
    let h = [a[0] + d[0] * far, a[1] + d[1] * far, a[2] + d[2] * far];
    let (to, (sun, way)) = if out.is_some() || far >= FAR {
        if d[1] <= 0.0 {
            (TO_GROUND, (0, 0))
        } else if far >= FAR {
            (TO_SKY, (0, 0))
        } else {
            (TO_OUTSIDE, sun_at(h, normal))
        }
    } else {
        let q = [h[0] + normal[0] * 0.25, js_max(0.3, js_min(2.4, h[1] + normal[1] * 0.25)), h[2] + normal[2] * 0.25];
        match metre_of(w, nx, nz, q, |_| true, tune) {
            // (the metre before it is out under the sky: it is an eave's, or a wall's beside an opening)
            Some(k) if !roof(c, w, (k % nx) as f64 + 0.5, (k / nx) as f64 + 0.5, tune) => (TO_OUTSIDE, sun_at(h, normal)),
            Some(k) => {
                let (sun, way) = sun_at(h, normal);
                (k as u16, (sun, if k == floor(q[2]) as usize * nx + floor(q[0]) as usize { way | OWN } else { way }))
            }
            None => (TO_DARK, (0, 0)),
        }
    };
    Ray { cm: js_min(65535.0, end * 100.0) as u16, to, of, sun, way }
}

/// IS A METRE ONE PLACE WITH THE NEXT, along x or along z: the way between them past the statics, panes among them, and no doorway
fn joined(w: &World, x: f64, z: f64, along_x: bool, tune: &Knobs) -> bool {
    let clear = |a: [f64; 3], b: [f64; 3]| {
        let see = |a: [f64; 3], b: [f64; 3]| lines::sight_line(w, false, 0, a[0], a[1], a[2], b[0], b[1], b[2], tune) == Some(false);
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
fn join(c: &mut Cast, w: &World, m: usize, tune: &Knobs) {
    if c.joined[m] == 0 {
        let (x, z) = ((m % c.nx) as f64 + 0.5, (m / c.nx) as f64 + 0.5);
        c.joined[m] = 4 | if m % c.nx + 1 < c.nx && joined(w, x, z, true, tune) { 1 } else { 0 } | if m / c.nx + 1 < c.nz && joined(w, x, z, false, tune) { 2 } else { 0 };
    }
}
/// THE LEVEL OF EVERY PLACE, some rounds of it: each metre under a roof is pulled to its own reading and, hard, to the metres it is one place with. `open` = the open air's
fn levels(c: &mut Cast, w: &World, st: &mut [f64; LEN], new: bool, rounds: usize, open: f64, tune: &Knobs) {
    let (nx, nz) = (c.nx, c.nz);
    for m in 0..nx * nz {
        join(c, w, m, tune);
    }
    let mut way = c.joined;
    // (a leaf across a way that is no doorway's: a gate, a shutter)
    for i in 0..w.leaves() {
        let at = i * REC;
        let r = &w.leaf[at..at + REC];
        let b = plan_of(r);
        let lim = |v: f32, n: usize| js_min(js_max(floor(v as f64), 0.0), n as f64 - 1.0) as usize;
        for z in lim(b[1] - 1.0, nz)..=lim(b[3] + 1.0, nz) {
            for x in lim(b[0] - 1.0, nx)..=lim(b[2] + 1.0, nx) {
                let m = z * nx + x;
                for (bit, d) in [(1, [1.0, 0.0, 0.0]), (2, [0.0, 0.0, 1.0])] {
                    if way[m] & bit != 0 && lines::ray_box_n(&w.cut, x as f64 + 0.5, HEIGHT, z as f64 + 0.5, d[0], d[1], d[2], r, 1.0).is_some() {
                        way[m] &= !bit;
                    }
                }
            }
        }
    }
    let open = log(js_max(open, NOTHING));
    let pull = REACH * REACH;
    let mut read = [0.0; MAX_GLOW];
    for m in 0..nx * nz {
        read[m] = if roof(c, w, (m % nx) as f64 + 0.5, (m / nx) as f64 + 0.5, tune) { log(js_max(st[O_GLOW_UP + m], NOTHING)) } else { open };
        if new {
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
fn lamps_on(w: &World, k: &K, p: [f64; 3], tune: &Knobs) -> Option<[f64; 6]> {
    let mut acc = [0.0; 6];
    if light::estimate(w, k.lamps, p, &light::NO_SHADE, k.rows, &mut acc, tune) == ASK_HOST {
        return None;
    }
    Some(acc)
}
fn mean(v: &[f64; 6]) -> f64 {
    (v[0] + v[1] + v[2] + v[3] + v[4] + v[5]) / 6.0
}

/// ONE METRE WORKED OUT: what its lines bring of the metres as they are
fn work(c: &mut Cast, w: &World, st: &mut [f64; LEN], m: usize, k: &K, tune: &Knobs) {
    if cast(c, w, m, tune) > 0 {
        unsafe { SWEPT[1] += 1 };
    }
    let a = [(m % c.nx) as f64 + 0.5, HEIGHT, (m / c.nx) as f64 + 0.5];
    // the leaves that hang near enough to be in a line's way
    let (mut near, mut n) = ([0usize; MOST], 0);
    for i in 0..w.leaves() {
        let at = i * REC;
        let (dx, dz) = (w.leaf[at + R_C] - a[0], w.leaf[at + R_C + 2] - a[2]);
        if dx * dx + dz * dz < NEAR * NEAR && n < MOST {
            near[n] = at;
            n += 1;
        }
    }
    let shut = |p: [f64; 3], d: [f64; 3], tmax: f64| near[..n].iter().any(|&at| lines::ray_box_n(&w.cut, p[0], p[1], p[2], d[0], d[1], d[2], &w.leaf[at..at + REC], tmax).is_some());
    let s = &k.sky;
    let open = open_air(s) + s.sun * js_max(0.0, s.to_sun[1]);
    let w_ = ways();
    let mut f = [0.0; 6];
    for (i, r) in c.rays[m].iter().enumerate() {
        let d = w_.d[i];
        let t = r.cm as f64 / 100.0;
        let back = if r.of == 0 { BARE } else { w.back[r.of as usize - 1] as f64 / 255.0 };
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
    if let Some(lamps) = lamps_on(w, k, a, tune) {
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
pub fn step(g: usize, w: &World, st: &mut [f64; LEN], nbx: usize, nbz: usize, k: &K, tune: &Knobs) {
    let (nx, nz) = (nbx * PER, nbz * PER);
    let n = nx * nz;
    let real = |v: f64| v - v == 0.0 && v >= 0.0;
    if n == 0 || n > MAX_GLOW || !(k.sweep >= 1.0) || !(real(k.sky.sun) && real(k.sky.zenith) && real(k.sky.horizon)) {
        return;
    }
    let c = fresh(g, w, nx, nz, Some(k.sky.to_sun));
    let lamps = light::word(k.lamps, k.rows);
    let new = st[O_GLOW_LAMPS] == 0.0;
    // (how many metres are to go at the quick pace; past that, whole sweeps of a new field)
    let quick = n as f64 * ROUNDS;
    if new {
        st[O_GLOW_LEFT] = quick + (n * FIRST) as f64;
    } else if st[O_GLOW_LAMPS] != lamps {
        st[O_GLOW_LEFT] = js_max(st[O_GLOW_LEFT], quick);
    }
    st[O_GLOW_LAMPS] = lamps;
    let young = st[O_GLOW_LEFT] > quick;
    let pace = if st[O_GLOW_LEFT] > 0.0 && QUICK < k.sweep { QUICK } else { k.sweep };
    let per = if young { n } else { (-floor(-(n as f64) / pace)) as usize };
    let mut i = if st[O_GLOW_AT] >= 0.0 && st[O_GLOW_AT] < n as f64 { st[O_GLOW_AT] as usize } else { 0 };
    let left = js_max(0.0, st[O_GLOW_LEFT] - per as f64);
    st[O_GLOW_LEFT] = if young && left <= quick { 0.0 } else { left };
    st[O_GLOW_AT] = ((i + per) % n) as f64;
    unsafe { SWEPT = [per, 0] };
    for _ in 0..per {
        if i >= n {
            i = 0;
        }
        if roof(c, w, (i % nx) as f64 + 0.5, (i / nx) as f64 + 0.5, tune) {
            work(c, w, st, i, k, tune);
        }
        i += 1;
    }
    levels(c, w, st, new, LEVELS[young as usize], open_air(&k.sky), tune);
}
/// what a tick between two steps makes ahead, in lines followed (a metre looked through counts as some)
const AHEAD: usize = 350;
const LOOK: usize = 6;
/// BETWEEN STEPS, some of what the next ones will want, in the order they come by: a static that is written is paid for over the ticks until then. No state is
/// in it: a step finds what was made, or makes it.
pub fn ahead(g: usize, w: &World, st: &[f64; LEN], nbx: usize, nbz: usize, tune: &Knobs) {
    let (nx, nz) = (nbx * PER, nbz * PER);
    let n = nx * nz;
    if n == 0 || n > MAX_GLOW || st[O_GLOW_LAMPS] == 0.0 {
        return;
    }
    let c = fresh(g, w, nx, nz, None);
    if c.made == c.now {
        return;
    }
    let (at, mut done) = (if st[O_GLOW_AT] >= 0.0 && st[O_GLOW_AT] < n as f64 { st[O_GLOW_AT] as usize } else { 0 }, 0);
    for k in 0..n {
        let m = (at + k) % n;
        if c.traced[m] != c.now && roof(c, w, (m % nx) as f64 + 0.5, (m / nx) as f64 + 0.5, tune) {
            done += LOOK + cast(c, w, m, tune);
            if done >= AHEAD {
                return;
            }
        }
    }
    c.made = c.now;
}
/// THE TABLE MADE BEFORE IT IS ASKED FOR, as a game stands: the lines of every metre under a roof and which metres are one place. No state is in it: a step that finds it
/// wanting makes what it wants.
pub fn ready(g: usize, w: &World, nbx: usize, nbz: usize, to_sun: [f64; 3], tune: &Knobs) {
    let (nx, nz) = (nbx * PER, nbz * PER);
    if nx * nz == 0 || nx * nz > MAX_GLOW {
        return;
    }
    let c = fresh(g, w, nx, nz, Some(to_sun));
    for m in 0..nx * nz {
        join(c, w, m, tune);
        if roof(c, w, (m % nx) as f64 + 0.5, (m / nx) as f64 + 0.5, tune) {
            cast(c, w, m, tune);
        }
    }
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
pub fn at(g: usize, w: &World, st: &[f64; LEN], nbx: usize, nbz: usize, p: [f64; 3], tune: &Knobs) -> Option<Field> {
    let (nx, nz) = (nbx * PER, nbz * PER);
    if nx * nz == 0 || nx * nz > MAX_GLOW || st[O_GLOW_LAMPS] == 0.0 {
        return None;
    }
    let c = fresh(g, w, nx, nz, None);
    if !roof(c, w, p[0], p[2], tune) {
        return None;
    }
    let m = metre_of(w, nx, nz, p, |m| roof(c, w, (m % nx) as f64 + 0.5, (m / nx) as f64 + 0.5, tune), tune)?;
    Some(Field { mean: st[O_GLOW_MEAN + m], best: st[O_GLOW_MAX + m], place: exp(st[O_GLOW_PLACE + m]) })
}
