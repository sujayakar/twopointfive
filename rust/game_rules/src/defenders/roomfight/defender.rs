//! A DEFENDER OF A ROOM: his evidence of them, what he watches, how he reacts, where he goes and how he holds it.
#![allow(unused_imports)]
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// `FIGHT_SKILL.hostile`: `readonly skill = FIGHT_SKILL`, so g.fight.skill is the one table
pub(super) fn hostile_skill(g: &Game) -> FightSkill { *g.fight.skill.get(RoomFightSkillKey::Hostile).unwrap() }

pub(super) fn entries_of(g: &Game, room: Option<RoomfightRoomId>) -> Vec<Entry> { match room { Some(r) => g.tab.roomfight_room[r].entries.clone(), None => Vec::new() } }

/// `D.ev[i]`: a room can gain entries after the row was dealt (a merge), and JavaScript reads past the end as undefined: NaN compares and sums the same
pub(super) fn ev_at(g: &Game, d: RoomfightDefId, i: usize) -> f64 { g.tab.roomfight_def[d].ev.get(i).copied().unwrap_or(f64::NAN) }

/// `D.ev[i] = v`, past the end as JavaScript writes it: the holes read back as undefined
pub(super) fn ev_set(g: &mut Game, d: RoomfightDefId, i: usize, v: f64) { let ev = &mut g.tab.roomfight_def[d].ev; while ev.len() <= i { ev.push(f64::NAN); } ev[i] = v; }

/// His watched point moves: what his duty has his gun on moves with it. His fix does not: where he thinks his man is, is his own.
pub(super) fn move_pt(g: &mut Game, d: RoomfightDefId, to: V3) { crate::roomfight::def_move_pt(g, d, to); }

/// His duty has his gun on his watched point.
pub(super) fn cover_pt(g: &mut Game, d: RoomfightDefId) {
    let (gd, pt) = (g.tab.roomfight_def[d].gd, g.tab.roomfight_def[d].pt);
    crate::belief::cover(g, gd, pt);
}

/// `DIGIN.on || squadsLive(g)`
pub(super) fn digs_in(g: &Game) -> bool { g.k.digin.on || crate::enemysquads::squads_live(g) }

pub(super) fn room_fight_adopt(g: &mut crate::game::Game, gd: GuardId) -> RoomfightDefId {
    let c = g.gd[gd].ch; let s = hostile_skill(g);
    let f = crate::character::character_forward(g, c); let p = g.ch[c].pos;
    let room = room_fight_room_at(g, p);
    let ev = match room { Some(r) => vec![0.0; g.tab.roomfight_room[r].entries.len()], None => Vec::new() };
    let front: V3 = [p[0] + f[0] * 6.0, 0.0, p[2] + f[2] * 6.0];
    let glance_next = g.time + pick(g, crate::dice::Who::Man(gd), s.hold);
    let nerve = pick(g, crate::dice::Who::Man(gd), s.nerve);
    let H = g.gd[gd].witness.heard;
    let row = Def {
        gd, pinned0: g.gd[gd].pinned, station: p, home: front, room, staged: None, ev, watch: -1.0, want: -1.0, want_at: 0.0,
        glance_to: -1.0, glance_until: -1.0, glance_next, pt: front,
        heard_t: H.last_t, cnt: DefCnt { shots: H.shots, kick: H.kick, bang: H.bang, small: H.small }, noise_at: None, noise_t: -100.0, noise_out: false,
        last_state: String::from(g.gd[gd].state.as_str()), last_seen: g.gd[gd].last_seen_t, fix_keep: -1.0, pre: false, nerve, lost_at: 0.0, moved_at: -100.0,
        mode: DefMode::Hold, move_kind: DefMoveKind::Cover, move_t: 0.0, leg_by: crate::owner::By::NOBODY, came_by: None, stall_at: p, stall_t: 0.0, free_at: 0.0,
        burst: DefBurst { until: -1.0, n: 0.0, next: 0.0, broke: false, daze: 0.0 }, bolt_until: -1.0, bolt_from: None,
    };
    g.tab.roomfight_def.push(row)
}

