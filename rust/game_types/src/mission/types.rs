//! The types of `mission`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TallyK {
    Keys,
    Spotted,
    Shots,
    Kills,
    Bodies,
    Alerts,
    Knockouts,
}

/// (src/game/mission.ts:62 DOWN_CARD_SECS)
pub const DOWN_CARD_SECS: f64 = 2.6;

/// WHAT A MISSION BRINGS, said by whoever starts one. Nothing said is the level's own night.
#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Facts {
    /// it is a fight for a floor of its own making: the level's own night (its shift and civilians, the lenses, the radio, the team that comes) is off, and loud things are louder
    pub fight: bool,
    /// it is played unseen: the suit, the partner's downs on the card, his own cards
    pub unseen: bool,
    /// it goes by its own rule, and the stealth mission's stages (in, the drive, out) stand still under it
    pub own_rule: bool,
    /// its men are drilled, where Hard drills men
    pub drilled: bool,
    /// the level's own team may come for it as a raid
    pub raid: bool,
}

/// THE MISSION THAT IS UP, AS IT SAYS OF ITSELF: whoever stages one writes it, whoever wants a fact of a mission reads it here, and no rule asks which mission it is.
#[derive(Clone, Debug, ToBytes, Default)]
pub struct Staged {
    /// its kind, for the cards and the lists (None: the level's own night)
    pub up: Option<crate::scene_scenarios::MissionKind>,
    pub facts: crate::mission::Facts,
    /// the teammates it staged, dead or alive
    pub team: Vec<GuardId>,
    /// the men it staged against them, dead or alive
    pub foes: Vec<GuardId>,
    /// which of the level's ways in the team came by
    pub way: f64,
    /// the wall charges it hands out, where it hands out any
    pub walls: Option<f64>,
    /// it has ended, and stays so
    pub over: bool,
}
impl Staged {
    pub fn begin(&mut self, kind: crate::scene_scenarios::MissionKind, facts: crate::mission::Facts) {
        self.up = Some(kind);
        self.facts = facts;
    }
    /// It is no mission's floor any more. (Its men go by whoever staged them: their lists stand)
    pub fn down(&mut self) {
        let (team, foes) = (core::mem::take(&mut self.team), core::mem::take(&mut self.foes));
        *self = Staged { team, foes, ..Staged::default() };
    }
}
