//! A ROOM WORKED FROM THE THRESHOLD, AND A GRENADE ON A DOOR ORDER: one man cuts it before it is called held, a can goes first once the floor is loud, and an order waits for the men a grenade scattered.
mod doors_ask;
use doors_ask::*;
use game::doortools::Phase;
use game::game::GuardState;
use game::ids::*;
use game::orders::Who;
use game_start::play::{dist, Opts, Play};

/// the west room empty, one man pinned behind the far wall, a squad of `squad` in the yard
fn at_the_door(s: u32, squad: u32) -> (Play, DoorId, [f64; 2]) {
    let (sc, at) = yard(s);
    let mut p = Play::scene(&sc, &Opts { seed: s, squad, ..Default::default() });
    p.sam_unhurt();
    p.men_spared();
    p.run(0.2);
    let keep = p.defender_at([24.25, 12.25]);
    p.drop_all_but(&[keep]);
    p.pin_man(keep, [24.25, 12.25], 0.0, GuardState::Patrol, 0.0);
    let d = p.door("west-yard");
    (p, d, at)
}
fn op_of(p: &mut Play, d: DoorId, s: u32) -> DoorOpId { p.until(3.0, |p| !p.ops_on(d).is_empty()).unwrap_or_else(|| panic!("seed {s}: no order on the door")); p.ops_on(d)[0] }
fn has(p: &Play, op: DoorOpId, w: &str) -> bool { p.op_beats(op).iter().any(|b| b.contains(w)) }
/// everybody's feet off their marks by more than a metre and a half, when the leaf first moves
fn off_marks(p: &Play, men: &[GuardId]) -> Vec<f64> { men.iter().filter_map(|&m| p.post_of(m).map(|q| dist(p.at(m), q))).filter(|&x| x > 1.5).collect() }

/// THE RULE: a stack on Sam's word, a grenade lands on it and the word comes a second later. "the door waits" is said, the word makes no sound until "the men are back", and the leaf then moves with
/// every man within 1.5 m of his mark; the order ends in the room. The control is the same word with no grenade: the leaf goes, and no door waited.
/// A FAILURE MEANS: the door opens with half the stack running from a grenade, or the order is lost to it.
#[test]
fn a_grenade_scatters_a_stack_and_the_door_waits_for_it() {
    for s in SEEDS {
        for frag in [true, false] {
            let (mut p, d, at) = at_the_door(s, 4);
            p.order_on_word("openClear", at, Who::All);
            let op = op_of(&mut p, d, s);
            p.until(40.0, |p| p.op_phase(op) == Phase::Ready || has(p, op, "stacked")).unwrap_or_else(|| panic!("seed {s}: never stacked: {:?}", p.op_beats(op)));
            p.run(0.5);
            let men = p.men();
            if frag { let f = p.at(p.op_lead(op)); p.frag_lands([f[0], at[1] + 3.2]); }
            p.run(1.0);
            p.tap("KeyF");
            let a0 = p.leaf_angle(d);
            p.until(40.0, |p| (p.leaf_angle(d) - a0).abs() > 0.02).unwrap_or_else(|| panic!("seed {s}: frag {frag}: the door never went: {:?}", p.op_beats(op)));
            assert_eq!(off_marks(&p, &men), Vec::<f64>::new(), "seed {s}: frag {frag}: off his mark as the leaf moved: {:?}", p.op_beats(op));
            assert_eq!(has(&p, op, "the door waits"), frag, "seed {s}: frag {frag}: {:?}", p.op_beats(op));
            assert_eq!(has(&p, op, "the men are back"), frag, "seed {s}: frag {frag}: {:?}", p.op_beats(op));
            assert!(!has(&p, op, "on without"), "seed {s}: {:?}", p.op_beats(op));
            p.until(30.0, |p| !p.across(d, 1.0).is_empty()).unwrap_or_else(|| panic!("seed {s}: frag {frag}: nobody went in: {:?}", p.op_beats(op)));
        }
    }
}

