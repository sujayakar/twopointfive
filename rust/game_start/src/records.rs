//! THE RECORDS OF A LEVEL FILE THAT THE GAME KEEPS AS ITS OWN TYPES, each type with the member of the file each of its fields is (`read.rs` reads them). A type that gains a field does not compile
//! until its line here names the member, and a member of the file no line names is found by the file's own tree (`json.rs unread`).
#![allow(clippy::all)]
use crate::read::{field, record, At, FromJson};
use crate::json::Json;
use alloc::{format, string::String};
use crate::{json_struct, json_words};

json_struct!(game::scene_scenarios::TeamStart { sam: "sam", sam_yaw: "samYaw", ops: "ops", ops_yaw: "opsYaw" });
json_struct!(game::scene_scenarios::NightBreachData { team: "team", room: "room" } drops { "lights": Option<String> });
json_struct!(game::scene_scenarios::Rect { x0: "x0", x1: "x1", z0: "z0", z1: "z1" });
json_struct!(game::scene_scenarios::StructureDoor { id: "id", from: "from" });
json_struct!(game::scene_scenarios::StructureMark { pos: "pos", watch: "watch", aim_at: "aimAt" });
json_struct!(game::scene_scenarios::StructureRoom { id: "id", name: "name", bounds: "bounds", doors: "doors", garrison: "garrison", marks: "marks" });
json_words!(game::scene_scenarios::SquadDataTraining { Trained: "trained", Green: "green", Mixed: "mixed" });
json_struct!(game::scene_scenarios::SquadArrive { door: "door", spawn: "spawn", men: "men" });
json_struct!(game::scene_scenarios::SquadData { id: "id", name: "name", rooms: "rooms", rally: "rally", barricade: "barricade", may_leave: "mayLeave", training: "training", arrive: "arrive" });
json_struct!(game::scene_scenarios::StructureData { name: "name", team: "team", rooms: "rooms", starts: "starts", max_men: "maxMen", way_out: "wayOut", dig_in: "digIn", wall_charges: "wallCharges", squads: "squads" });
json_struct!(game::scene_scenarios::RescueDataRoom { name: "name", where_: "where", door: "door", bounds: "bounds", door_aim: "doorAim", outside: "outside", hold: "hold" } drops { "lights": Option<String> });
json_struct!(game::scene_scenarios::RescueDataSideFrom { door: "door", bounds: "bounds" });
json_struct!(game::scene_scenarios::RescueDataSide { door: "door", hold: "hold", from: "from", pair: "pair", stack: "stack", standoff: "standoff" });
json_struct!(game::scene_scenarios::RescueDataHostagesItem { pos: "pos", face: "face" });
json_struct!(game::scene_scenarios::ScenarioMark { pos: "pos", aim_at: "aimAt" });
json_struct!(game::scene_scenarios::RescueDataDoorMan { inside: "inside", outside: "outside" });
json_struct!(game::scene_scenarios::RescueDataCast { roamers: "roamers" });
json_struct!(game::scene_scenarios::RescueFlowDataRegionsItem { id: "id", rect: "rect" } nulls { door: "door" });
json_words!(game::scene_scenarios::RouteOpeningKind { Door: "door", Glass: "glass", Doorway: "doorway" });
json_words!(game::scene_scenarios::RouteOpeningSide { Left: "left", Right: "right" });
json_struct!(game::scene_scenarios::RouteOpening { id: "id", name: "name", short: "short", region: "region", kind: "kind", at: "at", mark: "mark", watch: "watch", half: "half", north: "north", call: "call", side: "side", look: "look" } nulls { door: "door" });
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
json_struct!(game::scene_scenarios::RailsData { allow: "allow", respawn: "respawn", fog: "fog", waits: "waits", lasers: "lasers", orders: "orders", flat_ring: "flatRing", rows: "rows" });
json_struct!(game::scene_scenarios::LessonData { title: "title", steps: "steps", rails: "rails" });
json_struct!(game::scene_scenarios::ParData { gold: "gold", silver: "silver", bronze: "bronze", loss_secs: "lossSecs" });
json_words!(game::kit::KitWeapon { Pistol: "pistol", Rifle: "rifle", Shotgun: "shotgun" });
json_words!(game::scene_scenarios::IssueCardLoaded { Ring: "ring", He40: "he40" });
json_struct!(game::scene_scenarios::IssueCard { weapon: "weapon", suppressed: "suppressed", light: "light", nvg: "nvg", bangs: "bangs", charges: "charges", smoke: "smoke", frags: "frags", walls: "walls", shells: "shells", rings: "rings", loaded: "loaded" });
json_struct!(game::scene_scenarios::IssueData { members: "members" });
json_words!(game::scene_markings::MarkFace { X: "+x", X2: "-x", Y: "+y", Y2: "-y", Z: "+z", Z2: "-z" });
json_words!(game::scene_markings::MarkRes { Base: "base", Fine: "fine", Coarse: "coarse" });
json_struct!(game::scene_markings::MarkPal { albedo: "albedo", mul: "mul", rough: "rough", emit: "emit" });
json_struct!(game::scene_markings::MarkingDef { face: "face", res: "res", at: "at", size: "size", pal: "pal", cells: "cells" });
json_struct!(game::scene_shapes::ShapeFormArc { axis: "axis", rise: "rise" });
json_struct!(game::scene_scenarios::ResponseWay { name: "name", door: "door", muster: "muster", entry: "entry", entry_face: "entryFace", coming: "coming", clear: "clear", stairs: "stairs" });
json_struct!(game::scene_scenarios::ResponsePost { name: "name", pos: "pos", yaw: "yaw", sweep: "sweep" });
json_struct!(game::scene_scenarios::ResponseDataSweepItem { name: "name", pos: "pos" });
json_struct!(game::scene_scenarios::ResponseDataRaid { room: "room" });
json_struct!(game::scene_scenarios::ResponseData { site: "site", at: "at", team: "team", tag: "tag", ways: "ways", men: "men", eta: "eta", posts: "posts", outside: "outside", sweep: "sweep", raid: "raid" } drops { "why": String });
json_struct!(game::scene_scenarios::LevelScenariosDoorNamesItem { door: "door", name: "name" });
json_struct!(game::scene_ways::WayHint { from: "from", to: "to", weight: "weight" });
json_struct!(game::scene_scenarios::LevelScenarios { night_breach: "nightBreach", room_clear: "roomClear", rescue: "rescue", places: "places", quiet: "quiet", lesson: "lesson", par: "par", issue: "issue", response: "response", door_names: "doorNames", walked: "walked" });

