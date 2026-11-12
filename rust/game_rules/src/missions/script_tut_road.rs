//! The script of the level tut-road: its define pass and a step function for each beat.

pub use ::game_types::script_tut_road::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap};
use crate::m;
use crate::v3::V3;
use alloc::{format, string::String, vec, vec::Vec};

/// the script's locals, as this run's define pass left them
fn tut_road_vars(g: &crate::game::Game, r: DirectorRunId) -> &crate::script_tut_road::TutRoadVars { match &g.tab.director_run[r].vars { crate::director::ScriptVars::TutRoad(v) => v, _ => panic!("the run is not of the level tut-road") } }

/// the truck's flank as the sentry sees it through the gateway
const TRUCK_FLANK: [V3; 4] = [[5.6, 1.35, 13.05], [5.6, 0.85, 13.4], [5.6, 1.5, 13.75], [5.6, 1.1, 13.2]];
const HUT_MARK: V3 = [20.7, 0.0, 11.45];
/// what the man behind the plank wall shoots at, blind, through the boards (:22)
const HUT_WALL: [V3; 4] = [[18.3, 2.3, 9.83], [19.0, 2.2, 9.83], [19.6, 2.35, 9.83], [18.7, 2.4, 9.83]];
/// what the gun shoots at as he comes out of the hut (:25)
const HUT_END: [V3; 8] = [[22.53, 1.5, 10.6], [22.53, 2.1, 11.6], [22.53, 1.2, 12.5], [22.53, 2.3, 14.3], [22.53, 1.8, 13.2], [22.53, 2.2, 15.0], [22.53, 1.4, 11.1], [22.53, 2.0, 14.7]];
/// along the sandbags, a point every half metre: the gun's traverse (:28, its map written out)
const BAGS_TOP: [V3; 10] = [[22.8, 1.42, 15.5], [23.3, 1.42, 15.5], [23.8, 1.42, 15.5], [24.3, 1.42, 15.5], [24.8, 1.42, 15.5], [25.3, 1.42, 15.5], [25.8, 1.42, 15.5], [26.3, 1.42, 15.5], [26.8, 1.42, 15.5], [27.2, 1.42, 15.5]];
/// the roadblock's wall, the gun's face of it (:30)
const WALL_FACE: [V3; 5] = [[27.8, 1.3, 15.37], [28.5, 1.7, 15.37], [29.1, 1.2, 15.37], [28.1, 1.9, 15.37], [28.8, 1.5, 15.37]];
const LAST_MEN: [V3; 3] = [[36.1, 0.0, 13.3], [35.4, 0.0, 15.3], [36.1, 0.0, 16.1]];
const WALL_HEAD: [[V3; 2]; 3] = [[[28.0, 2.3, 15.5], [28.4, 2.25, 15.5]], [[28.8, 2.3, 15.5], [28.6, 2.2, 15.5]], [[29.3, 2.3, 15.5], [29.0, 2.25, 15.5]]];
/// the gateway and the upper gate, chest high: where a held man looks (:37)
const GATEWAY: V3 = [11.0, 1.2, 15.2];
const HUT_DOOR: V3 = [17.0, 1.2, 12.75];
const HUT_SIDE: V3 = [22.5, 1.3, 14.4];
const GATE: V3 = [37.0, 1.2, 14.25];

///  a man told to aim somewhere turns his whole body to it
fn aim(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &crate::director::UnitRef, at: V3, hot: bool) -> Result<(), String> {
    let face = crate::director::util::yaw_to(crate::director::director_query_pos_unit(g, r, u)?, at);
    let patch = crate::director::OrderPatch2 {
        aim_at: Some(Some(crate::director::Place::List(at))),
        upper: Some(Some(crate::character::UpperMode::Aim)),
        face: Some(Some(face)),
        hot: if hot { Some(Some(true)) } else { None },
        ..Default::default()
    };
    crate::director::director_beat_order(g, r, b, u, &patch)
}

