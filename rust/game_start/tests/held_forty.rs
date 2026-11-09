//! THE 40 MM: one right click on a man far off, the round arms on the way and the frag's own blast takes him, once; short of its arming distance it is a slug, a clatter and a lump on the floor.
mod kit_ask;
use game::ids::GuardId;
use game_start::play::{nudge, Button, Opts, Play};
use game_start::scene::{Axis, Scene};
use kit_ask::KitAsk;

/// A dark room, Sam with the rifle up and the 40 mm on its button (2 again), a calm man `d` metres east of him with his back to him.
fn forty_stage(d: f64, s: u32) -> (Play, GuardId, f64) {
    let mut sc = Scene::new("forty");
    let z = 14.0 + 0.4 * nudge(s, 0);
    sc.room("room", 10.0, 10.0, 40.0, 18.0).door("south", Axis::X, 18.0, 20.0).bulb_off(16.0, 14.0);
    sc.sentry("man", [13.0 + d, z], [39.0, z]).sam(13.0, z);
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.sam_takes_his_rifle();
    p.sam_issued_the_forty();
    assert!(!p.forty_up(), "seed {s}: a fresh rifle has the ring on its button");
    digit_two(&mut p);
    assert!(p.forty_up(), "seed {s}: 2 again puts the 40 mm up");
    let man = p.guard("man");
    (p, man, z)
}

/// The rifle's digit twice: once opens the setup, once more is the next load.
fn digit_two(p: &mut Play) {
    p.tap("Digit2");
    p.frame();
    p.tap("Digit2");
    p.run(0.1);
}

fn right_click(p: &mut Play, at: [f64; 3]) {
    p.cursor_on(at);
    p.run(0.4);
    p.press(Button::Right);
    p.frame();
    p.release(Button::Right);
    p.frame();
    p.frame();
}

/// THE RULE: fourteen metres off, one click sends one round, gone from the tube; it arms on the way, inside half a second he dies by the blast's one report, Sam not in it; nothing leaves the rifle as gunfire and no dud lies there.
/// THE CONTROL: the digit once more puts the ring back on the button, and the same click sends no round of the 40 mm.
/// A FAILURE MEANS: the launcher is a gun, or a blast counted twice, or it hits its own man.
#[test]
fn far_off_it_arms_and_the_blast_takes_him_once() {
    for s in [1, 2, 3] {
        let (mut p, man, _) = forty_stage(14.0, s);
        let (air0, tube0, arm) = p.forty();
        assert_eq!(air0, 0);
        let at = p.chest(man);
        right_click(&mut p, at);
        let (air, tube, _) = p.forty();
        assert_eq!((air, tube), (1, tube0 - 1.0), "seed {s}: one round in the air, one gone from the tube");
        let mut shots = 0;
        p.until(0.5, |p| { shots = shots.max(p.gunfire_and_bangs().0); p.forty().0 == 0 }).unwrap_or_else(|| panic!("seed {s}: still flying after half a second"));
        assert_eq!((shots, p.gunfire_and_bangs().1), (0, 1), "seed {s}: gunfire and blast events");
        p.run(1.0);
        assert!(!p.up(man), "seed {s}: he is up");
        assert_eq!(p.blasts(), 1, "seed {s}: blast reports");
        let (took, sam_in_it) = p.burst_took(man);
        assert!(took && !sam_in_it, "seed {s}: the report has him {took}, Sam {sam_in_it}");
        let (armed, flown, _) = p.forty_end().expect("the round's end is logged");
        assert!(armed && flown >= arm && (flown - 14.0).abs() < 1.2, "seed {s}: armed {armed} after {flown:.2} m");
        assert_eq!(p.forty_duds(), 0, "seed {s}: a dud after a burst");
        assert!(p.log().iter().any(|l| l.contains("40 mm grenade") && l.contains("1 down")), "seed {s}: the log: {:?}", p.log());
        drop(p);

        let (mut p, man, _) = forty_stage(14.0, s);
        digit_two(&mut p);
        assert!(!p.forty_up(), "seed {s}, control: the digit again is the ring");
        let at = p.chest(man);
        right_click(&mut p, at);
        assert_eq!(p.forty().0, 0, "seed {s}, control: the ring's click sends no round of the 40 mm");
    }
}

/// THE RULE: five metres off it has not armed: no blast, he folds and lives and knows, the shell lies inert by him and stays so.
/// THE CONTROL: the round at fourteen metres, above, is a blast.
/// A FAILURE MEANS: a man can frag himself point blank, or a dud kills.
#[test]
fn short_of_its_arming_it_is_a_slug() {
    for s in [1, 2, 3] {
        let (mut p, man, _) = forty_stage(5.0, s);
        let (_, _, arm) = p.forty();
        let at = p.chest(man);
        right_click(&mut p, at);
        let mut folded = false;
        for _ in 0..60 { p.frame(); folded |= p.folded_by_a_blow(man); }
        assert!(folded, "seed {s}: he did not fold");
        assert_eq!(p.blasts(), 0, "seed {s}: a blast at five metres");
        assert!(p.up(man), "seed {s}: a dud does no damage");
        assert!(p.awareness(man) >= 0.8, "seed {s}: he knows now: {}", p.awareness(man));
        let (armed, flown, on_man) = p.forty_end().expect("the round's end is logged");
        assert!(!armed && on_man && flown < arm, "seed {s}: armed {armed}, on a man {on_man}, flown {flown:.2}");
        assert_eq!(p.forty_duds(), 1, "seed {s}: one lump on the floor");
        p.run(6.0);
        assert_eq!((p.blasts(), p.forty_duds()), (0, 1), "seed {s}: the dud did not stay inert");
    }
}

/// THE RULE: a wall three metres off is a dud with no blast; a wall nine metres off is a burst.
/// THE CONTROL: the same tube, the same wall: the only difference is the distance.
/// A FAILURE MEANS: the arming distance has stopped being a distance.
#[test]
fn a_wall_is_a_dud_near_and_a_burst_far() {
    for s in [1, 2, 3] {
        let (mut p, man, z) = forty_stage(20.0, s);
        p.man_posted(man);
        right_click(&mut p, [10.0, 1.2, z]);
        p.run(1.0);
        let (armed, flown, _) = p.forty_end().expect("near: the round's end is logged");
        assert!(!armed && flown < 4.0, "seed {s}: the near wall: armed {armed} after {flown:.2} m");
        assert_eq!(p.blasts(), 0, "seed {s}: the near wall made a blast");
        let ends = p.forty_ends();
        p.sam_put([19.0, z]);
        p.run(3.0);
        right_click(&mut p, [10.0, 1.2, z]);
        p.run(1.0);
        assert_eq!(p.forty_ends(), ends + 1, "seed {s}: the second round went nowhere");
        let (armed, flown, _) = p.forty_end().unwrap();
        assert!(armed && flown > 7.0, "seed {s}: the far wall: armed {armed} after {flown:.2} m");
        assert_eq!(p.blasts(), 1, "seed {s}: the far wall made no blast");
    }
}
