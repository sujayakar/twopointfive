//! A HOLE A BLAST OPENS CAN BE WALKED THROUGH, AND WHAT THE WALK GRID CALLS FLOOR, FEET CROSS.
mod world_ask;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use world_ask::{a_box, scene_with, WallAsk};

const SEEDS: [u32; 3] = [1, 2, 3];
fn seed(seed: u32) -> Opts { Opts { seed, ..Default::default() } }
fn hands_off(p: &mut Play) { p.g.god_mode = true; p.g.ai_enabled = false; p.g.tune.chaos = 0.5; p.run(0.1); }

/// Two rooms with a partition at x 18 and a door well away; a chest of drawers stood hard against the partition's east face at `z`.
fn chest_behind_a_partition(z: f64, s: u32) -> Play {
    let mut sc = Scene::new("chest-behind");
    sc.room("west", 10.0, 8.0, 18.0, 18.0).room("east", 18.0, 8.0, 26.0, 18.0).door("yard", Axis::Z, 10.0, 15.4);
    sc.sam(2.25, 2.25);
    scene_with(&sc, vec![a_box("chest", [18.36, 0.55, z], [0.3, 0.55, 0.45], "white", &[])], &seed(s))
}
/// where a body goes through a hole in the partition at x 18, between z0 and z1: the first lane that lets it through
fn lane_through(p: &mut Play, z0: f64, z1: f64) -> Option<f64> {
    let (from, to) = (17.4, 19.2);
    let mut z = z0;
    while z <= z1 { if dist(p.body_walks([from, z], [to, z]), [to, z]) < 0.05 { return Some(z); } z += 0.1; }
    None
}

/// THE RULE: a 40 mm into a partition that opens it takes the thing that stood hard against the hole's far side: a body as wide as a man goes through. (The wall shielded a chest of
/// drawers from the burst, so that neither the wall's rule nor the furniture's touched it: the records office's front room, a hole with a chest standing in it.)
/// CONTROL: nothing of that size is the rule's to take: the same burst, the same hole, the chest stands in it and the body stops. And no burst at all: no lane.
/// A FAILURE MEANS: a blast opens a hole nobody can use.
#[test]
fn a_hole_a_blast_opens_can_be_walked_through() {
    for s in SEEDS {
        let z = 13.0 + 0.5 * nudge(s, 0);
        let mut p = chest_behind_a_partition(z, s);
        hands_off(&mut p);
        assert_eq!(lane_through(&mut p, z - 1.0, z + 1.0), None, "seed {s}: a lane through a wall nothing has touched");
        p.burst_at([17.79, 1.0, z]);
        p.run(0.05);
        assert!(p.holes() > 0, "seed {s}: the burst opened nothing");
        assert!(lane_through(&mut p, z - 1.0, z + 1.0).is_some(), "seed {s}: a hole and no lane through it ({} holes)", p.holes());
        drop(p);

        let mut p = chest_behind_a_partition(z, s);
        hands_off(&mut p);
        p.g.k.furn.mouth.vol = 0.0;
        p.burst_at([17.79, 1.0, z]);
        p.run(0.05);
        assert!(p.holes() > 0, "seed {s}: the control's burst opened nothing");
        assert_eq!(lane_through(&mut p, z - 0.15, z + 0.15), None, "seed {s}: the chest is not taken and a body goes through it");
    }
}

/// THE PLACE of the maintainer's tape on the pass (10-17: "one set of guys couldn't pathfind through the wall breach"): the store's outer wall, sacks to the beams against it, set from the yard.
const STORE: [f64; 3] = [31.3, 0.0, 5.0];
fn the_pass(s: u32) -> Play { let mut p = Play::level("pass.json", false, &seed(s)); hands_off(&mut p); p }

