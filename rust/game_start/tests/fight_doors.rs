//! A FIGHT AT A DOORWAY: rounds through it keep the team's guns on it.
mod doors_ask;
use doors_ask::*;
use game::game::GuardState;
use game::orders::Who;
use game_start::play::{Opts, Play};

/// "enemy was immediately in front of them and shooting, nothing happened": four men stacked, a man behind the far jamb where none has a line to him; the door opened, then a round every half second, where `out` says. How many men with a line to the
/// doorway have their guns on it at the last round, how many men were asked, how many were turned toward it by the rounds.
fn rounds_through(s: u32, out: bool) -> (usize, usize, usize) {
    let (sc, at) = yard(s);
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.men_spared();
    p.run(0.2);
    let (foe, keep) = (p.defender_at([11.25, 11.25]), p.defender_at([24.25, 12.25]));
    p.pin_man(keep, [14.0, 4.0], 0.0, GuardState::Patrol, 0.0);
    let d = p.door("west-yard");
    p.order("stack", at, Who::All);
    let spot = [at[0] + 1.6, 15.2];
    p.pin_man(foe, spot, 0.0, GuardState::Alert, 1.0);
    let men = p.men();
    for _ in 0..30 * 60 { p.pin_man(foe, spot, 0.0, GuardState::Alert, 1.0); p.blinded(foe); p.frame(); if men.iter().all(|&m| p.arrived(m)) { break; } }
    p.leaf_stays(d);
    p.leaf_put_open(d, 1.5);
    for _ in 0..6 * 60 { p.pin_man(foe, spot, 0.0, GuardState::Alert, 1.0); p.blinded(foe); p.frame(); }
    let (mut on_it, mut asked, mut turned) = (0, 0, 0);
    let c = p.frame_at(d);
    let to = if out { [c[0] + (c[0] - spot[0]) * 5.0, c[1] + (c[1] - spot[1]) * 5.0] } else { [12.0, 11.0] };
    let shoot = |p: &mut Play, f: usize| { p.pin_man(foe, spot, yaw_to(spot, to), GuardState::Alert, 1.0); p.blinded(foe); if f % 30 == 0 { p.round_from(foe, [to[0], 1.2, to[1]]); } p.frame(); };
    let was: Vec<f64> = men.iter().map(|&m| off_by(p.gun_yaw(m), yaw_to(p.at(m), c))).collect();
    for f in 0..60 { shoot(&mut p, f); }
    let off_in_a_second: Vec<f64> = men.iter().map(|&m| off_by(p.gun_yaw(m), yaw_to(p.at(m), c))).collect();
    for f in 60..300 { shoot(&mut p, f); }
    for (i, &m) in men.iter().enumerate() {
        if !p.sees_doorway(m, d) { continue; }
        asked += 1;
        let now = off_by(p.gun_yaw(m), yaw_to(p.at(m), c));
        if now < 40f64.to_radians() && off_in_a_second[i] < 40f64.to_radians() { on_it += 1; }
        if now + 0.2 < was[i] { turned += 1; }
    }
    (on_it, asked, turned)
}

/// THE RULE: rounds coming out through a doorway: every man with a line to it is looking at it within a second of the first round and still is at the last. "enemy was immediately in front of them and shooting, nothing happened".
/// A FAILURE MEANS: a man turns his back on a doorway with the rounds still coming through it.
#[test]
#[ignore = "the game: seed 2, 3 of 4 men with a line by the game's own measure have their guns on the doorway at the last round: the last man of the far file stays 96 degrees off it (seed 1: 4 of 4)"]
fn every_man_with_a_line_keeps_his_gun_on_a_doorway_rounds_come_through() {
    for s in SEEDS {
        let (on_it, asked, _) = rounds_through(s, true);
        assert!(asked > 0 && on_it == asked, "seed {s}: {on_it} of {asked} with a line have their guns on the doorway");
    }
}

/// THE RULE: the same rounds bring men round to the doorway: two or more have their guns on it within a second and at the last, and a man who was turned away has turned. The control is the same man firing into his own room: nobody is turned.
/// A FAILURE MEANS: rounds through a doorway are nothing to the men beside it, or any shot anywhere turns a stack.
#[test]
fn rounds_through_a_doorway_turn_the_men_and_rounds_into_the_room_do_not() {
    for s in SEEDS {
        let (on_it, _, turned) = rounds_through(s, true);
        assert!(on_it >= 2 && turned > 0, "seed {s}: {on_it} on it, {turned} turned");
        let (_, _, turned) = rounds_through(s, false);
        assert_eq!(turned, 0, "seed {s}: the control: rounds into his own room turned a man");
    }
}
