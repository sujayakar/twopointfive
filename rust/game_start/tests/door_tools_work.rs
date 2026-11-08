//! THE DOOR TOOLS: a charge, a stun, and what they cost. "bang and clear" and "breach with a charge" on the squad's own menu.
mod doors_ask;
use doors_ask::*;
use game::ids::*;
use game::orders::Who;
use game_start::play::{dist, Opts, Play};

fn four(seed: u32) -> Opts { Opts { seed, ..Default::default() } }
fn two(seed: u32) -> Opts { Opts { seed, squad: 2, ..Default::default() } }
fn stood(seed: u32, o: Opts) -> (Play, DoorId, [f64; 2]) {
    let (s, at) = yard(seed);
    let mut p = Play::scene(&s, &o);
    p.sam_unhurt();
    p.run(0.2);
    let d = p.door("west-yard");
    (p, d, at)
}
/// the room's men are off the floor but the one behind the far wall: nobody shoots, and the night is not over
fn hush(p: &mut Play) { let keep = p.defender_at([24.25, 12.25]); p.drop_all_but(&[keep]); p.pin_man(keep, [24.25, 12.25], 0.0, game::game::GuardState::Patrol, 0.0); }

/// THE RULE: a charge order ends with the charge on the door, one spent, the squad stood off past the backblast, and then NOTHING for as long as Sam
/// does not press: the leaf shut, no blast, nobody over the line. F then blows it: the keep broken, one spent, a man a stride behind the leaf down, the
/// other in the room blind and doubled over, a man behind a wall untouched, the squad not caught, and in they go.
/// A FAILURE MEANS: the charge has a timer of its own, or the stand-off is not out of the blast, or a blast reaches through a wall.
#[test]
fn a_charge_waits_for_the_trigger_then_takes_the_door_and_the_men_behind_it() {
    for s in SEEDS {
        let (mut p, d, at) = stood(s, two(s));
        let c0 = p.left().0;
        let (near, far) = (p.defender_at([11.25, 11.25]), p.defender_at([24.25, 12.25]));
        p.order("charge", at, Who::All);
        let armed = |p: &Play| p.last_op(d).is_some_and(|o| p.op_phase(o) == game::doortools::Phase::Armed);
        p.until(45.0, |p| armed(p)).unwrap_or_else(|| panic!("seed {s}: 45 s and the charge is not set and armed: {}", p.roll()));
        let op = p.last_op(d).unwrap();
        assert!(p.beat_t(op, "set").unwrap() < p.beat_t(op, "armed").unwrap(), "seed {s}: {:?}", p.op_beats(op));
        assert_eq!((p.seat_door_charges(0), p.left().0), (1, c0 - 1.0), "seed {s}: one on the door, one spent");
        let back = p.tools_k().backblast_m;
        for m in p.men() { assert!(dist(p.at(m), p.frame_at(d)) > back, "seed {s}: {:.2} m from the door with a backblast of {back} m", dist(p.at(m), p.frame_at(d))); }
        if let Some(t) = p.always(5.0, |p| armed(p) && p.door_shut(d) && p.crossed_t(op).is_empty() && p.up(near) && !p.blind(near)) { panic!("seed {s}: it did not wait on him: {t:.2} s in, {:?}", p.op_beats(op)); }
        assert!(p.row("charge", at).0.starts_with("blow the charge"), "seed {s}: {:?}", p.row("charge", at));
        let (h, w) = (p.frame_at(d), 1.3);
        p.man_stands(near, [h[0] - 1.0, h[1] - w]);
        p.key_down("KeyF");
        p.frame();
        p.key_up("KeyF");
        assert_eq!((p.tool_stats().blown, p.seat_door_charges(0)), (1.0, 0), "seed {s}: F blows it");
        assert!(!p.latched(d) && p.keep_broken(d), "seed {s}: the door is in");
        assert!(p.struck(near) && !p.up(near), "seed {s}: the man a stride behind the leaf is down: {:?}", p.state(near));
        assert!(p.up(far) && !p.blind(far) && !p.staggered(far), "seed {s}: a man behind a wall is untouched");
        assert_eq!(p.tool_stats().backblast, 0.0, "seed {s}: nobody of ours was in the backblast");
        for m in p.men() { assert!(!p.blind(m), "seed {s}: the squad is not blinded by its own charge"); }
        let blew = p.beat_t(op, "blow").unwrap_or(p.time());
        p.until(8.0, |p| !p.crossed_t(op).is_empty()).unwrap_or_else(|| panic!("seed {s}: nobody went in behind it: {:?}", p.op_beats(op)));
        assert!(p.crossed_t(op).iter().all(|&t| t > blew), "seed {s}: nobody over the line before the blast");
    }
}

