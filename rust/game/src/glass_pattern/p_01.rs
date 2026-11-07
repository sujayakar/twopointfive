//! src/game/glassPattern.ts lines 1 to 333
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// the tag of an edge made by a crack (the TypeScript's `0`)
const CRACK: Tag = Tag::Num(0.0);

/// The one hash: (seed, a, b) → 24 bits; WGSL repeats it.   (glassPattern.ts gpHash)
pub fn gp_hash(seed: f64, a: f64, b: f64) -> f64 {
    let mut h = (js::to_i32(seed)
        ^ m::imul(js::to_i32(a + 1.0), 0x9E3779B1u32 as i32)
        ^ m::imul(js::to_i32(b + 1.0), 0x85EBCA77u32 as i32)) as u32;
    h = m::imul((h ^ (h >> 15)) as i32, 0x2C1B3C6D) as u32;
    h = m::imul((h ^ (h >> 12)) as i32, 0x297A2D39) as u32;
    ((h ^ (h >> 15)) >> 8) as f64
}

/// The hash as a share of one.   (glassPattern.ts gpJit)
pub fn gp_jit(seed: f64, a: f64, b: f64) -> f64 {
    gp_hash(seed, a, b) / 16777216.0
}

/// k into 0‥n.   (glassPattern.ts wrap)
pub fn wrap(k: f64, n: f64) -> f64 {
    js::rem(js::rem(k, n) + n, n)
}

/// Where radial k sits, in sectors (k may be −1 ‥ n: the fan wraps).   (glassPattern.ts gpBound)
pub fn gp_bound(s: crate::glass_pattern::GpSite, k: f64) -> f64 {
    k + GP_ANG_JIT * (gp_jit(s.seed, wrap(k, s.n), 7.0) - 0.5)
}

/// Radial k's bearing.   (glassPattern.ts gpAngle)
pub fn gp_angle(s: crate::glass_pattern::GpSite, k: f64) -> f64 {
    s.rot + 2.0 * m::PI * gp_bound(s, k) / s.n
}

/// The radius at which chord b (0‥3) meets radial k.   (glassPattern.ts gpRho)
pub fn gp_rho(s: crate::glass_pattern::GpSite, k: f64, b: f64) -> f64 {
    s.rho[b as usize] * (GP_RHO_LO + GP_RHO_SPAN * gp_jit(s.seed, wrap(k, s.n), b))
}

/// Where chord b meets radial k.   (glassPattern.ts gpChordPt)
pub fn gp_chord_pt(s: crate::glass_pattern::GpSite, k: f64, b: f64) -> [f64; 2] {
    let a = gp_angle(s, k);
    let r = gp_rho(s, k, b);
    [s.u + r * m::cos(a), s.v + r * m::sin(a)]
}

/// How far radial k is drawn into the uncut sheet before the pane runs (m from the site).   (glassPattern.ts gpReach)
pub fn gp_reach(s: crate::glass_pattern::GpSite, k: f64) -> f64 {
    gp_rho(s, k, 2.0) * (1.25 + 2.0 * gp_jit(s.seed, wrap(k, s.n), 9.0)) * m::min(1.6, 0.6 + 0.4 * s.e)
}

/// Sector s_2 has no middle chord: its two ring pieces are one (band 1).   (glassPattern.ts gpMerged)
pub fn gp_merged(s: crate::glass_pattern::GpSite, s_2: f64) -> bool {
    gp_jit(s.seed, s_2, 21.0) < s.merge
}

/// Where sector s_2's branch crack sits, in sectors.   (glassPattern.ts gpSplit)
pub fn gp_split(s: crate::glass_pattern::GpSite, s_2: f64) -> f64 {
    let b0 = gp_bound(s, s_2);
    let b1 = gp_bound(s, s_2 + 1.0);
    b0 + (b1 - b0) * (GP_SPLIT_LO + GP_SPLIT_SPAN * gp_jit(s.seed, s_2, 23.0))
}

/// The snap across half h of sector s_2: a line through P, unit normal N away from the site; None = the dagger runs unbroken.   (glassPattern.ts gpCut)
pub fn gp_cut(s: crate::glass_pattern::GpSite, s_2: f64, h: f64) -> Option<crate::glass_pattern::GpCut> {
    if gp_jit(s.seed, s_2, 27.0 + h) < GP_CUT_NONE {
        return None;
    }
    let sp = gp_split(s, s_2);
    let mid = if h == 0.0 { (gp_bound(s, s_2) + sp) / 2.0 } else { (sp + gp_bound(s, s_2 + 1.0)) / 2.0 };
    let am = s.rot + 2.0 * m::PI * mid / s.n;
    let rc = s.rho[3] * (GP_CUT_LO + GP_CUT_SPAN * gp_jit(s.seed, s_2, 25.0 + h));
    let an = am + (gp_jit(s.seed, s_2, 29.0 + h) - 0.5) * GP_CUT_TILT;
    Some(GpCut { px: s.u + rc * m::cos(am), py: s.v + rc * m::sin(am), nx: m::cos(an), ny: m::sin(an) })
}

/// Whose territory (u, v) is in: the least power distance (−1 = no sites).   (glassPattern.ts gpNearest)
pub fn gp_nearest(sites: &[crate::glass_pattern::GpSite], u: f64, v: f64) -> f64 {
    let (mut best, mut bd) = (-1.0, m::INF);
    for i in 0..sites.len() {
        let s = sites[i];
        let d = m::pow(s.u - u, 2.0) + m::pow(s.v - v, 2.0) - s.rz * s.rz;
        if d < bd {
            bd = d;
            best = i as f64;
        }
    }
    best
}

/// Past the chord a → b (the site is to its left).   (glassPattern.ts beyond)
pub fn beyond(a: [f64; 2], b: [f64; 2], u: f64, v: f64) -> bool {
    (b[0] - a[0]) * (v - a[1]) - (b[1] - a[1]) * (u - a[0]) < 0.0
}

