//! HIS HANDS UP CLOSE: the right click puts a man out from every side, the left is no blow, and nothing reaches through a wall.
mod kit_ask;
use game::game::DownKind;
use game_start::play::{dist, nudge, Button, Opts, Play};
use game_start::scene::{Axis, Scene};
use kit_ask::KitAsk;

fn seed(s: u32) -> Opts { Opts { seed: s, ..Default::default() } }
fn click(p: &mut Play, b: Button) { p.press(b); p.frame(); p.release(b); p.frame(); }

/// One man in a dark room looking east, nobody able to see Sam, Sam's guns put away, `gap` metres from the man and `bearing` degrees round from his facing (180 is behind him).
fn beside_a_man(bearing: f64, gap: f64, s: u32) -> (Play, game::ids::GuardId) {
    let mut sc = Scene::new("beside-a-man");
    let a = std::f64::consts::FRAC_PI_2 + bearing.to_radians();
    let at = [15.0 + 0.5 * nudge(s, 0), 14.0 + 0.5 * nudge(s, 1)];
    sc.room("room", 10.0, 10.0, 22.0, 18.0).door("south", Axis::X, 18.0, 20.0).bulb_off(16.0, 14.0);
    sc.sentry("man", at, [21.0, at[1]]).sam(at[0] + a.sin() * gap, at[1] + a.cos() * gap);
    let mut p = Play::scene(&sc, &seed(s));
    p.sam_a_ghost();
    p.sam_unhurt();
    let man = p.guard("man");
    p.point_at(p.at(man));
    p.run(0.6);
    (p, man)
}

/// THE RULE: "melee attacks work from all angles": from any side of a man who stands, empty hands in reach, the right click puts him out cold: struck, breathing, a knockout and no kill.
/// Out of reach (the control) the same click does nothing.
/// A FAILURE MEANS: a side of a man has no blow, or the blow has become a kill, or it reaches across a room.
#[test]
fn the_right_click_is_the_knockout_from_every_side() {
    for (s, b) in [(1, 0.0), (2, 45.0), (3, 90.0), (1, 135.0), (2, 180.0), (3, 225.0), (1, 270.0), (2, 315.0)] {
        let (mut p, man) = beside_a_man(b, 0.8, s);
        click(&mut p, Button::Right);
        p.until(4.0, |p| !p.up(man)).unwrap_or_else(|| panic!("{b}°: the right click from {:.2} m and he is still up", dist(p.sam_at(), p.at(man))));
        assert_eq!(p.down_how(man), Some(DownKind::Struck), "{b}°: he went down some other way");
        assert_eq!(p.card(), (0.0, 1.0), "{b}°: kills, knockouts");
    }
    let (mut p, man) = beside_a_man(180.0, 3.0, 1);
    click(&mut p, Button::Right);
    p.run(1.5);
    assert!(p.up(man), "the control: the right click from 3 m put him down");
}

/// THE RULE: "there's only rmb to choke out": the left click with empty hands starts no blow on a man who stands, and on a man in the arm kills nobody; the right click on him in the arm is the choke.
/// A FAILURE MEANS: the knife has come back under the left button.
#[test]
fn the_left_click_is_no_blow() {
    for (s, b) in [(1, 180.0), (2, 90.0), (3, 0.0)] {
        let (mut p, man) = beside_a_man(b, 0.8, s);
        click(&mut p, Button::Left);
        assert!(!p.sam_mid_blow(), "{b}°: the left click began a blow");
        p.run(1.5);
        assert!(p.up(man), "{b}°: a second and a half after the left click he is down");
    }
    let (mut p, man) = beside_a_man(180.0, 0.8, 4);
    p.sam_grabs(man);
    p.until(2.5, |p| p.in_the_arm()).expect("the grab never settled");
    click(&mut p, Button::Left);
    p.run(1.0);
    assert!(p.up(man) && p.in_the_arm(), "the left click on a man in the arm did something");
    click(&mut p, Button::Right);
    p.until(15.0, |p| !p.up(man)).expect("the right click on a man in the arm: he never went down");
    assert_eq!(p.down_how(man), Some(DownKind::Choked));
    assert_eq!(p.card(), (0.0, 1.0), "a knockout and no kill");
}