/// THE RULE: a man inside a room when the charge goes, in reach and not beside the leaf, is blind and doubled over but up. A FAILURE MEANS: the near and far bands of a blast have parted from their knobs.
#[test]
fn a_man_further_into_the_room_is_stunned_not_knocked_out() {
    for s in SEEDS {
        let (mut p, d, at) = stood(s, two(s));
        let near = p.defender_at([11.25, 11.25]);
        p.order("charge", at, Who::All);
        p.until(45.0, |p| p.last_op(d).is_some_and(|o| p.op_phase(o) == game::doortools::Phase::Armed)).unwrap_or_else(|| panic!("seed {s}: not armed"));
        let (h, k) = (p.frame_at(d), p.tools_k().knock_m + 0.5);
        p.man_stands(near, [h[0], h[1] - k]);
        assert!(dist(p.at(near), h) < p.tools_k().stun_m, "seed {s}: the scene: he stands in the stun's reach");
        p.key_down("KeyF");
        p.frame();
        p.key_up("KeyF");
        assert!(p.up(near) && p.blind(near) && p.staggered(near), "seed {s}: {:?}, blind {}, doubled over {}", p.state(near), p.blind(near), p.staggered(near));
    }
}

/// THE RULE: the counts run out. With one of each: the bang spent, its row is grey in words and refused in them, nothing runs; the charge is spent on being
/// set and a second door is refused while the first still blows; a restart puts the counts back.
/// A FAILURE MEANS: a tool is free, a refused row runs, or a run's counts carry into the next.
#[test]
fn the_counts_run_out_and_a_restart_puts_them_back() {
    for s in SEEDS {
        let (mut p, d, at) = stood(s, two(s));
        let other = p.frame_at(p.door("east-yard"));
        hush(&mut p);
        p.carries(1.0, 0.0);
        assert_eq!(p.row("bangClear", at).1.as_deref(), Some("no bangs left."), "seed {s}: the row is grey in words: {:?}", p.row("bangClear", at));
        assert_eq!(p.try_order("bangClear", at, Who::All).as_deref(), Some("no bangs left."), "seed {s}");
        assert!(p.ops_on(d).is_empty(), "seed {s}: nothing runs");
        p.order("charge", at, Who::All);
        p.until(45.0, |p| p.seat_door_charges(0) == 1).unwrap_or_else(|| panic!("seed {s}: not set"));
        assert_eq!(p.left().0, 0.0, "seed {s}: spent on being set");
        assert_eq!(p.try_order("charge", other, Who::All).as_deref(), Some("no charges left."), "seed {s}: a second door");
        p.until(20.0, |p| p.last_op(d).is_some_and(|o| p.op_phase(o) == game::doortools::Phase::Armed)).unwrap_or_else(|| panic!("seed {s}: not armed"));
        assert_eq!(p.try_order("charge", at, Who::All), None, "seed {s}: the set one still blows from its row");
        p.frame();
        assert_eq!(p.tool_stats().blown, 1.0, "seed {s}");
        p.restarted();
        p.run(0.1);
        assert!(p.left().0 > 0.0 && p.left().1 > 0.0 && p.seat_door_charges(0) == 0 && !p.any_op(), "seed {s}: a restart is a fresh run: {:?}", p.left());
    }
}

