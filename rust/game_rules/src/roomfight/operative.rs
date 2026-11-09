//! A SQUAD MAN IN THE FIGHT: his eyes on each of them, who his muzzle is on, what he does with nobody in his sight, and his fire.
#![allow(unused_imports)]
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// the sight gain, with the hunting man's x2.2 for a scripted man.   (roomfight.ts rateOn)
pub fn room_fight_rate_on(g: &mut crate::game::Game, viewer: GuardId, target: GuardId, dist: f64, known: Option<bool>) -> f64 {
    let known = known.unwrap_or(false);
    let r = crate::guards::sight_gain_on(g, viewer, target, dist);
    let st = g.gd[viewer].state;
    if st != crate::game::GuardState::Alert && st != crate::game::GuardState::Search && (known || g.time - room_fight_hot_of(g, viewer) < (if g.gd[viewer].team == crate::teams::Team::Operative { g.k.roomfight.team_hot } else { 10.0 })) { r * 2.2 } else { r }
}

/// this teammate's feet are held this frame.   (roomfight.ts short)
pub fn room_fight_short(g: &crate::game::Game, gd: GuardId) -> bool {
    if !g.fight.live { return false; }
    match op_id(g, gd) { Some(o) => { let s = g.tab.roomfight_op[o].short_t; g.time - s >= 0.0 && g.time - s < 0.25 } None => false }
}

/// FIGHT_SKILL.operative: `readonly skill = FIGHT_SKILL`, one table (roomfight.ts:260)
pub(super) fn skill_operative(g: &Game) -> FightSkill { *g.fight.skill.get(RoomFightSkillKey::Operative).unwrap() }

/// `gd.weapon.def`: the row of WEAPONS behind a man's gun
pub(super) fn weapon_def_of(g: &Game, gd: GuardId) -> crate::weapons::WeaponDef {
    g.m.weapons.weapons.get(g.fa[g.gd[gd].weapon].def).unwrap().clone()
}

/// `s.over === O.ov`: only this module ever puts an overlay on a script, and each man has one Op
pub(super) fn over_is_mine(g: &Game, gd: GuardId) -> bool {
    g.gd[gd].script.as_ref().map_or(false, |s| s.over.is_some())
}

/// O.ov.aimAt IS O.aim and s.over IS O.ov in the TypeScript: after a write to either, the script's copy follows
pub(super) fn over_follow(g: &mut Game, o: RoomfightOpId) {
    let over = over_of(g, o);
    let gd = g.tab.roomfight_op[o].gd;
    if let Some(s) = g.gd[gd].script.as_mut() { if s.over.is_some() { s.over = Some(over); } }
}

/// the script's copy of O.ov (OpOv and GuardScriptOver are one object in the TypeScript; its aimAt is O.aim)
pub(super) fn over_of(g: &Game, o: RoomfightOpId) -> crate::game::GuardScriptOver {
    let (aim, ov) = (g.tab.roomfight_op[o].aim, g.tab.roomfight_op[o].ov);
    crate::game::GuardScriptOver { aim_at: aim, face: ov.face, walk: ov.walk }
}

/// A teammate's record in the fight, as it starts. (roomfight.ts newOp)
pub fn room_fight_new_op(g: &mut crate::game::Game, gd: GuardId) -> RoomfightOpId {
    let aim: V3 = [0.0, 0.0, 0.0];
    g.tab.roomfight_op.push(Op {
        gd, meters: JsMap::new(), tgt: None, turn: None, pass: None, kept: None, track: 0.0, pre: false, ready_at: 0.0, n: 0.0, next: 0.0, lost_at: -100.0,
        fell: None, first: true, aim, ov: OpOv { face: None, walk: None }, step: None, hot_t: -100.0, step_t: -100.0, step_by: None, door_by: None,
        funnel_t: 0.0, line_t: 0.0, line_by: None, line_at: -100.0, wall_t: 0.0, step_until: 0.0, said_t: -100.0, rounds: 0.0, held_for_friend: 0.0,
        held_for_line: 0.0, block_t: 0.0, feet_sm: None, feet_t: None, short_seen: -100.0, short_since: -100.0, short_t: -100.0,
    })
}

/// A word, at most once a gap. (roomfight.ts speak)
pub fn room_fight_speak(g: &mut crate::game::Game, o: RoomfightOpId, text: &str, gap: Option<f64>) {
    let gap = gap.unwrap_or(1.2);
    if g.time - g.tab.roomfight_op[o].said_t < gap { return; }
    g.tab.roomfight_op[o].said_t = g.time;
    crate::game::game_say(g, g.tab.roomfight_op[o].gd, text, None, None, None);
}

/// One teammate's frame in the fight: his eyes, who his muzzle is on, his trigger. (roomfight.ts tickOp)
pub fn room_fight_tick_op(g: &mut crate::game::Game, o: RoomfightOpId, dt: f64) {
    let gd = g.tab.roomfight_op[o].gd;
    let seat = crate::seats::view_of(g, gd);
    crate::seats::with_seat(g, seat, &mut |g: &mut Game| tick_op_in_view(g, o, dt));
}

/// one frame of a squad man, with his player's seat on the game (`room_fight_tick_op`)
pub(super) fn tick_op_in_view(g: &mut Game, o: RoomfightOpId, dt: f64) {
    let gd = g.tab.roomfight_op[o].gd;
    let c = g.gd[gd].ch;
    let sk = skill_operative(g);
    g.gd[gd].skill = Some(crate::game::GuardSkill { settle_k: sk.settle_k, spread_k: sk.spread_k });
    if g.gd[gd].state == crate::game::GuardState::Dead { g.tab.roomfight_op[o].tgt = None; g.tab.roomfight_op[o].turn = None; g.tab.roomfight_op[o].step = None; room_fight_restore(g, o); return; }
    if g.tab.roomfight_op[o].tgt.is_some() { let gd = g.tab.roomfight_op[o].gd; crate::gunslots::side_seen(g, gd); }   // his sidearm stays out while he has a man
    calls_mate_down(g, o);
    if g.gd[gd].script.is_none() { under_the_fight_ai(g, o); return; }
    let vel = g.ch[c].vel;
    let speed = m::hyp2(vel[0], vel[2]);
    feet(g, o, speed, dt);
    let blind = crate::control::man::blind(g, gd) || crate::control::man::staggered(g, gd) || g.gd[gd].held.is_some();
    let since_hot = g.time - m::max(g.tab.roomfight_op[o].hot_t, g.fight.hot_t);
    let hot = since_hot < g.k.roomfight.team_hot || crate::spotted::spotted_keen(g, gd);
    let mut best = his_eyes(g, o, dt, &sk, blind, hot);
    if blind { best = None; g.tab.roomfight_op[o].turn = None; } // he has nobody: no round goes on the memory of where a man was
    if best != g.tab.roomfight_op[o].tgt { muzzle_to(g, o, best, &sk); }
    own_step_back(g, o, since_hot);
    if g.tab.roomfight_op[o].tgt.is_none() { nobody_in_sight(g, o, dt, speed, blind); return; }
    engage(g, o, dt, speed, &sk);
}

/// a mate down in his view: said once
fn calls_mate_down(g: &mut Game, o: RoomfightOpId) {
    let gd = g.tab.roomfight_op[o].gd;
    let c = g.gd[gd].ch;
    let mut i = 0;
    while i < g.fight.ops.len() {
        let xo = g.fight.ops[i];
        i += 1;
        let x = g.tab.roomfight_op[xo].gd;
        if xo == o || g.gd[x].state != crate::game::GuardState::Dead || g.fight.down_called.has(x) { continue; }
        let e = eye_of(g, c);
        let xp = g.ch[g.gd[x].ch].pos;
        if crate::scene_collision::segment_blocked(g, e, [xp[0], 0.4, xp[2]], None, None, None) { continue; }
        g.fight.down_called.add(x);
        room_fight_speak(g, o, "man down.", Some(0.0));
    }
}

