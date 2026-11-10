//! A SPRINT HAS WEIGHT: into a door a shoulder or a boot, by the way it opens; straight at a wall, the wall is taken.
mod kit_ask;
use game::ids::DoorId;
use game_start::play::{nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use kit_ask::KitAsk;

/// Two rooms with one door in the wall between them, three metres of floor before both faces, Sam nobody can see, the camera square.
fn two_rooms(s: u32) -> (Play, DoorId, f64) {
    let mut sc = Scene::new("sprint");
    let z = 12.4 + 0.3 * nudge(s, 0);
    sc.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0).door("between", Axis::Z, 18.0, z).bulb(14.0, 13.0).bulb(22.0, 13.0).sam(11.0, 15.0);
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.sam_a_ghost();
    p.camera_square();
    p.run(0.2);
    let d = p.door("between");
    (p, d, z)
}

/// Where three metres off a face of it is: the face the leaf opens toward, or the other.
fn start_at(p: &Play, d: DoorId, toward: bool) -> [f64; 2] {
    let w = p.leaf_w(d);
    let a = p.leaf_pt(d, w * 0.5, 3.0);
    let b = p.leaf_pt(d, w * 0.5, -3.0);
    if p.opens_toward(d, a) == toward { a } else { b }
}

#[derive(Debug)]
struct Run { before: f64, slowest: f64, back_in: Option<f64>, crossed: Option<f64>, kicked: bool, ended: f64 }

/// W from three metres, the camera square on it: Shift held too if `sprint`. How fast he came, how fast he kept going after the door, when he crossed.
fn run_at(p: &mut Play, d: DoorId, from: [f64; 2], sprint: bool) -> Run {
    let mid = p.frame_mid(d);
    p.sam_put(from);
    p.run(0.1);
    let key = if mid[0] > from[0] { "KeyD" } else { "KeyA" };
    p.key_down(key);
    if sprint { p.key_down("ShiftLeft"); }
    let side = (from[0] - mid[0]).signum();
    let t0 = p.time();
    let (mut r, mut hit) = (Run { before: 0.0, slowest: 99.0, back_in: None, crossed: None, kicked: false, ended: 0.0 }, None);
    let counts0 = p.sprint_door_counts();
    for _ in 0..200 {
        p.frame();
        let (at, v) = (p.sam_at(), p.sam_speed());
        r.kicked |= p.sam_kicking();
        if hit.is_none() && (p.sprint_door_counts() != counts0 || r.kicked) { hit = Some(p.time()); }
        match hit {
            None => r.before = v.max(r.before),
            Some(h) => {
                if p.time() - h < 0.3 { r.slowest = r.slowest.min(v); }
                if r.back_in.is_none() && v > r.before - 0.2 { r.back_in = Some(p.time() - h); }
            }
        }
        if r.crossed.is_none() && (at[0] - mid[0]) * side < -0.5 { r.crossed = Some(p.time() - t0); }
    }
    r.ended = p.sam_at()[0];
    p.key_up(key);
    p.key_up("ShiftLeft");
    r
}

/// THE RULE: at the face it opens away from, a sprint is a shoulder: one bash, he keeps the table's share of his speed, has the sprint back inside half a second and is through.
/// THE CONTROL: a walk into the same leaf is no shoulder and no sprint to lose.
/// A FAILURE MEANS: a door is no more than a soft patch in the floor, or stops a running man dead.
#[test]
fn at_the_face_it_opens_away_from_a_sprint_is_a_shoulder() {
    for s in [1, 2, 3] {
        let (mut p, d, _) = two_rooms(s);
        let from = start_at(&p, d, false);
        let r = run_at(&mut p, d, from, true);
        assert_eq!(p.sprint_door_counts(), (1, 0, 0), "seed {s}: {r:?}");
        assert!(!r.kicked, "seed {s}: no boot on this face");
        assert!(r.before > p.sprint_pace() * 0.9, "seed {s}: he came at a sprint: {r:?}");
        let keep = p.shoulder_keeps();
        assert!(r.slowest > r.before * keep - 0.3 && r.slowest < r.before - 0.1, "seed {s}: he keeps {keep} of it: {r:?}");
        assert!(r.back_in.is_some_and(|t| t < 0.5), "seed {s}: the sprint back inside half a second: {r:?}");
        assert!(r.crossed.is_some(), "seed {s}: he is through: {r:?}");
        drop(p);

        let (mut p, d, _) = two_rooms(s);
        let from = start_at(&p, d, false);
        let r = run_at(&mut p, d, from, false);
        assert_eq!(p.sprint_door_counts(), (0, 0, 0), "seed {s}, control: a walk: {r:?}");
    }
}

/// THE RULE: at the face it opens toward, shut in its frame, a sprint is a boot at the run: no shoulder, the leaf off its hinges and he is through.
/// THE CONTROL: a walk at that face boots nothing.
/// A FAILURE MEANS: a running man hauls a leaf through himself, or a sprint at a pull door stops him at a shut leaf with no answer.
#[test]
fn at_the_face_it_opens_toward_a_sprint_is_a_boot() {
    for s in [1, 2, 3] {
        let (mut p, d, _) = two_rooms(s);
        let from = start_at(&p, d, true);
        let r = run_at(&mut p, d, from, true);
        assert!(r.kicked, "seed {s}: no boot: {r:?}");
        assert_eq!(p.sprint_door_counts().0, 0, "seed {s}: a shoulder too: {r:?}");
        assert!(p.door_off_hinges(d), "seed {s}: the leaf is on its hinges: {r:?}");
        assert!(r.crossed.is_some(), "seed {s}: he is not through: {r:?}");
        drop(p);

        let (mut p, d, _) = two_rooms(s);
        let from = start_at(&p, d, true);
        let r = run_at(&mut p, d, from, false);
        assert!(!r.kicked && !p.door_off_hinges(d), "seed {s}, control: a walk: {r:?}");
    }
}

