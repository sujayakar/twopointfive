//! The types of `guards`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// what litPlace last read the light for besides the man himself: the man he has, or the point he is turning to (guards.ts litPlace's `key`; null is the Option's None)
#[derive(Clone, Debug)]
pub enum LitSeenTgt {
    Man { gd: GuardId },
    Pt,
}

/// (TypeScript: Engagement, src/game/guards.ts:3571)
#[derive(Clone, Debug)]
pub struct Engagement {
    pub operatives: Vec<GuardId>,
    pub hostiles: Vec<GuardId>,
    pub t0: f64,
    pub timeout: f64,
    pub done: bool,
    pub result: Option<crate::guards::EngageResult>,
    pub kills: Vec<crate::guards::EngageResultKillsItem>,
    pub rounds: crate::guards::EngagementRounds,
    pub alive: JsSet<GuardId>,
}
impl Engagement {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Engagement {
            operatives: Vec::new(),
            hostiles: Vec::new(),
            t0: 0.0,
            timeout: 0.0,
            done: false,
            result: None,
            kills: Vec::new(),
            rounds: crate::guards::EngagementRounds::zeroed(),
            alive: JsSet::new(),
        }
    }
}
impl Default for Engagement { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: EngageResult, src/game/guards.ts:3570)
#[derive(Clone, Debug)]
pub struct EngageResult {
    pub outcome: crate::guards::EngageResultOutcome,
    pub secs: f64,
    pub kills: Vec<crate::guards::EngageResultKillsItem>,
    pub rounds: crate::guards::EngageResultRounds,
    pub operatives_left: f64,
    pub hostiles_left: f64,
}
impl EngageResult {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EngageResult {
            outcome: crate::guards::EngageResultOutcome::Wiped,
            secs: 0.0,
            kills: Vec::new(),
            rounds: crate::guards::EngageResultRounds::zeroed(),
            operatives_left: 0.0,
            hostiles_left: 0.0,
        }
    }
}
impl Default for EngageResult { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum EngageResultOutcome {
    Wiped,
    Reset,
    Cleared,
    Timeout,
    Superseded,
}
impl EngageResultOutcome {
    pub const fn as_str(self) -> &'static str {
        match self {
            EngageResultOutcome::Wiped => "wiped",
            EngageResultOutcome::Reset => "reset",
            EngageResultOutcome::Cleared => "cleared",
            EngageResultOutcome::Timeout => "timeout",
            EngageResultOutcome::Superseded => "superseded",
        }
    }
}

/// (TypeScript: inline at src/game/guards.ts:3570 EngageResult.kills, src/game/guards.ts:3570)
#[derive(Clone, Debug)]
pub struct EngageResultKillsItem {
    pub victim: String,
    pub team: crate::teams::Team,
    pub by: String,
    pub t: f64,
}
impl EngageResultKillsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EngageResultKillsItem {
            victim: String::new(),
            team: crate::teams::Team::Guard,
            by: String::new(),
            t: 0.0,
        }
    }
}
impl Default for EngageResultKillsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:3570 EngageResult.rounds, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct EngageResultRounds {
    pub guard: Option<f64>,
    pub operative: Option<f64>,
    pub hostile: Option<f64>,
    pub player: Option<f64>,
    pub civilian: Option<f64>,
}
impl EngageResultRounds {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EngageResultRounds {
            guard: None,
            operative: None,
            hostile: None,
            player: None,
            civilian: None,
        }
    }
}
impl Default for EngageResultRounds { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:3573 Engagement.rounds, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct EngagementRounds {
    pub guard: Option<f64>,
    pub operative: Option<f64>,
    pub hostile: Option<f64>,
    pub player: Option<f64>,
    pub civilian: Option<f64>,
}
impl EngagementRounds {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EngagementRounds {
            guard: None,
            operative: None,
            hostile: None,
            player: None,
            civilian: None,
        }
    }
}
impl Default for EngagementRounds { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Standoff, src/game/guards.ts:851)
#[derive(Clone, Copy, Debug)]
pub struct Standoff {
    pub pair: GuardId,
    pub since: f64,
    pub side: f64,
    pub way: f64,
    pub orbit_goal: Option<V3>,
    pub replan_t: f64,
    pub rest_t: f64,
    pub seen_t: f64,
    pub last_bark_t: f64,
    pub line_open: bool,
    pub shots: f64,
    pub orbit_m: f64,
    pub stalls: f64,
    pub cornered: bool,
    pub backing: bool,
}
impl Standoff {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Standoff {
            pair: GuardId(0),
            since: 0.0,
            side: 0.0,
            way: 0.0,
            orbit_goal: None,
            replan_t: 0.0,
            rest_t: 0.0,
            seen_t: 0.0,
            last_bark_t: 0.0,
            line_open: false,
            shots: 0.0,
            orbit_m: 0.0,
            stalls: 0.0,
            cornered: false,
            backing: false,
        }
    }
}
impl Default for Standoff { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Pair, src/game/guards.ts:856)
#[derive(Clone, Copy, Debug)]
pub struct Pair {
    pub key: u32,
    pub held: GuardId,
    pub sam: V3,
    pub held_axis: V3,
    pub centre: V3,
    pub facing: f64,
    pub aim: V3,
    pub vel: V3,
}
impl Pair {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Pair {
            key: 0,
            held: GuardId(0),
            sam: [0.0; 3],
            held_axis: [0.0; 3],
            centre: [0.0; 3],
            facing: 0.0,
            aim: [0.0; 3],
            vel: [0.0; 3],
        }
    }
}
impl Default for Pair { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet Pair.key, quiet Pair.key)
#[derive(Clone, Copy, Debug)]
pub struct PairKey {
}
impl PairKey {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PairKey {
        }
    }
}
impl Default for PairKey { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum StandoffMoment {
    Lost,
    Enter,
    Line,
    Cornered,
    NoShot,
    Dropped,
    Freed,
    HitHim,
}
impl StandoffMoment {
    pub const fn as_str(self) -> &'static str {
        match self {
            StandoffMoment::Lost => "lost",
            StandoffMoment::Enter => "enter",
            StandoffMoment::Line => "line",
            StandoffMoment::Cornered => "cornered",
            StandoffMoment::NoShot => "noShot",
            StandoffMoment::Dropped => "dropped",
            StandoffMoment::Freed => "freed",
            StandoffMoment::HitHim => "hitHim",
        }
    }
}

/// (TypeScript: Eye, src/game/guards.ts:1319)
#[derive(Clone, Copy, Debug)]
pub struct Eye {
    pub pos: V3,
    pub look: V3,
    pub range: f64,
    pub half_deg: f64,
    pub near_m: f64,
    pub near_deg: f64,
    pub v_half_deg: Option<f64>,
}
impl Eye {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Eye {
            pos: [0.0; 3],
            look: [0.0; 3],
            range: 0.0,
            half_deg: 0.0,
            near_m: 0.0,
            near_deg: 0.0,
            v_half_deg: None,
        }
    }
}
impl Default for Eye { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: UnstickLeg, src/game/guards.ts:1375)
#[derive(Clone, Copy, Debug)]
pub struct UnstickLeg {
    pub to: V3,
    pub best: f64,
    pub stall_t: f64,
}
impl UnstickLeg {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        UnstickLeg {
            to: [0.0; 3],
            best: 0.0,
            stall_t: 0.0,
        }
    }
}
impl Default for UnstickLeg { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SearchSpot, src/game/guards.ts:1788)
#[derive(Clone, Debug)]
pub struct SearchSpot {
    pub pos: V3,
    pub cell: f64,
    pub key: String,
    pub kind: crate::guards::SearchSpotKind,
    pub hidden: bool,
    pub cover: f64,
    pub dark: f64,
    pub e: Option<f64>,
    pub need: f64,
    pub plaus: f64,
    pub score: f64,
    pub visited: bool,
    pub by: f64,
    pub unreachable: bool,
    pub samples: Vec<crate::guards::SearchSpotSamplesItem>,
}
impl SearchSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SearchSpot {
            pos: [0.0; 3],
            cell: 0.0,
            key: String::new(),
            kind: crate::guards::SearchSpotKind::Wall,
            hidden: false,
            cover: 0.0,
            dark: 0.0,
            e: None,
            need: 0.0,
            plaus: 0.0,
            score: 0.0,
            visited: false,
            by: 0.0,
            unreachable: false,
            samples: Vec::new(),
        }
    }
}
impl Default for SearchSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SearchSpotKind {
    Wall,
    Prop,
    Open,
    Corner,
    Leaf,
}
impl SearchSpotKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            SearchSpotKind::Wall => "wall",
            SearchSpotKind::Prop => "prop",
            SearchSpotKind::Open => "open",
            SearchSpotKind::Corner => "corner",
            SearchSpotKind::Leaf => "leaf",
        }
    }
}

