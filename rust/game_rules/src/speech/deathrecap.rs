#![allow(unused_imports, unused_variables)]
//! EVERY DEATH EXPLAINS ITSELF. For every man of OUR side who goes down (a player, a teammate, a hostage) the tick keeps WHO did it, FROM WHERE and with what, and
//! the floor's last seconds: every man's place and gun line, every door leaf, every round. The end card shows how it happened.
//!
//!   THE RULE A SECOND PLAYER NEEDS: nothing kept here is handed out while anybody is still playing. `recap_card` answers None until `recap_over`, the run being
//!   over for EVERYBODY, so a man who is down beside live friends never gets the shooter's place or the floor without its fog.
//!   WHILE THE RUN IS ON the squad knows only its own (`Recap.lost_live`): a teammate lost, and, if somebody still up SAW him go down or HEARD the shot, the way it
//!   came from to the nearest eighth of the compass, and the man who saw says the side. Never who, never from where; a man found later is only down.
//!
//! IT MOVES NOTHING A RULE READS. It runs last in the tick, draws no die, deals no serial and writes its own record, its own counts and one thing more: the word a
//! teammate says over his head (`cues_aside`, a bubble of the cues' own, no strip row). Its line tests add to the ray counters. It asks the sound rule WITHOUT that
//! rule's cache (`SoundPaths.propagate`'s): the cache is the men's own ears', and an entry filed from here would hand them this file's answer for a pair of cells.
//!
//! The record (`Game.recap`) starts over with every mission and when the clock runs back: that run's deaths are not this run's. It is NOT in a kept game (`recap_left_behind`):
//! it is the picture of a death, and a load is a run in which that death has not happened.

pub use ::game_types::deathrecap::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{format, string::String, vec::Vec};

use crate::character::Bone;
use crate::game::{DownKind, GuardState, MissionStage};
use crate::Num;

/// how long after a round or a blast a man going down is still that round's doing (a shotgun's pellets are judged a few frames late; a ragdoll's state flips on the frame)
const CAUSE_SECS: f64 = 0.3;
/// a fragmentation burst reaches this far (frag FRAG.wound_m, with room for the step he took as it went)
const BLAST_M: f64 = 9.0;
/// a round with nobody's name on it whose line passed this close to him as he went down was his (an AI round never tests a civilian: a captor's round goes through
/// his hostage and the game puts her down by hand, rescue)
const LINE_M: f64 = 0.6;
/// deaths kept, the oldest let go
const DEATHS_KEPT: usize = 32;
/// how often a man lost and not yet settled is looked at, seconds
const WITNESS_SECS: f64 = 0.25;
/// a floor's path that long is too long for ears, metres
const EARS_PATH_M: f64 = 28.0;

/// a character on the floor as this tick has him: the players first, then the guards, the civilians last
#[derive(Clone, Copy)]
struct OnFloor { ch: CharId, down: bool, gd: Option<GuardId>, pl: Option<PlayerId> }

fn on_floor(g: &Game) -> Vec<OnFloor> {
    let mut out = Vec::with_capacity(g.guards.len() + g.seats.len() + 8);
    for pl in crate::seats::men(g) { out.push(OnFloor { ch: g.pl[pl].ch, down: g.pl[pl].down, gd: None, pl: Some(pl) }); }
    let guard = |g: &Game, gd: GuardId| OnFloor { ch: g.gd[gd].ch, down: g.gd[gd].state == GuardState::Dead, gd: Some(gd), pl: None };
    for &gd in g.guards.iter() { out.push(guard(g, gd)); }
    for gd in crate::civilians::civilians_shown(g) { out.push(guard(g, gd)); }
    out
}

