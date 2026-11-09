//! A WALL CHARGE KILLS WHO STANDS AT IT, AND THE MEN WITH SAM STEP BACK FROM IT, OUT OF THE KNOWN ENEMIES' FIRE.
mod world_ask;
use game::ids::GuardId;
use game::wallcharge::{wall_at, wall_charge_blast_share, wall_charge_fire_any, wall_charge_known_foes, wall_charge_safe_posts, wall_charge_set_spot, wall_charge_too_close, wall_charge_under_fire, wall_pt, WallSpot};
use game_start::play::{dist, Opts, Play};
use world_ask::WallAsk;

const AT: [f64; 3] = [12.7, 0.0, 7.0];
const FROM: [f64; 2] = [12.7, 6.0];
const STEP_BACK: f64 = 2.2;
/// The mousehole's room clear of four, a charge of Sam's own on its long wall, everybody the line does not name stood off the map, and Sam where it wants him.
fn charged(s: u32, men: &[(usize, [f64; 2])], foe: Option<[f64; 2]>, sam: [f64; 2], god: bool) -> (Play, game::ids::WallChargeId, Vec<GuardId>) {
    let mut p = Play::mission("mousehole.json", "?levelFile=mousehole.json&roomclear&squad4", &Opts { seed: s, ..Default::default() });
    p.g.ai_enabled = false; p.g.god_mode = true; p.g.tune.chaos = 0.5;
    p.run(0.1);
    let c = p.sam_sets_charge(AT, FROM).expect("Sam sets the charge");
    p.g.god_mode = god;
    let (ours, foes) = (p.mates(), p.hostiles());
    for (i, &m) in ours.iter().chain(foes.iter()).enumerate() { p.man_stands(m, [300.0 + i as f64 * 2.0, 300.0]); }
    let mut named = Vec::new();
    for &(i, q) in men { p.man_stands(ours[i], q); named.push(ours[i]); }
    if let Some(q) = foe { p.man_stands(foes[0], q); named.push(foes[0]); }
    p.sam_put(sam);
    p.frame();
    (p, c, named)
}
fn sam_down(p: &Play) -> bool { p.g.pl[p.g.player].down }
fn sam_blind(p: &Play) -> bool { p.g.time < p.g.cond[p.g.pl[p.g.player].ch].blind_until }

/// THE RULE (the maintainer, 10-19, 0.9 m from a charge he had set: "blowing a wall charge with me right next to it should kill me"): the player at the charge goes down. CONTROL: god mode keeps him up.
/// A FAILURE MEANS: the charge is a thing to stand beside.
#[test]
fn a_charge_kills_the_player_who_stands_at_it() {
    for (i, s) in [1u32].into_iter().enumerate() {
        let near = [AT[0] + 0.37, AT[2] - 0.9 - 0.02 * i as f64];
        let (mut p, c, _) = charged(s, &[], None, near, false);
        p.charge_blows(c);
        p.run(0.3);
        assert!(sam_down(&p), "seed {s}: 0.9 m from it and he stands");
        drop(p);
        let (mut p, c, _) = charged(s, &[], None, near, true);
        p.charge_blows(c);
        assert!(!sam_down(&p), "seed {s}: down in god mode");
    }
}

/// THE CONTROL: past the near reach, inside the safety radius, he lives, blind.
#[test]
fn past_the_near_reach_the_player_lives_blind() {
    for s in [1u32] {
        let (mut p, c, _) = charged(s, &[], None, [AT[0], AT[2] - 1.9], false);
        let reach = p.g.k.wallkill.near_m;
        assert!(reach < 1.9, "the staging: 1.9 m is inside the near reach of {reach} m");
        p.charge_blows(c);
        p.frame();
        assert!(!sam_down(&p) && sam_blind(&p), "seed {s}: past {reach} m: down {}, blind {}", sam_down(&p), sam_blind(&p));
    }
}

