//! THE FRACTURE PATTERN'S POLYGONS (src/game/glassPattern.ts gpFragments, gpHull, gpClipRect, gpPolyArea, gpDicePoly, gpDiceKey): what a pane's sites cut it
//! into, as convex polygons in the pane's own (u along it, v up). glass.rs holds the hashes and which piece a point is in; this file holds what each piece IS.
//! No state: every answer is the sites' and the frame's. common.wgsl keeps the shader's copy of the half glass.rs holds.
//!
//! WHAT IS KEPT. Nothing of a game. A pane's pieces are worked out when they are asked for and the answer is held here, in `CACHE`, against the very numbers it
//! was made from: a later asker compares those numbers, so no write has to say what changed, a snapshot put back is no case, and two games share nothing but
//! the arithmetic. The neighbours across each crack (what the support rule walks) are worked out only for a support pass: they are the dear half, asked of
//! every crack at six points.
use crate::glass::{classify_row, site_of, GW_FLAGS, GW_N, GW_SEED, G_SITE};
use crate::maths::{cos, hypot2, sin};
use crate::world::{js_max, js_min};
use prim::floor;

pub type Pt = (f64, f64);
/// the most corners one polygon is held with: a frame's four, and one each for the seven other sites, two radials, a chord, a snap (a dice's cell: a clump's
/// twenty-four neighbours at most, of which glassPattern.ts counts twelve as ever standing)
pub const MAX_PTS: usize = 32;
/// the most corners one piece of a site has, by the same count (4 + 7 + 2 + 1 + 1)
pub const PIECE_PTS: usize = 16;
pub const FRAGS_PER_SITE: usize = 84;
pub const MAX_SITES: usize = 8;
pub const MAX_FRAGS: usize = MAX_SITES * FRAGS_PER_SITE;
const PI: f64 = core::f64::consts::PI;
const NB_SAMPLES: usize = 6;
/// what lies past an edge: a crack, or a side of the frame
const CRACK: u8 = 0;
const BOTTOM: u8 = 1;
const RIGHT: u8 = 2;
const TOP: u8 = 3;
const LEFT: u8 = 4;
/// a piece's edge lengths on the frame, by side
pub const F_BOTTOM: usize = 0;
pub const F_RIGHT: usize = 1;
pub const F_TOP: usize = 2;
pub const F_LEFT: usize = 3;
/// glassPattern.ts GP_REMAINDER: the uncut sheet past chord 2 of a pane that has not run
pub const REMAINDER: i32 = -2;

/// a polygon with what made each edge (edge k runs from corner k to corner k + 1)
#[derive(Clone, Copy)]
pub struct Poly {
    pub n: usize,
    pub p: [Pt; MAX_PTS],
    pub t: [u8; MAX_PTS],
}
const NO_POLY: Poly = Poly { n: 0, p: [(0.0, 0.0); MAX_PTS], t: [CRACK; MAX_PTS] };

