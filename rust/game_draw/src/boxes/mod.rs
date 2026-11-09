//! THE WORLD'S BOXES, as the GPU reads them. Statics first (their place in the list is who they are), packed again only where one changed; then the frame's
//! own, which a frame drawn between two ticks stands between where the two left them. Behind the rows: the list of what is drawn, the planes of what is cut,
//! the table of painted marks, the panes of glass and the grid every ray walks (grid.rs). The page hands each to the GPU as it lies.
use crate::Row;
use alloc::vec::Vec;
use game::game::Game;
use game::ids::*;
use game::scene_boxes::Box;

pub mod glass;
pub mod grid;
pub mod marks;
pub mod pack;
pub use pack::{GLASS, NO_SHADOW};
pub mod rank;

/// no key: the box is another thing each frame
pub const NO_KEY: u32 = 0xffff_ffff;
/// boxes the GPU's tables have room for
pub const MAX: usize = 4096;
/// a move longer than this in one tick is a cut, and is not drawn through (metres)
const SNAP: f64 = 1.5;

section! {
    /// a box's place: cx cy cz, yaw's cosine and sine as halves · hx hy hz, flags
    pub struct Geo {
        pub w: [u32; 8],
    }
}
section! {
    /// its material: colour and cut height · emission and finish · rotation · the colour a secondary ray finds and its marks' gain
    pub struct Mat {
        pub w: [f32; 16],
    }
}
section! {
    /// its planes, where it is cut: eight of n and d, then the core's colour
    pub struct Cut {
        pub w: [f32; 36],
    }
}
section! {
    /// four floats of a table
    pub struct Row4 {
        pub w: [f32; 4],
    }
}
section! {
    /// one word of a list
    pub struct Word {
        pub w: u32,
    }
}
section! {
    /// a door's leaf where the tick left it: what the picture's own plans ask of it (openings, places, what shuts smoke in)
    pub struct Leaf {
        pub c: [f32; 3],
        pub h: [f32; 3],
        pub yaw: f32,
        pub flags: u32,
        pub key: u32,
    }
}
section! {
    /// a line of light (a weapon's laser, a tracer's streak): its box, along its own z, and what it emits
    pub struct Beam {
        pub c: [f32; 3],
        pub h: [f32; 3],
        pub rot: [f32; 4],
        pub emits: [f32; 3],
        /// 0 a mirror shows it, 1 the air does, 2 the air does and it ends on a dot
        pub air: u32,
    }
}
section! {
    /// what the page is told of the boxes' tables this frame: one row
    pub struct Head {
        /// how many boxes, how many of them statics
        pub count: u32,
        pub statics: u32,
        /// the statics' rows that changed since the last frame the host took: from, and to the one after (from = to: none)
        pub from: u32,
        pub to: u32,
        /// plain boxes drawn, and after them in the list the cut ones, the first of which a ray can meet
        pub visible: u32,
        pub visible_cut: u32,
        pub visible_cut_traced: u32,
        /// cut boxes a ray can meet
        pub cut_traced: u32,
        /// the rows of the cut table written this frame: the statics' and the frame's own
        pub cut_from: u32,
        pub cut_to: u32,
        pub cut_from_own: u32,
        pub cut_to_own: u32,
        /// the marking table grew
        pub marks_moved: u32,
        /// the grid is another than the last frame's, and how many items it holds
        pub grid_moved: u32,
        pub items: u32,
        /// the panes' broken cells are to be sent
        pub panes_moved: u32,
        /// the game's own count of what broke of the statics (the fog's thin bits are read off the list of that count)
        pub serial: u32,
        /// which stuffs some box has been, a bit a stuff's number
        pub stuffs: u32,
        /// the boxes a ray's grid reaches past the cells they are registered in (metres)
        pub slack: f32,
        /// keyed boxes found a frame ago, met for the first time, and boxes with no key
        pub matched: u32,
        pub fresh: u32,
        pub unkeyed: u32,
        /// the order the statics are drawn in is of the list as it stood some frames ago
        pub order_stale: u32,
    }
}

