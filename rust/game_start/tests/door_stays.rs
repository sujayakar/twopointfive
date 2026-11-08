//! A DOOR STAYS WHERE IT IS LEFT: keyed and opened, it stands open with its bolt still thrown until a hand shuts it, and a man whose way is past a leaf on its stop gets there.
mod doors_ask;
use doors_ask::*;
use game::game::GuardState;
use game::ids::*;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};

fn at_a_door(s: u32, locked: bool) -> (Play, DoorId, [f64; 2]) {
    let (mut sc, at) = yard(s);
    if locked { sc.locked("west-yard"); }
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.run(0.2);
    let keep = p.defender_at([24.25, 12.25]);
    p.drop_all_but(&[keep]);
    p.pin_man(keep, [24.25, 12.25], 0.0, GuardState::Patrol, 0.0);
    let d = p.door("west-yard");
    p.leaf_stays(d);
    (p, d, at)
}

/// THE RULE: "doors ... doesn't automatically close". A keyed door opened with the key stands where the hand left it, unlatched, its bolt still thrown and nothing broken, and open it is no wall to
/// a man with no key. A hand pulls it to: it latches and the lock bites again, for a man with no key and not for the key, and never from the room side. The control is the plain door pulled to: no lock bites.
/// A FAILURE MEANS: a leaf has a closer of its own, or a door opened with a key stays open to anyone once it is shut.
#[test]
fn a_keyed_door_stands_open_until_a_hand_shuts_it_and_then_it_locks() {
    for s in SEEDS {
        let (mut p, d, _) = at_a_door(s, true);
        p.leaf_put_shut(d);
        p.frame();
        let side = p.key_face(d);
        let (k, far) = (p.on_face(d, p.door_w(d) / 2.0, side * 0.6), p.on_face(d, p.door_w(d) / 2.0, -side * 0.6));
        assert_eq!(p.locked_out(d, k), (true, false), "seed {s}: shut, it bites on its keyed face for a man with no key");
        assert_eq!(p.locked_out(d, far), (false, false), "seed {s}: and never from the room side");
        p.door_used(d, k, true);
        p.run(12.0);
        let a0 = p.leaf_angle(d);
        p.run(30.0);
        assert!(!p.latched(d) && a0.abs() > 0.6 && (p.leaf_angle(d) - a0).abs() < 1e-6, "seed {s}: the leaf went from {a0:.3} to {:.3}", p.leaf_angle(d));
        assert!(p.door_locked_now(d) && !p.keep_broken(d), "seed {s}: the bolt stays thrown, nothing broken");
        assert_eq!(p.locked_out(d, k), (false, false), "seed {s}: standing open it is no wall");
        p.door_used(d, far, false);
        p.until(8.0, |p| p.latched(d)).unwrap_or_else(|| panic!("seed {s}: a hand pulled it and it never latched"));
        assert_eq!(p.locked_out(d, k), (true, false), "seed {s}: pulled to, it bites again");
        drop(p);

        let (mut p, d, _) = at_a_door(s, false);
        p.leaf_put_shut(d);
        p.frame();
        let k = p.on_face(d, p.door_w(d) / 2.0, 0.6);
        let far = p.on_face(d, p.door_w(d) / 2.0, -0.6);
        p.door_used(d, k, false);
        p.run(12.0);
        p.door_used(d, far, false);
        p.until(8.0, |p| p.latched(d)).unwrap_or_else(|| panic!("seed {s}: the plain door never latched"));
        assert_eq!(p.locked_out(d, k), (false, false), "seed {s}: the control: no key, no lock");
    }
}

/// a man who walks from the west room to the east through the door between them, the door as `open` says
fn past(s: u32, open: bool) -> (Option<f64>, f64, f64, bool) {
    let n = nudge(s, 95);
    let to = [24.0, 13.0 + n];
    let mut sc = Scene::new("past");
    sc.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    sc.door("between", Axis::Z, 18.0, 12.4 + n).bulb(14.0, 13.0).bulb(22.0, 13.0).sam(14.0, 22.0);
    sc.patrol("walker", &[[12.0, 13.0 + n], to], 3.0);
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    let (d, m) = (p.door("between"), p.guard("walker"));
    p.leaf_stays(d);
    if open { p.leaf_put_open(d, 1.35); }
    let a0 = p.leaf_angle(d);
    let t = p.until(25.0, |p| dist(p.at(m), to) < 1.0);
    p.run(1.0);
    (t, a0, p.leaf_angle(d), p.latched(d))
}

/// THE RULE: a man whose way is past a leaf on its stop gets there, and the leaf has not been pulled to behind him. The control is the same man and the same door found shut: he opens it, and so the
/// reading of the leaf can move.
/// A FAILURE MEANS: an open leaf stops a man in a doorway, or he shuts it after him when nobody told him to.
#[test]
fn a_man_goes_past_a_leaf_on_its_stop_and_leaves_it_there() {
    for s in SEEDS {
        let (t, a0, a1, latched) = past(s, true);
        assert!(t.is_some(), "seed {s}: he never got past");
        assert!(!latched && a1.abs() >= a0.abs() - 0.05, "seed {s}: the leaf went from {a0:.2} to {a1:.2}, latched {latched}");
        let (_, a0, a1, _) = past(s, false);
        assert!((a1 - a0).abs() > 0.3, "seed {s}: the control: found shut he went through it and it stayed at {a1:.2}");
    }
}
