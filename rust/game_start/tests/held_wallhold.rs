//! THE WALL HOLD: what can be taken, by which hand, and that the slide stays on it.
mod kit_ask;
use game_start::play::{dist, Opts, Play};
use kit_ask::KitAsk;

/// The office with nobody able to see or hear Sam, his guns put away.
fn office(seed: u32) -> Play {
    let mut p = Play::level("office.json", false, &Opts { seed, ..Default::default() });
    p.sam_a_ghost();
    p.sam_unhurt();
    p.run(0.1);
    p
}
/// Q tapped at a place: the wall he holds half a second later, and he is let go again.
fn q_at(p: &mut Play, at: [f64; 2]) -> Option<[f64; 2]> {
    p.sam_put(at);
    p.run(0.1);
    p.tap("KeyQ");
    p.run(0.6);
    let held = p.on_wall();
    if held.is_some() { p.tap("KeyQ"); p.run(0.3); }
    held
}

/// THE RULE: Q takes a wall within reach: a wall, a partition, a bank of racks, glazing with its sill and header, his back to it. Desks, counters, a door's leaf and open floor refuse.
/// A FAILURE MEANS: the hold has stopped reading what stands tall enough to put a back to.
#[test]
fn q_takes_a_wall_and_refuses_a_desk_a_leaf_and_open_floor() {
    let mut p = office(1);
    for (what, at, n) in [
        ("the corridor's north wall", [20.0, 10.7], [0.0, 1.0]), ("the corridor's south wall", [18.0, 11.5], [0.0, -1.0]), ("a cubicle's end panel", [13.5, 15.2], [-1.0, 0.0]),
        ("the racks' fronts", [16.3, 7.8], [0.0, 1.0]), ("the conference glazing", [12.0, 10.7], [0.0, 1.0]), ("the break room's partition", [28.7, 21.0], [1.0, 0.0]),
    ] {
        let got = q_at(&mut p, at).unwrap_or_else(|| panic!("{what}: Q took nothing: {:?}", p.log()));
        assert!(dist(got, n) < 0.01, "{what}: his back is to {got:?}, not {n:?}");
    }
    for (what, at) in [("the manager's desk", [28.0, 6.3]), ("the reception counter", [8.2, 16.2]), ("the kitchen counter", [34.6, 15.0]), ("the server door's leaf", [17.6, 10.6]), ("open floor", [19.9, 18.0])] {
        assert_eq!(q_at(&mut p, at), None, "{what}: Q took it");
    }
}

/// THE RULE: "when you like push against the wall, you snap to it": a walk straight at a wall has it before he touches it, with no key but his walk, his back to it. A walk along it, into a door's leaf, or let go short of it, does not.
/// A FAILURE MEANS: the hold has lost the key-less take, or it grabs a man who is only walking past a wall.
#[test]
fn a_key_held_into_a_wall_takes_it_and_walking_along_it_does_not() {
    let mut p = office(2);
    p.sam_put([21.0, 11.5]);
    p.run(0.1);
    p.key_down("KeyW");
    p.until(2.0, |p| p.on_wall().is_some()).expect("a walk straight at the corridor's north wall took nothing");
    p.run(0.5);
    p.key_up("KeyW");
    p.run(0.1);
    let n = p.on_wall().expect("he let go");
    assert!(dist(n, [0.0, 1.0]) < 0.01, "his back is to {n:?}");
    assert!((p.sam_at()[1] - 10.51).abs() < 0.02, "he stands {:.3} m off the wall's plane, where Q's hold stands him 0.45 m", p.sam_at()[1] - 10.06);
    p.tap("KeyQ");
    p.run(0.3);
    for (what, at, keys) in [("along the wall", [21.0, 10.7], &["KeyD"][..]), ("the server door's leaf", [17.6, 10.75], &["KeyW"][..])] {
        p.sam_put(at);
        p.run(0.2);
        for k in keys { p.key_down(k); }
        let held = p.always(1.5, |p| p.on_wall().is_none());
        for k in keys { p.key_up(k); }
        assert_eq!(held, None, "{what}: he took a wall");
        p.run(0.3);
    }
    p.sam_put([21.0, 11.5]);
    p.run(0.1);
    p.key_down("KeyW");
    p.until(0.25, |_| false);
    p.key_up("KeyW");
    assert_eq!(p.always(0.5, |p| p.on_wall().is_none()), None, "let go short of the wall and he took it");
}

/// THE RULE: on the wall his keys slide him along it and stop at its ends: his back stays on its plane, walking and crouched, right across a run with things on it.
/// The control: stepping off, Shift puts him on the floor again, and he walks away from the plane.
/// A FAILURE MEANS: the slide lets go, drifts off the wall or runs through a jamb.
#[test]
fn the_slide_stays_on_the_wall() {
    let mut p = office(3);
    for low in [false, true] {
        p.sam_put([20.0, 10.7]);
        p.sam_low(low);
        p.run(0.1);
        p.tap("KeyQ");
        p.run(0.6);
        assert!(p.on_wall().is_some(), "low {low}: Q took nothing");
        let z = p.sam_at()[1];
        for key in ["KeyD", "KeyA"] {
            let from = p.sam_at();
            p.key_down(key);
            let on = p.always(3.0, |p| p.on_wall().is_some() && (p.sam_at()[1] - z).abs() < 0.02);
            let at = p.sam_at();
            p.key_up(key);
            assert_eq!(on, None, "low {low}, {key}: he came off the plane at {at:?}");
            assert!((at[0] - from[0]).abs() > 1.0, "low {low}, {key}: he never moved along it ({at:?} from {from:?})");
        }
        p.tap("KeyQ");
        p.run(0.3);
        p.sam_low(false);
    }
    p.sam_put([20.0, 10.7]);
    p.run(0.1);
    p.tap("KeyQ");
    p.run(0.6);
    let z = p.sam_at()[1];
    p.tap("ShiftLeft");
    p.key_down("KeyS");
    p.run(0.5);
    p.key_up("KeyS");
    assert!(p.on_wall().is_none() && p.sam_at()[1] > z + 0.3, "the control: off the wall he should walk away from it ({:?})", p.sam_at());
}