/// the fight AI has him (no script of an order's): Sam is not on its list, so his line is kept here
fn under_the_fight_ai(g: &mut Game, o: RoomfightOpId) {
    let gd = g.tab.roomfight_op[o].gd;
    let c = g.gd[gd].ch;
    room_fight_restore(g, o);
    g.tab.roomfight_op[o].tgt = None;
    g.tab.roomfight_op[o].turn = None;
    g.tab.roomfight_op[o].step = None;
    let fc = match g.gd[gd].foe { Some(crate::game::Foe::Guard(f)) => Some(g.gd[f].ch), _ => None };
    if let (crate::game::GuardState::Alert, Some(fc)) = (g.gd[gd].state, fc) {
        let muzzle = g.ch[c].muzzle;
        let at = chest_of(g, fc);
        if matches!(room_fight_friend_in_line(g, muzzle, at, gd), Some(RoomFightFriendInLine::Player)) {
            g.gd[gd].muzzle_down = true;
            g.gd[gd].muzzle_clear_t = 0.0;
            g.tab.roomfight_op[o].held_for_friend += 1.0;
            g.fight.stats.held_for_friend += 1.0;
        }
    }
}

/// how his feet move his aim, let go of slowly
fn feet(g: &mut Game, o: RoomfightOpId, speed: f64, dt: f64) {
    if !g.touring {
        let since = match g.tab.roomfight_op[o].feet_t { None => dt, Some(t) => m::max(dt, g.time - t) };
        let prev = g.tab.roomfight_op[o].feet_sm.unwrap_or(0.0);
        g.tab.roomfight_op[o].feet_sm = Some(crate::aimcost::feet_hold(g, prev, speed, since));
        g.tab.roomfight_op[o].feet_t = Some(g.time);
    }
}

/// HIS EYES: every living hostile through the game's sight test, at the game's rate; whom his muzzle should be on, of those he has made out
fn his_eyes(g: &mut Game, o: RoomfightOpId, dt: f64, sk: &FightSkill, blind: bool, hot: bool) -> Option<GuardId> {
    let gd = g.tab.roomfight_op[o].gd;
    let c = g.gd[gd].ch;
    let mut best: Option<GuardId> = None;
    let mut bs = -1.0;
    let mut i = 0;
    while i < g.guards.len() {
        let h = g.guards[i];
        i += 1;
        let (mine, theirs) = (g.gd[gd].team, g.gd[h].team);
        if theirs == mine || !crate::teams::is_hostile(g, mine, theirs) { continue; }
        if !g.tab.roomfight_op[o].meters.has(h) { g.tab.roomfight_op[o].meters.set(h, Meter { m: 0.0, seen_t: -100.0, glass_t: None }); }
        if g.gd[h].state == crate::game::GuardState::Dead { g.tab.roomfight_op[o].meters.get_mut(h).unwrap().m = 0.0; continue; }
        if g.gd[h].fled { g.tab.roomfight_op[o].meters.get_mut(h).unwrap().m = 0.0; continue; } // a man who dropped it and ran is nobody's target
        let mut no_shot = false;
        if let Some(p) = g.tab.roomfight_op[o].pass {
            if p.h == h { // a masked man let go of (letGo): not his again for that long
                if g.time >= p.until { g.tab.roomfight_op[o].pass = None; } else if !p.no_shot.unwrap_or(false) { continue; } else { no_shot = true; }
            }
        }
        let hc = g.gd[h].ch;
        let vis = if blind { None } else { Some(crate::guards::can_see_char(g, gd, hc, false)) };
        let visible = vis.map_or(false, |v| v.visible);
        let known = g.tab.roomfight_op[o].meters.get(h).unwrap().m >= sk.notice;
        let rate = match vis { Some(v) if v.visible => room_fight_rate_on(g, gd, h, v.dist, Some(known)), _ => 0.0 };
        {
            let time = g.time;
            let me = g.tab.roomfight_op[o].meters.get_mut(h).unwrap();
            if rate > 0.05 {
                me.m = m::min(1.0, me.m + rate * dt);
                if rate > 0.25 { me.seen_t = time; }
            } else {
                me.m = m::max(0.0, me.m - 0.5 * dt);
            }
        }
        if no_shot { // walked on from for want of a shot: his again the moment there is one
            let mut clear = false;
            if visible {
                let at = chest_of(g, hc);
                clear = crate::guards::clear_shot(g, c, at);
                if !clear { let head = head_of(g, hc); clear = crate::guards::clear_shot(g, c, head); }
            }
            if clear { g.tab.roomfight_op[o].pass = None; } else { continue; }
        }
        let me = *g.tab.roomfight_op[o].meters.get(h).unwrap();
        // a real sighting now, and either he has made the shape out over time or the floor is hot
        if g.time - me.seen_t < 0.25 && (me.m >= sk.notice || hot) && !crate::pairorders::pair_passes(g, gd, h) && !crate::workthings::work_passes(g, gd, h) && !room_fight_cover_passes(g, &crate::seats::here(g), o, h) {
            let (hp, cp) = (g.ch[hc].pos, g.ch[c].pos);
            let on_us = if m::abs(v3::wrap_angle(v3::yaw_to(hp, cp) - g.ch[hc].aim_yaw)) < 0.35 { 2.0 } else { 0.0 }; // a muzzle on him outranks one that is not
            let r = g.k.roomfight;
            let dist = match vis { Some(v) => v.dist, None => 9.0 };
            let alarm = g.gd[h].state == crate::game::GuardState::Alert || g.gd[h].state == crate::game::GuardState::Search;
            let passed = me.glass_t.is_some(); // he has held a round at this man for a pane in the way (never true of a loud squad)
            let sc = if passed && !alarm && g.time - me.glass_t.unwrap() < GLASS_LOOK {
                0.01 / (1.0 + dist)
            } else {
                let stays = if g.tab.roomfight_op[o].tgt == Some(h) && !(g.tab.roomfight_op[o].block_t >= r.no_shot) { 10.0 } else { 0.0 };
                let close = if js::truthy(on_us) && dist < r.close_m { 10.0 } else { 0.0 };
                let raised = match g.tab.roomfight_op[o].tgt { Some(t) if passed && alarm && g.gd[t].state != crate::game::GuardState::Alert && g.gd[t].state != crate::game::GuardState::Search => 20.0, _ => 0.0 };
                stays + on_us + close + 1.0 / (1.0 + dist) + raised
            };
            if sc > bs { bs = sc; best = Some(h); }
        }
    }
    best
}

