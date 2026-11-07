//! A LEVEL FILE INTO THE GAME: `src/scene/levelLoad.ts` `loadLevel`, record by record, reading the file as the TypeScript reads its parsed object.
//! The level's boxes become rows of `g.tab.boxes` (listed in the answer, in the file's order: index i of the file IS static box i), its routes rows
//! of `g.tab.route`, and `g.level` is the `Level` the game's constructor reads.
//!
//! WHAT IS NOT CARRIED IS SAID, NEVER PASSED OVER. The file's tree remembers every member a reader asked for (`json.rs`), so when the file has
//! been read, whatever nobody asked for is named by its place: a record this reader has no line for, a member a record gained since. A start
//! refuses a file with any (`load`), because a level stood without part of itself is another level; `lacks_of` is the same list for a tool.
//! What the loader leaves behind BY RULE it passes by in so many words (`skip`: a floor, an author's notes, the editor's own block).
//! The file's own checks (`levelData.ts validateLevelData`: unknown keys, ids that point nowhere) are not here either: a file is taken as one the
//! TypeScript's validator has passed.
use crate::json::Json;
use alloc::{format, string::String, vec, vec::Vec};
use game::game::Game;
use game::ids::*;
use game::scene_boxes::MakeBoxP;
use game::scene_level::*;
use game::scene_markings::{MarkFace, MarkPal, MarkRes, MarkingDef};
use game::scene_materials::DoesWord;
use crate::read::FromJson;
use game::scene_stuff::BoxStuff;
use game::V3;

/// a word of the file as one of a type's words (the types say their words by `as_str`; this is the other way, over a list written once here)
macro_rules! words {
    ($f:ident, $t:ty, [$($v:ident),* $(,)?]) => {
        pub fn $f(w: &str) -> Option<$t> { $(if <$t>::$v.as_str() == w { return Some(<$t>::$v); })* None }
    };
}
words!(stuff_word, BoxStuff, [Carpet, Grass, Rubber, Metal, Glass, Wood, Plastic, Fabric, Steel, Enamel, Tarmac, MudWall, Plaster, Block, Earth, Gravel, Slab, Sheet, Cloth, Paint, Veneer, Carpaint, Stucco, Skim, GlossBlock, BoardConcrete, CastPanel, Paver, Brushed, Louvre, Plain, Waits]);
words!(does_word, DoesWord, [Snow, Carpet, Lino, Stone, Grass, Wood, Plastic, Concrete, Fabric, Steel, Enamel, Tarmac, Casing, Screen, UnnamedDark, Plasterboard, Partition, Ceiling, Laminate, Pane, DarkCarpet, Card, Frame, Plant, PackedSnow, OldSnow, Unnamed, UnnamedGrey, UnnamedLight, UnnamedLightGrey, UnnamedPale, UnnamedPaleGrey, DarkPlastic, GreyPlastic, DarkGlass]);
words!(prop_kind, game::scene_level_data::PropKindName, [Cardboard, Plant, Chair, Bin]);
words!(weapon_word, game::weapons::WeaponId, [Fiveseven, Guard9mm, Aks74u, Ak74m, Akms, Fn2000, M870, Mk12, Mp7, Mk46, M1014, Hk45, G18, Pkm, Mk18bare, G19bare]);
words!(mark_face, MarkFace, [X, X2, Y, Y2, Z, Z2]);
words!(mark_res, MarkRes, [Base, Fine, Coarse]);
words!(light_kind, LightDefKind, [Spot, Dir, Point]);
words!(volume_kind, VolumeDefKind, [Trigger, Hide, Utility]);


/// the box flags a file may say, as the bits of `Box.flags` (levelData.ts FLAG_BITS)
fn flags_to_bits(names: &Json) -> f64 {
    let f = game::scene_boxes::box_flag();
    let mut bits = 0u32;
    for n in names.items() {
        bits |= match n.str() { Some("NoPrimary") => f.no_primary, Some("NoShadow") => f.no_shadow, Some("Cutaway") => f.cutaway, Some("Gloss") => f.gloss, Some("Glass") => f.glass, w => panic!("unknown box flag '{}'", w.unwrap_or("?")) } as u32;
    }
    bits as f64
}

