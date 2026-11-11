//! WHERE THE GAME'S OWN SMOKE GOES: up, sideways, with the wind, through what opens and not through what is shut; what the list of sources does when it is full; how it ends.
mod world_ask;
use game::smoke_types::SmokeEmitter;
use game_start::play::{Opts, Play};
use game_start::scene::{Axis, Scene};
use world_ask::FurnAsk;

fn stand(s: &Scene) -> Play {
    let mut p = Play::scene(s, &Opts::default());
    p.sam_unhurt();
    p.sam_unseen();
    p.run(0.1);
    p
}
/// A room under a roof, so no wind reaches its smoke; Sam out of the way.
fn room() -> Scene {
    let mut s = Scene::new("room");
    s.room("room", 10.0, 8.0, 18.0, 16.0).door("room-door", Axis::X, 16.0, 13.0).sam(4.0, 24.0);
    s
}
/// A canister's worth, by the call every source of smoke in the rules goes by.
fn can_like(at: [f64; 3], secs: f64) -> SmokeEmitter { SmokeEmitter { pos: at, dir: [0.0, 1.0, 0.0], speed: 1.9, radius: 0.2, density: 95.0, temperature: 1.6, ttl: secs, ..Default::default() } }
fn can(p: &mut Play, at: [f64; 3], secs: f64) { game::smoke_types::smoke_system_like_emit(&mut p.g, &can_like(at, secs)); }
fn through(p: &Play, a: [f64; 3], b: [f64; 3]) -> f64 { game::sense::smoke_between(&p.g, a, b) }
/// what a line across (x, z) at height y, a metre long along x, lets through
fn across(p: &Play, x: f64, y: f64, z: f64) -> f64 { through(p, [x - 0.5, y, z], [x + 0.5, y, z]) }

/// THE RULE: a shut door holds smoke, and swung open it lets it through. CONTROL: the same air before it opens, with the can as long at work.
/// A FAILURE MEANS: smoke stays behind a door that has been kicked in as if the door were still there, so a room next to a full one stays clear to every eye.
#[test]
fn a_door_opened_lets_the_smoke_through() {
    let mut s = Scene::new("two-rooms");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("between", Axis::Z, 18.0, 12.4).door("yard", Axis::X, 16.0, 14.0).sam(4.0, 24.0);
    let mut p = stand(&s);
    let leaf = game::doors::doors_by_name(&p.g, "between").expect("the door");
    let c = p.g.dr[leaf].frame_centre;
    can(&mut p, [17.0, 0.2, c[2]], 40.0);
    p.run(8.0);
    let far = [19.5, 1.0, c[2] + 0.2];
    assert!(across(&p, 17.0, 1.0, c[2]) < 0.1, "the staging: the room with the can is not full of smoke");
    assert!(across(&p, far[0], far[1], far[2]) > 0.999, "the control: behind the shut door {:.3} of the light comes through", across(&p, far[0], far[1], far[2]));
    let from = if game::doors::doors_boot_holds(&p.g, leaf, [c[0] - 1.0, 0.0, c[2]]) { [c[0] + 1.0, 0.0, c[2]] } else { [c[0] - 1.0, 0.0, c[2]] };
    game::doors::doors_kick_in(&mut p.g, leaf, from, 0.0, None);
    p.run(8.0);
    let t = across(&p, far[0], far[1], far[2]);
    assert!(t < 0.8, "eight seconds after the door was kicked open {t:.3} of the light comes through the next room");
}

/// THE RULE: a wall holds smoke and a hole blown in it lets it through for as long as there is smoke to come. CONTROL: the same air before the blast.
/// A FAILURE MEANS: smoke stays behind a wall that has a hole in it, and the room next door clears of the blast's own dust while the first is still full.
#[test]
fn a_hole_in_a_wall_lets_the_smoke_through() {
    let mut s = Scene::new("partition");
    s.room("west", 10.0, 8.0, 18.0, 16.0).room("east", 18.0, 8.0, 26.0, 16.0).door("west-door", Axis::X, 16.0, 13.0).sam(4.0, 24.0);
    let mut p = stand(&s);
    can(&mut p, [17.0, 0.2, 12.0], 90.0);
    p.run(8.0);
    assert!(across(&p, 17.0, 0.6, 12.0) < 0.1, "the staging: the side with the can is not full of smoke");
    assert!(across(&p, 19.5, 0.6, 12.0) > 0.999, "the control: behind the wall {:.3} of the light comes through", across(&p, 19.5, 0.6, 12.0));
    assert!(p.wall_blast([17.7, 0.3, 12.0], None) > 0.0, "the staging: the blast opened nothing");
    p.run(40.0);
    let t = across(&p, 19.5, 0.6, 12.0);
    assert!(t < 0.8, "forty seconds after the hole was blown {t:.3} of the light comes through past it");
}