/// The sector of (u, v) round the site and its position across the fan.   (glassPattern.ts gpSector)
pub fn gp_sector(s: crate::glass_pattern::GpSite, u: f64, v: f64) -> crate::glass_pattern::GpSector {
    let n = s.n;
    let mut t = (m::atan2(v - s.v, u - s.u) - s.rot) / (2.0 * m::PI);
    t = (t - m::floor(t)) * n;
    let s0 = m::min(n - 1.0, m::floor(t));
    if t < gp_bound(s, s0) {
        return if s0 == 0.0 { GpSector { s: n - 1.0, t: t + n } } else { GpSector { s: s0 - 1.0, t } };
    }
    if t >= gp_bound(s, s0 + 1.0) {
        return if s0 == n - 1.0 { GpSector { s: 0.0, t: t - n } } else { GpSector { s: s0 + 1.0, t } };
    }
    GpSector { s: s0, t }
}

/// The sector and band of (u, v) round the site.   (glassPattern.ts gpSectorBand)
pub fn gp_sector_band(s: crate::glass_pattern::GpSite, u: f64, v: f64) -> crate::glass_pattern::GpSectorBand {
    let GpSector { s: sec, t } = gp_sector(s, u, v);
    if !beyond(gp_chord_pt(s, sec, 0.0), gp_chord_pt(s, sec + 1.0, 0.0), u, v) {
        return GpSectorBand { s: sec, b: 0.0 };
    }
    let merged = gp_merged(s, sec);
    if !merged && !beyond(gp_chord_pt(s, sec, 1.0), gp_chord_pt(s, sec + 1.0, 1.0), u, v) {
        return GpSectorBand { s: sec, b: 1.0 };
    }
    if !beyond(gp_chord_pt(s, sec, 2.0), gp_chord_pt(s, sec + 1.0, 2.0), u, v) {
        return GpSectorBand { s: sec, b: if merged { 1.0 } else { 2.0 } };
    }
    let h = if t >= gp_split(s, sec) { 1.0 } else { 0.0 };
    let far = match gp_cut(s, sec, h) {
        Some(c) => c.nx * (u - c.px) + c.ny * (v - c.py) > 0.0,
        None => false,
    };
    GpSectorBand { s: sec, b: (if far { 5.0 } else { 3.0 }) + h }
}

/// A piece's number.   (glassPattern.ts gpFid)
pub fn gp_fid(i: f64, s: f64, b: f64) -> f64 {
    i * GP_FRAGS_PER_SITE + b * GP_MAX_SECTORS + s
}

/// A piece's site, band and sector.   (glassPattern.ts gpUnfid)
pub fn gp_unfid(fid: f64) -> crate::glass_pattern::GpUnfid {
    GpUnfid {
        i: m::floor(fid / GP_FRAGS_PER_SITE),
        b: m::floor(js::rem(fid, GP_FRAGS_PER_SITE) / GP_MAX_SECTORS),
        s: js::rem(fid, GP_MAX_SECTORS),
    }
}

/// The piece (u, v) is in: fid, GP_REMAINDER (the uncut sheet), or −1 (no sites).   (glassPattern.ts gpClassify)
pub fn gp_classify(sites: &[crate::glass_pattern::GpSite], run: bool, u: f64, v: f64) -> f64 {
    let i = gp_nearest(sites, u, v);
    if i < 0.0 {
        return -1.0;
    }
    let GpSectorBand { s, b } = gp_sector_band(sites[i as usize], u, v);
    if b >= 3.0 && !run {
        return GP_REMAINDER;
    }
    gp_fid(i, s, b)
}

/// Sutherland–Hodgman against one half-plane n·p ≤ d; tags[k] names what made the edge pts[k] → pts[k+1].   (glassPattern.ts clip)
pub fn clip(pts: &[[f64; 2]], tags: &[crate::glass_pattern::Tag], nx: f64, ny: f64, d: f64) -> crate::glass_pattern::Clip {
    if pts.len() < 3 {
        return Clip { pts: Vec::new(), tags: Vec::new() };
    }
    let inside = |p: [f64; 2]| nx * p[0] + ny * p[1] <= d + 1e-12;
    let mut np: Vec<[f64; 2]> = Vec::new();
    let mut n_in: Vec<Tag> = Vec::new(); // n_in[k] = tag of the edge ARRIVING at np[k]
    for k in 0..pts.len() {
        let (a, b, t) = (pts[k], pts[(k + 1) % pts.len()], tags[k]);
        let (ain, bin) = (inside(a), inside(b));
        if ain && bin {
            np.push(b);
            n_in.push(t);
        } else if ain && !bin {
            np.push(isect(a, b, nx, ny, d));
            n_in.push(t);
        } else if !ain && bin {
            np.push(isect(a, b, nx, ny, d));
            n_in.push(CRACK);
            np.push(b);
            n_in.push(t);
        }
    }
    let tags = (0..n_in.len()).map(|k| n_in[(k + 1) % n_in.len()]).collect();
    Clip { pts: np, tags }
}

/// Where the edge a → b crosses the line n·p = d.   (glassPattern.ts isect)
pub fn isect(a: [f64; 2], b: [f64; 2], nx: f64, ny: f64, d: f64) -> [f64; 2] {
    let da = nx * a[0] + ny * a[1] - d;
    let db = nx * b[0] + ny * b[1] - d;
    let t = da / (da - db);
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]
}

/// The whole pane as a polygon.   (glassPattern.ts rectPoly)
pub fn rect_poly(hu: f64, hv: f64) -> crate::glass_pattern::Poly {
    Poly {
        pts: vec![[-hu, -hv], [hu, -hv], [hu, hv], [-hu, hv]],
        tags: vec![Tag::Bottom, Tag::Right, Tag::Top, Tag::Left],
    }
}

/// `clip` of a Poly, as a Poly.
fn clip_poly(p: &Poly, nx: f64, ny: f64, d: f64) -> Poly {
    let c = clip(&p.pts, &p.tags, nx, ny, d);
    Poly { pts: c.pts, tags: c.tags }
}

