//! SHIFT WITH THE SQUAD'S KEY GIVES WHAT THE KEY GIVES (rust/game_rules/src/autopilot `quick`, planmode `plan_goal`; the maintainer, 11-08: "i think shift Q on a
//! point should just move the squad there, should match what regular Q does. you should need to open the radial menu to get covering a point").
//!
//! One function answers what the key means: a plain tap gives it to everyone, Shift gives the same to the part `goal_men` picks. Every press here goes through
//! the keys as a hand presses them, on the cursor. Before this rule a Shift tap on a spot had the part cover it and the plain key had everyone move there; run on
//! that rule, `every_kind_of_target_gets_the_same_order` fails at 'a bare spot'.
use game::ids::GuardId;
use game::orders::{OrderTarget, Who};
use game_start::play::{Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 3] = [1, 2, 3];
const NORTH: f64 = std::f64::consts::PI;
const DOOR: [f64; 2] = [13.4, 16.0];
const SECOND_DOOR: [f64; 2] = [21.4, 16.0];
const SPOT: [f64; 2] = [20.0, 22.0];

/// the team in a yard, two rooms with a defender each
fn yard(seed: u32) -> Play {
    let mut s = Scene::new("shift-yard");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("west-yard", Axis::X, 16.0, DOOR[0]).door("east-yard", Axis::X, 16.0, SECOND_DOOR[0]).door("between", Axis::Z, 18.0, 12.4);
    s.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    s.team([14.0, 21.0], NORTH, &[[13.0, 22.0], [15.0, 22.0], [13.0, 23.0], [15.0, 23.0]]);
    s.defender("west", [12.25, 12.25], "west-yard").defender("east", [24.25, 12.25], "east-yard");
    let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.run(1.0);
    p
}
/// Sam and his four on the office's aisle, with what a level has that a bare yard has not: a wall that takes a charge
fn office(seed: u32) -> Play {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.run(0.5);
    p
}

fn men(p: &Play) -> Vec<GuardId> { game::orders::squad_orders_men(&p.g, &game::seats::here(&p.g), Some(game::orders::SquadOrdersMenSel::All), None) }
/// the order a man is on: None with no post (nothing to do, or in the file behind Sam)
fn on(p: &Play, m: GuardId) -> Option<String> {
    let a = game::orders::squad_orders_of(&p.g, &game::seats::here(&p.g), m)?;
    let row = &p.g.tab.orders_assignment[a];
    row.post.map(|_| row.order.clone())
}
fn on_all(p: &Play) -> Vec<Option<String>> { men(p).into_iter().map(|m| on(p, m)).collect() }
fn the(p: &Play, l: &[GuardId], id: &str) -> Vec<GuardId> { l.iter().copied().filter(|&m| on(p, m).as_deref() == Some(id)).collect() }
/// what the key would read under the cursor, as the press takes it
fn under(p: &Play) -> OrderTarget {
    let now = game::view::host_now(&p.g, p.host.input, p.host.cam, p.host.canvas);
    game::orders::hover(&p.g, &game::seats::seat(&p.g, p.g.seats[0]), &now).expect("the key is live")
}
/// the key tapped, Shift held or not, as a hand presses it: the row it logged, if it logged one
fn tap(p: &mut Play, shift: bool) -> Option<String> {
    let (key, n) = (p.g.k.cmdring.key.clone(), p.g.orders.log.len());
    if shift { p.key_down("ShiftLeft"); p.frame(); }
    p.key_down(&key);
    p.frame();
    p.key_up(&key);
    p.frame(); p.frame();
    if shift { p.key_up("ShiftLeft"); p.frame(); }
    (p.g.orders.log.len() > n).then(|| p.g.orders.log.last().unwrap().order.clone())
}
/// Shift and the key kept down until the ring is up
fn wheel(p: &mut Play) {
    let key = p.g.k.cmdring.key.clone();
    p.key_down("ShiftLeft");
    p.frame();
    p.key_down(&key);
    p.until(2.0, |p| p.view().ring.is_some()).unwrap_or_else(|| panic!("no ring two seconds after Shift and the key went down"));
}
/// a wedge of the ring given by its digit, and every key let go: the row it logged
fn wedge(p: &mut Play, key: f64) -> Option<String> {
    let (k, n, d) = (p.g.k.cmdring.key.clone(), p.g.orders.log.len(), format!("Digit{}", key as i32));
    p.key_down(&d);
    p.frame();
    p.key_up(&d);
    p.frame();
    p.key_up(&k);
    p.key_up("ShiftLeft");
    p.frame(); p.frame();
    (p.g.orders.log.len() > n).then(|| p.g.orders.log.last().unwrap().order.clone())
}
/// the whole pair of `all` whose mean distance to `at` is the less
fn nearer_pair(p: &Play, all: &[GuardId], at: [f64; 2]) -> Vec<GuardId> {
    let mut parts: Vec<Vec<GuardId>> = Vec::new();
    for id in game::orders::squad_orders_pairs(&p.g, &game::seats::here(&p.g)) {
        parts.push(all.iter().copied().filter(|&m| game::orders::squad_orders_pair_of(&p.g, m) == Some(id)).collect());
    }
    let far = |l: &Vec<GuardId>| l.iter().map(|&m| game_start::play::dist(p.at(m), at)).sum::<f64>() / l.len() as f64;
    parts.sort_by(|a, b| far(a).partial_cmp(&far(b)).unwrap());
    parts.remove(0)
}
fn sorted(mut l: Vec<GuardId>) -> Vec<GuardId> { l.sort_by_key(|m| m.0); l }

