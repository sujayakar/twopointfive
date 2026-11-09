//! HIS HANDS (rust/game_rules/src/guards/frame.rs `hands`): a change of magazine a man has begun goes on, whoever has the rest of him, and it is the same one
//! for every man of either side.
use game::game::GuardState;
use game::orders::Who;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 3] = [1, 2, 3];
fn seed(seed: u32) -> Opts { Opts { seed, ..Default::default() } }

/// A lit room 14 m by 8 m with nothing in it: a sentry at its west end looking east, and Sam standing 6 to 7 m in front of him, in the open.
fn open_room(seed: u32) -> Scene {
    let mut s = Scene::new("open-room");
    s.room("room", 10.0, 10.0, 24.0, 18.0).door("south", Axis::X, 18.0, 20.0);
    s.bulb(14.0, 14.0).bulb(20.0, 14.0);
    s.sentry("post", [11.5, 14.25], [24.0, 14.25]).sam(18.0 + 0.5 * nudge(seed, 0), 14.25 + 0.75 * nudge(seed, 1));
    s
}

/// THE RULE: a guard whose gun runs dry at a man he can see gets his magazine in, in the time it takes, though the man is gone a moment later.
/// A FAILURE MEANS: his hands stop when his eyes lose the man, and the next time he sees him he stands there with an empty gun and the change still to do.
#[test]
fn a_guard_who_loses_his_man_still_gets_his_magazine_in() {
    for s in SEEDS {
        let mut p = Play::scene(&open_room(s), &seed(s));
        p.sam_unhurt();
        let post = p.guard("post");
        p.until(6.0, |p| p.state(post) == GuardState::Alert).unwrap_or_else(|| panic!("seed {s}: he never saw Sam {:.1} m in front of him in light ({})", dist(p.at(post), p.sam_at()), p.roll()));
        p.gun_has(post, 2.0);
        p.until(15.0, |p| p.reloading(post).is_some()).unwrap_or_else(|| panic!("seed {s}: two rounds in his gun and it never ran dry ({})", p.why(post)));
        let left = p.reloading(post).unwrap();
        p.sam_put([34.0, 24.0]);
        p.until(left + 0.3, |p| p.reloading(post).is_none()).unwrap_or_else(|| panic!("seed {s}: {left:.1} s of it were left when Sam went, and {:.1} s after that it is still not in ({})", left + 0.3, p.why(post)));
        assert!(p.up(post), "seed {s}: the staging is wrong: he is down");
    }
}

/// THE RULE: while a man changes magazine no round leaves his gun, and the change gets nearer its end every frame he has a hand for it. The squad's men too.
/// A FAILURE MEANS: a man of the squad and a man of theirs have two kinds of reload again, or one of them shoots with his magazine out.
#[test]
fn nobody_fires_with_his_magazine_out_and_no_change_stands_still() {
    let (mut ours, mut theirs) = (0, 0);
    for s in SEEDS {
        let mut p = Play::mission("hall.json", "?levelFile=hall.json&roomclear&squad4", &seed(s));
        p.sam_unhurt();
        p.run(0.5);
        let (men, mates) = (p.g.guards.clone(), p.mates());
        for &m in &men { p.gun_has(m, 2.0); p.no_second_gun(m); }
        let mut was: Vec<(Option<f64>, f64)> = men.iter().map(|&m| (p.reloading(m), p.rounds_fired(m))).collect();
        for k in 0..(40 * 60) {
            if k % 300 == 0 {
                let Some(&lead) = mates.iter().find(|&&m| p.up(m)) else { break };
                let at = p.at(lead);
                let Some(near) = p.hostiles_by(at).into_iter().find(|&h| p.up(h)) else { break };
                p.tries_order("move", p.at(near), Who::All);
            }
            p.frame();
            for (i, &m) in men.iter().enumerate() {
                let now = (p.reloading(m), p.rounds_fired(m));
                if let (Some(a), Some(b), true) = (was[i].0, now.0, p.up(m)) {
                    if mates.contains(&m) { ours += 1; } else { theirs += 1; }
                    assert_eq!(was[i].1, now.1, "seed {s}: a round left his gun with {b:.2} s of his reload to go ({})", p.why(m));
                    assert!(b < a || p.no_hand_for_it(m), "seed {s}: his reload stood still at {b:.2} s ({})", p.why(m));
                }
                was[i] = now;
            }
        }
    }
    assert!(ours > 60 && theirs > 60, "the staging is wrong: the squad's men were changing magazine for {ours} frames and theirs for {theirs}");
}

/// The office's room clear with nobody on the floor but the squad, its first man with `rounds` in his long gun and no other: how long until he has begun a change, if he does within `secs`.
fn begins(seed: u32, rounds: f64, secs: f64) -> Option<f64> {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.run(0.1);
    let mates = p.mates();
    p.drop_all_but(&[]);
    let m = mates[0];
    p.run(6.0);
    p.no_second_gun(m);
    p.gun_has(m, rounds);
    p.until(secs, |p| p.reloading(m).is_some())
}

/// THE RULE: a man left with an empty gun on a quiet floor changes his magazine then; with a low one, once it has been quiet a while; with a full one, never.
/// A FAILURE MEANS: the last man of a fight walks into the next room on an empty gun, and his first pull there is a click.
#[test]
fn a_man_on_a_quiet_floor_sees_to_his_gun() {
    for s in SEEDS {
        let dry = begins(s, 0.0, 10.0).unwrap_or_else(|| panic!("seed {s}: ten seconds on a quiet floor with nothing in his gun"));
        assert!(dry < 1.0, "seed {s}: dry, and {dry:.1} s before he saw to it");
        assert!(begins(s, 3.0, 10.0).is_some(), "seed {s}: ten seconds on a quiet floor with three rounds in his gun");
        assert!(begins(s, 28.0, 10.0).is_none(), "seed {s}: he changed a magazine that was nearly full");
    }
}
