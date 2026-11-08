//! THE SILENT PARTNER AND THE TWO PAIRS: a man taken on the mark, one pair holding while the other clears, one word for two doors.
mod doors_ask;
use doors_ask::*;
use game::game::GuardState;
use game::ids::*;
use game::orders::Who;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};

const EAST: f64 = std::f64::consts::FRAC_PI_2;

/// The office with Sam and one partner on the aisle, silent; one man of the other side held on the aisle with his back to them. The partner, the man, and `more` others.
fn aisle(seed: u32, more: usize) -> (Play, GuardId, GuardId, Vec<GuardId>) {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad=1", &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.run(0.3);
    let m = p.mates()[0];
    let foes = p.hostiles();   // (the rest stay up behind their rooms' doors: a man whose fellows all fell goes looking for them, and that is another rule)
    p.quiet_pair();
    let at = aisle_at(seed);
    p.sam_put([11.0, 18.5]);
    p.man_stands(m, [13.0, 17.6]);
    for _ in 0..10 { p.pin_man(foes[0], at, EAST, GuardState::Patrol, 0.0); p.frame(); }
    (p, m, foes[0], foes[1..1 + more].to_vec())
}
fn aisle_at(seed: u32) -> [f64; 2] { [22.0 + nudge(seed, 60), 17.6] }
/// a frame with him held on his mark, his back to them
fn held(p: &mut Play, foe: GuardId, seed: u32) { if p.up(foe) { p.pin_man(foe, aisle_at(seed), EAST, GuardState::Patrol, p.awareness(foe)); } p.frame(); }
fn f(p: &mut Play, foe: GuardId, seed: u32) { p.key_down("KeyF"); held(p, foe, seed); p.key_up("KeyF"); held(p, foe, seed); }
fn until_set(p: &mut Play, m: GuardId, foe: GuardId, seed: u32) {
    for _ in 0..30 * 60 { if p.job(m).is_some_and(|j| j.0) { return; } held(p, foe, seed); }
    panic!("seed {seed}: 30 s and he is not set at the man's back: {}", p.why(m));
}

/// THE RULE: take him on my mark: he works round to the man's back on a knee, says set and WAITS, the man none the wiser, for as long as Sam gives no mark (the control); F then sends him, and the man is out cold, struck and not shot, no round fired, nobody roused, and he holds.
/// A FAILURE MEANS: he does not wait for the mark, or the mark is loud.
#[test]
fn take_that_guard_on_my_mark() {
    for s in SEEDS {
        let (mut p, m, foe, _) = aisle(s, 0);
        assert_eq!(p.order_on_foe("pairMark", foe), None, "seed {s}: the order on the man");
        until_set(&mut p, m, foe, s);
        assert!(p.lines(m).contains(&"set."), "seed {s}: {:?}", p.lines(m));
        let r0 = p.squad_rounds();
        for _ in 0..300 { held(&mut p, foe, s); }
        assert!(p.up(foe) && p.job(m).is_some_and(|j| j.0 && j.1 < 0.0) && dist(p.at(m), p.at(foe)) > 0.8, "seed {s}: the control: with no mark he stays set: {}", p.why(m));
        assert!(p.awareness(foe) < 0.3, "seed {s}: the man is none the wiser: {:.2}", p.awareness(foe));
        f(&mut p, foe, s);
        for _ in 0..360 { if !p.up(foe) { break; } held(&mut p, foe, s); }
        assert!(!p.up(foe) && p.struck(foe), "seed {s}: down, struck: {:?} {:?}", p.state(foe), p.why(m));
        assert_eq!(p.squad_rounds(), r0, "seed {s}: no round fired");
        assert!(p.silent(), "seed {s}: the squad is still silent");
        p.run(1.0);
        assert!(p.job(m).is_none(), "seed {s}: the job is over");
    }
}