/// what a press came to: the row it logged and who the row names, whether a man answered, and whether anybody's order changed
struct Press { order: Option<String>, who: Vec<String>, said: bool, moved: bool }
/// the same stage twice, the key on the same place, without Shift and with it, and that the cursor read what `is` says
fn both(what: &str, seed: u32, stage: &dyn Fn(u32) -> Play, cursor: &dyn Fn(&Play) -> [f64; 3], is: &dyn Fn(&OrderTarget) -> bool) -> (Press, Press) {
    let press = |shift: bool| {
        let mut p = stage(seed);
        // (a hand moves the cursor about until the panel reads the thing: a man stood on the door's mark is over part of its picture)
        let at = cursor(&p);
        let mut t = None;
        'looking: for y in [at[1], 1.0, 1.8] { for dx in [0.0, 0.45, -0.45, 0.2, -0.2] { for dz in [0.0, -0.3, -0.6] {
            p.cursor_on([at[0] + dx, y, at[2] + dz]);
            p.run(0.05);
            let u = under(&p);
            if is(&u) { t = Some(u); break 'looking; }
        } } }
        assert!(t.is_some(), "seed {seed}: the staging is wrong: the cursor around {what} never reads it: {:?}", under(&p));
        p.run(0.1);
        let (said0, was) = (p.said.len(), on_all(&p));
        let order = tap(&mut p, shift);
        let who = if order.is_some() { p.g.orders.log.last().unwrap().who.clone() } else { Vec::new() };
        Press { order, who, said: p.said.len() > said0, moved: on_all(&p) != was }
    };
    (press(false), press(true))
}
const fn on_floor(at: [f64; 2]) -> [f64; 3] { [at[0], 0.05, at[1]] }
fn names(p: &Play, l: &[GuardId]) -> Vec<String> { let mut n: Vec<String> = l.iter().map(|&m| p.g.gd[m].callsign.clone()).collect(); n.sort(); n }
fn sort_names(mut n: Vec<String>) -> Vec<String> { n.sort(); n }

