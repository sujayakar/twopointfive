//! WHAT A PLAYER SEES OF A POSE: feet that stand where they were put, a step that lands, a hand that stays on its gun, a lean that comes back,
//! joints that bend as joints do, nothing that jumps from one frame to the next, and a man who falls lying on the floor all in one piece. All of it
//! is read off the joints the picture is drawn from, on the real rig, whatever layers of the animator got them there.
mod kit_ask;
use game::character::Bone;
use game::ids::CharId;
use game_start::play::{nudge, Button, Opts, Play};
use game_start::scene::{Axis, Scene};
use kit_ask::KitAsk;

const SEEDS: [u32; 3] = [1, 2, 3];
type P3 = [f64; 3];

fn sub(a: P3, b: P3) -> P3 { [a[0] - b[0], a[1] - b[1], a[2] - b[2]] }
fn dot(a: P3, b: P3) -> f64 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }
fn dist(a: P3, b: P3) -> f64 { let v = sub(a, b); dot(v, v).sqrt() }
fn flat(a: P3, b: P3) -> f64 { ((a[0] - b[0]).powi(2) + (a[2] - b[2]).powi(2)).sqrt() }
/// the angle at `at` between two others, in degrees
fn bend(a: P3, at: P3, b: P3) -> f64 { (dot(sub(a, at), sub(b, at)) / (dist(a, at) * dist(b, at))).clamp(-1.0, 1.0).acos().to_degrees() }

/// A long lit room with nobody near, Sam at its west end, the camera square so that D walks him east, and a man on a patrol far off along its north wall.
fn floor(s: u32) -> Play {
    let mut sc = Scene::new("poses");
    let z = 14.0 + 0.5 * nudge(s, 0);
    sc.room("room", 4.0, 10.0, 66.0, 18.0).door("south", Axis::X, 18.0, 20.0).bulb(16.0, 14.0).bulb(34.0, 14.0).bulb(52.0, 14.0).sam(6.0, z);
    sc.patrol("post", &[[50.0, 11.0 + 0.3 * nudge(s, 1)], [64.0, 11.0]], 0.0);
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.sam_a_ghost();
    p.camera_square();
    p.run(0.2);
    p
}

/// His rifle up, the cursor far ahead of him down the room.
fn rifle_up(s: u32) -> (Play, CharId) {
    let mut p = floor(s);
    p.sam_takes_his_rifle();
    aim_east(&mut p);
    p.run(0.5);
    let c = p.sam_body();
    (p, c)
}
fn aim_east(p: &mut Play) { let (x, z) = (p.sam_at()[0] + 60.0, p.sam_at()[1]); p.point_at([x, z]); }

/// The ways a man goes along the floor, each a settled second in.
#[derive(Clone, Copy, Debug)]
enum Gait { Walk, Sprint }
fn going(s: u32, g: Gait) -> (Play, CharId) {
    let (mut p, c) = rifle_up(s);
    p.key_down("KeyD");
    if let Gait::Sprint = g { p.key_down("ShiftLeft"); }
    p.run(1.0);
    (p, c)
}

/// One foot over a stretch of walking.
#[derive(Default)]
struct Foot { down: Option<P3>, slid: f64, top: f64, under: f64, landed: Vec<usize> }
const ON_THE_FLOOR: f64 = 0.02;
impl Foot {
    fn see(&mut self, toe: P3, frame: usize) {
        self.top = self.top.max(toe[1]);
        self.under = self.under.min(toe[1]);
        if toe[1] >= ON_THE_FLOOR { self.down = None; return; }
        match self.down {
            None => { self.down = Some(toe); self.landed.push(frame); }
            Some(at) => self.slid = self.slid.max(flat(toe, at)),
        }
    }
}
/// `frames` of a man going along, his two feet watched, and each footfall the game filed.
fn feet(p: &mut Play, c: CharId, frames: usize) -> ([Foot; 2], Vec<usize>) {
    let mut f = [Foot::default(), Foot::default()];
    let mut heard = Vec::new();
    for i in 0..frames {
        p.frame();
        f[0].see(p.joint(c, Bone::ToeL), i);
        f[1].see(p.joint(c, Bone::ToeR), i);
        if !p.footfalls_now().is_empty() { heard.push(i); }
    }
    (f, heard)
}

