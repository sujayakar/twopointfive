//! WHO IS WHOSE ENEMY, AS THE EYES AND THE GUNS HAVE IT.
mod stealth_ask;
use game::game::GuardState;
use game::ids::GuardId;
use game::teams::Team;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use stealth_ask::StealthAsk;

const SEEDS: [u32; 3] = [1, 2, 3];
fn seed(seed: u32) -> Opts { Opts { seed, ..Default::default() } }
const EAST: f64 = std::f64::consts::FRAC_PI_2;

/// A lit room with Sam in the middle of it, the man to be stood two metres west of him.
fn lit_room(s: u32) -> (Scene, [f64; 2], [f64; 2]) {
    let z = 13.25 + nudge(s, 0).round() * 0.5;
    let (sam, man) = ([21.25, z], [19.25, z]);
    let mut sc = Scene::new("lit-room");
    sc.room("room", 12.0, 10.0, 28.0, 16.0).door("south", Axis::X, 16.0, 16.0).bulb(20.0, 13.0).sam(sam[0], sam[1]);
    (sc, sam, man)
}

/// THE RULE: a lit Sam in front of an alert teammate is never his foe and never draws a round. CONTROL: a hostile stood in the same place has him inside a second and a
/// half, and fires.
/// A FAILURE MEANS: the sides' table no longer keeps a man from his own side, or it keeps him from the other.
#[test]
fn a_lit_sam_in_front_of_an_alert_teammate_is_never_his_foe() {
    for s in SEEDS {
        let (sc, _, man) = lit_room(s);
        let mut p = Play::scene(&sc, &seed(s));
        p.sam_unhurt();
        let op = p.man_of(Team::Ours, man, EAST);
        p.pin_man(op, man, EAST, GuardState::Alert, 1.0);
        p.run(0.2);
        assert!(p.sam_light() > 0.3 || p.until(1.0, |p| p.sam_light() > 0.3).is_some(), "seed {s}: staging: Sam is lit ({:.2})", p.sam_light());
        let fired = p.rounds_fired(op);
        if let Some(t) = p.always(5.0, |p| !p.has_foe(op) && !p.saw_his_man(op) && p.rounds_fired(op) == fired) { panic!("seed {s}: at {t:.2} s he has a foe {}, saw him {}, rounds {}", p.has_foe(op), p.saw_his_man(op), p.rounds_fired(op) - fired); }
        drop(p);

        let (sc, _, man) = lit_room(s);
        let mut p = Play::scene(&sc, &seed(s));
        p.sam_unhurt();
        let ho = p.man_of(Team::Hostile, man, EAST);
        let fired = p.rounds_fired(ho);
        p.until(1.5, |p| p.state(ho) == GuardState::Alert && p.has_foe(ho)).unwrap_or_else(|| panic!("seed {s}: a hostile two metres from a lit Sam, for a second and a half: {:?}", p.state(ho)));
        p.until(5.0, |p| p.rounds_fired(ho) > fired).unwrap_or_else(|| panic!("seed {s}: he never fired"));
    }
}

/// THE RULE: a man behind a wall is never fought until he fires, and then he is placed by ear: an alert teammate has no foe and no fix on a hostile round a wall, and
/// after the hostile's one round he has a fix a few metres off him and still no foe. CONTROL: no round, no fix.
/// A FAILURE MEANS: the eyes see through walls, or a report no longer places a man.
#[test]
fn a_man_behind_a_wall_is_never_fought_until_he_fires_and_is_placed_by_ear() {
    for s in SEEDS {
        for fires in [true, false] {
            let mut sc = Scene::new("behind-a-wall");
            sc.room("north", 12.0, 6.0, 28.0, 12.0).room("south", 12.0, 12.0, 28.0, 18.0).door("through", Axis::X, 12.0, 25.0).door("out", Axis::Z, 12.0, 15.0);
            sc.bulb(18.0, 9.0).bulb(18.0, 15.0).sam(38.0, 26.0);
            let mut p = Play::scene(&sc, &seed(s));
            let at = [18.25, 9.25 + 0.5 * nudge(s, 1).round()];
            let (op, ho) = (p.man_of(Team::Ours, [18.25, 15.25], -EAST), p.man_of(Team::Hostile, at, EAST));
            p.pin_man(op, [18.25, 15.25], -EAST, GuardState::Alert, 1.0);
            p.pin_man(ho, at, EAST, GuardState::Patrol, 0.0);
            if let Some(t) = p.always(3.0, |p| !p.has_foe(op) && p.fix(op).is_none()) { panic!("seed {s}: through the wall at {t:.2} s: foe {}, fix {:?}", p.has_foe(op), p.fix(op)); }
            if fires { p.man_fires_at(ho, [at[0] + 3.0, at[1]]); }
            p.run(0.5);
            assert!(!p.has_foe(op), "seed {s}: he has seen nothing, and has a foe");
            match p.fix(op) {
                Some(fix) if fires => assert!(dist(fix, at) < 9.0, "seed {s}: his fix {fix:?} is not a few metres off the man at {at:?}"),
                None if !fires => {}
                f => panic!("seed {s}: fired {fires}: his fix {f:?}, shots heard {}", p.shots_heard(op)),
            }
        }
    }
}

