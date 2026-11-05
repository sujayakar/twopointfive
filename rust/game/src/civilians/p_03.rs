//! src/game/civilians.ts lines 875 to 1308
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};
use crate::character::{AnimAct, Bone, HoldPhase};
use crate::chronicle::{ChronCountKey, ChronKind, ChronicleDataValue, ChronicleNDataValue};
use crate::dialogue::{Band, DialoguePick, DialoguePickLineK, DialoguePickWidened, IntelKind, Knowledge, StageKey};
use crate::game::{DownBy, DownKind, GuardState, MissionStage, PlayerBubble, Temperament, WitnessCalledToBody, WitnessSawBody};
use crate::guards::{body_pos, escalation_of, follow_path, go_to, point_in_view, say};
use crate::scene_level::{CivilianDefKind, CivilianTask};
use crate::teams::Team;

// ---------------------------------------------------------------- seams: what skeleton 3 still cannot give

/// g.audio.play(name, pos, vol, { rate }): `opts` of game_audio_like_play holds `crate::Opaque`, which cannot carry a rate
fn seam_audio_play_rate(g: &mut Game, name: &str, at: V3, vol: f64, rate: f64) { todo!("needs: game_audio_like_play's opts to carry a rate") }

// ---------------------------------------------------------------- small helpers

/// `n.toFixed(1)`: the exact decimal value, a tie going to the larger magnitude
fn to_fixed1(x: f64) -> String {
    if x == 0.0 { return String::from("0.0"); }
    let a = m::abs(x);
    let q = a * 4.0;
    if q == m::floor(q) && js::rem(q, 2.0) == 1.0 {
        let n = m::floor(a * 10.0) + 1.0;
        return format!("{}{:.1}", if x < 0.0 { "-" } else { "" }, n / 10.0);
    }
    format!("{:.1}", x)
}
/// `n.toFixed(2)`, the same way: an exact tie at two places is an odd number of eighths
fn to_fixed2(x: f64) -> String {
    if x == 0.0 { return String::from("0.00"); }
    let a = m::abs(x);
    let q = a * 8.0;
    if q == m::floor(q) && js::rem(q, 2.0) == 1.0 {
        let n = m::floor(a * 100.0) + 1.0;
        return format!("{}{:.2}", if x < 0.0 { "-" } else { "" }, n / 100.0);
    }
    format!("{:.2}", x)
}
/// the row behind firearm.def, as plainShort reads it ({ id, short })
fn weapon_short_def(g: &Game, fa: FirearmId) -> crate::gear::PlainShortDef {
    let row = g.m.weapons.weapons.get(g.fa[fa].def).unwrap();
    crate::gear::PlainShortDef { id: String::from(row.id.as_str()), short: row.short.clone() }
}
/// a cause as a template string prints it
fn cause_text(c: Option<CivCause>) -> &'static str { match c { Some(c) => c.as_str(), None => "null" } }
/// a bone's place, or the offset from the body's feet the TypeScript falls back to
fn bone_or(g: &Game, c: CharId, b: Bone, up: f64) -> V3 {
    match g.ch[c].bones.get(b) { Some(p) => *p, None => v3::add(g.ch[c].pos, [0.0, up, 0.0]) }
}
/// an empty bubble test: he has a line up for less than `secs` since it began
fn bubble_within(g: &Game, gd: GuardId, secs: f64) -> bool {
    match &g.gd[gd].bubble { Some(b) => g.time - b.t < secs, None => false }
}
/// a row of a chronicle's data, in the order the TypeScript's literal has it
enum Dv { T(String), B(bool) }
fn data_c(rows: Vec<(&str, Dv)>) -> JsMap<String, Option<ChronicleDataValue>> {
    let mut d = JsMap::new();
    for (k, v) in rows { d.set(String::from(k), Some(match v { Dv::T(s) => ChronicleDataValue::Text(s), Dv::B(b) => ChronicleDataValue::Bool(b) })); }
    d
}
fn data_n(rows: Vec<(&str, Dv)>) -> JsMap<String, Option<ChronicleNDataValue>> {
    let mut d = JsMap::new();
    for (k, v) in rows { d.set(String::from(k), Some(match v { Dv::T(s) => ChronicleNDataValue::Text(s), Dv::B(b) => ChronicleNDataValue::Bool(b) })); }
    d
}
/// the debrief's row for a report: on the card only if she saw it herself
fn chronicle_reported(g: &mut Game, cause: Option<CivCause>, rows: Vec<(&str, Dv)>, pos: V3) {
    if first_hand(cause) { crate::chronicle::chronicle_n(g, ChronKind::CivilianReported, ChronCountKey::CivReported, Some(&data_n(rows)), Some(pos)); }
    else { crate::chronicle::chronicle(g, ChronKind::CivilianReported, Some(&data_c(rows)), Some(pos)); }
}
/// `text.replace(/pat/i, with)`, `pat` in lower case
fn replace_ci(text: &str, pat: &str, with: &str) -> String {
    match text.to_ascii_lowercase().find(pat) {
        Some(at) => format!("{}{}{}", &text[..at], with, &text[at + pat.len()..]),
        None => String::from(text),
    }
}

/// a man about to be told about a spot: suspicious toward it, rooted for `beat` seconds (civilians.ts primeGuard)
pub fn prime_guard(g: &mut crate::game::Game, o: GuardId, pos: V3, beat: f64) {
    if g.gd[o].state == GuardState::Dead || g.gd[o].held.is_some() || g.gd[o].state == GuardState::Alert { return; }
    let man = &mut g.gd[o];
    man.awareness = m::max(man.awareness, 0.5); man.last_known = Some(pos);
    if man.state == GuardState::Patrol { man.state = GuardState::Suspicious; man.search_t = 0.0; man.body_duty = None; }
    man.react_t = m::max(man.react_t, beat); man.path = Vec::new(); man.path_goal = None; man.stall_ref = None;
}

/// she tells him, face to face (civilians.ts report)
pub fn report(g: &mut crate::game::Game, gd: GuardId, dt: f64) {
    let cid = g.gd[gd].civ.unwrap(); let c = g.gd[gd].ch; let o = g.cv[cid].report_to;
    g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 10.0, dt); g.ch[c].vel = [0.0, 0.0, 0.0];
    let lost = match o { None => true, Some(o) => !tellable(g, o) || v3::dist_xz(g.ch[g.gd[o].ch].pos, g.ch[c].pos) > 6.0 };
    if lost {
        g.ch[c].anim.act = None; g.cv[cid].report_to = None;
        if let Some(o) = o { g.cv[cid].bad_targets.add(CivBadTarget::Guard(o)); }
        let t = if g.cv[cid].reported { None } else { pick_target(g, gd, None) };
        match t { Some(t) => to_flee(g, gd, &t, false), None => to_cower(g, gd, false) }
        return;
    }
    let o = o.unwrap();
    let (op, cp) = (g.ch[g.gd[o].ch].pos, g.ch[c].pos);
    let yaw_to = m::atan2(op[0] - cp[0], op[2] - cp[2]);
    g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, yaw_to, 5.0 * dt);
    if g.gd[o].state != GuardState::Alert { g.gd[o].react_t = m::max(g.gd[o].react_t, 0.5); }
    g.cv[cid].talk_t += dt;
    if g.cv[cid].talk_t >= g.k.civ.report_secs && !g.cv[cid].acked { g.cv[cid].acked = true; deliver_report(g, gd, o, DeliverReportHow::Told); }
    if g.cv[cid].talk_t >= g.k.civ.report_secs + 1.4 { g.ch[c].anim.act = None; to_cower(g, gd, false); }
}

