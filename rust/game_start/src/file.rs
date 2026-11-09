//! A LEVEL FILE, AS IT IS WRITTEN: every record of it and the member each field is (docs/level-format.md). This is the one place the format is read: `level.rs` lays what was read into the game, and
//! `faults.rs` holds it to the rules a file can be held to by itself.
//!
//! WHAT IS NOT CARRIED IS SAID, NEVER PASSED OVER: what no type here has a field for is named by its place (`json.rs unread`). What is left behind by rule says so in so many words (`drops`: the
//! floor a thing is on, for one floor is played; an author's notes).
use crate::json::Json;
use crate::json_struct;
use crate::json_words;
use crate::kit_file::KitView;
use crate::read::{blank, field, At, Faults, FromJson};
use alloc::{format, string::String, vec::Vec};
use game::js::JsMap;
use game::scene_level::{BreachPlan, CivilianDef, DeviceDef, RoomClearPlan, VentDef};
use game::scene_level_data::{PropKindName, SurfacesData};
use game::scene_markings::MarkingDef;
use game::scene_scenarios::LevelScenarios;
use game::scene_shapes::ShapeFormArc;
use game::weapons::WeaponId;
use game::{Quat, V3};

/// the version this reader reads
pub const VERSION: f64 = 2.0;

/// one storey. A file has them all and the game plays the first
pub struct Floor { pub id: String, pub name: Option<String>, pub y: f64, pub ceiling: f64 }
json_struct!(Floor { id: "id", name: "name", y: "y", ceiling: "ceiling" });
/// the traceable world the level must fit
pub struct Bounds { pub min: V3, pub max: V3 }
json_struct!(Bounds { min: "min", max: "max" });
/// the editor's and the generator's own words, and the two the picture reads
pub struct Meta { pub grid: Option<f64>, pub bounds: Option<Bounds>, pub generator: Option<String>, pub notes: Option<String>, pub mark_texel: Option<f64>, pub kept: Option<f64> }
json_struct!(Meta { grid: "grid", bounds: "bounds", generator: "generator", notes: "notes", mark_texel: "markTexel", kept: "kept" });
/// a named paint: its colour, its finish, what it is made of and what it does
pub struct Material { pub albedo: V3, pub rough: Option<f64>, pub metal: Option<bool>, pub stuff: Option<String>, pub does: Option<String>, pub notes: Option<String> }
json_struct!(Material { albedo: "albedo", rough: "rough", metal: "metal", stuff: "stuff", does: "does", notes: "notes" });
/// one box: its place and size, its paint and finish, what it is made of and does, the marks on it and the shape it is drawn in. `group`: the optional furniture it goes with
pub struct BoxData {
    pub id: String, pub group: Option<String>, pub name: Option<String>, pub c: V3, pub h: V3, pub yaw: Option<f64>, pub rot: Option<Quat>, pub round: Option<bool>, pub material: Option<String>,
    pub albedo: Option<V3>, pub emissive: Option<V3>, pub rough: Option<f64>, pub metal: Option<bool>, pub stuff: Option<String>, pub does: Option<String>, pub flags: Option<Vec<FlagName>>,
    pub cut_height: Option<f64>, pub markings: Option<Vec<MarkingDef>>, pub shape: Option<ShapeData>,
}
json_struct!(BoxData {
    id: "id", group: "group", name: "name", c: "c", h: "h", yaw: "yaw", rot: "rot", round: "round", material: "material", albedo: "albedo", emissive: "emissive", rough: "rough", metal: "metal",
    stuff: "stuff", does: "does", flags: "flags", cut_height: "cutHeight", markings: "markings", shape: "shape",
} drops { "floor": Option<String> });
#[derive(PartialEq)]
pub enum FlagName { NoPrimary, NoShadow, Cutaway, Gloss, Glass }
json_words!(FlagName { NoPrimary: "NoPrimary", NoShadow: "NoShadow", Cutaway: "Cutaway", Gloss: "Gloss", Glass: "Glass" });
/// the shape a box is drawn in: the pieces laid with it, its rounding, its arc and what is cut off it
pub struct ShapeData { pub with: Option<Vec<ShapePieceData>>, pub round: Option<f64>, pub arc: Option<ShapeFormArc>, pub cut: Option<Vec<[f64; 4]>>, pub old: Option<bool> }
json_struct!(ShapeData { with: "with", round: "round", arc: "arc", cut: "cut", old: "old" });
pub struct ShapePieceData {
    pub off: V3, pub h: V3, pub yaw: Option<f64>, pub material: Option<String>, pub albedo: Option<V3>, pub rough: Option<f64>, pub metal: Option<bool>, pub does: Option<String>,
    pub stuff: Option<String>, pub round: Option<f64>, pub arc: Option<ShapeFormArc>, pub cut: Option<Vec<[f64; 4]>>, pub old: Option<bool>,
}
json_struct!(ShapePieceData {
    off: "off", h: "h", yaw: "yaw", material: "material", albedo: "albedo", rough: "rough", metal: "metal", does: "does", stuff: "stuff", round: "round", arc: "arc", cut: "cut", old: "old",
});
/// one lamp: `fixture` is the box it burns in
pub struct LightData {
    pub id: String, pub kind: LightDataKind, pub pos: V3, pub dir: V3, pub color: V3, pub intensity: f64, pub range: f64, pub inner_deg: f64, pub outer_deg: f64, pub group: String, pub enabled: bool,
    pub volumetric: f64, pub radius: Option<f64>, pub mains: Option<bool>, pub broad: Option<bool>, pub fixture: Option<String>,
}
json_struct!(LightData {
    id: "id", kind: "kind", pos: "pos", dir: "dir", color: "color", intensity: "intensity", range: "range", inner_deg: "innerDeg", outer_deg: "outerDeg", group: "group", enabled: "enabled",
    volumetric: "volumetric", radius: "radius", mains: "mains", broad: "broad", fixture: "fixture",
});
#[derive(Clone, Copy, PartialEq)]
pub enum LightDataKind { Spot, Dir, Point }
json_words!(LightDataKind { Spot: "spot", Dir: "dir", Point: "point" });
pub struct DoorData {
    pub id: String, pub hinge: [f64; 2], pub closed_dir: f64, pub width: f64, pub angle: Option<f64>, pub exterior: Option<bool>, pub closer: Option<bool>, pub min_angle: Option<f64>,
    pub max_angle: Option<f64>, pub locked: Option<bool>, pub lock: Option<String>, pub key_side: Option<f64>, pub swing: Option<f64>, pub steel: Option<bool>,
}
json_struct!(DoorData {
    id: "id", hinge: "hinge", closed_dir: "closedDir", width: "width", angle: "angle", exterior: "exterior", closer: "closer", min_angle: "minAngle", max_angle: "maxAngle", locked: "locked",
    lock: "lock", key_side: "keySide", swing: "swing", steel: "steel",
} drops { "floor": Option<String> });
json_words!(PropKindName { Cardboard: "cardboard", Plant: "plant", Chair: "chair", Bin: "bin" });
/// one box of a loose thing
pub struct Part { pub off: V3, pub h: V3, pub yaw: Option<f64>, pub material: Option<String>, pub albedo: Option<V3>, pub stuff: Option<String>, pub does: Option<String>, pub markings: Option<Vec<MarkingDef>> }
json_struct!(Part { off: "off", h: "h", yaw: "yaw", material: "material", albedo: "albedo", stuff: "stuff", does: "does", markings: "markings" });
/// loose furniture: `half` and `height` are fitted from its parts where it says none
pub struct Prop {
    pub id: String, pub kind: PropKindName, pub name: Option<String>, pub x: f64, pub z: f64, pub yaw: Option<f64>, pub mass: f64, pub parts: Vec<Part>, pub half: Option<[f64; 2]>, pub height: Option<f64>,
}
json_struct!(Prop { id: "id", kind: "kind", name: "name", x: "x", z: "z", yaw: "yaw", mass: "mass", parts: "parts", half: "half", height: "height" } drops { "floor": Option<String> });
/// emissive fixtures that are switched
pub struct Switchable { pub id: String, pub boxes: Vec<String>, pub group: String, pub on: bool, pub base_emissive: V3, pub flicker: Option<bool>, pub mains: Option<bool> }
json_struct!(Switchable { id: "id", boxes: "boxes", group: "group", on: "on", base_emissive: "baseEmissive", flicker: "flicker", mains: "mains" });
json_words!(WeaponId {
    Fiveseven: "fiveseven", Guard9mm: "guard9mm", Aks74u: "aks74u", Ak74m: "ak74m", Akms: "akms", Fn2000: "fn2000", M870: "m870", Mk12: "mk12", Mp7: "mp7", Mk46: "mk46", M1014: "m1014", Hk45: "hk45",
    G18: "g18", Pkm: "pkm", Mk18bare: "mk18bare", G19bare: "g19bare",
});
/// one man's beat: known by its `name`, else its id
pub struct Route { pub id: String, pub name: Option<String>, pub points: Vec<V3>, pub wait: Vec<f64>, pub weapon: Option<WeaponId>, pub keys: Option<Vec<String>> }
json_struct!(Route { id: "id", name: "name", points: "points", wait: "wait", weapon: "weapon", keys: "keys" });
pub struct Chokepoint { pub id: String, pub name: Option<String>, pub pos: V3, pub yaw: f64 }
json_struct!(Chokepoint { id: "id", name: "name", pos: "pos", yaw: "yaw" } drops { "floor": Option<String> });
/// what a volume is for: the author's word, and no rule's
pub enum VolumeKind { Trigger, Hide, Utility }
json_words!(VolumeKind { Trigger: "trigger", Hide: "hide", Utility: "utility" });
json_struct!(game::scene_level::VolumeDef { name: "id", c: "c", h: "h", yaw: "yaw" } drops { "kind": VolumeKind, "floor": Option<String>, "notes": Option<String> });
json_struct!(game::scene_level::MarkDef { name: "id", pos: "pos" } drops { "yaw": Option<f64> });

