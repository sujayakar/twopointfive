//! src/game/director.ts lines 857 to 1116
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- the seams: one blocked step each, the bodies stay whole

/// `throw new BeatFailure(msg)`: no verb's caller can catch yet (ts 887, 906, 964)
fn seam_throw_beat_failure(msg: String) -> ! { todo!("needs: a way for a verb to fail its beat (BeatFailure caught by resume): {}", msg) }
/// `g.audio.play(name, at, gain, { rate })`: game_audio_like_play's opts hold crate::Opaque, which cannot carry the rate (ts 986)
fn seam_audio_play_rate(g: &mut Game, name: &str, at: V3, gain: f64, rate: f64) { todo!("needs: game_audio_like_play's opts to carry a number") }
/// `gd.flashlight.enabled = false` (ts 909)
fn seam_flashlight_off(g: &mut Game, gd: GuardId) { todo!("needs: an accessor for a light's enabled behind LightId (rust/sim's row number, no accessor in the skeleton)") }
/// `lb.it.return()`: closes another beat's generator (ts 896)
fn seam_close_beat(g: &mut Game, r: &mut Run, name: &str) { todo!("needs: LiveBeat.it is Opaque, a beat's generator has no close") }
/// `() => B.left <= 0` over the burst's own record, which outlives R.bursts (ts 1012)
fn seam_burst_done_test(r: &Run, burst_at: usize) -> crate::FnField { todo!("needs: RunBurstsItem.serial and CondArm2.test as an enum (the goal's ruling on beats)") }

// ---------------------------------------------------------------- small helpers of the context

/// A number as a template literal prints it, for the messages and the log (no exponent form: nothing here is that large)
fn js_num(x: f64) -> String {
    if x.is_nan() { return String::from("NaN"); }
    if x == f64::INFINITY { return String::from("Infinity"); }
    if x == f64::NEG_INFINITY { return String::from("-Infinity"); }
    if x == 0.0 { return String::from("0"); }
    format!("{}", x)
}
/// `+x.toFixed(2)`, for the log's rows only
fn fixed2(x: f64) -> f64 { format!("{:.2}", x).parse::<f64>().unwrap_or(f64::NAN) }
/// a `{ key: value }` of the log's rows, in the written order
fn obj(rows: Vec<(&str, Option<Json>)>) -> Json {
    let mut o = JsMap::new();
    for (k, v) in rows { o.set(String::from(k), v); }
    Json::Arm4(o)
}
fn text(s: &str) -> Option<Json> { Some(Json::Text(String::from(s))) }
fn push(g: &mut Game, beat: &str, kind: DirectorPushRowKind, what: String, data: Option<Json>) {
    director_push(g, &DirectorPushRow { kind, what, data, beat: String::from(beat), t: None });
}
fn unit_ref_of(g: &Game, gd: GuardId) -> UnitRef {
    UnitRef { kind: UnitRefKind::Unit, id: g.ch[g.gd[gd].ch].id, callsign: g.gd[gd].callsign.clone() }
}
fn touch(b: &mut LiveBeat, u: &UnitRef) { b.touched.add(crate::Num(u.id)); }
fn chest_or(g: &Game, gd: GuardId) -> V3 {
    let c = g.gd[gd].ch;
    match g.ch[c].bones.get(crate::character::Bone::Chest) { Some(p) => *p, None => v3::add(g.ch[c].pos, [0.0, 1.1, 0.0]) }
}

/// c.order: his script's patch, a man's chest followed from here (ts 865). Absent stays absent, null stays null. A goal made here is a new leg: `held` stays false.
fn order_now(g: &mut Game, r: &mut Run, b: &mut LiveBeat, u: &UnitRef, o: &OrderPatch2) {
    let gd = director_guard_of(g, r, u);
    touch(b, u);
    let mut patch = crate::squad::OrderPatch::default();
    match &o.goal {
        None => {}
        Some(None) => patch.goal = Some(None),
        Some(Some(goal)) => patch.goal = Some(Some(v3::copy(director_place_of(g, goal)))),
    }
    if let Some(aim) = &o.aim_at {
        r.tracks.retain(|t| t.gd != gd); // any earlier man he was tracking: let go
        match aim {
            None => patch.aim_at = Some(None),
            Some(OrderPatch2AimAt::Unit(t)) => {
                let tgt = director_guard_of(g, r, t);
                let vec = v3::copy(chest_or(g, tgt));
                patch.aim_at = Some(Some(vec));
                r.tracks.push(RunTracksItem { gd, vec, tgt });
            }
            Some(OrderPatch2AimAt::List(p)) => patch.aim_at = Some(Some(v3::copy(director_place_of(g, &Place::List(*p))))),
            Some(OrderPatch2AimAt::Mark(m_)) => patch.aim_at = Some(Some(v3::copy(director_place_of(g, &Place::Mark(m_.clone()))))),
        }
    }
    if o.speed.is_some() { patch.speed = o.speed; }
    if o.face.is_some() { patch.face = o.face; }
    if o.strafe.is_some() { patch.strafe = o.strafe; }
    if o.stance.is_some() { patch.stance = o.stance; }
    if o.stack_side.is_some() { patch.stack_side = o.stack_side; }
    if o.crouch.is_some() { patch.crouch = o.crouch; }
    if o.upper.is_some() { patch.upper = o.upper; }
    if o.light.is_some() { patch.light = o.light; }
    if o.hot.is_some() { patch.hot = o.hot; }
    if o.flinch.is_some() { patch.flinch = o.flinch; }
    if o.direct.is_some() { patch.direct = o.direct; }
    if o.smoke.is_some() { patch.smoke = o.smoke; }
    g.gd[gd].hold = false;
    crate::squad::order(g, gd, patch);
}

