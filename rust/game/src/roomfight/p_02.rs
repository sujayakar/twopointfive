//! src/game/roomfight.ts lines 433 to 891
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- small readers
fn def_of(g: &Game, gd: GuardId) -> Option<RoomfightDefId> { g.fight.defs.iter().copied().find(|&d| g.tab.roomfight_def[d].gd == gd) }
fn op_id(g: &Game, gd: GuardId) -> Option<RoomfightOpId> { g.fight.ops.iter().copied().find(|&o| g.tab.roomfight_op[o].gd == gd) }
/// `FIGHT_SKILL.hostile`
fn hostile_skill() -> FightSkill { fight_skill().get(String::from("hostile")).copied().unwrap() }
/// `g.col.segmentBlocked(a, b)`
fn seg_blocked(g: &mut Game, a: V3, b: V3) -> bool { crate::scene_collision::segment_blocked(g, a, b, None, None, None) }
fn entries_of(g: &Game, room: Option<RoomfightRoomId>) -> Vec<Entry> { match room { Some(r) => g.tab.roomfight_room[r].entries.clone(), None => Vec::new() } }
fn pos_of(g: &Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }
fn is_dead(g: &Game, gd: GuardId) -> bool { g.gd[gd].state == crate::game::GuardState::Dead }
fn door_name(g: &Game, d: DoorId) -> String { g.dr[d].def.name.clone() }
/// a man off `g.guards` by his body's number (`g.guards.find(x => x.char.id === e.who)`)
fn guard_by_char_id(g: &Game, who: Option<f64>) -> Option<GuardId> { let w = who?; g.guards.iter().copied().find(|&x| g.ch[g.gd[x].ch].id == w) }
/// `D.ev[i]`: a room can gain entries after the row was dealt (a merge), and JavaScript reads past the end as undefined: NaN compares and sums the same
fn ev_at(g: &Game, d: RoomfightDefId, i: usize) -> f64 { g.tab.roomfight_def[d].ev.get(i).copied().unwrap_or(f64::NAN) }
/// `D.ev[i] = v`, past the end as JavaScript writes it: the holes read back as undefined
fn ev_set(g: &mut Game, d: RoomfightDefId, i: usize, v: f64) { let ev = &mut g.tab.roomfight_def[d].ev; while ev.len() <= i { ev.push(f64::NAN); } ev[i] = v; }
/// `gd.lastKnown === D.pt`: one object in the TypeScript, two copies here. The flag says this module last wrote pt as the fix; nobody else's whole write
/// of the fix can clear it (they hold a Guard, not a Def), so a fix that no longer has pt's bits is somebody else's, and the flag is put right on sight (notes.md Traps)
fn fix_is_pt(g: &mut Game, d: RoomfightDefId) -> bool {
    let D = &mut g.tab.roomfight_def[d];
    if D.fix_is_pt && g.gd[D.gd].last_known != Some(D.pt) { D.fix_is_pt = false; }
    D.fix_is_pt
}
/// D.pt moved in place: a fix that is that object moves with it
fn move_pt(g: &mut Game, d: RoomfightDefId, to: V3) { let same = fix_is_pt(g, d); g.tab.roomfight_def[d].pt = to; if same { let gd = g.tab.roomfight_def[d].gd; g.gd[gd].last_known = Some(to); } }
/// `gd.lastKnown = D.pt`: a whole write of the fix only if it was not that object already
fn fix_to_pt(g: &mut Game, d: RoomfightDefId) {
    let own = fix_is_pt(g, d); let D = &mut g.tab.roomfight_def[d]; D.fix_is_pt = true; let (gd, pt) = (D.gd, D.pt);
    g.gd[gd].last_known = Some(pt); if !own { g.gd[gd].fix_stamp += 1; }
}
/// `gd.lastKnown = at`, a point that is not D.pt: a whole write of the fix
fn fix_to(g: &mut Game, d: RoomfightDefId, at: V3) { let gd = g.tab.roomfight_def[d].gd; g.tab.roomfight_def[d].fix_is_pt = false; g.gd[gd].last_known = Some(at); g.gd[gd].fix_stamp += 1; }
/// `D.room.entries[e]?.door.def.name`, for e the row's number or -1
fn entry_name(g: &Game, room: Option<RoomfightRoomId>, e: f64) -> Option<String> {
    let r = room?; if !(e >= 0.0) { return None; }
    g.tab.roomfight_room[r].entries.get(e as usize).map(|x| door_name(g, x.door))
}
fn digs_in(g: &mut Game, gd: GuardId) -> bool { g.k.digin.on || crate::enemysquads::squads_live(g) || crate::laststand::last_stand_man(g, gd) }

// ---------------------------------------------------------------- what the other parts call
/// the sight gain, with the hunting man's x2.2 for a scripted man.   (roomfight.ts rateOn)
pub fn room_fight_rate_on(g: &mut crate::game::Game, viewer: GuardId, target: GuardId, dist: f64, known: Option<bool>) -> f64 {
    let known = known.unwrap_or(false);
    let r = crate::guards::sight_gain_on(g, viewer, target, dist);
    let st = g.gd[viewer].state;
    if st != crate::game::GuardState::Alert && st != crate::game::GuardState::Search && (known || g.time - room_fight_hot_of(g, viewer) < (if g.gd[viewer].team == crate::teams::Team::Operative { g.k.roomfight.team_hot } else { 10.0 })) { r * 2.2 } else { r }
}
/// is the floor hot in this teammate's ears.   (roomfight.ts hotFor)
pub fn room_fight_hot_for(g: &mut crate::game::Game, gd: GuardId) -> bool { g.time - room_fight_hot_of(g, gd) < g.k.roomfight.team_hot }
/// (roomfight.ts hotOf)
pub fn room_fight_hot_of(g: &mut crate::game::Game, gd: GuardId) -> f64 { match op_id(g, gd) { Some(o) => m::max(g.tab.roomfight_op[o].hot_t, g.fight.hot_t), None => g.fight.hot_t } }
/// where a defender is watching, which entry that is, his mode.   (roomfight.ts watchOf)
pub fn room_fight_watch_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::roomfight::RoomFightWatchOf> {
    let d = def_of(g, gd)?; let D = &g.tab.roomfight_def[d];
    let glancing = g.time < D.glance_until;
    Some(RoomFightWatchOf { at: D.pt, door: entry_name(g, D.room, if glancing { D.glance_to } else { D.watch }), settled: entry_name(g, D.room, D.watch), glancing, mode: D.mode })
}
/// (roomfight.ts opOf)
pub fn room_fight_op_of(g: &mut crate::game::Game, gd: GuardId) -> Option<RoomfightOpId> { op_id(g, gd) }
/// this teammate's feet are held this frame.   (roomfight.ts short)
pub fn room_fight_short(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if !g.fight.live { return false; }
    match op_id(g, gd) { Some(o) => { let s = g.tab.roomfight_op[o].short_t; g.time - s >= 0.0 && g.time - s < 0.25 } None => false }
}
/// (roomfight.ts summary)
pub fn room_fight_summary(g: &mut crate::game::Game) -> String {
    if !g.fight.live { return String::from("room fight idle"); }
    let mut men: Vec<String> = Vec::new();
    for d in g.fight.defs.clone() {
        let gd = g.tab.roomfight_def[d].gd; if is_dead(g, gd) { continue; }
        let door = room_fight_watch_of(g, gd).and_then(|w| w.door).unwrap_or_else(|| String::from("front"));
        men.push(alloc::format!("{} {}\u{2192}{}", g.gd[gd].callsign, g.tab.roomfight_def[d].mode.as_str(), door));
    }
    let o = g.fight.ops.iter().filter(|&&x| !is_dead(g, g.tab.roomfight_op[x].gd)).count(); let s = g.fight.stats;
    let list = if men.is_empty() { String::from("none") } else { men.join(", ") };
    alloc::format!("room fight \u{b7} team {} up ({} rounds, {} held for a friend) \u{b7} defenders {} up \u{2014} {} \u{b7} fell back {} \u{b7} cover {} \u{b7} rallied {} \u{b7} surprised {}", o, s.op_rounds, s.held_for_friend, men.len(), list, s.fell_back, s.took_cover, s.rallied, s.surprised)
}

