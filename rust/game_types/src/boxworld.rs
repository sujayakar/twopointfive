//! THE BOX WORLD: the boxes a level is made of, and their rows as the loops that ask of them read them (the lines, the circles, the searches, the light, the smoke).
//!
//! A box is read by anybody (`boxes[b]`) and written in one place, `edit`: what it hands out writes the box's row as it goes, so a question finds the boxes as they stand whoever
//! asks it and whenever. A row is worked out of its box alone, so the rows are in no bytes. The floor's grid is: a piece is filed under the plan of what it broke from, so what is filed
//! where is not to be had from the boxes.
//!
//! WHICH BOXES HAVE A ROW. A static (the level's list, in its order), a leaf (a door's box), a part of the furniture in the level. Every other box (a piece of debris, a skin) has none,
//! and is written without one.
use crate::bytes::{ToBytes, W};
use crate::made::next;
use crate::ids::*;
use crate::scene_boxes::Box as SBox;
use crate::V3;
use alloc::vec::Vec;
use core::ops::{Deref, DerefMut, Index};

/// a static is asked of by its place in the level's list, and a question's own list of what it met holds this many (game/levelfit: a level of more is not loaded)
pub const MAX_BOXES: usize = 8192;
/// leaves, and the parts of the furniture
pub const MAX_LEAVES: usize = 512;
/// things of furniture a level may have
pub const MAX_THINGS: usize = 128;
/// cells of the floor's grid, and the entries of them all
pub const MAX_CELLS: usize = 16384;
pub const MAX_ITEMS: usize = 131072;
/// tilted statics
pub const MAX_TILTED: usize = 256;
/// cut and shaped statics, and the planes one box carries
pub const MAX_CUTS: usize = 1024;
pub const CUT_PLANES: usize = 8;
/// the doubles of a box's planes
pub const CUT_ROW: usize = CUT_PLANES * 4;
/// the sheets of glass a level may have
pub const MAX_SHEETS: usize = 256;

/// DOUBLES A BOX: centre, half sizes, cos and sin of its yaw, the yaw itself, the rotation (x y z w), the place of its planes and how many they are, and what kind of box it is.
pub const REC: usize = 16;
pub const R_C: usize = 0;
pub const R_H: usize = 3;
pub const R_COS: usize = 6;
pub const R_SIN: usize = 7;
pub const R_YAW: usize = 8;
pub const R_ROT: usize = 9;
pub const R_CUT: usize = 13;
pub const R_CUTN: usize = 14;
pub const R_KIND: usize = 15;
/// the kinds (R_KIND, a sum): fully rotated (Box.rot), round, cut (a box with planes), and a pane of glass a walker goes through
pub const K_ROT: u32 = 1;
pub const K_ROUND: u32 = 2;
pub const K_CUT: u32 = 4;
pub const K_WALK: u32 = 8;
/// the kinds that say a box is not a plain upright one (a line or a circle then takes the longer way round it)
pub const K_SHAPED: u32 = K_ROT | K_ROUND | K_CUT;

/// the flags a question reads
pub const F_NO_SHADOW: u32 = 2;
pub const F_GLASS: u32 = 1 << 17;
/// …and those a row keeps: a write of any other moves no row
pub const F_KEPT: u32 = 4294901770;

/// what snow a static is (`World.snow`): ground somebody cleared, trodden, old, and snow that stands or caps
pub const SNOW_BARE: u8 = 1;
pub const SNOW_PACKED: u8 = 2;
pub const SNOW_OLD: u8 = 3;
pub const SNOW_BANK: u8 = 4;

/// doubles of a thing of furniture: where it stands, its yaw, the radius of the upright cylinder that holds it
pub const TH: usize = 4;
pub const TH_X: usize = 0;
pub const TH_Z: usize = 1;
pub const TH_YAW: usize = 2;
pub const TH_RADIUS: usize = 3;

/// A sheet of glass: which static, which face has the sky, how far it stands on the plan from the nearest door's opening.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sheet {
    pub bi: u32,
    pub sky: i32,
    pub gap: f64,
}