// ---- waiting (ts 887)
fn ctx_wait(r: &Run, secs: f64, label: Option<&str>) -> Cond {
    if !secs.is_finite() { seam_throw_beat_failure(format!("wait({}): not a finite number of seconds", js_num(secs))); }
    Cond::Arm0(CondArm0 { k: CondArm0K::Time, until: r.t + m::max(0.0, secs), label: label.map(String::from) })
}
fn ctx_frames(r: &Run, n: Option<f64>) -> Cond {
    let n = n.unwrap_or(1.0);
    Cond::Arm1(CondArm1 { k: CondArm1K::Frames, due: r.tick_n + m::max(1.0, m::floor(n)) })
}
fn ctx_until(test: crate::FnField, label: Option<&str>) -> Cond {
    Cond::Arm2(CondArm2 { k: CondArm2K::Pred, test, label: String::from(label.unwrap_or("until")) })
}
/// c.on: the union `filterOrName` comes resolved, a name or a filter
fn ctx_on(r: &Run, kind: CondArm3Kind, name: Option<&str>, filter: Option<EventFilter>) -> Cond {
    Cond::Arm3(CondArm3 { k: CondArm3K::Event, kind, name: name.map(String::from), filter, got: None, tick0: r.tick_n })
}
/// c.done, of a beat's name (of a drill: its `.beat`)
fn ctx_done(beat: &str) -> Cond { Cond::Arm4(CondArm4 { k: CondArm4K::Beat, name: String::from(beat), state: CondArm4State::Done }) }
fn ctx_started(beat: &str) -> Cond { Cond::Arm4(CondArm4 { k: CondArm4K::Beat, name: String::from(beat), state: CondArm4State::Started }) }
fn ctx_all(of: Vec<Cond>) -> Cond { Cond::Arm5(CondArm5 { k: CondArm5K::All, of }) }
fn ctx_any(of: Vec<Cond>) -> Cond { Cond::Arm5(CondArm5 { k: CondArm5K::Any, of }) }

// ---- flow (ts 895)
fn ctx_start(g: &mut Game, r: &mut Run, b: &LiveBeat, name: &str) {
    push(g, &b.name, DirectorPushRowKind::Trigger, format!("start {}", name), None);
    director_start_beat(g, r, name);
}
/// c.cancel. R.beats and `b` are copies of what the TypeScript holds as one record: both are written. A running beat is the only row of its name
/// (startBeat), so `lb !== b` is a test of the name.
fn ctx_cancel(g: &mut Game, r: &mut Run, b: &mut LiveBeat, name: &str) {
    let own = name == b.name && b.state == BeatState::Running;
    let row = r.beats.iter().position(|lb| lb.name == name);
    let step = if own { b.step } else {
        match row { Some(i) if r.beats[i].state == BeatState::Running => r.beats[i].step, _ => return }
    };
    if let Some(i) = row { r.beats[i].state = BeatState::Cancelled; }
    if own { b.state = BeatState::Cancelled; } else { seam_close_beat(g, r, name); } // (a beat cancelling ITSELF is mid-resume: resume() closes it)
    push(g, name, DirectorPushRowKind::Beat, String::from("cancelled"), Some(obj(vec![("by", text(&b.name)), ("step", Some(Json::Num(step)))])));
}
fn ctx_signal(g: &mut Game, r: &mut Run, b: &LiveBeat, name: &str, data: Option<Json>) {
    r.signals.push(RunSignalsItem { name: String::from(name), t: r.t, data: data.clone() });
    push(g, &b.name, DirectorPushRowKind::Note, format!("signal:{}", name), data);
}
fn ctx_note(g: &mut Game, b: &LiveBeat, what: &str, data: Option<Json>) { push(g, &b.name, DirectorPushRowKind::Note, String::from(what), data); }
fn ctx_finish(b: &mut LiveBeat) { b.flow.finish = true; }
/// c.rand: a draw of the run's own die
fn ctx_rand(r: &mut Run) -> f64 { r.dice.roll() }

