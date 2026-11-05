//! A LEAF OFF ITS HINGES, IN THE AIR (src/game/doorblast.ts stepSlab) — the loose slab a blast or a boot at a run sent out of its frame, for one tick: it
//! flies, drags its foot, topples about it, is stopped dead by the walls its rays find, is pushed out of what its toppling reaches, slams flat, slides
//! out and comes to rest clear of every static.
//!
//! STATEMENT FOR STATEMENT, as doors.rs and props.rs are. tools/qa/slab-ident.ts and the missions of tools/qa/rust-check.ts hold the two together.
//!
//! WHAT IS STATE, AND WHAT IS HANDED IN. A slab's own numbers are the `slabs` part of the game's block (state.rs), laid out as the furniture's are
//! (props.rs says why): DENSE ROWS, and the rows in the air first — rows 0 .. fly are the slabs a tick steps, in the order of their doors, with no test;
//! rows fly .. n the ones lying at rest, which nothing steps again. `door[row]` says whose leaf a row is, `row[door]` where a door's slab is, PLUS ONE
//! (0 = that door has none: a part of zeros is no slab at all). The host moves rows (src/sim/slabshome.ts SlabStore: a leaf torn off, a slab come to
//! rest, a door hung again); no rule here does, so a slab that comes to rest in a step is still in its row when the call returns, with F_REST set.
//! Eleven doubles are read every substep of a slab in the air (where it is, how it moves, its heading and spin, its tip and its clock) and lie
//! together; the angle its door reads as, the clock it is wedged until and the lock rail's four numbers are cold. The truths are bits of `flags`.
//! 152 bytes a row.
//!
//! One slab a call: the host steps them in turn and poses each one's box before the next is stepped, as it always did, so a slab's rays meet the ones
//! before it where this tick left them. What the step needs of its door is an argument (`PAR`, scratch); what it met comes back the same way (the slam,
//! for the host to say; what its settling did, for the host's tallies).
//!
//! THE SLAB'S OWN BOX IS NOT A WALL TO IT: every ray here leaves one leaf of the world's copy out (`lines::ray_but`), the slab's own.
//! A RAY THE MODULE CANNOT WALK IS THE HOST'S (one of no length or no number: lines.rs ray_rule). The step then writes nothing and says so (HOST),
//! and the TypeScript takes this slab's step for this tick; the step works on a copy of its row for that. A pane of glass is no such case: a ray
//! meets it or flies on through a hole in it here (glass.rs).
use crate::collide;
use crate::lines::{ray_rule, RAY_HOST};
use crate::maths::{cos, exp, hypot2, sin};
use crate::state::state;
use crate::world::{clamp, js_abs, js_ceil, js_max, js_min, or_one, world, REC, R_C, R_COS, R_H, R_SIN};
use core::mem::{offset_of, size_of};

/// slabs a game can have: one a door (doors.rs CAP). A multiple of 4.
pub const CAP: usize = 64;
const _: () = assert!(CAP % 4 == 0 && CAP == crate::doors::CAP);

// a slab's truths (`flags`): lying at rest · slammed flat · its top fell the way `side` −1 names · it is the lock rail of a leaf gunfire took apart
pub const F_REST: u32 = 1;
pub const F_LANDED: u32 = 2;
pub const F_BACK: u32 = 4;
pub const F_BITS: u32 = 8;