/// what telling a man does; once per fright (civilians.ts deliverReport)
pub fn deliver_report(g: &mut crate::game::Game, gd: GuardId, o: GuardId, how: crate::civilians::DeliverReportHow) {
    let cid = g.gd[gd].civ.unwrap(); if g.cv[cid].reported { return; }
    g.cv[cid].reported = true; g.cv[cid].report_to = Some(o); g.stats.civilians.reports += 1;
    let cause = g.cv[cid].cause;
    let at = g.ch[g.gd[gd].ch].pos;
    let pos = g.cv[cid].cause_pos.unwrap_or(at); let armed = g.cv[cid].armed || cause == Some(CivCause::Body);
    {
        let man = &mut g.gd[o];
        man.awareness = m::max(man.awareness, if armed { 0.8 } else { 0.55 }); man.last_known = Some(pos); man.body_duty = None; man.task = None; man.path = Vec::new(); man.path_goal = None; man.stall_ref = None;
        if armed { man.state = GuardState::Search; man.search_t = 0.0; man.search_plan = None; man.react_t = 0.6; }
        else { if man.state == GuardState::Patrol || man.state == GuardState::Suspicious { man.state = GuardState::Suspicious; man.search_t = 0.0; } man.react_t = 0.9; }
    }
    let now = g.time; crate::game::guard_track_fix(g, o, now);
    let lines = crate::dialogue::civ_lines();
    let pool: Vec<String> = if armed { lines.guard_ack.armed.clone() } else {
        lines.guard_ack.someone.iter().map(|t| if t.contains("behind the desk") { lines.now.guard_ack.clone() } else { t.clone() }).collect()
    };
    let t = pick(g, gd, &pool); let t = fill(g, gd, &t, Some(o));
    say(g, o, &t, None, None, None);
    let radio_key = match cause {
        Some(CivCause::Grabbed) => "grabbed", Some(CivCause::Body) => "body",
        Some(CivCause::Gunfire) | Some(CivCause::Bang) => "gunfire",
        _ => if armed { "armed" } else { "someone" },
    };
    later(g, 1.6, &LaterP { kind: PendKind::ReportRadio, cv: gd, o: Some(o), key: Some(String::from(radio_key)), armed: Some(armed), pos: Some(pos), ..LaterP::zeroed() });
    let rows = vec![
        ("who", Dv::T(g.cv[cid].name.clone())), ("to", Dv::T(g.gd[o].callsign.clone())),
        ("cause", Dv::T(String::from(cause.map_or("sam", |c| c.as_str())))), ("armed", Dv::B(armed)), ("how", Dv::T(String::from(how.as_str()))),
    ];
    let here = g.ch[g.gd[gd].ch].pos;
    chronicle_reported(g, cause, rows, here);
    let text = format!("told {} ({}): {}{} at ({}, {}) → he {}", g.gd[o].callsign, how.as_str(), cause_text(cause), if armed { ", armed" } else { "" }, to_fixed1(pos[0]), to_fixed1(pos[2]), if armed { "searches it" } else { "walks it" });
    log(g, gd, &text);
}

/// she takes the handset off the hook (civilians.ts toPhone)
pub fn to_phone(g: &mut crate::game::Game, gd: GuardId, s: &crate::civilians::CivSpot) {
    let cid = g.gd[gd].civ.unwrap(); let c = g.gd[gd].ch;
    g.cv[cid].talk_t = 0.0; g.cv[cid].target = None; g.gd[gd].path = Vec::new(); g.gd[gd].path_goal = None;
    let lines = crate::dialogue::civ_lines();
    let armed = g.cv[cid].armed;
    let pool: Vec<String> = if away(g) { vec![if armed { lines.own.phone_armed.clone() } else { lines.own.phone.clone() }] }
        else if armed { lines.phone_armed[..1].to_vec() } else { lines.phone[..1].to_vec() };
    let t = pick(g, gd, &pool);
    civ_say(g, gd, &t, Some(1.0), None);
    g.ch[c].anim.act = Some(AnimAct { clip: String::from("Idle_Talking_Loop"), loop_: Some(true), rate: Some(1.3), in_: Some(0.3), out: Some(0.4) });
    let at = v3::add(g.ch[c].pos, [0.0, 1.2, 0.0]);
    seam_audio_play_rate(g, "click", at, 0.5, 0.8);
    log(g, gd, &format!("on {}", s.name));
    set_state(g, gd, CivState::Phone);
}

/// on the line (civilians.ts phone)
pub fn phone(g: &mut crate::game::Game, gd: GuardId, dt: f64) {
    let cid = g.gd[gd].civ.unwrap(); let c = g.gd[gd].ch;
    let here = g.ch[c].pos;
    let s = civ_spots(g).phones.into_iter().find(|p| v3::dist_xz(p.pos, here) < 2.0);
    g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 10.0, dt); g.ch[c].vel = [0.0, 0.0, 0.0];
    if let Some(s) = s { g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, s.face, 4.0 * dt); }
    let before = g.cv[cid].talk_t; g.cv[cid].talk_t += dt;
    if before < 3.0 && g.cv[cid].talk_t >= 3.0 {
        let lines = crate::dialogue::civ_lines();
        let pool = if g.cv[cid].armed { lines.phone_armed[1..].to_vec() } else { lines.phone[1..].to_vec() };
        let t = pick(g, gd, &pool);
        civ_say(g, gd, &t, Some(1.0), None);
    }
    if before < g.k.civ.phone_secs && g.cv[cid].talk_t >= g.k.civ.phone_secs { phone_call(g, gd); }
    if g.cv[cid].talk_t >= g.k.civ.phone_secs + 1.5 { g.ch[c].anim.act = None; to_cower(g, gd, false); }
}

/// the call lands: control puts it out, the nearest calm man is sent; once per fright (civilians.ts phoneCall)
pub fn phone_call(g: &mut crate::game::Game, gd: GuardId) {
    let cid = g.gd[gd].civ.unwrap(); if g.cv[cid].phoned { return; }
    g.cv[cid].phoned = true; g.cv[cid].reported = true; g.stats.civilians.phoned += 1;
    let cause = g.cv[cid].cause;
    let here = g.ch[g.gd[gd].ch].pos;
    let pos = g.cv[cid].cause_pos.unwrap_or(here); let armed = g.cv[cid].armed || cause == Some(CivCause::Body);
    {
        let k = if cause == Some(CivCause::Body) { "body" } else if armed { "armed" } else { "someone" };
        let key = String::from(k);
        let lines = crate::dialogue::civ_lines();
        let own = if away(g) { lines.guide.own.control.get(key.clone()).or(lines.own.control.get(key.clone())) } else { None };
        let line = own.or(lines.guide.control.get(key.clone())).or(lines.control.get(key)).cloned().unwrap_or_default();
        control(g, gd, &line);
    }
    let sent = if g.quiet_utility { None } else { crate::devices::send_to_look(g, pos, "civilian") };
    if armed && !g.quiet_utility {
        g.alarm.pos[0] = pos[0]; g.alarm.pos[1] = 0.0; g.alarm.pos[2] = pos[2]; g.alarm.placed = true;
        later(g, 1.4, &LaterP { kind: PendKind::Escalate, cv: gd, o: sent, ..LaterP::zeroed() });
    }
    let sent_name = match sent { Some(s) => g.gd[s].callsign.clone(), None => String::new() };
    let rows = vec![
        ("who", Dv::T(g.cv[cid].name.clone())), ("to", Dv::T(String::from("control"))),
        ("cause", Dv::T(String::from(cause.map_or("sam", |c| c.as_str())))), ("armed", Dv::B(armed)),
        ("how", Dv::T(String::from("phone"))), ("sent", Dv::T(sent_name.clone())),
    ];
    let at = g.ch[g.gd[gd].ch].pos;
    chronicle_reported(g, cause, rows, at);
    let tail = if sent.is_some() { format!(" — {} sent", sent_name) } else { String::from(" — nobody free to send") };
    let text = format!("rang it in: {}{} at ({}, {}){}", cause_text(cause), if armed { ", armed" } else { "" }, to_fixed1(pos[0]), to_fixed1(pos[2]), tail);
    log(g, gd, &text);
}

/// out of the building: gone; unless evacuated, control rings the floor a little later (civilians.ts toGone)
pub fn to_gone(g: &mut crate::game::Game, gd: GuardId, evac: bool) {
    let cid = g.gd[gd].civ.unwrap(); let c = g.gd[gd].ch;
    g.ch[c].anim.act = None; g.gd[gd].path = Vec::new(); g.gd[gd].path_goal = None; g.cv[cid].target = None;
    if evac { g.stats.civilians.evacuated += 1; log(g, gd, "off the floor (evacuated)"); }
    else if crate::snatch::snatch_is_man(g, gd) { log(g, gd, "off the floor — gone"); }
    else if !g.cv[cid].reported {
        g.cv[cid].reported = true; g.stats.civilians.outside_calls += 1;
        log(g, gd, "out of the building — control will ring the floor");
        let secs = g.k.civ.outside_call_secs;
        later(g, secs, &LaterP { kind: PendKind::OutsideCall, cv: gd, o: None, ..LaterP::zeroed() });
    }
    forget_meter(g, gd);
    set_state(g, gd, CivState::Gone);
}

