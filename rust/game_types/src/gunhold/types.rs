//! The types of `gunhold`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default)]
pub enum Phase {
    #[default] Back,
    Run,
    Cover,
    Plan,
    Line,
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
            Phase::Plan => "plan",
            Phase::Line => "line",
            Phase::React => "react",
            Phase::Peek => "peek",
            Phase::Flank => "flank",
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct GUNHOLDDEBUG {
}

#[derive(Clone, Debug, Default)]
pub struct LINES {
    pub gun: Vec<String>,
    pub shot: Vec<String>,
    pub round: Vec<String>,
    pub gone: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct PartOf {
    pub k: crate::gunhold::Known,
    pub st: crate::gunhold::St,
}

#[derive(Clone, Debug, Default)]
pub struct WayTo {
    pub path: Vec<V3>,
    pub len: f64,
    pub show: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PickCover {
    pub spot: V3,
    pub low: bool,
    pub edge: Option<V3>,
}

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

#[derive(Clone, Copy, Debug, Default)]
pub struct GunHoldFeetFs {
    pub ok: bool,
    pub rear: f64,
    pub clear: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GunHoldFeet {
    pub move_: f64,
    pub run: bool,
    pub look: Option<V3>,
    pub crouch: f64,
    pub quiet: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
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
            // (there from the start, with `free_t` at −1, on every host: asking for it, a HUD's asking included, must not be what makes it)
            stats_of: Some(crate::gunhold::GunHoldStats { armed: 0.0, ran: 0.0, no_cover: 0.0, rounds: 0.0, no_way: 0.0, looks: 0.0, gone: 0.0, lines: 0.0, free_t: -1.0 }),
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
pub fn gunhold_now() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("shots! get off his line — go round him!"), String::from("shots! get out of his line of fire — go around him!")); t.set(String::from("going round him — keep him talking"), String::from("going around him — keep him talking")); t.set(String::from("he's drawn from the hold — get behind something!"), String::from("he's pulled a pistol — get behind something!")); t.set(String::from("rounds from the hold — get off it, go round him!"), String::from("he's shooting — get out of his line of fire, go around him!")); t.set(String::from("going round behind him — keep his eyes on you"), String::from("going around behind him — keep his eyes on you")); t.set(String::from("coming at his back. hold him where he is"), String::from("coming up behind him. keep him where he is")); t.set(String::from("he's moved — nobody where we left them!"), String::from("he's moved — they're not where we left them!")); t } }
