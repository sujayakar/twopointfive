pub use ::game_types::breach::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

type OP = crate::squad::OrderPatch;
type St = crate::character::Stance;
type Up = crate::character::UpperMode;

/// The drill's fields: never a function, so no reference into `g` is ever returned or kept.
macro_rules! dr { ($g:expr) => { $g.night_breach.drill.as_mut().unwrap() } }
/// A line of the plan said by a man (the text is cloned out of the plan before the call).
macro_rules! say { ($g:expr, $gd:expr, $key:expr, $($path:ident).+) => {{ let s = dr!($g).plan.$($path).+.clone(); breach_drill_say($g, $gd, $key, &s, None); }} }

// ---------------------------------------------------------------- small private helpers
fn man(g: &mut Game, i: usize) -> GuardId { if i == 0 { dr!(g).a } else { dr!(g).b } }
/// `H[k]`: undefined past the end.
fn host(g: &mut Game, k: usize) -> Option<GuardId> { dr!(g).hostiles.get(k).copied() }
/// `c.bones.<b> ?? v3.add(c.pos, [0, y, 0])`
fn bone_or(g: &Game, c: CharId, b: crate::character::Bone, y: f64) -> V3 {
    match g.ch[c].bones.get(b) { Some(p) => *p, None => v3::add(g.ch[c].pos, [0.0, y, 0.0]) }
}
/// The live object (`this.live`): its one home is the drill's own field, which NightBreach writes as play goes.
fn live_of(g: &mut Game) -> BreachLive { dr!(g).live }
/// What the pair says in place of the plan's line (WORDS.on and WORDS.guide settled on).
fn live_text(charge: bool, key: &str, text: &str) -> String {
    let k = String::from(key);
    if !charge { if let Some(s) = live_no_charge().get(k.clone()) { return s.clone(); } }
    if let Some(Some(s)) = live_guide().get(k.clone()) { return s.clone(); }
    if let Some(s) = live_now().get(k.clone()) { return s.clone(); }
    if let Some(s) = live_lines().get(k) { return s.clone(); }
    String::from(text)
}
/// route() on a list: new waypoints and the order onto the first (`goal: wps[0] ?? null`).
fn route_on(g: &mut Game, gd: GuardId, wps: &mut Vec<V3>, pts: &[V3], mut patch: OP) {
    wps.clear();
    for p in pts { wps.push(v3::copy(*p)); }
    patch.goal = Some(wps.first().copied());
    crate::squad::order(g, gd, patch);
}
/// route() of man i on his own waypoint list.
fn route_i(g: &mut Game, i: usize, pts: &[V3], patch: OP) {
    let gd = man(g, i);
    let mut w = if i == 0 { core::mem::take(&mut dr!(g).wp_a) } else { core::mem::take(&mut dr!(g).wp_b) };
    route_on(g, gd, &mut w, pts, patch);
    if i == 0 { dr!(g).wp_a = w; } else { dr!(g).wp_b = w; }
}
/// advance() of man i on his own waypoint list.
fn adv(g: &mut Game, i: usize, pass: Option<f64>) -> bool {
    let gd = man(g, i);
    let mut w = if i == 0 { core::mem::take(&mut dr!(g).wp_a) } else { core::mem::take(&mut dr!(g).wp_b) };
    let r = breach_drill_advance(g, gd, &mut w, pass);
    if i == 0 { dr!(g).wp_a = w; } else { dr!(g).wp_b = w; }
    r
}
/// One vector in the TypeScript: aimA is flows[0].aim, and while ordered onto it, the man's script.aimAt. A script now aiming at anything else keeps it.
fn set_aim(g: &mut Game, i: usize, new: V3) {
    let old = dr!(g).flows[i].aim;
    dr!(g).flows[i].aim = new;
    if i == 0 { dr!(g).aim_a = new; } else { dr!(g).aim_b = new; }
    let gd = man(g, i);
    if let Some(s) = g.gd[gd].script.as_mut() { if s.aim_at == Some(old) { s.aim_at = Some(new); } }
}
/// `F.sweep.at(t, F.aim)`
fn sweep_tick(g: &mut Game, i: usize, t: f64) {
    let mut out = dr!(g).flows[i].aim;
    crate::squad::sweep_at(&mut dr!(g).flows[i].sweep, t, &mut out);
    set_aim(g, i, out);
}
fn targets(g: &mut Game, i: usize) -> (Vec<usize>, Vec<usize>) {
    let t = &dr!(g).plan.targets;
    let (centre, corner) = if i == 0 { (&t.0.centre, &t.0.corner) } else { (&t.1.centre, &t.1.corner) };
    (centre.iter().map(|k| *k as usize).collect(), corner.iter().map(|k| *k as usize).collect())
}
fn hostile_dead(g: &mut Game, k: usize) -> bool { match host(g, k) { Some(h) => crate::condition::down(g, g.gd[h].ch), None => false } }
fn mk(aim: V3) -> OpFlow {
    OpFlow { st: FlowSt::Wait, t0: 0.0, sweep: crate::squad::Sweep::blank(), aim, q: Vec::new(), on: -1, since: 0.0, n: 0.0, next: 0.0, in_at: f64::NAN, corner_at: f64::NAN }
}

// ---------------------------------------------------------------- the drill
/// A drill, not yet started.
pub(super) fn breach_drill_new(plan: &crate::scene_level::BreachPlan, a: GuardId, b: GuardId, hostiles: &[GuardId], live: crate::breach::BreachLive) -> crate::breach::BreachDrill {
    let mut d = BreachDrill::blank();
    d.plan = plan.clone();
    d.a = a;
    d.b = b;
    d.hostiles = hostiles.to_vec();
    d.aim_a = v3::copy(plan.stack_aim[0]);
    d.aim_b = v3::copy(plan.stack_aim[1]);
    d.flows = [mk(d.aim_a), mk(d.aim_b)];
    d.down_logged = hostiles.iter().map(|_| false).collect();
    d.live = live;
    d
}

