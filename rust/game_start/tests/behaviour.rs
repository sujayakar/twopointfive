//! BEHAVIOUR TESTS: each stands a scene of a few lines and holds one rule of the game, beside a control that differs in the one thing the rule
//! is about. README.md beside this crate says how to write one. A failure's message says what the game did; the comment over a test says what a
//! failure would mean.
//!
//! EACH SEED IS ANOTHER SCENE, not only another die: in scenes this small the die is hardly drawn (a night draws it five times in fifteen
//! seconds), so a test that changed the seed alone would play one run three times. `nudge` moves a mark, a range, a door or a landing point by
//! the seed, and the rule is held over the three.
use game::game::GuardState;
use game::orders::Who;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 3] = [1, 2, 3];
fn seed(seed: u32) -> Opts { Opts { seed, ..Default::default() } }
const NORTH: f64 = std::f64::consts::PI;   // (a yaw: 0 looks south, along +z)

/// A room 10 m by 8 m with one door in its south wall and a bulb in its middle, a sentry at its west end looking east, and Sam standing about
/// 6 m in front of him (5.75 to 6.75 m, up to 75 cm off his line, by the seed).
fn watched_room(lit: bool, seed: u32) -> Scene {
    let mut s = Scene::new("watched-room");
    s.room("room", 10.0, 10.0, 20.0, 18.0).door("south", Axis::X, 18.0, 14.0);
    if lit { s.bulb(15.0, 14.0); } else { s.bulb_off(15.0, 14.0); }
    s.sentry("post", [11.5, 14.25], [20.0, 14.25]).sam(17.75 + 0.5 * nudge(seed, 0), 14.25 + 0.75 * nudge(seed, 1));
    s
}

/// where the two defenders of `two_rooms` are posted, by the seed (each within half a metre of his room's north-west and north-east quarter)
fn west_post(seed: u32) -> [f64; 2] { [12.25 + 0.5 * nudge(seed, 2), 12.25 + 0.5 * nudge(seed, 3)] }
fn east_post(seed: u32) -> [f64; 2] { [24.25 + 0.5 * nudge(seed, 4), 12.25 + 0.5 * nudge(seed, 5)] }
/// Two rooms side by side, each with a door to the yard south of them and one door between them, a bulb in each; the team in the yard under
/// the moon, facing the building; a defender posted in each room with his gun on its yard door. The west room's yard door stands up to a metre
/// either way along its wall, by the seed.
fn two_rooms(seed: u32) -> Scene {
    let mut s = Scene::new("two-rooms");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("west-yard", Axis::X, 16.0, 13.4 + nudge(seed, 6)).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    s.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    s.team([14.0, 21.0], NORTH, &[[13.0, 22.0], [15.0, 22.0], [13.0, 23.0], [15.0, 23.0]]);
    s.defender("west", west_post(seed), "west-yard").defender("east", east_post(seed), "east-yard");
    s
}

