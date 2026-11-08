//! A DOOR'S LEAF: which side of it a man works it from, and what is left of it after a blast. The maintainer: "open has to be on the handle side for AI".
mod doors_ask;
use doors_ask::*;
use game::ids::*;
use game::orders::Who;
use game_start::play::{dist, Opts, Play};

fn two(seed: u32) -> Opts { Opts { seed, squad: 2, ..Default::default() } }
fn stood(seed: u32) -> (Play, DoorId, [f64; 2]) {
    let (s, at) = yard(seed);
    let mut p = Play::scene(&s, &two(seed));
    p.sam_unhurt();
    p.run(0.2);
    let d = p.door("west-yard");
    (p, d, at)
}
fn hush(p: &mut Play) { let keep = p.defender_at([24.25, 12.25]); p.drop_all_but(&[keep]); p.pin_man(keep, [24.25, 12.25], 0.0, game::game::GuardState::Patrol, 0.0); }

/// THE RULE: the place the game deals a man at each face of each door is on that face, in the handle half, within reach of the handle (the long reach only
/// where it says so), and on the face the leaf opens into, clear of its sweep. Faces with only a long reach or with no place are the level's corners, at most a tenth of them.
/// A FAILURE MEANS: a door of a level has a man opening it from the hinge, or the leaf comes round through him. The test prints each.
#[test]
fn every_door_of_every_level_is_worked_from_its_handle() {
    let (mut faces, mut far, mut none, mut bad) = (0, 0, 0, Vec::new());
    let dir = game_start::files::root().join("levels");
    let mut names: Vec<String> = std::fs::read_dir(&dir).unwrap().map(|e| e.unwrap().file_name().to_str().unwrap().to_string()).filter(|n| n.ends_with(".json") && !n.ends_with(".lint.json")).collect();
    names.sort();
    for name in names {
        let mut p = Play::level(&name, false, &Opts { seed: 1, ..Default::default() });
        for d in p.doors() {
            let w = p.door_w(d);
            if p.door_gone(d) { continue; }
            for side in [1.0, -1.0] {
                faces += 1;
                let Some((at, says_pull, long)) = p.handle_place(d, side) else { none += 1; continue };
                let a = p.along(d, at);
                if long { far += 1; }
                let mut why = Vec::new();
                if p.off(d, at, 1.0) * side < 0.0 { why.push("on the other face".to_string()); }
                if a < w / 2.0 { why.push(format!("in the hinge half ({a:.2} of {w:.2})")); }
                let reach = p.from_handle(d, at);
                if reach > (if long { 1.5 } else { HANDLE_REACH }) + 0.05 { why.push(format!("{reach:.2} m from the handle")); }
                let pull = p.pull_face(d, at);
                if pull != says_pull { why.push("says the wrong face".to_string()); }
                if pull && p.in_sweep(d, at) { why.push("inside the sweep".to_string()); }
                if !why.is_empty() { bad.push(format!("{name} {} {side}: {}", p.door_name(d), why.join(", "))); }
            }
        }
    }
    assert!(bad.is_empty(), "{} of {faces} faces:\n{}", bad.len(), bad.join("\n"));
    assert!(faces > 100 && (far + none) * 10 <= faces, "{faces} faces: {far} with only a long reach, {none} with no place");
}

/// THE RULE: a man on the AI's own walk through a shut door, from either face, opens it at the handle: the handle half, within reach, out of the sweep on the
/// face the leaf opens toward, and he gets through. The control is what a man would do with no rule: he starts square on the middle of the leaf, the walk's straight line, which on the pull face is inside its sweep.
/// A FAILURE MEANS: the leaf comes round through its opener, or he opens it from the hinge.
#[test]
fn a_man_on_his_own_walk_opens_a_door_at_its_handle_from_both_faces() {
    for s in SEEDS {
        for pull in [true, false] {
            let (mut p, d, at) = stood(s);
            hush(&mut p);
            let out = [at[0], at[1] + 1.1];
            if pull { p.hung_toward(d, out); } else { p.hung_toward(d, [at[0], at[1] - 1.0]); }
            assert_eq!(p.pull_face(d, out), pull);
            if pull { assert!(p.in_sweep(d, out), "seed {s}: the control: square on the middle of the leaf a man on the pull face is in its sweep"); }
            let m = p.men()[0];
            p.order("move", out, Who::Num(0.0));
            p.until(10.0, |p| p.arrived(m)).unwrap_or_else(|| panic!("seed {s}: he never got to the door: {}", p.roll()));
            assert!(p.door_shut(d), "seed {s}: the staging: the leaf is shut");
            let to = [at[0], at[1] - 1.8];
            p.order("move", to, Who::Num(0.0));
            let mut stood_at = None;
            let through = p.until(12.0, |p| { if stood_at.is_none() && !p.latched(d) { stood_at = Some(p.at(m)); } dist(p.at(m), to) < 0.6 });
            let o = stood_at.unwrap_or_else(|| panic!("seed {s}, pull {pull}: the door never opened: {}", p.roll()));
            assert!(through.is_some(), "seed {s}, pull {pull}: not through in 12 s: at {:?}, leaf {:.2}", p.at(m), p.leaf_angle(d));
            let (a, w, reach) = (p.along(d, o), p.door_w(d), p.from_handle(d, o));
            assert!(a >= w / 2.0, "seed {s}, pull {pull}: he opened it {a:.2} m along a {w:.2} m leaf from its hinge");
            assert!(reach <= 1.5, "seed {s}, pull {pull}: he opened it {reach:.2} m from its handle");
            if pull { assert!(!p.in_sweep(d, o), "seed {s}: he stood in the sweep at {o:?}"); }
        }
    }
}