/// THE RULE: for every kind of target, the key with Shift logs the order the key logs without it (a part takes it, the same order), and each gives what that target gives.
/// A FAILURE MEANS: two functions answer what the key means again. Before: 'cover' with Shift on a spot and 'move' without it.
#[test]
fn every_kind_of_target_gets_the_same_order() {
    for s in SEEDS {
        let is_door = |t: &OrderTarget| matches!(t, OrderTarget::Door(_));
        let is_spot = |t: &OrderTarget| matches!(t, OrderTarget::Floor(f) if f.thing.is_none() && f.wall.is_none());
        let plain = |seed: u32| yard(seed);
        let stacked = |seed: u32| { let mut p = yard(seed); p.order("stack", DOOR, Who::All); p.until(40.0, |p| { let l = men(p); l.iter().all(|&m| game::orders::squad_orders_of(&p.g, &game::seats::here(&p.g), m).is_some_and(|a| p.g.tab.orders_assignment[a].arrived)) }).expect("the stack stands"); p };
        let charged = |seed: u32| { let mut p = yard(seed); p.order("charge", DOOR, Who::All); p.until(60.0, |p| game::doortools::door_tools_charge_on(&p.g, &game::seats::here(&p.g), door_at(p, DOOR)).is_some()).expect("the charge is set"); p.run(1.0); p };
        let cases: [(&str, &dyn Fn(u32) -> Play, &dyn Fn(&Play) -> [f64; 3], &dyn Fn(&OrderTarget) -> bool, Option<&str>); 4] = [
            ("a door nobody is on", &plain, &|_| on_floor(DOOR), &is_door, Some("stack")),
            ("a door with a charge set", &charged, &|_| on_floor(DOOR), &is_door, Some("go")),
            ("a bare spot", &plain, &|_| on_floor(SPOT), &is_spot, Some("move")),
            ("himself", &plain, &|p| { let a = p.sam_at(); on_floor(a) }, &|t| matches!(t, OrderTarget::None(n) if n.man.is_none()), Some("fallIn")),
        ];
        for (what, stage, cursor, is, want) in cases {
            let (key, shifted) = both(what, s, stage, cursor, is);
            assert_eq!(key.order, shifted.order, "seed {s}: on {what}");
            assert_eq!(key.order.as_deref(), want, "seed {s}: on {what}");
        }
        // a stacked door: nothing logged either way, a man answers, nobody moves
        let (key, shifted) = both("a door they are stacked on", s, &stacked, &|_| on_floor(DOOR), &is_door);
        for (k, x) in [("the key", &key), ("with Shift", &shifted)] {
            assert_eq!(x.order, None, "seed {s}: {k} on a set stack");
            assert!(x.said, "seed {s}: {k} on a set stack: a man answers");
            assert!(!x.moved, "seed {s}: {k} on a set stack: nobody is told anything new");
        }
        // a teammate: under the cursor with the whole squad addressed he is the squad's, and the key is fall in. Asked by the two functions on the man himself, it is the same
        let (key, shifted) = both("a teammate", s, &plain, &|p| { let a = p.at(men(p)[1]); on_floor(a) }, &|t| matches!(t, OrderTarget::None(_)));
        assert_eq!(key.order, shifted.order, "seed {s}: on a teammate");
        assert_eq!(key.order.as_deref(), Some("fallIn"), "seed {s}: on a teammate");
        let on_man = |shift: bool| { let mut p = yard(s); let m = men(&p)[1]; let a = p.at(m); let t = game::orders::squad_orders_target_at(&mut p.g, [a[0], 0.0, a[1]]); assert!(matches!(&t, OrderTarget::None(n) if n.man == Some(m)), "seed {s}: the point on him is him: {t:?}"); let n = p.g.orders.log.len(); if shift { game::planmode::plan_goal(&mut p.g, &t); } else { game::autopilot::auto_quick(&mut p.g, &t); } (p.g.orders.log.len() > n).then(|| p.g.orders.log.last().unwrap().order.clone()) };
        assert_eq!(on_man(false), on_man(true), "seed {s}: on the man himself");
        // a spot by a wall that takes a charge, with one carried: the nearer whole pair
        let walled = |seed: u32| { let mut p = office(seed); game::wallcharge::wall_charge_raise_carried(&mut p.g, 2.0); p.sam_put([14.6, 15.2]); p.run(0.5); p };
        let (key, shifted) = both("a spot by a wall", s, &walled, &|_| [12.06, 1.2, 13.78], &|t| matches!(t, OrderTarget::Floor(f) if f.wall.is_some()));
        assert_eq!(key.order.as_deref(), Some("move"), "seed {s}: on a spot by a wall");
        assert_eq!(shifted.order.as_deref(), Some("move"), "seed {s}: on a spot by a wall");
        { let p = walled(s); let all = men(&p); assert_eq!(sort_names(shifted.who.clone()), names(&p, &nearer_pair(&p, &all, [12.06, 13.78])), "seed {s}: by a wall, the nearer pair"); assert_eq!(key.who.len(), 4, "seed {s}: by a wall, the plain key is everyone's"); }
        // a thing to work: the breaker box. The row sends who it picks, and with Shift picks among the nearer whole pair
        let by_box = |seed: u32| { let mut p = Play::mission("kit-yard.json", "?levelFile=kit-yard.json&roomclear&squad4", &Opts { seed, ..Default::default() }); p.sam_unhurt(); p.run(0.5); let b = breaker(&p); p.sam_put([b[0], b[2] + 2.5]); p.run(0.5); p };
        let (key, shifted) = both("a thing to work", s, &by_box, &|p| breaker(p), &|t| matches!(t, OrderTarget::Floor(f) if f.thing.is_some()));
        assert_eq!(key.order.as_deref(), Some("work"), "seed {s}: on a thing to work");
        assert_eq!(shifted.order.as_deref(), Some("work"), "seed {s}: on a thing to work");
        { let p = by_box(s); let all = men(&p); let b = breaker(&p); let near = names(&p, &nearer_pair(&p, &all, [b[0], b[2]])); assert!(!shifted.who.is_empty() && shifted.who.iter().all(|w| near.contains(w)), "seed {s}: on a thing, {:?} of the nearer pair {near:?}", shifted.who); }
        // a hostage, on a rescue with two teammates: a place on the floor to the squad's key
        let rescue = |seed: u32| { let mut p = Play::mission("consulate.json", "?levelFile=consulate.json&rescue&squad=2", &Opts { seed, ..Default::default() }); p.sam_unhurt(); p.run(0.5); let h = p.g.rescue.hostages[0]; let a = p.at(h); p.sam_put([a[0], a[1] + 2.5]); p.run(0.5); p };
        let (key, shifted) = both("a hostage", s, &rescue, &|p| { let a = p.at(p.g.rescue.hostages[0]); on_floor(a) }, &|t| matches!(t, OrderTarget::Floor(f) if f.thing.is_none()));
        assert_eq!(key.order.as_deref(), Some("move"), "seed {s}: on a hostage");
        assert_eq!(shifted.order, key.order, "seed {s}: on a hostage");
        assert_eq!(shifted.who.len(), 1, "seed {s}: on a hostage, half of two");
        // a foe, with a squad of four so that a part is asked: the partner's menu is raised for four by hand (no mission has that), and the two functions are asked on the target itself
        let foe_order = |shift: bool| {
            let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: s, ..Default::default() });
            p.sam_unhurt();
            p.g.k.pairup.on = true;
            p.g.k.pair.max_squad = 4.0;
            game::pairup::pair_up_apply(&mut p.g, None);
            p.g.k.silent.on = true;
            p.run(0.3);
            let foe = p.g.guards.iter().copied().find(|&gd| p.g.gd[gd].team != game::teams::Team::Operative).unwrap();
            p.drop_all_but(&[foe]);
            game::silent::set_silent(&mut p.g, true, None);
            p.sam_put([11.0, 18.5]);
            for (k, &m) in p.mates().iter().enumerate() { p.man_stands(m, [12.0 + k as f64 * 0.8, 17.6]); }
            for _ in 0..30 { p.pin_man(foe, [22.0, 17.6], std::f64::consts::FRAC_PI_2, game::game::GuardState::Patrol, 0.0); p.frame(); }
            let a = p.at(foe);
            let t = OrderTarget::Foe(game::orders::OrderTargetFoe { at: [a[0], 0.0, a[1]], foe });
            let (part, n) = (game::planmode::goal_men(&p.g, &t).map(|g| g.men), p.g.orders.log.len());
            if shift { game::planmode::plan_goal(&mut p.g, &t); } else { game::autopilot::auto_quick(&mut p.g, &t); }
            let row = (p.g.orders.log.len() > n).then(|| p.g.orders.log.last().unwrap().clone());
            (row.as_ref().map(|r| r.order.clone()), row.map_or(Vec::new(), |r| r.who), part.map_or(Vec::new(), |l| names(&p, &l)))
        };
        let ((key, _, _), (shifted, who, part)) = (foe_order(false), foe_order(true));
        assert_eq!(key, shifted, "seed {s}: on a foe");
        assert!(key.is_some(), "seed {s}: on a foe the key gives nothing");
        assert_eq!(part.len(), 2, "seed {s}: on a foe the part is a pair: {part:?}");
        assert!(!who.is_empty() && who.iter().all(|w| part.contains(w)), "seed {s}: on a foe, {who:?} of {part:?}");
    }
}