/// WHO HIS MUZZLE IS ON, changed: how long before it is on the new man, and what he says of it
fn muzzle_to(g: &mut Game, o: RoomfightOpId, best: Option<GuardId>, sk: &FightSkill) {
    let gd = g.tab.roomfight_op[o].gd;
    let c = g.gd[gd].ch;
    if let Some(b) = best {
        let cp = g.ch[c].pos;
        let bp = g.ch[g.gd[b].ch].pos;
        let off = m::abs(v3::wrap_angle(v3::yaw_to(cp, bp) - g.ch[c].aim_yaw));
        let t = g.time;
        let beat = if g.tab.roomfight_op[o].tgt.is_some() || t - g.tab.roomfight_op[o].lost_at < 1.5 {
            pick(g, crate::dice::Who::Op(o), sk.shift)
        } else if off > sk.sector_deg * crate::v3::DEG && !g.tab.roomfight_op[o].turn.map_or(false, |u| u.to == b && t >= u.at) {
            let a = pick(g, crate::dice::Who::Op(o), sk.react);
            a + pick(g, crate::dice::Who::Op(o), sk.surprise)
        } else {
            pick(g, crate::dice::Who::Op(o), sk.react) * sk.ready
        };
        g.tab.roomfight_op[o].ready_at = t + beat;
        g.tab.roomfight_op[o].track = 0.0;
        g.tab.roomfight_op[o].n = 0.0;
        g.tab.roomfight_op[o].next = 0.0;
        g.tab.roomfight_op[o].fell = Some(false);
        let raised = match g.gd[gd].script.as_ref() { Some(s) => s.over.is_some() || s.upper.unwrap_or(crate::character::UpperMode::Aim) == crate::character::UpperMode::Aim, None => false };
        g.tab.roomfight_op[o].pre = raised && off < g.k.roomfight.preaim_deg * crate::v3::DEG; // on the line his gun was already held on
        if let (None, Some(k)) = (g.tab.roomfight_op[o].tgt, g.tab.roomfight_op[o].kept) {
            if k.h == b && t - k.t < 1.5 { // the man he was on a moment ago: neither his track nor the beat starts from nothing
                let gap = (t - k.t) / 1.5;
                g.tab.roomfight_op[o].track = k.track * (1.0 - gap);
                g.tab.roomfight_op[o].ready_at = t + pick(g, crate::dice::Who::Op(o), sk.shift) * gap;
            }
        }
        if !g.fight.called.has(b) {
            g.fight.called.add(b);
            let rel = v3::wrap_angle(v3::yaw_to(cp, bp) - g.ch[c].body_yaw);
            room_fight_speak(g, o, if rel > 0.5 { "contact left." } else if rel < -0.5 { "contact right." } else { "contact front." }, Some(0.4));
        }
        let text = alloc::format!("eyes on {}", g.gd[b].callsign);
        room_fight_note(g, Some(gd), &text);
    } else {
        let was = g.tab.roomfight_op[o].tgt;
        let fell = was.map_or(false, |t| g.gd[t].state == crate::game::GuardState::Dead);
        if let (Some(t), true) = (was, fell) {
            let mine = g.gd[t].killer_id == g.ch[c].id;
            let text: &str = if mine { "tango down." } else { "he's down." };
            room_fight_speak(g, o, text, Some(0.6));
        }
        g.tab.roomfight_op[o].fell = Some(fell);
        g.tab.roomfight_op[o].lost_at = g.time;
        g.tab.roomfight_op[o].kept = match was { Some(t) if !fell => Some(OpKept { h: t, track: g.tab.roomfight_op[o].track, t: g.time }), _ => None };
    }
    g.tab.roomfight_op[o].tgt = best;
    g.tab.roomfight_op[o].line_t = 0.0;
    g.tab.roomfight_op[o].block_t = 0.0;
}

/// a step of this module's handed back: re-ordered under it, the new order stands; else, the reason gone, the order's own legs again
fn own_step_back(g: &mut Game, o: RoomfightOpId, since_hot: f64) {
    let gd = g.tab.roomfight_op[o].gd;
    if let Some(st) = g.tab.roomfight_op[o].step {
        let goal = g.gd[gd].script.as_ref().and_then(|s| s.goal);
        if goal != Some(st.mine) {
            g.tab.roomfight_op[o].step = None;
        } else if g.time >= g.tab.roomfight_op[o].step_until && g.tab.roomfight_op[o].tgt.is_none() && g.time - g.tab.roomfight_op[o].lost_at > 1.0 {
            let gone = !st.door || since_hot >= 10.0 || match st.goal { None => true, Some(p) => room_fight_funnel_of(g, p).is_none() };
            if gone { room_fight_hand_back(g, o); }
        }
    }
}

/// NOBODY IN HIS SIGHT: a round at his side, or the man a mate has just called, brings his muzzle round to look; never a round at a noise
fn nobody_in_sight(g: &mut Game, o: RoomfightOpId, dt: f64, speed: f64, blind: bool) {
    let gd = g.tab.roomfight_op[o].gd;
    let c = g.gd[gd].ch;
    if !blind { room_fight_cue(g, o); }
    let look = match g.tab.roomfight_op[o].turn { Some(u) if g.time >= u.at && g.gd[u.to].state != crate::game::GuardState::Dead => Some(u), _ => None };
    if let Some(u) = look {
        let tc = chest_of(g, g.gd[u.to].ch);
        let to = match u.pt {
            Some(pt) => { let e = eye_of(g, c); if crate::guards::eye_blocked(g, gd, e, tc) { pt } else { tc } }
            None => tc,
        };
        room_fight_sam_in_line(g, o, to);
        room_fight_point(g, o, to, dt, speed);
    } else if !blind && crate::coverfire::cover_op(g, o, dt, speed, &mut |g: &mut Game, to: V3| room_fight_point(g, o, to, dt, speed)) {
        // a man he knows is behind something breakable: his rounds go into it
    } else if over_is_mine(g, gd) {
        if g.time - g.tab.roomfight_op[o].lost_at > 0.5 || (g.tab.roomfight_op[o].fell == Some(true) && crate::doortools::door_tools_owes_corner(g, &crate::seats::here(g), gd)) {
            room_fight_restore(g, o);
        } else if flat(g.ch[c].pos, g.tab.roomfight_op[o].aim) < 2.5 { // the half second his gun stays where the man was: a BEARING
            let (p, yaw) = (g.ch[c].pos, g.ch[c].aim_yaw);
            g.tab.roomfight_op[o].aim[0] = p[0] + m::sin(yaw) * 2.5;
            g.tab.roomfight_op[o].aim[2] = p[2] + m::cos(yaw) * 2.5;
            over_follow(g, o);
        }
    }
    room_fight_doorway(g, o, dt, speed);
}

/// HE HAS A MAN: his gun on him, what is in the way, and the round
fn engage(g: &mut Game, o: RoomfightOpId, dt: f64, speed: f64, sk: &FightSkill) {
    let gd = g.tab.roomfight_op[o].gd;
    let c = g.gd[gd].ch;
    g.tab.roomfight_op[o].turn = None;
    let h = g.tab.roomfight_op[o].tgt.unwrap();
    let hc = g.gd[h].ch;
    let chest = chest_of(g, hc);
    let dist = flat(g.ch[c].pos, g.ch[hc].pos);
    let brg = room_fight_point(g, o, chest, dt, speed);
    let sam_in = room_fight_sam_in_line(g, o, chest);
    if sam_in { room_fight_hold_short(g, o); }
    if sam_in && g.time >= g.tab.roomfight_op[o].ready_at && !crate::control::man::changing_magazine(g, gd) { // the fire gate's own answer to a friend in the line: with the gun down he never gets that far
        g.tab.roomfight_op[o].held_for_friend += 1.0;
        g.fight.stats.held_for_friend += 1.0;
        g.tab.roomfight_op[o].line_t += dt;
        g.tab.roomfight_op[o].line_by = Some(OpLineBy::Player);
        g.tab.roomfight_op[o].line_at = g.time;
        room_fight_side_step(g, o, chest, speed, None, None);
        room_fight_let_go(g, o);
        if g.tab.roomfight_op[o].tgt.is_none() { return; }
    }
    room_fight_doorway(g, o, dt, speed);
    if let Some(to) = tracked(g, o, h, dt, speed, dist, brg, sk) { fires(g, o, h, to, dt, speed, dist, sk); }
}

