//! The standing pieces of one wall: the ragged edges round what is open of it, and what its frame leaves hanging there.
use super::*;

/// how far a point of a ragged edge may stand out, by its place along the edge
#[derive(Clone, Copy)]
enum Reach { Flat(f64), Arch { lo: f64, hi: f64 } }
impl Reach { fn at(self, t: f64) -> f64 { match self { Reach::Flat(r) => r, Reach::Arch { lo, hi } => lo + (hi - lo) * m::pow(m::max(0.0, t - 0.3) / 0.7, 1.5) } } }

/// A wall's cells along it, two high: which are open, and what stands past them.
pub(super) struct Cells<'a> {
    b: &'a BreakLike,
    f: Frame,
    pub n: i64,
}

/// Open cells side by side at one level: the first and the last, and the two edges of the opening.
#[derive(Clone, Copy)]
struct Run {
    k: i64,
    k1: i64,
    ua: f64,
    ub: f64,
    w: f64,
}

impl<'a> Cells<'a> {
    pub fn of(f: Frame, b: &'a BreakLike) -> Cells<'a> { Cells { b, f, n: f.n as i64 } }
    pub fn open(&self, k: i64, iy: i64) -> bool {
        let (b, kf) = (self.b, k as f64);
        let (ix, iz) = if self.f.along_x { (kf, 0.0) } else { (0.0, kf) };
        k >= 0 && k < self.n && b.open.get(((iy as f64 * b.nz + iz) * b.nx + ix) as usize) == Some(&1)
    }
    /// Nothing of THIS wall to bury a plate in: open, or past the wall's end.
    fn gone(&self, k: i64, iy: i64) -> bool { k < 0 || k >= self.n || self.open(k, iy) }
    /// The boundary on the low side of column k.
    pub fn edge_u(&self, k: i64) -> f64 { -self.f.l / 2.0 + self.f.w * k as f64 }
    /// Standing wall of this level past column j going `s` (m, a little short of it; 0 = open there, or the wall's end).
    fn past(&self, mut j: i64, s: i64, iy: i64) -> f64 {
        let mut c = 0.0;
        while !self.gone(j, iy) && c < 2.0 { c += 1.0; j += s; }
        c * self.f.w * 0.88
    }
    /// Standing wall behind an edge at level iy, from column ke away from the opening (m, a little short of it).
    fn behind(&self, ke: i64, s: i64, iy: i64) -> f64 { m::min(0.85, self.past(ke, -s, iy)) }
    fn runs(&self, iy: i64) -> Vec<Run> {
        (0..self.n).filter(|&k| self.open(k, iy) && !self.open(k - 1, iy)).map(|k| {
            let mut k1 = k;
            while self.open(k1 + 1, iy) { k1 += 1; }
            let (ua, ub) = (self.edge_u(k), self.edge_u(k1 + 1));
            Run { k, k1, ua, ub, w: ub - ua }
        }).collect()
    }
    /// The sides of a run that this wall stands at: the column there, the edge, and the way into the opening.
    fn sides(&self, r: &Run) -> Vec<(i64, f64, i64)> {
        [(r.k - 1, r.ua, 1i64), (r.k1 + 1, r.ub, -1i64)].into_iter().filter(|s| s.0 >= 0 && s.0 < self.n).collect()
    }
}

/// What lays one wall's pieces: its frame, what it is painted from, and the boxes so far.
struct Lay<'a> {
    w: Cells<'a>, f: Frame, e: crate::knobs::EdgeLookK, p: Painted,
    fc: V3, f_yaw: f64, f_rough: Option<f64>,
    out: Vec<Bx>, key: i32,
    /// the charges' gaps along it
    bl: Option<Vec<[f64; 2]>>,
    /// what it is framed with
    frame_c: V3,
}

/// An opening above the sills: a port, or the upper half of a hole, under its lintel if it has one.
struct Upper {
    run: Run,
    is_hole: bool,
    /// how deep its lintel is (0 = none), and the underside of it
    lin: f64,
    y_l: f64,
}

impl Upper {
    fn has_lintel(&self) -> bool { js::truthy(self.lin) }
}

