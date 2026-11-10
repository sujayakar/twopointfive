//! WHAT OF THE SKY REACHES A PLACE UNDER IT: fifteen ways up and eight down to the ground about it, asked of the statics from the middle of the place's half metre (or of its
//! quarter, where something stands between the place and that), and which of them are open. What the light is worth that comes by them is the rules' (game_rules hours).
//!
//! The answer is kept, a table a slot of the games, no state: the ways that pass something which has changed since are asked again, those and no more (`glow::Marks`: every
//! static that is written is marked on the plan).

use game_types::knobs::Knobs;
use crate::glow::{self, Marks};
use crate::lines;
use crate::maths::{cos, sin};
use crate::state::GAMES;
use crate::world::{js_max, js_min, World, F_GLASS};
use prim::floor;

/// a way: its direction, and what it counts for on each of the six faces
#[derive(Clone, Copy)]
pub struct Ray {
    pub d: [f64; 3],
    pub c6: [f64; 6],
}
pub const SKY: usize = 15;
pub const GROUND: usize = 8;
/// THE WAYS, up and down
pub struct Fan {
    pub sky: [Ray; SKY],
    pub ground: [Ray; GROUND],
    made: bool,
}
const NO_RAY: Ray = Ray { d: [0.0; 3], c6: [0.0; 6] };
static mut FAN: Fan = Fan { sky: [NO_RAY; SKY], ground: [NO_RAY; GROUND], made: false };
const PI: f64 = core::f64::consts::PI;
const DEG: f64 = 0.017453292519943295;
/// how far down the ways to the ground go
const DOWN: f64 = -0.20943951023931956;
/// the height the ways go out from
pub const HEIGHT: f64 = 1.3;

/// `out.len()` ways round at one height, the first `turn` of a step on
fn round(out: &mut [Ray], elev: f64, w: f64, turn: f64) {
    let n = out.len() as f64;
    for (i, r) in out.iter_mut().enumerate() {
        let a = (i as f64 + turn) / n * 2.0 * PI;
        let d = [cos(a) * cos(elev), sin(elev), sin(a) * cos(elev)];
        // (a face's normal has two noughts, whose products are added)
        let face = |n: [f64; 3]| js_max(0.0, n[0] * d[0] + n[1] * d[1] + n[2] * d[2]) * w;
        *r = Ray { d, c6: [face([1.0, 0.0, 0.0]), face([-1.0, 0.0, 0.0]), face([0.0, 1.0, 0.0]), face([0.0, -1.0, 0.0]), face([0.0, 0.0, 1.0]), face([0.0, 0.0, -1.0])] };
    }
}
pub fn fan() -> &'static Fan {
    let f = unsafe { &mut *core::ptr::addr_of_mut!(FAN) };
    if !f.made {
        let s = |deg: f64| sin(deg * DEG);
        round(&mut f.sky[..1], 90.0 * DEG, 2.0 * PI * (1.0 - s(70.0)), 0.0);
        round(&mut f.sky[1..7], 50.0 * DEG, 2.0 * PI * (s(70.0) - s(32.0)) / 6.0, 0.0);
        round(&mut f.sky[7..], 16.0 * DEG, 2.0 * PI * s(32.0) / 8.0, 0.5);
        round(&mut f.ground, DOWN, 2.0 * PI * sin(25.0 * DEG) / 8.0, 0.25);
        f.made = true;
    }
    f
}

/// the box the shadow casters stand in
#[derive(Clone, Copy, PartialEq)]
pub struct Casters {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub top: f64,
}
impl Casters {
    /// how far a ray from P along d runs before it is out of them (60 m at most)
    pub fn reach(&self, p: [f64; 3], d: [f64; 3]) -> f64 {
        let mut t = 60.0;
        if d[1] > 1e-4 { t = js_min(t, (self.top - p[1]) / d[1]); }
        if d[0] > 1e-4 { t = js_min(t, (self.x1 - p[0]) / d[0]); } else if d[0] < -1e-4 { t = js_min(t, (self.x0 - p[0]) / d[0]); }
        if d[2] > 1e-4 { t = js_min(t, (self.z1 - p[2]) / d[2]); } else if d[2] < -1e-4 { t = js_min(t, (self.z0 - p[2]) / d[2]); }
        js_max(t, 0.0)
    }
    /// …and one of the ways down, to the ground
    pub fn reach_down(&self, p: [f64; 3], d: [f64; 3]) -> f64 {
        js_min((p[1] - 0.03) / -d[1], self.reach(p, d))
    }
}

/// WHICH WAYS ARE OPEN, a bit each: up, and down to ground the sun stands on and to ground that has the sky over it
#[derive(Clone, Copy, Default)]
pub struct Open {
    pub sky: u16,
    pub sun: u8,
    pub ground: u8,
}
/// a place's half metre, or its quarter
#[derive(Clone, Copy)]
pub enum Of {
    Half(i32, i32),
    Quarter(i32, i32),
}
#[derive(Clone, Copy)]
struct Kept {
    /// the change of the statics it is of (0 = none), and which quarter
    of: u32,
    key: u32,
    open: Open,
}
const NONE: Kept = Kept { of: 0, key: 0, open: Open { sky: 0, sun: 0, ground: 0 } };
const HALVES: usize = glow::MAX_HALF;
const QUARTERS: usize = 4096;
struct Table {
    half: [Kept; HALVES],
    quarter: [Kept; QUARTERS],
    /// which standing of the statics these are of
    stood: u32,
    casters: Casters,
    to_sun: [f64; 3],
}
static mut TABLES: [Table; GAMES] = [const { Table { half: [NONE; HALVES], quarter: [NONE; QUARTERS], stood: 0, casters: Casters { x0: 0.0, x1: 0.0, z0: 0.0, z1: 0.0, top: 0.0 }, to_sun: [0.0; 3] } }; GAMES];

