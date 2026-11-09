//! The types of `cues`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum CueKind {
    #[default] Stay,
    Calm,
    Cover,
    Flinch,
    Still,
    Help,
    StayOurs,
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
impl CueKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            CueKind::Stay => "stay",
            CueKind::Calm => "calm",
            CueKind::Cover => "cover",
            CueKind::Flinch => "flinch",
            CueKind::Still => "still",
            CueKind::Help => "help",
            CueKind::StayOurs => "stayOurs",
            CueKind::HuntOver => "huntOver",
            CueKind::FellBack => "fellBack",
            CueKind::Planted => "planted",
            CueKind::PinnedIn => "pinnedIn",
            CueKind::SquadHears => "squadHears",
            CueKind::SquadCopies => "squadCopies",
            CueKind::SquadWaits => "squadWaits",
            CueKind::SquadCopiesWaits => "squadCopiesWaits",
            CueKind::SquadStays => "squadStays",
            CueKind::SquadQuiet => "squadQuiet",
            CueKind::Surprised => "surprised",
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CueRow {
    pub t: f64,
    pub text: String,
    pub cls: f64,
    pub rank: f64,
    pub who: String,
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
    pub noise_t: f64,
    pub ep: f64,
    pub rally_at: f64,
    pub covered: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes)]
pub enum ManMoved {
    Cover,
    Fallback,
}

