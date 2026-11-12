use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// the order's row
macro_rules! o { ($g:ident, $op:expr) => { $g.tab.doortools_door_op[$op] } }
/// an assignment's row
macro_rules! asg { ($g:ident, $a:expr) => { $g.tab.task[$a] } }
/// the order's threshold (`op.thr!`)
macro_rules! thr { ($g:ident, $op:expr) => { $g.tab.doortools_door_op[$op].thr.as_mut().unwrap() } }
/// a man's slice (`op.slice.get(gd)!`)
macro_rules! sl { ($g:ident, $op:expr, $gd:expr) => { $g.tab.doortools_door_op[$op].slice.get_mut($gd).unwrap() } }
/// a man's cuts (`s.pie!`)
macro_rules! pie { ($g:ident, $op:expr, $gd:expr) => { $g.tab.doortools_door_op[$op].slice.get_mut($gd).unwrap().pie.as_mut().unwrap() } }
/// a search's row (`SR`)
macro_rules! sr { ($g:ident, $s:expr) => { $g.tab.doortools_room_search[$s] } }

mod cuts;
use cuts::step_thr;
mod blind;
use blind::step_pocket;

/// what the head of `step` works out once
struct Cx { d: DoorId, t: f64, el: f64, lead: GuardId, name: String }

fn in_ops(seat: &crate::seats::SeatRef, o: DoorOpId) -> bool { seat.door_tools.ops.contains(&o) }
/// `this.lights ?? NIGHTBREACH.opLights` (settled: on)
pub(super) fn light_of(seat: &crate::seats::SeatRef) -> bool { seat.door_tools.lights.unwrap_or(true) }
pub(crate) fn is_pre(p: Phase) -> bool { matches!(p, Phase::Stack | Phase::Ready | Phase::Place | Phase::Standoff | Phase::Armed) }

/// `{ post: P.pos, face: P.face, watch: the frame, stance: 'highReady', upper: 'none', move, stackSide, going, there }`: stood off from the charge
pub(super) fn standing_off(g: &Game, d: DoorId, p: &crate::orders::StackPost, name: &str) -> DoorToolsAssignA {
    let c = g.dr[d].frame_centre;
    DoorToolsAssignA {
        post: Some(p.pos), face: Some(p.face), watch: Some([c[0], 1.05, c[2]]), stance: Some(crate::character::Stance::HighReady), upper: Some(crate::character::UpperMode::None),
        move_: Some(DoorToolsDoneItemLaneBackMove { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim }),
        stack_side: Some(p.stack_side), going: crate::orders::Doing::new(crate::orders::At::StandingOff, "standing off"), there: format!("stood off · {}", name).into(), ..Default::default()
    }
}
/// `stowFight(g, lead, A, op)`: the order's `stowOffT` lent to it and written back
fn stow_fight_of(g: &mut Game, gd: GuardId, a: crate::workstow::StowFightA, op: DoorOpId) -> bool {
    let mut s = crate::workstow::StowFightOp { stow_off_t: o!(g, op).stow_off_t };
    let r = crate::workstow::stow_fight(g, gd, Some(a), &mut s);
    o!(g, op).stow_off_t = s.stow_off_t;
    r
}

/// rear security while the door is worked
pub(super) fn door_tools_spare_rear(g: &mut crate::game::Game, op: DoorOpId) {
    let (men, door, lead, from) = { let r = &o!(g, op); (r.men.clone(), r.door, r.lead, r.from.clone()) };
    crate::orders::squad_orders_spare_rear(g, &men, door, Some(lead), Some(&from));
    let b = match &o!(g, op).lane { Some(l) => o!(g, op).mine.get(l.gd).copied(), None => None };
    if let Some(b) = b { asg!(g, b).spare = Some(false); }
}

/// is a walk to the marks still worth waiting on
fn door_tools_waiting(g: &crate::game::Game, op: DoorOpId, el: f64) -> bool {
    if el >= g.k.doortools.stack_cap { return false; }
    if el < g.k.doortools.still_secs { return true; }
    let men = o!(g, op).men.clone();
    men.iter().any(|&gd| { let a = o!(g, op).mine.get(gd).copied().unwrap(); !asg!(g, a).arrived && g.time - asg!(g, a).moved_t < g.k.doortools.still_secs })
}

/// somebody not yet on his mark is held by the fight, or blind
fn door_tools_busy(g: &crate::game::Game, op: DoorOpId, el: f64) -> bool {
    if !(el < g.k.doortools.stack_cap) { return false; }
    let men = o!(g, op).men.clone();
    men.iter().any(|&gd| { let a = o!(g, op).mine.get(gd).copied().unwrap(); !asg!(g, a).arrived && (asg!(g, a).yield_t >= 0.0 || crate::control::man::blind(g, gd)) })
}

/// the order is over
pub(super) fn door_tools_end(g: &mut crate::game::Game, op: DoorOpId, why: crate::doortools::Ended) {
    let i = match g.groups[g.group].tools.ops.iter().position(|&x| x == op) { Some(i) => i, None => return };
    g.groups[g.group].tools.ops.remove(i);
    door_tools_defuse(g, op);
    door_tools_off_lock(g, op);
    if o!(g, op).bang_held {
        o!(g, op).bang_held = false;
        let what = if o!(g, op).frag.unwrap_or(false) { crate::doortools::DoorToolsCan::Frag } else { crate::doortools::DoorToolsCan::Bang };
        door_tools_refund(g, what);
    }
    door_tools_pie_off(g, op);
    door_tools_beat(g, op, &format!("end: {}", why.as_str()));
    o!(g, op).ended = Some(why);
    g.groups[g.group].tools.done.push(op);
    if g.groups[g.group].tools.done.len() > 12 { g.groups[g.group].tools.done.remove(0); }
}