/// (TypeScript: inline at src/game/guards.ts:1802 SearchSpot.samples, src/game/guards.ts:1802)
#[derive(Clone, Copy, Debug)]
pub struct SearchSpotSamplesItem {
    pub t: f64,
    pub e: f64,
}
impl SearchSpotSamplesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SearchSpotSamplesItem {
            t: 0.0,
            e: 0.0,
        }
    }
}
impl Default for SearchSpotSamplesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SearchPlan, src/game/guards.ts:1804)
#[derive(Clone, Debug)]
pub struct SearchPlan {
    pub fix: V3,
    pub t0: f64,
    pub serial: f64,
    pub built_at: f64,
    pub flood_at: f64,
    pub spots: Vec<crate::guards::SearchSpot>,
    pub current: f64,
    pub phase: crate::guards::SearchPlanPhase,
    pub phase_t: f64,
    pub visits: f64,
    pub rr: f64,
    pub said: bool,
}
impl SearchPlan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SearchPlan {
            fix: [0.0; 3],
            t0: 0.0,
            serial: 0.0,
            built_at: 0.0,
            flood_at: 0.0,
            spots: Vec::new(),
            current: 0.0,
            phase: crate::guards::SearchPlanPhase::Done,
            phase_t: 0.0,
            visits: 0.0,
            rr: 0.0,
            said: false,
        }
    }
}
impl Default for SearchPlan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SearchPlanPhase {
    Done,
    Look,
    Walk,
    Sweep,
}
impl SearchPlanPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            SearchPlanPhase::Done => "done",
            SearchPlanPhase::Look => "look",
            SearchPlanPhase::Walk => "walk",
            SearchPlanPhase::Sweep => "sweep",
        }
    }
}

/// (TypeScript: Pieing, src/game/guards.ts:2005)
#[derive(Clone, Copy, Debug)]
pub struct Pieing {
    pub door: DoorId,
    pub phase: crate::guards::PieingPhase,
    pub side: f64,
    pub at: V3,
    pub near: f64,
    pub t: f64,
    pub t0: f64,
    pub path_ref: u32,
    pub bent: bool,
}
impl Pieing {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Pieing {
            door: DoorId(0),
            phase: crate::guards::PieingPhase::Slice,
            side: 0.0,
            at: [0.0; 3],
            near: 0.0,
            t: 0.0,
            t0: 0.0,
            path_ref: 0,
            bent: false,
        }
    }
}
impl Default for Pieing { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PieingPhase {
    Slice,
    Approach,
    Cross,
}
impl PieingPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            PieingPhase::Slice => "slice",
            PieingPhase::Approach => "approach",
            PieingPhase::Cross => "cross",
        }
    }
}

/// (TypeScript: Rollcall, src/game/guards.ts:3073)
#[derive(Clone, Copy, Debug)]
pub struct Rollcall {
    pub next_at: f64,
    pub stage: crate::guards::RollcallStage,
    pub t0: f64,
    pub caller: Option<GuardId>,
    pub missing: Option<GuardId>,
    pub answered: bool,
    pub checks: f64,
}
impl Rollcall {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Rollcall {
            next_at: 0.0,
            stage: crate::guards::RollcallStage::Idle,
            t0: 0.0,
            caller: None,
            missing: None,
            answered: false,
            checks: 0.0,
        }
    }
}
impl Default for Rollcall { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RollcallStage {
    Idle,
    Called,
    Asked,
}
impl RollcallStage {
    pub const fn as_str(self) -> &'static str {
        match self {
            RollcallStage::Idle => "idle",
            RollcallStage::Called => "called",
            RollcallStage::Asked => "asked",
        }
    }
}

