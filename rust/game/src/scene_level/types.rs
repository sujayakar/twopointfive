//! The types of `scene_level`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// which level's script this is, by the level file's name (scene/levelScripts.ts LEVEL_SCRIPTS); the body is levels/<name>.script.ts, its define pass called by a match
#[derive(Clone, Debug)]
pub enum LevelScriptFn {
    TutSky,
    TutTeam,
    TutQueue,
    TutSolo,
    TutRoad,
    TutKit,
    TutSquad,
    TutQuiet,
    TutPair,
}

/// (TypeScript: PropDef, src/scene/level.ts:149)
#[derive(Clone, Debug)]
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
impl PropDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PropDef {
            kind: crate::scene_level_data::PropKindName::Cardboard,
            name: String::new(),
            x: 0.0,
            z: 0.0,
            yaw: 0.0,
            parts: Vec::new(),
            half: [0.0; 2],
            height: 0.0,
            mass: 0.0,
        }
    }
}
impl Default for PropDef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PropPart, src/scene/level.ts:146)
#[derive(Clone, Debug)]
pub struct PropPart {
    pub off: V3,
    pub h: V3,
    pub yaw: f64,
    pub albedo: V3,
    pub marks: Option<Vec<crate::scene_markings::MarkingDef>>,
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
}
impl PropPart {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PropPart {
            off: [0.0; 3],
            h: [0.0; 3],
            yaw: 0.0,
            albedo: [0.0; 3],
            marks: None,
            stuff: None,
            does: None,
        }
    }
}
impl Default for PropPart { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: VentDef, src/scene/level.ts:142)
#[derive(Clone, Debug)]
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
impl VentDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        VentDef {
            name: String::new(),
            pos: [0.0; 3],
            dir: [0.0; 3],
            speed: 0.0,
            spread: 0.0,
            reach: 0.0,
            size: None,
            period: None,
            duty: None,
            phase: None,
            mains: None,
        }
    }
}
impl Default for VentDef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BreachPlan, src/scene/level.ts:78)
#[derive(Clone, Debug)]
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
    pub breach: Option<crate::scene_level::BreachPlanBreach>,
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
impl BreachPlan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreachPlan {
            inward: None,
            hostiles: Vec::new(),
            retreat: crate::scene_level::BreachPlanRetreat::zeroed(),
            toss: crate::scene_level::BreachPlanToss::zeroed(),
            flow: core::array::from_fn(|_| crate::scene_level::BreachFlow::zeroed()),
            targets: (crate::scene_level::BreachPlanTargets0::zeroed(), crate::scene_level::BreachPlanTargets1::zeroed(),),
            next: crate::scene_level::BreachPlanNext::zeroed(),
            breach_lines: crate::scene_level::BreachPlanBreachLines::zeroed(),
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
            lines: crate::scene_level::BreachPlanLines::zeroed(),
        }
    }
}
impl Default for BreachPlan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/level.ts:83 BreachPlan.hostiles, src/scene/level.ts:83)
#[derive(Clone, Copy, Debug)]
pub struct BreachPlanHostilesItem {
    pub pos: V3,
    pub yaw: f64,
    pub crouch: Option<bool>,
    pub aim_at: V3,
}
impl BreachPlanHostilesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreachPlanHostilesItem {
            pos: [0.0; 3],
            yaw: 0.0,
            crouch: None,
            aim_at: [0.0; 3],
        }
    }
}
impl Default for BreachPlanHostilesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/level.ts:85 BreachPlan.retreat, src/scene/level.ts:85)
#[derive(Clone, Copy, Debug)]
pub struct BreachPlanRetreat {
    pub mark: V3,
    pub face: f64,
}
impl BreachPlanRetreat {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreachPlanRetreat {
            mark: [0.0; 3],
            face: 0.0,
        }
    }
}
impl Default for BreachPlanRetreat { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/level.ts:88 BreachPlan.toss, src/scene/level.ts:88)
#[derive(Clone, Copy, Debug)]
pub struct BreachPlanToss {
    pub mark: V3,
    pub face: f64,
    pub land: V3,
}
impl BreachPlanToss {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreachPlanToss {
            mark: [0.0; 3],
            face: 0.0,
            land: [0.0; 3],
        }
    }
}
impl Default for BreachPlanToss { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BreachFlow, src/scene/level.ts:105)
#[derive(Clone, Debug)]
pub struct BreachFlow {
    pub into: Vec<V3>,
    pub centre: [V3; 2],
    pub corner: Vec<V3>,
    pub pod_face: f64,
    pub corner_aim: Vec<V3>,
    pub hold: V3,
}
impl BreachFlow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreachFlow {
            into: Vec::new(),
            centre: [[0.0; 3]; 2],
            corner: Vec::new(),
            pod_face: 0.0,
            corner_aim: Vec::new(),
            hold: [0.0; 3],
        }
    }
}
impl Default for BreachFlow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/level.ts:92 BreachPlan.targets, src/scene/level.ts:92)
#[derive(Clone, Debug)]
pub struct BreachPlanTargets0 {
    pub centre: Vec<f64>,
    pub corner: Vec<f64>,
}
impl BreachPlanTargets0 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreachPlanTargets0 {
            centre: Vec::new(),
            corner: Vec::new(),
        }
    }
}
impl Default for BreachPlanTargets0 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/level.ts:92 BreachPlan.targets, src/scene/level.ts:92)
#[derive(Clone, Debug)]
pub struct BreachPlanTargets1 {
    pub centre: Vec<f64>,
    pub corner: Vec<f64>,
}
impl BreachPlanTargets1 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreachPlanTargets1 {
            centre: Vec::new(),
            corner: Vec::new(),
        }
    }
}
impl Default for BreachPlanTargets1 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/level.ts:96 BreachPlan.next, src/scene/level.ts:96)
#[derive(Clone, Debug)]
pub struct BreachPlanNext {
    pub door: String,
    pub stack: [V3; 2],
    pub stack_face: [f64; 2],
    pub stack_side: f64,
    pub stack_aim: [V3; 2],
    pub via: Option<Vec<V3>>,
}
impl BreachPlanNext {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreachPlanNext {
            door: String::new(),
            stack: [[0.0; 3]; 2],
            stack_face: [0.0; 2],
            stack_side: 0.0,
            stack_aim: [[0.0; 3]; 2],
            via: None,
        }
    }
}
impl Default for BreachPlanNext { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/level.ts:99 BreachPlan.breachLines, src/scene/level.ts:99)
#[derive(Clone, Debug)]
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
impl BreachPlanBreachLines {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreachPlanBreachLines {
            breach1: String::new(),
            bang2: String::new(),
            going1: String::new(),
            clear1: String::new(),
            clear2: String::new(),
            room_clear1: String::new(),
            next1: String::new(),
            radio1: String::new(),
        }
    }
}
impl Default for BreachPlanBreachLines { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BreachPlanBreach {
    Kick,
    Open,
}
impl BreachPlanBreach {
    pub const fn as_str(self) -> &'static str {
        match self {
            BreachPlanBreach::Kick => "kick",
            BreachPlanBreach::Open => "open",
        }
    }
}

