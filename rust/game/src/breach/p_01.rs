//! src/game/breach.ts lines 1 to 458
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};


static LIVE_LINES: &[(&str, &str)] = &[("clear1", "right side — clear."), ("clear2", "left clear. i have the door."), ("roomClear1", "room clear.")];
static LIVE_GUIDE: &[(&str, &str)] = &[("clear1", "right side — clear. i've got the door.")];
static LIVE_NOW: &[(&str, &str)] = &[("clear1", "right side — clear. i have the door."), ("clear2", "left clear.")];
static LIVE_NO_CHARGE: &[(&str, &str)] = &[("stack1", "stack on me — side door. on my boot."), ("stack2", "set.")];

type OP = crate::squad::OrderPatch;
type St = crate::character::Stance;
type Up = crate::character::UpperMode;

/// The drill's fields: never a function, so no reference into `g` is ever returned or kept.
macro_rules! dr { ($g:expr) => { $g.night_breach.drill.as_mut().unwrap() } }
/// A line of the plan said by a man (the text is cloned out of the plan before the call).
macro_rules! say { ($g:expr, $gd:expr, $key:expr, $($path:ident).+) => {{ let s = dr!($g).plan.$($path).+.clone(); breach_drill_say($g, $gd, $key, &s, None); }} }

// ---------------------------------------------------------------- seam: what skeleton 3 cannot yet carry
fn seam_audio_play_rate(g: &mut Game, name: &str, pos: V3, gain: f64, rate: f64) { todo!("needs: GameAudioLike.play's opts able to hold a rate (crate::Opaque holds nothing)") }

// ---------------------------------------------------------------- small private helpers
fn dead(g: &Game, gd: GuardId) -> bool { g.gd[gd].state == crate::game::GuardState::Dead }
fn man(g: &mut Game, i: usize) -> GuardId { if i == 0 { dr!(g).a } else { dr!(g).b } }
/// `H[k]`: undefined past the end or below it.
fn host(g: &mut Game, k: f64) -> Option<GuardId> { if k < 0.0 { return None; } dr!(g).hostiles.get(k as usize).copied() }
/// `c.bones.<b> ?? v3.add(c.pos, [0, y, 0])`
fn bone_or(g: &Game, c: CharId, b: crate::character::Bone, y: f64) -> V3 {
    match g.ch[c].bones.get(b) { Some(p) => *p, None => v3::add(g.ch[c].pos, [0.0, y, 0.0]) }
}
/// The drill's live object is NightBreach's own, which it writes as play goes: the copy on the drill only says whether there is one.
fn live_of(g: &mut Game) -> Option<BreachLive> { if dr!(g).live.is_some() { Some(g.night_breach.live) } else { None } }
fn look(table: &'static [(&'static str, &'static str)], key: &str) -> Option<&'static str> { table.iter().find(|r| r.0 == key).map(|r| r.1) }
/// What a live pair says in place of the show's line (WORDS.on and WORDS.guide settled on).
fn live_text<'a>(charge: bool, key: &str, text: &'a str) -> &'a str {
    if !charge { if let Some(s) = look(LIVE_NO_CHARGE, key) { return s; } }
    look(LIVE_GUIDE, key).or_else(|| look(LIVE_NOW, key)).or_else(|| look(LIVE_LINES, key)).unwrap_or(text)
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
fn targets(g: &mut Game, i: usize) -> (Vec<f64>, Vec<f64>) {
    let t = &dr!(g).plan.targets;
    if i == 0 { (t.0.centre.clone(), t.0.corner.clone()) } else { (t.1.centre.clone(), t.1.corner.clone()) }
}
fn hostile_dead(g: &mut Game, k: f64) -> bool { match host(g, k) { Some(h) => dead(g, h), None => false } }
fn clear_leg(g: &mut Game, p: V3, q: V3) -> bool {
    !crate::scene_collision::solid_blocked(g, [p[0], 0.45, p[2]], [q[0], 0.45, q[2]], None) && !crate::scene_collision::solid_blocked(g, [p[0], 1.2, p[2]], [q[0], 1.2, q[2]], None)
}
fn mk(aim: V3) -> OpFlow {
    OpFlow { st: FlowSt::Wait, t0: 0.0, wps: Vec::new(), sweep: crate::squad::Sweep::blank(), aim, q: Vec::new(), on: -1.0, since: 0.0, n: 0.0, next: 0.0, said: false, in_at: f64::NAN, corner_at: f64::NAN, hold_at: f64::NAN }
}

// ---------------------------------------------------------------- the drill
/// Flat distance.   (breach.ts flat)
pub fn flat(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }

/// A drill, not yet started.   (breach.ts BreachDrill.constructor)
pub fn breach_drill_new(g: &mut crate::game::Game, plan: &crate::scene_level::BreachPlan, a: GuardId, b: GuardId, hostiles: &[GuardId], opts: Option<crate::breach::BreachDrillNewOpts>) -> crate::breach::BreachDrill {
    let opts = opts.unwrap_or_default();
    let mut d = BreachDrill::blank();
    d.plan = plan.clone();
    d.a = a;
    d.b = b;
    d.hostiles = hostiles.to_vec();
    d.aim_a = v3::copy(plan.stack_aim[0]);
    d.aim_b = v3::copy(plan.stack_aim[1]);
    d.flows = [mk(d.aim_a), mk(d.aim_b)];
    d.down_logged = hostiles.iter().map(|_| false).collect();
    d.guarantee_downs = opts.guarantee_downs.unwrap_or(true);
    d.live = opts.live;
    d
}

/// The door to breach.   (breach.ts BreachDrill.door)
pub fn breach_drill_door(g: &mut crate::game::Game) -> Option<DoorId> {
    let n = dr!(g).plan.door.clone();
    crate::doors::doors_by_name(g, &n)
}

