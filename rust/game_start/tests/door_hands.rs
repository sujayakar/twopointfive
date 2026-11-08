//! WHO STANDS WHERE WHEN A DOOR OPENS TOWARD THEM, OR OPENS UNDER THEM: the leaf on the team's own side, a boot beside glass, a charge on a door that comes open.
mod doors_ask;
use doors_ask::*;
use game::ids::*;
use game::orders::Who;
use game_start::play::{dist, Opts, Play};

fn two(seed: u32) -> Opts { Opts { seed, squad: 2, ..Default::default() } }
fn four(seed: u32) -> Opts { Opts { seed, ..Default::default() } }
fn hush(p: &mut Play) { let keep = p.defender_at([24.25, 12.25]); p.drop_all_but(&[keep]); p.pin_man(keep, [24.25, 12.25], 0.0, game::game::GuardState::Patrol, 0.0); }

/// does the leaf (hinge to tip) stand across the line p to q
fn leaf_across(h: [f64; 2], t: [f64; 2], p: [f64; 2], q: [f64; 2]) -> bool {
    let o = |a: [f64; 2], b: [f64; 2], c: [f64; 2]| (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
    o(h, t, p) * o(h, t, q) < 0.0 && o(p, q, h) * o(p, q, t) < 0.0
}
/// how far a place is from the leaf
fn from_leaf(h: [f64; 2], t: [f64; 2], p: [f64; 2]) -> f64 {
    let (ux, uz) = (t[0] - h[0], t[1] - h[1]);
    let k = (((p[0] - h[0]) * ux + (p[1] - h[1]) * uz) / (ux * ux + uz * uz)).clamp(0.0, 1.0);
    dist(p, [h[0] + ux * k, h[1] + uz * k])
}

/// THE RULE: "guy working the door from the other side is blocked by the door." A door held from the face it opens toward: nobody of the order holds the back of the open leaf: it is not across his line to the point he watches or to the doorway's middle, and he stands half a metre or more from it. The control is a man stood against the leaf: the same three tests read him.
/// A FAILURE MEANS: a man is dealt the jamb behind the leaf that has just swung onto his side.
#[test]
fn nobody_holds_the_back_of_a_leaf_that_opens_toward_them() {
    for s in SEEDS {
        let (sc, at) = yard(s);
        let mut p = Play::scene(&sc, &two(s));
        p.sam_unhurt();
        p.run(0.2);
        hush(&mut p);
        let d = p.door("west-yard");
        let out = [at[0], at[1] + 1.0];
        p.hung_toward(d, out);
        assert!(p.pull_face(d, out), "seed {s}: the staging");
        p.order("thresholdHold", at, Who::All);
        p.until(3.0, |p| !p.ops_on(d).is_empty());
        let op = p.ops_on(d)[0];
        p.until(40.0, |p| p.op_phase(op) == game::doortools::Phase::Thr).unwrap_or_else(|| panic!("seed {s}: never held: {:?}", p.op_beats(op)));
        p.run(4.0);
        let (h, t, c) = (p.hinge(d), p.leaf_tip(d), p.frame_at(d));
        assert!(p.leaf_angle(d).abs() > 1.0, "seed {s}: the leaf stands open on the team's side: {:.2}", p.leaf_angle(d));
        let men = p.op_front_men(op);
        assert!(!men.is_empty(), "seed {s}");
        let bad = |p: &Play, m: GuardId| { let a = p.at(m); let w = p.watch_of(m).unwrap_or(c); leaf_across(h, t, a, w) || leaf_across(h, t, a, c) || from_leaf(h, t, a) < 0.5 };
        for &m in &men { assert!(!bad(&p, m), "seed {s}: he stands {:.2} m from the leaf at {:?}", from_leaf(h, t, p.at(m)), p.at(m)); }
        let (mid, side) = ([(h[0] + t[0]) / 2.0, (h[1] + t[1]) / 2.0], 0.2);
        p.man_stands(men[0], [mid[0] - side, mid[1]]);
        assert!(bad(&p, men[0]), "seed {s}: the control: a man against the leaf is read");
    }
}

/// THE RULE: no boot from beside a latch jamb in front of glass: where rifles reach the kicker through a pane and clearly fewer reach the square spot in front of the leaf, the boot comes from the front. The rule moves a boot nowhere else. The control: the levels hold faces it does move.
/// A FAILURE MEANS: a kicker stands in front of a window to kick a door. The test prints each.
#[test]
fn no_boot_from_beside_a_door_in_front_of_glass() {
    let dir = game_start::files::root().join("levels");
    let mut names: Vec<String> = std::fs::read_dir(&dir).unwrap().map(|e| e.unwrap().file_name().to_str().unwrap().to_string()).filter(|n| n.ends_with(".json") && !n.ends_with(".lint.json")).collect();
    names.sort();
    let (mut in_glass, mut astray, mut moved) = (Vec::new(), Vec::new(), 0);
    for name in names {
        let mut p = Play::level(&name, false, &Opts { seed: 1, ..Default::default() });
        let (eyes, least) = p.glass_bar();
        for d in p.doors() {
            for side in [1.0, -1.0] {
                let (now, sq, beside) = p.boot_spots(d, side);
                let tag = format!("{name} {} {side}", p.door_name(d));
                let at_side = dist(now, sq) > 0.05;
                let was = if !p.floor_free(beside) { sq } else { beside };
                if at_side {
                    let n = p.glass_lines(d, side, now);
                    if n >= eyes { let f = p.glass_lines(d, side, sq); if f <= least * n && p.floor_free(sq) { in_glass.push(format!("{tag}: {n} lines beside the jamb, {f} on the front")); } }
                }
                if dist(was, now) > 1e-6 && !at_side {
                    moved += 1;
                    let (n, f) = (p.glass_lines(d, side, was), p.glass_lines(d, side, now));
                    if n < eyes || f > least * n { astray.push(format!("{tag}: moved with {n} lines where it stood and {f} on the front")); }
                }
            }
        }
    }
    assert!(in_glass.is_empty(), "{} boots in front of glass:\n{}", in_glass.len(), in_glass.join("\n"));
    assert!(astray.is_empty(), "{} boots moved for no glass:\n{}", astray.len(), astray.join("\n"));
    assert!(moved > 0, "the control: the levels asked hold no face the rule moves: nothing proved");
}

/// The compound's room clear with the four in file along the wall past `hand` jamb of the door (-1 hinge, 1 latch), one defender left far off; the charge order given, and half a second on the leaf put open if `open`. Eight seconds watched: who stood in front of the opening, the order's log, was a mark dealt across the doorway.
fn charge_under_them(s: u32, open: bool, hand: f64) -> (Vec<GuardId>, Vec<String>, bool) {
    let mut p = Play::mission("compound.json", "?levelFile=compound.json&roomclear&squad4", &four(s));
    p.sam_unhurt();
    p.run(0.2);
    let d = p.door("a_s");
    let keep = p.hostiles_by(p.frame_at(d)).pop().unwrap();
    p.drop_all_but(&[keep]);
    let w = p.door_w(d);
    let side = if p.on_face(d, w / 2.0, 3.0)[1] > p.frame_at(d)[1] { 1.0 } else { -1.0 };
    let men = p.men();
    let place = |k: usize| { let a = if hand < 0.0 { -1.3 - k as f64 * 0.9 } else { w + 1.3 + k as f64 * 0.9 }; a };
    let spots: Vec<[f64; 2]> = (0..12).map(|k| p.on_face(d, place(k), 0.75 * side)).filter(|q| p.floor_free(*q)).take(men.len() + 1).collect();
    assert_eq!(spots.len(), men.len() + 1, "seed {s}: the staging: no room along the wall past that jamb");
    for (i, &m) in men.iter().enumerate() { p.man_stands(m, spots[i]); }
    p.sam_put(spots[men.len()]);
    p.run(0.5);
    let from = p.on_face(d, w / 2.0, side);
    assert_eq!(p.order_on_face("charge", d, from), None, "seed {s}: the order");
    let op = p.ops_on(d)[0];
    let across = p.op_men(op).iter().any(|&m| p.post_of(m).is_some_and(|q| { let a = p.along(d, q); if hand < 0.0 { a > w - 0.1 } else { a < 0.1 } }));
    p.run(0.5);
    if open { p.leaf_put_open(d, 1.4); }
    let mut front = Vec::new();
    let go = p.op_men(op);
    for f in 0..480 {
        p.frame();
        if open && f > 6 { for &m in &go { if p.up(m) { let (a, o) = (p.along(d, p.at(m)), p.off(d, p.at(m), side)); if a > 0.1 && a < w - 0.1 && o > 0.0 && o < 3.0 && !front.contains(&m) { front.push(m); } } } }
    }
    (front, p.op_beats(op), across)
}

/// THE RULE (a replay of Mousehole: three men walked to stand-off marks past the far jamb, across the open doorway; one was shot in it). A charge order whose door comes open under it: nobody walks across the opening to a stand-off mark, and the order says whose hand it kept. The control is the door left shut: the charge is set and no hand is kept.
/// A FAILURE MEANS: the order's marks are not read again when its door comes open.
#[test]
fn a_charge_on_a_door_that_comes_open_walks_nobody_across_it() {
    for s in [1, 2] {
        let mut hand = -1.0;
        let (mut front, mut beats, mut across) = charge_under_them(s, true, hand);
        if !across { hand = 1.0; (front, beats, across) = charge_under_them(s, true, hand); }
        assert!(across, "seed {s}: the staging: no mark was dealt across the doorway on either hand: nothing asked");
        assert!(front.is_empty(), "seed {s}: {front:?} stood in front of the opening: {beats:?}");
        assert!(beats.iter().any(|b| b.starts_with("hand kept")), "seed {s}: {beats:?}");
        let (_, beats, _) = charge_under_them(s, false, hand);
        assert!(beats.iter().any(|b| b == "set") && !beats.iter().any(|b| b.starts_with("hand kept")), "seed {s}: the control: {beats:?}");
    }
}

/// The compound's floor with the fog on, quiet but for the farthest man. The door, the team.
fn answered(s: u32) -> (Play, DoorId, Vec<GuardId>) {
    let mut p = Play::mission("compound.json", "?levelFile=compound.json&roomclear&squad4", &four(s));
    p.sam_unhurt();
    p.fog_on();
    p.run(0.2);
    let d = p.door("c_n");
    let keep = p.hostiles_by(p.frame_at(d)).pop().unwrap();
    p.drop_all_but(&[keep]);
    p.run(0.5);
    let men = p.men();
    (p, d, men)
}

/// THE RULE: a door charge order is carried out or answered. Given from the mission's start and the leaf opens unseen while they walk: the order ends within two frames of the picture showing it open, a man says so, the men hold. Never silent, never a man left reading 'setting the charge'. The control is a door left shut: they are walking to it still.
/// A FAILURE MEANS: an order comes to nothing with nobody saying so: men stand at a door until the clock runs out.
#[test]
fn a_charge_order_is_carried_out_or_answered() {
    for s in SEEDS {
        let (mut p, d, men) = answered(s);
        let from = p.at(men[0]);
        assert_eq!(p.order_on_face("charge", d, from), None, "seed {s}");
        let op = p.ops_on(d)[0];
        p.run(0.5);
        assert!(p.op_ended(op).is_none(), "seed {s}: the control: with the door shut the order runs: {:?}", p.op_beats(op));
        p.leaf_put_open(d, 1.4);
        p.leaf_stays(d);
        let t = p.until(90.0, |p| p.op_ended(op).is_some()).unwrap_or_else(|| panic!("seed {s}: the order never ended: {:?}", p.op_beats(op)));
        assert_eq!(p.op_ended(op).as_deref(), Some("the door opened"), "seed {s}, {t:.1} s on: {:?}", p.op_beats(op));
        assert!(p.said_it("door's open."), "seed {s}: a man says so: {:?}", p.said.iter().map(|l| l.text.clone()).collect::<Vec<_>>());
        p.run(1.5);
        for &m in &men { assert_eq!(p.doing(m).as_deref(), Some("hold"), "seed {s}: the men hold on it"); }
        assert_eq!(p.seat_door_charges(0), 0, "seed {s}: nothing was set");
    }
}

/// THE RULE: refused as it is given, in words: the door open in the picture; nothing in the bag. A man says why and nobody moves. The control is the same order with a door shut and a charge carried: it is taken.
/// A FAILURE MEANS: a refused order walks the men anyway, or is refused with nobody saying so.
#[test]
fn a_charge_refused_as_it_is_given_is_said_and_nobody_moves() {
    for s in SEEDS {
        let (mut p, d, men) = answered(s);
        p.fog_off();
        p.leaf_put_open(d, 1.4);
        p.leaf_stays(d);
        p.run(3.0);
        let was: Vec<_> = men.iter().map(|&m| p.at(m)).collect();
        let from = p.at(men[0]);
        assert_eq!(p.order_on_face("charge", d, from).as_deref(), Some("it's open."), "seed {s}");
        p.run(2.0);
        assert!(p.said_it("it's open."), "seed {s}");
        for (i, &m) in men.iter().enumerate() { assert!(dist(p.at(m), was[i]) < 0.5, "seed {s}: he moved"); }
        drop(p);

        let (mut p, d, men) = answered(s);
        p.carries(0.0, 3.0);
        p.run(3.0);
        let was: Vec<_> = men.iter().map(|&m| p.at(m)).collect();
        let from = p.at(men[0]);
        assert_eq!(p.order_on_face("charge", d, from).as_deref(), Some("no charges left."), "seed {s}");
        p.run(2.0);
        assert!(p.said_it("no charges left."), "seed {s}");
        for (i, &m) in men.iter().enumerate() { assert!(dist(p.at(m), was[i]) < 0.5, "seed {s}: he moved"); }
        drop(p);

        let (mut p, d, men) = answered(s);
        let from = p.at(men[0]);
        assert_eq!(p.order_on_face("charge", d, from), None, "seed {s}: the control");
    }
}