/// THE RULE: a pair halted on a door the other pair is charging steps back out of the blast and reads so, its order still on that door. The control is
/// the same entry on the same door with nobody charging it: the pair stays at the jambs.
/// A FAILURE MEANS: a door order found shut leaves its men standing in another order's backblast.
#[test]
fn a_pair_on_a_door_the_other_pair_is_charging_steps_out_of_the_blast() {
    for s in SEEDS {
        let (mut p, d, at) = stood(s, four(s));
        hush(&mut p);
        p.wedged_from(d, [at[0], at[1] + 1.0]);
        let red = p.red();
        p.order("charge", at, Who::Blue);
        p.run(0.1);
        p.order("openClear", at, Who::Red);
        p.until(60.0, |p| p.seat_door_charges(0) == 1 && red.iter().all(|&m| p.there_of(m).starts_with("rear security") && p.arrived(m))).unwrap_or_else(|| panic!("seed {s}: red is not on rear security with a charge set: {:?} {:?}", red.iter().map(|&m| p.there_of(m)).collect::<Vec<_>>(), p.tool_stats()));
        let back = p.tools_k().backblast_m;
        for &m in &red {
            assert_eq!((p.doing(m).as_deref(), p.order_door(m)), (Some("stack"), Some(d)), "seed {s}: his order is still on that door");
            assert!(dist(p.at(m), p.frame_at(d)) >= back, "seed {s}: {:.2} m from the frame, backblast {back} m", dist(p.at(m), p.frame_at(d)));
        }
        p.until(30.0, |p| p.last_op(d).is_some_and(|o| p.op_phase(o) == game::doortools::Phase::Armed)).unwrap_or_else(|| panic!("seed {s}: not armed"));
        p.key_down("KeyF");
        p.frame();
        p.key_up("KeyF");
        assert_eq!((p.tool_stats().blown, p.tool_stats().backblast), (1.0, 0.0), "seed {s}: blown and nobody of ours in the backblast");
        drop(p);

        let (mut p, d, at) = stood(s, four(s));
        hush(&mut p);
        p.wedged_from(d, [at[0], at[1] + 1.0]);
        let red = p.red();
        p.order("openClear", at, Who::Red);
        p.until(3.0, |p| !p.ops_on(d).is_empty());
        p.until(40.0, |p| p.ops_on(d).is_empty()).unwrap_or_else(|| panic!("seed {s}: the control never halted"));
        p.run(1.5);
        assert!(red.iter().all(|&m| !p.there_of(m).starts_with("rear security")), "seed {s}: the control: nobody is charging it, and they stepped back: {:?}", red.iter().map(|&m| p.there_of(m)).collect::<Vec<_>>());
    }
}

/// the room's bounds, as the scene has them
fn in_west_room(at: [f64; 2]) -> bool { at[0] > 10.0 && at[0] < 18.0 && at[1] > 10.0 && at[1] < 16.0 }

