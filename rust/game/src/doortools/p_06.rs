//! src/game/doortools.ts lines 2146 to 2580
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
macro_rules! asg { ($g:ident, $a:expr) => { $g.tab.orders_assignment[$a] } }
/// the order's threshold (`op.thr!`)
macro_rules! thr { ($g:ident, $op:expr) => { $g.tab.doortools_door_op[$op].thr.as_mut().unwrap() } }
/// a man's slice (`op.slice.get(gd)!`)
macro_rules! sl { ($g:ident, $op:expr, $gd:expr) => { $g.tab.doortools_door_op[$op].slice.get_mut($gd).unwrap() } }
/// a man's cuts (`s.pie!`)
macro_rules! pie { ($g:ident, $op:expr, $gd:expr) => { $g.tab.doortools_door_op[$op].slice.get_mut($gd).unwrap().pie.as_mut().unwrap() } }
/// a search's row (`SR`)
macro_rules! sr { ($g:ident, $s:expr) => { $g.tab.doortools_room_search[$s] } }

/// what the head of `step` works out once
struct Cx { d: DoorId, t: f64, el: f64, lead: GuardId, name: String }

fn tr(b: Option<bool>) -> bool { b.unwrap_or(false) }
fn pos_of(g: &Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }
fn alive(g: &Game, gd: GuardId) -> bool { g.gd[gd].state != crate::game::GuardState::Dead }
fn say(g: &mut Game, gd: GuardId, text: &str) { crate::game::game_say(g, gd, text, None, None, None); }
fn in_ops(g: &Game, o: DoorOpId) -> bool { g.door_tools.ops.contains(&o) }
/// `this.lights ?? NIGHTBREACH.opLights` (settled: on)
fn light_of(g: &Game) -> bool { g.door_tools.lights.unwrap_or(true) }
fn is_pre(p: Phase) -> bool { matches!(p, Phase::Stack | Phase::Ready | Phase::Place | Phase::Standoff | Phase::Armed) }

/// a man's next mark on the cuts: `{ post, face, watch, stance: 'none', upper: 'aim', light, search, ...pace, ...lean, going, there }`
fn cut_a(g: &Game, post: V3, face: f64, watch: V3, search: Option<bool>, lean: Option<f64>, crouch: Option<bool>, going: String, there: String) -> DoorToolsAssignA {
    let k = &g.k.doortools;
    DoorToolsAssignA {
        post: Some(post), face: Some(face), watch: Some(watch), stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim),
        light: Some(light_of(g)), search, pace: if k.slice_pace > 0.0 { Some(k.slice_pace) } else { None }, lean, crouch, going, there, ..Default::default()
    }
}
/// `{ post: P.pos, face: P.face, watch: the frame, stance: 'highReady', upper: 'none', move, stackSide, going, there }`: stood off from the charge
fn standing_off(g: &Game, d: DoorId, p: &crate::orders::StackPost, name: &str) -> DoorToolsAssignA {
    let c = g.dr[d].frame_centre;
    DoorToolsAssignA {
        post: Some(p.pos), face: Some(p.face), watch: Some([c[0], 1.05, c[2]]), stance: Some(crate::character::Stance::HighReady), upper: Some(crate::character::UpperMode::None),
        move_: Some(DoorToolsDoneItemLaneBackMove { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim }),
        stack_side: Some(p.stack_side), going: String::from("standing off"), there: format!("stood off · {}", name), ..Default::default()
    }
}
/// `Object.assign(op, { ended })` as the list of done orders holds it: a copy of the row at this moment
fn done_of(op: &DoorOp, why: &str) -> DoorToolsDoneItem {
    let o = op.clone();
    DoorToolsDoneItem {
        kind: o.kind, door: o.door, side: o.side, coded: o.coded, men: o.men, lead: o.lead, fled: o.fled, fled_w: o.fled_w, fled_t: o.fled_t, fled_said: o.fled_said, fled_go: o.fled_go,
        rear: o.rear, lane: o.lane, from: o.from, phase: o.phase, t0: o.t0, bang_held: o.bang_held, frag: o.frag, frag_way_t: o.frag_way_t, leaf_a: o.leaf_a, leaf_t: o.leaf_t,
        mine: o.mine, post: o.post, kick: o.kick, no_gun: o.no_gun, big: o.big, open_t: o.open_t, can: o.can, toss_t: o.toss_t, land_t: o.land_t, bang_t: o.bang_t, bang_at: o.bang_at,
        queue: o.queue, set_t: o.set_t, stow_off_t: o.stow_off_t, pod: o.pod, on_pod: o.on_pod, slice: o.slice, crossed: o.crossed, corner: o.corner, search: o.search, first: o.first,
        threw: o.threw, aim: o.aim, pass: o.pass, beats: o.beats, on_word: o.on_word, word: o.word, hold: o.hold, ready_t: o.ready_t, gate: o.gate, thr: o.thr, thr_hold: o.thr_hold,
        thr_pre: o.thr_pre, worded: o.worded, last_stack: o.last_stack, last_hold: o.last_hold, last_tossed: o.last_tossed, other: o.other, threat: o.threat, ended: String::from(why),
    }
}

/// `stowFight(g, lead, A, op)`: the order's `stowOffT` lent to it and written back (workstow.ts writes it)
fn stow_fight_of(g: &mut Game, gd: GuardId, a: crate::workstow::StowFightA, op: DoorOpId) -> bool {
    let mut s = crate::workstow::StowFightOp { stow_off_t: o!(g, op).stow_off_t };
    let r = crate::workstow::stow_fight(g, gd, Some(a), &mut s);
    o!(g, op).stow_off_t = s.stow_off_t;
    r
}
/// the tail of the hold's beats: `SR ? ` · ${SR.left} of ${SR.pts.length} cells unseen` : ''`, read as the beat is said
fn unseen_of(g: &Game, sr: Option<DoortoolsRoomSearchId>) -> String {
    sr.map_or(String::new(), |s| format!(" · {} of {} cells unseen", sr!(g, s).left, sr!(g, s).pts.len()))
}

/// rear security while the door is worked (doortools.ts DoorTools.spareRear)
pub fn door_tools_spare_rear(g: &mut crate::game::Game, op: DoorOpId) {
    let (men, door, lead, from) = { let r = &o!(g, op); (r.men.clone(), r.door, r.lead, r.from.clone()) };
    crate::orders::squad_orders_spare_rear(g, &men, door, Some(lead), Some(&from));
    let b = match &o!(g, op).lane { Some(l) => o!(g, op).mine.get(l.gd).copied(), None => None };
    if let Some(b) = b { asg!(g, b).spare = Some(false); }
}

/// is a walk to the marks still worth waiting on (doortools.ts DoorTools.waiting)
pub fn door_tools_waiting(g: &mut crate::game::Game, op: DoorOpId, el: f64) -> bool {
    if el >= g.k.doortools.stack_cap { return false; }
    if el < g.k.doortools.still_secs { return true; }
    let men = o!(g, op).men.clone();
    men.iter().any(|&gd| { let a = o!(g, op).mine.get(gd).copied().unwrap(); !asg!(g, a).arrived && g.time - asg!(g, a).moved_t < g.k.doortools.still_secs })
}

/// somebody not yet on his mark is held by the fight, or blind (doortools.ts DoorTools.busy)
pub fn door_tools_busy(g: &mut crate::game::Game, op: DoorOpId, el: f64) -> bool {
    if !(el < g.k.doortools.stack_cap) { return false; }
    let men = o!(g, op).men.clone();
    men.iter().any(|&gd| { let a = o!(g, op).mine.get(gd).copied().unwrap(); !asg!(g, a).arrived && (asg!(g, a).yield_t >= 0.0 || g.time < g.gd[gd].dazzled_until) })
}

/// the order is over (doortools.ts DoorTools.end)
pub fn door_tools_end(g: &mut crate::game::Game, op: DoorOpId, why: &str) {
    let i = match g.door_tools.ops.iter().position(|&x| x == op) { Some(i) => i, None => return };
    g.door_tools.ops.remove(i);
    door_tools_defuse(g, op);
    door_tools_off_lock(g, op);
    if o!(g, op).bang_held {
        o!(g, op).bang_held = false;
        let what = if tr(o!(g, op).frag) { DoorToolsRefundWhat::Frag } else { DoorToolsRefundWhat::Bang };
        door_tools_refund(g, what);
    }
    door_tools_pie_off(g, op);
    door_tools_beat(g, op, &format!("end: {}", why));
    let item = done_of(&o!(g, op), why);
    g.door_tools.done.push(item);
    if g.door_tools.done.len() > 12 { g.door_tools.done.remove(0); }
}

/// a charge order over short of its charge: the men still on it hold on that door (doortools.ts DoorTools.short)
pub fn door_tools_short(g: &mut crate::game::Game, op: DoorOpId, why: &str) {
    if !g.k.doortools.charge_answer { door_tools_end(g, op, why); return; }
    let (d, side, men) = { let r = &o!(g, op); (r.door, r.side, r.men.clone()) };
    let c = g.dr[d].frame_centre;
    let open = is_open(g, d) || door_tools_known_open(g, d);
    let mut left: Vec<GuardId> = Vec::new();
    for &m in &men {
        let of = crate::orders::squad_orders_of(g, m);
        if alive(g, m) && o!(g, op).post.has(m) && of.is_some() && of == o!(g, op).mine.get(m).copied() { left.push(m); }
    }
    let mut stay: JsSet<GuardId> = JsSet::new();
    if open {
        for &m in &left {
            let p = o!(g, op).post.get(m).unwrap().pos;
            let mp = pos_of(g, m);
            if v3::dist_xz(mp, p) > 0.6 && door_tools_crosses_open(g, d, side, mp, p) { stay.add(m); }
        }
    }
    for m in stay.keys() {
        g.door_tools.stats.hand_kept += 1.0;
        let w = format!("hand kept · {}", g.gd[m].callsign);
        door_tools_beat(g, op, &w);
    }
    door_tools_end(g, op, why);
    if left.is_empty() { return; }
    crate::orders::squad_orders_new_group(g);
    for &m in &left {
        let post = o!(g, op).post.get(m).unwrap().clone();
        let mp = pos_of(g, m);
        let kept = stay.has(m);
        let at: V3 = if kept { [mp[0], 0.0, mp[2]] } else { post.pos };
        let words = crate::orders::door_words(g, d);
        let a = crate::orders::SquadOrdersAssignA {
            order: String::from("hold"), post: Some(at), face: Some(if kept { m::atan2(c[0] - at[0], c[2] - at[2]) } else { post.face }), watch: Some([c[0], 1.1, c[2]]),
            door: Some(d), shy: Some(d), stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim), light: Some(true),
            going: format!("to hold · {}", words), there: format!("holding · {}", words), ..Default::default()
        };
        crate::orders::squad_orders_assign(g, m, &a);
    }
}