/// a charge order over short of its charge: the men still on it hold on that door
fn door_tools_short(g: &mut crate::game::Game, op: DoorOpId, why: crate::doortools::Ended) {
    let (d, side, men) = { let r = &o!(g, op); (r.door, r.side, r.men.clone()) };
    let c = g.dr[d].frame_centre;
    let open = is_open(g, d) || door_tools_known_open(g, d);
    let mut left: Vec<GuardId> = Vec::new();
    for &m in &men {
        let of = crate::orders::squad_orders_of(&crate::seats::here(g), m);
        if !crate::condition::down(g, g.gd[m].ch) && o!(g, op).post.has(m) && of.is_some() && of == o!(g, op).mine.get(m).copied() { left.push(m); }
    }
    let mut stay: JsSet<GuardId> = JsSet::new();
    if open {
        for &m in &left {
            let p = o!(g, op).post.get(m).unwrap().pos;
            let mp = g.pos_of(m);
            if v3::dist_xz(mp, p) > 0.6 && door_tools_crosses_open(g, d, side, mp, p) { stay.add(m); }
        }
    }
    for m in stay.keys() {
        let w = format!("hand kept · {}", g.gd[m].callsign);
        door_tools_beat(g, op, &w);
    }
    door_tools_end(g, op, why);
    if left.is_empty() { return; }
    crate::orders::squad_orders_new_group(g);
    for &m in &left {
        let post = o!(g, op).post.get(m).unwrap().clone();
        let mp = g.pos_of(m);
        let kept = stay.has(m);
        let at: V3 = if kept { [mp[0], 0.0, mp[2]] } else { post.pos };
        let words = crate::orders::door_words(g, d);
        let a = crate::duty::TaskGiven {
            order: String::from(crate::orders::rows::hold::ORDER.id), post: Some(at), face: Some(if kept { v3::yaw_to(at, c) } else { post.face }), watch: Some([c[0], 1.1, c[2]]),
            door: Some(d), shy: Some(d), stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim), light: Some(true),
            going: format!("to hold · {}", words).into(), there: format!("holding · {}", words).into(), ..Default::default()
        };
        crate::orders::squad_orders_assign(g, m, &a);
    }
}

/// the charge came off the door under the men stood off from it
fn door_tools_charge_gone(g: &mut crate::game::Game, op: DoorOpId) {
    let d = o!(g, op).door;
    let lead = o!(g, op).lead;
    let line = if g.dr[d].leaf.off { "door's gone. charge went with it." } else { "door's open. charge is off." };
    crate::game::game_say(g, lead, line, None, None);
    door_tools_short(g, op, crate::doortools::Ended::ChargeGone);
}

/// the leaf stands open in the team's picture of it
pub fn door_tools_known_open(g: &crate::game::Game, d: DoorId) -> bool {
    if crate::fogofwar::fog_of_war_door_held(g, d) { m::abs(crate::fogofwar::fog_of_war_door_shown(g, d)) >= OPEN_RAD } else { is_open(g, d) }
}

/// a canister this order threw and has not fired: left to its own short fuse
pub(super) fn door_tools_defuse(g: &mut crate::game::Game, op: DoorOpId) {
    let it = o!(g, op).can;
    if let (Some(it), Some(p)) = (it, o!(g, op).pass) {
        if !g.tab.item[it].fired {
            g.tab.item[it].pos = v3::copy(p.to);
            g.tab.item[it].vel = v3::copy(p.v0);
            g.tab.item[it].heading = v3::yaw_of(p.v0);
            g.tab.item[it].bounces = 0.0;
            g.tab.item[it].settled = false;
        }
    }
    o!(g, op).pass = None;
    if let Some(it) = it {
        if !g.tab.item[it].fired && g.tab.item[it].kind != crate::throwables::ItemKind::Frag {
            let f = m::min(g.tab.item[it].fuse, g.k.doortools.fuse_after_land);
            g.tab.item[it].fuse = f;
        }
    }
}

/// one tick of an order: the head, then a case a phase
pub(super) fn door_tools_step(g: &mut crate::game::Game, op: DoorOpId) {
    let phase = o!(g, op).phase;
    if crate::orders::squad_orders_sams(&crate::seats::here(g)) && (phase == Phase::Kick || is_pre(phase)) && door_tools_frag_step(g, op) { return; }
    let d = o!(g, op).door;
    let name = crate::orders::door_words(g, d);
    let cx = Cx { d, t: g.time, el: g.time - o!(g, op).t0, lead: o!(g, op).lead, name };
    if o!(g, op).thr.is_some() && o!(g, op).phase != Phase::Thr { door_tools_pie_off(g, op); }
    match o!(g, op).phase {
        Phase::Stack => step_stack(g, op, &cx),
        Phase::Ready => { if o!(g, op).word.is_none() { door_tools_open(g, op); } }
        Phase::Kick => step_kick(g, op, &cx),
        Phase::Toss => step_toss(g, op, &cx),
        Phase::Fuse => step_fuse(g, op, &cx),
        Phase::Wait => step_wait(g, op),
        Phase::Enter => step_enter(g, op, &cx),
        Phase::Place => step_place(g, op, &cx),
        Phase::Standoff => step_standoff(g, op, &cx),
        Phase::Armed => { if door_tools_charge_on(g, &crate::seats::here(g), d).is_none() { door_tools_charge_gone(g, op); } }
        Phase::Thr => step_thr(g, op, &cx),
        Phase::Pocket => step_pocket(g, op, &cx),
    }
}

/// the opening they are working stays open
fn keep_open(g: &mut Game, d: DoorId) {
    if g.dr[d].leaf.quiet_t > -1.0 && is_open(g, d) { g.dr[d].leaf.quiet_t = -1.0; }
}

