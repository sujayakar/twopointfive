use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// This game's wall reaction, made on first asking: by then the destruction engine stands. It lives at
/// `g.m.wallreact.wall_react`; `unsub`, set by its constructor, says it was made.
pub(in crate::room_fight) fn wall_react_of(g: &mut crate::game::Game) {
    if !g.m.wallreact.wall_react.unsub {
        g.m.wallreact.wall_react = wall_react_new(g);
    }
}

/// Once a frame, before the squad's books and the room fight.
pub fn wall_react_tick_2(g: &mut crate::game::Game) {
    wall_react_of(g);
    wall_react_tick(g);
}

/// A restart.
pub fn wall_react_reset_2(g: &mut crate::game::Game) {
    if g.m.wallreact.wall_react.unsub {
        wall_react_reset(g);
    }
}

/// roomfight.ts applyWatch: a hole he knows of may take his gun. An unmade one has no breaches.
pub(in crate::room_fight) fn wall_react_look_2(g: &mut crate::game::Game, gd: GuardId, pt: &mut V3) {
    if !g.m.wallreact.wall_react.breaches.is_empty() {
        wall_react_look(g, gd, pt);
    }
}

/// tactics.ts: is this plan stale by a hole one of its men has learned of since it was dealt.
pub fn wall_react_replan_2(g: &mut crate::game::Game, team: crate::teams::Team, since: f64, men: &[GuardId]) -> bool {
    !g.m.wallreact.wall_react.breaches.is_empty() && wall_react_replan(g, team, since, men)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// With nothing made, the three askers answer from their own guards and never reach the unmade reaction's methods.
    #[test]
    fn an_unmade_reaction_is_not_asked() {
        let mut g = crate::game::Game::blank();
        wall_react_reset_2(&mut g);
        assert!(!g.m.wallreact.wall_react.unsub);
        let gd = GuardId(0);
        let mut pt: V3 = [1.0, 2.0, 3.0];
        wall_react_look_2(&mut g, gd, &mut pt);
        assert_eq!(pt, [1.0, 2.0, 3.0]);
        assert!(!wall_react_replan_2(&mut g, crate::teams::Team::Guard, 0.0, &[gd]));
    }
}
