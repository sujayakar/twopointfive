//! src/game/dialogue.ts lines 376 to 817
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- names: rooms, routes, doors, numbers
/// the one key of roomRects / ownRects: a Game has one level, reached at g.level (notes, Decisions)
const LEVEL_KEY: u32 = 0;

fn with_the_room(room: &str) -> String { if room.starts_with("the ") { String::from(room) } else { alloc::format!("the {}", room) } }
fn rect_dist_of(x0: f64, x1: f64, z0: f64, z1: f64, p: V3) -> f64 {
    m::hyp2(m::max_n(&[x0 - p[0], 0.0, p[0] - x1]), m::max_n(&[z0 - p[2], 0.0, p[2] - z1]))
}
fn rect_holds(r: &Rect, p: V3) -> bool { p[0] >= r.x0 && p[0] <= r.x1 && p[2] >= r.z0 && p[2] <= r.z1 }
fn place_of_room(r: &Rect) -> PlaceRect { PlaceRect { id: None, name: r.name.clone(), x0: r.x0, x1: r.x1, z0: r.z0, z1: r.z1 } }
/// a number as `String(n)` says it
fn num_string(n: f64) -> String {
    if n.is_nan() { return String::from("NaN"); }
    if n == f64::INFINITY { return String::from("Infinity"); }
    if n == f64::NEG_INFINITY { return String::from("-Infinity"); }
    if m::floor(n) == n && m::abs(n) < 1e15 { return alloc::format!("{}", n as i64); }
    alloc::format!("{}", n)
}

