//! FIVE FAULTS FOUND IN HIS RECORDINGS: a man who changes his mind frame after frame; a man on the wrong side of a leaf; his own can; a mark in another door's frame; a mark at a window.
mod doors_ask;
use doors_ask::*;
use game::game::GuardState;
use game::ids::*;
use game::orders::Who;
use game_start::play::{dist, Opts, Play};

fn walled(s: u32, squad: u32) -> (Play, DoorId, [f64; 2]) {
    let (sc, at) = yard(s);
    let mut p = Play::scene(&sc, &Opts { seed: s, squad, ..Default::default() });
    p.sam_unhurt();
    p.run(0.2);
    let keep = p.defender_at([24.25, 12.25]);
    p.drop_all_but(&[keep]);
    p.pin_man(keep, [24.25, 12.25], 0.0, GuardState::Patrol, 0.0);
    let d = p.door("west-yard");
    (p, d, at)
}
/// Sam walks there by the keys, in a straight line
fn sam_goes(p: &mut Play, to: [f64; 2], mut each: impl FnMut(&mut Play)) {
    let mut held: Vec<&str> = Vec::new();
    for _ in 0..30 * 60 {
        let at = p.sam_at();
        if dist(at, to) < 0.25 { break; }
        let (dx, dz) = (to[0] - at[0], to[1] - at[1]);
        let mut want = Vec::new();
        if dx.abs() > 0.1 { want.push(if dx > 0.0 { "KeyD" } else { "KeyA" }); }
        if dz.abs() > 0.1 { want.push(if dz > 0.0 { "KeyS" } else { "KeyW" }); }
        for k in &held { if !want.contains(k) { p.key_up(k); } }
        for k in &want { if !held.contains(k) { p.key_down(k); } }
        held = want;
        p.point_at(to);
        p.frame();
        each(p);
    }
    for k in held { p.key_up(k); }
    assert!(dist(p.sam_at(), to) < 0.6, "Sam never got to {to:?}: he stands at {:?}", p.sam_at());
}

/// the office, the partner fallen in behind Sam, who walks into the break room by its north door and on down it, the men off the floor but one. Frames he moved away from his next step, back-to-back changes of
/// 'forward' and 'backwards', all changes.
fn walk_in(s: u32) -> (i32, i32, i32) {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear=10&squad=1", &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.run(0.3);
    let m = p.men()[0];
    let d = p.door("breakroom_n");
    let foes: Vec<(GuardId, [f64; 2])> = p.g.guards.clone().into_iter().filter(|&x| !p.g.team(x).ours()).map(|x| (x, p.at(x))).collect();
    let still = |p: &mut Play| for &(x, at) in &foes { p.pin_man(x, at, 0.0, GuardState::Patrol, 0.0); };
    p.leaf_stays(d);
    p.leaf_put_open(d, 1.35);
    let sam = p.sam_at();
    p.order("fallIn", sam, Who::All);
    p.run(0.8);
    let c = p.frame_at(d);
    let (mut away, mut flips, mut changes, mut was, mut last, mut n) = (0, 0, 0, None, -9, 0);
    p.sam_put([c[0], c[1] - 2.0]);
    for _ in 0..180 { still(&mut p); p.frame(); }
    for to in [[c[0], c[1] + 1.5], [34.3, c[1] + 1.5], [34.3, 17.0], [34.3, 21.6]] {
        sam_goes(&mut p, to, |p| {
            still(p);
            n += 1;
            if p.walks_away_from_his_path(m) { away += 1; }
            let back = p.walks_backwards(m);
            if let Some(w) = was { if w != back { changes += 1; if n - last == 1 { flips += 1; } last = n; } }
            was = Some(back);
        });
    }
    (away, flips, changes)
}

/// THE RULE: a man walking backwards on his opening does not change his mind every frame, and does not walk away from his next step. The control is that he does change, a few times, on the way. (The rule's own switch is gone from the game: it cannot be put off.)
/// A FAILURE MEANS: a follower shudders on his feet in a doorway.
#[test]
fn a_man_who_is_following_does_not_change_his_mind_frame_after_frame() {
    for s in SEEDS {
        let (away, flips, changes) = walk_in(s);
        assert!(changes > 0, "seed {s}: the control: he never walked backwards, so nothing was asked");
        assert!(flips <= 4 && away <= 20, "seed {s}: {flips} back-to-back changes of {changes}, {away} frames walking away from his next step");
    }
}