/// THE RULE: while a man walks or sprints at a steady pace, a foot on the floor stays where it was put: it may roll over its toe, and his body goes on over it, but it does not slide.
/// THE CONTROL: he has gone a good way in that time, and each foot has been down over and over.
/// A FAILURE MEANS: he skates: his stride and his pace have come apart.
#[test]
fn a_foot_on_the_floor_stays_where_it_was_put() {
    for s in SEEDS {
        for g in [Gait::Walk, Gait::Sprint] {
            let (mut p, c) = going(s, g);
            let x0 = p.sam_at()[0];
            let (f, _) = feet(&mut p, c, 180);
            assert!(p.sam_at()[0] - x0 > 3.0, "seed {s} {g:?}: he went {:.2} m", p.sam_at()[0] - x0);
            for (name, foot) in [("left", &f[0]), ("right", &f[1])] {
                assert!(foot.landed.len() >= 3, "seed {s} {g:?}: his {name} foot came down {} times", foot.landed.len());
                assert!(foot.slid < 0.08, "seed {s} {g:?}: his {name} foot slid {:.3} m on the floor", foot.slid);
            }
        }
        // and a man of the shift on his rounds
        let mut p = floor(s);
        let post = p.guard("post");
        p.run(2.0);
        let c = p.body_of(post);
        let x0 = p.at(post)[0];
        let (f, _) = feet(&mut p, c, 180);
        assert!((p.at(post)[0] - x0).abs() > 2.0, "seed {s}: his round took him {:.2} m", p.at(post)[0] - x0);
        for foot in &f { assert!(foot.landed.len() >= 2 && foot.slid < 0.08, "seed {s}: on his rounds a foot came down {} times and slid {:.3} m", foot.landed.len(), foot.slid); }
    }
}

/// THE RULE: a foot that is down stays down through a change of what he is doing: setting off, stopping, turning. It goes somewhere else through the air.
/// THE CONTROL: each foot has somewhere to go in each: he ends up elsewhere, or facing elsewhere, and his feet are not where they were.
/// A FAILURE MEANS: his feet slide over the floor into the next pose.
#[test]
fn a_foot_down_stays_down_as_he_sets_off_stops_and_turns() {
    for s in SEEDS {
        let (mut p, c) = rifle_up(s);
        let toes = |p: &Play| [p.joint(c, Bone::ToeL), p.joint(c, Bone::ToeR)];
        let mut check = |p: &mut Play, what: &str, frames: usize| {
            let was = toes(p);
            let (f, _) = feet(p, c, frames);
            for foot in &f { assert!(foot.slid < 0.05, "seed {s} {what}: a foot slid {:.3} m on the floor", foot.slid); }
            let now = toes(p);
            assert!(flat(now[0], was[0]).max(flat(now[1], was[1])) > 0.1, "seed {s} {what}: his feet stayed where they were");
        };
        p.key_down("KeyD");
        p.run(2.0);
        p.key_up("KeyD");
        check(&mut p, "stopping", 60);
        let (x, z) = (p.sam_at()[0], p.sam_at()[1]);
        p.point_at([x - 60.0, z]);
        check(&mut p, "turning away", 90);
        p.point_at([x + 9.0, z]);
        check(&mut p, "turning back", 90);
    }
}

