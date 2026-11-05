#![allow(unused_variables)]
//! Every function and method of `director`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/director.ts:275 liftedBudget)
pub fn lifted_budget(g: &mut crate::game::Game) -> Option<f64> { todo!() }

/// (src/game/director.ts:291 manifestOf)
pub fn manifest_of(script: &crate::scene_level::LevelScriptFn) -> crate::director::Manifest { todo!() }

/// (src/game/director.ts:338 lintManifest)
pub fn lint_manifest(g: &mut crate::game::Game, m: &crate::director::Manifest, level: &crate::director::LintManifestLevel) -> Vec<crate::director::LintProblem> { todo!() }

/// (src/game/director.ts:379 scanShowcaseVerbs)
pub fn scan_showcase_verbs(m: &crate::director::Manifest) -> Vec<crate::director::ScanShowcaseVerbsItem> { todo!() }

/// (src/game/director.ts:439 Director.injectInput)
pub fn director_inject_input(g: &mut crate::game::Game, input: InputId) { todo!() }

/// (src/game/director.ts:440 Director.constructor)
pub fn director_new(g: &mut crate::game::Game, script: Option<&crate::scene_level::LevelScriptFn>) -> crate::director::Director { todo!() }

/// (src/game/director.ts:450 Director.report)
pub fn director_report(g: &mut crate::game::Game) { todo!() }

/// (src/game/director.ts:452 Director.attach)
pub fn director_attach(g: &mut crate::game::Game, o: &crate::director::DirectorAttachO) { todo!() }

/// (src/game/director.ts:453 Director.programs)
pub fn director_programs(g: &mut crate::game::Game) -> Vec<String> { todo!() }

/// (src/game/director.ts:455 Director.showing)
pub fn director_showing(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/director.ts:456 Director.lowerThird)
pub fn director_lower_third(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/director.ts:458 Director.handedOver)
pub fn director_handed_over(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/director.ts:460 Director.canStart)
pub fn director_can_start(g: &mut crate::game::Game, name: &str) -> Option<String> { todo!() }

/// (src/game/director.ts:472 Director.running)
pub fn director_running(g: &mut crate::game::Game) -> Option<String> { todo!() }

/// (src/game/director.ts:474 Director.lessonRuns)
pub fn director_lesson_runs(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/director.ts:476 Director.theirFragsOff)
pub fn director_their_frags_off(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/director.ts:477 Director.objective)
pub fn director_objective(g: &mut crate::game::Game) -> Option<crate::director::DirectorObjective> { todo!() }

/// (src/game/director.ts:479 Director.signal)
pub fn director_signal(g: &mut crate::game::Game, name: &str, data: Option<&crate::director::DirectorSignalData>) { todo!() }

/// (src/game/director.ts:481 Director.signalled)
pub fn director_signalled(g: &mut crate::game::Game, name: &str) -> bool { todo!() }

/// (src/game/director.ts:482 Director.chapters)
pub fn director_chapters(g: &mut crate::game::Game) -> Option<crate::director::DirectorChapters> { todo!() }

/// (src/game/director.ts:486 Director.summary)
pub fn director_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/director.ts:493 Director.push)
pub fn director_push(g: &mut crate::game::Game, row: &crate::director::DirectorPushRow) { todo!() }

/// (src/game/director.ts:498 Director.rowsOf)
pub fn director_rows_of(g: &mut crate::game::Game, serial: Option<f64>) -> Vec<crate::director::LogRow> { todo!() }

/// (src/game/director.ts:502 Director.start)
pub fn director_start(g: &mut crate::game::Game, name: &str, opts: Option<crate::director::DirectorStartOpts>) -> Option<String> { todo!() }

/// (src/game/director.ts:518 Director.stop)
pub fn director_stop(g: &mut crate::game::Game, finished: Option<bool>) { todo!() }

/// (src/game/director.ts:541 Director.onEncounterRestart)
pub fn director_on_encounter_restart(g: &mut crate::game::Game) { todo!() }

/// (src/game/director.ts:545 Director.tick)
pub fn director_tick(g: &mut crate::game::Game, dt: f64, input: Option<InputId>) { todo!() }