#[inline]
fn isect(a: Pt, b: Pt, nx: f64, ny: f64, d: f64) -> Pt {
    let (da, db) = (nx * a.0 + ny * a.1 - d, nx * b.0 + ny * b.1 - d);
    let t = da / (da - db);
    (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t)
}
/// glassPattern.ts clip: Sutherland–Hodgman against the half-plane n·p ≤ d
pub fn clip(g: &Poly, nx: f64, ny: f64, d: f64) -> Poly {
    let mut o = NO_POLY;
    if g.n < 3 {
        return o;
    }
    let mut arrive = [CRACK; MAX_PTS];
    let inside = |p: Pt| nx * p.0 + ny * p.1 <= d + 1e-12;
    let mut m = 0;
    for k in 0..g.n {
        let (a, b, t) = (g.p[k], g.p[(k + 1) % g.n], g.t[k]);
        let (ain, bin) = (inside(a), inside(b));
        if ain && bin {
            o.p[m] = b;
            arrive[m] = t;
            m += 1;
        } else if ain && !bin {
            o.p[m] = isect(a, b, nx, ny, d);
            arrive[m] = t;
            m += 1;
        } else if !ain && bin {
            o.p[m] = isect(a, b, nx, ny, d);
            arrive[m] = CRACK;
            m += 1;
            o.p[m] = b;
            arrive[m] = t;
            m += 1;
        }
    }
    o.n = m;
    for k in 0..m {
        o.t[k] = arrive[(k + 1) % m];
    }
    o
}
#[inline]
fn rect(hu: f64, hv: f64) -> Poly {
    let mut r = NO_POLY;
    r.n = 4;
    r.p[0] = (-hu, -hv);
    r.p[1] = (hu, -hv);
    r.p[2] = (hu, hv);
    r.p[3] = (-hu, hv);
    r.t[0] = BOTTOM;
    r.t[1] = RIGHT;
    r.t[2] = TOP;
    r.t[3] = LEFT;
    r
}
/// the side away from the site of chord A → B, or the side the site is on
#[inline]
fn half(g: &Poly, a: Pt, b: Pt, far: bool) -> Poly {
    let (ex, ey) = (b.0 - a.0, b.1 - a.1);
    if far {
        clip(g, -ey, ex, -ey * a.0 + ex * a.1)
    } else {
        clip(g, ey, -ex, ey * a.0 - ex * a.1)
    }
}
/// anticlockwise of the ray from the site, or clockwise of it
#[inline]
fn ray(g: &Poly, su: f64, sv: f64, ang: f64, left: bool) -> Poly {
    let (dx, dy) = (cos(ang), sin(ang));
    if left {
        clip(g, dy, -dx, dy * su - dx * sv)
    } else {
        clip(g, -dy, dx, -dy * su + dx * sv)
    }
}
#[inline]
fn is_merged(d: &[f64], w: &[u32], i: usize, s: i32) -> bool {
    site_of(d, w, i).merged(s)
}
const S_U: usize = 0;
const S_V: usize = 1;
const S_ROT: usize = 2;
const S_RZ: usize = 7;

/// glassPattern.ts gpRegion: site i's territory, its power cell clipped to the pane
fn region(d: &[f64], ns: usize, i: usize, hu: f64, hv: f64) -> Poly {
    let mut g = rect(hu, hv);
    let a = &d[i * G_SITE..i * G_SITE + G_SITE];
    let mut j = 0;
    while j < ns && g.n >= 3 {
        if j == i {
            j += 1;
            continue;
        }
        let b = &d[j * G_SITE..j * G_SITE + G_SITE];
        g = clip(&g, b[S_U] - a[S_U], b[S_V] - a[S_V], (b[S_U] * b[S_U] + b[S_V] * b[S_V] - a[S_U] * a[S_U] - a[S_V] * a[S_V] + a[S_RZ] * a[S_RZ] - b[S_RZ] * b[S_RZ]) / 2.0);
        j += 1;
    }
    g
}

/// one piece of a pane (glassPattern.ts GpFrag, but for its neighbours, which `neighbours` works out)
#[derive(Clone, Copy)]
pub struct Frag {
    pub fid: u32,
    pub i: u16,
    pub s: u16,
    pub b: u16,
    pub n: u16,
    pub p: [Pt; PIECE_PTS],
    pub t: [u8; PIECE_PTS],
    pub area: f64,
    pub cu: f64,
    pub cv: f64,
    pub perim: f64,
    pub u_min: f64,
    pub u_max: f64,
    pub v_min: f64,
    pub v_max: f64,
    pub frame: [f64; 4],
    /// its edge on the sill: whether it has one, and the u extent of it
    pub has_foot: bool,
    pub foot: (f64, f64),
}
const NO_FRAG: Frag = Frag { fid: 0, i: 0, s: 0, b: 0, n: 0, p: [(0.0, 0.0); PIECE_PTS], t: [CRACK; PIECE_PTS], area: 0.0, cu: 0.0, cv: 0.0, perim: 0.0, u_min: 0.0, u_max: 0.0, v_min: 0.0, v_max: 0.0, frame: [0.0; 4], has_foot: false, foot: (0.0, 0.0) };
impl Frag {
    /// on the frame anywhere (a blast's and the walk's tooth)
    #[inline]
    pub fn frame_sum(&self) -> f64 {
        self.frame[F_BOTTOM] + self.frame[F_LEFT] + self.frame[F_RIGHT] + self.frame[F_TOP]
    }
}

