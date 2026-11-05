//! src/game/qrf.ts lines 877 to 1202
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- seams: what skeleton 3 does not have yet
/// `g.audio.play(name, at, vol, { rate })`: `opts` is a map of crate::Opaque, which holds no number
fn seam_audio_play_rate(g: &mut Game, name: &str, at: V3, vol: f64, rate: f64) { todo!("needs: a rate in game_audio_like_play's opts") }
/// `(op as DoorOp & { ended?: string }).ended`: DoorTools' end() writes it on the op; the row has no such field, and a DoorToolsDoneItem does not say which op it was
fn seam_op_ended(g: &mut Game, op: DoorOpId) -> Option<String> { todo!("needs: DoorOp.ended on g.tab.doortools_door_op") }
/// `try { spawnUnit(g, o) } catch (e)`: spawn_unit throws on a full budget and on no light or id left; it returns a GuardId and cannot say so
fn seam_try_spawn_unit(g: &mut Game, o: &crate::guards::UnitOpts) -> Result<GuardId, String> { todo!("needs: spawn_unit returning a Result") }

// ---------------------------------------------------------------- small reads
fn man_pos(g: &Game, m: GuardId) -> V3 { g.ch[g.gd[m].ch].pos }
fn man_yaw(g: &Game, m: GuardId) -> f64 { g.ch[g.gd[m].ch].body_yaw }
fn man_arrived(g: &Game, m: GuardId) -> bool { g.gd[m].script.as_ref().map_or(false, |s| s.arrived == Some(true)) }
fn player_pos(g: &Game) -> V3 { g.ch[g.pl[g.player].ch].pos }
fn say(g: &mut Game, m: GuardId, text: &str) { crate::game::game_say(g, m, text, None, None, None) }
/// `Q.target!.plan!`
fn target_plan(g: &Game) -> crate::scene_level::RoomClearPlan { g.qrf.target.as_ref().unwrap().plan.clone().unwrap() }
/// `Q.target!.room`
fn target_room(g: &Game) -> String { g.qrf.target.as_ref().unwrap().room.clone() }
/// `g.audio.play(name, at, vol)`
fn audio(g: &mut Game, name: &str, at: Option<V3>, vol: f64) { crate::audio_iface::game_audio_like_play(g, name, at, Some(vol), None) }
/// `C.tools.x` / `C.orders.x` read: fields of the pair are read on the held value
fn cast(g: &Game) -> &crate::squadcast::SquadCast { g.qrf.cast.as_ref().unwrap() }
/// `C.tools.m()` / `C.orders.m()`: a method of the pair runs with the cast on Game; the holder takes it out for the call and puts it back
fn in_cast<R>(g: &mut Game, mut f: impl FnMut(&mut Game) -> R) -> R {
    let mut c = g.qrf.cast.take().unwrap();
    let r = crate::squadcast::with_cast(g, &mut c, &mut f);
    g.qrf.cast = Some(c);
    r
}
/// `g.audio.play('radio', man.char.bones.head ?? man.char.pos, vol)`'s place
fn head_or_pos(g: &Game, m: GuardId) -> V3 { let c = g.gd[m].ch; g.ch[c].bones.get(crate::character::Bone::Head).copied().unwrap_or(g.ch[c].pos) }

/// The point on the trail `s` metres back from the head, walking the crumbs backwards; the oldest crumb if the trail is shorter. (qrf.ts trailPoint)
pub fn trail_point(trail: &[V3], head: V3, s: f64) -> V3 {
    let mut prev = head;
    let mut left = s;
    let mut i = trail.len();
    while i > 0 {
        i -= 1;
        let q = trail[i];
        let d = flat(prev, q);
        if d >= left {
            let t = left / m::max(d, 1e-6);
            return [prev[0] + (q[0] - prev[0]) * t, 0.0, prev[2] + (q[2] - prev[2]) * t];
        }
        left -= d;
        prev = q;
    }
    v3::copy(prev)
}

/// The trail's forward direction at that point (unit, flat), or None if the trail has no length yet. (qrf.ts trailDir)
pub fn trail_dir(trail: &[V3], head: V3, s: f64) -> Option<V3> {
    let a = trail_point(trail, head, s + 0.5);
    let b = trail_point(trail, head, m::max(0.0, s - 0.5));
    let d = flat(a, b);
    if d < 0.05 { return None; }
    Some([(b[0] - a[0]) / d, 0.0, (b[2] - a[2]) / d])
}

/// Hold all round where they stand: each man squares off to a quarter, guns up, for a beat. (qrf.ts hold360)
pub fn hold360(g: &mut crate::game::Game, el: &[GuardId]) {
    g.qrf.phase = Phase::Hold;
    g.qrf.phase_t = 0.0;
    let f = man_yaw(g, el[0]);
    for (k, &man) in el.iter().enumerate() {
        let yaw = f + [0.0, -m::PI / 2.0, m::PI / 2.0, m::PI][k % 4];
        let aim = sector_point(man_pos(g, man), yaw, None, None);
        crate::squad::order(g, man, crate::squad::OrderPatch {
            goal: Some(None), face: Some(Some(yaw)), strafe: Some(Some(false)), stance: Some(Some(crate::character::Stance::None)), upper: Some(Some(crate::character::UpperMode::Aim)),
            crouch: Some(Some(k == 3)), aim_at: Some(Some(aim)), ..Default::default()
        });
    }
    note(g, "hold");
}

/// Where n men stack on a plan's door: its two marks, then on down the line, on free floor. (qrf.ts stackMarksFor)
pub fn stack_marks_for(g: &mut crate::game::Game, p: &crate::scene_level::RoomClearPlan, n: f64) -> Vec<V3> {
    let mut out: Vec<V3> = vec![v3::copy(p.stack[0]), v3::copy(p.stack[1])];
    let u = v3::normalize([p.stack[1][0] - p.stack[0][0], 0.0, p.stack[1][2] - p.stack[0][2]]);
    let mut k = 2.0;
    while k < n {
        let mut q: Option<V3> = None;
        for s in [0.85 * (k - 1.0), 0.85 * (k - 1.0) + 0.4, 0.85 * (k - 1.0) - 0.3] {
            let c: V3 = [p.stack[1][0] + u[0] * s, 0.0, p.stack[1][2] + u[2] * s];
            if !crate::scene_collision::nav_grid_is_blocked(g, c[0], c[2]) { q = Some(c); break; }
        }
        let fill = q.unwrap_or_else(|| v3::copy(out[out.len() - 1]));
        out.push(fill);
        k += 1.0;
    }
    out.truncate(n as usize);
    out
}