/// THE RULE: a charge, a grenade lands as it is going on the door: the man setting it is not given up on, and "armed" is said with nobody running and every man on his mark. The control is the same
/// charge with no grenade.
/// A FAILURE MEANS: the order gives up its man at the lock, or calls armed with the squad in the open.
#[test]
fn a_charge_waits_for_the_men_a_grenade_scattered() {
    for s in SEEDS {
        for frag in [true, false] {
            let (mut p, d, at) = at_the_door(s, 2);
            p.order("charge", at, Who::All);
            let op = op_of(&mut p, d, s);
            p.until(30.0, |p| p.from_handle(d, p.at(p.op_lead(op))) < 1.2).unwrap_or_else(|| panic!("seed {s}: never at the door: {:?}", p.op_beats(op)));
            p.run(0.3);
            if frag { let f = p.at(p.op_lead(op)); p.frag_lands([f[0], at[1] + 3.2]); }
            let men = p.men();
            p.until(60.0, |p| has(p, op, "armed")).unwrap_or_else(|| panic!("seed {s}: frag {frag}: never armed: {:?}", p.op_beats(op)));
            assert_eq!(off_marks(&p, &men), Vec::<f64>::new(), "seed {s}: frag {frag}: {:?}", p.op_beats(op));
            assert!(has(&p, op, "set"), "seed {s}: {:?}", p.op_beats(op));
            assert_eq!(has(&p, op, "grenade on the stack"), frag, "seed {s}: frag {frag}: {:?}", p.op_beats(op));
            assert!(!has(&p, op, "never made the door"), "seed {s}: {:?}", p.op_beats(op));
        }
    }
}

/// THE RULE: a door held from the threshold, a grenade a second before Sam calls for a can: the door waits, no can goes and nobody steps in until "the men are back"; then a can goes from within
/// the throw's reach of the frame and they go in. The same with the word: nobody steps in until they are back, and no can is spent. The control is the grenade's absence: no door waits.
/// A FAILURE MEANS: the men who ran are left behind, or a can goes in with nobody at the door to follow it.
#[test]
fn a_held_door_waits_for_the_men_a_grenade_scattered() {
    for s in SEEDS {
        for (row, frag) in [(Some("bangClear"), true), (None, true), (Some("bangClear"), false)] {
            let (mut p, d, at) = at_the_door(s, 2);
            p.order("thresholdHold", at, Who::All);
            let op = op_of(&mut p, d, s);
            p.until(50.0, |p| p.thr_told(op)).unwrap_or_else(|| panic!("seed {s}: never held: {:?}", p.op_beats(op)));
            p.run(2.0);
            if frag { let f = p.at(p.op_lead(op)); p.frag_lands([f[0], at[1] + 3.2]); }
            p.run(1.0);
            match row { Some(r) => { assert_eq!(p.try_order(r, at, Who::All), None, "seed {s}"); } None => p.tap("KeyF") }
            p.until(50.0, |p| p.op_beats(op).iter().any(|b| b.starts_with("step in"))).unwrap_or_else(|| panic!("seed {s}: {row:?} frag {frag}: nobody went: {:?}", p.op_beats(op)));
            let beats = p.op_beats(op);
            let at_beat = |w: &str| beats.iter().position(|b| b.contains(w));
            assert_eq!(at_beat("the door waits").is_some(), frag, "seed {s}: {row:?} frag {frag}: {beats:?}");
            if frag {
                let back = at_beat("the men are back").unwrap_or_else(|| panic!("seed {s}: never back: {beats:?}"));
                for (i, b) in beats.iter().enumerate() { if b.starts_with("toss:") || b.starts_with("step in") { assert!(i > back, "seed {s}: {b} before they were back: {beats:?}"); } }
            }
            assert_eq!(beats.iter().filter(|b| b.starts_with("toss:")).count() > 0, row.is_some(), "seed {s}: {row:?}: {beats:?}");
        }
    }
}

