//! levels/tut-queue.script.ts translated: its define pass and a step function for each beat.
#![allow(unused_imports, unused_variables)]
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec, format};

const GUNNER: &str = "Tango 3";
const MATE: &str = "Tango 4";
const ROAD: [&str; 2] = ["Tango 5", "Tango 6"];
const LANE: [&str; 2] = ["Tango 7", "Tango 8"];
const LATE_SECS: f64 = 2.4;
const GATE_SECS: f64 = 4.0;
const ON_SECS: f64 = 9.0;
const BACK_SECS: f64 = 1.5;
const ALONE_BURSTS: f64 = 5.0;

// The c.until ids of the beat 'gun'.
const GUN_GONE: u32 = 0;
const GUN_HIT: u32 = 1;
const GUN_BACK: u32 = 2;
const GUN_FRONT: u32 = 3;
const GUN_TOO: u32 = 4;
// The c.until ids of the beat 'gates'.
const GATES_INSIDE: u32 = 0;
const GATES_ONE: u32 = 1;
const GATES_OPEN: u32 = 2;

fn mark(m: &mut crate::director::Manifest, id: &str, pos: V3) -> crate::director::MarkRef {
    crate::director::manifest_mark(m, id, Some(pos), None)
}

/// The define pass: its names in the script's order.
pub fn tut_queue_define(m: &mut crate::director::Manifest) -> Result<crate::script_tut_queue::TutQueueVars, String> {
    let front = crate::director::manifest_door(m, "hall");
    let back = crate::director::manifest_door(m, "hall_back");
    let road_gate = crate::director::manifest_door(m, "gate_e");
    let back_gate = crate::director::manifest_door(m, "gate_n");
    let track_gate = crate::director::manifest_door(m, "gate_w");
    let gun_mark = mark(m, "hall.gun", [31.2, 0.0, 15.2]);
    let mate_mark = mark(m, "hall.mate", [31.4, 0.0, 14.2]);
    let jamb = mark(m, "hall.jamb", [27.5, 0.0, 13.3]);
    let doorway_at: V3 = [26.0, 0.0, 13.5];
    let doorway = mark(m, "hall.doorway", doorway_at);
    let road_camp = vec![mark(m, "road.camp0", [38.4, 0.0, 8.2]), mark(m, "road.camp1", [38.6, 0.0, 11.8])];
    let lane_camp = vec![mark(m, "lane.camp0", [19.6, 0.0, 1.6]), mark(m, "lane.camp1", [24.4, 0.0, 1.6])];
    let road_in = vec![mark(m, "road.in0", [33.4, 0.0, 9.5]), mark(m, "road.in1", [34.0, 0.0, 10.6])];
    let lane_in = vec![mark(m, "lane.in0", [21.6, 0.0, 6.9]), mark(m, "lane.in1", [22.5, 0.0, 6.2])];
    let post_a = mark(m, "post.a", [29.25, 0.0, 10.25]);
    let post_b = mark(m, "post.b", [24.0, 0.0, 10.3]);
    let road_on = vec![mark(m, "road.on0", [31.2, 0.0, 7.6]), mark(m, "road.on1", [31.4, 0.0, 12.2])];
    let lane_on = vec![mark(m, "lane.on0", [22.2, 0.0, 8.0]), mark(m, "lane.on1", [24.9, 0.0, 8.0])];

    m.systems.set("theirFrags", false);
    m.systems.set("qrf", false);

    crate::director::manifest_program(m, "tutorial", &mut |p: &mut crate::director::ProgramDef| -> Result<(), String> {
        crate::director::program_on_start(p, "gun");
        crate::director::program_beat(p, "gun", &crate::scene_level::LevelScriptFn::TutQueue, None)?;
        crate::director::program_on_start(p, "gates");
        crate::director::program_beat(p, "gates", &crate::scene_level::LevelScriptFn::TutQueue, None)?;
        Ok(())
    }, None)?;

    Ok(crate::script_tut_queue::TutQueueVars {
        front: Some(front),
        back: Some(back),
        road_gate: Some(road_gate),
        back_gate: Some(back_gate),
        track_gate: Some(track_gate),
        gun_mark: Some(gun_mark),
        mate_mark: Some(mate_mark),
        jamb: Some(jamb),
        doorway_at,
        doorway: Some(doorway),
        road_camp,
        lane_camp,
        road_in,
        lane_in,
        post_a: Some(post_a),
        post_b: Some(post_b),
        road_on,
        lane_on,
    })
}

