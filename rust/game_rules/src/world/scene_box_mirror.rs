pub use ::game_types::scene_box_mirror::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

type SBox = crate::scene_boxes::Box;

// the skeleton's numbers as what they index and test by
const REC_: usize = REC as usize;
const R_COS_: usize = R_COS as usize;
const R_SIN_: usize = R_SIN as usize;
const R_YAW_: usize = R_YAW as usize;
const R_ROT_: usize = R_ROT as usize;
const R_CUT_: usize = R_CUT as usize;
const R_CUTN_: usize = R_CUTN as usize;
const R_KIND_: usize = R_KIND as usize;
const K_ROT_: u32 = K_ROT as u32;
const K_ROUND_: u32 = K_ROUND as u32;
const K_CUT_: u32 = K_CUT as u32;
const K_WALK_: u32 = K_WALK as u32;
const F_GLASS_: u32 = F_GLASS as u32;
const THING_SET_: usize = THING_SET as usize;
const TH_: usize = TH as usize;
const TL_: usize = TL as usize;
const OWE_BOXES_: u32 = OWE_BOXES as u32;
const OWE_GLASS_: u32 = OWE_GLASS as u32;
const OWE_VIEW_: u32 = OWE_VIEW as u32;
const PLANE_: usize = PLANE as usize;
const CUT_ROW_: usize = CUT_ROW as usize;
const MAX_PLANES_: usize = crate::scene_cut::MAX_CUT_PLANES as usize;

/// the table of world `w` under the name rust/sim gives it. Never kept across a call that may ask the module.
#[inline]
fn world(w: usize) -> &'static mut sim::world::World {
    sim::world::world(w)
}
/// the same world, to read
fn world_read(w: usize) -> &'static sim::world::World {
    sim::world::world_read(w)
}

/// A box's row (Box.* in the TypeScript), by its ID. The one blocked step of this file.
fn seam_box(g: &mut Game, b: BoxId) -> &mut SBox { &mut g.tab.boxes[b] }

/// `Object.is`: every double by its bits (a −0 is not a 0), and a NaN is itself
#[inline]
fn is(a: f64, b: f64) -> bool {
    a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan())
}

/// `(b.flags & MIRRORED) >>> 0`
#[inline]
fn flags_kept(flags: f64) -> u32 {
    js::to_u32(flags) & js::to_u32(mirrored())
}

/// `n` as `cut.length < MAX_CUT_PLANES ? cut.length : MAX_CUT_PLANES`
#[inline]
fn planes_of(cut: &[crate::scene_cut::CutPlane]) -> usize {
    if cut.len() < MAX_PLANES_ { cut.len() } else { MAX_PLANES_ }
}

/// A LEAF WAS WRITTEN: said where a door places or resets its leaf and a slab is posed.
pub fn leaf_wrote(g: &mut crate::game::Game) {
    g.m.scene_box_mirror.leaves.n += 1.0;
}

/// A thing of furniture was written, taken out of the level, or the level put back.
pub fn thing_wrote(g: &mut crate::game::Game) {
    g.m.scene_box_mirror.things.n += 1.0;
}

/// The one sentence for a level the game will not load.
pub(super) fn too_big(name: Option<&str>, why: &str) -> String {
    // (an empty name is falsy there: no name)
    match name {
        Some(n) if !n.is_empty() => format!("the level '{}' is too big to load: {}", n, why),
        _ => format!("this level is too big to load: {}", why),
    }
}

/// What kind of row a box is: rotated, round, cut.
fn kind_of(g: &crate::game::Game, b: BoxId, cut_on: bool) -> f64 {
    let (rot, round, has_cut, cut_len) = {
        let x = &g.tab.boxes[b];
        (x.rot.is_some(), x.round == Some(true), x.cut.is_some(), x.cut.as_ref().map_or(0, |c| c.len()))
    };
    let cut = has_cut && (cut_on || crate::scene_boxes::shape_cut(g, b.into())) && cut_len > 0;
    ((if rot { K_ROT_ } else { 0 }) | (if round { K_ROUND_ } else { 0 }) | (if cut { K_CUT_ } else { 0 })) as f64
}

/// A static's kind, with the pane rule's answer for it as it stands.
fn kind_at(g: &mut crate::game::Game, b: BoxId, i: f64, cut_on: bool, walk: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>) -> f64 {
    let flags = seam_box(g, b).flags;
    let glass = (js::to_u32(flags) & F_GLASS_) != 0;
    if let Some(walk) = walk {
        if glass && walk(g, i) {
            return ((kind_of(g, b, cut_on) as u32) | K_WALK_) as f64;
        }
    }
    kind_of(g, b, cut_on)
}

/// kindAt under the rule the copy was handed: `g.col.glassWalkable`, whose one maker is the game's glass.
fn kind_at_copy(g: &mut Game, b: BoxId, i: usize, cut_on: bool) -> f64 {
    if g.col.mirror.walk.is_some() {
        let mut rule = |g: &mut Game, bi: f64| crate::glassbreak::glass_break_walkable(g, bi);
        kind_at(g, b, i as f64, cut_on, Some(&mut rule))
    } else {
        kind_at(g, b, i as f64, cut_on, None)
    }
}