/// The slabs' part of a game's block (src/sim/slabshome.ts SlabStore lays its own array out the same, byte for byte).
#[repr(C)]
pub struct Slabs {
    /// slabs in all, and how many of them are in the air (rows 0 .. fly)
    pub n: u32,
    pub fly: u32,
    pub pad: [u32; 2],
    // ---- doubles a slab in the air reads every substep
    pub px: [f64; CAP],
    pub py: [f64; CAP],
    pub pz: [f64; CAP],
    pub vx: [f64; CAP],
    pub vy: [f64; CAP],
    pub vz: [f64; CAP],
    pub a: [f64; CAP],
    pub spin: [f64; CAP],
    pub tip: [f64; CAP],
    pub tip_v: [f64; CAP],
    pub t: [f64; CAP],
    // ---- cold: the angle the door reads as, the clock a wedged slab is not searched for before, the lock rail (half width, half length, the handles on it)
    pub ang: [f64; CAP],
    pub wedged: [f64; CAP],
    pub bw: [f64; CAP],
    pub bl: [f64; CAP],
    pub bx: [f64; CAP],
    pub by: [f64; CAP],
    // ---- words
    pub flags: [u32; CAP],
    pub who: [i32; CAP],
    /// whose leaf a row is, and where a door's slab is plus one (0: none)
    pub door: [u32; CAP],
    pub row: [u32; CAP],
}
const _: () = assert!(size_of::<Slabs>() == 16 + 17 * CAP * 8 + 4 * CAP * 4);
const _: () = assert!(offset_of!(Slabs, px) % 16 == 0 && offset_of!(Slabs, flags) % 16 == 0 && (CAP * 4) % 16 == 0);
/// the columns in their order (src/sim/slabshome.ts SLAB_F64 and SLAB_U32 name them the same)
pub const COLS: [usize; 21] = [
    offset_of!(Slabs, px),
    offset_of!(Slabs, py),
    offset_of!(Slabs, pz),
    offset_of!(Slabs, vx),
    offset_of!(Slabs, vy),
    offset_of!(Slabs, vz),
    offset_of!(Slabs, a),
    offset_of!(Slabs, spin),
    offset_of!(Slabs, tip),
    offset_of!(Slabs, tip_v),
    offset_of!(Slabs, t),
    offset_of!(Slabs, ang),
    offset_of!(Slabs, wedged),
    offset_of!(Slabs, bw),
    offset_of!(Slabs, bl),
    offset_of!(Slabs, bx),
    offset_of!(Slabs, by),
    offset_of!(Slabs, flags),
    offset_of!(Slabs, who),
    offset_of!(Slabs, door),
    offset_of!(Slabs, row),
];

// ---- WHAT THE HOST HANDS IN AND TAKES BACK FOR ONE SLAB'S STEP (scratch)
// in: the frame's dt, the row, the leaf (its height, thickness and half width, the heading it shuts at, the middle of its frame), which leaf of the
// world's copy is its own box (−1: none of them), whether the host has a rule for panes, and doorblast.ts DOORBLAST's numbers
pub const P_DT: usize = 0;
pub const P_ROW: usize = 1;
pub const P_HEIGHT: usize = 2;
pub const P_THICK: usize = 3;
pub const P_HU: usize = 4;
pub const P_CLOSED: usize = 5;
pub const P_FCX: usize = 6;
pub const P_FCZ: usize = 7;
pub const P_BUT: usize = 8;
pub const P_ASK: usize = 9;
pub const P_SCRAPE: usize = 10;
pub const P_SLAM: usize = 11;
pub const P_SLIDE: usize = 12;
pub const P_BOUNCE: usize = 13;
pub const P_TIP: usize = 14;
// out: it slammed flat this step (and where, for the noise) · it came to rest · what its settling did (slabs moved clear, the furthest, laid in their
// own doorway) · counts for the checks (rays asked, gathers, substeps)
pub const P_LANDED: usize = 16;
pub const P_LAND_X: usize = 17;
pub const P_LAND_Z: usize = 18;
pub const P_RESTED: usize = 19;
pub const P_NUDGED: usize = 20;
pub const P_NUDGE_MAX: usize = 21;
pub const P_LAID: usize = 22;
pub const P_RAYS: usize = 23;
pub const P_GATHERS: usize = 24;
pub const P_SUBSTEPS: usize = 25;
pub const NPAR: usize = 32;

/// the step's answers: taken · the host's to take (a ray ended on a pane, or was no number: nothing was written) · not a step this could be (no such row,
/// or a slab at rest)
pub const STEPPED: u32 = 0;
pub const HOST: u32 = 1;
pub const WRONG: u32 = 2;

static mut PAR: [f64; NPAR] = [0.0; NPAR];
#[inline(always)]
fn par() -> &'static mut [f64; NPAR] {
    unsafe { &mut *core::ptr::addr_of_mut!(PAR) }
}
/// where the hand-over is (a byte address): 0 par
pub fn addr(table: u32) -> usize {
    match table {
        0 => par().as_ptr() as usize,
        _ => 0,
    }
}
/// 0 = rows · 1 = the hand-over's length · 13 = the part's length in bytes · 100 + k = where column k of the part starts (COLS)
pub fn cap(table: u32) -> usize {
    match table {
        0 => CAP,
        1 => NPAR,
        13 => size_of::<Slabs>(),
        k if k >= 100 && ((k - 100) as usize) < COLS.len() => COLS[(k - 100) as usize],
        _ => 0,
    }
}

