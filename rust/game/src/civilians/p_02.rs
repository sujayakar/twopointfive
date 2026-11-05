//! src/game/civilians.ts lines 414 to 875
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

fn civ_of(g: &Game, gd: GuardId) -> CivId { g.gd[gd].civ.unwrap() }
/// `c.bones.<bone> ?? v3.add(c.pos, up)`
fn bone_or(g: &Game, c: CharId, bone: crate::character::Bone, up: V3) -> V3 {
    match g.ch[c].bones.get(bone) { Some(p) => *p, None => v3::add(g.ch[c].pos, up) }
}

// ---------------------------------------------------------------- seam: what skeleton 3 still cannot say
/// civilians.ts:424,514 g.audio.play(name, at, vol, { rate }): `opts` of game_audio_like_play holds `crate::Opaque`, which cannot carry the rate
fn seam_audio_play_rate(g: &mut Game, name: &str, at: Option<V3>, vol: f64, rate: f64) { todo!("needs: game_audio_like_play's opts to carry a rate") }

/// `s.view.viewProj` and `g.viewProj`, as in_view reads them
fn as_in_view(vp: &Option<crate::game::GameViewProj>) -> Option<InViewVp> {
    match vp {
        Some(crate::game::GameViewProj::Array(v)) => Some(InViewVp::Array(v.clone())),
        Some(crate::game::GameViewProj::Float32Array(v)) => Some(InViewVp::Float32Array(v.clone())),
        None => None,
    }
}

// ---------------------------------------------------------------- her voice, the gates, the floor's queue
/// A hostage says a line of this moment, off her own die, never the one over her head now. False: down, held, or no such line.   (civilians.ts hostageSay)
pub fn hostage_say(g: &mut crate::game::Game, gd: GuardId, moment: crate::dialogue::HostageMoment, other: Option<GuardId>) -> bool {
    use crate::dialogue::HostageMoment as M;
    let cv = match g.gd[gd].civ { Some(c) => c, None => return false };
    if g.gd[gd].state == crate::game::GuardState::Dead || g.gd[gd].held.is_some() || g.cv[cv].state == CivState::Gone { return false; }
    let name: Option<String> = other.and_then(|o| g.gd[o].civ).map(|oc| g.cv[oc].name.clone());
    let up: Option<String> = g.gd[gd].bubble.as_ref().map(|b| b.text.clone());
    let lines = crate::dialogue::hostage_lines_table();
    let rows = match moment { M::Shot => lines.shot, M::Wait => lines.wait, M::Threat => lines.threat, M::Aimed => lines.aimed, M::Entry => lines.entry, M::Alone => lines.alone, M::Secured => lines.secured };
    let mut pool: Vec<String> = Vec::new();
    for t in rows.iter() {
        if name.is_none() && t.contains("{other}") { continue; }
        let t = t.replace("{other}", name.as_deref().unwrap_or(""));
        if up.as_deref() != Some(t.as_str()) { pool.push(t); }
    }
    if pool.is_empty() { return false; }
    let line = pick(g, gd, &pool);
    civ_say(g, gd, &line, Some(0.0), Some(matches!(moment, M::Wait | M::Threat | M::Alone)));
    if matches!(moment, M::Aimed | M::Shot | M::Entry) {
        let c = g.gd[gd].ch;
        let head = bone_or(g, c, crate::character::Bone::Head, [0.0, 0.4, 0.0]);
        let rate = if g.cv[cv].def.kind == crate::scene_level::CivilianDefKind::Worker { 1.15 } else { 1.5 };
        seam_audio_play_rate(g, "heldProtest", Some(head), 0.5, rate);
    }
    true
}

/// Her light meter's keys are dropped.   (civilians.ts forgetMeter)
pub fn forget_meter(g: &mut crate::game::Game, gd: GuardId) {
    let id = g.ch[g.gd[gd].ch].id;
    crate::game::game_light_forget(g, &format!("civC{}", id));
    crate::game::game_light_forget(g, &format!("civH{}", id));
}

/// Are the civilians on the floor at all right now.   (civilians.ts civiliansOnFloor)
pub fn civilians_on_floor(g: &mut crate::game::Game) -> bool {
    !g.civilians.is_empty() && !g.quiet_utility && !g.touring && !crate::lighting::is_daylight(g)
}

/// The ones to draw and collide with this frame.   (civilians.ts civiliansShown)
pub fn civilians_shown(g: &mut crate::game::Game) -> Vec<GuardId> {
    if !civilians_on_floor(g) { return Vec::new(); }
    let mut out = Vec::new();
    for &cv in g.civilians.iter() { if g.cv[civ_of(g, cv)].state != CivState::Gone { out.push(cv); } }
    out
}

/// The living ones a round can find and Sam cannot walk through.   (civilians.ts civilianBodies)
pub fn civilian_bodies(g: &mut crate::game::Game) -> Vec<GuardId> {
    let arg = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
    let men = crate::seats::men(g, &arg);
    let shown = civilians_shown(g);
    let mut out = Vec::new();
    for cv in shown {
        if g.gd[cv].state == crate::game::GuardState::Dead || !g.ch[g.gd[cv].ch].alive { continue; }
        if men.iter().any(|&pl| g.pl[pl].holding.as_ref().map(|h| h.g) == Some(cv)) { continue; }
        out.push(cv);
    }
    out
}

/// Their door contacts: a standing civilian pushes a leaf like anybody.   (civilians.ts civilianDoorContacts)
pub fn civilian_door_contacts(g: &mut crate::game::Game, contacts: &mut Vec<crate::doors::DoorContact>) {
    for cv in civilians_shown(g) {
        if g.gd[cv].state == crate::game::GuardState::Dead || g.gd[cv].held.is_some() { continue; }
        let c = g.gd[cv].ch;
        contacts.push(crate::doors::DoorContact { pos: g.ch[c].pos, radius: 0.34, bash: false, who: g.ch[c].id, quiet: g.cv[civ_of(g, cv)].state == CivState::Work, ..Default::default() });
    }
}

/// The floor-wide memory: the row itself, made if it is not there.   (civilians.ts floorOf)
pub fn floor_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::civilians::CiviliansModFloorsValue {
    g.m.civilians.floors.get_or_insert_with(|| CiviliansModFloorsValue { evac_told: false, greet_at: -1e9, pending: Vec::new() })
}

/// A word or a doing due `secs` from now.   (civilians.ts later)
pub fn later(g: &mut crate::game::Game, secs: f64, p: &crate::civilians::LaterP) {
    let at = g.time + secs;
    let row = Pending { at, kind: p.kind, cv: p.cv, o: p.o, key: p.key.clone(), armed: p.armed, pos: p.pos, lvl: p.lvl, dead: p.dead };
    floor_of(g).pending.push(row);
}

/// `o && g.guards.includes(o) ? o : null`
fn on_shift(g: &Game, o: Option<GuardId>) -> Option<GuardId> { o.filter(|x| g.guards.contains(x)) }