/// A mark as a place an order names.
fn place(x: &crate::director::MarkRef) -> crate::director::Place {
    crate::director::Place::Mark(x.clone())
}

/// `!!u && c.query.alive(u)`
fn alive(g: &Game, r: DirectorRunId, u: &Option<crate::director::UnitRef>) -> bool {
    match u {
        Some(u) => crate::director::director_query_alive(g, r, u),
        None => false,
    }
}

/// `s.open || s.broken` of a door
fn is_open(g: &mut Game, d: &crate::director::DoorRef) -> Result<bool, String> {
    let s = crate::director::director_query_door(g, d)?;
    Ok(s.open || s.broken)
}

/// The hall's men still standing, m before n
fn up(g: &mut Game, r: DirectorRunId, f: &crate::script_tut_queue::TutQueueGunFrame) -> Vec<crate::director::UnitRef> {
    let mut l = Vec::new();
    for u in [&f.m, &f.n] {
        if let Some(u) = u {
            if crate::director::director_query_alive(g, r, u) { l.push(u.clone()); }
        }
    }
    l
}

fn until_back() -> crate::director::Cond {
    crate::director::director_beat_until(crate::director::CondTest::Script { id: GUN_BACK }, Some("the back door opens"))
}
fn until_front() -> crate::director::Cond {
    crate::director::director_beat_until(crate::director::CondTest::Script { id: GUN_FRONT }, Some("the front door opens"))
}
fn too() -> crate::director::Cond {
    crate::director::director_beat_until(crate::director::CondTest::Script { id: GUN_TOO }, Some("the back door opens too"))
}

