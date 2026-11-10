//! THE DARK: a green man alone stays out of a dark room; on alert nobody sits blind in the dark; the lights out, somebody goes to put them back.
mod world_ask;
use game::knobs::DarkdoorKWho;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use world_ask::{DarkAsk, WorldAsk};

const SEEDS: [u32; 3] = [1, 2, 3];
fn seed(seed: u32) -> Opts { Opts { seed, ..Default::default() } }

/// A lit room and, through a door that stands open, a room with its lamp off or on; a man walking a step to and fro in line with the doorway, a mate of his
/// `mate` metres behind him if said; Sam out in the dark lot.
fn beside_a_room(s: u32, lit: bool, who: DarkdoorKWho, mate: Option<f64>) -> (Play, game::ids::GuardId, [f64; 2]) {
    let z = 13.0;
    let x = 15.0 + 0.5 * nudge(s, 0);
    let mut sc = Scene::new("beside-a-room");
    sc.room("near", 8.0, 9.0, 18.0, 17.0).room("far", 18.0, 9.0, 28.0, 17.0).door("between", Axis::Z, 18.0, 12.4).door("yard", Axis::Z, 8.0, 15.4);
    sc.bulb(13.0, 13.0);
    if lit { sc.bulb(23.0, 13.0).bulb(25.0, 11.0).bulb(25.0, 15.0); } else { sc.bulb_off(23.0, 13.0); }
    sc.patrol("walk", &[[x, z], [x + 0.25, z]], 60.0);
    if let Some(m) = mate { sc.patrol("mate", &[[x - m, z + 2.0], [x - m + 0.25, z + 2.0]], 60.0); }
    let mut p = Play::scene(&sc, &seed(s));
    p.g.k.darkdoor.who = who;
    p.door_stands_open("between");
    p.run(0.5);
    let m = p.guard("walk");
    p.clack_at([x + 5.5, z]);
    (p, m, [18.0, z])
}
fn crossed(p: &Play, m: game::ids::GuardId) -> bool { p.at(m)[0] > 18.0 }

/// THE RULE: a green man alone, nobody to call, walked at a dark room: he stops short of the frame and off its middle, says one word to himself as he stops, spoken, and never goes through. After the
/// wait he says nobody's coming, once, and gives it up where he stands. CONTROL: the same room with its lamps on: he walks straight in and says nothing of it.
/// A FAILURE MEANS: a frightened man no longer shows it, or every room stops him.
#[test]
fn a_green_man_alone_stays_out_of_a_dark_room() {
    for s in SEEDS {
        let (mut p, m, frame) = beside_a_room(s, false, DarkdoorKWho::All, None);
        p.g.k.darkdoor.wait_secs = 6.0;   // (the wait's length is a number: what is asked is what he does at its end)
        let (reach, wait) = (p.g.k.darkdoor.stop_m, p.g.k.darkdoor.wait_secs);
        p.until(10.0, |p| p.balks(m)).unwrap_or_else(|| panic!("seed {s}: he never balked: {}, at {:?}, {:?}", p.why(m), p.at(m), p.state(m)));
        let d = dist(p.at(m), frame);
        assert!(d <= reach + 0.3 && d > 0.8, "seed {s}: he balked {d:.2} m from the frame");
        if let Some(t) = p.always(wait - 1.0, |p| !crossed(p, m) && p.balks(m)) { panic!("seed {s}: {t:.2} s on he crossed {} or was let go: at {:?}", crossed(&p, m), p.at(m)); }
        let spoken: Vec<_> = p.said.iter().filter(|l| l.who == m).filter(|l| p.dark_line(&l.text, game::darkdoor::LineKind::Self_)).map(|l| l.radio).collect();
        assert_eq!(spoken, vec![false], "seed {s}: to himself, once, out loud: he said {:?}", p.lines(m));
        p.until(3.0, |p| !p.balks(m)).unwrap_or_else(|| panic!("seed {s}: past the wait and he still stands there"));
        p.run(0.5);
        let gave_up: Vec<_> = p.lines(m).into_iter().filter(|l| p.dark_line(l, game::darkdoor::LineKind::Nobody)).collect();
        assert!(gave_up.len() == 1 && !crossed(&p, m), "seed {s}: he said {:?}, crossed {}", p.lines(m), crossed(&p, m));
        drop(p);

        let (mut p, m, _) = beside_a_room(s, true, DarkdoorKWho::All, None);
        p.until(8.0, |p| crossed(p, m)).unwrap_or_else(|| panic!("seed {s}: the room is lit and he did not go in: {}", p.why(m)));
        assert!(!p.balks(m) && p.lines(m).iter().all(|l| !p.dark_any(l)), "seed {s}: into a lit room, he said {:?}", p.lines(m));
    }
}