/// Site i's territory: its power cell clipped to the pane.   (glassPattern.ts gpRegion)
pub fn gp_region(sites: &[crate::glass_pattern::GpSite], i: f64, hu: f64, hv: f64) -> crate::glass_pattern::Poly {
    let mut p = rect_poly(hu, hv);
    let i = i as usize;
    let a = sites[i];
    let mut j = 0;
    while j < sites.len() && p.pts.len() >= 3 {
        if j == i {
            j += 1;
            continue;
        }
        let b = sites[j];
        p = clip_poly(&p, b.u - a.u, b.v - a.v, (b.u * b.u + b.v * b.v - a.u * a.u - a.v * a.v + a.rz * a.rz - b.rz * b.rz) / 2.0);
        j += 1;
    }
    p
}

/// gpFragments' `add`: measure a piece and ask who lies across each of its cracks; no area, no piece.
fn add(out: &mut Vec<GpFrag>, sites: &[GpSite], run: bool, i: f64, s: f64, b: f64, poly: Poly) {
    let n = poly.pts.len();
    if n < 3 {
        return;
    }
    let (mut area, mut cx, mut cy, mut perim) = (0.0, 0.0, 0.0, 0.0);
    let (mut u_min, mut u_max, mut v_min, mut v_max) = (m::INF, -m::INF, m::INF, -m::INF);
    let fid = gp_fid(i, s, b);
    let mut frame = GpFragFrame { bottom: 0.0, right: 0.0, top: 0.0, left: 0.0 };
    let mut nb: Vec<GpNb> = Vec::new();
    let mut foot: Option<[f64; 2]> = None;
    for k in 0..n {
        let (p, q) = (poly.pts[k], poly.pts[(k + 1) % n]);
        let cr = p[0] * q[1] - q[0] * p[1];
        area += cr;
        cx += (p[0] + q[0]) * cr;
        cy += (p[1] + q[1]) * cr;
    }
    area /= 2.0;
    if area < 1e-7 {
        return;
    }
    for k in 0..n {
        let (p, q) = (poly.pts[k], poly.pts[(k + 1) % n]);
        let (ex, ey) = (q[0] - p[0], q[1] - p[1]);
        let len = m::hyp2(ex, ey);
        perim += len;
        u_min = m::min(u_min, p[0]);
        u_max = m::max(u_max, p[0]);
        v_min = m::min(v_min, p[1]);
        v_max = m::max(v_max, p[1]);
        if len < 1e-7 {
            continue;
        }
        match poly.tags[k] {
            Tag::Num(_) => {}
            Tag::Right => { frame.right += len; continue; }
            Tag::Top => { frame.top += len; continue; }
            Tag::Left => { frame.left += len; continue; }
            Tag::Bottom => {
                frame.bottom += len;
                foot = Some(match foot {
                    Some(f) => [m::min_n(&[f[0], p[0], q[0]]), m::max_n(&[f[1], p[0], q[0]])],
                    None => [m::min(p[0], q[0]), m::max(p[0], q[0])],
                });
                continue;
            }
        }
        let (nx, ny) = (ey / len, -ex / len); // (anticlockwise polygon: the outward normal of p → q)
        let samples = NB_SAMPLES;
        for j in 0..NB_SAMPLES as usize {
            let (a, c) = (j as f64 / samples, (j + 1) as f64 / samples);
            let mid = (a + c) / 2.0;
            let f = gp_classify(sites, run, p[0] + ex * mid + nx * 1e-4, p[1] + ey * mid + ny * 1e-4);
            if f == -1.0 || f == fid {
                continue;
            }
            let (ua, ub) = (p[0] + ex * a, p[0] + ex * c);
            match nb.last_mut() {
                Some(last) if last.fid == f && last.ny == ny => {
                    last.len += len / samples;
                    last.u0 = m::min_n(&[last.u0, ua, ub]);
                    last.u1 = m::max_n(&[last.u1, ua, ub]);
                }
                _ => nb.push(GpNb { fid: f, len: len / samples, ny, u0: m::min(ua, ub), u1: m::max(ua, ub) }),
            }
        }
    }
    out.push(GpFrag {
        fid,
        i,
        s,
        b,
        pts: poly.pts,
        area,
        cen: [cx / (6.0 * area), cy / (6.0 * area)],
        perim,
        u_min,
        u_max,
        v_min,
        v_max,
        frame,
        foot,
        nb,
    });
}

/// gpFragments' `half`: far = the side away from the site of chord a → b.
fn half(p: &Poly, a: [f64; 2], b: [f64; 2], far: bool) -> Poly {
    let (ex, ey) = (b[0] - a[0], b[1] - a[1]);
    if far {
        clip_poly(p, -ey, ex, -ey * a[0] + ex * a[1])
    } else {
        clip_poly(p, ey, -ex, ey * a[0] - ex * a[1])
    }
}

/// gpFragments' `ray`: left = anticlockwise of the ray.
fn ray(p: &Poly, s: GpSite, ang: f64, left: bool) -> Poly {
    let (dx, dy) = (m::cos(ang), m::sin(ang));
    if left {
        clip_poly(p, dy, -dx, dy * s.u - dx * s.v)
    } else {
        clip_poly(p, -dy, dx, -dy * s.u + dx * s.v)
    }
}

