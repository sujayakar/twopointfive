//! THE KIT PICTURE'S PLAN: everything the Team & kit picture needs to know about ONE way in, worked out from the level's own data and nothing else, so that the staging can be
//! level-free. A level writes a way's ANCHOR (a `door`, or a `span`: two points on a blank wall to breach or the two posts of an open gate) and whatever it wants to pin; every other
//! value is MEASURED here, the same way for every level. Pure: plain level data in, plain data out; no die, no clock.
//!
//! A LEVEL OR A SET. The plan is made from whatever geometry it is handed and reaches for nothing else: a way that names a bespoke set is planned from the set's own records and its
//! `ref`, with not one record of the level's in reach (`plan_way` is the one place that chooses).
//!
//! AUTHORED KEYS ALWAYS WIN. Every value the plan measures can be written on the way and is then taken as written, key by key, never corrected: level numbers tuned in the viewer are
//! authored data. `src` says, per value, which it was.
//!
//! FREE FLOOR is the nav grid's own question, asked of the file's boxes: a 0.5 m cell is shut when a 0.34 m circle at its middle is pushed more than 5 mm by the boxes that stand between
//! knee and shoulder. WHAT IT CANNOT SEE: loose props, door leaves, anything the game adds or takes at run time, boxes a stand leaves out (hand it the boxes that stand loads), a tilted
//! box beyond its yawed footprint, as the grid itself, and the PICTURE: the camera here is the worked-out pin BEFORE the page's frame slides and scales it, so `occluders` is what stands
//! in THAT line.
//!
//! EVERY NUMBER IS THE TYPESCRIPT'S TO THE BIT (tools/blitz/kitplan-diff.ts): the order of every sum is the original's, do not tidy the arithmetic.
use alloc::borrow::Cow;
use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::cmp::Ordering;
use game::kitstage::{KitPlanNumbers, KIT_PLAN};
pub use game::kitstage::{act4, porch_lit, Anchor, Cam, Fill, Fixture, Lamps, Lane, Occluder, Pane, Planned, Porch, Src, Wall, V2, V3};
use game::m;

use crate::json::{self, Json};

const K: &KitPlanNumbers = &KIT_PLAN;
const RAD: f64 = m::PI / 180.0;
/// how the problem "an occluder that is no thing was worked out" begins (`plan_child` carries the first pass's across by it)
pub const IN_WAY: &str = "in the camera's way and not a thing to sink";

// ---- what the planner reads off the file

/// A box, as far as the plan reads one: its yaw's cosine and sine are kept, for they are asked for thousands of times a plan
#[derive(Clone, Debug)]
pub struct PlanBox { pub id: String, pub group: Option<String>, pub c: V3, pub h: V3, pub cy: f64, pub sy: f64, pub no_shadow: bool, pub glass: bool, pub no_primary: bool }
impl PlanBox {
    pub fn of(b: &Json) -> PlanBox {
        let flag = |f: &str| b.get("flags").items().iter().any(|x| x.str() == Some(f));
        let yaw = b.get("yaw").num().unwrap_or(0.0);
        PlanBox { id: text(b, "id"), group: b.get("group").str().map(String::from), c: b.get("c").v3().expect("kit plan: a box's c"), h: b.get("h").v3().expect("kit plan: a box's h"), cy: m::cos(yaw), sy: m::sin(yaw), no_shadow: flag("NoShadow"), glass: flag("Glass"), no_primary: flag("NoPrimary") }
    }
}
#[derive(Clone, Debug)]
pub struct PlanDoor { pub id: String, pub hinge: V2, pub closed_dir: f64, pub width: f64, pub min_angle: Option<f64>, pub max_angle: Option<f64>, pub swing: Option<f64>, pub steel: bool, pub locked: bool, pub exterior: bool }
impl PlanDoor {
    pub fn of(d: &Json) -> PlanDoor {
        let on = |k: &str| d.get(k).bool() == Some(true);
        PlanDoor { id: text(d, "id"), hinge: v2(d.get("hinge")), closed_dir: d.get("closedDir").num().expect("kit plan: a door's closedDir"), width: d.get("width").num().expect("kit plan: a door's width"), min_angle: d.get("minAngle").num(), max_angle: d.get("maxAngle").num(), swing: d.get("swing").num(), steel: on("steel"), locked: on("locked"), exterior: on("exterior") }
    }
}
#[derive(Clone, Debug)]
pub struct PlanLight { pub id: String, pub dir: bool, pub pos: V3, pub range: f64, pub group: String }
impl PlanLight {
    pub fn of(l: &Json) -> PlanLight {
        PlanLight { id: text(l, "id"), dir: l.get("kind").str() == Some("dir"), pos: l.get("pos").v3().expect("kit plan: a light's pos"), range: l.get("range").num().unwrap_or(m::NAN), group: text(l, "group") }
    }
}
#[derive(Clone, Debug)]
pub struct PlanProp { pub id: String, pub x: f64, pub z: f64 }
/// The level as the planner reads it: the file's own records. `props`: a prop is OF a group when its id is the group's name and a dotted tail. `left`: names of what the stand left out of
/// the file; a `hide` / `show` that names one names nothing here, and is no problem.
#[derive(Clone, Debug, Default)]
pub struct PlanLevel { pub boxes: Vec<PlanBox>, pub doors: Vec<PlanDoor>, pub lights: Vec<PlanLight>, pub props: Vec<PlanProp>, pub left: Vec<String> }
impl PlanLevel {
    /// a level file's records, read once
    pub fn of(data: &Json) -> PlanLevel {
        PlanLevel { boxes: boxes_of(data.get("boxes")), doors: doors_of(data.get("doors")), lights: lights_of(data.get("lights")),
            props: data.get("props").items().iter().map(|p| PlanProp { id: text(p, "id"), x: p.get("x").num().expect("kit plan: a prop's x"), z: p.get("z").num().expect("kit plan: a prop's z") }).collect(), left: Vec::new() }
    }
}
fn boxes_of(j: &Json) -> Vec<PlanBox> { j.items().iter().map(PlanBox::of).collect() }
fn doors_of(j: &Json) -> Vec<PlanDoor> { j.items().iter().map(PlanDoor::of).collect() }
fn lights_of(j: &Json) -> Vec<PlanLight> { j.items().iter().map(PlanLight::of).collect() }
fn text(j: &Json, k: &str) -> String { String::from(j.get(k).str().unwrap_or_else(|| panic!("kit plan: a record's {}", k))) }
fn v2(j: &Json) -> V2 { let a = j.items(); [a[0].num().expect("kit plan: a point"), a[1].num().expect("kit plan: a point")] }
fn strs(j: &Json) -> Option<Vec<String>> { if j.is_null() { None } else { Some(j.items().iter().map(|x| String::from(x.str().expect("kit plan: a name"))).collect()) } }

/// A bespoke set: its records are a level of their own, and `ref_` a point on the team's side of its anchor
#[derive(Clone, Debug)]
pub struct PlanSet { pub level: PlanLevel, pub ref_: V2 }
impl PlanSet { pub fn of(t: &Json) -> PlanSet { PlanSet { level: PlanLevel { boxes: boxes_of(t.get("boxes")), doors: doors_of(t.get("doors")), lights: lights_of(t.get("lights")), props: Vec::new(), left: Vec::new() }, ref_: v2(t.get("ref")) } } }
/// The block's sets, by name
#[derive(Clone, Debug, Default)]
pub struct PlanBlock { pub sets: Vec<(String, PlanSet)> }
impl PlanBlock {
    /// `scenarios.kitView` as the file has it
    pub fn of(k: &Json) -> PlanBlock { PlanBlock { sets: k.get("sets").members().iter().map(|(n, t)| (n.clone(), PlanSet::of(t))).collect() } }
    fn set(&self, name: &str) -> Option<&PlanSet> { self.sets.iter().find(|(n, _)| n == name).map(|(_, t)| t) }
}

#[derive(Clone, Debug)]
pub struct Pin { pub yaw: f64, pub pitch: f64, pub dist: Option<f64> }
#[derive(Clone, Debug, Default)]
pub struct WayWall { pub face: Option<f64>, pub thickness: Option<f64>, pub top: Option<f64> }
#[derive(Clone, Debug)]
pub enum WayPane { No, Box { name: String, stand: Option<V3> } }
#[derive(Clone, Debug, Default)]
pub struct WayLight { pub porch_offset: Option<V3>, pub porch_aim: Option<V3>, pub porch_fixture_at: Option<V3>, pub fill_offset: Option<V3>, pub fill_aim: Option<V3> }
#[derive(Clone, Debug, Default)]
pub struct WayAdd { pub boxes: Vec<PlanBox>, pub lamps: Vec<PlanLight>, pub laid: bool }
/// A way as the planner takes it: every key it reads, absent kept apart from any value
#[derive(Clone, Debug, Default)]
pub struct Way {
    pub door: Option<String>, pub side: Option<f64>, pub pin: Option<Pin>, pub slots: Option<Vec<Option<V3>>>, pub light: Option<WayLight>, pub hide: Option<Vec<String>>, pub hide_paint: Option<Vec<String>>, pub hide_more: Option<Vec<String>>,
    pub span: Option<[V2; 2]>, pub kind: Option<String>, pub set: Option<String>, pub ref_: Option<V2>, pub hand: Option<String>, pub slot_gap: Option<f64>, pub wall: Option<WayWall>, pub lane: Option<f64>, pub out: Option<f64>, pub inside: Option<f64>,
    pub pane: Option<WayPane>, pub show: Option<Vec<String>>, pub indoors: Option<bool>, pub keep_lights: Option<Vec<String>>, pub day_lights: Option<Vec<String>>, pub lit_inside: bool, pub add: Option<WayAdd>,
}
impl Way {
    pub fn of(y: &Json) -> Way {
        let word = |k: &str| y.get(k).str().map(String::from);
        let light = y.get("light");
        Way {
            door: word("door"), side: y.get("side").num(), pin: if y.get("pin").is_null() { None } else { let p = y.get("pin"); Some(Pin { yaw: p.get("yaw").num().expect("kit plan: a pin's yaw"), pitch: p.get("pitch").num().expect("kit plan: a pin's pitch"), dist: p.get("dist").num() }) },
            slots: if y.get("slots").is_null() { None } else { Some(y.get("slots").items().iter().map(|p| if p.is_null() { None } else { Some(p.v3().expect("kit plan: a place")) }).collect()) },
            light: if light.is_null() { None } else { Some(WayLight { porch_offset: light.get("porch").get("offset").v3(), porch_aim: light.get("porch").get("aim").v3(), porch_fixture_at: light.get("porch").get("fixture").get("at").v3(), fill_offset: light.get("fill").get("offset").v3(), fill_aim: light.get("fill").get("aim").v3() }) },
            hide: strs(y.get("hide")), hide_paint: strs(y.get("hidePaint")), hide_more: strs(y.get("hideMore")),
            span: if y.get("span").is_null() { None } else { Some([v2(y.get("span").at(0)), v2(y.get("span").at(1))]) }, kind: word("kind"), set: word("set"), ref_: if y.get("ref").is_null() { None } else { Some(v2(y.get("ref"))) }, hand: word("hand"), slot_gap: y.get("slotGap").num(),
            wall: if y.get("wall").is_null() { None } else { let w = y.get("wall"); Some(WayWall { face: w.get("face").num(), thickness: w.get("thickness").num(), top: w.get("top").num() }) }, lane: y.get("lane").num(), out: y.get("out").num(), inside: y.get("inside").num(),
            pane: match y.get("pane") { Json::Null => None, Json::Bool(false) => Some(WayPane::No), p => Some(WayPane::Box { name: text(p, "box"), stand: p.get("stand").v3() }) },
            show: strs(y.get("show")), indoors: y.get("indoors").bool(), keep_lights: strs(y.get("keepLights")), day_lights: strs(y.get("dayLights")), lit_inside: y.get("litInside").bool() == Some(true),
            add: if y.get("add").is_null() { None } else { let a = y.get("add"); Some(WayAdd { boxes: boxes_of(a.get("boxes")), lamps: lights_of(a.get("lamps")), laid: a.get("laid").bool() == Some(true) }) },
        }
    }
}

