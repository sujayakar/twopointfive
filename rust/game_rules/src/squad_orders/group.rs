//! THE GROUPS: men commanded together, whichever side they are on. A pass over the squad's code (its orders, its door tools, what it covers unasked) is about one of them, `g.group`.
pub use ::game_types::group::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::JsSet;
use crate::teams::Team;
use alloc::string::String;

/// A group of these stands from now: nobody of it has been told anything, its menu and its tools made as a player's are.
pub fn new(g: &mut Game, side: Team, commander: Commander) -> GroupId {
    let id = g.groups.push(Group::new(side, commander));
    g.groups[id].ring.add(String::from(EVERY_KEY));   // (a player's men try his own keys: `squad_orders_ring`)
    with(g, id, |g| {
        g.groups[id].orders = crate::orders::squad_orders_new(g);
        g.groups[id].tools = crate::doortools::door_tools_new(g);
    });
    id
}

/// A PASS ABOUT ANOTHER GROUP: the one it was about comes back after. Every other read and every write is the one Game's.
pub fn with<R>(g: &mut Game, id: GroupId, f: impl FnOnce(&mut Game) -> R) -> R {
    let was = core::mem::replace(&mut g.group, id);
    let r = f(g);
    g.group = was;
    r
}

/// The pass is about a player's own squad.
pub fn is_seats(g: &Game) -> bool { g.groups[g.group].by_a_player() }

/// on a ring: it opens every lock
pub const EVERY_KEY: &str = "*";

/// A ring of keys as a door asks it: it opens what it names, and everything if every key is on it.
pub fn ring_has(ring: &JsSet<String>, key: &str) -> bool { ring.holds(key) || ring.holds(EVERY_KEY) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_key_opens_anything_and_a_ring_only_what_it_names() {
        let mut ring = JsSet::new();
        assert!(!ring_has(&ring, "vault"));
        ring.add(String::from("boiler-room"));
        assert!(ring_has(&ring, "boiler-room") && !ring_has(&ring, "vault"));
        ring.add(String::from(EVERY_KEY));
        assert!(ring_has(&ring, "vault"));
    }

    #[test]
    fn a_pass_about_a_group_reads_and_writes_its_own_book_and_the_one_game() {
        let mut g = Game::blank();
        let seats = g.group;
        g.groups[seats].tools.charges = 1.0;
        let other = g.groups.push(Group::new(Team::Guard, Commander::Response));
        g.groups[other].tools.charges = 7.0;
        let got = with(&mut g, other, |g| {
            assert!(!is_seats(g) && crate::seats::here(g).door_tools.charges == 7.0);
            g.groups[g.group].tools.charges = 9.0;
            g.time = 5.0;
            42
        });
        assert_eq!((got, g.group, g.time), (42, seats, 5.0));
        assert!(is_seats(&g));
        assert_eq!((g.groups[seats].tools.charges, g.groups[other].tools.charges), (1.0, 9.0));
    }

    #[test]
    fn a_pass_about_the_seat_inside_it_leaves_the_group_as_it_was() {
        let mut g = Game::blank();
        let other = g.groups.push(Group::new(Team::Guard, Commander::Response));
        with(&mut g, other, |g| {
            let seat = g.seat;
            crate::seats::with_seat(g, seat, &mut |g| assert_eq!(g.group, other));
            assert_eq!(g.group, other);
        });
    }
}