/// THE RULE: the mark with another man looking at his man: no hands: he says who has eyes on him and stays set; F again sends him anyway. The control is the test above, nobody looking.
/// A FAILURE MEANS: the partner walks up behind a man another man is watching.
#[test]
fn with_another_looking_he_says_who_and_waits() {
    // (one seed: a watcher has to stand twenty seconds for the way he mostly faces to BE this way)
    for s in [1] {
        let (mut p, m, foe, others) = aisle(s, 1);
        let w = others[0];
        let at = aisle_at(s);
        let from = [at[0] + 5.0, at[1]];
        assert_eq!(p.order_on_foe("pairMark", foe), None, "seed {s}");
        until_set(&mut p, m, foe, s);
        let hold_w = |p: &mut Play| p.pin_man(w, from, -EAST, GuardState::Patrol, 0.0);
        for _ in 0..20 * 60 { hold_w(&mut p); held(&mut p, foe, s); }
        hold_w(&mut p);
        f(&mut p, foe, s);
        for _ in 0..180 { hold_w(&mut p); held(&mut p, foe, s); }
        let name = p.callsign(w);
        assert!(p.lines(m).iter().any(|l| l.contains(&format!("{name} has eyes on him"))), "seed {s}: he says who: {:?}", p.lines(m));
        assert!(p.up(foe) && p.job(m).is_some_and(|j| j.2 == Some(w) && j.1 < 0.0), "seed {s}: no hands: {:?}", p.job(m));
        f(&mut p, foe, s);
        for _ in 0..360 { if !p.up(foe) { break; } hold_w(&mut p); held(&mut p, foe, s); }
        assert!(!p.up(foe), "seed {s}: F again sends him anyway");
    }
}

/// THE RULE: take him now is the same job with no mark: he goes round and takes him, no round, nobody roused. The control: the order on the mark does not.
/// A FAILURE MEANS: 'now' waits for a mark, or goes in loud.
#[test]
fn take_him_now_needs_no_mark() {
    for s in SEEDS {
        let (mut p, m, foe, _) = aisle(s, 0);
        let r0 = p.squad_rounds();
        assert_eq!(p.order_on_foe("pairTake", foe), None, "seed {s}");
        for _ in 0..45 * 60 { if !p.up(foe) { break; } held(&mut p, foe, s); }
        assert!(!p.up(foe) && p.struck(foe), "seed {s}: he took him: {:?}", p.why(m));
        assert_eq!(p.squad_rounds(), r0, "seed {s}: no round");
    }
}

/// THE RULE: a man with the partner's hands on him sees nobody until they are off: turned to face the partner the frame the hands go on, he never stirs, and goes down. The control is the same man turned to face him an arm's length off with no hands on: he notices.
/// A FAILURE MEANS: a held man raises the alarm in the partner's arm.
#[test]
fn a_man_in_the_partners_hands_sees_nobody() {
    for s in SEEDS {
        let (mut p, m, foe, _) = aisle(s, 0);
        assert_eq!(p.order_on_foe("pairMark", foe), None, "seed {s}");
        until_set(&mut p, m, foe, s);
        f(&mut p, foe, s);
        for _ in 0..300 { if p.job(m).is_some_and(|j| j.1 >= 0.0) { break; } held(&mut p, foe, s); }
        let (mut frames, mut held_frames, mut peak) = (0, 0, 0.0f64);
        for _ in 0..300 {
            if !p.up(foe) { break; }
            let a = p.at(m);
            p.turned_to(foe, a);
            frames += 1;
            if p.gripped(foe) { held_frames += 1; }
            peak = peak.max(p.awareness(foe));
            p.frame();
        }
        assert!(!p.up(foe) && p.struck(foe), "seed {s}: he goes down: {:?}", p.state(foe));
        assert!(frames > 20 && held_frames == frames - 1 || held_frames == frames, "seed {s}: {held_frames} of {frames} frames gripped");
        assert!(peak < 0.05, "seed {s}: awareness reached {peak:.2}");
        drop(p);

        let (mut p, m, foe, _) = aisle(s, 0);
        let at = aisle_at(s);
        p.man_stands(m, [at[0] + 0.7, at[1]]);
        let mut peak = 0.0f64;
        for _ in 0..120 {
            let a = p.at(m);
            p.turned_to(foe, a);
            p.man_stands(foe, at);
            p.frame();
            peak = peak.max(p.awareness(foe));
        }
        assert!(peak > 0.05 || p.state(foe) != GuardState::Patrol, "seed {s}: the control: facing him with no hands on he noticed nothing: {peak:.2}");
    }
}

