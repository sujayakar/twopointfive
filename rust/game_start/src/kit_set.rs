//! THE KIT PICTURE'S SET LOADER: `set_level(level, way) → level'`. Where the way being staged names a BESPOKE SET, the picture loads the set's boxes, doors and lights INSTEAD of the level's geometry
//! and keeps what makes the level the level. A way with no set gets the very object it handed in, so nothing downstream can differ. Pure, as the planner beside it: nothing handed in is written to.
//!
//! KEPT, the level's own: `name` (what the weather's snow levels and the look's door paint are keyed by), `version`, `meta`, `floors`, `materials` (the set's boxes take the level's paint by name),
//! `surfaces`, `sky`, `daylight` (less the lamps it names that are gone), every directional light and `scenarios.issue`. The hour and the weather are the link's and are not touched here.
//! SWAPPED: `boxes`, `doors` and the lamps are the set's. Box i of the set is box i of the level that loads (the plan's `hidden` indexes hold).
//! GONE: props, switchables, routes, chokepoints, civilians, volumes, devices, vents, connectors, beacons, the clear and breach plans, and every scenario block but the two below: each names a place
//! in the level that is no longer there.
//!
//! STOOD IN, what the level format and the room clear's own check ask of any level, satisfied here and nowhere else:
//!   · the breaker's two boxes: two 10 cm boxes sunk 30 m under the floor, after the set's boxes;
//!   · the mission's three objectives, on the stand-in room below (nobody plays the solo thread in a picture);
//!   · `scenarios.roomClear`: the picture is a room clear. Its `team` is the plan's five places on the set's `ref` side, repeated as `starts` so that way k of the block is still way k of the game; ONE room:
//!     a 2 m square of free floor a man can walk to from the team, behind the anchor where there is any, 3 m at least from every place of the team; one man on it; its two doors are the set's own (the one
//!     leaf named twice where the set has one) and where the set has no leaf at all ONE stand-in leaf in the corner of the world furthest from the anchor.
//! WHAT THIS CANNOT SEE: the page. That the set is drawn, that its doorway can be walked, and anything the child still reads off a level's own names.
use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering;
use game::m;

use crate::json::{self, Json};
use crate::kit_plan::{self, Planned, Way, PlanBox, PlanOpts, PlanSet, V2, V3};
use game::kitstage::KIT_PLAN;

/// the stand-ins' names and numbers (the head)
pub struct KitSetNumbers {
    pub panel: &'static str, pub led: &'static str, pub door: &'static str, pub room: &'static str,
    /// the breaker's boxes: this far under the floor, this big (half) · the stand-in leaf: its width, in from the world's corner
    pub sunk: f64, pub stub: f64, pub leaf: f64, pub corner: f64,
    /// the stand-in room: its half-size · the man is put this far behind the wall's far face where he can be · no nearer any place of the team than
    pub room_half: f64, pub behind: f64, pub apart: f64,
}
pub const KIT_SET: KitSetNumbers = KitSetNumbers { panel: "kitset_breaker_panel", led: "kitset_breaker_led", door: "kitset_door", room: "kitset_room", sunk: -30.0, stub: 0.05, leaf: 0.86, corner: 2.0, room_half: 1.0, behind: 1.75, apart: 3.0 };
const K: &KitSetNumbers = &KIT_SET;