/// (TypeScript: inline at src/scene/level.ts:60 BreachPlan.bounds, src/scene/level.ts:60)
#[derive(Clone, Copy, Debug)]
pub struct BreachPlanBounds {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}
impl BreachPlanBounds {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreachPlanBounds {
            x0: 0.0,
            x1: 0.0,
            z0: 0.0,
            z1: 0.0,
        }
    }
}
impl Default for BreachPlanBounds { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/level.ts:68 BreachPlan.lines, src/scene/level.ts:68)
#[derive(Clone, Debug)]
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
impl BreachPlanLines {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreachPlanLines {
            stack1: String::new(),
            stack2: String::new(),
            go2: String::new(),
            in1: String::new(),
            side2: String::new(),
            side1: String::new(),
            push1: String::new(),
            under2: String::new(),
            nothing2: String::new(),
            corner1: String::new(),
            empty2: String::new(),
            radio1: String::new(),
        }
    }
}
impl Default for BreachPlanLines { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RoomClearPlan, src/scene/level.ts:56)
#[derive(Clone, Debug)]
pub struct RoomClearPlan {
    pub door: String,
    pub room: Option<String>,
    pub breach: Option<crate::scene_level::RoomClearPlanBreach>,
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
impl RoomClearPlan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
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
            lines: crate::scene_level::BreachPlanLines::zeroed(),
        }
    }
}
impl Default for RoomClearPlan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RoomClearPlanBreach {
    Kick,
    Open,
}
impl RoomClearPlanBreach {
    pub const fn as_str(self) -> &'static str {
        match self {
            RoomClearPlanBreach::Kick => "kick",
            RoomClearPlanBreach::Open => "open",
        }
    }
}

