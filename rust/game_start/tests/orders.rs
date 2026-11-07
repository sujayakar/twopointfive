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