/// The door to breach.
fn breach_drill_door(g: &mut crate::game::Game) -> Option<DoorId> {
    let n = dr!(g).plan.door.clone();
    crate::doors::doors_by_name(g, &n)
}

/// The next door.
fn breach_drill_next_door(g: &mut crate::game::Game) -> Option<DoorId> {
    let n = dr!(g).plan.next.door.clone();
    crate::doors::doors_by_name(g, &n)
}

/// Log a beat.
fn breach_drill_beat(g: &mut crate::game::Game, what: &str) {
    dr!(g).beats.push(BreachDrillBeatsItem { what: String::from(what) });
}

/// True the first time for a key.
fn breach_drill_once(g: &mut crate::game::Game, key: &str) -> bool {
    dr!(g).done.add(String::from(key))
}

/// Is there a stun charge in this entry.
fn breach_drill_charge(g: &mut crate::game::Game) -> bool {
    let b = dr!(g).b;
    live_of(g).stun && !crate::condition::down(g, g.gd[b].ch)
}

/// A line, once, by a living man; live play swaps what the drill cannot know.
fn breach_drill_say(g: &mut crate::game::Game, gd: GuardId, key: &str, text: &str, radio: Option<bool>) {
    let radio = radio.unwrap_or(false);
    let charge = breach_drill_charge(g);
    let text = live_text(charge, key, text);
    if !text.is_empty() && breach_drill_once(g, &alloc::format!("say:{}", key)) && !crate::condition::down(g, g.gd[gd].ch) {
        crate::game::game_say(g, gd, &text, Some(radio), None);
    }
}

/// Past a waypoint, on to the next; true when on the last and arrived.
fn breach_drill_advance(g: &mut crate::game::Game, gd: GuardId, wps: &mut Vec<V3>, pass: Option<f64>) -> bool {
    let pass = pass.unwrap_or(0.7);
    if crate::condition::down(g, g.gd[gd].ch) { return true; }
    let c = g.gd[gd].ch;
    while wps.len() > 1 && v3::dist_xz(g.ch[c].pos, wps[0]) < pass {
        wps.remove(0);
        crate::squad::order(g, gd, OP { goal: Some(Some(wps[0])), ..Default::default() });
    }
    wps.len() <= 1 && g.gd[gd].script.as_ref().map_or(false, |s| s.arrived == Some(true))
}

/// A door pulled to and latched.
fn shut(g: &mut Game, d: DoorId) {
    crate::doors::door_reset(g, d);
    g.dr[d].leaf.angle = 0.0;
    g.dr[d].leaf.vel = 0.0;
    g.dr[d].leaf.latched = true;
    g.dr[d].leaf.closing = false;
}

/// Everybody on his mark, the leaves shut.
pub(super) fn breach_drill_start(g: &mut crate::game::Game) {
    for i in 0..2 {
        let gd = man(g, i);
        let c = g.gd[gd].ch;
        let stack = dr!(g).plan.stack[i];
        let from = match dr!(g).plan.from { Some(f) => f[i], None => stack };
        { let to = v3::copy(from); crate::character::pos_replace(g, c, to); }
        g.ch[c].vel = [0.0, 0.0, 0.0];
        let yaw = v3::yaw_to(from, stack);
        g.ch[c].aim_yaw = yaw;
        g.ch[c].body_yaw = yaw;
        g.ch[c].aim_pitch = 0.0;
        g.gd[gd].speed = 0.0;
        crate::mind::set_state(g, gd, crate::game::GuardState::Patrol, "breach");
        crate::mind::set(g, gd, 0.0, "breach");
        if g.gd[gd].last_known.is_some() { crate::mind::fix(g, gd, None, "breach"); }
        crate::control::set_hold(g, gd, false, "breach");
        crate::control::set_pinned(g, gd, false, "breach");
        g.gd[gd].react_t = 0.0;
        g.gd[gd].script = None;
        let fa = g.gd[gd].weapon;
        crate::weapons::firearm_rearm(g, fa);
        crate::character::character_update(g, c, 0.0);
        let face = g.ch[c].body_yaw;
        crate::squad::order(g, gd, OP { goal: Some(None), face: Some(Some(face)), stance: Some(Some(St::LowReady)), upper: Some(Some(Up::None)), aim_at: Some(None), light: Some(Some(false)), crouch: Some(Some(false)), ..Default::default() });
    }
    if let Some(d) = breach_drill_door(g) { shut(g, d); }
    if let Some(d) = breach_drill_next_door(g) { shut(g, d); }
    breach_drill_beat(g, "start");
}

/// Everybody off his script.
pub(super) fn breach_drill_end(g: &mut crate::game::Game) {
    let mut all = vec![dr!(g).a, dr!(g).b];
    all.extend_from_slice(&dr!(g).hostiles);
    for gd in all {
        if g.gd[gd].script.is_some() { crate::squad::end_guard_script(g, gd, Some(crate::squad::EndGuardScriptState::Patrol)); }
    }
}