/// the free nav cell near her with the most wall about it, away from the fright; her own spot if nothing better (civilians.ts cornerSpot)
pub fn corner_spot(g: &mut crate::game::Game, gd: GuardId) -> V3 {
    use crate::scene_collision::{nav_grid_idx, nav_grid_walkable};
    let cid = g.gd[gd].civ.unwrap();
    let cs = g.col.nav.cell; let from = g.ch[g.gd[gd].ch].pos; let threat = g.cv[cid].cause_pos;
    let corner_m = g.k.civ.corner_m;
    let r = m::ceil(corner_m / cs); let cx = m::floor(from[0] / cs); let cz = m::floor(from[2] / cs);
    let mut best: V3 = from; let mut bs = f64::NEG_INFINITY;
    let mut dz = -r;
    while dz <= r {
        let mut dx = -r;
        while dx <= r {
            let (x, z) = (cx + dx, cz + dz);
            dx += 1.0;
            if x < 1.0 || z < 1.0 || x >= g.col.nav.w - 1.0 || z >= g.col.nav.h - 1.0 { continue; }
            let at = nav_grid_idx(g, x, z) as usize;
            if g.col.nav.blocked[at] != 0 { continue; }
            let q: V3 = [(x + 0.5) * cs, 0.0, (z + 0.5) * cs]; let d = v3::dist_xz(from, q); if d > corner_m { continue; }
            let (e, w_, n, s) = (nav_grid_idx(g, x + 1.0, z) as usize, nav_grid_idx(g, x - 1.0, z) as usize, nav_grid_idx(g, x, z + 1.0) as usize, nav_grid_idx(g, x, z - 1.0) as usize);
            let blocked = &g.col.nav.blocked;
            let walls = blocked[e] as f64 + blocked[w_] as f64 + blocked[n] as f64 + blocked[s] as f64;
            if walls == 0.0 { continue; }
            let mut sc = walls * 3.0 - d * 0.6;
            if let Some(t) = threat { sc += m::min(6.0, v3::dist_xz(t, q)) * 0.5; }
            if sc <= bs { continue; }
            if !nav_grid_walkable(g, from, q) { continue; }
            bs = sc; best = q;
        }
        dz += 1.0;
    }
    best
}

/// into the corner, or (told) down where she stands (civilians.ts toCower)
pub fn to_cower(g: &mut crate::game::Game, gd: GuardId, complied: bool) {
    let cid = g.gd[gd].civ.unwrap(); let c = g.gd[gd].ch;
    g.cv[cid].complied = complied; if complied { g.cv[cid].comply_until = g.time + g.k.civ.comply_min * 60.0; }
    let spot = if complied { g.ch[c].pos } else { corner_spot(g, gd) };
    let now = g.time;
    let roll = g.cv[cid].dice.roll();
    let cv = &mut g.cv[cid];
    cv.cower_at = Some(spot); cv.found_t = 0.0; cv.found_by = None; cv.found_line_at = -1.0; cv.found_told = false; cv.whimper_at = now + 2.0 + roll * 4.0;
    g.gd[gd].path = Vec::new(); g.gd[gd].path_goal = None; g.ch[c].anim.act = None; g.cv[cid].target = None;
    g.stats.civilians.cowered += 1; if complied { g.stats.civilians.complied += 1; }
    set_state(g, gd, CivState::Cower);
}

/// a rescue's hostage on her feet: she follows `lead`, stopping `gap` behind, or (none) goes down where she stands (civilians.ts hostageLead)
pub fn hostage_lead(g: &mut crate::game::Game, gd: GuardId, lead: Option<CharId>, gap: f64) {
    let Some(cid) = g.gd[gd].civ else { return; };
    if g.gd[gd].state == GuardState::Dead || g.cv[cid].state != CivState::Cower { return; }
    g.cv[cid].lead = lead; g.cv[cid].lead_gap = gap; g.cv[cid].lead_to = None;
    g.gd[gd].path = Vec::new(); g.gd[gd].path_goal = None; g.gd[gd].stall_ref = None;
    if lead.is_none() { g.cv[cid].cower_at = Some(g.ch[g.gd[gd].ch].pos); }
}

/// a led civilian is sent the last steps to `to`, or (none) follows her leader again (civilians.ts civilianLedTo)
pub fn civilian_led_to(g: &mut crate::game::Game, gd: GuardId, to: Option<V3>) {
    let Some(cid) = g.gd[gd].civ else { return; };
    if g.cv[cid].lead.is_none() { return; }
    g.cv[cid].lead_to = to; g.gd[gd].path = Vec::new(); g.gd[gd].path_goal = None; g.gd[gd].stall_ref = None;
}

/// taken off the floor by the team: gone, nobody told (civilians.ts civilianOff)
pub fn civilian_off(g: &mut crate::game::Game, gd: GuardId) {
    let Some(cid) = g.gd[gd].civ else { return; };
    if g.gd[gd].state == GuardState::Dead || g.cv[cid].state == CivState::Gone { return; }
    let c = g.gd[gd].ch;
    g.cv[cid].lead = None; g.cv[cid].lead_to = None; g.ch[c].anim.act = None; g.gd[gd].path = Vec::new(); g.gd[gd].path_goal = None; g.cv[cid].target = None;
    log(g, gd, "off the floor — in the team's hands"); forget_meter(g, gd); set_state(g, gd, CivState::Gone);
}

/// the half-second tick her path search is asked again on
fn half_second_tick(g: &Game, dt: f64) -> bool { m::floor(g.time * 2.0) != m::floor((g.time - dt) * 2.0) }

/// the walk behind him (civilians.ts follow)
pub fn follow(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> crate::civilians::Follow {
    let cid = g.gd[gd].civ.unwrap(); let c = g.gd[gd].ch; let l = g.cv[cid].lead.unwrap(); let mut move_ = 0.0;
    g.ch[c].anim.act = None; g.ch[c].anim.look_yaw_extra = 0.0;
    if let Some(to) = g.cv[cid].lead_to {
        if v3::dist_xz(to, g.ch[c].pos) > 0.3 {
            if !g.gd[gd].path.is_empty() || half_second_tick(g, dt) { go_to(g, gd, to, None); }
            follow_path(g, gd, dt, 1.6); move_ = g.gd[gd].speed;
        } else {
            g.gd[gd].path = Vec::new(); g.gd[gd].path_goal = None; g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 10.0, dt); g.ch[c].vel = [0.0, 0.0, 0.0];
        }
        return Follow { move_, crouch: 0.0 };
    }
    let lp = g.ch[l].pos;
    let d = v3::dist_xz(lp, g.ch[c].pos); let going = !g.gd[gd].path.is_empty();
    let gap = g.cv[cid].lead_gap;
    if d > gap + (if going { 0.0 } else { 0.6 }) {
        if !g.gd[gd].path.is_empty() || half_second_tick(g, dt) { go_to(g, gd, lp, None); }
        let (run, walk) = (g.k.civ.run_speed, g.k.civ.walk_speed);
        let speed = if d > gap + 2.2 { run } else { m::max(walk, m::min(run, 1.4 + (d - gap))) };
        follow_path(g, gd, dt, speed);
        move_ = g.gd[gd].speed;
    } else {
        g.gd[gd].path = Vec::new(); g.gd[gd].path_goal = None; g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 10.0, dt); g.ch[c].vel = [0.0, 0.0, 0.0];
        let p = g.ch[c].pos;
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, m::atan2(lp[0] - p[0], lp[2] - p[2]), 3.0 * dt);
    }
    Follow { move_, crouch: 0.0 }
}

/// the corner: the scuttle, the whimper, and every way out of it (civilians.ts cower)
pub fn cower(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> crate::civilians::Cower {
    let cid = g.gd[gd].civ.unwrap(); let c = g.gd[gd].ch; let pl_c = g.pl[g.player].ch; let mut move_ = 0.0;
    let spot = g.cv[cid].cower_at.unwrap_or(g.ch[c].pos); let d = v3::dist_xz(spot, g.ch[c].pos);
    if d > 0.5 && g.time - g.cv[cid].state_at < 6.0 {
        go_to(g, gd, spot, None); let creep = g.k.civ.creep_speed; follow_path(g, gd, dt, creep); move_ = g.gd[gd].speed;
    } else {
        g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 10.0, dt); g.ch[c].vel = [0.0, 0.0, 0.0];
        let p = g.ch[c].pos;
        let away_yaw = match g.cv[cid].cause_pos { Some(t) => m::atan2(p[0] - t[0], p[2] - t[2]), None => g.ch[c].body_yaw };
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, away_yaw, 2.0 * dt);
        g.ch[c].anim.look_yaw_extra = m::sin(g.time * 1.7 + g.ch[c].id) * 0.5;
    }
    if !crate::rescue::rescue_is_hostage(g, gd) && g.time >= g.cv[cid].whimper_at && g.time - g.cv[cid].state_at < 70.0 {
        let roll = g.cv[cid].dice.roll();
        g.cv[cid].whimper_at = g.time + 7.0 + roll * 7.0;
        let up = match &g.gd[gd].bubble { Some(b) => g.time - b.t < b.dur, None => false };
        if !up {
            let pool = crate::dialogue::civ_lines().cower;
            let t = pick(g, gd, &pool);
            civ_say(g, gd, &t, None, None);
        }
    }
    let complying = g.cv[cid].complied && g.time < g.cv[cid].comply_until;
    if !complying && !g.cv[cid].refled && g.cv[cid].saw_gun && g.time - g.cv[cid].seen_at < 0.3 && v3::dist_xz(g.ch[pl_c].pos, g.ch[c].pos) < g.k.civ.reflee_m && g.time - g.cv[cid].state_at > 2.0 {
        let at = g.ch[pl_c].pos;
        to_alarmed(g, gd, CivCause::Gun, at, true);
        return Cower { move_: 0.0, crouch: 1.0 };
    }
    if !complying && !g.cv[cid].reported && g.time - g.cv[cid].state_at > g.k.civ.lone_secs && g.time - g.cv[cid].retarget_at > g.k.civ.lone_secs {
        g.cv[cid].bad_targets.clear(); g.cv[cid].retarget_at = g.time;
        if let Some(t) = pick_target(g, gd, None) {
            log(g, gd, "nobody came — going to find someone");
            to_flee(g, gd, &t, false);
            return Cower { move_: 0.0, crouch: 1.0 };
        }
    }
    if !complying && g.cv[cid].reported && g.time - g.cv[cid].state_at > g.k.civ.settle_secs && escalation_of(g) == 0.0 && !g.alarm.episode && g.gd[gd].awareness < 0.1 {
        to_work(g, gd, true);
        return Cower { move_: 0.0, crouch: 0.0 };
    }
    if g.cv[cid].complied && g.time >= g.cv[cid].comply_until && g.time - g.cv[cid].state_at > g.k.civ.settle_secs && escalation_of(g) == 0.0 && !g.alarm.episode && g.gd[gd].awareness < 0.1 {
        if !g.cv[cid].reported {
            if let Some(t) = pick_target(g, gd, None) { to_flee(g, gd, &t, false); return Cower { move_: 0.0, crouch: 1.0 }; }
        }
        to_work(g, gd, true);
        return Cower { move_: 0.0, crouch: 0.0 };
    }
    Cower { move_, crouch: 1.0 }
}

