//! SHIFT AND THE SQUAD'S KEY: "shift Q on a second door will naturally split the squad in two, and F tells all of them to go in"; "first shift Q should just send half the squad, whoever's closest";
//! "double tap Q -> cancel everything and fall in".
mod doors_ask;
use doors_ask::*;
use game::game::GuardState;
use game::ids::*;
use game_start::play::{Opts, Play};

fn yard_of(s: u32, squad: u32) -> (Play, DoorId, DoorId, [f64; 2], [f64; 2]) {
    let (sc, _) = yard(s);
    let mut p = Play::scene(&sc, &Opts { seed: s, squad, ..Default::default() });
    p.sam_unhurt();
    p.men_spared();
    p.run(0.2);
    let keep = p.defender_at([24.25, 12.25]);
    p.drop_all_but(&[keep]);
    p.pin_man(keep, [14.0, 4.0], 0.0, GuardState::Patrol, 0.0);
    p.run(0.2);
    let (a, b) = (p.door("west-yard"), p.door("east-yard"));
    let (ca, cb) = (p.frame_at(a), p.frame_at(b));
    (p, a, b, ca, cb)
}
fn chest(at: [f64; 2]) -> [f64; 3] { [at[0], 1.2, at[1]] }
fn floor(at: [f64; 2]) -> [f64; 3] { [at[0], 0.04, at[1]] }
fn on(p: &Play, d: DoorId) -> Vec<GuardId> { p.men().into_iter().filter(|&m| p.doing(m).as_deref() == Some("stack") && p.order_door(m) == Some(d)).collect() }
fn set(p: &Play, d: DoorId, n: usize) -> bool { let o = on(p, d); o.len() == n && o.iter().all(|&m| p.arrived(m)) }
fn fallen(p: &Play) -> usize { p.men().into_iter().filter(|&m| p.doing(m).as_deref() == Some("fallIn")).count() }
fn one_pair(p: &Play, m: &[GuardId]) -> bool { m.len() == 2 && p.callsign(m[0]).split(' ').next() == p.callsign(m[1]).split(' ').next() }
/// the groups covering, biggest first
fn covering(p: &Play) -> Vec<usize> {
    let mut by: Vec<([i64; 2], usize)> = Vec::new();
    for m in p.men() {
        if p.doing(m).as_deref() != Some("cover") { continue; }
        let Some(w) = p.watch_of(m) else { continue };
        let k = [(w[0] * 10.0) as i64, (w[1] * 10.0) as i64];
        match by.iter_mut().find(|x| x.0 == k) { Some(x) => x.1 += 1, None => by.push((k, 1)) }
    }
    let mut n: Vec<usize> = by.into_iter().map(|x| x.1).collect();
    n.sort_by(|a, b| b.cmp(a));
    n
}
const SPOTS: [[f64; 2]; 5] = [[9.0, 24.0], [19.0, 25.0], [23.0, 24.0], [27.0, 22.0], [7.0, 21.0]];

/// THE RULE: "first shift Q should just send half the squad, whoever's closest": nobody on anything, Shift and the key on a door: one whole pair goes, the nearer, the rest stay with him. On the second door the two
/// who stayed go. Which pair is nearer decides: the control is the pairs stood the other way round. The switch off, the first goal is all four's.
/// A FAILURE MEANS: the player cannot peel off half the squad, or peels the far men off past the near ones.
#[test]
fn the_first_goal_sends_the_nearer_pair_and_the_second_door_the_rest() {
    for s in SEEDS {
        for near_red in [true, false] {
            let (mut p, a, b, ca, cb) = yard_of(s, 4);
            let men = p.men();
            let (near, far) = if near_red { (&men[0..2], &men[2..4]) } else { (&men[2..4], &men[0..2]) };
            for (i, &m) in near.iter().enumerate() { p.man_stands(m, [ca[0] - 0.8 + i as f64 * 1.6, ca[1] + 3.0]); }
            for (i, &m) in far.iter().enumerate() { p.man_stands(m, [cb[0] - 0.8 + i as f64 * 1.6, cb[1] + 8.0]); }
            p.run(0.1);
            p.shift_tap(chest(ca));
            let got = on(&p, a);
            assert!(one_pair(&p, &got) && near.iter().all(|m| got.contains(m)), "seed {s}, near red {near_red}: {:?}", got);
            assert!(far.iter().all(|&m| p.doing(m).as_deref() != Some("stack")), "seed {s}: the rest stay with him");
            p.run(0.3);
            p.shift_tap(chest(cb));
            assert!(on(&p, a).len() == 2 && far.iter().all(|m| on(&p, b).contains(m)), "seed {s}: the second door");
        }
        let (mut p, a, _, ca, _) = yard_of(s, 4);
        p.goal_half_off();
        p.shift_tap(chest(ca));
        assert_eq!(on(&p, a).len(), 4, "seed {s}: the control: the switch off, the first goal is everyone's");
    }
}

