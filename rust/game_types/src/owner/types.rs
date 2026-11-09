//! WHOSE MAN HE IS: the taskers that may have a man, as a set.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// A tasker that may have a man, or several: a bit each, the higher the stronger. They are or-ed (`by::PRESS | by::WALK`).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, crate::bytes::ToBytes)]
pub struct By(pub u32);
impl core::ops::BitOr for By { type Output = By; fn bitor(self, o: By) -> By { By(self.0 | o.0) } }
impl By {
    /// no tasker: below every one
    pub const NOBODY: By = By(0);
    /// the strongest of them, 1 the weakest tasker; 0 nobody
    pub const fn rank(self) -> u32 { 32 - self.0.leading_zeros() }
    pub const fn any(self, o: By) -> bool { self.0 & o.0 != 0 }
}

/// A tasker: its bit, its name, and the reasons it may give for having him (the number handed to `claim` is a place in this list).
pub struct Tasker { pub by: By, pub name: &'static str, pub why: &'static [&'static str] }

macro_rules! taskers {
    ($($n:ident: $name:literal, [$($why:literal),+];)*) => {
        /// each tasker by name
        pub mod by {
            use super::By;
            #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
            enum At { $($n),*, COUNT }
            $(pub const $n: By = By(1 << (At::COUNT as u32 - 1 - At::$n as u32));)*
        }
        /// THE TASKERS, strongest first
        pub const TASKERS: &[Tasker] = &[$(Tasker { by: by::$n, name: $name, why: &[$($why),+] }),*];
    };
}

// THE LIST. The order of the lines is the rank: the strongest that has a hand on him is who has him, and a tasker may take a man only if
// none above it has him (`kept`). To add one: a line here at its rank, `claim` and `release` where it takes and lets go, and `kept` before it takes.
taskers! {
    FRAG: "a live grenade", ["a live grenade he knows of has a line to him"];
    RUNNER: "broke and ran", ["has said he is done", "dropped it and ran"];
    MORALE: "shaken by a blast", ["a blast has him ducked or broken"];
    STARTLE: "startled", ["caught alone at first sight, running for cover"];
    MISSION: "the mission's man", ["the mission's script holds him where he is"];
    LAST_MAN: "the last man", ["the last man up, at his act"];
    DIG: "digging in", ["on his way to his cover, or at a door with a wedge"];
    SQUAD_LEG: "sent by his squad", ["his squad has sent him, or is about to"];
    SQUAD_WORD: "his squad's word", ["his squad has heard and not yet said what it does, or has him going round or looking"];
    LANE: "a covered lane", ["held short of a covered lane, or looking round its corner"];
    DUG: "dug in", ["set on his door"];
    WALK: "a silent mate", ["walking to the post of a mate who does not answer, nothing learned of the team by it"];
    BREAKER: "the lights", ["out for the lights, or waiting to be sent"];
    SQUAD: "his squad", ["at rest, his squad's call"];
    PRESS: "the press", ["the press has him, or he stays to keep the room it leaves"];
    NEEDS: "a need of his own", ["off for a coffee, a smoke, some water or a stretch"];
}

#[derive(Clone, Debug, Default)]
pub struct OwnerModWITNESSESItem {
    pub rule: f64,
    pub name: String,
    pub says: crate::FnField,
    pub at_read: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct OwnerMod {
    /// (src/game/owner.ts:108 WITNESSES)
    pub witnesses: Vec<crate::owner::OwnerModWITNESSESItem>,
}
impl OwnerMod {
    pub fn blank() -> Self {
        OwnerMod {
            witnesses: Vec::new(),
        }
    }
}
