//! SAM'S LOW WHISTLE: who hears it along the sound's path, who answers, and what a man who is already hunting makes of it.
mod stealth_ask;
use game::game::GuardState;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use stealth_ask::StealthAsk;

const SEEDS: [u32; 3] = [1, 2, 3];
fn seed(seed: u32) -> Opts { Opts { seed, ..Default::default() } }
const SAM: [f64; 2] = [7.0, 12.0];

/// A long dark room, Sam at its west end, one man per range east of him with his back turned, looking on east.
fn corridor(ranges: &[f64], s: u32) -> Scene {
    let mut sc = Scene::new("whistle-corridor");
    sc.room("hall", 4.0, 10.0, 34.0, 14.0).door("west", Axis::Z, 4.0, 11.4).bulb_off(19.0, 12.0);
    sc.sam(SAM[0], SAM[1]);
    for (i, r) in ranges.iter().enumerate() {
        let at = [SAM[0] + r + 0.3 * nudge(s, i as u32), SAM[1] + 0.4 * (i as f64)];
        sc.sentry(&format!("m{i}"), at, [33.0, at[1]]);
    }
    sc
}

/// THE RULE: a calm man with his back turned answers a whistle at 6 m (wary, his fix on the place it came from, a line not on the net), and at 11 m
/// nothing at all happens to him.
/// A FAILURE MEANS: the whistle's reach has moved, or a man answers past it, or he calls it in on the radio.
#[test]
fn a_calm_man_answers_a_whistle_at_six_metres_and_not_at_eleven() {
    for s in SEEDS {
        let mut p = Play::scene(&corridor(&[6.0], s), &seed(s));
        let m = p.guard("m0");
        p.run(0.2);
        let sam = p.sam_at();
        p.sam_whistles();
        p.run(0.1);
        assert_eq!(p.state(m), GuardState::Suspicious, "seed {s}: at 6 m he answers it");
        let fix = p.fix(m).unwrap_or_else(|| panic!("seed {s}: he answered with no place to go to"));
        assert!(dist(fix, sam) < 0.5, "seed {s}: his fix {fix:?} is not where Sam whistled {sam:?}");
        assert_eq!(p.lines(m).len(), 1, "seed {s}: he says one line: {:?}", p.lines(m));
        assert!(!p.spoke_on_radio(m), "seed {s}: nothing on the net: {:?}", p.lines(m));
        drop(p);

        // CONTROL: the same man 11 m off
        let mut p = Play::scene(&corridor(&[11.0], s), &seed(s));
        let m = p.guard("m0");
        p.run(0.2);
        p.sam_whistles();
        if let Some(t) = p.always(2.0, |p| p.state(m) == GuardState::Patrol && p.awareness(m) == 0.0 && p.fix(m).is_none()) { panic!("seed {s}: at 11 m something reached him {t:.2} s on: {:?}, awareness {:.2}", p.state(m), p.awareness(m)); }
        assert!(p.lines(m).is_empty(), "seed {s}: at 11 m he says nothing: {:?}", p.lines(m));
    }
}

/// THE RULE: a shut door between Sam and a man 4 m off eats the whistle; the same door standing open carries it.
/// A FAILURE MEANS: a leaf no longer muffles the sound's path, or an open doorway no longer lets it through.
#[test]
fn a_shut_door_eats_a_whistle_and_an_open_one_does_not() {
    for s in SEEDS {
        for open in [false, true] {
            let mut sc = Scene::new("whistle-door");
            sc.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0).door("between", Axis::Z, 18.0, 12.4);
            sc.sam(16.0, 13.0 + 0.1 * nudge(s, 0)).sentry("post", [20.0 + 0.2 * nudge(s, 1), 13.0], [25.0, 13.0]);
            let mut p = Play::scene(&sc, &seed(s));
            let m = p.guard("post");
            if open { p.door_stands_open("between"); }
            p.run(0.3);
            p.sam_whistles();
            p.run(0.2);
            assert_eq!(p.state(m) == GuardState::Suspicious, open, "seed {s}: the door {}: he is {:?}, awareness {:.2}", if open { "open" } else { "shut" }, p.state(m), p.awareness(m));
            assert_eq!(p.fix(m).is_some(), open, "seed {s}: the door {}: his fix is {:?}", if open { "open" } else { "shut" }, p.fix(m));
        }
    }
}