/// WHAT THE PAGE SAYS OF ITS LOOK, once and when it changes
#[derive(Clone)]
pub struct Look {
    /// drawn with the fog off: no face as the team last knew it (a viewer's, and no number of a page's look)
    pub bared: bool,
    pub on: bool,
    pub marks: bool,
    pub cut: bool,
    pub round: bool,
    pub motion: bool,
    pub panes: bool,
    /// lines of light: mirrors show them, and the air shows a weapon's
    pub mirrors: bool,
    pub air: bool,
    /// solid glass: the darkest it is sent and its roughness (under 0: as any box)
    pub glass_lum: f64,
    pub glass_rough: f64,
    pub texel: f64,
    /// by stuff: its number in the look's maps, and the roughness its coat lends (under 0: none)
    pub stuff: [u32; STUFFS],
    pub coat: [f64; STUFFS],
}
/// the stuffs a box may say
pub const STUFFS: usize = 32;
/// the numbers of a look
pub const LOOK: usize = 11 + 2 * STUFFS;
impl Look {
    pub fn read(v: &[f64]) -> Look {
        let at = |i: usize| v[11 + i];
        Look {
            bared: false, on: v[0] != 0.0, marks: v[1] != 0.0, cut: v[2] != 0.0, round: v[3] != 0.0, motion: v[4] != 0.0, panes: v[5] != 0.0, mirrors: v[6] != 0.0, air: v[7] != 0.0, glass_lum: v[8], glass_rough: v[9], texel: v[10],
            stuff: core::array::from_fn(|i| at(i) as u32), coat: core::array::from_fn(|i| at(STUFFS + i)),
        }
    }
}
impl Default for Look { fn default() -> Look { Look { bared: false, on: false, marks: true, cut: true, round: true, motion: true, panes: true, mirrors: false, air: false, glass_lum: -1.0, glass_rough: 0.0, texel: 0.025, stuff: [0; STUFFS], coat: [-1.0; STUFFS] } } }

/// what a static's rows were packed of: where it stands, and everything else of it as one word
#[derive(Clone, Copy, PartialEq)]
pub struct Was { c: [f64; 3], h: [f64; 3], yaw: f64, rot: Option<[f64; 4]>, flags: u32, cut_height: f64, rest: u64 }
impl Was {
    pub fn of(b: &Box, look: &Look) -> Was {
        let mut k = 0xcbf2_9ce4_8422_2325u64;
        let mut eat = |v: u64| { k = (k ^ v).wrapping_mul(0x0000_0100_0000_01b3); k ^= k >> 29; };
        for v in pack::sent(b, look).iter().chain(pack::emission(b, look.bared).iter()) { eat(v.to_bits()); }
        eat(pack::finish(b, look).to_bits());
        eat(pack::gain(b, look.bared).to_bits());
        eat(b.owner.to_bits());
        eat(b.stuff.map_or(99, |s| s as u64));
        if let Some(list) = b.marks.as_ref() { eat(list.as_ptr() as u64); eat(list.len() as u64); }
        if let Some(planes) = b.cut.as_ref() { for p in planes { for v in p.n.iter().chain(core::iter::once(&p.d)) { eat(v.to_bits()); } } eat(planes.len() as u64); }
        Was { c: b.c, h: b.h, yaw: b.yaw, rot: b.rot, flags: pack::flags(b), cut_height: b.cut_height, rest: k }
    }
    /// the same box as far as where it stands and how far it is drawn
    fn stands(&self, o: &Was) -> bool {
        const DRAWN: u32 = pack::NO_PRIMARY | pack::GLASS | pack::CUTAWAY;
        self.c == o.c && self.h == o.h && self.yaw == o.yaw && self.rot == o.rot && self.cut_height == o.cut_height && (self.flags ^ o.flags) & DRAWN == 0
    }
}

/// one of the frame's own boxes as a tick left it: what a frame between two ticks needs of it
#[derive(Clone, Copy)]
struct Own { id: AnyBoxId, key: u32, c: [f64; 3], h: [f64; 3], yaw: f64, rot: Option<[f64; 4]>, owner: f64, flags: u32, round: bool, cut: bool }

