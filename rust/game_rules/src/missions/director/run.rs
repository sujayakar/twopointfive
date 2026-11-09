use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// `this.run === R && !R.ended`
fn live(g: &Game, r: DirectorRunId) -> bool {
    g.director.run == Some(r) && !g.tab.director_run[r].ended
}
/// `const R = this.run; R && !R.ended`: the running program
fn running(g: &Game) -> Option<DirectorRunId> {
    g.director.run.filter(|&r| !g.tab.director_run[r].ended)
}

/// A BeatFailure with no try over it: TS 626's filters and tests, 672 and 786 throw straight through Director.tick. The frame dies as it does there.
fn uncaught<T>(r: Result<T, String>) -> T {
    match r {
        Ok(v) => v,
        Err(m) => panic!("{}", m),
    }
}

// ---------------------------------------------------------------- small helpers
/// `+x.toFixed(places)`: only ever a log row's data
fn fixed_num(x: f64, places: u32) -> f64 {
    js::to_fixed(x, places).parse::<f64>().unwrap_or(x)
}
/// an object of Json
fn obj(pairs: Vec<(&str, Json)>) -> Json {
    let mut o = JsMap::new();
    for (k, v) in pairs {
        o.set(String::from(k), Some(v));
    }
    Json::Arm4(o)
}
/// `this.push({ beat, kind, what, data })`
fn note(g: &mut Game, beat: &str, kind: DirectorPushRowKind, what: &str, data: Option<Json>) {
    director_push(g, &DirectorPushRow { t: None });
}
/// `g.guards.find(x => x.char.id === id)`
fn guard_by_char_id(g: &Game, id: f64) -> Option<GuardId> {
    g.guards.iter().copied().find(|&x| g.ch[g.gd[x].ch].id == id)
}
/// `R.byName.get(name)`: names are unique in R.beats, as startBeat replaces a spent record
fn beat_by_name(g: &Game, r: DirectorRunId, name: &str) -> Option<DirectorLiveBeatId> {
    g.tab.director_run[r].beats.iter().copied().find(|&b| g.tab.director_live_beat[b].name == name)
}
/// `gd.team === 'guard' ? 'search' : 'alert'`
fn hand_back_state(g: &Game, gd: GuardId) -> crate::squad::EndGuardScriptState {
    if g.gd[gd].team == crate::teams::Team::Guard { crate::squad::EndGuardScriptState::Search } else { crate::squad::EndGuardScriptState::Alert }
}
/// the man's chest, else `up` over his feet
fn chest_of(g: &Game, c: CharId, up: f64) -> V3 {
    match g.ch[c].bones.get(crate::character::Bone::Chest) {
        Some(p) => *p,
        None => v3::add(g.ch[c].pos, [0.0, up, 0.0]),
    }
}
/// `0x2545F491 ^ (serial * 0x9E3779B9)`, or the seed asked for, as an unsigned word (TS 505)
fn seed_of(asked: Option<f64>, serial: f64) -> f64 {
    let s = match asked {
        Some(x) => x,
        None => (js::to_i32(0x2545F491u32 as f64) ^ js::to_i32(serial * (0x9E3779B9u32 as f64))) as f64,
    };
    js::to_u32(s) as f64
}

/// A level's director: its script defined and linted, armed if the lint found no error.
pub fn director_new(g: &crate::game::Game, script: Option<crate::scene_level::LevelScriptFn>) -> crate::director::Director {
    let mut d = Director::blank();
    let Some(script) = script else {
        d.manifest = None;
        return d;
    };
    let manifest = match manifest_of(script) {
        Ok(m) => m,
        Err(e) => {
            d.manifest = None;
            d.problems.push(LintProblem { level: LintProblemLevel::Error, what: format!("the script's define pass threw — {}", e) });
            return d;
        }
    };
    let level = LintManifestLevel {
        lights: g.level.lights.clone(),
        routes: g.level.routes.clone(),
        breach: g.level.breach.clone(),
        devices: g.level.devices.clone(),
        doors: g.level.doors.clone(),
        switchables: g.level.switchables.clone(),
        clear_plans: g.level.clear_plans.clone(),
        volumes: g.level.volumes.clone(),
    };
    d.problems.extend(lint_manifest(g, &manifest, &level));
    d.armed = !d.problems.iter().any(|p| p.level == LintProblemLevel::Error);
    d.manifest = Some(manifest);
    d
}

/// The script's programs by name.
pub(in crate::missions) fn director_programs(g: &crate::game::Game) -> Vec<String> {
    match &g.director.manifest {
        Some(m) => m.programs.keys(),
        None => Vec::new(),
    }
}

/// Why `name` cannot start now; None = it can.
fn director_can_start(g: &crate::game::Game, name: &str) -> Option<String> {
    let Some(manifest) = &g.director.manifest else { return Some(String::from("this level has no script")) };
    if manifest.programs.get(String::from(name)).is_none() {
        let list = manifest.programs.keys().join(", ");
        return Some(format!("no program '{}' (programs: {})", name, if list.is_empty() { "none" } else { list.as_str() }));
    }
    if !g.director.armed {
        return Some(String::from("the script is not armed (lint errors — see the console)"));
    }
    if let Some(r) = running(g) {
        return Some(format!("program '{}' is running", g.tab.director_run[r].name));
    }
    None
}

