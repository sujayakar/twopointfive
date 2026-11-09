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
/// bulb goes out that light goes within five seconds, a slice of the level a step and never all of it in one.
/// A FAILURE MEANS: a room lit round a corner is dark to every eye, or a lamp shot out costs a tick the whole level's sweep.
#[test]
fn a_lamp_put_out_takes_its_light_off_the_walls_a_slice_at_a_time() {
    let mut p = stand(&two_rooms());
    p.run(1.0);
    let at = [11.2, 1.2, 15.2];
    let lit = round(&mut p, at);
    assert!(lit > 0.01, "a lit room's corner has light off its walls ({lit})");
    let bulb = p.g.targets.clone().into_iter().find(|&t| game::sim_lightshome::fixture_on(&mut p.g, t)).expect("the bulb");
    let on = game::sim_lightshome::light_in().on;
    game::sim_lightshome::light_home_in(&mut p.g, on, Some(bulb), Some(0.0), None, None, None);
    let mut most = 0;
    for _ in 0..300 { p.frame(); most = most.max(sim::glow::swept()[0]); }
    let left = round(&mut p, at);
    assert!(left < 0.02 * lit, "five seconds after the bulb went out {:.0} % of its light is still on the walls", 100.0 * left / lit);
    let (nx, nz) = sim::bricks::plan(p.g.sim);
    assert!(most as f64 <= (nx * nz / 8.0).ceil(), "a step swept {most} of the level's {} metres", nx * nz);
}
/// The light at a point under a roof that comes round: the mean of the six faces.
fn round(p: &mut Play, at: [f64; 3]) -> f64 { game::scene_collision::window(&mut p.g, &mut |g| game::sense::under_roof(g, at)).expect("a point under a roof").mean }
fn light(p: &mut Play, at: [f64; 3]) -> f64 { game::guards::light_at(&mut p.g, at, None, None, None).0 }
fn seen(p: &mut Play, at: [f64; 3]) -> f64 { let e = game::guards::light_estimate_at(&mut p.g, at, None, None, None); game::game::game_visibility_of(&mut p.g, e) }
fn door(p: &Play, name: &str) -> game::ids::DoorId { game::doors::doors_by_name(&p.g, name).expect("the door") }

/// THE RULE: by day a room is as light as what it is open to: shut up it is dark, with its door open it is lit by the way in and darker the deeper it goes, and the
/// room behind it, past a shut door, stays dark.
/// A FAILURE MEANS: a man in a room that looks lit goes unseen, or one in a dark room is seen.
#[test]
fn by_day_a_room_is_lit_by_what_it_is_open_to() {
    let mut s = Scene::new("two-rooms");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("between", Axis::Z, 18.0, 13.0).door("yard", Axis::X, 16.0, 14.0).sam(30.0, 22.0);
    let mut p = stand(&s);
    assert!(game::hours::apply_hour(&mut p.g, game::hours::HourId::Midday));
    p.run(8.0);
    let (near, deep, behind) = ([14.6, 1.2, 14.6], [11.2, 1.2, 10.8], [24.5, 1.2, 11.0]);
    let shut = [light(&mut p, near), light(&mut p, deep), light(&mut p, behind)];
    assert!(shut.iter().all(|e| *e < 0.03), "shut up at noon the rooms read {shut:?}");
    let yard = door(&p, "yard");
    let c = p.g.dr[yard].frame_centre;
    game::doors::doors_kick_in(&mut p.g, yard, [c[0], 0.0, c[2] + 1.0], 0.0, None);
    p.run(20.0);
    assert!(game::doors::door_angle(&p.g, yard).abs() > 0.8, "the scene: the door to the yard was to stand open");
    let open = [light(&mut p, near), light(&mut p, deep), light(&mut p, behind)];
    assert!(open[0] > 0.3, "by the open door at noon {open:?}");
    assert!(open[1] > 0.01 && open[1] < 0.4 * open[0], "the far corner of the room with the open door {open:?}");
    assert!(open[2] < 0.2 * open[1] + 0.003, "the room behind the shut door {open:?}");
}