fn xz(p: V3) -> [f64; 2] { [p[0], p[2]] }
fn head_of(g: &Game, c: CharId) -> V3 { match g.ch[c].bones.get(Bone::Head) { Some(h) => *h, None => v3::add(g.ch[c].pos, [0.0, 1.65, 0.0]) } }
fn squad_up(g: &Game, gd: GuardId) -> bool { g.gd[gd].unit.is_some() && g.gd[gd].team == crate::teams::Team::Operative && g.gd[gd].state != GuardState::Dead }

/// the man, as a take names him
fn recap_man(g: &Game, man: OnFloor) -> RecapMan {
    let id = g.ch[man.ch].id;
    match (man.gd, man.pl) {
        (Some(gd), _) => {
            let (civ, team) = (g.gd[gd].civ.is_some(), g.gd[gd].team);
            RecapMan { id, name: g.gd[gd].callsign.clone(), side: String::from(if civ { "civilian" } else { team.as_str() }), kind: if civ { RecapManKind::Civ } else { RecapManKind::Man } }
        }
        (None, pl) => RecapMan { id, name: match pl { Some(p) => crate::seats::player_name(g, p), None => String::from("Player") }, side: String::from("player"), kind: RecapManKind::Player },
    }
}

/// Whose deaths are explained: the players', their teammates', the people they came for.
fn ours(side: &str) -> bool { side == "player" || side == "operative" || side == "civilian" }

/// somebody plays: no puppet
fn live(g: &Game) -> bool { g.puppet.is_none() }

/// A GAME IS BEING KEPT (a checkpoint, by value or as bytes: the same either way): the copy has no record, and the next tick starts one from the floor as it is.
pub fn recap_left_behind(g: &mut Game) { g.recap = Recap::blank(); }

/// the record is this run's: a new mission, or a clock put back, starts it over
fn this_run(g: &mut Game) {
    let run = Some(g.mission.serial);
    if g.recap.run != run || g.time < g.recap.last_t { g.recap = Recap { run, last_t: g.time, next: g.time, ..Recap::blank() }; }
}

/// A round is about to land on a man (combat's fire_weapon, before anything is done to him): remembered as the last round into him. Whether it puts him down is the
/// game's to say: the tick reads the answer off the man himself. `was_down`: a round into a man on the floor is not the round that put him there, and must not replace it.
pub fn recap_round(g: &mut Game, victim: Option<CharId>, was_down: bool, shooter: CharId, muzzle: V3, hit: V3, def: &crate::weapons::WeaponDef, blind: Option<bool>) {
    let Some(victim) = victim else { return; };
    if was_down || !live(g) { return; }
    this_run(g);
    g.stats.deathrecap.rounds += 1;
    let (id, by, by_at, t) = (g.ch[victim].id, g.ch[shooter].id, xz(g.ch[shooter].pos), g.time);
    g.recap.rounds.set(Num(id), Round { t, by, by_at, from: xz(muzzle), to: xz(hit), weapon: crate::gear::plain_short(def), blind: blind.unwrap_or(false), loud: def.report.level >= 1.0 });
}

fn off_line(p: [f64; 2], a: [f64; 2], b: [f64; 2]) -> f64 {
    let (ux, uz) = (b[0] - a[0], b[1] - a[1]);
    let len = ux * ux + uz * uz;
    let k = if len > 1e-9 { m::max(0.0, m::min(1.0, ((p[0] - a[0]) * ux + (p[1] - a[1]) * uz) / len)) } else { 0.0 };
    m::hyp2(p[0] - a[0] - ux * k, p[1] - a[1] - uz * k)
}