fn v3(j: &Json, what: &str) -> V3 { j.v3().unwrap_or_else(|| panic!("level: {} is not three numbers", what)) }
fn num(j: &Json, what: &str) -> f64 { j.num().unwrap_or_else(|| panic!("level: {} is not a number", what)) }
fn texts(j: &Json, what: &str) -> Vec<String> { j.items().iter().map(|x| text(x, what)).collect() }
fn text(j: &Json, what: &str) -> String { String::from(j.str().unwrap_or_else(|| panic!("level: {} is not text", what))) }
fn rect(j: &Json, what: &str) -> [f64; 4] { [num(j.get("x0"), what), num(j.get("x1"), what), num(j.get("z0"), what), num(j.get("z1"), what)] }

/// (markings.ts marking / palEntry: the same structure the generator builds)
fn copy_markings(ms: &Json) -> Option<Vec<MarkingDef>> {
    if ms.items().is_empty() { return None; }
    Some(ms.items().iter().map(|m| MarkingDef {
        face: mark_face(m.get("face").str().unwrap_or("")).unwrap_or_else(|| panic!("level: a marking's face '{:?}'", m.get("face"))),
        res: match m.get("res").str() { None | Some("base") => None, Some(w) => Some(mark_res(w).unwrap_or_else(|| panic!("level: a marking's res '{}'", w))) },
        at: { let a = m.get("at").nums(); [a[0], a[1]] },
        size: { let a = m.get("size").nums(); [a[0], a[1]] },
        pal: m.get("pal").items().iter().map(|p| {
            let mut o = MarkPal { albedo: None, mul: None, rough: None, emit: None };
            if let Some(a) = p.get("albedo").v3() { o.albedo = Some(a); } else if let Some(x) = p.get("mul").num() { if x != 1.0 { o.mul = Some(x); } }
            if let Some(x) = p.get("rough").num() { o.rough = Some(x); }
            if let Some(e) = p.get("emit").v3() { if e[0] > 0.0 || e[1] > 0.0 || e[2] > 0.0 { o.emit = Some(e); } }
            o
        }).collect(),
        cells: m.get("cells").items().iter().map(|c| text(c, "a marking's row")).collect(),
    }).collect())
}

/// the file's paint palette, and the readers of a record's paint, finish and words (levelLoad.ts albedoOf, finishOf, stuffOf, doesOf: ONE reader for a
/// box, a prop's part and a shape's piece)
struct Paint<'a> { name: &'a str, mats: &'a Json }
impl<'a> Paint<'a> {
    fn row(&self, o: &Json) -> &'a Json { match o.get("material").str() { Some(m) => self.mats.get(m), None => &Json::Null } }
    fn albedo(&self, o: &Json) -> V3 {
        match o.get("material").str() {
            None => v3(o.get("albedo"), "a paint"),
            Some(m) => { let r = self.mats.get(m); if r.is_null() { panic!("level '{}': unknown material '{}'", self.name, m); } v3(r.get("albedo"), "a material's albedo") }
        }
    }
    fn rough(&self, o: &Json) -> Option<f64> { o.get("rough").num().or_else(|| self.row(o).get("rough").num()) }
    /// (`metal` is only ever carried as true: false is the default and stays unwritten)
    fn metal(&self, o: &Json) -> Option<bool> { match o.get("metal").bool().or_else(|| self.row(o).get("metal").bool()) { Some(true) => Some(true), _ => None } }
    /// what it is made of: its own word, else its material row's; nothing for plain
    fn stuff(&self, o: &Json) -> Option<BoxStuff> {
        match o.get("stuff").str().or_else(|| self.row(o).get("stuff").str()) { None | Some("plain") => None, Some(w) => Some(stuff_word(w).unwrap_or_else(|| panic!("level '{}': '{}' is nothing a box is made of", self.name, w))) }
    }
    /// what it does: its own word, else its material row's; nothing for the default row or a word that is none (materials.ts doesSaid)
    fn does(&self, o: &Json) -> Option<DoesWord> {
        match o.get("does").str().or_else(|| self.row(o).get("does").str()) { None | Some("unnamed") => None, Some(w) => does_word(w) }
    }
}

