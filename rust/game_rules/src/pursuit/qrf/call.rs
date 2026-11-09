//! src/game/qrf.ts lines 1 to 426
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- private helpers

/// `text.length > 96 ? text.slice(0, 94) + '…' : text`, in JavaScript's UTF-16 units.
fn clip_line(text: &str) -> String {
    if text.encode_utf16().count() <= 96 { return String::from(text); }
    let head: Vec<u16> = text.encode_utf16().take(94).collect();
    let mut s = String::from_utf16_lossy(&head);
    s.push('…');
    s
}

fn team_of(side: crate::qrf::SideOfTeam) -> crate::teams::Team {
    match side {
        crate::qrf::SideOfTeam::Guard => crate::teams::Team::Guard,
        crate::qrf::SideOfTeam::Hostile => crate::teams::Team::Hostile,
    }
}

fn word_key(who: crate::picture::Who) -> crate::qrf::QrfWordKey {
    match who {
        crate::picture::Who::Guard(x) => crate::qrf::QrfWordKey::Guard(x),
        crate::picture::Who::Player => crate::qrf::QrfWordKey::Player,
    }
}

// ---------------------------------------------------------------- the level's own team

/// This level's block, if it has one.   (qrf.ts responseOf)
pub(super) fn response_of(g: &crate::game::Game) -> Option<crate::scene_scenarios::ResponseData> {
    g.level.scenarios.as_ref().and_then(|s| s.response.clone())
}

/// A squad's room clear under way on a level whose block carries `raid`.   (qrf.ts raid)
pub(super) fn raid(g: &crate::game::Game) -> bool {
    if !g.night_breach.active || g.rescue.active { return false; }
    response_of(g).map_or(false, |r| r.raid.is_some())
}

/// The raid is over: the room clear ended.   (qrf.ts raidOver)
pub(super) fn raid_over(g: &crate::game::Game) -> bool {
    g.room_clear.end.is_some()
}

/// The side the team stands on: the shift's, or in a raid the defenders'.   (qrf.ts sideOfTeam)
pub(super) fn side_of_team(g: &crate::game::Game) -> crate::qrf::SideOfTeam {
    if raid(g) { crate::qrf::SideOfTeam::Hostile } else { crate::qrf::SideOfTeam::Guard }
}

/// A raid: the men who can call it in.   (qrf.ts callers)
pub(super) fn callers(g: &crate::game::Game) -> Vec<GuardId> {
    let id = response_of(g).and_then(|r| r.raid).and_then(|a| a.room).filter(|s| !s.is_empty());
    let of: Vec<GuardId> = match &id {
        Some(id) => match g.room_clear.rooms.iter().find(|r| r.def.id == *id) {
            Some(room) => room.defenders.clone(),
            None => Vec::new(),
        },
        None => g.night_breach.hostiles.clone(),
    };
    let mut out = Vec::new();
    for x in of {
        if g.gd[x].state != crate::game::GuardState::Dead && g.gd[x].held.is_none() && !crate::owner::has(g, x, crate::owner::by::RUNNER) && g.guards.contains(&x) {
            out.push(x);
        }
    }
    out
}

/// A raid: the caller who heard gunfire and has lived `callSecs` since.   (qrf.ts raidVoice)
pub(super) fn raid_voice(g: &crate::game::Game) -> Option<GuardId> {
    for x in callers(g) {
        let heard = g.qrf.heard_at.get(x).copied().unwrap_or(f64::INFINITY);
        if g.time - heard >= g.k.response.call_secs { return Some(x); }
    }
    None
}

/// A raid: is the team up, a man of it living?   (qrf.ts responseUp)
pub fn response_up(g: &crate::game::Game) -> bool {
    if !raid(g) { return false; }
    if g.qrf.state != crate::qrf::QrfState::Arriving && g.qrf.state != crate::qrf::QrfState::Deployed { return false; }
    g.qrf.men.iter().any(|&m| g.gd[m].state != crate::game::GuardState::Dead && g.guards.contains(&m))
}

/// The team's name on the net.   (qrf.ts responseTeam)
pub fn response_team(g: &crate::game::Game) -> String {
    response_of(g).map_or_else(|| String::from("the response team"), |r| r.team)
}

