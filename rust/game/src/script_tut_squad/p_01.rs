//! levels/tut-squad.script.ts translated: its define pass and a step function for each beat.
#![allow(unused_imports, unused_variables)]
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec, format};

const STORE_MEN: [&str; 2] = ["Tango 2", "Tango 3"];
const SHOP_MAN: &str = "Tango 4";
const GUNNER: &str = "Tango 5";
const LOADER: &str = "Tango 6";
const PICKET: [&str; 3] = ["Tango 9", "Tango 10", "Tango 11"];
const HALL_MEN: [&str; 2] = ["Tango 7", "Tango 8"];
const STREET_SECS: f64 = 4.0;
const COVER_SECS: f64 = 40.0;
const OPEN_SECS: f64 = 6.0;
const STORE_SECS: f64 = 2.5;
const SEEN_SECS: f64 = 1.5;
const SIDE_SECS: f64 = 2.0;
const SHOP_OPEN: f64 = 1.55;
const DOWN_SECS: f64 = 5.0;
const FRAG_SECS: f64 = 3.5;
const SHOCK_SECS: f64 = 40.0;
/// what `ends` writes in the log, by how the workshop man's hold ended (tut-squad.script.ts:171)
static SHOP_ENDS: [&str; 5] = ["", "the side door", "one of them in his room", "the player in his line", "he is down"];

/// c.until with an id of this beat's test, and its label
fn until(id: u32, label: &str) -> crate::director::Cond {
    crate::director::director_beat_until(crate::director::CondTest::Script { id }, Some(label))
}
/// c.wait
fn wait(g: &mut Game, r: DirectorRunId, secs: f64) -> Result<crate::director::Cond, String> {
    crate::director::director_beat_wait(g, r, secs, None)
}
/// c.signal, no data
fn signal(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, name: &str) {
    crate::director::director_beat_signal(g, r, b, name, None)
}
/// c.release(u, 'alert')
fn release_alert(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &crate::director::UnitRef) -> Result<(), String> {
    crate::director::director_beat_release(g, r, b, u, Some(crate::squad::EndGuardScriptState::Alert))
}
/// the closure `alive` (tut-squad.script.ts:67)
fn alive(g: &Game, r: DirectorRunId, u: &Option<crate::director::UnitRef>) -> bool {
    match u {
        Some(u) => crate::director::director_query_alive(g, r, u),
        None => false,
    }
}
/// the test of the closure `open` (tut-squad.script.ts:68)
fn door_opens(g: &mut Game, d: &Option<crate::director::DoorRef>) -> Result<bool, String> {
    let s = crate::director::director_query_door(g, d.as_ref().unwrap())?;
    Ok(s.open || s.broken)
}
/// the closure `open`'s c.until
fn open_cond(id: u32) -> crate::director::Cond {
    until(id, "a door of his opens")
}
/// c.unit of each callsign, the living ones (tut-squad.script.ts:79, :144, :206)
fn living(g: &Game, r: DirectorRunId, names: &[&str]) -> Vec<crate::director::UnitRef> {
    let mut men = Vec::new();
    for cs in names {
        let u = crate::director::director_beat_unit(g, r, cs);
        if alive(g, r, &u) { men.extend(u); }
    }
    men
}
/// the men of a list still alive
fn up_of(g: &Game, r: DirectorRunId, men: &[crate::director::UnitRef]) -> Vec<crate::director::UnitRef> {
    let mut out = Vec::new();
    for u in men {
        if crate::director::director_query_alive(g, r, u) { out.push(u.clone()); }
    }
    out
}
/// the men of a list alive and not runner.ts's (lastMan's `left` :35, the gun's `left` :256, the hall's `mine` :207)
fn left_of(g: &Game, r: DirectorRunId, men: &[crate::director::UnitRef]) -> Vec<crate::director::UnitRef> {
    let mut out = Vec::new();
    for u in men {
        if crate::director::director_query_alive(g, r, u) && !crate::director::director_query_routed(g, r, u) { out.push(u.clone()); }
    }
    out
}

