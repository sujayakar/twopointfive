//! The types of `director`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// which events a c.on takes: every distinct filter closure of levels/*.script.ts, with what it captured (director.ts EventFilter)
#[derive(Clone, Debug)]
pub enum EventFilter {
    InBox { x0: f64, x1: f64, z0: f64, z1: f64 },
    XAbove { x: f64 },
    XBelow { x: f64 },
    XZAbove { x: f64, z: f64 },
    OrderSignal,
    Heard { by: crate::director::UnitRef, run_level: f64, run_within: f64, walk_level: f64, walk_within: f64 },
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum Json {
    Text(String),
    Num(f64),
    Bool(bool),
    Array(Vec<Option<crate::director::Json>>),
    Arm4(JsMap<String, Option<crate::director::Json>>),
}

/// (TypeScript: MarkRef, src/game/director.ts:77)
#[derive(Clone, Debug)]
pub struct MarkRef {
    pub kind: crate::director::MarkRefKind,
    pub id: String,
}
impl MarkRef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MarkRef {
            kind: crate::director::MarkRefKind::Mark,
            id: String::new(),
        }
    }
}
impl Default for MarkRef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MarkRefKind {
    Mark,
}
impl MarkRefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            MarkRefKind::Mark => "mark",
        }
    }
}

/// (TypeScript: DoorRef, src/game/director.ts:78)
#[derive(Clone, Debug)]
pub struct DoorRef {
    pub kind: crate::director::DoorRefKind,
    pub id: String,
}
impl DoorRef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorRef {
            kind: crate::director::DoorRefKind::Door,
            id: String::new(),
        }
    }
}
impl Default for DoorRef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorRefKind {
    Door,
}
impl DoorRefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorRefKind::Door => "door",
        }
    }
}

/// (TypeScript: VolumeRef, src/game/director.ts:79)
#[derive(Clone, Debug)]
pub struct VolumeRef {
    pub kind: crate::director::VolumeRefKind,
    pub id: String,
}
impl VolumeRef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        VolumeRef {
            kind: crate::director::VolumeRefKind::Volume,
            id: String::new(),
        }
    }
}
impl Default for VolumeRef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum VolumeRefKind {
    Volume,
}
impl VolumeRefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            VolumeRefKind::Volume => "volume",
        }
    }
}

/// (TypeScript: RouteRef, src/game/director.ts:80)
#[derive(Clone, Debug)]
pub struct RouteRef {
    pub kind: crate::director::RouteRefKind,
    pub id: String,
}
impl RouteRef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RouteRef {
            kind: crate::director::RouteRefKind::Route,
            id: String::new(),
        }
    }
}
impl Default for RouteRef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RouteRefKind {
    Route,
}
impl RouteRefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            RouteRefKind::Route => "route",
        }
    }
}

/// (TypeScript: GroupRef, src/game/director.ts:81)
#[derive(Clone, Debug)]
pub struct GroupRef {
    pub kind: crate::director::GroupRefKind,
    pub id: String,
}
impl GroupRef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GroupRef {
            kind: crate::director::GroupRefKind::Group,
            id: String::new(),
        }
    }
}
impl Default for GroupRef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GroupRefKind {
    Group,
}
impl GroupRefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            GroupRefKind::Group => "group",
        }
    }
}

/// (TypeScript: VariantRef, src/game/director.ts:82)
#[derive(Clone, Copy, Debug)]
pub struct VariantRef {
    pub kind: crate::director::VariantRefKind,
    pub id: crate::director::VariantRefId,
}
impl VariantRef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        VariantRef {
            kind: crate::director::VariantRefKind::Variant,
            id: crate::director::VariantRefId::Daylight,
        }
    }
}
impl Default for VariantRef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum VariantRefKind {
    Variant,
}
impl VariantRefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            VariantRefKind::Variant => "variant",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum VariantRefId {
    Daylight,
}
impl VariantRefId {
    pub const fn as_str(self) -> &'static str {
        match self {
            VariantRefId::Daylight => "daylight",
        }
    }
}

/// (TypeScript: LoadoutRef, src/game/director.ts:83)
#[derive(Clone, Debug)]
pub struct LoadoutRef {
    pub kind: crate::director::LoadoutRefKind,
    pub id: String,
}
impl LoadoutRef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LoadoutRef {
            kind: crate::director::LoadoutRefKind::Loadout,
            id: String::new(),
        }
    }
}
impl Default for LoadoutRef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LoadoutRefKind {
    Loadout,
}
impl LoadoutRefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            LoadoutRefKind::Loadout => "loadout",
        }
    }
}

/// (TypeScript: DeviceRef, src/game/director.ts:85)
#[derive(Clone, Debug)]
pub struct DeviceRef {
    pub kind: crate::director::DeviceRefKind,
    pub id: String,
}
impl DeviceRef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DeviceRef {
            kind: crate::director::DeviceRefKind::Device,
            id: String::new(),
        }
    }
}
impl Default for DeviceRef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DeviceRefKind {
    Device,
}
impl DeviceRefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DeviceRefKind::Device => "device",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PlanKind {
    Breach,
    RoomClear,
}
impl PlanKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            PlanKind::Breach => "breach",
            PlanKind::RoomClear => "roomClear",
        }
    }
}

/// (TypeScript: UnitRef, src/game/director.ts:88)
#[derive(Clone, Debug)]
pub struct UnitRef {
    pub kind: crate::director::UnitRefKind,
    pub id: f64,
    pub callsign: String,
}
impl UnitRef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        UnitRef {
            kind: crate::director::UnitRefKind::Unit,
            id: 0.0,
            callsign: String::new(),
        }
    }
}
impl Default for UnitRef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum UnitRefKind {
    Unit,
}
impl UnitRefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            UnitRefKind::Unit => "unit",
        }
    }
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum Place {
    List(V3),
    Mark(crate::director::MarkRef),
}

