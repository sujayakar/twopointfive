//! The types of `owner`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum OwnerRule {
    Frag,
    Breaker,
    Press,
    Lane,
    Stand,
    Runner,
    Morale,
    Startle,
    SquadLeg,
    SquadWord,
    LastMan,
    Walk,
    Buddy,
    Needs,
    Squad,
}
impl OwnerRule {
    pub const fn as_str(self) -> &'static str {
        match self {
            OwnerRule::Frag => "frag",
            OwnerRule::Breaker => "breaker",
            OwnerRule::Press => "press",
            OwnerRule::Lane => "lane",
            OwnerRule::Stand => "stand",
            OwnerRule::Runner => "runner",
            OwnerRule::Morale => "morale",
            OwnerRule::Startle => "startle",
            OwnerRule::SquadLeg => "squadLeg",
            OwnerRule::SquadWord => "squadWord",
            OwnerRule::LastMan => "lastMan",
            OwnerRule::Walk => "walk",
            OwnerRule::Buddy => "buddy",
            OwnerRule::Needs => "needs",
            OwnerRule::Squad => "squad",
        }
    }
}

/// (TypeScript: inline at src/game/owner.ts:19 RULES, src/game/owner.ts:21)
#[derive(Clone, Debug)]
pub struct RULES0 {
    pub name: String,
    pub why: [String; 1],
}
impl RULES0 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RULES0 {
            name: String::new(),
            why: core::array::from_fn(|_| String::new()),
        }
    }
}
impl Default for RULES0 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/owner.ts:19 RULES, src/game/owner.ts:22)
#[derive(Clone, Debug)]
pub struct RULES1 {
    pub name: String,
    pub why: [String; 2],
}
impl RULES1 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RULES1 {
            name: String::new(),
            why: core::array::from_fn(|_| String::new()),
        }
    }
}
impl Default for RULES1 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/owner.ts:19 RULES, src/game/owner.ts:23)
#[derive(Clone, Debug)]
pub struct RULES2 {
    pub name: String,
    pub why: [String; 1],
}
impl RULES2 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RULES2 {
            name: String::new(),
            why: core::array::from_fn(|_| String::new()),
        }
    }
}
impl Default for RULES2 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/owner.ts:19 RULES, src/game/owner.ts:24)
#[derive(Clone, Debug)]
pub struct RULES3 {
    pub name: String,
    pub why: [String; 1],
}
impl RULES3 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RULES3 {
            name: String::new(),
            why: core::array::from_fn(|_| String::new()),
        }
    }
}
impl Default for RULES3 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/owner.ts:19 RULES, src/game/owner.ts:26)
#[derive(Clone, Debug)]
pub struct RULES4 {
    pub name: String,
    pub why: [String; 1],
}
impl RULES4 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RULES4 {
            name: String::new(),
            why: core::array::from_fn(|_| String::new()),
        }
    }
}
impl Default for RULES4 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/owner.ts:19 RULES, src/game/owner.ts:27)
#[derive(Clone, Debug)]
pub struct RULES5 {
    pub name: String,
    pub why: [String; 1],
}
impl RULES5 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RULES5 {
            name: String::new(),
            why: core::array::from_fn(|_| String::new()),
        }
    }
}
impl Default for RULES5 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/owner.ts:19 RULES, src/game/owner.ts:28)
#[derive(Clone, Debug)]
pub struct RULES6 {
    pub name: String,
    pub why: [String; 1],
}
impl RULES6 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RULES6 {
            name: String::new(),
            why: core::array::from_fn(|_| String::new()),
        }
    }
}
impl Default for RULES6 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/owner.ts:19 RULES, src/game/owner.ts:29)
#[derive(Clone, Debug)]
pub struct RULES7 {
    pub name: String,
    pub why: [String; 1],
}
impl RULES7 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RULES7 {
            name: String::new(),
            why: core::array::from_fn(|_| String::new()),
        }
    }
}
impl Default for RULES7 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/owner.ts:19 RULES, src/game/owner.ts:30)
#[derive(Clone, Debug)]
pub struct RULES8 {
    pub name: String,
    pub why: [String; 1],
}
impl RULES8 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RULES8 {
            name: String::new(),
            why: core::array::from_fn(|_| String::new()),
        }
    }
}
impl Default for RULES8 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/owner.ts:19 RULES, src/game/owner.ts:31)
#[derive(Clone, Debug)]
pub struct RULES9 {
    pub name: String,
    pub why: [String; 1],
}
impl RULES9 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RULES9 {
            name: String::new(),
            why: core::array::from_fn(|_| String::new()),
        }
    }
}
impl Default for RULES9 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/owner.ts:19 RULES, src/game/owner.ts:32)
#[derive(Clone, Debug)]
pub struct RULES10 {
    pub name: String,
    pub why: [String; 1],
}
impl RULES10 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RULES10 {
            name: String::new(),
            why: core::array::from_fn(|_| String::new()),
        }
    }
}
impl Default for RULES10 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/owner.ts:19 RULES, src/game/owner.ts:33)
#[derive(Clone, Debug)]
pub struct RULES11 {
    pub name: String,
    pub why: [String; 1],
}
impl RULES11 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RULES11 {
            name: String::new(),
            why: core::array::from_fn(|_| String::new()),
        }
    }
}
impl Default for RULES11 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/owner.ts:19 RULES, src/game/owner.ts:34)
#[derive(Clone, Debug)]
pub struct RULES12 {
    pub name: String,
    pub why: [String; 1],
}
impl RULES12 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RULES12 {
            name: String::new(),
            why: core::array::from_fn(|_| String::new()),
        }
    }
}
impl Default for RULES12 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/owner.ts:19 RULES, src/game/owner.ts:35)
#[derive(Clone, Debug)]
pub struct RULES13 {
    pub name: String,
    pub why: [String; 1],
}
impl RULES13 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RULES13 {
            name: String::new(),
            why: core::array::from_fn(|_| String::new()),
        }
    }
}
impl Default for RULES13 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/owner.ts:19 RULES, src/game/owner.ts:36)
#[derive(Clone, Debug)]
pub struct RULES14 {
    pub name: String,
    pub why: [String; 1],
}
impl RULES14 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RULES14 {
            name: String::new(),
            why: core::array::from_fn(|_| String::new()),
        }
    }
}
impl Default for RULES14 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/owner.ts:46 OWN, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1597)
#[derive(Clone, Copy, Debug)]
pub struct OWN {
    pub frag: f64,
    pub breaker: f64,
    pub press: f64,
    pub lane: f64,
    pub stand: f64,
    pub runner: f64,
    pub morale: f64,
    pub startle: f64,
    pub squad_leg: f64,
    pub squad_word: f64,
    pub last_man: f64,
    pub walk: f64,
    pub buddy: f64,
    pub needs: f64,
    pub squad: f64,
}
impl OWN {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OWN {
            frag: 0.0,
            breaker: 0.0,
            press: 0.0,
            lane: 0.0,
            stand: 0.0,
            runner: 0.0,
            morale: 0.0,
            startle: 0.0,
            squad_leg: 0.0,
            squad_word: 0.0,
            last_man: 0.0,
            walk: 0.0,
            buddy: 0.0,
            needs: 0.0,
            squad: 0.0,
        }
    }
}
impl Default for OWN { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/owner.ts:54 OWNER_QA, src/game/owner.ts:54)
#[derive(Clone, Copy, Debug)]
pub struct OWNERQA {
    pub read: Option<crate::FnField>,
    pub drop: f64,
}
impl OWNERQA {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OWNERQA {
            read: None,
            drop: 0.0,
        }
    }
}
impl Default for OWNERQA { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/owner.ts:108 WITNESSES, src/game/owner.ts:108)
#[derive(Clone, Debug)]
pub struct OwnerModWITNESSESItem {
    pub rule: f64,
    pub name: String,
    pub says: crate::FnField,
    pub at_read: bool,
}
impl OwnerModWITNESSESItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OwnerModWITNESSESItem {
            rule: 0.0,
            name: String::new(),
            says: crate::FnField,
            at_read: false,
        }
    }
}
impl Default for OwnerModWITNESSESItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum RowOf {
    Arm0(crate::owner::RULES0),
    Arm1(crate::owner::RULES1),
    Arm2(crate::owner::RULES2),
    Arm3(crate::owner::RULES3),
    Arm4(crate::owner::RULES4),
    Arm5(crate::owner::RULES5),
    Arm6(crate::owner::RULES6),
    Arm7(crate::owner::RULES7),
    Arm8(crate::owner::RULES8),
    Arm9(crate::owner::RULES9),
    Arm10(crate::owner::RULES10),
    Arm11(crate::owner::RULES11),
    Arm12(crate::owner::RULES12),
    Arm13(crate::owner::RULES13),
    Arm14(crate::owner::RULES14),
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum OwnerOf {
    Frag,
    Dead,
    Breaker,
    Held,
    Press,
    Lane,
    Stand,
    Script,
    Runner,
    Morale,
    Startle,
    SquadLeg,
    SquadWord,
    LastMan,
    Walk,
    Buddy,
    Needs,
    Squad,
}
impl OwnerOf {
    pub const fn as_str(self) -> &'static str {
        match self {
            OwnerOf::Frag => "frag",
            OwnerOf::Dead => "dead",
            OwnerOf::Breaker => "breaker",
            OwnerOf::Held => "held",
            OwnerOf::Press => "press",
            OwnerOf::Lane => "lane",
            OwnerOf::Stand => "stand",
            OwnerOf::Script => "script",
            OwnerOf::Runner => "runner",
            OwnerOf::Morale => "morale",
            OwnerOf::Startle => "startle",
            OwnerOf::SquadLeg => "squadLeg",
            OwnerOf::SquadWord => "squadWord",
            OwnerOf::LastMan => "lastMan",
            OwnerOf::Walk => "walk",
            OwnerOf::Buddy => "buddy",
            OwnerOf::Needs => "needs",
            OwnerOf::Squad => "squad",
        }
    }
}

