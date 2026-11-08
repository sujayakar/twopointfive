//! A MAN WHO HAS HEARD HIM BEHIND A SHUT DOOR does not wait at it: rounds, then a boot; and a post takes the walk off him.
mod kit_ask;
use game::ids::{DoorId, GuardId};
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use kit_ask::KitAsk;

/// Two lit rooms and the shut door between them. An alert man on a post 3 m off the leaf on the face it opens away from; Sam at 0.9 m on the other face, or (`far`) in the far corner of his room.
fn at_a_shut_door(far: bool, s: u32) -> (Play, GuardId, DoorId, [f64; 2], [f64; 2]) {
    let mut sc = Scene::new("shut-door");
    sc.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    sc.door("between", Axis::Z, 18.0, 12.4 + 0.8 * nudge(s, 0)).door("yard", Axis::X, 16.0, 21.4);
    sc.bulb(14.0, 13.0).bulb(22.0, 13.0);
    sc.sentry("man", [12.0, 11.0], [10.0, 11.0]);
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    let (man, d) = (p.guard("man"), p.door("between"));
    p.run(0.1);
    let w = p.leaf_w(d);
    let (him, me) = (p.leaf_pt(d, w / 2.0, -3.0), p.leaf_pt(d, w / 2.0, 0.9));
    let sam = if far { let m = p.leaf_pt(d, w / 2.0, 6.0); [m[0], if m[1] > 13.0 { 11.0 } else { 15.0 }] } else { me };
    p.sam_put(sam);
    p.fights_from(man, him, me, 100.0, 0.0);
    p.man_posted(man);
    (p, man, d, him, me)
}
/// Sam runs up and down behind the leaf for `secs`, his footfalls filed as his run files them. `each` is asked each frame.
fn pacing(p: &mut Play, at: [f64; 2], secs: f64, mut each: impl FnMut(&mut Play)) {
    p.key_down("ShiftLeft");
    for _ in 0..(secs * 60.0).round() as usize {
        if p.frames % 45 == 0 { p.sam_walks_to([at[0], at[1] + if p.frames % 90 < 45 { 0.6 } else { -0.6 }], false); }
        p.frame();
        each(p);
    }
}

/// Sam paces until the posted man has put rounds through the leaf, for at most 30 s: he has not left his post for it, and the door stands.
fn heard_and_fired(p: &mut Play, man: GuardId, d: DoorId, me: [f64; 2], him: [f64; 2], s: u32) {
    let mut fired = false;
    for _ in 0..(30 * 60) { if !fired { pacing(p, me, 1.0 / 60.0, |p| fired = p.rounds_fired(man) > 0.0); } }
    assert!(fired, "seed {s}: 30 s of footfalls behind the leaf and no round: {}", p.why(man));
    assert!(!p.door_in(d) && dist(p.at(man), him) < 0.3, "seed {s}: he left his post for it (door in {}, {:.2} m off)", p.door_in(d), dist(p.at(man), him));
}

/// THE RULE: "an alerted man does not wait at a shut door": a man who has heard footfalls behind a shut wooden leaf fires through it, holding his post, and let off his post he walks up and boots it in. The control: alert with nothing to hear behind it, no round and no boot.
/// A FAILURE MEANS: a defender stands at a shut door with Sam walking about a hand behind it, for ever.
#[test]
fn a_man_who_has_heard_him_behind_a_shut_door_boots_it() {
    for s in [2, 3, 4] {
        let (mut p, man, d, him, me) = at_a_shut_door(false, s);
        heard_and_fired(&mut p, man, d, me, him, s);
        p.man_let_off_his_post(man);
        let mut inn = false;
        for _ in 0..(16 * 60) { if !inn { pacing(&mut p, me, 1.0 / 60.0, |p| inn = p.door_in(d)); } }
        assert!(inn, "seed {s}: 16 s off his post and the door is whole: {}", p.why(man));
        drop(p);

        let (mut p, man, d, _, _) = at_a_shut_door(true, s);
        p.run(10.0);
        assert_eq!(p.rounds_fired(man), 0.0, "seed {s}: the control: nothing to hear and he fired");
        assert!(!p.door_in(d), "seed {s}: the control: nothing to hear and he booted it");
    }
}

/// THE RULE: a man walking up to boot a door who is put on a post gives up the walk: he takes no step in the second after and still believes in the door. The control: left free he goes on and reaches it.
/// A FAILURE MEANS: a rule that stops a man short does not stop him, and he leaves his post for the door under it.
#[test]
fn a_post_ends_the_walk_to_a_door() {
    for (s, post) in [(2, true), (3, true), (4, true), (2, false)] {
        let (mut p, man, d, him, me) = at_a_shut_door(false, s);
        heard_and_fired(&mut p, man, d, me, him, s);
        p.man_let_off_his_post(man);
        let mut walking = false;
        for _ in 0..(16 * 60) { if !walking { pacing(&mut p, me, 1.0 / 60.0, |p| walking = p.walking_to_boot(man)); } }
        assert!(walking, "seed {s}: he never set out for the door: {}", p.why(man));
        if post { p.man_posted(man); }
        let was = p.at(man);
        let door_at = dist(was, p.frame_mid(d));
        p.run(1.0);
        if post {
            assert!(dist(p.at(man), was) < 0.25, "seed {s}: posted, he walked {:.2} m in the second after", dist(p.at(man), was));
            assert!(!p.walking_to_boot(man) && p.on_a_door(man), "seed {s}: posted: the walk goes on, or he no longer believes in the door");
        } else {
            assert!(dist(p.at(man), p.frame_mid(d)) < door_at, "seed {s}: the control: left free he went no nearer the door");
        }
    }
}