/// (TypeScript: LoadoutSpec, src/game/director.ts:92)
#[derive(Clone, Copy, Debug)]
pub struct LoadoutSpec {
    pub team: crate::teams::Team,
    pub weapon: Option<crate::weapons::WeaponId>,
    pub tint: Option<V3>,
    pub tint2: Option<V3>,
    pub light: Option<bool>,
    pub trained: Option<bool>,
    pub god: Option<bool>,
}
impl LoadoutSpec {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LoadoutSpec {
            team: crate::teams::Team::Guard,
            weapon: None,
            tint: None,
            tint2: None,
            light: None,
            trained: None,
            god: None,
        }
    }
}
impl Default for LoadoutSpec { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:96 OrderPatch, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct OrderPatch2 {
    pub goal: Option<Option<crate::director::Place>>,
    pub speed: Option<Option<f64>>,
    pub face: Option<Option<f64>>,
    pub strafe: Option<Option<bool>>,
    pub aim_at: Option<Option<crate::director::OrderPatch2AimAt>>,
    pub stance: Option<Option<crate::character::Stance>>,
    pub stack_side: Option<Option<f64>>,
    pub crouch: Option<Option<bool>>,
    pub upper: Option<Option<crate::character::UpperMode>>,
    pub light: Option<Option<bool>>,
    pub hot: Option<Option<bool>>,
    pub flinch: Option<Option<bool>>,
    pub direct: Option<Option<bool>>,
    pub smoke: Option<Option<bool>>,
    /// INVENTED: true only from a caller that passes a goal it keeps and moves (orders.ts:1519, qrf.ts:828, :851): that one compares by value. goal Some(Some(_)) from anyone else is a new leg, even at the same spot (squad.ts:146)
    pub held: bool,
}
impl OrderPatch2 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OrderPatch2 {
            goal: None,
            speed: None,
            face: None,
            strafe: None,
            aim_at: None,
            stance: None,
            stack_side: None,
            crouch: None,
            upper: None,
            light: None,
            hot: None,
            flinch: None,
            direct: None,
            smoke: None,
            held: false,
        }
    }
}
impl Default for OrderPatch2 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum OrderPatch2AimAt {
    List(V3),
    Unit(crate::director::UnitRef),
    Mark(crate::director::MarkRef),
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum EventKind {
    Shot,
    Bang,
    Kick,
    Door,
    LightOut,
    GuardShot,
    Step,
    Prop,
    Whistle,
    Glass,
}
impl EventKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            EventKind::Shot => "shot",
            EventKind::Bang => "bang",
            EventKind::Kick => "kick",
            EventKind::Door => "door",
            EventKind::LightOut => "lightOut",
            EventKind::GuardShot => "guardShot",
            EventKind::Step => "step",
            EventKind::Prop => "prop",
            EventKind::Whistle => "whistle",
            EventKind::Glass => "glass",
        }
    }
}

/// (TypeScript: EventView, src/game/director.ts:100)
#[derive(Clone, Debug)]
pub struct EventView {
    pub kind: crate::director::EventViewKind,
    pub pos: V3,
    pub level: f64,
    pub loud: bool,
    pub who: f64,
    pub name: Option<String>,
    pub data: Option<crate::director::Json>,
}
impl EventView {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EventView {
            kind: crate::director::EventViewKind::Shot,
            pos: [0.0; 3],
            level: 0.0,
            loud: false,
            who: 0.0,
            name: None,
            data: None,
        }
    }
}
impl Default for EventView { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum EventViewKind {
    Shot,
    Bang,
    Kick,
    Door,
    LightOut,
    GuardShot,
    Step,
    Prop,
    Whistle,
    Glass,
    Signal,
}
impl EventViewKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            EventViewKind::Shot => "shot",
            EventViewKind::Bang => "bang",
            EventViewKind::Kick => "kick",
            EventViewKind::Door => "door",
            EventViewKind::LightOut => "lightOut",
            EventViewKind::GuardShot => "guardShot",
            EventViewKind::Step => "step",
            EventViewKind::Prop => "prop",
            EventViewKind::Whistle => "whistle",
            EventViewKind::Glass => "glass",
            EventViewKind::Signal => "signal",
        }
    }
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum Cond {
    Arm0(crate::director::CondArm0),
    Arm1(crate::director::CondArm1),
    Arm2(crate::director::CondArm2),
    Arm3(crate::director::CondArm3),
    Arm4(crate::director::CondArm4),
    Arm5(crate::director::CondArm5),
}

/// (TypeScript: inline at src/game/director.ts:106 Cond, src/game/director.ts:107)
#[derive(Clone, Debug)]
pub struct CondArm0 {
    pub k: crate::director::CondArm0K,
    pub until: f64,
    pub label: Option<String>,
}
impl CondArm0 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CondArm0 {
            k: crate::director::CondArm0K::Time,
            until: 0.0,
            label: None,
        }
    }
}
impl Default for CondArm0 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CondArm0K {
    Time,
}
impl CondArm0K {
    pub const fn as_str(self) -> &'static str {
        match self {
            CondArm0K::Time => "time",
        }
    }
}

/// (TypeScript: inline at src/game/director.ts:106 Cond, src/game/director.ts:108)
#[derive(Clone, Copy, Debug)]
pub struct CondArm1 {
    pub k: crate::director::CondArm1K,
    pub due: f64,
}
impl CondArm1 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CondArm1 {
            k: crate::director::CondArm1K::Frames,
            due: 0.0,
        }
    }
}
impl Default for CondArm1 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CondArm1K {
    Frames,
}
impl CondArm1K {
    pub const fn as_str(self) -> &'static str {
        match self {
            CondArm1K::Frames => "frames",
        }
    }
}

/// (TypeScript: inline at src/game/director.ts:106 Cond, src/game/director.ts:109)
#[derive(Clone, Debug)]
pub struct CondArm2 {
    pub k: crate::director::CondArm2K,
    pub test: crate::FnField,
    pub label: String,
}
impl CondArm2 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CondArm2 {
            k: crate::director::CondArm2K::Pred,
            test: crate::FnField,
            label: String::new(),
        }
    }
}
impl Default for CondArm2 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CondArm2K {
    Pred,
}
impl CondArm2K {
    pub const fn as_str(self) -> &'static str {
        match self {
            CondArm2K::Pred => "pred",
        }
    }
}

