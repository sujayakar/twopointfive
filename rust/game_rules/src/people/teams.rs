pub use ::game_types::teams::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use alloc::string::String;

/// Does side `a` treat side `b` as an enemy: the whole rule of who fights whom.
pub fn is_hostile(g: &crate::game::Game, a: crate::teams::Team, b: crate::teams::Team) -> bool {
    cell(&g.m.teams, a, b)
}

/// One cell of HOSTILE; the table holds all 25 from the start and only ever has cells rewritten.
fn cell(t: &TeamsMod, a: Team, b: Team) -> bool {
    *t.hostile.get(a).unwrap().get(b).unwrap()
}

/// A scenario's defender, for the words of a line both sides' AI says: a spawned hostile, up against a team.
pub fn defender_words(g: &crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].unit.is_some() && g.gd[gd].team == Team::Hostile
}

/// The name a man's own side calls him by in a line; none for a defender.
pub fn spoken_name(g: &crate::game::Game, o: GuardId) -> Option<String> {
    if defender_words(g, o) { None } else { Some(g.gd[o].callsign.clone()) }
}

/// The quiet mission with a partner: on for that run, the night staff and an operative are foes; off, neutral.
pub fn staff_vs_operatives(g: &mut crate::game::Game, on: bool) {
    count_switch(&mut g.m.teams, on);
}

/// The switch is counted, never goes below nought, and its answer is written in the table's two cells, which guards.ts reads directly.
fn count_switch(t: &mut TeamsMod, on: bool) {
    t.staff_ops = m::max(0.0, t.staff_ops + if on { 1.0 } else { -1.0 });
    let foes = t.staff_ops > 0.0;
    t.hostile.get_mut(Team::Operative).unwrap().set(Team::Guard, foes);
    t.hostile.get_mut(Team::Guard).unwrap().set(Team::Operative, foes);
}

/// The side a body is on: a player's, or that of the Guard whose body it is ('guard' for a stray).
pub(super) fn team_of_char(g: &mut crate::game::Game, c_2: CharId) -> crate::teams::Team {
    if crate::seats::man_of(g, Some(c_2)).is_some() { return Team::Player; }
    g.guards.iter().find(|&&x| g.gd[x].ch == c_2).map_or(Team::Guard, |&x| g.gd[x].team)
}

/// The side of the body with this id: 'player' for a player's, else the living or dead Guard's, else 'guard'.
pub fn team_of_id(g: &crate::game::Game, id_2: f64) -> crate::teams::Team {
    if crate::seats::man_by_id(g, id_2).is_some() { return Team::Player; }
    g.guards.iter().find(|&&x| g.ch[g.gd[x].ch].id == id_2).map_or(Team::Guard, |&x| g.gd[x].team)
}

/// Is a man of `side` an enemy of this man's side.
pub fn hostile_to(g: &crate::game::Game, gd: GuardId, side: crate::teams::Team) -> bool { is_hostile(g, g.gd[gd].team, side) }

/// The body his foe is: the player he has in mind, or the hostile AI man; none when he has nobody.
pub fn foe_char(g: &crate::game::Game, gd: GuardId) -> Option<CharId> {
    match g.gd[gd].foe {
        Some(crate::game::Foe::Player) => {
            let p = crate::seats::faced_man(g, gd);
            Some(g.pl[p].ch)
        }
        Some(crate::game::Foe::Guard(f)) => Some(g.gd[f].ch),
        None => None,
    }
}

/// His foe is out of the fight: the player down, or the man dead. False with nobody.
pub fn foe_down(g: &crate::game::Game, gd: GuardId) -> bool {
    match g.gd[gd].foe {
        Some(crate::game::Foe::Player) => {
            let p = crate::seats::faced_man(g, gd);
            crate::condition::down(g, g.pl[p].ch)
        }
        Some(crate::game::Foe::Guard(f)) => crate::condition::down(g, g.gd[f].ch),
        None => false,
    }
}

/// His foe is crouched: the player's crouch, or the AI man's crouch pose.
pub fn foe_crouched(g: &crate::game::Game, gd: GuardId) -> bool {
    match g.gd[gd].foe {
        Some(crate::game::Foe::Player) => {
            let p = crate::seats::faced_man(g, gd);
            g.pl[p].crouch
        }
        Some(crate::game::Foe::Guard(f)) => {
            let c = g.gd[f].ch;
            g.ch[c].anim.crouch > 0.5
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [Team; 5] = [Team::Guard, Team::Operative, Team::Hostile, Team::Player, Team::Civilian];

    /// Every cell, as the TypeScript's literal has it, with the moving pair as asked.
    fn cells_match(t: &TeamsMod, staff_on: bool) {
        for a in ALL {
            for b in ALL {
                let want = match (a.as_str(), b.as_str()) {
                    ("guard", "player") | ("player", "guard") => true,
                    ("operative", "hostile") | ("hostile", "operative") => true,
                    ("hostile", "player") | ("player", "hostile") => true,
                    ("guard", "operative") | ("operative", "guard") => staff_on,
                    _ => false,
                };
                assert_eq!(cell(t, a, b), want, "{} -> {}", a.as_str(), b.as_str());
            }
        }
    }

    #[test]
    fn the_table_starts_as_written() {
        cells_match(&TeamsMod::blank(), false);
    }

    #[test]
    fn the_switch_is_counted_writes_both_cells_and_stops_at_nought() {
        let mut t = TeamsMod::blank();
        count_switch(&mut t, true);
        count_switch(&mut t, true);
        assert_eq!(t.staff_ops, 2.0);
        cells_match(&t, true);
        count_switch(&mut t, false);
        assert_eq!(t.staff_ops, 1.0); // one still on
        cells_match(&t, true);
        count_switch(&mut t, false);
        assert_eq!(t.staff_ops, 0.0);
        cells_match(&t, false);
        count_switch(&mut t, false);
        assert_eq!(t.staff_ops.to_bits(), 0.0f64.to_bits()); // never below nought
        cells_match(&t, false);
    }
}