/// a thing of furniture in the level now: its id, where its parts start and how many they are
#[derive(Clone, Copy, Debug, Default)]
pub struct Thing {
    pub id: u32,
    pub at: u32,
    pub parts: u32,
}

/// the glass as a question counts it: it stands where it stands · what a blast has just taken is gone to it · what a burst would break is counted out
pub const VIEW_NONE: u32 = 0;
pub const VIEW_BLOWN: u32 = 1;
pub const VIEW_GIVES: u32 = 2;
/// which view, its numbers, and the panes a burst meets (`World.view`, `view_at`, `view_met`)
pub type View = (u32, [f64; 6], Vec<(u32, [f64; 3])>);

/// THE ROWS, as the loops read them.
#[derive(Clone, Debug, Default)]
pub struct World {
    /// the statics' rows (REC doubles each), the leaves', the furniture's parts'
    pub rec: Vec<f64>,
    pub leaf: Vec<f64>,
    pub part: Vec<f64>,
    /// how low and how high a static reaches that a line or a ray can meet (one that none can reaches nowhere): what a walk passes by on before it asks the row
    pub tall: Vec<[f64; 2]>,
    /// …their flags (a part has none that a question asks for)
    pub flags: Vec<u32>,
    pub leaf_flags: Vec<u32>,
    /// the planes of a cut box (CUT_ROW doubles, n.x n.y n.z d each), by R_CUT of its row
    pub cut: Vec<f64>,
    /// what a static sends back of the light on it, of 255: its colour by luminance
    pub back: Vec<u8>,
    /// what snow a static is, a byte a static (0 none); none at all where the level has no snow
    pub snow: Vec<u8>,
    /// the grid: cell c holds items[start[c] .. start[c + 1]], in the order they were filed; its cells across and deep, a cell's side
    pub start: Vec<u32>,
    pub items: Vec<u32>,
    pub w: u32,
    pub h: u32,
    pub cs: f64,
    /// five a tilted static: its index, then the cells it is filed in (x0 x1 z0 z1)
    pub tilted: Vec<i32>,
    /// the statics that stand over the grid's edge
    pub outer: Vec<u32>,
    /// the furniture by its id (TH doubles each), and what of it is in the level now, in its own order
    pub thing: Vec<f64>,
    pub things: Vec<Thing>,
    /// the level's sheets of glass, ascending
    pub sheets: Vec<Sheet>,
    /// …and those of them that have been touched
    pub panes: crate::panes::Panes,
    /// the view of the glass a judgment in hand takes (VIEW_*), and its numbers: the point, how far it reaches, how hard, and 1 where a man counts on it
    pub view: u32,
    pub view_at: [f64; 6],
    /// …and the panes a burst there has a line to, each with the point it meets (a view of what it would break alone)
    pub view_met: Vec<(u32, [f64; 3])>,
    /// which writing of a static's row this is: no two alike in a process, so a table made from the statics is told by it
    pub wrote: u64,
    /// …and of how one stands (its place, its size, its turn), for a table that is made of that alone
    pub stood: u64,
    /// the places of the planes nobody has
    cut_free: Vec<u32>,
    /// which panes a walker goes through, a byte a static
    walk: Vec<u8>,
}
impl World {
    /// how many statics, and leaves
    #[inline(always)]
    pub fn boxes(&self) -> usize { self.flags.len() }
    #[inline(always)]
    pub fn leaves(&self) -> usize { self.leaf_flags.len() }
    /// static `bi`'s row
    #[inline(always)]
    pub fn row(&self, bi: usize) -> &[f64] { &self.rec[bi * REC..bi * REC + REC] }

    /// does a walker go through static `bi`, a pane of glass
    pub fn walked(&self, bi: usize) -> bool { self.walk[bi] != 0 }