/// the frame the arm goes round her: whatever she was doing stops (civilians.ts civilianGrabbed)
pub fn civilian_grabbed(g: &mut crate::game::Game, gd: GuardId) {
    let cid = g.gd[gd].civ.unwrap(); let c = g.gd[gd].ch;
    end_task(g, gd);
    g.cv[cid].rise_t = 0.0; g.ch[c].anim.act = None; g.ch[c].anim.crouch_target = 0.0;
    g.cv[cid].target = None; g.cv[cid].report_to = None; g.cv[cid].look_at = None;
    let at = g.ch[g.pl[g.player].ch].pos;
    let now = g.time;
    let cv = &mut g.cv[cid];
    cv.cause = Some(CivCause::Grabbed); cv.cause_pos = Some(at); cv.armed = true; cv.cause_at = now; cv.complied = false;
    log(g, gd, "grabbed");
}

/// the fields every answer of hers shares (the TypeScript's `base`)
fn answer_base(cell: &str, text: String, id: &str, stage: StageKey, stage_n: f64, kind: Option<IntelKind>) -> DialoguePick {
    DialoguePick {
        text, id: String::from(id), kind, stage, stage_n, cell: String::from(cell),
        k: Knowledge::Saw, band: Band::High, t: Temperament::Nervy, line_k: DialoguePickLineK::Empty, widened: DialoguePickWidened::Empty,
        after: None, script: None, shape: None,
    }
}

/// her answers in the arm: fear, then the thing or two she knows, then nothing (civilians.ts civilianAnswer)
pub fn civilian_answer(g: &mut crate::game::Game, gd: GuardId, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) -> Option<crate::dialogue::DialoguePick> {
    let cid = g.gd[gd].civ.unwrap(); let t_id = g.gd[gd].talk;
    g.tab.game_talk[t_id].stage += 1.0; g.tab.game_talk[t_id].presses += 1.0; g.tab.game_talk[t_id].last_t = g.time;
    let stage = g.tab.game_talk[t_id].stage;
    let def_kind = g.cv[cid].def.kind;
    let worker = def_kind == CivilianDefKind::Worker;
    let cell = format!("civ:{}", def_kind.as_str());
    if crate::snatch::snatch_is_man(g, gd) {
        let s = crate::dialogue::snatch_lines_table();
        return Some(if stage <= 1.0 {
            let text = pick(g, gd, &s.held);
            answer_base(&cell, text, "civ:fear", StageKey::Num(1.0), stage, None)
        } else {
            let text = pick(g, gd, &s.dry);
            answer_base(&cell, text, "civ:dry", StageKey::X, stage, None)
        });
    }
    let lines = crate::dialogue::civ_lines();
    if stage <= 1.0 {
        let pool: Vec<String> = if worker { lines.held_fear_worker.clone() }
            else if away(g) { lines.held_fear.iter().map(|t| if t.contains("CLEAN here") { lines.own.held_fear.clone() } else { t.clone() }).collect() }
            else { lines.held_fear.clone() };
        let text = pick(g, gd, &pool);
        return Some(answer_base(&cell, text, "civ:fear", StageKey::Num(1.0), stage, None));
    }
    let own = away(g);
    let mut men_n = 0usize;
    if own {
        for i in 0..g.guards.len() { let x = g.guards[i]; if g.gd[x].unit.is_none() && g.gd[x].team == Team::Guard { men_n += 1; } }
    }
    let here = g.ch[g.gd[gd].ch].pos;
    let intel_row: Option<Vec<String>> = lines.intel.get(String::from(def_kind.as_str())).cloned();
    let intel_len = intel_row.as_ref().map_or(0, |r| r.len());
    let pool: Vec<String> = if own {
        let mut p = Vec::new();
        if crate::devices::devices_pad_near(g, here).is_some() { p.push(lines.own.intel_pad.clone()); }
        if men_n != 0 { p.extend(lines.guide.own.intel.iter().cloned()); }
        p
    } else { intel_row.unwrap_or_default() };
    if g.cv[cid].intel_n < pool.len() {
        let mut text = pool[g.cv[cid].intel_n].clone(); g.cv[cid].intel_n += 1;
        if own {
            let count = crate::dialogue::civ_count();
            text = text.replacen("{n}", &count[core::cmp::min(men_n, count.len() - 1)], 1);
            if g.cv[cid].intel_n >= pool.len() { g.cv[cid].intel_n = core::cmp::max(g.cv[cid].intel_n, intel_len); }
        }
        let mut kind: Option<IntelKind> = None;
        if own && text.contains("{code}") {
            let pad = crate::devices::devices_pad_near(g, here).unwrap();
            text = text.replacen("{code}", &spell_words(&pad.def.code), 1); kind = Some(IntelKind::Code);
        }
        if text.to_ascii_lowercase().contains("seven three nine one") {
            if let Some(pad) = crate::devices::devices_pad_near(g, here) {
                text = replace_ci(&text, "seven three nine one", &spell_words(&pad.def.code)); kind = Some(IntelKind::Code);
            }
        }
        let id = format!("civ:intel:{}", g.cv[cid].intel_n);
        return Some(answer_base(&cell, text, &id, StageKey::Num(if stage == 2.0 { 2.0 } else { 3.0 }), stage, kind));
    }
    if own { g.cv[cid].intel_n = core::cmp::max(g.cv[cid].intel_n, intel_len); }
    let text = pick(g, gd, &lines.dry);
    Some(answer_base(&cell, text, "civ:dry", StageKey::X, stage, None))
}