/// The doing itself, when its time comes.   (civilians.ts runPending)
pub fn run_pending(g: &mut crate::game::Game, p: &mut crate::civilians::Pending) {
    use crate::chronicle::{ChronCountKey, ChronKind, ChronicleDataValue as D, ChronicleNDataValue as N};
    use crate::game::GuardState;
    use crate::guards::RaiseEscalationCause as Why;
    let cv = p.cv;
    let o = p.o;
    let civ = civ_of(g, cv);
    match p.kind {
        PendKind::GreetReply => {
            if let Some(o) = o {
                let chatty = chatty_enough(g, o);
                if chatty || (g.gd[o].state == GuardState::Patrol && g.gd[o].awareness < 0.15 && g.gd[o].held.is_none() && g.guards.contains(&o)) {
                    let aw = away(g);
                    let bins = g.cv[civ].def.tasks.as_ref().map_or(false, |t| t.contains(&crate::scene_level::CivilianTask::Bin));
                    let lines = crate::dialogue::civ_lines();
                    let pool: Vec<String> = lines.guide.greet_guard.iter()
                        .filter(|t| !((t.contains("stairs") || t.contains("elevator")) && aw) && !(t.contains("trash") && !bins))
                        .cloned().collect();
                    let line = pick(g, cv, &pool);
                    let text = fill(g, cv, &line, Some(o));
                    crate::guards::say(g, o, &text, None, None, None);
                }
            }
        }
        PendKind::ReportRadio => {
            let o = match o { Some(o) if g.gd[o].state != GuardState::Dead && g.gd[o].held.is_none() => o, _ => return };
            let lines = crate::dialogue::civ_lines();
            let key = String::from(p.key.as_deref().unwrap_or("someone"));
            let line = match lines.guard_radio.get(key) { Some(t) => t.clone(), None => lines.guard_radio.get(String::from("someone")).unwrap().clone() };
            let text = fill(g, cv, &line, Some(o));
            crate::guards::say(g, o, &text, Some(true), None, None);
            let oc = g.gd[o].ch;
            let at = match g.ch[oc].bones.get(crate::character::Bone::Head) { Some(h) => *h, None => g.ch[oc].pos };
            crate::audio_iface::game_audio_like_play(g, "radio", Some(at), Some(0.7), None);
            if p.armed.unwrap_or(false) && !g.quiet_utility {
                let pos = p.pos.or(g.cv[civ].cause_pos).unwrap_or(g.ch[g.gd[cv].ch].pos);
                g.alarm.pos[0] = pos[0]; g.alarm.pos[1] = 0.0; g.alarm.pos[2] = pos[2]; g.alarm.placed = true;
                later(g, 1.2, &LaterP { kind: PendKind::Escalate, cv, o: Some(o), ..Default::default() });
            }
        }
        PendKind::Escalate => {
            let except = on_shift(g, o);
            crate::guards::raise_escalation(g, 1.0, except, Some(Why::Civilian), None);
        }
        PendKind::OutsideCall => {
            if !g.civilians.contains(&cv) { return; }
            let pos = g.cv[civ].cause_pos.unwrap_or(g.ch[g.gd[cv].ch].pos);
            let aw = away(g);
            let lines = crate::dialogue::civ_lines();
            let table = if aw { &lines.guide.own.control } else { &lines.guide.control };
            let line = table.get(String::from("outside")).unwrap().clone();
            control(g, cv, &line);
            let sent = crate::devices::send_to_look(g, pos, "civilian");
            if g.cv[civ].armed { later(g, 1.4, &LaterP { kind: PendKind::Escalate, cv, o: sent, ..Default::default() }); }
            let (who, cause, armed) = (g.cv[civ].name.clone(), g.cv[civ].cause, g.cv[civ].armed);
            let cause_word = String::from(cause.map_or("sam", |k| k.as_str()));
            let sent_name = sent.map_or(String::new(), |s| g.gd[s].callsign.clone());
            if first_hand(cause) {
                let mut data: JsMap<String, Option<N>> = JsMap::new();
                data.set(String::from("who"), Some(N::Text(who)));
                data.set(String::from("to"), Some(N::Text(String::from("control"))));
                data.set(String::from("cause"), Some(N::Text(cause_word)));
                data.set(String::from("armed"), Some(N::Bool(armed)));
                data.set(String::from("how"), Some(N::Text(String::from("outside"))));
                data.set(String::from("sent"), Some(N::Text(sent_name)));
                crate::chronicle::chronicle_n(g, ChronKind::CivilianReported, ChronCountKey::CivReported, Some(&data), Some(pos));
            } else {
                let mut data: JsMap<String, Option<D>> = JsMap::new();
                data.set(String::from("who"), Some(D::Text(who)));
                data.set(String::from("to"), Some(D::Text(String::from("control"))));
                data.set(String::from("cause"), Some(D::Text(cause_word)));
                data.set(String::from("armed"), Some(D::Bool(armed)));
                data.set(String::from("how"), Some(D::Text(String::from("outside"))));
                data.set(String::from("sent"), Some(D::Text(sent_name)));
                crate::chronicle::chronicle(g, ChronKind::CivilianReported, Some(&data), Some(pos));
            }
        }
        PendKind::ComplyLine => {
            if g.gd[cv].state != GuardState::Dead && g.gd[cv].held.is_none() {
                let lines = crate::dialogue::civ_lines();
                let line = pick(g, cv, &lines.comply.civ);
                civ_say(g, cv, &line, None, None);
            }
        }
        PendKind::FoundReport => {
            if let Some(o) = o {
                if g.gd[cv].state != GuardState::Dead && g.gd[cv].held.is_none() && staff_calm(g, o) && g.cv[civ].state == CivState::Cower {
                    deliver_report(g, cv, o, DeliverReportHow::Found);
                }
            }
        }
        PendKind::BodyEsc => {
            let to = if p.dead.unwrap_or(false) || p.lvl.unwrap_or(0.0) >= 1.0 || g.alarm.episode { 2.0 } else { 1.0 };
            let except = on_shift(g, o);
            crate::guards::raise_escalation(g, to, except, Some(Why::Body), None);
        }
        PendKind::Outrage => {
            let lines = crate::dialogue::civ_lines();
            control(g, cv, &lines.outrage);
        }
    }
}

/// She says it: a bubble over her head, and the log under her name when she is off screen.   (civilians.ts civSay)
pub fn civ_say(g: &mut crate::game::Game, gd: GuardId, text: &str, hold: Option<f64>, unheard: Option<bool>) {
    let hold = hold.unwrap_or(0.0);
    let unheard = unheard.unwrap_or(false);
    let mut text = String::from(text);
    if crate::snatch::snatch_is_man(g, gd) {
        match crate::snatch::snatch_voice(g, gd, &text) { Some(own) => text = own, None => return }
    }
    let cv = civ_of(g, gd);
    let t = fill(g, gd, &text, None);
    let length = t.encode_utf16().count() as f64;   // (a JavaScript string's length)
    let now = g.time;
    g.gd[gd].bubble = Some(crate::game::GuardBubble { text: t.clone(), t: now, dur: m::min(4.5, m::max(2.0, 1.5 + length * 0.055 + hold)), radio: false });
    g.cv[cv].last_line_at = now;
    if !unheard && (crate::fogofwar::fog_of_war_hidden(g, gd) || !on_screen_head(g, gd)) {
        let line = format!("{}: {}", g.cv[cv].name, t);
        crate::game::game_msg(g, &line);
    }
}

/// On a player's screen: with two, on either's.   (civilians.ts onScreenHead)
pub fn on_screen_head(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let p = bone_or(g, g.gd[gd].ch, crate::character::Bone::Head, [0.0, 1.7, 0.0]);
    if g.seats.len() > 1 {
        let seats = g.seats.clone();
        for s in seats {
            let man = crate::cmd::seat_man(g, s);
            let vp = crate::seats::with_seat(g, man, &mut |g| as_in_view(&g.view_proj));
            if in_view(vp.as_ref(), p) { return true; }
        }
        return false;
    }
    let vp = as_in_view(&g.view_proj);
    in_view(vp.as_ref(), p)
}

/// Is `p` inside this view-projection's frame.   (civilians.ts inView)
pub fn in_view(vp: Option<&crate::civilians::InViewVp>, p: V3) -> bool {
    let vp = match vp { Some(v) => v, None => return false };
    let at = |i: usize| -> f64 { match vp { InViewVp::Array(v) => v[i], InViewVp::Float32Array(v) => v[i] as f64 } };
    let w = at(3) * p[0] + at(7) * p[1] + at(11) * p[2] + at(15);
    if w <= 0.0 { return false; }
    let x = (at(0) * p[0] + at(4) * p[1] + at(8) * p[2] + at(12)) / w;
    let y = (at(1) * p[0] + at(5) * p[1] + at(9) * p[2] + at(13)) / w;
    x > -0.92 && x < 0.92 && y > -0.9 && y < 0.9
}

/// One line of a pool, off her own die.   (civilians.ts pick)
pub fn pick(g: &mut crate::game::Game, gd: GuardId, pool: &[String]) -> String {
    let cv = civ_of(g, gd);
    let n = pool.len() as f64;
    let r = g.cv[cv].dice.roll();
    pool[js::rem(m::floor(r * n), n) as usize].clone()
}

/// This is not the office's floor.   (civilians.ts away)
pub fn away(g: &mut crate::game::Game) -> bool { !crate::dialogue::is_office(g) }

/// What she tells the man she reaches, by cause.   (civilians.ts reportPool)
pub fn report_pool(g: &mut crate::game::Game, cause: Option<&str>) -> Vec<String> {
    let lines = crate::dialogue::civ_lines();
    let k = match cause { Some(c) if lines.report.has(String::from(c)) => c, _ => "sam" };
    let was = lines.report.get(String::from(k));
    match lines.guide.report.get(String::from(k)).or(was) { Some(p) => p.clone(), None => Vec::new() }
}

/// A pool without its shouts for 'security' where there is no such desk.   (civilians.ts noSecurity)
pub fn no_security(g: &mut crate::game::Game, pool: &[String]) -> Vec<String> {
    if !away(g) { return pool.to_vec(); }
    pool.iter().filter(|t| !t.to_ascii_lowercase().contains("security")).cloned().collect()
}

/// {room} {name} {label} {she} {her} {cs} filled off her record.   (civilians.ts fill)
pub fn fill(g: &mut crate::game::Game, gd: GuardId, text: &str, guard: Option<GuardId>) -> String {
    let cv = civ_of(g, gd);
    let at = g.cv[cv].cause_pos.unwrap_or(g.ch[g.gd[gd].ch].pos);
    let room = crate::dialogue::room_name(g, at);
    let man = guard.or(g.cv[cv].report_to).or(g.cv[cv].found_by);
    let mut others: Vec<GuardId> = g.civilians.iter().copied().filter(|&o| o != gd && g.gd[o].civ.is_some()).collect();
    js::sort_by(&mut others, |a, b| g.cv[civ_of(g, *b)].scream_at - g.cv[civ_of(g, *a)].scream_at);
    let other = match others.first() { Some(&o) => g.cv[civ_of(g, o)].name.clone(), None => String::from("hey") };
    let mut first = room.chars();
    let big_room = match first.next() { Some(h) => { let mut s: String = h.to_uppercase().collect(); s.push_str(first.as_str()); s } None => String::new() };
    let cs = match man { Some(x) => g.gd[x].callsign.clone(), None => String::from("control") };
    let c = &g.cv[cv];
    text.replace("{room}", &room).replace("{Room}", &big_room).replace("{other}", &other).replace("{name}", &c.name)
        .replace("{label}", &c.label).replace("{she}", &c.she).replace("{her}", &c.her).replace("{cs}", &cs)
}

