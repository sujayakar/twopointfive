//! The types of `suppression`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Sup, src/game/suppression.ts:70)
#[derive(Clone, Copy, Debug)]
pub struct Sup {
    pub v: f64,
    pub kick: f64,
    pub src: Option<V3>,
    pub src_id: f64,
    pub src_t: f64,
    pub last_add: f64,
    pub flinch_at: f64,
    pub pinned_at: f64,
    pub band: f64,
    pub down_t: f64,
    pub latched: bool,
    pub pinned_t: f64,
    pub cover: Option<V3>,
    pub displaced_at: f64,
    pub displaces: f64,
    pub cover_until: f64,
    pub bark_t: f64,
    pub bark_was: Option<crate::suppression::SupMoment>,
    pub pending: Option<crate::suppression::SupMoment>,
    pub pending_t: f64,
    pub was_pinned: bool,
}
impl Sup {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Sup {
            v: 0.0,
            kick: 0.0,
            src: None,
            src_id: -1.0,
            src_t: -100.0,
            last_add: 0.0,
            flinch_at: -100.0,
            pinned_at: -100.0,
            band: 0.0,
            down_t: 0.0,
            latched: false,
            pinned_t: 0.0,
            cover: None,
            displaced_at: -100.0,
            displaces: 0.0,
            cover_until: -1.0,
            bark_t: -100.0,
            bark_was: None,
            pending: None,
            pending_t: -100.0,
            was_pinned: false,
        }
    }
}
impl Default for Sup { fn default() -> Self { Self::blank() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SupMoment {
    Flinch,
    Pinned,
    Friendly,
    Displace,
    Recover,
}
impl SupMoment {
    pub const fn as_str(self) -> &'static str {
        match self {
            SupMoment::Flinch => "flinch",
            SupMoment::Pinned => "pinned",
            SupMoment::Friendly => "friendly",
            SupMoment::Displace => "displace",
            SupMoment::Recover => "recover",
        }
    }
}

/// (TypeScript: SamPass, src/game/suppression.ts:104)
#[derive(Clone, Copy, Debug)]
pub struct SamPass {
    pub from: V3,
    pub to: V3,
    pub closest: V3,
    pub travelled: f64,
    pub d_near: f64,
    pub d_hit: f64,
    pub add: f64,
    pub shooter_id: f64,
}
impl SamPass {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SamPass {
            from: [0.0; 3],
            to: [0.0; 3],
            closest: [0.0; 3],
            travelled: 0.0,
            d_near: 0.0,
            d_hit: 0.0,
            add: 0.0,
            shooter_id: 0.0,
        }
    }
}
impl Default for SamPass { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/suppression.ts:99 SUPPRESSION_DEBUG, src/game/suppression.ts:99)
#[derive(Clone, Copy, Debug)]
pub struct SUPPRESSIONDEBUG {
}
impl SUPPRESSIONDEBUG {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SUPPRESSIONDEBUG {
        }
    }
}
impl Default for SUPPRESSIONDEBUG { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/suppression.ts:105 SUPPRESSION_FX, src/game/suppression.ts:105)
#[derive(Clone, Copy, Debug)]
pub struct SUPPRESSIONFX {
    pub fx_range: f64,
}
impl SUPPRESSIONFX {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SUPPRESSIONFX {
            fx_range: 0.0,
        }
    }
}
impl Default for SUPPRESSIONFX { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HeadDown {
    Down,
    None,
    Displace,
}
impl HeadDown {
    pub const fn as_str(self) -> &'static str {
        match self {
            HeadDown::Down => "down",
            HeadDown::None => "none",
            HeadDown::Displace => "displace",
        }
    }
}

/// (TypeScript: inline at src/game/suppression.ts:298 guardFireMods ->, src/game/suppression.ts:298)
#[derive(Clone, Copy, Debug)]
pub struct GuardFireMods {
    pub spread: f64,
    pub gap: f64,
}
impl GuardFireMods {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardFireMods {
            spread: 0.0,
            gap: 0.0,
        }
    }
}
impl Default for GuardFireMods { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SuppressionMod {
    /// (src/game/suppression.ts:144 floorLineT)
    pub floor_line_t: Option<f64>,
    /// (src/game/suppression.ts:144 floorBarkT)
    pub floor_bark_t: Option<f64>,
    /// (src/game/suppression.ts:144 lastLine)
    pub last_line: JsMap<GuardId, String>,
}
impl SuppressionMod {
    pub fn blank() -> Self {
        SuppressionMod {
            floor_line_t: None,
            floor_bark_t: None,
            last_line: JsMap::new(),
        }
    }
}

/// (src/game/suppression.ts:99 SUPPRESSION_DEBUG: a table nothing writes)
pub fn suppression_debug() -> crate::suppression::SUPPRESSIONDEBUG { crate::suppression::SUPPRESSIONDEBUG {  } }

/// (src/game/suppression.ts:105 SUPPRESSION_FX: a table nothing writes)
pub fn suppression_fx() -> crate::suppression::SUPPRESSIONFX { crate::suppression::SUPPRESSIONFX { fx_range: 4.0 } }

/// (src/game/suppression.ts:149 RANK: a table nothing writes)
pub fn rank() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("recover"), 0.0); t.set(String::from("friendly"), 1.0); t.set(String::from("flinch"), 1.0); t.set(String::from("pinned"), 2.0); t.set(String::from("displace"), 3.0); t } }

/// (src/game/suppression.ts:151 SUP_LINES: a table nothing writes)
pub fn sup_lines() -> JsMap<String, JsMap<String, Option<Vec<String>>>> { { let mut t = JsMap::new(); t.set(String::from("flinch"), { let mut t = JsMap::new(); t.set(String::from("*"), Some(alloc::vec![String::from("taking fire!"), String::from("rounds coming in!"), String::from("he's shooting — get down!"), String::from("that was close—")])); t.set(String::from("nervy"), Some(alloc::vec![String::from("christ— CHRIST—"), String::from("he's shooting at me!")])); t.set(String::from("hard"), Some(alloc::vec![String::from("he's found his nerve."), String::from("incoming. stay on him.")])); t }); t.set(String::from("friendly"), { let mut t = JsMap::new(); t.set(String::from("*"), Some(alloc::vec![String::from("watch your fire!"), String::from("check fire — that's past MY head!"), String::from("easy! I'm in front of you!")])); t.set(String::from("nervy"), Some(alloc::vec![String::from("stop— stop shooting past me!")])); t }); t.set(String::from("pinned"), { let mut t = JsMap::new(); t.set(String::from("*"), Some(alloc::vec![String::from("I'm pinned!"), String::from("can't get my head up—"), String::from("he's got me pinned here!"), String::from("head down— head down!")])); t.set(String::from("nervy"), Some(alloc::vec![String::from("I can't— I can't move—"), String::from("jesus, he's right on me—")])); t.set(String::from("hard"), Some(alloc::vec![String::from("he's got the angle. wait him out."), String::from("let him spend them.")])); t }); t.set(String::from("displace"), { let mut t = JsMap::new(); t.set(String::from("*"), Some(alloc::vec![String::from("moving!"), String::from("shifting — cover me!"), String::from("can't stay here — moving!")])); t.set(String::from("nervy"), Some(alloc::vec![String::from("I'm getting out of here—")])); t.set(String::from("hard"), Some(alloc::vec![String::from("relocating.")])); t }); t.set(String::from("recover"), { let mut t = JsMap::new(); t.set(String::from("hard"), Some(alloc::vec![String::from("that all you've got?"), String::from("my turn.")])); t }); t } }
