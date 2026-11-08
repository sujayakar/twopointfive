//! HIS OWN CHARGES: a charge going on has both his hands, the count is his card's, and it goes off from his own actions and not the squad's key.
mod kit_ask;
use game::game::{InteractableKind as Kind, Verb};
use game::orders::Who;
use game_start::play::{Opts, Play};
use game_start::scene::{Axis, Scene};
use kit_ask::KitAsk;

/// Alone in the office with `n` charges on his card, nobody to see or hear him: the door, Sam on its corridor side.
fn alone(door: &str, n: f64, s: u32) -> (Play, game::ids::DoorId, f64) {
    let mut p = Play::level("office.json", false, &Opts { seed: s, ..Default::default() });
    p.sam_a_ghost();
    p.sam_unhurt();
    p.g.ai_enabled = false;
    p.run(0.2);
    p.alone_with_charges(n);
    let d = p.door(door);
    let at = game::doortools::door_pt(&p.g, d, 0.4, 1.0, 1.0, None);
    let side = if at[2] < p.g.dr[d].frame_centre[2] { 1.0 } else { -1.0 };   // (the corridor lies north of the rooms on the level's south side)
    (p, d, side)
}
/// The row given and held to its end; the frames his hands had nothing in them out of all of them.
fn sets(p: &mut Play) -> (usize, usize) {
    assert!(p.takes_door_row(Verb::Charge), "no door row sets a charge: {:?}", p.rows());
    p.until(1.0, |p| p.sam_charging()).unwrap_or_else(|| panic!("the row began no job: {:?}", p.log()));
    let (mut away, mut all) = (0, 0);
    while p.sam_charging() && all < 8 * 60 { p.frame(); all += 1; if p.sam_charging() && p.in_hand() == "nothing" { away += 1; } }
    (away, all)
}

/// THE RULE: "characters should fully unequip their weapon when planting door charges, etc. -- they can reequip after they're done": what he held is away from the row to the charge set, both hands are on it, it is
/// back after, and a click on the frame the charge is set fires nothing. Once it is up the trigger is his. Hands empty at the start (the control) stay empty.
/// A FAILURE MEANS: the charge goes on one-handed with a gun in the other, or the gun does not come back.
#[test]
fn a_charge_going_on_has_both_his_hands_and_what_he_held_comes_back() {
    for (s, gun) in [(11, "pistol"), (12, "rifle"), (13, "nothing")] {
        let (mut p, d, side) = alone("breakroom_w", 2.0, s);
        p.sam_at_door(d, side, 0.75);
        p.sam_arms(gun);
        assert_eq!(p.in_hand(), gun, "the staging: his hand");
        p.sam_at_door(d, side, 0.75);
        let rounds = p.sam_rounds();
        assert!(p.takes_door_row(Verb::Charge), "{gun}: no door row sets a charge: {:?}", p.rows());
        p.until(1.0, |p| p.sam_charging()).unwrap_or_else(|| panic!("{gun}: the row began no job"));
        let (mut two, mut bare) = (false, false);
        p.until(8.0, |p| { two |= p.two_hands_on_work(); bare |= p.in_hand() == "nothing"; !p.sam_charging() }).unwrap_or_else(|| panic!("{gun}: the job never ended"));
        assert!(two && bare, "{gun}: both hands on the job {two}, the hand empty for it {bare}");
        assert_eq!(p.seat_door_charges(0), 1, "{gun}: the charge is not on the door");
        p.press(game_start::play::Button::Left);
        p.frame();
        p.release(game_start::play::Button::Left);
        p.until(3.0, |p| p.in_hand() == gun).unwrap_or_else(|| panic!("{gun}: {} in his hand 3 s after", p.in_hand()));
        assert_eq!(p.sam_rounds(), rounds, "{gun}: the click on the frame the charge was set fired a round");
    }
}

