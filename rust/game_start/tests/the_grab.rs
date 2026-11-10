//! SAM'S ARM ON A MAN: where the grab is offered, the choke's clock, how a hold ends, and what the net makes of a man in it.
mod stealth_ask;
use game::character::HoldPhase;
use game::game::{DownBy, DownKind, GuardState};
use game::ids::GuardId;
use game_start::play::{nudge, Button, Opts, Play};
use game_start::scene::{Axis, Scene};
use stealth_ask::StealthAsk;

const SEEDS: [u32; 3] = [1, 2, 3];
fn seed(seed: u32) -> Opts { Opts { seed, ..Default::default() } }
const EAST: f64 = std::f64::consts::FRAC_PI_2;

/// A dark room and a man in it, Sam well off for now, his guns put away as a night starts him.
fn dark_room(s: u32) -> Scene {
    let at = [15.0 + 0.5 * nudge(s, 0), 13.0 + 0.5 * nudge(s, 1)];
    let mut sc = Scene::new("dark-room");
    sc.room("room", 10.0, 10.0, 24.0, 16.0).door("south", Axis::X, 16.0, 16.0).bulb_off(16.0, 13.0);
    sc.sentry("post", at, [23.0, at[1]]).sam(22.0, 15.0);
    sc
}
/// The man is held looking east, calm, and Sam is put `off` metres from him, `deg` degrees round from his back.
fn sam_by_him(p: &mut Play, m: GuardId, at: [f64; 2], deg: f64, off: f64) {
    let b = deg.to_radians();
    for _ in 0..10 { p.pin_man(m, at, EAST, GuardState::Patrol, 0.0); p.frame(); }
    p.sam_put([at[0] - b.cos() * off, at[1] + b.sin() * off]);
    for _ in 0..6 { p.pin_man(m, at, EAST, GuardState::Patrol, 0.0); p.frame(); }
}
/// …and the hand tries: the right button kept down past the tap's time, and let go. Who is in Sam's arm then.
fn the_hand_tries(p: &mut Play) -> bool {
    p.press(Button::Right);
    p.run(0.5);
    p.release(Button::Right);
    p.in_arm().is_some()
}
fn held(s: u32) -> (Play, GuardId) {
    let mut p = Play::scene(&dark_room(s), &seed(s));
    let m = p.guard("post");
    let at = p.at(m);
    sam_by_him(&mut p, m, at, 0.0, 0.8);
    assert!(p.sam_takes(m), "seed {s}: staging: the right button behind him took nobody: {:?}", p.hold_phase());
    p.run(0.3);
    (p, m)
}

/// THE RULE: "grabbing only works from behind": the row and the hand's grab are there in his rear arc, and from his sides and front there is no grab
/// whatever the hand does.
/// A FAILURE MEANS: a man can be taken from in front, or from behind he no longer can.
#[test]
fn the_grab_is_offered_from_behind_only() {
    for s in SEEDS {
        for (deg, grabs) in [(0.0, true), (30.0, true), (-60.0, true), (90.0, false), (-90.0, false), (180.0, false), (140.0, false)] {
            let mut p = Play::scene(&dark_room(s), &seed(s));
            let m = p.guard("post");
            let at = p.at(m);
            sam_by_him(&mut p, m, at, deg, 0.8);
            assert_eq!(p.grab_row(m), grabs, "seed {s}, {deg}° off his back: the grab row");
            assert_eq!(the_hand_tries(&mut p), grabs, "seed {s}, {deg}° off his back: the hand's grab, with him {:?}", p.state(m));
        }
    }
}

/// THE RULE: the grab is within reach only: 0.8 m behind him it is there, 1.6 m behind him it is not.
/// A FAILURE MEANS: the reach has moved, and a man is taken from across the room.
#[test]
fn the_grab_is_offered_within_reach_only() {
    for s in SEEDS {
        for (off, grabs) in [(0.8, true), (1.6, false)] {
            let mut p = Play::scene(&dark_room(s), &seed(s));
            let m = p.guard("post");
            let at = p.at(m);
            sam_by_him(&mut p, m, at, 0.0, off);
            assert_eq!(p.grab_row(m), grabs, "seed {s}, {off} m behind him: the grab row");
            assert_eq!(the_hand_tries(&mut p), grabs, "seed {s}, {off} m behind him: the hand's grab");
        }
    }
}