// ---- doorblast.ts's own numbers (tools/qa/rust-check.ts reads both sources and is red where one differs)
const HALF_PI: f64 = core::f64::consts::PI / 2.0;
const PI: f64 = core::f64::consts::PI;
const LIFT: f64 = 0.005;
const G: f64 = 9.8;
/// statics one gather of the TypeScript's can list (doorblast.ts OV)
const OV: usize = 512;

/// a slab as the step works on it (the columns the step may write, copied in and, if the step is this module's to take, out)
#[derive(Clone, Copy)]
struct Now {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    a: f64,
    spin: f64,
    tip: f64,
    tip_v: f64,
    t: f64,
    wedged: f64,
    rest: bool,
    landed: bool,
}
/// its door, and the world it flies in
struct Leaf {
    w: usize,
    height: f64,
    thick: f64,
    hu: f64,
    closed: f64,
    fcx: f64,
    fcz: f64,
    but: i32,
    ask: bool,
    /// a ray ended where only the host can judge
    host: bool,
    rays: f64,
    gathers: f64,
    nudged: f64,
    nudge_max: f64,
    laid: f64,
}

/// doorblast.ts support: the slab's centre height with its foot (or its face) on the floor at this much tip
#[inline(always)]
fn support(d: &Leaf, tip: f64) -> f64 {
    (d.height / 2.0) * cos(tip) + (d.thick / 2.0) * sin(tip) + LIFT
}
/// doorblast.ts acrossOf: the slab's reach across its own line at this much tip
#[inline(always)]
fn across_of(d: &Leaf, tip: f64) -> f64 {
    (d.height / 2.0) * sin(tip) + (d.thick / 2.0) * cos(tip)
}
/// doorblast.ts reachAlong: its half extent along the horizontal unit direction (ex, ez)
#[inline(always)]
fn reach_along(d: &Leaf, s: &Now, ex: f64, ez: f64) -> f64 {
    let (ux, uz) = (cos(s.a), sin(s.a));
    js_abs(ex * ux + ez * uz) * d.hu + js_abs(-ex * uz + ez * ux) * across_of(d, s.tip)
}
/// doorblast.ts hitAt: the nearest static, or other door's leaf, along a horizontal ray: how far, and the face's normal in plan
fn hit_at(d: &mut Leaf, x: f64, y: f64, z: f64, ex: f64, ez: f64, tmax: f64) -> Option<(f64, f64, f64)> {
    let mut out = [0.0f64; 4];
    d.rays += 1.0;
    let k = ray_rule(d.w, x, y, z, ex, 0.0, ez, tmax, 0, d.but, d.ask, &mut out);
    if k == RAY_HOST {
        d.host = true; // (a ray collision.ts walks itself: the host's question — lines.rs ray_rule)
        return None;
    }
    if k < 0 {
        return None;
    }
    Some((out[0], out[1], out[3]))
}
/// doorblast.ts overlaps: does a floor rectangle — centre (x, z), heading a, half extents hu along it × hn across — cut into a static standing in the
/// heights [y0, y1]?
#[allow(clippy::too_many_arguments)]
fn overlaps(d: &mut Leaf, x: f64, z: f64, a: f64, hu: f64, hn: f64, y0: f64, y1: f64) -> bool {
    let (ux, uz, su, sn) = (cos(a), sin(a), hu - 0.012, hn - 0.012);
    let (ex, ez) = (js_abs(ux) * hu + js_abs(uz) * hn, js_abs(uz) * hu + js_abs(ux) * hn);
    d.gathers += 1.0;
    let n = collide::gather(d.w, x - ex, z - ez, x + ex, z + ez, y0, y1, OV);
    let o = world(d.w);
    for i in 0..n {
        let b = o.list[i] as usize * REC;
        let (bx, bz) = (o.rec[b + R_COS], -o.rec[b + R_SIN]);
        let (dx, dz) = (o.rec[b + R_C] - x, o.rec[b + R_C + 2] - z);
        let (h0, h2) = (o.rec[b + R_H], o.rec[b + R_H + 2]);
        let (c, s) = (js_abs(ux * bx + uz * bz), js_abs(ux * bz - uz * bx));
        if js_abs(dx * ux + dz * uz) > su + h0 * c + h2 * s {
            continue;
        }
        if js_abs(-dx * uz + dz * ux) > sn + h0 * s + h2 * c {
            continue;
        }
        if js_abs(dx * bx + dz * bz) > h0 + su * c + sn * s {
            continue;
        }
        if js_abs(-dx * bz + dz * bx) > h2 + su * s + sn * c {
            continue;
        }
        return true;
    }
    false
}
/// doorblast.ts freeSpot: the nearest spot within max_r where that rectangle stands clear (rings of sixteen bearings, nearest first)
#[allow(clippy::too_many_arguments)]
fn free_spot(d: &mut Leaf, x: f64, z: f64, a: f64, hu: f64, hn: f64, y0: f64, y1: f64, max_r: f64) -> Option<(f64, f64)> {
    let mut r = 0.03;
    while r <= max_r + 1e-9 {
        let mut i = 0.0;
        while i < 16.0 {
            let th = a + i * PI / 8.0;
            let (px, pz) = (x + cos(th) * r, z + sin(th) * r);
            if !overlaps(d, px, pz, a, hu, hn, y0, y1) {
                return Some((px, pz));
            }
            i += 1.0;
        }
        r += if r < 0.3 { 0.03 } else { 0.06 };
    }
    None
}
/// doorblast.ts unstick: in the air, once out of its frame, a slab that has come to cut into a static is moved to the nearest clear spot and loses what
/// speed it had into the thing
fn unstick(d: &mut Leaf, s: &mut Now) {
    let (hu, hn, y0, y1) = (d.hu, across_of(d, s.tip), 0.06, js_max(0.3, 2.0 * s.py - 0.06));
    if s.t < s.wedged || !overlaps(d, s.px, s.pz, s.a, hu, hn, y0, y1) {
        return;
    }
    let Some((t0, t1)) = free_spot(d, s.px, s.pz, s.a, hu, hn, y0, y1, 0.45) else {
        s.wedged = s.t + 0.25;
        return;
    };
    let (sx, sz) = (t0 - s.px, t1 - s.pz);
    let l = or_one(hypot2(sx, sz));
    let vn = (s.vx * sx + s.vz * sz) / l;
    if vn < 0.0 {
        s.vx -= vn * sx / l;
        s.vz -= vn * sz / l;
    }
    s.px = t0;
    s.pz = t1;
    s.spin *= 0.3;
}
/// doorblast.ts settle: at rest it lies clear of every static — where it stopped if that is clear; else the nearest clear spot, as it lies or turned a
/// little; else laid through its own doorway
fn settle(d: &mut Leaf, s: &mut Now) {
    let (hu, hn, y0, y1) = (d.hu, d.height / 2.0, 0.06, 0.3);
    if !overlaps(d, s.px, s.pz, s.a, hu, hn, y0, y1) {
        return;
    }
    d.nudged += 1.0;
    for da in [0.0, 0.2, -0.2, 0.45, -0.45, HALF_PI] {
        if let Some((t0, t1)) = free_spot(d, s.px, s.pz, s.a + da, hu, hn, y0, y1, if da != 0.0 { 0.9 } else { 1.2 }) {
            d.nudge_max = js_max(d.nudge_max, hypot2(t0 - s.px, t1 - s.pz));
            s.px = t0;
            s.pz = t1;
            s.a += da;
            return;
        }
    }
    let (a, fx, fz) = (d.closed, d.fcx, d.fcz);
    let to = if overlaps(d, fx, fz, a, hu, hn, y0, y1) { free_spot(d, fx, fz, a, hu, hn, y0, y1, 1.5) } else { Some((fx, fz)) };
    if let Some((t0, t1)) = to {
        s.px = t0;
        s.pz = t1;
        s.a = a;
        d.laid += 1.0;
    }
}
/// doorblast.ts pushOut: keep the slab's floor rectangle out of the statics as toppling (and its spin) lengthen it
fn push_out(d: &mut Leaf, s: &mut Now) {
    let (ux, uz) = (cos(s.a), sin(s.a));
    let (nx, nz, hu, hn) = (-uz, ux, d.hu, across_of(d, s.tip));
    let ym = clamp(s.py, 0.4, 1.3);
    let ways = [(nx, nz, hn, ux, uz, hu), (-nx, -nz, hn, ux, uz, hu), (ux, uz, hu, nx, nz, hn), (-ux, -uz, hu, nx, nz, hn)];
    for _pass in 0..2 {
        for (ex, ez, he, sx, sz, hs) in ways {
            let mut pen = 0.0;
            for f in [0.0, -0.8, 0.8] {
                let ys: &[f64] = if f == 0.0 { &[0.2, ym] } else { &[0.2] };
                for &y in ys {
                    if let Some((t, _, _)) = hit_at(d, s.px + sx * hs * f, y, s.pz + sz * hs * f, ex, ez, he + 0.02) {
                        if he - t > pen {
                            pen = he - t;
                        }
                    }
                }
            }
            if pen > 0.0 {
                s.px -= ex * (pen + 0.002);
                s.pz -= ez * (pen + 0.002);
                let vn = s.vx * ex + s.vz * ez;
                if vn > 0.0 {
                    s.vx -= vn * ex;
                    s.vz -= vn * ez;
                }
            }
        }
    }
}