/// The element walks onto the stack marks of the target's door. (qrf.ts startStack)
pub fn start_stack(g: &mut crate::game::Game, el: &[GuardId]) {
    let p = target_plan(g);
    g.qrf.phase = Phase::Stack;
    g.qrf.phase_t = 0.0;
    g.qrf.stack_marks = stack_marks_for(g, &p, el.len() as f64);
    g.qrf.shoved_at = f64::NAN;
    g.qrf.can = None;
    for (i, &man) in el.iter().enumerate() {
        let mark = g.qrf.stack_marks[i];
        let row = if i < 1 { i } else { 1 };
        route(g, man, &[mark], Some(RoutePatch {
            speed: Some(1.7 - i as f64 * 0.08), face: Some(p.stack_face[row]),
            stance: Some(if i == 0 { crate::character::Stance::Stack } else { crate::character::Stance::HighReady }),
            stack_side: Some(p.stack_side), upper: Some(crate::character::UpperMode::None), aim_at: Some(v3::copy(p.stack_aim[row])),
            crouch: Some(false), strafe: Some(false), ..Default::default()
        }));
    }
    let room = target_room(g);
    let said = door_said(g, &p.door);
    radio(g, Some(el[0]), &format!("stack on the {} — {}", said, room));
    note(g, &format!("stack:{}", room));
}

/// The stack forms; planted (or out of time) the point man steps to the lock. (qrf.ts phaseStack)
pub fn phase_stack(g: &mut crate::game::Game, el: &mut Vec<GuardId>) {
    for &man in el.iter() { advance(g, man); }
    let planted = el.iter().filter(|&&man| man_arrived(g, man)).count() as f64;
    if (planted >= m::min(el.len() as f64, 3.0) && g.qrf.phase_t > 1.2) || g.qrf.phase_t > STACK_CAP {
        if let Some(&two) = el.get(1) { let c = g.gd[two].ch; crate::character::anim_signal(g, c, crate::character::SignalKind::Go, None); }
        g.qrf.phase = Phase::Breach;
        g.qrf.phase_t = 0.0;
        let p = target_plan(g);
        let d = crate::doors::doors_by_name(g, &p.door);
        if let Some(d) = d {
            if crate::doors::door_is_closed(g, d) {
                route(g, el[0], &[p.kick_from], Some(RoutePatch {
                    speed: Some(1.3), face: Some(p.kick_face), stance: Some(crate::character::Stance::HighReady),
                    upper: Some(crate::character::UpperMode::None), ..Default::default()
                }));
                // (`aimAt: null`: all order does with it is the assign; RoutePatch cannot say null)
                if let Some(s) = g.gd[el[0]].script.as_mut() { s.aim_at = None; }
            }
        }
        note(g, "squeeze");
    }
}

/// The shove of the leaf, then the can chopped in, or going dry. (qrf.ts phaseBreach)
pub fn phase_breach(g: &mut crate::game::Game, el: &mut Vec<GuardId>) {
    let p = target_plan(g);
    let d = crate::doors::doors_by_name(g, &p.door);
    let lead = el[0];
    advance(g, lead);
    let s = js::or(m::sign(p.fling), -1.0);
    if g.qrf.shoved_at.is_nan() {
        // step one: the point man planted square to the leaf by the lock (or out of time) works the lever and throws it into the room
        let shut = match d { Some(d) => crate::doors::door_is_closed(g, d), None => false };
        let planted = !shut || (man_arrived(g, lead) && flat(man_pos(g, lead), p.kick_from) < 0.15 && m::abs(v3::wrap_angle(man_yaw(g, lead) - p.kick_face)) < 0.15);
        if !((planted && g.qrf.phase_t > 0.45) || g.qrf.phase_t > 1.8) { return; }
        if let Some(d) = d {
            if !crate::doors::door_off(g, d) && crate::doors::doors_open_for(g, d, s) < 1.0 {
                crate::doors::door_set_latched(g, d, false);
                crate::doors::door_set_closing(g, d, false);
                let v = s * m::max(m::abs(crate::doors::door_vel(g, d)), m::abs(p.fling));
                crate::doors::door_set_vel(g, d, v);
                crate::doors::door_set_quiet_t(g, d, -1.0);
                crate::doors::door_set_creak_armed(g, d, false);
                let who = g.ch[g.gd[lead].ch].id;
                crate::doors::door_set_last_who(g, d, who);
                let at = crate::doors::door_pos(g, d);
                seam_audio_play_rate(g, "doorBash", at, 0.6, 1.1);
                say(g, lead, "door!");
            }
        }
        g.qrf.shoved_at = g.time;
        note(g, "shove");
        return;
    }
    // step two: once the leaf is round (or 0.8 s of it), the charge, if the pool has one, chopped in from where he stands square to the doorway
    if let Some(d) = d {
        if crate::doors::door_angle(g, d) * s < 0.9 && g.time - g.qrf.shoved_at < 0.8 { return; }
    }
    g.qrf.shoved_at = f64::NAN;
    match d {
        Some(d) if g.qrf.bangs > 0.0 => {
            let fc = g.dr[d].frame_centre;
            let b = p.bounds;
            let closed_dir = g.dr[d].def.closed_dir;
            let mut n: V3 = [m::cos(closed_dir + m::PI / 2.0), 0.0, m::sin(closed_dir + m::PI / 2.0)];   // square off the closed leaf's line
            let probe: V3 = [fc[0] + n[0], 0.0, fc[2] + n[2]];
            let in_room = if b.is_some() {
                crate::squad::in_bounds(probe, b, None)
            } else {
                let end = p.entry[0][p.entry[0].len() - 1];
                flat(probe, end) < flat([fc[0] - n[0], 0.0, fc[2] - n[2]], end)
            };
            if !in_room { n = [-n[0], 0.0, -n[2]]; }
            let depth = BANG_DEPTH;
            let land: V3 = [fc[0] + n[0] * depth, 0.05, fc[2] + n[2] * depth];
            let lead_c = g.gd[lead].ch;
            let hand = match crate::character::character_joint_under(g, lead_c, "handL") { Some(h) => h, None => v3::add(g.ch[lead_c].pos, [0.0, 1.3, 0.0]) };
            let from: V3 = [hand[0], m::max(0.9, hand[1]), hand[2]];
            let sol = crate::throwables::throwables_solve(g, from, land, Some(9.0));
            let who = g.ch[lead_c].id;
            let it = crate::throwables::throwables_spawn(g, crate::throwables::ItemKind::Flash, from, sol.v0, Some(crate::throwables::ThrowablesSpawnOpts {
                fuse: Some(99.0), who: Some(who), heading: Some(v3::yaw_of(sol.v0)), ..Default::default()
            }));   // (fuse 99: it goes on OUR count)
            g.qrf.can = Some(it);
            g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::Can);
            audio(g, "throw", Some(from), 0.7);
            crate::character::anim_toss_item(g, lead_c, Some(crate::character::TOSS_QUICK));
            g.qrf.bangs -= 1.0;
            g.qrf.can_land_t = f64::NAN;
            say(g, lead, "bang out!");
            note(g, "bang-out");
            // everybody's head off the doorway while it cooks
            for (i, &man) in el.iter().enumerate() {
                let pos = man_pos(g, man);
                let i = i as f64;
                let away: V3 = [pos[0] - n[0] * 3.0 + (i - 1.5) * n[2], 0.6, pos[2] - n[2] * 3.0 - (i - 1.5) * n[0]];
                crate::squad::order(g, man, crate::squad::OrderPatch { aim_at: Some(Some(away)), ..Default::default() });
            }
            g.qrf.phase = Phase::Bang;
            g.qrf.phase_t = 0.0;
        }
        _ => {
            say(g, lead, "no bang — going dry. go!");
            start_flow(g, el);
        }
    }
}