/// The stun can in #2's left fist: two boxes off the hand bone, for one frame.
fn breach_drill_can_in_hand(g: &mut crate::game::Game) {
    let b = dr!(g).b;
    let c = g.gd[b].ch;
    let (hand, knuckle) = match (g.ch[c].bones.get(crate::character::Bone::HandL), g.ch[c].bones.get(crate::character::Bone::FingerL)) {
        (Some(h), Some(k)) => (*h, *k),
        _ => return,
    };
    let fingers = v3::normalize(v3::sub(knuckle, hand));
    let f = crate::character::character_forward(g, c);
    let mut axis = v3::cross(fingers, [0.0, 1.0, 0.0]);
    if v3::len(axis) < 0.2 { axis = v3::cross(fingers, f); }
    axis = v3::normalize(axis);
    if axis[1] < 0.0 { axis = v3::scale(axis, -1.0); }
    let centre = v3::mad(v3::mad(v3::lerp3(hand, knuckle, 0.55), axis, 0.03), fingers, 0.015);
    let yaw = m::atan2(axis[0], axis[2]);
    let up: V3 = if m::abs(axis[1]) > 0.95 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    let x = v3::normalize(v3::cross(up, axis));
    let y = v3::cross(axis, x);
    let rot = v3::quat::from_basis(x, y, axis);
    let owner = (js::to_i32(g.ch[c].id) & 0xff) as f64;
    let flag = crate::scene_boxes::box_flag();
    let box_ = crate::scene_boxes::make_box_kept(g, &crate::scene_boxes::MakeBoxP { c: centre, h: [0.032, 0.032, 0.07], yaw: Some(yaw), rot: Some(rot), albedo: Some([0.07, 0.075, 0.08]), stuff: Some(crate::scene_stuff::BoxStuff::Metal), flags: Some(flag.dynamic), owner: Some(owner), ..Default::default() });
    g.fx.push(crate::game::GameFxItem { box_, ttl: 1e-3 });
    let box_ = crate::scene_boxes::make_box_kept(g, &crate::scene_boxes::MakeBoxP { c: v3::mad(centre, axis, 0.045), h: [0.034, 0.034, 0.012], yaw: Some(yaw), rot: Some(rot), albedo: Some([0.7, 0.7, 0.66]), flags: Some((js::to_i32(flag.dynamic) | js::to_i32(flag.no_shadow)) as f64), owner: Some(owner), ..Default::default() });
    g.fx.push(crate::game::GameFxItem { box_, ttl: 1e-3 });
}