    /// does a static stand over the grid's edge, some of it on the grid
    fn over_edge(&self, i: usize) -> bool {
        let r = self.row(i);
        let abs = |v: f64| if v < 0.0 { -v } else { v };
        let (cs, sn) = (abs(r[R_COS]), abs(r[R_SIN]));
        let over = |c: f64, e: f64, far: f64| c - e < far && c + e > 0.0 && (c - e < -1e-9 || c + e > far + 1e-9);
        let (ex, ez) = (cs * r[R_H] + sn * r[R_H + 2], sn * r[R_H] + cs * r[R_H + 2]);
        over(r[R_C], ex, self.w as f64 * self.cs) || over(r[R_C + 2], ez, self.h as f64 * self.cs)
    }
    fn told(&mut self) {
        self.wrote = next();
        self.stood = self.wrote;
    }
    /// `row` of one of the three tables is box `b`
    fn put(&mut self, of: Of, b: &SBox) {
        let (at, list) = match of { Of::Static(i) => (i, 0), Of::Leaf(i) => (i, 1), Of::Part(i) => (i, 2) };
        let o = at as usize * REC;
        let rows = match list { 0 => &mut self.rec, 1 => &mut self.leaf, _ => &mut self.part };
        let mut kind = if b.rot.is_some() { K_ROT } else { 0 } | if b.round == Some(true) { K_ROUND } else { 0 };
        let planes = b.cut.as_deref().unwrap_or(&[]);
        let planes = &planes[..planes.len().min(CUT_PLANES)];
        if !planes.is_empty() { kind |= K_CUT; }
        let flags = crate::js::to_u32(b.flags) & F_KEPT;
        if list == 0 && flags & F_GLASS != 0 && self.walk[at as usize] != 0 { kind |= K_WALK; }
        let mut r = [0.0; REC];
        r[..3].copy_from_slice(&b.c);
        r[3..6].copy_from_slice(&b.h);
        (r[R_COS], r[R_SIN], r[R_YAW]) = (maths::cos(b.yaw), maths::sin(b.yaw), b.yaw);
        r[R_KIND] = kind as f64;
        if let Some(q) = b.rot { r[R_ROT..R_ROT + 4].copy_from_slice(&q); }
        // (its planes keep the place they have)
        let had = rows[o + R_KIND] as u32 & K_CUT != 0;
        let mut moved = false;
        if !planes.is_empty() {
            let place = if had { rows[o + R_CUT] as u32 } else if let Some(p) = self.cut_free.pop() { p } else {
                self.cut.resize(self.cut.len() + CUT_ROW, 0.0);
                (self.cut.len() / CUT_ROW - 1) as u32
            };
            (r[R_CUT], r[R_CUTN]) = (place as f64, planes.len() as f64);
            for (to, p) in self.cut[place as usize * CUT_ROW..].chunks_exact_mut(4).zip(planes) {
                let now = [p.n[0], p.n[1], p.n[2], p.d];
                moved |= !same(to, &now);
                to.copy_from_slice(&now);
            }
        } else if had { self.cut_free.push(rows[o + R_CUT] as u32); }
        let stands = !same(&rows[o..o + R_CUT], &r[..R_CUT]) || (rows[o + R_KIND] as u32 ^ kind) & K_ROT != 0;
        moved |= !same(&rows[o..o + REC], &r);
        rows[o..o + REC].copy_from_slice(&r);
        match list {
            0 => {
                let back = (crate::v3::clamp(crate::v3::lum(b.albedo), 0.0, 1.0) * 255.0) as u8;
                moved |= self.flags[at as usize] != flags || self.back[at as usize] != back;
                (self.flags[at as usize], self.back[at as usize]) = (flags, back);
                let abs = |v: f64| if v < 0.0 { -v } else { v };
                let up = abs(r[R_H + 1]) + if kind & K_ROT != 0 { abs(r[R_H]) + abs(r[R_H + 2]) } else { 0.0 };
                self.tall[at as usize] = if flags & F_NO_SHADOW != 0 { [f64::INFINITY, f64::NEG_INFINITY] } else { [r[R_C + 1] - up, r[R_C + 1] + up] };
                if moved { self.wrote = next(); }
                if stands { self.stood = self.wrote; }
                if moved && self.w > 0 && !self.outer.contains(&at) && self.over_edge(at as usize) { self.outer.push(at); }
            }
            1 => self.leaf_flags[at as usize] = flags,
            _ => {}
        }
    }
    /// the planes a list's rows hold are nobody's
    fn give_back(cut_free: &mut Vec<u32>, rows: &[f64]) {
        for r in rows.chunks_exact(REC) { if r[R_KIND] as u32 & K_CUT != 0 { cut_free.push(r[R_CUT] as u32); } }
    }
}
/// WHAT IS FILED IN THE CELLS OF A GRID `w` across: cell c holds items[start[c] .. start[c + 1]] of those given, in their order, each in the cells of a plan (x0 x1 z0 z1)
pub fn filed(w: usize, cells: usize, list: &[(u32, [i32; 4])]) -> (Vec<u32>, Vec<u32>) {
    let each = |f: &mut dyn FnMut(usize, u32)| for &(i, [x0, x1, z0, z1]) in list { for z in z0..=z1 { for x in x0..=x1 { f(z as usize * w + x as usize, i); } } };
    // (a cell's own count stands one place on, and is summed up into where it starts)
    let mut start = alloc::vec![0; cells + 1];
    each(&mut |c, _| start[c + 1] += 1);
    for c in 0..cells { start[c + 1] += start[c]; }
    let mut free = start.clone();
    let mut items = alloc::vec![0; start[cells] as usize];
    each(&mut |c, i| { items[free[c] as usize] = i; free[c] += 1; });
    (start, items)
}
/// every double by its bits (a −0 is not a 0)
fn same(a: &[f64], b: &[f64]) -> bool { a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits()) }

