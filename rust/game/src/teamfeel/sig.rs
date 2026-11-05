#![allow(unused_variables)]
//! Every function and method of `teamfeel`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/teamfeel.ts:81 feelRange)
pub fn feel_range(n: crate::teamfeel::FeelName, key: &str) -> Option<[f64; 3]> { todo!() }

/// (src/game/teamfeel.ts:84 feelTables)
pub fn feel_tables() -> JsMap<crate::teamfeel::FeelName, JsMap<String, crate::Opaque>> { todo!() }

/// (src/game/teamfeel.ts:89 feelDefaults)
pub fn feel_defaults(g: &mut crate::game::Game) -> JsMap<crate::teamfeel::FeelName, JsMap<String, crate::Opaque>> { todo!() }

/// (src/game/teamfeel.ts:93 leaf)
pub fn leaf(t: &JsMap<String, crate::Opaque>, key: &str) -> (JsMap<String, crate::Opaque>, String) { todo!() }

/// (src/game/teamfeel.ts:94 read)
pub fn read(t: &JsMap<String, crate::Opaque>, key: &str) -> crate::Opaque { todo!() }

/// (src/game/teamfeel.ts:95 split)
pub fn split(name: &str) -> (crate::teamfeel::FeelName, String) { todo!() }

/// (src/game/teamfeel.ts:106 saveTeamFeel)
pub fn save_team_feel(g: &mut crate::game::Game, name: &str) { todo!() }

/// (src/game/teamfeel.ts:116 loadTeamFeel)
pub fn load_team_feel(g: &mut crate::game::Game) { todo!() }