/// the charge came off the door under the men stood off from it (doortools.ts DoorTools.chargeGone)
pub fn door_tools_charge_gone(g: &mut crate::game::Game, op: DoorOpId, hands: bool) {
    let d = o!(g, op).door;
    if !g.k.doortools.charge_answer {
        if hands && is_open(g, d) { door_tools_stay_hands(g, op); }
        door_tools_end(g, op, "the charge is gone");
        return;
    }
    let lead = o!(g, op).lead;
    let line = if crate::doors::door_off(g, d) { "door's gone. charge went with it." } else { "door's open. charge is off." };
    say(g, lead, line);
    door_tools_short(g, op, "the charge is gone");
}

/// the leaf stands open in the team's picture of it (doortools.ts DoorTools.knownOpen)
pub fn door_tools_known_open(g: &mut crate::game::Game, d: DoorId) -> bool {
    if crate::fogofwar::fog_of_war_door_held(g, d) { m::abs(crate::fogofwar::fog_of_war_door_shown(g, d)) >= OPEN_RAD } else { is_open(g, d) }
}

/// a canister this order threw and has not fired: left to its own short fuse (doortools.ts DoorTools.defuse)
pub fn door_tools_defuse(g: &mut crate::game::Game, op: DoorOpId) {
    let it = o!(g, op).can;
    if let (Some(it), Some(p)) = (it, o!(g, op).pass) {
        if !crate::throwables::item_fired(g, it) {
            crate::throwables::item_set_pos(g, it, v3::copy(p.to));
            crate::throwables::item_set_vel(g, it, v3::copy(p.v0));
            crate::throwables::item_set_heading(g, it, v3::yaw_of(p.v0));
            crate::throwables::item_set_bounces(g, it, 0.0);
            crate::throwables::item_set_settled(g, it, false);
        }
    }
    o!(g, op).pass = None;
    if let Some(it) = it {
        if !crate::throwables::item_fired(g, it) && crate::throwables::item_kind(g, it) != crate::throwables::ItemKind::Frag {
            let f = m::min(crate::throwables::item_fuse(g, it), g.k.doortools.fuse_after_land);
            crate::throwables::item_set_fuse(g, it, f);
        }
    }
}

/// one tick of an order: the head, then a case a phase (doortools.ts DoorTools.step)
pub fn door_tools_step(g: &mut crate::game::Game, op: DoorOpId) {
    let phase = o!(g, op).phase;
    if crate::orders::squad_orders_sams(g) && (phase == Phase::Kick || is_pre(phase)) && door_tools_frag_step(g, op) { return; }
    let d = o!(g, op).door;
    let name = crate::orders::door_words(g, d);
    let cx = Cx { d, t: g.time, el: g.time - o!(g, op).t0, lead: o!(g, op).lead, name };
    if o!(g, op).thr.is_some() && o!(g, op).phase != Phase::Thr { door_tools_pie_off(g, op); }
    match o!(g, op).phase {
        Phase::Stack => step_stack(g, op, &cx),
        Phase::Ready => { if !o!(g, op).on_word && o!(g, op).word.is_none() { door_tools_open(g, op); } }
        Phase::Kick => step_kick(g, op, &cx),
        Phase::Toss => step_toss(g, op, &cx),
        Phase::Fuse => step_fuse(g, op, &cx),
        Phase::Wait => step_wait(g, op),
        Phase::Enter => step_enter(g, op, &cx),
        Phase::Place => step_place(g, op, &cx),
        Phase::Standoff => step_standoff(g, op, &cx),
        Phase::Armed => { if door_tools_charge_on(g, d).is_none() { door_tools_charge_gone(g, op, false); } }
        Phase::Slice => step_slice(g, op, &cx),
        Phase::Held => keep_open(g, d),
        Phase::Thr => step_thr(g, op, &cx),
        Phase::Pocket => step_pocket(g, op, &cx),
    }
}

/// the opening they are working stays open
fn keep_open(g: &mut Game, d: DoorId) {
    if crate::doors::door_quiet_t(g, d) > -1.0 && is_open(g, d) { crate::doors::door_set_quiet_t(g, d, -1.0); }
}

/// the men walk to their marks; then the door is opened, or it waits on a word or a signal
fn step_stack(g: &mut Game, op: DoorOpId, cx: &Cx) {
    let (d, t, el, lead) = (cx.d, cx.t, cx.el, cx.lead);
    if o!(g, op).lane.is_some() {
        let (lgd, cover) = { let l = o!(g, op).lane.as_ref().unwrap(); (l.gd, l.cover) };
        let a = o!(g, op).mine.get(lgd).copied();
        let mine = a.is_some() && a == Some(cover) && alive(g, lgd) && o!(g, op).men.contains(&lgd);
        let fop = if mine { crate::roomfight::room_fight_op_of(g, lgd) } else { None };
        let mut hot = false;
        if let Some(f) = fop {
            for (h, mt) in g.tab.roomfight_op[f].meters.iter() {
                if alive(g, h) && t - mt.seen_t < g.k.doormanners.cover_quiet { hot = true; break; }
            }
        }
        let there = mine && { let r = &asg!(g, a.unwrap()); r.arrived || (hot && r.post.is_some() && v3::dist_xz(pos_of(g, lgd), r.post.unwrap()) < 0.5) };
        if mine {
            asg!(g, a.unwrap()).search_t = Some(t);
            let l = o!(g, op).lane.as_mut().unwrap();
            if there && l.at.is_none() { l.at = Some(t); }
        }
        let set = !alive(g, lead) || door_tools_arrived(g, op, lead) || !door_tools_waiting(g, op, el);
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
        if last && !tr(o!(g, op).last_stack) {
            door_tools_last_stack(g, op);
            if o!(g, op).lead != lead { return; }
        }
        if !tr(o!(g, op).last_stack) {
            let men = o!(g, op).men.clone();
            if !men.iter().all(|&m| door_tools_arrived(g, op, m)) && (door_tools_waiting(g, op, el) || (o!(g, op).gate.is_some() && door_tools_busy(g, op, el))) { return; }
        }
        let lp = pos_of(g, lead);
        let mark = o!(g, op).post.get(lead).map_or(lp, |p| p.pos);
        if v3::dist_xz(lp, mark) > 1.5 {
            say(g, lead, "can't get on that door.");
            door_tools_end(g, op, "never made the stack");
            return;
        }
        if o!(g, op).lane.is_some() { door_tools_lane_in(g, op); return; }
        o!(g, op).ready_t = t;
        door_tools_beat(g, op, "stacked");
        let men = o!(g, op).men.clone();
        if !(tr(o!(g, op).big) && door_tools_big_opens(g, d, &men)) && door_tools_halt_at(g, op) { return; }
    }
    if o!(g, op).on_word || o!(g, op).word.is_some() {
        o!(g, op).phase = Phase::Ready;
        o!(g, op).hold = 1.0;
        o!(g, op).t0 = t;
        door_tools_beat(g, op, "ready");
        if o!(g, op).on_word {
            say(g, lead, "set.");
            let men = o!(g, op).men.clone();
            for gd in men {
                if let Some(a) = o!(g, op).mine.get(gd).copied() { asg!(g, a).there = format!("ready on your word · {}", cx.name); }
            }
        }
        return;
    }
    if let Some(gt) = o!(g, op).gate {
        let gops = g.tab.doortools_door_gate[gt].ops.clone();
        if g.tab.doortools_door_gate[gt].open_t.is_nan() {
            if !g.tab.doortools_door_gate[gt].go { return; }
            for &x in &gops {
                if in_ops(g, x) && (o!(g, x).ready_t.is_nan() || door_tools_frag_wait(g, x)) { return; }
            }
            g.tab.doortools_door_gate[gt].open_t = t;
            for &x in &gops { if in_ops(g, x) { door_tools_frag_on(g, x); } }
        }
        let gops = g.tab.doortools_door_gate[gt].ops.clone();
        let beside: Vec<DoorOpId> = gops.iter().copied().filter(|&x| x != op && in_ops(g, x)).collect();
        if !door_tools_will_kick(g, op) && door_tools_boot_beside(g, op) {
            let side = o!(g, op).side;
            if !g.door_tools.gate_asked.has(op) && !is_open(g, d) && !door_tools_locked_out(g, d, side) {
                g.door_tools.gate_asked.add(op);
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
        if tr(k.pick) { door_tools_pick_step(g, op); return; }
        if k.gun.is_some() { door_tools_gun_step(g, op); return; }
    }
    let k = match k0 { Some(k) => k, None => return };
    if tr(k.hand) { door_tools_hand_step(g, op, el); return; }
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
            say(g, lead, "can't get on the door.");
            door_tools_end(g, op, "never made the kick spot");
            return;
        }
        if o!(g, op).word.is_some() && kind != DoorToolKind::BangClear { o!(g, op).hold = 2.0; return; }
        if el < g.k.doortools.kick_cap {
            if let Some(gt) = o!(g, op).gate {
                let gops = g.tab.doortools_door_gate[gt].ops.clone();
                if gops.iter().any(|&x| x != op && in_ops(g, x) && o!(g, x).phase == Phase::Kick && o!(g, x).kick.map_or(false, |q| tr(q.gate) && !tr(q.on_it))) { return; }
            }
        }
        crate::character::anim_kick_door(g, c, None);
        o!(g, op).kick.as_mut().unwrap().t = t;
        return;
    }
    if !(crate::character::anim_kick_impact(g, c) || crate::character::anim_kick_time(g, c) >= crate::character::KICK_IMPACT || !crate::character::anim_kicking(g, c)) { return; }
    let (cp, cid) = (g.ch[c].pos, g.ch[c].id);
    let gave = crate::doors::doors_kick_in(g, d, cp, cid, None);
    g.door_tools.stats.kicks += 1.0;
    door_tools_beat(g, op, "kick");
    door_tools_back_to_jamb(g, op, lead);
    if !gave && !is_open(g, d) {
        o!(g, op).kick = None;
        door_tools_beat(g, op, "the frame held");
        let side = o!(g, op).side;
        if !door_tools_frame_held(g, d, side) || !door_tools_frame_answer(g, op) { door_tools_end(g, op, "the door held the boot"); }
        return;
    }
    door_tools_opened(g, op);
}

