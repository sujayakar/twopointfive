//! THE WORLD'S EDGE, A CELL NO PLAN CAN LEAVE, AND A MOVE CUT OFF: he never leaves the slab, a man is never left standing where he was stranded, and a teleport ends what he was doing.
mod kit_ask;
use game_start::play::{dist, Opts, Play};
use kit_ask::KitAsk;

const R_BODY: f64 = 0.42;
const FAR: [f64; 2] = [5.0, 15.0];
const DEST: [f64; 2] = [6.5, 19.0];
const NOOK: [f64; 2] = [22.66, 21.90];

fn inside(at: [f64; 2]) -> bool { at[0] >= R_BODY - 1e-3 && at[0] <= 40.0 - R_BODY + 1e-3 && at[1] >= R_BODY - 1e-3 && at[1] <= 28.0 - R_BODY + 1e-3 }

fn office(seed: u32) -> Play {
    let mut p = Play::level("office.json", false, &Opts { seed, ..Default::default() });
    p.sam_a_ghost();
    p.sam_unhurt();
    p.run(0.2);
    p
}

/// THE RULE: walking or sprinting straight off any side or corner of the 40 by 28 metre slab he stays on it, however long the keys are held.
/// THE CONTROL: he gets to the edge: the run ends within two body widths of the rim, not stopped short by something else.
/// A FAILURE MEANS: a player falls off the world or stands past its rim.
#[test]
fn walking_or_sprinting_off_the_slab_he_stays_on_it() {
    let walks: [([f64; 2], [f64; 2], bool); 5] = [
        ([2.2, 14.0], [-6.0, 14.0], false),
        ([38.0, 18.0], [48.0, 18.5], true),
        ([20.0, 2.0], [20.0, -6.0], false),
        ([26.0, 26.3], [26.5, 36.0], true),
        ([2.0, 26.0], [-5.0, 33.0], false),
    ];
    let mut p = office(4);
    for (from, goal, sprint) in walks {
        p.sam_put(from);
        p.frame();
        p.sam_walks_to(goal, false);
        if sprint { p.key_down("ShiftLeft"); }
        let off = p.always(5.0, |p| inside(p.sam_at()) && p.sam_at()[0].is_finite());
        p.key_up("ShiftLeft");
        p.sam_stops();
        assert!(off.is_none(), "from {from:?} toward {goal:?}: off the slab at {:?}", p.sam_at());
        let at = p.sam_at();
        let to_rim = at[0].min(40.0 - at[0]).min(at[1]).min(28.0 - at[1]);
        assert!(to_rim < 3.0 * R_BODY, "control: from {from:?} toward {goal:?} he stopped {to_rim:.2} m short of the rim, at {at:?}");
    }
}

/// THE RULE: a teleport past the rim, however far, sets him down on the slab, not inside anything and on a free cell.
/// THE CONTROL: a teleport to an open place on it sets him down there.
/// A FAILURE MEANS: a debug jump or a script's set-down leaves him past the rim or in a wall.
#[test]
fn a_teleport_past_the_rim_sets_him_down_on_it() {
    let mut p = office(4);
    for to in [[-3.0, 14.0], [44.0, 30.0], [20.0, -2.0], [41.5, 14.0], [1e6, -1e6]] {
        p.sam_put(to);
        assert!(inside(p.sam_at()), "teleport to {to:?}: at {:?}", p.sam_at());
        p.frame();
        let at = p.sam_at();
        assert!(inside(at) && p.sam_in_a_wall() < 0.01 && !p.cell_blocked(at), "teleport to {to:?}: at {at:?}, {:.2} m in a wall, cell blocked {}", p.sam_in_a_wall(), p.cell_blocked(at));
    }
    p.sam_put(DEST);
    assert!(dist(p.sam_at(), DEST) < 0.01, "control: at {:?}", p.sam_at());
}

