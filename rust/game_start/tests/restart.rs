//! THE MISSION OVER: the level stands again, the men are stood on it by the stand's own lines, what an evening keeps goes over and nothing else (game::savestate::control `restart`).
use game::bytes::Parts;
use game::savestate::control;
use game_start::play::{Opts, Play};

const MISSIONS: &[(&str, &str)] = &[("pass.json", "?levelFile=pass.json&roomclear&squad4"), ("office.json", "?levelFile=office.json&rescue"), ("consulate.json", "?levelFile=consulate.json&rescue&squad=2"), ("crossing.json", "?levelFile=crossing.json&quiet&squad=1"), ("bazaar.json", "?levelFile=bazaar.json&quiet"), ("office.json", "?levelFile=office.json&nightbreach")];

fn pass() -> Play { Play::mission("pass.json", "?levelFile=pass.json&roomclear&squad4", &Opts { seed: 3, dress: true, ..Default::default() }) }

/// THE RULE: an evening of restarts holds no more than its first try: the men of a try that ended are gone, row and all.
/// how much a game's state comes to, part by part as two games are held to each other
fn state_bytes(p: &mut Play) -> usize {
    let (mut w, mut n) = (game::bytes::W::new(), 0);
    let mut say = |x: &dyn game::bytes::ToBytes| { w.b.clear(); x.w(&mut w); n += w.b.len(); };
    p.g.parts(&mut |_, x| say(x));
    p.g.m.parts(&mut |_, x| say(x));
    p.g.engine.parts(&mut |_, x| say(x));
    p.g.tab.parts(&mut |_, x| say(x));
    n
}
/// A FAILURE MEANS: every Again makes each later save, clone and rollback copy dearer.
#[test]
fn ten_restarts_hold_no_more_than_one() {
    let mut p = pass();
    p.run(4.0);
    assert!(control::restart(&mut p.g));
    p.run(4.0);
    let (one, chars) = (state_bytes(&mut p), p.g.ch.len());
    for _ in 0..9 { assert!(control::restart(&mut p.g)); p.run(4.0); }
    let (ten, later) = (state_bytes(&mut p), p.g.ch.len());
    assert_eq!(later, chars, "the men's rows went on growing");
    assert!(ten < one + one / 50, "the state grew from {one} to {ten} bytes over nine restarts");
}

/// THE RULE: what the player and this machine set goes over, the die goes on, numbers are not dealt twice, and the clock and the floor are the first step's.
/// A FAILURE MEANS: god mode ticked and the card's restart kills him, or a restart plays the last try's luck again.
#[test]
fn an_evening_keeps_its_switches_and_its_die() {
    let mut p = pass();
    p.run(1.0);
    p.g.god_mode = true;
    p.g.tune.vis_floor *= 2.0;
    p.run(1.0);
    p.g.dice.n += 5.0;   // (the die is ahead of where the first step had it)
    let (floor, rolled, serial, restarts, run) = (p.g.tune.vis_floor, p.g.dice.n, p.g.serial_next, p.g.stats.cleanrestart.restarts, p.g.mission.serial);
    assert!(control::restart(&mut p.g));
    assert!(p.g.god_mode && p.g.tune.vis_floor == floor, "the switches went back with the game");
    assert!(p.g.dice.n >= rolled, "the die went back with the game");
    assert!(p.g.serial_next >= serial, "a number would be dealt twice");
    assert_eq!(p.g.time, 0.0, "the clock starts over");
    assert_ne!(p.g.mission.serial, run, "it is a new run");
    assert!(p.g.stats.cleanrestart.restarts > restarts);
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
    assert!(p.g.time < 0.1, "the clock did not start over: {}", p.g.time);
    p.run(0.5);
    assert_eq!(p.g.saves.again, false);
}

