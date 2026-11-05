//! THE LIGHT THAT COMES ROUND — what `SENSE.bounce` adds to the light on a man (src/game/sense.ts; guards.ts lightEstimateAt): the lamps' light that reaches
//! him off the walls, the floor and the ceiling, which the estimate of the direct light has never had (a room lit only round a corner reads dark to it and
//! is not dark in the picture). It is a MODEL of the picture's bounce, as the coarse grid is a model of its smoke, and a coarse one on purpose: one number
//! a SQUARE METRE of the plan (four to a square of the smoke grid's), kept in that grid's state row (bricks.rs: a part of the game's block).
//!
//! WHAT A METRE HOLDS (bricks.rs O_GLOW_…): the LAMPS' LIGHT at its middle at chest height (the estimate of the direct light there: what there is to
//! bounce), the BOUNCED light there, and whether THE WAY IS OPEN from its middle to the next one's toward +x and toward +z (one line, asked both ways
//! as the grid asks its faces: a wall between two middles shuts the light in, a doorway lets it by). So every metre is of the room its middle is in, and
//! whoever reads the field takes the nearest metre whose middle he has a clear line to (sense.ts SmokeGrid.glowAt): a man behind a wall of a lit room
//! does not stand in its bounce.
//! A STEP (a tenth of a second, after the smoke's): a slice of the metres is SWEPT (their lamps' light and their ways are worked out again, so the
//! whole plan is as it stands every `sweep` steps: a door swung, a wall blown, a lamp that moves are in the bounce within that), and the WHOLE plan in
//! the step that finds other lamps than it was last swept whole under (light.rs word: a new field, a lamp shot out, a breaker thrown), then the bounce is RELAXED
//! `passes` times, every metre at once: a metre's bounce is `albedo` of the light there, direct and bounced, of which it keeps what its shut sides do
//! not let out and hands `share` across each open side.
//! Nothing here is kept but those four numbers a metre and the lamps' word, and which metres a step sweeps is the count of steps: the same bits on every machine, and a game
//! put back from a snapshot steps as one that never was.
//!
//! WHAT IT LEAVES OUT, named: daylight (the hour's sun and sky have their own estimate, and bounce none here), a lamp somebody carries (it moves faster
//! than a sweep), a flash, and the smoke (the bounce is not dimmed by it; the direct light is, in light.rs).

use crate::bricks::{clear_but, H_STEPS, LEN, MAX_GLOW, O_GLOW_B, O_GLOW_D, O_GLOW_LAMPS, O_GLOW_X, O_GLOW_Z, O_RHO, SIDE};
use crate::light;
use crate::world::{js_max, ASK_HOST, F_GLASS};
use prim::floor;

/// what a step is told (sense.ts SENSE.glow), and the lamps it is swept from: what the host says of the level's rows (light.rs Rows: the steady ones
/// and their steady selves) and how many lamps of nobody's fixture it wrote beside them (light.rs LAMPS)
pub struct K {
    pub albedo: f64,
    pub share: f64,
    pub passes: u32,
    pub sweep: f64,
    pub lamps: usize,
    pub rows: Option<light::Rows>,
}
/// the height the light is read at (a man's chest), a metre of the plan, and how many of those a square of the smoke grid is across
pub const CHEST: f64 = 1.2;
pub const METRE: f64 = 1.0;
const PER: usize = (SIDE / METRE) as usize;

/// the most passes a step takes, whatever it is told (a number from a knob table is not trusted with the tick)
pub const MAX_PASSES: u32 = 16;

/// one pass's answers, written before any is read back
static mut NEXT: [f64; MAX_GLOW] = [0.0; MAX_GLOW];

/// the brightest of the meter's six faces under the sweep's lamps at a point (None: a line of it is the host's to judge)
fn lamps_on(w: usize, set: i32, k: &K, p: [f64; 3]) -> Option<f64> {
    let (mut acc, mut at) = ([0.0; 6], 0usize);
    if light::estimate(w, set, k.lamps, p, &light::NO_SHADE, false, k.rows, true, &mut at, &mut acc) == ASK_HOST {
        return None;
    }
    let mut m = acc[0];
    for k in 1..6 {
        m = js_max(m, acc[k]);
    }
    Some(m)
}

