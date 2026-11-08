//! THE WORD: it sends one or two in from a plain stack, and when it spends no can it waits for the cuts.
mod doors_ask;
use doors_ask::*;
use game::doortools::{DoorToolKind, Phase};
use game::game::GuardState;
use game::ids::*;
use game::orders::Who;
use game_start::play::{Opts, Play};

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

/// THE RULE: one or two teammates stacked on a door by the quick key wait on the word's key as the squad of four does: the door carries the key's mark and would go now; the key files one 'go' for all of
/// them, an open-and-clear is made from the stack and a man goes through. The key again a second later leaves them on that entry. The control is the same key with no stack made: no mark, no order.
/// A FAILURE MEANS: a small squad stacks and is stuck there, or the word sends it in twice.
#[test]
fn the_word_sends_one_or_two_in_from_a_plain_stack() {
    for s in SEEDS {
        for n in [2, 1] {
            let (mut p, d, at) = at_the_door(s, n);
            p.run(0.5);
            assert_eq!(p.stack_mark(d), None, "seed {s}: the control: a mark on a door nobody is stacked on");
            p.tap("KeyF");
            p.run(0.5);
            assert!(p.ops_on(d).is_empty(), "seed {s}: the control: the key made an order with no stack");
            p.key_tapped_on([at[0], 1.2, at[1]]);
            let men = p.men();
            p.until(40.0, |p| men.iter().all(|&m| p.doing(m).as_deref() == Some("stack") && p.arrived(m))).unwrap_or_else(|| panic!("seed {s}, {n}: never stacked"));
            p.run(0.5);
            assert_eq!(p.stack_mark(d), Some((Some(String::from("F")), true)), "seed {s}, {n}: the door's mark");
            let given = p.orders_given();
            p.tap("KeyF");
            assert_eq!(p.last_order().map(|o| o.0), Some(String::from("go")), "seed {s}, {n}: filed");
            assert_eq!(p.orders_given(), given + 1, "seed {s}, {n}");
            let op = p.ops_on(d)[0];
            assert_eq!(p.op_kind(op), DoorToolKind::OpenClear, "seed {s}, {n}");
            assert!(men.iter().all(|m| p.op_men(op).contains(m)) && p.op_beats(op).iter().any(|b| b.starts_with("from the stack")), "seed {s}, {n}: {:?}", p.op_beats(op));
            p.run(1.0);
            p.tap("KeyF");
            p.run(0.5);
            assert_eq!(p.orders_given(), given + 1, "seed {s}, {n}: the second press filed something");
            assert!(p.ops_on(d) == vec![op] && men.iter().all(|m| p.op_men(op).contains(m)), "seed {s}, {n}: taken off the entry");
            p.until(25.0, |p| !p.across(d, p.face(d, [at[0], at[1] + 1.0])).is_empty()).unwrap_or_else(|| panic!("seed {s}, {n}: nobody through: {:?}", p.op_beats(op)));
        }
    }
}

/// a held door, its man awake and pinned on it, the word a second and a half into the hold with the cuts not done
fn word_on_the_cuts(s: u32, again: Option<f64>, cap: Option<f64>) -> Vec<(f64, String)> {
    let (sc, at) = yard(s);
    let mut p = Play::scene(&sc, &Opts { seed: s, squad: 2, ..Default::default() });
    p.sam_unhurt();
    p.men_spared();
    p.run(0.2);
    if let Some(c) = cap { p.word_cap_put(c); }
    let (foe, keep) = (p.defender_at([11.25, 11.25]), p.defender_at([24.25, 12.25]));
    p.pin_man(keep, [24.25, 12.25], 0.0, GuardState::Patrol, 0.0);
    p.pin_man(foe, [11.25, 11.25], yaw_to([11.25, 11.25], at), GuardState::Alert, 1.0);
    let d = p.door("west-yard");
    p.order("thresholdHold", at, Who::All);
    p.until(3.0, |p| !p.ops_on(d).is_empty());
    let op = p.ops_on(d)[0];
    p.until(40.0, |p| p.op_phase(op) == Phase::Thr).unwrap_or_else(|| panic!("seed {s}: never on the jambs: {:?}", p.op_beats(op)));
    p.run(0.5);
    assert!(!p.op_beats(op).iter().any(|b| b.starts_with("sliced")), "seed {s}: the staging: the cuts are done already: {:?}", p.op_beats(op));
    let n0 = p.op_beats(op).len();
    let t0 = p.time();
    p.tap("KeyF");
    if let Some(a) = again { p.run(a); p.tap("KeyF"); }
    p.until(25.0, |p| p.op_beats(op).iter().any(|b| b.starts_with("step in")));
    let (ts, what) = (p.beat_times(op), p.op_beats(op));
    ts.into_iter().zip(what).skip(n0).map(|(t, w)| (t - t0, w)).collect()
}

/// THE RULE: the word given while the jamb man is still cutting the doorway is taken and waits: "the word waits for the cuts", he finishes, "the cuts are done · in", and only then the step in, and no can
/// goes. The word again a second later: in at once, the cuts not waited for: the player's say stands. The cap at half a second: in when the cap is spent, said as that.
/// A FAILURE MEANS: the men walk off their cuts into the door's middle and are shot in its width: "ten of sixteen" on the tape that found it.
#[test]
fn the_word_that_spends_no_can_waits_for_the_cuts() {
    let at = |b: &[(f64, String)], w: &str| b.iter().find(|x| x.1.starts_with(w)).map(|x| x.0);
    for s in SEEDS {
        let b = word_on_the_cuts(s, None, None);
        let (waits, done, step) = (at(&b, "the word waits"), at(&b, "the cuts are done · in"), at(&b, "step in"));
        assert!(waits.map_or(false, |t| t < 0.2) && done.is_some() && step.is_some() && step >= done, "seed {s}: {b:?}");
        assert!(!b.iter().any(|x| x.1.starts_with("toss:")), "seed {s}: a can: {b:?}");
        let b = word_on_the_cuts(s, Some(1.0), None);
        assert!(at(&b, "the word waits").is_some() && at(&b, "the cuts are done").is_none() && at(&b, "step in").map_or(false, |t| t < 2.5), "seed {s}: the word again: {b:?}");
        let b = word_on_the_cuts(s, None, Some(0.5));
        assert!(at(&b, "the cuts not done in 0.5 s · in").map_or(false, |t| (t - 0.5).abs() < 0.2) && at(&b, "step in") >= at(&b, "the cuts not done"), "seed {s}: the cap: {b:?}");
    }
}