/// A can of the element's that went off this tick: the floor-wide 'bang' report is re-filed as THEIRS, a slam's worth of noise carrying their man's id. (qrf.ts theirBang)
pub fn their_bang(g: &mut crate::game::Game, at: V3, who: GuardId) {
    let mut i = g.events.len();
    while i > 0 {
        i -= 1;
        let e = g.events[i];
        if e.kind == crate::director::EventKind::Bang && e.time == g.time && flat(e.pos, at) < 1.0 { g.events.remove(i); break; }
    }
    let id = g.ch[g.gd[who].ch].id;
    let (time, serial) = (g.time, g.serial());
    g.events.push(crate::game::GameEvent {
        kind: crate::director::EventKind::Door, pos: [at[0], 0.0, at[2]], time, loud: true, level: Some(1.0), who: Some(id), serial, ..Default::default()
    });
    note(g, "bang");
}

/// The can cooks: on their own count it goes off, with the element out of its view, and they flow. (qrf.ts phaseBang)
pub fn phase_bang(g: &mut crate::game::Game, el: &[GuardId]) {
    let it = match g.qrf.can { Some(it) => it, None => { start_flow(g, el); return; } };
    if g.qrf.can_land_t.is_nan() && (crate::throwables::item_bounces(g, it) > 0.0 || crate::throwables::item_settled(g, it)) { g.qrf.can_land_t = g.time; }
    if (!g.qrf.can_land_t.is_nan() && g.time - g.qrf.can_land_t >= BANG_FUSE) || g.qrf.phase_t > 2.4 {
        crate::throwables::item_set_fired(g, it, true);
        // detonateStun's daze falls on everybody but the element; what it writes to the list of men is put back with the list
        let all = g.guards.clone();
        g.guards = all.iter().copied().filter(|x| !g.qrf.men.contains(x)).collect();
        crate::combat::detonate_stun(g, it, None, None);
        g.guards = all;
        let at = crate::throwables::item_pos(g, it);
        their_bang(g, at, el[0]);
        start_flow(g, el);
    }
}

/// All of them in, a beat apart. (qrf.ts startFlow)
pub fn start_flow(g: &mut crate::game::Game, el: &[GuardId]) {
    g.qrf.phase = Phase::Flow;
    g.qrf.phase_t = 0.0;
    g.qrf.flowed = 0.0;
    g.qrf.can = None;
    say(g, el[0], "go! go! go!");
    note(g, "flow");
}

/// Man k's way in: one and two the plan's legs, three up the point man's wall to the deep point, four onto the door side. (qrf.ts flowLegs)
pub fn flow_legs(p: &crate::scene_level::RoomClearPlan, k: f64) -> crate::qrf::FlowLegs {
    fn last(a: &[V3]) -> V3 { a[a.len() - 1] }
    if k == 0.0 { return FlowLegs { pts: p.entry[0].clone(), face: p.pod_face[0], aim: v3::copy(last(&p.entry_aim[0])) }; }
    if k == 1.0 { return FlowLegs { pts: p.entry[1].clone(), face: p.pod_face[1], aim: v3::copy(last(&p.entry_aim[1])) }; }
    if k == 2.0 { return FlowLegs { pts: vec![p.entry[0][0], p.search_path[0]], face: p.search_face, aim: v3::copy(last(&p.search_aim_walk)) }; }
    // (`formUp[1] ?? formUp[0]`, `formFace[1] ?? formFace[0]`: both are pairs, the level's check says so: index 1 is always there)
    FlowLegs { pts: vec![p.entry[1][0], last(&p.form_up[1])], face: p.form_face[1], aim: v3::copy(p.stack_aim[0]) }
}

/// They go in on the beat; in (or out of time) the room is called. (qrf.ts phaseFlow)
pub fn phase_flow(g: &mut crate::game::Game, el: &mut Vec<GuardId>) {
    let p = target_plan(g);
    let due = m::min(el.len() as f64, m::floor(g.qrf.phase_t / FLOW_GAP) + 1.0);
    while g.qrf.flowed < due {
        let k = g.qrf.flowed;
        let man = match el.get(k as usize) { Some(&man) => man, None => break };
        let l = flow_legs(&p, k);
        route(g, man, &l.pts, Some(RoutePatch {
            speed: Some(if k < 2.0 { 2.4 } else { 2.1 }), face: Some(l.face), stance: Some(crate::character::Stance::None),
            upper: Some(crate::character::UpperMode::Aim), aim_at: Some(l.aim), crouch: Some(false), strafe: Some(false), ..Default::default()
        }));
        g.qrf.flowed += 1.0;
    }
    for &man in el.iter() { advance(g, man); }
    let in_count = el.iter().filter(|&&man| man_arrived(g, man)).count();
    if g.qrf.phase_t > 1.6 && g.qrf.flowed >= 2.0 {
        let what = format!("sides:{}", target_room(g));
        let said = g.qrf.log.iter().any(|l| l.what == what);
        if !said && man_arrived(g, el[0]) {
            let side1: &str = if p.lines.side1.is_empty() { "my side clear." } else { &p.lines.side1 };
            say(g, el[0], side1);
            if let Some(&two) = el.get(1) {
                let side2: &str = if p.lines.side2.is_empty() { "door side clear." } else { &p.lines.side2 };
                say(g, two, side2);
            }
            note(g, &what);
        }
    }
    if (in_count >= el.len() && g.qrf.phase_t > 2.5) || g.qrf.phase_t > FLOW_CAP {
        g.qrf.phase = Phase::Called;
        g.qrf.phase_t = 0.0;
        let room = target_room(g);
        let t = g.time;
        g.qrf.cleared.set(room.clone(), t);
        g.cleared_at.set(room.clone(), t);   // (the shift's pair reads the same ledger)
        if let Some(&three) = el.get(2) { say(g, three, "deep — clear."); }
        if let Some(&four) = el.get(3) { say(g, four, "door — clear."); }
        let rest = if g.qrf.bangs > 0.0 { "moving on" } else { "no bangs left, moving on" };
        radio(g, Some(el[0]), &format!("{} clear — negative contact. {}", room, rest));
        note(g, &format!("clear:{}", room));
    }
}