/// is the row at `o` of `rows` (its flags `flag`, its cut planes in `cuts` from `cut0`) box b as it stands.
fn row_same(rows: &[f64], o: usize, flag: u32, cuts: &[f64], cut0: usize, b: &SBox, kind: f64) -> bool {
    let (c, h) = (b.c, b.h);
    if !is(rows[o], c[0]) || !is(rows[o + 1], c[1]) || !is(rows[o + 2], c[2]) || !is(rows[o + 3], h[0]) || !is(rows[o + 4], h[1]) || !is(rows[o + 5], h[2])
        || !is(rows[o + R_YAW_], b.yaw) || flag != flags_kept(b.flags) || rows[o + R_KIND_] != kind
    {
        return false;
    }
    if kind == 0.0 {
        return true;
    }
    let k = kind as u32;
    if k & K_ROT_ != 0 {
        let q = b.rot.unwrap();
        if !is(rows[o + R_ROT_], q[0]) || !is(rows[o + R_ROT_ + 1], q[1]) || !is(rows[o + R_ROT_ + 2], q[2]) || !is(rows[o + R_ROT_ + 3], q[3]) {
            return false;
        }
    }
    if k & K_CUT_ != 0 {
        let cut = b.cut.as_ref().unwrap();
        let n = planes_of(cut);
        if rows[o + R_CUTN_] != n as f64 {
            return false;
        }
        let mut p = cut0 + (rows[o + R_CUT_] * CUT_ROW) as usize;
        for pl in &cut[..n] {
            let v = pl.n;
            if !is(cuts[p], v[0]) || !is(cuts[p + 1], v[1]) || !is(cuts[p + 2], v[2]) || !is(cuts[p + 3], pl.d) {
                return false;
            }
            p += PLANE_;
        }
    }
    true
}

/// the row's own half of `put`: everything but the planes, which stand in another table here.
fn row_put(rows: &mut [f64], o: usize, flag: &mut u32, b: &SBox, kind: f64, cut_row: f64) {
    let (c, h, yaw) = (b.c, b.h, b.yaw);
    rows[o] = c[0];
    rows[o + 1] = c[1];
    rows[o + 2] = c[2];
    rows[o + 3] = h[0];
    rows[o + 4] = h[1];
    rows[o + 5] = h[2];
    rows[o + R_COS_] = crate::m::cos(yaw);
    rows[o + R_SIN_] = crate::m::sin(yaw);
    rows[o + R_YAW_] = yaw;
    rows[o + R_KIND_] = kind;
    *flag = flags_kept(b.flags);
    let k = kind as u32;
    if k & K_ROT_ != 0 {
        let q = b.rot.unwrap();
        rows[o + R_ROT_] = q[0];
        rows[o + R_ROT_ + 1] = q[1];
        rows[o + R_ROT_ + 2] = q[2];
        rows[o + R_ROT_ + 3] = q[3];
    }
    if k & K_CUT_ != 0 {
        rows[o + R_CUT_] = cut_row;
        rows[o + R_CUTN_] = planes_of(b.cut.as_ref().unwrap()) as f64;
    }
}

/// the planes' half of `put`, into the record that starts at `p`.
fn planes_put(cuts: &mut [f64], mut p: usize, b: &SBox) {
    let cut = b.cut.as_ref().unwrap();
    for pl in &cut[..planes_of(cut)] {
        let v = pl.n;
        cuts[p] = v[0];
        cuts[p + 1] = v[1];
        cuts[p + 2] = v[2];
        cuts[p + 3] = pl.d;
        p += PLANE_;
    }
}

/// what a box sends back of the light on it, of 255
fn back_of(b: &SBox) -> u8 { (v3::clamp(crate::hours::lum(b.albedo), 0.0, 1.0) * 255.0) as u8 }
/// static `i`'s row in world `w` is box b: the row of the statics' table, the planes in the cut table.
fn static_same(g: &mut Game, w: usize, i: usize, b: BoxId, kind: f64) -> bool {
    let (o, x) = (world_read(w), seam_box(g, b));
    o.back[i] == back_of(x) && row_same(&o.rec, i * REC_, o.flags[i], &o.cut, 0, x, kind)
}
fn static_put(g: &mut Game, w: usize, i: usize, b: BoxId, kind: f64, cut_row: f64) {
    let (o, x) = (world(w), seam_box(g, b));
    o.wrote = o.wrote.wrapping_add(1);
    o.back[i] = back_of(x);
    row_put(&mut o.rec, i * REC_, &mut o.flags[i], x, kind, cut_row);
    if (kind as u32) & K_CUT_ != 0 {
        planes_put(&mut o.cut, (cut_row * CUT_ROW) as usize, x);
    }
}
/// row `at` of the leaves' table (a list's rows one after another) in world `w`; a leaf's cut record is its own, past the statics'
fn leaf_same(g: &mut Game, w: usize, at: usize, b: BoxId, kind: f64) -> bool {
    let o = world_read(w);
    row_same(&o.leaf, at * REC_, o.leaf_flags[at], &o.cut, 0, seam_box(g, b), kind)
}
fn leaf_put(g: &mut Game, w: usize, at: usize, b: BoxId, kind: f64, cut_row: f64) {
    let (o, x) = (world(w), seam_box(g, b));
    row_put(&mut o.leaf, at * REC_, &mut o.leaf_flags[at], x, kind, cut_row);
    if (kind as u32) & K_CUT_ != 0 {
        planes_put(&mut o.cut, (cut_row * CUT_ROW) as usize, x);
    }
}

/// the rows of boxes a question adds, and their cut planes: as the copy's own rows are written
pub(super) fn rows_of(g: &Game, boxes: &[BoxId]) -> (Vec<f64>, Vec<f64>) {
    let cut_on = true;
    let (mut rows, mut cuts) = (vec![0.0; boxes.len() * REC_], vec![0.0; boxes.len() * CUT_ROW as usize]);
    for (i, &b) in boxes.iter().enumerate() {
        let kind = kind_of(g, b, cut_on);
        let x = &g.tab.boxes[b];
        row_put(&mut rows, i * REC_, &mut 0, x, kind, i as f64);
        if (kind as u32) & K_CUT_ != 0 { planes_put(&mut cuts, i * CUT_ROW as usize, x); }
    }
    (rows, cuts)
}