// ---- actors (ts 905)
fn ctx_spawn(g: &mut Game, r: &mut Run, b: &mut LiveBeat, l: &LoadoutRef, at: &Place, callsign: Option<&str>, yaw: Option<f64>, light: Option<bool>) -> UnitRef {
    let spec = match g.director.manifest.as_ref().unwrap().loadouts.get(l.id.clone()) { Some(s) => *s, None => seam_throw_beat_failure(format!("no loadout '{}'", l.id)) };
    let p = director_place_of(g, at);
    let yaw = match yaw {
        Some(y) => y,
        None => match at {
            Place::List(_) => 0.0,
            Place::Mark(mk) => g.director.manifest.as_ref().unwrap().marks.get(mk.id.clone()).and_then(|v| v.yaw).unwrap_or(0.0),
        },
    };
    let lit = light.or(spec.light);
    let budget = lifted_budget(g);
    let gd = crate::guards::spawn_unit(g, &crate::guards::UnitOpts { team: spec.team, pos: [p[0], 0.0, p[2]], yaw: Some(yaw), weapon: spec.weapon, tint: spec.tint, tint2: spec.tint2, name: callsign.map(String::from), light: lit, budget, ..Default::default() });
    if lit == Some(false) { seam_flashlight_off(g, gd); } // dark from the first frame
    if spec.trained == Some(true) { g.gd[gd].trained = true; }
    if spec.god == Some(true) && r.show.is_some() { g.gd[gd].god = true; } // (only a SHOWCASE run's cast can carry the net)
    let ref_ = unit_ref_of(g, gd);
    r.units.set(crate::Num(ref_.id), UnitRec { ref_: ref_.clone(), gd, loadout: l.id.clone(), by: b.name.clone() });
    touch(b, &ref_);
    let what = g.gd[gd].callsign.clone();
    push(g, &b.name, DirectorPushRowKind::Spawn, what, Some(obj(vec![
        ("id", Some(Json::Num(ref_.id))),
        ("loadout", text(&l.id)),
        ("team", text(spec.team.as_str())),
        ("at", Some(Json::Array(vec![Some(Json::Num(fixed2(p[0]))), Some(Json::Num(0.0)), Some(Json::Num(fixed2(p[2])))]))),
    ])));
    ref_
}
fn ctx_despawn(g: &mut Game, r: &mut Run, b: &LiveBeat, u: &UnitRef) {
    let gd = match r.units.get(crate::Num(u.id)) { Some(rec) => rec.gd, None => return };
    crate::guards::despawn_unit(g, gd);
    r.units.delete(crate::Num(u.id));
    push(g, &b.name, DirectorPushRowKind::Spawn, format!("despawn {}", u.callsign), None);
}
fn ctx_unit(g: &Game, r: &Run, callsign: &str) -> Option<UnitRef> {
    let mut found = g.guards.iter().copied().find(|x| g.gd[*x].callsign == callsign);
    if found.is_none() {
        if let Some(benched) = r.show.as_ref().and_then(|s| s.benched.as_ref()) { found = benched.iter().copied().find(|x| g.gd[*x].callsign == callsign); }
    }
    found.map(|gd| unit_ref_of(g, gd))
}
fn ctx_arrived(g: &mut Game, r: &Run, u: &UnitRef) -> bool {
    let gd = director_guard_of(g, r, u);
    g.gd[gd].script.as_ref().and_then(|s| s.arrived).unwrap_or(false)
}
fn ctx_show(g: &mut Game, r: &Run, b: &LiveBeat, u: &UnitRef, on: Option<bool>) {
    let on = on.unwrap_or(true);
    let gd = director_guard_of(g, r, u);
    director_show_man(g, gd, on);
    push(g, &b.name, DirectorPushRowKind::Note, format!("{} {}", if on { "show" } else { "unshow" }, u.callsign), None);
}
fn ctx_rout(g: &mut Game, r: &Run, b: &mut LiveBeat, u: &UnitRef) -> bool {
    if r.units.has(crate::Num(u.id)) { return false; }
    let gd = director_guard_of(g, r, u);
    if g.gd[gd].state == crate::game::GuardState::Dead { return false; }
    if crate::runner::runner_owns(g, gd) { return true; }
    if !crate::runner::runner_break(g, gd) { return false; } // (asked while he is still in this beat's hand: refused, he is exactly as he was)
    if g.gd[gd].script.is_some() { crate::squad::end_guard_script(g, gd, Some(crate::squad::EndGuardScriptState::Alert)); }
    g.gd[gd].hold = false;
    director_show_man(g, gd, false);
    b.touched.delete(crate::Num(u.id));
    push(g, &b.name, DirectorPushRowKind::Note, format!("rout {}", u.callsign), None);
    true
}
fn ctx_release(g: &mut Game, r: &Run, b: &mut LiveBeat, u: &UnitRef, state: Option<crate::squad::EndGuardScriptState>) {
    let state = state.unwrap_or(crate::squad::EndGuardScriptState::Search);
    let gd = director_guard_of(g, r, u);
    if g.gd[gd].script.is_some() { crate::squad::end_guard_script(g, gd, Some(state)); }
    g.gd[gd].hold = false;
    director_show_man(g, gd, false);
    b.touched.delete(crate::Num(u.id));
    push(g, &b.name, DirectorPushRowKind::Note, format!("release {}", u.callsign), Some(obj(vec![("state", text(state.as_str()))])));
}

