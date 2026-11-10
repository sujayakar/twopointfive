//! The types of `scene_level`.
#![allow(unused_imports)]
use crate::{Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, sync::Arc, vec::Vec};

/// which level's script this is, by the level file's name; the body is levels/<name>.script.ts, its define pass called by a match
#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub enum LevelScriptFn {
    #[default] Unset,
    TutTeam,
    TutQueue,
    TutRoad,
    TutKit,
    TutQuiet,
    TutPair,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct PropDef {
    pub kind: crate::scene_level_data::PropKindName,
    pub name: String,
    pub x: f64,
    pub z: f64,
    pub yaw: f64,
    pub parts: Vec<crate::scene_level::PropPart>,
    pub half: [f64; 2],
    pub height: f64,
    pub mass: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct PropPart {
    pub off: V3,
    pub h: V3,
    pub yaw: f64,
    pub albedo: V3,
    pub marks: Option<Arc<[crate::scene_markings::MarkingDef]>>,
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct VentDef {
    pub name: String,
    pub pos: V3,
    pub dir: V3,
    pub speed: f64,
    pub spread: f64,
    pub reach: f64,
    pub size: Option<f64>,
    pub period: Option<f64>,
    pub duty: Option<f64>,
    pub phase: Option<f64>,
    pub mains: Option<bool>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default, PartialEq)]
pub enum HumKind {
    #[default]
    Hum,
    Servers,
    Buzz,
    Wind,
}
impl HumKind {
    pub fn as_str(self) -> &'static str {
        match self {
            HumKind::Hum => "hum",
            HumKind::Servers => "servers",
            HumKind::Buzz => "buzz",
            HumKind::Wind => "wind",
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct HumDef {
    pub name: String,
    pub kind: crate::scene_level::HumKind,
    pub pos: V3,
    pub level: f64,
    pub radius: f64,
    pub mains: Option<bool>,
}

#[derive(Clone, Debug, ToBytes)]
pub struct BreachPlan {
    pub inward: Option<[f64; 2]>,
    pub hostiles: Vec<crate::scene_level::BreachPlanHostilesItem>,
    pub retreat: crate::scene_level::BreachPlanRetreat,
    pub toss: crate::scene_level::BreachPlanToss,
    pub flow: [crate::scene_level::BreachFlow; 2],
    pub targets: (crate::scene_level::BreachPlanTargets0, crate::scene_level::BreachPlanTargets1),
    pub next: crate::scene_level::BreachPlanNext,
    pub breach_lines: crate::scene_level::BreachPlanBreachLines,
    pub door: String,
    pub room: Option<String>,
    pub breach: Option<crate::scene_level::PlanBreach>,
    pub bounds: Option<crate::scene_level::BreachPlanBounds>,
    pub from: Option<[V3; 2]>,
    pub stack: [V3; 2],
    pub stack_face: [f64; 2],
    pub stack_side: f64,
    pub stack_aim: [V3; 2],
    pub kick_from: V3,
    pub kick_face: f64,
    pub fling: f64,
    pub entry: [Vec<V3>; 2],
    pub pod_face: [f64; 2],
    pub entry_aim: [Vec<V3>; 2],
    pub search_path: Vec<V3>,
    pub search_face: f64,
    pub search_aim_walk: Vec<V3>,
    pub search_aim_there: Vec<V3>,
    pub under_aim: Vec<V3>,
    pub up_aim: V3,
    pub form_up: [Vec<V3>; 2],
    pub form_face: [f64; 2],
    pub lines: crate::scene_level::BreachPlanLines,
}
impl Default for BreachPlan {
    fn default() -> Self {
        BreachPlan {
            inward: None,
            hostiles: Vec::new(),
            retreat: crate::scene_level::BreachPlanRetreat::default(),
            toss: crate::scene_level::BreachPlanToss::default(),
            flow: core::array::from_fn(|_| crate::scene_level::BreachFlow::default()),
            targets: (crate::scene_level::BreachPlanTargets0::default(), crate::scene_level::BreachPlanTargets1::default(),),
            next: crate::scene_level::BreachPlanNext::default(),
            breach_lines: crate::scene_level::BreachPlanBreachLines::default(),
            door: String::new(),
            room: None,
            breach: None,
            bounds: None,
            from: None,
            stack: [[0.0; 3]; 2],
            stack_face: [0.0; 2],
            stack_side: 0.0,
            stack_aim: [[0.0; 3]; 2],
            kick_from: [0.0; 3],
            kick_face: 0.0,
            fling: 0.0,
            entry: core::array::from_fn(|_| Vec::new()),
            pod_face: [0.0; 2],
            entry_aim: core::array::from_fn(|_| Vec::new()),
            search_path: Vec::new(),
            search_face: 0.0,
            search_aim_walk: Vec::new(),
            search_aim_there: Vec::new(),
            under_aim: Vec::new(),
            up_aim: [0.0; 3],
            form_up: core::array::from_fn(|_| Vec::new()),
            form_face: [0.0; 2],
            lines: crate::scene_level::BreachPlanLines::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BreachPlanHostilesItem {
    pub pos: V3,
    pub yaw: f64,
    pub crouch: Option<bool>,
    pub aim_at: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BreachPlanRetreat {
    pub mark: V3,
    pub face: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BreachPlanToss {
    pub mark: V3,
    pub face: f64,
    pub land: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct BreachFlow {
    pub into: Vec<V3>,
    pub centre: [V3; 2],
    pub corner: Vec<V3>,
    pub pod_face: f64,
    pub corner_aim: Vec<V3>,
    pub hold: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct BreachPlanTargets0 {
    pub centre: Vec<f64>,
    pub corner: Vec<f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct BreachPlanTargets1 {
    pub centre: Vec<f64>,
    pub corner: Vec<f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct BreachPlanNext {
    pub door: String,
    pub stack: [V3; 2],
    pub stack_face: [f64; 2],
    pub stack_side: f64,
    pub stack_aim: [V3; 2],
    pub via: Option<Vec<V3>>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct BreachPlanBreachLines {
    pub breach1: String,
    pub bang2: String,
    pub going1: String,
    pub clear1: String,
    pub clear2: String,
    pub room_clear1: String,
    pub next1: String,
    pub radio1: String,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BreachPlanBounds {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct BreachPlanLines {
    pub stack1: String,
    pub stack2: String,
    pub go2: String,
    pub in1: String,
    pub side2: String,
    pub side1: String,
    pub push1: String,
    pub under2: String,
    pub nothing2: String,
    pub corner1: String,
    pub empty2: String,
    pub radio1: String,
}

#[derive(Clone, Debug, ToBytes)]
pub struct RoomClearPlan {
    pub door: String,
    pub room: Option<String>,
    pub breach: Option<crate::scene_level::PlanBreach>,
    pub bounds: Option<crate::scene_level::BreachPlanBounds>,
    pub from: Option<[V3; 2]>,
    pub stack: [V3; 2],
    pub stack_face: [f64; 2],
    pub stack_side: f64,
    pub stack_aim: [V3; 2],
    pub kick_from: V3,
    pub kick_face: f64,
    pub fling: f64,
    pub entry: [Vec<V3>; 2],
    pub pod_face: [f64; 2],
    pub entry_aim: [Vec<V3>; 2],
    pub search_path: Vec<V3>,
    pub search_face: f64,
    pub search_aim_walk: Vec<V3>,
    pub search_aim_there: Vec<V3>,
    pub under_aim: Vec<V3>,
    pub up_aim: V3,
    pub form_up: [Vec<V3>; 2],
    pub form_face: [f64; 2],
    pub lines: crate::scene_level::BreachPlanLines,
}
impl Default for RoomClearPlan {
    fn default() -> Self {
        RoomClearPlan {
            door: String::new(),
            room: None,
            breach: None,
            bounds: None,
            from: None,
            stack: [[0.0; 3]; 2],
            stack_face: [0.0; 2],
            stack_side: 0.0,
            stack_aim: [[0.0; 3]; 2],
            kick_from: [0.0; 3],
            kick_face: 0.0,
            fling: 0.0,
            entry: core::array::from_fn(|_| Vec::new()),
            pod_face: [0.0; 2],
            entry_aim: core::array::from_fn(|_| Vec::new()),
            search_path: Vec::new(),
            search_face: 0.0,
            search_aim_walk: Vec::new(),
            search_aim_there: Vec::new(),
            under_aim: Vec::new(),
            up_aim: [0.0; 3],
            form_up: core::array::from_fn(|_| Vec::new()),
            form_face: [0.0; 2],
            lines: crate::scene_level::BreachPlanLines::default(),
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Level {
    pub props: Vec<Arc<crate::scene_level::PropDef>>,
    pub lights: Vec<crate::scene_level::LightDef>,
    pub switchables: Vec<crate::scene_level::LevelSwitchablesItem>,
    pub player_spawn: V3,
    pub routes: Vec<RouteId>,
    pub chokepoints: Option<Vec<crate::scene_level::Chokepoint>>,
    pub clear_plans: Option<Vec<crate::scene_level::RoomClearPlan>>,
    pub doors: Vec<crate::scene_level::LevelDoorDef>,
    pub devices: Option<Vec<crate::scene_level::DeviceDef>>,
    pub vents: Option<Vec<crate::scene_level::VentDef>>,
    pub hums: Option<Vec<crate::scene_level::HumDef>>,
    pub civilians: Option<Vec<crate::scene_level::CivilianDef>>,
    pub volumes: Option<Vec<crate::scene_level::VolumeDef>>,
    pub marks: Option<Vec<crate::scene_level::MarkDef>>,
    pub script: Option<crate::scene_level::LevelScriptFn>,
    pub scenarios: Option<crate::scene_scenarios::LevelScenarios>,
    pub surfaces: Option<crate::scene_level_data::SurfacesData>,
    pub breaker: crate::scene_level::LevelBreaker,
    pub beacons: Vec<crate::scene_level::LevelBeaconsItem>,
    pub sky: crate::scene_level::LevelSky,
    pub breach: Option<crate::scene_level::BreachPlan>,
    pub mark_texel: Option<f64>,
    pub kept: Option<f64>,
    pub errand: crate::mission::Errand,
}
/// a BreachPlan is a RoomClearPlan with more on it: the part of it doorPlan's callers read
fn plan_of_breach(b: &crate::scene_level::BreachPlan) -> crate::scene_level::RoomClearPlan {
    crate::scene_level::RoomClearPlan {
        door: b.door.clone(),
        room: b.room.clone(),
        breach: b.breach,
        bounds: b.bounds,
        from: b.from,
        stack: b.stack,
        stack_face: b.stack_face,
        stack_side: b.stack_side,
        stack_aim: b.stack_aim,
        kick_from: b.kick_from,
        kick_face: b.kick_face,
        fling: b.fling,
        entry: b.entry.clone(),
        pod_face: b.pod_face,
        entry_aim: b.entry_aim.clone(),
        search_path: b.search_path.clone(),
        search_face: b.search_face,
        search_aim_walk: b.search_aim_walk.clone(),
        search_aim_there: b.search_aim_there.clone(),
        under_aim: b.under_aim.clone(),
        up_aim: b.up_aim,
        form_up: b.form_up.clone(),
        form_face: b.form_face,
        lines: b.lines.clone(),
    }
}

/// WHAT A LEVEL'S FILE SAYS OF ITS FLOOR, whichever mission is played on it.
impl Level {
    fn floor(&self) -> Option<&crate::scene_scenarios::StructureData> { self.scenarios.as_ref()?.room_clear.as_ref() }
    fn own(&self) -> Option<&crate::scene_scenarios::QuietData> { self.scenarios.as_ref()?.quiet.as_ref() }
    /// its rooms, each with its name, its bounds and its doors (empty: it names none)
    pub fn rooms(&self) -> &[crate::scene_scenarios::StructureRoom] { self.floor().map_or(&[], |f| &f.rooms) }
    pub fn room_at(&self, p: V3) -> Option<&crate::scene_scenarios::StructureRoom> {
        self.rooms().iter().find(|r| { let b = &r.bounds; p[0] > b.x0 && p[0] < b.x1 && p[2] > b.z0 && p[2] < b.z1 })
    }
    /// its defenders may dig in
    pub fn digs_in(&self) -> bool { self.floor().map_or(true, |f| f.dig_in != Some(false)) }
    /// the room its captives are held in, as its file says it
    pub fn held_room(&self) -> Option<&crate::scene_scenarios::RescueData> { self.scenarios.as_ref()?.rescue.as_ref() }
    /// it says its own places: where its staff's needs are answered, and what there is to take
    pub fn own_places(&self) -> bool { self.own().is_some() }
    pub fn needs(&self) -> Option<&crate::scene_scenarios::QuietDataNeeds> { self.own()?.needs.as_ref() }
    /// where the thing to take may lie
    pub fn spots(&self) -> &[crate::scene_scenarios::QuietSpot] { self.own().map_or(&[], |q| &q.spots) }
    /// its shift checks in by radio
    pub fn radio_check(&self) -> bool { self.own().map_or(true, |q| q.radio_check != Some(false)) }
    /// its own stack-and-enter plan for a door
    pub fn door_plan(&self, door: &str) -> Option<crate::scene_level::RoomClearPlan> {
        if let Some(p) = self.clear_plans.as_ref().and_then(|v| v.iter().find(|p| p.door == door)) { return Some(p.clone()); }
        self.breach.as_ref().filter(|b| b.door == door).map(plan_of_breach)
    }
    /// it is a lesson's
    pub fn lesson(&self) -> Option<&crate::scene_scenarios::LessonData> { self.scenarios.as_ref()?.lesson.as_deref() }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LightDef {
    pub kind: crate::scene_level::LightDefKind,
    pub pos: V3,
    pub dir: V3,
    pub color: V3,
    pub intensity: f64,
    pub range: f64,
    pub inner_deg: f64,
    pub outer_deg: f64,
    pub name: String,
    pub group: String,
    pub mains: Option<bool>,
    pub broad: Option<bool>,
    pub fixture_box: Option<f64>,
    pub enabled: bool,
    pub volumetric: f64,
    pub radius: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum LightDefKind {
    #[default] Spot,
    Dir,
    Point,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LevelSwitchablesItem {
    pub name: String,
    pub boxes: Vec<f64>,
    pub group: String,
    pub on: bool,
    pub base_emissive: V3,
    pub flicker: Option<bool>,
    pub mains: Option<bool>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Chokepoint {
    pub name: String,
    pub pos: V3,
    pub yaw: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LevelDoorDef {
    pub name: String,
    pub hinge: [f64; 2],
    pub closed_dir: f64,
    pub width: f64,
    pub angle: Option<f64>,
    pub exterior: Option<bool>,
    pub closer: Option<bool>,
    pub min_angle: Option<f64>,
    pub max_angle: Option<f64>,
    pub locked: Option<bool>,
    pub lock: Option<String>,
    pub key_side: Option<f64>,
    pub steel: Option<bool>,
    pub swing: Option<f64>,
}

#[derive(Clone, Debug, ToBytes)]
pub enum DeviceDef {
    Camera(crate::devices::CameraDef2),
    Keypad(crate::devices::KeypadDef2),
    Laser(crate::devices::LaserDef2),
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CivilianDef {
    pub name: String,
    pub kind: crate::scene_level::CivilianDefKind,
    pub tint: V3,
    pub tint2: Option<V3>,
    pub points: Vec<V3>,
    pub wait: Vec<f64>,
    pub tasks: Option<Vec<crate::scene_level::CivilianTask>>,
    pub face: Option<Vec<f64>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words)]
pub enum CivilianTask {
    Bin,
    Stand,
    Wipe,
    Sit,
    Cooler,
    Printer,
    Shelf,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct VolumeDef {
    pub name: String,
    pub c: V3,
    pub h: V3,
    pub yaw: Option<f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct MarkDef {
    pub name: String,
    pub pos: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LevelBreaker {
    pub pos: V3,
    pub boxes: Vec<f64>,
    pub fumble: Option<f64>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct LevelBeaconsItem {
    pub pos: V3,
    pub box_: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct LevelSky {
    pub zenith: V3,
    pub horizon: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct PatrolRoute {
    pub name: String,
    pub points: Vec<V3>,
    pub wait: Vec<f64>,
    pub weapon: Option<crate::weapons::WeaponId>,
    pub keys: Option<Vec<String>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum PlanBreach {
    Kick,
    Open,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum CivilianDefKind {
    #[default] Cleaner,
    Worker,
}
