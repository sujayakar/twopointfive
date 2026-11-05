//! The types of `verdict`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Approach {
    Doors,
    Quiet,
    Walls,
    Dark,
}
impl Approach {
    pub const fn as_str(self) -> &'static str {
        match self {
            Approach::Doors => "doors",
            Approach::Quiet => "quiet",
            Approach::Walls => "walls",
            Approach::Dark => "dark",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ManState {
    Dead,
    Wounded,
    Out,
    Up,
}
impl ManState {
    pub const fn as_str(self) -> &'static str {
        match self {
            ManState::Dead => "dead",
            ManState::Wounded => "wounded",
            ManState::Out => "out",
            ManState::Up => "up",
        }
    }
}

/// (TypeScript: VerdictMan, src/game/verdict.ts:72)
#[derive(Clone, Debug)]
pub struct VerdictMan {
    pub id: f64,
    pub name: String,
    pub tone: Option<crate::host_findme::OutlineHue>,
    pub human: bool,
    pub state: crate::verdict::ManState,
    pub down_at: f64,
    pub down_room: String,
    pub down_by: String,
    pub down_pos: Option<[f64; 2]>,
    pub drops: Vec<f64>,
    pub path: Vec<f64>,
}
impl VerdictMan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        VerdictMan {
            id: 0.0,
            name: String::new(),
            tone: None,
            human: false,
            state: crate::verdict::ManState::Dead,
            down_at: 0.0,
            down_room: String::new(),
            down_by: String::new(),
            down_pos: None,
            drops: Vec::new(),
            path: Vec::new(),
        }
    }
}
impl Default for VerdictMan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RunVerdict, src/game/verdict.ts:77)
#[derive(Clone, Debug)]
pub struct RunVerdict {
    pub kind: Option<crate::verdict::RunVerdictKind>,
    pub won: bool,
    pub word: String,
    pub why: String,
    pub secs: f64,
    pub approach: Option<crate::verdict::Approach>,
    pub void: bool,
    pub clean: bool,
    pub men: Vec<crate::verdict::VerdictMan>,
    pub lost: f64,
    pub of: f64,
    pub foes: crate::verdict::RunVerdictFoes,
    pub hostages: Option<crate::verdict::RunVerdictHostages>,
    pub start: [f64; 2],
}
impl RunVerdict {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RunVerdict {
            kind: None,
            won: false,
            word: String::new(),
            why: String::new(),
            secs: 0.0,
            approach: None,
            void: false,
            clean: false,
            men: Vec::new(),
            lost: 0.0,
            of: 0.0,
            foes: crate::verdict::RunVerdictFoes::zeroed(),
            hostages: None,
            start: [0.0; 2],
        }
    }
}
impl Default for RunVerdict { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RunVerdictKind {
    Nightbreach,
    Roomclear,
    Rescue,
    Quiet,
}
impl RunVerdictKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            RunVerdictKind::Nightbreach => "nightbreach",
            RunVerdictKind::Roomclear => "roomclear",
            RunVerdictKind::Rescue => "rescue",
            RunVerdictKind::Quiet => "quiet",
        }
    }
}

