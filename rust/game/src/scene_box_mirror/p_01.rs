//! src/scene/boxMirror.ts lines 1 to 505
use super::*;                      // your own module: its types, its other parts' functions
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
const GD_: usize = GD as usize;
const GW_: usize = GW as usize;
const G_SITE_: usize = G_SITE as usize;
const G_SITES_: usize = G_SITES as usize;
const G_DICE_AT_: usize = G_DICE_AT as usize;
const GW_FLAGS_: usize = GW_FLAGS as usize;
const GW_COLS_: usize = GW_COLS as usize;
const GW_ROWS_: usize = GW_ROWS as usize;
const GW_DSEED_: usize = GW_DSEED as usize;
const GW_N_: usize = GW_N as usize;
const GW_SEED_: usize = GW_SEED as usize;
const GW_OPEN_: usize = GW_OPEN as usize;
const GW_GOING_: usize = GW_GOING as usize;
const GW_CRATER_: usize = GW_CRATER as usize;
const GW_BITS_: usize = GW_BITS as usize;
const G_GONE_: u32 = G_GONE as u32;
const G_DICE_: u32 = G_DICE as u32;
const G_RUN_: u32 = G_RUN as u32;
const VD_: usize = VD as usize;
const FRAGS_PER_SITE_: usize = FRAGS_PER_SITE as usize;
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

/// WHICH OF THE MODULE'S WORLDS IS THIS GAME'S. The TypeScript's HELD hands a process's worlds round its games by who stepped longest ago, and writes the game it
/// took one from; a `Game` here reaches no other game. `g.sim` is the game's slot for its life and there are as many worlds as slots (world.rs WORLDS, state.rs GAMES).
#[inline]
fn seam_world_slot(g: &Game) -> usize {
    g.sim
}

/// a box by its ID. ids.txt says BoxId has no table (`g.none`): the one `Tab<BoxId, Box>` the skeleton has is `g.col.boxes`.
#[inline]
fn seam_box(g: &Game, b: BoxId) -> &SBox {
    &g.col.boxes[b]
}

/// RUST.sight, light, smoke, sound or paths (:109): `senseRows` has no `g` to read them by, and `g.m.sim_sim.rust` came out an empty struct.
fn seam_rust_row(row: &str) -> bool {
    todo!("needs: g in sense_rows and the rows of RUST on g.m.sim_sim.rust")
}

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

/// A LEAF WAS WRITTEN: said where a door places or resets its leaf and a slab is posed.   (boxMirror.ts leafWrote)
pub fn leaf_wrote(g: &mut crate::game::Game) {
    g.m.scene_box_mirror.leaves.n += 1.0;
}

/// A thing of furniture was written, taken out of the level, or the level put back.   (boxMirror.ts thingWrote)
pub fn thing_wrote(g: &mut crate::game::Game) {
    g.m.scene_box_mirror.things.n += 1.0;
}

/// The one sentence for a level the game will not load.   (boxMirror.ts tooBig)
pub fn too_big(name: Option<&str>, why: &str) -> String {
    // (an empty name is falsy there: no name)
    match name {
        Some(n) if !n.is_empty() => format!("the level '{}' is too big to load: {}", n, why),
        _ => format!("this level is too big to load: {}", why),
    }
}

/// A SNAPSHOT WAS PUT BACK: no copy stands. A `Game` reaches only its own copy.   (boxMirror.ts mirrorsShut)
pub fn mirrors_shut(g: &mut crate::game::Game) {
    box_mirror_shut(g);
}

/// Which of the senses' rows are on now, a bit each.   (boxMirror.ts senseRows)
pub fn sense_rows() -> f64 {
    let s = senses();
    (if seam_rust_row("sight") { s.sight as u32 } else { 0 }
        | if seam_rust_row("light") { s.light as u32 } else { 0 }
        | if seam_rust_row("smoke") { s.smoke as u32 } else { 0 }
        | if seam_rust_row("sound") { s.sound as u32 } else { 0 }
        | if seam_rust_row("paths") { s.paths as u32 } else { 0 }) as f64
}

/// What kind of row a box is: rotated, round, cut.   (boxMirror.ts kindOf)
pub fn kind_of(g: &mut crate::game::Game, b: BoxId, cut_on: bool) -> f64 {
    let (rot, round, has_cut, cut_len) = {
        let x = seam_box(g, b);
        (x.rot.is_some(), x.round == Some(true), x.cut.is_some(), x.cut.as_ref().map_or(0, |c| c.len()))
    };
    let cut = has_cut && (cut_on || crate::scene_boxes::shape_cut(g, b)) && cut_len > 0;
    ((if rot { K_ROT_ } else { 0 }) | (if round { K_ROUND_ } else { 0 }) | (if cut { K_CUT_ } else { 0 })) as f64
}

/// A static's kind, with the pane rule's answer for it as it stands.   (boxMirror.ts kindAt)
pub fn kind_at(g: &mut crate::game::Game, b: BoxId, i: f64, cut_on: bool, walk: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>) -> f64 {
    let glass = (js::to_u32(seam_box(g, b).flags) & F_GLASS_) != 0;
    if let Some(walk) = walk {
        if glass && walk(g, i) {
            return ((kind_of(g, b, cut_on) as u32) | K_WALK_) as f64;
        }
    }
    kind_of(g, b, cut_on)
}