/// Every piece the sites cut the pane into, with who is across each crack, made symmetric.   (glassPattern.ts gpFragments)
pub fn gp_fragments(sites: &[crate::glass_pattern::GpSite], run: bool, hu: f64, hv: f64) -> Vec<crate::glass_pattern::GpFrag> {
    let mut out: Vec<GpFrag> = Vec::new();
    for ii in 0..sites.len() {
        let i = ii as f64;
        let site = sites[ii];
        let region = if run { Some(gp_region(sites, i, hu, hv)) } else { None };
        let mut si = 0;
        while (si as f64) < site.n {
            let s = si as f64;
            si += 1;
            let (a0, a1) = (gp_angle(site, s), gp_angle(site, s + 1.0));
            let c = [
                [gp_chord_pt(site, s, 0.0), gp_chord_pt(site, s + 1.0, 0.0)],
                [gp_chord_pt(site, s, 1.0), gp_chord_pt(site, s + 1.0, 1.0)],
                [gp_chord_pt(site, s, 2.0), gp_chord_pt(site, s + 1.0, 2.0)],
            ];
            let wedge = ray(&ray(&rect_poly(hu, hv), site, a0, true), site, a1, false);
            let merged = gp_merged(site, s);
            add(&mut out, sites, run, i, s, 0.0, half(&wedge, c[0][0], c[0][1], false));
            let ring = if merged { 2 } else { 1 };
            add(&mut out, sites, run, i, s, 1.0, half(&half(&wedge, c[0][0], c[0][1], true), c[ring][0], c[ring][1], false));
            if !merged {
                add(&mut out, sites, run, i, s, 2.0, half(&half(&wedge, c[1][0], c[1][1], true), c[2][0], c[2][1], false));
            }
            let Some(region) = region.as_ref() else { continue };
            let split_at = site.rot + 2.0 * m::PI * gp_split(site, s) / site.n;
            for hi in 0..2 {
                let h = hi as f64;
                let mut p = if hi == 0 {
                    ray(&ray(region, site, a0, true), site, split_at, false)
                } else {
                    ray(&ray(region, site, split_at, true), site, a1, false)
                };
                p = half(&p, c[2][0], c[2][1], true);
                let Some(cut) = gp_cut(site, s, h) else {
                    add(&mut out, sites, run, i, s, 3.0 + h, p);
                    continue;
                };
                let d = cut.nx * cut.px + cut.ny * cut.py;
                add(&mut out, sites, run, i, s, 3.0 + h, clip_poly(&p, cut.nx, cut.ny, d));
                add(&mut out, sites, run, i, s, 5.0 + h, clip_poly(&p, -cut.nx, -cut.ny, -d));
            }
        }
    }
    // symmetric: what A says of B, B says of A (the longer of the two measures of their shared crack)
    let mut by: JsMap<i32, usize> = JsMap::new();
    for fi in 0..out.len() {
        by.set(out[fi].fid as i32, fi);
    }
    for fi in 0..out.len() {
        let f_fid = out[fi].fid;
        // A live index in place of the TypeScript's walk of a copy: this walk only ever removes the entry it stands on, and a piece is never its own
        // neighbour, so nothing else changes this list under it.
        let mut k = 0;
        while k < out[fi].nb.len() {
            let n = out[fi].nb[k];
            if n.fid < 0.0 {
                k += 1;
                continue;
            }
            let Some(&oi) = by.get(n.fid as i32) else {
                out[fi].nb.remove(k); // (a sliver too thin to be a piece is nobody's neighbour)
                continue;
            };
            let back = out[oi].nb.iter().position(|e| e.fid == f_fid && m::abs(e.ny + n.ny) < 0.05);
            match back {
                None => out[oi].nb.push(GpNb { fid: f_fid, len: n.len, ny: -n.ny, u0: n.u0, u1: n.u1 }),
                Some(bi) => {
                    let bk = out[oi].nb[bi];
                    let (len, u0, u1) = (m::max(bk.len, n.len), m::min(bk.u0, n.u0), m::max(bk.u1, n.u1));
                    let e = &mut out[oi].nb[bi];
                    e.len = len;
                    e.u0 = u0;
                    e.u1 = u1;
                    let e = &mut out[fi].nb[k];
                    e.len = len;
                    e.u0 = u0;
                    e.u1 = u1;
                }
            }
            k += 1;
        }
    }
    out
}

/// Distance from (u, v) to the line a → b.   (glassPattern.ts lineDist)
pub fn line_dist(a: [f64; 2], b: [f64; 2], u: f64, v: f64) -> f64 {
    let (ex, ey) = (b[0] - a[0], b[1] - a[1]);
    m::abs(ex * (v - a[1]) - ey * (u - a[0])) / m::max(1e-6, m::hyp2(ex, ey))
}

/// gpCrackAt's `take`: keep the nearest line that has any weight.
fn take(d: &mut f64, w: &mut f64, dd: f64, ww: f64) {
    if ww > 0.0 && dd < *d {
        *d = dd;
        *w = ww;
    }
}