/// a keypad's digits in words, capitalised (civilians.ts spellWords)
pub fn spell_words(code: &str) -> String {
    const DIGIT_WORDS: [&str; 10] = ["oh", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut w_ = String::new();
    for (i, d) in code.chars().enumerate() {
        if i > 0 { w_.push(' '); }
        // Number(d): a digit is itself, white space is 0, anything else is NaN and stays the character
        match d.to_digit(10) {
            Some(n) if d.is_ascii_digit() => w_.push_str(DIGIT_WORDS[n as usize]),
            _ if d.is_whitespace() => w_.push_str(DIGIT_WORDS[0]),
            _ => w_.push(d),
        }
    }
    let mut cs = w_.chars();
    match cs.next() { Some(f) => { let mut out: String = f.to_uppercase().collect(); out.push_str(cs.as_str()); out } None => w_ }
}

/// Sam's side of a question to her (civilians.ts civilianSamLine)
pub fn civilian_sam_line(g: &mut crate::game::Game, gd: GuardId) -> crate::civilians::CivilianSamLine {
    let lines = crate::dialogue::civ_lines();
    let pool: Vec<String> = if away(g) { lines.sam_ask.iter().map(|t| if t.contains("on this floor") { lines.own.sam_ask.clone() } else { t.clone() }).collect() } else { lines.sam_ask.clone() };
    let text = pick(g, gd, &pool);
    CivilianSamLine { text, id: String::from("sam:civ") }
}

/// 'stay down, stay quiet': she is let go where she stands, told (civilians.ts civilianComply)
pub fn civilian_comply(g: &mut crate::game::Game) -> bool {
    let pl = g.player; let pc = g.pl[pl].ch;
    let gd = match &g.pl[pl].holding { Some(h) if g.gd[h.g].civ.is_some() && h.phase == HoldPhase::Held => h.g, _ => return false };
    let cid = g.gd[gd].civ.unwrap(); let c = g.gd[gd].ch;
    let pool = crate::dialogue::civ_lines().comply.sam;
    let line = pick(g, gd, &pool);
    let units = line.encode_utf16().count() as f64;
    g.pl[pl].bubble = Some(PlayerBubble { text: line.clone(), t: g.time, dur: v3::clamp(0.7 + 0.018 * units, 0.9, 1.4) + 0.4, radio: false });
    let at = v3::add(g.ch[pc].pos, [0.0, 1.6, 0.0]);
    crate::audio_iface::game_audio_like_play(g, "murmur", Some(at), Some(0.2), None);
    crate::game::game_note(g, &format!("you ▸ {}", line));
    g.cv[cid].complied = true; g.cv[cid].comply_until = g.time + g.k.civ.comply_min * 60.0;
    let t_id = g.gd[gd].talk;
    g.gd[gd].held = None; g.ch[c].anim.held_pose = None; g.tab.game_talk[t_id].held_since = -1.0; g.gd[gd].speed = 0.0; g.ch[c].vel = [0.0, 0.0, 0.0];
    g.pl[pl].holding = None; g.ch[pc].anim.hold_pose = None; g.pl[pl].fire_cd = m::max(g.pl[pl].fire_cd, 0.4);
    later(g, 0.9, &LaterP { kind: PendKind::ComplyLine, cv: gd, o: None, ..LaterP::zeroed() });
    to_cower(g, gd, true);
    let pos = g.ch[c].pos;
    let d = data_n(vec![("who", Dv::T(g.cv[cid].name.clone()))]);
    crate::chronicle::chronicle_n(g, ChronKind::CivilianComplied, ChronCountKey::CivComplied, Some(&d), Some(pos));
    let text = format!("told to stay down — complies for {} min", g.k.civ.comply_min);
    log(g, gd, &text);
    let text = format!("{} is down and quiet. {}'ll stay put for a while and won't say anything unless they find {}", g.cv[cid].name, g.cv[cid].she, g.cv[cid].her);
    crate::game::game_msg(g, &text);
    true
}

/// how she went, by whose hand; her state is 'dead' either way (civilians.ts civilianDown)
pub fn civilian_down(g: &mut crate::game::Game, gd: GuardId, dir: V3, quiet: bool, how: crate::game::DownKind, by: crate::game::DownBy, line: Option<&str>) {
    let cid = g.gd[gd].civ.unwrap(); let c = g.gd[gd].ch;
    if g.gd[gd].state == GuardState::Dead { return; }
    if bubble_within(g, gd, 0.8) {
        let text = format!("{}: {}", g.cv[cid].name, g.gd[gd].bubble.as_ref().unwrap().text);
        crate::game::game_msg(g, &text);
    }
    let now = g.time;
    {
        let man = &mut g.gd[gd];
        man.state = GuardState::Dead; man.speed = 0.0; man.died_at = now; man.down_kind = Some(how); man.down_by = Some(by); man.script = None;
    }
    if g.gd[gd].held.is_some() { g.gd[gd].held = None; g.ch[c].anim.held_pose = None; }
    g.ch[c].anim.act = None; g.cv[cid].target = None; g.cv[cid].report_to = None;
    let world = crate::ragdoll::RagdollWorld { boxes: g.col.boxes.ids().collect(), dynamic_boxes: g.col.dynamic_boxes.clone() };
    crate::character::character_ragdollize(g, c, Some(&world), dir, quiet);
    forget_meter(g, gd);
    let dead = how == DownKind::Shot;
    let mark = crate::snatch::snatch_is_man(g, gd);
    let hostage = crate::rescue::rescue_is_hostage(g, gd);
    let (name, label, her) = (g.cv[cid].name.clone(), g.cv[cid].label.clone(), g.cv[cid].her.clone());
    if by == DownBy::Player {
        if dead {
            g.stats.civilians.killed += 1;
            let pl = g.player;
            let weapon = match crate::game::player_gun_in_hand(g, pl) {
                Some(fa) => { let def = weapon_short_def(g, fa); crate::gear::plain_short(&def) }
                None => String::from("a round"),
            };
            let d = data_n(vec![("who", Dv::T(name.clone())), ("label", Dv::T(label.clone())), ("weapon", Dv::T(weapon))]);
            let pos = g.ch[c].pos;
            crate::chronicle::chronicle_n(g, ChronKind::CivilianKilled, ChronCountKey::CivKilled, Some(&d), Some(pos));
        } else if !mark {
            g.stats.civilians.ko += 1;
            let l = line.unwrap_or("");
            let method = if how == DownKind::Choked { "choke" } else if l.contains("whip") { "whip" } else if l.starts_with("ring") { "ring" } else if l.ends_with("behind the wall when it went") { "charge" } else { "takedown" };
            let d = data_n(vec![("who", Dv::T(name.clone())), ("label", Dv::T(label.clone())), ("method", Dv::T(String::from(method)))]);
            let pos = g.ch[c].pos;
            crate::chronicle::chronicle_n(g, ChronKind::CivilianKO, ChronCountKey::CivKO, Some(&d), Some(pos));
        }
        if dead && g.tune.civilian_kill_fails && crate::mission::mission_live(g) {
            let at = if g.mission_in_tour { -1.0 } else { g.time + crate::mission::DOWN_CARD_SECS };
            g.mission.stage = MissionStage::Failed; g.mission.drive_t = -1.0; g.mission.down_card_at = at;
            let text = format!("MISSION FAILED — you killed {}, the {}. press Enter to restart", name, label);
            crate::game::game_msg(g, &text);
        } else if !hostage && !mark {
            let text = match line {
                Some(l) => String::from(l),
                None => if dead { format!("CIVILIAN CASUALTY — {}, the {}, is dead. that is on your record, and they will find {}", name, label, her) }
                    else if how == DownKind::Choked { format!("{} choked out — breathing. a civilian: that costs, and they may find {}", name, her) }
                    else { format!("{} out cold — breathing. a civilian: that costs, and they may find {}", name, her) },
            };
            crate::game::game_msg(g, &text);
        }
    } else if !hostage && !mark {
        let text = match line { Some(l) => String::from(l), None => format!("{}, the {}, is down", name, label) };
        crate::game::game_msg(g, &text);
    }
    log(g, gd, &format!("DOWN — {} by {}", how.as_str(), by.as_str()));
    let at = bone_or(g, c, Bone::Chest, 1.1);
    let body_m = g.k.civ.body_m;
    let mut i = 0;
    while i < g.civilians.len() {
        let o = g.civilians[i]; i += 1;
        if o == gd || g.gd[o].state == GuardState::Dead || g.gd[o].held.is_some() { continue; }
        let oc = g.gd[o].civ.unwrap();
        if g.cv[oc].state == CivState::Gone || !civ_point_in_view(g, o, at, body_m) { continue; }
        g.cv[oc].known_bodies.add(gd);
        let pos = g.ch[c].pos;
        to_alarmed(g, o, CivCause::Body, pos, true);
    }
}

/// a point in her open view, with a clear line (civilians.ts civPointInView)
pub fn civ_point_in_view(g: &mut crate::game::Game, gd: GuardId, p: V3, range: f64) -> bool {
    let c = g.gd[gd].ch; let eye = bone_or(g, c, Bone::Head, 1.65); let look = g.ch[c].chest_dir; let e = eye_of(g, gd);
    let to = v3::sub(p, eye); let l = v3::len(to); if l > range { return false; } if m::hyp2(to[0], to[2]) < 0.3 { return true; }
    if v3::dot([look[0], 0.0, look[2]], v3::normalize([to[0], 0.0, to[2]])) < m::cos(m::max(45.0, e.half_deg) * v3::DEG) { return false; }
    !crate::scene_collision::segment_blocked(g, eye, p, None, None, None)
}

/// living staff, free, on the shift's list (civilians.ts staffCalm)
pub fn staff_calm(g: &mut crate::game::Game, o: GuardId) -> bool {
    g.gd[o].state != GuardState::Dead && g.gd[o].team == Team::Guard && g.gd[o].held.is_none() && g.gd[o].script.is_none() && g.guards.contains(&o)
}

/// once a frame: the men react to a frightened woman, one in a corner, one on the floor (civilians.ts guardsNotice)
pub fn guards_notice(g: &mut crate::game::Game, dt: f64) {
    if !g.ai_enabled { return; }
    let mut ci = 0;
    while ci < g.civilians.len() {
        let cv = g.civilians[ci]; ci += 1;
        let cid = g.gd[cv].civ.unwrap(); let c = g.gd[cv].ch;
        if g.cv[cid].state == CivState::Gone { continue; }
        if g.gd[cv].state == GuardState::Dead {
            if !g.gd[cv].found {
                let mut i = 0;
                while i < g.guards.len() {
                    let o = g.guards[i]; i += 1;
                    if g.gd[o].state == GuardState::Dead || g.gd[o].team != Team::Guard || g.gd[o].held.is_some() || !g.guards.contains(&o) || g.gd[o].state == GuardState::Alert || g.time < g.gd[o].dazzled_until { continue; }
                    if body_in_view_of(g, o, cv) { found_down(g, cv, o); break; }
                }
            }
            continue;
        }
        if g.gd[cv].held.is_some() { continue; }
        let st = g.cv[cid].state;
        let panicking = st == CivState::Flee || st == CivState::Alarmed || (st == CivState::Report && !g.cv[cid].acked);
        if panicking { notice_panic(g, cv, cid, c, dt); }
        if g.cv[cid].state == CivState::Cower && !g.cv[cid].reported && !(g.cv[cid].found_told && g.cv[cid].found_line_at >= 0.0 && g.time - g.cv[cid].found_line_at < 4.0) {
            notice_cowering(g, cv, cid, c, dt);
        }
    }
}

/// (1) of guardsNotice: every calm man who hears or sees her running turns toward where she came from; she runs to him if he is nearer
fn notice_panic(g: &mut Game, cv: GuardId, cid: CivId, c: CharId, dt: f64) {
    let vis = civ_visibility(g, cv, dt);
    let mouth = bone_or(g, c, Bone::Head, 1.6); let chest = bone_or(g, c, Bone::Chest, 1.2);
    let screaming = g.time - g.cv[cid].scream_at < 0.5;
    let (scream_m, see_m, see_lit) = (g.k.civ.scream_m, g.k.civ.see_m, g.k.civ.see_lit);
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i]; i += 1;
        let oc = g.gd[o].ch;
        if !staff_calm(g, o) || (g.gd[o].state != GuardState::Patrol && g.gd[o].state != GuardState::Suspicious) { continue; }
        let alarm_n = g.cv[cid].alarm_n;
        if g.cv[cid].told.get(o).copied() == Some(alarm_n) || g.cv[cid].report_to == Some(o) { continue; }
        let d = v3::dist(g.ch[oc].pos, g.ch[c].pos); if d > m::max(scream_m, see_m) + 2.0 { continue; }
        let (mut heard, mut seen) = (false, false);
        if screaming && d <= scream_m + 2.0 {
            let ear = bone_or(g, oc, Bone::Head, 1.6);
            let pr = crate::weather::hear_path(g, mouth, ear, None);
            heard = pr.path_len < scream_m * pr.carry;
        }
        if !heard && d <= see_m { seen = point_in_view(g, o, chest, Some(see_m), Some(58.0)) && (vis > see_lit || d < 4.0); }
        if !heard && !seen { continue; }
        g.cv[cid].told.set(o, alarm_n); g.stats.civilians.noticed += 1;
        let from = g.cv[cid].cause_pos.unwrap_or(g.ch[c].pos);
        {
            let man = &mut g.gd[o];
            man.awareness = m::max(man.awareness, 0.5);
            if man.last_known.is_none() || man.state == GuardState::Patrol { man.last_known = Some(from); }
            if man.state == GuardState::Patrol { man.state = GuardState::Suspicious; man.search_t = 0.0; man.react_t = m::max(man.react_t, 0.7); man.path = Vec::new(); man.path_goal = None; man.stall_ref = None; }
        }
        if !bubble_within(g, o, 1.5) {
            let text = if seen { format!("{}? — hey. HEY. what is it —", g.cv[cid].name) } else { format!("— that was {} —", g.cv[cid].name) };
            say(g, o, &text, None, None, None);
        }
        let text = format!("{} {} ({} m) → looks toward ({}, {})", g.gd[o].callsign, if heard { "heard her" } else { "saw her running" }, to_fixed1(d), to_fixed1(from[0]), to_fixed1(from[2]));
        log(g, cv, &text);
        // she runs to HIM if he is the nearer thing now
        let run_to = g.cv[cid].state == CivState::Flee && !g.cv[cid].pin_target && match &g.cv[cid].target { Some(t) => t.guard != Some(o), None => false };
        if run_to && tellable(g, o) {
            let t = g.cv[cid].target.clone().unwrap();
            let here = g.ch[c].pos;
            let cur = path_metres(g, here, t.pos).m + (if t.kind == CivTargetKind::Guard { 0.0 } else { g.k.civ.guard_bonus_m });
            let op = g.ch[oc].pos;
            let alt = path_metres(g, here, op).m;
            if alt < cur - 2.0 {
                let name = g.gd[o].callsign.clone();
                g.cv[cid].target = Some(CivTarget { kind: CivTargetKind::Guard, guard: Some(o), spot: None, pos: op, name: name.clone(), plan_pos: op });
                go_to(g, cv, op, Some(true));
                log(g, cv, &format!("→ running to {} instead", name));
            }
        }
    }
}