/// THE RULE: kept down on a door, the ring is for the part and its first wedge is the tap's: a whole pair stacks on a door nobody is on, and on a door all four stand set on
/// it is the stack's own answer, with nobody moved.
/// A FAILURE MEANS: the ring's first wedge asks something other than the tap.
#[test]
fn the_parts_first_wedge_on_a_door_is_the_tap() {
    for s in SEEDS {
        let mut p = yard(s);
        let all = men(&p);
        let near = sorted(nearer_pair(&p, &all, DOOR));
        p.cursor_on(on_floor(DOOR));
        p.run(0.2);
        wheel(&mut p);
        assert_eq!(sorted(p.g.orders.menu.clone().unwrap().part.expect("a part's ring")), near, "seed {s}");
        assert_eq!(p.g.orders.menu.clone().unwrap().quick.as_deref(), Some("stack up"), "seed {s}");
        assert_eq!(wedge(&mut p, 1.0).as_deref(), Some("stack"), "seed {s}");
        assert_eq!(sorted(the(&p, &all, "stack")), near, "seed {s}: {:?}", on_all(&p));
        drop(p);

        let mut p = yard(s);
        let all = men(&p);
        p.order("stack", DOOR, Who::All);
        p.until(40.0, |p| { let l = men(p); l.iter().all(|&m| game::orders::squad_orders_of(&p.g, &game::seats::here(&p.g), m).is_some_and(|a| p.g.tab.orders_assignment[a].arrived)) }).expect("the stack stands");
        'looking: for y in [0.05, 1.0, 1.8] { for dx in [0.0, 0.45, -0.45, 0.2, -0.2] { for dz in [0.0, -0.3, -0.6] {
            p.cursor_on([DOOR[0] + dx, y, DOOR[1] + dz]);
            p.run(0.05);
            if matches!(under(&p), OrderTarget::Door(_)) { break 'looking; }
        } } }
        wheel(&mut p);
        let m = p.g.orders.menu.clone().unwrap();
        assert_eq!(m.part.expect("a part's ring").len(), 4, "seed {s}: the door's men");
        assert_eq!(m.quick.as_deref(), Some("stack up"), "seed {s}");
        let (said, was) = (p.said.len(), on_all(&p));
        assert_eq!(wedge(&mut p, 1.0), None, "seed {s}: answered, not logged");
        assert!(p.said.len() > said, "seed {s}: a man answers");
        assert_eq!(on_all(&p), was, "seed {s}: nobody is told anything new");
        assert_eq!(the(&p, &all, "stack").len(), 4, "seed {s}");
    }
}

