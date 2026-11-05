//! src/game/director.ts lines 424 to 857
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// THE RUN'S HOME. `Run` lives at `g.director.run` and stays there for a whole tick: forty files read `g.director.showing` and friends in the
// middle of one, and a beat can reach `director_stop` with no `r` in hand. So the private workers below go by `g` alone, and a stub that takes
// `r: &mut Run` lends it back to `g` for the call (`lend`). `this.run === R && !R.ended` is `live(g, serial)`. See notes.md, Decisions.

macro_rules! run {
    ($g:expr) => {
        $g.director.run.as_mut().unwrap()
    };
}

/// `this.run === R && !R.ended`, with R named by its serial
fn live(g: &Game, serial: f64) -> bool {
    match &g.director.run {
        Some(x) => x.serial == serial && !x.ended,
        None => false,
    }
}

/// `r` came out of `g.director.run`: back home for the call, and out again after. Stopped meanwhile: it comes back ended, and empty.
fn lend<T>(g: &mut Game, r: &mut Run, f: impl FnOnce(&mut Game) -> T) -> T {
    let serial = r.serial;
    g.director.run = Some(core::mem::replace(r, Run::zeroed()));
    let out = f(g);
    let home = matches!(&g.director.run, Some(x) if x.serial == serial);
    if home {
        *r = g.director.run.take().unwrap();
    } else {
        r.serial = serial;
        r.ended = true;
    }
    out
}

/// the reverse, for a stub of another part that takes `r` and only builds something (no reader of the run can run under it)
fn with_run_out<T>(g: &mut Game, f: impl FnOnce(&mut Game, &mut Run) -> T) -> T {
    let mut r = g.director.run.take().unwrap();
    let out = f(g, &mut r);
    g.director.run = Some(r);
    out
}

// ---------------------------------------------------------------- seams: each is one blocked step (notes.md, Skeleton)
/// TS 586: the script is a closure and a beat a generator: `B.fn(ctx)`
fn seam_beat_begin(g: &mut Game, name: &str, ctx: &BeatCtx) -> crate::Opaque { todo!("needs: ProgramDefBeatsValue.fn_ as something callable, LiveBeat.it as a resumable beat") }
/// TS 608: `b.it.next(b.result)`. Err = it threw; None = it ended; Some = what it yielded. The beat is the row of that name in R.beats
fn seam_beat_next(g: &mut Game, name: &str, result: &CondResult) -> Result<Option<Cond>, String> { todo!("needs: LiveBeat.it as a resumable beat") }
/// TS 523,612,646,720: `try { b.it.return(); } catch {}`
fn seam_beat_return(g: &mut Game, name: &str) { todo!("needs: LiveBeat.it as a resumable beat") }
/// TS 597: a LiveBeat remembered by identity across a pass
fn seam_beat_key(b: &LiveBeat) -> u32 { todo!("needs: LiveBeat.serial, or an ID") }
/// TS 467: `/\.\s*spawn\s*\(/.test(String(B.fn))`, a function's own source read
fn seam_beat_spawns(g: &mut Game, program: &str, beat: &str) -> bool { todo!("needs: ProgramDefBeatsValue.spawns, worked out at definition") }
/// TS 625: `c.test(q)`
fn seam_cond_test(g: &mut Game, c: &CondArm2, q: Query) -> bool { todo!("needs: CondArm2.test as an enum, with the beat's frame") }
/// TS 671: `typeof s.look === 'function' ? s.look() : this.placeOf(s.look)`
fn seam_shot_look(g: &mut Game, s: &CameraShotSpec) -> V3 { todo!("needs: CameraShotSpec.look, dropped from the skeleton") }
/// TS 764: `I.e.done`, `I.e.result`: the live engagement, which guards.ts holds
fn seam_engagement(g: &mut Game, e: &crate::guards::Engagement) -> (bool, Option<crate::guards::EngageResult>) { todo!("needs: an EngagementId") }
/// TS 635: `e.message` of an unknown thing
fn seam_error_message(e: crate::Opaque) -> String { todo!("needs: director_fail_beat's e as a &str") }
/// TS 818,820,823,825,828,829: `throw new BeatFailure(msg)` from a stub that returns the value
fn seam_beat_throw(msg: &str) -> ! { todo!("needs: place_of, door_of, guard_of, plan_of returning Result") }

// ---------------------------------------------------------------- small helpers
/// `+x.toFixed(places)`: only ever a log row's data
fn fixed_num(x: f64, places: usize) -> f64 {
    format!("{:.*}", places, x).parse::<f64>().unwrap_or(x)
}
/// an object of Json
fn obj(pairs: Vec<(&str, Json)>) -> Json {
    let mut o = JsMap::new();
    for (k, v) in pairs {
        o.set(String::from(k), Some(v));
    }
    Json::Arm4(o)
}
fn log_kind(k: DirectorPushRowKind) -> LogRowKind {
    match k {
        DirectorPushRowKind::Camera => LogRowKind::Camera,
        DirectorPushRowKind::Radio => LogRowKind::Radio,
        DirectorPushRowKind::Error => LogRowKind::Error,
        DirectorPushRowKind::Spawn => LogRowKind::Spawn,
        DirectorPushRowKind::Beat => LogRowKind::Beat,
        DirectorPushRowKind::Drill => LogRowKind::Drill,
        DirectorPushRowKind::Note => LogRowKind::Note,
        DirectorPushRowKind::Program => LogRowKind::Program,
        DirectorPushRowKind::Trigger => LogRowKind::Trigger,
        DirectorPushRowKind::Yield => LogRowKind::Yield,
        DirectorPushRowKind::Stimulus => LogRowKind::Stimulus,
    }
}
/// `this.push({ beat, kind, what, data })`
fn note(g: &mut Game, beat: &str, kind: DirectorPushRowKind, what: &str, data: Option<Json>) {
    director_push(g, &DirectorPushRow { kind, what: String::from(what), data, beat: String::from(beat), t: None });
}
/// `g.guards.find(x => x.char.id === id)`
fn guard_by_char_id(g: &Game, id: f64) -> Option<GuardId> {
    g.guards.iter().copied().find(|&x| g.ch[g.gd[x].ch].id == id)
}
/// where a beat of that name stands in R.beats (names are unique there: startBeat replaces a spent record)
fn beat_i(g: &Game, name: &str) -> Option<usize> {
    g.director.run.as_ref().and_then(|r| r.beats.iter().position(|b| b.name == name))
}
/// a beat's state, on its one record: R.beats
fn set_beat_state(g: &mut Game, i: usize, st: BeatState) {
    run!(g).beats[i].state = st;
}
/// `gd.team === 'guard' ? 'search' : 'alert'`
fn hand_back_state(g: &Game, gd: GuardId) -> crate::squad::EndGuardScriptState {
    if g.gd[gd].team == crate::teams::Team::Guard { crate::squad::EndGuardScriptState::Search } else { crate::squad::EndGuardScriptState::Alert }
}
fn run_ref(g: &Game) -> &Run {
    g.director.run.as_ref().unwrap()
}
/// the man's chest, else `up` over his feet
fn chest_of(g: &Game, c: CharId, up: f64) -> V3 {
    match g.ch[c].bones.get(crate::character::Bone::Chest) {
        Some(p) => *p,
        None => v3::add(g.ch[c].pos, [0.0, up, 0.0]),
    }
}
fn show_on(g: &Game) -> bool {
    run_ref(g).show.is_some()
}
/// `0x2545F491 ^ (serial * 0x9E3779B9)`, or the seed asked for, as an unsigned word (TS 505)
fn seed_of(asked: Option<f64>, serial: f64) -> f64 {
    let s = match asked {
        Some(x) => x,
        None => (js::to_i32(0x2545F491u32 as f64) ^ js::to_i32(serial * (0x9E3779B9u32 as f64))) as f64,
    };
    js::to_u32(s) as f64
}

// ---------------------------------------------------------------- what the host and other files ask
/// A showcase beat's queued keys become this frame's. (director.ts Director.injectInput)
pub fn director_inject_input(g: &mut crate::game::Game, input: InputId) {
    if g.director.due_keys.is_empty() && !js::truthy(g.director.due_clicks) {
        return;
    }
    let keys = g.director.due_keys.keys();
    let clicks = g.director.due_clicks;
    let i = &mut g.tab.input[input];
    for k in keys {
        i.pressed.add(k);
    }
    i.clicked = (js::to_i32(i.clicked) | js::to_i32(clicks)) as f64;
    g.director.due_keys.clear();
    g.director.due_clicks = 0.0;
}

/// A level's director: its script defined and linted, armed if the lint found no error. (director.ts Director.constructor)
pub fn director_new(g: &mut crate::game::Game, script: Option<&crate::scene_level::LevelScriptFn>) -> crate::director::Director {
    let mut d = Director::blank();
    let Some(script) = script else {
        d.manifest = None;
        return d;
    };
    let manifest = manifest_of(script);
    let level = LintManifestLevel {
        lights: g.level.lights.clone(),
        routes: g.level.routes.clone(),
        breach: g.level.breach.clone(),
        devices: g.level.devices.clone(),
        doors: g.level.doors.clone(),
        switchables: g.level.switchables.clone(),
        clear_plans: g.level.clear_plans.clone(),
        daylight: g.level.daylight.clone(),
        volumes: g.level.volumes.clone(),
    };
    d.problems.extend(lint_manifest(g, &manifest, &level));
    for h in scan_showcase_verbs(&manifest) {
        d.scan.push(DirectorScanItem { program: h.program, beat: h.beat, verb: h.verb });
    }
    for h in &d.scan {
        d.problems.push(LintProblem { level: LintProblemLevel::Error, what: format!("program '{}' is not a showcase but beat '{}' calls c.{}(…) — captions, camera and puppet are showcase-only", h.program, h.beat, h.verb) });
    }
    d.armed = !d.problems.iter().any(|p| p.level == LintProblemLevel::Error);
    d.manifest = Some(manifest);
    d
}