/// The next door.   (breach.ts BreachDrill.nextDoor)
pub fn breach_drill_next_door(g: &mut crate::game::Game) -> Option<DoorId> {
    let n = dr!(g).plan.next.door.clone();
    crate::doors::doors_by_name(g, &n)
}

/// Log a beat.   (breach.ts BreachDrill.beat)
pub fn breach_drill_beat(g: &mut crate::game::Game, what: &str) {
    let t = dr!(g).t;
    dr!(g).beats.push(BreachDrillBeatsItem { t, what: String::from(what) });
}

/// True the first time for a key.   (breach.ts BreachDrill.once)
pub fn breach_drill_once(g: &mut crate::game::Game, key: &str) -> bool {
    dr!(g).done.add(String::from(key))
}

/// Is there a stun charge in this entry.   (breach.ts BreachDrill.charge)
pub fn breach_drill_charge(g: &mut crate::game::Game) -> bool {
    match live_of(g) {
        None => true,
        Some(l) => { let b = dr!(g).b; l.stun && !dead(g, b) }
    }
}

/// A line, once, by a living man; live play swaps what the drill cannot know.   (breach.ts BreachDrill.say)
pub fn breach_drill_say(g: &mut crate::game::Game, gd: GuardId, key: &str, text: &str, radio: Option<bool>) {
    let radio = radio.unwrap_or(false);
    let text = if live_of(g).is_some() { let charge = breach_drill_charge(g); live_text(charge, key, text) } else { text };
    if !text.is_empty() && breach_drill_once(g, &alloc::format!("say:{}", key)) && !dead(g, gd) {
        crate::game::game_say(g, gd, text, Some(radio), None, None);
    }
}

/// New waypoints and the order onto the first.   (breach.ts BreachDrill.route)
pub fn breach_drill_route(g: &mut crate::game::Game, gd: GuardId, wps: &mut Vec<V3>, pts: &[V3], patch: Option<crate::breach::BreachDrillRoutePatch>) {
    let p = patch.unwrap_or_default();
    let o = OP { goal: p.goal.map(Some), speed: p.speed.map(Some), face: p.face.map(Some), strafe: p.strafe.map(Some), aim_at: p.aim_at.map(Some), stance: p.stance.map(Some), stack_side: p.stack_side.map(Some), crouch: p.crouch.map(Some), lean: p.lean.map(Some), upper: p.upper.map(Some), light: p.light.map(Some), arrived: p.arrived.map(Some), hot: p.hot.map(Some), flinch: p.flinch.map(Some), direct: p.direct.map(Some), smoke: p.smoke.map(Some), over: p.over.map(Some), held: false };
    route_on(g, gd, wps, pts, o);
}

/// Past a waypoint, on to the next; true when on the last and arrived.   (breach.ts BreachDrill.advance)
pub fn breach_drill_advance(g: &mut crate::game::Game, gd: GuardId, wps: &mut Vec<V3>, pass: Option<f64>) -> bool {
    let pass = pass.unwrap_or(0.7);
    if dead(g, gd) { return true; }
    let c = g.gd[gd].ch;
    while wps.len() > 1 && flat(g.ch[c].pos, wps[0]) < pass {
        wps.remove(0);
        crate::squad::order(g, gd, OP { goal: Some(Some(wps[0])), ..Default::default() });
    }
    wps.len() <= 1 && g.gd[gd].script.as_ref().map_or(false, |s| s.arrived == Some(true))
}

/// Is a point inside the doorway's opening.   (breach.ts BreachDrill.inFunnel)
pub fn breach_drill_in_funnel(g: &mut crate::game::Game, p: V3) -> bool {
    let d = match breach_drill_door(g) { Some(d) => d, None => return false };
    let [hx, hz] = g.dr[d].def.hinge;
    let along_x = m::abs(m::sin(g.dr[d].def.closed_dir)) < 0.5;
    let (u0, u, w) = if along_x { (hx, p[0], p[2] - hz) } else { (hz, p[2], p[0] - hx) };
    u > u0 - 0.05 && u < u0 + g.dr[d].def.width + 0.05 && m::abs(w) < 0.9
}

/// A door pulled to and latched.
fn shut(g: &mut Game, d: DoorId) {
    crate::doors::door_reset(g, d);
    crate::doors::door_set_angle(g, d, 0.0);
    crate::doors::door_set_vel(g, d, 0.0);
    crate::doors::door_set_latched(g, d, true);
    crate::doors::door_set_closing(g, d, false);
}