/// (TypeScript: inline at src/game/director.ts:106 Cond, src/game/director.ts:110)
#[derive(Clone, Debug)]
pub struct CondArm3 {
    pub k: crate::director::CondArm3K,
    pub kind: crate::director::CondArm3Kind,
    pub name: Option<String>,
    pub filter: Option<crate::director::EventFilter>,
    pub got: Option<crate::director::EventView>,
    pub tick0: f64,
}
impl CondArm3 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CondArm3 {
            k: crate::director::CondArm3K::Event,
            kind: crate::director::CondArm3Kind::Shot,
            name: None,
            filter: None,
            got: None,
            tick0: 0.0,
        }
    }
}
impl Default for CondArm3 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CondArm3K {
    Event,
}
impl CondArm3K {
    pub const fn as_str(self) -> &'static str {
        match self {
            CondArm3K::Event => "event",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CondArm3Kind {
    Shot,
    Bang,
    Kick,
    Door,
    LightOut,
    GuardShot,
    Step,
    Prop,
    Whistle,
    Glass,
    Signal,
}
impl CondArm3Kind {
    pub const fn as_str(self) -> &'static str {
        match self {
            CondArm3Kind::Shot => "shot",
            CondArm3Kind::Bang => "bang",
            CondArm3Kind::Kick => "kick",
            CondArm3Kind::Door => "door",
            CondArm3Kind::LightOut => "lightOut",
            CondArm3Kind::GuardShot => "guardShot",
            CondArm3Kind::Step => "step",
            CondArm3Kind::Prop => "prop",
            CondArm3Kind::Whistle => "whistle",
            CondArm3Kind::Glass => "glass",
            CondArm3Kind::Signal => "signal",
        }
    }
}

/// (TypeScript: inline at src/game/director.ts:106 Cond, src/game/director.ts:111)
#[derive(Clone, Debug)]
pub struct CondArm4 {
    pub k: crate::director::CondArm4K,
    pub name: String,
    pub state: crate::director::CondArm4State,
}
impl CondArm4 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CondArm4 {
            k: crate::director::CondArm4K::Beat,
            name: String::new(),
            state: crate::director::CondArm4State::Done,
        }
    }
}
impl Default for CondArm4 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CondArm4K {
    Beat,
}
impl CondArm4K {
    pub const fn as_str(self) -> &'static str {
        match self {
            CondArm4K::Beat => "beat",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CondArm4State {
    Done,
    Started,
}
impl CondArm4State {
    pub const fn as_str(self) -> &'static str {
        match self {
            CondArm4State::Done => "done",
            CondArm4State::Started => "started",
        }
    }
}

/// (TypeScript: inline at src/game/director.ts:106 Cond, src/game/director.ts:112)
#[derive(Clone, Debug)]
pub struct CondArm5 {
    pub k: crate::director::CondArm5K,
    pub of: Vec<crate::director::Cond>,
}
impl CondArm5 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CondArm5 {
            k: crate::director::CondArm5K::All,
            of: Vec::new(),
        }
    }
}
impl Default for CondArm5 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CondArm5K {
    All,
    Any,
}
impl CondArm5K {
    pub const fn as_str(self) -> &'static str {
        match self {
            CondArm5K::All => "all",
            CondArm5K::Any => "any",
        }
    }
}

/// (TypeScript: CondResult, src/game/director.ts:114)
#[derive(Clone, Debug)]
pub struct CondResult {
    pub index: f64,
    pub event: Option<crate::director::EventView>,
}
impl CondResult {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CondResult {
            index: 0.0,
            event: None,
        }
    }
}
impl Default for CondResult { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BeatState {
    Done,
    Failed,
    Running,
    Cancelled,
}
impl BeatState {
    pub const fn as_str(self) -> &'static str {
        match self {
            BeatState::Done => "done",
            BeatState::Failed => "failed",
            BeatState::Running => "running",
            BeatState::Cancelled => "cancelled",
        }
    }
}

/// (TypeScript: Query, src/game/director.ts:119)
#[derive(Clone, Copy, Debug)]
pub struct Query {
    pub t: f64,
    pub time: f64,
    pub escalation: f64,
    pub mains_on: bool,
    pub daylight: bool,
}
impl Query {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Query {
            t: 0.0,
            time: 0.0,
            escalation: 0.0,
            mains_on: false,
            daylight: false,
        }
    }
}
impl Default for Query { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DrillRef, src/game/director.ts:140)
#[derive(Clone, Debug)]
pub struct DrillRef {
    pub kind: crate::director::DrillRefKind,
    pub name: String,
    pub beat: String,
    pub phase: String,
    pub t: f64,
    pub done: bool,
    pub aborted: bool,
    pub cleared: bool,
    pub ops: Vec<crate::director::UnitRef>,
}
impl DrillRef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DrillRef {
            kind: crate::director::DrillRefKind::Drill,
            name: String::new(),
            beat: String::new(),
            phase: String::new(),
            t: 0.0,
            done: false,
            aborted: false,
            cleared: false,
            ops: Vec::new(),
        }
    }
}
impl Default for DrillRef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DrillRefKind {
    Drill,
}
impl DrillRefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DrillRefKind::Drill => "drill",
        }
    }
}

