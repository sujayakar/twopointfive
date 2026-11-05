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
pub fn beat_frame_new(script: &crate::scene_level::LevelScriptFn, beat: &str) -> crate::director::BeatFrame { todo!() }
/// INVENTED: it.next(b.result) at director.ts:608: one step, reading g.tab.director_live_beat[b].result. Err = it threw (resume fails the beat), Ok(None) = it returned, Ok(Some) = it yielded. Body by hand, with the nine scripts
pub fn beat_next(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId) -> Result<Option<crate::director::Cond>, String> { todo!() }
/// INVENTED: c.test(query) at director.ts:625 for CondTest::Script: reads and may write the frame of `b` (tut-squad.script.ts:166). An Err is the TypeScript's throw out of holds, which nothing catches. Body by hand, with the nine scripts
pub fn beat_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, id: u32) -> Result<bool, String> { todo!() }

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
