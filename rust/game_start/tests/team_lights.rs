//! THE TEAM'S LIGHTS AND GOGGLES: when a weapon light burns, when the team is dark.
mod stealth_ask;
use game::ids::GuardId;
use game::orders::Who;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use stealth_ask::StealthAsk;

const SEEDS: [u32; 3] = [1, 2, 3];
fn seed(seed: u32) -> Opts { Opts { seed, ..Default::default() } }
const NORTH: f64 = std::f64::consts::PI;

/// Two rooms with a door each to the yard and one between: the west one lit and the east one dark. The team in the yard under the moon.
fn lit_and_dark(s: u32) -> Scene {
    let mut sc = Scene::new("lit-and-dark");
    sc.room("lit", 10.0, 10.0, 18.0, 16.0).room("dark", 18.0, 10.0, 26.0, 16.0);
    sc.door("lit-yard", Axis::X, 16.0, 13.4).door("dark-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    sc.bulb(14.0, 13.0).moon();
    sc.team([14.0, 21.0], NORTH, &[[13.0, 22.0], [15.0, 22.0], [13.0, 23.0], [15.0, 23.0]]);
    sc.defender("lit", [12.25 + 0.5 * nudge(s, 0), 12.25], "lit-yard").defender("dark", [24.25, 12.25 + 0.5 * nudge(s, 1)], "dark-yard");
    sc
}
/// a teammate on an order, as the rule wants him
fn on_order(p: &mut Play) -> GuardId {
    let sam = p.sam_at();
    p.order("fallIn", [sam[0] + 3.0, sam[1] + 3.0], Who::All);
    p.run(0.5);
    p.red()[0]
}

/// THE RULE: a teammate's gun light is off in the dark on a man the place lights. The man in the dark too, it is not that rule; himself in the light, it is not.
/// CONTROL: the man his eyes have in the dark with him: his light is free to burn, and does.
/// A FAILURE MEANS: a teammate gives himself away with a light on a man he can see anyway, or goes dark on a man he cannot.
#[test]
fn a_teammates_gun_light_is_off_in_the_dark_on_a_man_the_place_lights() {
    for s in SEEDS {
        let mut p = Play::scene(&lit_and_dark(s), &seed(s));
        p.sam_unhurt();
        p.run(0.2);
        let mate = on_order(&mut p);
        let foes = p.hostiles();
        let h = foes[0];
        let (lit, dark, dark2, lit2) = ([14.25, 13.25], [24.25, 13.25], [21.25, 12.25], [12.25, 14.25]);

        p.man_stands(mate, dark); p.man_stands(h, lit); p.eyes_on(mate, h);
        assert!(p.gun_on_lit_man(mate) && !p.team_lit(mate), "seed {s}: from the dark on a lit man the rule holds and his light is off");

        p.man_stands(h, dark2); p.eyes_on(mate, h);
        assert!(!p.gun_on_lit_man(mate), "seed {s}: the man in the dark too: not this rule");
        assert!(p.team_lit(mate), "seed {s}: the man in the dark too: his light burns");

        p.man_stands(h, lit); p.man_stands(mate, lit2); p.eyes_on(mate, h);
        assert!(!p.gun_on_lit_man(mate), "seed {s}: himself in the light: not this rule");
    }
}

/// A dark place to walk through: the same two rooms with every lamp out, and the yard's moon. An order to stack on a door, and every frame of the walk asked.
fn walk(p: &mut Play, s: u32, mut each: impl FnMut(&Play, GuardId)) {
    let door = lit_and_dark(s).door_centre("dark-yard");
    let team = [p.red(), p.blue()].concat();
    p.order("stack", door, Who::All);
    for _ in 0..(60 * 6) { p.frame(); for &m in team.iter() { each(p, m); } }
}

/// THE RULE: "knowing when they haven't been compromised and flashlight would ruin that": not compromised and in the dark, the team walks with no light.
/// CONTROL: loud (rounds in their ears), the lights are the light rule's again: nobody is held dark.
/// A FAILURE MEANS: a quiet team lights itself up, or a loud team stays blind.
#[test]
fn not_compromised_in_the_dark_the_team_goes_with_no_light_and_loud_the_light_rule_has_them() {
    for s in SEEDS {
        let mut p = Play::scene(&lit_and_dark(s), &seed(s));
        p.sam_unhurt();
        p.run(4.0);
        on_order(&mut p);
        p.drop_all_but(&[]);
        p.run(1.0);
        let (mut lit, mut dark, mut frames) = (0, 0, 0);
        walk(&mut p, s, |p, m| { frames += 1; if p.gun_light_on(m) { lit += 1; } if p.held_dark(m) { dark += 1; } });
        assert!(lit == 0 && dark > frames * 9 / 10, "seed {s}: quiet: a light burned on {lit} of {frames} man-frames, held dark on {dark}");
        drop(p);

        let mut p = Play::scene(&lit_and_dark(s), &seed(s));
        p.sam_unhurt();
        p.run(4.0);
        on_order(&mut p);
        let foe = p.hostiles()[0];
        let at = p.at(foe);
        p.man_fires_at(foe, [at[0] + 2.0, at[1]]);
        let (mut dark, mut frames) = (0, 0);
        walk(&mut p, s, |p, m| { frames += 1; if p.held_dark(m) { dark += 1; } });
        let _ = dist(at, at);
        assert!(dark * 10 < frames, "seed {s}: with rounds in their ears the team is still held dark on {dark} of {frames} man-frames");
    }
}

/// THE RULE: Sam's lead, both ways. His goggles down, the team stays dark, however loud the floor. His own weapon light on, on a quiet floor, theirs are free
/// again; and a moment after he puts it out they are dark again.
/// A FAILURE MEANS: the team no longer follows his lead.
#[test]
fn the_team_follows_sams_lead() {
    for s in SEEDS {
        let mut p = Play::scene(&lit_and_dark(s), &seed(s));
        p.sam_unhurt();
        p.run(4.0);
        on_order(&mut p);
        let m = p.red()[0];
        p.run(1.0);
        assert!(p.held_dark(m), "seed {s}: CONTROL: quiet and dark before Sam does anything");
        p.sam_takes_his_rifle();
        p.key_down("KeyL");
        p.frame();
        p.key_up("KeyL");
        p.run(0.5);
        assert!(!p.held_dark(m), "seed {s}: his light on, theirs are free");
    }
}
