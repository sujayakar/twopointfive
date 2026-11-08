//! WHERE MEN ARE SENT TO STAND: the charge's stand-off, and the marks dealt for a room.
mod doors_ask;
use doors_ask::*;
use game::game::GuardState;
use game::orders::Who;
use game_start::play::{dist, Opts, Play};

fn level_names() -> Vec<String> {
    let dir = game_start::files::root().join("levels");
    let mut n: Vec<String> = std::fs::read_dir(&dir).unwrap().map(|e| e.unwrap().file_name().to_str().unwrap().to_string()).filter(|n| n.ends_with(".json") && !n.ends_with(".lint.json")).collect();
    n.sort();
    n
}

/// THE RULE: the order deals the charge's marks by nearness: the man nearest the frame takes the first mark, whichever man he is, and the row and the walk ask the same deal. The control is the
/// same two men the other way round.
/// A FAILURE MEANS: the row tested a walk the order never makes. (Not asked: the old check also had one man's way to one mark refused, to see the row refuse it. The game has no such hand.)
#[test]
fn the_nearest_man_takes_the_first_mark_of_a_charge() {
    for s in SEEDS {
        for near in [0, 1] {
            let (sc, at) = yard(s);
            let mut p = Play::scene(&sc, &Opts { seed: s, squad: 2, ..Default::default() });
            p.sam_unhurt();
            p.run(0.2);
            let keep = p.defender_at([24.25, 12.25]);
            p.drop_all_but(&[keep]);
            p.pin_man(keep, [24.25, 12.25], 0.0, GuardState::Patrol, 0.0);
            let d = p.door("west-yard");
            let men = p.men();
            p.man_stands(men[near], [at[0] + 1.0, at[1] + 2.0]);
            p.man_stands(men[1 - near], [at[0] + 3.0, at[1] + 3.5]);
            let side = p.face(d, [at[0], at[1] + 1.0]);
            let marks = p.standoff_marks(d, side, &men);
            assert_eq!(marks.len(), 2, "seed {s}: no marks");
            assert_eq!(p.try_order("charge", at, Who::All), None, "seed {s}");
            p.until(3.0, |p| !p.ops_on(d).is_empty());
            let op = p.ops_on(d)[0];
            p.until(40.0, |p| p.op_phase(op) == game::doortools::Phase::Armed).unwrap_or_else(|| panic!("seed {s}: never armed: {:?}", p.op_beats(op)));
            p.run(4.0);
            let (nearest, other) = (p.at(men[near]), p.at(men[1 - near]));
            assert!(dist(nearest, marks[0]) < 0.3 && dist(other, marks[1]) < 0.3, "seed {s}, man {near}: he stands at {nearest:?}, the other at {other:?}, the marks are {marks:?}: {:?}", p.op_beats(op));
        }
    }
}