/// The lint's errors on the console: nothing else, so nothing here. (director.ts Director.report)
pub fn director_report(g: &mut crate::game::Game) {}

/// Hand the director the camera and the input. (director.ts Director.attach)
pub fn director_attach(g: &mut crate::game::Game, o: &crate::director::DirectorAttachO) {
    if o.cam.is_some() {
        g.director.cam = o.cam.clone();
    }
    if o.input.is_some() {
        g.director.input = o.input.clone();
    }
}

/// The script's programs by name. (director.ts Director.programs)
pub fn director_programs(g: &mut crate::game::Game) -> Vec<String> {
    match &g.director.manifest {
        Some(m) => m.programs.keys(),
        None => Vec::new(),
    }
}

/// Is a showcase holding the frame. (director.ts Director.showing)
pub fn director_showing(g: &mut crate::game::Game) -> bool {
    match &g.director.run {
        Some(r) => r.show.is_some() && !r.ended,
        None => false,
    }
}

/// (director.ts Director.lowerThird)
pub fn director_lower_third(g: &mut crate::game::Game) -> bool {
    if !director_showing(g) {
        return false;
    }
    let r = run_ref(g);
    r.def.opts.lower_third.unwrap_or(false) && !r.show.as_ref().unwrap().handed_over
}

/// Has the running showcase handed the encounter to the player. (director.ts Director.handedOver)
pub fn director_handed_over(g: &mut crate::game::Game) -> bool {
    director_showing(g) && run_ref(g).show.as_ref().unwrap().handed_over
}

/// Why `name` cannot start now; None = it can. (director.ts Director.canStart)
pub fn director_can_start(g: &mut crate::game::Game, name: &str) -> Option<String> {
    let Some(manifest) = &g.director.manifest else { return Some(String::from("this level has no script")) };
    let Some(p) = manifest.programs.get(String::from(name)) else {
        let list = manifest.programs.keys().join(", ");
        return Some(format!("no program '{}' (programs: {})", name, if list.is_empty() { "none" } else { list.as_str() }));
    };
    let showcase = p.opts.showcase.unwrap_or(false);
    let has_loadouts = manifest.loadouts.len() > 0;
    let beats = p.beats.keys();
    if !g.director.armed {
        return Some(String::from("the script is not armed (lint errors — see the console)"));
    }
    if let Some(r) = &g.director.run {
        if !r.ended {
            return Some(format!("program '{}' is running", r.name));
        }
    }
    if showcase && g.touring {
        return Some(String::from("the tour or attract mode has the floor"));
    }
    if showcase && (g.qrf.state != crate::qrf::QrfState::Idle || g.guards.iter().any(|&x| g.gd[x].unit.is_some())) {
        return Some(String::from("a response unit is on the floor (the cast would not fit the character budget)"));
    }
    if showcase && !g.tune.teams && has_loadouts {
        for b in &beats {
            if seam_beat_spawns(g, name, b) {
                return Some(String::from("sides are switched off (tune.teams) — the cast cannot be spawned"));
            }
        }
    }
    None
}

/// The running program's name. (director.ts Director.running)
pub fn director_running(g: &mut crate::game::Game) -> Option<String> {
    match &g.director.run {
        Some(r) if !r.ended => Some(r.name.clone()),
        _ => None,
    }
}

/// A tutorial mission's own program is running. (director.ts Director.lessonRuns)
pub fn director_lesson_runs(g: &mut crate::game::Game) -> bool {
    let on = matches!(&g.director.run, Some(r) if !r.ended);
    on && crate::rails::rails_live(g)
}

/// The level's script put the defenders' own grenades off. (director.ts Director.theirFragsOff)
pub fn director_their_frags_off(g: &mut crate::game::Game) -> bool {
    match &g.director.manifest {
        Some(m) if g.director.armed => m.systems.get("theirFrags") == Some(&false),
        _ => false,
    }
}

/// (director.ts Director.objective)
pub fn director_objective(g: &mut crate::game::Game) -> Option<crate::director::DirectorObjective> {
    match &g.director.run {
        Some(r) if !r.ended => r.objective.as_ref().map(|o| DirectorObjective { text: o.text.clone(), pos: o.pos }),
        _ => None,
    }
}

/// A signal from outside a beat, met by the running program on its next tick. (director.ts Director.signal)
pub fn director_signal(g: &mut crate::game::Game, name: &str, data: Option<&crate::director::DirectorSignalData>) {
    let on = matches!(&g.director.run, Some(r) if !r.ended);
    if !on {
        return;
    }
    let data = data.map(|d| match d {
        DirectorSignalData::Text(x) => Json::Text(x.clone()),
        DirectorSignalData::Num(x) => Json::Num(*x),
        DirectorSignalData::Bool(x) => Json::Bool(*x),
        DirectorSignalData::Array(x) => Json::Array(x.clone()),
        DirectorSignalData::Arm4(x) => Json::Arm4(x.clone()),
    });
    let r = run!(g);
    let t = r.t;
    r.signals.push(RunSignalsItem { name: String::from(name), t, data: data.clone() });
    note(g, "", DirectorPushRowKind::Note, &format!("signal:{}", name), data);
}

/// Has the running program raised, or been sent, this signal. (director.ts Director.signalled)
pub fn director_signalled(g: &mut crate::game::Game, name: &str) -> bool {
    match &g.director.run {
        Some(r) if !r.ended => r.signals.iter().any(|s| s.name == name),
        _ => false,
    }
}

/// (director.ts Director.chapters)
pub fn director_chapters(g: &mut crate::game::Game) -> Option<crate::director::DirectorChapters> {
    let r = g.director.run.as_ref()?;
    let s = r.show.as_ref()?;
    if s.chapters.is_empty() || s.handed_over {
        return None;
    }
    let cur = if s.shot_i >= 0.0 { s.chapters.get(s.shot_i as usize) } else { None };
    let frac = match cur {
        Some(c) if s.shot.is_some() => m::min(1.0, (r.t - s.chapter_t0) / c.dur),
        _ => 0.0,
    };
    let list = s.chapters.iter().map(|c| DirectorChaptersListItem { title: c.title.clone(), dur: c.dur }).collect();
    Some(DirectorChapters { list, i: s.shot_i, frac })
}

/// (director.ts Director.summary)
pub fn director_summary(g: &mut crate::game::Game) -> String {
    let Some(manifest) = &g.director.manifest else { return String::from("director: no script") };
    let programs = manifest.programs.keys().join("/");
    let mut tail = String::new();
    match (&g.director.run, &g.director.last_run) {
        (Some(r), _) if !r.ended => {
            let beats: Vec<String> = r.beats.iter().map(|b| {
                let wait = match &b.waiting {
                    Some(c) if b.state == BeatState::Running => format!("({})", cond_label(c)),
                    _ => String::new(),
                };
                format!("{}:{}{}", b.name, b.state.as_str(), wait)
            }).collect();
            // (R.drills is always empty on the page: the line's `drills ` stands with nothing after it)
            tail = format!(" · RUNNING {} t={:.1} beats {} units {} drills ", r.name, r.t, beats.join(" "), r.units.len());
        }
        (_, Some(l)) => {
            tail = format!(" · last {} {} at {:.1} s", l.name, if l.finished { "finished" } else { "stopped" }, l.t);
        }
        _ => {}
    }
    format!("director: {} · programs {}{} · log {}", if g.director.armed { "armed" } else { "NOT armed" }, programs, tail, g.director.log.len())
}

/// A row of the beat log, a ring of LOG_CAP. (director.ts Director.push)
pub fn director_push(g: &mut crate::game::Game, row: &crate::director::DirectorPushRow) {
    let (t, run, program) = match &g.director.run {
        Some(r) => (row.t.unwrap_or(r.t), r.serial, r.name.clone()),
        None => (row.t.unwrap_or(g.time), 0.0, String::new()),
    };
    g.director.log.push(LogRow { t, run, program, beat: row.beat.clone(), kind: log_kind(row.kind), what: row.what.clone(), data: row.data.clone() });
    let n = g.director.log.len();
    let cap = LOG_CAP as usize;
    if n > cap {
        g.director.log.drain(0..n - cap);
    }
}

/// The rows of one run: by default the current, else the last. (director.ts Director.rowsOf)
pub fn director_rows_of(g: &mut crate::game::Game, serial: Option<f64>) -> Vec<crate::director::LogRow> {
    let serial = match serial {
        Some(s) => s,
        None => match (&g.director.run, &g.director.last_run) {
            (Some(r), _) => r.serial,
            (None, Some(l)) => l.serial,
            (None, None) => -1.0,
        },
    };
    g.director.log.iter().filter(|r| r.run == serial).cloned().collect()
}