/// THE RULE: his breach is a hole, the grid's way goes straight through it, and a man walked along that way with the walkers' own collision call gets through, whatever the blast left of the
/// pile lying under the bake's band. CONTROL: at chaos 0 a charge opens nothing: no such way.
/// A FAILURE MEANS: the pair sent through a hole walk on the spot at the wall's line, as on the tape.
#[test]
fn a_man_on_the_grids_way_gets_through_the_breach() {
    for s in [1u32, 2] {
        let (a, b) = ([31.25 + 0.5 * (nudge(s, 0) * 0.3), 3.75], [31.25, 6.75]);
        let mut p = the_pass(s);
        let c = p.sam_sets_charge(STORE, [31.3, 3.6]).unwrap_or_else(|why| panic!("seed {s}: {why}"));
        assert!(p.charge_blows(c), "seed {s}: the charge opened no hole");
        p.run(0.8);
        let (len, end) = p.body_walks_the_grid(a, b).unwrap_or_else(|| panic!("seed {s}: the grid has no way"));
        assert!(len < 5.0, "seed {s}: the grid's way is {len:.1} m for 3 m in a line");
        assert!(dist(end, b) < 0.35, "seed {s}: he ends {:.2} m from the far point, at {end:?}: the wall's line is z 5", dist(end, b));
        drop(p);

        let mut p = the_pass(s);
        p.g.tune.chaos = 0.0;
        let way = p.sam_sets_charge(STORE, [31.3, 3.6]).ok().and_then(|c| { p.charge_blows(c); p.run(0.8); p.body_walks_the_grid(a, b) });
        assert!(way.map_or(true, |(len, _)| len >= 5.0), "seed {s}: at chaos 0 the grid goes straight through the wall: {way:?}");
    }
}

/// THE RULE: a slab 0.2 m high laid across open floor is floor to the grid, and a man walked along the grid's way crosses it. CONTROL: 0.5 m high it is not floor, and the grid goes round it.
/// A FAILURE MEANS: the grid and the feet are two models of what can be walked on: a route straight across something that pushes a walker back.
#[test]
fn what_the_walk_grid_calls_floor_feet_cross() {
    for (top, floor) in [(0.2, true), (0.5, false)] {
        for s in SEEDS {
            let z = 12.0 + 0.5 * nudge(s, 0);
            let mut sc = Scene::new("slab");
            sc.room("room", 6.0, 8.0, 26.0, 18.0).door("yard", Axis::Z, 6.0, 15.4);
            let mut p = scene_with(&sc, vec![a_box("slab", [16.0, top / 2.0, z], [8.0, top / 2.0, 0.15], "white", &[])], &seed(s));
            hands_off(&mut p);
            assert_eq!(p.grid_floor([16.0, z]), floor, "seed {s}: top {top} m: the grid under it");
            let (a, b) = ([16.0, z - 1.0], [16.0, z + 1.0]);
            let (len, end) = p.body_walks_the_grid(a, b).expect("a way");
            if floor { assert!(len < 2.5 && dist(end, b) < 0.35, "seed {s}: top {top} m: way {len:.1} m, ends {end:?}"); } else { assert!(len > 8.0, "seed {s}: top {top} m: way {len:.1} m straight over it"); }
        }
    }
}

/// The wall in the maintainer's screenshot of the barracks, between two bunk rooms' windows: two cots stand against its inner face the whole of its length.
const BARRACKS_WALL: f64 = 24.12;

/// THE RULE (the maintainer on the barracks, 10-11: "weird that it doesn't let me use a wall charge here"): the floor behind a wall with a cot stood against it counts, because the charge that
/// opens the wall takes the cot; the hole is then a way through. CONTROL: the game's own question without the cot's rule, on the same places: the grid alone has no floor behind it.
/// A FAILURE MEANS: a wall a player can see a bed through is refused a charge, or a charge is taken where no body can follow it.
#[test]
fn a_cot_against_a_wall_does_not_refuse_it_a_charge() {
    for s in SEEDS {
        let x = 20.3 + 0.2 * nudge(s, 0);
        let (at, from) = ([x, 0.0, BARRACKS_WALL], [x, 25.6]);
        let mut p = Play::level("barracks.json", false, &seed(s));
        hands_off(&mut p);
        assert!(!p.floor_behind(at, from, false), "seed {s}: the staging: the grid alone has floor behind the wall at x {x:.2}");
        assert!(p.floor_behind(at, from, true), "seed {s}: a cot behind it and refused: no floor behind it");
        let c = p.sam_sets_charge(at, from).unwrap_or_else(|why| panic!("seed {s}: {why}"));
        assert!(p.charge_blows(c), "seed {s}: no hole");
        p.run(0.5);
        let (len, end) = p.body_walks_the_grid([x, 24.9], [x, 22.5]).unwrap_or_else(|| panic!("seed {s}: no way through the hole"));
        assert!(len < 5.0 && dist(end, [x, 22.5]) < 0.35, "seed {s}: way {len:.1} m, ends {end:?}");
    }
}