/// A man with his back to a wall 12 cm thick and Sam behind it within a blow's reach of him (`through` true), or (the control) the same two out on the open floor of the room.
fn wall_at_his_back(through: bool, s: u32) -> (Play, game::ids::GuardId, [f64; 2]) {
    let mut sc = Scene::new("wall-at-his-back");
    sc.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0).door("between", Axis::Z, 18.0, 10.4).door("yard", Axis::X, 16.0, 21.4);
    let z = 13.5 + 0.5 * nudge(s, 0);
    let x = if through { 18.45 } else { 21.0 };
    sc.sentry("man", [x, z], [25.0, z]).sam(x - 0.95, z);
    let mut p = Play::scene(&sc, &seed(s));
    p.sam_a_ghost();
    p.sam_unhurt();
    let man = p.guard("man");
    p.point_at(p.at(man));
    p.run(0.3);
    let at = p.sam_at();
    (p, man, at)
}

/// THE RULE: no takedown through a thin wall: within reach but behind it, the row is not in reach, the blow does not land and a forced lunge leaves him on his own side. On the man's side it is offered.
/// A FAILURE MEANS: the reach reads distance and not the wall, and Sam takes men through partitions.
#[test]
fn no_takedown_through_a_thin_wall() {
    for s in [1, 2, 3] {
        let (mut p, man, at) = wall_at_his_back(true, s);
        assert!(dist(at, p.at(man)) <= 1.05, "seed {s}: the staging: reach would refuse it, not the wall ({:.2} m)", dist(at, p.at(man)));
        assert!(p.wall_between(at, p.at(man)), "seed {s}: the staging: no wall between them");
        assert_ne!(p.man_row_in_reach(man), Some(true), "seed {s}: the takedown is offered through the wall");
        click(&mut p, Button::Right);
        p.run(1.0);
        assert!(p.up(man), "seed {s}: the right click reached him through the wall");
        p.sam_lunges(man);
        p.run(1.0);
        assert!(!p.wall_between(at, p.sam_at()), "seed {s}: a forced lunge carried him through the wall");
        drop(p);

        let (mut p, man, _) = wall_at_his_back(false, s);
        assert_eq!(p.man_row_in_reach(man), Some(true), "seed {s}: the control: on his side of the wall the row is in reach");
        click(&mut p, Button::Right);
        p.until(4.0, |p| !p.up(man)).unwrap_or_else(|| panic!("seed {s}: the control: on his side of the wall the blow did not land"));
    }
}

/// THE RULE: Sam's head turns to a man close in front of where he aims, and not to one behind it. The man and the aim are put just either side of the compass's seam, so their two yaws read as nearly a
/// whole turn apart unless the angle is brought back.
/// CONTROL: the same man with Sam aiming the other way: his head stays his own.
/// A FAILURE MEANS: Sam looks at nobody when he faces one way of the map, or stares over his shoulder.
#[test]
fn sams_head_turns_to_a_man_in_front_of_his_aim_whichever_way_he_faces() {
    for s in [1u32, 2, 3] {
        let mut sc = Scene::new("sam-looks");
        sc.room("room", 10.0, 6.0, 22.0, 18.0).door("south", Axis::X, 18.0, 20.0).bulb_off(16.0, 14.0);
        sc.sentry("man", [16.2, 10.0], [16.2, 9.0]).sam(16.0, 14.0);
        let mut p = Play::scene(&sc, &seed(s));
        p.sam_a_ghost();
        p.sam_unhurt();
        let man = p.guard("man");
        let looking = |p: &Play| { let c = p.g.pl[p.g.player].ch; p.g.ch[c].anim.watch_k > 0.0 };
        // CONTROL
        p.point_at([16.0, 17.9]);
        p.run(0.5);
        assert!(!looking(&p), "seed {s}: he looks round at a man behind his aim");
        // his aim a hair on the far side of the seam from the man
        p.point_at([15.8, 10.0]);
        p.run(0.5);
        let c = p.g.pl[p.g.player].ch;
        let (aim, q, me) = (p.g.ch[c].aim_yaw, p.at(man), p.sam_at());
        let bearing = (q[0] - me[0]).atan2(q[1] - me[1]);
        assert!((aim - bearing).abs() > 5.0, "seed {s}: the two yaws lie either side of the seam: aim {aim}, the man {bearing}");
        assert!(looking(&p), "seed {s}: no look at a man right in front of him");
    }
}