pub(super) fn room_fight_tick_def(g: &mut crate::game::Game, d: RoomfightDefId, dt: f64, k: f64) {
    let gd = g.tab.roomfight_def[d].gd; let base = hostile_skill(g);
    g.gd[gd].skill = Some(crate::game::GuardSkill { settle_k: base.settle_k, spread_k: base.spread_k });
    if g.gd[gd].held.is_some() { room_fight_blind(g, d, k); return; }
    if crate::owner::has(g, gd, crate::owner::by::MORALE) { return; }
    if crate::owner::has(g, gd, crate::owner::by::RUNNER) { return; }
    if digs_in(g) {
        if g.level.digs_in() && !crate::lanes::lane_throws(g, gd) { crate::digin::dig_in_tick(g, d, dt); }
    }
    let mode = g.tab.roomfight_def[d].mode;
    if mode == DefMode::Move { room_fight_tick_move(g, d, dt); return; }
    if mode == DefMode::Free { room_fight_tick_free(g, d); return; }
    if g.gd[gd].script.is_some() && crate::director::director_lesson_runs(g) { g.tab.roomfight_def[d].staged = Some(true); return; }
    if g.tab.roomfight_def[d].staged == Some(true) {
        g.tab.roomfight_def[d].staged = Some(false); crate::control::set_hold(g, gd, true, "roomfight");
        if v3::dist_xz(pos_of(g, gd), g.tab.roomfight_def[d].station) > 1.5 {
            g.fight.defs.retain(|&x| x != d);
            return;
        }
    }
    if g.gd[gd].script.is_some() {
        let st = match g.gd[gd].state { crate::game::GuardState::Dead | crate::game::GuardState::Patrol => crate::squad::EndGuardScriptState::Patrol, crate::game::GuardState::Suspicious => crate::squad::EndGuardScriptState::Suspicious, crate::game::GuardState::Alert => crate::squad::EndGuardScriptState::Alert, crate::game::GuardState::Search => crate::squad::EndGuardScriptState::Search };
        crate::squad::end_guard_script(g, gd, Some(st));
    }
    if !g.gd[gd].hold {
        g.gd[gd].skill = None; g.fight.defs.retain(|&x| x != d);
        return;
    }
    crate::control::set_pinned(g, gd, true, "roomfight");
    let dazzled = crate::control::man::blind(g, gd);
    room_fight_evidence(g, d, dt);
    if !dazzled { room_fight_choose_watch(g, d); room_fight_apply_watch(g, d, dt); }
    room_fight_reactions(g, d);
    room_fight_blind(g, d, k);
    if dazzled { return; }
    if let Some((rule, leave)) = crate::control::grant_post(g, gd, dt) {
        match leave {
            crate::control::LeavePost::On(leg) => send(g, d, leg, crate::owner::By::NOBODY),
            crate::control::LeavePost::Loose => loose(g, gd, crate::control::rule_name(rule)),
        }
        crate::control::commit(g, gd, rule, crate::control::Outcome::Left);
    }
}

/// what his own senses have put at each entry since last frame.
fn room_fight_evidence(g: &mut crate::game::Game, d: RoomfightDefId, dt: f64) {
    let skill = hostile_skill(g);
    let gd = g.tab.roomfight_def[d].gd; let H = g.gd[gd].witness.heard;
    let room = g.tab.roomfight_def[d].room; let ent = entries_of(g, room);
    let k = m::exp(-dt / g.k.roomfight.evidence_secs * m::log(3.0));
    for v in g.tab.roomfight_def[d].ev.iter_mut() { *v *= k; }
    if H.last_t > g.tab.roomfight_def[d].heard_t {
        let cnt = g.tab.roomfight_def[d].cnt;
        let loud = H.shots > cnt.shots || H.kick > cnt.kick || H.bang > cnt.bang;
        let fix = H.loud_at;
        let at = if loud { fix.or(g.tab.roomfight_def[d].noise_at) } else if H.small > cnt.small && H.small_where.is_some() { H.small_where } else { fix };
        if let Some(at) = at {
            let e = room_fight_entry_toward(g, d, at);
            if e >= 0.0 { let i = e as usize; let v = m::min(3.0, ev_at(g, d, i) + (if loud { 1.0 } else { 0.45 })); ev_set(g, d, i, v); }
            if loud {
                let out = !room_fight_in_room(g, room, at); let now = g.time; let D = &mut g.tab.roomfight_def[d];
                D.noise_at = Some(at); D.noise_t = now; D.noise_out = out; D.glance_until = -1.0;
            }
            if H.kick > cnt.kick && e >= 0.0 && flat(ent[e as usize].at, at) < 2.5 && !crate::control::man::blind(g, gd) {
                let door = ent[e as usize].door;
                if skill.startle[1] > 0.0 && !(digs_in(g) && crate::digin::dig_in_braced(g, gd, door)) {
                    let p = pick(g, crate::dice::Who::Def(d), skill.startle);
                    let t = p * (if room_fight_drilled(g) { g.k.drilled.shaken } else { 1.0 });
                    g.gd[gd].react_t = m::max(g.gd[gd].react_t, t); g.fight.stats.startled += 1.0;
                    tell_news(g, News::Man(gd, Did::Flinches));
                }
            }
        }
        let D = &mut g.tab.roomfight_def[d];
        D.heard_t = H.last_t; D.cnt = DefCnt { shots: H.shots, kick: H.kick, bang: H.bang, small: H.small };
    }
    if g.time - g.gd[gd].last_seen_t < 0.3 {
        if let Some(seen) = crate::belief::eyes_fix_def(g, gd, d) {
            let e = room_fight_entry_toward(g, d, seen);
            if e >= 0.0 { let i = e as usize; let v = m::min(3.0, ev_at(g, d, i) + 1.2 * dt); ev_set(g, d, i, v); }
        }
    }
}

/// a man of his own side has told him where they are. False = no post of this file's.
pub fn room_fight_told(g: &mut crate::game::Game, gd: GuardId, at: V3) -> bool {
    let d = match def_of(g, gd) { Some(d) => d, None => return false };
    let e = room_fight_entry_toward(g, d, at);
    if e >= 0.0 { let i = e as usize; let v = m::min(3.0, ev_at(g, d, i) + 1.0); ev_set(g, d, i, v); }
    let room = g.tab.roomfight_def[d].room; let out = !room_fight_in_room(g, room, at); let now = g.time;
    let D = &mut g.tab.roomfight_def[d];
    D.noise_at = Some(at); D.noise_t = now; D.noise_out = out; D.glance_until = -1.0;
    crate::belief::word(g, gd, at);
    true
}

/// the entry nearest a point, or -1.
fn room_fight_entry_toward(g: &crate::game::Game, d: RoomfightDefId, at: V3) -> f64 {
    let (mut best, mut bd) = (-1.0, f64::INFINITY);
    for (i, e) in entries_of(g, g.tab.roomfight_def[d].room).iter().enumerate() { let q = flat(e.at, at); if q < bd { bd = q; best = i as f64; } }
    best
}