/// THE RULE: a steady man in the same place walks straight in with no line. It is the green man the room stops.
/// A FAILURE MEANS: the rule has stopped asking who he is.
#[test]
fn a_steady_man_walks_into_the_dark() {
    for s in SEEDS {
        let (mut p, m, _) = beside_a_room(s, false, DarkdoorKWho::None, None);
        p.until(8.0, |p| crossed(p, m)).unwrap_or_else(|| panic!("seed {s}: a steady man and he did not go in: {}", p.why(m)));
        assert!(p.lines(m).iter().all(|l| !p.dark_any(l)), "seed {s}: he said {:?}", p.lines(m));
    }
}

/// THE RULE: a green man with a free mate in earshot calls out loud, the mate answers out loud, walks over and says his line at his shoulder, and both then go through within seconds. CONTROL is the man alone, above.
/// A FAILURE MEANS: the fear stops the squad for good, or nobody comes when called.
#[test]
fn a_mate_in_earshot_comes_and_they_go_in_together() {
    for s in SEEDS {
        let (mut p, m, _) = beside_a_room(s, false, DarkdoorKWho::All, Some(4.0));
        let mate = p.guard("mate");
        p.until(30.0, |p| crossed(p, m) && crossed(p, mate)).unwrap_or_else(|| panic!("seed {s}: 30 s and not both through: {:?} {:?}; {:?} / {:?}", p.at(m), p.at(mate), p.lines(m), p.lines(mate)));
        let of = |who, kind| p.said.iter().find(|l| l.who == who && p.dark_line(&l.text, kind)).map(|l| (l.t, l.radio));
        use game::darkdoor::LineKind::*;
        let (call, copy, with) = (of(m, Call), of(mate, Copy), of(mate, With));
        let (call, copy, with) = (call.expect("the call"), copy.expect("the answer"), with.expect("his line at his shoulder"));
        assert!(!call.1 && !copy.1 && !with.1 && call.0 < copy.0 && copy.0 < with.0, "seed {s}: {call:?} {copy:?} {with:?}");
    }
}

/// Two dark rooms, or lit ones, a defender in each with his gun on the door; the first of them with a second man facing him from across his room; the team out in the yard, Sam unhurt.
fn dark_floor(s: u32, lit: bool) -> (Play, game::ids::GuardId, game::ids::GuardId) {
    let (a, b) = ([12.25 + 0.3 * nudge(s, 0), 12.25], [15.75, 12.25 + 0.3 * nudge(s, 1)]);
    let mut sc = Scene::new("dark-floor");
    sc.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0).door("west-yard", Axis::X, 16.0, 13.4).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    if lit { sc.bulb(14.0, 13.0).bulb(22.0, 13.0).bulb(12.0, 12.0); } else { sc.bulb_off(14.0, 13.0).bulb_off(22.0, 13.0); }
    sc.team([14.0, 24.0], std::f64::consts::PI, &[[13.0, 25.0], [15.0, 25.0], [13.0, 26.0], [15.0, 26.0]]);
    sc.defender("west", a, "west-yard").defender("west", b, "west-yard").defender("east", [24.25, 12.25], "east-yard");
    let mut p = Play::scene(&sc, &Opts { seed: s, squad: 4, ..Default::default() });
    p.sam_unhurt();
    p.g.k.darkfoe.carry = 1.0;
    p.run(0.5);
    let (ma, mb) = (p.defender_at(a), p.defender_at(b));
    (p, ma, mb)
}

