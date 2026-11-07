//! THE GAME'S OWN SENSES, held in scenes of a few lines: what smoke hides and what light a man stands in, worked out by the game and by nothing else.
use game::game::GuardState;
use game_start::play::{dist, Opts, Play};
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
    p
}
/// A can venting at `at` for `secs`, by the call every source of smoke in the rules goes by.
fn can(p: &mut Play, at: [f64; 3], secs: f64) {
    let e = game::smoke_types::SmokeEmitter { pos: at, dir: [0.0, 1.0, 0.0], speed: 1.9, radius: 0.2, density: 95.0, temperature: 1.6, ttl: secs, ..Default::default() };
    game::smoke_types::smoke_system_like_emit(&mut p.g, &e);
}
/// What of the light along a line the smoke lets through.
fn through(p: &mut Play, a: [f64; 3], b: [f64; 3]) -> f64 { game::sense::smoke_between(&p.g, a, b) }
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
    let lit = game::sense::glow_at(&mut p.g, at);
    assert!(lit > 0.01, "a lit room's corner has light off its walls ({lit})");
    let bulb = p.g.targets.clone().into_iter().find(|&t| game::sim_lightshome::fixture_on(&mut p.g, t)).expect("the bulb");
    let on = game::sim_lightshome::light_in().on;
    game::sim_lightshome::light_home_in(&mut p.g, on, Some(bulb), Some(0.0), None, None, None);
    let mut most = 0;
    for _ in 0..90 { p.frame(); most = most.max(sim::glow::swept()); }
    let left = game::sense::glow_at(&mut p.g, at);
    assert!(left < 0.02 * lit, "a second and a half after the bulb went out {:.0} % of its light is still on the walls", 100.0 * left / lit);
    let (nx, nz) = sim::bricks::plan(p.g.sim);
    assert!(most as f64 <= (nx * nz / 8.0).ceil(), "a step swept {most} of the level's {} metres", nx * nz);
}

/// A room 10 m by 8 m with a bulb over its east end, a sentry at its west end looking east, and Sam under the bulb 7 m in front of him.
fn watched_room() -> Scene {
    let mut s = Scene::new("watched-room");
    s.room("room", 10.0, 10.0, 20.0, 18.0).door("south", Axis::X, 18.0, 12.0);
    s.bulb(18.0, 13.0).sentry("post", [11.0, 13.0], [19.0, 13.0]).sam(18.0, 13.0);
    s
}

/// THE RULE: a man under a lamp in front of a guard is not seen through a canister's smoke, and is once it has gone.
/// A FAILURE MEANS: smoke hides nobody from the men it is thrown to blind.
#[test]
fn smoke_hides_a_lit_man_from_a_guard() {
    let mut p = stand(&watched_room());
    p.run(0.2);
    can(&mut p, [14.0, 0.2, 13.0], 30.0);
    can(&mut p, [15.5, 0.2, 13.0], 30.0);
    // (the sentry is held blind while the smoke fills the room between them)
    let post = p.guard("post");
    for _ in 0..360 { p.g.gd[post].dazzled_until = p.g.time + 0.1; p.frame(); }
    let eye = { let a = p.at(post); [a[0], 1.65, a[1]] };
    let sam = { let a = p.sam_at(); [a[0], 1.2, a[1]] };
    assert!(through(&mut p, eye, sam) < 0.05, "the scene: six seconds on, {:.3} of the light comes through the smoke", through(&mut p, eye, sam));
    if let Some(t) = p.always(12.0, |p| p.state(post) == GuardState::Patrol) { panic!("{t:.2} s in, the guard is {:?} of a man behind a canister's smoke (awareness {:.2})", p.state(post), p.awareness(post)); }
    let seen = p.until(90.0, |p| p.state(post) != GuardState::Patrol);
    assert!(seen.is_some(), "the smoke long gone, the guard never notices the lit man in front of him");
}

/// THE RULE: a searching man knows the dark places from the lit ones: each place he might look in is read by the light that stands on it.
/// A FAILURE MEANS: a search goes by walls alone, and hiding in the dark buys nothing.
#[test]
fn a_searcher_reads_the_light_on_every_place_he_might_look_in() {
    let mut p = stand(&watched_room());
    let post = p.guard("post");
    p.until(8.0, |p| p.state(post) == GuardState::Alert).expect("the sentry sees the lit man");
    p.sam_put([35.0, 25.0]);
    p.until(60.0, |p| p.state(post) == GuardState::Search && p.g.gd[post].search_plan.as_ref().map_or(false, |s| s.spots.len() > 3)).expect("he loses him and searches");
    p.run(3.0);
    let plan = p.g.gd[post].search_plan.clone().expect("his plan");
    let read: Vec<f64> = plan.spots.iter().filter_map(|s| s.e.map(|_| s.dark)).collect();
    assert!(read.len() * 2 > plan.spots.len(), "three seconds into his search {} of {} places have been read", read.len(), plan.spots.len());
    let (lo, hi) = read.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), &x| (a.min(x), b.max(x)));
    assert!(hi - lo > 0.3, "every place reads as dark as every other ({lo:.2} to {hi:.2}): a room with a lamp at one end has both");
    let _ = dist;
}