// ---------------------------------------------------------------- start / stop
/// Start program `name` now; the reason when refused. (director.ts Director.start)
pub fn director_start(g: &mut crate::game::Game, name: &str, opts: Option<crate::director::DirectorStartOpts>) -> Option<String> {
    if let Some(why) = director_can_start(g, name) {
        return Some(why);
    }
    let def = g.director.manifest.as_ref().unwrap().programs.get(String::from(name)).unwrap().clone();
    let seed = seed_of(opts.and_then(|o| o.seed), g.director.serial);
    g.director.serial += 1.0;
    let showcase = def.opts.showcase.unwrap_or(false);
    let triggers = def.triggers.clone();
    g.director.run = Some(Run {
        serial: g.director.serial,
        name: String::from(name),
        def,
        t: 0.0,
        dt: 0.0,
        tick_n: 0.0,
        dice: crate::dice::Die::new(js::to_u32(seed)),
        seed,
        beats: Vec::new(),
        fired: JsSet::new(),
        signals: Vec::new(),
        sig_seen: 0.0,
        flags: JsMap::new(),
        units: JsMap::new(),
        bursts: Vec::new(),
        kicks: Vec::new(),
        tracks: Vec::new(),
        show: None,
        objective: None,
        systems_saved: None,
        ended: false,
        finishing: false,
        failed_show: false,
    });
    g.director.frame = None;
    note(g, "", DirectorPushRowKind::Program, "start", Some(obj(vec![("seed", Json::Num(seed))])));
    if showcase { enter_show(g) } else { apply_systems(g) }
    for i in 0..g.events.len() {
        let k = g.events[i].serial;
        g.director.seen.add(k);
    }
    for (i, t) in triggers.iter().enumerate() {
        if t.kind == TriggerDefKind::Start {
            run!(g).fired.add(crate::Num(i as f64));
            note(g, &t.beat, DirectorPushRowKind::Trigger, "onStart", None);
            start_beat(g, &t.beat);
        }
    }
    resume_all(g, &[]);
    tail(g, 0.0);
    None
}

/// Stop the running program: everything it holds let go of, the log noted, onProgramEnd told. Idempotent. (director.ts Director.stop)
pub fn director_stop(g: &mut crate::game::Game, finished: Option<bool>) {
    let finished = finished.unwrap_or(false);
    let on = matches!(&g.director.run, Some(r) if !r.ended);
    if !on || g.director.exiting {
        return;
    }
    g.director.exiting = true;
    for gd in g.director.shown.keys() {
        director_show_man(g, gd, false);
    }
    let mut i = 0;
    while i < run_ref(g).beats.len() {
        if run_ref(g).beats[i].state == BeatState::Running {
            cancel_beat(g, i, None);
        }
        i += 1;
    }
    if show_on(g) {
        let gds: Vec<GuardId> = run_ref(g).units.values().map(|u| u.gd).collect();
        for gd in gds {
            crate::guards::despawn_unit(g, gd);
        }
        run!(g).units.clear();
        exit_show(g);
    } else {
        let gds: Vec<GuardId> = run_ref(g).units.values().map(|u| u.gd).collect();
        for gd in gds {
            if g.gd[gd].script.is_some() && g.gd[gd].state != crate::game::GuardState::Dead {
                let st = hand_back_state(g, gd);
                crate::squad::end_guard_script(g, gd, Some(st));
            }
        }
        let touched: Vec<crate::Num> = run_ref(g).beats.iter().flat_map(|b| b.touched.keys()).collect();
        for id in touched {
            if run_ref(g).units.has(id) {
                continue;
            }
            if let Some(gd) = guard_by_char_id(g, id.0) {
                if g.gd[gd].script.is_some() && g.gd[gd].state != crate::game::GuardState::Dead {
                    let st = hand_back_state(g, gd);
                    crate::squad::end_guard_script(g, gd, Some(st));
                }
            }
        }
        restore_systems(g);
    }
    run!(g).ended = true;
    note(g, "", DirectorPushRowKind::Program, if finished { "finished" } else { "stopped" }, None);
    let (serial, name, t) = { let r = run_ref(g); (r.serial, r.name.clone(), r.t) };
    g.director.last_run = Some(DirectorLastRun { serial, name, t, finished });
    g.director.run = None;
    g.director.frame = None;
    g.director.due_keys.clear();
    g.director.due_clicks = 0.0;
    g.director.exiting = false;
}

/// `b.state = 'cancelled'; b.it.return(); log it`: the walk that closes a beat, in stop, the finish and the hand-over
fn cancel_beat(g: &mut Game, i: usize, why: Option<&str>) {
    set_beat_state(g, i, BeatState::Cancelled);
    let name = run_ref(g).beats[i].name.clone();
    seam_beat_return(g, &name);
    let step = run_ref(g).beats[i].step;
    let mut data = vec![("step", Json::Num(step))];
    if let Some(w) = why {
        data.push(("why", Json::Text(String::from(w))));
    }
    note(g, &name, DirectorPushRowKind::Beat, "cancelled", Some(obj(data)));
}

/// The encounter restarted under a play program: its men are gone, the program ends where it stood. (director.ts Director.onEncounterRestart)
pub fn director_on_encounter_restart(g: &mut crate::game::Game) {
    let play = matches!(&g.director.run, Some(r) if !r.ended && r.show.is_none());
    if play && !g.director.exiting {
        note(g, "", DirectorPushRowKind::Program, "reset", None);
        run!(g).units.clear();
        director_stop(g, Some(false));
    }
}

// ---------------------------------------------------------------- the tick
/// Once a frame from Game.update, before tacticsTick. (director.ts Director.tick)
pub fn director_tick(g: &mut crate::game::Game, dt: f64, input: Option<InputId>) {
    if input.is_some() {
        g.director.input = input;
    }
    let on = matches!(&g.director.run, Some(r) if !r.ended);
    if !on {
        return;
    }
    let r = run!(g);
    let serial = r.serial;
    r.t += dt;
    r.dt = dt;
    r.tick_n += 1.0;
    if let Some(s) = r.show.as_mut() {
        if let Some(shot) = s.shot.as_mut() {
            shot.t += dt;
        }
    }
    tick_tracks(g, dt);
    tick_bursts(g);
    tick_kicks(g);
    director_tick_shown(g);
    // this frame's news: hearing events not seen before, signals raised since
    let mut fresh: Vec<EventView> = Vec::new();
    let mut i = 0;
    while i < g.events.len() {
        let k = g.events[i].serial;
        if !g.director.seen.has(k) {
            g.director.seen.add(k);
            let e = g.events[i].clone();
            fresh.push(with_run_out(g, |g, r| view_of(g, r, e)));
        }
        i += 1;
    }
    while (run_ref(g).sig_seen as usize) < run_ref(g).signals.len() {
        let r = run!(g);
        let s = &r.signals[r.sig_seen as usize];
        let s = SignalViewS { name: s.name.clone(), t: s.t, data: s.data.clone() };
        fresh.push(with_run_out(g, |g, r| signal_view(g, r, &s)));
        run!(g).sig_seen += 1.0;
    }
    fire_triggers(g, &fresh);
    resume_all(g, &fresh);
    if live(g, serial) {
        tail(g, dt);
    }
}

/// `fresh.find(e => e.kind === t.event)`
fn trigger_event(t: &TriggerDef, fresh: &[EventView]) -> Option<EventView> {
    let kind = t.event.map(|k| k.as_str());
    fresh.iter().find(|e| Some(e.kind.as_str()) == kind).cloned()
}

fn fire_triggers(g: &mut Game, fresh: &[EventView]) {
    let serial = run_ref(g).serial;
    let n = run_ref(g).def.triggers.len();
    for i in 0..n {
        if !live(g, serial) {
            return;
        }
        if run_ref(g).fired.has(crate::Num(i as f64)) {
            continue;
        }
        let t = run_ref(g).def.triggers[i].clone();
        let mut ev: Option<EventView> = None;
        let go = match t.kind {
            TriggerDefKind::Start => false,
            TriggerDefKind::After => run_ref(g).t >= t.secs.unwrap(),
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
                inside_volume(g, t.volume.as_deref().unwrap(), &who)
            }
            TriggerDefKind::When => unreachable!("P.when has no page caller: a trigger of this kind is never made"),
        };
        if !go {
            continue;
        }
        run!(g).fired.add(crate::Num(i as f64));
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
        start_beat(g, &t.beat);
    }
}
/// (director.ts Director.fireTriggers)
pub fn director_fire_triggers(g: &mut crate::game::Game, r: &mut crate::director::Run, fresh: &[crate::director::EventView]) {
    lend(g, r, |g| fire_triggers(g, fresh))
}

/// → where the beat stands in R.beats. A second trigger for a running beat is a no-op; one re-started after it ended replaces its record.
fn start_beat(g: &mut Game, name: &str) -> Option<usize> {
    if !run_ref(g).def.beats.has(String::from(name)) {
        note(g, name, DirectorPushRowKind::Error, &format!("no beat '{}'", name), None);
        return None;
    }
    let cur = beat_i(g, name);
    if let Some(i) = cur {
        if run_ref(g).beats[i].state == BeatState::Running {
            return Some(i);
        }
    }
    let mut lb = LiveBeat {
        name: String::from(name),
        it: crate::Opaque,
        waiting: None,
        result: CondResult { index: 0.0, event: None },
        t0: run_ref(g).t,
        step: 0.0,
        state: BeatState::Running,
        touched: JsSet::new(),
        pinned_camera: false,
        flow: LiveBeatFlow { finish: false },
    };
    let ctx = with_run_out(g, |g, r| director_ctx_of(g, r, &mut lb));
    lb.it = seam_beat_begin(g, name, &ctx);
    if let Some(i) = cur {
        run!(g).beats.remove(i);
    }
    let r = run!(g);
    r.beats.push(lb);
    let at = r.beats.len() - 1;
    note(g, name, DirectorPushRowKind::Beat, "start", None);
    Some(at)
}
/// (director.ts Director.startBeat)
pub fn director_start_beat(g: &mut crate::game::Game, r: &mut crate::director::Run, name: &str) -> Option<crate::director::LiveBeat> {
    lend(g, r, |g| start_beat(g, name).map(|i| run_ref(g).beats[i].clone()))
}