#[derive(Default)]
pub struct Boxes {
    pub look: Look,
    epoch: u32,
    packed: u32,
    was: Vec<Was>,
    pub geo: Vec<Geo>,
    pub mat: Vec<Mat>,
    pub cut: Vec<Cut>,
    pub visible: Vec<Word>,
    pub head: Vec<Head>,
    pub marks: marks::Marks,
    pub rank: rank::Rank,
    pub grid: grid::Grid,
    pub glass: glass::Glass,
    pub leaves: Vec<Leaf>,
    pub beams: Vec<Beam>,
    pub wet: crate::wet::Wet,
    /// the frame's own that are drawn as light and not as boxes
    unboxed: Vec<u32>,
    /// every box as it is drawn this frame, the statics first
    stood: Vec<grid::Stood>,
    own: Vec<Own>,
    /// the tick before: its own boxes by key
    before: Vec<(u32, Own)>,
    have: u32,
    /// the last frame drawn: where in the list each key was
    drawn: Vec<(u32, u32)>,
    cut_static: (u32, u32),
    stuffs: u32,
    moved: (u32, u32),
    undrawn: Vec<u32>,
}

impl Boxes {
    pub fn set_look(&mut self, l: Look) {
        if l.texel != self.look.texel { self.marks.texel = l.texel; self.marks.again(); }
        self.look = Look { bared: self.look.bared, ..l };
        self.epoch += 1;
    }
    /// …with the fog off, or on again
    pub fn bare(&mut self, on: bool) { if on != self.look.bared { self.look.bared = on; self.epoch += 1; } }
    /// nothing is drawn through what went before: the game is another than the one last drawn
    pub fn hold(&mut self) { self.have = self.have.min(1); }
    pub fn cut(&mut self) { self.have = 0; self.drawn.clear(); self.glass.cut(); }