/// THE RULE: a can on the floor stands its smoke up to a man's chest within seconds, by its plume, not by what creeps up from the floor. CONTROL: at the same moment, the air at that height two metres off is still clear.
/// A FAILURE MEANS: a can thrown at a man's feet hides nobody until long after it has landed.
#[test]
fn a_cans_smoke_stands_up_to_a_mans_chest_at_once() {
    let mut p = stand(&room());
    can(&mut p, [14.0, 0.1, 12.0], 30.0);
    p.run(3.5);
    let (chest, ankle, off) = (across(&p, 14.0, 1.2, 12.0), across(&p, 14.0, 0.2, 12.0), across(&p, 14.0, 1.2, 15.0));
    assert!(chest < 0.2, "three and a half seconds on, {chest:.3} of the light comes through at chest height");
    assert!(-chest.ln() > 0.25 * -ankle.ln(), "the smoke at his chest is thin against what is at his ankles: {chest:.3} to {ankle:.3}");
    assert!(off > 0.5, "the control: three metres off at the same height {off:.3}");
}

/// THE RULE: in a room smoke goes on up past its plume, to the ceiling. CONTROL: over its plume, the moment it is let out, the air is clear.
/// A FAILURE MEANS: a can fills a room to the height of a man's chest and a man's eyes stay above it.
#[test]
fn in_a_room_smoke_goes_up_to_the_ceiling() {
    let mut p = stand(&room());
    can(&mut p, [14.0, 0.1, 12.0], 30.0);
    p.run(1.0);
    let early = across(&p, 14.0, 2.7, 12.0);
    assert!(early > 0.9, "the control: a second on, under the ceiling {early:.3}");
    p.run(14.0);
    let late = across(&p, 14.0, 2.7, 12.0);
    assert!(late < 0.3, "fifteen seconds on, under the ceiling {late:.3} of the light comes through");
}

/// Rain over the lot, its wind straight along x so that the grid carries it with no slant, and a unit along it.
fn wind_over(p: &mut Play) -> ([f64; 2], f64) {
    game::weather::weather_set_kind_2(&mut p.g, Some(game::weather_kinds::WeatherKind::Rain));
    p.g.weather.next_flash_at = 1e9;
    let v = 2.5;
    p.g.weather.wind = [v, 0.0];
    ([1.0, 0.0], v)
}
/// A lot under open sky, and a hut out of the way in its corner.
fn lot() -> Scene {
    let mut s = Scene::new("lot");
    s.room("hut", 30.0, 20.0, 36.0, 26.0).door("hut-door", Axis::X, 20.0, 32.0).sam(4.0, 24.0);
    s
}
/// a line of two metres across the wind, `down` metres downwind of `at` and `side` metres to one side, at height y
fn across_wind(p: &Play, at: [f64; 2], u: [f64; 2], down: f64, side: f64, y: f64, half: f64) -> f64 {
    let c = [at[0] + u[0] * down - u[1] * side, at[1] + u[1] * down + u[0] * side];
    through(p, [c[0] + u[1] * half, y, c[1] - u[0] * half], [c[0] - u[1] * half, y, c[1] + u[0] * half])
}
/// a line along the wind, from `from` to `to` metres downwind of `at`, `side` metres to one side, at height y
fn along_wind(p: &Play, at: [f64; 2], u: [f64; 2], from: f64, to: f64, side: f64, y: f64) -> f64 {
    let o = [at[0] - u[1] * side, at[1] + u[0] * side];
    through(p, [o[0] + u[0] * from, y, o[1] + u[1] * from], [o[0] + u[0] * to, y, o[1] + u[1] * to])
}