/// A tutorial mission's own program is running.
pub fn director_lesson_runs(g: &crate::game::Game) -> bool {
    running(g).is_some() && crate::rails::rails_live(g)
}

/// The level's script put the defenders' own grenades off.
pub fn director_their_frags_off(g: &crate::game::Game) -> bool {
    match &g.director.manifest {
        Some(m) if g.director.armed => m.systems.get("theirFrags") == Some(&false),
        _ => false,
    }
}

pub(in crate::missions) fn director_objective(g: &crate::game::Game) -> Option<crate::director::DirectorObjective> {
    let r = running(g)?;
    g.tab.director_run[r].objective.as_ref().map(|o| DirectorObjective { text: o.text.clone(), pos: o.pos })
}

/// A signal from outside a beat, met by the running program on its next tick.
pub(in crate::missions) fn director_signal(g: &mut crate::game::Game, name: &str, data: Option<&crate::director::DirectorSignalData>) {
    let Some(r) = running(g) else { return };
    let data = data.map(|d| match d {
        DirectorSignalData::Text(x) => Json::Text(x.clone()),
        DirectorSignalData::Num(x) => Json::Num(*x),
        DirectorSignalData::Bool(x) => Json::Bool(*x),
        DirectorSignalData::Array(x) => Json::Array(x.clone()),
        DirectorSignalData::Arm4(x) => Json::Arm4(x.clone()),
    });
    let rr = &mut g.tab.director_run[r];
    rr.signals.push(RunSignalsItem { name: String::from(name) });
    note(g, "", DirectorPushRowKind::Note, &format!("signal:{}", name), data);
}

/// Has the running program raised, or been sent, this signal.
pub(in crate::missions) fn director_signalled(g: &crate::game::Game, name: &str) -> bool {
    match running(g) {
        Some(r) => g.tab.director_run[r].signals.iter().any(|s| s.name == name),
        None => false,
    }
}

/// A row of the beat log, a ring of LOG_CAP.
pub(super) fn director_push(g: &mut crate::game::Game, row: &crate::director::DirectorPushRow) {
    let (t, run, program) = match g.director.run {
        Some(r) => {
            let rr = &g.tab.director_run[r];
            (row.t.unwrap_or(rr.t), rr.serial, rr.name.clone())
        }
        None => (row.t.unwrap_or(g.time), 0.0, String::new()),
    };
    g.director.log.push(LogRow {  });
    let n = g.director.log.len();
    let cap = LOG_CAP as usize;
    if n > cap {
        g.director.log.drain(0..n - cap);
    }
}

// ---------------------------------------------------------------- start / stop
/// Start program `name` now; the reason when refused.
pub(in crate::missions) fn director_start(g: &mut crate::game::Game, name: &str, opts: Option<crate::director::DirectorStartOpts>) -> Option<String> {
    if let Some(why) = director_can_start(g, name) {
        return Some(why);
    }
    let def = g.director.manifest.as_ref().unwrap().programs.get(String::from(name)).unwrap().clone();
    let seed = seed_of(opts.and_then(|o| o.seed), g.director.serial);
    g.director.serial += 1.0;
    let triggers = def.triggers.clone();
    let vars = g.director.manifest.as_ref().unwrap().vars.clone();
    let r = g.tab.director_run.push(Run {
        serial: g.director.serial,
        name: String::from(name),
        def,
        t: 0.0,
        dt: 0.0,
        tick_n: 0.0,
        dice: crate::dice::Die::new(js::to_u32(seed)),
        beats: Vec::new(),
        fired: JsSet::new(),
        signals: Vec::new(),
        sig_seen: 0.0,
        flags: JsMap::new(),
        units: JsMap::new(),
        bursts: Vec::new(),
        kicks: Vec::new(),
        objective: None,
        systems_saved: None,
        ended: false,
        finishing: false,
        vars,
    });
    g.director.run = Some(r);
    note(g, "", DirectorPushRowKind::Program, "start", Some(obj(vec![("seed", Json::Num(seed))])));
    director_apply_systems(g, r);
    for i in 0..g.events.len() {
        let k = g.events[i].serial;
        g.director.seen.add(k);
    }
    for (i, t) in triggers.iter().enumerate() {
        if t.kind == TriggerDefKind::Start {
            g.tab.director_run[r].fired.add(crate::Num(i as f64));
            note(g, &t.beat, DirectorPushRowKind::Trigger, "onStart", None);
            director_start_beat(g, r, &t.beat);
        }
    }
    director_resume_all(g, r, None);
    None
}

