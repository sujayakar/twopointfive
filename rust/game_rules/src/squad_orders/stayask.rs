use crate::game::Game;

/// Does the rule hold on this floor: two squads or more, every one told to stay in its rooms, no lesson on the level. (stayask.ts stayAskHere)
pub fn stay_ask_here(g: &crate::game::Game, squads: &[crate::scene_scenarios::SquadData]) -> bool {
    squads.len() > 1
        && squads.iter().all(|q| q.may_leave == Some(false))
        && !g.level.scenarios.as_ref().is_some_and(|s| s.lesson.is_some())
}
