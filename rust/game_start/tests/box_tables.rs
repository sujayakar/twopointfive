//! THE TICK LISTS THE BOX WORLD ITSELF (`game_step_seats` → `scene_boxes::box_world_listed`).
//!
//! What this holds. What a tick's rays meet, and which rows of the frame's table are free again, are the game's: they used to wait for a host to call the
//! listing, so a host that drew seldom kept frame rows longer than one that drew every frame, and the numbers of every box dealt after that parted between them.
//! Here a game is stepped with nothing round the step at all: no listing, no lights, no camera.
use game_start::play::{Opts, Play};

const DT: f64 = 1.0 / 60.0;
/// on the office's partition, where the level's wall 131 takes a charge (tests/wallcharge.rs)
const WALL: [f64; 3] = [12.06, 0.0, 13.78];

/// the game's update alone: what a host that only steps calls
fn step(p: &mut Play, n: usize) { for _ in 0..n { game::game::game_update(&mut p.g, DT, p.host.input, p.host.cam, p.host.canvas); p.g.out.clear(); } }

/// THE RULE: with no host round it, a tick opens on the list of what stands as it opens, a game that has no list yet included, and the frame's table holds no
/// more rows than the frames need.
/// A FAILURE MEANS: something of the box world waits for a host again; a host that never called it would keep every frame box as a row for good.
#[test]
fn a_step_lists_the_box_world_itself() {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: 5, ..Default::default() });
    p.sam_unhurt();
    step(&mut p, 6);
    game::wallcharge::wall_charge_raise_carried(&mut p.g, 2.0);
    p.sam_put([14.6, 15.2]);
    step(&mut p, 1);
    if let Some(why) = game::wallcharge::wall_charge_order(&mut p.g, WALL, None) { panic!("the wall charge was refused: {why}"); }
    for _ in 0..3600 { step(&mut p, 1); if !p.g.sets[p.g.seat].wall_charge.set.is_empty() { break; } }
    let c = *p.g.sets[p.g.seat].wall_charge.set.first().expect("a minute and no charge is on the wall");
    game::wallcharge::wall_charge_blow(&mut p.g, Some(c), Some(true));
    step(&mut p, 120);
    let at_two_seconds = p.g.tab.frame_boxes.len();
    step(&mut p, 600);
    // the tick brings the ray grid up with no host's call: a grid thrown away is there again after one tick
    p.g.ray_grid = Default::default();
    step(&mut p, 1);
    assert!(p.g.ray_grid.get::<game::scene_boxes::RayGrid>().is_some(), "the tick brought no grid up");
    assert!(p.g.tab.frame_boxes.len() < at_two_seconds + 200, "the frame's table grew from {} to {} rows in ten seconds: nothing freed them", at_two_seconds, p.g.tab.frame_boxes.len());
}
