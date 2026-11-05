//! The types of `player`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Hold, src/game/player.ts:131)
#[derive(Clone, Debug)]
pub struct Hold {
    pub g: GuardId,
    pub variant: crate::character::HoldVariant,
    pub phase: crate::character::HoldPhase,
    pub t: f64,
    pub since: f64,
    pub wind_t: f64,
    pub swap_t: f64,
    pub remarked: bool,
    pub gun_out: f64,
    pub fire_queued: bool,
    pub shot_t: f64,
    pub ask: Option<crate::player::HoldAsk>,
    pub after: Option<crate::player::HoldAfter>,
    pub strip: Option<crate::disarm::Strip>,
    pub from: Option<V3>,
    pub hit_at: Option<f64>,
    pub yank_m: Option<f64>,
    pub yanked: Option<f64>,
    pub step_e: Option<f64>,
    pub step_m: Option<f64>,
    pub yaw0: Option<f64>,
    pub coil0: Option<f64>,
    pub rmb_choke: Option<f64>,
    pub rmb_t: Option<f64>,
    /// INVENTED: dealt when a hold starts; a new hold of the same man counts as new
    pub serial: u32,
}
impl Hold {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Hold {
            g: GuardId(0),
            variant: crate::character::HoldVariant::Gun,
            phase: crate::character::HoldPhase::Held,
            t: 0.0,
            since: 0.0,
            wind_t: 0.0,
            swap_t: 0.0,
            remarked: false,
            gun_out: 0.0,
            fire_queued: false,
            shot_t: 0.0,
            ask: None,
            after: None,
            strip: None,
            from: None,
            hit_at: None,
            yank_m: None,
            yanked: None,
            step_e: None,
            step_m: None,
            yaw0: None,
            coil0: None,
            rmb_choke: None,
            rmb_t: None,
            serial: 0,
        }
    }
}
impl Default for Hold { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:131 Hold.ask, src/game/player.ts:131)
#[derive(Clone, Copy, Debug)]
pub struct HoldAsk {
    pub t: f64,
    pub dur: f64,
}
impl HoldAsk {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldAsk {
            t: 0.0,
            dur: 0.0,
        }
    }
}
impl Default for HoldAsk { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:131 Hold.after, src/game/player.ts:131)
#[derive(Clone, Debug)]
pub struct HoldAfter {
    pub text: String,
    pub id: String,
}
impl HoldAfter {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldAfter {
            text: String::new(),
            id: String::new(),
        }
    }
}
impl Default for HoldAfter { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Legs, src/game/player.ts:413)
#[derive(Clone, Copy, Debug)]
pub struct Legs {
    pub char: crate::player::LegsChar,
    pub safe: Option<V3>,
    pub crouch: bool,
    pub sprinting: bool,
    pub dash: bool,
    pub backpedal: bool,
    pub speed_sm: f64,
    pub pace: f64,
    pub dragging: Option<GuardId>,
    pub haul: Option<crate::hands::Haul>,
    pub shoved: [f64; 2],
    pub shoved_t: f64,
    pub shoved_from: [f64; 2],
    pub down: bool,
    pub door_rip_until: f64,
    pub door_bash_at: f64,
    pub door_bash_lean: f64,
}
impl Legs {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Legs {
            char: crate::player::LegsChar::zeroed(),
            safe: None,
            crouch: false,
            sprinting: false,
            dash: false,
            backpedal: false,
            speed_sm: 0.0,
            pace: 0.0,
            dragging: None,
            haul: None,
            shoved: [0.0; 2],
            shoved_t: 0.0,
            shoved_from: [0.0; 2],
            down: false,
            door_rip_until: -1.0,
            door_bash_at: -10.0,
            door_bash_lean: 0.0,
        }
    }
}
impl Default for Legs { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:414 Legs.char, src/game/player.ts:414)
#[derive(Clone, Copy, Debug)]
pub struct LegsChar {
    pub pos: V3,
    pub vel: V3,
    pub body_yaw: f64,
    pub aim_yaw: f64,
}
impl LegsChar {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LegsChar {
            pos: [0.0; 3],
            vel: [0.0; 3],
            body_yaw: 0.0,
            aim_yaw: 0.0,
        }
    }
}
impl Default for LegsChar { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: WallHold, src/game/player.ts:927)
#[derive(Clone, Copy, Debug)]
pub struct WallHold {
    pub n: V3,
    pub along: V3,
    pub d: f64,
    pub lo: f64,
    pub hi: f64,
    pub open_lo: bool,
    pub open_hi: bool,
    pub u: f64,
    pub v: f64,
    pub side: f64,
    pub off: f64,
    pub away_t: f64,
    pub settle: f64,
    pub from: V3,
    pub block_dir: f64,
    pub block_t: f64,
    pub seated: bool,
    pub drawn: bool,
    pub at_edge: f64,
    pub peek: f64,
    pub peek_dir: f64,
    pub crouched: bool,
    pub take_t: Option<f64>,
    pub yaw0: Option<f64>,
    pub yaw_d: Option<f64>,
    pub hit_t: Option<f64>,
    pub kind: Option<crate::player::WallHoldKind>,
    pub up: Option<f64>,
    pub lean: Option<f64>,
    pub lean_want: Option<bool>,
    pub lean_dir: Option<f64>,
    pub lean_dwell: Option<f64>,
    pub lean_out_t: Option<f64>,
    pub lean_off: Option<f64>,
    pub lean_more: Option<f64>,
    pub flip_t: Option<f64>,
    pub say_t: Option<f64>,
    pub rise: Option<f64>,
    pub owed: Option<f64>,
    pub cam_along: Option<f64>,
    pub cam: bool,
    pub ran: Option<f64>,
}
impl WallHold {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallHold {
            n: [0.0; 3],
            along: [0.0; 3],
            d: 0.0,
            lo: 0.0,
            hi: 0.0,
            open_lo: false,
            open_hi: false,
            u: 0.0,
            v: 0.0,
            side: 0.0,
            off: 0.0,
            away_t: 0.0,
            settle: 0.0,
            from: [0.0; 3],
            block_dir: 0.0,
            block_t: 0.0,
            seated: false,
            drawn: false,
            at_edge: 0.0,
            peek: 0.0,
            peek_dir: 0.0,
            crouched: false,
            take_t: None,
            yaw0: None,
            yaw_d: None,
            hit_t: None,
            kind: None,
            up: None,
            lean: None,
            lean_want: None,
            lean_dir: None,
            lean_dwell: None,
            lean_out_t: None,
            lean_off: None,
            lean_more: None,
            flip_t: None,
            say_t: None,
            rise: None,
            owed: None,
            cam_along: None,
            cam: false,
            ran: None,
        }
    }
}
impl Default for WallHold { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WallHoldKind {
    Stack,
}
impl WallHoldKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            WallHoldKind::Stack => "stack",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WhKind {
    Stack,
    Unit,
    Bool,
    Vec,
    Num,
    Opt,
    Optbool,
    Sign,
    Sign0,
    Optsign,
    Derived,
}
impl WhKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            WhKind::Stack => "stack",
            WhKind::Unit => "unit",
            WhKind::Bool => "bool",
            WhKind::Vec => "vec",
            WhKind::Num => "num",
            WhKind::Opt => "opt",
            WhKind::Optbool => "optbool",
            WhKind::Sign => "sign",
            WhKind::Sign0 => "sign0",
            WhKind::Optsign => "optsign",
            WhKind::Derived => "derived",
        }
    }
}

