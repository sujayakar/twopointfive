//! A MAN IN SAM'S ARM: a man who finds it calls it in, comes to a ring, talks, goes round and fires nothing while the shield is up.
mod doors_ask;
use doors_ask::*;
use game::game::GuardState;
use game::ids::*;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};

/// a lit hall with one man Sam is behind and two more parked out of it behind a shut door. Who is in the arm, the two, and where the pair stands.
fn the_hall(s: u32) -> (Play, GuardId, GuardId, GuardId, [f64; 2]) {
    let at = [14.0 + nudge(s, 90), 15.0 + nudge(s, 91)];
    let mut sc = Scene::new("hall");
    sc.room("hall", 6.0, 6.0, 34.0, 24.0).room("back", 6.0, 24.0, 34.0, 28.0).door("back-door", Axis::X, 24.0, 28.0);
    sc.bulb(14.0, 15.0).bulb(24.0, 15.0).bulb(8.0, 26.0).sam(at[0] - 0.8, at[1]);
    sc.sentry("held", at, [at[0] + 6.0, at[1]]).sentry("a", [9.0, 26.0], [9.0, 28.0]).sentry("b", [12.0, 26.0], [12.0, 28.0]);
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.run(0.1);
    let (held, a, b) = (p.guard("held"), p.guard("a"), p.guard("b"));
    (p, held, a, b, at)
}
fn stood_facing(p: &mut Play, m: GuardId, at: [f64; 2], on: [f64; 2]) { p.pin_man(m, at, yaw_to(at, on), GuardState::Patrol, 0.0); p.frame(); p.let_go(m); }

/// THE RULE: a man walks in on Sam holding one of his: he sees it at once, calls "he's got him, hold your fire" over the net once, the floor goes to lockdown the frame he sees it, and for 15 s he stays in the ring
/// (never nearer than 3.2 m, no farther than 7 once he has come over), talks to Sam and fires nothing. The control is the same man at the same place and just as awake with nobody in Sam's arm: he fires.
/// A FAILURE MEANS: the shield does not stop a round, or the man has nothing to do about it.
#[test]
fn a_man_who_finds_sam_with_one_of_his_calls_it_and_holds_his_fire() {
    for s in SEEDS {
        let (mut p, held, a, _, at) = the_hall(s);
        assert!(p.sam_holds(held), "seed {s}: the grab never settled");
        assert_eq!(p.escalation(), 0.0, "seed {s}: the staging: nobody has seen it");
        let from = [at[0] + 12.0, at[1] + nudge(s, 92)];
        stood_facing(&mut p, a, from, at);
        let entered = p.until(2.0, |p| { p.point_at(p.at(a)); p.in_standoff(a).is_some() }).unwrap_or_else(|| panic!("seed {s}: he never saw it: {}", p.why(a)));
        assert_eq!(p.saw_held(a), Some(p.g.gd[held].callsign.clone()), "seed {s}");
        assert_eq!(p.escalation(), 2.0, "seed {s}: the floor goes up when he sees it ({entered:.2} s)");
        let (near, mut far, n0) = (dist(p.at(a), at), 0.0f64, p.lines(a).len());
        let mut nearest = near;
        for f in 0..15 * 60 {
            p.point_at(p.at(a));
            p.frame();
            assert!(p.in_standoff(a).is_some(), "seed {s}: the record dropped with the pair in plain view");
            let d = dist(p.at(a), p.sam_at());
            nearest = nearest.min(d);
            if f > 9 * 60 { far = far.max(d); }
        }
        assert_eq!(p.rounds_fired(a), 0.0, "seed {s}: he fired with his own man square between");
        assert!(nearest >= 3.2 && far > 0.0 && far <= 7.0, "seed {s}: nearest {nearest:.1}, farthest after settling {far:.1}");
        assert_eq!(p.said_count("nobody push him"), (1, true), "seed {s}: the call goes out once, over the net");
        assert!(p.said.iter().filter(|l| l.who == a && !l.radio).count() > 0 && p.lines(a).len() > n0, "seed {s}: he said nothing to Sam");
        assert_eq!(p.state(a), GuardState::Alert, "seed {s}");
        drop(p);

        let (mut p, _, a, _, at) = the_hall(s);
        p.drop_all_but(&[a]);
        let from = [at[0] + 12.0, at[1] + nudge(s, 92)];
        p.pin_man(a, from, yaw_to(from, at), GuardState::Alert, 1.0);
        p.frame();
        p.let_go(a);
        p.until(8.0, |p| p.rounds_fired(a) > 0.0).unwrap_or_else(|| panic!("seed {s}: the control: nobody in the arm and he never fired: {}", p.why(a)));
    }
}