/// what the record can say of a man going down now
fn explain(g: &mut Game, man: OnFloor, me: &RecapMan, floor: &[OnFloor], gaze: f64) -> RecapDeath {
    let c = man.ch;
    let (at, now) = (xz(g.ch[c].pos), g.time);
    g.recap.deaths_dealt += 1;
    let mut d = RecapDeath { t: now, victim: me.id, name: me.name.clone(), side: me.side.clone(), player: man.gd.is_none(), at, gaze, cause: RecapCause::Unknown, by: -1.0, to: at, serial: g.recap.deaths_dealt, ..RecapDeath::default() };
    let (mut blast, mut through): (Option<&RecapShot>, Option<&RecapShot>) = (None, None);
    for s in g.recap.shots.iter().rev() {
        if now - s.t > CAUSE_SECS { break; }
        if blast.is_none() && s.kind == RecapShotKind::Blast && m::hyp2(s.from[0] - at[0], s.from[1] - at[1]) <= BLAST_M { blast = Some(s); }
        if through.is_none() && s.kind == RecapShotKind::Round && s.who != me.id { if let Some(to) = s.to { if off_line(at, s.from, to) <= LINE_M { through = Some(s); } } }
    }
    let round = g.recap.rounds.get(Num(me.id)).filter(|r| now - r.t <= CAUSE_SECS);
    if let Some(r) = round { d.cause = RecapCause::Round; d.by = r.by; d.by_at = Some(r.by_at); d.from = Some(r.from); d.to = r.to; d.weapon = r.weapon.clone(); d.blind = r.blind; d.loud = r.loud; }
    else if let Some(s) = blast { d.cause = RecapCause::Blast; d.by = s.who; d.from = Some(s.from); d.loud = true; }
    else if let Some(s) = through { d.cause = RecapCause::Round; d.by = s.who; d.by_at = s.at; d.from = Some(s.from); d.weapon = s.weapon.clone().unwrap_or_default(); d.loud = s.loud.unwrap_or(false); }
    else if let Some(gd) = man.gd { if g.gd[gd].killer_id >= 0.0 { d.cause = RecapCause::Other; d.by = g.gd[gd].killer_id; } }
    if d.by >= 0.0 {
        if let Some(who) = g.recap.cast.get(Num(d.by)) { d.by_name = who.name.clone(); d.by_side = who.side.clone(); }
        if d.by_at.is_none() { if let Some(k) = floor.iter().find(|k| g.ch[k.ch].id == d.by) { d.by_at = Some(xz(g.ch[k.ch].pos)); } }
    }
    if let Some(src) = d.from.or(d.by_at) {
        let (dx, dz) = (src[0] - at[0], src[1] - at[1]);
        d.dist = m::hyp2(dx, dz);
        if d.dist > 1e-3 { let a = m::atan2(dx, dz) - gaze; d.off = m::abs(m::atan2(m::sin(a), m::cos(a))) * 180.0 / m::PI; }
    }
    d
}

/// one sample of the floor
fn sample(g: &mut Game, floor: &[OnFloor]) {
    let k = g.k.recap;
    let keep = m::ceil((k.secs + k.after + 1.0) * k.hz) as usize;
    let mut f = if g.recap.frames.len() >= keep { g.recap.frames.pop_front().unwrap_or_default() } else { RecapFrame::default() };
    f.t = g.time;
    f.n = floor.len() as f64;
    f.men.clear();
    f.doors.clear();
    for man in floor {
        let c = &g.ch[man.ch];
        let roused = man.gd.is_some_and(|gd| matches!(g.gd[gd].state, GuardState::Alert | GuardState::Search));
        let low = man.pl.is_some_and(|p| g.pl[p].crouch);
        f.men.extend_from_slice(&[c.id as f32, c.pos[0] as f32, c.pos[2] as f32, c.aim_yaw as f32, ((man.down as u8) | ((roused as u8) << 1) | ((low as u8) << 2)) as f32]);
    }
    for i in 0..g.doors.list.len() {
        let d = g.doors.list[i];
        if crate::doors::door_off(g, d) { f.doors.extend_from_slice(&[f32::NAN, f32::NAN]); continue; }
        let u = crate::doors::door_dir(g, d);
        f.doors.extend_from_slice(&[u[0] as f32, u[1] as f32]);
    }
    g.recap.frames.push_back(f);
}