    /// A TICK HAS RUN: the statics that changed are packed again, and the frame's own boxes are packed where the tick left them (`again`: the tick already taken)
    pub fn take(&mut self, g: &Game, night: bool, again: bool, watched: &[crate::watched::Watched]) {
        let w = &g.engine.world;
        let (st, all) = (&w.statics, &g.tab.boxes);
        if self.marks.texel == 0.0 { self.marks.texel = self.look.texel; }
        if self.marks.lit(&g.m.scene_markings.area_lit_marks) { self.epoch += 1; }
        let n = st.len().min(MAX);
        self.stood.truncate(self.was.len());
        let whole = self.packed != self.epoch || self.was.len() != n;
        if self.was.len() != n { self.rank.stale(); }
        self.packed = self.epoch;
        let total = (n + w.dynamics.len()).min(MAX);
        self.geo.resize(total, Geo::ZERO);
        self.mat.resize(total, Mat::ZERO);
        self.cut.resize(total, Cut::ZERO);
        let (mut from, mut to) = (n as u32, 0u32);
        let mut cut_at = (u32::MAX, 0u32);
        self.wet.follow(g);
        for i in 0..n {
            let b = if self.wet.is(i as u32) { let was = Was::of(&all[st[i]], &self.look); self.wet.dressed(g, i as u32, &all[st[i]], was) } else { &all[st[i]] };
            let now = Was::of(b, &self.look);
            if !whole && self.was[i] == now { continue; }
            let anew = i >= self.was.len();
            if anew || !self.was[i].stands(&now) { self.rank.stale(); }
            let stood = grid::Stood { c: b.c, h: b.h, yaw: b.yaw, rot: b.rot, flags: now.flags };
            if anew { self.was.push(now); } else { self.was[i] = now; }
            if i < self.stood.len() { self.stood[i] = stood; } else { self.stood.push(stood); }
            pack::pack(b, b.c, b.h, false, &self.look, &mut self.marks, &mut self.geo[i], &mut self.mat[i], &mut self.cut[i]);
            if pack::cut(b) || pack::round(b) { cut_at = (cut_at.0.min(i as u32), i as u32 + 1); }
            let s = pack::stuff(b.stuff, &self.look);
            if s > 0 { self.stuffs |= 1 << s; }
            from = from.min(i as u32);
            to = i as u32 + 1;
        }
        self.was.truncate(n);
        // (what changed is told to the host until it has taken it)
        if to > from { self.moved = if self.moved.1 > self.moved.0 { (self.moved.0.min(from), self.moved.1.max(to)) } else { (from, to) }; self.grid.statics_moved(); }
        if cut_at.0 != u32::MAX { self.cut_static = if self.cut_static.1 > self.cut_static.0 { (self.cut_static.0.min(cut_at.0), self.cut_static.1.max(cut_at.1)) } else { cut_at }; }
        self.undrawn.clear();
        for i in g.destr.walls.skins.out.iter() { self.undrawn.push(i.0 as u32); }
        self.undrawn.sort_unstable();

        self.stood.truncate(n);

        // ---- the frame's own
        if !again {
            self.before.clear();
            for o in &self.own { if o.key != NO_KEY { self.before.push((o.key, *o)); } }
            self.before.sort_unstable_by_key(|x| x.0);
            self.have = (self.have + 1).min(2);
        }
        self.own.clear();
        self.leaves.clear();
        let (ir, thin, px) = (&g.m.irlaser.ir_made, &g.m.thinline.thin, game::game::game_shown_px(g));
        // (a viewer's own picture: the list before the fog cut it, or the infrared lines of a tube the run's picture was not)
        let raw = &g.fog_picture.raw_list;
        let bare = self.look.bared && g.fog_picture.viewer;
        let mut list: Vec<(AnyBoxId, &Box, Option<game::thinline::Thin>)> = (if bare { raw } else { &w.dynamics }).iter().filter(|id| night || !ir.has(**id)).map(|&id| (id, &g.tab[id], thin.get(id).copied())).collect();
        // (a box of no list has a number no box of the game has)
        list.extend(watched.iter().enumerate().filter(|(_, x)| night || !x.line).map(|(k, x)| (AnyBoxId::Frame(game::ids::FrameBoxId(u32::MAX - 1 - k as u32)), &x.b, x.thin)));
        list.truncate(MAX - n);
        let total = n + list.len();
        self.geo.resize(total, Geo::ZERO);
        self.mat.resize(total, Mat::ZERO);
        self.cut.resize(total, Cut::ZERO);
        for (id, b, thin) in list {
            let (c, mut h) = match b.dc { Some(c) => (c, b.dh.unwrap_or(b.h)), None => (b.c, b.h) };
            let i = n + self.own.len();
            // (a thin line is as wide as the shown seat's view has it seen, and dimmer by as much: as thin as it is, it is a crawling row of dashes)
            let wide = thin.map(|t| (t.hw0, t.k * px)).filter(|&(hw0, hw)| hw > hw0);
            if let Some((hw0, hw)) = wide { for x in &mut h { if *x == hw0 { *x = hw; } } }
            pack::pack(b, c, h, true, &self.look, &mut self.marks, &mut self.geo[i], &mut self.mat[i], &mut self.cut[i]);
            if let Some((hw0, hw)) = wide { for k in 4..7 { self.mat[i].w[k] *= (hw0 / hw) as f32; } }
            let s = pack::stuff(b.stuff, &self.look);
            if s > 0 { self.stuffs |= 1 << s; }
            let f = pack::flags(b);
            if pack::key(b) >> 24 == pack::DOOR || f & pack::SOUND_SEAL != 0 { self.leaves.push(Leaf { c: c.map(|x| x as f32), h: h.map(|x| x as f32), yaw: b.yaw as f32, flags: f, key: pack::key(b) }); }
            self.own.push(Own { id, key: pack::key(b), c, h, yaw: b.yaw, rot: b.rot, owner: b.owner, flags: pack::flags(b), round: self.look.round && pack::round(b), cut: self.look.cut && pack::cut(b) });
        }
        self.geo.truncate(n + self.own.len());
        self.mat.truncate(n + self.own.len());
        self.cut.truncate(n + self.own.len());
        self.glass.take(g, n);
    }

    /// how many statics, and the frame's own boxes in their order: a box's place in the list is the one, plus its place in the other
    pub fn statics(&self) -> u32 { self.was.len() as u32 }
    pub fn own_ids(&self) -> impl Iterator<Item = AnyBoxId> + '_ { self.own.iter().map(|o| o.id) }
    /// how the `k`th of the frame's own stands this frame
    pub fn own_stood(&self, k: usize) -> Option<&grid::Stood> { self.stood.get(self.was.len() + k) }