/// the can (or the frag): asked again as it leaves the hand, then thrown
fn step_toss(g: &mut Game, op: DoorOpId, cx: &Cx) {
    let (d, t, lead) = (cx.d, cx.t, cx.lead);
    let side = o!(g, op).side;
    let kicked = o!(g, op).kick.map_or(false, |k| !k.t.is_nan());
    let (leaf_a, leaf_t) = (o!(g, op).leaf_a, o!(g, op).leaf_t);
    let ang = crate::doors::door_angle(g, d);
    o!(g, op).leaf_a = Some(ang);
    o!(g, op).leaf_t = Some(t);
    if t - o!(g, op).open_t < (if kicked { g.k.doortools.toss_after_kick } else { g.k.doortools.toss_after_open }) { return; }
    let frag = tr(o!(g, op).frag);
    if !o!(g, op).bang_held {
        say(g, lead, if frag { "no frag." } else { "no bang." });
        door_tools_end(g, op, "no bang to throw");
        return;
    }
    let dname = g.dr[d].def.name.clone();
    if frag {
        let hostages = g.rescue.hostages.clone().iter().any(|&h| alive(g, h));
        let men = o!(g, op).men.clone();
        let who = if hostages { Some(String::from("hostages")) } else { door_tools_frag_behind(g, d, side, &men) };
        if let Some(who) = who {
            let line = if who == "hostages" { String::from("hostages — no frag. going in.") }
                else if who == "you" { String::from(crate::words::wg(g, &"you're in there — no frag. going in.", &"you are in there — no frag. going in.")) }
                else { format!("{} is in there — no frag. going in.", who) };
            say(g, lead, &line);
            o!(g, op).bang_held = false;
            o!(g, op).frag = Some(false);
            door_tools_refund(g, DoorToolsRefundWhat::Frag);
            door_tools_beat(g, op, "frag held");
            door_tools_note(g, &format!("frag held on {}: {} in its reach — in without it", dname, who));
            door_tools_enter(g, op);
            return;
        }
    } else if door_tools_sam_behind(g, d, side) {
        let line = crate::words::wg(g, &"you're in there — no bang. going in.", &"you are in there — no bang. going in.");
        say(g, lead, line);
        o!(g, op).bang_held = false;
        door_tools_refund(g, DoorToolsRefundWhat::Bang);
        door_tools_beat(g, op, "no bang · you are in there");
        door_tools_note(g, &format!("bang held on {}: you in its reach — in without it", dname));
        if o!(g, op).word.is_some() { o!(g, op).phase = Phase::Wait; o!(g, op).hold = 2.0; return; }
        door_tools_enter(g, op);
        return;
    } else if let Some(mate) = door_tools_mate_behind(g, op) {
        let cs = g.gd[mate].callsign.clone();
        say(g, lead, &format!("{} is in there, so no bang. going in.", cs));
        o!(g, op).bang_held = false;
        door_tools_refund(g, DoorToolsRefundWhat::Bang);
        door_tools_beat(g, op, &format!("no bang · {} is in there", cs));
        door_tools_note(g, &format!("bang held on {}: {} in its reach — in without it", dname, cs));
        if o!(g, op).word.is_some() { o!(g, op).phase = Phase::Wait; o!(g, op).hold = 2.0; return; }
        door_tools_enter(g, op);
        return;
    }
    let first = if kicked { lead } else { let f = door_tools_first_in(g, op, None, None); o!(g, op).first = Some(f); f };
    let fc = g.dr[d].frame_centre;
    let mut near: Vec<(GuardId, f64)> = Vec::new();
    for m_ in o!(g, op).men.clone() {
        if m_ == first || !alive(g, m_) { continue; }
        let p = pos_of(g, m_);
        if off_of(g, d, p, side) > 0.0 && (kicked || v3::dist_xz(p, fc) < g.k.doortools.throw_reach) { near.push((m_, v3::dist_xz(p, fc))); }
    }
    js::sort_by(&mut near, |a, b| a.1 - b.1);
    let th = near.first().map_or(first, |x| x.0);
    let w = g.dr[d].def.width;
    let thc = g.gd[th].ch;
    let thp = g.ch[thc].pos;
    let turn = g.k.doortools.toss_turn;
    if turn > 0.0 && th != lead && !kicked && t - o!(g, op).open_t < g.k.doortools.toss_after_open + turn && m::abs(v3::wrap_angle(g.ch[thc].aim_yaw - v3::yaw_to(thp, fc))) > 1.2 { return; }
    if frag || o!(g, op).thr.is_none() {
        let away = -side;
        let angle = crate::doors::door_angle(g, d);
        let a = angle * away;
        let turning = match leaf_t { Some(lt) if t > lt => (angle - leaf_a.unwrap()) / (t - lt), _ => crate::doors::door_vel(g, d) };
        let swing = turning * away;
        if !(m::abs(angle) >= g.k.doortools.frag_way_open || (a >= g.k.doortools.frag_way_ajar && swing > 0.0)) {
            let (what, tag, refund) = if frag { ("frag", "frag", DoorToolsRefundWhat::Frag) } else { ("bang", "can", DoorToolsRefundWhat::Bang) };
            if o!(g, op).frag_way_t.is_none() {
                o!(g, op).frag_way_t = Some(t);
                door_tools_beat(g, op, &format!("{} waits: the door is in the way ({} rad)", tag, js::to_fixed(angle, 2)));
            }
            if t - o!(g, op).frag_way_t.unwrap() < g.k.doortools.frag_way_wait { return; }
            let line = crate::words::wg(g, &format!("the door's in the way — no {}. going in.", what), &format!("door is in the way — no {}. going in.", what));
            say(g, th, &line);
            o!(g, op).bang_held = false;
            o!(g, op).frag = Some(false);
            door_tools_refund(g, refund);
            door_tools_beat(g, op, &format!("{} held", tag));
            door_tools_note(g, &format!("{} held on {}: the leaf is in its way ({} rad) — in without it", what, dname, js::to_fixed(angle, 2)));
            door_tools_enter(g, op);
            return;
        }
    }
    o!(g, op).bang_held = false;
    let along = along_of(g, d, thp);
    let from = door_pt(g, d, v3::clamp(along, 0.25, w - 0.25), 0.35, side, Some(1.3));
    let into = if !frag && o!(g, op).thr.is_none() { door_tools_bang_spot(g, op, from) } else { None };
    let land = match &into { Some(i) => i.at, None => { let deep = if frag { Some(g.k.frag.order_in) } else { None }; door_tools_land_at(g, op, deep) } };
    let sol = crate::throwables::throwables_solve(g, from, land, Some(9.0));
    let hand: V3 = [thp[0] + (from[0] - thp[0]) * 0.2, 1.3, thp[2] + (from[2] - thp[2]) * 0.2];
    let gap = v3::dist_xz(hand, from);
    let pass = if into.is_some() && gap > g.k.doortools.bang_hand_m && g.k.doortools.bang_pass_v > 0.0 && !crate::scene_collision::solid_blocked(g, hand, from, None) {
        Some(DoorToolsDoneItemPass { to: from, v0: sol.v0, t: m::max(0.15, gap / g.k.doortools.bang_pass_v) })
    } else { None };
    let at0 = if pass.is_some() { hand } else { from };
    let v0: V3 = match &pass {
        Some(p) => [(from[0] - hand[0]) / p.t, (from[1] - hand[1] + 0.5 * crate::throwables::GRAVITY * p.t * p.t) / p.t, (from[2] - hand[2]) / p.t],
        None => sol.v0,
    };
    o!(g, op).aim = into.as_ref().map(|i| DoorToolsDoneItemAim { at: i.at, from });
    o!(g, op).pass = pass;
    let who = g.ch[thc].id;
    let it = if frag {
        let opts = crate::throwables::ThrowablesSpawnOpts { fuse: Some(g.k.frag.order_fuse), who: Some(who), heading: Some(v3::yaw_of(sol.v0)), ..Default::default() };
        crate::throwables::throwables_spawn(g, crate::throwables::ItemKind::Frag, from, sol.v0, Some(opts))
    } else {
        let opts = crate::throwables::ThrowablesSpawnOpts { fuse: Some(99.0), who: Some(who), heading: Some(v3::yaw_of(v0)), ..Default::default() };
        crate::throwables::throwables_spawn(g, crate::throwables::ItemKind::Flash, at0, v0, Some(opts))
    };
    g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::Can);
    crate::character::anim_toss_item(g, thc, Some(crate::character::TOSS_QUICK));
    say(g, th, if frag { "frag out!" } else { "bang out!" });
    if frag { crate::game::game_msg(g, "frag out — stay off the door"); }
    if let Some(i) = &into {
        let inside = -off_of(g, d, i.at, side);
        let al = along_of(g, d, i.at) - w / 2.0;
        let tail = if pass.is_some() { format!(" · passed {} m to the door", js::to_fixed(gap, 1)) } else { String::new() };
        door_tools_beat(g, op, &format!("can into the room · {} m in, {} along · {} of {} places lit{}", js::to_fixed(inside, 1), js::to_fixed(al, 1), i.n, i.of, tail));
    }
    o!(g, op).can = Some(it);
    o!(g, op).toss_t = t;
    o!(g, op).threw = Some(th);
    o!(g, op).phase = Phase::Fuse;
    let cs = g.gd[th].callsign.clone();
    door_tools_beat(g, op, &format!("toss:{}", cs));
}

