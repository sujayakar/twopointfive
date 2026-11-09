//! THE MISSION OVER: the run's first step stands again with what an evening keeps, and nothing else goes over (game::savestate::control `restart`).
use game::savestate::control;
use game_start::play::{Opts, Play};

fn pass() -> Play { Play::mission("pass.json", "?levelFile=pass.json&roomclear&squad4", &Opts { seed: 3, dress: true, ..Default::default() }) }

/// THE RULE: an evening of restarts holds no more than its first try: the men of a try that ended are gone, row and all.
/// A FAILURE MEANS: every Again makes each later save, clone and rollback copy dearer.
#[test]
fn ten_restarts_hold_no_more_than_one() {
    let mut p = pass();
    p.run(4.0);
    assert!(control::restart(&mut p.g, None));
    p.run(4.0);
    let (one, chars) = (game::io::export(&p.g).len(), p.g.ch.len());
    for _ in 0..9 { assert!(control::restart(&mut p.g, None)); p.run(4.0); }
    let (ten, later) = (game::io::export(&p.g).len(), p.g.ch.len());
    assert_eq!(later, chars, "the men's rows went on growing");
    assert!(ten < one + one / 50, "the state grew from {one} to {ten} bytes over nine restarts");
}

/// THE RULE: what the player and this machine set goes over, the die goes on, numbers are not dealt twice, and the clock and the floor are the first step's.
/// A FAILURE MEANS: god mode ticked and the card's restart kills him, or a restart plays the last try's luck again.
#[test]
fn an_evening_keeps_its_switches_and_its_die() {
    let mut p = pass();
    p.run(1.0);
    let start = control::kept_of(control::stood(&mut p.g)).into_iter().find(|(k, _)| k == control::START).expect("the run's first step is kept").1.image.game.time;
    p.g.god_mode = true;
    p.g.tune.vis_floor *= 2.0;
    p.run(1.0);
    for _ in 0..5 { game::math_dice::sim_random(&mut p.g); }   // (the die is ahead of where the first step had it)
    let (floor, rolled, serial, restarts, run) = (p.g.tune.vis_floor, sim::state::state_read(p.g.sim).dice.n, p.g.serial_next, p.g.stats.cleanrestart.restarts, p.g.mission.serial);
    assert!(control::restart(&mut p.g, None));
    assert!(p.g.god_mode && p.g.tune.vis_floor == floor, "the switches went back with the game");
    assert_eq!(sim::state::state_read(p.g.sim).dice.n, rolled, "the die went back with the game");
    assert!(p.g.serial_next >= serial, "a number would be dealt twice");
    assert_eq!(p.g.time, start, "the clock is the first step's");
    assert_ne!(p.g.mission.serial, run, "it is a new run");
    assert_eq!(p.g.stats.cleanrestart.restarts, restarts + 1);
}

/// THE RULE: Shift+Enter inside a step asks; the step ends on the try it began on, and the mission starts over as it ends, on every kind of host.
/// A FAILURE MEANS: a whole game swapped under a rule in the middle of its tick, or the key does nothing.
#[test]
fn shift_enter_starts_it_over_as_the_step_ends() {
    let mut p = pass();
    p.run(1.0);
    let run = p.g.mission.serial;
    p.key_down("ShiftLeft");
    p.key_down("Enter");
    p.frame();
    p.key_up("ShiftLeft");
    p.key_up("Enter");
    assert_ne!(p.g.mission.serial, run, "the key restarted nothing");
    assert!(p.told.iter().any(|t| t.back && t.cut), "the host is told to take it whole and cut the picture");
    assert!(p.g.time < 0.1, "the clock is the first step's: {}", p.g.time);
    p.run(0.5);
    assert_eq!(p.g.saves.again, None);
}

/// THE RULE: the key on the very step that keeps the run's start starts it over once, and the game goes on from there.
/// A FAILURE MEANS: a start kept with the word still in it, and a game that restarts every frame for good.
#[test]
fn the_key_on_the_first_step_restarts_once() {
    let mut p = pass();
    p.key_down("ShiftLeft");
    p.key_down("Enter");
    p.frame();
    p.key_up("ShiftLeft");
    p.key_up("Enter");
    p.run(0.5);
    assert!(p.g.time > 0.4, "the clock did not go on: {}", p.g.time);
}

/// THE RULE: a run started some other way than a step (a mission's start between two steps) goes back to itself, not to the run before it.
/// A FAILURE MEANS: a card's restart that lands on the mission before.
#[test]
fn a_restart_goes_back_to_the_run_it_is_in() {
    let mut p = pass();
    p.run(0.5);
    game::game::game_restart_encounter(&mut p.g);
    let run = p.g.mission.serial;
    assert!(control::restart(&mut p.g, None));
    let start = control::kept_of(control::stood(&mut p.g)).into_iter().find(|(k, _)| k == control::START).unwrap().1;
    assert_eq!(start.image.game.mission.serial, run, "it went back to the run before");
}