/// the define pass (levels/tut-squad.script.ts script)
pub fn tut_squad_define(m: &mut crate::director::Manifest) -> Result<crate::script_tut_squad::TutSquadVars, String> {
    let store_door = crate::director::manifest_door(m, "store");
    let shop_door = crate::director::manifest_door(m, "workshop");
    let side_door = crate::director::manifest_door(m, "workshop_side");
    let hall_door = crate::director::manifest_door(m, "hall");
    let road_gate = crate::director::manifest_door(m, "gate_e");
    let back_gate = crate::director::manifest_door(m, "gate_n");
    let track_gate = crate::director::manifest_door(m, "gate_w");
    let shop_fire = crate::director::manifest_mark(m, "shop.fire", Some([19.3, 0.0, 8.52]), None);
    let shop_aim: V3 = [13.0, 1.0, 8.9];
    let in_store: V3 = [10.75, 1.1, 17.0];
    let in_hall: V3 = [31.4, 1.1, 19.75];
    let hall_window: V3 = [31.775, 1.4, 17.0];
    let shop_lane = crate::director::manifest_volume(m, "shop_lane");
    let shop_line = crate::director::manifest_volume(m, "shop_line");
    let shop_in = crate::director::manifest_volume(m, "shop_in");
    let hall_post = vec![crate::director::manifest_mark(m, "hall.post0", Some([31.75, 0.0, 17.9]), None), crate::director::manifest_mark(m, "hall.post1", Some([31.75, 0.0, 21.4]), None)];
    let hall_back = vec![crate::director::manifest_mark(m, "hall.back0", Some([29.4, 0.0, 17.4]), None), crate::director::manifest_mark(m, "hall.back1", Some([28.3, 0.0, 17.7]), None)];
    let lane = crate::director::manifest_mark(m, "nest.lane", Some([23.7, 0.0, 13.0]), None);
    let camp = vec![crate::director::manifest_mark(m, "picket.camp0", Some([38.4, 0.0, 11.6]), None), crate::director::manifest_mark(m, "picket.camp1", Some([38.8, 0.0, 9.8]), None), crate::director::manifest_mark(m, "picket.camp2", Some([38.4, 0.0, 8.0]), None)];
    let gate_post = crate::director::manifest_mark(m, "picket.gate", Some([35.9, 0.0, 14.0]), None);
    let bags = crate::director::manifest_mark(m, "picket.bags", Some([31.0, 0.0, 13.8]), None);
    let round = vec![crate::director::manifest_mark(m, "picket.round0", Some([26.6, 0.0, 4.2]), None), crate::director::manifest_mark(m, "picket.round1", Some([28.0, 0.0, 4.2]), None)];
    let open = crate::director::manifest_mark(m, "picket.open0", Some([34.0, 0.0, 14.0]), None);
    let out = vec![crate::director::manifest_mark(m, "picket.out0", Some([27.8, 0.0, 12.3]), None), crate::director::manifest_mark(m, "picket.out1", Some([27.0, 0.0, 13.0]), None)];

    m.systems.set("theirFrags", false);

    crate::director::manifest_program(m, "tutorial", &mut |p: &mut crate::director::ProgramDef| -> Result<(), String> {
        let script = crate::scene_level::LevelScriptFn::TutSquad;
        for beat in ["picket", "store", "shop", "hall", "gun"] {
            crate::director::program_on_start(p, beat);
            crate::director::program_beat(p, beat, &script, None)?;
        }
        Ok(())
    }, None)?;

    Ok(crate::script_tut_squad::TutSquadVars {
        store_door: Some(store_door),
        shop_door: Some(shop_door),
        side_door: Some(side_door),
        hall_door: Some(hall_door),
        road_gate: Some(road_gate),
        back_gate: Some(back_gate),
        track_gate: Some(track_gate),
        shop_fire: Some(shop_fire),
        shop_aim,
        in_store,
        in_hall,
        hall_window,
        shop_lane: Some(shop_lane),
        shop_line: Some(shop_line),
        shop_in: Some(shop_in),
        hall_post,
        hall_back,
        lane: Some(lane),
        camp,
        gate_post: Some(gate_post),
        bags: Some(bags),
        round,
        open: Some(open),
        out,
    })
}

// ---- the beat 'picket'
const PICKET_ONE: u32 = 1;
const PICKET_THERE_FRONT: u32 = 3;
const PICKET_THERE_LANE: u32 = 4;
const PICKET_COVERED: u32 = 5;
const PICKET_THERE_UP: u32 = 8; // there(up()) at :128: up() is taken as it is yielded, so which men it held is in the id: this + a bit per man of `men`

/// `men` (:79), exactly: its first is `front`, the rest are `lane` in another order, and it runs in PICKET's order
fn picket_men(f: &crate::script_tut_squad::TutSquadPicketFrame) -> Vec<crate::director::UnitRef> {
    let mut men = Vec::new();
    for cs in PICKET {
        if let Some(u) = f.front.iter().chain(f.lane.iter()).find(|u| u.callsign == cs) { men.push(u.clone()); }
    }
    men
}
/// lastMan's `one` (:38)
fn picket_one() -> crate::director::Cond {
    until(PICKET_ONE, "one of the picket is left")
}
/// lastMan's `broke` (:39-43)
fn picket_broke(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, men: &[crate::director::UnitRef], off: &mut bool) -> Result<bool, String> {
    let l = left_of(g, r, men);
    if l.is_empty() { return Ok(true); }
    if l.len() > 1 || *off { return Ok(false); }
    if crate::director::director_beat_rout(g, r, b, &l[0])? {
        crate::director::director_beat_note(g, b, &format!("{} is the last of the picket: he drops it and runs", l[0].callsign), None);
        return Ok(true);
    }
    *off = true;
    Ok(false)
}
/// the closure `out` (:84). `===` of two men is by id: all of them are the one object of `men` in the TypeScript
fn picket_out(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_squad::TutSquadPicketFrame, v: &crate::script_tut_squad::TutSquadVars, u: &crate::director::UnitRef) -> Result<(), String> {
    let goal = if f.front.as_ref().map_or(false, |x| x.id == u.id) {
        v.open.clone().unwrap()
    } else {
        v.out[f.lane.iter().position(|x| x.id == u.id).unwrap()].clone()
    };
    crate::director::director_beat_order(g, r, b, u, &crate::director::OrderPatch2 {
        goal: Some(Some(crate::director::Place::Mark(goal))),
        aim_at: Some(Some(crate::director::Place::Mark(v.bags.clone().unwrap()))),
        upper: Some(Some(crate::character::UpperMode::Aim)),
        direct: Some(Some(true)),
        ..Default::default()
    })
}
/// the test of the closure `there` (:85)
fn picket_there(g: &Game, r: DirectorRunId, us: &[crate::director::UnitRef]) -> Result<bool, String> {
    for u in us {
        if crate::director::director_query_alive(g, r, u) && !crate::director::director_query_routed(g, r, u) && !crate::director::director_query_arrived(g, r, u)? { return Ok(false); }
    }
    Ok(true)
}