/// At the door: the element handed to the squad's door tools as a second cast. False (and nothing begun) when the tools refuse it. (qrf.ts startDrill)
pub fn start_drill(g: &mut crate::game::Game, el: &[GuardId]) -> bool {
    let t = g.qrf.target.clone().unwrap();
    let p = t.plan.clone().unwrap();
    let d = match crate::doors::doors_by_name(g, &p.door) { Some(d) => d, None => return false };
    if g.qrf.cast.is_none() {
        // (`mine: gd => Q.men.includes(gd)`: squad_orders_mine answers it from g.qrf.men, the one cast there is)
        let team = match side_of_team(g) { SideOfTeam::Guard => crate::teams::Team::Guard, SideOfTeam::Hostile => crate::teams::Team::Hostile };
        let of = crate::orders::OtherCast { team, ring: crate::squadcast::every_key() };
        let c = crate::squadcast::squad_cast(g, &of);
        g.qrf.cast = Some(c);
    }
    in_cast(g, |g| crate::doortools::door_tools_sync(g));   // (sync first: a roster that has changed starts the tools afresh, counts and all)
    let bangs = g.qrf.bangs;
    let tools = &mut g.qrf.cast.as_mut().unwrap().tools;
    tools.bangs = bangs;
    tools.charges = 0.0;
    tools.frags = 0.0;
    tools.lights = Some(true);
    let inside = match g.qrf.fix { Some(fix) => p.bounds.is_some() && crate::squad::in_bounds(fix, p.bounds, None), None => false };
    let kind = if !inside { crate::doortools::DoorToolsOrderKind::Threshold } else if g.qrf.bangs > 0.0 { crate::doortools::DoorToolsOrderKind::BangClear } else { crate::doortools::DoorToolsOrderKind::OpenClear };
    let side = crate::orders::side_of(g, d, p.stack[0]);
    let op = in_cast(g, |g| crate::doortools::door_tools_order(g, kind, el, d, side, None, None));
    let op = match op {
        crate::doortools::DoorToolsOrder::Text(why) => {
            g.qrf.bangs = cast(g).tools.bangs;
            note(g, &format!("drill-no:{}", why));
            return false;
        }
        crate::doortools::DoorToolsOrder::DoorOp(op) => op,
    };
    g.qrf.op = Some(op);
    g.qrf.op_beat = 0.0;
    g.qrf.drills += 1.0;
    g.qrf.phase = Phase::Drill;
    g.qrf.phase_t = 0.0;
    g.qrf.can = None;
    g.qrf.can_land_t = f64::NAN;
    let said = door_said(g, &p.door);
    radio(g, Some(el[0]), &format!("stack on the {} — {}", said, t.room));
    note(g, &format!("stack:{}", t.room));
    note(g, &format!("drill:{}", kind.as_str()));
    true
}

/// The entry let go of: every man off the cast's orders where he stands, the tools emptied, a can not yet thrown back in the element's pool. (qrf.ts dropDrill)
pub fn drop_drill(g: &mut crate::game::Game) {
    let op = g.qrf.op;
    g.qrf.op = None;
    if g.qrf.cast.is_none() { return; }
    if let Some(op) = op {
        let bangs = cast(g).tools.bangs;
        if cast(g).tools.ops.contains(&op) {
            g.qrf.bangs = bangs + (if g.tab.doortools_door_op[op].bang_held { 1.0 } else { 0.0 });   // let go of while it ran: the can in a hand comes back
        } else if seam_op_ended(g, op).is_some() {
            g.qrf.bangs = bangs;   // ended by the tools: their count is the count
        }
        // (neither: the tools started afresh under it: the element's own count, as last read, stands)
    }
    in_cast(g, |g| {
        crate::orders::squad_orders_clear(g, None);
        crate::doortools::door_tools_reset(g);
    });
    let tools = &mut g.qrf.cast.as_mut().unwrap().tools;
    tools.charges = 0.0;
    tools.frags = 0.0;
}

/// Once a frame while the entry runs: the cast's two ticks, its beats into the log, its can made theirs, and its end read. (qrf.ts phaseDrill)
pub fn phase_drill(g: &mut crate::game::Game, el: &mut Vec<GuardId>) {
    let (op, t) = match (g.qrf.cast.is_some(), g.qrf.op, g.qrf.target.clone()) {
        (true, Some(op), Some(t)) => (op, t),
        _ => { drop_drill(g); hold360(g, el); return; }
    };
    // a fresher fix somewhere else while nobody is through the door yet: the entry is let go of and they go to it
    if g.tab.doortools_door_op[op].crossed.len() == 0 && g.time - g.qrf.target_at > 3.0 {
        let bounds = t.plan.as_ref().and_then(|p| p.bounds);
        let here = |p: V3| bounds.is_some() && crate::squad::in_bounds(p, bounds, None);
        let bf = fix_of(g);
        let fix = g.qrf.fix;
        let net = match bf { Some(bf) => bf.t > g.qrf.fix_t + 0.5 && flat(bf.pos, fix.unwrap_or(bf.pos)) > RETARGET_M && !here(bf.pos), None => false };
        let own = !net && match fix { Some(fix) => g.qrf.fix_t > g.qrf.target_at && g.qrf.cue_rush && flat(fix, t.pos) > RETARGET_M && !here(fix), None => false };
        if net || own {
            if net { let bf = bf.unwrap(); g.qrf.fix = Some(bf.pos); g.qrf.fix_t = bf.t; }
            drop_drill(g);
            note(g, "drill-end:re-target");
            pick_target(g, el, false);
            return;
        }
    }
    for &man in el.iter() {
        if g.gd[man].held.is_some() {   // (in Sam's arm: off the entry: the orders would walk him out of it)
            in_cast(g, |g| { if crate::orders::squad_orders_has(g, man) { crate::orders::squad_orders_clear(g, Some(&[man])); } });
        }
    }
    in_cast(g, |g| { crate::doortools::door_tools_tick(g); crate::orders::squad_orders_tick(g); });
    while (g.qrf.op_beat as usize) < g.tab.doortools_door_op[op].beats.len() {
        let what = g.tab.doortools_door_op[op].beats[g.qrf.op_beat as usize].what.clone();
        note(g, &format!("drill:{}", what));
        g.qrf.op_beat += 1.0;
    }
    if let Some(can) = g.tab.doortools_door_op[op].can {
        if !crate::throwables::item_fired(g, can) { g.qrf.can = Some(can); g.qrf.can_land_t = g.time; }   // (perceiveScripted: the charge they put in is not a noise to them)
    }
    if g.tab.doortools_door_op[op].bang_t == g.time {
        if let Some(at) = g.tab.doortools_door_op[op].bang_at {
            let who = g.tab.doortools_door_op[op].threw.unwrap_or(g.tab.doortools_door_op[op].lead);
            their_bang(g, at, who);   // (it went off this tick: the shift were told on the net what is going in)
        }
    }
    let running = cast(g).tools.ops.contains(&op);
    if running && g.qrf.phase_t <= DRILL_CAP {
        g.qrf.bangs = cast(g).tools.bangs + (if g.tab.doortools_door_op[op].bang_held { 1.0 } else { 0.0 });
        return;
    }
    let why: String = if running { String::from("too long") } else { seam_op_ended(g, op).unwrap_or_else(|| String::from("gone")) };
    drop_drill(g);
    note(g, &format!("drill-end:{}", why));
    if why != "gone" { g.qrf.spent = Some(QrfSpent { room: t.room.clone(), fix_t: g.qrf.fix_t }); }
    if why == "gone" { hold360(g, el); return; }   // (the tools started afresh under it: they re-form where they stand)
    if why == "in" {
        g.qrf.drills_in += 1.0;
        g.qrf.phase = Phase::Called;
        g.qrf.phase_t = 0.0;
        let now = g.time;
        g.qrf.cleared.set(t.room.clone(), now);
        g.cleared_at.set(t.room.clone(), now);   // (the shift's pair reads the same ledger: phaseFlow)
        let rest = if g.qrf.bangs > 0.0 { "moving on" } else { "no bangs left, moving on" };
        radio(g, Some(el[0]), &format!("{} clear — negative contact. {}", t.room, rest));
        note(g, &format!("clear:{}", t.room));
    } else {
        let line = if why == "too long" {
            format!("{} is taking too long — leave it, next", t.room)
        } else {
            let word = crate::words::wg(g, &"into", &"onto");
            format!("can't get {} the {} from here — next", word, t.room)
        };
        radio(g, Some(el[0]), &line);
        let at = g.time - RECLEAR_SECS + 30.0;
        g.qrf.cleared.set(t.room.clone(), at);
        hold360(g, el);
    }
}