/// WHERE, for a line on the net: the level's own name for the spot, else roomName.   (dialogue.ts placeName)
pub fn place_name(g: &mut crate::game::Game, p: V3) -> String {
    match own_name(g, p) { Some(n) => n, None => room_name(g, p) }
}
/// the level the game was written on (its three patrol routes)   (dialogue.ts isOffice)
pub fn is_office(g: &mut crate::game::Game) -> bool {
    ["corridor", "cubicles", "lobby_break"].iter().all(|n| g.level.routes.iter().any(|r| g.tab.route[*r].name == *n))
}
/// (dialogue.ts withThe)
pub fn with_the(n: &str) -> String {
    if n.starts_with("the ") || n.as_bytes().last().map_or(false, |c| c.is_ascii_digit()) { String::from(n) } else { alloc::format!("the {}", n) }
}
/// THE LEVEL'S OWN NAMES for its floor, first match wins; null on the office and on a level that names nothing.   (dialogue.ts levelRects)
pub fn level_rects(g: &mut crate::game::Game) -> Option<Vec<crate::dialogue::PlaceRect>> {
    if let Some(r) = g.m.dialogue.own_rects.get(LEVEL_KEY) { return r.clone(); }
    let mut r: Option<Vec<PlaceRect>> = None;
    if !is_office(g) {
        let mut out: Vec<PlaceRect> = Vec::new();
        let level = &g.level;
        let scen = level.scenarios.as_ref();
        if let Some(rc) = scen.and_then(|s| s.room_clear.as_ref()) {
            for x in rc.rooms.iter() { out.push(PlaceRect { name: with_the(&x.name), id: Some(x.id.clone()), x0: x.bounds.x0, x1: x.bounds.x1, z0: x.bounds.z0, z1: x.bounds.z1 }); }
        }
        if let Some(cps) = level.clear_plans.as_ref() {
            for cp in cps.iter() {
                if let (Some(b), Some(room)) = (cp.bounds.as_ref(), cp.room.as_ref().filter(|r| !r.is_empty())) {
                    let name = with_the(room);
                    if !out.iter().any(|x| x.name == name) { out.push(PlaceRect { name, id: None, x0: b.x0, x1: b.x1, z0: b.z0, z1: b.z1 }); }
                }
            }
        }
        if let Some(pl) = scen.and_then(|s| s.places.as_ref()) {
            for a in pl.areas.iter() { out.push(PlaceRect { name: with_the(&a.name), id: None, x0: a.bounds.x0, x1: a.bounds.x1, z0: a.bounds.z0, z1: a.bounds.z1 }); }
        }
        if let Some(q) = scen.and_then(|s| s.quiet.as_ref()) {
            for sp in q.spots.iter() {
                let name = with_the(&sp.place);
                if !out.iter().any(|x| x.name == name) { out.push(PlaceRect { name, id: None, x0: sp.area.x0, x1: sp.area.x1, z0: sp.area.z0, z1: sp.area.z1 }); }
            }
        }
        if !out.is_empty() { r = Some(out); }
    }
    g.m.dialogue.own_rects.set(LEVEL_KEY, r.clone());
    r
}
/// (dialogue.ts rectDist)
pub fn rect_dist(r: &crate::dialogue::Rect, p: V3) -> f64 { rect_dist_of(r.x0, r.x1, r.z0, r.z1, p) }
/// ownPlace with the rectangle's place in the list, the identity watchedPlace compares   (dialogue.ts ownPlace)
fn own_place_at(g: &mut Game, p: V3) -> Option<(usize, OwnPlace)> {
    let r = level_rects(g)?;
    let (mut best, mut bd) = (0usize, f64::INFINITY);
    for (i, x) in r.iter().enumerate() {
        let d = rect_dist_of(x.x0, x.x1, x.z0, x.z1, p);
        if d == 0.0 { return Some((i, OwnPlace { rect: x.clone(), d: 0.0 })); }
        if d < bd - 0.02 { bd = d; best = i; }
    }
    Some((best, OwnPlace { rect: r[best].clone(), d: if bd <= PLACE_EDGE { 0.0 } else { bd } }))
}
/// the named rectangle a point is in, or the nearest (`d` metres off its edge)   (dialogue.ts ownPlace)
pub fn own_place(g: &mut crate::game::Game, p: V3) -> Option<crate::dialogue::OwnPlace> {
    own_place_at(g, p).map(|x| x.1)
}
/// a squad row's word for floor a man WATCHES from `from`, null for his own floor or floor nobody named   (dialogue.ts watchedPlace)
pub fn watched_place(g: &mut crate::game::Game, from: V3, p: V3) -> Option<String> {
    if is_office(g) {
        let a = room_name(g, p);
        return if a != room_name(g, from) { Some(a) } else { None };
    }
    let (ai, a) = own_place_at(g, p)?;
    if a.d > 0.0 { return None; }
    match own_place_at(g, from) {
        Some((bi, b)) if b.d == 0.0 && bi == ai => None,
        _ => Some(a.rect.name),
    }
}
/// (dialogue.ts ownName)
pub fn own_name(g: &mut crate::game::Game, p: V3) -> Option<String> {
    let w = own_place(g, p)?;
    Some(if w.d == 0.0 { w.rect.name } else { alloc::format!("outside {}", w.rect.name) })
}
/// WHERE A MAN STANDS, for a line that has to be true of the spot: the named rectangle he is INSIDE, or null.   (dialogue.ts placeIn)
pub fn place_in(g: &mut crate::game::Game, p: V3) -> Option<crate::dialogue::PlaceRect> {
    if let Some(w) = own_place(g, p) { return if w.d == 0.0 { Some(w.rect) } else { None }; }
    if !is_office(g) { return None; }
    room_name(g, p);
    let rr = g.m.dialogue.room_rects.get(LEVEL_KEY).unwrap();
    for r in rr.rooms.iter() { if rect_holds(r, p) { return Some(place_of_room(r)); } }
    None
}
/// the name of the place a point is in, for wayName (its closure `at`)
fn at_name(g: &mut Game, p: V3) -> Option<String> {
    match own_place(g, p) { Some(w) if w.d == 0.0 => Some(w.rect.name), _ => None }
}
/// THE WAY A WALK COMES BY, for a line on the net: 'through the …' / 'from the …'. Words only: the search is asked and thrown away.   (dialogue.ts wayName)
pub fn way_name(g: &mut crate::game::Game, a: V3, b: V3, into: Option<&str>) -> Option<String> {
    if level_rects(g).is_none() { return None; }
    let room = into.filter(|s| !s.is_empty()).map(with_the);
    let end = at_name(g, b);
    let from = at_name(g, a);
    let path = crate::scene_collision::nav_grid_find_path(g, a, b, Some(6000.0), None)?;
    if from.is_some() && (from == room || from == end) { return None; }
    let mut way = from.clone();
    let mut prev = a;
    'walk: for q in path.iter().copied() {
        let n = m::max(1.0, m::ceil(v3::dist(prev, q) / 0.5));
        let mut i = 1.0;
        while i <= n {
            let r = at_name(g, v3::lerp3(prev, q, i / n));
            if r.is_some() && (r == room || r == end) { break 'walk; }
            if r.is_some() { way = r; }
            i += 1.0;
        }
        prev = q;
    }
    let way = way?;
    if from.as_deref() == Some(way.as_str()) { Some(alloc::format!("from {}", way)) } else { Some(alloc::format!("through {}", way)) }
}
/// every rectangle placeIn answers by   (dialogue.ts placeRects)
pub fn place_rects(g: &mut crate::game::Game) -> Vec<crate::dialogue::PlaceRect> {
    if let Some(r) = level_rects(g) { return r; }
    if !is_office(g) { return Vec::new(); }
    room_name(g, [0.0, 0.0, 0.0]);
    g.m.dialogue.room_rects.get(LEVEL_KEY).unwrap().rooms.iter().map(place_of_room).collect()
}
/// The room a floor point is in, as the men would say it.   (dialogue.ts roomName)
pub fn room_name(g: &mut crate::game::Game, p: V3) -> String {
    if !g.m.dialogue.room_rects.has(LEVEL_KEY) {
        let office = is_office(g);
        let level = &g.level;
        let mut rooms: Vec<Rect> = Vec::new();
        if let Some(cps) = level.clear_plans.as_ref() {
            for cp in cps.iter() {
                if let (Some(b), Some(room)) = (cp.bounds.as_ref(), cp.room.as_ref().filter(|r| !r.is_empty())) {
                    if !rooms.iter().any(|r| r.name.ends_with(room.as_str())) { rooms.push(Rect { name: with_the_room(room), x0: b.x0, x1: b.x1, z0: b.z0, z1: b.z1 }); }
                }
            }
        }
        if let Some(q) = level.scenarios.as_ref().and_then(|s| s.quiet.as_ref()) {
            for sp in q.spots.iter() {
                if !rooms.iter().any(|r| r.name == sp.place) { rooms.push(Rect { name: sp.place.clone(), x0: sp.area.x0, x1: sp.area.x1, z0: sp.area.z0, z1: sp.area.z1 }); }
            }
        } else if !rooms.iter().any(|r| r.name.contains("server")) {
            let s = &level.mission.server_room;
            rooms.push(Rect { name: String::from("the server room"), x0: s.x0, x1: s.x1, z0: s.z0, z1: s.z1 });
        }
        if office { rooms.extend(office_areas()); }
        g.m.dialogue.room_rects.set(LEVEL_KEY, DialogueModRoomRectsValue { rooms, building: office_walls(), office });
    }
    if !g.m.dialogue.room_rects.get(LEVEL_KEY).unwrap().office {
        return match own_place(g, p) { Some(w) => w.rect.name, None => String::from(NAMELESS) };
    }
    let rr = g.m.dialogue.room_rects.get(LEVEL_KEY).unwrap();
    for r in rr.rooms.iter() { if rect_holds(r, p) { return r.name.clone(); } }
    if rect_holds(&rr.building, p) { return String::from("the back offices"); }
    String::from("the car park")
}
/// (dialogue.ts routeName)
pub fn route_name(g: &mut crate::game::Game, route_i: f64) -> String {
    let route = if route_i >= 0.0 && m::floor(route_i) == route_i { g.level.routes.get(route_i as usize).copied() } else { None };
    let n = match route { Some(r) => g.tab.route[r].name.clone(), None => String::from("route") };
    if let Some(s) = route_names().get(n.clone()) { return s.clone(); }
    if let Some(Some(s)) = route_names_guide().get(n.clone()) { return s.clone(); }
    if let Some(s) = route_names_now().get(n.clone()) { return s.clone(); }
    alloc::format!("the {}", n.replace('_', " "))
}
/// (dialogue.ts doorName)
pub fn door_name(g: &mut crate::game::Game, n: &str) -> String {
    let key = String::from(n);
    if let Some(s) = door_names().get(key.clone()) { return s.clone(); }
    if let Some(Some(s)) = door_names_guide().get(key.clone()) { return s.clone(); }
    if let Some(s) = door_names_now().get(key) { return s.clone(); }
    alloc::format!("the {} door", n.replace('_', " "))
}
/// (dialogue.ts numWord)
pub fn num_word(n: f64) -> String {
    if n >= 0.0 && m::floor(n) == n { if let Some(w) = num_words().get(n as usize) { return w.clone(); } }
    num_string(n)
}
/// (dialogue.ts distToRoute)
pub fn dist_to_route(pts: &[V3], p: V3) -> f64 {
    let mut best = f64::INFINITY;
    for i in 0..pts.len() {
        let (a, b) = (pts[i], pts[(i + 1) % pts.len()]);
        let (abx, abz) = (b[0] - a[0], b[2] - a[2]);
        let l2 = abx * abx + abz * abz;
        let t = if l2 > 1e-6 { m::max(0.0, m::min(1.0, ((p[0] - a[0]) * abx + (p[2] - a[2]) * abz) / l2)) } else { 0.0 };
        best = m::min(best, m::hyp2(a[0] + abx * t - p[0], a[2] + abz * t - p[2]));
    }
    best
}
/// the keyed door nearest this man's route line, within DOOR_ROUTE_M, by its authored lock   (dialogue.ts keyedDoorOn)
pub fn keyed_door_on(g: &mut crate::game::Game, gd: GuardId) -> Option<DoorId> {
    let route = g.level.routes.get(g.gd[gd].route_i).copied()?;
    let pts = g.tab.route[route].points.clone();
    if pts.is_empty() { return None; }
    let mut best: Option<DoorId> = None;
    let mut bd = DOOR_ROUTE_M;
    for d in g.doors.list.clone() {
        if g.dr[d].def.locked != Some(true) || crate::devices::devices_pad_of(g, d).is_some() { continue; }
        let dd = dist_to_route(&pts, g.dr[d].frame_centre);
        if dd < bd { bd = dd; best = Some(d); }
    }
    best
}
/// …as the men call it, or null   (dialogue.ts keyedDoorOnRoute)
pub fn keyed_door_on_route(g: &mut crate::game::Game, gd: GuardId) -> Option<String> {
    let d = keyed_door_on(g, gd)?;
    let name = g.dr[d].def.name.clone();
    Some(door_name(g, &name))
}
/// coarse words for the seconds until the next radio check   (dialogue.ts etaWords)
pub fn eta_words(secs: f64) -> String {
    if secs <= 20.0 { String::from("any second now") }
    else if secs <= 75.0 { String::from("in a minute or so") }
    else if secs <= 150.0 { String::from("in a couple of minutes") }
    else if secs <= 900.0 { alloc::format!("in {} minutes", num_word(m::round(secs / 60.0))) }
    else { String::from("before long") }
}