/// the men walk to their marks; then the door is opened, or it waits on a word or a signal
fn step_stack(g: &mut Game, op: DoorOpId, cx: &Cx) {
    let (d, t, el, lead) = (cx.d, cx.t, cx.el, cx.lead);
    if o!(g, op).lane.is_some() {
        let (lgd, cover) = { let l = o!(g, op).lane.as_ref().unwrap(); (l.gd, l.cover) };
        let a = o!(g, op).mine.get(lgd).copied();
        let mine = a.is_some() && a == Some(cover) && !crate::condition::down(g, g.gd[lgd].ch) && o!(g, op).men.contains(&lgd);
        let fop = if mine { crate::roomfight::room_fight_op_of(g, lgd) } else { None };
        let mut hot = false;
        if let Some(f) = fop {
            for (h, mt) in g.tab.roomfight_op[f].meters.iter() {
                if !crate::condition::down(g, g.gd[h].ch) && t - mt.seen_t < g.k.doormanners.cover_quiet { hot = true; break; }
            }
        }
        let there = mine && { let r = &asg!(g, a.unwrap()); r.arrived || (hot && r.post.is_some() && v3::dist_xz(g.pos_of(lgd), r.post.unwrap()) < 0.5) };
        if mine {
            asg!(g, a.unwrap()).search_t = Some(t);
            let l = o!(g, op).lane.as_mut().unwrap();
            if there && l.at.is_none() { l.at = Some(t); }
        }
        let set = crate::condition::down(g, g.gd[lead].ch) || door_tools_arrived(g, op, lead) || !door_tools_waiting(g, op, el);
        let at = o!(g, op).lane.as_ref().unwrap().at;
        if !mine
            || (at.is_some() && t - at.unwrap() >= g.k.doormanners.cover_cap)
            || door_tools_last_call(g, op)
            || (!hot && asg!(g, a.unwrap()).arrived && t - asg!(g, a.unwrap()).arr_t >= g.k.doormanners.cover_look && set)
        {
            door_tools_lane_in(g, op);
        } else if there && (hot || door_tools_arrived(g, op, lead)) {
            return;
        }
    }
    if o!(g, op).ready_t.is_nan() {
        let last = door_tools_last_call(g, op);
        if last && !o!(g, op).last_stack.unwrap_or(false) {
            door_tools_last_stack(g, op);
            if o!(g, op).lead != lead { return; }
        }
        if !o!(g, op).last_stack.unwrap_or(false) {
            let men = o!(g, op).men.clone();
            if !men.iter().all(|&m| door_tools_arrived(g, op, m)) && (door_tools_waiting(g, op, el) || (o!(g, op).gate.is_some() && door_tools_busy(g, op, el))) { return; }
        }
        let lp = g.pos_of(lead);
        let mark = o!(g, op).post.get(lead).map_or(lp, |p| p.pos);
        if v3::dist_xz(lp, mark) > 1.5 {
            crate::game::game_say(g, lead, "can't get on that door.", None, None);
            door_tools_end(g, op, crate::doortools::Ended::NoStack);
            return;
        }
        if o!(g, op).lane.is_some() { door_tools_lane_in(g, op); return; }
        o!(g, op).ready_t = t;
        door_tools_beat_is(g, op, crate::doortools::Beat::Stacked, "stacked");
        let men = o!(g, op).men.clone();
        if !(o!(g, op).big.unwrap_or(false) && door_tools_big_opens(g, d, &men)) && door_tools_halt_at(g, op) { return; }
    }
    if o!(g, op).word.is_some() {
        o!(g, op).phase = Phase::Ready;
        o!(g, op).hold = 1.0;
        o!(g, op).t0 = t;
        door_tools_beat(g, op, "ready");
        return;
    }
    if let Some(gt) = o!(g, op).gate {
        let gops = g.tab.doortools_door_gate[gt].ops.clone();
        if g.tab.doortools_door_gate[gt].open_t.is_nan() {
            if !g.tab.doortools_door_gate[gt].go { return; }
            for &x in &gops {
                if in_ops(&crate::seats::here(g), x) && (o!(g, x).ready_t.is_nan() || door_tools_frag_wait(g, x)) { return; }
            }
            g.tab.doortools_door_gate[gt].open_t = t;
            for &x in &gops { if in_ops(&crate::seats::here(g), x) { door_tools_frag_on(g, x); } }
        }
        let gops = g.tab.doortools_door_gate[gt].ops.clone();
        let beside: Vec<DoorOpId> = gops.iter().copied().filter(|&x| x != op && in_ops(&crate::seats::here(g), x)).collect();
        if !door_tools_will_kick(g, op) && door_tools_boot_beside(g, op) {
            let side = o!(g, op).side;
            if !g.groups[g.group].tools.gate_asked.has(op) && !is_open(g, d) && !door_tools_locked_out(g, d, side) {
                g.groups[g.group].tools.gate_asked.add(op);
                door_tools_to_handle(g, op, true);
            }
            return;
        }
        if o!(g, op).kind == DoorToolKind::OpenClear && beside.iter().any(|&x| {
            let r = &o!(g, x);
            r.kind == DoorToolKind::BangClear && matches!(r.phase, Phase::Stack | Phase::Kick | Phase::Toss | Phase::Fuse | Phase::Wait)
        }) { return; }
    }
    door_tools_open(g, op);
}

/// the boot, or the tool the lock asked for
fn step_kick(g: &mut Game, op: DoorOpId, cx: &Cx) {
    let (d, t, el, lead) = (cx.d, cx.t, cx.el, cx.lead);
    let k0 = o!(g, op).kick;
    if let Some(k) = k0 {
        if k.pick.unwrap_or(false) { door_tools_pick_step(g, op); return; }
        if k.gun.is_some() { door_tools_gun_step(g, op); return; }
    }
    let k = match k0 { Some(k) => k, None => return };
    if k.hand.unwrap_or(false) { door_tools_hand_step(g, op, el); return; }
    let c = g.gd[lead].ch;
    let kind = o!(g, op).kind;
    if is_open(g, d) && k.t.is_nan() {
        door_tools_back_to_jamb(g, op, lead);
        if o!(g, op).word.is_some() && kind != DoorToolKind::BangClear { o!(g, op).phase = Phase::Ready; o!(g, op).hold = 2.0; return; }
        door_tools_opened(g, op);
        return;
    }
    if k.t.is_nan() {
        let cp = g.ch[c].pos;
        let on = v3::dist_xz(cp, k.pos) < 0.4 && m::abs(v3::wrap_angle(g.ch[c].body_yaw - k.yaw)) < 0.35;
        if !on && el < g.k.doortools.kick_cap { return; }
        if v3::dist_xz(cp, k.pos) > 1.2 {
            crate::game::game_say(g, lead, "can't get on the door.", None, None);
            door_tools_end(g, op, crate::doortools::Ended::NoKickSpot);
            return;
        }
        if o!(g, op).word.is_some() && kind != DoorToolKind::BangClear { o!(g, op).hold = 2.0; return; }
        if el < g.k.doortools.kick_cap {
            if let Some(gt) = o!(g, op).gate {
                let gops = g.tab.doortools_door_gate[gt].ops.clone();
                if gops.iter().any(|&x| x != op && in_ops(&crate::seats::here(g), x) && o!(g, x).phase == Phase::Kick && o!(g, x).kick.map_or(false, |q| q.gate.unwrap_or(false) && !q.on_it.unwrap_or(false))) { return; }
            }
        }
        crate::character::anim_kick_door(g, c, None);
        o!(g, op).kick.as_mut().unwrap().t = t;
        return;
    }
    if !(crate::character::anim_kick_impact(g, c) || crate::character::anim_kick_time(g, c) >= crate::character::KICK_IMPACT || !crate::character::anim_kicking(g, c)) { return; }
    let (cp, cid) = (g.ch[c].pos, g.ch[c].id);
    let gave = crate::doors::doors_kick_in(g, d, cp, cid, None);
    g.groups[g.group].tools.stats.kicks += 1.0;
    door_tools_beat_is(g, op, crate::doortools::Beat::Kick, "kick");
    door_tools_back_to_jamb(g, op, lead);
    if !gave && !is_open(g, d) {
        o!(g, op).kick = None;
        door_tools_beat(g, op, "the frame held");
        let side = o!(g, op).side;
        if !door_tools_frame_held(g, d, side) || !door_tools_frame_answer(g, op) { door_tools_end(g, op, crate::doortools::Ended::HeldTheBoot); }
        return;
    }
    door_tools_opened(g, op);
}