// ---- what comes out

/// the plan as a value, keyed as the page reads it
pub fn planned_json(p: &Planned) -> Json {
    let v = |x: &[f64]| json::nums(x);
    let names = |x: &[String]| json::arr(x.iter().map(|s| json::s(s)).collect());
    let idx = |x: &[usize]| json::arr(x.iter().map(|&i| json::n(i as f64)).collect());
    let opt = |x: &Option<String>| x.as_ref().map_or(Json::Null, |s| json::s(s));
    let an = &p.anchor;
    json::obj(vec![
        ("anchor", json::obj(vec![("kind", json::s(&an.kind)), ("door", opt(&an.door)), ("a", v(&an.a)), ("b", v(&an.b)), ("mid", v(&an.mid)), ("t", v(&an.t)), ("n", v(&an.n)), ("width", json::n(an.width)), ("steel", json::b(an.steel)), ("locked", json::b(an.locked)), ("exterior", json::b(an.exterior))])),
        ("ref", v(&p.ref_)), ("side", json::n(p.side)), ("s", v(&p.s)), ("hand", json::s(&p.hand)),
        ("wall", json::obj(vec![("face", json::n(p.wall.face)), ("back", json::n(p.wall.back)), ("thickness", json::n(p.wall.thickness)), ("top", json::n(p.wall.top)), ("boxes", names(&p.wall.boxes))])),
        ("slots", json::arr(p.slots.iter().map(|p| v(p)).collect())), ("slotSrc", json::arr(p.slot_src.iter().map(|x| json::s(x.word())).collect())), ("fit", json::n(p.fit as f64)), ("stepped", json::n(p.stepped as f64)), ("slotGap", json::n(p.slot_gap)),
        ("lane", json::obj(vec![("out", json::n(p.lane.out)), ("a", v(&p.lane.a)), ("b", v(&p.lane.b)), ("free", json::b(p.lane.free)), ("clearM", json::n(p.lane.clear_m))])),
        ("out", json::n(p.out)), ("inside", json::n(p.inside)), ("beyond", json::n(p.beyond)),
        ("pane", p.pane.as_ref().map_or(Json::Null, |x| json::obj(vec![("box", json::s(&x.name)), ("index", json::n(x.index as f64)), ("at", v(&x.at)), ("width", json::n(x.width)), ("n", v(&x.n)), ("stand", v(&x.stand)), ("from", json::n(x.from)), ("onWall", json::b(x.on_wall)), ("along", json::n(x.along))]))),
        ("occluders", json::arr(p.occluders.iter().map(|o| json::obj(vec![("id", json::s(&o.id)), ("index", json::n(o.index as f64)), ("group", json::s(&o.group)), ("why", json::s(o.why))])).collect())),
        ("hidden", idx(&p.hidden)), ("hiddenProps", idx(&p.hidden_props)),
        ("lamps", json::obj(vec![
            ("porch", json::obj(vec![("offset", v(&p.lamps.porch.offset)), ("aim", v(&p.lamps.porch.aim)), ("pos", v(&p.lamps.porch.pos)), ("fixture", p.lamps.porch.fixture.as_ref().map_or(Json::Null, |f| json::obj(vec![("at", v(&f.at)), ("pos", v(&f.pos))])))])),
            ("fill", json::obj(vec![("offset", v(&p.lamps.fill.offset)), ("aim", v(&p.lamps.fill.aim)), ("pos", v(&p.lamps.fill.pos))])),
        ])),
        ("keepLights", names(&p.keep_lights)), ("indoors", json::b(p.indoors)), ("covered", json::n(p.covered as f64)),
        ("cam", json::obj(vec![("look", v(&p.cam.look)), ("yaw", json::n(p.cam.yaw)), ("pitch", json::n(p.cam.pitch)), ("dist", json::n(p.cam.dist)), ("pos", v(&p.cam.pos))])),
        ("src", json::obj(p.src.iter().map(|(k, x)| (*k, json::s(x.word()))).collect())),
        ("problems", names(&p.problems)), ("set", opt(&p.set)),
    ])
}

// ---- small arithmetic

fn dot(a: V2, b: V2) -> f64 { a[0] * b[0] + a[1] * b[1] }
fn r2(v: f64) -> f64 { m::round(v * 100.0) / 100.0 }
fn by_diff(d: f64) -> Ordering { if d < 0.0 { Ordering::Less } else if d > 0.0 { Ordering::Greater } else { Ordering::Equal } }
/// a number in words as the language writes it: the same shortest digits as `{}`, no sign on a zero, an exponent from 1e21 up and under 1e-6
pub(crate) fn js_num(x: f64) -> String {
    if x != x { return String::from("NaN"); }
    if x == 0.0 { return String::from("0"); }
    if x == m::INF { return String::from("Infinity"); }
    if x == -m::INF { return String::from("-Infinity"); }
    let a = m::abs(x);
    if a >= 1e-6 && a < 1e21 { return format!("{}", x); }
    let t = format!("{:e}", x);
    match t.split_once('e') { Some((d, e)) if !e.starts_with('-') => format!("{}e+{}", d, e), _ => t }
}
/// the language's default sort of names: by UTF-16 unit, equal ones in the order they came
pub(crate) fn js_sort(v: &mut [String]) { v.sort_by(|a, b| a.encode_utf16().cmp(b.encode_utf16())); }
fn uniq(v: impl IntoIterator<Item = String>) -> Vec<String> { let mut o: Vec<String> = Vec::new(); for x in v { if !o.contains(&x) { o.push(x); } } o }

/// the floor distance from (x, z) to a box's yawed footprint (0 inside)
fn gap_to(b: &PlanBox, x: f64, z: f64) -> f64 {
    let (c, s, dx, dz) = (b.cy, b.sy, x - b.c[0], z - b.c[2]);
    let (lx, lz) = (c * dx - s * dz, s * dx + c * dz);
    m::hyp2(lx - m::max(-b.h[0], m::min(b.h[0], lx)), lz - m::max(-b.h[2], m::min(b.h[2], lz)))
}
/// the floor gap between two boxes' yawed footprints, to a few cm (corners of each against the other; 0 where they overlap)
fn gap_boxes(p: &PlanBox, q: &PlanBox) -> f64 {
    fn pts(b: &PlanBox) -> Vec<V2> {
        let (c, s) = (b.cy, b.sy);
        let mut o = Vec::with_capacity(9);
        for kx in [-1.0, 0.0, 1.0] { for kz in [-1.0, 0.0, 1.0] { let (lx, lz): (f64, f64) = (kx * b.h[0], kz * b.h[2]); o.push([b.c[0] + c * lx + s * lz, b.c[2] - s * lx + c * lz]); } }
        o
    }
    let mut r = m::INF;
    for v in pts(p) { r = m::min(r, gap_to(q, v[0], v[1])); }
    for v in pts(q) { r = m::min(r, gap_to(p, v[0], v[1])); }
    r
}
/// half a box's reach along a floor direction
fn reach(b: &PlanBox, d: V2) -> f64 { let (c, s) = (b.cy, b.sy); m::abs(d[0] * c - d[1] * s) * b.h[0] + m::abs(d[0] * s + d[1] * c) * b.h[2] }
/// does the segment p → q meet the box (yawed; a tilted one by its yaw alone)?
fn seg_box(b: &PlanBox, p: V3, q: V3) -> bool {
    let (c, s) = (b.cy, b.sy);
    let l = |v: V3| -> V3 { let (dx, dz) = (v[0] - b.c[0], v[2] - b.c[2]); [c * dx - s * dz, v[1] - b.c[1], s * dx + c * dz] };
    let (o, e) = (l(p), l(q));
    let (mut t0, mut t1) = (0.0, 1.0);
    for ax in 0..3 {
        let (d, h) = (e[ax] - o[ax], b.h[ax]);
        if m::abs(d) < 1e-12 { if m::abs(o[ax]) > h { return false; } continue; }
        let (mut ta, mut tb) = ((-h - o[ax]) / d, (h - o[ax]) / d);
        if ta > tb { core::mem::swap(&mut ta, &mut tb); }
        if ta > t0 { t0 = ta; }
        if tb < t1 { t1 = tb; }
        if t0 > t1 { return false; }
    }
    true
}

