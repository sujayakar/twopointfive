//! A HELD ROOM'S WAYS IN, dealt between the men who hold it together by the dealing their like on the move are dealt openings by (`deal.rs`): one way a man, two on
//! one they have been heard at, what each man has heard at a way weighing it for him. Asked of a group, whichever side it is on: the men of one that is drilled, who are at their posts in the
//! same room and roused. Anybody else, and a man left over, picks for himself (`on_his_own`).
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::JsMap;
use crate::m;
use crate::roomfight::{def_of, Entry};
use crate::v3::V3;
use alloc::{vec, vec::Vec};

/// they have been heard at a way: this much of a shot just now
const HEARD: f64 = 0.3;
/// a way his eye has no line to, or that a man of his is in the line of, weighs this much of itself to him
const NO_LINE: f64 = 0.6;

/// Four times a second: the ways into every room a drilled group holds.
pub fn posts_tick(g: &mut Game) {
    if g.time < g.cover.posts_at { return; }
    g.cover.posts_at = g.time + g.k.cover.deal_secs;
    posts_deal(g);
}

/// … dealt now.
pub fn posts_deal(g: &mut Game) {
    let mut dealt: JsMap<GuardId, PostWay> = JsMap::new();
    if g.fight.live {
        for id in g.groups.ids().collect::<Vec<_>>() {
            if !crate::group::drilled(g, id) { continue; }
            crate::group::with(g, id, |g| for (room, men) in rooms_held(g) { deal_room(g, room, &men, &mut dealt); });
        }
    }
    g.cover.posts = dealt;
}

/// The way into his room this man was dealt, by its place in the room's list of them.
pub fn way_dealt(g: &Game, gd: GuardId) -> Option<usize> {
    let p = g.cover.posts.get(gd)?;
    if g.tab.roomfight_def[def_of(g, gd)?].room != Some(p.room) { return None; }
    g.tab.roomfight_room[p.room].entries.iter().position(|e| e.door == p.door)
}

/// The way into his room this man has: the one he was dealt, or is getting ready for or dug in on, or watches from his post.
fn way_he_has(g: &Game, d: RoomfightDefId) -> Option<usize> {
    let row = &g.tab.roomfight_def[d];
    let dealt = way_dealt(g, row.gd).map(|i| i as f64);
    let way = dealt.or_else(|| crate::digin::dig_in_way(g, d)).unwrap_or(if crate::roomfight::at_post(g, d) { row.watch } else { -1.0 });
    if way >= 0.0 { Some(way as usize) } else { None }
}

/// How many men of this room but him have that way.
pub fn on_way(g: &Game, room: RoomfightRoomId, way: usize, but: RoomfightDefId) -> f64 {
    let on = |&&x: &&RoomfightDefId| x != but && g.tab.roomfight_def[x].room == Some(room) && !crate::condition::down(g, g.gd[g.tab.roomfight_def[x].gd].ch) && way_he_has(g, x) == Some(way);
    g.fight.defs.iter().filter(on).count() as f64
}

/// What he has heard at a way into his room, 0 to 3.
fn heard(g: &Game, d: RoomfightDefId, way: usize) -> f64 { g.tab.roomfight_def[d].ev.get(way).copied().filter(|v| *v > 0.0).unwrap_or(0.0) }

/// How well he is placed for a way: whole with a line to it that no man of his is in.
fn placed(g: &Game, d: RoomfightDefId, e: &Entry, mates: &[V3]) -> f64 {
    let c = g.gd[g.tab.roomfight_def[d].gd].ch;
    let (p, eye, at) = (g.ch[c].pos, g.ch[c].bone_or(crate::character::Bone::Head, 1.65), [e.at[0] - e.out[0] * 0.5, 1.2, e.at[2] - e.out[1] * 0.5]);
    let shut = crate::scene_collision::segment_blocked(g, eye, at, None, None, None) || mates.iter().any(|&b| in_lane(b, p, at, g.k.cover.lane));
    if shut { NO_LINE } else { 1.0 }
}

/// THE WAY A MAN PICKS FOR HIMSELF: where he has heard most, less for every mate who has it, the one he has for choice; none below `fear_min`.
pub fn on_his_own(g: &Game, d: RoomfightDefId) -> Option<usize> {
    let (room, watch) = { let row = &g.tab.roomfight_def[d]; (row.room?, row.watch) };
    let k = &g.k.roomfight;
    let (mut best, mut most) = (None, k.fear_min);
    for (i, e) in g.tab.roomfight_room[room].entries.iter().enumerate() {
        if heard(g, d, i) <= 0.02 { continue; }
        let worth = heard(g, d, i) * placed(g, d, e, &[]) - k.crowd * on_way(g, room, i, d) + (if watch == i as f64 { k.sticky } else { 0.0 });
        if worth > most { most = worth; best = Some(i); }
    }
    best
}

/// The ways of his room nobody but him has.
pub fn left_open(g: &Game, d: RoomfightDefId) -> Vec<f64> {
    let Some(room) = g.tab.roomfight_def[d].room else { return Vec::new() };
    let mine = way_he_has(g, d);
    (0..g.tab.roomfight_room[room].entries.len()).filter(|&i| Some(i) != mine && on_way(g, room, i, d) == 0.0).map(|i| i as f64).collect()
}

