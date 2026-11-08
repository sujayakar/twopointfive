//! AN ORDER WRITTEN AS A MODULE (rust/game/src/orders/rows/): tests that stand a scene, give the order by the menu's own call and hold what
//! the squad does. README.md beside this crate says how such a test is written; each holds a rule over three seeds, each seed another scene.
use game::orders::Who;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 3] = [1, 2, 3];
fn seed(seed: u32) -> Opts { Opts { seed, ..Default::default() } }
const NORTH: f64 = std::f64::consts::PI;   // (a yaw: 0 looks south, along +z)

/// Two rooms side by side, each with a door to the yard south of them and one door between them; a defender posted in each; the team in the
/// yard under the moon, facing the building. The spot a shell is ordered onto is open ground in the yard, 12 to 14 m east of the nearest of
/// the team (a shell arms 8 m out) and clear of every man, by the seed.
fn yard() -> Scene {
    let mut s = Scene::new("yard");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("west-yard", Axis::X, 16.0, 13.4).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    s.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    s.team([14.0, 21.0], NORTH, &[[13.0, 22.0], [15.0, 22.0], [13.0, 23.0], [15.0, 23.0]]);
    s.defender("west", [12.25, 12.25], "west-yard").defender("east", [24.25, 12.25], "east-yard");
    s
}
fn spot(seed: u32) -> [f64; 2] { [28.0 + nudge(seed, 0), 24.0 + nudge(seed, 1)] }

fn left(p: &mut Play) -> f64 { game::squad40::squad40_left(&p.g, &game::seats::here(&p.g)) }
fn on_the_menu(p: &Play) -> bool { p.on_menu("shellThere") }

/// THE RULE: with one man of the squad carrying 40 mm shells, "put a shell there" is on the menu; given, that man turns to the spot and
/// fires one shell onto it within a few seconds, and nobody else leaves his place.
/// A FAILURE MEANS: the order written as a module is no longer reached by the menu (its row, its `only`, `valid`, `issue` or `tick`), or its
/// `frame` no longer puts the row on the menu when the squad brings shells.
#[test]
fn a_shell_ordered_onto_open_ground_is_fired_by_the_man_who_carries_them() {
    for s in SEEDS {
        let mut p = Play::scene(&yard(), &seed(s));
        let mates = p.mates();
        let at = spot(s);
        // (the man nearest the spot carries them: nobody of his own stands between him and it)
        let carrier = *mates.iter().min_by(|a, b| dist(p.at(**a), at).partial_cmp(&dist(p.at(**b), at)).unwrap()).unwrap();
        assert!(!on_the_menu(&p), "seed {s}: nobody carries a shell yet and the order is on the menu");
        game::squad40::squad40_stage(&mut p.g, &[(carrier, 2.0)]);
        assert_eq!(left(&mut p), 2.0, "seed {s}: the man was handed two shells (is the launcher under the gun he carries?)");
        p.run(0.5);
        assert!(on_the_menu(&p), "seed {s}: half a second after the squad brought shells the order is not on the menu");
        let was: Vec<[f64; 2]> = mates.iter().map(|m| p.at(*m)).collect();
        p.order("shellThere", at, Who::All);
        let took = p.until(6.0, |p| left(p) == 1.0).unwrap_or_else(|| panic!("seed {s}: 6 s after the order no shell was fired: {}", p.roll()));
        assert!(took > 0.2, "seed {s}: the shell left {took:.2} s after the order: he was to turn and aim first");
        p.run(1.0);
        assert_eq!(left(&mut p), 1.0, "seed {s}: one order, one shell");
        for (i, m) in mates.iter().enumerate() { assert!(dist(p.at(*m), was[i]) < 1.0, "seed {s}: man {i} moved {:.1} m for an order that asks one man to turn and fire", dist(p.at(*m), was[i])); }
        assert!(p.sounds.iter().any(|x| x.at.map_or(false, |q| dist([q[0], q[2]], at) < 3.0)), "seed {s}: nothing was heard at the spot the shell was ordered onto");
    }
}

