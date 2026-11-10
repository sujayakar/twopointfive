//! One sheet cut again: what has gone of what it lies on, the rectangles of it that stand, and a piece of its own paint for each.
use super::*;

/// A sheet and what it lies on: its thin axis, its long one, and half its face each way.
struct Sheet {
    j: f64,
    s: Bx,
    seats: Vec<Seat>,
    t: f64,
    a: f64,
    hu: f64,
    hv: f64,
}

/// What stands behind it, in its own frame: what it lay on, how near, and what has gone of each; and of what stands, the pieces that END RAGGED
/// (a breakable's standing piece under a cut record): the part of the sheet on such a piece takes the outline of that piece's face.
struct Behind {
    on: Vec<Rect>,
    near: Vec<f64>,
    open: Vec<Vec<Rect>>,
    parts: Vec<Rect>,
    rag: Vec<(Bx, Seat)>,
}

/// One sheet cut again from what stands behind it.
pub(super) fn wall_skin_recut(g: &mut Game, j: f64) {
    let Some(sh) = Sheet::of(g, j) else { return };
    let mut bh = Behind::of(g, &sh);
    if bh.nothing_gone() { return uncut(g, j); }
    let rects = bh.standing(&sh, false);
    if sh.whole(&rects) && bh.parts.is_empty() { return uncut(g, j); }   // (something solid stands behind every part of it that lay on anything: the authored box, whole)
    if sh.marked() && sh.base().len() as f64 + 4.0 > crate::scene_cut::MAX_CUT_PLANES {   // (its marks are laid out on the whole box and no record can be drawn, or has room: left as it was)
        g.destr.walls.skins.marked.add(crate::Num(j));
        return;
    }
    let Some(rects) = fitted(g, &sh, &mut bh, rects) else { return };
    if sh.whole(&rects) && bh.parts.is_empty() { return uncut(g, j); }   // (its ragged ends given up for the caps, nothing of it is cut)
    let (mut pieces, mut kept, mut bit): (Vec<Bx>, _, _) = (Vec::new(), Vec::new(), false);
    for x in places(g, j, &rects) {
        let (p, bitten) = sh.piece(&bh, &x);
        bit |= bitten;
        if let Some(p) = p { pieces.push(p); kept.push(x); }
    }
    let area = rects.iter().fold(0.0, |sum, r| sum + (r[1] - r[0]) * (r[3] - r[2]));
    if !bit && area >= 4.0 * sh.hu * sh.hv - EPS { return uncut(g, j); }   // (split at a ragged piece's bounds, and no plane of it bites the sheet: the authored box, whole)
    g.destr.walls.skins.laid.set(crate::Num(j), kept);
    let rows: Vec<FrameBoxId> = pieces.into_iter().map(|p| crate::scene_boxes::box_kept(g, p)).collect();
    if let Some(old) = g.destr.walls.skins.cut.set(crate::Num(j), rows) { for b in old { crate::scene_boxes::box_let_go(g, b); } }
}

/// The authored box again, whole.
fn uncut(g: &mut Game, j: f64) {
    if let Some(old) = g.destr.walls.skins.cut.delete(crate::Num(j)) { for b in old { crate::scene_boxes::box_let_go(g, b); } }
    g.destr.walls.skins.laid.delete(crate::Num(j));
}

/// The rectangles as the caps allow: past one the ragged ends are given up (`bh` forgets them) for square ones, whole if nothing then is cut, and past it
/// still, it is cut by whole columns. None = it stays as it last stood.
fn fitted(g: &Game, sh: &Sheet, bh: &mut Behind, rects: Vec<Rect>) -> Option<Vec<Rect>> {
    let kk = g.k.wallskin;
    // (what the other sheets hand over: one the hangers have since thrown spends none of the cap)
    let mut others = 0.0;
    for (i, l) in g.destr.walls.skins.cut.iter() { if i.0 != sh.j && crate::scene_boxes::box_drawn(g, &g.col.boxes, g.col.boxes[i.0 as usize]) { others += l.len() as f64; } }
    let fits = |r: &[Rect]| r.len() as f64 <= m::min(kk.per_skin, SLOTS) && others + r.len() as f64 <= kk.max_pieces;
    if fits(&rects) { return Some(rects); }
    if !bh.parts.is_empty() {
        bh.parts.clear();
        bh.rag.clear();
        let rects = bh.standing(sh, false);
        if fits(&rects) || sh.whole(&rects) { return Some(rects); }
    }
    Some(bh.standing(sh, true)).filter(|r| fits(r))
}