/// (TypeScript: CameraShotSpec, src/game/director.ts:142)
#[derive(Clone, Debug)]
pub struct CameraShotSpec {
    pub look_y: Option<f64>,
    pub track: Option<f64>,
    pub yaw: f64,
    pub yaw_to: Option<f64>,
    pub pitch: f64,
    pub dist: f64,
    pub dist_to: Option<f64>,
    pub dur: f64,
    pub title: Option<String>,
    pub chapter: Option<f64>,
}
impl CameraShotSpec {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CameraShotSpec {
            look_y: None,
            track: None,
            yaw: 0.0,
            yaw_to: None,
            pitch: 0.0,
            dist: 0.0,
            dist_to: None,
            dur: 0.0,
            title: None,
            chapter: None,
        }
    }
}
impl Default for CameraShotSpec { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BeatCtx, src/game/director.ts:150)
#[derive(Clone, Debug)]
pub struct BeatCtx {
    pub flag: JsMap<String, Option<crate::director::Json>>,
    pub query: crate::director::Query,
    pub dt: f64,
    pub view: crate::director::SamView,
    pub camera: crate::director::BeatCtxCamera,
}
impl BeatCtx {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BeatCtx {
            flag: JsMap::new(),
            query: crate::director::Query::zeroed(),
            dt: 0.0,
            view: crate::director::SamView::zeroed(),
            camera: crate::director::BeatCtxCamera::zeroed(),
        }
    }
}
impl Default for BeatCtx { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SamView, src/game/director.ts:225)
#[derive(Clone, Debug)]
pub struct SamView {
    pub pos: V3,
    pub yaw: f64,
    pub speed: f64,
    pub crouch: bool,
    pub holstered: bool,
    pub slot: String,
    pub nv: bool,
    pub hover: Option<crate::director::SamViewHover>,
    pub rows: Vec<String>,
    pub holding: Option<crate::director::SamViewHolding>,
    pub dragging: Option<String>,
    pub takedown: bool,
    pub easing: Option<String>,
    pub picking: Option<String>,
    pub coding: Option<String>,
    pub drive_t: f64,
    pub pulled: bool,
    pub stage: String,
    pub codes: Vec<String>,
    pub keys: Vec<String>,
    pub aim_target: Option<String>,
    pub aim_guard: Option<String>,
    pub ocp: f64,
    pub canisters: f64,
    pub flashbangs: f64,
    pub door_push: Option<String>,
}
impl SamView {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SamView {
            pos: [0.0; 3],
            yaw: 0.0,
            speed: 0.0,
            crouch: false,
            holstered: false,
            slot: String::new(),
            nv: false,
            hover: None,
            rows: Vec::new(),
            holding: None,
            dragging: None,
            takedown: false,
            easing: None,
            picking: None,
            coding: None,
            drive_t: 0.0,
            pulled: false,
            stage: String::new(),
            codes: Vec::new(),
            keys: Vec::new(),
            aim_target: None,
            aim_guard: None,
            ocp: 0.0,
            canisters: 0.0,
            flashbangs: 0.0,
            door_push: None,
        }
    }
}
impl Default for SamView { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:228 SamView.hover, src/game/director.ts:228)
#[derive(Clone, Debug)]
pub struct SamViewHover {
    pub kind: String,
    pub verb: Option<String>,
    pub in_reach: bool,
    pub who: Option<String>,
    pub running: bool,
}
impl SamViewHover {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SamViewHover {
            kind: String::new(),
            verb: None,
            in_reach: false,
            who: None,
            running: false,
        }
    }
}
impl Default for SamViewHover { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:232 SamView.holding, src/game/director.ts:232)
#[derive(Clone, Debug)]
pub struct SamViewHolding {
    pub who: String,
    pub phase: String,
    pub variant: String,
    pub t: f64,
}
impl SamViewHolding {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SamViewHolding {
            who: String::new(),
            phase: String::new(),
            variant: String::new(),
            t: 0.0,
        }
    }
}
impl Default for SamViewHolding { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:206 BeatCtx.camera, src/game/director.ts:206)
#[derive(Clone, Copy, Debug)]
pub struct BeatCtxCamera {
    pub shot_t: f64,
}
impl BeatCtxCamera {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BeatCtxCamera {
            shot_t: 0.0,
        }
    }
}
impl Default for BeatCtxCamera { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PuppetSpec, src/game/director.ts:223)
#[derive(Clone, Debug)]
pub struct PuppetSpec {
    pub goal: Option<crate::director::Place>,
    pub aim: Option<crate::director::Place>,
    pub crouch: Option<bool>,
    pub walk: Option<f64>,
    pub fixtures: Option<bool>,
    pub throw_at: Option<crate::director::Place>,
}
impl PuppetSpec {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PuppetSpec {
            goal: None,
            aim: None,
            crouch: None,
            walk: None,
            fixtures: None,
            throw_at: None,
        }
    }
}
impl Default for PuppetSpec { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ProgramOpts, src/game/director.ts:243)
#[derive(Clone, Copy, Debug)]
pub struct ProgramOpts {
    pub showcase: Option<bool>,
    pub lower_third: Option<bool>,
    pub end_veil: Option<f64>,
    pub end_hold: Option<f64>,
    pub fresh_encounter: Option<bool>,
}
impl ProgramOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ProgramOpts {
            showcase: None,
            lower_third: None,
            end_veil: None,
            end_hold: None,
            fresh_encounter: None,
        }
    }
}
impl Default for ProgramOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ProgramApi, src/game/director.ts:255)
#[derive(Clone, Copy, Debug)]
pub struct ProgramApi {
}
impl ProgramApi {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ProgramApi {
        }
    }
}
impl Default for ProgramApi { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DirectorApi, src/game/director.ts:262)
#[derive(Clone, Copy, Debug)]
pub struct DirectorApi {
}
impl DirectorApi {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DirectorApi {
        }
    }
}
impl Default for DirectorApi { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Manifest, src/game/director.ts:280)
#[derive(Clone, Debug)]
pub struct Manifest {
    pub marks: JsMap<String, crate::director::ManifestMarksValue>,
    pub volumes: JsSet<String>,
    pub routes: JsSet<String>,
    pub doors: JsSet<String>,
    pub devices: JsSet<String>,
    pub plans: JsMap<String, crate::director::PlanKind>,
    pub loadouts: JsMap<String, crate::director::LoadoutSpec>,
    pub groups: JsSet<String>,
    pub variants: JsSet<String>,
    pub programs: JsMap<String, crate::director::ProgramDef>,
    pub expects: Vec<crate::director::ManifestExpectsItem>,
    pub systems: JsMap<&'static str, bool>,
}
impl Manifest {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Manifest {
            marks: JsMap::new(),
            volumes: JsSet::new(),
            routes: JsSet::new(),
            doors: JsSet::new(),
            devices: JsSet::new(),
            plans: JsMap::new(),
            loadouts: JsMap::new(),
            groups: JsSet::new(),
            variants: JsSet::new(),
            programs: JsMap::new(),
            expects: Vec::new(),
            systems: JsMap::new(),
        }
    }
}
impl Default for Manifest { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:281 Manifest.marks, src/game/director.ts:281)
#[derive(Clone, Copy, Debug)]
pub struct ManifestMarksValue {
    pub pos: Option<V3>,
    pub yaw: Option<f64>,
}
impl ManifestMarksValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ManifestMarksValue {
            pos: None,
            yaw: None,
        }
    }
}
impl Default for ManifestMarksValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ProgramDef, src/game/director.ts:287)
#[derive(Clone, Debug)]
pub struct ProgramDef {
    pub name: String,
    pub opts: crate::director::ProgramOpts,
    pub beats: JsMap<String, crate::director::ProgramDefBeatsValue>,
    pub triggers: Vec<crate::director::TriggerDef>,
}
impl ProgramDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ProgramDef {
            name: String::new(),
            opts: crate::director::ProgramOpts::zeroed(),
            beats: JsMap::new(),
            triggers: Vec::new(),
        }
    }
}
impl Default for ProgramDef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:287 ProgramDef.beats, src/game/director.ts:287)
#[derive(Clone, Copy, Debug)]
pub struct ProgramDefBeatsValue {
    pub fn_: crate::FnField,
    pub atomic: bool,
}
impl ProgramDefBeatsValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ProgramDefBeatsValue {
            fn_: crate::FnField,
            atomic: false,
        }
    }
}
impl Default for ProgramDefBeatsValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: TriggerDef, src/game/director.ts:286)
#[derive(Clone, Debug)]
pub struct TriggerDef {
    pub kind: crate::director::TriggerDefKind,
    pub beat: String,
    pub secs: Option<f64>,
    pub event: Option<crate::director::EventKind>,
    pub name: Option<String>,
    pub volume: Option<String>,
    pub who: Option<crate::director::TriggerDefWho>,
    pub once: Option<bool>,
    pub label: Option<String>,
}
impl TriggerDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TriggerDef {
            kind: crate::director::TriggerDefKind::Start,
            beat: String::new(),
            secs: None,
            event: None,
            name: None,
            volume: None,
            who: None,
            once: None,
            label: None,
        }
    }
}
impl Default for TriggerDef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TriggerDefKind {
    Start,
    Enter,
    After,
    Event,
    Signal,
    When,
}
impl TriggerDefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            TriggerDefKind::Start => "start",
            TriggerDefKind::Enter => "enter",
            TriggerDefKind::After => "after",
            TriggerDefKind::Event => "event",
            TriggerDefKind::Signal => "signal",
            TriggerDefKind::When => "when",
        }
    }
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum TriggerDefWho {
    Sam,
    AnyUnit,
    Unit(crate::director::UnitRef),
}