/// (TypeScript: UnitOpts, src/game/guards.ts:3498)
#[derive(Clone, Debug)]
pub struct UnitOpts {
    pub team: crate::teams::Team,
    pub pos: V3,
    pub yaw: Option<f64>,
    pub weapon: Option<crate::weapons::WeaponId>,
    pub tint: Option<V3>,
    pub tint2: Option<V3>,
    pub carbine_rig: Option<bool>,
    pub name: Option<String>,
    pub light: Option<bool>,
    pub pair: Option<crate::game::PairId>,
    pub budget: Option<f64>,
}
impl UnitOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        UnitOpts {
            team: crate::teams::Team::Guard,
            pos: [0.0; 3],
            yaw: None,
            weapon: None,
            tint: None,
            tint2: None,
            carbine_rig: None,
            name: None,
            light: None,
            pair: None,
            budget: None,
        }
    }
}
impl Default for UnitOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:267 groundTrack, src/game/guards.ts:267)
#[derive(Clone, Copy, Debug)]
pub struct GuardsModGroundTrackValue {
    pub pos: V3,
    pub t: f64,
    pub vel: V3,
}
impl GuardsModGroundTrackValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardsModGroundTrackValue {
            pos: [0.0; 3],
            t: 0.0,
            vel: [0.0; 3],
        }
    }
}
impl Default for GuardsModGroundTrackValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:365 BODY_LIGHT_DEFAULTS, src/game/guards.ts:365)
#[derive(Clone, Copy, Debug)]
pub struct BODYLIGHTDEFAULTS {
    pub lit: f64,
    pub lit_near: f64,
    pub near_m: f64,
    pub far_m: f64,
    pub smoke: f64,
}
impl BODYLIGHTDEFAULTS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BODYLIGHTDEFAULTS {
            lit: 0.0,
            lit_near: 0.0,
            near_m: 0.0,
            far_m: 0.0,
            smoke: 0.0,
        }
    }
}
impl Default for BODYLIGHTDEFAULTS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:367 bodyMeters, src/game/guards.ts:367)
#[derive(Clone, Copy, Debug)]
pub struct GuardsModBodyMetersValue {
    pub vis: f64,
    pub e: f64,
    pub t: f64,
}
impl GuardsModBodyMetersValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardsModBodyMetersValue {
            vis: 0.0,
            e: 0.0,
            t: 0.0,
        }
    }
}
impl Default for GuardsModBodyMetersValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:372 bodyProbeN, src/game/guards.ts:372)
#[derive(Clone, Copy, Debug)]
pub struct GuardsModBodyProbeNValue {
    pub t: f64,
    pub n: f64,
}
impl GuardsModBodyProbeNValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardsModBodyProbeNValue {
            t: 0.0,
            n: 0.0,
        }
    }
}
impl Default for GuardsModBodyProbeNValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:671 LINE_DEBUG, src/game/guards.ts:671)
#[derive(Clone, Copy, Debug)]
pub struct LINEDEBUG {
}
impl LINEDEBUG {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LINEDEBUG {
        }
    }
}
impl Default for LINEDEBUG { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:853 STANDOFF_DEBUG, src/game/guards.ts:853)
#[derive(Clone, Copy, Debug)]
pub struct STANDOFFDEBUG {
}
impl STANDOFFDEBUG {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        STANDOFFDEBUG {
        }
    }
}
impl Default for STANDOFFDEBUG { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:911 HELD_METER, src/game/guards.ts:911)
#[derive(Clone, Copy, Debug)]
pub struct HELDMETER {
    pub gap_secs: f64,
    pub moved_m: f64,
}
impl HELDMETER {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HELDMETER {
            gap_secs: 0.0,
            moved_m: 0.0,
        }
    }
}
impl Default for HELDMETER { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:912 heldMeters, src/game/guards.ts:912)
#[derive(Clone, Copy, Debug)]
pub struct GuardsModHeldMetersValue {
    pub vis: f64,
    pub t: f64,
    pub at: V3,
}
impl GuardsModHeldMetersValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardsModHeldMetersValue {
            vis: 0.0,
            t: 0.0,
            at: [0.0; 3],
        }
    }
}
impl Default for GuardsModHeldMetersValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:1000 STANDOFF_LINES, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct STANDOFFLINESValue {
    pub steady: Option<Vec<String>>,
    pub nervy: Option<Vec<String>>,
    pub hard: Option<Vec<String>>,
    pub under: Option<Vec<String>>,
}
impl STANDOFFLINESValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        STANDOFFLINESValue {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
        }
    }
}
impl Default for STANDOFFLINESValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:1017 STANDOFF_NAMELESS, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct STANDOFFNAMELESS {
    pub lost: Option<String>,
    pub enter: Option<String>,
    pub line: Option<String>,
    pub cornered: Option<String>,
    pub no_shot: Option<String>,
    pub dropped: Option<String>,
    pub freed: Option<String>,
    pub hit_him: Option<String>,
}
impl STANDOFFNAMELESS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        STANDOFFNAMELESS {
            lost: None,
            enter: None,
            line: None,
            cornered: None,
            no_shot: None,
            dropped: None,
            freed: None,
            hit_him: None,
        }
    }
}
impl Default for STANDOFFNAMELESS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:1383 UNSTICK, src/game/guards.ts:1383)
#[derive(Clone, Copy, Debug)]
pub struct GuardsModUNSTICK {
    pub legs: f64,
    pub snaps: f64,
    pub last_snap_t: f64,
    pub last_snap_who: Option<GuardId>,
}
impl GuardsModUNSTICK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardsModUNSTICK {
            legs: 0.0,
            snaps: 0.0,
            last_snap_t: 0.0,
            last_snap_who: None,
        }
    }
}
impl Default for GuardsModUNSTICK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:1557 litSeen, src/game/guards.ts:1557)
#[derive(Clone, Debug)]
pub struct GuardsModLitSeenValue {
    pub t: f64,
    pub lit: bool,
    pub a: Option<OrdersAssignmentId>,
    pub tgt: Option<crate::guards::LitSeenTgt>,
}
impl GuardsModLitSeenValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardsModLitSeenValue {
            t: 0.0,
            lit: false,
            a: None,
            tgt: None,
        }
    }
}
impl Default for GuardsModLitSeenValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:1575 darkSeen, src/game/guards.ts:1575)
#[derive(Clone, Copy, Debug)]
pub struct GuardsModDarkSeenValue {
    pub t: f64,
    pub dark: bool,
}
impl GuardsModDarkSeenValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardsModDarkSeenValue {
            t: 0.0,
            dark: false,
        }
    }
}
impl Default for GuardsModDarkSeenValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:1591 wallSeen, src/game/guards.ts:1591)
#[derive(Clone, Copy, Debug)]
pub struct GuardsModWallSeenValue {
    pub t: f64,
    pub since: f64,
    pub dark: bool,
    pub out: Option<bool>,
}
impl GuardsModWallSeenValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardsModWallSeenValue {
            t: 0.0,
            since: 0.0,
            dark: false,
            out: None,
        }
    }
}
impl Default for GuardsModWallSeenValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:1654 torchLowSeen, src/game/guards.ts:1654)
#[derive(Clone, Copy, Debug)]
pub struct GuardsModTorchLowSeenValue {
    pub t: f64,
    pub low: bool,
}
impl GuardsModTorchLowSeenValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardsModTorchLowSeenValue {
            t: 0.0,
            low: false,
        }
    }
}
impl Default for GuardsModTorchLowSeenValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:1677 litManSeen, src/game/guards.ts:1677)
#[derive(Clone, Copy, Debug)]
pub struct GuardsModLitManSeenValue {
    pub t: f64,
    pub tgt: GuardId,
    pub v: bool,
}
impl GuardsModLitManSeenValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardsModLitManSeenValue {
            t: 0.0,
            tgt: GuardId(0),
            v: false,
        }
    }
}
impl Default for GuardsModLitManSeenValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:2612 NO_SIGHT, src/game/guards.ts:2612)
#[derive(Clone, Copy, Debug)]
pub struct NOSIGHT {
    pub visible: bool,
    pub dist: f64,
    pub in_cone: bool,
}
impl NOSIGHT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NOSIGHT {
            visible: false,
            dist: 0.0,
            in_cone: false,
        }
    }
}
impl Default for NOSIGHT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:3630 BREACH_HOOKS, src/game/guards.ts:3630)
#[derive(Clone, Copy, Debug)]
pub struct BREACHHOOKS {
}
impl BREACHHOOKS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BREACHHOOKS {
        }
    }
}
impl Default for BREACHHOOKS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:219 shotRelation(e), src/game/guards.ts:219)
#[derive(Clone, Debug)]
pub struct ShotRelationE {
    pub kind: String,
    pub who: Option<f64>,
}
impl ShotRelationE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShotRelationE {
            kind: String::new(),
            who: None,
        }
    }
}
impl Default for ShotRelationE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ShotRelation {
    None,
    Own,
    Threat,
}
impl ShotRelation {
    pub const fn as_str(self) -> &'static str {
        match self {
            ShotRelation::None => "none",
            ShotRelation::Own => "own",
            ShotRelation::Threat => "threat",
        }
    }
}

/// (TypeScript: inline at src/game/guards.ts:226 colleagueFirePos(e), src/game/guards.ts:226)
#[derive(Clone, Copy, Debug)]
pub struct ColleagueFirePosE {
    pub who: Option<f64>,
    pub pos: V3,
    pub to: Option<V3>,
    pub blind: Option<bool>,
}
impl ColleagueFirePosE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ColleagueFirePosE {
            who: None,
            pos: [0.0; 3],
            to: None,
            blind: None,
        }
    }
}
impl Default for ColleagueFirePosE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:249 stepIsThreat(e), src/game/guards.ts:249)
#[derive(Clone, Copy, Debug)]
pub struct StepIsThreatE {
    pub who: Option<f64>,
}
impl StepIsThreatE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StepIsThreatE {
            who: None,
        }
    }
}
impl Default for StepIsThreatE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:401 loadBodyLight(tune), src/game/guards.ts:401)
#[derive(Clone, Copy, Debug)]
pub struct LoadBodyLightTune {
    pub body_light: bool,
}
impl LoadBodyLightTune {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LoadBodyLightTune {
            body_light: false,
        }
    }
}
impl Default for LoadBodyLightTune { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:408 saveBodyLight(tune), src/game/guards.ts:408)
#[derive(Clone, Copy, Debug)]
pub struct SaveBodyLightTune {
    pub body_light: bool,
}
impl SaveBodyLightTune {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SaveBodyLightTune {
            body_light: false,
        }
    }
}
impl Default for SaveBodyLightTune { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:409 resetBodyLight(tune), src/game/guards.ts:409)
#[derive(Clone, Copy, Debug)]
pub struct ResetBodyLightTune {
    pub body_light: bool,
}
impl ResetBodyLightTune {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ResetBodyLightTune {
            body_light: false,
        }
    }
}
impl Default for ResetBodyLightTune { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:431 lightOutSeen(e), src/game/guards.ts:431)
#[derive(Clone, Copy, Debug)]
pub struct LightOutSeenE {
    pub pos: V3,
    pub from: Option<V3>,
}
impl LightOutSeenE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LightOutSeenE {
            pos: [0.0; 3],
            from: None,
        }
    }
}
impl Default for LightOutSeenE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AlertCause {
    Body,
    Radio,
    Sound,
    Sight,
}
impl AlertCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            AlertCause::Body => "body",
            AlertCause::Radio => "radio",
            AlertCause::Sound => "sound",
            AlertCause::Sight => "sight",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorTamper {
    Picked,
    Forced,
}
impl DoorTamper {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorTamper::Picked => "picked",
            DoorTamper::Forced => "forced",
        }
    }
}

