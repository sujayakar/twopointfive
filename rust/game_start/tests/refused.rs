//! WHAT THE SQUAD SAYS NO TO, ASKED OF THE RULE: a door found shut stops an order at its handle and the same row then boots it; a door the men hold
//! refuses being held again; the whole squad's posture is one order that takes the place of its other half, and on a door it waits behind 'more'. Every
//! order goes by the menu's own call (`squad_orders_give`) and every word is read off the menu the ring puts up.
use game::ids::DoorId;
use game::orders::{squad_orders_give, squad_orders_target_at, Who};
use game_start::play::{nudge, Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 3] = [1, 2, 3];
const NORTH: f64 = std::f64::consts::PI;

/// Two rooms with a door between them, each with its door to the yard south of them; the team in the yard, a little to one side by the seed. The defenders the game wants are taken off the
/// floor: what a door does is all that plays.
fn yard(seed: u32) -> (Play, DoorId, [f64; 2]) {
    let x = 13.4 + nudge(seed, 0);
    let mut s = Scene::new("refused-yard");
    s.room("inside", 10.0, 10.0, 18.0, 16.0).room("next", 18.0, 10.0, 26.0, 16.0);
    s.door("way-in", Axis::X, 16.0, x).door("next-in", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    s.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    s.team([x, 21.0], NORTH, &[[x - 1.0, 22.0], [x + 1.0, 22.0], [x - 1.0, 23.0], [x + 1.0, 23.0]]);
    s.defender("inside", [11.25, 11.25], "way-in").defender("next", [24.25, 12.25], "next-in");
    let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    for gd in p.hostiles_by([x, 13.0]) { p.falls(gd, game::game::DownBy::Player); }
    // (nobody carries a hammer or a shotgun: a man with one opens a shut door his own way, and that isn't what this asks)
    for m in p.mates() { if let Some(mut e) = p.g.m.kit.man_of.get(m).cloned() { e.kit.big = game::kit::BigKit::None; p.g.m.kit.man_of.set(m, e); } }
    p.run(0.2);
    let d = game::doors::doors_by_name(&p.g, "way-in").unwrap();
    (p, d, [x + 0.6, 16.0])
}
/// wedged from inside, as digin's own hand leaves a door: locked against the yard's face, to a lock no ring fits
fn wedge(p: &mut Play, d: DoorId) {
    let at = p.at(p.mates()[0]);
    let side = game::orders::side_of(&p.g, d, [at[0], 0.0, at[1]]);
    let name = p.g.dr[d].def.name.clone();
    p.g.dr[d].key_side = side;
    p.g.dr[d].lock_id = format!("wedge:{name}");
    game::doors::door_set_locked(&mut p.g, d, true);
}
fn give(p: &mut Play, id: &str, at: [f64; 2], who: Who) -> Option<String> {
    let t = squad_orders_target_at(&mut p.g, [at[0], 0.0, at[1]]);
    squad_orders_give(&mut p.g, id, Some(&t), Some(who), None)
}
fn shut(p: &Play, d: DoorId) -> bool { game::doors::door_angle(&p.g, d).abs() < 0.05 && game::doors::door_latched(&p.g, d) && !game::doors::door_off(&p.g, d) }
fn ops_on(p: &Play, d: DoorId) -> usize { p.g.door_tools.ops.iter().filter(|&&o| p.g.tab.doortools_door_op[o].door == d).count() }
/// how many of the squad have gone through to the room's side of the door
fn across(p: &Play) -> usize { p.mates().iter().filter(|&&m| p.at(m)[1] < 15.85).count() }
/// the squad's key held on a place until the ring is up: the menu it reads, and the words on the ring's first page
fn menu_on(p: &mut Play, at: [f64; 2]) -> (game::orders::Menu, Vec<String>) {
    p.point_at(at);
    p.frame();
    let key = p.g.k.cmdring.key.clone();
    p.key_down(&key);
    p.until(2.0, |p| p.view().ring.is_some()).unwrap_or_else(|| panic!("no ring two seconds after its key went down on {at:?}"));
    let (m, first) = (p.g.orders.menu.clone().unwrap(), p.view().ring.unwrap().wedges.into_iter().map(|w| w.text).collect());
    p.key_up(&key);
    p.key_down("Escape");
    p.frame();
    p.key_up("Escape");
    p.run(0.1);
    (m, first)
}
fn row_ids(p: &Play, m: &game::orders::Menu) -> Vec<String> { m.rows.iter().map(|&r| p.g.tab.orders_order_def[r].id.clone()).collect() }
fn word_of(p: &Play, m: &game::orders::Menu, id: &str) -> String {
    let ids = row_ids(p, m);
    let i = ids.iter().position(|r| r == id).unwrap_or_else(|| panic!("no row {id}: {ids:?} {:?}", m.words));
    m.words[i].clone()
}

/// THE RULE: an entry order at a door nobody knew was wedged stops at the handle: the lead says what he found, nobody boots it, the door stays shut and the men stay stacked on it,
/// and the same row, which now reads as a kick, boots it. A FAILURE MEANS: a squad walks into a wedged door and kicks it unasked, or stands at it for ever. The control is a door that isn't wedged.
#[test]
fn an_entry_at_a_door_found_wedged_stops_at_the_handle_and_the_same_row_boots_it() {
    for seed in SEEDS {
        let (mut p, d, door) = yard(seed);
        wedge(&mut p, d);
        assert_eq!(give(&mut p, "openClear", door, Who::All), None, "seed {seed}");
        p.until(40.0, |p| p.g.door_tools.stats.halted > 0.0).unwrap_or_else(|| panic!("seed {seed}: 40 s and nobody stopped at the handle: {:?}", p.g.door_tools.stats));
        p.run(1.0);
        let st = &p.g.door_tools.stats;
        assert_eq!((st.halted, st.kicks), (1.0, 0.0), "seed {seed}: one stop and no boot");
        assert!(shut(&p, d), "seed {seed}: the door is open");
        assert!(p.said.iter().any(|s| s.text.contains("wedged from inside. holding on it")), "seed {seed}: nobody said it: {:?}", p.said.iter().map(|s| &s.text).collect::<Vec<_>>());
        assert_eq!(across(&p), 0, "seed {seed}: somebody is through a shut door");
        assert_eq!(ops_on(&p, d), 0, "seed {seed}: the order is not over");
        let (m, _) = menu_on(&mut p, door);
        assert_eq!(word_of(&p, &m, "openClear"), "kick and clear · wedged from inside", "seed {seed}");
        assert_eq!(give(&mut p, "openClear", door, Who::All), None, "seed {seed}");
        p.until(30.0, |p| !shut(p, d)).unwrap_or_else(|| panic!("seed {seed}: the same row did not boot it: {:?}", p.g.door_tools.stats));
        assert_eq!((p.g.door_tools.stats.halted, p.g.door_tools.stats.kicks), (1.0, 1.0), "seed {seed}");
        drop(p);
        // the control: the same order at the same door, not wedged, opens it with nobody stopping
        let (mut p, d, door) = yard(seed);
        assert_eq!(give(&mut p, "openClear", door, Who::All), None, "seed {seed}");
        p.until(30.0, |p| !shut(p, d)).unwrap_or_else(|| panic!("seed {seed}: the open door stayed shut"));
        assert_eq!((p.g.door_tools.stats.halted, p.g.door_tools.stats.kicks), (0.0, 0.0), "seed {seed}: the control");
    }
}

/// THE RULE: a plain stack tries the handle, the lead says what he found, and the entry after it goes straight to the boot with no stop. A FAILURE MEANS: the team forgets a door it tried.
#[test]
fn a_door_tried_by_a_stack_is_known_and_the_entry_after_it_is_not_stopped() {
    for seed in SEEDS {
        let (mut p, d, door) = yard(seed);
        wedge(&mut p, d);
        assert_eq!(give(&mut p, "stack", door, Who::All), None, "seed {seed}");
        p.until(30.0, |p| p.g.door_tools.stats.tested > 0.0).unwrap_or_else(|| panic!("seed {seed}: nobody tried the handle"));
        p.run(1.0);
        assert!(p.said.iter().any(|s| s.text.ends_with("wedged from inside.")), "seed {seed}: {:?}", p.said.iter().map(|s| &s.text).collect::<Vec<_>>());
        assert_eq!(give(&mut p, "openClear", door, Who::All), None, "seed {seed}");
        p.until(30.0, |p| !shut(p, d)).unwrap_or_else(|| panic!("seed {seed}: the entry did not boot it: {:?}", p.g.door_tools.stats));
        assert_eq!((p.g.door_tools.stats.halted, p.g.door_tools.stats.kicks), (0.0, 1.0), "seed {seed}");
    }
}

/// THE RULE: "clear from outside" holds the door from the jambs with nobody across. Given again it's refused in the men's own words and changes nothing, its row says so, and going in is a
/// second order that spends no can. A FAILURE MEANS: a held door takes the same order twice, or going in costs a bang nobody ordered.
#[test]
fn a_door_held_from_outside_refuses_being_held_again_and_going_in_is_a_second_order() {
    for seed in SEEDS {
        let (mut p, d, door) = yard(seed);
        assert_eq!(give(&mut p, "thresholdHold", door, Who::All), None, "seed {seed}");
        p.until(40.0, |p| p.g.door_tools.ops.iter().any(|&o| { let o = &p.g.tab.doortools_door_op[o]; o.door == d && o.thr.is_some() && matches!(o.phase, game::doortools::Phase::Thr) }))
            .unwrap_or_else(|| panic!("seed {seed}: 40 s and nobody holds the jambs"));
        assert_eq!(across(&p), 0, "seed {seed}");
        let bangs = p.g.door_tools.bangs;
        assert_eq!(give(&mut p, "thresholdHold", door, Who::All).as_deref(), Some("already holding it."), "seed {seed}");
        let (m, _) = menu_on(&mut p, door);
        let i = row_ids(&p, &m).iter().position(|r| r == "thresholdHold").unwrap();
        // (the ring reads a row's why-not when its page opens, and this row is behind 'more': asked of the rule, as that page asks it)
        assert_eq!(game::orders::squad_orders_why_not(&mut p.g, m.rows[i], &m.target, Some(&m.men)).as_deref(), Some("already holding it."), "seed {seed}");
        assert_eq!(m.words[i], "clear from outside · holding it", "seed {seed}");
        assert_eq!(word_of(&p, &m, "openClear"), "go in and clear · no bang", "seed {seed}");
        assert_eq!(give(&mut p, "openClear", door, Who::All), None, "seed {seed}");
        p.until(25.0, |p| across(p) > 0).unwrap_or_else(|| panic!("seed {seed}: 25 s and nobody went in"));
        assert_eq!(p.g.door_tools.bangs, bangs, "seed {seed}: going in spent a can");
    }
}

/// THE RULE: going silent is one order for the whole squad, whoever it's addressed to, and it moves nobody. A menu has "go silent" and not "go loud", and the other way round once silent; on
/// a door it waits behind 'more' and takes no wedge from the door's tools; it isn't logged as one of the player's orders. A FAILURE MEANS: the posture is one man's, or a door's tools lost a place to it.
#[test]
fn the_squads_posture_is_one_order_for_all_and_waits_behind_more_on_a_door() {
    for seed in SEEDS {
        let (mut p, _d, door) = yard(seed);
        let floor = [door[0] + 1.6, 19.0];
        let posture = |ids: &[String]| (ids.iter().any(|r| r == "silent"), ids.iter().any(|r| r == "loud"));
        let (m, _) = menu_on(&mut p, floor);
        assert_eq!(posture(&row_ids(&p, &m)), (true, false), "seed {seed}: loud, on the floor: {:?}", m.words);
        let (m, first) = menu_on(&mut p, door);
        assert_eq!(posture(&row_ids(&p, &m)), (true, false), "seed {seed}: a door's menu: {:?}", m.words);
        assert!(!first.iter().any(|w| w == "go silent"), "seed {seed}: it took a wedge on the door's first page: {first:?}");
        assert_eq!(give(&mut p, "hold", floor, Who::All), None, "seed {seed}");
        p.run(0.4);
        let orders = |p: &Play| { let here = game::seats::here(&p.g); p.mates().iter().map(|&m| game::orders::squad_orders_of(&here, m)).collect::<Vec<_>>() };
        let silent = |p: &Play| game::silent::squad_silent(&p.g, &game::seats::here(&p.g));
        let (before, logged) = (orders(&p), p.g.orders.log.len());
        assert_eq!(give(&mut p, "silent", floor, Who::Num(0.0)), None, "seed {seed}");
        p.run(0.1);
        assert!(silent(&p), "seed {seed}");
        for m in p.mates() { assert!(game::silent::under_silence(&mut p.g, m), "seed {seed}: addressed to one man and {m:?} did not take it"); }
        assert_eq!(p.g.orders.log.len(), logged, "seed {seed}: it went on the log of the player's orders");
        assert_eq!(orders(&p), before, "seed {seed}: the posture moved somebody's order");
        let (m, _) = menu_on(&mut p, floor);
        assert_eq!(posture(&row_ids(&p, &m)), (false, true), "seed {seed}: silent, on the floor: {:?}", m.words);
        assert_eq!(give(&mut p, "loud", floor, Who::All), None, "seed {seed}");
        assert!(!silent(&p), "seed {seed}");
    }
}