/// a shape's form as the file says it: its planes copied (shapes.ts formOf)
fn form_of(f: &Json) -> (Option<f64>, Option<game::scene_shapes::ShapeFormArc>, Option<Vec<[f64; 4]>>, Option<bool>) {
    let round = f.get("round").num().filter(|&r| r != 0.0);
    let arc = if f.has("arc") { Some(game::scene_shapes::ShapeFormArc { axis: num(f.get("arc").get("axis"), "an arc's axis"), rise: num(f.get("arc").get("rise"), "an arc's rise") }) } else { None };
    let cut = if f.has("cut") { Some(f.get("cut").items().iter().map(|q| { let q = q.nums(); [q[0], q[1], q[2], q[3]] }).collect()) } else { None };
    (round, arc, cut, if f.get("old").bool() == Some(true) { Some(true) } else { None })
}
/// A BOX'S SHAPE, carried on the box as authored: a piece's paint, finish and word read as a box's are, and a piece that says no paint left
/// with none (it wears its box's: the shape pass) (levelLoad.ts)
fn shape_of(s: &Json, paint: &Paint) -> game::scene_shapes::BoxShape {
    let (round, arc, cut, old) = form_of(s);
    let with = if s.has("with") { Some(s.get("with").items().iter().map(|w| {
        let (round, arc, cut, old) = form_of(w);
        let painted = w.has("material") || w.has("albedo");
        game::scene_shapes::ShapePiece {
            off: v3(w.get("off"), "a piece's off"), h: v3(w.get("h"), "a piece's h"), yaw: w.get("yaw").num().filter(|&y| y != 0.0),
            albedo: if painted { Some(paint.albedo(w)) } else { None }, rough: if painted { paint.rough(w) } else { None }, metal: if painted { paint.metal(w) } else { None },
            stuff: if painted { paint.stuff(w) } else { None }, does: if painted { paint.does(w) } else { None },
            round, arc, cut, old,
        }
    }).collect()) } else { None };
    game::scene_shapes::BoxShape { with, round, arc, cut, old }
}

/// (levelLoad.ts fitPropFootprint: a loose thing's half size in plan and its height, fitted from its parts)
fn fit_prop_footprint(parts: &[PropPart]) -> ([f64; 2], f64) {
    use game::m;
    let (mut x0, mut x1, mut z0, mut z1, mut height) = (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY, 0.1);
    for q in parts {
        let (cs, sn) = (m::abs(m::cos(q.yaw)), m::abs(m::sin(q.yaw)));
        let (ex, ez) = (cs * q.h[0] + sn * q.h[2], sn * q.h[0] + cs * q.h[2]);
        x0 = m::min(x0, q.off[0] - ex); x1 = m::max(x1, q.off[0] + ex); z0 = m::min(z0, q.off[2] - ez); z1 = m::max(z1, q.off[2] + ez); height = m::max(height, q.off[1] + q.h[1]);
    }
    ([m::max(0.05, (x1 - x0) / 2.0), m::max(0.05, (z1 - z0) / 2.0)], height)
}

/// What a file's reading made, beside `g.level` itself.
pub struct Loaded {
    /// the file's name (`LevelData.name`)
    pub name: String,
    /// the level's boxes as rows, in the file's order: what `level.boxes` is to the TypeScript
    pub boxes: Vec<BoxId>,
}

