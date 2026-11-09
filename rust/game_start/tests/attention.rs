//! A GUARD HAS ONE ATTENTION: a small thing takes it for a time, his eye narrows to it, and the thread the floor shows runs the way he will walk.
mod world_ask;
use game::attention::{DrawKind, HoldMode};
use game::game::GuardState;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use world_ask::{a_box, scene_with, WorldAsk};

const SEEDS: [u32; 3] = [1, 2, 3];
fn seed(seed: u32) -> Opts { Opts { seed, ..Default::default() } }

/// A long lit hall with a man walking its length; Sam out in the dark lot, where nobody has a line to him.
fn hall(s: u32) -> Scene {
    let z = 13.0 + nudge(s, 0);
    let mut sc = Scene::new("attention-hall");
    sc.room("hall", 4.0, 9.0, 34.0, 17.0).door("west", Axis::Z, 4.0, 11.4);
    sc.bulb(12.0, 13.0).bulb(20.0, 13.0).bulb(28.0, 13.0);
    sc.patrol("walk", &[[8.25, z], [30.25, z]], 3.0);
    sc
}

/// THE RULE: one clack and the calm man's attention is on the place he put it, whole while he walks; at the thing it runs exactly the seconds a noise is
/// worth; he says he is done once, the telling's seconds before the end, and goes back on his route. A knock as faint that nobody made to be looked at
/// (the control) takes nobody.
/// A FAILURE MEANS: a lure no longer draws a man, holds him for ever, or he tells it twice; or every faint knock on the floor has become a lure.
#[test]
fn one_clack_takes_a_calm_man_for_a_time_and_he_says_he_is_done_once() {
    for s in SEEDS {
        let mut p = Play::scene(&hall(s), &seed(s));
        let m = p.guard("walk");
        p.run(1.0);
        let at = p.ahead(m, 5.0 + nudge(s, 1), 0.0);
        p.clack_at(at);
        p.run(0.1);
        let h = p.held(m).unwrap_or_else(|| panic!("seed {s}: a clack {:.1} m ahead of him took nothing: {}", dist(p.at(m), at), p.why(m)));
        assert!(h.mode == HoldMode::Go && h.kind == DrawKind::Noise && dist(h.at, at) < 0.6 && p.state(m) == GuardState::Suspicious, "seed {s}: {h:?}, {:?}", p.state(m));
        let (mut walk, mut dwell, mut told, mut end, secs, tell) = (0.0, 0.0, None, None, p.g.k.attention.noise_secs, p.g.k.attention.tell_secs);
        let nothing = |p: &Play| p.lines(m).iter().filter(|l| **l == "must have been nothing").count();
        p.until(30.0, |p| {
            match p.held(m) {
                Some(h) => { if h.at_it { dwell += 1.0 / 60.0; } else { walk += 1.0 / 60.0; } end = Some(p.time()); }
                None => {}
            }
            if told.is_none() && nothing(p) > 0 { told = Some(p.time()); }
            p.state(m) == GuardState::Patrol
        }).unwrap_or_else(|| panic!("seed {s}: 30 s on he is {:?}", p.state(m)));
        assert!(walk > 0.5 && (dwell - secs).abs() < 0.1, "seed {s}: he walked {walk:.2} s and stood at it {dwell:.2} s (a noise is worth {secs} s)");
        let (told, end) = (told.expect("he told"), end.unwrap());
        assert!(((end - told) - tell).abs() < 0.1, "seed {s}: told {:.2} s before the end (the telling is {tell})", end - told);
        p.run(1.0);
        assert_eq!(nothing(&p), 1, "seed {s}: said {:?}", p.lines(m));
        drop(p);

        // CONTROL: the same noise at the same place, with nobody making it to be looked at
        let mut p = Play::scene(&hall(s), &seed(s));
        let m = p.guard("walk");
        p.run(1.0);
        let at = p.ahead(m, 5.0 + nudge(s, 1), 0.0);
        p.knock_at(at);
        if let Some(t) = p.always(3.0, |p| p.state(m) == GuardState::Patrol && p.held(m).is_none() && p.awareness(m) < 0.3) { panic!("seed {s}: a knock nobody meant had him at {t:.2} s: {:?}, awareness {:.2}", p.state(m), p.awareness(m)); }
    }
}