/// what gpFragments' `add` measures of a polygon; None = no area (a ring zone hanging over the frame, a snap past the frame)
fn measure(fid: u32, i: usize, s: i32, b: u32, g: &Poly) -> Option<Frag> {
    if g.n < 3 {
        return None;
    }
    let (mut area, mut cx, mut cy, mut perim) = (0.0, 0.0, 0.0, 0.0);
    let (mut u_min, mut u_max, mut v_min, mut v_max) = (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY);
    for k in 0..g.n {
        let (p, q) = (g.p[k], g.p[(k + 1) % g.n]);
        let cr = p.0 * q.1 - q.0 * p.1;
        area += cr;
        cx += (p.0 + q.0) * cr;
        cy += (p.1 + q.1) * cr;
    }
    area /= 2.0;
    if area < 1e-7 {
        return None;
    }
    let mut f = NO_FRAG;
    for k in 0..g.n {
        let (p, q) = (g.p[k], g.p[(k + 1) % g.n]);
        let (ex, ey) = (q.0 - p.0, q.1 - p.1);
        let len = hypot2(ex, ey);
        perim += len;
        u_min = js_min(u_min, p.0);
        u_max = js_max(u_max, p.0);
        v_min = js_min(v_min, p.1);
        v_max = js_max(v_max, p.1);
        if len < 1e-7 {
            continue;
        }
        let t = g.t[k];
        if t != CRACK {
            f.frame[(t - 1) as usize] += len;
            if t == BOTTOM {
                f.foot = if f.has_foot { (js_min(js_min(f.foot.0, p.0), q.0), js_max(js_max(f.foot.1, p.0), q.0)) } else { (js_min(p.0, q.0), js_max(p.0, q.0)) };
                f.has_foot = true;
            }
        }
    }
    f.fid = fid;
    f.i = i as u16;
    f.s = s as u16;
    f.b = b as u16;
    f.n = g.n as u16;
    for k in 0..g.n {
        f.p[k] = g.p[k];
        f.t[k] = g.t[k];
    }
    f.area = area;
    f.cu = cx / (6.0 * area);
    f.cv = cy / (6.0 * area);
    f.perim = perim;
    f.u_min = u_min;
    f.u_max = u_max;
    f.v_min = v_min;
    f.v_max = v_max;
    Some(f)
}

// ---- one pane's pieces, held against what they were made from
/// panes whose pieces are held at once. More panes coming apart in one tick than this are worked out again as they are met: slower, the same answers.
const CACHED: usize = 8;
/// the doubles of a row a piece's shape is made from (every site's, and the frame), the words (radials and seeds, whether it has run)
pub struct Held {
    used: u32,
    ns: u32,
    run: bool,
    hu: f64,
    hv: f64,
    d: [f64; MAX_SITES * G_SITE],
    w: [u32; 2 * MAX_SITES],
    n: usize,
    frags: [Frag; MAX_FRAGS],
    /// a piece's place in `frags` by its id, plus one (0: no such piece)
    at: [u16; MAX_FRAGS],
}
static mut CACHE: [Held; CACHED] = unsafe { core::mem::zeroed() };
static mut CLOCK: u32 = 0;