/// (TypeScript: inline at src/game/guards.ts:551 bodyAftermath ->, src/game/guards.ts:551)
#[derive(Clone, Copy, Debug)]
pub struct BodyAftermath {
    pub move_: f64,
    pub aim: Option<V3>,
}
impl BodyAftermath {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BodyAftermath {
            move_: 0.0,
            aim: None,
        }
    }
}
impl Default for BodyAftermath { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:677 lineGeom ->, src/game/guards.ts:677)
#[derive(Clone, Copy, Debug)]
pub struct LineGeom {
    pub t: f64,
    pub e: f64,
    pub d: f64,
    pub clear: f64,
}
impl LineGeom {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LineGeom {
            t: 0.0,
            e: 0.0,
            d: 0.0,
            clear: 0.0,
        }
    }
}
impl Default for LineGeom { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LineStep {
    Clear,
    None,
    Step,
}
impl LineStep {
    pub const fn as_str(self) -> &'static str {
        match self {
            LineStep::Clear => "clear",
            LineStep::None => "none",
            LineStep::Step => "step",
        }
    }
}

/// (TypeScript: inline at src/game/guards.ts:988 flankShot ->, src/game/guards.ts:988)
#[derive(Clone, Copy, Debug)]
pub struct FlankShot {
    pub ok: bool,
    pub rear: f64,
    pub clear: f64,
    pub dist: f64,
}
impl FlankShot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FlankShot {
            ok: false,
            rear: 0.0,
            clear: 0.0,
            dist: 0.0,
        }
    }
}
impl Default for FlankShot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum StandoffBarkForce {
    Bool(bool),
    Always,
}

