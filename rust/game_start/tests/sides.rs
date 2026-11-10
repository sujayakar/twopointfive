//! WHO IS WHOSE ENEMY, AS THE EYES AND THE GUNS HAVE IT.
mod stealth_ask;
use game::game::GuardState;
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