/// (TypeScript: inline at src/game/director.ts:283 Manifest.expects, src/game/director.ts:283)
#[derive(Clone, Debug)]
pub struct ManifestExpectsItem {
    pub kind: crate::director::ManifestExpectsItemKind,
    pub program: Option<String>,
    pub beats: Option<Vec<String>>,
    pub a: Option<String>,
    pub b: Option<String>,
    pub max: Option<f64>,
    pub what: Option<String>,
}
impl ManifestExpectsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ManifestExpectsItem {
            kind: crate::director::ManifestExpectsItemKind::Order,
            program: None,
            beats: None,
            a: None,
            b: None,
            max: None,
            what: None,
        }
    }
}
impl Default for ManifestExpectsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ManifestExpectsItemKind {
    Order,
    Within,
    Never,
}
impl ManifestExpectsItemKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            ManifestExpectsItemKind::Order => "order",
            ManifestExpectsItemKind::Within => "within",
            ManifestExpectsItemKind::Never => "never",
        }
    }
}

/// (TypeScript: inline at quiet Manifest.systems, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct ManifestSystems {
    pub civilians: Option<bool>,
    pub qrf: Option<bool>,
    pub needs: Option<bool>,
    pub autosave: Option<bool>,
    pub their_frags: Option<bool>,
}
impl ManifestSystems {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ManifestSystems {
            civilians: None,
            qrf: None,
            needs: None,
            autosave: None,
            their_frags: None,
        }
    }
}
impl Default for ManifestSystems { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: LintProblem, src/game/director.ts:288)
#[derive(Clone, Debug)]
pub struct LintProblem {
    pub level: crate::director::LintProblemLevel,
    pub what: String,
}
impl LintProblem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LintProblem {
            level: crate::director::LintProblemLevel::Error,
            what: String::new(),
        }
    }
}
impl Default for LintProblem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LintProblemLevel {
    Error,
    Warn,
}
impl LintProblemLevel {
    pub const fn as_str(self) -> &'static str {
        match self {
            LintProblemLevel::Error => "error",
            LintProblemLevel::Warn => "warn",
        }
    }
}

/// (TypeScript: LogRow, src/game/director.ts:396)
#[derive(Clone, Debug)]
pub struct LogRow {
    pub t: f64,
    pub run: f64,
    pub program: String,
    pub beat: String,
    pub kind: crate::director::LogRowKind,
    pub what: String,
    pub data: Option<crate::director::Json>,
}
impl LogRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LogRow {
            t: 0.0,
            run: 0.0,
            program: String::new(),
            beat: String::new(),
            kind: crate::director::LogRowKind::Camera,
            what: String::new(),
            data: None,
        }
    }
}
impl Default for LogRow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LogRowKind {
    Camera,
    Radio,
    Error,
    Spawn,
    Beat,
    Drill,
    Note,
    Program,
    Trigger,
    Yield,
    Stimulus,
}
impl LogRowKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            LogRowKind::Camera => "camera",
            LogRowKind::Radio => "radio",
            LogRowKind::Error => "error",
            LogRowKind::Spawn => "spawn",
            LogRowKind::Beat => "beat",
            LogRowKind::Drill => "drill",
            LogRowKind::Note => "note",
            LogRowKind::Program => "program",
            LogRowKind::Trigger => "trigger",
            LogRowKind::Yield => "yield",
            LogRowKind::Stimulus => "stimulus",
        }
    }
}