/// THE TRACK (aim.ts's law, this side's K): whether a round may go now, and at what point of him
fn tracked(g: &mut Game, o: RoomfightOpId, h: GuardId, dt: f64, speed: f64, dist: f64, brg: f64, sk: &FightSkill) -> Option<V3> {
    let gd = g.tab.roomfight_op[o].gd;
    let c = g.gd[gd].ch;
    let hc = g.gd[h].ch;
    let chest = chest_of(g, hc);
    let p = g.k.aim.guard;
    let facing_err = m::abs(v3::wrap_angle(brg - g.ch[c].aim_yaw));
    let on_man = g.time >= g.tab.roomfight_op[o].ready_at && facing_err < p.on_deg * crate::v3::DEG + m::atan2(0.35, m::max(dist, 0.3)) && v3::dist(g.tab.roomfight_op[o].aim, chest) < 0.3;
    let on = on_man && !g.gd[gd].muzzle_down && !crate::teampace::team_sprinting(g, gd); // nothing is fired at a run
    let settle = m::max(p.settle_secs, 1e-3) * v3::clamp(dist / m::max(p.range_ref, 0.1), p.near_k, 1.0) * sk.settle_k;
    if on && g.tab.roomfight_op[o].pre { g.tab.roomfight_op[o].track = m::max(g.tab.roomfight_op[o].track, sk.preaim); g.tab.roomfight_op[o].pre = false; }
    g.tab.roomfight_op[o].track = if on { m::min(1.0, g.tab.roomfight_op[o].track + dt / settle) } else { m::max(0.0, g.tab.roomfight_op[o].track - p.lose_rate * dt) };
    if !g.tune.aim_model { g.tab.roomfight_op[o].track = if on { 1.0 } else { 0.0 }; }
    if on_man && crate::gunwall::gun_wall_is(g, c) { // the gun IN a wall: no shot from here, as with no line
        g.tab.roomfight_op[o].held_for_line += 1.0;
        g.tab.roomfight_op[o].block_t += dt;
        g.tab.roomfight_op[o].wall_t += dt;
        room_fight_walk_on(g, o);
        if g.tab.roomfight_op[o].tgt.is_some() {
            let waited = g.tab.roomfight_op[o].wall_t;
            if room_fight_side_step(g, o, chest, speed, Some(waited), Some("steps aside: his gun is in a wall")) { g.tab.roomfight_op[o].wall_t = 0.0; }
        }
        return None;
    }
    g.tab.roomfight_op[o].wall_t = 0.0;
    if !on { g.tab.roomfight_op[o].block_t = m::max(0.0, g.tab.roomfight_op[o].block_t - dt); } // off him, the count runs back down
    if g.tab.roomfight_op[o].track < p.acquire_min - 1e-9 || g.time < g.tab.roomfight_op[o].next || crate::control::man::changing_magazine(g, gd) || crate::gunslots::side_locked(g, gd) || !on { return None; }
    if crate::silent::round_held(g, gd, h) { // the man has not noticed the team: the gun stays on him, no round goes
        room_fight_speak(g, o, "eyes on. holding.", Some(8.0));
        return None;
    }
    let mut to = chest;
    if !crate::guards::clear_shot(g, c, to) {
        let head = head_of(g, hc);
        if crate::guards::clear_shot(g, c, head) {
            to = head;
        } else if !crate::coverfire::cover_seen(g, gd, Some(crate::game::Foe::Guard(h)), chest) {
            g.tab.roomfight_op[o].held_for_line += 1.0;
            g.tab.roomfight_op[o].block_t += dt;
            room_fight_walk_on(g, o);
            return None;
        }
    }
    g.tab.roomfight_op[o].block_t = 0.0;
    if crate::silent::silent_holds_glass(g, gd, h, c, to) { // a pane on the way to a man who has raised no alarm: held, said once a hold
        if let Some(me) = g.tab.roomfight_op[o].meters.get(h).copied() {
            let say = match me.glass_t { None => true, Some(t) => g.time - t > GLASS_LOOK };
            if say { room_fight_speak(g, o, "eyes on, behind glass. holding.", Some(8.0)); }
            let time = g.time;
            g.tab.roomfight_op[o].meters.get_mut(h).unwrap().glass_t = Some(time);
        }
        return None;
    }
    Some(to)
}

/// THE ROUND: how wide it may go, who else it could find, and what follows it
fn fires(g: &mut Game, o: RoomfightOpId, h: GuardId, to: V3, dt: f64, speed: f64, dist: f64, sk: &FightSkill) {
    let gd = g.tab.roomfight_op[o].gd;
    let c = g.gd[gd].ch;
    let hc = g.gd[h].ch;
    let p = g.k.aim.guard;
    let def = weapon_def_of(g, gd);
    let spread = def.ai_spread;
    let hands = (spread.base + m::max(0.0, dist - spread.range_from) * spread.range_k) * sk.spread_k;
    let sup_v = g.gd[gd].sup.v;
    let sup_k = if g.tune.suppression && crate::condition::condition_sup(g, crate::condition::Man::Guard(gd), sup_v) >= g.k.suppression.flinch { 1.6 } else { 1.0 };
    let string = crate::gunroles::gun_string(g, gd); // a belt-fed gun's string, the Mk 12's single aimed round; 0: the controlled pair
    let grow = if string > 1.0 { crate::gunroles::gun_string_grow(g, gd, js::rem(g.tab.roomfight_op[o].n, string)) } else { 0.0 };
    let feet = if !g.touring { m::max(speed, g.tab.roomfight_op[o].feet_sm.unwrap_or(0.0)) * g.k.aimcost.move_k } else { speed };
    let wide = hands * v3::lerp(p.unsettled_k, 1.0, v3::clamp(g.tab.roomfight_op[o].track, 0.0, 1.0)) * sup_k + grow + feet * p.shooter_move_k * 0.5; // how wide this round would be thrown
    let muzzle = g.ch[c].muzzle;
    let mut in_line = room_fight_friend_in_line(g, muzzle, to, gd);
    if in_line.is_none() { // a friend anywhere this round could go holds it as one in the lane does
        let cone = wide + match def.pellets { Some(pl) => pl.cone * 2.0, None => 0.0 };
        in_line = crate::friendlyfire::friend_in_cone(g, gd, muzzle, to, cone, None, Some(hc)).map(cone_friend);
        if in_line.is_some() { crate::friendlyfire::ff_held(g); }
    }
    if let Some(f) = in_line {
        g.tab.roomfight_op[o].held_for_friend += 1.0;
        g.fight.stats.held_for_friend += 1.0;
        g.tab.roomfight_op[o].line_t += dt;
        g.tab.roomfight_op[o].line_by = Some(line_by(f));
        g.tab.roomfight_op[o].line_at = g.time;
        if matches!(f, RoomFightFriendInLine::Player) { room_fight_hold_short(g, o); }
        room_fight_side_step(g, o, to, speed, None, None);
        room_fight_let_go(g, o);
        return;
    }
    g.tab.roomfight_op[o].line_t = 0.0;
    let fa = g.gd[gd].weapon;
    if !crate::weapons::firearm_fire(g, fa) {
        if crate::gunslots::side_out(g, gd, dist) { return; } // dry on a man close in front of him: his sidearm
        crate::guards::begin_reload(g, gd);
        room_fight_speak(g, o, "reloading.", Some(0.0));
        return;
    }
    crate::combat::fire_weapon(g, c, to, false, Some(&crate::combat::FireOpts { steady: Some(true), spread: Some(wide), ..Default::default() }));
    g.tab.roomfight_op[o].rounds += 1.0;
    g.tab.roomfight_op[o].n += 1.0;
    g.fight.stats.op_rounds += 1.0;
    let gap = if js::truthy(string) {
        if js::rem(g.tab.roomfight_op[o].n, string) == 0.0 { pick(g, crate::dice::Who::Op(o), sk.shift) } else { crate::gunroles::gun_string_gap(g, gd) }
    } else if js::rem(g.tab.roomfight_op[o].n, 2.0) == 0.0 {
        pick(g, crate::dice::Who::Op(o), sk.shift)
    } else {
        sk.pair
    };
    g.tab.roomfight_op[o].next = g.time + gap;
    let def = weapon_def_of(g, gd);
    if def.pellets.is_some() { g.tab.roomfight_op[o].next = m::max(g.tab.roomfight_op[o].next, g.time + def.cd); } // no pair comes faster than a pump gun's action is worked
}