// ---------------------------------------------------------------- the frame
/// Once a frame from Game.update.   (roomfight.ts tick)
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
    room_fight_smoke(g);
    for (k, d) in g.fight.defs.clone().into_iter().enumerate() {
        let gd = g.tab.roomfight_def[d].gd;
        if !is_dead(g, gd) { room_fight_tick_def(g, d, dt, k as f64); }
    }
    let mut i = 0;
    while i < g.fight.ops.len() { let o = g.fight.ops[i]; i += 1; room_fight_tick_op(g, o, dt); }
    crate::pressure::press_tick(g, dt);
    crate::laststand::last_stand_tick(g);
    crate::buddy::buddy_tick(g);
}
/// an enemy's footfall: heard by a teammate as any man hears a step (roomfight.ts:480)
fn hear_step(g: &mut Game, key: u32, e: crate::game::GameEvent) {
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
/// a door noise under an enemy's hand, kept for cue() (roomfight.ts:493)
fn hear_door(g: &mut Game, key: u32, e: crate::game::GameEvent) {
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
/// every man as this module found him.   (roomfight.ts stop)
pub fn room_fight_stop(g: &mut crate::game::Game) {
    crate::pressure::press_stop(g); crate::buddy::buddy_stop(g); crate::laststand::last_stand_stop(g);
    let mut i = 0;
    while i < g.fight.ops.len() {
        let o = g.fight.ops[i]; i += 1;
        room_fight_restore(g, o); room_fight_hand_back(g, o);
    }
    for d in g.fight.defs.clone() {
        let (gd, mode, pinned0) = { let D = &g.tab.roomfight_def[d]; (D.gd, D.mode, D.pinned0) };
        if mode == DefMode::Move && g.gd[gd].script.is_some() && !is_dead(g, gd) { crate::squad::end_guard_script(g, gd, Some(crate::squad::EndGuardScriptState::Alert)); }
        if mode == DefMode::Free && !is_dead(g, gd) { g.gd[gd].hold = true; }
        g.gd[gd].pinned = pinned0;
    }
    for d in g.fight.defs.clone() { let gd = g.tab.roomfight_def[d].gd; g.gd[gd].skill = None; }
    for o in g.fight.ops.clone() { let x = g.tab.roomfight_op[o].gd; g.gd[x].skill = None; }
    for k in g.fight.trans.keys() { crate::sensors::sensors_forget(g, &k); }
    g.fight.live = false; g.fight.defs = Vec::new(); g.fight.ops = Vec::new(); g.fight.trans.clear(); g.fight.called.clear(); g.fight.down_called.clear(); g.fight.blind_rounds = 0.0; g.fight.hot_t = -100.0; g.fight.reports = Vec::new();
    crate::digin::dig_in_reset(g);
}
/// a fresh staging.   (roomfight.ts reset)
pub fn room_fight_reset(g: &mut crate::game::Game) {
    room_fight_stop(g);
    g.fight.blind_fire = true; g.fight.drill_said = false; g.fight.log.clear(); g.fight.stats = RoomFightStats::zeroed();
}
/// the smoke between every operative and every defender in reach of each other.   (roomfight.ts smoke)
pub fn room_fight_smoke(g: &mut crate::game::Game) {
    let mut pairs: Vec<(GuardId, GuardId)> = Vec::new();
    for i in 0..g.fight.ops.len() {
        let og = g.tab.roomfight_op[g.fight.ops[i]].gd; if is_dead(g, og) { continue; }
        for &d in &g.fight.defs { let dg = g.tab.roomfight_def[d].gd; if !is_dead(g, dg) && flat(pos_of(g, og), pos_of(g, dg)) < 18.0 { pairs.push((og, dg)); } }
    }
    if pairs.is_empty() { for k in g.fight.trans.keys() { crate::sensors::sensors_forget(g, &k); } g.fight.trans.clear(); return; }
    let mut live: JsSet<String> = JsSet::default();
    let n = pairs.len() as f64;
    for (i, &(a, b)) in pairs.iter().enumerate() {
        let (ac, bc) = (g.gd[a].ch, g.gd[b].ch);
        let key = pair_key(g.ch[ac].id, g.ch[bc].id); live.add(key.clone());
        let ask = js::rem(js::rem(i as f64 - g.fight.rr, n) + n, n) < g.k.roomfight.pairs;
        let (eye, chest) = (eye_of(g, ac), chest_of(g, bc));
        let v = crate::sensors::sensors_segment(g, &key, eye, chest, Some(ask));
        g.fight.trans.set(key, match v { Some(x) if x.is_finite() => x, _ => 1.0 });
    }
    g.fight.rr = js::rem(g.fight.rr + g.k.roomfight.pairs, n);
    for k in g.fight.trans.keys() { if !live.has(k.clone()) { g.fight.trans.delete(k.clone()); crate::sensors::sensors_forget(g, &k); } }
}

// ---------------------------------------------------------------- rooms
fn cell_free(g: &mut Game, dc: &JsMap<crate::Num, DoorId>, x: f64, z: f64) -> bool {
    let (w, h) = (g.col.nav.w, g.col.nav.h);
    if !(x >= 0.0 && z >= 0.0 && x < w && z < h) { return false; }
    let i = crate::scene_collision::nav_grid_idx(g, x, z);
    g.col.nav.blocked[i as usize] != 1 && !dc.has(crate::Num(i)) && !g.destr.walls.shell_gaps.has(crate::Num(i))
}
/// the room a point stands in: the nav cells reachable without crossing a doorway, and the doors on its edge (cached).   (roomfight.ts roomAt)
pub fn room_fight_room_at(g: &mut crate::game::Game, p: V3) -> Option<RoomfightRoomId> {
    let cs = g.col.nav.cell; let (w, h) = (g.col.nav.w, g.col.nav.h);
    let dc = crate::guards::door_cells(g);
    let (mut sx, mut sz) = (m::floor(p[0] / cs), m::floor(p[2] / cs));
    if !cell_free(g, &dc, sx, sz) {
        let mut found = false;
        'look: for r in 1..=3i32 { for dz in -r..=r { for dx in -r..=r {
            if cell_free(g, &dc, sx + dx as f64, sz + dz as f64) { sx += dx as f64; sz += dz as f64; found = true; break 'look; }
        } } }
        if !found { return None; }
    }
    let start = crate::scene_collision::nav_grid_idx(g, sx, sz);
    if let Some(&hit) = g.fight.room_of_cell.get(crate::Num(start)) { return Some(hit); }
    let mut cells: JsSet<crate::Num> = JsSet::default(); cells.add(crate::Num(start));
    let mut q: Vec<f64> = vec![start];
    let mut touched: JsMap<DoorId, f64> = JsMap::default();
    let size = (w * h) as usize;
    if g.fight.fill_mark.len() != size { g.fight.fill_mark = vec![0u8; size]; }
    g.fight.fill_mark[start as usize] = 1;
    let mut merge: Option<RoomfightRoomId> = None;
    let (dxs, dzs) = (room_dx(), room_dz());
    let mut qi = 0;
    while qi < q.len() && (cells.len() as f64) < ROOM_MAX_CELLS && merge.is_none() {
        let c = q[qi]; qi += 1;
        let x = js::rem(c, w); let z = js::to_i32(c / w) as f64;
        for k in 0..4 {
            let (nx, nz) = (x + dxs[k], z + dzs[k]);
            if nx < 0.0 || nz < 0.0 || nx >= w || nz >= h { continue; }
            let n = crate::scene_collision::nav_grid_idx(g, nx, nz);
            if let Some(&d) = dc.get(crate::Num(n)) { if !touched.has(d) { touched.set(d, c); } continue; }
            if g.col.nav.blocked[n as usize] == 1 || g.fight.fill_mark[n as usize] == 1 || g.destr.walls.shell_gaps.has(crate::Num(n)) { continue; }
            if let Some(&other) = g.fight.room_of_cell.get(crate::Num(n)) { merge = Some(other); break; }
            cells.add(crate::Num(n)); q.push(n); g.fight.fill_mark[n as usize] = 1;
        }
    }
    for &c in &q { g.fight.fill_mark[c as usize] = 0; }
    let room = match merge {
        Some(r) => { for c in cells.keys() { g.tab.roomfight_room[r].cells.add(c); } r }
        None => { let id = g.fight.rooms.len() as f64; g.tab.roomfight_room.push(Room { id, cells: cells.clone(), entries: Vec::new() }) }
    };
    let met: Vec<(DoorId, f64)> = touched.iter().map(|(d, &c)| (d, c)).collect();
    for (d, c) in met {
        if g.tab.roomfight_room[room].entries.iter().any(|e| e.door == d) { continue; }
        let f = crate::guards::frame_of(g, d);
        let cx = (js::rem(c, w) + 0.5) * cs; let cz = (js::to_i32(c / w) as f64 + 0.5) * cs;
        let side = if (cx - f.hx) * f.nx + (cz - f.hz) * f.nz >= 0.0 { 1.0 } else { -1.0 };
        let fc = g.dr[d].frame_centre;
        g.tab.roomfight_room[room].entries.push(Entry { door: d, at: [fc[0], 0.0, fc[2]], out: [-side * f.nx, -side * f.nz] });
    }
    if merge.is_none() { g.fight.rooms.push(room); }
    for c in cells.keys() { g.fight.room_of_cell.set(c, room); }
    Some(room)
}
/// (roomfight.ts inRoom)
pub fn room_fight_in_room(g: &mut crate::game::Game, r: Option<RoomfightRoomId>, p: V3) -> bool {
    let r = match r { Some(r) => r, None => return false };
    let cell = g.col.nav.cell;
    let i = crate::scene_collision::nav_grid_idx(g, m::floor(p[0] / cell), m::floor(p[2] / cell));
    g.tab.roomfight_room[r].cells.has(crate::Num(i))
}
/// (roomfight.ts freeNear)
pub fn room_fight_free_near(g: &mut crate::game::Game, p: V3) -> Option<V3> {
    if !crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) { return Some([p[0], 0.0, p[2]]); }
    for r in [0.5, 1.0, 1.5] { for a in 0..8 {
        let a = a as f64;
        let x = p[0] + m::cos(a * m::PI / 4.0) * r; let z = p[2] + m::sin(a * m::PI / 4.0) * r;
        if !crate::scene_collision::nav_grid_is_blocked(g, x, z) { return Some([x, 0.0, z]); }
    } }
    None
}