/// THE RULE: nobody has been told: longer than any thumb takes, no lamp burns. One man told (gunfire reached him) in the dark: his lamp comes on, inside the seconds of the switch under his thumb. CONTROL: the same man told in a lit room leaves it off.
/// A FAILURE MEANS: a garrison lights up by itself and gives itself away, or an enemy on alert sits blind in the dark.
#[test]
fn a_man_on_alert_does_not_sit_blind_in_the_dark() {
    for s in [1u32, 2] {
        let (mut p, a, _) = dark_floor(s, false);
        if let Some(t) = p.always(3.5, |p| p.lamps_burning() == 0) { panic!("seed {s}: nobody told and a lamp burns at {t:.2} s"); }
        p.gunfire_reached(a);
        p.until(5.0, |p| p.lamp_on(a)).unwrap_or_else(|| panic!("seed {s}: told in the dark, and no lamp: {}", p.why(a)));
        drop(p);
        if s != 1 { continue; }

        let (mut p, a, _) = dark_floor(s, true);
        p.gunfire_reached(a);
        if let Some(t) = p.always(3.5, |p| !p.lamp_on(a)) { panic!("seed {s}: told in a lit room and his lamp comes on at {t:.2} s"); }
    }
}

/// THE RULE: the lamps are the word: a man who sees a burning lamp of a mate who knows lights his own, though nobody told him and he never knows for good by it.
/// A FAILURE MEANS: one lit lamp no longer spreads through a room, or seeing a lamp turns into knowing.
#[test]
fn a_lamp_is_the_word_and_not_knowing() {
    for s in SEEDS {
        let (mut p, a, b) = dark_floor(s, false);
        p.face_each_other(a, b);
        p.run(1.0);
        p.gunfire_reached(a);
        p.until(10.0, |p| { p.face_each_other(a, b); p.lamp_on(b) }).unwrap_or_else(|| panic!("seed {s}: 10 s facing his mate's lamp ({}) and his own is off: {}", p.lamp_on(a), p.why(b)));
        assert!(p.heard_nothing(b) && !p.dark_knows(b), "seed {s}: his lamp is lit by the word alone, and he knows {}", p.dark_knows(b));
    }
}

/// The depot's room clear at night, the team on its marks, Sam out of it, the mains pulled by hand: who goes, and what they say.
fn depot(s: u32) -> Play {
    let mut p = Play::mission("depot.json", "?levelFile=depot.json&roomclear&squad4", &seed(s));
    p.sam_unhurt();
    p.g.player_invisible = true;
    p.run(1.5);
    p
}

/// THE RULE: the lights go: their word is on the net before any man has left his post, the first step comes the beat after it; no more than the rule's men go, every man who left was sent, every room that had a man keeps one.
/// A FAILURE MEANS: a blackout empties a garrison, or nobody ever comes to put the lights on.
#[test]
fn the_lights_go_out_and_somebody_goes_to_put_them_back() {
    let s = 11;
    let mut p = depot(s);
    let men = p.hostiles();
    let posts: Vec<[f64; 2]> = men.iter().map(|&m| p.at(m)).collect();
    p.mains_pulled();
    let (t0, beat, most) = (p.time(), p.g.k.powerout.beat, p.g.k.powerout.men);
    let mut first_off = None;
    p.until(12.0, |p| { if first_off.is_none() && men.iter().zip(&posts).any(|(&m, q)| dist(p.at(m), *q) > 0.6) { first_off = Some(p.time()); } first_off.is_some() }).unwrap_or_else(|| panic!("12 s and nobody has left his post: {:?}", p.powerout_line()));
    let (word, go) = (p.powerout_at(game::powerout::PowerOutEventWhat::Said), p.powerout_at(game::powerout::PowerOutEventWhat::Go));
    let (word, go) = (word.expect("their word"), go.expect("the step"));
    assert!(word < first_off.unwrap() && go - word >= beat - 0.05, "the lights at {t0:.1}: word {word:.1}, step {go:.1}, first man off {:.1}", first_off.unwrap());
    p.run(4.0);
    let left: Vec<_> = men.iter().zip(&posts).filter(|(&m, q)| dist(p.at(m), **q) > 0.6).map(|(&m, _)| m).collect();
    let sent = p.powerout_sent();
    assert!(!left.is_empty() && left.len() as f64 <= most && left.iter().all(|m| sent.contains(m)), "left {left:?}, sent {sent:?} (at most {most})");
}

