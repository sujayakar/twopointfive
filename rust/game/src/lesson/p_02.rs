//! src/game/lesson.ts lines 298 to 429
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A new lesson: nothing done, no run yet.   (lesson.ts Lesson.constructor)
pub fn lesson_new(g: &mut crate::game::Game) -> crate::lesson::Lesson {
    crate::lesson::Lesson::blank()
}

/// The lesson in force (None: not live).   (lesson.ts Lesson.data)
pub fn lesson_data(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::LessonData> {
    if !g.k.lesson.on {
        return None;
    }
    let on_rails = match g.level.scenarios.as_ref().and_then(|s| s.lesson.as_ref()) {
        Some(l) if !l.steps.is_empty() => l.rails.is_some(),
        _ => return None,
    };
    if on_rails && !g.k.rails.on {
        return None;
    }
    if g.touring || g.quiet_utility || g.puppet.is_some() {
        return None;
    }
    if g.quiet.active || crate::roomclear::room_clear_scenario_active(g) || g.night_breach.active || g.rescue.active {
        g.level.scenarios.as_ref().and_then(|s| s.lesson.clone())
    } else {
        None
    }
}

/// The step that is up for the mission that is running: a fresh run is on its first step.   (lesson.ts Lesson.at)
pub fn lesson_at(g: &mut crate::game::Game) -> f64 {
    if run_is_mission(g) { g.lesson.i } else { 0.0 }
}

/// The index of the step that is up for the mission that is running.   (lesson.ts Lesson.index)
pub fn lesson_index(g: &mut crate::game::Game) -> f64 {
    lesson_at(g)
}

/// The step that is up, if any.   (lesson.ts Lesson.step)
pub fn lesson_step(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::LessonStep> {
    let l = lesson_data(g)?;
    let at = lesson_at(g);
    l.steps.into_iter().nth(at as usize)
}

/// Every step of the lesson in force is done.   (lesson.ts Lesson.finished)
pub fn lesson_finished(g: &mut crate::game::Game) -> bool {
    match lesson_data(g) {
        Some(l) => lesson_at(g) >= l.steps.len() as f64,
        None => false,
    }
}

/// Once a frame, after updateMission: counts the frame's deeds, asks the step's test, moves on when it is done and settled.   (lesson.ts Lesson.tick)
pub fn lesson_tick(g: &mut crate::game::Game, dt: f64) {
    let l = match lesson_data(g) {
        Some(l) => l,
        None => return,
    };
    if !run_is_mission(g) {
        g.lesson.run = Some(g.mission.serial);
        g.lesson.i = 0.0;
        g.lesson.done_at.clear();
        g.lesson.seen = None;
        g.lesson.log.clear();
        g.stats.lesson.started += 1;
        lesson_note(g, &format!("lesson '{}': {} steps", l.title, l.steps.len()));
    }
    if g.lesson.i >= l.steps.len() as f64 || g.mission.stage == crate::game::MissionStage::Failed || who_down(g) {
        return;
    }
    // The row is lent out of `g.lesson.seen` while `g` is passed along beside it; nothing it calls reads `seen` (private in the TypeScript, read by `settling` only).
    let mut s = match g.lesson.seen.take() {
        Some(s) => s,
        None => lesson_fresh(g, Some(1e-6)),
    };
    if s.done_t.is_some() && s.done_i != Some(g.lesson.i) {
        s.done_t = None;
        s.done_i = None;
        s.idle = None;
        s.held = 0.0;
    }
    lesson_count(g, &mut s);
    let i = g.lesson.i;
    let t = &l.steps[i as usize];
    if s.done_t.is_none() {
        if seam_passes(g, &t.done, &mut s) {
            s.held += dt;
            let hold = t.hold.unwrap_or(0.0);
            if s.held > hold || !js::truthy(hold) {
                s.done_t = Some(g.time);
                s.done_i = Some(i);
                if g.time - s.t0 <= dt * 1.5 {
                    s.idle = Some(true);
                }
            }
        } else {
            s.held = 0.0;
        }
    }
    let (done_t, idle) = (s.done_t, s.idle.unwrap_or(false));
    g.lesson.seen = Some(s);
    if let Some(done_t) = done_t {
        let waits = (!idle && g.time - done_t < t.settle.unwrap_or(0.0)) || crate::rails::rails_settles(g, &l, i, done_t);
        if !waits {
            if g.time > done_t {
                let said = format!("'{}' settled {} s", t.id, to_fixed(g.time - done_t, 1));
                lesson_note(g, &said);
            }
            lesson_advance(g, &l, t);
        }
    }
}

/// The step that is up has done its deed and is waiting for the moment to play out.   (lesson.ts Lesson.settling)
pub fn lesson_settling(g: &mut crate::game::Game) -> bool {
    if !run_is_mission(g) {
        return false;
    }
    match g.lesson.seen.as_ref() {
        Some(s) => s.done_t.is_some() && s.done_i == Some(g.lesson.i),
        None => false,
    }
}

/// The step is done: the next comes up, and a checkpoint is asked for unless the level is on rails.   (lesson.ts Lesson.advance)
pub fn lesson_advance(g: &mut crate::game::Game, l: &crate::scene_scenarios::LessonData, t: &crate::scene_scenarios::LessonStep) {
    let now = g.time;
    g.lesson.done_at.set(t.id.clone(), now);
    g.lesson.i += 1.0;
    g.lesson.seen = Some(lesson_fresh(g, None));
    g.stats.lesson.steps += 1;
    let (i, len) = (g.lesson.i, l.steps.len() as f64);
    lesson_note(g, &format!("done {}/{} '{}'", i as usize, l.steps.len(), t.id));
    if i >= len {
        g.stats.lesson.finished += 1;
        lesson_note(g, "every step done");
    }
    if crate::rails::rails_live(g) {
        return;
    }
    let label = if i < len { format!("training {}/{}", i as usize + 1, l.steps.len()) } else { String::from("training done") };
    crate::savestate::checkpoints_request(g, "lesson", &label);
}

/// What a step that has just come up has seen: nothing, counted from now.   (lesson.ts Lesson.fresh)
pub fn lesson_fresh(g: &mut crate::game::Game, back: Option<f64>) -> crate::lesson::StepSeen {
    let back = back.unwrap_or(0.0);
    let ev = g.chronicle.events.len() as f64;
    let last_order = g.orders.log.last().map(|r| r.serial);
    let last_pair = g.m.pairorders.pair_log.last().map(|r| r.serial);
    let seat = g.seat;
    let (yaw, dist) = (g.tab.cmd_seat[seat].cam.yaw, g.tab.cmd_seat[seat].cam.distance);
    crate::lesson::StepSeen {
        t0: g.time - back,
        n: JsMap::new(),
        chron: ev,
        chron0: ev,
        held_at: -1.0,
        last_order,
        last_pair,
        words: JsSet::new(),
        heard: JsSet::new(),
        held: 0.0,
        yaw0: yaw,
        dist0: dist,
        yaw_last: yaw,
        turned: 0.0,
        done_t: None,
        idle: None,
        done_i: None,
    }
}

/// This frame's deeds into the step's counts.   (lesson.ts Lesson.count)
pub fn lesson_count(g: &mut crate::game::Game, s: &mut crate::lesson::StepSeen) {
    let seat = g.seat;
    if js::truthy(g.rails.fresh) {
        let yaw = g.tab.cmd_seat[seat].cam.yaw;
        s.yaw_last = yaw;
        s.yaw0 = yaw;
        s.dist0 = g.tab.cmd_seat[seat].cam.distance;
        s.turned = 0.0;
    }
    {
        let y = g.tab.cmd_seat[seat].cam.yaw;
        let mut d = y - s.yaw_last;
        d = m::atan2(m::sin(d), m::cos(d));
        s.turned += m::abs(d);
        s.yaw_last = y;
    }
    if let Some(verb) = g.act.as_ref().and_then(|a| a.verb) {
        if !who_down(g) {
            add(s, format!("act:{}", verb.as_str()));
        }
    }
    let mut at = 0;
    while at < g.events.len() {
        let (who_did, time, kind, serial) = (g.events[at].who, g.events[at].time, g.events[at].kind, g.events[at].serial);
        if who_did == Some(crate::consts::PLAYER_ID) && !(time <= s.t0) && !s.heard.has(serial) {
            s.heard.add(serial);
            add(s, format!("event:{}", kind.as_str()));
        }
        at += 1;
    }
    {
        let len = g.chronicle.events.len() as f64;
        if len < s.chron {
            s.chron = len;
        }
        if len < s.chron0 {
            s.chron0 = len;
        }
        while s.chron < len {
            add(s, format!("chron:{}", g.chronicle.events[s.chron as usize].kind.as_str()));
            s.chron += 1.0;
        }
        if s.held_at < 0.0 {
            let p = who(g);
            if g.pl[p].throw_held {
                s.held_at = len;
            }
        }
    }
    {
        let serials: Vec<u32> = g.orders.log.iter().map(|r| r.serial).collect();
        let mut k = after(&serials, s.last_order);
        while k < g.orders.log.len() {
            add(s, format!("order:{}", g.orders.log[k].order));
            add(s, format!("order:{}@{}", g.orders.log[k].order, g.orders.log[k].target));
            k += 1;
        }
        if let Some(r) = g.orders.log.last() {
            s.last_order = Some(r.serial);
        }
    }
    for &id in &g.door_tools.ops {
        let op = &g.tab.doortools_door_op[id];
        if !s.words.has(op.door) && op.beats.iter().any(|b| b.what == "the word" && b.t > s.t0) {
            s.words.add(op.door);
        }
    }
}

/// Not translated: a clone of Game replaces it (conventions 6).   (lesson.ts Lesson.snapshot)
pub fn lesson_snapshot(g: &mut crate::game::Game) -> crate::lesson::LessonSnap {
    todo!("not translated: a checkpoint is a clone of Game")
}

/// Not translated: a clone of Game replaces it (conventions 6); see notes, Decisions, for what the load must still do.   (lesson.ts Lesson.restore)
pub fn lesson_restore(g: &mut crate::game::Game, s: Option<&crate::lesson::LessonSnap>) {
    todo!("not translated: a checkpoint is a clone of Game")
}

/// The run is over, whichever scenario it was.   (lesson.ts Lesson.over)
pub fn lesson_over(g: &mut crate::game::Game) -> bool {
    let stage = g.mission.stage;
    stage == crate::game::MissionStage::Failed
        || stage == crate::game::MissionStage::Done
        || who_down(g)
        || (crate::roomclear::room_clear_scenario_active(g) && g.room_clear.end.is_some())
}

/// The objective while a step is up (None: the scenario's own).   (lesson.ts Lesson.objective)
pub fn lesson_objective(g: &mut crate::game::Game) -> Option<crate::lesson::LessonObjective> {
    let l = lesson_data(g)?;
    let i = lesson_at(g);
    if i >= l.steps.len() as f64 {
        return None;
    }
    if lesson_over(g) {
        return None;
    }
    let t = &l.steps[i as usize];
    let mut man = None;
    if let Some(name) = filled(&t.at_man) {
        let mut k = 0;
        while k < g.guards.len() {
            let x = g.guards[k];
            let route = g.gd[x].route;
            if g.tab.route[route].name == name && !downed(g, x) {
                man = Some(x);
                break;
            }
            k += 1;
        }
    }
    let line = line_of(g, t);
    let said = lesson_line(g, &line);
    let plain = keys_plain(&said, &mut |k| if k == "Ctrl" { crate::host_keymap::cam_cap(g) } else { String::from(k) });
    let text = format!("training {}/{} \u{25b8} {}", i as usize + 1, l.steps.len(), plain);
    let pos = match man {
        Some(x) => {
            let p = g.ch[g.gd[x].ch].pos;
            Some([p[0], 1.0, p[2]])
        }
        None => t.at,
    };
    Some(crate::lesson::LessonObjective { text, pos })
}

/// Not translated: host work, the debugger's `window.__lesson` alone reads it.   (lesson.ts Lesson.summary)
pub fn lesson_summary(g: &mut crate::game::Game) -> String {
    todo!("not translated: host work")
}

/// One line of the lesson's own log, kept to its last rows.   (lesson.ts Lesson.note)
pub fn lesson_note(g: &mut crate::game::Game, s: &str) {
    let row = format!("{} {}", to_fixed(g.time, 2), s);
    g.lesson.log.push(row);
    if g.lesson.log.len() > 80 {
        g.lesson.log.drain(0..40);
    }
}

/// A step's line as the keys stand.   (lesson.ts lineOf)
pub fn line_of(g: &mut crate::game::Game, t: &crate::scene_scenarios::LessonStep) -> String {
    let line = filled(&t.blow_line).or(filled(&t.ring_line)).or(filled(&t.setup_line)).unwrap_or(t.line.as_str());
    String::from(line)
}

/// A step's line as the page says it.   (lesson.ts stepLine)
pub fn step_line(g: &mut crate::game::Game, t: &crate::scene_scenarios::LessonStep) -> String {
    let line = line_of(g, t);
    lesson_line(g, &line)
}

/// Not translated: tooling, tools/qa/lesson-check.ts and the probes.   (lesson.ts checkLesson)
pub fn check_lesson(g: &mut crate::game::Game) -> Vec<String> {
    todo!("not translated: tooling")
}

/// The taught player is down (`who(g).down`).
fn who_down(g: &mut Game) -> bool {
    let p = who(g);
    g.pl[p].down
}

/// One more of a deed, by its key.   (lesson.ts Lesson.count `add`)
fn add(s: &mut crate::lesson::StepSeen, k: String) {
    let n = s.n.get(k.clone()).copied().unwrap_or(0.0);
    s.n.set(k, n + 1.0);
}

/// An optional string as a truthy test takes it: an empty one is no line.
fn filled(o: &Option<String>) -> Option<&str> {
    o.as_deref().filter(|s| !s.is_empty())
}

/// `line.replace(/\[([^\]]+)\]/g, (_m, k) => say(k))`.   (lesson.ts Lesson.objective)
fn keys_plain(line: &str, say: &mut dyn FnMut(&str) -> String) -> String {
    let mut out = String::new();
    let mut rest = line;
    while let Some(open) = rest.find('[') {
        let tail = &rest[open + 1..];
        match tail.find(']') {
            Some(close) if close > 0 => {
                out.push_str(&rest[..open]);
                out.push_str(&say(&tail[..close]));
                rest = &tail[close + 1..];
            }
            Some(_) => {
                out.push_str(&rest[..open + 1]);
                rest = tail;
            }
            None => break,
        }
    }
    out.push_str(rest);
    out
}

/// `x.toFixed(digits)` for a finite x under 1e21: an exact tie goes away from zero, where Rust's format goes to even, and -0 has no sign.
fn to_fixed(x: f64, digits: usize) -> String {
    let a = m::abs(x);
    let mut scaled = a;
    for _ in 0..=digits {
        scaled *= 2.0;
    }
    // a decimal tie at `digits` places is exactly an odd number over 2^(digits + 1)
    let tie = m::floor(scaled) == scaled && js::rem(scaled, 2.0) == 1.0;
    let a = if tie { f64::from_bits(a.to_bits() + 1) } else { a };
    let s = format!("{:.*}", digits, a);
    if x < 0.0 { format!("-{}", s) } else { s }
}

/// `this.run === g.mission`: the same run, by the Mission's serial.   (lesson.ts:320, 330, 348)
fn run_is_mission(g: &Game) -> bool {
    g.lesson.run == Some(g.mission.serial)
}

/// lesson.ts:342: `passes(g, T.done, S)`.
fn seam_passes(g: &mut Game, t: &crate::scene_scenarios::LessonTestKind, s: &mut crate::lesson::StepSeen) -> bool {
    todo!("needs: passes to take the step's own test (LessonStep.done is scene_scenarios::LessonTestKind, passes takes lesson::LessonTest, which has no Pace band) and s: &mut StepSeen")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Expected strings printed by node from the TypeScript's own expressions.
    #[test]
    fn to_fixed_is_the_languages() {
        let rows: &[(f64, usize, &str)] = &[
            (0.125, 2, "0.13"),
            (0.375, 2, "0.38"),
            (2.5, 0, "3"),
            (0.25, 1, "0.3"),
            (0.75, 1, "0.8"),
            (1.005, 2, "1.00"),
            (-0.125, 2, "-0.13"),
            (-0.001, 2, "-0.00"),
            (-0.0, 2, "0.00"),
            (0.0, 1, "0.0"),
            (12.34, 1, "12.3"),
            (1234.5678, 2, "1234.57"),
            (0.1 + 0.2, 2, "0.30"),
        ];
        for &(x, digits, want) in rows {
            assert_eq!(to_fixed(x, digits), want, "{x} to {digits}");
        }
    }

    #[test]
    fn keys_plain_is_the_regexs() {
        let rows: &[(&str, &str)] = &[
            ("press [B] now", "press <B> now"),
            ("[Ctrl] and [B]", "<Ctrl> and <B>"),
            ("no keys", "no keys"),
            ("[] stays", "[] stays"),
            ("[]x]", "[]x]"),
            ("[[x]", "<[x>"),
            ("open [ only", "open [ only"),
            ("] [a] ]", "] <a> ]"),
            ("[] [a]", "[] <a>"),
            ("\u{25b8} [\u{25b8}]", "\u{25b8} <\u{25b8}>"),
        ];
        for &(line, want) in rows {
            assert_eq!(keys_plain(line, &mut |k| format!("<{k}>")), want, "{line}");
        }
    }
}
