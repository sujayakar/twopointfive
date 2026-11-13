//! The rubble at a broken wall's foot: what each opened cell leaves, cut, painted, and thrown to where it lies.
use super::*;

/// One opened cell, and everything its pieces have in common.
struct Cell<'a> {
    b: &'a BreakLike,
    c: f64,
    /// The wall's frame; None for cover.
    f: Option<Frame>,
    blast: bool,
    e: crate::knobs::EdgeLookK,
    p: Painted,
    /// The cell: its middle on plan, half its width, the way off it and along it, and its foot and its head.
    centre: V3,
    half: f64,
    nrm: V3,
    along: V3,
    y_lo: f64,
    y_hi: f64,
    floor: f64,
    /// The side the round or the blast sends most of it.
    far: f64,
    /// Over a sill still standing.
    sill_up: bool,
    /// How many pieces it leaves.
    n: f64,
}

/// Where a piece is thrown: from where, which way, how far, and how far along the cell.
struct Thrown {
    from: V3,
    rd: V3,
    d: f64,
    u: f64,
}

/// What a piece is cut to.
struct Cut {
    h: V3,
    lump: bool,
    slab: bool,
}

/// What one opened cell leaves at the foot: fragments at rest positions hashed off the cell, flying there from the cell unless quiet.
pub(super) fn edge_look_shed(g: &mut Game, b: &BreakLike, f: Option<Frame>, c: f64, dir: Option<V3>, quiet: bool, blast: bool) {
    let cell = Cell::of(g, b, f, c, dir, blast);
    let mut i = 0.0;
    while i < cell.n {
        cell.piece(g, i, quiet);
        i += 1.0;
    }
}

