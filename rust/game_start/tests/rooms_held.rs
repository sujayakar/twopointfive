//! ROOMS, AND THE MEN IN THEM: a room flips to cleared by its own men; an order under fire stands; a man told to cover keeps his angle; one man shuts a door.
mod doors_ask;
use doors_ask::*;
use game::game::{DownBy, GuardState};
use game::ids::*;
use game::orders::Who;
use game::roomclear::RoomClearEnd;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};

/// the west room with two defenders, the east with one; the squad in the yard
fn three_men(seed: u32, squad: u32) -> Play {
    let mut s = Scene::new("three-men");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("west-yard", Axis::X, 16.0, 13.4).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    s.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    s.team([14.0, 23.0], NORTH, &[[13.0, 21.0], [15.0, 21.0], [13.0, 22.0], [15.0, 22.0]]);
    s.defender("west", [11.25, 11.25], "west-yard").defender("west", [16.5, 11.25], "west-yard").defender("east", [24.25, 12.25], "east-yard");
    let mut p = Play::scene(&s, &Opts { seed, squad, ..Default::default() });
    p.sam_unhurt();
    p.run(0.5);
    p
}

/// THE RULE: a room flips to cleared when its own men are down and not before: one of two down, still held; the other room's man struck out cold, that room clear, and it alone. A living man carried into a cleared room holds it again. The control is each step's other room.
/// A FAILURE MEANS: the objective ticks a room with a man in it, or never ticks one that is empty.
#[test]
fn a_room_flips_to_cleared_when_its_own_men_are_down() {
    for s in SEEDS {
        let mut p = three_men(s, 2);
        let (west, east) = (p.room_men("west"), p.room_men("east"));
        assert_eq!((west.len(), east.len()), (2, 1), "seed {s}: the deal");
        p.falls(west[0], DownBy::Player);
        p.run(2.0 / 60.0);
        assert!(!p.room_cleared("west") && !p.room_cleared("east"), "seed {s}: one of two down: still held");
        p.falls_as(east[0], game::game::DownKind::Struck, true, DownBy::Player);
        p.run(2.0 / 60.0);
        assert!(p.room_cleared("east") && !p.room_cleared("west"), "seed {s}: out cold is down, and only that room is clear");
        assert_eq!(p.room_clear_end(), None, "seed {s}: nothing has ended");
        p.man_stands(west[1], [22.0 + nudge(s, 80), 13.0]);
        p.run(2.0 / 60.0);
        assert!(!p.room_cleared("east"), "seed {s}: a living man in it holds it again");
        p.falls(west[1], DownBy::Player);
        p.run(2.0 / 60.0);
        assert!(p.room_cleared("east") && p.room_cleared("west"), "seed {s}: down, both are clear");
    }
}

/// THE RULE: 'taken' fires on the last defender and not one man sooner, once; 'wiped' when Sam goes down, with the pair still up. The control is the run before the last man falls.
/// A FAILURE MEANS: a mission ends early, or twice, or carries on with the player down.
#[test]
fn the_structure_is_taken_on_the_last_man_and_wiped_when_sam_goes_down() {
    for s in SEEDS {
        let mut p = three_men(s, 2);
        let (west, east) = (p.room_men("west"), p.room_men("east"));
        p.falls(east[0], DownBy::Player);
        p.falls(west[0], DownBy::Player);
        p.run(0.1);
        assert_eq!(p.room_clear_end(), None, "seed {s}: one man left");
        p.falls(west[1], DownBy::Player);
        p.run(0.1);
        assert_eq!(p.room_clear_end().map(|e| e.0), Some(RoomClearEnd::Taken), "seed {s}");
        let at = p.room_clear_end().unwrap().1;
        p.run(2.0);
        assert_eq!(p.room_clear_end().map(|e| e.1), Some(at), "seed {s}: it ends once");
        drop(p);

        let mut p = three_men(s, 2);
        p.sam_hurtable();
        p.seat_goes_down(0);
        p.run(2.0 / 60.0);
        assert_eq!(p.room_clear_end().map(|e| e.0), Some(RoomClearEnd::Wiped), "seed {s}");
        assert_eq!(p.men().len(), 2, "seed {s}: the pair is still up");
    }
}

