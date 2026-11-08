//! THE WORLD'S BOXES HAVE THE SAME NUMBERS ON EVERY HOST (`g.tab.boxes`, `scene_boxes::box_new`).
//!
//! What this holds. A host lists the box world when it likes: a page as it draws, a server far less often. The listing frees the rows of the frames gone by,
//! so a host that lists seldom needs more rows of the frame than one that lists every frame. The frame's boxes have a table of their own for that reason: when
//! they shared one table with the world's, the first box the world made after that (a wall's piece, a chunk of debris) took a number that said how many frame
//! rows this host had ever needed, and `col.boxes`, the debris rows and everything else that names a world box parted between a page and a server.
use game_start::play::{Opts, Play};
use game::ids::BoxId;

const DT: f64 = 1.0 / 60.0;
/// on the office's partition, where the level's wall 131 takes a charge (tests/wallcharge.rs)
const WALL: [f64; 3] = [12.06, 0.0, 13.78];

/// one frame of a host that lists the box world only when `list` says (stand::frame's lines, with the listing asked)
fn frame(p: &mut Play, list: bool) {
    game::game::game_update(&mut p.g, DT, p.host.input, p.host.cam, p.host.canvas);
    let me = game::game::game_host_seat(&mut p.g);
    let (follow, aim) = game::seats::with_seat(&mut p.g, me, &mut |g| (game::game::game_follow_pos(g, None), g.aim_point));
    game::camera::follow_camera_update(&mut p.g, p.host.cam, DT, follow, Some(aim));
    let now = p.g.time;
    game::render_lights::light_set_update(&mut p.g, DT, now);
    if list { game::host_api::b_01::host_world_upload(&mut p.g); }
    game::input::input_end_frame(&mut p.g, p.host.input);
    p.g.out.clear();
}

/// The office's room clear, a wall charge blown at once, some seconds of debris after it, listed by a host of this cadence. What the game has made of the world.
fn played(every: usize) -> (Vec<BoxId>, usize, Vec<String>) {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: 5, ..Default::default() });
    p.sam_unhurt();
    let mut k = 0;
    let mut step = |p: &mut Play, n: usize| for _ in 0..n { k += 1; frame(p, k % every == 0); };
    step(&mut p, 6);
    game::wallcharge::wall_charge_raise_carried(&mut p.g, 2.0);
    p.sam_put([14.6, 15.2]);
    step(&mut p, 1);
    if let Some(why) = game::wallcharge::wall_charge_order(&mut p.g, WALL, None) { panic!("the wall charge was refused: {why}"); }
    for _ in 0..3600 { step(&mut p, 1); if !p.g.wall_charge.set.is_empty() { break; } }
    let c = *p.g.wall_charge.set.first().expect("a minute and no charge is on the wall");
    game::wallcharge::wall_charge_blow(&mut p.g, Some(c), Some(true));
    step(&mut p, 240);
    // (a chunk that has a piece of rubble for its box names a row of the frame's table, whose number is the host's own: tests/box_tables.rs's head)
    let kept: Vec<String> = p.g.tab.debris_chunk.iter().map(|(_, c)| format!("{:?}", c.box_)).filter(|b| !b.contains("Frame")).collect();
    (p.g.col.boxes.clone(), p.g.tab.boxes.len(), kept)
}

/// THE RULE: a host that lists every fourth frame numbers the world's boxes as one that lists every frame.
/// A FAILURE MEANS: a row of the world is being dealt by something that follows the host's listing.
#[test]
fn a_host_that_lists_seldom_numbers_the_world_alike() {
    let (often, rows_often, chunks_often) = played(1);
    let (seldom, rows_seldom, chunks_seldom) = played(4);
    assert_eq!(rows_often, rows_seldom, "the world's table is not the same length on the two hosts");
    assert_eq!(often, seldom, "`col.boxes` names other rows on the two hosts");
    assert_eq!(chunks_often, chunks_seldom, "a chunk of debris names another box on the two hosts");
}