/// The muzzle onto a point, through this module's overlay on his script; nothing the order holds is written. Returns the bearing. (roomfight.ts point)
pub fn room_fight_point(g: &mut crate::game::Game, o: RoomfightOpId, to: V3, dt: f64, speed: f64) -> f64 {
    let gd = g.tab.roomfight_op[o].gd;
    let c = g.gd[gd].ch;
    let (pos, vel, aim_yaw, body_yaw) = (g.ch[c].pos, g.ch[c].vel, g.ch[c].aim_yaw, g.ch[c].body_yaw);
    if !g.gd[gd].script.as_ref().unwrap().over.is_some() { // from where the muzzle is: it travels, it does not snap
        let r = m::max(1.0, flat(pos, to));
        g.tab.roomfight_op[o].aim[0] = pos[0] + m::sin(aim_yaw) * r;
        g.tab.roomfight_op[o].aim[1] = 1.2;
        g.tab.roomfight_op[o].aim[2] = pos[2] + m::cos(aim_yaw) * r;
        g.tab.roomfight_op[o].ov.face = None;
        let over = over_of(g, o);
        g.gd[gd].script.as_mut().unwrap().over = Some(over);
    }
    for i in 0..3 { g.tab.roomfight_op[o].aim[i] = v3::damp(g.tab.roomfight_op[o].aim[i], to[i], 16.0, dt); }
    let brg = m::atan2(to[0] - pos[0], to[2] - pos[2]);
    let off = m::abs(v3::wrap_angle(brg - body_yaw));
    let walk = speed >= 0.3;
    if walk { // walking, it is the way he is GOING the point is behind, or not: a test on the chest would turn him to and fro
        let off_go = m::abs(v3::wrap_angle(brg - m::atan2(vel[0], vel[2])));
        g.tab.roomfight_op[o].ov.face = if off_go > 1.2 || (g.tab.roomfight_op[o].ov.walk == Some(true) && off_go > 0.7) { Some(brg) } else { None };
    } else {
        g.tab.roomfight_op[o].ov.face = if speed < 0.3 && (off > 1.2 || (g.tab.roomfight_op[o].ov.face.is_some() && off > 0.3)) { Some(brg) } else { None }; // behind his shoulder and he is stood: he turns to it
    }
    g.tab.roomfight_op[o].ov.walk = Some(walk && g.tab.roomfight_op[o].ov.face.is_some());
    over_follow(g, o);
    brg
}

/// Is this man inside the arc an arrived hold gives him (the `mine` of roomfight.ts cue)
pub(super) fn in_arc(g: &Game, c: CharId, arc: Option<f64>, h: GuardId) -> bool {
    match arc {
        None => true,
        Some(a) => m::abs(v3::wrap_angle(v3::yaw_to(g.ch[c].pos, g.ch[g.gd[h].ch].pos) - a)) < g.k.roomfight.arc_half_deg * crate::v3::DEG,
    }
}

/// What turns a teammate with nobody in his sight: a hostile's round from where he has a line, else the man a mate's muzzle is on, a footfall, a door. (roomfight.ts cue)
pub fn room_fight_cue(g: &mut crate::game::Game, o: RoomfightOpId) {
    let gd = g.tab.roomfight_op[o].gd;
    let c = g.gd[gd].ch;
    let sk = skill_operative(g);
    if let Some(u) = g.tab.roomfight_op[o].turn {
        let lapse = g.time > u.until || g.gd[u.to].state == crate::game::GuardState::Dead || (u.door.is_none() && (crate::workthings::work_passes(g, gd, u.to) || room_fight_cover_passes(g, &crate::seats::here(g), o, u.to)));
        if lapse { g.tab.roomfight_op[o].turn = None; g.tab.roomfight_op[o].lost_at = g.time; }
    }
    let mut look: Option<OpTurn> = g.tab.roomfight_op[o].turn.filter(|u| u.pt.is_some()); // a look at where steps come from: it follows them and lasts while they keep coming
    if let (Some(mut l), Some(sb)) = (look, g.tab.roomfight_op[o].step_by) {
        if l.door.is_none() && sb.by == l.to && g.time - sb.t < 0.6 {
            l.pt = Some(sb.at);
            l.until = m::min(m::max(l.until, sb.t + 2.5), l.t0.unwrap_or(g.time) + g.k.roomfight.step_look_secs);
            look = Some(l);
            g.tab.roomfight_op[o].turn = Some(l);
        }
    }
    if g.tab.roomfight_op[o].turn.is_some() && look.is_none() { return; }
    let eye = eye_of(g, c);
    let mut who: Option<GuardId> = None;
    let mut bd = 16.0;
    let mut call = false;
    let mut pt: Option<V3> = None;
    let gone = match g.tab.roomfight_op[o].pass { Some(p) if p.no_shot.unwrap_or(false) && g.time < p.until => Some(p.h), _ => None }; // the man he has just walked on from: only the eye loop turns him back
    let a = crate::seats::order_of(g, gd);
    let arc = match a {
        Some(id) => {
            let row = &g.tab.orders_assignment[id];
            match row.watch { Some(w) if crate::orders::rows::id_is(&row.order, crate::orders::Is::Holds) && row.arrived => Some(v3::yaw_to(g.ch[c].pos, w)), _ => None }
        }
        None => None,
    };
    let on_way_to_door = match a { Some(id) => g.tab.orders_assignment[id].door.is_some() && !g.tab.orders_assignment[id].arrived, None => false };
    let mut i = 0;
    while i < g.fight.reports.len() {
        let r = g.fight.reports[i];
        i += 1;
        if g.time - r.t > 0.6 || g.gd[r.by].state == crate::game::GuardState::Dead || Some(r.by) == gone || room_fight_cover_passes(g, &crate::seats::here(g), o, r.by) { continue; }
        let by = g.gd[r.by].ch;
        let dist = flat(g.ch[c].pos, g.ch[by].pos);
        if dist < bd {
            let at = chest_of(g, by);
            if !crate::guards::eye_blocked(g, gd, eye, at) { bd = dist; who = Some(r.by); }
        }
    }
    if who.is_none() {
        let mut i = 0;
        while i < g.fight.ops.len() {
            let xo = g.fight.ops[i];
            i += 1;
            let (xg, xt) = (g.tab.roomfight_op[xo].gd, g.tab.roomfight_op[xo].tgt);
            let t = match xt { Some(t) if xo != o => t, _ => continue };
            if Some(t) == gone || g.gd[t].state == crate::game::GuardState::Dead || g.gd[xg].state == crate::game::GuardState::Dead { continue; }
            let tc = g.gd[t].ch;
            if flat(g.ch[c].pos, g.ch[tc].pos) >= 16.0 || !in_arc(g, c, arc, t) { continue; }
            let at = chest_of(g, tc);
            if crate::guards::eye_blocked(g, gd, eye, at) || crate::workthings::work_passes(g, gd, t) || room_fight_cover_passes(g, &crate::seats::here(g), o, t) { continue; }
            who = Some(t);
            call = true;
        }
    }
    if who.is_none() {
        if let Some(sb) = g.tab.roomfight_op[o].step_by { // footsteps coming, and his the nearest ears: he looks
            if g.time - sb.t < 0.6 && g.gd[sb.by].state != crate::game::GuardState::Dead && Some(sb.by) != gone && in_arc(g, c, arc, sb.by) && !crate::workthings::work_passes(g, gd, sb.by) && !room_fight_cover_passes(g, &crate::seats::here(g), o, sb.by) {
                let at = chest_of(g, g.gd[sb.by].ch);
                if !crate::guards::eye_blocked(g, gd, eye, at) {
                    who = Some(sb.by);
                    call = true;
                } else if look.is_none() && !on_way_to_door { // at where they come from, with no line to the man himself
                    who = Some(sb.by);
                    call = true;
                    pt = Some(sb.at);
                }
            }
        }
    }
    let mut secs = 2.5;
    let mut door: Option<DoorId> = None;
    if let Some(db) = g.tab.roomfight_op[o].door_by {
        if g.time - db.t >= 0.6 || g.gd[db.by].state == crate::game::GuardState::Dead || look.map_or(false, |l| l.door == Some(db.door)) {
            g.tab.roomfight_op[o].door_by = None; // stale, or the door he is already looking at
        } else if who.is_none() && Some(db.by) != gone && !crate::doors::door_latched(g, db.door) && m::abs(crate::doors::door_angle(g, db.door)) > 0.1 && !on_way_to_door
            && arc.map_or(true, |a| m::abs(v3::wrap_angle(v3::yaw_to(g.ch[c].pos, db.at) - a)) < g.k.roomfight.arc_half_deg * crate::v3::DEG) {
            // a door an enemy's hand has OPENED in his sight: he looks at its doorway
            who = Some(db.by);
            call = true;
            pt = Some(db.at);
            secs = g.k.roomfight.door_look_secs;
            door = Some(db.door);
            g.tab.roomfight_op[o].door_by = None;
            room_fight_note(g, Some(gd), "a door has come open in his sight: he looks at it");
        }
    }
    let mut shot_door: Option<DoorId> = None; // last of all: a man in his line, a mate's call, a footfall and a door just opened each come first
    if who.is_none() && !on_way_to_door {
        let only = look.and_then(|l| l.door);
        if let Some(ds) = room_fight_door_shot(g, o, eye, gone, arc, a, only) {
            who = Some(ds.by);
            pt = Some(ds.at);
            secs = g.k.roomfight.door_look_secs;
            door = Some(ds.door);
            shot_door = Some(ds.door);
            if look.map_or(true, |l| l.door != Some(ds.door)) { room_fight_note(g, Some(gd), "rounds are coming through a doorway in his sight: he looks at it"); }
        }
    }
    if let Some(w) = who {
        let same = look.filter(|l| l.to == w || (shot_door.is_some() && l.door == shot_door)); // the man he was already looking for by ear: no second beat
        let at = match same {
            Some(l) => if door.is_some() { l.at } else { m::min(l.at, g.time) },
            None => { let t = g.time; t + pick(g, crate::dice::Who::Op(o), sk.react) + if call { 0.25 } else { 0.0 } }
        };
        g.tab.roomfight_op[o].turn = Some(OpTurn { to: w, at, until: g.time + secs, pt, t0: Some(g.time), door });
    }
}

