//! src/game/attention.ts lines 1 to 362
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};


/// A new man's row: the literal of attention.ts mem (`Mem::zeroed()` has the wrong clocks).
fn fresh() -> Mem {
    Mem { hold: None, cand: None, lent: None, takes: Vec::new(), tick: -1e9, told_at: -1e9, line: -1, heard_t: -1e9, lent_off: -1e9, owed: -1e9, series: false }
}

/// His row of the table, made if new; the row itself, to write through. (attention.ts mem)
pub fn mem<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::attention::Mem {
    g.m.attention.mem_of.get_or_insert_with(gd, fresh)
}

/// The last few endings, for the check. (attention.ts log)
pub fn log(g: &mut crate::game::Game, gd: GuardId, what: &str, h: crate::attention::LogH) {
    let item = AttentionModATTENTIONLOGItem { t: g.time, who: g.gd[gd].callsign.clone(), what: String::from(what), kind: h.kind, secs: h.secs, nth: h.nth };
    let list = &mut g.m.attention.attention_log;
    list.push(item);
    if list.len() > 48 { list.remove(0); }
}

fn log_h(h: &Hold) -> LogH { LogH { kind: h.kind, secs: h.secs, nth: h.nth } }

/// Every line of this file's pools, for the words check. (attention.ts attentionLines)
pub fn attention_lines() -> Vec<String> {
    let mut all = wary();
    all.extend(wary_foe());
    all.extend(spent());
    all.extend(spent_foe());
    all.extend(broke());
    all.extend(done_foe());
    all
}

/// `g.say(gd, text)` with the rest of its arguments left out.
fn say(g: &mut Game, gd: GuardId, text: &str) { crate::game::game_say(g, gd, text, None, None, None); }

/// A line from a pool, never the one he used last. One die. (attention.ts line)
pub fn line(g: &mut crate::game::Game, gd: GuardId, pool: &[String]) {
    let last = mem(g, gd).line;
    let n = pool.len() as f64;
    let mut i = js::rem(m::floor(crate::math_dice::sim_random(g) * n), n) as i32;
    if pool.len() > 1 && i == last { i = (i + 1) % pool.len() as i32; }
    mem(g, gd).line = i;
    say(g, gd, &pool[i as usize]);
}

/// Write a changed hold back to his row (`H` is the row's own object in the TypeScript).
fn set_hold(g: &mut Game, gd: GuardId, h: Option<Hold>) { mem(g, gd).hold = h; }

/// The hold on this man's attention right now, or none: a copy of the row's, so a change goes back by `set_hold`. (attention.ts attentionOf)
pub fn attention_of(g: &crate::game::Game, gd: GuardId) -> Option<crate::attention::Hold> {
    let now = g.time;
    let row = g.m.attention.mem_of.get(gd)?;
    if now - row.tick <= STALE { row.hold } else { None }
}

/// The cosine of the angle at his eye between the thing that holds him and a man at `p`. (attention.ts offCos)
pub fn off_cos(g: &crate::game::Game, gd: GuardId, h: crate::attention::Hold, p: V3) -> f64 {
    let c = g.gd[gd].ch;
    let e = g.ch[c].bones.get(crate::character::Bone::Head).copied().unwrap_or(g.ch[c].pos);
    let mut ax = h.at[0] - e[0];
    let mut az = h.at[2] - e[2];
    let al = m::hyp2(ax, az);
    if al < 1.0 {
        let y = h.yaw.unwrap_or(g.ch[c].body_yaw);
        ax = m::sin(y);
        az = m::cos(y);
    } else {
        ax /= al;
        az /= al;
    }
    let px = p[0] - e[0];
    let pz = p[2] - e[2];
    let pl = m::hyp2(px, pz);
    if pl < 1e-3 { return 1.0; }
    (ax * px + az * pz) / pl
}

/// Is the floor point `p` more than `focusDeg` off the line from him to the thing. (attention.ts outside)
pub fn outside(g: &crate::game::Game, gd: GuardId, h: crate::attention::Hold, p: V3) -> bool {
    off_cos(g, gd, h, p) < m::cos(g.k.attention.focus_deg * m::PI / 180.0)
}