/// One resume of the beat 'gun': see the numbers in notes.md.
pub fn tut_queue_gun_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_queue::TutQueueGunFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    use crate::director::{OrderPatch2, RunBurstsItemAt};
    let v = crate::script_tut_queue::tut_queue_vars(g, r).clone();
    let back = v.back.as_ref().unwrap();
    let doorway = v.doorway.as_ref().unwrap();
    let jamb = v.jamb.as_ref().unwrap();
    let mate_mark = v.mate_mark.as_ref().unwrap();
    loop {
        match f.at {
            0 => {
                f.m = crate::director::director_beat_unit(g, r, GUNNER);
                f.n = crate::director::director_beat_unit(g, r, MATE);
                if up(g, r, f).is_empty() { return Ok(None); }
                if alive(g, r, &f.m) {
                    let m = f.m.clone().unwrap();
                    let at = crate::director::director_query_pos_unit(g, r, &m)?;
                    let o = OrderPatch2 {
                        goal: Some(Some(place(v.gun_mark.as_ref().unwrap()))),
                        aim_at: Some(Some(place(doorway))),
                        face: Some(Some(crate::director::util::yaw_to(at, v.doorway_at))),
                        upper: Some(Some(crate::character::UpperMode::Aim)),
                        ..Default::default()
                    };
                    crate::director::director_beat_order(g, r, b, &m, &o)?;
                }
                if alive(g, r, &f.n) {
                    let n = f.n.clone().unwrap();
                    let o = OrderPatch2 {
                        goal: Some(Some(place(mate_mark))),
                        aim_at: Some(Some(place(doorway))),
                        crouch: Some(Some(true)),
                        upper: Some(Some(crate::character::UpperMode::Relaxed)),
                        ..Default::default()
                    };
                    crate::director::director_beat_order(g, r, b, &n, &o)?;
                }
                f.gone = Some(crate::director::director_beat_until(crate::director::CondTest::Script { id: GUN_GONE }, Some("the hall is down")));
                f.hit = Some(crate::director::director_beat_until(crate::director::CondTest::Script { id: GUN_HIT }, Some("one of them is hit through a wall")));
                f.at = 1;
                return Ok(Some(crate::director::director_beat_any(vec![until_back(), until_front(), f.gone.clone().unwrap(), f.hit.clone().unwrap()])));
            }
            1 => {
                f.r = Some(got.clone());
                let index = got.index;
                let laid = crate::director::director_query_signalled(g, r, "lesson:breach");
                let which = ["the back door", "the front door", "both down", "a man hit"][index as usize];
                let what = format!("the hall's wait ends: {}{}", which, if laid { ", a stack on each door" } else { "" });
                crate::director::director_beat_note(g, b, &what, None);
                f.alone = index == 1.0 && !laid;
                if index == 1.0 && laid {
                    if alive(g, r, &f.m) {
                        let m = f.m.clone().unwrap();
                        let burst = crate::director::director_beat_burst(g, r, b, &m, RunBurstsItemAt::Mark(jamb.clone()), 4.0, Some(0.1), Some(0.02))?;
                        f.at = 2;
                        return Ok(Some(crate::director::director_beat_any(vec![burst, too(), f.gone.clone().unwrap()])));
                    }
                    f.at = 2;
                } else {
                    f.at = 4;
                }
            }
            2 => {
                if !is_open(g, back)? {
                    let wait = crate::director::director_beat_wait(g, r, BACK_SECS, None)?;
                    f.at = 3;
                    return Ok(Some(crate::director::director_beat_any(vec![too(), wait, f.gone.clone().unwrap()])));
                }
                f.at = 3;
            }
            3 => {
                f.alone = !is_open(g, back)?;
                f.at = 4;
            }
            4 => {
                if f.alone {
                    crate::director::director_beat_note(g, b, "one stack, alone: the gun is laid on its doorway", None);
                    if alive(g, r, &f.n) {
                        let n = f.n.clone().unwrap();
                        let o = OrderPatch2 {
                            goal: Some(Some(place(mate_mark))),
                            aim_at: Some(Some(place(doorway))),
                            crouch: Some(Some(false)),
                            upper: Some(Some(crate::character::UpperMode::Aim)),
                            ..Default::default()
                        };
                        crate::director::director_beat_order(g, r, b, &n, &o)?;
                    }
                    f.k = 0.0;
                    f.at = 5;
                } else {
                    f.at = 8;
                }
            }
            5 => {
                if f.k < ALONE_BURSTS && alive(g, r, &f.m) && !is_open(g, back)? {
                    let m = f.m.clone().unwrap();
                    let burst = crate::director::director_beat_burst(g, r, b, &m, RunBurstsItemAt::Mark(doorway.clone()), 6.0, Some(0.09), Some(0.03))?;
                    let wait = crate::director::director_beat_wait(g, r, 1.4, None)?;
                    f.at = 6;
                    return Ok(Some(crate::director::director_beat_any(vec![burst, wait, f.gone.clone().unwrap()])));
                }
                for u in up(g, r, f).iter() {
                    crate::director::director_beat_release(g, r, b, u, Some(crate::squad::EndGuardScriptState::Alert))?;
                }
                return Ok(None);
            }
            6 => {
                let wait = crate::director::director_beat_wait(g, r, 0.35, None)?;
                f.at = 7;
                return Ok(Some(crate::director::director_beat_any(vec![wait, f.gone.clone().unwrap()])));
            }
            7 => {
                f.k += 1.0;
                f.at = 5;
            }
            8 => {
                crate::director::director_beat_signal(g, r, b, "done:turned", None);
                if f.r.as_ref().unwrap().index < 2.0 {
                    if alive(g, r, &f.m) {
                        let m = f.m.clone().unwrap();
                        let burst = crate::director::director_beat_burst(g, r, b, &m, RunBurstsItemAt::Mark(jamb.clone()), 4.0, Some(0.1), Some(0.02))?;
                        let wait = crate::director::director_beat_wait(g, r, 1.0, None)?;
                        f.at = 9;
                        return Ok(Some(crate::director::director_beat_any(vec![burst, wait, f.gone.clone().unwrap()])));
                    }
                    f.at = 9;
                } else {
                    f.at = 10;
                }
            }
            9 => {
                let wait = crate::director::director_beat_wait(g, r, LATE_SECS, None)?;
                f.at = 10;
                return Ok(Some(crate::director::director_beat_any(vec![wait, f.gone.clone().unwrap()])));
            }
            10 => {
                if alive(g, r, &f.n) {
                    let n = f.n.clone().unwrap();
                    let o = OrderPatch2 {
                        goal: Some(Some(place(mate_mark))),
                        aim_at: Some(Some(place(doorway))),
                        crouch: Some(Some(false)),
                        upper: Some(Some(crate::character::UpperMode::Aim)),
                        ..Default::default()
                    };
                    crate::director::director_beat_order(g, r, b, &n, &o)?;
                    let wait = crate::director::director_beat_wait(g, r, 0.3, None)?;
                    f.at = 11;
                    return Ok(Some(crate::director::director_beat_any(vec![wait, f.gone.clone().unwrap()])));
                }
                f.at = 11;
            }
            11 => {
                for u in up(g, r, f).iter() {
                    crate::director::director_beat_release(g, r, b, u, Some(crate::squad::EndGuardScriptState::Alert))?;
                }
                return Ok(None);
            }
            at => return Err(format!("tut-queue gun: no resume point {}", at)),
        }
    }
}