/// c.query.inside(v, 'sam') of one of the script's volumes
fn sam_in(g: &Game, r: DirectorRunId, pick: fn(&TutRoadVars) -> &Option<crate::director::VolumeRef>) -> bool {
    let id = pick(tut_road_vars(g, r)).as_ref().expect("declared by tut_road_define").id.clone();
    crate::director::director_inside_volume(g, &crate::seats::here(g), r, &id, &crate::director::DirectorInsideVolumeWho::Sam)
}

///  the player is on the sentry's firing lane, or coming round the truck's nose to it
fn lane(g: &Game, r: DirectorRunId) -> bool {
    sam_in(g, r, |v| &v.road_east) || sam_in(g, r, |v| &v.road_nose)
}

///  the player is out on the gun's own ground
fn out(g: &Game, r: DirectorRunId) -> bool {
    sam_in(g, r, |v| &v.field) || sam_in(g, r, |v| &v.open_east)
}

///  how many of the last three still stand
fn up(g: &Game, r: DirectorRunId, men: &[TutRoadGateMen]) -> f64 {
    let mut n = 0.0;
    for m in men {
        if crate::director::director_query_alive(g, r, m.u.as_ref().expect("men holds found men only")) { n += 1.0; }
    }
    n
}

/// `c.flag[name]` as a condition: a Json's truth in JavaScript
fn flag_on(g: &Game, r: DirectorRunId, name: &str) -> bool {
    match g.tab.director_run[r].flags.of(name) {
        None | Some(None) => false,
        Some(Some(crate::director::Json::Bool(x))) => *x,
        Some(Some(crate::director::Json::Num(x))) => js::truthy(*x),
        Some(Some(crate::director::Json::Text(s))) => !s.is_empty(),
        Some(Some(_)) => true,
    }
}

///  the define pass: everything the script names and declares, through crate::director::manifest_* and program_*. Its locals a beat reads go in the answer.
pub(super) fn tut_road_define(m: &mut crate::director::Manifest) -> Result<crate::script_tut_road::TutRoadVars, String> {
    let hut_door = crate::director::manifest_door(m, "hut");
    let gate = crate::director::manifest_door(m, "gate");
    let road_east = crate::director::manifest_volume(m, "road_east");
    let road_nose = crate::director::manifest_volume(m, "road_nose");
    let hut_north = crate::director::manifest_volume(m, "hut_north");
    let field = crate::director::manifest_volume(m, "field");
    let open_east = crate::director::manifest_volume(m, "open_east");
    let staff: Vec<String> = ["Kowalski", "Novak", "Reyes", "Okafor", "Lindqvist", "Brandt"].iter().map(|n| String::from(*n)).collect();
    let last: Vec<String> = ["Okafor", "Lindqvist", "Brandt"].iter().map(|n| String::from(*n)).collect();
    let mut aims: JsMap<String, V3> = JsMap::new();
    aims.set(String::from("Kowalski"), TRUCK_FLANK[1]);
    aims.set(String::from("Novak"), HUT_DOOR);
    aims.set(String::from("Reyes"), HUT_SIDE);
    aims.set(String::from("Okafor"), GATE);
    aims.set(String::from("Lindqvist"), GATE);
    aims.set(String::from("Brandt"), GATE);

    crate::director::manifest_program(m, "tutorial", &mut |p: &mut crate::director::ProgramDef| {
        crate::director::program_on_start(p, "hold");
        for cue in ["road", "cease", "way", "hut", "gun", "bags", "ease", "gate", "open"] {
            crate::director::program_on_signal(p, &format!("lesson:{}", cue), cue);
        }
        let s = crate::scene_level::LevelScriptFn::TutRoad;
        for beat in ["hold", "cease", "road", "way", "hut", "gun", "bags", "ease", "gate", "open"] {
            crate::director::program_beat(p, beat, &s, None)?;
        }
        Ok(())
    })?;

    Ok(TutRoadVars {
        hut_door: Some(hut_door),
        gate: Some(gate),
        road_east: Some(road_east),
        road_nose: Some(road_nose),
        hut_north: Some(hut_north),
        field: Some(field),
        open_east: Some(open_east),
        staff,
        last,
        aims: Some(aims),
    })
}

