//! THE WEATHER'S DUST IN THE ROOMS — src/game/airDust.ts's field: the share of the open air's density at each half metre of the plan, what the men's eyes read along a
//! line (weather.rs air_trans), carried through the openings between cells and settling back to the roof's leak. Statement for statement, the field and its openings a
//! part of the weather's table. What the picture takes (a copy for the renderer, sent when it has moved enough) stays TypeScript and reads this.
//!
//! An opening between two cells is asked of the statics alone (three heights, both ways), and of the door leaves as rectangles, re-read whenever a leaf has moved.
//!
//! WHAT IS NOT STATE and is made again (`Plan`, a set a slot of the blocks, as bricks.rs's tables): which cells are roofed and the lists walked over them are the roofs',
//! and which edges a leaf stands across is the leaves'. Whose a slot's set is goes by the block's stamp (src/sim/state.ts SimState.stamp).
use crate::knobs::{k, AIRDUST_DIFFUSE, AIRDUST_RECHECK_SECS, AIRDUST_SETTLE, AIRDUST_STEP};
use crate::lines;
use crate::maths::{cos, hypot2, sin};
use crate::state::GAMES;
use crate::world::{js_abs, js_ceil, js_max, js_min, world, K_ROT, MAX_LEAVES, REC, R_C, R_H, R_KIND, R_YAW};
use core::mem::size_of;
use prim::floor;

/// the plan: scene/airGrid.ts's half-metre cells over the 40 × 28 m world (the host holds its own numbers to these as it stands up: airDust.ts)
pub const AIR_W: usize = 80;
pub const AIR_H: usize = 56;
pub const AIR_CELL: f64 = 0.5;
pub const N: usize = AIR_W * AIR_H;
/// the world's corner on the plan (scene/world.ts WORLD.min)
pub const X0: f64 = 0.0;
pub const Z0: f64 = 0.0;
/// the heights a way between two cells is asked at
const PROBE_Y: [f64; 3] = [1.6, 0.4, 2.05];
/// a share this close to the leak IS the leak
const SNAP: f64 = 1e-6;

#[repr(C)]
pub struct Air {
    /// the men's eyes read the field this run
    pub on: f64,
    pub leak: f64,
    pub acc: f64,
    /// the re-check's place in `edges`
    pub cur: f64,
    pub n_cells: f64,
    pub n_edges: f64,
    /// the share of the open air's density, a cell
    pub s: [f64; N],
    /// by edge: no static stands across it
    pub stat: [u8; N * 2],
}
const _: () = assert!(size_of::<Air>() == 6 * 8 + 8 * N + 2 * N);
const _: () = assert!(N % 4 == 0);

/// the tables of a slot of the blocks: what the block's roofs and the door leaves make
pub struct Plan {
    /// the stamp of the block they were made from
    made: f64,
    /// the leaves as they stood when `leaf` was last read: one 32-bit hash, or no number at all
    sig: f64,
    /// the roofed cells, and the edges with a roofed cell at either end (cell × 2 + 0 to +x, 1 to +z): in the order the step walks them and the re-check goes round them
    pub cells: [u32; N],
    pub edges: [u32; N * 2],
    pub indoor: [u8; N],
    /// by edge: how many leaves stand across it
    pub leaf: [u8; N * 2],
}
static mut PLANS: [Plan; GAMES] = [const { Plan { made: 0.0, sig: 0.0, cells: [0; N], edges: [0; N * 2], indoor: [0; N], leaf: [0; N * 2] } }; GAMES];

/// THE TABLES OF SLOT `g` ARE THE BLOCK'S THAT STANDS THERE: made from the block of this `stamp`, or made again now (no roofed list in the block: none here either). The leaves'
/// half is owed: the next look at them reads them.
pub fn plan(g: usize, stamp: f64, air: &Air, roofs: &[f64], n: usize) -> &'static mut Plan {
    let p = unsafe { &mut (*core::ptr::addr_of_mut!(PLANS))[g] };
    if p.made != stamp {
        p.made = stamp;
        p.sig = f64::NAN;
        p.leaf.fill(0);
        if air.n_cells == 0.0 && air.n_edges == 0.0 {
            p.indoor.fill(0);
        } else {
            p.make(|x, z| crate::weather::under(roofs, n, x, z));
        }
    }
    p
}