/// Nearest drawn crack at (u, v), annealed or laminated (`web`).   (glassPattern.ts gpCrackAt)
pub fn gp_crack_at(sites: &[crate::glass_pattern::GpSite], run: bool, web: bool, u: f64, v: f64) -> crate::glass_pattern::GpCrack {
    let i = gp_nearest(sites, u, v);
    if i < 0.0 {
        return GpCrack { d: m::INF, w: 0.0, fid: -1.0 };
    }
    let site = sites[i as usize];
    let GpSector { s, t } = gp_sector(site, u, v);
    let GpSectorBand { b, .. } = gp_sector_band(site, u, v);
    let (dx, dy) = (u - site.u, v - site.v);
    let full = run && !web;
    let (mut d, mut w) = (m::INF, 0.0);
    for k in [s, s + 1.0] {
        // the radials: perpendicular distance, while the crack has run this far (tapering over its last third)
        let a = gp_angle(site, k);
        let along = dx * m::cos(a) + dy * m::sin(a);
        let perp = m::abs(dx * m::sin(a) - dy * m::cos(a));
        let reach = if full { m::INF } else { gp_reach(site, k) };
        if along <= reach {
            take(&mut d, &mut w, perp, if full { 1.0 } else { m::min(1.0, (reach - along) / (0.33 * reach) + 0.25) });
        } else {
            take(&mut d, &mut w, m::hyp2(along - reach, perp), 0.25);
        }
    }
    let p = [
        [gp_chord_pt(site, s, 0.0), gp_chord_pt(site, s + 1.0, 0.0)],
        [gp_chord_pt(site, s, 1.0), gp_chord_pt(site, s + 1.0, 1.0)],
        [gp_chord_pt(site, s, 2.0), gp_chord_pt(site, s + 1.0, 2.0)],
        [gp_chord_pt(site, s, 3.0), gp_chord_pt(site, s + 1.0, 3.0)],
    ];
    if web {
        let mut wb = 0usize;
        while wb < 4 && beyond(p[wb][0], p[wb][1], u, v) {
            wb += 1;
        }
        let mut cb = wb.saturating_sub(1);
        while cb <= core::cmp::min(2, wb) {
            take(&mut d, &mut w, line_dist(p[cb][0], p[cb][1], u, v), if cb == 0 { 1.0 } else { 0.8 });
            cb += 1;
        }
        if wb >= 3 && gp_jit(site.seed, s, 33.0) < GP_WEB_RING {
            take(&mut d, &mut w, line_dist(p[3][0], p[3][1], u, v), 0.6);
        }
        let wbf = wb as f64;
        if wb >= 1 && wb <= 3 && gp_jit(site.seed, s, 11.0 + wbf) > 0.35 {
            // a sub-chord inside the band
            let f0 = 0.3 + 0.4 * gp_jit(site.seed, s, 15.0 + wbf);
            let f1 = 0.3 + 0.4 * gp_jit(site.seed, wrap(s + 1.0, site.n), 15.0 + wbf);
            let a2 = [p[wb - 1][0][0] + (p[wb][0][0] - p[wb - 1][0][0]) * f0, p[wb - 1][0][1] + (p[wb][0][1] - p[wb - 1][0][1]) * f0];
            let b2 = [p[wb - 1][1][0] + (p[wb][1][0] - p[wb - 1][1][0]) * f1, p[wb - 1][1][1] + (p[wb][1][1] - p[wb - 1][1][1]) * f1];
            take(&mut d, &mut w, line_dist(a2, b2, u, v), 0.55);
        }
    } else {
        let merged = gp_merged(site, s);
        let inner: i32 = if b == 0.0 { -1 } else if b >= 3.0 { 2 } else if b == 2.0 { 1 } else { 0 };
        let outer: i32 = if b == 0.0 { 0 } else if b == 1.0 { if merged { 2 } else { 1 } } else if b == 2.0 { 2 } else { -1 };
        let drawn = |cb: i32| cb >= 0 && (cb < 2 || full || gp_jit(site.seed, s, 31.0) < GP_C2_DRAWN);
        if drawn(inner) {
            let c = inner as usize;
            take(&mut d, &mut w, line_dist(p[c][0], p[c][1], u, v), if inner == 0 { 1.0 } else { 0.8 });
        }
        if drawn(outer) {
            let c = outer as usize;
            take(&mut d, &mut w, line_dist(p[c][0], p[c][1], u, v), if outer == 0 { 1.0 } else { 0.8 });
        }
    }
    if full && b >= 3.0 {
        let split_at = site.rot + 2.0 * m::PI * gp_split(site, s) / site.n;
        take(&mut d, &mut w, m::abs(dx * m::sin(split_at) - dy * m::cos(split_at)), 0.9); // the branch crack
        if let Some(c) = gp_cut(site, s, if t >= gp_split(site, s) { 1.0 } else { 0.0 }) {
            take(&mut d, &mut w, m::abs(c.nx * (u - c.px) + c.ny * (v - c.py)), 0.8); // this half's snap
        }
    }
    if full && sites.len() >= 2 {
        // the territory line toward the next-nearest site
        let (mut j2, mut d2) = (-1i32, m::INF);
        let d1 = dx * dx + dy * dy - site.rz * site.rz;
        for j in 0..sites.len() {
            if j == i as usize {
                continue;
            }
            let q = sites[j];
            let dd = m::pow(q.u - u, 2.0) + m::pow(q.v - v, 2.0) - q.rz * q.rz;
            if dd < d2 {
                d2 = dd;
                j2 = j as i32;
            }
        }
        if j2 >= 0 {
            let q = sites[j2 as usize];
            take(&mut d, &mut w, (d2 - d1) / (2.0 * m::max(1e-6, m::hyp2(q.u - site.u, q.v - site.v))), 0.8);
        }
    }
    GpCrack { d, w, fid: if b >= 3.0 && !run { GP_REMAINDER } else { gp_fid(i, s, b) } }
}

/// `seed ^ k`: a number's bits as the language xors them.
fn xor(seed: f64, k: i32) -> f64 {
    (js::to_i32(seed) ^ k) as f64
}

/// The centre of clump (c_2, r): its lattice cell's centre, jittered.   (glassPattern.ts gpDiceCentre)
pub fn gp_dice_centre(d: crate::glass_pattern::GpDice, c_2: f64, r: f64) -> [f64; 2] {
    let du = 2.0 * d.hu / d.cols;
    let dv = 2.0 * d.hv / d.rows;
    [
        -d.hu + (c_2 + 0.5 + GP_DICE_JIT * (gp_jit(d.seed, c_2, r) - 0.5)) * du,
        -d.hv + (r + 0.5 + GP_DICE_JIT * (gp_jit(xor(d.seed, 0x5bd1e9), c_2, r) - 0.5)) * dv,
    ]
}

