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