/// THE RULE: a man on a nav cell no plan can leave, with a fix five metres off, is on a free cell and walking inside two seconds and nearer his fix by five, on his own feet.
/// THE CONTROL: the same man the same distance from the same fix on a free cell has no such step to take: he walks it, and goes further.
/// A FAILURE MEANS: a man stands in a corner for the rest of the mission while every plan he makes comes back empty.
#[test]
fn a_man_on_a_cell_no_plan_can_leave_walks_off_it() {
    const FIX: [f64; 2] = [26.4, 23.0];
    for seed in [101, 102, 103] {
        let mut p = office(seed);
        let men = p.men_on_the_floor();
        let m = men[0];
        for &o in &men[1..] { p.man_posted(o); p.g.gd[o].dazzled_until = 1e9; }
        p.sam_put(FAR);
        p.man_let_off_his_post(m);
        p.man_stands(m, NOOK);
        assert!(p.stranded(NOOK), "seed {seed}: the staging: the nook is no longer stranded, so this says nothing");
        p.man_goes_to_look(m, FIX);
        let d0 = dist(p.at(m), FIX);
        let (mut last, mut free, mut longest) = (p.at(m), None, 0.0f64);
        for i in 0..(5 * 60) {
            p.frame();
            let at = p.at(m);
            longest = longest.max(dist(at, last));
            last = at;
            if free.is_none() && !p.cell_blocked(at) { free = Some(i); }
        }
        let free = free.unwrap_or_else(|| panic!("seed {seed}: never on a free cell, at {last:?}"));
        assert!((free as f64) / 60.0 <= 2.0, "seed {seed}: on a free cell after {:.2} s", free as f64 / 60.0);
        assert!(d0 - dist(last, FIX) >= 1.5, "seed {seed}: only {:.2} m nearer his fix in 5 s", d0 - dist(last, FIX));
        assert!(longest <= 0.3, "seed {seed}: a one-frame move of {longest:.2} m: snapped, not walked");
        drop(p);

        let mut p = office(seed);
        let men = p.men_on_the_floor();
        let m = men[0];
        for &o in &men[1..] { p.man_posted(o); p.g.gd[o].dazzled_until = 1e9; }
        p.sam_put(FAR);
        p.man_let_off_his_post(m);
        let open = [22.0, 23.0];
        assert!(!p.stranded(open), "seed {seed}, control: the open floor is stranded");
        p.man_stands(m, open);
        p.man_goes_to_look(m, FIX);
        let d1 = dist(p.at(m), FIX);
        p.run(5.0);
        assert!(d1 - dist(p.at(m), FIX) >= 1.5, "seed {seed}, control: the same walk from open floor made {:.2} m", d1 - dist(p.at(m), FIX));
    }
}

/// THE RULE: teleported away mid-boot, the boot is over; he is not reeled back, the door is whole, and a fresh boot at it lands.
/// THE CONTROL: left alone, the same boot breaks the lock.
/// A FAILURE MEANS: a door is kicked in from the far side of the building, or his leg eats the next kick.
#[test]
fn a_teleport_ends_a_boot() {
    for frames in [8, 21] {
        let mut p = office(71);
        let d = p.door("storage");
        let spot = p.kick_spot(d, [32.6, 10.9]);
        p.sam_put([spot[0] + 0.15, spot[1] + 0.1]);
        p.frame();
        p.sam_starts_kick(d);
        for _ in 0..frames { p.frame(); }
        assert!(p.sam_in_a_move().0, "{frames} frames in: the staging: no boot");
        p.sam_put(DEST);
        assert!(p.always(1.5, |p| dist(p.sam_at(), DEST) <= 0.3).is_none(), "{frames} frames in: reeled off where he was put, at {:?}", p.sam_at());
        let (locked, broken, _) = p.lock_state(d);
        assert!(!p.sam_in_a_move().0 && locked && !broken, "{frames} frames in: boot {} · locked {locked} · broken {broken}", p.sam_in_a_move().0);
        p.sam_put(spot);
        p.frame();
        p.sam_starts_kick(d);
        p.until(1.5, |p| p.lock_state(d).1).unwrap_or_else(|| panic!("{frames} frames in: a fresh boot after the cancelled one never landed"));
        drop(p);

        let mut p = office(71);
        let d = p.door("storage");
        let spot = p.kick_spot(d, [32.6, 10.9]);
        p.sam_put([spot[0] + 0.15, spot[1] + 0.1]);
        p.frame();
        p.sam_starts_kick(d);
        p.until(1.5, |p| p.lock_state(d).1).expect("control: left alone the boot lands");
    }
}