/// (TypeScript: WallSnap, src/game/player.ts:1222)
#[derive(Clone, Debug)]
pub struct WallSnap {
    pub hold: Option<crate::player::WallHold>,
    pub press: Option<crate::player::WallSnapPress>,
}
impl WallSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallSnap {
            hold: None,
            press: None,
        }
    }
}
impl Default for WallSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:1222 WallSnap.press, src/game/player.ts:1222)
#[derive(Clone, Debug)]
pub struct WallSnapPress {
    pub t: f64,
    pub at: f64,
    pub n: Option<Vec<f64>>,
    pub dx: f64,
    pub dz: f64,
    pub held: bool,
    pub until: f64,
}
impl WallSnapPress {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallSnapPress {
            t: 0.0,
            at: 0.0,
            n: None,
            dx: 0.0,
            dz: 0.0,
            held: false,
            until: 0.0,
        }
    }
}
impl Default for WallSnapPress { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:148 DOOR_FIT, src/game/player.ts:148)
#[derive(Clone, Copy, Debug)]
pub struct DOORFIT {
    pub lat: [f64; 2],
    pub out: [f64; 2],
}
impl DOORFIT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DOORFIT {
            lat: [0.0; 2],
            out: [0.0; 2],
        }
    }
}
impl Default for DOORFIT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:1134 wsPend, src/game/player.ts:1134)
#[derive(Clone, Copy, Debug)]
pub struct PlayerModWsPendValue {
    pub t: f64,
    pub at: f64,
    pub blocked: bool,
}
impl PlayerModWsPendValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerModWsPendValue {
            t: 0.0,
            at: 0.0,
            blocked: false,
        }
    }
}
impl Default for PlayerModWsPendValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:1164 wsPress, src/game/player.ts:1164)
#[derive(Clone, Copy, Debug)]
pub struct PlayerModWsPressValue {
    pub t: f64,
    pub at: f64,
    pub n: Option<V3>,
    pub dx: f64,
    pub dz: f64,
    pub held: bool,
    pub until: f64,
}
impl PlayerModWsPressValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerModWsPressValue {
            t: 0.0,
            at: 0.0,
            n: None,
            dx: 0.0,
            dz: 0.0,
            held: false,
            until: 0.0,
        }
    }
}
impl Default for PlayerModWsPressValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:1216 WH, src/game/player.ts:1216)
#[derive(Clone, Debug)]
pub struct WH {
    pub n: String,
    pub along: String,
    pub from: String,
    pub d: String,
    pub lo: String,
    pub hi: String,
    pub u: String,
    pub v: String,
    pub off: String,
    pub away_t: String,
    pub block_t: String,
    pub settle: String,
    pub peek: String,
    pub open_lo: String,
    pub open_hi: String,
    pub seated: String,
    pub drawn: String,
    pub crouched: String,
    pub side: String,
    pub peek_dir: String,
    pub block_dir: String,
    pub at_edge: String,
    pub take_t: String,
    pub yaw0: String,
    pub yaw_d: String,
    pub hit_t: String,
    pub ran: String,
    pub up: String,
    pub lean: String,
    pub lean_dwell: String,
    pub lean_out_t: String,
    pub lean_off: String,
    pub lean_more: String,
    pub flip_t: String,
    pub say_t: String,
    pub rise: String,
    pub owed: String,
    pub cam_along: String,
    pub lean_want: String,
    pub lean_dir: String,
    pub kind: String,
}
impl WH {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WH {
            n: String::new(),
            along: String::new(),
            from: String::new(),
            d: String::new(),
            lo: String::new(),
            hi: String::new(),
            u: String::new(),
            v: String::new(),
            off: String::new(),
            away_t: String::new(),
            block_t: String::new(),
            settle: String::new(),
            peek: String::new(),
            open_lo: String::new(),
            open_hi: String::new(),
            seated: String::new(),
            drawn: String::new(),
            crouched: String::new(),
            side: String::new(),
            peek_dir: String::new(),
            block_dir: String::new(),
            at_edge: String::new(),
            take_t: String::new(),
            yaw0: String::new(),
            yaw_d: String::new(),
            hit_t: String::new(),
            ran: String::new(),
            up: String::new(),
            lean: String::new(),
            lean_dwell: String::new(),
            lean_out_t: String::new(),
            lean_off: String::new(),
            lean_more: String::new(),
            flip_t: String::new(),
            say_t: String::new(),
            rise: String::new(),
            owed: String::new(),
            cam_along: String::new(),
            lean_want: String::new(),
            lean_dir: String::new(),
            kind: String::new(),
        }
    }
}
impl Default for WH { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:176 collideMan(pl), src/game/player.ts:176)
#[derive(Clone, Copy, Debug)]
pub struct CollideManPl {
    pub char: crate::player::CollideManPlChar,
    pub safe: Option<V3>,
}
impl CollideManPl {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CollideManPl {
            char: crate::player::CollideManPlChar::zeroed(),
            safe: None,
        }
    }
}
impl Default for CollideManPl { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:176 CollideManPl.char, src/game/player.ts:176)
#[derive(Clone, Copy, Debug)]
pub struct CollideManPlChar {
    pub pos: V3,
}
impl CollideManPlChar {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CollideManPlChar {
            pos: [0.0; 3],
        }
    }
}
impl Default for CollideManPlChar { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ShellBreak {
    Fire,
    Bolt,
}
impl ShellBreak {
    pub const fn as_str(self) -> &'static str {
        match self {
            ShellBreak::Fire => "fire",
            ShellBreak::Bolt => "bolt",
        }
    }
}