/// in the air: it goes off when it should
fn step_fuse(g: &mut Game, op: DoorOpId, cx: &Cx) {
    let (d, t) = (cx.d, cx.t);
    let side = o!(g, op).side;
    let it = o!(g, op).can.unwrap();
    let frag = tr(o!(g, op).frag);
    let ip = crate::throwables::item_pos(g, it);
    if crate::throwables::item_fired(g, it) {
        o!(g, op).bang_t = t;
        o!(g, op).bang_at = Some(v3::copy(ip));
        o!(g, op).phase = Phase::Wait;
        let what = if frag {
            let off = off_of(g, d, ip, side);
            format!("frag · {} {} m", if off < 0.0 { "inside" } else { "OUTSIDE" }, js::to_fixed(m::abs(off), 1))
        } else { String::from("bang (its own fuse)") };
        door_tools_beat(g, op, &what);
        if frag { g.door_tools.stats.frags += 1.0; door_tools_close_up(g, op, None); }
        if o!(g, op).thr.is_some() { door_tools_thr_banged(g, op); }
        return;
    }
    if frag {
        if !g.items.items.contains(&it) { door_tools_end(g, op, "the frag is gone"); }
        return;
    }
    if let Some(p) = o!(g, op).pass {
        if t - o!(g, op).toss_t < p.t && v3::dist_xz(ip, p.to) > 0.12 && crate::throwables::item_bounces(g, it) == 0.0 { return; }
        crate::throwables::item_set_pos(g, it, v3::copy(p.to));
        crate::throwables::item_set_vel(g, it, v3::copy(p.v0));
        crate::throwables::item_set_heading(g, it, v3::yaw_of(p.v0));
        crate::throwables::item_set_bounces(g, it, 0.0);
        crate::throwables::item_set_settled(g, it, false);
        let at = crate::throwables::item_pos(g, it);
        crate::audio_iface::game_audio_like_play(g, "canisterBounce", Some(at), Some(0.5), None);
        o!(g, op).pass = None;
        return;
    }
    if o!(g, op).land_t.is_nan() && (crate::throwables::item_bounces(g, it) > 0.0 || crate::throwables::item_settled(g, it)) { o!(g, op).land_t = t; }
    let land_t = o!(g, op).land_t;
    let aim = o!(g, op).thr.as_ref().and_then(|x| x.aim).or(o!(g, op).aim.map(|x| x.at));
    let hand = o!(g, op).thr.as_ref().and_then(|x| x.from).or(o!(g, op).aim.map(|x| x.from));
    let air = match (aim, hand) {
        (Some(a), Some(h)) => land_t.is_nan() && (v3::dist_xz(ip, a) < 0.1 || v3::dist_xz(ip, h) >= v3::dist_xz(a, h) - 0.05),
        _ => land_t.is_nan() && crate::throwables::item_vel(g, it)[1] < 0.0 && ip[1] <= g.k.doortools.air_h && off_of(g, d, ip, side) <= -g.k.doortools.air_in,
    };
    if air || (!land_t.is_nan() && t - land_t >= g.k.doortools.fuse_after_land) || t - o!(g, op).toss_t >= g.k.doortools.toss_cap { door_tools_bang(g, op, it); }
}

/// behind a bang or a charge: nobody crosses until the blind burst has run dry
fn step_wait(g: &mut Game, op: DoorOpId) {
    if !door_tools_burst_dry(g, op) { return; }
    if let Some(gt) = o!(g, op).gate {
        for x in g.tab.doortools_door_gate[gt].ops.clone() {
            if x != op && in_ops(g, x) && o!(g, x).kind == DoorToolKind::BangClear && !tr(o!(g, x).frag)
                && (o!(g, x).phase == Phase::Fuse || (o!(g, x).phase == Phase::Wait && !door_tools_burst_dry(g, x))) { return; }
        }
    }
    if o!(g, op).word.is_some() { o!(g, op).hold = 2.0; return; }
    door_tools_enter(g, op);
}

/// the room's id and name, of the mission's own rooms, at a point (`roomOf` in the entry's end)
fn room_of(g: &mut Game, p: V3) -> Option<(String, String)> {
    if !crate::roomclear::room_clear_scenario_active(g) { return None; }
    let at = crate::roomclear::room_clear_scenario_room_at(g, p);
    g.room_clear.rooms.iter().find(|r| Some(&r.def.id) == at.as_ref()).map(|r| (r.def.id.clone(), r.def.name.clone()))
}