/// the leaf hung to open into the yard and stood open across the file's way, the men in the yard behind it
fn leaf_across_the_way(s: u32) -> (Play, DoorId, [f64; 2], Vec<GuardId>) {
    let (mut p, d, at) = walled(s, 4);
    let out = [at[0], at[1] + 1.0];
    p.hung_toward(d, out);
    p.leaf_stays(d);
    p.sam_put([at[0] - 1.0, at[1] - 3.0]);
    p.run(0.1);
    let side = p.face(d, out);
    let men = p.men();
    p.order("hold", p.sam_at(), Who::All);
    for (i, &m) in men.iter().enumerate() { p.man_stands(m, p.on_face(d, -1.6 - 0.9 * i as f64, side * 0.8)); }
    p.run(0.1);
    p.leaf_put_open(d, 1.7);
    (p, d, at, men)
}

/// THE RULE: a teammate at a walk goes round a leaf standing open across his way: sent into the room it stands where it was left, the doors count a leaf gone round, and every man is in the room. The control
/// is a man stood still against the back of the leaf: he is not walked along it, and no leaf is counted.
/// A FAILURE MEANS: the file herds the door shut ahead of it, or a man holding is dragged along a leaf.
#[test]
fn a_file_goes_round_a_leaf_across_its_way() {
    for s in SEEDS {
        let (mut p, d, at, men) = leaf_across_the_way(s);
        let (was, r0, goal) = (p.leaf_angle(d).abs(), p.leaves_gone_round(), [at[0] - 1.0, at[1] - 3.0]);
        p.order_floor("move", goal, Who::All);
        let mut low = was;
        p.run_each(10.0, |p| low = low.min(p.leaf_angle(d).abs()));
        assert!(low > was - 0.1, "seed {s}: the leaf stood at {was:.2}, least {low:.2}");
        assert!(p.leaves_gone_round() > r0, "seed {s}: no leaf counted");
        assert!(men.iter().all(|&m| p.at(m)[1] < at[1] - 0.3), "seed {s}: not all in: {:?}", men.iter().map(|&m| p.at(m)).collect::<Vec<_>>());
        drop(p);

        let (mut p, d, at, men) = leaf_across_the_way(s);
        let side = p.face(d, [at[0], at[1] + 1.0]);
        let (a, h) = (p.leaf_tip(d), p.hinge(d));
        let back = [(a[0] + h[0]) / 2.0 - side * 0.0, (a[1] + h[1]) / 2.0];
        let dir = yaw_to(h, a);
        let spot = [back[0] - dir.cos() * 0.33 * side, back[1] + dir.sin() * 0.33 * side];
        p.man_stands(men[0], spot);
        assert_eq!(p.order_men("hold", spot, &men[..1]), None, "seed {s}");
        for (i, &o) in men.iter().enumerate().skip(1) { p.man_stands(o, [at[0] + 6.0 + i as f64, at[1] + 5.0]); }
        p.run(0.1);
        let (r0, mut trod, mut last) = (p.leaves_gone_round(), 0.0, p.at(men[0]));
        p.run_each(3.0, |p| { trod += dist(p.at(men[0]), last); last = p.at(men[0]); });
        assert!(p.leaves_gone_round() == r0 && trod < 0.2, "seed {s}: the control: his feet moved {trod:.2} m, leaves {}", p.leaves_gone_round() - r0);
    }
}