/// a death with the last seconds there are of it (None: no frame of them)
fn take_of(g: &Game, death: &RecapDeath) -> Option<RecapTake> {
    let k = &g.k.recap;
    let (t0, t1) = (death.t - k.secs, m::min(g.time, death.t + k.after));
    let frames: Vec<RecapFrame> = g.recap.frames.iter().filter(|f| f.t >= t0 - 1e-6 && f.t <= t1 + 1e-6).cloned().collect();
    if frames.is_empty() { return None; }
    Some(RecapTake {
        death: death.clone(), t0, t1, frames,
        cast: g.recap.cast.values().cloned().collect(),
        doors: g.doors.list.iter().map(|&d| RecapDoor { hinge: g.dr[d].def.hinge, width: g.dr[d].def.width }).collect(),
        shots: g.recap.shots.iter().filter(|s| s.t >= t0 && s.t <= t1).cloned().collect(),
        deaths: g.recap.deaths.iter().filter(|x| x.t >= t0 && x.t <= t1).cloned().collect(),
    })
}

/// a take onto a list of them. Past what is kept the oldest goes: a player's is never the one dropped while another's is there
fn file_take(takes: &mut Vec<RecapTake>, t: RecapTake, keep: f64) {
    takes.push(t);
    while takes.len() as f64 > m::max(1.0, keep) { let i = takes.iter().position(|x| !x.death.player).unwrap_or(0); takes.remove(i); }
}

/// this tick's rounds and blasts (the game forgets an event after half a second)
fn read_events(g: &mut Game) {
    use crate::director::EventKind;
    let seen = g.recap.seen_to;
    for i in 0..g.events.len() {
        let e = &g.events[i];
        if e.serial <= seen { continue; }
        g.recap.seen_to = g.recap.seen_to.max(e.serial);
        let shot = match e.kind {
            EventKind::Shot | EventKind::GuardShot => {
                let (Some(from), Some(to), Some(who)) = (e.from, e.to, e.who) else { continue; };
                let weapon = match e.weapon { Some(w) => g.m.weapons.weapons.get(w).map(crate::gear::plain_short).unwrap_or_default(), None => String::new() };
                RecapShot { t: e.time, kind: RecapShotKind::Round, who, from: xz(from), to: Some(xz(to)), at: Some(xz(e.pos)), weapon: Some(weapon), loud: Some(e.level.unwrap_or(0.0) >= 1.0) }
            }
            EventKind::Bang => RecapShot { t: e.time, kind: if e.frag.unwrap_or(false) { RecapShotKind::Blast } else { RecapShotKind::Bang }, who: e.who.unwrap_or(-1.0), from: xz(e.pos), to: None, at: None, weapon: None, loud: None },
            _ => continue,
        };
        g.recap.shots.push_back(shot);
    }
    let k = g.k.recap;
    let old = g.time - (k.secs + k.after + 1.0);
    while g.recap.shots.front().is_some_and(|s| s.t < old) { g.recap.shots.pop_front(); }
}

