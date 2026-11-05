//! The types of `sneak`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Why {
    Door,
    Near,
    Open,
    Lit,
    Long,
}
impl Why {
    pub const fn as_str(self) -> &'static str {
        match self {
            Why::Door => "door",
            Why::Near => "near",
            Why::Open => "open",
            Why::Lit => "lit",
            Why::Long => "long",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Mode {
    Cross,
    Back,
    Wait,
    Stand,
    Walk,
}
impl Mode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Mode::Cross => "cross",
            Mode::Back => "back",
            Mode::Wait => "wait",
            Mode::Stand => "stand",
            Mode::Walk => "walk",
        }
    }
}

/// (TypeScript: Crumb, src/game/sneak.ts:170)
#[derive(Clone, Copy, Debug)]
pub struct Crumb {
    pub p: V3,
    pub lit: f64,
    pub t: f64,
    pub low: bool,
    pub amb: f64,
    pub cpu0: f64,
}
impl Crumb {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Crumb {
            p: [0.0; 3],
            lit: 0.0,
            t: 0.0,
            low: false,
            amb: 0.0,
            cpu0: 0.0,
        }
    }
}
impl Default for Crumb { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Man, src/game/sneak.ts:176)
#[derive(Clone, Debug)]
pub struct Man {
    pub i: f64,
    pub t: f64,
    pub lost_t: f64,
    pub mode: crate::sneak::Mode,
    pub plan_t: f64,
    pub wait_t: f64,
    pub why: crate::sneak::Why,
    pub nagged: bool,
    pub cue_t: f64,
    pub voice_t: f64,
    pub back_t: f64,
    pub hide_t: f64,
    pub hid_i: f64,
    pub mem: Option<crate::sneak::ManMem>,
    pub cross_from: f64,
    pub cross_to: f64,
    pub cross_t: f64,
    pub stand: f64,
    pub side: Option<V3>,
    pub side_t: f64,
    pub way: Option<Vec<V3>>,
}
impl Man {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Man {
            i: 0.0,
            t: 0.0,
            lost_t: 0.0,
            mode: crate::sneak::Mode::Cross,
            plan_t: 0.0,
            wait_t: 0.0,
            why: crate::sneak::Why::Door,
            nagged: false,
            cue_t: 0.0,
            voice_t: 0.0,
            back_t: 0.0,
            hide_t: 0.0,
            hid_i: 0.0,
            mem: None,
            cross_from: 0.0,
            cross_to: 0.0,
            cross_t: 0.0,
            stand: 0.0,
            side: None,
            side_t: 0.0,
            way: None,
        }
    }
}
impl Default for Man { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sneak.ts:181 Man.mem, src/game/sneak.ts:181)
#[derive(Clone, Copy, Debug)]
pub struct ManMem {
    pub h: GuardId,
    pub a: f64,
    pub b: f64,
    pub t: f64,
}
impl ManMem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ManMem {
            h: GuardId(0),
            a: 0.0,
            b: 0.0,
            t: 0.0,
        }
    }
}
impl Default for ManMem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: State, src/game/sneak.ts:192)
#[derive(Clone, Debug)]
pub struct State {
    pub trail: Vec<crate::sneak::Crumb>,
    pub off: f64,
    pub t: f64,
    pub men: JsMap<GuardId, crate::sneak::Man>,
    pub list: Vec<crate::sneak::Man>,
    pub live: bool,
    pub amb: f64,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            trail: Vec::new(),
            off: 0.0,
            t: 0.0,
            men: JsMap::new(),
            list: Vec::new(),
            live: false,
            amb: 0.0,
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Exp, src/game/sneak.ts:267)
#[derive(Clone, Copy, Debug)]
pub struct Exp {
    pub seen: bool,
    pub watched: bool,
    pub near: bool,
    pub by: Option<GuardId>,
}
impl Exp {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Exp {
            seen: false,
            watched: false,
            near: false,
            by: None,
        }
    }
}
impl Default for Exp { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SneakWalk, src/game/sneak.ts:336)
#[derive(Clone, Copy, Debug)]
pub struct SneakWalk {
    pub by: Option<GuardId>,
}
impl SneakWalk {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SneakWalk {
            by: None,
        }
    }
}
impl Default for SneakWalk { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Feet, src/game/sneak.ts:441)
#[derive(Clone, Copy, Debug)]
pub struct Feet {
    pub goal: Option<V3>,
    pub speed: f64,
    pub crouch: bool,
    pub must: Option<bool>,
}
impl Feet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Feet {
            goal: None,
            speed: 0.0,
            crouch: false,
            must: None,
        }
    }
}
impl Default for Feet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sneak.ts:154 SNEAK_LOG, src/game/sneak.ts:154)
#[derive(Clone, Debug)]
pub struct SneakModSNEAKLOGItem {
    pub t: f64,
    pub who: String,
    pub what: String,
    pub at: [f64; 2],
}
impl SneakModSNEAKLOGItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SneakModSNEAKLOGItem {
            t: 0.0,
            who: String::new(),
            what: String::new(),
            at: [0.0; 2],
        }
    }
}
impl Default for SneakModSNEAKLOGItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sneak.ts:157 LINES, src/game/sneak.ts:157)
#[derive(Clone, Debug)]
pub struct LINES {
    pub wait: crate::sneak::LINESWait,
    pub nag: crate::sneak::LINESNag,
    pub go: String,
}
impl LINES {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LINES {
            wait: crate::sneak::LINESWait::zeroed(),
            nag: crate::sneak::LINESNag::zeroed(),
            go: String::new(),
        }
    }
}
impl Default for LINES { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sneak.ts:158 LINES.wait, src/game/sneak.ts:158)
#[derive(Clone, Debug)]
pub struct LINESWait {
    pub lit: String,
    pub near: String,
    pub long: String,
    pub open: String,
    pub door: String,
}
impl LINESWait {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LINESWait {
            lit: String::new(),
            near: String::new(),
            long: String::new(),
            open: String::new(),
            door: String::new(),
        }
    }
}
impl Default for LINESWait { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sneak.ts:159 LINES.nag, src/game/sneak.ts:159)
#[derive(Clone, Debug)]
pub struct LINESNag {
    pub lit: String,
    pub near: String,
    pub long: String,
    pub open: String,
    pub door: String,
}
impl LINESNag {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LINESNag {
            lit: String::new(),
            near: String::new(),
            long: String::new(),
            open: String::new(),
            door: String::new(),
        }
    }
}
impl Default for LINESNag { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sneak.ts:165 LINES_NOW, src/game/sneak.ts:165)
#[derive(Clone, Debug)]
pub struct LINESNOW {
    pub wait: crate::sneak::LINESNOWWait,
    pub nag: crate::sneak::LINESNOWNag,
}
impl LINESNOW {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LINESNOW {
            wait: crate::sneak::LINESNOWWait::zeroed(),
            nag: crate::sneak::LINESNOWNag::zeroed(),
        }
    }
}
impl Default for LINESNOW { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sneak.ts:165 LINESNOW.wait, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct LINESNOWWait {
    pub door: Option<String>,
    pub near: Option<String>,
    pub open: Option<String>,
    pub lit: Option<String>,
    pub long: Option<String>,
}
impl LINESNOWWait {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LINESNOWWait {
            door: None,
            near: None,
            open: None,
            lit: None,
            long: None,
        }
    }
}
impl Default for LINESNOWWait { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sneak.ts:165 LINESNOW.nag, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct LINESNOWNag {
    pub door: Option<String>,
    pub near: Option<String>,
    pub open: Option<String>,
    pub lit: Option<String>,
    pub long: Option<String>,
}
impl LINESNOWNag {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LINESNOWNag {
            door: None,
            near: None,
            open: None,
            lit: None,
            long: None,
        }
    }
}
impl Default for LINESNOWNag { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sneak.ts:188 leafT, src/game/sneak.ts:188)
#[derive(Clone, Copy, Debug)]
pub struct SneakModLeafTValue {
    pub a: f64,
    pub t: f64,
}
impl SneakModLeafTValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SneakModLeafTValue {
            a: 0.0,
            t: 0.0,
        }
    }
}
impl Default for SneakModLeafTValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sneak.ts:199 liveAt, src/game/sneak.ts:199)
#[derive(Clone, Copy, Debug)]
pub struct SneakModLiveAtValue {
    pub t: f64,
    pub v: bool,
}
impl SneakModLiveAtValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SneakModLiveAtValue {
            t: 0.0,
            v: false,
        }
    }
}
impl Default for SneakModLiveAtValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sneak.ts:325 keptLines, src/game/sneak.ts:325)
#[derive(Clone, Debug)]
pub struct SneakModKeptLinesValue {
    pub x: f64,
    pub z: f64,
    pub t: f64,
    pub low: bool,
    pub map: JsMap<crate::Num, f64>,
}
impl SneakModKeptLinesValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SneakModKeptLinesValue {
            x: 0.0,
            z: 0.0,
            t: 0.0,
            low: false,
            map: JsMap::new(),
        }
    }
}
impl Default for SneakModKeptLinesValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sneak.ts:189 leafMoved(d), src/game/sneak.ts:189)
#[derive(Clone, Copy, Debug)]
pub struct LeafMovedD {
    pub angle: f64,
}
impl LeafMovedD {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LeafMovedD {
            angle: 0.0,
        }
    }
}
impl Default for LeafMovedD { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SneakMod {
    /// (src/game/sneak.ts:154 SNEAK_LOG)
    pub sneak_log: Vec<crate::sneak::SneakModSNEAKLOGItem>,
    /// (src/game/sneak.ts:188 leafT)
    pub leaf_t: JsMap<DoorId, crate::sneak::SneakModLeafTValue>,
    /// (src/game/sneak.ts:193 states)
    pub states: Option<crate::sneak::State>,
    /// (src/game/sneak.ts:199 liveAt)
    pub live_at: Option<crate::sneak::SneakModLiveAtValue>,
    /// (src/game/sneak.ts:324 cellLit)
    pub cell_lit: Option<JsMap<crate::Num, [f64; 2]>>,
    /// (src/game/sneak.ts:325 keptLines)
    pub kept_lines: JsMap<GuardId, crate::sneak::SneakModKeptLinesValue>,
}
impl SneakMod {
    pub fn blank() -> Self {
        SneakMod {
            sneak_log: Vec::new(),
            leaf_t: JsMap::new(),
            states: None,
            live_at: None,
            cell_lit: None,
            kept_lines: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct SneakStats {
    pub waits: i32,
    pub wait_s: i32,
    pub crossings: i32,
    pub shade: i32,
    pub back_offs: i32,
    pub asides: i32,
    pub cues: i32,
    pub noise_walks: i32,
}

/// (src/game/sneak.ts:143 SNEAK_DEFAULTS: a table nothing writes)
pub fn sneak_defaults() -> crate::knobs::SneakK { crate::knobs::page_sneak() }

/// (src/game/sneak.ts:144 LS)
pub const LS: &str = "twopointsix.sneak";

/// (src/game/sneak.ts:146 KEPT: a table nothing writes)
pub fn kept() -> [String; 11] { [String::from("on"), String::from("crouch"), String::from("post"), String::from("trail"), String::from("look"), String::from("stand"), String::from("side"), String::from("door"), String::from("cue"), String::from("patience"), String::from("skill")] }

/// (src/game/sneak.ts:157 LINES: a table nothing writes)
pub fn lines() -> crate::sneak::LINES { crate::sneak::LINES { wait: crate::sneak::LINESWait { lit: String::from("holding. it's lit and he's looking."), near: String::from("holding. too close while he's looking."), long: String::from("holding. too much open light to cross."), open: String::from("holding in the dark. you're stood in the light."), door: String::from("i'm off the door. ready when you are.") }, nag: crate::sneak::LINESNag { lit: String::from("can't cross while it's lit. he's still on it."), near: String::from("he's still facing me. waiting for his back."), long: String::from("it's a long lit stretch. kill the light or go on."), open: String::from("you're stood in the light. i'll hold in the dark."), door: String::from("") }, go: String::from("moving.") } }

/// (src/game/sneak.ts:165 LINES_NOW: a table nothing writes)
pub fn lines_now() -> crate::sneak::LINESNOW { crate::sneak::LINESNOW { wait: crate::sneak::LINESNOWWait { door: None, near: None, open: Some(String::from("holding in the dark. you're standing in the light.")), lit: None, long: None }, nag: crate::sneak::LINESNOWNag { door: None, near: Some(String::from("he's still facing me. waiting for him to turn.")), open: Some(String::from("you're standing in the light. i'll hold in the dark.")), lit: None, long: None } } }

/// (src/game/sneak.ts:195 DEG)
pub const DEG: f64 = 0.017453292519943295;

/// (src/game/sneak.ts:217 POSTED: a table nothing writes)
pub fn posted() -> Vec<String> { alloc::vec![String::from("move"), String::from("cover")] }

/// (src/game/sneak.ts:251 LOW_Y: a table nothing writes)
pub fn low_y() -> Vec<f64> { alloc::vec![0.81, 0.98] }

/// (src/game/sneak.ts:251 UP_Y: a table nothing writes)
pub fn up_y() -> Vec<f64> { alloc::vec![1.27, 1.5] }

/// (src/game/sneak.ts:268 NONE: a table nothing writes)
pub fn none() -> crate::sneak::Exp { crate::sneak::Exp { seen: false, watched: false, near: false, by: None } }

/// (src/game/sneak.ts:297 MAP_SHORT)
pub const MAP_SHORT: f64 = 0.05;

/// (src/game/sneak.ts:299 BYEYE_BREAK)
pub const BYEYE_BREAK: bool = false;
