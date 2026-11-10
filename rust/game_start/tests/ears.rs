//! A MAN HEARS AND SEES WHOEVER HAS HIS BODY (rust/game_rules/src/people/guards/senses/ `sense`): his ears and eyes run every tick, before any rule of the list or any
//! script is asked.
use game::ids::GuardId;
use game_start::play::{nudge, Button, Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 3] = [1, 2, 3];
const GRENADE: &str = "a live grenade";

/// A lit hall 26 m by 12 m: a sentry in the middle looking east, Sam out of his sight behind a stub of wall in the south-west corner with his rifle on the far wall.
fn hall(seed: u32) -> (Play, GuardId) {
    let mut s = Scene::new("hall");
    s.room("hall", 6.0, 8.0, 32.0, 20.0).door("gate", Axis::X, 20.0, 28.0);
    s.wall_z(9.0, 15.0, 20.0);
    s.bulb(12.0, 12.0).bulb(19.0, 14.0).bulb(26.0, 12.0);
    s.sentry("post", [19.0 + 0.5 * nudge(seed, 0), 14.0 + 0.5 * nudge(seed, 1)], [32.0, 14.0]).sam(7.5, 18.5);
    let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    let gd = p.guard("post");
    p.sam_takes_his_rifle();
    p.point_at([7.5, 8.2]);
    p.run(1.0);
    (p, gd)
}

/// A can lands by him and he is off: on return the rule has had him for `frames` frames.
fn running(p: &mut Play, gd: GuardId, seed: u32, frames: usize) {
    let at = p.at(gd);
    p.frag_lands([at[0] + 1.0, at[1] + 0.4]);
    p.until(2.0, |p| p.rule(gd) == Some(GRENADE)).unwrap_or_else(|| panic!("seed {seed}: he never ran from the can ({})", p.why(gd)));
    for _ in 0..frames { p.frame(); }
    assert_eq!(p.rule(gd), Some(GRENADE), "seed {seed}: the staging is wrong: the rule let him go ({})", p.why(gd));
}

/// THE RULE: a man running from a live grenade hears a shot fired meanwhile.
/// A FAILURE MEANS: a man a rule has is deaf again.
#[test]
fn a_man_running_from_a_grenade_hears_a_shot() {
    for s in SEEDS {
        let (mut p, gd) = hall(s);
        running(&mut p, gd, s, 4);
        let was = p.shots_heard(gd);
        p.press(Button::Left);
        for _ in 0..12 { p.frame(); }
        p.release(Button::Left);
        assert_eq!(p.rule(gd), Some(GRENADE), "seed {s}: the staging is wrong: the rule let him go ({})", p.why(gd));
        assert!(p.shots_heard(gd) > was, "seed {s}: Sam's rifle went off in the same hall and he heard nothing ({})", p.why(gd));
    }
}

/// THE RULE: a man running from a live grenade sees a man he runs toward.
/// A FAILURE MEANS: a man a rule has is blind again.
#[test]
fn a_man_running_from_a_grenade_sees_who_is_in_front_of_him() {
    for s in SEEDS {
        let (mut p, gd) = hall(s);
        assert!(!p.saw_within(gd, 5.0), "seed {s}: the staging is wrong: he has seen Sam already");
        running(&mut p, gd, s, 4);
        let (at, yaw) = (p.at(gd), p.facing(gd));
        p.sam_put([at[0] + 3.0 * yaw.sin(), at[1] + 3.0 * yaw.cos()]);
        let (mut had, mut saw) = (0, false);
        for _ in 0..30 { p.frame(); if p.rule(gd) == Some(GRENADE) { had += 1; saw |= p.saw_within(gd, 0.1); } }
        assert!(had > 10, "seed {s}: the staging is wrong: the rule let him go ({})", p.why(gd));
        assert!(saw, "seed {s}: Sam stood three metres in front of him in the light and he saw nobody ({})", p.why(gd));
    }
}
