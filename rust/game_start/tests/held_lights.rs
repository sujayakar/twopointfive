//! LIGHT AND THE WALL: a gun light pressed to a wall stays on this side of it, a teammate's goes dark there; a room's lights go by the switch on its wall.
mod kit_ask;
use game::game::{InteractableKind as Kind, Verb};
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use kit_ask::KitAsk;

/// THE RULE: Sam pressed to a wall with his gun's light on: the light stands on his side of it, the whole emitter clear of the face. A metre and a half back (the control) it stands well out from his shoulder at the lens.
/// A FAILURE MEANS: a torch shines from the far side of a wall, and lights a room its carrier has not reached.
#[test]
fn a_gun_light_pressed_to_a_wall_is_on_this_side_of_it() {
    for s in [1, 2, 3] {
        let mut sc = Scene::new("light-wall");
        let z = 13.0 + 0.5 * nudge(s, 0);
        sc.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0).door("between", Axis::Z, 18.0, 10.4).door("south", Axis::X, 16.0, 13.0);
        sc.sam(17.4, z);
        let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
        p.sam_unhurt();
        p.sam_a_ghost();
        p.sam_arms("pistol");
        p.sam_put([17.4, z]);
        p.point_at([25.0, z]);
        p.run(0.5);
        if !p.gun_light().2 { p.tap("KeyL"); p.run(0.5); }
        let (at, r, lit) = p.gun_light();
        assert!(lit, "seed {s}: the staging: his gun's light is not on");
        let (shoulder, face) = (p.sam_shoulder(), 18.0 - 0.06);
        assert!(at[0] + r <= face + 1e-6, "seed {s}: pressed to the wall the emitter's edge is at x {:.3}, the face at {face:.3}", at[0] + r);
        assert!(!p.solid_between(shoulder, at), "seed {s}: something solid stands between his shoulder and his light");
        let pressed = shoulder[0].max(at[0]) - shoulder[0];
        p.sam_put([15.9, z]);
        p.run(0.3);
        let (back, _, _) = p.gun_light();
        let free = back[0] - p.sam_shoulder()[0];
        assert!(free > pressed + 0.02, "seed {s}: the control: back from the wall the light is {free:.3} m out from his shoulder, pressed {pressed:.3}");
    }
}

/// THE RULE: a teammate's torch goes dark into a wall at his muzzle, and burns again back from it.
/// A FAILURE MEANS: every man of the squad stacked on a wall lights a white disc on it for the room to see.
#[test]
fn a_teammates_light_goes_dark_at_a_wall() {
    for s in [1, 2, 3] {
        let mut sc = Scene::new("mate-light");
        let z = 22.0 + 0.5 * nudge(s, 0);
        sc.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
        sc.door("west-yard", Axis::X, 16.0, 13.4).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
        sc.wall_z(30.0, 18.0, 27.0);
        sc.team([3.0, 27.0], std::f64::consts::FRAC_PI_2, &[[6.0, 26.0], [8.0, 26.0], [6.0, 27.0], [8.0, 27.0]]);
        sc.defender("west", [12.25, 12.25], "west-yard").defender("east", [24.25, 12.25], "east-yard");
        let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
        p.sam_unhurt();
        p.sam_a_ghost();
        p.run(0.2);
        let man = p.mates()[0];
        let east = std::f64::consts::FRAC_PI_2;
        for _ in 0..60 { p.pin_man(man, [29.2, z], east, game::game::GuardState::Patrol, 0.0); p.frame(); }
        assert!(p.wall_has_his_light(man), "seed {s}: a man's torch into the wall at his muzzle burns");
        let mut dark = true;
        for _ in 0..60 { p.pin_man(man, [24.0, z], east, game::game::GuardState::Patrol, 0.0); p.frame(); dark = p.wall_has_his_light(man); }
        assert!(!dark, "seed {s}: the control: back from the wall his torch stays dark");
    }
}

/// THE RULE: a room's lights go by the switch on its wall: at the plate the row says lights off, the tap has the room dark to the light estimate, and the tap again has it as the level left it. Beside the door the tap is still the door's.
/// A FAILURE MEANS: a room's light no longer answers its switch, and a player cannot take a room dark without a gun.
#[test]
fn a_rooms_lights_go_by_the_switch_on_its_wall() {
    let mut p = Play::level("office.json", false, &Opts { seed: 1, ..Default::default() });
    p.sam_unhurt();
    p.sam_a_ghost();
    p.no_wits();
    p.run(0.3);
    let sw = p.switch_near([28.0, 8.0]).expect("the manager's room has a plate");
    let t = sw.targets[0];
    let lamp = p.g.tab.sim_lightshome_fixture[t].pos;
    let under = [lamp[0], 0.9, lamp[2] + 0.4];
    p.sam_put([sw.stand[0], sw.stand[2]]);
    p.run(0.1);
    let row_off = (Kind::Switch, Some(Verb::LightsOff));
    assert!(p.rows().contains(&row_off), "at the plate: {:?}", p.rows());
    let lit = p.light_at(under);
    assert!(p.switch_on(&sw) && lit > 0.5, "the staging: the room is not lit: {lit:.2}");
    assert!(game::game::game_select_verb(&mut p.g, Kind::Switch, Some(Verb::LightsOff)));
    p.tap("Space");
    p.run(0.1);
    assert!(!p.switch_on(&sw), "the tap left the room lit");
    assert!(p.light_at(under) < lit * 0.05, "the room is dark to nobody: {:.2} of {lit:.2}", p.light_at(under));
    assert!(p.rows().contains(&(Kind::Switch, Some(Verb::LightsOn))), "the row did not turn to lights on: {:?}", p.rows());
    assert!(game::game::game_select_verb(&mut p.g, Kind::Switch, Some(Verb::LightsOn)));
    p.tap("Space");
    p.run(0.1);
    assert!(p.switch_on(&sw) && p.light_at(under) > lit * 0.9, "the tap again: lit {}, {:.2} of {lit:.2}", p.switch_on(&sw), p.light_at(under));
    let (c, s) = (p.frame_mid(sw.door), [sw.stand[0], sw.stand[2]]);
    let between = [(c[0] + s[0]) / 2.0 + sw.n[0] * 0.5, (c[1] + s[1]) / 2.0 + sw.n[2] * 0.5];
    p.sam_put(between);
    p.run(0.1);
    assert!(dist(p.sam_at(), between) < 0.3, "the staging: he was not set down between them");
    assert_eq!(p.rows().first().map(|r| r.0), Some(Kind::Door), "between the plate and the door the tap is the door's: {:?}", p.rows());
}