/// THE RULE: after a take the partner comes back to where he was: sent from Sam's shoulder he falls in again and says so; sent from a hold he holds where the man fell.
/// A FAILURE MEANS: the partner is left at the body for Sam to order back, or wanders off a hold.
#[test]
fn after_a_take_the_partner_comes_back_to_the_shoulder() {
    for s in SEEDS {
        for from in ["fallIn", "hold"] {
            let (mut p, m, foe, _) = aisle(s, 0);
            let at = if from == "hold" { p.at(m) } else { p.sam_at() };
            p.order(from, at, Who::All);
            for _ in 0..30 { held(&mut p, foe, s); }
            assert_eq!(p.doing(m).as_deref(), Some(from), "seed {s}: the staging");
            assert_eq!(p.order_on_foe("pairMark", foe), None, "seed {s}");
            until_set(&mut p, m, foe, s);
            f(&mut p, foe, s);
            for _ in 0..15 * 60 { if !p.up(foe) { break; } held(&mut p, foe, s); }
            assert!(!p.up(foe), "seed {s}: he never took him");
            p.run(0.6);
            assert_eq!(p.doing(m).as_deref(), Some(from), "seed {s}: back on '{from}'");
            let on_you = p.lines(m).iter().any(|l| l.contains("he's down. on you."));
            assert_eq!(on_you, from == "fallIn", "seed {s}, from {from}: {:?}", p.lines(m));
        }
    }
}

// ---------------------------------------------------------------- two pairs

fn fours(seed: u32) -> Opts { Opts { seed, ..Default::default() } }
fn hush(p: &mut Play, keep_at: [f64; 2]) { let keep = p.defender_at(keep_at); p.drop_all_but(&[keep]); p.pin_man(keep, keep_at, 0.0, GuardState::Patrol, 0.0); }

/// THE RULE: one pair holds while the other clears: red stacked on one door keeps its stack, on its marks, from the order to the end, while blue, alone, opens and clears through the other. The control is what the order takes: two men, blue.
/// A FAILURE MEANS: an order to one pair moves the other, or the other's stack lets go when the fight starts.
#[test]
fn one_pair_keeps_its_stack_while_the_other_clears() {
    for s in SEEDS {
        let (sc, at) = yard(s);
        let mut p = Play::scene(&sc, &fours(s));
        p.sam_unhurt();
        p.run(0.2);
        hush(&mut p, [24.25, 12.25]);
        let (red, blue) = (p.red(), p.blue());
        let (e, w) = (p.door("east-yard"), p.door("west-yard"));
        p.order("stack", p.frame_at(e), Who::Red);
        p.until(25.0, |p| red.iter().all(|&m| p.arrived(m))).unwrap_or_else(|| panic!("seed {s}: red never set"));
        p.run(1.0);
        let posts: Vec<_> = red.iter().map(|&m| p.post_of(m).unwrap()).collect();
        p.order("openClear", at, Who::Blue);
        p.until(3.0, |p| !p.ops_on(w).is_empty());
        let op = p.ops_on(w)[0];
        let sent = p.op_men(op);
        assert!(sent.len() == 2 && sent.iter().all(|m| blue.contains(m)), "seed {s}: the order took {sent:?}");
        let mut drift = 0.0f64;
        p.until(45.0, |p| { for (i, &m) in red.iter().enumerate() { assert_eq!((p.doing(m).as_deref(), p.order_door(m)), (Some("stack"), Some(e)), "seed {s}: red let go"); drift = drift.max(dist(p.at(m), posts[i])); } !p.any_op() }).unwrap_or_else(|| panic!("seed {s}: blue's order never ended: {:?}", p.op_beats(op)));
        assert_eq!(p.op_ended(op).as_deref(), Some("in"), "seed {s}: {:?}", p.op_beats(op));
        assert!(drift < 0.8, "seed {s}: red moved {drift:.2} m off its marks");
    }
}