/// The nearest OPEN doorway he can see that a hostile's round has just flown through towards his side of its wall, with no line of his to the shooter. (roomfight.ts doorShot)
pub fn room_fight_door_shot(g: &mut crate::game::Game, o: RoomfightOpId, eye: V3, gone: Option<GuardId>, arc: Option<f64>, a: Option<OrdersAssignmentId>, only: Option<DoorId>) -> Option<crate::roomfight::RoomFightDoorShot> {
    let r_k = g.k.roomfight;
    let p = g.ch[g.gd[g.tab.roomfight_op[o].gd].ch].pos;
    let mut best: Option<RoomFightDoorShot> = None;
    let mut bd = f64::INFINITY;
    let mut i = 0;
    while i < g.fight.reports.len() {
        let r = g.fight.reports[i];
        i += 1;
        let (from, to) = match (r.from, r.to) { (Some(f), Some(t)) => (f, t), _ => continue };
        if g.time - r.t > 0.6 || g.gd[r.by].state == crate::game::GuardState::Dead || Some(r.by) == gone || room_fight_cover_passes(g, &crate::seats::here(g), o, r.by) { continue; }
        let mut j = 0;
        while j < g.doors.list.len() {
            let d = g.doors.list[j];
            j += 1;
            if only.map_or(false, |x| x != d) { continue; } // a door he is already looking at keeps his look until it lapses
            if !crate::doors::door_off(g, d) && (crate::doors::door_latched(g, d) || m::abs(crate::doors::door_angle(g, d)) <= 0.1) { continue; } // a shut leaf is no opening
            if let Some(id) = a { // a gun an order has on ANOTHER door keeps it
                let row = &g.tab.orders_assignment[id];
                if let (true, Some(ad), false, Some(w)) = (row.arrived, row.door, row.spare.unwrap_or(false), row.watch) {
                    if ad != d && flat(w, g.dr[ad].frame_centre) < 1.6 { continue; }
                }
            }
            let fc = g.dr[d].frame_centre;
            let far = flat(fc, from);
            let near = flat(fc, p);
            if far > r_k.door_shot_m || near > 9.0 || near < 0.5 || near >= bd { continue; }
            let th = g.dr[d].def.closed_dir;
            let nx = -m::sin(th);
            let nz = m::cos(th);
            let sa = (from[0] - fc[0]) * nx + (from[2] - fc[2]) * nz;
            let sb = (to[0] - fc[0]) * nx + (to[2] - fc[2]) * nz;
            if sa * sb >= 0.0 || sa * ((p[0] - fc[0]) * nx + (p[2] - fc[2]) * nz) >= 0.0 { continue; } // the round did not cross that wall's line, or was fired from his own side of it
            let k = sa / (sa - sb);
            let cx = from[0] + (to[0] - from[0]) * k;
            let cz = from[2] + (to[2] - from[2]) * k;
            if m::abs((cx - fc[0]) * m::cos(th) + (cz - fc[2]) * m::sin(th)) > g.dr[d].def.width / 2.0 + 0.1 { continue; } // or crossed it somewhere else than this opening
            let q = 0.35 / near;
            let at: V3 = [fc[0] + (p[0] - fc[0]) * q, 1.1, fc[2] + (p[2] - fc[2]) * q]; // the doorway as he sees it
            if crate::scene_collision::segment_blocked(g, eye, at, None, None, None) { continue; }
            if let Some(ar) = arc { if m::abs(v3::wrap_angle(v3::yaw_to(p, at) - ar)) >= r_k.arc_half_deg * crate::v3::DEG { continue; } }
            bd = near;
            best = Some(RoomFightDoorShot { by: r.by, at, door: d });
        }
    }
    best
}

/// In the dark on a lit man, Sam in the line to him, and an order's walk still to make: his feet wait; the order itself is untouched. (roomfight.ts holdShort)
pub fn room_fight_hold_short(g: &mut crate::game::Game, o: RoomfightOpId) {
    let gd = g.tab.roomfight_op[o].gd;
    let p = g.ch[g.gd[gd].ch].pos;
    let h = match g.tab.roomfight_op[o].tgt { Some(h) => h, None => return };
    let script = match g.gd[gd].script.as_ref() { Some(s) => s, None => return };
    let goal = match script.goal { Some(goal) => goal, None => return };
    if script.arrived.unwrap_or(false) || g.tab.roomfight_op[o].step.is_some() { return; }
    let hp = g.ch[g.gd[h].ch].pos;
    if flat(goal, hp) > flat(p, hp) - 0.3 || !crate::guards::gun_on_lit_man(g, gd) || room_fight_funnel_of(g, p).is_some() { return; }
    if g.time - g.tab.roomfight_op[o].short_seen > 1.0 || g.time < g.tab.roomfight_op[o].short_seen {
        g.tab.roomfight_op[o].short_since = g.time;
        let text = alloc::format!("holds short of {} in the dark: the player in his line", g.gd[h].callsign);
        room_fight_note(g, Some(gd), &text);
    }
    g.tab.roomfight_op[o].short_seen = g.time;
    if g.time - g.tab.roomfight_op[o].short_since < g.k.roomfight.hold_short_secs { g.tab.roomfight_op[o].short_t = g.time; }
}

