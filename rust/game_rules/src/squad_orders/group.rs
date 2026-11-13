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

/// WHOSE MAN HE IS: the group he is of, or no group's.
pub fn of(g: &Game, gd: GuardId) -> Option<GroupId> {
    if g.gd[gd].duty.group.is_some() { return g.gd[gd].duty.group; }
    if g.team(gd).ours() { return Some(g.sets[crate::seats::seat_of(g, gd)].group); }
    g.qrf.group.filter(|_| g.qrf.men.contains(&gd))
}

/// He is one of its men.
pub fn has(g: &Game, id: GroupId, gd: GuardId) -> bool {
    match g.groups[id].commander {
        Commander::Seat(s) => g.team(gd).ours() && crate::seats::in_seat(g, s, gd),   // (a game of more than one: each seat has its own)
        Commander::Squad(_) => g.gd[gd].duty.group == Some(id),
        Commander::Response => g.qrf.men.contains(&gd),
    }
}

/// Whom a pass about a group must not hurt: the men of its side.
pub fn ours(g: &Game, gd: GuardId) -> bool {
    let (side, his) = (g.groups[g.group].side, g.team(gd));
    side.ours() == his.ours() && (his.ours() || his.theirs())
}

/// Whom it is up against.
pub fn foe(g: &Game, gd: GuardId) -> bool { g.groups[g.group].side.foe_of(g.team(gd)) }

/// The player who commands it, if one does and he is up.
pub fn player(g: &Game) -> Option<PlayerId> { Some(g.player).filter(|&p| is_seats(g) && !crate::condition::down(g, g.pl[p].ch)) }

/// Who leads it, for where its men are sent and what they come back to: the player who commands it, else the first of its men who is up.
pub fn lead(g: &Game, seat: &crate::seats::SeatRef) -> Option<CharId> {
    if seat.group.by_a_player() { return Some(g.pl[seat.player].ch); }
    g.guards.iter().find(|&&x| has(g, seat.group_id, x) && !crate::condition::down(g, g.gd[x].ch)).map(|&x| g.gd[x].ch)
}
/// … and where that one is
pub fn lead_pos(g: &Game, seat: &crate::seats::SeatRef) -> crate::V3 { lead(g, seat).map_or(g.seat_pos(seat), |c| g.ch[c].pos) }

/// It works as a squad: a steady man of it is up (`morale::training_of`). A player's always does.
pub fn drilled(g: &mut Game, id: GroupId) -> bool {
    if g.groups[id].by_a_player() { return true; }
    let men: alloc::vec::Vec<GuardId> = g.guards.iter().copied().filter(|&x| has(g, id, x) && !crate::condition::down(g, g.gd[x].ch) && !g.gd[x].fled).collect();
    men.into_iter().any(|x| crate::morale::training_of(g, x) == crate::morale::TrainingOf::Steady)
}

/// A FRAME OF THE GROUPS NO PLAYER COMMANDS: their door work, what their men have been told, and the fight as they do it. (The response team's drill runs its own.)
pub fn groups_tick(g: &mut Game, dt: f64) {
    for id in g.groups.ids().collect::<alloc::vec::Vec<_>>() {
        if !matches!(g.groups[id].commander, Commander::Squad(_)) || (g.groups[id].orders.asg.is_empty() && g.groups[id].tools.ops.is_empty()) { continue; }
        hurry(g, id);
        with(g, id, |g| {
            crate::doortools::door_tools_tick(g);
            crate::orders::squad_orders_tick(g);
        });
        for gd in g.groups[id].orders.asg.keys() {
            if !crate::condition::down(g, g.gd[gd].ch) && g.gd[gd].script.is_some() { crate::guards::behave::fight_on_the_way(g, gd, dt); }
        }
    }
}

/// Its men go to a mark at a run and onto it at the order's own pace.
fn hurry(g: &mut Game, id: GroupId) {
    for (gd, a) in g.groups[id].orders.asg.iter().map(|(gd, &a)| (gd, a)).collect::<alloc::vec::Vec<_>>() {
        let far = g.tab.task[a].post.is_some_and(|p| crate::v3::dist_xz(p, g.pos_of(gd)) > ONTO_M);
        g.tab.task[a].pace = far.then_some(g.k.tune.guard_chase);
    }
}
const ONTO_M: f64 = 2.5;

/// A round of somebody it is up against: a man's, or a player's if it is up against players.
pub fn their_shot(g: &Game, e: &crate::game::GameEvent) -> bool {
    use crate::director::EventKind as K;
    match e.kind {
        K::GuardShot => e.who.is_some_and(|who| !g.guards.iter().any(|&x| ours(g, x) && g.char_id(x) == who)),
        K::Shot => !g.groups[g.group].side.ours(),
        _ => false,
    }
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