impl<'a> Cell<'a> {
    fn of(g: &Game, b: &'a BreakLike, f: Option<Frame>, c: f64, dir: Option<V3>, blast: bool) -> Cell<'a> {
        let p = painted(g, b);
        let (bc, bh, byaw) = { let x = &g.tab.boxes[b.box_]; (x.c, x.h, x.yaw) };
        let (ix, iz, iy) = (js::rem(c, b.nx), js::rem(m::floor(c / b.nx), b.nz), m::floor(c / (b.nx * b.nz)));
        let e = g.k.edge_look;
        let upper = iy != 0.0;
        let mut cell = Cell {
            b, c, f, blast, e, p,
            centre: [0.0; 3], half: 0.0, nrm: [0.0; 3], along: [0.0; 3], y_lo: 0.0, y_hi: 0.0,
            floor: m::max(0.0, bc[1] - bh[1]),
            far: 0.0, sill_up: f.is_some() && iy == 1.0 && b.open.get((c - b.nx * b.nz) as usize) == Some(&0), n: 0.0,
        };
        if let Some(f) = f {
            let k = if f.along_x { ix } else { iz }; let u = -f.l / 2.0 + f.w * (k + 0.5);
            cell.n = m::round((if upper { e.per_upper } else { e.per_sill }) * (if blast { e.blast.more } else { 1.0 }));
            cell.centre = edge_look_at(g, f, u, 0.0, 0.0); cell.half = f.w / 2.0; cell.nrm = f.nrm; cell.along = f.along;
            cell.y_lo = if upper { f.y_sill } else { f.y0 }; cell.y_hi = if upper { f.y_top } else { f.y_sill };
        } else {   // cover: a plan cell, top to bottom; thrown the way the round went
            let (cs, sn) = (m::cos(byaw), m::sin(byaw));
            let lx = -bh[0] + 2.0 * bh[0] * (ix + 0.5) / b.nx; let lz = -bh[2] + 2.0 * bh[2] * (iz + 0.5) / b.nz;
            cell.n = e.per_cover; cell.centre = [bc[0] + cs * lx + sn * lz, 0.0, bc[2] - sn * lx + cs * lz]; cell.half = m::min(bh[0] / b.nx, bh[2] / b.nz);
            let d: V3 = match dir { Some(d) if m::hyp2(d[0], d[2]) > 0.05 => v3::normalize([d[0], 0.0, d[2]]), _ => { let a = cell.hh(1.0, 0.0) * 6.283; [m::cos(a), 0.0, m::sin(a)] } };
            cell.nrm = d; cell.along = [-d[2], 0.0, d[0]]; cell.y_lo = bc[1] - bh[1]; cell.y_hi = bc[1] + bh[1];
        }
        cell.far = match dir { Some(d) => if v3::dot(d, cell.nrm) >= 0.0 { 1.0 } else { -1.0 }, None => if cell.hh(2.0, 0.0) < 0.5 { 1.0 } else { -1.0 } };
        cell
    }

    /// The cell's own luck: `a` the question, `k` the piece.
    fn hh(&self, a: f64, k: f64) -> f64 { look_hash([self.b.id + 1.0, self.c + 5.0, a + 3.0, k + 17.0]) }
    fn wood(&self) -> bool { self.b.surf == "wood" }
    fn soft(&self) -> bool { self.b.surf == "cardboard" || self.b.surf == "fabric" }

    /// One piece, cut and lying where it was thrown, or on its way there.
    fn piece(&self, g: &mut Game, i: f64, quiet: bool) {
        let th = self.thrown(g, i);
        let cut = self.cut(i);
        let [_, hy, hz] = cut.h;
        let tilt = self.tilt(i, &th, &cut);
        let q1 = quat::normalize(quat::mul(quat::yaw(self.hh(19.0, i) * 6.283), quat::axis_angle([1.0, 0.0, 0.0], tilt)));
        let (px, pz) = (th.from[0] + th.rd[0] * th.d, th.from[2] + th.rd[2] * th.d);
        // (a READ: the floor it lands on: cover on legs sheds onto the ground beside it)
        let ground = match crate::scene_collision::raycast(g, [px, self.floor + 0.3, pz], [0.0, -1.0, 0.0], self.floor + 0.35) { Some(dn) => self.floor + 0.3 - dn.t, None => 0.0 };
        let serial = g.destr.edge_look.serial;
        g.destr.edge_look.serial = serial + 1.0;
        let y1 = ground + hy + m::abs(m::sin(tilt)) * hz + 0.002 + 0.0012 * js::rem(serial + i, 7.0);
        let at: V3 = [px, y1, pz];
        let soot = if !self.blast { 1.0 } else if th.d < 0.7 { 0.4 + 0.5 * self.hh(21.0, i) } else { 0.75 + 0.25 * self.hh(21.0, i) };
        let bx = self.box_of(g, at, &cut, q1, self.colour(i, soot), serial);
        let p1 = self.broken(g, bx, &cut, i, soot).unwrap_or(at);
        let mut r = Rubble { box_: bx, t: 1.0, t_2: 1.0, p0: p1, p1, vy: 0.0, q0: q1, q1, foot: None };
        if !quiet {
            self.flies(&mut r, i, &th, y1);
            let x = &mut g.tab.frame_boxes[bx]; x.c = r.p0; x.rot = Some(r.q0);
            g.destr.edge_look.flying += 1.0;
        }
        g.destr.edge_look.rubble.push(r);
    }

    /// Where it is thrown. Over a sill still standing the bits lie at its foot, either side: not inside it.
    fn thrown(&self, g: &Game, i: f64) -> Thrown {
        let (e, blast) = (&self.e, self.blast);
        let reach = if blast { e.throw_blast } else { e.throw_fire };
        let side = match self.f { Some(_) => if self.hh(10.0, i) < (if blast { 0.6 } else { 0.78 }) { self.far } else { -self.far }, None => 1.0 };
        let flung = blast && self.hh(28.0, i) < e.blast.flung;   // (a few go well into the room; the rest heap at the wall's foot)
        let out = if flung { reach * (1.2 + 1.6 * self.hh(11.0, i)) } else { reach * m::pow(self.hh(11.0, i), if blast { 2.6 } else { 2.0 }) };
        let mut d = (match self.f { Some(f) => f.t, None => self.half }) + 0.04 + out * (if side == self.far { 1.0 } else if blast { e.blast.near_k } else { 0.45 });
        let u = (self.hh(12.0, i) - 0.5) * (2.0 * self.half + (if blast { 0.5 } else { 0.25 }));
        let rd = v3::scale(self.nrm, side);
        let off = match self.f { Some(f) if self.sill_up => f.t + 0.02, _ => 0.0 };
        let from: V3 = [self.centre[0] + self.along[0] * u + rd[0] * off, self.floor + 0.12, self.centre[2] + self.along[2] * u + rd[2] * off];
        d = m::max(0.03, d - off);
        // (a READ: rubble stops short of whatever stands there)
        if let Some(hit) = crate::scene_collision::raycast(g, from, rd, d + 0.14) { d = m::max(0.03, hit.t - 0.14) * (0.5 + 0.5 * self.hh(13.0, i)); }
        Thrown { from, rd, d, u }
    }

    /// What it is cut to: a board flake lying nearly flat (a splinter off wood), now and then a lump; a blast's heap off a wall has a few big SLABS of the
    /// wall's own sheet lying flat, and of the chunks a good share much smaller.
    fn cut(&self, i: f64) -> Cut {
        let (e, ch, blast, wood) = (&self.e, &self.e.cut_heap, self.blast, self.wood());
        let lump = self.hh(14.0, i) < (if blast { e.blast.lumps } else { 0.34 });
        let mut hx = 0.022 + 0.085 * self.hh(15.0, i) * self.hh(15.0, i); let mut hz = 0.018 + 0.055 * self.hh(16.0, i);
        let mut hy = match self.f { Some(f) => m::min(0.011, m::max(0.005, f.t * 0.18)), None => 0.008 };
        if wood {
            hx = 0.06 + 0.12 * self.hh(15.0, i); hz = 0.008 + 0.012 * self.hh(16.0, i); hy = 0.006;
        } else if lump {
            hx = (0.018 + 0.03 * self.hh(15.0, i)) * (if blast { 2.5 * (if self.hh(29.0, i) < e.blast.slabs { e.blast.slab_k } else { 1.0 }) } else { 1.0 });
            hz = hx * (0.6 + 0.5 * self.hh(16.0, i)); hy = hx * (0.5 + 0.4 * self.hh(17.0, i));
            if let (true, Some(f)) = (blast, self.f) { hy = m::min(hy, m::max(f.t, 0.04)); }   // (a blast's chunk is a piece of the wall: never thicker than it)
        } else if blast {
            hy *= e.blast.flake_k;
        }
        let mut slab = false;
        if let (true, Some(f), false, false) = (blast, self.f, wood, self.soft()) {
            let cls = self.hh(33.0, i);
            if cls < ch.slabs { slab = true; hx = ch.slab[0] + (ch.slab[1] - ch.slab[0]) * self.hh(15.0, i); hz = hx * (0.45 + 0.5 * self.hh(16.0, i)); hy = m::min(ch.slab_t, m::max(0.012, f.t * 0.45)); }
            else if lump && cls > 1.0 - ch.smalls { let k = ch.small[0] + (ch.small[1] - ch.small[0]) * self.hh(34.0, i); hx *= k; hz *= k; hy *= k; }
        }
        if blast && !lump && self.hh(17.0, i) < 0.2 { hx *= 1.5; hz *= 1.4; }
        Cut { h: [hx, hy, hz], lump, slab }
    }

    /// How far from flat it lies: a slab lies all but flat, and what lies against the wall leans.
    fn tilt(&self, i: f64, th: &Thrown, cut: &Cut) -> f64 {
        let k = self.hh(18.0, i) - 0.5;
        if cut.slab { k * 0.16 } else if cut.lump || th.d > 0.5 { k * 0.3 } else { k * 0.9 * (1.0 - th.d / 0.5) }
    }

    /// Off a wall one of its paint, its core and its facing, each piece of a blast its own tone inside the material; off cover its paint, lighter or darker.
    fn colour(&self, i: f64, soot: f64) -> V3 {
        let (tone, paint) = (self.hh(20.0, i), self.p.paint);
        let mut col: V3 = match self.f {
            Some(_) => if tone < 0.45 { paint } else if tone < 0.8 { self.p.core } else if tone < 0.92 { self.p.inside.facing.unwrap_or(self.p.core) } else { v3::scale(paint, 0.55) },
            None => v3::scale(paint, if self.soft() { 0.75 + 0.3 * tone } else { 0.7 + 0.45 * tone }),
        };
        if self.blast { col = v3::scale(col, soot); }
        if self.blast && self.f.is_some() { col = v3::scale(col, 1.0 - self.e.cut_heap.tone * self.hh(35.0, i)); }
        col
    }

    fn box_of(&self, g: &mut Game, at: V3, cut: &Cut, q: Quat, col: V3, serial: f64) -> FrameBoxId {
        let b = crate::scene_boxes::make_box_value(&crate::scene_boxes::MakeBoxP { c: at, h: cut.h, rot: Some(q), albedo: Some(col), stuff: self.p.stuff, does: self.p.does,
            flags: Some((crate::scene_boxes::box_flag::DYNAMIC as i32 | crate::scene_boxes::box_flag::NO_SHADOW as i32) as f64),
            mkey: Some(MKEY_LOOK + 8388608.0 + (js::to_i32(serial) & 0x7fffff) as f64), ..Default::default() });
        crate::scene_boxes::box_kept(g, b)
    }

    /// Its outline by its material's row, where that has one: its middle then. A big piece casts its shadow, and its broken faces are the material's core,
    /// sooted with the piece, a little of the paper's brown in some, no two the same white.
    fn broken(&self, g: &mut Game, bx: FrameBoxId, cut: &Cut, i: f64, soot: f64) -> Option<V3> {
        if !crate::break_table::breaks_shape_loose(g, bx.into(), &self.b.surf, (self.b.id * 4096.0 + self.c) * 32.0 + i) { return None; }
        let at = g.tab.frame_boxes[bx].c;
        let long = m::max(cut.h[0], cut.h[2]);
        let no_shadow = crate::scene_boxes::box_flag::NO_SHADOW as i32;
        if long >= 0.15 && !edge_look_in_beam(g, at, long) { let x = &mut g.tab.frame_boxes[bx]; x.flags = (js::to_i32(x.flags) & !no_shadow) as f64; }
        if let (true, true, Some(cc)) = (self.blast, self.f.is_some(), g.tab.frame_boxes[bx].cut_core) {
            let ch = &self.e.cut_heap;
            let core = v3::lerp3(cc, self.p.inside.facing.unwrap_or(cc), ch.core_paper * self.hh(36.0, i));
            g.tab.frame_boxes[bx].cut_core = Some(v3::scale(core, soot * (1.0 - ch.core_tone * self.hh(37.0, i))));
        }
        Some(at)
    }

    /// From the cell that went, down a short arc to where it lies: more of a shove from a blast.
    fn flies(&self, r: &mut Rubble, i: f64, th: &Thrown, y1: f64) {
        let y_s = self.y_lo + (self.y_hi - self.y_lo) * (0.15 + 0.75 * self.hh(22.0, i)); let u_s = th.u * 0.6;
        r.p0 = [self.centre[0] + self.along[0] * u_s, y_s, self.centre[2] + self.along[2] * u_s];
        let up = (if self.blast { 1.6 } else { 0.5 }) * self.hh(23.0, i);
        r.t_2 = (up + m::sqrt(up * up + 19.6 * m::max(0.02, y_s - y1))) / 9.8 + 0.03 * i * (if self.blast { 0.3 } else { 1.0 });
        r.vy = (y1 - y_s + 4.9 * r.t_2 * r.t_2) / r.t_2; r.t = 0.0;
        r.q0 = quat::normalize(quat::mul(quat::axis_angle(v3::normalize([self.hh(24.0, i) - 0.5, self.hh(25.0, i) - 0.5, self.hh(26.0, i) - 0.5]), 2.0 + 5.0 * self.hh(27.0, i)), r.q1));
    }
}
