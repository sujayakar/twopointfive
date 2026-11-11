//! WHAT STOPS A LINE: a line and a ray take a thing as the shape it is, wherever along the line it stands and however the line starts and ends.
mod world_ask;
use game::scene_collision::{raycast, segment_blocked, shot_blocked, solid_blocked, things_ray};
use game_start::json::{self, Json};
use game_start::play::{Opts, Play};
use game_start::scene::{Axis, Scene};
use world_ask::a_box;
use std::f64::consts::FRAC_1_SQRT_2;

/// a box of a level's file with one word more
fn with(b: Json, key: &str, value: Json) -> Json {
    let Json::Obj(mut members, _) = b else { panic!("a box is an object") };
    members.push((String::from(key), value));
    json::obj(members.iter().map(|(k, v)| (k.as_str(), v.clone())).collect())
}

/// One wide room with a door, nobody's mind running and Sam out of it, and the boxes and furniture asked about.
fn room_props(boxes: Vec<Json>, props: Vec<Json>) -> Play {
    let mut s = Scene::new("lines");
    s.room("hall", 8.0, 6.0, 30.0, 22.0).door("yard", Axis::X, 22.0, 9.4).sam(9.0, 7.0);
    let mut level = s.level();
    let Json::Obj(members, _) = &mut level else { panic!("a level is an object") };
    for (key, rows) in [("boxes", boxes), ("props", props)] {
        let (_, list) = members.iter_mut().find(|(k, _)| k == key).expect("a level has it");
        let Json::Arr(list) = list else { panic!("{key} is a list") };
        list.extend(rows);
    }
    let path = std::env::temp_dir().join(format!("rules-lines-{}-{:?}.json", std::process::id(), std::thread::current().id()));
    std::fs::write(&path, json::to_text(&level)).unwrap();
    let mut p = Play::level(path.to_str().unwrap(), false, &Opts { seed: 1, ..Default::default() });
    std::fs::remove_file(&path).ok();
    p.g.ai_enabled = false;
    p.run(0.05);
    p
}
fn room(boxes: Vec<Json>) -> Play { room_props(boxes, Vec::new()) }

/// THE RULE: a round pillar is round: a line past where the corner of its square would be goes by, a line at the same height through the pillar stops, to an eye, a round, a foot and a ray.
/// A FAILURE MEANS: men are hidden by, and rounds stop in, the air beside a pillar.
#[test]
fn a_round_pillar_stops_a_line_where_it_is_round() {
    let c = [18.0, 1.5, 14.0];
    let mut p = room(vec![with(a_box("pillar", c, [1.0, 1.5, 1.0], "white", &[]), "round", json::b(true))]);
    // a line slanted across the corner of the square, 1.2 m from the middle (the corner stands 1.41 m out); the same line 0.8 m out goes through the pillar
    let line = |off: f64| {
        let n = [FRAC_1_SQRT_2, FRAC_1_SQRT_2];
        let t = [FRAC_1_SQRT_2, -FRAC_1_SQRT_2];
        ([c[0] + n[0] * off - t[0] * 3.0, 1.2, c[2] + n[1] * off - t[1] * 3.0], [c[0] + n[0] * off + t[0] * 3.0, 1.2, c[2] + n[1] * off + t[1] * 3.0], [t[0], 0.0, t[1]])
    };
    let (a, b, d) = line(1.2);
    let g = &mut p.g;
    assert!(!segment_blocked(g, a, b, None, None, None) && !solid_blocked(g, a, b, None) && !shot_blocked(g, a, b, None, None), "past the round of it the line is stopped");
    assert!(raycast(g, a, d, 6.0).is_none(), "past the round of it a ray meets something");
    let (a, b, d) = line(0.8);
    assert!(segment_blocked(g, a, b, None, None, None) && solid_blocked(g, a, b, None) && shot_blocked(g, a, b, None, None), "through the pillar the line goes on");
    let hit = raycast(g, a, d, 6.0).expect("through the pillar a ray meets nothing");
    assert!((hit.t - (3.0 - (1.0f64 - 0.64).sqrt())).abs() < 0.02, "the ray meets the pillar's round at {:.3}", hit.t);
}

