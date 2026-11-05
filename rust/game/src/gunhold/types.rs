//! The types of `gunhold`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Known, src/game/gunhold.ts:81)
#[derive(Clone, Debug)]
pub struct Known {
    pub gun_t: f64,
    pub shot_t: f64,
    pub sam: V3,
    pub facing: f64,
    pub fix_t: f64,
    pub gone_t: f64,
    pub expo: JsMap<crate::Num, f64>,
    pub expo_at: V3,
    pub expo_t: f64,
    pub talker: Option<GuardId>,
    pub said_t: f64,
    pub said_gun: bool,
    pub said_shot: bool,
}
impl Known {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Known {
            gun_t: 0.0,
            shot_t: 0.0,
            sam: [0.0; 3],
            facing: 0.0,
            fix_t: 0.0,
            gone_t: 0.0,
            expo: JsMap::new(),
            expo_at: [0.0; 3],
            expo_t: 0.0,
            talker: None,
            said_t: 0.0,
            said_gun: false,
            said_shot: false,
        }
    }
}
impl Default for Known { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Phase {
    Back,
    Run,
    Cover,
    Line,
    Plan,
    React,
    Peek,
    Flank,
}
impl Phase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Phase::Back => "back",
            Phase::Run => "run",
            Phase::Cover => "cover",
            Phase::Line => "line",
            Phase::Plan => "plan",
            Phase::React => "react",
            Phase::Peek => "peek",
            Phase::Flank => "flank",
        }
    }
}

/// (TypeScript: St, src/game/gunhold.ts:85)
#[derive(Clone, Copy, Debug)]
pub struct St {
    pub phase: crate::gunhold::Phase,
    pub go_t: f64,
    pub spot: Option<V3>,
    pub low: bool,
    pub edge: Option<V3>,
    pub goal: Option<V3>,
    pub plan_sam: Option<V3>,
    pub plan_facing: f64,
    pub plan_t: f64,
    pub peek_t: f64,
    pub peek_end: f64,
    pub peek_from: f64,
    pub check_t: f64,
    pub run_t: f64,
    pub line_t: f64,
    pub retry_t: f64,
    pub cover_bad: bool,
    pub shown_t: f64,
    pub bad: Option<V3>,
    pub said_round: bool,
}
impl St {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        St {
            phase: crate::gunhold::Phase::Back,
            go_t: 0.0,
            spot: None,
            low: false,
            edge: None,
            goal: None,
            plan_sam: None,
            plan_facing: 0.0,
            plan_t: 0.0,
            peek_t: 0.0,
            peek_end: 0.0,
            peek_from: 0.0,
            check_t: 0.0,
            run_t: 0.0,
            line_t: 0.0,
            retry_t: 0.0,
            cover_bad: false,
            shown_t: 0.0,
            bad: None,
            said_round: false,
        }
    }
}
impl Default for St { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: GunHoldStats, src/game/gunhold.ts:86)
#[derive(Clone, Copy, Debug)]
pub struct GunHoldStats {
    pub armed: f64,
    pub ran: f64,
    pub no_cover: f64,
    pub rounds: f64,
    pub no_way: f64,
    pub looks: f64,
    pub gone: f64,
    pub lines: f64,
    pub free_t: f64,
}
impl GunHoldStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GunHoldStats {
            armed: 0.0,
            ran: 0.0,
            no_cover: 0.0,
            rounds: 0.0,
            no_way: 0.0,
            looks: 0.0,
            gone: 0.0,
            lines: 0.0,
            free_t: 0.0,
        }
    }
}
impl Default for GunHoldStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunhold.ts:90 GUNHOLD_DEBUG, src/game/gunhold.ts:90)
#[derive(Clone, Copy, Debug)]
pub struct GUNHOLDDEBUG {
}
impl GUNHOLDDEBUG {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GUNHOLDDEBUG {
        }
    }
}
impl Default for GUNHOLDDEBUG { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunhold.ts:262 LINES, src/game/gunhold.ts:262)
#[derive(Clone, Debug)]
pub struct LINES {
    pub gun: Vec<String>,
    pub shot: Vec<String>,
    pub round: Vec<String>,
    pub gone: Vec<String>,
}
impl LINES {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LINES {
            gun: Vec::new(),
            shot: Vec::new(),
            round: Vec::new(),
            gone: Vec::new(),
        }
    }
}
impl Default for LINES { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunhold.ts:117 partOf ->, src/game/gunhold.ts:117)
#[derive(Clone, Debug)]
pub struct PartOf {
    pub k: crate::gunhold::Known,
    pub st: crate::gunhold::St,
}
impl PartOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PartOf {
            k: crate::gunhold::Known::zeroed(),
            st: crate::gunhold::St::zeroed(),
        }
    }
}
impl Default for PartOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunhold.ts:177 wayTo ->, src/game/gunhold.ts:177)
#[derive(Clone, Debug)]
pub struct WayTo {
    pub path: Vec<V3>,
    pub len: f64,
    pub show: f64,
}
impl WayTo {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WayTo {
            path: Vec::new(),
            len: 0.0,
            show: 0.0,
        }
    }
}
impl Default for WayTo { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunhold.ts:190 pickCover ->, src/game/gunhold.ts:190)
#[derive(Clone, Copy, Debug)]
pub struct PickCover {
    pub spot: V3,
    pub low: bool,
    pub edge: Option<V3>,
}
impl PickCover {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PickCover {
            spot: [0.0; 3],
            low: false,
            edge: None,
        }
    }
}
impl Default for PickCover { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GunHoldPoolKind {
    Shot,
    Round,
    Gone,
    Gun,
}
impl GunHoldPoolKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            GunHoldPoolKind::Shot => "shot",
            GunHoldPoolKind::Round => "round",
            GunHoldPoolKind::Gone => "gone",
            GunHoldPoolKind::Gun => "gun",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BarkKind {
    Shot,
    Round,
    Gone,
    Gun,
}
impl BarkKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            BarkKind::Shot => "shot",
            BarkKind::Round => "round",
            BarkKind::Gone => "gone",
            BarkKind::Gun => "gun",
        }
    }
}