// ---------------------------------------------------------------- the context: everything the picker needs, read off the game once per exchange
/// does a fact stamped on the net at game time T reach a man held since `held_since`   (dialogue.ts heardOnNet)
pub fn heard_on_net(held_since: f64, t: f64) -> bool { t >= 0.0 && (held_since < 0.0 || t <= held_since + 0.5) }
/// the first colleague the net has asked after by name and not raised, as far as THIS man followed it   (dialogue.ts missingKnownTo)
pub fn missing_known_to(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> {
    let hs = held_since_of(g, gd);
    g.guards.iter().copied().find(|&x| {
        let (at, raised) = (g.gd[x].missed_at, g.gd[x].missing_raised);
        x != gd && (at >= 0.0 || raised) && heard_on_net(hs, if at >= 0.0 { at } else { 0.0 })
    })
}
/// the men this man KNOWS to be down   (dialogue.ts knownDown)
pub fn known_down(g: &mut crate::game::Game, gd: GuardId) -> JsSet<String> {
    let mut s: JsSet<String> = JsSet::default();
    let w = &g.gd[gd].witness;
    if let Some(x) = w.saw_act.as_ref() { s.add(x.victim.clone()); }
    if let Some(x) = w.saw_body.as_ref() { s.add(x.victim.clone()); }
    if let Some(x) = w.called_to_body.as_ref() { s.add(x.victim.clone()); }
    if let Some(x) = w.saw_held.as_ref() { s.add(x.who.clone()); }
    for &x in g.guards.iter() { if x != gd && g.gd[x].missing_raised { s.add(g.gd[x].callsign.clone()); } }
    s
}
/// the roster he counts from: the level's own men   (dialogue.ts staffCount)
pub fn staff_count(g: &mut crate::game::Game) -> f64 {
    let mut n = 0.0;
    for &x in g.guards.iter() { if g.gd[x].unit.is_none() { n += 1.0; } }
    m::max(1.0, n)
}
/// his own side and his own kind   (dialogue.ts partnerOf, its closure `mate`)
fn is_mate(g: &Game, gd: GuardId, x: GuardId) -> bool { g.gd[x].team == g.gd[gd].team && g.gd[x].unit.is_none() == g.gd[gd].unit.is_none() }
/// (dialogue.ts partnerOf)
pub fn partner_of(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> {
    let down = known_down(g, gd);
    if let Some(l) = g.gd[gd].leader { if l != gd && !down.has(g.gd[l].callsign.clone()) { return Some(l); } }
    let gs: &Game = g;
    let follower = gs.guards.iter().copied().find(|&x| x != gd && gs.gd[x].leader == Some(gd) && !down.has(gs.gd[x].callsign.clone()) && is_mate(gs, gd, x));
    if follower.is_some() { return follower; }
    let mut best: Option<GuardId> = None;
    let mut bd = f64::INFINITY;
    for &x in gs.guards.iter() {
        if x == gd || down.has(gs.gd[x].callsign.clone()) || !is_mate(gs, gd, x) { continue; }
        let d = v3::dist_xz(gs.ch[gs.gd[x].ch].pos, gs.ch[gs.gd[gd].ch].pos);
        if d < bd { bd = d; best = Some(x); }
    }
    best
}
/// since when this man counts as HELD: the hold's own clock, else the sandbox's stand-in, else -1   (dialogue.ts heldSinceOf)
pub fn held_since_of(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    if let Some(h) = g.gd[gd].held.as_ref() { return h.since; }
    let talk = &g.tab.game_talk[g.gd[gd].talk];
    if talk.held_since >= 0.0 && (talk.last_t < talk.held_since || g.time - talk.last_t <= HOLD_LAPSE_SECS) { talk.held_since } else { -1.0 }
}

/// Read the game into a DialogueCtx for this man.   (dialogue.ts gather)
pub fn gather(g: &mut crate::game::Game, gd: GuardId) -> crate::dialogue::DialogueCtx {
    let w = g.gd[gd].witness.clone();
    let h = w.heard.clone();
    let r = crate::guards::rollcall_state(g).clone();   // (read only, and nothing below writes it)
    let lvl = crate::guards::escalation_of(g);
    let k = knowledge_of(g, gd);
    let terms = agitation_terms(g, gd);
    let (a, why) = (terms.a, terms.why);
    let band = band_of(a);
    let t = temperament_of(g, gd);
    let hs = held_since_of(g, gd);
    let held = if hs >= 0.0 { g.time - hs } else { 0.0 };
    // tokens
    let partner = partner_of(g, gd);
    let missing_man = missing_known_to(g, gd);
    let checker = g.guards.iter().copied().find(|&x| g.gd[x].state != crate::game::GuardState::Dead && matches!(g.gd[x].task, Some(crate::game::GuardTask::CheckOn(_))));
    let check_task = match checker { Some(c) => match g.gd[c].task { Some(crate::game::GuardTask::CheckOn(x)) => Some(x), _ => None }, None => None };
    let post_man = g.guards.iter().copied().find(|&x| g.gd[x].state != crate::game::GuardState::Dead && g.gd[x].post.is_some());
    let keyed = keyed_door_on(g, gd);
    let my_pos = g.ch[g.gd[gd].ch].pos;
    let pad = crate::devices::devices_pad_near(g, my_pos);
    let next_at = if r.next_at >= 0.0 { r.next_at } else { g.time + (if lvl >= 1.0 { 45.0 } else { 90.0 }) };
    let route_i = g.gd[gd].route_i as f64;
    let callsign = g.gd[gd].callsign.clone();
    let idle = r.stage == crate::guards::RollcallStage::Idle;

    let mut tokens = DialogueCtxTokens::zeroed();   // (every field is written below)
    let partner_word = partner.map(|x| g.gd[x].callsign.clone());
    tokens.partner = partner_word;
    tokens.victim = w.saw_act.as_ref().map(|x| x.victim.clone()).or_else(|| w.saw_held.as_ref().map(|x| x.who.clone()));
    tokens.body = w.saw_body.as_ref().map(|x| x.victim.clone()).or_else(|| w.called_to_body.as_ref().map(|x| x.victim.clone()));
    let missing_word = missing_man.map(|x| g.gd[x].callsign.clone());
    tokens.missing = missing_word;
    tokens.room = Some(room_name(g, my_pos));
    let saw_where = if g.gd[gd].sight_t >= 0.0 {
        let q = g.gd[gd].sight_pos;
        Some(room_name(g, q))
    } else if g.gd[gd].last_seen_t >= 0.0 || w.alerted_by == Some(crate::game::WitnessAlertedBy::Sight) {
        let q = g.gd[gd].last_known.unwrap_or(my_pos);
        Some(room_name(g, q))
    } else { None };
    tokens.saw_where = saw_where;
    let heard_where = match h.small_where { Some(q) if h.small > 0.0 => Some(room_name(g, q)), _ => None };
    tokens.heard_where = heard_where;
    tokens.breaker_room = Some({ let q = g.level.breaker.pos; room_name(g, q) });
    tokens.route = Some(route_name(g, route_i));
    let partner_route = match partner { Some(x) => { let i = g.gd[x].route_i as f64; Some(route_name(g, i)) } None => None };
    tokens.partner_route = partner_route;
    let post_word = match post_man.and_then(|x| g.gd[x].post.as_ref().map(|p| p.name.clone())) { Some(n) => alloc::format!("the {}", n), None => route_name(g, route_i) };
    tokens.post = Some(post_word);
    let door_word = match keyed { Some(d) => { let n = g.dr[d].def.name.clone(); Some(door_name(g, &n)) } None => None };
    tokens.door = door_word;
    let caller_word = match r.caller { Some(c) if c != gd => Some(g.gd[c].callsign.clone()), _ => None };
    tokens.caller = caller_word;
    let checker_word = match checker { Some(c) if c != gd => Some(g.gd[c].callsign.clone()), _ => None };
    tokens.checker = checker_word;
    let checked_word = check_task.map(|x| g.gd[x.who].callsign.clone());
    tokens.checked = checked_word;
    let clear_cur = match g.clearing.as_ref() { Some(c) if c.stage != crate::squad::ClearStage::Done => c.cur.as_ref().map(|x| (x.room.clone(), x.door.clone())), _ => None };
    let clear_room = match clear_cur {
        Some((Some(room), _)) if !room.is_empty() => Some(with_the_room(&room)),
        Some((_, door)) => Some(door_name(g, &door)),
        None => None,
    };
    tokens.clear_room = clear_room;
    tokens.check_eta = Some(if !idle { String::from("any second now") } else { eta_words(next_at - g.time) });
    tokens.minutes = if r.checks > 0.0 { Some(num_word(m::max(1.0, m::round((g.time - r.t0) / 60.0)))) } else { None };
    let staff = staff_count(g);
    tokens.n = Some(num_word(staff));
    let down = known_down(g, gd);
    let down_n = down.iter().filter(|n| !g.civilians.iter().any(|&c| g.gd[c].callsign == *n)).count() as f64;
    let staff_up = staff_count(g);
    tokens.up = Some(num_word(m::max(1.0, staff_up - down_n)));
    tokens.badge = Some(badge_of(&callsign));
    tokens.code = pad.as_ref().map(|p| p.def.code.clone());
    let pad_door = match pad.as_ref() { Some(p) => { let n = g.dr[p.door].def.name.clone(); Some(door_name(g, &n)) } None => None };
    tokens.pad_door = pad_door;
    let nameless = crate::teams::defender_words(g, gd);
    if nameless { tokens.partner = None; tokens.victim = None; tokens.body = None; tokens.missing = None; }
    // facts
    let shift = g.gd[gd].team == crate::teams::Team::Guard && g.gd[gd].unit.is_none();
    let office = shift && is_office(g);
    let weapon = g.gd[gd].weapon;
    let drew_on_you = crate::weapons::firearm_spent(g, weapon) > 0.0;
    let blackout = crate::sim_lightshome::mains_active(g);
    let saw_kind = w.saw_act.as_ref().map(|x| x.kind);
    let facts = DialogueCtxFacts {
        floor_up: Some(lvl >= 1.0), lockdown: Some(lvl >= 2.0), episode: Some(g.alarm.episode && !g.quiet_utility),
        heard_small: Some(h.small > 0.0 && h.small_where.is_some()), heard_shots: Some(h.shots > 0.0), heard_bang: Some(h.kick + h.bang > 0.0),
        body_found: Some(w.saw_body.is_some()), body_called: Some(w.called_to_body.is_some() && w.saw_body.is_none()), body_known: Some(w.saw_body.is_some() || w.called_to_body.is_some()),
        missing_known: Some(missing_man.is_some()), blackout: Some(blackout), held_long: Some(held >= HELD_LONG_SECS),
        rollcall_live: Some(!idle), dazzled: Some(w.dazzled > 0.0),
        saw_shot: Some(saw_kind == Some(crate::game::WitnessSawActKind::Shot)),
        saw_drop: Some(saw_kind.is_some() && saw_kind != Some(crate::game::WitnessSawActKind::Shot) && saw_kind != Some(crate::game::WitnessSawActKind::Grab)),
        alert_now: Some(g.gd[gd].state == crate::game::GuardState::Alert), drew_on_you: Some(drew_on_you), partner_known: Some(partner.is_some()),
        shift: Some(shift), office: Some(office),
    };
    // intel: live = true of the world now; known = he could plausibly know it
    let raised_at = g.alarm.raised_at;
    let mut intel: Vec<IntelItem> = Vec::new();
    let check_text = if checker == Some(gd) { check_on_self(g) } else { intel_text(g, IntelKind::CheckOn, t, k) };
    intel.push(IntelItem { kind: IntelKind::CheckOn, text: check_text, live: check_task.map_or(false, |x| x.who != gd), known: check_task.map_or(false, |x| checker == Some(gd) || heard_on_net(hs, x.t0)) });
    let clearing_live = lvl >= 2.0 && g.clearing.as_ref().map_or(false, |c| c.stage != crate::squad::ClearStage::Done && c.a != gd && c.b != Some(gd));
    intel.push(IntelItem { kind: IntelKind::Clearing, text: intel_text(g, IntelKind::Clearing, t, k), live: clearing_live, known: heard_on_net(hs, raised_at) });
    intel.push(IntelItem { kind: IntelKind::RollcallLive, text: intel_text(g, IntelKind::RollcallLive, t, k), live: !idle && r.caller.is_some() && r.caller != Some(gd), known: true });
    intel.push(IntelItem { kind: IntelKind::Blackout, text: intel_text(g, IntelKind::Blackout, t, k), live: blackout, known: true });
    let door_live = match keyed { Some(d) => g.gd[gd].keys.has(g.dr[d].lock_id.clone()), None => false };
    intel.push(IntelItem { kind: IntelKind::Door, text: intel_text(g, IntelKind::Door, t, k), live: door_live, known: true });
    let code_live = match pad.as_ref() { Some(p) => !g.pl[g.player].codes.has(p.def.name.clone()), None => false };
    intel.push(IntelItem { kind: IntelKind::Code, text: intel_text(g, IntelKind::Code, t, k), live: code_live, known: true });
    intel.push(IntelItem { kind: IntelKind::Rollcall, text: intel_text(g, IntelKind::Rollcall, t, k), live: !g.quiet_utility && g.ai_enabled && partner.is_some(), known: true });
    intel.push(IntelItem { kind: IntelKind::LockdownPosts, text: intel_text(g, IntelKind::LockdownPosts, t, k), live: lvl >= 2.0 && post_man.is_some(), known: heard_on_net(hs, raised_at) || post_man == Some(gd) });
    intel.push(IntelItem { kind: IntelKind::Routes, text: intel_text(g, IntelKind::Routes, t, k), live: partner.is_some(), known: true });
    intel.push(IntelItem { kind: IntelKind::Missing, text: intel_text(g, IntelKind::Missing, t, k), live: missing_man.is_some() && lvl < 2.0, known: missing_man.is_some() });
    let drive_text = if office { intel_text(g, IntelKind::Drive, t, k) } else { drive_away_text(g, t) };
    let drive_live = g.mission.pulled && g.mission.sweep_at < 0.0 && crate::takecheck::take_told(g);
    intel.push(IntelItem { kind: IntelKind::Drive, text: drive_text, live: drive_live, known: true });
    if !shift { for it in intel.iter_mut() { it.live = false; } }
    let recent = g.m.dialogue.recent_by_game.get_or_insert_with(JsMap::new).clone();
    let own_rooms = !is_office(g);
    DialogueCtx { who: callsign, k, a, band, t, why, facts, tokens, own_rooms: Some(own_rooms), nameless: Some(nameless), intel, time: g.time, recent: Some(recent) }
}

// ---------------------------------------------------------------- the picker (pure over a DialogueCtx + the man's Talk state)
/// `driveAway()[t] ?? driveAway()['*']`   (dialogue.ts gather)
fn drive_away_text(g: &mut Game, t: crate::game::Temperament) -> String {
    let d = drive_away(g);
    let own = match t { crate::game::Temperament::Steady => d.steady, crate::game::Temperament::Nervy => d.nervy, crate::game::Temperament::Hard => d.hard };
    own.or(d.under).unwrap_or_default()
}
/// `SHAPE_LEAD[shape]`: `Some("")` is a shape written to take no lead, `None` one with no entry   (dialogue.ts SHAPE_LEAD)
fn lead_of_shape(shape: ShapeKey) -> Option<String> {
    let l = shape_lead();
    match shape { ShapeKey::Pedant => l.pedant, ShapeKey::Selfown => l.selfown, ShapeKey::Folder => l.folder, ShapeKey::Ramble => l.ramble, ShapeKey::Fan => l.fan, ShapeKey::Stoic => l.stoic, ShapeKey::Con => l.con, ShapeKey::Game => l.game, ShapeKey::Menace => l.menace }
}
/// the ctx's tokens as the table `expand` reads, by the TypeScript's keys (dialogue.ts:524)
fn tokens_table(t: &DialogueCtxTokens) -> JsMap<String, Option<String>> {
    let mut o: JsMap<String, Option<String>> = JsMap::new();
    let rows: [(&str, &Option<String>); 23] = [
        ("door", &t.door), ("code", &t.code), ("missing", &t.missing), ("body", &t.body), ("n", &t.n), ("up", &t.up), ("post", &t.post), ("room", &t.room),
        ("route", &t.route), ("partner", &t.partner), ("victim", &t.victim), ("sawWhere", &t.saw_where), ("heardWhere", &t.heard_where), ("partnerRoute", &t.partner_route),
        ("caller", &t.caller), ("checker", &t.checker), ("checked", &t.checked), ("clearRoom", &t.clear_room), ("breakerRoom", &t.breaker_room), ("checkEta", &t.check_eta),
        ("minutes", &t.minutes), ("badge", &t.badge), ("padDoor", &t.pad_door),
    ];
    for (k, v) in rows { o.set(String::from(k), v.clone()); }
    o
}
/// the ctx's facts as needsHold takes them: one shape, two generated types (dialogue.ts:523, 671)
fn hold_facts(f: &DialogueCtxFacts) -> NeedsHoldFacts {
    NeedsHoldFacts { shift: f.shift, rollcall_live: f.rollcall_live, blackout: f.blackout, body_found: f.body_found, floor_up: f.floor_up, lockdown: f.lockdown, episode: f.episode, heard_small: f.heard_small, heard_shots: f.heard_shots, heard_bang: f.heard_bang, body_called: f.body_called, body_known: f.body_known, missing_known: f.missing_known, held_long: f.held_long, dazzled: f.dazzled, saw_shot: f.saw_shot, saw_drop: f.saw_drop, alert_now: f.alert_now, drew_on_you: f.drew_on_you, partner_known: f.partner_known, office: f.office }
}
/// `!!facts[n]`   (dialogue.ts needsHold)
fn hold_fact(f: &NeedsHoldFacts, n: Need) -> bool {
    let v = match n {
        Need::Shift => f.shift, Need::RollcallLive => f.rollcall_live, Need::Blackout => f.blackout, Need::BodyFound => f.body_found, Need::FloorUp => f.floor_up, Need::Lockdown => f.lockdown, Need::Episode => f.episode,
        Need::HeardSmall => f.heard_small, Need::HeardShots => f.heard_shots, Need::HeardBang => f.heard_bang, Need::BodyCalled => f.body_called, Need::BodyKnown => f.body_known, Need::MissingKnown => f.missing_known,
        Need::HeldLong => f.held_long, Need::Dazzled => f.dazzled, Need::SawShot => f.saw_shot, Need::SawDrop => f.saw_drop, Need::AlertNow => f.alert_now, Need::DrewOnYou => f.drew_on_you, Need::PartnerKnown => f.partner_known, Need::Office => f.office,
    };
    v == Some(true)
}
fn stage_eq(a: StageKey, b: StageKey) -> bool {
    match (a, b) { (StageKey::X, StageKey::X) => true, (StageKey::Num(x), StageKey::Num(y)) => x == y, _ => false }
}
/// `String(s)` of a stage: '1', '2', '3', 'x'
fn stage_str(s: StageKey) -> String {
    match s { StageKey::X => String::from("x"), StageKey::Num(n) => num_string(n) }
}
/// a `{token}` at byte `i` of the text (TOKEN_RE): where it ends (past the brace) and its letters
fn token_at(text: &str, i: usize) -> Option<(usize, &str)> {
    let b = text.as_bytes();
    if b[i] != b'{' { return None; }
    let mut j = i + 1;
    while j < b.len() && b[j].is_ascii_alphabetic() { j += 1; }
    if j > i + 1 && j < b.len() && b[j] == b'}' { Some((j + 1, &text[i + 1..j])) } else { None }
}
/// the token's key: lower-cased first letter
fn token_key(raw: &str) -> String {
    let mut key = String::new();
    key.push(raw.as_bytes()[0].to_ascii_lowercase() as char);
    key.push_str(&raw[1..]);
    key
}
/// the tokens a text uses, as Token keys   (dialogue.ts tokensIn)
pub fn tokens_in(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < text.len() {
        if let Some((end, raw)) = token_at(text, i) { out.push(token_key(raw)); i = end; } else { i += 1; }
    }
    out
}
/// expand {tokens} from the table ({Name} capitalises); null if any comes back empty   (dialogue.ts expand)
pub fn expand(text: &str, tokens: &JsMap<String, Option<String>>) -> Option<String> {
    let mut ok = true;
    let mut out = String::new();
    let (mut i, mut last) = (0, 0);
    while i < text.len() {
        let Some((end, raw)) = token_at(text, i) else { i += 1; continue; };
        out.push_str(&text[last..i]);
        let key = token_key(raw);
        match tokens.get(key.clone()).and_then(|v| v.as_deref()).filter(|v| !v.is_empty()) {
            None => ok = false,
            Some(v) if raw.as_bytes()[0] != key.as_bytes()[0] => {
                let mut cs = v.chars();
                if let Some(c) = cs.next() { out.extend(c.to_uppercase()); }
                out.push_str(cs.as_str());
            }
            Some(v) => out.push_str(v),
        }
        i = end;
        last = end;
    }
    out.push_str(&text[last..]);
    if ok { Some(out) } else { None }
}
/// (dialogue.ts stageKeyOf)
pub fn stage_key_of(n: f64) -> crate::dialogue::StageKey {
    if n <= 1.0 { StageKey::Num(1.0) } else if n == 2.0 { StageKey::Num(2.0) } else if n == 3.0 { StageKey::Num(3.0) } else { StageKey::X }
}
/// Is stage `s` one this man GIVES the useful thing at?   (dialogue.ts givingStage)
pub fn giving_stage(t: crate::game::Temperament, band: crate::dialogue::Band, s: crate::dialogue::StageKey, stage_n: f64, given_none: bool) -> bool {
    let gives = gives_at(t, band);
    gives.iter().any(|x| stage_eq(*x, s)) || (stage_eq(s, StageKey::X) && given_none && !gives.is_empty() && stage_n == 4.0)
}
/// (dialogue.ts lineMatches)
pub fn line_matches(l: &crate::dialogue::Line, k: crate::dialogue::Knowledge, band: crate::dialogue::Band, t: crate::game::Temperament, s: crate::dialogue::StageKey, use_t: bool, use_a: bool) -> bool {
    if !stage_eq(l.s, s) { return false; }
    let lk = l.k.as_str();
    if lk != "*" && lk != k.as_str() { return false; }
    if use_t { if let Some(lt) = l.t { if lt != t { return false; } } }
    if use_a {
        match l.a {
            Some(LineA::Mid2) => { if band == Band::Low { return false; } }
            Some(LineA::Low) => { if band != Band::Low { return false; } }
            Some(LineA::Mid) => { if band != Band::Mid { return false; } }
            Some(LineA::High) => { if band != Band::High { return false; } }
            None => {}
        }
    }
    true
}
/// (dialogue.ts needsHold)
pub fn needs_hold(g: &mut crate::game::Game, l: &crate::dialogue::Line, facts: crate::dialogue::NeedsHoldFacts) -> bool {
    let only_ok = match l.only { None => true, Some(LineOnly::Shift) => hold_fact(&facts, Need::Shift), Some(LineOnly::Office) => hold_fact(&facts, Need::Office) };
    only_ok && l.needs.as_ref().map_or(true, |ns| ns.iter().all(|n| hold_fact(&facts, *n)))
}
/// The pending useful things: true now, known to him, not yet given, in the order he would give them.   (dialogue.ts pendingIntel)
pub fn pending_intel(g: &mut crate::game::Game, ctx: &crate::dialogue::DialogueCtx, talk: GameTalkId, sc: Option<&crate::dialogue::Script>) -> Vec<crate::dialogue::PendingIntelItem> {
    let mut out: Vec<PendingIntelItem> = Vec::new();
    let kinds = intel_kinds();
    let order: Vec<String> = match sc.and_then(|x| x.pin) {
        Some(pin) => { let mut o = vec![String::from(pin.as_str())]; o.extend(kinds.iter().filter(|k| k.as_str() != pin.as_str()).cloned()); o }
        None => kinds.to_vec(),
    };
    let tokens = tokens_table(&ctx.tokens);
    for word in order {
        let Some(it) = ctx.intel.iter().find(|i| i.kind.as_str() == word) else { continue; };
        let kind = it.kind;
        if !it.live || !it.known || g.tab.game_talk[talk].given.has(word) { continue; }
        let own = kind == IntelKind::CheckOn && it.text == check_on_self(g);
        let shaped = match sc {
            Some(x) if !own => shaped_intel_text(g, kind, &ShapedIntelTextSc { id: x.id.clone(), shape: x.shape }, ctx.t),
            _ => None,
        };
        let lead = if shaped.as_deref().map_or(false, |s| !s.is_empty()) { String::new() } else {
            let shape_lead = match sc { Some(x) if ctx.t != crate::game::Temperament::Hard => lead_of_shape(x.shape), _ => None };
            match shape_lead { Some(s) => s, None => intel_lead(ctx.t, ctx.k) }
        };
        let fact = run_on(g, &lead, shaped.as_deref().unwrap_or(it.text.as_str()));
        let Some(text) = expand(&alloc::format!("{}{}", lead, fact), &tokens) else { continue; };
        out.push(PendingIntelItem { kind, text });
    }
    out
}
/// The first pending useful thing, or null.   (dialogue.ts intelFor)
pub fn intel_for(g: &mut crate::game::Game, ctx: &crate::dialogue::DialogueCtx, talk: GameTalkId, sc: Option<&crate::dialogue::Script>) -> Option<crate::dialogue::IntelFor> {
    pending_intel(g, ctx, talk, sc).into_iter().next().map(|x| IntelFor { kind: x.kind, text: x.text })
}
/// The rambler's stream swallows a giving stage, when exactly ONE thing is pending.   (dialogue.ts swallowed)
pub fn swallowed(g: &mut crate::game::Game, sc: Option<&crate::dialogue::Script>, s: crate::dialogue::StageKey, ctx: &crate::dialogue::DialogueCtx, talk: GameTalkId) -> bool {
    match sc {
        Some(x) => x.shape == ShapeKey::Ramble && RAMBLE_SWALLOWS > 0.0 && stage_eq(s, StageKey::Num(2.0)) && ctx.t == crate::game::Temperament::Nervy && pending_intel(g, ctx, talk, sc).len() == 1,
        None => false,
    }
}
fn pick_k(k: crate::dialogue::Knowledge) -> DialoguePickLineK {
    match k { Knowledge::None => DialoguePickLineK::None, Knowledge::Heard => DialoguePickLineK::Heard, Knowledge::Glimpsed => DialoguePickLineK::Glimpsed, Knowledge::Saw => DialoguePickLineK::Saw, Knowledge::SawHurt => DialoguePickLineK::SawHurt }
}
fn pick_k_of_him(k: HimOfK) -> DialoguePickLineK {
    match k { HimOfK::None => DialoguePickLineK::None, HimOfK::Heard => DialoguePickLineK::Heard, HimOfK::Glimpsed => DialoguePickLineK::Glimpsed, HimOfK::Saw => DialoguePickLineK::Saw, HimOfK::SawHurt => DialoguePickLineK::SawHurt, HimOfK::Empty => DialoguePickLineK::Empty }
}
fn pick_k_of_line(k: LineK) -> DialoguePickLineK {
    match k { LineK::None => DialoguePickLineK::None, LineK::Heard => DialoguePickLineK::Heard, LineK::Glimpsed => DialoguePickLineK::Glimpsed, LineK::Saw => DialoguePickLineK::Saw, LineK::SawHurt => DialoguePickLineK::SawHurt, LineK::Empty => DialoguePickLineK::Empty }
}
/// `sayable`: a rejoinder as it would be said, or null   (dialogue.ts pickLine)
fn sayable(text: Option<&str>, tokens: &JsMap<String, Option<String>>) -> Option<String> {
    match text { Some(x) if !x.is_empty() && REJOINDERS => expand(x, tokens), _ => None }
}
/// `eligible`: the pool for this cell and stage: (index in the lines, the text, its weight)   (dialogue.ts pickLine)
fn eligible(g: &mut Game, lines: &[Line], ctx: &DialogueCtx, tokens: &JsMap<String, Option<String>>, talk: GameTalkId, s: StageKey, use_t: bool, use_a: bool, allow_said: bool) -> Vec<(usize, String, f64)> {
    let mut out: Vec<(usize, String, f64)> = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        if !line_matches(l, ctx.k, ctx.band, ctx.t, s, use_t, use_a) || !needs_hold(g, l, hold_facts(&ctx.facts)) { continue; }
        if !allow_said && g.tab.game_talk[talk].said.has(l.id.clone()) { continue; }
        let said = said_text(g, &SaidTextL { text: l.text.clone(), now: l.now.clone(), away: l.away.clone(), bare: l.bare.clone(), guide: l.guide.clone() }, ctx);
        let Some(text) = expand(&said, tokens) else { continue; };
        let mut w = l.w.unwrap_or(1.0)
            * (if l.needs.as_ref().map_or(false, |n| !n.is_empty()) { 1.6 } else { 1.0 })
            * (if l.t == Some(ctx.t) { 1.5 } else { 1.0 })
            * (if l.a.is_some() { 1.3 } else { 1.0 })
            * (if l.k.as_str() != "*" { 1.15 } else { 1.0 });
        if let Some(rt) = ctx.recent.as_ref().and_then(|r| r.get(l.id.clone()).copied()) { if ctx.time - rt < RECENT_SECS { w *= RECENT_W; } }
        if allow_said { w /= 1.0 + g.tab.game_talk[talk].said_n.get(l.id.clone()).copied().unwrap_or(0.0) * 3.0; }
        out.push((i, text, w));
    }
    out
}
/// One exchange: advance the man's stage, pick the line for (his cell × that stage), mark it said.   (dialogue.ts pickLine)
pub fn pick_line(g: &mut crate::game::Game, ctx: &mut crate::dialogue::DialogueCtx, talk: GameTalkId, rnd: &mut dyn FnMut(&mut crate::game::Game) -> f64) -> crate::dialogue::DialoguePick {
    let sc0 = script_for(g, ctx, talk);
    let stage_n = g.tab.game_talk[talk].stage + 1.0;
    let s = stage_key_of(stage_n);
    {
        let time = ctx.time;
        let row = &mut g.tab.game_talk[talk];
        row.stage = stage_n; row.presses += 1.0; row.last_t = time;
    }
    let cell = alloc::format!("{}·{}·{}", ctx.k.as_str(), ctx.band.as_str(), ctx.t.as_str());
    let sc = sc0.filter(|x| stage_n <= 4.0 && script_live(x, ctx));
    let beat = sc.as_ref().map(|x| beat_for(x, s, ctx));
    let (script_id, shape) = (sc.as_ref().map(|x| x.id.clone()), sc.as_ref().map(|x| x.shape));
    let tokens = tokens_table(&ctx.tokens);
    let base = DialoguePick { text: String::new(), id: String::new(), kind: None, stage: s, stage_n, cell: cell.clone(), k: ctx.k, band: ctx.band, t: ctx.t, line_k: pick_k(ctx.k), widened: DialoguePickWidened::Empty, after: None, script: script_id, shape };
    let stage_word = stage_str(s);
    // the giving stages: the INTEL generator first
    let given_none = g.tab.game_talk[talk].given.is_empty();
    if giving_stage(ctx.t, ctx.band, s, stage_n, given_none) && !swallowed(g, sc.as_ref(), s, ctx, talk) {
        if let Some(it) = intel_for(g, ctx, talk, sc.as_ref()) {
            let id = alloc::format!("intel:{}", it.kind.as_str());
            let row = &mut g.tab.game_talk[talk];
            row.given.add(String::from(it.kind.as_str()));
            row.said.add(id.clone());
            row.last = Some(crate::game::TalkLast { id: id.clone(), cell, stage: stage_word, kind: Some(String::from(it.kind.as_str())) });
            return DialoguePick { text: it.text, id, kind: Some(it.kind), after: sayable(beat.as_ref().and_then(|b| b.take.as_deref()), &tokens), ..base };
        }
    }
    // the script's own beat for this press
    if let (Some(x), Some(b)) = (sc.as_ref(), beat.as_ref()) {
        if let Some(h) = him_of(b, ctx) {
            let id = alloc::format!("{}:{}", x.id, stage_word);
            let row = &mut g.tab.game_talk[talk];
            row.said.add(id.clone());
            let n = row.said_n.get(id.clone()).copied().unwrap_or(0.0) + 1.0;
            row.said_n.set(id.clone(), n);
            row.last = Some(crate::game::TalkLast { id: id.clone(), cell, stage: stage_word, kind: None });
            return DialoguePick { text: h.text, id, line_k: pick_k_of_him(h.k), after: sayable(b.after.as_deref(), &tokens), ..base };
        }
    }
    // the pool for this cell and stage: exact, else any temperament, else any agitation too; never another man's knowledge
    let lines = crate::dialogue::lines();
    let chain: [(bool, bool, bool, DialoguePickWidened); 4] = if stage_eq(s, StageKey::X) {
        [(true, true, false, DialoguePickWidened::Empty), (true, true, true, DialoguePickWidened::Repeat), (false, true, true, DialoguePickWidened::T), (false, false, true, DialoguePickWidened::A)]
    } else {
        [(true, true, false, DialoguePickWidened::Empty), (false, true, false, DialoguePickWidened::T), (false, false, false, DialoguePickWidened::A), (false, false, true, DialoguePickWidened::Repeat)]
    };
    let mut widened = DialoguePickWidened::Empty;
    let mut pool: Vec<(usize, String, f64)> = Vec::new();
    for (use_t, use_a, allow_said, tag) in chain {
        pool = eligible(g, &lines, ctx, &tokens, talk, s, use_t, use_a, allow_said);
        widened = tag;
        if allow_said && pool.len() > 1 {
            if let Some(last_id) = g.tab.game_talk[talk].last.as_ref().map(|x| x.id.clone()) {
                let fresh: Vec<(usize, String, f64)> = pool.iter().filter(|p| lines[p.0].id != last_id).cloned().collect();
                if !fresh.is_empty() { pool = fresh; }
            }
        }
        if !pool.is_empty() { break; }
    }
    if pool.is_empty() {
        let id = String::from("-");
        g.tab.game_talk[talk].last = Some(crate::game::TalkLast { id: id.clone(), cell, stage: stage_word, kind: None });
        return DialoguePick { text: String::from("…"), id, widened: DialoguePickWidened::Repeat, ..base };
    }
    let mut total = 0.0;
    for p in pool.iter() { total += p.2; }
    let mut r = rnd(g) * total;
    let mut chosen = pool.len() - 1;
    for (i, p) in pool.iter().enumerate() { r -= p.2; if r <= 0.0 { chosen = i; break; } }
    let (li, text, _) = pool.swap_remove(chosen);
    let l = &lines[li];
    let row = &mut g.tab.game_talk[talk];
    row.said.add(l.id.clone());
    let n = row.said_n.get(l.id.clone()).copied().unwrap_or(0.0) + 1.0;
    row.said_n.set(l.id.clone(), n);
    // ctx.recent is the game's table in the TypeScript: here a copy, so the write goes to both
    if let Some(rec) = ctx.recent.as_mut() {
        rec.set(l.id.clone(), ctx.time);
        g.m.dialogue.recent_by_game.get_or_insert_with(JsMap::new).set(l.id.clone(), ctx.time);
    }
    g.tab.game_talk[talk].last = Some(crate::game::TalkLast { id: l.id.clone(), cell, stage: stage_word, kind: None });
    DialoguePick { text, id: l.id.clone(), line_k: pick_k_of_line(l.k), widened, ..base }
}
/// The whole exchange for a real man: gather → pick; null for a man who cannot talk.   (dialogue.ts interrogate)
pub fn interrogate(g: &mut crate::game::Game, gd: GuardId, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) -> Option<crate::dialogue::DialoguePick> {
    if g.gd[gd].state == crate::game::GuardState::Dead || !g.guards.contains(&gd) { return None; }
    let talk = g.gd[gd].talk;
    let time = g.time;
    let unheld = g.gd[gd].held.is_none();
    let row = &mut g.tab.game_talk[talk];
    if unheld && (row.held_since < 0.0 || (row.last_t >= 0.0 && time - row.last_t > HOLD_LAPSE_SECS)) { row.held_since = time; }
    let mut ctx = gather(g, gd);
    let pick = match rnd {
        Some(r) => pick_line(g, &mut ctx, talk, r),
        None => pick_line(g, &mut ctx, talk, &mut |g: &mut Game| crate::math_dice::sim_random(g)),
    };
    let player = g.player;
    let holds_him = g.pl[player].holding.as_ref().map_or(false, |h| h.g == gd);
    if let Some(after) = pick.after.as_ref().filter(|a| !a.is_empty()) {
        // (`.length` of the TypeScript counts UTF-16 units: '…' and '—' are one, three bytes here)
        if holds_him && after.encode_utf16().count() as f64 <= SAM_MAX_CHARS {
            let id = alloc::format!("{}+", pick.id);
            g.pl[player].holding.as_mut().unwrap().after = Some(crate::player::HoldAfter { text: after.clone(), id: id.clone() });
            let mem = sam_memory_of(g);
            mem.said.set(id.clone(), time);
            mem.last = Some(id);
        }
    }
    Some(pick)
}
/// The VERB on the held man's question row for the next exchange.   (dialogue.ts promptFor)
pub fn prompt_for(g: &mut crate::game::Game, gd: GuardId) -> crate::dialogue::PromptFor {
    let talk = g.gd[gd].talk;
    let next = stage_key_of(g.tab.game_talk[talk].stage + 1.0);
    if stage_eq(next, StageKey::Num(1.0)) { return PromptFor { text: String::from("make him talk"), spent: false }; }
    let mut ctx = gather(g, gd);
    if peek_give(g, &mut ctx, talk).is_some() { return PromptFor { text: String::from("ask what he knows"), spent: false }; }
    if stage_eq(next, StageKey::Num(2.0)) { PromptFor { text: String::from("lean on him"), spent: false } }
    else if stage_eq(next, StageKey::Num(3.0)) { PromptFor { text: String::from("ask what he knows"), spent: false } }
    else { PromptFor { text: String::from("he's got nothing left"), spent: true } }
}
/// One line for the panel and probes: his cell, the next press, what he has said, what he would give next.   (dialogue.ts dialogueSummary)
pub fn dialogue_summary(g: &mut crate::game::Game, gd: GuardId) -> String {
    let ctx = gather(g, gd);
    let talk = g.gd[gd].talk;
    let next = stage_key_of(g.tab.game_talk[talk].stage + 1.0);
    let sc = peek_script(g, talk);
    let live = sc.as_ref().filter(|x| script_live(x, &ctx));
    let it = intel_for(g, &ctx, talk, live);
    let row = &g.tab.game_talk[talk];
    let why = if ctx.why.is_empty() { String::from("—") } else { ctx.why.join(", ") };
    let script = match sc.as_ref() { Some(x) => alloc::format!("{} ({})", x.id, x.shape.as_str()), None => String::from("—") };
    let gives = if gives_at(ctx.t, ctx.band).iter().any(|x| stage_eq(*x, next)) { " (gives)" } else { "" };
    let gave = if row.given.is_empty() { String::new() } else { alloc::format!(" · gave {}", row.given.keys().join("/")) };
    let last = match row.last.as_ref() { Some(l) => alloc::format!(" · last: {} @{} s{}", l.id, l.cell, l.stage), None => String::new() };
    alloc::format!("{} ({}{}) · cell {}·{}·{} · A {:.2} [{}] · script {} · next press: stage {}{} · said {}{} · next intel: {}{}",
        g.gd[gd].callsign, ctx.t.as_str(), if g.gd[gd].state == crate::game::GuardState::Dead { ", down" } else { "" },
        ctx.k.as_str(), ctx.band.as_str(), ctx.t.as_str(), ctx.a, why, script, stage_str(next), gives, row.said.len(), gave,
        it.as_ref().map_or("—", |x| x.kind.as_str()), last)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn table(rows: &[(&str, Option<&str>)]) -> JsMap<String, Option<String>> {
        let mut t: JsMap<String, Option<String>> = JsMap::default();
        for (k, v) in rows { t.set(String::from(*k), v.map(String::from)); }
        t
    }
    #[test]
    fn expand_capitalises_and_fails_on_empty() {
        let t = table(&[("room", Some("the lobby")), ("door", None), ("code", Some(""))]);
        assert_eq!(expand("in {room}. {Room}!", &t).as_deref(), Some("in the lobby. The lobby!"));
        assert_eq!(expand("{door}", &t), None);
        assert_eq!(expand("{code}", &t), None);
        assert_eq!(expand("{nobody}", &t), None);
        assert_eq!(expand("— {x y} {} { …", &t).as_deref(), Some("— {x y} {} { …"));
    }
    #[test]
    fn tokens_in_lowers_first_letter() {
        let got = tokens_in("{Room} {{padDoor} {no way} {}");
        assert_eq!(got, vec![String::from("room"), String::from("padDoor")]);
    }
    #[test]
    fn names_and_numbers() {
        assert_eq!(with_the("yard"), "the yard");
        assert_eq!(with_the("the yard"), "the yard");
        assert_eq!(with_the("block 2"), "block 2");
        assert_eq!(num_word(3.0), "three");
        assert_eq!(num_word(15.0), "fifteen");
        assert_eq!(num_word(16.0), "16");
        assert_eq!(num_word(-1.0), "-1");
        assert_eq!(num_word(1.5), "1.5");
        assert_eq!(eta_words(20.0), "any second now");
        assert_eq!(eta_words(300.0), "in five minutes");
        assert_eq!(eta_words(901.0), "before long");
    }
    #[test]
    fn the_ctx_tables_reach_the_picker() {
        let mut t = DialogueCtxTokens::zeroed();
        t.saw_where = Some(String::from("the yard"));
        t.pad_door = Some(String::from("the pad door"));
        let tab = tokens_table(&t);
        assert_eq!(expand("{SawWhere}, {padDoor}", &tab).as_deref(), Some("The yard, the pad door"));
        assert_eq!(expand("{clearRoom}", &tab), None);
        let mut f = DialogueCtxFacts::zeroed();
        f.office = Some(true);
        f.saw_drop = Some(false);
        let h = hold_facts(&f);
        assert!(hold_fact(&h, Need::Office));
        assert!(!hold_fact(&h, Need::SawDrop));
        assert!(!hold_fact(&h, Need::Shift));
        assert_eq!(lead_of_shape(ShapeKey::Folder).as_deref(), Some(""));
        assert_eq!(lead_of_shape(ShapeKey::Fan).as_deref(), Some("Oh— and "));
        assert_eq!(lead_of_shape(ShapeKey::Stoic), None);
    }
    #[test]
    fn stages() {
        assert!(matches!(stage_key_of(0.0), StageKey::Num(x) if x == 1.0));
        assert!(matches!(stage_key_of(3.0), StageKey::Num(x) if x == 3.0));
        assert!(matches!(stage_key_of(4.0), StageKey::X));
        assert_eq!(stage_str(StageKey::Num(2.0)), "2");
        assert_eq!(stage_str(StageKey::X), "x");
    }
    #[test]
    fn route_distance_wraps_the_loop() {
        let pts = [[0.0, 0.0, 0.0], [4.0, 0.0, 0.0], [4.0, 0.0, 4.0]];
        assert_eq!(dist_to_route(&pts, [2.0, 0.0, -3.0]), 3.0);
        assert_eq!(dist_to_route(&[], [0.0, 0.0, 0.0]), f64::INFINITY);
    }
    #[test]
    fn net_reach() {
        assert!(heard_on_net(-1.0, 5.0));
        assert!(heard_on_net(10.0, 10.5));
        assert!(!heard_on_net(10.0, 10.6));
        assert!(!heard_on_net(-1.0, -1.0));
    }
}
