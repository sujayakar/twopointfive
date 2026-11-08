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

/// THE RULE: the team is down when all of it is: one man down and the other up, the floor goes on as if they are up. A rule about one man asks that man.
/// A failure means the first player's fall stops the alarm's rise and the guards' dozing for both.
#[test]
fn the_team_is_down_when_all_of_it_is() {
    for s in SEEDS {
        for seat in 0..2 {
            let (mut p, t1) = two(s);
            p.pin_man(t1, [21.0, AISLE], 0.0, game::game::GuardState::Patrol, 0.0);
            assert!(game::doze::live(&mut p.g), "seed {s}: CONTROL: both up, a calm floor dozes");
            let m = man(&p, seat);
            p.g.pl[m].down = true;
            assert!(game::doze::live(&mut p.g), "seed {s}: seat {seat}'s man down and the other up: the floor is still calm");
            p.g.gd[t1].team = game::teams::Team::Guard;   // (the floor's alarm is the guards': a level's night, not this room clear's defenders)
            p.g.alarm.episode = true;
            let was = p.g.escalation;
            p.pin_man(t1, [21.0, AISLE], 0.0, game::game::GuardState::Patrol, 0.0);
            p.frame();
            assert!(p.g.escalation > was, "seed {s}: an alarm over with seat {seat}'s man down and the other up raises the floor: {} from {was}", p.g.escalation);
            drop(p);
            let (mut q, t2) = two(s);
            let (a, b) = (man(&q, 0), man(&q, 1));
            q.g.pl[a].down = true;
            q.g.pl[b].down = true;
            assert!(!game::doze::live(&mut q.g), "seed {s}: CONTROL: both down, nobody is left to doze for");
            q.g.gd[t2].team = game::teams::Team::Guard;
            q.g.alarm.episode = true;
            let was = q.g.escalation;
            q.pin_man(t2, [21.0, AISLE], 0.0, game::game::GuardState::Patrol, 0.0);
            game::guards::update_escalation(&mut q.g, 1.0 / 60.0);
            assert_eq!(q.g.escalation, was, "seed {s}: CONTROL: both down, an alarm over raises nothing");
        }
    }
}

fn close(p: &Play, gd: GuardId) -> bool { game::lanes::close_fight(&p.g, gd) }

/// THE RULE: a defender's foe, when it is a player, is the player he faces.
/// A failure means he is measured against the first player wherever the second stands.
#[test]
fn a_defenders_foe_is_the_player_he_faces() {
    for s in SEEDS {
        for seat in 0..2 {
            let (mut p, t1) = two(s);
            p.seat_stands(seat, [20.0, AISLE]);
            p.seat_stands(1 - seat, [3.0, 3.0]);
            p.pin_man(t1, [21.0, AISLE], 0.0, game::game::GuardState::Alert, 1.0);
            p.g.gd[t1].foe = Some(Foe::Player);
            p.g.gd[t1].faces = Some(seat as f64);
            p.g.gd[t1].last_seen_t = p.g.time;
            assert!(close(&p, t1), "seed {s}: he has seat {seat}'s man a metre off and calls it no close fight");
        }
    }
}
