//! THE RING AIRFOIL: on a calm man's head it puts him out; off a wall it is a clack that nobody made.
mod kit_ask;
use game::game::{DownKind, GuardState};
use game_start::play::{dist, nudge, Button, Opts, Play};
use game_start::scene::{Axis, Scene};
use kit_ask::KitAsk;

/// A dark room, Sam with his rifle up (a fresh one has the ring on its button) and a calm man `d` metres east of him with his back to him.
fn ring_stage(d: f64, s: u32) -> (Play, game::ids::GuardId) {
    let mut sc = Scene::new("ring");
    let z = 14.0 + 0.4 * nudge(s, 0);
    sc.room("room", 10.0, 10.0, 30.0, 18.0).door("south", Axis::X, 18.0, 20.0).bulb_off(16.0, 14.0);
    sc.sentry("man", [13.0 + d, z], [29.0, z]).sam(13.0, z);
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.sam_takes_his_rifle();
    let man = p.guard("man");
    (p, man)
}

/// THE RULE: a ring on the head of a calm man takes him: out cold, breathing, a knockout on the card and no kill, the ring lying spent. The press shows the arc and the release sends it; a left click would be a round.
/// The control: the same right click at the wall beside him takes nobody.
/// A FAILURE MEANS: the launcher kills, or the less-lethal way to take a man has stopped reaching him.
#[test]
fn a_ring_to_the_head_of_a_calm_man_takes_him() {
    for s in [1, 2, 3] {
        let (mut p, man) = ring_stage(8.0, s);
        let h = p.head(man);
        p.cursor_on(h);
        p.run(0.1);
        p.press(Button::Right);
        p.frame();
        assert_eq!(p.rings_in_air(), 0, "seed {s}: the press sent it");
        p.release(Button::Right);
        p.until(0.25, |p| p.rings_in_air() == 1).unwrap_or_else(|| panic!("seed {s}: the release did not send it"));
        p.until(1.5, |p| !p.up(man)).unwrap_or_else(|| panic!("seed {s}: a ring to the head and he is up"));
        assert_eq!(p.down_how(man), Some(DownKind::Struck), "seed {s}: how he went down");
        assert!(p.out_cold(man), "seed {s}: he is not breathing");
        assert_eq!(p.card(), (0.0, 1.0), "seed {s}: kills, knockouts");
        p.run(0.5);
        assert_eq!(p.rings_on_floor(), 1, "seed {s}: the ring is not on the floor");
        assert_eq!(p.rounds_fired(man), 0.0);
        drop(p);

        let (mut p, man) = ring_stage(8.0, s);
        let at = p.at(man);
        p.cursor_on([at[0], 1.2, 10.06]);
        p.run(0.1);
        p.press(Button::Right);
        p.frame();
        p.release(Button::Right);
        p.run(1.0);
        assert!(p.up(man), "seed {s}: the control: a ring at the wall beside him took him");
    }
}

/// THE RULE: a ring off a wall is a clack nobody made: the calm man 3 or 4 metres from it pricks up and his fix is by the wall and not on Sam, and it does not make him alert. The control: nothing fired, he stays calm with no fix.
/// A FAILURE MEANS: a clack gives away the shooter, or a thrown distraction no longer draws a man.
#[test]
fn off_a_wall_it_is_a_clack_nobody_made() {
    for s in [1, 2, 3] {
        let (mut p, man) = ring_stage(9.0, s);
        p.sam_a_ghost();
        let at = p.at(man);
        let mark = [at[0] - 3.0, 10.1];
        p.run(0.3);
        let aw = p.awareness(man);
        p.cursor_on([mark[0], 1.0, mark[1]]);
        p.run(0.1);
        p.press(Button::Right);
        p.frame();
        p.release(Button::Right);
        p.run(2.0);
        assert!(p.awareness(man) > aw + 0.05, "seed {s}: he did not prick up: {aw:.2} to {:.2}", p.awareness(man));
        let fix = p.fix(man).unwrap_or_else(|| panic!("seed {s}: no fix at all"));
        assert!(dist(fix, mark) < dist(fix, p.sam_at()), "seed {s}: his fix {fix:?} is nearer Sam than the clack at {mark:?}");
        assert!(!matches!(p.state(man), GuardState::Alert | GuardState::Dead), "seed {s}: he is {:?} off a clack", p.state(man));
        assert_eq!(p.rings_on_floor(), 1, "seed {s}: the ring is not on the floor");
        drop(p);

        let (mut p, man) = ring_stage(9.0, s);
        p.sam_a_ghost();
        p.run(3.0);
        assert!(p.fix(man).is_none() && p.state(man) == GuardState::Patrol, "seed {s}: the control: nothing fired and he went {:?} with a fix {:?}", p.state(man), p.fix(man));
    }
}