/// The dealt way of the level's block, as the rest reads it; kept until the way changes.   (qrf.ts marksOf)
pub(super) fn marks_of(g: &mut crate::game::Game) -> Option<crate::qrf::Marks> {
    if g.qrf.marks.is_some() && g.qrf.marks_on && g.qrf.marks_way == g.qrf.way { return g.qrf.marks.clone(); }
    let way = g.qrf.way;
    let made = response_of(g).and_then(|r| {
        let y = r.ways.get(way as usize).or(r.ways.first()).cloned()?;
        Some(crate::qrf::Marks {
            door: y.door,
            coming: y.coming.unwrap_or_else(|| alloc::format!("coming in by {}", y.name)),
            name: y.name,
            muster: y.muster,
            entry: y.entry,
            entry_face: y.entry_face,
            clear: y.clear,
            stairs: y.stairs.unwrap_or(false),
            posts: r.posts.unwrap_or_default(),
            outside: r.outside.unwrap_or_default(),
            site: r.site,
            at: r.at,
            team: r.team,
            tag: r.tag.unwrap_or_else(|| String::from("QRF")),
        })
    });
    g.qrf.marks = made.clone();
    g.qrf.marks_on = true;
    g.qrf.marks_way = way;
    made
}

/// How many more men the floor can be given.   (qrf.ts roomFor)
pub(super) fn room_for(g: &crate::game::Game) -> f64 {
    let n = (crate::guards::MAX_SKINNED - g.civilians.len() as f64) - crate::seats::seat_count(g) - g.guards.len() as f64;
    if raid(g) {
        m::min(n, crate::scene_scenarios::SQUAD_MAX_MEN - g.night_breach.hostiles.len() as f64 - g.qrf.men.len() as f64)
    } else {
        n
    }
}

/// The spawn's character budget.   (qrf.ts budgetOf)
pub(super) fn budget_of(g: &crate::game::Game) -> crate::qrf::BudgetOf {
    crate::qrf::BudgetOf::Arm0(crate::qrf::BudgetOfArm0 { budget: crate::guards::MAX_SKINNED - g.civilians.len() as f64 })
}

/// The most men this level's response may ever have on the floor.   (qrf.ts mostMen)
pub(super) fn most_men(g: &crate::game::Game) -> f64 {
    match response_of(g).and_then(|r| r.men) {
        Some(men) => men[1],
        None => 8.0,
    }
}

// ---------------------------------------------------------------- class Qrf

/// Does this man belong to the response (element or perimeter)?   (qrf.ts Qrf.owns)
pub fn qrf_owns(g: &crate::game::Game, gd: GuardId) -> bool {
    g.qrf.men.contains(&gd) || g.qrf.perim.as_ref().map_or(false, |p| p.men.iter().any(|m| m.gd == gd))
}

// ---------------------------------------------------------------- knobs kept per machine

// ---------------------------------------------------------------- small shared helpers

/// Distance on the floor.   (qrf.ts flat)
pub(super) fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// A row of the response's log, which keeps its last 150 to 200.   (qrf.ts note)
pub(super) fn note(g: &mut crate::game::Game, what: &str) {
    g.qrf.log.push(crate::qrf::QrfLogItem { what: String::from(what) });
    if g.qrf.log.len() > 200 { g.qrf.log.drain(0..50); }
}

/// Control's voice: a line on the net with nobody in the building saying it.   (qrf.ts control)
pub(super) fn control(g: &mut crate::game::Game, text: &str) {
    crate::game::game_msg(g, &alloc::format!("radio: CONTROL — {}", text));
    crate::audio_iface::game_audio_like_play(g, "radio", None, Some(0.55), None);
}

/// Control's voice, with a row in the run's timeline.   (qrf.ts controlRow)
pub(super) fn control_row(g: &mut crate::game::Game, text: &str) {
    control(g, text);
    let mut data: JsMap<String, Option<crate::chronicle::ChronicleDataValue>> = JsMap::new();
    data.set(String::from("who"), Some(crate::chronicle::ChronicleDataValue::Text(String::from("CONTROL"))));
    data.set(String::from("text"), Some(crate::chronicle::ChronicleDataValue::Text(clip_line(text))));
    crate::chronicle::chronicle(g, crate::chronicle::ChronKind::Radio, Some(&data), None);
}

