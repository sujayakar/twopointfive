//! THE ROOM FIGHT: its tick, what is heard in it, what it keeps of the whole, and the small questions asked of it.
#![allow(unused_imports)]
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Hard's drilled defenders: only the night breach by itself, under the press's own switch.
pub fn drilled_on(g: &crate::game::Game) -> bool {
    g.k.pressure.on && g.k.pressure.drilled && g.night_breach.active && g.night_breach.layout.is_none()
}

/// × the seconds a stun canister doubles this man over: DRILLED.shaken for a drilled defender, 1 for anybody else.
pub fn drilled_k(g: &crate::game::Game, gd: GuardId) -> f64 {
    if g.gd[gd].team == crate::teams::Team::Hostile && drilled_on(g) { g.k.drilled.shaken } else { 1.0 }
}

/// Distance on the floor.
pub fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// A draw of the simulation's die in a range.
pub fn pick(g: &mut crate::game::Game, who: crate::dice::Who, r: [f64; 2]) -> f64 {
    r[0] + crate::dice::draw_for(g, "roomfight::pick#1", who) * (r[1] - r[0])
}

/// His head bone, or where it would be.
pub fn eye_of(g: &crate::game::Game, c_2: CharId) -> V3 {
    match g.ch[c_2].bones.get(crate::character::Bone::Head) {
        Some(p) => *p,
        None => v3::add(g.ch[c_2].pos, [0.0, 1.65, 0.0]),
    }
}

/// His chest bone, or where it would be.
pub fn chest_of(g: &crate::game::Game, c_2: CharId) -> V3 {
    match g.ch[c_2].bones.get(crate::character::Bone::Chest) {
        Some(p) => *p,
        None => v3::add(g.ch[c_2].pos, [0.0, 1.1, 0.0]),
    }
}

/// What of the light from `a`'s eye to `b`'s chest comes through the smoke, a picked weather's fog on top.
pub fn pair_smoke(g: &mut crate::game::Game, a: CharId, b: CharId) -> f64 {
    let (eye, chest) = (eye_of(g, a), chest_of(g, b));
    let t = crate::sense::smoke_between(g, eye, chest);
    if crate::weather::weather_fog(g).is_some() {
        let ca = chest_of(g, a);
        t * crate::weather::air_trans(g, ca, chest)
    } else {
        t
    }
}

/// The light to read for `o` when `eye` stands inside the cone of his burning weapon light, else 0: as strong beside an hour's light as the lamp is, to eyes used to the place.
pub fn lamp_glare(g: &mut crate::game::Game, eye: CharId, o: GuardId) -> f64 {
    if !g.fight.live { return 0.0; }
    let l = g.gd[o].flashlight;
    let light = &g.tab.light[l];
    if !light.enabled || !(light.intensity > 0.0) || g.gd[o].state == crate::game::GuardState::Dead { return 0.0; }
    let (pos, dir, range, outer_deg) = (light.pos, light.dir, light.range, light.outer_deg);
    let e = eye_of(g, eye);
    let (dx, dy, dz) = (e[0] - pos[0], e[1] - pos[1], e[2] - pos[2]);
    let len = m::hyp3(dx, dy, dz);
    if len > range || len < 1e-3 { return 0.0; }
    if !((dx * dir[0] + dy * dir[1] + dz * dir[2]) / len >= m::cos(outer_deg * v3::DEG)) { return 0.0; }
    let exposure = crate::hours::exposure(g);
    if exposure >= 1.0 { return g.k.roomfight.lamp; }
    let k = sim::light::by_day(crate::render_lights::light_by_day(light), light.intensity, light.peak_intensity, exposure) / light.intensity;
    let k = m::min(1.0, k * crate::guards::eyes_in(g, e));
    m::max(0.0, g.k.roomfight.lamp + g.tune.vis_slope * m::log(k))
}

/// Is the room fight running this frame?
pub fn fight_live(g: &crate::game::Game) -> bool {
    g.fight.live
}

/// The clearance a friend is owed against a line of fire `l` long, he `t` of the way along it.
pub fn close_lane(g: &crate::game::Game, clear: f64, t: f64, l: f64) -> f64 {
    lane(clear, t, l, g.k.roomfight.point_blank)
}