/// LAST IN THE TICK: this tick's rounds, who went down in it, a sample of the floor when one is due.
pub fn recap_tick(g: &mut Game) {
    this_run(g);
    g.recap.all_down = crate::seats::all_down(g);
    // (not watching: whoever goes down meanwhile is not a death seen, and the next look starts from the floor as it is)
    if !live(g) { g.recap.primed = false; return; }
    let now = g.time;
    g.recap.last_t = now;
    read_events(g);
    let floor = on_floor(g);
    for &man in &floor {
        let id = Num(g.ch[man.ch].id);
        if !g.recap.cast.has(id) { let rm = recap_man(g, man); g.recap.cast.set(id, rm); }
        if !man.down { g.recap.down.delete(id); continue; }
        if !g.recap.down.add(id) { continue; }
        if !g.recap.primed { continue; }   // (down before this record's first look: a body a checkpoint brought back is no death of this run)
        let Some(me) = g.recap.cast.get(id).cloned() else { continue; };
        if !ours(&me.side) { continue; }
        // the way he looked the sample before he fell
        let last_aim = g.recap.frames.back().and_then(|f| f.men.chunks_exact(MAN_STRIDE).find(|x| x[0] == id.0 as f32).map(|x| x[3] as f64));
        let gaze = last_aim.unwrap_or(g.ch[man.ch].aim_yaw);
        let d = explain(g, man, &me, &floor, gaze);
        // (put out cold by a hand, a civilian the player chokes: nobody was shot, there is nothing to explain)
        if let Some(gd) = man.gd { if matches!(g.gd[gd].down_kind, Some(DownKind::Struck | DownKind::Choked)) && d.cause != RecapCause::Round && d.cause != RecapCause::Blast { continue; } }
        g.recap.deaths.push_back(d.clone());
        if g.recap.deaths.len() > DEATHS_KEPT { g.recap.deaths.pop_front(); }
        let after = g.k.recap.after;
        g.recap.due.push(RecapDue { death: d.clone(), at: now + after });
        g.stats.deathrecap.deaths += 1;
        if let Some(gd) = man.gd { if g.gd[gd].unit.is_some() && me.side == "operative" { squad_lost(g, &d); } }
    }
    if !g.recap.lost.is_empty() && now >= g.recap.lost_next { g.recap.lost_next = now + WITNESS_SECS; witness(g); }
    g.recap.primed = true;
    if g.recap.over_at == f64::INFINITY && recap_over(g) { g.recap.over_at = now; }
    if now >= g.recap.next {
        let (next, hz) = (g.recap.next, g.k.recap.hz);
        g.recap.next = if now - next > 1.0 { now + 1.0 / hz } else { next + 1.0 / hz };
        sample(g, &floor);
    }
    file_ripe(g);
}

/// the deaths whose last second has run get their take
fn file_ripe(g: &mut Game) {
    let mut i = 0;
    while i < g.recap.due.len() {
        if g.time < g.recap.due[i].at { i += 1; continue; }
        let death = g.recap.due.remove(i).death;
        if let Some(t) = take_of(g, &death) { let keep = g.k.recap.keep; file_take(&mut g.recap.takes, t, keep); g.stats.deathrecap.takes += 1; }
    }
}

/// a man of the squad lost: what the squad may know of it while the run is on
fn squad_lost(g: &mut Game, d: &RecapDeath) {
    let src = d.from.or(d.by_at).and_then(|s| eighth(s[0] - d.at[0], s[1] - d.at[1]));
    let heard_it = d.loud && match d.from { Some(f) => heard(g, [f[0], 1.3, f[1]]), None => false };
    g.recap.lost_live.push(LostMan { id: d.victim, t: d.t, at: d.at, from: None, known_at: f64::INFINITY });
    g.recap.lost.push(Lost { src, heard: heard_it, saw: None, players_saw: Vec::new(), done: false });
    g.recap.lost_next = 0.0;
}

/// a way on the plan to the nearest eighth of the compass (None: no way at all)
fn eighth(dx: f64, dz: f64) -> Option<[f64; 2]> {
    if m::hyp2(dx, dz) <= 0.05 { return None; }
    let a = m::round(m::atan2(dx, dz) / (m::PI / 4.0)) * (m::PI / 4.0);
    Some([m::sin(a), m::cos(a)])
}

/// a line from a man still up to one going down (the room fight's own 'man down' test, with a reach): blind men see nothing
fn line_to(g: &mut Game, c: CharId, at: [f64; 2]) -> bool {
    let p = g.ch[c].pos;
    if m::hyp2(p[0] - at[0], p[2] - at[1]) > g.k.recap.live_see_m { return false; }
    let h = head_of(g, c);
    !crate::scene_collision::segment_blocked(g, h, [at[0], 0.4, at[1]], None, None, None)
}