/// How much each table of the copy holds, by name.
pub fn box_mirror_tables(g: &mut crate::game::Game) -> JsMap<String, f64> {
    if g.col.mirror.caps.boxes == 0.0 {
        box_mirror_boot(g); // (`!this.x && !simWhyNot()`: the module is linked here, and no table of it holds nothing)
    }
    let c = g.col.mirror.caps;
    let mut t = JsMap::new();
    for (k, v) in [("boxes", c.boxes), ("leaves", c.leaves), ("cuts", c.cuts), ("panes", c.panes), ("cells", c.cells), ("items", c.items), ("tilted", c.tilted), ("things", c.things), ("glass", c.glass), ("view", c.view)] {
        t.set(String::from(k), v);
    }
    t
}

/// Reads what each table holds, and where the scratch and the senses' tables are.
fn box_mirror_boot(g: &mut crate::game::Game) {
    let cap = |t: f64| sim::world_cap(t as u32) as f64;
    let c = BoxMirrorCaps {
        boxes: cap(T_REC),
        leaves: cap(T_LEAF),
        cuts: cap(T_CUT),
        panes: cap(T_PANE_T),
        cells: cap(T_START),
        items: cap(T_ITEMS),
        tilted: cap(T_TILTED),
        things: cap(T_THING),
        glass: cap(T_GLASS),
        view: cap(T_VIEW),
    };
    let x = &mut g.col.mirror;
    x.caps = c;
    if c.glass != PANE_CAP || c.view != PANE_CAP {
        panic!("the Rust module holds {} broken panes and a view of {}: src/scene/boxMirror.ts says {} of each", c.glass, c.view, PANE_CAP);
    }
    if c.leaves != crate::props::PROP_PARTS || c.things != crate::props::PROP_CAP {
        panic!("the Rust module's copy lists {} leaves and {} things of furniture: the furniture says {} boxes and {} things", c.leaves, c.things, crate::props::PROP_PARTS, crate::props::PROP_CAP);
    }
    x.scratch_len = sim::sim_scratch_len() as f64;
    x.lamp_cap = sim::light_cap() as f64;
    x.wind_cap = sim::grid_wind_cap() as f64;
    x.nav_cap = sim::nav_cap(0) as f64;
    x.cut_of = vec![-1; c.boxes as usize];
}

/// THE MIRROR KEEPS NO ADDRESS of the module's tables: where a table lies in memory differs by machine and by process, and a state must mean the same on
/// any of them. A reader asks the module (sim::world_addr and its like) when it reads.
/// This game's world in the module. True = the world is newly this game's and nothing in it is ours. HELD hands a process's worlds round its games there;
/// a `Game` here reaches no other game, and `g.sim` is its slot for its life, with as many worlds as slots (world.rs WORLDS, state.rs GAMES).
fn box_mirror_take(g: &mut crate::game::Game) -> bool {
    if g.col.mirror.caps.boxes == 0.0 {
        box_mirror_boot(g);
    }
    if g.col.mirror.slot >= 0.0 {
        return false; // (no other game can have taken it: g.sim)
    }
    let s = g.sim;
    let x = &mut g.col.mirror;
    x.slot = s as f64;
    true
}

/// A TICK OPENS: the copy is made the boxes as they stand.
pub(super) fn box_mirror_open(g: &mut crate::game::Game, boxes: &[BoxId], cells: &[Vec<f64>], tilted_at: &[f64], w: f64, h: f64, cs: f64, leaves: &[BoxId], walk: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, glass: Option<&crate::scene_box_mirror::PaneSource>, lines: bool, rays: bool) {
    g.col.mirror.walk = walk.map(|_| crate::FnField); // (the rule itself is g.col.glass_walkable: kind_at_copy)
    g.col.mirror.glass = glass.cloned();
    let fresh = box_mirror_take(g);
    let (epoch, put_backs) = (g.m.math_exact.maths_epoch, g.m.math_side.put_backs.n);
    // (…or the rows in the module are another game's, or another moment's of this one: whatever they say is compared with nothing)
    let all = fresh || g.col.mirror.epoch != epoch || g.col.mirror.put_backs != put_backs || g.col.mirror.stood == 0 || g.col.mirror.stood != world_read(g.sim).stood;
    g.col.mirror.epoch = epoch;
    g.col.mirror.put_backs = put_backs;
    let s = g.sim;
    if all {
        g.col.mirror.sheets_in = None;
        for r in g.col.mirror.cut_of.iter_mut() {
            *r = -1;
        }
        g.col.mirror.cut_free.clear();
        g.col.mirror.cut_next = 0.0;
        g.col.mirror.boxes = 0.0;
        box_mirror_grid_drop(g);
        g.col.mirror.things_all = true;
        g.col.mirror.snow_len = -1.0;
        let cap = g.col.mirror.caps.boxes as usize;
        for r in world(s).glass_of[..cap].iter_mut() {
            *r = 0;
        }
        g.col.mirror.glass_n = 0.0;
        g.col.mirror.glass_bis.clear();
    }
    if all || g.col.mirror.cells_wrote {
        g.col.mirror.cells_wrote = false;
        box_mirror_grid(g, cells, tilted_at, w, h, boxes.len() as f64);
    }
    box_mirror_statics(g, boxes, all);
    sim::world_shape(s, w as u32, h as u32, cs, boxes.len() as u32, g.col.mirror.n_tilted as u32);
    // (cells and tiltedAt are held by reference there and read at `again` as they then stand: here they are read from g.col at `again`, and only the numbers kept)
    g.col.mirror.grid_of = Some(BoxMirrorGridOf { w, h, cs });
    box_mirror_panes(g);
    let x = &mut g.col.mirror;
    x.own_at = -1.0;
    x.things_at = -1.0;
    x.own = Some(leaves.to_vec());
    x.owes = false;
    x.owed = 0.0;
    x.row_lines = lines;
    x.row_rays = rays;
    x.live = true;
    x.lines = lines;
    x.rays = rays;
    copied(g);
}

