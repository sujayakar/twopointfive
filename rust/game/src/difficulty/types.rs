//! The types of `difficulty`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// what difficulty.ts same() compares: the levers as they stand, or one of the two defenders' tables (difficulty.ts:163). Two of different arms are not the same.
#[derive(Clone, Debug)]
pub enum SameOf {
    Levers { shown: crate::difficulty::LeversShown },
    Pressure { table: crate::knobs::PressureK },
    Digin { table: crate::knobs::DiginK },
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Difficulty {
    Hard,
    Standard,
}
impl Difficulty {
    pub const fn as_str(self) -> &'static str {
        match self {
            Difficulty::Hard => "hard",
            Difficulty::Standard => "standard",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FogRung {
    Team,
    Own,
    Cones,
}
impl FogRung {
    pub const fn as_str(self) -> &'static str {
        match self {
            FogRung::Team => "team",
            FogRung::Own => "own",
            FogRung::Cones => "cones",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum KitRung {
    None,
    Stuns,
}
impl KitRung {
    pub const fn as_str(self) -> &'static str {
        match self {
            KitRung::None => "none",
            KitRung::Stuns => "stuns",
        }
    }
}

/// (TypeScript: Levers, src/game/difficulty.ts:106)
#[derive(Clone, Copy, Debug)]
pub struct Levers {
    pub fog: crate::quietfog::QuietRung,
    pub ai: crate::difficulty::Difficulty,
    pub kit: crate::difficulty::KitRung,
    pub told: Option<crate::told::ToldRung>,
}
impl Levers {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Levers {
            fog: crate::quietfog::QuietRung::Team,
            ai: crate::difficulty::Difficulty::Hard,
            kit: crate::difficulty::KitRung::None,
            told: None,
        }
    }
}
impl Default for Levers { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LeverKey {
    Fog,
    Told,
    Ai,
    Kit,
}
impl LeverKey {
    pub const fn as_str(self) -> &'static str {
        match self {
            LeverKey::Fog => "fog",
            LeverKey::Told => "told",
            LeverKey::Ai => "ai",
            LeverKey::Kit => "kit",
        }
    }
}

/// (TypeScript: LeverDef, src/game/difficulty.ts:109)
#[derive(Clone, Debug)]
pub struct LeverDef {
    pub key: crate::difficulty::LeverKey,
    pub param: String,
    pub label: String,
    pub rungs: Vec<crate::difficulty::LeverDefRungsItem>,
}
impl LeverDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LeverDef {
            key: crate::difficulty::LeverKey::Fog,
            param: String::new(),
            label: String::new(),
            rungs: Vec::new(),
        }
    }
}
impl Default for LeverDef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/difficulty.ts:109 LeverDef.rungs, src/game/difficulty.ts:109)
#[derive(Clone, Debug)]
pub struct LeverDefRungsItem {
    pub id: String,
    pub label: String,
    pub words: String,
}
impl LeverDefRungsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LeverDefRungsItem {
            id: String::new(),
            label: String::new(),
            words: String::new(),
        }
    }
}
impl Default for LeverDefRungsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/difficulty.ts:131 FOG_OFF_RUNG, src/game/difficulty.ts:131)
#[derive(Clone, Debug)]
pub struct FOGOFFRUNG {
    pub id: String,
    pub label: String,
    pub words: String,
}
impl FOGOFFRUNG {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FOGOFFRUNG {
            id: String::new(),
            label: String::new(),
            words: String::new(),
        }
    }
}
impl Default for FOGOFFRUNG { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/difficulty.ts:145 SET, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct DifficultyModSET {
    pub fog: Option<crate::quietfog::QuietRung>,
    pub ai: Option<crate::difficulty::Difficulty>,
    pub kit: Option<crate::difficulty::KitRung>,
    pub told: Option<crate::told::ToldRung>,
}
impl DifficultyModSET {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DifficultyModSET {
            fog: None,
            ai: None,
            kit: None,
            told: None,
        }
    }
}
impl Default for DifficultyModSET { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/difficulty.ts:244 HELD, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct DifficultyModHELD {
    pub fog: Option<crate::quietfog::QuietRung>,
    pub ai: Option<crate::difficulty::Difficulty>,
    pub kit: Option<crate::difficulty::KitRung>,
    pub told: Option<crate::told::ToldRung>,
}
impl DifficultyModHELD {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DifficultyModHELD {
            fog: None,
            ai: None,
            kit: None,
            told: None,
        }
    }
}
impl Default for DifficultyModHELD { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/difficulty.ts:337 SQUAD_HARD, src/game/difficulty.ts:337)
#[derive(Clone, Debug)]
pub struct SQUADHARDItem {
    pub file: String,
    pub structure: String,
    pub words: String,
}
impl SQUADHARDItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SQUADHARDItem {
            file: String::new(),
            structure: String::new(),
            words: String::new(),
        }
    }
}
impl Default for SQUADHARDItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/difficulty.ts:164 tablesOf(set), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct TablesOfSet {
    pub fog: Option<crate::quietfog::QuietRung>,
    pub ai: Option<crate::difficulty::Difficulty>,
    pub kit: Option<crate::difficulty::KitRung>,
    pub told: Option<crate::told::ToldRung>,
}
impl TablesOfSet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TablesOfSet {
            fog: None,
            ai: None,
            kit: None,
            told: None,
        }
    }
}
impl Default for TablesOfSet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/difficulty.ts:164 tablesOf ->, src/game/difficulty.ts:164)
#[derive(Clone, Copy, Debug)]
pub struct TablesOf {
    pub p: crate::knobs::PressureK,
    pub d: crate::knobs::DiginK,
}
impl TablesOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TablesOf {
            p: crate::knobs::PressureK::zeroed(),
            d: crate::knobs::DiginK::zeroed(),
        }
    }
}
impl Default for TablesOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DifficultyNow {
    Hard,
    Standard,
    Custom,
}
impl DifficultyNow {
    pub const fn as_str(self) -> &'static str {
        match self {
            DifficultyNow::Hard => "hard",
            DifficultyNow::Standard => "standard",
            DifficultyNow::Custom => "custom",
        }
    }
}