/// A plan's door as the element says it.   (qrf.ts doorSaid)
pub(super) fn door_said(g: &crate::game::Game, name: &str) -> String {
    let d = crate::doors::doors_by_name(g, name);
    let written = match d { Some(d) => crate::doornames::door_written_name(g, d), None => None };
    if let Some(n) = written { return n; }
    match d {
        Some(d) => alloc::format!("{} door", crate::tactics::door_label(g, d)),
        None => alloc::format!("{} door", name.replace('_', " ")),
    }
}

/// A response man's radio line: his bubble, and the net's log line under his callsign.   (qrf.ts radio)
pub(super) fn radio(g: &mut crate::game::Game, gd: Option<GuardId>, text: &str) {
    let x = match gd {
        Some(x) if g.gd[x].state != crate::game::GuardState::Dead => x,
        _ => { control(g, text); return; }
    };
    let dur = crate::guards::bubble_secs(text);
    g.gd[x].bubble = Some(crate::game::GuardBubble { text: String::from(text), t: g.time, dur, radio: true });
    crate::handwork::radio_hand(g, x, dur);
    let line = alloc::format!("radio: {} — {}", g.gd[x].callsign, text);
    crate::game::game_msg(g, &line);
    let c = g.gd[x].ch;
    let at = g.ch[c].bones.get(crate::character::Bone::Head).copied().unwrap_or(g.ch[c].pos);
    crate::audio_iface::game_audio_like_play(g, "radio", Some(at), Some(0.7), None);
    crate::chronicle::chron_radio(g, x, text);
}

/// The shift's man best placed to call it in: living, not held, staff, calm for choice.   (qrf.ts staffVoice)
fn staff_voice(g: &crate::game::Game) -> Option<GuardId> {
    let mut best = None;
    let mut br = -1;
    for &x in &g.guards {
        let m = &g.gd[x];
        if m.unit.is_some() || m.team != crate::teams::Team::Guard || m.state == crate::game::GuardState::Dead || m.held.is_some() { continue; }
        let r = if m.state == crate::game::GuardState::Alert { 0 } else if m.state == crate::game::GuardState::Patrol { 2 } else { 1 };
        if r > br { br = r; best = Some(x); }
    }
    best
}

/// The shift's dead who have been found.   (qrf.ts bodiesFound)
pub(super) fn bodies_found(g: &crate::game::Game) -> f64 {
    let mut n = 0.0;
    for &x in &g.guards {
        let m = &g.gd[x];
        if m.unit.is_none() && m.state == crate::game::GuardState::Dead && m.found { n += 1.0; }
    }
    n
}

/// The run is Sam's to answer for.   (qrf.ts live)
pub(super) fn live(g: &mut crate::game::Game) -> bool {
    if raid(g) {
        return g.ai_enabled && !crate::seats::all_down(g) && has_response(g);
    }
    g.ai_enabled && !crate::seats::all_down(g) && !g.night_breach.active && !g.rescue.active && has_response(g)
}

/// This level has a response team.   (qrf.ts hasResponse)
pub(super) fn has_response(g: &crate::game::Game) -> bool {
    let r = match response_of(g) { Some(r) => r, None => return false };
    if r.raid.is_some() && !raid(g) { return false; }
    for y in &r.ways {
        match y.door.as_deref() {
            None | Some("") => return true,
            Some(d) => if crate::doors::doors_by_name(g, d).is_some() { return true; },
        }
    }
    false
}

/// For the panel's buttons: is there a way in to stand a team on?   (qrf.ts responseHere)
pub(super) fn response_here(g: &crate::game::Game) -> bool {
    has_response(g)
}

/// The element's living men on the floor.   (qrf.ts alive)
pub(super) fn alive(g: &crate::game::Game) -> Vec<GuardId> {
    g.qrf.men.iter().copied().filter(|&m| g.gd[m].state != crate::game::GuardState::Dead && g.guards.contains(&m)).collect()
}

/// Is this man on the defenders' net?   (qrf.ts onTheirNet)
fn on_their_net(g: &crate::game::Game, x: GuardId) -> bool {
    let side = team_of(side_of_team(g));
    if g.gd[x].team != side || g.gd[x].civ.is_some() || g.gd[x].state == crate::game::GuardState::Dead || g.gd[x].held.is_some() || crate::owner::has(g, x, crate::owner::by::RUNNER) { return false; }
    !raid(g) || g.qrf.men.contains(&x) || callers(g).contains(&x)
}