/// THE RULE: nobody on anything, Shift and the key on a spot: two cover it. A second spot: two cover each. A third: two, one and one. Every group down to one man: one more spot gives nothing and the squad says so.
/// A FAILURE MEANS: "shift Q on a spot to watch automatically has them split up angles" has gone, or a man is taken off a goal he was given to make a fifth.
#[test]
fn goals_on_spots_split_the_squad_until_there_is_nobody_left() {
    for s in SEEDS {
        let (mut p, ..) = yard_of(s, 4);
        let mut want = vec![vec![2], vec![2, 2], vec![2, 1, 1], vec![1, 1, 1, 1]].into_iter();
        for &q in &SPOTS[..4] { p.shift_tap(floor(q)); p.run(0.2); assert_eq!(covering(&p), want.next().unwrap(), "seed {s}"); }
        let (n0, said0) = (p.orders_given(), p.said.len());
        p.shift_tap(floor(SPOTS[4]));
        p.run(0.3);
        assert_eq!((covering(&p), p.orders_given()), (vec![1, 1, 1, 1], n0), "seed {s}: a fifth gave something");
        let _ = said0;
    }
}

/// THE RULE: "F tells all of them to go in": both doors stacked, ONE press of the word's key starts an entry on each, on one word, and men are through both within two seconds of each other. The key without Shift after
/// that split is everyone's again: one goal. The control is the word with nobody stacked: nothing starts.
/// A FAILURE MEANS: the two pairs go in apart, or the plain key leaves half the squad on the old goal.
#[test]
fn one_word_sends_both_pairs_in_and_the_plain_key_makes_one_goal_again() {
    for s in SEEDS {
        let (mut p, a, b, ca, cb) = yard_of(s, 4);
        p.shift_tap(chest(ca));
        p.run(0.3);
        p.shift_tap(chest(cb));
        p.until(60.0, |p| set(p, a, 2) && set(p, b, 2)).unwrap_or_else(|| panic!("seed {s}: never both stacked"));
        p.run(0.5);
        p.tap("KeyF");
        let (oa, ob) = (p.ops_on(a), p.ops_on(b));
        assert!(oa.len() == 1 && ob.len() == 1, "seed {s}: entries {oa:?} {ob:?}");
        for op in [oa[0], ob[0]] { assert!(p.op_beats(op).iter().any(|w| w == "the word"), "seed {s}: {:?}", p.op_beats(op)); }
        let yard_side = |p: &Play, d: DoorId| p.face(d, [p.frame_at(d)[0], p.frame_at(d)[1] + 1.0]);
        let (sa, sb) = (yard_side(&p, a), yard_side(&p, b));
        let (mut ta, mut tb) = (None, None);
        let t0 = p.time();
        p.until(30.0, |p| {
            if ta.is_none() && !p.across(a, sa).is_empty() { ta = Some(p.time() - t0); }
            if tb.is_none() && !p.across(b, sb).is_empty() { tb = Some(p.time() - t0); }
            ta.is_some() && tb.is_some()
        }).unwrap_or_else(|| panic!("seed {s}: not through both: {ta:?} {tb:?}"));
        assert!((ta.unwrap() - tb.unwrap()).abs() <= 2.0, "seed {s}: {ta:?} {tb:?}");
        drop(p);

        let (mut p, a, b, ca, cb) = yard_of(s, 4);
        p.tap("KeyF");
        assert!(!p.any_op(), "seed {s}: the control: the word with nobody stacked");
        p.shift_tap(chest(ca));
        p.run(0.3);
        p.shift_tap(chest(cb));
        p.run(0.5);
        assert!(on(&p, a).len() == 2 && on(&p, b).len() == 2, "seed {s}: split");
        p.key_tapped_on(chest(ca));
        p.run(0.3);
        assert!(on(&p, a).len() == 4 && on(&p, b).is_empty(), "seed {s}: the plain key: {} and {}", on(&p, a).len(), on(&p, b).len());
    }
}