/// through the frame: corners, sectors, the floor looked over; then the order is called
fn step_enter(g: &mut Game, op: DoorOpId, cx: &Cx) {
    let (d, t, el, lead) = (cx.d, cx.t, cx.el, cx.lead);
    let side = o!(g, op).side;
    door_tools_threat_deal(g, op);
    let mut i = o!(g, op).queue.len();
    while i > 0 {
        i -= 1;
        let q = o!(g, op).queue[i];
        if crate::rearwatch::rear_holds(g, op, q.gd) { continue; }
        let mq = crate::doorpath::DoorPathMayGoQ { gd: q.gd, pod: crate::doorpath::DoorPathMayGoQPod { pos: q.pod.pos } };
        if !crate::doorpath::door_path_may_go(g, op, mq, i as f64) { continue; }
        o!(g, op).queue.remove(i);
        if o!(g, op).men.contains(&q.gd) {
            o!(g, op).pod.set(q.gd, q.pod);
            // `this.assign(op, q.gd, { …, shy: null, … })`: null over the `shy: op.door` assign would give, so its one line is said here with no shy
            let a = crate::orders::SquadOrdersAssignA {
                order: String::from(o!(g, op).kind.as_str()), door: Some(d), shy: None, post: Some(q.pod.pos), face: Some(q.pod.face), watch: None, thru: Some(d), pace: Some(g.orders.knobs.entry),
                stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim), light: Some(light_of(g)), search: Some(true), going: format!("entering · {}", cx.name), there: String::from("holding the room"), ..Default::default()
            };
            crate::orders::squad_orders_assign(g, q.gd, &a);
            let mine = crate::orders::squad_orders_of(g, q.gd).unwrap();
            o!(g, op).mine.set(q.gd, mine);
            crate::doorpath::door_path_sent(g, op, q.gd, crate::doorpath::DoorPathSentPod { pos: q.pod.pos });
        }
    }
    let sr = door_tools_search_of(g, op, None);
    if let Some(s) = sr { door_tools_look(g, op, s, None, None); }
    let other = g.k.doortools.other_door_m > 0.0 && door_tools_other_doors(g, op, None, None, None);
    let mut swept = true;
    let mut w = 0;
    while w < o!(g, op).men.len() {
        let gd = o!(g, op).men[w];
        w += 1;
        let pod = o!(g, op).pod.get(gd).copied();
        let a = o!(g, op).mine.get(gd).copied();
        let pod = match pod { Some(p) if !o!(g, op).queue.iter().any(|q| q.gd == gd) => p, _ => { swept = false; continue; } };
        let a = a.unwrap();
        let p = pos_of(g, gd);
        asg!(g, a).search_t = Some(t);
        let dealt = o!(g, op).threat.as_ref().map_or(false, |x| x.deal.as_ref().map_or(false, |m_| m_.len() > 0));
        if dealt && door_tools_threat_step(g, op, gd, a, pod) { swept = false; continue; }
        if door_tools_corner_step(g, op, gd, a, pod) { swept = false; continue; }
        if !door_tools_on_point(g, gd, a, pod) { swept = false; continue; }
        let t1 = match o!(g, op).on_pod.get(gd).copied() { Some(x) => x, None => { o!(g, op).on_pod.set(gd, t); t } };
        let k = m::floor((t - t1) / g.k.doortools.sweep_secs);
        if other && door_tools_on_other(g, op, gd) { continue; }
        if k < 2.0 && sr.is_none() {
            swept = false;
            let yaw = pod.face + (if k <= 0.0 { -g.k.doortools.sweep_rad } else { g.k.doortools.sweep_rad });
            asg!(g, a).watch = Some([p[0] + m::sin(yaw) * 4.0, 1.2, p[2] + m::cos(yaw) * 4.0]);
            asg!(g, a).face = Some(yaw);
            continue;
        }
        if let Some(s) = sr { if door_tools_scan(g, op, s, gd, a) { swept = false; continue; } }
        door_tools_on_sector(g, gd, a, pod);
    }
    let handover = { let r = &o!(g, op);
        match &r.thr { Some(x) => !x.handed && r.queue.is_empty() && x.jamb.iter().all(|&m_| !r.men.contains(&m_) || r.on_pod.has(m_)), None => false } };
    if handover { thr!(g, op).handed = true; door_tools_beat(g, op, "handover"); }
    let flowing = match sr { Some(s) => door_tools_flows(g, op, s), None => false };
    if let (Some(s), true) = (sr, flowing) {
        if sr!(g, s).fight_at != Some(t) {
            let mut any = false;
            for x in g.door_tools.ops.clone() {
                if o!(g, x).search.flatten() == Some(s) && o!(g, x).crossed.len() > 0 && o!(g, x).men.iter().any(|&m_| g.gd[m_].script.as_ref().map_or(false, |c| c.over.is_some())) { any = true; break; }
            }
            let r = &mut sr!(g, s);
            if any { r.fight_t = Some(r.fight_t.unwrap_or(0.0) + v3::clamp(t - r.fight_at.unwrap_or(t), 0.0, 0.05)); }
            r.fight_at = Some(t);
        }
    }
    let (search_cap, in_cap, flow_secs, flow_fight) = { let k = &g.k.doortools; (k.search_cap, k.in_cap, k.flow_secs, k.flow_fight) };
    let cap = match sr {
        Some(s) => {
            let r = &sr!(g, s);
            search_cap + (if flowing { (r.flows.unwrap_or(0.0) + r.closes.unwrap_or(0.0)) * flow_secs + m::min(r.fight_t.unwrap_or(0.0), flow_fight) } else { 0.0 })
        }
        None => in_cap,
    };
    if let (Some(s), true) = (sr, flowing) {
        if el >= search_cap && el < cap && !tr(sr!(g, s).late) { sr!(g, s).late = Some(true); g.door_tools.stats.flow_late += 1.0; }
    }
    if !o!(g, op).queue.is_empty() || (!swept && el < cap) { return; }
    if let Some(s) = sr { if swept && el < cap && door_tools_move_on(g, op, s) { return; } }
    if let Some(s) = sr {
        let w = { let r = &sr!(g, s); format!("searched · {} of {} cells unseen · {} moved on", r.left, r.pts.len(), r.moves) };
        door_tools_beat(g, op, &w);
    }
    let deal: Vec<GuardId> = o!(g, op).threat.as_ref().and_then(|x| x.deal.as_ref()).map_or(Vec::new(), |m_| m_.keys());
    for m_ in deal {
        let r = o!(g, op).threat.as_mut().unwrap().deal.as_mut().unwrap().get_mut(m_).unwrap();
        if r.done_t.is_nan() {
            r.done_t = t;
            let faced = tr(r.faced);
            if let Some(a) = o!(g, op).mine.get(m_).copied() { if faced { asg!(g, a).strafe = Some(false); } }
        }
    }
    for gd in o!(g, op).men.clone() {
        let (pod, a) = (o!(g, op).pod.get(gd).copied(), o!(g, op).mine.get(gd).copied());
        if let (Some(pod), Some(a)) = (pod, a) {
            if !o!(g, op).queue.iter().any(|q| q.gd == gd) {
                let face = door_tools_on_sector(g, gd, a, pod);
                g.door_tools.held.set(gd, DoorToolsHeldValue { a, face, d: 0.0, hit_t: f64::NAN, arr_t: f64::NAN, out: None, door: None });
            }
        }
    }
    if g.k.doortools.other_door_m > 0.0 {
        door_tools_other_doors(g, op, Some(true), Some(false), None);
        g.door_tools.watches.push(op);
    }
    let ins: Vec<GuardId> = o!(g, op).men.iter().copied().filter(|&m_| alive(g, m_) && g.door_tools.held.has(m_)).collect();
    if ins.len() >= 3 {
        let width = g.dr[d].def.width;
        let at = door_pt(g, d, width / 2.0, -0.2, side, Some(1.2));
        let mut order = ins.clone();
        { let crossed = &o!(g, op).crossed;
          js::sort_by(&mut order, |a, b| crossed.get(*b).copied().unwrap_or(f64::NEG_INFINITY) - crossed.get(*a).copied().unwrap_or(f64::NEG_INFINITY)); }
        for m_ in order {
            let (ra, rdoor) = { let r = g.door_tools.held.get(m_).unwrap(); (r.a, r.door) };
            let p = pos_of(g, m_);
            if crate::scene_collision::segment_blocked(g, [p[0], 1.2, p[2]], at, None, None, None) || (g.k.doortools.other_door_m > 0.0 && rdoor.is_some()) { continue; }
            let yaw = v3::yaw_to(p, at);
            let far = v3::dist_xz(p, at) + g.k.doortools.last_out_m;
            asg!(g, ra).watch = Some([p[0] + m::sin(yaw) * far, 1.2, p[2] + m::cos(yaw) * far]);
            asg!(g, ra).face = Some(yaw);
            { let r = g.door_tools.held.get_mut(m_).unwrap(); r.face = yaw; r.out = Some(true); }
            let w = format!("door · {}", g.gd[m_].callsign);
            door_tools_beat(g, op, &w);
            break;
        }
    }
    let pod0 = if crate::orders::squad_orders_sams(g) { o!(g, op).pod.values().next().copied() } else { None };
    let lp = pos_of(g, lead);
    let room = match room_of(g, lp) { Some(r) => Some(r), None => match pod0 { Some(p) => room_of(g, p.pos), None => None } };
    let dead_cells = g.k.doortools.dead_cells;
    let left = match sr { Some(s) if sr!(g, s).left >= dead_cells => sr!(g, s).left, _ => 0.0 };
    if let Some((id, rname)) = room {
        let mut ins: Vec<GuardId> = Vec::new();
        for x in g.guards.clone() {
            if alive(g, x) && g.gd[x].team == crate::teams::Team::Hostile && !crate::runner::runner_nobody(g, x) {
                let xp = pos_of(g, x);
                if crate::roomclear::room_clear_scenario_room_at(g, xp).as_ref() == Some(&id) { ins.push(x); }
            }
        }
        let seen = ins.iter().any(|&x| crate::autopilot::in_sight(g, x));
        if let Some(call) = crate::squadnet::room_call(g, &rname, left, !ins.is_empty(), seen) { say(g, lead, &call); }
    } else if let Some(p) = pod0 {
        if let Some(r) = crate::roomfight::room_fight_room_at(g, p.pos) {
            let flood = crate::squadnet::InFloodR {
                cells: g.tab.roomfight_room[r].cells.clone(),
                entries: g.tab.roomfight_room[r].entries.iter().map(|e| crate::squadnet::InFloodREntriesItem { door: e.door }).collect(),
            };
            let mut ins: Vec<GuardId> = Vec::new();
            for x in g.guards.clone() {
                if alive(g, x) && g.gd[x].team == crate::teams::Team::Hostile && !crate::runner::runner_nobody(g, x) {
                    let xp = pos_of(g, x);
                    if crate::squadnet::in_flood(g, &flood, xp) { ins.push(x); }
                }
            }
            let open = open_floor(g, &OpenFloorRoom { cells: g.tab.roomfight_room[r].cells.clone() });
            let seen = ins.iter().any(|&x| crate::autopilot::in_sight(g, x));
            if let Some(call) = crate::squadnet::plain_call(g, open, left, !ins.is_empty(), seen) { say(g, lead, &call); }
        }
    }
    if o!(g, op).thr.is_some() {
        if g.k.doortools.other_door_m > 0.0 { door_tools_other_doors(g, op, Some(true), Some(true), Some(true)); }
        door_tools_hold_doors(g, op);
    }
    door_tools_beat(g, op, "in");
    g.door_tools.stats.entries += 1.0;
    door_tools_end(g, op, "in");
}