/// Contact: every man off his script and onto the AI, on the seer's fix; the entry let go of. (qrf.ts toFight)
pub fn to_fight(g: &mut crate::game::Game, el: &[GuardId], seer: GuardId) {
    let fix = match g.gd[seer].last_known { Some(k) => v3::copy(k), None => v3::copy(player_pos(g)) };
    for &man in el {
        if g.gd[man].script.is_some() { crate::squad::end_guard_script(g, man, Some(crate::squad::EndGuardScriptState::Alert)); }
        let is_seer = man == seer;
        g.gd[man].hold = false;
        g.gd[man].pinned = false;
        g.gd[man].state = crate::game::GuardState::Alert;
        g.gd[man].awareness = m::max(g.gd[man].awareness, if is_seer { 1.0 } else { 0.95 });
        let known = match g.gd[man].last_known { Some(k) if is_seer => k, _ => fix };
        g.gd[man].last_known = Some(v3::copy(known));
        let react = if is_seer { 0.12 } else { 0.2 + crate::math_dice::sim_random(g) * 0.25 };
        g.gd[man].react_t = react;
        g.gd[man].fire_cd = m::max(g.gd[man].fire_cd, 0.35);
        g.gd[man].search_t = 0.0;
        g.gd[man].foe = Some(g.gd[seer].foe.unwrap_or(crate::game::Foe::Player));
        crate::guards::reset_chase(g, man);
        g.gd[man].witness.alerted_by = Some(if is_seer { crate::game::WitnessAlertedBy::Sight } else { crate::game::WitnessAlertedBy::Radio });
        g.gd[man].witness.alert_t = g.time;
    }
    drop_drill(g);   // (the entry they were on is let go of: a can already in the air still goes off, on its own short fuse)
    g.qrf.fix = Some(fix);
    g.qrf.fix_t = g.time;
    g.qrf.phase = Phase::Fight;
    g.qrf.phase_t = 0.0;
    g.qrf.can = None;
    say(g, seer, "CONTACT!");
    let voice = el.iter().copied().find(|&man| man != seer).unwrap_or(seer);
    let tag = match marks_of(g) { Some(mk) => mk.tag, None => String::from("QRF") };
    let room = crate::dialogue::room_name(g, fix);
    radio(g, Some(voice), &format!("{} — contact, {}! push him", tag, room));
    let head = head_or_pos(g, seer);
    audio(g, "radio", Some(head), 0.8);
    if g.time - g.last_sting_t > 8.0 {
        audio(g, "alertSting", None, 0.75);
        g.last_sting_t = g.time;
    }
    let callsign = g.gd[seer].callsign.clone();
    note(g, &format!("contact:{}", callsign));
}

/// The fight is the AI's: warm, men dropped to the search go back on the fix; cold, the element re-forms on the point man. (qrf.ts phaseFight)
pub fn phase_fight(g: &mut crate::game::Game, el: &mut Vec<GuardId>) {
    // contact = anybody of theirs with eyes on him lately, or Sam's own report in their ears lately (the dead man's last sight counts)
    let mut contact_t = -1e9;
    for i in 0..g.qrf.men.len() {
        let man = g.qrf.men[i];
        contact_t = m::max(contact_t, g.gd[man].last_seen_t);
        if g.gd[man].state != crate::game::GuardState::Dead { contact_t = m::max(contact_t, g.gd[man].witness.heard.last_shot_t); }
    }
    let seeing = el.iter().any(|&man| g.gd[man].saw_player_this_frame || g.gd[man].saw_foe_this_frame);
    // a man the chase has dropped to the search while it is still warm goes straight back on the freshest fix (no 'CONTACT!': he never left it)
    if g.time - contact_t <= REGROUP_SECS {
        let bf = fix_of(g);
        for &man in el.iter() {
            if g.gd[man].state != crate::game::GuardState::Alert {
                g.gd[man].state = crate::game::GuardState::Alert;
                g.gd[man].awareness = m::max(g.gd[man].awareness, 0.9);
                g.gd[man].react_t = 0.15;
                if let Some(bf) = bf { g.gd[man].last_known = Some(v3::copy(bf.pos)); }
                g.gd[man].search_t = 0.0;
                g.gd[man].search_plan = None;
                crate::guards::reset_chase(g, man);
            }
        }
        return;
    }
    if seeing { return; }
    // cold: re-form on the point man where he stands, then choose again off the freshest fix
    g.qrf.phase = Phase::Regroup;
    g.qrf.phase_t = 0.0;
    let lead = el[0];
    let f = man_yaw(g, lead);
    for (k, &man) in el.iter().enumerate() {
        g.gd[man].standoff = None;
        g.gd[man].pieing = None;
        g.gd[man].search_plan = None;
        g.gd[man].state = crate::game::GuardState::Search;
        g.gd[man].awareness = HUNT_AWARENESS;
        let yaw = f + [0.0, -m::PI / 2.0, m::PI / 2.0, m::PI][k % 4];
        if k == 0 {
            let aim = sector_point(man_pos(g, man), f, None, None);
            crate::squad::order(g, man, crate::squad::OrderPatch {
                goal: Some(None), face: Some(Some(f)), strafe: Some(Some(false)), stance: Some(Some(crate::character::Stance::None)), upper: Some(Some(crate::character::UpperMode::Aim)),
                crouch: Some(Some(false)), aim_at: Some(Some(aim)), ..Default::default()
            });
        } else {
            let lp = man_pos(g, lead);
            let n = k as f64;
            let off = if k == 1 { 0.5 } else if k == 2 { -0.5 } else { 0.0 };
            let st: V3 = [lp[0] - m::sin(f) * n * 1.1 + m::cos(f) * off, 0.0, lp[2] - m::cos(f) * n * 1.1 - m::sin(f) * off];
            let spot = nearest_free(g, st).unwrap_or(st);
            route(g, man, &[spot], Some(RoutePatch {
                speed: Some(2.2), face: Some(yaw), strafe: Some(false), stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim),
                crouch: Some(false), aim_at: Some(sector_point(st, yaw, None, None)), ..Default::default()
            }));
        }
    }
    let the = crate::words::w(g, &"", &"the ");
    let at = match fix_of(g) { Some(bf) => bf.pos, None => man_pos(g, lead) };
    let room = crate::dialogue::room_name(g, at);
    radio(g, Some(lead), &format!("lost him. on me — re-form, we clear toward {}{}", the, room));
    note(g, "regroup");
}