/// THE RULE: told to hold as a pair in a corridor, the two split the ways out: each man a watch point, gun and light up, the arcs more than 90 degrees apart, and they stay while the other pair works. The control is the same order to one man alone: he has one arc and nobody to split with.
/// A FAILURE MEANS: both men look the same way down a corridor with two ends.
#[test]
fn a_pair_told_to_hold_splits_the_ways_out() {
    for s in SEEDS {
        let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &fours(s));
        p.sam_unhurt();
        p.run(0.2);
        p.drop_all_but(&[]);
        p.sam_put([19.5, 11.2]);
        let red = p.red();
        p.order("move", [28.5, 11.0], Who::Red);
        p.until(25.0, |p| red.iter().all(|&m| p.arrived(m))).unwrap_or_else(|| panic!("seed {s}: never there: {}", p.roll()));
        p.order("hold", p.sam_at(), Who::Red);
        p.run(1.0);
        let here: Vec<_> = red.iter().map(|&m| p.at(m)).collect();
        let yaws: Vec<f64> = red.iter().map(|&m| { assert!(p.light_on(m), "seed {s}: his light is up"); yaw_to(p.at(m), p.watch_of(m).expect("a watch point")) }).collect();
        assert!(off_by(yaws[0], yaws[1]) > std::f64::consts::FRAC_PI_2, "seed {s}: the arcs are {:.2} rad apart", off_by(yaws[0], yaws[1]));
        p.run(4.0);
        for (i, &m) in red.iter().enumerate() { assert!(dist(p.at(m), here[i]) < 0.8 && p.doing(m).as_deref() == Some("hold"), "seed {s}: he stays"); }
    }
}

/// A room with two doors, one on the yard to its south and one on the yard to its west; the four in the yard, a defender in the room and one in the next. Which door is which, and where.
fn two_doors(seed: u32) -> (Play, DoorId, DoorId) {
    let mut sc = Scene::new("two-doors");
    sc.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    sc.door("south", Axis::X, 16.0, 13.4 + nudge(seed, 61)).door("side", Axis::Z, 10.0, 12.4).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    sc.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    sc.team([7.0, 20.0], NORTH, &[[6.0, 21.0], [8.0, 21.0], [6.0, 22.0], [8.0, 22.0]]);
    sc.defender("west", [15.25, 11.25], "south").defender("east", [24.25, 12.25], "east-yard");
    let mut p = Play::scene(&sc, &fours(seed));
    p.sam_unhurt();
    p.run(0.2);
    hush(&mut p, [24.25, 12.25]);
    let (a, b) = (p.door("south"), p.door("side"));
    (p, a, b)
}

/// THE RULE: "simultaneous breach ... didn't really work". A pair stacked on each door of a room, both on their marks, one word: both go on one frame, each through its own door, to places apart, and no can is thrown. The control is the word to one pair: the other stays.
/// A FAILURE MEANS: the two entries are no longer on one word.
#[test]
fn two_doors_go_on_one_word() {
    for s in SEEDS {
        let (mut p, a, b) = two_doors(s);
        let (red, blue) = (p.red(), p.blue());
        p.order("stack", p.frame_at(a), Who::Red);
        p.order("stack", p.frame_at(b), Who::Blue);
        p.until(40.0, |p| red.iter().chain(blue.iter()).all(|&m| p.arrived(m))).unwrap_or_else(|| panic!("seed {s}: never both stacked: {}", p.roll()));
        p.run(1.0);
        let bangs = p.left().1;
        p.key_down("KeyF");
        p.frame();
        p.key_up("KeyF");
        p.run(0.1);
        let (oa, ob) = (*p.ops_on(a).first().expect("the south door's entry runs"), *p.ops_on(b).first().expect("the side door's entry runs"));
        p.until(45.0, |p| !p.any_op()).unwrap_or_else(|| panic!("seed {s}: the entries never ended: {:?} {:?}", p.op_beats(oa), p.op_beats(ob)));
        let (ga, gb) = (p.beat_t(oa, "go").expect("go"), p.beat_t(ob, "go").expect("go"));
        assert!((ga - gb).abs() < 1.5 / 60.0, "seed {s}: the two went {:.3} s apart", (ga - gb).abs());
        assert_eq!((p.op_ended(oa).as_deref(), p.op_ended(ob).as_deref()), (Some("in"), Some("in")), "seed {s}");
        assert_eq!(p.left().1, bangs, "seed {s}: a plain entry throws no can");
        let (pa, pb) = (p.op_pods(oa), p.op_pods(ob));
        let apart = p.tools_k().pod_apart - 0.01;
        for x in &pa { for y in &pb { assert!(dist(*x, *y) >= apart, "seed {s}: two men dealt places {:.2} m apart", dist(*x, *y)); } }
        drop(p);

        let (mut p, a, b) = two_doors(s);
        let (red, blue) = (p.red(), p.blue());
        p.order("stack", p.frame_at(a), Who::Red);
        p.order("stack", p.frame_at(b), Who::Blue);
        p.until(40.0, |p| red.iter().chain(blue.iter()).all(|&m| p.arrived(m))).unwrap_or_else(|| panic!("seed {s}: the control: never both stacked"));
        p.order("go", p.sam_at(), Who::Red);
        p.run(0.1);
        assert!(!p.ops_on(a).is_empty() && p.ops_on(b).is_empty(), "seed {s}: the control: the word to red sends red alone");
    }
}