/// THE RULE: teleported away with the tools in the lock and the key held, the pick is over; the lock turns no further from there and he can walk off.
/// THE CONTROL: left at the door, the same pick goes on turning.
/// A FAILURE MEANS: a lock opens from across the building, or he is rooted to a floor with no door on it.
#[test]
fn a_teleport_ends_a_pick() {
    let mut p = office(72);
    let d = p.door("server");
    let spot = p.work_spot(d, [17.6, 10.9]);
    p.sam_put([spot[0] + 0.2, spot[1] + 0.15]);
    p.frame();
    p.key_down("Space");
    p.frame();
    p.sam_starts_pick(d);
    p.run(2.5);
    let (_, _, pick0) = p.lock_state(d);
    assert!(p.sam_in_a_move().1 && pick0 > 0.0, "staging: never got to work the lock (pick {pick0:.2})");
    p.run(0.3);
    assert!(p.lock_state(d).2 > pick0, "control: at the door it goes on turning");
    let pick1 = p.lock_state(d).2;
    p.sam_put(DEST);
    assert!(p.always(1.5, |p| dist(p.sam_at(), DEST) <= 0.3).is_none(), "reeled off where he was put, at {:?}", p.sam_at());
    let (locked, _, pick2) = p.lock_state(d);
    assert!(!p.sam_in_a_move().1 && locked && pick2 <= pick1 + 1e-6, "the lock kept turning from the lobby: {pick1:.2} to {pick2:.2}, locked {locked}");
    p.key_up("Space");
    p.frame();
    p.sam_low(false);
    p.sam_walks_to([DEST[0] + 2.0, DEST[1]], false);
    p.run(1.5);
    p.sam_stops();
    assert!(dist(p.sam_at(), DEST) >= 1.5, "could not walk off afterwards");
}

/// THE RULE: teleported away before the blow lands, the blow is over: the man lives and walks on, and he is not pulled back to him.
/// THE CONTROL: left alone, the same blow drops the man.
/// A FAILURE MEANS: a man is strangled from rooms away.
#[test]
fn a_teleport_ends_a_blow() {
    let stage = |seed: u32| -> (Play, game::ids::GuardId) {
        let mut p = office(seed);
        let m = p.men_on_the_floor()[1];
        p.man_let_off_his_post(m);
        p.run(1.0);
        let (at, y) = (p.at(m), p.facing(m));
        p.sam_put([at[0] - y.sin() * 0.8, at[1] - y.cos() * 0.8]);
        p.sam_low(true);
        p.frame();
        p.sam_starts_takedown(m);
        p.run(0.1);
        assert!(p.sam_in_a_move().2, "seed {seed}: the staging: the blow never started");
        (p, m)
    };
    let (mut p, m) = stage(73);
    p.sam_put(DEST);
    let at0 = p.at(m);
    assert!(p.always(4.0, |p| dist(p.sam_at(), DEST) <= 0.3).is_none(), "pulled off where he was put, at {:?}", p.sam_at());
    assert!(!p.sam_in_a_move().2 && p.up(m), "blow on {} · up {}", p.sam_in_a_move().2, p.up(m));
    assert!(dist(p.at(m), at0) >= 0.5, "the man never moved again");
    drop(p);
    let (mut p, m) = stage(73);
    p.until(4.0, |p| !p.up(m)).expect("control: left alone the blow takes him");
}

/// THE RULE: teleported away with a body in his hands, the body stays where it was let go and his hands are free where he is.
/// THE CONTROL: before the teleport the body has come up off the floor into his hands.
/// A FAILURE MEANS: a body is dragged through the walls behind him, or his hands stay full.
#[test]
fn a_teleport_lets_go_of_a_body() {
    let mut p = office(75);
    let m = p.men_on_the_floor()[2];
    p.man_stands(m, [22.0, 17.6]);
    p.man_posted(m);
    p.run(0.2);
    p.falls(m, game::game::DownBy::Player);
    p.run(1.5);
    let h0 = p.head(m);
    p.sam_put([h0[0] + 0.7, h0[2] + 0.4]);
    p.frame();
    p.sam_starts_drag(m);
    p.run(2.0);
    let h1 = p.head(m);
    assert!(h1[1] - h0[1] > 0.5, "control: in his hands the body comes up off the floor: {:.2} to {:.2}", h0[1], h1[1]);
    p.sam_put(DEST);
    assert!(p.always(2.0, |p| dist(p.sam_at(), DEST) <= 0.3).is_none(), "pulled off where he was put, at {:?}", p.sam_at());
    let h2 = p.head(m);
    assert!(!p.sam_in_a_move().3, "his hands are still full");
    assert!(dist([h2[0], h2[2]], [h1[0], h1[2]]) <= 1.0, "the body went {:.2} m after him", dist([h2[0], h2[2]], [h1[0], h1[2]]));
    p.sam_low(false);
    p.sam_walks_to([DEST[0] + 2.0, DEST[1]], false);
    p.run(1.5);
    assert!(dist(p.sam_at(), DEST) >= 1.5, "could not walk off afterwards");
}
