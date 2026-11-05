#![allow(unused_variables)]
//! Every function and method of `teamfeel`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/teamfeel.ts:81 feelRange)
pub fn feel_range(n: crate::teamfeel::FeelName, key: &str) -> Option<Option<[f64; 3]>> { todo!() }

/// (src/game/teamfeel.ts:84 feelTables)
pub fn feel_tables() -> JsMap<crate::teamfeel::FeelName, JsMap<String, crate::Opaque>> { todo!() }

/// (src/game/teamfeel.ts:89 feelDefaults)
pub fn feel_defaults(g: &mut crate::game::Game) -> JsMap<String, crate::teamfeel::FeelValue> { todo!() }

/// (src/game/teamfeel.ts:93 leaf)
pub fn leaf(key: &str, k: &mut crate::knobs::Knobs, n: crate::teamfeel::FeelName, v: crate::teamfeel::FeelValue) { todo!() }

/// (src/game/teamfeel.ts:94 read)
pub fn read(key: &str, k: &crate::knobs::Knobs, n: crate::teamfeel::FeelName) -> crate::teamfeel::FeelValue { todo!() }

/// (src/game/teamfeel.ts:95 split)
pub fn split(name: &str) -> Option<(crate::teamfeel::FeelName, String)> { todo!() }

/// (src/game/teamfeel.ts:106 saveTeamFeel)
pub fn save_team_feel(g: &mut crate::game::Game, name: &str) { todo!() }

/// (src/game/teamfeel.ts:116 loadTeamFeel)
pub fn load_team_feel(g: &mut crate::game::Game) { todo!() }