/// The 'gun' beat's c.until tests.
pub fn tut_queue_gun_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_queue::TutQueueGunFrame, id: u32) -> Result<bool, String> {
    let (front, back) = {
        let v = crate::script_tut_queue::tut_queue_vars(g, r);
        (v.front.clone().unwrap(), v.back.clone().unwrap())
    };
    match id {
        GUN_GONE => Ok(up(g, r, f).is_empty()),
        GUN_HIT => Ok(up(g, r, f).len() < 2 && !is_open(g, &front)? && !is_open(g, &back)?),
        GUN_BACK | GUN_TOO => is_open(g, &back),
        GUN_FRONT => is_open(g, &front),
        _ => Err(format!("tut-queue gun: no c.until of id {}", id)),
    }
}

/// `men.filter(u => q.alive(u) && !q.routed(u))`
fn left(g: &mut Game, r: DirectorRunId, men: &[crate::director::UnitRef]) -> Vec<crate::director::UnitRef> {
    let mut l = Vec::new();
    for u in men {
        if crate::director::director_query_alive(g, r, u) && !crate::director::director_query_routed(g, r, u) { l.push(u.clone()); }
    }
    l
}

fn one() -> crate::director::Cond {
    crate::director::director_beat_until(crate::director::CondTest::Script { id: GATES_ONE }, Some("one of them is left"))
}

/// Are they all down or fled, or is the last one now running?
fn broke(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_queue::TutQueueGatesFrame) -> Result<bool, String> {
    let l = left(g, r, &f.men);
    if l.is_empty() { return Ok(true); }
    if l.len() > 1 || f.off { return Ok(false); }
    if crate::director::director_beat_rout(g, r, b, &l[0])? {
        let what = format!("{} is the last: he drops it and runs", l[0].callsign);
        crate::director::director_beat_note(g, b, &what, None);
        return Ok(true);
    }
    f.off = true;
    Ok(false)
}