/// something of the module's rows was written by this game: they are its, of this moment
fn copied(g: &mut Game) {
    let o = world(g.sim);
    o.stood = o.stood.wrapping_add(1).max(1);
    g.col.mirror.stood = o.stood;
}
/// does the copy stand, the game's own and of this moment, with no debt to pay before a question may read it
pub(super) fn box_mirror_stands(g: &Game) -> Result<(), &'static str> {
    let x = &g.col.mirror;
    if !x.live { return Err("no copy of the boxes stands"); }
    if x.stood == 0 || x.stood != world_read(g.sim).stood { return Err("the copy is another game's, or another moment's of this one"); }
    if x.owes && (x.owed != 0.0 || x.glass_busy == 0.0) { return Err("a static or a pane was written and the copy has not been compared since"); }
    if x.own.as_deref() != Some(&g.col.dynamic_boxes[..]) || x.own_at != g.m.scene_box_mirror.leaves.n || x.own_len != g.col.dynamic_boxes.len() as f64 { return Err("a leaf was written and the copy's leaves are as they were"); }
    Ok(())
}

/// The copy is put down.
pub(super) fn box_mirror_shut(g: &mut crate::game::Game) {
    let x = &mut g.col.mirror;
    x.block = None;
    if x.live && x.slot >= 0.0 {
        sim::panes_world(x.slot as usize, -1);
    }
    x.live = false;
    x.lines = false;
    x.rays = false;
    x.owes = false;
    x.owed = 0.0;
    x.own = None;
    x.walk = None;
    x.glass = None;
    x.grid_of = None;
}

/// A table too small for the box world as it stands: a throw by name.
fn box_mirror_outgrown(g: &crate::game::Game, what: &str, has: f64, cap: f64) {
    let of = g.col.mirror.of.as_deref();
    if g.col.mirror.played {
        let level = match of {
            Some(n) if !n.is_empty() => format!(" of the level '{}'", n),
            _ => String::new(),
        };
        panic!("the box world{} has outgrown the Rust module's copy of it in play: {} {}, and the copy holds {} (game/wallbreak.ts refuses a piece over the room the level left: something else grew it)", level, has, what, cap);
    }
    panic!("{}", too_big(of, &format!("{} {} ({}: what the Rust module's copy of the box world holds — scene/boxMirror.ts, game/levelfit.ts)", has, what, cap)));
}

/// The grid: cell c's statics in the order they were filed, and the tilted statics' own list.
fn box_mirror_grid(g: &mut crate::game::Game, cells: &[Vec<f64>], tilted_at: &[f64], w: f64, h: f64, boxes: f64) {
    let n = w * h;
    let c = g.col.mirror.caps;
    if n > c.cells {
        box_mirror_outgrown(g, "cells of the floor grid", n, c.cells);
    }
    if tilted_at.len() as f64 > c.tilted * 5.0 {
        box_mirror_outgrown(g, "tilted statics", tilted_at.len() as f64 / 5.0, c.tilted);
    }
    if boxes > c.boxes {
        box_mirror_outgrown(g, "statics", boxes, c.boxes);
    }
    let n = n as usize;
    let mut total = 0usize;
    for i in 0..n {
        total += cells[i].len();
    }
    if total as f64 > c.items {
        box_mirror_outgrown(g, "entries of the grid's cells", total as f64, c.items);
    }
    let o = world(g.sim);
    let mut at = 0usize;
    for i in 0..n {
        let cell = &cells[i];
        o.start[i] = at as u32;
        for k in 0..cell.len() {
            o.items[at] = js::to_u32(cell[k]);
            at += 1;
        }
    }
    o.start[n] = at as u32;
    for i in 0..tilted_at.len() {
        o.tilted[i] = js::to_i32(tilted_at[i]);
    }
    g.col.mirror.n_tilted = tilted_at.len() as f64 / 5.0;
}

/// Every static against its row; a row that differs (or every row, `all`) written again.
fn box_mirror_statics(g: &mut crate::game::Game, boxes: &[BoxId], all: bool) {
    let n = boxes.len();
    let s = g.sim;
    let grid = !all && g.col.mirror.grid_over;
    let was = g.col.mirror.boxes as usize;
    for i in n..was {
        world(s).wrote = world(s).wrote.wrapping_add(1);
        let r = g.col.mirror.cut_of[i];
        if r >= 0 {
            g.col.mirror.cut_free.push(r as f64);
            g.col.mirror.cut_of[i] = -1;
        }
        if grid {
            box_mirror_grid_dirty(g, &world_read(s).rec, (i * REC_) as f64);
        }
    }
    g.col.mirror.boxes = n as f64;
    for i in 0..n {
        let b = boxes[i];
        let kind = kind_at_copy(g, b, i, true);
        if !all && i < was && static_same(g, s, i, b, kind) {
            continue;
        }
        if grid {
            box_mirror_grid_dirty(g, &world_read(s).rec, (i * REC_) as f64);
            let (c, h) = {
                let x = seam_box(g, b);
                (x.c, x.h)
            };
            let e = h[0] + h[1] + h[2];
            sim::grid_dirty(s, c[0] - e, c[2] - e, c[0] + e, c[2] + e);
        }
        let mut row = g.col.mirror.cut_of[i];
        if (kind as u32) & K_CUT_ != 0 {
            if row < 0 {
                let x = &mut g.col.mirror;
                row = if let Some(r) = x.cut_free.pop() {
                    r as i32
                } else if x.cut_next < x.caps.cuts {
                    x.cut_next += 1.0;
                    (x.cut_next - 1.0) as i32
                } else {
                    -1
                };
                if row < 0 {
                    let cap = g.col.mirror.caps.cuts;
                    box_mirror_outgrown(g, "cut and shaped statics", cap + 1.0, cap);
                }
                g.col.mirror.cut_of[i] = row;
            }
        } else if row >= 0 {
            g.col.mirror.cut_free.push(row as f64);
            g.col.mirror.cut_of[i] = -1;
        }
        static_put(g, s, i, b, kind, row as f64);
    }
}

