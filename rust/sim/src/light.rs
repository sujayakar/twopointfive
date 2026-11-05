//! THE LIGHT ON A POINT — src/game/guards.ts lightEstimateAt: the direct light of every lamp in reach on the six faces of the meter, each lamp shadowed by
//! one line to it. The hour's sun and sky and a stroke of lightning are added to the same six faces by the host afterwards (hours.ts hourLight,
//! weather.ts strokeLight), as they were.
//!
//! WHERE A QUESTION'S LAMPS COME FROM. The lamps the level's fixtures feed are READ WHERE THEY STAND: the rows of the lights' part of the game's block
//! (lights.rs), in the order of the engine's list, each through the reader's own gates (`row`). Every other lamp (a man's torch, a flash, a tracer's
//! lamp) is the question's own: the host writes it into the scratch table here, having passed it by its own gates (whose it is,
//! passing or not), with how many rows stand before it in the list. The two are taken as ONE LIST IN THAT ORDER (`each`), so the sum of floats over
//! them is the sum a single loop over the engine's list makes.
//!
//! The cosines and the lines' lengths are the game's own, so this answers only where EXACT.on.

use crate::bricks;
use crate::lights::{Lights, ROWS, R_ON, R_STEADY};
use crate::lines;
use crate::smoke;
use crate::state::state;
use crate::maths::cos;
use crate::world::{ASK_HOST, clamp, js_max, js_max3, js_min, smoothstep, DEG, F_GLASS};
use prim::sqrt;

/// lamps of its own ONE PART of a question can carry (the level's are not carried). More of them than this in reach of a point are asked IN PARTS: the
/// host writes the next and asks again with `more`, and the six faces and the place among the rows go on from where the part before left them (`estimate`)
pub const MAX_LAMPS: usize = 256;
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

static mut LAMPS: [f64; MAX_LAMPS * LAMP] = [0.0; MAX_LAMPS * LAMP];