// ---- speech (ts 955). c.say('sam', …) is showcase-only: not translated
fn ctx_say(g: &mut Game, r: &Run, b: &LiveBeat, u: &UnitRef, line: &str, radio: Option<bool>) {
    let radio = radio.unwrap_or(false);
    let gd = director_guard_of(g, r, u);
    if !line.is_empty() && g.gd[gd].state != crate::game::GuardState::Dead {
        crate::guards::say(g, gd, line, Some(radio), Some(true), None); // (forced: a scripted line is deliberate)
        let what = format!("{}{}: {}", g.gd[gd].callsign, if radio { "" } else { " (says)" }, line);
        push(g, &b.name, DirectorPushRowKind::Radio, what, None);
    }
}
fn ctx_learn_code(g: &mut Game, r: &Run, b: &LiveBeat, dev: &DeviceRef, from: Option<&UnitRef>) -> bool {
    if !g.devices.keypads.iter().any(|k| k.def.name == dev.id) { seam_throw_beat_failure(format!("device '{}' is not a keypad", dev.id)); }
    let gd = match from { Some(u) => Some(director_guard_of(g, r, u)), None => None };
    let news = crate::devices::learn_code(g, &dev.id, crate::devices::LearnCodeHow::Told, gd);
    let from_name = match gd { Some(x) => text(&g.gd[x].callsign), None => None };
    push(g, &b.name, DirectorPushRowKind::Note, format!("code {}", dev.id), Some(obj(vec![("from", from_name), ("news", Some(Json::Bool(news)))])));
    news
}
/// c.radio: a line on the net under a name, said by that man if he stands in the world, else by the voice alone
fn ctx_radio(g: &mut Game, b: &LiveBeat, callsign: &str, line: &str) {
    let gd = g.guards.iter().copied().find(|x| g.gd[*x].callsign == callsign && g.gd[*x].state != crate::game::GuardState::Dead);
    if let Some(x) = gd {
        let dur = crate::guards::bubble_secs(line);
        g.gd[x].bubble = Some(crate::game::GuardBubble { text: String::from(line), t: g.time, dur, radio: true });
        crate::handwork::radio_hand(g, x, dur);
        crate::chronicle::chron_radio(g, x, line);
    }
    crate::game::game_msg(g, &format!("radio: {} — {}", callsign, line));
    let at = gd.map(|x| { let c = g.gd[x].ch; match g.ch[c].bones.get(crate::character::Bone::Head) { Some(p) => *p, None => g.ch[c].pos } });
    crate::audio_iface::game_audio_like_play(g, "radio", at, Some(if gd.is_some() { 0.7 } else { 0.55 }), None);
    push(g, &b.name, DirectorPushRowKind::Radio, format!("{}: {}", callsign, line), None);
}
fn ctx_control(g: &mut Game, b: &LiveBeat, line: &str) {
    crate::game::game_msg(g, &format!("radio: CONTROL — {}", line));
    crate::audio_iface::game_audio_like_play(g, "radio", None, Some(0.55), None);
    push(g, &b.name, DirectorPushRowKind::Radio, format!("CONTROL: {}", line), None);
}

// ---- world: lights (ts 978)
fn ctx_lights_group(g: &mut Game, b: &LiveBeat, gr: &GroupRef, on: bool) {
    crate::lighting::set_group(g, &gr.id, on);
    push(g, &b.name, DirectorPushRowKind::Stimulus, format!("group {} {}", gr.id, if on { "on" } else { "off" }), None);
}
fn ctx_lights_mains(g: &mut Game, b: &LiveBeat, on: bool) {
    if on { crate::game::game_end_blackout(g); } else { crate::game::game_set_blackout(g, 1e6, crate::game::GameSetBlackoutCause::Hand); }
    push(g, &b.name, DirectorPushRowKind::Stimulus, format!("mains {}", if on { "on" } else { "off" }), None);
}
fn ctx_lights_blackout(g: &mut Game, b: &LiveBeat, secs: f64) {
    crate::game::game_set_blackout(g, secs, crate::game::GameSetBlackoutCause::Hand);
    push(g, &b.name, DirectorPushRowKind::Stimulus, format!("blackout {}s", js_num(secs)), None);
}