/// THE RULE: the fog of war shows the team no man behind smoke, and shows him when any one of them has a line to him that the smoke lets through.
/// A FAILURE MEANS: the team is shown men it cannot see, or loses a man one of them has in plain view.
#[test]
fn the_fog_hides_a_man_behind_smoke_and_not_from_the_one_who_sees_past_it() {
    let mut s = Scene::new("hall");
    s.room("hall", 8.0, 10.0, 24.0, 18.0).door("yard", Axis::X, 18.0, 22.0).door("north", Axis::X, 10.0, 22.0).bulb(16.0, 14.0);
    s.team([10.0, 14.0], std::f64::consts::FRAC_PI_2, &[[10.0, 12.0], [10.0, 16.0], [9.0, 13.0], [9.0, 15.0]]).defender("hall", [22.5, 14.0], "yard");
    let mut p = stand(&s);
    let man = p.defender_at([22.5, 14.0]);
    p.g.ai_enabled = false;   // (nobody moves and nobody fires: what is asked is what is shown)
    p.g.fog_of_war.on = true;
    p.run(0.5);
    assert!(game::fogofwar::fog_of_war_active(&p.g) && !game::fogofwar::fog_of_war_hidden(&mut p.g, man), "the scene: with clear air between them the team sees him");
    for z in [11.5, 14.0, 16.5] { can(&mut p, [17.0, 0.2, z], 30.0); }
    p.run(8.0);
    let chest = { let a = p.at(man); [a[0], 1.2, a[1]] };
    let lines: Vec<f64> = p.g.fog_of_war.eyes.clone().iter().map(|e| through(&mut p, e.pos, chest)).collect();
    assert!(lines.len() >= 5 && lines.iter().all(|t| *t < p.g.k.fog.smoke), "the scene: every eye of the team has smoke before it ({lines:?})");
    assert!(game::fogofwar::fog_of_war_hidden(&mut p.g, man), "he is shown to a team that has only smoke to look at");
    p.sam_put([20.5, 12.0]);
    p.point_at([22.5, 14.0]);
    p.run(0.5);
    let sam = { let a = p.sam_at(); [a[0], 1.6, a[1]] };
    let past = through(&mut p, sam, chest);
    if past >= p.g.k.fog.smoke { assert!(!game::fogofwar::fog_of_war_hidden(&mut p.g, man), "Sam has him in view past the smoke ({past:.2} of the light) and the team is not shown him"); }
    else { panic!("the scene: Sam was to stand past the smoke ({past:.2})"); }
}

/// THE RULE: a game put back (a restart, a checkpoint) has the smoke it had then and no other, and a canister let out after it is smoke like any.
/// A FAILURE MEANS: after a restart smoke hides nobody where it stands, or hides men where there is none.
#[test]
fn a_game_put_back_has_no_smoke_of_the_try_before() {
    let mut p = stand(&two_rooms());
    p.run(0.3);
    let (kept, block) = (p.g.clone(), game_start::block::bytes(p.g.sim).to_vec());
    let (a, b) = ([12.5, 1.2, 13.0], [17.5, 1.2, 13.0]);
    let mut last = 0.0;
    for (n, at) in [[15.0, 0.1, 13.0], [15.0, 0.1, 13.0], [25.0, 0.1, 13.0], [15.0, 0.1, 13.0]].into_iter().enumerate() {
        if n > 0 {
            let slot = p.g.sim;
            sim::state::put(slot, &block);
            p.g = kept.clone();
            game::io::set_slot(&mut p.g, slot);
            p.g.col.mirror.put_backs = -1.0;
            p.run(0.3);
            assert!(through(&mut p, a, b) == 1.0, "try {n}: {:.3} of the light comes through a room with nothing let out in it", through(&mut p, a, b));
        }
        can(&mut p, at, 20.0);
        p.run(5.0);
        let t = through(&mut p, a, b);
        if at[0] < 20.0 {
            assert!(t < 0.05, "try {n}: {t:.3} of the light comes through a canister's smoke, five seconds on");
            assert!(n == 0 || (t - last).abs() < 1e-9, "try {n}: {t} of the light, where the try before it had {last}: the same canister in the same room");
            last = t;
        } else {
            assert!(t > 0.95, "try {n}: {t:.3} of the light through a room whose smoke is in the next one, behind a shut door");
        }
    }
}

/// THE RULE: what hides nobody takes no room: a source of the picture's alone, or one that lets out next to nothing in its whole life, is no smoke of the
/// game's, and the room left for dust is counted without it.
/// A FAILURE MEANS: a fire fight's muzzle smoke uses up the room, and no wall gives off dust.
#[test]
fn a_fire_fights_wisps_leave_room_for_dust() {
    let mut p = stand(&two_rooms());
    p.run(0.2);
    let room = game::smoke_types::smoke_system_like_budget(&p.g);
    let e = game::smoke_types::SmokeEmitter { pos: [14.0, 1.4, 13.0], dir: [0.0, 1.0, 0.0], speed: 1.0, radius: 0.05, density: 6.0, ttl: 2.5, ..Default::default() };
    for _ in 0..90 { game::smoke_types::smoke_system_like_emit(&mut p.g, &e); }
    for _ in 0..90 { game::smoke_types::smoke_system_like_emit(&mut p.g, &game::smoke_types::SmokeEmitter { look: Some(true), radius: 0.5, density: 95.0, ..e.clone() }); }
    assert_eq!(game::smoke_types::smoke_system_like_budget(&p.g), room, "wisps and the picture's own sources took room");
    can(&mut p, [14.0, 0.1, 13.0], 1.0);
    assert_eq!(game::smoke_types::smoke_system_like_budget(&p.g), room - 1.0);
    p.run(1.5);
    assert_eq!(game::smoke_types::smoke_system_like_budget(&p.g), room, "a source that is spent gives its room back");
}