fn room_fight_choose_watch(g: &mut crate::game::Game, d: RoomfightDefId) {
    let gd = g.tab.roomfight_def[d].gd; let s = hostile_skill(g);
    let room = g.tab.roomfight_def[d].room; let ent = entries_of(g, room);
    let (mut best, mut bs) = (-1.0, g.k.roomfight.fear_min);
    let c = g.gd[gd].ch; let eye = eye_of(g, c);
    for (i, e) in ent.iter().enumerate() {
        let n = i as f64;
        if ev_at(g, d, i) <= 0.02 { continue; }
        let view = if seg_blocked(g, eye, [e.at[0] - e.out[0] * 0.5, 1.2, e.at[2] - e.out[1] * 0.5]) { 0.6 } else { 1.0 };
        let mut mates = 0.0;
        for &x in &g.fight.defs { let X = &g.tab.roomfight_def[x]; if x != d && !crate::condition::down(g, g.gd[X.gd].ch) && X.room == room && X.mode == DefMode::Hold && X.watch == n { mates += 1.0; } }
        let sc = ev_at(g, d, i) * view - g.k.roomfight.crowd * mates + (if n == g.tab.roomfight_def[d].watch { g.k.roomfight.sticky } else { 0.0 });
        if sc > bs { bs = sc; best = n; }
    }
    if best != g.tab.roomfight_def[d].want {
        let at = g.time + pick(g, crate::dice::Who::Def(d), s.shift); let D = &mut g.tab.roomfight_def[d];
        D.want = best; D.want_at = at;
    }
    if g.tab.roomfight_def[d].want != g.tab.roomfight_def[d].watch && g.time >= g.tab.roomfight_def[d].want_at {
        let w = g.tab.roomfight_def[d].want; g.tab.roomfight_def[d].watch = w;
    }
    if !ent.is_empty() && g.time >= g.tab.roomfight_def[d].glance_next && g.time - g.gd[gd].last_seen_t > 2.0 && g.time - g.tab.roomfight_def[d].noise_t > 3.0 && !crate::enemysquads::squad_drill(g).map_or(false, |k| k.drill_angles) {
        let watch = g.tab.roomfight_def[d].watch;
        let mut others: Vec<f64> = (0..ent.len()).map(|i| i as f64).filter(|&i| i != watch).collect();
        if watch >= 0.0 { others.push(-1.0); }
        let r = crate::dice::draw_def(g, "roomfight::room_fight_choose_watch#1", d);
        let to = others.get(m::floor(r * others.len() as f64) as usize).copied().unwrap_or(-1.0);
        let until = g.time + pick(g, crate::dice::Who::Def(d), s.glance);
        let next = until + pick(g, crate::dice::Who::Def(d), s.hold);
        let D = &mut g.tab.roomfight_def[d];
        D.glance_to = to; D.glance_until = until; D.glance_next = next;
    }
}

/// what he watches becomes where he faces and the fix his gun covers.
fn room_fight_apply_watch(g: &mut crate::game::Game, d: RoomfightDefId, dt: f64) {
    let skill = hostile_skill(g);
    let gd = g.tab.roomfight_def[d].gd; let c = g.gd[gd].ch;
    let room = g.tab.roomfight_def[d].room; let ent = entries_of(g, room);
    let e = if g.time < g.tab.roomfight_def[d].glance_until { g.tab.roomfight_def[d].glance_to } else { g.tab.roomfight_def[d].watch };
    let tgt = if e >= 0.0 && (e as usize) < ent.len() { ent[e as usize].at } else { g.tab.roomfight_def[d].home };
    move_pt(g, d, [tgt[0], 0.0, tgt[2]]);
    // (each look writes into D.pt in place: lent as a local and put back, the fix carried with it if it is that object)
    if digs_in(g) { let mut p = g.tab.roomfight_def[d].pt; crate::digin::dig_in_look(g, gd, &mut p); move_pt(g, d, p); }
    if let Some(at) = crate::mission::gun_place(g, gd) { let p = g.tab.roomfight_def[d].pt; move_pt(g, d, [at[0], p[1], at[2]]); }
    { let mut p = g.tab.roomfight_def[d].pt; crate::netquiet::net_quiet_look(g, gd, &mut p); move_pt(g, d, p); }
    { let mut p = g.tab.roomfight_def[d].pt; crate::wallreact::wall_react_look_2(g, gd, &mut p); move_pt(g, d, p); }
    let pt = g.tab.roomfight_def[d].pt;
    if g.gd[gd].state == crate::game::GuardState::Patrol {
        let want = crate::character::character_want_body(g, c);
        g.ch[c].body_yaw = v3::approach_angle(want, v3::yaw_to(g.ch[c].pos, pt), skill.turn * dt);
        return;
    }
    if g.time - g.gd[gd].last_seen_t < 0.6 { crate::belief::uncover(g, gd); return; }
    if let Some(seen) = crate::belief::eyes_fix_def(g, gd, d) {
        if g.time - g.gd[gd].sight_t < 1.0 && room_fight_in_room(g, room, seen) { g.tab.roomfight_def[d].fix_keep = g.time + 3.0; }
    }
    if g.time >= g.tab.roomfight_def[d].fix_keep { cover_pt(g, d); } else { crate::belief::uncover(g, gd); }
}