/// THE PIECES OF A PANE (gpFragments, bands 0‥2 always and the daggers once it has run), in the order the TypeScript made them: `d` and `w` are its row
pub fn frags_of(d: &[f64], w: &[u32], hu: f64, hv: f64) -> &'static Held {
    let ns = (w[GW_FLAGS] >> 8) as usize;
    let run = w[GW_FLAGS] & crate::glass::G_RUN != 0;
    let cache = unsafe { &mut *core::ptr::addr_of_mut!(CACHE) };
    let clock = unsafe { &mut *core::ptr::addr_of_mut!(CLOCK) };
    *clock = clock.wrapping_add(1);
    let mut oldest = 0;
    for k in 0..CACHED {
        let c = &cache[k];
        if c.used != 0 && c.ns as usize == ns && c.run == run && c.hu == hu && c.hv == hv && same(c, d, w, ns) {
            cache[k].used = *clock;
            return unsafe { &*(core::ptr::addr_of!(cache[k])) };
        }
        if c.used < cache[oldest].used {
            oldest = k;
        }
    }
    let c = &mut cache[oldest];
    c.used = *clock;
    c.ns = ns as u32;
    c.run = run;
    c.hu = hu;
    c.hv = hv;
    c.d[..ns * G_SITE].copy_from_slice(&d[..ns * G_SITE]);
    for i in 0..ns {
        c.w[2 * i] = w[GW_N + i];
        c.w[2 * i + 1] = w[GW_SEED + i];
    }
    make(c, d, w, ns, run, hu, hv);
    unsafe { &*(core::ptr::addr_of!(*c)) }
}
#[inline]
fn same(c: &Held, d: &[f64], w: &[u32], ns: usize) -> bool {
    for i in 0..ns {
        if c.w[2 * i] != w[GW_N + i] || c.w[2 * i + 1] != w[GW_SEED + i] {
            return false;
        }
    }
    for k in 0..ns * G_SITE {
        if c.d[k].to_bits() != d[k].to_bits() {
            return false;
        }
    }
    true
}
impl Held {
    #[inline]
    pub fn list(&self) -> &[Frag] {
        &self.frags[..self.n]
    }
    /// the piece with this id
    #[inline]
    pub fn by(&self, fid: u32) -> Option<&Frag> {
        let k = self.at[fid as usize];
        if k == 0 {
            None
        } else {
            Some(&self.frags[k as usize - 1])
        }
    }
    /// its place in the list, plus one (0: no such piece)
    #[inline]
    pub fn place(&self, fid: u32) -> usize {
        self.at[fid as usize] as usize
    }
}
fn make(c: &mut Held, d: &[f64], w: &[u32], ns: usize, run: bool, hu: f64, hv: f64) {
    c.n = 0;
    c.at = [0; MAX_FRAGS];
    for i in 0..ns {
        let g = site_of(d, w, i);
        let (su, sv, rot, n) = (g.d[S_U], g.d[S_V], g.d[S_ROT], g.n);
        let reg = if run { region(d, ns, i, hu, hv) } else { NO_POLY };
        for s in 0..n {
            let (a0, a1) = (g.angle(s), g.angle(s + 1));
            let c0 = (g.chord(s, 0), g.chord(s + 1, 0));
            let c1 = (g.chord(s, 1), g.chord(s + 1, 1));
            let c2 = (g.chord(s, 2), g.chord(s + 1, 2));
            let wedge = ray(&ray(&rect(hu, hv), su, sv, a0, true), su, sv, a1, false);
            let merged = is_merged(d, w, i, s);
            add(c, i, s, 0, &half(&wedge, c0.0, c0.1, false));
            let outer = if merged { c2 } else { c1 };
            add(c, i, s, 1, &half(&half(&wedge, c0.0, c0.1, true), outer.0, outer.1, false));
            if !merged {
                add(c, i, s, 2, &half(&half(&wedge, c1.0, c1.1, true), c2.0, c2.1, false));
            }
            if !run {
                continue;
            }
            let as_ = rot + 2.0 * PI * g.split(s) / n as f64;
            for h in 0..2u32 {
                let mut p = if h == 0 { ray(&ray(&reg, su, sv, a0, true), su, sv, as_, false) } else { ray(&ray(&reg, su, sv, as_, true), su, sv, a1, false) };
                p = half(&p, c2.0, c2.1, true);
                match g.cut(s, h) {
                    None => add(c, i, s, 3 + h, &p),
                    Some((px, py, nx, ny)) => {
                        let dd = nx * px + ny * py;
                        add(c, i, s, 3 + h, &clip(&p, nx, ny, dd));
                        add(c, i, s, 5 + h, &clip(&p, -nx, -ny, -dd));
                    }
                }
            }
        }
    }
}
#[inline]
fn add(c: &mut Held, i: usize, s: i32, b: u32, g: &Poly) {
    let fid = i as u32 * FRAGS_PER_SITE as u32 + b * 12 + s as u32;
    if let Some(f) = measure(fid, i, s, b, g) {
        c.frags[c.n] = f;
        c.n += 1;
        c.at[fid as usize] = c.n as u16;
    }
}