/// (TypeScript: inline at src/game/guards.ts:1136 standoffAlert ->, src/game/guards.ts:1136)
#[derive(Clone, Copy, Debug)]
pub struct StandoffAlert {
    pub move_: f64,
    pub aim: V3,
    pub crouch: f64,
}
impl StandoffAlert {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StandoffAlert {
            move_: 0.0,
            aim: [0.0; 3],
            crouch: 0.0,
        }
    }
}
impl Default for StandoffAlert { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:1239 canSee ->, src/game/guards.ts:1239)
#[derive(Clone, Copy, Debug)]
pub struct CanSee {
    pub visible: bool,
    pub dist: f64,
    pub in_cone: bool,
}
impl CanSee {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CanSee {
            visible: false,
            dist: 0.0,
            in_cone: false,
        }
    }
}
impl Default for CanSee { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:1245 canSeeChar ->, src/game/guards.ts:1245)
#[derive(Clone, Copy, Debug)]
pub struct CanSeeChar {
    pub visible: bool,
    pub dist: f64,
    pub in_cone: bool,
}
impl CanSeeChar {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CanSeeChar {
            visible: false,
            dist: 0.0,
            in_cone: false,
        }
    }
}
impl Default for CanSeeChar { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:1327 eyeContact ->, src/game/guards.ts:1327)
#[derive(Clone, Copy, Debug)]
pub struct EyeContact {
    pub visible: bool,
    pub dist: f64,
    pub in_cone: bool,
}
impl EyeContact {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EyeContact {
            visible: false,
            dist: 0.0,
            in_cone: false,
        }
    }
}
impl Default for EyeContact { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:1342 eyeContactHere ->, src/game/guards.ts:1342)
#[derive(Clone, Copy, Debug)]
pub struct EyeContactHere {
    pub visible: bool,
    pub dist: f64,
    pub in_cone: bool,
}
impl EyeContactHere {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EyeContactHere {
            visible: false,
            dist: 0.0,
            in_cone: false,
        }
    }
}
impl Default for EyeContactHere { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FreeCellNearTest {
    Room,
    Walk,
}
impl FreeCellNearTest {
    pub const fn as_str(self) -> &'static str {
        match self {
            FreeCellNearTest::Room => "room",
            FreeCellNearTest::Walk => "walk",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum UnstickBegin {
    Leg,
    Snapped,
    No,
}
impl UnstickBegin {
    pub const fn as_str(self) -> &'static str {
        match self {
            UnstickBegin::Leg => "leg",
            UnstickBegin::Snapped => "snapped",
            UnstickBegin::No => "no",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum UnstickStep {
    Done,
    Stale,
    Walking,
}
impl UnstickStep {
    pub const fn as_str(self) -> &'static str {
        match self {
            UnstickStep::Done => "done",
            UnstickStep::Stale => "stale",
            UnstickStep::Walking => "walking",
        }
    }
}

/// (TypeScript: inline at src/game/guards.ts:1830 floodFrom ->, src/game/guards.ts:1830)
#[derive(Clone, Debug)]
pub struct FloodFrom {
    pub secs: Vec<f32>,
    pub metres: Vec<f32>,
}
impl FloodFrom {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FloodFrom {
            secs: Vec::new(),
            metres: Vec::new(),
        }
    }
}
impl Default for FloodFrom { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:2015 crossesDoor ->, src/game/guards.ts:2015)
#[derive(Clone, Copy, Debug)]
pub struct CrossesDoor {
    pub t: f64,
    pub side: f64,
    pub at: V3,
}
impl CrossesDoor {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CrossesDoor {
            t: 0.0,
            side: 0.0,
            at: [0.0; 3],
        }
    }
}
impl Default for CrossesDoor { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:2032 pieDoorway ->, src/game/guards.ts:2032)
#[derive(Clone, Copy, Debug)]
pub struct PieDoorway {
    pub speed: f64,
    pub look: Option<V3>,
}
impl PieDoorway {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PieDoorway {
            speed: 0.0,
            look: None,
        }
    }
}
impl Default for PieDoorway { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:2099 chaseFix ->, src/game/guards.ts:2099)
#[derive(Clone, Copy, Debug)]
pub struct ChaseFix {
    pub move_: f64,
    pub aim: Option<V3>,
    pub pied: bool,
    pub crouch: f64,
}
impl ChaseFix {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChaseFix {
            move_: 0.0,
            aim: None,
            pied: false,
            crouch: 0.0,
        }
    }
}
impl Default for ChaseFix { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RigOf {
    Pistol,
    Carbine,
    Sc20k,
}
impl RigOf {
    pub const fn as_str(self) -> &'static str {
        match self {
            RigOf::Pistol => "pistol",
            RigOf::Carbine => "carbine",
            RigOf::Sc20k => "sc20k",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LongRigOf {
    Carbine,
    Sc20k,
}
impl LongRigOf {
    pub const fn as_str(self) -> &'static str {
        match self {
            LongRigOf::Carbine => "carbine",
            LongRigOf::Sc20k => "sc20k",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HandCarry {
    Torch,
    Weapon,
}
impl HandCarry {
    pub const fn as_str(self) -> &'static str {
        match self {
            HandCarry::Torch => "torch",
            HandCarry::Weapon => "weapon",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum InHand {
    Torch,
    Weapon,
}
impl InHand {
    pub const fn as_str(self) -> &'static str {
        match self {
            InHand::Torch => "torch",
            InHand::Weapon => "weapon",
        }
    }
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum InViewVp {
    Array(Vec<f64>),
    Float32Array(Vec<f32>),
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RaiseEscalationCause {
    Search,
    Civilian,
    Device,
    Missing,
    Body,
    Hostage,
}
impl RaiseEscalationCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            RaiseEscalationCause::Search => "search",
            RaiseEscalationCause::Civilian => "civilian",
            RaiseEscalationCause::Device => "device",
            RaiseEscalationCause::Missing => "missing",
            RaiseEscalationCause::Body => "body",
            RaiseEscalationCause::Hostage => "hostage",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PlanClearsCause {
    Search,
    Missing,
    Body,
    Panel,
}
impl PlanClearsCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            PlanClearsCause::Search => "search",
            PlanClearsCause::Missing => "missing",
            PlanClearsCause::Body => "body",
            PlanClearsCause::Panel => "panel",
        }
    }
}

/// (TypeScript: inline at src/game/guards.ts:3595 engage(opts), src/game/guards.ts:3595)
#[derive(Clone, Copy, Debug)]
pub struct EngageOpts {
    pub timeout: Option<f64>,
    pub pin: Option<bool>,
    pub surprise: Option<f64>,
}
impl EngageOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EngageOpts {
            timeout: None,
            pin: None,
            surprise: None,
        }
    }
}
impl Default for EngageOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum EndEngagementOutcome {
    Wiped,
    Reset,
    Cleared,
    Timeout,
    Superseded,
}
impl EndEngagementOutcome {
    pub const fn as_str(self) -> &'static str {
        match self {
            EndEngagementOutcome::Wiped => "wiped",
            EndEngagementOutcome::Reset => "reset",
            EndEngagementOutcome::Cleared => "cleared",
            EndEngagementOutcome::Timeout => "timeout",
            EndEngagementOutcome::Superseded => "superseded",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct GuardsMod {
    /// (src/game/guards.ts:267 groundTrack)
    pub ground_track: JsMap<PlayerId, crate::guards::GuardsModGroundTrackValue>,
    /// (src/game/guards.ts:367 bodyMeters)
    pub body_meters: JsMap<GuardId, crate::guards::GuardsModBodyMetersValue>,
    /// (src/game/guards.ts:372 bodyProbeN)
    pub body_probe_n: Option<crate::guards::GuardsModBodyProbeNValue>,
    /// (src/game/guards.ts:871 holdsCalled)
    pub holds_called: JsSet<u32>,
    /// (src/game/guards.ts:871 floorBarkT)
    pub floor_bark_t: Option<f64>,
    /// (src/game/guards.ts:871 lastBark)
    pub last_bark: JsMap<GuardId, String>,
    /// (src/game/guards.ts:912 heldMeters)
    pub held_meters: JsMap<u32, crate::guards::GuardsModHeldMetersValue>,
    /// (src/game/guards.ts:1376 unstickLegs)
    pub unstick_legs: JsMap<GuardId, crate::guards::UnstickLeg>,
    /// (src/game/guards.ts:1378 unstickRetryAt)
    pub unstick_retry_at: JsMap<GuardId, f64>,
    /// (src/game/guards.ts:1380 heldLastFrame)
    pub held_last_frame: JsSet<GuardId>,
    /// (src/game/guards.ts:1383 UNSTICK)
    pub unstick: crate::guards::GuardsModUNSTICK,
    /// (src/game/guards.ts:1557 litSeen)
    pub lit_seen: JsMap<GuardId, crate::guards::GuardsModLitSeenValue>,
    /// (src/game/guards.ts:1575 darkSeen)
    pub dark_seen: JsMap<GuardId, crate::guards::GuardsModDarkSeenValue>,
    /// (src/game/guards.ts:1589 litUntil)
    pub lit_until: JsMap<GuardId, f64>,
    /// (src/game/guards.ts:1591 wallSeen)
    pub wall_seen: JsMap<GuardId, crate::guards::GuardsModWallSeenValue>,
    /// (src/game/guards.ts:1654 torchLowSeen)
    pub torch_low_seen: JsMap<GuardId, crate::guards::GuardsModTorchLowSeenValue>,
    /// (src/game/guards.ts:1677 litManSeen)
    pub lit_man_seen: JsMap<GuardId, crate::guards::GuardsModLitManSeenValue>,
    /// (src/game/guards.ts:1813 doorCellsOf)
    pub door_cells_of: Option<JsMap<crate::Num, DoorId>>,
    /// (src/game/guards.ts:3074 rollcalls)
    pub rollcalls: Option<crate::guards::Rollcall>,
    /// (src/game/guards.ts:3511 unitSerial)
    pub unit_serial: f64,
    /// (src/game/guards.ts:3574 engagements)
    pub engagements: Option<crate::guards::Engagement>,
}
impl GuardsMod {
    pub fn blank() -> Self {
        GuardsMod {
            ground_track: JsMap::new(),
            body_meters: JsMap::new(),
            body_probe_n: None,
            holds_called: JsSet::new(),
            floor_bark_t: None,
            last_bark: JsMap::new(),
            held_meters: JsMap::new(),
            unstick_legs: JsMap::new(),
            unstick_retry_at: JsMap::new(),
            held_last_frame: JsSet::new(),
            unstick: crate::guards::GuardsModUNSTICK { legs: 0.0, snaps: 0.0, last_snap_t: -1.0, last_snap_who: None },
            lit_seen: JsMap::new(),
            dark_seen: JsMap::new(),
            lit_until: JsMap::new(),
            wall_seen: JsMap::new(),
            torch_low_seen: JsMap::new(),
            lit_man_seen: JsMap::new(),
            door_cells_of: None,
            rollcalls: None,
            unit_serial: 0.0,
            engagements: None,
        }
    }
}

/// (src/game/guards.ts:59 OCP_MUTTER: a table nothing writes)
pub fn ocp_mutter() -> Vec<String> { alloc::vec![String::from("light's on the blink again"), String::from("hm — bulb's going"), String::from("power dip?"), String::from("these fittings…"), String::from("that light again")] }

/// (src/game/guards.ts:258 MOTION_STILL)
pub const MOTION_STILL: f64 = 0.35;

/// (src/game/guards.ts:258 MOTION_LO)
pub const MOTION_LO: f64 = 0.05;

/// (src/game/guards.ts:258 MOTION_HI)
pub const MOTION_HI: f64 = 1.2;

/// (src/game/guards.ts:258 MOTION_LIT_LO)
pub const MOTION_LIT_LO: f64 = 0.3;

/// (src/game/guards.ts:258 MOTION_LIT_HI)
pub const MOTION_LIT_HI: f64 = 0.75;

/// (src/game/guards.ts:260 MOTION_RADIAL)
pub const MOTION_RADIAL: f64 = 0.4;

/// (src/game/guards.ts:263 MOTION_SPRINT)
pub const MOTION_SPRINT: f64 = 1.6;

/// (src/game/guards.ts:263 MOTION_SPRINT_MS)
pub const MOTION_SPRINT_MS: f64 = 3.5;

/// (src/game/guards.ts:263 MOTION_NEAR_M)
pub const MOTION_NEAR_M: f64 = 2.5;

/// (src/game/guards.ts:263 MOTION_NEAR_FLOOR)
pub const MOTION_NEAR_FLOOR: f64 = 0.8;

/// (src/game/guards.ts:263 MOTION_HUNT_FLOOR)
pub const MOTION_HUNT_FLOOR: f64 = 0.7;

/// (src/game/guards.ts:365 BODY_LIGHT_DEFAULTS: a table nothing writes)
pub fn body_light_defaults() -> crate::guards::BODYLIGHTDEFAULTS { crate::guards::BODYLIGHTDEFAULTS { lit: 0.22, lit_near: 0.04, near_m: 2.0, far_m: 5.0, smoke: 0.3 } }

/// (src/game/guards.ts:371 BODY_PROBE_BUDGET)
pub const BODY_PROBE_BUDGET: f64 = 6.0;

/// (src/game/guards.ts:400 BODY_LIGHT_LS)
pub const BODY_LIGHT_LS: &str = "twopointsix.bodyLight";

/// (src/game/guards.ts:490 BODY_DUTY_CAP)
pub const BODY_DUTY_CAP: f64 = 45.0;

/// (src/game/guards.ts:658 MUZZLE_CLEAR_M)
pub const MUZZLE_CLEAR_M: f64 = 0.35;

/// (src/game/guards.ts:658 MUZZLE_LEAD_SECS)
pub const MUZZLE_LEAD_SECS: f64 = 0.3;

/// (src/game/guards.ts:658 MUZZLE_LEAD_MAX_M)
pub const MUZZLE_LEAD_MAX_M: f64 = 0.4;

/// (src/game/guards.ts:658 MUZZLE_PAST_M)
pub const MUZZLE_PAST_M: f64 = 1.0;

/// (src/game/guards.ts:658 MUZZLE_REACH_M)
pub const MUZZLE_REACH_M: f64 = 8.0;

/// (src/game/guards.ts:658 MUZZLE_UP_SECS)
pub const MUZZLE_UP_SECS: f64 = 0.2;

/// (src/game/guards.ts:661 LINE_STEP_MARGIN_M)
pub const LINE_STEP_MARGIN_M: f64 = 0.12;

/// (src/game/guards.ts:661 LINE_STEP_MAX_M)
pub const LINE_STEP_MAX_M: f64 = 2.2;

/// (src/game/guards.ts:661 LINE_PUSH_MIN_M)
pub const LINE_PUSH_MIN_M: f64 = 2.5;

/// (src/game/guards.ts:661 LINE_BARK_SECS)
pub const LINE_BARK_SECS: f64 = 10.0;

/// (src/game/guards.ts:665 LINE_STUCK_SECS)
pub const LINE_STUCK_SECS: f64 = 0.4;

/// (src/game/guards.ts:665 LINE_FAIL_SECS)
pub const LINE_FAIL_SECS: f64 = 5.0;

/// (src/game/guards.ts:665 LINE_FAIL_M)
pub const LINE_FAIL_M: f64 = 0.75;

/// (src/game/guards.ts:668 LINE_WALK_CLEAR_M)
pub const LINE_WALK_CLEAR_M: f64 = 0.5;

/// (src/game/guards.ts:671 LINE_DEBUG: a table nothing writes)
pub fn line_debug() -> crate::guards::LINEDEBUG { crate::guards::LINEDEBUG {  } }

/// (src/game/guards.ts:823 STANDOFF_MIN_M)
pub const STANDOFF_MIN_M: f64 = 3.5;

/// (src/game/guards.ts:823 STANDOFF_MAX_M)
pub const STANDOFF_MAX_M: f64 = 7.0;

/// (src/game/guards.ts:823 STANDOFF_ORBIT_M)
pub const STANDOFF_ORBIT_M: f64 = 4.5;

/// (src/game/guards.ts:823 STANDOFF_SETTLE_DEG)
pub const STANDOFF_SETTLE_DEG: f64 = 22.0;

/// (src/game/guards.ts:823 STANDOFF_FIRE_MAX_M)
pub const STANDOFF_FIRE_MAX_M: f64 = 5.0;

/// (src/game/guards.ts:828 STANDOFF_REAR_DEG)
pub const STANDOFF_REAR_DEG: f64 = 70.0;

/// (src/game/guards.ts:828 STANDOFF_CLEAR_M)
pub const STANDOFF_CLEAR_M: f64 = 0.45;

/// (src/game/guards.ts:828 STANDOFF_SHOT_SECS)
pub const STANDOFF_SHOT_SECS: f64 = 1.2;

/// (src/game/guards.ts:828 STANDOFF_AIM_Y)
pub const STANDOFF_AIM_Y: f64 = 1.42;

/// (src/game/guards.ts:830 SAM_R)
pub const SAM_R: f64 = 0.28;

/// (src/game/guards.ts:834 STANDOFF_VIEW_M)
pub const STANDOFF_VIEW_M: f64 = 16.0;

/// (src/game/guards.ts:834 STANDOFF_VIEW_DEG)
pub const STANDOFF_VIEW_DEG: f64 = 60.0;

/// (src/game/guards.ts:834 STANDOFF_LOSE_SECS)
pub const STANDOFF_LOSE_SECS: f64 = 4.0;

/// (src/game/guards.ts:834 STANDOFF_REPLAN_SECS)
pub const STANDOFF_REPLAN_SECS: f64 = 1.0;

/// (src/game/guards.ts:834 STANDOFF_STALL_SECS)
pub const STANDOFF_STALL_SECS: f64 = 2.5;

/// (src/game/guards.ts:834 STANDOFF_REST_SECS)
pub const STANDOFF_REST_SECS: f64 = 1.5;

/// (src/game/guards.ts:836 STANDOFF_BARK_SECS)
pub const STANDOFF_BARK_SECS: f64 = 6.0;

/// (src/game/guards.ts:836 STANDOFF_BARK_FLOOR_SECS)
pub const STANDOFF_BARK_FLOOR_SECS: f64 = 3.0;

/// (src/game/guards.ts:843 SHIELD_RELEASE_BEAT_MIN)
pub const SHIELD_RELEASE_BEAT_MIN: f64 = 0.35;

/// (src/game/guards.ts:843 SHIELD_RELEASE_BEAT_JITTER)
pub const SHIELD_RELEASE_BEAT_JITTER: f64 = 0.3;

/// (src/game/guards.ts:853 STANDOFF_DEBUG: a table nothing writes)
pub fn standoff_debug() -> crate::guards::STANDOFFDEBUG { crate::guards::STANDOFFDEBUG {  } }

/// (src/game/guards.ts:897 PAIR_LEGIBLE)
pub const PAIR_LEGIBLE: f64 = 3.0;

/// (src/game/guards.ts:911 HELD_METER: a table nothing writes)
pub fn held_meter() -> crate::guards::HELDMETER { crate::guards::HELDMETER { gap_secs: 0.5, moved_m: 1.0 } }

/// (src/game/guards.ts:1000 STANDOFF_LINES: a table nothing writes)
pub fn standoff_lines() -> JsMap<String, crate::guards::STANDOFFLINESValue> { { let mut t = JsMap::new(); t.set(String::from("enter"), crate::guards::STANDOFFLINESValue { steady: Some(alloc::vec![String::from("{held}, hold still — we've got you"), String::from("easy. easy. nobody has to get hurt here")]), nervy: Some(alloc::vec![String::from("{held}! —jesus— okay, okay, nobody shoot!"), String::from("oh god— he's got {held}—")]), hard: Some(alloc::vec![String::from("let {him} go. now."), String::from("you. hands off {him}.")]), under: None }); t.set(String::from("noShot"), crate::guards::STANDOFFLINESValue { steady: Some(alloc::vec![String::from("let {him} go and we talk about it"), String::from("{held}, stay loose — I'm coming round")]), nervy: Some(alloc::vec![String::from("{held}! —I can't— I haven't got a line—"), String::from("don't you hurt {him}— don't—")]), hard: Some(alloc::vec![String::from("you've got nowhere to take {him}"), String::from("the second {he} slips, you're done")]), under: Some(alloc::vec![String::from("let {him} go!"), String::from("drop {him} — NOW"), String::from("{held}, don't move"), String::from("I don't have a shot"), String::from("no shot — {held}'s square in front of him")]) }); t.set(String::from("line"), crate::guards::STANDOFFLINESValue { steady: None, nervy: Some(alloc::vec![String::from("I— I think I have him—")]), hard: Some(alloc::vec![String::from("there you are.")]), under: Some(alloc::vec![String::from("I have him."), String::from("got a line —")]) }); t.set(String::from("cornered"), crate::guards::STANDOFFLINESValue { steady: None, nervy: Some(alloc::vec![String::from("just— just let {him} go, please—")]), hard: Some(alloc::vec![String::from("one way out of this for you. through us.")]), under: Some(alloc::vec![String::from("can't get round him — hold what you've got"), String::from("no way round. {held}, hang on"), String::from("nowhere for either of us to go — let {him} walk and this ends quiet")]) }); t.set(String::from("dropped"), crate::guards::STANDOFFLINESValue { steady: None, nervy: Some(alloc::vec![String::from("{held}! —he's— shoot, SHOOT—")]), hard: None, under: Some(alloc::vec![String::from("he's dropped {held} — TAKE HIM"), String::from("{held}'s down — light him up!")]) }); t.set(String::from("freed"), crate::guards::STANDOFFLINESValue { steady: None, nervy: None, hard: None, under: Some(alloc::vec![String::from("{held}, DOWN — get clear!"), String::from("he's loose — {held}, out of the way!")]) }); t.set(String::from("lost"), crate::guards::STANDOFFLINESValue { steady: None, nervy: Some(alloc::vec![String::from("where— where did they go—")]), hard: None, under: Some(alloc::vec![String::from("lost them — where'd he take {him}?"), String::from("I've lost sight — moving up")]) }); t.set(String::from("hitHim"), crate::guards::STANDOFFLINESValue { steady: None, nervy: Some(alloc::vec![String::from("no— no no no— {held}—")]), hard: Some(alloc::vec![String::from("…{held}'s hit. that's on him. keep on him—")]), under: Some(alloc::vec![String::from("…christ— I hit {him}. I hit {held}—")]) }); t } }

/// (src/game/guards.ts:1011 STANDOFF_NOW: a table nothing writes)
pub fn standoff_now() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("{held}, stay loose — I\\m coming round"), String::from("{held}, stay loose — I'm coming around")); t.set(String::from("{held}! —I can\\t— I haven\\t got a line—"), String::from("{held}! —I can't— I don't have a shot—")); t.set(String::from("I have him."), String::from("I've got him.")); t.set(String::from("I— I think I have him—"), String::from("I— I think I've got him—")); t.set(String::from("can\\t get round him — hold what you\\ve got"), String::from("can't get around him — hold what you've got")); t.set(String::from("no way round. {held}, hang on"), String::from("no way around. {held}, hang on")); t } }

/// (src/game/guards.ts:1017 STANDOFF_NAMELESS: a table nothing writes)
pub fn standoff_nameless() -> crate::guards::STANDOFFNAMELESS { crate::guards::STANDOFFNAMELESS { lost: None, enter: None, line: None, cornered: None, no_shot: None, dropped: Some(String::from("he's dropped {him} — TAKE HIM")), freed: Some(String::from("he's loose — get clear!")), hit_him: Some(String::from("…christ— I hit {him}—")) } }

/// (src/game/guards.ts:1315 SIGHTS_COS)
pub const SIGHTS_COS: f64 = 0.866;

/// (src/game/guards.ts:1372 UNSTICK_REACH_M)
pub const UNSTICK_REACH_M: f64 = 2.4;

/// (src/game/guards.ts:1372 UNSTICK_STALL_SECS)
pub const UNSTICK_STALL_SECS: f64 = 1.0;

/// (src/game/guards.ts:1372 UNSTICK_DONE_M)
pub const UNSTICK_DONE_M: f64 = 0.08;

/// (src/game/guards.ts:1372 UNSTICK_SNAP_REACH_M)
pub const UNSTICK_SNAP_REACH_M: f64 = 3.5;

/// (src/game/guards.ts:1372 UNSTICK_BODY_R)
pub const UNSTICK_BODY_R: f64 = 0.15;

/// (src/game/guards.ts:1590 WALL_BREAK)
pub const WALL_BREAK: bool = false;

/// (src/game/guards.ts:1597 FLOOR_BREAK)
pub const FLOOR_BREAK: bool = false;

/// (src/game/guards.ts:1598 FLOOR_TOP)
pub const FLOOR_TOP: f64 = 0.249;

/// (src/game/guards.ts:1653 TORCH_WALL_BREAK)
pub const TORCH_WALL_BREAK: bool = false;

/// (src/game/guards.ts:1771 SEARCH_RADIUS_M)
pub const SEARCH_RADIUS_M: f64 = 9.0;

/// (src/game/guards.ts:1771 SEARCH_SPOTS)
pub const SEARCH_SPOTS: f64 = 12.0;

/// (src/game/guards.ts:1771 SEARCH_CONTROLS)
pub const SEARCH_CONTROLS: f64 = 3.0;

/// (src/game/guards.ts:1771 SEARCH_SPACING_M)
pub const SEARCH_SPACING_M: f64 = 1.5;

/// (src/game/guards.ts:1774 SEARCH_SWEEP0_SECS)
pub const SEARCH_SWEEP0_SECS: f64 = 1.2;

/// (src/game/guards.ts:1774 SEARCH_LOOK_SECS)
pub const SEARCH_LOOK_SECS: f64 = 1.5;

/// (src/game/guards.ts:1774 SEARCH_LOOK_DIST_M)
pub const SEARCH_LOOK_DIST_M: f64 = 1.9;

/// (src/game/guards.ts:1777 SEARCH_MIN_SECS)
pub const SEARCH_MIN_SECS: f64 = 12.0;

/// (src/game/guards.ts:1777 SEARCH_CAP_SECS)
pub const SEARCH_CAP_SECS: f64 = 26.0;

/// (src/game/guards.ts:1777 SEARCH_REFLOOD_SECS)
pub const SEARCH_REFLOOD_SECS: f64 = 2.0;

/// (src/game/guards.ts:1781 PROBE_BUDGET)
pub const PROBE_BUDGET: f64 = 8.0;

/// (src/game/guards.ts:1781 PROBE_HEIGHT)
pub const PROBE_HEIGHT: f64 = 0.5;

/// (src/game/guards.ts:1781 DARK_WINDOW_SECS)
pub const DARK_WINDOW_SECS: f64 = 3.0;

/// (src/game/guards.ts:1784 LATCH_SECS)
pub const LATCH_SECS: f64 = 5.5;

/// (src/game/guards.ts:1786 PLAUS_SLACK_SECS)
pub const PLAUS_SLACK_SECS: f64 = 1.2;

/// (src/game/guards.ts:1786 PLAUS_RAMP_SECS)
pub const PLAUS_RAMP_SECS: f64 = 2.5;

/// (src/game/guards.ts:2002 PIE_DETECT_M)
pub const PIE_DETECT_M: f64 = 3.2;

/// (src/game/guards.ts:2002 PIE_SLOW_M)
pub const PIE_SLOW_M: f64 = 1.5;

/// (src/game/guards.ts:2002 PIE_SLICE_M)
pub const PIE_SLICE_M: f64 = 1.7;

/// (src/game/guards.ts:2002 PIE_SLICE_SECS)
pub const PIE_SLICE_SECS: f64 = 1.0;

/// (src/game/guards.ts:2002 PIE_CORNER_SECS)
pub const PIE_CORNER_SECS: f64 = 0.4;

/// (src/game/guards.ts:2002 PIE_AGAIN_SECS)
pub const PIE_AGAIN_SECS: f64 = 6.0;

/// (src/game/guards.ts:2002 PIE_CAP_SECS)
pub const PIE_CAP_SECS: f64 = 7.0;

/// (src/game/guards.ts:2004 PIE_EYES_SECS)
pub const PIE_EYES_SECS: f64 = 1.0;

/// (src/game/guards.ts:2560 GUARD_SWEEP_STEP)
pub const GUARD_SWEEP_STEP: f64 = 0.2;

/// (src/game/guards.ts:2562 SEP_MAX_M)
pub const SEP_MAX_M: f64 = 0.25;

/// (src/game/guards.ts:2566 SEP_SETTLE_MIN)
pub const SEP_SETTLE_MIN: f64 = 0.02;

/// (src/game/guards.ts:2612 NO_SIGHT: a table nothing writes)
pub fn no_sight() -> crate::guards::NOSIGHT { crate::guards::NOSIGHT { visible: false, dist: 99.0, in_cone: false } }

/// (src/game/guards.ts:2614 METER_NORMALS: a table nothing writes)
pub fn meter_normals() -> Vec<V3> { alloc::vec![[1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, -1.0]] }

/// (src/game/guards.ts:2838 BURST_BROKEN_CD)
pub const BURST_BROKEN_CD: f64 = 0.4;

/// (src/game/guards.ts:2838 SUPPRESS_FIRST_MIN_M)
pub const SUPPRESS_FIRST_MIN_M: f64 = 10.0;

/// (src/game/guards.ts:2838 SUPPRESS_FIRST_OFF_M)
pub const SUPPRESS_FIRST_OFF_M: f64 = 0.6;

/// (src/game/guards.ts:2838 SUPPRESS_FIRST_JITTER_M)
pub const SUPPRESS_FIRST_JITTER_M: f64 = 0.4;

/// (src/game/guards.ts:2838 SUPPRESS_FIRST_UP_M)
pub const SUPPRESS_FIRST_UP_M: f64 = 0.3;

/// (src/game/guards.ts:2838 SUPPRESS_FIRST_WALK_M)
pub const SUPPRESS_FIRST_WALK_M: f64 = 0.07;

/// (src/game/guards.ts:2838 SUPPRESS_FIRST_COVER_M)
pub const SUPPRESS_FIRST_COVER_M: f64 = 2.5;

/// (src/game/guards.ts:2838 SUPPRESS_FIRST_LAPSE)
pub const SUPPRESS_FIRST_LAPSE: f64 = 8.0;

/// (src/game/guards.ts:2998 HEIGHTENED_SECS)
pub const HEIGHTENED_SECS: f64 = 75.0;

/// (src/game/guards.ts:2998 LOCKDOWN_SECS)
pub const LOCKDOWN_SECS: f64 = 120.0;

/// (src/game/guards.ts:2998 STEPDOWN_SECS)
pub const STEPDOWN_SECS: f64 = 45.0;

/// (src/game/guards.ts:3000 KEEN)
pub const KEEN: f64 = 1.25;

/// (src/game/guards.ts:3062 ROLLCALL_CALM_SECS)
pub const ROLLCALL_CALM_SECS: f64 = 90.0;

/// (src/game/guards.ts:3062 ROLLCALL_ALARMED_SECS)
pub const ROLLCALL_ALARMED_SECS: f64 = 45.0;

/// (src/game/guards.ts:3062 ROLLCALL_JITTER)
pub const ROLLCALL_JITTER: f64 = 0.1;

/// (src/game/guards.ts:3064 ROLLCALL_ANSWER_SECS)
pub const ROLLCALL_ANSWER_SECS: f64 = 2.2;

/// (src/game/guards.ts:3064 ROLLCALL_ASK_SECS)
pub const ROLLCALL_ASK_SECS: f64 = 4.0;

/// (src/game/guards.ts:3064 ROLLCALL_SEND_SECS)
pub const ROLLCALL_SEND_SECS: f64 = 8.0;

/// (src/game/guards.ts:3067 ROLLCALL_BUSY_RETRY_SECS)
pub const ROLLCALL_BUSY_RETRY_SECS: f64 = 15.0;

/// (src/game/guards.ts:3067 ROLLCALL_QUIET_AFTER_SECS)
pub const ROLLCALL_QUIET_AFTER_SECS: f64 = 30.0;

/// (src/game/guards.ts:3067 ROLLCALL_SEND_GIVEUP_SECS)
pub const ROLLCALL_SEND_GIVEUP_SECS: f64 = 40.0;

/// (src/game/guards.ts:3069 CHECK_LOOK_SECS)
pub const CHECK_LOOK_SECS: f64 = 1.2;

/// (src/game/guards.ts:3069 CHECK_CAP_SECS)
pub const CHECK_CAP_SECS: f64 = 150.0;

/// (src/game/guards.ts:3071 PARTNER_UNSEEN_SECS)
pub const PARTNER_UNSEEN_SECS: f64 = 20.0;

/// (src/game/guards.ts:3071 PARTNER_SPOT_M)
pub const PARTNER_SPOT_M: f64 = 2.5;

/// (src/game/guards.ts:3244 RECLEAR_SECS)
pub const RECLEAR_SECS: f64 = 60.0;

/// (src/game/guards.ts:3246 TAMPER_MEMORY_SECS)
pub const TAMPER_MEMORY_SECS: f64 = 120.0;

/// (src/game/guards.ts:3380 PARTNER_TRUST_SECS)
pub const PARTNER_TRUST_SECS: f64 = 3.0;

/// (src/game/guards.ts:3506 MAX_CHARACTERS)
pub const MAX_CHARACTERS: f64 = 8.0;

/// (src/game/guards.ts:3508 MAX_SKINNED)
pub const MAX_SKINNED: f64 = 16.0;

/// (src/game/guards.ts:3510 UNIT_ID0)
pub const UNIT_ID0: f64 = 20.0;

/// (src/game/guards.ts:3510 UNIT_ID1)
pub const UNIT_ID1: f64 = 250.0;

/// (src/game/guards.ts:3513 TEAM_TINT: a table nothing writes)
pub fn team_tint() -> JsMap<String, V3> { { let mut t = JsMap::new(); t.set(String::from("guard"), [0.42, 0.36, 0.25]); t.set(String::from("operative"), [0.07, 0.09, 0.1]); t.set(String::from("hostile"), [0.36, 0.33, 0.24]); t.set(String::from("player"), [0.05, 0.06, 0.07]); t.set(String::from("civilian"), [0.75, 0.62, 0.08]); t } }

/// (src/game/guards.ts:3514 TEAM_TINT2: a table nothing writes)
pub fn team_tint2() -> JsMap<String, V3> { { let mut t = JsMap::new(); t.set(String::from("guard"), [0.02, 0.02, 0.025]); t.set(String::from("operative"), [0.05, 0.07, 0.06]); t.set(String::from("hostile"), [0.09, 0.08, 0.05]); t.set(String::from("player"), [0.1, 0.32, 0.18]); t.set(String::from("civilian"), [0.05, 0.06, 0.1]); t } }

/// (src/game/guards.ts:3630 BREACH_HOOKS: a table nothing writes)
pub fn breach_hooks() -> crate::guards::BREACHHOOKS { crate::guards::BREACHHOOKS {  } }
