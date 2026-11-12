//! THE GAME'S OWN SMOKE ON A COARSE GRID: the smoke the SENSES see through, as density in
//! half-metre cells held in bricks (a brick is 2 m square in plan and 4 m high: floor to ceiling), stepped ten times a second. It is fed by the very emitters
//! the picture's solver is handed and knows nothing else of that solver: no velocity and no pressure, and of the wind only a drift under the open sky
//! (`K.wind`). What it has is THE WALLS:
//! smoke passes from a cell to its neighbour only where nothing solid and no leaf stands between their centres, so a canister fills the room it is in, goes
//! out by the open door and by a window whose pane has fallen out, and stops at the shut one.
//!
//! THE STATE is the game's (game_types smoke.rs `Smoke`): the plan, the emitters still letting out, the bricks by slot and what is in their cells, the light that comes round (glow.rs).
//! WHAT IS NOT STATE is made again from it and the box world (`Ways`): which faces of each cell are open, asked of the box world's lines once a cell; again round every leaf at every step,
//! since a door swings, wherever a static's row is another, and round a pane that is open otherwise than it was. So the faces are at every step what asking them all afresh would give: a
//! game put back from a snapshot and one that never was step alike.
//!
//! The arithmetic is + − × ÷ and the game's own exp and length, in an order that is written down (bricks by slot, cells by index): the same bits on every
//! machine.

use game_types::knobs::Knobs;
use crate::lines;
use crate::maths::{exp, hypot3};
use crate::world::{js_max, js_min, K_ROT, REC, R_C, R_H, R_KIND, World, Seen};
use alloc::vec::Vec;
use game_types::made::{next, Made, Tables};
pub use game_types::smoke::{Brick, Emitter, Smoke};
use prim::floor;

/// metres a cell
pub const CELL: f64 = 0.5;
/// cells along a brick's side in plan, and up
pub const BX: usize = 4;
pub const BY: usize = 8;
pub const CELLS: usize = BX * BX * BY;
/// THE PLAN A GRID STANDS ON, in bricks, at most, AND the bricks a game can hold: one number, so there is a brick for every place of the plan and smoke is
/// never refused one (a larger plan has no grid: `step`). A canister in the open is in about twenty by the time it is spent.
pub const MAX_BRICKS: usize = 512;
/// emitters letting out at once (the solver's own list holds as many)
pub const MAX_EMIT: usize = 64;
/// square metres of the plan at most (glow.rs)
pub const MAX_GLOW: usize = MAX_BRICKS * (SIDE * SIDE) as usize;
/// seconds a step
pub const SUB: f64 = 0.1;
const VOL: f64 = CELL * CELL * CELL;
pub const SIDE: f64 = BX as f64 * CELL;

/// THE WAYS FROM CELL TO CELL, made from the bricks and the box world the game steps over, and what they were made from. A game put back has none till its next step or `ways`
#[derive(Default)]
pub struct Ways {
    /// a byte a cell, as the cells lie: 1 = open toward +x · 2 = open upward · 4 = open toward +z · 8 = asked (the three bits are an answer) · 16, 32 = on the plan's near
    /// edge, open toward −x, toward −z, and 64 = those two asked (F_WEST, F_NORTH, F_EDGE: only a step with wind asks them)
    faces: Vec<u8>,
    /// the leaves as the last step found them, a row each as the box world holds it
    leaves: Vec<f64>,
    /// the statics as the faces were last asked of them: each one's word, and the rectangle of the plan round it
    seen: Seen,
    /// the panes that are open somewhere, as the faces were last asked of them: each one's static, and a word of where it is open
    panes: Vec<(u32, u64)>,
    /// which step or asking these are of: no two alike (0 = none was)
    asked: u64,
    /// a step's change of every cell, summed before any is applied; which bricks the step left with next to nothing in them, and what each holds; what of a step came into a brick from the
    /// bricks round it, and what is missing from its color
    t: Scratch,
}
#[derive(Default)]
struct Scratch {
    delta: Vec<f64>,
    thin: Vec<bool>,
    held: Vec<f64>,
    came: Vec<[f64; 4]>,
}
impl Scratch {
    /// room for `n` bricks
    fn fit(&mut self, n: usize) {
        self.delta.resize(n * CELLS, 0.0);
        self.thin.resize(n, false);
        self.held.resize(n, 0.0);
        self.came.resize(n, [0.0; 4]);
    }
}
const F_X: u8 = 1;
const F_Y: u8 = 2;
const F_Z: u8 = 4;
const F_ASKED: u8 = 8;
/// …and, of a cell on the plan's near edges alone, the ways OUT of the plan toward −x and toward −z (the far edges' ways out are F_X and F_Z of their
/// last cells): what the wind carries smoke off the level by, asked the first time a wind blows over the cell (F_EDGE = asked)
const F_WEST: u8 = 16;
const F_NORTH: u8 = 32;
const F_EDGE: u8 = 64;

