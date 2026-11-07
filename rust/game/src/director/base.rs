//! What the parts of `director` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


// ---- what the by-hand scripts call: the verbs of a level script, which are closures of the TypeScript's D, P, c and q objects. No TypeScript function has these names.

/// INVENTED: B.fn(ctx) at director.ts:586: the beat's frame before its first step. Body by hand, with the nine scripts
pub fn beat_frame_new(script: &crate::scene_level::LevelScriptFn, beat: &str) -> crate::director::BeatFrame {
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
        (S::TutSky, "hold") => F::TutSkyHold(Default::default()),
        (S::TutSky, "ambush") => F::TutSkyAmbush(Default::default()),
        (S::TutSky, "pinned") => F::TutSkyPinned(Default::default()),
        (S::TutSky, "smoke") => F::TutSkySmoke(Default::default()),
        (S::TutSky, "break") => F::TutSkyBreak(Default::default()),
        (S::TutSky, "ordered") => F::TutSkyOrdered(Default::default()),
        (S::TutSky, "blind") => F::TutSkyBlind(Default::default()),
        (S::TutSky, "push") => F::TutSkyPush(Default::default()),
        (S::TutSky, "indoors") => F::TutSkyIndoors(Default::default()),
        (S::TutSky, "wall") => F::TutSkyWall(Default::default()),
        (S::TutSky, "dark") => F::TutSkyDark(Default::default()),
        (S::TutSky, "nv") => F::TutSkyNv(Default::default()),
        (S::TutSky, "torches") => F::TutSkyTorches(Default::default()),
        (S::TutSky, "inside") => F::TutSkyInside(Default::default()),
        (S::TutSky, "out") => F::TutSkyOut(Default::default()),
        (S::TutSolo, "hold") => F::TutSoloHold(Default::default()),
        (S::TutSolo, "cease") => F::TutSoloCease(Default::default()),
        (S::TutSolo, "road") => F::TutSoloRoad(Default::default()),
        (S::TutSolo, "way") => F::TutSoloWay(Default::default()),
        (S::TutSolo, "gun") => F::TutSoloGun(Default::default()),
        (S::TutSolo, "bags") => F::TutSoloBags(Default::default()),
        (S::TutSolo, "gap") => F::TutSoloGap(Default::default()),
        (S::TutSolo, "blind") => F::TutSoloBlind(Default::default()),
        (S::TutSolo, "shop") => F::TutSoloShop(Default::default()),
        (S::TutSolo, "room") => F::TutSoloRoom(Default::default()),
        (S::TutSolo, "loose") => F::TutSoloLoose(Default::default()),
        (S::TutSolo, "trucks") => F::TutSoloTrucks(Default::default()),
        (S::TutSquad, "picket") => F::TutSquadPicket(Default::default()),
        (S::TutSquad, "store") => F::TutSquadStore(Default::default()),
        (S::TutSquad, "shop") => F::TutSquadShop(Default::default()),
        (S::TutSquad, "hall") => F::TutSquadHall(Default::default()),
        (S::TutSquad, "gun") => F::TutSquadGun(Default::default()),
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
pub fn beat_next(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId) -> Result<Option<crate::director::Cond>, String> {
    use crate::director::BeatFrame as F;
    let got = g.tab.director_live_beat[b].result.clone();
    let mut it = core::mem::replace(&mut g.tab.director_live_beat[b].it, F::Spent);
    let out = match &mut it {
        F::Spent => Ok(None),
        F::TutKitHold(f) => crate::script_tut_kit::tut_kit_hold_step(g, r, b, f, &got),
        F::TutKitGun(f) => crate::script_tut_kit::tut_kit_gun_step(g, r, b, f, &got),
        F::TutKitBags(f) => crate::script_tut_kit::tut_kit_bags_step(g, r, b, f, &got),
        F::TutKitGap(f) => crate::script_tut_kit::tut_kit_gap_step(g, r, b, f, &got),
        F::TutKitBlind(f) => crate::script_tut_kit::tut_kit_blind_step(g, r, b, f, &got),
        F::TutKitShop(f) => crate::script_tut_kit::tut_kit_shop_step(g, r, b, f, &got),
        F::TutKitRoom(f) => crate::script_tut_kit::tut_kit_room_step(g, r, b, f, &got),
        F::TutKitLoose(f) => crate::script_tut_kit::tut_kit_loose_step(g, r, b, f, &got),
        F::TutKitTrucks(f) => crate::script_tut_kit::tut_kit_trucks_step(g, r, b, f, &got),
        F::TutKitCall(f) => crate::script_tut_kit::tut_kit_call_step(g, r, b, f, &got),
        F::TutKitBox(f) => crate::script_tut_kit::tut_kit_box_step(g, r, b, f, &got),
        F::TutKitDark(f) => crate::script_tut_kit::tut_kit_dark_step(g, r, b, f, &got),
        F::TutKitBunk(f) => crate::script_tut_kit::tut_kit_bunk_step(g, r, b, f, &got),
        F::TutKitWall(f) => crate::script_tut_kit::tut_kit_wall_step(g, r, b, f, &got),
        F::TutKitBunkroom(f) => crate::script_tut_kit::tut_kit_bunkroom_step(g, r, b, f, &got),
        F::TutKitFuelroom(f) => crate::script_tut_kit::tut_kit_fuelroom_step(g, r, b, f, &got),
        F::TutKitFree(f) => crate::script_tut_kit::tut_kit_free_step(g, r, b, f, &got),
        F::TutKitSeen(f) => crate::script_tut_kit::tut_kit_seen_step(g, r, b, f, &got),
        F::TutKitYard(f) => crate::script_tut_kit::tut_kit_yard_step(g, r, b, f, &got),
        F::TutPairGate(f) => crate::script_tut_pair::tut_pair_gate_step(g, r, b, f, &got),
        F::TutPairPark(f) => crate::script_tut_pair::tut_pair_park_step(g, r, b, f, &got),
        F::TutPairYard(f) => crate::script_tut_pair::tut_pair_yard_step(g, r, b, f, &got),
        F::TutQueueGun(f) => crate::script_tut_queue::tut_queue_gun_step(g, r, b, f, &got),
        F::TutQueueGates(f) => crate::script_tut_queue::tut_queue_gates_step(g, r, b, f, &got),
        F::TutQuietLane(f) => crate::script_tut_quiet::tut_quiet_lane_step(g, r, b, f, &got),
        F::TutQuietNight(f) => crate::script_tut_quiet::tut_quiet_night_step(g, r, b, f, &got),
        F::TutQuietStore(f) => crate::script_tut_quiet::tut_quiet_store_step(g, r, b, f, &got),
        F::TutRoadHold(f) => crate::script_tut_road::tut_road_hold_step(g, r, b, f, &got),
        F::TutRoadCease(f) => crate::script_tut_road::tut_road_cease_step(g, r, b, f, &got),
        F::TutRoadRoad(f) => crate::script_tut_road::tut_road_road_step(g, r, b, f, &got),
        F::TutRoadWay(f) => crate::script_tut_road::tut_road_way_step(g, r, b, f, &got),
        F::TutRoadHut(f) => crate::script_tut_road::tut_road_hut_step(g, r, b, f, &got),
        F::TutRoadGun(f) => crate::script_tut_road::tut_road_gun_step(g, r, b, f, &got),
        F::TutRoadBags(f) => crate::script_tut_road::tut_road_bags_step(g, r, b, f, &got),
        F::TutRoadEase(f) => crate::script_tut_road::tut_road_ease_step(g, r, b, f, &got),
        F::TutRoadGate(f) => crate::script_tut_road::tut_road_gate_step(g, r, b, f, &got),
        F::TutRoadOpen(f) => crate::script_tut_road::tut_road_open_step(g, r, b, f, &got),
        F::TutSkyHold(f) => crate::script_tut_sky::tut_sky_hold_step(g, r, b, f, &got),
        F::TutSkyAmbush(f) => crate::script_tut_sky::tut_sky_ambush_step(g, r, b, f, &got),
        F::TutSkyPinned(f) => crate::script_tut_sky::tut_sky_pinned_step(g, r, b, f, &got),
        F::TutSkySmoke(f) => crate::script_tut_sky::tut_sky_smoke_step(g, r, b, f, &got),
        F::TutSkyBreak(f) => crate::script_tut_sky::tut_sky_break_step(g, r, b, f, &got),
        F::TutSkyOrdered(f) => crate::script_tut_sky::tut_sky_ordered_step(g, r, b, f, &got),
        F::TutSkyBlind(f) => crate::script_tut_sky::tut_sky_blind_step(g, r, b, f, &got),
        F::TutSkyPush(f) => crate::script_tut_sky::tut_sky_push_step(g, r, b, f, &got),
        F::TutSkyIndoors(f) => crate::script_tut_sky::tut_sky_indoors_step(g, r, b, f, &got),
        F::TutSkyWall(f) => crate::script_tut_sky::tut_sky_wall_step(g, r, b, f, &got),
        F::TutSkyDark(f) => crate::script_tut_sky::tut_sky_dark_step(g, r, b, f, &got),
        F::TutSkyNv(f) => crate::script_tut_sky::tut_sky_nv_step(g, r, b, f, &got),
        F::TutSkyTorches(f) => crate::script_tut_sky::tut_sky_torches_step(g, r, b, f, &got),
        F::TutSkyInside(f) => crate::script_tut_sky::tut_sky_inside_step(g, r, b, f, &got),
        F::TutSkyOut(f) => crate::script_tut_sky::tut_sky_out_step(g, r, b, f, &got),
        F::TutSoloHold(f) => crate::script_tut_solo::tut_solo_hold_step(g, r, b, f, &got),
        F::TutSoloCease(f) => crate::script_tut_solo::tut_solo_cease_step(g, r, b, f, &got),
        F::TutSoloRoad(f) => crate::script_tut_solo::tut_solo_road_step(g, r, b, f, &got),
        F::TutSoloWay(f) => crate::script_tut_solo::tut_solo_way_step(g, r, b, f, &got),
        F::TutSoloGun(f) => crate::script_tut_solo::tut_solo_gun_step(g, r, b, f, &got),
        F::TutSoloBags(f) => crate::script_tut_solo::tut_solo_bags_step(g, r, b, f, &got),
        F::TutSoloGap(f) => crate::script_tut_solo::tut_solo_gap_step(g, r, b, f, &got),
        F::TutSoloBlind(f) => crate::script_tut_solo::tut_solo_blind_step(g, r, b, f, &got),
        F::TutSoloShop(f) => crate::script_tut_solo::tut_solo_shop_step(g, r, b, f, &got),
        F::TutSoloRoom(f) => crate::script_tut_solo::tut_solo_room_step(g, r, b, f, &got),
        F::TutSoloLoose(f) => crate::script_tut_solo::tut_solo_loose_step(g, r, b, f, &got),
        F::TutSoloTrucks(f) => crate::script_tut_solo::tut_solo_trucks_step(g, r, b, f, &got),
        F::TutSquadPicket(f) => crate::script_tut_squad::tut_squad_picket_step(g, r, b, f, &got),
        F::TutSquadStore(f) => crate::script_tut_squad::tut_squad_store_step(g, r, b, f, &got),
        F::TutSquadShop(f) => crate::script_tut_squad::tut_squad_shop_step(g, r, b, f, &got),
        F::TutSquadHall(f) => crate::script_tut_squad::tut_squad_hall_step(g, r, b, f, &got),
        F::TutSquadGun(f) => crate::script_tut_squad::tut_squad_gun_step(g, r, b, f, &got),
        F::TutTeamHold(f) => crate::script_tut_team::tut_team_hold_step(g, r, b, f, &got),
        F::TutTeamCook(f) => crate::script_tut_team::tut_team_cook_step(g, r, b, f, &got),
        F::TutTeamAmbush(f) => crate::script_tut_team::tut_team_ambush_step(g, r, b, f, &got),
        F::TutTeamPinned(f) => crate::script_tut_team::tut_team_pinned_step(g, r, b, f, &got),
        F::TutTeamSmoke(f) => crate::script_tut_team::tut_team_smoke_step(g, r, b, f, &got),
        F::TutTeamBreak(f) => crate::script_tut_team::tut_team_break_step(g, r, b, f, &got),
        F::TutTeamOrdered(f) => crate::script_tut_team::tut_team_ordered_step(g, r, b, f, &got),
        F::TutTeamBlind(f) => crate::script_tut_team::tut_team_blind_step(g, r, b, f, &got),
        F::TutTeamLinger(f) => crate::script_tut_team::tut_team_linger_step(g, r, b, f, &got),
        F::TutTeamPush(f) => crate::script_tut_team::tut_team_push_step(g, r, b, f, &got),
        F::TutTeamIndoors(f) => crate::script_tut_team::tut_team_indoors_step(g, r, b, f, &got),
        F::TutTeamHouse(f) => crate::script_tut_team::tut_team_house_step(g, r, b, f, &got),
        F::TutTeamOut(f) => crate::script_tut_team::tut_team_out_step(g, r, b, f, &got),
    };
    g.tab.director_live_beat[b].it = it;   // (in the same place: the caller closes a beat that cancelled itself)
    out
}
/// INVENTED: c.test(query) at director.ts:625 for CondTest::Script: reads and may write the frame of `b` (tut-squad.script.ts:166). An Err is the TypeScript's throw out of holds, which nothing catches. Body by hand, with the nine scripts
pub fn beat_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, id: u32) -> Result<bool, String> {
    use crate::director::BeatFrame as F;
    let mut it = core::mem::replace(&mut g.tab.director_live_beat[b].it, F::Spent);
    let out = match &mut it {
        F::TutKitShop(f) => crate::script_tut_kit::tut_kit_shop_test(g, r, b, f, id),
        F::TutKitRoom(f) => crate::script_tut_kit::tut_kit_room_test(g, r, b, f, id),
        F::TutKitTrucks(f) => crate::script_tut_kit::tut_kit_trucks_test(g, r, b, f, id),
        F::TutKitBunkroom(f) => if f.sub_room.is_some() { crate::script_tut_kit::tut_kit_room_helper_test(g, r, b, f.sub_room.as_mut().unwrap(), id) } else { Err(alloc::format!("a c.until of id {} on the beat bunkroom, with no helper running", id)) },
        F::TutKitFuelroom(f) => if f.sub_room.is_some() { crate::script_tut_kit::tut_kit_room_helper_test(g, r, b, f.sub_room.as_mut().unwrap(), id) } else { Err(alloc::format!("a c.until of id {} on the beat fuelroom, with no helper running", id)) },
        F::TutKitYard(f) => crate::script_tut_kit::tut_kit_yard_test(g, r, b, f, id),
        F::TutQueueGun(f) => crate::script_tut_queue::tut_queue_gun_test(g, r, b, f, id),
        F::TutQueueGates(f) => crate::script_tut_queue::tut_queue_gates_test(g, r, b, f, id),
        F::TutQuietNight(f) => crate::script_tut_quiet::tut_quiet_night_test(g, r, b, f, id),
        F::TutRoadRoad(f) => crate::script_tut_road::tut_road_road_test(g, r, b, f, id),
        F::TutRoadHut(f) => crate::script_tut_road::tut_road_hut_test(g, r, b, f, id),
        F::TutRoadGun(f) => crate::script_tut_road::tut_road_gun_test(g, r, b, f, id),
        F::TutRoadGate(f) => crate::script_tut_road::tut_road_gate_test(g, r, b, f, id),
        F::TutSkyHold(f) => crate::script_tut_sky::tut_sky_hold_test(g, r, b, f, id),
        F::TutSkyPinned(f) => crate::script_tut_sky::tut_sky_pinned_test(g, r, b, f, id),
        F::TutSkySmoke(f) => crate::script_tut_sky::tut_sky_smoke_test(g, r, b, f, id),
        F::TutSkyBreak(f) => crate::script_tut_sky::tut_sky_break_test(g, r, b, f, id),
        F::TutSkyPush(f) => crate::script_tut_sky::tut_sky_push_test(g, r, b, f, id),
        F::TutSkyTorches(f) => crate::script_tut_sky::tut_sky_torches_test(g, r, b, f, id),
        F::TutSkyInside(f) => crate::script_tut_sky::tut_sky_inside_test(g, r, b, f, id),
        F::TutSoloRoad(f) => crate::script_tut_solo::tut_solo_road_test(g, r, b, f, id),
        F::TutSoloShop(f) => crate::script_tut_solo::tut_solo_shop_test(g, r, b, f, id),
        F::TutSoloRoom(f) => crate::script_tut_solo::tut_solo_room_test(g, r, b, f, id),
        F::TutSoloTrucks(f) => crate::script_tut_solo::tut_solo_trucks_test(g, r, b, f, id),
        F::TutSquadPicket(f) => crate::script_tut_squad::tut_squad_picket_test(g, r, b, f, id),
        F::TutSquadStore(f) => crate::script_tut_squad::tut_squad_store_test(g, r, b, f, id),
        F::TutSquadShop(f) => crate::script_tut_squad::tut_squad_shop_test(g, r, b, f, id),
        F::TutSquadHall(f) => crate::script_tut_squad::tut_squad_hall_test(g, r, b, f, id),
        F::TutSquadGun(f) => crate::script_tut_squad::tut_squad_gun_test(g, r, b, f, id),
        F::TutTeamHold(f) => crate::script_tut_team::tut_team_hold_test(g, r, b, f, id),
        F::TutTeamCook(f) => crate::script_tut_team::tut_team_cook_test(g, r, b, f, id),
        F::TutTeamPinned(f) => crate::script_tut_team::tut_team_pinned_test(g, r, b, f, id),
        F::TutTeamSmoke(f) => crate::script_tut_team::tut_team_smoke_test(g, r, b, f, id),
        F::TutTeamBreak(f) => crate::script_tut_team::tut_team_break_test(g, r, b, f, id),
        F::TutTeamLinger(f) => crate::script_tut_team::tut_team_linger_test(g, r, b, f, id),
        F::TutTeamPush(f) => crate::script_tut_team::tut_team_push_test(g, r, b, f, id),
        _ => Err(alloc::format!("a c.until of id {} on a beat with none", id)),
    };
    g.tab.director_live_beat[b].it = it;
    out
}