/// the charge: he is at the lock for setSecs, then it is on the door and he walks off to the stand-off
fn step_place(g: &mut Game, op: DoorOpId, cx: &Cx) {
    let (d, t, el, lead) = (cx.d, cx.t, cx.el, cx.lead);
    let side = o!(g, op).side;
    let fc = g.dr[d].frame_centre;
    let a = o!(g, op).mine.get(lead).copied().unwrap();
    let lp = pos_of(g, lead);
    let at = asg!(g, a).arrived && v3::dist_xz(lp, fc) < 1.6;
    if g.k.doortools.charge_answer {
        if door_tools_known_open(g, d) || (is_open(g, d) && (at || !o!(g, op).set_t.is_nan())) {
            say(g, lead, "door's open.");
            door_tools_short(g, op, "the door opened");
            return;
        }
    } else if is_open(g, d) {
        say(g, lead, "door's open.");
        let p = o!(g, op).post.get(lead).cloned();
        if let Some(p) = p {
            if crate::orders::squad_orders_big(g) { let aa = standing_off(g, d, &p, &cx.name); door_tools_assign(g, op, lead, &aa); }
        }
        door_tools_stay_hands(g, op);
        door_tools_end(g, op, "the door opened");
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
            say(g, lead, "can't get on that door.");
            door_tools_short(g, op, "never made the door");
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
    if !door_tools_take(g, DoorToolsTakeWhat::Charge) {
        say(g, lead, "no charges left.");
        door_tools_short(g, op, "no charges");
        return;
    }
    let by = g.gd[lead].callsign.clone();
    let c = g.tab.doortools_set_charge.push(SetCharge { door: d, side, t, by });
    g.door_tools.set.push(c);
    let dp = crate::doors::door_pos(g, d);
    crate::audio_iface::game_audio_like_play(g, "lockPick", Some(dp), Some(0.6), None);
    say(g, lead, "charge set.");
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
    if door_tools_charge_on(g, d).is_none() { door_tools_charge_gone(g, op, true); return; }
    let fc = g.dr[d].frame_centre;
    let men = o!(g, op).men.clone();
    let near: Vec<GuardId> = men.iter().copied().filter(|&m_| v3::dist_xz(pos_of(g, m_), fc) < g.k.doortools.backblast_m + 0.05).collect();
    let all = men.iter().all(|&m_| door_tools_arrived(g, op, m_));
    let wait = if !all { door_tools_waiting(g, op, el) } else {
        !near.is_empty() && el < g.k.doortools.stack_cap && (el < g.k.doortools.still_secs || near.iter().any(|&m_| {
            let a = o!(g, op).mine.get(m_).copied().unwrap();
            g.time - asg!(g, a).moved_t < g.k.doortools.still_secs
        }))
    };
    if wait { return; }
    say(g, lead, "stood off. your call.");
    let key = g.k.doortools.blow_key.replacen("Key", "", 1);
    crate::game::game_msg(g, &format!("charge set on the {}. press {} to blow it", cx.name, key));
    o!(g, op).phase = Phase::Armed;
    o!(g, op).t0 = t;
    door_tools_beat(g, op, "armed");
    if o!(g, op).word.is_some() { o!(g, op).hold = 1.0; }
}

/// the angle worked station by station from outside, then held
fn step_slice(g: &mut Game, op: DoorOpId, cx: &Cx) {
    let (d, t, el, lead) = (cx.d, cx.t, cx.el, cx.lead);
    keep_open(g, d);
    let mut all = true;
    let mut w = 0;
    while w < o!(g, op).men.len() {
        let gd = o!(g, op).men[w];
        w += 1;
        if o!(g, op).slice.get(gd).is_none() { continue; }
        let a = o!(g, op).mine.get(gd).copied().unwrap();
        let p = pos_of(g, gd);
        let (si, sj) = { let s = sl!(g, op, gd); (s.i as usize, s.j as usize) };
        let spt = sl!(g, op, gd).pts[si];
        if v3::dist_xz(p, spt) > 0.5 && !asg!(g, a).arrived { sl!(g, op, gd).t = f64::NAN; all = false; continue; }
        if sl!(g, op, gd).t.is_nan() || crate::roomfight::room_fight_contact(g, gd) || crate::roomfight::room_fight_engaged(g, gd) { sl!(g, op, gd).t = t; }
        if t - sl!(g, op, gd).t < g.k.doortools.slice_look { all = false; continue; }
        let looks = sl!(g, op, gd).looks[si].clone();
        if sj + 1 < looks.len() {
            let s = sl!(g, op, gd);
            s.j += 1.0; s.t = t;
            all = false;
            let at = looks[sj + 1];
            asg!(g, a).watch = Some(at);
            asg!(g, a).face = Some(v3::yaw_to(p, at));
            continue;
        }
        let npts = sl!(g, op, gd).pts.len();
        if si + 1 >= npts { continue; }
        let ahead = sl!(g, op, gd).ahead;
        if let Some(h) = ahead {
            if o!(g, op).men.contains(&h) {
                if let Some(up) = o!(g, op).slice.get(h) {
                    if up.i < m::min(up.pts.len() as f64 - 1.0, si as f64 + 3.0) { all = false; continue; }
                }
            }
        }
        { let s = sl!(g, op, gd); s.i += 1.0; s.j = 0.0; s.t = f64::NAN; }
        all = false;
        let i2 = si + 1;
        let (post, look0) = { let s = sl!(g, op, gd); (s.pts[i2], s.looks[i2][0]) };
        let there = if i2 + 1 >= npts { format!("holding outside · {}", cx.name) } else { format!("slicing · {}", cx.name) };
        let aa = cut_a(g, post, v3::yaw_to(post, look0), look0, Some(true), None, None, format!("slicing · {}", cx.name), there);
        door_tools_assign(g, op, gd, &aa);
    }
    if all || el > 40.0 {
        say(g, lead, "holding on the door.");
        o!(g, op).phase = Phase::Held;
        o!(g, op).t0 = t;
        door_tools_beat(g, op, "held outside");
        for gd in o!(g, op).men.clone() {
            let (has, a) = (o!(g, op).slice.get(gd).is_some(), o!(g, op).mine.get(gd).copied());
            if let (true, Some(a)) = (has, a) {
                let at = { let s = sl!(g, op, gd); let l = &s.looks[s.i as usize]; l[l.len() >> 1] };
                let p = pos_of(g, gd);
                asg!(g, a).watch = Some(at);
                asg!(g, a).face = Some(v3::yaw_to(p, at));
            }
        }
    }
}

/// `...(P.lean ? { lean: P.lean, crouch: P.knee[s.i] } : {})`
fn lean_of(g: &mut Game, op: DoorOpId, gd: GuardId) -> (Option<f64>, Option<bool>) {
    let s = sl!(g, op, gd);
    let i = s.i as usize;
    match &s.pie { Some(p) if js::truthy(p.lean) => (Some(p.lean), Some(p.knee[i])), _ => (None, None) }
}

/// the room worked from the threshold: both jambs' diagonals held, the cuts walked, the hold's clock
fn step_thr(g: &mut Game, op: DoorOpId, cx: &Cx) {
    let (d, t, el) = (cx.d, cx.t, cx.el);
    keep_open(g, d);
    door_tools_thr_men(g, op);
    door_tools_thr_watch(g, op);
    if tr(thr!(g, op).step_owed) && !door_tools_word_owed(g, op) { return; }
    let jamb = thr!(g, op).jamb.clone();
    if g.k.doortools.thr_light == 1.0 && !jamb.is_empty() && !jamb.iter().any(|&m_| o!(g, op).mine.get(m_).map_or(false, |&a| tr(asg!(g, a).search))) {
        let m_ = jamb[0];
        if let Some(a) = o!(g, op).mine.get(m_).copied() {
            if !crate::nvg::nvg_worn(g, m_) { asg!(g, a).search = Some(true); }
        }
    }
    for &m_ in &jamb {
        let a = match o!(g, op).mine.get(m_).copied() { Some(a) => a, None => continue };
        if g.k.doortools.thr_light > 0.0 && !tr(asg!(g, a).search) {
            let mate = { let r = &o!(g, op);
                let second = jamb.get(1).copied();
                match second {
                    Some(b) if !(r.slice.get(jamb[0]).map_or(false, |s| s.pie.is_some()) && r.slice.get(b).is_none()) => b,
                    _ => jamb[0],
                } };
            if m_ == mate && crate::nvg::nvg_black(g, m_) { asg!(g, a).search = Some(true); }
        }
        if tr(asg!(g, a).search) { asg!(g, a).search_t = Some(t); }
    }
    let sr = o!(g, op).search.flatten();
    if let Some(s) = sr {
        if t >= thr!(g, op).los_t && !thr!(g, op).told {
            thr!(g, op).los_t = t + 0.5;
            for &m_ in &jamb { sr!(g, s).los.delete(m_); }
        }
        let mut ok = |g: &mut Game, m_: GuardId, i: f64| -> bool {
            g.tab.light[g.gd[m_].flashlight].enabled || door_tools_thr_lit_cell(g, op, s, i)
                || (crate::nvg::nvg_down(g, m_) && (door_tools_thr_tube_cell(g, op, s, i) || { let q = sr!(g, s).pts[i as usize]; crate::irlight::ir_shows_at(g, m_, q) }))
        };
        door_tools_look(g, op, s, Some(&jamb), Some(&mut ok));
    }
    let mut done = true;
    let pie_op = jamb.iter().any(|&m_| o!(g, op).slice.get(m_).map_or(false, |s| s.pie.is_some()));
    let walkers: Vec<GuardId> = if pie_op { jamb.clone() } else { vec![o!(g, op).lead] };
    let going = format!("slicing · {}", cx.name);
    let holding = format!("holding the threshold · {}", cx.name);
    for &gd in &walkers {
        let a = match (o!(g, op).slice.get(gd).is_some(), o!(g, op).mine.get(gd).copied()) { (true, Some(a)) => a, _ => continue };
        let has_p = sl!(g, op, gd).pie.is_some();
        if has_p && pie!(g, op, gd).fin { continue; }
        let mut fin = false;
        if has_p { pie!(g, op, gd).begun = true; }
        let p = pos_of(g, gd);
        let c = g.gd[gd].ch;
        if has_p && v3::dist_xz(p, { let s = sl!(g, op, gd); s.pts[s.i as usize] }) < 0.12 { pie!(g, op, gd).on = true; }
        let a_search = asg!(g, a).search;
        if has_p && sl!(g, op, gd).i > 0.0 && !pie!(g, op, gd).on && pie!(g, op, gd).to.is_none() && pie!(g, op, gd).gun == 0.0
            && (crate::roomfight::room_fight_contact(g, gd) || crate::roomfight::room_fight_engaged(g, gd)) {
            let here: V3 = [p[0], 0.0, p[2]];
            let i = sl!(g, op, gd).i as usize;
            let at = asg!(g, a).watch.unwrap_or(sl!(g, op, gd).looks[i][0]);
            pie!(g, op, gd).to = Some(sl!(g, op, gd).pts[i]);
            sl!(g, op, gd).pts[i] = here;
            pie!(g, op, gd).on = true;
            let (lean, crouch) = lean_of(g, op, gd);
            let aa = cut_a(g, here, v3::yaw_to(here, at), at, a_search, lean, crouch, going.clone(), going.clone());
            door_tools_assign(g, op, gd, &aa);
        }
        let i = sl!(g, op, gd).i as usize;
        let spt = sl!(g, op, gd).pts[i];
        let a_arrived = asg!(g, a).arrived;
        let unset = if has_p {
            t - pie!(g, op, gd).at <= PIE_WAIT && (!(a_arrived || v3::dist_xz(p, spt) < 0.12) || (pie!(g, op, gd).knee[i] && g.ch[c].anim.crouch < 0.8))
        } else { v3::dist_xz(p, spt) > 0.5 && !a_arrived };
        if unset {
            sl!(g, op, gd).t = f64::NAN;
        } else {
            if sl!(g, op, gd).t.is_nan() || crate::roomfight::room_fight_contact(g, gd) || crate::roomfight::room_fight_engaged(g, gd) { sl!(g, op, gd).t = t; }
            if has_p && crate::roomfight::room_fight_no_shot(g, gd) > pie_gun().after && t - pie!(g, op, gd).gun_at > pie_gun().every {
                let at = spt;
                let inw = pie!(g, op, gd).inw;
                let q: V3 = [at[0] + inw[0] * pie_gun().step, 0.0, at[2] + inw[2] * pie_gun().step];
                let other = jamb.iter().copied().find(|&m_| m_ != gd && alive(g, m_));
                let os_at = other.and_then(|x| o!(g, op).slice.get(x).map(|s| s.pts[s.i as usize]));
                let apart = g.k.doortools.thr_pie_apart;
                let ok = pie!(g, op, gd).gun < pie_gun().most
                    && !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2])
                    && !crate::scene_collision::solid_blocked(g, [at[0], 0.45, at[2]], [q[0], 0.45, q[2]], None)
                    && !(other.is_some() && (v3::dist_xz(pos_of(g, other.unwrap()), q) < apart || os_at.map_or(false, |x| v3::dist_xz(x, q) < apart)));
                pie!(g, op, gd).gun_at = t;
                if ok || sl!(g, op, gd).i > 0.0 {
                    if ok {
                        pie!(g, op, gd).gun += 1.0;
                        sl!(g, op, gd).pts[i] = q;
                    } else {
                        sl!(g, op, gd).i -= 1.0;
                        let r = pie!(g, op, gd);
                        r.gun = 0.0; r.fin = true; r.to = None;
                    }
                    let i2 = sl!(g, op, gd).i as usize;
                    let l = sl!(g, op, gd).looks[i2].clone();
                    let at2 = if ok { asg!(g, a).watch.unwrap_or(l[0]) } else { l[l.len() >> 1] };
                    let post = sl!(g, op, gd).pts[i2];
                    let (lean, crouch) = if ok && i2 > 0 { lean_of(g, op, gd) } else { (None, None) };
                    let word = if ok { going.clone() } else { holding.clone() };
                    let aa = cut_a(g, post, v3::yaw_to(post, at2), at2, a_search, lean, crouch, word.clone(), word);
                    door_tools_assign(g, op, gd, &aa);
                    if !ok {
                        let w = format!("no shot from the cut · {} back", g.gd[gd].callsign);
                        door_tools_beat(g, op, &w);
                        continue;
                    }
                }
            }
            let i = sl!(g, op, gd).i as usize;
            let looks = sl!(g, op, gd).looks[i].clone();
            let slice_look = g.k.doortools.slice_look;
            if has_p && pie!(g, op, gd).to.is_some() && t - sl!(g, op, gd).t >= slice_look {
                let to = pie!(g, op, gd).to.unwrap();
                sl!(g, op, gd).pts[i] = to;
                { let r = pie!(g, op, gd); r.to = None; r.on = false; r.gun = 0.0; r.at = t; }
                { let s = sl!(g, op, gd); s.j = 0.0; s.t = f64::NAN; }
                let (lean, crouch) = lean_of(g, op, gd);
                let aa = cut_a(g, to, v3::yaw_to(to, looks[0]), looks[0], a_search, lean, crouch, going.clone(), going.clone());
                door_tools_assign(g, op, gd, &aa);
                done = false;
                continue;
            }
            let end_on = has_p && pie!(g, op, gd).end_on;
            let sj = sl!(g, op, gd).j as usize;
            if t - sl!(g, op, gd).t < (if end_on { g.k.doortools.thr_lean_look } else { slice_look }) {
                // resting on this angle
            } else if sj + 1 < looks.len() {
                { let s = sl!(g, op, gd); s.j += 1.0; s.t = t; }
                let at = looks[sj + 1];
                asg!(g, a).watch = Some(at);
                asg!(g, a).face = Some(v3::yaw_to(p, at));
            } else {
                let npts = sl!(g, op, gd).pts.len();
                let next = if !has_p { if i + 1 < npts { i as f64 + 1.0 } else { -1.0 } }
                    else if end_on { -1.0 }
                    else {
                        let arg = { let s = sl!(g, op, gd); let r = s.pie.as_ref().unwrap();
                            DoorToolsPieNextS { pts: s.pts.clone(), i: s.i, pie: Some(DoorToolsPieNextSPie { eyes: r.eyes.clone(), knee: r.knee.clone() }) } };
                        door_tools_pie_next(g, op, gd, &arg)
                    };
                if next >= 0.0 {
                    { let s = sl!(g, op, gd); s.i = next; s.j = 0.0; s.t = f64::NAN; }
                    if has_p { let r = pie!(g, op, gd); r.at = t; r.gun = 0.0; r.on = false; r.to = None; }
                    let i2 = next as usize;
                    let last = i2 + 1 >= npts;
                    let (post, look0) = { let s = sl!(g, op, gd); (s.pts[i2], s.looks[i2][0]) };
                    let thr_light = g.k.doortools.thr_light;
                    let search = if has_p { a_search } else { Some(thr_light >= 2.0 || (thr_light == 1.0 && jamb.len() < 2)) };
                    let (lean, crouch) = lean_of(g, op, gd);
                    let aa = cut_a(g, post, v3::yaw_to(post, look0), look0, search, lean, crouch, going.clone(), if last { holding.clone() } else { going.clone() });
                    door_tools_assign(g, op, gd, &aa);
                } else if has_p && js::truthy(pie!(g, op, gd).end) && !end_on && i > 0 {
                    pie!(g, op, gd).end_on = true;
                    sl!(g, op, gd).t = t;
                    let lt = m::min(thr!(g, op).los_t, t + 0.3);
                    thr!(g, op).los_t = lt;
                    let post = sl!(g, op, gd).pts[i];
                    let end = pie!(g, op, gd).end;
                    let aa = cut_a(g, post, v3::yaw_to(post, looks[0]), looks[0], a_search, Some(end), None, going.clone(), going.clone());
                    door_tools_assign(g, op, gd, &aa);
                } else {
                    fin = true;
                    let mid = looks[looks.len() >> 1];
                    if has_p {
                        pie!(g, op, gd).fin = true;
                        let post = sl!(g, op, gd).pts[i];
                        let aa = cut_a(g, post, v3::yaw_to(post, mid), mid, a_search, None, None, holding.clone(), holding.clone());
                        door_tools_assign(g, op, gd, &aa);
                    } else if !thr!(g, op).sliced {
                        thr!(g, op).sliced = true;
                        asg!(g, a).watch = Some(mid);
                        asg!(g, a).face = Some(v3::yaw_to(p, mid));
                        let w = format!("set{}", unseen_of(g, sr));
                        door_tools_beat(g, op, &w);
                    }
                }
            }
        }
        done = done && fin;
    }
    if done && pie_op && !thr!(g, op).sliced {
        thr!(g, op).sliced = true;
        let w = format!("sliced{}", unseen_of(g, sr));
        door_tools_beat(g, op, &w);
    }
    if !thr!(g, op).hold && door_tools_last_call(g, op) {
        if !tr(o!(g, op).last_hold) {
            o!(g, op).last_hold = Some(true);
            o!(g, op).last_tossed = Some(thr!(g, op).tossed);
            door_tools_beat(g, op, "last warning · no hold");
        }
        thr!(g, op).force = true;
        door_tools_to_pockets(g, op);
        return;
    }
    let min = if thr!(g, op).first == Some(ThresholdFirst::Done) { g.k.doortools.thr_first_min } else { g.k.doortools.thr_min };
    let quiet = done && el >= min && t - thr!(g, op).quiet_t >= g.k.doortools.thr_quiet;
    let cap = g.k.doortools.thr_cap;
    if thr!(g, op).hold {
        if (quiet || el >= cap) && !thr!(g, op).told && !tr(thr!(g, op).cut_owed) {
            thr!(g, op).told = true;
            door_tools_beat(g, op, "holding · on your word");
            let lead = cx.lead;
            say(g, lead, "holding on the door.");
            for m_ in thr!(g, op).jamb.clone() {
                if let Some(j) = o!(g, op).mine.get(m_).copied() { asg!(g, j).there = format!("holding the threshold on your word · {}", cx.name); }
            }
        }
        return;
    }
    if !(quiet || el >= cap) || tr(thr!(g, op).step_owed) { return; }
    door_tools_to_pockets(g, op);
}

