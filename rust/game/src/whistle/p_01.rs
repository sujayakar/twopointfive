//! src/game/whistle.ts lines 1 to 280
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// What a new man's memory holds (the literal in whistle.ts mem).
fn fresh_mem() -> WhistleMem {
    WhistleMem { responded_at: NEVER, responses: 0.0, searches: 0.0, cues: 0.0, glances: 0.0, glance_until: -1.0, glance_at: None, soft: false, soft_at: NEVER, last_look: -1.0, last_twice: -1.0 }
}

/// A man's whistle memory, made on first asking.   (whistle.ts mem)
pub fn mem<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::whistle::WhistleMem {
    g.m.whistle.mem_of.get_or_insert_with(gd, fresh_mem)
}

/// Crane a calm man's head toward a point for `secs`.   (whistle.ts glanceToward)
pub fn glance_toward(g: &mut crate::game::Game, gd: GuardId, at: V3, secs: Option<f64>) {
    let secs = secs.unwrap_or(g.k.whistle.glance_secs);
    let until = g.time + secs;
    let row = mem(g, gd);
    row.glance_until = until;
    row.glance_at = Some(v3::copy(at));
}

/// Read-only peek for the panel / the QA harness (None: no whistle has ever concerned this man).   (whistle.ts whistleMem)
pub fn whistle_mem(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::whistle::WhistleMem> {
    g.m.whistle.mem_of.get(gd).copied()
}

/// What a new player's breath holds (the literal in whistle.ts blower).
fn fresh_blower() -> Blower {
    Blower { ready_at: -1.0, blown: 0.0, last_at: -100.0 }
}

/// The seat's player's breath, made on first asking.   (whistle.ts blower)
pub fn blower<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::whistle::Blower {
    let key = g.player;
    g.m.whistle.blower_of.get_or_insert_with(key, fresh_blower)
}

/// Seconds until V will sound again (0 = ready).   (whistle.ts whistleReadyIn)
pub fn whistle_ready_in(g: &mut crate::game::Game) -> f64 {
    m::max(0.0, blower(g).ready_at - g.time)
}

/// V pressed this frame.   (whistle.ts whistleKey)
pub fn whistle_key(g: &mut crate::game::Game, input: InputId) {
    if crate::input::input_hit(g, input, "KeyV") { try_whistle(g); }
}

/// Blow it if the breath is back: the event at his feet, the meter's tick, the voice at his head. False = nothing happened.   (whistle.ts tryWhistle)
pub fn try_whistle(g: &mut crate::game::Game) -> bool {
    let pl = g.player;
    if g.pl[pl].down { return false; }
    let now = g.time;
    let (cooldown, level, gain) = (g.k.whistle.cooldown, g.k.whistle.level, g.k.whistle.gain);
    let b = blower(g);
    if now < b.ready_at { return false; }
    b.ready_at = now + cooldown;
    b.blown += 1.0;
    b.last_at = now;
    g.stats.whistle.blown += 1;
    let c = g.pl[pl].ch;
    let (pos, who) = (g.ch[c].pos, g.ch[c].id);
    let serial = g.serial();
    g.events.push(crate::game::GameEvent {
        kind: crate::director::EventKind::Whistle,
        pos: [pos[0], 0.0, pos[2]],
        time: now,
        loud: false,
        level: Some(level),
        who: Some(who),
        serial,
        ..crate::game::GameEvent::zeroed()
    });
    g.pl[pl].noise = m::max(g.pl[pl].noise, level);
    let at = match g.ch[c].bones.get(crate::character::Bone::Head) {
        Some(head) => *head,
        None => v3::add(pos, [0.0, if g.pl[pl].crouch { 1.05 } else { 1.6 }, 0.0]),
    };
    crate::audio_iface::game_audio_like_play(g, "whistle", Some(at), Some(gain), None);
    true
}

/// Mouth height off the event's floor position (a crouched man whistles from lower).   (whistle.ts mouthOf)
pub fn mouth_of(g: &mut crate::game::Game, e: crate::game::GameEvent) -> V3 {
    let man = crate::seats::man_by_id(g, e.who.unwrap_or(crate::consts::PLAYER_ID));
    let crouched = man.map_or(false, |p| g.pl[p].crouch);
    [e.pos[0], if crouched { 1.0 } else { 1.55 }, e.pos[2]]
}

/// Can this man be the one who ANSWERS: calm, free, on nobody's errand, of a side that minds the player.   (whistle.ts canAnswer)
pub fn can_answer(g: &mut crate::game::Game, gd: GuardId, who: f64) -> bool {
    let state = g.gd[gd].state;
    if state != crate::game::GuardState::Patrol && state != crate::game::GuardState::Suspicious { return false; }
    if g.gd[gd].held.is_some() || g.gd[gd].script.is_some() || g.gd[gd].task.is_some() { return false; }
    if crate::seats::is_man_id(g, who) {
        let team = g.gd[gd].team;
        crate::teams::is_hostile(g, team, crate::teams::Team::Player)
    } else {
        false
    }
}

/// Who a whistle reached and which calm man is to answer it, worked out ONCE per event (its serial is the TypeScript's object identity).   (whistle.ts resolve)
pub fn resolve<'g>(g: &'g mut crate::game::Game, e: crate::game::GameEvent, keen: f64) -> &'g mut crate::whistle::Resolution {
    let key = e.serial;
    if g.m.whistle.res_of.has(key) { return g.m.whistle.res_of.get_mut(key).unwrap(); }
    let mut r = Resolution { hearers: JsMap::new(), responder: None, answered_by: None, t: g.time };
    let src = mouth_of(g, e);
    let who = e.who.unwrap_or(crate::consts::PLAYER_ID);
    let mut best: Option<GuardId> = None;
    let mut bd = f64::INFINITY;
    let mut at = 0;
    while at < g.guards.len() {
        let gd = g.guards[at];
        at += 1;
        if g.gd[gd].state == crate::game::GuardState::Dead { continue; }
        let team = g.gd[gd].team;
        if crate::seats::is_man_id(g, who) && !crate::teams::is_hostile(g, team, crate::teams::Team::Player) { continue; }
        let pos = g.ch[g.gd[gd].ch].pos;
        if v3::dist(e.pos, pos) > g.k.whistle.horizon { continue; }
        let prop = crate::weather::hear_path(g, src, v3::add(pos, [0.0, 1.5, 0.0]), None);
        let reach = g.k.whistle.reach_k * prop.carry * keen;
        if prop.path_len >= reach { continue; }
        r.hearers.set(gd, Heard { d: prop.path_len, at: [prop.via[0], 0.0, prop.via[2]], direct: prop.direct, carry: prop.carry });
        if can_answer(g, gd, who) && prop.path_len < bd {
            bd = prop.path_len;
            best = Some(gd);
        }
    }
    r.responder = best;
    g.stats.whistle.resolved += 1;
    g.stats.whistle.heard_by += r.hearers.len() as i32;
    if best.is_none() { g.stats.whistle.unanswered += 1; }
    let hearers: Vec<String> = r.hearers.keys().into_iter().map(|x| g.gd[x].callsign.clone()).collect();
    let responder = best.map(|x| g.gd[x].callsign.clone());
    g.m.whistle.whistle_log.push(WhistleModWHISTLELOGItem { t: g.time, pos: v3::copy(e.pos), hearers, responder, answered: None, search: false });
    if g.m.whistle.whistle_log.len() > 32 { g.m.whistle.whistle_log.remove(0); }
    g.m.whistle.res_of.set(key, r);
    g.m.whistle.res_of.get_mut(key).unwrap()
}

