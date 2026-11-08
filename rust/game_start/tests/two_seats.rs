//! A GAME OF TWO: what the world's lines of the tick say about the team is about every player, not the first (tools/blitz/sweeps/seat-tick.py lists them).
use game::game::Foe;
use game::ids::GuardId;
use game_start::play::{Opts, Play};

const SEEDS: [u32; 2] = [21, 22];
/// a clear run of floor down the office's aisle
const AISLE: f64 = 17.6;

/// The office's room clear for two, the players at each end of the aisle with every man of both squads out of the way behind them, one defender left, held.
fn two(seed: u32) -> (Play, GuardId) {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed, seats: 2, ..Default::default() });
    p.g.god_mode = true;
    p.run(0.3);
    let foes: Vec<GuardId> = p.g.guards.iter().copied().filter(|&gd| p.g.gd[gd].team != game::teams::Team::Operative).collect();
    p.drop_all_but(&foes[..1]);
    (p, foes[0])
}
fn man(p: &Play, seat: usize) -> game::ids::PlayerId { game::seats::seat(&p.g, p.g.seats[seat]).player }

/// THE RULE: a round in flight stops at either player. A failure means it flies through one of them.
#[test]
fn a_round_in_flight_stops_at_either_player() {
    for s in SEEDS {
        for seat in 0..2 {
            for shell in [false, true] {
                let (mut p, t1) = two(s);
                let (x_man, x_from) = (14.0, 21.0);
                p.seat_stands(seat, [x_man, AISLE]);
                p.seat_stands(1 - seat, [3.0, 3.0]);
                for m in p.mates() { p.man_stands(m, [3.5, 3.5]); }
                p.pin_man(t1, [x_from, AISLE], -std::f64::consts::FRAC_PI_2, game::game::GuardState::Patrol, 0.0);
                p.frame();
                let shooter = p.g.gd[t1].ch;
                let (from, to) = ([x_from - 0.5, 1.3, AISLE], [x_man, 1.2, AISLE]);
                if shell { game::he40::fire_shell(&mut p.g, shooter, from, to); } else { game::launcher::fire_ring(&mut p.g, shooter, from, to); }
                let mut least = f64::INFINITY;
                for _ in 0..40 {
                    p.pin_man(t1, [x_from, AISLE], -std::f64::consts::FRAC_PI_2, game::game::GuardState::Patrol, 0.0);
                    p.frame();
                    if let Some(r) = p.g.rings.first() { least = least.min(r.pos[0]); }
                    if let Some(r) = p.g.shells.first() { least = least.min(r.pos[0]); }
                }
                assert!(least > x_man - 1.0, "seed {s}: the {} from {x_from} went on to x {least} past seat {seat}'s man at {x_man}", if shell { "shell" } else { "ring" });
                assert!(least < x_man + 2.0, "seed {s}: CONTROL: the {} stopped short of him at x {least}", if shell { "shell" } else { "ring" });
            }
        }
    }
}
