//! src/game/rescueflush.ts lines 1 to 214
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Distance on the floor.   (rescueflush.ts flat)
pub fn flat(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }

/// A new one: the field initialisers; the constructor only keeps `g` and `R`.   (rescueflush.ts RescueFlush.constructor)
pub fn rescue_flush_new(g: &mut crate::game::Game) -> crate::rescueflush::RescueFlush { RescueFlush::blank() }

/// Back to nothing having happened.   (rescueflush.ts RescueFlush.reset)
pub fn rescue_flush_reset(g: &mut crate::game::Game) {
    let f = &mut g.rescue.flush;
    f.fell_at = m::NAN; f.next_at = m::NAN; f.gone_at = m::NAN; f.count = 0.0; f.sent.clear(); f.barred.clear();
}

/// The rule is up on this run (RESCUEFLUSH.on is settled true).   (rescueflush.ts RescueFlush.live)
pub fn rescue_flush_live(g: &mut crate::game::Game) -> bool {
    g.rescue.active && g.rescue.result.is_none() && g.ai_enabled && crate::roomfight::room_fight_on(g)
}

/// The living hostages, in the list's order.   (rescueflush.ts RescueFlush.alive)
pub fn rescue_flush_alive(g: &mut crate::game::Game) -> Vec<GuardId> {
    g.rescue.hostages.iter().copied().filter(|&h| g.gd[h].state != crate::game::GuardState::Dead).collect()
}

/// He came for her: his wait is `killSecs`.   (rescueflush.ts RescueFlush.came)
pub fn rescue_flush_came(g: &mut crate::game::Game, gd: GuardId) -> bool { g.rescue.flush.sent.has(gd) }

/// HIS LINE ON HER: the distance, or Infinity when out of reach, blocked or smoked.   (rescueflush.ts RescueFlush.line)
pub fn rescue_flush_line(g: &mut crate::game::Game, by: GuardId, v: GuardId) -> f64 {
    let (cb, cv) = (g.gd[by].ch, g.gd[v].ch);
    let (pb, pv) = (g.ch[cb].pos, g.ch[cv].pos);
    let d = flat(pv, pb);
    if d >= g.k.rescueflush.reach_m { return f64::INFINITY; }
    let e: V3 = [pb[0], 1.5, pb[2]];
    let c = match g.ch[cv].bones.get(crate::character::Bone::Chest) { Some(p) => *p, None => v3::add(pv, [0.0, 0.6, 0.0]) };
    if crate::scene_collision::shot_blocked(g, e, c, None, None) { return f64::INFINITY; }
    let key = format!("rescueS{}_{}", g.ch[cb].id, g.ch[cv].id);
    let t = crate::sensors::sensors_segment(g, &key, e, c, None);
    match t { Some(t) if t < g.k.rescueflush.smoke_k => f64::INFINITY, _ => d }
}

/// His nearest line on any of `alive`, or Infinity when he cannot hold a gun on anybody. (the closure `has` of ts:94)
fn has(g: &mut Game, h: GuardId, alive: &[GuardId]) -> f64 {
    if g.gd[h].state == crate::game::GuardState::Dead || g.gd[h].held.is_some() || g.time < g.gd[h].dazzled_until
        || crate::runner::runner_owns(g, h) || crate::morale::morale_owns(g, h) { return f64::INFINITY; }
    let mut best = f64::INFINITY;
    for &v in alive { let d = rescue_flush_line(g, h, v); best = m::min(best, d); }
    best
}

/// THE GUN ON HER: the man of theirs with a clear line to a living hostage, nearest to one; `keep` keeps it while it holds for him.   (rescueflush.ts RescueFlush.holder)
pub fn rescue_flush_holder(g: &mut crate::game::Game, keep: Option<GuardId>) -> Option<GuardId> {
    if !rescue_flush_live(g) || g.rescue.flush.fell_at.is_nan() { return None; }
    let alive = rescue_flush_alive(g);
    if alive.is_empty() { return None; }
    if let Some(k) = keep { if has(g, k, &alive) < f64::INFINITY { return Some(k); } }
    let (mut who, mut bd) = (None, f64::INFINITY);
    let hostiles = g.rescue.hostiles.clone();
    for h in hostiles { let d = has(g, h, &alive); if d < bd { bd = d; who = Some(h); } }
    who
}

/// Rescue's bounds as the shape `inBounds` takes: the same four numbers (ts:108, ts:120 and the rest).
fn plan_bounds(g: &mut Game) -> Option<crate::scene_level::BreachPlanBounds> {
    crate::rescue::rescue_bounds(g).map(|r| crate::scene_level::BreachPlanBounds { x0: r.x0, x1: r.x1, z0: r.z0, z1: r.z1 })
}