/// THE RULE: he steps one foot and then the other, each lifted clear of the floor and set down on it and not through it, and what the game hears is one footfall a step, as a foot comes down.
/// THE CONTROL: a sprint lifts his feet higher than a walk.
/// A FAILURE MEANS: his feet drag, or go through the floor, or the sound of a step has come apart from the step.
#[test]
fn a_step_lands_one_foot_after_the_other_and_is_heard_as_one() {
    for s in SEEDS {
        let mut lift = [0.0; 2];
        for (i, g) in [Gait::Walk, Gait::Sprint].iter().enumerate() {
            let (mut p, c) = going(s, *g);
            let (f, heard) = feet(&mut p, c, 240);
            let mut all: Vec<(usize, usize)> = f[0].landed.iter().map(|&t| (t, 0)).chain(f[1].landed.iter().map(|&t| (t, 1))).collect();
            all.sort();
            assert!(all.windows(2).all(|w| w[0].1 != w[1].1), "seed {s} {g:?}: the same foot came down twice running: {all:?}");
            for foot in &f {
                assert!(foot.top > 0.06, "seed {s} {g:?}: a foot lifted only {:.3} m", foot.top);
                assert!(foot.under > -0.01, "seed {s} {g:?}: a toe went {:.3} m through the floor", foot.under);
            }
            lift[i] = f[0].top.min(f[1].top);
            assert!((heard.len() as i64 - all.len() as i64).abs() <= 1, "seed {s} {g:?}: {} steps and {} footfalls", all.len(), heard.len());
            for h in heard { assert!(all.iter().any(|l| l.0.abs_diff(h) <= 8) || h <= 8 || h >= 232, "seed {s} {g:?}: a footfall at frame {h} with no foot coming down: {all:?}"); }
        }
        assert!(lift[1] > lift[0], "seed {s}: a sprint lifts {:.3} m, a walk {:.3} m", lift[1], lift[0]);
    }
}

/// How far his left hand stands from his right and from the gun's line, the least and the most over a stretch.
#[derive(Clone, Copy)]
struct Grip { apart: (f64, f64), off: (f64, f64) }
fn grip(p: &mut Play, c: CharId, frames: usize) -> Grip {
    let mut g = Grip { apart: (9.0, 0.0), off: (9.0, 0.0) };
    for _ in 0..frames {
        p.frame();
        let hand = p.joint(c, Bone::HandL);
        let a = dist(hand, p.joint(c, Bone::HandR));
        let (mz, d) = p.gun_line(c);
        let v = sub(hand, mz);
        let along = dot(v, d);
        let o = dist(v, [d[0] * along, d[1] * along, d[2] * along]);
        g.apart = (g.apart.0.min(a), g.apart.1.max(a));
        g.off = (g.off.0.min(o), g.off.1.max(o));
    }
    g
}

/// THE RULE: with his rifle up his left hand stays on it, the same place on the gun, whatever the rest of him does: standing, walking, in a sprint, low, turning, firing.
/// THE CONTROL: a reload does take it off, and it comes back to the same place.
/// A FAILURE MEANS: a hand floats off a gun, or through it.
#[test]
fn his_left_hand_stays_on_his_rifle() {
    for s in SEEDS {
        let (mut p, c) = rifle_up(s);
        let held = grip(&mut p, c, 30);
        let mut steps: Vec<(&str, Grip)> = Vec::new();
        p.key_down("KeyD");
        steps.push(("walking", grip(&mut p, c, 120)));
        p.key_down("ShiftLeft");
        steps.push(("sprinting", grip(&mut p, c, 120)));
        p.key_up("ShiftLeft");
        p.key_up("KeyD");
        steps.push(("stopping", grip(&mut p, c, 60)));
        let (x, z) = (p.sam_at()[0], p.sam_at()[1]);
        p.point_at([x - 40.0, z]);
        steps.push(("turning", grip(&mut p, c, 60)));
        aim_east(&mut p);
        steps.push(("turning back", grip(&mut p, c, 60)));
        p.tap("KeyC");
        steps.push(("going low", grip(&mut p, c, 60)));
        p.key_down("KeyD");
        steps.push(("low, walking", grip(&mut p, c, 90)));
        p.key_up("KeyD");
        p.tap("KeyC");
        steps.push(("standing up", grip(&mut p, c, 60)));
        p.press(Button::Left);
        steps.push(("firing", grip(&mut p, c, 30)));
        p.release(Button::Left);
        steps.push(("after", grip(&mut p, c, 30)));
        for (what, g) in steps {
            assert!(g.apart.1 - held.apart.0 < 0.01 && held.apart.1 - g.apart.0 < 0.01, "seed {s} {what}: his hands were {:.3} to {:.3} m apart, and {:.3} standing", g.apart.0, g.apart.1, held.apart.0);
            assert!(g.off.1 < held.off.1 + 0.01, "seed {s} {what}: his hand went {:.3} m off the gun's line, and {:.3} standing", g.off.1, held.off.1);
        }
        p.tap("KeyR");
        let change = grip(&mut p, c, 180);
        assert!(change.apart.1 > held.apart.1 + 0.1, "seed {s}: a reload moved his hand only as far as {:.3} m", change.apart.1);
        let again = grip(&mut p, c, 10);
        assert!((again.apart.0 - held.apart.0).abs() < 0.01, "seed {s}: after the reload his hands are {:.3} m apart, and were {:.3}", again.apart.0, held.apart.0);
    }
}

