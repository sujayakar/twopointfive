//! A QUIET MISSION TO ITS END, AND A SAVE OF ONE.
mod stealth_ask;
use game::game::{InteractableKind, Verb};
use game::game::MissionStage as Stage;
use game::savestate::control;
use game_start::play::{dist, Opts, Play};
use stealth_ask::StealthAsk;

fn bazaar(pin: u32) -> Play {
    let mut p = Play::mission("bazaar.json", &format!("?levelFile=bazaar.json&quiet={pin}"), &Opts { seed: 4, ..Default::default() });
    p.sam_unhurt();
    p.g.player_invisible = true;
    p
}

/// THE RULE: a quiet mission plays to its end by its way out, each of them: the room reached, the thing taken, not done a stride short of that exit's line, done a stride past it, and the
/// last word names that way out.
/// A FAILURE MEANS: a way out of a quiet level does not end the mission, or ends it early, or the player is told the name of another.
#[test]
fn a_quiet_mission_plays_to_its_end_by_each_way_out() {
    let exits = bazaar(11).g.level.errand.exits.clone();
    assert!(exits.len() >= 2, "the bazaar has more than one way out: {}", exits.len());
    for k in 0..exits.len() {
        let mut p = bazaar(11 + k as u32);
        let spot = p.g.level.errand.clone();
        let exit = p.g.level.errand.exits[k].clone();
        p.sam_put(p.free_near([spot.front[0], spot.front[2]], 1.2));
        p.until(3.0, |p| p.mission_stage() != Stage::Infiltrate).unwrap_or_else(|| panic!("exit {k}: at the spot and the mission does not know it"));
        p.cursor_on(spot.front);
        p.run(0.2);
        assert!(p.row_taken(InteractableKind::Objective, Verb::Drive), "exit {k}: nothing to take at the spot");
        p.until(12.0, |p| p.g.mission.pulled).unwrap_or_else(|| panic!("exit {k}: 12 s and the thing is not taken"));
        p.sam_put(p.free_near([exit.x - exit.out[0], exit.z - exit.out[1]], 0.8));
        p.run(0.5);
        assert!(p.objective().contains(&exit.name), "exit {k}: the objective names '{}': {}", exit.name, p.objective());
        assert_ne!(p.mission_stage(), Stage::Done, "exit {k}: done a stride short of the line");
        let past = game::quiet::exit_point(&exit, Some(0.9));
        p.sam_put([past[0], past[2]]);
        p.until(2.0, |p| p.mission_stage() == Stage::Done).unwrap_or_else(|| panic!("exit {k}: past '{}' and not done", exit.name));
    }
}

/// THE RULE: the office's own night is the same errand: the room reached, the drive taken from its rack, not done a stride short of the fire exit's line, done a stride past it, each said by its name.
/// A FAILURE MEANS: the solo night can't be finished, ends early, or tells the player of another room, another thing or another way out than its level's.
#[test]
fn the_offices_night_plays_to_its_end() {
    let mut p = Play::level("office.json", false, &Opts { seed: 5, ..Default::default() });
    p.g.player_invisible = true;
    let e = p.g.level.errand.clone();
    assert_eq!((e.place.as_str(), e.thing.as_str(), e.holder.as_str(), e.outs().as_str()), ("the server room", "the drive", "rack 6", "the fire exit"));
    p.run(0.2);
    assert_eq!(p.objective(), "objective ▸ reach the server room");
    p.sam_put(p.free_near([e.front[0], e.front[2] + 0.6], 0.6));
    p.until(3.0, |p| p.mission_stage() != Stage::Infiltrate).expect("at the rack and the mission does not know it");
    let told = |p: &Play, what: &str| assert!(p.g.chronicle.events.iter().any(|e| format!("{e:?}").contains(what)), "the run's record does not say '{what}': {:?}", p.g.chronicle.events);
    told(&p, "reached the server room");
    assert!(p.objective().starts_with("objective ▸ take the drive from rack 6"), "{}", p.objective());
    p.cursor_on(e.front);
    p.run(0.2);
    assert!(p.row_taken(InteractableKind::Objective, Verb::Drive), "nothing to take at the rack");
    p.until(12.0, |p| p.g.mission.pulled).expect("12 s and the drive is not taken");
    told(&p, "the drive is out of rack 6");
    assert_eq!(p.objective(), "objective ▸ get out through the fire exit");
    let exit = &e.exits[0];
    p.sam_put(p.free_near([exit.x - exit.out[0], exit.z - exit.out[1]], 0.8));
    p.run(0.5);
    assert_ne!(p.mission_stage(), Stage::Done, "done a stride short of the line");
    let past = game::quiet::exit_point(exit, Some(0.9));
    p.sam_put([past[0], past[2]]);
    p.until(2.0, |p| p.mission_stage() == Stage::Done).expect("past the fire exit and not done");
    told(&p, "out through the fire exit");
}

/// THE RULE: walking out before the thing is taken ends nothing. (The control of the rule above.)
/// A FAILURE MEANS: the mission can be finished by leaving.
#[test]
fn walking_out_before_the_thing_is_taken_ends_nothing() {
    for pin in [5, 6, 7] {
        let mut p = bazaar(pin);
        let exit = p.g.level.errand.exits[0].clone();
        let past = game::quiet::exit_point(&exit, Some(0.9));
        p.sam_put([past[0], past[2]]);
        p.run(1.5);
        assert_ne!(p.mission_stage(), Stage::Done, "pin {pin}: past '{}' with nothing taken", exit.name);
    }
}

/// THE RULE: a save carries the deal: a load puts the same start, spot and patrol set and every man back, over a run played on since. (A restart ends the run's saves: `that checkpoint is gone`.)
/// A FAILURE MEANS: a load on a quiet mission stands the player on another night than the one he saved.
#[test]
fn a_save_on_a_quiet_mission_carries_the_deal() {
    for pin in [1, 2, 3] {
        let mut p = bazaar(pin);
        let mut kept: Vec<String> = Vec::new();
        let back = |p: &mut Play, kept: &mut Vec<String>| kept.extend(p.told.drain(..).filter_map(|t| t.kept));
        for _ in 0..60 { p.frame(); back(&mut p, &mut kept); }
        p.key_down("F5");
        p.frame();
        back(&mut p, &mut kept);
        p.key_up("F5");
        assert_eq!(kept.len(), 1, "pin {pin}: the key saves on a calm quiet run");
        let (deal, sam, men) = (p.quiet_deal(), p.sam_at(), p.g.guards.iter().map(|&m| p.at(m)).collect::<Vec<_>>());
        p.key_down("KeyW");
        for _ in 0..120 { p.frame(); back(&mut p, &mut kept); }
        p.key_up("KeyW");
        assert!(dist(p.sam_at(), sam) > 0.5, "pin {pin}: CONTROL: he walked away from the save");
        let key = kept[0].clone();
        assert_eq!(control::ask_load(&mut p.g, &key), None, "pin {pin}: the load was refused");
        p.frame();
        back(&mut p, &mut kept);
        assert_eq!(p.quiet_deal(), deal, "pin {pin}: the deal");
        assert!(dist(p.sam_at(), sam) < 0.05, "pin {pin}: Sam");
        let now: Vec<[f64; 2]> = p.g.guards.iter().map(|&m| p.at(m)).collect();
        assert!(now.len() == men.len() && now.iter().zip(&men).all(|(a, b)| dist(*a, *b) < 0.1), "pin {pin}: the men: {now:?} against {men:?}");
    }
}