/// the smoke in hand, and its ways
struct Grid<'a> {
    s: &'a mut Smoke,
    faces: &'a mut Vec<u8>,
    nbx: usize,
    nbz: usize,
}
/// a cell's faces in four bytes a word
pub const WORDS: usize = CELLS / 4;
/// WHICH ASKING THE WAYS ARE OF, for whoever draws the bricks, and a byte of them a cell (bits 1, 2, 4; 8 = asked). None before the first step, or where nobody has asked the faces of the
/// smoke that stands there (a game put back, till its next step or `ways`)
pub fn drawn<R>(made: &Made, f: impl FnOnce(u64, &[u8]) -> R) -> Option<R> {
    made.with(|w: &mut Ways| if w.asked == 0 { None } else { Some(f(w.asked, &w.faces)) })
}
/// …of a brick's cells, four to a word
pub fn open_words(faces: &[u8]) -> [u32; WORDS] {
    core::array::from_fn(|w| (0..4).fold(0, |o, j| o | ((faces[w * 4 + j] & (F_X | F_Y | F_Z | F_ASKED)) as u32) << (8 * j)))
}
/// THE LIGHT UNDER A ROOF at a point, over box world `w` (glow.rs at)
pub fn under_roof(s: &Smoke, made: &Tables, w: &World, p: [f64; 3], tune: &Knobs) -> Option<crate::glow::Field> {
    crate::glow::at(&made.light, w, &s.glow, s.wide as usize, s.deep as usize, p, tune)
}
/// THE PLAN A GAME'S GRID STANDS ON, in metres along x and z (0 by 0 before its first step)
pub fn plan(s: &Smoke) -> (f64, f64) {
    (s.wide as f64 * SIDE, s.deep as f64 * SIDE)
}
#[inline(always)]
fn cell_at(x: usize, y: usize, z: usize) -> usize {
    (y * BX + z) * BX + x
}

impl Grid<'_> {
    /// the brick at (bx, bz) of the plan, if there is one
    #[inline(always)]
    fn brick(&self, bx: i32, bz: i32) -> Option<usize> {
        if bx < 0 || bz < 0 || bx as usize >= self.nbx || bz as usize >= self.nbz {
            return None;
        }
        (self.s.at[bz as usize * self.nbx + bx as usize] as usize).checked_sub(1)
    }
    fn slots(&self) -> usize {
        self.s.bricks.len()
    }
    #[inline(always)]
    fn key(&self, b: usize) -> Option<(i32, i32)> {
        self.s.bricks[b].at.map(|(x, z)| (x as i32, z as i32))
    }
    /// the brick at (bx, bz), made in the lowest free slot if there is none: None = off the plan
    fn brick_or_new(&mut self, bx: i32, bz: i32) -> Option<usize> {
        if bx < 0 || bz < 0 || bx as usize >= self.nbx || bz as usize >= self.nbz {
            return None;
        }
        if let Some(b) = self.brick(bx, bz) {
            return Some(b);
        }
        let b = self.s.bricks.iter().position(|b| b.at.is_none()).unwrap_or(self.slots());
        if b == self.slots() {
            self.s.bricks.push(Brick::default());
            self.s.rho.resize((b + 1) * CELLS, 0.0);
            self.faces.resize((b + 1) * CELLS, 0);
        }
        self.s.bricks[b].at = Some((bx as u16, bz as u16));
        self.s.at[bz as usize * self.nbx + bx as usize] = b as u16 + 1;
        self.faces[b * CELLS..(b + 1) * CELLS].fill(0);
        Some(b)
    }
    /// (what is free is zeros)
    fn free(&mut self, b: usize) {
        if let Some((bx, bz)) = self.key(b) {
            self.s.at[bz as usize * self.nbx + bx as usize] = 0;
        }
        self.s.bricks[b] = Brick::default();
        self.s.rho[b * CELLS..(b + 1) * CELLS].fill(0.0);
    }
    /// `share` of a brick's smoke is of color `d`, and the rest what it was
    fn mix(&mut self, b: usize, share: f64, d: [f64; 3]) {
        let (was, s) = (self.s.bricks[b].dye, unit(share));
        self.s.bricks[b].dye = core::array::from_fn(|i| was[i] + (d[i] - was[i]) * s);
    }
    /// the faces of every cell whose centre lies within a cell of the rectangle are unknown again (a face is a line from a centre to the next one over)
    fn dirty(&mut self, [x0, z0, x1, z1]: [f64; 4]) {
        for b in 0..self.slots() {
            let Some((bx, bz)) = self.key(b) else { continue };
            let (ox, oz) = (bx as f64 * SIDE, bz as f64 * SIDE);
            if !(ox <= x1 + CELL && oz <= z1 + CELL && ox + SIDE >= x0 - CELL && oz + SIDE >= z0 - CELL) {
                continue;
            }
            for z in 0..BX {
                for x in 0..BX {
                    let (cx, cz) = (ox + (x as f64 + 0.5) * CELL, oz + (z as f64 + 0.5) * CELL);
                    if cx < x0 - CELL || cx > x1 + CELL || cz < z0 - CELL || cz > z1 + CELL {
                        continue;
                    }
                    for y in 0..BY {
                        self.faces[b * CELLS + cell_at(x, y, z)] = 0;
                    }
                }
            }
        }
    }
    /// no free slot stands at the end
    fn trim(&mut self) {
        let top = self.s.bricks.iter().rposition(|b| b.at.is_some()).map_or(0, |b| b + 1);
        self.s.bricks.truncate(top);
        self.s.rho.truncate(top * CELLS);
        self.faces.truncate(top * CELLS);
    }
}
/// the smoke and its ways in hand, the faces of the statics and the panes as they stand: where one is not as it was, they are asked again (where it stood, and where it stands)
fn over<'a>(s: &'a mut Smoke, faces: &'a mut Vec<u8>, seen: &mut Seen, panes: &mut Vec<(u32, u64)>, w: &World) -> Grid<'a> {
    let (nbx, nbz) = (s.wide as usize, s.deep as usize);
    faces.resize(s.bricks.len() * CELLS, 0);
    let mut g = Grid { s, faces, nbx, nbz };
    let round = |r: &[f64]| {
        let e = r[R_H] + r[R_H + 1] + r[R_H + 2];
        [r[R_C] - e, r[R_C + 2] - e, r[R_C] + e, r[R_C + 2] + e, r[R_C + 1] + e]
    };
    seen.look(w, round, |r| g.dirty([r[0], r[1], r[2], r[3]]));
    if !crate::panes::open_panes(&w.panes).eq(panes.iter().copied()) {
        let now: Vec<(u32, u64)> = crate::panes::open_panes(&w.panes).collect();
        for p in now.iter().filter(|p| !panes.contains(p)).chain(panes.iter().filter(|p| !now.contains(p))) {
            let r = w.row(p.0 as usize);
            let e = r[R_H] + r[R_H + 2];
            g.dirty([r[R_C] - e, r[R_C + 2] - e, r[R_C] + e, r[R_C + 2] + e]);
        }
        *panes = now;
    }
    g
}
/// NO SMOKE ANYWHERE: the cells, the emitters, the book and the count of steps are nothing. The plan the grid stands on, the beat and the light that comes round
/// stay: they are the level's, not the smoke's.
pub fn empty(s: &mut Smoke, made: &Made) {
    s.emitters.clear();
    s.bricks.clear();
    s.rho.clear();
    s.at.fill(0);
    (s.steps, s.out, s.given) = (0, 0.0, 0.0);
    forget(made);
}
/// THE SMOKE IS ANOTHER THAN THE WAYS ARE OF: no face is known
pub fn forget(made: &Made) {
    made.with(|w: &mut Ways| {
        w.faces.clear();
        w.asked = 0;
    })
}