/// A place for each rectangle: a piece's key is its sheet and one of 16 places. A piece a later cut leaves alone keeps its place, and a new one takes a
/// place no piece of the last cut had while there is one.
fn places(g: &Game, j: f64, rects: &[Rect]) -> Vec<WallSkinLaidValueItem> {
    let prev: Vec<WallSkinLaidValueItem> = g.destr.walls.skins.laid.get(crate::Num(j)).cloned().unwrap_or_default();
    let old: Vec<f64> = prev.iter().map(|p| p.slot).collect(); let mut now: Vec<f64> = Vec::new();
    let mut lay: Vec<WallSkinLaidValueItem> = rects.iter().map(|r| {
        let same = prev.iter().find(|p| (0..4).all(|i| m::abs(p.r[i] - r[i]) <= EPS));
        if let Some(sm) = same { if !now.contains(&sm.slot) { now.push(sm.slot); } }
        WallSkinLaidValueItem { r: *r, slot: same.map(|x| x.slot).unwrap_or(-1.0) }
    }).collect();
    for x in lay.iter_mut() {
        if x.slot >= 0.0 { continue; }
        let mut k = 0.0; while k < SLOTS && (now.contains(&k) || old.contains(&k)) { k += 1.0; }
        if k == SLOTS { k = 0.0; while now.contains(&k) { k += 1.0; } }
        x.slot = k; now.push(k);
    }
    lay
}

impl Sheet {
    fn of(g: &mut Game, j: f64) -> Option<Sheet> {
        let seats = wall_skin_backs(g, j);
        let t = seats.first()?.t; let a = 2.0 - t;
        let s = g.tab.boxes[g.col.boxes[j as usize]].clone();
        let (hu, hv) = (s.h[ax(a)], s.h[1]);
        Some(Sheet { j, s, seats, t, a, hu, hv })
    }
    fn marked(&self) -> bool { self.s.marks.as_ref().is_some_and(|x| !x.is_empty()) }
    /// The planes of its own shape.
    fn base(&self) -> &[CutPlane] { self.s.cut.as_deref().unwrap_or(&[]) }
    fn whole(&self, r: &[Rect]) -> bool {
        let (hu, hv) = (self.hu, self.hv);
        r.len() == 1 && r[0][0] <= -hu + EPS && r[0][1] >= hu - EPS && r[0][2] <= -hv + EPS && r[0][3] >= hv - EPS
    }
    /// Does a rectangle of its frame lie on it at all.
    fn lies_on(&self, r: &Rect) -> bool { m::min(r[1], self.hu) - m::max(r[0], -self.hu) > EPS && m::min(r[3], self.hv) - m::max(r[2], -self.hv) > EPS }
    /// The rectangle of its frame that a box `b`'s extent from `lo` to `hi` covers.
    fn covered(&self, b: &Bx, lo: V3, hi: V3) -> Rect {
        let (p, q, a) = (into_b(&self.s, out_of_b(b, lo)), into_b(&self.s, out_of_b(b, hi)), ax(self.a));
        [m::min(p[a], q[a]), m::max(p[a], q[a]), m::min(p[1], q[1]), m::max(p[1], q[1])]
    }

