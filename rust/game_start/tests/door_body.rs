//! A SHUT DOOR OPENED ON SAM'S BODY: the leaf swings his way and his body stops it, between him and the man who opened it.
use game::game::GuardState;
use game_start::play::{Opts, Play};

/// Sam at `sam` behind the shut door to the break room, walking back onto his spot if he is pushed off it; an alert man on the corridor with a second-old fix on him.
/// (When he first fires from the man's starting, the longest the man stood alert after opening the door with no round going off.)
fn opened_on(sam: [f64; 2], seed: u32) -> (Option<f64>, f64) {
    let mut p = Play::level("office.json", false, &Opts { seed, ..Default::default() });
    p.g.god_mode = true;
    p.run(0.17);
    let door = game::doors::doors_by_name(&p.g, "breakroom_n").expect("the office's break room door");
    assert!(p.g.dr[door].leaf.latched, "the staging is wrong: the door stands open");
    let gd = p.g.guards[0];
    for o in p.g.guards.clone().into_iter().skip(1) { let m = &mut p.g.gd[o]; (m.hold, m.pinned, m.dazzled_until) = (true, true, 1e9); }
    p.sam_put(sam);
    let at = p.sam_at();
    p.g.puppet = Some(game::game::GamePuppet { goal: Some([at[0], 0.0, at[1]]), aim: Some([27.5, 1.2, 11.3]), ..game::game::GamePuppet::default() });
    let c = p.g.gd[gd].ch;
    let (x, z) = (27.5, 11.3);
    p.g.ch[c].pos = [x, 0.0, z];
    p.g.ch[c].vel = [0.0; 3];
    let yaw = f64::atan2(at[0] - x, at[1] - z);
    (p.g.ch[c].body_yaw, p.g.ch[c].aim_yaw) = (yaw, yaw);
    let t0 = p.g.time;
    let m = &mut p.g.gd[gd];
    (m.state, m.awareness, m.react_t, m.fire_cd, m.last_known, m.last_seen_t, m.dazzled_until, m.pinned, m.hold, m.foe) = (GuardState::Alert, 1.0, 0.0, 0.3, Some([at[0], 0.0, at[1]]), t0 - 1.0, -1.0, false, false, Some(game::game::Foe::Player));
    m.path.clear();
    m.path_goal = None;
    let (mut first, mut last_shot, mut opened, mut spell, mut worst) = (None, t0, false, 0.0f64, 0.0f64);
    for _ in 0..(14 * 60) {
        p.frame();
        let now = p.g.time;
        if p.g.events.iter().any(|e| e.kind == game::director::EventKind::GuardShot && e.time == now) { last_shot = now; first = first.or(Some(now - t0)); }
        opened = opened || p.g.dr[door].leaf.angle.abs() > 0.1;
        let m = &p.g.gd[gd];
        if opened && m.state == GuardState::Alert && m.speed < 0.25 && now - last_shot > 2.0 { spell += 1.0 / 60.0; worst = worst.max(spell); } else { spell = 0.0; }
    }
    assert!(opened, "the staging is wrong: Sam at {sam:?}: nobody opened the door");
    (first, worst)
}

/// THE RULE: a man who has opened a door on the man he is after gets on with it: inside three seconds of standing alert with nothing fired he has moved or fired, and his
/// first round goes inside eight seconds of the start. Sam right behind the leaf, and a pace off it to either side.
/// A FAILURE MEANS: a defender stands in the frame at arm's length from Sam, seeing him and doing nothing.
#[test]
fn a_man_who_opens_a_door_on_sam_does_not_stand_in_the_frame() {
    for (seed, sam) in [(13, [30.6, 12.75]), (14, [30.3, 13.35]), (15, [30.9, 12.65])] {
        let (first, worst) = opened_on(sam, seed);
        assert!(worst < 3.0, "Sam at {sam:?}: he stood {worst} s alert with nothing fired");
        assert!(first.map_or(false, |f| f < 8.0), "Sam at {sam:?}: first round at {first:?}");
    }
}
