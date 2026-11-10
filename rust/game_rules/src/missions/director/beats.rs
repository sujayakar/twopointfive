//! What the parts of `director` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

// ---- what the by-hand scripts call: the verbs of a level script, which are closures of the TypeScript's D, P, c and q objects. No TypeScript function has these names.

/// INVENTED: B.fn(ctx) at director.ts:586: the beat's frame before its first step. Body by hand, with the nine scripts
pub(super) fn beat_frame_new(script: &crate::scene_level::LevelScriptFn, beat: &str) -> crate::director::BeatFrame {
    use crate::scene_level::LevelScriptFn as S; use crate::director::BeatFrame as F;
    match (script, beat) {
        (S::Unset, b) => panic!("the beat {} has no script: its script was not dealt (io::deal_scripts), or the blank was read", b),
        (S::TutKit, "hold") => F::TutKitHold(Default::default()),
        (S::TutKit, "gun") => F::TutKitGun(Default::default()),
        (S::TutKit, "bags") => F::TutKitBags(Default::default()),
        (S::TutKit, "gap") => F::TutKitGap(Default::default()),
        (S::TutKit, "blind") => F::TutKitBlind(Default::default()),
        (S::TutKit, "shop") => F::TutKitShop(Default::default()),
        (S::TutKit, "room") => F::TutKitRoom(Default::default()),
        (S::TutKit, "loose") => F::TutKitLoose(Default::default()),
        (S::TutKit, "trucks") => F::TutKitTrucks(Default::default()),
        (S::TutKit, "call") => F::TutKitCall(Default::default()),
        (S::TutKit, "box") => F::TutKitBox(Default::default()),
        (S::TutKit, "dark") => F::TutKitDark(Default::default()),
        (S::TutKit, "bunk") => F::TutKitBunk(Default::default()),
        (S::TutKit, "wall") => F::TutKitWall(Default::default()),
        (S::TutKit, "bunkroom") => F::TutKitBunkroom(Default::default()),
        (S::TutKit, "fuelroom") => F::TutKitFuelroom(Default::default()),
        (S::TutKit, "free") => F::TutKitFree(Default::default()),
        (S::TutKit, "seen") => F::TutKitSeen(Default::default()),
        (S::TutKit, "yard") => F::TutKitYard(Default::default()),
        (S::TutPair, "gate") => F::TutPairGate(Default::default()),
        (S::TutPair, "park") => F::TutPairPark(Default::default()),
        (S::TutPair, "yard") => F::TutPairYard(Default::default()),
        (S::TutQueue, "gun") => F::TutQueueGun(Default::default()),
        (S::TutQueue, "gates") => F::TutQueueGates(Default::default()),
        (S::TutQuiet, "lane") => F::TutQuietLane(Default::default()),
        (S::TutQuiet, "night") => F::TutQuietNight(Default::default()),
        (S::TutQuiet, "store") => F::TutQuietStore(Default::default()),
        (S::TutRoad, "hold") => F::TutRoadHold(Default::default()),
        (S::TutRoad, "cease") => F::TutRoadCease(Default::default()),
        (S::TutRoad, "road") => F::TutRoadRoad(Default::default()),
        (S::TutRoad, "way") => F::TutRoadWay(Default::default()),
        (S::TutRoad, "hut") => F::TutRoadHut(Default::default()),
        (S::TutRoad, "gun") => F::TutRoadGun(Default::default()),
        (S::TutRoad, "bags") => F::TutRoadBags(Default::default()),
        (S::TutRoad, "ease") => F::TutRoadEase(Default::default()),
        (S::TutRoad, "gate") => F::TutRoadGate(Default::default()),
        (S::TutRoad, "open") => F::TutRoadOpen(Default::default()),
        (S::TutTeam, "hold") => F::TutTeamHold(Default::default()),
        (S::TutTeam, "cook") => F::TutTeamCook(Default::default()),
        (S::TutTeam, "ambush") => F::TutTeamAmbush(Default::default()),
        (S::TutTeam, "pinned") => F::TutTeamPinned(Default::default()),
        (S::TutTeam, "smoke") => F::TutTeamSmoke(Default::default()),
        (S::TutTeam, "break") => F::TutTeamBreak(Default::default()),
        (S::TutTeam, "ordered") => F::TutTeamOrdered(Default::default()),
        (S::TutTeam, "blind") => F::TutTeamBlind(Default::default()),
        (S::TutTeam, "linger") => F::TutTeamLinger(Default::default()),
        (S::TutTeam, "push") => F::TutTeamPush(Default::default()),
        (S::TutTeam, "indoors") => F::TutTeamIndoors(Default::default()),
        (S::TutTeam, "house") => F::TutTeamHouse(Default::default()),
        (S::TutTeam, "out") => F::TutTeamOut(Default::default()),
        _ => F::Spent,
    }
}
/// INVENTED: it.next(b.result) at director.ts:608: one step, reading g.tab.director_live_beat[b].result. Err = it threw (resume fails the beat), Ok(None) = it returned, Ok(Some) = it yielded. Body by hand, with the nine scripts
pub(super) fn beat_next(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId) -> Result<Option<crate::director::Cond>, String> {
    use crate::director::BeatFrame as F;
    let got = g.tab.director_live_beat[b].result.clone();
    let mut it = core::mem::replace(&mut g.tab.director_live_beat[b].it, F::Spent);
    let out = match &mut it {
        F::Spent => Ok(None),
        F::TutKitHold(f) => crate::script_tut_kit::tut_kit_hold_step(g, r, b, f),
        F::TutKitGun(f) => crate::script_tut_kit::tut_kit_gun_step(g, r, b, f),
        F::TutKitBags(f) => crate::script_tut_kit::tut_kit_bags_step(g, r, f),
        F::TutKitGap(f) => crate::script_tut_kit::tut_kit_gap_step(g, r, f),
        F::TutKitBlind(f) => crate::script_tut_kit::tut_kit_blind_step(g, r, f),
        F::TutKitShop(f) => crate::script_tut_kit::tut_kit_shop_step(g, r, b, f, &got),
        F::TutKitRoom(f) => crate::script_tut_kit::tut_kit_room_step(g, r, b, f, &got),
        F::TutKitLoose(f) => crate::script_tut_kit::tut_kit_loose_step(g, r, f),
        F::TutKitTrucks(f) => crate::script_tut_kit::tut_kit_trucks_step(g, r, b, f),
        F::TutKitCall(f) => crate::script_tut_kit::tut_kit_call_step(g, r, f),
        F::TutKitBox(f) => crate::script_tut_kit::tut_kit_box_step(g, r, f),
        F::TutKitDark(f) => crate::script_tut_kit::tut_kit_dark_step(g, r, f),
        F::TutKitBunk(f) => crate::script_tut_kit::tut_kit_bunk_step(g, r, f),
        F::TutKitWall(f) => crate::script_tut_kit::tut_kit_wall_step(g, r, f),
        F::TutKitBunkroom(f) => crate::script_tut_kit::tut_kit_bunkroom_step(g, r, b, f, &got),
        F::TutKitFuelroom(f) => crate::script_tut_kit::tut_kit_fuelroom_step(g, r, b, f, &got),
        F::TutKitFree(f) => crate::script_tut_kit::tut_kit_free_step(g, r, f),
        F::TutKitSeen(f) => crate::script_tut_kit::tut_kit_seen_step(g, r, f),
        F::TutKitYard(f) => crate::script_tut_kit::tut_kit_yard_step(g, r, b, f),
        F::TutPairGate(f) => crate::script_tut_pair::tut_pair_gate_step(g, r, f),
        F::TutPairPark(f) => crate::script_tut_pair::tut_pair_park_step(g, r, f),
        F::TutPairYard(f) => crate::script_tut_pair::tut_pair_yard_step(g, r, f),
        F::TutQueueGun(f) => crate::script_tut_queue::tut_queue_gun_step(g, r, b, f, &got),
        F::TutQueueGates(f) => crate::script_tut_queue::tut_queue_gates_step(g, r, b, f),
        F::TutQuietLane(f) => crate::script_tut_quiet::tut_quiet_lane_step(g, r, f),
        F::TutQuietNight(f) => crate::script_tut_quiet::tut_quiet_night_step(g, r, b, f, &got),
        F::TutQuietStore(f) => crate::script_tut_quiet::tut_quiet_store_step(g, r, f),
        F::TutRoadHold(f) => crate::script_tut_road::tut_road_hold_step(g, r, b, f),
        F::TutRoadCease(f) => crate::script_tut_road::tut_road_cease_step(g, r, f),
        F::TutRoadRoad(f) => crate::script_tut_road::tut_road_road_step(g, r, b, f),
        F::TutRoadWay(f) => crate::script_tut_road::tut_road_way_step(g, r, f),
        F::TutRoadHut(f) => crate::script_tut_road::tut_road_hut_step(g, r, b, f),
        F::TutRoadGun(f) => crate::script_tut_road::tut_road_gun_step(g, r, b, f),
        F::TutRoadBags(f) => crate::script_tut_road::tut_road_bags_step(g, r, f),
        F::TutRoadEase(f) => crate::script_tut_road::tut_road_ease_step(g, r, f),
        F::TutRoadGate(f) => crate::script_tut_road::tut_road_gate_step(g, r, b, f),
        F::TutRoadOpen(f) => crate::script_tut_road::tut_road_open_step(g, r, f),
        F::TutTeamHold(f) => crate::script_tut_team::tut_team_hold_step(g, r, b, f),
        F::TutTeamCook(f) => crate::script_tut_team::tut_team_cook_step(g, r, b, f),
        F::TutTeamAmbush(f) => crate::script_tut_team::tut_team_ambush_step(g, r, b, f, &got),
        F::TutTeamPinned(f) => crate::script_tut_team::tut_team_pinned_step(g, r, f),
        F::TutTeamSmoke(f) => crate::script_tut_team::tut_team_smoke_step(g, r, b, f),
        F::TutTeamBreak(f) => crate::script_tut_team::tut_team_break_step(g, r, b, f),
        F::TutTeamOrdered(f) => crate::script_tut_team::tut_team_ordered_step(g, r, f),
        F::TutTeamBlind(f) => crate::script_tut_team::tut_team_blind_step(g, r, b, f),
        F::TutTeamLinger(f) => crate::script_tut_team::tut_team_linger_step(g, r, b, f, &got),
        F::TutTeamPush(f) => crate::script_tut_team::tut_team_push_step(g, r, b, f),
        F::TutTeamIndoors(f) => crate::script_tut_team::tut_team_indoors_step(g, r, b, f),
        F::TutTeamHouse(f) => crate::script_tut_team::tut_team_house_step(g, r, f),
        F::TutTeamOut(f) => crate::script_tut_team::tut_team_out_step(g, r, f),
    };
    g.tab.director_live_beat[b].it = it;   // (in the same place: the caller closes a beat that cancelled itself)
    out
}
pub(super) fn beat_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, id: u32) -> Result<bool, String> {
    use crate::director::BeatFrame as F;
    let mut it = core::mem::replace(&mut g.tab.director_live_beat[b].it, F::Spent);
    let out = match &mut it {
        F::TutKitShop(f) => crate::script_tut_kit::tut_kit_shop_test(g, r, f, id),
        F::TutKitRoom(f) => crate::script_tut_kit::tut_kit_room_test(g, r, f, id),
        F::TutKitTrucks(f) => crate::script_tut_kit::tut_kit_trucks_test(g, r, f, id),
        F::TutKitBunkroom(f) => if f.sub_room.is_some() { crate::script_tut_kit::tut_kit_room_helper_test(g, r, f.sub_room.as_mut().unwrap(), id) } else { Err(alloc::format!("a c.until of id {} on the beat bunkroom, with no helper running", id)) },
        F::TutKitFuelroom(f) => if f.sub_room.is_some() { crate::script_tut_kit::tut_kit_room_helper_test(g, r, f.sub_room.as_mut().unwrap(), id) } else { Err(alloc::format!("a c.until of id {} on the beat fuelroom, with no helper running", id)) },
        F::TutKitYard(f) => crate::script_tut_kit::tut_kit_yard_test(g, r, f, id),
        F::TutQueueGun(f) => crate::script_tut_queue::tut_queue_gun_test(g, r, f, id),
        F::TutQueueGates(f) => crate::script_tut_queue::tut_queue_gates_test(g, r, f, id),
        F::TutQuietNight(f) => crate::script_tut_quiet::tut_quiet_night_test(g, r, f, id),
        F::TutRoadRoad(f) => crate::script_tut_road::tut_road_road_test(g, r, f, id),
        F::TutRoadHut(f) => crate::script_tut_road::tut_road_hut_test(g, r, f, id),
        F::TutRoadGun(f) => crate::script_tut_road::tut_road_gun_test(g, r, id),
        F::TutRoadGate(f) => crate::script_tut_road::tut_road_gate_test(g, r, f, id),
        F::TutTeamHold(f) => crate::script_tut_team::tut_team_hold_test(g, r, id),
        F::TutTeamCook(f) => crate::script_tut_team::tut_team_cook_test(g, r, f, id),
        F::TutTeamPinned(f) => crate::script_tut_team::tut_team_pinned_test(g, r, id),
        F::TutTeamSmoke(f) => crate::script_tut_team::tut_team_smoke_test(g, r, id),
        F::TutTeamBreak(f) => crate::script_tut_team::tut_team_break_test(g, r, id),
        F::TutTeamLinger(f) => crate::script_tut_team::tut_team_linger_test(g, r, id),
        F::TutTeamPush(f) => crate::script_tut_team::tut_team_push_test(g, r, b, f, id),
        _ => Err(alloc::format!("a c.until of id {} on a beat with none", id)),
    };
    g.tab.director_live_beat[b].it = it;
    out
}