/// THE RULE: bang and clear goes in its order: stacked, the door opened, the can out, the bang inside the room, 'go', and only then the first man over the
/// frame line, no sooner than `after_bang` behind it. It blinds the room's man and not the pair, and one bang is spent.
/// A FAILURE MEANS: a man is in the doorway before the stun, or the can burst somewhere it does the room no good.
#[test]
fn a_stun_is_in_the_room_before_the_first_man_is_over_the_threshold() {
    for s in SEEDS {
        let (mut p, d, at) = stood(s, two(s));
        let b0 = p.left().1;
        let inner = p.defender_at([11.25, 11.25]);
        p.order("bangClear", at, Who::All);
        let mut blind_at_bang = None;
        p.until(3.0, |p| !p.ops_on(d).is_empty());
        let op = p.ops_on(d)[0];
        p.until(40.0, |p| { if blind_at_bang.is_none() && p.op_bang_at(op).is_some() { blind_at_bang = Some(p.blind(inner) || !p.up(inner)); } !p.crossed_t(op).is_empty() }).unwrap_or_else(|| panic!("seed {s}: nobody went in: {:?}", p.op_beats(op)));
        let t = |n: &str| p.beat_t(op, n).unwrap_or_else(|| panic!("seed {s}: no beat '{n}': {:?}", p.op_beats(op)));
        let first = p.crossed_t(op).into_iter().fold(f64::MAX, f64::min);
        assert!(t("stacked") <= t("shove") && t("shove") < t("toss") && t("toss") < t("bang") && t("bang") < t("go") && t("go") < first, "seed {s}: {:?}, first over {first:.2}", p.op_beats(op));
        assert!(first - t("bang") >= p.tools_k().after_bang, "seed {s}: the first man over {:.2} s after it", first - t("bang"));
        let burst = p.op_bang_at(op).unwrap();
        assert!(in_west_room(burst), "seed {s}: the can went off at {burst:?}, outside the room");
        assert_eq!(blind_at_bang, Some(true), "seed {s}: the bang blinds the room's man");
        for m in p.men() { assert!(!p.blind(m), "seed {s}: the pair is not blinded by its own can"); }
        assert_eq!(p.left().1, b0 - 1.0, "seed {s}: one spent");
    }
}

/// THE RULE: the men ordered elsewhere with the can in the air: the order ends and the can goes off on a short fuse. The control is the order left alone: the can's own short fuse from the landing.
/// A FAILURE MEANS: a live can is left lying on its long fuse, to go off when nobody is looking.
#[test]
fn a_can_in_the_air_when_the_order_is_taken_goes_off_on_a_short_fuse() {
    for s in SEEDS {
        let (mut p, d, at) = stood(s, two(s));
        hush(&mut p);
        p.order("bangClear", at, Who::All);
        p.until(3.0, |p| !p.ops_on(d).is_empty());
        let op = p.ops_on(d)[0];
        p.until(40.0, |p| p.can_out(op).is_some()).unwrap_or_else(|| panic!("seed {s}: no can out: {:?}", p.op_beats(op)));
        let (can, long, _) = p.can_out(op).unwrap();
        assert!(long > 10.0, "seed {s}: the staging: a can in the air is on its long fuse: {long}");
        p.order("hold", p.sam_at(), Who::All);
        p.run(2.0 / 60.0);
        assert!(p.ops_on(d).is_empty(), "seed {s}: the order ended");
        let (short, fired) = p.can_now(can);
        assert!(fired || short <= p.tools_k().fuse_after_land + 1e-6, "seed {s}: fuse {long:.2} and now {short:.2}");
        p.run(p.tools_k().fuse_after_land + 1.2);
        assert!(p.can_now(can).1, "seed {s}: it went off");
    }
}

/// THE RULE: a locked door gets the boot before the can. A FAILURE MEANS: a can is thrown at a shut leaf and goes off on the squad's side.
#[test]
fn a_door_known_shut_gets_the_boot_first_and_then_the_can() {
    for s in SEEDS {
        let (mut p, d, at) = stood(s, two(s));
        hush(&mut p);
        p.wedged_from(d, [at[0], at[1] + 1.0]);
        p.order("bangClear", at, Who::All);
        p.until(3.0, |p| !p.ops_on(d).is_empty());
        p.until(40.0, |p| p.ops_on(d).is_empty()).unwrap_or_else(|| panic!("seed {s}: it never halted at the handle"));
        p.order("bangClear", at, Who::All);
        p.until(3.0, |p| !p.ops_on(d).is_empty());
        let op = p.ops_on(d)[0];
        p.until(40.0, |p| !p.crossed_t(op).is_empty()).unwrap_or_else(|| panic!("seed {s}: nobody went in: {:?}", p.op_beats(op)));
        let t = |n: &str| p.beat_t(op, n).unwrap_or_else(|| panic!("seed {s}: no beat '{n}': {:?}", p.op_beats(op)));
        assert!(t("kick") < t("toss") && t("toss") < t("bang"), "seed {s}: {:?}", p.op_beats(op));
        assert!(p.keep_broken(d), "seed {s}: the keep is broken");
        assert!(in_west_room(p.op_bang_at(op).unwrap()), "seed {s}: {:?}", p.op_bang_at(op));
    }
}