impl Plan {
    /// the cells `roofed` answers for by their centres, and the lists over them → how many of each
    fn make(&mut self, roofed: impl Fn(f64, f64) -> bool) -> (usize, usize) {
        let mut nc = 0;
        let mut i = 0;
        for iz in 0..AIR_H {
            for ix in 0..AIR_W {
                let r = roofed(X0 + (ix as f64 + 0.5) * AIR_CELL, Z0 + (iz as f64 + 0.5) * AIR_CELL);
                self.indoor[i] = if r { 1 } else { 0 };
                if r {
                    self.cells[nc] = i as u32;
                    nc += 1;
                }
                i += 1;
            }
        }
        let mut ne = 0;
        let mut i = 0;
        for iz in 0..AIR_H {
            for ix in 0..AIR_W {
                if ix + 1 < AIR_W && (self.indoor[i] != 0 || self.indoor[i + 1] != 0) {
                    self.edges[ne] = (i * 2) as u32;
                    ne += 1;
                }
                if iz + 1 < AIR_H && (self.indoor[i] != 0 || self.indoor[i + AIR_W] != 0) {
                    self.edges[ne] = (i * 2 + 1) as u32;
                    ne += 1;
                }
                i += 1;
            }
        }
        (nc, ne)
    }
}

static mut NEXT: [f64; N] = [0.0; N];

/// what a step tells the host: how far the field moved, the steps and the lines asked, the open ways the last step found
pub struct Said {
    pub moved: f64,
    pub steps: f64,
    pub probes: f64,
    pub open: f64,
}

/// the fixed step in use (airDust.ts stepSecs): held to what the explicit scheme and a frame can take
#[inline]
fn step_secs() -> f64 {
    let h = k(AIRDUST_STEP);
    if h >= 1.0 / 240.0 && h <= 0.1 {
        h
    } else {
        1.0 / 30.0
    }
}
/// `+x || 0`
#[inline(always)]
fn or_zero(x: f64) -> f64 {
    if x != x {
        0.0
    } else {
        x
    }
}

impl Said {
    pub fn tell(&self, out: &mut [f64]) {
        out[0] = self.moved;
        out[1] = self.steps;
        out[2] = self.probes;
        out[3] = self.open;
    }
}