/// kindAt under the rule the copy was handed: `g.col.glassWalkable`, whose one maker is the game's glass (glassbreak.ts:215).
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

/// is row `o` (flags at `fi`) box b as it stands, a memory that holds rows and planes together.   (boxMirror.ts same)
pub fn same(g: &mut crate::game::Game, m: &[f64], fl: &[u32], o: f64, fi: f64, b: BoxId, kind: f64, cut0: f64) -> bool {
    row_same(m, o as usize, fl[fi as usize], m, cut0 as usize, seam_box(g, b), kind)
}

/// write box b into row `o`, its cut record into record `cut_row`, a memory that holds rows and planes together.   (boxMirror.ts put)
pub fn put(g: &mut crate::game::Game, m: &mut Vec<f64>, fl: &mut Vec<u32>, o: f64, fi: f64, b: BoxId, kind: f64, cut0: f64, cut_row: f64) {
    let x = seam_box(g, b);
    row_put(m, o as usize, &mut fl[fi as usize], x, kind, cut_row);
    if (kind as u32) & K_CUT_ != 0 {
        planes_put(m, cut0 as usize + (cut_row * CUT_ROW) as usize, x);
    }
}

/// static `i`'s row in world `w` is box b: the row of the statics' table, the planes in the cut table.
fn static_same(g: &Game, w: usize, i: usize, b: BoxId, kind: f64) -> bool {
    let o = world(w);
    row_same(&o.rec, i * REC_, o.flags[i], &o.cut, 0, seam_box(g, b), kind)
}
fn static_put(g: &Game, w: usize, i: usize, b: BoxId, kind: f64, cut_row: f64) {
    let (o, x) = (world(w), seam_box(g, b));
    row_put(&mut o.rec, i * REC_, &mut o.flags[i], x, kind, cut_row);
    if (kind as u32) & K_CUT_ != 0 {
        planes_put(&mut o.cut, (cut_row * CUT_ROW) as usize, x);
    }
}
/// row `at` of the leaves' table (a list's rows one after another) in world `w`; a leaf's cut record is its own, past the statics'
fn leaf_same(g: &Game, w: usize, at: usize, b: BoxId, kind: f64) -> bool {
    let o = world(w);
    row_same(&o.leaf, at * REC_, o.leaf_flags[at], &o.cut, 0, seam_box(g, b), kind)
}
fn leaf_put(g: &Game, w: usize, at: usize, b: BoxId, kind: f64, cut_row: f64) {
    let (o, x) = (world(w), seam_box(g, b));
    row_put(&mut o.leaf, at * REC_, &mut o.leaf_flags[at], x, kind, cut_row);
    if (kind as u32) & K_CUT_ != 0 {
        planes_put(&mut o.cut, (cut_row * CUT_ROW) as usize, x);
    }
}

/// How much each table of the copy holds, by name.   (boxMirror.ts BoxMirror.tables)
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

/// Reads what each table holds, and where the scratch and the senses' tables are.   (boxMirror.ts BoxMirror.boot)
pub fn box_mirror_boot(g: &mut crate::game::Game) {
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
    x.list_cap = cap(T_LIST);
    if c.glass != PANE_CAP || c.view != PANE_CAP {
        panic!("the Rust module holds {} broken panes and a view of {}: src/scene/boxMirror.ts says {} of each", c.glass, c.view, PANE_CAP);
    }
    if c.leaves != crate::sim_propshome::PROP_PARTS || c.things != crate::sim_propshome::PROP_CAP {
        panic!("the Rust module's copy lists {} leaves and {} things of furniture: src/sim/propshome.ts says {} boxes and {} things", c.leaves, c.things, crate::sim_propshome::PROP_PARTS, crate::sim_propshome::PROP_CAP);
    }
    x.scratch = (sim::sim_scratch() as usize / 8) as f64;
    x.scratch_len = sim::sim_scratch_len() as f64;
    x.lamps = (sim::light_lamps() / 8) as f64;
    x.lamp_cap = sim::light_cap() as f64;
    x.blobs = (sim::smoke_blobs() / 8) as f64;
    x.blob_cap = sim::smoke_cap() as f64;
    x.wind = (sim::grid_wind() as usize / 8) as f64;
    x.wind_cap = sim::grid_wind_cap() as f64;
    x.nav_shut = sim::nav_addr(0) as f64;
    x.nav_cap = sim::nav_cap(0) as f64;
    x.way = (sim::nav_addr(1) / 8) as f64;
    x.nav_want = (sim::nav_addr(2) / 4) as f64;
    x.nav_price = (sim::nav_addr(3) / 8) as f64;
    x.cut_of = vec![-1; c.boxes as usize];
}