#[derive(Clone, Debug, ToBytes)]
pub struct Cues {
    pub rows: Vec<crate::cues::CueRow>,
    pub stats: crate::cues::CuesStats,
    pub kinds: crate::cues::CuesKinds,
    pub log: Vec<crate::cues::CuesLogItem>,
    pub bubbles: JsMap<GuardId, crate::cues::Bubble>,
    pub men: JsMap<GuardId, crate::cues::Man>,
    pub last_note: Option<crate::roomfight::FightNote>,
    pub last_t: f64,
    pub said: JsMap<String, f64>,
    pub class_t: Vec<Option<f64>>,
    pub squad_t: Vec<Option<f64>>,
    pub kind_t: crate::cues::CuesKindT,
    pub n: f64,
    pub squelch_due: bool,
    pub heard_row: Option<crate::cues::CuesHeardRow>,
    pub heard_log: Vec<crate::cues::CuesHeardLogItem>,
    pub heard_said: JsMap<String, f64>,
    pub shout_t: Option<f64>,
}
impl Cues {
    /// what a new one holds
    pub fn blank() -> Self {
        Cues {
            rows: Vec::new(),
            stats: crate::cues::CuesStats { heard: 0.0, heard_drawn: 0.0, shouts: 0.0, shouts_drawn: 0.0, cues: 0.0, cue_net: 0.0, cue_drawn: 0.0, dup: 0.0, gap: 0.0, full: 0.0, bumped: 0.0, bumped_young: 0.0, filed: alloc::vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], drawn: alloc::vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] },
            kinds: crate::cues::CuesKinds { stay: None, calm: None, cover: None, flinch: None, still: None, help: None, stay_ours: None, hunt_over: None, fell_back: None, planted: None, pinned_in: None, squad_hears: None, squad_copies: None, squad_waits: None, squad_copies_waits: None, squad_stays: None, squad_quiet: None, surprised: None },
            log: Vec::new(),
            bubbles: JsMap::new(),
            men: JsMap::new(),
            last_note: None,
            last_t: -1.0,
            said: JsMap::new(),
            class_t: Vec::new(),
            squad_t: Vec::new(),
            kind_t: crate::cues::CuesKindT { stay: None, calm: None, cover: None, flinch: None, still: None, help: None, stay_ours: None, hunt_over: None, fell_back: None, planted: None, pinned_in: None, squad_hears: None, squad_copies: None, squad_waits: None, squad_copies_waits: None, squad_stays: None, squad_quiet: None, surprised: None },
            n: 0.0,
            squelch_due: false,
            heard_row: None,
            heard_log: Vec::new(),
            heard_said: JsMap::new(),
            shout_t: None,
        }
    }
}
impl Default for Cues { fn default() -> Self { Self::blank() } }

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
    pub stay: Option<f64>,
    pub calm: Option<f64>,
    pub cover: Option<f64>,
    pub flinch: Option<f64>,
    pub still: Option<f64>,
    pub help: Option<f64>,
    pub stay_ours: Option<f64>,
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
    pub t: f64,
    pub who: String,
    pub text: String,
    pub cls: f64,
    pub kind: crate::cues::CuesLogItemKind,
    pub drawn: bool,
    pub why: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum CuesLogItemKind {
    #[default] Heard,
    Stay,
    Calm,
    Cover,
    Flinch,
    Still,
    Call,
    Help,
    StayOurs,
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CuesKindT {
    pub stay: Option<f64>,
    pub calm: Option<f64>,
    pub cover: Option<f64>,
    pub flinch: Option<f64>,
    pub still: Option<f64>,
    pub help: Option<f64>,
    pub stay_ours: Option<f64>,
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

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CuesHeardLogItem {
    pub t: f64,
    pub text: String,
    pub drawn: bool,
    pub why: String,
}

#[derive(Clone, Debug, Default)]
pub struct LINESValue {
    pub cls: f64,
    pub say: Vec<String>,
    pub now: Option<Vec<String>>,
    pub guide: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct KINDGAP {
    pub stay: Option<f64>,
    pub calm: Option<f64>,
    pub cover: Option<f64>,
    pub flinch: Option<f64>,
    pub still: Option<f64>,
    pub help: Option<f64>,
    pub stay_ours: Option<f64>,
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

#[derive(Clone, Debug, Default)]
pub struct CueLinesItem {
    pub kind: crate::cues::CueKind,
    pub cls: f64,
    pub text: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CuesFileKind {
    Heard,
    Stay,
    Calm,
    Cover,
    Flinch,
    Still,
    Call,
    Help,
    StayOurs,
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
pub struct CuesAdoptMsgsItem {
    pub text: String,
    pub t: f64,
}

#[derive(Clone, Debug, Default)]
pub struct CuesAt {
    pub the: String,
    pub at: String,
}

/// (src/game/cues.ts:90 CLASSES: a table nothing writes)
pub fn classes() -> &'static [(&'static [&'static str], &'static [&'static str], &'static [&'static str], f64)] { &[(&["fall back", "falling back", "fell back", "is lost", "out the back", "get back", "behind me", "i'm coming to you"], &[], &[], 5.0), (&["still out there"], &[], &[], 6.0), (&["hold where you are", "lost them", "quiet now", "standing down", "gone quiet", "covering him", "not answering — going back"], &[], &[], 2.0), (&["going round", "going around", "moving round", "moving around", "locked down. box him in", "moving to", "moved on — after them", "coming to you", "coming at", "coming up behind", "with you", "get round", "get around", "hit the others", "on that door", "get in there", "in position", "go go go", "go — go", "go, go", "pushing", "on my way", "two on this room", "everybody, go", "stand and fight", "charge first", "moving up behind", "coming in — i", "bang out", "move up", "putting fire", "covering — move"], &["moving!"], &["set"], 1.0), (&["lock the doors", "on the doors", "back on our doors", "stay on your doors", "digging in", "getting ready", "ready for them", "room ready", "staying on my door", "holding my room", "staying on my post", "holding my post", "let them come", "not coming out", "holding my corner", "holding here", "i've got this door", "on my own"], &[], &[], 3.0)] }

/// (src/game/cues.ts:104 LINES: a table nothing writes)
pub fn lines() -> JsMap<String, crate::cues::LINESValue> { { let mut t = JsMap::new(); t.set(String::from("help"), crate::cues::LINESValue { cls: 1.0, say: alloc::vec![String::from("moving to the fight — {the}"), String::from("they're hit {at} — moving to them")], now: None, guide: None }); t.set(String::from("huntOver"), crate::cues::LINESValue { cls: 2.0, say: alloc::vec![String::from("lost them — holding {the}"), String::from("lost them — staying {at}")], now: None, guide: None }); t.set(String::from("calm"), crate::cues::LINESValue { cls: 2.0, say: alloc::vec![String::from("quiet now — staying put"), String::from("standing down — holding here")], now: None, guide: None }); t.set(String::from("cover"), crate::cues::LINESValue { cls: 2.0, say: alloc::vec![String::from("one of them down {at} — covering him")], now: None, guide: None }); t.set(String::from("squadQuiet"), crate::cues::LINESValue { cls: 2.0, say: alloc::vec![String::from("{sq}: gone quiet here — still holding")], now: None, guide: None }); t.set(String::from("stayOurs"), crate::cues::LINESValue { cls: 3.0, say: alloc::vec![String::from("they're hit {at} — staying on my door"), String::from("shots {at} — staying on my door")], now: Some(alloc::vec![String::from("they're hit {at} — staying on my post"), String::from("that was {at} — staying on my post")]), guide: None }); t.set(String::from("stay"), crate::cues::LINESValue { cls: 3.0, say: alloc::vec![String::from("shots {at} — holding my room"), String::from("heard that, {the} — holding my room")], now: Some(alloc::vec![String::from("that was {at} — holding my post"), String::from("heard that, {the} — holding my post")]), guide: None }); t.set(String::from("squadStays"), crate::cues::LINESValue { cls: 3.0, say: alloc::vec![String::from("{sq}: still on the doors — nobody goes")], now: None, guide: None }); t.set(String::from("squadHears"), crate::cues::LINESValue { cls: 4.0, say: alloc::vec![String::from("{sq}: heard that — stand by")], now: None, guide: None }); t.set(String::from("squadCopies"), crate::cues::LINESValue { cls: 4.0, say: alloc::vec![String::from("{sq}: copy, {ally} — stand by")], now: None, guide: None }); t.set(String::from("squadWaits"), crate::cues::LINESValue { cls: 3.0, say: alloc::vec![String::from("{sq}: hold — let them come in. Wait for my word!")], now: Some(alloc::vec![String::from("{sq}: hold — let them come in. wait for my word!")]), guide: Some(alloc::vec![String::from("{sq}: hold — let them come in. wait for my call!")]) }); t.set(String::from("squadCopiesWaits"), crate::cues::LINESValue { cls: 3.0, say: alloc::vec![String::from("{sq}: copy, {ally} — hold, let them come in. Wait for my word!")], now: Some(alloc::vec![String::from("{sq}: copy, {ally} — hold, let them come in. wait for my word!")]), guide: Some(alloc::vec![String::from("{sq}: copy, {ally} — hold, let them come in. wait for my call!")]) }); t.set(String::from("fellBack"), crate::cues::LINESValue { cls: 5.0, say: alloc::vec![String::from("{the} is lost — falling back"), String::from("falling back — {the} is lost")], now: None, guide: None }); t.set(String::from("planted"), crate::cues::LINESValue { cls: 5.0, say: alloc::vec![String::from("fell back — set {at}")], now: None, guide: None }); t.set(String::from("pinnedIn"), crate::cues::LINESValue { cls: 5.0, say: alloc::vec![String::from("can't get out — fell back to cover {at}")], now: None, guide: None }); t.set(String::from("still"), crate::cues::LINESValue { cls: 6.0, say: alloc::vec![String::from("they're still out there — holding {the}"), String::from("still out there — staying on this door")], now: Some(alloc::vec![String::from("they're still out there — holding {the}"), String::from("still out there — staying on my post")]), guide: None }); t.set(String::from("surprised"), crate::cues::LINESValue { cls: 0.0, say: alloc::vec![String::from("behind—!"), String::from("there—!")], now: None, guide: None }); t.set(String::from("flinch"), crate::cues::LINESValue { cls: 0.0, say: alloc::vec![String::from("the door—!")], now: None, guide: None }); t } }

/// (src/game/cues.ts:127 KIND_GAP: a table nothing writes)
pub fn kind_gap() -> crate::cues::KINDGAP { crate::cues::KINDGAP { stay: Some(3.0), calm: Some(4.0), cover: Some(15.0), flinch: None, still: Some(6.0), help: None, stay_ours: Some(3.0), hunt_over: Some(4.0), fell_back: None, planted: None, pinned_in: None, squad_hears: None, squad_copies: None, squad_waits: None, squad_copies_waits: None, squad_stays: None, squad_quiet: None, surprised: None } }