/// Each pair walks in by its gate, the man nearer it taking the farther mark.
fn walk(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, us: &[Option<crate::director::UnitRef>], gate: [f64; 2], inn: &[crate::director::MarkRef], at: &crate::director::MarkRef) -> Result<(), String> {
    let mut chosen: Vec<(crate::director::UnitRef, f64)> = Vec::new();
    for u in us {
        if alive(g, r, u) {
            let u = u.clone().unwrap();
            let p = crate::director::director_query_pos_unit(g, r, &u)?;
            let d = m::hyp2(p[0] - gate[0], p[2] - gate[1]);
            chosen.push((u, d));
        }
    }
    js::sort_by(&mut chosen, |a, b| a.1 - b.1);
    for (i, (u, _)) in chosen.iter().enumerate() {
        let o = crate::director::OrderPatch2 {
            goal: Some(Some(place(&inn[i]))),
            aim_at: Some(Some(place(at))),
            upper: Some(Some(crate::character::UpperMode::Aim)),
            ..Default::default()
        };
        crate::director::director_beat_order(g, r, b, u, &o)?;
    }
    Ok(())
}

/// Men nobody has shot walk on into the open.
fn walk_on(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, men: &[crate::director::UnitRef], us: &[Option<crate::director::UnitRef>], to: &[crate::director::MarkRef], at: &crate::director::MarkRef) -> Result<(), String> {
    let mut chosen: Vec<crate::director::UnitRef> = Vec::new();
    for u in us.iter().flatten() {
        if left(g, r, men).iter().any(|x| x.id == u.id) { chosen.push(u.clone()); }
    }
    for (i, u) in chosen.iter().enumerate() {
        let o = crate::director::OrderPatch2 {
            goal: Some(Some(place(&to[i]))),
            aim_at: Some(Some(place(at))),
            upper: Some(Some(crate::character::UpperMode::Aim)),
            ..Default::default()
        };
        crate::director::director_beat_order(g, r, b, u, &o)?;
    }
    Ok(())
}

