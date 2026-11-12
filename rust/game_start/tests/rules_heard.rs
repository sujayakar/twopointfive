//! WHAT A MAN'S EAR TAKES IN: how a sound reaches him round what stands in the way, and whose noise draws him. Each test stands a few men and a wall or a pane, asks how a
//! sound comes, or makes the noise and asks what he did, and holds the answer beside a control that differs in the one thing the rule is about.
mod world_ask;
use game::director::EventKind;
use game::game::GuardState;
use game::glasspen::PaneType;
use game::ids::GuardId;
use game_start::play::{nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use world_ask::{GlassAsk, WorldAsk, YardAsk};

const SEEDS: [u32; 3] = [1, 2, 3];
fn seed(seed: u32) -> Opts { Opts { seed, ..Default::default() } }
/// How a sound at `src` reaches an ear at `ear`, as every man's ear asks it.
fn comes(p: &mut Play, src: [f64; 3], ear: [f64; 3]) -> game::scene_collision::Propagation { game::scene_collision::sound_paths_propagate(&mut p.g, src, ear, Some(false)) }

/// THE RULE: a sound with a clear line that passes close by a jamb reaches an ear a touch duller than the same sound in open air, and it is still the straight line.
/// A FAILURE MEANS: nothing that stands beside a sound's way changes how it sounds, so a shot in a doorway sounds as a shot in the open.
#[test]
fn a_sound_that_passes_a_jamb_is_duller() {
    for s in SEEDS {
        let z = 13.0 + 0.1 * nudge(s, 0);
        let hall = |stubs: &[(f64, f64)]| {
            let mut sc = Scene::new("jambs");
            sc.room("hall", 4.0, 6.0, 30.0, 20.0).door("gate", Axis::X, 20.0, 20.0).sentry("far", [28.25, 7.25], [30.0, 7.25]);
            for &(a, b) in stubs { sc.wall_z(10.6, z + a, z + b); }
            Play::scene(&sc, &seed(s))
        };
        let (src, ear) = ([10.0, 1.0, z], [18.0, 1.5, z]);
        let mut open = hall(&[]);
        open.run(0.1);
        let air = comes(&mut open, src, ear);
        assert!(air.direct && air.muffle < 0.02, "seed {s}: CONTROL: in open air direct {} and {:.2} dull", air.direct, air.muffle);
        drop(open);
        let mut p = hall(&[(0.3, 3.0)]);
        p.run(0.1);
        let one = comes(&mut p, src, ear);
        assert!(one.direct && one.muffle > 0.05, "seed {s}: a jamb on one side: direct {} and {:.2} dull", one.direct, one.muffle);
        drop(p);
        let mut p = hall(&[(0.3, 3.0), (-3.0, -0.3)]);
        p.run(0.1);
        let two = comes(&mut p, src, ear);
        assert!(two.direct && two.muffle > one.muffle + 0.05, "seed {s}: a jamb on each side {:.2} dull, on one {:.2}", two.muffle, one.muffle);
    }
}

/// THE RULE: a sound passes a hole in a pane as it passes air, and is stopped by the whole glass beside the hole.
/// A FAILURE MEANS: a shot-out window keeps the room behind it as quiet as a whole one.
#[test]
fn a_sound_passes_a_hole_in_a_pane() {
    for s in [913u32, 914] {
        let mut p = Play::level("office.json", false, &Opts { seed: s, dress: true, ..Default::default() });
        p.g.god_mode = true;
        p.g.k.chaos.def = 0.5;
        p.run(0.08);
        for (k, m) in p.g.guards.clone().into_iter().enumerate() { p.parked_far(m, k); }
        let bi = p.pane_at(7.5, 10.0);
        p.pane_is(bi, PaneType::Annealed);
        p.door_shut_locked("conference", false);
        let (x, y) = (7.6, 2.25);
        let across = |p: &mut Play, x: f64, y: f64| comes(p, [x, y, 10.8], [x, y, 9.2]);
        let whole = across(&mut p, x, y);
        assert!(!whole.direct, "seed {s}: CONTROL: the whole pane lets the sound straight through");
        game::glass::glass_impact(&mut p.g, bi, [x, y, 10.0], Some([0.0, 0.0, -1.0]), 1.0, 1.0, None, None, None);
        p.run(0.7);
        assert!(game::glassbreak::glass_break_open_at(&p.g, bi, [x, y, 10.0]), "seed {s}: the staging: the blow opened nothing there");
        let through = across(&mut p, x, y);
        assert!(through.direct && through.carry > 0.8, "seed {s}: through the hole: direct {}, carry {:.2}", through.direct, through.carry);
        assert!(!game::glassbreak::glass_break_open_at(&p.g, bi, [7.2, 1.0, 10.0]), "seed {s}: the staging: the low corner of the pane has gone too");
        let beside = across(&mut p, 7.2, 1.0);
        assert!(!beside.direct, "seed {s}: the whole glass low in the same pane lets the sound straight through");
    }
}

/// Two halves of a long building with a wall between them and a door standing open well off the straight line. Sam in the west half, a sentry in the east, `metres` from him in a straight line.
fn the_long_way(s: u32, metres: f64) -> (Play, GuardId, [f64; 2]) {
    let z = 11.5 + 0.2 * nudge(s, 0);
    let sam = [5.0, z];
    let mut sc = Scene::new("the-long-way");
    sc.room("west", 2.0, 8.0, 20.0, 20.0).room("east", 20.0, 8.0, 38.0, 20.0).door("through", Axis::Z, 20.0, 16.4).door("gate", Axis::X, 8.0, 10.0);
    sc.sentry("post", [sam[0] + metres, z], [38.0, z]).sam(sam[0], sam[1]);
    let mut p = Play::scene(&sc, &seed(s));
    p.sam_unhurt();
    p.door_stands_open("through");
    p.run(0.2);
    let m = p.guard("post");
    (p, m, sam)
}

/// THE RULE: a sound fired further off than the search for a way round goes, behind walls, comes as through the walls: no way, a quarter of it or so let through, longer than the straight line. The same sound nearer, with the same wall and door, comes round by the door.
/// (A player hears the difference in how dull a far shot is, and a man who hears such a shot puts the shooter further from where he stood.)
/// A FAILURE MEANS: every noise on the level pays for a search, and comes round the building as clearly as a noise next door.
#[test]
fn a_sound_too_far_round_comes_through_the_walls() {
    for s in SEEDS {
        for (metres, way) in [(24.0, true), (30.0, false)] {
            let (mut p, m, sam) = the_long_way(s, metres);
            let at = p.at(m);
            let r = comes(&mut p, [sam[0], 1.0, sam[1]], [at[0], 1.5, at[1]]);
            assert!(!r.direct && r.routed == way, "seed {s}: {metres} m in a straight line: direct {}, a way {}", r.direct, r.routed);
            if way {
                assert!(r.carry > 0.5 && r.muffle < 0.5 && (r.via[2] - sam[1]).abs() > 1.0, "seed {s}: {metres} m round by the open door: carry {:.2}, dull {:.2}, comes through ({:.1}, {:.1})", r.carry, r.muffle, r.via[0], r.via[2]);
            } else {
                assert!(r.carry < 0.3 && r.muffle > 0.9 && r.path_len > metres * 1.3, "seed {s}: {metres} m through the walls: carry {:.2}, dull {:.2}, {:.1} m", r.carry, r.muffle, r.path_len);
            }
        }
    }
}

/// A sentry in a room with a door to the yard, and beside it a room with no way in at all.
fn beside_a_sealed_room(s: u32) -> (Play, GuardId, f64) {
    let z = 13.0 + 0.3 * nudge(s, 0);
    let mut sc = Scene::new("sealed");
    sc.room("his", 8.0, 8.0, 18.0, 18.0).room("sealed", 18.0, 8.0, 28.0, 18.0).door("gate", Axis::X, 8.0, 12.0).bulb(12.0, 13.0);
    sc.sentry("post", [16.25, z], [8.0, z]);
    let mut p = Play::scene(&sc, &seed(s));
    p.sam_unhurt();
    p.run(0.5);
    let m = p.guard("post");
    (p, m, z)
}

/// THE RULE: a footfall in a room with no way out of it does not reach a man three metres off through the wall; the same footfall as near on his own side does.
/// A FAILURE MEANS: walls that have no way round them pass a sound as if a door stood open.
#[test]
fn a_room_with_no_way_round_keeps_its_noises() {
    for s in SEEDS {
        let (mut p, m, z) = beside_a_sealed_room(s);
        let at = p.at(m);
        let inside = [at[0] + 3.0, z];
        let n = p.small_heard(m);
        p.footfall_at(inside, 0.6);
        p.run(0.1);
        assert_eq!(p.small_heard(m), n, "seed {s}: he heard a footfall in the sealed room");
        p.footfall_at([at[0] - 3.0, z], 0.6);
        p.run(0.1);
        assert!(p.small_heard(m) > n, "seed {s}: CONTROL: the same footfall as near on his own side did not reach him");
        let r = comes(&mut p, [inside[0], 1.0, inside[1]], [at[0], 1.5, at[1]]);
        assert!(!r.direct && !r.routed && r.carry < 0.3 && r.path_len > 4.0, "seed {s}: through the wall: direct {}, a way {}, carry {:.2}, {:.1} m", r.direct, r.routed, r.carry, r.path_len);
    }
}

/// A sentry; a colleague who walks up and down right by him; nobody else on the floor near.
fn walked_past(s: u32) -> (Play, GuardId, GuardId) {
    let z = 11.0 + 0.3 * nudge(s, 0);
    let mut sc = Scene::new("colleague");
    sc.room("hall", 4.0, 8.0, 30.0, 20.0).door("gate", Axis::X, 20.0, 26.0).bulb(16.0, 14.0);
    sc.sentry("post", [16.25, z], [30.0, z]);
    sc.patrol("round", &[[10.25, z + 2.0], [22.25, z + 2.0]], 0.5);
    let mut p = Play::scene(&sc, &seed(s));
    p.sam_unhurt();
    p.run(0.2);
    let (m, o) = (p.guard("post"), p.guard("round"));
    (p, m, o)
}

/// A noise of a colleague's own, by his id, as the game files it.
fn his_noise(p: &mut Play, o: GuardId, kind: EventKind, at: [f64; 2], level: f64) {
    let (time, serial, who) = (p.g.time, p.g.serial(), p.id_of(o));
    p.g.events.push(game::game::GameEvent { kind, pos: [at[0], 0.0, at[1]], time, loud: kind == EventKind::Door, level: Some(level), who: Some(who), serial, ..game::game::GameEvent::default() });
}

/// THE RULE: a colleague's footfall draws no man's ear; Sam's as near does.
/// A FAILURE MEANS: guards go to look at each other all shift, and a floor of them never settles.
#[test]
fn a_colleagues_footfall_does_not_draw_him() {
    for s in SEEDS {
        let (mut p, m, o) = walked_past(s);
        let start = p.at(o);
        if let Some(t) = p.always(6.0, |p| p.small_heard(m) == 0.0 && p.state(m) == GuardState::Patrol && p.awareness(m) < 0.05) { panic!("seed {s}: at {t:.2} s a colleague walking past has him: {} small noises, {:?}, awareness {:.2}", p.small_heard(m), p.state(m), p.awareness(m)); }
        assert!(game_start::play::dist(start, p.at(o)) > 1.0, "seed {s}: the staging: the colleague did not walk");
        let at = p.ahead(m, 3.0, 0.0);
        his_noise(&mut p, o, EventKind::Step, at, 0.8);
        p.run(0.2);
        assert!(p.small_heard(m) == 0.0 && p.awareness(m) < 0.05, "seed {s}: a colleague's footfall has him: {} small noises, awareness {:.2}", p.small_heard(m), p.awareness(m));
        p.footfall_at(at, 0.8);
        p.run(0.2);
        assert!(p.small_heard(m) > 0.0 && p.awareness(m) > 0.05, "seed {s}: CONTROL: a footfall of Sam's as near did not reach him");
    }
}

/// THE RULE: a door a colleague bangs draws no man's ear; one of Sam's as near does.
/// A FAILURE MEANS: a floor's own comings and goings set it looking for an intruder.
#[test]
fn a_colleagues_door_does_not_draw_him() {
    for s in SEEDS {
        let (mut p, m, o) = walked_past(s);
        let at = p.ahead(m, 4.0, 0.0);
        his_noise(&mut p, o, EventKind::Door, at, 1.0);
        p.run(0.2);
        assert!(p.small_heard(m) == 0.0 && p.awareness(m) < 0.05, "seed {s}: a colleague's door has him: {} small noises, awareness {:.2}", p.small_heard(m), p.awareness(m));
        p.slam_at(at);
        p.run(0.2);
        assert!(p.small_heard(m) > 0.0 && p.awareness(m) > 0.1, "seed {s}: CONTROL: a door of Sam's as near did not reach him: awareness {:.2}", p.awareness(m));
    }
}

/// THE RULE: Sam's rifle is no alarm to a man of his own side: a guard hears it, and the man of ours beside him does not count it. (A man under an order has his ears in the room fight; this is the man on no order.)
/// A FAILURE MEANS: Sam's own men go alert and say so every time he fires.
#[test]
fn sams_rifle_is_no_alarm_to_a_man_of_his_side() {
    for s in SEEDS {
        let (mut p, m, _) = walked_past(s);
        let at = p.at(m);
        let ours = game::guards::spawn_unit(&mut p.g, &game::guards::UnitOpts { team: game::teams::Team::Ours, pos: [at[0] + 2.0, 0.0, at[1]], yaw: Some(0.0), weapon: Some(game::weapons::WeaponId::Fn2000), light: Some(false), ..Default::default() }).expect("a man stood");
        p.run(0.2);
        assert!(p.g.gd[ours].script.is_none(), "seed {s}: the staging: the man of ours is on a script, and his ears are kept elsewhere");
        p.sam_fires_at([2.6, 2.6], 0.2);
        p.run(0.2);
        assert!(p.shots_heard(m) > 0.0, "seed {s}: CONTROL: the guard did not hear the rifle");
        assert_eq!(p.shots_heard(ours), 0.0, "seed {s}: the man of ours took Sam's rifle for a shot at him: {:?}, awareness {:.2}", p.state(ours), p.awareness(ours));
    }
}
