//! A SILENT SQUAD: it picks the lock, holds its fire on a man who has not noticed, and goes loud only when a loud tool is picked or it is caught.
mod doors_ask;
use doors_ask::*;
use game::doortools::Phase;
use game::game::GuardState;
use game::ids::*;
use game::orders::Who;
use game_start::play::{Opts, Play};

/// the west door found locked, the room behind it empty, one man pinned behind the far wall; the squad in the yard
fn at_a_locked_door(s: u32) -> (Play, DoorId, [f64; 2]) {
    let (mut sc, at) = yard(s);
    sc.locked("west-yard");
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.run(0.2);
    let keep = p.defender_at([24.25, 12.25]);
    p.drop_all_but(&[keep]);
    p.pin_man(keep, [24.25, 12.25], 0.0, GuardState::Patrol, 0.0);
    p.run(0.3);
    let d = p.door("west-yard");
    (p, d, at)
}
fn go_silent(p: &mut Play) { let at = p.sam_at(); p.order("silent", at, Who::All); p.frame(); assert!(p.silent(), "the staging: the order took"); }
fn kicked(p: &Play, op: DoorOpId) -> bool { p.op_beats(op).iter().any(|b| b.contains("kick")) }
/// the lock taken by force: the boot, or the shotgun where the squad carries one
fn forced(p: &Play, op: DoorOpId) -> bool { kicked(p, op) || p.op_beats(op).iter().any(|b| b.contains("shotgun")) }

/// THE RULE: silent at a locked door, "open and clear": no boot. The lead says "locked. picking." over the net, works the lock at the door for its seconds, the lock is picked and not
/// broken, and they go in, still silent. The control is the same order on the same door with the squad loud: the door is found shut and they hold on it.
/// A FAILURE MEANS: a silent order breaks the lock, or stops at it, or the lock gives with nobody at it.
#[test]
fn a_silent_squad_picks_the_lock_it_would_have_booted() {
    for s in SEEDS {
        let (mut p, d, at) = at_a_locked_door(s);
        go_silent(&mut p);
        p.order("openClear", at, Who::All);
        p.until(3.0, |p| !p.ops_on(d).is_empty());
        let op = p.ops_on(d)[0];
        let first = p.until(40.0, |p| p.hands_on_lock().is_some()).unwrap_or_else(|| panic!("seed {s}: nobody at the lock: {:?}", p.op_beats(op)));
        let lead = p.hands_on_lock().unwrap();
        assert!(p.from_handle(d, p.at(lead)) < 1.3, "seed {s}: he is {:.2} m off the handle", p.from_handle(d, p.at(lead)));
        let took = p.until(10.0, |p| !p.door_locked_now(d)).unwrap_or_else(|| panic!("seed {s}: the lock never gave ({first:.1} s)"));
        assert!((took - game::doors::DOORS_PICK_SECS).abs() < 0.3, "seed {s}: {took:.2} s on the lock");
        assert!(!p.keep_broken(d), "seed {s}: picked, not broken");
        assert_eq!(p.said_count("locked. picking."), (1, true), "seed {s}: said once, over the net");
        p.until(40.0, |p| !p.across(d, 1.0).is_empty()).unwrap_or_else(|| panic!("seed {s}: nobody went in: {:?}", p.op_beats(op)));
        assert!(!forced(&p, op) && p.hands_on_lock().is_none() && p.silent(), "seed {s}: no boot, hands off, still silent: {:?}", p.op_beats(op));

        drop(p);
        let (mut p, d, at) = at_a_locked_door(s);
        p.order("openClear", at, Who::All);
        p.until(30.0, |p| p.op_ended_on(d)).unwrap_or_else(|| panic!("seed {s}: the control: the order never ended"));
        assert!(p.door_locked_now(d) && !p.keep_broken(d) && p.hands_on_lock().is_none(), "seed {s}: the control: loud, the door is left as it was found");
        assert_eq!(p.said_count("locked. picking.").0, 0, "seed {s}: the control");
    }
}