// ---- what lies across each crack (the second half of gpFragments: asked only by the support rule)
/// the most neighbours the pieces of one pane can list between them: every edge of every piece at six points, and as many made symmetric
const MAX_NB: usize = MAX_FRAGS * PIECE_PTS * NB_SAMPLES * 2;
#[derive(Clone, Copy)]
pub struct Nb {
    pub fid: i32,
    /// the next of this piece's list (+1; 0 = the last), and whether a sliver's entry was taken out of it
    pub next: u32,
    pub dead: bool,
    pub len: f64,
    pub ny: f64,
    pub u0: f64,
    pub u1: f64,
}
static mut NBS: [Nb; MAX_NB] = unsafe { core::mem::zeroed() };
static mut NB_HEAD: [u32; MAX_FRAGS] = [0; MAX_FRAGS];
static mut NB_TAIL: [u32; MAX_FRAGS] = [0; MAX_FRAGS];
/// the neighbours as worked out last: entry `head[k]` − 1 starts piece k's list (k: its place in the pane's list)
pub struct Across {
    pub nbs: &'static [Nb],
    pub head: &'static [u32],
}
/// gpFragments' neighbours for the pieces `h`, in the order the TypeScript's lists hold them
pub fn across(h: &Held, d: &[f64], w: &[u32]) -> Across {
    let nbs = unsafe { &mut *core::ptr::addr_of_mut!(NBS) };
    let head = unsafe { &mut *core::ptr::addr_of_mut!(NB_HEAD) };
    let tail = unsafe { &mut *core::ptr::addr_of_mut!(NB_TAIL) };
    let list = h.list();
    let mut used = 0u32;
    for (k, f) in list.iter().enumerate() {
        head[k] = 0;
        tail[k] = 0;
        let n = f.n as usize;
        for e in 0..n {
            let (p, q) = (f.p[e], f.p[(e + 1) % n]);
            let (ex, ey) = (q.0 - p.0, q.1 - p.1);
            let len = hypot2(ex, ey);
            if len < 1e-7 || f.t[e] != CRACK {
                continue;
            }
            let (nx, ny) = (ey / len, -ex / len);
            let per = len / NB_SAMPLES as f64;
            for j in 0..NB_SAMPLES {
                let (a, c) = (j as f64 / NB_SAMPLES as f64, (j as f64 + 1.0) / NB_SAMPLES as f64);
                let m = (a + c) / 2.0;
                let o = classify_row(d, w, p.0 + ex * m + nx * 1e-4, p.1 + ey * m + ny * 1e-4);
                if o == -1 || o == f.fid as i32 {
                    continue;
                }
                let (ua, ub) = (p.0 + ex * a, p.0 + ex * c);
                let t = tail[k];
                if t != 0 && nbs[t as usize - 1].fid == o && nbs[t as usize - 1].ny == ny {
                    let l = &mut nbs[t as usize - 1];
                    l.len += per;
                    l.u0 = js_min(js_min(l.u0, ua), ub);
                    l.u1 = js_max(js_max(l.u1, ua), ub);
                } else {
                    nbs[used as usize] = Nb { fid: o, next: 0, dead: false, len: per, ny, u0: js_min(ua, ub), u1: js_max(ua, ub) };
                    used += 1;
                    if t != 0 {
                        nbs[t as usize - 1].next = used;
                    } else {
                        head[k] = used;
                    }
                    tail[k] = used;
                }
            }
        }
    }
    // symmetric: what A says of B, B says of A (the longer of the two measures of their shared crack). A piece's list is walked as it stood when its turn
    // came: what a later piece adds to it is not walked again.
    for k in 0..list.len() {
        let last = tail[k];
        let mut e = head[k];
        while e != 0 {
            let at = e as usize - 1;
            let next = nbs[at].next;
            if !nbs[at].dead && nbs[at].fid >= 0 {
                let n = nbs[at];
                match h.at[n.fid as usize] {
                    0 => nbs[at].dead = true, // (a sliver too thin to be a piece, under 0.1 mm², is nobody's neighbour)
                    place => {
                        let o = place as usize - 1;
                        let me = list[k].fid as i32;
                        let mut b = head[o];
                        let mut back = 0;
                        while b != 0 {
                            let m = &nbs[b as usize - 1];
                            if !m.dead && m.fid == me && crate::world::js_abs(m.ny + n.ny) < 0.05 {
                                back = b;
                                break;
                            }
                            b = m.next;
                        }
                        if back == 0 {
                            nbs[used as usize] = Nb { fid: me, next: 0, dead: false, len: n.len, ny: -n.ny, u0: n.u0, u1: n.u1 };
                            used += 1;
                            if tail[o] != 0 {
                                nbs[tail[o] as usize - 1].next = used;
                            } else {
                                head[o] = used;
                            }
                            tail[o] = used;
                        } else {
                            let bk = nbs[back as usize - 1];
                            let (len, u0, u1) = (js_max(bk.len, n.len), js_min(bk.u0, n.u0), js_max(bk.u1, n.u1));
                            let b1 = &mut nbs[back as usize - 1];
                            b1.len = len;
                            b1.u0 = u0;
                            b1.u1 = u1;
                            let n1 = &mut nbs[at];
                            n1.len = len;
                            n1.u0 = u0;
                            n1.u1 = u1;
                        }
                    }
                }
            }
            if e == last {
                break;
            }
            e = next;
        }
    }
    Across { nbs: unsafe { &*core::ptr::addr_of!(NBS) }, head: unsafe { &*core::ptr::addr_of!(NB_HEAD) } }
}