/// THE RULE: a lit man at the edge of a held eye feeds his awareness and writes no fix; the glimpses add up, the hold breaks, he says so, and only then is
/// his fix the man. CONTROL: the same man at the same range in line with the thing: no hold can hide him, his fix is the man at once.
/// A FAILURE MEANS: a lure no longer lets a player cross behind a man's attention, or it blinds him to what is in front of his eyes.
#[test]
fn a_lit_man_at_the_edge_of_a_held_eye_writes_no_fix_until_the_hold_breaks() {
    let walked_to_it = |s: u32| {
        let mut p = Play::scene(&hall(s), &seed(s));
        let m = p.guard("walk");
        p.run(1.0);
        let at = p.ahead(m, 4.0 + nudge(s, 1) * 0.5, 0.0);
        p.clack_at(at);
        p.until(15.0, |p| p.held(m).map_or(false, |h| h.at_it)).unwrap_or_else(|| panic!("seed {s}: he never got to the thing"));
        (p, m, at)
    };
    for s in SEEDS {
        let (mut p, m, at) = walked_to_it(s);
        let sam = p.ahead(m, 3.0, 3.0);
        p.sam_put(sam);
        let (mut no_fix, grew, was) = (0, std::cell::Cell::new(false), p.awareness(m));
        p.until(12.0, |p| {
            if p.held(m).is_some() {
                if p.fix(m).map_or(false, |f| dist(f, at) < 0.6) { no_fix += 1; }
                if p.awareness(m) > was + 0.02 { grew.set(true); }
                false
            } else { true }
        }).unwrap_or_else(|| panic!("seed {s}: 12 s lit at the edge of his eye and the hold stands: {:?}", p.state(m)));
        assert!(no_fix > 3, "seed {s}: only {no_fix} frames with his fix on the thing");
        let broke = game::attention::attention_lines();
        assert!(p.lines(m).iter().any(|l| broke.iter().any(|b| b == l)), "seed {s}: he said {:?}", p.lines(m));
        p.until(3.0, |p| p.fix(m).map_or(false, |f| dist(f, p.sam_at()) < 0.6)).unwrap_or_else(|| panic!("seed {s}: the hold broke and his fix is {:?}", p.fix(m)));
        drop(p);

        // CONTROL: in line with the thing
        let (mut p, m, _) = walked_to_it(s);
        let sam = p.ahead(m, 4.5, 0.0);
        p.sam_put(sam);
        let t = p.until(3.0, |p| p.fix(m).map_or(false, |f| dist(f, p.sam_at()) < 0.6)).unwrap_or_else(|| panic!("seed {s}: lit in front of him and his fix is {:?}", p.fix(m)));
        assert!(t < 1.5, "seed {s}: {t:.2} s to fix a man in the line of what he was looking at");
    }
}

/// THE RULE: a walking step three metres behind a held man does not reach him; the same step reaches a man nothing holds (the control); a running one breaks
/// the hold and his fix is the step.
/// A FAILURE MEANS: the player cannot walk behind a man who is looking at something, or he can run behind him too.
#[test]
fn a_held_ear_misses_a_walking_step_and_not_a_running_one() {
    for s in SEEDS {
        let stood = |s: u32, level: f64, hold: bool| {
            let mut p = Play::scene(&hall(s), &seed(s));
            let m = p.guard("walk");
            p.run(1.0);
            if hold {
                let at = p.ahead(m, 4.0, 0.0);
                p.clack_at(at);
                p.until(15.0, |p| p.held(m).map_or(false, |h| h.at_it)).unwrap_or_else(|| panic!("seed {s}: he never got to the thing"));
            }
            let (n0, b) = (p.small_heard(m), p.ahead(m, -3.0, nudge(s, 2) * 0.3));
            p.footfall_at(b, level);
            p.run(0.05);
            let heard = p.small_heard(m) > n0;
            (heard, p.held(m).is_some(), p.fix(m).map_or(false, |f| dist(f, b) < 1.5))
        };
        let (heard, held, _) = stood(s, 0.3, true);
        assert!(!heard && held, "seed {s}: a walking step behind a held man: heard {heard}, the hold stands {held}");
        let (heard, ..) = stood(s, 0.3, false);
        assert!(heard, "seed {s}: the same step behind a man nothing holds did not reach him");
        let (heard, held, to_it) = stood(s, 0.6, true);
        assert!(heard && !held && to_it, "seed {s}: a running step behind a held man: heard {heard}, the hold stands {held}, his fix the step {to_it}");
    }
}