/// (TypeScript: inline at src/game/difficulty.ts:178 leversShown ->, src/game/difficulty.ts:178)
#[derive(Clone, Copy, Debug)]
pub struct LeversShown {
    pub fog: Option<crate::quietfog::QuietRung>,
    pub ai: Option<crate::difficulty::Difficulty>,
    pub kit: Option<crate::difficulty::KitRung>,
    pub told: Option<crate::told::ToldRung>,
}
impl LeversShown {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LeversShown {
            fog: None,
            ai: None,
            kit: None,
            told: None,
        }
    }
}
impl Default for LeversShown { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/difficulty.ts:199 leversSet ->, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct LeversSet {
    pub fog: Option<crate::quietfog::QuietRung>,
    pub ai: Option<crate::difficulty::Difficulty>,
    pub kit: Option<crate::difficulty::KitRung>,
    pub told: Option<crate::told::ToldRung>,
}
impl LeversSet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LeversSet {
            fog: None,
            ai: None,
            kit: None,
            told: None,
        }
    }
}
impl Default for LeversSet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/difficulty.ts:216 keptHidden ->, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct KeptHidden {
    pub fog: Option<crate::quietfog::QuietRung>,
    pub ai: Option<crate::difficulty::Difficulty>,
    pub kit: Option<crate::difficulty::KitRung>,
    pub told: Option<crate::told::ToldRung>,
}
impl KeptHidden {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KeptHidden {
            fog: None,
            ai: None,
            kit: None,
            told: None,
        }
    }
}
impl Default for KeptHidden { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/difficulty.ts:299 keptLevers ->, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct KeptLevers {
    pub fog: Option<crate::quietfog::QuietRung>,
    pub ai: Option<crate::difficulty::Difficulty>,
    pub kit: Option<crate::difficulty::KitRung>,
    pub told: Option<crate::told::ToldRung>,
}
impl KeptLevers {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KeptLevers {
            fog: None,
            ai: None,
            kit: None,
            told: None,
        }
    }
}
impl Default for KeptLevers { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/difficulty.ts:331 squadsOf(e), src/game/difficulty.ts:331)
#[derive(Clone, Debug)]
pub struct SquadsOfE {
    pub level: Option<String>,
    pub squads: Option<bool>,
}
impl SquadsOfE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadsOfE {
            level: None,
            squads: None,
        }
    }
}
impl Default for SquadsOfE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum SquadsOf {
    Text(String),
    Bool(bool),
}

