#![allow(unused_variables)]
//! Every function and method of `difficulty`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/difficulty.ts:98 structureOfLevel)
pub fn structure_of_level(file: Option<&str>) -> String { todo!() }

/// (src/game/difficulty.ts:133 leversNow)
pub fn levers_now(g: &mut crate::game::Game) -> Vec<crate::difficulty::LeverDef> { todo!() }

/// (src/game/difficulty.ts:140 fogOffNow)
pub fn fog_off_now(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/difficulty.ts:146 anySet)
pub fn any_set(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/difficulty.ts:147 isRung)
pub fn is_rung(g: &mut crate::game::Game, k: crate::difficulty::LeverKey, v: Option<&str>) -> bool { todo!() }

/// (src/game/difficulty.ts:149 presetLevers)
pub fn preset_levers(g: &mut crate::game::Game, level: crate::difficulty::Difficulty, structure: Option<&str>) -> crate::difficulty::Levers { todo!() }

/// (src/game/difficulty.ts:155 difficultyHere)
pub fn difficulty_here(g: &mut crate::game::Game, structure: &str) { todo!() }

/// (src/game/difficulty.ts:163 same)
pub fn same(a: &crate::difficulty::SameOf, b: &crate::difficulty::SameOf) -> bool { todo!() }

/// (src/game/difficulty.ts:164 tablesOf)
pub fn tables_of(g: &mut crate::game::Game, level: crate::difficulty::Difficulty, set: Option<crate::difficulty::TablesOfSet>) -> crate::difficulty::TablesOf { todo!() }

/// (src/game/difficulty.ts:172 difficultyNow)
pub fn difficulty_now(g: &mut crate::game::Game) -> crate::difficulty::DifficultyNow { todo!() }

/// (src/game/difficulty.ts:178 leversShown)
pub fn levers_shown(g: &mut crate::game::Game, structure: Option<&str>) -> crate::difficulty::LeversShown { todo!() }

/// (src/game/difficulty.ts:188 tablesStand)
pub fn tables_stand(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/difficulty.ts:189 coneOf)
pub fn cone_of(rung: crate::quietfog::QuietRung) -> bool { todo!() }

/// (src/game/difficulty.ts:191 levelOfThree)
pub fn level_of_three(g: &mut crate::game::Game) -> crate::difficulty::Difficulty { todo!() }

/// (src/game/difficulty.ts:196 fogStands)
pub fn fog_stands(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/difficulty.ts:197 shownWhole)
pub fn shown_whole(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/difficulty.ts:199 leversSet)
pub fn levers_set(g: &mut crate::game::Game) -> crate::difficulty::LeversSet { todo!() }

/// (src/game/difficulty.ts:200 leversBase)
pub fn levers_base(g: &mut crate::game::Game) -> crate::difficulty::Difficulty { todo!() }

/// (src/game/difficulty.ts:202 leversMixed)
pub fn levers_mixed(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/difficulty.ts:207 write)
pub fn write(g: &mut crate::game::Game, fog_only: Option<bool>) { todo!() }

/// (src/game/difficulty.ts:214 storeName)
pub fn store_name(level: crate::difficulty::Difficulty) { todo!() }

/// (src/game/difficulty.ts:216 keptHidden)
pub fn kept_hidden(g: &mut crate::game::Game) -> crate::difficulty::KeptHidden { todo!() }

/// (src/game/difficulty.ts:221 storeLevers)
pub fn store_levers(g: &mut crate::game::Game, all: Option<bool>) { todo!() }

/// (src/game/difficulty.ts:225 setDifficulty)
pub fn set_difficulty(g: &mut crate::game::Game, level: crate::difficulty::Difficulty, keep: Option<bool>) { todo!() }

/// (src/game/difficulty.ts:232 setBase)
pub fn set_base(g: &mut crate::game::Game, level: crate::difficulty::Difficulty) { todo!() }

/// (src/game/difficulty.ts:236 adopt)
pub fn adopt(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/difficulty.ts:250 own)
pub fn own(key: crate::difficulty::LeverKey) -> bool { todo!() }

/// (src/game/difficulty.ts:251 setLever)
pub fn set_lever(g: &mut crate::game::Game, key: crate::difficulty::LeverKey, rung: Option<&str>, keep: Option<bool>, structure: Option<&str>) { todo!() }

/// (src/game/difficulty.ts:260 leversFog)
pub fn levers_fog(g: &mut crate::game::Game, load: &mut dyn FnMut(&mut crate::game::Game)) { todo!() }

/// (src/game/difficulty.ts:267 setConeVisit)
pub fn set_cone_visit(g: &mut crate::game::Game, v: Option<bool>) { todo!() }

/// (src/game/difficulty.ts:269 coneVisitNow)
pub fn cone_visit_now(g: &mut crate::game::Game) -> Option<bool> { todo!() }

/// (src/game/difficulty.ts:272 difficultyCustom)
pub fn difficulty_custom(g: &mut crate::game::Game) { todo!() }

/// (src/game/difficulty.ts:283 initDifficulty)
pub fn init_difficulty(g: &mut crate::game::Game, q: crate::Opaque, plain: Option<bool>) { todo!() }

/// (src/game/difficulty.ts:299 keptLevers)
pub fn kept_levers(g: &mut crate::game::Game) -> crate::difficulty::KeptLevers { todo!() }

/// (src/game/difficulty.ts:305 leversKept)
pub fn levers_kept(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/difficulty.ts:310 difficultyHref)
pub fn difficulty_href(g: &mut crate::game::Game, href: &str) -> String { todo!() }

/// (src/game/difficulty.ts:319 leversHref)
pub fn levers_href(g: &mut crate::game::Game, href: &str) -> String { todo!() }

/// (src/game/difficulty.ts:331 squadsOf)
pub fn squads_of(e: Option<&crate::difficulty::SquadsOfE>) -> crate::difficulty::SquadsOf { todo!() }

/// (src/game/difficulty.ts:334 squadsHeld)
pub fn squads_held(g: &mut crate::game::Game, e: Option<&crate::difficulty::SquadsHeldE>) -> crate::difficulty::SquadsHeld { todo!() }

/// (src/game/difficulty.ts:345 difficultyWords)
pub fn difficulty_words(g: &mut crate::game::Game, level: Option<crate::difficulty::DifficultyWordsLevel>, squads: Option<&crate::difficulty::DifficultyWordsSquads>) -> String { todo!() }

/// (src/game/difficulty.ts:358 leverWords)
pub fn lever_words(g: &mut crate::game::Game, key: crate::difficulty::LeverKey, rung: Option<&str>) -> String { todo!() }

/// (src/game/difficulty.ts:360 leversLine)
pub fn levers_line(g: &mut crate::game::Game, all: Option<bool>) -> String { todo!() }