/// The rooms this pass's group holds, each with its men there who are roused and themselves.
fn rooms_held(g: &Game) -> Vec<(RoomfightRoomId, Vec<RoomfightDefId>)> {
    use crate::owner::by;
    let mut out: Vec<(RoomfightRoomId, Vec<RoomfightDefId>)> = Vec::new();
    for &d in &g.fight.defs {
        let (gd, room) = { let row = &g.tab.roomfight_def[d]; (row.gd, row.room) };
        let Some(room) = room else { continue };
        let man = &g.gd[gd];
        if crate::condition::down(g, man.ch) || !crate::group::has(g, g.group, gd) || !crate::roomfight::at_post(g, d) || man.state == crate::game::GuardState::Patrol { continue; }
        if man.held.is_some() || man.script.is_some() || crate::owner::has(g, gd, by::MORALE | by::RUNNER) { continue; }
        match out.iter_mut().find(|x| x.0 == room) { Some(x) => x.1.push(d), None => out.push((room, vec![d])) }
    }
    out
}

/// One room's ways, one a man, and two on one that somebody of them has heard them at.
fn deal_room(g: &mut Game, room: RoomfightRoomId, men: &[RoomfightDefId], dealt: &mut JsMap<GuardId, PostWay>) {
    let k = g.k.cover;
    let entries = g.tab.roomfight_room[room].entries.clone();
    let openings = cover_ready(g);
    let of_door = |g: &Game, door: DoorId| openings.iter().copied().find(|&o| g.tab.cover_opening[o].door == Some(door));
    let held = |g: &Game, way: usize| g.fight.defs.iter().any(|&x| !men.contains(&x) && g.tab.roomfight_def[x].room == Some(room) && !crate::condition::down(g, g.gd[g.tab.roomfight_def[x].gd].ch) && way_he_has(g, x) == Some(way));
    let mut rows: Vec<(f64, usize, LiveOpening)> = Vec::new();
    for (i, e) in entries.iter().enumerate() {
        let Some(o) = of_door(g, e.door) else { continue };
        let leaf = &g.dr[e.door].leaf;
        let (shut, held) = (!leaf.off && m::abs(leaf.angle) < 0.5, held(g, i));
        let w = (if shut { k.shut_k } else { 1.0 }) * (if held { k.held_k } else { 1.0 });
        let can = men.iter().map(|&d| g.tab.roomfight_def[d].gd).collect();
        let first = LiveOpening { o, w, can, edge: Vec::new(), at: JsMap::new(), by: None, edge_by: None, sam: false, held };
        let most = men.iter().map(|&d| heard(g, d, i)).fold(0.0, m::max);
        let reason = 1.0 + k.heard_k * m::min(1.0, most);
        if most > HEARD { rows.push((w * k.second_k * reason, i, LiveOpening { w: w * k.second_k, ..first.clone() })); }
        rows.push((w * reason, i, first));
    }
    crate::js::sort_by(&mut rows, |a, b| b.0 - a.0);
    let ways: Vec<usize> = rows.iter().map(|x| x.1).collect();
    let mut h = Hand { live: rows.into_iter().map(|x| x.2).collect(), got: JsMap::new() };
    // (a gun on it that is not given yet, if it has one)
    let row_of = |h: &Hand, way: usize| { let mut on = (0..ways.len()).filter(|&i| ways[i] == way); on.clone().find(|&i| h.live[i].by.is_none()).or_else(|| on.next()) };
    // a man with one of them in his eyes, or blind, has what he has
    for &d in men {
        let gd = g.tab.roomfight_def[d].gd;
        if g.time - g.gd[gd].last_seen_t >= k.contact_secs && !crate::control::man::blind(g, gd) { continue; }
        if let Some(i) = way_he_has(g, d).and_then(|way| row_of(&h, way)) { give(&mut h, gd, i); }
    }
    // a man keeps what he was just dealt
    for &d in men {
        let gd = g.tab.roomfight_def[d].gd;
        let Some(p) = g.cover.posts.get(gd).copied().filter(|p| p.room == room && g.time - p.t0 < k.keep_secs) else { continue };
        if h.got.has(gd) { continue; }
        if let Some(i) = h.live.iter().position(|l| l.by.is_none() && g.tab.cover_opening[l.o].door == Some(p.door)) { give(&mut h, gd, i); }
    }
    let hands: Vec<GuardId> = men.iter().map(|&d| g.tab.roomfight_def[d].gd).filter(|&gd| !h.got.has(gd)).collect();
    let spots: Vec<V3> = men.iter().map(|&d| g.pos_of(g.tab.roomfight_def[d].gd)).collect();
    best_dealing(&mut h, &hands, |live, pool, gd| {
        let d = def_of(g, gd).unwrap();
        let mates: Vec<V3> = spots.iter().copied().filter(|&q| q != g.pos_of(gd)).collect();
        pool.iter().map(|&i| {
            let way = ways[i];
            let has = way_he_has(g, d) == Some(way);
            // (a man dug in on it has his spot for it)
            let placed = if crate::digin::dig_in_way(g, d) == Some(way as f64) { 1.0 } else { placed(g, d, &entries[way], &mates) };
            worth(g, gd, entries[way].at, 10.0 * live[i].w * placed * (1.0 + k.heard_k * m::min(1.0, heard(g, d, way))), has)
        }).collect()
    });
    for &d in men {
        let gd = g.tab.roomfight_def[d].gd;
        let Some(&i) = h.got.get(gd) else { continue };
        let door = entries[ways[i]].door;
        let t0 = g.cover.posts.get(gd).filter(|p| p.room == room && p.door == door).map_or(g.time, |p| p.t0);
        dealt.set(gd, PostWay { room, door, t0 });
    }
}