/// A LINK'S WORDS, as the page's own query reader has them: in order, a key as many times as it is written.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Query(pub Vec<(String, String)>);
impl Query {
    /// a link's search text, with or without its `?`: `+` is a space and %XX a byte, as the language's reader decodes them
    pub fn parse(text: &str) -> Query {
        let t = text.strip_prefix('?').unwrap_or(text);
        Query(t.split('&').filter(|p| !p.is_empty()).map(|p| match p.split_once('=') { Some((k, v)) => (decode(k), decode(v)), None => (decode(p), String::new()) }).collect())
    }
    pub fn has(&self, k: &str) -> bool { self.0.iter().any(|(n, _)| n == k) }
    pub fn get(&self, k: &str) -> Option<&str> { self.0.iter().find(|(n, _)| n == k).map(|(_, v)| v.as_str()) }
    pub fn delete(&mut self, k: &str) { self.0.retain(|(n, _)| n != k); }
    /// the first of the key takes the value and the others go; none: it joins the end
    pub fn set(&mut self, k: &str, v: &str) {
        match self.0.iter().position(|(n, _)| n == k) {
            Some(i) => { self.0[i].1 = String::from(v); let mut j = 0; self.0.retain(|(n, _)| { let keep = n != k || j == i; j += 1; keep }); }
            None => self.0.push((String::from(k), String::from(v))),
        }
    }
}
fn decode(s: &str) -> String {
    let b = s.as_bytes();
    let hex = |c: u8| (c as char).to_digit(16);
    let (mut o, mut i) = (Vec::with_capacity(b.len()), 0);
    while i < b.len() {
        match b[i] {
            b'+' => { o.push(b' '); i += 1; }
            b'%' if i + 2 < b.len() && hex(b[i + 1]).is_some() && hex(b[i + 2]).is_some() => { o.push((hex(b[i + 1]).unwrap() * 16 + hex(b[i + 2]).unwrap()) as u8); i += 3; }
            c => { o.push(c); i += 1; }
        }
    }
    String::from_utf8_lossy(&o).into_owned()
}
/// `String.prototype.trim`: the language's white space and line ends, its list and not Unicode's
fn js_trim(s: &str) -> &str {
    let white = |c: char| matches!(c, '\u{9}' | '\u{a}' | '\u{b}' | '\u{c}' | '\u{d}' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}');
    s.trim_matches(white)
}

