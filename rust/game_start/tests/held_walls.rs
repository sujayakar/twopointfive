//! NOTHING WALKS OR SHOVES SAM THROUGH A WALL: not his own speed, not a leaf kicked onto him, not a man running into him. Each with a control in which he does go through: the doorway.
mod kit_ask;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use kit_ask::KitAsk;

fn seed(s: u32) -> Opts { Opts { seed: s, ..Default::default() } }

/// Two rooms sharing a wall 12 cm thick on x = 18 with a door in it (its slot starting at z = 12.4), Sam in the west one at `sam`, a ghost and unhurt; one man parked in the far corner of the east one.
fn two_rooms(sam: [f64; 2], s: u32) -> Play {
    let mut sc = Scene::new("two-rooms");
    sc.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0).door("between", Axis::Z, 18.0, 12.4).door("yard", Axis::X, 16.0, 21.4);
    sc.bulb(14.0, 13.0).bulb(22.0, 13.0);
    sc.sentry("man", [25.0, 15.0], [20.0, 15.0]).sam(sam[0], sam[1]);
    let mut p = Play::scene(&sc, &seed(s));
    p.sam_a_ghost();
    p.sam_unhurt();
    p.run(0.1);
    p
}

/// Run `secs` at `dt` a step: the deepest Sam ever stood in a wall, and whether he ended on the far side of the wall line x = 18.
fn drive(p: &mut Play, secs: f64, dt: f64) -> (f64, f64) {
    let (mut pen, mut far) = (0.0f64, f64::NEG_INFINITY);
    for _ in 0..(secs / dt).round() as usize {
        p.step(dt);
        pen = pen.max(p.sam_in_a_wall());
        far = far.max(p.sam_at()[0] - 18.0);
    }
    (pen, far)
}

/// THE RULE: a wall stays a wall at any speed and any frame time. Driven square and at 45° into a thin wall, walking, sprinting and crouched, at 60 and at 20 steps a second, he never stands in it and never gets
/// past it. The control: walked at the same wall's open doorway he is through in the same time.
/// A FAILURE MEANS: one fast step or one long frame takes a player through a wall, the fault this family was written for.
#[test]
fn no_speed_takes_him_through_a_thin_wall() {
    for (s, z, dz, low, sprint, dt) in [
        (1, 15.0, 0.0, false, false, 1.0 / 60.0), (2, 11.0, 0.0, false, true, 1.0 / 60.0), (3, 15.2, 0.0, true, false, 1.0 / 60.0),
        (1, 11.0, 2.5, false, true, 1.0 / 60.0), (2, 15.0, 0.0, false, true, 1.0 / 20.0), (3, 11.2, 2.5, false, true, 1.0 / 20.0), (1, 15.0, 0.0, true, false, 1.0 / 20.0),
    ] {
        let z = z + 0.2 * nudge(s, 0);
        let mut p = two_rooms([17.3, z], s);
        p.sam_walks_to([22.0, z + dz], low);
        if sprint { p.key_down("ShiftLeft"); }
        let (pen, far) = drive(&mut p, 1.5, dt);
        assert!(pen < 0.02, "z {z:.1}, sprint {sprint}, low {low}, dt {dt:.3}: he stood {pen:.3} m in the wall");
        assert!(far < -0.1, "z {z:.1}, sprint {sprint}, low {low}, dt {dt:.3}: he got {far:.2} m past the wall's line");
    }
    let mut p = two_rooms([17.3, 13.0], 1);
    let d = p.door("between");
    p.leaf_left_at(d, 1.7);
    p.sam_walks_to([22.0, 13.0], false);
    p.key_down("ShiftLeft");
    let (_, far) = drive(&mut p, 2.5, 1.0 / 60.0);
    assert!(far > 1.0, "the control: through the open doorway he got only {far:.2} m past the line: the drive shows nothing");
}