/// INVENTED: what q.door answers; `swing` is D.swing, DOORWAY.oneWay being settled true on the page
#[derive(Clone, Copy, Debug)]
pub struct QueryDoor { pub angle: f64, pub open: bool, pub latched: bool, pub locked: bool, pub broken: bool, pub swing: f64 }

pub mod util {
    #[allow(unused_imports)]
    use crate::{ids::*, js::*, Quat, V3};
    #[allow(unused_imports)]
    use alloc::{string::String, vec::Vec};

}

/// INVENTED: the define pass of the level this names (the script's `script(D)`), its locals left on the manifest
pub(super) fn script_define(script: crate::scene_level::LevelScriptFn, m: &mut crate::director::Manifest) -> Result<(), String> {
    use crate::scene_level::LevelScriptFn as S; use crate::director::ScriptVars as V;
    m.vars = match script {
        S::Unset => return Err(String::from("a level with no script was asked to define one")),
        S::TutKit => V::TutKit(crate::script_tut_kit::tut_kit_define(m)?),
        S::TutPair => V::TutPair(crate::script_tut_pair::tut_pair_define(m)?),
        S::TutQueue => V::TutQueue(crate::script_tut_queue::tut_queue_define(m)?),
        S::TutQuiet => V::TutQuiet(crate::script_tut_quiet::tut_quiet_define(m)?),
        S::TutRoad => V::TutRoad(crate::script_tut_road::tut_road_define(m)?),
        S::TutTeam => V::TutTeam(crate::script_tut_team::tut_team_define(m)?),
    };
    Ok(())
}

/// INVENTED: the script of the level of this name, as scene/levelScripts.ts LEVEL_SCRIPTS hangs it (each script file is levels/<name>.script.ts)
pub fn level_script_of(name: &str) -> Option<crate::scene_level::LevelScriptFn> {
    use crate::scene_level::LevelScriptFn as S;
    match name {
        "tut-kit" => Some(S::TutKit),
        "tut-pair" => Some(S::TutPair),
        "tut-queue" => Some(S::TutQueue),
        "tut-quiet" => Some(S::TutQuiet),
        "tut-road" => Some(S::TutRoad),
        "tut-team" => Some(S::TutTeam),
        _ => None,
    }
}