// ---- world: doors (ts 983)
fn ctx_doors_lock(g: &mut Game, d: &DoorRef, on: bool) {
    let door = director_door_of(g, d);
    crate::doors::door_set_locked(g, door, on);
    if on { crate::doors::door_set_lock_broken(g, door, false); }
}
fn ctx_doors_open(g: &mut Game, d: &DoorRef, angle: Option<f64>, hold: Option<bool>) {
    let door = director_door_of(g, d);
    crate::doors::door_set_latched(g, door, false);
    crate::doors::door_set_closing(g, door, false);
    match angle {
        Some(a) => { crate::doors::door_set_angle(g, door, a); crate::doors::door_set_vel(g, door, 0.0); crate::doors::door_place(g, door); }
        None => {
            let sign = js::or(m::sign(g.dr[door].def.angle.unwrap_or(1.0)), 1.0);
            crate::doors::door_set_vel(g, door, sign * 3.2);
        }
    }
    if hold == Some(true) { crate::doors::door_set_quiet_t(g, door, -3600.0); } // (an hour of quiet is a leaf on its stay; Door.reset puts it back)
}
/// c.doors.fling (qrf.ts flingOpen: the crash-bar side, the bolt as it was)
fn ctx_doors_fling(g: &mut Game, r: &Run, b: &LiveBeat, d: &DoorRef, by: &UnitRef) {
    let door = director_door_of(g, d);
    let gd = director_guard_of(g, r, by);
    if m::abs(crate::doors::door_angle(g, door)) > 1.0 { return; }
    let c = g.gd[gd].ch;
    let side = crate::doors::doors_side(g, door, g.ch[c].pos);
    crate::doors::door_set_latched(g, door, false);
    crate::doors::door_set_closing(g, door, false);
    crate::doors::door_set_vel(g, door, -side * 7.5);
    crate::doors::door_set_since_bash(g, door, 0.0);
    crate::doors::door_set_quiet_t(g, door, -0.5);
    crate::doors::door_set_creak_armed(g, door, false);
    let who = g.ch[c].id;
    crate::doors::door_set_last_who(g, door, who);
    let at = crate::doors::door_pos(g, door);
    seam_audio_play_rate(g, "doorBash", at, 0.9, 0.85);
    push(g, &b.name, DirectorPushRowKind::Stimulus, format!("fling {}", d.id), Some(obj(vec![("by", text(&by.callsign))])));
}
fn ctx_doors_shut(g: &mut Game, d: &DoorRef) {
    let door = director_door_of(g, d);
    if !crate::doors::door_latched(g, door) {
        crate::doors::door_set_closing(g, door, true);
        let v = -m::sign(crate::doors::door_angle(g, door)) * 1.2;
        crate::doors::door_set_vel(g, door, v);
    }
}
fn ctx_doors_latch(g: &mut Game, d: &DoorRef) {
    let door = director_door_of(g, d);
    crate::doors::door_reset(g, door);
    crate::doors::door_set_angle(g, door, 0.0);
    crate::doors::door_set_vel(g, door, 0.0);
    crate::doors::door_set_latched(g, door, true);
    crate::doors::door_set_closing(g, door, false);
    crate::doors::door_place(g, door);
}
/// c.doors.kick: a CAUSE: the animator's kick now, doors.ts kickIn on its impact frame (tickKicks)
fn ctx_doors_kick(g: &mut Game, r: &mut Run, b: &mut LiveBeat, d: &DoorRef, by: &UnitRef, leg: Option<crate::character::AnimKickDoorOptsLeg>) {
    let door = director_door_of(g, d);
    let gd = director_guard_of(g, r, by);
    touch(b, by);
    if g.gd[gd].state == crate::game::GuardState::Dead || r.kicks.iter().any(|k| k.gd == gd) { return; }
    let leg = leg.unwrap_or(crate::character::AnimKickDoorOptsLeg::Right);
    crate::squad::order(g, gd, crate::squad::OrderPatch { goal: Some(None), aim_at: Some(None), ..Default::default() });
    let c = g.gd[gd].ch;
    crate::character::anim_kick_door(g, c, Some(crate::character::AnimKickDoorOpts { leg: Some(leg) }));
    r.kicks.push(RunKicksItem { door, gd, by: by.callsign.clone(), beat: b.name.clone() });
    push(g, &b.name, DirectorPushRowKind::Stimulus, format!("kick:start {}", d.id), Some(obj(vec![("by", text(&by.callsign)), ("leg", text(leg.as_str()))])));
}
fn ctx_doors_kick_spot(g: &mut Game, r: &Run, d: &DoorRef, by: &UnitRef) -> crate::doors::DoorsKickSpot {
    let door = director_door_of(g, d);
    let gd = director_guard_of(g, r, by);
    let c = g.gd[gd].ch;
    let s = crate::doors::doors_kick_spot(g, door, g.ch[c].pos);
    crate::doors::DoorsKickSpot { pos: v3::copy(s.pos), yaw: s.yaw }
}