/// THE RULE: taps on spots go on by the same rule: a third spot takes one man off a pair, the same spot tapped twice sends the part and then the others (with Shift it is
/// never 'fall in'), and with two teammates each goes alone until nobody is free.
/// A FAILURE MEANS: the Shift tap has its own idea of who is free.
#[test]
fn more_taps_part_the_squad_down() {
    let groups = |p: &Play| { let mut by: Vec<(f64, usize)> = Vec::new(); for m in men(p) { if let Some(a) = game::orders::squad_orders_of(&p.g, &game::seats::here(&p.g), m) { let r = &p.g.tab.orders_assignment[a]; if r.post.is_some() { if let Some(x) = by.iter_mut().find(|x| x.0 == r.group) { x.1 += 1; } else { by.push((r.group, 1)); } } } } let mut l: Vec<usize> = by.into_iter().map(|x| x.1).collect(); l.sort(); l };
    for s in SEEDS {
        let mut p = yard(s);
        for at in [SPOT, [16.0, 24.5], [11.0, 20.0]] { p.cursor_on(on_floor(at)); p.run(0.2); assert_eq!(tap(&mut p, true).as_deref(), Some("move"), "seed {s}"); p.run(0.3); }
        assert_eq!(groups(&p), vec![1, 1, 2], "seed {s}: the third tap takes one man off a pair");
        drop(p);

        let mut p = yard(s);
        let all = men(&p);
        p.cursor_on(on_floor(SPOT));
        p.run(0.2);
        assert_eq!(tap(&mut p, true).as_deref(), Some("move"), "seed {s}");
        assert_eq!(tap(&mut p, true).as_deref(), Some("move"), "seed {s}: tapped again on the spot");
        assert_eq!(the(&p, &all, "move").len(), 4, "seed {s}: the others take it: {:?}", on_all(&p));
        assert_eq!(groups(&p), vec![2, 2], "seed {s}");
        drop(p);

        let mut p = Play::mission("kit-yard.json", "?levelFile=kit-yard.json&roomclear&squad=2", &Opts { seed: s, ..Default::default() });
        p.sam_unhurt();
        p.run(1.0);
        let all = men(&p);
        assert_eq!(all.len(), 2, "seed {s}: the staging");
        let sam = p.sam_at();
        let mut spots: Vec<[f64; 2]> = Vec::new();
        for d in [[3.0, 0.0], [-3.0, 0.0], [0.0, 3.0], [0.0, -3.0], [2.5, 2.5], [-2.5, -2.5], [2.5, -2.5], [-2.5, 2.5], [4.5, 0.0], [-4.5, 0.0], [0.0, 4.5], [0.0, -4.5]] {
            let at = [sam[0] + d[0], sam[1] + d[1]];
            p.cursor_on(on_floor(at));
            p.run(0.05);
            if matches!(under(&p), OrderTarget::Floor(f) if f.thing.is_none() && f.wall.is_none()) && all.iter().all(|&m| game_start::play::dist(p.at(m), at) > 1.5) && spots.iter().all(|&q| game_start::play::dist(q, at) > 2.0) { spots.push(at); }
        }
        assert!(spots.len() >= 3, "seed {s}: the staging: {spots:?}");
        let first = if game_start::play::dist(p.at(all[0]), spots[0]) <= game_start::play::dist(p.at(all[1]), spots[0]) { all[0] } else { all[1] };
        p.cursor_on(on_floor(spots[0]));
        p.run(0.2);
        assert_eq!(tap(&mut p, true).as_deref(), Some("move"), "seed {s}");
        assert_eq!(the(&p, &all, "move"), vec![first], "seed {s}: the nearer man alone");
        p.run(0.3);
        p.cursor_on(on_floor(spots[1]));
        p.run(0.2);
        assert_eq!(tap(&mut p, true).as_deref(), Some("move"), "seed {s}");
        assert_eq!(the(&p, &all, "move").len(), 2, "seed {s}");
        p.run(0.3);
        p.cursor_on(on_floor(spots[2]));
        p.run(0.2);
        assert_eq!(tap(&mut p, true), None, "seed {s}: nobody is free");
        assert!(p.said.iter().any(|l| l.text.contains("nobody's free")), "seed {s}: and the squad says so");
    }
}

