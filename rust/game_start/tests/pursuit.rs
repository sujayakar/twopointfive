//! MEN ON A FIX. Two on one fix come from two sides; a doorway somebody fell in is priced.
mod stealth_ask;
use game::game::GuardState;
use game::ids::GuardId;
use game::tactics::{GatePhase, PlanPhase, RoleKind};
use game::teams::Team;
use game_start::play::{dist, Opts, Play};
use stealth_ask::StealthAsk;

const SEEDS: [u32; 3] = [1, 2, 3];
const EAST: f64 = std::f64::consts::FRAC_PI_2;
const FIX: [f64; 2] = [24.0, 17.6];

/// The office, Sam a ghost, nobody on the floor but the men this test frees.
fn office(seed: u32) -> (Play, [GuardId; 3]) {
    let mut p = Play::level("office.json", false, &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.run(0.1);
    p.sam_put([26.5, 20.5]);
    p.g.player_invisible = true;
    let m = [p.g.guards[0], p.g.guards[1], p.g.guards[2]];
    (p, m)
}
fn apart_deg(a: [f64; 2], b: [f64; 2], about: [f64; 2]) -> f64 {
    let (x, y) = ((a[0] - about[0]).atan2(a[1] - about[1]), (b[0] - about[0]).atan2(b[1] - about[1]));
    let d = (x - y).abs() % std::f64::consts::TAU;
    d.min(std::f64::consts::TAU - d).to_degrees()
}
fn in_the_corridor(p: &mut Play, man: GuardId, at: [f64; 2]) { p.pin_man(man, at, EAST, GuardState::Alert, 1.0); p.alert_on_fix(man, FIX); }

/// THE RULE: two men alert on one fix with two ways round it split: a presser and a flanker, and they come into the last four metres at least ninety degrees apart about the fix, and the presser holds before the push.
/// CONTROL: the same man alone on the same fix has no flanker.
/// A FAILURE MEANS: men in a pair walk up the same aisle in single file.
#[test]
fn two_on_one_fix_come_from_two_sides() {
    for s in SEEDS {
        let (mut p, [a, b, c]) = office(s);
        p.parked(c);
        p.man_stands(c, [37.5, 9.0]);
        in_the_corridor(&mut p, a, [12.0, 11.2]);
        in_the_corridor(&mut p, b, [10.5, 11.4]);
        let (mut enter_a, mut enter_b): (Option<[f64; 2]>, Option<[f64; 2]>) = (None, None);
        let mut roles = Vec::new();
        let mut pushed = false;
        p.until(30.0, |p| {
            if let Some((men, phase)) = p.pursuit().into_iter().next() { if roles.is_empty() { roles = men.iter().map(|m| m.1).collect(); } pushed |= phase == PlanPhase::Push; }
            if enter_a.is_none() && dist(p.at(a), FIX) <= 4.0 { enter_a = Some(p.at(a)); }
            if enter_b.is_none() && dist(p.at(b), FIX) <= 4.0 { enter_b = Some(p.at(b)); }
            enter_a.is_some() && enter_b.is_some()
        }).unwrap_or_else(|| panic!("seed {s}: thirty seconds and they are not both at the fix: {} / {}", p.why(a), p.why(b)));
        assert!(roles.contains(&RoleKind::Presser) && roles.contains(&RoleKind::Flanker), "seed {s}: the roles dealt: {roles:?}");
        let apart = apart_deg(enter_a.unwrap(), enter_b.unwrap(), FIX);
        assert!(apart >= 90.0, "seed {s}: they came in {apart:.0} degrees apart");
        assert!(pushed, "seed {s}: the plan never reached its push");
        drop(p);

        let (mut p, [a, b, c]) = office(s);
        for (i, o) in [b, c].into_iter().enumerate() { p.parked(o); p.man_stands(o, [37.5, 9.0 + i as f64 * 1.5]); }
        in_the_corridor(&mut p, a, [12.0, 11.2]);
        p.run(5.0);
        assert!(p.pursuit().iter().all(|(men, _)| men.iter().all(|m| m.1 != RoleKind::Flanker)), "seed {s}: CONTROL: alone and yet a flanker");
    }
}

/// The door staging: `c` struck down in the storage doorway with `a` four metres up the corridor, looking at him or away.
fn struck_down(p: &mut Play, [a, b, c]: [GuardId; 3], looking: bool) {
    p.parked(b);
    p.man_stands(b, [37.5, 9.0]);
    p.sam_put([6.0, 22.0]);
    p.pin_man(c, [32.7, 10.45], std::f64::consts::PI, GuardState::Patrol, 0.0);
    p.let_go(c);
    p.pin_man(a, [28.5, 11.2], if looking { EAST } else { -EAST }, GuardState::Patrol, 0.0);
    p.let_go(a);
    p.frame();
    p.falls(c, game::game::DownBy::Player);
}

/// THE RULE: a doorway a man fell in under a mate's eyes is priced at once: that mate, sent through it, stacks on the jamb, waits, slices and crosses, in that order, and stands in no gap before his drill.
/// CONTROL: with his back to it nobody has priced the door.
/// A FAILURE MEANS: men stroll one after another through the doorway their friend just died in.
#[test]
fn a_doorway_a_man_fell_in_is_priced() {
    for s in SEEDS {
        let (mut p, men) = office(s);
        struck_down(&mut p, men, true);
        assert!(p.door_dangerous("storage", Team::Guard), "seed {s}: the door was not priced the frame he dropped, under his mate's eyes");
        let a = men[0];
        p.alert_on_fix(a, [33.5, 6.5]);
        let mut seen: Vec<GatePhase> = Vec::new();
        p.until(26.0, |p| {
            if let Some(ph) = p.at_the_gate(a) { if seen.last() != Some(&ph) { seen.push(ph); } }
            seen.contains(&GatePhase::Cross)
        }).unwrap_or_else(|| panic!("seed {s}: he never crossed: {seen:?} {}", p.why(a)));
        let order = [GatePhase::Stack, GatePhase::Wait, GatePhase::Slice, GatePhase::Cross];
        let got: Vec<_> = seen.iter().copied().filter(|x| order.contains(x)).collect();
        assert_eq!(got, order, "seed {s}: the drill's order");
        drop(p);

        let (mut p, men) = office(s);
        struck_down(&mut p, men, false);
        assert!(!p.door_dangerous("storage", Team::Guard), "seed {s}: CONTROL: nobody saw him fall and the door is priced");
    }
}