/// is the way between two cells' centres open: nothing solid, no glass that is not open there and no leaf between them, asked both ways (a line that STARTS inside a box is not stopped by
/// it, the rule for an eye in a wall, so a cell inside a wall is shut off by the line that ends in it)
fn open(w: &World, a: [f64; 3], b: [f64; 3], tune: &Knobs) -> bool {
    clear(w, a, b, CELL, tune) && clear(w, b, a, CELL, tune)
}
/// nothing of that on the line a → b of this length (the one place this file walks a line: the walk is not copied into every asker)
#[inline(never)]
fn clear(w: &World, a: [f64; 3], b: [f64; 3], len: f64, tune: &Knobs) -> bool {
    lines::blocked(w, a[0], a[1], a[2], b[0], b[1], b[2], len, 0.02, 0, true, lines::ASK_GLASS, tune) != -1
}
/// can what is let out at `p` reach the point `c` (a line of any length; one too short to be a line is open)
fn reaches(w: &World, p: [f64; 3], c: [f64; 3], tune: &Knobs) -> bool {
    let len = hypot3(c[0] - p[0], c[1] - p[1], c[2] - p[2]);
    len < 1e-4 || clear(w, p, c, len, tune)
}

/// what a step is told: the model's numbers
pub struct K<'a> {
    pub decay: f64,
    pub spread: f64,
    pub rise: f64,
    pub rise_rate: f64,
    pub gone: f64,
    /// THE WIND the smoke drifts by under the open sky, m/s along x and z (0 and 0 = still air, and no statement of the drift
    /// is reached: the step as it was). `share`: what share of it a square metre of the plan has (0 under a roof … 1 in the open: the footprints the picture's wind goes by), row by row, `2 · nbx` to a row.
    pub wind_x: f64,
    pub wind_z: f64,
    pub share: &'a [f64],
    /// …and how it rises with height, as the storm's field does: `wind_low` of it at the ground, `wind_gain` more by
    /// `wind_full_at` metres up
    pub wind_low: f64,
    pub wind_gain: f64,
    pub wind_full_at: f64,
    /// …and how much of its own spread the smoke keeps where the wind blows whole (0 ‥ 1; step 4)
    pub wind_keep: f64,
}

/// the most of a cell's smoke the wind carries across one face in a pass (a cell hands on at most half of what it holds, along x and along z), and
/// the most passes a step takes: together the fastest wind the smoke keeps up with, 10 m/s along an axis
const DRIFT_MAX: f64 = 0.25;
const MAX_PASSES: f64 = 8.0;
/// a share as the host wrote it, held to 0 ‥ 1 (what is no number is no wind)
#[inline(always)]
fn unit(s: f64) -> f64 {
    if s > 0.0 {
        if s < 1.0 {
            s
        } else {
            1.0
        }
    } else {
        0.0
    }
}
/// THE WIND'S SHARE at the cell (x, z) of the brick whose first cell is (gx, gz) of the plan, and at the squares beyond its +x and +z faces (this
/// brick's, or the next brick's first where one stands: `east`, `south` — inside the plan wherever that brick can stand; else the cell's own)
#[inline(always)]
fn shares(share: &[f64], wide: usize, gx: usize, gz: usize, x: usize, z: usize, east: bool, south: bool) -> (f64, f64, f64) {
    let s = unit(share[(gz + z) / 2 * wide + (gx + x) / 2]);
    let sx = if x + 1 < BX || east { unit(share[(gz + z) / 2 * wide + (gx + x + 1) / 2]) } else { s };
    let sz = if z + 1 < BX || south { unit(share[(gz + z + 1) / 2 * wide + (gx + x) / 2]) } else { s };
    (s, sx, sz)
}
/// LET OUT: an emitter joins the list. When the list is full the one with the least still to let out gives way, and that may be the newcomer.
pub fn emit(s: &mut Smoke, e: Emitter) {
    let real = |v: f64| v - v == 0.0;
    // (an emitter that is no number lets nothing out, and must not sit in the list as one)
    if !(e.at.iter().all(|&v| real(v)) && e.radius >= 0.0 && e.radius < 1e6 && e.rate > 0.0 && e.ttl > 0.0 && e.rate * e.ttl < 1e12) {
        return;
    }
    let e = Emitter { age: 0.0, dye: e.dye.map(unit), ..e };
    if s.emitters.len() < MAX_EMIT {
        s.emitters.push(e);
        return;
    }
    let left = |e: &Emitter| e.rate * js_max(0.0, e.ttl - e.age);
    let mut k = 0;
    for i in 1..s.emitters.len() {
        if left(&s.emitters[i]) < left(&s.emitters[k]) {
            k = i;
        }
    }
    if left(&s.emitters[k]) < e.rate * e.ttl {
        s.emitters[k] = e;
    }
}
/// SHUT: every emitter has let out its last; what is in the air stays
pub fn shut(s: &mut Smoke) {
    for e in &mut s.emitters {
        e.ttl = e.age;
    }
}
/// the emitters letting out
pub fn letting(s: &Smoke) -> impl Iterator<Item = &Emitter> {
    s.emitters.iter().filter(|e| e.age < e.ttl)
}
/// is there smoke: an emitter, or a brick standing
pub fn smoky(s: &Smoke) -> bool {
    !s.emitters.is_empty() || !s.bricks.is_empty()
}
/// `dt` SECONDS OF THE GAME → how many steps have come due: four at most (a game that stood still does not owe a hundred)
pub fn beat(s: &mut Smoke, dt: f64) -> u32 {
    if !(dt > 0.0) {
        return 0;
    }
    let mut left = s.clock + dt;
    let mut n = 0;
    while left >= SUB && n < 4 {
        left -= SUB;
        n += 1;
    }
    s.clock = if left >= SUB { 0.0 } else { left };
    n
}

