//! A MAN WHO CANNOT SEE GOES NOWHERE (rust/game_rules/src/guards/p_07.rs, the cover branch of `alert`).
//!
//! What this holds. A sentry in a yard has rounds put past him until the suppression rule sends him to cover round the end of a wall. A flash
//! lands at his feet as he is sent. Before the mend he walked the 2.25 m to his spot with his eyes gone, because neither the rule nor the
//! fight's cover branch asked whether he could see. Now a dazzled or staggered man gets down where he is, whatever spot a rule names.
use game_start::play::{dist, Button, Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 3] = [1, 2, 3];

/// a lit yard: a sentry with a wall a pace behind him (the rounds that pass him strike it) and its end to get round, Sam eleven metres off
fn yard(seed: u32) -> (Play, game::ids::GuardId) {
    let mut s = Scene::new("pinned");
    s.room("yard", 6.0, 8.0, 30.0, 20.0);
    s.door("gate", Axis::X, 20.0, 8.0);
    s.wall_z(21.2, 12.5, 16.5).wall_x(17.5, 17.0, 19.5);
    s.bulb(10.0, 14.0).bulb(20.0, 14.0).bulb(26.0, 14.0);
    s.sentry("post", [20.0, 14.0], [10.0, 14.0]).sam(9.0, 14.0);
    let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    let gd = p.guard("post");
    p.sam_takes_his_rifle();
    p.run(0.3);
    (p, gd)
}

/// One frame of Sam's fire: a round every fifth of a second past the man's side, never at him.
fn frame_under_fire(p: &mut Play, gd: game::ids::GuardId, k: usize) {
    if k % 12 == 0 { let a = p.at(gd); p.point_at([a[0] + 0.9, a[1] + 1.2]); p.press(Button::Left); }
    if k % 12 == 3 { p.release(Button::Left); }
    p.frame();
}

/// Rounds past him until he is sent to cover: the spot, and the frame it was.
fn pinned_and_sent(p: &mut Play, gd: game::ids::GuardId, seed: u32) -> ([f64; 2], usize) {
    for k in 0..(15 * 60) {
        frame_under_fire(p, gd, k);
        assert!(p.up(gd), "seed {seed}: a round meant to pass him hit him ({})", p.roll());
        if let Some(spot) = p.sent_to_cover(gd) { return (spot, k + 1); }
    }
    panic!("seed {seed}: fifteen seconds of rounds past him and he was never sent to cover ({})", p.roll());
}

/// THE RULE: a man sent to cover who is flashed as he sets off stays where he is for as long as he cannot see.
/// A FAILURE MEANS: a rule's spot is being walked to by a man with no eyes (before the mend: 2.25 m, round the end of a wall).
#[test]
fn a_flashed_man_sent_to_cover_stays_where_he_is_until_he_can_see() {
    for s in SEEDS {
        let (mut p, gd) = yard(s);
        let (_spot, k0) = pinned_and_sent(&mut p, gd, s);
        let at = p.at(gd);
        p.flash_lands([at[0] + 0.3, at[1]]);
        let (mut from, mut far, mut frames) = (None, 0.0f64, 0);
        for k in k0..(k0 + 10 * 60) {
            frame_under_fire(&mut p, gd, k);
            if !p.blind(gd) { if from.is_some() { break; } else { continue; } }
            let here = p.at(gd);
            let f = *from.get_or_insert(here);
            far = far.max(dist(here, f));
            frames += 1;
        }
        assert!(frames > 60, "seed {s}: the flash at his feet did not take his eyes for a second ({frames} frames; {})", p.roll());
        assert!(far < 0.3, "seed {s}: he went {far:.2} m with his eyes gone ({})", p.roll());
    }
}

/// THE RULE: the time he cannot see is no part of the time his shift may take. A man sent to cover and flashed for longer than that goes to the
/// same spot once his eyes are back, however long they were gone.
/// A FAILURE MEANS: the shift's clock runs while he is kept from shifting, and he gives up his cover the moment he could use it.
#[test]
fn a_man_flashed_for_longer_than_his_shift_may_take_still_goes_to_his_cover() {
    for s in SEEDS {
        let (mut p, gd) = yard(s);
        let (spot, mut k) = pinned_and_sent(&mut p, gd, s);
        let t0 = p.time();
        let cap = p.g.k.suppression.displace_cap_secs;
        // a flash at his feet again each time his eyes are nearly back, until they have been gone for a second more than the shift may take
        let mut next = 0.0;
        while p.time() - t0 < cap + 1.0 {
            if p.time() >= next { let at = p.at(gd); p.flash_lands([at[0] + 0.3, at[1]]); next = p.time() + 2.0; }
            frame_under_fire(&mut p, gd, k); k += 1;
        }
        while p.blind(gd) { frame_under_fire(&mut p, gd, k); k += 1; }
        assert!(p.sent_to_cover(gd).map_or(false, |c| dist(c, spot) < 0.1), "seed {s}: his eyes came back {:.1} s after he was sent and his spot is gone ({})", p.time() - t0, p.roll());
        let mut got = false;
        for _ in 0..(4 * 60) { p.frame(); if dist(p.at(gd), spot) < 0.8 { got = true; break; } }   // (Sam holds his fire: the man is on the move now)
        assert!(got, "seed {s}: four seconds after his eyes came back he is {:.2} m from his spot ({})", dist(p.at(gd), spot), p.roll());
    }
}

/// THE CONTROL: with no flash the same man does go to his spot, so the rule above is not a man who never moves.
#[test]
fn the_same_man_with_his_eyes_goes_to_his_cover() {
    for s in SEEDS {
        let (mut p, gd) = yard(s);
        let (spot, k0) = pinned_and_sent(&mut p, gd, s);
        let from = p.at(gd);
        let mut got = false;
        for k in k0..(k0 + 6 * 60) { frame_under_fire(&mut p, gd, k); if dist(p.at(gd), spot) < 0.8 { got = true; break; } }
        assert!(got, "seed {s}: sent to ({:.1}, {:.1}) from ({:.1}, {:.1}) and six seconds later he is not there ({})", spot[0], spot[1], from[0], from[1], p.roll());
    }
}