/// THE WAY the page will stage, as the game folds the link's `way`: absent / 1 = the level's own (0), N = its Nth, one past the last = the last; 'any' (a dealt way: no picture) = 0.
pub fn way_of_query(way: Option<&str>, data: &Json) -> usize {
    let r = data.get("scenarios").get("roomClear");
    let n = if truthy(r.get("team")) { 1.0 + r.get("starts").items().len() as f64 } else { 1.0 };
    let w = match way { Some(t) if !t.is_empty() && t.bytes().all(|c| c.is_ascii_digit()) => t.parse::<f64>().unwrap_or(0.0), _ => 0.0 };
    if w >= 1.0 { (m::min(n, w) - 1.0) as usize } else { 0 }
}
/// THE WAY THE PICTURE STAGES, off the whole link: only a room clear stands its team on a way in of its own. A rescue, the night breach, a quiet mission and the solo night have the level's one start:
/// way 0, whatever `way` the link still carries (a rescue's link with `way=2` laid the second way's pieces under a picture staged on the first).
pub fn way_of_link(q: &Query, data: &Json) -> usize {
    let room_clear = q.has("roomclear") || (q.has("squad4") && !["nightbreach", "rescue", "quiet"].iter().any(|k| q.has(k)));
    if room_clear { way_of_query(q.get("way"), data) } else { 0 }
}
/// the set a way names
#[derive(Clone, Copy, Debug)]
pub struct SetOf<'a> { pub name: &'a str, pub set: &'a Json, pub way: &'a Json }
/// the set the way names, or none (no block, no such way, no `set` on it, or a set the block does not have: the level itself is staged)
pub fn set_of(data: &Json, way: usize) -> Option<SetOf<'_>> {
    let k = data.get("scenarios").get("kitView");
    let y = match k.get("ways") { Json::Arr(w) => w.get(way)?, _ => return None };
    let name = y.get("set").str()?;
    let t = k.get("sets").get(name);
    if t.is_null() { return None; }
    Some(SetOf { name, set: t, way: y })
}
/// THE QUERY a set is staged with: the picture is a room clear whatever the mission's own kind, so the kind's key is swapped for `roomclear` and every other key is left as it is. THE TEAM'S SIZE IS THE
/// MISSION'S: a quiet mission is Sam alone unless `squad=N` asks for a partner, and then one partner, and it never read `squad4`; a link that names no mission is Sam alone. A room clear with no size key
/// is Sam and two, so those two say their size outright. A copy: the page's own query is not written to.
pub fn set_query(q: &Query) -> Query {
    let mut o = q.clone();
    let loud = ["roomclear", "nightbreach", "rescue"].iter().any(|k| q.has(k));
    let quiet = !loud && q.has("quiet");
    let none = !loud && !quiet && !q.has("squad4");
    if quiet || none {
        let t = js_trim(q.get("squad").unwrap_or(""));
        let n = if quiet && t.len() == 1 && matches!(t.as_bytes()[0], b'1'..=b'4') { "1" } else { "0" };
        o.delete("squad4");
        o.set("squad", n);
    }
    for k in ["rescue", "nightbreach", "quiet", "solo"] { o.delete(k); }
    if !o.has("roomclear") { o.set("roomclear", ""); }
    o
}
/// which quiet kind a mission's link is
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuietKind { Quiet, Solo }
/// WHICH QUIET KIND a mission's link is: a quiet mission, alone or as two; a link that names no mission (the solo night); none: a loud one. Read off the link and never off the scenario that is up: a set
/// way's picture is staged as a room clear (`set_query` strips the words), so the game's own quiet is off there.
pub fn quiet_kind(q: &Query) -> Option<QuietKind> {
    let loud = ["roomclear", "nightbreach", "rescue"].iter().any(|k| q.has(k));
    let quiet = !loud && q.has("quiet");
    let none = !loud && !quiet && !q.has("squad4");
    if quiet { Some(QuietKind::Quiet) } else if none { Some(QuietKind::Solo) } else { None }
}
/// what a set's men wear: `suit` the sneaking suit's word (none: Sam alone in it, as the plain night has him), `sling` where the rifle goes
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dress { pub suit: Option<&'static str>, pub sling: f64 }
/// THE DRESS a set's men wear is THE MISSION'S, not the room clear's it is staged as, read off the mission's own link. A quiet mission: the sneaking suit on Sam and the team; no mission: Sam alone in it;
/// both with the rifle across the back. A loud mission's link: none, and the scenario that is up decides.
pub fn set_dress(q: &Query) -> Option<Dress> {
    match quiet_kind(q) { Some(QuietKind::Quiet) => Some(Dress { suit: Some("quiet"), sling: 2.0 }), Some(QuietKind::Solo) => Some(Dress { suit: None, sling: 2.0 }), None => None }
}

/// LOOK TWINS: a set box's `material` may name alternatives, 'a|b|…': the first the file's palette holds is the one it takes, so one block text serves a level's plain file and its look file. No '|': the box as it is.
pub fn set_paint(b: &Json, materials: &Json) -> Json {
    let t = match b.get("material").str() { Some(t) if t.contains('|') => t, _ => return b.clone() };
    let pick = t.split('|').find(|x| materials.has(x)).or_else(|| t.split('|').next()).unwrap_or("");
    let mut o = b.clone();
    o.set_member("material", json::s(pick));
    o
}
fn truthy(j: &Json) -> bool {
    match j { Json::Null => false, Json::Bool(x) => *x, Json::Num(x) => *x != 0.0 && *x == *x, Json::Str(s) => !s.is_empty(), _ => true }
}

/// A LEVEL'S OWN WAY WITH A PIECE OR A LAMP MORE: the level the picture loads for `way` is the level itself with the way's boxes and lamps laid in AFTER its own (box i of the level is still box i; a box's
/// look alternatives resolved as a set's are), and nothing else touched. A mission to play never comes here, so the pieces are never in it. No `add` on that way, a way on a set, no block: the very level handed in.
pub fn add_level(data: &Json, way: usize) -> Cow<'_, Json> {
    let k = data.get("scenarios").get("kitView");
    let ways = k.get("ways");
    let y = match ways { Json::Arr(w) => match w.get(way) { Some(y) => y, None => return Cow::Borrowed(data) }, _ => return Cow::Borrowed(data) };
    let a = y.get("add");
    if y.has("set") || a.is_null() || truthy(a.get("laid")) { return Cow::Borrowed(data); }
    let has = |list: &Json, id: Option<&str>| list.items().iter().any(|q| q.get("id").str() == id);
    let nb: Vec<&Json> = a.get("boxes").items().iter().filter(|b| !has(data.get("boxes"), b.get("id").str())).collect();
    let nl: Vec<&Json> = a.get("lamps").items().iter().filter(|l| !has(data.get("lights"), l.get("id").str())).collect();
    if nb.is_empty() && nl.is_empty() { return Cow::Borrowed(data); }
    // (the block that rides on it says so, `add.laid`: and no other way's `add` rides along: one way is laid in a level, ever, so two ways may write a piece under the same id, and left on the block the other
    // way's records would be judged against a level that now holds that id)
    let new_ways: Vec<Json> = ways.items().iter().enumerate().map(|(i, y)| {
        if i == way { let mut add = y.get("add").clone(); add.set_member("laid", json::b(true)); let mut o = y.clone(); o.set_member("add", add); o }
        else if y.has("add") { y.without("add") } else { y.clone() }
    }).collect();
    let mut block = k.clone();
    block.set_member("ways", json::arr(new_ways));
    let mut scen = data.get("scenarios").clone();
    scen.set_member("kitView", block);
    let materials = data.get("materials");
    let mut o = data.clone();
    o.set_member("boxes", json::arr(data.get("boxes").items().iter().cloned().chain(nb.iter().map(|b| set_paint(b, materials))).collect()));
    o.set_member("lights", json::arr(data.get("lights").items().iter().cloned().chain(nl.iter().map(|l| (*l).clone())).collect()));
    o.set_member("scenarios", scen);
    Cow::Owned(o)
}

