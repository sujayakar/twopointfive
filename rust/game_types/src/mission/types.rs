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
    /// its foes are its own to lead: no rule of a garrison has them
    pub leads_foes: bool,
    /// the level's own team may come for it as a raid
    pub raid: bool,
}

/// THE MISSION THAT IS UP, AS IT SAYS OF ITSELF: whoever stages one writes it, whoever wants a fact of a mission reads it here. No rule should ask which mission it is.
#[derive(Clone, Debug, ToBytes, Default)]
pub struct Staged {
    /// its kind, for the cards and the lists (None: the level's own night)
    pub up: Option<crate::scene_scenarios::MissionKind>,
    pub facts: crate::mission::Facts,
    /// the teammates it staged, dead or alive
    pub team: Vec<GuardId>,
    /// the men it staged against them, dead or alive
    pub foes: Vec<GuardId>,
    /// the people its foes hold: nobody fires or throws where they would be hurt
    pub captives: Vec<GuardId>,
    /// the rooms it is fought for, each with who holds it
    pub rooms: Vec<crate::roomclear::RoomState>,
    /// which of the level's ways in the team came by
    pub way: f64,
    /// the wall charges it hands out, where it hands out any
    pub walls: Option<f64>,
    /// it has ended, and stays so
    pub over: bool,
    /// the captives who lived, as it ended
    pub saved: Option<f64>,
}
impl Staged {
    pub fn begin(&mut self, kind: crate::scene_scenarios::MissionKind, facts: crate::mission::Facts) {
        self.up = Some(kind);
        self.facts = facts;
    }
    /// a man of its foes that it leads itself
    pub fn leads(&self, gd: GuardId) -> bool { self.facts.leads_foes && self.foes.contains(&gd) }
    /// It is no mission's floor any more. (Its men go by whoever staged them: their lists stand)
    pub fn down(&mut self) {
        let was = core::mem::take(self);
        *self = Staged { team: was.team, foes: was.foes, captives: was.captives, rooms: was.rooms, ..Staged::default() };
    }
}
