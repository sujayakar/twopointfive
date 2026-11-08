//! HOW A MAN'S SHOT COMES TO BE: where it cannot come, how soon it may, and what a shot needs to do what it does.
mod kit_ask;
use game::game::GuardState;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use kit_ask::KitAsk;

const SEEDS: [u32; 3] = [1, 2, 3];
fn seed(seed: u32) -> Opts { Opts { seed, ..Default::default() } }

// ---------------------------------------------------------------- sight and no line of fire

/// An alert man of the office planted at a doorway with its leaf at `angle`, Sam lit and rooted beyond it. The longest he stood planted with eyes on Sam and nothing reaching
/// him, in seconds, and when his first round reached Sam (None: not in the run).
fn at_a_doorway(door: &str, angle: Option<f64>, man: [f64; 2], sam: [f64; 2], s: u32) -> (f64, Option<f64>) {
    let mut p = Play::level("office.json", false, &seed(s));
    p.sam_unhurt();
    p.run(0.17);
    let d = p.door(door);
    if let Some(a) = angle { p.leaf_left_at(d, a); }
    let gd = p.g.guards[0];
    p.all_but_held(gd);
    p.sam_put(sam);
    p.sam_holds_his_spot(man);
    p.fights_from(gd, man, p.sam_at(), 0.0, 0.0);
    let (t0, mut last_saw, mut spell, mut worst) = (p.time(), -100.0, 0.0f64, 0.0f64);
    for _ in 0..(24 * 60) {
        p.frame();
        if p.sam_just_hit() { return (worst, Some(p.time() - t0)); }
        let now = p.time();
        if p.saw_sam(gd) { last_saw = now; }
        if p.state(gd) == GuardState::Alert && p.legs(gd) < 0.25 && now - last_saw < 0.5 && now - t0 > 2.0 { spell += 1.0 / 60.0; worst = worst.max(spell); } else { spell = 0.0; }
    }
    (worst, None)
}

/// THE RULE: "the AI sometimes has its aim get stuck in doors": a man with sight of Sam through a leaf that stands in the way of his gun does not stand there firing into it. He moves
/// to where a round can go, and a round reaches Sam. Three leaves ajar; the control is a door that stands out of everybody's way.
/// A FAILURE MEANS: sight and the line of fire have parted again, and a man plants himself in a doorway for as long as Sam stays put.
#[test]
fn an_alert_man_at_a_leaf_in_his_line_does_not_stand_and_fire_into_it() {
    for (door, angle, man, sam) in [
        ("server", Some(-0.9), [17.5, 11.4], [18.9, 7.6]),
        ("server", Some(1.7), [18.6, 10.9], [16.8, 8.0]),
        ("server", Some(-1.92), [16.5, 10.9], [18.6, 8.0]),
        ("manager", None, [23.6, 11.0], [27.0, 6.5]),
    ] {
        let (worst, first) = at_a_doorway(door, angle, man, sam, 13);
        assert!(worst < 3.0, "{door} at {angle:?}: planted {worst:.1} s with eyes on Sam and nothing reaching him");
        assert!(first.is_some(), "{door} at {angle:?}: 24 s and no round of his reached a man standing still");
    }
}

// ---------------------------------------------------------------- first sight, first round

/// A long lit room with nothing in it: a sentry at its west end looking east, Sam `range` metres in front of him.
fn long_room(range: f64, s: u32) -> Scene {
    let mut sc = Scene::new("long-room");
    sc.room("room", 4.0, 10.0, 36.0, 18.0).door("south", Axis::X, 18.0, 30.0);
    for x in [8.0, 14.0, 20.0, 26.0, 32.0] { sc.bulb(x, 14.0); }
    sc.sentry("post", [6.0, 14.25], [36.0, 14.25]).sam(6.0 + range + 0.4 * nudge(s, 0), 14.25 + 0.5 * nudge(s, 1));
    sc
}

/// THE RULE: from the moment a man is alert no round leaves him before his beat, his draw and the first part of his gun's settle are over, and once settled his rounds come far closer together than that.
/// A FAILURE MEANS: a defender who has just seen Sam can drop him in the frame he saw him, which is the whole of what the settle is for.
#[test]
fn no_round_before_the_beat_and_the_settle_are_over() {
    for s in SEEDS {
        let mut p = Play::scene(&long_room(8.5, s), &seed(s));
        p.sam_unhurt();
        let post = p.guard("post");
        p.run(0.1);
        p.fights_from(post, p.at(post), p.sam_at(), 0.0, 0.35);
        p.gun_comes_round_at_once(post);
        p.frame();
        let floor = p.beat_on(post) + p.gate_after_settling(dist(p.at(post), p.sam_at()));
        let first = p.until(8.0, |p| p.rounds_fired(post) > 0.0).unwrap_or_else(|| panic!("seed {s}: no round in 8 s ({})", p.why(post)));
        assert!(first >= floor - 1.5 / 60.0, "seed {s}: first round {first:.3} s after he went alert, under the beat and the settle's {floor:.3} s");
        assert!(first <= floor + 0.35, "seed {s}: first round only {first:.3} s after he went alert: the beat and the settle are {floor:.3} s");
        let second = p.until(4.0, |p| p.rounds_fired(post) > 1.0).unwrap_or_else(|| panic!("seed {s}: no second round ({})", p.why(post)));
        assert!(second < first, "seed {s}: the control: his second round ({second:.3} s) comes sooner than his first ({first:.3} s)");
    }
}

