//! A BURST IS AT ONE MAN (rust/game_rules/src/mind.rs `foe`): the man it was begun at down, it ends, and the next man is aimed at like the first.
use game::ids::GuardId;
use game_start::play::{nudge, Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 6] = [1, 2, 3, 4, 5, 8];

/// Two rooms side by side, a defender in each; three of the team stand in a bunch in the west one, in its light, with nothing to see by for a while, so that they do not shoot back.
fn bunch(seed: u32) -> (Play, GuardId, Vec<GuardId>) {
    let mut s = Scene::new("bunch");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("west-yard", Axis::X, 16.0, 13.4).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    s.bulb(12.0, 13.0).bulb(15.5, 13.0).bulb(22.0, 13.0).moon();
    s.team([14.0, 24.0], std::f64::consts::PI, &[[13.0, 25.0], [15.0, 25.0], [13.0, 26.0], [15.0, 26.0]]);
    s.defender("west", [16.75, 13.0], "between").defender("east", [24.25, 12.25], "east-yard");
    let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.run(0.5);
    let gd = p.defender_at([16.75, 13.0]);
    let men: Vec<GuardId> = p.mates().into_iter().take(3).collect();
    for (i, &m) in men.iter().enumerate() {
        p.man_stands(m, [12.2 + 0.2 * nudge(seed, i as u32), 12.2 + 0.7 * i as f64]);
        p.blinded(m, 60.0);
    }
    (p, gd, men)
}

/// THE RULE: an enemy with three men of the team in front of him drops them one at a time: between one and the next he has to turn his gun and lay it.
/// A FAILURE MEANS: the rest of a burst goes into the next man, with no aim of his own (before: three men down in a fifth of a second).
#[test]
fn a_burst_ends_with_the_man_it_was_at() {
    let mut pairs = 0;
    for s in SEEDS {
        let (mut p, gd, men) = bunch(s);
        let mut fell: Vec<f64> = Vec::new();
        for _ in 0..(20 * 60) {
            let up = men.iter().filter(|&&m| p.up(m)).count();
            p.frame();
            let now = men.iter().filter(|&&m| p.up(m)).count();
            for _ in now..up { fell.push(p.time()); }
            if now == 0 { break; }
        }
        assert!(p.up(gd), "seed {s}: the staging is wrong: the defender is down ({})", p.roll());
        for w in fell.windows(2) {
            pairs += 1;
            assert!(w[1] - w[0] > 0.3, "seed {s}: two of them fell {:.2} s apart to one man ({:?})", w[1] - w[0], fell);
        }
    }
    assert!(pairs >= SEEDS.len(), "the staging is wrong: {pairs} second men fell over {} seeds", SEEDS.len());
}