/// the untrained man's beats.
fn room_fight_reactions(g: &mut crate::game::Game, d: RoomfightDefId) {
    let gd = g.tab.roomfight_def[d].gd; let c = g.gd[gd].ch; let s = hostile_skill(g);
    if g.gd[gd].state == crate::game::GuardState::Alert && g.tab.roomfight_def[d].last_state != "alert" { let p = pick(g, crate::dice::Who::Def(d), s.react); g.gd[gd].react_t = m::max(g.gd[gd].react_t, p); }
    let fc = g.gd[gd].foe;
    if let Some(fc) = fc { if g.gd[gd].last_seen_t > g.tab.roomfight_def[d].last_seen + 1.5 && g.time - g.gd[gd].last_seen_t < 0.1 {
        let (cp, fp, pt) = (g.ch[c].pos, g.ch[fc].pos, g.tab.roomfight_def[d].pt);
        let off = m::abs(v3::wrap_angle(v3::yaw_to(cp, fp) - v3::yaw_to(cp, pt)));
        let B = g.tab.roomfight_def[d].burst;
        let dazed = g.time < B.until + B.daze;
        let caught = dazed || off > s.sector_deg * v3::DEG; let lim = g.k.roomfight.preaim_deg * v3::DEG;
        let a = pick(g, crate::dice::Who::Def(d), s.react) * (if caught { 1.0 } else { s.ready });
        let b = if caught { pick(g, crate::dice::Who::Def(d), s.surprise) } else { 0.0 };
        g.gd[gd].react_t = m::max(g.gd[gd].react_t, a + b);
        g.tab.roomfight_def[d].pre = !dazed && g.gd[gd].state != crate::game::GuardState::Patrol && off < lim && m::abs(v3::wrap_angle(v3::yaw_to(cp, fp) - g.ch[c].aim_yaw)) < lim;
        if caught {
            g.fight.stats.surprised += 1.0;
            if !dazed { tell_news(g, News::Man(gd, Did::Surprised)); }
        }
    } }
    if g.tab.roomfight_def[d].pre {
        if g.time - g.gd[gd].last_seen_t > 1.0 { g.tab.roomfight_def[d].pre = false; }
        else if g.gd[gd].state == crate::game::GuardState::Alert && g.gd[gd].react_t <= 0.0 {
            g.arms[g.gd[gd].ch].aim.track = m::max(g.arms[g.gd[gd].ch].aim.track, s.preaim); g.arms[g.gd[gd].ch].fire_cd = m::min(g.arms[g.gd[gd].ch].fire_cd, 0.0); g.tab.roomfight_def[d].pre = false;
        }
    }
    let (seen, st) = (g.gd[gd].last_seen_t, g.gd[gd].state);
    let D = &mut g.tab.roomfight_def[d];
    D.last_seen = seen; D.last_state = String::from(st.as_str());
}

/// a dazzled defender empties a burst down what he was watching, or ducks.
fn room_fight_blind(g: &mut crate::game::Game, d: RoomfightDefId, k: f64) {
    let h = g.tab.roomfight_def[d].gd; let s = hostile_skill(g);
    if !crate::control::man::blind(g, h) { return; }
    let bl = crate::nightbreach::blind();
    if g.cond[g.gd[h].ch].blind_until > g.tab.roomfight_def[d].burst.until + 1e-6 {
        let (until, next) = (g.cond[g.gd[h].ch].blind_until, g.time + bl.after + k * bl.stagger);
        let broke = crate::dice::draw_def(g, "roomfight::room_fight_blind#1", d) < s.bang_break;
        let daze = if s.daze[1] > 0.0 { let p = pick(g, crate::dice::Who::Def(d), s.daze); p * (if room_fight_drilled(g) { g.k.drilled.shaken } else { 1.0 }) } else { 0.0 };
        let B = &mut g.tab.roomfight_def[d].burst;
        B.until = until; B.n = 0.0; B.next = next; B.broke = broke; B.daze = daze;
        if broke { g.fight.stats.broke += 1.0; }
    }
    if g.gd[h].held.is_none() { cover_pt(g, d); }
    if g.tab.roomfight_def[d].burst.broke {
        if g.tune.suppression { g.cond[g.gd[h].ch].sup.down_t = m::max(g.cond[g.gd[h].ch].sup.down_t, 0.2); }
        return;
    }
    if !g.fight.blind_fire || g.tab.roomfight_def[d].burst.n >= bl.rounds || g.time < g.tab.roomfight_def[d].burst.next { return; }
    g.tab.roomfight_def[d].burst.n += 1.0;
    let next = g.time + crate::gunroles::gun_blind_gap(g, h, bl.every);
    g.tab.roomfight_def[d].burst.next = next;
    let c = g.gd[h].ch;
    if g.gd[h].held.is_some() || crate::control::man::staggered(g, h) || g.ch[c].gun_dir[1] < -bl.low_gun { return; }
    if g.tune.suppression && (g.cond[g.gd[h].ch].sup.v >= g.k.suppression.pinned || g.cond[g.gd[h].ch].sup.latched || g.cond[g.gd[h].ch].sup.down_t > 0.0) { return; }
    let gun = g.gd[h].weapon;
    if !crate::weapons::firearm_fire(g, gun) { return; }
    let at = v3::mad(g.ch[c].muzzle, g.ch[c].gun_dir, 8.0);
    let o = crate::combat::FireOpts { steady: Some(true), spread: Some(bl.spread), blind: Some(true), ..Default::default() };
    crate::combat::fire_weapon(g, c, at, Some(&o));
    g.fight.blind_rounds += 1.0;
    if !g.fight.drill_said && room_fight_drilled(g) {
        g.fight.drill_said = true;
        let line = crate::words::wg(g, &"can't see \u{2014} shoot through the doorway!", &"can't see \u{2014} put it down the door!");
        crate::game::game_say(g, h, line, None, None);
    }
}