impl Air {
    /// airDust.ts build's head: its switch, nothing owed, no opening known yet
    pub fn begin(&mut self, p: &mut Plan, on: bool) {
        self.on = if on { 1.0 } else { 0.0 };
        self.acc = 0.0;
        self.cur = 0.0;
        p.sig = f64::NAN;
        p.leaf.fill(0);
        self.stat.fill(0);
    }
    /// no weather in the air: every cell the open air's
    pub fn clear(&mut self, p: &mut Plan) {
        self.s.fill(1.0);
        p.indoor.fill(0);
        self.n_cells = 0.0;
        self.n_edges = 0.0;
    }
    /// airDust.ts shareAt: off the grid = the open air
    #[inline]
    pub fn share_at(&self, x: f64, z: f64) -> f64 {
        let ix = floor((x - X0) / AIR_CELL);
        let iz = floor((z - Z0) / AIR_CELL);
        if ix < 0.0 || iz < 0.0 || ix >= AIR_W as f64 || iz >= AIR_H as f64 {
            1.0
        } else {
            self.s[ix as usize + iz as usize * AIR_W]
        }
    }
    /// airDust.ts ask: is the way along edge `e` clear of every static
    fn ask(&self, wo: usize, ask: u32, e: usize, probes: &mut f64) -> bool {
        let i = e >> 1;
        let j = if e & 1 != 0 { i + AIR_W } else { i + 1 };
        let ax = X0 + ((i % AIR_W) as f64 + 0.5) * AIR_CELL;
        let az = Z0 + ((i / AIR_W) as f64 + 0.5) * AIR_CELL;
        let bx = X0 + ((j % AIR_W) as f64 + 0.5) * AIR_CELL;
        let bz = Z0 + ((j / AIR_W) as f64 + 0.5) * AIR_CELL;
        for y in PROBE_Y.iter() {
            *probes += 1.0;
            if lines::solid_line(wo, -1, ask, ax, *y, az, bx, *y, bz) != Some(true) && lines::solid_line(wo, -1, ask, bx, *y, bz, ax, *y, az) != Some(true) {
                return true;
            }
        }
        false
    }
    /// airDust.ts recheck: the shut edges asked again, a share of the list a step
    fn recheck(&mut self, p: &Plan, wo: usize, ask: u32, h: f64, probes: &mut f64) {
        let n = self.n_edges as usize;
        if n == 0 {
            return;
        }
        let per = js_min(n as f64, js_max(1.0, js_ceil(n as f64 * h / js_max(0.25, or_zero(k(AIRDUST_RECHECK_SECS)))))) as usize;
        let mut cur = self.cur as usize;
        for _ in 0..per {
            let e = p.edges[cur] as usize;
            cur = (cur + 1) % n;
            if self.stat[e] == 0 && self.ask(wo, ask, e, probes) {
                self.stat[e] = 1;
            }
        }
        self.cur = cur as f64;
    }
    /// airDust.ts leaves: which edges the door leaves stand across, re-read only when a leaf has moved
    fn leaves(p: &mut Plan, wo: usize, set: i32) {
        let o = world(wo);
        let n = if set < 0 { 0 } else { o.leaves[set as usize] as usize };
        let base = if set < 0 { 0 } else { set as usize * MAX_LEAVES * REC };
        let rows = &o.leaf;
        let mut sig = n as i32;
        for i in 0..n {
            let q = base + i * REC;
            let rot = (rows[q + R_KIND] as u32) & K_ROT != 0;
            sig = (sig ^ js_round(rows[q + R_C] * 1000.0)).wrapping_mul(16777619);
            sig = (sig ^ js_round(rows[q + R_C + 2] * 1000.0)).wrapping_mul(16777619);
            sig = (sig ^ js_round(rows[q + R_YAW] * 1000.0)).wrapping_mul(16777619);
            sig = (sig ^ js_round(rows[q + R_C + 1] * 1000.0) ^ (if rot { 0x5bd1e995 } else { 0 })).wrapping_mul(16777619);
        }
        if sig as f64 == p.sig {
            return;
        }
        p.sig = sig as f64;
        p.leaf.fill(0);
        let (w_max, h_max) = ((AIR_W - 1) as f64, (AIR_H - 1) as f64);
        for i in 0..n {
            let q = base + i * REC;
            let (c0, c1, c2, h0, h1, h2) = (rows[q + R_C], rows[q + R_C + 1], rows[q + R_C + 2], rows[q + R_H], rows[q + R_H + 1], rows[q + R_H + 2]);
            let rot = (rows[q + R_KIND] as u32) & K_ROT != 0;
            if rot || c1 - h1 > 1.0 || c1 + h1 < 1.0 {
                continue;
            }
            let yaw = rows[q + R_YAW];
            let (cs, sn) = (cos(yaw), sin(yaw));
            let (hx, hz) = (h0 + 0.02, h2 + 0.02);
            let r = hypot2(h0, h2) + AIR_CELL;
            let ix0 = js_max(0.0, floor((c0 - r - X0) / AIR_CELL - 0.5)) as usize;
            let ix1 = js_min(w_max, js_ceil((c0 + r - X0) / AIR_CELL - 0.5));
            let iz0 = js_max(0.0, floor((c2 - r - Z0) / AIR_CELL - 0.5)) as usize;
            let iz1 = js_min(h_max, js_ceil((c2 + r - Z0) / AIR_CELL - 0.5));
            let mut iz = iz0;
            while iz as f64 <= iz1 {
                let mut ix = ix0;
                while ix as f64 <= ix1 {
                    let px = X0 + (ix as f64 + 0.5) * AIR_CELL - c0;
                    let pz = Z0 + (iz as f64 + 0.5) * AIR_CELL - c2;
                    let lx = cs * px - sn * pz;
                    let lz = sn * px + cs * pz;
                    let c = ix + iz * AIR_W;
                    if ix + 1 < AIR_W && crosses(lx, lz, cs * AIR_CELL, sn * AIR_CELL, hx, hz) {
                        p.leaf[c * 2] = p.leaf[c * 2].wrapping_add(1);
                    }
                    if iz + 1 < AIR_H && crosses(lx, lz, -sn * AIR_CELL, cs * AIR_CELL, hx, hz) {
                        p.leaf[c * 2 + 1] = p.leaf[c * 2 + 1].wrapping_add(1);
                    }
                    ix += 1;
                }
                iz += 1;
            }
        }
    }
    /// airDust.ts stepOnce: one step of the equation → the most any cell moved
    fn step_once(&mut self, p: &Plan, dt: f64, open_out: &mut f64) -> f64 {
        let next = unsafe { &mut *core::ptr::addr_of_mut!(NEXT) };
        let leak = self.leak;
        let ks = js_min(0.2, js_max(0.0, or_zero(k(AIRDUST_DIFFUSE))) * dt / (AIR_CELL * AIR_CELL));
        let r = js_min(0.5, js_max(0.0, or_zero(k(AIRDUST_SETTLE))) * dt);
        let (mut moved, mut open) = (0.0, 0.0);
        let n = self.n_cells as usize;
        for q in 0..n {
            let i = p.cells[q] as usize;
            let v = self.s[i];
            let ix = i % AIR_W;
            let mut a = 0.0;
            if ix > 0 {
                let e = (i - 1) * 2;
                if self.stat[e] != 0 && p.leaf[e] == 0 {
                    a += self.s[i - 1] - v;
                    open += 1.0;
                }
            }
            if ix + 1 < AIR_W {
                let e = i * 2;
                if self.stat[e] != 0 && p.leaf[e] == 0 {
                    a += self.s[i + 1] - v;
                    open += 1.0;
                }
            }
            if i >= AIR_W {
                let e = (i - AIR_W) * 2 + 1;
                if self.stat[e] != 0 && p.leaf[e] == 0 {
                    a += self.s[i - AIR_W] - v;
                    open += 1.0;
                }
            }
            if i + AIR_W < N {
                let e = i * 2 + 1;
                if self.stat[e] != 0 && p.leaf[e] == 0 {
                    a += self.s[i + AIR_W] - v;
                    open += 1.0;
                }
            }
            let mut w = v + ks * a - r * (v - leak);
            if js_abs(w - leak) < SNAP {
                w = leak;
            }
            let d = js_abs(w - v);
            if d > moved {
                moved = d;
            }
            next[i] = w;
        }
        for q in 0..n {
            let i = p.cells[q] as usize;
            self.s[i] = next[i];
        }
        *open_out = open;
        moved
    }
    /// airDust.ts build, once the roofs have been asked: the field from nothing. `roofed` answers for a cell's centre.
    pub fn seed(&mut self, p: &mut Plan, leak: f64, roofed: impl Fn(f64, f64) -> bool) {
        self.leak = leak;
        let (nc, ne) = p.make(roofed);
        for i in 0..N {
            self.s[i] = if p.indoor[i] != 0 { leak } else { 1.0 };
        }
        self.n_cells = nc as f64;
        self.n_edges = ne as f64;
    }
    /// …and with the switch on: every opening asked, the leaves read, and run on until it stands still (the level opens with the dust already in its open doorways)
    pub fn settle(&mut self, p: &mut Plan, wo: usize, set: i32, ask: u32, said: &mut Said) {
        let n = self.n_edges as usize;
        for q in 0..n {
            let e = p.edges[q] as usize;
            self.stat[e] = if self.ask(wo, ask, e, &mut said.probes) { 1 } else { 0 };
        }
        Air::leaves(p, wo, set);
        let h = step_secs();
        let mut i = 0;
        while i < 1500 {
            said.steps += 1.0;
            if !(self.step_once(p, h, &mut said.open) > 1e-4) {
                break;
            }
            i += 1;
        }
    }
    /// airDust.ts update: through the openings as they stand this tick
    pub fn update(&mut self, p: &mut Plan, wo: usize, set: i32, ask: u32, dt: f64, said: &mut Said) {
        let h = step_secs();
        self.acc += dt;
        let mut n = 0;
        while self.acc >= h && n < 4 {
            n += 1;
            self.acc -= h;
            self.recheck(p, wo, ask, h, &mut said.probes);
            Air::leaves(p, wo, set);
            said.steps += 1.0;
            said.moved += self.step_once(p, h, &mut said.open);
        }
        if self.acc > h {
            self.acc = h;
        }
    }
    /// airDust.ts restore's last line: the leaves read again over a field put back
    pub fn reread(p: &mut Plan, wo: usize, set: i32) {
        p.sig = f64::NAN;
        Air::leaves(p, wo, set);
    }
}

