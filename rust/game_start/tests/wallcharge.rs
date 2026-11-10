//! THE WALL CHARGE CATCHES WHOM ITS WARNING NAMED (rust/game_rules/src/entries/wallcharge/blow.rs, `wall_charge_blow_raw`).
//!
//! What this holds. In the office a charge is set on a partition (the level's wall 131, at x 12). Sam is put 2.5 m along that wall on the charge's
//! side, but round the corner of a second partition (the wall along z 12.2), which stands on the line from the charge to him. The TOO CLOSE
//! warning asks "who does this catch" before the charge goes and does not name him. The blast opens both walls. Before the mend everything on
//! the charge's own side was asked again AFTER the blast, each test with a line of its own, and Sam, with nothing left between him and the
//! charge, was rocked at half its strength and blinded for two seconds: caught by a blast whose warning had not named him. Now the question is
//! asked once, as the charge goes, by the warning's own call.
//!
//! The wall charge's own calls are used as the game has them (`wall_charge_order`, `wall_charge_too_close`, `wall_charge_blow`): Play has no
//! words for them yet.
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [5, 6, 7];
/// on the partition's east face, where the level's wall 131 takes a charge
const WALL: [f64; 3] = [12.06, 0.0, 13.78];
/// 2.5 m along the wall from the charge and half a metre off its face, north of the second partition
const ROUND_THE_CORNER: [f64; 2] = [12.597, 11.338];
/// a metre and a half along the same face in the charge's own room, nothing between
const IN_THE_OPEN: [f64; 2] = [12.7, 15.3];

/// The office's room clear with a charge set on the partition from its east side, and Sam put at `sam`. The charge, and who the warning names.
fn set_and_warned(seed: u32, sam: [f64; 2]) -> (Play, game::ids::WallChargeId, Vec<String>) {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed, ..Default::default() });
    p.sam_unhurt();   // (he is put beside a charge here)
    p.run(0.1);
    game::wallcharge::wall_charge_raise_carried(&mut p.g, 2.0);   // (the mission's own issue: the squad brings charges)
    p.sam_put([14.6, 15.2]);   // (a wall is worked from Sam's side of it: east)
    p.frame();
    if let Some(why) = game::wallcharge::wall_charge_order(&mut p.g, WALL, None) { panic!("seed {seed}: the wall charge was refused: {why}"); }
    let set = p.until(60.0, |p| !p.g.sets[p.g.seat].wall_charge.set.is_empty());
    assert!(set.is_some(), "seed {seed}: a minute and no charge is on the wall ({})", p.roll());
    let c = p.g.sets[p.g.seat].wall_charge.set[0];
    p.sam_put(sam);
    p.frame();
    let named = game::wallcharge::wall_charge_too_close(&mut p.g, Some(c));
    (p, c, named)
}

/// Who of the squad the blast caught on its own side: blinded, or down.
fn caught(p: &Play) -> Vec<String> {
    p.mates().into_iter().filter(|&m| !p.up(m) || p.blind(m)).map(|m| p.g.gd[m].callsign.clone()).collect()
}

/// THE RULE: a man the warning did not name is not caught. Sam stands round the corner of a wall the blast will open.
/// A FAILURE MEANS: the blast is deciding who it catches on its own side after it has changed the room (Sam rocked at 0.50, blind 2 s).
#[test]
fn a_man_the_warning_did_not_name_is_not_caught() {
    for s in SEEDS {
        let (mut p, c, named) = set_and_warned(s, ROUND_THE_CORNER);
        assert!(!named.iter().any(|n| n == "you"), "seed {s}: the staging is wrong: the warning names Sam ({named:?})");
        game::wallcharge::wall_charge_blow(&mut p.g, Some(c), Some(true));
        let blow = p.g.sets[p.g.seat].wall_charge.last_blow.expect("the charge went");
        assert_eq!(blow.sam, 0.0, "seed {s}: Sam was not named and took {:.2} of the blast", blow.sam);
        assert!(!p.sam_blind(), "seed {s}: Sam was not named and the blast has his eyes");
        let mut hit = caught(&p);
        let mut said: Vec<String> = named.into_iter().filter(|n| n != "you").collect();
        hit.sort(); said.sort();
        assert_eq!(hit, said, "seed {s}: the men of the squad the blast caught are not the men the warning named ({})", p.roll());
    }
}

/// THE CONTROL: in the open beside the charge the warning names him and the blast has him, so the rule above is not a blast that catches nobody.
#[test]
fn a_man_the_warning_named_is_caught() {
    for s in SEEDS {
        let (mut p, c, named) = set_and_warned(s, IN_THE_OPEN);
        assert!(named.iter().any(|n| n == "you"), "seed {s}: Sam is a metre and a half from the charge in the open and the warning does not name him ({named:?})");
        game::wallcharge::wall_charge_blow(&mut p.g, Some(c), Some(true));
        let blow = p.g.sets[p.g.seat].wall_charge.last_blow.expect("the charge went");
        assert!(blow.sam > 0.0 && p.sam_blind(), "seed {s}: Sam was named and took {:.2}, blind {}", blow.sam, p.sam_blind());
    }
}