/// THE RULE: tools given on the word wait in ready; on the word the bang goes first and the quiet door opens on the bang door's go, so both pairs enter after the bang. The control is three seconds with no word: both still wait.
/// A FAILURE MEANS: a pair on the word goes by itself, or the quiet door goes in ahead of the bang.
#[test]
fn tools_on_the_word_wait_and_the_bang_goes_first() {
    for s in SEEDS {
        let (mut p, a, b) = two_doors(s);
        p.order_on_word("bangClear", p.frame_at(a), Who::Red);
        p.order_on_word("openClear", p.frame_at(b), Who::Blue);
        p.run(0.1);
        let (oa, ob) = (p.ops_on(a)[0], p.ops_on(b)[0]);
        let ready = |p: &Play| p.op_phase(oa) == game::doortools::Phase::Ready && p.op_phase(ob) == game::doortools::Phase::Ready;
        p.until(45.0, |p| ready(p)).unwrap_or_else(|| panic!("seed {s}: never both ready: {:?} {:?}", p.op_beats(oa), p.op_beats(ob)));
        if let Some(t) = p.always(3.0, |p| ready(p)) { panic!("seed {s}: he went with no word, {t:.2} s on"); }
        p.key_down("KeyF");
        p.frame();
        p.key_up("KeyF");
        p.until(45.0, |p| !p.any_op()).unwrap_or_else(|| panic!("seed {s}: never ended"));
        let (bang, rgo, bshove, bgo) = (p.beat_t(oa, "bang").expect("bang"), p.beat_t(oa, "go").expect("go"), p.beat_t(ob, "shove").expect("shove"), p.beat_t(ob, "go").expect("go"));
        assert!(bang < bgo && (bshove - rgo).abs() < 2.5 / 60.0 && bgo >= rgo, "seed {s}: bang {bang:.2} red go {rgo:.2} blue shove {bshove:.2} blue go {bgo:.2}");
        assert_eq!((p.op_ended(oa).as_deref(), p.op_ended(ob).as_deref()), (Some("in"), Some("in")), "seed {s}");
    }
}