/// Control's voice on the net; the timeline keeps it.   (civilians.ts control)
pub fn control(g: &mut crate::game::Game, gd: GuardId, text: &str) {
    use crate::chronicle::{ChronKind, ChronicleDataValue as D};
    let t = fill(g, gd, text, None);
    let line = format!("radio: CONTROL — {}", t);
    crate::game::game_msg(g, &line);
    crate::audio_iface::game_audio_like_play(g, "radio", None, Some(0.55), None);
    let mut data: JsMap<String, Option<D>> = JsMap::new();
    data.set(String::from("who"), Some(D::Text(String::from("CONTROL"))));
    data.set(String::from("text"), Some(D::Text(t)));
    let pos = g.ch[g.gd[gd].ch].pos;
    crate::chronicle::chronicle(g, ChronKind::Radio, Some(&data), Some(pos));
}

/// The scream: the sound of it, and the stamp the men and the other civilian hear it by.   (civilians.ts scream)
pub fn scream(g: &mut crate::game::Game, gd: GuardId, shout: Option<&str>) {
    let cv = civ_of(g, gd);
    g.cv[cv].scream_at = g.time;
    let head = bone_or(g, g.gd[gd].ch, crate::character::Bone::Head, [0.0, 1.6, 0.0]);
    let rate = if g.cv[cv].def.kind == crate::scene_level::CivilianDefKind::Worker { 1.15 } else { 1.5 };
    seam_audio_play_rate(g, "heldProtest", Some(head), 0.7, rate);
    if let Some(s) = shout { if !s.is_empty() { civ_say(g, gd, s, None, None); } }
    hear_scream(g, gd);
}

// ---------------------------------------------------------------- once a frame
/// Once a frame, after the shift has moved and heard.   (civilians.ts updateCivilians)
pub fn update_civilians(g: &mut crate::game::Game, dt: f64) {
    if g.civilians.is_empty() { return; }
    floor_of(g);
    if !civilians_on_floor(g) { floor_of(g).pending.clear(); return; }
    let mut i = floor_of(g).pending.len();
    while i > 0 {   // backwards: what a doing pushes lands behind the walk
        i -= 1;
        let now = g.time;
        let row = floor_of(g);
        if now >= row.pending[i].at {
            let mut p = row.pending.remove(i);
            run_pending(g, &mut p);
        }
    }
    evacuation_check(g);
    let one_seat = g.seats.len() == 1;
    let mut n = 0;
    while n < g.civilians.len() {   // (a live walk: the run can change the list)
        let cv = g.civilians[n];
        n += 1;
        if one_seat { update_one(g, cv, dt); }
        else { let view = crate::seats::view_for(g, cv); crate::seats::with_seat(g, view, &mut |g| update_one(g, cv, dt)); }
    }
    guards_notice(g, dt);
    greetings(g);
}

/// Her state changes, with its log line.   (civilians.ts setState)
pub fn set_state(g: &mut crate::game::Game, gd: GuardId, s: crate::civilians::CivState) {
    let cv = civ_of(g, gd);
    if g.cv[cv].state != s {
        let c = &g.cv[cv];
        let whither = match &c.target { Some(t) if s == CivState::Flee => format!(" ({})", t.name), _ => String::new() };
        let why = match c.cause {
            Some(k) if s == CivState::Alarmed => format!(" [{}{}]", k.as_str(), if c.armed { ", armed" } else { "" }),
            _ => String::new(),
        };
        let line = format!("→ {}{}{}", s.as_str(), whither, why);
        log(g, gd, &line);
    }
    g.cv[cv].state = s;
    g.cv[cv].state_at = g.time;
    g.gd[gd].stall_ref = None;
}

/// `yieldTo`: she gives way to what stands at `p`.
fn yield_to(g: &mut Game, c: CharId, p: V3, r: f64) {
    let (dx, dz) = (g.ch[c].pos[0] - p[0], g.ch[c].pos[2] - p[2]);
    let l = m::hyp2(dx, dz);
    if l < r && l > 1e-4 {
        g.ch[c].pos[0] += dx / l * (r - l);
        g.ch[c].pos[2] += dz / l * (r - l);
    }
}

/// One civilian's frame: perception, the little state machine, the figure, solidity.   (civilians.ts updateOne)
pub fn update_one(g: &mut crate::game::Game, gd: GuardId, dt: f64) {
    use crate::game::GuardState;
    let cv = civ_of(g, gd);
    let c = g.gd[gd].ch;
    if g.cv[cv].state == CivState::Gone { return; }
    if g.gd[gd].state == GuardState::Dead {
        g.ch[c].anim.act = None; g.gd[gd].speed = 0.0; g.ch[c].vel = [0.0, 0.0, 0.0]; g.ch[c].anim.speed = 0.0;
        crate::character::character_update(g, c, dt);
        crate::game::game_body_thud(g, c);
        return;
    }
    if g.gd[gd].held.is_some() {
        if g.cv[cv].state != CivState::Held {
            set_state(g, gd, CivState::Held);
            g.ch[c].anim.act = None; g.ch[c].anim.crouch_target = 0.0;
            g.gd[gd].path = Vec::new(); g.gd[gd].path_goal = None; g.cv[cv].target = None;
        }
        crate::character::character_update(g, c, dt);
        let sp = g.gd[gd].speed;
        step_sound(g, gd, sp);
        return;
    }
    if g.cv[cv].state == CivState::Held { released(g, gd); }
    if g.ai_enabled { see(g, gd, dt); hear(g, gd); bodies(g, gd); } else { g.gd[gd].awareness = m::max(0.0, g.gd[gd].awareness - dt * 0.5); }
    let mut move_ = 0.0;
    let mut crouch = 0.0;
    match g.cv[cv].state {
        CivState::Work => { move_ = work(g, gd, dt).move_; }
        CivState::Startled => startled(g, gd, dt),
        CivState::Alarmed => alarmed(g, gd, dt),
        CivState::Flee | CivState::Evacuate => { move_ = flee(g, gd, dt); }
        CivState::Report => report(g, gd, dt),
        CivState::Phone => phone(g, gd, dt),
        CivState::Cower => {
            if g.cv[cv].lead.is_some() { let r = follow(g, gd, dt); move_ = r.move_; crouch = r.crouch; }
            else { let r = cower(g, gd, dt); move_ = r.move_; crouch = r.crouch; }
        }
        CivState::Held | CivState::Gone => {}
    }
    if g.cv[cv].state == CivState::Gone { return; }
    g.ch[c].aim_yaw = v3::approach_angle(g.ch[c].aim_yaw, g.ch[c].body_yaw, 6.0 * dt);
    g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, 0.0, 4.0, dt);
    {
        let a = &mut g.ch[c].anim;
        a.speed = move_; a.upper = crate::character::UpperMode::None; a.stance = crate::character::Stance::None;
        a.crouch_target = crouch; a.reverse = false; a.deliberate = 0.0;
    }
    let cower_now = g.cv[cv].state == CivState::Cower;
    let led = g.cv[cv].lead.is_some();
    let bound = if cower_now && led && crate::snatch::snatch_is_tied(g, gd) { 1.0 }
        else if cower_now && move_ == 0.0 && !led {
            if crate::rescue::rescue_is_hostage(g, gd) || crate::snatch::snatch_is_tied(g, gd) { 1.0 }
            else if g.cv[cv].complied && g.time < g.cv[cv].comply_until { 2.0 } else { 0.0 }
        } else { 0.0 };
    g.ch[c].anim.bound = bound;
    crate::character::character_update(g, c, dt);
    step_sound(g, gd, move_);
    if g.cv[cv].seated { return; }
    let r = g.ch[c].radius;
    let mut p = g.ch[c].pos;
    crate::scene_collision::collide_circle(g, &mut p, r, 0.2, 1.5, None);
    crate::scene_collision::clamp_to_world(g, &mut p, r);
    g.ch[c].pos = p;
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if g.gd[o].state != GuardState::Dead { let at = g.ch[g.gd[o].ch].pos; yield_to(g, c, at, 0.6); }
    }
    let pl = g.player;
    if !g.pl[pl].down { let at = g.ch[g.pl[pl].ch].pos; yield_to(g, c, at, 0.62); }
    let mut i = 0;
    while i < g.civilians.len() {
        let o = g.civilians[i];
        i += 1;
        if o != gd && g.gd[o].state != GuardState::Dead && g.cv[civ_of(g, o)].state != CivState::Gone {
            let at = g.ch[g.gd[o].ch].pos;
            yield_to(g, c, at, 0.55);
        }
    }
}