/// THE RULE: a bang is its own side's can to every teammate: a mate with a line to it 3.3 m off is untouched, one at 1.5 m is blind for a beat and does not fold; a defender and a quiet mission's staff at the same
/// reach take all of it. The control is the defender.
/// A FAILURE MEANS: a door tool's can folds the other pair, or spares a man of the wrong side.
#[test]
fn a_bang_is_its_own_sides_can_to_every_teammate() {
    for s in SEEDS {
        let (sc, at) = yard(s);
        let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
        p.sam_unhurt();
        p.run(0.2);
        let (foe, staff) = (p.defender_at([11.25, 11.25]), p.defender_at([24.25, 12.25]));
        p.is_staff(staff);
        let d = p.door("west-yard");
        let men = p.men();
        let (reds, mate, close) = (&men[0..2], men[2], men[3]);
        for &m in &men[2..] { p.man_stands(m, [at[0] + 6.0, at[1] + 6.0]); }
        assert_eq!(p.order_men("hold", p.sam_at(), &men[2..]), None, "seed {s}");
        assert_eq!(p.order_men("bangClear", at, reds), None, "seed {s}");
        p.until(3.0, |p| !p.ops_on(d).is_empty());
        let op = p.ops_on(d)[0];
        let mut placed = None;
        p.until(40.0, |p| {
            if let Some((can, _, _)) = p.can_out(op) { placed = Some(p.item_at(can)); }
            if let Some(a) = placed {
                p.man_stands(mate, [a[0] - 2.9, a[1] - 1.6]);
                p.man_stands(close, [a[0] + 1.5, a[1]]);
                p.pin_man(foe, [a[0] - 3.6, a[1]], 0.0, GuardState::Patrol, 0.0);
                p.pin_man(staff, [(a[0] + 2.7).min(17.5), a[1] + 0.2], 0.0, GuardState::Patrol, 0.0);
            } else {
                p.pin_man(foe, [11.25, 11.25], 0.0, GuardState::Patrol, 0.0);
                p.pin_man(staff, [24.25, 12.25], 0.0, GuardState::Patrol, 0.0);
            }
            p.op_beats(op).iter().any(|b| b.starts_with("bang"))
        }).unwrap_or_else(|| panic!("seed {s}: no bang: {:?}", p.op_beats(op)));
        p.frame();
        let k = p.g.k.flash;
        assert!(p.blind_secs(foe) >= k.secs[1] - 0.1 && p.fold_secs(foe) >= k.stagger[1] - 0.1, "seed {s}: the control: the defender blind {:.1} folded {:.1}", p.blind_secs(foe), p.fold_secs(foe));
        assert!(p.blind_secs(staff) >= k.secs[1] - 0.1 && p.fold_secs(staff) > 0.0, "seed {s}: the staff man is spared: {:.1}", p.blind_secs(staff));
        assert_eq!((p.blind_secs(mate), p.fold_secs(mate)), (0.0, 0.0), "seed {s}: a mate 3.3 m off");
        assert!(p.blind_secs(close) > 0.0 && p.blind_secs(close) <= k.own_secs + 0.01 && p.fold_secs(close) == 0.0, "seed {s}: a mate 1.5 m off blind {:.1} folded {:.1}", p.blind_secs(close), p.fold_secs(close));
    }
}

/// THE RULE: every door of every level, both faces, 2 and 4 men: no point an entry is dealt, nor the shallow points of a threshold, stands in another door's opening, nor beyond one from the door's mouth. The
/// control is the measure asked of a point put in a door's own opening: it is read.
/// A FAILURE MEANS: an entry sends a man to stand in the next door's frame, as the barracks' gallery did.
#[test]
fn no_point_of_an_entry_is_in_another_doors_frame() {
    let dir = game_start::files::root().join("levels");
    let mut names: Vec<String> = std::fs::read_dir(&dir).unwrap().map(|e| e.unwrap().file_name().to_str().unwrap().to_string()).filter(|n| n.ends_with(".json") && !n.ends_with(".lint.json")).collect();
    names.sort();
    let (mut bad, mut asked, mut read) = (Vec::new(), 0, 0);
    for name in names {
        let mut p = Play::level(&name, false, &Opts { seed: 1, ..Default::default() });
        let doors = p.doors();
        let fr = |p: &Play, o: DoorId, q: [f64; 2]| { let u = p.on_face(o, 1.0, 0.0); let h = p.hinge(o); let (ux, uz) = (u[0] - h[0], u[1] - h[1]); let (rx, rz) = (q[0] - h[0], q[1] - h[1]); (rx * ux + rz * uz, -rx * uz + rz * ux) };
        for &d in &doors {
            if read < 1 && doors.len() > 1 { let o = doors[0]; let w = p.door_w(o); let c = p.on_face(o, w / 2.0, 0.1); let (a, b) = fr(&p, o, c); assert!(a > 0.0 && a < w && b.abs() < 0.45, "the control: the measure does not read a door's own opening: {a:.2} {b:.2}"); read += 1; }
            for side in [1.0, -1.0] {
                for n in [2.0, 4.0] {
                    let w = p.door_w(d);
                    let mouth = p.on_face(d, w / 2.0, -side * 0.9);
                    let pts: Vec<[f64; 2]> = p.entry_pods(d, side, n).into_iter().chain(p.shallow_pods(d, side, n)).collect();
                    asked += 1;
                    for q in pts {
                        for &o in &doors {
                            if o == d { continue; }
                            let (a, b) = fr(&p, o, q);
                            let wo = p.door_w(o);
                            if a > 0.0 && a < wo && b.abs() < 0.45 { bad.push(format!("{name} {}: {q:?} in {}'s frame", p.door_name(d), p.door_name(o))); continue; }
                            let m = fr(&p, o, mouth).1;
                            if m * b >= 0.0 { continue; }
                            let t = m / (m - b);
                            let x = fr(&p, o, [mouth[0] + (q[0] - mouth[0]) * t, mouth[1] + (q[1] - mouth[1]) * t]).0;
                            if x > 0.0 && x < wo { bad.push(format!("{name} {}: {q:?} beyond {}", p.door_name(d), p.door_name(o))); }
                        }
                    }
                }
            }
        }
    }
    assert!(asked > 200, "only {asked} deals asked");
    assert!(bad.is_empty(), "{} of {asked} deals: {:?}", bad.len(), &bad[..bad.len().min(12)]);
}