/// THE RULE: in a game of two, the second player's Shift moves part of HIS men and nobody of the first's. Asked in his seat, by the function behind his keys.
/// A FAILURE MEANS: the rule reads the first seat somewhere.
#[test]
fn the_second_seats_shift_moves_his_own_men() {
    for s in SEEDS {
        let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: s, seats: 2, ..Default::default() });
        p.sam_unhurt();
        p.run(1.0);
        let (a, b) = (p.g.seats[0], p.g.seats[1]);
        let of = |p: &Play, seat| -> Vec<(GuardId, Option<String>)> { let sr = game::seats::seat(&p.g, seat); game::orders::squad_orders_men(&p.g, &sr, Some(game::orders::SquadOrdersMenSel::All), None).into_iter().map(|m| { let o = game::orders::squad_orders_of(&p.g, &sr, m).and_then(|x| { let r = &p.g.tab.orders_assignment[x]; r.post.map(|_| r.order.clone()) }); (m, o) }).collect() };
        let first0 = of(&p, a);
        let at = p.seat_at(1);
        game::seats::with_seat(&mut p.g, b, &mut |g| { let t = game::orders::squad_orders_target_at(g, [at[0] + 3.0, 0.0, at[1]]); assert!(matches!(&t, OrderTarget::Floor(f) if f.thing.is_none()), "seed {s}: the staging: {t:?}"); game::planmode::plan_goal(g, &t); });
        p.frame();
        let moved = of(&p, b).into_iter().filter(|x| x.1.as_deref() == Some("move")).count();
        let his = of(&p, b).len();
        assert!(his >= 2 && moved == his / 2, "seed {s}: half of his men: {:?}", of(&p, b));
        assert_eq!(of(&p, a), first0, "seed {s}: the first player's men keep what they had");
    }
}
fn door_at(p: &Play, at: [f64; 2]) -> game::ids::DoorId {
    p.g.doors.list.iter().copied().min_by(|&a, &b| { let d = |x: game::ids::DoorId| { let c = p.g.dr[x].frame_centre; game_start::play::dist([c[0], c[2]], at) }; d(a).partial_cmp(&d(b)).unwrap() }).unwrap()
}
fn breaker(p: &Play) -> [f64; 3] { p.g.level.breaker.pos }

