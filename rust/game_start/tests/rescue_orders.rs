//! THE RESCUE'S ORDERS: F bangs and clears the hostage room; the menu has the pair otherwise; the last man covers each opening of the corridor.
mod doors_ask;
use doors_ask::*;
use game::orders::Who;
use game_start::play::{dist, Opts, Play};

fn stood(s: u32, set: f64) -> Play {
    let mut p = Play::mission("office.json", "?levelFile=office.json&rescue", &Opts { seed: s, squad: 2, ..Default::default() });
    p.sam_unhurt();
    p.rescue_quiet_floor();
    p.rescue_on(99.0, 99.0, set);
    p.run(0.3);
    p
}
fn walks(p: &mut Play, to: [f64; 2], mut each: impl FnMut(&mut Play)) {
    let mut at = p.sam_at();
    while dist(at, to) > 0.03 {
        let k = (1.55 / 60.0 / dist(at, to)).min(1.0);
        at = [at[0] + (to[0] - at[0]) * k, at[1] + (to[1] - at[1]) * k];
        p.sam_put(at);
        p.frame();
        each(p);
    }
}

/// THE RULE: one F on a hot start: the order itself starts the clock, so it is bang and clear in one word: the door tools take them at the door, one can out of the squad's: stack, boot, door, can, bang, entry, in order.
/// A FAILURE MEANS: F no longer does the whole of it, or the pair wait for a word nobody tells the player of.
#[test]
fn f_bangs_and_clears_the_hostage_room() {
    for s in SEEDS {
        let mut p = stood(s, 30.0);
        let cans = p.cans_left();
        p.tap("KeyF");
        p.until(46.0, |p| p.rescue_log().iter().any(|l| l.1.starts_with("enter1"))).unwrap_or_else(|| panic!("seed {s}: no entry: {:?}", p.rescue_log()));
        let log = p.rescue_log();
        let at = |w: &str| log.iter().position(|l| l.1.starts_with(w)).unwrap_or_else(|| panic!("seed {s}: no '{w}': {log:?}"));
        let want = ["order:bang", "stacked", "kick", "door-flung", "toss", "bang", "enter1"];
        for w in want.windows(2) { assert!(at(w[0]) < at(w[1]), "seed {s}: {} after {}: {log:?}", w[0], w[1]); }
        assert_eq!(p.cans_left(), cans - 1.0, "seed {s}: one can");
    }
}

/// THE RULE: F has the pair on the menu's own bang and clear; a menu order on the way to the stack takes them off it, and the can comes back, their lights off. A first order to one man starts the clock and the other falls in unasked. The
/// control is F with nothing after it: they stay on it.
/// A FAILURE MEANS: the menu has no hold on a pair in a drill, or a man is left at the arch.
#[test]
fn the_menu_has_the_pair_otherwise() {
    for s in SEEDS {
        let mut p = stood(s, 30.0);
        let cans = p.cans_left();
        p.tap("KeyF");
        p.run(2.0);
        let men = p.men();
        assert!(men.iter().all(|&m| p.doing(m).as_deref() == Some("bangClear")) && p.cans_left() == cans - 1.0, "seed {s}: the control: on the bang and clear: {:?}", men.iter().map(|&m| p.doing(m)).collect::<Vec<_>>());
        let sam = p.sam_at();
        assert_eq!(p.try_order("fallIn", sam, Who::All), None, "seed {s}");
        p.run(1.0);
        assert!(p.rescue_free() && p.rescue_log().iter().any(|l| l.1.starts_with("drill:called-off")) && p.cans_left() == cans, "seed {s}: taken off: {:?}", p.rescue_log());
        assert!(men.iter().all(|&m| p.doing(m).as_deref() == Some("fallIn")), "seed {s}");
        p.run(3.0);
        assert!(men.iter().all(|&m| !p.light_on(m)), "seed {s}: their lights");
        drop(p);

        let mut p = stood(s, 30.0);
        let (men, sam) = (p.men(), p.sam_at());
        assert_eq!(p.order_men("hold", sam, &men[..1]), None, "seed {s}");
        p.run(1.0);
        assert!(p.rescue_t0() >= 0.0 && p.doing(men[0]).as_deref() == Some("hold") && p.doing(men[1]).as_deref() == Some("fallIn"), "seed {s}: {:?} {:?}", p.doing(men[0]), p.doing(men[1]));
    }
}