/// (levels/tut-squad.script.ts:75-134 P.beat 'picket') one resume. lastMan's `off` is the frame's
pub fn tut_squad_picket_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_squad::TutSquadPicketFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    let v = tut_squad_vars(g, r).clone();
    loop {
        match f.at {
            0 => {
                for d in [&v.road_gate, &v.back_gate, &v.track_gate] { crate::director::director_beat_door_lock(g, d.as_ref().unwrap(), true)?; }
                for (i, cs) in PICKET.iter().enumerate() {
                    let u = crate::director::director_beat_unit(g, r, cs);
                    if alive(g, r, &u) {
                        crate::director::director_beat_order(g, r, b, u.as_ref().unwrap(), &crate::director::OrderPatch2 {
                            goal: Some(Some(crate::director::Place::Mark(v.camp[i].clone()))),
                            upper: Some(Some(crate::character::UpperMode::Relaxed)),
                            ..Default::default()
                        })?;
                    }
                }
                f.at = 1;
                return Ok(Some(crate::director::director_beat_on(g, r, crate::director::CondArm3Kind::Signal, Some("lesson:picket"), None)));
            }
            1 => {
                let men = living(g, r, &PICKET);
                f.front = men.first().cloned();
                let mut lane: Vec<crate::director::UnitRef> = men.iter().skip(1).cloned().collect();
                for d in [&v.road_gate, &v.back_gate] { crate::director::director_beat_door_lock(g, d.as_ref().unwrap(), false)?; }
                crate::director::director_beat_door_open(g, v.back_gate.as_ref().unwrap(), None, Some(true))?;
                if lane.len() > 1 {
                    let mut keyed = Vec::new();
                    for u in lane { keyed.push((crate::director::director_query_pos_unit(g, r, &u)?[2], u)); }
                    js::sort_by(&mut keyed, |a, c| a.0 - c.0);
                    lane = keyed.into_iter().map(|k| k.1).collect();
                }
                f.lane = lane.clone();
                for (i, u) in lane.iter().enumerate() {
                    crate::director::director_beat_order(g, r, b, u, &crate::director::OrderPatch2 {
                        goal: Some(Some(crate::director::Place::Mark(v.round[i].clone()))),
                        upper: Some(Some(crate::character::UpperMode::Aim)),
                        direct: Some(Some(true)),
                        ..Default::default()
                    })?;
                }
                let Some(front) = f.front.clone() else { f.at = 3; continue; };
                crate::director::director_beat_door_open(g, v.road_gate.as_ref().unwrap(), None, Some(true))?;
                crate::director::director_beat_order(g, r, b, &front, &crate::director::OrderPatch2 {
                    goal: Some(Some(crate::director::Place::Mark(v.gate_post.clone().unwrap()))),
                    aim_at: Some(Some(crate::director::Place::Mark(v.bags.clone().unwrap()))),
                    upper: Some(Some(crate::character::UpperMode::Aim)),
                    direct: Some(Some(true)),
                    ..Default::default()
                })?;
                f.at = 2;
                let x = until(PICKET_THERE_FRONT, "the front man is in the gate");
                return Ok(Some(crate::director::director_beat_any(vec![x, wait(g, r, 6.0)?])));
            }
            2 => {
                if picket_broke(g, r, b, &picket_men(f), &mut f.off)? { signal(g, r, b, "done:flank"); signal(g, r, b, "done:picket"); return Ok(None); }
                if alive(g, r, &f.front) { release_alert(g, r, b, f.front.as_ref().unwrap())?; }
                f.at = 3;
                return Ok(Some(wait(g, r, 1.2)?));
            }
            3 => {
                signal(g, r, b, "done:flank");
                f.at = 4;
                let x = until(PICKET_THERE_LANE, "the two are at the back gate");
                return Ok(Some(crate::director::director_beat_any(vec![x, picket_one(), wait(g, r, 30.0)?])));
            }
            4 => {
                if picket_broke(g, r, b, &picket_men(f), &mut f.off)? { signal(g, r, b, "done:picket"); return Ok(None); }
                f.at = 5;
                let x = until(PICKET_COVERED, "the squad is turned to the alley");
                return Ok(Some(crate::director::director_beat_any(vec![x, picket_one(), wait(g, r, COVER_SECS)?])));
            }
            5 => {
                if picket_broke(g, r, b, &picket_men(f), &mut f.off)? { signal(g, r, b, "done:picket"); return Ok(None); }
                for u in f.lane.iter() {
                    if crate::director::director_query_alive(g, r, u) { picket_out(g, r, b, f, &v, u)?; }
                }
                f.at = 6;
                let x = until(PICKET_THERE_LANE, "the two are in the street");
                return Ok(Some(crate::director::director_beat_any(vec![x, picket_one(), wait(g, r, 16.0)?])));
            }
            6 => {
                signal(g, r, b, "done:picket");
                if picket_broke(g, r, b, &picket_men(f), &mut f.off)? { return Ok(None); }
                f.at = 7;
                return Ok(Some(crate::director::director_beat_any(vec![picket_one(), wait(g, r, STREET_SECS)?])));
            }
            7 => {
                if picket_broke(g, r, b, &picket_men(f), &mut f.off)? { return Ok(None); }
                for u in f.lane.iter() {
                    if crate::director::director_query_alive(g, r, u) { release_alert(g, r, b, u)?; }
                }
                f.at = 8;
            }
            // the head of `for (;;)` (:124): no yield comes back here, all its state is `lane`, `front` and `off`
            8 => {
                f.at = 9;
                return Ok(Some(crate::director::director_beat_any(vec![picket_one(), wait(g, r, 12.0)?])));
            }
            9 => {
                let men = picket_men(f);
                if picket_broke(g, r, b, &men, &mut f.off)? { return Ok(None); }
                for u in left_of(g, r, &men).iter() { picket_out(g, r, b, f, &v, u)?; }
                let us = left_of(g, r, &men);
                let mut mask = 0u32;
                for (i, u) in men.iter().enumerate() {
                    if us.iter().any(|x| x.id == u.id) { mask |= 1 << i; }
                }
                f.at = 10;
                let x = until(PICKET_THERE_UP + mask, "they are in the open");
                return Ok(Some(crate::director::director_beat_any(vec![x, picket_one(), wait(g, r, 16.0)?])));
            }
            10 => {
                if picket_broke(g, r, b, &picket_men(f), &mut f.off)? { return Ok(None); }
                f.at = 11;
                return Ok(Some(crate::director::director_beat_any(vec![picket_one(), wait(g, r, OPEN_SECS)?])));
            }
            11 => {
                let men = picket_men(f);
                if picket_broke(g, r, b, &men, &mut f.off)? { return Ok(None); }
                for u in left_of(g, r, &men).iter() { release_alert(g, r, b, u)?; }
                f.at = 8;
            }
            at => return Err(format!("tut-squad picket: no resume point {}", at)),
        }
    }
}