/// a frag on the yard's side of the door, `off` metres from the leaf, with a man pinned a stride behind it, run through the fuse: the man
fn burst(p: &mut Play, d: DoorId, off: f64, behind: f64) -> GuardId {
    let c = p.frame_at(d);
    let man = p.defender_at([11.25, 11.25]);
    let keep = p.defender_at([24.25, 12.25]);
    p.drop_all_but(&[man, keep]);
    let at = [c[0], c[1] - behind];
    p.frag_lands([c[0], c[1] + off]);
    let fuse = p.frag_fuse();
    for _ in 0..(fuse * 60.0 + 12.0) as usize { p.pin_man(man, at, 0.0, game::game::GuardState::Patrol, 0.0); p.pin_man(keep, [24.25, 12.25], 0.0, game::game::GuardState::Patrol, 0.0); p.frame(); }
    man
}

/// THE RULE: "40mm on door should maybe stun around the blast?". A burst that takes a door off stuns the man behind it and never kills him: the leaf took the fragments. The
/// control is the same burst too far off to take the leaf: it hangs, and he is untouched behind it.
/// A FAILURE MEANS: the blast judged him as shielded by a leaf it had just torn off, or a door stops nothing now.
#[test]
fn a_blast_takes_a_leaf_off_and_stuns_the_man_behind_it() {
    for s in SEEDS {
        let (mut p, d, _) = stood(s);
        let man = burst(&mut p, d, 0.6, 1.0);
        assert!(p.door_gone(d), "seed {s}: the leaf is off its frame");
        assert!(p.up(man), "seed {s}: stunned, never killed: {:?}", p.state(man));
        assert!(p.staggered(man), "seed {s}: he is doubled over");
        drop(p);

        let (mut p, d, _) = stood(s);
        let man = burst(&mut p, d, 4.5, 1.0);
        assert!(!p.door_gone(d) && p.latched(d), "seed {s}: the control: the leaf hangs");
        assert!(p.up(man) && !p.staggered(man), "seed {s}: the control: he is untouched behind it");
    }
}

/// THE RULE: a leaf blown off lands flat beyond the frame, on the far side from the blast; through the doorway a line is clear where a shut leaf stopped it; no verb moves it and Sam beside it is offered no row.
/// A FAILURE MEANS: the blown door still stands in the sight or the shot, or a leaf can be worked after it is gone.
#[test]
fn a_blown_leaf_lies_beyond_the_frame_and_the_doorway_reads_open() {
    for s in SEEDS {
        let (mut p, d, _) = stood(s);
        hush(&mut p);
        let c = p.frame_at(d);
        let (south, north) = ([c[0], c[1] + 1.5], [c[0], c[1] - 1.5]);
        assert!(!p.clear_line(south, north), "seed {s}: shut, the line through it is stopped");
        let torn = p.blast_tears([c[0], c[1] + 1.0]);
        assert_eq!(torn, vec![d], "seed {s}: the blast answers that door");
        p.until(4.0, |p| p.slab_rest(d, 1.0).is_some_and(|x| x.0)).unwrap_or_else(|| panic!("seed {s}: the slab never came to rest"));
        let side = p.face(d, south);
        let beyond = p.slab_rest(d, -side).unwrap().1;
        assert!(beyond > 0.5, "seed {s}: it went away from the blast: {beyond:.2} m past the frame on the far face");
        assert!(p.clear_line(south, north), "seed {s}: the doorway reads open");
        assert!(p.keep_broken(d) && !p.latched(d), "seed {s}: to the AI it is an open doorway");
        p.sam_put(south);
        p.run(0.4);
        assert_eq!(p.door_rows(d), 0, "seed {s}: Sam beside it is offered no door row");
    }
}

/// THE RULE: a man sent through a doorway whose leaf is off walks it as an open one: as soon as through the same door standing wide, and nothing about the door changes. The control is that door.
/// A FAILURE MEANS: the gone leaf still stops a walker, or a walker still moves it.
#[test]
fn a_doorway_with_its_leaf_off_walks_as_an_open_one() {
    for s in SEEDS {
        let walk = |off: bool| -> f64 {
            let (mut p, d, at) = stood(s);
            hush(&mut p);
            if off { p.blast_tears([at[0], at[1] + 1.0]); p.run(4.0); } else { p.leaf_put_open(d, 1.45); p.leaf_stays(d); p.run(4.0); }
            let m = p.men()[0];
            let (from, to) = ([at[0], at[1] + 2.2], [at[0], at[1] - 2.2]);
            p.order("move", from, Who::Num(0.0));
            p.until(10.0, |p| p.arrived(m)).unwrap_or_else(|| panic!("seed {s}, off {off}: not at the door: {}", p.roll()));
            p.order("move", to, Who::Num(0.0));
            let t = p.until(15.0, |p| dist(p.at(m), to) < 0.6).unwrap_or_else(|| panic!("seed {s}, off {off}: not through: at {:?}", p.at(m)));
            if off { assert!(p.door_gone(d) && p.leaf_vel(d) == 0.0, "seed {s}: nothing about the door changed"); }
            t
        };
        let (gone, hung) = (walk(true), walk(false));
        assert!(gone <= hung + 0.75, "seed {s}: {gone:.2} s off its hinges, {hung:.2} s hung open");
    }
}