/// × the rate at which this man's eye takes in a man standing at `p`. (attention.ts attentionSideK)
pub fn attention_side_k(g: &crate::game::Game, gd: GuardId, p: V3) -> f64 {
    match attention_of(g, gd) {
        Some(h) if outside(g, gd, h, p) => g.k.attention.side_k,
        _ => 1.0,
    }
}

/// A glimpse worth `gain` of a man at `p`: true = it was at the edge of a held eye and writes no fix. (attention.ts attentionAside)
pub fn attention_aside(g: &mut crate::game::Game, gd: GuardId, p: V3, gain: f64) -> bool {
    let Some(mut h) = attention_of(g, gd) else { return false };
    if !outside(g, gd, h, p) { return false; }
    if g.gd[gd].state == crate::game::GuardState::Patrol {
        if g.gd[gd].awareness < g.k.attention.break_gain { return true; }
        if h.mode == HoldMode::Lent {
            let off = g.time + g.k.attention.lent_back_secs;
            mem(g, gd).lent_off = off;
        }
        end(g, gd, "glimpse");
        return false;
    }
    h.glimpse += gain;
    set_hold(g, gd, Some(h));
    if h.glimpse < g.k.attention.break_gain { return true; }
    end(g, gd, "glimpse");
    line(g, gd, &broke());
    false
}

/// × a footfall's level at his ear. (attention.ts attentionEarK)
pub fn attention_ear_k(g: &crate::game::Game, gd: GuardId) -> f64 {
    attention_of(g, gd).map_or(1.0, |h| h.ears)
}

/// × any other small noise's level at his ear. (attention.ts attentionNoiseK)
pub fn attention_noise_k(g: &crate::game::Game, gd: GuardId) -> f64 {
    attention_of(g, gd).map_or(1.0, |h| h.noises)
}

/// A footfall did reach him, placed at `at`: the hold is over and he turns to it. (attention.ts attentionStep)
pub fn attention_step(g: &mut crate::game::Game, gd: GuardId, at: V3) {
    let Some(h) = attention_of(g, gd) else { return };
    if h.mode == HoldMode::Lent {
        let off = g.time + g.k.attention.lent_back_secs;
        mem(g, gd).lent_off = off;
    }
    end(g, gd, "step");
    if g.gd[gd].state == crate::game::GuardState::Suspicious {
        if !crate::belief::word_now(g, gd) {
            crate::mind::fix(g, gd, Some(v3::copy(at)), "attention");
        }
        let man = &mut g.gd[gd];
        man.path = Vec::new();
        man.path_serial = man.path_serial.wrapping_add(1);
        man.path_goal = None; man.path_goal_orbit = false;   // (guard_set_path_goal's rule, for the row in hand)
    }
}

/// Did he say he was done inside the last few seconds. (attention.ts attentionTold)
pub fn attention_told(g: &crate::game::Game, gd: GuardId) -> bool {
    let now = g.time;
    g.m.attention.mem_of.get(gd).map_or(false, |row| now - row.told_at < 4.0)
}

/// A calm man tipped over the 'huh?' line: true = this file's line stands for the 'huh?'. (attention.ts attentionAgain)
pub fn attention_again(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let now = g.time;
    let memory = g.k.attention.memory_secs;
    let Some(row) = g.m.attention.mem_of.get_mut(gd) else { return false };
    let Some(cand) = row.cand else { return false };
    if cand.t != now || cand.kind == DrawKind::Whistle || !row.takes.iter().any(|t| now - *t <= memory) { return false; }
    row.owed = now;
    true
}

/// A small thing reached this man and he placed it at `where_`; attentionTick weighs it. (attention.ts attentionHeard)
pub fn attention_heard(g: &mut crate::game::Game, gd: GuardId, e: crate::game::GameEvent, where_: V3) {
    if e.kind == crate::director::EventKind::Step || e.kind == crate::director::EventKind::Glass || e.loud { return; }
    let kind = match e.kind {
        crate::director::EventKind::Whistle => DrawKind::Whistle,
        crate::director::EventKind::Door => DrawKind::Door,
        crate::director::EventKind::LightOut => DrawKind::Light,
        _ => DrawKind::Noise,
    };
    if e.lure == Some(true) && g.gd[gd].awareness < g.k.attention.draw_aw && able(g, gd) { let to = g.k.attention.draw_aw; crate::mind::set(g, gd, to, "attention"); }
    let t = g.time;
    mem(g, gd).cand = Some(MemCand { at: [where_[0], 0.0, where_[2]], kind, t });
}