/// The plan's rectangle round row `o` as the module has it, said to the smoke's grid.
fn box_mirror_grid_dirty(g: &mut crate::game::Game, m: &[f64], o: f64) {
    let o = o as usize;
    let e = m[o + 3] + m[o + 4] + m[o + 5];
    sim::grid_dirty(g.sim, m[o] - e, m[o + 2] - e, m[o] + e, m[o + 2] + e);
}

/// The copy's number, for a step of a smoke grid over it: from here a row that is written is said to the grid.
pub(super) fn box_mirror_grid_copy(g: &mut crate::game::Game) -> f64 {
    g.col.mirror.grid_over = true;
    g.col.mirror.copy
}

/// This is another copy to a smoke grid.
fn box_mirror_grid_drop(g: &mut crate::game::Game) {
    g.m.scene_box_mirror.copies.n += 1.0;
    g.col.mirror.copy = g.m.scene_box_mirror.copies.n;
    g.col.mirror.grid_over = false;
}

/// A static was written inside the tick: the next question compares them again.
pub(super) fn box_mirror_wrote(g: &mut crate::game::Game) {
    if g.col.mirror.live {
        box_mirror_debt(g, OWE_BOXES);
    }
}

/// The next question owes this before the module may answer it.
fn box_mirror_debt(g: &mut crate::game::Game, what: f64) {
    let x = &mut g.col.mirror;
    x.owed = ((x.owed as u32) | (what as u32)) as f64;
    if !x.owes {
        x.owes = true;
        x.lines = false;
        x.rays = false;
    }
}

/// The glass was written with no question of its own between.
pub(super) fn box_mirror_glass_wrote(g: &mut crate::game::Game) {
    if g.col.mirror.live {
        box_mirror_debt(g, OWE_GLASS);
    }
}

/// The glass itself was hung, or taken down, under a standing copy.
pub(super) fn box_mirror_glass_is(g: &mut crate::game::Game, g_2: Option<&crate::scene_box_mirror::PaneSource>) {
    if !g.col.mirror.live {
        return;
    }
    g.col.mirror.glass = g_2.cloned();
    box_mirror_debt(g, OWE_GLASS);
}

/// A static's write is still owed.
fn box_mirror_owes_boxes(g: &crate::game::Game) -> bool {
    ((g.col.mirror.owed as u32) & OWE_BOXES_) != 0
}

/// The level's snow, a byte a static: compared with the table and written where it differs.
pub fn box_mirror_snow(g: &mut crate::game::Game, kinds: &[u8]) {
    let cap = g.col.mirror.caps.boxes as usize;
    let n = if kinds.len() < cap { kinds.len() } else { cap };
    let o = world(g.sim);
    for i in 0..n {
        if o.snow[i] != kinds[i] {
            o.snow[i] = kinds[i];
        }
    }
    if g.col.mirror.snow_len != n as f64 {
        for b in o.snow[n..cap].iter_mut() {
            *b = 0;
        }
        g.col.mirror.snow_len = n as f64;
    }
}

/// The compare a question owes after `wrote`: the copy stands again.
pub(super) fn box_mirror_again(g: &mut crate::game::Game, boxes: &[BoxId], walk: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>) {
    let owed = (g.col.mirror.owed as u32) | (if g.col.mirror.glass_busy > 0.0 { OWE_GLASS_ } else { 0 });
    g.col.mirror.owes = false;
    g.col.mirror.owed = 0.0;
    if !g.col.mirror.live {
        return;
    }
    g.col.mirror.walk = walk.map(|_| crate::FnField);
    if owed & OWE_BOXES_ != 0 {
        let (w, h, cs) = {
            let o = g.col.mirror.grid_of.as_ref().unwrap();
            (o.w, o.h, o.cs)
        };
        if g.col.mirror.cells_wrote {
            g.col.mirror.cells_wrote = false;
            // (the box world's own arrays, as they stand now, as the references held there read; lent for the call: the grid's writer reads neither through g)
            let cells = core::mem::take(&mut g.col.cells);
            let tilted = core::mem::take(&mut g.col.tilted_at);
            box_mirror_grid(g, &cells, &tilted, w, h, boxes.len() as f64);
            g.col.cells = cells;
            g.col.tilted_at = tilted;
        }
        box_mirror_statics(g, boxes, false);
        sim::world_shape(g.sim, w as u32, h as u32, cs, boxes.len() as u32, g.col.mirror.n_tilted as u32);
    }
    if owed & OWE_GLASS_ != 0 {
        box_mirror_panes(g);
    } else if owed & OWE_VIEW_ != 0 {
        box_mirror_view_in(g);
    }
    copied(g);
    let x = &mut g.col.mirror;
    if x.glass_busy > 0.0 {
        x.owes = true;
    }
    x.lines = x.row_lines;
    x.rays = x.row_rays;
}

/// The grid was written: the same debt, and the compare that pays it copies the cells again first.
pub(super) fn box_mirror_wrote_grid(g: &mut crate::game::Game) {
    g.col.mirror.cells_wrote = true;
    box_mirror_wrote(g);
}

/// The panes' rows for the copy's lines and rays: they stand in the game's block, the rule's own; nothing is copied. With no glass hung, no row stands.
fn box_mirror_panes(g: &mut crate::game::Game) {
    let s = g.sim;
    {
        let o = world(s);
        let x = &g.col.mirror;
        for r in 0..x.glass_n as usize {
            o.glass_of[x.glass_bis[r] as usize] = 0;
        }
    }
    // (`G.home`, `G.sheets` and `G.view` are live getters of the game's glass there: the held source says only that it is hung)
    let home = if g.col.mirror.glass.is_some() { crate::glassbreak::glass_break_home() } else { None };
    if home.is_some() {
        panes_home(g, home);
        let sh = crate::glassbreak::glass_break_sheets(g);
        sheets_written(g, sh);
        box_mirror_view_in(g);
        return;
    }
    sim::panes_world(s, -1);
    let x = &mut g.col.mirror;
    x.block = None;
    x.glass_angles = false;
    x.glass_bis.clear();
    x.glass_n = 0.0;
    sim::world_glass(s, 0);
    box_mirror_view_in(g);
}

