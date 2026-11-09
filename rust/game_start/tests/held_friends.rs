//! ROUNDS AND FRIENDS: a teammate does not press with a friend where his round could go.
mod kit_ask;
use game_start::play::{Opts, Play};
use game_start::scene::{Axis, Scene};
use kit_ask::KitAsk;

/// The yard with nobody's wits on: two of the squad and a defender stood on a line along z = 22, Sam away.
fn a_line(s: u32) -> (Play, game::ids::GuardId, game::ids::GuardId, game::ids::GuardId) {
    let mut sc = Scene::new("yard");
    sc.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    sc.door("west-yard", Axis::X, 16.0, 13.4).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    sc.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    sc.team([3.0, 27.0], std::f64::consts::PI, &[[6.0, 26.0], [8.0, 26.0], [6.0, 27.0], [8.0, 27.0]]);
    sc.defender("west", [12.25, 12.25], "west-yard").defender("east", [24.25, 12.25], "east-yard");
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.sam_a_ghost();
    p.no_wits();
    p.run(0.2);
    let (mates, foe) = (p.mates(), p.hostiles_by([12.25, 12.25])[0]);
    p.pin_man(mates[0], [26.0, 22.0], -std::f64::consts::FRAC_PI_2, game::game::GuardState::Patrol, 0.0);
    p.man_stands(foe, [16.0, 22.0]);
    p.man_stands(mates[1], [6.0, 26.0]);
    p.run(0.5);
    (p, mates[0], mates[1], foe)
}

/// THE RULE: a teammate with a friend between his muzzle and his man does not press: Sam there or a mate there, the round is held and the man stands. Nobody between (the control): it is pressed and finds the man.
/// A FAILURE MEANS: the squad shoots through its own men, as a matter of course.
#[test]
fn a_teammate_does_not_press_with_a_friend_in_the_way() {
    for s in [1, 2, 3] {
        let (mut p, shooter, friend, foe) = a_line(s);
        p.sam_put(p.on_the_line(shooter, foe, 5.0));
        p.run(0.2);
        let held = p.rounds_held_back();
        p.man_fires_at(shooter, p.at(foe));
        assert_eq!(p.rounds_held_back(), held + 1, "seed {s}: Sam in the way and the round was pressed");
        p.run(0.2);
        assert!(p.up(foe), "seed {s}: the round went through Sam and dropped him");
        p.sam_put([3.0, 27.0]);
        p.man_stands(friend, p.on_the_line(shooter, foe, 5.0));
        p.run(0.2);
        p.man_fires_at(shooter, p.at(foe));
        assert_eq!(p.rounds_held_back(), held + 2, "seed {s}: a mate in the way and the round was pressed");
        p.run(0.2);
        assert!(p.up(friend), "seed {s}: the mate in the way is down");
        p.man_stands(friend, [6.0, 26.0]);
        p.run(0.2);
        let held = p.rounds_held_back();
        for _ in 0..8 { p.man_fires_at(shooter, p.at(foe)); p.run(0.1); }
        assert_eq!(p.rounds_held_back(), held, "seed {s}: the control: nobody in the way and a round was held");
        assert!(!p.up(foe), "seed {s}: the control: eight rounds at a man who stands and none found him");
    }
}