/// THE RULE: a posted man lowers his muzzle while a man of the other pair crosses his line, and it is back up when he is out of it. The control is the time before: blue elsewhere, the muzzle is up.
/// A FAILURE MEANS: a held gun tracks through a friend.
#[test]
fn nobody_stands_in_the_other_pairs_line_with_a_muzzle_on_him() {
    for s in SEEDS {
        let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &fours(s));
        p.sam_unhurt();
        p.run(0.2);
        p.drop_all_but(&[]);
        p.sam_put([19.5, 11.2]);
        let (red, blue) = (p.red(), p.blue());
        p.order("move", [28.5, 11.0], Who::Red);
        p.until(25.0, |p| red.iter().all(|&m| p.arrived(m))).unwrap_or_else(|| panic!("seed {s}: red never there"));
        p.order("hold", p.sam_at(), Who::Red);
        p.run(1.0);
        for &m in &red { assert!(!p.muzzle_down(m), "seed {s}: the control: with nobody in his line his muzzle is up"); }
        p.order("move", [34.5, 11.0], Who::Blue);
        let (mut crossed, mut up) = (0, 0);
        let (blue2, red2) = (blue.clone(), red.clone());
        p.until(30.0, |p| {
            for &w in &red2 {
                let Some(q) = p.watch_of(w) else { continue };
                if !p.arrived(w) { continue; }
                let (a, y) = (p.at(w), yaw_to(p.at(w), q));
                let (fx, fz) = (y.sin(), y.cos());
                let hit = blue2.iter().any(|&b| { let (sx, sz) = (p.at(b)[0] - a[0], p.at(b)[1] - a[1]); let al = sx * fx + sz * fz; al > 0.3 && al < dist(a, q) - 0.1 && (sx * fz - sz * fx).abs() < 0.3 });
                if hit { crossed += 1; if !p.muzzle_down(w) { up += 1; } }
            }
            blue2.iter().all(|&m| p.arrived(m))
        }).unwrap_or_else(|| panic!("seed {s}: blue never got there: {}", p.roll()));
        p.order("move", [17.5, 11.0], Who::Blue);
        p.until(40.0, |p| {
            for &w in &red2 {
                let Some(q) = p.watch_of(w) else { continue };
                if !p.arrived(w) { continue; }
                let (a, y) = (p.at(w), yaw_to(p.at(w), q));
                let (fx, fz) = (y.sin(), y.cos());
                let hit = blue2.iter().any(|&b| { let (sx, sz) = (p.at(b)[0] - a[0], p.at(b)[1] - a[1]); let al = sx * fx + sz * fz; al > 0.3 && al < dist(a, q) - 0.1 && (sx * fz - sz * fx).abs() < 0.3 });
                if hit { crossed += 1; if !p.muzzle_down(w) { up += 1; } }
            }
            blue2.iter().all(|&m| p.arrived(m))
        }).unwrap_or_else(|| panic!("seed {s}: blue never came back"));
        p.run(1.5);
        assert!(crossed > 10 && up <= 2, "seed {s}: {crossed} man-frames in a red line, {up} with the muzzle still up");
        for &m in &red { assert!(!p.muzzle_down(m), "seed {s}: back up once blue is out of the line"); }
    }
}

/// THE RULE: "the second team threw a flashbang when our guys were already on the inside". A bang and clear given to one pair with a man of the other inside the room as the door opens: the can stays in the kit, the beat says who, they go in, he is not dazzled. The control is the same order with nobody of ours inside: it throws its can.
/// A FAILURE MEANS: a can into a room our own men hold.
#[test]
fn no_can_into_a_room_our_own_men_hold() {
    for s in SEEDS {
        for inside in [true, false] {
            let (mut p, a, _) = two_doors(s);
            let (red, blue) = (p.red(), p.blue());
            let mate = blue[0];
            let spot = [p.frame_at(a)[0], p.frame_at(a)[1] - 2.0];
            let bangs = p.left().1;
            let side = p.face(a, [p.frame_at(a)[0], p.frame_at(a)[1] + 1.0]);
            p.order("bangClear", p.frame_at(a), Who::Red);
            p.run(0.1);
            let op = p.ops_on(a)[0];
            let (mut dazzled, mut opened) = (false, false);
            p.until(40.0, |p| {
                opened = opened || !p.door_shut(a);
                if inside && opened { p.man_stands(mate, spot); }
                dazzled = dazzled || p.blind(mate);
                red.iter().any(|&m| p.off(a, p.at(m), side) < -0.15)
            }).unwrap_or_else(|| panic!("seed {s}, inside {inside}: nobody went in: {:?}", p.op_beats(op)));
            p.run(1.0);
            let beats = p.op_beats(op);
            let thrown = beats.iter().any(|b| b.starts_with("toss:"));
            assert_eq!(thrown, !inside, "seed {s}, inside {inside}: {beats:?}");
            assert_eq!(p.left().1 + if thrown { 1.0 } else { 0.0 }, bangs, "seed {s}, inside {inside}: the kit");
            if inside { assert!(beats.iter().any(|b| b.starts_with("no bang · ") && b.ends_with("is in there")) && !dazzled, "seed {s}: {beats:?}"); }
        }
    }
}