/// two men on the yard, a man of the other side stood where `at` says, blind: Blue One told to cover a way 52 degrees off him, and Red One, if `red`, told to cover him. Frames: how far Blue's gun went off the way he was given, frames on the man, and frames Red's gun was on him.
fn cover_stage(s: u32, red: bool) -> (f64, i32, i32, i32) {
    let mut p = three_men(s, 4);
    let men = p.men();
    let (mate, blue) = (men[0], men[1]);   // (the fight deals its men in turn: a mate ahead of him in the list has the man first when both see him on one frame)
    let foe = p.room_men("east")[0];
    for m in p.room_men("west") { p.falls(m, DownBy::Player); }
    let (a, b) = ([12.0 + nudge(s, 81), 24.0], [13.2 + nudge(s, 81), 24.8]);
    for (i, &m) in men.iter().enumerate().skip(2) { p.man_stands(m, [6.0 + i as f64 * 1.2, 26.0]); p.blinded(m); }
    let way = [24.0, 27.0];
    let dir = yaw_to(a, way);
    let side = dir + 52f64.to_radians();
    let man = [a[0] + side.sin() * 9.0, a[1] + side.cos() * 9.0];
    let away = yaw_to(man, a) + std::f64::consts::PI * 0.6;
    for _ in 0..20 { p.puppet(foe, [-30.0, 3.0], 0.0); p.man_stands(blue, a); p.man_stands(mate, b); p.frame(); }
    assert_eq!(p.order_men("cover", way, &[blue]), None, "seed {s}");
    assert_eq!(p.order_men("cover", man, &[mate]), None, "seed {s}");
    for _ in 0..90 { p.puppet(foe, [-30.0, 3.0], 0.0); if !red { p.blinded(mate); } for &m in men.iter().skip(2) { p.blinded(m); } p.frame(); }
    let settled = off_by(p.gun_yaw(blue), dir).to_degrees();
    let (mut most, mut on, mut red_on, mut frames) = (settled, 0, 0, 0);
    for _ in 0..240 {
        p.puppet(foe, man, away);
        if !red { p.blinded(mate); }
        for &m in men.iter().skip(2) { p.blinded(m); }
        p.frame();
        frames += 1;
        most = most.max(off_by(p.gun_yaw(blue), dir).to_degrees());
        if p.gun_on(blue) == Some(foe) { on += 1; }
        if p.gun_on(mate) == Some(foe) { red_on += 1; }
    }
    (most, on, red_on, frames)
}

/// THE RULE: a man told to cover keeps his angle: a man 52 degrees off his way, whom a mate is already on, he sees and leaves to the mate: his gun stays in his lane. The control is the same man with no mate on him: he is his, and the gun goes.
/// A FAILURE MEANS: the whole team turns on the first man it sees and the door behind it goes unwatched.
#[test]
fn a_man_told_to_cover_keeps_his_angle() {
    for s in SEEDS {
        let (most, on, red_on, frames) = cover_stage(s, true);
        assert_eq!(on, 0, "seed {s}: he took the man Red One was on");
        assert!(most < 15.0, "seed {s}: his gun went {most:.0} degrees off his way");
        assert!(red_on * 2 > frames, "seed {s}: the staging: the mate is on the man: {red_on} of {frames}");
        let (most, on, _, _) = cover_stage(s, false);
        assert!(on > 0 && most > 40.0, "seed {s}: the control: no mate on him: {on} frames on him, gun {most:.0} degrees off");
    }
}

/// THE RULE: an order under an engagement is never written over: while a man's muzzle is his own, what his order wrote on him stands as ordered, and the gun comes up through the fight's overlay. The control is the frames before he saw the man: no overlay and no round.
/// A FAILURE MEANS: a man let off an order by a fight comes back with another order's carriage, or has forgotten his own.
#[test]
fn an_order_under_fire_is_never_written_over() {
    for s in SEEDS {
        let mut p = three_men(s, 2);
        let men = p.men();
        let m = men[0];
        let foe = p.room_men("east")[0];
        for w in p.room_men("west") { p.falls(w, DownBy::Player); }
        p.blinded(men[1]);
        let spot = [13.0 + nudge(s, 82), 24.0];
        let at = [spot[0] + 5.0, spot[1]];
        p.order_floor("move", spot, Who::Num(0.0));
        p.until(10.0, |p| p.arrived(m)).unwrap_or_else(|| panic!("seed {s}: never there"));
        assert_eq!(p.order_men("cover", at, &[m]), None, "seed {s}");
        p.run(1.5);
        let before = p.script_says(m).expect("his order writes his script");
        assert!(!before.3, "seed {s}: the control: no overlay before he has seen anybody");
        let r0 = p.rounds_fired(m);
        let mut during = None;
        for _ in 0..360 {
            if !p.up(foe) { break; }
            p.pin_man(foe, at, 0.0, GuardState::Alert, 1.0);
            p.blinded(foe);
            p.blinded(men[1]);
            p.frame();
            if let Some(x) = p.script_says(m) { if x.3 && during.is_none() { during = Some((x, p.arms_shown(m))); } }
        }
        let (x, arms) = during.unwrap_or_else(|| panic!("seed {s}: he never engaged: {}", p.why(m)));
        assert_eq!((x.0, x.1, x.2), (before.0, before.1, before.2), "seed {s}: the order on his script is exactly as ordered");
        assert_eq!(arms, game::character::UpperMode::Aim, "seed {s}: the gun comes up through the overlay");
        assert!(p.rounds_fired(m) > r0, "seed {s}: he fired");
        assert_eq!(p.doing(m).as_deref(), Some("cover"), "seed {s}: his order stands");
    }
}