/// Stop the running program: everything it holds let go of, the log noted, onProgramEnd told. Idempotent.
pub(in crate::missions) fn director_stop(g: &mut crate::game::Game, finished: Option<bool>) {
    let finished = finished.unwrap_or(false);
    let Some(r) = running(g) else { return };
    if g.director.exiting {
        return;
    }
    g.director.exiting = true;
    let mut i = 0;
    while i < g.tab.director_run[r].beats.len() {
        let b = g.tab.director_run[r].beats[i];
        if g.tab.director_live_beat[b].state == BeatState::Running {
            cancel_beat(g, b, None);
        }
        i += 1;
    }
    let gds: Vec<GuardId> = g.tab.director_run[r].units.values().map(|u| u.gd).collect();
    for gd in gds {
        if g.gd[gd].script.is_some() && g.gd[gd].state != crate::game::GuardState::Dead {
            let st = hand_back_state(g, gd);
            crate::squad::end_guard_script(g, gd, Some(st));
        }
    }
    let touched: Vec<crate::Num> = g.tab.director_run[r].beats.iter().flat_map(|&b| g.tab.director_live_beat[b].touched.keys()).collect();
    for id in touched {
        if g.tab.director_run[r].units.has(id) {
            continue;
        }
        if let Some(gd) = guard_by_char_id(g, id.0) {
            if g.gd[gd].script.is_some() && g.gd[gd].state != crate::game::GuardState::Dead {
                let st = hand_back_state(g, gd);
                crate::squad::end_guard_script(g, gd, Some(st));
            }
        }
    }
    director_restore_systems(g, r);
    g.tab.director_run[r].ended = true;
    note(g, "", DirectorPushRowKind::Program, if finished { "finished" } else { "stopped" }, None);
    let (serial, name, t) = { let rr = &g.tab.director_run[r]; (rr.serial, rr.name.clone(), rr.t) };
    g.director.last_run = Some(DirectorLastRun {  });
    g.director.run = None;
    g.director.exiting = false;
}

/// `b.state = 'cancelled'; b.it.return(); log it`: the walk that closes a beat, in stop, the finish and the hand-over
fn cancel_beat(g: &mut Game, b: DirectorLiveBeatId, why: Option<&str>) {
    g.tab.director_live_beat[b].state = BeatState::Cancelled;
    g.tab.director_live_beat[b].it = BeatFrame::Spent;
    let (name, step) = { let lb = &g.tab.director_live_beat[b]; (lb.name.clone(), lb.step) };
    let mut data = vec![("step", Json::Num(step))];
    if let Some(w) = why {
        data.push(("why", Json::Text(String::from(w))));
    }
    note(g, &name, DirectorPushRowKind::Beat, "cancelled", Some(obj(data)));
}

/// The encounter restarted under a play program: its men are gone, the program ends where it stood.
pub fn director_on_encounter_restart(g: &mut crate::game::Game) {
    let Some(r) = running(g) else { return };
    if !g.director.exiting {
        note(g, "", DirectorPushRowKind::Program, "reset", None);
        g.tab.director_run[r].units.clear();
        director_stop(g, Some(false));
    }
}

// ---------------------------------------------------------------- the tick
/// Once a frame from Game.update, before tacticsTick.
/// (`_input`, `_cam`: the call surface's row still carries them, and nothing reads them: the host's camera and keys are the game's `host_cam` /
/// `host_input`, handed at the stand)
pub fn director_tick(g: &mut crate::game::Game, dt: f64, _input: Option<InputId>, _cam: Option<CameraId>) {
    let Some(r) = running(g) else { return };
    let rr = &mut g.tab.director_run[r];
    rr.t += dt;
    rr.dt = dt;
    rr.tick_n += 1.0;
    director_tick_bursts(g, r);
    director_tick_kicks(g, r);
    // this frame's news: hearing events not seen before, signals raised since
    let mut fresh: Vec<EventView> = Vec::new();
    let mut i = 0;
    while i < g.events.len() {
        let k = g.events[i].serial;
        if !g.director.seen.has(k) {
            g.director.seen.add(k);
            let e = g.events[i].clone();
            fresh.push(view_of(g, r, e));
        }
        i += 1;
    }
    while (g.tab.director_run[r].sig_seen as usize) < g.tab.director_run[r].signals.len() {
        let rr = &g.tab.director_run[r];
        let s = &rr.signals[rr.sig_seen as usize];
        let s = SignalViewS { name: s.name.clone() };
        fresh.push(signal_view(g, &crate::seats::here(g), r, &s));
        g.tab.director_run[r].sig_seen += 1.0;
    }
    director_fire_triggers(g, r, &fresh);
    director_resume_all(g, r, Some(&fresh));
}

/// `fresh.find(e => e.kind === t.event)`
fn trigger_event(t: &TriggerDef, fresh: &[EventView]) -> Option<EventView> {
    let kind = t.event.map(|k| k.as_str());
    fresh.iter().find(|e| Some(e.kind.as_str()) == kind).cloned()
}

