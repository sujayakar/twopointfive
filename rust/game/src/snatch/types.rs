//! The types of `snatch`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SnatchEnd {
    Lost,
    Won,
}
impl SnatchEnd {
    pub const fn as_str(self) -> &'static str {
        match self {
            SnatchEnd::Lost => "lost",
            SnatchEnd::Won => "won",
        }
    }
}

/// (TypeScript: SnatchSnap, src/game/snatch.ts:55)
#[derive(Clone, Debug)]
pub struct SnatchSnap {
    pub hands_up: bool,
    pub tied: bool,
    pub near_t: f64,
    pub alone_t: f64,
    pub end: Option<crate::snatch::SnatchEnd>,
    pub why: String,
    pub seen: Option<V3>,
    pub fear: Option<V3>,
    pub lost: Option<crate::snatch::SnatchSnapLost>,
    pub killer: Option<crate::snatch::SnatchSnapKiller>,
    pub times: crate::snatch::SnatchSnapTimes,
    pub stats: crate::snatch::SnatchSnapStats,
    pub last_exit: Option<String>,
    pub at: V3,
    pub yaw: f64,
    pub out: Option<crate::snatchout::SnatchOutSnap>,
}
impl SnatchSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchSnap {
            hands_up: false,
            tied: false,
            near_t: 0.0,
            alone_t: 0.0,
            end: None,
            why: String::new(),
            seen: None,
            fear: None,
            lost: None,
            killer: None,
            times: crate::snatch::SnatchSnapTimes::zeroed(),
            stats: crate::snatch::SnatchSnapStats::zeroed(),
            last_exit: None,
            at: [0.0; 3],
            yaw: 0.0,
            out: None,
        }
    }
}
impl Default for SnatchSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SnatchSnapLost {
    Dead,
    Gone,
}
impl SnatchSnapLost {
    pub const fn as_str(self) -> &'static str {
        match self {
            SnatchSnapLost::Dead => "dead",
            SnatchSnapLost::Gone => "gone",
        }
    }
}

/// (TypeScript: inline at src/game/snatch.ts:55 SnatchSnap.killer, src/game/snatch.ts:55)
#[derive(Clone, Debug)]
pub struct SnatchSnapKiller {
    pub side: String,
    pub who: String,
}
impl SnatchSnapKiller {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchSnapKiller {
            side: String::new(),
            who: String::new(),
        }
    }
}
impl Default for SnatchSnapKiller { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snatch.ts:55 SnatchSnap.times, src/game/snatch.ts:55)
#[derive(Clone, Copy, Debug)]
pub struct SnatchSnapTimes {
    pub ran: f64,
    pub up: f64,
    pub tied: f64,
    pub end: f64,
}
impl SnatchSnapTimes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchSnapTimes {
            ran: 0.0,
            up: 0.0,
            tied: 0.0,
            end: 0.0,
        }
    }
}
impl Default for SnatchSnapTimes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snatch.ts:55 SnatchSnap.stats, src/game/snatch.ts:55)
#[derive(Clone, Copy, Debug)]
pub struct SnatchSnapStats {
    pub turned: f64,
    pub bolted: f64,
}
impl SnatchSnapStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchSnapStats {
            turned: 0.0,
            bolted: 0.0,
        }
    }
}
impl Default for SnatchSnapStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Snatch, src/game/snatch.ts:57)
#[derive(Clone, Debug)]
pub struct Snatch {
    pub man: Option<GuardId>,
    pub end: Option<crate::snatch::SnatchEnd>,
    pub why: String,
    pub lost: Option<crate::snatch::SnatchLost>,
    pub killer: Option<crate::snatch::SnatchKiller>,
    pub hands_up: bool,
    pub tied: bool,
    pub times: crate::snatch::SnatchTimes,
    pub stats: crate::snatch::SnatchStats,
    pub log: Vec<String>,
    pub run: Option<u32>,
    pub near_t: f64,
    pub alone_t: f64,
    pub seen: Option<V3>,
    pub fear: Option<V3>,
    pub said_out: bool,
    pub out: crate::snatchout::SnatchOut,
    pub notes: f64,
    pub last_exit: Option<String>,
}
impl Snatch {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Snatch {
            man: None,
            end: None,
            why: String::from(""),
            lost: None,
            killer: None,
            hands_up: false,
            tied: false,
            times: crate::snatch::SnatchTimes { ran: -1.0, up: -1.0, tied: -1.0, end: -1.0 },
            stats: crate::snatch::SnatchStats { turned: 0.0, bolted: 0.0 },
            log: Vec::new(),
            run: None,
            near_t: 0.0,
            alone_t: 0.0,
            seen: None,
            fear: None,
            said_out: false,
            out: crate::snatchout::SnatchOut::blank(),
            notes: 0.0,
            last_exit: None,
        }
    }
}
impl Default for Snatch { fn default() -> Self { Self::blank() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SnatchLost {
    Dead,
    Gone,
}
impl SnatchLost {
    pub const fn as_str(self) -> &'static str {
        match self {
            SnatchLost::Dead => "dead",
            SnatchLost::Gone => "gone",
        }
    }
}

/// (TypeScript: inline at src/game/snatch.ts:62 Snatch.killer, src/game/snatch.ts:62)
#[derive(Clone, Debug)]
pub struct SnatchKiller {
    pub side: String,
    pub who: String,
}
impl SnatchKiller {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchKiller {
            side: String::new(),
            who: String::new(),
        }
    }
}
impl Default for SnatchKiller { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snatch.ts:65 Snatch.times, src/game/snatch.ts:65)
#[derive(Clone, Copy, Debug)]
pub struct SnatchTimes {
    pub ran: f64,
    pub up: f64,
    pub tied: f64,
    pub end: f64,
}
impl SnatchTimes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchTimes {
            ran: 0.0,
            up: 0.0,
            tied: 0.0,
            end: 0.0,
        }
    }
}
impl Default for SnatchTimes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snatch.ts:67 Snatch.stats, src/game/snatch.ts:67)
#[derive(Clone, Copy, Debug)]
pub struct SnatchStats {
    pub turned: f64,
    pub bolted: f64,
}
impl SnatchStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchStats {
            turned: 0.0,
            bolted: 0.0,
        }
    }
}
impl Default for SnatchStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet Snatch.run, quiet Snatch.run)
#[derive(Clone, Copy, Debug)]
pub struct SnatchRun {
}
impl SnatchRun {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchRun {
        }
    }
}
impl Default for SnatchRun { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snatch.ts:98 teamOf ->, src/game/snatch.ts:98)
#[derive(Clone, Copy, Debug)]
pub struct SnatchTeamOfItem {
    pub c: CharId,
    pub gd: Option<GuardId>,
}
impl SnatchTeamOfItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchTeamOfItem {
            c: CharId(0),
            gd: None,
        }
    }
}
impl Default for SnatchTeamOfItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SnatchLoseHow {
    Dead,
    Gone,
}
impl SnatchLoseHow {
    pub const fn as_str(self) -> &'static str {
        match self {
            SnatchLoseHow::Dead => "dead",
            SnatchLoseHow::Gone => "gone",
        }
    }
}

/// (TypeScript: inline at src/game/snatch.ts:216 objective ->, src/game/snatch.ts:216)
#[derive(Clone, Debug)]
pub struct SnatchObjective {
    pub text: String,
    pub pos: Option<V3>,
}
impl SnatchObjective {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchObjective {
            text: String::new(),
            pos: None,
        }
    }
}
impl Default for SnatchObjective { fn default() -> Self { Self::zeroed() } }