/// THE RULE: a lean round an edge takes his head out a good way, gets there and comes all the way back when it is let go, with no jump either way, and with nothing left of it once it is back.
/// A FAILURE MEANS: he stays tilted after a peek, or pops out and back.
#[test]
fn a_lean_goes_out_and_comes_all_the_way_back() {
    for s in SEEDS {
        let (mut p, c) = rifle_up(s);
        p.run(1.0);
        let head = |p: &Play| sub(p.joint(c, Bone::Head), p.joint(c, Bone::Hips));
        let (mut most, mut step) = (0.0f64, 0.0f64);
        let was = head(&p);
        for by in [1.0, 0.0] {
            p.lean(c, by);
            let mut last = head(&p);
            for _ in 0..60 {
                p.frame();
                let h = head(&p);
                step = step.max(dist(h, last));
                most = most.max(dist(h, was));
                last = h;
            }
            if by == 1.0 { assert!(p.lean_shown(c) > 0.9, "seed {s}: a second on he is only {:.2} of the way out", p.lean_shown(c)); }
        }
        assert!(most > 0.1, "seed {s}: his head went {most:.3} m");
        assert!(step < 0.03, "seed {s}: his head moved {step:.3} m in a frame");
        assert_eq!(p.lean_shown(c), 0.0, "seed {s}: the lean is not over");
        assert!(dist(head(&p), was) < 0.04, "seed {s}: his head came back to {:.3} m off where it was", dist(head(&p), was));
    }
}

/// The whole of what a man does on his feet, handed to `each` a frame at a time.
fn a_full_day(s: u32, mut each: impl FnMut(&Play, CharId, &str)) {
    let (mut p, c) = rifle_up(s);
    let mut go = |p: &mut Play, what: &str, secs: f64| { for _ in 0..(secs * 60.0) as usize { p.frame(); each(p, c, what); } };
    go(&mut p, "standing", 1.0);
    p.key_down("KeyD");
    go(&mut p, "setting off", 2.0);
    p.key_down("ShiftLeft");
    go(&mut p, "going into a sprint", 2.0);
    p.key_up("ShiftLeft");
    go(&mut p, "coming out of a sprint", 1.5);
    p.key_up("KeyD");
    go(&mut p, "stopping", 1.5);
    // a turn on the spot, right round and back to a mark not far ahead of him, then straight off low
    let (back, z) = (p.sam_at()[0] + 9.0, p.sam_at()[1]);
    p.point_at([back - 60.0, z]);
    go(&mut p, "turning away", 1.5);
    p.point_at([back, z]);
    go(&mut p, "turning back", 1.5);
    p.tap("KeyC");
    go(&mut p, "going low", 1.0);
    p.key_down("KeyD");
    go(&mut p, "low, setting off", 2.0);
    p.key_up("KeyD");
    go(&mut p, "low, stopping", 1.0);
    p.tap("KeyC");
    go(&mut p, "standing up", 1.0);
    p.press(Button::Left);
    go(&mut p, "firing", 0.5);
    p.release(Button::Left);
    go(&mut p, "after firing", 1.0);
    p.tap("KeyR");
    go(&mut p, "reloading", 3.0);
    p.tap("Digit1");
    go(&mut p, "taking the pistol", 1.5);
    p.tap("Digit2");
    go(&mut p, "taking the rifle", 1.5);
}

