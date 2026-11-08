//! A ROW OF THE MENU IS ONE ROW OF THE TABLE, HOWEVER OFTEN IT IS LISTED (rust/game_rules/src/orders/p_02.rs `squad_orders_row`).
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

/// THE RULE: a digit is the first row's that has it. On a wall the fifth is the wall's own; the floor's own fifth row stays on the menu after it, the same row of the table, and
/// opening the menu adds none.
/// A FAILURE MEANS: a row is being copied to take its digit off (before: `silent` twice in the table, once with its digit and once without).
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
            let fifth: Vec<String> = up.iter().filter(|&&d| p.g.tab.orders_order_def[d].key == Some(5.0)).map(|&d| p.g.tab.orders_order_def[d].id.clone()).collect();
            assert!(fifth.len() > 1 && fifth[0].starts_with("wall"), "seed {s}: the rows with the fifth digit: {fifth:?}, the wall's first");
            for d in &up { assert!(p.g.orders.defs.contains(d), "seed {s}: a row of the wall's menu is no row of the squad's"); }
            p.run(0.3);
        }
        assert_eq!((p.menu_rows(), p.menu()), (rows, menu), "seed {s}: opening a wall's menu changed the table");
    }
}

/// THE RULE: the quick key's words on the panel are the row its press gives, for what that seat's cursor is on; none while a menu is up.
/// A FAILURE MEANS: the panel and the press have gone back to asking apart.
#[test]
fn the_quick_keys_words_are_the_order_it_gives() {
    for s in SEEDS {
        let mut p = yard(s);
        p.run(1.0);
        let (mut said, mut none) = (Vec::new(), 0);
        for at in [[13.4, 16.0], [21.4, 16.0], [16.0, 24.0], [20.0, 22.0]] {
            p.point_at(at);
            p.run(0.2);
            let Some(words) = p.g.orders.shown.quick.clone() else { none += 1; continue };
            let n = p.g.orders.log.len();
            let t = game::orders::squad_orders_target_at(&mut p.g, [at[0], 0.0, at[1]]);
            game::orders::squad_orders_quick_give(&mut p.g, &t);
            assert!(p.g.orders.log.len() > n, "seed {s}: '{words}' is on the panel at {at:?} and the key gives nothing");
            let id = p.g.orders.log.last().unwrap().order.clone();
            let d = p.g.orders.defs.iter().copied().find(|&d| p.g.tab.orders_order_def[d].id == id).unwrap();
            let row = game::orders::squad_orders_label_of(&mut p.g, d, &t);
            assert_eq!(row, words, "seed {s}: the panel says '{words}' and the key gave '{id}'");
            said.push(words);
        }
        assert!(said.len() >= 3, "seed {s}: the panel said {said:?}, and nothing on {none} of four");
        let t = game::orders::squad_orders_target_at(&mut p.g, [16.0, 0.0, 24.0]);
        game::orders::squad_orders_open(&mut p.g, Some(&t), None, None);
        p.run(0.1);
        assert_eq!(p.g.orders.shown.quick, None, "seed {s}: a menu is up");
    }
}

/// THE RULE: two players' panels are two: each seat keeps the words for its own cursor, whichever seat stands.
#[test]
fn each_seat_keeps_its_own_quick_words() {
    for s in SEEDS {
        let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: s, seats: 2, ..Default::default() });
        p.run(1.0);
        let (a, b) = (p.g.seats[0], p.g.seats[1]);
        let g = &*p.g;
        let (sa, sb) = (game::seats::seat(g, a).orders.shown.clone(), game::seats::seat(g, b).orders.shown.clone());
        assert!(sa.quick.is_some() && sb.quick.is_some(), "seed {s}: {sa:?} and {sb:?}");
    }
}
