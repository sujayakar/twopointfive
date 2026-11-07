//! The types of `blast_look`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Seen, src/game/blastLook.ts:148)
#[derive(Clone, Copy, Debug)]
pub struct Seen {
    pub c: WallChargeId,
    pub t0: f64,
    pub run: f64,
}
impl Seen {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Seen {
            c: WallChargeId(0),
            t0: 0.0,
            run: 0.0,
        }
    }
}
impl Default for Seen { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: State, src/game/blastLook.ts:149)
#[derive(Clone, Debug)]
pub struct State {
    pub seen: Vec<crate::blast_look::Seen>,
    pub queue: Vec<crate::blast_look::StateQueueItem>,
    pub serial: f64,
    pub first: bool,
    pub last: f64,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            seen: Vec::new(),
            queue: Vec::new(),
            serial: 0.0,
            first: false,
            last: 0.0,
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:149 State.queue, src/game/blastLook.ts:149)
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

/// (TypeScript: Fan, src/game/blastLook.ts:301)
#[derive(Clone, Debug)]
pub struct Fan {
    pub count: Vec<f64>,
    pub speed: Vec<f64>,
    pub jit: f64,
    pub cone: Vec<f64>,
    pub tilt: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
    pub after: Option<f64>,
    pub aspect: Option<f64>,
}
impl Fan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Fan {
            count: Vec::new(),
            speed: Vec::new(),
            jit: 0.0,
            cone: Vec::new(),
            tilt: 0.0,
            life: 0.0,
            life_jit: 0.0,
            size: 0.0,
            size_jit: 0.0,
            after: None,
            aspect: None,
        }
    }
}
impl Default for Fan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:57 BLAST, src/game/blastLook.ts:57)
#[derive(Clone, Copy, Debug)]
pub struct BLAST {
    pub on: bool,
    pub stun: bool,
    pub door: bool,
    pub glow: bool,
}
impl BLAST {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BLAST {
            on: false,
            stun: false,
            door: false,
            glow: false,
        }
    }
}
impl Default for BLAST { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:69 CHARGE_LOOK, src/game/blastLook.ts:69)
#[derive(Clone, Copy, Debug)]
pub struct CHARGELOOK {
    pub frame_half: f64,
    pub frame_lo: f64,
    pub frame_hi: f64,
    pub strip: f64,
    pub proud: f64,
    pub strip_albedo: V3,
    pub lamp: f64,
    pub blink_every: f64,
    pub blink_on: f64,
    pub lamp_on: V3,
    pub lamp_off: V3,
    pub lamp_spill: f64,
    pub led_spill: f64,
    pub tape: f64,
    pub wire_tape: f64,
    pub wire: f64,
    pub wire_px: f64,
    pub wire_max: f64,
    pub run: [f64; 3],
    pub short: f64,
    pub wire_albedo: V3,
    pub reel: V3,
    pub reel_albedo: V3,
    pub led: f64,
    pub led_on: V3,
}
impl CHARGELOOK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CHARGELOOK {
            frame_half: 0.0,
            frame_lo: 0.0,
            frame_hi: 0.0,
            strip: 0.0,
            proud: 0.0,
            strip_albedo: [0.0; 3],
            lamp: 0.0,
            blink_every: 0.0,
            blink_on: 0.0,
            lamp_on: [0.0; 3],
            lamp_off: [0.0; 3],
            lamp_spill: 0.0,
            led_spill: 0.0,
            tape: 0.0,
            wire_tape: 0.0,
            wire: 0.0,
            wire_px: 0.0,
            wire_max: 0.0,
            run: [0.0; 3],
            short: 0.0,
            wire_albedo: [0.0; 3],
            reel: [0.0; 3],
            reel_albedo: [0.0; 3],
            led: 0.0,
            led_on: [0.0; 3],
        }
    }
}
impl Default for CHARGELOOK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:92 REF, src/game/blastLook.ts:92)
#[derive(Clone, Copy, Debug)]
pub struct REF {
    pub sparks: f64,
    pub speed: f64,
    pub life: f64,
    pub size: f64,
}
impl REF {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        REF {
            sparks: 0.0,
            speed: 0.0,
            life: 0.0,
            size: 0.0,
        }
    }
}
impl Default for REF { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:98 WALL_BLAST, src/game/blastLook.ts:98)
#[derive(Clone, Debug)]
pub struct WALLBLAST {
    pub jet: crate::blast_look::WALLBLASTJet,
    pub core: crate::blast_look::WALLBLASTCore,
    pub shower: crate::blast_look::WALLBLASTShower,
    pub embers: crate::blast_look::WALLBLASTEmbers,
    pub chips: crate::blast_look::WALLBLASTChips,
    pub dust: crate::blast_look::WALLBLASTDust,
}
impl WALLBLAST {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLBLAST {
            jet: crate::blast_look::WALLBLASTJet::zeroed(),
            core: crate::blast_look::WALLBLASTCore::zeroed(),
            shower: crate::blast_look::WALLBLASTShower::zeroed(),
            embers: crate::blast_look::WALLBLASTEmbers::zeroed(),
            chips: crate::blast_look::WALLBLASTChips::zeroed(),
            dust: crate::blast_look::WALLBLASTDust::zeroed(),
        }
    }
}
impl Default for WALLBLAST { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:99 WALLBLAST.jet, src/game/blastLook.ts:99)
#[derive(Clone, Debug)]
pub struct WALLBLASTJet {
    pub count: Vec<f64>,
    pub speed: Vec<f64>,
    pub jit: f64,
    pub cone: Vec<f64>,
    pub tilt: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
}
impl WALLBLASTJet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLBLASTJet {
            count: Vec::new(),
            speed: Vec::new(),
            jit: 0.0,
            cone: Vec::new(),
            tilt: 0.0,
            life: 0.0,
            life_jit: 0.0,
            size: 0.0,
            size_jit: 0.0,
        }
    }
}
impl Default for WALLBLASTJet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:102 WALLBLAST.core, src/game/blastLook.ts:102)
#[derive(Clone, Debug)]
pub struct WALLBLASTCore {
    pub count: Vec<f64>,
    pub speed: Vec<f64>,
    pub jit: f64,
    pub cone: Vec<f64>,
    pub tilt: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
}
impl WALLBLASTCore {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLBLASTCore {
            count: Vec::new(),
            speed: Vec::new(),
            jit: 0.0,
            cone: Vec::new(),
            tilt: 0.0,
            life: 0.0,
            life_jit: 0.0,
            size: 0.0,
            size_jit: 0.0,
        }
    }
}
impl Default for WALLBLASTCore { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:103 WALLBLAST.shower, src/game/blastLook.ts:103)
#[derive(Clone, Debug)]
pub struct WALLBLASTShower {
    pub count: Vec<f64>,
    pub speed: Vec<f64>,
    pub jit: f64,
    pub cone: Vec<f64>,
    pub tilt: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
    pub after: f64,
}
impl WALLBLASTShower {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLBLASTShower {
            count: Vec::new(),
            speed: Vec::new(),
            jit: 0.0,
            cone: Vec::new(),
            tilt: 0.0,
            life: 0.0,
            life_jit: 0.0,
            size: 0.0,
            size_jit: 0.0,
            after: 0.0,
        }
    }
}
impl Default for WALLBLASTShower { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:104 WALLBLAST.embers, src/game/blastLook.ts:104)
#[derive(Clone, Debug)]
pub struct WALLBLASTEmbers {
    pub count: Vec<f64>,
    pub speed: Vec<f64>,
    pub jit: f64,
    pub cone: Vec<f64>,
    pub tilt: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
    pub after: f64,
}
impl WALLBLASTEmbers {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLBLASTEmbers {
            count: Vec::new(),
            speed: Vec::new(),
            jit: 0.0,
            cone: Vec::new(),
            tilt: 0.0,
            life: 0.0,
            life_jit: 0.0,
            size: 0.0,
            size_jit: 0.0,
            after: 0.0,
        }
    }
}
impl Default for WALLBLASTEmbers { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:106 WALLBLAST.chips, src/game/blastLook.ts:106)
#[derive(Clone, Debug)]
pub struct WALLBLASTChips {
    pub count: Vec<f64>,
    pub speed: Vec<f64>,
    pub jit: f64,
    pub cone: Vec<f64>,
    pub tilt: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
    pub aspect: f64,
    pub after: f64,
}
impl WALLBLASTChips {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLBLASTChips {
            count: Vec::new(),
            speed: Vec::new(),
            jit: 0.0,
            cone: Vec::new(),
            tilt: 0.0,
            life: 0.0,
            life_jit: 0.0,
            size: 0.0,
            size_jit: 0.0,
            aspect: 0.0,
            after: 0.0,
        }
    }
}
impl Default for WALLBLASTChips { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:108 WALLBLAST.dust, src/game/blastLook.ts:108)
#[derive(Clone, Debug)]
pub struct WALLBLASTDust {
    pub count: Vec<f64>,
    pub speed: Vec<f64>,
    pub jit: f64,
    pub cone: Vec<f64>,
    pub tilt: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
}
impl WALLBLASTDust {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLBLASTDust {
            count: Vec::new(),
            speed: Vec::new(),
            jit: 0.0,
            cone: Vec::new(),
            tilt: 0.0,
            life: 0.0,
            life_jit: 0.0,
            size: 0.0,
            size_jit: 0.0,
        }
    }
}
impl Default for WALLBLASTDust { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:112 WALL_FLASH, src/game/blastLook.ts:112)
#[derive(Clone, Copy, Debug)]
pub struct WALLFLASH {
    pub color: V3,
}
impl WALLFLASH {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLFLASH {
            color: [0.0; 3],
        }
    }
}
impl Default for WALLFLASH { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:116 AFTERGLOW, src/game/blastLook.ts:116)
#[derive(Clone, Copy, Debug)]
pub struct AFTERGLOW {
    pub after: f64,
    pub intensity: f64,
    pub ttl: f64,
    pub range: f64,
    pub radius: f64,
    pub y: f64,
    pub color: V3,
}
impl AFTERGLOW {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AFTERGLOW {
            after: 0.0,
            intensity: 0.0,
            ttl: 0.0,
            range: 0.0,
            radius: 0.0,
            y: 0.0,
            color: [0.0; 3],
        }
    }
}
impl Default for AFTERGLOW { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:119 STUN_SPARKS, src/game/blastLook.ts:119)
#[derive(Clone, Copy, Debug)]
pub struct STUNSPARKS {
    pub lobes: crate::blast_look::STUNSPARKSLobes,
    pub fan: crate::blast_look::STUNSPARKSFan,
    pub fountain: crate::blast_look::STUNSPARKSFountain,
    pub embers: crate::blast_look::STUNSPARKSEmbers,
}
impl STUNSPARKS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        STUNSPARKS {
            lobes: crate::blast_look::STUNSPARKSLobes::zeroed(),
            fan: crate::blast_look::STUNSPARKSFan::zeroed(),
            fountain: crate::blast_look::STUNSPARKSFountain::zeroed(),
            embers: crate::blast_look::STUNSPARKSEmbers::zeroed(),
        }
    }
}
impl Default for STUNSPARKS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:121 STUNSPARKS.lobes, src/game/blastLook.ts:121)
#[derive(Clone, Copy, Debug)]
pub struct STUNSPARKSLobes {
    pub count: f64,
    pub speed: f64,
    pub jit: f64,
    pub cone: f64,
    pub fan: f64,
    pub rise: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
}
impl STUNSPARKSLobes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        STUNSPARKSLobes {
            count: 0.0,
            speed: 0.0,
            jit: 0.0,
            cone: 0.0,
            fan: 0.0,
            rise: 0.0,
            life: 0.0,
            life_jit: 0.0,
            size: 0.0,
            size_jit: 0.0,
        }
    }
}
impl Default for STUNSPARKSLobes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:122 STUNSPARKS.fan, src/game/blastLook.ts:122)
#[derive(Clone, Copy, Debug)]
pub struct STUNSPARKSFan {
    pub count: f64,
    pub speed: f64,
    pub jit: f64,
    pub lift: f64,
    pub spread: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
}
impl STUNSPARKSFan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        STUNSPARKSFan {
            count: 0.0,
            speed: 0.0,
            jit: 0.0,
            lift: 0.0,
            spread: 0.0,
            life: 0.0,
            life_jit: 0.0,
            size: 0.0,
            size_jit: 0.0,
        }
    }
}
impl Default for STUNSPARKSFan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:123 STUNSPARKS.fountain, src/game/blastLook.ts:123)
#[derive(Clone, Copy, Debug)]
pub struct STUNSPARKSFountain {
    pub count: f64,
    pub speed: f64,
    pub jit: f64,
    pub spread: f64,
    pub lift: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
}
impl STUNSPARKSFountain {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        STUNSPARKSFountain {
            count: 0.0,
            speed: 0.0,
            jit: 0.0,
            spread: 0.0,
            lift: 0.0,
            life: 0.0,
            life_jit: 0.0,
            size: 0.0,
            size_jit: 0.0,
        }
    }
}
impl Default for STUNSPARKSFountain { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:124 STUNSPARKS.embers, src/game/blastLook.ts:124)
#[derive(Clone, Copy, Debug)]
pub struct STUNSPARKSEmbers {
    pub count: f64,
    pub speed: f64,
    pub jit: f64,
    pub spread: f64,
    pub lift: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
}
impl STUNSPARKSEmbers {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        STUNSPARKSEmbers {
            count: 0.0,
            speed: 0.0,
            jit: 0.0,
            spread: 0.0,
            lift: 0.0,
            life: 0.0,
            life_jit: 0.0,
            size: 0.0,
            size_jit: 0.0,
        }
    }
}
impl Default for STUNSPARKSEmbers { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:135 DOOR_SPARKS, src/game/blastLook.ts:135)
#[derive(Clone, Copy, Debug)]
pub struct DOORSPARKS {
    pub jet: crate::blast_look::DOORSPARKSJet,
    pub skitter: crate::blast_look::DOORSPARKSSkitter,
    pub back: crate::blast_look::DOORSPARKSBack,
    pub fountain: f64,
    pub ahead: f64,
    pub lead: f64,
}
impl DOORSPARKS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DOORSPARKS {
            jet: crate::blast_look::DOORSPARKSJet::zeroed(),
            skitter: crate::blast_look::DOORSPARKSSkitter::zeroed(),
            back: crate::blast_look::DOORSPARKSBack::zeroed(),
            fountain: 0.0,
            ahead: 0.0,
            lead: 0.0,
        }
    }
}
impl Default for DOORSPARKS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:136 DOORSPARKS.jet, src/game/blastLook.ts:136)
#[derive(Clone, Copy, Debug)]
pub struct DOORSPARKSJet {
    pub count: f64,
    pub speed: f64,
    pub jit: f64,
    pub cone: f64,
    pub tilt: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
}
impl DOORSPARKSJet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DOORSPARKSJet {
            count: 0.0,
            speed: 0.0,
            jit: 0.0,
            cone: 0.0,
            tilt: 0.0,
            life: 0.0,
            life_jit: 0.0,
            size: 0.0,
            size_jit: 0.0,
        }
    }
}
impl Default for DOORSPARKSJet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:137 DOORSPARKS.skitter, src/game/blastLook.ts:137)
#[derive(Clone, Copy, Debug)]
pub struct DOORSPARKSSkitter {
    pub count: f64,
    pub speed: f64,
    pub jit: f64,
    pub cone: f64,
    pub tilt: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
}
impl DOORSPARKSSkitter {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DOORSPARKSSkitter {
            count: 0.0,
            speed: 0.0,
            jit: 0.0,
            cone: 0.0,
            tilt: 0.0,
            life: 0.0,
            life_jit: 0.0,
            size: 0.0,
            size_jit: 0.0,
        }
    }
}
impl Default for DOORSPARKSSkitter { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:138 DOORSPARKS.back, src/game/blastLook.ts:138)
#[derive(Clone, Copy, Debug)]
pub struct DOORSPARKSBack {
    pub count: f64,
    pub speed: f64,
    pub jit: f64,
    pub cone: f64,
    pub tilt: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
}
impl DOORSPARKSBack {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DOORSPARKSBack {
            count: 0.0,
            speed: 0.0,
            jit: 0.0,
            cone: 0.0,
            tilt: 0.0,
            life: 0.0,
            life_jit: 0.0,
            size: 0.0,
            size_jit: 0.0,
        }
    }
}
impl Default for DOORSPARKSBack { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:268 doorChargeAt ->, src/game/blastLook.ts:268)
#[derive(Clone, Copy, Debug)]
pub struct DoorChargeAt {
    pub into: V3,
    pub from: V3,
    pub rest: V3,
}
impl DoorChargeAt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorChargeAt {
            into: [0.0; 3],
            from: [0.0; 3],
            rest: [0.0; 3],
        }
    }
}
impl Default for DoorChargeAt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:280 doorBlast(D), src/game/blastLook.ts:280)
#[derive(Clone, Copy, Debug)]
pub struct DoorBlastD {
    pub into: V3,
    pub from: V3,
    pub rest: V3,
}
impl DoorBlastD {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorBlastD {
            into: [0.0; 3],
            from: [0.0; 3],
            rest: [0.0; 3],
        }
    }
}
impl Default for DoorBlastD { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/blastLook.ts:287 burstOf(k), src/game/blastLook.ts:299)
#[derive(Clone, Copy, Debug)]
pub struct BurstOfK {
    pub n: f64,
    pub v: f64,
    pub l: f64,
    pub s: f64,
}
impl BurstOfK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BurstOfK {
            n: 0.0,
            v: 0.0,
            l: 0.0,
            s: 0.0,
        }
    }
}
impl Default for BurstOfK { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct BlastLookMod {
    /// (src/game/blastLook.ts:150 STATE)
    pub state: JsMap<PlayerId, crate::blast_look::State>,
}
impl BlastLookMod {
    pub fn blank() -> Self {
        BlastLookMod {
            state: JsMap::new(),
        }
    }
}

/// (src/game/blastLook.ts:57 BLAST: a table nothing writes)
pub fn blast() -> crate::blast_look::BLAST { crate::blast_look::BLAST { on: true, stun: true, door: true, glow: false } }

/// (src/game/blastLook.ts:69 CHARGE_LOOK: a table nothing writes)
pub fn charge_look() -> crate::blast_look::CHARGELOOK { crate::blast_look::CHARGELOOK { frame_half: 0.3, frame_lo: 0.5, frame_hi: 1.6, strip: 0.045, proud: 0.012, strip_albedo: [0.62, 0.46, 0.1], lamp: 0.04, blink_every: 1.1, blink_on: 0.35, lamp_on: [70.0, 5.0, 2.5], lamp_off: [0.008, 0.0008, 0.0005], lamp_spill: 0.0025, led_spill: 0.02, tape: 0.07, wire_tape: 0.04, wire: 0.018, wire_px: 1.4, wire_max: 0.05, run: [0.4, 0.2, 0.6], short: 0.36, wire_albedo: [0.85, 0.36, 0.05], reel: [0.05, 0.035, 0.08], reel_albedo: [0.1, 0.11, 0.08], led: 0.013, led_on: [7.0, 3.6, 0.4] } }

/// (src/game/blastLook.ts:92 REF: a table nothing writes)
pub fn ref_() -> crate::blast_look::REF { crate::blast_look::REF { sparks: 160.0, speed: 20.0, life: 0.55, size: 0.0025 } }

/// (src/game/blastLook.ts:98 WALL_BLAST: a table nothing writes)
pub fn wall_blast_table() -> crate::blast_look::WALLBLAST { crate::blast_look::WALLBLAST { jet: crate::blast_look::WALLBLASTJet { count: alloc::vec![30.0, 14.0], speed: alloc::vec![21.0, 15.0], jit: 0.65, cone: alloc::vec![0.36, 0.5], tilt: -0.2, life: 0.24, life_jit: 0.5, size: 0.003, size_jit: 0.5 }, core: crate::blast_look::WALLBLASTCore { count: alloc::vec![14.0, 7.0], speed: alloc::vec![1.6, 1.1], jit: 0.6, cone: alloc::vec![1.3, 1.3], tilt: 0.1, life: 0.95, life_jit: 0.5, size: 0.04, size_jit: 0.5 }, shower: crate::blast_look::WALLBLASTShower { count: alloc::vec![18.0, 10.0], speed: alloc::vec![10.0, 8.0], jit: 0.7, cone: alloc::vec![0.85, 0.95], tilt: -0.3, life: 0.55, life_jit: 0.5, size: 0.004, size_jit: 0.5, after: 1.0 }, embers: crate::blast_look::WALLBLASTEmbers { count: alloc::vec![18.0, 9.0], speed: alloc::vec![6.0, 4.5], jit: 0.85, cone: alloc::vec![1.1, 1.2], tilt: -0.1, life: 1.2, life_jit: 0.8, size: 0.0035, size_jit: 0.8, after: 2.0 }, chips: crate::blast_look::WALLBLASTChips { count: alloc::vec![48.0, 22.0], speed: alloc::vec![7.5, 4.5], jit: 0.75, cone: alloc::vec![0.7, 0.85], tilt: 0.12, life: 20.0, life_jit: 0.3, size: 0.04, size_jit: 0.7, aspect: 1.7, after: 1.0 }, dust: crate::blast_look::WALLBLASTDust { count: alloc::vec![44.0, 24.0], speed: alloc::vec![5.5, 3.6], jit: 0.6, cone: alloc::vec![0.55, 0.7], tilt: 0.05, life: 1.7, life_jit: 0.3, size: 0.3, size_jit: 0.35 } } }

/// (src/game/blastLook.ts:112 WALL_FLASH: a table nothing writes)
pub fn wall_flash() -> crate::blast_look::WALLFLASH { crate::blast_look::WALLFLASH { color: [1.0, 0.62, 0.25] } }

/// (src/game/blastLook.ts:116 AFTERGLOW: a table nothing writes)
pub fn afterglow_table() -> crate::blast_look::AFTERGLOW { crate::blast_look::AFTERGLOW { after: 4.0, intensity: 50.0, ttl: 0.55, range: 9.0, radius: 0.3, y: 1.0, color: [1.0, 0.5, 0.18] } }

/// (src/game/blastLook.ts:119 STUN_SPARKS: a table nothing writes)
pub fn stun_sparks_table() -> crate::blast_look::STUNSPARKS { crate::blast_look::STUNSPARKS { lobes: crate::blast_look::STUNSPARKSLobes { count: 34.0, speed: 17.0, jit: 0.6, cone: 0.17, fan: 0.3, rise: 0.05, life: 0.3, life_jit: 0.5, size: 0.0025, size_jit: 0.4 }, fan: crate::blast_look::STUNSPARKSFan { count: 40.0, speed: 10.0, jit: 0.7, lift: 0.12, spread: 0.5, life: 0.5, life_jit: 0.5, size: 0.003, size_jit: 0.4 }, fountain: crate::blast_look::STUNSPARKSFountain { count: 22.0, speed: 6.0, jit: 0.5, spread: 0.55, lift: 0.5, life: 0.6, life_jit: 0.4, size: 0.003, size_jit: 0.4 }, embers: crate::blast_look::STUNSPARKSEmbers { count: 18.0, speed: 3.0, jit: 0.6, spread: 0.8, lift: 0.2, life: 1.5, life_jit: 0.5, size: 0.0045, size_jit: 0.4 } } }

/// (src/game/blastLook.ts:135 DOOR_SPARKS: a table nothing writes)
pub fn door_sparks() -> crate::blast_look::DOORSPARKS { crate::blast_look::DOORSPARKS { jet: crate::blast_look::DOORSPARKSJet { count: 56.0, speed: 19.0, jit: 0.65, cone: 0.4, tilt: -0.08, life: 0.45, life_jit: 0.5, size: 0.003, size_jit: 0.5 }, skitter: crate::blast_look::DOORSPARKSSkitter { count: 44.0, speed: 10.0, jit: 0.7, cone: 0.9, tilt: -0.3, life: 0.55, life_jit: 0.5, size: 0.004, size_jit: 0.5 }, back: crate::blast_look::DOORSPARKSBack { count: 22.0, speed: 12.0, jit: 0.65, cone: 0.5, tilt: -0.2, life: 0.3, life_jit: 0.5, size: 0.003, size_jit: 0.5 }, fountain: 16.0, ahead: 0.2, lead: 1.5 } }

/// (src/game/blastLook.ts:154 FRAME)
pub const FRAME: f64 = 0.016666666666666666;