/// THE RULE: never through a wall. A man with his back to a wall and Sam close behind it is not offered; with the wall a step on, the same two places
/// are.
/// A FAILURE MEANS: the gate's clear line is gone, and Sam reaches through a wall.
#[test]
fn the_grab_is_never_through_a_wall() {
    for s in SEEDS {
        for wall in [true, false] {
            let line = 15.0;
            let mut sc = Scene::new("through-a-wall");
            sc.room("west", 10.0, 10.0, line, 16.0).room("east", line, 10.0, 24.0, 16.0).door("between", Axis::Z, line, 14.5).bulb_off(16.0, 13.0);
            let at = [if wall { line + 0.4 } else { line + 1.2 }, 12.0 + 0.5 * nudge(s, 0)];
            sc.sentry("post", at, [23.0, at[1]]).sam(22.0, 15.0);
            let mut p = Play::scene(&sc, &seed(s));
            let m = p.guard("post");
            let at = p.at(m);
            sam_by_him(&mut p, m, at, 0.0, 0.8);
            assert_eq!(p.grab_row(m), !wall, "seed {s}, the wall between them {wall}: the grab row");
            assert_eq!(the_hand_tries(&mut p), !wall, "seed {s}, the wall between them {wall}: the hand's grab");
        }
    }
}

/// THE RULE: never through glass. In the office a man with his back to the conference room's glazing and Sam close behind it on the other side is not
/// offered; a step south of the pane, both on the corridor's side of it, the same two places are.
/// A FAILURE MEANS: glass, which the eye sees through, has come to pass the hand as well.
#[test]
fn the_grab_is_never_through_glass() {
    for s in SEEDS {
        for glass in [true, false] {
            let (at, sam_z) = if glass { ([12.0, 10.45], 9.6) } else { ([12.0, 11.45], 10.65) };
            let mut p = Play::level("office.json", false, &seed(s));
            let m = p.g.guards[0];
            for _ in 0..10 { p.pin_man(m, at, 0.0, GuardState::Patrol, 0.0); p.frame(); }
            p.sam_put([at[0], sam_z]);
            for _ in 0..6 { p.pin_man(m, at, 0.0, GuardState::Patrol, 0.0); p.frame(); }
            assert_eq!(p.grab_row(m), !glass, "seed {s}, the pane between them {glass}: the grab row: {:?} at {:?}, Sam at {:?}", p.state(m), p.at(m), p.sam_at());
            assert_eq!(the_hand_tries(&mut p), !glass, "seed {s}, the pane between them {glass}: the hand's grab");
        }
    }
}

/// THE RULE: the choke's clock. A tap of the right button with him in the arm closes it, and at the end of the choke's seconds he goes, out cold and
/// breathing, with the hold over. Not a moment sooner.
/// A FAILURE MEANS: the choke kills, or runs on another clock, or leaves the hold on a man who has gone.
#[test]
fn the_choke_puts_him_out_cold_and_breathing_at_the_end_of_its_clock() {
    for s in SEEDS {
        let (mut p, m) = held(s);
        let secs = p.choke_secs();
        p.press(Button::Right);
        p.frame();
        p.release(Button::Right);
        p.until(0.2, |p| p.hold_phase() == Some(HoldPhase::Choke)).unwrap_or_else(|| panic!("seed {s}: the tap did not close the arm: {:?}", p.hold_phase()));
        let t = p.until(secs + 0.5, |p| p.state(m) == GuardState::Dead).unwrap_or_else(|| panic!("seed {s}: {secs} s of the choke and he is up"));
        assert!((t - secs).abs() < 0.1, "seed {s}: he went at {t:.2} s of a {secs} s choke");
        assert!(p.out_cold(m) && p.down_kind(m) == Some(DownKind::Choked), "seed {s}: he is down as {:?}, breathing {}", p.down_kind(m), p.out_cold(m));
        assert!(p.hold_phase().is_none(), "seed {s}: the hold outlives him: {:?}", p.hold_phase());
    }
}

/// THE RULE: stopped sooner he lives: the button let go with the arm closing is the end of it, he is held again, alive; and left held he stays so.
/// A FAILURE MEANS: a choke once begun can no longer be broken off, or a hold chokes by itself.
#[test]
fn a_choke_stopped_sooner_leaves_him_alive_and_held() {
    for s in SEEDS {
        let (mut p, m) = held(s);
        let secs = p.choke_secs();
        p.press(Button::Right);
        p.run(1.0);
        assert_eq!(p.hold_phase(), Some(HoldPhase::Choke), "seed {s}: a second of the button down is the choke");
        p.release(Button::Right);
        p.until(1.0, |p| p.hold_phase() == Some(HoldPhase::Held)).unwrap_or_else(|| panic!("seed {s}: let go, the arm stays shut: {:?}", p.hold_phase()));
        p.run(secs + 1.0);
        assert!(p.up(m) && p.in_arm() == Some(m), "seed {s}: he is down or out of the arm after the choke was stopped: {:?}", p.state(m));
        drop(p);

        // CONTROL: no button at all: as long again and he is still held
        let (mut p, m) = held(s);
        p.run(secs + 2.0);
        assert!(p.up(m) && p.hold_phase() == Some(HoldPhase::Held), "seed {s}: left alone the hold ended by itself: {:?}", p.hold_phase());
    }
}