/// (levels/tut-squad.script.ts:75-134) the c.untils of 'picket'
pub fn tut_squad_picket_test(g: &crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_squad::TutSquadPicketFrame, id: u32) -> Result<bool, String> {
    match id {
        PICKET_ONE => Ok(left_of(g, r, &picket_men(f)).len() <= if f.off { 0 } else { 1 }),
        PICKET_THERE_FRONT => { let us: Vec<crate::director::UnitRef> = f.front.iter().cloned().collect(); picket_there(g, r, &us) }
        PICKET_THERE_LANE => { let us = f.lane.clone(); picket_there(g, r, &us) }
        PICKET_COVERED => Ok(crate::director::director_query_signalled(g, r, "lesson:covered")),
        id if id >= PICKET_THERE_UP && id < PICKET_THERE_UP + 8 => {
            let mask = id - PICKET_THERE_UP;
            let us: Vec<crate::director::UnitRef> = picket_men(f).into_iter().enumerate().filter(|(i, _)| mask & (1 << *i) != 0).map(|(_, u)| u).collect();
            picket_there(g, r, &us)
        }
        id => Err(format!("tut-squad picket: no test {}", id)),
    }
}

// ---- the beat 'store'
const STORE_OPEN: u32 = 1;
const STORE_HURT: u32 = 2;
/// the closure `hurt` (:145)
fn store_hurt() -> crate::director::Cond {
    until(STORE_HURT, "one of the store is down, or the player is in it")
}

/// (levels/tut-squad.script.ts:143-149 P.beat 'store') one resume
pub fn tut_squad_store_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_squad::TutSquadStoreFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    let v = tut_squad_vars(g, r).clone();
    loop {
        match f.at {
            0 => {
                f.men = living(g, r, &STORE_MEN);
                for u in f.men.iter() {
                    crate::director::director_beat_order(g, r, b, u, &crate::director::OrderPatch2 {
                        goal: Some(None),
                        aim_at: Some(Some(crate::director::Place::List(v.in_store))),
                        upper: Some(Some(crate::character::UpperMode::Aim)),
                        ..Default::default()
                    })?;
                }
                f.at = 1;
                return Ok(Some(crate::director::director_beat_any(vec![open_cond(STORE_OPEN), store_hurt()])));
            }
            1 => {
                if got.index == 0.0 {
                    f.at = 2;
                    return Ok(Some(crate::director::director_beat_any(vec![wait(g, r, STORE_SECS)?, store_hurt()])));
                }
                f.at = 2;
            }
            2 => {
                for u in f.men.iter() {
                    if crate::director::director_query_alive(g, r, u) { release_alert(g, r, b, u)?; }
                }
                return Ok(None);
            }
            at => return Err(format!("tut-squad store: no resume point {}", at)),
        }
    }
}

/// (levels/tut-squad.script.ts:143-149) the c.untils of 'store'
pub fn tut_squad_store_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_squad::TutSquadStoreFrame, id: u32) -> Result<bool, String> {
    match id {
        STORE_OPEN => { let d = tut_squad_vars(g, r).store_door.clone(); door_opens(g, &d) }
        STORE_HURT => {
            for u in f.men.iter() {
                if !crate::director::director_query_alive(g, r, u) { return Ok(true); }
            }
            Ok(crate::director::director_query_room_sam(g) == "store")
        }
        id => Err(format!("tut-squad store: no test {}", id)),
    }
}

