//! A HOLE AGREES FOR FEET, EYES AND ROUNDS. CHEWED COVER STOPS HIDING. A RESTART PUTS IT BACK. ASKING FIRST SAYS WHAT IT WILL DO.
mod world_ask;
use game::ask::Over;
use game::scene_collision::{nav_grid_is_blocked, raycast, shot_blocked, shot_blocked_over, segment_blocked, segment_blocked_over, solid_blocked, solid_blocked_over};
use game_start::play::{nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use world_ask::{a_box_of, scene_with, FurnAsk, WallAsk};

const SEEDS: [u32; 3] = [1, 2, 3];

/// Two rooms with a partition at x 18 and a door well away, a crate on the floor of the west room.
fn rooms(s: u32) -> Play {
    let mut sc = Scene::new("partition");
    sc.room("west", 10.0, 8.0, 18.0, 18.0).room("east", 18.0, 8.0, 26.0, 18.0).door("yard", Axis::Z, 10.0, 15.4);
    sc.sam(2.25, 2.25);
    let mut p = scene_with(&sc, vec![a_box_of("crate", [14.0, 0.8, 12.0], [0.2, 0.8, 0.6], "white", "wood")], &Opts { seed: s, ..Default::default() });
    p.g.god_mode = true; p.g.ai_enabled = false; p.g.tune.chaos = 0.5;
    p.run(0.1);
    p
}
/// what each of the game's questions says of the straight line from one side of the partition to the other, at z: [ray, eye, round, sound, feet, walk grid], true = stopped.
fn line(p: &mut Play, z: f64) -> [bool; 6] {
    let (a, b) = ([16.8, 1.3, z], [19.2, 1.3, z]);
    let (eye_a, eye_b) = ([a[0], 1.6, z], [b[0], 1.6, z]);
    let ray = raycast(&mut p.g, a, [1.0, 0.0, 0.0], 2.4).is_some();
    let walked = p.body_walks([17.4, z], [18.6, z]);
    [ray, segment_blocked(&mut p.g, eye_a, eye_b, None, None, None), shot_blocked(&mut p.g, a, b, None, None), solid_blocked(&mut p.g, eye_a, eye_b, None),
     (walked[0] - 18.6).abs() > 0.05, nav_grid_is_blocked(&p.g, 18.0, z)]
}

/// THE RULE (one model): before the blast the partition is a wall to the round's ray, the eye, the shot, the sound, the feet and the grid; after it, in the hole, all six say open. CONTROL: three metres along the same wall all six still say wall.
/// A FAILURE MEANS: a hole somebody can see through and nobody can walk through, or the reverse: men shot through what they took for a wall.
#[test]
fn a_hole_agrees_for_feet_eyes_and_rounds() {
    for s in SEEDS {
        let z = 11.0 + 0.5 * nudge(s, 0);
        let mut p = rooms(s);
        assert_eq!(line(&mut p, z), [true; 6], "seed {s}: before: not a wall to all of them");
        assert!(p.wall_blast([17.7, 0.3, z], None) > 0.0, "seed {s}: the blast opened nothing");
        p.run(0.1);
        assert_eq!(line(&mut p, z), [false; 6], "seed {s}: the questions disagree about the hole [ray, eye, round, sound, feet, grid]");
        assert_eq!(line(&mut p, z + 3.0), [true; 6], "seed {s}: three metres along: not a wall any more");
    }
}

fn gone(p: &Play) -> bool { game::wallbreak::wall_break_all(&p.g).iter().any(|w| w.kind == game::wallbreak::BKind::Cover && w.open >= w.cells) }
/// THE RULE: a crate that rounds have chewed until nothing stands stops the ray and hides nobody any more, and its floor is floor. CONTROL: the first rounds alone leave it a crate.
/// A FAILURE MEANS: a man goes on hiding behind cover that has gone, or walks into it.
#[test]
fn chewed_cover_stops_hiding() {
    for s in [1u32, 2] {
        let mut p = rooms(s);
        let (a, b) = ([11.9, 0.7, 12.0], [16.1, 0.7, 12.0]);
        assert!(segment_blocked(&mut p.g, a, b, None, None, None) && nav_grid_is_blocked(&p.g, 14.0, 12.0), "seed {s}: the crate hides nothing to start with");
        p.sam_put([11.0, 12.0]);
        p.sam_fires_at([16.5, 12.0], 0.5);
        assert!(segment_blocked(&mut p.g, a, b, None, None, None), "seed {s}: one round took the whole crate");
        let mut n = 1;
        while !gone(&p) && n < 300 { p.sam_fires_at([16.5, 12.0 + 0.7 * ((n % 2) as f64 - 0.5)], 0.0); n += 1; }
        assert!(gone(&p) && !segment_blocked(&mut p.g, a, b, None, None, None), "seed {s}: {n} squeezes and the crate is not gone, or it still hides");
        p.run(0.2);
        assert!(!nav_grid_is_blocked(&p.g, 14.0, 12.0), "seed {s}: gone to the eye, and still a block to the feet");
    }
}

/// THE RULE: a restart puts the wrecked level back: the same statics, every flag word, the grid, and the wall a wall to all six. CONTROL: the wreck did change those.
/// A FAILURE MEANS: a restart plays on the last try's ruins.
#[test]
fn a_restart_puts_the_walls_back() {
    for s in [1u32, 2] {
        let z = 11.0 + 0.5 * nudge(s, 0);
        let mut p = rooms(s);
        let before = p.level_as_it_stands();
        p.wall_blast([17.7, 0.3, z], None);
        p.run(0.1);
        assert_ne!(p.level_as_it_stands(), before, "seed {s}: the wreck changed nothing");
        game::savestate::control::restart(&mut p.g, None);
        p.run(0.5);
        assert_eq!(p.level_as_it_stands(), before, "seed {s}: the restart left the wreck");
        assert_eq!(line(&mut p, z), [true; 6], "seed {s}: the wall is not a wall again");
    }
}

/// THE RULE: asking which walls a blast here would open changes nothing, and what it says is what the blast does: the wall it lists gains the cells it said, and a line through the place reads the same in the forecast as after. CONTROL: outside the forecast the line is the wall again.
/// A FAILURE MEANS: men plan a charge on a forecast the charge does not keep, or the asking itself moves the game.
#[test]
fn asking_first_says_what_the_blast_will_do() {
    for s in SEEDS {
        let z = 11.0 + 0.5 * nudge(s, 0);
        let mut p = rooms(s);
        let (a, b, at) = ([16.8, 1.6, z], [19.2, 1.6, z], [17.7, 0.3, z]);
        let as_was = p.level_as_it_stands();
        let open0: Vec<_> = game::wallbreak::wall_break_all(&p.g).iter().map(|w| (w.id, w.open)).collect();
        let said = game::destruction::destruction_walls_for(&mut p.g, at, None);
        assert_eq!(said.len(), 1, "seed {s}: the forecast lists {} walls", said.len());
        assert!(said[0].hole && said[0].cells > 0.0, "seed {s}: the forecast has no hole in it");
        assert_eq!(p.level_as_it_stands(), as_was, "seed {s}: asking moved the level");
        let over = Over::OWN.walls_open(&p.g, &said);
        let seen = (segment_blocked_over(&mut p.g, &over, a, b, None, None), shot_blocked_over(&mut p.g, &over, a, b, None, None), solid_blocked_over(&mut p.g, &over, a, b, None));
        assert_eq!(seen, (false, false, false), "seed {s}: the forecast's line is shut");
        assert_eq!(line(&mut p, z)[1..3], [true, true], "seed {s}: out of the forecast the wall is not a wall");
        p.wall_blast(at, None);
        p.run(0.05);
        let open1: Vec<_> = game::wallbreak::wall_break_all(&p.g).iter().map(|w| (w.id, w.open)).collect();
        let grew: Vec<_> = open1.iter().zip(&open0).filter(|(n, o)| n.1 > o.1).map(|(n, o)| (n.0, n.1 - o.1)).collect();
        assert_eq!(grew, vec![(said[0].id, said[0].cells)], "seed {s}: the blast did not do what the forecast said");
        let after = (segment_blocked(&mut p.g, a, b, None, None, None), shot_blocked(&mut p.g, a, b, None, None), solid_blocked(&mut p.g, a, b, None));
        assert_eq!(after, seen, "seed {s}: after the blast the line reads otherwise");
    }
}