/// what must be gone when a hold has ended
fn no_hold_left(p: &Play, s: u32, how: &str) {
    assert!(p.hold_phase().is_none(), "seed {s}, {how}: Sam still holds: {:?}", p.hold_phase());
    for &m in p.g.guards.iter() { assert!(p.g.gd[m].held.is_none(), "seed {s}, {how}: {} is still held", p.g.gd[m].callsign); }
}

/// THE RULE: the hold ends when Sam goes down: the man is free where he stands, and alert.
/// A FAILURE MEANS: a dead Sam goes on holding a man, or the man wakes up calm.
#[test]
fn the_hold_ends_when_sam_goes_down() {
    for s in SEEDS {
        let (mut p, m) = held(s);
        assert_eq!(p.in_arm(), Some(m), "seed {s}: CONTROL: the hold stands before he goes down");
        p.seat_goes_down(0);
        p.run(0.1);
        no_hold_left(&p, s, "Sam down");
        assert_eq!(p.state(m), GuardState::Alert, "seed {s}: the man let go by a dead arm");
    }
}

/// THE RULE: the hold ends on a restart, and the restart has nothing of it: the man is back, calm, unknowing.
/// A FAILURE MEANS: a restart inherits the hold or the man's fright.
#[test]
fn the_hold_ends_on_a_restart() {
    for s in SEEDS {
        let (mut p, _) = held(s);
        game::savestate::control::restart(&mut p.g);
        p.run(0.1);
        no_hold_left(&p, s, "a restart");
        let m = p.guard("post");
        assert!(p.state(m) == GuardState::Patrol && p.awareness(m) == 0.0 && p.fix(m).is_none(), "seed {s}: the fresh man is {:?}, awareness {:.2}", p.state(m), p.awareness(m));
    }
}

/// THE RULE: the hold ends when the man dies in the arm by somebody else's round: Sam is free, and the kill is not his.
/// A FAILURE MEANS: Sam goes on holding a corpse, or a colleague's kill counts on his card.
#[test]
fn the_hold_ends_when_the_man_dies_in_the_arm() {
    for s in SEEDS {
        let (mut p, m) = held(s);
        p.falls(m, DownBy::Guard);
        p.run(0.1);
        no_hold_left(&p, s, "he died");
        assert!(p.state(m) == GuardState::Dead && p.down_kind(m) == Some(DownKind::Shot), "seed {s}: {:?} {:?}", p.state(m), p.down_kind(m));
    }
}

/// THE RULE: a man held through a radio check is silent on the net: his name is asked for and a checker is sent. Let go, he is back on the net and the
/// walk round his route is dropped. CONTROL: with nobody held the check runs and nobody is sent.
/// A FAILURE MEANS: a man in the arm answers the radio; a hold no longer draws a checker; or a man let go stays missing.
#[test]
fn a_man_held_through_a_radio_check_is_silent_and_a_checker_is_sent() {
    for s in SEEDS {
        for hold in [true, false] {
            let mut sc = Scene::new("radio-check");
            sc.room("shop", 10.0, 10.0, 24.0, 16.0).room("store", 24.0, 10.0, 34.0, 16.0).door("between", Axis::Z, 24.0, 12.0).door("yard", Axis::X, 16.0, 16.0).bulb_off(16.0, 13.0);
            sc.sentry("held", [29.0 + 0.5 * nudge(s, 0), 13.0], [33.0, 13.0]).sam(32.0, 15.0);
            sc.sentry("caller", [14.0, 12.0 + 0.5 * nudge(s, 1)], [11.0, 12.0]);
            let mut p = Play::scene(&sc, &seed(s));
            let (m, caller) = (p.guard("held"), p.guard("caller"));
            let at = p.at(m);
            sam_by_him(&mut p, m, at, 0.0, 0.8);
            if hold { assert!(p.sam_takes(m), "seed {s}: staging: nobody in the arm"); assert!(p.off_the_net(m), "seed {s}: a held man is off the net"); }
            let heard = p.said.len();
            p.radio_check_due(0.1);
            let sent = p.until(12.0, |p| p.checker_of(m).is_some());
            let lines: Vec<String> = p.said[heard..].iter().map(|l| l.text.clone()).collect();
            if !hold {
                assert!(sent.is_none(), "seed {s}: with nobody held a checker was sent: {lines:?}");
                continue;
            }
            assert_eq!(p.checker_of(m), Some(caller), "seed {s}: 12 s after the radio check nobody is out walking his route: {lines:?}");
            let name = p.g.gd[m].callsign.clone();
            assert!(lines.iter().any(|l| l.contains(&format!("{name}?"))), "seed {s}: his name is asked for: {lines:?}");
            assert!(p.lines(m).is_empty(), "seed {s}: he spoke from inside the arm: {:?}", p.lines(m));
            p.seat_goes_down(0);
            p.run(0.5);
            assert!(!p.off_the_net(m) && p.state(m) == GuardState::Alert, "seed {s}: let go he is on the net and alert: {:?}", p.state(m));
            assert!(p.checker_of(m).is_none() || p.state(caller) == GuardState::Alert, "seed {s}: he is back and somebody still walks his route");
        }
    }
}