/// THE RULE: "let's make shift-Q long press open the radial menu to append another order too". All four on a door, the keys kept down on the second: the ring is one whole pair's, named on it, nobody is told anything; its first
/// wedge sends that pair there and the other stays. Nobody on anything, kept down on a door: one pair's too. All four covering, on a spot: two; a third spot: one. Every group down to one man: no ring, and nothing given.
/// A FAILURE MEANS: the ring gives before it is asked, or gives to everyone.
#[test]
fn kept_down_the_key_is_that_parts_ring() {
    for s in SEEDS {
        let (mut p, a, b, ca, cb) = yard_of(s, 4);
        p.key_tapped_on(chest(ca));
        p.until(40.0, |p| set(p, a, 4)).unwrap_or_else(|| panic!("seed {s}: never stacked"));
        let n0 = p.orders_given();
        assert!(p.shift_held(chest(cb)), "seed {s}: no ring");
        let part = p.ring_part();
        assert!(one_pair(&p, &part), "seed {s}: the ring is for {part:?}");
        let pair = p.callsign(part[0]).split(' ').next().unwrap().to_string();
        assert!(p.ring_caption().to_lowercase().starts_with(&pair.to_lowercase()), "seed {s}: under it '{}'", p.ring_caption());
        assert_eq!(p.orders_given(), n0, "seed {s}: told while it was up");
        p.wedge_taken(None);
        p.run(0.3);
        assert!(part.iter().all(|m| on(&p, b).contains(m)) && on(&p, a).len() == 2, "seed {s}: the wedge");
        drop(p);

        let (mut p, a, _, ca, _) = yard_of(s, 4);
        assert!(p.shift_held(chest(ca)), "seed {s}");
        let part = p.ring_part();
        assert!(one_pair(&p, &part), "seed {s}: half, on the ring: {part:?}");
        p.wedge_taken(None);
        p.run(0.3);
        assert_eq!(on(&p, a).len(), 2, "seed {s}");
        drop(p);

        let (mut p, ..) = yard_of(s, 4);
        p.shift_tap(floor(SPOTS[0]));
        p.run(0.2);
        p.shift_tap(floor(SPOTS[1]));
        p.run(0.2);
        assert_eq!(covering(&p), vec![2, 2], "seed {s}");
        assert!(p.shift_held(floor(SPOTS[2])), "seed {s}");
        assert_eq!(p.ring_part().len(), 1, "seed {s}: the third spot's ring is one man's");
        let one = p.ring_part()[0];
        p.wedge_taken(None);
        p.run(0.2);
        assert_eq!((p.doing(one).as_deref(), covering(&p)), (Some("move"), vec![2, 1]), "seed {s}: its first wedge");
        drop(p);

        let (mut p, ..) = yard_of(s, 4);
        for &q in &SPOTS[..4] { p.shift_tap(floor(q)); p.run(0.2); }
        let n0 = p.orders_given();
        assert!(!p.shift_held(floor(SPOTS[4])), "seed {s}: every group is one man and a ring came up");
        p.keys_let_go();
        assert_eq!((p.orders_given(), covering(&p)), (n0, vec![1, 1, 1, 1]), "seed {s}");
    }
}

/// THE RULE: half of a small squad. One teammate: the door is his. Two: the nearer alone, then the other. Three: the nearer of the whole pair and the odd man, then the rest.
/// A FAILURE MEANS: a goal strands a small squad, or rounds the wrong way.
#[test]
fn a_small_squad_splits_by_the_same_rule() {
    for s in SEEDS {
        let (mut p, a, _, ca, _) = yard_of(s, 1);
        p.shift_tap(chest(ca));
        assert_eq!(on(&p, a).len(), 1, "seed {s}: one man: his");
        drop(p);

        let (mut p, a, b, ca, cb) = yard_of(s, 2);
        let men = p.men();
        p.man_stands(men[1], [ca[0], ca[1] + 3.0]);
        p.man_stands(men[0], [cb[0], cb[1] + 8.0]);
        p.run(0.1);
        p.shift_tap(chest(ca));
        assert!(on(&p, a) == vec![men[1]] && p.doing(men[0]).as_deref() != Some("stack"), "seed {s}: two men: the nearer alone: {:?}", on(&p, a));
        p.run(0.3);
        p.shift_tap(chest(cb));
        assert_eq!(on(&p, b), vec![men[0]], "seed {s}: the other");
        drop(p);

        let (mut p, a, b, ca, cb) = yard_of(s, 3);
        p.shift_tap(chest(ca));
        let first = on(&p, a).len();
        assert_eq!(first, 2, "seed {s}: three men: {first} on the door");
        p.run(0.3);
        p.shift_tap(chest(cb));
        assert_eq!(on(&p, b).len(), 1, "seed {s}: the rest");
    }
}