/// the callsigns of the men who ran from a grenade, for a beat
fn fled_names(g: &mut Game, op: DoorOpId) -> String {
    let fled = thr!(g, op).fled.as_ref().map_or(Vec::new(), |f| f.keys());
    let names: Vec<String> = fled.iter().map(|&m_| g.gd[m_].callsign.clone()).collect();
    names.join(", ")
}

/// the blind pockets: a can for the floor nobody has had a muzzle across, then the last burst's own count, and in
fn step_pocket(g: &mut Game, op: DoorOpId, cx: &Cx) {
    let (d, t) = (cx.d, cx.t);
    let side = o!(g, op).side;
    keep_open(g, d);
    door_tools_thr_men(g, op);
    door_tools_thr_watch(g, op);
    let first = thr!(g, op).first;
    if first == Some(ThresholdFirst::Burst) {
        if door_tools_burst_dry(g, op) { door_tools_thr_out(g, op, None); }
        return;
    }
    if t < thr!(g, op).next_t { return; }
    let fled_n = thr!(g, op).fled.as_ref().map_or(0, |f| f.len());
    let tossed_fresh = js::truthy(thr!(g, op).tossed) && t - o!(g, op).bang_t < g.k.doortools.entry_cap;
    if fled_n > 0 && !tossed_fresh {
        if door_tools_thr_frag_wait(g, op) {
            if let Some(ft) = thr!(g, op).frag_t {
                let w = thr!(g, op).hand_w.unwrap_or(0.0) + m::min(t - ft, 0.1);
                thr!(g, op).hand_w = Some(w);
            }
            thr!(g, op).frag_t = Some(t);
            thr!(g, op).hand_t = None;
            if !tr(thr!(g, op).frag_said) {
                thr!(g, op).frag_said = Some(true);
                let names = fled_names(g, op);
                door_tools_beat(g, op, &format!("the door waits: {} not back", names));
                let who = { let r = &o!(g, op); let f = r.thr.as_ref().unwrap().fled.as_ref().unwrap(); r.men.iter().copied().find(|&m_| !f.has(m_)).unwrap_or(r.lead) };
                say(g, who, "hold — back on the door.");
            }
            if door_tools_thr_frag_wait(g, op) { return; }
        }
        thr!(g, op).frag_t = None;
        if tr(thr!(g, op).frag_said) {
            thr!(g, op).frag_said = Some(false);
            let names = fled_names(g, op);
            let why = if tr(thr!(g, op).hand_done) || thr!(g, op).hand_w.unwrap_or(0.0) >= g.k.doortools.thr_hand_wait { "the order's wait is spent" } else { "not waited for" };
            door_tools_beat(g, op, &format!("on without {} · {}", names, why));
        }
    } else if tr(thr!(g, op).frag_said) {
        thr!(g, op).frag_said = Some(false);
        thr!(g, op).frag_t = None;
        if fled_n == 0 {
            let was = thr!(g, op).thrower;
            if let Some(w) = was { if !thr!(g, op).jamb.contains(&w) { o!(g, op).rear.add(w); } }
            let nt = door_tools_thr_thrower(g, op);
            thr!(g, op).thrower = Some(nt);
            o!(g, op).rear.delete(nt);
            thr!(g, op).next_t = t + g.k.doortools.thr_turn;
            return;
        }
    }
    let th = match thr!(g, op).thrower { Some(x) => x, None => { let x = door_tools_thr_thrower(g, op); thr!(g, op).thrower = Some(x); x } };
    let men = o!(g, op).men.clone();
    let who = door_tools_can_behind(g, d, side, &men);
    let mate = match who.as_deref() { Some("you") => String::from("you are in there"), Some(w) => format!("{} is in there", w), None => String::new() };
    let none = g.door_tools.bangs <= 0.0 || who.is_some();
    o!(g, op).rear.delete(th);
    let tossed = thr!(g, op).tossed;
    let force = thr!(g, op).force;
    let most = if tr(o!(g, op).last_hold) {
        if tossed > 0.0 && !tr(thr!(g, op).wide) && t - o!(g, op).bang_t < g.k.doortools.thr_last_fresh { tossed } else { o!(g, op).last_tossed.unwrap_or(0.0) + 1.0 }
    } else {
        let tc = thr!(g, op).clone();
        m::max(door_tools_thr_cans(g, &tc), if force { 1.0 } else { 0.0 })
    };
    let can_first = first == Some(ThresholdFirst::Can);
    let blank_ok = (force || can_first) && !js::truthy(tossed);
    let pick = if tossed >= most || none { None } else {
        match door_tools_pocket_of(g, op, th) { Some(p) => Some(p), None => if blank_ok { Some(DoorToolsPocketOf { at: None, n: 0.0 }) } else { None } }
    };
    let pick = match pick { Some(p) => p, None => {
        if can_first { door_tools_thr_out(g, op, Some(if mate.is_empty() { "none left" } else { &mate })); return; }
        if js::truthy(tossed) { o!(g, op).phase = Phase::Wait; return; }
        let w = if most <= 0.0 { String::from("no bang · none to spend on a threshold") }
            else if !mate.is_empty() { format!("no bang · {}", mate) }
            else if none { String::from("no bang · none left") }
            else { String::from("no bang · no blind pocket") };
        door_tools_beat(g, op, &w);
        door_tools_to_step(g, op);
        return;
    } };
    if !can_first && !crate::doors::door_off(g, d) && (thr!(g, op).shut_t.is_some() || !is_open(g, d)) {
        let away = -side;
        let angle = crate::doors::door_angle(g, d);
        if !(m::abs(angle) >= g.k.doortools.frag_way_open || (angle * away >= g.k.doortools.frag_way_ajar && crate::doors::door_vel(g, d) * away > 0.0)) {
            if thr!(g, op).shut_t.is_none() {
                let locked = door_tools_locked_out(g, d, side);
                thr!(g, op).shut_t = Some(if locked { -1e9 } else { t });
                door_tools_beat(g, op, &format!("can waits: the door has come shut ({} rad)", js::to_fixed(angle, 2)));
                if !locked { door_tools_shove(g, op); }
            }
            if t - thr!(g, op).shut_t.unwrap() < g.k.doortools.frag_way_wait { return; }
            let line = crate::words::wg(g, &"the door's in the way — no bang. going in.", &"door is in the way — no bang. going in.");
            say(g, th, line);
            door_tools_beat(g, op, "can held");
            thr!(g, op).shut_t = None;
            door_tools_to_step(g, op);
            return;
        }
        thr!(g, op).shut_t = None;
    }
    let shared_word = match o!(g, op).word { Some(wd) => g.door_tools.ops.iter().any(|&x| x != op && o!(g, x).word == Some(wd)), None => false };
    if !tr(thr!(g, op).hand_done) && !tr(o!(g, op).last_stack) && !tr(o!(g, op).last_hold) && !crate::rescue::rescue_last_warning(g, d) && !shared_word {
        let lead_now = o!(g, op).lead;
        let wait = g.k.doortools.thr_hand_wait;
        let mut h = if th != lead_now && door_tools_thr_at(g, op, th) { Some(th) } else {
            let mut fit = |g: &mut Game, m_: GuardId| -> bool { m_ == th || blank_ok || door_tools_pocket_of(g, op, m_).is_some() };
            door_tools_thr_pick(g, op, true, Some(&mut fit))
        };
        if h.is_some() && h != Some(th) && thr!(g, op).hand_w.unwrap_or(0.0) >= wait { h = None; }
        if h.is_none() {
            if thr!(g, op).hand_w.unwrap_or(0.0) < wait {
                let every = m::max(0.05, g.k.doortools.thr_hand_every);
                match thr!(g, op).hand_t {
                    None => {
                        let what = if door_tools_thr_at(g, op, th) {
                            let mut off: Vec<String> = Vec::new();
                            for m_ in o!(g, op).men.clone() { if m_ != lead_now && !door_tools_thr_at(g, op, m_) { off.push(g.gd[m_].callsign.clone()); } }
                            let off = off.join(", ");
                            format!("but the lead's own ({} not back)", if off.is_empty() { "nobody" } else { &off })
                        } else {
                            format!("({} {} m off)", g.gd[th].callsign, js::to_fixed(v3::dist_xz(pos_of(g, th), g.dr[d].frame_centre), 1))
                        };
                        door_tools_beat(g, op, &format!("can waits: no hand at the door {}", what));
                        say(g, lead_now, "hold the bang — back on the door.");
                    }
                    Some(ht) => {
                        let w = thr!(g, op).hand_w.unwrap_or(0.0) + m::min(t - ht, 2.0 * every);
                        thr!(g, op).hand_w = Some(w);
                    }
                }
                thr!(g, op).hand_t = Some(t);
                if thr!(g, op).hand_w.unwrap_or(0.0) < wait { thr!(g, op).next_t = t + every; return; }
            }
            thr!(g, op).hand_t = None;
            say(g, lead_now, if can_first { "nobody on the door — no bang." } else { "nobody on the door — no bang. going in." });
            if can_first {
                door_tools_thr_out(g, op, Some("nobody at the door"));
            } else {
                door_tools_beat(g, op, "no bang · nobody at the door");
                if js::truthy(tossed) { o!(g, op).phase = Phase::Wait; } else { door_tools_to_step(g, op); }
            }
            return;
        }
        thr!(g, op).hand_t = None;
        if let Some(hh) = h {
            if hh != th {
                if !thr!(g, op).jamb.contains(&th) { o!(g, op).rear.add(th); }
                thr!(g, op).thrower = Some(hh);
                o!(g, op).rear.delete(hh);
                let turn = g.k.doortools.thr_turn;
                let w = thr!(g, op).hand_w.unwrap_or(0.0) + turn;
                thr!(g, op).hand_w = Some(w);
                thr!(g, op).next_t = t + turn;
                let cs = g.gd[hh].callsign.clone();
                door_tools_beat(g, op, &format!("can to {} · at the door", cs));
                return;
            }
        }
    } else if thr!(g, op).hand_t.is_some() {
        thr!(g, op).hand_t = None;
    }
    if !door_tools_take(g, DoorToolsTakeWhat::Bang) { return; }
    let w = g.dr[d].def.width;
    let from = door_tools_hand_at(g, op, th);
    let land = match pick.at { Some(a) => a, None => door_tools_land_at(g, op, None) };
    let sol = crate::throwables::throwables_solve(g, from, land, Some(9.0));
    let thc = g.gd[th].ch;
    let who_id = g.ch[thc].id;
    let opts = crate::throwables::ThrowablesSpawnOpts { fuse: Some(99.0), who: Some(who_id), heading: Some(v3::yaw_of(sol.v0)), ..Default::default() };
    let it = crate::throwables::throwables_spawn(g, crate::throwables::ItemKind::Flash, from, sol.v0, Some(opts));
    g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::Can);
    crate::character::anim_toss_item(g, thc, Some(crate::character::TOSS_QUICK));
    say(g, th, "bang out!");
    let mut aim = pick.at;
    let mut short = false;
    let first_in = g.k.doortools.thr_first_in;
    if let (true, Some(at), true) = (can_first, pick.at, first_in > 0.0) {
        let o0 = off_of(g, d, from, side);
        let k = (o0 + first_in) / (o0 - off_of(g, d, at, side));
        if k > 0.0 && k < 1.0 {
            aim = Some([from[0] + (at[0] - from[0]) * k, at[1], from[2] + (at[2] - from[2]) * k]);
            short = true;
        }
    }
    o!(g, op).can = Some(it);
    o!(g, op).toss_t = t;
    o!(g, op).land_t = f64::NAN;
    o!(g, op).threw = Some(th);
    { let r = thr!(g, op); r.aim = aim; r.from = Some(from); r.tossed += 1.0; }
    o!(g, op).phase = Phase::Fuse;
    let head = if can_first { "first · " } else { "" };
    let tail = match pick.at {
        Some(at) => {
            let inside = -off_of(g, d, at, side);
            let al = along_of(g, d, at) - w / 2.0;
            let cooked = if short { format!(" · cooked short, {} m in", first_in) } else { String::new() };
            format!("pocket · {} cells unseen · {} m in, {} along{}", pick.n, js::to_fixed(inside, 1), js::to_fixed(al, 1), cooked)
        }
        None => String::from(if o!(g, op).search.flatten().is_some() { "no blind pocket: the middle" } else { "no model of the room: the middle" }),
    };
    let cs = g.gd[th].callsign.clone();
    door_tools_beat(g, op, &format!("toss:{} · {}{}", cs, head, tail));
}
