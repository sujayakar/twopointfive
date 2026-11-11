//! WHAT A LAMP DOES TO A POINT: how far it reaches, what stops its light and what does not, which lamps a question leaves out, and the sky's and a stroke's share of it.
mod world_ask;
use game::character::Bone;
use game::guards::light_at;
use game::ids::LightId;
use game::sense::light_asked;
use game_start::play::{Opts, Play};
use game_start::scene::{Axis, Scene};
use world_ask::{a_box, scene_with};

/// A lot at night under no moon, a hut out of the way in its corner: open ground, black until a test brings a lamp.
fn lot() -> Scene {
    let mut s = Scene::new("lot");
    s.room("hut", 30.0, 20.0, 36.0, 26.0).door("hut-door", Axis::X, 20.0, 32.0).sam(4.0, 24.0);
    s
}
fn stand(s: &Scene) -> Play {
    let mut p = Play::scene(s, &Opts::default());
    p.sam_unhurt();
    p.sam_unseen();
    p.run(0.1);
    p
}
/// A lamp nobody's fixture feeds, by the call that puts a man's torch in the list: 0 a point, 1 a spot, 3 a panel.
fn lamp(p: &mut Play, kind: f64, at: [f64; 3], dir: [f64; 3], range: f64, intensity: f64, outer: f64) -> LightId {
    let mut l = game::render_lights::RtLight::default();
    l.name = String::from("test lamp");
    l.group = String::from("test");
    l.kind = kind;
    l.pos = at;
    l.dir = dir;
    l.color = [1.0, 1.0, 1.0];
    l.intensity = intensity;
    l.peak_intensity = intensity;
    l.range = range;
    l.inner_deg = outer - 1.0;
    l.outer_deg = outer;
    l.radius = 0.05;
    l.enabled = true;
    l.fixture_box = -1.0;
    l.ttl = -1.0;
    let id = p.g.tab.light.push(l);
    game::render_lights::light_set_add(&mut p.g, id)
}
/// The light that stands on a point, all of it.
fn light(p: &mut Play, at: [f64; 3]) -> f64 { light_at(&mut p.g, at, None, None, None).0 }
/// What the lamps and the sky give a point by their own lines, without what comes round off the walls: a question that leaves out a kind of lamp asks only this, and no beacon stands in these scenes.
fn direct(p: &mut Play, at: [f64; 3]) -> f64 { light_asked(&mut p.g, at, &[], false, sim::lights::R_PASSING as f64) }

/// THE RULE: a lamp's light dies away before the end of its reach: just inside it there is next to nothing, not the most of what a lamp that far off would give, and past it nothing.
/// A FAILURE MEANS: a lamp's light stops at an edge on the floor, and a man one step over that edge is lit or not by a hard line nobody can see.
#[test]
fn a_lamps_light_dies_away_before_the_end_of_its_reach() {
    let mut p = stand(&lot());
    let (x, y, z, range) = (15.0, 1.5, 10.0, 6.0);
    let dark = light(&mut p, [x + 3.0, y, z]);
    lamp(&mut p, 0.0, [x, y, z], [0.0, -1.0, 0.0], range, 40.0, 180.0);
    let near = light(&mut p, [x + 3.0, y, z]) - dark;
    let edge = light(&mut p, [x + 0.98 * range, y, z]) - dark;
    let past = light(&mut p, [x + 1.1 * range, y, z]) - dark;
    assert!(near > 1.0, "the staging: three metres from the lamp {near:.3}");
    let square_law = near * (3.0f64 / (0.98 * range)).powi(2);
    assert!(edge < 0.1 * square_law, "just inside its reach the lamp gives {edge:.4}; the square of the distance alone would leave {square_law:.4}");
    assert!(past.abs() < 1e-9, "past its reach the lamp gives {past:.4}");
}

/// THE RULE: a panel lights what is in front of it and nothing behind it. CONTROL: the same panel turned round lights what was behind it.
/// A FAILURE MEANS: a lit screen or sign lights a man standing behind it as if he stood before it.
#[test]
fn a_panel_lights_what_is_in_front_of_it_and_not_its_back() {
    let mut p = stand(&lot());
    let (at, front, back) = ([15.0, 1.5, 10.0], [18.0, 1.5, 10.0], [12.0, 1.5, 10.0]);
    let dark = light(&mut p, front);
    let l = lamp(&mut p, 3.0, at, [1.0, 0.0, 0.0], 10.0, 40.0, 90.0);
    let (lit, behind) = (light(&mut p, front) - dark, light(&mut p, back) - dark);
    assert!(lit > 1.0, "the staging: in front of the panel {lit:.3}");
    assert!(behind < 0.01 * lit, "behind the panel {behind:.3}, in front of it {lit:.3}");
    p.g.tab.light[l].dir = [-1.0, 0.0, 0.0];
    assert!(light(&mut p, back) - dark > 0.9 * lit, "the control: the panel turned round does not light what is now in front of it");
}