/// THE RULE: a leaf kicked onto Sam moves him but never into or through a wall, wherever he stands about the doorway, at 60 and at 20 steps a second. The leaf does move: the control.
/// A FAILURE MEANS: a door's shove is an uncapped write on his place, and a kicked door pushes him through the wall behind him.
#[test]
fn a_leaf_kicked_onto_him_does_not_put_him_through_a_wall() {
    for (s, along, off, dt) in [
        (1, 12.3, 0.55, 1.0 / 60.0), (2, 12.9, 0.55, 1.0 / 60.0), (3, 13.6, 0.5, 1.0 / 20.0), (1, 12.5, 0.9, 1.0 / 20.0), (2, 13.1, 0.9, 1.0 / 60.0), (3, 11.9, 0.5, 1.0 / 20.0),
    ] {
        let mut p = two_rooms([14.0, 13.0], s);
        let d = p.door("between");
        // (he stands on the face the leaf opens toward; the boot comes from the other)
        let side = if p.opens_toward(d, [19.5, 13.0]) { 1.0 } else { -1.0 };
        let (sam, from) = ([18.0 + side * off, along], [18.0 - side * 1.5, 13.0]);
        p.sam_put(sam);
        p.run(0.05);
        let at = p.sam_at();
        p.door_booted(d, from);
        let (mut pen, mut moved, mut swung) = (0.0f64, 0.0f64, 0.0f64);
        for _ in 0..(2.2 / dt) as usize { p.step(dt); pen = pen.max(p.sam_in_a_wall()); moved = moved.max(dist(p.sam_at(), at)); swung = swung.max(p.leaf(d).abs()); }
        assert!(pen < 0.02, "{sam:?} at dt {dt:.3}: he stood {pen:.3} m in a wall (moved {moved:.2} m)");
        assert!(!p.wall_between(at, p.sam_at()), "{sam:?} at dt {dt:.3}: a wall stands between where he stood and where he is");
        assert!(swung > 0.5, "{sam:?}: the control: the boot did not swing the leaf ({swung:.2} rad)");
    }
}

/// THE RULE: a man walking or running into Sam with a wall at Sam's back shoulders past him: Sam is never in the wall, is moved by no jump of a frame and by no more than a stride or so, and the man gets by.
/// The control: on open floor with nobody coming, he stays where he stands.
/// A FAILURE MEANS: men bulldoze the player through partitions or pin him, or get stuck on him for ever.
#[test]
fn a_man_running_into_him_does_not_put_him_through_a_wall() {
    for (s, speed, sam, from, goal) in [(1, 1.3, [17.55, 14.2], [17.5, 11.0], [17.5, 15.5]), (2, 2.7, [17.55, 14.8], [17.5, 11.5], [17.5, 15.6]), (3, 2.7, [17.5, 11.0], [17.5, 15.0], [17.5, 10.5])] {
        let mut p = two_rooms(sam, s);
        let at = p.sam_at();
        let man = p.guard("man");
        p.man_walks_to(man, from, goal, speed);
        let (mut pen, mut jump, mut far, mut near, mut last) = (0.0f64, 0.0f64, 0.0f64, f64::MAX, at);
        for _ in 0..(8 * 60) {
            p.frame();
            let now = p.sam_at();
            pen = pen.max(p.sam_in_a_wall());
            jump = jump.max(dist(now, last));
            far = far.max(dist(now, at));
            near = near.min(dist(now, p.at(man)));
            last = now;
        }
        assert!(near < 0.8, "seed {s}: the staging: he never came to Sam ({near:.2} m at closest)");
        assert!(pen < 0.02, "seed {s}: Sam stood {pen:.3} m in the wall at worst");
        assert!(jump < 0.11, "seed {s}: Sam jumped {jump:.3} m in one frame");
        assert!(far < 1.6, "seed {s}: Sam was carried {far:.2} m");
        assert!(dist(p.at(man), goal) < 1.0, "seed {s}: the man never got past him ({:.2} m from where he was sent)", dist(p.at(man), goal));
        assert!(p.sam_at()[0] < 18.0, "seed {s}: Sam is through the wall");
    }
    let mut p = two_rooms([14.0, 13.0], 1);
    let at = p.sam_at();
    p.run(3.0);
    assert!(dist(p.sam_at(), at) < 0.01, "the control: nobody near and he moved");
}