/// beat 'hold': every man on his mark, his gun on the way the player will come.
pub(super) fn tut_road_hold_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_road::TutRoadHoldFrame) -> Result<Option<crate::director::Cond>, String> {
    if f.at == 0 {
        let staff = tut_road_vars(g, r).staff.clone();
        for n in &staff {
            let Some(u) = crate::director::director_beat_unit(g, n) else { continue };
            if !crate::director::director_query_alive(g, r, &u) { continue; }
            let at = *tut_road_vars(g, r).aims.as_ref().expect("set by tut_road_define").get(n.clone()).expect("every man of STAFF has his aim");
            let face = crate::director::util::yaw_to(crate::director::director_query_pos_unit(g, r, &u)?, at);
            let patch = crate::director::OrderPatch2 {
                goal: Some(None),
                aim_at: Some(Some(crate::director::Place::List(at))),
                upper: Some(Some(crate::character::UpperMode::Aim)),
                face: Some(Some(face)),
                ..Default::default()
            };
            crate::director::director_beat_order(g, r, b, &u, &patch)?;
        }
        f.at = 1;
        return Ok(Some(crate::director::director_beat_frames(g, r, Some(1.0))));
    }
    Ok(None)
}

/// beat 'cease': the sentry's cue to stop shooting at the truck.
pub(super) fn tut_road_cease_step(g: &mut crate::game::Game, r: DirectorRunId, f: &mut crate::script_tut_road::TutRoadCeaseFrame) -> Result<Option<crate::director::Cond>, String> {
    if f.at == 0 {
        g.tab.director_run[r].flags.set(String::from("cease"), Some(crate::director::Json::Bool(true)));
        f.at = 1;
        return Ok(Some(crate::director::director_beat_frames(g, r, Some(1.0))));
    }
    Ok(None)
}

/// beat 'road': the sentry puts rounds into the truck until the player is at the post's wall.
pub(super) fn tut_road_road_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_road::TutRoadRoadFrame) -> Result<Option<crate::director::Cond>, String> {
    loop {
        match f.at {
            0 => {
                let Some(s) = crate::director::director_beat_unit(g, "Kowalski") else { return Ok(None) };
                f.s = Some(s);
                f.at = 1;
                return Ok(Some(crate::director::director_beat_wait(g, r, 0.8)?));
            }
            // the first wait is over: the loop's start
            1 => { f.i = 0.0; f.at = 2; }
            // the loop's head
            2 => {
                let s = f.s.clone().expect("set at 0");
                if crate::director::director_query_alive(g, r, &s) && !flag_on(g, r, "cease") {
                    if lane(g, r) {
                        f.at = 3;
                        return Ok(Some(crate::director::director_beat_wait(g, r, 0.2)?));
                    }
                    let at = TRUCK_FLANK[f.i as usize % TRUCK_FLANK.len()];
                    aim(g, r, b, &s, at, true)?;
                    f.at = 4;
                    return Ok(Some(crate::director::director_beat_burst(g, r, b, &s, crate::director::RunBurstsItemAt::List(at), 3.0, Some(0.12), Some(0.02))?));
                }
                if crate::director::director_query_alive(g, r, &s) { aim(g, r, b, &s, GATEWAY, false)?; }
                f.at = 6;
                return Ok(Some(crate::director::director_beat_until(crate::director::CondTest::Script { id: 0 })));
            }
            // `continue` in a for: the update still runs
            3 => { f.i += 1.0; f.at = 2; }
            4 => {
                let secs = 1.3 + crate::director::director_beat_rand(g, r) * 1.1;
                f.at = 5;
                return Ok(Some(crate::director::director_beat_wait(g, r, secs)?));
            }
            5 => { f.i += 1.0; f.at = 2; }
            // 6: the sentry is down
            _ => return Ok(None),
        }
    }
}