/// the plan of box world `w`, in squares
fn plan_of(w: &World) -> (f64, f64) {
    (-floor(-(w.w as f64 * w.cs) / SIDE), -floor(-(w.h as f64 * w.cs) / SIDE))
}
/// …if a grid stands on one that large
fn grid_of(w: &World) -> Option<(usize, usize)> {
    let (x, z) = plan_of(w);
    (x >= 1.0 && z >= 1.0 && x * z <= MAX_BRICKS as f64).then_some((x as usize, z as usize))
}
/// the light's tables of box world `w` made before a step asks for them, `most` lines of them (glow.rs ready). → are they whole
pub fn light_ready(made: &Tables, w: &World, to_sun: [f64; 3], most: usize, tune: &Knobs) -> bool {
    grid_of(w).is_none_or(|(x, z)| crate::glow::ready(&made.light, w, x, z, to_sun, most, tune))
}
/// what a tick with no step in it makes ahead for the light (glow.rs ahead)
pub fn ahead(s: &Smoke, made: &Tables, w: &World, tune: &Knobs) {
    crate::glow::ahead(&made.light, w, &s.glow, s.wide as usize, s.deep as usize, tune);
}
/// `n` STEPS of a tenth of a second each, over box world `w` as it stands. A level whose plan is larger than a grid can be has none: the smoke is as it was.
pub fn step(s: &mut Smoke, made: &Tables, w: &World, n: u32, k: &K, glow: Option<&crate::glow::K>, tune: &Knobs) {
    let Some((nbx, nbz)) = grid_of(w) else { return };
    made.ways.with(|t: &mut Ways| {
        if (s.wide as usize, s.deep as usize) != (nbx, nbz) {
            // (the first step on this level)
            (s.wide, s.deep) = (nbx as u32, nbz as u32);
            (s.bricks, s.rho, s.glow) = Default::default();
            s.at.clear();
            s.at.resize(nbx * nbz, 0);
        }
        t.asked = next();
        let Ways { faces, leaves, seen, panes, t, .. } = t;
        let mut g = over(s, faces, seen, panes, w);
        for _ in 0..n {
            substep(w, &mut g, leaves, t, k, tune);
            match glow {
                Some(gk) => crate::glow::step(&made.light, w, &mut g.s.glow, nbx, nbz, gk, tune),
                None => crate::glow::down(&mut g.s.glow),
            }
            g.s.steps += 1;
        }
        g.trim();
    })
}

/// THE WAYS OF THE SMOKE THAT STANDS are asked, over box world `w` as it stands: what the next step would
/// ask first, for whoever draws a game put back before it steps. Nothing, where its plan is not the level's (no step yet).
pub fn ways(s: &mut Smoke, made: &Made, w: &World, tune: &Knobs) {
    if s.wide == 0 || grid_of(w) != Some((s.wide as usize, s.deep as usize)) {
        return;
    }
    made.with(|t: &mut Ways| {
        t.asked = next();
        let Ways { faces, leaves: was, seen, panes, .. } = t;
        let mut g = over(s, faces, seen, panes, w);
        leaves(w, &mut g, was);
        ask(w, &mut g, tune);
    })
}
/// the faces round every leaf that is not as it was last found (a door swings; one is new, one is gone) are to be asked again: where it stood and where
/// it stands. A leaf that has not moved keeps the answers it had.
fn leaves(w: &World, g: &mut Grid, was: &mut Vec<f64>) {
    // (its reach in plan whichever way it is turned: the two half sides, and the third where it is tilted off the upright)
    let round = |r: &[f64]| {
        let e = r[R_H] + r[R_H + 2] + if r[R_KIND] as u32 & K_ROT != 0 { r[R_H + 1] } else { 0.0 };
        [r[R_C] - e, r[R_C + 2] - e, r[R_C] + e, r[R_C + 2] + e]
    };
    let same = |a: &[f64], b: &[f64]| a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits());
    let (mut old, mut new) = (was.chunks_exact(REC), w.leaf.chunks_exact(REC));
    let mut moved = false;
    loop {
        match (old.next(), new.next()) {
            (None, None) => break,
            (Some(a), Some(b)) if same(a, b) => continue,
            (a, b) => {
                moved = true;
                for r in [a, b].into_iter().flatten() {
                    g.dirty(round(r));
                }
            }
        }
    }
    if moved {
        was.clone_from(&w.leaf);
    }
}
/// the faces nobody has asked (a new brick's, those round a leaf, those round a static that was written)
fn ask(w: &World, g: &mut Grid, tune: &Knobs) {
    for b in 0..g.slots() {
        let Some((bx, bz)) = g.key(b) else { continue };
        for y in 0..BY {
            for z in 0..BX {
                for x in 0..BX {
                    let c = b * CELLS + cell_at(x, y, z);
                    if g.faces[c] & F_ASKED != 0 {
                        continue;
                    }
                    let p = middle(bx, bz, x, y, z);
                    let mut f = F_ASKED;
                    if open(w, p, [p[0] + CELL, p[1], p[2]], tune) {
                        f |= F_X;
                    }
                    if y + 1 < BY && open(w, p, [p[0], p[1] + CELL, p[2]], tune) {
                        f |= F_Y;
                    }
                    if open(w, p, [p[0], p[1], p[2] + CELL], tune) {
                        f |= F_Z;
                    }
                    g.faces[c] = f;
                }
            }
        }
    }
}