/// THE WORLD AS A QUESTION TAKES IT: the boxes as they stand, or as they would be. `leaves` = do the leaves stand; `but` = those that are let through; `open` = statics that are let
/// through; `extra` = boxes that are not of the level and stand for this question (REC doubles each; a cut one's planes in `extra_cut`, its record's place there).
#[derive(Clone, Copy)]
pub struct Over<'a> {
    pub leaves: bool,
    pub but: &'a [u32],
    pub open: &'a [u32],
    pub extra: &'a [f64],
    pub extra_cut: &'a [f64],
}
impl Over<'_> {
    /// the statics, and the leaves
    #[inline(always)]
    pub const fn with(leaves: bool) -> Over<'static> { Over { leaves, but: &[], open: &[], extra: &[], extra_cut: &[] } }
}

/// which row is a box's
#[derive(Clone, Copy, Debug, PartialEq)]
enum Of {
    Static(u32),
    Leaf(u32),
    Part(u32),
}

/// the boxes, and their rows
#[derive(Clone, Debug, Default)]
pub struct Boxes {
    of: Tab<BoxId, SBox>,
    rows: World,
    /// whose row each box is (a box past the end has none)
    place: Vec<Option<Of>>,
}
impl Boxes {
    /// the rows as they stand
    #[inline(always)]
    pub fn world(&self) -> &World { &self.rows }
    pub fn push(&mut self, b: SBox) -> BoxId { self.of.push(b) }
    /// box `b`, to write: its row is written as what this hands out goes
    #[inline]
    pub fn edit(&mut self, b: BoxId) -> Edit<'_> { Edit(In::World(self, b)) }
    fn row(&mut self, b: BoxId) {
        if let Some(Some(of)) = self.place.get(b.ix()).copied() { self.rows.put(of, &self.of[b]); }
    }
    fn placed(&mut self, b: BoxId, of: Option<Of>) {
        if self.place.len() <= b.ix() { self.place.resize(b.ix() + 1, None); }
        self.place[b.ix()] = of;
    }

