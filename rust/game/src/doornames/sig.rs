#![allow(unused_variables)]
//! Every function and method of `doornames`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/doornames.ts:37 shorthand)
pub fn shorthand(g: &mut crate::game::Game, t: &str) -> bool { todo!() }

/// (src/game/doornames.ts:39 shorthandId)
pub fn shorthand_id(g: &mut crate::game::Game, id_2: &str) -> bool { todo!() }

/// (src/game/doornames.ts:41 bare)
pub fn bare(s: &str) -> String { todo!() }

/// (src/game/doornames.ts:45 doorNamesOf)
pub fn door_names_of(g: &mut crate::game::Game, ids: &[String], written: Option<&JsMap<String, String>>) -> JsMap<String, String> { todo!() }

/// (src/game/doornames.ts:69 authoredDoorNames)
pub fn authored_door_names(g: &mut crate::game::Game, ids: &[String]) -> JsMap<String, String> { todo!() }

/// (src/game/doornames.ts:75 nameDoors)
pub fn name_doors(g: &mut crate::game::Game, office: bool, doors: &[DoorId]) { todo!() }

/// (src/game/doornames.ts:85 doorWrittenName)
pub fn door_written_name(g: &mut crate::game::Game, d: DoorId) -> Option<String> { todo!() }

/// (src/game/doornames.ts:87 doorSaidName)
pub fn door_said_name(g: &mut crate::game::Game, d: DoorId) -> Option<String> { todo!() }
