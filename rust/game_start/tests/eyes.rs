//! A MAN'S LOOK IS LEVEL (rust/game_rules/src/guards/p_04.rs `look_of`): his pose turns his eyes and does not tip them.
//!
//! What this holds. The team and one defender stand four metres apart in one lit room. Before the look was levelled, a man kneeling with his gun
//! low had his chest pointing at the floor and his look with it, 65 degrees below level: the defender was outside his cone, and the squad that
//! knelt because Sam crouched watched him for three seconds and lost two men (defender down at 3.18, 3.30, 3.45 s on seeds 1 to 3, two of four
//! up). Standing, the same squad dropped him in 0.6 s. A pose is not meant to take a man's sight; the poses that are say so by name.
use game::character::{Stance, UpperMode};
use game::orders::SquadOrdersAssignA;
use game_start::play::{Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 3] = [1, 2, 3];
const NORTH: f64 = std::f64::consts::PI;
/// the longest any of these fights took with the look level (0.77 s), with room
const SOON: f64 = 1.2;

enum How {
    /// the squad on the order it is dealt (fall in), Sam on his feet
    Dealt,
    /// the same, and Sam crouches: the men following him go down with him
    SamCrouched,
    /// every man told to stand where he is in this pose (no order of the menu gives the first of these today: it is the pose a new order would)
    Posed(Stance, UpperMode, bool),
}

/// The fight: when the defender fell, and how many of the four are still up then.
fn fight(seed: u32, how: How) -> (Option<f64>, usize, String) {
    let mut s = Scene::new("eyes");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);   // (a room clear wants a room with two ways in, and a man in each)
    s.door("west-yard", Axis::X, 16.0, 13.4).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    s.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    s.team([14.0, 14.5], NORTH, &[[12.0, 15.0], [13.0, 15.0], [15.5, 15.0], [16.5, 15.0]]);
    s.defender("west", [12.25, 11.0], "west-yard").defender("east", [24.25, 12.25], "east-yard");
    let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
    p.g.god_mode = true;   // (Sam is not what is measured)
    let (men, foe) = (p.mates(), p.hostiles_by([12.25, 11.0])[0]);
    p.frame();
    p.frame();
    match how {
        How::Dealt => {}
        How::SamCrouched => { p.key_down("KeyC"); p.frame(); p.key_up("KeyC"); }
        How::Posed(stance, upper, crouch) => for &gd in &men {
            let at = p.at(gd);
            let face = p.facing(gd);
            game::orders::squad_orders_assign(&mut p.g, gd, &SquadOrdersAssignA {
                order: String::from("hold"), post: Some([at[0], 0.0, at[1]]), face: Some(face), stance: Some(stance), upper: Some(upper), crouch: Some(crouch),
                going: String::from("holding"), there: String::from("holding"), ..Default::default()
            });
        },
    }
    let fell = p.until(5.0, |p| !p.up(foe));
    (fell, men.iter().filter(|m| p.up(**m)).count(), p.roll())
}

fn quick_and_whole(name: &str, how: fn() -> How) {
    for s in SEEDS {
        let (fell, up, roll) = fight(s, how());
        assert!(matches!(fell, Some(t) if t < SOON), "{name}, seed {s}: the defender four metres in front of them fell at {fell:?} ({roll})");
        assert_eq!(up, 4, "{name}, seed {s}: a man was lost to a defender they all had in front of them ({roll})");
    }
}

/// THE RULE: the squad that kneels because Sam crouched still sees the man in the room with it.
/// A FAILURE MEANS: a kneeling man's look has followed his chest to the floor again (three seconds, two men lost).
#[test]
fn the_squad_that_kneels_with_sam_still_sees_the_man_in_the_room() { quick_and_whole("Sam crouched", || How::SamCrouched); }

/// THE CONTROL: on its feet the squad is as quick as it was.
#[test]
fn the_squad_on_its_feet_sees_him_as_it_did() { quick_and_whole("as dealt", || How::Dealt); }

/// THE RULE, pose by pose: down with the gun low, down with the gun up, standing with the gun low. None takes a man's sight.
#[test]
fn no_pose_takes_a_mans_sight() {
    quick_and_whole("down, gun low", || How::Posed(Stance::LowReady, UpperMode::None, true));
    quick_and_whole("down, gun up", || How::Posed(Stance::None, UpperMode::Aim, true));
    quick_and_whole("standing, gun low", || How::Posed(Stance::LowReady, UpperMode::None, false));
}