/// (TypeScript: inline at src/game/difficulty.ts:334 squadsHeld(e), src/game/difficulty.ts:334)
#[derive(Clone, Debug)]
pub struct SquadsHeldE {
    pub level: Option<String>,
    pub squads: Option<bool>,
    pub kind: Option<String>,
}
impl SquadsHeldE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadsHeldE {
            level: None,
            squads: None,
            kind: None,
        }
    }
}
impl Default for SquadsHeldE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum SquadsHeld {
    Text(String),
    Bool(bool),
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DifficultyWordsLevel {
    Hard,
    Standard,
    Custom,
}
impl DifficultyWordsLevel {
    pub const fn as_str(self) -> &'static str {
        match self {
            DifficultyWordsLevel::Hard => "hard",
            DifficultyWordsLevel::Standard => "standard",
            DifficultyWordsLevel::Custom => "custom",
        }
    }
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DifficultyWordsSquads {
    Text(String),
    Bool(bool),
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct DifficultyMod {
    /// (src/game/difficulty.ts:95 HERE)
    pub here: String,
    /// (src/game/difficulty.ts:144 BASE)
    pub base: crate::difficulty::Difficulty,
    /// (src/game/difficulty.ts:145 SET)
    pub set: crate::difficulty::DifficultyModSET,
    /// (src/game/difficulty.ts:203 coneVisit)
    pub cone_visit: Option<bool>,
    /// (src/game/difficulty.ts:204 ownWas)
    pub own_was: Option<bool>,
    /// (src/game/difficulty.ts:205 ownFor)
    pub own_for: Option<crate::quietfog::QuietRung>,
    /// (src/game/difficulty.ts:244 HELD)
    pub held: Option<crate::difficulty::DifficultyModHELD>,
}
impl DifficultyMod {
    pub fn blank() -> Self {
        DifficultyMod {
            here: String::from(""),
            base: crate::difficulty::Difficulty::Standard,
            set: crate::difficulty::DifficultyModSET { fog: None, ai: None, kit: None, told: None },
            cone_visit: None,
            own_was: None,
            own_for: None,
            held: None,
        }
    }
}

/// (src/game/difficulty.ts:40 DIFFICULTIES: a table nothing writes)
pub fn difficulties() -> Vec<String> { alloc::vec![String::from("standard"), String::from("hard")] }

/// (src/game/difficulty.ts:97 LEVEL_STRUCTURE: a table nothing writes)
pub fn level_structure() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("block.json"), String::from("the block")); t } }

