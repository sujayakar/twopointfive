//! THE MARKS ON THE WORLD, as the GPU reads them (render/decals.ts says the layout, render/shaders/decals.wgsl reads it): one buffer, a header, the table of the floor's metres (a word a cell:
//! where its list starts and how long it is), the lists (a mark's place, oldest first), and three rows of four words a mark: (pos, radius) (normal, param) (tangent, bits). What the game
//! has is the game's (`game::render_decals`), each with its key; this is only how they stand now: a mark that rides a box goes where the box is drawn this frame, on that box alone, and is
//! not drawn while the box is not. The page sends the buffer when a word of it moved.
use crate::boxes::{Boxes, Word};
use crate::Row;
use alloc::vec::Vec;
use game::game::Game;
use game::ids::AnyBoxId;
use game::m;
use game::render_decals::{mark_set, to_world, Mark, CAP};
use game::scene_world::world;

/// the floor's metres
const CELLS: usize = 40 * 28;
const CELLS_V4: usize = CELLS.div_ceil(4);
/// entries of all the cells' lists, and a cell's newest kept
const LIST_CAP: usize = 4096;
const MOST_A_CELL: usize = 48;
/// the buffer's parts, in rows of four words
const CELL0: usize = 1;
const LIST0: usize = CELL0 + CELLS_V4;
const REC0: usize = LIST0 + LIST_CAP / 4;
const TOTAL: usize = REC0 + CAP * 3;
/// a record's words
const RECORD: usize = 12;
/// a mark on any box
const ANY_BOX: u32 = 0xffff;
/// marks sit in the surface they lie on (the header's fourth word)
const SIT: u32 = 1;

section! {
    /// what the page is told of the marks' buffer this frame: one row
    pub struct DecalHead {
        /// a word of it moved: send it
        pub moved: u32,
        /// marks in the list, and of them drawn this frame
        pub count: u32,
        pub drawn: u32,
        /// entries of the cells' lists, and left out of them by their caps
        pub listed: u32,
        pub dropped: u32,
    }
}
section! {
    /// who a record is, in the same order: the id of what made it (its tick; the arm in the top byte and its place among that tick's)
    pub struct DecalKey {
        pub tick: u32,
        pub made: u32,
    }
}

#[derive(Default)]
pub struct Marks {
    /// the buffer: a word a row
    pub words: Vec<Word>,
    pub keys: Vec<DecalKey>,
    pub head: Vec<DecalHead>,
    /// the records as they were last laid in the buffer
    was: Vec<[u32; RECORD]>,
    now: Vec<[u32; RECORD]>,
    /// the frame's own boxes by id: its place in the list, and in the frame's own
    own: Vec<(AnyBoxId, u32)>,
    cells: Vec<Vec<u16>>,
    fresh: bool,
}

/// the three rows of a mark, as it stands now: none where the box it rides is not drawn
fn record(g: &Game, d: &Mark, b: &Boxes, own: &[(AnyBoxId, u32)]) -> ([u32; RECORD], bool) {
    let (mut pos, mut n, mut t, mut radius, mut on) = (d.pos, d.n, d.t, d.radius, ANY_BOX);
    if let Some(p) = d.pane { on = p; }
    let mut drawn = true;
    if let Some((id, l)) = &d.rides {
        match own.binary_search_by_key(&AnyBoxId::from(*id), |x| x.0).ok().and_then(|i| b.own_stood(own[i].1 as usize).map(|s| (own[i].1, s))) {
            Some((k, s)) => { pos = to_world(s.c, s.yaw, s.rot, l.p, true); n = to_world(s.c, s.yaw, s.rot, l.n, false); t = to_world(s.c, s.yaw, s.rot, l.t, false); on = b.statics() + k; }
            None => { drawn = false; radius = 0.0; }
        }
    }
    let param = match &d.dries { Some(w) => w.at(g.time), None => d.param };
    let bits = (d.kind as u32 + 1) | (d.rides.is_some() as u32) << 3 | d.seed << 4 | on << 16;
    let f = |x: f64| (x as f32).to_bits();
    ([f(pos[0]), f(pos[1]), f(pos[2]), f(radius), f(n[0]), f(n[1]), f(n[2]), f(param), f(t[0]), f(t[1]), f(t[2]), bits], drawn)
}

