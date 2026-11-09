use crate::game::Game;

/// Does the rule hold on this floor: two squads or more, every one told to stay in its rooms, no lesson on the level.
pub fn stay_ask_here(g: &crate::game::Game, squads: &[crate::scene_scenarios::SquadData]) -> bool {
    squads.len() > 1
        && squads.iter().all(|q| q.may_leave == Some(false))
        && g.level.lesson().is_none()
}