/// This game's world in the module. True = the world is newly this game's and nothing in it is ours.   (boxMirror.ts BoxMirror.take)
pub fn box_mirror_take(g: &mut crate::game::Game) -> bool {
    if g.col.mirror.caps.boxes == 0.0 {
        box_mirror_boot(g);
    }
    g.m.scene_box_mirror.clock.n += 1.0;
    g.col.mirror.used = g.m.scene_box_mirror.clock.n;
    if g.col.mirror.slot >= 0.0 {
        return false; // (no other game can have taken it: seam_world_slot)
    }
    let s = seam_world_slot(g);
    let at = |t: f64, by: usize| (sim::world_addr(s, t as u32) / by) as f64;
    let x = &mut g.col.mirror;
    x.slot = s as f64;
    x.rec = at(T_REC, 8);
    x.leaf = at(T_LEAF, 8);
    x.cut = at(T_CUT, 8);
    x.pane_t = at(T_PANE_T, 8);
    x.flags = at(T_FLAGS, 4);
    x.leaf_flags = at(T_LEAF_FLAGS, 4);
    x.start = at(T_START, 4);
    x.items = at(T_ITEMS, 4);
    x.tilted = at(T_TILTED, 4);
    x.pane_bi = at(T_PANE_BI, 4);
    x.pt = at(T_PT, 8);
    x.list = at(T_LIST, 4);
    x.eyes = (sim::fog_eyes(s) / 8) as f64;
    x.thing = at(T_THING, 8);
    x.thing_live = at(T_THING_LIVE, 4);
    x.snow_at0 = at(T_SNOW, 1);
    x.glass_d = at(T_GLASS, 8);
    x.glass_w = at(T_GLASS_W, 4);
    x.glass_of = at(T_GLASS_OF, 4);
    x.view_d = at(T_VIEW, 8);
    x.view_bi = at(T_VIEW_BI, 4);
    true
}

