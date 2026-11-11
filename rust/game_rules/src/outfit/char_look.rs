#![allow(unused_imports, unused_variables)]

/// Which side's dress a team wears. A rule reads it too: a defender's holster stands out.
pub fn side_of_team(team: crate::teams::Team, trained: Option<bool>) -> crate::anim_gltf::CastSide {
    use crate::anim_gltf::CastSide;
    use crate::teams::Team;
    match team {
        Team::Ours => CastSide::Operator,
        Team::Civilian => CastSide::Civilian,
        _ => if trained.unwrap_or(false) { CastSide::Responder } else { CastSide::Defender },
    }
}