/// THE RULE: alone, the count is his card's: none on the card, no door offers one; two, a door's row sets one and the bag holds one fewer. A step breaks the job off with nothing spent, and the gun back.
/// A FAILURE MEANS: a solo player gets the squad's bag, or a broken job costs him a charge.
#[test]
fn alone_the_count_is_his_cards_and_a_step_breaks_the_job_off() {
    let (mut p, d, side) = alone("breakroom_w", 0.0, 21);
    p.sam_at_door(d, side, 0.75);
    assert!(!p.rows().contains(&(Kind::Door, Some(Verb::Charge))), "none on the card and the door offers one: {:?}", p.rows());
    p.alone_with_charges(2.0);
    p.run(0.1);
    assert!(p.rows().contains(&(Kind::Door, Some(Verb::Charge))), "two on the card and no door row: {:?}", p.rows());
    p.sam_arms("pistol");
    p.sam_at_door(d, side, 0.75);
    assert!(p.takes_door_row(Verb::Charge), "no door row sets a charge");
    p.run(0.5);
    assert!(p.sam_charging(), "the job did not start");
    p.tap("KeyS");
    p.key_down("KeyS");
    p.run(0.4);
    p.key_up("KeyS");
    assert!(!p.sam_charging(), "a step did not break it off");
    assert_eq!((p.charges_left(), p.seat_door_charges(0)), (2.0, 0), "the count and what is set after a job broken off");
    p.until(2.0, |p| p.in_hand() == "pistol").unwrap_or_else(|| panic!("the pistol did not come back: {}", p.in_hand()));
    p.sam_at_door(d, side, 0.75);
    let _ = sets(&mut p);
    assert_eq!(p.seat_door_charges(0), 1, "the charge is not on the door");
}

/// THE RULE: his own charge goes off from his own actions: the squad's key with only his own set does nothing, and the interaction key tapped, stood off, is one blast and the door in. The squad's own charge (the control)
/// goes on the squad's key, as it did.
/// A FAILURE MEANS: F blows a charge the player stands next to, or his charge has no way to go.
#[test]
fn his_own_charge_goes_off_from_his_own_actions_and_the_squads_on_the_squads_key() {
    for s in [31, 32, 33] {
        let (mut p, d, side) = alone("manager", 2.0, s);
        p.sam_at_door(d, side, 0.75);
        let _ = sets(&mut p);
        p.run(0.05);
        assert_eq!(p.seat_door_charges(0), 1, "seed {s}: the charge is not on the door: {:?}", p.log());
        let w = p.g.dr[d].def.width;
        let off = game::doortools::door_pt(&p.g, d, w / 2.0 + 2.6, 1.0, side, None);
        p.sam_put([off[0], off[2]]);
        p.run(0.15);
        p.tap("KeyF");
        p.run(0.3);
        assert_eq!(p.seat_door_charges(0), 1, "seed {s}: the squad's key took his own charge");
        assert!(!p.door_in(d), "seed {s}: the squad's key blew the door");
        assert_eq!(p.rows().first(), Some(&(Kind::Charge, Some(Verb::Blow))), "seed {s}: stood off with nothing else in reach, his row is 'blow the charge': {:?}", p.rows());
        p.tap("Space");
        p.run(0.1);
        assert_eq!(p.seat_door_charges(0), 0, "seed {s}: the tap did not spend the charge");
        assert!(p.door_in(d), "seed {s}: the tap did not put the door in");
    }
    // (two rooms with a door each from the yard and one between them; the pair sets the charge on order, and it is left to his word)
    let mut sc = Scene::new("yard");
    sc.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    sc.door("west-yard", Axis::X, 16.0, 13.4).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    sc.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    sc.team([14.0, 21.0], std::f64::consts::PI, &[[13.0, 22.0], [15.0, 22.0], [13.0, 23.0], [15.0, 23.0]]);
    sc.defender("west", [12.25, 12.25], "west-yard").defender("east", [24.25, 12.25], "east-yard");
    let mut p = Play::scene(&sc, &Opts { seed: 5, ..Default::default() });
    p.sam_unhurt();
    p.run(0.3);
    let d = p.door("west-yard");
    p.order("charge", sc.door_centre("west-yard"), Who::All);
    p.until(40.0, |p| p.seat_door_charges(0) > 0).unwrap_or_else(|| panic!("the control: the squad never set it: {}", p.roll()));
    p.run(1.0);
    p.tap("KeyF");
    p.until(5.0, |p| p.door_in(d)).unwrap_or_else(|| panic!("the control: the squad's key did not blow the squad's charge: {}", p.roll()));
}