/// Control keeps what is said on the net, per enemy, the freshest.   (qrf.ts readNet)
pub(super) fn read_net(g: &mut crate::game::Game) {
    if g.time - g.qrf.word_t < g.k.response.word_secs { return; }
    g.qrf.word_t = g.time;
    let men = g.guards.clone();
    for m in men {
        if !on_their_net(g, m) { continue; }
        for i in crate::picture::ideas_of(g, m) {
            if i.how == crate::picture::IdeaHow::Fix { continue; }
            if i.how != crate::picture::IdeaHow::Told && g.time - i.since < g.k.picture.tell_secs { continue; }
            if i.lost { continue; }
            let key = word_key(i.who);
            let newer = match g.qrf.word.get(key) { None => true, Some(w) => i.t > w.t + 1e-6 };
            if newer { g.qrf.word.set(key, crate::qrf::QrfWordValue { at: i.at, t: i.t }); }
        }
    }
    for who in g.qrf.word.keys() {
        if let crate::qrf::QrfWordKey::Guard(x) = who {
            if g.gd[x].state == crate::game::GuardState::Dead || !g.guards.contains(&x) { g.qrf.word.delete(who); }
        }
    }
}

/// The freshest word on the net, and how many enemies it has heard of.   (qrf.ts netWord)
fn net_word(g: &crate::game::Game) -> Option<crate::qrf::NetWord> {
    let mut best: Option<(V3, f64)> = None;
    for w in g.qrf.word.values() {
        if best.map_or(true, |(_, t)| w.t > t) { best = Some((w.at, w.t)); }
    }
    let n = g.qrf.word.len() as f64;
    best.map(|(pos, t)| crate::qrf::NetWord { pos, t, n })
}

/// Where the team believes he is.   (qrf.ts fixOf)
pub(super) fn fix_of(g: &crate::game::Game) -> Option<crate::qrf::FixOf> {
    let w = net_word(g).map(|f| crate::qrf::FixOf { pos: f.pos, t: f.t });
    let a = if g.alarm.placed { Some(crate::qrf::FixOf { pos: g.alarm.pos, t: g.alarm.raised_at }) } else { None };
    match (w, a) {
        (Some(w), Some(a)) => Some(if a.t > w.t { a } else { w }),
        (Some(w), None) => Some(w),
        (None, a) => a,
    }
}

/// The brief as control gives it with the ETA.   (qrf.ts brief)
pub(super) fn brief(g: &crate::game::Game) -> String {
    let w = net_word(g);
    let f = match fix_of(g) { Some(f) => f, None => return String::from("no word on where") };
    let age = g.time - f.t;
    let old = if age < 20.0 { String::from("just now") } else if age < 90.0 { String::from("a minute old") } else { alloc::format!("{} minutes old", js::num_str(m::round(age / 60.0))) };
    let w = match w {
        Some(w) if !(f.t > w.t) => w,
        _ => { let room = crate::dialogue::room_name(g, f.pos); return alloc::format!("last word: the alarm, {}, {}", room, old); }
    };
    let who = if w.n >= 2.0 { alloc::format!("{} of them", js::num_str(m::min(w.n, 5.0))) } else { String::from("one man") };
    let room = crate::dialogue::room_name(g, f.pos);
    alloc::format!("last word: {}, {}, {}", who, room, old)
}

// ---------------------------------------------------------------- the trigger and the telegraph

/// Every man of the response, element then perimeter.
fn all_men(g: &Game) -> Vec<GuardId> {
    let mut men = g.qrf.men.clone();
    if let Some(p) = &g.qrf.perim { men.extend(p.men.iter().map(|m| m.gd)); }
    men
}