/// The sound of her feet: nothing of the game reads it.   (civilians.ts stepSound)
pub fn step_sound(g: &mut crate::game::Game, gd: GuardId, move_: f64) {
    let c = g.gd[gd].ch;
    let prev = g.gd[gd].step_phase;
    g.gd[gd].step_phase = g.ch[c].anim.phase;
    if move_ > 0.4 && ((prev < 0.5) != (g.gd[gd].step_phase < 0.5)) {
        let p = g.ch[c].pos;
        crate::audio_iface::game_audio_like_footstep(g, p, if move_ > 2.0 { 0.7 } else { 0.3 }, false, None);
    }
}

// ---------------------------------------------------------------- perception
/// Her eye this frame: busy at a task, scanning once frightened, else the walking-about eye.   (civilians.ts eyeOf)
pub fn eye_of(g: &mut crate::game::Game, gd: GuardId) -> crate::knobs::CivKEye {
    use crate::knobs::CivKEye;
    let st = g.cv[civ_of(g, gd)].state;
    if st != CivState::Work && st != CivState::Held {
        let e = g.k.civ.eye_alarmed;
        return CivKEye { range: e.range, half_deg: e.half_deg, near_m: e.near_m, near_deg: e.near_deg };
    }
    let task = task_at(g, gd);
    if st == CivState::Work && g.gd[gd].wait > 0.0 && task != crate::scene_level::CivilianTask::Stand {
        let e = g.k.civ.eye_busy;
        return CivKEye { range: e.range, half_deg: e.half_deg, near_m: e.near_m, near_deg: e.near_deg };
    }
    g.k.civ.eye
}

/// Sam in her eye: awareness fills at the guards' sight rate and drains when he is out of it.   (civilians.ts see)
pub fn see(g: &mut crate::game::Game, gd: GuardId, dt: f64) {
    use crate::character::Bone;
    let cv = civ_of(g, gd);
    let c = g.gd[gd].ch;
    let pl = g.player;
    let pc = g.pl[pl].ch;
    let eye = bone_or(g, c, Bone::Head, [0.0, 1.6, 0.0]);
    let head_dir = crate::character::character_head_dir(g, c);
    let look = v3::normalize(v3::lerp3(g.ch[c].chest_dir, head_dir, 0.5));
    let chest = bone_or(g, pc, Bone::Chest, [0.0, 1.1, 0.0]);
    let key = format!("civS{}", g.ch[c].id);
    let probe = crate::sensors::sensors_segment(g, &key, eye, chest, None).unwrap_or(1.0);
    g.gd[gd].smoke_trans = probe * crate::weather::air_trans(g, eye, chest);
    let e = eye_of(g, gd);
    let smoke = g.gd[gd].smoke_trans;
    let vis = crate::guards::eye_contact(g, crate::guards::Eye { pos: eye, look, range: e.range, half_deg: e.half_deg, near_m: e.near_m, near_deg: e.near_deg, v_half_deg: None }, pc, true, false, smoke, None);
    let decay = if g.cv[cv].state == CivState::Work { 0.08 } else { 0.05 };
    if !vis.visible { g.gd[gd].awareness = m::max(0.0, g.gd[gd].awareness - dt * decay); return; }
    let holding_other = g.pl[pl].holding.as_ref().map_or(false, |h| h.g != gd);
    let violent = crate::game::player_gun_in_hand(g, pl).is_some() || g.pl[pl].takedown.is_some() || holding_other || g.time - g.pl[pl].last_fire_t < 2.0;
    let gun = violent && vis.dist < g.k.civ.gun_m;
    let rate = crate::guards::sight_gain(g, gd, vis.dist) * g.k.civ.keen_k * (if gun { g.k.civ.gun_k } else { 1.0 });
    if rate <= 0.05 { g.gd[gd].awareness = m::max(0.0, g.gd[gd].awareness - dt * decay); return; }
    g.gd[gd].awareness = v3::clamp(g.gd[gd].awareness + rate * dt, 0.0, 1.0);
    if rate > 0.25 {
        let at = g.ch[pc].pos;
        let now = g.time;
        let spot_at = g.k.civ.spot_at;
        let aware = g.gd[gd].awareness;
        let x = &mut g.cv[cv];
        x.seen_at = now;
        x.seen_pos = Some(at);
        if gun { x.saw_gun = true; }
        if x.state != CivState::Work && x.state != CivState::Startled && aware >= spot_at {
            x.cause_pos = Some(at);
            if gun && !x.armed {
                x.armed = true;
                if x.cause == Some(CivCause::Sam) { x.cause = Some(CivCause::Gun); }
            }
        }
    }
}

/// The hearing model's share: this frame's events past her marker.   (civilians.ts hear)
pub fn hear(g: &mut crate::game::Game, gd: GuardId) {
    use crate::director::EventKind as K;
    let cv = civ_of(g, gd);
    let c = g.gd[gd].ch;
    let mut i = 0;
    while i < g.events.len() {   // (a live walk: what she does here can push an event)
        let ix = i;
        i += 1;
        if g.events[ix].id.is_none() { g.event_seq += 1.0; g.events[ix].id = Some(g.event_seq); }
        let e = g.events[ix];
        let eid = e.id.unwrap();
        if eid <= g.gd[gd].heard_up_to { continue; }
        g.gd[gd].heard_up_to = m::max(g.gd[gd].heard_up_to, eid);
        let cid = g.ch[c].id;
        if e.who == Some(cid) { continue; }
        let pos = g.ch[c].pos;
        let d_straight = v3::dist(e.pos, pos);
        let gunfire = (e.kind == K::Shot || e.kind == K::GuardShot) && e.level.unwrap_or(0.0) >= 1.0;
        let kick = e.kind == K::Kick;
        let bang = e.kind == K::Bang;
        if e.kind == K::LightOut || e.kind == K::Step { continue; }
        let reach = if bang || gunfire { 80.0 } else if kick { 40.0 } else if e.kind == K::Glass { g.k.glass.hear_m } else if e.kind == K::Whistle { 10.0 } else { 22.0 };
        if d_straight > reach { continue; }
        let ear = v3::add(pos, [0.0, 1.5, 0.0]);
        let prop = if e.kind == K::Glass { crate::guards::glass_heard(g, e, ear) } else { crate::weather::hear_path(g, v3::add(e.pos, [0.0, 1.0, 0.0]), ear, None) };
        let (d, carry) = (prop.path_len, prop.carry);
        let heard_at: V3 = [prop.via[0], 0.0, prop.via[2]];
        if gunfire || bang { to_alarmed(g, gd, if bang { CivCause::Bang } else { CivCause::Gunfire }, heard_at, true); continue; }
        if kick { to_alarmed(g, gd, CivCause::Noise, heard_at, false); continue; }
        if e.kind == K::Shot && !gunfire {
            if d < 7.0 * carry * m::max(0.3, e.level.unwrap_or(0.6) / 0.6) + 0.5 { to_alarmed(g, gd, CivCause::Shot, heard_at, true); }
            continue;
        }
        if e.kind == K::Glass {
            let a = e.level.unwrap_or(g.k.glass.hear_level) * v3::clamp(1.0 - d / g.k.glass.hear_m, 0.15, 1.0) * carry;
            if a > 0.25 { startle_or_worse(g, gd, heard_at); }
            continue;
        }
        if (e.kind == K::Door || e.kind == K::Prop) && e.loud && d < 8.0 * carry { startle_or_worse(g, gd, heard_at); continue; }
        if e.kind == K::Whistle && prop.direct && d < 8.0 {
            if g.cv[cv].state == CivState::Work { to_startled(g, gd, heard_at, true); }
            continue;
        }
    }
}

/// A crash right by her: the first startles, a second inside the startle is somebody IN here.   (civilians.ts startleOrWorse)
pub fn startle_or_worse(g: &mut crate::game::Game, gd: GuardId, at: V3) {
    let cv = civ_of(g, gd);
    let st = g.cv[cv].state;
    if st == CivState::Work { to_startled(g, gd, at, true); }
    else if st == CivState::Startled || (st == CivState::Cower && !g.cv[cv].complied) {
        let armed = g.cv[cv].armed;
        to_alarmed(g, gd, CivCause::Noise, at, armed);
    }
}

/// `test` of bodies: does this body frighten her now.
fn body_test(g: &mut Game, gd: GuardId, o: GuardId, eye: V3, look: V3, e: crate::knobs::CivKEye) -> bool {
    let cv = civ_of(g, gd);
    if o == gd || g.gd[o].state != crate::game::GuardState::Dead || g.cv[cv].known_bodies.has(o) { return false; }
    let oc = g.gd[o].ch;
    let bp = match g.ch[oc].bones.get(crate::character::Bone::Hips) { Some(p) => *p, None => g.ch[oc].pos };
    let to = v3::sub(bp, eye);
    let l = v3::len(to);
    if l > g.k.civ.body_m || l > e.range + 2.0 { return false; }
    if m::hyp2(to[0], to[2]) > 0.5 && v3::dot([look[0], 0.0, look[2]], v3::normalize([to[0], 0.0, to[2]])) < m::cos(m::max(e.half_deg, 45.0) * v3::DEG) { return false; }
    if crate::scene_collision::segment_blocked(g, eye, v3::add(bp, [0.0, 0.3, 0.0]), None, None, None) { return false; }
    g.cv[cv].known_bodies.add(o);
    let at = crate::guards::body_pos(g, o);
    to_alarmed(g, gd, CivCause::Body, at, true);
    true
}

