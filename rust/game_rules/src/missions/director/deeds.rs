use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js;
use crate::m;
use crate::v3::{self, V3};
use alloc::{format, string::String, vec::Vec};

// ---------------------------------------------------------------- small helpers of the context

fn unit_ref_of(g: &Game, gd: GuardId) -> UnitRef {
    UnitRef { id: g.char_id(gd), callsign: g.gd[gd].callsign.clone() }
}
fn touch(g: &mut Game, b: DirectorLiveBeatId, u: &UnitRef) { g.tab.director_live_beat[b].touched.add(crate::Num(u.id)); }

/// c.order: his script's patch. Absent stays absent, null stays null. A goal made here is a new leg: `held` stays false.
pub(in crate::missions) fn director_beat_order(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &crate::director::UnitRef, o: &crate::director::OrderPatch2) -> Result<(), String> {
    let gd = director_guard_of(g, r, u)?;
    touch(g, b, u);
    let mut patch = crate::squad::OrderPatch::default();
    match &o.goal {
        None => {}
        Some(None) => patch.goal = Some(None),
        Some(Some(goal)) => patch.goal = Some(Some(v3::copy(director_place_of(g, goal)?))),
    }
    // (ts 871's R.tracks filter has nothing to drop: no script aims at a unit, so R.tracks stays empty and Run has no such field. The Unit arm of ts 873 went with it)
    match &o.aim_at {
        None => {}
        Some(None) => patch.aim_at = Some(None),
        Some(Some(aim)) => patch.aim_at = Some(Some(v3::copy(director_place_of(g, aim)?))),
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
    crate::duty::hold(g, gd, false, "director");
    crate::squad::order(g, gd, patch);
    Ok(())
}

// ---- waiting (ts 888)
/// c.wait
pub(in crate::missions) fn director_beat_wait(g: &crate::game::Game, r: DirectorRunId, secs: f64) -> Result<crate::director::Cond, String> {
    if !secs.is_finite() { return Err(format!("wait({}): not a finite number of seconds", js::num_str(secs))); }
    Ok(Cond::Arm0(CondArm0 { until: g.tab.director_run[r].t + m::max(0.0, secs) }))
}
/// c.frames
pub(in crate::missions) fn director_beat_frames(g: &crate::game::Game, r: DirectorRunId, n: Option<f64>) -> crate::director::Cond {
    let n = n.unwrap_or(1.0);
    Cond::Arm1(CondArm1 { due: g.tab.director_run[r].tick_n + m::max(1.0, m::floor(n)) })
}
/// c.until
pub(in crate::missions) fn director_beat_until(test: crate::director::CondTest) -> crate::director::Cond {
    Cond::Arm2(CondArm2 { test })
}
/// c.on: the union `filterOrName` comes resolved, a name or a filter
pub(in crate::missions) fn director_beat_on(g: &crate::game::Game, r: DirectorRunId, kind: crate::director::EventViewKind, name: Option<&str>, filter: Option<crate::director::EventFilter>) -> crate::director::Cond {
    Cond::Arm3(CondArm3 { kind, name: name.map(String::from), filter, got: None, tick0: g.tab.director_run[r].tick_n })
}
/// c.all
pub(in crate::missions) fn director_beat_all(of: Vec<crate::director::Cond>) -> crate::director::Cond { Cond::Arm5(CondArm5 { k: CondArm5K::All, of }) }
/// c.any
pub(in crate::missions) fn director_beat_any(of: Vec<crate::director::Cond>) -> crate::director::Cond { Cond::Arm5(CondArm5 { k: CondArm5K::Any, of }) }

// ---- flow (ts 896)
/// c.start
pub(in crate::missions) fn director_beat_start(g: &mut crate::game::Game, r: DirectorRunId, name: &str) {
    director_start_beat(g, r, name);
}
/// c.signal
pub(in crate::missions) fn director_beat_signal(g: &mut crate::game::Game, r: DirectorRunId, name: &str) {
    g.tab.director_run[r].signals.push(RunSignalsItem { name: String::from(name) });
}
/// c.rand: a draw of the run's own die
pub(in crate::missions) fn director_beat_rand(g: &mut crate::game::Game, r: DirectorRunId) -> f64 { run_rand(&mut g.tab.director_run[r]) }
fn run_rand(run: &mut Run) -> f64 { run.dice.roll() }

// ---- actors (ts 906)
/// c.spawn
pub(in crate::missions) fn director_beat_spawn(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, l: &crate::director::LoadoutRef, at: &crate::director::Place, callsign: Option<&str>, yaw: Option<f64>, light: Option<bool>) -> Result<crate::director::UnitRef, String> {
    let spec = match g.director.manifest.as_ref().unwrap().loadouts.get(l.id.clone()) { Some(s) => *s, None => return Err(format!("no loadout '{}'", l.id)) };
    let p = director_place_of(g, at)?;
    let yaw = match yaw {
        Some(y) => y,
        None => match at {
            Place::List(_) => 0.0,
            Place::Mark(mk) => g.director.manifest.as_ref().unwrap().marks.get(mk.id.clone()).and_then(|v| v.yaw).unwrap_or(0.0),
        },
    };
    let lit = light.or(spec.light);
    let budget = lifted_budget(g);
    let gd = crate::guards::spawn_unit(g, &crate::guards::UnitOpts { team: spec.team, pos: [p[0], 0.0, p[2]], yaw: Some(yaw), weapon: spec.weapon, tint: spec.tint, tint2: spec.tint2, name: callsign.map(String::from), light: lit, budget, ..Default::default() })?;
    if lit == Some(false) { let l = g.gd[gd].flashlight; g.tab.light[l].enabled = false; } // dark from the first frame
    if spec.trained == Some(true) { g.gd[gd].trained = true; }
    let ref_ = unit_ref_of(g, gd);
    g.tab.director_run[r].units.set(crate::Num(ref_.id), UnitRec { gd });
    touch(g, b, &ref_);
    Ok(ref_)
}
/// c.unit
pub(in crate::missions) fn director_beat_unit(g: &crate::game::Game, callsign: &str) -> Option<crate::director::UnitRef> {
    let found = g.guards.iter().copied().find(|x| g.gd[*x].callsign == callsign);
    found.map(|gd| unit_ref_of(g, gd))
}
/// c.rout
pub(in crate::missions) fn director_beat_rout(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &crate::director::UnitRef) -> Result<bool, String> {
    if g.tab.director_run[r].units.has(crate::Num(u.id)) { return Ok(false); }
    let gd = director_guard_of(g, r, u)?;
    if crate::condition::down(g, g.gd[gd].ch) { return Ok(false); }
    if crate::owner::has(g, gd, crate::owner::by::RUNNER) { return Ok(true); }
    if !crate::runner::runner_break(g, gd) { return Ok(false); } // (asked while he is still in this beat's hand: refused, he is exactly as he was)
    if g.gd[gd].script.is_some() { crate::squad::end_guard_script(g, gd, Some(crate::game::GuardState::Alert)); }
    crate::duty::hold(g, gd, false, "director");
    g.tab.director_live_beat[b].touched.delete(crate::Num(u.id));
    Ok(true)
}
/// c.release
pub(in crate::missions) fn director_beat_release(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &crate::director::UnitRef, state: Option<crate::game::GuardState>) -> Result<(), String> {
    let state = state.unwrap_or(crate::game::GuardState::Search);
    let gd = director_guard_of(g, r, u)?;
    if g.gd[gd].script.is_some() { crate::squad::end_guard_script(g, gd, Some(state)); }
    crate::duty::hold(g, gd, false, "director");
    g.tab.director_live_beat[b].touched.delete(crate::Num(u.id));
    Ok(())
}

// ---- speech
/// c.say of a man
pub(in crate::missions) fn director_beat_say(g: &mut crate::game::Game, r: DirectorRunId, u: &crate::director::UnitRef, line: &str, radio: Option<bool>) -> Result<(), String> {
    let radio = radio.unwrap_or(false);
    let gd = director_guard_of(g, r, u)?;
    if !line.is_empty() && !crate::condition::down(g, g.gd[gd].ch) {
        crate::guards::say(g, gd, line, Some(radio), Some(true)); // (forced: a scripted line is deliberate)
    }
    Ok(())
}
/// c.radio: a line on the net under a name, said by that man if he stands in the world, else by the voice alone
pub(in crate::missions) fn director_beat_radio(g: &mut crate::game::Game, callsign: &str, line: &str) {
    let gd = g.guards.iter().copied().find(|x| g.gd[*x].callsign == callsign && !crate::condition::down(g, g.gd[*x].ch));
    if let Some(x) = gd {
        let dur = crate::guards::bubble_secs(line);
        g.gd[x].bubble = Some(crate::game::GuardBubble { text: String::from(line), t: g.time, dur, radio: true });
        crate::handwork::radio_hand(g, x, dur);
        crate::chronicle::chron_radio(g, x, line);
    }
    crate::cues::cues_radio(g, &format!("{} — {}", callsign, line));
    let at = gd.map(|x| { let c = g.gd[x].ch; g.ch[c].bone(crate::character::Bone::Head).unwrap_or(g.ch[c].pos) });
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Radio, at, Some(if gd.is_some() { 0.7 } else { 0.55 }), None);
}
// ---- world: doors (ts 985)
/// c.doors.lock
pub(in crate::missions) fn director_beat_door_lock(g: &mut crate::game::Game, d: &crate::director::DoorRef, on: bool) -> Result<(), String> {
    let door = director_door_of(g, d)?;
    g.dr[door].leaf.locked = on;
    if on { g.dr[door].leaf.lock_broken = false; }
    Ok(())
}
/// c.doors.open
pub(in crate::missions) fn director_beat_door_open(g: &mut crate::game::Game, d: &crate::director::DoorRef, angle: Option<f64>, hold: Option<bool>) -> Result<(), String> {
    let door = director_door_of(g, d)?;
    g.dr[door].leaf.latched = false;
    g.dr[door].leaf.closing = false;
    match angle {
        Some(a) => { g.dr[door].leaf.angle = a; g.dr[door].leaf.vel = 0.0; crate::doors::door_place(g, door); }
        None => {
            let sign = js::or(m::sign(g.dr[door].def.angle.unwrap_or(1.0)), 1.0);
            g.dr[door].leaf.vel = sign * 3.2;
        }
    }
    if hold == Some(true) { g.dr[door].leaf.quiet_t = -3600.0; } // (an hour of quiet is a leaf on its stay; Door.reset puts it back)
    Ok(())
}
// ---- world: stimulus (ts 999)
/// c.stimulus.charge: a CAUSE, never a result: the item's own fuse detonates it
pub(in crate::missions) fn director_beat_charge(g: &mut crate::game::Game, r: DirectorRunId, at: &crate::director::Place, by: &crate::director::UnitRef, kind: Option<crate::throwables::ItemKind>, fuse: Option<f64>, toss: Option<bool>) -> Result<(), String> {
    let gd = director_guard_of(g, r, by)?;
    let c = g.gd[gd].ch;
    let hand = match crate::character::character_joint_under(g, c, "handL") { Some(h) => h, None => v3::add(g.ch[c].pos, [0.0, 1.3, 0.0]) };
    let from: V3 = [hand[0], m::max(1.0, hand[1]), hand[2]];
    let to = director_place_of(g, at)?;
    let sol = crate::throwables::throwables_solve(g, from, [to[0], 0.0, to[2]], Some(9.0));
    let kind = kind.unwrap_or(crate::throwables::ItemKind::Flash);
    let who = g.ch[c].id;
    let it = crate::throwables::throwables_spawn(g, kind, from, sol.v0, Some(crate::throwables::ThrowablesSpawnOpts { fuse: Some(fuse.unwrap_or(1.2)), who: Some(who), heading: Some(v3::yaw_of(sol.v0)), ..Default::default() }));
    g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::Can);
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Throw, Some(from), Some(0.7), None);
    if toss == Some(true) { crate::character::anim_toss_item(g, c, Some(crate::character::TOSS_QUICK)); }
    Ok(())
}
/// c.stimulus.burst: the cond it hands back asks of the burst's own record, by its serial
pub(in crate::missions) fn director_beat_burst(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &crate::director::UnitRef, at: crate::director::RunBurstsItemAt, rounds: f64, cadence: Option<f64>, spread: Option<f64>) -> Result<crate::director::Cond, String> {
    director_guard_of(g, r, u)?;
    touch(g, b, u);
    let serial = g.serial();
    let next = g.tab.director_run[r].t;
    g.tab.director_run[r].bursts.push(RunBurstsItem {
        u: u.clone(),
        at,
        left: m::max(0.0, m::floor(rounds)),
        cadence: m::max(0.05, cadence.unwrap_or(0.16)),
        spread: spread.unwrap_or(0.01),
        next,
        serial,
    });
    Ok(Cond::Arm2(CondArm2 { test: CondTest::Burst { serial } }))
}

// ---------------------------------------------------------------- the part's functions

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
/// What a script sees of a hearing event.
pub(super) fn view_of(e: crate::game::GameEvent) -> crate::director::EventView {
    EventView {
        kind: view_kind(e.kind),
        pos: v3::copy(e.pos),
        level: e.level.unwrap_or(if e.loud { 1.0 } else { 0.3 }),
        name: None,
    }
}
/// What a script sees of a signal: where the player stands.
pub(super) fn signal_view(g: &crate::game::Game, seat: &crate::seats::SeatRef, s: &crate::director::SignalViewS) -> crate::director::EventView {
    let c = g.pl[seat.player].ch;
    EventView {
        kind: EventViewKind::Signal,
        pos: v3::copy(g.ch[c].pos),
        level: 0.0,
        name: Some(s.name.clone()),
    }
}