/// (src/game/director.ts:561 Director.fireTriggers)
pub fn director_fire_triggers(g: &mut crate::game::Game, r: &mut crate::director::Run, fresh: &[crate::director::EventView]) { todo!() }

/// (src/game/director.ts:581 Director.startBeat)
pub fn director_start_beat(g: &mut crate::game::Game, r: &mut crate::director::Run, name: &str) -> Option<crate::director::LiveBeat> { todo!() }

/// (src/game/director.ts:594 Director.resumeAll)
pub fn director_resume_all(g: &mut crate::game::Game, r: &mut crate::director::Run, fresh: Option<&[crate::director::EventView]>) { todo!() }

/// (src/game/director.ts:602 Director.resume)
pub fn director_resume(g: &mut crate::game::Game, r: &mut crate::director::Run, b: &mut crate::director::LiveBeat, fresh: &[crate::director::EventView]) -> bool { todo!() }

/// (src/game/director.ts:621 Director.holds)
pub fn director_holds(g: &mut crate::game::Game, r: &mut crate::director::Run, c_2: &mut crate::director::Cond, fresh: &[crate::director::EventView]) -> Option<crate::director::CondResult> { todo!() }

/// (src/game/director.ts:633 Director.failBeat)
pub fn director_fail_beat(g: &mut crate::game::Game, r: &mut crate::director::Run, b: &mut crate::director::LiveBeat, e: crate::Opaque) { todo!() }

/// (src/game/director.ts:643 Director.beginFinish)
pub fn director_begin_finish(g: &mut crate::game::Game, r: &mut crate::director::Run) { todo!() }

/// (src/game/director.ts:650 Director.tail)
pub fn director_tail(g: &mut crate::game::Game, r: &crate::director::Run, dt: f64) { todo!() }

/// (src/game/director.ts:663 Director.frameShot)
pub fn director_frame_shot(g: &mut crate::game::Game, s: &mut crate::director::ShowState, shot: &crate::director::Shot, dt: f64) { todo!() }

/// (src/game/director.ts:671 Director.lookOf)
pub fn director_look_of(g: &mut crate::game::Game, s: &crate::director::CameraShotSpec) -> V3 { todo!() }

/// (src/game/director.ts:674 Director.enterShow)
pub fn director_enter_show(g: &mut crate::game::Game, r: &mut crate::director::Run) { todo!() }

/// (src/game/director.ts:688 Director.exitShow)
pub fn director_exit_show(g: &mut crate::game::Game, r: &mut crate::director::Run) { todo!() }

/// (src/game/director.ts:711 Director.restoreWeather)
pub fn director_restore_weather(g: &mut crate::game::Game, s: &mut crate::director::ShowState) { todo!() }

/// (src/game/director.ts:713 Director.restoreChaos)
pub fn director_restore_chaos(g: &mut crate::game::Game, s: &mut crate::director::ShowState) { todo!() }

/// (src/game/director.ts:717 Director.handOver)
pub fn director_hand_over(g: &mut crate::game::Game, r: &mut crate::director::Run, caption: &str, sub: &str, caller: Option<&crate::director::LiveBeat>) { todo!() }

/// (src/game/director.ts:738 Director.applySystems)
pub fn director_apply_systems(g: &mut crate::game::Game, r: &mut crate::director::Run) { todo!() }

/// (src/game/director.ts:744 Director.restoreSystems)
pub fn director_restore_systems(g: &mut crate::game::Game, r: &mut crate::director::Run) { todo!() }

/// (src/game/director.ts:747 Director.tickDrill)
pub fn director_tick_drill(g: &mut crate::game::Game, r: &crate::director::Run, d: &mut crate::director::DrillRun, dt: f64) { todo!() }

/// (src/game/director.ts:767 Director.endDrill)
pub fn director_end_drill(g: &mut crate::game::Game, r: &crate::director::Run, d: &mut crate::director::DrillRun) { todo!() }

/// (src/game/director.ts:777 Director.showMan)
pub fn director_show_man(g: &mut crate::game::Game, gd: GuardId, on: bool) { todo!() }

