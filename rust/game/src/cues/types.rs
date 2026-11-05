//! The types of `cues`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CueKind {
    Stay,
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

/// (TypeScript: CueRow, src/game/cues.ts:131)
#[derive(Clone, Debug)]
pub struct CueRow {
    pub t: f64,
    pub text: String,
    pub cls: f64,
    pub rank: f64,
    pub who: String,
}
impl CueRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CueRow {
            t: 0.0,
            text: String::new(),
            cls: 0.0,
            rank: 0.0,
            who: String::new(),
        }
    }
}
impl Default for CueRow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Bubble, src/game/cues.ts:136)
#[derive(Clone, Debug)]
pub struct Bubble {
    pub text: String,
    pub t: f64,
    pub dur: f64,
    pub radio: bool,
}
impl Bubble {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Bubble {
            text: String::new(),
            t: 0.0,
            dur: 0.0,
            radio: false,
        }
    }
}
impl Default for Bubble { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Man, src/game/cues.ts:137)
#[derive(Clone, Debug)]
pub struct Man {
    pub state: String,
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
impl Man {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Man {
            state: String::new(),
            alert_t: 0.0,
            said_t: 0.0,
            again_t: 0.0,
            stood_t: 0.0,
            moved: None,
            from: String::new(),
            noise_t: 0.0,
            ep: 0.0,
            rally_at: 0.0,
            covered: false,
        }
    }
}
impl Default for Man { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ManMoved {
    Cover,
    Fallback,
}
impl ManMoved {
    pub const fn as_str(self) -> &'static str {
        match self {
            ManMoved::Cover => "cover",
            ManMoved::Fallback => "fallback",
        }
    }
}

/// (TypeScript: Cues, src/game/cues.ts:143)
#[derive(Clone, Debug)]
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
    pub class_t: Vec<f64>,
    pub squad_t: Vec<f64>,
    pub kind_t: crate::cues::CuesKindT,
    pub n: f64,
    pub squelch_due: bool,
    pub heard_row: Option<crate::cues::CuesHeardRow>,
    pub heard_log: Vec<crate::cues::CuesHeardLogItem>,
    pub heard_said: JsMap<String, f64>,
    pub shout_t: Option<f64>,
}
impl Cues {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
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

/// (TypeScript: inline at src/game/cues.ts:146 Cues.stats, src/game/cues.ts:138)
#[derive(Clone, Debug)]
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
impl CuesStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CuesStats {
            heard: 0.0,
            heard_drawn: 0.0,
            shouts: 0.0,
            shouts_drawn: 0.0,
            cues: 0.0,
            cue_net: 0.0,
            cue_drawn: 0.0,
            dup: 0.0,
            gap: 0.0,
            full: 0.0,
            bumped: 0.0,
            bumped_young: 0.0,
            filed: Vec::new(),
            drawn: Vec::new(),
        }
    }
}
impl Default for CuesStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cues.ts:148 Cues.kinds, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
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
impl CuesKinds {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CuesKinds {
            stay: None,
            calm: None,
            cover: None,
            flinch: None,
            still: None,
            help: None,
            stay_ours: None,
            hunt_over: None,
            fell_back: None,
            planted: None,
            pinned_in: None,
            squad_hears: None,
            squad_copies: None,
            squad_waits: None,
            squad_copies_waits: None,
            squad_stays: None,
            squad_quiet: None,
            surprised: None,
        }
    }
}
impl Default for CuesKinds { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cues.ts:149 Cues.log, src/game/cues.ts:149)
#[derive(Clone, Debug)]
pub struct CuesLogItem {
    pub t: f64,
    pub who: String,
    pub text: String,
    pub cls: f64,
    pub kind: crate::cues::CuesLogItemKind,
    pub drawn: bool,
    pub why: String,
}
impl CuesLogItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CuesLogItem {
            t: 0.0,
            who: String::new(),
            text: String::new(),
            cls: 0.0,
            kind: crate::cues::CuesLogItemKind::Heard,
            drawn: false,
            why: String::new(),
        }
    }
}
impl Default for CuesLogItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CuesLogItemKind {
    Heard,
    Stay,
    Calm,
    Cover,
    Call,
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
    Raw,
}
impl CuesLogItemKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            CuesLogItemKind::Heard => "heard",
            CuesLogItemKind::Stay => "stay",
            CuesLogItemKind::Calm => "calm",
            CuesLogItemKind::Cover => "cover",
            CuesLogItemKind::Call => "call",
            CuesLogItemKind::Flinch => "flinch",
            CuesLogItemKind::Still => "still",
            CuesLogItemKind::Help => "help",
            CuesLogItemKind::StayOurs => "stayOurs",
            CuesLogItemKind::HuntOver => "huntOver",
            CuesLogItemKind::FellBack => "fellBack",
            CuesLogItemKind::Planted => "planted",
            CuesLogItemKind::PinnedIn => "pinnedIn",
            CuesLogItemKind::SquadHears => "squadHears",
            CuesLogItemKind::SquadCopies => "squadCopies",
            CuesLogItemKind::SquadWaits => "squadWaits",
            CuesLogItemKind::SquadCopiesWaits => "squadCopiesWaits",
            CuesLogItemKind::SquadStays => "squadStays",
            CuesLogItemKind::SquadQuiet => "squadQuiet",
            CuesLogItemKind::Surprised => "surprised",
            CuesLogItemKind::Raw => "raw",
        }
    }
}