/// the copy's lines and rays read a pane in this game's block: its slot is `g.sim`, for the game's life.
fn panes_home(g: &mut Game, home: Option<crate::Opaque>) {
    let s = g.sim;
    let x = &mut g.col.mirror;
    x.glass_n = 0.0;
    x.glass_bis.clear();
    x.glass_angles = true;
    sim::world_glass(s, 0);
    sim::panes_world(s, s as i32);
    x.block = home;
}

/// the level's sheets, written unless the copy holds them (`sh !== this.sheetsIn`: by every double's bits here).
fn sheets_written(g: &mut Game, sh: Vec<f64>) {
    let held = match g.col.mirror.sheets_in.as_deref() {
        Some(o) => o.len() == sh.len() && o.iter().zip(sh.iter()).all(|(a, b)| is(*a, *b)),
        None => false,
    };
    if held {
        return;
    }
    let m = unsafe { core::slice::from_raw_parts_mut(sim::sim_scratch(), sim::sim_scratch_len()) };
    m[..sh.len()].copy_from_slice(&sh);
    sim::world_sheets(g.sim, sh.len() / 3);
    g.col.mirror.sheets_in = Some(sh);
}

/// A burst's view was put on or taken off: the next question owes its rows written.
pub(super) fn box_mirror_viewed(g: &mut crate::game::Game) {
    if g.col.mirror.live {
        box_mirror_debt(g, OWE_VIEW);
    }
}

/// The view in hand, its numbers said to the module: a line or a ray there reads each pane's row where it stands.
fn box_mirror_view_in(g: &mut crate::game::Game) {
    let v = if g.col.mirror.glass.is_some() { crate::glassbreak::glass_break_view(g) } else { None };
    g.col.mirror.view_lengths = false;
    let s = g.sim;
    let v = match v {
        None => {
            sim::world_view(s, VIEW_NONE as u32, 0);
            return;
        }
        Some(v) => v,
    };
    let a = &v.at;
    sim::panes_view(s, js::to_u32(v.kind), a[0], a[1], a[2], a[3], a[4], a[5]);
}

/// May a pane's open question be asked of the module now.
pub(super) fn box_mirror_glass_in(g: &mut crate::game::Game) -> bool {
    g.col.mirror.glass.is_some()
}

/// Does no answer of it need an angle or a length worked out.
fn box_mirror_glass_plain(g: &crate::game::Game) -> bool {
    !g.col.mirror.glass_angles && !g.col.mirror.view_lengths
}

/// THE CHECK'S QUESTION of the panes' copy: how many of its rows are not the game's glass now. With glass hung there is no copy: the row is the state.
fn box_mirror_stale_panes(g: &crate::game::Game) -> f64 {
    if g.col.mirror.glass.is_none() {
        return g.col.mirror.glass_n;
    }
    0.0
}

/// THE LEAVES A QUESTION IS ASKED WITH: which of the module's lists holds them. −1 no leaves, −2 more than a list holds.
pub fn box_mirror_leaves(g: &mut crate::game::Game, dyn_: &[BoxId]) -> f64 {
    let n = dyn_.len();
    if n == 0 {
        return -1.0;
    }
    let own = g.col.mirror.own.as_deref() == Some(dyn_); // (`dyn === this.own`: see the notes)
    let writes = g.m.scene_box_mirror.leaves.n;
    if own && g.col.mirror.own_at == writes && g.col.mirror.own_len == n as f64 {
        return 0.0;
    }
    let caps = g.col.mirror.caps;
    if n as f64 > caps.leaves {
        return -2.0;
    }
    let set = if own { 0usize } else { 1usize };
    let base = set * caps.leaves as usize;
    let s = g.sim;
    for i in 0..n {
        let b = dyn_[i];
        let kind = kind_of(g, b, true);
        leaf_put(g, s, base + i, b, kind, caps.cuts + (base + i) as f64);
    }
    sim::world_leaves(s, set, n as u32);
    copied(g);
    if own {
        g.col.mirror.own_at = writes;
        g.col.mirror.own_len = n as f64;
    }
    set as f64
}

/// THE FURNITURE'S PARTS, FOR A QUESTION THAT MEETS THEM: list THING_SET of the leaves, read of the game's things only where one was written since
/// (`thing_wrote`). → does the copy stand.
pub fn box_mirror_things(g: &mut crate::game::Game) -> bool {
    if !g.col.mirror.live {
        return false;
    }
    let writes = g.m.scene_box_mirror.things.n;
    if g.col.mirror.things_at == writes {
        return g.col.mirror.things_ok;
    }
    g.col.mirror.things_at = writes;
    g.col.mirror.things_ok = false;
    let c = g.col.mirror.caps;
    let (all, props) = (g.props.all.clone(), g.props.props.clone());
    let parts: usize = props.iter().map(|&p| g.tab.props_prop[p].boxes.len()).sum();
    if all.len() as f64 > c.things || parts as f64 > c.leaves {
        return false;
    }
    let base = THING_SET_ * c.leaves as usize;
    let every = g.col.mirror.things_all;
    let s = g.sim;
    for (i, &p) in all.iter().enumerate() {
        let row = [g.tab.props_prop[p].x, g.tab.props_prop[p].z, g.tab.props_prop[p].yaw, g.tab.props_prop[p].radius];
        world(s).thing[i * TH_..i * TH_ + 4].copy_from_slice(&row);
    }
    let mut at = 0usize;
    for (r, &p) in props.iter().enumerate() {
        let boxes = g.tab.props_prop[p].boxes.clone();
        world(s).thing_live[r * TL_..r * TL_ + 3].copy_from_slice(&[js::to_u32(g.tab.props_prop[p].id), at as u32, boxes.len() as u32]);
        for b in boxes {
            let kind = kind_of(g, b, true);
            let row = base + at;
            if every || !leaf_same(g, s, row, b, kind) {
                leaf_put(g, s, row, b, kind, c.cuts + row as f64);
            }
            at += 1;
        }
    }
    sim::world_leaves(s, THING_SET_, parts as u32);
    sim::world_things(s, props.len() as u32);
    copied(g);
    g.col.mirror.things_all = false;
    g.col.mirror.things_ok = true;
    true
}