/// Everybody on his mark, the leaves shut.   (breach.ts BreachDrill.start)
pub fn breach_drill_start(g: &mut crate::game::Game) {
    for i in 0..2 {
        let gd = man(g, i);
        let c = g.gd[gd].ch;
        let stack = dr!(g).plan.stack[i];
        let from = match dr!(g).plan.from { Some(f) => f[i], None => stack };
        g.ch[c].pos = v3::copy(from);
        g.ch[c].vel = [0.0, 0.0, 0.0];
        let yaw = v3::yaw_to(from, stack);
        g.ch[c].aim_yaw = yaw;
        g.ch[c].body_yaw = yaw;
        g.ch[c].aim_pitch = 0.0;
        g.gd[gd].speed = 0.0;
        g.gd[gd].state = crate::game::GuardState::Patrol;
        g.gd[gd].awareness = 0.0;
        g.gd[gd].last_known = None;
        g.gd[gd].hold = false;
        g.gd[gd].pinned = false;
        g.gd[gd].react_t = 0.0;
        g.gd[gd].script = None;
        let fa = g.gd[gd].weapon;
        crate::weapons::firearm_rearm(g, fa);
        crate::character::character_update(g, c, 0.0);
        let face = g.ch[c].body_yaw;
        crate::squad::order(g, gd, OP { goal: Some(None), face: Some(Some(face)), stance: Some(Some(St::LowReady)), upper: Some(Some(Up::None)), aim_at: Some(None), light: Some(Some(false)), crouch: Some(Some(false)), ..Default::default() });
    }
    if live_of(g).is_none() {
        let hs = dr!(g).plan.hostiles.clone();
        for (i, h) in hs.iter().enumerate() {
            let gd = match dr!(g).hostiles.get(i).copied() { Some(x) => x, None => continue };
            let c = g.gd[gd].ch;
            g.ch[c].pos = v3::copy(h.pos);
            g.ch[c].vel = [0.0, 0.0, 0.0];
            g.ch[c].aim_yaw = h.yaw;
            g.ch[c].body_yaw = h.yaw;
            g.gd[gd].state = crate::game::GuardState::Patrol;
            g.gd[gd].awareness = 0.0;
            crate::character::character_update(g, c, 0.0);
            crate::squad::order(g, gd, OP { goal: Some(None), face: Some(Some(h.yaw)), aim_at: Some(Some(v3::copy(h.aim_at))), stance: Some(Some(St::None)), upper: Some(Some(Up::Aim)), crouch: Some(Some(h.crouch == Some(true))), light: Some(Some(false)), ..Default::default() });
        }
    }
    if let Some(d) = breach_drill_door(g) { shut(g, d); }
    if let Some(d) = breach_drill_next_door(g) { shut(g, d); }
    breach_drill_beat(g, "start");
}

/// Everybody off his script.   (breach.ts BreachDrill.end)
pub fn breach_drill_end(g: &mut crate::game::Game) {
    let mut all = vec![dr!(g).a, dr!(g).b];
    all.extend_from_slice(&dr!(g).hostiles);
    for gd in all {
        if g.gd[gd].script.is_some() { crate::squad::end_guard_script(g, gd, Some(crate::squad::EndGuardScriptState::Patrol)); }
    }
}

/// The stun can in #2's left fist: two boxes off the hand bone, for one frame.   (breach.ts BreachDrill.canInHand)
pub fn breach_drill_can_in_hand(g: &mut crate::game::Game) {
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
    let box_ = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP { c: centre, h: [0.032, 0.032, 0.07], yaw: Some(yaw), rot: Some(rot), albedo: Some([0.07, 0.075, 0.08]), stuff: Some(crate::scene_stuff::BoxStuff::Metal), flags: Some(flag.dynamic), owner: Some(owner), ..Default::default() });
    g.fx.push(crate::game::GameFxItem { box_, ttl: 1e-3 });
    let box_ = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP { c: v3::mad(centre, axis, 0.045), h: [0.034, 0.034, 0.012], yaw: Some(yaw), rot: Some(rot), albedo: Some([0.7, 0.7, 0.66]), flags: Some((js::to_i32(flag.dynamic) | js::to_i32(flag.no_shadow)) as f64), owner: Some(owner), ..Default::default() });
    g.fx.push(crate::game::GameFxItem { box_, ttl: 1e-3 });
}