/// the can (or the frag): asked again as it leaves the hand, then thrown
fn step_toss(g: &mut Game, op: DoorOpId, cx: &Cx) {
    let (d, t, lead) = (cx.d, cx.t, cx.lead);
    let kicked = o!(g, op).kick.map_or(false, |k| !k.t.is_nan());
    let angle = g.dr[d].leaf.angle;
    let (was_a, was_t) = (o!(g, op).leaf_a.replace(angle), o!(g, op).leaf_t.replace(t));
    let open_for = t - o!(g, op).open_t;
    let (after_open, turn) = (g.k.doortools.toss_after_open, g.k.doortools.toss_turn);
    if open_for < (if kicked { g.k.doortools.toss_after_kick } else { after_open }) { return; }
    let frag = o!(g, op).frag.unwrap_or(false);
    if toss_held(g, op, cx, frag) { return; }
    let th = thrower_of(g, op, cx, kicked);
    let (yaw, to) = (g.ch[g.gd[th].ch].aim_yaw, v3::yaw_to(g.pos_of(th), g.dr[d].frame_centre));
    if turn > 0.0 && th != lead && !kicked && open_for < after_open + turn && m::abs(v3::wrap_angle(yaw - to)) > 1.2 { return; }
    if frag || o!(g, op).thr.is_none() {
        let vel = match (was_t, was_a) { (Some(lt), Some(la)) if t > lt => (angle - la) / (t - lt), _ => g.dr[d].leaf.vel };
        if !way_for_can(g, d, o!(g, op).side, vel) { leaf_in_way(g, op, t, th, frag); return; }
    }
    tossed(g, op, cx, th, frag);
}

/// Is the way through the frame open to a can: the leaf far enough out of it, or ajar and swinging away.
fn way_for_can(g: &Game, d: DoorId, side: f64, vel: f64) -> bool {
    let (angle, away) = (g.dr[d].leaf.angle, -side);
    m::abs(angle) >= g.k.doortools.frag_way_open || (angle * away >= g.k.doortools.frag_way_ajar && vel * away > 0.0)
}

/// It is not thrown after all: there is none, or one of their own is in there. They go in without it. True: so it was.
fn toss_held(g: &mut Game, op: DoorOpId, cx: &Cx, frag: bool) -> bool {
    let (d, lead, side) = (cx.d, cx.lead, o!(g, op).side);
    if !o!(g, op).bang_held {
        crate::game::game_say(g, lead, if frag { "no frag." } else { "no bang." }, None, None);
        door_tools_end(g, op, crate::doortools::Ended::NoBang);
        return true;
    }
    if frag {
        let men = o!(g, op).men.clone();
        let who = if crate::mission::captives_up(g).is_empty() { door_tools_frag_behind(g, d, side, &men) } else { Some(String::from("hostages")) };
        let Some(who) = who else { return false };
        let line = match who.as_str() { "hostages" => String::from("hostages — no frag. going in."), "you" => String::from("you're in there — no frag. going in."), _ => format!("{who} is in there — no frag. going in.") };
        crate::game::game_say(g, lead, &line, None, None);
        o!(g, op).frag = Some(false);
        can_back(g, op, crate::doortools::DoorToolsCan::Frag);
        door_tools_beat(g, op, "frag held");
        door_tools_enter(g, op);
        return true;
    }
    let (line, beat) = if door_tools_sam_behind(g, d, side) { (String::from("you're in there — no bang. going in."), String::from("no bang · you are in there")) }
        else if let Some(mate) = door_tools_mate_behind(g, op) { let who = &g.gd[mate].callsign; (format!("{who} is in there, so no bang. going in."), format!("no bang · {who} is in there")) }
        else { return false };
    crate::game::game_say(g, lead, &line, None, None);
    can_back(g, op, crate::doortools::DoorToolsCan::Bang);
    door_tools_beat(g, op, &beat);
    if o!(g, op).word.is_some() { o!(g, op).phase = Phase::Wait; o!(g, op).hold = 2.0; } else { door_tools_enter(g, op); }
    true
}
fn can_back(g: &mut Game, op: DoorOpId, can: crate::doortools::DoorToolsCan) { o!(g, op).bang_held = false; door_tools_refund(g, can); }

/// Who throws it: the nearest to the frame of those on this side of it who are not the first in; with nobody near, he does.
fn thrower_of(g: &mut Game, op: DoorOpId, cx: &Cx, kicked: bool) -> GuardId {
    let (d, side) = (cx.d, o!(g, op).side);
    let first = if kicked { cx.lead } else { let f = door_tools_first_in(g, op, None, None); o!(g, op).first = Some(f); f };
    let fc = g.dr[d].frame_centre;
    let mut near: Vec<(GuardId, f64)> = Vec::new();
    for m_ in o!(g, op).men.clone() {
        if m_ == first || crate::condition::down(g, g.gd[m_].ch) { continue; }
        let p = g.pos_of(m_);
        if off_of(g, d, p, side) > 0.0 && (kicked || v3::dist_xz(p, fc) < g.k.doortools.throw_reach) { near.push((m_, v3::dist_xz(p, fc))); }
    }
    js::sort_by(&mut near, |a, b| a.1 - b.1);
    near.first().map_or(first, |x| x.0)
}

/// The leaf is in the way: waited for, then they go in without.
fn leaf_in_way(g: &mut Game, op: DoorOpId, t: f64, th: GuardId, frag: bool) {
    let (what, tag, can) = if frag { ("frag", "frag", crate::doortools::DoorToolsCan::Frag) } else { ("bang", "can", crate::doortools::DoorToolsCan::Bang) };
    let since = match o!(g, op).frag_way_t {
        Some(x) => x,
        None => {
            o!(g, op).frag_way_t = Some(t);
            let angle = g.dr[o!(g, op).door].leaf.angle;
            door_tools_beat(g, op, &format!("{} waits: the door is in the way ({} rad)", tag, js::to_fixed(angle, 2)));
            t
        }
    };
    if t - since < g.k.doortools.frag_way_wait { return; }
    crate::game::game_say(g, th, &format!("the door's in the way — no {}. going in.", what), None, None);
    o!(g, op).frag = Some(false);
    can_back(g, op, can);
    door_tools_beat(g, op, &format!("{} held", tag));
    door_tools_enter(g, op);
}