/// A dark room, a teammate of Sam's on a hold in it, and 8 m behind his back a man of a side, held where he was put.
fn his_back_to_him(team: Team, s: u32) -> (Play, GuardId, GuardId, [f64; 2], [f64; 2]) {
    let z = 11.25 + nudge(s, 0).round() * 0.5;
    let (at, man) = ([20.25, z], [28.25, z]);
    let mut sc = Scene::new("his-back-to-him");
    sc.room("room", 10.0, 9.0, 30.0, 18.0).door("south", Axis::X, 18.0, 28.0).wall_z(24.0, 13.0, 18.0);
    sc.room("far", 30.0, 9.0, 36.0, 13.5).room("farther", 30.0, 13.5, 36.0, 18.0).door("far-out", Axis::X, 9.0, 33.0).door("farther-out", Axis::X, 18.0, 33.0).door("between", Axis::X, 13.5, 33.0);
    sc.defender("far", [34.25, 12.25], "far-out").defender("farther", [34.25, 14.75], "farther-out").defender("room", [28.75, 10.25], "south");
    sc.team([6.0, 24.0], -EAST, &[at, [6.0, 25.0]]);
    let mut p = Play::scene(&sc, &seed(s));
    p.sam_unseen();
    let op = p.mates()[0];
    let x = p.hostiles_by([28.75, 10.25])[0];
    for other in p.mates().into_iter().skip(1) { p.falls(other, game::game::DownBy::Player); }
    p.order("hold", at, game::orders::Who::All);
    let c = p.g.gd[x].ch;
    p.g.ch[c].team = team;
    for _ in 0..400 { p.pin_man(x, [28.25, 16.75], -EAST, GuardState::Patrol, 0.0); p.frame(); }
    assert!(p.g.fight.ops.iter().all(|&o| p.g.tab.roomfight_op[o].turn.is_none()), "seed {s}: the staging is wrong: something has turned him already");
    (p, op, x, at, man)
}

/// He fires past the teammate three times a second, for four seconds or until `done`: did it come.
fn rounds_past_him(p: &mut Play, x: GuardId, at: [f64; 2], man: [f64; 2], mut done: impl FnMut(&Play) -> bool) -> bool {
    let mut f = 0;
    p.until(4.0, |p| { p.pin_man(x, man, -EAST, GuardState::Patrol, 0.0); if f % 20 == 10 { p.man_fires_at(x, [at[0] - 2.0, at[1] + 1.2]); } f += 1; done(p) }).is_some()
}

/// THE RULE: a teammate on a hold with his back to a man of any side against him turns to the rounds that go past him.
/// A FAILURE MEANS: a partner on a quiet mission is deaf to the staff again (only a fight's defenders turned him).
#[test]
fn a_teammate_turns_to_the_rounds_of_a_man_of_the_place() {
    for s in SEEDS {
        for team in [Team::Guard, Team::Hostile] {
            let (mut p, op, x, at, man) = his_back_to_him(team, s);
            assert!(p.gun_off(op, man) > 90.0, "seed {s} {team:?}: the staging is wrong: his gun is {:.0} degrees off", p.gun_off(op, man));
            assert!(rounds_past_him(&mut p, x, at, man, |p| p.gun_off(op, man) < 30.0), "seed {s} {team:?}: rounds past him for four seconds and his gun is {:.0} degrees off the man", p.gun_off(op, man));
        }
    }
}
