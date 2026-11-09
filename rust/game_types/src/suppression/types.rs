//! The types of `suppression`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Blank)]
pub struct Sup {
    pub v: f64,
    pub kick: f64,
    pub src: Option<V3>,
    #[is(-1.0)] pub src_id: f64,
    #[is(-100.0)] pub src_t: f64,
    pub last_add: f64,
    #[is(-100.0)] pub flinch_at: f64,
    #[is(-100.0)] pub pinned_at: f64,
    pub band: f64,
    pub down_t: f64,
    pub latched: bool,
    pub pinned_t: f64,
    pub cover: Option<V3>,
    #[is(-100.0)] pub displaced_at: f64,
    pub displaces: f64,
    #[is(-1.0)] pub cover_until: f64,
    #[is(-100.0)] pub bark_t: f64,
    pub bark_was: Option<crate::suppression::SupMoment>,
    pub pending: Option<crate::suppression::SupMoment>,
    #[is(-100.0)] pub pending_t: f64,
    pub was_pinned: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words)]
pub enum SupMoment {
    Flinch,
    Friendly,
    Pinned,
    Displace,
    Recover,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HeadDown {
    Down,
    None,
    Displace,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GuardFireMods {
    pub spread: f64,
    pub gap: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct SuppressionMod {
    /// (src/game/suppression.ts:144 floorLineT)
    pub floor_line_t: Option<f64>,
    /// (src/game/suppression.ts:144 floorBarkT)
    pub floor_bark_t: Option<f64>,
    /// (src/game/suppression.ts:144 lastLine)
    pub last_line: JsMap<GuardId, String>,
}

/// (src/game/suppression.ts:149 RANK: a table nothing writes)
pub fn rank() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("recover"), 0.0); t.set(String::from("friendly"), 1.0); t.set(String::from("flinch"), 1.0); t.set(String::from("pinned"), 2.0); t.set(String::from("displace"), 3.0); t } }

/// (src/game/suppression.ts:151 SUP_LINES: a table nothing writes)
pub fn sup_lines() -> JsMap<String, JsMap<String, Option<Vec<String>>>> { { let mut t = JsMap::new(); t.set(String::from("flinch"), { let mut t = JsMap::new(); t.set(String::from("*"), Some(alloc::vec![String::from("taking fire!"), String::from("rounds coming in!"), String::from("he's shooting — get down!"), String::from("that was close—")])); t.set(String::from("nervy"), Some(alloc::vec![String::from("christ— CHRIST—"), String::from("he's shooting at me!")])); t.set(String::from("hard"), Some(alloc::vec![String::from("he's found his nerve."), String::from("incoming. stay on him.")])); t }); t.set(String::from("friendly"), { let mut t = JsMap::new(); t.set(String::from("*"), Some(alloc::vec![String::from("watch your fire!"), String::from("check fire — that's past MY head!"), String::from("easy! I'm in front of you!")])); t.set(String::from("nervy"), Some(alloc::vec![String::from("stop— stop shooting past me!")])); t }); t.set(String::from("pinned"), { let mut t = JsMap::new(); t.set(String::from("*"), Some(alloc::vec![String::from("I'm pinned!"), String::from("can't get my head up—"), String::from("he's got me pinned here!"), String::from("head down— head down!")])); t.set(String::from("nervy"), Some(alloc::vec![String::from("I can't— I can't move—"), String::from("jesus, he's right on me—")])); t.set(String::from("hard"), Some(alloc::vec![String::from("he's got the angle. wait him out."), String::from("let him spend them.")])); t }); t.set(String::from("displace"), { let mut t = JsMap::new(); t.set(String::from("*"), Some(alloc::vec![String::from("moving!"), String::from("shifting — cover me!"), String::from("can't stay here — moving!")])); t.set(String::from("nervy"), Some(alloc::vec![String::from("I'm getting out of here—")])); t.set(String::from("hard"), Some(alloc::vec![String::from("relocating.")])); t }); t.set(String::from("recover"), { let mut t = JsMap::new(); t.set(String::from("hard"), Some(alloc::vec![String::from("that all you've got?"), String::from("my turn.")])); t }); t } }
