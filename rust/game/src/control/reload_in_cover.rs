//! CHANGE MAGAZINE BEHIND COVER: a defender in a fight whose gun runs dry does not stand in the open with his hands full. If he is already
//! out of the danger's sight he kneels; if a spot it cannot see is near enough to reach before the magazine is in, he goes there crouched;
//! else he kneels where he is. The reload goes on in his hands, and when it is done he holds that cover a moment before he closes again.
//!
//! THE WORKED EXAMPLE of a rule added to the list (control/): this file and its line in `rules!` are the whole of it.
//!
//! He decides ONCE, as the reload begins, and only where every line of `may` holds. Each of those lines is a question the game will one day
//! answer from a man's task (his post, his bounds) and from who has him; today they are read off his fields.
use super::{Intent, Outcome, TakeCover};
use crate::game::Game;
use crate::ids::*;
use crate::v3;
use crate::V3;

/// he goes only where he would be there with this share of the reload still to do
const IN_TIME: f64 = 0.7;
/// a walk is this much longer than the straight line, as a guess made before any path is planned
const WALK_K: f64 = 1.3;
/// at the spot
const AT_M: f64 = 0.6;
/// no nearer than this to a spot another man is going to
const APART_M: f64 = 1.2;
/// eyes on the man this lately, for the gun to have run dry "in the fight"
const FRESH_SECS: f64 = 1.0;

/// what he is doing about this reload
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum Plan {
    /// nothing decided (no reload, or this one not yet looked at)
    #[default]
    None,
    /// to this spot, crouched
    Go(V3),
    /// down where he is
    Kneel,
    /// as he always did: this rule leaves him alone for the rest of this reload
    Leave,
}

/// what the rule keeps of a man, for the length of one reload
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Row {
    pub plan: Plan,
    /// the game's clock when the rule last looked at him: a row older than a moment is of another reload
    pub t: f64,
}

/// May the rule take this man at all, as his reload begins?
fn may(g: &mut Game, gd: GuardId) -> bool {
    let now = g.time;
    let m = &g.gd[gd];
    // a defender: the player's own men keep their place by him and their orders
    if m.team == crate::teams::Team::Operative || m.civ.is_some() { return false; }
    // not from a post (his room's, his lane's), not out of a standoff, not out of the rooms his squad keeps him to
    if m.hold || m.pinned || m.standoff.is_some() || m.stays_in_rooms { return false; }
    // his gun ran dry with his eyes on a man who is still up
    if now - m.last_seen_t > FRESH_SECS { return false; }
    if crate::teams::foe_char(g, gd).is_none() || crate::teams::foe_down(g, gd) { return false; }
    // his hands are not between two guns
    !crate::gunslots::side_locked(g, gd)
}

/// A man who cannot act this tick (flashed, staggered, startled, still reacting) does not walk to cover either: the rule waits.
fn stopped(g: &Game, gd: GuardId) -> bool {
    let now = g.time;
    let m = &g.gd[gd];
    now < m.dazzled_until || now < m.stagger_until || m.react_t > 0.0 || g.m.startle.state_of.get(gd).map_or(false, |s| now < s.until)
}

/// Is another man already going to a spot beside `p`?
fn spoken_for(g: &Game, gd: GuardId, p: V3) -> bool {
    g.ctl.rows.reload_in_cover.iter().any(|(o, r)| o != gd && matches!(r.plan, Plan::Go(s) if v3::dist_xz(s, p) < APART_M))
}

fn decide(g: &mut Game, gd: GuardId) -> Plan {
    if !may(g, gd) { return Plan::Leave; }
    let Some(threat) = crate::belief::gun_point(g, gd) else { return Plan::Leave };
    let pos = g.ch[g.gd[gd].ch].pos;
    // out of its sight already: stay
    if crate::scene_collision::segment_blocked(g, [threat[0], 1.15, threat[2]], [pos[0], 0.9, pos[2]], None, None, None) { return Plan::Kneel; }
    let Some(spot) = crate::suppression::cover_from(g, gd, threat) else { return Plan::Kneel };
    if spoken_for(g, gd, spot) { return Plan::Kneel; }
    let pace = g.tune.guard_investigate * g.k.suppression.displace_pace;
    let walk_secs = v3::dist_xz(pos, spot) * WALK_K / pace.max(0.1);
    if walk_secs > g.gd[gd].reload_t * IN_TIME { return Plan::Kneel; }
    Plan::Go(spot)
}

pub fn transition(g: &mut Game, gd: GuardId, _dt: f64) {
    let now = g.time;
    let mut row = *g.ctl.rows.reload_in_cover.at(gd);
    if now - row.t > 0.5 { row.plan = Plan::None; }   // of another reload
    row.t = now;
    if g.gd[gd].reload_t < 0.0 {
        // the magazine is in: a man who went to cover for it holds that cover a moment, as one who shifted under fire does
        if matches!(row.plan, Plan::Go(_) | Plan::Kneel) { g.gd[gd].sup.cover_until = now + g.k.suppression.cover_hold; }
        row.plan = Plan::None;
    } else if row.plan == Plan::None {
        row.plan = decide(g, gd);
    } else if let Plan::Go(spot) = row.plan {
        let pos = g.ch[g.gd[gd].ch].pos;
        if v3::dist_xz(pos, spot) < AT_M { row.plan = Plan::Kneel; }
    }
    *g.ctl.rows.reload_in_cover.at(gd) = row;
}