/// THE LEVEL THE PICTURE LOADS for `way` (an index into the block's ways). No set on that way: the very level handed in.
pub fn set_level(data: &Json, way: usize) -> Cow<'_, Json> {
    let hit = match set_of(data, way) { Some(h) => h, None => return Cow::Borrowed(data) };
    let (t, y) = (hit.set, Way::of(hit.way));
    let n = &KIT_PLAN.nav;
    let (wx, wz) = (n.w as f64 * n.cell, n.h as f64 * n.cell);
    let set = PlanSet::of(t);
    let p = kit_plan::plan(&set.level, &y, set.ref_, &PlanOpts::default());
    // the doors: the set's (the anchor's first), else one stand-in leaf in the corner furthest from the anchor
    let anchor_door = p.anchor.door.as_deref();
    let mut own: Vec<&Json> = t.get("doors").items().iter().collect();
    let on_anchor = |d: &Json| anchor_door.is_some() && d.get("id").str() == anchor_door;
    own.sort_by(|a, b| (on_anchor(b) as i32 - on_anchor(a) as i32).cmp(&0));
    let mut corners: [V2; 4] = [[K.corner, K.corner], [wx - K.corner, K.corner], [K.corner, wz - K.corner], [wx - K.corner, wz - K.corner]];
    corners.sort_by(|a, b| { let d = m::hyp2(b[0] - p.anchor.mid[0], b[1] - p.anchor.mid[1]) - m::hyp2(a[0] - p.anchor.mid[0], a[1] - p.anchor.mid[1]); if d < 0.0 { Ordering::Less } else if d > 0.0 { Ordering::Greater } else { Ordering::Equal } });
    let far = corners[0];
    let doors: Json = if !own.is_empty() { t.get("doors").clone() } else { json::arr(vec![json::obj(vec![("id", json::s(K.door)), ("hinge", json::nums(&far)), ("closedDir", json::n(0.0)), ("width", json::n(K.leaf))])]) };
    let two: [String; 2] = if !own.is_empty() { let first = own[0]; [id_of(first), id_of(*own.get(1).unwrap_or(&first))] } else { [String::from(K.door), String::from(K.door)] };
    // the team: the plan's five places
    let at = |q: &V3| json::nums(&[q[0], 0.0, q[1]]);
    let rad = m::PI / 180.0;
    let sam = at(&p.slots[0]);
    let team = json::obj(vec![("sam", sam.clone()), ("samYaw", json::n(p.slots[0][2] * rad)), ("ops", json::arr(p.slots[1..].iter().map(at).collect())), ("opsYaw", json::n(p.slots[1][2] * rad))]);
    // the stand-in room: a free cell a man can walk to from Sam's, nearest the spot behind the anchor
    let (pos, aim_at) = stand_in(&set.level.boxes, &p);
    let (x0, x1, z0, z1) = (pos[0] - K.room_half, pos[0] + K.room_half, pos[2] - K.room_half, pos[2] + K.room_half);
    let bounds = json::obj(vec![("x0", json::n(x0)), ("x1", json::n(x1)), ("z0", json::n(z0)), ("z1", json::n(z1))]);
    let was = data.get("scenarios").get("roomClear");
    let ways = data.get("scenarios").get("kitView").get("ways");
    let was_name = was.get("name");
    let mut rc: Vec<(&str, Json)> = vec![("name", if was_name.is_null() { data.get("name").clone() } else { was_name.clone() }), ("team", team.clone())];
    if ways.items().len() > 1 { rc.push(("starts", json::arr(ways.items()[1..].iter().map(|_| team.clone()).collect()))); }
    rc.push(("rooms", json::arr(vec![json::obj(vec![("id", json::s(K.room)), ("name", json::s("the set")), ("bounds", bounds.clone()), ("doors", json::arr(two.iter().map(|id| json::obj(vec![("id", json::s(id)), ("from", json::s("outside"))])).collect())), ("garrison", json::n(1.0)),
        ("marks", json::arr(vec![json::obj(vec![("pos", json::nums(&pos)), ("watch", json::s(&two[0])), ("aimAt", json::nums(&aim_at))])]))])])));
    let room_clear = json::obj(rc);
    let stub = |id: &str, k: f64| json::obj(vec![("id", json::s(id)), ("c", json::nums(&[far[0] + k * 4.0 * K.stub, K.sunk, far[1]])), ("h", json::nums(&[K.stub, K.stub, K.stub])), ("albedo", json::nums(&[0.0, 0.0, 0.0])), ("flags", json::arr(vec![json::s("NoPrimary"), json::s("NoShadow")]))]);
    let lights: Vec<Json> = data.get("lights").items().iter().filter(|l| l.get("kind").str() == Some("dir")).chain(t.get("lights").items().iter()).cloned().collect();
    let lit: Vec<&str> = lights.iter().filter_map(|l| l.get("id").str()).collect();
    let materials = data.get("materials");
    let mut boxes: Vec<Json> = t.get("boxes").items().iter().map(|b| set_paint(b, materials)).collect();
    boxes.push(stub(K.panel, 0.0));
    boxes.push(stub(K.led, 1.0));
    let mut o: Vec<(&str, Json)> = Vec::new();
    for k in ["version", "name", "meta", "floors", "materials"] { if data.has(k) { o.push((k, data.get(k).clone())); } }
    if truthy(data.get("surfaces")) { o.push(("surfaces", data.get("surfaces").clone())); }
    o.push(("boxes", json::arr(boxes)));
    o.push(("props", json::arr(Vec::new())));
    o.push(("lights", json::arr(lights.clone())));
    o.push(("switchables", json::arr(Vec::new())));
    o.push(("doors", doors));
    for k in ["routes", "chokepoints", "volumes", "devices", "connectors"] { o.push((k, json::arr(Vec::new()))); }
    o.push(("mission", json::obj(vec![("playerSpawn", sam), ("objectives", json::arr(vec![
        json::obj(vec![("id", json::s("reach_room")), ("kind", json::s("reach")), ("area", bounds), ("door", json::s(&two[0]))]),
        json::obj(vec![("id", json::s("pull_drive")), ("kind", json::s("drive")), ("rackIndex", json::n(0.0)), ("front", json::nums(&pos)), ("halfW", json::n(0.0))]),
        json::obj(vec![("id", json::s("exfil")), ("kind", json::s("exfil")), ("door", json::s(&two[0])), ("x", json::n(far[0])), ("z", json::n(far[1]))]),
    ]))])));
    o.push(("breaker", json::obj(vec![("pos", json::nums(&[far[0], K.sunk, far[1]])), ("panel", json::s(K.panel)), ("led", json::s(K.led))])));
    o.push(("beacons", json::arr(Vec::new())));
    if data.has("sky") { o.push(("sky", data.get("sky").clone())); }
    if truthy(data.get("daylight")) {
        let mut dl = data.get("daylight").clone();
        dl.set_member("lightsOff", json::arr(data.get("daylight").get("lightsOff").items().iter().filter(|id| id.str().map_or(false, |x| lit.contains(&x))).cloned().collect()));
        o.push(("daylight", dl));
    }
    o.push(("plans", json::obj(vec![("roomClear", json::arr(Vec::new()))])));
    // (every way of the block on THIS set keeps its place; a way that is the level's own, or another set's, stands in as the staged one: its door is gone)
    let mut scen: Vec<(&str, Json)> = vec![("kitView", json::obj(vec![("ways", json::arr(ways.items().iter().map(|w| if w.get("set").str() == Some(hit.name) { w.clone() } else { hit.way.clone() }).collect())), ("sets", json::obj(vec![(hit.name, t.clone())]))])), ("roomClear", room_clear)];
    if truthy(data.get("scenarios").get("issue")) { scen.push(("issue", data.get("scenarios").get("issue").clone())); }
    o.push(("scenarios", json::obj(scen)));
    Cow::Owned(json::obj(o))
}
fn id_of(d: &Json) -> String { String::from(d.get("id").str().expect("kit set: a door's id")) }