impl Lay<'_> {
    fn of<'a>(g: &Game, b: &'a BreakLike) -> Lay<'a> {
        let f = edge_look_frame(g, b);
        let p = painted(g, b);
        let x = &g.tab.boxes[b.box_];
        Lay {
            w: Cells::of(f, b), f, e: g.k.edge_look,
            fc: x.c, f_yaw: x.yaw, f_rough: x.rough,
            out: Vec::new(), key: (js::to_i32(b.id) & 0x3fff) << 9,
            bl: g.destr.edge_look.blasted.get(crate::Num(b.id)).cloned(),
            frame_c: if b.surf == "wood" { v3::scale(p.paint, 0.8) } else { steel() },
            p,
        }
    }
    fn top(&self) -> f64 { self.f.y_top - 0.004 }
    fn span(&self) -> f64 { self.f.y_top - self.f.y_sill }
    fn h(&self, a: f64, b2: f64, c: f64) -> f64 { look_hash([self.w.b.id + 1.0, a + 7.0, b2 + 3.0, c + 11.0]) }
    fn push(&mut self, b: Bx) { if (self.out.len() as f64) < self.e.per_wall { self.out.push(b); } }
    /// a charge's gap: what stands round it is sooted, heaviest at the gap
    fn tone(&self, c: V3, u: f64) -> V3 {
        let Some(bl) = &self.bl else { return c };
        let mut x = 1.0;
        for &[uc, half] in bl.iter() { x = m::min(x, m::max(0.0, (m::abs(u - uc) - half * 0.5) / (half * 0.5 + js::or(self.e.scorch.rim, 0.9)))); }
        if x >= 1.0 { c } else { v3::scale(c, 0.4 + 0.6 * x * x * (3.0 - 2.0 * x)) }
    }
    fn next_key(&mut self) -> i32 { let k = self.key; self.key = self.key.wrapping_add(1); k }
    /// a plate in the wall's plane: half extents a × hb × ht, turned `th` in the plane, centred (u, y), v across
    fn plate(&self, u: f64, y: f64, v: f64, a: f64, hb: f64, ht: f64, th: f64, albedo: V3, key: i32, solid: bool) -> Bx {
        let f = &self.f;
        let (c, s) = (m::cos(th), m::sin(th));
        let e1: V3 = [f.along[0] * c, s, f.along[2] * c]; let e2: V3 = [-f.along[0] * s, c, -f.along[2] * s]; let e3 = v3::cross(e1, e2);
        crate::scene_boxes::make_box_value(&crate::scene_boxes::MakeBoxP {
            c: at_c(self.fc, f, u, y, v), h: [a, hb, ht], yaw: Some(self.f_yaw), rot: Some(quat::from_basis(e1, e2, e3)), albedo: Some(albedo),
            flags: Some((crate::scene_boxes::box_flag::DYNAMIC as i32 | (if solid { 0 } else { crate::scene_boxes::box_flag::NO_SHADOW as i32 })) as f64),
            mkey: Some(MKEY_LOOK + (key & 0x7fffff) as f64), rough: self.f_rough, stuff: self.p.stuff, does: self.p.does, ..Default::default() })
    }
    /// A plate of what the wall is built on, which is not the wall: a wall that stands by itself shows none.
    fn frame(&mut self, u: f64, y: f64, v: f64, a: f64, hb: f64, ht: f64, th: f64, albedo: V3, key: i32) {
        if !self.p.inside.framed { return; }
        let mut p = self.plate(u, y, v, a, hb, ht, th, albedo, key, false);
        p.stuff = None; p.does = None;
        self.push(p);
    }
    /// A RAGGED EDGE as one broken line: control points standing d out of the straight cut, each stretch between two of them one plate
    #[allow(clippy::too_many_arguments)]
    fn ragged(&mut self, ax: f64, ay: f64, bx: f64, by: f64, nx: f64, ny: f64, reach: Reach, bury: f64, past_a: f64, past_b: f64, tag: f64) {
        let len = m::hyp2(bx - ax, by - ay); if len < 0.12 { return; }
        let (ex, ey) = ((bx - ax) / len, (by - ay) / len);
        let t_wall = self.f.t;
        for coat in 0..2 {
            let cf = coat as f64; let two = coat == 1;
            let mut ts: Vec<f64> = vec![0.0]; let mut ds: Vec<f64> = Vec::new();
            let (st0, st1) = (if two { 0.2 } else { 0.24 }, if two { 0.32 } else { 0.34 });
            let mut t = 0.0;
            for i in 0..14 { let st = st0 + st1 * self.h(tag, i as f64, 30.0 + cf); if len - t - st < st0 * 0.7 { break; } t += st; ts.push(t); }
            ts.push(len);
            let par = if self.h(tag, 99.0, cf) < 0.5 { 0 } else { 1 };
            for j in 0..ts.len() {
                let r = m::min(reach.at(ts[j] / len), bury * 0.8) * (if two { 1.12 } else { 1.0 }) + (if two { 0.012 } else { 0.0 });
                let k = if (j + par) % 2 == 0 { 0.55 + 0.5 * self.h(tag, j as f64, 40.0 + cf) } else { 0.04 + 0.3 * self.h(tag, j as f64, 50.0 + cf) };
                ds.push(m::max(0.012, r * k));
            }
            // a plate's body leans past the end of its stretch that stands further out: by a hard end that lean has to fit in what is left of the edge
            for _pass in 0..2 { for j in 0..ts.len() - 1 {
                let dt = ts[j + 1] - ts[j]; let up = ds[j + 1] > ds[j]; let room = if up { len - ts[j + 1] + past_b } else { ts[j] + past_a };
                let (dmax, dmin) = (m::max(ds[j], ds[j + 1]), m::min(ds[j], ds[j + 1])); let most = room * dt / (dmax + 0.015);
                if dmax - dmin > most { if up { ds[j + 1] = dmin + most; } else { ds[j] = dmin + most; } }
            } }
            for j in 0..ts.len() - 1 {
                let dt = ts[j + 1] - ts[j]; let dd = ds[j + 1] - ds[j]; let ls = m::hyp2(dt, dd); let dmax = m::max(ds[j], ds[j + 1]);
                let (gx, gy, mx, my) = ((ex * dt + nx * dd) / ls, (ey * dt + ny * dd) / ls, -(nx * dt - ex * dd) / ls, -(ny * dt - ey * dd) / ls);   // along the stretch; into the wall
                let d = (dmax + 0.015) * ls / dt;
                let e0 = if j == 0 && past_a <= 0.0 { 0.0 } else { 0.012 }; let e1 = if j + 2 == ts.len() && past_b <= 0.0 { 0.0 } else { 0.012 };
                let cu = ax + ex * (ts[j] + dt / 2.0) + nx * (ds[j] + dd / 2.0) + gx * (e1 - e0) / 2.0 + mx * d / 2.0;
                let cy = ay + ey * (ts[j] + dt / 2.0) + ny * (ds[j] + dd / 2.0) + gy * (e1 - e0) / 2.0 + my * d / 2.0;
                let col = self.tone(if two { self.p.core } else { self.p.paint }, cu);
                let key = self.next_key();
                let b = self.plate(cu, cy, 0.0, (ls + e0 + e1) / 2.0, d / 2.0, (if two { t_wall * 0.5 } else { t_wall - 0.003 }) - 0.0007 * (j % 3) as f64, m::atan2(gy, gx), col, key, !two);
                self.push(b);
            }
        }
    }

    /// A run of open SILL cells: the lower sides of a hole, and a bent stub of the frame by each.
    fn sill_run(&mut self, run: &Run) {
        let (f, e) = (self.f, self.e);
        for (ke, ue, s) in self.w.sides(run) {
            let (kf, sf) = (ke as f64, s as f64);
            let r = m::min(e.hole_out * 1.5, run.w * e.run_k * 0.6);
            let (bury, past_b) = (self.w.behind(ke, s, 0), if self.w.open(ke, 1) { 0.0 } else { 0.9 });
            self.ragged(ue, f.y0, ue, f.y_sill, sf, 0.0, Reach::Flat(r), bury, 9.0, past_b, kf * 8.0 + (if s > 0 { 3.0 } else { 4.0 }));
            if self.h(kf, 160.0, sf + 2.0) < 0.7 {
                let hgt = 0.16 + 0.3 * self.h(kf, 161.0, sf + 2.0); let th = sf * (0.25 + 0.5 * self.h(kf, 162.0, sf + 2.0)); let at = ue + sf * (0.02 + 0.06 * self.h(kf, 163.0, sf + 2.0));
                let v = (self.h(kf, 164.0, 0.0) - 0.5) * f.t; let key = self.next_key();
                self.frame(at + m::sin(th) * hgt / 2.0, m::max(f.y0, 0.0) + m::cos(th) * hgt / 2.0, v, 0.018, hgt / 2.0, 0.018, -th, self.frame_c, key);
            }
        }
    }

    /// A run of open UPPER cells (a port, or the upper half of a hole): its lintel, its sides, the torn tops of the sills under it, its track, and what hangs and stands in it.
    fn upper_run(&mut self, run: Run) {
        let is_hole = (run.k..=run.k1).any(|j| self.w.open(j, 0));
        let lin = self.lintel_depth(&run, is_hole);
        let o = Upper { run, is_hole, lin, y_l: self.f.y_top - lin };
        if o.has_lintel() { self.lintel(&o); }
        self.upper_sides(&o);
        self.torn_sills(&o);
        if !o.has_lintel() { self.top_track(&o); }   // (else inside the lintel)
        for j in run.k..=run.k1 {
            if self.w.open(j, 0) { continue; }
            if !is_hole && !o.has_lintel() { self.hanging_boards(j); }
            if self.h(j as f64, 150.0, 0.0) < (if is_hole { 0.3 } else { 0.55 }) { self.stud(&o, j); }
        }
    }

    /// THE LINTEL: the top of the wall still spans the opening (a band of its sheet, torn along its underside). Under the ceiling only a deep lintel
    /// reads from above: a bare rail up there is a dark bar. How deep it is, 0 for none.
    fn lintel_depth(&self, run: &Run, is_hole: bool) -> f64 {
        let (e, span, kf) = (&self.e, self.span(), run.k as f64);
        let flush = span >= 1.3 && self.top() >= crate::scene_world::world().ceiling_y - 0.03;
        let spans = span >= 1.1 && (is_hole || flush || self.h(kf, 170.0, 0.0) < e.lintel_odds);
        let lin = if spans { m::min(span * 0.3, (if is_hole { e.lintel_hole } else { e.lintel_port }) * (0.75 + 0.5 * self.h(kf, 171.0, 0.0))) } else { 0.0 };
        if flush { m::max(lin, e.lintel_cut + 0.07 * self.h(kf, 172.0, 0.0)) } else { lin }
    }

    /// how far the torn edge of a sill or a lintel stands out
    fn tear(&self, o: &Upper, most: f64) -> Reach { Reach::Flat(m::min(if o.is_hole { self.e.sill_hole } else { self.e.sill_port }, most)) }

    fn lintel(&mut self, o: &Upper) {
        let (Run { k, k1, ua, ub, .. }, y_l, top) = (o.run, o.y_l, self.top());
        for j in k..=k1 {
            let xa = if j == k && !self.w.gone(j - 1, 1) { 0.02 } else { 0.0 }; let xb = if j == k1 && !self.w.gone(j + 1, 1) { 0.02 } else { 0.0 }; let uc = self.w.edge_u(j) + self.f.w / 2.0;
            let col = self.tone(self.p.paint, uc); let key = self.next_key();
            let p = self.plate(uc + (xb - xa) / 2.0, (y_l + top) / 2.0, 0.0, (self.f.w + xa + xb) / 2.0, (top - y_l) / 2.0, self.f.t - 0.0035, 0.0, col, key, true);
            self.push(p);
        }
        let (past_a, past_b) = (self.w.past(k - 1, -1, 1), self.w.past(k1 + 1, 1, 1));
        self.ragged(ua, y_l, ub, y_l, 0.0, -1.0, self.tear(o, o.lin * 0.7), o.lin, past_a, past_b, k as f64 * 8.0 + 6.0);
    }

    /// The two sides (only against a standing column of this wall). A hole's upper corners are left hanging in: a breach with a ragged arch.
    fn upper_sides(&mut self, o: &Upper) {
        let (f, e, wide) = (self.f, self.e, o.run.w * self.e.run_k);
        let lo = m::min(if o.is_hole { e.hole_out } else { e.port_out * 0.7 }, wide); let hi = m::min(if o.is_hole { e.hole_top } else { e.port_out * 1.3 }, wide);
        let y1 = if o.has_lintel() { o.y_l + o.lin * 0.5 } else { self.top() };
        for (ke, ue, s) in self.w.sides(&o.run) {
            let (bury, past_a) = (self.w.behind(ke, s, 1), if self.w.open(ke, 0) { 0.0 } else { 0.6 });
            self.ragged(ue, f.y_sill, ue, y1, s as f64, 0.0, Reach::Arch { lo, hi }, bury, past_a, 0.0, ke as f64 * 8.0 + (if s > 0 { 1.0 } else { 2.0 }));
        }
    }

    /// The torn tops of the sills left under it, run by run.
    fn torn_sills(&mut self, o: &Upper) {
        let (Run { k, k1, .. }, y) = (o.run, self.f.y_sill);
        for j in k..=k1 {
            if self.w.open(j, 0) || (j > k && !self.w.open(j - 1, 0)) { continue; }
            let mut j1 = j; while j1 < k1 && !self.w.open(j1 + 1, 0) { j1 += 1; }
            let (past_a, past_b) = (self.w.past(j - 1, -1, 0), self.w.past(j1 + 1, 1, 0));
            self.ragged(self.w.edge_u(j), y, self.w.edge_u(j1 + 1), y, 0.0, 1.0, self.tear(o, self.span() * 0.3), 0.6, past_a, past_b, j as f64 * 8.0 + 5.0);
        }
    }

    /// With no lintel, the top track: straight across a port, snapped and hanging from either side over a hole.
    fn top_track(&mut self, o: &Upper) {
        let (Run { k, k1, ua, ub, w }, t) = (o.run, self.f.t);
        let ty = self.f.y_top - 0.022; let tt = m::min(t * 0.6, 0.022);
        if !o.is_hole {
            let xa = if k > 0 { 0.03 } else { 0.0 }; let xb = if k1 < self.w.n - 1 { 0.03 } else { 0.0 };
            let col = self.tone(self.frame_c, (ua + ub) / 2.0); let key = self.next_key();
            self.frame((ua + ub + xb - xa) / 2.0, ty, 0.0, (w + xa + xb) / 2.0, 0.018, tt, 0.0, col, key);
            return;
        }
        for (ke, ue, s) in self.w.sides(&o.run) {
            let (kef, sf) = (ke as f64, s as f64);
            let len = m::min(0.7, w * (0.2 + 0.2 * self.h(kef, 31.0, sf + 2.0))); let phi = 0.3 + 0.7 * self.h(kef, 32.0, sf + 2.0);
            let v = (self.h(kef, 33.0, 0.0) - 0.5) * t * 0.6; let col = self.tone(self.frame_c, ue); let key = self.next_key();
            self.frame(ue + sf * (len / 2.0) * m::cos(phi), ty - (len / 2.0) * m::sin(phi), v, len / 2.0, 0.016, tt, -sf * phi, col, key);
        }
    }

    /// What still hangs from the top of a port's column: uneven lengths of board, square to the top so nothing shows above it.
    fn hanging_boards(&mut self, j: i64) {
        let (u0, jf, w, t, top) = (self.w.edge_u(j), j as f64, self.f.w, self.f.t, self.top());
        let mut u = u0 + 0.02 + 0.12 * self.h(jf, 39.0, 0.0); let mut i = 0;
        while u < u0 + w - 0.06 && i < 3 {
            let fi = i as f64;
            let ww = m::min(u0 + w - u, 0.09 + 0.17 * self.h(jf, 40.0 + fi, 0.0)); let d = 0.04 + 0.5 * m::pow(self.h(jf, 50.0 + fi, 0.0), 2.2);
            let whole = self.h(jf, 70.0 + fi, 0.0) < 0.65; let key = self.next_key();
            let p = self.plate(u + ww / 2.0, top - d / 2.0, 0.0, ww / 2.0, d / 2.0, (if whole { t - 0.004 } else { t * 0.5 }) - 0.0007 * fi, 0.0, if whole { self.p.paint } else { self.p.core }, key, false);
            self.push(p);
            u += ww * (0.9 + 1.3 * self.h(jf, 80.0 + fi, 0.0));
            i += 1;
        }
    }

    /// A stud the fire left standing in a column: whole, or snapped with its top hanging.
    fn stud(&mut self, o: &Upper, j: i64) {
        let (f, jf, span, c, hs) = (self.f, j as f64, self.span(), self.frame_c, 0.018);
        let us = self.w.edge_u(j) + f.w * (0.25 + 0.5 * self.h(jf, 151.0, 0.0));
        if !o.is_hole && self.h(jf, 152.0, 0.0) >= 0.5 {
            let th = (self.h(jf, 153.0, 0.0) - 0.5) * 0.06; let key = self.next_key();
            self.frame(us, (f.y_sill + f.y_top) / 2.0, 0.0, hs, span / 2.0, hs, th, c, key);
            return;
        }
        let lo = span * (0.2 + 0.25 * self.h(jf, 154.0, 0.0)); let hi = span * (0.1 + 0.16 * self.h(jf, 155.0, 0.0));
        let t1 = (self.h(jf, 156.0, 0.0) - 0.5) * 0.5; let t2 = (self.h(jf, 157.0, 0.0) - 0.5) * 0.6;
        let key = self.next_key();
        self.frame(us - m::sin(t1) * lo / 2.0, f.y_sill + m::cos(t1) * lo / 2.0 - 0.01, 0.0, hs, lo / 2.0, hs, t1, c, key);
        if !o.is_hole {
            let key = self.next_key();
            self.frame(us + m::sin(t2) * hi / 2.0, o.y_l - m::cos(t2) * hi / 2.0 + (if o.has_lintel() { 0.02 } else { -0.03 }), 0.0, hs, hi / 2.0, hs, t2, c, key);
        }
    }
}