/// THE RULE: a calm guard who has an armed man 6 m in front of him in light grows suspicious, then alert, believes the man is where he stood,
/// and calls the contact once; with the lamp off, at the same range, he never notices.
/// A FAILURE MEANS: sight no longer runs on the light the man stands in (the meter, the awareness that fills from it, the two thresholds), or
/// the alert no longer leaves a fix on the man.
#[test]
fn a_calm_guard_sees_a_man_in_light_and_not_in_the_dark() {
    for s in SEEDS {
        let mut p = Play::scene(&watched_room(true, s), &seed(s));
        let (post, sam) = (p.guard("post"), p.sam_at());
        let range = dist(p.at(post), sam);
        assert!(range > 5.5 && range < 7.0, "seed {s}: the scene: the sentry stands about 6 m from Sam ({range:.2} m)");
        let lit = p.until(2.0, |p| p.sam_light() > 0.5).unwrap_or_else(|| panic!("seed {s}: under the lamp Sam's light never passed 0.5 (it is {:.2}): the test has no lit man to show", p.sam_light()));
        assert_eq!(p.state(post), GuardState::Patrol, "seed {s}: the sentry is still calm when the meter first reads Sam as lit ({lit:.2} s)");
        let sus = p.until(3.0, |p| p.state(post) != GuardState::Patrol).unwrap_or_else(|| panic!("seed {s}: 3 s with a lit man {range:.1} m in front of him and the sentry is calm (awareness {:.2})", p.awareness(post)));
        assert_eq!(p.state(post), GuardState::Suspicious, "seed {s}: he is suspicious before he is alert");
        let alert = p.until(4.0, |p| p.state(post) == GuardState::Alert).unwrap_or_else(|| panic!("seed {s}: suspicious after {sus:.2} s and never alert (awareness {:.2})", p.awareness(post)));
        let fix = p.fix(post).unwrap_or_else(|| panic!("seed {s}: alert and no place he believes the man is"));
        assert!(dist(fix, sam) < 0.5, "seed {s}: alert after {alert:.2} s more, and his fix ({:.1}, {:.1}) is not where the man stood ({:.1}, {:.1})", fix[0], fix[1], sam[0], sam[1]);
        // he calls it ONCE, and not on the frame he turns alert: the start he was given holds the call back about half a second
        let t_alert = p.time();
        p.run(2.5);
        let calls: Vec<f64> = p.said.iter().filter(|l| l.who == post && l.text.to_uppercase().contains("CONTACT")).map(|l| l.t - t_alert).collect();
        assert!(calls.len() == 1 && calls[0] > 0.0 && calls[0] < 1.0, "seed {s}: he calls the contact once, within a second of turning alert: he called it {:?} s after, and said {:?}", calls, p.lines(post));
        drop(p);

        let mut p = Play::scene(&watched_room(false, s), &seed(s));
        let post = p.guard("post");
        p.run(1.5);   // (his torch sweeps the room as he settles: the meter may read it for a moment)
        if let Some(t) = p.always(8.0, |p| p.state(post) == GuardState::Patrol && p.sam_light() < 0.1) { panic!("seed {s}: with the lamp off the sentry left calm, or Sam was lit, {t:.2} s in: state {:?}, awareness {:.2}, Sam's light {:.2}", p.state(post), p.awareness(post), p.sam_light()); }
    }
}

/// THE RULE: a live grenade that lands beside a defender with a line to him: he shouts, runs from it and gets down, and nothing else moves him
/// until it has gone off. With no grenade he stands on his mark.
/// A FAILURE MEANS: the grenade reaction (seeing it land, the flee, the crouch) is lost or late, or another rule took the man back toward the
/// grenade while it was live.
#[test]
fn a_defender_runs_from_a_live_grenade_and_gets_down() {
    for s in SEEDS {
        let (mark, grenade) = (west_post(s), [west_post(s)[0] + 1.25 + 0.3 * nudge(s, 7), west_post(s)[1] + 0.75 + 0.3 * nudge(s, 8)]);
        let mut p = Play::scene(&two_rooms(s), &seed(s));
        let d = p.defender_at(mark);
        p.run(1.0);
        let d0 = dist(p.at(d), grenade);
        p.frag_lands(grenade);
        let fuse = p.frag_fuse();
        // from the frame it lands until the fuse is out: how far he got, how much nearer he ever came than he had been, when he got down
        let (mut far, mut back, mut low_at, mut had) = (d0, 0.0f64, None, 0);
        let t0 = p.time();
        while p.time() - t0 < fuse - 0.05 {
            p.frame();
            if p.rule(d) == Some("a live grenade") { had += 1; }
            let dn = dist(p.at(d), grenade);
            back = back.max(far - dn);
            far = far.max(dn);
            if low_at.is_none() && p.low(d) { low_at = Some(p.time() - t0); }
        }
        assert!(p.lines(d).iter().filter(|l| l.contains("grenade")).count() == 1, "seed {s}: he shouts the grenade once: he said {:?}", p.lines(d));
        assert!(far - d0 > 1.0, "seed {s}: he ran from it: {d0:.2} m from the grenade when it landed, {far:.2} m at the most before it went off");
        let end = dist(p.at(d), grenade);
        assert!(far - end < 0.2 && back < 0.75, "seed {s}: nothing took him back toward it: {end:.2} m away as the fuse runs out, {far:.2} m at his farthest, and at some frame {back:.2} m nearer than he had been");
        assert!(low_at.is_some(), "seed {s}: he is down before it goes off");
        assert!(had > 30, "seed {s}: the grenade's rule of the list had him for {had} frames of the fuse: {}", p.why(d));
        drop(p);

        let mut p = Play::scene(&two_rooms(s), &seed(s));
        let d = p.defender_at(mark);
        if let Some(t) = p.always(1.0 + fuse, |p| dist(p.at(d), mark) < 0.3 && !p.low(d) && p.rule(d).is_none()) { panic!("seed {s}: with no grenade the defender left his mark, got down or was taken by a rule, {t:.2} s in: {}", p.why(d)); }
        assert!(p.lines(d).is_empty(), "seed {s}: with no grenade he says nothing: {:?}", p.lines(d));
    }
}