/// where the stand-in man stands and the point his gun rests on: of the free cells joined on foot to Sam's place with a free cell beside them, `apart` from every place of the team, the nearest to the spot
/// `behind` m behind the wall's far face (no such cell: beside Sam, and the game's check says so)
fn stand_in(boxes: &[PlanBox], p: &Planned) -> (V3, V3) {
    let n = &KIT_PLAN.nav;
    let fl = kit_plan::floor(boxes, None);
    let (w, h) = (n.w as i64, n.h as i64);
    let mid = |i: i64| (i as f64 + 0.5) * n.cell;
    let free = |x: i64, z: i64| x >= 0 && z >= 0 && x < w && z < h && !fl.blocked(mid(x), mid(z));
    let (sx, sz) = (m::floor(p.slots[0][0] / n.cell), m::floor(p.slots[0][1] / n.cell));
    let (sx, sz) = (sx as i64, sz as i64);
    let mut seen = vec![false; n.w * n.h];
    let mut todo: Vec<(i64, i64)> = Vec::new();
    if free(sx, sz) { seen[(sz * w + sx) as usize] = true; todo.push((sx, sz)); }
    let four: [(i64, i64); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut i = 0;
    while i < todo.len() {
        for (dx, dz) in four { let (x, z) = (todo[i].0 + dx, todo[i].1 + dz); if free(x, z) && !seen[(z * w + x) as usize] { seen[(z * w + x) as usize] = true; todo.push((x, z)); } }
        i += 1;
    }
    let (am, an) = (p.anchor.mid, p.anchor.n);
    let want: V2 = [am[0] - an[0] * (K.behind - p.wall.back), am[1] - an[1] * (K.behind - p.wall.back)];
    let mut best: Option<(V3, V3)> = None;
    let mut best_d = m::INF;
    for &(x, z) in &todo {
        let (px, pz) = (mid(x), mid(z));
        let d = m::hyp2(px - want[0], pz - want[1]);
        if d >= best_d || p.slots.iter().any(|q| m::hyp2(q[0] - px, q[1] - pz) < K.apart) { continue; }
        let nb = four.iter().find(|(dx, dz)| free(x + dx, z + dz) && seen[((z + dz) * w + x + dx) as usize]);
        let (dx, dz) = match nb { Some(v) => *v, None => continue };
        best = Some(([px, 0.0, pz], [mid(x + dx), 1.2, mid(z + dz)]));
        best_d = d;
    }
    let s0 = p.slots[0];
    best.unwrap_or(([s0[0], 0.0, s0[1]], [s0[0] + an[0] * -0.5, 1.2, s0[1] + an[1] * -0.5]))
}