/// THE RULE: "men should try to pie from the outside as much as they can before considering their job done ... really only one guy should be doing it ... just do the lean, not the crouch". A held
/// door: the lead walks his cuts with his hips on each, the far jamb's man has none and stays on his diagonal, the lead leans once, after his last cut, his head past his hips and toward the middle;
/// nobody kneels, nobody crosses the frame line; only then is the door called held, and everybody then stands square. The word given on the lean takes him off it. The control is a plain entry: no jamb men, no cuts.
/// A FAILURE MEANS: the door is called held with floor unseen, or both men shuffle in step, or a man steps into the opening to look.
#[test]
fn one_man_cuts_the_door_before_it_is_called_held() {
    for s in SEEDS {
        let (mut p, d, at) = at_the_door(s, 2);
        p.order("thresholdHold", at, Who::All);
        let op = op_of(&mut p, d, s);
        let side = p.face(d, [at[0], at[1] + 1.0]);
        let (mut cuts, mut stood, mut leans, mut leaning, mut lean_frames, mut head, mut low, mut over) = (std::collections::BTreeSet::new(), std::collections::BTreeSet::new(), 0, false, 0, -9.0f64, 0.0f64, 9.0f64);
        let (mut far_home, mut far_strayed, mut cut_after_lean) = (None, 0.0f64, false);
        let frame = |p: &mut Play| {
            for m in p.thr_jamb(op) {
                low = low.max(p.knee_of(m));
                over = over.min(p.off(d, p.at(m), side));
                match p.cut_of(op, m) {
                    Some((n, i, q, _)) if n > 0 => {
                        cuts.insert(i);
                        if dist(p.at(m), q) < 0.15 { stood.insert(i); }
                        let now = p.leaning(m);
                        if now && !leaning { leans += 1; }
                        if now { lean_frames += 1; let c = p.frame_at(d); let w = yaw_to(p.at(m), c); head = head.max(p.head_past_hips(m, [w.sin(), w.cos()])); }
                        if leaning && !now { cut_after_lean = cut_after_lean || i > *cuts.iter().max().unwrap_or(&0); }
                        leaning = now;
                    }
                    _ => { if cuts.len() > 1 { let a = p.at(m); let h = *far_home.get_or_insert(a); far_strayed = far_strayed.max(dist(a, h)); } assert!(!p.leaning(m), "seed {s}: the man holding leaned"); }
                }
            }
            p.thr_told(op)
        };
        let mut frame = frame;
        p.until(50.0, |p| frame(p)).unwrap_or_else(|| panic!("seed {s}: never held: {:?}", p.op_beats(op)));
        assert!(cuts.len() >= 2 && cuts == stood, "seed {s}: sent to cuts {cuts:?}, stood on {stood:?}");
        assert!(far_strayed < 0.3, "seed {s}: the other man went {far_strayed:.2} m off his diagonal");
        assert_eq!(leans, 1, "seed {s}: one lean");
        assert!(lean_frames >= 45 && head > 0.0, "seed {s}: {lean_frames} frames, head {head:.2} past his hips");
        assert!(low < 0.1, "seed {s}: nobody kneels: {low:.2}");
        assert!(over > 0.3, "seed {s}: {over:.2} m from the frame line");
        p.run(1.0);
        for m in p.thr_jamb(op) { assert!(!p.leaning(m), "seed {s}: still leaning with the door held"); }
        drop(p);

        let (mut p, d, at) = at_the_door(s, 2);
        p.order("openClear", at, Who::All);
        let op = op_of(&mut p, d, s);
        p.run(3.0);
        assert!(p.thr_jamb(op).is_empty(), "seed {s}: the control: a plain entry has its men on no jamb");
    }
}