/// Bodies in her eye: each frightens her once; every twelfth frame, staggered by id.   (civilians.ts bodies)
pub fn bodies(g: &mut crate::game::Game, gd: GuardId) {
    let cv = civ_of(g, gd);
    let c = g.gd[gd].ch;
    if matches!(g.cv[cv].state, CivState::Flee | CivState::Evacuate | CivState::Alarmed | CivState::Report | CivState::Phone) { return; }
    if js::rem(m::floor(g.time * 60.0) + g.ch[c].id, 12.0) != 0.0 { return; }
    let eye = bone_or(g, c, crate::character::Bone::Head, [0.0, 1.6, 0.0]);
    let look = g.ch[c].chest_dir;
    let e = eye_of(g, gd);
    let mut i = 0;
    while i < g.guards.len() { let o = g.guards[i]; i += 1; if body_test(g, gd, o, eye, look, e) { return; } }
    let mut i = 0;
    while i < g.civilians.len() { let o = g.civilians[i]; i += 1; if body_test(g, gd, o, eye, look, e) { return; } }
}

/// Her light meter, for the men's eyes on her.   (civilians.ts civVisibility)
pub fn civ_visibility(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> f64 {
    let cv = civ_of(g, gd);
    let c = g.gd[gd].ch;
    let e = crate::guards::meter_of(g, c, "civ");
    let seen = crate::game::game_visibility_of(g, e);
    g.cv[cv].vis = v3::damp(g.cv[cv].vis, seen, 6.0, dt);
    g.gd[gd].visibility = g.cv[cv].vis;
    g.gd[gd].irradiance = e;
    g.cv[cv].vis
}

// ---------------------------------------------------------------- WORK: the loop and its tasks
/// The task at her current mark.   (civilians.ts taskAt)
pub fn task_at(g: &mut crate::game::Game, gd: GuardId) -> crate::scene_level::CivilianTask {
    let wp = g.gd[gd].wp;
    let cv = civ_of(g, gd);
    match g.cv[cv].def.tasks.as_ref().and_then(|t| t.get(wp)) { Some(t) => *t, None => crate::scene_level::CivilianTask::Stand }
}

/// Does this civilian shove the furniture this frame: not while calm.   (civilians.ts civilianPushesProps)
pub fn civilian_pushes_props(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let cv = match g.gd[gd].civ { Some(c) => c, None => return true };
    let st = g.cv[cv].state;
    st != CivState::Work && st != CivState::Startled && st != CivState::Gone
}

/// A civilian heading for / in his chair is no contact for the furniture.   (civilians.ts civilianSeated)
pub fn civilian_seated(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let cv = match g.gd[gd].civ { Some(c) => c, None => return false };
    if g.cv[cv].state != CivState::Work { return false; }
    if g.cv[cv].seated { return true; }
    if task_at(g, gd) != crate::scene_level::CivilianTask::Sit { return false; }
    let r = g.gd[gd].route;
    let n = g.tab.route[r].points.len();
    let mark = g.tab.route[r].points[g.gd[gd].wp % n];
    v3::dist_xz(mark, g.ch[g.gd[gd].ch].pos) < 1.6
}

/// A row of TASK_CLIP: the whole-figure clip a task plays (every one loops), and how long it takes to get up out of it.
#[derive(Clone, Copy)]
struct TaskClip { clip: &'static str, rate: f64, fade_in: f64, fade_out: f64, rise: Option<f64> }
/// TASK_CLIP   (civilians.ts:675)
fn task_clip(task: crate::scene_level::CivilianTask) -> Option<TaskClip> {
    use crate::scene_level::CivilianTask as T;
    match task {
        T::Stand => None,
        T::Wipe => Some(TaskClip { clip: "Interact", rate: 0.85, fade_in: 0.4, fade_out: 0.4, rise: None }),
        T::Bin => Some(TaskClip { clip: "Fixing_Kneeling", rate: 0.8, fade_in: 0.6, fade_out: 0.7, rise: Some(0.8) }),
        T::Sit => Some(TaskClip { clip: "Sitting_Idle_Loop", rate: 1.0, fade_in: 0.7, fade_out: 0.8, rise: Some(1.0) }),
        T::Cooler => Some(TaskClip { clip: "Interact", rate: 0.6, fade_in: 0.4, fade_out: 0.4, rise: None }),
        T::Printer => Some(TaskClip { clip: "Interact", rate: 0.7, fade_in: 0.4, fade_out: 0.4, rise: None }),
        T::Shelf => Some(TaskClip { clip: "PickUp_Table", rate: 0.7, fade_in: 0.4, fade_out: 0.5, rise: None }),
    }
}
/// `C.def.face?.[gd.wp]`
fn face_at(g: &Game, cv: CivId, wp: usize) -> Option<f64> { g.cv[cv].def.face.as_ref().and_then(|f| f.get(wp).copied()) }

/// Her loop: the two lines of fright, then the task at her mark, the settle onto it, or the walk to it.   (civilians.ts work)
pub fn work(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> crate::civilians::Work {
    use crate::scene_level::CivilianTask as T;
    let cv = civ_of(g, gd);
    let c = g.gd[gd].ch;
    let r = g.gd[gd].route;
    let mut move_ = 0.0;
    if g.cv[cv].rise_t > 0.0 {
        g.cv[cv].rise_t -= dt; g.gd[gd].speed = 0.0; g.ch[c].vel = [0.0, 0.0, 0.0];
        return Work { move_: 0.0 };
    }
    let seen = g.cv[cv].seen_pos.unwrap_or(g.ch[g.pl[g.player].ch].pos);
    if g.gd[gd].awareness >= g.k.civ.spot_at {
        let saw = g.cv[cv].saw_gun;
        to_alarmed(g, gd, if saw { CivCause::Gun } else { CivCause::Sam }, seen, saw);
        return Work { move_: 0.0 };
    }
    if g.gd[gd].awareness >= g.k.civ.glimpse_at && g.time - g.cv[cv].shrug_at > g.k.civ.startle_again_secs {
        to_startled(g, gd, seen, false);
        return Work { move_: 0.0 };
    }
    let n = g.tab.route[r].points.len();
    let wp = g.gd[gd].wp;
    let target = g.tab.route[r].points[wp % n];
    if g.gd[gd].wait > 0.0 {
        g.gd[gd].wait -= dt;
        g.cv[cv].task_t += dt;
        g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 8.0, dt);
        g.ch[c].vel = [0.0, 0.0, 0.0];
        let task = task_at(g, gd);
        let k = task_clip(task);
        let face = face_at(g, cv, wp);
        if let Some(f) = face { if f.is_finite() { g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, f, 3.0 * dt); } }
        let settled = match face { None => true, Some(f) => m::abs(v3::wrap_angle(g.ch[c].body_yaw - f)) < 0.35 };
        g.ch[c].anim.act = match k {
            Some(k) if settled => Some(crate::character::AnimAct { clip: String::from(k.clip), loop_: Some(true), rate: Some(k.rate), in_: Some(k.fade_in), out: Some(k.fade_out) }),
            _ => None,
        };
        g.cv[cv].seated = task == T::Sit && settled;
        if g.cv[cv].seated {
            let f = face.unwrap_or(g.ch[c].body_yaw);
            let tx = target[0] + m::sin(f) * SIT_ADVANCE;
            let tz = target[2] + m::cos(f) * SIT_ADVANCE;
            let k = 1.0 - m::exp(-5.0 * dt);
            g.ch[c].pos[0] += (tx - g.ch[c].pos[0]) * k;
            g.ch[c].pos[2] += (tz - g.ch[c].pos[2]) * k;
        }
        let phase = g.gd[gd].look_phase;
        g.ch[c].anim.look_yaw_extra = if task == T::Sit { m::sin(phase * 0.4) * 0.12 } else if task == T::Stand { m::sin(phase * 0.9) * 0.4 } else { 0.0 };
        g.gd[gd].look_phase += dt;
        if g.gd[gd].wait <= 0.0 {
            end_task(g, gd);
            g.gd[gd].wp = (g.gd[gd].wp + 1) % n;
        }
    } else if g.cv[cv].settle_t >= 0.0 {
        g.cv[cv].settle_t += dt;
        g.ch[c].anim.act = None;
        let dx = target[0] - g.ch[c].pos[0];
        let dz = target[2] - g.ch[c].pos[2];
        let d = m::hyp2(dx, dz);
        if d < 0.04 || g.cv[cv].settle_t > 1.5 {
            g.cv[cv].settle_t = -1.0;
            g.gd[gd].speed = 0.0;
            g.ch[c].vel = [0.0, 0.0, 0.0];
            g.gd[gd].wait = g.tab.route[r].wait.get(wp).copied().unwrap_or(0.0);
            g.cv[cv].task_t = 0.0;
            if g.gd[gd].wait <= 0.0 { g.gd[gd].wp = (g.gd[gd].wp + 1) % n; }
        } else {
            let sp = m::min(0.8, d / 0.4 + 0.15);
            g.gd[gd].speed = sp;
            let st = m::min(d, sp * dt);
            g.ch[c].pos[0] += dx / d * st;
            g.ch[c].pos[2] += dz / d * st;
            g.ch[c].vel = [dx / d * sp, 0.0, dz / d * sp];
            move_ = sp;
            if let Some(f) = face_at(g, cv, wp) { g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, f, 3.0 * dt); }
        }
    } else {
        g.ch[c].anim.act = None;
        g.ch[c].anim.look_yaw_extra = 0.0;
        crate::guards::go_to(g, gd, target, None);
        let walk = g.k.civ.walk_speed;
        let arrived = crate::guards::follow_path(g, gd, dt, walk);
        move_ = g.gd[gd].speed;
        if arrived {
            g.gd[gd].stall_ref = None;
            g.cv[cv].settle_t = 0.0;
        } else if crate::guards::not_getting_anywhere(g, gd, dt, 8.0) {
            let line = format!("no way to mark {} — skipping it", g.gd[gd].wp);
            log(g, gd, &line);
            g.gd[gd].path = Vec::new(); g.gd[gd].path_goal = None; g.gd[gd].stall_ref = None;
            g.gd[gd].wp = (g.gd[gd].wp + 1) % n;
        }
    }
    Work { move_ }
}