/// Another file's standing claim on this man's attention, renewed every update it stands. (attention.ts attentionLend)
pub fn attention_lend(g: &mut crate::game::Game, gd: GuardId, at: V3, o: Option<crate::attention::AttentionLendO>) {
    let o = o.unwrap_or(AttentionLendO { ears: None, noises: None });
    let lent = MemLent { at: [at[0], 0.0, at[2]], t: g.time, ears: o.ears.unwrap_or(g.k.attention.ear_k), noises: o.noises.unwrap_or(1.0) };
    mem(g, gd).lent = Some(lent);
}

/// Can this man's attention be held at all this frame. (attention.ts able)
pub fn able(g: &crate::game::Game, gd: GuardId) -> bool {
    let man = &g.gd[gd];
    let calm = (man.state == crate::game::GuardState::Patrol || man.state == crate::game::GuardState::Suspicious)
        && man.body_duty.is_none()
        && man.task.is_none()
        && man.awareness < g.k.attention.calm_aw
        && g.time >= man.dazzled_until
        && !g.touring
        && !g.quiet_utility;
    if !calm { return false; }
    if crate::roomfight::fight_live(g) && !g.quiet.active { return false; }
    let team = g.gd[gd].team;
    crate::teams::is_hostile(g, team, crate::teams::Team::Player)
}

/// Seconds AT the thing a fresh taking of this kind is worth. (attention.ts secsFor)
pub fn secs_for(g: &crate::game::Game, kind: crate::attention::DrawKind) -> f64 {
    let k = &g.k.attention;
    match kind {
        DrawKind::Whistle => k.whistle_secs,
        DrawKind::Door => k.door_secs,
        DrawKind::Light => k.light_secs,
        _ => k.noise_secs,
    }
}

/// The hold is over, for `why`. (attention.ts end)
pub fn end(g: &mut crate::game::Game, gd: GuardId, why: &str) {
    let row = mem(g, gd);
    let Some(h) = row.hold else { return };
    row.hold = None;
    if why == "done" { g.stats.attention.released += 1; } else if why != "over" { g.stats.attention.broken += 1; }
    log(g, gd, why, log_h(&h));
}

/// His suspicion is on the held thing while his fix is the thing. (attention.ts attentionTick onIt)
fn on_it(g: &Game, gd: GuardId, h: &Hold) -> bool {
    g.gd[gd].last_known.map_or(false, |k| v3::dist_xz(k, h.at) <= 0.6)
}