/// closeLane's sum, with the knob it reads handed in.
pub(super) fn lane(clear: f64, t: f64, l: f64, point_blank: f64) -> f64 {
    m::min(clear, if t > 1.0 && l < point_blank { 0.3 } else { 0.3 + 0.25 * m::max(0.0, t) * l })
}

/// How fast the player on this body is really getting anywhere; Infinity while not known yet.
pub fn room_fight_sam_speed_of(g: &crate::game::Game, ch: CharId) -> f64 {
    let mut i = 0;
    while i < g.seats.len() {
        let seat = g.seats[i];
        let man = crate::cmd::seat_man(g, seat);
        if g.pl[man].ch == ch {
            return match g.fight.sams.get(i) {
                Some(t) if !(t.known < 0.25) => m::hyp2(t.v[0], t.v[1]),
                _ => m::INF,
            };
        }
        i += 1;
    }
    m::INF
}

/// A new room fight: the class's field initialisers; its constructor only keeps `g`.
pub fn room_fight_new(g: &crate::game::Game) -> crate::roomfight::RoomFight {
    crate::roomfight::RoomFight::blank()
}

/// May the module run at all: the AI, sides, and no show holding the floor (a tutorial's own program does not hold it).
pub fn room_fight_on(g: &crate::game::Game) -> bool {
    if !(g.ai_enabled && g.tune.teams && !g.touring && !g.quiet_utility && g.puppet.is_none()) { return false; }
    let show_runs = match g.director.run { Some(r) => !g.tab.director_run[r].ended, None => false };
    !(show_runs && !crate::rails::rails_live(g))
}

/// A line of the fight's log, kept to its last few hundred.
pub fn room_fight_note(g: &mut crate::game::Game, gd: Option<GuardId>, what: &str) {
    let who = match gd { Some(x) => g.gd[x].callsign.clone(), None => String::from("-") };
    let t = g.time;
    g.fight.log.push(crate::roomfight::FightNote { t, who, what: String::from(what) });
    if g.fight.log.len() > 600 { g.fight.log.drain(0..200); }
}

/// `this.ops.find(o => o.gd === gd)`: the first operative's row for this man.
pub(super) fn op_of(g: &crate::game::Game, gd: GuardId) -> Option<RoomfightOpId> {
    g.fight.ops.iter().copied().find(|&o| g.tab.roomfight_op[o].gd == gd)
}

/// An operative's muzzle is on a man, or coming onto him.
pub fn room_fight_engaged(g: &crate::game::Game, gd: GuardId) -> bool {
    match op_of(g, gd) { Some(o) => g.tab.roomfight_op[o].tgt.is_some(), None => false }
}

/// Seconds he has been on his man with no round at him from where he stands.
pub fn room_fight_no_shot(g: &crate::game::Game, gd: GuardId) -> f64 {
    match op_of(g, gd) {
        Some(o) if g.tab.roomfight_op[o].tgt.is_some() => g.tab.roomfight_op[o].block_t,
        _ => 0.0,
    }
}

/// An operative is on a man, turning to one, or lost one inside the last `secs`.
pub fn room_fight_busy(g: &crate::game::Game, gd: GuardId, secs: f64) -> bool {
    let now = g.time;
    match op_of(g, gd) {
        Some(o) => {
            let o = &g.tab.roomfight_op[o];
            o.tgt.is_some() || o.turn.is_some() || now - o.lost_at < secs
        }
        None => false,
    }
}

/// An operative has a living hostile in his sight right now, target or not.
pub fn room_fight_contact(g: &crate::game::Game, gd: GuardId) -> bool {
    let now = g.time;
    let Some(o) = op_of(g, gd) else { return false; };
    for (h, meter) in g.tab.roomfight_op[o].meters.iter() {
        if g.gd[h].state != crate::game::GuardState::Dead && now - meter.seen_t < 0.3 { return true; }
    }
    false
}

/// Does any living teammate see this man right now?
pub fn room_fight_team_sees(g: &crate::game::Game, h: GuardId) -> bool {
    room_fight_seen_by(g, h).len() > 0
}