/// how much of its rise a plume in the whole of the wind loses (the drawn one, in a storm's yard, tops out at about 1.1 m)
const LAID: f64 = 0.4;
/// the four neighbours a brick can spill into
const OUT: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

/// what one step of the smoke is made by
struct Sub<'a> {
    w: &'a World,
    k: &'a K<'a>,
    tune: &'a Knobs,
    /// the plan's squares across, and whether there is a wind
    wide: usize,
    windy: bool,
    /// is there any color to carry
    dyes: bool,
    /// what of a difference crosses a face in a step, and the density a brick spills at
    rate: f64,
    spill: f64,
    /// what of a density a step leaves
    keep: f64,
}
/// ONE STEP OF THE SMOKE (SUB of a second). The leaves as they stand now, what the emitters let out, the bricks it would spill into, the faces nobody has
/// asked; then the wind carries it, it passes toward the thinner side, it thins, and the bricks with next to nothing in them are given back.
fn substep(w: &World, g: &mut Grid, was: &mut Vec<f64>, t: &mut Scratch, k: &K, tune: &Knobs) {
    leaves(w, g, was);
    let mut s = Sub {
        w,
        k,
        tune,
        wide: g.nbx * 2,
        windy: k.wind_x != 0.0 || k.wind_z != 0.0,
        dyes: g.s.bricks.iter().any(|b| b.dye != [0.0; 3]),
        rate: k.spread * 0.5 * SUB / (CELL * CELL),
        spill: k.gone / (CELLS as f64 * VOL),
        keep: exp(-k.decay * SUB),
    };
    let_out(&mut s, g);
    spills(&s, g);
    ask(w, g, tune);
    t.fit(g.slots());
    if s.dyes {
        t.came.fill([0.0; 4]);
    }
    let blown = if s.windy { wind(&s, g, t) } else { 0.0 };
    passes_across(&s, g, t);
    thins(&s, g, t);
    let given = gives_back(&s, g, t);
    g.s.given += given + blown * VOL * s.keep;
}

/// The emitters let out, each up the column it stands under, and the spent ones leave the list.
fn let_out(s: &mut Sub, g: &mut Grid) {
    let mut list = core::mem::take(&mut g.s.emitters);
    for e in &mut list {
        s.dyes |= e.dye != [0.0; 3];
        if e.age < e.ttl {
            lets_out(s, g, e);
        }
        e.age += SUB;
    }
    list.retain(|e| e.age < e.ttl);
    g.s.emitters = list;
}

/// One emitter: from its own foot to the top of its plume, its width and the rise the blobs had; each cell between takes what of the plume is in it.
fn lets_out(s: &Sub, g: &mut Grid, e: &Emitter) {
    let Emitter { at: [x, y0, z], rate, ttl, age, radius, dye } = *e;
    let amount = rate * js_min(SUB, ttl - age);
    let top = (BY - 1) as f64;
    let foot = js_min(js_max((y0 - radius) / CELL, 0.0), top);
    let (fx, fz) = (floor(x / CELL), floor(z / CELL));
    // (the wind lays a plume over: in all of it, it rises LAID less)
    let (mx, mz) = (floor(x), floor(z));
    let blown = if s.windy && mx >= 0.0 && mz >= 0.0 && mx < s.wide as f64 && mz < (g.nbz * 2) as f64 { unit(s.k.share[mz as usize * s.wide + mx as usize]) } else { 0.0 };
    let rise = js_min(s.k.rise, s.k.rise_rate * age) * (1.0 - LAID * blown);
    let head = js_min(js_max((y0 + radius + rise) / CELL, foot + 0.1), top + 1.0);
    // (a point off the plan lets out nowhere)
    if !(fx >= 0.0 && fz >= 0.0 && fx < (g.nbx * BX) as f64 && fz < (g.nbz * BX) as f64) {
        return;
    }
    let (ux, uz) = column(s, g, [x, js_max(y0, 0.05), z], fx, fz);
    g.s.out += amount;
    let Some(b) = g.brick_or_new((ux / BX) as i32, (uz / BX) as i32) else {
        return;
    };
    if s.dyes {
        let had: f64 = g.s.rho[b * CELLS..(b + 1) * CELLS].iter().sum();
        g.mix(b, amount / VOL / (had + amount / VOL), dye);
    }
    let each = amount / VOL / (head - foot);
    let mut y = floor(foot);
    while y < head {
        g.s.rho[b * CELLS + cell_at(ux % BX, y as usize, uz % BX)] += each * (js_min(head, y + 1.0) - js_max(foot, y));
        y += 1.0;
    }
}