fn room_fight_drilled(g: &crate::game::Game) -> bool { drilled_on(g) }

/// is the room lost, as far as he can tell.
pub fn room_fight_room_lost(g: &mut crate::game::Game, d: RoomfightDefId) -> bool {
    let gd = g.tab.roomfight_def[d].gd; let c = g.gd[gd].ch;
    if g.gd[gd].state != crate::game::GuardState::Alert || g.time < g.tab.roomfight_def[d].lost_at || g.time - g.tab.roomfight_def[d].moved_at < 6.0 { return false; }
    g.tab.roomfight_def[d].lost_at = g.time + 0.3;
    let eye = eye_of(g, c); let (mut dead, mut up) = (0.0, 0.0);
    let room = g.tab.roomfight_def[d].room;
    for x in g.fight.defs.clone() {
        let (xr, xg, xm) = { let X = &g.tab.roomfight_def[x]; (X.room, X.gd, X.mode) };
        if xr != room { continue; }
        if !crate::condition::down(g, g.gd[xg].ch) { if xm != DefMode::Free { up += 1.0; } continue; }
        let xc = g.gd[xg].ch;
        let b = g.ch[xc].bones.get(crate::character::Bone::Hips).copied().unwrap_or(g.ch[xc].pos);
        if seg_blocked(g, eye, [b[0], 0.3, b[2]]) { continue; }
        if flat(b, g.ch[c].pos) < 5.0 || !g.tune.body_light || crate::guards::body_visibility(g, xg) >= g.k.body_light.lit { dead += 1.0; }
    }
    let sup_v = g.cond[g.gd[gd].ch].sup.v;
    let pressed = g.time - g.gd[gd].last_seen_t < 4.0 || (g.tune.suppression && sup_v >= g.k.suppression.flinch) || g.time - g.gd[gd].witness.heard.last_shot_t < 2.5;
    let frac = dead / m::max(1.0, dead + up);
    let alone = dead + up <= 1.0 && g.tune.suppression && g.cond[g.gd[gd].ch].sup.latched && g.cond[g.gd[gd].ch].sup.pinned_t > 1.2;
    pressed && ((dead > 0.0 && frac >= g.tab.roomfight_def[d].nerve) || alone)
}

/// where the trouble is, as far as he knows.
fn room_fight_threat_of(g: &crate::game::Game, d: RoomfightDefId) -> V3 {
    let gd = g.tab.roomfight_def[d].gd;
    if g.time - g.gd[gd].sight_t < 6.0 { return g.gd[gd].sight_pos; }
    if let Some(src) = g.cond[g.gd[gd].ch].sup.src { if g.time - g.cond[g.gd[gd].ch].sup.src_t < g.k.suppression.src_memory_secs { return src; } }
    let D = &g.tab.roomfight_def[d];
    let (mut bi, mut bv) = (-1i32, 0.3);
    for (i, &v) in D.ev.iter().enumerate() { if v > bv { bv = v; bi = i as i32; } }
    match D.room { Some(r) if bi >= 0 => g.tab.roomfight_room[r].entries[bi as usize].at, _ => D.pt }
}

/// a man on a post caught unaware and alone goes as a man whose nerve has gone. None = no post of this file's.
pub fn room_fight_bolt(g: &mut crate::game::Game, gd: GuardId, clear_secs: f64) -> Option<bool> {
    let d = def_of(g, gd)?;
    if !crate::roomfight::at_post(g, d) || crate::condition::down(g, g.gd[gd].ch) || g.gd[gd].held.is_some() { return Some(false); }
    let (moved, out, pic) = (g.tab.roomfight_def[d].moved_at, g.fight.stats.fall_back_out, g.stats.picture.moved);
    let Some(way) = break_off_way(g, d) else {
        g.tab.roomfight_def[d].moved_at = moved; g.fight.stats.fall_back_out = out; g.stats.picture.moved = pic;   // (nowhere to go: not a try)
        return Some(false);
    };
    let leg = way.leg(g);
    send(g, d, leg, crate::owner::By::NOBODY);
    break_off_said(g, d, way);
    let (until, foe) = (g.time + clear_secs, g.gd[gd].foe);
    let D = &mut g.tab.roomfight_def[d];
    D.bolt_until = until; D.bolt_from = foe;
    Some(true)
}

/// a believed man beside a straight leg of the walk
pub(super) fn near_leg(a: V3, b: V3, k: V3) -> bool {
    let (dx, dz) = (b[0] - a[0], b[2] - a[2]); let d2 = dx * dx + dz * dz;
    let t = if d2 > 0.0 { m::max(0.0, m::min(1.0, ((k[0] - a[0]) * dx + (k[2] - a[2]) * dz) / d2)) } else { 0.0 };
    m::hyp2(k[0] - a[0] - dx * t, k[2] - a[2] - dz * t) < 2.0
}