/// THE RULE: under the open sky the wind carries smoke off downwind, and not upwind. CONTROL: the same two lines as far off on the side it comes from.
/// A FAILURE MEANS: a smoke screen thrown across a storm's yard stands where it landed, and the picture's smoke, which goes with the wind, is not what any eye is asked about.
#[test]
fn the_wind_carries_smoke_downwind() {
    let mut p = stand(&lot());
    let (u, _) = wind_over(&mut p);
    let at = [18.0, 12.0];
    can(&mut p, [at[0], 0.2, at[1]], 30.0);
    p.run(8.0);
    let (down, up) = (across_wind(&p, at, u, 6.0, 0.0, 0.6, 2.0), across_wind(&p, at, u, -6.0, 0.0, 0.6, 2.0));
    assert!(down < 0.5, "six metres downwind {down:.3} of the light comes through");
    assert!(up > 0.99, "the control: six metres upwind {up:.3}");
}

/// THE RULE: in the wind a plume still widens: some of its own spread sideways stays with it. CONTROL: its middle is thicker than its edge.
/// A FAILURE MEANS: smoke blown along stays a line one cell wide, and cannot get round what it is blown against.
#[test]
fn in_the_wind_a_plume_still_widens() {
    let mut p = stand(&lot());
    let (u, _) = wind_over(&mut p);
    let at = [10.0, 12.0];
    can(&mut p, [at[0], 0.2, at[1]], 30.0);
    p.run(10.0);
    let (mid, side) = (along_wind(&p, at, u, 2.0, 10.0, 0.0, 0.3), along_wind(&p, at, u, 2.0, 10.0, 1.25, 0.3));
    assert!(mid < 0.05, "the staging: along the plume {mid:.3}");
    assert!(side < 0.9, "a metre and a quarter to one side of the plume {side:.3}: it has not widened");
}

/// THE RULE: the wind is slower at the ground: what is let out over a man's height has gone further than what is at his ankles.
/// A FAILURE MEANS: a smoke screen in a storm stays the same shape from the floor up and a low man is hidden from the same men a tall one is, for as long.
#[test]
fn the_wind_is_slower_at_the_ground() {
    let mut p = stand(&lot());
    let (u, v) = wind_over(&mut p);
    let at = [8.0, 12.0];
    // (a wide source, so that the wind has as much to carry low as high)
    let e = SmokeEmitter { radius: 1.2, ..can_like([at[0], 1.2, at[1]], 30.0) };
    game::smoke_types::smoke_system_like_emit(&mut p.g, &e);
    p.run(4.0);
    let tau = |y: f64| -across_wind(&p, at, u, 0.8 * v * 4.0, 0.0, y, 1.5).ln();
    let (low, high) = (tau(0.2), tau(2.2));
    assert!(high > 2.0 * low, "ahead of the plume, the smoke high up {high:.4}, at the ankles {low:.4}");
}

/// THE RULE: when the list of what lets smoke out is full, a big source takes the place of the least, and one as small as the least takes none.
/// A FAILURE MEANS: on a floor full of dust, a can thrown lets out nothing, and a man trusts a screen no eye is stopped by.
#[test]
fn a_big_source_takes_the_place_of_the_least_when_the_list_is_full() {
    let mut p = stand(&room());
    let small = SmokeEmitter { density: 1.0, ttl: 5.0, ..can_like([11.0, 0.2, 9.0], 5.0) };
    assert!(game::sense::smoke_held(&p.g, &small), "the staging: the small source is one the game holds");
    for _ in 0..sim::bricks::MAX_EMIT { game::sense::smoke_emit(&mut p.g, &small); }
    assert_eq!(sim::bricks::emitting(p.g.sim), sim::bricks::MAX_EMIT, "the staging: the list is full");
    let spot = [16.0, 0.2, 14.0];
    let there = |p: &Play, s: [f64; 3]| sim::bricks::letting(p.g.sim).any(|e| e.at == s);
    let late = [15.0, 0.2, 14.0];
    game::sense::smoke_emit(&mut p.g, &SmokeEmitter { pos: late, ..small.clone() });
    assert!(!there(&p, late), "the control: one no bigger than the rest took a place");
    can(&mut p, spot, 30.0);
    assert!(there(&p, spot), "the can took no place on the full list");
    p.run(2.5);
    assert!(across(&p, spot[0], 0.6, spot[2]) < 0.2, "a can on a full list lets out: {:.3} of the light comes through", across(&p, spot[0], 0.6, spot[2]));
}