/// Once a frame for a man on his own senses: break a hold that no longer stands, weigh a small thing that reached him, keep a lent one. (attention.ts attentionTick)
pub fn attention_tick(g: &mut crate::game::Game, gd: GuardId) {
    let now = g.time;
    let row = mem(g, gd);
    let cand = row.cand.filter(|c| c.t == now);
    row.cand = None;
    let stale = if now - row.tick > STALE { row.hold } else { None };
    if let Some(s) = stale {
        log(g, gd, "stale", log_h(&s));
        set_hold(g, gd, None);
    }
    let row = mem(g, gd);
    let prev = row.tick;
    row.tick = now;
    let owed = row.owed == now;
    let lent = row.lent.filter(|l| l.t >= prev && now - l.t <= STALE);
    if lent.is_none() { row.lent = None; }
    let free = able(g, gd);
    let mut h = mem(g, gd).hold;
    if h.is_some() && !free {
        end(g, gd, "roused");
        h = None;
    }
    if !free {
        if owed { say(g, gd, "huh?"); }
        return;
    }
    if let Some(mut hh) = h {
        let suspicious = g.gd[gd].state == crate::game::GuardState::Suspicious;
        match hh.mode {
            HoldMode::Go => {
                if !suspicious || (!on_it(g, gd, &hh) && cand.is_none()) {
                    end(g, gd, "elsewhere");
                    h = None;
                }
            }
            HoldMode::Look => {
                if suspicious && !g.gd[gd].hold && on_it(g, gd, &hh) {
                    hh.mode = HoldMode::Go;
                    hh.until = -1.0;
                    hh.told = false;
                    set_hold(g, gd, Some(hh));
                    h = Some(hh);
                } else if suspicious && !on_it(g, gd, &hh) && cand.is_none() {
                    end(g, gd, "elsewhere");
                    h = None;
                } else if now >= hh.until {
                    if suspicious {
                        crate::mind::cap(g, gd, 0.1 + hh.glimpse, "attention");
                        end(g, gd, "done");
                    } else {
                        end(g, gd, "over");
                    }
                    h = None;
                } else if suspicious {
                    { let to = g.k.attention.floor; crate::mind::raise(g, gd, to, "attention"); }
                    tell(g, gd, &mut hh);
                    h = Some(hh);
                }
            }
            HoldMode::Lent => match lent {
                Some(l) if g.gd[gd].state == crate::game::GuardState::Patrol => {
                    hh.at = l.at;
                    hh.ears = l.ears;
                    hh.noises = l.noises;
                    set_hold(g, gd, Some(hh));
                    h = Some(hh);
                }
                _ => {
                    end(g, gd, "over");
                    h = None;
                }
            },
        }
    }
    let mut spoke = false;
    if let Some(cand) = cand {
        let same_secs = g.k.attention.same_secs;
        let row = mem(g, gd);
        let same = now - row.heard_t < same_secs;
        row.heard_t = now;
        if !same { row.series = false; }
        let series = row.series;
        let state = g.gd[gd].state;
        let going = state == crate::game::GuardState::Suspicious && g.gd[gd].last_known.map_or(false, |k| v3::dist_xz(k, cand.at) < 0.6);
        let whistle = crate::whistle::whistle_mem(g, gd);
        let glance = whistle.as_ref().map(|w| (w.glance_until, w.glance_at));
        let glancing = state == crate::game::GuardState::Patrol && glance.map_or(false, |(until, at)| now < until && at.map_or(false, |a| v3::dist_xz(a, cand.at) < 0.6));
        let not_lent = h.filter(|x| x.mode != HoldMode::Lent);
        if let (Some(mut hh), true) = (not_lent, same) {
            if going {
                if hh.mode == HoldMode::Go && hh.until >= 0.0 && v3::dist_xz(hh.at, cand.at) > 0.6 {
                    hh.until = -1.0;
                    hh.yaw = None;
                    hh.told = false;
                }
                hh.at = v3::copy(cand.at);
                set_hold(g, gd, Some(hh));
                h = Some(hh);
            }
        } else if going && !(same && series) {
            if h.is_some() { end(g, gd, "elsewhere"); }
            let memory = g.k.attention.memory_secs;
            let row = mem(g, gd);
            row.takes.retain(|t| now - *t <= memory);
            let nth = (row.takes.len() + 1) as i32;
            row.takes.push(now);
            row.series = true;
            let foe_lines = g.gd[gd].team != crate::teams::Team::Guard;
            if nth as f64 >= g.k.attention.spent && cand.kind != DrawKind::Whistle {
                crate::mind::set_state(g, gd, crate::game::GuardState::Search, "attention");
                g.gd[gd].search_t = 0.0;
                g.gd[gd].search_plan = None;
                g.gd[gd].pieing = None;
                g.gd[gd].react_t = 0.0;
                g.gd[gd].path = Vec::new();
                g.gd[gd].path_serial = g.gd[gd].path_serial.wrapping_add(1);
                g.gd[gd].path_goal = None; g.gd[gd].path_goal_orbit = false;   // (guard_set_path_goal's rule, for the row in hand)
                g.gd[gd].stall_ref = None;
                g.stats.attention.spent += 1;
                log(g, gd, "spent", LogH { kind: cand.kind, secs: 0.0, nth });
                line(g, gd, &if foe_lines { spent_foe() } else { spent() });
                return;
            }
            let full = secs_for(g, cand.kind);
            let secs = full * m::pow(g.k.attention.wear_k, (nth - 1) as f64);
            let posted = g.gd[gd].hold;
            let until = if posted { now + m::max(0.0, g.gd[gd].react_t) + secs } else { -1.0 };
            let made = Hold { at: v3::copy(cand.at), kind: cand.kind, mode: if posted { HoldMode::Look } else { HoldMode::Go }, t0: now, secs, full, until, glimpse: 0.0, told: false, nth, ears: g.k.attention.ear_k, noises: 1.0, yaw: None, serial: g.serial() };
            set_hold(g, gd, Some(made));
            h = Some(made);
            g.stats.attention.taken += 1;
            log(g, gd, "taken", log_h(&made));
            if nth > 1 {
                g.stats.attention.worn += 1;
                if cand.kind != DrawKind::Whistle {
                    line(g, gd, &if foe_lines { wary_foe() } else { wary() });
                    spoke = true;
                }
            }
        } else if glancing && h.map_or(true, |x| x.mode == HoldMode::Lent) {
            let until = glance.unwrap().0;
            let secs = until - now;
            let made = Hold { at: v3::copy(cand.at), kind: cand.kind, mode: HoldMode::Look, t0: now, secs, full: secs, until, glimpse: 0.0, told: true, nth: 1, ears: g.k.attention.ear_k, noises: 1.0, yaw: None, serial: g.serial() };
            set_hold(g, gd, Some(made));
            h = Some(made);
            g.stats.attention.looks += 1;
            log(g, gd, "look", log_h(&made));
        }
    }
    if owed && !spoke { say(g, gd, "huh?"); }
    if h.is_none() {
        if let Some(l) = lent {
            let lent_off = mem(g, gd).lent_off;
            if g.gd[gd].state == crate::game::GuardState::Patrol && now >= lent_off && g.gd[gd].awareness < g.k.attention.break_gain {
                let made = Hold { at: l.at, kind: DrawKind::Lent, mode: HoldMode::Lent, t0: now, secs: 0.0, full: 0.0, until: -1.0, glimpse: 0.0, told: true, nth: 1, ears: l.ears, noises: l.noises, yaw: None, serial: g.serial() };
                set_hold(g, gd, Some(made));
                g.stats.attention.lent += 1;
                log(g, gd, "lent", log_h(&made));
            }
        }
    }
}