    /// The piece of it that stands over one rectangle (None: the ragged piece behind leaves nothing of it), and whether a plane of what is behind bites it.
    fn piece(&self, bh: &Behind, x: &WallSkinLaidValueItem) -> (Option<Bx>, bool) {
        let s = &self.s; let fl = crate::scene_boxes::box_flag();
        let mut p = s.clone();
        p.flags = ((js::to_i32(s.flags) & !js::to_i32(fl.no_primary)) | js::to_i32(fl.dynamic)) as f64; p.name = Some(String::from("skin"));
        p.mkey = Some(crate::scene_boxes::mkey().skin + self.j * SLOTS + x.slot);
        if self.marked() { p.cut = Some(self.marked_planes(&x.r)); } else { self.trim(&mut p, &x.r); }
        // (a plain box of the sheet's paint: what the level's stand-up keeps on the authored one is not its)
        p.shape = None; p.shaped = None; p.piece = None; p.of = None;
        let more = bh.outline(self, &x.r, p.c);
        if more.is_empty() { return (Some(p), false); }
        let mine = p.cut.clone().unwrap_or_default();
        let own = crate::scene_cut::cut_record(p.h, &mine).cut.map_or(0, |c| c.len());
        let mut both = mine; both.extend(more);
        let cut = crate::scene_cut::cut_record(p.h, &both);
        if cut.empty { return (None, true); }
        let bitten = cut.cut.as_ref().map_or(0, |c| c.len()) > own;
        p.cut = cut.cut;
        (Some(p), bitten)
    }

    /// A marked sheet is the whole box again, under the planes of the rectangle that stands (a side that is the box's own takes none): every mark where it was.
    fn marked_planes(&self, r: &Rect) -> Vec<CutPlane> {
        let (a, hu, hv) = (ax(self.a), self.hu, self.hv);
        let mut planes: Vec<CutPlane> = self.base().to_vec();
        let mut side = |axis: usize, sign: f64, at: f64| { let mut n: V3 = [0.0, 0.0, 0.0]; n[axis] = sign; planes.push(CutPlane { n, d: sign * at }); };
        if r[0] > -hu + EPS { side(a, -1.0, r[0]); }
        if r[1] < hu - EPS { side(a, 1.0, r[1]); }
        if r[2] > -hv + EPS { side(1, -1.0, r[2]); }
        if r[3] < hv - EPS { side(1, 1.0, r[3]); }
        planes
    }

    /// Any other sheet's piece is a box of the rectangle (a shape's planes are in the box's own frame: the same planes about the piece's middle).
    fn trim(&self, p: &mut Bx, r: &Rect) {
        let (mut lc, mut h): (V3, V3) = ([0.0, (r[2] + r[3]) / 2.0, 0.0], [0.0, (r[3] - r[2]) / 2.0, 0.0]);
        lc[ax(self.a)] = (r[0] + r[1]) / 2.0; h[ax(self.a)] = (r[1] - r[0]) / 2.0; h[ax(self.t)] = self.s.h[ax(self.t)];
        p.c = out_of_b(&self.s, lc); p.h = h;
        let planes: Vec<CutPlane> = self.base().iter().map(|c| CutPlane { n: c.n, d: c.d - (c.n[0] * lc[0] + c.n[1] * lc[1] + c.n[2] * lc[2]) }).collect();
        p.cut = if planes.is_empty() { None } else { Some(planes) };
    }
}

impl Behind {
    fn of(g: &Game, sh: &Sheet) -> Behind {
        let mut bh = Behind {
            on: sh.seats.iter().map(|q| q.r).collect(),
            near: sh.seats.iter().map(|q| q.near).collect(),
            open: sh.seats.iter().map(|q| gone_of(g, sh, q)).collect(),
            parts: Vec::new(),
            rag: Vec::new(),
        };
        for q in sh.seats.iter() {
            for p in standing_of(g, q) {
                let r = sh.covered(p, [-p.h[0], -p.h[1], -p.h[2]], p.h);
                if sh.lies_on(&r) { bh.parts.push(r); bh.rag.push((p.clone(), *q)); }
            }
        }
        bh
    }
    fn nothing_gone(&self) -> bool { self.parts.is_empty() && self.open.iter().all(|l| l.is_empty()) }
    /// The rectangles of it that stand; `cols`: whole columns only.
    fn standing(&self, sh: &Sheet, cols: bool) -> Vec<Rect> {
        standing(sh.hu, sh.hv, &self.on, &self.near, &self.open, Some(cols), if cols { None } else { Some(&self.parts) })
    }