/// THE RULE: Sam's facing kept still with his two free to go round him: 20 s and no round while the man is up in his arm, both men point and talk, and one gets a clean line and holds it. The
/// moment the man is out of the arm the fight has the round back, after a human beat: the first round no sooner than 0.35 s after he folds and no later than 4 s. The fold is the control.
/// A FAILURE MEANS: "when Sam is holding a hostage, enemies should point but not fire" has gone, or they never fire again.
#[test]
fn men_point_and_go_round_but_fire_nothing_until_the_shield_is_gone() {
    for s in SEEDS {
        let (mut p, held, a, b, at) = the_hall(s);
        assert!(p.sam_holds(held), "seed {s}");
        p.point_at([at[0] + 8.0, at[1]]);
        p.run(0.2);
        stood_facing(&mut p, a, [at[0] + 4.4, at[1] + 1.2], at);
        stood_facing(&mut p, b, [at[0] + 4.5, at[1] - 1.6], at);
        p.until(1.5, |p| p.in_standoff(a).is_some() && p.in_standoff(b).is_some()).unwrap_or_else(|| panic!("seed {s}: the pair is in their view and they have not seen it"));
        let (mut line, mut aiming) = (0, 0);
        for _ in 0..20 * 60 {
            p.frame();
            assert!(p.up(held), "seed {s}: the hold ended");
            for m in [a, b] {
                assert!(p.state(m) == GuardState::Alert, "seed {s}: not alert");
                if p.in_standoff(m) == Some(true) { line += 1; if p.arms_shown(m) == game::character::UpperMode::Aim { aiming += 1; } }
            }
        }
        assert_eq!(p.rounds_fired(a) + p.rounds_fired(b), 0.0, "seed {s}: rounds with his man alive in Sam's arm");
        assert!(line >= 2 * 60, "seed {s}: nobody got round to a clean line: {line} frames");
        assert!(aiming * 10 >= line * 9, "seed {s}: a clean line {line} frames, the pistol up on {aiming}");
        assert!(p.said.iter().any(|l| (l.who == a || l.who == b) && !l.radio), "seed {s}: neither said a word");
        p.choke_him();
        let fold = p.until(10.0, |p| !p.up(held)).unwrap_or_else(|| panic!("seed {s}: he never folded"));
        assert_eq!(p.rounds_fired(a) + p.rounds_fired(b), 0.0, "seed {s}: rounds during the choke ({fold:.1} s)");
        let first = p.until(4.0, |p| p.rounds_fired(a) + p.rounds_fired(b) > 0.0).unwrap_or_else(|| panic!("seed {s}: no round 4 s after he folded: {}", p.why(a)));
        assert!(first >= 0.35, "seed {s}: the first round {first:.2} s after: no beat");
    }
}

/// THE RULE: a round of his own side meant for Sam goes into the man in his arm: fired from square in front it takes the man, shot by his own, Sam untouched, and the shooter says so; the shot itself does not raise the floor. The control is
/// the round from dead astern, past the man: it stops in Sam.
/// A FAILURE MEANS: a shield that does not shield, or one that stops a round from behind.
#[test]
fn a_round_of_his_own_side_goes_into_the_man_in_his_arm() {
    for s in SEEDS {
        let (mut p, held, a, _, at) = the_hall(s);
        p.sam_hurtable();
        assert!(p.sam_holds(held), "seed {s}");
        p.point_at([at[0] + 8.0, at[1]]);
        p.run(0.3);
        let front = [p.sam_at()[0] + 4.2, p.sam_at()[1]];
        stood_facing(&mut p, a, front, at);
        p.until(1.5, |p| p.in_standoff(a).is_some()).unwrap_or_else(|| panic!("seed {s}: he never saw it"));
        p.pin_man(a, front, yaw_to(front, at), GuardState::Alert, 1.0);
        p.frame();
        let (was, esc, n0) = (p.sam_down_hits(), p.escalation(), p.lines(a).len());
        let chest = p.sam_at();
        p.round_from(a, [chest[0], 1.25, chest[1]]);
        p.run(3.0 / 60.0);
        assert!(!p.up(held), "seed {s}: the round did not take him");
        assert_eq!(p.down_how(held), Some((game::game::DownKind::Shot, game::game::DownBy::Guard)), "seed {s}");
        assert_eq!(p.sam_down_hits(), was, "seed {s}: Sam took it");
        assert_eq!(p.escalation(), esc, "seed {s}: the shot itself moved the alarm");
        assert!(p.lines(a).len() > n0, "seed {s}: the shooter says nothing");
        drop(p);

        let (mut p, held, a, _, at) = the_hall(s);
        p.sam_hurtable();
        assert!(p.sam_holds(held), "seed {s}");
        p.point_at([at[0] + 8.0, at[1]]);
        p.run(0.3);
        let was = p.sam_down_hits();
        let behind = [p.sam_at()[0] - 4.2, p.sam_at()[1]];
        p.pin_man(a, behind, yaw_to(behind, at), GuardState::Alert, 1.0);
        p.frame();
        let chest = p.sam_at();
        p.round_from(a, [chest[0], 1.25, chest[1]]);
        p.run(3.0 / 60.0);
        assert!(p.up(held), "seed {s}: from astern it took the man in front");
        assert_ne!(p.sam_down_hits(), was, "seed {s}: from astern it should stop in Sam");
    }
}

/// THE RULE: a man finds no shield in the dark: the same pair with the lights off, the same man looking: no standoff forms, and no round is fired at a pair nobody can see. The control is the lit hall above.
/// A FAILURE MEANS: a guard sees through the dark because a hold is on, and the picture says otherwise.
#[test]
fn nobody_stands_off_a_pair_he_cannot_see() {
    for s in SEEDS {
        let at = [14.0 + nudge(s, 90), 15.0 + nudge(s, 91)];
        let mut sc = Scene::new("hall");
        sc.room("hall", 6.0, 6.0, 34.0, 24.0).room("back", 6.0, 24.0, 34.0, 28.0).door("back-door", Axis::X, 24.0, 28.0);
        sc.bulb_off(14.0, 15.0).bulb_off(24.0, 15.0).sam(at[0] - 0.8, at[1]);
        sc.sentry("held", at, [at[0] + 6.0, at[1]]).sentry("a", [9.0, 26.0], [9.0, 28.0]);
        let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
        p.sam_unhurt();
        p.run(0.1);
        let (held, a) = (p.guard("held"), p.guard("a"));
        assert!(p.sam_holds(held), "seed {s}");
        stood_facing(&mut p, a, [at[0] + 12.0, at[1]], at);
        for _ in 0..6 * 60 { p.frame(); assert!(p.in_standoff(a).is_none(), "seed {s}: a standoff round a pair in the dark"); }
        assert_eq!(p.rounds_fired(a), 0.0, "seed {s}");
    }
}