/// (TypeScript: Level, src/scene/level.ts:150)
#[derive(Clone, Debug)]
pub struct Level {
    pub boxes: Vec<crate::scene_boxes::Box>,
    pub props: Vec<crate::scene_level::PropDef>,
    pub lights: Vec<crate::scene_level::LightDef>,
    pub switchables: Vec<crate::scene_level::LevelSwitchablesItem>,
    pub player_spawn: V3,
    pub routes: Vec<RouteId>,
    pub chokepoints: Option<Vec<crate::scene_level::Chokepoint>>,
    pub clear_plans: Option<Vec<crate::scene_level::RoomClearPlan>>,
    pub doors: Vec<crate::scene_level::LevelDoorDef>,
    pub devices: Option<Vec<crate::scene_level::DeviceDef>>,
    pub vents: Option<Vec<crate::scene_level::VentDef>>,
    pub civilians: Option<Vec<crate::scene_level::CivilianDef>>,
    pub volumes: Option<Vec<crate::scene_level::VolumeDef>>,
    pub marks: Option<Vec<crate::scene_level::MarkDef>>,
    pub script: Option<crate::scene_level::LevelScriptFn>,
    pub scenarios: Option<crate::scene_scenarios::LevelScenarios>,
    pub surfaces: Option<crate::scene_level_data::SurfacesData>,
    pub breaker: crate::scene_level::LevelBreaker,
    pub beacons: Vec<crate::scene_level::LevelBeaconsItem>,
    pub sky: crate::scene_level::LevelSky,
    pub daylight: Option<crate::scene_level::DaylightDef>,
    pub breach: Option<crate::scene_level::BreachPlan>,
    pub mark_texel: Option<f64>,
    pub kept: Option<f64>,
    pub mission: crate::quiet::QuietMissionAuthoredMission,
}
impl Level {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Level {
            boxes: Vec::new(),
            props: Vec::new(),
            lights: Vec::new(),
            switchables: Vec::new(),
            player_spawn: [0.0; 3],
            routes: Vec::new(),
            chokepoints: None,
            clear_plans: None,
            doors: Vec::new(),
            devices: None,
            vents: None,
            civilians: None,
            volumes: None,
            marks: None,
            script: None,
            scenarios: None,
            surfaces: None,
            breaker: crate::scene_level::LevelBreaker::zeroed(),
            beacons: Vec::new(),
            sky: crate::scene_level::LevelSky::zeroed(),
            daylight: None,
            breach: None,
            mark_texel: None,
            kept: None,
            mission: crate::quiet::QuietMissionAuthoredMission::zeroed(),
        }
    }
}
impl Default for Level { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: LightDef, src/scene/level.ts:18)
#[derive(Clone, Debug)]
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
impl LightDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LightDef {
            kind: crate::scene_level::LightDefKind::Spot,
            pos: [0.0; 3],
            dir: [0.0; 3],
            color: [0.0; 3],
            intensity: 0.0,
            range: 0.0,
            inner_deg: 0.0,
            outer_deg: 0.0,
            name: String::new(),
            group: String::new(),
            mains: None,
            broad: None,
            fixture_box: None,
            enabled: false,
            volumetric: 0.0,
            radius: None,
        }
    }
}
impl Default for LightDef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LightDefKind {
    Spot,
    Dir,
    Point,
}
impl LightDefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            LightDefKind::Spot => "spot",
            LightDefKind::Dir => "dir",
            LightDefKind::Point => "point",
        }
    }
}