/// (TypeScript: DirectorFrame, src/game/director.ts:399)
#[derive(Clone, Debug)]
pub struct DirectorFrame {
    pub look: Option<V3>,
    pub caption: String,
    pub sub: String,
    pub fade: f64,
    pub lower_third: bool,
    pub program: String,
    pub handed_over: bool,
}
impl DirectorFrame {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DirectorFrame {
            look: None,
            caption: String::new(),
            sub: String::new(),
            fade: 0.0,
            lower_third: false,
            program: String::new(),
            handed_over: false,
        }
    }
}
impl Default for DirectorFrame { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: LiveBeat, src/game/director.ts:401)
#[derive(Clone, Debug)]
pub struct LiveBeat {
    pub name: String,
    pub it: crate::Opaque,
    pub waiting: Option<crate::director::Cond>,
    pub result: crate::director::CondResult,
    pub t0: f64,
    pub step: f64,
    pub state: crate::director::BeatState,
    pub touched: JsSet<crate::Num>,
    pub pinned_camera: bool,
    pub flow: crate::director::LiveBeatFlow,
}
impl LiveBeat {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LiveBeat {
            name: String::new(),
            it: crate::Opaque,
            waiting: None,
            result: crate::director::CondResult::zeroed(),
            t0: 0.0,
            step: 0.0,
            state: crate::director::BeatState::Done,
            touched: JsSet::new(),
            pinned_camera: false,
            flow: crate::director::LiveBeatFlow::zeroed(),
        }
    }
}
impl Default for LiveBeat { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:401 LiveBeat.flow, src/game/director.ts:401)
#[derive(Clone, Copy, Debug)]
pub struct LiveBeatFlow {
    pub finish: bool,
}
impl LiveBeatFlow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LiveBeatFlow {
            finish: false,
        }
    }
}
impl Default for LiveBeatFlow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: UnitRec, src/game/director.ts:402)
#[derive(Clone, Debug)]
pub struct UnitRec {
    pub ref_: crate::director::UnitRef,
    pub gd: GuardId,
    pub loadout: String,
    pub by: String,
}
impl UnitRec {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        UnitRec {
            ref_: crate::director::UnitRef::zeroed(),
            gd: GuardId(0),
            loadout: String::new(),
            by: String::new(),
        }
    }
}
impl Default for UnitRec { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DrillImpl {
    Breach(crate::director::DrillImplBreach),
    Clear(crate::director::DrillImplClear),
    Hold,
    Exchange(crate::director::DrillImplExchange),
}

/// (TypeScript: inline at src/game/director.ts:403 DrillImpl, src/game/director.ts:404)
#[derive(Clone, Copy, Debug)]
pub struct DrillImplBreach {
    pub forwarded: f64,
}
impl DrillImplBreach {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DrillImplBreach {
            forwarded: 0.0,
        }
    }
}
impl Default for DrillImplBreach { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:403 DrillImpl, src/game/director.ts:405)
#[derive(Clone, Debug)]
pub struct DrillImplClear {
    pub d: RoomClearId,
    pub clock: f64,
    pub last_phase: String,
}
impl DrillImplClear {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DrillImplClear {
            d: RoomClearId(0),
            clock: 0.0,
            last_phase: String::new(),
        }
    }
}
impl Default for DrillImplClear { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:403 DrillImpl, src/game/director.ts:407)
#[derive(Clone, Debug)]
pub struct DrillImplExchange {
    pub e: crate::guards::Engagement,
}
impl DrillImplExchange {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DrillImplExchange {
            e: crate::guards::Engagement::zeroed(),
        }
    }
}
impl Default for DrillImplExchange { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DrillRun, src/game/director.ts:408)
#[derive(Clone, Debug)]
pub struct DrillRun {
    pub ref_: crate::director::DrillRef,
    pub impl_: crate::director::DrillImpl,
    pub beat: String,
    pub owner: String,
    pub ops: Vec<crate::director::UnitRef>,
    pub men: Vec<GuardId>,
    pub times: JsMap<String, f64>,
    pub t: f64,
    pub phase: String,
    pub done: bool,
    pub aborted: bool,
    pub cleared: bool,
}
impl DrillRun {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DrillRun {
            ref_: crate::director::DrillRef::zeroed(),
            impl_: crate::director::DrillImpl::Breach(crate::director::DrillImplBreach::zeroed()),
            beat: String::new(),
            owner: String::new(),
            ops: Vec::new(),
            men: Vec::new(),
            times: JsMap::new(),
            t: 0.0,
            phase: String::new(),
            done: false,
            aborted: false,
            cleared: false,
        }
    }
}
impl Default for DrillRun { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Shot, src/game/director.ts:409)
#[derive(Clone, Debug)]
pub struct Shot {
    pub spec: crate::director::CameraShotSpec,
    pub t: f64,
    pub title: String,
}
impl Shot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Shot {
            spec: crate::director::CameraShotSpec::zeroed(),
            t: 0.0,
            title: String::new(),
        }
    }
}
impl Default for Shot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ShowState, src/game/director.ts:410)
#[derive(Clone, Debug)]
pub struct ShowState {
    pub saved: crate::director::ShowStateSaved,
    pub look: V3,
    pub shot: Option<crate::director::Shot>,
    pub shot_i: f64,
    pub chapter_t0: f64,
    pub chapters: Vec<crate::director::ShowStateChaptersItem>,
    pub caption: String,
    pub sub: String,
    pub fade: f64,
    pub fade_to: f64,
    pub fade_rate: f64,
    pub ending: Option<crate::director::ShowStateEnding>,
    pub benched: Option<Vec<GuardId>>,
    pub variant_on: bool,
    pub puppet: bool,
    pub weather_saved: Option<crate::director::ShowStateWeatherSaved>,
    pub mission_on: bool,
    pub handed_over: bool,
    pub chaos_set: bool,
}
impl ShowState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShowState {
            saved: crate::director::ShowStateSaved::zeroed(),
            look: [0.0; 3],
            shot: None,
            shot_i: 0.0,
            chapter_t0: 0.0,
            chapters: Vec::new(),
            caption: String::new(),
            sub: String::new(),
            fade: 0.0,
            fade_to: 0.0,
            fade_rate: 0.0,
            ending: None,
            benched: None,
            variant_on: false,
            puppet: false,
            weather_saved: None,
            mission_on: false,
            handed_over: false,
            chaos_set: false,
        }
    }
}
impl Default for ShowState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:410 ShowState.saved, src/game/director.ts:410)
#[derive(Clone, Copy, Debug)]
pub struct ShowStateSaved {
    pub ai: bool,
    pub god: bool,
    pub invisible: bool,
    pub quiet: bool,
    pub touring: bool,
    pub cam: Option<crate::director::ShowStateSavedCam>,
}
impl ShowStateSaved {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShowStateSaved {
            ai: false,
            god: false,
            invisible: false,
            quiet: false,
            touring: false,
            cam: None,
        }
    }
}
impl Default for ShowStateSaved { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:410 ShowStateSaved.cam, src/game/director.ts:410)
#[derive(Clone, Copy, Debug)]
pub struct ShowStateSavedCam {
    pub yaw: f64,
    pub pitch: f64,
    pub dist: f64,
}
impl ShowStateSavedCam {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShowStateSavedCam {
            yaw: 0.0,
            pitch: 0.0,
            dist: 0.0,
        }
    }
}
impl Default for ShowStateSavedCam { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:411 ShowState.chapters, src/game/director.ts:411)
#[derive(Clone, Debug)]
pub struct ShowStateChaptersItem {
    pub title: String,
    pub dur: f64,
}
impl ShowStateChaptersItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShowStateChaptersItem {
            title: String::new(),
            dur: 0.0,
        }
    }
}
impl Default for ShowStateChaptersItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:411 ShowState.ending, src/game/director.ts:411)
#[derive(Clone, Copy, Debug)]
pub struct ShowStateEnding {
    pub t: f64,
}
impl ShowStateEnding {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShowStateEnding {
            t: 0.0,
        }
    }
}
impl Default for ShowStateEnding { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:414 ShowState.weatherSaved, src/game/director.ts:414)
#[derive(Clone, Copy, Debug)]
pub struct ShowStateWeatherSaved {
    pub storm: f64,
    pub rain: f64,
}
impl ShowStateWeatherSaved {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShowStateWeatherSaved {
            storm: 0.0,
            rain: 0.0,
        }
    }
}
impl Default for ShowStateWeatherSaved { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Run, src/game/director.ts:415)
#[derive(Clone, Debug)]
pub struct Run {
    pub serial: f64,
    pub name: String,
    pub def: crate::director::ProgramDef,
    pub t: f64,
    pub dt: f64,
    pub tick_n: f64,
    pub dice: crate::dice::Die,
    pub seed: f64,
    pub beats: Vec<DirectorLiveBeatId>,
    pub fired: JsSet<crate::Num>,
    pub signals: Vec<crate::director::RunSignalsItem>,
    pub sig_seen: f64,
    pub flags: JsMap<String, Option<crate::director::Json>>,
    pub units: JsMap<crate::Num, crate::director::UnitRec>,
    pub bursts: Vec<crate::director::RunBurstsItem>,
    pub kicks: Vec<crate::director::RunKicksItem>,
    pub show: Option<crate::director::ShowState>,
    pub objective: Option<crate::director::RunObjective>,
    pub systems_saved: Option<JsMap<&'static str, bool>>,
    pub ended: bool,
    pub finishing: bool,
    pub failed_show: bool,
}
impl Run {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Run {
            serial: 0.0,
            name: String::new(),
            def: crate::director::ProgramDef::zeroed(),
            t: 0.0,
            dt: 0.0,
            tick_n: 0.0,
            dice: crate::dice::Die::default(),
            seed: 0.0,
            beats: Vec::new(),
            fired: JsSet::new(),
            signals: Vec::new(),
            sig_seen: 0.0,
            flags: JsMap::new(),
            units: JsMap::new(),
            bursts: Vec::new(),
            kicks: Vec::new(),
            show: None,
            objective: None,
            systems_saved: None,
            ended: false,
            finishing: false,
            failed_show: false,
        }
    }
}
impl Default for Run { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:415 Run.signals, src/game/director.ts:415)
#[derive(Clone, Debug)]
pub struct RunSignalsItem {
    pub name: String,
    pub t: f64,
    pub data: Option<crate::director::Json>,
}
impl RunSignalsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RunSignalsItem {
            name: String::new(),
            t: 0.0,
            data: None,
        }
    }
}
impl Default for RunSignalsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:416 Run.bursts, src/game/director.ts:416)
#[derive(Clone, Debug)]
pub struct RunBurstsItem {
    pub u: crate::director::UnitRef,
    pub at: crate::director::RunBurstsItemAt,
    pub left: f64,
    pub cadence: f64,
    pub spread: f64,
    pub next: f64,
    pub cond: crate::director::RunBurstsItemCond,
    pub fired: f64,
}
impl RunBurstsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RunBurstsItem {
            u: crate::director::UnitRef::zeroed(),
            at: crate::director::RunBurstsItemAt::List([0.0; 3]),
            left: 0.0,
            cadence: 0.0,
            spread: 0.0,
            next: 0.0,
            cond: crate::director::RunBurstsItemCond::zeroed(),
            fired: 0.0,
        }
    }
}
impl Default for RunBurstsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum RunBurstsItemAt {
    List(V3),
    Unit(crate::director::UnitRef),
    Mark(crate::director::MarkRef),
}