/// the two rooms with their men up, the squad in the yard, silent or not, told into the west room; both men kept unaware
fn in_on_men_who_have_not_noticed(s: u32, silent: bool) -> (Play, Vec<GuardId>) {
    let (sc, at) = yard(s);
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.run(0.2);
    let foes = vec![p.defender_at([11.25, 11.25]), p.defender_at([24.25, 12.25])];
    if silent { go_silent(&mut p); }
    p.order("openClear", at, Who::All);
    let mut past = None;   // (a silent squad watched on for three seconds after it first holds a round; a loud one to its first round)
    for f in 0..25 * 60 {
        p.pin_man(foes[0], [11.25, 11.25], 0.0, GuardState::Patrol, 0.0);
        p.pin_man(foes[1], [24.25, 12.25], 0.0, GuardState::Patrol, 0.0);
        p.frame();
        if p.squad_rounds() > 0.0 { break; }
        if silent && past.is_none() && p.said_count("eyes on. holding.").0 > 0 { past = Some(f + 3 * 60); }
        if past == Some(f) { break; }
    }
    (p, foes)
}

/// THE RULE: silent, guns on men who have not noticed, not one round, and they say so over the net. The men notice: the four shoot, the order stands, and the first loud thing is said once:
/// "compromised." The control is the same room with the squad loud: they shoot the same men where they stand.
/// A FAILURE MEANS: a silent squad gives itself away, or sits on its hands in a fight.
#[test]
fn a_silent_squad_holds_its_fire_until_it_is_noticed() {
    for s in SEEDS {
        let (mut p, foes) = in_on_men_who_have_not_noticed(s, true);
        assert_eq!(p.squad_rounds(), 0.0, "seed {s}: rounds on men who have not noticed");
        assert!(p.up(foes[0]) && p.up(foes[1]), "seed {s}");
        let (eyes, net) = p.said_count("eyes on. holding.");
        assert!(eyes > 0 && net, "seed {s}: they say so on the net: {eyes}");
        assert!(!p.compromised(), "seed {s}: nothing loud yet");
        for &f in &foes { let a = p.at(f); p.pin_man(f, a, 0.0, GuardState::Alert, 1.0); }
        p.until(15.0, |p| p.squad_rounds() > 0.0 && (!p.up(foes[0]) || !p.up(foes[1]))).unwrap_or_else(|| panic!("seed {s}: noticed, they fire: {} rounds", p.squad_rounds()));
        p.run(1.0);
        assert!(p.silent() && p.compromised(), "seed {s}: the order stands, compromised");
        assert_eq!(p.said_count("compromised."), (1, true), "seed {s}: said once, over the net");
        drop(p);

        let (p, _) = in_on_men_who_have_not_noticed(s, false);
        assert!(p.squad_rounds() > 0.0, "seed {s}: the control: loud, they shoot men who have not noticed");
    }
}

/// THE RULE: a loud tool stays on the door's menu and picking it ends silent at the order: bang and clear on a locked door flips the squad at once, and it then does what a loud squad does: it
/// halts on the lock and the same row again is the boot, then the can. The control is a quiet order at a door with the squad silent: held at the threshold nothing is thrown and it stays silent.
/// A FAILURE MEANS: a silent squad throws a can with its word still silent, or a loud tool is refused.
#[test]
fn only_a_loud_tool_ends_silent() {
    for s in SEEDS {
        let (mut p, d, at) = at_a_locked_door(s);
        go_silent(&mut p);
        assert_eq!(p.try_order("bangClear", at, Who::All), None, "seed {s}");
        assert!(!p.silent(), "seed {s}: loud at the order");
        p.until(3.0, |p| !p.ops_on(d).is_empty());
        let op = p.ops_on(d)[0];
        p.until(40.0, |p| p.ops_on(d).is_empty()).unwrap_or_else(|| panic!("seed {s}: never halted: {:?}", p.op_beats(op)));
        assert!(!forced(&p, op) && p.door_locked_now(d), "seed {s}: the first order halts on the lock: {:?}", p.op_beats(op));
        assert_eq!(p.try_order("bangClear", at, Who::All), None, "seed {s}");
        p.until(3.0, |p| !p.ops_on(d).is_empty());
        let op = p.ops_on(d)[0];
        p.until(40.0, |p| p.op_beats(op).iter().any(|b| b.starts_with("bang"))).unwrap_or_else(|| panic!("seed {s}: no can: {:?}", p.op_beats(op)));
        assert!(forced(&p, op) && p.keep_broken(d), "seed {s}: the boot, then the can: {:?}", p.op_beats(op));
        drop(p);

        let (sc, at) = yard(s);
        let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
        p.sam_unhurt();
        p.run(0.2);
        let keep = p.defender_at([24.25, 12.25]);
        p.drop_all_but(&[keep]);
        p.pin_man(keep, [24.25, 12.25], 0.0, GuardState::Patrol, 0.0);
        let d = p.door("west-yard");
        go_silent(&mut p);
        assert_eq!(p.try_order("thresholdHold", at, Who::All), None, "seed {s}");
        p.until(3.0, |p| !p.ops_on(d).is_empty());
        let op = p.ops_on(d)[0];
        p.until(40.0, |p| p.op_phase(op) == Phase::Thr).unwrap_or_else(|| panic!("seed {s}: never held: {:?}", p.op_beats(op)));
        p.run(1.0);
        assert!(p.silent() && !p.op_beats(op).iter().any(|b| b.starts_with("bang")), "seed {s}: the control: {:?}", p.op_beats(op));
        assert_eq!(p.try_order("bangClear", at, Who::All), None, "seed {s}");
        assert!(!p.silent(), "seed {s}: the row ends silent");
    }
}