// ---- a crater's outline, and a polygon's measure
/// gpHull: Andrew's chain, anticlockwise (`pts` is sorted here, as the TypeScript sorts its copy). → how many corners in `out`
pub fn hull(pts: &mut [Pt], out: &mut [Pt]) -> usize {
    let n = pts.len();
    // (by u, then v: two corners that compare alike are the same point)
    for i in 1..n {
        let v = pts[i];
        let mut j = i;
        while j > 0 && (pts[j - 1].0 > v.0 || (pts[j - 1].0 == v.0 && pts[j - 1].1 > v.1)) {
            pts[j] = pts[j - 1];
            j -= 1;
        }
        pts[j] = v;
    }
    if n < 3 {
        out[..n].copy_from_slice(pts);
        return n;
    }
    let cr = |o: Pt, a: Pt, b: Pt| (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0);
    let mut lo = [(0.0, 0.0); 32];
    let mut nl = 0;
    for &p in pts.iter() {
        while nl >= 2 && cr(lo[nl - 2], lo[nl - 1], p) <= 0.0 {
            nl -= 1;
        }
        lo[nl] = p;
        nl += 1;
    }
    let mut up = [(0.0, 0.0); 32];
    let mut nu = 0;
    for &p in pts.iter().rev() {
        while nu >= 2 && cr(up[nu - 2], up[nu - 1], p) <= 0.0 {
            nu -= 1;
        }
        up[nu] = p;
        nu += 1;
    }
    nl -= 1;
    nu -= 1;
    out[..nl].copy_from_slice(&lo[..nl]);
    out[nl..nl + nu].copy_from_slice(&up[..nu]);
    nl + nu
}
/// gpClipRect: a polygon with no frame of its own, held to the pane
pub fn clip_rect(pts: &[Pt], hu: f64, hv: f64) -> Poly {
    let mut g = NO_POLY;
    g.n = pts.len();
    g.p[..pts.len()].copy_from_slice(pts);
    g = clip(&g, 0.0, -1.0, hv);
    g = clip(&g, 1.0, 0.0, hu);
    g = clip(&g, 0.0, 1.0, hv);
    clip(&g, -1.0, 0.0, hu)
}
/// gpPolyArea: its area (never below nought) and its centre
pub fn poly_area(p: &[Pt]) -> (f64, f64, f64) {
    let n = p.len();
    let (mut area, mut cx, mut cy) = (0.0, 0.0, 0.0);
    for k in 0..n {
        let (a, b) = (p[k], p[(k + 1) % n]);
        let cr = a.0 * b.1 - b.0 * a.1;
        area += cr;
        cx += (a.0 + b.0) * cr;
        cy += (a.1 + b.1) * cr;
    }
    area /= 2.0;
    let abs = crate::world::js_abs(area);
    if abs > 1e-12 {
        (abs, cx / (6.0 * area), cy / (6.0 * area))
    } else if n > 0 {
        (abs, p[0].0, p[0].1)
    } else {
        (abs, 0.0, 0.0)
    }
}