fn director_fire_triggers(g: &mut crate::game::Game, r: DirectorRunId, fresh: &[crate::director::EventView]) {
    let n = g.tab.director_run[r].def.triggers.len();
    for i in 0..n {
        if g.tab.director_run[r].fired.has(crate::Num(i as f64)) {
            continue;
        }
        let t = g.tab.director_run[r].def.triggers[i].clone();
        let mut ev: Option<EventView> = None;
        let go = match t.kind {
            TriggerDefKind::Start => false,
            TriggerDefKind::After => g.tab.director_run[r].t >= t.secs.unwrap(),
            TriggerDefKind::Event => {
                ev = trigger_event(&t, fresh);
                ev.is_some()
            }
            TriggerDefKind::Signal => {
                ev = fresh.iter().find(|e| e.kind == EventViewKind::Signal && e.name == t.name).cloned();
                ev.is_some()
            }
            TriggerDefKind::Enter => {
                let who = match &t.who {
                    None | Some(TriggerDefWho::Sam) => DirectorInsideVolumeWho::Sam,
                    Some(TriggerDefWho::AnyUnit) => DirectorInsideVolumeWho::AnyUnit,
                    Some(TriggerDefWho::Unit(u)) => DirectorInsideVolumeWho::Unit(u.clone()),
                };
                director_inside_volume(g, &crate::seats::here(g), r, t.volume.as_deref().unwrap(), &who)
            }
            TriggerDefKind::When => unreachable!("P.when has no page caller: a trigger of this kind is never made"),
        };
        if !go {
            continue;
        }
        g.tab.director_run[r].fired.add(crate::Num(i as f64));
        let more = match t.kind {
            TriggerDefKind::After => format!(" {}s", t.secs.unwrap()),
            TriggerDefKind::Event => format!(" {}", t.event.map(|k| k.as_str()).unwrap_or("undefined")),
            TriggerDefKind::Signal => format!(" {}", t.name.as_deref().unwrap_or("undefined")),
            TriggerDefKind::Enter => format!(" {}", t.volume.as_deref().unwrap_or("undefined")),
            TriggerDefKind::When => format!(" {}", t.label.as_deref().unwrap_or("undefined")),
            TriggerDefKind::Start => String::new(),
        };
        let data = ev.map(|e| obj(vec![("at", Json::Array(vec![Some(Json::Num(fixed_num(e.pos[0], 2))), Some(Json::Num(0.0)), Some(Json::Num(fixed_num(e.pos[2], 2)))]))]));
        note(g, &t.beat, DirectorPushRowKind::Trigger, &format!("{}{}", t.kind.as_str(), more), data);
        director_start_beat(g, r, &t.beat);
    }
}

/// A second trigger for a running beat is a no-op; one re-started after it ended replaces its record.
pub(super) fn director_start_beat(g: &mut crate::game::Game, r: DirectorRunId, name: &str) -> Option<DirectorLiveBeatId> {
    let Some(script) = g.tab.director_run[r].def.beats.get(String::from(name)).map(|b| b.fn_) else {
        note(g, name, DirectorPushRowKind::Error, &format!("no beat '{}'", name), None);
        return None;
    };
    let cur = beat_by_name(g, r, name);
    if let Some(c) = cur {
        if g.tab.director_live_beat[c].state == BeatState::Running {
            return Some(c);
        }
    }
    let t0 = g.tab.director_run[r].t;
    let lb = g.tab.director_live_beat.push(LiveBeat {
        name: String::from(name),
        it: beat_frame_new(&script, name),
        waiting: None,
        result: CondResult { index: 0.0, event: None },
        t0,
        step: 0.0,
        state: BeatState::Running,
        touched: JsSet::new(),
        flow: LiveBeatFlow { finish: false },
    });
    if let Some(c) = cur {
        let beats = &mut g.tab.director_run[r].beats;
        if let Some(i) = beats.iter().position(|&x| x == c) {
            beats.remove(i);
        }
    }
    g.tab.director_run[r].beats.push(lb);
    note(g, name, DirectorPushRowKind::Beat, "start", None);
    Some(lb)
}

/// Resume every running beat whose condition holds, in start order, passing again while any moved. A snapshot: a beat started this pass waits for the next.
fn director_resume_all(g: &mut crate::game::Game, r: DirectorRunId, fresh: Option<&[crate::director::EventView]>) {
    let fresh = fresh.unwrap_or(&[]);
    for _pass in 0..MAX_PASSES as usize {
        let mut moved = false;
        let snapshot = g.tab.director_run[r].beats.clone();
        for b in snapshot {
            if !live(g, r) {
                break;
            }
            if g.tab.director_live_beat[b].state == BeatState::Running && director_resume(g, r, b, fresh) {
                moved = true;
            }
        }
        if !moved || !live(g, r) {
            break;
        }
    }
}

