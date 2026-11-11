//! THE LIGHT ON A POINT: the direct light of every lamp in reach on the six faces of the meter, each lamp shadowed by one line to it. The hour's sun and sky and a stroke of
//! lightning are added to the same six faces afterwards (game_rules guards light_at).
//!
//! WHERE A QUESTION'S LAMPS COME FROM. The lamps the level's fixtures feed are READ WHERE THEY STAND: the rows of the lights' part of the game's block
//! (lights.rs), in the order of the engine's list, each through the reader's own gates (`row`). Every other lamp (a man's torch, a flash, a tracer's
//! lamp) is the question's own: the host writes it into the scratch table here, having passed it by its own gates (whose it is,
//! passing or not), with how many rows stand before it in the list. The two are taken as ONE LIST IN THAT ORDER (`each`), so the sum of floats over
//! them is the sum a single loop over the engine's list makes.
//!
//! The cosines and the lines' lengths are the game's own, so this answers only where EXACT.on.

use game_types::knobs::Knobs;
use crate::bricks;
use crate::lights::{Lights, ROWS, R_DAY, R_ON, R_STEADY};
use crate::lines;
use crate::state::state;
use crate::maths::cos;
use crate::world::{ASK_HOST, clamp, js_max, js_max3, js_min, smoothstep, DEG, F_GLASS, World};
use prim::sqrt;

/// DOUBLES A LAMP OF THE QUESTION'S OWN: where it is (x y z), its range, its radius, its kind (0 point · 1 spot · 3 panel), where it points (x y z), its
/// outer and inner half-angles in degrees, its colour (r g b), its intensity, and how many of the level's rows stand before it in the list
pub const LAMP: usize = 16;
const L_POS: usize = 0;
const L_RANGE: usize = 3;
const L_RADIUS: usize = 4;
const L_KIND: usize = 5;
const L_DIR: usize = 6;
const L_OUTER: usize = 9;
const L_INNER: usize = 10;
const L_COLOR: usize = 11;
const L_I: usize = 14;
const L_ROWS: usize = 15;