/// The column a thing lets out up: the one it stands in (`fx`, `fz`) if what it lets out can reach that cell's middle, else the first of those round it
/// that it can reach (a canister against a wall stands in a cell whose middle is inside the wall).
fn column(s: &Sub, g: &Grid, p: [f64; 3], fx: f64, fz: f64) -> (usize, usize) {
    const ROUND: [(f64, f64); 9] = [(0.0, 0.0), (-1.0, 0.0), (1.0, 0.0), (0.0, -1.0), (0.0, 1.0), (-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)];
    for (dx, dz) in ROUND {
        let (tx, tz) = (fx + dx, fz + dz);
        if tx < 0.0 || tz < 0.0 || tx >= (g.nbx * BX) as f64 || tz >= (g.nbz * BX) as f64 {
            continue;
        }
        let y = js_min(js_max(floor(p[1] / CELL), 0.0), (BY - 1) as f64);
        if reaches(s.w, p, [(tx + 0.5) * CELL, (y + 0.5) * CELL, (tz + 0.5) * CELL], s.tune) {
            return (tx as usize, tz as usize);
        }
    }
    (fx as usize, fz as usize)
}

/// A brick gets the neighbour it would spill into: where a cell at its edge holds smoke AND the way out of that cell into the neighbour is open. A wall
/// along a brick's edge costs no brick behind it.
fn spills(s: &Sub, g: &mut Grid) {
    for b in 0..g.slots() {
        let Some((bx, bz)) = g.key(b) else { continue };
        for (side, &(dx, dz)) in OUT.iter().enumerate() {
            let (nx, nz) = (bx + dx, bz + dz);
            if nx < 0 || nz < 0 || nx as usize >= g.nbx || nz as usize >= g.nbz || g.brick(nx, nz).is_some() {
                continue;
            }
            if spills_out(s, g, b, (bx, bz), side) {
                g.brick_or_new(nx, nz);
            }
        }
    }
}

/// whether brick `b` has a cell at its edge on `side` that holds smoke and is open into the neighbour there
fn spills_out(s: &Sub, g: &Grid, b: usize, (bx, bz): (i32, i32), side: usize) -> bool {
    let (dx, dz) = OUT[side];
    for y in 0..BY {
        for i in 0..BX {
            let (x, z) = edge_cell(side, i);
            if !(g.s.rho[b * CELLS + cell_at(x, y, z)] > s.spill) {
                continue;
            }
            let p = middle(bx, bz, x, y, z);
            if open(s.w, p, [p[0] + dx as f64 * CELL, p[1], p[2] + dz as f64 * CELL], s.tune) {
                return true;
            }
        }
    }
    false
}

/// the middle of a cell of the brick at (bx, bz), in the world
#[inline(always)]
fn middle(bx: i32, bz: i32, x: usize, y: usize, z: usize) -> [f64; 3] {
    let (ox, oz) = (bx as f64 * SIDE, bz as f64 * SIDE);
    [ox + (x as f64 + 0.5) * CELL, (y as f64 + 0.5) * CELL, oz + (z as f64 + 0.5) * CELL]
}

/// the cell east of a cell (`c` of all, x of its brick), in its brick or the one beside it
#[inline(always)]
fn east_of(c: usize, (x, y, z): (usize, usize, usize), east: Option<usize>) -> Option<usize> {
    if x + 1 < BX { Some(c + 1) } else { east.map(|e| e * CELLS + cell_at(0, y, z)) }
}
/// …and the one south of it
#[inline(always)]
fn south_of(c: usize, (x, y, z): (usize, usize, usize), south: Option<usize>) -> Option<usize> {
    if z + 1 < BX { Some(c + BX) } else { south.map(|s| s * CELLS + cell_at(x, y, 0)) }
}
/// `over` of smoke goes from cell `c` to cell `n`, with its color
#[inline(always)]
fn goes(s: &Sub, g: &Grid, t: &mut Scratch, c: usize, n: usize, over: f64) {
    t.delta[c] -= over;
    t.delta[n] += over;
    if s.dyes {
        carry(g, &mut t.came, c, n, over);
    }
}

/// THE WIND (K.wind): under the open sky the air moves and the smoke goes with it, across the open faces in plan, out of the cell UPWIND (a donor's own
/// density: nothing is made), at the wind's speed at that height by the share of it the two squares have (none under a roof: a room's smoke stays a
/// room's, and the wind comes in at a door only as far as the lee lets it). What it carries over the plan's edge has left the level: into the book as
/// given back. A PASS carries at most DRIFT_MAX of a cell across a face, so a faster wind takes more passes of the step, each from the densities the one
/// before left; a wind faster than MAX_PASSES carry is slowed WHOLE, both axes by one number, never bent. A brick the smoke is blown toward is made by
/// the spill of the next step. None of this is reached in still air. → what it blew off the plan.
fn wind(s: &Sub, g: &mut Grid, t: &mut Scratch) -> f64 {
    let k = s.k;
    let fast = js_max(js_max(k.wind_x, -k.wind_x), js_max(k.wind_z, -k.wind_z)) * SUB / CELL * (k.wind_low + js_max(k.wind_gain, 0.0));
    let passes = js_max(1.0, js_min(MAX_PASSES, -floor(-fast / DRIFT_MAX)));
    let slowed = js_min(1.0, passes * DRIFT_MAX / fast) / passes;
    let by = (k.wind_x * SUB / CELL * slowed, k.wind_z * SUB / CELL * slowed);
    let mut blown = 0.0;
    let mut pass = 0.0;
    while pass < passes {
        pass += 1.0;
        t.delta.fill(0.0);
        for b in 0..g.slots() {
            wind_over(s, g, t, b, by, &mut blown);
        }
        for b in 0..g.slots() {
            if g.key(b).is_some() {
                for c in 0..CELLS {
                    g.s.rho[b * CELLS + c] = js_max(0.0, g.s.rho[b * CELLS + c] + t.delta[b * CELLS + c]);
                }
            }
        }
    }
    blown
}