/// → did the beat advance at all.
fn director_resume(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, fresh: &[crate::director::EventView]) -> bool {
    let mut n: f64 = 0.0;
    while g.tab.director_live_beat[b].state == BeatState::Running {
        if let Some(mut c) = g.tab.director_live_beat[b].waiting.take() {
            match director_holds(g, r, &mut c, fresh, b) {
                None => {
                    g.tab.director_live_beat[b].waiting = Some(c); // (what it latched stays with it)
                    return n > 0.0;
                }
                Some(res) => g.tab.director_live_beat[b].result = res,
            }
        }
        n += 1.0;
        if n > MAX_RESUMES {
            director_fail_beat(g, r, b, &format!("resumed {} times in one tick without waiting — a runaway loop", MAX_RESUMES));
            return true;
        }
        let out = match beat_next(g, r, b) {
            Ok(out) => out,
            Err(msg) => {
                director_fail_beat(g, r, b, &msg);
                return true;
            }
        };
        if !live(g, r) {
            return true; // the beat stopped the program from inside
        }
        g.tab.director_live_beat[b].step += 1.0;
        if g.tab.director_live_beat[b].state == BeatState::Cancelled {
            g.tab.director_live_beat[b].it = BeatFrame::Spent; // it cancelled itself during that step: closed here, logged there, never also 'done'
            return true;
        }
        let name = g.tab.director_live_beat[b].name.clone();
        let Some(c) = out else {
            g.tab.director_live_beat[b].state = BeatState::Done;
            let lb = &g.tab.director_live_beat[b];
            let (step, t0, finish) = (lb.step, lb.t0, lb.flow.finish);
            let t = g.tab.director_run[r].t;
            note(g, &name, DirectorPushRowKind::Beat, "done", Some(obj(vec![("step", Json::Num(step)), ("secs", Json::Num(fixed_num(t - t0, 3)))])));
            if finish {
                director_begin_finish(g, r);
            }
            return true;
        };
        let lab = cond_label(&c);
        g.tab.director_live_beat[b].waiting = Some(c);
        if !lab.is_empty() {
            note(g, &name, DirectorPushRowKind::Yield, &lab, None);
        }
        if g.tab.director_live_beat[b].flow.finish {
            director_begin_finish(g, r);
        }
    }
    n > 0.0
}

/// A c.on's filter: the closures of levels/*.script.ts, by what each captured. (a throw from `c.query.pos` in Heard is uncaught there: TS 627)
fn filter_holds(g: &Game, r: DirectorRunId, f: &EventFilter, e: &EventView) -> bool {
    let p = e.pos;
    match f {
        EventFilter::InBox { x0, x1, z0, z1 } => p[0] > *x0 && p[0] < *x1 && p[2] > *z0 && p[2] < *z1,
        EventFilter::XAbove { x } => p[0] > *x,
        EventFilter::XBelow { x } => p[0] < *x,
        EventFilter::XZAbove { x, z } => p[0] > *x && p[2] > *z,
        EventFilter::OrderSignal => e.name.as_deref().map_or(false, |n| n.starts_with("order:")),
        EventFilter::Heard { by, run_level, run_within, walk_level, walk_within } => {
            let at = uncaught(director_query_pos_unit(g, r, by));
            let near = |within: f64| m::hyp2(at[0] - p[0], at[2] - p[2]) <= within;
            (e.level >= *run_level && near(*run_within)) || (e.level >= *walk_level && near(*walk_within))
        }
    }
}

/// Does condition `c` hold this tick: what goes back into the generator, None = keep waiting.
fn director_holds(g: &mut crate::game::Game, r: DirectorRunId, c_2: &mut crate::director::Cond, fresh: &[crate::director::EventView], b: DirectorLiveBeatId) -> Option<crate::director::CondResult> {
    let yes = |event: Option<EventView>| Some(CondResult { index: 0.0, event });
    match c_2 {
        Cond::Arm0(c) => if g.tab.director_run[r].t >= c.until { yes(None) } else { None },
        Cond::Arm1(c) => if g.tab.director_run[r].tick_n >= c.due { yes(None) } else { None },
        Cond::Arm2(c) => {
            let holds = match c.test {
                CondTest::Script { id } => uncaught(beat_test(g, r, b, id)),
                CondTest::Burst { serial } => !g.tab.director_run[r].bursts.iter().any(|x| x.serial == serial && x.left > 0.0),
            };
            if holds { yes(None) } else { None }
        }
        Cond::Arm3(c) => {
            if let Some(got) = &c.got {
                return yes(Some(got.clone()));
            }
            if g.tab.director_run[r].tick_n == c.tick0 {
                return None;
            }
            for e in fresh {
                if e.kind.as_str() != c.kind.as_str() {
                    continue;
                }
                let any_name = c.name.as_deref().map_or(true, |n| n.is_empty());
                if c.kind == CondArm3Kind::Signal && !any_name && e.name != c.name {
                    continue;
                }
                if c.filter.as_ref().map_or(true, |f| filter_holds(g, r, f, e)) {
                    c.got = Some(e.clone());
                    return yes(Some(e.clone()));
                }
            }
            None
        }
        Cond::Arm4(c) => {
            let lb = beat_by_name(g, r, &c.name)?;
            if c.state == CondArm4State::Started || g.tab.director_live_beat[lb].state != BeatState::Running { yes(None) } else { None }
        }
        Cond::Arm5(c) => {
            if c.k == CondArm5K::All {
                // EVERY branch evaluated every tick: an event branch latches what it caught
                let (mut ok, mut ev) = (true, None);
                for x in c.of.iter_mut() {
                    match director_holds(g, r, x, fresh, b) {
                        None => ok = false,
                        Some(res) => if res.event.is_some() { ev = res.event },
                    }
                }
                if ok { yes(ev) } else { None }
            } else {
                for (i, x) in c.of.iter_mut().enumerate() {
                    if let Some(res) = director_holds(g, r, x, fresh, b) {
                        return Some(CondResult { index: i as f64, event: res.event });
                    }
                }
                None
            }
        }
    }
}