/// WHAT A QUESTION SAYS OF THE LEVEL'S ROWS: whose block they are in (a slot), which it leaves out (lights.rs R_* bits: who asks what the PLACE lights
/// leaves the passing ones, a sleeper's face the screens'), and whether a row with a steady self is taken as that self (a fire's: what the place lights)
#[derive(Clone, Copy)]
pub struct Rows {
    pub g: usize,
    pub leave: u32,
    pub steady: bool,
    /// the hour's exposure (game_rules hours; 1 = a night): how strong each is beside it (`by_day`)
    pub exposure: f64,
}
/// WHAT A LAMP IS, for how strong it stands beside an hour's light: none of these (as it is: the hour's own, a device's tell) · a fixture · a light in a hand · a
/// vehicle's · a flash or a fire · a glowing face
pub const DAY_NONE: u32 = 0;
pub const DAY_FIXTURE: u32 = 1;
pub const DAY_HAND: u32 = 2;
pub const DAY_VEHICLE: u32 = 3;
pub const DAY_FLASH: u32 = 4;
pub const DAY_FACE: u32 = 5;
/// A LAMP BESIDE THE HOUR. A level's lamps are those of a night, and a night's one unit is 3.3 lux. An hour's sun and sky are the real ones times that hour's
/// exposure, so a lamp's real strength times it is what it counts for: what a fixture gives is real as authored, what a hand, a vehicle, a flash or a glowing
/// face gives was held down for the night to be bearable, and is never more than that. `intensity` is the lamp's now, `full` its whole.
#[inline(always)]
pub fn by_day(what: u32, intensity: f64, full: f64, exposure: f64) -> f64 {
    if exposure >= 1.0 {
        return intensity;
    }
    let full = if full > 0.0 { full } else { intensity };
    let real = match what {
        DAY_FIXTURE => 1.0,
        DAY_HAND => 10000.0 / 3.3 / full,
        DAY_VEHICLE => 15000.0 / 3.3 / full,
        DAY_FLASH => 6.0,
        DAY_FACE => 50.0,
        _ => return intensity,
    };
    intensity * js_min(1.0, js_max(real, 1.0) * exposure)
}
/// a lamp as the sum takes it, whichever of the two it came from
#[derive(Clone, Copy)]
struct Lamp {
    pos: [f64; 3],
    range: f64,
    radius: f64,
    kind: f64,
    dir: [f64; 3],
    outer: f64,
    inner: f64,
    color: [f64; 3],
    int: f64,
}
/// a lamp of the question's own, as the host wrote it
#[inline(always)]
fn carried(l: &[f64]) -> Lamp {
    Lamp { pos: [l[L_POS], l[L_POS + 1], l[L_POS + 2]], range: l[L_RANGE], radius: l[L_RADIUS], kind: l[L_KIND], dir: [l[L_DIR], l[L_DIR + 1], l[L_DIR + 2]], outer: l[L_OUTER], inner: l[L_INNER], color: [l[L_COLOR], l[L_COLOR + 1], l[L_COLOR + 2]], int: l[L_I] }
}
/// THE READER'S GATES ON A ROW (guards.ts lampAsked's, for a lamp nobody carries): it burns, and NOTHING ELSE OF A ROW THAT IS OUT IS READ (what feeds
/// it may have stopped writing it: a fire's lamp that is out is the picture's alone) · the question does not leave it out · its first self has a
/// strength above nothing · it is no moon (kind 2 has the sun's own shadow test, hours.ts). Then the self the question takes: the steady one where it
/// asks for those and the row has one (its place, size and strength; the rest is the lamp's).
#[inline(always)]
fn row(l: &Lights, r: usize, q: &Rows) -> Option<Lamp> {
    let b = l.row_bits[r];
    if b & R_ON == 0 {
        return None;
    }
    if b & q.leave != 0 || l.intensity[r] <= 0.0 || l.kind[r] == 2.0 {
        return None;
    }
    let s = q.steady && b & R_STEADY != 0;
    let int = if s { l.s_intensity[r] } else { l.intensity[r] };
    Some(Lamp {
        pos: if s { [l.sx[r], l.sy[r], l.sz[r]] } else { [l.x[r], l.y[r], l.z[r]] },
        range: l.range[r],
        radius: if s { l.s_radius[r] } else { l.radius[r] },
        kind: l.kind[r],
        dir: [l.dx[r], l.dy[r], l.dz[r]],
        outer: l.outer[r],
        inner: l.inner[r],
        color: [l.r[r], l.g[r], l.b[r]],
        int: by_day(b >> R_DAY & 7, int, int, q.exposure),
    })
}
/// THE ONE LIST: the question's own lamps and the rows `rows` names, in the order of the engine's list: before each of its own come the rows that
/// stand before it (its own count, never past the table's), and after the last the rows that are left. `f` is handed each lamp that passed; what it says
/// other than 0 ends the walk and is returned.
#[inline(always)]
fn each(lamps: &[[f64; LAMP]], rows: Option<Rows>, mut f: impl FnMut(&Lamp) -> u32) -> u32 {
    let table: Option<(&Lights, Rows)> = rows.map(|q| (&state(q.g).lights, q));
    let total = match table {
        Some((l, _)) => {
            let t = l.rows as usize;
            if t > ROWS { ROWS } else { t }
        }
        None => 0,
    };
    let mut at = 0;
    for c in lamps {
        let before = c[L_ROWS];
        let to = if before >= total as f64 { total } else if before > 0.0 { before as usize } else { 0 };
        let said = rows_to(table, to, &mut at, &mut f);
        if said != 0 {
            return said;
        }
        let said = f(&carried(c));
        if said != 0 {
            return said;
        }
    }
    rows_to(table, total, &mut at, &mut f)
}
/// …the rows from `at` up to `to`, each that passes the reader's gates
#[inline(always)]
fn rows_to<F: FnMut(&Lamp) -> u32>(table: Option<(&Lights, Rows)>, to: usize, at: &mut usize, f: &mut F) -> u32 {
    if let Some((l, q)) = table {
        while *at < to {
            let r = *at;
            *at += 1;
            if let Some(lamp) = row(l, r, &q) {
                let said = f(&lamp);
                if said != 0 {
                    return said;
                }
            }
        }
    }
    0
}

