//! THE GAME'S OWN SENSES, held in scenes of a few lines: what smoke hides and what light a man stands in, worked out by the game and by nothing else.
use game_start::play::{Opts, Play};
use game_start::scene::{Axis, Scene};

/// Two rooms either side of a wall with a door in it, a bulb in the west one, Sam in the west one's middle.
fn two_rooms() -> Scene {
    let mut s = Scene::new("two-rooms");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("between", Axis::Z, 18.0, 13.0).door("yard", Axis::X, 16.0, 14.0);
    s.bulb(12.0, 12.0).sam(14.0, 13.0);
    s
}
fn stand(s: &Scene) -> Play {
    let mut p = Play::scene(s, &Opts::default());
    p.sam_unhurt();
    p.g.k.sense.grid = true;
    p.g.k.sense.bounce = true;
    p
}
/// A can venting at `at` for `secs`, by the call every source of smoke in the rules goes by.
fn can(p: &mut Play, at: [f64; 3], secs: f64) {
    let e = game::smoke_types::SmokeEmitter { pos: at, dir: [0.0, 1.0, 0.0], speed: 1.9, radius: 0.2, density: 95.0, temperature: 1.6, ttl: secs, ..Default::default() };
    game::smoke_types::smoke_system_like_emit(&mut p.g, &e);
}
/// What of the light along a line the smoke lets through.
fn through(p: &mut Play, a: [f64; 3], b: [f64; 3]) -> f64 { game::sense::smoke_grid_along(&mut p.g, a, b) }
/// What the smoke's cells hold, with what was given back, against what was let out: the share that is not accounted for.
fn unaccounted(p: &Play) -> f64 {
    use sim::bricks as b;
    let st = &sim::state::state_read(p.g.sim).smoke.st;
    let held: f64 = st[b::O_RHO..].iter().sum::<f64>() * b::CELL * b::CELL * b::CELL;
    (held + st[b::H_GIVEN] - st[b::H_OUT]).abs() / st[b::H_OUT].max(1e-12)
}
fn bricks(p: &Play) -> usize {
    use sim::bricks as b;
    let st = &sim::state::state_read(p.g.sim).smoke.st;
    (0..b::MAX_BRICKS).filter(|k| st[b::O_KEYS + k * 2] != 0.0).count()
}

/// THE RULE: however much smoke there is, the grid holds it and answers for it: a canister on every three metres of the level, and every line across it
/// is answered, every unit let out is in a cell or in the book.
/// A FAILURE MEANS: smoke was let out into no cell, so a line through it reads clearer than it is.
#[test]
fn smoke_is_never_refused_a_cell() {
    let mut p = stand(&two_rooms());
    p.run(0.2);
    let mut n = 0;
    for z in 0..9 { for x in 0..13 { if n < 60 && (x + z) % 2 == 0 { can(&mut p, [1.5 + 3.0 * x as f64, 0.2, 1.5 + 3.0 * z as f64], 60.0); n += 1; } } }
    for s in 0..20 {
        p.run(2.0);
        let t = through(&mut p, [1.0, 1.2, 1.0], [39.0, 1.2, 27.0]);
        assert!((0.0..=1.0).contains(&t), "{} s in, {} canisters and {} bricks: a line across the level is not answered ({t})", 2 * (s + 1), n, bricks(&p));
        assert!(unaccounted(&p) < 1e-9, "{} s in: {:.1e} of what was let out is nowhere", 2 * (s + 1), unaccounted(&p));
    }
    assert!(bricks(&p) > 200, "the scene: the smoke was to be in more than two hundred bricks ({})", bricks(&p));
}

/// THE RULE: smoke fills the room it is in and stops at a shut door.
/// A FAILURE MEANS: the senses see through smoke that stands before them, or are blinded through a wall.
#[test]
fn a_shut_door_holds_the_smoke() {
    let mut p = stand(&two_rooms());
    p.run(0.2);
    can(&mut p, [16.5, 0.2, 13.0], 40.0);
    p.run(8.0);
    let (near, far) = (through(&mut p, [16.2, 1.2, 12.0], [17.4, 1.2, 14.0]), through(&mut p, [18.6, 1.2, 12.0], [19.8, 1.2, 14.0]));
    assert!(near < 0.1, "8 s on, the air before the shut door lets {near:.3} through");
    assert!(far > 0.999, "…and the air behind it {far:.3}: the smoke went through a shut door");
}

/// THE RULE: light comes round a corner: a man the bulb has no line to, in a room it lights, reads brighter than the bulb's own light on him; and when the
/// bulb goes out that light goes within a second and a half, a slice of the level a step and never all of it in one.
/// A FAILURE MEANS: a room lit round a corner is dark to every eye, or a lamp shot out costs a tick the whole level's sweep.
#[test]
fn a_lamp_put_out_takes_its_light_off_the_walls_a_slice_at_a_time() {
    let mut p = stand(&two_rooms());
    p.run(1.0);
    let at = [11.2, 1.2, 15.2];
    let lit = game::sense::smoke_grid_glow_at(&mut p.g, at);
    assert!(lit > 0.01, "a lit room's corner has light off its walls ({lit})");
    let bulb = p.g.targets.clone().into_iter().find(|&t| game::sim_lightshome::fixture_on(&mut p.g, t)).expect("the bulb");
    let on = game::sim_lightshome::light_in().on;
    game::sim_lightshome::light_home_in(&mut p.g, on, Some(bulb), Some(0.0), None, None, None);
    let mut most = 0;
    for _ in 0..90 { p.frame(); most = most.max(sim::glow::swept()); }
    let left = game::sense::smoke_grid_glow_at(&mut p.g, at);
    assert!(left < 0.02 * lit, "a second and a half after the bulb went out {:.0} % of its light is still on the walls", 100.0 * left / lit);
    let (nx, nz) = sim::bricks::plan(p.g.sim);
    assert!(most as f64 <= (nx * nz / 8.0).ceil(), "a step swept {most} of the level's {} metres", nx * nz);
}