fn director_fail_beat(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, e: &str) {
    let msg = e;
    g.tab.director_live_beat[b].state = BeatState::Failed;
    let (name, step) = { let lb = &g.tab.director_live_beat[b]; (lb.name.clone(), lb.step) };
    note(g, &name, DirectorPushRowKind::Error, &format!("threw at step {}: {}", step, msg), None);
    // armed men are never left frozen
    let touched = g.tab.director_live_beat[b].touched.keys();
    for id in touched {
        let gd = match g.tab.director_run[r].units.get(id) {
            Some(u) => Some(u.gd),
            None => guard_by_char_id(g, id.0),
        };
        if let Some(gd) = gd {
            if g.gd[gd].script.is_some() && g.gd[gd].state != crate::game::GuardState::Dead {
                let st = hand_back_state(g, gd);
                crate::squad::end_guard_script(g, gd, Some(st));
            }
        }
    }
}
fn director_begin_finish(g: &mut crate::game::Game, r: DirectorRunId) {
    let rr = &mut g.tab.director_run[r];
    if rr.finishing || rr.ended {
        return;
    }
    rr.finishing = true;
    note(g, "", DirectorPushRowKind::Program, "finish", None);
    director_stop(g, Some(true));
}

/// Only what the script set is kept, so only that is put back: a setting the player changed under the program stays his.
fn director_apply_systems(g: &mut crate::game::Game, r: DirectorRunId) {
    let m = &g.director.manifest.as_ref().unwrap().systems;
    let (qrf, needs, civilians, autosave) = (m.get("qrf").copied(), m.get("needs").copied(), m.get("civilians").copied(), m.get("autosave").copied());
    if qrf.is_none() && needs.is_none() && civilians.is_none() && autosave.is_none() {
        return;
    }
    let mut saved: JsMap<&'static str, bool> = JsMap::new();
    if qrf.is_some() { saved.set("qrf", g.tune.qrf); }
    if needs.is_some() { saved.set("needs", g.tune.needs); }
    if civilians.is_some() { saved.set("civilians", g.tune.civilians); }
    if autosave.is_some() { saved.set("autosave", g.saves.enabled); }
    g.tab.director_run[r].systems_saved = Some(saved);
    if let Some(x) = qrf { g.tune.qrf = x; }
    if let Some(x) = needs { g.tune.needs = x; }
    if let Some(x) = civilians { g.tune.civilians = x; }
    if let Some(x) = autosave { g.saves.enabled = x; }
}

fn director_restore_systems(g: &mut crate::game::Game, r: DirectorRunId) {
    let Some(s) = g.tab.director_run[r].systems_saved.take() else { return };
    if let Some(&x) = s.get("qrf") { g.tune.qrf = x; }
    if let Some(&x) = s.get("needs") { g.tune.needs = x; }
    if let Some(&x) = s.get("civilians") { g.tune.civilians = x; }
    if let Some(&x) = s.get("autosave") { g.saves.enabled = x; }
}

// ---------------------------------------------------------------- men in the picture through the fog; bursts; tracks; kicks
/// a man the program spawned, or one of the level's own it took in hand
fn unit_guard(g: &Game, r: DirectorRunId, id: f64) -> Option<GuardId> {
    match g.tab.director_run[r].units.get(crate::Num(id)) {
        Some(u) => Some(u.gd),
        None => guard_by_char_id(g, id),
    }
}

/// A scripted round is a REAL round: hits, suppression, marks, heat, glass, hearing all follow from combat.ts.
fn director_tick_bursts(g: &mut crate::game::Game, r: DirectorRunId) {
    let mut i = 0;
    while i < g.tab.director_run[r].bursts.len() {
        let k = i;
        i += 1;
        let b = g.tab.director_run[r].bursts[k].clone();
        if b.left <= 0.0 {
            continue;
        }
        let gd = unit_guard(g, r, b.u.id).filter(|&x| g.gd[x].state != crate::game::GuardState::Dead);
        let Some(gd) = gd else {
            g.tab.director_run[r].bursts[k].left = 0.0;
            continue;
        };
        let weapon = g.gd[gd].weapon;
        if g.tab.director_run[r].t < b.next || crate::weapons::firearm_reloading(g, weapon) {
            continue;
        }
        let at: Option<V3> = match &b.at {
            RunBurstsItemAt::Unit(u) => unit_guard(g, r, u.id).map(|v| chest_of(g, g.gd[v].ch, 1.1)),
            RunBurstsItemAt::List(v) => Some([v[0], js::or(v[1], 1.1), v[2]]),
            RunBurstsItemAt::Mark(mk) => {
                let p = uncaught(director_place_of(g, &Place::Mark(mk.clone())));
                Some([p[0], 1.1, p[2]]) // (a mark has no [1]: `|| 1.1`)
            }
        };
        let Some(at) = at else {
            g.tab.director_run[r].bursts[k].left = 0.0;
            continue;
        };
        if crate::weapons::firearm_fire(g, weapon) {
            let c = g.gd[gd].ch;
            crate::combat::fire_weapon(g, c, at, false, Some(&crate::combat::FireOpts { steady: Some(true), spread: Some(b.spread), ..Default::default() }));
            let rr = &mut g.tab.director_run[r];
            let t = rr.t;
            let row = &mut rr.bursts[k];
            row.left -= 1.0;
            row.fired += 1.0;
            row.next = t + row.cadence;
        } else {
            crate::weapons::firearm_rearm(g, weapon);
        }
        let row = &g.tab.director_run[r].bursts[k];
        if row.left <= 0.0 {
            let data = obj(vec![("by", Json::Text(row.u.callsign.clone())), ("rounds", Json::Num(row.fired))]);
            note(g, "", DirectorPushRowKind::Stimulus, "burst:done", Some(data));
        }
    }
    // the finished leave the list: a long program fires hundreds
    g.tab.director_run[r].bursts.retain(|b| b.left > 0.0);
}