/// It goes: in through the frame from his hand, or, for a can into the room from a man off the frame, passed to the frame first.
fn tossed(g: &mut Game, op: DoorOpId, cx: &Cx, th: GuardId, frag: bool) {
    let (d, side) = (cx.d, o!(g, op).side);
    let (p, w) = (g.pos_of(th), g.dr[d].def.width);
    o!(g, op).bang_held = false;
    let along = along_of(g, d, p);
    let from = door_pt(g, d, v3::clamp(along, 0.25, w - 0.25), 0.35, side, Some(1.3));
    let into = if !frag && o!(g, op).thr.is_none() { door_tools_bang_spot(g, op, from) } else { None };
    let land = match &into { Some(i) => i.at, None => { let deep = if frag { Some(g.k.frag.order_in) } else { None }; door_tools_land_at(g, op, deep) } };
    let sol = crate::throwables::throwables_solve(g, from, land, Some(9.0));
    let hand: V3 = [p[0] + (from[0] - p[0]) * 0.2, 1.3, p[2] + (from[2] - p[2]) * 0.2];
    let gap = v3::dist_xz(hand, from);
    let k = &g.k.doortools;
    let pass = if into.is_some() && gap > k.bang_hand_m && k.bang_pass_v > 0.0 && !crate::scene_collision::solid_blocked(g, hand, from, None) {
        Some(DoorToolsDoneItemPass { to: from, v0: sol.v0, t: m::max(0.15, gap / g.k.doortools.bang_pass_v) })
    } else { None };
    let (at0, v0): (V3, V3) = match &pass {
        Some(p) => (hand, [(from[0] - hand[0]) / p.t, (from[1] - hand[1] + 0.5 * crate::throwables::GRAVITY * p.t * p.t) / p.t, (from[2] - hand[2]) / p.t]),
        None => (from, sol.v0),
    };
    let passed = pass.is_some();
    o!(g, op).aim = into.as_ref().map(|i| DoorToolsDoneItemAim { at: i.at, from });
    o!(g, op).pass = pass;
    let (kind, at, v, fuse) = if frag { (crate::throwables::ItemKind::Frag, from, sol.v0, g.k.frag.order_fuse) } else { (crate::throwables::ItemKind::Flash, at0, v0, 99.0) };
    let it = leaves_hand(g, th, kind, at, v, fuse);
    if let Some(i) = &into {
        let inside = -off_of(g, d, i.at, side);
        let al = along_of(g, d, i.at) - w / 2.0;
        let tail = if passed { format!(" · passed {} m to the door", js::to_fixed(gap, 1)) } else { String::new() };
        door_tools_beat(g, op, &format!("can into the room · {} m in, {} along · {} of {} places lit{}", js::to_fixed(inside, 1), js::to_fixed(al, 1), i.n, i.of, tail));
    }
    let r = &mut o!(g, op);
    r.can = Some(it);
    r.toss_t = cx.t;
    r.threw = Some(th);
    r.phase = Phase::Fuse;
    let cs = g.gd[th].callsign.clone();
    door_tools_beat_is(g, op, crate::doortools::Beat::Toss, &format!("toss:{}", cs));
}

/// A can, or a frag, out of his hand, and his word with it.
fn leaves_hand(g: &mut Game, th: GuardId, kind: crate::throwables::ItemKind, at: V3, v: V3, fuse: f64) -> ItemId {
    let c = g.gd[th].ch;
    let who = g.ch[c].id;
    let it = crate::throwables::throwables_spawn(g, kind, at, v, Some(crate::throwables::ThrowablesSpawnOpts { fuse: Some(fuse), who: Some(who), heading: Some(v3::yaw_of(v)), ..Default::default() }));
    g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::Can);
    crate::character::anim_toss_item(g, c, Some(crate::character::TOSS_QUICK));
    crate::game::game_say(g, th, if kind == crate::throwables::ItemKind::Frag { "frag out!" } else { "bang out!" }, None, None);
    it
}

/// in the air: it goes off when it should
fn step_fuse(g: &mut Game, op: DoorOpId, cx: &Cx) {
    let (d, t) = (cx.d, cx.t);
    let side = o!(g, op).side;
    let it = o!(g, op).can.unwrap();
    let frag = o!(g, op).frag.unwrap_or(false);
    let ip = g.tab.item[it].pos;
    if g.tab.item[it].fired {
        o!(g, op).bang_t = t;
        o!(g, op).bang_at = Some(v3::copy(ip));
        o!(g, op).phase = Phase::Wait;
        if frag {
            let off = off_of(g, d, ip, side);
            door_tools_beat(g, op, &format!("frag · {} {} m", if off < 0.0 { "inside" } else { "OUTSIDE" }, js::to_fixed(m::abs(off), 1)));
        } else { door_tools_beat_is(g, op, crate::doortools::Beat::Bang, "bang (its own fuse)"); }
        if frag { g.groups[g.group].tools.stats.frags += 1.0; door_tools_close_up(g, op, None); }
        if o!(g, op).thr.is_some() { door_tools_thr_banged(g, op); }
        return;
    }
    if frag {
        if !g.items.items.contains(&it) { door_tools_end(g, op, crate::doortools::Ended::FragGone); }
        return;
    }
    if let Some(p) = o!(g, op).pass {
        if t - o!(g, op).toss_t < p.t && v3::dist_xz(ip, p.to) > 0.12 && g.tab.item[it].bounces == 0.0 { return; }
        g.tab.item[it].pos = v3::copy(p.to);
        g.tab.item[it].vel = v3::copy(p.v0);
        g.tab.item[it].heading = v3::yaw_of(p.v0);
        g.tab.item[it].bounces = 0.0;
        g.tab.item[it].settled = false;
        let at = g.tab.item[it].pos;
        crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::CanisterBounce, Some(at), Some(0.5), None);
        o!(g, op).pass = None;
        return;
    }
    if o!(g, op).land_t.is_nan() && (g.tab.item[it].bounces > 0.0 || g.tab.item[it].settled) { o!(g, op).land_t = t; }
    let land_t = o!(g, op).land_t;
    let aim = o!(g, op).thr.as_ref().and_then(|x| x.aim).or(o!(g, op).aim.map(|x| x.at));
    let hand = o!(g, op).thr.as_ref().and_then(|x| x.from).or(o!(g, op).aim.map(|x| x.from));
    let air = match (aim, hand) {
        (Some(a), Some(h)) => land_t.is_nan() && (v3::dist_xz(ip, a) < 0.1 || v3::dist_xz(ip, h) >= v3::dist_xz(a, h) - 0.05),
        _ => land_t.is_nan() && g.tab.item[it].vel[1] < 0.0 && ip[1] <= g.k.doortools.air_h && off_of(g, d, ip, side) <= -g.k.doortools.air_in,
    };
    if air || (!land_t.is_nan() && t - land_t >= g.k.doortools.fuse_after_land) || t - o!(g, op).toss_t >= g.k.doortools.toss_cap { door_tools_bang(g, op, it); }
}