/// THE RULE: the pair ordered to stack on a door goes to it and ends one man each side of it, against the wall the door is in, on the side it
/// was ordered from; the pair that was given no order does not go with them.
/// A FAILURE MEANS: the stack order no longer deals its men the door's two sides (the order's own places, the walk to them), or an order
/// reaches men it was not given to.
#[test]
fn the_pair_ordered_to_a_door_stacks_on_its_two_sides() {
    for s in SEEDS {
        let scene = two_rooms(s);
        let door = scene.door_centre("west-yard");
        let mut p = Play::scene(&scene, &seed(s));
        let (red, blue) = (p.red(), p.blue());
        assert_eq!((red.len(), blue.len()), (2, 2), "seed {s}: the room clear stages four teammates in two pairs");
        let mates = [red[0], red[1], blue[0], blue[1]];
        let was: Vec<[f64; 2]> = mates.iter().map(|m| p.at(*m)).collect();
        p.run(0.5);
        p.order("stack", door, Who::Red);
        let near = |p: &Play, m| dist(p.at(m), door) < 1.5 && p.speed(m) < 0.05;
        let set = p.until(12.0, |p| near(p, mates[0]) && near(p, mates[1])).unwrap_or_else(|| panic!("seed {s}: 12 s after the order the pair is not standing at the door: {}", p.roll()));
        p.run(1.0);
        let at: Vec<[f64; 2]> = mates.iter().map(|m| p.at(*m)).collect();
        for i in 0..2 { assert!(at[i][1] > 16.0 && at[i][1] < 17.2, "seed {s}: man {i} of the pair is against the wall on the yard's side of it {set:.1} s after the order (z {:.2})", at[i][1]); }
        assert!((at[0][0] - door[0]) * (at[1][0] - door[0]) < 0.0, "seed {s}: the pair stands one man each side of the door: x {:.2} and {:.2} by a door at {:.2}", at[0][0], at[1][0], door[0]);
        assert!(p.stacked(mates[0]) || p.stacked(mates[1]), "seed {s}: one of the pair is set on the door in the stack's stance");
        // (the other pair may shift a step as the first leaves: what it must not do is go to the door)
        for i in 2..4 { assert!(dist(at[i], was[i]) < 1.5 && dist(at[i], door) > 4.0, "seed {s}: man {i}, of the pair that was given no order, moved {:.1} m and stands {:.1} m from the door", dist(at[i], was[i]), dist(at[i], door)); }
        assert!(p.said.iter().any(|l| l.text.contains("stack")), "seed {s}: somebody acknowledges the order: {:?}", p.said.iter().map(|l| l.text.as_str()).collect::<Vec<_>>());
    }
}

/// THE RULE: a defender posted in a room who hears a shot outside does not leave the room he holds: he is alert, and for the next ten seconds he
/// is inside its walls. With no shot he is calm on his mark.
/// A FAILURE MEANS: a posted man's hold gave way to a noise (he went to look, or was pulled out by another rule), or a shot in the yard is no
/// longer heard through a room's walls.
#[test]
fn a_posted_defender_holds_his_room_when_a_shot_is_fired_outside() {
    let inside = |at: [f64; 2]| at[0] > 18.0 && at[0] < 26.0 && at[1] > 10.0 && at[1] < 16.0;
    for s in SEEDS {
        let mut p = Play::scene(&two_rooms(s), &seed(s));
        let east = p.defender_at(east_post(s));
        p.run(0.5);
        p.sam_fires_at([11.0 + nudge(s, 9), 16.0], 0.6);   // (one round into the west room's wall, from the yard)
        assert!(p.sounds.iter().filter(|x| x.name == game::audio_iface::Sfx::RifleSuppressed).count() == 1 && !p.sounds.iter().any(|x| x.name == game::audio_iface::Sfx::Launcher), "seed {s}: Sam fired one rifle round and nothing else: the sounds were {:?}", p.sounds.iter().map(|x| x.name).collect::<Vec<_>>());
        let heard = p.until(3.0, |p| p.state(east) != GuardState::Patrol).unwrap_or_else(|| panic!("seed {s}: 3 s after a shot in the yard the defender next door is calm"));
        if let Some(t) = p.always(10.0, |p| inside(p.at(east)) && p.up(east)) { let a = p.at(east); panic!("seed {s}: he heard it after {heard:.2} s and {t:.2} s later he is at ({:.1}, {:.1}), outside the room he holds, or down", a[0], a[1]); }
        drop(p);

        let mut p = Play::scene(&two_rooms(s), &seed(s));
        let east = p.defender_at(east_post(s));
        if let Some(t) = p.always(6.0, |p| p.state(east) == GuardState::Patrol && dist(p.at(east), east_post(s)) < 0.3) { panic!("seed {s}: with no shot the defender left calm or his mark, {t:.2} s in"); }
    }
}

