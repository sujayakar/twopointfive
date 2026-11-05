//! The types of `blast_feel`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Face, src/game/blastFeel.ts:89)
#[derive(Clone, Copy, Debug)]
pub struct Face {
    pub o: V3,
    pub d: V3,
    pub mask: f64,
}
impl Face {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Face {
            o: [0.0; 3],
            d: [0.0; 3],
            mask: 0.0,
        }
    }
}
impl Default for Face { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Kick, src/game/blastFeel.ts:90)
#[derive(Clone, Copy, Debug)]
pub struct Kick {
    pub t0: f64,
    pub at: V3,
}
impl Kick {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Kick {
            t0: 0.0,
            at: [0.0; 3],
        }
    }
}
impl Default for Kick { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: State, src/game/blastFeel.ts:91)
#[derive(Clone, Debug)]
pub struct State {
    pub seen: Vec<WallChargeId>,
    pub queue: Vec<crate::blast_feel::StateQueueItem>,
    pub kicks: Vec<crate::blast_feel::Kick>,
    pub serial: f64,
    pub last: f64,
    pub list: Option<u32>,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            seen: Vec::new(),
            queue: Vec::new(),
            kicks: Vec::new(),
            serial: 0.0,
            last: 0.0,
            list: None,
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastFeel.ts:91 State.queue, src/game/blastFeel.ts:91)
#[derive(Clone, Copy, Debug)]
pub struct StateQueueItem {
    pub at: f64,
    pub run: crate::FnField,
}
impl StateQueueItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateQueueItem {
            at: 0.0,
            run: crate::FnField,
        }
    }
}
impl Default for StateQueueItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastFeel.ts:47 FEEL, src/game/blastFeel.ts:47)
#[derive(Clone, Debug)]
pub struct BlastFeelModFEEL {
    pub on: bool,
    pub kick: f64,
    pub kick_m: f64,
    pub kick_secs: f64,
    pub kick_tau: f64,
    pub kick_hz: f64,
    pub kick_reach: f64,
    pub sound: crate::blast_feel::BlastFeelModFEELSound,
    pub ring: crate::blast_feel::BlastFeelModFEELRing,
    pub push: Vec<[f64; 3]>,
    pub push_up: f64,
    pub ceil: crate::blast_feel::BlastFeelModFEELCeil,
    pub fire: crate::blast_feel::BlastFeelModFEELFire,
}
impl BlastFeelModFEEL {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlastFeelModFEEL {
            on: false,
            kick: 0.0,
            kick_m: 0.0,
            kick_secs: 0.0,
            kick_tau: 0.0,
            kick_hz: 0.0,
            kick_reach: 0.0,
            sound: crate::blast_feel::BlastFeelModFEELSound::zeroed(),
            ring: crate::blast_feel::BlastFeelModFEELRing::zeroed(),
            push: Vec::new(),
            push_up: 0.0,
            ceil: crate::blast_feel::BlastFeelModFEELCeil::zeroed(),
            fire: crate::blast_feel::BlastFeelModFEELFire::zeroed(),
        }
    }
}
impl Default for BlastFeelModFEEL { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastFeel.ts:53 BlastFeelModFEEL.sound, src/game/blastFeel.ts:53)
#[derive(Clone, Copy, Debug)]
pub struct BlastFeelModFEELSound {
    pub crack: f64,
    pub thump: f64,
    pub debris: f64,
    pub debris_after: f64,
    pub thump_after: f64,
    pub carry: f64,
    pub carry_from: f64,
    pub carry_over: f64,
    pub c: f64,
    pub late_max: f64,
    pub dull_m: f64,
    pub ring: f64,
    pub ring_m: f64,
}
impl BlastFeelModFEELSound {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlastFeelModFEELSound {
            crack: 0.0,
            thump: 0.0,
            debris: 0.0,
            debris_after: 0.0,
            thump_after: 0.0,
            carry: 0.0,
            carry_from: 0.0,
            carry_over: 0.0,
            c: 0.0,
            late_max: 0.0,
            dull_m: 0.0,
            ring: 0.0,
            ring_m: 0.0,
        }
    }
}
impl Default for BlastFeelModFEELSound { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastFeel.ts:66 BlastFeelModFEEL.ring, src/game/blastFeel.ts:66)
#[derive(Clone, Copy, Debug)]
pub struct BlastFeelModFEELRing {
    pub stages: f64,
    pub sectors: f64,
    pub arc: f64,
    pub r0: f64,
    pub step: f64,
    pub every: f64,
    pub y: f64,
    pub speed: f64,
    pub life: f64,
    pub size: f64,
    pub color: V3,
}
impl BlastFeelModFEELRing {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlastFeelModFEELRing {
            stages: 0.0,
            sectors: 0.0,
            arc: 0.0,
            r0: 0.0,
            step: 0.0,
            every: 0.0,
            y: 0.0,
            speed: 0.0,
            life: 0.0,
            size: 0.0,
            color: [0.0; 3],
        }
    }
}
impl Default for BlastFeelModFEELRing { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastFeel.ts:70 BlastFeelModFEEL.ceil, src/game/blastFeel.ts:70)
#[derive(Clone, Copy, Debug)]
pub struct BlastFeelModFEELCeil {
    pub spots: f64,
    pub reach: f64,
    pub after: f64,
    pub speed: f64,
    pub puffs: f64,
    pub crumbs: f64,
    pub color: V3,
}
impl BlastFeelModFEELCeil {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlastFeelModFEELCeil {
            spots: 0.0,
            reach: 0.0,
            after: 0.0,
            speed: 0.0,
            puffs: 0.0,
            crumbs: 0.0,
            color: [0.0; 3],
        }
    }
}
impl Default for BlastFeelModFEELCeil { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastFeel.ts:72 BlastFeelModFEEL.fire, src/game/blastFeel.ts:72)
#[derive(Clone, Debug)]
pub struct BlastFeelModFEELFire {
    pub core: crate::blast_feel::BlastFeelModFEELFireCore,
    pub roll: crate::blast_feel::BlastFeelModFEELFireRoll,
    pub soot: Vec<crate::blast_feel::BlastFeelModFEELFireSootItem>,
    pub soot_color: V3,
    pub chunks: crate::blast_feel::BlastFeelModFEELFireChunks,
}
impl BlastFeelModFEELFire {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlastFeelModFEELFire {
            core: crate::blast_feel::BlastFeelModFEELFireCore::zeroed(),
            roll: crate::blast_feel::BlastFeelModFEELFireRoll::zeroed(),
            soot: Vec::new(),
            soot_color: [0.0; 3],
            chunks: crate::blast_feel::BlastFeelModFEELFireChunks::zeroed(),
        }
    }
}
impl Default for BlastFeelModFEELFire { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastFeel.ts:73 BlastFeelModFEELFire.core, src/game/blastFeel.ts:73)
#[derive(Clone, Debug)]
pub struct BlastFeelModFEELFireCore {
    pub count: Vec<f64>,
    pub speed: Vec<f64>,
    pub life: f64,
    pub size: f64,
}
impl BlastFeelModFEELFireCore {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlastFeelModFEELFireCore {
            count: Vec::new(),
            speed: Vec::new(),
            life: 0.0,
            size: 0.0,
        }
    }
}
impl Default for BlastFeelModFEELFireCore { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastFeel.ts:74 BlastFeelModFEELFire.roll, src/game/blastFeel.ts:74)
#[derive(Clone, Debug)]
pub struct BlastFeelModFEELFireRoll {
    pub count: Vec<f64>,
    pub speed: Vec<f64>,
    pub life: f64,
    pub size: f64,
    pub after: f64,
}
impl BlastFeelModFEELFireRoll {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlastFeelModFEELFireRoll {
            count: Vec::new(),
            speed: Vec::new(),
            life: 0.0,
            size: 0.0,
            after: 0.0,
        }
    }
}
impl Default for BlastFeelModFEELFireRoll { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastFeel.ts:75 BlastFeelModFEELFire.soot, src/game/blastFeel.ts:75)
#[derive(Clone, Debug)]
pub struct BlastFeelModFEELFireSootItem {
    pub after: f64,
    pub count: Vec<f64>,
    pub speed: f64,
    pub size: f64,
    pub life: f64,
}
impl BlastFeelModFEELFireSootItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlastFeelModFEELFireSootItem {
            after: 0.0,
            count: Vec::new(),
            speed: 0.0,
            size: 0.0,
            life: 0.0,
        }
    }
}
impl Default for BlastFeelModFEELFireSootItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastFeel.ts:78 BlastFeelModFEELFire.chunks, src/game/blastFeel.ts:78)
#[derive(Clone, Debug)]
pub struct BlastFeelModFEELFireChunks {
    pub count: Vec<f64>,
    pub speed: Vec<f64>,
    pub life: f64,
    pub size: f64,
    pub color: V3,
}
impl BlastFeelModFEELFireChunks {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlastFeelModFEELFireChunks {
            count: Vec::new(),
            speed: Vec::new(),
            life: 0.0,
            size: 0.0,
            color: [0.0; 3],
        }
    }
}
impl Default for BlastFeelModFEELFireChunks { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct BlastFeelMod {
    /// (src/game/blastFeel.ts:47 FEEL)
    pub feel: crate::blast_feel::BlastFeelModFEEL,
    /// (src/game/blastFeel.ts:82 loaded)
    pub loaded: bool,
    /// (src/game/blastFeel.ts:92 STATE)
    pub state: JsMap<PlayerId, crate::blast_feel::State>,
}
impl BlastFeelMod {
    pub fn blank() -> Self {
        BlastFeelMod {
            feel: crate::blast_feel::BlastFeelModFEEL { on: true, kick: 1.0, kick_m: 0.45, kick_secs: 0.45, kick_tau: 0.14, kick_hz: 7.0, kick_reach: 28.0, sound: crate::blast_feel::BlastFeelModFEELSound { crack: 1.0, thump: 1.0, debris: 0.6, debris_after: 0.1, thump_after: 0.016666666666666666, carry: 0.8, carry_from: 3.0, carry_over: 7.0, c: 343.0, late_max: 0.25, dull_m: 14.0, ring: 0.3, ring_m: 7.0 }, ring: crate::blast_feel::BlastFeelModFEELRing { stages: 9.0, sectors: 13.0, arc: 1.4, r0: 0.5, step: 0.55, every: 0.025, y: 0.14, speed: 5.0, life: 0.5, size: 0.3, color: [0.74, 0.72, 0.68] }, push: alloc::vec![[0.0, 3.0, 7.0], [0.075, 5.5, 4.5], [0.15, 8.0, 3.0]], push_up: 0.5, ceil: crate::blast_feel::BlastFeelModFEELCeil { spots: 13.0, reach: 6.5, after: 0.1, speed: 14.0, puffs: 3.0, crumbs: 3.0, color: [0.82, 0.81, 0.78] }, fire: crate::blast_feel::BlastFeelModFEELFire { core: crate::blast_feel::BlastFeelModFEELFireCore { count: alloc::vec![30.0, 16.0], speed: alloc::vec![4.2, 3.0], life: 0.3, size: 0.11 }, roll: crate::blast_feel::BlastFeelModFEELFireRoll { count: alloc::vec![14.0, 8.0], speed: alloc::vec![2.2, 1.6], life: 0.5, size: 0.15, after: 2.0 }, soot: alloc::vec![crate::blast_feel::BlastFeelModFEELFireSootItem { after: 2.0, count: alloc::vec![12.0, 6.0], speed: 2.8, size: 0.3, life: 1.4 }, crate::blast_feel::BlastFeelModFEELFireSootItem { after: 5.0, count: alloc::vec![9.0, 5.0], speed: 2.2, size: 0.38, life: 1.7 }, crate::blast_feel::BlastFeelModFEELFireSootItem { after: 8.0, count: alloc::vec![8.0, 4.0], speed: 1.7, size: 0.44, life: 2.0 }, crate::blast_feel::BlastFeelModFEELFireSootItem { after: 13.0, count: alloc::vec![6.0, 3.0], speed: 1.1, size: 0.54, life: 2.6 }], soot_color: [0.05, 0.044, 0.038], chunks: crate::blast_feel::BlastFeelModFEELFireChunks { count: alloc::vec![28.0, 14.0], speed: alloc::vec![8.0, 5.5], life: 7.0, size: 0.05, color: [0.05, 0.046, 0.042] } } },
            loaded: false,
            state: JsMap::new(),
        }
    }
}

/// (src/game/blastFeel.ts:81 LS_KEY)
pub const LS_KEY: &str = "twopointsix.blastfeel";

/// (src/game/blastFeel.ts:97 FRAME)
pub const FRAME: f64 = 0.016666666666666666;