/// THE RULE: a box with a corner cut off stops nothing where the corner was, and stops what crosses the rest of it. A FAILURE MEANS: a broken end, a sloped sill, stops rounds and eyes in the air it no longer fills.
#[test]
fn a_cut_box_stops_a_line_only_where_it_is_left() {
    let c = [18.0, 1.5, 14.0];
    // the plane keeps the part under the slope x + y <= 0.6 in the box's own frame: the top and the +x side have gone
    let n = FRAC_1_SQRT_2;
    let shape = json::obj(vec![("cut", json::arr(vec![json::nums(&[n, n, 0.0, 0.6 * n])]))]);
    let mut p = room(vec![with(a_box("sloped", c, [1.0, 1.0, 1.0], "white", &[]), "shape", shape)]);
    let across = |x: f64, y: f64| ([c[0] + x, c[1] + y, c[2] - 3.0], [c[0] + x, c[1] + y, c[2] + 3.0]);
    let g = &mut p.g;
    // a line through the corner that has gone: x + y = 1.5, in the plain box's square
    let (a, b) = across(0.75, 0.75);
    assert!(!segment_blocked(g, a, b, None, None, None) && !solid_blocked(g, a, b, None) && !shot_blocked(g, a, b, None, None), "the corner cut off stops a line");
    assert!(raycast(g, a, [0.0, 0.0, 1.0], 6.0).is_none(), "the corner cut off stops a ray");
    // and through what is left of it
    let (a, b) = across(-0.5, -0.5);
    assert!(segment_blocked(g, a, b, None, None, None) && solid_blocked(g, a, b, None) && shot_blocked(g, a, b, None, None), "what is left of it stops no line");
    assert!(raycast(g, a, [0.0, 0.0, 1.0], 6.0).is_some_and(|h| (h.t - 2.0).abs() < 0.01), "what is left of it stops no ray at its face");
}

/// THE RULE: a plank leaned over stops a line where it leans, and not where it would lie flat, to an eye (the near walk), a round (the long walk) and a ray.
/// A FAILURE MEANS: men see through a fallen slab, or are hidden by the air beside it.
#[test]
fn a_tilted_plank_stops_a_line_where_it_leans() {
    let c = [18.0, 1.6, 14.0];
    // 30 degrees up about the z axis: its +x end stands 0.75 m above its middle at 1.3 m out
    let (s, w) = ((15.0f64).to_radians().sin(), (15.0f64).to_radians().cos());
    let mut p = room(vec![with(a_box("plank", c, [1.5, 0.1, 0.5], "white", &[]), "rot", json::nums(&[0.0, 0.0, s, w]))]);
    let across = |x: f64, y: f64| ([c[0] + x, c[1] + y, c[2] - 2.0], [c[0] + x, c[1] + y, c[2] + 2.0]);
    let g = &mut p.g;
    let (a, b) = across(1.1, 0.635);
    assert!(segment_blocked(g, a, b, None, None, None), "up its slope the plank stops no eye");
    assert!(solid_blocked(g, a, b, None), "up its slope the plank stops no solid line");
    assert!(shot_blocked(g, a, b, None, None), "up its slope the plank stops no round");
    assert!(raycast(g, a, [0.0, 0.0, 1.0], 4.0).is_some_and(|h| (h.t - 1.5).abs() < 0.01), "up its slope the ray does not meet its side");
    // where it would lie if it were flat, the same distance out, there is air under it
    let (a, b) = across(1.1, 0.0);
    assert!(!segment_blocked(g, a, b, None, None, None) && !solid_blocked(g, a, b, None) && !shot_blocked(g, a, b, None, None), "under its slope a line is stopped");
    assert!(raycast(g, a, [0.0, 0.0, 1.0], 4.0).is_none(), "under its slope a ray meets something");
}

/// THE RULE: a ray that meets a turned box reads the face it met, turned with the box: a round square on to the face comes back along itself. CONTROL: the same box not turned.
/// A FAILURE MEANS: marks on a turned wall lie along the wrong face, a bounce leaves it the wrong way.
#[test]
fn a_ray_reads_the_face_of_a_turned_box() {
    for yaw in [0.0, 0.6, -1.1] {
        let c = [18.0, 1.5, 14.0];
        let mut p = room(vec![with(a_box("turned", c, [2.0, 1.5, 0.2], "white", &[]), "yaw", json::n(yaw))]);
        // the box's own +z face looks along (sin yaw, cos yaw): a ray coming down that line
        let face = [yaw.sin(), 0.0, yaw.cos()];
        let from = [c[0] + face[0] * 3.0, 1.2, c[2] + face[2] * 3.0];
        let hit = raycast(&mut p.g, from, [-face[0], 0.0, -face[2]], 6.0).expect("the box");
        let back = hit.n[0] * face[0] + hit.n[1] * face[1] + hit.n[2] * face[2];
        assert!(back > 0.999 && (hit.t - 2.8).abs() < 0.01, "yaw {yaw}: the normal {:?} is {back:.3} along the face, {:.2} m off", hit.n, hit.t);
    }
}