/// One frame of the drill.
pub(super) fn breach_drill_tick(g: &mut crate::game::Game, dt: f64) {
    let (a, b) = (dr!(g).a, dr!(g).b);
    let d = breach_drill_door(g);
    dr!(g).t += dt;
    let t = dr!(g).t;
    let d = match d { Some(d) => d, None => return };
    let phase = dr!(g).phase;
    if phase != Phase::Wait && phase != Phase::Approach && phase != Phase::Stack && phase != Phase::Done {
        crate::teampace::team_fight(g, a, None);
        crate::teampace::team_fight(g, b, None);
    }
    if breach_drill_casualties(g) { return; }
    if dr!(g).phase == Phase::Done { return; }
    let a_in = adv(g, 0, None);
    let b_in = adv(g, 1, None);
    let hs = dr!(g).hostiles.clone();
    for (k, h) in hs.iter().enumerate() {
        if !dr!(g).down_logged[k] && crate::condition::down(g, g.gd[*h].ch) { dr!(g).down_logged[k] = true; breach_drill_beat(g, &alloc::format!("down:{}", k)); }
    }
    if !dr!(g).t_set.is_nan() && !dr!(g).chopped && breach_drill_charge(g) { breach_drill_can_in_hand(g); }
    let (ca, cb) = (g.gd[a].ch, g.gd[b].ch);
    match dr!(g).phase {
        Phase::Wait => {
            if live_of(g).orders >= 1.0 {
                dr!(g).phase = Phase::Approach;
                breach_drill_beat(g, "go");
                dr!(g).t_run = t;
                let stack = dr!(g).plan.stack;
                route_i(g, 0, &[stack[0]], OP { speed: Some(Some(RUN)), face: Some(None), stance: Some(Some(St::LowReady)), upper: Some(Some(Up::None)), aim_at: Some(None), light: Some(Some(false)), crouch: Some(Some(false)), ..Default::default() });
                route_i(g, 1, &[stack[1]], OP { speed: Some(Some(RUN * 0.965)), face: Some(None), stance: Some(Some(St::LowReady)), upper: Some(Some(Up::None)), aim_at: Some(None), light: Some(Some(false)), crouch: Some(Some(false)), ..Default::default() });
            }
        }
        Phase::Approach => {
            let base = dr!(g).t_run;
            let stack0 = dr!(g).plan.stack[0];
            if v3::dist_xz(g.ch[ca].pos, stack0) < 3.2 || t > base + 16.0 {
                dr!(g).phase = Phase::Stack;
                dr!(g).t_stack = t;
                breach_drill_beat(g, "approach-done");
                let (face, side, aim_a, aim_b) = (dr!(g).plan.stack_face, dr!(g).plan.stack_side, dr!(g).aim_a, dr!(g).aim_b);
                crate::squad::order(g, a, OP { speed: Some(Some(1.7)), face: Some(Some(face[0])), stance: Some(Some(St::Stack)), stack_side: Some(Some(side)), upper: Some(Some(Up::None)), aim_at: Some(Some(aim_a)), ..Default::default() });
                crate::squad::order(g, b, OP { speed: Some(Some(1.75)), face: Some(Some(face[1])), stance: Some(Some(St::HighReady)), upper: Some(Some(Up::None)), aim_at: Some(Some(aim_b)), ..Default::default() });
            }
        }
        Phase::Stack => {
            let ts = t - dr!(g).t_stack;
            if ts > 0.5 { say!(g, a, "stack1", lines.stack1); }
            if dr!(g).t_set.is_nan() && ((a_in && b_in) || ts > 7.0) {
                dr!(g).t_set = t;
                breach_drill_beat(g, if a_in && b_in { "stacked" } else { "stacked-late" });
                if b_in { crate::character::anim_signal(g, cb, crate::character::SignalKind::Squeeze, None); }
            }
            let t_set = dr!(g).t_set;
            if !t_set.is_nan() && t > t_set + 0.3 { say!(g, b, "stack2", lines.stack2); }
            let live = live_of(g);
            if !t_set.is_nan() && t >= t_set + SET_HOLD && (!crate::character::anim_signalling(g, cb) || t > t_set + SET_HOLD + 0.6) && live.orders >= 2.0 {
                say!(g, a, "breach1", breach_lines.breach1);
                dr!(g).t_go = t;
                dr!(g).phase = Phase::Breach;
                breach_drill_beat(g, "squeeze");
                let (from, kf, mark, face, aim0) = (dr!(g).plan.kick_from, dr!(g).plan.kick_face, dr!(g).plan.toss.mark, dr!(g).plan.toss.face, dr!(g).plan.stack_aim[0]);
                route_i(g, 0, &[from], OP { speed: Some(Some(1.3)), face: Some(Some(kf)), stance: Some(Some(St::HighReady)), upper: Some(Some(Up::None)), strafe: Some(Some(false)), aim_at: Some(None), ..Default::default() });
                route_i(g, 1, &[mark], OP { speed: Some(Some(1.1)), face: Some(Some(face)), strafe: Some(Some(true)), stance: Some(Some(St::HighReady)), upper: Some(Some(Up::None)), aim_at: Some(Some(v3::copy(aim0))), ..Default::default() });
            }
        }
        Phase::Breach => {
            let (from, kf, t_go) = (dr!(g).plan.kick_from, dr!(g).plan.kick_face, dr!(g).t_go);
            let planted = a_in && v3::dist_xz(g.ch[ca].pos, from) < 0.07 && m::abs(v3::wrap_angle(g.ch[ca].body_yaw - kf)) < 0.1;
            if (planted && t > t_go + 0.5) || t > t_go + 1.8 {
                crate::squad::order(g, a, OP { goal: Some(None), aim_at: Some(None), ..Default::default() });
                crate::character::anim_kick_door(g, ca, Some(crate::character::AnimKickDoorOpts { leg: Some(crate::character::AnimKickDoorOptsLeg::Left) }));
                dr!(g).t_kick = t;
                dr!(g).phase = Phase::Kick;
                breach_drill_beat(g, "kick");
                breach_drill_beat(g, "kickL");
            }
        }
        Phase::Kick => tick_kick(g, t, d),
        Phase::Bang => tick_bang(g, t),
        Phase::Enter => {
            if dr!(g).t_enter.is_nan() {
                if t < dr!(g).t_bang + GO_AFTER_BANG { return; }
                dr!(g).t_enter = t;
                if !crate::condition::down(g, g.gd[a].ch) { crate::mind::set_state(g, a, crate::game::GuardState::Search, "breach"); }
                if !crate::condition::down(g, g.gd[b].ch) { crate::mind::set_state(g, b, crate::game::GuardState::Search, "breach"); }
                say!(g, a, "going1", breach_lines.going1);
                breach_drill_beat(g, "enter1");
                if !crate::condition::down(g, g.gd[a].ch) { breach_drill_start_flow(g, 0); }
            }
            let te = t - dr!(g).t_enter;
            let bounds = dr!(g).plan.bounds;
            if dr!(g).flows[1].st == FlowSt::Wait && !crate::condition::down(g, g.gd[b].ch) && te >= TWO_AFTER_ONE && (crate::squad::in_bounds(g.ch[ca].pos, bounds, None) || te > 2.5 || crate::condition::down(g, g.gd[a].ch)) {
                breach_drill_beat(g, "enter2");
                breach_drill_start_flow(g, 1);
            }
            breach_drill_tick_flow(g, 0, dt);
            breach_drill_tick_flow(g, 1, dt);
            let clear = breach_drill_room_clear(g);
            if clear { dr!(g).phase = Phase::Clear; dr!(g).t_clear = t; breach_drill_beat(g, "clear"); }
        }
        Phase::Clear => {
            let tc = t - dr!(g).t_clear;
            breach_drill_tick_hold(g, 0, dt);
            breach_drill_tick_hold(g, 1, dt);
            if tc > 0.3 { let who = if !crate::condition::down(g, g.gd[a].ch) { a } else { b }; say!(g, who, "roomClear1", breach_lines.room_clear1); }
            if tc >= 1.0 {  dr!(g).phase = Phase::Done; breach_drill_beat(g, "done"); }
        }
        Phase::Done => {}
    }
}