/// THE RULE: "the squad does waste a lot of bangs". A bang and clear given from the face a door opens toward waits for the leaf: no can goes off on the
/// squad's side of it, and one does go off. The control is the same door hung the other way, pushed.
/// A FAILURE MEANS: a can met the leaf swinging toward the stack.
#[test]
fn a_can_is_not_thrown_at_a_door_the_squad_pulls_open() {
    for s in SEEDS {
        for pulled in [true, false] {
            let (mut p, d, at) = stood(s, two(s));
            hush(&mut p);
            let out = [at[0], at[1] + 1.0];
            if pulled { p.hung_toward(d, out); } else { p.hung_toward(d, [at[0], at[1] - 1.0]); }
            assert_eq!(p.pull_face(d, out), pulled, "seed {s}: the staging");
            p.order("bangClear", at, Who::All);
            p.until(3.0, |p| !p.ops_on(d).is_empty());
            let op = p.ops_on(d)[0];
            p.until(40.0, |p| p.op_bang_at(op).is_some()).unwrap_or_else(|| panic!("seed {s}, pulled {pulled}: no can went off: {:?}", p.op_beats(op)));
            assert!(in_west_room(p.op_bang_at(op).unwrap()) && !p.op_beats(op).iter().any(|b| b.starts_with("bang · OUTSIDE")), "seed {s}, pulled {pulled}: {:?} at {:?}", p.op_beats(op), p.op_bang_at(op));
        }
    }
}

/// the squad holds the door from outside, the leaf as the arm has it, and is told bang, then go in: the order
fn told_at_a_held_door(s: u32, shut: bool) -> (Play, DoorId, DoorOpId, usize) {
    let (mut p, d, at) = stood(s, four(s));
    hush(&mut p);
    p.order("thresholdHold", at, Who::All);
    p.until(3.0, |p| !p.ops_on(d).is_empty());
    let op = p.ops_on(d)[0];
    p.until(40.0, |p| p.op_phase(op) == game::doortools::Phase::Thr).unwrap_or_else(|| panic!("seed {s}: they never held it: {:?}", p.op_beats(op)));
    p.run(2.5);
    let n = p.op_beats(op).len();
    if shut { p.leaf_put_shut(d); p.run(10.0 / 60.0); assert!(p.latched(d), "seed {s}: the leaf would not stay shut"); }
    p.order("bangClear", at, Who::All);
    (p, d, op, n)
}

/// THE RULE (a replay of Mousehole): a bang told at a held door that has come shut waits for the leaf: it is opened first, no can bursts on the team's side, and they
/// go in. The control is the door left standing open: no wait, and the can bursts inside.
/// A FAILURE MEANS: the can is thrown at the shut leaf.
#[test]
fn a_bang_told_at_a_held_door_that_has_come_shut_waits_for_the_leaf() {
    for s in SEEDS {
        let (mut p, d, op, n) = told_at_a_held_door(s, true);
        let side = p.face(d, [p.frame_at(d)[0], p.frame_at(d)[1] + 1.0]);
        let went = p.until(14.0, |p| !p.across(d, side).is_empty());
        let beats = p.op_beats(op)[n..].to_vec();
        assert!(beats.iter().any(|b| b.starts_with("can waits: the door has come shut")), "seed {s}: {beats:?}");
        assert!(!beats.iter().any(|b| b.starts_with("bang · OUTSIDE")), "seed {s}: {beats:?}");
        assert!(went.is_some(), "seed {s}: nobody over the line in 14 s: {beats:?}");
        drop(p);

        let (mut p, d, op, n) = told_at_a_held_door(s, false);
        p.until(14.0, |p| p.op_beats(op)[n..].iter().any(|b| b.starts_with("bang · "))).unwrap_or_else(|| panic!("seed {s}: the control: no burst: {:?}", p.op_beats(op)));
        let beats = p.op_beats(op)[n..].to_vec();
        assert!(!beats.iter().any(|b| b.starts_with("can waits")) && beats.iter().any(|b| b.starts_with("bang · inside")), "seed {s}: the control: {beats:?}");
        assert!(!p.door_shut(d), "seed {s}: the control");
    }
}