/// behind a bang or a charge: nobody crosses until the blind burst has run dry
fn step_wait(g: &mut Game, op: DoorOpId) {
    if !door_tools_burst_dry(g, op) { return; }
    if let Some(gt) = o!(g, op).gate {
        for x in g.tab.doortools_door_gate[gt].ops.clone() {
            if x != op && in_ops(&crate::seats::here(g), x) && o!(g, x).kind == DoorToolKind::BangClear && !o!(g, x).frag.unwrap_or(false)
                && (o!(g, x).phase == Phase::Fuse || (o!(g, x).phase == Phase::Wait && !door_tools_burst_dry(g, x))) { return; }
        }
    }
    if o!(g, op).word.is_some() { o!(g, op).hold = 2.0; return; }
    door_tools_enter(g, op);
}

/// the room's id and name, of the mission's own rooms, at a point (`roomOf` in the entry's end)
fn room_of(g: &Game, p: V3) -> Option<(String, String)> {
    if g.staged.rooms.is_empty() { return None; }
    g.level.room_at(p).map(|r| (r.id.clone(), r.name.clone()))
}

/// through the frame: corners, sectors, the floor looked over; then the order is called
fn step_enter(g: &mut Game, op: DoorOpId, cx: &Cx) {
    let el = cx.el;
    door_tools_threat_deal(g, op);
    next_through(g, op, cx);
    let sr = door_tools_search_of(g, op, None);
    if let Some(s) = sr { door_tools_look(g, op, s, None, None); }
    let others = g.k.doortools.other_door_m > 0.0;
    let other = others && door_tools_other_doors(g, op, Watched::Entry);
    let mut swept = true;
    let mut w = 0;
    while w < o!(g, op).men.len() {
        let gd = o!(g, op).men[w];
        w += 1;
        if !man_sweeps(g, op, cx.t, gd, sr, other) { swept = false; }
    }
    handover(g, op);
    let cap = entry_cap(g, op, cx, sr);
    if !o!(g, op).queue.is_empty() || (!swept && el < cap) { return; }
    if let Some(s) = sr {
        if swept && el < cap && door_tools_move_on(g, op, s) { return; }
        let w = { let r = &sr!(g, s); format!("searched · {} of {} cells unseen · {} moved on", r.left, r.pts.len(), r.moves) };
        door_tools_beat(g, op, &w);
    }
    threats_done(g, op, cx.t);
    sectors_held(g, op);
    if others {
        door_tools_other_doors(g, op, Watched::Kept);
        g.groups[g.group].tools.watches.push(op);
    }
    one_looks_back(g, op, cx.d);
    let left = match sr { Some(s) if sr!(g, s).left >= g.k.doortools.dead_cells => sr!(g, s).left, _ => 0.0 };
    room_called(g, op, cx.lead, left);
    if o!(g, op).thr.is_some() {
        if others { door_tools_other_doors(g, op, Watched::Seen); }
        door_tools_hold_doors(g, op);
    }
    door_tools_beat(g, op, "in");
    door_tools_end(g, op, crate::doortools::Ended::In);
}

/// The men still queued at the frame, the last first: each who may goes to his point.
fn next_through(g: &mut Game, op: DoorOpId, cx: &Cx) {
    for i in (0..o!(g, op).queue.len()).rev() {
        let q = o!(g, op).queue[i];
        if !crate::doorpath::door_path_may_go(g, op, crate::doorpath::DoorPathMayGoQ { gd: q.gd }, i as f64) { continue; }
        o!(g, op).queue.remove(i);
        if !o!(g, op).men.contains(&q.gd) { continue; }
        o!(g, op).pod.set(q.gd, q.pod);
        // (the order's own assignment shies from its door: this one, that goes through it, does not)
        let a = crate::duty::TaskGiven {
            order: String::from(o!(g, op).kind.as_str()), door: Some(cx.d), shy: None, post: Some(q.pod.pos), face: Some(q.pod.face), watch: None, thru: Some(cx.d), pace: Some(g.groups[g.group].orders.knobs.entry),
            stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim), light: Some(light_of(&crate::seats::here(g))), search: Some(true), going: format!("entering · {}", cx.name).into(), there: "holding the room".into(), ..Default::default()
        };
        crate::orders::squad_orders_assign(g, q.gd, &a);
        let mine = crate::orders::squad_orders_of(&crate::seats::here(g), q.gd).unwrap();
        o!(g, op).mine.set(q.gd, mine);
        crate::doorpath::door_path_sent(g, op, q.gd, crate::doorpath::DoorPathSentPod { pos: q.pod.pos });
    }
}