/// the walk to a spot: its length, or None if it will not do
pub(super) fn leg_of(g: &mut Game, from: V3, threat: V3, known: &[V3], spot: V3, mind: bool) -> Option<f64> {
    let path = crate::scene_collision::nav_grid_find_path(g, from, spot, None, None)?; if path.is_empty() { return None; }
    let (mut l, mut q, mut toward) = (0.0, from, false);
    for &w in &path {
        if mind && known.iter().any(|&k| near_leg(q, w, k)) { toward = true; }
        l += flat(q, w); q = w;
        if flat(w, threat) < 2.0 { toward = true; }
    }
    if l > 18.0 || toward || flat(q, spot) > 0.8 { None } else { Some(l) }
}

#[derive(Clone, Copy)]
struct Way { spot: V3, door: DoorId, l: f64, safe: bool }

/// where a man who breaks off goes
#[derive(Clone, Copy, Debug, crate::bytes::ToBytes)]
pub enum WayOut { ToSquad(V3), By(DoorId, V3), ToCover(V3) }
impl WayOut {
    pub fn leg(self, g: &Game) -> Leg {
        let chase = g.tune.guard_chase;
        match self {
            WayOut::ToSquad(to) => Leg::to(to, chase + 0.3),
            WayOut::By(door, to) => Leg { by: Some(door), ..Leg::to(to, chase + 0.3) },
            WayOut::ToCover(to) => Leg { cover: true, ..Leg::to(to, chase) },
        }
    }
}

const BEYOND: [(f64, f64); 9] = [(3.0, 1.6), (3.0, -1.6), (2.4, 1.2), (2.4, -1.2), (1.2, 2.4), (1.2, -2.4), (1.2, 1.7), (1.2, -1.7), (2.6, 0.0)];

/// his way out: to his squad's rally point, by a door nothing has come at, or, no such door, to cover.
pub fn break_off_way(g: &mut crate::game::Game, d: RoomfightDefId) -> Option<WayOut> {
    let gd = g.tab.roomfight_def[d].gd; let c = g.gd[gd].ch; let threat = room_fight_threat_of(g, d);
    g.tab.roomfight_def[d].moved_at = g.time;
    if let Some(rp) = crate::enemysquads::squad_rally(g, gd, threat) { return Some(WayOut::ToSquad(rp)); }
    let mut asker = crate::picture::exposure_asker(g, gd, None);
    let ask = asker.is_some();
    let known: Vec<V3> = if ask { crate::picture::believed_of(g, gd) } else { Vec::new() };
    let (mut best, mut was): (Option<Way>, Option<Way>) = (None, None);
    let cp = g.ch[c].pos;
    let here = match &mut asker { Some(a) => crate::picture::exposure_ask(g, a, cp), None => 0.0 };
    let room = g.tab.roomfight_def[d].room;
    let ent = if g.gd[gd].stays_in_rooms { Vec::new() } else { entries_of(g, room) };
    for (i, e) in ent.iter().enumerate() {
        if ev_at(g, d, i) > 0.3 || flat(e.at, threat) < 3.0 || g.dr[e.door].def.steel == Some(true) || crate::doors::doors_locked_out(g, e.door, cp, None) { continue; }
        let f = crate::guards::frame_of(g, e.door);
        let (mut spot, mut safe): (Option<V3>, Option<V3>) = (None, None);
        for &(depth, lat) in BEYOND.iter() {
            let p: V3 = [e.at[0] + e.out[0] * depth + f.ux * lat * (1.0 + f.w / 3.2), 0.0, e.at[2] + e.out[1] * depth + f.uz * lat * (1.0 + f.w / 3.2)];
            if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) || room_fight_in_room(g, room, p) { continue; }
            if room_fight_out_of_building(g, p) { if spot.is_none() { g.fight.stats.fall_back_out += 1.0; } continue; }
            if g.fight.defs.iter().any(|&x| x != d && { let xg = g.tab.roomfight_def[x].gd; !crate::condition::down(g, g.gd[xg].ch) && flat(pos_of(g, xg), p) < 0.9 }) { continue; }
            if spot.is_none() { spot = Some(p); }
            let a = match &mut asker { Some(a) => a, None => break };
            if crate::picture::exposure_ask(g, a, p) <= 0.0 { safe = Some(p); break; }
        }
        let spot = match spot { Some(s) => s, None => continue };
        if let Some(l0) = leg_of(g, cp, threat, &known, spot, false) { if was.map_or(true, |w| l0 < w.l) { was = Some(Way { spot, door: e.door, l: l0, safe: false }); } }
        let a = match &mut asker { Some(a) => a, None => continue };
        let mouth = crate::picture::exposure_ask(g, a, [e.at[0] + e.out[0], 0.0, e.at[2] + e.out[1]]);
        if known.iter().any(|&k| flat(e.at, k) < 3.0) || (mouth > g.k.picture.maybe_k && mouth > here) { continue; }
        let mut sp = safe.unwrap_or(spot); let mut l1 = leg_of(g, cp, threat, &known, sp, true);
        if l1.is_none() && sp != spot { sp = spot; l1 = leg_of(g, cp, threat, &known, sp, true); }
        let hid = safe == Some(sp);
        if let Some(l1) = l1 { if best.map_or(true, |b| (hid && !b.safe) || (hid == b.safe && l1 < b.l)) { best = Some(Way { spot: sp, door: e.door, l: l1, safe: hid }); } }
    }
    if !ask { best = was; } else if best.map(|b| (b.door, b.spot)) != was.map(|w| (w.door, w.spot)) { g.stats.picture.moved += 1; }   // (the same object: the same door's spot, and one door's spots all differ)
    if let Some(b) = best { return Some(WayOut::By(b.door, b.spot)); }
    crate::suppression::cover_from(g, gd, threat).map(WayOut::ToCover)
}