/// The living teammates who see this man: the scripted by this module's meters, one on the fight AI by its own eyes.
pub fn room_fight_seen_by(g: &crate::game::Game, h: GuardId) -> Vec<GuardId> {
    let t = g.time;
    let mut out = Vec::new();
    for &id in g.fight.ops.iter() {
        let o = &g.tab.roomfight_op[id];
        let man = &g.gd[o.gd];
        if man.state == crate::game::GuardState::Dead { continue; }
        let by_meter = match o.meters.get(h) { Some(meter) => t - meter.seen_t < 0.3, None => false };
        let by_own_eyes = man.script.is_none()
            && matches!(man.foe, Some(crate::game::Foe::Guard(f)) if f == h)
            && t - man.last_seen_t < 0.3;
        if by_meter || by_own_eyes { out.push(o.gd); }
    }
    out
}

pub(super) fn op_id(g: &Game, gd: GuardId) -> Option<RoomfightOpId> { g.fight.ops.iter().copied().find(|&o| g.tab.roomfight_op[o].gd == gd) }

/// `g.col.segmentBlocked(a, b)`
pub(super) fn seg_blocked(g: &mut Game, a: V3, b: V3) -> bool { crate::scene_collision::segment_blocked(g, a, b, None, None, None) }

pub(super) fn pos_of(g: &Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }

pub(super) fn is_dead(g: &Game, gd: GuardId) -> bool { g.gd[gd].state == crate::game::GuardState::Dead }

/// a man off `g.guards` by his body's number (`g.guards.find(x => x.char.id === e.who)`)
pub(super) fn guard_by_char_id(g: &Game, who: Option<f64>) -> Option<GuardId> { let w = who?; g.guards.iter().copied().find(|&x| g.ch[g.gd[x].ch].id == w) }

/// is the floor hot in this teammate's ears.
pub fn room_fight_hot_for(g: &crate::game::Game, gd: GuardId) -> bool { g.time - room_fight_hot_of(g, gd) < g.k.roomfight.team_hot }

pub fn room_fight_hot_of(g: &crate::game::Game, gd: GuardId) -> f64 { match op_id(g, gd) { Some(o) => m::max(g.tab.roomfight_op[o].hot_t, g.fight.hot_t), None => g.fight.hot_t } }

pub fn room_fight_op_of(g: &crate::game::Game, gd: GuardId) -> Option<RoomfightOpId> { op_id(g, gd) }