/// (TypeScript: inline at src/game/cues.ts:152 Cues.kindT, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
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
impl CuesKindT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CuesKindT {
            stay: None,
            calm: None,
            cover: None,
            flinch: None,
            still: None,
            help: None,
            stay_ours: None,
            hunt_over: None,
            fell_back: None,
            planted: None,
            pinned_in: None,
            squad_hears: None,
            squad_copies: None,
            squad_waits: None,
            squad_copies_waits: None,
            squad_stays: None,
            squad_quiet: None,
            surprised: None,
        }
    }
}
impl Default for CuesKindT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cues.ts:154 Cues.heardRow, src/game/cues.ts:154)
#[derive(Clone, Debug)]
pub struct CuesHeardRow {
    pub t: f64,
    pub text: String,
}
impl CuesHeardRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CuesHeardRow {
            t: 0.0,
            text: String::new(),
        }
    }
}
impl Default for CuesHeardRow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cues.ts:156 Cues.heardLog, src/game/cues.ts:156)
#[derive(Clone, Debug)]
pub struct CuesHeardLogItem {
    pub t: f64,
    pub text: String,
    pub drawn: bool,
    pub why: String,
}
impl CuesHeardLogItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CuesHeardLogItem {
            t: 0.0,
            text: String::new(),
            drawn: false,
            why: String::new(),
        }
    }
}
impl Default for CuesHeardLogItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cues.ts:103 LINES, src/game/cues.ts:103)
#[derive(Clone, Debug)]
pub struct LINESValue {
    pub cls: f64,
    pub say: Vec<String>,
    pub now: Option<Vec<String>>,
    pub guide: Option<Vec<String>>,
}
impl LINESValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LINESValue {
            cls: 0.0,
            say: Vec::new(),
            now: None,
            guide: None,
        }
    }
}
impl Default for LINESValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cues.ts:126 KIND_GAP, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
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
impl KINDGAP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KINDGAP {
            stay: None,
            calm: None,
            cover: None,
            flinch: None,
            still: None,
            help: None,
            stay_ours: None,
            hunt_over: None,
            fell_back: None,
            planted: None,
            pinned_in: None,
            squad_hears: None,
            squad_copies: None,
            squad_waits: None,
            squad_copies_waits: None,
            squad_stays: None,
            squad_quiet: None,
            surprised: None,
        }
    }
}
impl Default for KINDGAP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cues.ts:128 cueLines ->, src/game/cues.ts:128)
#[derive(Clone, Debug)]
pub struct CueLinesItem {
    pub kind: crate::cues::CueKind,
    pub cls: f64,
    pub text: String,
}
impl CueLinesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CueLinesItem {
            kind: crate::cues::CueKind::Stay,
            cls: 0.0,
            text: String::new(),
        }
    }
}
impl Default for CueLinesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CuesFileKind {
    Heard,
    Stay,
    Calm,
    Cover,
    Call,
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
    Raw,
}
impl CuesFileKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            CuesFileKind::Heard => "heard",
            CuesFileKind::Stay => "stay",
            CuesFileKind::Calm => "calm",
            CuesFileKind::Cover => "cover",
            CuesFileKind::Call => "call",
            CuesFileKind::Flinch => "flinch",
            CuesFileKind::Still => "still",
            CuesFileKind::Help => "help",
            CuesFileKind::StayOurs => "stayOurs",
            CuesFileKind::HuntOver => "huntOver",
            CuesFileKind::FellBack => "fellBack",
            CuesFileKind::Planted => "planted",
            CuesFileKind::PinnedIn => "pinnedIn",
            CuesFileKind::SquadHears => "squadHears",
            CuesFileKind::SquadCopies => "squadCopies",
            CuesFileKind::SquadWaits => "squadWaits",
            CuesFileKind::SquadCopiesWaits => "squadCopiesWaits",
            CuesFileKind::SquadStays => "squadStays",
            CuesFileKind::SquadQuiet => "squadQuiet",
            CuesFileKind::Surprised => "surprised",
            CuesFileKind::Raw => "raw",
        }
    }
}