/// THE RULE: a thing right beside the start of a line is in its way, whichever way the line runs: a thin wall in the same square of the grid as the eye, walked from either side, along either axis.
/// CONTROL: the same line stopped short of the wall is clear.
/// A FAILURE MEANS: a man with his nose to a wall sees through it.
#[test]
fn a_wall_at_the_start_of_a_line_stops_it() {
    let cs = game::scene_world::world().grid_cell;
    let col = (18.0 / cs).floor();
    let x = (col + 0.5) * cs;
    let mut p = room(vec![a_box("post", [x, 1.5, 12.0], [0.06, 1.5, cs * 2.0], "white", &[])]);
    let g = &mut p.g;
    for (from, to, name) in [(x - 0.2, x + 1.5 * cs, "from its low side"), (x + 0.2, x - 1.5 * cs, "from its high side")] {
        assert!(segment_blocked(g, [from, 1.3, 12.0], [to, 1.3, 12.0], None, None, None), "x-major, {name}: the eye sees through the wall");
        assert!(solid_blocked(g, [from, 1.3, 12.0], [to, 1.3, 12.0], None), "x-major, {name}: a solid line passes the wall");
        let short = from + (x - from) * 0.5;
        assert!(!segment_blocked(g, [from, 1.3, 12.0], [short, 1.3, 12.0], None, None, None), "x-major, {name}: stopped short of the wall");
    }
    drop(p);
    let row = (14.0 / cs).floor();
    let zz = (row + 0.5) * cs;
    let mut p = room(vec![a_box("post", [14.0, 1.5, zz], [cs * 2.0, 1.5, 0.06], "white", &[])]);
    let g = &mut p.g;
    for (from, to, name) in [(zz - 0.2, zz + 1.5 * cs, "from its low side"), (zz + 0.2, zz - 1.5 * cs, "from its high side")] {
        assert!(segment_blocked(g, [14.0, 1.3, from], [14.0, 1.3, to], None, None, None), "z-major, {name}: the eye sees through the wall");
    }
}

/// THE RULE: a line that ends on the face of a wall is not stopped by that wall (a mark on the wall is seen), and one that ends deep in it is. A FAILURE MEANS: nobody can see a thing stuck to a wall.
#[test]
fn a_line_that_ends_on_a_face_is_not_stopped_by_it() {
    let mut p = room(vec![a_box("slab", [18.0, 1.5, 14.0], [0.5, 1.5, 3.0], "white", &[])]);
    let g = &mut p.g;
    let face = 17.5;
    assert!(!segment_blocked(g, [14.0, 1.3, 14.0], [face + 0.01, 1.3, 14.0], None, None, None), "ending a centimetre into the face, the line is stopped");
    assert!(!segment_blocked(g, [14.0, 1.3, 14.0], [face, 1.3, 14.0], None, None, None), "ending on the face, the line is stopped");
    assert!(segment_blocked(g, [14.0, 1.3, 14.0], [face + 0.1, 1.3, 14.0], None, None, None), "ending ten centimetres into it, the line is not stopped");
}

/// THE RULE: a ray for what is thrown or kicked meets the furniture where it stands. CONTROL: a metre beside it, nothing.
/// A FAILURE MEANS: a thrown can flies through a chair.
#[test]
fn a_ray_meets_the_furniture_where_it_stands() {
    let chair = json::obj(vec![("id", json::s("chair")), ("kind", json::s("chair")), ("x", json::n(18.0)), ("z", json::n(14.0)), ("mass", json::n(8.0)),
        ("parts", json::arr(vec![json::obj(vec![("off", json::nums(&[0.0, 0.5, 0.0])), ("h", json::nums(&[0.3, 0.5, 0.3])), ("material", json::s("white"))])]))]);
    let mut p = room_props(Vec::new(), vec![chair]);
    let g = &mut p.g;
    let hit = things_ray(g, [15.0, 0.5, 14.0], [1.0, 0.0, 0.0], 6.0).expect("a ray at the chair flies through it");
    assert!((hit.t - 2.7).abs() < 0.02, "the ray meets the chair {:.2} m off", hit.t);
    assert!(things_ray(g, [15.0, 0.5, 15.0], [1.0, 0.0, 0.0], 6.0).is_none(), "a metre beside the chair a ray meets it");
    assert!(things_ray(g, [15.0, 0.5, 14.0], [1.0, 0.0, 0.0], 2.0).is_none(), "a ray that ends short of the chair meets it");
}
