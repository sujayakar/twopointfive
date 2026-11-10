//! A RESTART EQUALS A FRESH STAGING, WITH NOTHING OF THE TRY BEFORE.
mod world_ask;
use game_start::play::{nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use world_ask::{the_floor, DarkAsk, FurnAsk};

const CLOCK: f64 = 30.0;
fn office(link: Option<&str>) -> Play {
    let o = Opts { seed: 1, ..Default::default() };
    let mut p = match link { Some(l) => Play::mission("office.json", l, &o), None => Play::level("office.json", false, &o) };
    p.g.god_mode = true;
    p
}
/// a game restarted as the page's cards and keys restart it, at a clock and a die both arms share, and a quarter of a second of quiet
fn restart(p: &mut Play) {
    p.g.time = CLOCK;
    p.g.dice = Some(game::math_dice::sim_dice_new(9.0));
    game::savestate::control::restart(&mut p.g);
    p.g.god_mode = true;
    p.run(0.25);
}
/// Everything a try leaves: rounds into the floor, a grenade, the mains pulled, a door stood open, a fixture broken, a pane starred, a doorway marked, the floor put up a step.
fn wreck(p: &mut Play) {
    p.sam_takes_his_rifle();
    for k in 0..4 { let at = p.sam_at(); p.sam_fires_at([at[0] + 2.0 + nudge(1, k), at[1] + nudge(2, k)], 0.05); }
    let at = p.sam_at();
    p.frag_lands([at[0] + 3.0, at[1]]);
    p.mains_pulled();
    let d = p.g.doors.list[0];
    let turn = game::doors::door_swing(&mut p.g, d) * 1.4;
    p.g.dr[d].leaf.latched = false;
    p.g.dr[d].leaf.angle = turn;
    game::tactics::mark_danger(&mut p.g, d, game::teams::Team::Guard, game::tactics::DangerWhy::Rounds, Some("Kowalski"));
    let pane = p.g.col.boxes.clone().into_iter().find(|&b| game::glass::is_pane(&mut p.g, Some(b))).expect("a pane");
    let c = p.g.tab.boxes[pane].c;
    game::glass::crack_near(&mut p.g, c, 0.5, 0.0);
    game::guards::raise_escalation(&mut p.g, 1.0, None, None, None);
    p.run(0.5);
}

/// THE RULE: the game after a restart is the game a fresh staging gives: the men where they start and as they start, every door, every lamp, the mains, every static, every prop, the walk grid, nothing thrown, the alarm down. CONTROL: the other arm is the same restart of a game nobody has touched, at the same clock on the same die; and the wreck did change the floor.
/// A FAILURE MEANS: something of the try before lives on. The part named says what.
#[test]
fn a_restart_after_play_equals_a_restart_of_an_untouched_game() {
    for link in [None, Some("?levelFile=office.json&rescue&squad=0")] {
        let mut p = office(link);
        restart(&mut p);
        let fresh = the_floor(&mut p);
        drop(p);
        let mut p = office(link);
        wreck(&mut p);
        let wrecked = the_floor(&mut p);
        assert!(wrecked.iter().zip(&fresh).filter(|(a, b)| a.1 != b.1).count() >= 4, "{link:?}: the staging: the wreck hardly touched the floor");
        restart(&mut p);
        let again = the_floor(&mut p);
        for ((name, a), (_, b)) in fresh.iter().zip(&again) { assert!(a == b, "{link:?}: the {name} after a restart are not as a fresh staging has them:\n fresh  {a}\n played {b}"); }
    }
}

/// THE RULE: a load is not a restart: a game kept in the dark, far from the start, comes back dark with its man where he stood, and the next restart still gives a clean floor.
/// A FAILURE MEANS: F9 clears the wreck the player saved in, or a restart after a load keeps it.
#[test]
fn a_load_is_not_a_restart() {
    let mut q = office(None);
    restart(&mut q);
    let fresh = the_floor(&mut q);
    drop(q);
    let mut p = office(None);
    p.mains_pulled();
    p.sam_put([30.0, 11.1]);
    p.run(0.5);
    let kept = p.keep();
    let at = p.sam_at();
    restart(&mut p);
    assert!(!p.g.light_home.mains.active, "a restart left the mains down");
    p.put_back(&kept);
    p.frame();
    assert!(p.g.light_home.mains.active && game_start::play::dist(p.sam_at(), at) < 0.05, "the load: the mains down {}, Sam {:?} kept at {at:?}", p.g.light_home.mains.active, p.sam_at());
    restart(&mut p);
    let again = the_floor(&mut p);
    for ((name, a), (_, b)) in fresh.iter().zip(&again) { assert!(a == b, "the {name} after a restart after a load are not as a fresh staging has them:\n fresh {a}\n here  {b}"); }
}

/// A man on a short walk in a dark hall; Sam six metres from him where nobody sees him.
fn hall(s: u32) -> Play {
    let mut sc = Scene::new("whistled");
    sc.room("hall", 6.0, 9.0, 30.0, 17.0).door("west", Axis::Z, 6.0, 15.4);
    sc.patrol("walk", &[[17.75, 13.0], [18.25, 13.0]], 30.0).sam(12.0, 13.0);
    Play::scene(&sc, &Opts { seed: s, ..Default::default() })
}

/// THE RULE: a man answers a whistle once. A FAILURE MEANS: the whistle no longer draws anybody, or draws him again a second later.
#[test]
fn a_whistle_is_answered_once() {
    for s in [1u32, 2] {
        let mut p = hall(s);
        p.run(0.5);
        p.key_down("KeyV");
        p.frame();
        p.key_up("KeyV");
        assert_eq!(p.g.stats.whistle.blown, 1, "seed {s}: the key blew no whistle");
        p.run(1.0);
        assert!(p.g.stats.whistle.resolved == 1 && p.g.stats.whistle.answered == 1, "seed {s}: resolved {}, answered {}", p.g.stats.whistle.resolved, p.g.stats.whistle.answered);
    }
}

/// THE RULE (a save is the whole game with its queue): a whistle in the air at the save is in the air after the load, and it is resolved once and answered once there, not twice. CONTROL: a second whistle blown after the load does count, so the count can move.
/// A FAILURE MEANS: a load drops what was in the air, or a man answers one whistle twice.
#[test]
fn a_whistle_in_the_air_at_the_save_is_resolved_once_after_the_load() {
    let whistle = |p: &mut Play| { p.key_down("KeyV"); p.frame(); p.key_up("KeyV"); };
    let mut p = hall(1);
    p.g.god_mode = true;
    p.run(0.5);
    whistle(&mut p);
    let kept = p.keep();
    p.run(1.0);
    assert_eq!(p.g.stats.whistle.answered, 1, "the staging: he answered it");
    p.put_back(&kept);
    assert!(p.g.events.iter().any(|e| e.kind == game::director::EventKind::Whistle), "the load dropped the whistle from the air");
    p.run(2.0);
    assert!(p.g.stats.whistle.resolved == 1 && p.g.stats.whistle.answered == 1, "after the load: resolved {}, answered {}", p.g.stats.whistle.resolved, p.g.stats.whistle.answered);
    whistle(&mut p);
    p.run(0.5);
    assert_eq!(p.g.stats.whistle.resolved, 2, "a second whistle after the load was not resolved: blown {}, resolved {}", p.g.stats.whistle.blown, p.g.stats.whistle.resolved);
}
