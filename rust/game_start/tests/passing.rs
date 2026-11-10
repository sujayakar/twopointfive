//! MEN ON THE MOVE KEEP A GUN ON AN OPEN WAY THEY PASS (rust/game_rules/src/teammates/cover/): a doorway, a gap, a hole just blown. Nobody tells them to.
use game::orders::Who;
use game_start::play::{dist, Opts, Play};

const SEEDS: [u32; 3] = [1, 2, 3];
/// the hole, on the bunk room's east wall
const HOLE: [f64; 2] = [34.1, 16.3];

/// The pass: a charge has just opened the bunk room's east wall, two of theirs are stunned inside, three of the squad are north along the wall and one south of the hole. Sam is east
/// of it with his gun turned away.
fn blown(seed: u32) -> Play {
    let mut p = Play::mission("pass.json", "?levelFile=pass.json&roomclear&squad4", &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.run(0.5);
    p.sam_put([36.0, 17.5]);
    let spots = [[35.0, 12.4], [34.8, 11.5], [34.6, 10.5], [35.5, 19.0]];
    let men = p.mates();
    for (i, &m) in men.iter().enumerate() { p.man_stands(m, spots[i]); }
    p.order("fallIn", p.sam_at(), Who::All);
    p.run(1.0);
    game::wallcharge::wall_charge_raise_carried(&mut p.g, 2.0);
    p.frame();
    p.orders_wall_charge([34.12, 0.0, HOLE[1]]);
    p.until(60.0, |p| p.wall_charges() > 0).unwrap_or_else(|| panic!("seed {seed}: a minute and no charge is on the wall ({})", p.roll()));
    for (i, &m) in men.iter().enumerate() { p.man_stands(m, spots[i]); }
    p.sam_put([37.7, 18.2]);
    p.point_at([39.5, 24.0]);
    p.run(0.5);
    let c = p.g.wall_charge.set[0];
    game::wallcharge::wall_charge_blow(&mut p.g, Some(c), Some(true));
    p
}

/// From `from` seconds on, for `secs`: the frames a man of the squad is within 3 m of the hole with a line to it, and those of them that one of their guns is on it.
fn on_the_hole(p: &mut Play, from: f64, secs: f64) -> (usize, usize) {
    p.run(from);
    let (mut near, mut on) = (0, 0);
    for _ in 0..(secs * 60.0) as usize {
        p.frame();
        if let Some(&(_, w)) = p.watched(3.0).iter().find(|(at, _)| dist(*at, HOLE) < 1.0) { near += 1; if w { on += 1; } }
    }
    (near, on)
}

/// THE RULE: a squad falling in past a hole just blown has a gun on it the whole way past.
/// A FAILURE MEANS: the men walk past it with their guns the way they walk: a man right beside an opening has come to count as standing in it, and nobody is dealt it.
#[test]
fn a_file_walking_past_a_hole_keeps_a_gun_on_it() {
    for s in SEEDS {
        let mut p = blown(s);
        p.order("fallIn", p.sam_at(), Who::All);
        let (near, on) = on_the_hole(&mut p, 0.5, 4.0);
        assert!(near > 120, "seed {s}: the staging is wrong: a man was by the hole for {near} frames ({})", p.roll());
        assert!(on as f64 > 0.9 * near as f64, "seed {s}: a gun was on the hole for {on} of the {near} frames a man was by it");
    }
}

/// THE RULE: men sent somewhere past it each have their gun on it as they go by: on the walk of a plain "move there" a man's eyes are his own.
/// A FAILURE MEANS: only a man falling in or holding covers anything, and one walking to a mark walks past a hole looking where he goes (before: two frames in five, and half of them with his
/// gun the other way).
#[test]
fn men_sent_past_a_hole_look_into_it() {
    for s in SEEDS {
        for to in [[36.5, 20.5], [35.2, 20.5]] {
            let mut p = blown(s);
            p.order("move", to, Who::All);
            p.run(0.3);
            let men = p.mates();
            let mut by: Vec<(usize, usize)> = vec![(0, 0); men.len()];
            for _ in 0..(6 * 60) {
                p.frame();
                for (i, &m) in men.iter().enumerate() {
                    let d = dist(p.at(m), HOLE);
                    if !p.up(m) || d > 2.0 { continue; }
                    by[i].0 += 1;
                    if p.gun_off(m, HOLE) < 60.0 { by[i].1 += 1; }
                }
            }
            let passed: Vec<(usize, usize)> = by.into_iter().filter(|b| b.0 > 60).collect();
            assert!(passed.len() >= 3, "seed {s}: the staging is wrong: {} men went by the hole ({})", passed.len(), p.roll());
            for (near, on) in passed { assert!(on as f64 > 0.8 * near as f64, "seed {s}, sent to {to:?}: one of them had his gun toward the hole for {on} of the {near} frames he was within two metres of it"); }
        }
    }
}
