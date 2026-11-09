//! THE RECORDS OF A LEVEL FILE THAT THE GAME KEEPS AS ITS OWN TYPES, each type with the member of the file each of its fields is (`read.rs`
//! reads them). A type that gains a field does not compile until its line here names the member, and a member of the file no line names is found by
//! the file's own tree (`json.rs unread`).
#![allow(clippy::all)]
use crate::read::{field, record, FromJson};
use crate::json::Json;
use alloc::string::String;
use crate::{json_struct, json_words};

json_struct!(game::scene_scenarios::TeamStart { sam: "sam", sam_yaw: "samYaw", ops: "ops", ops_yaw: "opsYaw" });
json_struct!(game::scene_scenarios::NightBreachData { team: "team", room: "room" } drops ["lights"]);
json_struct!(game::scene_scenarios::Rect { x0: "x0", x1: "x1", z0: "z0", z1: "z1" });
json_struct!(game::scene_scenarios::StructureDoor { id: "id", from: "from" });
json_struct!(game::scene_scenarios::StructureMark { pos: "pos", watch: "watch", aim_at: "aimAt" });
json_struct!(game::scene_scenarios::StructureRoom { id: "id", name: "name", bounds: "bounds", doors: "doors", garrison: "garrison", marks: "marks" });
json_words!(game::scene_scenarios::SquadDataTraining { Trained: "trained", Green: "green", Mixed: "mixed" });
json_struct!(game::scene_scenarios::SquadArrive { door: "door", spawn: "spawn", men: "men" });
json_struct!(game::scene_scenarios::SquadData { id: "id", name: "name", rooms: "rooms", rally: "rally", barricade: "barricade", may_leave: "mayLeave", training: "training", arrive: "arrive" });
json_struct!(game::scene_scenarios::StructureData { name: "name", team: "team", rooms: "rooms", starts: "starts", max_men: "maxMen", way_out: "wayOut", dig_in: "digIn", wall_charges: "wallCharges", squads: "squads" });
json_struct!(game::scene_scenarios::RescueDataRoom { name: "name", where_: "where", door: "door", bounds: "bounds", door_aim: "doorAim", outside: "outside", hold: "hold" } drops ["lights"]);
json_struct!(game::scene_scenarios::RescueDataSideFrom { door: "door", bounds: "bounds" });
json_struct!(game::scene_scenarios::RescueDataSide { door: "door", hold: "hold", from: "from", pair: "pair", stack: "stack", standoff: "standoff" });
json_struct!(game::scene_scenarios::RescueDataHostagesItem { pos: "pos", face: "face" });
json_struct!(game::scene_scenarios::ScenarioMark { pos: "pos", aim_at: "aimAt" });
json_struct!(game::scene_scenarios::RescueDataDoorMan { inside: "inside", outside: "outside" });
json_struct!(game::scene_scenarios::RescueDataCast { roamers: "roamers" });
json_struct!(game::scene_scenarios::RescueFlowDataRegionsItem { id: "id", door: "door", rect: "rect" });
json_words!(game::scene_scenarios::RouteOpeningKind { Door: "door", Glass: "glass", Doorway: "doorway" });
json_words!(game::scene_scenarios::RouteOpeningSide { Left: "left", Right: "right" });
json_struct!(game::scene_scenarios::RouteOpening { id: "id", name: "name", short: "short", region: "region", kind: "kind", door: "door", at: "at", mark: "mark", watch: "watch", half: "half", north: "north", call: "call", side: "side", look: "look" });
json_struct!(game::scene_scenarios::RescueFlowDataHold { ahead: "ahead", x_max: "xMax", z: "z" });
json_struct!(game::scene_scenarios::RescueFlowData { corridor: "corridor", regions: "regions", route: "route", hold: "hold" });
json_struct!(game::scene_scenarios::RescueData { team: "team", room: "room", side: "side", hostages: "hostages", captor: "captor", door_man: "doorMan", roamers: "roamers", guards: "guards", cast: "cast", flow: "flow" });
json_struct!(game::scene_scenarios::PlaceArea { name: "name", bounds: "bounds" });
json_struct!(game::scene_scenarios::PlacesData { areas: "areas" });
json_struct!(game::scene_scenarios::QuietStart { sam: "sam", sam_yaw: "samYaw", mate: "mate", name: "name" });
json_struct!(game::scene_scenarios::QuietSpot { place: "place", area: "area", door: "door", holder: "holder", rack_index: "rackIndex", front: "front", half_w: "halfW", wired: "wired" });
json_struct!(game::scene_scenarios::QuietExit { name: "name", door: "door", x: "x", z: "z", out: "out", half_w: "halfW" });
json_struct!(game::scene_scenarios::QuietStand { name: "name", pos: "pos", face: "face", loop_: "loop" });
json_struct!(game::scene_scenarios::QuietDataNeeds { coffee: "coffee", smoke: "smoke", water: "water", stretch: "stretch" });
json_struct!(game::scene_scenarios::QuietData { thing: "thing", starts: "starts", spots: "spots", exits: "exits", patrols: "patrols", needs: "needs", phones: "phones", wayouts: "wayouts", radio_check: "radioCheck" });
json_struct!(game::scene_scenarios::LessonStepEye { at: "at", yaw: "yaw", fov: "fov" });
json_struct!(game::scene_scenarios::LessonStepExplain { title: "title", text: "text", quiet: "quiet", now: "now" });
json_struct!(game::scene_scenarios::LessonStepOnWay { nv: "nv", run: "run" });
json_struct!(game::scene_scenarios::LessonStepAfter { line: "line", on: "on", word: "word" });
json_struct!(game::scene_scenarios::LessonStep { id: "id", line: "line", setup_line: "setupLine", ring_line: "ringLine", blow_line: "blowLine", menu_line: "menuLine", page_line: "pageLine", flat_line: "flatLine", done: "done", hold: "hold", settle: "settle", at: "at", at_man: "atMan", allow: "allow", deny: "deny", cue: "cue", checkpoint: "checkpoint", fog: "fog", no_tap: "noTap", eye: "eye", explain: "explain", toward: "toward", on_way: "onWay", after: "after" });
json_struct!(game::scene_scenarios::RailsDataWaits { low: "low" });
json_struct!(game::scene_scenarios::RailsDataSkyEye { at: "at", yaw: "yaw", fov: "fov" });
json_struct!(game::scene_scenarios::RailsDataSky { park: "park", reach: "reach", eye: "eye", low: "low" });
json_struct!(game::scene_scenarios::RailsData { allow: "allow", respawn: "respawn", fog: "fog", waits: "waits", sky: "sky", lasers: "lasers", orders: "orders", flat_ring: "flatRing", rows: "rows" });
json_struct!(game::scene_scenarios::LessonData { title: "title", steps: "steps", rails: "rails" });
json_struct!(game::scene_scenarios::ParData { gold: "gold", silver: "silver", bronze: "bronze", loss_secs: "lossSecs" });
json_words!(game::kit::KitWeapon { Pistol: "pistol", Rifle: "rifle", Shotgun: "shotgun" });
json_words!(game::scene_scenarios::IssueCardLoaded { Ring: "ring", He40: "he40" });
json_struct!(game::scene_scenarios::IssueCard { weapon: "weapon", suppressed: "suppressed", light: "light", nvg: "nvg", bangs: "bangs", charges: "charges", smoke: "smoke", frags: "frags", walls: "walls", shells: "shells", rings: "rings", loaded: "loaded" });
json_struct!(game::scene_scenarios::IssueData { members: "members" });
json_words!(game::scene_level_data::FlagName { NoPrimary: "NoPrimary", NoShadow: "NoShadow", Cutaway: "Cutaway", Gloss: "Gloss", Glass: "Glass" });
json_words!(game::scene_markings::MarkFace { X: "+x", X2: "-x", Y: "+y", Y2: "-y", Z: "+z", Z2: "-z" });
json_words!(game::scene_markings::MarkRes { Base: "base", Fine: "fine", Coarse: "coarse" });
json_struct!(game::scene_markings::MarkPal { albedo: "albedo", mul: "mul", rough: "rough", emit: "emit" });
json_struct!(game::scene_markings::MarkingDef { face: "face", res: "res", at: "at", size: "size", pal: "pal", cells: "cells" });
json_struct!(game::scene_shapes::ShapeFormArc { axis: "axis", rise: "rise" });
json_struct!(game::scene_level_data::ShapePieceData { off: "off", h: "h", yaw: "yaw", material: "material", albedo: "albedo", rough: "rough", metal: "metal", does: "does", stuff: "stuff", round: "round", arc: "arc", cut: "cut", old: "old" });
json_struct!(game::scene_level_data::ShapeData { with: "with", round: "round", arc: "arc", cut: "cut", old: "old" });
json_struct!(game::scene_level_data::BoxData { id: "id", name: "name", c: "c", h: "h", yaw: "yaw", rot: "rot", round: "round", material: "material", albedo: "albedo", emissive: "emissive", rough: "rough", metal: "metal", stuff: "stuff", does: "does", flags: "flags", cut_height: "cutHeight", markings: "markings", shape: "shape" } drops ["group", "floor"]);
json_words!(game::scene_level_data::LightDataKind { Spot: "spot", Dir: "dir", Point: "point" });
json_struct!(game::scene_level_data::LightData { id: "id", kind: "kind", pos: "pos", dir: "dir", color: "color", intensity: "intensity", range: "range", inner_deg: "innerDeg", outer_deg: "outerDeg", group: "group", enabled: "enabled", volumetric: "volumetric", radius: "radius", mains: "mains", broad: "broad", fixture: "fixture" });
json_struct!(game::scene_level_data::DoorData { id: "id", hinge: "hinge", closed_dir: "closedDir", width: "width", angle: "angle", exterior: "exterior", closer: "closer", min_angle: "minAngle", max_angle: "maxAngle", locked: "locked", lock: "lock", key_side: "keySide", swing: "swing", steel: "steel" } drops ["floor"]);
json_struct!(game::scene_scenarios::ResponseWay { name: "name", door: "door", muster: "muster", entry: "entry", entry_face: "entryFace", coming: "coming", clear: "clear", stairs: "stairs" });
json_struct!(game::scene_scenarios::ResponsePost { name: "name", pos: "pos", yaw: "yaw", sweep: "sweep" });
json_struct!(game::scene_scenarios::ResponseDataSweepItem { name: "name", pos: "pos" });
json_struct!(game::scene_scenarios::ResponseDataRaid { room: "room" });
json_struct!(game::scene_scenarios::ResponseData { site: "site", at: "at", team: "team", tag: "tag", ways: "ways", men: "men", eta: "eta", posts: "posts", outside: "outside", sweep: "sweep", raid: "raid" } drops ["why"]);
json_struct!(game::scene_scenarios::LevelScenariosDoorNamesItem { door: "door", name: "name" });
json_struct!(game::scene_ways::WayHint { from: "from", to: "to", weight: "weight" });
json_struct!(game::scene_scenarios::LevelScenarios { night_breach: "nightBreach", room_clear: "roomClear", rescue: "rescue", places: "places", quiet: "quiet", lesson: "lesson", par: "par", issue: "issue", response: "response", door_names: "doorNames", walked: "walked" } drops ["kitView"]);