/// the wind over one brick
fn wind_over(s: &Sub, g: &mut Grid, t: &mut Scratch, b: usize, (cx, cz): (f64, f64), blown: &mut f64) {
    let Some((bx, bz)) = g.key(b) else { return };
    let (k, east, south) = (s.k, g.brick(bx + 1, bz), g.brick(bx, bz + 1));
    let (gx, gz) = (bx as usize * BX, bz as usize * BX);
    let (last_x, last_z) = (bx as usize + 1 == g.nbx, bz as usize + 1 == g.nbz);
    for z in 0..BX {
        for x in 0..BX {
            let (sh, sx, sz) = shares(s.k.share, s.wide, gx, gz, x, z, east.is_some(), south.is_some());
            if sh == 0.0 && sx == 0.0 && sz == 0.0 {
                continue;
            }
            for y in 0..BY {
                // (the wind at this layer's middle, as the storm's field has it: slower at the ground)
                let h = k.wind_low + k.wind_gain * js_max(0.0, js_min(1.0, (y as f64 + 0.5) * CELL / k.wind_full_at));
                let (ux, uz) = (cx * h * 0.5 * (sh + sx), cz * h * 0.5 * (sh + sz));
                let c = b * CELLS + cell_at(x, y, z);
                if ((x == 0 && bx == 0) || (z == 0 && bz == 0)) && g.faces[c] & F_EDGE == 0 {
                    ask_edge(s, g, c, (bx, bz), (x, y, z));
                }
                let (f, here) = (g.faces[c], g.s.rho[c]);
                let mut leaves = |t: &mut Scratch, out: f64| {
                    t.delta[c] -= out;
                    *blown += out;
                };
                if f & F_X != 0 {
                    if let Some(n) = east_of(c, (x, y, z), east) {
                        let over = if ux > 0.0 { ux * here } else { ux * g.s.rho[n] };
                        goes(s, g, t, c, n, over);
                    } else if cx > 0.0 && last_x {
                        leaves(t, cx * h * sh * here);
                    }
                }
                if f & F_Z != 0 {
                    if let Some(n) = south_of(c, (x, y, z), south) {
                        let over = if uz > 0.0 { uz * here } else { uz * g.s.rho[n] };
                        goes(s, g, t, c, n, over);
                    } else if cz > 0.0 && last_z {
                        leaves(t, cz * h * sh * here);
                    }
                }
                if f & F_WEST != 0 && cx < 0.0 {
                    leaves(t, -cx * h * sh * here);
                }
                if f & F_NORTH != 0 && cz < 0.0 {
                    leaves(t, -cz * h * sh * here);
                }
            }
        }
    }
}

/// a cell on the plan's near edge is asked its ways out, the first time a wind blows over it
fn ask_edge(s: &Sub, g: &mut Grid, c: usize, (bx, bz): (i32, i32), (x, y, z): (usize, usize, usize)) {
    let p = middle(bx, bz, x, y, z);
    let mut f = g.faces[c] | F_EDGE;
    if x == 0 && bx == 0 && open(s.w, p, [p[0] - CELL, p[1], p[2]], s.tune) {
        f |= F_WEST;
    }
    if z == 0 && bz == 0 && open(s.w, p, [p[0], p[1], p[2] - CELL], s.tune) {
        f |= F_NORTH;
    }
    g.faces[c] = f;
}

/// Smoke passes across every open face toward the thinner side: each cell's change summed from the densities as the step found them, applied after.
/// WHERE THE WIND BLOWS THE SMOKE GOES WITH THE AIR AND DOES LITTLE OF ITS OWN (K.wind; the picture's solver pulls the smoke's whole velocity toward the
/// wind's, and the wind is flat, so its own motions die and it stays as low as it was let out, smoke/wind_force.wgsl): a face passes less by the share of
/// the wind its squares have. UPWARD, nothing in the open; SIDEWAYS, down to `wind_keep` of it (enough to get round what the wind pushes it against, as
/// the picture's does by its pressure). In still air and under a roof: all of it, as ever.
fn passes_across(s: &Sub, g: &mut Grid, t: &mut Scratch) {
    let calm = 1.0 - unit(s.k.wind_keep);
    t.delta.fill(0.0);
    for b in 0..g.slots() {
        let Some((bx, bz)) = g.key(b) else { continue };
        let (east, south) = (g.brick(bx + 1, bz), g.brick(bx, bz + 1));
        let (gx, gz) = (bx as usize * BX, bz as usize * BX);
        for y in 0..BY {
            for z in 0..BX {
                for x in 0..BX {
                    let c = b * CELLS + cell_at(x, y, z);
                    let (f, here) = (g.faces[c], g.s.rho[c]);
                    let (rx, ry, rz) = if s.windy {
                        let (sh, sx, sz) = shares(s.k.share, s.wide, gx, gz, x, z, east.is_some(), south.is_some());
                        (s.rate * (1.0 - calm * 0.5 * (sh + sx)), s.rate * (1.0 - sh), s.rate * (1.0 - calm * 0.5 * (sh + sz)))
                    } else {
                        (s.rate, s.rate, s.rate)
                    };
                    if f & F_X != 0 {
                        if let Some(n) = east_of(c, (x, y, z), east) {
                            goes(s, g, t, c, n, rx * (here - g.s.rho[n]));
                        }
                    }
                    if f & F_Y != 0 {
                        let n = c + BX * BX;
                        let pass = ry * (here - g.s.rho[n]);
                        t.delta[c] -= pass;
                        t.delta[n] += pass;
                    }
                    if f & F_Z != 0 {
                        if let Some(n) = south_of(c, (x, y, z), south) {
                            goes(s, g, t, c, n, rz * (here - g.s.rho[n]));
                        }
                    }
                }
            }
        }
    }
}