/// `tellSecs` before the end he says he is done. `h` is the caller's copy of his row's hold: `told` goes to both. (attention.ts tell)
pub fn tell(g: &mut crate::game::Game, gd: GuardId, h: &mut crate::attention::Hold) {
    if h.told || h.until < 0.0 || g.time < h.until - g.k.attention.tell_secs { return; }
    h.told = true;
    if let Some(on_row) = mem(g, gd).hold.as_mut() { on_row.told = true; }
    let now = g.time;
    let pos = g.ch[g.gd[gd].ch].pos;
    if crate::guards::near_known_body(g, pos, Some(4.0)).is_some()
        || g.gd[gd].bubble.as_ref().map_or(false, |b| b.radio && now - b.t < 4.0)
        || (matches!(g.gd[gd].foe, Some(crate::game::Foe::Player)) && g.pl[g.player].down)
    {
        return;
    }
    mem(g, gd).told_at = now;
    if g.gd[gd].team == crate::teams::Team::Guard {
        let text = if g.escalation >= 1.0 { "clear here — staying on it" } else { "must have been nothing" };
        say(g, gd, text);
    } else {
        line(g, gd, &done_foe());
    }
}

/// Still walking to his fix: a held attention does not drain under him short of the thing. (attention.ts attentionWalk)
pub fn attention_walk(g: &mut crate::game::Game, gd: GuardId) {
    let Some(h) = attention_of(g, gd) else { return };
    if h.mode != HoldMode::Go || h.until >= 0.0 { return; }
    if g.time - h.t0 > g.k.attention.walk_cap {
        end(g, gd, "far");
        return;
    }
    { let to = g.k.attention.floor; crate::mind::raise(g, gd, to, "attention"); }
}

/// Arrived at his fix: true = his attention is on the thing and his suspicion stands. (attention.ts attentionDwell)
pub fn attention_dwell(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let Some(mut h) = attention_of(g, gd) else { return false };
    if h.mode != HoldMode::Go { return false; }
    if h.until < 0.0 {
        h.until = g.time + h.secs;
        h.yaw = Some(g.ch[g.gd[gd].ch].body_yaw);
        set_hold(g, gd, Some(h));
    }
    if g.time >= h.until {
        crate::mind::cap(g, gd, 0.1 + h.glimpse, "attention");
        end(g, gd, "done");
        return false;
    }
    { let to = g.k.attention.floor; crate::mind::raise(g, gd, to, "attention"); }
    tell(g, gd, &mut h);
    true
}