json_struct!(game::scene_level_data::Rect { x0: "x0", x1: "x1", z0: "z0", z1: "z1" });
json_words!(game::floor::FloorMat { Carpet: "carpet", Lino: "lino", Stone: "stone", Tile: "tile", Asphalt: "asphalt", Grass: "grass" });
json_struct!(game::scene_level_data::SurfacesDataZonesItem { mat: "mat", x0: "x0", z0: "z0", x1: "x1", z1: "z1" } drops ["notes", "id"]);
json_struct!(game::scene_level_data::SurfacesData { building: "building", grass_edge: "grassEdge", inside: "inside", outside: "outside", verge: "verge", zones: "zones" });

/// A lesson's test (`{ is: 'in', area: … }`): the word `is` says which test it is, and a word this build has no test for is kept as the word.
impl FromJson for game::scene_scenarios::LessonTestKind {
    fn from_json(j: &Json, at: &str) -> Self {
        use game::scene_scenarios::LessonTestKind as K;
        let is = j.get("is").str().unwrap_or_else(|| panic!("level: {} is a lesson's test and says no `is`: {:?}", at, j));
        match is {
            "in" => { K::In { area: field(j, "area", at) } }
            "near" => { K::Near { at: field(j, "at", at), r: field(j, "r", at) } }
            "flag" => { K::Flag { flag: field(j, "flag", at), value: field(j, "value", at) } }
            "slot" => { K::Slot { slot: field(j, "slot", at), drawn: field(j, "drawn", at) } }
            "pace" => { K::Pace { below: field(j, "below", at), above: field(j, "above", at) } }
            "meter" => { K::Meter { which: field(j, "which", at), below: field(j, "below", at), above: field(j, "above", at) } }
            "stage" => { K::Stage { stage: field(j, "stage", at) } }
            "door" => { K::Door { door: field(j, "door", at), state: field(j, "state", at) } }
            "target" => { K::Target { name: field(j, "name", at), on: field(j, "on", at) } }
            "men" => { K::Men { down: field(j, "down", at) } }
            "did" => { K::Did { what: field(j, "what", at), on: field(j, "on", at), n: field(j, "n", at) } }
            "downed" => { K::Downed { how: field(j, "how", at), n: field(j, "n", at) } }
            "body" => { K::Body { area: field(j, "area", at) } }
            "calm" => { K::Calm { route: field(j, "route", at) } }
            "loaded" => { K::Loaded }
            "reload" => { K::Reload }
            "launcher" => { K::Launcher { on: field(j, "on", at), fired: field(j, "fired", at) } }
            "burst" => { K::Burst { n: field(j, "n", at) } }
            "threw" => { K::Threw { kind: field(j, "kind", at), quick: field(j, "quick", at), area: field(j, "area", at) } }
            "orderTo" => { K::OrderTo { who: field(j, "who", at) } }
            "stackOn" => { K::StackOn { door: field(j, "door", at), who: field(j, "who", at), set: field(j, "set", at) } }
            "pairsOn" => { K::PairsOn { doors: field(j, "doors", at) } }
            "word" => { K::Word { doors: field(j, "doors", at) } }
            "taken" => { K::Taken { room: field(j, "room", at) } }
            "following" => { K::Following }
            "mate" => { K::Mate { area: field(j, "area", at), at: field(j, "at", at), r: field(j, "r", at), by: field(j, "by", at), away: field(j, "away", at) } }
            "pair" => { K::Pair { what: field(j, "what", at), n: field(j, "n", at) } }
            "cue" => { K::Cue { name: field(j, "name", at) } }
            "view" => { K::View { turned: field(j, "turned", at), zoomed: field(j, "zoomed", at), panned: field(j, "panned", at) } }
            "squad" => { K::Squad { area: field(j, "area", at), n: field(j, "n", at) } }
            "stacks" => { K::Stacks { doors: field(j, "doors", at), n: field(j, "n", at), set: field(j, "set", at) } }
            "watching" => { K::Watching { at: field(j, "at", at), r: field(j, "r", at), n: field(j, "n", at) } }
            "all" => { K::All { of: field(j, "of", at) } }
            "any" => { K::Any { of: field(j, "of", at) } }
            w => K::Unknown { is: alloc::string::String::from(w) },
        }
    }
}