/// Once a frame from Game.update.
pub fn room_fight_tick(g: &mut crate::game::Game, dt: f64) {
    if !room_fight_on(g) { if g.fight.live { room_fight_stop(g); } return; }
    let any = g.guards.iter().any(|&x| g.gd[x].unit.is_some() && (g.gd[x].team == crate::teams::Team::Hostile || g.gd[x].team == crate::teams::Team::Operative));
    if !any { if g.fight.live { room_fight_stop(g); } return; }
    g.fight.live = true;
    for i in 0..g.seats.len() {
        let s = g.seats[i]; let man = crate::cmd::seat_man(g, s); let p = g.ch[g.pl[man].ch].pos;
        while g.fight.sams.len() <= i { g.fight.sams.push(RoomFightSamsItem { x: 0.0, z: 0.0, t: f64::NEG_INFINITY, v: [0.0, 0.0], known: 0.0 }); }
        let now = g.time; let A = &mut g.fight.sams[i]; let since = now - A.t;
        if since > 1e-4 && since < 0.25 { let k = 1.0 - m::exp(-since / 0.12); A.v[0] += ((p[0] - A.x) / since - A.v[0]) * k; A.v[1] += ((p[2] - A.z) / since - A.v[1]) * k; A.known += since; }
        else if since != 0.0 { A.v = [0.0, 0.0]; A.known = 0.0; }
        A.x = p[0]; A.z = p[2]; A.t = now;
    }
    let guards = g.guards.clone();
    g.fight.defs.retain(|&d| guards.contains(&g.tab.roomfight_def[d].gd));
    g.fight.ops.retain(|&o| guards.contains(&g.tab.roomfight_op[o].gd));
    let mut gi = 0;
    while gi < g.guards.len() {
        let x = g.guards[gi]; gi += 1;
        if g.gd[x].unit.is_none() || is_dead(g, x) { continue; }
        if g.gd[x].team == crate::teams::Team::Hostile && g.gd[x].hold && def_of(g, x).is_none() { let a = room_fight_adopt(g, x); g.fight.defs.push(a); }
        else if g.gd[x].team == crate::teams::Team::Operative && op_id(g, x).is_none() { let o = room_fight_new_op(g, x); g.fight.ops.push(o); }
    }
    let mut ei = 0;
    while ei < g.events.len() {
        let e = g.events[ei]; let key = e.serial; ei += 1;
        if g.fight.heard_ev.has(key) { continue; }
        if e.kind == crate::director::EventKind::Step && e.who.is_some() { hear_step(g, key, e); continue; }
        if e.kind == crate::director::EventKind::Door && e.who.is_some() { hear_door(g, key, e); continue; }
        let gunfire = (e.kind == crate::director::EventKind::Shot || e.kind == crate::director::EventKind::GuardShot) && e.level.unwrap_or(0.0) >= 1.0;
        if !gunfire && e.kind != crate::director::EventKind::Bang && e.kind != crate::director::EventKind::Kick { continue; }
        g.fight.heard_ev.add(key);
        for i in 0..g.fight.ops.len() {
            let o = g.fight.ops[i]; let og = g.tab.roomfight_op[o].gd;
            if !is_dead(g, og) && v3::dist(e.pos, pos_of(g, og)) <= (if e.kind == crate::director::EventKind::Kick { 40.0 } else { 80.0 }) && e.time > g.tab.roomfight_op[o].hot_t { g.tab.roomfight_op[o].hot_t = e.time; }
        }
        if e.kind == crate::director::EventKind::GuardShot {
            if let Some(w) = guard_by_char_id(g, e.who) { if g.gd[w].team == crate::teams::Team::Hostile {
                let both = e.from.is_some() && e.to.is_some();
                g.fight.reports.push(RoomFightReportsItem { by: w, t: e.time, from: if both { e.from } else { None }, to: if both { e.to } else { None } });
            } }
        }
    }
    while !g.fight.reports.is_empty() && g.time - g.fight.reports[0].t > 2.0 { g.fight.reports.remove(0); }
    for (k, d) in g.fight.defs.clone().into_iter().enumerate() {
        let gd = g.tab.roomfight_def[d].gd;
        if !is_dead(g, gd) { room_fight_tick_def(g, d, dt, k as f64); }
    }
    let mut i = 0;
    while i < g.fight.ops.len() { let o = g.fight.ops[i]; i += 1; room_fight_tick_op(g, o, dt); }
    crate::pressure::press_tick(g, dt);
    crate::laststand::last_stand_tick(g);
}

/// an enemy's footfall: heard by a teammate as any man hears a step
pub(super) fn hear_step(g: &mut Game, key: u32, e: crate::game::GameEvent) {
    g.fight.heard_ev.add(key);
    let w = match guard_by_char_id(g, e.who) { Some(w) if g.gd[w].team == crate::teams::Team::Hostile => w, _ => return };
    let wc = g.gd[w].ch;
    let (mut near, mut nd, mut from): (Option<RoomfightOpId>, f64, V3) = (None, f64::INFINITY, e.pos);
    for i in 0..g.fight.ops.len() {
        let o = g.fight.ops[i]; let og = g.tab.roomfight_op[o].gd; let oc = g.gd[og].ch;
        if is_dead(g, og) || v3::dist(e.pos, g.ch[oc].pos) > 9.0 { continue; }
        let pr = crate::weather::hear_path(g, v3::add(e.pos, [0.0, 1.0, 0.0]), v3::add(g.ch[oc].pos, [0.0, 1.5, 0.0]), None);
        if !(pr.carry > 0.5 && e.level.unwrap_or(0.3) * v3::clamp(1.0 - pr.path_len / 9.0, 0.0, 1.0) * pr.carry > 0.12) { continue; }
        g.tab.roomfight_op[o].step_t = e.time;
        let (eye, chest) = (eye_of(g, oc), chest_of(g, wc));
        let sc = if crate::guards::eye_blocked(g, og, eye, chest) { 100.0 + pr.path_len } else { m::abs(v3::wrap_angle(v3::yaw_to(g.ch[oc].pos, g.ch[wc].pos) - g.ch[oc].aim_yaw)) };
        if sc < nd { nd = sc; near = Some(o); from = if pr.routed { pr.via } else { e.pos }; }
    }
    if let Some(o) = near { g.tab.roomfight_op[o].step_by = Some(OpStepBy { by: w, t: e.time, at: [from[0], 1.1, from[2]] }); }
}