// ---- the beat 'shop'
const SHOP_CUE: u32 = 1;
const SHOP_SIDE: u32 = 2;
const SHOP_ROOM: u32 = 3;
const SHOP_SEEN: u32 = 4;
const SHOP_DOWN: u32 = 5;
/// the closures `side`, `room`, `seen`, `down` as they stand in a c.any, in that order (:165-168)
fn shop_ways() -> [crate::director::Cond; 4] {
    [open_cond(SHOP_SIDE), until(SHOP_ROOM, "one of them is in his room"), until(SHOP_SEEN, "the player stays in his line"), until(SHOP_DOWN, "he is down")]
}
/// the closure `hold` (:168)
fn shop_hold(x: crate::director::Cond) -> crate::director::Cond {
    let mut of = vec![x];
    of.extend(shop_ways());
    crate::director::director_beat_any(of)
}
/// the closure `ends` (:169-173)
fn shop_ends(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_squad::TutSquadShopFrame) -> Result<(), String> {
    if !alive(g, r, &f.m) { return Ok(()); }
    let m = f.m.as_ref().unwrap();
    crate::director::director_beat_note(g, b, &format!("his hold ends: {}", SHOP_ENDS[f.end as usize]), None);
    crate::director::director_beat_show(g, r, b, m, Some(false))?;
    release_alert(g, r, b, m)
}

/// (levels/tut-squad.script.ts:161-195 P.beat 'shop') one resume
pub fn tut_squad_shop_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_squad::TutSquadShopFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    let v = tut_squad_vars(g, r).clone();
    loop {
        match f.at {
            0 => {
                f.m = crate::director::director_beat_unit(g, r, SHOP_MAN);
                if !alive(g, r, &f.m) { signal(g, r, b, "done:shop"); return Ok(None); }
                f.since = -1.0;
                let shop_door = v.shop_door.as_ref().unwrap();
                let swing = crate::director::director_query_door(g, shop_door)?.swing;
                crate::director::director_beat_door_open(g, shop_door, Some(js::or(swing, 1.0) * SHOP_OPEN), Some(true))?;
                crate::director::director_beat_order(g, r, b, f.m.as_ref().unwrap(), &crate::director::OrderPatch2 {
                    goal: Some(Some(crate::director::Place::Mark(v.shop_fire.clone().unwrap()))),
                    aim_at: Some(Some(crate::director::Place::List(v.shop_aim))),
                    upper: Some(Some(crate::character::UpperMode::Aim)),
                    ..Default::default()
                })?;
                f.at = 1;
                return Ok(Some(shop_hold(until(SHOP_CUE, "the cue 'shop'"))));
            }
            1 => {
                f.end = got.index;
                f.said = false;
                if !js::truthy(f.end) {
                    crate::director::director_beat_show(g, r, b, f.m.as_ref().unwrap(), None)?;
                    f.at = 2;
                    return Ok(Some(shop_hold(wait(g, r, 0.6)?)));
                }
                f.at = 3;
            }
            2 => {
                f.end = got.index;
                f.at = 3;
            }
            // the head of `while (!end)` (:179): `end`, `k`, `said` are the frame's
            3 => {
                if js::truthy(f.end) { f.at = 10; continue; }
                f.k = 0.0;
                f.at = 4;
            }
            // the head of `for k` (:180)
            4 => {
                if !(f.k < 3.0 && !js::truthy(f.end)) { f.at = 8; continue; }
                let lane = v.shop_lane.as_ref().unwrap().id.clone();
                if !crate::director::director_inside_volume(g, r, &lane, &crate::director::DirectorInsideVolumeWho::Team) {
                    let burst = crate::director::director_beat_burst(g, r, b, f.m.as_ref().unwrap(), crate::director::RunBurstsItemAt::List(v.shop_aim), 1.0, None, Some(0.03))?;
                    let half = wait(g, r, 0.5)?;
                    let mut of = vec![burst, half];
                    of.extend(shop_ways());
                    f.at = 5;
                    return Ok(Some(crate::director::director_beat_any(of)));
                }
                f.at = 6;
            }
            5 => {
                f.b = got.index;
                crate::director::director_beat_cease(g, r, f.m.as_ref().unwrap());
                if !js::truthy(f.b) && !f.said { f.said = true; signal(g, r, b, "done:shop"); }
                f.end = if f.b > 1.0 { f.b - 1.0 } else { 0.0 };
                f.at = 6;
            }
            6 => {
                if !js::truthy(f.end) {
                    f.at = 7;
                    return Ok(Some(shop_hold(wait(g, r, 0.13)?)));
                }
                f.k += 1.0;
                f.at = 4;
            }
            7 => {
                f.end = got.index;
                f.k += 1.0;
                f.at = 4;
            }
            8 => {
                if !js::truthy(f.end) {
                    let secs = 1.5 + crate::director::director_beat_rand(g, r) * 0.9;
                    f.at = 9;
                    return Ok(Some(shop_hold(wait(g, r, secs)?)));
                }
                f.at = 3;
            }
            9 => {
                f.end = got.index;
                f.at = 3;
            }
            10 => {
                if !f.said { signal(g, r, b, "done:shop"); }
                if f.end == 1.0 {
                    f.at = 11;
                    let x = wait(g, r, SIDE_SECS)?;
                    return Ok(Some(crate::director::director_beat_any(vec![x, until(SHOP_DOWN, "he is down")])));
                }
                f.at = 11;
            }
            11 => {
                shop_ends(g, r, b, f)?;
                return Ok(None);
            }
            at => return Err(format!("tut-squad shop: no resume point {}", at)),
        }
    }
}