/// Leaving a mark: the clip comes off, and a seated / kneeling woman takes a moment to be on her feet.   (civilians.ts endTask)
pub fn end_task(g: &mut crate::game::Game, gd: GuardId) {
    let cv = civ_of(g, gd);
    let c = g.gd[gd].ch;
    let k = task_clip(task_at(g, gd));
    g.ch[c].anim.act = None;
    let up = g.cv[cv].seated || crate::character::anim_act_shown(g, c) > 0.5;
    g.cv[cv].rise_t = if up { k.and_then(|k| k.rise).unwrap_or(0.0) } else { 0.0 };
    g.cv[cv].task_t = 0.0;
    g.cv[cv].settle_t = -1.0;
    if g.cv[cv].seated {
        let f = face_at(g, cv, g.gd[gd].wp).unwrap_or(g.ch[c].body_yaw);
        g.ch[c].pos[0] -= m::sin(f) * SIT_ADVANCE;
        g.ch[c].pos[2] -= m::cos(f) * SIT_ADVANCE;
        crate::character::character_update(g, c, 0.0);
    }
    g.cv[cv].seated = false;
}

/// Back to the loop from wherever she is: the nearest mark is the next, walked to.   (civilians.ts toWork)
pub fn to_work(g: &mut crate::game::Game, gd: GuardId, line: bool) {
    let cv = civ_of(g, gd);
    let c = g.gd[gd].ch;
    let r = g.gd[gd].route;
    let mut best = 0;
    let mut bd = m::INF;
    let at = g.ch[c].pos;
    for (i, p) in g.tab.route[r].points.iter().enumerate() {
        let d = v3::dist_xz(*p, at);
        if d < bd { bd = d; best = i; }
    }
    g.gd[gd].wp = best; g.gd[gd].wait = 0.0; g.gd[gd].path = Vec::new(); g.gd[gd].path_goal = None;
    g.ch[c].anim.act = None;
    let x = &mut g.cv[cv];
    x.look_at = None; x.target = None; x.report_to = None; x.settle_t = -1.0; x.rise_t = 0.0; x.seated = false;
    g.gd[gd].awareness = m::min(g.gd[gd].awareness, 0.1);
    if line {
        let lines = crate::dialogue::civ_lines();
        let s = pick(g, gd, &lines.settle);
        civ_say(g, gd, &s, None, None);
    }
    set_state(g, gd, CivState::Work);
}

// ---------------------------------------------------------------- STARTLED: she stops and looks
/// A glimpse or a noise stops her at her work.   (civilians.ts toStartled)
pub fn to_startled(g: &mut crate::game::Game, gd: GuardId, at: V3, sound: bool) {
    let cv = civ_of(g, gd);
    let c = g.gd[gd].ch;
    if g.cv[cv].state != CivState::Work { return; }
    end_task(g, gd);
    g.cv[cv].rise_t = 0.0;
    g.ch[c].anim.act = None;
    g.gd[gd].path = Vec::new(); g.gd[gd].path_goal = None;
    g.cv[cv].look_at = Some(at);
    g.cv[cv].look_until = g.time + g.k.civ.startle_secs;
    g.stats.civilians.startled += 1;
    let lines = crate::dialogue::civ_lines();
    let pool = no_security(g, &lines.glimpse);
    let s = pick(g, gd, &pool);
    civ_say(g, gd, &s, None, None);
    let line = format!("startled by {} at ({:.1}, {:.1})", if sound { "a noise" } else { "a glimpse" }, at[0], at[2]);
    log(g, gd, &line);
    set_state(g, gd, CivState::Startled);
}

/// She stops and looks, until the fright fills or she talks herself out of it.   (civilians.ts startled)
pub fn startled(g: &mut crate::game::Game, gd: GuardId, dt: f64) {
    let cv = civ_of(g, gd);
    let c = g.gd[gd].ch;
    g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 8.0, dt);
    g.ch[c].vel = [0.0, 0.0, 0.0];
    g.ch[c].anim.act = None;
    let at = if g.cv[cv].seen_at > g.cv[cv].state_at { g.cv[cv].seen_pos } else { g.cv[cv].look_at };
    if let Some(a) = at {
        let yaw_to = m::atan2(a[0] - g.ch[c].pos[0], a[2] - g.ch[c].pos[2]);
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, yaw_to, 2.5 * dt);
        g.ch[c].anim.look_yaw_extra = v3::clamp(v3::wrap_angle(yaw_to - g.ch[c].body_yaw), -0.9, 0.9);
    }
    if g.gd[gd].awareness >= g.k.civ.spot_at {
        let saw = g.cv[cv].saw_gun;
        let where_ = g.cv[cv].seen_pos.or(at).unwrap_or(g.ch[c].pos);
        to_alarmed(g, gd, if saw { CivCause::Gun } else { CivCause::Sam }, where_, saw);
        return;
    }
    if g.time >= g.cv[cv].look_until && g.gd[gd].awareness < g.k.civ.glimpse_at * 0.5 {
        g.cv[cv].shrug_at = g.time;
        let lines = crate::dialogue::civ_lines();
        let s = pick(g, gd, &lines.shrug);
        civ_say(g, gd, &s, None, None);
        log(g, gd, "talked herself out of it");
        g.cv[cv].look_at = None;
        set_state(g, gd, CivState::Work);
        g.gd[gd].wait = m::max(g.gd[gd].wait, 0.0);
    } else if g.time >= g.cv[cv].look_until + 6.0 {
        g.cv[cv].shrug_at = g.time;
        g.cv[cv].look_at = None;
        set_state(g, gd, CivState::Work);
    }
}

// ---------------------------------------------------------------- ALARMED: the frozen beat, then the run
/// How bad a cause is: a worse one replaces what she will tell.   (civilians.ts toAlarmed rank)
fn rank(k: Option<CivCause>) -> i32 {
    match k {
        None => -1,
        Some(CivCause::Noise) => 0,
        Some(CivCause::Scream) | Some(CivCause::Panel) => 1,
        Some(CivCause::Shot) | Some(CivCause::Sam) => 2,
        Some(CivCause::Bang) | Some(CivCause::Gunfire) => 3,
        Some(CivCause::Body) => 4,
        Some(CivCause::Gun) => 5,
        Some(CivCause::Grabbed) => 6,
    }
}

/// The fright: the news is folded in; from a calm-ish state she freezes with a gasp, then runs.   (civilians.ts toAlarmed)
pub fn to_alarmed(g: &mut crate::game::Game, gd: GuardId, cause: crate::civilians::CivCause, at: V3, armed: bool) {
    use crate::chronicle::{ChronCountKey, ChronKind, ChronicleNDataValue as N};
    let cv = civ_of(g, gd);
    let c = g.gd[gd].ch;
    let st = g.cv[cv].state;
    if g.gd[gd].state == crate::game::GuardState::Dead || g.gd[gd].held.is_some() || st == CivState::Gone || st == CivState::Evacuate { return; }
    let was_armed = g.cv[cv].armed;
    let fresh = st == CivState::Work || st == CivState::Startled || st == CivState::Cower
        || (st == CivState::Report && armed && !was_armed)
        || (st == CivState::Phone && armed && !was_armed && cause != CivCause::Scream);
    if rank(Some(cause)) >= rank(g.cv[cv].cause) || g.cv[cv].cause_pos.is_none() { g.cv[cv].cause = Some(cause); g.cv[cv].cause_pos = Some(at); }
    g.cv[cv].armed = was_armed || armed;
    g.cv[cv].cause_at = g.time;
    if !fresh { return; }
    if st == CivState::Cower && g.cv[cv].complied && g.time < g.cv[cv].comply_until && cause != CivCause::Grabbed { return; }
    if st == CivState::Cower && (g.cv[cv].refled || cause == CivCause::Noise || cause == CivCause::Scream) { return; }
    let was_cower = st == CivState::Cower;
    end_task(g, gd);
    g.cv[cv].rise_t = 0.0;
    g.ch[c].anim.act = None;
    g.gd[gd].path = Vec::new(); g.gd[gd].path_goal = None;
    {
        let x = &mut g.cv[cv];
        x.look_at = None;
        x.alarm_n += 1.0; x.reported = false; x.acked = false; x.phoned = false; x.report_to = None;
        x.bad_targets.clear(); x.target = None; x.found_t = 0.0; x.pin_target = false;
    }
    if was_cower { g.cv[cv].refled = true; g.stats.civilians.refled += 1; } else { g.cv[cv].refled = false; }
    let [f0, f1] = g.k.civ.freeze;
    let beat = if was_cower { 0.25 } else { f0 + g.cv[cv].dice.roll() * (f1 - f0) };
    g.cv[cv].freeze_until = g.time + beat;
    g.stats.civilians.alarmed += 1;
    let saw_him = cause == CivCause::Sam || cause == CivCause::Gun || cause == CivCause::Grabbed;
    if saw_him {
        g.stats.civilians.sam_seen += 1;
        let mut data: JsMap<String, Option<N>> = JsMap::new();
        data.set(String::from("who"), Some(N::Text(g.cv[cv].name.clone())));
        data.set(String::from("cause"), Some(N::Text(String::from(cause.as_str()))));
        data.set(String::from("armed"), Some(N::Bool(g.cv[cv].armed)));
        let pos = g.ch[c].pos;
        crate::chronicle::chronicle_n(g, ChronKind::CivilianSpotted, ChronCountKey::CivSeen, Some(&data), Some(pos));
    }
    g.gd[gd].awareness = m::max(g.gd[gd].awareness, if saw_him { 1.0 } else { 0.5 });
    set_state(g, gd, CivState::Alarmed);
    let lines = crate::dialogue::civ_lines();
    let pool = match lines.gasp.get(String::from(cause.as_str())) { Some(p) => p, None => lines.gasp.get(String::from("sam")).unwrap() };
    let s = pick(g, gd, pool);
    scream(g, gd, Some(&s));
}