    /// The planes of the ragged piece behind rectangle `r`, if one is, about a box of the sheet's frame centred at `pc`: each taken where it meets the face
    /// the sheet lies on, and stood square to that face.
    fn outline(&self, sh: &Sheet, r: &Rect, pc: V3) -> Vec<CutPlane> {
        let Some(k) = self.parts.iter().position(|x| inside(*x, (r[0] + r[1]) / 2.0, (r[2] + r[3]) / 2.0)) else { return Vec::new(); };
        let (p, q) = (&self.rag[k].0, &self.rag[k].1);
        let t = ax(sh.t);
        let o = into_b(p, pc); let tf = q.side * (sh.s.h[t] + m::max(q.near, 0.0)); let zero = into_b(&sh.s, p.c);
        let mut out: Vec<CutPlane> = Vec::new();
        for c in p.cut.as_ref().unwrap().iter() {
            let mut n = into_b(&sh.s, out_of_b(p, c.n)); n[0] -= zero[0]; n[1] -= zero[1]; n[2] -= zero[2];
            let d = c.d - (c.n[0] * o[0] + c.n[1] * o[1] + c.n[2] * o[2]) - n[t] * tf; n[t] = 0.0;
            let l = m::hyp3(n[0], n[1], n[2]);
            if l > 1e-6 { out.push(CutPlane { n: [n[0] / l, n[1] / l, n[2] / l], d: d / l }); }
        }
        out
    }
}

/// What has gone of a thing it lay on: a breakable's open cells at that face; any other solid whole, once something has hidden it.
fn gone_of(g: &Game, sh: &Sheet, q: &Seat) -> Vec<Rect> {
    if let Some(b) = brk_id(g, q.k) { return open_cells(g, sh, q, &g.tab.wallbreak_breakable[b]); }
    let hidden = js::to_i32(g.tab.boxes[g.col.boxes[q.k as usize]].flags) & js::to_i32(crate::scene_boxes::box_flag().no_primary) != 0;
    if hidden { vec![all()] } else { Vec::new() }
}

/// The standing pieces of a breakable that end ragged.
fn standing_of<'a>(g: &'a Game, q: &Seat) -> Vec<&'a Bx> {
    let Some(b) = brk_id(g, q.k) else { return Vec::new() };
    let w = &g.tab.wallbreak_breakable[b];
    let pieces = w.slots.iter().take(m::max(0.0, m::ceil(w.live)) as usize).filter_map(|x| g.col.boxes.get(*x as usize));
    pieces.map(|p| &g.tab.boxes[*p]).filter(|p| p.cut.as_ref().is_some_and(|c| !c.is_empty())).collect()
}

/// A breakable's open cells at the face it lies on, in its own frame.
fn open_cells(g: &Game, sh: &Sheet, q: &Seat, brk: &crate::wallbreak::Breakable) -> Vec<Rect> {
    let w = &g.tab.boxes[brk.box_];
    let (n, face, u) = (q.n, q.face, 2.0 - q.n);
    let nn = if n == 0.0 { brk.nz } else { brk.nx };
    let at = if face > 0.0 { (if n == 0.0 { brk.nx } else { brk.nz }) - 1.0 } else { 0.0 };
    let mut out: Vec<Rect> = Vec::new();
    let mut k = 0.0;
    while k < nn {
        let mut iy = 0.0;
        while iy < brk.ny {
            let (ix, iz) = (if n == 0.0 { at } else { k }, if n == 0.0 { k } else { at });
            if brk.open.get(((iy * brk.nz + iz) * brk.nx + ix) as usize).copied().unwrap_or(0) != 0 {
                let (mut lo, mut hi): (V3, V3) = ([0.0, brk.yb[iy as usize], 0.0], [0.0, brk.yb[iy as usize + 1], 0.0]);
                lo[ax(n)] = face * w.h[ax(n)]; hi[ax(n)] = face * w.h[ax(n)];
                lo[ax(u)] = -w.h[ax(u)] + 2.0 * w.h[ax(u)] * k / nn; hi[ax(u)] = -w.h[ax(u)] + 2.0 * w.h[ax(u)] * (k + 1.0) / nn;
                let r = sh.covered(w, lo, hi);
                if sh.lies_on(&r) { out.push(r); }
            }
            iy += 1.0;
        }
        k += 1.0;
    }
    out
}