/// what is said and counted of a man who has broken off
pub fn break_off_said(g: &mut crate::game::Game, d: RoomfightDefId, way: WayOut) {
    let gd = g.tab.roomfight_def[d].gd;
    match way {
        WayOut::ToSquad(_) => {
            g.fight.stats.fell_back += 1.0;
            tell_news(g, News::Man(gd, Did::FallsBack));
            crate::game::game_say(g, gd, "fall back \u{2014} back to the others!", None, None);
        }
        WayOut::By(..) => {
            g.fight.stats.fell_back += 1.0;
            tell_news(g, News::Man(gd, Did::FallsBack));
            let line = if crate::dice::draw_def(g, "roomfight::room_fight_break_off#1", d) < 0.5 { "fall back \u{2014} fall back!" } else { "they're in \u{2014} get out!" };
            crate::game::game_say(g, gd, line, None, None);
        }
        WayOut::ToCover(_) => {
            g.fight.stats.took_cover += 1.0;
            tell_news(g, News::Man(gd, Did::TakesCover));
            crate::game::game_say(g, gd, "cover \u{2014} get to cover!", None, None);
        }
    }
}

/// a defender on the move: an enemy seen close stops him to fight; there, or going nowhere, he plants.
fn room_fight_tick_move(g: &mut crate::game::Game, d: RoomfightDefId, dt: f64) {
    let gd = g.tab.roomfight_def[d].gd; let c = g.gd[gd].ch; let s = hostile_skill(g);
    let arrived0 = match &g.gd[gd].script { Some(x) => x.arrived.unwrap_or(false), None => { room_fight_plant(g, d, false); return; } };
    g.tab.roomfight_def[d].move_t += dt;
    // the man his eyes have (guards/senses), near enough to stop for; never the one he is getting away from
    let from = if g.time < g.tab.roomfight_def[d].bolt_until { g.tab.roomfight_def[d].bolt_from } else { None };
    let reach = g.k.roomfight.move_engage_m;
    let near = g.gd[gd].foe.map_or(false, |fc| v3::dist(g.ch[fc].pos, g.ch[c].pos) < reach);
    let contact = g.gd[gd].foe.filter(|&f| g.gd[gd].saw_foe_this_frame && near && !crate::control::man::blind(g, gd) && Some(f) != from);
    if let Some(ct) = contact {
        let fc = ct;
        crate::mind::foe(g, gd, Some(ct), "roomfight");
        crate::aim::aim_state_clear(&mut g.arms[g.gd[gd].ch].aim);
        room_fight_plant(g, d, true);
        crate::mind::set(g, gd, 1.0, "roomfight");
        g.gd[gd].react_t = pick(g, crate::dice::Who::Def(d), s.react);
        let fp = g.ch[fc].pos;
        crate::guards::eyes_on(g, gd, fp);
        return;
    }
    let cp = g.ch[c].pos;
    let D = &mut g.tab.roomfight_def[d];
    if flat(cp, D.stall_at) > 0.4 { D.stall_at = cp; D.stall_t = 0.0; } else { D.stall_t += dt; }
    let (stall, moved) = (D.stall_t, D.move_t);
    let arrived = g.gd[gd].script.as_ref().map_or(arrived0, |x| x.arrived.unwrap_or(false));
    if arrived || stall > 1.5 || moved > 10.0 { room_fight_plant(g, d, false); }
}

/// a man whose way lies down a covered lane stops short of it, his gun on `look`.
pub(in crate::defenders) fn room_fight_lane_stop(g: &mut crate::game::Game, gd: GuardId, look: V3) -> bool {
    let d = match def_of(g, gd) { Some(d) => d, None => return false };
    if crate::condition::down(g, g.gd[gd].ch) || g.gd[gd].held.is_some() { return false; }
    { let D = &mut g.tab.roomfight_def[d]; D.move_kind = DefMoveKind::Fallback; D.came_by = None; }
    room_fight_plant(g, d, false);
    if let Some(r) = g.tab.roomfight_def[d].room { let n = g.tab.roomfight_room[r].entries.len(); g.tab.roomfight_def[d].ev = vec![0.0; n]; }
    let pt: V3 = [look[0], 0.0, look[2]];
    { let D = &mut g.tab.roomfight_def[d]; D.watch = -1.0; D.want = -1.0; D.home = pt; }
    move_pt(g, d, pt); cover_pt(g, d);
    true
}