/// Which of `of` he HAS: those in the room with him, and those on his line once the room has fallen.   (rescueflush.ts RescueFlush.victims)
pub fn rescue_flush_victims(g: &mut crate::game::Game, by: GuardId, of: &[GuardId]) -> Vec<GuardId> {
    let b = plan_bounds(g);
    let here = crate::squad::in_bounds(g.ch[g.gd[by].ch].pos, b, None);
    let far = rescue_flush_live(g) && !g.rescue.flush.fell_at.is_nan();
    let mut out = Vec::new();
    for &v in of {
        if (here && crate::squad::in_bounds(g.ch[g.gd[v].ch].pos, b, None)) || (far && rescue_flush_line(g, by, v) < f64::INFINITY) { out.push(v); }
    }
    out
}

/// The room fight's record of this man, if he has one. (`g.fight.defs.find(d => d.gd === h)`, ts:140, ts:191)
fn fight_def_of(g: &Game, h: GuardId) -> Option<RoomfightDefId> {
    g.fight.defs.iter().copied().find(|&d| g.tab.roomfight_def[d].gd == h)
}

/// May he be sent now? (the closure `free` of ts:136)
fn free(g: &mut Game, h: GuardId, b: Option<crate::scene_level::BreachPlanBounds>, in_room: bool) -> bool {
    if g.gd[h].state == crate::game::GuardState::Dead || g.gd[h].held.is_some() || g.time < g.gd[h].dazzled_until || g.time < g.gd[h].stagger_until
        || crate::runner::runner_owns(g, h) || crate::morale::morale_owns(g, h) || crate::rescuecheck::rescue_check_owns(g, h) { return false; }
    if crate::squad::in_bounds(g.ch[g.gd[h].ch].pos, b, None) && in_room { return false; }
    if g.time - g.gd[h].last_seen_t < g.k.rescueflush.busy_secs || g.gd[h].saw_foe_this_frame { return false; }
    let f = fight_def_of(g, h);
    if let Some(d) = f { if g.tab.roomfight_def[d].mode == crate::roomfight::DefMode::Move { return false; } }
    let Some(s_at) = g.rescue.flush.sent.get(h).map(|s| s.at) else { return true; };
    if g.time - s_at < g.k.rescueflush.again_secs { return false; }
    let st = g.gd[h].state;
    let at_rest = match f { None => true, Some(d) => g.tab.roomfight_def[d].mode == crate::roomfight::DefMode::Hold };
    at_rest || st == crate::game::GuardState::Patrol || st == crate::game::GuardState::Suspicious || st == crate::game::GuardState::Search
}

/// One way a man could go: goal, marks on the way, where he walks now, the door it takes him to. (the map of ts:164)
fn way_of(g: &mut Game, goal: Goal, alive: &[GuardId], room: V3) -> (Goal, Vec<V3>, Option<V3>, Option<V3>) {
    let via = if goal == Goal::Side { crate::rescue::rescue_side_way(g) } else { Vec::new() };
    let at = match via.first() { Some(p) => Some(*p), None => rescue_flush_point_of(g, goal, alive) };
    let door = if goal == Goal::Side { via.last().copied() } else if goal == Goal::Room { Some(room) } else { at };
    (goal, via, at, door)
}