/// `gd.lastKnown = v3.copy(at)`, a new object in the TypeScript: every whole write bumps `fix_stamp` (doze.ts:244's identity question).
fn write_fix(g: &mut crate::game::Game, gd: GuardId, at: V3) {
    let man = &mut g.gd[gd];
    man.last_known = Some(v3::copy(at));
    man.fix_stamp = man.fix_stamp.wrapping_add(1);
}

/// hearingCheck, for a whistle inside the horizon: what it does to THIS man. `small` = hearingCheck's note of a small noise.   (whistle.ts hearWhistle)
pub fn hear_whistle(g: &mut crate::game::Game, gd: GuardId, e: crate::game::GameEvent, keen: f64, small: &mut dyn FnMut(&mut crate::game::Game, V3)) {
    use crate::game::GuardState;
    let who = e.who.unwrap_or(crate::consts::PLAYER_ID);
    if crate::seats::is_man_id(g, who) && g.player_invisible { return; }
    let r = resolve(g, e, keen);
    let (heard, responder, res_t) = (r.hearers.get(gd).copied(), r.responder, r.t);
    let Some(h) = heard else { return };
    mem(g, gd);
    small(g, h.at);
    let knob_aw = g.k.whistle.awareness;
    // already hunting: one more fix on where you are, nothing said
    let state = g.gd[gd].state;
    if state == GuardState::Alert || state == GuardState::Search || g.gd[gd].awareness >= 0.85 {
        // (a word on the radio this tick stands over what he hears: belief.ts wordNow)
        if !crate::belief::word_now(g, gd) && (state != GuardState::Alert || g.gd[gd].last_known.is_none() || g.time - g.gd[gd].last_seen_t > 1.0) {
            write_fix(g, gd, h.at);
        }
        g.gd[gd].awareness = m::max(g.gd[gd].awareness, knob_aw);
        let row = mem(g, gd);
        row.cues += 1.0;
        row.soft = false;
        g.stats.whistle.cued += 1;
        return;
    }
    // somebody nearer answers it (or nobody free can): he turns his head that way and carries on
    if responder != Some(gd) {
        let until = g.time + g.k.whistle.glance_secs;
        let row = mem(g, gd);
        row.glance_until = until;
        row.glance_at = Some(v3::copy(h.at));
        row.glances += 1.0;
        g.stats.whistle.glanced += 1;
        return;
    }
    // he answers it
    let now = g.time;
    let twice_window = g.k.whistle.twice_window;
    let row = mem(g, gd);
    let twice = now - row.responded_at <= twice_window;
    row.responded_at = now;
    row.responses += 1.0;
    resolve(g, e, keen).answered_by = Some(gd);
    g.stats.whistle.answered += 1;
    if !crate::belief::word_now(g, gd) { write_fix(g, gd, h.at); }
    {
        let man = &mut g.gd[gd];
        man.path = Vec::new();
        man.path_serial = man.path_serial.wrapping_add(1);
        man.path_goal = None;
        man.path_goal_orbit = false;
        man.stall_ref = None;
        man.stall_t = 0.0;
        man.search_t = 0.0;
        man.awareness = m::max(man.awareness, knob_aw);
    }
    let callsign = g.gd[gd].callsign.clone();
    if let Some(log) = g.m.whistle.whistle_log.last_mut() {
        if log.t == res_t { log.answered = Some(callsign); }
    }
    let is_guard = g.gd[gd].team == crate::teams::Team::Guard;
    if twice {
        // the second inside the window: a proper search of the place, gun out, no more 'hello?'
        {
            let man = &mut g.gd[gd];
            man.state = GuardState::Search;
            man.search_plan = None;
            man.pieing = None;
            man.body_duty = None;
            man.react_t = 0.0;
        }
        let row = mem(g, gd);
        row.searches += 1.0;
        row.soft = false;
        g.stats.whistle.searched += 1;
        if let Some(log) = g.m.whistle.whistle_log.last_mut() {
            if log.t == res_t { log.search = true; }
        }
        let pool = if is_guard { twice_now() } else { twice_foe() };
        let line = pick(g, &pool, gd, PickKey::LastTwice);
        crate::game::game_say(g, gd, &line, None, None, None);
    } else {
        // the first: over to it for a look, torch in hand
        if g.gd[gd].state == GuardState::Patrol { g.gd[gd].state = GuardState::Suspicious; }
        g.gd[gd].react_t = g.k.whistle.react_t;
        let row = mem(g, gd);
        row.soft = true;
        row.soft_at = now;
        let pool = if is_guard { look_lines() } else { look_foe() };
        let line = pick(g, &pool, gd, PickKey::LastLook);
        crate::game::game_say(g, gd, &line, None, None, None);
    }
}

