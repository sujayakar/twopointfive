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
    for _ in 0..30 * 60 { p.pin_man(foe, spot, 0.0, GuardState::Alert, 1.0); p.blind_now(foe); p.frame(); if men.iter().all(|&m| p.arrived(m)) { break; } }
    p.leaf_stays(d);
    p.leaf_put_open(d, 1.5);
    for _ in 0..14 * 60 { p.pin_man(foe, spot, 0.0, GuardState::Alert, 1.0); p.blind_now(foe); p.frame(); }
    let (mut on_it, mut asked, mut turned) = (0, 0, 0);
    let c = p.frame_at(d);
    let to = if out { [c[0] + (c[0] - spot[0]) * 5.0, c[1] + (c[1] - spot[1]) * 5.0] } else { [12.0, 11.0] };
    let shoot = |p: &mut Play, f: usize| { p.pin_man(foe, spot, yaw_to(spot, to), GuardState::Alert, 1.0); p.blind_now(foe); if f % 30 == 0 { p.round_from(foe, [to[0], 1.2, to[1]]); } p.frame(); };
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

/// THE RULE: rounds coming out through a doorway: every man with a line to it is looking at it within a second of the first round and still is at the last, and a man who was turned away has turned. "enemy was immediately in front of them and
/// shooting, nothing happened". The control is the same man firing into his own room: fewer men turn. (Not all of them stay: one on one seed turns to the door by himself.)
/// A FAILURE MEANS: a man turns his back on a doorway with the rounds still coming through it, or any shot anywhere turns a stack.
#[test]
fn rounds_through_a_doorway_keep_the_teams_guns_on_it() {
    for s in SEEDS {
        let (on_it, asked, turned) = rounds_through(s, true);
        assert!(asked >= 2 && on_it == asked && turned > 0, "seed {s}: {on_it} of {asked} with a line have their guns on the doorway, {turned} turned");
        let (_, _, into_the_room) = rounds_through(s, false);
        assert!(into_the_room < turned, "seed {s}: the control: rounds into his own room turned {into_the_room}, through the doorway {turned}");
    }
}