/// (src/game/director.ts:778 Director.tickShown)
pub fn director_tick_shown(g: &mut crate::game::Game) { todo!() }

/// (src/game/director.ts:779 Director.tickBursts)
pub fn director_tick_bursts(g: &mut crate::game::Game, r: &mut crate::director::Run) { todo!() }

/// (src/game/director.ts:795 Director.tickTracks)
pub fn director_tick_tracks(g: &mut crate::game::Game, r: &mut crate::director::Run, dt: f64) { todo!() }

/// (src/game/director.ts:801 Director.tickKicks)
pub fn director_tick_kicks(g: &mut crate::game::Game, r: &mut crate::director::Run) { todo!() }

/// (src/game/director.ts:814 Director.placeOf)
pub fn director_place_of(g: &mut crate::game::Game, p: &crate::director::Place) -> V3 { todo!() }

/// (src/game/director.ts:820 Director.doorOf)
pub fn director_door_of(g: &mut crate::game::Game, d: &crate::director::DoorRef) -> DoorId { todo!() }

/// (src/game/director.ts:821 Director.guardOf)
pub fn director_guard_of(g: &mut crate::game::Game, r: &crate::director::Run, u: &crate::director::UnitRef) -> GuardId { todo!() }

/// (src/game/director.ts:827 Director.planOf)
pub fn director_plan_of(g: &mut crate::game::Game, p: &crate::director::PlanRef) -> crate::director::DirectorPlanOf { todo!() }

/// (src/game/director.ts:831 Director.insideVolume)
pub fn director_inside_volume(g: &mut crate::game::Game, r: &crate::director::Run, id_2: &str, who: &crate::director::DirectorInsideVolumeWho) -> bool { todo!() }

/// (src/game/director.ts:839 Director.queryOf)
pub fn director_query_of(g: &mut crate::game::Game, r: &mut crate::director::Run) -> crate::director::Query { todo!() }

/// (src/game/director.ts:860 Director.ctxOf)
pub fn director_ctx_of(g: &mut crate::game::Game, r: &mut crate::director::Run, b: &mut crate::director::LiveBeat) -> crate::director::BeatCtx { todo!() }

/// (src/game/director.ts:1081 condLabel)
pub fn cond_label(c_2: &crate::director::Cond) -> String { todo!() }

/// (src/game/director.ts:1091 nearOf)
pub fn near_of(g: &mut crate::game::Game, pos: V3) { todo!() }

/// (src/game/director.ts:1099 samViewOf)
pub fn sam_view_of(g: &mut crate::game::Game) -> crate::director::SamView { todo!() }

/// (src/game/director.ts:1114 viewOf)
pub fn view_of(g: &mut crate::game::Game, r: &crate::director::Run, e: crate::game::GameEvent) -> crate::director::EventView { todo!() }

/// (src/game/director.ts:1115 signalView)
pub fn signal_view(g: &mut crate::game::Game, r: &crate::director::Run, s: &crate::director::SignalViewS) -> crate::director::EventView { todo!() }

pub mod util {
    #[allow(unused_imports)]
    use crate::{ids::*, js::*, Quat, V3};
    #[allow(unused_imports)]
    use alloc::{string::String, vec::Vec};
    /// (src/game/director.ts:330 UTIL.yawTo)
    pub fn yaw_to(a: V3, b: V3) -> f64 { todo!() }

    /// (src/game/director.ts:330 UTIL.distXZ)
    pub fn dist_xz(a: V3, b: V3) -> f64 { todo!() }

    /// (src/game/director.ts:331 UTIL.lerp)
    pub fn lerp(a: V3, b: V3, t: f64) -> [f64; 3] { todo!() }

    /// (src/game/director.ts:331 UTIL.add)
    pub fn add(a: V3, b: V3) -> [f64; 3] { todo!() }

    /// (src/game/director.ts:332 UTIL.ahead)
    pub fn ahead(p: V3, yaw: f64, d: f64, y: Option<f64>) -> [f64; 3] { todo!() }

}