/// a door noise under an enemy's hand, kept for cue()
pub(super) fn hear_door(g: &mut Game, key: u32, e: crate::game::GameEvent) {
    g.fight.heard_ev.add(key);
    let w = match guard_by_char_id(g, e.who) { Some(w) if g.gd[w].team == crate::teams::Team::Hostile && g.gd[w].unit.is_some() => w, _ => return };
    let (mut dr, mut dm): (Option<DoorId>, f64) = (None, 1.6);
    for x in g.doors.list.clone() { let q = flat(crate::doors::door_pos(g, x), e.pos); if q < dm { dm = q; dr = Some(x); } }
    let dr = match dr { Some(x) if !crate::doors::door_off(g, x) => x, _ => return };
    let fc = g.dr[dr].frame_centre;
    let (mut free, mut post, mut post_at, mut post_turn): (i32, Option<RoomfightOpId>, Option<V3>, f64) = (0, None, None, f64::INFINITY);
    for i in 0..g.fight.ops.len() {
        let o = g.fight.ops[i]; let og = g.tab.roomfight_op[o].gd; let oc = g.gd[og].ch; let op = g.ch[oc].pos; let d = flat(fc, op);
        if is_dead(g, og) || d > 9.0 || d < 0.5 { continue; }
        if let Some(b) = g.tab.roomfight_op[o].door_by { if b.door == dr && e.time - b.t < 0.6 { continue; } }
        let k = 0.35 / d; let at: V3 = [fc[0] + (op[0] - fc[0]) * k, 1.1, fc[2] + (op[2] - fc[2]) * k];
        let pr = crate::weather::hear_path(g, at, v3::add(op, [0.0, 1.5, 0.0]), None);
        if !(pr.carry > 0.5 && e.level.unwrap_or(0.3) * v3::clamp(1.0 - pr.path_len / 9.0, 0.0, 1.0) * pr.carry > 0.05) { continue; }
        let eye = eye_of(g, oc); if seg_blocked(g, eye, at) { continue; }
        if let Some(a) = crate::seats::order_of(g, og) {
            let A = &g.tab.orders_assignment[a];
            if let (true, Some(ad), false, Some(aw)) = (A.arrived, A.door, A.spare.unwrap_or(false), A.watch) {
                if ad != dr && flat(aw, g.dr[ad].frame_centre) < 1.6 {
                    let turn = m::abs(v3::wrap_angle(v3::yaw_to(op, at) - g.ch[oc].aim_yaw));
                    if turn < post_turn { post_turn = turn; post = Some(o); post_at = Some(at); }
                    continue;
                }
            }
        }
        g.tab.roomfight_op[o].door_by = Some(OpDoorBy { by: w, t: e.time, at, door: dr }); free += 1;
    }
    if free == 0 { if let (Some(o), Some(at)) = (post, post_at) { g.tab.roomfight_op[o].door_by = Some(OpDoorBy { by: w, t: e.time, at, door: dr }); } }
}

