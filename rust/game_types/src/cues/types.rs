//! The types of `cues`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Words)]
pub enum CueKind {
    #[default] Calm,
    Cover,
    Flinch,
    Still,
    HuntOver,
    FellBack,
    Planted,
    PinnedIn,
    SquadHears,
    SquadCopies,
    SquadWaits,
    SquadCopiesWaits,
    SquadStays,
    SquadQuiet,
    Surprised,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CueRow {
    pub t: f64,
    pub text: String,
    pub cls: f64,
    pub rank: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Bubble {
    pub text: String,
    pub t: f64,
    pub dur: f64,
    pub radio: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Man {
    pub state: crate::game::GuardState,
    pub alert_t: f64,
    pub said_t: f64,
    pub again_t: f64,
    pub stood_t: f64,
    pub moved: Option<crate::cues::ManMoved>,
    pub from: String,
    pub covered: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes)]
pub enum ManMoved {
    Cover,
    Fallback,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Cues {
    pub rows: Vec<crate::cues::CueRow>,
    #[is(crate::cues::CuesStats { heard: 0.0, heard_drawn: 0.0, shouts: 0.0, shouts_drawn: 0.0, cues: 0.0, cue_net: 0.0, cue_drawn: 0.0, dup: 0.0, gap: 0.0, full: 0.0, bumped: 0.0, bumped_young: 0.0, filed: alloc::vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], drawn: alloc::vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] })] pub stats: crate::cues::CuesStats,
    #[is(crate::cues::CuesKinds { calm: None, cover: None, flinch: None, still: None, hunt_over: None, fell_back: None, planted: None, pinned_in: None, squad_hears: None, squad_copies: None, squad_waits: None, squad_copies_waits: None, squad_stays: None, squad_quiet: None, surprised: None })] pub kinds: crate::cues::CuesKinds,
    pub log: Vec<crate::cues::CuesLogItem>,
    pub bubbles: JsMap<GuardId, crate::cues::Bubble>,
    pub men: JsMap<GuardId, crate::cues::Man>,
    pub last_note: Option<crate::roomfight::FightNote>,
    #[is(-1.0)] pub last_t: f64,
    pub said: JsMap<String, f64>,
    pub class_t: Vec<Option<f64>>,
    pub squad_t: Vec<Option<f64>>,
    #[is(crate::cues::CuesKindT { calm: None, cover: None, flinch: None, still: None, hunt_over: None, fell_back: None, planted: None, pinned_in: None, squad_hears: None, squad_copies: None, squad_waits: None, squad_copies_waits: None, squad_stays: None, squad_quiet: None, surprised: None })] pub kind_t: crate::cues::CuesKindT,
    pub n: f64,
    pub squelch_due: bool,
    pub heard_row: Option<crate::cues::CuesHeardRow>,
    pub heard_said: JsMap<String, f64>,
    pub shout_t: Option<f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CuesStats {
    pub heard: f64,
    pub heard_drawn: f64,
    pub shouts: f64,
    pub shouts_drawn: f64,
    pub cues: f64,
    pub cue_net: f64,
    pub cue_drawn: f64,
    pub dup: f64,
    pub gap: f64,
    pub full: f64,
    pub bumped: f64,
    pub bumped_young: f64,
    pub filed: Vec<f64>,
    pub drawn: Vec<f64>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CuesKinds {
    pub calm: Option<f64>,
    pub cover: Option<f64>,
    pub flinch: Option<f64>,
    pub still: Option<f64>,
    pub hunt_over: Option<f64>,
    pub fell_back: Option<f64>,
    pub planted: Option<f64>,
    pub pinned_in: Option<f64>,
    pub squad_hears: Option<f64>,
    pub squad_copies: Option<f64>,
    pub squad_waits: Option<f64>,
    pub squad_copies_waits: Option<f64>,
    pub squad_stays: Option<f64>,
    pub squad_quiet: Option<f64>,
    pub surprised: Option<f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CuesLogItem {
    pub why: String,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CuesKindT {
    pub calm: Option<f64>,
    pub cover: Option<f64>,
    pub flinch: Option<f64>,
    pub still: Option<f64>,
    pub hunt_over: Option<f64>,
    pub fell_back: Option<f64>,
    pub planted: Option<f64>,
    pub pinned_in: Option<f64>,
    pub squad_hears: Option<f64>,
    pub squad_copies: Option<f64>,
    pub squad_waits: Option<f64>,
    pub squad_copies_waits: Option<f64>,
    pub squad_stays: Option<f64>,
    pub squad_quiet: Option<f64>,
    pub surprised: Option<f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CuesHeardRow {
    pub t: f64,
    pub text: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CuesFileKind {
    Heard,
    Calm,
    Cover,
    Flinch,
    Still,
    Call,
    HuntOver,
    FellBack,
    Planted,
    PinnedIn,
    SquadHears,
    SquadCopies,
    SquadWaits,
    SquadCopiesWaits,
    SquadStays,
    SquadQuiet,
    Surprised,
    Raw,
}

#[derive(Clone, Debug, Default)]
pub struct CuesAt {
    pub the: String,
    pub at: String,
}

