//! A QUIET MISSION TO ITS END, AND A SAVE OF ONE.
mod stealth_ask;
use game::game::{Game, InteractableKind, Verb};
use game::game::MissionStage as Stage;
use game::savestate::control::{self, Do};
use game_start::play::{dist, Opts, Play};
use stealth_ask::StealthAsk;
use std::collections::BTreeMap;

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
    let exits = game::quiet::quiet_mission_exits(&bazaar(11).g);
    assert!(exits.len() >= 2, "the bazaar has more than one way out: {}", exits.len());
    for k in 0..exits.len() {
        let mut p = bazaar(11 + k as u32);
        let spot = game::quiet::quiet_mission_spot(&p.g).expect("a dealt spot");
        let exit = game::quiet::quiet_mission_exits(&p.g)[k].clone();
        p.sam_put(p.free_near([spot.front[0], spot.front[2]], 1.2));
        p.until(3.0, |p| p.mission_stage() != Stage::Infiltrate).unwrap_or_else(|| panic!("exit {k}: at the spot and the mission does not know it"));
        p.cursor_on(spot.front);
        p.run(0.2);
        assert!(p.row_taken(InteractableKind::Objective, Verb::Drive), "exit {k}: nothing to take at the spot");
        p.until(12.0, |p| p.g.mission.pulled).unwrap_or_else(|| panic!("exit {k}: 12 s and the thing is not taken"));
        p.sam_put(p.free_near([exit.x - exit.out[0], exit.z - exit.out[1]], 0.8));
        p.run(0.5);
        assert_ne!(p.mission_stage(), Stage::Done, "exit {k}: done a stride short of the line");
        let past = game::quiet::exit_point(&exit, Some(0.9));
        p.sam_put([past[0], past[2]]);
        p.until(2.0, |p| p.mission_stage() == Stage::Done).unwrap_or_else(|| panic!("exit {k}: past '{}' and not done", exit.name));
        assert!(p.log().iter().any(|l| l.contains(&exit.name)), "exit {k}: the last word names '{}': {:?}", exit.name, p.log());
    }
}

/// THE RULE: walking out before the thing is taken ends nothing. (The control of the rule above.)
/// A FAILURE MEANS: the mission can be finished by leaving.
#[test]
fn walking_out_before_the_thing_is_taken_ends_nothing() {
    for pin in [5, 6, 7] {
        let mut p = bazaar(pin);
        let exit = game::quiet::quiet_mission_exits(&p.g)[0].clone();
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
    fn block(slot: usize) -> Vec<u8> { unsafe { std::slice::from_raw_parts(sim::state::addr(slot) as *const u8, std::mem::size_of::<sim::state::State>()).to_vec() } }
    for pin in [1, 2, 3] {
        let mut p = bazaar(pin);
        let mut kept: BTreeMap<String, (Box<Game>, Vec<u8>)> = BTreeMap::new();
        let back = |p: &mut Play, kept: &mut BTreeMap<String, (Box<Game>, Vec<u8>)>| match control::tick(&mut p.g, 1.0 / 60.0) {
            Some(Do::Keep { key, .. }) => { let b = block(p.g.sim); kept.insert(key, (p.g.clone(), b)); }
            Some(Do::Back { key }) => {
                let was = control::abandoned(&p.g);
                let (g, b) = &kept[&key];
                unsafe { std::ptr::copy_nonoverlapping(b.as_ptr(), sim::state::addr(p.g.sim) as *mut u8, b.len()); }
                p.g = g.clone();
                game::scene_collision::unstand(&mut p.g);
                game::scene_collision::stands(&mut p.g);
                control::put_back(&mut p.g, &key, was);
            }
            _ => {}
        };
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
        let key = kept.keys().next().unwrap().clone();
        assert_eq!(control::ask_load(&mut p.g, &key), None, "pin {pin}: the load was refused");
        p.frame();
        back(&mut p, &mut kept);
        assert_eq!(p.quiet_deal(), deal, "pin {pin}: the deal");
        assert!(dist(p.sam_at(), sam) < 0.05, "pin {pin}: Sam");
        let now: Vec<[f64; 2]> = p.g.guards.iter().map(|&m| p.at(m)).collect();
        assert!(now.len() == men.len() && now.iter().zip(&men).all(|(a, b)| dist(*a, *b) < 0.1), "pin {pin}: the men: {now:?} against {men:?}");
    }
}