/// Rooted, staring at it, until the freeze ends; then she picks whom to run to.   (civilians.ts alarmed)
pub fn alarmed(g: &mut crate::game::Game, gd: GuardId, dt: f64) {
    let cv = civ_of(g, gd);
    let c = g.gd[gd].ch;
    g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 10.0, dt);
    g.ch[c].vel = [0.0, 0.0, 0.0];
    g.ch[c].anim.act = None;
    if let Some(a) = g.cv[cv].cause_pos {
        let yaw_to = m::atan2(a[0] - g.ch[c].pos[0], a[2] - g.ch[c].pos[2]);
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, yaw_to, 5.0 * dt);
    }
    g.ch[c].anim.look_yaw_extra = 0.0;
    if g.time < g.cv[cv].freeze_until { return; }
    match pick_target(g, gd, None) {
        None => { log(g, gd, "nowhere to run: cowering here"); to_cower(g, gd, false); }
        Some(t) => to_flee(g, gd, &t, false),
    }
}

/// Her hands come off Sam's arm on her feet: the fright of her life, from where he stands.   (civilians.ts released)
pub fn released(g: &mut crate::game::Game, gd: GuardId) {
    let cv = civ_of(g, gd);
    let c = g.gd[gd].ch;
    g.ch[c].anim.held_pose = None;
    let talk = g.gd[gd].talk;
    g.tab.game_talk[talk].held_since = -1.0;
    if g.cv[cv].complied { to_cower(g, gd, true); return; }
    g.cv[cv].state = CivState::Work;   // (so toAlarmed takes it as fresh)
    let at = g.ch[g.pl[g.player].ch].pos;
    to_alarmed(g, gd, CivCause::Grabbed, at, true);
}

// ---------------------------------------------------------------- FLEE: whom to, and the run
/// Metres by path, and the path; no path is Infinity.   (civilians.ts pathMetres)
pub fn path_metres(g: &mut crate::game::Game, from: V3, to: V3) -> crate::civilians::PathMetres {
    let p = match crate::scene_collision::nav_grid_find_path(g, from, to, Some(6000.0), None) { Some(p) if !p.is_empty() => p, _ => return PathMetres { m: m::INF, path: None } };
    let mut s = 0.0;
    let mut prev = from;
    for q in p.iter() { s += m::hyp2(q[0] - prev[0], q[2] - prev[2]); prev = *q; }
    PathMetres { m: s, path: Some(p) }
}

/// May this man be run to and told.   (civilians.ts tellable)
pub fn tellable(g: &mut crate::game::Game, o: GuardId) -> bool {
    let x = &g.gd[o];
    x.state != crate::game::GuardState::Dead && x.team == crate::teams::Team::Guard && x.held.is_none() && x.script.is_none() && x.state != crate::game::GuardState::Alert && g.guards.contains(&o)
}

/// A spot's identity (`badTargets` held the object): its place in civSpots, the phones first, then the exits.
fn spot_key(g: &mut Game, kind: CivTargetKind, spot: &CivSpot) -> CivBadTarget {
    let spots = civ_spots(g);
    let same = |s: &CivSpot| s.name == spot.name && s.pos == spot.pos;
    let n = match kind {
        CivTargetKind::Phone => spots.phones.iter().position(same),
        _ => spots.exits.iter().position(same).map(|i| spots.phones.len() + i),
    };
    CivBadTarget::Spot(n.map_or(u32::MAX, |i| i as u32))
}
/// `T.guard ?? T.spot`, as what it is known by.
fn target_key(g: &mut Game, t: &CivTarget) -> CivBadTarget {
    match (t.guard, &t.spot) {
        (Some(o), _) => CivBadTarget::Guard(o),
        (None, Some(s)) => spot_key(g, t.kind, s),
        (None, None) => CivBadTarget::Spot(u32::MAX),
    }
}

/// A candidate of pickTarget, and what she would pay for it.
struct Cand { t: CivTarget, cost: f64 }
/// `consider` of pickTarget.
fn consider(g: &mut Game, cv: CivId, cands: &mut Vec<Cand>, t: CivTarget, key: CivBadTarget, adj: f64, from: V3, threat: Option<V3>) {
    if g.cv[cv].bad_targets.has(key) { return; }
    let metres = path_metres(g, from, t.pos).m;
    if !metres.is_finite() { return; }
    let mut cost = metres + adj;
    if let Some(th) = threat {
        if v3::dist_xz(th, t.pos) < v3::dist_xz(th, from) - 1.0 && v3::dist_xz(from, t.pos) > 3.0 { cost += g.k.civ.past_threat_m; }
    }
    cands.push(Cand { t, cost });
}

/// Whom / what she runs to: the nearest by path, a man nearer than he is, a door further, past the fright further still.   (civilians.ts pickTarget)
pub fn pick_target(g: &mut crate::game::Game, gd: GuardId, opts: Option<crate::civilians::PickTargetOpts>) -> Option<crate::civilians::CivTarget> {
    let cv = civ_of(g, gd);
    let from = g.ch[g.gd[gd].ch].pos;
    let threat = g.cv[cv].cause_pos;
    let exits_only = opts.and_then(|o| o.exits_only).unwrap_or(false);
    let spots = civ_spots(g);
    let mut cands: Vec<Cand> = Vec::new();
    if !exits_only {
        let mut i = 0;
        while i < g.guards.len() {
            let o = g.guards[i];
            i += 1;
            if tellable(g, o) {
                let p = g.ch[g.gd[o].ch].pos;
                let t = CivTarget { kind: CivTargetKind::Guard, guard: Some(o), spot: None, pos: p, name: g.gd[o].callsign.clone(), plan_pos: p };
                let adj = -g.k.civ.guard_bonus_m;
                consider(g, cv, &mut cands, t, CivBadTarget::Guard(o), adj, from, threat);
            }
        }
        if !g.cv[cv].phoned {
            for (i, s) in spots.phones.iter().enumerate() {
                let t = CivTarget { kind: CivTargetKind::Phone, guard: None, spot: Some(s.clone()), pos: s.pos, name: s.name.clone(), plan_pos: s.pos };
                consider(g, cv, &mut cands, t, CivBadTarget::Spot(i as u32), 0.0, from, threat);
            }
        }
    }
    let adj = if exits_only { 0.0 } else { g.k.civ.exit_malus_m };
    for (i, s) in spots.exits.iter().enumerate() {
        let t = CivTarget { kind: CivTargetKind::Exit, guard: None, spot: Some(s.clone()), pos: s.pos, name: s.name.clone(), plan_pos: s.pos };
        consider(g, cv, &mut cands, t, CivBadTarget::Spot((spots.phones.len() + i) as u32), adj, from, threat);
    }
    js::sort_by(&mut cands, |a, b| a.cost - b.cost);
    if cands.is_empty() { None } else { Some(cands.swap_remove(0).t) }
}