/// where the sentry of `shot_next_door` is posted, by the seed (up to a metre either way along the west room)
fn door_post(seed: u32) -> [f64; 2] { [12.25, 13.25 + nudge(seed, 10)] }
/// Two rooms with a shut door between them, lit; a sentry in the west room with his back to the door, Sam in the east room.
fn shot_next_door(seed: u32) -> Scene {
    let mut s = Scene::new("shot-next-door");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("between", Axis::Z, 18.0, 12.4).door("east-yard", Axis::X, 16.0, 21.4);
    s.bulb(14.0, 13.0).bulb(22.0, 13.0);
    let post = door_post(seed);
    s.sentry("post", post, [10.0, post[1]]).sam(23.75 + 0.5 * nudge(seed, 11), 13.25);
    s
}

/// THE RULE: a guard who hears a shot behind a shut door says so, turns to it and goes: within seconds he is alert, believes the shooter is in the
/// room the shot was fired in, and has left his post for the door. With no shot he stands calm with his back to it.
/// A FAILURE MEANS: a shot is no longer heard through a shut door (the sound's path, its loudness against the door), or a man who hears one no
/// longer acts on where it came from.
#[test]
fn a_guard_who_hears_a_shot_behind_a_shut_door_turns_to_it_and_goes() {
    let east_room = |at: [f64; 2]| at[0] > 18.0 && at[0] < 26.0 && at[1] > 10.0 && at[1] < 16.0;
    for s in SEEDS {
        let mut p = Play::scene(&shot_next_door(s), &seed(s));
        let post = p.guard("post");
        p.run(1.0);
        let was = p.at(post);
        assert_eq!(p.state(post), GuardState::Patrol, "seed {s}: the sentry is calm before the shot (the shut door hides Sam from him)");
        p.sam_fires_at([25.9, 13.25], 0.6);   // (one round into the east room's far wall)
        assert!(p.sounds.iter().filter(|x| x.name == game::audio_iface::Sfx::RifleSuppressed).count() == 1 && !p.sounds.iter().any(|x| x.name == game::audio_iface::Sfx::Launcher), "seed {s}: Sam fired one rifle round and nothing else: the sounds were {:?}", p.sounds.iter().map(|x| x.name).collect::<Vec<_>>());
        p.until(1.0, |p| p.state(post) == GuardState::Alert).unwrap_or_else(|| panic!("seed {s}: a second after a rifle shot next door the sentry is {:?}", p.state(post)));
        assert!(p.lines(post).iter().any(|l| l.contains("shot")), "seed {s}: he says what he heard: {:?}", p.lines(post));
        let fix = p.fix(post).unwrap_or_else(|| panic!("seed {s}: alert and no place he believes the shooter is"));
        assert!(east_room(fix), "seed {s}: he believes the shooter is in the room the shot was fired in: his fix is ({:.1}, {:.1})", fix[0], fix[1]);
        p.until(5.0, |p| p.at(post)[0] - was[0] > 3.0).unwrap_or_else(|| { let a = p.at(post); panic!("seed {s}: 5 s after the shot he is at ({:.1}, {:.1}), {:.1} m nearer the door than his post", a[0], a[1], a[0] - was[0]) });
        drop(p);

        let mut p = Play::scene(&shot_next_door(s), &seed(s));
        let post = p.guard("post");
        if let Some(t) = p.always(7.0, |p| p.state(post) == GuardState::Patrol && dist(p.at(post), door_post(s)) < 0.3) { panic!("seed {s}: with no shot the sentry left calm or his post, {t:.2} s in"); }
    }
}

/// Every scene of this file as a level file under `target/scenes/`, and each read back as the level it was written from.
#[test]
fn the_scenes_are_level_files() {
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/scenes");
    std::fs::create_dir_all(&dir).unwrap();
    for (name, s) in [("watched-room-lit", watched_room(true, 1)), ("watched-room-dark", watched_room(false, 1)), ("two-rooms", two_rooms(1)), ("shot-next-door", shot_next_door(1))] {
        let text = s.level_text();
        assert_eq!(game_start::json::parse(&text).unwrap(), s.level(), "{}: the text reads back as the value it was written from", name);
        std::fs::write(dir.join(format!("{}.json", name)), text).unwrap();
    }
}