///  id 0: the sentry is down.
pub(super) fn tut_road_road_test(g: &crate::game::Game, r: DirectorRunId, f: &crate::script_tut_road::TutRoadRoadFrame, id: u32) -> Result<bool, String> {
    match id {
        0 => Ok(!crate::director::director_query_alive(g, r, f.s.as_ref().expect("set at 0"))),
        _ => Err(format!("tut_road 'road': no c.until of id {}", id)),
    }
}

/// beat 'way': the hut's door is thrown open if the lesson went on with it still standing.
pub(super) fn tut_road_way_step(g: &mut crate::game::Game, r: DirectorRunId, f: &mut crate::script_tut_road::TutRoadWayFrame) -> Result<Option<crate::director::Cond>, String> {
    if f.at == 0 {
        let d = tut_road_vars(g, r).hut_door.clone().expect("set by tut_road_define");
        if !crate::director::director_query_door(g, &d)?.open {
            crate::director::director_beat_door_lock(g, &d, false)?;
            crate::director::director_beat_door_open(g, &d, Some(1.5), Some(true))?;
        }
        f.at = 1;
        return Ok(Some(crate::director::director_beat_frames(g, r, Some(1.0))));
    }
    Ok(None)
}

/// beat 'hut': the man in the back room shoots through the boards at the wall over the front room.
pub(super) fn tut_road_hut_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_road::TutRoadHutFrame) -> Result<Option<crate::director::Cond>, String> {
    loop {
        match f.at {
            0 => {
                let Some(u) = crate::director::director_beat_unit(g, "Novak") else { return Ok(None) };
                let patch = crate::director::OrderPatch2 {
                    goal: Some(Some(crate::director::Place::List(HUT_MARK))),
                    speed: Some(Some(2.6)),
                    aim_at: Some(Some(crate::director::Place::List(HUT_WALL[0]))),
                    upper: Some(Some(crate::character::UpperMode::Aim)),
                    hot: Some(Some(true)),
                    ..Default::default()
                };
                crate::director::director_beat_order(g, r, b, &u, &patch)?;
                f.u = Some(u);
                f.at = 1;
                let there = crate::director::director_beat_until(crate::director::CondTest::Script { id: 0 });
                let wait = crate::director::director_beat_wait(g, r, 4.0)?;
                return Ok(Some(crate::director::director_beat_any(vec![there, wait])));
            }
            1 => {
                let u = f.u.clone().expect("set at 0");
                if crate::director::director_query_alive(g, r, &u) {
                    let face = crate::director::util::yaw_to(crate::director::director_query_pos_unit(g, r, &u)?, HUT_WALL[0]);
                    let patch = crate::director::OrderPatch2 {
                        goal: Some(None),
                        aim_at: Some(Some(crate::director::Place::List(HUT_WALL[0]))),
                        upper: Some(Some(crate::character::UpperMode::Aim)),
                        face: Some(Some(face)),
                        hot: Some(Some(true)),
                        ..Default::default()
                    };
                    crate::director::director_beat_order(g, r, b, &u, &patch)?;
                }
                f.at = 2;
                return Ok(Some(crate::director::director_beat_wait(g, r, 0.3)?));
            }
            // the loop's start
            2 => { f.i = 0.0; f.at = 3; }
            // the loop's head
            3 => {
                let u = f.u.clone().expect("set at 0");
                if !crate::director::director_query_alive(g, r, &u) { return Ok(None); }
                if sam_in(g, r, |v| &v.hut_north) {
                    f.at = 4;
                    return Ok(Some(crate::director::director_beat_wait(g, r, 0.2)?));
                }
                let at = HUT_WALL[f.i as usize % HUT_WALL.len()];
                aim(g, r, b, &u, at, true)?;
                f.at = 5;
                return Ok(Some(crate::director::director_beat_burst(g, r, b, &u, crate::director::RunBurstsItemAt::List(at), 3.0, Some(0.12), Some(0.006))?));
            }
            // `continue` in a for: the update still runs
            4 => { f.i += 1.0; f.at = 3; }
            5 => {
                let secs = 0.9 + crate::director::director_beat_rand(g, r) * 0.7;
                f.at = 6;
                return Ok(Some(crate::director::director_beat_wait(g, r, secs)?));
            }
            6 => { f.i += 1.0; f.at = 3; }
            _ => return Ok(None),
        }
    }
}

