//! A ROW OF THE MENU IS ONE ROW OF THE TABLE, HOWEVER OFTEN IT IS LISTED (rust/game_rules/src/squad_orders/orders/menu.rs `squad_orders_row`).
//!
//! What this holds. Modules list their rows again as the game goes: the door tools when the squad's roster changes or a held door's page is
//! rebuilt, the big tools when the man with the hammer falls. Each listing used to push a new row and leave the old one in the table, so the
//! table grew, every later row stood further on, and two machines of one game that had seen a different number of re-listings numbered their
//! rows differently (found as `thrWord` twice in one machine's table and once in the other's).
use game::orders::Who;
use game_start::play::{dist, Button, Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 3] = [1, 2, 3];
const NORTH: f64 = std::f64::consts::PI;

/// the team in a yard, two rooms with a defender each
fn yard(seed: u32) -> Play {
    let mut s = Scene::new("menu-yard");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("west-yard", Axis::X, 16.0, 13.4).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    s.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    s.team([14.0, 21.0], NORTH, &[[13.0, 22.0], [15.0, 22.0], [13.0, 23.0], [15.0, 23.0]]);
    s.defender("west", [12.25, 12.25], "west-yard").defender("east", [24.25, 12.25], "east-yard");
    Play::scene(&s, &Opts { seed, ..Default::default() })
}

/// the ids the table holds more than once
fn twice(rows: &[String]) -> Vec<String> {
    let mut seen: Vec<&String> = Vec::new();
    let mut out = Vec::new();
    for r in rows { if seen.contains(&r) { if !out.contains(r) { out.push(r.clone()); } } else { seen.push(r); } }
    out
}

/// THE RULE: a page rebuilt again and again adds no row. The held door's page is rebuilt by the door tools' own call, five times.
/// A FAILURE MEANS: a listing is pushing a row again (before: two more rows each time, `thresholdHold` and then the word's row).
#[test]
fn a_page_rebuilt_again_and_again_adds_no_row() {
    for s in SEEDS {
        let mut p = yard(s);
        p.run(1.0);
        let (rows, mut menu) = (p.menu_rows(), p.menu());
        assert!(rows.iter().any(|r| r == "thrWord") && rows.iter().any(|r| r == "thresholdHold"), "seed {s}: the staging is wrong: the door tools have listed nothing ({rows:?})");
        for k in 0..5 {
            game::doortools::door_tools_thr_rows(&mut p.g);
            p.run(0.2);
            assert_eq!(p.menu_rows(), rows, "seed {s}: rebuild {k} changed the table's rows");
        }
        let mut now = p.menu();
        menu.sort(); now.sort();
        assert_eq!(now, menu, "seed {s}: the menu does not hold the rows it held");
    }
}

/// THE RULE, in a fight: forty seconds of orders and fire with a grenade among the squad on the way, and at the end the table holds no id
/// twice. (Before, it held the word's row twice a second into ANY squad game, at rows 16 and 22: the door tools list it, then list it again
/// when they first meet the squad's roster.)
#[test]
fn a_fight_leaves_no_row_twice() {
    for s in SEEDS {
        let mut p = yard(s);
        p.sam_takes_his_rifle();
        p.run(1.0);
        let mates = p.mates();
        for k in 0..(40 * 60) {
            if k == 5 * 60 { let at = p.at(mates[3]); p.frag_lands(at); }
            if k % 360 == 60 || k % 600 == 300 {
                let sam = p.sam_at();
                let foe = p.g.guards.iter().copied().filter(|&gd| !mates.contains(&gd) && p.up(gd)).map(|gd| p.at(gd)).min_by(|a, b| dist(*a, sam).partial_cmp(&dist(*b, sam)).unwrap());
                if let Some(at) = foe {
                    if k % 360 == 60 { let t = game::orders::squad_orders_target_at(&mut p.g, [at[0], 0.0, at[1]]); let _ = game::orders::squad_orders_give(&mut p.g, "move", Some(&t), Some(Who::All), None); }
                    else { p.point_at(at); p.press(Button::Left); }
                }
            }
            if k % 600 == 303 { p.release(Button::Left); }
            p.frame();
        }
        let rows = p.menu_rows();
        assert_eq!(twice(&rows), Vec::<String>::new(), "seed {s}: the table holds a row twice ({} rows; {})", rows.len(), p.roll());
    }
}

/// THE RULE: a wall's menu is the floor's rows and the wall's own, each the squad's one row of the table, and opening it adds none.
/// A FAILURE MEANS: a row is being copied to put it on a menu (before: `silent` twice in the table).
#[test]
fn a_walls_menu_copies_no_row() {
    for s in SEEDS {
        let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: s, ..Default::default() });
        p.sam_unhurt();
        p.run(0.1);
        game::wallcharge::wall_charge_raise_carried(&mut p.g, 2.0);
        p.sam_put([14.6, 15.2]);
        p.run(0.5);
        let (rows, menu) = (p.menu_rows(), p.menu());
        assert_eq!(twice(&rows), Vec::<String>::new(), "seed {s}: the staging holds a row twice");
        for _ in 0..2 {
            let t = game::orders::squad_orders_target_at(&mut p.g, [12.06, 0.0, 13.78]);
            assert!(matches!(&t, game::orders::OrderTarget::Floor(f) if f.wall.is_some()), "seed {s}: the staging is wrong: the point is on no wall");
            let up: Vec<_> = game::orders::squad_orders_rows_for(&p.g, &game::seats::here(&p.g), &t, None, None);
            assert!(up.iter().any(|&d| p.g.tab.orders_order_def[d].id.starts_with("wall")), "seed {s}: the wall's own row is not among them");
            for d in &up { assert!(p.g.sets[p.g.seat].orders.defs.contains(d), "seed {s}: a row of the wall's menu is no row of the squad's"); }
            p.run(0.3);
        }
        assert_eq!((p.menu_rows(), p.menu()), (rows, menu), "seed {s}: opening a wall's menu changed the table");
    }
}

