#![allow(unused_variables)]
//! Every function and method of `director`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/director.ts:276 liftedBudget)
pub fn lifted_budget(g: &mut crate::game::Game) -> Option<f64> { todo!() }

/// (src/game/director.ts:292 manifestOf)
pub fn manifest_of(script: crate::scene_level::LevelScriptFn) -> Result<crate::director::Manifest, String> { todo!() }

/// (src/game/director.ts:339 lintManifest)
pub fn lint_manifest(g: &mut crate::game::Game, m: &crate::director::Manifest, level: &crate::director::LintManifestLevel) -> Vec<crate::director::LintProblem> { todo!() }

/// (src/game/director.ts:440 Director.injectInput)
pub fn director_inject_input(g: &mut crate::game::Game, input: InputId) { todo!() }

/// (src/game/director.ts:441 Director.constructor)
pub fn director_new(g: &mut crate::game::Game, script: Option<crate::scene_level::LevelScriptFn>) -> crate::director::Director { todo!() }

/// (src/game/director.ts:451 Director.report)
pub fn director_report(g: &mut crate::game::Game) { todo!() }

/// (src/game/director.ts:453 Director.attach)
pub fn director_attach(g: &mut crate::game::Game, o: crate::director::DirectorAttachO) { todo!() }

/// (src/game/director.ts:454 Director.programs)
pub fn director_programs(g: &mut crate::game::Game) -> Vec<String> { todo!() }

/// (src/game/director.ts:456 Director.showing)
pub fn director_showing(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/director.ts:457 Director.lowerThird)
pub fn director_lower_third(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/director.ts:459 Director.handedOver)
pub fn director_handed_over(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/director.ts:461 Director.canStart)
pub fn director_can_start(g: &mut crate::game::Game, name: &str) -> Option<String> { todo!() }

/// (src/game/director.ts:473 Director.running)
pub fn director_running(g: &mut crate::game::Game) -> Option<String> { todo!() }

/// (src/game/director.ts:475 Director.lessonRuns)
pub fn director_lesson_runs(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/director.ts:477 Director.theirFragsOff)
pub fn director_their_frags_off(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/director.ts:478 Director.objective)
pub fn director_objective(g: &mut crate::game::Game) -> Option<crate::director::DirectorObjective> { todo!() }

/// (src/game/director.ts:480 Director.signal)
pub fn director_signal(g: &mut crate::game::Game, name: &str, data: Option<&crate::director::DirectorSignalData>) { todo!() }

/// (src/game/director.ts:482 Director.signalled)
pub fn director_signalled(g: &mut crate::game::Game, name: &str) -> bool { todo!() }

/// (src/game/director.ts:483 Director.chapters)
pub fn director_chapters(g: &mut crate::game::Game) -> Option<crate::director::DirectorChapters> { todo!() }

/// (src/game/director.ts:487 Director.summary)
pub fn director_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/director.ts:494 Director.push)
pub fn director_push(g: &mut crate::game::Game, row: &crate::director::DirectorPushRow) { todo!() }

/// (src/game/director.ts:499 Director.rowsOf)
pub fn director_rows_of(g: &mut crate::game::Game, serial: Option<f64>) -> Vec<crate::director::LogRow> { todo!() }

/// (src/game/director.ts:503 Director.start)
pub fn director_start(g: &mut crate::game::Game, name: &str, opts: Option<crate::director::DirectorStartOpts>) -> Option<String> { todo!() }

/// (src/game/director.ts:519 Director.stop)
pub fn director_stop(g: &mut crate::game::Game, finished: Option<bool>) { todo!() }

/// (src/game/director.ts:542 Director.onEncounterRestart)
pub fn director_on_encounter_restart(g: &mut crate::game::Game) { todo!() }

/// (src/game/director.ts:546 Director.tick)
pub fn director_tick(g: &mut crate::game::Game, dt: f64, input: Option<InputId>, cam: Option<CameraId>) { todo!() }

/// (src/game/director.ts:562 Director.fireTriggers)
pub fn director_fire_triggers(g: &mut crate::game::Game, r: DirectorRunId, fresh: &[crate::director::EventView]) { todo!() }

/// (src/game/director.ts:582 Director.startBeat)
pub fn director_start_beat(g: &mut crate::game::Game, r: DirectorRunId, name: &str) -> Option<DirectorLiveBeatId> { todo!() }