// ---- toughened glass: the dice (glass.rs dice_centre is the centre of a clump; these are its outline and its place in the order)
/// the dice of a pane coming down: the blow, the lattice, the seed, the half sides
#[derive(Clone, Copy)]
pub struct Dice {
    pub u: f64,
    pub v: f64,
    pub cols: u32,
    pub rows: u32,
    pub seed: u32,
    pub hu: f64,
    pub hv: f64,
}
const DICE_JIT: f64 = 0.6;
const DICE_DELAY: f64 = 1.4;
impl Dice {
    /// gpDiceCentre
    #[inline]
    pub fn centre(&self, c: u32, r: u32) -> Pt {
        let (du, dv) = (2.0 * self.hu / self.cols as f64, 2.0 * self.hv / self.rows as f64);
        (-self.hu + (c as f64 + 0.5 + DICE_JIT * (crate::glass::jit(self.seed, c, r) - 0.5)) * du, -self.hv + (r as f64 + 0.5 + DICE_JIT * (crate::glass::jit(self.seed ^ 0x005b_d1e9, c, r) - 0.5)) * dv)
    }
    /// gpDiceKey: metres from the blow to its centre, shuffled by a hashed share of a cell
    #[inline]
    pub fn key(&self, c: u32, r: u32) -> f64 {
        let p = self.centre(c, r);
        let cell = js_min(2.0 * self.hu / self.cols as f64, 2.0 * self.hv / self.rows as f64);
        hypot2(p.0 - self.u, p.1 - self.v) + DICE_DELAY * cell * crate::glass::jit(self.seed ^ 0x003c_6ef3, c, r)
    }
    /// gpDiceClump: the clump (u, v) belongs to, r × cols + c
    pub fn clump(&self, u: f64, v: f64) -> u32 {
        let (cols, rows) = (self.cols as f64, self.rows as f64);
        let c0 = js_max(0.0, js_min(cols - 1.0, floor((u + self.hu) / (2.0 * self.hu) * cols))) as u32;
        let r0 = js_max(0.0, js_min(rows - 1.0, floor((v + self.hv) / (2.0 * self.hv) * rows))) as u32;
        let (mut best, mut bd) = (r0 * self.cols + c0, f64::INFINITY);
        let r_hi = if r0 + 1 < self.rows { r0 + 1 } else { self.rows - 1 };
        let c_hi = if c0 + 1 < self.cols { c0 + 1 } else { self.cols - 1 };
        let mut r = r0.saturating_sub(1);
        while r <= r_hi {
            let mut c = c0.saturating_sub(1);
            while c <= c_hi {
                let p = self.centre(c, r);
                let (eu, ev) = (p.0 - u, p.1 - v);
                let dd = eu * eu + ev * ev;
                if dd < bd {
                    bd = dd;
                    best = r * self.cols + c;
                }
                c += 1;
            }
            r += 1;
        }
        best
    }
    /// gpDicePoly: clump (c, r)'s outline. → how many corners in `out`
    pub fn poly(&self, c: u32, r: u32, out: &mut [Pt; MAX_PTS]) -> usize {
        let (du, dv) = (2.0 * self.hu / self.cols as f64, 2.0 * self.hv / self.rows as f64);
        let a = self.centre(c, r);
        let (fc, fr) = (c as f64, r as f64);
        let u0 = js_max(-self.hu, -self.hu + (fc - 1.0) * du);
        let u1 = js_min(self.hu, -self.hu + (fc + 2.0) * du);
        let v0 = js_max(-self.hv, -self.hv + (fr - 1.0) * dv);
        let v1 = js_min(self.hv, -self.hv + (fr + 2.0) * dv);
        let mut g = NO_POLY;
        g.n = 4;
        g.p[0] = (u0, v0);
        g.p[1] = (u1, v0);
        g.p[2] = (u1, v1);
        g.p[3] = (u0, v1);
        let rr_hi = if r + 2 < self.rows { r + 2 } else { self.rows - 1 };
        let cc_hi = if c + 2 < self.cols { c + 2 } else { self.cols - 1 };
        let mut rr = r.saturating_sub(2);
        while rr <= rr_hi {
            let mut cc = c.saturating_sub(2);
            while cc <= cc_hi {
                if !(cc == c && rr == r) {
                    let b = self.centre(cc, rr);
                    g = clip(&g, b.0 - a.0, b.1 - a.1, (b.0 * b.0 + b.1 * b.1 - a.0 * a.0 - a.1 * a.1) / 2.0);
                }
                cc += 1;
            }
            rr += 1;
        }
        // (corners that are one point are one corner)
        let mut m = 0;
        for k in 0..g.n {
            let (p, q) = (g.p[k], g.p[(k + 1) % g.n]);
            if hypot2(q.0 - p.0, q.1 - p.1) > 1e-9 {
                out[m] = p;
                m += 1;
            }
        }
        m
    }
}