/// THE WORD OF THE LAMPS A SWEEP TAKES (the question's own and the rows, as one list): which lamps they are and how strong (kind, reach, size,
/// colour, strength), and nothing of where they are or point. It changes when a lamp is shot out, a breaker is thrown or a fixture is dimmed, and not
/// when a lamp sweeps (glow.rs sweeps the whole plan again when it changes). Folded anew a sweep, from the rows as they stand: nothing of it is kept
/// with the lights. Never 0.
pub fn word(lamps: &[[f64; LAMP]], rows: Option<Rows>) -> f64 {
    let mut count = 0usize;
    each(lamps, rows, |_| {
        count += 1;
        0
    });
    let (mut s, mut i) = (1.0 + count as f64, 0usize);
    each(lamps, rows, |l| {
        let own = l.kind + 3.0 * l.range + 5.0 * l.radius + 7.0 * l.int + 11.0 * l.color[0] + 13.0 * l.color[1] + 17.0 * l.color[2];
        s = s * 0.75 + own * (1.0 + (i % 61) as f64);
        i += 1;
        0
    });
    s
}
/// THE SMOKE'S SHADOW of a question, `sigma` > 0: a lamp no wall hides is dimmed by the smoke of the coarse grid of the game in slot `g` (`stamp`: its block's) along the
/// line to it, under that `sigma` (extinction per metre per unit of density). 0 = no shadow.
pub struct Shade {
    pub sigma: f64,
    pub g: usize,
    pub stamp: f64,
}
pub const NO_SHADE: Shade = Shade { sigma: 0.0, g: 0, stamp: 0.0 };

/// the six axis normals the renderer's meter takes its best face over
const N6: [[f64; 3]; 6] = [[1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, -1.0]];

/// (1 − (d/range)⁴)², clamped
#[inline(always)]
fn light_window(d: f64, range: f64) -> f64 {
    let x = d / range;
    let w = clamp(1.0 - x * x * x * x, 0.0, 1.0);
    w * w
}