/// could these ears have it: the sound model's own rule (a clear line, or over the furniture, else the walk round at the path's length, a shut leaf across it
/// muffling it hard), without its cache
fn ear(g: &mut Game, c: CharId, a: V3) -> bool {
    let reach = g.k.recap.live_hear_m;
    let h = head_of(g, c);
    let b = [h[0], m::max(h[1], 0.9), h[2]];
    let straight = m::hyp3(a[0] - b[0], a[1] - b[1], a[2] - b[2]);
    if straight > reach { return false; }
    if !crate::scene_collision::solid_blocked(g, a, b, None) { return true; }
    if !crate::scene_collision::solid_blocked(g, [a[0], 2.0, a[2]], [b[0], 2.0, b[2]], None) { return straight <= reach * 0.9; }
    if straight > EARS_PATH_M { return false; }
    let Some(path) = crate::scene_collision::nav_grid_find_path(g, b, a, Some(6000.0), None) else { return false; };
    if path.is_empty() { return false; }
    let seal = crate::scene_boxes::box_flag().sound_seal;
    let (mut len, mut door, mut pa) = (0.0, false, [b[0], 1.0, b[2]]);
    for q in path {
        let pb = [q[0], 1.0, q[2]];
        len += m::hyp2(pb[0] - pa[0], pb[2] - pa[2]);
        if !door && crate::scene_collision::segment_blocked_dynamic(g, pa, pb, Some(seal)) { door = true; }
        pa = pb;
    }
    let path_len = m::max(len + m::hyp2(a[0] - pa[0], a[2] - pa[2]), straight);
    path_len <= reach * (if door { 0.32 } else { 0.72 })
}

/// Did anybody of the squad still up hear it.
fn heard(g: &mut Game, src: V3) -> bool {
    let a = [src[0], m::max(src[1], 0.9), src[2]];
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if squad_up(g, gd) { let c = g.gd[gd].ch; if ear(g, c, a) { return true; } }
    }
    for pl in crate::seats::men_up(g) { let c = g.pl[pl].ch; if ear(g, c, a) { return true; } }
    false
}

/// did this player's man see him go down: a line, and where the fog is a cone (hard) inside the wedge he looks down or at his shoulder
fn player_sees(g: &mut Game, pl: PlayerId, at: [f64; 2]) -> bool {
    let c = g.pl[pl].ch;
    if g.pl[pl].down || crate::condition::is_blind(g, g.pl[pl].ch) || !line_to(g, c, at) { return false; }
    if !g.k.fog.cone { return true; }
    let p = g.ch[c].pos;
    let (dx, dz) = (at[0] - p[0], at[1] - p[2]);
    let d = m::hyp2(dx, dz);
    if d <= g.k.fog.cone_near_m { return true; }
    let u = crate::fogofwar::gaze_of(g, c);
    (dx * u[0] + dz * u[2]) / d >= m::cos(g.k.fog.cone_deg * m::PI / 360.0)
}

/// the side of the player's man a way lies on, in the words of the fog's own calls (asked of a point a long way off along it)
fn side_word(g: &Game, pl: PlayerId, u: [f64; 2]) -> String {
    use crate::close::CloseId;
    let p = g.ch[g.pl[pl].ch].pos;
    match crate::fogofwar::fog_of_war_side_of(g, [p[0] + u[0] * 50.0, 0.0, p[2] + u[1] * 50.0]) {
        CloseId::Front => String::from("from the front"),
        CloseId::Behind => String::from("from behind us"),
        side => format!("from the {}", side.as_str()),
    }
}