/// A TICK OPENS: the copy is made the boxes as they stand.   (boxMirror.ts BoxMirror.open)
pub fn box_mirror_open(g: &mut crate::game::Game, boxes: &[BoxId], cells: &[Vec<f64>], tilted_at: &[f64], w: f64, h: f64, cs: f64, leaves: &[BoxId], walk: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, glass: Option<&crate::scene_box_mirror::PaneSource>, lines: bool, circles: bool, sight: bool, senses: Option<f64>, rays: Option<bool>) {
    let (senses, rays) = (senses.unwrap_or(0.0), rays.unwrap_or(false));
    g.col.mirror.walk = walk.map(|_| crate::FnField); // (the rule itself is g.col.glass_walkable: kind_at_copy)
    g.col.mirror.glass = glass.cloned();
    let fresh = box_mirror_take(g);
    let (epoch, put_backs) = (g.m.math_exact.maths_epoch, g.m.math_side.put_backs.n);
    let all = fresh || g.col.mirror.epoch != epoch || g.col.mirror.put_backs != put_backs;
    g.col.mirror.epoch = epoch;
    g.col.mirror.put_backs = put_backs;
    let s = seam_world_slot(g);
    if all {
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
    g.col.mirror.grid_of = Some(BoxMirrorGridOf { cells: Vec::new(), tilted_at: Vec::new(), w, h, cs });
    box_mirror_panes(g);
    let x = &mut g.col.mirror;
    x.own_at = -1.0;
    x.things_at = -1.0;
    x.own = Some(leaves.to_vec());
    x.owes = false;
    x.owed = 0.0;
    x.row_lines = lines;
    x.row_rays = rays;
    x.row_circles = circles;
    x.row_sight = sight;
    x.row_senses = senses;
    x.live = true;
    x.lines = lines;
    x.rays = rays;
    x.circles = circles;
    x.sight = sight;
    x.senses = senses;
}

/// The tick is over.   (boxMirror.ts BoxMirror.shut)
pub fn box_mirror_shut(g: &mut crate::game::Game) {
    let x = &mut g.col.mirror;
    x.live = false;
    x.lines = false;
    x.rays = false;
    x.circles = false;
    x.sight = false;
    x.owes = false;
    x.owed = 0.0;
    x.senses = 0.0;
    x.own = None;
    x.walk = None;
    x.glass = None;
    x.grid_of = None;
}

/// A table too small for the box world as it stands: a throw by name.   (boxMirror.ts BoxMirror.outgrown)
pub fn box_mirror_outgrown(g: &mut crate::game::Game, what: &str, has: f64, cap: f64) {
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

/// The grid: cell c's statics in the order they were filed, and the tilted statics' own list.   (boxMirror.ts BoxMirror.grid)
pub fn box_mirror_grid(g: &mut crate::game::Game, cells: &[Vec<f64>], tilted_at: &[f64], w: f64, h: f64, boxes: f64) {
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
    let o = world(seam_world_slot(g));
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

/// Every static against its row; a row that differs (or every row, `all`) written again.   (boxMirror.ts BoxMirror.statics)
pub fn box_mirror_statics(g: &mut crate::game::Game, boxes: &[BoxId], all: bool) {
    let n = boxes.len();
    let cut_on = crate::scene_cut::cut_boxes_on(g);
    let s = seam_world_slot(g);
    let grid = !all && g.col.mirror.grid_over;
    let was = g.col.mirror.boxes as usize;
    for i in n..was {
        let r = g.col.mirror.cut_of[i];
        if r >= 0 {
            g.col.mirror.cut_free.push(r as f64);
            g.col.mirror.cut_of[i] = -1;
        }
        if grid {
            box_mirror_grid_dirty(g, &world(s).rec, (i * REC_) as f64);
        }
    }
    g.col.mirror.boxes = n as f64;
    for i in 0..n {
        let b = boxes[i];
        let kind = kind_at_copy(g, b, i, cut_on);
        if !all && i < was && static_same(g, s, i, b, kind) {
            continue;
        }
        if grid {
            box_mirror_grid_dirty(g, &world(s).rec, (i * REC_) as f64);
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

/// The plan's rectangle round row `o` as the module has it, said to the smoke's grid.   (boxMirror.ts BoxMirror.gridDirty)
pub fn box_mirror_grid_dirty(g: &mut crate::game::Game, m: &[f64], o: f64) {
    let o = o as usize;
    let e = m[o + 3] + m[o + 4] + m[o + 5];
    sim::grid_dirty(seam_world_slot(g), m[o] - e, m[o + 2] - e, m[o] + e, m[o + 2] + e);
}

/// The copy's number, for a step of a smoke grid over it: from here a row that is written is said to the grid.   (boxMirror.ts BoxMirror.gridCopy)
pub fn box_mirror_grid_copy(g: &mut crate::game::Game) -> f64 {
    g.col.mirror.grid_over = true;
    g.col.mirror.copy
}

/// This is another copy to a smoke grid.   (boxMirror.ts BoxMirror.gridDrop)
pub fn box_mirror_grid_drop(g: &mut crate::game::Game) {
    g.m.scene_box_mirror.copies.n += 1.0;
    g.col.mirror.copy = g.m.scene_box_mirror.copies.n;
    g.col.mirror.grid_over = false;
}

/// A static was written inside the tick: the next question compares them again.   (boxMirror.ts BoxMirror.wrote)
pub fn box_mirror_wrote(g: &mut crate::game::Game) {
    if g.col.mirror.live {
        box_mirror_debt(g, OWE_BOXES);
    }
}

/// The next question owes this before the module may answer it.   (boxMirror.ts BoxMirror.debt)
pub fn box_mirror_debt(g: &mut crate::game::Game, what: f64) {
    let x = &mut g.col.mirror;
    x.owed = ((x.owed as u32) | (what as u32)) as f64;
    if !x.owes {
        x.owes = true;
        x.lines = false;
        x.rays = false;
        x.circles = false;
        x.sight = false;
        x.senses = 0.0;
    }
}

/// The glass is being worked on: every question owes the panes' rows until `glass_end`, and the first one after.   (boxMirror.ts BoxMirror.glassBegin)
pub fn box_mirror_glass_begin(g: &mut crate::game::Game) {
    g.col.mirror.glass_busy += 1.0;
    if g.col.mirror.live {
        box_mirror_debt(g, OWE_GLASS);
    }
}

/// (boxMirror.ts BoxMirror.glassEnd)
pub fn box_mirror_glass_end(g: &mut crate::game::Game) {
    g.col.mirror.glass_busy -= 1.0;
    if g.col.mirror.live {
        box_mirror_debt(g, OWE_GLASS);
    }
}

/// The glass was written with no question of its own between.   (boxMirror.ts BoxMirror.glassWrote)
pub fn box_mirror_glass_wrote(g: &mut crate::game::Game) {
    if g.col.mirror.live {
        box_mirror_debt(g, OWE_GLASS);
    }
}

/// The glass itself was hung, or taken down, under a standing copy.   (boxMirror.ts BoxMirror.glassIs)
pub fn box_mirror_glass_is(g: &mut crate::game::Game, g_2: Option<&crate::scene_box_mirror::PaneSource>) {
    if !g.col.mirror.live {
        return;
    }
    g.col.mirror.glass = g_2.cloned();
    box_mirror_debt(g, OWE_GLASS);
}

/// A static's write is still owed.   (boxMirror.ts BoxMirror.owesBoxes)
pub fn box_mirror_owes_boxes(g: &mut crate::game::Game) -> bool {
    ((g.col.mirror.owed as u32) & OWE_BOXES_) != 0
}

/// The level's snow, a byte a static: compared with the table and written where it differs.   (boxMirror.ts BoxMirror.snow)
pub fn box_mirror_snow(g: &mut crate::game::Game, kinds: &[u8]) {
    let cap = g.col.mirror.caps.boxes as usize;
    let n = if kinds.len() < cap { kinds.len() } else { cap };
    let o = world(seam_world_slot(g));
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

/// The compare a question owes after `wrote`: the copy stands again.   (boxMirror.ts BoxMirror.again)
pub fn box_mirror_again(g: &mut crate::game::Game, boxes: &[BoxId], walk: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>) {
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
        sim::world_shape(seam_world_slot(g), w as u32, h as u32, cs, boxes.len() as u32, g.col.mirror.n_tilted as u32);
    }
    if owed & OWE_GLASS_ != 0 {
        box_mirror_panes(g);
    } else if owed & OWE_VIEW_ != 0 {
        box_mirror_view_in(g);
    }
    let x = &mut g.col.mirror;
    if x.glass_busy > 0.0 {
        x.owes = true;
    }
    x.lines = x.row_lines;
    x.rays = x.row_rays;
    x.circles = x.row_circles;
    x.sight = x.row_sight;
    x.senses = x.row_senses;
}

/// The grid was written: the same debt, and the compare that pays it copies the cells again first.   (boxMirror.ts BoxMirror.wroteGrid)
pub fn box_mirror_wrote_grid(g: &mut crate::game::Game) {
    g.col.mirror.cells_wrote = true;
    box_mirror_wrote(g);
}

/// The panes that are no longer whole, written whole: a row each in the order the game hands them over.   (boxMirror.ts BoxMirror.panes)
pub fn box_mirror_panes(g: &mut crate::game::Game) {
    let s = seam_world_slot(g);
    {
        let o = world(s);
        let x = &g.col.mirror;
        for r in 0..x.glass_n as usize {
            o.glass_of[x.glass_bis[r] as usize] = 0;
        }
    }
    let cap = g.col.mirror.caps.glass;
    let mut n = 0.0f64;
    if g.col.mirror.glass.is_some() {
        crate::glassbreak::glass_break_each(g, &mut |_g, p| {
            if n < cap {
                let (o, r) = (world(s), n as usize);
                pane_row(&mut o.glass[r * GD_..(r + 1) * GD_], &mut o.glass_w[r * GW_..(r + 1) * GW_], &p);
            }
            n += 1.0;
        });
    }
    if n > cap {
        box_mirror_outgrown(g, "panes of glass no longer whole", n, cap);
    }
    let n = n as usize;
    let o = world(s);
    let mut angles = false;
    for r in 0..n {
        if o.glass_w[r * GW_ + GW_FLAGS_] & G_GONE_ == 0 {
            angles = true;
            break;
        }
    }
    let x = &mut g.col.mirror;
    x.glass_angles = angles;
    x.glass_bis.clear();
    for r in 0..n {
        let bi = o.glass_w[r * GW_];
        x.glass_bis.push(bi as f64);
        o.glass_of[bi as usize] = r as u32 + 1;
    }
    x.glass_n = n as f64;
    sim::world_glass(s, n as u32);
    box_mirror_view_in(g);
}

/// One pane's row, `m` and `w` being its doubles and its words.
fn pane_row(m: &mut [f64], w: &mut [u32], p: &PaneRow) {
    let sites = &p.sites;
    let ns = if sites.len() < G_SITES_ { sites.len() } else { G_SITES_ };
    let dice = p.dice.as_ref();
    w[0] = js::to_u32(p.bi);
    w[GW_FLAGS_] = (if p.gone { G_GONE_ } else { 0 }) | (if dice.is_some() { G_DICE_ } else { 0 }) | (if p.run { G_RUN_ } else { 0 }) | ((ns as u32) << 8);
    for i in 0..ns {
        let q = i * G_SITE_;
        let s = &sites[i];
        let rho = &s.rho;
        m[q] = s.u;
        m[q + 1] = s.v;
        m[q + 2] = s.rot;
        m[q + 3] = rho[0];
        m[q + 4] = rho[1];
        m[q + 5] = rho[2];
        m[q + 6] = rho[3];
        m[q + 7] = s.rz;
        m[q + 8] = s.merge;
        w[GW_N_ + i] = js::to_u32(s.n);
        w[GW_SEED_ + i] = js::to_u32(s.seed);
    }
    if let Some(d) = dice {
        let q = G_DICE_AT_;
        w[GW_COLS_] = js::to_u32(d.cols);
        w[GW_ROWS_] = js::to_u32(d.rows);
        w[GW_DSEED_] = js::to_u32(d.seed);
        // (the last clump OUT in the order they go: `out` is what the rule answers by, and a clump is marked out before `next` counts it)
        let mut last = d.next as usize;
        while last < d.clumps.len() && d.out.get(d.clumps[last].k as usize) == Some(&1) {
            last += 1;
        }
        m[q] = d.u;
        m[q + 1] = d.v;
        m[q + 2] = d.hu;
        m[q + 3] = d.hv;
        m[q + 4] = if last > 0 { d.clumps[last - 1].key } else { f64::NEG_INFINITY };
        m[q + 5] = if last > 0 { d.clumps[last - 1].k } else { -1.0 };
    }
    // a bit a piece that is out (only the ids of the sites there are can be set), and a bit a piece queued to leave, with a bit a site whose crater is
    let bits = ns * FRAGS_PER_SITE_;
    let mut at = 0usize;
    for k in 0..GW_BITS_ {
        let mut x = 0u32;
        let mut j = 0u32;
        while j < 32 && at < bits {
            if p.open.get(at) == Some(&1) {
                x |= 1u32 << j;
            }
            j += 1;
            at += 1;
        }
        w[GW_OPEN_ + k] = x;
        w[GW_GOING_ + k] = 0;
    }
    let mut crater = 0i32;
    for q in &p.pending {
        let fid = q.fid;
        w[GW_GOING_ + (js::to_u32(fid) >> 5) as usize] |= 1u32 << (js::to_i32(fid) & 31);
        if js::rem(fid, FRAGS_PER_SITE) < SECTORS {
            crater |= 1i32.wrapping_shl(js::to_i32(fid / FRAGS_PER_SITE) as u32);
        }
    }
    w[GW_CRATER_] = crater as u32;
}

/// One pane's row, its doubles from `d` and its words from `o`.   (boxMirror.ts BoxMirror.pane)
pub fn box_mirror_pane(g: &mut crate::game::Game, m: &mut Vec<f64>, w: &mut Vec<u32>, d: f64, o: f64, p: &crate::scene_box_mirror::PaneRow) {
    let (d, o) = (d as usize, o as usize);
    pane_row(&mut m[d..d + GD_], &mut w[o..o + GW_], p);
}

/// A burst's view was put on or taken off: the next question owes its rows written.   (boxMirror.ts BoxMirror.viewed)
pub fn box_mirror_viewed(g: &mut crate::game::Game) {
    if g.col.mirror.live {
        box_mirror_debt(g, OWE_VIEW);
    }
}

/// The view in hand, written.   (boxMirror.ts BoxMirror.viewIn)
pub fn box_mirror_view_in(g: &mut crate::game::Game) {
    let v = if g.col.mirror.glass.is_some() { crate::glassbreak::glass_break_view(g) } else { None };
    g.col.mirror.view_lengths = false;
    let s = seam_world_slot(g);
    let v = match v {
        None => {
            sim::world_view(s, VIEW_NONE as u32, 0);
            return;
        }
        Some(v) => v,
    };
    let n = v.bi.len();
    let cap = g.col.mirror.caps.view;
    if n as f64 > cap {
        box_mirror_outgrown(g, "panes of glass a burst's view names", n as f64, cap);
    }
    let o = world(s);
    for k in 0..n {
        o.view_bi[k] = js::to_u32(v.bi[k]);
    }
    for k in 0..n * VD_ {
        o.view[k] = v.at[k];
        if k % VD_ == VD_ - 1 && v.at[k] >= 0.0 {
            g.col.mirror.view_lengths = true;
        }
    }
    sim::world_view(s, js::to_u32(v.kind), n as u32);
}

/// May a pane's open question be asked of the module now.   (boxMirror.ts BoxMirror.glassIn)
pub fn box_mirror_glass_in(g: &mut crate::game::Game) -> bool {
    g.col.mirror.glass.is_some()
}

/// Does no answer of it need an angle or a length worked out.   (boxMirror.ts BoxMirror.glassPlain)
pub fn box_mirror_glass_plain(g: &mut crate::game::Game) -> bool {
    !g.col.mirror.glass_angles && !g.col.mirror.view_lengths
}

/// THE LEAVES A QUESTION IS ASKED WITH: which of the module's lists holds them. −1 no leaves, −2 more than a list holds.   (boxMirror.ts BoxMirror.leaves)
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
    let cut_on = crate::scene_cut::cut_boxes_on(g);
    let base = set * caps.leaves as usize;
    let s = seam_world_slot(g);
    for i in 0..n {
        let b = dyn_[i];
        let kind = kind_of(g, b, cut_on);
        leaf_put(g, s, base + i, b, kind, caps.cuts + (base + i) as f64);
    }
    sim::world_leaves(s, set, n as u32);
    if own {
        g.col.mirror.own_at = writes;
        g.col.mirror.own_len = n as f64;
    }
    set as f64
}

/// THE FURNITURE'S PARTS, FOR A QUESTION THAT MEETS THEM: list THING_SET of the leaves. → does the copy stand.   (boxMirror.ts BoxMirror.things)
pub fn box_mirror_things(g: &mut crate::game::Game, p: &crate::scene_box_mirror::Furniture) -> bool {
    if !g.col.mirror.live {
        return false;
    }
    let writes = g.m.scene_box_mirror.things.n;
    if g.col.mirror.things_at == writes && g.col.mirror.things_of.is_some() {
        return g.col.mirror.things_ok; // (`thingsOf === P`: a game has one furniture; the marker says it was copied)
    }
    g.col.mirror.things_at = writes;
    g.col.mirror.things_of = Some(Furniture::zeroed());
    g.col.mirror.things_ok = false;
    let c = g.col.mirror.caps;
    if p.all.len() as f64 > c.things {
        return false;
    }
    let mut parts = 0usize;
    for t in &p.props {
        parts += t.boxes.len();
    }
    if parts as f64 > c.leaves {
        return false;
    }
    let cut_on = crate::scene_cut::cut_boxes_on(g);
    let base = THING_SET_ * c.leaves as usize;
    let every = g.col.mirror.things_all;
    let s = seam_world_slot(g);
    {
        let o = world(s);
        for (i, t) in p.all.iter().enumerate() {
            let q = i * TH_;
            o.thing[q] = t.x;
            o.thing[q + 1] = t.z;
            o.thing[q + 2] = t.yaw;
            o.thing[q + 3] = t.radius;
        }
    }
    let mut at = 0usize;
    for (r, t) in p.props.iter().enumerate() {
        {
            let o = world(s);
            let q = r * TL_;
            o.thing_live[q] = js::to_u32(t.id);
            o.thing_live[q + 1] = at as u32;
            o.thing_live[q + 2] = t.boxes.len() as u32;
        }
        for &b in &t.boxes {
            let kind = kind_of(g, b, cut_on);
            let row = base + at;
            if every || !leaf_same(g, s, row, b, kind) {
                leaf_put(g, s, row, b, kind, c.cuts + row as f64);
            }
            at += 1;
        }
    }
    sim::world_leaves(s, THING_SET_, parts as u32);
    sim::world_things(s, p.props.len() as u32);
    g.col.mirror.things_all = false;
    g.col.mirror.things_ok = true;
    true
}

/// Part `at` of the copied list → its box.   (boxMirror.ts BoxMirror.thingBox)
pub fn box_mirror_thing_box(g: &mut crate::game::Game, p: &crate::scene_box_mirror::Furniture, at: f64) -> Option<BoxId> {
    let mut at = at;
    for t in &p.props {
        if at < t.boxes.len() as f64 {
            return Some(t.boxes[at as usize]);
        }
        at -= t.boxes.len() as f64;
    }
    None
}

/// Where the glass the last line crossed is listed: which static.   (boxMirror.ts BoxMirror.glassBi)
pub fn box_mirror_glass_bi(g: &mut crate::game::Game) -> f64 {
    g.col.mirror.pane_bi
}

/// …and how far along the line it is entered.   (boxMirror.ts BoxMirror.glassT)
pub fn box_mirror_glass_t(g: &mut crate::game::Game) -> f64 {
    g.col.mirror.pane_t
}

/// May a rule of the module's step over this world now, with the game's own leaves in list 0 as they stand.   (boxMirror.ts BoxMirror.rules)
pub fn box_mirror_rules(g: &mut crate::game::Game, dyn_: &[BoxId]) -> bool {
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
        sim::world_leaves(seam_world_slot(g), 0, 0);
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

/// Leaf i of list 0 as the module's rule left it → its box. → the yaw.   (boxMirror.ts BoxMirror.followLeaf)
pub fn box_mirror_follow_leaf(g: &mut crate::game::Game, i: f64, b: BoxId) -> f64 {
    let r = box_mirror_leaf_row(g, i);
    box_mirror_follow_row(g, b, &r)
}

/// A leaf's row as leafRow read it → its box, the same writes as followLeaf. → the yaw.   (boxMirror.ts BoxMirror.followRow)
pub fn box_mirror_follow_row(g: &mut crate::game::Game, b: BoxId, r: &[f64]) -> f64 {
    let x = &mut g.col.boxes[b];
    x.c = [r[0], r[1], r[2]];
    x.yaw = r[3];
    x.flags = flags_from(x.flags, r[4]);
    r[3]
}

/// Leaf i's row of list 0: its centre, its yaw, its flags.   (boxMirror.ts BoxMirror.leafRow)
pub fn box_mirror_leaf_row(g: &mut crate::game::Game, i: f64) -> Vec<f64> {
    let (o, i) = (world(seam_world_slot(g)), i as usize);
    let q = i * REC_;
    vec![o.leaf[q], o.leaf[q + 1], o.leaf[q + 2], o.leaf[q + R_YAW_], o.leaf_flags[i] as f64]
}

/// The game's own leaves are being placed box by box while what the rule answered is heard.   (boxMirror.ts BoxMirror.leavesTurned)
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
        let mut b = SBox::zeroed();
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
        let mut g = game(0);
        let mut b = a_box();
        b.rot = Some([0.0, 0.0, 0.0, 1.0]);
        b.cut = Some(vec![CutPlane { n: [1.0, 0.0, 0.0], d: 0.25 }, CutPlane { n: [0.0, 1.0, 0.0], d: -0.0 }]);
        let id = g.col.boxes.push(b);
        let kind = (K_ROT_ | K_CUT_) as f64;
        let (mut m, mut fl) = (vec![0.0; 256], vec![0u32; 4]);
        put(&mut g, &mut m, &mut fl, 16.0, 1.0, id, kind, 64.0, 2.0);
        assert!(same(&mut g, &m, &fl, 16.0, 1.0, id, kind, 64.0));
        assert_eq!(fl[1], (1 << 17) | 2, "only the flags the copy keeps");
        assert_eq!(m[16 + R_COS_].to_bits(), crate::m::cos(0.5).to_bits());
        assert_eq!(m[16 + R_CUT_], 2.0);
        assert_eq!(m[16 + R_CUTN_], 2.0);
        assert_eq!(m[64 + 2 * CUT_ROW_ + 3], 0.25);
        // a flag the copy does not keep is no news
        g.col.boxes[id].flags = ((1u32 << 17) | 0x0102) as f64;
        assert!(same(&mut g, &m, &fl, 16.0, 1.0, id, kind, 64.0));
        // a −0 is not a 0: a plane's distance by its bits
        g.col.boxes[id].cut.as_mut().unwrap()[1].d = 0.0;
        assert!(!same(&mut g, &m, &fl, 16.0, 1.0, id, kind, 64.0));
        g.col.boxes[id].cut.as_mut().unwrap()[1].d = -0.0;
        assert!(same(&mut g, &m, &fl, 16.0, 1.0, id, kind, 64.0));
        g.col.boxes[id].c[2] = 3.5;
        assert!(!same(&mut g, &m, &fl, 16.0, 1.0, id, kind, 64.0));
        // a NaN is itself
        g.col.boxes[id].c[2] = f64::NAN;
        m[16 + 2] = -f64::NAN;
        assert!(same(&mut g, &m, &fl, 16.0, 1.0, id, kind, 64.0));
    }

    #[test]
    fn a_plain_row_leaves_the_rotation_and_the_record_alone() {
        let mut g = game(0);
        let id = g.col.boxes.push(a_box());
        let (mut m, mut fl) = (vec![7.0; 16], vec![0u32; 1]);
        put(&mut g, &mut m, &mut fl, 0.0, 0.0, id, 0.0, 0.0, -1.0);
        assert_eq!(m[R_ROT_], 7.0);
        assert_eq!(m[R_CUT_], 7.0);
        assert_eq!(m[R_KIND_], 0.0);
    }

    /// the TypeScript's own bits: boxMirror.ts `pane` run under bun, EXACT on, over this pane
    #[test]
    fn a_panes_row_has_the_typescripts_bits() {
        let mut g = game(0);
        let mut open = vec![0u8; 672];
        for i in [0usize, 3, 31, 32, 83, 84, 167, 168] {
            open[i] = 1;
        }
        open[200] = 2;
        let mut out = vec![0u8; 4];
        out[2] = 1;
        out[0] = 1;
        let site = |u: f64, v: f64, rot: f64, n: f64, seed: f64, rho: [f64; 4], rz: f64, merge: f64| PaneSite { u, v, rot, n, seed, rho: rho.to_vec(), rz, merge };
        let p = PaneRow {
            bi: 1234.0,
            gone: true,
            run: true,
            sites: vec![site(0.5, -0.25, 1.5, 7.0, 4294967295.0, [1.0, 2.0, 3.0, 4.0], 0.1, 0.2), site(1.0, 2.0, 3.0, 4.0, 5.0, [6.0, 7.0, 8.0, 9.0], 10.0, 11.0)],
            open,
            pending: [31.0, 96.0, 671.0, 95.0, 96.0 + 0.0].iter().map(|&fid| PaneRowPendingItem { fid }).collect(),
            dice: Some(PaneDice {
                u: 1.25,
                v: 2.5,
                cols: 3.0,
                rows: 4.0,
                seed: 99.0,
                hu: 0.5,
                hv: 0.75,
                next: 1.0,
                out,
                clumps: vec![PaneDiceClumpsItem { k: 1.0, key: 0.125 }, PaneDiceClumpsItem { k: 2.0, key: 0.25 }, PaneDiceClumpsItem { k: 0.0, key: 0.5 }, PaneDiceClumpsItem { k: 3.0, key: 0.75 }],
            }),
        };
        let (mut m, mut w) = (vec![0.0; GD_], vec![0u32; GW_]);
        box_mirror_pane(&mut g, &mut m, &mut w, 0.0, 0.0, &p);
        let words: [u32; 64] = [
            0x4d2, 0x207, 3, 4, 0x63, 7, 4, 0, 0, 0, 0, 0, 0, 0xffffffff, 5, 0, 0, 0, 0, 0, 0, 0x80000009, 1, 0x180000, 0, 0, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0x80000000, 0, 0x80000000, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80000000, 2,
        ];
        assert_eq!(&w[..], &words[..]);
        let head: [u64; 18] = [
            0x3fe0000000000000, 0xbfd0000000000000, 0x3ff8000000000000, 0x3ff0000000000000, 0x4000000000000000, 0x4008000000000000, 0x4010000000000000, 0x3fb999999999999a, 0x3fc999999999999a,
            0x3ff0000000000000, 0x4000000000000000, 0x4008000000000000, 0x4018000000000000, 0x401c000000000000, 0x4020000000000000, 0x4022000000000000, 0x4024000000000000, 0x4026000000000000,
        ];
        for (i, &x) in head.iter().enumerate() {
            assert_eq!(m[i].to_bits(), x, "double {}", i);
        }
        let dice: [u64; 6] = [0x3ff4000000000000, 0x4004000000000000, 0x3fe0000000000000, 0x3fe8000000000000, 0x3fe0000000000000, 0];
        for (i, &x) in dice.iter().enumerate() {
            assert_eq!(m[G_DICE_AT_ + i].to_bits(), x, "dice double {}", i);
        }
    }

    /// the TypeScript's own answers: `followRow` under bun
    #[test]
    fn a_box_follows_its_row_as_an_int32() {
        let mut g = game(0);
        let mut b = SBox::zeroed();
        b.flags = ((1u32 << 31) | (1 << 17) | 0x12 | 0x300) as f64;
        let id = g.col.boxes.push(b);
        assert_eq!(box_mirror_follow_row(&mut g, id, &[1.0, 2.0, 3.0, 0.5, 0x8000000au32 as f64]), 0.5);
        assert_eq!(g.col.boxes[id].flags, -2147482854.0);
        assert_eq!(g.col.boxes[id].c, [1.0, 2.0, 3.0]);
        g.col.boxes[id].flags = 0x0302 as f64;
        box_mirror_follow_row(&mut g, id, &[1.0, 2.0, 3.0, 0.5, 0x00020008u32 as f64]);
        assert_eq!(g.col.boxes[id].flags, 131848.0);
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
            x.senses = 16.0;
            x.row_senses = 16.0;
        }
        box_mirror_glass_begin(&mut g);
        assert!(g.col.mirror.owes && !g.col.mirror.lines);
        assert_eq!(g.col.mirror.senses, 0.0);
        assert!(!box_mirror_owes_boxes(&mut g));
        box_mirror_again(&mut g, &[], None);
        assert!(g.col.mirror.owes, "still being worked on: the next question writes its rows again");
        assert!(g.col.mirror.lines);
        assert_eq!(g.col.mirror.senses, 16.0);
        box_mirror_glass_end(&mut g);
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

    #[test]
    fn a_table_holds_what_the_module_says() {
        let mut g = game(0);
        let t = box_mirror_tables(&mut g);
        assert_eq!(t.get(String::from("glass")), Some(&PANE_CAP));
        assert_eq!(t.keys().first().map(|k| k.as_str()), Some("boxes"));
    }

    #[test]
    fn the_copied_list_names_a_part_by_where_it_stands() {
        let mut g = game(0);
        let thing = |ids: &[u32]| FurnitureThing { id: 0.0, x: 0.0, z: 0.0, yaw: 0.0, radius: 0.0, boxes: ids.iter().map(|&i| BoxId(i)).collect() };
        let p = Furniture { all: Vec::new(), props: vec![thing(&[4, 5]), thing(&[]), thing(&[9])] };
        assert_eq!(box_mirror_thing_box(&mut g, &p, 1.0), Some(BoxId(5)));
        assert_eq!(box_mirror_thing_box(&mut g, &p, 2.0), Some(BoxId(9)));
        assert_eq!(box_mirror_thing_box(&mut g, &p, 3.0), None);
    }
}
