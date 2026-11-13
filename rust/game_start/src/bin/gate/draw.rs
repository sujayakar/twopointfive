//! DRAWING CHANGES NOTHING OF THE GAME, AND WHAT CHANGED IS IN THE FRAME'S MARKS. The run played again as a host plays it: drawn while the tick's out list is still there, between two ticks and at the
//! whole, some frames after two or three ticks, its look said twice, and once put back from a moment. Beside it a mirror of the page's buffers, filled ONLY as the page fills them, which must be the
//! module's own tables after every frame, and once a second what a host that has just opened is told.
//!
//! What the page sends only when a mark says so, each with what the mirror does for it:
//!   src/draw/world.ts upload    geo, mat: the statics of `from..to` and the frame's own (mirrored)
//!   src/draw/world.ts upload    marks when `marksMoved`, panes when `panesMoved` (mirrored)
//!   src/draw/world.ts upload    cut: the rows of `cutFrom..cutTo` and `cutFromOwn..cutToOwn` (mirrored; a row is read only for a box whose flags say cut or round)
//!   src/draw/world.ts upload    cells, items when `gridMoved` (mirrored)
//!   src/draw/world.ts upload    `geomSerial`, read by src/picture/rain.ts (when to walk the drip lines again) and render/fogofwar.ts; the module's snow.rs and winds.rs key on the same count. It is
//!                               held to what boxes/mod.rs says of it: a static whose place was sent anew has another serial, or the list has another length
//!   src/draw/marks.ts           the decal words when `moved` (mirrored)
//!   src/draw/smoke.ts           the plan and the bricks where `made` is another (mirrored)
//!   src/draw/sky.ts             the weather's rows where the row says `weather` (mirrored: a game that has told a weather goes on telling one)
//! Once, as the level opens, and so held to nothing here: traffic and the doorways.
//! What it reads every frame or by a value it compares itself, so that no mark can leave it behind, and is not mirrored: visible, info · dust · fog.ts · lights.ts ·
//! men.ts (materials by the row's own words) · rims.ts · winds.ts · sky.ts's sky, haze, gloss, exposure · src/picture/air.ts, rain.ts's roofs, fire.ts (a key of the rows they read).
//! What is told by an id, once, and is no table: events.ts and src/picture/snow's feet, things and blasts. Snow's words (`sayOf`) are read as they lie.
//!
//! A HOST THAT HAS JUST OPENED is told the same as the mirror holds (`differs` against a fresh `Drawn`), but where the order of a table is its history. A fresh host cannot be held to:
//!   marks     a list's rows are dealt as each list is first met, so a box's slot differs (and `HISTORY` masks what says the slot)
//!   decals    a build leaves the rows of a list that was longer behind its end
use crate::rows::{Flags, RUN_SECS, SEED};
use game::handed::PageLook;
use game::rows::words;
use game_draw::boxes::{glass::Pane, Cut, Geo, Mat, Row4, Word};
use game_draw::{want, Drawn, Row};
use game_start::hand::act;
use game::savestate::control::Moment;
use game_start::parts;
use std::time::Instant;

/// the flag words of the GPU that say a cut row is read (rust/game_draw boxes/pack.rs)
use game_draw::boxes::pack::geo_flag::{CUT, ROUND};
/// THE WORDS THAT HOLD A PLACE IN A TABLE WITH A HISTORY, and nothing else, masked for a fresh host: geo word 7 from bit 18 up is the box's first row in the marks' table (boxes/pack.rs `slot << 18`)
const HISTORY: u32 = (1 << 18) - 1;
/// a run put back as a page does: kept at this second and put back at that one, and it goes on from where it was kept
const MOMENT: (usize, usize) = (18, 20);
/// when the look is said a second time
const LOOK_AGAIN: usize = 600;