/// THE RULE: a lamp is as strong as it is drawn: beside a noon's light next to nothing. And eyes get used to a place: in a shut room lit by that lamp a man is
/// seen at noon as well as at night, and in the dark end of a shut room as badly.
/// A FAILURE MEANS: by day a lamp nobody can see burning lights a man in the yard, or the men in a lit room are blind at three paces.
#[test]
fn a_lamp_is_a_nights_lamp_by_night_and_next_to_nothing_at_noon() {
    let mut p = stand(&two_rooms());
    p.run(6.0);
    let at = [12.6, 1.2, 12.6];
    let night = light(&mut p, at);
    game::hours::apply_hour(&mut p.g, game::hours::HourId::Midday);
    p.run(8.0);
    let noon = light(&mut p, at);
    assert!(night > 0.5, "the scene: under the bulb at night ({night})");
    assert!(noon < 0.02 * night, "under the same bulb in the shut room at noon: {noon} of the night's {night}");
    let (lit, dark) = (seen(&mut p, at), seen(&mut p, [24.5, 1.2, 11.0]));
    assert!(lit > 0.5, "seen under the bulb in the shut room at noon: {lit}");
    let aside = seen(&mut p, [at[0] + 3.0, 1.2, at[2]]);
    assert!(aside > 0.3, "three metres from the only bulb of the shut room at noon: {aside}");
    assert!(dark < 0.2, "seen in the shut room with no lamp at noon: {dark}");
}

/// THE RULE: what comes round is of the level as it stands: a game put back to a moment, its tables made again or not, goes on light for light as the one that
/// never was, and a wall that changes is in it.
/// A FAILURE MEANS: two machines of one game part after a rollback.
#[test]
fn a_game_put_back_has_the_light_of_one_that_never_was() {
    let mut s = Scene::new("two-rooms");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("between", Axis::Z, 18.0, 13.0).door("yard", Axis::X, 16.0, 14.0).bulb(22.0, 12.0).sam(30.0, 22.0);
    let mut p = stand(&s);
    game::hours::apply_hour(&mut p.g, game::hours::HourId::Dusk);
    p.run(3.0);
    let yard = door(&p, "yard");
    let c = p.g.dr[yard].frame_centre;
    game::doors::doors_kick_in(&mut p.g, yard, [c[0], 0.0, c[2] + 1.0], 0.0, None);
    p.run(0.35);   // (the leaf is swinging, and the sweep is half way round)
    for b in p.g.engine.world.statics.clone() { p.g.tab.boxes[b].albedo = [0.05, 0.05, 0.05]; }   // (and all of it is painted: a lamp breaks, a lot is soaked)
    game::scene_collision::wrote(&mut p.g);
    p.run(0.2);
    let (kept, block) = (p.g.clone(), game_start::block::bytes(p.g.sim).to_vec());
    let field = |p: &Play| -> Vec<u64> { sim::state::state_read(p.g.sim).smoke.st[sim::bricks::O_GLOW_MAX..sim::bricks::O_RHO].iter().map(|v| v.to_bits()).collect() };
    p.run(4.0);
    let want = field(&p);
    let slot = p.g.sim;
    sim::state::put(slot, &block);
    p.g = kept.clone();
    game::game::stand_in(&mut p.g, slot);
    p.g.col.mirror.put_backs = -1.0;
    p.run(4.0);
    let got = field(&p);
    let parts = want.iter().zip(&got).filter(|(a, b)| a != b).count();
    assert_eq!(parts, 0, "{parts} of the field's {} numbers differ after a put-back", want.len());
    assert!(want.iter().filter(|v| **v != 0).count() > 100, "the scene: the field was to hold something");
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
    for _ in 0..360 { p.g.ch[p.g.gd[post].ch].cond.dazzled_until = p.g.time + 0.1; p.frame(); }
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
            game::game::stand_in(&mut p.g, slot);
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


/// A FAILURE MEANS: asking what light stands on a point changes the game. A page's throw preview asks it for a block of floor at a time, whenever a menu is up, and the host
/// takes no state back after it (src/view/worldask.ts `light`): a question that writes would part two machines of one game, and a replay from its run.
#[test]
fn asking_the_light_writes_nothing() {
    for seed in [1u32, 2, 3] {
        let run = |ask: bool| -> Vec<Vec<u32>> {
            let mut p = Play::level("killhouse.json", true, &Opts { seed, ..Default::default() });
            let mut out = Vec::new();
            for s in 0..8 {
                if s == 3 { let at = p.sam_at(); can(&mut p, [at[0], 0.2, at[1]], 2.0); }
                for _ in 0..60 {
                    p.frame();
                    if !ask { continue; }
                    let at = p.sam_at();
                    for (i, d) in [[0.0, 0.0], [2.0, 1.0], [-3.0, 2.0], [40.0, 40.0]].into_iter().enumerate() {
                        let q = [at[0] + d[0], 1.0, at[1] + d[1]];
                        let owner = [p.g.ch[p.g.pl[p.g.player].ch].id as f64];
                        assert!(game::sense::light_asked(&mut p.g, q, if i % 3 == 0 { &owner } else { &[] }, i % 2 == 0, if i == 3 { 1.0 } else { 0.0 }).is_finite());
                    }
                }
                out.push(p.words());
            }
            out
        };
        let (never, always) = (run(false), run(true));
        for (s, (a, b)) in never.iter().zip(always.iter()).enumerate() {
            let parts: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).collect();
            assert!(parts.is_empty(), "seed {}: after {} s the asked game differs in parts {:?}", seed, s + 1, parts);
        }
    }
}