/// (TypeScript: inline at src/game/owner.ts:112 ownerDisagrees ->, src/game/owner.ts:112)
#[derive(Clone, Debug)]
pub struct OwnerDisagreesItem {
    pub name: String,
    pub record: bool,
    pub rule: bool,
}
impl OwnerDisagreesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OwnerDisagreesItem {
            name: String::new(),
            record: false,
            rule: false,
        }
    }
}
impl Default for OwnerDisagreesItem { fn default() -> Self { Self::zeroed() } }

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

/// (src/game/owner.ts:19 RULES: a table nothing writes)
pub fn rules() -> (crate::owner::RULES0, crate::owner::RULES1, crate::owner::RULES2, crate::owner::RULES3, crate::owner::RULES4, crate::owner::RULES5, crate::owner::RULES6, crate::owner::RULES7, crate::owner::RULES8, crate::owner::RULES9, crate::owner::RULES10, crate::owner::RULES11, crate::owner::RULES12, crate::owner::RULES13, crate::owner::RULES14) { (crate::owner::RULES0 { name: String::from("frag"), why: [String::from("a live grenade he knows of has a line to him")] }, crate::owner::RULES1 { name: String::from("runner"), why: [String::from("has said he is done"), String::from("dropped it and ran")] }, crate::owner::RULES2 { name: String::from("morale"), why: [String::from("a blast has him ducked or broken")] }, crate::owner::RULES3 { name: String::from("startle"), why: [String::from("caught alone at first sight, running for cover")] }, crate::owner::RULES4 { name: String::from("stand"), why: [String::from("one of the last stand: in its room, on his way to it, or told to hold where he is")] }, crate::owner::RULES5 { name: String::from("squadLeg"), why: [String::from("his squad has sent him, or is about to")] }, crate::owner::RULES6 { name: String::from("squadWord"), why: [String::from("his squad has heard and not yet said what it does, or has him going round or looking")] }, crate::owner::RULES7 { name: String::from("lastMan"), why: [String::from("the last man up, at his act")] }, crate::owner::RULES8 { name: String::from("lane"), why: [String::from("held short of a covered lane, or looking round its corner")] }, crate::owner::RULES9 { name: String::from("press"), why: [String::from("the press has him, or he stays to keep the room it leaves")] }, crate::owner::RULES10 { name: String::from("walk"), why: [String::from("walking to the post of a mate who does not answer, nothing learned of the team by it")] }, crate::owner::RULES11 { name: String::from("breaker"), why: [String::from("out for the lights, or waiting to be sent")] }, crate::owner::RULES12 { name: String::from("buddy"), why: [String::from("a colleague is on his way to him, or the two have just paired up")] }, crate::owner::RULES13 { name: String::from("needs"), why: [String::from("off for a coffee, a smoke, some water or a stretch")] }, crate::owner::RULES14 { name: String::from("squad"), why: [String::from("at rest, his squad's call")] }) }

/// (src/game/owner.ts:40 TOP)
pub const TOP: f64 = 14.0;

/// (src/game/owner.ts:46 OWN: a table nothing writes)
pub fn own() -> crate::owner::OWN { crate::owner::OWN { frag: 16384.0, breaker: 8.0, press: 32.0, lane: 64.0, stand: 1024.0, runner: 8192.0, morale: 4096.0, startle: 2048.0, squad_leg: 512.0, squad_word: 256.0, last_man: 128.0, walk: 16.0, buddy: 4.0, needs: 2.0, squad: 1.0 } }

/// (src/game/owner.ts:54 OWNER_QA: a table nothing writes)
pub fn owner_qa() -> crate::owner::OWNERQA { crate::owner::OWNERQA { read: None, drop: 0.0 } }

/// (src/game/owner.ts:90 SCRIPT_RANK: a table nothing writes)
pub fn script_rank() -> f64 { 12.0 }