/// The debug panel's line. (attention.ts attentionSummary)
pub fn attention_summary(g: &crate::game::Game) -> String {
    let mut parts: Vec<String> = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        let Some(h) = attention_of(g, gd) else { continue };
        let left = if h.until >= 0.0 { alloc::format!(", {} s left", js::to_fixed(m::max(0.0, h.until - g.time), 1)) } else { String::new() };
        let nth = if h.nth > 1 { alloc::format!(", taking {}", h.nth) } else { String::new() };
        parts.push(alloc::format!("{} on a {} ({}{}{})", g.gd[gd].callsign, h.kind.as_str(), h.mode.as_str(), left, nth));
    }
    let st = &g.stats.attention;
    parts.push(alloc::format!("{} taken, {} looks, {} let go, {} broken, {} spent", st.taken, st.looks, st.released, st.broken, st.spent));
    alloc::format!("attention ▸ {}", parts.join(" · "))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A blank game with one man on it; leaked, so a test holds it as a plain reference.
    fn game() -> (&'static mut Game, GuardId) {
        let g: &'static mut Game = Box::leak(Box::new(Game::blank()));
        let c = g.ch.push(crate::character::Character::blank());
        let mut man = crate::game::Guard::blank();
        man.ch = c;
        let gd = g.gd.push(man);
        g.time = 100.0;
        (g, gd)
    }

    fn hold(g: &mut Game, gd: GuardId, mode: HoldMode, until: f64) {
        let now = g.time;
        let row = g.m.attention.mem_of.get_or_insert_with(gd, fresh);
        row.tick = now;
        row.hold = Some(Hold { at: [0.0, 0.0, 10.0], kind: DrawKind::Noise, mode, t0: now, secs: 5.0, full: 5.0, until, glimpse: 0.0, told: true, nth: 1, ears: 0.5, noises: 1.0, yaw: None, serial: 0 });
    }

    #[test]
    fn a_new_row_has_the_literals_clocks() {
        let (mut g, gd) = game();
        let row = mem(&mut g, gd);
        assert_eq!((row.tick, row.told_at, row.line, row.heard_t, row.lent_off, row.owed), (-1e9, -1e9, -1, -1e9, -1e9, -1e9));
    }

    #[test]
    fn a_hold_unticked_past_stale_is_nobodys() {
        let (mut g, gd) = game();
        assert!(attention_of(&mut g, gd).is_none());
        hold(&mut g, gd, HoldMode::Go, -1.0);
        g.time += 0.25;
        assert!(attention_of(&mut g, gd).is_some());
        g.time += 0.01;
        assert!(attention_of(&mut g, gd).is_none());
        assert_eq!(attention_ear_k(&mut g, gd), 1.0);
    }

    #[test]
    fn the_angle_at_his_eye() {
        let (mut g, gd) = game();
        let h = Hold { at: [0.0, 0.0, 10.0], ..Hold::zeroed() };
        assert_eq!(off_cos(&mut g, gd, h, [0.0, 0.0, 3.0]), 1.0);
        assert_eq!(off_cos(&mut g, gd, h, [3.0, 0.0, 0.0]), 0.0);
        assert_eq!(off_cos(&mut g, gd, h, [0.0, 0.0, 1e-4]), 1.0);   // a man on his own eye
        assert!(outside(&mut g, gd, h, [3.0, 0.0, 0.0]));
        assert!(!outside(&mut g, gd, h, [0.0, 0.0, 3.0]));
        // the thing under his feet: the way he faced it, not the line to it
        let under = Hold { at: [0.0, 0.0, 0.5], yaw: Some(0.0), ..Hold::zeroed() };
        assert_eq!(off_cos(&mut g, gd, under, [0.0, 0.0, -2.0]), -1.0);
    }

    #[test]
    fn a_man_at_his_business_breaks_by_his_own_awareness_with_no_line() {
        let (mut g, gd) = game();
        g.gd[gd].state = crate::game::GuardState::Patrol;
        hold(&mut g, gd, HoldMode::Lent, -1.0);
        g.gd[gd].awareness = 0.1;
        assert!(attention_aside(&mut g, gd, [10.0, 0.0, 0.0], 0.5));
        assert!(attention_of(&mut g, gd).is_some());
        g.gd[gd].awareness = g.k.attention.break_gain;
        assert!(!attention_aside(&mut g, gd, [10.0, 0.0, 0.0], 0.5));
        assert!(attention_of(&mut g, gd).is_none());
        assert_eq!(g.m.attention.mem_of.get(gd).unwrap().lent_off, 100.0 + g.k.attention.lent_back_secs);
        assert_eq!(g.stats.attention.broken, 1);
    }

    #[test]
    fn a_glimpse_is_summed_on_the_row() {
        let (mut g, gd) = game();
        g.gd[gd].state = crate::game::GuardState::Suspicious;
        hold(&mut g, gd, HoldMode::Go, -1.0);
        assert!(attention_aside(&mut g, gd, [10.0, 0.0, 0.0], 0.05));
        assert!(attention_aside(&mut g, gd, [10.0, 0.0, 0.0], 0.05));
        assert_eq!(g.m.attention.mem_of.get(gd).unwrap().hold.unwrap().glimpse, 0.1);
        assert!(!attention_aside(&mut g, gd, [0.0, 0.0, 5.0], 0.05));   // inside the focus: not summed
        assert_eq!(g.m.attention.mem_of.get(gd).unwrap().hold.unwrap().glimpse, 0.1);
    }

    #[test]
    fn the_dwell_starts_its_clock_on_arrival_and_lets_go_at_the_end() {
        let (mut g, gd) = game();
        g.gd[gd].state = crate::game::GuardState::Suspicious;
        g.gd[gd].awareness = 0.2;
        g.ch[g.gd[gd].ch].body_yaw = 1.5;
        hold(&mut g, gd, HoldMode::Go, -1.0);
        assert!(attention_dwell(&mut g, gd));
        let h = g.m.attention.mem_of.get(gd).unwrap().hold.unwrap();
        assert_eq!((h.until, h.yaw), (105.0, Some(1.5)));
        assert_eq!(g.gd[gd].awareness, g.k.attention.floor);
        g.time = 105.0;
        g.m.attention.mem_of.get_mut(gd).unwrap().tick = 105.0;
        assert!(!attention_dwell(&mut g, gd));
        assert_eq!(g.gd[gd].awareness, 0.1);
        assert_eq!(g.stats.attention.released, 1);
        assert!(!attention_dwell(&mut g, gd));
    }

    #[test]
    fn the_walk_holds_his_suspicion_up_to_the_cap() {
        let (mut g, gd) = game();
        g.gd[gd].awareness = 0.0;
        hold(&mut g, gd, HoldMode::Go, -1.0);
        attention_walk(&mut g, gd);
        assert_eq!(g.gd[gd].awareness, g.k.attention.floor);
        g.m.attention.mem_of.get_mut(gd).unwrap().hold.as_mut().unwrap().t0 = 100.0 - g.k.attention.walk_cap - 0.1;
        attention_walk(&mut g, gd);
        assert!(attention_of(&mut g, gd).is_none());
        assert_eq!(g.stats.attention.broken, 1);
    }

    #[test]
    fn again_takes_this_frames_thing_inside_the_memory_only() {
        let (mut g, gd) = game();
        assert!(!attention_again(&mut g, gd));
        let row = g.m.attention.mem_of.get_or_insert_with(gd, fresh);
        row.cand = Some(MemCand { at: [0.0; 3], kind: DrawKind::Noise, t: 100.0 });
        row.takes.push(100.0 - 61.0);
        assert!(!attention_again(&mut g, gd));
        g.m.attention.mem_of.get_mut(gd).unwrap().takes.push(100.0 - 60.0);
        assert!(attention_again(&mut g, gd));
        assert_eq!(g.m.attention.mem_of.get(gd).unwrap().owed, 100.0);
        g.m.attention.mem_of.get_mut(gd).unwrap().cand.as_mut().unwrap().kind = DrawKind::Whistle;
        assert!(!attention_again(&mut g, gd));
    }

    #[test]
    fn a_small_thing_is_flat_on_the_floor_and_a_step_or_a_loud_one_is_none() {
        let (mut g, gd) = game();
        let mut e = crate::game::GameEvent::zeroed();
        e.kind = crate::director::EventKind::LightOut;
        attention_heard(&mut g, gd, e.clone(), [1.0, 2.0, 3.0]);
        let c = g.m.attention.mem_of.get(gd).unwrap().cand.unwrap();
        assert_eq!((c.at, c.kind, c.t), ([1.0, 0.0, 3.0], DrawKind::Light, 100.0));
        g.m.attention.mem_of.get_mut(gd).unwrap().cand = None;
        e.kind = crate::director::EventKind::Step;
        attention_heard(&mut g, gd, e.clone(), [1.0, 2.0, 3.0]);
        e.kind = crate::director::EventKind::Kick;
        e.loud = true;
        attention_heard(&mut g, gd, e, [1.0, 2.0, 3.0]);
        assert!(g.m.attention.mem_of.get(gd).unwrap().cand.is_none());
    }

    #[test]
    fn a_tick_when_he_cannot_be_held_rouses_him_and_clears_what_was_heard() {
        let (mut g, gd) = game();
        g.touring = true;   // able() answers before it reaches the fight
        g.gd[gd].state = crate::game::GuardState::Patrol;
        hold(&mut g, gd, HoldMode::Go, -1.0);
        let row = g.m.attention.mem_of.get_mut(gd).unwrap();
        row.cand = Some(MemCand { at: [0.0; 3], kind: DrawKind::Noise, t: 100.0 });
        row.lent = Some(MemLent { at: [0.0; 3], t: 90.0, ears: 1.0, noises: 1.0 });
        attention_tick(&mut g, gd);
        let row = g.m.attention.mem_of.get(gd).unwrap();
        assert!(row.hold.is_none() && row.cand.is_none() && row.lent.is_none());
        assert_eq!(row.tick, 100.0);
        assert_eq!(g.stats.attention.broken, 1);
        assert_eq!(g.m.attention.attention_log[0].what, "roused");
    }

    #[test]
    fn a_write_through_mem_lands_in_the_row() {
        let (mut g, gd) = game();
        mem(&mut g, gd).lent_off = 7.0;
        assert_eq!(g.m.attention.mem_of.get(gd).unwrap().lent_off, 7.0);
    }

    #[test]
    fn tell_before_its_time_writes_nothing() {
        let (mut g, gd) = game();
        hold(&mut g, gd, HoldMode::Look, 105.0);
        let mut h = g.m.attention.mem_of.get(gd).unwrap().hold.unwrap();
        h.told = false;
        tell(&mut g, gd, &mut h);
        assert!(!h.told);
        assert!(g.m.attention.mem_of.get(gd).unwrap().hold.unwrap().told);   // the row's stays as it was
    }

    #[test]
    fn the_panels_line_and_the_list_of_lines() {
        let (mut g, gd) = game();
        g.gd[gd].callsign = String::from("Novak");
        g.guards.push(gd);
        g.m.attention.mem_of.get_or_insert_with(gd, fresh).tick = 100.0;
        g.m.attention.mem_of.get_mut(gd).unwrap().hold = Some(Hold { kind: DrawKind::Noise, mode: HoldMode::Go, until: 103.2, nth: 2, ..Hold::zeroed() });
        g.stats.attention.taken = 4;
        assert_eq!(attention_summary(&mut g), "attention ▸ Novak on a noise (go, 3.2 s left, taking 2) · 4 taken, 0 looks, 0 let go, 0 broken, 0 spent");
        assert_eq!(attention_lines().len(), 12);
    }

    #[test]
    fn the_log_keeps_the_last_48() {
        let (mut g, gd) = game();
        for i in 0..50 { log(&mut g, gd, "x", LogH { kind: DrawKind::Noise, secs: i as f64, nth: 1 }); }
        assert_eq!(g.m.attention.attention_log.len(), 48);
        assert_eq!(g.m.attention.attention_log[0].secs, 2.0);
    }
}