/// The clump (u, v) belongs to (index r × cols + c): the nearest jittered centre among the 3 × 3 cells round its own.   (glassPattern.ts gpDiceClump)
pub fn gp_dice_clump(d: crate::glass_pattern::GpDice, u: f64, v: f64) -> f64 {
    let c0 = m::max(0.0, m::min(d.cols - 1.0, m::floor((u + d.hu) / (2.0 * d.hu) * d.cols)));
    let r0 = m::max(0.0, m::min(d.rows - 1.0, m::floor((v + d.hv) / (2.0 * d.hv) * d.rows)));
    let (mut best, mut bd) = (r0 * d.cols + c0, m::INF);
    let mut r = m::max(0.0, r0 - 1.0);
    while r <= m::min(d.rows - 1.0, r0 + 1.0) {
        let mut c = m::max(0.0, c0 - 1.0);
        while c <= m::min(d.cols - 1.0, c0 + 1.0) {
            let p = gp_dice_centre(d, c, r);
            let dd = m::pow(p[0] - u, 2.0) + m::pow(p[1] - v, 2.0);
            if dd < bd {
                bd = dd;
                best = r * d.cols + c;
            }
            c += 1.0;
        }
        r += 1.0;
    }
    best
}

/// The order key of clump (c_2, r): metres from the blow to its centre, shuffled by a hashed share of a cell.   (glassPattern.ts gpDiceKey)
pub fn gp_dice_key(d: crate::glass_pattern::GpDice, c_2: f64, r: f64) -> f64 {
    let p = gp_dice_centre(d, c_2, r);
    let cell = m::min(2.0 * d.hu / d.cols, 2.0 * d.hv / d.rows);
    m::hyp2(p[0] - d.u, p[1] - d.v) + GP_DICE_DELAY * cell * gp_jit(xor(d.seed, 0x3c6ef3), c_2, r)
}

/// Clump (c_2, r)'s outline: the cell of its centre among its neighbours', clipped to the pane.   (glassPattern.ts gpDicePoly)
pub fn gp_dice_poly(d: crate::glass_pattern::GpDice, c_2: f64, r: f64) -> Vec<[f64; 2]> {
    let du = 2.0 * d.hu / d.cols;
    let dv = 2.0 * d.hv / d.rows;
    let a = gp_dice_centre(d, c_2, r);
    let u0 = m::max(-d.hu, -d.hu + (c_2 - 1.0) * du);
    let u1 = m::min(d.hu, -d.hu + (c_2 + 2.0) * du);
    let v0 = m::max(-d.hv, -d.hv + (r - 1.0) * dv);
    let v1 = m::min(d.hv, -d.hv + (r + 2.0) * dv);
    let mut p = Poly { pts: vec![[u0, v0], [u1, v0], [u1, v1], [u0, v1]], tags: vec![CRACK; 4] };
    let mut rr = m::max(0.0, r - 2.0);
    while rr <= m::min(d.rows - 1.0, r + 2.0) {
        let mut cc = m::max(0.0, c_2 - 2.0);
        while cc <= m::min(d.cols - 1.0, c_2 + 2.0) {
            if !(cc == c_2 && rr == r) {
                let b = gp_dice_centre(d, cc, rr);
                p = clip_poly(&p, b[0] - a[0], b[1] - a[1], (b[0] * b[0] + b[1] * b[1] - a[0] * a[0] - a[1] * a[1]) / 2.0);
            }
            cc += 1.0;
        }
        rr += 1.0;
    }
    let arr = &p.pts;
    let mut kept = Vec::new();
    for k in 0..arr.len() {
        let (a, q) = (arr[k], arr[(k + 1) % arr.len()]);
        if m::hyp2(q[0] - a[0], q[1] - a[1]) > 1e-9 {
            kept.push(a);
        }
    }
    kept
}

/// The dice as drawn: distance to the nearest edge of a jittered cellular pattern of cell size `grain` (F2 − F1).   (glassPattern.ts gpDiceEdge)
pub fn gp_dice_edge(u: f64, v: f64, grain: f64, seed: f64) -> f64 {
    let (x, y) = (u / grain, v / grain);
    let (ix, iy) = (m::floor(x), m::floor(y));
    let (mut f1, mut f2) = (m::INF, m::INF);
    for j in -1i32..=1 {
        for i in -1i32..=1 {
            let (cx, cy) = (ix + i as f64, iy + j as f64);
            let (wx, wy) = ((js::to_i32(cx) & 0xffff) as f64, (js::to_i32(cy) & 0xffff) as f64);
            let px = cx + 0.15 + 0.7 * gp_jit(seed, wx, wy);
            let py = cy + 0.15 + 0.7 * gp_jit(xor(seed, 0x5bd1e9), wx, wy);
            let d = m::hyp2(px - x, py - y);
            if d < f1 {
                f2 = f1;
                f1 = d;
            } else if d < f2 {
                f2 = d;
            }
        }
    }
    (f2 - f1) * 0.5 * grain
}

