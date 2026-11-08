//! WHAT BREAKS OFF A THING IS MADE OF THAT THING (rust/game_rules/src/break_table, rust/game_rules/src/edge_look).
//!
//! The Pass's inside partitions are mud (`mudLight`: stuff "mud wall", does "plasterboard": a charge cuts it, a round goes through). A charge is set on one
//! by the game's own order and blown by the game's own call, and what is on the floor a few seconds on is asked what it is and where it lies.
use game::scene_boxes::Box as Bx;
use game::scene_stuff::BoxStuff;
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [5, 6, 7];

fn hostiles(p: &Play) -> Vec<game::ids::GuardId> {
    let mates = p.mates();
    p.g.guards.iter().copied().filter(|gd| !mates.contains(gd) && p.up(*gd)).collect()
}

/// A mission with a charge set on the wall at `wall` from Sam's side, `sam`.
fn charge_set(level: &str, link: &str, seed: u32, wall: [f64; 3], sam: [f64; 2]) -> (Play, game::ids::WallChargeId) {
    let mut p = Play::mission(level, link, &Opts { seed, dress: true, ..Default::default() });
    p.g.god_mode = true;   // (nothing hurts Sam, who stands near a charge here)
    p.run(0.1);
    game::wallcharge::wall_charge_raise_carried(&mut p.g, 2.0);   // (the mission's own issue: the squad brings charges)
    // (the house is taken: this is about what a blast leaves, and a squad shot on its way to the wall sets nothing. One stays so that the mission runs on)
    let hostiles: Vec<game::ids::GuardId> = hostiles(&p);
    for &gd in hostiles.iter().skip(1) { game::combat::kill_guard(&mut p.g, gd, [1.0, 0.0, 0.0], Some(true), None, None, None, None); }
    p.sam_put(sam);
    p.frame();
    if let Some(why) = game::wallcharge::wall_charge_order(&mut p.g, wall, None) { panic!("seed {seed}: the wall charge was refused: {why}"); }
    p.until(90.0, |p| !p.g.wall_charge.set.is_empty()).unwrap_or_else(|| panic!("seed {seed}: no charge on the wall ({})", p.roll()));
    let c = p.g.wall_charge.set[0];
    (p, c)
}

fn pass(seed: u32) -> (Play, game::ids::WallChargeId) {
    charge_set("pass.json", "?levelFile=pass.json&roomclear&squad4", seed, MUD_WALL, [24.0, 9.0])
}
fn office(seed: u32) -> (Play, game::ids::WallChargeId) {
    charge_set("office.json", "?levelFile=office.json&roomclear&squad4", seed, [12.06, 0.0, 13.78], [14.6, 15.2])
}
/// on the east face of The Pass's partition at x 22
const MUD_WALL: [f64; 3] = [22.06, 0.0, 9.0];

/// every box the game keeps of what a blast broke off walls: the pool's pieces of a wall's row, the rubble at the wall's foot
fn pieces(p: &Play) -> Vec<Bx> {
    use game::break_table::Mat;
    let mut out: Vec<Bx> = p.g.destr.debris.list.iter().map(|&c| &p.g.tab.debris_chunk[c])
        .filter(|c| matches!(c.mat.map(|m| m.mat), Some(Mat::Plasterboard | Mat::Blockwork)))
        .map(|c| p.g.tab.boxes[c.box_].clone()).collect();
    out.extend(p.g.destr.edge_look.rubble.iter().map(|r| p.g.tab.boxes[r.box_].clone()));
    out
}
/// how much whiter than `paint` a colour is: its mean, less the paint's
fn whiter(c: [f64; 3], paint: [f64; 3]) -> f64 { (c[0] + c[1] + c[2] - paint[0] - paint[1] - paint[2]) / 3.0 }

/// THE RULE: a mud wall blown: every piece says it is mud and does what the wall does, and nothing of it, broken faces included, is whiter than mud.
/// A FAILURE MEANS: a piece takes what it is from the break table's row and not from the wall (the row of a plasterboard partition: white-cored slabs of nothing).
#[test]
fn a_mud_wall_goes_off_as_mud() {
    for s in SEEDS {
        let (mut p, c) = pass(s);
        let paint = [0.56, 0.46, 0.33];   // (levels/pass.json mudLight)
        game::wallcharge::wall_charge_blow(&mut p.g, Some(c), Some(true));
        p.run(3.0);
        let all = pieces(&p);
        assert!(all.len() >= 10, "seed {s}: the blast left {} pieces", all.len());
        for b in all.iter() {
            assert_eq!(b.stuff, Some(BoxStuff::MudWall), "seed {s}: a piece of a mud wall says it is {:?}: at {:?}, half {:?}", b.stuff, b.c, b.h);
            assert!(b.does.is_some(), "seed {s}: a piece of a wall does nothing it was told: at {:?}, half {:?}", b.c, b.h);
            assert!(whiter(b.albedo, paint) < 0.02, "seed {s}: a piece is {:?}, whiter than the wall's mud {paint:?}", b.albedo);
            if let Some(core) = b.cut_core { assert!(whiter(core, paint) < 0.02, "seed {s}: a piece's broken faces are {core:?}, whiter than the wall's mud {paint:?}"); }
        }
        // what still stands round the hole: mud too, and no steel in it
        for rows in p.g.destr.edge_look.decor.values() {
            for &b in rows.iter() {
                let x = &p.g.tab.boxes[b];
                assert!(whiter(x.albedo, paint) < 0.02, "seed {s}: the hole's edge shows {:?}, whiter than the wall's mud {paint:?}", x.albedo);
            }
        }
        let studs = p.g.tab.boxes.iter().filter(|(_, b)| b.name.as_deref() == Some("stud")).count();
        assert_eq!(studs, 0, "seed {s}: a mud wall has no studs to leave standing");
    }
}

/// THE CONTROL: an office partition is plaster over board: its pieces say so, and its broken faces are white, so the rule above is not "everything is brown".
#[test]
fn an_office_partition_goes_off_as_board() {
    for s in SEEDS {
        let (mut p, c) = office(s);
        let paint = [0.66, 0.64, 0.6];   // (levels/office.json wall)
        game::wallcharge::wall_charge_blow(&mut p.g, Some(c), Some(true));
        p.run(3.0);
        let all = pieces(&p);
        assert!(all.len() >= 10, "seed {s}: the blast left {} pieces", all.len());
        let board: Vec<&Bx> = all.iter().filter(|b| matches!(b.stuff, Some(BoxStuff::Plaster | BoxStuff::Skim))).collect();
        let white = board.iter().filter(|b| b.cut_core.map_or(false, |k| whiter(k, paint) > 0.05)).count();
        assert!(white >= 3, "seed {s}: of {} pieces of board {white} show a white break ({} pieces in all)", board.len(), all.len());
    }
}