/// THE RULE: at a point in every room of every level, for two, three and four men, each dealt mark is off every frame and leaf, a body's room from the next, and no further from the point than
/// the spread, and nine points in ten have a deal. The control is the plain line abreast read the same way: a mate stands in far more of its lines.
/// A FAILURE MEANS: 'move there' sends a man into a doorway, into a mate, or to the far end of a room.
#[test]
fn every_room_deals_marks_a_man_can_stand_on() {
    let (mut pts, mut none, mut bad, mut lanes_fit, mut lanes_line) = (0, 0, Vec::new(), 0, 0);
    let in_line = |m: [f64; 2], tip: [f64; 2], o: [f64; 2]| { let (bx, bz) = (tip[0] - m[0], tip[1] - m[1]); let l2 = bx * bx + bz * bz; if l2 < 1e-6 { return false; } let k = ((o[0] - m[0]) * bx + (o[1] - m[1]) * bz) / l2; k > 0.08 && k < 1.0 && dist([m[0] + bx * k, m[1] + bz * k], o) < 0.5 };
    for name in level_names() {
        let mut p = Play::level(&name, false, &Opts { seed: 1, ..Default::default() });
        let k = p.g.k.roomfit;
        let from = p.sam_at();
        for (room, at) in p.room_points() {
            for n in [2.0, 3.0, 4.0] {
                pts += 1;
                let Some(f) = p.fit_marks(at, from, n) else { none += 1; continue };
                let spread = k.spread0 + k.spread_per * n;
                for (i, m) in f.iter().enumerate() {
                    if p.no_place_to_stand(m.0) { bad.push(format!("{name} room {room}, {n} men: {:?} is in a frame", m.0)); }
                    if dist(m.0, at) > spread + 0.01 { bad.push(format!("{name} room {room}, {n} men: {:.1} m off the point", dist(m.0, at))); }
                    for o in &f[i + 1..] { if dist(m.0, o.0) < k.apart - 0.01 { bad.push(format!("{name} room {room}, {n} men: {:.2} m apart", dist(m.0, o.0))); } }
                    let tip = m.1.unwrap_or([m.0[0] + m.2.sin() * 5.0, m.0[1] + m.2.cos() * 5.0]);
                    if f.iter().any(|o| o.0 != m.0 && in_line(m.0, tip, o.0)) { lanes_fit += 1; }
                }
                let l = p.line_marks(at, from, n);
                let face = yaw_to(from, at);
                for m in &l { let tip = [m[0] + face.sin() * 5.0, m[1] + face.cos() * 5.0]; if l.iter().any(|o| o != m && in_line(*m, tip, *o)) { lanes_line += 1; } }
            }
        }
    }
    assert!(bad.is_empty(), "{} marks out of {pts} deals: {:?}", bad.len(), &bad[..bad.len().min(12)]);
    assert!(none * 10 <= pts, "{none} of {pts} points had no deal");
    assert!(pts > 300, "only {pts} points");
    assert!(lanes_fit * 10 <= lanes_line, "the control: a mate in a man's line {lanes_fit} times, in a line abreast {lanes_line}");
}

/// THE RULE: four men sent to a point with 12 m and more of clear floor along a bearing: a gun is down that run in six cases of seven and more. The control is the same runs read against a deal
/// with every gun turned about: it leaves most of them bare.
/// A FAILURE MEANS: the men look at the near doors and nobody watches the length of the hall.
#[test]
fn a_long_clear_run_has_a_gun_down_it() {
    let (mut runs, mut bare, mut bare_about, mut eg) = (0, 0, 0, Vec::new());
    let on = |m: &([f64; 2], Option<[f64; 2]>, f64), b: f64, about: f64| m.1.map_or(false, |w| off_by(yaw_to(m.0, w) + about, b) < 40f64.to_radians());
    for name in level_names() {
        let mut p = Play::level(&name, false, &Opts { seed: 1, ..Default::default() });
        let from = p.sam_at();
        for (room, at) in p.room_points() {
            let Some(f) = p.fit_marks(at, from, 4.0) else { continue };
            let step = std::f64::consts::TAU / 24.0;
            let run: Vec<f64> = (0..24).map(|b| p.run_along(at, b as f64 * step, 14.0)).collect();
            for b in 0..24usize {
                if run[b] < 12.0 || run[(b + 1) % 24] > run[b] || run[(b + 23) % 24] >= run[b] { continue; }   // (a peak: one run, one count)
                runs += 1;
                let y = b as f64 * step;
                if !f.iter().any(|m| on(m, y, 0.0)) { bare += 1; if eg.len() < 6 { eg.push(format!("{name} room {room} bearing {}", b * 15)); } }
                if !f.iter().any(|m| on(m, y, std::f64::consts::PI)) { bare_about += 1; }
            }
        }
    }
    assert!(runs >= 20, "only {runs} long runs");
    assert!(bare * 100 <= runs * 15, "{bare} of {runs} long runs have no gun down them: {eg:?}");
    assert!(bare_about > bare, "the control: guns turned about leave {bare_about} of {runs} bare, against {bare}");
}