/// THE RULE: a free man closing on a man he can see stops his feet for the shot: nearly every round leaves him at a stand. With the aim's model off (the control) his rounds leave him on the run.
/// A FAILURE MEANS: closing men fire at a run with a settled man's aim, or the brake that makes them stop has gone.
#[test]
fn a_man_who_closes_brakes_before_he_fires() {
    let frac = |model: bool, s: u32| {
        let mut p = Play::scene(&long_room(20.0, s), &seed(s));
        p.sam_unhurt();
        p.aim_model(model);
        let post = p.guard("post");
        p.until(8.0, |p| p.state(post) == GuardState::Alert).unwrap_or_else(|| panic!("seed {s}: he never saw him ({})", p.why(post)));
        let (mut slow, mut all, mut last) = (0, 0, p.rounds_fired(post));
        for _ in 0..(14 * 60) {
            p.frame();
            let n = p.rounds_fired(post);
            if n > last { all += 1; if p.speed(post) <= p.stop_to_shoot() + 1e-6 { slow += 1; } last = n; }
        }
        (slow as f64 / (all.max(1)) as f64, all)
    };
    for s in SEEDS {
        let (on, n) = frac(true, s);
        assert!(n >= 4 && on >= 0.8, "seed {s}: {n} rounds and only {:.0} % at a stand", on * 100.0);
    }
    let (off, n) = frac(false, 1);
    assert!(n >= 4 && off < 0.8, "the control: model off, {n} rounds, {:.0} % at a stand: the test has shown nothing", off * 100.0);
}

// ---------------------------------------------------------------- what a shot costs

/// A lit room, Sam 8 m from a bulb, his pistol up.
fn under_a_lamp(s: u32) -> (Play, [f64; 3], game::ids::FixtureId) {
    let mut sc = Scene::new("under-a-lamp");
    sc.room("room", 10.0, 10.0, 26.0, 18.0).door("south", Axis::X, 18.0, 20.0).bulb(22.0, 14.0);
    sc.sam(14.0 + 0.4 * nudge(s, 0), 14.0 + 0.5 * nudge(s, 1));
    let mut p = Play::scene(&sc, &seed(s));
    p.sam_takes_his_rifle();
    let (t, at) = p.lamp_at([22.0, 14.0]);
    (p, at, t)
}

/// THE RULE: "yeah, needs a real hit": a round must meet the bulb to break it. Dead on it goes out; a round laid 0.25 m under it passes and it stays lit.
/// A FAILURE MEANS: distant lamps are point and click again, or a lamp can no longer be shot out at all.
#[test]
fn a_lamp_needs_a_real_hit() {
    for s in SEEDS {
        let (mut p, at, t) = under_a_lamp(s);
        p.run(0.5);
        p.sam_round_at([at[0], at[1] - 0.25, at[2]]);
        p.run(0.1);
        assert!(!p.lamp_out(t), "seed {s}: a round 0.25 m under the bulb broke it");
        p.sam_round_at(at);
        p.run(0.1);
        assert!(p.lamp_out(t), "seed {s}: a round dead on did not break it");
    }
}

/// THE RULE: "we should also really penalize accuracy while moving": a gun's cone is much wider on moving feet than standing, still wider a moment after he stops, and closes again.
/// A FAILURE MEANS: walking costs a shot nothing, or the memory of the feet has gone.
#[test]
fn moving_feet_open_the_cone() {
    for s in SEEDS {
        let (mut p, at, _) = under_a_lamp(s);
        let away = [at[0] + 2.0, at[2]];
        p.point_at(away);
        p.run(2.0);
        let stood = p.ring_cone().expect("the ring is up with the gun out");
        p.key_down("KeyW");
        p.run(1.0);
        let walking = p.ring_cone().unwrap();
        p.key_up("KeyW");
        p.run(0.1);
        let stopped = p.ring_cone().unwrap();
        p.run(2.5);
        let settled = p.ring_cone().unwrap();
        assert!(walking > 1.8 * stood, "seed {s}: walking {walking:.4} against standing {stood:.4}");
        assert!(stopped > 1.2 * stood, "seed {s}: just stopped {stopped:.4} against standing {stood:.4}: the feet are not remembered");
        assert!(settled < 1.2 * stood, "seed {s}: the control: stood again {settled:.4}, standing {stood:.4}");
    }
}