/// gpHull's `cr`: the turn at o.
fn cr(o: [f64; 2], a: [f64; 2], b: [f64; 2]) -> f64 {
    (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
}

/// Convex hull (Andrew's chain), anticlockwise.   (glassPattern.ts gpHull)
pub fn gp_hull(pts: &[[f64; 2]]) -> Vec<[f64; 2]> {
    let mut p = pts.to_vec();
    js::sort_by(&mut p, |a, b| js::or(a[0] - b[0], a[1] - b[1]));
    if p.len() < 3 {
        return p;
    }
    let mut lo: Vec<[f64; 2]> = Vec::new();
    for &q in p.iter() {
        while lo.len() >= 2 && cr(lo[lo.len() - 2], lo[lo.len() - 1], q) <= 0.0 {
            lo.pop();
        }
        lo.push(q);
    }
    let mut up: Vec<[f64; 2]> = Vec::new();
    for &q in p.iter().rev() {
        while up.len() >= 2 && cr(up[up.len() - 2], up[up.len() - 1], q) <= 0.0 {
            up.pop();
        }
        up.push(q);
    }
    lo.pop();
    up.pop();
    lo.extend(up);
    lo
}

/// A polygon clipped to the pane.   (glassPattern.ts gpClipRect)
pub fn gp_clip_rect(pts: &[[f64; 2]], hu: f64, hv: f64) -> Vec<[f64; 2]> {
    let tags = vec![CRACK; pts.len()];
    let mut p = clip(pts, &tags, 0.0, -1.0, hv);
    p = clip(&p.pts, &p.tags, 1.0, 0.0, hu);
    p = clip(&p.pts, &p.tags, 0.0, 1.0, hv);
    p = clip(&p.pts, &p.tags, -1.0, 0.0, hu);
    p.pts
}

/// A polygon's area and centroid.   (glassPattern.ts gpPolyArea)
pub fn gp_poly_area(pts: &[[f64; 2]]) -> crate::glass_pattern::GpPolyArea {
    let (mut area, mut cx, mut cy) = (0.0, 0.0, 0.0);
    for k in 0..pts.len() {
        let (p, q) = (pts[k], pts[(k + 1) % pts.len()]);
        let cr = p[0] * q[1] - q[0] * p[1];
        area += cr;
        cx += (p[0] + q[0]) * cr;
        cy += (p[1] + q[1]) * cr;
    }
    area /= 2.0;
    GpPolyArea {
        area: m::abs(area),
        cen: if m::abs(area) > 1e-12 { [cx / (6.0 * area), cy / (6.0 * area)] } else { pts.first().copied().unwrap_or([0.0, 0.0]) },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// FNV-1a over the doubles' bits, against the same walk of the TypeScript under bun with EXACT.on (any NaN counts as one NaN).
    struct Bits(u64);
    impl Bits {
        fn new() -> Self {
            Bits(0xcbf29ce484222325)
        }
        fn put(&mut self, x: f64) {
            let b = if x != x { 0x7ff8000000000000 } else { x.to_bits() };
            self.0 = (self.0 ^ b).wrapping_mul(0x100000001b3);
        }
        fn pts(&mut self, p: &[[f64; 2]]) {
            self.put(p.len() as f64);
            for q in p {
                self.put(q[0]);
                self.put(q[1]);
            }
        }
    }
    fn site(u: f64, v: f64, rot: f64, n: f64, seed: f64, rho: [f64; 4], rz: f64, e: f64, merge: f64, hits: f64) -> GpSite {
        GpSite { u, v, rot, n, seed, rho, rz, e, merge, hits }
    }
    fn abc() -> [GpSite; 3] {
        [
            site(-0.4, 0.1, 0.7, 7.0, 0x3a5f21 as f64, [0.02, 0.05, 0.11, 0.3], 0.14, 1.3, 0.25, 1.0),
            site(0.5, -0.2, 2.1, 9.0, 0xc0ffee as f64, [0.03, 0.07, 0.16, 0.4], 0.2, 3.0, 0.5, 2.0),
            site(0.1, 0.6, -1.2, 12.0, 0x12 as f64, [0.025, 0.05, 0.09, 0.2], 0.1, 0.5, 0.0, 1.0),
        ]
    }
    fn grid() -> Vec<(f64, f64)> {
        let mut g = Vec::new();
        for j in 0..41 {
            for i in 0..41 {
                g.push((-1.0 + i as f64 * 0.05 + 0.0013, -1.0 + j as f64 * 0.05 + 0.0007));
            }
        }
        g
    }

    #[test]
    fn the_hash_is_the_typescript_s() {
        let mut h = Bits::new();
        for seed in [0, 1, 0x3a5f21, 0xffffff, 0x5bd1e9 ^ 0xc0ffee] {
            let seed = seed as f64;
            for a in (-2..40).step_by(3) {
                for b in (-1..35).step_by(4) {
                    h.put(gp_hash(seed, a as f64, b as f64));
                    h.put(gp_jit(seed, a as f64, b as f64));
                }
            }
        }
        assert_eq!(h.0, 0x6f09d00fe65abe15);
    }

    #[test]
    fn a_site_s_numbers_are_the_typescript_s() {
        let mut h = Bits::new();
        for s in abc() {
            let mut k = -1.0;
            while k <= s.n {
                h.put(gp_bound(s, k));
                h.put(gp_angle(s, k));
                h.put(gp_reach(s, k));
                for b in 0..4 {
                    h.put(gp_rho(s, k, b as f64));
                    let p = gp_chord_pt(s, k, b as f64);
                    h.put(p[0]);
                    h.put(p[1]);
                }
                k += 1.0;
            }
        }
        for s in abc() {
            let mut i = 0.0;
            while i < s.n {
                h.put(if gp_merged(s, i) { 1.0 } else { 0.0 });
                h.put(gp_split(s, i));
                for hh in 0..2 {
                    match gp_cut(s, i, hh as f64) {
                        None => h.put(-99.0),
                        Some(c) => {
                            h.put(c.px);
                            h.put(c.py);
                            h.put(c.nx);
                            h.put(c.ny);
                        }
                    }
                }
                i += 1.0;
            }
        }
        assert_eq!(h.0, 0x7ea09ee8dc7710dc);
    }

    #[test]
    fn sectors_pieces_and_cracks_on_a_grid_are_the_typescript_s() {
        let [a, b, c] = abc();
        let sets: [&[GpSite]; 4] = [&[a], &[a, b], &[a, b, c], &[]];
        let g = grid();
        let mut h = Bits::new();
        for s in [a, b, c] {
            for &(u, v) in &g {
                let q = gp_sector(s, u, v);
                h.put(q.s);
                h.put(q.t);
                let r = gp_sector_band(s, u, v);
                h.put(r.s);
                h.put(r.b);
            }
        }
        assert_eq!(h.0, 0x936f7ce6491aaf59, "sector");
        let mut h = Bits::new();
        for ss in sets {
            for &(u, v) in &g {
                h.put(gp_nearest(ss, u, v));
                for run in [false, true] {
                    h.put(gp_classify(ss, run, u, v));
                }
            }
        }
        assert_eq!(h.0, 0x180b9fac5023fb95, "classify");
        let mut h = Bits::new();
        for ss in sets {
            for run in [false, true] {
                for web in [false, true] {
                    for &(u, v) in &g {
                        let r = gp_crack_at(ss, run, web, u, v);
                        h.put(r.d);
                        h.put(r.w);
                        h.put(r.fid);
                    }
                }
            }
        }
        assert_eq!(h.0, 0xa8d08ca6c8bc9e49, "crack");
        let mut h = Bits::new();
        let mut f = 0.0;
        while f < 700.0 {
            let q = gp_unfid(f);
            h.put(q.i);
            h.put(q.b);
            h.put(q.s);
            h.put(gp_fid(q.i, q.s, q.b));
            f += 7.0;
        }
        assert_eq!(h.0, 0xead21bdaafb34265, "fid");
    }

    fn frags(ss: &[GpSite], run: bool, hu: f64, hv: f64) -> u64 {
        let mut h = Bits::new();
        let fr = gp_fragments(ss, run, hu, hv);
        h.put(fr.len() as f64);
        for f in &fr {
            for x in [f.fid, f.i, f.s, f.b] {
                h.put(x);
            }
            h.pts(&f.pts);
            for x in [f.area, f.cen[0], f.cen[1], f.perim, f.u_min, f.u_max, f.v_min, f.v_max, f.frame.bottom, f.frame.right, f.frame.top, f.frame.left] {
                h.put(x);
            }
            match f.foot {
                Some(q) => {
                    h.put(q[0]);
                    h.put(q[1]);
                }
                None => h.put(-99.0),
            }
            h.put(f.nb.len() as f64);
            for n in &f.nb {
                for x in [n.fid, n.len, n.ny, n.u0, n.u1] {
                    h.put(x);
                }
            }
        }
        h.0
    }

    #[test]
    fn regions_and_pieces_are_the_typescript_s() {
        let [a, b, c] = abc();
        let sets: [&[GpSite]; 3] = [&[a], &[a, b], &[a, b, c]];
        let mut h = Bits::new();
        for ss in sets {
            for i in 0..ss.len() {
                h.pts(&gp_region(ss, i as f64, 1.0, 0.8).pts);
            }
        }
        assert_eq!(h.0, 0xcc002afd3a5cdaaa, "region");
        let want = [
            (0x_f55c4728a9d854a9u64, 0x_31daacacdb1ec991u64),
            (0x_284da900405041cd, 0x_323b27737a4e9254),
            (0x_9b23d63d5b105260, 0x_3376df6dd92ab809),
        ];
        for (ss, (w0, w1)) in sets.into_iter().zip(want) {
            assert_eq!(frags(ss, false, 1.0, 0.8), w0, "not run, {} sites", ss.len());
            assert_eq!(frags(ss, true, 1.0, 0.8), w1, "run, {} sites", ss.len());
        }
        assert_eq!(frags(&[a, b, c], true, 0.45, 0.3), 0x386c89b4e4cd6b0f, "sites past the frame");
    }

    /// 400 two-site panes, which between them hit line 213's sliver (a neighbour too thin to be a piece, 24 times under bun); the abc() sets never do.
    #[test]
    fn slivers_are_nobody_s_neighbour() {
        let mut h = Bits::new();
        for k in 0u32..400 {
            let kf = k as f64;
            let s = [
                site(((k * 37) % 100) as f64 / 100.0 - 0.5, ((k * 71) % 100) as f64 / 100.0 - 0.5, kf * 0.1, (5 + k % 8) as f64, ((k * 2654435) & 0xffffff) as f64, [0.02, 0.05, 0.11, 0.3], 0.14, 1.3, 0.25, 1.0),
                site(((k * 13) % 100) as f64 / 100.0 - 0.5, ((k * 29) % 100) as f64 / 100.0 - 0.5, kf * 0.2, (5 + (k >> 2) % 8) as f64, ((k * 40503) & 0xffffff) as f64, [0.03, 0.07, 0.16, 0.4], 0.2, 3.0, 0.5, 2.0),
            ];
            let d = frags(&s, true, 0.6, 0.4);
            h.put((d >> 32) as f64);
            h.put((d & 0xffffffff) as f64);
        }
        assert_eq!(h.0, 0x5bea16f4aff5b1a5);
    }

    #[test]
    fn the_dice_are_the_typescript_s() {
        let d = GpDice { u: 0.2, v: -0.1, cols: 5.0, rows: 4.0, seed: 0xabcdef as f64, hu: 0.9, hv: 0.7 };
        let mut h = Bits::new();
        for r in 0..4 {
            for c in 0..5 {
                let (c, r) = (c as f64, r as f64);
                let p = gp_dice_centre(d, c, r);
                h.put(p[0]);
                h.put(p[1]);
                h.put(gp_dice_key(d, c, r));
                h.pts(&gp_dice_poly(d, c, r));
            }
        }
        for (u, v) in grid() {
            h.put(gp_dice_clump(d, u, v));
            h.put(gp_dice_edge(u, v, 0.07, 0xabcdef as f64));
            h.put(gp_dice_edge(u * 3000.0, v * 3000.0, 0.07, 0x12 as f64));
        }
        assert_eq!(h.0, 0x8cd45b6082130bc2);
    }

    #[test]
    fn hulls_clips_and_areas_are_the_typescript_s() {
        let cloud: Vec<[f64; 2]> =
            (0..40).map(|k| [gp_jit(9.0, k as f64, 0.0) - 0.5, if k % 5 == 0 { 0.25 } else { gp_jit(9.0, k as f64, 1.0) - 0.5 }]).collect();
        let mut h = Bits::new();
        let hull = gp_hull(&cloud);
        h.pts(&hull);
        h.pts(&gp_hull(&cloud[..2]));
        h.pts(&gp_clip_rect(&hull, 0.3, 0.2));
        h.pts(&gp_clip_rect(&hull, 2.0, 2.0));
        h.pts(&gp_clip_rect(&cloud[..2], 2.0, 2.0));
        for p in [&hull[..], &cloud[..2], &[]] {
            let q = gp_poly_area(p);
            h.put(q.area);
            h.put(q.cen[0]);
            h.put(q.cen[1]);
        }
        assert_eq!(h.0, 0x1600d0573d352fac);
    }
}