/// (src/game/director.ts:595 Director.resumeAll)
pub fn director_resume_all(g: &mut crate::game::Game, r: DirectorRunId, fresh: Option<&[crate::director::EventView]>) { todo!() }

/// (src/game/director.ts:603 Director.resume)
pub fn director_resume(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, fresh: &[crate::director::EventView]) -> bool { todo!() }

/// (src/game/director.ts:622 Director.holds)
pub fn director_holds(g: &mut crate::game::Game, r: DirectorRunId, c_2: &mut crate::director::Cond, fresh: &[crate::director::EventView], b: DirectorLiveBeatId) -> Option<crate::director::CondResult> { todo!() }

/// (src/game/director.ts:634 Director.failBeat)
pub fn director_fail_beat(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, e: &str) { todo!() }

/// (src/game/director.ts:644 Director.beginFinish)
pub fn director_begin_finish(g: &mut crate::game::Game, r: DirectorRunId) { todo!() }

/// (src/game/director.ts:651 Director.tail)
pub fn director_tail(g: &mut crate::game::Game, r: DirectorRunId, dt: f64) { todo!() }

/// (src/game/director.ts:664 Director.frameShot)
pub fn director_frame_shot(g: &mut crate::game::Game, s_key: DirectorRunId, shot: &crate::director::Shot, dt: f64) { todo!() }

/// (src/game/director.ts:672 Director.lookOf)
pub fn director_look_of(g: &mut crate::game::Game, s: &crate::director::CameraShotSpec) -> V3 { todo!() }

/// (src/game/director.ts:675 Director.enterShow)
pub fn director_enter_show(g: &mut crate::game::Game, r: DirectorRunId) { todo!() }

/// (src/game/director.ts:689 Director.exitShow)
pub fn director_exit_show(g: &mut crate::game::Game, r: DirectorRunId) { todo!() }

/// (src/game/director.ts:712 Director.restoreWeather)
pub fn director_restore_weather(g: &mut crate::game::Game, s_key: DirectorRunId) { todo!() }

/// (src/game/director.ts:714 Director.restoreChaos)
pub fn director_restore_chaos(g: &mut crate::game::Game, s_key: DirectorRunId) { todo!() }

/// (src/game/director.ts:718 Director.handOver)
pub fn director_hand_over(g: &mut crate::game::Game, r: DirectorRunId, caption: &str, sub: &str, caller: Option<DirectorLiveBeatId>) { todo!() }

/// (src/game/director.ts:739 Director.applySystems)
pub fn director_apply_systems(g: &mut crate::game::Game, r: DirectorRunId) { todo!() }

/// (src/game/director.ts:745 Director.restoreSystems)
pub fn director_restore_systems(g: &mut crate::game::Game, r: DirectorRunId) { todo!() }

/// (src/game/director.ts:748 Director.tickDrill)
pub fn director_tick_drill(g: &mut crate::game::Game, r: DirectorRunId, d: &mut crate::director::DrillRun, dt: f64) { todo!() }

/// (src/game/director.ts:768 Director.endDrill)
pub fn director_end_drill(g: &mut crate::game::Game, r: DirectorRunId, d: &mut crate::director::DrillRun) { todo!() }

/// (src/game/director.ts:778 Director.showMan)
pub fn director_show_man(g: &mut crate::game::Game, gd: GuardId, on: bool) { todo!() }

/// (src/game/director.ts:779 Director.tickShown)
pub fn director_tick_shown(g: &mut crate::game::Game) { todo!() }

/// (src/game/director.ts:780 Director.tickBursts)
pub fn director_tick_bursts(g: &mut crate::game::Game, r: DirectorRunId) { todo!() }

/// (src/game/director.ts:796 Director.tickTracks)
pub fn director_tick_tracks(g: &mut crate::game::Game, r: DirectorRunId, dt: f64) { todo!() }

/// (src/game/director.ts:802 Director.tickKicks)
pub fn director_tick_kicks(g: &mut crate::game::Game, r: DirectorRunId) { todo!() }

/// (src/game/director.ts:815 Director.placeOf)
pub fn director_place_of(g: &mut crate::game::Game, p: &crate::director::Place) -> Result<V3, String> { todo!() }

/// (src/game/director.ts:821 Director.doorOf)
pub fn director_door_of(g: &mut crate::game::Game, d: &crate::director::DoorRef) -> Result<DoorId, String> { todo!() }

