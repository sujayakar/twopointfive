//! THE BIG SLOT: the breaching shotgun and the hammer open a door from beside the frame, for a teammate on the squad's row and for Sam.
mod kit_ask;
use game::game::{InteractableKind as Kind, Verb};
use game::kit::BigKit;
use game::orders::Who;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use kit_ask::KitAsk;

/// The yard: the squad outside the shut west door, every man's big slot as `big` says and every defender but one down far off.
fn the_door(big: BigKit, s: u32) -> (Play, Scene, game::ids::DoorId) {
    let mut sc = Scene::new("yard");
    sc.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    sc.door("west-yard", Axis::X, 16.0, 13.4 + 0.5 * nudge(s, 0)).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    sc.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    sc.team([14.0, 23.5], std::f64::consts::PI, &[[13.0, 21.5], [15.0, 21.5], [13.0, 22.5], [15.0, 22.5]]);
    sc.defender("west", [12.25, 12.25], "west-yard").defender("east", [24.25, 12.25], "east-yard");
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.sam_a_ghost();
    p.run(0.2);
    p.drop_all_but(&[]);
    for m in p.mates() { p.man_carries(m, BigKit::None); }
    let lead = p.mates()[0];
    p.man_carries(lead, big);
    p.run(0.1);
    let d = p.door("west-yard");
    (p, sc, d)
}

/// THE RULE: "these things should work for unlocked doors too": the squad's row on a door with a hammer aboard: one blow, from beside the frame and not in the opening, no shot, the door ends open. With the shotgun aboard it is a shell, and the man has his own gun back after.
/// The control: nobody carries one, and the game refuses the row.
/// A FAILURE MEANS: the tool does not reach the door, the man stands in the fatal funnel to use it, or he keeps a shotgun in his hands for the fight.
#[test]
fn the_squads_tool_opens_a_door_from_beside_the_frame() {
    for s in [1, 2, 3] {
        let (mut p, sc, d) = the_door(BigKit::Hammer, s);
        let c = sc.door_centre("west-yard");
        p.order("bigClear", c, Who::All);
        let lead = p.mates()[0];
        let (mut in_frame, mut at_blow) = (false, f64::MAX);
        p.until(30.0, |p| { let (_, _, blows, _) = p.big_counts(); if blows > 0 && at_blow == f64::MAX { at_blow = dist(p.at(lead), c); in_frame = (p.at(lead)[0] - c[0]).abs() < 0.35; } p.leaf(d).abs() > 0.5 }).unwrap_or_else(|| panic!("seed {s}: 30 s and the hammer did not open the door: {}", p.roll()));
        let (_, _, blows, hammered) = p.big_counts();
        assert_eq!((blows, hammered), (1, 1), "seed {s}: blows, doors hammered");
        assert!(!in_frame && at_blow < 2.0, "seed {s}: he stood in the opening for the blow, or was nowhere near: {at_blow:.2} m from its middle");
        assert_eq!(p.rounds_fired(lead), 0.0, "seed {s}: the hammer fired a round");
        drop(p);

        let (mut p, sc, d) = the_door(BigKit::Shotgun, s);
        let lead = p.mates()[0];
        p.order("bigClear", sc.door_centre("west-yard"), Who::All);
        p.until(30.0, |p| p.leaf(d).abs() > 0.5).unwrap_or_else(|| panic!("seed {s}: 30 s and the shotgun did not open the door: {}", p.roll()));
        p.until(6.0, |p| p.own_gun_up(lead)).unwrap_or_else(|| panic!("seed {s}: 6 s after the door and his own gun is not back: {}", p.why(lead)));
        drop(p);

        let (mut p, sc, _) = the_door(BigKit::None, s);
        let c = sc.door_centre("west-yard");
        let t = game::orders::squad_orders_target_at(&mut p.g, [c[0], 0.0, c[1]]);
        assert!(game::orders::squad_orders_give(&mut p.g, "bigClear", Some(&t), Some(Who::All), None).is_some(), "seed {s}: the control: nobody carries one and the row was taken");
    }
}

/// Alone with a hammer on him at a locked door between two rooms, on the face `pull` says.
fn alone_at_a_locked_door(pull: bool, tool: BigKit, s: u32) -> (Play, game::ids::DoorId) {
    let mut sc = Scene::new("locked");
    sc.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    sc.door("between", Axis::Z, 18.0, 12.4 + 0.4 * nudge(s, 0)).door("yard", Axis::X, 16.0, 21.4).locked("between");
    sc.bulb(14.0, 13.0).bulb(22.0, 13.0);
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.sam_a_ghost();
    p.no_wits();
    p.run(0.1);
    p.sam_carries(tool);
    let d = p.door("between");
    let w = p.leaf_w(d);
    p.sam_put(p.leaf_pt(d, w / 2.0, if pull { 0.75 } else { -0.75 }));
    p.run(0.2);
    (p, d)
}

/// THE RULE: the row stands at a locked door in the boot's place: one blow where a boot would do, the frame's count of blows where the frame holds a boot, no round; and his own hand is back after. With nothing in the slot (the control) no door offers it.
/// A FAILURE MEANS: the hammer does nothing where a boot does nothing, which is the one place it is carried for.
#[test]
fn his_hammer_takes_a_locked_door_in_one_blow_or_two_where_the_frame_holds_a_boot() {
    for s in [1, 2, 3] {
        for (pull, want) in [(false, 1.0), (true, 0.0)] {
            let (mut p, d) = alone_at_a_locked_door(pull, BigKit::Hammer, s);
            let want = if pull { p.frame_blows() } else { want };
            assert!(p.rows().contains(&(Kind::Door, Some(Verb::Hammer))), "seed {s}, pull {pull}: no row: {:?}", p.rows());
            let rounds = p.sam_rounds();
            assert!(game::game::game_select_verb(&mut p.g, Kind::Door, Some(Verb::Hammer)));
            p.tap("Space");
            p.until(6.0, |p| !p.g.dr[d].leaf.locked).unwrap_or_else(|| panic!("seed {s}, pull {pull}: 6 s and the lock holds: {}", p.roll()));
            p.run(0.5);
            assert_eq!(p.big_counts().0 as f64, want, "seed {s}, pull {pull}: blows");
            assert_eq!(p.sam_rounds(), rounds, "seed {s}, pull {pull}: the hammer fired");
            drop(p);
        }
        let (p, _) = alone_at_a_locked_door(false, BigKit::None, s);
        assert!(!p.rows().contains(&(Kind::Door, Some(Verb::Hammer))), "seed {s}: the control: nothing in the slot and the door offers the hammer");
    }
}