/// The step's changes, applied and thinned by its decay, and the book with it: what each brick holds now, and whether that is next to nothing.
fn thins(s: &Sub, g: &mut Grid, t: &mut Scratch) {
    g.s.out *= s.keep;
    g.s.given *= s.keep;
    for b in 0..g.slots() {
        t.thin[b] = true;
        if g.key(b).is_none() {
            continue;
        }
        let mut mass = 0.0;
        for c in 0..CELLS {
            let v = js_max(0.0, (g.s.rho[b * CELLS + c] + t.delta[b * CELLS + c]) * s.keep);
            g.s.rho[b * CELLS + c] = v;
            mass += v;
        }
        t.held[b] = mass * VOL;
        t.thin[b] = t.held[b] < s.k.gone;
        // (what came in is mixed into what is there now; the thinning leaves the color as it is)
        if s.dyes && t.came[b][0] > 0.0 && mass > 0.0 {
            let [i, d @ ..] = t.came[b];
            g.mix(b, i * s.keep / mass, d.map(|v| v / i));
        }
    }
}

/// A brick with next to nothing in it is given back once nothing can feed it: every brick is judged by what the step left, before any is given back.
/// → what the ones that went held.
fn gives_back(s: &Sub, g: &mut Grid, t: &mut Scratch) -> f64 {
    let mut given = 0.0;
    for b in 0..g.slots() {
        let Some((bx, bz)) = g.key(b) else { continue };
        if !t.thin[b] {
            continue;
        }
        // (`thin` from here on: this brick goes)
        t.thin[b] = !fed(s, g, bx, bz);
        if t.thin[b] {
            given += t.held[b];
        }
    }
    for b in 0..g.slots() {
        if t.thin[b] && g.key(b).is_some() {
            g.free(b);
        }
    }
    given
}

/// Whether anything can feed a brick: an emitter in it or beside it, or a brick round it with smoke at the edge they share: the very test that makes a
/// brick, so one made this step is not given back in it.
fn fed(s: &Sub, g: &Grid, bx: i32, bz: i32) -> bool {
    // (the side of the neighbour that faces this brick is the opposite one: OUT pairs them 0 and 1, 2 and 3)
    let beside = OUT.iter().enumerate().any(|(side, &(dx, dz))| {
        g.brick(bx + dx, bz + dz).map_or(false, |n| {
            (0..BY).any(|y| {
                (0..BX).any(|i| {
                    let (x, z) = edge_cell(side ^ 1, i);
                    g.s.rho[n * CELLS + cell_at(x, y, z)] > s.spill
                })
            })
        })
    });
    beside
        || g.s.emitters.iter().any(|e| {
            let (ex, ez) = (floor(e.at[0] / SIDE) - bx as f64, floor(e.at[2] / SIDE) - bz as f64);
            ex >= -1.0 && ex <= 1.0 && ez >= -1.0 && ez <= 1.0
        })
}

/// `over` of smoke goes from cell `c` to cell `n` (or back, where it is less than nothing): into another brick it takes its brick's color along
#[inline(always)]
fn carry(g: &Grid, came: &mut [[f64; 4]], c: usize, n: usize, over: f64) {
    let (from, to, v) = if over > 0.0 { (c / CELLS, n / CELLS, over) } else { (n / CELLS, c / CELLS, -over) };
    if from == to || !(v > 0.0) {
        return;
    }
    let d = g.s.bricks[from].dye;
    came[to][0] += v;
    for i in 0..3 {
        came[to][1 + i] += v * d[i];
    }
}

/// the cell `i` along a brick's edge on `side` (0 = toward −x · 1 = toward +x · 2 = toward −z · 3 = toward +z), as (x, z) in the brick
#[inline(always)]
fn edge_cell(side: usize, i: usize) -> (usize, usize) {
    match side {
        0 => (0, i),
        1 => (BX - 1, i),
        2 => (i, 0),
        _ => (i, BX - 1),
    }
}

/// HOW MUCH OF THE LIGHT ALONG a → b COMES THROUGH the smoke (1 = clear air): the density of the
/// cell each quarter metre of the line lies in, summed, times `sigma` per metre per unit. Read from the smoke alone: no box
/// is asked, so it answers whether or not a tick is open.
pub fn trans(s: &Smoke, a: [f64; 3], b: [f64; 3], sigma: f64) -> f64 {
    if s.wide == 0 {
        return 1.0;
    }
    let (dx, dy, dz) = (b[0] - a[0], b[1] - a[1], b[2] - a[2]);
    let len = hypot3(dx, dy, dz);
    if !(len >= 1e-6) || !(len < 1e6) {
        return 1.0;
    }
    let n = -floor(-len / (CELL * 0.5));
    let ds = len / n;
    let mut sum = 0.0;
    let mut i = 0.0;
    while i < n {
        let t = (i + 0.5) / n;
        sum += rho_at(s, a[0] + dx * t, a[1] + dy * t, a[2] + dz * t);
        i += 1.0;
    }
    let tau = sigma * sum * ds;
    if tau <= 0.0 {
        1.0
    } else {
        exp(-tau)
    }
}

/// the density of the cell a point lies in (0 where there is no brick, and above and below the grid)
#[inline]
fn rho_at(s: &Smoke, px: f64, py: f64, pz: f64) -> f64 {
    let (fx, fy, fz) = (floor(px / CELL), floor(py / CELL), floor(pz / CELL));
    // (a point that is no number, or off the plan, meets nothing)
    if !(fx >= 0.0 && fz >= 0.0 && fy >= 0.0 && fy < BY as f64 && fx < (s.wide as usize * BX) as f64 && fz < (s.deep as usize * BX) as f64) {
        return 0.0;
    }
    let (x, y, z) = (fx as usize, fy as usize, fz as usize);
    match s.at[z / BX * s.wide as usize + x / BX] {
        0 => 0.0,
        b => s.rho[(b as usize - 1) * CELLS + cell_at(x % BX, y, z % BX)],
    }
}