/// THE RULE: a can against a wall lets its smoke out into the room, wherever the grid's cell falls. CONTROL: the same can in the middle of a cell of the room fills it alike.
/// A FAILURE MEANS: a can that rolled to a wall vents into the wall and hides nobody.
#[test]
fn a_can_against_a_wall_lets_out_into_the_room() {
    let read = |x: f64| {
        let mut s = Scene::new("room-by-a-cell");
        // (the wall's middle line is the middle of a cell of the grid, half a metre to a cell)
        s.room("room", 10.0, 8.0, 17.75, 16.0).door("room-door", Axis::X, 16.0, 13.0).sam(4.0, 24.0);
        let mut p = stand(&s);
        can(&mut p, [x, 0.2, 12.0], 30.0);
        p.run(5.0);
        across(&p, 16.5, 0.6, 12.0)
    };
    let (against, clear_of_it) = (read(17.6), read(17.25));
    assert!(clear_of_it < 0.1, "the control: a can in the room {clear_of_it:.3}");
    assert!(against < 0.1, "a can against the wall: {against:.3} of the light comes through in front of it");
}

/// THE RULE: when a can is spent its smoke thins to nothing and the game has none left. CONTROL: a few seconds on it is still there.
/// A FAILURE MEANS: a floor that has seen one can is hazy for good, and every eye pays for that smoke for the rest of the mission.
#[test]
fn a_spent_cans_smoke_comes_to_an_end() {
    let mut p = stand(&room());
    can(&mut p, [14.0, 0.2, 12.0], 3.0);
    p.run(5.0);
    assert!(game::sense::smoky(&p.g), "the control: five seconds on there is smoke");
    assert!(p.until(150.0, |p| !game::sense::smoky(&p.g)).is_some(), "two and a half minutes on the game still holds smoke");
    assert_eq!(across(&p, 14.0, 0.6, 12.0), 1.0, "with none held, the line where the can stood");
}

/// THE RULE: told to stop, every source lets out no more, and what is in the air stays. CONTROL: not told, it goes on and the air gets thicker.
/// A FAILURE MEANS: a scene that ends the smoke sources goes on filling with smoke, or empties the air along with them.
#[test]
fn sources_told_to_stop_let_out_no_more_and_what_is_in_the_air_stays() {
    let run = |shut: bool| {
        let mut p = stand(&room());
        can(&mut p, [14.0, 0.2, 12.0], 30.0);
        p.run(2.0);
        let held = -across(&p, 14.0, 0.3, 12.0).ln();
        if shut { game::sense::smoke_shut(&mut p.g); }
        let letting = sim::bricks::letting(p.g.sim).count();
        let at_once = -across(&p, 14.0, 0.3, 12.0).ln();
        p.run(3.0);
        (held, at_once, letting, st_out(&p), -across(&p, 14.0, 0.3, 12.0).ln())
    };
    let (held, at_once, letting, out_shut, _) = run(true);
    assert_eq!(letting, 0, "told to stop, {letting} still let out");
    assert!(at_once > 0.9 * held, "told to stop, the air went too: {held:.2} before, {at_once:.2} after");
    let (_, _, letting, out_on, _) = run(false);
    assert_eq!(letting, 1, "the control: not told, the can goes on");
    assert!(out_shut < 0.6 * out_on, "three seconds on the book says {out_shut:.2} let out when told to stop, and {out_on:.2} when not");
}
/// what has been let out, by the grid's own book
fn st_out(p: &Play) -> f64 { sim::state::state_read(p.g.sim).smoke.st[sim::bricks::H_OUT] }

/// THE RULE: a line above the top of the grid meets no smoke, whatever stands in the top cells under it. CONTROL: the same line down in those cells does.
/// A FAILURE MEANS: a man on a roof or a lamp high up is hidden by smoke that stands well below his line.
#[test]
fn a_line_over_the_top_of_the_grid_meets_no_smoke() {
    let mut p = stand(&room());
    // (a source high in the air of a hall that is not there: the top cells of the grid, over the open lot north of the room)
    can(&mut p, [14.0, 3.75, 4.0], 10.0);
    p.run(1.5);
    let (top, above) = (across(&p, 14.0, 3.8, 4.0), across(&p, 14.0, 5.0, 4.0));
    assert!(top < 0.5, "the control: through the top cells {top:.3}");
    assert_eq!(above, 1.0, "over the top of the grid");
}
