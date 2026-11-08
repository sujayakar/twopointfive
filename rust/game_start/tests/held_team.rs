//! THE TEAM'S LEGS: they walk at the player's pace, slow for him, and run only for a reason. "teammates never sprint".
mod kit_ask;
use game::orders::Who;
use game_start::play::{dist, Opts, Play};
use game_start::scene::{Axis, Scene};
use kit_ask::KitAsk;

/// A long moonlit yard south of two rooms that hold a defender each, out of the way behind shut doors; the team at its west end.
fn long_yard(s: u32) -> Play {
    let mut sc = Scene::new("long-yard");
    sc.room("west", 10.0, 2.0, 18.0, 8.0).room("east", 18.0, 2.0, 26.0, 8.0);
    sc.door("west-yard", Axis::X, 8.0, 13.4).door("east-yard", Axis::X, 8.0, 21.4).door("between", Axis::Z, 18.0, 4.4);
    sc.moon();
    sc.team([2.5, 16.0], std::f64::consts::FRAC_PI_2, &[[4.0, 15.0], [4.0, 17.0], [3.0, 15.0], [3.0, 17.0]]);
    sc.defender("west", [12.25, 4.25], "west-yard").defender("east", [24.25, 4.25], "east-yard");
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.sam_a_ghost();
    p.run(0.2);
    p
}

/// A leg of about 30 m: the fastest any man's legs went, against what his own card lets him, with `each` asked every frame; and whether any man broke into a run.
fn ordered_leg(p: &mut Play, mut each: impl FnMut(&mut Play)) -> (f64, f64, bool) {
    let men = p.mates();
    p.order("move", [34.0, 16.0], Who::All);
    let (mut top, mut over, mut ran) = (0.0f64, 0.0f64, false);
    for _ in 0..(12 * 60) {
        p.frame();
        each(p);
        for &m in &men {
            let (v, cap) = (p.speed(m), p.walks().0 * p.card_pace(m) + 0.03);
            top = top.max(v);
            over = over.max(v - cap);
            ran |= p.at_a_run(m).is_some();
        }
    }
    (top, over, ran)
}

/// THE RULE: "teammates never sprint": on an ordered walk no man's legs go over the player's own walk times his card, they do walk at it (not the old flat share of it), and nobody breaks into a run.
/// A FAILURE MEANS: the squad races ahead of a walking player, or plods at a fixed four fifths of his pace with him standing still.
#[test]
fn an_ordered_walk_is_the_players_own_walk_and_no_run() {
    for s in [1, 2, 3] {
        let mut p = long_yard(s);
        let (top, over, ran) = ordered_leg(&mut p, |_| {});
        let (own, flat) = p.walks();
        assert!(over <= 0.0, "seed {s}: a man's legs went {over:.2} m/s over his card (top {top:.2}, the walk {own:.2})");
        assert!(top > flat + 0.1, "seed {s}: with Sam stood they walk at the flat share: top {top:.2}, flat {flat:.2}, his walk {own:.2}");
        assert!(!ran, "seed {s}: a man broke into a run on an ordered walk");
    }
}

/// THE RULE: "if you're walking with the squad but behind them, they slow down to let you catch up": with Sam a few metres behind the lead and walking after him, the lead goes no faster than the flat share. Left standing (the control) the lead walks at Sam's own pace, faster than that.
/// A FAILURE MEANS: the squad runs off ahead of the player it is following orders from.
#[test]
fn they_slow_down_for_a_player_walking_behind_them() {
    for s in [1, 2, 3] {
        let mut p = long_yard(s);
        let men = p.mates();
        p.order("move", [34.0, 16.0], Who::All);
        p.run(4.0);
        let lead = *men.iter().max_by(|a, b| p.at(**a)[0].partial_cmp(&p.at(**b)[0]).unwrap()).unwrap();
        let (mut top, mut near_frames) = (0.0f64, 0);
        let flat = p.walks().1;
        for _ in 0..(5 * 60) {
            let at = p.at(lead);
            let sam = p.sam_at();
            if (at[0] - sam[0]) > 5.0 { p.sam_put([at[0] - 3.0, sam[1]]); }
            p.sam_walks_to([at[0], at[1]], false);
            p.frame();
            let (d, on_mark) = (dist(p.sam_at(), p.at(lead)), p.on_his_mark(lead));
            if d < 6.0 && !on_mark && p.frames % 2 == 0 { near_frames += 1; if near_frames > 40 { top = top.max(p.speed(lead)); } }
        }
        assert!(near_frames > 60, "seed {s}: the staging: Sam was seldom near him ({near_frames} frames)");
        assert!(top <= flat + 0.06, "seed {s}: Sam walking after him and his legs went {top:.2} m/s, the flat share {flat:.2}");
        drop(p);

        let mut p = long_yard(s);
        let (top, _, _) = ordered_leg(&mut p, |_| {});
        assert!(top > flat + 0.1, "seed {s}: the control: left standing he walks at only {top:.2}");
    }
}

/// THE RULE: they run only for a reason: fallen in on a Sam who sprints, every follower is at a run with him; a second and a half after he stops they are down to a walk. On a Sam who walks (the control) nobody runs.
/// A FAILURE MEANS: followers trail a sprinting player by half the floor, or sprint with him standing still.
#[test]
fn they_run_with_a_sprinting_player_and_not_with_a_walking_one() {
    for s in [1, 2, 3] {
        let mut p = long_yard(s);
        let men = p.mates();
        let at = p.sam_at();
        p.order("fallIn", at, Who::All);
        p.camera_square();
        p.run(3.0);
        p.key_down("KeyD");
        let (walk, closing) = (p.walks().0, p.closing_pace());
        for _ in 0..(3 * 60) { p.frame(); for &m in &men { assert!(p.speed(m) <= closing + 0.05, "seed {s}: the control: Sam at a walk and a man at {:.2} m/s", p.speed(m)); } }
        p.key_down("ShiftLeft");
        let (mut ran, mut fast) = (0, 0.0f64);
        for _ in 0..(3 * 60) { p.frame(); fast = fast.max(p.sam_speed()); if men.iter().any(|&m| p.speed(m) > closing + 0.2) { ran += 1; } }
        assert!(fast > walk * 1.5, "seed {s}: the staging: Sam's sprint was only {fast:.2} m/s");
        assert!(ran > 60, "seed {s}: Sam sprinting and nobody ran with him ({ran} frames)");
        p.key_up("ShiftLeft");
        p.key_up("KeyD");
        p.run(1.5);
        for &m in &men { assert!(p.speed(m) <= closing + 0.05, "seed {s}: 1.5 s after he stopped a man is at {:.2} m/s", p.speed(m)); }
    }
}