/// Resume every running beat whose condition holds, in start order, passing again while any moved. A snapshot: a beat started this pass waits for the next.
fn resume_all(g: &mut Game, fresh: &[EventView]) {
    let serial = run_ref(g).serial;
    for _pass in 0..MAX_PASSES as usize {
        let mut moved = false;
        let snapshot: Vec<(String, u32)> = run_ref(g).beats.iter().map(|b| (b.name.clone(), seam_beat_key(b))).collect();
        for (name, key) in snapshot {
            if !live(g, serial) {
                break;
            }
            // the snapshot's own record: replaced meanwhile means it had ended
            let Some(i) = beat_i(g, &name) else { continue };
            let b = &run_ref(g).beats[i];
            if seam_beat_key(b) != key || b.state != BeatState::Running {
                continue;
            }
            if resume(g, &name, fresh) {
                moved = true;
            }
        }
        if !moved || !live(g, serial) {
            break;
        }
    }
}
/// (director.ts Director.resumeAll)
pub fn director_resume_all(g: &mut crate::game::Game, r: &mut crate::director::Run, fresh: Option<&[crate::director::EventView]>) {
    lend(g, r, |g| resume_all(g, fresh.unwrap_or(&[])))
}

/// → did the beat advance at all
fn resume(g: &mut Game, name: &str, fresh: &[EventView]) -> bool {
    let serial = run_ref(g).serial;
    let mut n: f64 = 0.0;
    loop {
        let Some(i) = beat_i(g, name) else { return n > 0.0 };
        if run_ref(g).beats[i].state != BeatState::Running {
            return n > 0.0;
        }
        if let Some(mut c) = run!(g).beats[i].waiting.take() {
            let held = holds(g, &mut c, fresh);
            let Some(i) = beat_i(g, name) else { return n > 0.0 };
            match held {
                None => {
                    run!(g).beats[i].waiting = Some(c);
                    return n > 0.0;
                }
                Some(res) => run!(g).beats[i].result = res,
            }
        }
        n += 1.0;
        if n > MAX_RESUMES {
            fail_beat(g, name, &format!("resumed {} times in one tick without waiting — a runaway loop", MAX_RESUMES));
            return true;
        }
        let result = run_ref(g).beats[beat_i(g, name).unwrap()].result.clone();
        let out = match seam_beat_next(g, name, &result) {
            Ok(out) => out,
            Err(msg) => {
                fail_beat(g, name, &msg);
                return true;
            }
        };
        if !live(g, serial) {
            return true; // the beat stopped the program from inside
        }
        let i = beat_i(g, name).unwrap();
        run!(g).beats[i].step += 1.0;
        if run_ref(g).beats[i].state == BeatState::Cancelled {
            seam_beat_return(g, name); // it cancelled itself during that step: closed here, logged there, never also 'done'
            return true;
        }
        let Some(c) = out else {
            set_beat_state(g, i, BeatState::Done);
            let (step, t0, t, finish) = { let r = run_ref(g); (r.beats[i].step, r.beats[i].t0, r.t, r.beats[i].flow.finish) };
            note(g, name, DirectorPushRowKind::Beat, "done", Some(obj(vec![("step", Json::Num(step)), ("secs", Json::Num(fixed_num(t - t0, 3)))])));
            if finish {
                begin_finish(g);
            }
            return true;
        };
        let lab = cond_label(&c);
        run!(g).beats[i].waiting = Some(c);
        if !lab.is_empty() {
            note(g, name, DirectorPushRowKind::Yield, &lab, None);
        }
        if run_ref(g).beats[i].flow.finish {
            begin_finish(g);
            if !live(g, serial) {
                return n > 0.0; // stopped: every running beat of it was cancelled
            }
        }
    }
}
/// (director.ts Director.resume) `b` stays in r.beats; this is a copy in the caller's hand, brought up to date.
pub fn director_resume(g: &mut crate::game::Game, r: &mut crate::director::Run, b: &mut crate::director::LiveBeat, fresh: &[crate::director::EventView]) -> bool {
    let name = b.name.clone();
    let moved = lend(g, r, |g| resume(g, &name, fresh));
    if let Some(row) = r.beats.iter().find(|x| x.name == name) {
        *b = row.clone();
    }
    moved
}

/// Does condition `c` hold this tick: what goes back into the generator, None = keep waiting
fn holds(g: &mut Game, c: &mut Cond, fresh: &[EventView]) -> Option<CondResult> {
    let yes = |event: Option<EventView>| Some(CondResult { index: 0.0, event });
    match c {
        Cond::Arm0(c) => if run_ref(g).t >= c.until { yes(None) } else { None },
        Cond::Arm1(c) => if run_ref(g).tick_n >= c.due { yes(None) } else { None },
        Cond::Arm2(c) => {
            let q = query_of(g);
            if seam_cond_test(g, c, q) { yes(None) } else { None }
        }
        Cond::Arm3(c) => {
            if let Some(got) = &c.got {
                return yes(Some(got.clone()));
            }
            if run_ref(g).tick_n == c.tick0 {
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
                if c.filter.as_ref().map_or(true, |f| filter_holds(g, f, e)) {
                    c.got = Some(e.clone());
                    return yes(Some(e.clone()));
                }
            }
            None
        }
        Cond::Arm4(c) => {
            let lb = run_ref(g).beats.iter().find(|b| b.name == c.name)?;
            if c.state == CondArm4State::Started || lb.state != BeatState::Running { yes(None) } else { None }
        }
        Cond::Arm5(c) => {
            if c.k == CondArm5K::All {
                // EVERY branch evaluated every tick: an event branch latches what it caught
                let (mut ok, mut ev) = (true, None);
                for x in c.of.iter_mut() {
                    match holds(g, x, fresh) {
                        None => ok = false,
                        Some(r) => if r.event.is_some() { ev = r.event },
                    }
                }
                if ok { yes(ev) } else { None }
            } else {
                for (i, x) in c.of.iter_mut().enumerate() {
                    if let Some(r) = holds(g, x, fresh) {
                        return Some(CondResult { index: i as f64, event: r.event });
                    }
                }
                None
            }
        }
    }
}
/// A c.on's filter: the closures of levels/*.script.ts, by what each captured. (a throw from `c.query.pos` in Heard is uncaught there too: TS 626)
fn filter_holds(g: &mut Game, f: &EventFilter, e: &EventView) -> bool {
    let p = e.pos;
    match f {
        EventFilter::InBox { x0, x1, z0, z1 } => p[0] > *x0 && p[0] < *x1 && p[2] > *z0 && p[2] < *z1,
        EventFilter::XAbove { x } => p[0] > *x,
        EventFilter::XBelow { x } => p[0] < *x,
        EventFilter::XZAbove { x, z } => p[0] > *x && p[2] > *z,
        EventFilter::OrderSignal => e.name.as_deref().map_or(false, |n| n.starts_with("order:")),
        EventFilter::Heard { by, run_level, run_within, walk_level, walk_within } => {
            let at = query_pos_unit(g, by);
            let near = |within: f64| m::hyp2(at[0] - p[0], at[2] - p[2]) <= within;
            (e.level >= *run_level && near(*run_within)) || (e.level >= *walk_level && near(*walk_within))
        }
    }
}
/// (director.ts Director.holds)
pub fn director_holds(g: &mut crate::game::Game, r: &mut crate::director::Run, c_2: &mut crate::director::Cond, fresh: &[crate::director::EventView]) -> Option<crate::director::CondResult> {
    lend(g, r, |g| holds(g, c_2, fresh))
}

