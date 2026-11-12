//! WHAT IS WRONG WITH A LEVEL FILE, each fault by its place in it: the one judge. The reading's own faults first (`file.rs`: a value that is not what its place wants, a member the game has no reading
//! of); where the file reads whole, the rules a file can be held to by itself: ids that are one thing's, names that point at something, numbers in their range, lists that go together.
//!
//! It judges the FILE. What the built floor says (a mark on free floor, a way on foot, a light in a wall) is asked of the game: `levels`, and tools/qa/level-lint.ts.
use crate::file::{self, BoxData, DoorPaintData, FlagName, LevelFile, LightData, Plan};
use crate::json::Json;
use crate::kit_file::*;
use crate::level::{stuff_word, Said};
use game::scene_materials::DoesWord;
use crate::passes::{planes_of, Form, ARC_PLANES};
use crate::read::Faults;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::{format, string::String, vec::Vec};
use game::m;
use game::scene_cut::{cut_plane, cut_record, MAX_CUT_PLANES};
use game::scene_level::{BreachPlanBounds, DeviceDef, RoomClearPlan};
use game::scene_markings::{MarkFace, MarkRes, MarkingDef, MARKING_TEXEL, MAX_CELLS_H, MAX_CELLS_W};
use game::scene_scenarios::*;
use game::scene_shapes::ShapeFormArc;
use game::V3;

/// a box's marks, a mark's paints, and how the paints are said in its cells
const MARKS_MOST: usize = 4;
const PAL_MOST: usize = 3;
const CELLS: &str = ".abc";
/// routes a quiet mission mans at once, at most
const QUIET_ROUTES: usize = 6;

/// Every fault of the file, none where it is a level.
pub fn of(data: &Json) -> Vec<String> {
    let (read, faults) = file::read(data);
    match read { Some(f) => Rules::of(data, &f).all(), None => faults }
}
/// …of a file's text
pub fn of_text(text: &str) -> Vec<String> {
    match crate::json::parse(text) { Ok(data) => of(&data), Err(e) => Vec::from([e]) }
}

struct Rules<'a> {
    f: &'a LevelFile,
    out: Faults,
    boxes: BTreeSet<&'a str>, doors: BTreeSet<&'a str>, lights: BTreeSet<&'a str>,
    /// each switchable's boxes, by whose they are
    switched: BTreeMap<&'a str, &'a str>,
    /// the lights' and the switchables' groups
    groups: BTreeSet<&'a str>,
}

fn whole(x: f64) -> bool { x == m::floor(x) }
fn hyp(v: &[f64]) -> f64 { m::sqrt(v.iter().map(|x| x * x).sum()) }
fn unit2(v: [f64; 2]) -> bool { m::abs(hyp(&v) - 1.0) < 1e-3 }
fn some(v: &V3) -> bool { hyp(v) > 1e-9 }
fn light(v: &V3) -> bool { v.iter().all(|x| *x >= 0.0) }
fn rect(x0: f64, x1: f64, z0: f64, z1: f64) -> bool { x0 <= x1 && z0 <= z1 }
fn is_rect(r: &Rect) -> bool { rect(r.x0, r.x1, r.z0, r.z1) }
fn bounds(b: &Option<BreachPlanBounds>) -> bool { b.as_ref().is_some_and(|b| rect(b.x0, b.x1, b.z0, b.z1)) }
fn inside(p: &V3, r: &Rect) -> bool { p[0] >= r.x0 && p[0] <= r.x1 && p[2] >= r.z0 && p[2] <= r.z1 }
fn named(i: usize, list: &str, id: &str) -> String { format!("{list}[{i}] '{id}'") }

/// WHAT IS WRONG WITH A FORM said of a box of half size h (a box's shape, or a piece of trim's): nothing = none. A plane is judged against the box alone: whether it still bites beside an arc's own
/// facets is not asked.
fn form_faults(h: V3, round: Option<f64>, arc: Option<ShapeFormArc>, cut: Option<&[[f64; 4]]>, old: Option<bool>) -> Vec<String> {
    let mut out = Vec::new();
    let cuts = cut.map_or(0, <[_]>::len);
    let mut formed = true;
    if old == Some(false) { out.push(String::from("old is true, or left out")); }
    if round.is_some() && arc.is_some() { out.push(String::from("says one of round and arc")); }
    if cut.is_some_and(|c| c.is_empty() || c.iter().any(|p| m::sqrt(p[0] * p[0] + p[1] * p[1] + p[2] * p[2]) <= 1e-9)) { out.push(String::from("cut: planes, each a way it faces and how far out")); formed = false; }
    if let Some(axis) = round {
        if axis != 1.0 && axis != 3.0 { out.push(String::from("round is the axis 1 (x) or 3 (z): a cylinder on its end is a round thing of the kit")); formed = false; }
        else {
            let a = if axis == 1.0 { h[2] } else { h[0] };
            if m::abs(h[1] - a) > 0.15 * m::min(h[1], a) + 1e-9 { out.push(format!("round {axis}: the box is square across that axis, or within 15 hundredths of it ({} by {a})", h[1])); }
        }
        if cuts > 0 { out.push(String::from("round takes every plane of the record: no cut beside it")); }
    } else if let Some(a) = arc {
        if a.axis != 1.0 && a.axis != 3.0 || a.rise <= 0.0 { out.push(String::from("arc: the axis 1 or 3, and a rise over 0")); formed = false; }
        else {
            let w = if a.axis == 1.0 { h[2] } else { h[0] };
            if a.rise > 2.0 * h[1] + 1e-6 { out.push(format!("arc: a rise of {} in a box {} high", a.rise, 2.0 * h[1])); }
            if a.rise > w + 1e-6 { out.push(format!("arc: a rise of {} over a half width of {w} (a half circle is the most)", a.rise)); }
        }
        if cuts > ARC_PLANES { out.push(format!("an arc leaves room for {ARC_PLANES} planes ({cuts})")); }
    } else if cuts as f64 > MAX_CUT_PLANES { out.push(format!("at most {MAX_CUT_PLANES} planes ({cuts})")); }
    if !formed || !out.is_empty() { return out; }
    for (i, q) in cut.into_iter().flatten().enumerate() {
        let r = cut_record(h, &[cut_plane([q[0], q[1], q[2]], q[3])]);
        if r.empty { out.push(format!("cut[{i}] leaves nothing of the box")); }
        else if r.cut.map_or(true, |c| c.is_empty()) { out.push(format!("cut[{i}] cuts nothing of the box")); }
    }
    if out.is_empty() && (cuts > 0 || arc.is_some() || round.is_some()) && cut_record(h, &planes_of(h, &Form { round, arc, cut, old })).empty { out.push(String::from("the form leaves nothing of the box")); }
    out
}