/// THE PAGE'S LOOK, written out: what src/draw/look.ts `lookNumbers` gives (nothing here asks it again) as src/host/drawpage.ts sets it up as a page opens (COATFINISH and SOLIDGLASS on, its cast, the settings as they stand, the cut boxes
/// on), with the one number a preset decides, the mirrors, as High has it.
const PAGE_LOOK: PageLook = PageLook {
    on: 1.0, marks: 1.0, cut: 1.0, round: 1.0, motion: 1.0, panes: 1.0, mirrors: 1.0, air: 1.0, glass_lum: 0.02, glass_rough: 0.05, texel: 0.025,
    stuff: [26.0, 28.0, 0.0, 10.0, 0.0, 7.0, 13.0, 14.0, 15.0, 25.0, 27.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 9.0, 11.0, 12.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 0.0, 0.0],
    coat: [-1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 0.2, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0],
    swaps: [1.0, 2.0, 1.0, 1.0], wet: 1.0,
};

/// a row as words: two rows are the same where their bits are (a NaN is the same as itself)
fn same<T>(a: &T, b: &T) -> bool { unsafe { std::slice::from_raw_parts(a as *const T as *const u8, size_of::<T>()) == std::slice::from_raw_parts(b as *const T as *const u8, size_of::<T>()) } }

/// one GPU buffer: what was sent stands until something else is, and what was never sent is zeros
struct Buffer<T> { rows: Vec<T> }
impl<T> Default for Buffer<T> { fn default() -> Self { Buffer { rows: Vec::new() } } }
impl<T: Row> Buffer<T> {
    fn send(&mut self, from: &[T], at: usize, count: usize) {
        if self.rows.len() < at + count { self.rows.resize(at + count, T::ZERO); }
        self.rows[at..at + count].copy_from_slice(&from[at..at + count]);
    }
    fn all(&mut self, from: &[T]) { self.send(from, 0, from.len()); }
    /// the first of `of`'s rows that `is_same` says is not what was sent
    fn first_as(&mut self, of: &[T], is_same: impl Fn(&T, &T) -> bool) -> Option<usize> {
        if self.rows.len() < of.len() { self.rows.resize(of.len(), T::ZERO); }
        (0..of.len()).find(|&i| !is_same(&self.rows[i], &of[i]))
    }
    fn first_differ(&mut self, of: &[T]) -> Option<usize> { self.first_as(of, same) }
}