// ---- the level's own records that the TypeScript loader reads member for member (levelLoad.ts): a device, a vent and a light say `id` where the
// game's type says `name`; a plan is the file's record without its id
json_words!(game::scene_level::CivilianDefKind { Cleaner: "cleaner", Worker: "worker" });
json_words!(game::scene_level::CivilianTask { Bin: "bin", Stand: "stand", Wipe: "wipe", Sit: "sit", Cooler: "cooler", Printer: "printer", Shelf: "shelf" });
json_struct!(game::scene_level::VentDef { name: "id", pos: "pos", dir: "dir", speed: "speed", spread: "spread", reach: "reach", size: "size", period: "period", duty: "duty", phase: "phase", mains: "mains" } drops ["floor", "notes"]);
json_words!(game::scene_level::RoomClearPlanBreach { Kick: "kick", Open: "open" });
json_struct!(game::scene_level::BreachPlanBounds { x0: "x0", x1: "x1", z0: "z0", z1: "z1" });
json_struct!(game::scene_level::BreachPlanLines { stack1: "stack1", stack2: "stack2", go2: "go2", in1: "in1", side2: "side2", side1: "side1", push1: "push1", under2: "under2", nothing2: "nothing2", corner1: "corner1", empty2: "empty2", radio1: "radio1" });
json_struct!(game::scene_level::RoomClearPlan { door: "door", room: "room", breach: "breach", bounds: "bounds", from: "from", stack: "stack", stack_face: "stackFace", stack_side: "stackSide", stack_aim: "stackAim", kick_from: "kickFrom", kick_face: "kickFace", fling: "fling", entry: "entry", pod_face: "podFace", entry_aim: "entryAim", search_path: "searchPath", search_face: "searchFace", search_aim_walk: "searchAimWalk", search_aim_there: "searchAimThere", under_aim: "underAim", up_aim: "upAim", form_up: "formUp", form_face: "formFace", lines: "lines" } drops ["id"]);
json_struct!(game::scene_level::BreachPlanHostilesItem { pos: "pos", yaw: "yaw", crouch: "crouch", aim_at: "aimAt" });
json_struct!(game::scene_level::BreachPlanRetreat { mark: "mark", face: "face" });
json_struct!(game::scene_level::BreachPlanToss { mark: "mark", face: "face", land: "land" });
json_struct!(game::scene_level::BreachFlow { into: "into", centre: "centre", corner: "corner", pod_face: "podFace", corner_aim: "cornerAim", hold: "hold" });
json_struct!(game::scene_level::BreachPlanTargets0 { centre: "centre", corner: "corner" });
json_struct!(game::scene_level::BreachPlanTargets1 { centre: "centre", corner: "corner" });
json_struct!(game::scene_level::BreachPlanNext { door: "door", stack: "stack", stack_face: "stackFace", stack_side: "stackSide", stack_aim: "stackAim", via: "via" });
json_struct!(game::scene_level::BreachPlanBreachLines { breach1: "breach1", bang2: "bang2", going1: "going1", clear1: "clear1", clear2: "clear2", room_clear1: "roomClear1", next1: "next1", radio1: "radio1" });
json_words!(game::scene_level::BreachPlanBreach { Kick: "kick", Open: "open" });
json_struct!(game::scene_level::BreachPlan { inward: "inward", hostiles: "hostiles", retreat: "retreat", toss: "toss", flow: "flow", targets: "targets", next: "next", breach_lines: "breachLines", door: "door", room: "room", breach: "breach", bounds: "bounds", from: "from", stack: "stack", stack_face: "stackFace", stack_side: "stackSide", stack_aim: "stackAim", kick_from: "kickFrom", kick_face: "kickFace", fling: "fling", entry: "entry", pod_face: "podFace", entry_aim: "entryAim", search_path: "searchPath", search_face: "searchFace", search_aim_walk: "searchAimWalk", search_aim_there: "searchAimThere", under_aim: "underAim", up_aim: "upAim", form_up: "formUp", form_face: "formFace", lines: "lines" } drops ["id"]);
json_struct!(game::devices::CameraDef2 { name: "id", pos: "pos", yaw: "yaw", pitch: "pitch", sweep: "sweep", sweep_period: "sweepPeriod", fov_deg: "fovDeg", range: "range", group: "group", mains: "mains" } drops ["floor", "notes", "kind"]);
json_struct!(game::devices::KeypadDef2 { name: "id", pos: "pos", yaw: "yaw", door: "door", code: "code" } drops ["floor", "notes", "kind"]);
json_words!(game::devices::LaserDef2Alarm { Alarm: "alarm", Suspicion: "suspicion" });
json_struct!(game::devices::LaserDef2 { name: "id", from: "from", to: "to", group: "group", mains: "mains", alarm: "alarm" } drops ["floor", "notes", "kind"]);
/// a civilian: named by its `name`, else its id (levelLoad.ts)
impl FromJson for game::scene_level::CivilianDef {
    fn from_json(j: &Json, at: &str) -> Self {
        record(j, at);
        j.skip(&["id", "floor", "notes"]);
        let name: Option<String> = field(j, "name", at);
        game::scene_level::CivilianDef { name: name.unwrap_or_else(|| field(j, "id", at)), kind: field(j, "kind", at), tint: field(j, "tint", at), tint2: field(j, "tint2", at), points: field(j, "points", at), wait: field(j, "wait", at), tasks: field(j, "tasks", at), face: field(j, "face", at) }
    }
}
/// a device is one of three records, by its `kind` (levelLoad.ts deviceOf)
impl FromJson for game::scene_level::DeviceDef {
    fn from_json(j: &Json, at: &str) -> Self {
        use game::scene_level::DeviceDef as D;
        match j.get("kind").str() {
            Some("camera") => D::Camera(FromJson::from_json(j, at)),
            Some("keypad") => D::Keypad(FromJson::from_json(j, at)),
            Some("laser") => D::Laser(FromJson::from_json(j, at)),
            _ => panic!("level: {}.kind should be 'camera', 'keypad' or 'laser', and the file has {:?}", at, j.get("kind")),
        }
    }
}