/// The standing pieces of one wall, made again from its cells.
pub(super) fn edge_look_rebuild(g: &mut Game, b: &BreakLike) {
    let mut l = Lay::of(g, b);
    for run in l.w.runs(0) { l.sill_run(&run); }
    for run in l.w.runs(1) { l.upper_run(run); }
    let out = l.out;
    dress(g, b.id, out);
    undress(g, b.id);
}

/// A wall's pieces put up, in place of its last ones.
fn dress(g: &mut Game, id: f64, out: Vec<Bx>) {
    if let Some(old) = g.destr.edge_look.decor.delete(crate::Num(id)) { for x in old { crate::scene_boxes::box_let_go(g, x); } }
    if out.is_empty() { return; }
    let rows: Vec<FrameBoxId> = out.into_iter().map(|x| crate::scene_boxes::box_kept(g, x)).collect();
    g.destr.edge_look.decor.set(crate::Num(id), rows);
}

/// Over the level's cap it is the walls dressed longest ago that go bare (`id` is the newest, whatever it was before).
fn undress(g: &mut Game, id: f64) {
    let most = g.k.edge_look.max_decor;
    let mut total = 0.0; for x in g.destr.edge_look.decor.values() { total += x.len() as f64; }
    let mut at = 0;
    while let Some(old) = g.destr.edge_look.decor.next_key(&mut at) {
        if total <= most { break; }
        if old.0 == id { continue; }
        let gone = g.destr.edge_look.decor.delete(old).unwrap_or_default();
        total -= gone.len() as f64;
        for x in gone { crate::scene_boxes::box_let_go(g, x); }
    }
}