/// A man's waypoint queue is set from `pts` and his goal is its head. (qrf.ts route)
pub fn route(g: &mut crate::game::Game, m: GuardId, pts: &[V3], patch: Option<crate::qrf::RoutePatch>) {
    let w: Vec<V3> = pts.iter().map(|p| v3::copy(*p)).collect();
    let goal = w.first().copied();   // (`w[0] ?? null`: None halts him. A new leg, even at the same spot: `held` stays false, every goal here is a new object)
    g.qrf.wps.set(m, w);
    let patch = patch.unwrap_or_default();
    crate::squad::order(g, m, crate::squad::OrderPatch {
        goal: Some(goal), speed: patch.speed.map(Some), face: patch.face.map(Some), strafe: patch.strafe.map(Some), aim_at: patch.aim_at.map(Some),
        stance: patch.stance.map(Some), stack_side: patch.stack_side.map(Some), crouch: patch.crouch.map(Some), lean: patch.lean.map(Some), upper: patch.upper.map(Some),
        light: patch.light.map(Some), arrived: patch.arrived.map(Some), hot: patch.hot.map(Some), flinch: patch.flinch.map(Some), direct: patch.direct.map(Some),
        smoke: patch.smoke.map(Some), over: patch.over.map(Some), held: false,
    });
}

/// Past a waypoint, on to the next; true when on the last one and arrived. (qrf.ts advance)
pub fn advance(g: &mut crate::game::Game, m: GuardId) -> bool {
    if !g.qrf.wps.has(m) || g.gd[m].script.is_none() { return man_arrived(g, m); }
    loop {
        let next = {
            let w = g.qrf.wps.get(m).unwrap();
            if w.len() > 1 { w[0] } else { break; }
        };
        if !(flat(man_pos(g, m), next) < 0.75) { break; }
        let w = g.qrf.wps.get_mut(m).unwrap();
        w.remove(0);
        let goal = w[0];
        crate::squad::order(g, m, crate::squad::OrderPatch { goal: Some(Some(goal)), ..Default::default() });
    }
    g.qrf.wps.get(m).unwrap().len() <= 1 && man_arrived(g, m)
}

/// Tier 2's trigger past deployment: one of theirs down, enough men down, a running gunfight. (qrf.ts maybePerimeter)
pub fn maybe_perimeter(g: &mut crate::game::Game) {
    if !g.tune.perimeter || !live(g) || g.qrf.perim.is_some() || raid(g) { return; }   // (a raid: one team, no second lift)
    if !marks_of(g).map_or(false, |mk| !mk.posts.is_empty()) { return; }   // (this place has no second tier)
    let mut down = 0usize;
    for i in 0..g.guards.len() {
        let x = g.guards[i];
        if g.gd[x].team == crate::teams::Team::Guard && g.gd[x].state == crate::game::GuardState::Dead && (!net_rule(g) || g.gd[x].found) { down += 1; }   // (RESPONSE.net: men they have FOUND down, not men who are)
    }
    let why = if g.qrf.lost > 0.0 {
        Some(String::from("one of ours down"))
    } else if down as f64 >= DOWN_FOR_PERIMETER {
        Some(format!("{} men down", down))
    } else if g.qrf.shots_since.len() as f64 >= GUNFIGHT_SHOTS {
        Some(String::from("a running gunfight"))
    } else {
        None
    };
    if let Some(why) = why { call_perimeter(g, Some(&why)); }
}