/// INVENTED: what q.door answers (director.ts:127, :849); `swing` is D.swing, DOORWAY.oneWay being settled true on the page
#[derive(Clone, Copy, Debug)]
pub struct QueryDoor { pub angle: f64, pub open: bool, pub latched: bool, pub locked: bool, pub broken: bool, pub swing: f64 }

pub mod util {
    #[allow(unused_imports)]
    use crate::{ids::*, js::*, Quat, V3};
    #[allow(unused_imports)]
    use alloc::{string::String, vec::Vec};
    /// (src/game/director.ts:331 UTIL.distXZ)
    pub fn dist_xz(a: V3, b: V3) -> f64 { todo!() }

    /// (src/game/director.ts:332 UTIL.lerp)
    pub fn lerp(a: V3, b: V3, t: f64) -> [f64; 3] { todo!() }

    /// (src/game/director.ts:332 UTIL.add)
    pub fn add(a: V3, b: V3) -> [f64; 3] { todo!() }

    /// (src/game/director.ts:333 UTIL.ahead)
    pub fn ahead(p: V3, yaw: f64, d: f64, y: Option<f64>) -> [f64; 3] { todo!() }

}

/// INVENTED: the define pass of the level this names (the script's `script(D)`), its locals left on the manifest
pub fn script_define(script: crate::scene_level::LevelScriptFn, m: &mut crate::director::Manifest) -> Result<(), String> {
    use crate::scene_level::LevelScriptFn as S; use crate::director::ScriptVars as V;
    m.vars = match script {
        S::Unset => return Err(String::from("a level with no script was asked to define one")),
        S::TutKit => V::TutKit(crate::script_tut_kit::tut_kit_define(m)?),
        S::TutPair => V::TutPair(crate::script_tut_pair::tut_pair_define(m)?),
        S::TutQueue => V::TutQueue(crate::script_tut_queue::tut_queue_define(m)?),
        S::TutQuiet => V::TutQuiet(crate::script_tut_quiet::tut_quiet_define(m)?),
        S::TutRoad => V::TutRoad(crate::script_tut_road::tut_road_define(m)?),
        S::TutSky => V::TutSky(crate::script_tut_sky::tut_sky_define(m)?),
        S::TutSolo => V::TutSolo(crate::script_tut_solo::tut_solo_define(m)?),
        S::TutSquad => V::TutSquad(crate::script_tut_squad::tut_squad_define(m)?),
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
        "tut-sky" => Some(S::TutSky),
        "tut-solo" => Some(S::TutSolo),
        "tut-squad" => Some(S::TutSquad),
        "tut-team" => Some(S::TutTeam),
        _ => None,
    }
}
