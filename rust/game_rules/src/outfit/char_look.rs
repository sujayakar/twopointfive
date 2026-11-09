#![allow(unused_imports, unused_variables)]
//! THE BRING-UP'S OWN PART of char_look: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)

/// Which side's dress a team wears. Pure, translated: a rule reads it too.
pub fn side_of_team(team: crate::teams::Team, trained: Option<bool>) -> crate::anim_gltf::CastSide {
    use crate::anim_gltf::CastSide;
    use crate::teams::Team;
    match team {
        Team::Operative | Team::Player => CastSide::Operator,
        Team::Civilian => CastSide::Civilian,
        _ => if trained.unwrap_or(false) { CastSide::Responder } else { CastSide::Defender },
    }
}