/// Where he has got to is his post now: held and pinned there, roused, watching the door he came through or the way the trouble was.
fn room_fight_plant(g: &mut crate::game::Game, d: RoomfightDefId, fighting: bool) {
    let gd = g.tab.roomfight_def[d].gd;
    let c = g.gd[gd].ch;
    let threat = v3::copy(room_fight_threat_of(g, d));
    if g.gd[gd].script.is_some() { crate::squad::end_guard_script(g, gd, Some(crate::squad::EndGuardScriptState::Alert)); }
    crate::mind::set_state(g, gd, crate::game::GuardState::Alert, "roomfight");
    crate::mind::raise(g, gd, 0.92, "roomfight");
    crate::control::set_hold(g, gd, true, "roomfight");
    crate::control::set_pinned(g, gd, true, "roomfight");
    let was = g.tab.roomfight_def[d].room;
    let pos = g.ch[c].pos;
    let room = room_fight_room_at(g, pos);
    let through = room != was; // he made it out: the door he came by is what he holds
    let time = g.time;
    let n_entries = room.map_or(0, |r| g.tab.roomfight_room[r].entries.len());
    {
        let def = &mut g.tab.roomfight_def[d];
        def.mode = DefMode::Hold;
        def.station = v3::copy(pos);
        def.room = room;
        if through { def.ev = vec![0.0; n_entries]; }
        def.want = -1.0;
        def.watch = -1.0;
        def.glance_until = -1.0;
        def.fix_keep = -1.0;
        def.moved_at = time;
    }
    let came_by = g.tab.roomfight_def[d].came_by;
    let e = match (through, came_by, room) {
        (true, Some(door), Some(r)) => match g.tab.roomfight_room[r].entries.iter().position(|x| x.door == door) { Some(i) => i as f64, None => -1.0 },
        _ => room_fight_entry_toward(g, d, threat),
    };
    let move_kind = g.tab.roomfight_def[d].move_kind;
    if e >= 0.0 && (move_kind == DefMoveKind::Fallback || !through) {
        let def = &mut g.tab.roomfight_def[d];
        def.ev[e as usize] = m::max(def.ev[e as usize], 1.5);
        def.want = e;
        def.watch = e;
    }
    let home = match room {
        Some(r) if e >= 0.0 => v3::copy(g.tab.roomfight_room[r].entries[e as usize].at),
        _ => [threat[0], 0.0, threat[2]],
    };
    g.tab.roomfight_def[d].home = home;
    move_pt(g, d, [home[0], 0.0, home[2]]);
    cover_pt(g, d);
    if move_kind == DefMoveKind::Cover && g.tune.suppression { g.cond[g.gd[gd].ch].sup.down_t = m::max(g.cond[g.gd[gd].ch].sup.down_t, 1.0); } // into cover: a breath with his head down
    tell_news(g, News::Man(gd, Did::Plants { fighting }));
}

/// Off his post on the fight AI's own hunt: once he is calm again, wherever that is, he holds there.
fn room_fight_tick_free(g: &mut crate::game::Game, d: RoomfightDefId) {
    let gd = g.tab.roomfight_def[d].gd;
    let c = g.gd[gd].ch;
    let (state, awareness) = (g.gd[gd].state, g.gd[gd].awareness);
    if g.time - g.tab.roomfight_def[d].free_at < 6.0 || !(state == crate::game::GuardState::Patrol || (state == crate::game::GuardState::Suspicious && awareness < 0.3)) { return; }
    let f = crate::character::character_forward(g, c);
    let pos = g.ch[c].pos;
    let room = room_fight_room_at(g, pos);
    let n_entries = room.map_or(0, |r| g.tab.roomfight_room[r].entries.len());
    let time = g.time;
    {
        let def = &mut g.tab.roomfight_def[d];
        def.mode = DefMode::Hold;
        def.station = v3::copy(pos);
        def.room = room;
        def.ev = vec![0.0; n_entries];
        def.want = -1.0;
        def.watch = -1.0;
        def.home = [pos[0] + f[0] * 6.0, 0.0, pos[2] + f[2] * 6.0];
        def.moved_at = time;
    }
    crate::control::set_hold(g, gd, true, "roomfight");
    crate::control::set_pinned(g, gd, true, "roomfight");
    tell_news(g, News::Man(gd, Did::HoldsWhereHeIs));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn near_leg_measures_to_the_segment_not_its_corners() {
        let (a, b): (V3, V3) = ([0.0, 0.0, 0.0], [10.0, 0.0, 0.0]);
        assert!(near_leg(a, b, [5.0, 0.0, 1.9]));    // beside the middle of the leg, far from both corners
        assert!(!near_leg(a, b, [5.0, 0.0, 2.0]));   // 2 m exactly is not "< 2"
        assert!(near_leg(a, b, [-1.9, 0.0, 0.0]));   // clamped to the first corner
        assert!(!near_leg(a, b, [12.5, 0.0, 0.0]));  // clamped to the last
        assert!(near_leg(a, a, [1.0, 9.0, 0.0]));    // a leg of no length, y ignored
    }
    fn post(g: &mut Game, pt: V3) -> (GuardId, RoomfightDefId) {
        let gd = g.gd.push(crate::game::Guard::blank());
        let mut row = Def::default(); row.gd = gd; row.pt = pt;
        (gd, g.tab.roomfight_def.push(row))
    }
    #[test]
    fn rule_on_a_cover_moves_with_pt_and_the_belief_stays() {
        let mut g = Game::blank();
        let (pt, q, seen): (V3, V3, V3) = ([1.0, 0.0, 2.0], [3.0, 0.0, 4.0], [5.0, 0.0, 6.0]);
        let (gd, d) = post(&mut g, pt);
        g.gd[gd].last_known = Some(pt); g.gd[gd].covers = Some(pt);   // a noise filed at the door he covers is still a noise
        move_pt(&mut g, d, q);
        assert_eq!((g.gd[gd].covers, g.gd[gd].last_known), (Some(q), Some(pt)));
        g.gd[gd].covers = Some(seen);                // a point given once is not his post
        move_pt(&mut g, d, pt);
        assert_eq!(g.gd[gd].covers, Some(seen));
    }
    #[test]
    fn the_table_on_the_game_has_the_hostile_row() {
        let g = Game::blank();
        let own = fight_skill().get(String::from("hostile")).copied().unwrap();
        let on_game = hostile_skill(&g);
        assert_eq!((on_game.react, on_game.ready, on_game.turn), (own.react, own.ready, own.turn));
    }
}