/// Once a frame from rescue.ts tick(), after its defenders() and before its clock().   (rescueflush.ts RescueFlush.tick)
pub fn rescue_flush_tick(g: &mut crate::game::Game) {
    if !rescue_flush_live(g) { return; }
    let b = plan_bounds(g);
    let alive = rescue_flush_alive(g);
    if alive.is_empty() || crate::rescue::rescue_noise_at(g).is_none() { return; }
    if g.rescue.flush.fell_at.is_nan() {
        for c in crate::rescue::rescue_cast(g) {
            if c.in_room && g.gd[c.gd].state != crate::game::GuardState::Dead && g.gd[c.gd].held.is_none() && !crate::runner::runner_fled(g, c.gd) { return; }
        }
        g.rescue.flush.fell_at = g.time;
        g.rescue.flush.next_at = g.time + g.k.rescueflush.after_secs;
        crate::rescue::rescue_note(g, "flush:fell");
    }
    let in_room = alive.iter().any(|&h| crate::squad::in_bounds(g.ch[g.gd[h].ch].pos, b, None));
    // one of them stands in the room and nobody is there to shoot: they have been taken out
    if g.rescue.flush.gone_at.is_nan() && !in_room {
        let by = g.rescue.hostiles.iter().copied().find(|&h| {
            g.gd[h].state != crate::game::GuardState::Dead && g.gd[h].held.is_none() && crate::squad::in_bounds(g.ch[g.gd[h].ch].pos, b, None)
        });
        if let Some(by) = by {
            g.rescue.flush.gone_at = g.time;
            let note = format!("flush:gone:{}", g.gd[by].callsign);
            crate::rescue::rescue_note(g, &note);
            crate::game::game_say(g, by, "the hostages are gone — they're moving them! cut them off!", Some(true), None, None);
            g.rescue.flush.next_at = m::min(g.rescue.flush.next_at, g.time + 1.0);
            for (_, s) in g.rescue.flush.sent.iter_mut() { s.at = -1e9; }
        }
    }
    // a man sent round has reached the side door: on through it, for them (no second word: it is the same errand)
    let mut walk = 0;
    while let Some(h) = g.rescue.flush.sent.next_key(&mut walk) {
        let Some(mark) = g.rescue.flush.sent.get(h).and_then(|s| s.via.first().copied()) else { continue; };
        if g.gd[h].state == crate::game::GuardState::Dead || g.gd[h].held.is_some() || flat(g.ch[g.gd[h].ch].pos, mark) > g.k.rescueflush.at_door_m { continue; }
        if g.time - g.gd[h].last_seen_t < g.k.rescueflush.busy_secs || g.gd[h].saw_foe_this_frame { continue; }
        g.rescue.flush.sent.get_mut(h).unwrap().via.remove(0);
        let next = g.rescue.flush.sent.get(h).unwrap().via.first().copied();
        let to = match next { Some(p) => Some(p), None => rescue_flush_point_of(g, Goal::Room, &alive) };
        let Some(to) = to else { continue; };
        g.gd[h].last_known = Some(v3::copy(to)); g.gd[h].path = Vec::new(); g.gd[h].path_goal = None;
        if g.rescue.flush.sent.get(h).unwrap().via.is_empty() {
            g.rescue.flush.sent.get_mut(h).unwrap().goal = Goal::Room;
            let note = format!("flush:through:{}", g.gd[h].callsign);
            crate::rescue::rescue_note(g, &note);
        }
    }
    if g.time < g.rescue.flush.next_at { return; }
    // the next man: never sent before the men who have been, then the nearest to where he would go
    let room = crate::rescue::rescue_outside_point(g);
    let mut pool = Vec::new();
    for h in g.rescue.hostiles.clone() { if free(g, h, b, in_room) { pool.push(h); } }
    {
        let g = &*g;
        let n_of = |h: GuardId| g.rescue.flush.sent.get(h).map_or(0.0, |s| s.n);
        js::sort_by(&mut pool, |&x, &y| js::or(n_of(x) - n_of(y), flat(g.ch[g.gd[x].ch].pos, room) - flat(g.ch[g.gd[y].ch].pos, room)));
    }
    for h in pool {
        let n = g.rescue.flush.sent.get(h).map_or(0.0, |s| s.n);
        let count = g.rescue.flush.count;
        let a = rescue_flush_goal_of(g, count, in_room);
        let bb = rescue_flush_goal_of(g, count + 1.0, in_room);
        // his turn's way, or the other when the floor outside his turn's door is in the line of guns his side knows of and the other's is not: the lanes only CHOOSE
        let goals = if a == bb { vec![a] } else { vec![a, bb] };
        let mut made = Vec::new();
        for goal in goals { made.push(way_of(g, goal, &alive, room)); }
        let mut ways = Vec::new();
        for w in made {
            let Some(at) = w.2 else { continue; };
            if !crate::lanes::lane_holds(g, h, Some(at)) { ways.push(w); }
        }
        if ways.is_empty() { continue; }
        let turned = ways.len() > 1 && covered(g, h, ways[0].3) && !covered(g, h, ways[1].3);
        let way = &ways[if turned { 1 } else { 0 }];
        if turned && !g.rescue.flush.barred.has(h) {
            g.rescue.flush.barred.add(h);
            let note = format!("flush:lane:{}", g.gd[h].callsign);
            crate::rescue::rescue_note(g, &note);
        }
        rescue_flush_send(g, h, way.0, way.2.unwrap(), n, Some(&way.1));
        g.rescue.flush.next_at = g.time + g.k.rescueflush.gap_secs;
        return;
    }
    g.rescue.flush.next_at = g.time + 1.0;   // nobody free this second: asked again the next
}

/// That door's floor is in the line of guns his side knows of. (the closure `covered` of ts:167)
fn covered(g: &mut Game, h: GuardId, door: Option<V3>) -> bool {
    match door { Some(d) => crate::lanes::lane_covers(g, h, d), None => false }
}