/// the one mission thread: a file says one objective of each kind (found by its kind: the file's id for it names nothing in the game)
pub struct Reach { pub area: game::scene_level_data::Rect, pub door: String }
json_struct!(Reach { area: "area", door: "door" } drops { "id": String });
pub struct Drive { pub rack_index: f64, pub front: V3, pub half_w: f64 }
json_struct!(Drive { rack_index: "rackIndex", front: "front", half_w: "halfW" } drops { "id": String });
pub struct Exfil { pub door: String, pub x: f64, pub z: f64, pub out: Option<[f64; 2]> }
json_struct!(Exfil { door: "door", x: "x", z: "z", out: "out" } drops { "id": String });
pub struct Mission { pub player_spawn: V3, pub reach: Reach, pub drive: Drive, pub exfil: Exfil }
impl FromJson for Mission {
    fn from_json(j: &Json, at: &At) -> Self {
        let at = &at.of_record(j);
        let player_spawn = field(j, "playerSpawn", at);
        let (list, here) = (j.get("objectives").items(), at.key("objectives"));
        fn one<T: FromJson>(list: &[Json], here: &At, kind: &str) -> T {
            // (two of a kind: each is read, so that neither's members are unread)
            let mut read: Vec<T> = (0..list.len()).filter(|&i| list[i].get("kind").str() == Some(kind))
                .map(|i| T::from_json(&list[i], &here.item(i, list[i].peek("id").and_then(Json::str)))).collect();
            if read.len() != 1 { here.fault(&format!("exactly one '{kind}' objective (found {})", read.len())); }
            read.pop().unwrap_or_else(blank)
        }
        let mission = Mission { player_spawn, reach: one(list, &here, "reach"), drive: one(list, &here, "drive"), exfil: one(list, &here, "exfil") };
        for (i, o) in list.iter().enumerate() {
            if !matches!(o.get("kind").str(), Some("reach" | "drive" | "exfil")) { o.skip_all(); here.item(i, None).key("kind").wants_word(&["reach", "drive", "exfil"], o.get("kind")); }
        }
        mission
    }
}
/// the mains panel: `panel` and `led` are boxes
pub struct Breaker { pub pos: V3, pub panel: String, pub led: String, pub fumble: Option<f64> }
json_struct!(Breaker { pos: "pos", panel: "panel", led: "led", fumble: "fumble" });
pub struct Beacon { pub id: String, pub pos: V3, pub box_: String }
json_struct!(Beacon { id: "id", pos: "pos", box_: "box" });
pub struct Sky { pub zenith: V3, pub horizon: V3 }
json_struct!(Sky { zenith: "zenith", horizon: "horizon" });
pub struct Daylight { pub sun: LightData, pub sky: Sky, pub groups_on: Vec<String>, pub groups_off: Vec<String>, pub lights_off: Vec<String>, pub haze: f64, pub exposure: f64 }
json_struct!(Daylight { sun: "sun", sky: "sky", groups_on: "groupsOn", groups_off: "groupsOff", lights_off: "lightsOff", haze: "haze", exposure: "exposure" });