/// Kicks in the air: on the animator's impact frame (or the instant the clip is somehow over) doors.ts kickIn does the boot's work from where he stands.
fn director_tick_kicks(g: &mut crate::game::Game, r: DirectorRunId) {
    if g.tab.director_run[r].kicks.is_empty() {
        return;
    }
    let kicks = core::mem::take(&mut g.tab.director_run[r].kicks);
    let mut kept = Vec::new();
    for k in kicks {
        if g.gd[k.gd].state == crate::game::GuardState::Dead {
            continue;
        }
        let c = g.gd[k.gd].ch;
        let landed = crate::character::anim_kick_impact(g, c) || crate::character::anim_kick_time(g, c) >= crate::character::KICK_IMPACT || !crate::character::anim_kicking(g, c);
        if !landed {
            kept.push(k);
            continue;
        }
        let d = k.door;
        let (pos, id) = (g.ch[c].pos, g.ch[c].id);
        crate::doors::doors_kick_in(g, d, pos, id, None);
        let pos = g.ch[c].pos;
        crate::marks::kick_scuff(g, d, pos);
        let what = format!("kick {}", g.dr[d].def.name);
        let angle = crate::doors::door_angle(g, d);
        note(g, &k.beat, DirectorPushRowKind::Stimulus, &what, Some(obj(vec![("by", Json::Text(k.by.clone())), ("angle", Json::Num(fixed_num(angle, 2)))])));
    }
    g.tab.director_run[r].kicks = kept;
}

// ---------------------------------------------------------------- resolving handles
pub(super) fn director_place_of(g: &crate::game::Game, p: &crate::director::Place) -> Result<V3, String> {
    let mk = match p {
        Place::List(v) => return Ok(*v),
        Place::Mark(mk) => mk,
    };
    if let Some(lm) = g.level.marks.as_ref().and_then(|l| l.iter().find(|x| x.name == mk.id)) {
        return Ok(lm.pos);
    }
    if let Some(pos) = g.director.manifest.as_ref().and_then(|m| m.marks.get(mk.id.clone())).and_then(|x| x.pos) {
        return Ok(pos);
    }
    Err(format!("mark '{}' does not resolve", mk.id))
}

pub(super) fn director_door_of(g: &crate::game::Game, d: &crate::director::DoorRef) -> Result<DoorId, String> {
    crate::doors::doors_by_name(g, &d.id).ok_or_else(|| format!("door '{}' does not resolve", d.id))
}

pub(super) fn director_guard_of(g: &crate::game::Game, r: DirectorRunId, u: &crate::director::UnitRef) -> Result<GuardId, String> {
    let rr = &g.tab.director_run[r];
    if let Some(rec) = rr.units.get(crate::Num(u.id)) {
        return Ok(rec.gd);
    }
    if let Some(gd) = guard_by_char_id(g, u.id) {
        return Ok(gd);
    }
    Err(format!("unit {} (#{}) is not in the world", u.callsign, u.id))
}

/// World to the volume's local frame with the boxes' own convention: feet anywhere from its floor to head height inside count.
fn in_volume(v: &crate::scene_level::VolumeDef, p: V3) -> bool {
    let (px, pz) = (p[0] - v.c[0], p[2] - v.c[2]);
    let (cs, sn) = (m::cos(v.yaw.unwrap_or(0.0)), m::sin(v.yaw.unwrap_or(0.0)));
    let (lx, lz) = (cs * px - sn * pz, sn * px + cs * pz);
    m::abs(lx) <= v.h[0] && m::abs(lz) <= v.h[2] && p[1] >= v.c[1] - v.h[1] - 0.1 && p[1] <= v.c[1] + v.h[1] + 2.0
}
pub(in crate::missions) fn director_inside_volume(g: &crate::game::Game, seat: &crate::seats::SeatRef, r: DirectorRunId, id_2: &str, who: &crate::director::DirectorInsideVolumeWho) -> bool {
    let g = &*g;
    let Some(v) = g.level.volumes.as_ref().and_then(|l| l.iter().find(|x| x.name == id_2)) else { return false };
    let pl = &g.pl[seat.player];
    let alive = |gd: GuardId| g.gd[gd].state != crate::game::GuardState::Dead;
    let sam = !pl.down && in_volume(v, g.ch[pl.ch].pos);
    let units = &g.tab.director_run[r].units;
    match who {
        DirectorInsideVolumeWho::Sam => sam,
        DirectorInsideVolumeWho::Team => sam || g.guards.iter().any(|&gd| g.gd[gd].team == crate::teams::Team::Operative && alive(gd) && in_volume(v, g.ch[g.gd[gd].ch].pos)),
        DirectorInsideVolumeWho::AnyUnit => units.values().any(|u| alive(u.gd) && in_volume(v, g.ch[g.gd[u.gd].ch].pos)),
        DirectorInsideVolumeWho::Unit(u) => units.get(crate::Num(u.id)).map_or(false, |x| alive(x.gd) && in_volume(v, g.ch[g.gd[x.gd].ch].pos)),
    }
}