/// The place picked from a pool: the die's `r`, and no saying the last one twice running.   (the arithmetic of whistle.ts pick)
fn pick_i(r: f64, len: f64, last: f64) -> f64 {
    let mut i = js::rem(m::floor(r * len), len);
    if len > 1.0 && i == last { i = js::rem(i + 1.0, len); }
    i
}

/// A line from the pool, and the place it had written in the man's row.   (whistle.ts pick)
pub fn pick(g: &mut crate::game::Game, pool: &[String], gd: GuardId, key: crate::whistle::PickKey) -> String {
    let r = crate::math_dice::sim_random(g);
    let row = mem(g, gd);
    let last = match key { PickKey::LastLook => row.last_look, PickKey::LastTwice => row.last_twice };
    let i = pick_i(r, pool.len() as f64, last);
    match key { PickKey::LastLook => row.last_look = i, PickKey::LastTwice => row.last_twice = i }
    pool[i as usize].clone()
}

/// Is this man's suspicion (still) only a whistle's?   (whistle.ts whistleSoft)
pub fn whistle_soft(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let Some(row) = g.m.whistle.mem_of.get(gd) else { return false };
    row.soft
        && g.gd[gd].state == crate::game::GuardState::Suspicious
        && g.gd[gd].awareness <= g.k.whistle.awareness + 0.02
        && g.gd[gd].witness.alert_t < row.soft_at
}

/// The patrol branch, once a frame: soft off, and a head still turned toward a whistle is craned that way.   (whistle.ts whistlePatrolTick)
pub fn whistle_patrol_tick(g: &mut crate::game::Game, gd: GuardId) {
    let Some(row) = g.m.whistle.mem_of.get_mut(gd) else { return };
    row.soft = false;
    let (until, glance_at) = (row.glance_until, row.glance_at);
    if g.time < until {
        if let Some(at) = glance_at {
            let c = g.gd[gd].ch;
            let pos = g.ch[c].pos;
            let to = m::atan2(at[0] - pos[0], at[2] - pos[2]);
            let max = g.k.whistle.glance_max;
            g.ch[c].anim.look_yaw_extra = v3::clamp(v3::wrap_angle(to - g.ch[c].body_yaw), -max, max);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pick_keeps_the_place_or_moves_on_from_the_last() {
        assert_eq!(pick_i(0.5, 5.0, -1.0), 2.0);
        assert_eq!(pick_i(0.5, 5.0, 2.0), 3.0);
        // the last place of the pool wraps to the first
        assert_eq!(pick_i(0.99, 3.0, 2.0), 0.0);
        // a pool of one has nowhere else to go
        assert_eq!(pick_i(0.5, 1.0, 0.0), 0.0);
    }
}
