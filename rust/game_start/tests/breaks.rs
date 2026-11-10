//! WHAT BREAKS OFF A THING IS MADE OF THAT THING, AND A PIECE LIES ON WHAT IS UNDER IT AND NOT ON A BODY (rust/game_rules/src/breakage/break_table,
//! rust/game_rules/src/breakage/edge_look.rs, rust/game_rules/src/breakage/debris).
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
    for &gd in hostiles.iter().skip(1) { game::combat::kill_guard(&mut p.g, gd, [1.0, 0.0, 0.0], Some(true), None, None, game::game::With::Hand); }
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
        .map(|c| p.g.tab[c.box_].clone()).collect();
    out.extend(p.g.destr.edge_look.rubble.iter().map(|r| p.g.tab.frame_boxes[r.box_].clone()));
    out
}
/// everything at rest on the floor that a blast threw or shed, whatever it was
fn lying(p: &Play) -> Vec<Bx> {
    let mut out: Vec<Bx> = p.g.destr.debris.list.iter().map(|&c| &p.g.tab.debris_chunk[c]).filter(|c| c.rest).map(|c| p.g.tab[c.box_].clone()).collect();
    out.extend(p.g.destr.edge_look.rubble.iter().map(|r| p.g.tab.frame_boxes[r.box_].clone()));
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
                let x = &p.g.tab.frame_boxes[b];
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

/// a man down on the far side of The Pass's partition, by where the charge will open it, and his bones
fn man_down(p: &mut Play, at: [f64; 2]) -> game::ids::GuardId { man_down_facing(p, at, None) }
fn man_down_facing(p: &mut Play, at: [f64; 2], yaw: Option<f64>) -> game::ids::GuardId {
    let gd = p.mates().into_iter().filter(|&gd| p.up(gd)).last().expect("somebody to lay down");   // (the charge is set: the order needs nobody now)
    let c = p.g.gd[gd].ch;
    p.g.ch[c].pos = [at[0], 0.0, at[1]];
    if let Some(yaw) = yaw { (p.g.ch[c].body_yaw, p.g.ch[c].aim_yaw) = (yaw, yaw); }   // (how he falls is the scene's: whichever way he was looking, or the way a check turned him)
    p.frame();
    game::combat::kill_guard(&mut p.g, gd, [1.0, 0.0, 0.0], Some(true), None, None, game::game::With::Hand);
    p.run(3.0);   // (he falls, and is still)
    gd
}
/// the pieces at rest lying over a down man, by a box's plan against his bones: what shows through him. Only what could cover him: a chip beside a limb is no slab on him
fn lying_on(p: &Play, gd: game::ids::GuardId) -> Vec<String> {
    let c = p.g.gd[gd].ch;
    let bones = [game::character::Bone::Hips, game::character::Bone::Chest, game::character::Bone::Head, game::character::Bone::ShinL, game::character::Bone::ShinR];
    let spots: Vec<[f64; 3]> = bones.iter().filter_map(|&k| p.g.ch[c].bones.get(k).copied()).collect();
    assert_eq!(spots.len(), bones.len(), "he has bones");
    let mut out = Vec::new();
    for b in lying(p) {
        let (yaw, reach) = (b.yaw, b.h[0].max(b.h[2]));
        let q = b.rot.unwrap_or_else(|| game::v3::quat::yaw(yaw));
        let inv = [-q[0], -q[1], -q[2], q[3]];
        for s in spots.iter() {
            if (s[0] - b.c[0]).abs() > reach + 0.1 || (s[2] - b.c[2]).abs() > reach + 0.1 { continue; }
            if b.c[1] - b.h[1] > s[1] + 0.1 { continue; }   // (it lies clear above him, on something else that stands: a sill)
            if b.c[1] + b.h[1] < s[1] { continue; }   // (its top is under his bone: he lies on it, and he hides it)
            // his bone in the piece's own frame, level with it: within its outline by more than a hand
            let l = game::v3::quat::rotate(inv, [s[0] - b.c[0], 0.0, s[2] - b.c[2]]);
            if l[0].abs() < b.h[0] - 0.02 && l[2].abs() < b.h[2] - 0.02 && reach >= 0.05 { out.push(format!("a piece at {:?} half {:?} over his bone at {:?}", b.c, b.h, s)); break; }
        }
    }
    out
}

/// THE RULE: blow a mud wall with a man down at its foot: no piece lies over him.
/// A FAILURE MEANS: a body is nothing to the pieces and they lie through him (the "zsorting" of a slab over a dead man).
#[test]
fn a_piece_does_not_come_to_rest_on_a_man_down() {
    for s in SEEDS {
        let (mut p, c) = pass(s);
        let gd = man_down(&mut p, [20.9, 9.0]);
        game::wallcharge::wall_charge_blow(&mut p.g, Some(c), Some(true));
        p.run(8.0);
        let on = lying_on(&p, gd);
        assert!(on.is_empty(), "seed {s}: {} pieces lie over the man down: {}", on.len(), on.join("; "));
    }
}

/// THE RULE: a man who falls with his head or his chest against the wall's foot has no piece on him either, whichever way he was looking when he fell.
/// A FAILURE MEANS: a slab lies on him where a slab has nowhere to slide to: the wall behind him holds it there.
#[test]
fn a_piece_does_not_come_to_rest_on_a_man_at_the_walls_foot() {
    let mut bad = Vec::new();
    for s in [1, 4, 8, 9] {
        for yaw in [core::f64::consts::FRAC_PI_2, -core::f64::consts::FRAC_PI_2] {
            let (mut p, c) = pass(s);
            let gd = man_down_facing(&mut p, [20.9, 9.0], Some(yaw));
            game::wallcharge::wall_charge_blow(&mut p.g, Some(c), Some(true));
            p.run(8.0);
            let on = lying_on(&p, gd);
            if !on.is_empty() { bad.push(format!("seed {s} yaw {yaw:.1}: {}", on.join("; "))); }
        }
    }
    assert!(bad.is_empty(), "{} lie under a piece:\n{}", bad.len(), bad.join("\n"));
}

/// THE CONTROL: the same blast with him left where he stood lays pieces on the spot he would have lain on, so the rule above is not a blast that throws nothing there.
#[test]
fn pieces_do_land_where_he_would_have_lain() {
    let mut there = 0;
    for s in SEEDS {
        let (mut p, c) = pass(s);
        game::wallcharge::wall_charge_blow(&mut p.g, Some(c), Some(true));
        p.run(8.0);
        there += lying(&p).iter().filter(|b| (b.c[0] - 20.9).abs() < 0.8 && (b.c[2] - 9.0).abs() < 0.8).count();
    }
    assert!(there >= 3, "over three seeds only {there} pieces lay where the man would lie: this staging proves nothing");
}

/// The Pass's bunk room with a charge set on its outer wall from outside: the cots go, and big sheets of canvas and steel lie among the wall's pieces.
fn bunk_room(seed: u32) -> (Play, game::ids::WallChargeId) {
    charge_set("pass.json", "?levelFile=pass.json&roomclear&squad4", seed, [34.12, 0.0, 15.0], [36.0, 15.0])
}

/// THE RULE: where two pieces of the pool lie over each other in plan, one lies on the other: no two lie through each other, near one plane (what read as bad depth sorting).
/// A FAILURE MEANS: a piece at rest finds only the floor, and thin sheets lie at nearly one height through each other.
#[test]
fn pieces_lie_on_each_other_not_through_each_other() {
    for s in SEEDS {
        let (mut p, c) = bunk_room(s);
        game::wallcharge::wall_charge_blow(&mut p.g, Some(c), Some(true));
        p.run(8.0);
        let rest: Vec<Bx> = p.g.destr.debris.list.iter().map(|&c| &p.g.tab.debris_chunk[c]).filter(|c| c.rest).map(|c| p.g.tab[c.box_].clone()).collect();
        assert!(rest.len() >= 10, "seed {s}: only {} pieces lie at rest", rest.len());
        let span = |b: &Bx| { let q = b.rot.unwrap_or_else(|| game::v3::quat::yaw(b.yaw)); let up = |a: [f64; 3]| game::v3::quat::rotate(q, a)[1].abs(); up([b.h[0], 0.0, 0.0]) + up([0.0, b.h[1], 0.0]) + up([0.0, 0.0, b.h[2]]) };
        let mut through = Vec::new();
        for i in 0..rest.len() {
            for j in 0..i {
                let (a, b) = (&rest[i], &rest[j]);
                // (a circle well inside each piece's plan: these two certainly overlap)
                let r = |x: &Bx| x.h[0].min(x.h[1]).min(x.h[2]).max(x.h[0] + x.h[1] + x.h[2] - x.h[0].max(x.h[1]).max(x.h[2]) - x.h[0].min(x.h[1]).min(x.h[2]));
                let d = ((a.c[0] - b.c[0]).powi(2) + (a.c[2] - b.c[2]).powi(2)).sqrt();
                if d > (r(a) + r(b)) * 0.7 { continue; }
                let (ah, bh) = (span(a), span(b));
                let gap = (a.c[1] - b.c[1]).abs() - ah - bh;
                if gap < -0.003 { through.push(format!("{:?}+{:?} and {:?}+{:?} (by {:.3})", a.c, a.h, b.c, b.h, -gap)); }
            }
        }
        assert!(through.is_empty(), "seed {s}: {} pairs of pieces lie through each other: {}", through.len(), through.join("; "));
    }
}