/// THE RULE: Sam walks the corridor from the arch to the hostage room's door and gives no order: every opening he passes is taken, in the order he passes them, and called by the man taking it; nobody goes into a room he was only told to pass,
/// and both are on Sam again within 8 s of his stopping. The control is the way round by the lobby: nothing was walked past, and nobody is sent back.
/// A FAILURE MEANS: the corridor's openings go unwatched behind him, or his men wander into its rooms.
#[test]
fn the_last_man_covers_each_opening_of_the_corridor() {
    for s in SEEDS {
        let mut p = stood(s, 30.0);
        let men = p.men();
        let mut inside = 0;
        let boxes = [[4.12, 13.94, 4.12, 9.94], [14.12, 21.88, 4.12, 9.94], [12.2, 27.8, 12.3, 23.9]];   // (the conference room, the server room, the cubicles)
        let rooms = |q: [f64; 2]| boxes.iter().any(|b| q[0] > b[0] && q[0] < b[1] && q[1] > b[2] && q[1] < b[3]);
        let mut see = |p: &mut Play| { for &m in &men { if rooms(p.at(m)) { inside += 1; } } };
        walks(&mut p, [8.2, 11.2], &mut see);
        walks(&mut p, [24.6, 11.2], &mut see);
        let t = p.until(15.0, |p| p.rescue_jobs() == 0 && men.iter().all(|&m| dist(p.at(m), p.sam_at()) < 4.0));
        let takes: Vec<String> = p.rescue_log().into_iter().filter(|l| l.1.starts_with("flow:take:")).map(|l| l.1.split(':').nth(2).unwrap_or("").to_string()).collect();
        assert_eq!(takes, ["conference", "cubicles-w", "server", "cubicles-e"], "seed {s}");
        for w in ["glass left.", "opening right.", "door left."] { assert!(p.said_it(w), "seed {s}: '{w}' was not called"); }
        assert_eq!(inside, 0, "seed {s}: man-frames in a room he was told to pass");
        assert!(t.map_or(false, |x| x < 8.0), "seed {s}: he catches up: {t:?}");
        drop(p);

        let mut p = stood(s, 30.0);
        let sam = p.sam_at();
        p.order("fallIn", sam, Who::All);
        p.run(0.2);
        p.sam_put([9.0, 16.0]);
        p.run(1.0);
        for q in [[14.0, 16.0], [22.75, 14.0], [22.75, 11.2], [24.6, 11.2]] { walks(&mut p, q, |_| {}); }
        p.run(3.0);
        let flow: Vec<String> = p.rescue_log().into_iter().map(|l| l.1).filter(|w| w.starts_with("flow:take") || w.starts_with("flow:pass") || w.starts_with("flow:queue") || w.starts_with("flow:corridor")).collect();
        assert!(flow.is_empty(), "seed {s}: the control: nothing was walked past: {flow:?}");
    }
}

/// THE RULE: a man of the team who stands in the room with his order done and nobody in his eye pans across it, to both sides of its middle, the short way round. The room's middle lies straight along
/// the seam of the compass from where he stands, so the room's two sides read as nearly a whole turn apart unless the angle is brought back.
/// A FAILURE MEANS: a man on one side of the hostage room sweeps only half of it and leaves the other half dark.
#[test]
fn a_man_searching_the_room_pans_to_both_sides_of_its_middle() {
    let mut p = stood(7, 30.0);
    p.tap("KeyF");
    p.run(0.2);
    let man = p.men()[0];
    let room = game::rescue::rescue_bounds(&p.g).expect("a hostage room");
    let middle = [(room.x0 + room.x1) / 2.0, (room.z0 + room.z1) / 2.0];
    let at = [middle[0], room.z1 - 0.6];
    let seat = game::seats::here(&p.g);
    let a = game::orders::squad_orders_of(&seat, man).expect("on the bang and clear");
    let mid_yaw = (middle[0] - at[0]).atan2(middle[1] - at[1]);
    let (mut least, mut most) = (0.0f64, 0.0f64);
    for _ in 0..360 {
        // he has come in and stands at his post; the door's tools and the room's fight have let him go
        let c = p.g.gd[man].ch;
        game::character::pos_replace(&mut p.g, c, [at[0], 0.0, at[1]]);
        p.g.tab.orders_assignment[a].post = Some([at[0], 0.0, at[1]]);
        p.g.sets[p.g.seat].door_tools.ops.clear();
        let fight = p.g.fight.ops.clone();
        p.g.fight.ops = fight.into_iter().filter(|&o| p.g.tab.roomfight_op[o].gd != man).collect();
        p.frame();
        if p.g.tab.orders_assignment[a].search != Some(true) { continue; }
        let off = game::v3::wrap_angle(p.g.tab.orders_assignment[a].face.unwrap() - mid_yaw);
        (least, most) = (least.min(off), most.max(off));
    }
    assert!(least < -0.5 && most > 0.5, "he panned from {least:.2} to {most:.2} round the room's middle");
}