/// THE RULE: locked, the boot from the face it opens away from breaks it; from the other face it is a wall and nothing broken.
/// THE CONTROL: the same door unlocked, above, goes on both faces.
/// A FAILURE MEANS: a lock holds nothing against a sprint, or a boot breaks a door from a side that cannot go.
#[test]
fn locked_a_sprint_kicks_it_in_from_one_face_and_is_stopped_at_the_other() {
    for s in [1, 2, 3] {
        let (mut p, d, _) = two_rooms(s);
        let from = start_at(&p, d, false);
        p.door_locked_to(d, from);
        let r = run_at(&mut p, d, from, true);
        assert!(r.kicked && p.door_broken(d), "seed {s}: the face it opens away from: {r:?}");
        drop(p);

        let (mut p, d, _) = two_rooms(s);
        let from = start_at(&p, d, true);
        p.door_locked_to(d, from);
        let r = run_at(&mut p, d, from, true);
        assert!(!p.door_broken(d) && !p.door_off_hinges(d), "seed {s}: the face it opens toward breaks nothing: {r:?}");
        assert!(r.crossed.is_none(), "seed {s}: he is through a locked door: {r:?}");
    }
}

fn ran_to_z(s: u32) -> f64 { 14.0 + 0.5 * nudge(s, 1) }

/// A room with seven metres of run-up to its east wall. Sam starts seven metres off.
fn at_a_wall(s: u32) -> Play {
    let mut sc = Scene::new("wall");
    let z = ran_to_z(s);
    sc.room("room", 10.0, 10.0, 26.0, 18.0).door("south", Axis::X, 18.0, 16.0).bulb(16.0, 14.0).sam(19.0, z);
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.sam_a_ghost();
    p.sam_takes_his_rifle();
    p.camera_square();
    p.run(0.2);
    p
}

/// THE RULE: a sprint straight at a wall takes it before he touches it, at his sprint the frame before, no frame of the take longer than a stride; it ends where a walk's hold ends, and the run still held does not take him off: three seconds on he is there.
/// THE CONTROL: a sprint let go three metres short of it takes nothing, and a sprint on the diagonal nothing at all.
/// A FAILURE MEANS: a man runs into the wall and stops, or a hold that cost nothing leaves him off it again.
#[test]
fn a_sprint_straight_at_a_wall_takes_it() {
    for s in [1, 2, 3] {
        let mut p = at_a_wall(s);
        p.key_down("KeyD");
        p.key_down("ShiftLeft");
        let (mut last, mut longest, mut speed_before, mut took) = (p.sam_at(), 0.0f64, 0.0, None);
        for _ in 0..(5 * 60) {
            let v = p.sam_speed();
            p.frame();
            let at = p.sam_at();
            longest = longest.max((at[0] - last[0]).hypot(at[1] - last[1]));
            last = at;
            if p.on_wall().is_some() { took = Some(at[0]); speed_before = v; break; }
        }
        let took_at = took.unwrap_or_else(|| panic!("seed {s}: he never took it: at {last:?}"));
        assert!(26.0 - took_at > 0.42, "seed {s}: he touched it first: {:.2} m off the face", 26.0 - took_at);
        assert!(speed_before > p.sprint_pace() * 0.9, "seed {s}: not at a sprint the frame before: {speed_before:.2}");
        assert!(longest < p.sprint_pace() / 60.0 * 1.6, "seed {s}: a frame of {longest:.3} m");
        p.run(3.0);
        let wall = p.on_wall().unwrap_or_else(|| panic!("seed {s}: still held, the run took him off it"));
        assert!(wall[0] < -0.9, "seed {s}: his back is to it: {wall:?}");
        let ran_to = p.sam_at()[0];
        drop(p);

        let mut p = at_a_wall(s);
        p.sam_put([25.0, ran_to_z(s)]);
        p.run(0.2);
        p.key_down("KeyD");
        p.run(0.3);
        p.tap("KeyQ");
        p.until(1.0, |p| p.on_wall().is_some()).unwrap_or_else(|| panic!("seed {s}: a walk and the key take no wall"));
        p.run(1.0);
        assert!((p.sam_at()[0] - ran_to).abs() < 0.02, "seed {s}: he ends {ran_to:.3}, a walk's hold ends {:.3}", p.sam_at()[0]);
        drop(p);

        let mut p = at_a_wall(s);
        p.key_down("KeyD");
        p.key_down("ShiftLeft");
        p.until(5.0, |p| p.sam_at()[0] > 23.0).unwrap_or_else(|| panic!("seed {s}: he never got there"));
        p.key_up("KeyD");
        p.key_up("ShiftLeft");
        assert!(p.always(1.5, |p| p.on_wall().is_none()).is_none(), "seed {s}, control: the keys let go short of it");
        drop(p);

        let mut p = at_a_wall(s);
        p.key_down("KeyD");
        p.key_down("KeyW");
        p.key_down("ShiftLeft");
        p.run(1.0);
        assert!(p.on_wall().is_none(), "seed {s}, control: on the diagonal");
    }
}