/// The throws' page opened on `at` with the shell's row lit.
fn lit(p: &mut Play, at: [f64; 2]) -> f64 {
    let t = game::orders::squad_orders_target_at(&mut p.g, [at[0], 0.0, at[1]]);
    assert!(game::orders::squad_orders_open(&mut p.g, Some(&t), None, None));
    game::orders::squad_orders_turn(&mut p.g, Some(game::squadthrow::THROW_PAGE), None);
    let rows = p.g.orders.menu.as_ref().unwrap().rows.clone();
    let i = rows.iter().position(|&d| p.g.tab.orders_order_def[d].id == "shellThere").expect("the shell's row is on the page") as f64;
    p.g.orders.menu.as_mut().unwrap().sel = i;
    i
}
fn shown(p: &Play) -> Option<game::squad40::ShellPlan> { p.g.m.squad40.states.get(p.g.player).and_then(|s| s.plan.clone()) }

/// THE RULE: what the menu shows of a shell is what its press gives: the man, and where it goes off. A spot too near to arm is refused in the words shown, and nothing is drawn of it.
/// A FAILURE MEANS: the picture, the row and the order have gone back to asking for themselves.
#[test]
fn the_shell_shown_is_the_shell_given() {
    for s in SEEDS {
        let mut p = Play::scene(&yard(), &seed(s));
        let at = spot(s);
        let carrier = *p.mates().iter().min_by(|a, b| dist(p.at(**a), at).partial_cmp(&dist(p.at(**b), at)).unwrap()).unwrap();
        game::squad40::squad40_stage(&mut p.g, &[(carrier, 2.0)]);
        p.run(0.5);
        let preview = |p: &mut Play| p.view().throws.iter().filter(|x| x.preview).count();
        // too near: shown as refused, refused as shown
        let me = p.at(carrier);
        let i = lit(&mut p, [me[0] + 3.0, me[1]]);
        p.run(0.2);
        let why = shown(&p).and_then(|x| x.no).unwrap_or_else(|| panic!("seed {s}: a shell 3 m off is shown as one that may be fired"));
        assert_eq!(preview(&mut p), 0, "seed {s}: a refused shell is drawn");
        assert_eq!(game::orders::squad_orders_pick(&mut p.g, Some(i)), Some(why), "seed {s}: refused in other words than were shown");
        p.run(0.2);
        // the open ground: the man and the burst
        let i = lit(&mut p, at);
        p.run(0.2);
        let was = shown(&p).and_then(|x| x.shot).unwrap_or_else(|| panic!("seed {s}: no shell is shown"));
        assert_eq!((was.gd, preview(&mut p)), (carrier, 1), "seed {s}");
        assert_eq!(game::orders::squad_orders_pick(&mut p.g, Some(i)), None, "seed {s}: shown, and refused");
        p.until(6.0, |p| left(p) == 1.0).unwrap_or_else(|| panic!("seed {s}: no shell was fired"));
        let log = p.g.m.squad40.squad40_log.last().unwrap();
        assert!(dist([log.burst[0], log.burst[2]], [was.burst[0], was.burst[2]]) < 0.5, "seed {s}: shown at {:?}, fired to {:?}", was.burst, log.burst);
        assert!(shown(&p).is_none(), "seed {s}: no menu is up and a shell is still shown");
    }
}

/// THE CONTROL: the same squad and the same spot, with no shells brought. The order is not on the menu and the menu's own call refuses it.
#[test]
fn with_no_shells_brought_the_order_is_not_on_the_menu() {
    for s in SEEDS {
        let mut p = Play::scene(&yard(), &seed(s));
        p.run(0.5);
        assert!(!on_the_menu(&p), "seed {s}: the order is on the menu of a squad that carries no shell");
        let at = spot(s);
        let t = game::orders::squad_orders_target_at(&mut p.g, [at[0], 0.0, at[1]]);
        let why = game::orders::squad_orders_give(&mut p.g, "shellThere", Some(&t), Some(Who::All), None);
        assert!(why.is_some(), "seed {s}: an order that is not on the menu was taken");
    }
}