/// UNDER A COVER: a box over the point with its underside at `cover_y` at least and no lower than `y` (a lamp's own height), `cover_half` wide both ways: a ceiling, a roof. What `keep_lights`
/// leaves out as a room's lamp, and what the day's photocell leaves burning.
pub fn covered(boxes: &[PlanBox], x: f64, z: f64, y: f64) -> bool {
    boxes.iter().any(|b| b.c[1] - b.h[1] >= m::max(K.cover_y, y - 0.05) && m::min(b.h[0], b.h[2]) >= K.cover_half && gap_to(b, x, z) < 0.02)
}

/// THE FLOOR of a level's boxes, as the nav grid bakes it: built once per level, shared by its ways
#[derive(Clone, Debug)]
pub struct Floor { pub solid: Vec<usize>, grid: Vec<u8> }
impl Floor {
    /// the nav grid's answer for the cell (x, z) lies in
    pub fn blocked(&self, wx: f64, wz: f64) -> bool {
        let nv = &K.nav;
        let (x, z) = (m::floor(wx / nv.cell), m::floor(wz / nv.cell));
        if x < 0.0 || z < 0.0 || x >= nv.w as f64 || z >= nv.h as f64 { return true; }
        if x != x || z != z { return false; }   // (no cell of the language's array: not 1)
        self.grid[z as usize * nv.w + x as usize] == 1
    }
}
/// `out` = boxes (indexes) that do not stand on it
pub fn floor(boxes: &[PlanBox], out: Option<&BTreeSet<usize>>) -> Floor {
    let nv = &K.nav;
    let mut solid = Vec::new();
    let mut grid = vec![0u8; nv.w * nv.h];
    for (i, b) in boxes.iter().enumerate() { if !out.map_or(false, |o| o.contains(&i)) && !b.no_shadow && b.c[1] + b.h[1] >= nv.y0 && b.c[1] - b.h[1] <= nv.y1 { solid.push(i); } }
    // the game's own bake, step for step (one pass of the collision): the circle is pushed out of each box IN TURN, the turn being the 1 m collision cells' (rows of z, then x; a box in the
    // first of its cells the circle's reach meets), then the file's order, and the cell is shut when it ends up more than 5 mm from where it began. So a circle squeezed back by a second box,
    // or nudged twice, answers as the game's does.
    let cs = nv.coarse;
    let (cw_f, ch_f) = (nv.w as f64 * nv.cell / cs, nv.h as f64 * nv.cell / cs);
    let (cw, ch) = (cw_f as usize, ch_f as usize);
    let clampi = |v: f64, hi: f64| m::max(0.0, m::min(hi, v));
    struct Reg { i: usize, x0: i64, x1: i64, z0: i64, z1: i64 }
    let reg: Vec<Reg> = solid.iter().map(|&i| {
        let b = &boxes[i];
        let (c, sn) = (m::abs(b.cy), m::abs(b.sy));
        let (ex, ez) = (c * b.h[0] + sn * b.h[2], sn * b.h[0] + c * b.h[2]);
        Reg { i, x0: m::max(0.0, m::floor((b.c[0] - ex) / cs)) as i64, x1: m::min(cw_f - 1.0, m::floor((b.c[0] + ex) / cs)) as i64, z0: m::max(0.0, m::floor((b.c[2] - ez) / cs)) as i64, z1: m::min(ch_f - 1.0, m::floor((b.c[2] + ez) / cs)) as i64 }
    }).collect();
    let mut coarse: Vec<Vec<usize>> = vec![Vec::new(); cw * ch];
    for (k, r) in reg.iter().enumerate() { let mut z = r.z0; while z <= r.z1 { let mut x = r.x0; while x <= r.x1 { coarse[z as usize * cw + x as usize].push(k); x += 1; } z += 1; } }
    let mut seen = vec![-1i32; reg.len()];
    let mut stamp = 0i32;
    for gz in 0..nv.h { for gx in 0..nv.w {
        let (px, pz) = ((gx as f64 + 0.5) * nv.cell, (gz as f64 + 0.5) * nv.cell);
        let (mut x, mut z, mut touched, id) = (px, pz, false, stamp);
        stamp += 1;
        let (x0, x1, z0, z1) = (clampi(m::floor((x - nv.r) / cs), cw_f - 1.0) as usize, clampi(m::floor((x + nv.r) / cs), cw_f - 1.0) as usize, clampi(m::floor((z - nv.r) / cs), ch_f - 1.0) as usize, clampi(m::floor((z + nv.r) / cs), ch_f - 1.0) as usize);
        for cz in z0..=z1 { for cx in x0..=x1 { for &k in &coarse[cz * cw + cx] {
            if seen[k] == id { continue; }
            seen[k] = id;
            let b = &boxes[reg[k].i];
            let (c, sn, dx, dz) = (b.cy, b.sy, x - b.c[0], z - b.c[2]);
            let (lx, lz) = (c * dx - sn * dz, sn * dx + c * dz);
            let (qx, qz) = (m::max(-b.h[0], m::min(b.h[0], lx)), m::max(-b.h[2], m::min(b.h[2], lz)));
            let (ddx, ddz) = (lx - qx, lz - qz);
            let d2 = ddx * ddx + ddz * ddz;
            if d2 >= nv.r * nv.r { continue; }
            let or_one = |v: f64| { let g = m::sign(v); if g == 0.0 || g != g { 1.0 } else { g } };
            let (nx, nz, pen);
            if d2 > 1e-10 { let d = m::sqrt(d2); nx = ddx / d; nz = ddz / d; pen = nv.r - d; }
            else {
                let (ox, oz) = (b.h[0] - m::abs(lx), b.h[2] - m::abs(lz));
                if ox < oz { nx = or_one(lx); nz = 0.0; pen = ox + nv.r; } else { nx = 0.0; nz = or_one(lz); pen = oz + nv.r; }
            }
            x += (c * nx + sn * nz) * pen;
            z += (-sn * nx + c * nz) * pen;
            touched = true;
        } } }
        if touched && m::hyp2(x - px, z - pz) > nv.moved { grid[gz * nv.w + gx] = 1; }
    } }
    Floor { solid, grid }
}

/// The line of places: `stepped` = places passed over, all five; `steps` = the same per place, how many were passed over to find THAT man's
#[derive(Clone, Debug)]
pub struct SlotLine { pub slots: Vec<V3>, pub stepped: usize, pub steps: Vec<usize> }
/// A door's (or a span's) places, [x, z, yaw°]: Sam, then the teammates from the anchor outward, with the wall man's and Sam's distances given from the anchor's line.
pub fn slots(a: V2, t: V2, width: f64, n: V2, side: f64, off: f64, sam_off0: f64, free: &dyn Fn(f64, f64) -> bool) -> SlotLine {
    let j: V2 = [a[0] + t[0] * width, a[1] + t[1] * width];
    let s: V2 = [t[0] * side, t[1] * side];
    let (near, far) = if side > 0.0 { (j, a) } else { (a, j) };
    let mut stepped = 0usize;
    let mut lines = [0.0f64, 0.0];
    let at = |line: usize, o: f64, al: f64| -> V2 { let (from, dir) = if line == 1 { (far, -1.0) } else { (near, 1.0) }; [from[0] + s[0] * dir * al + n[0] * o, from[1] + s[1] * dir * al + n[1] * o] };
    let mut take = |line: usize, o: f64, stepped: &mut usize| -> V2 {
        for _ in 0..10 {
            let p = at(line, o, K.jamb + K.gap * lines[line]);
            lines[line] += 1.0;
            if free(p[0], p[1]) { return p; }
            *stepped += 1;
        }
        at(line, o, K.jamb)
    };
    let (c, sn) = (m::cos(K.sam_turn_deg * RAD), m::sin(K.sam_turn_deg * RAD));
    let yaw_of = |d: V2| m::atan2(d[0], d[1]) / RAD;
    let along = width / 2.0 + K.jamb;
    let sam_off = m::max(sam_off0, m::sqrt(m::max(0.0, K.sam_clear * K.sam_clear - along * along)));
    let (mut out, mut steps, mut was) = (Vec::with_capacity(5), Vec::with_capacity(5), 0usize);
    {
        let p = take(1, sam_off, &mut stepped);
        steps.push(stepped - was); was = stepped;
        out.push([p[0], p[1], yaw_of([s[0] * c - n[0] * sn, s[1] * c - n[1] * sn])]);
    }
    for i in 0..4 {
        let behind = i >= 3;
        let p = take(if behind { 1 } else { 0 }, off, &mut stepped);
        steps.push(stepped - was); was = stepped;
        out.push([p[0], p[1], yaw_of(if behind { s } else { [-s[0], -s[1]] })]);
    }
    SlotLine { slots: out, stepped, steps }
}

/// THE LAMPS THAT LIGHT THE WEDGE BEHIND THE ANCHOR (a way's `litInside`: their groups stay lit with the room lights out): every lamp that is no sun or moon, standing behind the wall's far face,
/// with the wedge point inside its range and inside `lit_reach`. A spot's cone is not asked about, nor a wall between: the level writes `keepLights` where that matters.
pub fn lit_inside<'a>(lights: &'a [PlanLight], mid: V2, n: V2, back: f64) -> Vec<&'a PlanLight> {
    let w: V3 = [mid[0] + n[0] * (back - K.lit_in), K.lit_y, mid[1] + n[1] * (back - K.lit_in)];
    lights.iter().filter(|l| !l.dir && dot([l.pos[0] - mid[0], l.pos[2] - mid[1]], n) < back && m::hyp3(l.pos[0] - w[0], l.pos[1] - w[1], l.pos[2] - w[2]) <= m::min(l.range, K.lit_reach)).collect()
}