/// Part `at` of the copied list → its box.
pub fn box_mirror_thing_box(g: &crate::game::Game, at: f64) -> Option<BoxId> {
    let mut at = at as usize;
    for &p in &g.props.props {
        let boxes = &g.tab.props_prop[p].boxes;
        if at < boxes.len() {
            return Some(boxes[at]);
        }
        at -= boxes.len();
    }
    None
}

/// May a rule of the module's step over this world now, with the game's own leaves in list 0 as they stand.
pub(super) fn box_mirror_rules(g: &mut crate::game::Game, dyn_: &[BoxId]) -> bool {
    {
        let x = &g.col.mirror;
        if !x.live || (x.owes && (x.owed != 0.0 || x.glass_busy == 0.0)) || x.own.as_deref() != Some(dyn_) {
            return false;
        }
    }
    if !dyn_.is_empty() {
        return box_mirror_leaves(g, dyn_) == 0.0;
    }
    // (a level with no leaf at all: list 0 is the game's own, and empty — said to the module once)
    let writes = g.m.scene_box_mirror.leaves.n;
    if g.col.mirror.own_at != writes || g.col.mirror.own_len != 0.0 {
        sim::world_leaves(g.sim, 0, 0);
        copied(g);
        g.col.mirror.own_at = writes;
        g.col.mirror.own_len = 0.0;
    }
    true
}

/// `b.flags ^= (b.flags & MIRRORED) ^ word`: the kept flags become the word's, an int32 as the operators leave it.
fn flags_from(flags: f64, word: f64) -> f64 {
    let f = js::to_i32(flags);
    (f ^ ((f & js::to_i32(mirrored())) ^ js::to_i32(word))) as f64
}

/// Leaf i of list 0 as the module's rule left it → its box. → the yaw.
pub fn box_mirror_follow_leaf(g: &mut crate::game::Game, i: f64, b: BoxId) -> f64 {
    let r = box_mirror_leaf_row(g, i);
    box_mirror_follow_row(g, b, &r)
}

/// A leaf's row as leafRow read it → its box, the same writes as followLeaf. → the yaw.
pub fn box_mirror_follow_row(g: &mut crate::game::Game, b: BoxId, r: &[f64]) -> f64 {
    let x = seam_box(g, b);
    x.c = [r[0], r[1], r[2]];
    x.yaw = r[3];
    x.flags = flags_from(x.flags, r[4]);
    r[3]
}

/// Leaf i's row of list 0: its centre, its yaw, its flags.
pub fn box_mirror_leaf_row(g: &crate::game::Game, i: f64) -> Vec<f64> {
    let (o, i) = (world_read(g.sim), i as usize);
    let q = i * REC_;
    vec![o.leaf[q], o.leaf[q + 1], o.leaf[q + 2], o.leaf[q + R_YAW_], o.leaf_flags[i] as f64]
}

