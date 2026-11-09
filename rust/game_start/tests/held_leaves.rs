//! A LEAF IS A THING WITH WEIGHT: it does not come through the man who pulls it, a man in its way stops it or is carried, a boot throws him.
mod kit_ask;
use game::game::GuardState;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use kit_ask::KitAsk;

fn two_rooms(s: u32) -> Scene {
    let mut sc = Scene::new("leaf");
    sc.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    sc.door("between", Axis::Z, 18.0, 12.4 + 0.4 * nudge(s, 0)).door("yard", Axis::X, 16.0, 21.4);
    sc.bulb(14.0, 13.0).bulb(22.0, 13.0);
    sc
}

/// Sam walks at the shut leaf by his own keys, `out` metres off it (positive: the side it opens toward), `along` of its width, on through the doorway: did the leaf ever pass through him, how deep it ever stood in him, and is he through.
fn walks_in(along: f64, out: f64, s: u32) -> (bool, f64, bool) {
    let mut sc = two_rooms(s);
    sc.sentry("man", [25.0, 15.0], [20.0, 15.0]);
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_a_ghost();
    p.sam_unhurt();
    p.run(0.1);
    let d = p.door("between");
    let w = p.leaf_w(d);
    p.camera_square();
    p.sam_put(p.leaf_pt(d, w * along, out));
    p.run(0.1);
    let start = p.sam_at();
    let goal = p.leaf_pt(d, w * along, -out.signum() * 2.0);
    let key = if goal[0] > start[0] { "KeyD" } else { "KeyA" };
    p.key_down(key);
    let (mut crossed, mut deepest, mut last) = (false, 0.0f64, p.to_leaf(d, p.sam_at()));
    for _ in 0..(4 * 60) {
        p.frame();
        let r = p.to_leaf(d, p.sam_at());
        if r.0 && last.0 && r.1 != last.1 { crossed = true; }
        if r.0 { deepest = deepest.max(r.2); }
        last = r;
    }
    let through = (p.sam_at()[0] - 18.0).signum() != (start[0] - 18.0).signum();
    (crossed, deepest, through)
}

/// THE RULE: "player should have to open it and walk aroudn it": walked at from the face it opens toward, the leaf comes round him and never through him, wherever along it he comes, and he is through the doorway all the same. The control: from the other face it simply swings away.
/// A FAILURE MEANS: the player clips through a door swinging out toward him, as the maintainer met it.
#[test]
fn a_leaf_pulled_open_does_not_come_through_him() {
    for (s, along) in [(1, 0.3), (2, 0.5), (3, 0.75)] {
        let (crossed, deepest, through) = walks_in(along, 1.5, s);
        assert!(!crossed, "{along} along: the leaf came through him");
        assert!(deepest < 0.06, "{along} along: it stood {deepest:.3} m in him");
        assert!(through, "{along} along: he never got through the doorway");
    }
    let (crossed, deepest, through) = walks_in(0.5, -1.5, 1);
    assert!(!crossed && deepest < 0.06 && through, "the control: from the far face: crossed {crossed}, {deepest:.3} m deep, through {through}");
}

/// A man `behind` metres off the shut leaf on the side it opens into, his wits off; another standing on the side it is pushed from; Sam out of it.
fn man_behind_a_leaf(s: u32) -> (Play, game::ids::GuardId, game::ids::GuardId, game::ids::DoorId, [f64; 2]) {
    let mut sc = two_rooms(s);
    sc.sentry("man", [25.0, 15.0], [20.0, 15.0]).sentry("other", [25.0, 11.0], [20.0, 11.0]).sam(11.0, 11.0);
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_a_ghost();
    p.sam_unhurt();
    p.no_wits();
    p.run(0.1);
    let (man, other, d) = (p.guard("man"), p.guard("other"), p.door("between"));
    let w = p.leaf_w(d);
    let at = p.leaf_pt(d, w * 0.6, 0.48);
    p.man_stands(man, at);
    p.man_stands(other, p.leaf_pt(d, w * 0.6, -1.6));
    p.run(0.05);
    (p, man, other, d, at)
}

/// THE RULE: "kicking down a door when an enemy on the side should stun or knock them out": a boot throws the man it meets: folded, blind, on a knee, unhurt.
/// A man of his own side walking it open is stopped by him: the leaf stops at a man in its way, and he is not moved.
/// A FAILURE MEANS: boots have no weight on a man, or a man walks through his own friend with a leaf.
#[test]
fn a_leaf_stops_at_a_man_in_its_way_and_a_kicked_one_throws_him() {
    for s in [1, 2, 3] {
        let (mut p, man, _, d, _) = man_behind_a_leaf(s);
        let w = p.leaf_w(d);
        p.door_booted(d, p.leaf_pt(d, w * 0.6, -1.3));
        let (mut fold, mut knee) = (0.0f64, false);
        for _ in 0..60 { p.frame(); fold = fold.max(p.folded(man)); knee |= p.on_a_knee(man); }
        assert!(fold > 0.5, "seed {s}: a boot and he is not folded ({fold:.2} s)");
        assert!(knee, "seed {s}: a boot and he is not on a knee");
        assert!(p.up(man), "seed {s}: a boot and he is down");
        drop(p);

        // (his own side's walk at it)
        let (mut p, man, other, d, _) = man_behind_a_leaf(s);
        let w = p.leaf_w(d);
        p.man_walks_to(other, p.leaf_pt(d, w * 0.5, -1.0), p.leaf_pt(d, w * 0.5, 2.0), 1.3);
        p.run(2.0);
        assert_eq!(p.door_on_men(), (0, 0, 0), "seed {s}: his own side's walk startled, carried or threw him");
        assert_eq!((p.folded(man), p.state(man)), (0.0, GuardState::Patrol), "seed {s}: his own side's door put him up");
        drop(p);

        // (the control: Sam's walk at the same leaf: startled, carried aside, and nothing worse)
        let (mut p, man, _, d, at) = man_behind_a_leaf(s);
        let w = p.leaf_w(d);
        p.camera_square();
        p.sam_put(p.leaf_pt(d, w * 0.6, -1.3));
        p.run(0.1);
        let (start, goal) = (p.sam_at(), p.leaf_pt(d, w * 0.6, 2.0));
        p.key_down(if goal[0] > start[0] { "KeyD" } else { "KeyA" });
        p.run(2.0);
        let (startled, _, thrown) = p.door_on_men();
        assert!(startled == 1 && thrown == 0 && p.folded(man) == 0.0, "seed {s}: the control: Sam's walk at the leaf: startled {startled}, thrown {thrown}, folded {:.2} s", p.folded(man));
        assert!(dist(p.at(man), at) > 0.2, "seed {s}: the control: Sam's walk carried him {:.2} m", dist(p.at(man), at));
    }
}