/// THE RULE: "double tap Q -> cancel everything and fall in": the key on a door sends all four at once; tapped again inside the window, all fall in, and two pairs on two doors likewise, with no door tool left running. After the
/// window nobody falls in. A tap on one door and at once on the other is the second door's. The plain tap after a Shift tap is its own order. A spot and a spot a pace off is the second spot's. Three taps: stack, fall in, stack.
/// On himself the first tap is fall in. The control is the switch off: tapped twice, nobody falls in.
/// A FAILURE MEANS: a quick hand cancels what it meant to give, or the double tap does not cancel.
#[test]
fn the_key_tapped_twice_is_fall_in() {
    for s in SEEDS {
        let (mut p, a, b, ca, cb) = yard_of(s, 4);
        p.key_tapped_on(chest(ca));
        assert_eq!(on(&p, a).len(), 4, "seed {s}: at once");
        p.key_tapped_on(chest(ca));
        assert!(fallen(&p) == 4 && on(&p, a).is_empty(), "seed {s}: T1: {} fell in", fallen(&p));
        p.run(p.twice_secs() + 0.2);
        p.key_tapped_on(chest(ca));
        assert_eq!(on(&p, a).len(), 4, "seed {s}: T10: the third tap starts again");
        p.run(p.twice_secs() + 0.2);
        p.key_tapped_on(chest(ca));
        assert_eq!((fallen(&p), on(&p, a).len()), (0, 4), "seed {s}: T3: after the window");
        p.run(p.twice_secs() + 0.2);
        p.key_tapped_on(chest(ca));
        p.key_tapped_on(chest(cb));
        assert_eq!((fallen(&p), on(&p, b).len()), (0, 4), "seed {s}: T4: the second door's");
        drop(p);

        let (mut p, a, b, ca, cb) = yard_of(s, 4);
        p.shift_tap(chest(ca));
        p.key_tapped_on(chest(ca));
        assert_eq!((fallen(&p), on(&p, a).len()), (0, 4), "seed {s}: T5: the plain key after a Shift tap");
        p.run(p.twice_secs() + 0.2);
        let sam = p.sam_at();
        p.order("fallIn", sam, game::orders::Who::All);
        p.run(0.5);
        p.shift_tap(chest(ca));
        p.run(0.3);
        p.shift_tap(chest(cb));
        p.until(60.0, |p| set(p, a, 2) && set(p, b, 2)).unwrap_or_else(|| panic!("seed {s}: never both stacked"));
        p.key_tapped_on(chest(ca));
        p.key_tapped_on(chest(ca));
        assert!(fallen(&p) == 4 && !p.any_op() && on(&p, a).is_empty() && on(&p, b).is_empty(), "seed {s}: T2: {} fell in", fallen(&p));
        drop(p);

        let (mut p, ..) = yard_of(s, 4);
        p.key_tapped_on(floor(SPOTS[0]));
        p.key_tapped_on(floor([SPOTS[0][0] + 1.2, SPOTS[0][1]]));
        assert_eq!(fallen(&p), 0, "seed {s}: T8: the hand moved");
        p.run(p.twice_secs() + 0.2);
        p.cursor_on(floor([17.5, 21.0]));
        p.run(4.0);   // (the view comes to rest under the hand: the pixel it stays on is what the rule reads)
        p.key_tapped_on(floor([17.5, 21.0]));
        p.key_tapped_on(floor([17.5, 21.0]));
        assert_eq!(fallen(&p), 4, "seed {s}: T9: the hand did not");
        p.run(p.twice_secs() + 0.2);
        let at = p.sam_at();
        p.cursor_on([at[0], 1.0, at[1]]);
        p.run(4.0);
        p.key_tapped_on([at[0], 1.0, at[1]]);
        assert_eq!(fallen(&p), 4, "seed {s}: T11: on himself");
        let n = p.orders_given();
        p.key_tapped_on([at[0], 1.0, at[1]]);
        assert_eq!((fallen(&p), p.orders_given()), (4, n), "seed {s}: T11: the second tells them nothing more");
        drop(p);

        let (mut p, a, _, ca, _) = yard_of(s, 4);
        p.twice_off();
        p.key_tapped_on(chest(ca));
        p.key_tapped_on(chest(ca));
        assert!(fallen(&p) == 0 && on(&p, a).len() == 4, "seed {s}: the control: the switch off");
    }
}