/// The game's own leaves are being placed box by box while what the rule answered is heard.
pub fn box_mirror_leaves_turned(g: &mut crate::game::Game) {
    g.col.mirror.own_at = -1.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene_cut::CutPlane;

    fn game(slot: usize) -> alloc::boxed::Box<Game> {
        let mut g = alloc::boxed::Box::new(Game::blank());
        g.sim = slot;
        g
    }
    fn a_box() -> SBox {
        let mut b = SBox::default();
        b.c = [1.0, 2.0, 3.0];
        b.h = [0.5, 0.25, 0.125];
        b.yaw = 0.5;
        b.flags = ((1u32 << 17) | 0x0302) as f64;
        b
    }

    #[test]
    fn the_sentence_for_a_level_too_big() {
        assert_eq!(too_big(Some("depot"), "x"), "the level 'depot' is too big to load: x");
        assert_eq!(too_big(Some(""), "x"), "this level is too big to load: x");
        assert_eq!(too_big(None, "x"), "this level is too big to load: x");
    }

    #[test]
    fn a_row_put_is_the_same_until_its_box_moves() {
        let mut b = a_box();
        b.rot = Some([0.0, 0.0, 0.0, 1.0]);
        b.cut = Some(vec![CutPlane { n: [1.0, 0.0, 0.0], d: 0.25 }, CutPlane { n: [0.0, 1.0, 0.0], d: -0.0 }]);
        let kind = (K_ROT_ | K_CUT_) as f64;
        let (mut m, mut fl) = (vec![0.0; 256], vec![0u32; 4]);
        row_put(&mut m, 16, &mut fl[1], &b, kind, 2.0);
        planes_put(&mut m, 64 + 2 * CUT_ROW_, &b);
        let same = |m: &[f64], fl: &[u32], b: &SBox| row_same(m, 16, fl[1], m, 64, b, kind);
        assert!(same(&m, &fl, &b));
        assert_eq!(fl[1], (1 << 17) | 2, "only the flags the copy keeps");
        assert_eq!(m[16 + R_COS_].to_bits(), crate::m::cos(0.5).to_bits());
        assert_eq!(m[16 + R_CUT_], 2.0);
        assert_eq!(m[16 + R_CUTN_], 2.0);
        assert_eq!(m[64 + 2 * CUT_ROW_ + 3], 0.25);
        // a flag the copy does not keep is no news
        b.flags = ((1u32 << 17) | 0x0102) as f64;
        assert!(same(&m, &fl, &b));
        // a −0 is not a 0: a plane's distance by its bits
        b.cut.as_mut().unwrap()[1].d = 0.0;
        assert!(!same(&m, &fl, &b));
        b.cut.as_mut().unwrap()[1].d = -0.0;
        assert!(same(&m, &fl, &b));
        b.c[2] = 3.5;
        assert!(!same(&m, &fl, &b));
        // a NaN is itself
        b.c[2] = f64::NAN;
        m[16 + 2] = -f64::NAN;
        assert!(same(&m, &fl, &b));
    }

    #[test]
    fn a_plain_row_leaves_the_rotation_and_the_record_alone() {
        let b = a_box();
        let (mut m, mut fl) = (vec![7.0; 16], vec![0u32; 1]);
        row_put(&mut m, 0, &mut fl[0], &b, 0.0, -1.0);
        assert_eq!(m[R_ROT_], 7.0);
        assert_eq!(m[R_CUT_], 7.0);
        assert_eq!(m[R_KIND_], 0.0);
    }

    /// the TypeScript's own answers: `followRow` under bun
    #[test]
    fn a_box_follows_its_row_as_an_int32() {
        let f = ((1u32 << 31) | (1 << 17) | 0x12 | 0x300) as f64;
        assert_eq!(flags_from(f, 0x8000000au32 as f64), -2147482854.0);
        assert_eq!(flags_from(0x0302 as f64, 0x00020008u32 as f64), 131848.0);
    }

    #[test]
    fn the_grid_is_cells_one_run_after_another() {
        let mut g = game(1);
        assert!(box_mirror_take(&mut g));
        assert!(!box_mirror_take(&mut g), "the world stays this game's");
        assert_eq!(g.col.mirror.slot, 1.0);
        let cells = [vec![3.0, 1.0], vec![], vec![2.0]];
        box_mirror_grid(&mut g, &cells, &[5.0, 0.0, 1.0, 0.0, 1.0], 3.0, 1.0, 4.0);
        let o = world(1);
        assert_eq!(&o.start[..4], &[0, 2, 2, 3]);
        assert_eq!(&o.items[..3], &[3, 1, 2]);
        assert_eq!(o.tilted[0], 5);
        assert_eq!(g.col.mirror.n_tilted, 1.0);
    }

    #[test]
    fn the_snow_clears_past_its_end_once() {
        let mut g = game(2);
        box_mirror_take(&mut g);
        world(2).snow[5] = 3;
        box_mirror_snow(&mut g, &[1, 2]);
        let o = world(2);
        assert_eq!(&o.snow[..3], &[1, 2, 0]);
        assert_eq!(o.snow[5], 0);
        assert_eq!(g.col.mirror.snow_len, 2.0);
    }

    #[test]
    fn a_debt_stops_the_rows_until_it_is_paid_and_busy_glass_owes_again() {
        let mut g = game(3);
        box_mirror_take(&mut g);
        {
            let x = &mut g.col.mirror;
            x.live = true;
            x.lines = true;
            x.row_lines = true;
        }
        box_mirror_glass_wrote(&mut g);
        assert!(g.col.mirror.owes && !g.col.mirror.lines);
        assert!(!box_mirror_owes_boxes(&mut g));
        box_mirror_again(&mut g, &[], None);
        assert!(!g.col.mirror.owes);
        assert!(g.col.mirror.lines);
        g.col.mirror.glass_busy = 1.0;
        box_mirror_again(&mut g, &[], None);
        assert!(g.col.mirror.owes, "still being worked on: the next question writes its rows again");
        g.col.mirror.glass_busy = 0.0;
        box_mirror_again(&mut g, &[], None);
        assert!(!g.col.mirror.owes);
        // (the debt is not taken with no copy standing, and `shut` forgets what was owed)
        box_mirror_viewed(&mut g);
        assert!(g.col.mirror.owes);
        box_mirror_shut(&mut g);
        assert!(!g.col.mirror.owes && !g.col.mirror.live);
        box_mirror_wrote(&mut g);
        assert!(!g.col.mirror.owes);
        box_mirror_wrote_grid(&mut g);
        assert!(g.col.mirror.cells_wrote && !g.col.mirror.owes);
    }

    /// the panes' rows stand in the block: no copy's row, the world told whose, and told no more as the tick shuts
    #[test]
    fn the_copy_reads_a_pane_in_the_block_until_it_shuts() {
        let mut g = game(0);
        box_mirror_take(&mut g);
        g.col.mirror.live = true;
        g.col.mirror.glass_n = 3.0;
        panes_home(&mut g, Some(crate::Opaque));
        let x = &g.col.mirror;
        assert!(x.block.is_some() && x.glass_angles && x.glass_n == 0.0 && x.glass_bis.is_empty());
        assert!(!box_mirror_glass_plain(&mut g), "an angle may be worked out");
        assert_eq!(world(0).glass_n, 0);
        assert_eq!(world(0).panes_g, 1);
        sheets_written(&mut g, vec![7.0, 1.5, 2.0, 9.0, -0.0, 3.0]);
        assert_eq!(world(0).sheet_n, 2);
        assert_eq!((world(0).sheets[1].bi, world(0).sheets[1].sky), (9, 3));
        // the same list is not written again; one that differs by a sky, or by a −0, is
        world(0).sheet_n = 0;
        sheets_written(&mut g, vec![7.0, 1.5, 2.0, 9.0, -0.0, 3.0]);
        assert_eq!(world(0).sheet_n, 0);
        sheets_written(&mut g, vec![7.0, 1.5, 2.0, 9.0, 0.0, 3.0]);
        assert_eq!(world(0).sheet_n, 2);
        box_mirror_shut(&mut g);
        assert!(g.col.mirror.block.is_none());
        assert_eq!(world(0).panes_g, 0);
        assert_eq!(box_mirror_stale_panes(&mut g), 0.0);
    }

    #[test]
    fn a_table_holds_what_the_module_says() {
        let mut g = game(0);
        let t = box_mirror_tables(&mut g);
        assert_eq!(t.get(String::from("glass")), Some(&PANE_CAP));
        assert_eq!(t.keys().first().map(|k| k.as_str()), Some("boxes"));
    }
}