// ---------------------------------------------------------------- defenders
/// (roomfight.ts adopt)
pub fn room_fight_adopt(g: &mut crate::game::Game, gd: GuardId) -> RoomfightDefId {
    let c = g.gd[gd].ch; let s = crate::enemysquads::squad_skill(g, hostile_skill());
    let f = crate::character::character_forward(g, c); let p = g.ch[c].pos;
    let room = room_fight_room_at(g, p);
    let what = match room {
        Some(r) => { let names: Vec<String> = g.tab.roomfight_room[r].entries.iter().map(|e| door_name(g, e.door)).collect(); alloc::format!("adopt room {} entries {}", g.tab.roomfight_room[r].id, names.join("/")) }
        None => String::from("adopt room none"),
    };
    room_fight_note(g, Some(gd), &what);
    let ev = match room { Some(r) => vec![0.0; g.tab.roomfight_room[r].entries.len()], None => Vec::new() };
    let front: V3 = [p[0] + f[0] * 6.0, 0.0, p[2] + f[2] * 6.0];
    let glance_next = g.time + pick(g, s.hold);
    let nerve = pick(g, s.nerve);
    let H = g.gd[gd].witness.heard;
    let row = Def {
        gd, pinned0: g.gd[gd].pinned, station: p, home: front, room, staged: None, ev, watch: -1.0, want: -1.0, want_at: 0.0,
        glance_to: -1.0, glance_until: -1.0, glance_next, pt: front,
        heard_t: H.last_t, cnt: DefCnt { shots: H.shots, kick: H.kick, bang: H.bang, small: H.small }, noise_at: None, noise_t: -100.0, noise_out: false,
        last_state: String::from(g.gd[gd].state.as_str()), last_seen: g.gd[gd].last_seen_t, fix_keep: -1.0, pre: false, nerve, lost_at: 0.0, moved_at: -100.0, rally_at: f64::NAN, rally_go: false,
        mode: DefMode::Hold, move_kind: DefMoveKind::Cover, move_t: 0.0, came_by: None, stall_at: p, stall_t: 0.0, free_at: 0.0,
        burst: DefBurst { until: -1.0, n: 0.0, next: 0.0, broke: false, daze: 0.0 }, bolt_until: -1.0, bolt_from: None, pt_stamp: None, fix_is_pt: false, pt_stamp_2: None,
    };
    g.tab.roomfight_def.push(row)
}
/// (roomfight.ts tickDef)
pub fn room_fight_tick_def(g: &mut crate::game::Game, d: RoomfightDefId, dt: f64, k: f64) {
    let gd = g.tab.roomfight_def[d].gd; let base = hostile_skill();
    g.gd[gd].skill = Some(crate::game::GuardSkill { settle_k: base.settle_k, spread_k: base.spread_k });
    if g.gd[gd].held.is_some() { room_fight_blind(g, d, k); return; }
    if crate::morale::morale_owns(g, gd) { return; }
    if crate::runner::runner_owns(g, gd) { return; }
    if digs_in(g, gd) {
        let keeps = g.level.scenarios.as_ref().and_then(|s| s.room_clear.as_ref()).and_then(|r| r.dig_in) != Some(false);
        if keeps && !crate::lanes::lane_throws(g, gd) { crate::digin::dig_in_tick(g, d, dt); }
    }
    let mode = g.tab.roomfight_def[d].mode;
    if mode == DefMode::Move { room_fight_tick_move(g, d, dt); return; }
    if mode == DefMode::Free { room_fight_tick_free(g, d); return; }
    if g.gd[gd].script.is_some() && crate::director::director_lesson_runs(g) { g.tab.roomfight_def[d].staged = Some(true); return; }
    if g.tab.roomfight_def[d].staged == Some(true) {
        g.tab.roomfight_def[d].staged = Some(false); g.gd[gd].hold = true;
        if v3::dist_xz(pos_of(g, gd), g.tab.roomfight_def[d].station) > 1.5 {
            g.fight.defs.retain(|&x| x != d);
            room_fight_note(g, Some(gd), "left on a new post by the lesson's script"); return;
        }
    }
    if g.gd[gd].script.is_some() {
        let st = match g.gd[gd].state { crate::game::GuardState::Dead | crate::game::GuardState::Patrol => crate::squad::EndGuardScriptState::Patrol, crate::game::GuardState::Suspicious => crate::squad::EndGuardScriptState::Suspicious, crate::game::GuardState::Alert => crate::squad::EndGuardScriptState::Alert, crate::game::GuardState::Search => crate::squad::EndGuardScriptState::Search };
        crate::squad::end_guard_script(g, gd, Some(st));
    }
    if !g.gd[gd].hold {
        g.gd[gd].skill = None; g.fight.defs.retain(|&x| x != d);
        room_fight_note(g, Some(gd), "let off his post by somebody else: not this module's man"); return;
    }
    g.gd[gd].pinned = true;
    let dazzled = g.time < g.gd[gd].dazzled_until;
    room_fight_evidence(g, d, dt);
    if !dazzled { room_fight_choose_watch(g, d); room_fight_apply_watch(g, d, dt); }
    room_fight_reactions(g, d);
    room_fight_blind(g, d, k);
    if !dazzled { room_fight_lost(g, d); room_fight_rally(g, d); }
}
/// what his own senses have put at each entry since last frame.   (roomfight.ts evidence)
pub fn room_fight_evidence(g: &mut crate::game::Game, d: RoomfightDefId, dt: f64) {
    let skill = hostile_skill();
    let gd = g.tab.roomfight_def[d].gd; let H = g.gd[gd].witness.heard;
    let room = g.tab.roomfight_def[d].room; let ent = entries_of(g, room);
    let k = m::exp(-dt / g.k.roomfight.evidence_secs * m::log(3.0));
    for v in g.tab.roomfight_def[d].ev.iter_mut() { *v *= k; }
    if H.last_t > g.tab.roomfight_def[d].heard_t {
        let cnt = g.tab.roomfight_def[d].cnt;
        let loud = H.shots > cnt.shots || H.kick > cnt.kick || H.bang > cnt.bang;
        let fix = if g.gd[gd].last_known.is_some() && !fix_is_pt(g, d) { g.gd[gd].last_known } else { None };
        let at = if loud { fix.or(g.tab.roomfight_def[d].noise_at) } else if H.small > cnt.small && H.small_where.is_some() { H.small_where } else { fix };
        if let Some(at) = at {
            let e = room_fight_entry_toward(g, d, at);
            if e >= 0.0 { let i = e as usize; let v = m::min(3.0, ev_at(g, d, i) + (if loud { 1.0 } else { 0.45 })); ev_set(g, d, i, v); }
            if loud {
                let out = !room_fight_in_room(g, room, at); let now = g.time; let D = &mut g.tab.roomfight_def[d];
                D.noise_at = Some(at); D.noise_t = now; D.noise_out = out; D.glance_until = -1.0;
            }
            if H.kick > cnt.kick && e >= 0.0 && flat(ent[e as usize].at, at) < 2.5 && g.time >= g.gd[gd].dazzled_until {
                let door = ent[e as usize].door;
                if skill.startle[1] > 0.0 && !(digs_in(g, gd) && crate::digin::dig_in_braced(g, gd, door)) {
                    let p = pick(g, skill.startle);
                    let t = p * (if room_fight_drilled(g) { g.k.drilled.shaken } else { 1.0 });
                    g.gd[gd].react_t = m::max(g.gd[gd].react_t, t); g.fight.stats.startled += 1.0;
                    let what = alloc::format!("flinches: {} booted in", door_name(g, door)); room_fight_note(g, Some(gd), &what);
                }
            }
            let what = alloc::format!("heard {} \u{2192} {}", if loud { "loud" } else { "small" }, if e >= 0.0 { door_name(g, ent[e as usize].door) } else { String::from("nowhere") });
            room_fight_note(g, Some(gd), &what);
        }
        let D = &mut g.tab.roomfight_def[d];
        D.heard_t = H.last_t; D.cnt = DefCnt { shots: H.shots, kick: H.kick, bang: H.bang, small: H.small };
    }
    if g.time - g.gd[gd].last_seen_t < 0.3 && !fix_is_pt(g, d) {
        if let Some(lk) = g.gd[gd].last_known {
            let e = room_fight_entry_toward(g, d, lk);
            if e >= 0.0 { let i = e as usize; let v = m::min(3.0, ev_at(g, d, i) + 1.2 * dt); ev_set(g, d, i, v); }
        }
    }
}
/// a man of his own side has told him where they are. False = no post of this file's.   (roomfight.ts told)
pub fn room_fight_told(g: &mut crate::game::Game, gd: GuardId, at: V3) -> bool {
    let d = match def_of(g, gd) { Some(d) => d, None => return false };
    let e = room_fight_entry_toward(g, d, at);
    if e >= 0.0 { let i = e as usize; let v = m::min(3.0, ev_at(g, d, i) + 1.0); ev_set(g, d, i, v); }
    let room = g.tab.roomfight_def[d].room; let out = !room_fight_in_room(g, room, at); let now = g.time;
    let D = &mut g.tab.roomfight_def[d];
    D.noise_at = Some(at); D.noise_t = now; D.noise_out = out; D.glance_until = -1.0;
    true
}
/// the entry nearest a point, or -1.   (roomfight.ts entryToward)
pub fn room_fight_entry_toward(g: &mut crate::game::Game, d: RoomfightDefId, at: V3) -> f64 {
    let (mut best, mut bd) = (-1.0, f64::INFINITY);
    for (i, e) in entries_of(g, g.tab.roomfight_def[d].room).iter().enumerate() { let q = flat(e.at, at); if q < bd { bd = q; best = i as f64; } }
    best
}
/// (roomfight.ts chooseWatch)
pub fn room_fight_choose_watch(g: &mut crate::game::Game, d: RoomfightDefId) {
    let gd = g.tab.roomfight_def[d].gd; let s = crate::enemysquads::squad_skill(g, hostile_skill());
    let room = g.tab.roomfight_def[d].room; let ent = entries_of(g, room);
    let (mut best, mut bs) = (-1.0, g.k.roomfight.fear_min);
    let c = g.gd[gd].ch; let eye = eye_of(g, c);
    for (i, e) in ent.iter().enumerate() {
        let n = i as f64;
        if ev_at(g, d, i) <= 0.02 { continue; }
        let view = if seg_blocked(g, eye, [e.at[0] - e.out[0] * 0.5, 1.2, e.at[2] - e.out[1] * 0.5]) { 0.6 } else { 1.0 };
        let mut mates = 0.0;
        for &x in &g.fight.defs { let X = &g.tab.roomfight_def[x]; if x != d && !is_dead(g, X.gd) && X.room == room && X.mode == DefMode::Hold && X.watch == n { mates += 1.0; } }
        let sc = ev_at(g, d, i) * view - g.k.roomfight.crowd * mates + (if n == g.tab.roomfight_def[d].watch { g.k.roomfight.sticky } else { 0.0 });
        if sc > bs { bs = sc; best = n; }
    }
    if best != g.tab.roomfight_def[d].want {
        let at = g.time + pick(g, s.shift); let D = &mut g.tab.roomfight_def[d];
        D.want = best; D.want_at = at;
    }
    if g.tab.roomfight_def[d].want != g.tab.roomfight_def[d].watch && g.time >= g.tab.roomfight_def[d].want_at {
        let w = g.tab.roomfight_def[d].want; g.tab.roomfight_def[d].watch = w;
        let what = alloc::format!("watch \u{2192} {}", if w >= 0.0 { door_name(g, ent[w as usize].door) } else { String::from("front") });
        room_fight_note(g, Some(gd), &what);
    }
    if !ent.is_empty() && g.time >= g.tab.roomfight_def[d].glance_next && g.time - g.gd[gd].last_seen_t > 2.0 && g.time - g.tab.roomfight_def[d].noise_t > 3.0 && !crate::enemysquads::squad_drill(g).map_or(false, |k| k.drill_angles) {
        let watch = g.tab.roomfight_def[d].watch;
        let mut others: Vec<f64> = (0..ent.len()).map(|i| i as f64).filter(|&i| i != watch).collect();
        if watch >= 0.0 { others.push(-1.0); }
        let r = crate::math_dice::sim_random(g);
        let to = others.get(m::floor(r * others.len() as f64) as usize).copied().unwrap_or(-1.0);
        let until = g.time + pick(g, s.glance);
        let next = until + pick(g, s.hold);
        let D = &mut g.tab.roomfight_def[d];
        D.glance_to = to; D.glance_until = until; D.glance_next = next;
    }
}
/// what he watches becomes where he faces and the fix his gun covers.   (roomfight.ts applyWatch)
pub fn room_fight_apply_watch(g: &mut crate::game::Game, d: RoomfightDefId, dt: f64) {
    let skill = hostile_skill();
    let gd = g.tab.roomfight_def[d].gd; let c = g.gd[gd].ch;
    let room = g.tab.roomfight_def[d].room; let ent = entries_of(g, room);
    let e = if g.time < g.tab.roomfight_def[d].glance_until { g.tab.roomfight_def[d].glance_to } else { g.tab.roomfight_def[d].watch };
    let tgt = if e >= 0.0 && (e as usize) < ent.len() { ent[e as usize].at } else { g.tab.roomfight_def[d].home };
    move_pt(g, d, [tgt[0], 0.0, tgt[2]]);
    // (each look writes into D.pt in place: lent as a local and put back, the fix carried with it if it is that object)
    if digs_in(g, gd) { let mut p = g.tab.roomfight_def[d].pt; crate::digin::dig_in_look(g, gd, &mut p); move_pt(g, d, p); }
    if g.rescue.active { let mut p = g.tab.roomfight_def[d].pt; crate::rescue::rescue_crew_look(g, gd, &mut p); move_pt(g, d, p); }
    { let mut p = g.tab.roomfight_def[d].pt; crate::netquiet::net_quiet_look(g, gd, &mut p); move_pt(g, d, p); }
    { let mut p = g.tab.roomfight_def[d].pt; crate::wallreact::wall_react_look_2(g, gd, &mut p); move_pt(g, d, p); }
    let pt = g.tab.roomfight_def[d].pt;
    if g.gd[gd].state == crate::game::GuardState::Patrol {
        let want = crate::character::character_want_body(g, c);
        g.ch[c].body_yaw = v3::approach_angle(want, v3::yaw_to(g.ch[c].pos, pt), skill.turn * dt);
        return;
    }
    if g.time - g.gd[gd].last_seen_t < 0.6 { return; }
    if !fix_is_pt(g, d) && g.time - g.gd[gd].sight_t < 1.0 {
        if let Some(lk) = g.gd[gd].last_known { if room_fight_in_room(g, room, lk) { g.tab.roomfight_def[d].fix_keep = g.time + 3.0; } }
    }
    if g.time >= g.tab.roomfight_def[d].fix_keep { fix_to_pt(g, d); }
}
/// the untrained man's beats.   (roomfight.ts reactions)
pub fn room_fight_reactions(g: &mut crate::game::Game, d: RoomfightDefId) {
    let gd = g.tab.roomfight_def[d].gd; let c = g.gd[gd].ch; let s = crate::enemysquads::squad_skill(g, hostile_skill());
    if g.gd[gd].state == crate::game::GuardState::Alert && g.tab.roomfight_def[d].last_state != "alert" { let p = pick(g, s.react); g.gd[gd].react_t = m::max(g.gd[gd].react_t, p); }
    let fc = crate::teams::foe_char(g, gd);
    if let Some(fc) = fc { if g.gd[gd].last_seen_t > g.tab.roomfight_def[d].last_seen + 1.5 && g.time - g.gd[gd].last_seen_t < 0.1 {
        let (cp, fp, pt) = (g.ch[c].pos, g.ch[fc].pos, g.tab.roomfight_def[d].pt);
        let off = m::abs(v3::wrap_angle(v3::yaw_to(cp, fp) - v3::yaw_to(cp, pt)));
        let B = g.tab.roomfight_def[d].burst;
        let dazed = g.time < B.until + B.daze;
        let caught = dazed || off > s.sector_deg * v3::DEG; let lim = g.k.roomfight.preaim_deg * v3::DEG;
        let a = pick(g, s.react) * (if caught { 1.0 } else { s.ready });
        let b = if caught { pick(g, s.surprise) } else { 0.0 };
        g.gd[gd].react_t = m::max(g.gd[gd].react_t, a + b);
        g.tab.roomfight_def[d].pre = !dazed && g.gd[gd].state != crate::game::GuardState::Patrol && off < lim && m::abs(v3::wrap_angle(v3::yaw_to(cp, fp) - g.ch[c].aim_yaw)) < lim;
        if caught {
            g.fight.stats.surprised += 1.0;
            let what = if dazed { String::from("still dazed: the man is a surprise") } else { alloc::format!("surprised {}\u{b0} off his watch", js::to_fixed(off / v3::DEG, 0)) };
            room_fight_note(g, Some(gd), &what);
        }
    } }
    if g.tab.roomfight_def[d].pre {
        if g.time - g.gd[gd].last_seen_t > 1.0 { g.tab.roomfight_def[d].pre = false; }
        else if g.gd[gd].state == crate::game::GuardState::Alert && g.gd[gd].react_t <= 0.0 {
            g.gd[gd].aim.track = m::max(g.gd[gd].aim.track, s.preaim); g.gd[gd].fire_cd = m::min(g.gd[gd].fire_cd, 0.0); g.tab.roomfight_def[d].pre = false;
        }
    }
    let (seen, st) = (g.gd[gd].last_seen_t, g.gd[gd].state);
    let D = &mut g.tab.roomfight_def[d];
    D.last_seen = seen; D.last_state = String::from(st.as_str());
}
/// a dazzled defender empties a burst down what he was watching, or ducks.   (roomfight.ts blind)
pub fn room_fight_blind(g: &mut crate::game::Game, d: RoomfightDefId, k: f64) {
    let h = g.tab.roomfight_def[d].gd; let s = crate::enemysquads::squad_skill(g, hostile_skill());
    if g.time >= g.gd[h].dazzled_until { return; }
    let bl = crate::nightbreach::blind();
    if g.gd[h].dazzled_until > g.tab.roomfight_def[d].burst.until + 1e-6 {
        let (until, next) = (g.gd[h].dazzled_until, g.time + bl.after + k * bl.stagger);
        let broke = crate::math_dice::sim_random(g) < s.bang_break;
        let daze = if s.daze[1] > 0.0 { let p = pick(g, s.daze); p * (if room_fight_drilled(g) { g.k.drilled.shaken } else { 1.0 }) } else { 0.0 };
        let B = &mut g.tab.roomfight_def[d].burst;
        B.until = until; B.n = 0.0; B.next = next; B.broke = broke; B.daze = daze;
        if broke { g.fight.stats.broke += 1.0; room_fight_note(g, Some(h), "breaks under the bang"); }
    }
    if g.gd[h].held.is_none() { fix_to_pt(g, d); }
    if g.tab.roomfight_def[d].burst.broke {
        if g.tune.suppression { g.gd[h].sup.down_t = m::max(g.gd[h].sup.down_t, 0.2); }
        return;
    }
    if !g.fight.blind_fire || g.tab.roomfight_def[d].burst.n >= bl.rounds || g.time < g.tab.roomfight_def[d].burst.next { return; }
    g.tab.roomfight_def[d].burst.n += 1.0;
    let next = g.time + crate::gunroles::gun_blind_gap(g, h, bl.every);
    g.tab.roomfight_def[d].burst.next = next;
    let c = g.gd[h].ch;
    if g.gd[h].held.is_some() || g.time < g.gd[h].stagger_until || g.ch[c].gun_dir[1] < -bl.low_gun { return; }
    if g.tune.suppression && (g.gd[h].sup.v >= g.k.suppression.pinned || g.gd[h].sup.latched || g.gd[h].sup.down_t > 0.0) { return; }
    let gun = g.gd[h].weapon;
    if !crate::weapons::firearm_fire(g, gun) { return; }
    let at = v3::mad(g.ch[c].muzzle, g.ch[c].gun_dir, 8.0);
    let o = crate::combat::FireOpts { steady: Some(true), spread: Some(bl.spread), blind: Some(true), ..Default::default() };
    crate::combat::fire_weapon(g, c, at, false, Some(&o));
    g.fight.blind_rounds += 1.0;
    if !g.fight.drill_said && room_fight_drilled(g) {
        g.fight.drill_said = true;
        let line = crate::words::wg(g, &"can't see \u{2014} shoot through the doorway!", &"can't see \u{2014} put it down the door!");
        crate::game::game_say(g, h, line, None, None, None);
    }
}
/// (roomfight.ts drilled)
pub fn room_fight_drilled(g: &mut crate::game::Game) -> bool { drilled_on(g) }
/// is the room lost, as far as he can tell.   (roomfight.ts lost)
pub fn room_fight_lost(g: &mut crate::game::Game, d: RoomfightDefId) {
    let gd = g.tab.roomfight_def[d].gd; let c = g.gd[gd].ch;
    if g.gd[gd].state != crate::game::GuardState::Alert || g.time < g.tab.roomfight_def[d].lost_at || g.time - g.tab.roomfight_def[d].moved_at < 6.0 { return; }
    g.tab.roomfight_def[d].lost_at = g.time + 0.3;
    let eye = eye_of(g, c); let (mut dead, mut up) = (0.0, 0.0);
    let room = g.tab.roomfight_def[d].room;
    for x in g.fight.defs.clone() {
        let (xr, xg, xm) = { let X = &g.tab.roomfight_def[x]; (X.room, X.gd, X.mode) };
        if xr != room { continue; }
        if !is_dead(g, xg) { if xm != DefMode::Free { up += 1.0; } continue; }
        let xc = g.gd[xg].ch;
        let b = g.ch[xc].bones.get(crate::character::Bone::Hips).copied().unwrap_or(g.ch[xc].pos);
        if seg_blocked(g, eye, [b[0], 0.3, b[2]]) { continue; }
        if flat(b, g.ch[c].pos) < 5.0 || !g.tune.body_light || crate::guards::body_visibility(g, xg) >= g.k.body_light.lit { dead += 1.0; }
    }
    let sup_v = g.gd[gd].sup.v;
    let pressed = g.time - g.gd[gd].last_seen_t < 4.0 || (g.tune.suppression && sup_v >= g.k.suppression.flinch) || g.time - g.gd[gd].witness.heard.last_shot_t < 2.5;
    let frac = dead / m::max(1.0, dead + up);
    let alone = dead + up <= 1.0 && g.tune.suppression && g.gd[gd].sup.latched && g.gd[gd].sup.pinned_t > 1.2;
    if !pressed || !((dead > 0.0 && frac >= g.tab.roomfight_def[d].nerve) || alone) { return; }
    room_fight_break_off(g, d);
}
/// `p` lies outside the rectangle round every room of the level's structure with open sky over it.   (roomfight.ts outOfBuilding)
pub fn room_fight_out_of_building(g: &mut crate::game::Game, p: V3) -> bool {
    let (mut x0, mut x1, mut z0, mut z1) = (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY);
    {
        let rooms = match g.level.scenarios.as_ref().and_then(|s| s.room_clear.as_ref()) { Some(r) if !r.rooms.is_empty() => &r.rooms, _ => return false };
        for r in rooms { x0 = m::min(x0, r.bounds.x0); x1 = m::max(x1, r.bounds.x1); z0 = m::min(z0, r.bounds.z0); z1 = m::max(z1, r.bounds.z1); }
    }
    if p[0] >= x0 && p[0] <= x1 && p[2] >= z0 && p[2] <= z1 { return false; }
    let mut roofed = 0;
    for (dx, dz) in [(0.0, 0.0), (0.35, 0.0), (-0.35, 0.0), (0.0, 0.35), (0.0, -0.35)] {
        if crate::scene_collision::solid_blocked(g, [p[0] + dx, 1.9, p[2] + dz], [p[0] + dx, 9.0, p[2] + dz], Some(0.02)) { roofed += 1; }
    }
    roofed < 3
}
/// where the trouble is, as far as he knows.   (roomfight.ts threatOf)
pub fn room_fight_threat_of(g: &mut crate::game::Game, d: RoomfightDefId) -> V3 {
    let gd = g.tab.roomfight_def[d].gd;
    if g.time - g.gd[gd].sight_t < 6.0 { return g.gd[gd].sight_pos; }
    if let Some(src) = g.gd[gd].sup.src { if g.time - g.gd[gd].sup.src_t < g.k.suppression.src_memory_secs { return src; } }
    let D = &g.tab.roomfight_def[d];
    let (mut bi, mut bv) = (-1i32, 0.3);
    for (i, &v) in D.ev.iter().enumerate() { if v > bv { bv = v; bi = i as i32; } }
    match D.room { Some(r) if bi >= 0 => g.tab.roomfight_room[r].entries[bi as usize].at, _ => D.pt }
}
/// a man on a post caught unaware and alone goes as a man whose nerve has gone. None = no post of this file's.   (roomfight.ts bolt)
pub fn room_fight_bolt(g: &mut crate::game::Game, gd: GuardId, clear_secs: f64) -> Option<bool> {
    let d = def_of(g, gd)?;
    if g.tab.roomfight_def[d].mode != DefMode::Hold || is_dead(g, gd) || g.gd[gd].held.is_some() { return Some(false); }
    let (moved, out, pic) = (g.tab.roomfight_def[d].moved_at, g.fight.stats.fall_back_out, g.stats.picture.moved);
    room_fight_break_off(g, d);
    if g.tab.roomfight_def[d].mode == DefMode::Hold {
        g.tab.roomfight_def[d].moved_at = moved; g.fight.stats.fall_back_out = out; g.stats.picture.moved = pic;
        return Some(false);
    }
    let (until, foe) = (g.time + clear_secs, g.gd[gd].foe);
    let D = &mut g.tab.roomfight_def[d];
    D.bolt_until = until; D.bolt_from = foe;
    Some(true)
}
/// a believed man beside a straight leg of the walk (roomfight.ts:807 nearLeg)
fn near_leg(a: V3, b: V3, k: V3) -> bool {
    let (dx, dz) = (b[0] - a[0], b[2] - a[2]); let d2 = dx * dx + dz * dz;
    let t = if d2 > 0.0 { m::max(0.0, m::min(1.0, ((k[0] - a[0]) * dx + (k[2] - a[2]) * dz) / d2)) } else { 0.0 };
    m::hyp2(k[0] - a[0] - dx * t, k[2] - a[2] - dz * t) < 2.0
}
/// the walk to a spot: its length, or None if it will not do (roomfight.ts:808 leg)
fn leg_of(g: &mut Game, from: V3, threat: V3, known: &[V3], spot: V3, mind: bool) -> Option<f64> {
    let path = crate::scene_collision::nav_grid_find_path(g, from, spot, None, None)?; if path.is_empty() { return None; }
    let (mut l, mut q, mut toward) = (0.0, from, false);
    for &w in &path {
        if mind && known.iter().any(|&k| near_leg(q, w, k)) { toward = true; }
        l += flat(q, w); q = w;
        if flat(w, threat) < 2.0 { toward = true; }
    }
    if l > 18.0 || toward || flat(q, spot) > 0.8 { None } else { Some(l) }
}
/// the closure exposureAsker hands back (picture.ts:315): each idea's weight dealt once, heaviest first; `maybe` dealt the first time an idea needs it
fn ask_exposure(g: &mut Game, asker: &mut crate::picture::ExposureAsker, p: V3) -> f64 {
    let crate::picture::ExposureAsker::Ideas { h, rows } = asker;
    let h = *h;
    g.stats.picture.asks += 1;
    let mut e = 0.0;
    for X in rows.iter_mut() {
        let (I, w) = (X.0, X.1);
        if e >= w { break; }
        if !I.lost && crate::picture::line_to(g, I.at, p, h) { e = w; continue; }
        let k = w * g.k.picture.maybe_k; if e >= k { continue; }
        if X.2.is_none() { X.2 = Some(crate::picture::maybe_of(g, I)); }
        let mut hit = false;
        for &q in X.2.as_ref().unwrap() { if crate::picture::line_to(g, q, p, h) { hit = true; break; } }
        if hit || (I.lost && crate::picture::line_to(g, I.at, p, h)) { e = k; }
    }
    e
}
#[derive(Clone, Copy)]
struct Way { spot: V3, door: DoorId, l: f64, safe: bool }
// (depth, lat) of the spots beyond a door, in the order they are tried (roomfight.ts:818)
const BEYOND: [(f64, f64); 9] = [(3.0, 1.6), (3.0, -1.6), (2.4, 1.2), (2.4, -1.2), (1.2, 2.4), (1.2, -2.4), (1.2, 1.7), (1.2, -1.7), (2.6, 0.0)];
/// out of the room by a door nothing has come at; no such door, to cover.   (roomfight.ts breakOff)
pub fn room_fight_break_off(g: &mut crate::game::Game, d: RoomfightDefId) {
    let gd = g.tab.roomfight_def[d].gd; let c = g.gd[gd].ch; let threat = room_fight_threat_of(g, d);
    g.tab.roomfight_def[d].moved_at = g.time;
    let stays = crate::laststand::last_stand_owns(g, d);
    let rp = if stays { None } else { crate::enemysquads::squad_rally(g, gd, threat) };
    if let Some(rp) = rp {
        crate::pressure::run(g, d, rp, None, Some(0.3));
        g.fight.stats.fell_back += 1.0;
        room_fight_note(g, Some(gd), "falls back to his squad's rally point");
        crate::game::game_say(g, gd, "fall back \u{2014} back to the others!", None, None, None);
        return;
    }
    let mut asker = crate::picture::exposure_asker(g, gd, None);
    let ask = asker.is_some();
    let known: Vec<V3> = if ask { crate::picture::believed_of(g, gd) } else { Vec::new() };
    let (mut best, mut was): (Option<Way>, Option<Way>) = (None, None);
    let cp = g.ch[c].pos;
    let here = match &mut asker { Some(a) => ask_exposure(g, a, cp), None => 0.0 };
    let room = g.tab.roomfight_def[d].room;
    let ent = if stays || crate::enemysquads::squad_stays_in(g, gd) { Vec::new() } else { entries_of(g, room) };
    for (i, e) in ent.iter().enumerate() {
        if ev_at(g, d, i) > 0.3 || flat(e.at, threat) < 3.0 || g.dr[e.door].def.steel == Some(true) || crate::doors::doors_locked_out(g, e.door, cp, None) { continue; }
        let f = crate::guards::frame_of(g, e.door);
        let (mut spot, mut safe): (Option<V3>, Option<V3>) = (None, None);
        for &(depth, lat) in BEYOND.iter() {
            let p: V3 = [e.at[0] + e.out[0] * depth + f.ux * lat * (1.0 + f.w / 3.2), 0.0, e.at[2] + e.out[1] * depth + f.uz * lat * (1.0 + f.w / 3.2)];
            if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) || room_fight_in_room(g, room, p) { continue; }
            if room_fight_out_of_building(g, p) { if spot.is_none() { g.fight.stats.fall_back_out += 1.0; } continue; }
            if g.fight.defs.iter().any(|&x| x != d && { let xg = g.tab.roomfight_def[x].gd; !is_dead(g, xg) && flat(pos_of(g, xg), p) < 0.9 }) { continue; }
            if spot.is_none() { spot = Some(p); }
            let a = match &mut asker { Some(a) => a, None => break };
            if ask_exposure(g, a, p) <= 0.0 { safe = Some(p); break; }
        }
        let spot = match spot { Some(s) => s, None => continue };
        if let Some(l0) = leg_of(g, cp, threat, &known, spot, false) { if was.map_or(true, |w| l0 < w.l) { was = Some(Way { spot, door: e.door, l: l0, safe: false }); } }
        let a = match &mut asker { Some(a) => a, None => continue };
        let mouth = ask_exposure(g, a, [e.at[0] + e.out[0], 0.0, e.at[2] + e.out[1]]);
        if known.iter().any(|&k| flat(e.at, k) < 3.0) || (mouth > g.k.picture.maybe_k && mouth > here) { continue; }
        let mut sp = safe.unwrap_or(spot); let mut l1 = leg_of(g, cp, threat, &known, sp, true);
        if l1.is_none() && sp != spot { sp = spot; l1 = leg_of(g, cp, threat, &known, sp, true); }
        let hid = safe == Some(sp);
        if let Some(l1) = l1 { if best.map_or(true, |b| (hid && !b.safe) || (hid == b.safe && l1 < b.l)) { best = Some(Way { spot: sp, door: e.door, l: l1, safe: hid }); } }
    }
    if !ask { best = was; } else if best.map(|b| (b.door, b.spot)) != was.map(|w| (w.door, w.spot)) { g.stats.picture.moved += 1; }   // (the same object: the same door's spot, and one door's spots all differ)
    if let Some(b) = best {
        let now_pos = g.ch[c].pos;
        { let D = &mut g.tab.roomfight_def[d]; D.mode = DefMode::Move; D.move_kind = DefMoveKind::Fallback; D.came_by = Some(b.door); D.move_t = 0.0; D.stall_at = now_pos; D.stall_t = 0.0; }
        g.gd[gd].pinned = false;
        let speed = g.tune.guard_chase + 0.3;
        crate::squad::order(g, gd, crate::squad::OrderPatch { goal: Some(Some(b.spot)), speed: Some(Some(speed)), face: Some(None), strafe: Some(Some(false)), aim_at: Some(None), upper: Some(Some(crate::character::UpperMode::Aim)), stance: Some(Some(crate::character::Stance::None)), crouch: Some(Some(false)), ..Default::default() });
        g.fight.stats.fell_back += 1.0;
        let what = alloc::format!("falls back through {} to {},{}", door_name(g, b.door), js::to_fixed(b.spot[0], 1), js::to_fixed(b.spot[2], 1));
        room_fight_note(g, Some(gd), &what);
        let line = if crate::math_dice::sim_random(g) < 0.5 { "fall back \u{2014} fall back!" } else { "they're in \u{2014} get out!" };
        crate::game::game_say(g, gd, line, None, None, None);
        return;
    }
    if let Some(p) = crate::suppression::cover_from(g, gd, threat) {
        let now_pos = g.ch[c].pos;
        { let D = &mut g.tab.roomfight_def[d]; D.mode = DefMode::Move; D.move_kind = DefMoveKind::Cover; D.came_by = None; D.move_t = 0.0; D.stall_at = now_pos; D.stall_t = 0.0; }
        g.gd[gd].pinned = false;
        let speed = g.tune.guard_chase;
        crate::squad::order(g, gd, crate::squad::OrderPatch { goal: Some(Some(p)), speed: Some(Some(speed)), face: Some(None), strafe: Some(Some(false)), aim_at: Some(None), upper: Some(Some(crate::character::UpperMode::Aim)), stance: Some(Some(crate::character::Stance::None)), crouch: Some(Some(true)), ..Default::default() });
        g.fight.stats.took_cover += 1.0;
        room_fight_note(g, Some(gd), "takes cover");
        crate::game::game_say(g, gd, "cover \u{2014} get to cover!", None, None, None);
    }
}
/// a defender on the move: an enemy seen close stops him to fight; there, or going nowhere, he plants.   (roomfight.ts tickMove)
pub fn room_fight_tick_move(g: &mut crate::game::Game, d: RoomfightDefId, dt: f64) {
    let gd = g.tab.roomfight_def[d].gd; let c = g.gd[gd].ch; let s = crate::enemysquads::squad_skill(g, hostile_skill());
    let arrived0 = match &g.gd[gd].script { Some(x) => x.arrived.unwrap_or(false), None => { room_fight_plant(g, d, "script gone"); return; } };
    g.tab.roomfight_def[d].move_t += dt;
    crate::guards::hearing_check(g, gd, None, None, None);
    let mut contact: Option<crate::game::Foe> = None;
    if g.time >= g.gd[gd].dazzled_until {
        let from = if g.time < g.tab.roomfight_def[d].bolt_until { g.tab.roomfight_def[d].bolt_from } else { None };
        let man = crate::seats::faced_man(g, gd); let team = g.gd[gd].team; let reach = g.k.roomfight.move_engage_m;
        if !matches!(from, Some(crate::game::Foe::Player)) && crate::teams::is_hostile(g, team, crate::teams::Team::Player) && !g.pl[man].down && !g.player_invisible {
            let view = crate::seats::faced_view(g, gd);
            let seen = crate::seats::with_seat(g, view, &mut |g| { let vis = crate::guards::can_see(g, gd); vis.visible && vis.dist < reach && crate::guards::sight_gain(g, gd, vis.dist) > 0.25 });
            if seen { contact = Some(crate::game::Foe::Player); }
        }
        let mut i = 0;
        while i < g.guards.len() && contact.is_none() {
            let o = g.guards[i]; i += 1;
            if matches!(from, Some(crate::game::Foe::Guard(x)) if x == o) || is_dead(g, o) { continue; }
            let ot = g.gd[o].team;
            if !crate::teams::is_hostile(g, team, ot) || crate::rails::rails_down(g, o) { continue; }
            let oc = g.gd[o].ch; let vis = crate::guards::can_see_char(g, gd, oc, false);
            if vis.visible && vis.dist < reach && crate::guards::sight_gain_on(g, gd, o, vis.dist) > 0.25 { contact = Some(crate::game::Foe::Guard(o)); }
        }
    }
    if let Some(ct) = contact {
        let fc = match ct { crate::game::Foe::Player => { let man = crate::seats::faced_man(g, gd); g.pl[man].ch } crate::game::Foe::Guard(o) => g.gd[o].ch };
        g.gd[gd].foe = Some(ct);
        crate::aim::aim_state_clear(&mut g.gd[gd].aim);
        room_fight_plant(g, d, "contact");
        g.gd[gd].awareness = 1.0;
        g.gd[gd].react_t = pick(g, s.react);
        let (fp, now) = (g.ch[fc].pos, g.time);
        fix_to(g, d, fp);
        let G = &mut g.gd[gd];
        G.sight_pos = fp; G.sight_t = now; G.last_seen_t = now;
        return;
    }
    let cp = g.ch[c].pos;
    let D = &mut g.tab.roomfight_def[d];
    if flat(cp, D.stall_at) > 0.4 { D.stall_at = cp; D.stall_t = 0.0; } else { D.stall_t += dt; }
    let (stall, moved) = (D.stall_t, D.move_t);
    let arrived = g.gd[gd].script.as_ref().map_or(arrived0, |x| x.arrived.unwrap_or(false));
    if arrived || stall > 1.5 || moved > 10.0 { room_fight_plant(g, d, if arrived { "there" } else { "going nowhere" }); }
}
/// a free hunter with his eyes on a man is planted where he stands with that man still his.   (roomfight.ts standOn)
pub fn room_fight_stand_on(g: &mut crate::game::Game, d: RoomfightDefId, set: bool) {
    let gd = g.tab.roomfight_def[d].gd;
    let fix = g.gd[gd].last_known; let own = fix_is_pt(g, d);   // (asked before plant: in the TypeScript that object IS D.pt, whatever plant does to it)
    { let D = &mut g.tab.roomfight_def[d]; D.move_kind = DefMoveKind::Fallback; D.came_by = None; D.rally_at = f64::NAN; }
    room_fight_plant(g, d, "stands and fights");
    if let Some(f) = fix { if !own {
        fix_to(g, d, f);
        if set { let D = &mut g.tab.roomfight_def[d]; D.pt[0] = f[0]; D.pt[2] = f[2]; }
    } }
    let (st, seen) = (g.gd[gd].state, g.gd[gd].last_seen_t);
    let D = &mut g.tab.roomfight_def[d];
    D.last_state = String::from(st.as_str()); if !set { D.last_seen = seen; }
}
/// a man whose way lies down a covered lane stops short of it, his gun on `look`.   (roomfight.ts laneStop)
pub fn room_fight_lane_stop(g: &mut crate::game::Game, gd: GuardId, look: V3) -> bool {
    let d = match def_of(g, gd) { Some(d) => d, None => return false };
    if is_dead(g, gd) || g.gd[gd].held.is_some() { return false; }
    { let D = &mut g.tab.roomfight_def[d]; D.move_kind = DefMoveKind::Fallback; D.came_by = None; D.rally_at = f64::NAN; }
    room_fight_plant(g, d, "a covered lane ahead");
    if let Some(r) = g.tab.roomfight_def[d].room { let n = g.tab.roomfight_room[r].entries.len(); g.tab.roomfight_def[d].ev = vec![0.0; n]; }
    let pt: V3 = [look[0], 0.0, look[2]];
    { let D = &mut g.tab.roomfight_def[d]; D.watch = -1.0; D.want = -1.0; D.home = pt; }
    move_pt(g, d, pt); fix_to_pt(g, d);
    true
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
}