/// THE RULE: a limb is the length it is and bends as a limb bends: a knee never past straight or folded flat, and only forward; an elbow the same.
/// A FAILURE MEANS: a layer has pulled a limb long, or bent a knee backward.
#[test]
fn joints_keep_their_length_and_bend_only_as_joints_do() {
    for s in SEEDS {
        let mut rest: Option<[f64; 6]> = None;
        let (mut knee, mut elbow, mut ahead) = ((999.0f64, 0.0f64), (999.0f64, 0.0f64), 9.0f64);
        a_full_day(s, |p, c, what| {
            let j = |b: Bone| p.joint(c, b);
            let lens = [dist(j(Bone::ThighL), j(Bone::ShinL)), dist(j(Bone::ShinL), j(Bone::FootL)), dist(j(Bone::ThighR), j(Bone::ShinR)), dist(j(Bone::ShinR), j(Bone::FootR)), dist(j(Bone::UpperArmL), j(Bone::ForearmL)), dist(j(Bone::UpperArmR), j(Bone::ForearmR))];
            let was = *rest.get_or_insert(lens);
            for i in 0..6 { assert!((lens[i] - was[i]).abs() < 1e-3, "seed {s} {what}: limb {i} is {:.4} m, it was {:.4}", lens[i], was[i]); }
            let yaw = p.body_yaw(c);
            let front = [yaw.sin(), 0.0, yaw.cos()];
            for (a, b, e) in [(Bone::ThighL, Bone::ShinL, Bone::FootL), (Bone::ThighR, Bone::ShinR, Bone::FootR)] {
                let k = bend(j(a), j(b), j(e));
                knee = (knee.0.min(k), knee.1.max(k));
                let mid = [(j(a)[0] + j(e)[0]) / 2.0, (j(a)[1] + j(e)[1]) / 2.0, (j(a)[2] + j(e)[2]) / 2.0];
                ahead = ahead.min(dot(sub(j(b), mid), front));
                assert!(k > 25.0 && k < 179.0, "seed {s} {what}: a knee at {k:.0} degrees");
            }
            for (a, b, e) in [(Bone::UpperArmL, Bone::ForearmL, Bone::HandL), (Bone::UpperArmR, Bone::ForearmR, Bone::HandR)] {
                let k = bend(j(a), j(b), j(e));
                elbow = (elbow.0.min(k), elbow.1.max(k));
                assert!(k > 25.0 && k < 179.0, "seed {s} {what}: an elbow at {k:.0} degrees");
            }
        });
        assert!(ahead > 0.0, "seed {s}: a knee went {:.3} m behind the line from his hip to his ankle", -ahead);
    }
}

/// THE RULE: in every gait a foot comes off the floor to go anywhere: it does not slide while it is on the floor, at mid swing its toe is well over the floor, and the toe is never under it.
/// THE CONTROL: a foot has somewhere to go in each: the bound on the top is the foot leaving the floor at all.
/// A FAILURE MEANS: he drags his feet, most of all crouched, or stubs them through the floor.
#[test]
fn a_foot_clears_the_floor_in_every_gait() {
    for s in SEEDS {
        let check = |p: &mut Play, c: CharId, what: &str, frames: usize, most: f64| {
            let (f, _) = feet(p, c, frames);
            for foot in &f {
                assert!(foot.slid < most, "seed {s} {what}: a foot slid {:.3} m on the floor", foot.slid);
                assert!(foot.top > 0.03, "seed {s} {what}: a toe came up {:.3} m at most", foot.top);
                assert!(foot.under > -0.01, "seed {s} {what}: a toe went {:.3} m under the floor", -foot.under);
            }
        };
        let (mut p, c) = rifle_up(s);
        p.key_down("KeyD");
        check(&mut p, c, "setting off", 60, 0.05);
        check(&mut p, c, "walking", 180, 0.05);
        p.key_up("KeyD");
        check(&mut p, c, "stopping", 60, 0.06);
        p.key_down("KeyD"); p.key_down("ShiftLeft");
        check(&mut p, c, "going to a sprint", 60, 0.05);
        check(&mut p, c, "sprinting", 180, 0.05);
        p.key_up("ShiftLeft"); p.key_up("KeyD");
        // (the last foot is still turning flat as it lands, and its toe comes forward with it)
        check(&mut p, c, "stopping a sprint", 60, 0.06);
        drop(p);
        let (mut p, c) = rifle_up(s);
        p.tap("KeyC");
        p.run(0.8);
        p.key_down("KeyD");
        check(&mut p, c, "low, setting off", 60, 0.05);
        check(&mut p, c, "low, walking", 180, 0.05);
        p.key_up("KeyD");
        check(&mut p, c, "low, stopping", 60, 0.05);
    }
}