/// tick's 'kick' case: the chop, the leaf flung, the step back, the toss.
fn tick_kick(g: &mut Game, t: f64, d: DoorId) {
    let (a, b) = (dr!(g).a, dr!(g).b);
    let (c, cb) = (g.gd[a].ch, g.gd[b].ch);
    if !dr!(g).chopped && t >= dr!(g).t_kick + crate::character::KICK_IMPACT + TOSS_AFTER_IMPACT - CHOP_LEAD {
        dr!(g).chopped = true;
        if !crate::condition::down(g, g.gd[b].ch) { crate::character::anim_toss_item(g, cb, Some(CHOP_LEAD)); }
        if breach_drill_charge(g) { say!(g, b, "bang2", breach_lines.bang2); }
    }
    if dr!(g).chopped && breach_drill_charge(g) { breach_drill_can_in_hand(g); }
    if dr!(g).t_flung.is_nan() && (crate::character::anim_kick_impact(g, c) || crate::character::anim_kick_time(g, c) >= crate::character::KICK_IMPACT || !crate::character::anim_kicking(g, c)) {
        dr!(g).t_flung = t;
        let (from, kf, fling) = (dr!(g).plan.kick_from, dr!(g).plan.kick_face, dr!(g).plan.fling);
        let on_it = v3::dist_xz(g.ch[c].pos, from) < 0.4 && m::abs(v3::wrap_angle(g.ch[c].body_yaw - kf)) < 0.35;
        if m::abs(g.dr[d].leaf.angle) < 1.4 {
            g.dr[d].leaf.latched = false;
            g.dr[d].leaf.closing = false;
            g.dr[d].leaf.vel = if on_it { fling } else { fling * 0.32 };
            let since = if on_it { 0.0 } else { g.dr[d].leaf.since_bash };
            g.dr[d].leaf.since_bash = since;
            g.dr[d].leaf.quiet_t = -0.5;
            g.dr[d].leaf.creak_armed = false;
        }
        let id = g.ch[c].id;
        g.dr[d].leaf.last_who = id;
        if on_it && !g.dr[d].leaf.off {
            let at = crate::doors::door_pos(g, d);
            crate::audio_iface::game_audio_like_play(g, "doorBash", Some(at), Some(1.0), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.9 }));
            let p = g.ch[c].pos;
            crate::particle_fx::kick_dust_fx(g, d, p);
        }
        if !g.dr[d].leaf.off {
            let p = crate::doors::door_pos(g, d);
            let time = g.time;
            let serial = g.serial();   // (an event is an object: every heard-once set keys on its serial, and one born with none is "the first again")
            g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Kick, pos: [p[0], 0.0, p[2]], time, loud: true, level: Some(1.0), who: Some(id), serial, ..Default::default() });
        }
        say!(g, a, "in1", lines.in1);
        breach_drill_beat(g, if on_it { "door-flung" } else { "door-pushed" });
        let (mark, face) = (dr!(g).plan.retreat.mark, dr!(g).plan.retreat.face);
        let aim = breach_drill_inside(g, 1.2, 1.1, None);
        route_i(g, 0, &[mark], OP { speed: Some(Some(1.9)), face: Some(Some(face)), strafe: Some(Some(true)), stance: Some(Some(St::HighReady)), upper: Some(Some(Up::None)), aim_at: Some(Some(aim)), ..Default::default() });
        breach_drill_beat(g, "retreat");
    }
    let t_flung = dr!(g).t_flung;
    if !breach_drill_charge(g) && !t_flung.is_nan() && t >= t_flung + TOSS_AFTER_IMPACT {
        dr!(g).t_bang = t;
        breach_drill_beat(g, "nobang");
        breach_drill_muzzles_on_door(g);
        dr!(g).phase = Phase::Enter;
        return;
    }
    if !t_flung.is_nan() && dr!(g).can.is_none() && t >= t_flung + TOSS_AFTER_IMPACT {
        let hand = match crate::character::character_joint_under(g, cb, "handL") { Some(h) => h, None => v3::add(g.ch[cb].pos, [0.0, 1.3, 0.0]) };
        let from: V3 = [hand[0], m::max(1.0, hand[1]), hand[2]];
        let land = dr!(g).plan.toss.land;
        let sol = crate::throwables::throwables_solve(g, from, land, Some(9.0));
        let who = g.ch[cb].id;
        let it = crate::throwables::throwables_spawn(g, crate::throwables::ItemKind::Flash, from, sol.v0, Some(crate::throwables::ThrowablesSpawnOpts { fuse: Some(99.0), who: Some(who), heading: Some(v3::yaw_of(sol.v0)), ..Default::default() }));
        dr!(g).can = Some(it);
        g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::Can);
        crate::audio_iface::game_audio_like_play(g, "throw", Some(from), Some(0.7), None);
        dr!(g).t_toss = t;
        breach_drill_beat(g, "toss");
        let (rm, rf, tm, tf) = (dr!(g).plan.retreat.mark, dr!(g).plan.retreat.face, dr!(g).plan.toss.mark, dr!(g).plan.toss.face);
        let fa: V3 = [m::sin(rf), 0.0, m::cos(rf)];
        let fb: V3 = [m::sin(tf), 0.0, m::cos(tf)];
        let to_b = v3::sub(tm, rm);
        let side = js::or(m::sign(fa[2] * to_b[0] - fa[0] * to_b[2]), 1.0);
        crate::squad::order(g, a, OP { aim_at: Some(Some([rm[0] - fa[2] * side * -1.4 + fa[0] * 0.3, 0.1, rm[2] + fa[0] * side * -1.4 + fa[2] * 0.3])), ..Default::default() });
        crate::squad::order(g, b, OP { aim_at: Some(Some([tm[0] + fb[0] * 1.3, 0.1, tm[2] + fb[2] * 1.3])), ..Default::default() });
        dr!(g).phase = Phase::Bang;
    }
}

/// tick's 'bang' case: it lands, and goes off on the drill's cue with the guard list being the opposition for the call.
fn tick_bang(g: &mut Game, t: f64) {
    let it = dr!(g).can.unwrap();
    let landed = g.tab.item[it].bounces > 0.0 || g.tab.item[it].settled;
    if landed && dr!(g).t_land.is_nan() { dr!(g).t_land = t; breach_drill_beat(g, "land"); }
    let (t_land, t_toss) = (dr!(g).t_land, dr!(g).t_toss);
    if (!t_land.is_nan() && t >= t_land + FUSE_AFTER_LAND) || t > t_toss + 2.0 {
        g.tab.item[it].fired = true;
        dr!(g).t_bang = t;
        let hs = dr!(g).hostiles.clone();
        let up: Vec<GuardId> = hs.iter().copied().filter(|h| !crate::condition::down(g, g.gd[*h].ch)).collect();
        let all = core::mem::replace(&mut g.guards, up);
        crate::combat::detonate_stun(g, it, Some(false), Some(crate::combat::DetonateStunSam { side: Some(crate::teams::Team::Operative), spare: None }));
        g.guards = all;
        breach_drill_beat(g, "bang");
        breach_drill_muzzles_on_door(g);
        dr!(g).phase = Phase::Enter;
    }
}