/// every man as this module found him.
pub fn room_fight_stop(g: &mut crate::game::Game) {
    crate::pressure::press_stop(g); crate::laststand::last_stand_stop(g);
    let mut i = 0;
    while i < g.fight.ops.len() {
        let o = g.fight.ops[i]; i += 1;
        room_fight_restore(g, o); room_fight_hand_back(g, o);
    }
    for d in g.fight.defs.clone() {
        let (gd, mode, pinned0) = { let D = &g.tab.roomfight_def[d]; (D.gd, D.mode, D.pinned0) };
        if mode == DefMode::Move && g.gd[gd].script.is_some() && !is_dead(g, gd) { crate::squad::end_guard_script(g, gd, Some(crate::squad::EndGuardScriptState::Alert)); }
        if mode == DefMode::Free && !is_dead(g, gd) { crate::control::set_hold(g, gd, true, "roomfight"); }
        crate::control::set_pinned(g, gd, pinned0, "roomfight");
    }
    for d in g.fight.defs.clone() { let gd = g.tab.roomfight_def[d].gd; g.gd[gd].skill = None; }
    for o in g.fight.ops.clone() { let x = g.tab.roomfight_op[o].gd; g.gd[x].skill = None; }
    g.fight.live = false; g.fight.defs = Vec::new(); g.fight.ops = Vec::new(); g.fight.called.clear(); g.fight.down_called.clear(); g.fight.blind_rounds = 0.0; g.fight.hot_t = -100.0; g.fight.reports = Vec::new();
    crate::digin::dig_in_reset(g);
}