/// THE RULE: the key on a game's very first step starts it over once, and the game goes on from there.
/// A FAILURE MEANS: a game that restarts every frame for good.
#[test]
fn the_key_on_the_first_step_restarts_once() {
    let mut p = pass();
    let (active, chars) = (p.g.staged.up, p.g.ch.len());
    p.key_down("ShiftLeft");
    p.key_down("Enter");
    p.frame();
    p.key_up("ShiftLeft");
    p.key_up("Enter");
    p.run(0.5);
    assert!(p.g.time > 0.4, "the clock did not go on: {}", p.g.time);
    p.run(2.0);
    assert!(control::restart(&mut p.g));
    assert_eq!((p.g.staged.up, p.g.ch.len()), (active, chars), "the mission did not stand again");
    assert!(p.g.time < 0.1, "the clock did not start over: {}", p.g.time);
}

/// THE RULE: a switch the player set in a try cannot stop the mission coming back: it stands as it first did, and the switch is on it after.
/// A FAILURE MEANS: AI off in the sandbox, then Restart, and the mission refuses with a line and leaves him on an empty floor.
#[test]
fn a_switch_set_in_a_try_does_not_stop_the_mission_coming_back() {
    let mut p = pass();
    p.run(0.5);
    let (active, guards) = (p.g.staged.up, p.g.guards.len());
    p.g.ai_enabled = false;
    p.g.tune.teams = false;
    assert!(control::restart(&mut p.g));
    assert_eq!((p.g.staged.up, p.g.guards.len()), (active, guards), "the mission did not stand again");
    assert!(!p.g.ai_enabled && !p.g.tune.teams, "the switches went back with the game");
}

/// THE RULE: the hour is the first step's, so a night's lights are never under a noon's sky.
/// A FAILURE MEANS: an hour picked in a try stays in the sky over a level whose lights went back to night.
#[test]
fn the_hour_goes_back() {
    let mut p = pass();
    let night = game::hours::told::told(&p.g);
    p.run(0.5);
    assert!(game::hours::apply_hour(&mut p.g, game::hours::HourId::Midday), "the hour did not take");
    assert_ne!(game::hours::told::told(&p.g).zenith, night.zenith);
    assert!(control::restart(&mut p.g));
    assert_eq!(game::hours::told::told(&p.g), night);
}

/// THE RULE: a lever of the difficulty goes over whole: the rung it is set to and the table it wrote.
/// A FAILURE MEANS: the next thing to work the tables out reads the first try's rung and the lever comes undone by itself.
#[test]
fn a_lever_of_the_difficulty_goes_over_whole() {
    let mut p = pass();
    let was = p.g.k.fog.cone;
    game::difficulty::set_cone_visit(&mut p.g, Some(!was));
    assert!(control::restart(&mut p.g));
    assert_eq!((p.g.m.difficulty.cone_visit, p.g.k.fog.cone), (Some(!was), !was));
}

/// THE RULE: a wand the player picked for a teammate reaches him on the next try and on every one after that, and the mission's words said again take nothing off him.
/// A FAILURE MEANS: the kit screen's choice is on the card and no man carries it, or it is lost on the second try.
#[test]
fn a_card_reaches_its_man_on_every_restart() {
    use game::kit::{KitField, KitValue};
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: 1, ..Default::default() });
    p.g.m.kit.deal_own = true;
    if !game::kit::kit_set(&mut p.g, 1.0, KitField::Wand, &KitValue::Bool(true)) {
        let bangs = p.g.m.kit.kit.members[1].bangs;
        assert!(game::kit::kit_set(&mut p.g, 1.0, KitField::Bangs, &KitValue::Num((bangs - 1.0).max(0.0))) && game::kit::kit_set(&mut p.g, 1.0, KitField::Wand, &KitValue::Bool(true)));
    }
    let carried = |p: &Play| {
        assert!(p.g.m.kit.kit.members[1].wand, "the card lost it");
        p.mates().into_iter().filter(|&m| game::wandorder::carries(&p.g, m)).count()
    };
    for tries in 1..=3 {
        assert!(control::restart(&mut p.g));
        p.run(0.5);
        assert_eq!(carried(&p), 1, "try {tries}: one man carries the wand on the card");
        // (the words of the start once more, on the mission that is up: it stands as it stands)
        game::scenario_run::start_from_query(&mut p.g, &game::scenario_run::StartFromQuerySearch::Text("?levelFile=office.json&roomclear&squad4".into()));
        p.run(0.1);
        assert_eq!(carried(&p), 1, "try {tries}: a second start took it off him");
    }
}