#[derive(Default)]
struct Mirror {
    geo: Buffer<Geo>, mat: Buffer<Mat>, cut: Buffer<Cut>, marks: Buffer<Row4>, panes: Buffer<Pane>, cells: Buffer<Word>, items: Buffer<Word>, decals: Buffer<Word>,
    smoke_at: Buffer<Word>, smoke_bricks: Buffer<game_draw::smoke::SmokeBrick>, smoke_made: Option<u32>, plan: (u32, u32),
    /// the list as last told: how many statics, and the serial
    list: Option<(u32, u32)>,
    /// a static whose place was sent anew under the serial and length that stood
    moved_unsaid: Option<usize>,
    told_weather: bool, carried: usize,
}
impl Mirror {
    /// a frame's rows handed over, as the page does. `skip`: the statics' changed rows are not, on the frames that carry them after the first.
    fn upload(&mut self, d: &Drawn, skip: bool) {
        let h = d.boxes.head[0];
        let (st, n, from, to) = (h.statics as usize, h.count as usize, h.from as usize, h.to as usize);
        if to > from {
            self.carried += 1;
            if self.list == Some((h.statics, h.serial)) && self.moved_unsaid.is_none() { self.moved_unsaid = (from..to).find(|&i| self.geo.rows.get(i).is_some_and(|was| words(was)[..7] != words(&d.boxes.geo[i])[..7])); }
            if !(skip && self.carried > 1) { self.geo.send(&d.boxes.geo, from, to - from); self.mat.send(&d.boxes.mat, from, to - from); }
        }
        self.list = Some((h.statics, h.serial));
        self.geo.send(&d.boxes.geo, st, n - st);
        self.mat.send(&d.boxes.mat, st, n - st);
        if h.marks_moved != 0 { self.marks.all(&d.boxes.marks.rows); }
        if h.panes_moved != 0 { self.panes.all(&d.boxes.glass.cells); }
        for (a, b) in [(h.cut_from, h.cut_to), (h.cut_from_own, h.cut_to_own)] { if b > a { self.cut.send(&d.boxes.cut, a as usize, (b - a) as usize); } }
        if h.grid_moved != 0 { self.cells.all(&d.boxes.grid.cells); self.items.all(&d.boxes.grid.items); }
        if d.marks.head.first().is_some_and(|m| m.moved != 0) { self.decals.all(&d.marks.words); }
        if let Some(s) = d.smoke.head.first() {
            if self.smoke_made != Some(s.made) { self.smoke_made = Some(s.made); self.plan = (s.wide, s.deep); self.smoke_at.all(&d.smoke.at); self.smoke_bricks.all(&d.smoke.bricks); }
        }
        if d.sky.first().is_some_and(|s| s.weather != 0) { self.told_weather = true; }
    }
    /// the first table that is not `d`'s: its name and first row. `opened`: `d` is a host that has just opened, and what is a table's history is left out (the file's head says which)
    fn differs(&mut self, d: &Drawn, opened: bool) -> Option<(&'static str, usize)> {
        let h = d.boxes.head[0];
        if let Some(i) = if opened { self.geo.first_as(&d.boxes.geo, |a, b| words(a)[..7] == words(b)[..7] && a.flags.0 & HISTORY == b.flags.0 & HISTORY) } else { self.geo.first_differ(&d.boxes.geo) } { return Some(("geo", i)); }
        if let Some(i) = self.mat.first_differ(&d.boxes.mat) { return Some(("mat", i)); }
        if self.cut.rows.len() < d.boxes.cut.len() { self.cut.rows.resize(d.boxes.cut.len(), Cut::ZERO); }
        if let Some(i) = (0..h.count as usize).find(|&i| d.boxes.geo[i].flags.0 & (ROUND | CUT) != 0 && !same(&self.cut.rows[i], &d.boxes.cut[i])) { return Some(("cut", i)); }
        if !opened {
            if let Some(i) = self.marks.first_differ(&d.boxes.marks.rows) { return Some(("marks", i)); }
            if let Some(i) = self.decals.first_differ(&d.marks.words) { return Some(("decal words", i)); }
        }
        if let Some(i) = self.panes.first_differ(&d.boxes.glass.cells) { return Some(("panes", i)); }
        if let Some(i) = self.cells.first_differ(&d.boxes.grid.cells) { return Some(("cells", i)); }
        if let Some(i) = self.items.first_differ(&d.boxes.grid.items) { return Some(("items", i)); }
        if let Some(s) = d.smoke.head.first() {
            if self.plan != (s.wide, s.deep) { return Some(("smoke's plan", 0)); }
            if let Some(i) = self.smoke_at.first_differ(&d.smoke.at) { return Some(("smoke's plan", i)); }
            if let Some(i) = self.smoke_bricks.first_differ(&d.smoke.bricks) { return Some(("smoke's bricks", i)); }
        }
        if let Some(i) = self.moved_unsaid.filter(|_| !opened) { return Some(("serial: a static stands elsewhere and the serial is the same, at", i)); }
        if self.told_weather && d.sky.first().is_some_and(|s| s.weather == 0) { return Some(("weather", 0)); }
        None
    }
}

/// what `hold draw` holds of the heads
fn heads(d: &Drawn) -> Result<(), String> {
    let h = d.boxes.head[0];
    if h.count as usize != d.boxes.geo.len() || h.visible as usize + h.visible_cut as usize != d.boxes.visible.len() { return Err(format!("the boxes' head says {} boxes and {} drawn, and their lists hold {} and {}", h.count, h.visible + h.visible_cut, d.boxes.geo.len(), d.boxes.visible.len())); }
    Ok(())
}