/// THE RULE ("a blast does to the player what it does to anyone"): one of ours and a defender at the charge are dead, not out cold. CONTROL: one of ours a step and a half back is blind and no worse, and the player clear of the radius feels nothing.
/// A FAILURE MEANS: a charge is kinder to the men than to the player, or it reaches past what the row warned of.
#[test]
fn a_charge_kills_anyone_at_it_and_only_them() {
    for s in [1u32, 2] {
        let (mut p, c, men) = charged(s, &[(0, [AT[0] - 0.5, AT[2] - 0.8]), (1, [AT[0], AT[2] - STEP_BACK])], Some([AT[0] + 0.6, AT[2] - 0.9]), [5.0, 3.0], false);
        assert!(p.g.k.wallkill.near_m + 0.3 < STEP_BACK, "the staging: the step back is inside the near reach");
        let warned = wall_charge_too_close(&mut p.g, Some(c));
        assert!(warned.contains(&p.g.gd[men[0]].callsign), "seed {s}: the row did not name the man at it: {warned:?}");
        p.charge_blows(c);
        p.frame();
        assert!(!p.up(men[0]) && !p.up(men[2]), "seed {s}: at the charge: ours up {}, the defender up {}", p.up(men[0]), p.up(men[2]));
        assert!(p.up(men[1]) && p.blind(men[1]), "seed {s}: a step and a half back: up {}, blind {}", p.up(men[1]), p.blind(men[1]));
        assert!(!sam_down(&p) && !sam_blind(&p), "seed {s}: the player clear of it felt it");
    }
}

/// The pass of four, the yard, the men bunched beside Sam at the foot of the bunk room's long wall, on the order given; nobody to fight (each stands far off).
const CHARGE: [f64; 3] = [31.28, 0.0, 18.14];
const CLEAR: [f64; 2] = [20.5, 21.5];
fn the_yard(s: u32, order: &str) -> (Play, WallSpot, Vec<GuardId>) {
    let mut p = Play::mission("pass.json", "?levelFile=pass.json&roomclear&squad4", &Opts { seed: s, ..Default::default() });
    p.g.god_mode = true; p.g.tune.chaos = 0.5;
    for (i, m) in p.hostiles().into_iter().enumerate() { p.man_stands(m, [400.0 + 2.0 * i as f64, 400.0]); }
    let w = wall_at(&mut p.g, CHARGE, Some([CHARGE[0], 0.0, CHARGE[2] + 1.0])).expect("a place there");
    let spot = wall_charge_set_spot(&mut p.g, &w).expect("a place to set it from");
    let men = p.mates();
    let bunch = |p: &mut Play| for (i, &m) in men.iter().enumerate() { let b = [[-0.9, 0.5], [0.9, 0.5], [-0.5, 1.4], [0.6, 1.5]][i % 4]; p.man_stands(m, [spot[0] + b[0], spot[2] + b[1]]); };
    p.sam_put([spot[0], spot[2]]);
    bunch(&mut p);
    p.run(0.3);
    let at = p.sam_at();
    p.order(order, at, game::orders::Who::All);
    bunch(&mut p);
    p.run(1.0);
    (p, w, men)
}
fn in_blast(p: &mut Play, w: &WallSpot, m: GuardId) -> bool { let q = p.at(m); wall_charge_blast_share(&mut p.g, w, [q[0], 0.0, q[1]], None, Some(false)) > 0.0 }

/// THE RULE (the maintainer, 10-16: "after i place a wall charge, my guys should move away with standoff"): with a charge of Sam's on the wall the men on fall in step out of its blast, each on a place of his own, and the row names nobody but Sam; Sam walks clear and they come with him.
/// A FAILURE MEANS: his own charge kills the men who are following him.
#[test]
fn the_men_with_sam_step_back_from_his_charge() {
    let s = 1u32;
    let (mut p, w, men) = the_yard(s, "fallIn");
    let bunched = men.iter().filter(|&&m| in_blast(&mut p, &w, m)).count();
    assert!(bunched >= 2, "the staging: only {bunched} men in the blast");
    let c = p.sam_sets_charge(CHARGE, [CHARGE[0], CHARGE[2] + 1.0]).expect("Sam sets the charge");
    p.until(6.0, |p| wall_charge_too_close(&mut p.g, Some(c)) == ["you"]).unwrap_or_else(|| { let w = wall_charge_too_close(&mut p.g, Some(c)); panic!("6 s on, too close: {w:?}") });
    for (i, &a) in men.iter().enumerate() { for &b in &men[i + 1..] { assert!(dist(p.at(a), p.at(b)) > 0.5, "two men on one place: {:?} {:?}", p.at(a), p.at(b)); } }
    p.sam_put(CLEAR);
    p.run(6.0);
    for &m in &men { assert!(dist(p.at(m), CLEAR) < 8.0, "Sam walked clear and a man stayed on his mark: {:?}", p.at(m)); }
}