json_struct!(game::scene_level_data::Rect { x0: "x0", x1: "x1", z0: "z0", z1: "z1" });
json_words!(game::floor::FloorMat { Carpet: "carpet", Lino: "lino", Stone: "stone", Tile: "tile", Asphalt: "asphalt", Grass: "grass" });
json_struct!(game::scene_level_data::SurfacesDataZonesItem { mat: "mat", x0: "x0", z0: "z0", x1: "x1", z1: "z1" } drops { "id": String, "notes": Option<String> });
json_struct!(game::scene_level_data::SurfacesData { building: "building", grass_edge: "grassEdge", inside: "inside", outside: "outside", verge: "verge", zones: "zones" });

/// what a test says that is true or left out
fn said(j: &Json, key: &str, at: &At) -> Option<bool> { field::<Option<bool>>(j, key, at).map(|b| b.unwrap_or(false)) }

/// A lesson's test (`{ is: 'in', area: … }`): the word `is` says which test it is.
impl FromJson for game::scene_scenarios::LessonTestKind {
    fn from_json(j: &Json, at: &At) -> Option<Self> {
        use game::scene_scenarios::LessonTestKind as K;
        record(j, at)?;
        let Some(is) = field::<String>(j, "is", at) else { j.skip_all(); return None };
        Some(match is.as_str() {
            "in" => K::In { area: field(j, "area", at)? },
            "near" => { let (at_, r) = (field(j, "at", at), field(j, "r", at)); K::Near { at: at_?, r: r? } }
            "flag" => { let (flag, value) = (field(j, "flag", at), field(j, "value", at)); K::Flag { flag: flag?, value: value? } }
            "slot" => { let (slot, drawn) = (field(j, "slot", at), field(j, "drawn", at)); K::Slot { slot: slot?, drawn: drawn? } }
            "pace" => { let (below, above) = (field(j, "below", at), field(j, "above", at)); K::Pace { below: below?, above: above? } }
            "meter" => { let (which, below, above) = (field(j, "which", at), field(j, "below", at), field(j, "above", at)); K::Meter { which: which?, below: below?, above: above? } }
            "stage" => K::Stage { stage: field(j, "stage", at)? },
            "door" => { let (door, state) = (field(j, "door", at), field(j, "state", at)); K::Door { door: door?, state: state? } }
            "target" => { let (name, on) = (field(j, "name", at), field(j, "on", at)); K::Target { name: name?, on: on? } }
            "men" => K::Men { down: field(j, "down", at)? },
            "did" => { let (what, on, n) = (field(j, "what", at), field(j, "on", at), field(j, "n", at)); K::Did { what: what?, on: on?, n: n? } }
            "downed" => { let (how, n) = (field(j, "how", at), field(j, "n", at)); K::Downed { how: how?, n: n? } }
            "body" => K::Body { area: field(j, "area", at)? },
            "calm" => K::Calm { route: field(j, "route", at)? },
            "loaded" => K::Loaded,
            "reload" => K::Reload,
            "launcher" => { let (on, fired) = (field(j, "on", at), said(j, "fired", at)); K::Launcher { on: on?, fired: fired? } }
            "burst" => K::Burst { n: field(j, "n", at)? },
            "threw" => { let (kind, quick, area) = (field(j, "kind", at), said(j, "quick", at), field(j, "area", at)); K::Threw { kind: kind?, quick: quick?, area: area? } }
            "orderTo" => K::OrderTo { who: field(j, "who", at)? },
            "stackOn" => { let (door, who, set) = (field(j, "door", at), field(j, "who", at), said(j, "set", at)); K::StackOn { door: door?, who: who?, set: set? } }
            "pairsOn" => K::PairsOn { doors: field(j, "doors", at)? },
            "word" => K::Word { doors: field(j, "doors", at)? },
            "taken" => K::Taken { room: field(j, "room", at)? },
            "following" => K::Following,
            "mate" => { let (area, at_, r, by, away) = (field(j, "area", at), field(j, "at", at), field(j, "r", at), field(j, "by", at), field(j, "away", at)); K::Mate { area: area?, at: at_?, r: r?, by: by?, away: away? } }
            "pair" => { let (what, n) = (field(j, "what", at), field(j, "n", at)); K::Pair { what: what?, n: n? } }
            "cue" => K::Cue { name: field(j, "name", at)? },
            "view" => { let (turned, zoomed) = (field(j, "turned", at), field(j, "zoomed", at)); K::View { turned: turned?, zoomed: zoomed? } }
            "squad" => { let (area, n) = (field(j, "area", at), field(j, "n", at)); K::Squad { area: area?, n: n? } }
            "stacks" => { let (doors, n, set) = (field(j, "doors", at), field(j, "n", at), said(j, "set", at)); K::Stacks { doors: doors?, n: n?, set: set? } }
            "watching" => { let (at_, r, n) = (field(j, "at", at), field(j, "r", at), field(j, "n", at)); K::Watching { at: at_?, r: r?, n: n? } }
            "all" => K::All { of: field(j, "of", at)? },
            "any" => K::Any { of: field(j, "of", at)? },
            w => { j.skip_all(); at.fault(format!("no lesson's test is called '{w}'")); return None; }
        })
    }
}