/// Live: what a man down means; true when the drill has ended.
fn breach_drill_casualties(g: &mut crate::game::Game) -> bool {
    if dr!(g).phase == Phase::Done { return false; }
    let (a, b) = (dr!(g).a, dr!(g).b);
    let (a_down, b_down) = (crate::condition::down(g, g.gd[a].ch), crate::condition::down(g, g.gd[b].ch));
    if !a_down && !b_down { return false; }
    if crate::condition::down(g, g.gd[a].ch) && breach_drill_once(g, "opdown:a") { breach_drill_beat(g, "opdown:1"); }
    if crate::condition::down(g, g.gd[b].ch) && breach_drill_once(g, "opdown:b") { breach_drill_beat(g, "opdown:2"); }
    let why = if a_down && b_down { Some("wiped") } else if a_down && dr!(g).t_flung.is_nan() { Some("abort") } else { None };
    if let Some(why) = why {
        dr!(g).phase = Phase::Done;
        breach_drill_beat(g, why);
        if let Some(can) = dr!(g).can {
            if !g.tab.item[can].fired {
                let fuse = m::min(g.tab.item[can].fuse, FUSE_AFTER_LAND);
                g.tab.item[can].fuse = fuse;
                breach_drill_beat(g, "can:short-fuse");
            }
        }
        return true;
    }
    false
}

/// Heads back on the door.
fn breach_drill_muzzles_on_door(g: &mut crate::game::Game) {
    let pa = breach_drill_inside(g, 1.5, 1.1, None);
    let pb = breach_drill_inside(g, 1.0, 1.5, Some(0.3));
    let (a, b) = (dr!(g).a, dr!(g).b);
    set_aim(g, 0, pa);
    if !crate::condition::down(g, g.gd[a].ch) { crate::squad::order(g, a, OP { aim_at: Some(Some(pa)), ..Default::default() }); }
    set_aim(g, 1, pb);
    if !crate::condition::down(g, g.gd[b].ch) { crate::squad::order(g, b, OP { aim_at: Some(Some(pb)), ..Default::default() }); }
}

/// A point `d` metres through the doorway at height `y`, `s` along the wall.
fn breach_drill_inside(g: &mut crate::game::Game, d: f64, y: f64, s: Option<f64>) -> V3 {
    let s = s.unwrap_or(0.0);
    let door = breach_drill_door(g).unwrap();
    let fc = g.dr[door].frame_centre;
    let n = dr!(g).plan.inward.unwrap_or([1.0, 0.0]);
    [fc[0] + n[0] * d + n[1] * s, y, fc[2] + n[1] * d - n[0] * s]
}

/// `p` inside the room's authored bounds by at least `m`.
fn breach_drill_inside_room(g: &mut crate::game::Game, p: V3, m: f64) -> bool {
    let b = match dr!(g).plan.bounds { Some(b) => b, None => return true };
    p[0] > b.x0 + m && p[0] < b.x1 - m && p[2] > b.z0 + m && p[2] < b.z1 - m
}

/// Man i's way through the room.
fn breach_drill_flow_of(g: &mut crate::game::Game, i: usize) -> crate::scene_level::BreachFlow {
    dr!(g).plan.flow[i].clone()
}

/// Send man i through the frame.
fn breach_drill_start_flow(g: &mut crate::game::Game, i: usize) {
    let fl = breach_drill_flow_of(g, i);
    let gd = man(g, i);
    let t = dr!(g).t;
    let (centre, _) = targets(g, i);
    let f = &mut dr!(g).flows[i];
    f.st = FlowSt::Into;
    f.t0 = t;
    f.q = centre;
    f.on = -1;
    f.n = 0.0;
    f.next = 0.0;
    let aim = f.aim;
    let n = fl.into.len();
    let face = v3::yaw_to(fl.into[n.saturating_sub(2)], fl.into[n - 1]);
    route_i(g, i, &fl.into, OP { speed: Some(Some(2.9)), face: Some(Some(face)), strafe: Some(Some(false)), stance: Some(Some(St::None)), upper: Some(Some(Up::Aim)), aim_at: Some(Some(aim)), hot: Some(Some(true)), crouch: Some(Some(false)), ..Default::default() });
    crate::squad::order(g, gd, OP { light: Some(Some(true)), ..Default::default() });
    crate::squad::sweep_start(&mut dr!(g).flows[i].sweep, aim, &[fl.centre[0]], t, 0.9);
}