/// (TypeScript: inline at src/game/verdict.ts:89 RunVerdict.foes, src/game/verdict.ts:89)
#[derive(Clone, Debug)]
pub struct RunVerdictFoes {
    pub down: f64,
    pub of: f64,
    pub fell: Vec<f64>,
}
impl RunVerdictFoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RunVerdictFoes {
            down: 0.0,
            of: 0.0,
            fell: Vec::new(),
        }
    }
}
impl Default for RunVerdictFoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/verdict.ts:90 RunVerdict.hostages, src/game/verdict.ts:90)
#[derive(Clone, Copy, Debug)]
pub struct RunVerdictHostages {
    pub alive: f64,
    pub of: f64,
}
impl RunVerdictHostages {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RunVerdictHostages {
            alive: 0.0,
            of: 0.0,
        }
    }
}
impl Default for RunVerdictHostages { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: KeptNote, src/game/verdict.ts:95)
#[derive(Clone, Copy, Debug)]
pub struct KeptNote {
    pub streak: f64,
    pub fresh: bool,
    pub gold: bool,
    pub ways: crate::verdict::KeptNoteWays,
    pub lesson: Option<bool>,
}
impl KeptNote {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KeptNote {
            streak: 0.0,
            fresh: false,
            gold: false,
            ways: crate::verdict::KeptNoteWays::zeroed(),
            lesson: None,
        }
    }
}
impl Default for KeptNote { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/verdict.ts:95 KeptNote.ways, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct KeptNoteWays {
    pub doors: Option<crate::verdict::KeptNoteWaysDoors>,
    pub quiet: Option<crate::verdict::KeptNoteWaysDoors>,
    pub walls: Option<crate::verdict::KeptNoteWaysDoors>,
    pub dark: Option<crate::verdict::KeptNoteWaysDoors>,
}
impl KeptNoteWays {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KeptNoteWays {
            doors: None,
            quiet: None,
            walls: None,
            dark: None,
        }
    }
}
impl Default for KeptNoteWays { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at KeptNoteWays.doors, src/game/verdict.ts:95)
#[derive(Clone, Copy, Debug)]
pub struct KeptNoteWaysDoors {
    pub first: bool,
    pub clean: bool,
}
impl KeptNoteWaysDoors {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KeptNoteWaysDoors {
            first: false,
            clean: false,
        }
    }
}
impl Default for KeptNoteWaysDoors { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Track, src/game/verdict.ts:97)
#[derive(Clone, Debug)]
pub struct Track {
    pub id: f64,
    pub name: String,
    pub tone: Option<crate::host_findme::OutlineHue>,
    pub human: bool,
    pub gd: Option<GuardId>,
    pub state: crate::verdict::ManState,
    pub down_at: f64,
    pub down_room: String,
    pub down_by: String,
    pub down_pos: Option<[f64; 2]>,
    pub drops: Vec<f64>,
    pub path: Vec<f64>,
    pub every: f64,
    pub next: f64,
}
impl Track {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Track {
            id: 0.0,
            name: String::new(),
            tone: None,
            human: false,
            gd: None,
            state: crate::verdict::ManState::Dead,
            down_at: 0.0,
            down_room: String::new(),
            down_by: String::new(),
            down_pos: None,
            drops: Vec::new(),
            path: Vec::new(),
            every: 0.0,
            next: 0.0,
        }
    }
}
impl Default for Track { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Verdict, src/game/verdict.ts:100)
#[derive(Clone, Debug)]
pub struct Verdict {
    pub run: Option<u32>,
    pub t0: f64,
    pub origin: V3,
    pub order0: Option<u32>,
    pub origins: Vec<V3>,
    pub fired0: f64,
    pub resumed: bool,
    pub cheat: bool,
    pub found_at: f64,
    pub dark_at: f64,
    pub dark_when_found: bool,
    pub through: bool,
    pub wall_serial: f64,
    pub holes: Vec<V3>,
    pub tracks: JsMap<crate::Num, crate::verdict::Track>,
    pub foes: JsSet<GuardId>,
    pub fell: JsSet<GuardId>,
    pub fell_at: Vec<f64>,
    pub res: Option<crate::verdict::RunVerdict>,
    pub card_at: f64,
    pub kept: Option<crate::verdict::KeptNote>,
}
impl Verdict {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Verdict {
            run: None,
            t0: f64::NAN,
            origin: [0.0, 0.0, 0.0],
            order0: None,
            origins: Vec::new(),
            fired0: 0.0,
            resumed: false,
            cheat: false,
            found_at: -1.0,
            dark_at: -1.0,
            dark_when_found: false,
            through: false,
            wall_serial: 0.0,
            holes: Vec::new(),
            tracks: JsMap::new(),
            foes: JsSet::new(),
            fell: JsSet::new(),
            fell_at: Vec::new(),
            res: None,
            card_at: -1.0,
            kept: None,
        }
    }
}
impl Default for Verdict { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at quiet Verdict.run, quiet Verdict.run)
#[derive(Clone, Copy, Debug)]
pub struct VerdictRun {
}
impl VerdictRun {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        VerdictRun {
        }
    }
}
impl Default for VerdictRun { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/verdict.ts:181 onlyMate(h), src/game/verdict.ts:181)
#[derive(Clone, Debug)]
pub struct VerdictOnlyMateH {
    pub killer_id: f64,
    pub down_by: Option<String>,
    pub down_kind: Option<String>,
}
impl VerdictOnlyMateH {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        VerdictOnlyMateH {
            killer_id: 0.0,
            down_by: None,
            down_kind: None,
        }
    }
}
impl Default for VerdictOnlyMateH { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/verdict.ts:229 ended ->, src/game/verdict.ts:229)
#[derive(Clone, Debug)]
pub struct VerdictEnded {
    pub won: bool,
    pub word: String,
    pub why: String,
}
impl VerdictEnded {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        VerdictEnded {
            won: false,
            word: String::new(),
            why: String::new(),
        }
    }
}
impl Default for VerdictEnded { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/verdict.ts:245 close(end), src/game/verdict.ts:245)
#[derive(Clone, Debug)]
pub struct VerdictCloseEnd {
    pub won: bool,
    pub word: String,
    pub why: String,
}
impl VerdictCloseEnd {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        VerdictCloseEnd {
            won: false,
            word: String::new(),
            why: String::new(),
        }
    }
}
impl Default for VerdictCloseEnd { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum VerdictApproachKind {
    Nightbreach,
    Roomclear,
    Rescue,
    Quiet,
}
impl VerdictApproachKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            VerdictApproachKind::Nightbreach => "nightbreach",
            VerdictApproachKind::Roomclear => "roomclear",
            VerdictApproachKind::Rescue => "rescue",
            VerdictApproachKind::Quiet => "quiet",
        }
    }
}

/// (src/game/verdict.ts:98 PATH_CAP)
pub const PATH_CAP: f64 = 1500.0;

/// (src/game/verdict.ts:98 PATH_SECS)
pub const PATH_SECS: f64 = 0.5;

/// (src/game/verdict.ts:98 HOLE_M)
pub const HOLE_M: f64 = 0.9;