// ---- world: stimulus (ts 998)
/// c.stimulus.charge: a CAUSE, never a result: the item's own fuse detonates it
fn ctx_stimulus_charge(g: &mut Game, r: &Run, b: &LiveBeat, at: &Place, by: &UnitRef, kind: Option<crate::throwables::ItemKind>, fuse: Option<f64>, toss: Option<bool>) {
    let gd = director_guard_of(g, r, by);
    let c = g.gd[gd].ch;
    let hand = match crate::character::character_joint_under(g, c, "handL") { Some(h) => h, None => v3::add(g.ch[c].pos, [0.0, 1.3, 0.0]) };
    let from: V3 = [hand[0], m::max(1.0, hand[1]), hand[2]];
    let to = director_place_of(g, at);
    let sol = crate::throwables::throwables_solve(g, from, [to[0], 0.0, to[2]], Some(9.0));
    let kind = kind.unwrap_or(crate::throwables::ItemKind::Flash);
    let who = g.ch[c].id;
    let it = crate::throwables::throwables_spawn(g, kind, from, sol.v0, Some(crate::throwables::ThrowablesSpawnOpts { fuse: Some(fuse.unwrap_or(1.2)), who: Some(who), heading: Some(v3::yaw_of(sol.v0)), ..Default::default() }));
    g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::Can);
    crate::audio_iface::game_audio_like_play(g, "throw", Some(from), Some(0.7), None);
    if toss == Some(true) { crate::character::anim_toss_item(g, c, Some(crate::character::TOSS_QUICK)); }
    push(g, &b.name, DirectorPushRowKind::Stimulus, format!("charge {}", kind.as_str()), Some(obj(vec![
        ("by", text(&by.callsign)),
        ("at", Some(Json::Array(vec![Some(Json::Num(fixed2(to[0]))), Some(Json::Num(0.0)), Some(Json::Num(fixed2(to[2])))]))),
    ])));
}
fn ctx_stimulus_cease(r: &mut Run, u: &UnitRef) {
    for x in r.bursts.iter_mut() { if x.u.id == u.id { x.left = 0.0; } }
}
fn ctx_stimulus_burst(g: &mut Game, r: &mut Run, b: &mut LiveBeat, u: &UnitRef, at: RunBurstsItemAt, rounds: f64, cadence: Option<f64>, spread: Option<f64>) -> Cond {
    director_guard_of(g, r, u);
    touch(b, u);
    let label = format!("burst {} ×{}", u.callsign, js_num(rounds));
    let burst_at = r.bursts.len();
    let test = seam_burst_done_test(r, burst_at);
    r.bursts.push(RunBurstsItem {
        u: u.clone(),
        at,
        left: m::max(0.0, m::floor(rounds)),
        cadence: m::max(0.05, cadence.unwrap_or(0.16)),
        spread: spread.unwrap_or(0.01),
        next: r.t,
        cond: RunBurstsItemCond { k: RunBurstsItemCondK::Pred, label: label.clone() },
        fired: 0.0,
    });
    push(g, &b.name, DirectorPushRowKind::Stimulus, format!("burst ×{}", js_num(rounds)), Some(obj(vec![("by", text(&u.callsign))])));
    Cond::Arm2(CondArm2 { k: CondArm2K::Pred, test, label })
}

// ---- objective (ts 1015). `markAt` comes resolved: a place or a door
fn ctx_objective(g: &mut Game, r: &mut Run, b: &LiveBeat, text_: Option<&str>, mark_place: Option<&Place>, mark_door: Option<&DoorRef>) {
    let t = match text_ { Some(t) => t, None => { r.objective = None; return; } };
    let pos = if let Some(d) = mark_door {
        let door = director_door_of(g, d);
        Some(v3::copy(g.dr[door].frame_centre))
    } else if let Some(p) = mark_place {
        Some(v3::copy(director_place_of(g, p)))
    } else { None };
    r.objective = Some(RunObjective { text: String::from(t), pos });
    crate::chronicle::chron_objective(g, t, pos);
    crate::game::game_msg(g, t);
    push(g, &b.name, DirectorPushRowKind::Note, format!("objective: {}", t), None);
}
/// c.chronicle('objective', text)
fn ctx_chronicle(g: &mut Game, text_: &str) {
    let mut data = JsMap::new();
    data.set(String::from("text"), Some(crate::chronicle::ChronicleDataValue::Text(String::from(text_))));
    crate::chronicle::chronicle(g, crate::chronicle::ChronKind::Objective, Some(&data), None);
}

// ---------------------------------------------------------------- the part's functions

/// The context a beat is handed: what of it is data. Its verbs are the `ctx_*` above. (director.ts Director.ctxOf)
pub fn director_ctx_of(g: &mut crate::game::Game, r: &mut crate::director::Run, b: &mut crate::director::LiveBeat) -> crate::director::BeatCtx {
    let flag = r.flags.clone();
    let query = director_query_of(g, r);
    BeatCtx {
        flag,
        query,
        dt: r.dt,
        view: SamView::zeroed(), // (a getter of the showcase's only, it throws in a play program: never read there)
        camera: BeatCtxCamera { shot_t: r.show.as_ref().and_then(|s| s.shot.as_ref()).map(|s| s.t).unwrap_or(0.0) },
    }
}