    fn earlier(&self, key: u32) -> Option<&Own> {
        let i = self.before.partition_point(|x| x.0 < key);
        // (two boxes under one key: neither is drawn through)
        if self.before.get(i)?.0 != key || self.before.get(i + 1).map_or(false, |x| x.0 == key) { return None; }
        Some(&self.before[i].1)
    }

    /// A FRAME IS DRAWN `a` of the way from the tick before to the last (1 = where the last left them). `focus` = where the view looks; `drawn` = this is drawn
    pub fn frame(&mut self, g: &Game, a: f64, focus: [f64; 3], lead: Option<&crate::lead::Moved>) {
        let n = self.was.len();
        self.stood.truncate(n);
        for k in 0..self.own.len() {
            let o = self.own[k];
            let (mut c, mut yaw, mut rot) = (o.c, o.yaw, o.rot);
            if a < 1.0 && self.have >= 2 && o.key != NO_KEY {
                if let Some(p) = self.earlier(o.key) {
                    let d = [c[0] - p.c[0], c[1] - p.c[1], c[2] - p.c[2]];
                    // (another box under the key, or one that was moved and did not move)
                    if p.h == o.h && p.owner == o.owner && p.rot.is_some() == o.rot.is_some() && d[0] * d[0] + d[1] * d[1] + d[2] * d[2] <= SNAP * SNAP {
                        c = core::array::from_fn(|i| p.c[i] + d[i] * a);
                        yaw = p.yaw + crate::turn(p.yaw, yaw) * a;
                        if let (Some(q), Some(r)) = (p.rot, o.rot) { rot = Some(crate::between(q, r, a)); }
                    }
                }
            }
            if let Some(by) = lead { if by.wears(o.key) { c = by.pt(c); yaw += by.turn; rot = rot.map(|q| by.rot(q)); } }
            pack::place(&mut self.geo[n + k], &mut self.mat[n + k], c, yaw, rot);
            self.stood.push(grid::Stood { c, h: o.h, yaw, rot, flags: o.flags });
        }
        self.lines(g, n, focus);
        self.list(g, n);
        let moved = self.grid.build(&self.stood, n);
        self.glass.frame(g, focus, &mut self.grid.info, self.look.panes);
        let h = self.head.last_mut().unwrap();
        h.grid_moved = moved as u32;
        h.items = self.grid.count as u32;
        h.slack = self.grid.slack as f32;
        h.panes_moved = self.glass.moved as u32;
    }