/// One frame of man i's flow.
fn breach_drill_tick_flow(g: &mut crate::game::Game, i: usize, dt: f64) {
    if dr!(g).flows[i].st == FlowSt::Wait { return; }
    let fl = breach_drill_flow_of(g, i);
    let gd = man(g, i);
    let t = dr!(g).t;
    if crate::condition::down(g, g.gd[gd].ch) { return; }
    let c = g.gd[gd].ch;
    let arrived = adv(g, i, Some(0.8));
    let engaged = breach_drill_staged_exchange(g, i, dt);
    if !engaged && dr!(g).flows[i].sweep.active { sweep_tick(g, i, t); }
    match dr!(g).flows[i].st {
        FlowSt::Into => {
            let last = fl.into[fl.into.len() - 1];
            if v3::dist_xz(g.ch[c].pos, last) < 0.55 || t > dr!(g).flows[i].t0 + 2.2 {
                let f = &mut dr!(g).flows[i];
                f.st = FlowSt::Corner;
                f.in_at = t;
                f.corner_at = t;
                if dr!(g).t_corner.is_nan() { dr!(g).t_corner = t; }
                let n = i + 1;
                breach_drill_beat(g, &alloc::format!("in{}", n));
                breach_drill_beat(g, &alloc::format!("centre{}", n));
                breach_drill_beat(g, &alloc::format!("corner{}", n));
                let old = dr!(g).flows[i].q.clone();
                let (_, corner) = targets(g, i);
                let mut q = Vec::new();
                for k in old.iter().copied() { if !hostile_dead(g, k) { q.push(k); } }
                for k in corner { if !hostile_dead(g, k) && !old.contains(&k) { q.push(k); } }
                let f = &mut dr!(g).flows[i];
                f.q = q;
                if f.on < 0 { f.n = 0.0; f.next = m::min(f.next, t + 0.05); }
                let aim = f.aim;
                route_i(g, i, &fl.corner, OP { speed: Some(Some(2.0)), face: Some(Some(fl.pod_face)), strafe: Some(Some(false)), stance: Some(Some(St::None)), upper: Some(Some(Up::Aim)), aim_at: Some(Some(aim)), hot: Some(Some(true)), ..Default::default() });
                let mut pts = vec![fl.centre[0], fl.centre[1]];
                pts.extend_from_slice(&fl.corner_aim);
                crate::squad::sweep_start(&mut dr!(g).flows[i].sweep, aim, &pts, t, CENTRE_SECS + 1.9);
            }
        }
        FlowSt::Centre => {
            let pending = dr!(g).flows[i].on >= 0 || breach_drill_pickable(g, i);
            let (active, in_at) = (dr!(g).flows[i].sweep.active, dr!(g).flows[i].in_at);
            if (!active && !pending && t > in_at + CENTRE_SECS) || t > in_at + 3.2 {
                let f = &mut dr!(g).flows[i];
                f.st = FlowSt::Corner;
                f.corner_at = t;
                if dr!(g).t_corner.is_nan() { dr!(g).t_corner = t; }
                breach_drill_beat(g, &alloc::format!("corner{}", i + 1));
                let old = dr!(g).flows[i].q.clone();
                let (_, corner) = targets(g, i);
                let mut q = Vec::new();
                for k in corner { if !hostile_dead(g, k) { q.push(k); } }
                for k in old { if !hostile_dead(g, k) { q.push(k); } }
                let f = &mut dr!(g).flows[i];
                f.q = q;
                f.on = -1;
                f.n = 0.0;
                f.next = t + 0.1;
                let aim = f.aim;
                route_i(g, i, &fl.corner, OP { speed: Some(Some(1.7)), face: Some(Some(fl.pod_face)), strafe: Some(Some(false)), stance: Some(Some(St::None)), upper: Some(Some(Up::Aim)), aim_at: Some(Some(aim)), hot: Some(Some(true)), ..Default::default() });
                crate::squad::sweep_start(&mut dr!(g).flows[i].sweep, aim, &fl.corner_aim, t, 1.9);
            }
        }
        FlowSt::Corner => {
            let pending = dr!(g).flows[i].on >= 0 || breach_drill_pickable(g, i);
            let (active, corner_at) = (dr!(g).flows[i].sweep.active, dr!(g).flows[i].corner_at);
            if ((arrived || t > corner_at + 4.0) && !active && !pending) || t > corner_at + 6.5 {
                dr!(g).flows[i].st = FlowSt::Hold;
                breach_drill_beat(g, &alloc::format!("hold{}", i + 1));
                if i == 0 { say!(g, gd, "clear1", breach_lines.clear1); } else { say!(g, gd, "clear2", breach_lines.clear2); }
                let aim = dr!(g).flows[i].aim;
                crate::squad::sweep_start(&mut dr!(g).flows[i].sweep, aim, &[fl.hold], t, 0.9);
                crate::squad::order(g, gd, OP { face: Some(Some(fl.pod_face)), ..Default::default() });
            }
        }
        FlowSt::Hold | FlowSt::Wait => {}
    }
}

/// The clear beat's exchange and resting muzzle.
fn breach_drill_tick_hold(g: &mut crate::game::Game, i: usize, dt: f64) {
    if !breach_drill_staged_exchange(g, i, dt) && dr!(g).flows[i].sweep.active { let t = dr!(g).t; sweep_tick(g, i, t); }
}

/// Both holding and every hostile down, a beat after.
fn breach_drill_room_clear(g: &mut crate::game::Game) -> bool {
    let own = crate::roomfight::room_fight_on(g);
    let hs = dr!(g).hostiles.clone();
    let mut all_down = true;
    for h in hs.iter().copied() {
        let gone = crate::condition::down(g, g.gd[h].ch) || g.gd[h].fled || (own && { let p = g.ch[g.gd[h].ch].pos; !breach_drill_inside_room(g, p, -0.3) });
        if !gone { all_down = false; break; }
    }
    let (a, b) = (dr!(g).a, dr!(g).b);
    let (st0, st1) = (dr!(g).flows[0].st, dr!(g).flows[1].st);
    let holding = (crate::condition::down(g, g.gd[a].ch) || st0 == FlowSt::Hold) && (crate::condition::down(g, g.gd[b].ch) || st1 == FlowSt::Hold);
    if holding && all_down {
        let t = dr!(g).t;
        if dr!(g).t_all_done.is_nan() { dr!(g).t_all_done = t; }
        return t > dr!(g).t_all_done + 0.5;
    }
    false
}