/// What the log says a beat waits on; empty = nothing worth a row. (director.ts condLabel)
pub fn cond_label(c_2: &crate::director::Cond) -> String {
    match c_2 {
        Cond::Arm0(c) => c.label.clone().unwrap_or_default(),
        Cond::Arm1(_) => String::new(),
        Cond::Arm2(c) => if c.label == "until" { String::new() } else { c.label.clone() },
        Cond::Arm3(c) => match &c.name {
            Some(n) if !n.is_empty() => format!("on {}:{}", c.kind.as_str(), n),
            _ => format!("on {}", c.kind.as_str()),
        },
        Cond::Arm4(c) => format!("{} {}", if c.state == CondArm4State::Done { "done" } else { "started" }, c.name),
        Cond::Arm5(c) => {
            let mut parts: Vec<String> = Vec::new();
            for x in c.of.iter() { let l = cond_label(x); if !l.is_empty() { parts.push(l); } }
            if parts.is_empty() { String::new() } else { format!("{}({})", c.k.as_str(), parts.join(", ")) }
        }
    }
}

/// `near(what, within)` of an event at `pos`: within metres of a door's frame (a door that does not resolve: false) or a place. Pass one of the two.
fn near_test(g: &mut Game, pos: V3, door: Option<&DoorRef>, place: Option<&Place>, within: f64) -> bool {
    let p = if let Some(d) = door {
        crate::doors::doors_by_name(g, &d.id).map(|x| g.dr[x].frame_centre)
    } else {
        place.map(|pl| director_place_of(g, pl))
    };
    match p { Some(p) => m::hyp2(p[0] - pos[0], p[2] - pos[2]) <= within, None => false }
}
/// The event view's `near`: the closure it returned is near_test's body, and with no `what` and `within` here there is nothing to answer. (director.ts nearOf)
pub fn near_of(g: &mut crate::game::Game, pos: V3) {
    todo!("needs: near_of(g, pos, what, within) -> bool, as review 1 asked: the skeleton dropped its return and kept no arguments")
}

fn who_of(g: &Game, r: Option<&crate::game::Interactable>) -> Option<String> {
    let r = r?;
    if let Some(x) = r.guard { return Some(g.gd[x].callsign.clone()); }
    if let Some(x) = r.door { return Some(g.dr[x].def.name.clone()); }
    if let Some(x) = r.target { return Some(g.tab.sim_lightshome_fixture[x].name.clone()); }
    None
}
fn door_name(g: &Game, d: Option<DoorId>) -> Option<String> { d.map(|x| g.dr[x].def.name.clone()) }

/// The read model of Sam (c.view): a fresh snapshot of what his own HUD shows. (director.ts samViewOf)
pub fn sam_view_of(g: &mut crate::game::Game) -> crate::director::SamView {
    let pl = g.player;
    let c = g.pl[pl].ch;
    let hover = g.hover.as_ref().map(|hv| SamViewHover {
        kind: String::from(hv.kind.as_str()),
        verb: hv.verb.map(|v| String::from(v.as_str())),
        in_reach: hv.in_reach,
        who: who_of(g, Some(hv)),
        running: hv.running.unwrap_or(false),
    });
    let mut rows = Vec::new();
    for x in g.nearby.iter() {
        rows.push(format!("{}:{}{}", x.kind.as_str(), x.verb.map(|v| v.as_str()).unwrap_or(""), if x.in_reach { "" } else { "(far)" }));
    }
    let holding = g.pl[pl].holding.as_ref().map(|h| SamViewHolding {
        who: g.gd[h.g].callsign.clone(),
        phase: String::from(h.phase.as_str()),
        variant: String::from(h.variant.as_str()),
        t: h.t,
    });
    SamView {
        pos: v3::copy(g.ch[c].pos),
        yaw: g.ch[c].body_yaw,
        speed: m::hyp2(g.ch[c].vel[0], g.ch[c].vel[2]),
        crouch: g.pl[pl].crouch,
        holstered: g.pl[pl].holstered,
        slot: String::from(g.pl[pl].slot.as_str()),
        nv: g.pl[pl].nv,
        hover,
        rows,
        holding,
        dragging: g.pl[pl].dragging.map(|x| g.gd[x].callsign.clone()),
        takedown: g.pl[pl].takedown.is_some(),
        easing: door_name(g, g.pl[pl].easing),
        picking: door_name(g, g.pl[pl].picking),
        coding: door_name(g, g.pl[pl].coding),
        drive_t: g.mission.drive_t,
        pulled: g.mission.pulled,
        stage: String::from(g.mission.stage.as_str()),
        codes: g.pl[pl].codes.keys(),
        keys: g.pl[pl].keys.keys(),
        aim_target: g.aim_target.map(|x| g.tab.sim_lightshome_fixture[x].name.clone()),
        aim_guard: g.aim_guard.map(|x| g.gd[x].callsign.clone()),
        ocp: g.pl[pl].ocp.charge,
        canisters: g.pl[pl].canisters,
        flashbangs: g.pl[pl].flashbangs,
        door_push: match &g.ch[c].anim.door_push { Some(p) if p.w > 0.0 => Some(String::from("leaf")), _ => None },
    }
}