/// Lock the compound down: control's word, the exterior lamps up, men to the outside posts, spawned if there is room, else the element's rear men. One a night. (qrf.ts callPerimeter)
pub fn call_perimeter(g: &mut crate::game::Game, why: Option<&str>) -> bool {
    let why = why.unwrap_or("panel");
    if g.qrf.perim.is_some() { return false; }
    if g.qrf.state != QrfState::Deployed && why != "panel" { return false; }
    if !has_response(g) { return false; }   // (the posts are the office's lot)
    let mk = marks_of(g).unwrap();
    if mk.posts.is_empty() { return false; }
    let want = m::min(v3::clamp(m::round(g.tune.perimeter_size), 2.0, 4.0), mk.posts.len() as f64);
    let now = g.time;
    g.qrf.perim = Some(Perimeter {
        state: PerimeterState::Called, called_at: now, eta_at: now + m::max(5.0, g.tune.perimeter_eta), set_at: -1.0, men: Vec::new(), spawned: 0.0, peeled: 0.0,
    });
    let lit = if g.targets.iter().any(|t| g.tab.sim_lightshome_fixture[*t].group == "exterior") { ". outside lights coming up" } else { "" };
    let word = format!("all stations — lock {} down, {}. nobody leaves: a man on every way out, {} seconds{}", mk.site, why, m::round(g.tune.perimeter_eta), lit);
    control(g, &word);
    crate::lighting::set_group(g, "exterior", true);   // the lot's lamps, all of them: back as authored on a restart
    if crate::mission::mission_live(g) { g.mission.stats.qrf = 2.0; }
    // who mans it
    let mut outside: Vec<V3> = Vec::new();   // (a mark an enemy stands by stands nobody up: the rear men go out instead)
    for &o in mk.outside.iter() { if clear_of_foes(g, o) { outside.push(o); } }
    let room = room_for(g);
    let most = most_men(g);
    let up = alive(g).len() as f64;
    let n_spawn = m::min_n(&[want, m::max(0.0, room), outside.len() as f64, m::max(0.0, most - up)]);   // (never more of them on the floor than the level's most)
    let mut i = 0usize;
    while (i as f64) < n_spawn {
        let budget = match budget_of(g) { BudgetOf::Arm0(b) => Some(b.budget), BudgetOf::Arm1(_) => None };
        let o = crate::guards::UnitOpts {
            team: crate::teams::Team::Guard, pos: outside[i], yaw: Some(m::PI), weapon: Some(crate::weapons::WeaponId::Aks74u), name: Some(format!("GATE-{}", i + 1)),
            light: Some(true), tint: Some(crate::consts::qrf_tint()), tint2: Some(crate::consts::qrf_tint2()), budget, carbine_rig: None, pair: None,
        };
        match seam_try_spawn_unit(g, &o) {
            Ok(gd) => {
                g.gd[gd].trained = true;
                g.gd[gd].hold = false;
                g.gd[gd].state = crate::game::GuardState::Search;
                g.gd[gd].awareness = HUNT_AWARENESS;
                g.gd[gd].heard_up_to = g.event_seq;
                let p = g.qrf.perim.as_mut().unwrap();
                let post = mk.posts[p.men.len()].clone();
                p.men.push(PerimeterMenItem { gd, post, arrived_at: -1.0, fighting: false });
                p.spawned += 1.0;
            }
            Err(e) => {
                let short: String = e.chars().take(40).collect();
                note(g, &format!("perim-spawn-failed:{}", short));
                break;
            }
        }
        i += 1;
    }
    let el = alive(g);
    let spare = m::max(0.0, el.len() as f64 - 2.0);
    let n_peel = m::min(want - g.qrf.perim.as_ref().unwrap().men.len() as f64, spare);
    let on_drill = n_peel > 0.0 && g.qrf.op.is_some();
    if on_drill { drop_drill(g); note(g, "drill-end:peel"); }   // (men leave the element under an entry: it is let go of whole, and who is left re-forms)
    let mut i = 0usize;
    while (i as f64) < n_peel {
        let gd = el[el.len() - 1 - i];   // the rear men go out
        if g.gd[gd].script.is_some() { crate::squad::end_guard_script(g, gd, Some(crate::squad::EndGuardScriptState::Search)); }
        g.gd[gd].state = crate::game::GuardState::Search;
        g.gd[gd].standoff = None;
        g.gd[gd].pieing = None;
        g.gd[gd].search_plan = None;
        let p = g.qrf.perim.as_mut().unwrap();
        let post = mk.posts[p.men.len()].clone();
        p.men.push(PerimeterMenItem { gd, post, arrived_at: -1.0, fighting: false });
        p.peeled += 1.0;
        i += 1;
    }
    let out: Vec<GuardId> = g.qrf.perim.as_ref().unwrap().men.iter().map(|p| p.gd).collect();
    g.qrf.men.retain(|man| !out.contains(man));   // (peeled men leave the element's order; they stay 'owned' through perim)
    for &man in out.iter() { g.qrf.wps.delete(man); g.qrf.stations.delete(man); }
    for i in 0..out.len() {
        let mut row = g.qrf.perim.as_ref().unwrap().men[i].clone();
        to_post(g, &mut row);
        g.qrf.perim.as_mut().unwrap().men[i] = row;
    }
    if on_drill { let left = alive(g); hold360(g, &left); }
    if let Some(&first) = out.first() {
        let names: Vec<String> = out.iter().map(|&man| g.gd[man].callsign.clone()).collect();
        let posts: Vec<String> = g.qrf.perim.as_ref().unwrap().men.iter().map(|p| p.post.name.clone()).collect();
        radio(g, Some(first), &format!("{} — going outside. {}, from the outside, lights on the doors", names.join(", "), posts.join(" and ")));
    } else {
        let line = crate::words::wg(g, &"there's no outside team — the lights are all you get. watch the exits from inside", &"no outside team to be had — the lamps are all you get. watch the exits from inside");
        control(g, line);
    }
    let (spawned, peeled) = { let p = g.qrf.perim.as_ref().unwrap(); (p.spawned, p.peeled) };
    note(g, &format!("perimeter:{}:{}s+{}p", why, spawned, peeled));
    true
}

/// An outside man's walk to his post. (qrf.ts toPost)
pub fn to_post(g: &mut crate::game::Game, p: &mut crate::qrf::PerimeterMenItem) {
    let gd = p.gd;
    g.gd[gd].pinned = false;
    g.gd[gd].hold = false;
    p.fighting = false;
    crate::squad::order(g, gd, crate::squad::OrderPatch {
        goal: Some(Some(v3::copy(p.post.pos))), speed: Some(Some(POST_SPEED)), face: Some(Some(p.post.yaw)), strafe: Some(Some(false)), stance: Some(Some(crate::character::Stance::None)),
        upper: Some(Some(crate::character::UpperMode::Aim)), crouch: Some(Some(false)), aim_at: Some(Some(v3::copy(p.post.sweep[0]))), ..Default::default()
    });
}