/// THE ROW. (a) the folded word every second is the undrawn run's, also after a moment put back. (b) the mirror is the module's tables after every frame. (c) the boxes' head says what their lists hold.
/// (d) once a second, what the mirror holds is what a host that has just opened is told.
/// BREAKS: `told` a die drawn once between the undrawn run and this one · `mark` the mirror skips `from..to` on the frames that carry one after the first · `head` the head made wrong before it is read ·
/// `opened` a word of the fresh host's made wrong before it is read.
pub fn row(name: &str, k: usize, series: &[u32], f: Flags) {
    let t = Instant::now();
    let (mut p, hand) = crate::rows::stand_run(k, SEED);
    if f.breaks("told") { p.g.dice.n += 1.0; }
    let mut d = Drawn::default();
    d.look(&PAGE_LOOK);
    let mut mirror = Mirror::default();
    let (mut a, mut b, mut c, mut e): (Option<String>, Option<String>, Option<String>, Option<String>) = (None, None, None, None);
    let (mut i, mut next, mut over, mut second_due) = (0, 0, false, false);
    let (mut w, mut words) = (game::bytes::W::new(), Vec::new());
    let mut kept = None;
    let mut back = false;
    while i < RUN_SECS * 60 {
        if i == MOMENT.0 * 60 && !back { kept = Some((Moment::of(&p.g), next, p.cursor)); }
        if i == MOMENT.1 * 60 && !back {
            // (as the page's `hold` does: the game whole again of that moment, and nothing drawn through it)
            let (m, n, cursor) = kept.take().expect("the moment was kept");
            p.lay(&m);
            (next, p.cursor, back, i, over) = (n, cursor, true, MOMENT.0 * 60, false);
            d.cut();
        }
        if i == LOOK_AGAIN { d.look(&PAGE_LOOK); }
        while next < hand.at.len() && hand.at[next].0 <= i { let _ = act(&mut p, &hand.at[next].1); next += 1; }
        // (some ticks are drawn by the next frame: two or three of them come before it, as on a host whose frame is long; the first of them is taken, the others over it)
        let seen = i % 5 != 4 && i % 6 != 5;
        if i % 60 == 59 { second_due = true; }
        p.frame_shown(|p| {
            if over { d.over(&p.g, u32::MAX); } else { d.take(&p.g, u32::MAX); }
            if !seen { return; }
            let focus = p.g.sets[p.g.seat].aim_point;
            for at in [0.5, 1.0] {
                d.frame(&p.g, at, focus);
                mirror.upload(&d, f.breaks("mark"));
                if b.is_none() { if let Some((table, r)) = mirror.differs(&d, false) { b = Some(format!("the page's {} parts from the module's at row {}, frame {} (at {}) of the run", table, r, i + 1, at)); } }
                if f.breaks("head") { d.boxes.head[0].count += 1; }
                if c.is_none() { if let Err(why) = heads(&d) { c = Some(format!("frame {}: {}", i + 1, why)); } }
            }
            if !second_due { return; }
            second_due = false;
            let mut fresh = Drawn::default();
            fresh.look(&PAGE_LOOK);
            fresh.take(&p.g, want::boxes | want::smoke | want::sky);
            fresh.frame(&p.g, 1.0, focus);
            if f.breaks("opened") { fresh.boxes.geo[0].c[0] += 1.0; }
            if e.is_none() { if let Some((table, r)) = mirror.differs(&fresh, true) { e = Some(format!("frame {}: {} row {} is another for a host that has just opened", i + 1, table, r)); } }
        });
        over = !seen;
        if i % 60 == 59 {
            parts::words(&mut p.g, &mut w, &mut words);
            if a.is_none() && crate::rows::word_of(&words) != series[i / 60] { a = Some(format!("drawing the game changed it by second {}{}", i / 60 + 1, if back { ", after a moment put back" } else { "" })); }
        }
        i += 1;
    }
    let fail: Vec<String> = [a, b, c, e].into_iter().flatten().collect();
    crate::rows::say(name, t.elapsed().as_secs_f64(), &fail, &format!("statics told again on {} frames, {} statics", mirror.carried, d.boxes.head[0].statics));
}