/// A man with his rifle in his hands, looking east on a dark floor, Sam well behind him. (A calm man of a level's rig carries his torch.)
fn rifleman(s: u32) -> (Play, GuardId) {
    let mut p = Play::scene(&dark_room(s), &seed(s));
    let at = [18.25 + 0.5 * nudge(s, 2).round(), 13.25];
    let m = p.man_of(game::teams::Team::Guard, at, EAST);
    p.run(0.3);
    sam_by_him(&mut p, m, at, 0.0, 0.8);
    (p, m)
}

/// THE RULE: "you really want a disarm and then a grab": a man taken by hand has his gun out of his hand before the arm lands, and it goes to the floor, where it
/// comes to rest. The strip is heard, and then the catch.
/// A FAILURE MEANS: a held man keeps his gun, or the order of the two has turned round, or the gun stays in Sam's hand or in the air.
#[test]
fn a_man_taken_by_hand_has_his_gun_taken_off_him_first_and_laid_down() {
    use game::guards::InHand;
    for s in SEEDS {
        let (mut p, m) = rifleman(s);
        assert_eq!(p.hand_of(m), Some(InHand::Weapon), "seed {s}: staging: his gun is in his hands");
        let heard = p.sounds.len();
        p.press(Button::Right);
        p.until(0.5, |p| p.in_arm() == Some(m)).unwrap_or_else(|| panic!("seed {s}: nobody in the arm"));
        p.release(Button::Right);
        let t0 = p.time();
        p.until(1.0, |p| p.hand_of(m) != Some(InHand::Weapon)).unwrap_or_else(|| panic!("seed {s}: a second on the gun is still in his hand"));
        let out = p.time() - t0;
        p.until(1.0, |p| p.hold_phase() == Some(HoldPhase::Held)).unwrap_or_else(|| panic!("seed {s}: never held"));
        let held = p.time() - t0;
        assert!(out <= held, "seed {s}: the gun left him at {out:.2} s, held at {held:.2} s");
        p.until(4.0, |p| matches!(p.dropped_gun(m), Some((false, true)))).unwrap_or_else(|| panic!("seed {s}: the gun never came to rest: {:?}", p.dropped_gun(m)));
        let names: Vec<&str> = p.sounds[heard..].iter().map(|x| x.name.as_str()).collect();
        let (strip, catch) = (names.iter().position(|n| *n == "gunStrip"), names.iter().position(|n| *n == "grabCatch"));
        assert!(strip.is_some() && catch.is_some() && strip < catch, "seed {s}: the strip, then the catch: {names:?}");
    }
}

/// THE RULE: nothing to take, nothing led: a man with his torch in his hand has the catch on the grab's frame and no strip. CONTROL of the rule above.
/// A FAILURE MEANS: a bare-handed grab plays a disarm of nothing.
#[test]
fn a_man_with_no_gun_in_his_hands_has_nothing_taken() {
    use game::guards::InHand;
    for s in SEEDS {
        let mut p = Play::scene(&dark_room(s), &seed(s));
        let m = p.guard("post");
        let at = p.at(m);
        sam_by_him(&mut p, m, at, 0.0, 0.8);
        assert_ne!(p.hand_of(m), Some(InHand::Weapon), "seed {s}: staging: a calm man of a level carries no gun in his hands");
        let heard = p.sounds.len();
        assert!(p.sam_takes(m), "seed {s}: nobody in the arm");
        let names: Vec<&str> = p.sounds[heard..].iter().map(|x| x.name.as_str()).collect();
        assert!(names.contains(&"grabCatch") && !names.contains(&"gunStrip"), "seed {s}: {names:?}");
    }
}