/// THE MOST CLUMPS A SHEET MAY COME DOWN IN (a sheet of more is refused as its level loads, by name: src/game/levelfit.ts): the order they let go in is worked out
/// whole when it is first asked for
pub const MAX_CLUMPS: usize = 8192;
#[derive(Clone, Copy)]
pub struct Clump {
    pub key: f64,
    pub k: u32,
    pub c: u16,
    pub r: u16,
}
const ORDERS: usize = 4;
struct Order {
    used: u32,
    d: Dice,
    n: usize,
    list: [Clump; MAX_CLUMPS],
}
static mut ORDER: [Order; ORDERS] = unsafe { core::mem::zeroed() };
/// THE CLUMPS OF A PANE COMING DOWN, in the order they let go (startDice's sort: by key, then by number): held against what they were made from, as a pane's pieces are
pub fn order_of(d: &Dice) -> &'static [Clump] {
    let all = unsafe { &mut *core::ptr::addr_of_mut!(ORDER) };
    let clock = unsafe { &mut *core::ptr::addr_of_mut!(CLOCK) };
    *clock = clock.wrapping_add(1);
    let mut oldest = 0;
    for k in 0..ORDERS {
        let o = &all[k];
        if o.used != 0 && o.d.cols == d.cols && o.d.rows == d.rows && o.d.seed == d.seed && o.d.u.to_bits() == d.u.to_bits() && o.d.v.to_bits() == d.v.to_bits() && o.d.hu.to_bits() == d.hu.to_bits() && o.d.hv.to_bits() == d.hv.to_bits() {
            all[k].used = *clock;
            return &all[k].list[..all[k].n];
        }
        if o.used < all[oldest].used {
            oldest = k;
        }
    }
    let o = &mut all[oldest];
    o.used = *clock;
    o.d = *d;
    let n = (d.cols * d.rows) as usize;
    o.n = n;
    for r in 0..d.rows {
        for c in 0..d.cols {
            let k = r * d.cols + c;
            o.list[k as usize] = Clump { key: d.key(c, r), k, c: c as u16, r: r as u16 };
        }
    }
    o.list[..n].sort_unstable_by(|a, b| {
        if a.key < b.key {
            core::cmp::Ordering::Less
        } else if a.key > b.key {
            core::cmp::Ordering::Greater
        } else {
            a.k.cmp(&b.k)
        }
    });
    &o.list[..n]
}