/// THE RULE: at a door no pick opens: silent and not yet caught, no boot: "wedged from inside. no quiet way in. go loud and we boot it." and the order ends on the door, still locked and silent.
/// The same door after something loud since the order: "wedged from inside. booting it." once, over the net, the boot, and in, the word still silent and "compromised." not said again for
/// their own boot. A fresh "go silent" starts clean.
/// A FAILURE MEANS: a quiet squad is made to boot, or a caught one is stuck at a door it may as well kick.
#[test]
fn a_wedged_door_is_booted_only_once_the_squad_is_caught() {
    for s in SEEDS {
        let (mut p, d, at) = at_a_locked_door(s);
        p.wedged_from(d, [at[0], at[1] + 1.0]);
        go_silent(&mut p);
        p.order("openClear", at, Who::All);
        p.until(3.0, |p| !p.ops_on(d).is_empty());
        let op = p.ops_on(d)[0];
        p.until(40.0, |p| p.ops_on(d).is_empty()).unwrap_or_else(|| panic!("seed {s}: it never ended: {:?}", p.op_beats(op)));
        assert!(p.op_ended(op).map_or(false, |e| e.contains("silent: no boot")), "seed {s}: ended {:?}", p.op_ended(op));
        assert_eq!(p.said_count("wedged from inside. no quiet way in. go loud and we boot it."), (1, true), "seed {s}");
        assert!(!forced(&p, op) && p.door_locked_now(d) && !p.keep_broken(d) && p.silent() && !p.compromised(), "seed {s}: {:?}", p.op_beats(op));
        drop(p);

        let (mut p, d, at) = at_a_locked_door(s);
        p.wedged_from(d, [at[0], at[1] + 1.0]);
        go_silent(&mut p);
        p.frag_lands([36.0, 24.0]);
        p.until(p.frag_fuse() + 2.0, |p| p.compromised()).unwrap_or_else(|| panic!("seed {s}: a frag 20 m off and nobody heard it"));
        let heard = p.said_count("compromised.").0;
        p.order("openClear", at, Who::All);
        p.until(3.0, |p| !p.ops_on(d).is_empty());
        let op = p.ops_on(d)[0];
        p.until(40.0, |p| !p.across(d, 1.0).is_empty()).unwrap_or_else(|| panic!("seed {s}: nobody went in: {:?}", p.op_beats(op)));
        assert_eq!(p.said_count("wedged from inside. booting it."), (1, true), "seed {s}");
        assert!(forced(&p, op) && p.keep_broken(d), "seed {s}: {:?}", p.op_beats(op));
        assert!(p.silent() && p.compromised(), "seed {s}: the word stands");
        assert_eq!(p.said_count("compromised.").0, heard, "seed {s}: said again at their own boot");
        let sam = p.sam_at();
        p.order("loud", sam, Who::All);
        p.frame();
        go_silent(&mut p);
        p.frame();
        assert!(!p.compromised(), "seed {s}: a fresh order starts clean");
    }
}