/// (levels/tut-squad.script.ts:161-195) the c.untils of 'shop'. SHOP_SEEN writes `since`
pub fn tut_squad_shop_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_squad::TutSquadShopFrame, id: u32) -> Result<bool, String> {
    match id {
        SHOP_CUE => Ok(crate::director::director_query_signalled(g, r, "lesson:shop")),
        SHOP_SIDE => { let d = tut_squad_vars(g, r).side_door.clone(); door_opens(g, &d) }
        SHOP_ROOM => {
            let v = tut_squad_vars(g, r).shop_in.as_ref().unwrap().id.clone();
            Ok(crate::director::director_inside_volume(g, r, &v, &crate::director::DirectorInsideVolumeWho::Team))
        }
        SHOP_SEEN => {
            let v = tut_squad_vars(g, r).shop_line.as_ref().unwrap().id.clone();
            if !crate::director::director_inside_volume(g, r, &v, &crate::director::DirectorInsideVolumeWho::Sam) { f.since = -1.0; return Ok(false); }
            let t = g.tab.director_run[r].t;
            if f.since < 0.0 { f.since = t; }
            Ok(t - f.since >= SEEN_SECS)
        }
        SHOP_DOWN => Ok(!crate::director::director_query_alive(g, r, f.m.as_ref().unwrap())),
        id => Err(format!("tut-squad shop: no test {}", id)),
    }
}

// ---- the beat 'hall'
const HALL_CUED: u32 = 1;
const HALL_OPEN: u32 = 2;
const HALL_ROOM: u32 = 3;
const HALL_FEWER: u32 = 10;  // `ends(k)`'s third test (:208): k is taken as it is yielded, so it is in the id: this + k
/// the closure `ends` (:208)
fn hall_ends(k: usize) -> crate::director::Cond {
    crate::director::director_beat_any(vec![open_cond(HALL_OPEN), until(HALL_ROOM, "the player is in the hall"), until(HALL_FEWER + k as u32, "one of the hall is down")])
}
/// the closure `post` (:221)
fn hall_post(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &crate::director::UnitRef, at: &crate::director::MarkRef, aim_at: V3) -> Result<(), String> {
    crate::director::director_beat_order(g, r, b, u, &crate::director::OrderPatch2 {
        goal: Some(Some(crate::director::Place::Mark(at.clone()))),
        aim_at: Some(Some(crate::director::Place::List(aim_at))),
        upper: Some(Some(crate::character::UpperMode::Aim)),
        crouch: Some(Some(false)),
        direct: Some(Some(true)),
        ..Default::default()
    })
}
/// the closure `slot` (:222)
fn hall_slot(u: &crate::director::UnitRef) -> usize {
    HALL_MEN.iter().position(|n| *n == u.callsign).unwrap_or(0)
}
/// the closure `turn` (:225)
fn hall_turn(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_squad::TutSquadHallFrame, v: &crate::script_tut_squad::TutSquadVars) -> Result<(), String> {
    for u in left_of(g, r, &f.men).iter() { hall_post(g, r, b, u, &v.hall_post[hall_slot(u)], v.hall_window)?; }
    Ok(())
}

/// (levels/tut-squad.script.ts:205-235 P.beat 'hall') one resume
pub fn tut_squad_hall_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_squad::TutSquadHallFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    let v = tut_squad_vars(g, r).clone();
    loop {
        match f.at {
            0 => {
                f.men = living(g, r, &HALL_MEN);
                if !crate::director::director_query_signalled(g, r, "lesson:window") {
                    for u in f.men.iter() { hall_post(g, r, b, u, &v.hall_back[hall_slot(u)], v.in_hall)?; }
                    f.at = 1;
                    return Ok(Some(crate::director::director_beat_any(vec![until(HALL_CUED, "the cue 'window'"), hall_ends(f.men.len())])));
                }
                f.at = 1;
            }
            1 => {
                if up_of(g, r, &f.men).len() == f.men.len() {
                    for u in left_of(g, r, &f.men).iter() { hall_post(g, r, b, u, &v.hall_post[hall_slot(u)], v.in_hall)?; }
                } else {
                    hall_turn(g, r, b, f, &v)?;
                }
                f.at = 2;
            }
            // the head of the `while` (:233): the count is read afresh, nothing of it lives across the yield
            2 => {
                let n = up_of(g, r, &f.men).len();
                if n > 0 {
                    f.at = 3;
                    return Ok(Some(hall_ends(n)));
                }
                f.at = 4;
            }
            3 => {
                if got.index == 2.0 {
                    hall_turn(g, r, b, f, &v)?;
                    f.at = 2;
                } else {
                    f.at = 4;
                }
            }
            4 => {
                for u in left_of(g, r, &f.men).iter() { release_alert(g, r, b, u)?; }
                return Ok(None);
            }
            at => return Err(format!("tut-squad hall: no resume point {}", at)),
        }
    }
}

/// (levels/tut-squad.script.ts:205-235) the c.untils of 'hall'
pub fn tut_squad_hall_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_squad::TutSquadHallFrame, id: u32) -> Result<bool, String> {
    match id {
        HALL_CUED => Ok(crate::director::director_query_signalled(g, r, "lesson:window")),
        HALL_OPEN => { let d = tut_squad_vars(g, r).hall_door.clone(); door_opens(g, &d) }
        HALL_ROOM => Ok(crate::director::director_query_room_sam(g) == "hall"),
        id if id >= HALL_FEWER => Ok((up_of(g, r, &f.men).len() as u32) < id - HALL_FEWER),
        id => Err(format!("tut-squad hall: no test {}", id)),
    }
}