/// THE RECORDS OF THE FILE THAT THE GAME KEEPS AS ITS OWN TYPES (`records.rs`), read member for member.
struct Typed {
    scenarios: Option<game::scene_scenarios::LevelScenarios>, surfaces: Option<game::scene_level_data::SurfacesData>,
    devices: Vec<DeviceDef>, vents: Option<Vec<VentDef>>, civilians: Option<Vec<CivilianDef>>, clear_plans: Vec<RoomClearPlan>, breach: Option<BreachPlan>,
}
fn typed(data: &Json) -> Typed {
    fn opt<T: FromJson>(data: &Json, key: &str) -> Option<T> { if data.has(key) { Some(T::from_json(data.get(key), key)) } else { None } }
    let plans = data.get("plans");
    Typed {
        scenarios: opt(data, "scenarios"), surfaces: opt(data, "surfaces"),
        devices: FromJson::from_json(data.get("devices"), "devices"), vents: opt(data, "vents"), civilians: opt(data, "civilians"),
        clear_plans: FromJson::from_json(plans.get("roomClear"), "plans.roomClear"),
        breach: if plans.has("breach") { Some(FromJson::from_json(plans.get("breach"), "plans.breach")) } else { None },
    }
}

/// EVERY PART OF THE FILE THIS READER DOES NOT CARRY, by its place in the file; empty = the whole file is carried. The file is read as `load`
/// reads it, into a game nobody keeps, and the file's own tree says which members were never asked for (`json.rs`).
pub fn lacks_of(data: &Json) -> Vec<String> {
    let mut g = alloc::boxed::Box::new(Game::blank());
    read(&mut g, data, true, true).1
}

/// THE LEVEL FILE `data` INTO `g`: its boxes and routes as rows, `g.level`, the level's name and its script. Refuses a file it does not carry whole.
/// `dressing` / `lived`: the two switches a file's optional furniture stands under (levelLoad.ts: DESTRUCTION.on, LIVED.on).
pub fn load(g: &mut Game, data: &Json, dressing: bool, lived: bool) -> Loaded {
    let (loaded, lacks) = read(g, data, dressing, lived);
    if !lacks.is_empty() { panic!("level '{}': this start does not carry {} parts of the file:\n  {}", loaded.name, lacks.len(), lacks.join("\n  ")); }
    loaded
}

/// what the loader leaves behind of a record BY RULE, as the TypeScript loader does: the floor a thing is on (one floor is played), an author's notes
const PASSED_BY: [&str; 2] = ["floor", "notes"];