/// a fresh staging.
pub fn room_fight_reset(g: &mut crate::game::Game) {
    room_fight_stop(g);
    g.fight.blind_fire = true; g.fight.drill_said = false; g.fight.log.clear(); g.fight.stats = RoomFightStats::default();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_ignores_height() {
        assert_eq!(flat([0.0, 9.0, 0.0], [3.0, -4.0, 4.0]).to_bits(), 5.0f64.to_bits());
    }

    fn man(g: &mut Game, state: crate::game::GuardState) -> GuardId {
        let mut m = crate::game::Guard::new(CharId(0), String::new(), Some(crate::teams::Team::Operative), RouteId(0), 0, LightId(0), JsSet::new(), FirearmId(0), [0.0; 3], 0.0, GameTalkId(0));
        m.state = state;
        g.gd.push(m)
    }

    fn op(g: &mut Game, gd: GuardId) -> RoomfightOpId {
        let mut o = crate::roomfight::Op::default();
        o.gd = gd;
        let id = g.tab.roomfight_op.push(o);
        g.fight.ops.push(id);
        id
    }

    /// A row of `ops` is an ID now: each question reads the row of the man asked about, and a man with no row answers no.
    #[test]
    fn the_questions_find_the_mans_own_row() {
        let mut g = Game::blank();
        g.time = 100.0;
        let (a, b, h, none) = (man(&mut g, crate::game::GuardState::Patrol), man(&mut g, crate::game::GuardState::Patrol), man(&mut g, crate::game::GuardState::Patrol), man(&mut g, crate::game::GuardState::Patrol));
        let (oa, ob) = (op(&mut g, a), op(&mut g, b));
        g.tab.roomfight_op[ob].tgt = Some(h);
        g.tab.roomfight_op[ob].block_t = 1.5;
        g.tab.roomfight_op[oa].block_t = 9.0;
        g.tab.roomfight_op[oa].lost_at = 99.5;
        assert!(!room_fight_engaged(&mut g, a) && room_fight_engaged(&mut g, b) && !room_fight_engaged(&mut g, none));
        // (no man, no count: his blockT is not read once he has no target)
        assert_eq!((room_fight_no_shot(&mut g, a), room_fight_no_shot(&mut g, b), room_fight_no_shot(&mut g, none)), (0.0, 1.5, 0.0));
        assert!(room_fight_busy(&mut g, a, 1.0) && !room_fight_busy(&mut g, a, 0.5) && !room_fight_busy(&mut g, none, 1e9));
    }

    /// Seen by a fresh meter or, off his script, by his own eyes on that very man; never by a dead teammate; a dead man is no contact.
    #[test]
    fn who_sees_a_man() {
        let mut g = Game::blank();
        g.time = 100.0;
        let (by_meter, stale, by_eyes, dead) = (man(&mut g, crate::game::GuardState::Patrol), man(&mut g, crate::game::GuardState::Patrol), man(&mut g, crate::game::GuardState::Patrol), man(&mut g, crate::game::GuardState::Dead));
        let (h, gone) = (man(&mut g, crate::game::GuardState::Patrol), man(&mut g, crate::game::GuardState::Dead));
        let (om, os, _oe, od) = (op(&mut g, by_meter), op(&mut g, stale), op(&mut g, by_eyes), op(&mut g, dead));
        let fresh = crate::roomfight::Meter { m: 1.0, seen_t: 99.9, glass_t: None };
        g.tab.roomfight_op[om].meters.set(h, fresh);
        g.tab.roomfight_op[os].meters.set(h, crate::roomfight::Meter { seen_t: 99.0, ..fresh });
        g.tab.roomfight_op[od].meters.set(h, fresh);
        crate::mind::foe(&mut g, by_eyes, Some(crate::game::Foe::Guard(h)), "roomfight");
        g.gd[by_eyes].last_seen_t = 99.9;
        crate::mind::foe(&mut g, stale, Some(crate::game::Foe::Player), "roomfight");
        g.gd[stale].last_seen_t = 99.9;
        assert_eq!(room_fight_seen_by(&mut g, h), vec![by_meter, by_eyes]);
        assert!(room_fight_team_sees(&mut g, h) && !room_fight_team_sees(&mut g, gone));
        assert!(room_fight_contact(&mut g, by_meter) && !room_fight_contact(&mut g, stale));
        g.gd[h].state = crate::game::GuardState::Dead;
        assert!(!room_fight_contact(&mut g, by_meter));
    }

    /// The lamp gives him away only burning, in range, inside its cone, with the fight live, and never a dead man's.
    #[test]
    fn a_lamp_gives_its_carrier_away_inside_its_cone() {
        let mut g = Game::blank();
        g.fight.live = true;
        let mut lamp = crate::render_lights::RtLight::default();
        lamp.enabled = true; lamp.intensity = 1.0; lamp.pos = [0.0, 0.0, 0.0]; lamp.dir = [0.0, 0.0, 1.0]; lamp.range = 10.0; lamp.outer_deg = 30.0;
        let l = g.tab.light.push(lamp);
        let eye = g.ch.push(crate::character::Character::blank());
        g.ch[eye].bones.set(crate::character::Bone::Head, [0.0, 0.0, 5.0]);
        let o = man(&mut g, crate::game::GuardState::Patrol);
        g.gd[o].flashlight = l;
        let lit = g.k.roomfight.lamp;
        assert_eq!(lamp_glare(&mut g, eye, o), lit);
        g.ch[eye].bones.set(crate::character::Bone::Head, [5.0, 0.0, 1.0]);   // (outside the cone)
        assert_eq!(lamp_glare(&mut g, eye, o), 0.0);
        g.ch[eye].bones.set(crate::character::Bone::Head, [0.0, 0.0, 11.0]);   // (past its range)
        assert_eq!(lamp_glare(&mut g, eye, o), 0.0);
        g.ch[eye].bones.set(crate::character::Bone::Head, [0.0, 0.0, 5.0]);
        g.tab.light[l].enabled = false;
        assert_eq!(lamp_glare(&mut g, eye, o), 0.0);
        g.tab.light[l].enabled = true;
        g.tab.light[l].intensity = 0.0;
        assert_eq!(lamp_glare(&mut g, eye, o), 0.0);
        g.tab.light[l].intensity = 1.0;
        g.gd[o].state = crate::game::GuardState::Dead;
        assert_eq!(lamp_glare(&mut g, eye, o), 0.0);
        g.gd[o].state = crate::game::GuardState::Patrol;
        g.fight.live = false;
        assert_eq!(lamp_glare(&mut g, eye, o), 0.0);
    }

    /// The doubles are the TypeScript's own: closeLane under bun with EXACT on, point blank 2.
    #[test]
    fn the_lane_at_arms_length_has_the_typescripts_bits() {
        let rows: [(f64, f64, f64, u64); 5] = [
            (0.95, 0.6, 3.3, 0x3fe970a3d70a3d70),
            (0.45, 1.2, 1.5, 0x3fd3333333333333),
            (0.45, 1.2, 2.5, 0x3fdccccccccccccd),
            (0.95, -0.2, 7.1, 0x3fd3333333333333),
            (0.2, 0.37, 11.3, 0x3fc999999999999a),
        ];
        for (clear, t, l, bits) in rows {
            assert_eq!(lane(clear, t, l, 2.0).to_bits(), bits);
        }
    }
}