    /// THE LEVEL'S STATICS are these, in this order
    pub fn statics(&mut self, list: &[BoxId]) {
        for p in self.place.iter_mut() { if matches!(p, Some(Of::Static(_))) { *p = None; } }
        World::give_back(&mut self.rows.cut_free, &self.rows.rec);
        (self.rows.rec, self.rows.tall, self.rows.flags, self.rows.back, self.rows.walk, self.rows.snow) = Default::default();
        for &b in list { self.add_static(b); }
        self.rows.told();
    }
    /// …and this one, after them
    pub fn add_static(&mut self, b: BoxId) {
        let w = &mut self.rows;
        let i = w.flags.len() as u32;
        assert!((i as usize) < MAX_BOXES, "more statics than a question can meet ({MAX_BOXES}): levelfit refuses such a level, and wallbreak such a piece");
        w.rec.resize(w.rec.len() + REC, 0.0);
        w.tall.push([0.0; 2]);
        w.flags.push(0);
        w.back.push(0);
        w.walk.push(0);
        if !w.snow.is_empty() { w.snow.push(0); }
        w.told();
        self.placed(b, Some(Of::Static(i)));
        self.row(b);
    }
    /// …and no more than the first `n`
    pub fn cut_statics(&mut self, list: &[BoxId], n: usize) {
        for &b in &list[n..] { self.placed(b, None); }
        let w = &mut self.rows;
        World::give_back(&mut w.cut_free, &w.rec[n * REC..]);
        w.rec.truncate(n * REC);
        w.tall.truncate(n);
        w.flags.truncate(n);
        w.back.truncate(n);
        w.walk.truncate(n);
        w.snow.truncate(n);
        let mut items = Vec::with_capacity(w.items.len());
        for c in 1..w.start.len() {
            let had = core::mem::replace(&mut w.start[c - 1], items.len() as u32) as usize..w.start[c] as usize;
            items.extend(w.items[had].iter().copied().filter(|&i| (i as usize) < n));
        }
        if let Some(end) = w.start.last_mut() { *end = items.len() as u32; }
        w.items = items;
        let keep: Vec<i32> = w.tilted.chunks_exact(5).filter(|t| (t[0] as usize) < n).flatten().copied().collect();
        w.tilted = keep;
        w.outer.retain(|&i| (i as usize) < n);
        w.told();
    }
    /// THE GRID THEY ARE FILED IN, with nothing filed: its cells across and deep, a cell's side
    pub fn grid(&mut self, w: u32, h: u32, cs: f64) {
        let o = &mut self.rows;
        (o.w, o.h, o.cs) = (w, h, cs);
        o.start = alloc::vec![0; (w * h) as usize + 1];
        o.items.clear();
        o.tilted.clear();
        o.outer.clear();
    }
    /// …and these statics filed in it, each in the cells of a plan (x0 x1 z0 z1), in their order and after all that were filed before
    pub fn file(&mut self, list: &[(u32, [i32; 4])]) {
        let o = &mut self.rows;
        let (from, more) = filed(o.w as usize, o.start.len() - 1, list);
        let mut items = Vec::with_capacity(o.items.len() + more.len());
        for c in 1..from.len() {
            let had = core::mem::replace(&mut o.start[c - 1], items.len() as u32) as usize..o.start[c] as usize;
            items.extend_from_slice(&o.items[had]);
            items.extend_from_slice(&more[from[c - 1] as usize..from[c] as usize]);
        }
        *o.start.last_mut().unwrap() = items.len() as u32;
        o.items = items;
        for &(i, _) in list {
            if (i as usize) < o.boxes() && !o.outer.contains(&i) && o.over_edge(i as usize) { o.outer.push(i); }
        }
    }
    /// …and this one told among the tilted
    pub fn file_tilted(&mut self, i: u32, [x0, x1, z0, z1]: [i32; 4]) { self.rows.tilted.extend_from_slice(&[i as i32, x0, x1, z0, z1]); }
    /// THE LEAVES are these
    pub fn leaves(&mut self, list: &[BoxId]) {
        for p in self.place.iter_mut() { if matches!(p, Some(Of::Leaf(_))) { *p = None; } }
        World::give_back(&mut self.rows.cut_free, &self.rows.leaf);
        self.rows.leaf = alloc::vec![0.0; list.len() * REC];
        self.rows.leaf_flags = alloc::vec![0; list.len()];
        for (i, &b) in list.iter().enumerate() {
            self.placed(b, Some(Of::Leaf(i as u32)));
            self.row(b);
        }
    }
    /// THE FURNITURE IN THE LEVEL is this: each thing's id and its parts, in its own order
    pub fn things<'a>(&mut self, list: impl Iterator<Item = (u32, &'a [BoxId])>) {
        for p in self.place.iter_mut() { if matches!(p, Some(Of::Part(_))) { *p = None; } }
        World::give_back(&mut self.rows.cut_free, &self.rows.part);
        self.rows.part.clear();
        self.rows.things.clear();
        for (id, parts) in list {
            let at = (self.rows.part.len() / REC) as u32;
            self.rows.things.push(Thing { id, at, parts: parts.len() as u32 });
            for (k, &b) in parts.iter().enumerate() {
                self.rows.part.resize(self.rows.part.len() + REC, 0.0);
                self.placed(b, Some(Of::Part(at + k as u32)));
                self.row(b);
            }
        }
    }
    /// where thing `id` stands, how it is turned, the cylinder that holds it
    pub fn thing(&mut self, id: u32, x: f64, z: f64, yaw: f64, radius: f64) {
        let (t, at) = (&mut self.rows.thing, id as usize * TH);
        if t.len() < at + TH { t.resize(at + TH, 0.0); }
        t[at..at + TH].copy_from_slice(&[x, z, yaw, radius]);
    }
    /// what snow the statics are
    pub fn snow(&mut self, kinds: Vec<u8>) { self.rows.snow = kinds; }
    /// may a walker go through static `bi`, a pane of glass
    pub fn walk(&mut self, bi: usize, b: BoxId, on: bool) {
        if (self.rows.walk[bi] != 0) == on { return; }
        self.rows.walk[bi] = on as u8;
        self.row(b);
    }
    /// the level's sheets of glass
    pub fn glass(&mut self, sheets: Vec<Sheet>) { self.rows.sheets = sheets; }
    /// the panes touched, to write
    pub fn panes(&mut self) -> &mut crate::panes::Panes { &mut self.rows.panes }
    /// the rows lent to a rule of the panes, which writes `panes` and asks lines of the rest
    pub fn lent(&mut self) -> &mut World { &mut self.rows }
    /// the view of the glass the judgment in hand takes
    /// → the one that was
    pub fn view(&mut self, (kind, at, met): View) -> View {
        let w = &mut self.rows;
        (core::mem::replace(&mut w.view, kind), core::mem::replace(&mut w.view_at, at), core::mem::replace(&mut w.view_met, met))
    }

    /// THE CHECK, against the lists as the game has them (a thing's id, its parts, where it stands): the first box that is in a list and not in its place here, or whose row is not
    /// the box as it stands, by what it is and its place
    pub fn astray<'a>(&self, statics: &[BoxId], leaves: &[BoxId], things: impl Iterator<Item = (u32, &'a [BoxId], [f64; TH])>) -> Option<(&'static str, usize)> {
        let is = |b: BoxId, of: Of| self.place.get(b.ix()).copied().flatten() == Some(of);
        if statics.len() != self.rows.boxes() { return Some(("count of the statics", statics.len())); }
        if leaves.len() != self.rows.leaves() { return Some(("count of the leaves", leaves.len())); }
        if let Some(i) = (0..statics.len()).find(|&i| !is(statics[i], Of::Static(i as u32))) { return Some(("static's place", i)); }
        if let Some(i) = (0..leaves.len()).find(|&i| !is(leaves[i], Of::Leaf(i as u32))) { return Some(("leaf's place", i)); }
        let (mut n, mut at) = (0, 0);
        for (id, parts, stands) in things {
            let o = id as usize * TH;
            if self.rows.things.get(n).map(|t| (t.id, t.at, t.parts)) != Some((id, at, parts.len() as u32)) { return Some(("thing of the furniture", n)); }
            if !same(&self.rows.thing[o..o + TH], &stands) { return Some(("place of a thing of the furniture", n)); }
            if let Some(k) = (0..parts.len()).find(|&k| !is(parts[k], Of::Part(at + k as u32))) { return Some(("part's place", at as usize + k)); }
            (n, at) = (n + 1, at + parts.len() as u32);
        }
        if n != self.rows.things.len() { return Some(("count of the furniture", n)); }
        if !self.rows.start.is_sorted() || self.rows.start.last().map_or(0, |&n| n as usize) != self.rows.items.len() { return Some(("grid", 0)); }
        if let Some(i) = self.rows.items.iter().find(|&&i| i as usize >= statics.len()) { return Some(("static the grid names", *i as usize)); }
        let mut w = self.rows.clone();
        for (i, of) in self.place.iter().enumerate().filter_map(|(i, p)| p.map(|p| (i, p))) { w.put(of, &self.of[BoxId::from_ix(i)]); }
        let off = |a: &[f64], b: &[f64]| a.iter().zip(b).position(|(x, y)| x.to_bits() != y.to_bits()).map(|i| i / REC);
        None.or(off(&w.rec, &self.rows.rec).map(|i| ("static", i)))
            .or(off(&w.leaf, &self.rows.leaf).map(|i| ("leaf", i)))
            .or(off(&w.part, &self.rows.part).map(|i| ("part of the furniture", i)))
            .or(w.flags.iter().zip(&self.rows.flags).position(|(x, y)| x != y).map(|i| ("static's flags", i)))
            .or(w.leaf_flags.iter().zip(&self.rows.leaf_flags).position(|(x, y)| x != y).map(|i| ("leaf's flags", i)))
            .or(off(&w.cut, &self.rows.cut).map(|i| ("cut", i)))
    }
}
impl Deref for Boxes {
    type Target = Tab<BoxId, SBox>;
    #[inline]
    fn deref(&self) -> &Tab<BoxId, SBox> { &self.of }
}
impl Index<BoxId> for Boxes {
    type Output = SBox;
    #[inline]
    fn index(&self, b: BoxId) -> &SBox { &self.of[b] }
}
impl ToBytes for Boxes {
    fn w(&self, o: &mut W) {
        self.of.w(o);
        self.rows.start.w(o);
        self.rows.items.w(o);
        self.rows.tilted.w(o);
    }
}