/// THE RULE: a shut door stops a lamp's light, and open it lets the light through: a line from the bulb through the doorway to a man in the next room.
/// A FAILURE MEANS: a door between two rooms hides nobody: a man behind a shut door stands in the light of the room beyond it.
#[test]
fn a_shut_door_stops_a_lamps_light_and_an_open_one_lets_it_through() {
    let mut s = Scene::new("door-between");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("between", Axis::Z, 18.0, 12.4).door("yard", Axis::X, 16.0, 14.0);
    s.sam(4.0, 24.0);
    let mut p = stand(&s);
    let leaf = game::doors::doors_by_name(&p.g, "between").expect("the door");
    let c = p.g.dr[leaf].frame_centre;
    // (a line through the doorway on the side the leaf does not swing to)
    let at = [20.5, 1.0, c[2] + 0.2];
    lamp(&mut p, 0.0, [15.5, 1.0, c[2] + 0.2], [0.0, -1.0, 0.0], 9.0, 40.0, 180.0);
    let shut = direct(&mut p, at);
    // (from the side it opens away from: from the other a boot only breaks the lock)
    let from = if game::doors::doors_boot_holds(&p.g, leaf, [c[0] - 1.0, 0.0, c[2]]) { [c[0] + 1.0, 0.0, c[2]] } else { [c[0] - 1.0, 0.0, c[2]] };
    game::doors::doors_kick_in(&mut p.g, leaf, from, 0.0, None);
    p.run(4.0);
    assert!(p.g.dr[leaf].leaf.angle.abs() > 0.8, "the staging: the door was to stand open");
    let open = direct(&mut p, at);
    assert!(open > 0.5, "the control: through the open door {open:.3}");
    assert!(shut < 0.02 * open, "behind the shut door {shut:.3}, through the open one {open:.3}");
}

/// THE RULE: a pane of glass lets a lamp's light through. CONTROL: a slab of wall of the same size in the same place stops it.
/// A FAILURE MEANS: a man behind a window is hidden from the lamp that shines through it, and a window lets the eye through into a dark the picture does not show.
#[test]
fn a_pane_lets_a_lamps_light_through_and_a_wall_does_not() {
    let (lamp_at, at) = ([12.0, 1.5, 10.0], [18.0, 1.5, 10.0]);
    let read = |flags: &[&str]| {
        let mut sc = lot();
        sc.sam(4.0, 24.0);
        let mut p = scene_with(&sc, vec![a_box("slab", [15.0, 1.5, 10.0], [0.03, 1.5, 2.0], "white", flags)], &Opts::default());
        p.sam_unhurt();
        p.run(0.1);
        let dark = light(&mut p, at);
        lamp(&mut p, 0.0, lamp_at, [0.0, -1.0, 0.0], 9.0, 40.0, 180.0);
        light(&mut p, at) - dark
    };
    let (through_glass, through_wall) = (read(&["Glass"]), read(&[]));
    assert!(through_glass > 0.5, "through the pane {through_glass:.3}");
    assert!(through_wall < 0.02 * through_glass, "through a wall {through_wall:.3}, through a pane {through_glass:.3}");
}

/// THE RULE: smoke between a lamp and a man dims the lamp on him. CONTROL: a man as far off on the lamp's other side, with no smoke on his line, keeps it all.
/// A FAILURE MEANS: a lamp on the far side of a smoke screen goes on lighting a man as if the screen were not there, so he is picked out of a dark he can see is dark.
#[test]
fn smoke_dims_a_lamp_on_a_man_behind_it() {
    // (under a roof, where no wind carries the smoke off)
    let mut s = Scene::new("hall");
    s.room("hall", 6.0, 6.0, 26.0, 16.0).door("hall-door", Axis::X, 16.0, 12.0).sam(4.0, 24.0);
    let mut p = stand(&s);
    let (lamp_at, behind, clear) = ([15.0, 1.2, 10.0], [19.5, 1.2, 10.0], [10.5, 1.2, 10.0]);
    lamp(&mut p, 0.0, lamp_at, [0.0, -1.0, 0.0], 9.0, 40.0, 180.0);
    let (behind0, clear0) = (direct(&mut p, behind), direct(&mut p, clear));
    let e = game::smoke_types::SmokeEmitter { pos: [17.0, 0.2, 10.0], dir: [0.0, 1.0, 0.0], speed: 1.9, radius: 0.2, density: 95.0, temperature: 1.6, ttl: 30.0, ..Default::default() };
    game::smoke_types::smoke_system_like_emit(&mut p.g, &e);
    p.run(6.0);
    let (behind1, clear1) = (direct(&mut p, behind), direct(&mut p, clear));
    assert!(behind0 > 0.5, "the staging: the lamp on him before the smoke {behind0:.3}");
    assert!(behind1 < 0.3 * behind0, "through the smoke {behind1:.3} of {behind0:.3}");
    assert!(clear1 > 0.9 * clear0, "the control: with no smoke on his line {clear1:.3} of {clear0:.3}");
}