/// THE RULE: "after being ordered to frag after externally clearing, blue one crosses across a dangerous threshold". A frag order on a door that stands open walks nobody across
/// the front of the opening; it bursts inside and then they go in. With one man left on the hinge hand he keeps his own hand. The control is a man set in the opening: the same line reads him.
/// A FAILURE MEANS: a man is dealt the far jamb across an open doorway.
#[test]
fn a_frag_on_an_open_door_walks_nobody_across_the_opening() {
    for (seed, lone) in [(1, false), (2, false), (3, true)] {
        let (mut p, d, at) = stood(seed, two(seed));
        hush(&mut p);
        let out = [at[0], at[1] + 1.0];
        let side = p.face(d, out);
        p.order("thresholdHold", at, Who::All);
        p.until(3.0, |p| !p.ops_on(d).is_empty());
        let hold = p.ops_on(d)[0];
        p.until(45.0, |p| p.op_phase(hold) == game::doortools::Phase::Thr).unwrap_or_else(|| panic!("seed {seed}: never held: {:?}", p.op_beats(hold)));
        p.run(2.5);
        if lone {
            let keep = p.men().into_iter().find(|&m| p.along(d, p.at(m)) <= -0.3).unwrap_or_else(|| panic!("seed {seed}: nobody on the hinge hand: {}", p.roll()));
            for m in p.men() { if m != keep { p.falls(m, game::game::DownBy::Player); } }
            p.run(0.5);
        }
        let w = p.door_w(d);
        let front = |p: &Play, m| { let (a, o) = (p.along(d, p.at(m)), p.off(d, p.at(m), side)); a > 0.05 && a < w - 0.05 && o > 0.1 && o < 3.0 };
        assert!(!p.door_shut(d), "seed {seed}: the leaf stands open as the order is given");
        p.order("fragClear", at, Who::All);
        p.frame();
        let op = p.ops_on(d).into_iter().find(|&o| !p.op_beats(o).is_empty() && p.op_is_can(o)).unwrap_or_else(|| panic!("seed {seed}: no frag order runs"));
        let mut crossed = 0;
        p.until(30.0, |p| { for m in p.men() { if front(p, m) { crossed += 1; } } p.op_beats(op).iter().any(|b| b.starts_with("frag · ")) }).unwrap_or_else(|| panic!("seed {seed}: no burst: {:?}", p.op_beats(op)));
        assert_eq!(crossed, 0, "seed {seed}: man-frames in front of the opening before the burst");
        assert!(p.op_beats(op).iter().any(|b| b.starts_with("frag · inside")), "seed {seed}: {:?}", p.op_beats(op));
        p.until(25.0, |p| !p.across(d, side).is_empty()).unwrap_or_else(|| panic!("seed {seed}: nobody went in: {:?}", p.op_beats(op)));
        assert!(!p.men().is_empty(), "seed {seed}: they lived through their own frag");
        let m = p.men()[0];
        p.man_stands(m, [p.frame_at(d)[0], p.frame_at(d)[1] + 1.0]);
        assert!(front(&p, m), "seed {seed}: the control: a man a metre out of the opening is in front of it");
    }
}