/// THE CONTROL: gunfire has reached the floor before the mains go: nobody leaves.
#[test]
fn a_floor_that_knows_does_not_send_for_the_mains() {
    let s = 11;
    let mut p = depot(s);
    let men = p.hostiles();
    let posts: Vec<[f64; 2]> = men.iter().map(|&m| p.at(m)).collect();
    p.gunfire_reached(men[0]);
    p.mains_pulled();
    if let Some(t) = p.always(12.0, |p| men.iter().zip(&posts).all(|(&m, q)| dist(p.at(m), *q) < 1.2 || !p.powerout_sent().contains(&m))) { panic!("the floor already knows and a man was sent at {t:.2} s: {:?}", p.powerout_line()); }
    assert!(p.powerout_sent().is_empty(), "the floor already knows and {:?} went", p.powerout_sent());
}

/// A small floor in the lot's corner by the breaker: two rooms, a man posted in each and a second in the near one, the team off in the yard.
fn by_the_breaker(s: u32) -> Play {
    let mut sc = Scene::new("by-the-breaker");
    sc.room("west", 24.0, 12.0, 31.0, 20.0).room("east", 31.0, 12.0, 38.0, 20.0).door("west-yard", Axis::X, 20.0, 26.4).door("east-yard", Axis::X, 20.0, 33.4).door("between", Axis::Z, 31.0, 15.4);
    sc.bulb(27.5, 16.0).bulb(34.5, 16.0);
    sc.team([10.0, 25.0], std::f64::consts::FRAC_PI_2, &[[9.0, 24.0], [9.0, 26.0], [8.0, 24.0], [8.0, 26.0]]);
    sc.defender("west", [26.25 + 0.3 * nudge(s, 0), 14.25], "west-yard").defender("west", [29.25, 14.25], "west-yard").defender("east", [36.25, 14.25], "east-yard").defender("east", [33.25, 14.25 + 0.3 * nudge(s, 1)], "east-yard");
    let mut p = Play::scene(&sc, &seed(s));
    p.sam_unhurt();
    p.g.player_invisible = true;
    p.run(1.0);
    p
}

/// THE RULE: the mains come back with a man of theirs at the lever, later than the word by the lever's own fumble, and every man out is back on the post he left. The lights go a second time: nobody walks out again.
/// A FAILURE MEANS: the men sent never get there, or never go home, and a floor comes apart by one blackout.
#[test]
fn the_men_put_the_mains_back_and_go_home_and_not_twice() {
    let mut p = by_the_breaker(1);
    let men = p.hostiles();
    let posts: Vec<[f64; 2]> = men.iter().map(|&m| p.at(m)).collect();
    p.mains_pulled();
    p.until(70.0, |p| !p.g.light_home.mains.active).unwrap_or_else(|| panic!("70 s and the mains are down: {}", p.powerout_line()));
    let word = p.powerout_at(game::powerout::PowerOutEventWhat::Said).expect("their word");
    assert!(p.time() - word >= 2.2, "the mains came back {:.1} s after the word", p.time() - word);
    let sent = p.powerout_sent();
    assert!(!sent.is_empty(), "the mains came back and nobody was sent: {}", p.powerout_line());
    p.until(40.0, |p| p.powerout_homes() == sent.len() && men.iter().zip(&posts).all(|(&m, q)| dist(p.at(m), *q) < 0.7)).unwrap_or_else(|| panic!("40 s on not all home: {}", p.roll()));
    let n = p.said.len();
    p.mains_pulled();
    if let Some(t) = p.always(4.0, |p| men.iter().zip(&posts).all(|(&m, q)| dist(p.at(m), *q) < 0.7)) { panic!("the lights went again and a man walked at {t:.2} s: {}", p.powerout_line()); }
    assert!(p.said[n..].iter().filter(|l| l.radio).count() <= 2, "the second time: {:?}", &p.said[n..]);
}