/// Each man lost and not yet settled: who has a line to him inside his first second down; after the beat, if somebody saw or ears had it, the squad has the way it came
/// from, and the teammate who saw says which side, when no player saw it himself and the player stands near enough for 'left' to mean anything. Nobody saw, nobody heard:
/// he is only down.
fn witness(g: &mut Game) {
    let (k, now) = (g.k.recap, g.time);
    let up = crate::seats::men_up(g);
    let pl = match up.first() { Some(&p) => p, None => crate::seats::men(g)[0] };   // (the player the side is called to: the first one up)
    let men = crate::seats::men(g);
    for i in 0..g.recap.lost.len().min(g.recap.lost_live.len()) {
        if g.recap.lost[i].done { continue; }
        let (t, at) = (g.recap.lost_live[i].t, g.recap.lost_live[i].at);
        let age = now - t;
        if age <= k.live_saw_secs {
            if g.recap.lost[i].saw.is_none() {
                for j in 0..g.guards.len() {
                    let gd = g.guards[j];
                    if !squad_up(g, gd) || crate::condition::is_blind(g, g.gd[gd].ch) { continue; }
                    let c = g.gd[gd].ch;
                    if line_to(g, c, at) { g.recap.lost[i].saw = Some(gd); break; }
                }
            }
            for &p in &men { if !g.recap.lost[i].players_saw.contains(&p) && player_sees(g, p, at) { g.recap.lost[i].players_saw.push(p); } }
        }
        if age < k.live_beat { continue; }
        let by = g.recap.lost[i].saw.filter(|&gd| squad_up(g, gd));   // (a witness who has gone down himself since tells nobody)
        let pl_down = g.pl[pl].down;
        let player_saw = g.recap.lost[i].players_saw.iter().any(|&p| !g.pl[p].down);   // (…and a player who has gone down since tells nobody either)
        if by.is_none() && !player_saw && !g.recap.lost[i].heard {
            if age > k.live_saw_secs { g.recap.lost[i].done = true; }
            continue;
        }
        let src = g.recap.lost[i].src;
        g.recap.lost[i].done = true;
        g.recap.lost_live[i].known_at = now;
        g.recap.lost_live[i].from = src;
        g.stats.deathrecap.known += 1;
        let (Some(u), Some(by)) = (src, by) else { continue; };
        if player_saw || pl_down { continue; }
        let p = g.ch[g.pl[pl].ch].pos;
        if m::hyp2(p[0] - at[0], p[2] - at[1]) > k.live_side_m { continue; }
        // (no cue of the cues' own is up over him: a contact call outranks this; his own 'man down.' is followed by it)
        let shown = crate::cues::cues_bubble_of(g, by);
        let his = g.gd[by].bubble.as_ref();
        let same = match (&shown, his) { (None, None) => true, (Some(a), Some(b)) => a.t == b.t && a.text == b.text, _ => false };
        if !same { continue; }
        let word = format!("{}!", side_word(g, pl, u));
        crate::cues::cues_aside(g, by, &word);
        g.stats.deathrecap.calls += 1;
    }
}

/// The run is over for everybody: nobody is left who could be told something he should not know. Every player down, or the mission ended either way.
fn recap_over(g: &Game) -> bool {
    crate::seats::all_down(g) || matches!(g.mission.stage, MissionStage::Failed | MissionStage::Done)
}

/// What the end card may show: None while anybody still plays or when nobody of ours went down. The card can come up before a death's last second has run: such a death
/// is shown with what there is of it. A run played on past its end (out with the drive and 'keep playing'; a mission lost with a player up) shows what happened up to
/// the end, until every player is down.
pub fn recap_card(g: &Game) -> Option<DeathRecap> {
    if !recap_over(g) || g.recap.run != Some(g.mission.serial) { return None; }
    let mut takes = g.recap.takes.clone();
    for due in g.recap.due.iter() { if let Some(t) = take_of(g, &due.death) { file_take(&mut takes, t, g.k.recap.keep); } }
    if !crate::seats::all_down(g) { let end = g.recap.over_at + 1e-6; takes.retain(|t| t.death.t <= end); }
    let main = takes.iter().rposition(|t| t.death.player).or(takes.len().checked_sub(1))?;
    Some(DeathRecap { takes, main: main as f64 })
}