/// A FAILURE MEANS: the light asked when the box world's copy does not stand answers something else, or moves the game, or dies. A host reaches this whenever no tick has
/// run since its copy was put down: a state read in, a page's write by path, a rule called from outside a tick. The copy that question stands is not the game's state, so
/// the asked game runs on word for word with the one whose copy was stood first. Controls: each way of losing the copy is first seen refused, and a lit and a dark place answer apart.
#[test]
fn the_light_is_the_same_when_the_copy_had_to_be_stood() {
    use game::ask::WorldAsk;
    let losses: [(&str, fn(&mut game::game::Game)); 2] = [
        ("put down", |g| game::scene_collision::unstand(g)),
        ("owing a write", |g| game::scene_collision::wrote(g)),
    ];
    for seed in [1u32, 2, 3] {
        for (how, lose) in losses {
            // `lost` false: the copy is stood again before the questions, so every one of them finds it standing
            let run = |lost: bool| -> (Vec<u64>, Vec<Vec<u32>>) {
                let mut p = Play::level("killhouse.json", true, &Opts { seed, ..Default::default() });
                p.run(1.0);
                let (at, owner) = (p.sam_at(), p.g.ch[p.g.pl[p.g.player].ch].id as f64);
                let mut answers = Vec::new();
                for (i, d) in [[0.0, 0.0], [2.0, 1.0], [-3.0, 2.0], [40.0, 40.0]].into_iter().enumerate() {
                    lose(&mut p.g);
                    if lost {
                        assert!(WorldAsk::try_of(&p.g).is_err(), "seed {seed}, {how}: the staging is wrong, the copy still answers");
                    } else {
                        game::scene_collision::stands(&mut p.g);
                        assert!(WorldAsk::try_of(&p.g).is_ok(), "seed {seed}, {how}: the control's copy stands");
                    }
                    let q = [at[0] + d[0], 1.0, at[1] + d[1]];
                    let owners: &[f64] = if i % 3 == 0 { &[owner] } else { &[] };
                    answers.push(game::sense::light_asked(&mut p.g, q, owners, i % 2 == 0, if i == 3 { 1.0 } else { 0.0 }).to_bits());
                    assert!(WorldAsk::try_of(&p.g).is_ok(), "seed {seed}, {how}: the question left no copy for the next");
                }
                let mut after = Vec::new();
                for _ in 0..2 { p.run(1.0); after.push(p.words()); }
                (answers, after)
            };
            let (control, lost) = (run(false), run(true));
            assert!(control.0.iter().any(|v| *v != control.0[0]), "seed {seed}, {how}: every place answers alike: the question holds nothing");
            assert_eq!(control.0, lost.0, "seed {seed}, {how}: the answers (their bits) differ");
            for (s, (a, b)) in control.1.iter().zip(lost.1.iter()).enumerate() {
                let parts: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).collect();
                assert!(parts.is_empty(), "seed {seed}, {how}: {} s on, the asked game differs in parts {:?}", s + 1, parts);
            }
        }
    }
}