// ---- the level's own records: a device, a vent and a light say `id` where the game's type says `name`; a plan is the file's record without its id (file.rs keeps it)
json_words!(game::scene_level::CivilianDefKind { Cleaner: "cleaner", Worker: "worker" });
json_words!(game::scene_level::CivilianTask { Bin: "bin", Stand: "stand", Wipe: "wipe", Sit: "sit", Cooler: "cooler", Printer: "printer", Shelf: "shelf" });
json_struct!(game::scene_level::VentDef { name: "id", pos: "pos", dir: "dir", speed: "speed", spread: "spread", reach: "reach", size: "size", period: "period", duty: "duty", phase: "phase", mains: "mains" } drops { "floor": Option<String>, "notes": Option<String> });
json_words!(game::scene_level::RoomClearPlanBreach { Kick: "kick", Open: "open" });
json_struct!(game::scene_level::BreachPlanBounds { x0: "x0", x1: "x1", z0: "z0", z1: "z1" });
json_struct!(game::scene_level::BreachPlanLines { stack1: "stack1", stack2: "stack2", go2: "go2", in1: "in1", side2: "side2", side1: "side1", push1: "push1", under2: "under2", nothing2: "nothing2", corner1: "corner1", empty2: "empty2", radio1: "radio1" });
json_struct!(game::scene_level::RoomClearPlan { door: "door", room: "room", breach: "breach", bounds: "bounds", from: "from", stack: "stack", stack_face: "stackFace", stack_side: "stackSide", stack_aim: "stackAim", kick_from: "kickFrom", kick_face: "kickFace", fling: "fling", entry: "entry", pod_face: "podFace", entry_aim: "entryAim", search_path: "searchPath", search_face: "searchFace", search_aim_walk: "searchAimWalk", search_aim_there: "searchAimThere", under_aim: "underAim", up_aim: "upAim", form_up: "formUp", form_face: "formFace", lines: "lines" });
json_struct!(game::scene_level::BreachPlanHostilesItem { pos: "pos", yaw: "yaw", crouch: "crouch", aim_at: "aimAt" });
json_struct!(game::scene_level::BreachPlanRetreat { mark: "mark", face: "face" });
json_struct!(game::scene_level::BreachPlanToss { mark: "mark", face: "face", land: "land" });
json_struct!(game::scene_level::BreachFlow { into: "into", centre: "centre", corner: "corner", pod_face: "podFace", corner_aim: "cornerAim", hold: "hold" });
json_struct!(game::scene_level::BreachPlanTargets0 { centre: "centre", corner: "corner" });
json_struct!(game::scene_level::BreachPlanTargets1 { centre: "centre", corner: "corner" });
json_struct!(game::scene_level::BreachPlanNext { door: "door", stack: "stack", stack_face: "stackFace", stack_side: "stackSide", stack_aim: "stackAim", via: "via" });
json_struct!(game::scene_level::BreachPlanBreachLines { breach1: "breach1", bang2: "bang2", going1: "going1", clear1: "clear1", clear2: "clear2", room_clear1: "roomClear1", next1: "next1", radio1: "radio1" });
json_words!(game::scene_level::BreachPlanBreach { Kick: "kick", Open: "open" });
json_struct!(game::scene_level::BreachPlan { inward: "inward", hostiles: "hostiles", retreat: "retreat", toss: "toss", flow: "flow", targets: "targets", next: "next", breach_lines: "breachLines", door: "door", room: "room", breach: "breach", bounds: "bounds", from: "from", stack: "stack", stack_face: "stackFace", stack_side: "stackSide", stack_aim: "stackAim", kick_from: "kickFrom", kick_face: "kickFace", fling: "fling", entry: "entry", pod_face: "podFace", entry_aim: "entryAim", search_path: "searchPath", search_face: "searchFace", search_aim_walk: "searchAimWalk", search_aim_there: "searchAimThere", under_aim: "underAim", up_aim: "upAim", form_up: "formUp", form_face: "formFace", lines: "lines" });
json_struct!(game::devices::CameraDef2 { name: "id", pos: "pos", yaw: "yaw", pitch: "pitch", sweep: "sweep", sweep_period: "sweepPeriod", fov_deg: "fovDeg", range: "range", group: "group", mains: "mains" } drops { "floor": Option<String>, "notes": Option<String> });
json_struct!(game::devices::KeypadDef2 { name: "id", pos: "pos", yaw: "yaw", door: "door", code: "code" } drops { "floor": Option<String>, "notes": Option<String> });
json_words!(game::devices::LaserDef2Alarm { Alarm: "alarm", Suspicion: "suspicion" });
json_struct!(game::devices::LaserDef2 { name: "id", from: "from", to: "to", group: "group", mains: "mains", alarm: "alarm" } drops { "floor": Option<String>, "notes": Option<String> });
/// a civilian: named by its `name`, else its id
impl FromJson for game::scene_level::CivilianDef {
    fn from_json(j: &Json, at: &At) -> Option<Self> {
        record(j, at)?;
        let (floor, notes): (Option<Option<String>>, Option<Option<String>>) = (field(j, "floor", at), field(j, "notes", at));   // (left behind)
        let (id, name): (Option<String>, Option<Option<String>>) = (field(j, "id", at), field(j, "name", at));
        let (kind, tint, tint2, points) = (field(j, "kind", at), field(j, "tint", at), field(j, "tint2", at), field(j, "points", at));
        let (wait, tasks, face) = (field(j, "wait", at), field(j, "tasks", at), field(j, "face", at));
        (floor?, notes?);
        Some(game::scene_level::CivilianDef { name: name?.unwrap_or(id?), kind: kind?, tint: tint?, tint2: tint2?, points: points?, wait: wait?, tasks: tasks?, face: face? })
    }
}
/// a device is one of three records, by its `kind`
impl FromJson for game::scene_level::DeviceDef {
    fn from_json(j: &Json, at: &At) -> Option<Self> {
        use game::scene_level::DeviceDef as D;
        record(j, at)?;
        match j.get("kind").str() {
            Some("camera") => FromJson::from_json(j, at).map(D::Camera),
            Some("keypad") => FromJson::from_json(j, at).map(D::Keypad),
            Some("laser") => FromJson::from_json(j, at).map(D::Laser),
            _ => { j.skip_all(); at.key("kind").wants("one of camera, keypad, laser", j.get("kind")) }
        }
    }
}