/// (2) of guardsNotice: a man by her corner asks, and a beat later she tells him
fn notice_cowering(g: &mut Game, cv: GuardId, cid: CivId, c: CharId, dt: f64) {
    let mut by: Option<GuardId> = None;
    let found_m = g.k.civ.found_m;
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i]; i += 1;
        let oc = g.gd[o].ch;
        if !staff_calm(g, o) || g.gd[o].state == GuardState::Alert || v3::dist_xz(g.ch[oc].pos, g.ch[c].pos) > found_m { continue; }
        let (a, b) = (v3::add(g.ch[oc].pos, [0.0, 1.3, 0.0]), v3::add(g.ch[c].pos, [0.0, 0.9, 0.0]));
        if crate::scene_collision::solid_blocked(g, a, b, None) { continue; }
        by = Some(o); break;
    }
    let Some(by) = by else { g.cv[cid].found_t = 0.0; g.cv[cid].found_by = None; return; };
    if g.cv[cid].found_by != Some(by) { g.cv[cid].found_by = Some(by); g.cv[cid].found_t = 0.0; g.cv[cid].found_line_at = -1.0; }
    g.cv[cid].found_t += dt;
    let pos = g.ch[c].pos;
    prime_guard(g, by, pos, 0.5);
    if g.cv[cid].found_line_at < 0.0 && g.cv[cid].found_t >= g.k.civ.found_secs {
        g.cv[cid].found_line_at = g.time; g.stats.civilians.found += 1;
        let pool = crate::dialogue::civ_lines().found;
        let t = pick(g, cv, &pool); let t = fill(g, cv, &t, Some(by));
        say(g, by, &t, None, None, None);
        let text = format!("{} found her cowering", g.gd[by].callsign);
        log(g, cv, &text);
    }
    if g.cv[cid].found_line_at >= 0.0 && !g.cv[cid].found_told && g.time - g.cv[cid].found_line_at >= 1.6 {
        g.cv[cid].found_told = true; g.cv[cid].report_to = Some(by);
        let cause = g.cv[cid].cause;
        let pool = report_pool(g, cause.map(|c| c.as_str()));
        let t = pick(g, cv, &pool);
        civ_say(g, cv, &t, None, None);
        later(g, 1.8, &LaterP { kind: PendKind::FoundReport, cv, o: Some(by), ..LaterP::zeroed() });
    }
}

/// a downed civilian in this man's open view (civilians.ts bodyInViewOf)
pub fn body_in_view_of(g: &mut crate::game::Game, o: GuardId, cv: GuardId) -> bool {
    let oc = g.gd[o].ch; let cc = g.gd[cv].ch;
    let eye = bone_or(g, oc, Bone::Head, 1.65); let look = g.ch[oc].chest_dir;
    let body = match g.ch[cc].bones.get(Bone::Hips) { Some(p) => *p, None => g.ch[cc].pos };
    let to = v3::sub(body, eye); if v3::len(to) > 12.0 { return false; }
    if m::hyp2(to[0], to[2]) > 0.4 && v3::dot([look[0], 0.0, look[2]], v3::normalize([to[0], 0.0, to[2]])) < m::cos(60.0 * v3::DEG) { return false; }
    !crate::scene_collision::segment_blocked(g, eye, v3::add(body, [0.0, 0.3, 0.0]), None, None, None)
}