/// THE RULE: a door one pair stands on and the others do not. The line says 'stack up' either way and it is the stack's row either way: the plain key has all four stack
/// there; with Shift the door is those men's, who are on their marks already, so they answer and nobody walks and no row is logged. THE ONE TARGET WHERE THE TWO LOGS DIFFER.
/// A FAILURE MEANS: the part on a door has gone back to asking something else, or a Shift tap on their own door pulls the others in.
#[test]
fn a_door_a_pair_stands_on_is_that_pairs() {
    for s in SEEDS {
        for shift in [false, true] {
            let mut p = yard(s);
            let l = men(&p);
            let t = game::orders::squad_orders_target_at(&mut p.g, [DOOR[0], 0.0, DOOR[1]]);
            let opts = game::orders::GiveOpts { men: Some(l[..2].to_vec()), ack: None, on_word: None };
            assert_eq!(game::orders::squad_orders_give(&mut p.g, "stack", Some(&t), None, Some(&opts)), None, "seed {s}");
            p.run(30.0);
            'looking: for y in [0.05, 1.0, 1.8] { for dx in [0.0, 0.45, -0.45, 0.2, -0.2] { for dz in [0.0, -0.3, -0.6] {
                p.cursor_on([DOOR[0] + dx, y, DOOR[1] + dz]);
                p.run(0.05);
                if matches!(under(&p), OrderTarget::Door(_)) { break 'looking; }
            } } }
            assert!(p.g.orders.shown.quick.as_deref().is_some_and(|q| q.starts_with("stack up")), "seed {s}: the line: {:?}", p.g.orders.shown.quick);
            let n = p.said.len();
            let got = tap(&mut p, shift);
            if shift {
                assert_eq!(got, None, "seed {s}");
                assert_eq!(the(&p, &l, "stack").len(), 2, "seed {s}: nobody else is on it: {:?}", on_all(&p));
                assert!(p.said.len() > n, "seed {s}: the men on it answer");
            } else {
                assert_eq!(got.as_deref(), Some("stack"), "seed {s}");
                assert_eq!(the(&p, &l, "stack").len(), 4, "seed {s}: {:?}", on_all(&p));
            }
        }
    }
}

/// THE RULE: with nothing going, Shift and the key on a spot sends the nearer whole pair there on a move; the other two stay with him and nobody covers.
/// With all four on a door, the nearer pair leaves the door for the spot and the other pair stays on it. A second spot takes the others, a whole pair.
/// A FAILURE MEANS: a part of the squad is told something the key does not say, or the wrong part is told it.
#[test]
fn on_a_spot_the_nearer_pair_moves_and_the_others_keep_what_they_had() {
    for s in SEEDS {
        let mut p = yard(s);
        let all = men(&p);
        let near = sorted(nearer_pair(&p, &all, SPOT));
        p.cursor_on(on_floor(SPOT));
        p.run(0.2);
        assert_eq!(tap(&mut p, true).as_deref(), Some("move"), "seed {s}");
        assert_eq!(sorted(the(&p, &all, "move")), near, "seed {s}: the nearer pair moves: {:?}", on_all(&p));
        for m in all.iter().filter(|m| !near.contains(m)) { assert_eq!(on(&p, *m), None, "seed {s}: he has nothing to do and is told nothing"); }
        // the other two take the next spot, whichever is nearer it
        p.run(0.5);
        p.cursor_on(on_floor([16.0, 24.5]));
        p.run(0.2);
        assert_eq!(tap(&mut p, true).as_deref(), Some("move"), "seed {s}");
        assert_eq!(the(&p, &all, "move").len(), 4, "seed {s}: the other pair: {:?}", on_all(&p));
        assert!(the(&p, &all, "cover").is_empty(), "seed {s}");

        // all four on a door: the nearer pair leaves it
        drop(p);
        let mut p = yard(s);
        let all = men(&p);
        p.order("stack", DOOR, Who::All);
        p.run(1.0);
        let near = sorted(nearer_pair(&p, &all, SPOT));
        p.cursor_on(on_floor(SPOT));
        p.run(0.2);
        assert_eq!(tap(&mut p, true).as_deref(), Some("move"), "seed {s}");
        assert_eq!(sorted(the(&p, &all, "move")), near, "seed {s}: {:?}", on_all(&p));
        assert_eq!(the(&p, &all, "stack").len(), 2, "seed {s}: the other pair stays on the door: {:?}", on_all(&p));
    }
}