/// THE RULE: the player's own card reaches him on the next try, a change of it too: what he carries is the card's, never the last try's.
/// A FAILURE MEANS: the kit screen says one thing and his hands the other, for good.
#[test]
fn the_players_own_card_reaches_him_changed() {
    use game::kit::{KitField, KitValue};
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: 1, ..Default::default() });
    p.g.m.kit.deal_own = true;
    let carried = |p: &Play| p.g.pl[p.g.player].wand;
    if !game::kit::kit_set(&mut p.g, 0.0, KitField::Wand, &KitValue::Bool(true)) {
        let bangs = p.g.m.kit.kit.members[0].bangs;
        assert!(game::kit::kit_set(&mut p.g, 0.0, KitField::Bangs, &KitValue::Num((bangs - 1.0).max(0.0))) && game::kit::kit_set(&mut p.g, 0.0, KitField::Wand, &KitValue::Bool(true)));
    }
    assert!(control::restart(&mut p.g));
    assert!(carried(&p), "his card gives him the wand and he has none");
    assert!(game::kit::kit_set(&mut p.g, 0.0, KitField::Wand, &KitValue::Bool(false)));
    assert!(control::restart(&mut p.g));
    assert!(!carried(&p), "his card takes the wand away and he has it still");
}

/// THE RULE: every kind of mission starts over as it started: the same team, the same floor, the same number of everything, the host's rows where they were.
/// A FAILURE MEANS: a try that plays on another level from the first, or a host that writes to rows that are another's.
#[test]
fn every_kind_of_mission_starts_over_as_it_started() {
    for (file, link) in MISSIONS {
        let mut p = Play::mission(file, link, &Opts { seed: 3, dress: true, ..Default::default() });
        p.run(1.0);
        let count = |p: &Play| (p.g.guards.len(), p.mates().len(), p.g.civilians.len(), p.g.ch.len(), p.g.doors.list.len(), p.g.engine.world.statics.len(), p.g.engine.lights.lights.len(), p.g.tab.camera.len());
        let (first, active, host) = (count(&p), p.g.staged.up, (p.g.host_cam, p.g.host_input));
        assert!(active.is_some(), "{link}: no mission is up");
        p.run(2.0);
        for tries in 1..=2 {
            assert!(control::restart(&mut p.g));
            assert_eq!(p.g.staged.up, active, "{link}: try {tries}");
            p.run(1.0);
            assert_eq!(count(&p), first, "{link}: try {tries}: (guards, mates, civilians, characters, doors, statics, lights, cameras)");
            assert_eq!((p.g.host_cam, p.g.host_input), host, "{link}: try {tries}");
        }
    }
}

/// THE RULE: the switches a quiet run took are its own: what stood under them is what the next try finds under it.
/// A FAILURE MEANS: a restart takes the run's own switches for the player's, and they stay on when the run is over.
#[test]
fn a_restart_keeps_what_stood_under_the_runs_own_switches() {
    let mut p = Play::mission("crossing.json", "?levelFile=crossing.json&quiet&squad=1", &Opts { seed: 1, ..Default::default() });
    let was = p.g.quiet.was.expect("a run with a partner holds the switches");
    assert!(game::savestate::control::restart(&mut p.g));
    let again = p.g.quiet.was.expect("the run again holds them");
    assert_eq!((again.pairup, again.silent, again.silent_start), (was.pairup, was.silent, was.silent_start), "what stood under the run");
}