fn view_kind(k: crate::director::EventKind) -> EventViewKind {
    match k {
        EventKind::Shot => EventViewKind::Shot,
        EventKind::Bang => EventViewKind::Bang,
        EventKind::Kick => EventViewKind::Kick,
        EventKind::Door => EventViewKind::Door,
        EventKind::LightOut => EventViewKind::LightOut,
        EventKind::GuardShot => EventViewKind::GuardShot,
        EventKind::Step => EventViewKind::Step,
        EventKind::Prop => EventViewKind::Prop,
        EventKind::Whistle => EventViewKind::Whistle,
        EventKind::Glass => EventViewKind::Glass,
    }
}
/// What a script sees of a hearing event. (director.ts viewOf)
pub fn view_of(g: &mut crate::game::Game, r: &crate::director::Run, e: crate::game::GameEvent) -> crate::director::EventView {
    EventView {
        kind: view_kind(e.kind),
        pos: v3::copy(e.pos),
        level: e.level.unwrap_or(if e.loud { 1.0 } else { 0.3 }),
        loud: e.loud,
        who: e.who.unwrap_or(-1.0),
        name: None,
        data: None,
    }
}
/// What a script sees of a signal: where the player stands. (director.ts signalView)
pub fn signal_view(g: &mut crate::game::Game, r: &crate::director::Run, s: &crate::director::SignalViewS) -> crate::director::EventView {
    let c = g.pl[g.player].ch;
    EventView {
        kind: EventViewKind::Signal,
        pos: v3::copy(g.ch[c].pos),
        level: 0.0,
        loud: false,
        who: -1.0,
        name: Some(s.name.clone()),
        data: s.data.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn time(label: Option<&str>) -> Cond { Cond::Arm0(CondArm0 { k: CondArm0K::Time, until: 1.0, label: label.map(String::from) }) }
    fn pred(label: &str) -> Cond { Cond::Arm2(CondArm2 { k: CondArm2K::Pred, test: crate::FnField, label: String::from(label) }) }

    #[test]
    fn labels_as_the_log_writes_them() {
        assert_eq!(cond_label(&time(None)), "");
        assert_eq!(cond_label(&time(Some("t"))), "t");
        assert_eq!(cond_label(&Cond::Arm1(CondArm1 { k: CondArm1K::Frames, due: 1.0 })), "");
        assert_eq!(cond_label(&pred("until")), "");
        assert_eq!(cond_label(&pred("door shut")), "door shut");
        let mut on = CondArm3 { k: CondArm3K::Event, kind: CondArm3Kind::Signal, name: Some(String::from("go")), filter: None, got: None, tick0: 0.0 };
        assert_eq!(cond_label(&Cond::Arm3(on.clone())), "on signal:go");
        on.name = Some(String::new());
        assert_eq!(cond_label(&Cond::Arm3(on.clone())), "on signal");
        on.name = None; on.kind = CondArm3Kind::LightOut;
        assert_eq!(cond_label(&Cond::Arm3(on)), "on lightOut");
        assert_eq!(cond_label(&ctx_done("a")), "done a");
        assert_eq!(cond_label(&ctx_started("b")), "started b");
        assert_eq!(cond_label(&ctx_all(vec![time(None), ctx_done("a"), pred("x")])), "all(done a, x)");
        assert_eq!(cond_label(&ctx_any(vec![time(None), pred("until")])), "");
    }

    #[test]
    fn numbers_as_the_log_writes_them() {
        assert_eq!(js_num(3.0), "3");
        assert_eq!(js_num(1.5), "1.5");
        assert_eq!(js_num(-0.0), "0");
        assert_eq!(js_num(f64::NAN), "NaN");
        assert_eq!(js_num(f64::NEG_INFINITY), "-Infinity");
        assert_eq!(fixed2(1.236).to_bits(), 1.24f64.to_bits());
        assert_eq!(fixed2(-0.001).to_bits(), (-0.0f64).to_bits());
    }

    /// the run's die against the TypeScript's own bits: math/dice.ts rollOn on { dice: -5 }, then the word it leaves
    #[test]
    fn the_run_die_is_roll_on() {
        let mut r = Run::zeroed();
        r.dice = crate::dice::Die::new(js::to_u32(-5.0));
        let want: [(u64, f64); 3] = DIE_BITS;
        for (bits, word) in want {
            assert_eq!(ctx_rand(&mut r).to_bits(), bits);
            assert_eq!((r.dice.s as i32) as f64, word);
        }
    }
    const DIE_BITS: [(u64, f64); 3] = [(0x3fdef75a33800000, 1831565808.0), (0x3fab1e8e36000000, -631835675.0), (0x3fee0c8f6ca00000, 1199730138.0)];
}