/// THE RULE: what the PLACE lights leaves out what passes: a flash on a point counts for the light on it now, and nothing for what it is lit by when the flash has gone. CONTROL: the steady lamp beside it counts for both.
/// A FAILURE MEANS: a man reads a dark corner as a lit one because a muzzle flashed there, and searches or hides by a light that has gone.
#[test]
fn a_flash_does_not_count_for_what_the_place_lights() {
    let mut p = stand(&lot());
    let at = [15.0, 1.2, 10.0];
    lamp(&mut p, 0.0, [15.0, 1.2, 12.0], [0.0, -1.0, 0.0], 9.0, 4.0, 180.0);
    let (place0, now0) = (light_asked(&mut p.g, at, &[], true, 0.0), light_asked(&mut p.g, at, &[], false, 0.0));
    assert!(place0 > 0.3 && (place0 - now0).abs() < 0.05 * now0, "the control: the steady lamp: {place0:.3} of the place, {now0:.3} now");
    game::render_lights::light_set_flash(&mut p.g, [13.0, 1.2, 10.0], [1.0, 1.0, 1.0], 40.0, 9.0, 0.5, 0.0);
    let (place1, now1) = (light_asked(&mut p.g, at, &[], true, 0.0), light_asked(&mut p.g, at, &[], false, 0.0));
    assert!(now1 > 2.0 * now0, "the flash on the point now: {now1:.3}, without it {now0:.3}");
    assert!((place1 - place0).abs() < 0.05 * place0, "what the place lights moved with a flash: {place1:.3}, without it {place0:.3}");
}

/// THE RULE: a man is as lit as the brighter of his chest and his head: a beam on his head alone lights him. CONTROL: the same beam a metre lower, on neither, does not.
/// A FAILURE MEANS: a man whose head stands up into the light of a window, over a sill or a counter, reads as hidden.
#[test]
fn a_beam_on_his_head_alone_lights_a_man() {
    let mut p = stand(&lot());
    p.sam_put([15.0, 10.0]);
    p.run(0.3);
    let c = p.sam_body();
    let (head, chest) = (p.joint(c, Bone::Head), p.joint(c, Bone::Chest));
    assert!(head[1] - chest[1] > 0.2, "the staging: head at {:.2}, chest at {:.2}", head[1], chest[1]);
    let dark = game::guards::light_on(&mut p.g, c)[0];
    let from = [head[0] - 5.0, head[1], head[2]];
    let aim = |to: [f64; 3]| game::v3::normalize([to[0] - from[0], to[1] - from[1], to[2] - from[2]]);
    let l = lamp(&mut p, 1.0, from, aim(head), 9.0, 40.0, 1.5);
    let on_chest = light(&mut p, chest) - dark;
    let on_head = light(&mut p, head) - dark;
    assert!(on_head > 0.5, "the staging: the beam on his head {on_head:.3}");
    assert!(on_chest < 0.02 * on_head, "the staging: the beam was to miss his chest: {on_chest:.3} of {on_head:.3}");
    let lit = game::guards::light_on(&mut p.g, c)[0] - dark;
    assert!(lit > 0.8 * on_head, "with a beam on his head he stands in {lit:.3}, the head reads {on_head:.3}");
    p.g.tab.light[l].dir = aim([head[0], head[1] - 1.1, head[2]]);
    let low = game::guards::light_on(&mut p.g, c)[0] - dark;
    assert!(low < 0.02 * lit, "the control: the beam at his knees: {low:.3}");
}