/// she is a body like any body: called in, the floor up (civilians.ts foundDown)
pub fn found_down(g: &mut crate::game::Game, cv: GuardId, o: GuardId) {
    let cid = g.gd[cv].civ.unwrap(); let body = body_pos(g, cv); let dead = g.gd[cv].down_kind == Some(DownKind::Shot);
    g.gd[cv].found = true; g.stats.civilians.found_down += 1; crate::mission::tally(g, crate::mission::TallyK::Bodies);
    let name = g.cv[cid].name.clone();
    let d = data_c(vec![("who", Dv::T(name.clone())), ("by", Dv::T(g.gd[o].callsign.clone())), ("breathing", Dv::B(!dead)), ("civilian", Dv::B(true))]);
    crate::chronicle::chronicle(g, ChronKind::BodyFound, Some(&d), Some(body));
    let now = g.time;
    g.gd[o].witness.saw_body = Some(WitnessSawBody { victim: name.clone(), t: now, breathing: !dead });
    if g.gd[o].script.is_none() {
        let man = &mut g.gd[o];
        man.awareness = m::max(man.awareness, 0.7); man.last_known = Some(body); man.react_t = 1.2; man.body_duty = None; man.task = None; man.path = Vec::new(); man.path_goal = None;
        if man.state == GuardState::Patrol || man.state == GuardState::Search { man.state = GuardState::Suspicious; man.search_t = 0.0; man.search_plan = None; man.pieing = None; }
    }
    let lines = crate::dialogue::civ_lines();
    let text = if dead { lines.found_down.dead.clone() } else if away(g) { lines.guide.own.found_cold.clone() } else { lines.found_down.cold.clone() };
    let t = fill(g, cv, &text, Some(o));
    say(g, o, &t, Some(true), None, None);
    let oc = g.gd[o].ch;
    let at = match g.ch[oc].bones.get(Bone::Head) { Some(p) => *p, None => g.ch[oc].pos };
    crate::audio_iface::game_audio_like_play(g, "radio", Some(at), Some(0.9), None);
    let mut i = 0;
    while i < g.guards.len() {
        let other = g.guards[i]; i += 1;
        if other == o || g.gd[other].state == GuardState::Dead || g.gd[other].team != Team::Guard || g.gd[other].held.is_some() { continue; }
        let man = &mut g.gd[other];
        man.witness.called_to_body = Some(WitnessCalledToBody { victim: name.clone(), t: now, breathing: !dead });
        if man.script.is_some() { continue; }
        man.awareness = m::max(man.awareness, 0.45);
        if man.task.is_none() && man.state != GuardState::Alert { man.last_known = Some(body); }
    }
    if !g.quiet_utility {
        g.alarm.pos[0] = body[0]; g.alarm.pos[1] = 0.0; g.alarm.pos[2] = body[2]; g.alarm.placed = true;
        let lvl = escalation_of(g);
        later(g, 1.4, &LaterP { kind: PendKind::BodyEsc, cv, o: Some(o), dead: Some(dead), lvl: Some(lvl), ..LaterP::zeroed() });
        if dead { later(g, 4.5, &LaterP { kind: PendKind::Outrage, cv, o: None, ..LaterP::zeroed() }); }
    }
    let text = format!("found {} by {}", if dead { "DEAD" } else { "out cold" }, g.gd[o].callsign);
    log(g, cv, &text);
}

/// her scream, to the other civilians' ears (civilians.ts hearScream)
pub fn hear_scream(g: &mut crate::game::Game, gd: GuardId) {
    let cid = g.gd[gd].civ.unwrap(); let c = g.gd[gd].ch;
    let mouth = bone_or(g, c, Bone::Head, 1.6);
    let scream_m = g.k.civ.scream_m;
    let mut i = 0;
    while i < g.civilians.len() {
        let o = g.civilians[i]; i += 1;
        if o == gd || g.gd[o].state == GuardState::Dead || g.gd[o].held.is_some() { continue; }
        let oc = g.gd[o].civ.unwrap(); let och = g.gd[o].ch;
        let st = g.cv[oc].state;
        if st != CivState::Work && st != CivState::Startled && st != CivState::Cower { continue; }
        if v3::dist(g.ch[och].pos, g.ch[c].pos) > scream_m + 2.0 { continue; }
        let ear = bone_or(g, och, Bone::Head, 1.6);
        let pr = crate::weather::hear_path(g, mouth, ear, None);
        if pr.path_len < scream_m * pr.carry {
            let at = g.cv[cid].cause_pos.unwrap_or(g.ch[c].pos); let armed = g.cv[cid].armed;
            to_alarmed(g, o, CivCause::Scream, at, armed);
        }
    }
}

/// a calm patrolling staff man with nothing on his hands or mind and no line in the air (civilians.ts chattyEnough)
pub fn chatty_enough(g: &mut crate::game::Game, x: GuardId) -> bool {
    let up = match &g.gd[x].bubble { Some(b) => g.time - b.t < b.dur + 1.0, None => false };
    let man = &g.gd[x];
    man.team == Team::Guard && man.unit.is_none() && man.state == GuardState::Patrol && man.held.is_none() && man.script.is_none() && man.task.is_none() && man.body_duty.is_none() && !man.pinned && !man.hold && man.leader.is_none() && man.post.is_none() && man.awareness < 0.15 && !up && g.guards.contains(&x)
}

/// passing pleasantries, one a floor at a time (civilians.ts greetings)
pub fn greetings(g: &mut crate::game::Game) {
    if escalation_of(g) != 0.0 || g.alarm.episode { return; }
    if crate::sim_lightshome::mains_active(g) { return; }
    let who = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
    let players = crate::seats::men(g, &who);
    if players.iter().any(|p| g.pl[*p].holding.is_some()) { return; }
    let floor_greet_at = floor_of(g).greet_at;
    if g.time - floor_greet_at < g.k.civ.greet_floor_secs { return; }
    let mut ci = 0;
    while ci < g.civilians.len() {
        let cv = g.civilians[ci]; ci += 1;
        let cid = g.gd[cv].civ.unwrap(); let cc = g.gd[cv].ch;
        if g.cv[cid].state != CivState::Work || g.gd[cv].state == GuardState::Dead { continue; }
        if task_at(g, cv) == CivilianTask::Sit && g.gd[cv].wait > 0.0 { continue; }
        if g.time - g.cv[cid].greet_at < g.k.civ.greet_pair_secs { continue; }
        let up = match &g.gd[cv].bubble { Some(b) => g.time - b.t < b.dur + 1.0, None => false };
        if up { continue; }
        let mut i = 0;
        while i < g.guards.len() {
            let o = g.guards[i]; i += 1;
            let oc = g.gd[o].ch;
            if !chatty_enough(g, o) || v3::dist_xz(g.ch[oc].pos, g.ch[cc].pos) > g.k.civ.greet_m { continue; }
            let (a, b) = (v3::add(g.ch[oc].pos, [0.0, 1.4, 0.0]), v3::add(g.ch[cc].pos, [0.0, 1.4, 0.0]));
            if crate::scene_collision::solid_blocked(g, a, b, None) { continue; }
            let now = g.time;
            g.cv[cid].greet_at = now; floor_of(g).greet_at = now; g.stats.civilians.greetings += 1;
            let wipes = g.cv[cid].def.tasks.as_ref().map_or(false, |t| t.contains(&CivilianTask::Wipe));
            let pool: Vec<String> = crate::dialogue::civ_lines().guide.greet_civ.into_iter().filter(|t| !(t.contains("wet floor") && !wipes)).collect();
            let t = pick(g, cv, &pool);
            civ_say(g, cv, &t, None, None);
            later(g, 1.3, &LaterP { kind: PendKind::GreetReply, cv, o: Some(o), ..LaterP::zeroed() });
            let text = format!("greets {}", g.gd[o].callsign);
            log(g, cv, &text);
            return;
        }
    }
}

/// the response team coming clears them off the floor (civilians.ts evacuationCheck)
pub fn evacuation_check(g: &mut crate::game::Game) {
    if g.qrf.state == crate::qrf::QrfState::Idle { return; }
    let mut movable: Vec<GuardId> = Vec::new();
    for i in 0..g.civilians.len() {
        let cv = g.civilians[i];
        if g.gd[cv].state == GuardState::Dead || g.gd[cv].held.is_some() { continue; }
        let st = g.cv[g.gd[cv].civ.unwrap()].state;
        if st == CivState::Gone || st == CivState::Evacuate || crate::snatch::snatch_is_man(g, cv) { continue; }
        movable.push(cv);
    }
    if !floor_of(g).evac_told {
        floor_of(g).evac_told = true;
        if !movable.is_empty() {
            let rw = if away(g) { crate::qrf::response_words(g) } else { None };
            let line = match rw {
                Some(rw) => format!("anybody who isn't ours, clear out of {} — {} is coming in", rw.site, rw.team),
                None => crate::dialogue::civ_lines().now.qrf,
            };
            control(g, movable[0], &line);
            log(g, movable[0], "control clears the floor (QRF inbound)");
        }
    }
    for cv in movable {
        let cid = g.gd[cv].civ.unwrap(); let c = g.gd[cv].ch;
        if g.cv[cid].state == CivState::Phone && g.cv[cid].talk_t < g.k.civ.phone_secs { continue; }
        end_task(g, cv); g.cv[cid].rise_t = 0.0; g.ch[c].anim.act = None;
        let Some(t) = pick_target(g, cv, Some(PickTargetOpts { exits_only: Some(true) })) else { continue; };
        to_flee(g, cv, &t, true);
    }
}

