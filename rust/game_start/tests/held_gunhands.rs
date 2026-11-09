//! THE GUN IN THE HOLD AND AT A WALL: what a click is, there and in the open.
mod kit_ask;
use game_start::play::{nudge, Button, Opts, Play};
use game_start::scene::{Axis, Scene};
use kit_ask::KitAsk;

/// A lit room: Sam with his pistol up behind one man, a second man planted 6 m ahead of them with his back to them. `grab`: the first is in Sam's arm.
fn the_gun_hold(grab: bool, s: u32) -> (Play, game::ids::GuardId, game::ids::GuardId) {
    let mut sc = Scene::new("gun-hold");
    let z = 14.0 + 0.5 * nudge(s, 0);
    sc.room("room", 6.0, 10.0, 28.0, 18.0).door("south", Axis::X, 18.0, 20.0).bulb(14.0, 14.0).bulb(22.0, 14.0);
    sc.sentry("held", [14.0, z], [27.0, z]).sentry("ahead", [20.27, z], [27.0, z]).sam(13.3, z);
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_a_ghost();
    p.sam_unhurt();
    p.no_wits();
    p.run(0.1);
    let (held, ahead) = (p.guard("held"), p.guard("ahead"));
    p.sam_arms("pistol");
    if grab {
        p.sam_grabs(held);
        p.until(2.5, |p| p.in_the_arm()).expect("the grab never settled");
        p.run(0.7);
    } else {
        p.man_stands(held, [25.0, z + 3.0]);
    }
    (p, held, ahead)
}
/// Sam clicks every frame for `secs` with his cursor on the man ahead: the seconds between each round and the next.
fn clicks(p: &mut Play, ahead: game::ids::GuardId, secs: f64) -> Vec<f64> {
    let (mut last, mut was, mut gaps) = (None, p.pistol_rounds(), Vec::new());
    p.point_at(p.at(ahead));
    for _ in 0..(secs * 60.0) as usize {
        p.press(Button::Left);
        p.frame();
        p.release(Button::Left);
        if p.up(ahead) { p.point_at(p.at(ahead)); }
        let now = p.pistol_rounds();
        if now < was { if let Some(t) = last { gaps.push(p.time() - t); } last = Some(p.time()); }
        was = now;
    }
    gaps
}

/// THE RULE: in the gun hold a click is one round past him, at least 0.7 s apart however fast he clicks; the man in the arm is never hit, the hold goes on, and the man ahead is. The control: out of the hold the same clicking fires far faster.
/// A FAILURE MEANS: the pistol at a man's jaw empties in a second, or a round goes through the man in the arm.
#[test]
fn in_the_gun_hold_a_click_is_a_round_past_him_at_least_07_s_apart() {
    for s in [1, 2, 3] {
        let (mut p, held, ahead) = the_gun_hold(true, s);
        let gaps = clicks(&mut p, ahead, 3.4);
        assert!((3..=6).contains(&gaps.len()), "seed {s}: {} rounds in 3.4 s of clicking: {gaps:?}", gaps.len() + 1);
        assert!(gaps.iter().all(|&g| g >= 0.7 - 1e-6), "seed {s}: two rounds closer than 0.7 s: {gaps:?}");
        assert!(p.up(held) && p.in_the_arm(), "seed {s}: the man in the arm went down or the hold ended");
        assert!(!p.up(ahead), "seed {s}: the man 6 m ahead was never hit");
        drop(p);

        let (mut p, _, ahead) = the_gun_hold(false, s);
        let gaps = clicks(&mut p, ahead, 1.2);
        assert!(gaps.iter().any(|&g| g < 0.7), "seed {s}: the control: out of the hold the pistol is just as slow: {gaps:?}");
    }
}

/// THE RULE: Sam pressed to a wall with his gun at it: the gun comes down and a click fires nothing; back from the wall it is up and a click fires.
/// (A man's gun at a wall: kit.rs a_gun_is_up_with_a_man_in_his_sights_as_soon_as_it_is_clear_of_the_wall. The ring barred: hud_marks.rs the_ring_is_barred_while_the_wall_has_his_gun.)
/// A FAILURE MEANS: the player fires out through a wall his gun is in, or stays unable to fire off it.
#[test]
fn at_a_wall_a_click_fires_nothing_and_back_from_it_a_click_fires() {
    for s in [1, 2, 3] {
        let mut sc = Scene::new("gun-wall");
        let z = 13.0 + 0.5 * nudge(s, 0);
        sc.room("west", 10.0, 10.0, 18.0, 16.0).door("south", Axis::X, 16.0, 12.4).bulb(14.0, 13.0);
        sc.sam(17.45, z);
        let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
        p.sam_unhurt();
        p.sam_a_ghost();
        for gun in ["pistol", "rifle"] {
            p.sam_arms(gun);
            p.sam_put([17.45, z]);
            p.point_at([25.0, z]);
            p.until(1.5, |p| p.view().aim.ring.is_some_and(|r| r.shut)).unwrap_or_else(|| panic!("seed {s}, {gun}: at the wall the gun is not shut"));
            let r0 = p.sam_rounds();
            p.press(Button::Left);
            p.frame();
            p.release(Button::Left);
            p.run(0.3);
            assert_eq!(p.sam_rounds(), r0, "seed {s}, {gun}: a click at the wall fired");
            p.sam_put([15.2, z]);
            p.until(1.5, |p| p.view().aim.ring.is_some_and(|r| !r.shut)).unwrap_or_else(|| panic!("seed {s}, {gun}: back from the wall the gun stays shut"));
            p.run(0.6);
            let r0 = p.sam_rounds();
            p.press(Button::Left);
            p.frame();
            p.release(Button::Left);
            p.run(0.3);
            assert!(p.sam_rounds() < r0, "seed {s}, {gun}: the control: back from the wall a click fired nothing");
        }
    }
}