/// THE RULE: a fallen man's toes stay over his floor, from the moment he falls until he lies still, on his round or off it.
/// THE CONTROL: he has fallen: his hips are down. (Off his round: Sam's shots have sent him away from it, where a man used to come to rest with a foot through the floor.)
/// A FAILURE MEANS: a dead man's foot goes through the floor.
#[test]
fn a_fallen_mans_toes_stay_over_the_floor() {
    for s in SEEDS {
        for (wait, shots) in [(2.0, false), (8.0, false), (14.0, false), (20.0, false), (26.0, false), (32.0, false), (6.0, true)] {
            let mut p = floor(s);
            let post = p.guard("post");
            if shots { p.sam_takes_his_rifle(); p.run(1.5); p.press(Button::Left); p.run(0.5); p.release(Button::Left); }
            p.run(wait);
            let c = p.body_of(post);
            p.falls(post, game::game::DownBy::Player);
            for _ in 0..300 {
                p.frame();
                for b in [Bone::ToeL, Bone::ToeR] {
                    let y = p.joint(c, b)[1];
                    assert!(y > -0.01, "seed {s}, {wait} s on: a fallen man's toe is {:.3} m under the floor", -y);
                }
            }
            assert!(p.joint(c, Bone::Hips)[1] < 0.4, "seed {s}, {wait} s on: his hips are at {:.2} m", p.joint(c, Bone::Hips)[1]);
        }
    }
}

/// THE RULE: however he goes from one thing to another, no joint moves more in one frame than a limb can: a change of what he is doing is a blend and not a cut. (Measured from his hips, so what is left is the pose.)
/// A FAILURE MEANS: a pop on the screen as a man starts, stops, goes low or changes guns.
#[test]
fn nothing_jumps_from_one_frame_to_the_next() {
    for s in SEEDS {
        let mut last: Option<Vec<(Bone, P3)>> = None;
        let mut feet_last: Option<[P3; 4]> = None;
        a_full_day(s, |p, c, what| {
            let hips = p.joint(c, Bone::Hips);
            let now: Vec<(Bone, P3)> = p.joints_named(c).into_iter().map(|(b, at)| (b, sub(at, hips))).collect();
            // a foot, from where he stands: seen from the hips it also moves as the hips do, and they come up 12 cm in the first frame of standing up. (A toe also turns about its ankle: a centimetre more.)
            let here = p.g.ch[c].pos;
            let feet_now = [Bone::FootL, Bone::ToeL, Bone::FootR, Bone::ToeR].map(|b| sub(p.joint(c, b), here));
            if let Some(was) = &feet_last {
                for i in 0..4 { assert!(dist(feet_now[i], was[i]) < [0.12, 0.13][i % 2], "seed {s} {what}: a foot (joint {i}) moved {:.3} m in a frame", dist(feet_now[i], was[i])); }
            }
            feet_last = Some(feet_now);
            if let Some(was) = &last {
                for (a, b) in now.iter().zip(was.iter()) {
                    let d = dist(a.1, b.1);
                    let leg = matches!(a.0, Bone::ThighL | Bone::ShinL | Bone::FootL | Bone::ToeL | Bone::ThighR | Bone::ShinR | Bone::FootR | Bone::ToeR);
                    assert!(d < if leg { 0.2 } else { 0.15 }, "seed {s} {what}: his {:?} moved {d:.3} m in a frame", a.0);
                }
            }
            last = Some(now);
        });
    }
}