/// the records office with its pair, on the outside of its front door. The door, its outside, the pair.
fn the_front(s: u32) -> (Play, DoorId, f64, Vec<GuardId>) {
    let mut p = Play::mission("records.json", "?levelFile=records.json&roomclear=1&squad=2", &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.run(0.3);
    let (d, men) = (p.door("front"), p.men());
    let mid = men.iter().fold([0.0, 0.0], |a, &m| { let q = p.at(m); [a[0] + q[0] / 2.0, a[1] + q[1] / 2.0] });
    let side = p.face(d, mid);
    (p, d, side, men)
}

/// THE RULE: nobody stacks at the window beside a door. The records office's front door has a pane beside its latch jamb and a rifle inside reaches both jambs through it: the marks of 'stack up' and of the door
/// tools, for the pair and for one man, stand off the pane and on one hand of the door. The control is the walls' own marks, as they were before the rule: a mark at the window or a jamb each.
/// A FAILURE MEANS: "4 of 48 first entries lost a man there": shot through the pane on his mark before the door is touched.
#[test]
fn nobody_stacks_at_the_window_beside_a_door() {
    for s in SEEDS {
        let (mut p, d, side, men) = the_front(s);
        let panes = p.panes_by(d);
        assert!(!panes.is_empty(), "the level changed under the test: no pane beside the door");
        let w = p.door_w(d);
        let at_window = |p: &Play, q: [f64; 2]| p.off(d, q, side) < 1.5 && panes.iter().any(|x| p.along(d, q) > x.0 - 0.45 && p.along(d, q) < x.1 + 0.45);
        let one_hand = |p: &Play, m: &[[f64; 2]]| m.iter().all(|&q| (p.along(d, q) > w / 2.0) == (p.along(d, m[0]) > w / 2.0));
        for (name, marks, want) in [("stack up, the pair", p.stack_marks(d, side, &men), 2), ("the door tools, the pair", p.tool_marks(d, side, &men), 2), ("stack up, one man", p.stack_marks(d, side, &men[..1]), 1)] {
            assert_eq!(marks.len(), want, "{name}");
            assert!(marks.iter().all(|&q| !at_window(&p, q)), "seed {s}: {name}: a mark at the window: {marks:?} (panes {panes:?})");
            assert!(one_hand(&p, &marks), "seed {s}: {name}: a jamb each: the man off the lead's hand walks across the door front: {marks:?}");
        }
        let walls = p.walls_own_marks(d, side, &men);
        assert!(walls.iter().any(|&q| at_window(&p, q)) || !one_hand(&p, &walls), "seed {s}: the control: the walls' own marks are no fault: {walls:?}");
        let lines = |p: &mut Play, m: &[[f64; 2]]| m.iter().map(|&q| p.glass_lines(d, side, q)).sum::<f64>();
        let (ours, own) = (p.stack_marks(d, side, &men), lines(&mut p, &walls));
        assert!(lines(&mut p, &ours) <= own, "seed {s}: the marks are in more lines through glass than the walls' own");
    }
}
