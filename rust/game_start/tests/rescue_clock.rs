//! THE HOSTAGE RESCUE'S TWO CLOCKS: the captor's, and how long a room takes to be set. Sam stands at the arch unless a test moves him; the pair is told to hold, which starts the clock with nobody moving.
mod doors_ask;
use doors_ask::*;
use game::orders::Who;
use game_start::play::{Opts, Play};

fn stood(seed: u32, first: f64, second: f64, set: f64) -> Play {
    let mut p = Play::mission("office.json", "?levelFile=office.json&rescue", &Opts { seed, squad: 2, ..Default::default() });
    p.sam_unhurt();
    p.rescue_on(first, second, set);
    p.run(0.1);
    p
}
fn first_at(p: &Play, what: &str) -> Option<f64> { p.rescue_log().iter().find(|l| l.1.starts_with(what)).map(|l| l.0) }
fn up_hostages(p: &Play) -> usize { p.rescue_hostages().iter().filter(|&&h| p.state(h) != game::game::GuardState::Dead).count() }

/// THE RULE: the captor's clock: shouting, counting, about to, he turns on her, and the first hostage is shot its seconds and his aim after the start, the second her seconds and his aim later, on real rounds, and the run is lost. The control is the next test: broken, nobody is shot.
/// A FAILURE MEANS: a hostage rescue has no clock, or it is not the one the words on screen say.
#[test]
fn the_captors_clock_runs_its_steps_and_shoots_her() {
    for s in [21, 22] {
        let mut p = stood(s, 5.0, 2.0, 2.0);
        p.order("hold", p.sam_at(), Who::All);
        p.until(40.0, |p| p.rescue_lost().is_some()).unwrap_or_else(|| panic!("seed {s}: never over: {:?}", p.rescue_log()));
        let log = p.rescue_log();
        let i = |w: &str| log.iter().position(|l| l.1 == w).unwrap_or_else(|| panic!("seed {s}: no '{w}': {log:?}"));
        assert!(i("step:shouting") < i("step:counting") && i("step:counting") < i("step:about to") && i("step:about to") < i("act"), "seed {s}: {log:?}");
        let t0 = p.rescue_t0();
        let aim = p.rescue_aim();
        let shots: Vec<f64> = log.iter().filter(|l| l.1.starts_with("executed:")).map(|l| l.0 - t0).collect();
        assert_eq!(shots.len(), 2, "seed {s}: both shot: {log:?}");
        assert!((shots[0] - (5.0 + aim)).abs() < 0.25, "seed {s}: the first at {:.2}, expected {:.2}", shots[0], 5.0 + aim);
        assert!((shots[1] - shots[0] - (2.0 + aim)).abs() < 0.25, "seed {s}: the second {:.2} later", shots[1] - shots[0]);
        assert_eq!((p.rescue_lost(), up_hostages(&p)), (Some(true), 0), "seed {s}: the run is lost");
    }
}

/// THE RULE: a captor dazzled for five seconds: his clock stands still for five. The captor dead and nobody of his in the room: the clock stops and nobody is shot; shot while he lines up on her, the round never comes. The control is the test above.
/// A FAILURE MEANS: a flash buys the team nothing, or she is shot by a dead man.
#[test]
fn a_flash_stands_his_clock_still_and_his_death_stops_it() {
    for s in [21, 22] {
        let mut p = stood(s, 5.0, 2.0, 2.0);
        p.order("hold", p.sam_at(), Who::All);
        p.run(1.5);
        let cap = p.rescue_men()[0];
        p.dazzled_for(cap, 5.0);
        let r0 = p.rescue_clock();
        p.run(4.9);
        assert!((p.rescue_clock() - r0).abs() < 0.01, "seed {s}: the clock ran {r0:.3} to {:.3} across the dazzle", p.rescue_clock());
        p.until(30.0, |p| first_at(p, "executed:").is_some()).unwrap_or_else(|| panic!("seed {s}: never shot"));
        let t = first_at(&p, "executed:").unwrap() - p.rescue_t0();
        let aim = p.rescue_aim();
        assert!((t - (10.0 + aim)).abs() < 0.4, "seed {s}: the first shot at {t:.2}, expected {:.2}", 10.0 + aim);
        drop(p);

        let mut p = stood(s, 5.0, 2.0, 2.0);
        p.order("hold", p.sam_at(), Who::All);
        p.run(3.5);
        p.until(4.0, |p| first_at(p, "act").is_some()).unwrap_or_else(|| panic!("seed {s}: he never turned on her: {:?}", p.rescue_log()));
        let cap = p.rescue_men()[0];
        p.falls(cap, game::game::DownBy::Player);
        p.run(9.0);
        assert_eq!(up_hostages(&p), 2, "seed {s}: nobody is shot: {:?}", p.rescue_log());
        assert!(first_at(&p, "act:broken").is_some() && first_at(&p, "step:stopped").is_some(), "seed {s}: broken, and stopped: {:?}", p.rescue_log());
        assert_eq!(p.rescue_lost(), None, "seed {s}: not lost");
    }
}

/// F straight off, with the room's men set after `set` seconds: blind rounds from the room, and what the roamer and the captor did.
fn assault(seed: u32, set: f64) -> (f64, Option<f64>, usize) {
    let mut p = stood(seed, 60.0, 2.0, set);
    p.run(0.2);
    p.tap("KeyF");
    p.until(40.0, |p| p.rescue_lost().is_some() || p.men().is_empty() || p.rescue_log().iter().any(|l| l.1 == "secured"));
    assert!(first_at(&p, "bang").is_some(), "seed {seed}: F went nowhere: {:?}", p.rescue_log());
    let at = |w: &str| first_at(&p, w);
    let off_post = match (at("learned:Roamer 1"), at("released:Roamer 1")) { (Some(a), Some(b)) => Some(b - a), _ => None };
    let cap = p.fight_notes("Captor").iter().filter(|n| n.contains("goes to the fight") || n.contains("falls back")).count();
    (p.rescue_blind(), off_post, cap)
}

/// THE RULE: a room hit before its men are set fires no blind round through its door; set, the roamer leaves his post no sooner than his seconds after he first hears it; and the captor never goes to a fight next door: he is the clock. The control is the room that is set on hearing: its roamer is off his post in seconds.
/// A FAILURE MEANS: a quick assault earns nothing over a slow one.
#[test]
fn a_room_hit_before_its_men_are_set_fires_no_blind_round() {
    for s in [11, 12] {
        let (blind, _, cap) = assault(s, 30.0);
        assert_eq!(blind, 0.0, "seed {s}: blind rounds from men not set");
        assert_eq!(cap, 0, "seed {s}: the captor left his post");
        let (_, off_post, cap) = assault(s, 0.5);
        assert_eq!(cap, 0, "seed {s}: the control: the captor left his post");
        assert!(off_post.is_none_or(|t| (0.45..8.5).contains(&t)), "seed {s}: the control: the roamer off his post after {off_post:?} s");
    }
}
