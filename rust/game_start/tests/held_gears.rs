//! HOW HE MOVES: two gears and a dash in a crouch, what each is heard as, and one footfall a step.
mod kit_ask;
use game_start::play::{nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use kit_ask::KitAsk;

/// A long lit room with nobody in it, Sam at its west end, the camera square so that D walks him east.
fn floor(s: u32) -> Play {
    let mut sc = Scene::new("gears");
    let z = 14.0 + 0.5 * nudge(s, 0);
    sc.room("room", 4.0, 10.0, 46.0, 18.0).door("south", Axis::X, 18.0, 20.0).bulb(16.0, 14.0).bulb(34.0, 14.0).sam(6.0, z);
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.sam_a_ghost();
    p.camera_square();
    p.run(0.2);
    p
}

/// Four seconds of D with these keys down: the pace he settles at, the loudest footfall filed, and whether he is still low.
fn walked(s: u32, low: bool, shift: bool) -> (f64, f64, Play) {
    let mut p = floor(s);
    if low { p.tap("KeyC"); p.run(0.6); }
    let x = p.sam_at()[0] + 40.0;
    p.point_at([x, p.sam_at()[1]]);
    p.key_down("KeyD");
    if shift { p.key_down("ShiftLeft"); }
    p.run(1.0);
    let x0 = p.sam_at()[0];
    let mut loud = 0.0f64;
    for _ in 0..(3 * 60) {
        p.frame();
        for l in p.footfalls_now() { loud = loud.max(l); }
    }
    (((p.sam_at()[0] - x0) / 3.0), loud, p)
}

/// THE RULE: the crouch walk settles at the table's own pace, over a patrol's walk and under his walk; his walk and sprint settle at theirs.
/// THE CONTROL: each pace against the next: the test is only as good as the three being apart.
/// A FAILURE MEANS: a crouched man can no longer gain on a patrol, or a crouch walks as fast as a walk.
#[test]
fn the_gears_settle_at_their_paces() {
    for s in [1, 2, 3] {
        let (crouch, walk, sprint, guard) = { let p = floor(s); let t = p.paces(); (t.0, t.2, t.3, t.4) };
        let (low, ..) = walked(s, true, false);
        let (stood, ..) = walked(s, false, false);
        let (ran, ..) = walked(s, false, true);
        assert!((low - crouch).abs() < 0.05, "seed {s}: the crouch walk {low:.2}, the table {crouch}");
        assert!(guard < crouch && crouch < walk, "seed {s}: a patrol {guard}, a crouch {crouch}, a walk {walk}");
        assert!((stood - walk).abs() < 0.05, "seed {s}: a walk {stood:.2}, the table {walk}");
        assert!((ran - sprint).abs() < 0.1, "seed {s}: a sprint {ran:.2}, the table {sprint}");
    }
}

/// THE RULE: Shift in a crouch keeps him down, is no sprint, covers the table's pace between his crouch walk and his sprint, and is heard louder than a step of a crouch walk or a walk and quieter than one of a sprint; it is no sneak, where the crouch walk is.
/// THE CONTROL: C with Shift held stands him and he sprints.
/// A FAILURE MEANS: there is no way to hurry low, or a dash in a crouch is as quiet as a creep.
#[test]
fn shift_in_a_crouch_is_a_fast_crouch_walk() {
    for s in [1, 2, 3] {
        let (crouch_s, dash, walk, sprint, _) = floor(s).paces();
        let (low, low_loud, p) = walked(s, true, false);
        assert!(p.sam_sneaking(), "seed {s}: the crouch walk is the sneak");
        drop(p);
        let (_, walk_loud, p) = walked(s, false, false);
        drop(p);
        let (_, run_loud, p) = walked(s, false, true);
        drop(p);
        let (fast, fast_loud, mut p) = walked(s, true, true);
        assert!(p.sam_is_low() && !p.sam_sprinting(), "seed {s}: he stays down and is not sprinting: low {} sprint {}", p.sam_is_low(), p.sam_sprinting());
        assert!(!p.sam_sneaking(), "seed {s}: it is no sneak");
        assert!((fast - dash).abs() < 0.1 && fast > low && fast > crouch_s && dash < sprint && dash > walk * 0.5, "seed {s}: dash {fast:.2} (the table {dash}), crouch {low:.2}, sprint {sprint}");
        assert!(fast_loud > low_loud && fast_loud > walk_loud && fast_loud < run_loud, "seed {s}: heard crouch {low_loud:.3}, walk {walk_loud:.3}, dash {fast_loud:.3}, run {run_loud:.3}");
        p.tap("KeyC");
        p.until(1.0, |p| p.sam_sprinting()).unwrap_or_else(|| panic!("seed {s}, control: C with Shift held: no sprint"));
        assert!(!p.sam_is_low(), "seed {s}, control: he stands to sprint");
    }
}

/// THE RULE: with the forward key and the back key by turns, a frame each, the walk cycle rocks over one edge and still files at most two footfalls in a third of a second, none on the next frame after another.
/// THE CONTROL: walked straight on, the same man files them at his own stride: more than one in two seconds, and not rocked.
/// A FAILURE MEANS: seven steps in a tenth of a second to the ears of the man he is creeping up on.
#[test]
fn rocking_keys_file_one_footfall_a_step() {
    for s in [1, 2, 3] {
        let mut p = floor(s);
        p.tap("KeyC");
        p.run(0.6);
        p.point_at([p.sam_at()[0] + 6.0, p.sam_at()[1]]);
        p.key_down("KeyD");
        let (mut steady, mut from) = (0, None);
        for i in 0..240 {
            p.frame();
            let n = p.footfalls_now().len();
            steady += n;
            if i > 40 && n > 0 { from = Some(i); break; }
        }
        assert!(from.is_some() && steady >= 1, "seed {s}, control: he files no footfall walking");
        p.point_at([p.sam_at()[0] + 5.0, p.sam_at()[1]]);
        let (mut got, mut last) = (0, -10);
        for i in 0..20 {
            let (on, off) = if i % 2 == 0 { ("KeyA", "KeyD") } else { ("KeyD", "KeyA") };
            p.key_up(off);
            p.key_down(on);
            p.frame();
            let n = p.footfalls_now().len();
            if n > 0 {
                assert!(i - last > 1, "seed {s}: footfalls on frames next to each other, {last} and {i}");
                last = i;
            }
            got += n;
        }
        assert!(got <= 2, "seed {s}: {got} footfalls in a third of a second");
    }
}