/// a drill's plan and the file's id of it
pub struct Plan<T> { pub id: String, pub plan: T }
impl<T: FromJson> FromJson for Plan<T> {
    fn from_json(j: &Json, at: &At) -> Self {
        let at = &at.of_record(j);
        Plan { id: field(j, "id", at), plan: T::from_json(j, at) }
    }
}
pub struct Plans { pub room_clear: Vec<Plan<RoomClearPlan>>, pub breach: Option<Plan<BreachPlan>> }
json_struct!(Plans { room_clear: "roomClear", breach: "breach" });

/// the missions' block: what the game keeps of it, and the Team & kit picture's part
pub struct Scenarios { pub kept: LevelScenarios, pub kit_view: Option<KitView> }
impl FromJson for Scenarios {
    fn from_json(j: &Json, at: &At) -> Self {
        let at = &at.of_record(j);
        Scenarios { kept: LevelScenarios::from_json(j, at), kit_view: field(j, "kitView", at) }
    }
}

/// a list nothing has ever stood in: the file says it empty
pub struct Nothing;
impl FromJson for Nothing {
    fn from_json(j: &Json, at: &At) -> Self { j.skip_all(); at.fault("a way between floors (a stair, a ladder) is in no game yet: the list is empty"); Nothing }
}

pub struct LevelFile {
    pub version: f64, pub name: String, pub meta: Meta, pub floors: Vec<Floor>, pub materials: JsMap<String, Material>, pub surfaces: Option<SurfacesData>,
    pub boxes: Vec<BoxData>, pub props: Vec<Prop>, pub lights: Vec<LightData>, pub switchables: Vec<Switchable>, pub doors: Vec<DoorData>, pub routes: Vec<Route>, pub chokepoints: Vec<Chokepoint>,
    pub civilians: Option<Vec<CivilianDef>>, pub volumes: Vec<game::scene_level::VolumeDef>, pub marks: Option<Vec<game::scene_level::MarkDef>>, pub devices: Vec<DeviceDef>, pub vents: Option<Vec<VentDef>>,
    pub connectors: Vec<Nothing>, pub mission: Mission, pub breaker: Breaker, pub beacons: Vec<Beacon>, pub sky: Sky, pub daylight: Option<Daylight>, pub plans: Plans, pub scenarios: Option<Scenarios>,
}
json_struct!(LevelFile {
    version: "version", name: "name", meta: "meta", floors: "floors", materials: "materials", surfaces: "surfaces", boxes: "boxes", props: "props", lights: "lights", switchables: "switchables", doors: "doors",
    routes: "routes", chokepoints: "chokepoints", civilians: "civilians", volumes: "volumes", marks: "marks", devices: "devices", vents: "vents", connectors: "connectors", mission: "mission",
    breaker: "breaker", beacons: "beacons", sky: "sky", daylight: "daylight", plans: "plans", scenarios: "scenarios",
});

/// THE FILE READ: the level it is, where nothing about the reading is wrong, and every fault of the reading (a value that is not what its place wants, a member the game has no reading of).
pub fn read(data: &Json) -> (Option<LevelFile>, Vec<String>) {
    let faults = Faults::default();
    let at = At::top(&faults);
    let v = data.get("version");
    if v.num() != Some(VERSION) {
        at.key("version").fault(&if v.num() == Some(1.0) { String::from("version 1 writes a round thing as four slabs: bun run tools/levels/round-convert.ts <file> makes it version 2") }
            else { format!("this build reads version {VERSION}, and the file says {}", crate::json::to_line(v)) });
        return (None, faults.all());
    }
    let file = LevelFile::from_json(data, &at);
    let mut unread = Vec::new();
    data.unread("", &mut unread);
    for place in unread { faults.say(place, "nothing in the game reads a member of this name here"); }
    let faults = faults.all();
    (faults.is_empty().then_some(file), faults)
}