    /// the lines of light nearest the view: the ones the air shows first, as many as its pass has room for
    fn lines(&mut self, g: &Game, n: usize, focus: [f64; 3]) {
        const MOST: usize = 8;
        self.beams.clear();
        self.unboxed.clear();
        if !(self.look.mirrors || self.look.air) { return; }
        let t = &g.m.thinline;
        let mut found: Vec<(bool, f64, usize, Option<Option<AnyBoxId>>)> = Vec::new();
        for (k, o) in self.own.iter().enumerate() {
            if !t.thin.has(o.id) || o.rot.is_none() { continue; }   // (a line of light lies along its own turn: a charge's firing line is thin, and no light)
            let bead = if self.look.air { t.air.get(o.id).copied() } else { None };
            if bead.is_none() && !self.look.mirrors { continue; }
            let c = self.stood[n + k].c;
            let d: f64 = (0..3).map(|i| (c[i] - focus[i]) * (c[i] - focus[i])).sum();
            found.push((bead.is_none(), d, k, bead));
        }
        found.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.total_cmp(&b.1)));
        for (_, _, k, bead) in found.into_iter().take(MOST) {
            let (s, mat) = (&self.stood[n + k], &self.mat[n + k]);
            let f3 = |v: [f64; 3]| v.map(|x| x as f32);
            let air = match bead { None => 0, Some(None) => 1, Some(Some(_)) => 2 };
            self.beams.push(Beam { c: f3(s.c), h: f3(s.h), rot: [mat.w[8], mat.w[9], mat.w[10], mat.w[11]], emits: [mat.w[4], mat.w[5], mat.w[6]], air });
            if air > 0 { self.unboxed.push((n + k) as u32); }
            if let Some(Some(b)) = bead { if let Some(j) = self.own.iter().position(|o| o.id == b) { self.unboxed.push((n + j) as u32); } }
        }
    }

    /// what is drawn, in the order it is drawn in
    fn list(&mut self, g: &Game, n: usize) {
        let st = &g.engine.world.statics;
        let mut head = Head::ZERO;
        self.visible.clear();
        let (mut front, mut back): (Vec<u32>, Vec<u32>) = (Vec::new(), Vec::new());
        let mut plain: Vec<u32> = Vec::new();
        for i in 0..n {
            let b = &g.tab.boxes[st[i]];
            let f = pack::flags(b);
            let is_cut = self.look.cut && pack::cut(b);
            if is_cut && f & pack::NO_SHADOW == 0 { head.cut_traced += 1; }
            if f & (pack::NO_PRIMARY | pack::GLASS) != 0 || self.undrawn.binary_search(&(i as u32)).is_ok() { continue; }
            // (trim is drawn while the box that says it is)
            if b.piece == Some(true) { if let Some(of) = b.of { if let Some(&o) = st.get(of as usize) { if pack::flags(&g.tab.boxes[o]) & pack::NO_PRIMARY != 0 { continue; } } } }
            if self.look.round && pack::round(b) { back.push(i as u32); }
            else if is_cut { if f & pack::NO_SHADOW != 0 { back.push(i as u32); } else { front.push(i as u32); } }
            else { plain.push(i as u32); }
        }
        head.order_stale = !self.rank.order(&self.was_boxes(g, n), &mut plain) as u32;
        for i in plain { self.visible.push(Word { w: i }); }
        let mut keys: Vec<(u32, u32)> = Vec::with_capacity(self.own.len());
        let (mut own_from, mut own_to) = (u32::MAX, 0u32);
        for (k, o) in self.own.iter().enumerate() {
            let i = (n + k) as u32;
            if o.cut && o.flags & pack::NO_SHADOW == 0 { head.cut_traced += 1; }
            if o.cut || o.round { own_from = own_from.min(i); own_to = i + 1; }
            let keyed = self.look.motion && o.key != NO_KEY;
            if keyed { keys.push((o.key, i)); }
            if o.flags & (pack::NO_PRIMARY | pack::GLASS) != 0 || self.unboxed.contains(&i) { continue; }
            // where this box's transform of the last frame is: 0 by its place, 0xffff met for the first time, else one more than its place then
            let was = if !keyed { head.unkeyed += 1; 0 } else { match self.drawn.binary_search_by_key(&o.key, |x| x.0) { Ok(at) => { head.matched += 1; self.drawn[at].1 + 1 } Err(_) => { head.fresh += 1; 0xffff } } };
            let w = i | (was << 16);
            if o.round { back.push(w); }
            else if o.cut { if o.flags & pack::NO_SHADOW != 0 { back.push(w); } else { front.push(w); } }
            else { self.visible.push(Word { w }); }
        }
        // (two boxes under one key: the last registered is the one found)
        keys.sort_by_key(|x| x.0);
        keys.dedup_by(|later, kept| if later.0 == kept.0 { kept.1 = later.1; true } else { false });
        self.drawn = keys;
        head.visible = self.visible.len() as u32;
        head.visible_cut_traced = front.len() as u32;
        head.visible_cut = (front.len() + back.len()) as u32;
        for w in front.into_iter().chain(back.into_iter().rev()) { self.visible.push(Word { w }); }
        head.count = (n + self.own.len()) as u32;
        head.statics = n as u32;
        (head.from, head.to) = if self.moved.1 > self.moved.0 { self.moved } else { (0, 0) };
        (head.cut_from, head.cut_to) = self.cut_static;
        (head.cut_from_own, head.cut_to_own) = if own_to > 0 { (own_from, own_to) } else { (0, 0) };
        head.marks_moved = self.marks.dirty as u32;
        head.serial = g.engine.world.geom_serial as u32;
        head.stuffs = self.stuffs;
        self.moved = (0, 0);
        self.cut_static = (0, 0);
        self.marks.dirty = false;
        self.head.clear();
        self.head.push(head);
    }
    fn was_boxes<'g>(&self, g: &'g Game, n: usize) -> Vec<&'g Box> { if self.rank.due() { g.engine.world.statics[..n].iter().map(|&b| &g.tab.boxes[b]).collect() } else { Vec::new() } }
}