/// (TypeScript: inline at src/game/director.ts:416 RunBurstsItem.cond, src/game/director.ts:416)
#[derive(Clone, Debug)]
pub struct RunBurstsItemCond {
    pub k: crate::director::RunBurstsItemCondK,
    pub label: String,
}
impl RunBurstsItemCond {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RunBurstsItemCond {
            k: crate::director::RunBurstsItemCondK::Pred,
            label: String::new(),
        }
    }
}
impl Default for RunBurstsItemCond { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RunBurstsItemCondK {
    Pred,
}
impl RunBurstsItemCondK {
    pub const fn as_str(self) -> &'static str {
        match self {
            RunBurstsItemCondK::Pred => "pred",
        }
    }
}

/// (TypeScript: inline at src/game/director.ts:418 Run.kicks, src/game/director.ts:418)
#[derive(Clone, Debug)]
pub struct RunKicksItem {
    pub door: DoorId,
    pub gd: GuardId,
    pub by: String,
    pub beat: String,
}
impl RunKicksItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RunKicksItem {
            door: DoorId(0),
            gd: GuardId(0),
            by: String::new(),
            beat: String::new(),
        }
    }
}
impl Default for RunKicksItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:419 Run.objective, src/game/director.ts:419)
#[derive(Clone, Debug)]
pub struct RunObjective {
    pub text: String,
    pub pos: Option<V3>,
}
impl RunObjective {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RunObjective {
            text: String::new(),
            pos: None,
        }
    }
}
impl Default for RunObjective { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet Run.systemsSaved, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct RunSystemsSaved {
    pub civilians: Option<bool>,
    pub qrf: Option<bool>,
    pub needs: Option<bool>,
    pub autosave: Option<bool>,
}
impl RunSystemsSaved {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RunSystemsSaved {
            civilians: None,
            qrf: None,
            needs: None,
            autosave: None,
        }
    }
}
impl Default for RunSystemsSaved { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BeatFailure, src/game/director.ts:420)
#[derive(Clone, Debug)]
pub struct BeatFailure {
    pub name: String,
    pub message: String,
    pub stack: Option<String>,
    pub cause: Option<crate::Opaque>,
}
impl BeatFailure {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        BeatFailure {
            name: String::new(),
            message: String::new(),
            stack: None,
            cause: None,
        }
    }
}
impl Default for BeatFailure { fn default() -> Self { Self::blank() } }