/// THE WAY'S OWN LAMPS: the lights of the set it stands on, or, a level's own way, the lamps it adds. Never a lamp of the level's.
pub fn own_lamps<'a>(block: Option<&'a PlanBlock>, way: &'a Way) -> &'a [PlanLight] {
    match &way.set {
        Some(name) => block.and_then(|k| k.set(name)).map_or(&[], |t| t.level.lights.as_slice()),
        None => way.add.as_ref().map_or(&[], |a| a.lamps.as_slice()),
    }
}
/// …AND THEY ARE ON THE SAME PHOTOCELL: the ids of the way's own lamps that are OUT. At an hour with the outside lamps out, on a way under the open sky: every own lamp that is no sun or moon, not of
/// the group `exterior` (the game's own photocell has those) and not under a cover (a lamp with a ceiling over it is a room's) less the ones `day` names, by id or by group. By id, never by group
/// in what comes out: an added lamp may share a group with the level's own lamps, which the mission leaves as they are.
pub fn day_out(own: &[PlanLight], boxes: &[PlanBox], day: Option<&[String]>, indoors: bool, lamps_out: bool) -> Vec<String> {
    if !(lamps_out && !indoors) { return Vec::new(); }
    let day = day.unwrap_or(&[]);
    own.iter().filter(|l| !l.dir && l.group != "exterior" && !covered(boxes, l.pos[0], l.pos[2], l.pos[1]) && !day.iter().any(|nm| *nm == l.id || *nm == l.group)).map(|l| l.id.clone()).collect()
}
/// A WAY'S OWN PIECES laid in AFTER the level's own: box i of the level is still box i. By id, once: the child plans from the records of the level the page built, which carry them already, and
/// a block that says `laid` lays nothing. Nothing to add: the very level handed in.
pub fn add<'a>(level: &'a PlanLevel, way: &Way) -> Cow<'a, PlanLevel> {
    let a = match (&way.set, &way.add) { (None, Some(a)) if !a.laid => a, _ => return Cow::Borrowed(level) };
    let nb: Vec<&PlanBox> = a.boxes.iter().filter(|b| !level.boxes.iter().any(|q| q.id == b.id)).collect();
    let nl: Vec<&PlanLight> = a.lamps.iter().filter(|l| !level.lights.iter().any(|q| q.id == l.id)).collect();
    if nb.is_empty() && nl.is_empty() { return Cow::Borrowed(level); }
    let mut o = level.clone();
    o.boxes.extend(nb.into_iter().cloned());
    o.lights.extend(nl.into_iter().cloned());
    Cow::Owned(o)
}
/// THE WAY AS THE CHILD PLANS IT: its `add` said to be laid, for the child's records are the level the page built, by definition. Laying a piece into them gave a plan one box longer than the world.
pub fn way_built(way: &Way) -> Way { let mut o = way.clone(); if let Some(a) = o.add.as_mut() { a.laid = true; } o }
/// What a way is planned from
#[derive(Clone, Debug)]
pub struct Geometry<'a> { pub level: Cow<'a, PlanLevel>, pub ref_: V2, pub set: Option<String> }
/// THE GEOMETRY a way is planned from, and the point on the team's side: the way's set where it names one (the set alone), else the level with the way's own `add` laid in, and `ref_`.
/// None = the way names a set the block does not have.
pub fn geometry<'a>(level: &'a PlanLevel, block: Option<&'a PlanBlock>, way: &Way, ref_: V2) -> Option<Geometry<'a>> {
    match &way.set {
        None => Some(Geometry { level: add(level, way), ref_, set: None }),
        Some(name) => block.and_then(|k| k.set(name)).map(|t| Geometry { level: Cow::Borrowed(&t.level), ref_: t.ref_, set: Some(name.clone()) }),
    }
}

/// `floor` = this level's floor built once · `free` = the game's own grid in its place · `hidden_stand` = the boxes the way's `hide` names still stand on the floor (else they are out of it, as they
/// are out of the picture) · `also_hide` = ids the caller has already taken out of the picture: out of the floor and among `hidden`, without the way's `hide` counting as written
#[derive(Clone, Copy, Default)]
pub struct PlanOpts<'a> { pub floor: Option<&'a Floor>, pub free: Option<&'a dyn Fn(f64, f64) -> bool>, pub hidden_stand: bool, pub also_hide: &'a [String] }

/// …and the plan from it. A set has its own floor, and so has a level with a piece added here: the level's `floor` and `free` are not handed on, for a `free` made on the bare level puts a man inside
/// the added crate and says nothing. What the caller says of the way's `hide` is the way's, whatever it is planned on.
pub fn plan_way(level: &PlanLevel, block: Option<&PlanBlock>, way: &Way, ref_: V2, opts: &PlanOpts) -> Planned {
    let g = match geometry(level, block, way, ref_) {
        Some(g) => g,
        None => {
            let mut p = plan(&PlanLevel::default(), way, ref_, &PlanOpts::default());
            let name = way.set.clone().unwrap_or_default();
            p.problems = vec![format!("no set '{}' in the block's sets", name)];
            p.set = way.set.clone();
            return p;
        }
    };
    let own = g.set.is_none() && matches!(g.level, Cow::Borrowed(_));
    let said = PlanOpts { floor: None, free: None, hidden_stand: opts.hidden_stand, also_hide: opts.also_hide };
    let mut p = plan(&g.level, way, g.ref_, if own { opts } else { &said });
    p.set = g.set;
    p
}

/// THE PLAN AS THE CHILD MAKES IT: two passes on the floor as the game has it. The first plans with the way's `hide` still standing on that floor and hands what is out of the picture to `sink`
/// (the child puts those boxes under the ground and rebakes; `free` must answer for the floor that leaves). The second plans again with the first's occluders named as already out, none where the way
/// writes its `hide`, and sinks what it finds. The plan handed back is the second's, with the occluders of both and the first pass's problem of a non-thing occluder: the second is told those ids are
/// out and would never say it.
pub fn plan_child(level: &PlanLevel, way: &Way, ref_: V2, free: &dyn Fn(f64, f64) -> bool, sink: &mut dyn FnMut(&[usize])) -> Planned {
    let p0 = plan(level, way, ref_, &PlanOpts { free: Some(free), hidden_stand: true, ..PlanOpts::default() });
    sink(&p0.hidden);
    let also: Vec<String> = if way.hide.is_none() { p0.occluders.iter().map(|o| o.id.clone()).collect() } else { Vec::new() };
    let mut p1 = plan(level, way, ref_, &PlanOpts { free: Some(free), also_hide: &also, ..PlanOpts::default() });
    sink(&p1.hidden);
    let first: Vec<String> = p0.problems.iter().filter(|p| p.starts_with(IN_WAY) && !p1.problems.contains(p)).cloned().collect();
    let more: Vec<Occluder> = p1.occluders.iter().filter(|o| !p0.occluders.iter().any(|q| q.index == o.index)).cloned().collect();
    let mut occ = p0.occluders;
    occ.extend(more);
    p1.occluders = occ;
    p1.problems.extend(first);
    p1
}