/// One resume of the beat 'gates': a resume point a yield.
pub fn tut_queue_gates_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_queue::TutQueueGatesFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    let v = crate::script_tut_queue::tut_queue_vars(g, r).clone();
    let road_gate = v.road_gate.as_ref().unwrap();
    let back_gate = v.back_gate.as_ref().unwrap();
    let track_gate = v.track_gate.as_ref().unwrap();
    let post_a = v.post_a.as_ref().unwrap();
    let post_b = v.post_b.as_ref().unwrap();
    match f.at {
        0 => {
            for d in [road_gate, back_gate, track_gate] { crate::director::director_beat_door_lock(g, d, true)?; }
            f.road = ROAD.iter().map(|cs| crate::director::director_beat_unit(g, r, cs)).collect();
            f.lane = LANE.iter().map(|cs| crate::director::director_beat_unit(g, r, cs)).collect();
            for i in 0..f.road.len() {
                if alive(g, r, &f.road[i]) {
                    let o = crate::director::OrderPatch2 {
                        goal: Some(Some(place(&v.road_camp[i]))),
                        upper: Some(Some(crate::character::UpperMode::Relaxed)),
                        ..Default::default()
                    };
                    crate::director::director_beat_order(g, r, b, f.road[i].as_ref().unwrap(), &o)?;
                }
            }
            for i in 0..f.lane.len() {
                if alive(g, r, &f.lane[i]) {
                    let o = crate::director::OrderPatch2 {
                        goal: Some(Some(place(&v.lane_camp[i]))),
                        upper: Some(Some(crate::character::UpperMode::Relaxed)),
                        ..Default::default()
                    };
                    crate::director::director_beat_order(g, r, b, f.lane[i].as_ref().unwrap(), &o)?;
                }
            }
            f.at = 1;
            Ok(Some(crate::director::director_beat_on(g, r, crate::director::CondArm3Kind::Signal, Some("lesson:gates"), None)))
        }
        1 => {
            let mut men = Vec::new();
            for i in 0..f.road.len() { if alive(g, r, &f.road[i]) { men.push(f.road[i].clone().unwrap()); } }
            for i in 0..f.lane.len() { if alive(g, r, &f.lane[i]) { men.push(f.lane[i].clone().unwrap()); } }
            f.men = men;
            f.off = false;
            for d in [road_gate, back_gate] {
                crate::director::director_beat_door_lock(g, d, false)?;
                crate::director::director_beat_door_open(g, d, None, Some(true))?;
            }
            f.at = 2;
            Ok(Some(crate::director::director_beat_wait(g, r, 0.8, None)?))
        }
        2 => {
            walk(g, r, b, &f.road, [35.0, 10.0], &v.road_in, post_a)?;
            walk(g, r, b, &f.lane, [22.0, 5.0], &v.lane_in, post_b)?;
            let inside = crate::director::director_beat_until(crate::director::CondTest::Script { id: GATES_INSIDE }, Some("they are inside their gates"));
            let wait = crate::director::director_beat_wait(g, r, 25.0, None)?;
            f.at = 3;
            Ok(Some(crate::director::director_beat_any(vec![inside, one(), wait])))
        }
        3 => {
            crate::director::director_beat_signal(g, r, b, "done:gates", None);
            if broke(g, r, b, f)? { return Ok(None); }
            let wait = crate::director::director_beat_wait(g, r, GATE_SECS, None)?;
            f.at = 4;
            Ok(Some(crate::director::director_beat_any(vec![one(), wait])))
        }
        4 => {
            if broke(g, r, b, f)? { return Ok(None); }
            for u in left(g, r, &f.men).iter() {
                crate::director::director_beat_release(g, r, b, u, Some(crate::squad::EndGuardScriptState::Alert))?;
            }
            let wait = crate::director::director_beat_wait(g, r, ON_SECS, None)?;
            f.at = 5;
            Ok(Some(crate::director::director_beat_any(vec![one(), wait])))
        }
        5 => {
            if broke(g, r, b, f)? { return Ok(None); }
            walk_on(g, r, b, &f.men, &f.road, &v.road_on, post_a)?;
            walk_on(g, r, b, &f.men, &f.lane, &v.lane_on, post_b)?;
            crate::director::director_beat_note(g, b, "nobody has them yet: they walk on into the yard", None);
            let open = crate::director::director_beat_until(crate::director::CondTest::Script { id: GATES_OPEN }, Some("they are in the open"));
            let wait = crate::director::director_beat_wait(g, r, 12.0, None)?;
            f.at = 6;
            Ok(Some(crate::director::director_beat_any(vec![open, one(), wait])))
        }
        6 => {
            if broke(g, r, b, f)? { return Ok(None); }
            for u in left(g, r, &f.men).iter() {
                crate::director::director_beat_release(g, r, b, u, Some(crate::squad::EndGuardScriptState::Alert))?;
            }
            f.at = 7;
            Ok(Some(one()))
        }
        7 => {
            broke(g, r, b, f)?;
            Ok(None)
        }
        at => Err(format!("tut-queue gates: no resume point {}", at)),
    }
}

/// The 'gates' beat's c.until tests.
pub fn tut_queue_gates_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_queue::TutQueueGatesFrame, id: u32) -> Result<bool, String> {
    match id {
        GATES_INSIDE | GATES_OPEN => {
            for u in left(g, r, &f.men).iter() {
                if !crate::director::director_query_arrived(g, r, u)? { return Ok(false); }
            }
            Ok(true)
        }
        GATES_ONE => Ok(left(g, r, &f.men).len() <= if f.off { 0 } else { 1 }),
        _ => Err(format!("tut-queue gates: no c.until of id {}", id)),
    }
}