/// THE RULE: he turns on the spot by stepping round: no toe goes further in a frame than a foot can, however far he has to come round.
/// (The way in matters: a walk, a sprint and a stop, then right round and back to a mark not far ahead of him, is what used to set a foot off half a metre at a stroke.)
/// A FAILURE MEANS: a foot pops to its place as he turns.
#[test]
fn he_turns_on_the_spot_by_stepping_round() {
    for s in SEEDS {
        let mut p = floor(s);
        let c = p.sam_body();
        p.sam_takes_his_rifle();
        let (x, z) = (p.sam_at()[0] + 30.0, p.sam_at()[1]);
        p.point_at([x, z]);
        p.run(1.5);
        p.key_down("KeyD"); p.run(3.0);
        p.key_down("ShiftLeft"); p.run(3.0);
        p.key_up("ShiftLeft"); p.run(1.5);
        p.key_up("KeyD"); p.run(1.5);
        for at in [x - 60.0, x] {
            p.point_at([at, z]);
            let toes = |p: &Play| [p.joint(c, Bone::ToeL), p.joint(c, Bone::ToeR)];
            let mut last = toes(&p);
            for _ in 0..90 {
                p.frame();
                let now = toes(&p);
                for i in 0..2 { assert!(flat(now[i], last[i]) < 0.12, "seed {s}: toe {i} went {:.3} m in a frame, turning to {at:.0}", flat(now[i], last[i])); }
                last = now;
            }
        }
    }
}

/// THE RULE: a man who falls ends up lying on the floor, still, all in one piece.
/// A FAILURE MEANS: a body stands, hangs in the air, goes on twitching for ever or comes apart.
#[test]
fn a_man_who_falls_lies_still_on_the_floor_with_his_limbs_whole() {
    for s in SEEDS {
        let mut p = floor(s);
        let post = p.guard("post");
        p.run(2.0);
        let c = p.body_of(post);
        let lens = |p: &Play| [dist(p.joint(c, Bone::ThighL), p.joint(c, Bone::ShinL)), dist(p.joint(c, Bone::ShinR), p.joint(c, Bone::FootR)), dist(p.joint(c, Bone::UpperArmL), p.joint(c, Bone::ForearmL))];
        let up = lens(&p);
        let stood = p.joint(c, Bone::Head)[1];
        assert!(stood > 1.4, "seed {s}: his head is at {stood:.2} m before he falls");
        p.falls(post, game::game::DownBy::Player);
        let mut step = 0.0f64;
        let mut last = p.joint(c, Bone::Hips);
        for _ in 0..480 {
            p.frame();
            let h = p.joint(c, Bone::Hips);
            step = step.max(dist(h, last));
            last = h;
            for (a, b) in lens(&p).iter().zip(up.iter()) { assert!((a - b).abs() < 2e-3, "seed {s}: a limb is {a:.4} m, it was {b:.4}"); }
        }
        let lying = p.joints_named(c);
        p.run(1.0);
        let moved = lying.iter().zip(p.joints_named(c).iter()).map(|(a, b)| dist(a.1, b.1)).fold(0.0f64, f64::max);
        let (head, hips) = (p.joint(c, Bone::Head)[1], p.joint(c, Bone::Hips)[1]);
        assert!(step < 0.2, "seed {s}: his hips moved {step:.3} m in a frame");
        assert!(moved < 5e-3, "seed {s}: eight seconds on he is still moving, {moved:.4} m in a second");
        assert!(head < 0.4 && hips < 0.4, "seed {s}: he lies with his head at {head:.2} m and his hips at {hips:.2} m");
        assert!(head > 0.0 && hips > 0.0, "seed {s}: under the floor: {head:.2}, {hips:.2}");
    }
}