pub fn propose(g: &Game, gd: GuardId) -> Option<Intent> {
    let row = g.ctl.rows.reload_in_cover.get(gd)?;
    if stopped(g, gd) { return None; }
    match row.plan {
        Plan::Go(spot) => Some(Intent::Cover(TakeCover { to: Some(spot), pace: g.tune.guard_investigate * g.k.suppression.displace_pace, crouch: g.k.suppression.displace_crouch, fire_wait: 0.0, threat: None })),
        Plan::Kneel => Some(Intent::Cover(TakeCover { to: None, pace: 0.0, crouch: 1.0, fire_wait: 0.0, threat: None })),
        Plan::None | Plan::Leave => None,
    }
}

pub fn commit(g: &mut Game, gd: GuardId, o: Outcome) {
    let pos = g.ch[g.gd[gd].ch].pos;
    let row = g.ctl.rows.reload_in_cover.at(gd);
    let Plan::Go(spot) = row.plan else { return };
    match o {
        // (a walk that could not be planned also says "arrived": he is there only if he is there)
        Outcome::Arrived | Outcome::GoingNowhere => row.plan = if v3::dist_xz(pos, spot) < AT_M { Plan::Kneel } else { Plan::Leave },
        _ => {}
    }
}

/// Something else took him (a grenade, his head went down): if he comes back still reloading he goes on with what he had decided.
pub fn lost(_: &mut Game, _: GuardId) {}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;

    fn stand() -> (Box<Game>, GuardId) {
        let mut g = Box::new(Game::blank());
        let c = g.ch.push(crate::character::Character::blank());
        let mut man = crate::game::Guard::blank();
        man.ch = c;
        man.team = crate::teams::Team::Hostile;
        man.reload_t = 2.0;
        let gd = g.gd.push(man);
        g.guards.push(gd);
        (g, gd)
    }
    fn plan(g: &mut Game, gd: GuardId) -> Plan { g.ctl.rows.reload_in_cover.at(gd).plan }

    #[test]
    fn a_posted_man_a_squad_mate_and_a_man_who_lost_sight_are_left_alone() {
        let (mut g, gd) = stand();
        g.time = 10.0;
        g.gd[gd].last_seen_t = 9.9;
        g.gd[gd].pinned = true;
        assert!(!may(&mut g, gd));
        g.gd[gd].pinned = false;
        g.gd[gd].stays_in_rooms = true;
        assert!(!may(&mut g, gd));
        g.gd[gd].stays_in_rooms = false;
        g.gd[gd].team = crate::teams::Team::Operative;
        assert!(!may(&mut g, gd));
        g.gd[gd].team = crate::teams::Team::Hostile;
        g.gd[gd].last_seen_t = 5.0;
        assert!(!may(&mut g, gd));
        transition(&mut g, gd, 0.016);
        assert_eq!(plan(&mut g, gd), Plan::Leave);
        assert!(propose(&g, gd).is_none());
    }

    #[test]
    fn he_goes_to_his_spot_kneels_when_he_is_at_it_and_holds_the_cover_when_the_magazine_is_in() {
        let (mut g, gd) = stand();
        g.time = 10.0;
        *g.ctl.rows.reload_in_cover.at(gd) = Row { plan: Plan::Go([3.0, 0.0, 0.0]), t: 10.0 };
        match propose(&g, gd) { Some(Intent::Cover(w)) => assert_eq!(w.to, Some([3.0, 0.0, 0.0])), _ => panic!("he should be going") }
        let c = g.gd[gd].ch;
        g.ch[c].pos = [2.8, 0.0, 0.1];
        g.time = 10.1;
        transition(&mut g, gd, 0.016);
        assert_eq!(plan(&mut g, gd), Plan::Kneel);
        match propose(&g, gd) { Some(Intent::Cover(w)) => assert_eq!((w.to, w.crouch), (None, 1.0)), _ => panic!("he should be down") }
        g.gd[gd].reload_t = -1.0;
        g.time = 10.2;
        transition(&mut g, gd, 0.016);
        assert_eq!(plan(&mut g, gd), Plan::None);
        assert!(g.gd[gd].sup.cover_until > 10.2);
        assert!(propose(&g, gd).is_none());
    }

    #[test]
    fn a_walk_that_says_arrived_far_from_the_spot_is_given_up_and_a_flashed_man_waits() {
        let (mut g, gd) = stand();
        g.time = 10.0;
        *g.ctl.rows.reload_in_cover.at(gd) = Row { plan: Plan::Go([3.0, 0.0, 0.0]), t: 10.0 };
        g.gd[gd].dazzled_until = 11.0;
        assert!(propose(&g, gd).is_none());
        g.gd[gd].dazzled_until = 0.0;
        commit(&mut g, gd, Outcome::Arrived);
        assert_eq!(plan(&mut g, gd), Plan::Leave);
    }

    #[test]
    fn a_row_left_from_another_reload_is_looked_at_afresh_and_two_men_do_not_share_a_spot() {
        let (mut g, gd) = stand();
        g.time = 20.0;
        g.gd[gd].reload_t = -1.0;
        *g.ctl.rows.reload_in_cover.at(gd) = Row { plan: Plan::Kneel, t: 10.0 };
        transition(&mut g, gd, 0.016);
        assert_eq!(plan(&mut g, gd), Plan::None);
        assert_eq!(g.gd[gd].sup.cover_until, crate::game::Guard::blank().sup.cover_until, "a stale row holds no cover");
        let other = g.gd.push(crate::game::Guard::blank());
        *g.ctl.rows.reload_in_cover.at(other) = Row { plan: Plan::Go([3.0, 0.0, 0.0]), t: 20.0 };
        assert!(spoken_for(&g, gd, [3.5, 0.0, 0.2]));
        assert!(!spoken_for(&g, other, [3.5, 0.0, 0.2]));
        assert!(!spoken_for(&g, gd, [9.0, 0.0, 0.0]));
    }
}