/// (TypeScript: inline at src/scene/level.ts:157 Level.switchables, src/scene/level.ts:157)
#[derive(Clone, Debug)]
pub struct LevelSwitchablesItem {
    pub name: String,
    pub boxes: Vec<f64>,
    pub group: String,
    pub on: bool,
    pub base_emissive: V3,
    pub flicker: Option<bool>,
    pub mains: Option<bool>,
}
impl LevelSwitchablesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LevelSwitchablesItem {
            name: String::new(),
            boxes: Vec::new(),
            group: String::new(),
            on: false,
            base_emissive: [0.0; 3],
            flicker: None,
            mains: None,
        }
    }
}
impl Default for LevelSwitchablesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Chokepoint, src/scene/level.ts:41)
#[derive(Clone, Debug)]
pub struct Chokepoint {
    pub name: String,
    pub pos: V3,
    pub yaw: f64,
}
impl Chokepoint {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Chokepoint {
            name: String::new(),
            pos: [0.0; 3],
            yaw: 0.0,
        }
    }
}
impl Default for Chokepoint { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: LevelDoorDef, src/scene/level.ts:125)
#[derive(Clone, Debug)]
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
impl LevelDoorDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LevelDoorDef {
            name: String::new(),
            hinge: [0.0; 2],
            closed_dir: 0.0,
            width: 0.0,
            angle: None,
            exterior: None,
            closer: None,
            min_angle: None,
            max_angle: None,
            locked: None,
            lock: None,
            key_side: None,
            steel: None,
            swing: None,
        }
    }
}
impl Default for LevelDoorDef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DeviceDef {
    Camera(crate::devices::CameraDef2),
    Keypad(crate::devices::KeypadDef2),
    Laser(crate::devices::LaserDef2),
}

/// (TypeScript: CivilianDef, src/scene/level.ts:50)
#[derive(Clone, Debug)]
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
impl CivilianDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CivilianDef {
            name: String::new(),
            kind: crate::scene_level::CivilianDefKind::Cleaner,
            tint: [0.0; 3],
            tint2: None,
            points: Vec::new(),
            wait: Vec::new(),
            tasks: None,
            face: None,
        }
    }
}
impl Default for CivilianDef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CivilianDefKind {
    Cleaner,
    Worker,
}
impl CivilianDefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            CivilianDefKind::Cleaner => "cleaner",
            CivilianDefKind::Worker => "worker",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CivilianTask {
    Bin,
    Stand,
    Wipe,
    Sit,
    Cooler,
    Printer,
    Shelf,
}
impl CivilianTask {
    pub const fn as_str(self) -> &'static str {
        match self {
            CivilianTask::Bin => "bin",
            CivilianTask::Stand => "stand",
            CivilianTask::Wipe => "wipe",
            CivilianTask::Sit => "sit",
            CivilianTask::Cooler => "cooler",
            CivilianTask::Printer => "printer",
            CivilianTask::Shelf => "shelf",
        }
    }
}