/// THE LAMPS ON THE POINT `p` (`each`: the question's own and the rows `rows` names, as one list): the six faces go to `acc`. 0, or ASK_HOST
/// (nothing in `acc` is then to be read).
#[allow(clippy::too_many_arguments)]
pub fn estimate(w: &World, lamps: &[[f64; LAMP]], p: [f64; 3], shade: &Shade, rows: Option<Rows>, acc: &mut [f64; 6], tune: &Knobs) -> u32 {
    *acc = [0.0; 6];
    // (a lamp this dim on the point is no lamp; beside an hour's light, at which eyes are used to as much less)
    let least = 0.002 * rows.map_or(1.0, |q| q.exposure);
    each(lamps, rows, |l| one(w, p, shade, l, least, acc, tune))
}
/// THE LAMPS OF A QUESTION ASKED OF MANY POINTS, said once
pub struct Set {
    n: usize,
    least: f64,
    lamps: [(Lamp, Cone); MOST],
}
pub const MOST: usize = ROWS + 64;
/// a spot's two cosines
#[derive(Clone, Copy)]
struct Cone {
    outer: f64,
    inner: f64,
}
const NO_LAMP: Lamp = Lamp { pos: [0.0; 3], range: 0.0, radius: 0.0, kind: 0.0, dir: [0.0; 3], outer: 0.0, inner: 0.0, color: [0.0; 3], int: 0.0 };
static mut SET: Set = Set { n: 0, least: 0.0, lamps: [(NO_LAMP, Cone { outer: 0.0, inner: 0.0 }); MOST] };
/// (scratch of one call's points: the next set is laid over it)
pub fn set(lamps: &[[f64; LAMP]], rows: Option<Rows>) -> &'static Set {
    let s = unsafe { &mut *core::ptr::addr_of_mut!(SET) };
    s.least = 0.002 * rows.map_or(1.0, |q| q.exposure);
    s.n = 0;
    each(lamps, rows, |l| {
        if s.n < MOST {
            s.lamps[s.n] = (*l, cone(l));
            s.n += 1;
        }
        0
    });
    s
}
impl Set {
    /// a word of where each lamp stands, and how far from there its light begins
    pub fn places(&self) -> impl Iterator<Item = u64> + '_ {
        self.lamps[..self.n].iter().map(|(l, _)| l.pos.iter().chain([&l.radius]).fold(0xcbf29ce484222325u64, |h, v| (h ^ v.to_bits()).wrapping_mul(0x9e3779b97f4a7c15).rotate_left(29)) | 1)
    }
    /// the six faces at a point. `shut`: whether the line to lamp `i` is (from the point to just short of it), None = the host's to judge, and so is this
    pub fn estimate(&self, p: [f64; 3], mut shut: impl FnMut(usize, [f64; 3]) -> Option<bool>) -> Option<[f64; 6]> {
        let mut acc = [0.0; 6];
        for (i, (l, c)) in self.lamps[..self.n].iter().enumerate() {
            if let Some((lum, at, end)) = reaches(l, || *c, p, self.least) {
                match end {
                    Some(end) if shut(i, end)? => continue,
                    _ => {}
                }
                onto(&mut acc, lum, at);
            }
        }
        Some(acc)
    }
}
fn cone(l: &Lamp) -> Cone {
    Cone { outer: cos(l.outer * DEG), inner: cos(js_min(l.inner, l.outer - 0.5) * DEG) }
}
/// one lamp's light on the point, onto the six faces: the shader's own terms, then one line to it, then the receiving cosine a face. 0, or ASK_HOST.
#[inline(always)]
fn one(w: &World, p: [f64; 3], shade: &Shade, l: &Lamp, least: f64, acc: &mut [f64; 6], tune: &Knobs) -> u32 {
    let Some((mut lum, at, end)) = reaches(l, || cone(l), p, least) else { return 0 };
    if let Some(end) = end {
        match lines::sight_line(w, true, F_GLASS, p[0], p[1], p[2], end[0], end[1], end[2], tune) {
            None => return ASK_HOST,
            Some(true) => return 0,
            Some(false) => {}
        }
        if shade.sigma > 0.0 {
            lum *= bricks::trans(shade.g, shade.stamp, p, end, shade.sigma);
        }
    }
    onto(acc, lum, at);
    0
}
/// what of a lamp comes to a point: how much, which way the lamp is, and where the line to it ends (None: it is at hand)
#[inline(always)]
fn reaches(l: &Lamp, cone: impl Fn() -> Cone, p: [f64; 3], least: f64) -> Option<(f64, [f64; 3], Option<[f64; 3]>)> {
    let (dx, dy, dz) = (l.pos[0] - p[0], l.pos[1] - p[1], l.pos[2] - p[2]);
    let range = l.range;
    // (a lamp further off along one axis than its range is further off than its range: the test below would pass it by, at the price of a root)
    if dx > range || -dx > range || dy > range || -dy > range || dz > range || -dz > range {
        return None;
    }
    let dist2 = dx * dx + dy * dy + dz * dz;
    let dist = sqrt(dist2);
    if dist > range || dist < 1e-3 {
        return None;
    }
    let (lx, ly, lz) = (dx / dist, dy / dist, dz / dist);
    let lr = l.radius;
    let mut att = light_window(dist, range) / js_max(dist2, lr * lr);
    let kind = l.kind;
    if kind == 1.0 {
        let cd = -(lx * l.dir[0] + ly * l.dir[1] + lz * l.dir[2]);
        let Cone { outer: cos_outer, inner: cos_inner } = cone();
        if cd < cos_outer {
            return None;
        }
        att *= smoothstep(cos_outer, cos_inner, cd);
    } else if kind == 3.0 {
        let ce = -(lx * l.dir[0] + ly * l.dir[1] + lz * l.dir[2]);
        if ce <= 0.0 {
            return None;
        }
        att *= ce;
    }
    if att <= 0.0 {
        return None;
    }
    let (c0, c1, c2, int) = (l.color[0], l.color[1], l.color[2], l.int);
    if js_max3(c0, c1, c2) * int * att < least {
        return None;
    }
    let lum = (0.2126 * c0 + 0.7152 * c1 + 0.0722 * c2) * int * att;
    let tmax = dist - lr - 0.02;
    let end = if tmax > 0.05 { Some([p[0] + lx * tmax, p[1] + ly * tmax, p[2] + lz * tmax]) } else { None };
    Some((lum, [lx, ly, lz], end))
}
/// light from that way onto the six faces
#[inline(always)]
fn onto(acc: &mut [f64; 6], lum: f64, [lx, ly, lz]: [f64; 3]) {
    for k in 0..6 {
        let nn = &N6[k];
        let cs = nn[0] * lx + nn[1] * ly + nn[2] * lz;
        if cs > 0.0 {
            acc[k] += lum * cs;
        }
    }
}