/// Ask for the response team; refused on the radio once one has come. Returns whether it was requested now.   (qrf.ts requestQrf)
pub fn request_qrf(g: &mut crate::game::Game, reason: crate::qrf::QrfReason) -> bool {
    let m0 = marks_of(g);
    if g.qrf.state != crate::qrf::QrfState::Idle {
        if g.time - g.qrf.refused_at > 20.0 {
            g.qrf.refused_at = g.time;
            let deployed = g.qrf.state == crate::qrf::QrfState::Deployed;
            let text = match &m0 {
                None => {
                    if deployed { String::from("negative, two-six — the response team is already on your floor. that's everyone we've got tonight") }
                    else { String::from("negative — QRF is already rolling to you. one team, one elevator. hold what you've got") }
                }
                Some(m0) => {
                    if deployed { alloc::format!("negative, {} — {} is already with you. that's everyone we've got tonight", m0.site, m0.team) }
                    else { alloc::format!("negative — {} is already on its way to you. there's no second team. hold what you've got", m0.team) }
                }
            };
            control(g, &text);
            note(g, "refused");
        }
        return false;
    }
    let panel = reason == crate::qrf::QrfReason::Panel;
    if !g.tune.qrf && !panel { return false; }
    if !live(g) && !panel { return false; }
    if !has_response(g) { note(g, "no-team:this level has none"); return false; }
    if room_for(g) < 2.0 {
        let line = alloc::format!("no-room:{}", js::num_str(room_for(g)));
        note(g, &line);
        return false;
    }
    let d = dealt(g, reason);
    g.qrf.way = d.way;
    g.qrf.size = d.men;
    note(g, &alloc::format!("dealt:way {}:men {}:eta {}", js::num_str(d.way), js::num_str(d.men), js::num_str(d.eta)));
    let marks = marks_of(g).unwrap();
    g.qrf.state = crate::qrf::QrfState::Requested;
    g.qrf.reason = Some(reason);
    g.qrf.requested_at = g.time;
    g.qrf.arrive_at = g.time + m::max(12.0, d.eta);
    g.qrf.answered = false;
    g.qrf.said30 = false;
    g.qrf.said10 = false;
    let rd = raid(g);
    let sp = if rd {
        match raid_voice(g) { Some(x) => Some(x), None => callers(g).first().copied() }
    } else {
        staff_voice(g)
    };
    let n = bodies_found(g);
    let who = match sp { Some(x) => g.gd[x].callsign.clone(), None => marks.site.clone() };
    let line = match reason {
        crate::qrf::QrfReason::Bodies => {
            let down = if n >= 2.0 { alloc::format!("{} men down", js::num_str(n)) } else { String::from("men down") };
            alloc::format!("control, {} — {} {} and whoever did it is still here. we need {}", who, down, marks.at, marks.team)
        }
        crate::qrf::QrfReason::Gunfire => {
            if rd { alloc::format!("control, {} — we are being hit {}. send {}", who, marks.at, marks.team) }
            else { alloc::format!("control, {} — shots fired {}. this is past us — send {}", who, marks.at, marks.team) }
        }
        _ => alloc::format!("control, {} — send {} to {}. now", who, marks.team, marks.site),
    };
    if sp.is_some() {
        radio(g, sp, &line);
    } else if reason == crate::qrf::QrfReason::Silent {
        control_row(g, &alloc::format!("all stations, control — shots reported {} and nobody there is answering. {}, go", marks.at, marks.team));
    } else {
        control(g, &alloc::format!("{} has gone quiet with an alarm up — {}, go", marks.site, marks.team));
    }
    note(g, &alloc::format!("request:{}", reason.as_str()));
    true
}

/// Staging: no enemy of theirs stands within `clearM` of this mark.   (qrf.ts clearOfFoes)
pub fn clear_of_foes(g: &crate::game::Game, p: V3) -> bool {
    let clear_m = g.k.response.clear_m;
    if crate::seats::men_left(g).into_iter().any(|pl| flat(g.ch[g.pl[pl].ch].pos, p) < clear_m) { return false; }
    for &x in &g.guards {
        if g.gd[x].team == crate::teams::Team::Operative && g.gd[x].state != crate::game::GuardState::Dead && flat(g.ch[g.gd[x].ch].pos, p) < clear_m { return false; }
    }
    true
}

/// The radio's words for this level's team, for the lines other modules say of it.   (qrf.ts responseWords)
pub fn response_words(g: &crate::game::Game) -> Option<crate::qrf::ResponseWords> {
    response_of(g).map(|r| crate::qrf::ResponseWords { site: r.site, team: r.team })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_long_line_is_clipped_in_utf16_units() {
        let short: String = "—".repeat(96);
        assert_eq!(clip_line(&short), short);
        let long: String = "—".repeat(97);
        let mut want: String = "—".repeat(94);
        want.push('…');
        assert_eq!(clip_line(&long), want);
    }
}