/// (TypeScript: inline at src/game/gunhold.ts:297 gunHoldFeet(fs), src/game/gunhold.ts:297)
#[derive(Clone, Copy, Debug)]
pub struct GunHoldFeetFs {
    pub ok: bool,
    pub rear: f64,
    pub clear: f64,
}
impl GunHoldFeetFs {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GunHoldFeetFs {
            ok: false,
            rear: 0.0,
            clear: 0.0,
        }
    }
}
impl Default for GunHoldFeetFs { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunhold.ts:297 gunHoldFeet ->, src/game/gunhold.ts:297)
#[derive(Clone, Copy, Debug)]
pub struct GunHoldFeet {
    pub move_: f64,
    pub run: bool,
    pub look: Option<V3>,
    pub crouch: f64,
    pub quiet: bool,
}
impl GunHoldFeet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GunHoldFeet {
            move_: 0.0,
            run: false,
            look: None,
            crouch: 0.0,
            quiet: false,
        }
    }
}
impl Default for GunHoldFeet { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct GunholdMod {
    /// (src/game/gunhold.ts:88 knowns)
    pub knowns: JsMap<u32, crate::gunhold::Known>,
    /// (src/game/gunhold.ts:88 states)
    pub states: JsMap<GuardsStandoffId, crate::gunhold::St>,
    /// (src/game/gunhold.ts:88 statsOf)
    pub stats_of: Option<crate::gunhold::GunHoldStats>,
    /// (src/game/gunhold.ts:88 planned)
    pub planned: Option<f64>,
}
impl GunholdMod {
    pub fn blank() -> Self {
        GunholdMod {
            knowns: JsMap::new(),
            states: JsMap::new(),
            stats_of: None,
            planned: None,
        }
    }
}

/// (src/game/gunhold.ts:76 GUNHOLD_DEFAULTS: a table nothing writes)
pub fn gunhold_defaults() -> crate::knobs::GunholdK { crate::knobs::page_gunhold() }

/// (src/game/gunhold.ts:90 GUNHOLD_DEBUG: a table nothing writes)
pub fn gunhold_debug() -> crate::gunhold::GUNHOLDDEBUG { crate::gunhold::GUNHOLDDEBUG {  } }

/// (src/game/gunhold.ts:111 HEAR_M)
pub const HEAR_M: f64 = 7.0;

/// (src/game/gunhold.ts:111 FRESH_S)
pub const FRESH_S: f64 = 1.0;

/// (src/game/gunhold.ts:143 GUN_Y)
pub const GUN_Y: f64 = 1.35;

/// (src/game/gunhold.ts:143 PISTOL_ENERGY)
pub const PISTOL_ENERGY: f64 = 0.6;

/// (src/game/gunhold.ts:228 AIM_Y)
pub const AIM_Y: f64 = 1.42;

/// (src/game/gunhold.ts:262 LINES: a table nothing writes)
pub fn lines() -> crate::gunhold::LINES { crate::gunhold::LINES { gun: alloc::vec![String::from("gun — he's got a gun! get behind something!"), String::from("he's armed — cover, get to cover!")], shot: alloc::vec![String::from("he's shooting — out of his line, move!"), String::from("shots! get off his line — go round him!")], round: alloc::vec![String::from("going round him — keep him talking"), String::from("I'll get behind him. keep his eyes on you")], gone: alloc::vec![String::from("he's moved — they're not there!")] } }

/// (src/game/gunhold.ts:273 GUNHOLD_DEFENDER_LINES: a table nothing writes)
pub fn gunhold_defender_lines() -> JsMap<String, Vec<String>> { { let mut t = JsMap::new(); t.set(String::from("gun"), alloc::vec![String::from("pistol's out behind his shield — get to cover!"), String::from("he's drawn from the hold — get behind something!")]); t.set(String::from("shot"), alloc::vec![String::from("he's firing past his shield — off that line, move!"), String::from("rounds from the hold — get off it, go round him!")]); t.set(String::from("round"), alloc::vec![String::from("going round behind him — keep his eyes on you"), String::from("coming at his back. hold him where he is")]); t.set(String::from("gone"), alloc::vec![String::from("he's moved — nobody where we left them!")]); t } }

/// (src/game/gunhold.ts:280 GUNHOLD_NOW: a table nothing writes)
pub fn gunhold_now() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("shots! get off his line — go round him!"), String::from("shots! get out of his line of fire — go around him!")); t.set(String::from("going round him — keep him talking"), String::from("going around him — keep him talking")); t.set(String::from("he\\s drawn from the hold — get behind something!"), String::from("he's pulled a pistol — get behind something!")); t.set(String::from("rounds from the hold — get off it, go round him!"), String::from("he's shooting — get out of his line of fire, go around him!")); t.set(String::from("going round behind him — keep his eyes on you"), String::from("going around behind him — keep his eyes on you")); t.set(String::from("coming at his back. hold him where he is"), String::from("coming up behind him. keep him where he is")); t.set(String::from("he\\s moved — nobody where we left them!"), String::from("he's moved — they're not where we left them!")); t } }