/// Math.round of a number a leaf's place makes, as a word (ToInt32 of it)
#[inline(always)]
fn js_round(x: f64) -> i32 {
    floor(x + 0.5) as i64 as i32
}
/// airDust.ts crosses: does the plan segment from (x, z) along (dx, dz), t in [0, 1], enter the rectangle |x| ≤ hx, |z| ≤ hz
#[inline]
fn crosses(x: f64, z: f64, dx: f64, dz: f64, hx: f64, hz: f64) -> bool {
    let (mut t0, mut t1) = (0.0, 1.0);
    if js_abs(dx) < 1e-9 {
        if js_abs(x) > hx {
            return false;
        }
    } else {
        let (mut a, mut b) = ((-hx - x) / dx, (hx - x) / dx);
        if a > b {
            core::mem::swap(&mut a, &mut b);
        }
        if a > t0 {
            t0 = a;
        }
        if b < t1 {
            t1 = b;
        }
        if t0 > t1 {
            return false;
        }
    }
    if js_abs(dz) < 1e-9 {
        if js_abs(z) > hz {
            return false;
        }
    } else {
        let (mut a, mut b) = ((-hz - z) / dz, (hz - z) / dz);
        if a > b {
            core::mem::swap(&mut a, &mut b);
        }
        if a > t0 {
            t0 = a;
        }
        if b < t1 {
            t1 = b;
        }
        if t0 > t1 {
            return false;
        }
    }
    true
}