/// Where the k-th word sends a man.   (rescueflush.ts RescueFlush.goalOf)
pub fn rescue_flush_goal_of(g: &mut crate::game::Game, k: f64, in_room: bool) -> crate::rescueflush::Goal {
    if in_room || g.rescue.flush.gone_at.is_nan() {
        return if js::rem(k, 2.0) == 1.0 && crate::rescue::rescue_data(g).map_or(false, |s| s.side.is_some()) { Goal::Side } else { Goal::Room };
    }
    if js::rem(k, 2.0) == 0.0 { Goal::Out } else { Goal::Noise }
}

/// The point a goal means: a hostage in the room or the room's middle, the team's noise, the way out.   (rescueflush.ts RescueFlush.pointOf)
pub fn rescue_flush_point_of(g: &mut crate::game::Game, goal: crate::rescueflush::Goal, alive: &[GuardId]) -> Option<V3> {
    if crate::rescue::rescue_data(g).is_none() { return None; }
    if goal == Goal::Room || goal == Goal::Side {
        let b = plan_bounds(g);
        let v = alive.iter().copied().find(|&h| crate::squad::in_bounds(g.ch[g.gd[h].ch].pos, b, None));
        if let Some(v) = v { let p = g.ch[g.gd[v].ch].pos; return Some([p[0], 0.0, p[2]]); }
        return b.map(|b| [(b.x0 + b.x1) / 2.0, 0.0, (b.z0 + b.z1) / 2.0]);
    }
    if goal == Goal::Noise { if let Some(p) = crate::rescue::rescue_noise_at(g) { return Some(v3::copy(p)); } }
    crate::rescueend::rescue_end_out(g)
}

/// Let go to hunt it, as rescue.ts lets a roamer go to the noise; and his word on the net.   (rescueflush.ts RescueFlush.send)
pub fn rescue_flush_send(g: &mut crate::game::Game, h: GuardId, goal: crate::rescueflush::Goal, at: V3, n: f64, via: Option<&[V3]>) {
    let via: &[V3] = via.unwrap_or(&[]);
    let f = fight_def_of(g, h);
    crate::rescue::rescue_let_go(g, h);
    g.gd[h].hold = false; g.gd[h].pinned = false;
    if let Some(d) = f {
        let time = g.time;
        let r = &mut g.tab.roomfight_def[d];
        r.mode = crate::roomfight::DefMode::Free; r.free_at = time; r.rally_at = m::NAN;
    }
    if g.gd[h].script.is_some() { crate::squad::end_guard_script(g, h, Some(crate::squad::EndGuardScriptState::Alert)); }
    let gd = &mut g.gd[h];
    gd.state = crate::game::GuardState::Alert; gd.awareness = 1.0; gd.react_t = 0.0; gd.last_known = Some(v3::copy(at)); gd.path = Vec::new(); gd.path_goal = None;
    crate::guards::reset_chase(g, h);
    let time = g.time;
    g.rescue.flush.sent.set(h, Sent { at: time, n: n + 1.0, goal, via: via.iter().map(|p| v3::copy(*p)).collect() });
    g.rescue.flush.count += 1.0;
    let note = format!("flush:{}:{}", goal.as_str(), g.gd[h].callsign);
    crate::rescue::rescue_note(g, &note);
    if n > 0.0 { return; }   // (a man's second errand is not announced: one line a man)
    let text: &str = match goal {
        Goal::Room => crate::words::w(g, &"it's gone quiet in there — I'm going in for them!", &"it's gone quiet in there — i'm going in for them!"),
        Goal::Side => {
            let now = crate::words::wg(g, &"going around — I'll take the other door!", &"going round — I'll take the other door!");
            crate::words::w(g, &now, &"going round — i'll take the other door!")
        }
        Goal::Out => "on the way out — they don't leave with the hostages!",
        Goal::Noise => "moving on that noise — keep them boxed in!",
    };
    crate::game::game_say(g, h, text, Some(true), None, None);
}

/// A checkpoint: a clone of Game replaces it (conventions 6); nothing calls it.   (rescueflush.ts RescueFlush.snapshot)
pub fn rescue_flush_snapshot(g: &mut crate::game::Game) -> crate::rescueflush::RescueFlushSnap { todo!("not translated: a clone of Game replaces a checkpoint") }

/// A checkpoint: a clone of Game replaces it (conventions 6); nothing calls it.   (rescueflush.ts RescueFlush.restore)
pub fn rescue_flush_restore(g: &mut crate::game::Game, s: &crate::rescueflush::RescueFlushSnap) { todo!("not translated: a clone of Game replaces a checkpoint") }