/// THE RULE: every row on the menu's table holds the order of its own id, each order has one line of the list,
/// and reading a game back deals every row what it was made with.
/// A FAILURE MEANS: a row was made from another order's `Row`, or the table in `deal_closures` has parted from what a row is registered with.
#[test]
fn every_row_holds_its_own_order_and_reading_a_game_back_deals_it_the_same() {
    use game::orders::rows;
    let mut p = Play::scene(&yard(), &seed(1));
    p.run(1.0);
    let rows_now = |p: &Play| -> Vec<String> { p.g.tab.orders_order_def.ids().map(|d| format!("{:?}", p.g.tab.orders_order_def[d])).collect() };
    let was = rows_now(&p);
    assert!(was.len() > 10, "only {} rows on the table", was.len());
    for d in p.g.tab.orders_order_def.ids() {
        let r = &p.g.tab.orders_order_def[d];
        assert_eq!(r.row.id, r.id, "the row {} holds another order's functions", r.id);
        assert!(rows::by_id(&r.id).is_some(), "the row {} is no line of the list", r.id);
    }
    for r in rows::ROWS {
        let same = rows::ROWS.iter().filter(|x| x.id == r.id).count();
        assert_eq!(same, 1, "{} lines of the list have the id {}", same, r.id);
    }
    game::io::deal_closures(&mut p.g);
    assert_eq!(rows_now(&p), was, "reading a game back deals a row other than it was made with");
}

/// THE RULE: what an order says of itself is what the lists of names used to say, word for word: every id that was in such a list is a row
/// that says it, and no other row does. (The old lists also held words no row has: threshold, outside, thrStep and thrBang.)
/// A FAILURE MEANS: an order says something it did not, or stopped saying it, and a rule that asks now treats that order another way.
#[test]
fn every_order_says_what_the_old_lists_said() {
    use game::orders::rows::{self, Is};
    let lists: [(Is, &[&str]); 13] = [
        (Is::Follows, &["fallIn"]),
        (Is::Holds, &["hold"]),
        (Is::Covers, &["cover"]),
        (Is::Stacks, &["stack"]),
        (Is::Resumed, &["move", "stack", "cover", "hold", "fallIn"]),
        (Is::KeepsLane, &["cover", "stack", "move", "hold"]),
        (Is::WalksLow, &["move", "cover"]),
        (Is::LiftsFire, &["openClear", "bigClear", "bangClear", "fragClear", "charge", "thresholdHold", "thrWord", "go", "wallCharge", "fragThere", "stunThere", "shellThere"]),
        (Is::Blows, &["charge", "wallCharge"]),
        (Is::Loud, &["bangClear", "bigClear", "charge", "fragClear", "wallCharge", "fragThere", "stunThere", "shellThere"]),
        (Is::Enters, &["openClear", "bangClear", "charge"]),
        (Is::Pair, &["pairMark", "pairWatch", "pairTake", "pairDark", "pairGo"]),
        (Is::Take, &["pairMark", "pairTake"]),
    ];
    for (what, ids) in lists {
        for id in ids { assert!(rows::by_id(id).is_some(), "{} is no row", id); assert!(rows::id_is(id, what), "{} no longer says {:?}", id, what); }
        for r in rows::ROWS { assert_eq!(r.is.contains(&what), ids.contains(&r.id), "{} and {:?}", r.id, what); }
    }
    assert!(!rows::id_is("thrBang", Is::Loud), "an id on no line is nothing");
}

/// THE RULE: the squad's block comes in the list's order: the door tools, the word, the pairs, smoke, the wall, the 40 mm, the throws.
/// A FAILURE MEANS: a line of `rows!` moved, and what a player reads in the block moved with it.
#[test]
fn the_block_comes_in_the_lists_order() {
    let said: Vec<&str> = game::orders::rows::ROWS.iter().filter(|r| r.notes.is_some()).map(|r| r.id).collect();
    assert_eq!(said, ["tools", "go", "pairGo", "smokeThere", "wallCharge", "shellThere", "throwThere"]);
}
