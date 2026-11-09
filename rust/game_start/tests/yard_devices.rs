//! THE FLOODLIT YARD, AND WHAT WATCHES A FLOOR: the yard is dark with the mains out; a camera and a beam see what the light and the line let them.
mod world_ask;
use game::sim_lightshome::CameraState;
use game::workthings::WorkThingKind;
use game_start::json::{b, n, nums, obj, s as text, Json};
use game_start::play::{nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use world_ask::{scene_plus, YardAsk};

const SEEDS: [u32; 3] = [1, 2, 3];
fn seed(seed: u32) -> Opts { Opts { seed, ..Default::default() } }

/// THE RULE: the floodlit yard's one idea is the mains: with them on the gate and the middle of the yard read bright; with them out the yard's sides and the start read under 0.2 to a bare eye, moon counted, and a tube still reads the yard;
/// each room by its lantern is dim, never black and never bright, and a tube reads it better than the eye. The numbers are the old check's, measured on both readings.
/// A FAILURE MEANS: the mission's breaker no longer buys the team the dark, or the rooms are black to everybody.
#[test]
fn the_yard_is_lit_off_the_mains_and_dark_with_them_out() {
    let mut p = Play::mission("floodlit.json", "?levelFile=floodlit.json&roomclear&squad4", &seed(1));
    p.g.god_mode = true;
    p.run(0.5);
    for (at, what) in [([20.0, 23.4], "the gate"), ([20.0, 18.5], "mid yard")] {
        let lit = p.place_light(at);
        assert!(lit.0 > 0.6, "{what}, the mains on: reads {lit:?}");
    }
    game::game::game_set_blackout(&mut p.g, 86400.0, game::game::GameSetBlackoutCause::Hand);
    p.run(4.0);
    for at in [[8.0, 18.0], [5.0, 21.8]] {
        let d = p.place_light(at);
        assert!(d.0 < 0.2 && d.1 < 0.2 && d.2 > 0.15, "the yard at {at:?}, the mains out: reads {d:?}");
    }
    for at in [[9.0, 26.2], [7.8, 26.7], [10.2, 26.7]] { let d = p.place_light(at); assert!(d.0 < 0.2 && d.1 < 0.2, "the start at {at:?}: reads {d:?}"); }
    for at in [[12.6, 12.3], [17.0, 10.0], [29.2, 10.5]] {
        let d = p.place_light(at);
        assert!(d.1 > 0.3 && d.1 < 0.85 && d.2 > d.1 + 0.15, "a room at {at:?}: reads {d:?}");
    }
}

/// THE SAME RULE at the places the game has moved from: the gate and the middle of the yard, the mains out, under 0.2 to a bare eye (the old check's bar: it read 0.00 to 0.13 on the meter it was written on, the battery lamp on the mast counted),
/// and the radio room's tube better than its eye by 0.15.
#[test]
fn the_gate_and_the_middle_of_the_yard_are_dark_with_the_mains_out() {
    let mut p = Play::mission("floodlit.json", "?levelFile=floodlit.json&roomclear&squad4", &seed(1));
    p.g.god_mode = true;
    game::game::game_set_blackout(&mut p.g, 86400.0, game::game::GameSetBlackoutCause::Hand);
    p.run(4.5);
    for at in [[20.0, 23.4], [20.0, 18.5]] { let d = p.place_light(at); assert!(d.0 < 0.2 && d.1 < 0.2, "the yard at {at:?}: reads {d:?}"); }
    let d = p.place_light([26.0, 10.6]);
    assert!(d.2 > d.1 + 0.15, "the radio room reads {d:?}");
}

/// THE RULE: the mission's own verb: the squad's work order has the breaker as "throw the breaker" with a place to stand inside the shed; nobody holds the shed; no emergency beacon on the level; nobody is issued a torch.
/// A FAILURE MEANS: the floodlit mission has lost the one thing it is about.
#[test]
fn the_breaker_is_the_squads_to_throw_from_inside_the_shed() {
    for s in SEEDS {
        let mut p = Play::mission("floodlit.json", "?levelFile=floodlit.json&roomclear&squad4", &seed(s));
        p.g.god_mode = true;
        p.run(0.3);
        let shed = p.g.level.scenarios.as_ref().and_then(|c| c.places.as_ref()).and_then(|v| v.areas.iter().find(|a| a.name == "generator shed")).expect("a shed").bounds;
        let inside = |q: [f64; 2], m: f64| q[0] > shed.x0 - m && q[0] < shed.x1 + m && q[1] > shed.z0 - m && q[1] < shed.z1 + m;
        let th = game::workthings::work_things_things(&p.g, &game::seats::here(&p.g)).into_iter().find(|x| x.kind == WorkThingKind::Breaker).expect("the breaker");
        let st = th.stand.expect("a place to stand");
        assert!(inside([st[0], st[2]], 0.15), "seed {s}: he throws it from {st:?}");
        assert_eq!(game::workthings::work_things_words(&p.g, &th), "throw the breaker", "seed {s}");
        for m in p.hostiles() { assert!(!inside(p.at(m), 0.0), "seed {s}: a man is dealt into the shed"); }
        assert!(p.g.level.beacons.is_empty(), "seed {s}: an emergency beacon");
    }
}

fn camera() -> Json { obj(vec![("id", text("cam")), ("kind", text("camera")), ("pos", nums(&[7.0, 2.6, 13.0])), ("yaw", n(std::f64::consts::FRAC_PI_2)), ("pitch", n(-0.2)), ("fovDeg", n(70.0)), ("range", n(14.0)), ("mains", b(false))]) }
/// A room with a camera on its own battery looking down it, a man posted at the far end with his back to the room, Sam under a bulb in its picture, lit or not, with a wall between or not.
fn watched(s: u32, lit: bool, wall: bool) -> Play {
    let z = 13.0 + 0.6 * nudge(s, 0);
    let mut sc = Scene::new("watched");
    sc.room("room", 6.0, 8.0, 30.0, 18.0).door("yard", Axis::Z, 6.0, 15.4);
    if lit { sc.bulb(14.0, z); } else { sc.bulb_off(14.0, z); }
    if wall { sc.wall_z(11.0, 8.0, 18.0); }
    sc.sentry("post", [28.25, 9.25], [30.0, 9.25]).sam(14.0, z);
    scene_plus(&sc, vec![], vec![camera()], &seed(s))
}

/// THE RULE: a camera's exposure is the light on the man: lit in its picture it calls him in and a calm man is sent to the spot. CONTROLS: the same place, the lamp off: nothing; the lamp on and a wall between: nothing.
/// A FAILURE MEANS: a camera sees through the dark or through a wall, or sees and tells nobody.
#[test]
fn a_camera_sees_what_the_light_and_the_line_let_it() {
    for s in SEEDS {
        let mut p = watched(s, true, false);
        p.g.god_mode = true;
        let t = p.until(6.0, |p| p.camera_state() != CameraState::Idle).unwrap_or_else(|| panic!("seed {s}: lit in its picture and the camera is idle: seen {:.2}, Sam's light {:.2}", p.camera_seen(), p.sam_light()));
        assert!(t > 0.3, "seed {s}: it called him at once ({t:.2} s)");
        p.run(0.1);
        assert!(p.sent_to_look().is_some() && p.log().iter().any(|l| l.contains("has movement")), "seed {s}: nobody sent: {:?}", p.log());
        drop(p);
        for (lit, wall, what) in [(false, false, "in the dark"), (true, true, "behind a wall")] {
            let mut p = watched(s, lit, wall);
            p.g.god_mode = true;
            if let Some(t) = p.always(3.5, |p| p.camera_state() == CameraState::Idle && p.camera_seen() < 0.05) { panic!("seed {s}: {what}: seen {:.2} at {t:.2} s", p.camera_seen()); }
        }
    }
}

fn beam(y: f64) -> Json { obj(vec![("id", text("beam")), ("kind", text("laser")), ("from", nums(&[16.0, y, 8.1])), ("to", nums(&[16.0, y, 17.9])), ("mains", b(false)), ("alarm", text("suspicion"))]) }
/// A room crossed by a beam at a height; a man walking his round across it; Sam on the west side.
fn crossed(s: u32, y: f64) -> Play {
    let z = 13.0 + 0.6 * nudge(s, 0);
    let mut sc = Scene::new("crossed");
    sc.room("room", 6.0, 8.0, 26.0, 18.0).door("yard", Axis::Z, 6.0, 15.4).bulb_off(16.0, 13.0);
    sc.patrol("round", &[[14.25, 9.25], [18.25, 9.25]], 0.2).sam(13.0, z);
    let mut p = scene_plus(&sc, vec![], vec![beam(y)], &seed(s));
    p.g.god_mode = true;
    p
}

/// THE RULE: a shin beam: a man of the floor walks through his own beam on his round and nothing trips; Sam walking through trips it once. A chest beam: crouched under it, nothing; on his feet, a trip.
/// A FAILURE MEANS: a beam is a line a player cannot read: staff set it off, or it does not care how high it is.
#[test]
fn a_beam_is_broken_by_a_stranger_at_its_height() {
    for s in SEEDS {
        let mut p = crossed(s, 0.35);
        let round = p.guard("round");
        let (mut crossings, mut was) = (0, p.at(round)[0]);
        p.until(8.0, |p| { let x = p.at(round)[0]; if (x - 16.0) * (was - 16.0) < 0.0 { crossings += 1; } was = x; crossings >= 1 });
        assert!(crossings >= 1 && p.beam_trips(0) == 0.0, "seed {s}: crossed {crossings} times, tripped {}", p.beam_trips(0));
        let z = p.sam_at()[1];
        assert!(p.sam_goes_to([18.0, z], 6.0), "seed {s}: Sam did not get across");
        assert_eq!(p.beam_trips(0), 1.0, "seed {s}: Sam walked through the shin beam");
        drop(p);

        let mut p = crossed(s, 1.25);
        let z = p.sam_at()[1];
        p.sam_crouched(true);
        assert!(p.sam_goes_to([18.0, z], 9.0), "seed {s}: Sam did not get across on his knees");
        assert_eq!(p.beam_trips(0), 0.0, "seed {s}: crouched under the chest beam");
        p.sam_crouched(false);
        assert!(p.sam_goes_to([13.5, z], 6.0), "seed {s}: Sam did not get back");
        assert_eq!(p.beam_trips(0), 1.0, "seed {s}: on his feet through the chest beam");
    }
}