/// (src/game/director.ts:822 Director.guardOf)
pub fn director_guard_of(g: &mut crate::game::Game, r: DirectorRunId, u: &crate::director::UnitRef) -> Result<GuardId, String> { todo!() }

/// (src/game/director.ts:828 Director.planOf)
pub fn director_plan_of(g: &mut crate::game::Game, p: &crate::director::PlanRef) -> Result<crate::director::DirectorPlanOf, String> { todo!() }

/// (src/game/director.ts:832 Director.insideVolume)
pub fn director_inside_volume(g: &mut crate::game::Game, r: DirectorRunId, id_2: &str, who: &crate::director::DirectorInsideVolumeWho) -> bool { todo!() }

/// (src/game/director.ts:840 Director.queryOf)
pub fn director_query_of(g: &mut crate::game::Game, r: DirectorRunId) -> crate::director::Query { todo!() }

/// (src/game/director.ts:861 Director.ctxOf)
pub fn director_ctx_of(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId) -> crate::director::BeatCtx { todo!() }

/// (src/game/director.ts:1082 condLabel)
pub fn cond_label(c_2: &crate::director::Cond) -> String { todo!() }

/// (src/game/director.ts:1092 nearOf)
pub fn near_of(g: &mut crate::game::Game, pos: V3, what: &crate::director::NearWhat, within: f64) { todo!() }

/// (src/game/director.ts:1100 samViewOf)
pub fn sam_view_of(g: &mut crate::game::Game) -> crate::director::SamView { todo!() }

/// (src/game/director.ts:1115 viewOf)
pub fn view_of(g: &mut crate::game::Game, r: DirectorRunId, e: crate::game::GameEvent) -> crate::director::EventView { todo!() }

/// (src/game/director.ts:1116 signalView)
pub fn signal_view(g: &mut crate::game::Game, r: DirectorRunId, s: &crate::director::SignalViewS) -> crate::director::EventView { todo!() }