///  id 0: he is at the boards, or down.
pub(super) fn tut_road_hut_test(g: &crate::game::Game, r: DirectorRunId, f: &crate::script_tut_road::TutRoadHutFrame, id: u32) -> Result<bool, String> {
    match id {
        0 => {
            let u = f.u.as_ref().expect("set at 0");
            if !crate::director::director_query_alive(g, r, u) { return Ok(true); }
            Ok(crate::director::util::dist_xz(crate::director::director_query_pos_unit(g, r, u)?, HUT_MARK) < 0.2)
        }
        _ => Err(format!("tut_road 'hut': no c.until of id {}", id)),
    }
}

/// beat 'gun': one man on a belt-fed gun: where his rounds go is the lesson's; a player out on the snow is the game's gunner's.
pub(super) fn tut_road_gun_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_road::TutRoadGunFrame) -> Result<Option<crate::director::Cond>, String> {
    loop {
        match f.at {
            0 => {
                let Some(u) = crate::director::director_beat_unit(g, "Reyes") else { return Ok(None) };
                f.u = Some(u);
                f.at = 1;
                return Ok(Some(crate::director::director_beat_wait(g, r, 0.5)?));
            }
            // the loop's start
            1 => {
                f.k = (BAGS_TOP.len() - 1) as f64;
                f.dir = -1.0;
                f.at = 2;
            }
            // the loop's head
            2 => {
                let u = f.u.clone().expect("set at 0");
                if !crate::director::director_query_alive(g, r, &u) { return Ok(None); }
                if out(g, r) {
                    crate::director::director_beat_release(g, r, b, &u, Some(crate::game::GuardState::Alert))?;
                    return Ok(None);
                }
                f.mode = super::script_tut_kit::flag_text(g, r, "gun");
                match f.mode.as_deref() {
                    Some("bags") => {
                        let at = BAGS_TOP[f.k as usize];
                        f.k += f.dir;
                        if f.k <= 0.0 || f.k >= (BAGS_TOP.len() - 1) as f64 { f.dir = -f.dir; }
                        aim(g, r, b, &u, at, true)?;
                        f.at = 3;
                        return Ok(Some(crate::director::director_beat_burst(g, r, b, &u, crate::director::RunBurstsItemAt::List(at), 4.0, Some(0.08), Some(0.01))?));
                    }
                    mode => {
                        let ease = mode == Some("ease");
                        let pts: &[V3] = if ease { &WALL_FACE } else { &HUT_END };
                        let n = pts.len() as f64;
                        let at = pts[js::rem(m::floor(crate::director::director_beat_rand(g, r) * n), n) as usize];
                        aim(g, r, b, &u, at, true)?;
                        f.at = 5;
                        return Ok(Some(crate::director::director_beat_burst(g, r, b, &u, crate::director::RunBurstsItemAt::List(at), if ease { 5.0 } else { 8.0 }, Some(0.1), Some(if ease { 0.008 } else { 0.016 }))?));
                    }
                }
            }
            3 => {
                f.at = 4;
                return Ok(Some(crate::director::director_beat_wait(g, r, 0.1)?));
            }
            4 => {  f.at = 2; }
            5 => {
                // (the mode this pass of the loop read, kept across the burst: the flag may have moved since)
                let secs = if f.mode.as_deref() == Some("ease") { 1.0 + crate::director::director_beat_rand(g, r) * 0.8 } else { 0.7 + crate::director::director_beat_rand(g, r) * 0.7 };
                let wait = crate::director::director_beat_wait(g, r, secs)?;
                let open = crate::director::director_beat_until(crate::director::CondTest::Script { id: 0 });
                f.at = 6;
                return Ok(Some(crate::director::director_beat_any(vec![wait, open])));
            }
            6 => {  f.at = 2; }
            _ => return Ok(None),
        }
    }
}