/// the squad holding at four distances from the west door, its leaf stood open. The door, its middle, the men.
fn open_door(s: u32, locked: bool) -> (Play, DoorId, [f64; 2], Vec<GuardId>) {
    let (mut sc, at) = yard(s);
    if locked { sc.locked("west-yard"); }
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.run(0.2);
    let keep = p.defender_at([24.25, 12.25]);
    p.drop_all_but(&[keep]);
    p.pin_man(keep, [24.25, 12.25], 0.0, GuardState::Patrol, 0.0);
    let d = p.door("west-yard");
    let men = p.men();
    for (i, &m) in men.iter().enumerate() { p.man_stands(m, [at[0] + 2.0 + i as f64 * 1.5, at[1] + 2.2]); }
    p.order("hold", p.sam_at(), Who::All);
    p.run(0.3);
    p.leaf_put_open(d, 1.35);
    p.leaf_stays(d);
    p.run(0.1);
    (p, d, at, men)
}

/// THE RULE: one man shuts a door on order, the nearest by his walk; the others keep their orders; he never goes above the team's pace, pulls the leaf with him clear of the floor it crosses and within reach of its handle; it latches, he says so once. The control: with no order it stands where it was left.
/// A FAILURE MEANS: the whole team goes for the door, or the leaf comes round through the man.
#[test]
fn one_man_shuts_a_door_on_order_the_nearest_by_his_walk() {
    for s in SEEDS {
        let (mut p, d, at, men) = open_door(s, false);
        let a0 = p.leaf_angle(d);
        p.run(1.0);
        assert!((p.leaf_angle(d) - a0).abs() < 1e-6, "seed {s}: the control: with no order it stays");
        assert_eq!(p.try_order("shutDoor", at, Who::All), None, "seed {s}");
        let man = men.iter().copied().find(|&m| p.doing(m).as_deref() == Some("shutDoor")).unwrap_or_else(|| panic!("seed {s}: nobody took it"));
        assert_eq!(man, *men.iter().min_by(|a, b| dist(p.at(**a), at).partial_cmp(&dist(p.at(**b), at)).unwrap()).unwrap(), "seed {s}: not the nearest");
        let (mut top, mut moved_at) = (0.0f64, None);
        let t = p.until(40.0, |p| {
            top = top.max(p.speed(man));
            if moved_at.is_none() && p.leaf_angle(d).abs() < a0.abs() - 1e-4 { moved_at = Some(p.at(man)); }
            for &o in &men { if o != man { assert_eq!(p.doing(o).as_deref(), Some("hold"), "seed {s}: another man let go"); } }
            p.latched(d) && p.doing(man).as_deref() != Some("shutDoor")
        }).unwrap_or_else(|| panic!("seed {s}: never shut: {}", p.roll()));
        let at_pull = moved_at.expect("the leaf moved");
        assert!(p.from_handle(d, at_pull) <= 1.55, "seed {s}: {:.2} m from the handle", p.from_handle(d, at_pull));
        assert!(!p.in_sweep(d, at_pull) || p.face(d, at_pull) != p.face(d, [at[0], at[1] + 1.0]) || p.pull_face(d, at_pull) == false, "seed {s}: he stood on the floor the leaf crosses: {at_pull:?}");
        assert!(top > 0.3 && top <= p.team_pace() + 0.05, "seed {s}: he went {top:.2} m/s, {t:.1} s");
        p.run(1.0);
        assert_eq!(p.lines(man).iter().filter(|l| l.contains("door's shut") || l.contains("it's shut")).count(), 1, "seed {s}: {:?}", p.lines(man));
        assert_eq!(p.doing(man).as_deref(), Some("hold"), "seed {s}: he holds by the door");
    }
}

/// THE RULE: a keyed door shut on order is locked as it latches: its lock bites again for a man with no key and not for a man with it. The control is the plain door: shut, it locks nothing.
/// A FAILURE MEANS: a door opened with a key stays open to anyone once it is shut.
#[test]
fn a_keyed_door_shut_on_order_locks_again() {
    for s in SEEDS {
        for keyed in [true, false] {
            let (mut p, d, at, _) = open_door(s, keyed);
            assert_eq!(p.try_order("shutDoor", at, Who::All), None, "seed {s}");
            p.until(40.0, |p| p.latched(d)).unwrap_or_else(|| panic!("seed {s}: never shut"));
            let from = p.on_face(d, p.door_w(d) / 2.0, p.key_face(d) * 1.0);
            let (no_key, with_key) = p.locked_out(d, from);
            assert_eq!((no_key, with_key), (keyed, false), "seed {s}, keyed {keyed}");
        }
    }
}