/// Held off his man for a friend in the line: a hostile who has fired from where he has a line takes his eyes, the masked man let go for two seconds. (roomfight.ts letGo)
pub fn room_fight_let_go(g: &mut crate::game::Game, o: RoomfightOpId) {
    let gd = g.tab.roomfight_op[o].gd;
    let c = g.gd[gd].ch;
    let let_go = g.k.roomfight.let_go;
    if !(let_go > 0.0) || g.tab.roomfight_op[o].line_t < let_go || g.tab.roomfight_op[o].step.is_some() { return; }
    let tgt = match g.tab.roomfight_op[o].tgt { Some(t) => t, None => return };
    let eye = eye_of(g, c);
    let mut who: Option<GuardId> = None;
    let mut bd = 16.0;
    let mut i = 0;
    while i < g.fight.reports.len() {
        let r = g.fight.reports[i];
        i += 1;
        if g.time - r.t > 1.5 || g.gd[r.by].state == crate::game::GuardState::Dead || r.by == tgt { continue; }
        let by = g.gd[r.by].ch;
        let dist = flat(g.ch[c].pos, g.ch[by].pos);
        if dist < bd {
            let at = chest_of(g, by);
            if !crate::guards::eye_blocked(g, gd, eye, at) { bd = dist; who = Some(r.by); }
        }
    }
    let who = match who { Some(w) => w, None => return };
    let text = alloc::format!("lets {} go (a friend in the line) for {}, who is shooting", g.gd[tgt].callsign, g.gd[who].callsign);
    room_fight_note(g, Some(gd), &text);
    g.tab.roomfight_op[o].pass = Some(OpPass { h: tgt, until: g.time + 2.0, no_shot: None });
    g.tab.roomfight_op[o].tgt = None;
    g.tab.roomfight_op[o].kept = None;
    g.tab.roomfight_op[o].lost_at = g.time;
    g.tab.roomfight_op[o].line_t = 0.0;
    g.tab.roomfight_op[o].track = 0.0;
    g.tab.roomfight_op[o].turn = Some(OpTurn { to: who, at: g.time, until: g.time + 2.5, pt: None, t0: None, door: None });
}

/// On his man with no round at him for a while, and an order's walk still to make: he says so, lets the man go and walks on. (roomfight.ts walkOn)
pub fn room_fight_walk_on(g: &mut crate::game::Game, o: RoomfightOpId) {
    let gd = g.tab.roomfight_op[o].gd;
    let r = g.k.roomfight;
    if !(r.no_shot_go > 0.0) || g.tab.roomfight_op[o].block_t < r.no_shot_go || g.tab.roomfight_op[o].step.is_some() { return; }
    let tgt = match g.tab.roomfight_op[o].tgt { Some(t) => t, None => return };
    match g.gd[gd].script.as_ref() {
        Some(s) if s.goal.is_some() && !s.arrived.unwrap_or(false) => {}
        _ => return,
    }
    let text = alloc::format!("no shot at {} for {} s: walks on", g.gd[tgt].callsign, js::to_fixed(g.tab.roomfight_op[o].block_t, 1));
    room_fight_note(g, Some(gd), &text);
    room_fight_speak(g, o, "no shot — moving.", Some(3.0));
    g.tab.roomfight_op[o].pass = Some(OpPass { h: tgt, until: g.time + r.no_shot_pass, no_shot: Some(true) });
    g.tab.roomfight_op[o].tgt = None;
    g.tab.roomfight_op[o].kept = None;
    g.tab.roomfight_op[o].lost_at = g.time;
    g.tab.roomfight_op[o].line_t = 0.0;
    g.tab.roomfight_op[o].block_t = 0.0;
    g.tab.roomfight_op[o].track = 0.0;
}

/// His muzzle back to the order: the overlay off his script. (roomfight.ts restore)
pub fn room_fight_restore(g: &mut crate::game::Game, o: RoomfightOpId) {
    if let Some(s) = g.gd[g.tab.roomfight_op[o].gd].script.as_mut() { if s.over.is_some() { s.over = None; } }
    g.tab.roomfight_op[o].track = 0.0;
}

/// A step of this module's: the order's legs kept, then the step ordered. (roomfight.ts stepTo)
pub fn room_fight_step_to(g: &mut crate::game::Game, o: RoomfightOpId, p: V3, door: bool, patch: crate::roomfight::RoomFightStepToPatch) {
    let gd = g.tab.roomfight_op[o].gd;
    let s = g.gd[gd].script.unwrap();
    match g.tab.roomfight_op[o].step {
        Some(mut st) if s.goal == Some(st.mine) => { st.mine = p; st.door = st.door || door; g.tab.roomfight_op[o].step = Some(st); }
        _ => g.tab.roomfight_op[o].step = Some(OpStep { mine: p, goal: s.goal, speed: s.speed, strafe: s.strafe, face: s.face, door }),
    }
    crate::squad::order(g, gd, crate::squad::OrderPatch {
        goal: Some(Some(p)), speed: patch.speed.map(Some), face: patch.face.map(Some), strafe: patch.strafe.map(Some), aim_at: patch.aim_at.map(Some), stance: patch.stance.map(Some),
        stack_side: patch.stack_side.map(Some), crouch: patch.crouch.map(Some), lean: patch.lean.map(Some), upper: patch.upper.map(Some), light: patch.light.map(Some),
        arrived: patch.arrived.map(Some), hot: patch.hot.map(Some), flinch: patch.flinch.map(Some), direct: patch.direct.map(Some), smoke: patch.smoke.map(Some), over: patch.over.map(Some), held: false,
    });
    crate::teampace::team_fight(g, gd, Some(2.0)); // the fight's own step keeps its pace
}

/// ...and handed back: the order as it stood before the step, if nobody has ordered him since. (roomfight.ts handBack)
pub fn room_fight_hand_back(g: &mut crate::game::Game, o: RoomfightOpId) {
    let b = g.tab.roomfight_op[o].step;
    g.tab.roomfight_op[o].step = None;
    let gd = g.tab.roomfight_op[o].gd;
    let b = match b { Some(b) => b, None => return };
    let goal = match g.gd[gd].script.as_ref() { Some(s) => s.goal, None => return };
    if goal != Some(b.mine) || g.gd[gd].state == crate::game::GuardState::Dead { return; }
    // Object.assign writes all four, undefined and null too: each is in the patch, so a goal of null is a new leg to nothing (squad.ts:146)
    crate::squad::order(g, gd, crate::squad::OrderPatch { goal: Some(b.goal), speed: Some(b.speed), strafe: Some(b.strafe), face: Some(b.face), ..Default::default() });
}

/// Stopped in an open doorway with the fight on: through it and off the opening on the side he faces, else back off it. (roomfight.ts doorway)
pub fn room_fight_doorway(g: &mut crate::game::Game, o: RoomfightOpId, dt: f64, speed: f64) {
    let gd = g.tab.roomfight_op[o].gd;
    let c = g.gd[gd].ch;
    let sk = skill_operative(g);
    if g.time < g.tab.roomfight_op[o].step_until { g.tab.roomfight_op[o].funnel_t = 0.0; return; }
    let pos = g.ch[c].pos;
    let d = if speed < 0.4 { room_fight_funnel_of(g, pos) } else { None };
    // a man in his sight, a mate's muzzle on one, or a shot / a charge in HIS ears just now
    let fight = g.tab.roomfight_op[o].tgt.is_some() || g.fight.ops.iter().any(|&x| g.tab.roomfight_op[x].tgt.is_some()) || g.time - g.tab.roomfight_op[o].hot_t < 3.0;
    let d = match d { Some(d) if fight => d, _ => { g.tab.roomfight_op[o].funnel_t = 0.0; return; } };
    g.tab.roomfight_op[o].funnel_t += dt;
    if g.tab.roomfight_op[o].funnel_t < sk.funnel { return; }
    let f = crate::guards::frame_of(g, d);
    let fw = crate::character::character_forward(g, c);
    let sgn = if fw[0] * f.nx + fw[2] * f.nz >= 0.0 { 1.0 } else { -1.0 };
    for side in [sgn, -sgn] {
        for lat in [f.w + 0.65, -0.65] {
            let p: V3 = [f.hx + f.ux * lat + f.nx * side * 1.05, 0.0, f.hz + f.uz * lat + f.nz * side * 1.05];
            if !room_fight_spot_free(g, p, gd) { continue; }
            room_fight_step_to(g, o, p, true, RoomFightStepToPatch { speed: Some(2.6), strafe: Some(false), ..Default::default() });
            g.tab.roomfight_op[o].step_until = g.time + 1.5;
            g.tab.roomfight_op[o].funnel_t = 0.0;
            g.fight.stats.stepped += 1.0;
            let text = alloc::format!("steps off {}'s opening", g.dr[d].def.name);
            room_fight_note(g, Some(gd), &text);
            return;
        }
    }
    g.tab.roomfight_op[o].step_until = g.time + 0.5;
}