/// INVENTED (director.ts:324 D.systems): a system the level turns on or off
pub fn manifest_system(m: &mut crate::director::Manifest, name: &'static str, on: bool) { m.systems.set(name, on); }
/// INVENTED (director.ts:854 q.flag, :899 c.flag): a flag of this run
pub fn run_flag(g: &crate::game::Game, r: DirectorRunId, name: &str) -> Option<crate::director::Json> { g.tab.director_run[r].flags.get(String::from(name)).cloned().flatten() }
/// INVENTED (director.ts:899 c.flag): a flag of this run set
pub fn run_set_flag(g: &mut crate::game::Game, r: DirectorRunId, name: &str, v: crate::director::Json) { g.tab.director_run[r].flags.set(String::from(name), Some(v)); }

// ---- what the by-hand scripts call: the verbs of a level script, which are closures of the TypeScript's D, P, c and q objects. No TypeScript function has these names.
/// INVENTED: D.mark, define time, no Game (director.ts:294): a named place; a mark named again keeps its pos unless given one
pub fn manifest_mark(m: &mut crate::director::Manifest, id: &str, pos: Option<V3>, yaw: Option<f64>) -> crate::director::MarkRef { todo!() }
/// INVENTED: D.volume (director.ts:295)
pub fn manifest_volume(m: &mut crate::director::Manifest, id: &str) -> crate::director::VolumeRef { todo!() }
/// INVENTED: D.door (director.ts:297)
pub fn manifest_door(m: &mut crate::director::Manifest, id: &str) -> crate::director::DoorRef { todo!() }
/// INVENTED: D.loadout (director.ts:300)
pub fn manifest_loadout(m: &mut crate::director::Manifest, id: &str, spec: crate::director::LoadoutSpec) -> crate::director::LoadoutRef { todo!() }
/// INVENTED: D.program (director.ts:303-317): Err = declared twice, or the body's Err; the row goes in m.programs after the body
pub fn manifest_program(m: &mut crate::director::Manifest, name: &str, body: &mut dyn FnMut(&mut crate::director::ProgramDef) -> Result<(), String>, opts: Option<crate::director::ProgramOpts>) -> Result<(), String> { todo!() }
/// INVENTED: P.onStart (director.ts:307)
pub fn program_on_start(p: &mut crate::director::ProgramDef, beat: &str) { todo!() }
/// INVENTED: P.onSignal (director.ts:310)
pub fn program_on_signal(p: &mut crate::director::ProgramDef, sig: &str, beat: &str) { todo!() }
/// INVENTED: P.beat (director.ts:313): `script` is whose match holds this beat's code; Err = declared twice
pub fn program_beat(p: &mut crate::director::ProgramDef, name: &str, script: &crate::scene_level::LevelScriptFn, atomic: Option<bool>) -> Result<(), String> { todo!() }

/// INVENTED: B.fn(ctx) at director.ts:586: the beat's frame before its first step. Body by hand, with the nine scripts
pub fn beat_frame_new(script: &crate::scene_level::LevelScriptFn, beat: &str) -> crate::director::BeatFrame {
    use crate::scene_level::LevelScriptFn as S; use crate::director::BeatFrame as F;
    match (script, beat) {
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

/// INVENTED: c.wait (director.ts:887): Err = not a finite number of seconds
pub fn director_beat_wait(g: &mut crate::game::Game, r: DirectorRunId, secs: f64, label: Option<&str>) -> Result<crate::director::Cond, String> { todo!() }
/// INVENTED: c.frames (director.ts:888)
pub fn director_beat_frames(g: &mut crate::game::Game, r: DirectorRunId, n: Option<f64>) -> crate::director::Cond { todo!() }
/// INVENTED: c.until (director.ts:889)
pub fn director_beat_until(test: crate::director::CondTest, label: Option<&str>) -> crate::director::Cond { todo!() }
/// INVENTED: c.on (director.ts:890), its filterOrName already apart
pub fn director_beat_on(g: &mut crate::game::Game, r: DirectorRunId, kind: crate::director::CondArm3Kind, name: Option<&str>, filter: Option<crate::director::EventFilter>) -> crate::director::Cond { todo!() }
/// INVENTED: c.all (director.ts:893)
pub fn director_beat_all(of: Vec<crate::director::Cond>) -> crate::director::Cond { todo!() }
/// INVENTED: c.any (director.ts:893)
pub fn director_beat_any(of: Vec<crate::director::Cond>) -> crate::director::Cond { todo!() }
/// INVENTED: c.start (director.ts:895)
pub fn director_beat_start(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, name: &str) { todo!() }
/// INVENTED: c.signal (director.ts:897)
pub fn director_beat_signal(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, name: &str, data: Option<crate::director::Json>) { todo!() }
/// INVENTED: c.note (director.ts:899)
pub fn director_beat_note(g: &mut crate::game::Game, b: DirectorLiveBeatId, what: &str, data: Option<crate::director::Json>) { todo!() }
/// INVENTED: c.rand (director.ts:901): the run's own die
pub fn director_beat_rand(g: &mut crate::game::Game, r: DirectorRunId) -> f64 { todo!() }
/// INVENTED: c.spawn (director.ts:905)
pub fn director_beat_spawn(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, l: &crate::director::LoadoutRef, at: &crate::director::Place, callsign: Option<&str>, yaw: Option<f64>, light: Option<bool>) -> Result<crate::director::UnitRef, String> { todo!() }
/// INVENTED: c.unit (director.ts:916)
pub fn director_beat_unit(g: &mut crate::game::Game, r: DirectorRunId, callsign: &str) -> Option<crate::director::UnitRef> { todo!() }
/// INVENTED: c.order (director.ts:865)
pub fn director_beat_order(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &crate::director::UnitRef, o: &crate::director::OrderPatch2) -> Result<(), String> { todo!() }
/// INVENTED: c.release (director.ts:929)
pub fn director_beat_release(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &crate::director::UnitRef, state: Option<crate::squad::EndGuardScriptState>) -> Result<(), String> { todo!() }
/// INVENTED: c.rout (director.ts:921)
pub fn director_beat_rout(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &crate::director::UnitRef) -> Result<bool, String> { todo!() }
/// INVENTED: c.show (director.ts:920)
pub fn director_beat_show(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &crate::director::UnitRef, on: Option<bool>) -> Result<(), String> { todo!() }
/// INVENTED: c.say of a man (director.ts:961); the 'sam' branch is a showcase's
pub fn director_beat_say(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &crate::director::UnitRef, line: &str, radio: Option<bool>) -> Result<(), String> { todo!() }
/// INVENTED: c.radio (director.ts:970)
pub fn director_beat_radio(g: &mut crate::game::Game, b: DirectorLiveBeatId, callsign: &str, line: &str) { todo!() }
/// INVENTED: c.doors.lock (director.ts:984)
pub fn director_beat_door_lock(g: &mut crate::game::Game, d: &crate::director::DoorRef, on: bool) -> Result<(), String> { todo!() }
/// INVENTED: c.doors.open (director.ts:985)
pub fn director_beat_door_open(g: &mut crate::game::Game, d: &crate::director::DoorRef, angle: Option<f64>, hold: Option<bool>) -> Result<(), String> { todo!() }
/// INVENTED: c.doors.shut (director.ts:987)
pub fn director_beat_door_shut(g: &mut crate::game::Game, d: &crate::director::DoorRef) -> Result<(), String> { todo!() }
/// INVENTED: c.stimulus.charge (director.ts:999)
pub fn director_beat_charge(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, at: &crate::director::Place, by: &crate::director::UnitRef, kind: Option<crate::throwables::ItemKind>, fuse: Option<f64>, toss: Option<bool>) -> Result<(), String> { todo!() }
/// INVENTED: c.stimulus.burst (director.ts:1008): deals RunBurstsItem.serial with g.serial() and returns CondTest::Burst of it
pub fn director_beat_burst(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &crate::director::UnitRef, at: crate::director::RunBurstsItemAt, rounds: f64, cadence: Option<f64>, spread: Option<f64>) -> Result<crate::director::Cond, String> { todo!() }
/// INVENTED: c.stimulus.cease (director.ts:1007)
pub fn director_beat_cease(g: &mut crate::game::Game, r: DirectorRunId, u: &crate::director::UnitRef) { todo!() }

/// INVENTED: what q.door answers (director.ts:127, :849); `swing` is D.swing, DOORWAY.oneWay being settled true on the page
#[derive(Clone, Copy, Debug)]
pub struct QueryDoor { pub angle: f64, pub open: bool, pub latched: bool, pub locked: bool, pub broken: bool, pub swing: f64 }
/// INVENTED: q.alive (director.ts:844)
pub fn director_query_alive(g: &mut crate::game::Game, r: DirectorRunId, u: &crate::director::UnitRef) -> bool { todo!() }
/// INVENTED: q.arrived (director.ts:845)
pub fn director_query_arrived(g: &mut crate::game::Game, r: DirectorRunId, u: &crate::director::UnitRef) -> Result<bool, String> { todo!() }
/// INVENTED: q.state (director.ts:846): a failed guardOf is None
pub fn director_query_state(g: &mut crate::game::Game, r: DirectorRunId, u: &crate::director::UnitRef) -> Option<crate::game::GuardState> { todo!() }
/// INVENTED: q.room('sam') (director.ts:847)
pub fn director_query_room_sam(g: &mut crate::game::Game) -> String { todo!() }
/// INVENTED: q.room of a man (director.ts:847)
pub fn director_query_room_unit(g: &mut crate::game::Game, r: DirectorRunId, u: &crate::director::UnitRef) -> Result<String, String> { todo!() }
/// INVENTED: q.pos of a man (director.ts:848)
pub fn director_query_pos_unit(g: &mut crate::game::Game, r: DirectorRunId, u: &crate::director::UnitRef) -> Result<V3, String> { todo!() }
/// INVENTED: q.pos of a place (director.ts:848)
pub fn director_query_pos_place(g: &mut crate::game::Game, p: &crate::director::Place) -> Result<V3, String> { todo!() }
/// INVENTED: q.door (director.ts:849)
pub fn director_query_door(g: &mut crate::game::Game, d: &crate::director::DoorRef) -> Result<crate::director::QueryDoor, String> { todo!() }
/// INVENTED: q.signalled (director.ts:854)
pub fn director_query_signalled(g: &mut crate::game::Game, r: DirectorRunId, n: &str) -> bool { todo!() }
/// INVENTED: q.routed (director.ts:855): a failed guardOf is false
pub fn director_query_routed(g: &mut crate::game::Game, r: DirectorRunId, u: &crate::director::UnitRef) -> bool { todo!() }

pub mod util {
    #[allow(unused_imports)]
    use crate::{ids::*, js::*, Quat, V3};
    #[allow(unused_imports)]
    use alloc::{string::String, vec::Vec};
    /// (src/game/director.ts:331 UTIL.yawTo)
    pub fn yaw_to(a: V3, b: V3) -> f64 { todo!() }

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