/// the switch and the fail rule, kept on the machine: the browser's storage is the host's, and with no storage the TypeScript's own try does nothing (civilians.ts saveCivTune)
pub fn save_civ_tune(g: &mut crate::game::Game) {}

/// the stored switch and `?nocivilians`: the browser's storage and the address are the host's, which sets g.tune; with neither the TypeScript's own tries do nothing (civilians.ts loadCivTune)
pub fn load_civ_tune(g: &mut crate::game::Game) {}

/// the switch flipped live (civilians.ts setCivilians)
pub fn set_civilians(g: &mut crate::game::Game, on: bool) {
    g.tune.civilians = on;
    save_civ_tune(g);
    reset_civilians(g);
}

/// the panel's line: 'civilians ▸ Marta WORK wipe @wp3 (14 s) aw 0.00 | Owen FLEE → Novak [gun, armed] aw 1.00' (civilians.ts civiliansSummary)
pub fn civilians_summary(g: &mut crate::game::Game) -> String {
    if !g.tune.civilians { return String::from("civilians ▸ off"); }
    if g.civilians.is_empty() { return String::from("civilians ▸ none authored on this level"); }
    if !civilians_on_floor(g) { return String::from("civilians ▸ off the floor (the tour / daylight)"); }
    let mut rows: Vec<String> = Vec::new();
    for i in 0..g.civilians.len() {
        let cv = g.civilians[i];
        let cid = g.gd[cv].civ.unwrap();
        let name = g.cv[cid].name.clone();
        if g.gd[cv].state == GuardState::Dead {
            let kind = g.gd[cv].down_kind.map_or("null", |k| k.as_str());
            rows.push(format!("{} DOWN ({}{})", name, kind, if g.gd[cv].found { ", found" } else { "" }));
            continue;
        }
        let st = g.cv[cid].state;
        let mut bits = vec![format!("{} {}", name, st.as_str().to_ascii_uppercase())];
        if st == CivState::Work {
            let (wait, wp) = (g.gd[cv].wait, g.gd[cv].wp);
            bits.push(if wait > 0.0 { format!("{} @wp{} ({} s)", task_at(g, cv).as_str(), wp, m::ceil(wait)) } else { format!("→ wp{}", wp) });
        }
        if let Some(t) = &g.cv[cid].target { bits.push(format!("→ {}", t.name)); }
        if let Some(o) = g.cv[cid].report_to {
            if st == CivState::Report || g.cv[cid].reported { bits.push(format!("told {}", g.gd[o].callsign)); }
        }
        if let Some(cause) = g.cv[cid].cause {
            let cv_row = &g.cv[cid];
            bits.push(format!("[{}{}{}{}]", cause.as_str(), if cv_row.armed { ", armed" } else { "" }, if cv_row.reported { ", reported" } else { "" }, if cv_row.phoned { ", phoned" } else { "" }));
        }
        if st == CivState::Cower && g.cv[cid].complied { bits.push(format!("complied {} min", m::max(0.0, m::ceil((g.cv[cid].comply_until - g.time) / 60.0)))); }
        bits.push(format!("aw {}", to_fixed2(g.gd[cv].awareness)));
        rows.push(bits.join(" "));
    }
    format!("civilians ▸ {}", rows.join(" | "))
}

/// the panel's and the probes' buttons: frighten one now, or send her to a phone (civilians.ts civPanic)
pub fn civ_panic(g: &mut crate::game::Game, gd: Option<GuardId>, what: Option<crate::civilians::CivPanicWhat>) -> String {
    let what = what.unwrap_or(CivPanicWhat::Panic);
    let Some(gd) = gd else { return String::from("no such civilian"); };
    let Some(cid) = g.gd[gd].civ else { return String::from("no such civilian"); };
    if !civilians_on_floor(g) { return String::from("not now (the tour / daylight)"); }
    let name = g.cv[cid].name.clone();
    if g.gd[gd].state == GuardState::Dead { return format!("{} is down", name); }
    if g.gd[gd].held.is_some() { return format!("{} is in your arm", name); }
    let pp = g.ch[g.pl[g.player].ch].pos; let c = g.gd[gd].ch;
    if what == CivPanicWhat::Phone {
        let now = g.time;
        {
            let cv = &mut g.cv[cid];
            cv.cause = Some(CivCause::Panel); cv.cause_pos = Some(pp); cv.cause_at = now; cv.alarm_n += 1.0; cv.reported = false; cv.phoned = false; cv.bad_targets.clear();
        }
        end_task(g, gd); g.cv[cid].rise_t = 0.0; g.ch[c].anim.act = None;
        let mut best: Option<CivSpot> = None; let mut bm = f64::INFINITY;
        for s in civ_spots(g).phones {
            let here = g.ch[c].pos;
            let d = path_metres(g, here, s.pos).m;
            if d < bm { bm = d; best = Some(s); }
        }
        let Some(best) = best else { return format!("{} cannot reach a phone from here", name); };
        let t = CivTarget { kind: CivTargetKind::Phone, guard: None, pos: best.pos, name: best.name.clone(), plan_pos: best.pos, spot: Some(best.clone()) };
        to_flee(g, gd, &t, false); g.cv[cid].pin_target = true;
        return format!("{} runs for {}", name, best.name);
    }
    let st = g.cv[cid].state;
    if st != CivState::Work && st != CivState::Startled && st != CivState::Cower { return format!("{} is already {}", name, st.as_str()); }
    to_alarmed(g, gd, CivCause::Panel, pp, false);
    format!("{} panics (as if she had seen you where you stand)", name)
}

/// a check of the spots and every civilian's marks against the level: free nav cells, every spot reachable (civilians.ts checkCivSpots)
pub fn check_civ_spots(g: &mut crate::game::Game) -> Vec<String> {
    use crate::scene_collision::{nav_grid_find_path, nav_grid_is_blocked};
    let mut errs: Vec<String> = Vec::new();
    let found = civ_spots(g);
    let spots: Vec<CivSpot> = found.phones.into_iter().chain(found.exits).collect();
    for s in &spots {
        if nav_grid_is_blocked(g, s.pos[0], s.pos[2]) { errs.push(format!("{} ({}, {}) is not a free nav cell", s.name, s.pos[0], s.pos[2])); }
    }
    let defs = g.level.civilians.clone().unwrap_or_default();
    for def in &defs {
        for (i, p) in def.points.iter().enumerate() {
            if nav_grid_is_blocked(g, p[0], p[2]) { errs.push(format!("{} mark {} ({}, {}) is not a free nav cell", def.name, i, p[0], p[2])); }
        }
        for s in &spots {
            if nav_grid_find_path(g, def.points[0], s.pos, Some(6000.0), None).is_none() { errs.push(format!("{} cannot reach {} from her first mark", def.name, s.name)); }
        }
        for i in 1..def.points.len() {
            if nav_grid_find_path(g, def.points[i - 1], def.points[i], Some(6000.0), None).is_none() { errs.push(format!("{}: no path mark {} → {}", def.name, i - 1, i)); }
        }
    }
    errs
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn spell_words_says_the_digits() {
        assert_eq!(spell_words("7391"), "Seven three nine one");
        assert_eq!(spell_words("0"), "Oh");
        assert_eq!(spell_words("a1"), "A one");
    }
    #[test]
    fn to_fixed_rounds_a_tie_up_as_the_language_does() {
        assert_eq!(to_fixed1(0.25), "0.3");
        assert_eq!(to_fixed1(-0.25), "-0.3");
        assert_eq!(to_fixed1(0.75), "0.8");
        assert_eq!(to_fixed1(1.45), "1.4"); // 1.45 is a little under, as a double
        assert_eq!(to_fixed1(-0.0), "0.0");
        assert_eq!(to_fixed1(12.0), "12.0");
    }
    #[test]
    fn to_fixed2_rounds_a_tie_up_as_the_language_does() {
        assert_eq!(to_fixed2(0.125), "0.13");
        assert_eq!(to_fixed2(0.375), "0.38");
        assert_eq!(to_fixed2(-0.125), "-0.13");
        assert_eq!(to_fixed2(1.005), "1.00"); // a little under, as a double
        assert_eq!(to_fixed2(0.0), "0.00");
        assert_eq!(to_fixed2(1.0), "1.00");
        assert_eq!(to_fixed2(0.456), "0.46");
    }
    #[test]
    fn replace_ci_takes_the_first_match_in_any_case() {
        assert_eq!(replace_ci("Code: SEVEN three nine one, ok", "seven three nine one", "One two"), "Code: One two, ok");
    }
}