/// is the way open between two middles a metre apart: no wall and no leaf on the line, asked both ways, and a pane lets light by (the lamps' own line in
/// light.rs passes glass, and so does the line a reader takes to a middle)
fn open_metre(w: usize, set: i32, a: [f64; 3], b: [f64; 3]) -> bool {
    clear_but(w, set, a, b, METRE, F_GLASS) && clear_but(w, set, b, a, METRE, F_GLASS)
}
/// A STEP WITH THE LIGHT MODEL DOWN: a field that was swept is no field (its doors and lamps would be old ones the day the model is raised again)
pub fn down(st: &mut [f64; LEN]) {
    if st[O_GLOW_LAMPS] != 0.0 {
        for v in st[O_GLOW_D..O_RHO].iter_mut() {
            *v = 0.0;
        }
    }
}

/// ONE STEP over a plan of `nbx` by `nbz` squares of the smoke grid, two metres to a square's side (a plan of more metres than the columns hold has no
/// bounce: the columns stay as they were)
pub fn step(w: usize, set: i32, st: &mut [f64; LEN], nbx: usize, nbz: usize, k: &K) {
    let (nx, nz) = (nbx * PER, nbz * PER);
    let n = nx * nz;
    if n == 0 || n > MAX_GLOW || !(k.sweep >= 1.0) || !(k.albedo >= 0.0 && k.albedo < 1.0) || !(k.share >= 0.0 && k.share <= 0.25) {
        return;
    }
    // 1. the sweep: this step's slice of the metres, by the count of steps; ALL of them where the lamps are not the ones the plan was last swept whole
    // under (a new field, a lamp shot out, a breaker thrown: the light that comes round does not outlast the lamp it came from)
    let lamps = light::word(k.lamps, k.rows);
    let whole = st[O_GLOW_LAMPS] != lamps;
    st[O_GLOW_LAMPS] = lamps;
    let per = if whole { n as f64 } else { -floor(-(n as f64) / k.sweep) };
    let turn = st[H_STEPS] - floor(st[H_STEPS] / k.sweep) * k.sweep;
    let from = if whole { 0.0 } else { turn * per };
    let mut f = from;
    while f < from + per && f < n as f64 {
        let i = f as usize;
        f += 1.0;
        let (ix, iz) = (i % nx, i / nx);
        let c = [(ix as f64 + 0.5) * METRE, CHEST, (iz as f64 + 0.5) * METRE];
        if let Some(v) = lamps_on(w, set, k, c) {
            st[O_GLOW_D + i] = v;
        }
        st[O_GLOW_X + i] = if ix + 1 < nx && open_metre(w, set, c, [c[0] + METRE, c[1], c[2]]) { 1.0 } else { 0.0 };
        st[O_GLOW_Z + i] = if iz + 1 < nz && open_metre(w, set, c, [c[0], c[1], c[2] + METRE]) { 1.0 } else { 0.0 };
    }
    // 2. the bounce, every metre from the metres as the pass found them
    let next = unsafe { &mut *core::ptr::addr_of_mut!(NEXT) };
    for _ in 0..if k.passes > MAX_PASSES { MAX_PASSES } else { k.passes } {
        for i in 0..n {
            let (bx, bz) = (i % nx, i / nx);
            let here = st[O_GLOW_D + i] + st[O_GLOW_B + i];
            let mut kept = 1.0;
            let mut got = 0.0;
            if bx + 1 < nx {
                let o = st[O_GLOW_X + i];
                kept -= k.share * o;
                got += o * (st[O_GLOW_D + i + 1] + st[O_GLOW_B + i + 1]);
            }
            if bx > 0 {
                let o = st[O_GLOW_X + i - 1];
                kept -= k.share * o;
                got += o * (st[O_GLOW_D + i - 1] + st[O_GLOW_B + i - 1]);
            }
            if bz + 1 < nz {
                let o = st[O_GLOW_Z + i];
                kept -= k.share * o;
                got += o * (st[O_GLOW_D + i + nx] + st[O_GLOW_B + i + nx]);
            }
            if bz > 0 {
                let o = st[O_GLOW_Z + i - nx];
                kept -= k.share * o;
                got += o * (st[O_GLOW_D + i - nx] + st[O_GLOW_B + i - nx]);
            }
            next[i] = k.albedo * (kept * here + k.share * got);
        }
        st[O_GLOW_B..O_GLOW_B + n].copy_from_slice(&next[..n]);
    }
}