/// the line the key's press on `t` must have shown: the row's words, then what it is on, unless it is on nothing and no man
fn line_of(p: &Play, row: String, t: &game::orders::OrderTarget) -> String {
    if matches!(t, game::orders::OrderTarget::None(n) if n.man.is_none()) { return row; }
    format!("{} · {}", row, game::orders::squad_orders_hover_words(&p.g, Some(t)))
}
/// the quick key's line on this player's panel
fn panel(p: &Play, seat: usize) -> Option<String> { p.view_of(seat).squad.and_then(|s| s.quick).map(|n| n.text) }

/// THE RULE: the quick key's line on the panel is what its press gives (the row's own words, on what that seat's cursor is on), for a door, the floor and a
/// teammate; none while a menu is up. A FAILURE MEANS: the panel and the press have gone back to asking apart.
#[test]
fn the_quick_keys_line_is_the_order_it_gives() {
    for s in SEEDS {
        let mate_at = { let p = yard(s); p.at(p.mates()[1]) };
        for (what, at) in [("a door", [13.4, 16.0]), ("the floor", [20.0, 22.0]), ("a teammate", mate_at)] {
            let mut p = yard(s);
            p.run(1.0);
            p.point_at(at);
            p.run(0.2);
            let line = panel(&p, 0).unwrap_or_else(|| panic!("seed {s}: nothing on the panel on {what}"));
            assert_eq!(p.g.sets[p.g.seat].orders.shown.quick.as_deref(), Some(line.as_str()), "seed {s}: the panel reads what the tick keeps");
            // (what the press takes: the cursor's own target. A teammate under it, with the whole squad addressed, is the squad's: "fall in", on nobody)
            let now = game::view::host_now(&p.g, p.host.input, p.host.cam, p.host.canvas);
            let t = game::orders::hover(&p.g, &game::seats::seat(&p.g, p.g.seats[0]), &now).expect("the key is live");
            let n = p.g.sets[p.g.seat].orders.log.len();
            game::autopilot::auto_quick(&mut p.g, &t);
            assert!(p.g.sets[p.g.seat].orders.log.len() > n, "seed {s}: '{line}' is on the panel on {what} and the key gives nothing");
            let id = p.g.sets[p.g.seat].orders.log.last().unwrap().order.clone();
            let d = p.g.sets[p.g.seat].orders.defs.iter().copied().find(|&d| p.g.tab.orders_order_def[d].id == id).unwrap();
            let row = game::orders::squad_orders_label_of(&mut p.g, d, &t);
            assert_eq!(line, line_of(&p, row, &t), "seed {s}: on {what} the key gave '{id}'");
        }
    }
}

/// THE CONTROL: with a menu up the key gives nothing of its own, and no line is kept or told.
#[test]
fn no_quick_line_with_a_menu_up() {
    for s in SEEDS {
        let mut p = yard(s);
        p.run(1.0);
        p.point_at([16.0, 24.0]);
        p.run(0.2);
        assert!(panel(&p, 0).is_some(), "seed {s}: the floor has a line");
        let t = game::orders::squad_orders_target_at(&mut p.g, [16.0, 0.0, 24.0]);
        game::orders::squad_orders_open(&mut p.g, Some(&t), None);
        p.ring_over_menu();
        p.run(0.1);
        assert_eq!(p.g.sets[p.g.seat].orders.shown.quick, None, "seed {s}: a menu is up");
        assert_eq!(panel(&p, 0), None, "seed {s}: a menu is up");
    }
}

/// THE RULE: two players' panels are two: each seat is told the line for its own cursor, whichever seat stands. A FAILURE MEANS: the second player reads the first's.
#[test]
fn each_seat_keeps_its_own_quick_line() {
    for s in SEEDS {
        let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: s, seats: 2, ..Default::default() });
        p.run(1.0);
        let (a, b) = (p.g.seats[0], p.g.seats[1]);
        {
            let g = &*p.g;
            let (sa, sb) = (game::seats::seat(g, a).orders.shown.clone(), game::seats::seat(g, b).orders.shown.clone());
            assert!(sa.quick.is_some() && sb.quick.is_some(), "seed {s}: {sa:?} and {sb:?}");
            assert_eq!((panel(&p, 0), panel(&p, 1)), (sa.quick, sb.quick), "seed {s}: each panel reads its own seat's");
        }
        // the first player's menu takes his line and leaves the second's
        let t = { let at = p.at(p.mates()[0]); game::orders::squad_orders_target_at(&mut p.g, [at[0] + 1.0, 0.0, at[1]]) };
        game::orders::squad_orders_open(&mut p.g, Some(&t), None);
        p.ring_over_menu();
        p.run(0.1);
        assert_eq!(panel(&p, 0), None, "seed {s}: his menu is up");
        assert!(panel(&p, 1).is_some(), "seed {s}: the other player has no menu up");
    }
}