fn read(g: &mut Game, data: &Json, dressing: bool, lived: bool) -> (Loaded, Vec<String>) {
    let mut lacks: Vec<String> = Vec::new();
    if data.get("version").num() != Some(1.0) { lacks.push(format!("version {:?}: this reader reads version 1", data.get("version"))); }
    if !data.get("connectors").items().is_empty() { lacks.push(format!("connectors[]: {} (a stair or a ladder between floors: the TypeScript loader passes them by too)", data.get("connectors").items().len())); }
    let name = text(data.get("name"), "name");
    // (the editor's and the generator's own words, the floors' list and the paint rows nothing wears: read by nobody in the game)
    data.get("floors").skip_all();
    data.get("meta").skip(&["grid", "bounds", "generator", "notes"]);
    // (an objective and a beacon are found by their kind and their box: the file's ids for them name nothing in the game)
    for o in data.get("mission").get("objectives").items() { o.skip(&["id"]); }
    for b in data.get("beacons").items() { b.skip(&["id"]); }
    for (_, row) in data.get("materials").members() { row.skip(&["albedo", "rough", "metal", "stuff", "does", "notes"]); }
    let paint = Paint { name: &name, mats: data.get("materials") };
    // (the DRESSING and the LIVED things are left out with their switch off: every other box keeps its index)
    let is_lived = |id: Option<&str>| id.map_or(false, |s| s.starts_with("lived."));
    let stays = |b: &&Json| { let group = b.get("group").str(); let keep = (dressing || group != Some("dressing")) && (lived || !is_lived(group)); if !keep { b.skip_all(); } keep };
    let file_boxes: Vec<&Json> = data.get("boxes").items().iter().filter(stays).collect();
    let keep = |j: &&Json| { let keep = lived || !is_lived(j.get("id").str()); if !keep { j.skip_all(); } keep };
    // boxes, in file order (index i of the file IS static box i)
    let mut ids: Vec<String> = Vec::new();
    let mut boxes: Vec<BoxId> = Vec::new();
    for b in file_boxes.iter() {
        b.skip(&PASSED_BY);
        ids.push(text(b.get("id"), "a box's id"));
        let mut p = MakeBoxP { c: v3(b.get("c"), "a box's c"), h: v3(b.get("h"), "a box's h"), ..blank_box() };
        p.yaw = Some(b.get("yaw").num().unwrap_or(0.0));
        p.albedo = Some(paint.albedo(b));
        p.emissive = Some(b.get("emissive").v3().unwrap_or([0.0, 0.0, 0.0]));
        p.flags = Some(flags_to_bits(b.get("flags")));
        if b.has("rot") { let r = b.get("rot").nums(); p.rot = Some([r[0], r[1], r[2], r[3]]); }
        if let Some(n) = b.get("name").str() { p.name = Some(String::from(n)); }
        if let Some(x) = b.get("cutHeight").num() { p.cut_height = Some(x); }
        p.marks = copy_markings(b.get("markings"));
        p.rough = paint.rough(b); p.metal = paint.metal(b); p.stuff = paint.stuff(b); p.does = paint.does(b);
        if b.has("shape") { p.shape = Some(shape_of(b.get("shape"), &paint)); }
        boxes.push(game::scene_boxes::make_box(g, &p));
    }
    let bx = |id: &Json| -> f64 { let id = id.str().unwrap_or(""); ids.iter().position(|x| x == id).unwrap_or_else(|| panic!("level '{}': unknown box '{}'", name, id)) as f64 };
    // loose props
    let props: Vec<PropDef> = data.get("props").items().iter().filter(keep).map(|p| {
        let parts: Vec<PropPart> = p.get("parts").items().iter().map(|q| PropPart { off: v3(q.get("off"), "a part's off"), h: v3(q.get("h"), "a part's h"), yaw: q.get("yaw").num().unwrap_or(0.0), albedo: paint.albedo(q), marks: copy_markings(q.get("markings")), stuff: paint.stuff(q), does: paint.does(q) }).collect();
        let (half, height) = fit_prop_footprint(&parts);
        PropDef {
            kind: prop_kind(p.get("kind").str().unwrap_or("")).unwrap_or_else(|| panic!("level '{}': a prop of kind {:?}", name, p.get("kind"))),
            name: String::from(p.get("name").str().or(p.get("id").str()).unwrap_or("")), x: num(p.get("x"), "a prop's x"), z: num(p.get("z"), "a prop's z"), yaw: p.get("yaw").num().unwrap_or(0.0),
            half: if p.has("half") { let h = p.get("half").nums(); [h[0], h[1]] } else { half }, height: p.get("height").num().unwrap_or(height), mass: num(p.get("mass"), "a prop's mass"), parts,
        }
    }).collect();
    // lights
    let light_of = |l: &Json| LightDef {
        kind: light_kind(l.get("kind").str().unwrap_or("")).unwrap_or_else(|| panic!("level '{}': a light of kind {:?}", name, l.get("kind"))),
        pos: v3(l.get("pos"), "a light's pos"), dir: v3(l.get("dir"), "a light's dir"), color: v3(l.get("color"), "a light's color"), intensity: num(l.get("intensity"), "a light's intensity"),
        range: num(l.get("range"), "a light's range"), inner_deg: num(l.get("innerDeg"), "innerDeg"), outer_deg: num(l.get("outerDeg"), "outerDeg"), name: text(l.get("id"), "a light's id"), group: text(l.get("group"), "a light's group"),
        enabled: l.get("enabled").bool().unwrap_or(false), volumetric: num(l.get("volumetric"), "volumetric"),
        fixture_box: if l.has("fixture") { Some(bx(l.get("fixture"))) } else { None }, radius: l.get("radius").num(), mains: l.get("mains").bool(), broad: l.get("broad").bool(),
    };
    let lights: Vec<LightDef> = data.get("lights").items().iter().filter(keep).map(light_of).collect();
    let switchables: Vec<LevelSwitchablesItem> = data.get("switchables").items().iter().filter(keep).map(|s| LevelSwitchablesItem {
        name: text(s.get("id"), "a switchable's id"), boxes: s.get("boxes").items().iter().map(&bx).collect(), group: text(s.get("group"), "a switchable's group"), on: s.get("on").bool().unwrap_or(false),
        base_emissive: v3(s.get("baseEmissive"), "baseEmissive"), flicker: s.get("flicker").bool(), mains: s.get("mains").bool(),
    }).collect();
    let doors: Vec<LevelDoorDef> = data.get("doors").items().iter().map(|o| LevelDoorDef {
        name: text(o.get("id"), "a door's id"), hinge: { let h = o.get("hinge").nums(); [h[0], h[1]] }, closed_dir: num(o.get("closedDir"), "closedDir"), width: num(o.get("width"), "a door's width"),
        angle: o.get("angle").num(), exterior: o.get("exterior").bool(), closer: o.get("closer").bool(), min_angle: o.get("minAngle").num(), max_angle: o.get("maxAngle").num(),
        locked: o.get("locked").bool(), lock: o.get("lock").str().map(String::from), key_side: o.get("keySide").num(), steel: o.get("steel").bool(), swing: o.get("swing").num(),
    }).collect();
    let routes: Vec<RouteId> = data.get("routes").items().iter().map(|r| {
        let route = PatrolRoute {
            name: String::from(r.get("name").str().or(r.get("id").str()).unwrap_or("")), points: r.get("points").items().iter().map(|p| v3(p, "a route's point")).collect(), wait: r.get("wait").nums(),
            weapon: r.get("weapon").str().map(|w| weapon_word(w).unwrap_or_else(|| panic!("level '{}': a route's weapon '{}'", name, w))), keys: if r.has("keys") { Some(r.get("keys").items().iter().map(|k| text(k, "a key")).collect()) } else { None },
        };
        g.tab.route.push(route)
    }).collect();
    let chokepoints: Vec<Chokepoint> = data.get("chokepoints").items().iter().map(|c| Chokepoint { name: String::from(c.get("name").str().or(c.get("id").str()).unwrap_or("")), pos: v3(c.get("pos"), "a chokepoint's pos"), yaw: num(c.get("yaw"), "a chokepoint's yaw") }).collect();
    // mission: the fixed thread's three objectives back onto Level.mission's shape (the validator guaranteed one of each)
    let ob = |k: &str| data.get("mission").get("objectives").items().iter().find(|o| o.get("kind").str() == Some(k)).unwrap_or_else(|| panic!("level '{}': the mission has no '{}' objective", name, k));
    let (reach, drive, exfil) = (ob("reach"), ob("drive"), ob("exfil"));
    let area = rect(reach.get("area"), "the reach objective's area");
    let records = typed(data);
    let mut level = Level::zeroed();
    level.props = props; level.lights = lights; level.switchables = switchables; level.doors = doors;
    level.devices = Some(records.devices); level.vents = records.vents; level.civilians = records.civilians; level.breach = records.breach;
    if data.has("daylight") {
        let d = data.get("daylight");
        level.daylight = Some(DaylightDef {
            sun: light_of(d.get("sun")), sky: DaylightDefSky { zenith: v3(d.get("sky").get("zenith"), "daylight.sky.zenith"), horizon: v3(d.get("sky").get("horizon"), "daylight.sky.horizon") },
            groups_on: texts(d.get("groupsOn"), "daylight.groupsOn"), groups_off: texts(d.get("groupsOff"), "daylight.groupsOff"),
            lights_off: texts(d.get("lightsOff"), "daylight.lightsOff"), haze: num(d.get("haze"), "daylight.haze"), exposure: num(d.get("exposure"), "daylight.exposure"),
        });
    }
    level.breaker = LevelBreaker { pos: v3(data.get("breaker").get("pos"), "the breaker's pos"), boxes: vec![bx(data.get("breaker").get("panel")), bx(data.get("breaker").get("led"))], fumble: data.get("breaker").get("fumble").num() };
    level.beacons = data.get("beacons").items().iter().map(|b| LevelBeaconsItem { pos: v3(b.get("pos"), "a beacon's pos"), box_: bx(b.get("box")) }).collect();
    level.player_spawn = v3(data.get("mission").get("playerSpawn"), "mission.playerSpawn");
    level.routes = routes; level.chokepoints = Some(chokepoints); level.clear_plans = Some(records.clear_plans);
    if !data.get("volumes").items().is_empty() {
        level.volumes = Some(data.get("volumes").items().iter().map(|v| VolumeDef { name: text(v.get("id"), "a volume's id"), kind: volume_kind(v.get("kind").str().unwrap_or("")).expect("a volume's kind"), c: v3(v.get("c"), "a volume's c"), h: v3(v.get("h"), "a volume's h"), yaw: v.get("yaw").num() }).collect());
    }
    if !data.get("marks").items().is_empty() {
        level.marks = Some(data.get("marks").items().iter().map(|m| MarkDef { name: text(m.get("id"), "a mark's id"), pos: v3(m.get("pos"), "a mark's pos"), yaw: m.get("yaw").num() }).collect());
    }
    level.sky = LevelSky { zenith: v3(data.get("sky").get("zenith"), "sky.zenith"), horizon: v3(data.get("sky").get("horizon"), "sky.horizon") };
    level.mission = game::quiet::QuietMissionAuthoredMission {
        server_room: game::quiet::QuietMissionAuthoredMissionServerRoom { x0: area[0], x1: area[1], z0: area[2], z1: area[3] }, entry_door: text(reach.get("door"), "the reach objective's door"),
        rack: game::quiet::QuietMissionAuthoredMissionRack { index: num(drive.get("rackIndex"), "rackIndex"), front: v3(drive.get("front"), "the drive's front"), half_w: num(drive.get("halfW"), "halfW") },
        exfil_door: text(exfil.get("door"), "the exfil's door"), exfil_x: num(exfil.get("x"), "the exfil's x"), exfil_z: num(exfil.get("z"), "the exfil's z"),
        exfil_out: if exfil.has("out") { let o = exfil.get("out").nums(); Some([o[0], o[1]]) } else { None },
    };
    level.mark_texel = data.get("meta").get("markTexel").num();
    level.kept = data.get("meta").get("kept").num();
    // the scenarios' block and the footsteps' map: the runtime structs verbatim (scene/scenarios.ts), each record as the game's own type (records.rs)
    level.scenarios = records.scenarios; level.surfaces = records.surfaces;
    level.script = game::director::level_script_of(&name);   // (a level that carries a script: scene/levelScripts.ts, by the file's name)
    g.level = level;
    g.m.scene_level_load.level_names = Some(name.clone());
    data.unread("", &mut lacks);
    (Loaded { name, boxes }, lacks)
}

/// `makeBox`'s argument with nothing said but a place and a size
pub fn blank_box() -> MakeBoxP {
    MakeBoxP { c: [0.0; 3], h: [0.0; 3], yaw: None, albedo: None, emissive: None, flags: None, owner: None, cut_height: None, wet_k: None, rot: None, dc: None, dh: None, name: None, rough: None, metal: None, stuff: None, does: None,
        aniso: None, marks: None, mark_gain: None, fog_emit: None, fog_mark: None, mkey: None, cut: None, cut_core: None, cut_glass: None, cut_round: None, cut_arc: None, cut_old: None, cut_core_stuff: None, round: None, was: None,
        shape: None, shaped: None, piece: None, of: None, dress: None }
}