/// the pass's file with every box's `shape` taken out: the level as it is with the shapes down
fn plain_pass() -> std::path::PathBuf {
    use game_start::json::{self, Json};
    let mut level = json::parse(&game_start::files::level_text("pass.json")).unwrap();
    let Json::Obj(members, _) = &mut level else { panic!("a level is an object") };
    let (_, boxes) = members.iter_mut().find(|(k, _)| k == "boxes").expect("the pass has boxes");
    let Json::Arr(boxes) = boxes else { panic!("boxes is a list") };
    for b in boxes.iter_mut() { if let Json::Obj(m, _) = b { m.retain(|(k, _)| k != "shape"); } }
    let path = std::env::temp_dir().join(format!("pass-plain-{}.json", std::process::id()));
    std::fs::write(&path, json::to_text(&level)).unwrap();
    path
}

/// half a metre off each side of the first boxes on the pass that wear trim, with the box each is beside
fn beside_trim() -> Vec<([f64; 3], [f64; 3])> {
    let p = Play::level("pass.json", false, &Opts { seed: 1, ..Default::default() });
    let mut owners: Vec<usize> = p.g.col.boxes.iter().filter_map(|&b| p.g.tab.boxes[b].of.filter(|_| p.g.tab.boxes[b].piece == Some(true))).map(|o| o as usize).collect();
    owners.sort();
    owners.dedup();
    assert!(owners.len() >= 16, "the pass's boxes that wear trim: {}", owners.len());
    let mut out = Vec::new();
    for &o in &owners[..16] {
        let t = &p.g.tab.boxes[p.g.col.boxes[o]];
        for (dx, dz) in [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)] { out.push(([t.c[0] + dx * (t.h[0] + 0.55), 0.6, t.c[2] + dz * (t.h[2] + 0.55)], t.c)); }
    }
    out
}

/// Each point in turn: a charge on the wall there if a wall takes one, then a bare burst. After each, the boxes of the level that are out of the
/// picture and what the walls have told, one line a point.
fn blown(level: &str, points: &[([f64; 3], [f64; 3])]) -> Vec<String> {
    let mut p = Play::level(level, false, &Opts { seed: 1, ..Default::default() });
    p.g.god_mode = true;
    p.g.tune.chaos = 0.5;
    p.drop_all_but(&[]);
    let own = p.g.col.boxes.iter().position(|&b| p.g.tab.boxes[b].piece == Some(true)).unwrap_or(p.g.col.boxes.len());
    let hidden = game::js::to_i32(game::scene_boxes::box_flag().no_primary);
    let mut out = Vec::new();
    for &(at, from) in points {
        if let Some(w) = game::wallcharge::wall_at(&mut p.g, at, Some(from)).filter(|w| w.ok) {
            let breach = if w.outer == Some(true) && game::wallbreak::wall_break_adopt(&mut p.g, w.index) { Some(w.index) } else { None };
            let hung = Some(game::wallbreak::BlastOptsHung { wall: w.index, r: p.g.k.wallcharge.hung_m });
            let pt = game::wallcharge::wall_pt(&p.g, &w, 0.0, 0.02, Some(1.0));
            game::destruction::destruction_blast(&mut p.g, pt, Some(game::wallbreak::BlastOpts { who: Some(1.0), chaos: Some(false), breach, hung, ..Default::default() }));
            p.run(1.0 / 6.0);
        }
        game::destruction::destruction_blast(&mut p.g, at, Some(game::wallbreak::BlastOpts { who: Some(1.0), radius: Some(2.4), ..Default::default() }));
        p.run(1.0 / 3.0);
        let gone: Vec<usize> = (0..own).filter(|&i| game::js::to_i32(p.g.tab.boxes[p.g.col.boxes[i]].flags) & hidden != 0).collect();
        let told: Vec<String> = p.g.destr.walls.changes.iter().map(|c| format!("{:?} {}", c.kind, c.id)).collect();
        out.push(format!("{gone:?} | {told:?}"));
    }
    out
}

/// THE RULE: a blast plays the same whether the level's drawn shapes stand or not: a charge opens the same wall and takes the same boxes on the pass
/// with its shapes up and with them taken out of the file.
/// A FAILURE MEANS: a rule of the blast reads the trim, so what a level is dressed in decides what breaks.
#[test]
fn a_blast_takes_the_same_with_the_shapes_down() {
    let points = beside_trim();
    let plain = plain_pass();
    let (up, down) = (blown("pass.json", &points), blown(plain.to_str().unwrap(), &points));
    std::fs::remove_file(&plain).ok();
    let holes = up.last().unwrap().matches("Hole").count();
    assert!(holes >= 1, "no hole opened, so this asked nothing: {}", up.last().unwrap());
    for (i, (a, b)) in up.iter().zip(down.iter()).enumerate() { assert_eq!(a, b, "point {i} at {:?}", points[i].0); }
}