/// (TypeScript: VolumeDef, src/scene/level.ts:139)
#[derive(Clone, Debug)]
pub struct VolumeDef {
    pub name: String,
    pub kind: crate::scene_level::VolumeDefKind,
    pub c: V3,
    pub h: V3,
    pub yaw: Option<f64>,
}
impl VolumeDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        VolumeDef {
            name: String::new(),
            kind: crate::scene_level::VolumeDefKind::Trigger,
            c: [0.0; 3],
            h: [0.0; 3],
            yaw: None,
        }
    }
}
impl Default for VolumeDef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum VolumeDefKind {
    Trigger,
    Hide,
    Utility,
}
impl VolumeDefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            VolumeDefKind::Trigger => "trigger",
            VolumeDefKind::Hide => "hide",
            VolumeDefKind::Utility => "utility",
        }
    }
}

/// (TypeScript: MarkDef, src/scene/level.ts:141)
#[derive(Clone, Debug)]
pub struct MarkDef {
    pub name: String,
    pub pos: V3,
    pub yaw: Option<f64>,
}
impl MarkDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MarkDef {
            name: String::new(),
            pos: [0.0; 3],
            yaw: None,
        }
    }
}
impl Default for MarkDef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/level.ts:184 Level.breaker, src/scene/level.ts:184)
#[derive(Clone, Debug)]
pub struct LevelBreaker {
    pub pos: V3,
    pub boxes: Vec<f64>,
    pub fumble: Option<f64>,
}
impl LevelBreaker {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LevelBreaker {
            pos: [0.0; 3],
            boxes: Vec::new(),
            fumble: None,
        }
    }
}
impl Default for LevelBreaker { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/level.ts:185 Level.beacons, src/scene/level.ts:185)
#[derive(Clone, Copy, Debug)]
pub struct LevelBeaconsItem {
    pub pos: V3,
    pub box_: f64,
}
impl LevelBeaconsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LevelBeaconsItem {
            pos: [0.0; 3],
            box_: 0.0,
        }
    }
}
impl Default for LevelBeaconsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/level.ts:186 Level.sky, src/scene/level.ts:186)
#[derive(Clone, Copy, Debug)]
pub struct LevelSky {
    pub zenith: V3,
    pub horizon: V3,
}
impl LevelSky {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LevelSky {
            zenith: [0.0; 3],
            horizon: [0.0; 3],
        }
    }
}
impl Default for LevelSky { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DaylightDef, src/scene/level.ts:109)
#[derive(Clone, Debug)]
pub struct DaylightDef {
    pub sun: crate::scene_level::LightDef,
    pub sky: crate::scene_level::DaylightDefSky,
    pub groups_on: Vec<String>,
    pub groups_off: Vec<String>,
    pub lights_off: Vec<String>,
    pub haze: f64,
    pub exposure: f64,
}
impl DaylightDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DaylightDef {
            sun: crate::scene_level::LightDef::zeroed(),
            sky: crate::scene_level::DaylightDefSky::zeroed(),
            groups_on: Vec::new(),
            groups_off: Vec::new(),
            lights_off: Vec::new(),
            haze: 0.0,
            exposure: 0.0,
        }
    }
}
impl Default for DaylightDef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/level.ts:111 DaylightDef.sky, src/scene/level.ts:111)
#[derive(Clone, Copy, Debug)]
pub struct DaylightDefSky {
    pub zenith: V3,
    pub horizon: V3,
}
impl DaylightDefSky {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DaylightDefSky {
            zenith: [0.0; 3],
            horizon: [0.0; 3],
        }
    }
}
impl Default for DaylightDefSky { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PatrolRoute, src/scene/level.ts:39)
#[derive(Clone, Debug)]
pub struct PatrolRoute {
    pub name: String,
    pub points: Vec<V3>,
    pub wait: Vec<f64>,
    pub weapon: Option<crate::weapons::WeaponId>,
    pub keys: Option<Vec<String>>,
}
impl PatrolRoute {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PatrolRoute {
            name: String::new(),
            points: Vec::new(),
            wait: Vec::new(),
            weapon: None,
            keys: None,
        }
    }
}
impl Default for PatrolRoute { fn default() -> Self { Self::zeroed() } }