// ---- the beat 'gun'
const GUN_CUE_GUN: u32 = 1;
const GUN_CUE_DUCKS: u32 = 2;
const GUN_CUE_FRAGGED: u32 = 3;
const GUN_CUE_ATBLOCKS: u32 = 4;
const GUN_DOWN: u32 = 5;
const GUN_ROOM: u32 = 6;
const GUN_FEWER: u32 = 10;  // `fewer(k, …)` (:256): k is in the id: this + k
/// the closure `on` (:255), by its test's id
fn gun_on(id: u32, k: &str) -> crate::director::Cond {
    until(id, &format!("the cue '{}'", k))
}
/// the closure `gunDown` (:257)
fn gun_down_cond() -> crate::director::Cond {
    until(GUN_DOWN, "the gunner is down")
}
/// the closure `fewer` at :294, :295, :297
fn gun_nest_down() -> crate::director::Cond {
    until(GUN_FEWER + 1, "the nest is down")
}
/// the closure `cue` (:255)
fn gun_cue(g: &Game, r: DirectorRunId, k: &str) -> bool {
    crate::director::director_query_signalled(g, r, &format!("lesson:{}", k))
}
/// the closure `left` (:256)
fn gun_left(g: &Game, r: DirectorRunId, f: &crate::script_tut_squad::TutSquadGunFrame) -> Vec<crate::director::UnitRef> {
    let men: Vec<crate::director::UnitRef> = f.m.iter().chain(f.n.iter()).cloned().collect();
    left_of(g, r, &men)
}
/// the closure `gun` (:257)
fn gun_gun(g: &Game, r: DirectorRunId, f: &crate::script_tut_squad::TutSquadGunFrame) -> bool {
    alive(g, r, &f.m) && !crate::director::director_query_routed(g, r, f.m.as_ref().unwrap())
}
/// the closure `burst` (:258)
fn gun_burst(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_squad::TutSquadGunFrame, v: &crate::script_tut_squad::TutSquadVars, rounds: f64) -> Result<crate::director::Cond, String> {
    let at = crate::director::RunBurstsItemAt::Mark(v.lane.clone().unwrap());
    crate::director::director_beat_burst(g, r, b, f.m.as_ref().unwrap(), at, rounds, Some(0.1), Some(if rounds > 3.0 { 0.035 } else { 0.03 }))
}
/// the closure `up` (:259)
fn gun_up(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_squad::TutSquadGunFrame, v: &crate::script_tut_squad::TutSquadVars) -> Result<(), String> {
    if !gun_gun(g, r, f) { return Ok(()); }
    crate::director::director_beat_order(g, r, b, f.m.as_ref().unwrap(), &crate::director::OrderPatch2 {
        goal: Some(None),
        aim_at: Some(Some(crate::director::Place::Mark(v.lane.clone().unwrap()))),
        upper: Some(Some(crate::character::UpperMode::Aim)),
        crouch: Some(Some(false)),
        flinch: Some(Some(true)),
        ..Default::default()
    })
}
/// the closure `down` (:260)
fn gun_down(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_squad::TutSquadGunFrame) -> Result<(), String> {
    if !gun_gun(g, r, f) { return Ok(()); }
    crate::director::director_beat_order(g, r, b, f.m.as_ref().unwrap(), &crate::director::OrderPatch2 {
        goal: Some(None),
        aim_at: Some(None),
        crouch: Some(Some(true)),
        upper: Some(Some(crate::character::UpperMode::Relaxed)),
        flinch: Some(Some(true)),
        ..Default::default()
    })
}