/// The outside men: the walk, the light working the door, eyes on him, back to the post when cold; the perimeter called set. (qrf.ts tickPerimeter)
pub fn tick_perimeter(g: &mut crate::game::Game, dt: f64) {
    let mut i = 0usize;
    while i < g.qrf.perim.as_ref().unwrap().men.len() {
        let idx = i;
        i += 1;
        let (gd, post) = { let row = &g.qrf.perim.as_ref().unwrap().men[idx]; (row.gd, row.post.clone()) };
        if g.gd[gd].state == crate::game::GuardState::Dead { continue; }
        if g.gd[gd].held.is_some() { continue; }   // in Sam's arm: off the script and off his own eyes, not re-ordered to the post from under the arm
        if g.gd[gd].script.is_some() {
            if g.qrf.perim.as_ref().unwrap().men[idx].arrived_at < 0.0 && (man_arrived(g, gd) || (flat(man_pos(g, gd), post.pos) < 0.7 && g.gd[gd].speed < 0.2)) {
                g.qrf.perim.as_mut().unwrap().men[idx].arrived_at = g.time;
                g.gd[gd].pinned = true;
                note(g, &format!("post:{}", post.name));
            }
            let arrived_at = g.qrf.perim.as_ref().unwrap().men[idx].arrived_at;
            // the light works the door: the aim point eased to and fro along the stretch of wall the post covers
            if arrived_at >= 0.0 {
                let u = 0.5 + 0.5 * m::sin((g.time - arrived_at) * (2.0 * m::PI / SWEEP_SECS));
                let [a, b] = post.sweep;
                g.gd[gd].script.as_mut().unwrap().aim_at = Some([a[0] + (b[0] - a[0]) * u, a[1] + (b[1] - a[1]) * u, a[2] + (b[2] - a[2]) * u]);
            }
            let eyes = perceive_scripted(g, gd, dt);
            let him = match crate::teams::foe_char(g, gd) { Some(c) => g.ch[c].pos, None => player_pos(g) };   // (the man he has made out: perceiveScripted names him)
            if eyes && arrived_at < 0.0 && flat(man_pos(g, gd), him) > WALKOUT_ENGAGE_M && g.gd[gd].sup.v < 0.3 {
                // on his way out and Sam not on top of him: he calls it (the element's fix) and keeps going: the post is the order
                g.gd[gd].awareness = m::min(g.gd[gd].awareness, 0.84);
                g.qrf.fix = Some(v3::copy(him));
                g.qrf.fix_t = g.time;
            } else if eyes {
                // eyes on him from the post (or at arm's length on the way): the AI fires from where he stands, the others are told where
                crate::squad::end_guard_script(g, gd, Some(crate::squad::EndGuardScriptState::Alert));
                g.gd[gd].pinned = arrived_at >= 0.0;
                g.gd[gd].state = crate::game::GuardState::Alert;
                g.gd[gd].awareness = 1.0;
                g.gd[gd].react_t = 0.35;
                g.gd[gd].fire_cd = m::max(g.gd[gd].fire_cd, 0.4);
                crate::guards::reset_chase(g, gd);
                g.qrf.perim.as_mut().unwrap().men[idx].fighting = true;
                g.gd[gd].witness.alerted_by = Some(crate::game::WitnessAlertedBy::Sight);
                g.gd[gd].witness.alert_t = g.time;
                say(g, gd, "CONTACT — outside!");
                radio(g, Some(gd), &format!("contact outside — {}, he's coming out!", post.name));
                if g.time - g.last_sting_t > 8.0 {
                    audio(g, "alertSting", None, 0.75);
                    g.last_sting_t = g.time;
                }
                note(g, &format!("perim-contact:{}", post.name));
            }
        } else {
            // fighting from the post; eight seconds cold (or the AI talked him down a state) and he squares back up to the door
            let cold = g.time - m::max(g.gd[gd].last_seen_t, g.gd[gd].witness.heard.last_shot_t) > REGROUP_SECS && !g.gd[gd].saw_player_this_frame && !g.gd[gd].saw_foe_this_frame;
            if cold || g.gd[gd].state != crate::game::GuardState::Alert {
                g.gd[gd].state = crate::game::GuardState::Search;
                g.gd[gd].awareness = HUNT_AWARENESS;
                g.gd[gd].standoff = None;
                g.gd[gd].search_plan = None;
                let mut row = g.qrf.perim.as_ref().unwrap().men[idx].clone();
                to_post(g, &mut row);
                g.qrf.perim.as_mut().unwrap().men[idx] = row;
                if g.qrf.perim.as_ref().unwrap().men[idx].arrived_at >= 0.0 && flat(man_pos(g, gd), post.pos) < 1.0 { g.gd[gd].pinned = true; }
            }
        }
    }
    if g.qrf.perim.as_ref().unwrap().state == PerimeterState::Called {
        let (n, eta_at) = { let p = g.qrf.perim.as_ref().unwrap(); (p.men.len(), p.eta_at) };
        let all_set = n > 0 && g.qrf.perim.as_ref().unwrap().men.iter().all(|p| g.gd[p.gd].state == crate::game::GuardState::Dead || p.arrived_at >= 0.0);
        if all_set || g.time > eta_at + 10.0 {
            let now = g.time;
            let p = g.qrf.perim.as_mut().unwrap();
            p.state = PerimeterState::Set;
            p.set_at = now;
            let up: Vec<(GuardId, String)> = g.qrf.perim.as_ref().unwrap().men.iter()
                .filter(|p| g.gd[p.gd].state != crate::game::GuardState::Dead && p.arrived_at >= 0.0)
                .map(|p| (p.gd, p.post.name.clone())).collect();
            if let Some(first) = up.first() {
                let names: Vec<String> = up.iter().map(|u| u.1.clone()).collect();
                radio(g, Some(first.0), &format!("perimeter set — eyes on {}. lights are up outside; nobody walks out of here", names.join(" and ")));
            } else {
                let line = crate::words::wg(g, &"perimeter is what it is — lights up, no eyes outside. hold the exits from inside", &"perimeter is what it is — lamps up, no eyes outside. hold the exits from inside");
                control(g, line);
            }
            note(g, &format!("perimeter-set:{}", up.len()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Which leg each man of the flow is given is this file's own decision: one and two the plan's entries, three the deep point, four the door side.
    #[test]
    fn flow_legs_gives_each_man_his_own_way_in() {
        let mut p = crate::scene_level::RoomClearPlan::zeroed();
        p.entry = [vec![[1.0, 0.0, 0.0], [2.0, 0.0, 0.0]], vec![[3.0, 0.0, 0.0], [4.0, 0.0, 0.0]]];
        p.entry_aim = [vec![[5.0, 0.0, 0.0], [6.0, 0.0, 0.0]], vec![[7.0, 0.0, 0.0], [8.0, 0.0, 0.0]]];
        p.pod_face = [0.25, 0.5];
        p.search_path = vec![[9.0, 0.0, 0.0], [10.0, 0.0, 0.0]];
        p.search_face = 0.75;
        p.search_aim_walk = vec![[11.0, 0.0, 0.0], [12.0, 0.0, 0.0]];
        p.form_up = [vec![[13.0, 0.0, 0.0]], vec![[14.0, 0.0, 0.0], [15.0, 0.0, 0.0]]];
        p.form_face = [1.25, 1.5];
        p.stack_aim = [[16.0, 0.0, 0.0], [17.0, 0.0, 0.0]];

        let l = flow_legs(&p, 0.0);
        assert_eq!((l.pts, l.face, l.aim), (p.entry[0].clone(), 0.25, [6.0, 0.0, 0.0]));
        let l = flow_legs(&p, 1.0);
        assert_eq!((l.pts, l.face, l.aim), (p.entry[1].clone(), 0.5, [8.0, 0.0, 0.0]));
        let l = flow_legs(&p, 2.0);
        assert_eq!((l.pts, l.face, l.aim), (vec![[1.0, 0.0, 0.0], [9.0, 0.0, 0.0]], 0.75, [12.0, 0.0, 0.0]));
        let l = flow_legs(&p, 3.0);
        assert_eq!((l.pts, l.face, l.aim), (vec![[3.0, 0.0, 0.0], [15.0, 0.0, 0.0]], 1.5, [16.0, 0.0, 0.0]));
    }
}