/// The run begins (or is re-chosen): a shout the first time.   (civilians.ts toFlee)
pub fn to_flee(g: &mut crate::game::Game, gd: GuardId, t: &crate::civilians::CivTarget, evac: bool) {
    let cv = civ_of(g, gd);
    let c = g.gd[gd].ch;
    g.cv[cv].target = Some(t.clone());
    g.cv[cv].retarget_at = g.time + 1.0;
    g.gd[gd].path = Vec::new(); g.gd[gd].path_goal = None; g.gd[gd].stall_ref = None;
    g.ch[c].anim.act = None;
    g.ch[c].anim.crouch_target = 0.0;
    crate::guards::go_to(g, gd, t.pos, Some(true));
    let again = g.cv[cv].state == CivState::Flee || g.cv[cv].state == CivState::Evacuate;
    set_state(g, gd, if evac { CivState::Evacuate } else { CivState::Flee });
    if again { return; }
    let lines = crate::dialogue::civ_lines();
    if !evac {
        g.stats.civilians.fled += 1;
        let pool = no_security(g, &lines.flee);
        let s = pick(g, gd, &pool);
        scream(g, gd, Some(&s));
    } else {
        let s = pick(g, gd, &lines.evac);
        civ_say(g, gd, &s, None, None);
    }
}

/// The running target's cost as flee compares two of them.
fn run_cost(g: &mut Game, from: V3, t: &CivTarget) -> f64 {
    let adj = match t.kind { CivTargetKind::Guard => -g.k.civ.guard_bonus_m, CivTargetKind::Exit => g.k.civ.exit_malus_m, CivTargetKind::Phone => 0.0 };
    path_metres(g, from, t.pos).m + adj
}

/// The run: keep the target honest, re-choose once a second, and what arriving means.   (civilians.ts flee)
pub fn flee(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> f64 {
    let cv = civ_of(g, gd);
    let c = g.gd[gd].ch;
    let evac = g.cv[cv].state == CivState::Evacuate;
    g.ch[c].anim.act = None;
    g.ch[c].anim.look_yaw_extra = 0.0;
    // `t` is a copy of C.target, the object it names in the TypeScript: a write to it goes to both
    let mut t = match g.cv[cv].target.clone() {
        Some(t) => t,
        None => match pick_target(g, gd, Some(PickTargetOpts { exits_only: Some(evac) })) {
            None => { to_cower(g, gd, false); return 0.0; }
            Some(nt) => { to_flee(g, gd, &nt, evac); nt }
        },
    };
    if t.kind == CivTargetKind::Guard {
        let o = t.guard.unwrap();
        if !tellable(g, o) {
            g.cv[cv].bad_targets.add(CivBadTarget::Guard(o));
            g.cv[cv].target = None;
            let line = format!("{} is no longer one to run to", g.gd[o].callsign);
            log(g, gd, &line);
            return g.gd[gd].speed;
        }
        let op = g.ch[g.gd[o].ch].pos;
        if v3::dist_xz(op, t.plan_pos) > 1.5 {
            t.pos = op;
            t.plan_pos = op;
            if let Some(x) = g.cv[cv].target.as_mut() { x.pos = op; x.plan_pos = op; }
            crate::guards::go_to(g, gd, t.pos, Some(true));
        }
        let op = g.ch[g.gd[o].ch].pos;
        let mine = g.ch[c].pos;
        let d = v3::dist_xz(mine, op);
        if d < g.k.civ.report_m && !crate::scene_collision::solid_blocked(g, v3::add(mine, [0.0, 1.3, 0.0]), v3::add(op, [0.0, 1.3, 0.0]), None) {
            to_report(g, gd, o);
            return 0.0;
        }
    }
    if g.time >= g.cv[cv].retarget_at && !evac && !g.cv[cv].pin_target {
        g.cv[cv].retarget_at = g.time + 1.0;
        if let Some(nt) = pick_target(g, gd, None) {
            if target_key(g, &nt) != target_key(g, &t) && (nt.kind == CivTargetKind::Guard || t.kind != CivTargetKind::Guard) {
                let mine = g.ch[c].pos;
                let cur = run_cost(g, mine, &t);
                let alt = run_cost(g, mine, &nt);
                if alt < cur - 3.0 {
                    let line = format!("{} is nearer now", nt.name);
                    log(g, gd, &line);
                    g.cv[cv].target = Some(nt.clone());
                    t = nt;
                    crate::guards::go_to(g, gd, t.pos, Some(true));
                }
            }
        }
    }
    crate::guards::go_to(g, gd, t.pos, None);
    let run = g.k.civ.run_speed;
    let arrived = crate::guards::follow_path(g, gd, dt, run);
    if arrived {
        g.gd[gd].stall_ref = None;
        if v3::dist_xz(g.ch[c].pos, t.pos) > 1.6 {
            let key = target_key(g, &t);
            g.cv[cv].bad_targets.add(key);
            g.cv[cv].target = None;
            let line = format!("no way to {}", t.name);
            log(g, gd, &line);
            return 0.0;
        }
        if t.kind == CivTargetKind::Phone { to_phone(g, gd, t.spot.as_ref().unwrap()); return 0.0; }
        if t.kind == CivTargetKind::Exit { to_gone(g, gd, evac); return 0.0; }
    } else if crate::guards::not_getting_anywhere(g, gd, dt, 2.5) {
        let key = target_key(g, &t);
        g.cv[cv].bad_targets.add(key);
        g.cv[cv].target = None;
        let line = format!("stuck on the way to {}", t.name);
        log(g, gd, &line);
    }
    g.gd[gd].speed
}

// ---------------------------------------------------------------- REPORT: telling the man
/// Is what she tells first-hand of him or his doing.   (civilians.ts firstHand)
pub fn first_hand(cause: Option<crate::civilians::CivCause>) -> bool {
    matches!(cause, Some(CivCause::Sam) | Some(CivCause::Gun) | Some(CivCause::Grabbed) | Some(CivCause::Body) | Some(CivCause::Gunfire) | Some(CivCause::Shot))
}

/// She has reached the man: she starts telling him, and he stops and takes it in.   (civilians.ts toReport)
pub fn to_report(g: &mut crate::game::Game, gd: GuardId, o: GuardId) {
    let cv = civ_of(g, gd);
    let c = g.gd[gd].ch;
    g.cv[cv].report_to = Some(o); g.cv[cv].talk_t = 0.0; g.cv[cv].acked = false;
    g.gd[gd].path = Vec::new(); g.gd[gd].path_goal = None;
    g.cv[cv].target = None;
    let cause = g.cv[cv].cause;
    let pool = report_pool(g, cause.map(|k| k.as_str()));
    let s = pick(g, gd, &pool);
    civ_say(g, gd, &s, Some(0.8), None);
    let at = g.cv[cv].cause_pos.unwrap_or(g.ch[c].pos);
    prime_guard(g, o, at, 0.5);
    g.ch[c].anim.act = Some(crate::character::AnimAct { clip: String::from("Idle_Talking_Loop"), loop_: Some(true), rate: Some(1.2), in_: Some(0.3), out: Some(0.4) });
    set_state(g, gd, CivState::Report);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn ident() -> Vec<f64> { vec![1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0] }
    #[test]
    fn in_view_keeps_the_frame_and_its_open_edges() {
        let vp = InViewVp::Array(ident());
        assert!(!in_view(None, [0.0; 3]));
        assert!(in_view(Some(&vp), [0.5, -0.5, 9.0]));
        assert!(!in_view(Some(&vp), [0.92, 0.0, 0.0]));   // (strict at the edge)
        assert!(!in_view(Some(&vp), [0.0, 0.9, 0.0]));
        let mut behind = ident();
        behind[15] = 0.0;
        assert!(!in_view(Some(&InViewVp::Array(behind)), [0.0; 3]));   // (w <= 0)
    }
    #[test]
    fn in_view_reads_singles_as_doubles_by_the_same_columns() {
        let single: Vec<f32> = ident().iter().map(|x| *x as f32).collect();
        assert!(in_view(Some(&InViewVp::Float32Array(single.clone())), [0.5, 0.5, 0.0]));
        assert!(!in_view(Some(&InViewVp::Float32Array(single)), [0.95, 0.0, 0.0]));
    }
    #[test]
    fn rank_orders_the_causes_as_the_table() {
        assert_eq!(rank(None), -1);
        assert!(rank(Some(CivCause::Noise)) < rank(Some(CivCause::Scream)));
        assert_eq!(rank(Some(CivCause::Scream)), rank(Some(CivCause::Panel)));
        assert_eq!(rank(Some(CivCause::Sam)), rank(Some(CivCause::Shot)));
        assert_eq!(rank(Some(CivCause::Bang)), rank(Some(CivCause::Gunfire)));
        assert!(rank(Some(CivCause::Body)) < rank(Some(CivCause::Gun)));
        assert_eq!(rank(Some(CivCause::Grabbed)), 6);
    }
    #[test]
    fn first_hand_is_him_or_his_doing_only() {
        assert!(first_hand(Some(CivCause::Shot)));
        assert!(!first_hand(Some(CivCause::Scream)));
        assert!(!first_hand(Some(CivCause::Noise)));
        assert!(!first_hand(Some(CivCause::Bang)));
        assert!(!first_hand(None));
    }
    #[test]
    fn only_chair_and_bin_take_time_to_rise_from() {
        use crate::scene_level::CivilianTask as T;
        assert!(task_clip(T::Stand).is_none());
        assert_eq!(task_clip(T::Sit).unwrap().rise, Some(1.0));
        assert_eq!(task_clip(T::Bin).unwrap().rise, Some(0.8));
        assert_eq!(task_clip(T::Wipe).unwrap().rise, None);
    }
}