/// A lit room cut across by a counter chest high from its north wall to two metres short of the south one; a man on the west side of it, walking a step to and fro.
fn across_a_counter(s: u32, way: bool) -> (Play, game::ids::GuardId) {
    let z = 12.25 + 0.5 * nudge(s, 0);
    let mut sc = Scene::new("attention-thread");
    sc.room("room", 6.0, 8.0, 26.0, 18.0).door("yard", Axis::Z, 6.0, 15.4).bulb(11.0, 13.0).bulb(21.0, 13.0);
    sc.patrol("walk", &[[13.75, z], [14.25, z]], 30.0);
    let mut p = scene_with(&sc, vec![a_box("counter", [16.0, 0.5, 12.0], [0.3, 0.5, 4.0], "white", &[])], &seed(s));
    p.g.k.attention.way = way;
    let m = p.guard("walk");
    p.run(1.0);
    (p, m)
}

/// THE RULE (the maintainer, of a dotted thread through a hut's wall: "weird that the line here is a straight line, not a pathed one. i'd expected pathed from
/// squad commands"): the thread of a man going over to a noise behind something he cannot walk through is told as his own way, with corners, and no leg of it through that thing; he
/// walks it, never over a metre off. CONTROL: the rule's own key off, the same man and noise: the floor is told a straight line.
/// A FAILURE MEANS: the picture says he will walk through a wall, or where he walks is not what the floor shows.
#[test]
fn the_thread_to_a_noise_past_a_counter_is_the_way_he_will_walk() {
    for s in SEEDS {
        let thing = [18.25, 12.0 + nudge(s, 1)];
        let going = |way: bool| {
            let (mut p, m) = across_a_counter(s, way);
            assert!(p.wall_between(p.at(m), thing), "seed {s}: the scene: the counter stands between him and the thing");
            p.clack_at(thing);
            p.until(4.0, |p| p.thread(m).map_or(false, |t| t.1.len() >= 2 || !way)).unwrap_or_else(|| panic!("seed {s}: no thread: {}; held {:?}, {:?}, fix {:?}, at {:?}", p.why(m), p.held(m), p.state(m), p.fix(m), p.at(m)));
            (p, m)
        };
        let (mut p, m) = going(true);
        let (walks, own) = p.thread(m).unwrap();
        assert!(walks && own.len() >= 2, "seed {s}: walks {walks}, {own:?}");
        let mut legs = vec![p.at(m)];
        legs.extend(own.iter().copied());
        for w in legs.windows(2).skip(1) { assert!(!p.wall_between(w[0], w[1]), "seed {s}: a leg of the thread goes through something solid: {w:?} of {legs:?}"); }
        assert!(dist(*legs.last().unwrap(), thing) < 1.2, "seed {s}: the thread ends {:.2} m from the thing", dist(*legs.last().unwrap(), thing));
        let off = |q: [f64; 2]| legs.windows(2).map(|w| { let (dx, dz) = (w[1][0] - w[0][0], w[1][1] - w[0][1]); let u = (((q[0] - w[0][0]) * dx + (q[1] - w[0][1]) * dz) / (dx * dx + dz * dz).max(1e-9)).clamp(0.0, 1.0); dist(q, [w[0][0] + dx * u, w[0][1] + dz * u]) }).fold(f64::MAX, f64::min);
        let mut far = 0.0f64;
        p.until(25.0, |p| { far = far.max(off(p.at(m))); p.held(m).map_or(true, |h| h.at_it) }).unwrap_or_else(|| panic!("seed {s}: 25 s and not at the thing"));
        assert!(far < 1.0 && p.held(m).is_some(), "seed {s}: {far:.2} m off the thread he set off on, held {:?}", p.held(m));
        drop(p);

        let (p, m) = going(false);
        assert!(!p.thread(m).unwrap().0, "seed {s}: the key off and the thread is still told as a way");
    }
}