/// One frame of the drill.   (breach.ts BreachDrill.tick)
pub fn breach_drill_tick(g: &mut crate::game::Game, dt: f64) {
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
    if live_of(g).is_some() {
        if breach_drill_casualties(g, t) { return; }
    } else if dead(g, a) || dead(g, b) {
        if dr!(g).phase != Phase::Done { dr!(g).phase = Phase::Done; dr!(g).t_done = t; breach_drill_beat(g, "abort"); }
        return;
    }
    if dr!(g).phase == Phase::Done { return; }
    let a_in = adv(g, 0, None);
    let b_in = adv(g, 1, None);
    let hs = dr!(g).hostiles.clone();
    for (k, h) in hs.iter().enumerate() {
        if !dr!(g).down_logged[k] && dead(g, *h) { dr!(g).down_logged[k] = true; breach_drill_beat(g, &alloc::format!("down:{}", k)); }
    }
    if !dr!(g).t_set.is_nan() && !dr!(g).chopped && breach_drill_charge(g) { breach_drill_can_in_hand(g); }
    let (ca, cb) = (g.gd[a].ch, g.gd[b].ch);
    match dr!(g).phase {
        Phase::Wait => {
            let go = match live_of(g) { Some(l) => l.orders >= 1.0, None => t >= HOLD_OPEN };
            if go {
                dr!(g).phase = Phase::Approach;
                breach_drill_beat(g, "go");
                dr!(g).t_run = t;
                let stack = dr!(g).plan.stack;
                route_i(g, 0, &[stack[0]], OP { speed: Some(Some(RUN)), face: Some(None), stance: Some(Some(St::LowReady)), upper: Some(Some(Up::None)), aim_at: Some(None), light: Some(Some(false)), crouch: Some(Some(false)), ..Default::default() });
                route_i(g, 1, &[stack[1]], OP { speed: Some(Some(RUN * 0.965)), face: Some(None), stance: Some(Some(St::LowReady)), upper: Some(Some(Up::None)), aim_at: Some(None), light: Some(Some(false)), crouch: Some(Some(false)), ..Default::default() });
            }
        }
        Phase::Approach => {
            let base = if live_of(g).is_some() { dr!(g).t_run } else { HOLD_OPEN };
            let stack0 = dr!(g).plan.stack[0];
            if flat(g.ch[ca].pos, stack0) < 3.2 || t > base + 16.0 {
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
            let set_hold = live.and_then(|l| l.set_hold).unwrap_or(SET_HOLD);
            if !t_set.is_nan() && t >= t_set + set_hold && (!crate::character::anim_signalling(g, cb) || t > t_set + set_hold + 0.6) && live.map_or(true, |l| l.orders >= 2.0) {
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
            let planted = a_in && flat(g.ch[ca].pos, from) < 0.07 && m::abs(v3::wrap_angle(g.ch[ca].body_yaw - kf)) < 0.1;
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
                if !dead(g, a) { g.gd[a].state = crate::game::GuardState::Search; }
                if !dead(g, b) { g.gd[b].state = crate::game::GuardState::Search; }
                say!(g, a, "going1", breach_lines.going1);
                breach_drill_beat(g, "enter1");
                if !dead(g, a) { breach_drill_start_flow(g, 0.0); }
            }
            let te = t - dr!(g).t_enter;
            let bounds = dr!(g).plan.bounds;
            if dr!(g).flows[1].st == FlowSt::Wait && !dead(g, b) && te >= TWO_AFTER_ONE && (crate::squad::in_bounds(g.ch[ca].pos, bounds, None) || te > 2.5 || dead(g, a)) {
                breach_drill_beat(g, "enter2");
                breach_drill_start_flow(g, 1.0);
            }
            breach_drill_tick_flow(g, 0.0, dt);
            breach_drill_tick_flow(g, 1.0, dt);
            let clear = breach_drill_room_clear(g, te);
            if clear { dr!(g).phase = Phase::Clear; dr!(g).t_clear = t; breach_drill_beat(g, "clear"); }
        }
        Phase::Clear => {
            let tc = t - dr!(g).t_clear;
            breach_drill_tick_hold(g, 0.0, dt);
            breach_drill_tick_hold(g, 1.0, dt);
            if tc > 0.3 { let who = if !dead(g, a) { a } else { b }; say!(g, who, "roomClear1", breach_lines.room_clear1); }
            if live_of(g).is_some() {
                if tc >= 1.0 { dr!(g).t_done = t; dr!(g).phase = Phase::Done; breach_drill_beat(g, "done"); }
                return;
            }
            if tc >= NEXT_AFTER_CLEAR && dr!(g).t_next.is_nan() {
                dr!(g).t_next = t;
                breach_drill_beat(g, "next");
                say!(g, a, "next1", breach_lines.next1);
                let (stack, face, side, aims) = (dr!(g).plan.next.stack, dr!(g).plan.next.stack_face, dr!(g).plan.next.stack_side, dr!(g).plan.next.stack_aim);
                let (aim_a, aim_b) = (dr!(g).aim_a, dr!(g).aim_b);
                let leg = breach_drill_leg_to(g, b, stack[0]);
                route_i(g, 1, &leg, OP { speed: Some(Some(2.0)), face: Some(Some(face[0])), strafe: Some(Some(false)), stance: Some(Some(St::Stack)), stack_side: Some(Some(side)), upper: Some(Some(Up::Aim)), aim_at: Some(Some(aim_b)), hot: Some(Some(true)), crouch: Some(Some(false)), ..Default::default() });
                let leg = breach_drill_leg_to(g, a, stack[1]);
                route_i(g, 0, &leg, OP { speed: Some(Some(2.3)), face: Some(Some(face[1])), strafe: Some(Some(false)), stance: Some(Some(St::HighReady)), upper: Some(Some(Up::Aim)), aim_at: Some(Some(aim_a)), hot: Some(Some(true)), crouch: Some(Some(false)), ..Default::default() });
                crate::squad::sweep_start(&mut dr!(g).flows[0].sweep, aim_a, &[aims[1]], t, 1.2);
                crate::squad::sweep_start(&mut dr!(g).flows[1].sweep, aim_b, &[aims[0]], t, 0.9);
                dr!(g).phase = Phase::Restack;
            }
        }
        Phase::Restack => {
            let tn = t - dr!(g).t_next;
            for i in 0..2 { if dr!(g).flows[i].sweep.active { sweep_tick(g, i, t); } }
            if tn > 1.6 { let s = dr!(g).plan.breach_lines.radio1.clone(); breach_drill_say(g, a, "radio1", &s, Some(true)); }
            if dr!(g).t_stacked2.is_nan() && ((a_in && b_in) || tn > RESTACK_CAP) {
                dr!(g).t_stacked2 = t;
                breach_drill_beat(g, if a_in && b_in { "stack2" } else { "stack2-late" });
                if b_in { crate::character::anim_signal(g, cb, crate::character::SignalKind::Squeeze, None); }
            }
            let t2 = dr!(g).t_stacked2;
            if !t2.is_nan() && t >= t2 + RESTACK_HOLD && dr!(g).t_done.is_nan() {
                dr!(g).t_done = t;
                dr!(g).phase = Phase::Done;
                breach_drill_beat(g, "cut");
                breach_drill_beat(g, "done");
            }
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
        if !dead(g, b) { crate::character::anim_toss_item(g, cb, Some(CHOP_LEAD)); }
        if breach_drill_charge(g) { say!(g, b, "bang2", breach_lines.bang2); }
    }
    if dr!(g).chopped && breach_drill_charge(g) { breach_drill_can_in_hand(g); }
    if dr!(g).t_flung.is_nan() && (crate::character::anim_kick_impact(g, c) || crate::character::anim_kick_time(g, c) >= crate::character::KICK_IMPACT || !crate::character::anim_kicking(g, c)) {
        dr!(g).t_flung = t;
        let (from, kf, fling) = (dr!(g).plan.kick_from, dr!(g).plan.kick_face, dr!(g).plan.fling);
        let on_it = flat(g.ch[c].pos, from) < 0.4 && m::abs(v3::wrap_angle(g.ch[c].body_yaw - kf)) < 0.35;
        if m::abs(crate::doors::door_angle(g, d)) < 1.4 {
            crate::doors::door_set_latched(g, d, false);
            crate::doors::door_set_closing(g, d, false);
            crate::doors::door_set_vel(g, d, if on_it { fling } else { fling * 0.32 });
            let since = if on_it { 0.0 } else { crate::doors::door_since_bash(g, d) };
            crate::doors::door_set_since_bash(g, d, since);
            crate::doors::door_set_quiet_t(g, d, -0.5);
            crate::doors::door_set_creak_armed(g, d, false);
        }
        let id = g.ch[c].id;
        crate::doors::door_set_last_who(g, d, id);
        if on_it && !crate::doors::door_off(g, d) {
            let at = crate::doors::door_pos(g, d);
            seam_audio_play_rate(g, "doorBash", at, 1.0, 0.9);
            let p = g.ch[c].pos;
            crate::particle_fx::kick_dust_fx(g, d, p);
        }
        if live_of(g).map_or(false, |l| l.loud_boot) && !crate::doors::door_off(g, d) {
            let p = crate::doors::door_pos(g, d);
            let time = g.time;
            g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Kick, pos: [p[0], 0.0, p[2]], time, loud: true, level: Some(1.0), who: Some(id), ..Default::default() });
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
    let landed = crate::throwables::item_bounces(g, it) > 0.0 || crate::throwables::item_settled(g, it);
    if landed && dr!(g).t_land.is_nan() { dr!(g).t_land = t; breach_drill_beat(g, "land"); }
    let (t_land, t_toss) = (dr!(g).t_land, dr!(g).t_toss);
    if (!t_land.is_nan() && t >= t_land + FUSE_AFTER_LAND) || t > t_toss + 2.0 {
        crate::throwables::item_set_fired(g, it, true);
        dr!(g).t_bang = t;
        let hs = dr!(g).hostiles.clone();
        let up: Vec<GuardId> = hs.iter().copied().filter(|h| !dead(g, *h)).collect();
        let all = core::mem::replace(&mut g.guards, up);
        crate::combat::detonate_stun(g, it, Some(false), Some(crate::combat::DetonateStunSam { side: Some(crate::teams::Team::Operative), spare: None }));
        g.guards = all;
        breach_drill_beat(g, "bang");
        if live_of(g).is_none() {
            for (i, h) in hs.iter().copied().enumerate() {
                if dead(g, h) { continue; }
                let c = g.gd[h].ch;
                let f = crate::character::character_forward(g, c);
                let p = g.ch[c].pos;
                crate::squad::order(g, h, OP { aim_at: Some(Some([p[0] + f[0] * 1.2, 0.1, p[2] + f[2] * 1.2])), upper: Some(Some(Up::Aim)), stance: Some(Some(St::None)), ..Default::default() });
                if i == 0 {
                    let (p, y) = (g.ch[c].pos, g.ch[c].body_yaw);
                    crate::squad::order(g, h, OP { goal: Some(Some([p[0] + 0.35, 0.0, p[2] - 0.3])), strafe: Some(Some(true)), face: Some(Some(y)), speed: Some(Some(0.9)), ..Default::default() });
                }
            }
        }
        breach_drill_muzzles_on_door(g);
        dr!(g).phase = Phase::Enter;
    }
}

/// Live: what a man down means; true when the drill has ended.   (breach.ts BreachDrill.casualties)
pub fn breach_drill_casualties(g: &mut crate::game::Game, t: f64) -> bool {
    if dr!(g).phase == Phase::Done { return false; }
    let (a, b) = (dr!(g).a, dr!(g).b);
    let (a_down, b_down) = (dead(g, a), dead(g, b));
    if !a_down && !b_down { return false; }
    if dead(g, a) && breach_drill_once(g, "opdown:a") { breach_drill_beat(g, "opdown:1"); }
    if dead(g, b) && breach_drill_once(g, "opdown:b") { breach_drill_beat(g, "opdown:2"); }
    let why = if a_down && b_down { Some("wiped") } else if a_down && dr!(g).t_flung.is_nan() { Some("abort") } else { None };
    if let Some(why) = why {
        dr!(g).phase = Phase::Done;
        dr!(g).t_done = t;
        breach_drill_beat(g, why);
        if let Some(can) = dr!(g).can {
            if !crate::throwables::item_fired(g, can) {
                let fuse = m::min(crate::throwables::item_fuse(g, can), FUSE_AFTER_LAND);
                crate::throwables::item_set_fuse(g, can, fuse);
                breach_drill_beat(g, "can:short-fuse");
            }
        }
        return true;
    }
    false
}

/// Heads back on the door.   (breach.ts BreachDrill.muzzlesOnDoor)
pub fn breach_drill_muzzles_on_door(g: &mut crate::game::Game) {
    let pa = breach_drill_inside(g, 1.5, 1.1, None);
    let pb = breach_drill_inside(g, 1.0, 1.5, Some(0.3));
    let (a, b) = (dr!(g).a, dr!(g).b);
    set_aim(g, 0, pa);
    if !dead(g, a) { crate::squad::order(g, a, OP { aim_at: Some(Some(pa)), ..Default::default() }); }
    set_aim(g, 1, pb);
    if !dead(g, b) { crate::squad::order(g, b, OP { aim_at: Some(Some(pb)), ..Default::default() }); }
}

/// A point `d` metres through the doorway at height `y`, `s` along the wall.   (breach.ts BreachDrill.inside)
pub fn breach_drill_inside(g: &mut crate::game::Game, d: f64, y: f64, s: Option<f64>) -> V3 {
    let s = s.unwrap_or(0.0);
    let door = breach_drill_door(g).unwrap();
    let fc = g.dr[door].frame_centre;
    let n = dr!(g).plan.inward.unwrap_or([1.0, 0.0]);
    [fc[0] + n[0] * d + n[1] * s, y, fc[2] + n[1] * d - n[0] * s]
}

/// Waypoints to a mark across the room, by the plan's `next.via` lane if the straight leg is not clear.   (breach.ts BreachDrill.legTo)
pub fn breach_drill_leg_to(g: &mut crate::game::Game, gd: GuardId, mark: V3) -> Vec<V3> {
    let c = g.gd[gd].ch;
    let from = g.ch[c].pos;
    if clear_leg(g, from, mark) { return vec![mark]; }
    let mut out: Vec<V3> = Vec::new();
    let mut p = from;
    let via = dr!(g).plan.next.via.clone().unwrap_or_default();
    for q in via {
        if clear_leg(g, p, mark) { break; }
        if flat(p, q) > 0.6 && (!out.is_empty() || flat(from, q) < flat(from, mark)) { out.push(v3::copy(q)); p = q; }
    }
    out.push(mark);
    out
}

/// `p` inside the room's authored bounds by at least `m`.   (breach.ts BreachDrill.insideRoom)
pub fn breach_drill_inside_room(g: &mut crate::game::Game, p: V3, m: f64) -> bool {
    let b = match dr!(g).plan.bounds { Some(b) => b, None => return true };
    p[0] > b.x0 + m && p[0] < b.x1 - m && p[2] > b.z0 + m && p[2] < b.z1 - m
}

/// Man i's way through the room.   (breach.ts BreachDrill.flowOf)
pub fn breach_drill_flow_of(g: &mut crate::game::Game, i: f64) -> crate::scene_level::BreachFlow {
    dr!(g).plan.flow[i as usize].clone()
}

/// Send man i through the frame.   (breach.ts BreachDrill.startFlow)
pub fn breach_drill_start_flow(g: &mut crate::game::Game, i: f64) {
    let fl = breach_drill_flow_of(g, i);
    let i = i as usize;
    let gd = man(g, i);
    let t = dr!(g).t;
    let (centre, _) = targets(g, i);
    let f = &mut dr!(g).flows[i];
    f.st = FlowSt::Into;
    f.t0 = t;
    f.q = centre;
    f.on = -1.0;
    f.n = 0.0;
    f.next = 0.0;
    let aim = f.aim;
    let n = fl.into.len();
    let face = v3::yaw_to(fl.into[n.saturating_sub(2)], fl.into[n - 1]);
    route_i(g, i, &fl.into, OP { speed: Some(Some(2.9)), face: Some(Some(face)), strafe: Some(Some(false)), stance: Some(Some(St::None)), upper: Some(Some(Up::Aim)), aim_at: Some(Some(aim)), hot: Some(Some(true)), crouch: Some(Some(false)), ..Default::default() });
    if live_of(g).map_or(false, |l| l.lights) { crate::squad::order(g, gd, OP { light: Some(Some(true)), ..Default::default() }); }
    crate::squad::sweep_start(&mut dr!(g).flows[i].sweep, aim, &[fl.centre[0]], t, 0.9);
}

/// One frame of man i's flow.   (breach.ts BreachDrill.tickFlow)
pub fn breach_drill_tick_flow(g: &mut crate::game::Game, i: f64, dt: f64) {
    let ix = i as usize;
    if dr!(g).flows[ix].st == FlowSt::Wait { return; }
    let fl = breach_drill_flow_of(g, i);
    let gd = man(g, ix);
    let t = dr!(g).t;
    if dead(g, gd) { return; }
    let c = g.gd[gd].ch;
    let arrived = adv(g, ix, Some(0.8));
    let engaged = breach_drill_staged_exchange(g, i, dt);
    if !engaged && dr!(g).flows[ix].sweep.active { sweep_tick(g, ix, t); }
    match dr!(g).flows[ix].st {
        FlowSt::Into => {
            let last = fl.into[fl.into.len() - 1];
            if flat(g.ch[c].pos, last) < 0.55 || t > dr!(g).flows[ix].t0 + 2.2 {
                let f = &mut dr!(g).flows[ix];
                f.st = FlowSt::Corner;
                f.in_at = t;
                f.corner_at = t;
                if dr!(g).t_corner.is_nan() { dr!(g).t_corner = t; }
                let n = ix + 1;
                breach_drill_beat(g, &alloc::format!("in{}", n));
                breach_drill_beat(g, &alloc::format!("centre{}", n));
                breach_drill_beat(g, &alloc::format!("corner{}", n));
                let old = dr!(g).flows[ix].q.clone();
                let (_, corner) = targets(g, ix);
                let mut q = Vec::new();
                for k in old.iter().copied() { if !hostile_dead(g, k) { q.push(k); } }
                for k in corner { if !hostile_dead(g, k) && !old.contains(&k) { q.push(k); } }
                let f = &mut dr!(g).flows[ix];
                f.q = q;
                if f.on < 0.0 { f.n = 0.0; f.next = m::min(f.next, t + 0.05); }
                let aim = f.aim;
                route_i(g, ix, &fl.corner, OP { speed: Some(Some(2.0)), face: Some(Some(fl.pod_face)), strafe: Some(Some(false)), stance: Some(Some(St::None)), upper: Some(Some(Up::Aim)), aim_at: Some(Some(aim)), hot: Some(Some(true)), ..Default::default() });
                let mut pts = vec![fl.centre[0], fl.centre[1]];
                pts.extend_from_slice(&fl.corner_aim);
                crate::squad::sweep_start(&mut dr!(g).flows[ix].sweep, aim, &pts, t, CENTRE_SECS + 1.9);
            }
        }
        FlowSt::Centre => {
            let pending = dr!(g).flows[ix].on >= 0.0 || breach_drill_pickable(g, i);
            let (active, in_at) = (dr!(g).flows[ix].sweep.active, dr!(g).flows[ix].in_at);
            if (!active && !pending && t > in_at + CENTRE_SECS) || t > in_at + 3.2 {
                let f = &mut dr!(g).flows[ix];
                f.st = FlowSt::Corner;
                f.corner_at = t;
                if dr!(g).t_corner.is_nan() { dr!(g).t_corner = t; }
                breach_drill_beat(g, &alloc::format!("corner{}", ix + 1));
                let old = dr!(g).flows[ix].q.clone();
                let (_, corner) = targets(g, ix);
                let mut q = Vec::new();
                for k in corner { if !hostile_dead(g, k) { q.push(k); } }
                for k in old { if !hostile_dead(g, k) { q.push(k); } }
                let f = &mut dr!(g).flows[ix];
                f.q = q;
                f.on = -1.0;
                f.n = 0.0;
                f.next = t + 0.1;
                let aim = f.aim;
                route_i(g, ix, &fl.corner, OP { speed: Some(Some(1.7)), face: Some(Some(fl.pod_face)), strafe: Some(Some(false)), stance: Some(Some(St::None)), upper: Some(Some(Up::Aim)), aim_at: Some(Some(aim)), hot: Some(Some(true)), ..Default::default() });
                crate::squad::sweep_start(&mut dr!(g).flows[ix].sweep, aim, &fl.corner_aim, t, 1.9);
            }
        }
        FlowSt::Corner => {
            let pending = dr!(g).flows[ix].on >= 0.0 || breach_drill_pickable(g, i);
            let (active, corner_at) = (dr!(g).flows[ix].sweep.active, dr!(g).flows[ix].corner_at);
            if ((arrived || t > corner_at + 4.0) && !active && !pending) || t > corner_at + 6.5 {
                dr!(g).flows[ix].st = FlowSt::Hold;
                dr!(g).flows[ix].hold_at = t;
                breach_drill_beat(g, &alloc::format!("hold{}", ix + 1));
                if ix == 0 { say!(g, gd, "clear1", breach_lines.clear1); } else { say!(g, gd, "clear2", breach_lines.clear2); }
                let aim = dr!(g).flows[ix].aim;
                crate::squad::sweep_start(&mut dr!(g).flows[ix].sweep, aim, &[fl.hold], t, 0.9);
                crate::squad::order(g, gd, OP { face: Some(Some(fl.pod_face)), ..Default::default() });
            }
        }
        FlowSt::Hold | FlowSt::Wait => {}
    }
}

/// The clear beat's exchange and resting muzzle.   (breach.ts BreachDrill.tickHold)
pub fn breach_drill_tick_hold(g: &mut crate::game::Game, i: f64, dt: f64) {
    let ix = i as usize;
    if !breach_drill_staged_exchange(g, i, dt) && dr!(g).flows[ix].sweep.active { let t = dr!(g).t; sweep_tick(g, ix, t); }
}

/// Both holding and every hostile down, a beat after; or, the show only, 14 s of entry.   (breach.ts BreachDrill.roomClear)
pub fn breach_drill_room_clear(g: &mut crate::game::Game, te: f64) -> bool {
    let live = live_of(g).is_some();
    let own = live && crate::roomfight::room_fight_on(g);
    let hs = dr!(g).hostiles.clone();
    let mut all_down = true;
    for h in hs.iter().copied() {
        let gone = dead(g, h) || crate::runner::runner_fled(g, h) || (own && { let p = g.ch[g.gd[h].ch].pos; !breach_drill_inside_room(g, p, -0.3) });
        if !gone { all_down = false; break; }
    }
    let (a, b) = (dr!(g).a, dr!(g).b);
    let (st0, st1) = (dr!(g).flows[0].st, dr!(g).flows[1].st);
    let holding = if live { (dead(g, a) || st0 == FlowSt::Hold) && (dead(g, b) || st1 == FlowSt::Hold) } else { st0 == FlowSt::Hold && st1 == FlowSt::Hold };
    if holding && all_down {
        let t = dr!(g).t;
        if dr!(g).t_all_done.is_nan() { dr!(g).t_all_done = t; }
        return t > dr!(g).t_all_done + 0.5;
    }
    if live { return false; }
    if te > 14.0 {
        for (k, h) in hs.iter().copied().enumerate() {
            if !dead(g, h) && dr!(g).guarantee_downs && !crate::runner::runner_fled(g, h) {
                breach_drill_beat(g, &alloc::format!("guaranteed:{}", k));
                crate::combat::kill_guard(g, h, [1.0, 0.0, 0.0], Some(false), Some(crate::game::DownKind::Shot), Some(crate::game::DownBy::Guard), Some("hostile down"), None);
            }
        }
        return true;
    }
    false
}

/// Is the front of man i's list a living man he has a line to.   (breach.ts BreachDrill.pickable)
pub fn breach_drill_pickable(g: &mut crate::game::Game, i: f64) -> bool {
    let ix = i as usize;
    let op = man(g, ix);
    let live = live_of(g).is_some();
    if live && crate::roomfight::room_fight_on(g) { return crate::roomfight::room_fight_contact(g, op); }
    let c = g.gd[op].ch;
    while let Some(k) = dr!(g).flows[ix].q.first().copied() {
        let gone = match host(g, k) { Some(h) => dead(g, h) || crate::runner::runner_fled(g, h), None => false };
        if !gone { break; }
        dr!(g).flows[ix].q.remove(0);
    }
    let eye = bone_or(g, c, crate::character::Bone::Head, 1.6);
    if live {
        let q = dr!(g).flows[ix].q.clone();
        let n = dr!(g).hostiles.len();
        let mut order_ = q.clone();
        for j in 0..n { let j = j as f64; if !q.contains(&j) { order_.push(j); } }
        let mut found = None;
        for k in order_ {
            let hk = match host(g, k) { Some(h) => h, None => continue };
            if dead(g, hk) || crate::runner::runner_fled(g, hk) { continue; }
            let ch = g.gd[hk].ch;
            let chest = bone_or(g, ch, crate::character::Bone::Chest, 1.1);
            let energy = crate::glasspen::shot_energy(g, c, None);
            if !crate::scene_collision::shot_blocked(g, eye, chest, Some(0.02), energy) { found = Some(k); break; }
        }
        let k = match found { Some(k) => k, None => return false };
        let mut nq = vec![k];
        nq.extend(q.iter().copied().filter(|j| *j != k));
        dr!(g).flows[ix].q = nq;
        return true;
    }
    let k = match dr!(g).flows[ix].q.first().copied() { Some(k) => k, None => return false };
    let h = host(g, k).unwrap();
    let ch = g.gd[h].ch;
    let chest = bone_or(g, ch, crate::character::Bone::Chest, 1.1);
    !crate::scene_collision::segment_blocked(g, eye, chest, None, None, None)
}

/// The exchange as staged: the muzzle on a man, the settle, the controlled pair; true while on somebody.   (breach.ts BreachDrill.stagedExchange)
pub fn breach_drill_staged_exchange(g: &mut crate::game::Game, i: f64, dt: f64) -> bool {
    let ix = i as usize;
    let op = man(g, ix);
    let c = g.gd[op].ch;
    let t = dr!(g).t;
    let live = live_of(g).is_some();
    if live && crate::roomfight::room_fight_on(g) { return crate::roomfight::room_fight_engaged(g, op); }
    if dr!(g).flows[ix].st == FlowSt::Wait || dead(g, op) { return false; }
    if dr!(g).flows[ix].st == FlowSt::Into { let p = g.ch[c].pos; if !breach_drill_inside_room(g, p, FIRE_INSIDE_M) { return false; } }
    let on = dr!(g).flows[ix].on;
    if on >= 0.0 && dr!(g).flows[ix].n == 0.0 {
        let h = host(g, on).unwrap();
        if dead(g, h) || crate::runner::runner_fled(g, h) { let f = &mut dr!(g).flows[ix]; f.on = -1.0; f.next = t + SHIFT; }
    }
    if dr!(g).flows[ix].on < 0.0 && t >= dr!(g).flows[ix].next && breach_drill_pickable(g, i) {
        let f = &mut dr!(g).flows[ix];
        f.on = f.q[0];
        f.since = t;
        f.n = 0.0;
        let on = f.on;
        breach_drill_beat(g, &alloc::format!("eyes{}:{}", ix + 1, on as i64));
    }
    let on = dr!(g).flows[ix].on;
    if on < 0.0 { return false; }
    let h = host(g, on).unwrap();
    let ch = g.gd[h].ch;
    let chest = bone_or(g, ch, crate::character::Bone::Chest, 1.1);
    let aim = dr!(g).flows[ix].aim;
    set_aim(g, ix, [v3::damp(aim[0], chest[0], 16.0, dt), v3::damp(aim[1], chest[1], 16.0, dt), v3::damp(aim[2], chest[2], 16.0, dt)]);
    let due = if dr!(g).flows[ix].n == 0.0 { dr!(g).flows[ix].since + SETTLE } else { dr!(g).flows[ix].next };
    let fa = g.gd[op].weapon;
    if live && t >= due {
        let head = bone_or(g, c, crate::character::Bone::Head, 1.6);
        let def = g.m.weapons.weapons.get(g.fa[fa].def).cloned();
        let energy = crate::glasspen::shot_energy(g, c, def.as_ref());
        if crate::scene_collision::shot_blocked(g, head, chest, Some(0.02), energy) {
            let f = &mut dr!(g).flows[ix];
            f.on = -1.0;
            f.next = t + 0.1;
            return false;
        }
    }
    if t >= due && !crate::weapons::firearm_reloading(g, fa) {
        if crate::weapons::firearm_fire(g, fa) {
            let dir = v3::normalize(v3::sub(chest, g.ch[c].muzzle));
            crate::combat::fire_weapon(g, c, chest, false, Some(&crate::combat::FireOpts { steady: Some(true), spread: Some(0.01), ..Default::default() }));
            dr!(g).flows[ix].n += 1.0;
            dr!(g).rounds[ix] += 1.0;
            breach_drill_beat(g, &alloc::format!("shot{}", ix + 1));
            if dr!(g).flows[ix].n >= 2.0 {
                if !dead(g, h) && dr!(g).guarantee_downs && !crate::runner::runner_fled(g, h) {
                    breach_drill_beat(g, &alloc::format!("guaranteed:{}", on as i64));
                    crate::combat::kill_guard(g, h, dir, Some(false), Some(crate::game::DownKind::Shot), Some(crate::game::DownBy::Guard), Some("hostile down"), None);
                }
                let f = &mut dr!(g).flows[ix];
                f.q.retain(|k| *k != on);
                f.on = -1.0;
                f.next = t + SHIFT;
            } else {
                dr!(g).flows[ix].next = t + PAIR;
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

    /// bits from breach.ts's own flat, under bun with EXACT on
    #[test]
    fn flat_bits() {
        assert_eq!(flat([1.3, 0.0, -2.7], [4.1, 9.0, 0.37]).to_bits(), 0x40109ed3ec0bed7b);
        assert_eq!(flat([0.1, 0.0, 0.2], [0.3, 0.0, 0.7]).to_bits(), 0x3fe13b86ea20ae5c);
    }

    #[test]
    fn live_lines_by_charge_and_key() {
        assert_eq!(live_text(false, "stack2", "show"), "set.");
        assert_eq!(live_text(true, "stack2", "show"), "show");
        assert_eq!(live_text(true, "clear1", "show"), "right side — clear. i've got the door.");
        assert_eq!(live_text(true, "clear2", "show"), "left clear.");
        assert_eq!(live_text(true, "roomClear1", "show"), "room clear.");
    }
}