/// (levels/tut-squad.script.ts:253-300 P.beat 'gun') one resume
pub fn tut_squad_gun_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_squad::TutSquadGunFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    let v = tut_squad_vars(g, r).clone();
    loop {
        match f.at {
            0 => {
                f.m = crate::director::director_beat_unit(g, r, GUNNER);
                f.n = crate::director::director_beat_unit(g, r, LOADER);
                for u in [&f.m, &f.n] {
                    if alive(g, r, u) {
                        crate::director::director_beat_order(g, r, b, u.as_ref().unwrap(), &crate::director::OrderPatch2 {
                            goal: Some(None),
                            aim_at: Some(None),
                            crouch: Some(Some(true)),
                            upper: Some(Some(crate::character::UpperMode::Relaxed)),
                            ..Default::default()
                        })?;
                    }
                }
                if !gun_cue(g, r, "gun") {
                    f.at = 1;
                    return Ok(Some(crate::director::director_beat_any(vec![gun_on(GUN_CUE_GUN, "gun"), until(GUN_FEWER + 2, "one of the nest is down"), until(GUN_ROOM, "the player is in the nest")])));
                }
                f.at = 2;
            }
            1 => {
                if got.index > 0.0 && !gun_cue(g, r, "gun") {
                    for u in gun_left(g, r, f).iter() { release_alert(g, r, b, u)?; }
                    return Ok(None);
                }
                f.at = 2;
            }
            2 => {
                if gun_gun(g, r, f) { crate::director::director_beat_show(g, r, b, f.m.as_ref().unwrap(), None)?; }
                if gun_gun(g, r, f) {
                    gun_up(g, r, b, f, &v)?;
                    f.at = 3;
                    return Ok(Some(wait(g, r, 0.4)?));
                }
                f.at = 5;
            }
            3 => {
                let x = gun_burst(g, r, b, f, &v, 5.0)?;
                f.at = 4;
                return Ok(Some(crate::director::director_beat_any(vec![x, wait(g, r, 3.0)?])));
            }
            4 => { f.at = 5; }
            5 => {
                signal(g, r, b, "done:gun");
                f.at = 6;
            }
            // the head of the first `while` (:271): it asks the game again at each pass, nothing of it lives across a yield
            6 => {
                if gun_gun(g, r, f) && !gun_cue(g, r, "ducks") {
                    f.at = 7;
                    return Ok(Some(crate::director::director_beat_any(vec![gun_on(GUN_CUE_DUCKS, "ducks"), gun_down_cond(), wait(g, r, 1.3)?])));
                }
                f.at = 9;
            }
            7 => {
                if !gun_gun(g, r, f) || gun_cue(g, r, "ducks") { f.at = 9; continue; }
                let x = gun_burst(g, r, b, f, &v, 3.0)?;
                f.at = 8;
                return Ok(Some(crate::director::director_beat_any(vec![x, gun_on(GUN_CUE_DUCKS, "ducks"), gun_down_cond(), wait(g, r, 2.0)?])));
            }
            8 => { f.at = 6; }
            // the head of the second `while` (:280), the same
            9 => {
                if gun_gun(g, r, f) && !gun_cue(g, r, "fragged") {
                    gun_down(g, r, b, f)?;
                    f.at = 10;
                    return Ok(Some(crate::director::director_beat_any(vec![gun_on(GUN_CUE_FRAGGED, "fragged"), gun_down_cond(), gun_on(GUN_CUE_ATBLOCKS, "atblocks"), wait(g, r, DOWN_SECS)?])));
                }
                f.at = 13;
            }
            10 => {
                if !gun_gun(g, r, f) || gun_cue(g, r, "fragged") || gun_cue(g, r, "atblocks") { f.at = 13; continue; }
                gun_up(g, r, b, f, &v)?;
                f.at = 11;
                let x = wait(g, r, 0.5)?;
                return Ok(Some(crate::director::director_beat_any(vec![x, gun_on(GUN_CUE_FRAGGED, "fragged"), gun_down_cond(), gun_on(GUN_CUE_ATBLOCKS, "atblocks")])));
            }
            11 => {
                if !gun_gun(g, r, f) || gun_cue(g, r, "fragged") || gun_cue(g, r, "atblocks") { f.at = 13; continue; }
                let x = gun_burst(g, r, b, f, &v, 3.0)?;
                f.at = 12;
                return Ok(Some(crate::director::director_beat_any(vec![x, gun_on(GUN_CUE_FRAGGED, "fragged"), gun_down_cond(), gun_on(GUN_CUE_ATBLOCKS, "atblocks"), wait(g, r, 2.0)?])));
            }
            12 => { f.at = 9; }
            13 => {
                gun_down(g, r, b, f)?;
                f.at = 14;
                return Ok(Some(crate::director::director_beat_any(vec![gun_on(GUN_CUE_FRAGGED, "fragged"), gun_nest_down()])));
            }
            14 => {
                f.at = 15;
                return Ok(Some(crate::director::director_beat_any(vec![wait(g, r, FRAG_SECS)?, gun_nest_down()])));
            }
            15 => {
                let last = gun_left(g, r, f);
                if last.len() == 1 {
                    crate::director::director_beat_order(g, r, b, &last[0], &crate::director::OrderPatch2 {
                        goal: Some(None),
                        aim_at: Some(None),
                        crouch: Some(Some(false)),
                        upper: Some(Some(crate::character::UpperMode::Relaxed)),
                        flinch: Some(Some(true)),
                        ..Default::default()
                    })?;
                    f.at = 16;
                    return Ok(Some(crate::director::director_beat_any(vec![wait(g, r, SHOCK_SECS)?, gun_nest_down()])));
                }
                f.at = 16;
            }
            16 => {
                if alive(g, r, &f.m) { crate::director::director_beat_show(g, r, b, f.m.as_ref().unwrap(), Some(false))?; }
                for u in gun_left(g, r, f).iter() { release_alert(g, r, b, u)?; }
                return Ok(None);
            }
            at => return Err(format!("tut-squad gun: no resume point {}", at)),
        }
    }
}

/// (levels/tut-squad.script.ts:253-300) the c.untils of 'gun'
pub fn tut_squad_gun_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_squad::TutSquadGunFrame, id: u32) -> Result<bool, String> {
    match id {
        GUN_CUE_GUN => Ok(gun_cue(g, r, "gun")),
        GUN_CUE_DUCKS => Ok(gun_cue(g, r, "ducks")),
        GUN_CUE_FRAGGED => Ok(gun_cue(g, r, "fragged")),
        GUN_CUE_ATBLOCKS => Ok(gun_cue(g, r, "atblocks")),
        GUN_DOWN => Ok(!gun_gun(g, r, f)),
        GUN_ROOM => Ok(crate::director::director_query_room_sam(g) == "nest"),
        id if id >= GUN_FEWER => Ok((gun_left(g, r, f).len() as u32) < id - GUN_FEWER),
        id => Err(format!("tut-squad gun: no test {}", id)),
    }
}