/// what a question is of: the statics, the box they stand in, where the sun is
pub struct Under<'a> {
    pub w: &'a World,
    pub casters: Casters,
    pub to_sun: [f64; 3],
    pub tune: &'a Knobs,
}
impl Under<'_> {
    /// the statics alone between a and b: no door leaf is in the way
    fn shut(&self, a: [f64; 3], b: [f64; 3]) -> bool {
        lines::sight_line(self.w, false, F_GLASS, a[0], a[1], a[2], b[0], b[1], b[2], self.tune) == Some(true)
    }
    fn sky(&self, at: [f64; 3], d: [f64; 3]) -> bool {
        let len = self.casters.reach(at, d);
        len > 0.05 && !self.shut(at, along(at, d, len))
    }
    /// → has the ground there the sun, the sky
    fn ground(&self, at: [f64; 3], d: [f64; 3]) -> (bool, bool) {
        let len = self.casters.reach_down(at, d);
        if len < 0.3 {
            return (false, false);
        }
        let p = along(at, d, len);
        if p[1] > 0.2 || self.shut(at, p) {
            return (false, false);
        }
        (self.to_sun[1] > 0.0 && !self.shut(p, along(p, self.to_sun, self.casters.reach(p, self.to_sun))), !self.shut(p, [p[0], self.casters.top, p[2]]))
    }
    /// every way, or those `again` says
    fn ask(&self, at: [f64; 3], open: &mut Open, again: impl Fn(bool, [f64; 3]) -> bool) {
        let f = fan();
        for (i, r) in f.sky.iter().enumerate() {
            if again(true, r.d) {
                let bit = 1 << i;
                open.sky = if self.sky(at, r.d) { open.sky | bit } else { open.sky & !bit };
            }
        }
        for (i, r) in f.ground.iter().enumerate() {
            if again(false, r.d) {
                let (bit, (sun, sky)) = (1 << i, self.ground(at, r.d));
                open.sun = if sun { open.sun | bit } else { open.sun & !bit };
                open.ground = if sky { open.ground | bit } else { open.ground & !bit };
            }
        }
    }
    /// whether a way passes something that has changed since
    fn changed(&self, m: &Marks, since: u32, at: [f64; 3], up: bool, d: [f64; 3]) -> bool {
        if up {
            // (what stands under the start of a way that rises is never met)
            return m.passed(at, d, self.casters.reach(at, d), false, since);
        }
        let len = self.casters.reach_down(at, d);
        let p = along(at, d, len);
        m.passed(at, d, len, true, since) || (self.to_sun[1] > 0.0 && m.passed(p, self.to_sun, self.casters.reach(p, self.to_sun), true, since))
    }
    /// WHICH WAYS ARE OPEN FROM `at`. `of`: the half metre or the quarter whose middle that is, whose answer is kept for the game in slot `g`
    pub fn open(&self, g: usize, at: [f64; 3], of: Option<Of>) -> Open {
        let t = unsafe { &mut (*core::ptr::addr_of_mut!(TABLES))[g] };
        let marks = glow::marks(g, self.w);
        let stood = marks.map_or(0, |m| m.stood);
        if t.casters != self.casters || t.to_sun != self.to_sun || t.stood != stood {
            (t.casters, t.to_sun, t.stood) = (self.casters, self.to_sun, stood);
            t.half.fill(NONE);
            t.quarter.fill(NONE);
        }
        let kept = match (of, marks) {
            (Some(Of::Half(x, z)), Some(m)) if m.on(x, z) => Some((&mut t.half[z as usize * m.nx * 2 + x as usize], 0)),
            (Some(Of::Quarter(x, z)), Some(m)) if m.on(x >> 1, z >> 1) => {
                let key = z as u32 * m.nx as u32 * 4 + x as u32;
                Some((&mut t.quarter[key as usize % QUARTERS], key))
            }
            _ => None,
        };
        let mut open = Open::default();
        match (kept, marks) {
            (Some((k, key)), Some(m)) => {
                if k.of == 0 || k.key != key {
                    self.ask(at, &mut open, |_, _| true);
                } else {
                    open = k.open;
                    if k.of != m.now {
                        self.ask(at, &mut open, |up, d| self.changed(m, k.of, at, up, d));
                    }
                }
                *k = Kept { of: m.now, key, open };
            }
            _ => self.ask(at, &mut open, |_, _| true),
        }
        open
    }
}
fn along(a: [f64; 3], d: [f64; 3], t: f64) -> [f64; 3] {
    [a[0] + d[0] * t, a[1] + d[1] * t, a[2] + d[2] * t]
}
/// the half metre, or the quarter, a place is in
pub fn half(p: [f64; 3]) -> (f64, f64) {
    (floor(p[0] / 0.5), floor(p[2] / 0.5))
}
pub fn quarter(p: [f64; 3]) -> (f64, f64) {
    (floor(p[0] / 0.25), floor(p[2] / 0.25))
}
