//! WHAT A CALM MAN ON A NIGHT MAKES OF WHAT REACHES HIM: a far glimpse, a death nobody found.
mod stealth_ask;
use game::game::{DownBy, DownKind, GameSetBlackoutCause, GuardState};
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use stealth_ask::StealthAsk;

const SEEDS: [u32; 3] = [1, 2, 3];
fn seed(seed: u32) -> Opts { Opts { seed, ..Default::default() } }

/// A long room, dark but for a lamp over Sam, who stands `range` metres east of a sentry looking east along it.
fn long_room(range: f64, s: u32) -> (Scene, [f64; 2]) {
    let z = 13.25 + nudge(s, 0).round() * 0.5;
    let sam = [6.25 + range, z];
    let mut sc = Scene::new("long-room");
    sc.room("hall", 4.0, 10.0, 36.0, 16.0).door("west", Axis::Z, 4.0, 11.4);
    sc.bulb(sam[0], z).sentry("post", [6.25, z], [20.0, z]).sam(sam[0], z);
    sc.sentry("by the panel", [36.25, 24.25], [36.25, 26.0]);   // (when the mains go the nearest man is sent to the breaker: this one, in the yard)
    (sc, sam)
}

/// THE RULE: a far glimpse gives a calm man no place to walk to. Sam lit at 20 m: his awareness rises, he stays calm, and he has no fix. And when the
/// mains then go (which puts every calm man up to wary) he stands where he stood, with no fix. CONTROL: at 12 m the same faint glimpse writes the fix.
/// A FAILURE MEANS: the fault it was written for is back: "a lit man glimpsed from past the old 16 m, far too faintly to make a man wary, still wrote his
/// fix, and when the mains went he walked off his post to it".
#[test]
fn a_far_glimpse_gives_a_calm_man_no_place_to_walk_to() {
    for s in SEEDS {
        let (sc, _) = long_room(20.0, s);
        let mut p = Play::scene(&sc, &seed(s));
        let m = p.guard("post");
        let was = p.at(m);
        p.until(10.0, |p| p.awareness(m) >= 0.08).unwrap_or_else(|| panic!("seed {s}: 10 s lit at 20 m and he took nothing in: Sam's light {:.2}", p.sam_light()));
        assert!(p.state(m) == GuardState::Patrol && p.fix(m).is_none(), "seed {s}: at 20 m, awareness {:.2}: he is {:?} with a fix {:?}", p.awareness(m), p.state(m), p.fix(m));
        game::game::game_set_blackout(&mut p.g, 86400.0, GameSetBlackoutCause::Hand);
        if let Some(t) = p.always(5.0, |p| p.fix(m).is_none() && dist(p.at(m), was) < 0.5) { panic!("seed {s}: {t:.2} s after the mains went: fix {:?}, {:.2} m off his post", p.fix(m), dist(p.at(m), was)); }
        drop(p);

        // CONTROL: inside 16 m
        let (sc, _) = long_room(12.0, s);
        let mut p = Play::scene(&sc, &seed(s));
        let m = p.guard("post");
        p.until(10.0, |p| p.awareness(m) > 0.0).unwrap_or_else(|| panic!("seed {s}: lit at 12 m and he took nothing in"));
        assert!(p.awareness(m) < 0.3 && p.fix(m).is_some(), "seed {s}: at 12 m a faint glimpse writes his fix: awareness {:.2}, fix {:?}", p.awareness(m), p.fix(m));
    }
}

/// THE RULE: a far glimpse that is what has him looking does give him the place. Sam left standing lit at 20 m: the first fix lands as he turns wary, not
/// before, and it is on Sam.
/// A FAILURE MEANS: a far man is blind for good, or the fix comes from the first faint glimpse.
#[test]
fn a_far_glimpse_that_has_him_looking_gives_him_the_place() {
    for s in SEEDS {
        let (sc, sam) = long_room(20.0, s);
        let mut p = Play::scene(&sc, &seed(s));
        let m = p.guard("post");
        p.until(30.0, |p| p.fix(m).is_some()).unwrap_or_else(|| panic!("seed {s}: 30 s lit at 20 m and no fix: awareness {:.2}, Sam's light {:.2}", p.awareness(m), p.sam_light()));
        assert!(p.awareness(m) >= 0.29, "seed {s}: his first fix landed at awareness {:.2}, under the 'huh?' line", p.awareness(m));
        assert!(dist(p.fix(m).unwrap(), sam) < 0.5, "seed {s}: his fix {:?} is not Sam at {sam:?}", p.fix(m));
    }
}

/// Three rooms in a row, each man shut in his own, Sam out on the lot.
fn three_rooms(s: u32) -> Scene {
    let mut sc = Scene::new("three-rooms");
    sc.room("a", 6.0, 10.0, 14.0, 16.0).room("b", 14.0, 10.0, 22.0, 16.0).room("c", 22.0, 10.0, 30.0, 16.0);
    sc.door("a-b", Axis::Z, 14.0, 12.4).door("b-c", Axis::Z, 22.0, 12.4).door("yard", Axis::X, 16.0, 8.0);
    sc.sentry("a", [8.25 + nudge(s, 0).round() * 0.5, 13.25], [7.0, 13.25]).sentry("b", [18.25, 13.25], [16.0, 13.25]).sentry("c", [26.25, 13.25 + nudge(s, 1).round() * 0.5], [28.0, 13.25]);
    sc.sam(36.0, 26.0);
    sc
}

/// THE RULE: a death nobody found becomes known through the radio check, and not a moment before. Up to the check nothing whatever comes of it; on the
/// check his name is asked for, and a man is sent to walk his route. CONTROL: the same check with nobody down sends nobody.
/// A FAILURE MEANS: the floor knows of a death it has no way to know of, or a silent man is no longer missed.
#[test]
fn a_death_nobody_found_becomes_known_through_the_radio_check_and_not_before() {
    for s in SEEDS {
        for dead in [true, false] {
            let mut p = Play::scene(&three_rooms(s), &seed(s));
            let (a, b, c) = (p.guard("a"), p.guard("b"), p.guard("c"));
            p.run(1.0);
            if dead { p.falls_as(c, DownKind::Struck, true, DownBy::Player); }
            p.radio_check_due(5.0);
            let calm = |p: &Play| [a, b].iter().all(|&x| p.state(x) == GuardState::Patrol) && p.alarm() == 0.0 && !p.body_found(c) && p.checker_of(c).is_none();
            if let Some(t) = p.always(4.9, |p| calm(p)) { panic!("seed {s}: something moved {t:.2} s after the death, before the check: {}", p.roll()); }
            assert!(p.said.iter().all(|l| !l.text.contains("sound off")), "seed {s}: the check came before it was due");
            let sent = p.until(15.0, |p| p.checker_of(c).is_some());
            let lines: Vec<&str> = p.said.iter().map(|l| l.text.as_str()).collect();
            assert!(lines.iter().any(|l| l.contains("sound off")), "seed {s}: no radio check, dead {dead}: {lines:?}");
            let name = format!("{}?", p.g.gd[c].callsign);
            assert_eq!(lines.iter().any(|l| l.contains(&name)), dead, "seed {s}, dead {dead}: {name} asked for: {lines:?}");
            assert_eq!(sent.is_some(), dead, "seed {s}, dead {dead}: a man sent to walk his route: {lines:?}");
        }
    }
}