///  id 0: the player is out in the open.
pub(super) fn tut_road_gun_test(g: &crate::game::Game, r: DirectorRunId, id: u32) -> Result<bool, String> {
    match id {
        0 => Ok(out(g, r)),
        _ => Err(format!("tut_road 'gun': no c.until of id {}", id)),
    }
}

/// beat 'bags': the gun's traverse along the sandbags.
pub(super) fn tut_road_bags_step(g: &mut crate::game::Game, r: DirectorRunId, f: &mut crate::script_tut_road::TutRoadBagsFrame) -> Result<Option<crate::director::Cond>, String> {
    if f.at == 0 {
        g.tab.director_run[r].flags.set(String::from("gun"), Some(crate::director::Json::Text(String::from("bags"))));
        f.at = 1;
        return Ok(Some(crate::director::director_beat_frames(g, r, Some(1.0))));
    }
    Ok(None)
}

/// beat 'ease': the gun moves to the roadblock's wall.
pub(super) fn tut_road_ease_step(g: &mut crate::game::Game, r: DirectorRunId, f: &mut crate::script_tut_road::TutRoadEaseFrame) -> Result<Option<crate::director::Cond>, String> {
    if f.at == 0 {
        g.tab.director_run[r].flags.set(String::from("gun"), Some(crate::director::Json::Text(String::from("ease"))));
        f.at = 1;
        return Ok(Some(crate::director::director_beat_frames(g, r, Some(1.0))));
    }
    Ok(None)
}