/// WHAT A QUESTION SAYS OF THE LEVEL'S ROWS: whose block they are in (a slot), which it leaves out (lights.rs R_* bits: who asks what the PLACE lights
/// leaves the passing ones, a sleeper's face the screens'), and whether a row with a steady self is taken as that self (a fire's: what the place lights)
#[derive(Clone, Copy)]
pub struct Rows {
    pub g: usize,
    pub leave: u32,
    pub steady: bool,
}
/// a lamp as the sum takes it, whichever of the two it came from
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
    Some(Lamp {
        pos: if s { [l.sx[r], l.sy[r], l.sz[r]] } else { [l.x[r], l.y[r], l.z[r]] },
        range: l.range[r],
        radius: if s { l.s_radius[r] } else { l.radius[r] },
        kind: l.kind[r],
        dir: [l.dx[r], l.dy[r], l.dz[r]],
        outer: l.outer[r],
        inner: l.inner[r],
        color: [l.r[r], l.g[r], l.b[r]],
        int: if s { l.s_intensity[r] } else { l.intensity[r] },
    })
}
/// THE ONE LIST: the first `n` lamps written and the rows `rows` names, in the order of the engine's list — before each written lamp come the rows that
/// stand before it (its own count, never past the table's), and after the last written lamp of the LAST part the rows that are left. `at` is the place
/// among the rows, the caller's to keep from part to part. `f` is handed each lamp that passed; what it says other than 0 ends the walk and is returned.
#[inline(always)]
fn each(n: usize, rows: Option<Rows>, last: bool, at: &mut usize, mut f: impl FnMut(&Lamp) -> u32) -> u32 {
    let lamps = unsafe { &*core::ptr::addr_of!(LAMPS) };
    let table: Option<(&Lights, Rows)> = rows.map(|q| (&state(q.g).lights, q));
    let total = match table {
        Some((l, _)) => {
            let t = l.rows as usize;
            if t > ROWS { ROWS } else { t }
        }
        None => 0,
    };
    for i in 0..n {
        let c = &lamps[i * LAMP..(i + 1) * LAMP];
        let before = c[L_ROWS];
        let to = if before >= total as f64 { total } else if before > 0.0 { before as usize } else { 0 };
        let said = rows_to(table, to, at, &mut f);
        if said != 0 {
            return said;
        }
        let said = f(&carried(c));
        if said != 0 {
            return said;
        }
    }
    if last {
        return rows_to(table, total, at, &mut f);
    }
    0
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

/// THE WORD OF THE LAMPS A SWEEP TAKES (the first `n` written and the rows, as one list): which lamps they are and how strong (kind, reach, size,
/// colour, strength), and nothing of where they are or point. It changes when a lamp is shot out, a breaker is thrown or a fixture is dimmed, and not
/// when a lamp sweeps (glow.rs sweeps the whole plan again when it changes). Folded anew a sweep, from the rows as they stand: nothing of it is kept
/// with the lights. Never 0.
pub fn word(n: usize, rows: Option<Rows>) -> f64 {
    let n = if n > MAX_LAMPS { 0 } else { n };
    let (mut count, mut at) = (0usize, 0usize);
    each(n, rows, true, &mut at, |_| {
        count += 1;
        0
    });
    let (mut s, mut i, mut at) = (1.0 + count as f64, 0usize, 0usize);
    each(n, rows, true, &mut at, |l| {
        let own = l.kind + 3.0 * l.range + 5.0 * l.radius + 7.0 * l.int + 11.0 * l.color[0] + 13.0 * l.color[1] + 17.0 * l.color[2];
        s = s * 0.75 + own * (1.0 + (i % 61) as f64);
        i += 1;
        0
    });
    s
}
/// where a question's lamps are written (a byte address)
pub fn lamps_addr() -> usize {
    core::ptr::addr_of!(LAMPS) as usize
}

/// the six faces of the question being asked and its place among the rows, kept from one part of its lamps to the next (`asked`): a question is asked
/// whole, part after part, before another is begun, and whatever else the host asks of the module between two parts leaves these alone
static mut SUM: [f64; 6] = [0.0; 6];
static mut AT: usize = 0;
/// `estimate` for the host's question (lib.rs light_estimate): the faces are summed in SUM and, as each part ends, copied to `out` for the host to read
/// after the last
#[allow(clippy::too_many_arguments)]
pub fn asked(w: usize, set: i32, n: usize, p: [f64; 3], shade: &Shade, more: bool, rows: Option<Rows>, last: bool, out: &mut [f64]) -> u32 {
    let (sum, at) = unsafe { (&mut *core::ptr::addr_of_mut!(SUM), &mut *core::ptr::addr_of_mut!(AT)) };
    let r = estimate(w, set, n, p, shade, more, rows, last, at, sum);
    out[..6].copy_from_slice(sum);
    r
}
/// THE SMOKE'S SHADOW of a question (sense.ts SENSE.bounce), `sigma` > 0: a lamp no wall hides is dimmed by the smoke of the coarse grid of the game
/// in slot `g`, whose block's stamp is `stamp`, along the very line the wall test took (bricks.rs trans under this much extinction a metre a unit of
/// density). WHERE THE GRID CANNOT ANSWER THAT LINE — it holds more smoke than bricks, or `stands` is false: the host's last step found no grid
/// standing on this level — the game's blobs do, as sense.ts CpuSmoke.along has it: the first `nb` blobs the host wrote (smoke.rs: none = clear air)
/// under the model's numbers `k` (spread, widest, rise, rise rate; the extinction is `sigma`). 0 = no shadow, and none of the rest is read.
pub struct Shade {
    pub sigma: f64,
    pub g: usize,
    pub stamp: f64,
    pub stands: bool,
    pub nb: usize,
    pub k: [f64; 4],
}
pub const NO_SHADE: Shade = Shade { sigma: 0.0, g: 0, stamp: 0.0, stands: false, nb: 0, k: [0.0; 4] };

/// the six axis normals the renderer's meter takes its best face over (guards.ts METER_NORMALS)
const N6: [[f64; 3]; 6] = [[1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, -1.0]];

/// guards.ts lightWindow: (1 − (d/range)⁴)², clamped
#[inline(always)]
fn light_window(d: f64, range: f64) -> f64 {
    let x = d / range;
    let w = clamp(1.0 - x * x * x * x, 0.0, 1.0);
    w * w
}

/// THE LAMPS ON THE POINT `p` (`each`: the first `n` written and the rows `rows` names, as one list): the six faces go to `acc`. 0, or ASK_HOST
/// (nothing in `acc` is then to be read).
/// IN PARTS: `more` = the written lamps are a later part of the question the call before began — `acc` and `at` are left as that part left them and
/// gone on from, so a list of any length is one sum in one order, the bits of a single loop over it. `last` = no part follows: the rows after the last
/// written lamp are taken. The faces are read after the last part alone.
#[allow(clippy::too_many_arguments)]
pub fn estimate(w: usize, set: i32, n: usize, p: [f64; 3], shade: &Shade, more: bool, rows: Option<Rows>, last: bool, at: &mut usize, acc: &mut [f64]) -> u32 {
    if !more {
        for a in acc[..6].iter_mut() {
            *a = 0.0;
        }
        *at = 0;
    }
    if n > MAX_LAMPS {
        return ASK_HOST;
    }
    each(n, rows, last, at, |l| one(w, set, p, shade, l, acc))
}
/// one lamp's light on the point, onto the six faces: the shader's own terms, then one line to it, then the receiving cosine a face. 0, or ASK_HOST.
#[inline(always)]
fn one(w: usize, set: i32, p: [f64; 3], shade: &Shade, l: &Lamp, acc: &mut [f64]) -> u32 {
    let (dx, dy, dz) = (l.pos[0] - p[0], l.pos[1] - p[1], l.pos[2] - p[2]);
    let range = l.range;
    // (a lamp further off along one axis than its range is further off than its range: the test below would pass it by, at the price of a root)
    if dx > range || -dx > range || dy > range || -dy > range || dz > range || -dz > range {
        return 0;
    }
    let dist2 = dx * dx + dy * dy + dz * dz;
    let dist = sqrt(dist2);
    if dist > range || dist < 1e-3 {
        return 0;
    }
    let (lx, ly, lz) = (dx / dist, dy / dist, dz / dist);
    let lr = l.radius;
    let mut att = light_window(dist, range) / js_max(dist2, lr * lr);
    let kind = l.kind;
    if kind == 1.0 {
        let cd = -(lx * l.dir[0] + ly * l.dir[1] + lz * l.dir[2]);
        let cos_outer = cos(l.outer * DEG);
        let cos_inner = cos(js_min(l.inner, l.outer - 0.5) * DEG);
        if cd < cos_outer {
            return 0;
        }
        att *= smoothstep(cos_outer, cos_inner, cd);
    } else if kind == 3.0 {
        let ce = -(lx * l.dir[0] + ly * l.dir[1] + lz * l.dir[2]);
        if ce <= 0.0 {
            return 0;
        }
        att *= ce;
    }
    if att <= 0.0 {
        return 0;
    }
    let (c0, c1, c2, int) = (l.color[0], l.color[1], l.color[2], l.int);
    if js_max3(c0, c1, c2) * int * att < 0.002 {
        return 0;
    }
    let mut lum = (0.2126 * c0 + 0.7152 * c1 + 0.0722 * c2) * int * att;
    let tmax = dist - lr - 0.02;
    if tmax > 0.05 {
        let end = [p[0] + lx * tmax, p[1] + ly * tmax, p[2] + lz * tmax];
        match lines::sight_line(w, set, F_GLASS, p[0], p[1], p[2], end[0], end[1], end[2]) {
            None => return ASK_HOST,
            Some(true) => return 0,
            Some(false) => {}
        }
        if shade.sigma > 0.0 {
            let mut t = if shade.stands { bricks::trans(shade.g, shade.stamp, p, end, shade.sigma) } else { -1.0 };
            if t < 0.0 {
                t = 1.0;
                if shade.nb > 0 && smoke::trans(w, set, shade.nb, p, end, shade.sigma, shade.k[0], shade.k[1], shade.k[2], shade.k[3], &mut t) == ASK_HOST {
                    return ASK_HOST;
                }
            }
            lum *= t;
        }
    }
    for k in 0..6 {
        let nn = &N6[k];
        let cs = nn[0] * lx + nn[1] * ly + nn[2] * lz;
        if cs > 0.0 {
            acc[k] += lum * cs;
        }
    }
    0
}