/// (TypeScript: inline at src/game/cues.ts:251 adopt(msgs), src/game/cues.ts:251)
#[derive(Clone, Debug)]
pub struct CuesAdoptMsgsItem {
    pub text: String,
    pub t: f64,
}
impl CuesAdoptMsgsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CuesAdoptMsgsItem {
            text: String::new(),
            t: 0.0,
        }
    }
}
impl Default for CuesAdoptMsgsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cues.ts:261 at ->, src/game/cues.ts:261)
#[derive(Clone, Debug)]
pub struct CuesAt {
    pub the: String,
    pub at: String,
}
impl CuesAt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CuesAt {
            the: String::new(),
            at: String::new(),
        }
    }
}
impl Default for CuesAt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cues.ts:354 stripRows ->, src/game/cues.ts:354)
#[derive(Clone, Debug)]
pub struct StripRowsItem {
    pub text: String,
    pub t: f64,
}
impl StripRowsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StripRowsItem {
            text: String::new(),
            t: 0.0,
        }
    }
}
impl Default for StripRowsItem { fn default() -> Self { Self::zeroed() } }

/// (src/game/cues.ts:79 CUES_DEFAULTS: a table nothing writes)
pub fn cues_defaults() -> crate::knobs::CuesK { crate::knobs::page_cues() }

/// (src/game/cues.ts:80 LS)
pub const LS: &str = "twopointsix.cues";

/// (src/game/cues.ts:82 KEPT: a table nothing writes)
pub fn kept() -> [String; 3] { [String::from("on"), String::from("squelch"), String::from("stillSecs")] }