/// beat 'gate': three more come in by the upper gate and shoot the head of the wall, until one of them is down.
pub(super) fn tut_road_gate_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_road::TutRoadGateFrame) -> Result<Option<crate::director::Cond>, String> {
    loop {
        match f.at {
            0 => {
                f.at = 1;
                return Ok(Some(crate::director::director_beat_wait(g, r, 0.6)?));
            }
            1 => {
                let last = tut_road_vars(g, r).last.clone();
                let mut found: Vec<TutRoadGateMen> = Vec::new();
                for (k, n) in last.iter().enumerate() {
                    found.push(TutRoadGateMen { k: k as f64, u: crate::director::director_beat_unit(g, n) });
                }
                let mut men: Vec<TutRoadGateMen> = Vec::new();
                for m in found {
                    let keep = match &m.u { Some(u) => crate::director::director_query_alive(g, r, u), None => false };
                    if keep { men.push(m); }
                }
                f.men = men;
                if f.men.is_empty() {
                    crate::director::director_beat_signal(g, r, "done:gate");
                    return Ok(None);
                }
                f.all = f.men.len() as f64;
                for m in &f.men {
                    let k = m.k as usize;
                    let patch = crate::director::OrderPatch2 {
                        goal: Some(Some(crate::director::Place::List(LAST_MEN[k]))),
                        speed: Some(Some(3.4)),
                        aim_at: Some(Some(crate::director::Place::List(WALL_HEAD[k][0]))),
                        upper: Some(Some(crate::character::UpperMode::Aim)),
                        hot: Some(Some(true)),
                        ..Default::default()
                    };
                    crate::director::director_beat_order(g, r, b, m.u.as_ref().expect("men holds found men only"), &patch)?;
                }
                f.at = 2;
                let lamp = crate::director::director_beat_until(crate::director::CondTest::Script { id: 0 });
                let wait = crate::director::director_beat_wait(g, r, 7.0)?;
                return Ok(Some(crate::director::director_beat_any(vec![lamp, wait])));
            }
            2 => {
                crate::director::director_beat_signal(g, r, "done:gate");
                let d = tut_road_vars(g, r).gate.clone().expect("set by tut_road_define");
                crate::director::director_beat_door_open(g, &d, Some(-1.45), Some(true))?;
                f.i = 0.0;
                f.at = 3;
            }
            // the loop's head
            3 => {
                if up(g, r, &f.men) != f.all {
                    f.at = 7;
                    return Ok(Some(crate::director::director_beat_wait(g, r, 0.9)?));
                }
                if sam_in(g, r, |v| &v.open_east) {
                    let wait = crate::director::director_beat_wait(g, r, 0.3)?;
                    let down = crate::director::director_beat_until(crate::director::CondTest::Script { id: 1 });
                    f.at = 4;
                    return Ok(Some(crate::director::director_beat_any(vec![wait, down])));
                }
                let mut shots: Vec<crate::director::Cond> = Vec::new();
                for m in &f.men {
                    let k = m.k as usize;
                    let u = m.u.as_ref().expect("men holds found men only");
                    let at = WALL_HEAD[k][f.i as usize % WALL_HEAD[k].len()];
                    let face = crate::director::util::yaw_to(crate::director::director_query_pos_unit(g, r, u)?, at);
                    let patch = crate::director::OrderPatch2 { aim_at: Some(Some(crate::director::Place::List(at))), face: Some(Some(face)), ..Default::default() };
                    crate::director::director_beat_order(g, r, b, u, &patch)?;
                    shots.push(crate::director::director_beat_burst(g, r, b, u, crate::director::RunBurstsItemAt::List(at), 4.0, Some(0.14), Some(0.008))?);
                }
                let all = crate::director::director_beat_all(shots);
                let down = crate::director::director_beat_until(crate::director::CondTest::Script { id: 2 });
                f.at = 5;
                return Ok(Some(crate::director::director_beat_any(vec![all, down])));
            }
            // `continue` in a for: the update still runs
            4 => { f.i += 1.0; f.at = 3; }
            5 => {
                let secs = 0.9 + crate::director::director_beat_rand(g, r) * 0.6;
                let wait = crate::director::director_beat_wait(g, r, secs)?;
                let down = crate::director::director_beat_until(crate::director::CondTest::Script { id: 3 });
                f.at = 6;
                return Ok(Some(crate::director::director_beat_any(vec![wait, down])));
            }
            6 => { f.i += 1.0; f.at = 3; }
            7 => {
                for m in &f.men {
                    let u = m.u.as_ref().expect("men holds found men only");
                    if crate::director::director_query_alive(g, r, u) {
                        crate::director::director_beat_release(g, r, b, u, Some(crate::game::GuardState::Alert))?;
                    }
                }
                return Ok(None);
            }
            _ => return Ok(None),
        }
    }
}

///  id 0: each of the men has arrived or is down; ids 1 to 3: one of them is down.
pub(super) fn tut_road_gate_test(g: &crate::game::Game, r: DirectorRunId, f: &crate::script_tut_road::TutRoadGateFrame, id: u32) -> Result<bool, String> {
    match id {
        0 => {
            for m in &f.men {
                let u = m.u.as_ref().expect("men holds found men only");
                if crate::director::director_query_alive(g, r, u) && !crate::director::director_query_arrived(g, r, u)? { return Ok(false); }
            }
            Ok(true)
        }
        1 | 2 | 3 => Ok(up(g, r, &f.men) < f.all),
        _ => Err(format!("tut_road 'gate': no c.until of id {}", id)),
    }
}

/// beat 'open': the gate stands open on its stay.
pub(super) fn tut_road_open_step(g: &mut crate::game::Game, r: DirectorRunId, f: &mut crate::script_tut_road::TutRoadOpenFrame) -> Result<Option<crate::director::Cond>, String> {
    if f.at == 0 {
        let d = tut_road_vars(g, r).gate.clone().expect("set by tut_road_define");
        crate::director::director_beat_door_open(g, &d, Some(-1.45), Some(true))?;
        f.at = 1;
        return Ok(Some(crate::director::director_beat_frames(g, r, Some(1.0))));
    }
    Ok(None)
}