/// One of the men through the frame: his threat, his corner, his point, then the floor he looks over from it. True: he has done his part.
fn man_sweeps(g: &mut Game, op: DoorOpId, t: f64, gd: GuardId, sr: Option<DoortoolsRoomSearchId>, other: bool) -> bool {
    if o!(g, op).queue.iter().any(|q| q.gd == gd) { return false; }
    let Some(pod) = o!(g, op).pod.get(gd).copied() else { return false };
    let a = o!(g, op).mine.get(gd).copied().unwrap();
    asg!(g, a).search_t = Some(t);
    let dealt = o!(g, op).threat.as_ref().is_some_and(|x| x.deal.as_ref().is_some_and(|m_| m_.len() > 0));
    if dealt && door_tools_threat_step(g, op, gd, a, pod) { return false; }
    if door_tools_corner_step(g, op, gd, a, pod) || !door_tools_on_point(g, gd, a, pod) { return false; }
    let since = match o!(g, op).on_pod.get(gd).copied() { Some(x) => x, None => { o!(g, op).on_pod.set(gd, t); t } };
    if other && door_tools_on_other(g, op, gd) { return true; }
    match sr {
        Some(s) => if door_tools_scan(g, s, gd, a) { return false; },
        None => {
            let k = m::floor((t - since) / g.k.doortools.sweep_secs);
            if k < 2.0 {   // (a floor nobody has a model of: his muzzle to one hand, then the other)
                let (p, yaw) = (g.pos_of(gd), pod.face + (if k <= 0.0 { -g.k.doortools.sweep_rad } else { g.k.doortools.sweep_rad }));
                asg!(g, a).watch = Some([p[0] + m::sin(yaw) * 4.0, 1.2, p[2] + m::cos(yaw) * 4.0]);
                asg!(g, a).face = Some(yaw);
                return false;
            }
        }
    }
    door_tools_on_sector(g, gd, a, pod);
    true
}

/// A room worked from the threshold: the men at the jambs are in and on their points.
fn handover(g: &mut Game, op: DoorOpId) {
    let r = &mut o!(g, op);
    let Some(x) = r.thr.as_mut() else { return };
    if x.handed || !r.queue.is_empty() || !x.jamb.iter().all(|&m_| !r.men.contains(&m_) || r.on_pod.has(m_)) { return; }
    x.handed = true;
    door_tools_beat(g, op, "handover");
}

/// How long the entry is given. A floor gone through piece by piece has more, for each piece and for the fight it was in.
fn entry_cap(g: &mut Game, op: DoorOpId, cx: &Cx, sr: Option<DoortoolsRoomSearchId>) -> f64 {
    let Some(s) = sr else { return g.k.doortools.in_cap };
    let base = g.k.doortools.search_cap;
    if !door_tools_flows(g, op, s) { return base; }
    fight_clock(g, s, cx.t);
    let k = &g.k.doortools;
    let (secs, fight) = (k.flow_secs, k.flow_fight);
    let r = &mut sr!(g, s);
    let cap = base + ((r.flows.unwrap_or(0.0) + r.closes.unwrap_or(0.0)) * secs + m::min(r.fight_t.unwrap_or(0.0), fight));
    if cx.el >= base && cx.el < cap { r.late = Some(true); }
    cap
}
/// A search's clock of the time its men spent in a fight, run once a tick whichever order asks.
fn fight_clock(g: &mut Game, s: DoortoolsRoomSearchId, t: f64) {
    if sr!(g, s).fight_at == Some(t) { return; }
    let fights = g.groups[g.group].tools.ops.iter().any(|&x| {
        let r = &o!(g, x);
        r.search.flatten() == Some(s) && r.crossed.len() > 0 && r.men.iter().any(|&m_| g.gd[m_].script.as_ref().is_some_and(|c| c.over.is_some()))
    });
    let r = &mut sr!(g, s);
    if fights { r.fight_t = Some(r.fight_t.unwrap_or(0.0) + v3::clamp(t - r.fight_at.unwrap_or(t), 0.0, 0.05)); }
    r.fight_at = Some(t);
}

/// The threats dealt are done with, and a man who went sideways for one walks straight again.
fn threats_done(g: &mut Game, op: DoorOpId, t: f64) {
    let deal: Vec<GuardId> = o!(g, op).threat.as_ref().and_then(|x| x.deal.as_ref()).map_or(Vec::new(), |m_| m_.keys());
    for m_ in deal {
        let r = o!(g, op).threat.as_mut().unwrap().deal.as_mut().unwrap().get_mut(m_).unwrap();
        if !r.done_t.is_nan() { continue; }
        r.done_t = t;
        if !r.faced.unwrap_or(false) { continue; }
        if let Some(a) = o!(g, op).mine.get(m_).copied() { asg!(g, a).strafe = Some(false); }
    }
}

/// Each man in keeps his sector.
fn sectors_held(g: &mut Game, op: DoorOpId) {
    for gd in o!(g, op).men.clone() {
        let (Some(pod), Some(a)) = (o!(g, op).pod.get(gd).copied(), o!(g, op).mine.get(gd).copied()) else { continue };
        if o!(g, op).queue.iter().any(|q| q.gd == gd) { continue; }
        let face = door_tools_on_sector(g, gd, a, pod);
        g.groups[g.group].tools.held.set(gd, DoorToolsHeldValue { a, face, d: 0.0, hit_t: f64::NAN, arr_t: f64::NAN, out: None, door: None });
    }
}

/// Of three or more in, the last through who has a line to the door they came by keeps his gun on it.
fn one_looks_back(g: &mut Game, op: DoorOpId, d: DoorId) {
    let mut ins: Vec<GuardId> = o!(g, op).men.iter().copied().filter(|&m_| !crate::condition::down(g, g.gd[m_].ch) && g.groups[g.group].tools.held.has(m_)).collect();
    if ins.len() < 3 { return; }
    let others = g.k.doortools.other_door_m > 0.0;
    let at = door_pt(g, d, g.dr[d].def.width / 2.0, -0.2, o!(g, op).side, Some(1.2));
    let crossed = &o!(g, op).crossed;
    let when = |m_: GuardId| crossed.get(m_).copied().unwrap_or(f64::NEG_INFINITY);
    js::sort_by(&mut ins, |a, b| when(*b) - when(*a));
    for m_ in ins {
        let (a, door) = { let r = g.groups[g.group].tools.held.get(m_).unwrap(); (r.a, r.door) };
        let p = g.pos_of(m_);
        if crate::scene_collision::segment_blocked(g, [p[0], 1.2, p[2]], at, None, None, None) || (others && door.is_some()) { continue; }
        let yaw = v3::yaw_to(p, at);
        let far = v3::dist_xz(p, at) + g.k.doortools.last_out_m;
        asg!(g, a).watch = Some([p[0] + m::sin(yaw) * far, 1.2, p[2] + m::cos(yaw) * far]);
        asg!(g, a).face = Some(yaw);
        let r = g.groups[g.group].tools.held.get_mut(m_).unwrap();
        r.face = yaw;
        r.out = Some(true);
        door_tools_beat(g, op, &format!("door · {}", g.gd[m_].callsign.clone()));
        return;
    }
}