/// (TypeScript: inline at src/game/player.ts:438 strideStep(tap), src/game/player.ts:438)
#[derive(Clone, Copy, Debug)]
pub struct StrideStepTap {
    pub speed: f64,
}
impl StrideStepTap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StrideStepTap {
            speed: 0.0,
        }
    }
}
impl Default for StrideStepTap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:438 strideStep(stalk), src/game/player.ts:438)
#[derive(Clone, Copy, Debug)]
pub struct StrideStepStalk {
    pub pace: f64,
    pub keep: Option<bool>,
}
impl StrideStepStalk {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StrideStepStalk {
            pace: 0.0,
            keep: None,
        }
    }
}
impl Default for StrideStepStalk { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:453 gunRaised(m), src/game/player.ts:453)
#[derive(Clone, Copy, Debug)]
pub struct GunRaisedM {
    pub sprinting: bool,
}
impl GunRaisedM {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GunRaisedM {
            sprinting: false,
        }
    }
}
impl Default for GunRaisedM { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:455 leafFacing(dp), src/game/player.ts:455)
#[derive(Clone, Debug)]
pub struct LeafFacingDp {
    pub w: f64,
    pub n: Vec<f64>,
    pub hold_only: Option<bool>,
}
impl LeafFacingDp {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LeafFacingDp {
            w: 0.0,
            n: Vec::new(),
            hold_only: None,
        }
    }
}
impl Default for LeafFacingDp { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:1020 findWall(why), src/game/player.ts:1020)
#[derive(Clone, Copy, Debug)]
pub struct FindWallWhy {
    pub blocked: bool,
}
impl FindWallWhy {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FindWallWhy {
            blocked: false,
        }
    }
}
impl Default for FindWallWhy { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:1020 findWall ->, src/game/player.ts:1020)
#[derive(Clone, Copy, Debug)]
pub struct FindWall {
    pub n: V3,
    pub t: V3,
    pub d: f64,
    pub lo: f64,
    pub hi: f64,
    pub u: f64,
}
impl FindWall {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FindWall {
            n: [0.0; 3],
            t: [0.0; 3],
            d: 0.0,
            lo: 0.0,
            hi: 0.0,
            u: 0.0,
        }
    }
}
impl Default for FindWall { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:1073 wallSnap(why), src/game/player.ts:1073)
#[derive(Clone, Copy, Debug)]
pub struct WallSnapWhy {
    pub blocked: bool,
}
impl WallSnapWhy {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallSnapWhy {
            blocked: false,
        }
    }
}
impl Default for WallSnapWhy { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WallStepOffHow {
    Run,
    Q,
    Away,
}
impl WallStepOffHow {
    pub const fn as_str(self) -> &'static str {
        match self {
            WallStepOffHow::Run => "run",
            WallStepOffHow::Q => "q",
            WallStepOffHow::Away => "away",
        }
    }
}