// The Query's methods: closures in the TypeScript (TS 845-856), with the skeleton's names: a beat calls them.
pub(in crate::missions) fn director_query_alive(g: &crate::game::Game, r: DirectorRunId, u: &crate::director::UnitRef) -> bool {
    unit_guard(g, r, u.id).map_or(false, |gd| g.gd[gd].state != crate::game::GuardState::Dead)
}
pub(in crate::missions) fn director_query_arrived(g: &crate::game::Game, r: DirectorRunId, u: &crate::director::UnitRef) -> Result<bool, String> {
    let gd = director_guard_of(g, r, u)?;
    Ok(g.gd[gd].script.as_ref().map_or(false, |s| s.arrived.unwrap_or(false)))
}
pub(in crate::missions) fn director_query_state(g: &crate::game::Game, r: DirectorRunId, u: &crate::director::UnitRef) -> Option<crate::game::GuardState> {
    director_guard_of(g, r, u).ok().map(|gd| g.gd[gd].state)
}
pub(in crate::missions) fn director_query_pos_unit(g: &crate::game::Game, r: DirectorRunId, u: &crate::director::UnitRef) -> Result<V3, String> {
    let gd = director_guard_of(g, r, u)?;
    Ok(v3::copy(g.ch[g.gd[gd].ch].pos))
}
/// (director.ts Query.door; DOORWAY.oneWay is settled on: swing is the door's)
pub(in crate::missions) fn director_query_door(g: &crate::game::Game, d: &crate::director::DoorRef) -> Result<crate::director::QueryDoor, String> {
    let id = director_door_of(g, d)?;
    Ok(QueryDoor {
        angle: crate::doors::door_angle(g, id),
        open: !crate::doors::door_is_closed(g, id),
        latched: crate::doors::door_latched(g, id),
        locked: crate::doors::door_locked(g, id),
        broken: crate::doors::door_lock_broken(g, id),
        swing: crate::doors::door_swing(g, id),
    })
}
pub(in crate::missions) fn director_query_signalled(g: &crate::game::Game, r: DirectorRunId, n: &str) -> bool {
    g.tab.director_run[r].signals.iter().any(|s| s.name == n)
}
pub(in crate::missions) fn director_query_routed(g: &crate::game::Game, r: DirectorRunId, u: &crate::director::UnitRef) -> bool {
    match director_guard_of(g, r, u) {
        Ok(gd) => crate::owner::has(g, gd, crate::owner::by::RUNNER),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// A script's dice are its own run's: the seed asked for is used, as an unsigned word; asked for none, each run has another and the same run the same.
    #[test]
    fn a_run_takes_the_seed_asked_for_and_otherwise_one_of_its_own() {
        assert_eq!(seed_of(Some(5.0), 9.0), 5.0);
        assert_eq!(seed_of(Some(-1.0), 0.0), 4294967295.0);
        assert_eq!(seed_of(None, 3.0), seed_of(None, 3.0));
        let seeds: alloc::vec::Vec<f64> = [0.0, 1.0, 2.0, 3.0].iter().map(|s| seed_of(None, *s)).collect();
        for (i, a) in seeds.iter().enumerate() {
            assert!(*a >= 0.0 && *a <= 4294967295.0 && *a == m::floor(*a), "{a}");
            for b in &seeds[i + 1..] { assert_ne!(a, b); }
        }
    }
    /// A FAILURE MEANS: a level's trigger fires where nobody is, or not where he stands: a volume is a box turned on its yaw, and a man is in it with his feet from just under its floor to a head's height over its top.
    #[test]
    fn a_man_is_in_a_turned_volume_from_its_floor_to_a_head_over_its_top() {
        let v = crate::scene_level::VolumeDef { c: [1.0, 0.5, 2.0], h: [3.0, 0.5, 1.0], yaw: Some(0.7), ..crate::scene_level::VolumeDef::default() };
        for (p, want) in VOLUME_CASES {
            assert_eq!(in_volume(&v, *p), *want, "{:?}", p);
        }
    }
    const VOLUME_CASES: &[(V3, bool)] = &[
        ([1.0, 0.0, 2.0], true),       // at its middle, on its floor
        ([1.0, -0.2, 2.0], false),     // under the floor
        ([1.0, 2.9, 2.0], true),       // a head over the top
        ([1.0, 3.1, 2.0], false),      // past that
        ([3.2, 0.0, 0.2], true),       // along the long side, turned: inside
        ([3.0, 0.0, 3.5], false),      // where the same side would lie unturned
        ([-1.0, 0.0, 3.2], true),      // the other end, turned
        ([4.1, 0.0, 2.0], false),      // inside it unturned, outside it turned
    ];
}