impl<'a> Rules<'a> {
    fn of(data: &'a Json, f: &'a LevelFile) -> Faults {
        let mut r = Rules {
            f, out: Faults::default(),
            boxes: f.boxes.iter().map(|b| b.id.as_str()).collect(), doors: f.doors.iter().map(|d| d.id.as_str()).collect(), lights: f.lights.iter().map(|l| l.id.as_str()).collect(),
            switched: BTreeMap::new(), groups: f.lights.iter().map(|l| l.group.as_str()).chain(f.switchables.iter().map(|s| s.group.as_str())).collect(),
        };
        r.names(data);
        r.ground();
        r.furniture();
        r.ways();
        r.machines();
        r.mission();
        r.plans();
        if let Some(s) = &f.scenarios {
            let said = data.get("scenarios");
            r.scenarios(&s.kept, said);
            if let Some(k) = &s.kit_view { r.kit_view(k, &s.kept, said.get("kitView")); }
        }
        r.out
    }
    fn say(&self, place: impl core::fmt::Display, words: impl core::fmt::Display) { self.out.say(place, words); }
    /// a name that is said is not empty
    fn word(&self, place: &str, w: &Option<String>) { if w.as_deref() == Some("") { self.say(place, "a name is not empty"); } }
    fn text(&self, place: impl core::fmt::Display, w: &str) { if w.is_empty() { self.say(place, "is not empty"); } }
    fn door(&self, place: impl core::fmt::Display, id: &str) { if !self.doors.contains(id) { self.say(place, format!("no door of the level is called '{id}'")); } }
    fn a_box(&self, place: impl core::fmt::Display, id: &str) { if !self.boxes.contains(id) { self.say(place, format!("no box of the level is called '{id}'")); } }
    /// each of a list's records has an id of its own
    fn own<'b>(&self, list: &str, ids: impl Iterator<Item = Option<&'b str>>) {
        let mut seen = BTreeSet::new();
        for (i, id) in ids.enumerate() {
            match id.filter(|w| !w.is_empty()) {
                None => self.say(format!("{list}[{i}]"), "says no id"),
                Some(id) => if !seen.insert(id) { self.say(format!("{list}[{i}]"), format!("the id '{id}' is another's already")); }
            }
        }
    }

    // ---- ids, floors, paints

    fn names(&self, data: &Json) {
        let f = self.f;
        self.text("name", &f.name);
        let m = &f.meta;
        if m.grid.is_some_and(|x| x <= 0.0) { self.say("meta.grid", "over 0"); }
        if m.mark_texel.is_some_and(|x| !(0.005..=0.2).contains(&x)) { self.say("meta.markTexel", "metres, 0.005 to 0.2"); }
        if m.kept.is_some_and(|x| !(0.0..=1.0).contains(&x)) { self.say("meta.kept", "a number from 0 to 1"); }
        for list in ["floors", "boxes", "props", "lights", "switchables", "doors", "routes", "chokepoints", "civilians", "volumes", "marks", "devices", "vents", "hums", "beacons"] {
            self.own(list, data.get(list).items().iter().map(|o| o.peek("id").and_then(Json::str)));
        }
        for (list, of) in [("surfaces.zones", data.get("surfaces").get("zones")), ("mission.objectives", data.get("mission").get("objectives")), ("plans.roomClear", data.get("plans").get("roomClear"))] {
            self.own(list, of.items().iter().map(|o| o.peek("id").and_then(Json::str)));
        }
        if f.floors.is_empty() { self.say("floors", "at least one floor"); }
        for (i, s) in f.floors.iter().enumerate() {
            let w = named(i, "floors", &s.id);
            if s.ceiling <= 0.0 { self.say(format!("{w}.ceiling"), "over 0"); }
            self.word(&format!("{w}.name"), &s.name);
        }
        for list in ["boxes", "props", "doors", "chokepoints", "civilians", "volumes", "devices", "vents", "hums"] { self.floors(list, data.get(list)); }
        for (name, m) in f.materials.iter() {
            let w = format!("materials.{name}");
            if !light(&m.albedo) { self.say(format!("{w}.albedo"), "0 or more"); }
            self.finish(&w, m.rough);
            self.words(&w, m.stuff.as_deref(), m.does.as_deref());
        }
        if let Some(p) = &f.door_paint { self.door_paint("doorPaint", p); }
        if let Some(s) = &f.surfaces {
            let b = &s.building;
            if !rect(b.x0, b.x1, b.z0, b.z1) { self.say("surfaces.building", "x0 to x1, z0 to z1"); }
            for (i, z) in s.zones.iter().enumerate() { if !rect(z.x0, z.x1, z.z0, z.z1) { self.say(format!("surfaces.zones[{i}]"), "x0 to x1, z0 to z1"); } }
        }
    }
    /// the floor each of a list's records is on (which the reading lets go) is a floor of the file's
    fn floors(&self, list: &str, of: &Json) {
        for (i, o) in of.items().iter().enumerate() {
            let said = |k: &str| o.peek(k).and_then(Json::str);
            let w = named(i, list, said("id").unwrap_or(""));
            if let Some(on) = said("floor") { if !self.f.floors.iter().any(|s| s.id == on) { self.say(format!("{w}.floor"), format!("the file has no floor '{on}'")); } }
        }
    }
    fn door_paint(&self, w: &str, p: &DoorPaintData) {
        for (side, x) in [("outside", &p.outside), ("inside", &p.inside)] {
            if !light(&x.albedo) { self.say(format!("{w}.{side}.albedo"), "0 or more"); }
            self.words(&format!("{w}.{side}"), x.stuff.as_deref(), None);
        }
    }
    fn finish(&self, w: &str, rough: Option<f64>) { if rough.is_some_and(|r| !(0.0..=1.0).contains(&r)) { self.say(format!("{w}.rough"), "a number from 0 to 1"); } }
    /// what a record says it is made of and does are words there are
    fn words(&self, w: &str, stuff: Option<&str>, does: Option<&str>) {
        if let Some(s) = stuff { if s != "plain" && stuff_word(s).is_none() { self.say(format!("{w}.stuff"), format!("'{s}' is nothing a box is made of")); } }
        if let Some(d) = does { if d != "unnamed" && DoesWord::named(d).is_none() { self.say(format!("{w}.does"), format!("'{d}' is no row of the table of materials that says what a thing does")); } }
    }
    /// one paint: a material of the file, or a colour. `or`: in a Team & kit picture's box a material may be alternatives 'a|b', and the first the file has is it
    fn paint(&self, w: &str, o: &Said, or: bool) {
        match (o.material, &o.albedo) {
            (Some(_), Some(_)) | (None, None) => self.say(w, "says one of material and albedo"),
            (Some(m), None) => {
                let has = |m: &str| self.f.materials.of(m).is_some();
                if !(has(m) || or && m.split('|').any(has)) { self.say(format!("{w}.material"), format!("the file has no material '{m}'")); }
            }
            (None, Some(a)) => if !light(a) { self.say(format!("{w}.albedo"), "0 or more"); }
        }
    }

    // ---- boxes

    fn marks(&self, w: &str, marks: &Option<Vec<MarkingDef>>, h: &V3, tilted: bool) {
        let Some(marks) = marks else { return };
        if marks.len() > MARKS_MOST { self.say(format!("{w}.markings"), format!("at most {MARKS_MOST} a box ({})", marks.len())); }
        if tilted { self.say(format!("{w}.markings"), "a tilted box carries none"); }
        for (i, k) in marks.iter().enumerate() { self.mark(&format!("{w}.markings[{i}]"), k, h); }
    }
    fn mark(&self, w: &str, k: &MarkingDef, h: &V3) {
        let ([u, v], [cols, rows]) = (k.at, k.size);
        if !whole(u) || !whole(v) { self.say(format!("{w}.at"), "whole cells"); }
        let sized = whole(cols) && whole(rows) && (1.0..=MAX_CELLS_W).contains(&cols) && (1.0..=MAX_CELLS_H).contains(&rows);
        if !sized { self.say(format!("{w}.size"), format!("whole cells, 1 to {MAX_CELLS_W} by 1 to {MAX_CELLS_H}")); }
        if k.pal.is_empty() || k.pal.len() > PAL_MOST { self.say(format!("{w}.pal"), format!("1 to {PAL_MOST} paints")); }
        for (i, p) in k.pal.iter().enumerate() {
            let at = format!("{w}.pal[{i}]");
            if p.albedo.is_some_and(|a| !a.iter().all(|x| (0.0..=1.0).contains(x))) { self.say(format!("{at}.albedo"), "from 0 to 1"); }
            if p.mul.is_some_and(|x| !(0.0..=4.0).contains(&x)) { self.say(format!("{at}.mul"), "from 0 to 4"); }
            if p.albedo.is_some() && p.mul.is_some() { self.say(&at, "says one of albedo and mul"); }
            self.finish(&at, p.rough);
            if p.emit.is_some_and(|e| !light(&e)) { self.say(format!("{at}.emit"), "0 or more"); }
        }
        if !sized { return; }
        if k.cells.len() as f64 != rows || k.cells.iter().any(|r| r.chars().count() as f64 != cols) { self.say(format!("{w}.cells"), format!("{rows} rows of {cols} cells, the top row first")); }
        else if let Some(c) = k.cells.iter().flat_map(|r| r.chars()).find(|c| CELLS.find(*c).map_or(true, |i| i > k.pal.len())) { self.say(format!("{w}.cells"), format!("'{c}' is none of its paints")); }
        // (the bitmap lies on its face)
        let t = self.f.meta.mark_texel.unwrap_or(MARKING_TEXEL) * match k.res { Some(MarkRes::Fine) => 0.5, Some(MarkRes::Coarse) => 2.0, _ => 1.0 };
        let (hu, hv) = match k.face { MarkFace::X | MarkFace::X2 => (h[2], h[1]), MarkFace::Y | MarkFace::Y2 => (h[0], h[2]), _ => (h[0], h[1]) };
        let over = |lo: f64, n: f64, half: f64| lo * t < -half - 1e-6 || (lo + n) * t > half + 1e-6;
        if over(u, cols, hu) || over(v, rows, hv) { self.say(w, format!("the bitmap leaves the face (half {hu:.3} by {hv:.3} m)")); }
    }
    /// a round thing: the cylinder about its own y, and nothing that would turn, tilt or cut it
    fn round(&self, w: &str, b: &BoxData) {
        let Some(round) = b.round else { return };
        if !round { self.say(format!("{w}.round"), "true, or left out"); }
        if b.h[0] != b.h[2] { self.say(format!("{w}.round"), "its radius is h[0] and h[2]: they are equal"); }
        for (k, has) in [("yaw", b.yaw.is_some()), ("rot", b.rot.is_some()), ("flags", b.flags.is_some()), ("shape", b.shape.is_some())] { if has { self.say(format!("{w}.round"), format!("a round thing says no {k}")); } }
    }
    /// one box: the level's own, and one a Team & kit way adds for its picture (`or`: see `paint`)
    fn one_box(&self, w: &str, b: &BoxData, or: bool) {
        if !light(&b.h) { self.say(format!("{w}.h"), "0 or more"); }
        if b.rot.is_some_and(|q| m::abs(hyp(&q) - 1.0) >= 1e-3) { self.say(format!("{w}.rot"), "a unit quaternion, x y z w"); }
        let said = Said::from(b);
        self.paint(w, &said, or);
        self.finish(w, b.rough);
        self.words(w, said.stuff, said.does);
        if b.emissive.is_some_and(|e| !light(&e)) { self.say(format!("{w}.emissive"), "0 or more"); }
        self.word(&format!("{w}.group"), &b.group);
        self.marks(w, &b.markings, &b.h, b.rot.is_some());
        self.round(w, b);
        let Some(s) = &b.shape else { return };
        let at = format!("{w}.shape");
        if b.rot.is_some() { self.say(&at, "a tilted box says no shape"); }
        let formed = s.round.is_some() || s.arc.is_some() || s.cut.is_some();
        if formed && b.markings.is_some() { self.say(&at, "a box with markings says no form (a mark is laid out on the whole face): give the marks a box of their own, or say the form of a piece"); }
        if formed && b.flags.iter().flatten().any(|g| *g == FlagName::Glass) { self.say(&at, "a Glass box says no form (a pane breaks and is drawn whole)"); }
        let none = !formed && s.with.is_none();
        if none { self.say(&at, "says nothing"); }
        for e in form_faults(b.h, s.round, s.arc, s.cut.as_deref(), s.old) { self.say(&at, e); }
        if s.with.as_ref().is_some_and(|l| l.is_empty()) { self.say(format!("{at}.with"), "at least one piece"); }
        for (i, q) in s.with.iter().flatten().enumerate() {
            let at = format!("{at}.with[{i}]");
            if !q.h.iter().all(|x| *x > 0.0) { self.say(format!("{at}.h"), "over 0"); continue; }
            for e in form_faults(q.h, q.round, q.arc, q.cut.as_deref(), q.old) { self.say(&at, e); }
            let said = Said::from(q);
            // (a piece's paint, finish and words are a box's, or it says none: it then wears its box's)
            if said.painted() { self.paint(&at, &said, false); self.finish(&at, q.rough); self.words(&at, said.stuff, said.does); }
            else if q.rough.is_some() || q.metal.is_some() || q.stuff.is_some() || q.does.is_some() { self.say(&at, "a piece that says no paint wears its box's finish and words too: no rough, metal, stuff or does"); }
        }
    }
    fn ground(&self) { for (i, b) in self.f.boxes.iter().enumerate() { self.one_box(&named(i, "boxes", &b.id), b, false); } }

    // ---- furniture and lights

    fn furniture(&mut self) {
        let f = self.f;
        let caps = game::levelfit::level_caps();
        let most = |list: &str, n: usize, cap: f64, of: &str| if n as f64 > cap { self.say(list, format!("at most {cap} {of} ({n})")); };
        most("props", f.props.len(), caps.things, "loose things of furniture");
        most("props", f.props.iter().map(|p| p.parts.len()).sum(), caps.parts, "boxes in all the loose furniture");
        for (i, p) in f.props.iter().enumerate() {
            let w = named(i, "props", &p.id);
            if p.mass <= 0.0 { self.say(format!("{w}.mass"), "over 0"); }
            if p.half.is_some_and(|h| h[0] <= 0.0 || h[1] <= 0.0) { self.say(format!("{w}.half"), "over 0"); }
            if p.height.is_some_and(|h| h <= 0.0) { self.say(format!("{w}.height"), "over 0"); }
            self.word(&format!("{w}.name"), &p.name);
            if p.parts.is_empty() { self.say(format!("{w}.parts"), "at least one"); }
            for (j, q) in p.parts.iter().enumerate() {
                let at = format!("{w}.parts[{j}]");
                if !light(&q.h) { self.say(format!("{at}.h"), "0 or more"); }
                let said = Said::from(q);
                self.paint(&at, &said, false);
                self.words(&at, said.stuff, said.does);
                self.marks(&at, &q.markings, &q.h, false);
            }
        }
        for (i, l) in f.lights.iter().enumerate() { self.lamp(&named(i, "lights", &l.id), l, &BTreeSet::new()); }
        for (i, s) in f.switchables.iter().enumerate() {
            let w = named(i, "switchables", &s.id);
            if s.boxes.is_empty() { self.say(format!("{w}.boxes"), "at least one box"); }
            for (j, b) in s.boxes.iter().enumerate() {
                let at = format!("{w}.boxes[{j}]");
                self.a_box(&at, b);
                match self.switched.insert(b, &s.id) {
                    Some(o) if o == s.id => self.say(at, format!("the box '{b}' is listed twice (it would be lit twice over)")),
                    Some(o) => self.say(at, format!("the box '{b}' is the switchable '{o}'s already")),
                    None => {}
                }
            }
            self.text(format!("{w}.group"), &s.group);
            if !light(&s.base_emissive) { self.say(format!("{w}.baseEmissive"), "0 or more"); }
        }
    }
    /// `more`: boxes a fixture may name beside the level's
    fn lamp(&self, w: &str, l: &LightData, more: &BTreeSet<&str>) {
        if !some(&l.dir) { self.say(format!("{w}.dir"), "points somewhere"); }
        if !light(&l.color) { self.say(format!("{w}.color"), "0 or more"); }
        for (k, v) in [("intensity", l.intensity), ("range", l.range), ("innerDeg", l.inner_deg), ("outerDeg", l.outer_deg), ("volumetric", l.volumetric), ("radius", l.radius.unwrap_or(0.0))] {
            if v < 0.0 { self.say(format!("{w}.{k}"), "0 or more"); }
        }
        if l.inner_deg > l.outer_deg { self.say(format!("{w}.innerDeg"), "no more than outerDeg"); }
        self.text(format!("{w}.group"), &l.group);
        if let Some(b) = &l.fixture { if !more.contains(b.as_str()) { self.a_box(format!("{w}.fixture"), b); } }
    }

    // ---- doors, routes, people

    fn ways(&self) {
        let f = self.f;
        let cap = game::levelfit::level_caps().doors;
        if f.doors.len() as f64 > cap { self.say("doors", format!("at most {cap} ({})", f.doors.len())); }
        for (i, d) in f.doors.iter().enumerate() {
            let w = named(i, "doors", &d.id);
            if d.width <= 0.0 { self.say(format!("{w}.width"), "over 0"); }
            for (k, v) in [("swing", d.swing), ("keySide", d.key_side)] { if v.is_some_and(|v| v != 1.0 && v != -1.0) { self.say(format!("{w}.{k}"), "1 or -1"); } }
            if let Some(s) = d.swing { if (if s > 0.0 { d.max_angle.unwrap_or(1.92) } else { -d.min_angle.unwrap_or(-1.92) }) <= 0.0 { self.say(format!("{w}.swing"), "its stop on that side leaves it no opening"); } }
            self.word(&format!("{w}.lock"), &d.lock);
        }
        for (i, r) in f.routes.iter().enumerate() {
            let w = named(i, "routes", &r.id);
            self.word(&format!("{w}.name"), &r.name);
            self.walk(&w, r.points.len(), &r.wait);
            for k in r.keys.iter().flatten() { self.text(format!("{w}.keys"), k); }
        }
        for (i, c) in f.chokepoints.iter().enumerate() { self.word(&format!("chokepoints[{i}].name"), &c.name); }
        for (i, c) in f.civilians.iter().flatten().enumerate() {
            let w = format!("civilians[{i}] ({})", c.name);
            if !light(&c.tint) { self.say(format!("{w}.tint"), "0 or more"); }
            if c.tint2.is_some_and(|t| !light(&t)) { self.say(format!("{w}.tint2"), "0 or more"); }
            self.text(format!("{w}.name"), &c.name);
            self.walk(&w, c.points.len(), &c.wait);
            for (k, n) in [("tasks", c.tasks.as_ref().map(Vec::len)), ("face", c.face.as_ref().map(Vec::len))] { if n.is_some_and(|n| n != c.points.len()) { self.say(format!("{w}.{k}"), "one a point"); } }
        }
    }
    /// a loop walked: a point at least, and a wait at each
    fn walk(&self, w: &str, points: usize, wait: &[f64]) {
        if points == 0 { self.say(format!("{w}.points"), "at least one"); }
        if wait.len() != points { self.say(format!("{w}.wait"), format!("one a point ({} for {points})", wait.len())); }
        if wait.iter().any(|x| *x < 0.0) { self.say(format!("{w}.wait"), "0 or more"); }
    }

    // ---- devices, vents

    fn machines(&self) {
        let f = self.f;
        let caps = game::levelfit::level_caps();
        let (mut cameras, mut lasers, mut pads) = (0, 0, BTreeMap::new());
        for (i, d) in f.devices.iter().enumerate() {
            match d {
                DeviceDef::Camera(c) => {
                    cameras += 1;
                    let w = named(i, "devices", &c.name);
                    self.word(&format!("{w}.group"), &c.group);
                    if c.pitch.is_some_and(|p| m::abs(p) >= 1.5) { self.say(format!("{w}.pitch"), "radians, inside 1.5 either way"); }
                    if c.sweep_period.is_some_and(|p| p <= 0.0) { self.say(format!("{w}.sweepPeriod"), "over 0"); }
                    if c.fov_deg.is_some_and(|p| p <= 0.0 || p > 170.0) { self.say(format!("{w}.fovDeg"), "degrees, over 0 and at most 170"); }
                    if c.range.is_some_and(|p| p <= 0.0) { self.say(format!("{w}.range"), "over 0"); }
                }
                DeviceDef::Keypad(k) => {
                    let w = named(i, "devices", &k.name);
                    if k.code.len() != 4 || !k.code.bytes().all(|b| b.is_ascii_digit()) { self.say(format!("{w}.code"), "four digits"); }
                    self.door(format!("{w}.door"), &k.door);
                    // (the lock IS the link: the door is locked, on this keypad's own lock, and nobody else's keypad is on it)
                    let Some(d) = f.doors.iter().find(|d| d.id == k.door) else { continue };
                    if d.locked != Some(true) { self.say(format!("{w}.door"), format!("'{}' is not locked: a keypad on an open door does nothing", d.id)); }
                    if d.lock.as_deref() != Some(&k.name) { self.say(format!("{w}.door"), format!("'{}' is not on this keypad's lock: the door's `lock` is the keypad's id", d.id)); }
                    if let Some(o) = pads.insert(&k.door, &k.name) { self.say(format!("{w}.door"), format!("'{}' has the keypad '{o}' already", d.id)); }
                }
                DeviceDef::Laser(l) => {
                    lasers += 1;
                    let w = named(i, "devices", &l.name);
                    self.word(&format!("{w}.group"), &l.group);
                    if hyp(&[l.to[0] - l.from[0], l.to[1] - l.from[1], l.to[2] - l.from[2]]) < 0.05 { self.say(&w, "from and to are the same point: a beam has a length"); }
                }
            }
        }
        let most = |list: &str, n: usize, cap: f64, of: &str| if n as f64 > cap { self.say(list, format!("at most {cap} {of} ({n})")); };
        most("devices", cameras, caps.cameras, "cameras");
        most("devices", lasers, caps.lasers, "tripwires");
        most("switchables", f.lights.len() + f.switchables.len() + cameras + lasers + 1, caps.fixtures, "light fixtures: lights, switchables, cameras, tripwires and the breaker together");
        for (i, v) in f.vents.iter().flatten().enumerate() {
            let w = named(i, "vents", &v.name);
            if !some(&v.dir) { self.say(format!("{w}.dir"), "points somewhere"); }
            let in_range = |k: &str, ok: bool, words: &str| if !ok { self.say(format!("{w}.{k}"), words); };
            in_range("speed", (0.0..=12.0).contains(&v.speed), "m/s, 0 to 12");
            in_range("spread", (0.0..89.0).contains(&v.spread), "degrees, 0 to under 89");
            in_range("reach", v.reach > 0.0 && v.reach <= 30.0, "m, over 0 and at most 30");
            in_range("size", v.size.map_or(true, |x| x > 0.0 && x <= 3.0), "m, over 0 and at most 3");
            in_range("period", v.period.map_or(true, |x| x > 0.0), "seconds, over 0");
            in_range("duty", v.duty.map_or(true, |x| (0.0..=1.0).contains(&x)), "from 0 to 1");
        }
        most("hums", f.hums.as_ref().map_or(0, |h| h.len()), 16.0, "hums: the mixer holds a voice for each");
        for (i, h) in f.hums.iter().flatten().enumerate() {
            let w = named(i, "hums", &h.name);
            if !(h.level > 0.0 && h.level <= 1.0) { self.say(format!("{w}.level"), "over 0 and at most 1"); }
            if !(h.radius > 0.0 && h.radius <= 40.0) { self.say(format!("{w}.radius"), "m, over 0 and at most 40"); }
        }
    }

    // ---- the mission's thread, the breaker, the sky, the day

    fn mission(&self) {
        let f = self.f;
        let (m, caps) = (&f.mission, game::levelfit::level_caps());
        let a = &m.reach.area;
        if !rect(a.x0, a.x1, a.z0, a.z1) { self.say("mission.objectives[reach].area", "x0 to x1, z0 to z1"); }
        self.door("mission.objectives[reach].door", &m.reach.door);
        if !whole(m.drive.rack_index) || m.drive.rack_index < 0.0 { self.say("mission.objectives[drive].rackIndex", "a whole number, 0 or more"); }
        if m.drive.half_w < 0.0 { self.say("mission.objectives[drive].halfW", "0 or more"); }
        for (w, name) in [("reach].name", &m.reach.name), ("drive].name", &m.drive.name), ("drive].holder", &m.drive.holder), ("exfil].name", &m.exfil.name)] { self.word(&format!("mission.objectives[{w}"), name); }
        self.door("mission.objectives[exfil].door", &m.exfil.door);
        if m.exfil.out.is_some_and(|o| !unit2(o)) { self.say("mission.objectives[exfil].out", "a unit vector, dx dz"); }
        self.a_box("breaker.panel", &f.breaker.panel);
        self.a_box("breaker.led", &f.breaker.led);
        if f.breaker.fumble.is_some_and(|s| s <= 0.0 || s > 7.0) { self.say("breaker.fumble", "seconds, over 0 and 7 at most (a room fight's lever party gives the panel up at 8)"); }
        if f.beacons.len() as f64 > caps.beacons { self.say("beacons", format!("at most {} ({})", caps.beacons, f.beacons.len())); }
        for (i, b) in f.beacons.iter().enumerate() {
            let w = format!("{}.box", named(i, "beacons", &b.id));
            self.a_box(&w, &b.box_);
            if self.boxes.contains(b.box_.as_str()) && !self.switched.contains_key(b.box_.as_str()) { self.say(w, format!("'{}' is no switchable's box (the beam goes with that fixture)", b.box_)); }
        }
    }

    // ---- the drills' plans

    fn plans(&self) {
        let p = &self.f.plans;
        for (i, Plan { id, plan }) in p.room_clear.iter().enumerate() { self.plan(&named(i, "plans.roomClear", id), &plan.door, &plan.bounds, plan.stack_side, &plan.entry); }
        if let Some(Plan { id, plan }) = &p.breach {
            let w = "plans.breach";
            self.text("plans.breach.id", id);
            self.plan(w, &plan.door, &plan.bounds, plan.stack_side, &plan.entry);
            if plan.hostiles.is_empty() { self.say("plans.breach.hostiles", "at least one"); }
        }
    }
    fn plan(&self, w: &str, door: &str, b: &Option<BreachPlanBounds>, side: f64, entry: &[Vec<V3>; 2]) {
        self.door(format!("{w}.door"), door);
        if b.is_some() && !bounds(b) { self.say(format!("{w}.bounds"), "x0 to x1, z0 to z1"); }
        if side != 1.0 && side != -1.0 { self.say(format!("{w}.stackSide"), "1 or -1"); }
        if entry.iter().any(|e| e.is_empty()) { self.say(format!("{w}.entry"), "a point at least for each of the two"); }
    }

    // ---- what the missions take from the level

    fn scenarios(&self, s: &LevelScenarios, data: &Json) {
        for (w, of) in [("scenarios.nightBreach", data.get("nightBreach")), ("scenarios.rescue.room", data.get("rescue").get("room"))] { self.lit(w, of.peek("lights").and_then(Json::str)); }
        if let Some(n) = &s.night_breach {
            self.word("scenarios.nightBreach.room", &n.room);
            if self.f.plans.breach.is_none() { self.say("scenarios.nightBreach", "wants plans.breach, the drill it stages"); }
        }
        if let Some(r) = &s.room_clear { self.room_clear(r, s); }
        if let Some(q) = &s.quiet { self.quiet(q); }
        if let Some(l) = &s.lesson { self.lesson(l, s); }
        if let Some(p) = &s.par {
            if !(p.gold > 0.0 && p.gold < p.silver && p.silver < p.bronze) { self.say("scenarios.par", "gold under silver under bronze, seconds over 0"); }
            if p.loss_secs < 0.0 { self.say("scenarios.par.lossSecs", "seconds, 0 or more"); }
            if s.room_clear.is_none() { self.say("scenarios.par", "a par stands over a room clear, and the level has no scenarios.roomClear"); }
        }
        if let Some(i) = &s.issue { self.issue(i); }
        let mut seen = BTreeSet::new();
        for (i, n) in s.door_names.iter().flatten().enumerate() {
            let w = format!("scenarios.doorNames[{i}]");
            self.door(format!("{w}.door"), &n.door);
            if !seen.insert(&n.door) { self.say(format!("{w}.door"), format!("'{}' is named twice", n.door)); }
            if n.name.trim().is_empty() || n.name.chars().count() > 40 { self.say(format!("{w}.name"), "1 to 40 letters"); }
        }
        for (i, h) in s.walked.iter().flatten().enumerate() {
            for (k, n) in [("from", &h.from), ("to", &h.to)] { if n.trim().is_empty() || n.chars().count() > 64 { self.say(format!("scenarios.walked[{i}].{k}"), "a name of 1 to 64 letters"); } }
            if h.weight.is_some_and(|x| x <= 0.0 || x > 8.0) { self.say(format!("scenarios.walked[{i}].weight"), "over 0 and at most 8"); }
        }
        if let Some(r) = &s.response {
            self.text("scenarios.response.why", data.get("response").get("why").str().unwrap_or(""));
            self.response(r, s);
        }
        for (i, a) in s.places.iter().flat_map(|p| &p.areas).enumerate() {
            self.text(format!("scenarios.places.areas[{i}].name"), &a.name);
            if !is_rect(&a.bounds) { self.say(format!("scenarios.places.areas[{i}].bounds"), "x0 to x1, z0 to z1"); }
        }
        if let Some(r) = &s.rescue { self.rescue(r); }
    }
    /// a group of lights that is named is one the level has
    fn lit(&self, w: &str, group: Option<&str>) {
        if let Some(g) = group { if !self.groups.contains(g) { self.say(format!("{w}.lights"), format!("no light and no switchable is in the group '{g}'")); } }
    }
    fn team(&self, w: &str, t: &TeamStart, ops: usize) {
        if ops > 0 && t.ops.as_ref().map_or(0, Vec::len) < ops { self.say(format!("{w}.team.ops"), format!("at least {ops} marks")); }
    }

    fn room_clear(&self, r: &StructureData, s: &LevelScenarios) {
        let (w, breach) = ("scenarios.roomClear", self.f.plans.breach.is_some());
        self.text(format!("{w}.name"), &r.name);
        match &r.team {
            Some(t) => self.team(w, t, if breach { 0 } else { 2 }),
            None if !breach => self.say(format!("{w}.team"), "a level without plans.breach says where the team starts (sam, samYaw, ops)"),
            None if s.night_breach.is_none() => self.say(format!("{w}.team"), "with no scenarios.nightBreach to take the start from, the room clear says where the team starts"),
            None => {}
        }
        if r.starts.is_some() && r.team.is_none() { self.say(format!("{w}.starts"), "other team starts stand beside one of its own"); }
        for (i, t) in r.starts.iter().flatten().enumerate() { self.team(&format!("{w}.starts[{i}]"), t, 2); }
        if r.rooms.is_empty() { self.say(format!("{w}.rooms"), "at least one"); }
        self.own(&format!("{w}.rooms"), r.rooms.iter().map(|x| Some(x.id.as_str())));
        for (i, x) in r.rooms.iter().enumerate() {
            let at = named(i, &format!("{w}.rooms"), &x.id);
            self.text(format!("{at}.name"), &x.name);
            if !is_rect(&x.bounds) { self.say(format!("{at}.bounds"), "x0 to x1, z0 to z1"); }
            if !whole(x.garrison) || x.garrison < 1.0 { self.say(format!("{at}.garrison"), "a whole number of men, one at least"); }
            for (j, d) in x.doors.iter().enumerate() { self.door(format!("{at}.doors[{j}]"), &d.id); self.text(format!("{at}.doors[{j}].from"), &d.from); }
            for (j, k) in x.marks.iter().enumerate() { if !x.doors.iter().any(|d| d.id == k.watch) { self.say(format!("{at}.marks[{j}].watch"), format!("'{}' is no door of this room", k.watch)); } }
        }
        if r.max_men.is_some_and(|n| !whole(n) || !(1.0..=SQUAD_MAX_MEN).contains(&n)) { self.say(format!("{w}.maxMen"), format!("a whole number of men, at most {SQUAD_MAX_MEN}")); }
        if r.wall_charges.is_some_and(|n| !whole(n) || !(1.0..=9.0).contains(&n)) { self.say(format!("{w}.wallCharges"), "a whole number of wall charges, 1 to 9"); }
        let Some(squads) = &r.squads else { return };
        self.own(&format!("{w}.squads"), squads.iter().map(|q| Some(q.id.as_str())));
        let room = |id: &str| r.rooms.iter().find(|x| x.id == id);
        let cap = r.max_men.unwrap_or(5.0);
        let mut owner: BTreeMap<&str, &str> = BTreeMap::new();
        for (i, q) in squads.iter().enumerate() {
            let at = named(i, &format!("{w}.squads"), &q.id);
            self.word(&format!("{at}.name"), &q.name);
            let mine = q.rooms.as_deref().unwrap_or(&[]);
            for (j, id) in mine.iter().enumerate() {
                if room(id).is_none() { self.say(format!("{at}.rooms[{j}]"), format!("the structure has no room '{id}'")); continue; }
                if let Some(o) = owner.get(id.as_str()) { self.say(format!("{at}.rooms[{j}]"), format!("the room '{id}' is the squad '{o}'s already")); } else { owner.insert(id, &q.id); }
            }
            match &q.arrive {
                None => if mine.is_empty() { self.say(format!("{at}.rooms"), "at least one (only a squad that arrives holds none)"); },
                Some(a) => {
                    if !mine.is_empty() { self.say(&at, "a squad that arrives comes in late: it holds no rooms"); }
                    self.door(format!("{at}.arrive.door"), &a.door);
                    if !whole(a.men) || a.men < 1.0 { self.say(format!("{at}.arrive.men"), "a whole number of men, one at least"); }
                    if (a.spawn.len() as f64) < a.men { self.say(format!("{at}.arrive.spawn"), format!("{} points for {} men", a.spawn.len(), a.men)); }
                    for (j, p) in a.spawn.iter().enumerate() { if let Some(x) = r.rooms.iter().find(|x| inside(p, &x.bounds)) { self.say(format!("{at}.arrive.spawn[{j}]"), format!("inside the room '{}': a late squad starts outside every room", x.id)); } }
                    if a.men > cap { self.say(format!("{at}.arrive.men"), format!("{} men never fit maxMen {cap}", a.men)); }
                }
            }
            if let Some(p) = &q.rally {
                if p[1] != 0.0 { self.say(format!("{at}.rally"), "a floor point, x 0 z"); }
                else if q.arrive.is_none() && !mine.iter().any(|id| room(id).is_some_and(|x| inside(p, &x.bounds))) { self.say(format!("{at}.rally"), "inside none of the squad's rooms"); }
            }
            for id in q.barricade.iter().flatten() { if !mine.iter().any(|x| room(x).is_some_and(|x| x.doors.iter().any(|d| d.id == *id))) { self.say(format!("{at}.barricade"), format!("'{id}' is no door of the squad's rooms")); } }
        }
        if squads.is_empty() { return; }
        for x in &r.rooms { if !owner.contains_key(x.id.as_str()) { self.say(format!("{w}.squads"), format!("the room '{}' is in no squad (with squads, every room is in exactly one)", x.id)); } }
        let men: f64 = r.rooms.iter().map(|x| x.garrison).sum();
        if men > cap { self.say(format!("{w}.squads"), format!("{men} men in the rooms' garrisons, and maxMen is {}", if r.max_men.is_some() { format!("{cap}") } else { String::from("not said (five)") })); }
    }

    fn quiet(&self, q: &QuietData) {
        let w = "scenarios.quiet";
        self.word(&format!("{w}.thing"), &q.thing);
        for (k, n) in [("starts", q.starts.len()), ("spots", q.spots.len()), ("exits", q.exits.len())] { if n == 0 { self.say(format!("{w}.{k}"), "at least one"); } }
        for (i, t) in q.starts.iter().enumerate() { self.word(&format!("{w}.starts[{i}].name"), &t.name); }
        for (i, p) in q.spots.iter().enumerate() {
            let at = format!("{w}.spots[{i}]");
            self.text(format!("{at}.place"), &p.place);
            self.text(format!("{at}.holder"), &p.holder);
            if !is_rect(&p.area) { self.say(format!("{at}.area"), "x0 to x1, z0 to z1"); }
            self.door(format!("{at}.door"), &p.door);
            if !whole(p.rack_index) || p.rack_index < 0.0 { self.say(format!("{at}.rackIndex"), "a whole number, 0 or more"); }
            if p.half_w < 0.0 { self.say(format!("{at}.halfW"), "0 or more"); }
        }
        for (i, x) in q.exits.iter().enumerate() {
            let at = format!("{w}.exits[{i}]");
            self.text(format!("{at}.name"), &x.name);
            self.door(format!("{at}.door"), &x.door);
            if !unit2(x.out) { self.say(format!("{at}.out"), "a unit vector, dx dz"); }
            if x.half_w.is_some_and(|h| h <= 0.0) { self.say(format!("{at}.halfW"), "over 0"); }
        }
        // (a route is known by its name, else its id)
        let routes: BTreeSet<&str> = self.f.routes.iter().map(|r| r.name.as_deref().unwrap_or(&r.id)).collect();
        match &q.patrols {
            None => if self.f.routes.len() > QUIET_ROUTES { self.say(format!("{w}.patrols"), format!("the level has {} routes and at most {QUIET_ROUTES} are manned at once: say which, in patrol sets", self.f.routes.len())); },
            Some(sets) => {
                if sets.is_empty() { self.say(format!("{w}.patrols"), "at least one set"); }
                for (i, set) in sets.iter().enumerate() {
                    let at = format!("{w}.patrols[{i}]");
                    if set.is_empty() { self.say(&at, "at least one route"); }
                    for n in set { if !routes.contains(n.as_str()) { self.say(&at, format!("no route of the level is called '{n}'")); } }
                    if set.iter().collect::<BTreeSet<_>>().len() != set.len() { self.say(&at, "a route is named twice"); }
                    if set.len() > QUIET_ROUTES { self.say(&at, format!("{} routes: at most {QUIET_ROUTES} are manned at once", set.len())); }
                }
            }
        }
        let stand = |at: String, s: &QuietStand, walks: bool| {
            self.text(format!("{at}.name"), &s.name);
            if s.loop_.as_ref().is_some_and(|l| l.is_empty()) { self.say(format!("{at}.loop"), "at least one point"); }
            if s.loop_.is_some() && !walks { self.say(format!("{at}.loop"), "a place stood at has no lap"); }
        };
        if let Some(n) = &q.needs {
            for (k, s) in [("coffee", &n.coffee), ("smoke", &n.smoke), ("water", &n.water), ("stretch", &n.stretch)] { if let Some(s) = s { stand(format!("{w}.needs.{k}"), s, true); } }
            if n.stretch.as_ref().is_some_and(|s| s.loop_.is_none()) { self.say(format!("{w}.needs.stretch.loop"), "the lap he walks"); }
        }
        for (k, l) in [("phones", &q.phones), ("wayouts", &q.wayouts)] { for (i, s) in l.iter().flatten().enumerate() { stand(format!("{w}.{k}[{i}]"), s, false); } }
    }

    /// a step's test: what no deed could ever answer, and a door, a room or a route this file has not. (A light and a man the mission stands: `tests/lessons.rs` asks of those)
    fn lesson_test(&self, at: &str, t: &game::lesson::LessonTest, s: &LevelScenarios) {
        use game::lesson::LessonTest as T;
        let door = |d: &str| if !self.doors.contains(d) { self.say(at, format!("no door of the level is called '{d}'")); };
        match t {
            T::In { area: r } | T::Body { area: r } | T::Squad { area: r, .. } => if !(r.x0 < r.x1 && r.z0 < r.z1) { self.say(at, format!("an area that holds nothing: {r:?}")); },
            T::Near { r: Some(r), .. } if *r <= 0.0 => self.say(at, format!("no man can be nearer than {r}")),
            T::Men { down: Some(n) } if *n < 1.0 => self.say(at, "down: how many, at least one"),
            T::Door { door: d, .. } | T::StackOn { door: d, .. } => door(d),
            T::Stacks { doors, .. } => doors.iter().for_each(|d| door(d)),
            T::Did { what, on: Some(d), .. } => { if !what.starts_with("order:") { self.say(at, format!("on: an order is given on a door, and '{what}' is none")); } door(d); }
            T::Taken { room } => if !s.room_clear.as_ref().is_some_and(|c| c.rooms.iter().any(|r| r.id == *room)) { self.say(at, format!("no room of the level is called '{room}'")); },
            T::Calm { route: Some(n) } => if !self.f.routes.iter().any(|r| r.name.as_deref().unwrap_or(&r.id) == n) { self.say(at, format!("no route of the level is called '{n}'")); },
            T::All { of } | T::Any { of } => { if of.is_empty() { self.say(at, "a list of no tests"); } for x in of { self.lesson_test(at, x, s); } }
            _ => {}
        }
    }

    fn lesson(&self, l: &LessonData, s: &LevelScenarios) {
        let w = "scenarios.lesson";
        self.text(format!("{w}.title"), &l.title);
        if let Some(r) = &l.rails {
            if r.respawn.is_some_and(|s| s < 0.0) { self.say(format!("{w}.rails.respawn"), "seconds, 0 or more"); }
            for v in r.rows.iter().flatten() { self.text(format!("{w}.rails.rows"), v); }
        }
        if l.steps.is_empty() { self.say(format!("{w}.steps"), "at least one"); }
        let mut ids = BTreeSet::new();
        for (i, t) in l.steps.iter().enumerate() {
            let at = named(i, &format!("{w}.steps"), &t.id);
            self.text(format!("{at}.id"), &t.id);
            self.text(format!("{at}.line"), &t.line);
            self.lesson_test(&format!("{at}.done"), &t.done, s);
            if !ids.insert(&t.id) { self.say(&at, "the id is another step's already"); }
            for (k, v) in [("cue", &t.cue), ("checkpoint", &t.checkpoint), ("setupLine", &t.setup_line), ("ringLine", &t.ring_line), ("blowLine", &t.blow_line), ("menuLine", &t.menu_line),
                ("noTap", &t.no_tap), ("atMan", &t.at_man)] { self.word(&format!("{at}.{k}"), v); }
            if t.settle.is_some_and(|s| s <= 0.0 || s > 10.0) { self.say(format!("{at}.settle"), "seconds, over 0 and at most 10"); }
            if t.hold.is_some_and(|s| s < 0.0) { self.say(format!("{at}.hold"), "seconds, 0 or more"); }
            if t.eye.as_ref().and_then(|e| e.fov).is_some_and(|x| x <= 0.0 || x > 360.0) { self.say(format!("{at}.eye.fov"), "degrees, over 0 and up to 360"); }
            if let Some(x) = &t.explain {
                self.text(format!("{at}.explain.text"), &x.text);
                self.word(&format!("{at}.explain.title"), &x.title);
                if x.quiet == Some(true) && x.now == Some(true) { self.say(format!("{at}.explain"), "says one of quiet and now"); }
            }
            if t.toward.is_some() && t.at.is_none() { self.say(format!("{at}.toward"), "wants the step's `at`, where the arrow starts"); }
            if t.on_way.is_some() && t.at.is_none() { self.say(format!("{at}.onWay"), "wants the step's `at`, the mark the order is to"); }
            if let Some(a) = &t.after {
                if a.line.is_none() && a.word.is_none() { self.say(format!("{at}.after"), "says a line or a word"); }
                self.word(&format!("{at}.after.line"), &a.line);
                self.word(&format!("{at}.after.word"), &a.word);
                if a.on.as_ref().is_some_and(|o| o.is_empty()) { self.say(format!("{at}.after.on"), "at least one order"); }
            }
        }
    }

    fn issue(&self, i: &IssueData) {
        use game::kit::KitWeapon;
        for (place, c) in i.members.iter() {
            let w = format!("scenarios.issue.members.{place}");
            if !matches!(place.as_str(), "0" | "1" | "2" | "3" | "4") { self.say(&w, "a place on the roster, '0' (you) to '4'"); }
            for (k, n) in [("bangs", c.bangs), ("charges", c.charges), ("smoke", c.smoke), ("frags", c.frags), ("walls", c.walls), ("shells", c.shells)] { if n.is_some_and(|n| !whole(n) || !(0.0..=9.0).contains(&n)) { self.say(format!("{w}.{k}"), "a whole number, 0 to 9"); } }
            if c.rings.is_some_and(|n| !whole(n) || !(0.0..=5.0).contains(&n)) { self.say(format!("{w}.rings"), "a whole number, 0 to 5"); }
            if place != "0" && c.shells.is_some() { self.say(format!("{w}.shells"), "yours alone: a teammate carries no 40 mm"); }
            for (k, has) in [("rings", c.rings.is_some()), ("loaded", c.loaded.is_some())] {
                if !has { continue; }
                if place != "0" { self.say(format!("{w}.{k}"), "yours alone: the launcher is on your rifle"); }
                else if matches!(c.weapon, Some(KitWeapon::Pistol | KitWeapon::Shotgun)) { self.say(format!("{w}.{k}"), "the launcher is the rifle's, and this card carries none"); }
            }
            let empty = match c.loaded { Some(IssueCardLoaded::He40) => c.shells == Some(0.0), Some(IssueCardLoaded::Ring) => !c.rings.is_some_and(|n| n > 0.0), None => false };
            if empty { self.say(format!("{w}.loaded"), "this card carries none of that load: the launcher would come up on the other one"); }
        }
    }

    /// the Team & kit picture. Whether a place is free floor is the planner's to say (kit_plan.rs, tools/qa/kitview-plan.ts)
    fn kit_view(&self, k: &KitView, s: &LevelScenarios, data: &Json) {
        let w = "scenarios.kitView";
        let starts = s.room_clear.as_ref().filter(|r| r.team.is_some()).map_or(1, |r| 1 + r.starts.as_ref().map_or(0, Vec::len));
        if let Some(sets) = &k.sets {
            for (name, t) in sets.iter() {
                let at = format!("{w}.sets.{name}");
                if t.boxes.is_empty() { self.say(format!("{at}.boxes"), "at least one"); }
                self.own(&format!("{at}.boxes"), t.boxes.iter().map(|b| Some(b.id.as_str())));
                self.own(&format!("{at}.doors"), t.doors.iter().flatten().map(|d| Some(d.id.as_str())));
                self.own(&format!("{at}.lights"), t.lights.iter().flatten().map(|l| Some(l.id.as_str())));
                for (i, b) in t.boxes.iter().enumerate() {
                    let at = named(i, &format!("{at}.boxes"), &b.id);
                    let said = Said::from(b);
                    self.paint(&at, &said, true);
                    self.words(&at, said.stuff, said.does);
                    self.round(&at, b);
                    if b.shape.is_some() { self.say(format!("{at}.shape"), "a picture's box says no shape"); }
                }
                if let Some(p) = &t.door_paint { self.door_paint(&format!("{at}.doorPaint"), p); }
                for (i, d) in t.doors.iter().flatten().enumerate() { if d.width <= 0.0 { self.say(format!("{}.width", named(i, &format!("{at}.doors"), &d.id)), "over 0"); } }
            }
        }
        if k.ways.is_empty() { self.say(format!("{w}.ways"), "at least one"); }
        if k.ways.len() > starts { self.say(format!("{w}.ways"), format!("{} ways for {starts} team starts (one a way in: the level's own, then each of scenarios.roomClear.starts)", k.ways.len())); }
        for (i, y) in k.ways.iter().enumerate() {
            let at = format!("{w}.ways[{i}]");
            let set = y.set.as_ref().map(|n| k.sets.as_ref().and_then(|s| s.get(n.clone())));
            if let Some(a) = &y.add { self.added(&format!("{at}.add"), a, y.set.is_some(), data.get("ways").at(i).get("add")); }
            if let (Some(n), Some(None)) = (&y.set, &set) { self.say(format!("{at}.set"), format!("the block has no set '{n}'")); }
            // (the anchor: a door of the level, or of the way's set where it names one; or a span; never both)
            match (&y.span, &y.door) {
                (Some(_), Some(_)) => self.say(format!("{at}.door"), "a way is anchored on a door or on a span, not both"),
                (Some(_), None) => {}
                (None, d) => {
                    let d = d.as_deref().unwrap_or("");
                    let has = match &set { None => self.doors.contains(d), Some(s) => s.is_some_and(|s| s.doors.iter().flatten().any(|o| o.id == d)) };
                    if !has { self.say(format!("{at}.door"), format!("the {} has no door '{d}'", if set.is_some() { "set" } else { "level" })); }
                }
            }
            if y.kind.is_some_and(|k| (k == KitViewWayKind::Door) == y.span.is_some()) { self.say(format!("{at}.kind"), if y.span.is_some() { "wall or gap" } else { "door" }); }
            if y.side.is_some_and(|s| s != 1.0 && s != -1.0) { self.say(format!("{at}.side"), "1 or -1"); }
            for (k, v) in [("slotGap", y.slot_gap), ("lane", y.lane), ("out", y.out), ("inside", y.inside), ("roomLights", y.room_lights)] { self.not_under(&format!("{at}.{k}"), v); }
            if y.pin.as_ref().and_then(|p| p.dist).is_some_and(|d| d <= 1.0) { self.say(format!("{at}.pin.dist"), "metres, over 1"); }
            if y.slots.as_ref().is_some_and(|s| s.len() > 5) { self.say(format!("{at}.slots"), "up to five: the player, then the teammates from the door outward"); }
            if let Some(KitViewWayPane::Bool(true)) = y.pane { self.say(format!("{at}.pane"), "a pane's own record, or false"); }
            if let Some(KitViewWayPane::Of(p)) = &y.pane { self.text(format!("{at}.pane.box"), &p.box_); }
            for (k, l) in [("show", &y.show), ("keepLights", &y.keep_lights), ("dayLights", &y.day_lights), ("hide", &y.hide), ("hideMore", &y.hide_more), ("hidePaint", &y.hide_paint)] { for n in l.iter().flatten() { self.text(format!("{at}.{k}"), n); } }
            let Some(l) = &y.light else { continue };
            let at = format!("{at}.light");
            for (k, v) in [("roomLights", l.room_lights), ("moonScale", l.moon_scale), ("skyScale", l.sky_scale)] { self.not_under(&format!("{at}.{k}"), v); }
            let hung = |at: String, kind: Option<f64>, n: [Option<f64>; 6]| {
                if kind.is_some_and(|k| k != 0.0 && k != 1.0 && k != 3.0) { self.say(format!("{at}.kind"), "0 (point), 1 (spot) or 3 (a disk)"); }
                for (k, v) in ["intensity", "range", "innerDeg", "outerDeg", "radius", "volumetric"].into_iter().zip(n) { self.not_under(&format!("{at}.{k}"), v); }
            };
            if let Some(p) = &l.porch { hung(format!("{at}.porch"), p.kind, [p.intensity, p.range, p.inner_deg, p.outer_deg, p.radius, p.volumetric]); self.not_under(&format!("{at}.porch.day"), p.day); }
            if let Some(p) = &l.fill { hung(format!("{at}.fill"), p.kind, [p.intensity, p.range, p.inner_deg, p.outer_deg, p.radius, p.volumetric]); }
            if let Some(p) = &l.sam { for (k, v) in [("intensity", p.intensity), ("innerDeg", p.inner_deg), ("outerDeg", p.outer_deg), ("volumetric", p.volumetric)] { self.not_under(&format!("{at}.sam.{k}"), v); } }
        }
    }
    fn not_under(&self, place: &str, v: Option<f64>) { if v.is_some_and(|x| x < 0.0) { self.say(place, "0 or more"); } }
    /// a piece or a lamp more on a level's own way: the level format's own records, with ids new to the level (two ways may say the same id: one way alone is ever laid)
    fn added(&self, w: &str, a: &KitViewWayAdd, on_a_set: bool, data: &Json) {
        if a.laid.is_some() { self.say(format!("{w}.laid"), "a file never says it: it is what the loader has done"); }
        if on_a_set { self.say(w, "a way on a set adds nothing: the set is the place for its boxes and lamps"); }
        self.own(&format!("{w}.boxes"), a.boxes.iter().flatten().map(|b| Some(b.id.as_str())));
        self.own(&format!("{w}.lamps"), a.lamps.iter().flatten().map(|l| Some(l.id.as_str())));
        self.floors(&format!("{w}.boxes"), data.get("boxes"));
        for (i, b) in a.boxes.iter().flatten().enumerate() {
            let at = named(i, &format!("{w}.boxes"), &b.id);
            if self.boxes.contains(b.id.as_str()) { self.say(format!("{at}.id"), "a box of the level is called that already"); }
            self.one_box(&at, b, true);
        }
        let mine: BTreeSet<&str> = a.boxes.iter().flatten().map(|b| b.id.as_str()).collect();
        for (i, l) in a.lamps.iter().flatten().enumerate() {
            let at = named(i, &format!("{w}.lamps"), &l.id);
            if self.lights.contains(l.id.as_str()) { self.say(format!("{at}.id"), "a light of the level is called that already"); }
            self.lamp(&at, l, &mine);
        }
    }

    fn response(&self, r: &ResponseData, s: &LevelScenarios) {
        let w = "scenarios.response";
        for (k, v) in [("site", &r.site), ("at", &r.at), ("team", &r.team)] { self.text(format!("{w}.{k}"), v); }
        if r.tag.as_ref().is_some_and(|t| !(2..=8).contains(&t.len()) || !t.bytes().all(|b| b.is_ascii_uppercase())) { self.say(format!("{w}.tag"), "two to eight capital letters"); }
        for (k, v, lo, hi) in [("men", r.men, 2.0, 4.0), ("eta", r.eta, 15.0, 240.0)] {
            if v.is_some_and(|[a, b]| a < lo || b > hi || a > b || !whole(a) || !whole(b)) { self.say(format!("{w}.{k}"), format!("fewest and most, whole numbers from {lo} to {hi}")); }
        }
        if r.ways.is_empty() { self.say(format!("{w}.ways"), "at least one"); }
        for (i, y) in r.ways.iter().enumerate() {
            let at = format!("{w}.ways[{i}]");
            self.text(format!("{at}.name"), &y.name);
            if let Some(d) = &y.door { self.door(format!("{at}.door"), d); }
            self.word(&format!("{at}.coming"), &y.coming);
            self.word(&format!("{at}.clear"), &y.clear);
            for (k, n) in [("muster", y.muster.len()), ("entry", y.entry.len()), ("entryFace", y.entry_face.len())] { if n != 4 { self.say(format!("{at}.{k}"), "four, in element order"); } }
        }
        let posts = r.posts.as_deref().unwrap_or(&[]);
        for (i, p) in posts.iter().enumerate() { self.text(format!("{w}.posts[{i}].name"), &p.name); }
        for (i, p) in r.sweep.iter().flatten().enumerate() { self.text(format!("{w}.sweep[{i}].name"), &p.name); }
        if !posts.is_empty() && r.outside.as_ref().map_or(0, Vec::len) < 2 { self.say(format!("{w}.outside"), "posts want at least two marks to stand their own men up on"); }
        if let Some(raid) = &r.raid {
            match (&s.room_clear, &raid.room) {
                (None, _) => self.say(format!("{w}.raid"), "a raid stands over a room clear, and the level has no scenarios.roomClear"),
                (Some(c), Some(id)) => if !c.rooms.iter().any(|x| x.id == *id) { self.say(format!("{w}.raid.room"), format!("the room clear has no room '{id}'")); },
                _ => {}
            }
            if !posts.is_empty() { self.say(format!("{w}.posts"), "a raid's team has no second lift: no posts"); }
        }
    }

    fn rescue(&self, r: &RescueData) {
        let w = "scenarios.rescue";
        // (a room's floor is its own words', or the drill's that goes through its door)
        let drilled = |door: &str| self.f.plans.room_clear.iter().any(|Plan { plan: RoomClearPlan { door: d, bounds: b, .. }, .. }| d == door && bounds(b));
        let floor = |at: &str, own: &Option<Rect>, door: &str| if !own.as_ref().map_or_else(|| drilled(door), is_rect) { self.say(at, "bounds, or a plans.roomClear entry with bounds for its door"); };
        self.team(w, &r.team, 2);
        self.text(format!("{w}.room.name"), &r.room.name);
        self.door(format!("{w}.room.door"), &r.room.door);
        self.word(&format!("{w}.room.where"), &r.room.where_);
        floor(&format!("{w}.room"), &r.room.bounds, &r.room.door);
        if let Some(s) = &r.side {
            self.door(format!("{w}.side.door"), &s.door);
            if s.door == r.room.door { self.say(format!("{w}.side.door"), "the room's main door: the side door is another one"); }
            if let Some(from) = &s.from { self.door(format!("{w}.side.from.door"), &from.door); floor(&format!("{w}.side.from"), &from.bounds, &from.door); }
        }
        if r.hostages.is_empty() { self.say(format!("{w}.hostages"), "at least one"); }
        let roamers = r.roamers.len() as f64;
        if r.cast.as_ref().is_some_and(|c| !whole(c.roamers) || c.roamers < 0.0 || c.roamers > roamers) { self.say(format!("{w}.cast.roamers"), "a whole number, at most the roamers' marks"); }
        // (the level's own cast fits: eleven defenders at most, and the player, the four, the hostages and they inside the sixteen the skinning holds)
        let (own, held) = (2.0 + r.guards.as_ref().map_or(0, Vec::len) as f64 + r.cast.as_ref().map_or(0.0, |c| c.roamers), r.hostages.len() as f64);
        if (r.guards.is_some() || r.cast.is_some()) && (own > 11.0 || 5.0 + held + own > 16.0) { self.say(format!("{w}.cast"), format!("{own} defenders and {held} hostages: at most 11 defenders, and 16 characters with the player and four teammates")); }
        let Some(f) = &r.flow else { return };
        if !is_rect(&f.corridor) { self.say(format!("{w}.flow.corridor"), "x0 to x1, z0 to z1"); }
        self.own(&format!("{w}.flow.regions"), f.regions.iter().map(|x| Some(x.id.as_str())));
        self.own(&format!("{w}.flow.route"), f.route.iter().map(|x| Some(x.id.as_str())));
        for (i, x) in f.regions.iter().enumerate() {
            let at = named(i, &format!("{w}.flow.regions"), &x.id);
            if !is_rect(&x.rect) { self.say(format!("{at}.rect"), "x0 to x1, z0 to z1"); }
            if let Some(d) = &x.door { self.door(format!("{at}.door"), d); }
        }
        for (i, o) in f.route.iter().enumerate() {
            let at = named(i, &format!("{w}.flow.route"), &o.id);
            if !f.regions.iter().any(|x| x.id == o.region) { self.say(format!("{at}.region"), format!("the flow has no region '{}'", o.region)); }
            if let Some(d) = &o.door { self.door(format!("{at}.door"), d); }
            for (k, v) in [("name", &o.name), ("short", &o.short), ("call", &o.call)] { self.text(format!("{at}.{k}"), v); }
        }
    }
}