/// a box in hand to write: its row is written as this goes
pub struct Edit<'a>(In<'a>);
enum In<'a> {
    World(&'a mut Boxes, BoxId),
    /// (a picture's own box: it has no row)
    Frame(&'a mut SBox),
}
impl<'a> Edit<'a> {
    pub fn frame(b: &'a mut SBox) -> Edit<'a> { Edit(In::Frame(b)) }
}
impl Deref for Edit<'_> {
    type Target = SBox;
    #[inline]
    fn deref(&self) -> &SBox { match &self.0 { In::World(w, b) => &w.of[*b], In::Frame(b) => b } }
}
impl DerefMut for Edit<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut SBox { match &mut self.0 { In::World(w, b) => &mut w.of[*b], In::Frame(b) => b } }
}
impl Drop for Edit<'_> {
    #[inline]
    fn drop(&mut self) { if let In::World(w, b) = &mut self.0 { w.row(*b) } }
}

/// the rows of boxes a question adds, and their planes: as the world's own rows are written
pub fn rows_of<'a>(boxes: impl ExactSizeIterator<Item = &'a SBox>) -> (Vec<f64>, Vec<f64>) {
    let mut w = World { part: alloc::vec![0.0; boxes.len() * REC], ..World::default() };
    for (i, b) in boxes.enumerate() { w.put(Of::Part(i as u32), b); }
    (w.part, w.cut)
}