fn fail_beat(g: &mut Game, name: &str, msg: &str) {
    let i = beat_i(g, name).unwrap();
    set_beat_state(g, i, BeatState::Failed);
    let step = run_ref(g).beats[i].step;
    note(g, name, DirectorPushRowKind::Error, &format!("threw at step {}: {}", step, msg), None);
    // armed men are never left frozen
    let touched = run_ref(g).beats[i].touched.keys();
    for id in touched {
        let gd = match run_ref(g).units.get(id) {
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
    if run_ref(g).beats[i].pinned_camera && g.director.cam.is_some() {
        g.director.cam.as_mut().unwrap().pin = None;
        if let Some(s) = run!(g).show.as_mut() {
            s.shot = None;
        }
    }
    if g.director.strict {
        panic!("{}", msg);
    }
    let r = run!(g);
    if r.show.is_some() && !r.finishing {
        r.failed_show = true; // a broken show does not hold the screen: cut, restore, and report it stopped
        begin_finish(g);
    }
}
/// (director.ts Director.failBeat)
pub fn director_fail_beat(g: &mut crate::game::Game, r: &mut crate::director::Run, b: &mut crate::director::LiveBeat, e: crate::Opaque) {
    let msg = seam_error_message(e);
    let name = b.name.clone();
    lend(g, r, |g| fail_beat(g, &name, &msg));
    b.state = BeatState::Failed;
}

fn begin_finish(g: &mut Game) {
    let r = run_ref(g);
    if r.finishing || r.ended {
        return;
    }
    run!(g).finishing = true;
    note(g, "", DirectorPushRowKind::Program, "finish", None);
    if show_on(g) {
        run!(g).show.as_mut().unwrap().ending = Some(ShowStateEnding { t: 0.0 });
        let mut i = 0;
        while i < run_ref(g).beats.len() {
            if run_ref(g).beats[i].state == BeatState::Running {
                cancel_beat(g, i, None);
            }
            i += 1;
        }
    } else {
        director_stop(g, Some(true));
    }
}
/// (director.ts Director.beginFinish)
pub fn director_begin_finish(g: &mut crate::game::Game, r: &mut crate::director::Run) {
    lend(g, r, begin_finish)
}

/// The end of every tick and of start(): the showcase's camera shot, veil and cut, and the frame main.ts reads.
fn tail(g: &mut Game, dt: f64) {
    let on = matches!(&g.director.run, Some(r) if !r.ended && r.show.is_some());
    if !on {
        g.director.frame = None;
        return;
    }
    let r = run!(g);
    let (end_veil, end_hold, failed, lower) = (r.def.opts.end_veil.unwrap_or(0.35), r.def.opts.end_hold.unwrap_or(0.5), r.failed_show, r.def.opts.lower_third.unwrap_or(false));
    let name = r.name.clone();
    let s = r.show.as_mut().unwrap();
    if s.ending.is_some() && s.handed_over {
        director_stop(g, Some(!failed)); // handed over already: the end is quiet
        return;
    }
    if let Some(e) = s.ending.as_mut() {
        e.t += dt;
        let t = e.t;
        s.fade = m::min(1.0, s.fade + dt / end_veil);
        if t > end_hold && s.fade >= 1.0 {
            director_stop(g, Some(!failed));
            return;
        }
    } else {
        if s.shot.is_some() && g.director.cam.is_some() && !s.handed_over {
            let shot = s.shot.clone().unwrap();
            let mut taken = run!(g).show.take().unwrap();
            director_frame_shot(g, &mut taken, &shot, dt);
            run!(g).show = Some(taken);
        }
        let s = run!(g).show.as_mut().unwrap();
        if s.fade_rate > 0.0 && s.fade != s.fade_to {
            let step = dt * s.fade_rate;
            s.fade = if s.fade < s.fade_to { m::min(s.fade_to, s.fade + step) } else { m::max(s.fade_to, s.fade - step) };
        }
    }
    let s = run_ref(g).show.as_ref().unwrap();
    let frame = DirectorFrame {
        look: if s.handed_over { None } else { Some(v3::copy(s.look)) },
        caption: s.caption.clone(),
        sub: s.sub.clone(),
        fade: s.fade,
        lower_third: lower && !s.handed_over,
        program: name,
        handed_over: s.handed_over,
    };
    g.director.frame = Some(frame);
}
/// (director.ts Director.tail) The run must be at home: `r` is only read, and this writes it.
pub fn director_tail(g: &mut crate::game::Game, r: &crate::director::Run, dt: f64) {
    tail(g, dt)
}

/// One camera set-up for this frame: the look-at locked or tracked, yaw and distance eased across the shot, pinned. (director.ts Director.frameShot)
pub fn director_frame_shot(g: &mut crate::game::Game, s: &mut crate::director::ShowState, shot: &crate::director::Shot, dt: f64) {
    let spec = &shot.spec;
    let want = director_look_of(g, spec);
    match spec.track {
        Some(k) if js::truthy(k) && shot.t > 1e-3 => {
            s.look = [v3::damp(s.look[0], want[0], k, dt), v3::damp(s.look[1], want[1], k, dt), v3::damp(s.look[2], want[2], k, dt)];
        }
        _ => s.look = want,
    }
    let u = v3::smoothstep(0.1, 0.9, shot.t / spec.dur);
    let yaw = (match spec.yaw_to {
        Some(to) => spec.yaw + v3::wrap_angle((to - spec.yaw) * v3::DEG) / v3::DEG * u,
        None => spec.yaw,
    }) * v3::DEG;
    let dist = match spec.dist_to {
        Some(to) => spec.dist + (to - spec.dist) * u,
        None => spec.dist,
    };
    if let Some(cam) = g.director.cam.as_mut() {
        cam.pin = Some(crate::camera::FollowCameraPin { look: v3::copy(s.look), yaw, pitch: spec.pitch * v3::DEG, dist });
    }
}

/// (director.ts Director.lookOf)
pub fn director_look_of(g: &mut crate::game::Game, s: &crate::director::CameraShotSpec) -> V3 {
    let p = seam_shot_look(g, s);
    [p[0], s.look_y.unwrap_or(0.8), p[2]]
}

// ---------------------------------------------------------------- showcase plumbing
fn enter_show(g: &mut Game) {
    let cam = g.director.cam.as_ref().map(|c| ShowStateSavedCam { yaw: c.yaw, pitch: c.pitch, dist: c.desired_distance });
    let saved = ShowStateSaved { ai: g.ai_enabled, god: g.god_mode, invisible: g.player_invisible, quiet: g.quiet_utility, touring: g.touring, cam };
    let look = v3::copy(crate::game::game_follow_pos(g, None));
    run!(g).show = Some(ShowState {
        saved, look, shot: None, shot_i: -1.0, chapter_t0: 0.0, chapters: Vec::new(), caption: String::new(), sub: String::new(), fade: 0.0, fade_to: 0.0, fade_rate: 0.0,
        ending: None, benched: None, variant_on: false, puppet: false, weather_saved: None, mission_on: false, handed_over: false, chaos_set: false,
    });
    g.quiet_utility = true;
    g.touring = true;
    crate::game::game_clear_aftermath(g);
    crate::smoke_types::smoke_system_like_clear_all(g);
    crate::player::cancel_actions(g);
    for d in g.doors.list.clone() {
        crate::doors::door_reset(g, d);
    }
    let p = g.player;
    if g.pl[p].down || run_ref(g).def.opts.fresh_encounter.unwrap_or(false) {
        crate::game::game_restart_encounter(g);
    }
    g.ai_enabled = true;
    g.god_mode = true;
    g.player_invisible = true;
    let p = g.player;
    g.pl[p].holstered = true;
    g.pl[p].crouch = false;
    g.pl[p].nv = false;
    g.pl[p].nv_amount = 0.0;
    if let Some(input) = g.director.input {
        let mut only = JsSet::new();
        only.add(String::from("KeyP"));
        let i = &mut g.tab.input[input];
        i.lock_except = Some(only);
        i.keys.clear();
        i.buttons = 0.0;
    }
    g.director.due_keys.clear();
    g.director.due_clicks = 0.0;
    let line = format!("{} — P to stop", run_ref(g).name);
    crate::game::game_msg(g, &line);
}
/// (director.ts Director.enterShow)
pub fn director_enter_show(g: &mut crate::game::Game, r: &mut crate::director::Run) {
    lend(g, r, enter_show)
}

/// the show's own flashlights back in the light list, the shift back on the list ahead of anything left
fn unbench(g: &mut Game) {
    let Some(benched) = run!(g).show.as_mut().unwrap().benched.take() else { return };
    for &gd in &benched {
        let l = g.gd[gd].flashlight;
        if !g.engine.lights.lights.contains(&l) {
            crate::render_lights::light_set_add(&mut g.engine.lights, l);
        }
    }
    let mut guards = benched;
    guards.extend_from_slice(&g.guards);
    g.guards = guards;
}
fn blackout_off_unless_permanent(g: &mut Game) {
    if crate::sim_lightshome::mains_active(g) && !crate::sim_lightshome::mains_permanent(g) {
        crate::game::game_end_blackout(g);
    }
}
fn keys_back(g: &mut Game) {
    if let Some(input) = g.director.input {
        let i = &mut g.tab.input[input];
        i.lock_except = None;
        i.keys.clear();
        i.buttons = 0.0;
    }
}
/// the follow camera at the framing he had, onto where the player now is
fn camera_back(g: &mut Game, sv: &ShowStateSaved) {
    if let (Some(cam), Some(c)) = (g.director.cam.as_mut(), sv.cam) {
        cam.yaw = c.yaw;
        cam.pitch = c.pitch;
        cam.distance = c.dist;
        cam.desired_distance = c.dist;
    }
    if g.director.cam.is_some() {
        let p = crate::game::game_follow_pos(g, None);
        let cam = g.director.cam.as_mut().unwrap();
        cam.target = v3::copy(p);
        cam.desired = v3::copy(p);
    }
}
fn restore_weather_chaos(g: &mut Game) {
    let mut s = run!(g).show.take().unwrap();
    director_restore_weather(g, &mut s);
    director_restore_chaos(g, &mut s);
    run!(g).show = Some(s);
}

fn exit_show(g: &mut Game) {
    let sv = run_ref(g).show.as_ref().unwrap().saved;
    restore_weather_chaos(g);
    if run_ref(g).show.as_ref().unwrap().handed_over {
        g.touring = sv.touring;
        g.mission_in_tour = false;
        note(g, "", DirectorPushRowKind::Note, "restore", Some(obj(vec![("handedOver", Json::Bool(true))])));
        return;
    }
    unbench(g);
    if run_ref(g).show.as_ref().unwrap().variant_on {
        crate::lighting::set_daylight(g, false);
        run!(g).show.as_mut().unwrap().variant_on = false;
    }
    g.mission_in_tour = false;
    crate::game::game_clear_aftermath(g);
    crate::smoke_types::smoke_system_like_clear_all(g);
    crate::game::game_restart_encounter(g);
    if let Some(cam) = g.director.cam.as_mut() {
        cam.pin = None;
    }
    g.ai_enabled = sv.ai;
    g.god_mode = sv.god;
    g.player_invisible = sv.invisible;
    g.quiet_utility = sv.quiet;
    g.touring = sv.touring;
    crate::opening::opening_start(g);
    camera_back(g, &sv);
    if run_ref(g).show.as_ref().unwrap().puppet {
        g.puppet = None;
    }
    blackout_off_unless_permanent(g);
    keys_back(g);
    note(g, "", DirectorPushRowKind::Note, "restore", None);
}
/// (director.ts Director.exitShow)
pub fn director_exit_show(g: &mut crate::game::Game, r: &mut crate::director::Run) {
    lend(g, r, exit_show)
}

/// The storm's knobs back as c.weather found them. (director.ts Director.restoreWeather)
pub fn director_restore_weather(g: &mut crate::game::Game, s: &mut crate::director::ShowState) {
    let Some(w) = s.weather_saved else { return };
    g.weather.storm = w.storm;
    g.weather.rain_rate = w.rain;
    s.weather_saved = None;
}

/// The program's own destruction knob dropped. (director.ts Director.restoreChaos)
pub fn director_restore_chaos(g: &mut crate::game::Game, s: &mut crate::director::ShowState) {
    if !s.chaos_set {
        return;
    }
    g.destr.show_chaos = None;
    s.chaos_set = false;
}

/// c.handover: under a whole veil a fresh encounter, the player's flags live, the follow camera and the keys given back; the caption stays as the top card. Idempotent.
fn hand_over(g: &mut Game, caption: &str, sub: &str, caller: Option<&str>) {
    let sv = run_ref(g).show.as_ref().unwrap().saved;
    if run_ref(g).show.as_ref().unwrap().handed_over {
        let s = run!(g).show.as_mut().unwrap();
        s.caption = String::from(caption);
        s.sub = String::from(sub);
        return;
    }
    // every OTHER beat ends here: the men it held are about to be replaced by the fresh encounter's
    let mut i = 0;
    while i < run_ref(g).beats.len() {
        let b = &run_ref(g).beats[i];
        if b.state == BeatState::Running && Some(b.name.as_str()) != caller {
            cancel_beat(g, i, Some("handover"));
        }
        i += 1;
    }
    restore_weather_chaos(g);
    let gds: Vec<GuardId> = run_ref(g).units.values().map(|u| u.gd).collect();
    for gd in gds {
        crate::guards::despawn_unit(g, gd);
    }
    let r = run!(g);
    r.units.clear();
    r.kicks.clear();
    r.tracks.clear();
    r.bursts.clear();
    unbench(g);
    if run_ref(g).show.as_ref().unwrap().variant_on {
        crate::lighting::set_daylight(g, false);
        run!(g).show.as_mut().unwrap().variant_on = false;
    }
    g.puppet = None;
    run!(g).show.as_mut().unwrap().puppet = false;
    g.mission_in_tour = false;
    run!(g).show.as_mut().unwrap().mission_on = false;
    blackout_off_unless_permanent(g);
    crate::game::game_clear_aftermath(g);
    crate::smoke_types::smoke_system_like_clear_all(g);
    crate::game::game_restart_encounter(g);
    g.player_invisible = false;
    g.god_mode = sv.god;
    g.ai_enabled = sv.ai;
    g.quiet_utility = false;
    let r = run!(g);
    r.show.as_mut().unwrap().shot = None;
    for b in r.beats.iter_mut() {
        b.pinned_camera = false;
    }
    if let Some(cam) = g.director.cam.as_mut() {
        cam.pin = None;
    }
    camera_back(g, &sv);
    keys_back(g);
    g.director.due_keys.clear();
    g.director.due_clicks = 0.0;
    let s = run!(g).show.as_mut().unwrap();
    s.caption = String::from(caption);
    s.sub = String::from(sub);
    s.chapters = Vec::new();
    s.handed_over = true;
    s.fade = 1.0;
    s.fade_to = 1.0;
    s.fade_rate = 0.0;
    note(g, "", DirectorPushRowKind::Note, "handover", None);
}
/// (director.ts Director.handOver)
pub fn director_hand_over(g: &mut crate::game::Game, r: &mut crate::director::Run, caption: &str, sub: &str, caller: Option<&crate::director::LiveBeat>) {
    let caller = caller.map(|b| b.name.clone());
    lend(g, r, |g| hand_over(g, caption, sub, caller.as_deref()))
}

/// Only what the script set is kept, so only that is put back: a setting the player changed under the program stays his.
fn apply_systems(g: &mut Game) {
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
    run!(g).systems_saved = Some(saved);
    if let Some(x) = qrf { g.tune.qrf = x; }
    if let Some(x) = needs { g.tune.needs = x; }
    if let Some(x) = civilians { g.tune.civilians = x; }
    if let Some(x) = autosave { g.saves.enabled = x; }
}
/// (director.ts Director.applySystems)
pub fn director_apply_systems(g: &mut crate::game::Game, r: &mut crate::director::Run) {
    lend(g, r, apply_systems)
}

fn restore_systems(g: &mut Game) {
    let Some(s) = run!(g).systems_saved.take() else { return };
    if let Some(&x) = s.get("qrf") { g.tune.qrf = x; }
    if let Some(&x) = s.get("needs") { g.tune.needs = x; }
    if let Some(&x) = s.get("civilians") { g.tune.civilians = x; }
    if let Some(&x) = s.get("autosave") { g.saves.enabled = x; }
}
/// (director.ts Director.restoreSystems)
pub fn director_restore_systems(g: &mut crate::game::Game, r: &mut crate::director::Run) {
    lend(g, r, restore_systems)
}

// ---------------------------------------------------------------- drills
fn drill_note(g: &mut Game, t: Option<f64>, beat: &str, what: &str, data: Option<Json>) {
    director_push(g, &DirectorPushRow { kind: DirectorPushRowKind::Drill, what: String::from(what), data, beat: String::from(beat), t });
}

/// A drill's frame; `d` is out of R.drills for the call.
fn tick_drill(g: &mut Game, d: &mut DrillRun, dt: f64) {
    if let DrillImpl::Breach(i) = &mut d.impl_ {
        // ticked past its own 'done' too: its clock runs on for since('done'), its men keep settling on the stack
        crate::breach::breach_drill_tick(g, dt);
        let (t, phase) = { let b = g.night_breach.drill.as_ref().unwrap(); (b.t, b.phase) };
        d.t = t;
        d.phase = String::from(phase.as_str());
        while (i.forwarded as usize) < g.night_breach.drill.as_ref().unwrap().beats.len() {
            let b = g.night_breach.drill.as_ref().unwrap().beats[i.forwarded as usize].clone();
            if !d.times.has(b.what.clone()) {
                d.times.set(b.what.clone(), b.t);
            }
            drill_note(g, Some(b.t), &d.owner, &b.what, None);
            if b.what == "abort" {
                d.aborted = true;
            }
            i.forwarded += 1.0;
        }
        if phase == crate::breach::Phase::Done && !d.done {
            d.done = true;
            d.cleared = !d.aborted;
            if d.cleared {
                for h in g.night_breach.drill.as_ref().unwrap().hostiles.clone() {
                    if !(g.gd[h].state == crate::game::GuardState::Dead || crate::runner::runner_fled(g, h)) {
                        d.cleared = false;
                        break;
                    }
                }
            }
        }
        return;
    }
    if d.done {
        return;
    }
    match &mut d.impl_ {
        DrillImpl::Breach(_) => {}
        DrillImpl::Clear(i) => {
            i.clock += dt;
            d.t = i.clock;
            let id = i.d;
            crate::squad::room_clear_tick(g, id, i.clock, dt);
            let phase = g.tab.squad_room_clear[id].phase.as_str();
            if phase != i.last_phase {
                i.last_phase = String::from(phase);
                d.phase = String::from(phase);
                d.times.set(String::from(phase), i.clock);
                drill_note(g, Some(i.clock), &d.owner, &format!("{}:{}", d.ref_.name, phase), None);
            }
            if !crate::squad::room_clear_intact(g, id) {
                d.done = true;
                d.aborted = true;
                drill_note(g, None, &d.owner, &format!("{}:broken", d.ref_.name), None);
            } else if crate::squad::room_clear_complete(g, id) {
                d.done = true;
                d.cleared = true;
                drill_note(g, None, &d.owner, &format!("{}:complete", d.ref_.name), None);
            }
        }
        DrillImpl::Hold => {
            d.t += dt;
            let set = d.men.iter().all(|&m| {
                g.gd[m].state == crate::game::GuardState::Dead || g.gd[m].script.as_ref().map_or(false, |s| s.arrived.unwrap_or(false))
            });
            if !d.done && set {
                d.done = true;
                d.cleared = true;
                d.phase = String::from("set");
                drill_note(g, None, &d.owner, &format!("{}:set", d.ref_.name), None);
            }
        }
        DrillImpl::Exchange(i) => {
            d.t += dt;
            let (done, result) = seam_engagement(g, &i.e);
            if done && !d.done {
                d.done = true;
                d.phase = match &result {
                    Some(r) => String::from(r.outcome.as_str()),
                    None => String::from("done"),
                };
                d.cleared = d.phase == "cleared";
                d.aborted = d.phase == "wiped" || d.phase == "reset";
                let data = result.as_ref().map(|r| obj(vec![("secs", Json::Num(fixed_num(r.secs, 2))), ("kills", Json::Num(r.kills.len() as f64))]));
                drill_note(g, None, &d.owner, &format!("{}:{}", d.ref_.name, d.phase), data);
            }
        }
    }
}
/// (director.ts Director.tickDrill) `d` is out of R.drills; `r` is not read: the log goes by the run at home.
pub fn director_tick_drill(g: &mut crate::game::Game, r: &crate::director::Run, d: &mut crate::director::DrillRun, dt: f64) {
    tick_drill(g, d, dt)
}

fn end_drill(g: &mut Game, d: &mut DrillRun, show: bool) {
    match &d.impl_ {
        DrillImpl::Breach(_) => crate::breach::breach_drill_end(g),
        DrillImpl::Clear(i) => {
            let st = if show { crate::squad::RoomClearEndState::Patrol } else { crate::squad::RoomClearEndState::Search };
            crate::squad::room_clear_end(g, i.d, Some(st));
        }
        DrillImpl::Hold => {
            for m in d.men.clone() {
                if g.gd[m].script.is_some() && g.gd[m].state != crate::game::GuardState::Dead {
                    let st = if show { crate::squad::EndGuardScriptState::Patrol } else { hand_back_state(g, m) };
                    crate::squad::end_guard_script(g, m, Some(st));
                }
            }
        }
        DrillImpl::Exchange(_) => {}
    }
    d.done = true;
}
/// (director.ts Director.endDrill)
pub fn director_end_drill(g: &mut crate::game::Game, r: &crate::director::Run, d: &mut crate::director::DrillRun) {
    end_drill(g, d, r.show.is_some())
}

// ---------------------------------------------------------------- men in the picture through the fog; bursts; tracks; kicks
/// (director.ts Director.showMan)
pub fn director_show_man(g: &mut crate::game::Game, gd: GuardId, on: bool) {
    if on {
        g.director.shown.add(gd);
    } else if !g.director.shown.delete(gd) {
        return;
    }
    crate::fogofwar::fog_of_war_show(g, gd, on);
}

/// A man is shown only while a script holds him. (director.ts Director.tickShown)
pub fn director_tick_shown(g: &mut crate::game::Game) {
    for gd in g.director.shown.keys() {
        if g.gd[gd].script.is_none() || g.gd[gd].state == crate::game::GuardState::Dead {
            director_show_man(g, gd, false);
        }
    }
}

/// a man the program spawned, or one of the level's own it took in hand
fn unit_guard(g: &Game, id: f64) -> Option<GuardId> {
    match run_ref(g).units.get(crate::Num(id)) {
        Some(u) => Some(u.gd),
        None => guard_by_char_id(g, id),
    }
}

/// A scripted round is a REAL round: hits, suppression, marks, heat, glass, hearing all follow from combat.ts.
fn tick_bursts(g: &mut Game) {
    let serial = run_ref(g).serial;
    let mut i = 0;
    while i < run_ref(g).bursts.len() {
        let k = i;
        i += 1;
        let b = run_ref(g).bursts[k].clone();
        if b.left <= 0.0 {
            continue;
        }
        let gd = unit_guard(g, b.u.id).filter(|&x| g.gd[x].state != crate::game::GuardState::Dead);
        let Some(gd) = gd else {
            run!(g).bursts[k].left = 0.0;
            continue;
        };
        let weapon = g.gd[gd].weapon;
        if run_ref(g).t < b.next || crate::weapons::firearm_reloading(g, weapon) {
            continue;
        }
        let at: Option<V3> = match &b.at {
            RunBurstsItemAt::Unit(u) => unit_guard(g, u.id).map(|v| chest_of(g, g.gd[v].ch, 1.1)),
            RunBurstsItemAt::List(v) => Some([v[0], js::or(v[1], 1.1), v[2]]),
            RunBurstsItemAt::Mark(mk) => {
                let p = director_place_of(g, &Place::Mark(mk.clone()));
                Some([p[0], 1.1, p[2]]) // (a mark has no [1]: `|| 1.1`)
            }
        };
        let Some(at) = at else {
            run!(g).bursts[k].left = 0.0;
            continue;
        };
        if crate::weapons::firearm_fire(g, weapon) {
            let c = g.gd[gd].ch;
            crate::combat::fire_weapon(g, c, at, false, Some(&crate::combat::FireOpts { steady: Some(true), spread: Some(b.spread), ..Default::default() }));
            if !live(g, serial) {
                return;
            }
            let r = run!(g);
            let t = r.t;
            let row = &mut r.bursts[k];
            row.left -= 1.0;
            row.fired += 1.0;
            row.next = t + row.cadence;
        } else {
            crate::weapons::firearm_rearm(g, weapon);
        }
        let row = &run_ref(g).bursts[k];
        if row.left <= 0.0 {
            let data = obj(vec![("by", Json::Text(row.u.callsign.clone())), ("rounds", Json::Num(row.fired))]);
            note(g, "", DirectorPushRowKind::Stimulus, "burst:done", Some(data));
        }
    }
    // the finished leave the list: a long program fires hundreds
    run!(g).bursts.retain(|b| b.left > 0.0);
}
/// (director.ts Director.tickBursts)
pub fn director_tick_bursts(g: &mut crate::game::Game, r: &mut crate::director::Run) {
    lend(g, r, tick_bursts)
}

/// Aim points ordered onto a man: his chest re-read into the very vector the runner points the gun along, eased at 14/s; dropped when the tracker is dead, off his script, or given another aim point.
fn tick_tracks(g: &mut Game, dt: f64) {
    if run_ref(g).tracks.is_empty() {
        return;
    }
    let mut tracks = core::mem::take(&mut run!(g).tracks);
    tracks.retain(|t| {
        g.gd[t.gd].state != crate::game::GuardState::Dead && g.gd[t.gd].script.as_ref().map_or(false, |s| s.aim_at == Some(t.vec))
    });
    for t in tracks.iter_mut() {
        let up = if g.gd[t.tgt].state == crate::game::GuardState::Dead { 0.3 } else { 1.1 };
        let ch = chest_of(g, g.gd[t.tgt].ch, up);
        t.vec[0] = v3::damp(t.vec[0], ch[0], 14.0, dt);
        t.vec[1] = v3::damp(t.vec[1], ch[1], 14.0, dt);
        t.vec[2] = v3::damp(t.vec[2], ch[2], 14.0, dt);
        // the script holds its own copy here: written through by hand
        g.gd[t.gd].script.as_mut().unwrap().aim_at = Some(t.vec);
    }
    run!(g).tracks = tracks;
}
/// (director.ts Director.tickTracks)
pub fn director_tick_tracks(g: &mut crate::game::Game, r: &mut crate::director::Run, dt: f64) {
    lend(g, r, |g| tick_tracks(g, dt))
}

/// Kicks in the air: on the animator's impact frame (or the instant the clip is somehow over) doors.ts kickIn does the boot's work from where he stands.
fn tick_kicks(g: &mut Game) {
    if run_ref(g).kicks.is_empty() {
        return;
    }
    let kicks = core::mem::take(&mut run!(g).kicks);
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
    run!(g).kicks = kept;
}
/// (director.ts Director.tickKicks)
pub fn director_tick_kicks(g: &mut crate::game::Game, r: &mut crate::director::Run) {
    lend(g, r, tick_kicks)
}

// ---------------------------------------------------------------- resolving handles
fn place_of(g: &Game, p: &Place) -> Result<V3, String> {
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
/// (director.ts Director.placeOf)
pub fn director_place_of(g: &mut crate::game::Game, p: &crate::director::Place) -> V3 {
    match place_of(g, p) {
        Ok(v) => v,
        Err(m) => seam_beat_throw(&m),
    }
}

fn door_of(g: &mut Game, d: &DoorRef) -> Result<DoorId, String> {
    crate::doors::doors_by_name(g, &d.id).ok_or_else(|| format!("door '{}' does not resolve", d.id))
}
/// (director.ts Director.doorOf)
pub fn director_door_of(g: &mut crate::game::Game, d: &crate::director::DoorRef) -> DoorId {
    match door_of(g, d) {
        Ok(v) => v,
        Err(m) => seam_beat_throw(&m),
    }
}

/// `rec`: what R.units has for him; `handed`: R.show?.handedOver; `benched`: R.show?.benched
fn guard_of_with(g: &Game, rec: Option<GuardId>, handed: bool, benched: Option<&Vec<GuardId>>, u: &UnitRef) -> Result<GuardId, String> {
    if let Some(gd) = rec {
        return Ok(gd);
    }
    if handed {
        return Err(format!("unit {} (#{}) belongs to the live floor after the hand-over", u.callsign, u.id));
    }
    if let Some(gd) = guard_by_char_id(g, u.id) {
        return Ok(gd);
    }
    if let Some(gd) = benched.and_then(|b| b.iter().copied().find(|&x| g.ch[g.gd[x].ch].id == u.id)) {
        return Ok(gd);
    }
    Err(format!("unit {} (#{}) is not in the world", u.callsign, u.id))
}
fn guard_of_in(g: &Game, r: &Run, u: &UnitRef) -> Result<GuardId, String> {
    let s = r.show.as_ref();
    guard_of_with(g, r.units.get(crate::Num(u.id)).map(|x| x.gd), s.map_or(false, |s| s.handed_over), s.and_then(|s| s.benched.as_ref()), u)
}
/// (director.ts Director.guardOf)
pub fn director_guard_of(g: &mut crate::game::Game, r: &crate::director::Run, u: &crate::director::UnitRef) -> GuardId {
    match guard_of_in(g, r, u) {
        Ok(v) => v,
        Err(m) => seam_beat_throw(&m),
    }
}

fn plan_of(g: &Game, p: &PlanRef) -> Result<DirectorPlanOf, String> {
    if p.plan == PlanKind::Breach {
        return match &g.level.breach {
            Some(b) => Ok(DirectorPlanOf::BreachPlan(b.clone())),
            None => Err(String::from("the level has no breach plan")),
        };
    }
    let found = g.level.clear_plans.as_ref().and_then(|l| l.iter().find(|x| x.room.as_deref() == Some(p.id.as_str()) || x.door == p.id));
    match found {
        Some(x) => Ok(DirectorPlanOf::RoomClearPlan(x.clone())),
        None => Err(format!("no room-clear plan '{}'", p.id)),
    }
}
/// (director.ts Director.planOf)
pub fn director_plan_of(g: &mut crate::game::Game, p: &crate::director::PlanRef) -> crate::director::DirectorPlanOf {
    match plan_of(g, p) {
        Ok(v) => v,
        Err(m) => seam_beat_throw(&m),
    }
}

/// World to the volume's local frame with the boxes' own convention: feet anywhere from its floor to head height inside count.
fn in_volume(v: &crate::scene_level::VolumeDef, p: V3) -> bool {
    let (px, pz) = (p[0] - v.c[0], p[2] - v.c[2]);
    let (cs, sn) = (m::cos(v.yaw.unwrap_or(0.0)), m::sin(v.yaw.unwrap_or(0.0)));
    let (lx, lz) = (cs * px - sn * pz, sn * px + cs * pz);
    m::abs(lx) <= v.h[0] && m::abs(lz) <= v.h[2] && p[1] >= v.c[1] - v.h[1] - 0.1 && p[1] <= v.c[1] + v.h[1] + 2.0
}
/// `units`: R.units as (id, man), in their order
fn inside_volume_with(g: &Game, units: &[(f64, GuardId)], id: &str, who: &DirectorInsideVolumeWho) -> bool {
    let Some(v) = g.level.volumes.as_ref().and_then(|l| l.iter().find(|x| x.name == id)) else { return false };
    let pl = &g.pl[g.player];
    let alive = |gd: GuardId| g.gd[gd].state != crate::game::GuardState::Dead;
    let sam = !pl.down && in_volume(v, g.ch[pl.ch].pos);
    match who {
        DirectorInsideVolumeWho::Sam => sam,
        DirectorInsideVolumeWho::Team => sam || g.guards.iter().any(|&gd| g.gd[gd].team == crate::teams::Team::Operative && alive(gd) && in_volume(v, g.ch[g.gd[gd].ch].pos)),
        DirectorInsideVolumeWho::AnyUnit => units.iter().any(|&(_, gd)| alive(gd) && in_volume(v, g.ch[g.gd[gd].ch].pos)),
        DirectorInsideVolumeWho::Unit(u) => units.iter().find(|x| x.0 == u.id).map_or(false, |&(_, gd)| alive(gd) && in_volume(v, g.ch[g.gd[gd].ch].pos)),
    }
}
fn units_of(r: &Run) -> Vec<(f64, GuardId)> {
    r.units.iter().map(|(k, u)| (k.0, u.gd)).collect()
}
fn inside_volume(g: &mut Game, id: &str, who: &DirectorInsideVolumeWho) -> bool {
    let units = units_of(run_ref(g));
    inside_volume_with(g, &units, id, who)
}
/// (director.ts Director.insideVolume)
pub fn director_inside_volume(g: &mut crate::game::Game, r: &crate::director::Run, id_2: &str, who: &crate::director::DirectorInsideVolumeWho) -> bool {
    inside_volume_with(g, &units_of(r), id_2, who)
}

/// the Query's getters as they read now (the TypeScript's are live; this is a Copy of plain numbers, so it is made afresh at each ask)
fn query_now(g: &mut Game, t: f64) -> Query {
    let active = crate::sim_lightshome::mains_active(g);
    let daylight = crate::lighting::is_daylight(g);
    Query { t, time: g.time, escalation: g.escalation, mains_on: !active, daylight }
}
fn query_of(g: &mut Game) -> Query {
    let t = run_ref(g).t;
    query_now(g, t)
}
/// (director.ts Director.queryOf)
pub fn director_query_of(g: &mut crate::game::Game, r: &mut crate::director::Run) -> crate::director::Query {
    query_now(g, r.t)
}

// The Query's methods: closures in the TypeScript, with no entry in the skeleton (notes.md, Skeleton). The run is at home.
fn query_alive(g: &mut Game, u: &UnitRef) -> bool {
    unit_guard(g, u.id).map_or(false, |gd| g.gd[gd].state != crate::game::GuardState::Dead)
}
fn query_arrived(g: &mut Game, u: &UnitRef) -> bool {
    let r = run_ref(g);
    match guard_of_in(g, r, u) {
        Ok(gd) => g.gd[gd].script.as_ref().map_or(false, |s| s.arrived.unwrap_or(false)),
        Err(m) => seam_beat_throw(&m),
    }
}
fn query_state(g: &mut Game, u: &UnitRef) -> Option<crate::game::GuardState> {
    guard_of_in(g, run_ref(g), u).ok().map(|gd| g.gd[gd].state)
}
fn query_room_sam(g: &mut Game) -> String {
    let p = g.ch[g.pl[g.player].ch].pos;
    crate::chronicle::room_of(g, p)
}
fn query_room_unit(g: &mut Game, u: &UnitRef) -> String {
    let gd = match guard_of_in(g, run_ref(g), u) {
        Ok(gd) => gd,
        Err(m) => seam_beat_throw(&m),
    };
    let p = g.ch[g.gd[gd].ch].pos;
    crate::chronicle::room_of(g, p)
}
fn query_pos_sam(g: &mut Game) -> V3 {
    if !show_on(g) {
        seam_beat_throw("query.pos('sam') is showcase-only");
    }
    v3::copy(g.ch[g.pl[g.player].ch].pos)
}
fn query_pos_unit(g: &mut Game, u: &UnitRef) -> V3 {
    match guard_of_in(g, run_ref(g), u) {
        Ok(gd) => v3::copy(g.ch[g.gd[gd].ch].pos),
        Err(m) => seam_beat_throw(&m),
    }
}
fn query_pos_place(g: &mut Game, p: &Place) -> V3 {
    v3::copy(director_place_of(g, p))
}
/// angle, open, latched, locked, broken, swing. (DOORWAY.oneWay is settled on: swing is the door's.)
fn query_door(g: &mut Game, d: &DoorRef) -> (f64, bool, bool, bool, bool, f64) {
    let id = director_door_of(g, d);
    (crate::doors::door_angle(g, id), !crate::doors::door_is_closed(g, id), crate::doors::door_latched(g, id), crate::doors::door_locked(g, id), crate::doors::door_lock_broken(g, id), crate::doors::door_swing(g, id))
}
fn query_group_on(g: &mut Game, gr: &GroupRef) -> bool {
    for t in g.targets.clone() {
        if g.tab.sim_lightshome_fixture[t].group == gr.id && crate::sim_lightshome::fixture_on(g, t) {
            return true;
        }
    }
    false
}
fn query_inside(g: &mut Game, v: &VolumeRef, who: &DirectorInsideVolumeWho) -> bool {
    inside_volume(g, &v.id, who)
}
fn query_plan(g: &mut Game, p: &PlanRef) -> DirectorPlanOf {
    director_plan_of(g, p)
}
fn query_flag(g: &mut Game, n: &str) -> Option<Option<Json>> {
    run_ref(g).flags.get(String::from(n)).cloned()
}
fn query_signalled(g: &mut Game, n: &str) -> bool {
    run_ref(g).signals.iter().any(|s| s.name == n)
}
fn query_routed(g: &mut Game, u: &UnitRef) -> bool {
    match guard_of_in(g, run_ref(g), u) {
        Ok(gd) => crate::runner::runner_owns(g, gd),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// the TypeScript's own words, from bun
    #[test]
    fn seeds() {
        for (serial, want) in [(0.0, 625341585.0), (1.0, 3144846632.0), (2.0, 422250467.0), (3.0, 4293106106.0), (7.0, 1992401054.0), (1000.0, 771132985.0)] {
            assert_eq!(seed_of(None, serial).to_bits(), f64::to_bits(want));
        }
        assert_eq!(seed_of(Some(5.0), 0.0), 5.0);
        assert_eq!(seed_of(Some(-1.0), 0.0), 4294967295.0);
        assert_eq!(seed_of(Some(4294967297.5), 9.0), 1.0);
    }
    /// the TypeScript's own answers, from bun
    #[test]
    fn volume() {
        let v = crate::scene_level::VolumeDef { c: [1.0, 0.5, 2.0], h: [3.0, 0.5, 1.0], yaw: Some(0.7), ..crate::scene_level::VolumeDef::zeroed() };
        for (p, want) in VOLUME_CASES {
            assert_eq!(in_volume(&v, *p), *want, "{:?}", p);
        }
    }
    const VOLUME_CASES: &[(V3, bool)] = &[
        ([1.0, 0.0, 2.0], true),
        ([1.0, -0.2, 2.0], false),
        ([1.0, 3.1, 2.0], false),
        ([1.0, 2.9, 2.0], true),
        ([3.0, 0.0, 3.5], false),
        ([-1.0, 0.0, 3.2], true),
        ([3.2, 0.0, 0.2], true),
        ([4.1, 0.0, 2.0], false),
    ];
}