/// THE RULE: of two calm men in reach only the nearer answers; the other stays calm with no fix. The far man alone, at the same range, does answer.
/// A FAILURE MEANS: a whistle draws everybody in earshot, so it no longer peels off one man.
#[test]
fn of_two_men_in_reach_the_nearer_comes() {
    for s in SEEDS {
        let mut p = Play::scene(&corridor(&[4.0, 7.0], s), &seed(s));
        let (near, far) = (p.guard("m0"), p.guard("m1"));
        p.run(0.2);
        p.sam_whistles();
        p.run(0.2);
        assert_eq!(p.state(near), GuardState::Suspicious, "seed {s}: the nearer man answers");
        assert_eq!(p.state(far), GuardState::Patrol, "seed {s}: the farther man does not");
        assert!(p.fix(far).is_none() && p.awareness(far) == 0.0, "seed {s}: the farther man is untouched: fix {:?}, awareness {:.2}", p.fix(far), p.awareness(far));
        assert!(p.lines(far).is_empty(), "seed {s}: the farther man says nothing: {:?}", p.lines(far));
        drop(p);

        // CONTROL: the farther man with nobody nearer
        let mut p = Play::scene(&corridor(&[7.0], s), &seed(s));
        let far = p.guard("m0");
        p.run(0.2);
        p.sam_whistles();
        p.run(0.2);
        assert_eq!(p.state(far), GuardState::Suspicious, "seed {s}: alone at 7 m he answers");
    }
}

/// THE RULE: a man already hunting takes a whistle as a fix on Sam: he stays alert, his fix jumps to where it came from, and he says no line of a
/// calm man's. With no whistle his fix stays on the old shot.
/// A FAILURE MEANS: a hunter ignores the whistle, or is talked down by it as a calm man is.
#[test]
fn a_man_already_hunting_takes_a_whistle_as_a_fix_on_you() {
    for s in SEEDS {
        for whistle in [true, false] {
            let mut sc = corridor(&[], s);
            sc.sentry("hunter", [20.0 + 0.3 * nudge(s, 2), 12.0], [33.0, 12.0]);
            let mut p = Play::scene(&sc, &seed(s));
            let m = p.guard("hunter");
            p.run(0.2);
            p.sam_fires_at([4.2, 12.0], 0.6);                                        // (one round into the west wall, 13 m from him)
            p.until(1.5, |p| p.state(m) == GuardState::Alert).unwrap_or_else(|| panic!("seed {s}: the shot does not rouse him: {:?}", p.state(m)));
            let old = p.fix(m).unwrap_or_else(|| panic!("seed {s}: alert with no fix"));
            let here = [p.at(m)[0] + 5.0, 12.0];                                     // (Sam is now in the dark behind him)
            p.sam_put(here);
            p.run(1.5);
            let lines = p.lines(m).len();
            if whistle { p.sam_whistles(); }
            p.run(0.2);
            let fix = p.fix(m).unwrap_or_else(|| panic!("seed {s}: he lost his fix"));
            assert_eq!(p.state(m), GuardState::Alert, "seed {s}: whistle {whistle}: he stays alert");
            if whistle {
                assert!(dist(fix, here) < 0.6, "seed {s}: his fix {fix:?} did not go to the whistle at {here:?}");
                assert_eq!(p.lines(m).len(), lines, "seed {s}: a hunter has nothing to say to it: {:?}", p.lines(m));
            } else {
                assert!(dist(fix, here) > 4.0 && dist(fix, old) < 6.0, "seed {s}: with no whistle his fix {fix:?} went to Sam at {here:?} from {old:?}");
            }
        }
    }
}