/// A friend has stood in his line a while (or his own gun is in a wall) and he is planted: a step to the side that opens it. True: he steps. (roomfight.ts sideStep)
pub fn room_fight_side_step(g: &mut crate::game::Game, o: RoomfightOpId, to: V3, speed: f64, waited: Option<f64>, why: Option<&str>) -> bool {
    let waited = waited.unwrap_or(g.tab.roomfight_op[o].line_t);
    let why = why.unwrap_or("steps round a friend in his line");
    let gd = g.tab.roomfight_op[o].gd;
    let c = g.gd[gd].ch;
    let sk = skill_operative(g);
    let walking = match g.gd[gd].script.as_ref() {
        Some(s) => s.goal.is_some() && !s.arrived.unwrap_or(false),
        None => return false,
    };
    if waited < sk.line || speed > 0.3 || g.time < g.tab.roomfight_op[o].step_until || walking { return false; }
    let pos = g.ch[c].pos;
    let dx = to[0] - pos[0];
    let dz = to[2] - pos[2];
    let l = js::or(m::hyp2(dx, dz), 1.0);
    for w in [0.9, -0.9, 1.6, -1.6, 2.4, -2.4] {
        let p: V3 = [pos[0] + dz / l * w, 0.0, pos[2] - dx / l * w];
        if !room_fight_spot_free(g, p, gd) || room_fight_funnel_of(g, p).is_some() || crate::scene_collision::solid_blocked(g, [pos[0], 0.45, pos[2]], [p[0], 0.45, p[2]], None) { continue; }
        let from: V3 = [p[0], 1.4, p[2]];
        let def = g.fa[g.gd[gd].weapon].def;
        let energy = crate::glasspen::shot_energy(g, c, Some(def));
        if crate::scene_collision::shot_blocked(g, from, to, Some(0.02), energy) || room_fight_friend_in_line(g, from, to, gd).is_some() || crate::friendlyfire::friend_in_cone(g, gd, from, to, 0.08, None, None).is_some() { continue; }
        let face = g.ch[c].body_yaw;
        room_fight_step_to(g, o, p, false, RoomFightStepToPatch { speed: Some(2.2), strafe: Some(true), face: Some(face), ..Default::default() });
        g.tab.roomfight_op[o].step_until = g.time + 2.0;
        g.tab.roomfight_op[o].line_t = 0.0;
        g.fight.stats.stepped += 1.0;
        room_fight_note(g, Some(gd), why);
        return true;
    }
    g.tab.roomfight_op[o].step_until = g.time + 1.0;
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A man on the tables at the origin facing +z (yaw 0), carrying this module's overlay, and his Op.
    fn man_with_overlay() -> (Game, GuardId, RoomfightOpId) {
        let mut g = Game::blank();
        let c = g.ch.push(crate::character::Character::blank());
        let gd = g.gd.push(crate::game::Guard::blank());
        g.gd[gd].ch = c;
        g.gd[gd].script = Some(crate::game::GuardScript { over: Some(crate::game::GuardScriptOver { aim_at: [0.0; 3], face: None, walk: None }), ..Default::default() });
        g.ch[c].pos = [0.0; 3];
        g.ch[c].vel = [0.0; 3];
        g.ch[c].body_yaw = 0.0;
        let o = room_fight_new_op(&mut g, gd);
        (g, gd, o)
    }
    fn at_bearing(a: f64) -> V3 { [m::sin(a) * 5.0, 0.0, m::cos(a) * 5.0] }

    /// `s.over` IS `O.ov` and its aimAt IS O.aim in the TypeScript: what the runner reads must follow every write of `point`.
    #[test]
    fn the_scripts_overlay_follows_the_op() {
        let (mut g, gd, o) = man_with_overlay();
        room_fight_point(&mut g, o, at_bearing(2.0), 1.0 / 60.0, 0.0);
        let op = &g.tab.roomfight_op[o];
        let over = g.gd[gd].script.as_ref().unwrap().over.unwrap();
        assert_ne!(op.aim, [0.0; 3]);
        assert_eq!(over.aim_at, op.aim);
        assert_eq!(over.face, op.ov.face);
        assert_eq!(over.walk, op.ov.walk);
        room_fight_restore(&mut g, o);
        assert!(g.gd[gd].script.as_ref().unwrap().over.is_none());
    }

    /// D.pt moved in place carries a cover or a fix that IS it, with no new fix; a point given once and a noise stay.
    #[test]
    fn the_post_moved_takes_what_is_on_it() {
        let (mut g, gd, _) = man_with_overlay();
        let d = g.tab.roomfight_def.push(Def::default());
        g.tab.roomfight_def[d].gd = gd;
        g.tab.roomfight_def[d].pt = [1.0, 0.0, 1.0];
        g.gd[gd].covers = Some([1.0, 0.0, 1.0]);
        g.gd[gd].last_known = Some([1.0, 0.0, 1.0]); // the same bits, but no stamp: a noise filed there is not the post
        move_pt(&mut g, d, [2.0, 0.0, 2.0]);
        assert_eq!(g.gd[gd].covers, Some([2.0, 0.0, 2.0]));
        assert_eq!(g.gd[gd].last_known, Some([1.0, 0.0, 1.0]));
    }

    /// Stood still: he turns once a point is past 1.2 rad off his chest, and finishes the turn down to 0.3 rad.
    #[test]
    fn a_stood_man_turns_and_finishes_the_turn() {
        let (mut g, _, o) = man_with_overlay();
        let dt = 1.0 / 60.0;
        room_fight_point(&mut g, o, at_bearing(1.0), dt, 0.0);
        assert_eq!(g.tab.roomfight_op[o].ov.face, None);
        let brg = room_fight_point(&mut g, o, at_bearing(1.3), dt, 0.0);
        assert_eq!(g.tab.roomfight_op[o].ov.face, Some(brg));
        let brg = room_fight_point(&mut g, o, at_bearing(0.5), dt, 0.0);
        assert_eq!(g.tab.roomfight_op[o].ov.face, Some(brg));
        room_fight_point(&mut g, o, at_bearing(0.2), dt, 0.0);
        assert_eq!(g.tab.roomfight_op[o].ov.face, None);
        assert_eq!(g.tab.roomfight_op[o].ov.walk, Some(false));
    }

    /// Walking: the test is on the way he is going, 1.2 rad to start facing and 0.7 rad to stop.
    #[test]
    fn a_walking_man_tests_his_path_not_his_chest() {
        let (mut g, gd, o) = man_with_overlay();
        let c = g.gd[gd].ch;
        g.ch[c].vel = [0.0, 0.0, 2.0];
        g.ch[c].body_yaw = 3.0; // the chest is nowhere near the point: it must not matter
        let dt = 1.0 / 60.0;
        room_fight_point(&mut g, o, at_bearing(1.0), dt, 2.0);
        assert_eq!(g.tab.roomfight_op[o].ov.face, None);
        assert_eq!(g.tab.roomfight_op[o].ov.walk, Some(false));
        let brg = room_fight_point(&mut g, o, at_bearing(1.3), dt, 2.0);
        assert_eq!(g.tab.roomfight_op[o].ov.face, Some(brg));
        assert_eq!(g.tab.roomfight_op[o].ov.walk, Some(true));
        let brg = room_fight_point(&mut g, o, at_bearing(1.0), dt, 2.0);
        assert_eq!(g.tab.roomfight_op[o].ov.face, Some(brg));
        room_fight_point(&mut g, o, at_bearing(0.5), dt, 2.0);
        assert_eq!(g.tab.roomfight_op[o].ov.face, None);
    }
}
