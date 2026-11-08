//! THE FRAG: in the open he dies, behind a wall that holds he does not, and a wall the burst will open is no cover.
mod kit_ask;
use game::frag::BlastResult;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use kit_ask::KitAsk;

/// A long room and a room behind its east wall: a man in the open west of the grenade inside its lethal reach, one the same distance off behind the wall, and one far down the room; Sam far off.
fn the_three(s: u32) -> (Play, [f64; 2], [game::ids::GuardId; 3]) {
    let mut sc = Scene::new("three-men");
    let z = 13.0 + 0.5 * nudge(s, 0);
    sc.room("west", 2.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0).door("between", Axis::Z, 18.0, 10.4).door("yard", Axis::X, 16.0, 21.4);
    sc.bulb(10.0, 13.0).bulb(22.0, 13.0);
    sc.sentry("open", [12.6, z], [2.0, z]).sentry("walled", [18.6, z], [26.0, z]).sentry("far", [3.5, z], [2.0, z]).sam(25.0, 15.0);
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.sam_a_ghost();
    p.no_wits();
    p.run(0.1);
    let men = [p.guard("open"), p.guard("walled"), p.guard("far")];
    (p, [15.0, z], men)
}

/// THE RULE: in the open inside the lethal reach he dies; the same distance off behind a wall that holds he does not, and is not folded; past the wound reach in the open he is untouched.
/// A FAILURE MEANS: a grenade reads the wall wrong or no longer kills, which is what its whole place in the room fight rests on.
#[test]
fn in_the_open_he_dies_and_behind_a_wall_that_holds_he_does_not() {
    for s in [1, 2, 3] {
        let (mut p, at, [open, walled, far]) = the_three(s);
        let (lethal, wound, _, fuse) = p.frag_reach();
        assert!(dist(p.at(open), at) < lethal && dist(p.at(walled), at) < wound && dist(p.at(far), at) > wound, "seed {s}: the staging: the three do not stand as the test says");
        assert_eq!(p.in_a_burst(at, p.at(walled)).1, 0.0, "seed {s}: the staging: the wall opens or does not cover him");
        p.frag_lands(at);
        p.run(fuse + 0.5);
        assert!(!p.up(open), "seed {s}: in the open {:.1} m from it and he stands", dist(p.at(open), at));
        assert!(p.up(walled) && p.folded(walled) == 0.0, "seed {s}: behind a wall that holds: up {}, folded {:.1} s", p.up(walled), p.folded(walled));
        assert!(p.up(far) && p.folded(far) == 0.0, "seed {s}: past the wound reach: up {}, folded {:.1} s", p.up(far), p.folded(far));
    }
}

/// THE RULE: a wall the burst will open is no cover: a grenade a hand off a partition, a man a stride behind it: hidden as the wall stands, and in its line as the burst will leave it, and the burst does leave him so. The control: a grenade whose reach opens nothing is reckoned as the walls stand.
/// A FAILURE MEANS: men crouch behind a partition with a grenade on the other side of it and are killed through the hole.
#[test]
fn a_wall_the_burst_will_open_is_no_cover() {
    for s in [1, 2, 3] {
        let (mut p, _, [_, walled, _]) = the_three(s);
        let z = p.at(walled)[1];
        let (near, far, man) = ([17.6, z], [12.0, z], [19.0, z]);
        p.man_stands(walled, man);
        p.run(0.05);
        let (plain, reckoned) = p.in_a_burst(near, man);
        assert_eq!(plain, 0.0, "seed {s}: the staging: the wall as it stands hides him ({plain})");
        assert!(reckoned > 0.0, "seed {s}: the wall this burst will open is reckoned no cover: {reckoned}");
        let (fp, fr) = p.in_a_burst(far, man);
        assert_eq!(fp, fr, "seed {s}: the control: a burst too far off to open it: as it stands {fp}, reckoned {fr}");
        p.frag_lands(near);
        p.run(p.frag_reach().3 + 0.5);
        assert!(p.in_a_burst(near, man).0 > 0.0, "seed {s}: the burst did not leave him in its line");
    }
}

/// THE RULE: "a teammate never frags his own": a frag and clear on a door goes off in the room, and catches nobody of ours.
/// A FAILURE MEANS: the squad's own grenade folds or kills the squad.
#[test]
fn a_teammate_never_frags_his_own() {
    for s in [3, 7, 11] {
        let mut sc = Scene::new("yard");
        sc.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
        sc.door("west-yard", Axis::X, 16.0, 13.4 + 0.5 * nudge(s, 0)).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
        sc.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
        sc.team([14.0, 23.5], std::f64::consts::PI, &[[13.0, 21.5], [15.0, 21.5], [13.0, 22.5], [15.0, 22.5]]);
        sc.defender("west", [13.0, 13.5], "west-yard").defender("east", [24.25, 12.25], "east-yard");
        let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
        p.sam_unhurt();
        p.run(0.2);
        let men = p.mates();
        p.order("fragClear", sc.door_centre("west-yard"), game::orders::Who::All);
        p.until(45.0, |p| p.bursts().0 > 0).unwrap_or_else(|| panic!("seed {s}: 45 s and no burst: {}", p.roll()));
        p.run(0.2);
        let hits = p.burst_hits();
        assert_eq!(p.bursts().1, 0, "seed {s}: the grenade caught {} of ours: {hits:?}", p.bursts().1);
        let ours = hits.iter().filter(|(t, r)| matches!(t, game::teams::Team::Operative | game::teams::Team::Player) && *r != BlastResult::Shielded).count();
        assert_eq!(ours, 0, "seed {s}: {ours} of ours in the report: {hits:?}");
        assert!(men.iter().all(|&m| p.up(m) && p.folded(m) == 0.0), "seed {s}: a man of ours is down or folded: {}", p.roll());
        let at = p.burst_at().unwrap();
        assert!(at[1] < 16.0, "seed {s}: the control: the burst at {at:?} is not in the room");
    }
}