fn say<T>(src: &mut Vec<(&'static str, Src)>, key: &'static str, authored: Option<T>, derive: impl FnOnce() -> T) -> T {
    match authored { Some(v) => { put(src, key, Src::Authored); v } None => { put(src, key, Src::Derived); derive() } }
}
fn put(src: &mut Vec<(&'static str, Src)>, key: &'static str, x: Src) { match src.iter_mut().find(|e| e.0 == key) { Some(e) => e.1 = x, None => src.push((key, x)) } }
fn of_name(id: &str, nm: &str) -> bool { id == nm || (id.len() > nm.len() && id.starts_with(nm) && id.as_bytes()[nm.len()] == b'.') }

/// THE PLAN for one way. `ref_` = a point on the team's side (Sam's start for this way; the way's own `ref` wins).
pub fn plan(level: &PlanLevel, way: &Way, ref_: V2, opts: &PlanOpts) -> Planned {
    let bs: &[PlanBox] = &level.boxes;
    // (what the way hides is out of the picture, so out of the picture's floor. The game's own grid still holds a sunk box until the child rebakes it: `hidden_stand` plans on the floor as that grid has it)
    let hide_more: Vec<String> = uniq(way.hide_more.clone().unwrap_or_default());
    let mut named: BTreeSet<&str> = BTreeSet::new();
    for x in way.hide.iter().flatten().chain(hide_more.iter()).chain(opts.also_hide.iter()) { named.insert(x.as_str()); }
    let is_named = |x: &str| named.contains(x);
    let mut sunk: BTreeSet<usize> = BTreeSet::new();
    if !opts.hidden_stand { for (i, bx) in bs.iter().enumerate() { if is_named(&bx.id) || bx.group.as_deref().map_or(false, is_named) { sunk.insert(i); } } }
    let own_floor: Option<Floor> = if !sunk.is_empty() { Some(floor(bs, Some(&sunk))) } else if opts.floor.is_none() { Some(floor(bs, None)) } else { None };
    let f: &Floor = own_floor.as_ref().or(opts.floor).expect("kit plan: a floor");
    let free_own = |x: f64, z: f64| !f.blocked(x, z);
    let free: &dyn Fn(f64, f64) -> bool = opts.free.unwrap_or(&free_own);
    let mut problems: Vec<String> = Vec::new();
    let mut src: Vec<(&'static str, Src)> = Vec::new();
    // boxes (indexes) whose footprint holds the floor point
    let over = |x: f64, z: f64, list: &[usize]| -> Vec<usize> { list.iter().copied().filter(|&i| gap_to(&bs[i], x, z) < 0.02).collect() };
    let all: Vec<usize> = (0..bs.len()).collect();
    let clear = |x: f64, z: f64, r: f64| f.solid.iter().all(|&i| gap_to(&bs[i], x, z) >= r);

    // THE ANCHOR: a door's frame, or the span as written
    let d: Option<&PlanDoor> = way.door.as_ref().and_then(|id| level.doors.iter().find(|x| x.id == *id));
    let (mut a, mut t, mut width, mut got): (V2, V2, f64, bool) = ([0.0, 0.0], [1.0, 0.0], 1.0, true);
    if let Some([p, q]) = way.span {
        let len = m::hyp2(q[0] - p[0], q[1] - p[1]);
        a = [p[0], p[1]];
        width = len;
        t = if len > 1e-6 { [(q[0] - p[0]) / len, (q[1] - p[1]) / len] } else { [1.0, 0.0] };
        if len < 0.4 { problems.push(format!("the span is {} m long: nothing to stand at", js_num(r2(len)))); }
    } else if let Some(dr) = d {
        a = [dr.hinge[0], dr.hinge[1]];
        t = [m::cos(dr.closed_dir), m::sin(dr.closed_dir)];
        width = dr.width;
    } else {
        got = false;
        problems.push(match &way.door { Some(id) => format!("no door '{}' in the level", id), None => String::from("no anchor: the way names neither a door nor a span") });
    }
    let (a, t, width, got) = (a, t, width, got);
    put(&mut src, "anchor", Src::Authored);
    let b: V2 = [a[0] + t[0] * width, a[1] + t[1] * width];
    let mid: V2 = [(a[0] + b[0]) / 2.0, (a[1] + b[1]) / 2.0];
    let rr: V2 = say(&mut src, "ref", way.ref_, || ref_);
    let n0: V2 = [-t[1], t[0]];
    let out_k = if dot([rr[0] - mid[0], rr[1] - mid[1]], n0) >= 0.0 { 1.0 } else { -1.0 };
    let n: V2 = [n0[0] * out_k, n0[1] * out_k];
    // (along a → b from the middle, out toward the team)
    let pt_at = |al: f64, o: f64| -> V2 { [mid[0] + t[0] * al + n[0] * o, mid[1] + t[1] * al + n[1] * o] };
    // what stands ON the anchor's line: wall boxes, thin across it
    let wallish: Vec<usize> = f.solid.iter().copied().filter(|&i| !bs[i].glass && reach(&bs[i], n) <= K.wall_half).collect();
    let on_line = |al: f64| -> Vec<usize> { let p = pt_at(al, 0.0); over(p[0], p[1], &wallish) };
    let within: Vec<f64> = [0.1, 0.3, 0.5, 0.7, 0.9].iter().map(|k: &f64| (k - 0.5) * width).collect();
    let shut = within.iter().filter(|&&al| !on_line(al).is_empty()).count();
    let kind: String = say(&mut src, "kind", way.kind.clone(), || String::from(if way.span.is_none() { "door" } else if shut * 2 > within.len() { "wall" } else { "gap" }));
    let is_wall = kind == "wall";
    let is_gap = kind == "gap";
    if way.span.is_some() && is_wall && shut < within.len() { problems.push(format!("the span is not all wall ({} of {} points along it are in one): a wall anchor lies on a wall", shut, within.len())); }
    if way.span.is_some() && is_gap && shut > 0 { problems.push(format!("the span is not all open ({} of {} points along it are in a wall): a gap anchor is a real gap", shut, within.len())); }
    if way.span.is_some() && kind == "door" { problems.push(String::from("a span is a 'wall' or a 'gap', never a 'door'")); }

    // THE WALL beside the anchor (both ways along it, the nearest first), and over it
    let beside = [0.15, 0.45, 0.8, 1.2, 1.7];
    let mut hits: Vec<usize> = Vec::new();
    let wall_d = [0.0];
    for &dd in if is_wall { &wall_d[..] } else { &beside[..] } { for sg in [1.0, -1.0] { for i in on_line(sg * (if is_wall { 0.0 } else { width / 2.0 + dd })) { if !hits.contains(&i) { hits.push(i); } } } }
    if is_wall { for &al in &within { for i in on_line(al) { if !hits.contains(&i) { hits.push(i); } } } }
    let face_of = |i: usize, k: f64| dot([bs[i].c[0] - mid[0], bs[i].c[2] - mid[1]], n) + k * reach(&bs[i], n);
    let med = |mut q: Vec<f64>| -> f64 { q.sort_by(|x, y| by_diff(x - y)); if q.is_empty() { 0.0 } else { q[(q.len() - 1) >> 1] } };
    let w_ = way.wall.clone().unwrap_or_default();
    let face = say(&mut src, "wall.face", w_.face, || med(hits.iter().map(|&i| face_of(i, 1.0)).collect()));
    let thickness = say(&mut src, "wall.thickness", w_.thickness, || if !hits.is_empty() { r2(med(hits.iter().map(|&i| face_of(i, 1.0)).collect()) - med(hits.iter().map(|&i| face_of(i, -1.0)).collect())) } else { 0.0 });
    let back = face - thickness;
    // (its top over the anchor: the lintel there, else the lower of the two sides: whatever stands on the line and is thin across it, any height, ceilings apart)
    let top_at = |al: f64| -> f64 {
        let p = pt_at(al, 0.0);
        let mut r = 0.0;
        for i in over(p[0], p[1], &all) { let q = &bs[i]; if !q.no_primary && !q.glass && reach(q, n) <= K.wall_half && q.c[1] + q.h[1] > 0.3 { r = m::max(r, q.c[1] + q.h[1]); } }
        r
    };
    let top = say(&mut src, "wall.top", w_.top, || { let v = top_at(0.0); r2(if v > 0.0 { v } else { m::min(top_at(width / 2.0 + 0.3), top_at(-width / 2.0 - 0.3)) }) });
    if got && !is_gap && hits.is_empty() { problems.push(String::from("the anchor is not on a wall: nothing stands on its line beside it")); }
    if got && is_gap && on_line(width / 2.0 + 0.15).len() + on_line(-width / 2.0 - 0.15).len() == 0 { problems.push(String::from("the gap has no post: nothing stands on its line at either end")); }

    // THE SIDE and THE PLACES
    let (off, sam_off) = (face + K.off_face, face + K.sam_off_face);
    let line = |sd: f64| slots(a, t, width, n, sd, off, sam_off, free);
    // (under the worked-out camera: the stack falls left of the anchor where n × s is positive)
    let left_of = |sd: f64| (n[0] * t[1] - n[1] * t[0]) * sd > 0.0;
    let side: f64 = say(&mut src, "side", way.side, || { let (p, q) = (line(1.0).stepped, line(-1.0).stepped); if p < q { 1.0 } else if q < p { -1.0 } else if left_of(1.0) { 1.0 } else if left_of(-1.0) { -1.0 } else { 1.0 } });
    // (`passed`: the places the chosen side's line passed over, in the order it asked: an added piece standing on one is named below)
    let s: V2 = [t[0] * side, t[1] * side];
    let passed_cell: RefCell<Vec<V2>> = RefCell::new(Vec::new());
    let ll = slots(a, t, width, n, side, off, sam_off, &|x, z| { let ok = free(x, z); if !ok { passed_cell.borrow_mut().push([x, z]); } ok });
    let passed = passed_cell.into_inner();
    // (an authored stack slot is spread from the head of the stack to the picture's spacing, as the child spreads it: by gap over what the file was laid out at)
    let empty: Vec<Option<V3>> = Vec::new();
    let aa: &Vec<Option<V3>> = way.slots.as_ref().unwrap_or(&empty);
    let slot_of = |i: usize| -> Option<V3> { aa.get(i).copied().flatten() };
    let head = slot_of(1);
    let laid = match (slot_of(1), slot_of(2)) { (Some(p), Some(q)) => m::abs(dot([q[0] - p[0], q[1] - p[1]], s)), _ => K.gap };
    let slot_gap = say(&mut src, "slotGap", way.slot_gap, || r2(laid));
    // (a place can be written alone: a null in `slots`, or a list that stops short, leaves that place to the line worked out above)
    let spread = |i: usize| -> Option<V3> {
        let p = slot_of(i)?;
        let hd = match head { Some(hd) if !(i < 2 || i > 3 || slot_gap >= K.gap || slot_gap <= 0.0) => hd, _ => return Some(p) };
        let k = K.gap / slot_gap - 1.0;
        let al = dot([p[0] - hd[0], p[1] - hd[1]], s);
        Some([p[0] + s[0] * al * k, p[1] + s[1] * al * k, p[2]])
    };
    let mut slot_src: Vec<Src> = Vec::with_capacity(5);
    let places: Vec<V3> = ll.slots.iter().enumerate().map(|(i, w)| { let p = spread(i); slot_src.push(if p.is_some() { Src::Authored } else { Src::Derived }); p.unwrap_or(*w) }).collect();
    put(&mut src, "slots", if slot_src.iter().all(|x| *x == Src::Authored) { Src::Authored } else { Src::Derived });
    // (only the places left to the line count: a man the way writes stands where he is written, whatever the line stepped over to find a place it then did not use)
    let stepped: usize = ll.steps.iter().enumerate().map(|(i, k)| if slot_src[i] == Src::Derived { *k } else { 0 }).sum();
    let mut fit = 0usize;
    if got {
        for (i, p) in places.iter().enumerate() {
            let mut why: Vec<String> = Vec::new();
            let depth = dot([p[0] - mid[0], p[1] - mid[1]], n);
            if !free(p[0], p[1]) { why.push(String::from("not on free floor")); }
            if let Some(&j) = f.solid.iter().find(|&&j| gap_to(&bs[j], p[0], p[1]) < K.man) { why.push(if gap_to(&bs[j], p[0], p[1]) < 0.02 { format!("inside the box {}", bs[j].id) } else { format!("within a body of the box {}", bs[j].id) }); }
            if depth < face + K.man { why.push(String::from(if depth < back { "on the far side of the wall" } else { "inside the wall" })); }
            for j in 0..i {
                let dd = m::hyp2(p[0] - places[j][0], p[1] - places[j][1]);
                let need = if i >= 2 && i <= 3 && j == i - 1 { K.gap - 0.01 } else { K.apart };
                if dd < need { why.push(format!("{} m from place {} ({} m at least)", js_num(r2(dd)), j, js_num(r2(need)))); }
            }
            if !why.is_empty() { problems.push(format!("place {} ({}, {}): {}", i, js_num(r2(p[0])), js_num(r2(p[1])), why.join(", "))); } else { fit += 1; }
        }
    }
    let past_at = problems.len();   // (a stack man stands on the wall he stacks on: asked below, once what is out of the picture is known; said HERE)
    if got && stepped > 0 { problems.push(format!("no free floor for five in line: {} place{} stepped over", stepped, if stepped == 1 { "" } else { "s" })); }
    // (A WRITTEN STACK PLACE AND THE DOOR ITSELF: the worked-out line is outside the leaf's sweep by construction; a written place is taken as it is. Two things are said of places 1‥3 where the way
    // writes them: IN THE OPENING, past the jamb the wrong way; and INSIDE THE LEAF'S SWING, on the team's side, nearer the hinge than the leaf is wide and within the leaf's stop ON THAT SIDE of its
    // shut line: the door's `maxAngle` where the team stands on the positive side, minus its `minAngle` on the other, else the plain stop; never the wider of the two, or a door that cannot open toward
    // the team says it opens onto him. A door whose written `swing` is away from the team is not asked; one with none is dealt its side by the game, so it is asked.)
    if got && !is_wall {
        let near: V2 = if side > 0.0 { b } else { a };
        let turn = if d.is_some() { if n[0] * -t[1] + n[1] * t[0] >= 0.0 { 1.0 } else { -1.0 } } else { 0.0 };
        let stop = match d { Some(dr) => if turn > 0.0 { dr.max_angle.unwrap_or(K.leaf_stop) } else { -(dr.min_angle.unwrap_or(-K.leaf_stop)) }, None => 0.0 };
        for i in 1..=3 {
            if slot_src[i] != Src::Authored { continue; }
            let p = places[i];
            let al = dot([p[0] - near[0], p[1] - near[1]], s);
            let v: V2 = [p[0] - a[0], p[1] - a[1]];
            let (dh, o) = (m::hyp2(v[0], v[1]), dot(v, n));
            if al < -width { problems.push(format!("place {} ({}, {}) is written {} m PAST THE FAR JAMB, on Sam's side of the {}: the stack stands on the other side of it, from {} m past its own jamb \u{2014} move it across, or write the way's `side`", i, js_num(r2(p[0])), js_num(r2(p[1])), js_num(r2(-al - width)), kind, js_num(K.jamb))); }
            else if al < 0.0 { problems.push(format!("place {} ({}, {}) is written {} m INSIDE the opening (past the jamb the wrong way): the stack stands beside the {}, from {} m past its jamb \u{2014} move it along the wall", i, js_num(r2(p[0])), js_num(r2(p[1])), js_num(r2(-al)), kind, js_num(K.jamb))); }
            else if let Some(dr) = d.filter(|dr| kind == "door" && (dr.swing.is_none() || dr.swing == Some(turn)) && stop > 0.0 && o > 0.0 && dh < width - 0.005 && m::atan2(o, dot(v, t)) <= stop) {
                problems.push(format!("place {} ({}, {}) is written inside the leaf's swing ({} m from the hinge of a {} m leaf{}): the door opens onto him \u{2014} move it out of the leaf's reach", i, js_num(r2(p[0])), js_num(r2(p[1])), js_num(r2(dh)), js_num(r2(width)), if dr.swing.is_none() { ", should it open toward the team" } else { "" }));
            }
        }
    }
    // (the fourth man is behind Sam: the card switch walks him round Sam's yard side to his acting place and back, and never across the anchor. One written nearer the anchor than Sam, or on the stack's side, is said)
    {
        let al = |i: usize| dot([places[i][0] - mid[0], places[i][1] - mid[1]], s);
        if got && (slot_src[4] == Src::Authored || slot_src[0] == Src::Authored) && al(4) > al(0) - 0.01 {
            problems.push(format!("place 4 ({}, {}) is not behind Sam ({} m along the wall toward the stack, Sam {}): the card switch walks the fourth man round Sam's yard side and never across the anchor \u{2014} write him past Sam, away from the anchor", js_num(r2(places[4][0])), js_num(r2(places[4][1])), js_num(r2(al(4))), js_num(r2(al(0)))));
        }
    }

    // THE LANE beside the stack (the card switch), the free floor in front, the room behind
    let lane_out = say(&mut src, "lane", way.lane, || K.lane);
    let l0 = dot([places[1][0] - mid[0], places[1][1] - mid[1]], s);
    let l1 = dot([places[3][0] - mid[0], places[3][1] - mid[1]], s) + K.gap;
    let lo = dot([places[1][0] - mid[0], places[1][1] - mid[1]], n) + lane_out;
    let lane_at = |al: f64| -> V2 { [mid[0] + s[0] * al + n[0] * lo, mid[1] + s[1] * al + n[1] * lo] };
    let (mut clear_m, mut lane_free) = (0.0, true);
    { let mut al = l0; while al <= l1 + 1e-6 { let p = lane_at(al); if !free(p[0], p[1]) { lane_free = false; break; } clear_m = al - l0; al += K.step; } }
    // (free floor from `from` m off the anchor's line, going out (1) or in (-1): how far from the line it runs)
    let walk = |from: f64, dir: f64| -> f64 {
        let (mut last, mut seen, mut dd) = (0.0, false, 0.0);
        while dd <= K.reach {
            let o = from + dir * dd;
            let p = pt_at(0.0, o);
            if free(p[0], p[1]) { seen = true; last = m::abs(o); } else if seen || dd > K.sill { break; }
            dd += K.step;
        }
        r2(last)
    };
    let out = say(&mut src, "out", way.out, || m::max(0.0, r2(walk(face + K.man, 1.0) - face)));
    let behind = if got { walk(back - 0.05, -1.0) } else { 0.0 };
    let inside = say(&mut src, "inside", way.inside, || if is_wall { 0.0 } else { behind });
    let beyond = if is_wall { m::max(0.0, r2(behind + back)) } else { 0.0 };
    if got && !is_wall && inside < K.in_m { problems.push(format!("no room behind the anchor for a throw: {} m of floor (a can is put {} m in, {} at the nearest)", js_num(inside), js_num(K.in_m), js_num(K.in_min))); }
    if got && !lane_free { problems.push(format!("the lane beside the stack is shut {} m along (the card switch walks straight instead)", js_num(r2(clear_m)))); }

    // THE CAMERA's pin: into the wall, turned toward the stack; at the middle of the five places
    let count = places.len() as f64;
    let look_xz: V2 = [places.iter().fold(0.0, |q, p| q + p[0]) / count, places.iter().fold(0.0, |q, p| q + p[1]) / count];
    let alongs: Vec<f64> = places.iter().map(|p| dot([p[0], p[1]], s)).collect();
    let len = m::max_n(&alongs) - m::min_n(&alongs);
    let pin: Pin = say(&mut src, "cam", way.pin.clone(), || { let (c, sn) = (m::cos(K.yaw), m::sin(K.yaw)); let fw: V2 = [-n[0] * c + s[0] * sn, -n[1] * c + s[1] * sn]; Pin { yaw: m::atan2(-fw[0], -fw[1]), pitch: K.pitch, dist: None } });
    let dist = pin.dist.unwrap_or(K.dist + K.dist_per_m * len);
    let fw: V3 = [-m::sin(pin.yaw) * m::cos(pin.pitch), -m::sin(pin.pitch), -m::cos(pin.yaw) * m::cos(pin.pitch)];
    let cam = Cam { look: [look_xz[0], 1.0, look_xz[1]], yaw: pin.yaw, pitch: pin.pitch, dist, pos: [look_xz[0] - fw[0] * dist, 1.0 - fw[1] * dist, look_xz[1] - fw[2] * dist] };
    let right: V2 = [m::cos(pin.yaw), -m::sin(pin.yaw)];
    let hand: String = say(&mut src, "hand", way.hand.clone(), || String::from(if dot(s, right) < 0.0 { "left" } else { "right" }));

    // A PANE to shoot through: glass at a gun's height, a free place to stand on the team's side of it, open air beyond
    let pane: Option<Pane> = (|| {
        if matches!(way.pane, Some(WayPane::No)) { put(&mut src, "pane", Src::Authored); return None; }
        let (asked, given): (Option<&str>, Option<V3>) = match &way.pane { Some(WayPane::Box { name, stand }) => (Some(name.as_str()), *stand), _ => (None, None) };
        put(&mut src, "pane", if asked.is_some() { Src::Authored } else { Src::Derived });
        let mut best: Option<Pane> = None;
        let mut unplayable = false;
        // (not written: the child's gun row's own filter and order, so the pane said here is the pane it tries first: in the anchor's wall, within `pane_along`, the nearest along the wall)
        for (i, g) in bs.iter().enumerate() {
            if !g.glass || asked.map_or(false, |x| g.id != x) { continue; }
            let (gl, go) = (dot([g.c[0] - mid[0], g.c[2] - mid[1]], s), dot([g.c[0] - mid[0], g.c[2] - mid[1]], n));
            // (written, the same tests: the child's gun row puts every pane through them. Written data that cannot be played is a problem, and the plan's pane is then none)
            let mut no: Vec<String> = Vec::new();
            if m::abs(go) > K.pane_wall { no.push(format!("{} m off the anchor's line ({} m at most)", js_num(r2(m::abs(go))), js_num(K.pane_wall))); }
            if m::abs(gl) > K.pane_along { no.push(format!("{} m along the wall from the anchor's middle ({} m at most)", js_num(r2(m::abs(gl))), js_num(K.pane_along))); }
            if g.c[1] - g.h[1] > K.pane_y[0] { no.push(format!("its bottom {} m up ({} m at most)", js_num(r2(g.c[1] - g.h[1])), js_num(K.pane_y[0]))); }
            if g.c[1] + g.h[1] < K.pane_y[1] { no.push(format!("its top {} m up ({} m at least)", js_num(r2(g.c[1] + g.h[1])), js_num(K.pane_y[1]))); }
            if !no.is_empty() {
                if let (Some(x), false) = (asked, unplayable) { unplayable = true; problems.push(format!("the pane '{}' is not one the gun row shoots through: {} \u{2014} a pane is glass in the anchor's own wall, at a gun's height: write that box, or `false`", x, no.join(", "))); }
                continue;
            }
            let (c, sn, thin_x) = (g.cy, g.sy, g.h[0] < g.h[2]);
            let pn0: V2 = if thin_x { [c, -sn] } else { [sn, c] };
            let pt: V2 = [-pn0[1], pn0[0]];
            let half = if thin_x { g.h[2] } else { g.h[0] };
            let k = if dot([rr[0] - g.c[0], rr[1] - g.c[2]], pn0) >= 0.0 { 1.0 } else { -1.0 };
            let pn: V2 = [pn0[0] * k, pn0[1] * k];
            let mut us: Vec<f64> = vec![0.0];
            { let mut u = 0.3; while u <= half - 0.2 { us.push(u); us.push(-u); u += 0.3; } }
            let mut stand: Option<V3> = given.map(|v| [v[0], v[1], v[2]]);
            for dd in K.pane_off {
                if stand.is_some() { break; }
                for &u in &us {
                    let (x, z) = (g.c[0] + pn[0] * dd + pt[0] * u, g.c[2] + pn[1] * dd + pt[1] * u);
                    if !free(x, z) || !clear(x, z, K.man) || f.solid.iter().any(|&j| j != i && seg_box(&bs[j], [x, K.gun_y, z], [g.c[0] + pt[0] * u, K.gun_y, g.c[2] + pt[1] * u])) { continue; }
                    stand = Some([r2(x), r2(z), m::round(m::atan2(-pn[0], -pn[1]) / RAD)]);
                    break;
                }
            }
            let stand = match stand { Some(v) => v, None => continue };
            let from = places.iter().fold(m::INF, |q, p| m::min(q, m::hyp2(p[0] - stand[0], p[1] - stand[1])));
            if best.as_ref().map_or(true, |x| m::abs(gl) < m::abs(x.along)) {
                best = Some(Pane { name: g.id.clone(), index: i, at: [g.c[0], g.c[1], g.c[2]], width: r2(half * 2.0), n: pn, stand, from: r2(from), on_wall: m::abs(dot([g.c[0] - mid[0], g.c[2] - mid[1]], n)) < m::max(0.3, thickness) && m::abs(dot(pn0, n)) > 0.9, along: r2(gl) });
            }
        }
        if let (Some(x), true, false) = (asked, best.is_none(), unplayable) { problems.push(format!("no pane '{}' a man can stand at", x)); }
        best
    })();

    // WHAT STANDS IN THE CAMERA'S WAY: every box a line from the camera to a man (or to the anchor) meets: paint, glass and what the camera never draws apart
    let show: Vec<String> = uniq(way.show.clone().unwrap_or_default());
    let has_show = |x: &str| show.iter().any(|q| q == x);
    let mut marks: Vec<V3> = Vec::new();
    for p in &places { for y in K.heights { for [ox, oz] in [[0.0, 0.0], [K.man, 0.0], [-K.man, 0.0], [0.0, K.man], [0.0, -K.man]] { marks.push([p[0] + ox, y, p[1] + oz]); } } }
    let n_men = marks.len();   // (the marks so far are the men's own: what an added piece is asked about below)
    // (the way's own added pieces: written for this picture, so shown in it. Never an occluder, never sunk by what is worked out here. Where one does stand in the men's way it is a problem that names it)
    let mut added: BTreeSet<&str> = BTreeSet::new();
    if way.set.is_none() { if let Some(x) = &way.add { for bx in &x.boxes { added.insert(bx.id.as_str()); } } }
    { let p = pt_at(0.0, face + K.man); for y in K.heights { marks.push([p[0], y, p[1]]); } }   // (the anchor, from just in front of its face)
    // (…and the stage's floor: from the face out past the lane, from a place behind the stack's last man to Sam's side: where the men walk for a beat)
    {
        let al: Vec<f64> = places.iter().map(|p| dot([p[0] - mid[0], p[1] - mid[1]], s)).collect();
        let o1 = m::max(lo, dot([places[0][0] - mid[0], places[0][1] - mid[1]], n) + K.round) + K.stage_out;
        let mut x = m::min_n(&al) - 0.5;
        while x <= l1 + 1e-6 {
            let mut o = face + K.man;
            while o <= o1 + 1e-6 {
                for y in [0.05, K.heights[1]] { marks.push([mid[0] + s[0] * x + n[0] * o, y, mid[1] + s[1] * x + n[1] * o]); }
                o += 0.5;
            }
            x += 0.5;
        }
    }
    let solid_to = |bx: &PlanBox| bx.c[1] + bx.h[1] > K.low_top && !bx.no_primary && !bx.glass;
    let shown = |bx: &PlanBox| has_show(&bx.id) || bx.group.as_deref().map_or(false, has_show);
    let drawn = |bx: &PlanBox| solid_to(bx) && !shown(bx) && !added.contains(bx.id.as_str());
    let thing = |bx: &PlanBox| bx.c[1] + bx.h[1] <= K.thing_top && m::max(bx.h[0], bx.h[2]) <= K.thing_half;
    let mut in_way = vec![false; bs.len()];
    if got { for (i, bx) in bs.iter().enumerate() { if drawn(bx) && marks.iter().any(|q| seg_box(bx, cam.pos, *q)) { in_way[i] = true; } } }
    // (a THING in the way goes whole: the boxes of its group that touch it, and theirs: a car's wheels and mirrors with its body; a wall is never grown)
    let touch = |p: &PlanBox, q: &PlanBox| (0..3).all(|ax| { let (ep, eq) = if ax == 1 { (p.h[1], q.h[1]) } else { (m::hyp2(p.h[0], p.h[2]), m::hyp2(q.h[0], q.h[2])) }; m::abs(p.c[ax] - q.c[ax]) <= ep + eq + 0.03 }) && gap_boxes(p, q) <= 0.03;
    let mut part = vec![false; bs.len()];
    let mut grown: Vec<usize> = (0..bs.len()).filter(|&i| in_way[i] && thing(&bs[i])).collect();
    let mut grew = true;
    while grew {
        grew = false;
        for (i, bx) in bs.iter().enumerate() {
            if in_way[i] || part[i] || !drawn(bx) || !thing(bx) { continue; }
            if grown.iter().any(|&j| bs[j].group == bx.group && touch(&bs[j], bx)) { part[i] = true; grown.push(i); grew = true; }
        }
    }
    let occluders: Vec<Occluder> = bs.iter().enumerate().filter(|(i, _)| in_way[*i] || part[*i]).map(|(i, bx)| Occluder { id: bx.id.clone(), index: i, group: bx.group.clone().unwrap_or_default(), why: if in_way[i] { "line" } else { "part" } }).collect();
    // WHAT GOES OUT OF THE PICTURE. An authored `hide` wins outright: the boxes it names and nothing else (`hide: []` is "nothing goes"). No `hide`: the occluders worked out above go, less `show`, and one
    // that is not a thing is said as a problem, for sinking it takes a piece of the building out of the picture. `hideMore` is added, never instead.
    let hide_auth = way.hide.is_some();
    put(&mut src, "hide", if hide_auth { Src::Authored } else { Src::Derived });
    let paint: Vec<&String> = way.hide_paint.iter().flatten().collect();
    let hidden: Vec<usize> = (0..bs.len()).filter(|&i| {
        let bx = &bs[i];
        let by_name = is_named(&bx.id) || bx.group.as_deref().map_or(false, |g| is_named(g) || (paint.iter().any(|q| *q == g) && bx.h[1] < 0.01));
        by_name || (!hide_auth && (in_way[i] || part[i]))
    }).collect();
    // A STACK MAN STANDS ON THE WALL HE STACKS ON: a worked-out wall place past the end of the anchor's wall stands in the open, and every row passed. Asked where he stands along the wall: something
    // must stand on the anchor's line there that comes down to his shoulder (a header beam over an open gateway is no wall to stand at; a window passes by its pane or its sill), thin or thick (the wall
    // that turns the corner IS the wall's end), or another door of the level on the line. What the plan itself takes out of the picture is no wall to stand at. A place the way writes is the author's
    // word, never asked; a gap's stack stands off a post, not a wall, and is not asked either.
    if got && !is_gap && !hits.is_empty() {
        let mut said: Vec<String> = Vec::new();
        let door_at = |p: V2| level.doors.iter().any(|dr| {
            if d.map_or(false, |x| core::ptr::eq(x, dr)) { return false; }
            let u: V2 = [m::cos(dr.closed_dir), m::sin(dr.closed_dir)];
            let v: V2 = [p[0] - dr.hinge[0], p[1] - dr.hinge[1]];
            let along = dot(v, u);
            along > -0.05 && along < dr.width + 0.05 && m::abs(v[1] * u[0] - v[0] * u[1]) < K.door_on
        });
        let wall_at = |x: f64| { let p = pt_at(x, 0.0); door_at(p) || over(p[0], p[1], &all).iter().any(|&j| !hidden.contains(&j) && !bs[j].no_primary && bs[j].c[1] + bs[j].h[1] > 0.3 && bs[j].c[1] - bs[j].h[1] < K.wall_foot) };
        for i in 1..=4 {
            if slot_src[i] != Src::Derived { continue; }
            let al = dot([places[i][0] - mid[0], places[i][1] - mid[1]], t);
            if wall_at(al) { continue; }
            let lim = m::max(0.02, m::abs(al) - width / 2.0);
            let mut past = 0.01;
            while past < lim && !wall_at(al - m::sign(al) * past) { past += 0.01; }   // (back toward the anchor, no further than its own jamb)
            said.push(if i < 4 { format!("place {} ({}, {}) is {} m past the end of the wall the stack stands on: write the way's `side` (the stack on the other side of the anchor) or his place in `slots` (a written place is taken as it is: `null` leaves the others to the plan)", i, js_num(r2(places[i][0])), js_num(r2(places[i][1])), js_num(r2(past + 0.02))) }
                else { format!("place 4 ({}, {}) is {} m past the end of the wall the fourth man stands at, behind Sam: write the way's `side` (Sam and he on the other side of the anchor) or his place in `slots` (a written place is taken as it is: `null` leaves the others to the plan)", js_num(r2(places[4][0])), js_num(r2(places[4][1])), js_num(r2(past + 0.02))) });
        }
        problems.splice(past_at..past_at, said);
    }
    if !hide_auth {
        let mut ends: Vec<V2> = Vec::with_capacity(level.doors.len() * 3);
        for dr in &level.doors { let dt: V2 = [m::cos(dr.closed_dir), m::sin(dr.closed_dir)]; ends.push([dr.hinge[0], dr.hinge[1]]); ends.push([dr.hinge[0] + dt[0] * dr.width / 2.0, dr.hinge[1] + dt[1] * dr.width / 2.0]); ends.push([dr.hinge[0] + dt[0] * dr.width, dr.hinge[1] + dt[1] * dr.width]); }
        let why = |bx: &PlanBox| -> Option<String> {
            if !thing(bx) { return Some(if bx.c[1] + bx.h[1] > K.thing_top { format!("{} m tall", js_num(r2(bx.c[1] + bx.h[1]))) } else { format!("{} m long", js_num(r2(2.0 * m::max(bx.h[0], bx.h[2])))) }); }
            if ends.iter().any(|e| gap_to(bx, e[0], e[1]) < 0.12) { return Some(String::from("it carries a door")); }
            if level.lights.iter().any(|l| !l.dir && gap_to(bx, l.pos[0], l.pos[2]) < 0.2 && l.pos[1] >= bx.c[1] - bx.h[1] - 0.4 && l.pos[1] <= bx.c[1] + bx.h[1] + 0.4) { return Some(String::from("it carries a lamp")); }
            None
        };
        let bad: Vec<(&Occluder, String)> = occluders.iter().filter(|o| !is_named(&o.id) && !hide_more.contains(&o.group)).filter_map(|o| why(&bs[o.index]).map(|y| (o, y))).collect();
        if !bad.is_empty() {
            let shown_: Vec<String> = bad.iter().take(6).map(|(o, y)| format!("{}{}: {}", o.id, if o.group.is_empty() { String::new() } else { format!(" ({})", o.group) }, y)).collect();
            problems.push(format!("{}: {}{} \u{2014} write the way's `hide` (what goes; [] = nothing), a `pin`, or stand it on a set", IN_WAY, shown_.join(", "), if bad.len() > 6 { format!(", \u{2026} {} more", bad.len() - 6) } else { String::new() }));
        }
    }
    // AN ADDED PIECE IN THE MEN'S WAY (authored data wins over what is worked out, and is never dropped without a word): one that still stands and is met by a line from the camera to a man, or stands
    // on a place the line of places passed over for a man it places, is said by name. (…only where it IS the thing on the place: a place the floor shuts without the added pieces is that box's, and an
    // added piece that merely stands within a body of it was blamed for it. The floor without them is baked once, and only when a piece is that near.)
    if got && !added.is_empty() {
        let near = K.nav.r + K.nav.cell * 0.71;
        let mut mine: Vec<V2> = Vec::new();
        let mut k = 0usize;
        for (i, &n_st) in ll.steps.iter().enumerate() { if slot_src[i] == Src::Derived { mine.extend_from_slice(&passed[k.min(passed.len())..(k + n_st).min(passed.len())]); } k += n_st; }
        let mut bare: Option<Floor> = None;
        for (i, bx) in bs.iter().enumerate() {
            if !added.contains(bx.id.as_str()) || hidden.contains(&i) { continue; }
            if solid_to(bx) && !shown(bx) && marks[..n_men].iter().any(|q| seg_box(bx, cam.pos, *q)) { problems.push(format!("the added '{}' hides the stack: move it (it stands between the camera and a man; the way's `show` with its name keeps it there, in front of him)", bx.id)); }
            let mut on_it = 0usize;
            if f.solid.contains(&i) {
                for p in &mine {
                    if gap_to(bx, p[0], p[1]) < near {
                        let fl = bare.get_or_insert_with(|| { let mut o = sunk.clone(); for (j, q) in bs.iter().enumerate() { if added.contains(q.id.as_str()) { o.insert(j); } } floor(bs, Some(&o)) });
                        if !fl.blocked(p[0], p[1]) { on_it += 1; }
                    }
                }
            }
            if on_it > 0 { problems.push(format!("the added '{}' stands on the line of places ({} stepped over it): move it, or write the men's `slots`", bx.id, on_it)); }
        }
    }
    // …AND ITS LOOSE PROPS (a `hide` of a table must not leave its stool standing alone on the pavement)
    let pr = &level.props;
    let mut hidden_props: Vec<usize> = Vec::new();
    for (i, p) in pr.iter().enumerate() {
        if show.iter().any(|nm| of_name(&p.id, nm)) { continue; }
        if named.iter().any(|nm| of_name(&p.id, nm)) || (!hide_auth && hidden.iter().any(|&j| bs[j].group.as_deref().map_or(false, |g| bs[j].c[1] + bs[j].h[1] > K.low_top && of_name(&p.id, g) && gap_to(&bs[j], p.x, p.z) <= K.prop_near))) { hidden_props.push(i); }
    }
    for id in way.hide.iter().flatten().chain(hide_more.iter()).chain(show.iter()) {
        if !bs.iter().any(|bx| bx.id == *id || bx.group.as_deref() == Some(id.as_str())) && !pr.iter().any(|p| of_name(&p.id, id)) && !level.left.iter().any(|nm| of_name(nm, id)) { problems.push(format!("hide / show names '{}': no box, group or prop by that name", id)); }
    }
    // (one name, both words: its boxes would go and its loose props stay. Said, never guessed at)
    {
        let both: Vec<String> = uniq(way.hide.iter().flatten().chain(hide_more.iter()).cloned()).into_iter().filter(|id| has_show(id)).collect();
        if !both.is_empty() { problems.push(format!("'{}' {} written in `show` and in `hide` / `hideMore`: its boxes would go and its loose props stay \u{2014} write the name in one of the two", both.join("', '"), if both.len() == 1 { "is" } else { "are" })); }
    }

    // UNDER A COVER, and the lamps: the porch lamp from the wall's top, the fill from the free floor in front
    // (the game's own roofs: a slab in the ceiling's height band, 0.9 m across both ways, the four largest, are where no rain falls)
    let mut roofs: Vec<&PlanBox> = bs.iter().filter(|bx| bx.c[1] - bx.h[1] >= K.roof.y0 && bx.c[1] + bx.h[1] <= K.roof.y1 && bx.h[0] >= K.roof.half && bx.h[2] >= K.roof.half).collect();
    roofs.sort_by(|p, q| by_diff(q.h[0] * q.h[2] - p.h[0] * p.h[2]));
    roofs.truncate(K.roof.most);
    let n_cov = places.iter().filter(|p| roofs.iter().any(|bx| m::abs(p[0] - bx.c[0]) <= bx.h[0] + K.roof.pad && m::abs(p[1] - bx.c[2]) <= bx.h[2] + K.roof.pad)).count();
    let indoors = say(&mut src, "indoors", way.indoors, || n_cov * 2 > places.len());
    // (a gap has no wall over its middle: its lamp and the lamp's box go on what can carry them: the wall or post at the stack's end of the opening, else the other end's, `porch_hand` past the jamb and
    // under that wall's own top; neither end `porch_carry` tall: the light alone, overhead, and no box. An authored `wall.top` is the stack end's. A door and a stretch of wall: along 0, under `top`.)
    let (carrier_al, carrier_top): (f64, f64) = if !is_gap { (0.0, top) } else if w_.top.is_some() { (if top > 0.0 { r2(width / 2.0 + K.porch_hand) } else { 0.0 }, top) } else {
        let mut found = (0.0, 0.0);
        'ends: for sd in [side, -side] { for h in [K.porch_hand, K.porch_hand / 2.0] {
            let c = (r2(sd * side * (width / 2.0 + h)), r2(top_at(sd * (width / 2.0 + h))));
            if c.1 >= K.porch_carry { found = c; break 'ends; }
        } }
        found
    };
    let o_ = way.light.clone().unwrap_or_default();
    let up = if carrier_top > 0.0 { m::min(K.porch_up, r2(carrier_top - K.porch_under)) } else { K.porch_up };
    let world = |o: V3| -> V3 { [mid[0] + s[0] * o[0] + n[0] * o[1], o[2], mid[1] + s[1] * o[0] + n[1] * o[1]] };
    let p_off: V3 = say(&mut src, "porch.offset", o_.porch_offset, || [carrier_al, r2(face + K.porch_out), up]);
    let p_aim: V3 = o_.porch_aim.unwrap_or(K.porch_aim);
    let fix_at: Option<V3> = say(&mut src, "porch.fixture", o_.porch_fixture_at.map(Some), || if carrier_top > 0.0 { Some([carrier_al, r2(face + K.fix_out), r2(up - K.fix_down)]) } else { None });
    let f_off: V3 = say(&mut src, "fill.offset", o_.fill_offset, || [0.0, r2(m::max(face + 1.0, m::min(face + K.fill_out, face + out - K.fill_back))), K.fill_up]);
    let f_aim: V3 = o_.fill_aim.unwrap_or(K.fill_aim);
    let lamps = Lamps { porch: Porch { offset: p_off, aim: p_aim, pos: world(p_off), fixture: fix_at.map(|at| Fixture { at, pos: world(at) }) }, fill: Fill { offset: f_off, aim: f_aim, pos: world(f_off) } };
    if got && !is_gap && top > 0.0 && top < K.porch_up - 0.4 { problems.push(format!("the wall at the anchor is {} m tall: the porch lamp hangs at {} m", js_num(top), js_num(up))); }
    let keep0: Vec<String> = say(&mut src, "keepLights", way.keep_lights.clone(), || {
        let mut o = uniq(level.lights.iter().filter(|l| !l.dir && l.group != "exterior" && !covered(bs, l.pos[0], l.pos[2], l.pos[1])).map(|l| l.group.clone()));
        js_sort(&mut o);
        o
    });
    // (…and the way's word for it, `litInside`: the room behind the anchor. Not the default: a staged door with a lit room behind it would change its picture. And the lamps the way adds: hung for the picture, so lit in it)
    let mut more: Vec<String> = Vec::new();
    if way.lit_inside { for l in lit_inside(&level.lights, mid, n, back) { if l.group != "exterior" { more.push(l.group.clone()); } } }
    if way.set.is_none() { if let Some(x) = &way.add { for l in &x.lamps { if l.group != "exterior" { more.push(l.group.clone()); } } } }
    let keep_lights = if more.is_empty() { keep0 } else { let mut o = uniq(keep0.into_iter().chain(more)); js_sort(&mut o); o };
    // (`dayLights` names the way's own lamps: a name that is none of them keeps nothing burning)
    {
        let no_lamps: &[PlanLight] = &[];
        let own: &[PlanLight] = if way.set.is_some() { &level.lights } else { way.add.as_ref().map_or(no_lamps, |x| x.lamps.as_slice()) };
        for nm in way.day_lights.iter().flatten() { if !own.iter().any(|l| !l.dir && (l.id == *nm || l.group == *nm)) { problems.push(format!("dayLights names '{}': no lamp of the way's own (its set's lights, its `add.lamps`) by that id or group", nm)); } }
    }

    let dd = if kind == "door" { d } else { None };
    let wall = Wall { face: r2(face), back: r2(back), thickness: r2(thickness), top, boxes: hits.iter().map(|&i| bs[i].id.clone()).collect() };
    let lane = Lane { out: lane_out, a: lane_at(l0), b: lane_at(l1), free: lane_free, clear_m: r2(clear_m) };
    let anchor = Anchor { door: dd.map(|x| x.id.clone()), a, b, mid, t, n, width, steel: dd.map_or(false, |x| x.steel), locked: dd.map_or(false, |x| x.locked), exterior: dd.map_or(false, |x| x.exterior), kind };
    let slots_final: Vec<V3> = places.iter().map(|p| [r2(p[0]), r2(p[1]), m::round(p[2] * 10.0) / 10.0]).collect();
    Planned { anchor, ref_: rr, side, s, hand, wall, slots: slots_final, slot_src, fit, stepped, slot_gap, lane, out, inside, beyond, pane, occluders, hidden, hidden_props, lamps, keep_lights, indoors, covered: n_cov, cam, src, problems, set: None }
}