/// THE RULE: a door worked from the threshold with a man of theirs in the room in line with it. As the leaf moves a man stands on each jamb, the lead one of them, everybody outside. The man is engaged
/// from outside, through the opening, with nobody over the frame line. The word then brings "step in", and only after it does a foot go over. The order of it: stacked, threshold, held, step in.
/// A FAILURE MEANS: they go in before they have looked, or stand in the opening to do it.
#[test]
fn a_pair_works_a_held_room_from_its_jambs() {
    for s in SEEDS {
        let (sc, at) = yard(s);
        let mut p = Play::scene(&sc, &Opts { seed: s, squad: 2, ..Default::default() });
        p.sam_unhurt();
        p.men_spared();
        p.run(0.2);
        let (foe, keep) = (p.defender_at([11.25, 11.25]), p.defender_at([24.25, 12.25]));
        p.pin_man(keep, [24.25, 12.25], 0.0, GuardState::Patrol, 0.0);
        let d = p.door("west-yard");
        let (spot, side, w) = ([at[0], 12.5], p.face(d, [at[0], at[1] + 1.0]), p.door_w(d));
        p.order("thresholdHold", at, Who::All);
        let op = op_of(&mut p, d, s);
        let a0 = p.leaf_angle(d);
        let (mut on_the_move, mut inside, mut steps) = (None, 0, false);
        for _ in 0..50 * 60 {
            if p.up(foe) { p.pin_man(foe, spot, yaw_to(spot, at), GuardState::Alert, 1.0); p.blind_now(foe); }
            p.frame();
            if on_the_move.is_none() && (p.leaf_angle(d) - a0).abs() > 0.02 { on_the_move = Some(p.men().iter().map(|&m| (m, p.along(d, p.at(m)), p.off(d, p.at(m), side))).collect::<Vec<_>>()); }
            for m in p.men() { if p.off(d, p.at(m), side) < 0.0 { inside += 1; } }
            if p.thr_told(op) && !p.up(foe) { steps = true; break; }
        }
        assert!(steps, "seed {s}: the door was never called held with the man down: {:?}", p.op_beats(op));
        let on = on_the_move.unwrap();
        let (hi, lo) = (on.iter().filter(|x| x.1 > w && x.1 < w + 1.6 && x.2 > 0.0 && x.2 < 1.3).count(), on.iter().filter(|x| x.1 < 0.0 && x.1 > -1.6 && x.2 > 0.0 && x.2 < 1.3).count());
        assert!(hi >= 1 && lo >= 1, "seed {s}: a man on each jamb: {on:?}");
        assert_eq!(inside, 0, "seed {s}: man-frames over the line before the word");
        assert!(p.squad_rounds() > 0.0, "seed {s}: the man was not engaged from outside");
        p.tap("KeyF");
        p.until(30.0, |p| p.op_beats(op).iter().any(|b| b.starts_with("step in"))).unwrap_or_else(|| panic!("seed {s}: the word and no step in: {:?}", p.op_beats(op)));
        let beats = p.op_beats(op);
        let i = |w: &str| beats.iter().position(|b| b.starts_with(w)).unwrap_or_else(|| panic!("seed {s}: no '{w}': {beats:?}"));
        assert!(i("stacked") < i("threshold") && i("threshold") < i("step in"), "seed {s}: {beats:?}");
        p.until(30.0, |p| !p.across(d, side).is_empty()).unwrap_or_else(|| panic!("seed {s}: nobody went in"));
    }
}

/// THE RULE: on a floor gone loud a can goes in before anybody shows in the opening: the leaf moves, the first can, its bang, "out onto the diagonals behind the can", the hold. One can for the door, and
/// the word given while it is in the air is kept and taken as they step out, with no second can. The control is a quiet floor: no can first, nobody sent out behind one.
/// A FAILURE MEANS: men stand in an opening on a floor that knows they are coming, or the word is lost.
#[test]
fn a_can_goes_first_once_the_floor_is_loud() {
    for s in SEEDS {
        for word in [false, true] {
            let (mut p, d, at) = at_the_door(s, 2);
            p.threshold_given(at);
            let op = op_of(&mut p, d, s);
            let side = p.face(d, [at[0], at[1] + 1.0]);
            p.frag_lands([36.0, 26.0]);
            let mut said = false;
            let mut out_past = 0.0f64;
            let t = p.until(50.0, |p| {
                if word && !said && p.op_beats(op).iter().any(|b| b.starts_with("toss:")) { said = true; p.key_down("KeyF"); p.frame(); p.key_up("KeyF"); }
                if !p.op_beats(op).iter().any(|b| b.starts_with("out onto")) { for m in p.men() { out_past = out_past.min(p.off(d, p.at(m), side)); } }
                p.op_beats(op).iter().any(|b| b.starts_with("held "))
            });
            let beats = p.op_beats(op);
            assert!(t.is_some(), "seed {s}: word {word}: {beats:?}");
            let i = |w: &str| beats.iter().position(|b| b.contains(w)).unwrap_or_else(|| panic!("seed {s}: word {word}: no '{w}': {beats:?}"));
            assert!(i("a can first") < i("· first ·") && i("· first ·") < i("bang ·") && i("bang ·") < i("out onto the diagonals behind the can"), "seed {s}: {beats:?}");
            assert_eq!(beats.iter().filter(|b| b.starts_with("toss:")).count(), 1, "seed {s}: one can a door: {beats:?}");
            assert!(out_past >= 0.0, "seed {s}: over the line before they were sent out: {out_past:.2}");
            drop(p);
        }
        let (mut p, d, at) = at_the_door(s, 2);
        p.threshold_given(at);
        let op = op_of(&mut p, d, s);
        p.until(50.0, |p| p.op_beats(op).iter().any(|b| b.starts_with("held "))).unwrap_or_else(|| panic!("seed {s}: quiet: never held: {:?}", p.op_beats(op)));
        let beats = p.op_beats(op);
        assert!(!beats.iter().any(|b| b.contains("a can first") || b.starts_with("out onto") || b.starts_with("toss:")), "seed {s}: the control: {beats:?}");
    }
}