/// doorblast.ts stepSlab for the slab in row PAR[P_ROW] of the game in slot `g`, over world `w` (which stands with the game's own leaves in it).
/// → STEPPED · HOST (nothing written) · WRONG.
pub fn step(g: usize, w: usize) -> u32 {
    let c = &mut state(g).slabs;
    let p = par();
    // (the row as the host wrote it, a double: a number that is no row in the air — a door with no slab is −1 — is no step, never row 0)
    if !(p[P_ROW] >= 0.0 && p[P_ROW] < c.fly as f64 && p[P_ROW] < CAP as f64) {
        return WRONG;
    }
    let r = p[P_ROW] as usize;
    if c.flags[r] & F_REST != 0 {
        return WRONG;
    }
    let mut d = Leaf {
        w,
        height: p[P_HEIGHT],
        thick: p[P_THICK],
        hu: p[P_HU],
        closed: p[P_CLOSED],
        fcx: p[P_FCX],
        fcz: p[P_FCZ],
        but: p[P_BUT] as i32,
        ask: p[P_ASK] != 0.0,
        host: false,
        rays: 0.0,
        gathers: 0.0,
        nudged: 0.0,
        nudge_max: 0.0,
        laid: 0.0,
    };
    let (k_scrape, k_slam, k_slide, k_bounce, k_tip) = (p[P_SCRAPE], p[P_SLAM], p[P_SLIDE], p[P_BOUNCE], p[P_TIP]);
    let mut s = Now {
        px: c.px[r],
        py: c.py[r],
        pz: c.pz[r],
        vx: c.vx[r],
        vy: c.vy[r],
        vz: c.vz[r],
        a: c.a[r],
        spin: c.spin[r],
        tip: c.tip[r],
        tip_v: c.tip_v[r],
        t: c.t[r],
        wedged: c.wedged[r],
        rest: false,
        landed: c.flags[r] & F_LANDED != 0,
    };
    let dt = p[P_DT];
    let n = js_max(1.0, js_ceil(dt * 120.0));
    let h = dt / n;
    let (mut landed, mut land_x, mut land_z, mut substeps) = (0.0, 0.0, 0.0, 0.0);
    let mut i = 0.0;
    while i < n && !s.rest {
        i += 1.0;
        substeps += 1.0;
        s.t += h;
        let flat = s.tip >= HALF_PI;
        if !flat && (s.tip_v > 0.0 || s.tip > 0.0) {
            s.tip_v += (1.5 * G / d.height) * sin(js_max(s.tip, 0.05)) * h;
            s.tip = js_min(HALF_PI, s.tip + s.tip_v * h);
        }
        // up and down: ballistic, never below what its tip stands it at
        let sup = support(&d, s.tip);
        s.vy -= G * h;
        s.py += s.vy * h;
        let down = s.py <= sup;
        if down {
            s.py = sup;
            if s.vy < 0.0 {
                s.vy = 0.0;
            }
        }
        // along the floor
        let mut sp = hypot2(s.vx, s.vz);
        if sp > 1e-5 {
            let ns = if down { js_max(0.0, sp - (if flat { k_slide } else { k_scrape }) * h) } else { sp };
            s.vx *= ns / sp;
            s.vz *= ns / sp;
            sp = ns;
        }
        s.a += s.spin * h;
        if down {
            s.spin *= exp(-(if flat { 14.0 } else { 1.5 }) * h);
        }
        let mut hit_wall = false;
        if sp > 1e-5 {
            let (ex, ez, len) = (s.vx / sp, s.vz / sp, sp * h);
            let he = reach_along(&d, &s, ex, ez);
            let (mut adv, mut nn): (f64, Option<(f64, f64)>) = (len, None);
            for y in [0.2, clamp(s.py, 0.4, 1.3)] {
                if let Some((t, n0, n2)) = hit_at(&mut d, s.px, y, s.pz, ex, ez, len + he + 0.02) {
                    if t - he - 0.01 < adv {
                        adv = js_max(0.0, t - he - 0.01);
                        nn = Some((n0, n2));
                    }
                }
            }
            if d.host {
                break; // (a ray ended on a pane: the step is the host's, and nothing of this one is kept)
            }
            s.px += ex * adv;
            s.pz += ez * adv;
            if let Some((n0, n2)) = nn {
                let nl = or_one(hypot2(n0, n2));
                let (wx, wz) = (n0 / nl, n2 / nl);
                let vn = s.vx * wx + s.vz * wz;
                if vn < 0.0 {
                    s.vx -= (1.0 + k_bounce) * vn * wx;
                    s.vz -= (1.0 + k_bounce) * vn * wz;
                }
                s.spin *= 0.3;
                hit_wall = true;
            }
        }
        if s.tip_v == 0.0 && s.tip == 0.0 && (hit_wall || sp < 0.4) {
            s.tip_v = k_tip; // (no room ahead: flat to the far wall first — now it comes back down)
        }
        push_out(&mut d, &mut s);
        if d.host {
            break;
        }
        if s.t > 0.15 {
            unstick(&mut d, &mut s);
        }
        if (s.tip >= HALF_PI || s.t > 6.0) && !s.landed {
            // flat: the slam (the six-second cap lands it too)
            s.tip = HALF_PI;
            s.landed = true;
            s.py = support(&d, HALF_PI);
            settle(&mut d, &mut s);
            s.vx *= k_slam;
            s.vz *= k_slam;
            if s.vy > 0.0 {
                s.vy = 0.0;
            }
            landed = 1.0;
            land_x = s.px;
            land_z = s.pz;
        }
        if s.landed && ((hypot2(s.vx, s.vz) < 0.05 && js_abs(s.spin) < 0.08) || s.t > 6.0) {
            s.rest = true;
            s.vx = 0.0;
            s.vy = 0.0;
            s.vz = 0.0;
            s.spin = 0.0;
            s.py = support(&d, HALF_PI);
            settle(&mut d, &mut s);
        }
    }
    p[P_RAYS] = d.rays;
    p[P_GATHERS] = d.gathers;
    p[P_SUBSTEPS] = substeps;
    if d.host {
        return HOST;
    }
    c.px[r] = s.px;
    c.py[r] = s.py;
    c.pz[r] = s.pz;
    c.vx[r] = s.vx;
    c.vy[r] = s.vy;
    c.vz[r] = s.vz;
    c.a[r] = s.a;
    c.spin[r] = s.spin;
    c.tip[r] = s.tip;
    c.tip_v[r] = s.tip_v;
    c.t[r] = s.t;
    c.wedged[r] = s.wedged;
    c.flags[r] = (c.flags[r] & !(F_REST | F_LANDED)) | if s.rest { F_REST } else { 0 } | if s.landed { F_LANDED } else { 0 };
    p[P_LANDED] = landed;
    p[P_LAND_X] = land_x;
    p[P_LAND_Z] = land_z;
    p[P_RESTED] = if s.rest { 1.0 } else { 0.0 };
    p[P_NUDGED] = d.nudged;
    p[P_NUDGE_MAX] = d.nudge_max;
    p[P_LAID] = d.laid;
    STEPPED
}