/// THE RULE: the words and the order are one. On a spot: the hover line opens on the words of the row the press gives; held, the ring is
/// for the part a tap would send, its first wedge says those words (menu.quick and the wedge's own text), and that wedge gives that row to that part.
/// A FAILURE MEANS: the player reads one order and the squad takes another.
#[test]
fn the_line_the_first_wedge_and_the_order_agree() {
    for s in SEEDS {
        let mut p = yard(s);
        let all = men(&p);
        let near = sorted(nearer_pair(&p, &all, SPOT));
        p.cursor_on(on_floor(SPOT));
        p.run(0.2);
        let t = under(&p);
        let d = p.g.orders.defs.iter().copied().find(|&d| p.g.tab.orders_order_def[d].id == "move").unwrap();
        let words = game::orders::squad_orders_label_of(&mut p.g, d, &t);
        let line = p.g.orders.shown.quick.clone().expect("a line on a spot");
        assert!(line.starts_with(&words), "seed {s}: the line '{line}', the row '{words}'");
        wheel(&mut p);
        let m = p.g.orders.menu.clone().expect("the ring's menu");
        assert_eq!(sorted(m.part.clone().expect("the ring is a part's")), near, "seed {s}: the ring is for the men a tap would send");
        assert_eq!(m.quick.as_deref(), Some(words.as_str()), "seed {s}: the ring's first word");
        let v = p.view().ring.expect("the ring");
        assert_eq!(v.wedges.iter().find(|w| w.key == 1.0).map(|w| w.text.as_str()), Some(words.as_str()), "seed {s}: {v:?}");
        assert_eq!(on_all(&p), vec![None; 4], "seed {s}: nobody is told anything while it is up");
        assert_eq!(wedge(&mut p, 1.0).as_deref(), Some("move"), "seed {s}");
        assert_eq!(sorted(the(&p, &all, "move")), near, "seed {s}: {:?}", on_all(&p));
    }
}

/// THE RULE: covering a spot comes from the wheel, and to that part alone. Shift and the key kept down on a spot: its `cover` wedge has the part cover it, turned to it,
/// and the others keep what they had. On a stack, the pair that covers leaves the door and the other pair stays on it.
/// A FAILURE MEANS: the only way to have men watch a point is gone, or it takes men it was not for.
#[test]
fn covering_a_spot_is_the_wheels_and_that_parts_only() {
    for s in SEEDS {
        for stacked in [false, true] {
            let mut p = yard(s);
            let all = men(&p);
            if stacked { p.order("stack", DOOR, Who::All); p.run(1.0); }
            let near = sorted(nearer_pair(&p, &all, SPOT));
            p.cursor_on(on_floor(SPOT));
            p.run(0.2);
            wheel(&mut p);
            let t = p.g.orders.menu.clone().expect("the ring's menu").target;
            let d = p.g.orders.defs.iter().copied().find(|&d| p.g.tab.orders_order_def[d].id == "cover").unwrap();
            let words = game::orders::squad_orders_label_of(&mut p.g, d, &t);
            let v = p.view().ring.expect("the ring");
            let w = v.wedges.iter().find(|w| w.text == words).unwrap_or_else(|| panic!("seed {s}: no wedge says '{words}': {v:?}"));
            assert_eq!(wedge(&mut p, w.key).as_deref(), Some("cover"), "seed {s}");
            assert_eq!(sorted(the(&p, &all, "cover")), near, "seed {s}: {:?}", on_all(&p));
            for &m in all.iter().filter(|m| !near.contains(m)) {
                assert_eq!(on(&p, m), if stacked { Some(String::from("stack")) } else { None }, "seed {s}: the others keep what they had");
            }
        }
    }
}