/// (TypeScript: Director, src/game/director.ts:422)
#[derive(Clone, Debug)]
pub struct Director {
    pub manifest: Option<crate::director::Manifest>,
    pub problems: Vec<crate::director::LintProblem>,
    pub scan: Vec<crate::director::DirectorScanItem>,
    pub armed: bool,
    pub log: Vec<crate::director::LogRow>,
    pub run: Option<DirectorRunId>,
    pub last_run: Option<crate::director::DirectorLastRun>,
    pub strict: bool,
    pub frame: Option<crate::director::DirectorFrame>,
    pub cam: Option<CameraId>,
    pub input: Option<InputId>,
    pub serial: f64,
    pub seen: JsSet<u32>,
    pub exiting: bool,
    pub due_keys: JsSet<String>,
    pub due_clicks: f64,
    pub shown: JsSet<GuardId>,
}
impl Director {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Director {
            manifest: None,
            problems: Vec::new(),
            scan: Vec::new(),
            armed: false,
            log: Vec::new(),
            run: None,
            last_run: None,
            strict: false,
            frame: None,
            cam: None,
            input: None,
            serial: 0.0,
            seen: JsSet::new(),
            exiting: false,
            due_keys: JsSet::new(),
            due_clicks: 0.0,
            shown: JsSet::new(),
        }
    }
}
impl Default for Director { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/director.ts:424 Director.scan, src/game/director.ts:424)
#[derive(Clone, Debug)]
pub struct DirectorScanItem {
    pub program: String,
    pub beat: String,
    pub verb: String,
}
impl DirectorScanItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DirectorScanItem {
            program: String::new(),
            beat: String::new(),
            verb: String::new(),
        }
    }
}
impl Default for DirectorScanItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:427 Director.lastRun, src/game/director.ts:427)
#[derive(Clone, Debug)]
pub struct DirectorLastRun {
    pub serial: f64,
    pub name: String,
    pub t: f64,
    pub finished: bool,
}
impl DirectorLastRun {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DirectorLastRun {
            serial: 0.0,
            name: String::new(),
            t: 0.0,
            finished: false,
        }
    }
}
impl Default for DirectorLastRun { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:329 UTIL, src/game/director.ts:271)
#[derive(Clone, Copy, Debug)]
pub struct UTIL {
}
impl UTIL {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        UTIL {
        }
    }
}
impl Default for UTIL { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:338 lintManifest(level), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1604)
#[derive(Clone, Debug)]
pub struct LintManifestLevel {
    pub lights: Vec<crate::scene_level::LightDef>,
    pub routes: Vec<RouteId>,
    pub breach: Option<crate::scene_level::BreachPlan>,
    pub devices: Option<Vec<crate::scene_level::DeviceDef>>,
    pub doors: Vec<crate::scene_level::LevelDoorDef>,
    pub switchables: Vec<crate::scene_level::LevelSwitchablesItem>,
    pub clear_plans: Option<Vec<crate::scene_level::RoomClearPlan>>,
    pub daylight: Option<crate::scene_level::DaylightDef>,
    pub volumes: Option<Vec<crate::scene_level::VolumeDef>>,
}
impl LintManifestLevel {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LintManifestLevel {
            lights: Vec::new(),
            routes: Vec::new(),
            breach: None,
            devices: None,
            doors: Vec::new(),
            switchables: Vec::new(),
            clear_plans: None,
            daylight: None,
            volumes: None,
        }
    }
}
impl Default for LintManifestLevel { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:379 scanShowcaseVerbs ->, src/game/director.ts:379)
#[derive(Clone, Debug)]
pub struct ScanShowcaseVerbsItem {
    pub program: String,
    pub beat: String,
    pub verb: String,
}
impl ScanShowcaseVerbsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ScanShowcaseVerbsItem {
            program: String::new(),
            beat: String::new(),
            verb: String::new(),
        }
    }
}
impl Default for ScanShowcaseVerbsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:452 attach(o), src/game/director.ts:452)
#[derive(Clone, Copy, Debug)]
pub struct DirectorAttachO {
    pub cam: Option<CameraId>,
    pub input: Option<InputId>,
}
impl DirectorAttachO {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DirectorAttachO {
            cam: None,
            input: None,
        }
    }
}
impl Default for DirectorAttachO { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:477 objective ->, src/game/director.ts:477)
#[derive(Clone, Debug)]
pub struct DirectorObjective {
    pub text: String,
    pub pos: Option<V3>,
}
impl DirectorObjective {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DirectorObjective {
            text: String::new(),
            pos: None,
        }
    }
}
impl Default for DirectorObjective { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DirectorSignalData {
    Text(String),
    Num(f64),
    Bool(bool),
    Array(Vec<Option<crate::director::Json>>),
    Arm4(JsMap<String, Option<crate::director::Json>>),
}

/// (TypeScript: inline at src/game/director.ts:482 chapters ->, src/game/director.ts:482)
#[derive(Clone, Debug)]
pub struct DirectorChapters {
    pub list: Vec<crate::director::DirectorChaptersListItem>,
    pub i: f64,
    pub frac: f64,
}
impl DirectorChapters {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DirectorChapters {
            list: Vec::new(),
            i: 0.0,
            frac: 0.0,
        }
    }
}
impl Default for DirectorChapters { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:482 DirectorChapters.list, src/game/director.ts:482)
#[derive(Clone, Debug)]
pub struct DirectorChaptersListItem {
    pub title: String,
    pub dur: f64,
}
impl DirectorChaptersListItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DirectorChaptersListItem {
            title: String::new(),
            dur: 0.0,
        }
    }
}
impl Default for DirectorChaptersListItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/director.ts:493 push(row), src/game/director.ts:493 push(row))
#[derive(Clone, Debug)]
pub struct DirectorPushRow {
    pub kind: crate::director::DirectorPushRowKind,
    pub what: String,
    pub data: Option<crate::director::Json>,
    pub beat: String,
    pub t: Option<f64>,
}
impl DirectorPushRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DirectorPushRow {
            kind: crate::director::DirectorPushRowKind::Camera,
            what: String::new(),
            data: None,
            beat: String::new(),
            t: None,
        }
    }
}
impl Default for DirectorPushRow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DirectorPushRowKind {
    Camera,
    Radio,
    Error,
    Spawn,
    Beat,
    Drill,
    Note,
    Program,
    Trigger,
    Yield,
    Stimulus,
}
impl DirectorPushRowKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DirectorPushRowKind::Camera => "camera",
            DirectorPushRowKind::Radio => "radio",
            DirectorPushRowKind::Error => "error",
            DirectorPushRowKind::Spawn => "spawn",
            DirectorPushRowKind::Beat => "beat",
            DirectorPushRowKind::Drill => "drill",
            DirectorPushRowKind::Note => "note",
            DirectorPushRowKind::Program => "program",
            DirectorPushRowKind::Trigger => "trigger",
            DirectorPushRowKind::Yield => "yield",
            DirectorPushRowKind::Stimulus => "stimulus",
        }
    }
}

/// (TypeScript: inline at src/game/director.ts:502 start(opts), src/game/director.ts:502)
#[derive(Clone, Copy, Debug)]
pub struct DirectorStartOpts {
    pub seed: Option<f64>,
}
impl DirectorStartOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DirectorStartOpts {
            seed: None,
        }
    }
}
impl Default for DirectorStartOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PlanRef, src/game/director.ts:87)
#[derive(Clone, Debug)]
pub struct PlanRef {
    pub kind: crate::director::PlanRefKind,
    pub id: String,
    pub plan: crate::director::PlanKind,
}
impl PlanRef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlanRef {
            kind: crate::director::PlanRefKind::Plan,
            id: String::new(),
            plan: crate::director::PlanKind::Breach,
        }
    }
}
impl Default for PlanRef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PlanRefKind {
    Plan,
}
impl PlanRefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            PlanRefKind::Plan => "plan",
        }
    }
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DirectorPlanOf {
    BreachPlan(crate::scene_level::BreachPlan),
    RoomClearPlan(crate::scene_level::RoomClearPlan),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DirectorInsideVolumeWho {
    Team,
    Sam,
    AnyUnit,
    Unit(crate::director::UnitRef),
}

/// (TypeScript: inline at src/game/director.ts:1115 signalView(s), src/game/director.ts:1115)
#[derive(Clone, Debug)]
pub struct SignalViewS {
    pub name: String,
    pub t: f64,
    pub data: Option<crate::director::Json>,
}
impl SignalViewS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SignalViewS {
            name: String::new(),
            t: 0.0,
            data: None,
        }
    }
}
impl Default for SignalViewS { fn default() -> Self { Self::zeroed() } }

/// (src/game/director.ts:329 UTIL: a table nothing writes)
pub fn util() -> crate::director::UTIL { crate::director::UTIL {  } }

/// (src/game/director.ts:334 TEAMS: a table nothing writes)
pub fn teams() -> Vec<String> { alloc::vec![String::from("guard"), String::from("operative"), String::from("hostile")] }

/// (src/game/director.ts:376 SHOWCASE_VERBS: a table nothing writes)
pub fn showcase_verbs() -> [String; 16] { [String::from("stage"), String::from("puppet"), String::from("press"), String::from("click"), String::from("camera"), String::from("caption"), String::from("chapters"), String::from("fade"), String::from("variant"), String::from("shift"), String::from("select"), String::from("view"), String::from("weather"), String::from("mission"), String::from("handover"), String::from("chaos")] }

/// (src/game/director.ts:393 MAX_RESUMES)
pub const MAX_RESUMES: f64 = 64.0;

/// (src/game/director.ts:395 LOG_CAP)
pub const LOG_CAP: f64 = 2500.0;

/// (src/game/director.ts:395 MAX_WHEN)
pub const MAX_WHEN: f64 = 32.0;

/// (src/game/director.ts:395 MAX_PASSES)
pub const MAX_PASSES: f64 = 8.0;