/// THE CONTROL: men told to hold where they stand stay in it, and the row names them.
#[test]
fn men_told_to_hold_stay_in_the_blast_and_the_row_says_so() {
    let (mut p, _, _) = the_yard(1, "hold");
    let c = p.sam_sets_charge(CHARGE, [CHARGE[0], CHARGE[2] + 1.0]).expect("Sam sets the charge");
    p.run(3.0);
    assert!(wall_charge_too_close(&mut p.g, Some(c)).len() >= 3, "men told to hold stepped back");
}

/// THE TEAM KNOWS OF A MAN: he was last seen `age` seconds ago, at this place.
fn know(p: &mut Play, list: &[([f64; 2], f64)]) {
    p.g.fog_of_war.on = true;
    let foes = p.hostiles();
    for &h in &foes { p.g.fog_of_war.recs.delete(h); }
    for (i, &(q, age)) in list.iter().enumerate() {
        let rec = game::fogofwar::Rec { t: p.g.time - age, at: Some([q[0], 0.0, q[1]]), ..game::fogofwar::Rec::default() };
        p.g.fog_of_war.recs.set(foes[i], rec);
    }
}

/// THE RULE (the maintainer, 10-14: "the two guys move away from the wall charge to a place that doesn't make sense (isn't safe since there are known enemies that can shoot them from there)"): a man the team knows of, square behind the charge: the floor square in front is under his fire, because the wall is about to go, and no mark is dealt in that lane. CONTROL: three metres along the wall nothing is under fire, nor is the same floor with the wall left standing. A sighting counts for so long and no longer: seen too long ago, he counts for nothing.
/// A FAILURE MEANS: the men stand off into the lane the charge is about to open.
#[test]
fn the_marks_keep_out_of_the_known_enemys_fire() {
    let s = 1u32;
    let (mut p, w, _) = the_yard(s, "fallIn");
    let spot = wall_charge_set_spot(&mut p.g, &w).expect("a place to set it from");
    let centre = wall_pt(&p.g, &w, 0.0, 0.0, None);
    let (front, aside) = (wall_pt(&p.g, &w, 0.0, 2.4, None), wall_pt(&p.g, &w, -3.0, 2.4, None));
    let known = p.g.k.wallcharge.fire_known_s;
    know(&mut p, &[([centre[0], 15.2], 5.0)]);
    assert_eq!(wall_charge_known_foes(&mut p.g).len(), 1);
    let (mut with, mut stands) = (wall_charge_under_fire(&mut p.g, Some(&w), None), wall_charge_under_fire(&mut p.g, None, None));
    assert!(wall_charge_fire_any(&mut p.g, &mut with.score, front), "the floor square in front of the charge is not under his fire");
    assert!(!wall_charge_fire_any(&mut p.g, &mut stands.score, front), "with the wall left standing he reaches that floor");
    assert!(!wall_charge_fire_any(&mut p.g, &mut with.score, aside), "three metres along the wall is under his fire");
    let marks = wall_charge_safe_posts(&mut p.g, &w, 3.0, spot, None, None).expect("marks");
    for q in &marks { assert!(!wall_charge_fire_any(&mut p.g, &mut with.score, *q), "a mark in his lane: {q:?}"); }
    know(&mut p, &[([centre[0], 15.2], known - 1.0)]);
    assert_eq!(wall_charge_known_foes(&mut p.g).len(), 1, "seen {} s ago, inside the {known} s, and forgotten", known - 1.0);
    know(&mut p, &[([centre[0], 15.2], known + 1.0)]);
    assert_eq!(wall_charge_known_foes(&mut p.g).len(), 0, "seen {} s ago, past the {known} s, and still counted", known + 1.0);
}