/// (src/game/difficulty.ts:110 LEVERS: a table nothing writes)
pub fn levers() -> Vec<crate::difficulty::LeverDef> { alloc::vec![crate::difficulty::LeverDef { key: crate::difficulty::LeverKey::Fog, param: String::from("fog"), label: String::from("Fog of war"), rungs: alloc::vec![crate::difficulty::LeverDefRungsItem { id: String::from("team"), label: String::from("all round"), words: String::from("everyone on your team sees all around them, and you see what they see") }, crate::difficulty::LeverDefRungsItem { id: String::from("cones"), label: String::from("cones"), words: String::from("each teammate sees only the cone he's looking down, and you see what they see. The floor behind everyone's back is dark, and they call out what you can't see") }, crate::difficulty::LeverDefRungsItem { id: String::from("own"), label: String::from("own eyes"), words: String::from("only the cone you're looking down lifts the fog, and your teammates call out what they see") }] }, crate::difficulty::LeverDef { key: crate::difficulty::LeverKey::Ai, param: String::from("enemyai"), label: String::from("Enemy AI"), rungs: alloc::vec![crate::difficulty::LeverDefRungsItem { id: String::from("standard"), label: String::from("standard"), words: String::from("the usual defenders. A defender who hears the fight will come to it on his own") }, crate::difficulty::LeverDefRungsItem { id: String::from("hard"), label: String::from("hard"), words: String::from("they work together. When you hit a room they'll come at it as a group and around through a second door. They'll get ready if they hear you early, and they'll stand and fight where they meet you") }] }, crate::difficulty::LeverDef { key: crate::difficulty::LeverKey::Kit, param: String::from("enemykit"), label: String::from("Enemy kit"), rungs: alloc::vec![crate::difficulty::LeverDefRungsItem { id: String::from("none"), label: String::from("guns"), words: String::from("the defenders carry their guns and nothing else") }, crate::difficulty::LeverDefRungsItem { id: String::from("stuns"), label: String::from("stuns"), words: String::from("each defender carries a stun charge, and he'll throw it through the door first when he comes at a room you're in") }] }] }

/// (src/game/difficulty.ts:125 TOLD_LEVER: a table nothing writes)
pub fn told_lever() -> crate::difficulty::LeverDef { crate::difficulty::LeverDef { key: crate::difficulty::LeverKey::Told, param: String::from("told"), label: String::from("You're told"), rungs: alloc::vec![crate::difficulty::LeverDefRungsItem { id: String::from("guided"), label: String::from("guided"), words: String::from("a mark shows where your team last saw an enemy, and it stays for a while") }, crate::difficulty::LeverDefRungsItem { id: String::from("standard"), label: String::from("standard"), words: String::from("a mark shows for a few seconds where your team last saw an enemy") }, crate::difficulty::LeverDefRungsItem { id: String::from("bare"), label: String::from("bare"), words: String::from("you don't get a mark when your team loses sight of an enemy") }] } }

/// (src/game/difficulty.ts:131 FOG_OFF_RUNG: a table nothing writes)
pub fn fog_off_rung() -> crate::difficulty::FOGOFFRUNG { crate::difficulty::FOGOFFRUNG { id: String::from("off"), label: String::from("off"), words: String::from("there's no fog, so you and your team always know where every enemy is") } }

/// (src/game/difficulty.ts:142 LEVER_STUNS)
pub const LEVER_STUNS: f64 = 1.0;

/// (src/game/difficulty.ts:161 LS)
pub const LS: &str = "twopointsix.difficulty";

/// (src/game/difficulty.ts:161 LS_LEVERS)
pub const LS_LEVERS: &str = "twopointsix.levers";

/// (src/game/difficulty.ts:337 SQUAD_HARD: a table nothing writes)
pub fn squad_hard() -> Vec<crate::difficulty::SQUADHARDItem> { alloc::vec![crate::difficulty::SQUADHARDItem { file: String::from("block-squads.json"), structure: String::from("the block, three squads"), words: String::from("They don't run at the first noise. You'll hear them hold on the radio and let you commit to a room, and then the pair that comes around will be set at its other door while you're still going in") }, crate::difficulty::SQUADHARDItem { file: String::from("compound.json"), structure: String::from("the compound"), words: String::from("A squad gets to one of its rooms faster when you hit it, and nobody walks into a room that's already lost. You'll hear them call it gone on the radio, and they'll be on their own doors when you get there") }, crate::difficulty::SQUADHARDItem { file: String::from("barracks.json"), structure: String::from("the barracks"), words: String::from("A pair sent around to hold a lost room's other door won't stay out there once the squad it went to help stops answering. You'll hear them call \"back on our doors\", and they'll be in their own rooms again and come at you when you take the next one") }, crate::difficulty::SQUADHARDItem { file: String::from("warehouse.json"), structure: String::from("the warehouse"), words: String::from("") }, crate::difficulty::SQUADHARDItem { file: String::from("block.json"), structure: String::from("the block"), words: String::from("") }] }