/// (src/game/cues.ts:89 CLASSES: a table nothing writes)
pub fn classes() -> &'static [(&'static [&'static str], &'static [&'static str], &'static [&'static str], f64)] { &[(&["fall back", "falling back", "fell back", "is lost", "out the back", "get back", "behind me", "i'm coming to you"], &[], &[], 5.0), (&["still out there"], &[], &[], 6.0), (&["hold where you are", "lost them", "quiet now", "standing down", "gone quiet", "covering him", "not answering — going back"], &[], &[], 2.0), (&["going round", "going around", "moving round", "moving around", "locked down. box him in", "moving to", "moved on — after them", "coming to you", "coming at", "coming up behind", "with you", "get round", "get around", "hit the others", "on that door", "get in there", "in position", "go go go", "go — go", "go, go", "pushing", "on my way", "two on this room", "everybody, go", "stand and fight", "charge first", "moving up behind", "coming in — i", "bang out", "move up", "putting fire", "covering — move"], &["moving!"], &["set"], 1.0), (&["lock the doors", "on the doors", "back on our doors", "stay on your doors", "digging in", "getting ready", "ready for them", "room ready", "staying on my door", "holding my room", "staying on my post", "holding my post", "let them come", "not coming out", "holding my corner", "holding here", "i've got this door", "on my own"], &[], &[], 3.0)] }

/// (src/game/cues.ts:103 LINES: a table nothing writes)
pub fn lines() -> JsMap<String, crate::cues::LINESValue> { { let mut t = JsMap::new(); t.set(String::from("help"), crate::cues::LINESValue { cls: 1.0, say: alloc::vec![String::from("moving to the fight — {the}"), String::from("they're hit {at} — moving to them")], now: None, guide: None }); t.set(String::from("huntOver"), crate::cues::LINESValue { cls: 2.0, say: alloc::vec![String::from("lost them — holding {the}"), String::from("lost them — staying {at}")], now: None, guide: None }); t.set(String::from("calm"), crate::cues::LINESValue { cls: 2.0, say: alloc::vec![String::from("quiet now — staying put"), String::from("standing down — holding here")], now: None, guide: None }); t.set(String::from("cover"), crate::cues::LINESValue { cls: 2.0, say: alloc::vec![String::from("one of them down {at} — covering him")], now: None, guide: None }); t.set(String::from("squadQuiet"), crate::cues::LINESValue { cls: 2.0, say: alloc::vec![String::from("{sq}: gone quiet here — still holding")], now: None, guide: None }); t.set(String::from("stayOurs"), crate::cues::LINESValue { cls: 3.0, say: alloc::vec![String::from("they're hit {at} — staying on my door"), String::from("shots {at} — staying on my door")], now: Some(alloc::vec![String::from("they're hit {at} — staying on my post"), String::from("that was {at} — staying on my post")]), guide: None }); t.set(String::from("stay"), crate::cues::LINESValue { cls: 3.0, say: alloc::vec![String::from("shots {at} — holding my room"), String::from("heard that, {the} — holding my room")], now: Some(alloc::vec![String::from("that was {at} — holding my post"), String::from("heard that, {the} — holding my post")]), guide: None }); t.set(String::from("squadStays"), crate::cues::LINESValue { cls: 3.0, say: alloc::vec![String::from("{sq}: still on the doors — nobody goes")], now: None, guide: None }); t.set(String::from("squadHears"), crate::cues::LINESValue { cls: 4.0, say: alloc::vec![String::from("{sq}: heard that — stand by")], now: None, guide: None }); t.set(String::from("squadCopies"), crate::cues::LINESValue { cls: 4.0, say: alloc::vec![String::from("{sq}: copy, {ally} — stand by")], now: None, guide: None }); t.set(String::from("squadWaits"), crate::cues::LINESValue { cls: 3.0, say: alloc::vec![String::from("{sq}: hold — let them come in. Wait for my word!")], now: Some(alloc::vec![String::from("{sq}: hold — let them come in. wait for my word!")]), guide: Some(alloc::vec![String::from("{sq}: hold — let them come in. wait for my call!")]) }); t.set(String::from("squadCopiesWaits"), crate::cues::LINESValue { cls: 3.0, say: alloc::vec![String::from("{sq}: copy, {ally} — hold, let them come in. Wait for my word!")], now: Some(alloc::vec![String::from("{sq}: copy, {ally} — hold, let them come in. wait for my word!")]), guide: Some(alloc::vec![String::from("{sq}: copy, {ally} — hold, let them come in. wait for my call!")]) }); t.set(String::from("fellBack"), crate::cues::LINESValue { cls: 5.0, say: alloc::vec![String::from("{the} is lost — falling back"), String::from("falling back — {the} is lost")], now: None, guide: None }); t.set(String::from("planted"), crate::cues::LINESValue { cls: 5.0, say: alloc::vec![String::from("fell back — set {at}")], now: None, guide: None }); t.set(String::from("pinnedIn"), crate::cues::LINESValue { cls: 5.0, say: alloc::vec![String::from("can't get out — fell back to cover {at}")], now: None, guide: None }); t.set(String::from("still"), crate::cues::LINESValue { cls: 6.0, say: alloc::vec![String::from("they're still out there — holding {the}"), String::from("still out there — staying on this door")], now: Some(alloc::vec![String::from("they're still out there — holding {the}"), String::from("still out there — staying on my post")]), guide: None }); t.set(String::from("surprised"), crate::cues::LINESValue { cls: 0.0, say: alloc::vec![String::from("behind—!"), String::from("there—!")], now: None, guide: None }); t.set(String::from("flinch"), crate::cues::LINESValue { cls: 0.0, say: alloc::vec![String::from("the door—!")], now: None, guide: None }); t } }

/// (src/game/cues.ts:126 KIND_GAP: a table nothing writes)
pub fn kind_gap() -> crate::cues::KINDGAP { crate::cues::KINDGAP { stay: Some(3.0), calm: Some(4.0), cover: Some(15.0), flinch: None, still: Some(6.0), help: None, stay_ours: Some(3.0), hunt_over: Some(4.0), fell_back: None, planted: None, pinned_in: None, squad_hears: None, squad_copies: None, squad_waits: None, squad_copies_waits: None, squad_stays: None, squad_quiet: None, surprised: None } }

/// (src/game/cues.ts:133 SQUAD_WORD)
pub const SQUAD_WORD: &str = "/^[A-Z][a-z]{1,11}: /";