/// (TypeScript: inline at src/game/player.ts:1135 wallSnapLater(why), src/game/player.ts:1135)
#[derive(Clone, Copy, Debug)]
pub struct WallSnapLaterWhy {
    pub blocked: bool,
}
impl WallSnapLaterWhy {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallSnapLaterWhy {
            blocked: false,
        }
    }
}
impl Default for WallSnapLaterWhy { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:1167 pressFace ->, src/game/player.ts:1167)
#[derive(Clone, Copy, Debug)]
pub struct PressFace {
    pub n: V3,
    pub at: V3,
    pub off: f64,
}
impl PressFace {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PressFace {
            n: [0.0; 3],
            at: [0.0; 3],
            off: 0.0,
        }
    }
}
impl Default for PressFace { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:1277 throwRest ->, src/game/player.ts:1277)
#[derive(Clone, Debug)]
pub struct ThrowRest {
    pub at: V3,
    pub path: Vec<V3>,
}
impl ThrowRest {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ThrowRest {
            at: [0.0; 3],
            path: Vec::new(),
        }
    }
}
impl Default for ThrowRest { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:1908 heldTalk ->, src/game/player.ts:1908)
#[derive(Clone, Debug)]
pub struct HeldTalk {
    pub asking: bool,
    pub sam_up: bool,
    pub ready: bool,
    pub settled: bool,
    pub speaking: Option<crate::game::GuardBubble>,
}
impl HeldTalk {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HeldTalk {
            asking: false,
            sam_up: false,
            ready: false,
            settled: false,
            speaking: None,
        }
    }
}
impl Default for HeldTalk { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FreeHeldWhy {
    Shot,
    Cancel,
}
impl FreeHeldWhy {
    pub const fn as_str(self) -> &'static str {
        match self {
            FreeHeldWhy::Shot => "shot",
            FreeHeldWhy::Cancel => "cancel",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum StartKickTool {
    Shotgun,
    Hammer,
}
impl StartKickTool {
    pub const fn as_str(self) -> &'static str {
        match self {
            StartKickTool::Shotgun => "shotgun",
            StartKickTool::Hammer => "hammer",
        }
    }
}

/// (TypeScript: inline at src/game/player.ts:2172 sideKickSpot ->, src/game/player.ts:2172)
#[derive(Clone, Copy, Debug)]
pub struct SideKickSpot {
    pub pos: V3,
    pub yaw: f64,
}
impl SideKickSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SideKickSpot {
            pos: [0.0; 3],
            yaw: 0.0,
        }
    }
}
impl Default for SideKickSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:2231 aimLevel(c), src/game/player.ts:2231)
#[derive(Clone, Copy, Debug)]
pub struct AimLevelC {
    pub pos: V3,
    pub body_yaw: f64,
}
impl AimLevelC {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AimLevelC {
            pos: [0.0; 3],
            body_yaw: 0.0,
        }
    }
}
impl Default for AimLevelC { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:2234 cursorHit(ray), src/game/player.ts:2234)
#[derive(Clone, Copy, Debug)]
pub struct CursorHitRay {
    pub ro: V3,
    pub rd: V3,
}
impl CursorHitRay {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CursorHitRay {
            ro: [0.0; 3],
            rd: [0.0; 3],
        }
    }
}
impl Default for CursorHitRay { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:2235 cursorPoint(ray), src/game/player.ts:2235)
#[derive(Clone, Copy, Debug)]
pub struct CursorPointRay {
    pub ro: V3,
    pub rd: V3,
}
impl CursorPointRay {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CursorPointRay {
            ro: [0.0; 3],
            rd: [0.0; 3],
        }
    }
}
impl Default for CursorPointRay { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:2235 cursorPoint(hit), src/game/player.ts:2235)
#[derive(Clone, Copy, Debug)]
pub struct CursorPointHit {
    pub t: f64,
}
impl CursorPointHit {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CursorPointHit {
            t: 0.0,
        }
    }
}
impl Default for CursorPointHit { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct PlayerMod {
    /// (src/game/player.ts:1134 wsPend)
    pub ws_pend: JsMap<PlayerId, crate::player::PlayerModWsPendValue>,
    /// (src/game/player.ts:1164 wsPress)
    pub ws_press: JsMap<PlayerId, crate::player::PlayerModWsPressValue>,
    /// (src/game/player.ts:1209 PRESS_THINGS)
    pub press_things: JsSet<String>,
    /// (src/game/player.ts:2300 lightAt)
    pub light_at: JsMap<PlayerId, [f64; 2]>,
}
impl PlayerMod {
    pub fn blank() -> Self {
        PlayerMod {
            ws_pend: JsMap::new(),
            ws_press: JsMap::new(),
            press_things: { let mut s = JsSet::new(); for k in ["door", "switch", "breaker", "objective", "cam", "body"] { s.add(String::from(k)); } s },
            light_at: JsMap::new(),
        }
    }
}

/// (src/game/player.ts:91 BODY_R)
pub const BODY_R: f64 = 0.42;

/// (src/game/player.ts:93 KEEP_OFF)
pub const KEEP_OFF: f64 = 0.55;

/// (src/game/player.ts:100 HOLD_PACE)
pub const HOLD_PACE: f64 = 0.85;

/// (src/game/player.ts:100 HOLD_PACE_BACK)
pub const HOLD_PACE_BACK: f64 = 0.65;

/// (src/game/player.ts:100 HOLD_TURN)
pub const HOLD_TURN: f64 = 2.6;

/// (src/game/player.ts:105 INTERROGATE_GAP)
pub const INTERROGATE_GAP: f64 = 1.2;

/// (src/game/player.ts:105 ANSWER_TAIL)
pub const ANSWER_TAIL: f64 = 0.5;

/// (src/game/player.ts:110 GRAB_NOISE)
pub const GRAB_NOISE: f64 = 0.12;

/// (src/game/player.ts:110 CHOKE_NOISE)
pub const CHOKE_NOISE: f64 = 0.15;

/// (src/game/player.ts:110 FOLD_NOISE)
pub const FOLD_NOISE: f64 = 0.3;

/// (src/game/player.ts:110 WHIP_NOISE)
pub const WHIP_NOISE: f64 = 0.35;

/// (src/game/player.ts:113 CHOKE_GRAIN)
pub const CHOKE_GRAIN: f64 = 0.4;

/// (src/game/player.ts:116 HOLD_FIRE_CD)
pub const HOLD_FIRE_CD: f64 = 0.7;

/// (src/game/player.ts:116 HOLD_SPREAD)
pub const HOLD_SPREAD: f64 = 0.09;

/// (src/game/player.ts:148 DOOR_R)
pub const DOOR_R: f64 = 0.3;

/// (src/game/player.ts:148 DOOR_FIT: a table nothing writes)
pub fn door_fit() -> crate::player::DOORFIT { crate::player::DOORFIT { lat: [0.12, 0.5], out: [0.55, 1.0] } }

/// (src/game/player.ts:166 SWEEP_STEP)
pub const SWEEP_STEP: f64 = 0.2;

/// (src/game/player.ts:218 PACE_MIN)
pub const PACE_MIN: f64 = 0.35;

/// (src/game/player.ts:218 PACE_STEP)
pub const PACE_STEP: f64 = 0.13;

/// (src/game/player.ts:260 RIFLE_EQUIP_SECS)
pub const RIFLE_EQUIP_SECS: f64 = 0.45;

/// (src/game/player.ts:290 LOW_AT)
pub const LOW_AT: f64 = 1.25;

/// (src/game/player.ts:290 LOW_UNDER)
pub const LOW_UNDER: f64 = 0.25;

/// (src/game/player.ts:290 LOW_TOP)
pub const LOW_TOP: f64 = 0.15;

/// (src/game/player.ts:305 LEVEL_REACH)
pub const LEVEL_REACH: f64 = 20.0;

/// (src/game/player.ts:895 WALL_REACH)
pub const WALL_REACH: f64 = 0.7;

/// (src/game/player.ts:897 WALL_GAP)
pub const WALL_GAP: f64 = 0.03;

/// (src/game/player.ts:900 KNEE_Y)
pub const KNEE_Y: f64 = 0.45;

/// (src/game/player.ts:900 CHEST_Y)
pub const CHEST_Y: f64 = 1.35;

/// (src/game/player.ts:900 HIGH_Y)
pub const HIGH_Y: f64 = 2.7;

/// (src/game/player.ts:902 COPLANAR)
pub const COPLANAR: f64 = 0.01;

/// (src/game/player.ts:904 UPPER_TOL)
pub const UPPER_TOL: f64 = 0.08;

/// (src/game/player.ts:906 WALL_BRIDGE)
pub const WALL_BRIDGE: f64 = 0.15;

/// (src/game/player.ts:908 WALL_MIN_FACE)
pub const WALL_MIN_FACE: f64 = 0.8999999999999999;

/// (src/game/player.ts:910 WALL_PACE)
pub const WALL_PACE: f64 = 1.4;

/// (src/game/player.ts:912 WALL_TURN)
pub const WALL_TURN: f64 = 0.9599310885968813;

/// (src/game/player.ts:914 WALL_LOOK)
pub const WALL_LOOK: f64 = 0.4363323129985824;

/// (src/game/player.ts:914 WALL_PEEK_LOOK)
pub const WALL_PEEK_LOOK: f64 = 0.5235987755982988;

/// (src/game/player.ts:917 WALL_SHOVE_OFF)
pub const WALL_SHOVE_OFF: f64 = 0.25;

/// (src/game/player.ts:917 WALL_AWAY_SECS)
pub const WALL_AWAY_SECS: f64 = 0.25;

/// (src/game/player.ts:919 WALL_SETTLE)
pub const WALL_SETTLE: f64 = 0.22;

/// (src/game/player.ts:919 WALL_PEEK_SECS)
pub const WALL_PEEK_SECS: f64 = 0.3;

/// (src/game/player.ts:991 PROP_BODY_R)
pub const PROP_BODY_R: f64 = 0.3;

/// (src/game/player.ts:1216 WH: a table nothing writes)
pub fn wh() -> crate::player::WH { crate::player::WH { n: String::from("vec"), along: String::from("derived"), from: String::from("vec"), d: String::from("num"), lo: String::from("num"), hi: String::from("num"), u: String::from("num"), v: String::from("num"), off: String::from("num"), away_t: String::from("num"), block_t: String::from("num"), settle: String::from("unit"), peek: String::from("unit"), open_lo: String::from("bool"), open_hi: String::from("bool"), seated: String::from("bool"), drawn: String::from("bool"), crouched: String::from("bool"), side: String::from("sign"), peek_dir: String::from("sign"), block_dir: String::from("sign0"), at_edge: String::from("sign0"), take_t: String::from("opt"), yaw0: String::from("opt"), yaw_d: String::from("opt"), hit_t: String::from("opt"), ran: String::from("opt"), up: String::from("opt"), lean: String::from("opt"), lean_dwell: String::from("opt"), lean_out_t: String::from("opt"), lean_off: String::from("opt"), lean_more: String::from("opt"), flip_t: String::from("opt"), say_t: String::from("opt"), rise: String::from("opt"), owed: String::from("opt"), cam_along: String::from("opt"), lean_want: String::from("optbool"), lean_dir: String::from("optsign"), kind: String::from("stack") } }

/// (src/game/player.ts:1381 LEAN_CHEST)
pub const LEAN_CHEST: f64 = 1.2;

/// (src/game/player.ts:1383 STACK_PRESS)
pub const STACK_PRESS: f64 = 0.12;

/// (src/game/player.ts:2237 WOUNDED_SNAP_PX)
pub const WOUNDED_SNAP_PX: f64 = 26.0;

/// (src/game/player.ts:2553 INTERACT_PRIORITY: a table nothing writes)
pub fn interact_priority() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("guard"), 0.0); t.set(String::from("held"), 0.0); t.set(String::from("body"), 1.0); t.set(String::from("objective"), 2.0); t.set(String::from("door"), 3.0); t.set(String::from("light"), 9.0); t.set(String::from("breaker"), 2.5); t.set(String::from("switch"), 3.2); t.set(String::from("charge"), 9.4); t.set(String::from("wall"), 9.5); t.set(String::from("cam"), 9.6); t.set(String::from("cancel"), 10.0); t } }

/// (src/game/player.ts:2561 NEARBY_M)
pub const NEARBY_M: f64 = 3.0;

/// (src/game/player.ts:2563 MAX_NEAR)
pub const MAX_NEAR: f64 = 3.0;

/// (src/game/player.ts:2567 EASE_RATE)
pub const EASE_RATE: f64 = 0.55;

/// (src/game/player.ts:2567 EASE_MAX)
pub const EASE_MAX: f64 = 1.45;

/// (src/game/player.ts:2567 EASE_OFFER)
pub const EASE_OFFER: f64 = 0.62;