impl Marks {
    /// the game is another than the one last drawn: the buffer is sent whatever it holds
    pub fn cut(&mut self) { self.fresh = true; }

    /// A FRAME IS DRAWN, with the boxes where it has them
    pub fn frame(&mut self, g: &Game, b: &Boxes) {
        let empty: &[Mark] = &[];
        let list = mark_set(g).map_or(empty, |s| &s.list[..]);
        let mut head = DecalHead::ZERO;
        head.count = list.len() as u32;
        self.own.clear();
        if list.iter().any(|d| d.rides.is_some()) { self.own.extend(b.own_ids().enumerate().map(|(k, id)| (id, k as u32))); self.own.sort_unstable(); }
        self.now.clear();
        for d in list { let (r, drawn) = record(g, d, b, &self.own); self.now.push(r); head.drawn += drawn as u32; }
        let serial = mark_set(g).map_or(0, |s| s.serial);
        if self.words.len() != TOTAL * 4 { self.words.resize(TOTAL * 4, Word::ZERO); self.cells.resize(CELLS, Vec::new()); self.fresh = true; }
        if self.fresh || self.now != self.was || self.words[2].w != serial {
            self.build(serial, list.len(), &mut head);
            self.keys.clear();
            self.keys.extend(list.iter().map(|d| DecalKey { tick: d.key[0], made: d.key[1] }));
            core::mem::swap(&mut self.now, &mut self.was);
            head.moved = 1;
            self.fresh = false;
        } else { let h = self.head.last(); head.listed = h.map_or(0, |h| h.listed); head.dropped = h.map_or(0, |h| h.dropped); }
        self.head.clear();
        self.head.push(head);
    }

    /// bucket every drawn mark into the metres its bounding circle overlaps (a cell's newest, and so many in all: the rest left out from the old end), then the records
    fn build(&mut self, serial: u32, n: usize, head: &mut DecalHead) {
        let wd = world();
        let (w, h, cs) = (wd.grid_w as i32, wd.grid_h as i32, wd.grid_cell);
        for l in self.cells.iter_mut() { l.clear(); }
        for (i, r) in self.now.iter().enumerate() {
            let f = |k: usize| f32::from_bits(r[k]) as f64;
            if f(3) == 0.0 { continue; }
            let reach = f(3) + 0.04;   // (and the plane's tolerance and the edge's softening round the shape)
            let cell = |v: f64, min: f64| m::floor((v - min) / cs) as i32;
            let (x0, x1) = (cell(f(0) - reach, wd.min[0]).max(0), cell(f(0) + reach, wd.min[0]).min(w - 1));
            let (z0, z1) = (cell(f(2) - reach, wd.min[2]).max(0), cell(f(2) + reach, wd.min[2]).min(h - 1));
            for z in z0..=z1 { for x in x0..=x1 { self.cells[(z * w + x) as usize].push(i as u16); } }
        }
        let mut k = 0usize;
        for (c, l) in self.cells.iter().enumerate() {
            let mut from = l.len().saturating_sub(MOST_A_CELL);
            let mut count = l.len() - from;
            if k + count > LIST_CAP { let room = LIST_CAP.saturating_sub(k); from += count - room; count = room; }
            head.dropped += (l.len() - count) as u32;
            self.words[CELL0 * 4 + c].w = k as u32 | (count as u32) << 16;
            for &i in &l[from..from + count] { self.words[LIST0 * 4 + k].w = i as u32; k += 1; }
        }
        for c in CELLS..CELLS_V4 * 4 { self.words[CELL0 * 4 + c].w = 0; }
        for (i, r) in self.now.iter().enumerate() { for (j, w) in r.iter().enumerate() { self.words[(REC0 + i * 3) * 4 + j].w = *w; } }
        head.listed = k as u32;
        self.words[0].w = n as u32;
        self.words[1].w = k as u32;
        self.words[2].w = serial;
        self.words[3].w = SIT;
    }
}