/// Is the front of man i's list a living man he has a line to.
fn breach_drill_pickable(g: &mut crate::game::Game, i: usize) -> bool {
    let op = man(g, i);
    if crate::roomfight::room_fight_on(g) { return crate::roomfight::room_fight_contact(g, op); }
    let c = g.gd[op].ch;
    while let Some(k) = dr!(g).flows[i].q.first().copied() {
        let gone = match host(g, k) { Some(h) => crate::condition::down(g, g.gd[h].ch) || g.gd[h].fled, None => false };
        if !gone { break; }
        dr!(g).flows[i].q.remove(0);
    }
    let eye = bone_or(g, c, crate::character::Bone::Head, 1.6);
    let q = dr!(g).flows[i].q.clone();
    let n = dr!(g).hostiles.len();
    let mut order_ = q.clone();
    for j in 0..n { if !q.contains(&j) { order_.push(j); } }
    let mut found = None;
    for k in order_ {
        let hk = match host(g, k) { Some(h) => h, None => continue };
        if crate::condition::down(g, g.gd[hk].ch) || g.gd[hk].fled { continue; }
        let ch = g.gd[hk].ch;
        let chest = bone_or(g, ch, crate::character::Bone::Chest, 1.1);
        let energy = crate::glasspen::shot_energy(g, c, None);
        if !crate::scene_collision::shot_blocked(g, eye, chest, Some(0.02), energy) { found = Some(k); break; }
    }
    let k = match found { Some(k) => k, None => return false };
    let mut nq = vec![k];
    nq.extend(q.iter().copied().filter(|j| *j != k));
    dr!(g).flows[i].q = nq;
    true
}

/// The exchange as staged: the muzzle on a man, the settle, the controlled pair; true while on somebody.
fn breach_drill_staged_exchange(g: &mut crate::game::Game, i: usize, dt: f64) -> bool {
    let op = man(g, i);
    let c = g.gd[op].ch;
    let t = dr!(g).t;
    if crate::roomfight::room_fight_on(g) { return crate::roomfight::room_fight_engaged(g, op); }
    if dr!(g).flows[i].st == FlowSt::Wait || crate::condition::down(g, g.gd[op].ch) { return false; }
    if dr!(g).flows[i].st == FlowSt::Into { let p = g.ch[c].pos; if !breach_drill_inside_room(g, p, FIRE_INSIDE_M) { return false; } }
    let on = dr!(g).flows[i].on;
    if on >= 0 && dr!(g).flows[i].n == 0.0 {
        let h = host(g, on as usize).unwrap();
        if crate::condition::down(g, g.gd[h].ch) || g.gd[h].fled { let f = &mut dr!(g).flows[i]; f.on = -1; f.next = t + SHIFT; }
    }
    if dr!(g).flows[i].on < 0 && t >= dr!(g).flows[i].next && breach_drill_pickable(g, i) {
        let f = &mut dr!(g).flows[i];
        f.on = f.q[0] as i32;
        f.since = t;
        f.n = 0.0;
        let on = f.on;
        breach_drill_beat(g, &alloc::format!("eyes{}:{}", i + 1, on));
    }
    let on = dr!(g).flows[i].on;
    if on < 0 { return false; }
    let h = host(g, on as usize).unwrap();
    let ch = g.gd[h].ch;
    let chest = bone_or(g, ch, crate::character::Bone::Chest, 1.1);
    let aim = dr!(g).flows[i].aim;
    set_aim(g, i, [v3::damp(aim[0], chest[0], 16.0, dt), v3::damp(aim[1], chest[1], 16.0, dt), v3::damp(aim[2], chest[2], 16.0, dt)]);
    let due = if dr!(g).flows[i].n == 0.0 { dr!(g).flows[i].since + SETTLE } else { dr!(g).flows[i].next };
    let fa = g.gd[op].weapon;
    if t >= due {
        let head = bone_or(g, c, crate::character::Bone::Head, 1.6);
        let def = g.fa[fa].def;
        let energy = crate::glasspen::shot_energy(g, c, Some(def));
        if crate::scene_collision::shot_blocked(g, head, chest, Some(0.02), energy) {
            let f = &mut dr!(g).flows[i];
            f.on = -1;
            f.next = t + 0.1;
            return false;
        }
    }
    if t >= due && !crate::weapons::firearm_reloading(g, fa) {
        if crate::weapons::firearm_fire(g, fa) {
            crate::combat::fire_weapon(g, c, chest, Some(&crate::combat::FireOpts { steady: Some(true), spread: Some(0.01), ..Default::default() }));
            dr!(g).flows[i].n += 1.0;
            breach_drill_beat(g, &alloc::format!("shot{}", i + 1));
            if dr!(g).flows[i].n >= 2.0 {
                let f = &mut dr!(g).flows[i];
                f.q.retain(|k| *k != on as usize);
                f.on = -1;
                f.next = t + SHIFT;
            } else {
                dr!(g).flows[i].next = t + PAIR;
            }
        } else {
            crate::weapons::firearm_rearm(g, fa);
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn live_lines_by_charge_and_key() {
        assert_eq!(live_text(false, "stack2", "show"), "set.");
        assert_eq!(live_text(true, "stack2", "show"), "show");
        assert_eq!(live_text(true, "clear1", "show"), "right side — clear. i've got the door.");
        assert_eq!(live_text(true, "clear2", "show"), "left clear.");
        assert_eq!(live_text(true, "roomClear1", "show"), "room clear.");
    }
}
