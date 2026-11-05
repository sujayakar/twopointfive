//! THE BRING-UP'S OWN PART of char_look: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/charLook.ts:63 charLookFrame)
pub fn char_look_frame(g: &mut crate::game::Game) {}

/// Which side's dress a team wears. Pure, translated (charLook.ts:15 sideOfTeam): a rule reads it too (combat.ts: a defender's holster stands out).
pub fn side_of_team(team: crate::teams::Team, trained: Option<bool>) -> crate::anim_gltf::CastSide {
    use crate::anim_gltf::CastSide;
    use crate::teams::Team;
    match team {
        Team::Operative | Team::Player => CastSide::Operator,
        Team::Civilian => CastSide::Civilian,
        _ => if trained.unwrap_or(false) { CastSide::Responder } else { CastSide::Defender },
    }
}