/// The lead's word on the room: by its name where the mission has rooms, else as the floor they are on; and whether anybody is still up in it.
fn room_called(g: &mut Game, op: DoorOpId, lead: GuardId, left: f64) {
    let first = if crate::orders::squad_orders_sams(&crate::seats::here(g)) { o!(g, op).pod.values().next().copied() } else { None };
    let call = if let Some((id, name)) = room_of(g, g.pos_of(lead)).or_else(|| first.and_then(|p| room_of(g, p.pos))) {
        let (up, seen) = hostiles_in(g, |g, p| g.level.room_at(p).is_some_and(|r| r.id == id));
        crate::squadnet::room_call(&name, left, up, seen)
    } else if let Some(r) = first.and_then(|p| crate::roomfight::room_fight_room_at(g, p.pos)) {
        let room = &g.tab.roomfight_room[r];
        let flood = crate::squadnet::InFloodR { cells: room.cells.clone(), entries: room.entries.iter().map(|e| crate::squadnet::InFloodREntriesItem { door: e.door }).collect() };
        let (up, seen) = hostiles_in(g, |g, p| crate::squadnet::in_flood(g, &flood, p));
        let open = open_floor(g, r);
        crate::squadnet::plain_call(open, left, up, seen)
    } else { None };
    if let Some(call) = call { crate::game::game_say(g, lead, &call, None, None); }
}
/// Is one of theirs up in there, and does the squad see one of those.
fn hostiles_in(g: &mut Game, inside: impl Fn(&mut Game, V3) -> bool) -> (bool, bool) {
    let mut ins: Vec<GuardId> = Vec::new();
    for x in g.guards.clone() {
        if !crate::condition::down(g, g.gd[x].ch) && crate::group::foe(g, x) && !crate::runner::runner_nobody(g, x) && inside(g, g.pos_of(x)) { ins.push(x); }
    }
    (!ins.is_empty(), ins.iter().any(|&x| crate::autopilot::in_sight(g, x)))
}

/// the charge: he is at the lock for setSecs, then it is on the door and he walks off to the stand-off
fn step_place(g: &mut Game, op: DoorOpId, cx: &Cx) {
    let (d, t, el, lead) = (cx.d, cx.t, cx.el, cx.lead);
    let side = o!(g, op).side;
    let fc = g.dr[d].frame_centre;
    let a = o!(g, op).mine.get(lead).copied().unwrap();
    let lp = g.pos_of(lead);
    let at = asg!(g, a).arrived && v3::dist_xz(lp, fc) < 1.6;
    if door_tools_known_open(g, d) || (is_open(g, d) && (at || !o!(g, op).set_t.is_nan())) {
        crate::game::game_say(g, lead, "door's open.", None, None);
        door_tools_short(g, op, crate::doortools::Ended::DoorOpened);
        return;
    }
    let sa = crate::workstow::StowFightA { yield_t: asg!(g, a).yield_t };
    if stow_fight_of(g, lead, sa, op) {
        o!(g, op).set_t = f64::NAN;
        o!(g, op).t0 = t;
        return;
    }
    if o!(g, op).set_t.is_nan() {
        if at {
            let lead_in = crate::workstow::mate_stow_lead(g);
            o!(g, op).set_t = t + lead_in;
        } else if !door_tools_waiting(g, op, el) {
            crate::game::game_say(g, lead, "can't get on that door.", None, None);
            door_tools_short(g, op, crate::doortools::Ended::NoDoor);
        }
        return;
    }
    if v3::dist_xz(lp, fc) > 1.6 { o!(g, op).set_t = f64::NAN; return; }
    crate::workstow::mate_stow(g, lead);
    let set_t = o!(g, op).set_t;
    let set_secs = g.k.doortools.set_secs;
    if t >= set_t {
        let o0 = door_pt(g, d, 0.0, 0.0, side, None);
        let n1 = door_pt(g, d, 0.0, 1.0, side, None);
        let width = g.dr[d].def.width;
        let on = door_pt(g, d, width - 0.22, 0.025, side, Some(1.05));
        let c = g.gd[lead].ch;
        crate::handwork::charge_hands(g, c, on, [n1[0] - o0[0], 0.0, n1[2] - o0[2]], t - set_t, set_secs, None);
    }
    if t - set_t < set_secs { return; }
    if !door_tools_take(g, crate::doortools::DoorToolsCan::Charge) {
        crate::game::game_say(g, lead, "no charges left.", None, None);
        door_tools_short(g, op, crate::doortools::Ended::NoCharges);
        return;
    }
    let by = g.gd[lead].callsign.clone();
    let c = g.tab.doortools_set_charge.push(SetCharge { door: d, side, by });
    g.groups[g.group].tools.set.push(c);
    let dp = crate::doors::door_pos(g, d);
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::LockPick, Some(dp), Some(0.6), None);
    crate::game::game_say(g, lead, "charge set.", None, None);
    door_tools_beat(g, op, "set");
    let p = o!(g, op).post.get(lead).cloned().unwrap();
    let aa = standing_off(g, d, &p, &cx.name);
    door_tools_assign(g, op, lead, &aa);
    o!(g, op).phase = Phase::Standoff;
    o!(g, op).t0 = t;
}

/// the men walk off the charge; then it is Sam's call
fn step_standoff(g: &mut Game, op: DoorOpId, cx: &Cx) {
    let (d, t, el, lead) = (cx.d, cx.t, cx.el, cx.lead);
    if door_tools_charge_on(g, &crate::seats::here(g), d).is_none() { door_tools_charge_gone(g, op); return; }
    let fc = g.dr[d].frame_centre;
    let men = o!(g, op).men.clone();
    let near: Vec<GuardId> = men.iter().copied().filter(|&m_| v3::dist_xz(g.pos_of(m_), fc) < g.k.doortools.backblast_m + 0.05).collect();
    let all = men.iter().all(|&m_| door_tools_arrived(g, op, m_));
    let wait = if !all { door_tools_waiting(g, op, el) } else {
        !near.is_empty() && el < g.k.doortools.stack_cap && (el < g.k.doortools.still_secs || near.iter().any(|&m_| {
            let a = o!(g, op).mine.get(m_).copied().unwrap();
            g.time - asg!(g, a).moved_t < g.k.doortools.still_secs
        }))
    };
    if wait { return; }
    crate::game::game_say(g, lead, "stood off. your call.", None, None);
    o!(g, op).phase = Phase::Armed;
    o!(g, op).t0 = t;
    door_tools_beat(g, op, "armed");
    if o!(g, op).word.is_some() { o!(g, op).hold = 1.0; }
}