/// THE RULE: the sky lights what can see it: under a shut room's ceiling a question that asks the sky's own lines gets none of it, at the height of a day where the open lot gets a lot.
/// A FAILURE MEANS: a sleeper in a shut room at noon has the whole sky on his face, and a man at the foot of a wall counts all of the sky that wall hides.
#[test]
fn the_sky_lights_only_what_can_see_it() {
    let mut s = Scene::new("hut-by-day");
    s.room("hut", 10.0, 10.0, 18.0, 16.0).door("hut-door", Axis::X, 16.0, 13.0).sam(4.0, 24.0);
    s.wall_x(21.0, 10.0, 30.0).wall_x(21.8, 10.0, 30.0);
    let mut p = stand(&s);
    assert!(game::hours::apply_hour(&mut p.g, game::hours::HourId::Overcast));
    p.run(0.2);
    let (yard, room, alley) = (direct(&mut p, [25.0, 1.2, 10.0]), direct(&mut p, [13.0, 1.2, 12.0]), direct(&mut p, [20.0, 1.2, 21.4]));
    assert!(yard > 0.3, "the open lot by day {yard:.3}");
    assert!(room < 0.02 * yard, "in the shut room {room:.3}, in the open {yard:.3}");
    assert!(alley < 0.7 * yard, "between two walls {alley:.3}, in the open {yard:.3}");
}

/// THE RULE: a stroke of lightning lights the open for as long as it lasts. CONTROL: under a roof, behind a shut door, it adds nothing.
/// A FAILURE MEANS: a storm's stroke shows nobody in the yard, and the flash the player sees lights no man to any eye.
#[test]
fn a_stroke_of_lightning_lights_the_open_and_not_a_shut_room() {
    let mut s = Scene::new("storm");
    s.room("hut", 10.0, 10.0, 18.0, 16.0).door("hut-door", Axis::X, 16.0, 13.0).sam(4.0, 24.0);
    let mut p = stand(&s);
    game::weather::weather_set_kind_2(&mut p.g, Some(game::weather_kinds::WeatherKind::Rain));
    p.g.weather.next_flash_at = 1e9;
    p.run(0.1);
    let (yard, room) = ([25.0, 1.2, 8.0], [13.0, 1.2, 12.0]);
    let (yard0, room0) = (direct(&mut p, yard), direct(&mut p, room));
    game::weather::weather_strike(&mut p.g, None).expect("a stroke");
    let (yard1, room1) = (direct(&mut p, yard), direct(&mut p, room));
    assert!(yard1 > yard0 + 0.5, "in the open under the stroke {yard1:.3}, before it {yard0:.3}");
    assert!((room1 - room0).abs() < 0.01 * yard1, "in the shut room under the stroke {room1:.3}, before it {room0:.3}");
}

/// THE RULE: a question that leaves out a kind of the level's lamps gets none of them: in front of a screen, one that leaves out the screens reads less than one that leaves out what no lamp there is.
/// A FAILURE MEANS: a man asleep at his desk is woken by his own screen, as if it were the room's light going on.
#[test]
fn a_question_that_leaves_out_the_screens_gets_none_of_them() {
    let mut p = Play::level("office.json", false, &Opts::default());
    p.sam_unhurt();
    p.g.ai_enabled = false;
    p.run(0.1);
    let at = {
        let l = &sim::state::state_read(p.g.sim).lights;
        let r = (0..l.rows as usize).find(|&r| l.row_bits[r] & sim::lights::R_SCREEN != 0 && l.row_bits[r] & sim::lights::R_ON != 0).expect("the office has a lit screen");
        [l.x[r] + 0.4 * l.dx[r], l.y[r] + 0.4 * l.dy[r], l.z[r] + 0.4 * l.dz[r]]
    };
    let with = light_asked(&mut p.g, at, &[], false, sim::lights::R_PASSING as f64);
    let without = light_asked(&mut p.g, at, &[], false, sim::lights::R_SCREEN as f64);
    assert!(with > 0.2, "the staging: in front of a screen {with:.3}");
    assert!(without < 0.7 * with, "leaving out the screens {without:.3}, not leaving them out {with:.3}");
}

/// THE RULE: every lamp counts once: a dim lamp of a man's added to a lit room moves the light by what it gives, and the room's own bulb is not counted again. CONTROL: the bulb alone.
/// A FAILURE MEANS: a torch switched on anywhere near doubles the room's light on everybody in it.
#[test]
fn every_lamp_counts_once() {
    let mut s = Scene::new("one-bulb");
    s.bulb(15.0, 10.0);
    s.room("hut", 30.0, 20.0, 36.0, 26.0).door("hut-door", Axis::X, 20.0, 32.0).sam(4.0, 24.0);
    let mut p = stand(&s);
    let at = [15.0, 1.0, 11.5];
    let bulb = direct(&mut p, at);
    assert!(bulb > 0.3, "the staging: under the bulb {bulb:.3}");
    lamp(&mut p, 0.0, [15.0, 1.0, 14.5], [0.0, -1.0, 0.0], 9.0, 0.5, 180.0);
    let both = direct(&mut p, at);
    assert!(both >= bulb && both < 1.1 * bulb, "the bulb alone {bulb:.3}, with a faint lamp three metres off {both:.3}");
}
