pub use ::game_types::teams::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use alloc::string::String;

/// A scenario's defender, for the words of a line both sides' AI says: a spawned hostile, up against a team.
pub fn defender_words(g: &Game, gd: GuardId) -> bool {
    g.team(gd) == Team::Hostile
}

/// The name a man's own side calls him by in a line; none for a defender.
pub fn spoken_name(g: &Game, o: GuardId) -> Option<String> {
    if defender_words(g, o) { None } else { Some(g.gd[o].callsign.clone()) }
}

/// The side of the body with this id; the place's own for a stray.
pub fn team_of_id(g: &Game, id: f64) -> Team {
    if let Some(p) = crate::seats::man_by_id(g, id) { return g.ch[g.pl[p].ch].team; }
    g.guards.iter().find(|&&x| g.ch[g.gd[x].ch].id == id).map_or(Team::Guard, |&x| g.team(x))
}

/// Is the man he has in mind a player's.
pub fn foe_is_player(g: &Game, gd: GuardId) -> bool { g.gd[gd].foe.is_some_and(|f| crate::character::is_player(g, f)) }

/// The AI man he has in mind; a player's is none.
pub fn foe_guard(g: &Game, gd: GuardId) -> Option<GuardId> { crate::character::guard_of(g, g.gd[gd].foe?) }

/// His foe is out of the fight. False with nobody.
pub fn foe_down(g: &Game, gd: GuardId) -> bool { g.gd[gd].foe.is_some_and(|f| crate::condition::down(g, f)) }

/// His foe is crouched: the player's crouch, or the AI man's crouch pose.
pub fn foe_crouched(g: &Game, gd: GuardId) -> bool {
    let Some(f) = g.gd[gd].foe else { return false };
    match g.ch[f].who {
        crate::character::Who::Player(p) => g.pl[p].crouch,
        crate::character::Who::Guard(_) => g.ch[f].anim.crouch > 0.5,
    }
}
