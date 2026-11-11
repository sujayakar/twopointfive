use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- names: rooms, routes, doors, numbers
fn with_the_room(room: &str) -> String { if room.starts_with("the ") { String::from(room) } else { alloc::format!("the {}", room) } }
fn rect_dist_of(x0: f64, x1: f64, z0: f64, z1: f64, p: V3) -> f64 {
    m::hyp2(m::max_n(&[x0 - p[0], 0.0, p[0] - x1]), m::max_n(&[z0 - p[2], 0.0, p[2] - z1]))
}
fn rect_holds(r: &Rect, p: V3) -> bool { p[0] >= r.x0 && p[0] <= r.x1 && p[2] >= r.z0 && p[2] <= r.z1 }
fn place_of_room(r: &Rect) -> PlaceRect { PlaceRect { id: None, name: r.name.clone(), x0: r.x0, x1: r.x1, z0: r.z0, z1: r.z1 } }
/// `level.routes[i]`: a negative index is no route
fn route_at(g: &Game, i: i32) -> Option<RouteId> { usize::try_from(i).ok().and_then(|i| g.level.routes.get(i).copied()) }

/// WHERE, for a line on the net: the level's own name for the spot, else roomName.
pub fn place_name(g: &crate::game::Game, p: V3) -> String {
    match own_name(g, p) { Some(n) => n, None => room_name(g, p) }
}
/// the level the game was written on (its three patrol routes)
pub fn is_office(g: &crate::game::Game) -> bool {
    ["corridor", "cubicles", "lobby_break"].iter().all(|n| g.level.routes.iter().any(|r| g.tab.route[*r].name == *n))
}
fn with_the(n: &str) -> String {
    if n.starts_with("the ") || n.as_bytes().last().map_or(false, |c| c.is_ascii_digit()) { String::from(n) } else { alloc::format!("the {}", n) }
}
/// THE LEVEL'S OWN NAMES for its floor, first match wins; null on the office and on a level that names nothing.
fn level_rects(g: &crate::game::Game) -> Option<Vec<crate::dialogue::PlaceRect>> {
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
    r
}
/// ownPlace over a list of named rectangles, with the rectangle's place in the list, the identity watchedPlace compares
fn place_among(r: &[PlaceRect], p: V3) -> (usize, OwnPlace) {
    let (mut best, mut bd) = (0usize, f64::INFINITY);
    for (i, x) in r.iter().enumerate() {
        let d = rect_dist_of(x.x0, x.x1, x.z0, x.z1, p);
        if d == 0.0 { return (i, OwnPlace { rect: x.clone(), d: 0.0 }); }
        if d < bd - 0.02 { bd = d; best = i; }
    }
    (best, OwnPlace { rect: r[best].clone(), d: if bd <= PLACE_EDGE { 0.0 } else { bd } })
}
fn own_place_at(g: &Game, p: V3) -> Option<(usize, OwnPlace)> {
    Some(place_among(&level_rects(g)?, p))
}
/// the named rectangle a point is in, or the nearest (`d` metres off its edge)
pub fn own_place(g: &crate::game::Game, p: V3) -> Option<crate::dialogue::OwnPlace> {
    own_place_at(g, p).map(|x| x.1)
}
/// a squad row's word for floor a man WATCHES from `from`, null for his own floor or floor nobody named
pub fn watched_place(g: &crate::game::Game, from: V3, p: V3) -> Option<String> {
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
fn own_name(g: &crate::game::Game, p: V3) -> Option<String> {
    let w = own_place(g, p)?;
    Some(if w.d == 0.0 { w.rect.name } else { alloc::format!("outside {}", w.rect.name) })
}
/// WHERE A MAN STANDS, for a line that has to be true of the spot: the named rectangle he is INSIDE, or null.
pub fn place_in(g: &crate::game::Game, p: V3) -> Option<crate::dialogue::PlaceRect> {
    if let Some(w) = own_place(g, p) { return if w.d == 0.0 { Some(w.rect) } else { None }; }
    if !is_office(g) { return None; }
    for r in office_rooms(g).0.iter() { if rect_holds(r, p) { return Some(place_of_room(r)); } }
    None
}
/// the name of the place a point is in, for wayName (its closure `at`)
fn at_name(rects: &[PlaceRect], p: V3) -> Option<String> {
    match place_among(rects, p).1 { w if w.d == 0.0 => Some(w.rect.name), _ => None }
}
/// THE WAY A WALK COMES BY, for a line on the net: 'through the …' / 'from the …'. Words only: the search is asked and thrown away.
pub fn way_name(g: &crate::game::Game, a: V3, b: V3, into: Option<&str>) -> Option<String> {
    let rects = level_rects(g)?;
    let room = into.filter(|s| !s.is_empty()).map(with_the);
    let end = at_name(&rects, b);
    let from = at_name(&rects, a);
    let path = crate::scene_collision::nav_grid_find_path(g, a, b, Some(6000.0), None)?;
    if from.is_some() && (from == room || from == end) { return None; }
    let mut way = from.clone();
    let mut prev = a;
    'walk: for q in path.iter().copied() {
        let n = m::max(1.0, m::ceil(v3::dist(prev, q) / 0.5));
        let mut i = 1.0;
        while i <= n {
            let r = at_name(&rects, v3::lerp3(prev, q, i / n));
            if r.is_some() && (r == room || r == end) { break 'walk; }
            if r.is_some() { way = r; }
            i += 1.0;
        }
        prev = q;
    }
    let way = way?;
    if from.as_deref() == Some(way.as_str()) { Some(alloc::format!("from {}", way)) } else { Some(alloc::format!("through {}", way)) }
}
/// every rectangle placeIn answers by
pub fn place_rects(g: &crate::game::Game) -> Vec<crate::dialogue::PlaceRect> {
    if let Some(r) = level_rects(g) { return r; }
    if !is_office(g) { return Vec::new(); }
    office_rooms(g).0.iter().map(place_of_room).collect()
}
/// The office's rooms as the men say them, the level's own first, and its walls: the only level that has them.
fn office_rooms(g: &crate::game::Game) -> (Vec<Rect>, Rect) {
    let level = &g.level;
    let mut rooms: Vec<Rect> = Vec::new();
    if let Some(cps) = level.clear_plans.as_ref() {
        for cp in cps.iter() {
            if let (Some(b), Some(room)) = (cp.bounds.as_ref(), cp.room.as_ref().filter(|r| !r.is_empty())) {
                if !rooms.iter().any(|r| r.name.ends_with(room.as_str())) { rooms.push(Rect { name: with_the_room(room), x0: b.x0, x1: b.x1, z0: b.z0, z1: b.z1 }); }
            }
        }
    }
    for sp in level.spots() {
        if !rooms.iter().any(|r| r.name == sp.place) { rooms.push(Rect { name: sp.place.clone(), x0: sp.area.x0, x1: sp.area.x1, z0: sp.area.z0, z1: sp.area.z1 }); }
    }
    if !level.own_places() && !rooms.iter().any(|r| r.name.contains("server")) {
        let s = &level.errand.area;
        rooms.push(Rect { name: String::from("the server room"), x0: s.x0, x1: s.x1, z0: s.z0, z1: s.z1 });
    }
    rooms.extend(office_areas());
    (rooms, office_walls())
}
/// The room a floor point is in, as the men would say it.
pub fn room_name(g: &crate::game::Game, p: V3) -> String {
    if !is_office(g) {
        return match own_place(g, p) { Some(w) => w.rect.name, None => String::from(NAMELESS) };
    }
    let (rooms, building) = office_rooms(g);
    for r in rooms.iter() { if rect_holds(r, p) { return r.name.clone(); } }
    if rect_holds(&building, p) { return String::from("the back offices"); }
    String::from("the car park")
}
/// the one named place every point of a walk is in
fn place_kept_to(g: &Game, points: &[V3]) -> Option<String> {
    let rects = level_rects(g)?;
    let (first, at) = place_among(&rects, *points.first()?);
    if at.d > 0.0 || points.iter().any(|p| { let (i, o) = place_among(&rects, *p); i != first || o.d > 0.0 }) { return None; }
    Some(at.rect.name)
}
/// What a man calls his route: the level's own place, where it keeps to one, else its name in words.
pub fn route_name(g: &crate::game::Game, route_i: f64) -> String {
    let route = if route_i >= 0.0 && m::floor(route_i) == route_i { g.level.routes.get(route_i as usize).copied() } else { None };
    if let Some(one) = route.and_then(|r| place_kept_to(g, &g.tab.route[r].points)) { return one; }
    let n = match route { Some(r) => g.tab.route[r].name.clone(), None => String::from("route") };
    if let Some((_, s)) = ROUTE_NAMES.iter().find(|(id, _)| *id == n) { return String::from(*s); }
    alloc::format!("the {}", n.replace('_', " "))
}
pub(super) fn door_name(n: &str) -> String {
    if let Some((_, s)) = DOOR_NAMES.iter().find(|(id, _)| *id == n) { return String::from(*s); }
    alloc::format!("the {} door", n.replace('_', " "))
}
fn num_word(n: f64) -> String {
    if n >= 0.0 && m::floor(n) == n { if let Some(w) = num_words().get(n as usize) { return w.clone(); } }
    js::num_str(n)
}
fn dist_to_route(pts: &[V3], p: V3) -> f64 {
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
/// the keyed door nearest this man's route line, within DOOR_ROUTE_M, by its authored lock
fn keyed_door_on(g: &crate::game::Game, gd: GuardId) -> Option<DoorId> {
    let route = route_at(g, g.gd[gd].route_i)?;
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
/// coarse words for the seconds until the next radio check
fn eta_words(secs: f64) -> String {
    if secs <= 20.0 { String::from("any second now") }
    else if secs <= 75.0 { String::from("in a minute or so") }
    else if secs <= 150.0 { String::from("in a couple of minutes") }
    else if secs <= 900.0 { alloc::format!("in {} minutes", num_word(m::round(secs / 60.0))) }
    else { String::from("before long") }
}

// ---------------------------------------------------------------- the context: everything the picker needs, read off the game once per exchange
/// does a fact stamped on the net at game time T reach a man held since `held_since`
fn heard_on_net(held_since: f64, t: f64) -> bool { t >= 0.0 && (held_since < 0.0 || t <= held_since + 0.5) }
/// the first colleague the net has asked after by name and not raised, as far as THIS man followed it
pub(super) fn missing_known_to(g: &crate::game::Game, gd: GuardId) -> Option<GuardId> {
    let hs = held_since_of(g, gd);
    g.guards.iter().copied().find(|&x| {
        let (at, raised) = (g.gd[x].missed_at, g.gd[x].missing_raised);
        x != gd && (at >= 0.0 || raised) && heard_on_net(hs, if at >= 0.0 { at } else { 0.0 })
    })
}
/// the men this man KNOWS to be down
pub(super) fn known_down(g: &crate::game::Game, gd: GuardId) -> JsSet<String> {
    let mut s: JsSet<String> = JsSet::default();
    let w = &g.gd[gd].witness;
    if let Some(x) = w.saw_act.as_ref() { s.add(x.victim.clone()); }
    if let Some(x) = w.saw_body.as_ref() { s.add(x.victim.clone()); }
    if let Some(x) = w.called_to_body.as_ref() { s.add(x.victim.clone()); }
    if let Some(x) = w.saw_held.as_ref() { s.add(x.who.clone()); }
    for &x in g.guards.iter() { if x != gd && g.gd[x].missing_raised { s.add(g.gd[x].callsign.clone()); } }
    s
}
/// the roster he counts from: the level's own men
fn staff_count(g: &crate::game::Game) -> f64 {
    let mut n = 0.0;
    for &x in g.guards.iter() { if g.gd[x].unit.is_none() { n += 1.0; } }
    m::max(1.0, n)
}
/// his own side and his own kind
fn is_mate(g: &Game, gd: GuardId, x: GuardId) -> bool { g.team(x) == g.team(gd) && g.gd[x].unit.is_none() == g.gd[gd].unit.is_none() }
fn partner_of(g: &crate::game::Game, gd: GuardId) -> Option<GuardId> {
    let down = known_down(g, gd);
    if let Some(l) = g.gd[gd].leader { if l != gd && !down.has(g.gd[l].callsign.clone()) { return Some(l); } }
    let gs: &Game = g;
    let follower = gs.guards.iter().copied().find(|&x| x != gd && gs.gd[x].leader == Some(gd) && !down.has(gs.gd[x].callsign.clone()) && is_mate(gs, gd, x));
    if follower.is_some() { return follower; }
    let mut best: Option<GuardId> = None;
    let mut bd = f64::INFINITY;
    for &x in gs.guards.iter() {
        if x == gd || down.has(gs.gd[x].callsign.clone()) || !is_mate(gs, gd, x) { continue; }
        let d = v3::dist_xz(gs.pos_of(x), gs.pos_of(gd));
        if d < bd { bd = d; best = Some(x); }
    }
    best
}
/// since when this man counts as HELD: the hold's own clock, else the sandbox's stand-in, else -1
pub(super) fn held_since_of(g: &crate::game::Game, gd: GuardId) -> f64 {
    if let Some(h) = g.gd[gd].held.as_ref() { return h.since; }
    let talk = &g.tab.game_talk[g.gd[gd].talk];
    if talk.held_since >= 0.0 && (talk.last_t < talk.held_since || g.time - talk.last_t <= HOLD_LAPSE_SECS) { talk.held_since } else { -1.0 }
}

/// What the game holds that a man's words may be about: who is missing, who checks on whom, what is near him.
struct About {
    level: f64,
    held_since: f64,
    roll: crate::guards::Rollcall,
    partner: Option<GuardId>, missing: Option<GuardId>,
    checker: Option<GuardId>, check: Option<crate::game::ErrandCheckOn>,
    post_man: Option<GuardId>,
    keyed: Option<DoorId>, pad: Option<crate::devices::KeypadRt>,
    shift: bool, office: bool,
}
impl About {
    fn of(g: &mut Game, gd: GuardId) -> About {
        let checks = |g: &Game, x: GuardId| match g.gd[x].errand { Some(crate::game::Errand::CheckOn(c)) if !crate::condition::down(g, g.gd[x].ch) => Some(c), _ => None };
        let checker = g.guards.iter().copied().find(|&x| checks(g, x).is_some());
        let at = g.pos_of(gd);
        let shift = g.team(gd) == crate::teams::Team::Guard && g.gd[gd].unit.is_none();
        About {
            level: crate::guards::escalation_of(g),
            held_since: held_since_of(g, gd),
            roll: crate::guards::rollcall_state(g).clone(),
            partner: partner_of(g, gd),
            missing: missing_known_to(g, gd),
            checker,
            check: checker.and_then(|c| checks(g, c)),
            post_man: g.guards.iter().copied().find(|&x| !crate::condition::down(g, g.gd[x].ch) && g.gd[x].post.is_some()),
            keyed: keyed_door_on(g, gd),
            pad: crate::devices::devices_pad_near(g, at),
            shift,
            office: shift && is_office(g),
        }
    }
    fn roll_idle(&self) -> bool { self.roll.stage == crate::guards::RollcallStage::Idle }
}

/// Read the game into a DialogueCtx for this man.
pub(super) fn gather(g: &mut crate::game::Game, gd: GuardId) -> crate::dialogue::DialogueCtx {
    let k = knowledge_of(g, gd);
    let band = band_of(agitation_terms(g, gd).a);
    let t = temperament_of(g, gd);
    let b = About::of(g, gd);
    let mut tokens = tokens_of(g, gd, &b);
    let nameless = crate::teams::defender_words(g, gd);
    if nameless { tokens.partner = None; tokens.victim = None; tokens.body = None; tokens.missing = None; }
    let facts = facts_of(g, gd, &b);
    let intel = intel_of(g, gd, &b, t, k);
    let recent = g.m.dialogue.recent_by_game.get_or_insert_with(JsMap::new).clone();
    DialogueCtx { who: g.gd[gd].callsign.clone(), k, band, t, facts, tokens, own_rooms: Some(!is_office(g)), nameless: Some(nameless), intel, time: g.time, recent: Some(recent) }
}

/// The words a line's blanks are filled with.
fn tokens_of(g: &Game, gd: GuardId, b: &About) -> DialogueCtxTokens {
    let w = g.gd[gd].witness.clone();
    let r = &b.roll;
    let at = g.pos_of(gd);
    let route = g.gd[gd].route_i as f64;
    let name = |g: &Game, x: GuardId| g.gd[x].callsign.clone();
    let door = |g: &Game, d: DoorId| door_name(&g.dr[d].def.name);
    let (staff, down) = (staff_count(g), known_down(g, gd));
    let down_n = down.iter().filter(|n| !g.civilians.iter().any(|&c| g.gd[c].callsign == *n)).count() as f64;
    let next_at = if r.next_at >= 0.0 { r.next_at } else { g.time + (if b.level >= 1.0 { 45.0 } else { 90.0 }) };
    let saw = if g.gd[gd].sight_t >= 0.0 { Some(g.gd[gd].sight_pos) }
        else if g.gd[gd].last_seen_t >= 0.0 || w.alerted_by == Some(crate::game::WitnessAlertedBy::Sight) { Some(g.gd[gd].last_known.unwrap_or(at)) }
        else { None };
    DialogueCtxTokens {
        partner: b.partner.map(|x| name(g, x)),
        victim: w.saw_act.as_ref().map(|x| x.victim.clone()).or_else(|| w.saw_held.as_ref().map(|x| x.who.clone())),
        body: w.saw_body.as_ref().map(|x| x.victim.clone()).or_else(|| w.called_to_body.as_ref().map(|x| x.victim.clone())),
        missing: b.missing.map(|x| name(g, x)),
        room: Some(room_name(g, at)),
        saw_where: saw.map(|q| room_name(g, q)),
        heard_where: w.heard.small_where.filter(|_| w.heard.small > 0.0).map(|q| room_name(g, q)),
        breaker_room: Some(room_name(g, g.level.breaker.pos)),
        route: Some(route_name(g, route)),
        partner_route: b.partner.map(|x| route_name(g, g.gd[x].route_i as f64)),
        post: Some(match b.post_man.and_then(|x| g.gd[x].post.as_ref()) { Some(p) => alloc::format!("the {}", p.name), None => route_name(g, route) }),
        door: b.keyed.map(|d| door(g, d)),
        caller: r.caller.filter(|&c| c != gd).map(|c| name(g, c)),
        checker: b.checker.filter(|&c| c != gd).map(|c| name(g, c)),
        checked: b.check.map(|c| name(g, c.who)),
        clear_room: g.clearing.as_ref().filter(|c| c.stage != crate::squad::ClearStage::Done).and_then(|c| c.cur.as_ref())
            .map(|x| match &x.room { Some(room) if !room.is_empty() => with_the_room(room), _ => door_name(&x.door) }),
        check_eta: Some(if b.roll_idle() { eta_words(next_at - g.time) } else { String::from("any second now") }),
        minutes: if r.checks > 0.0 { Some(num_word(m::max(1.0, m::round((g.time - r.t0) / 60.0)))) } else { None },
        n: Some(num_word(staff)),
        up: Some(num_word(m::max(1.0, staff - down_n))),
        badge: Some(badge_of(&g.gd[gd].callsign)),
        code: b.pad.as_ref().map(|p| p.def.code.clone()),
        pad_door: b.pad.as_ref().map(|p| door(g, p.door)),
    }
}

/// What a line may ask to be true.
fn facts_of(g: &Game, gd: GuardId, b: &About) -> DialogueCtxFacts {
    use crate::game::WitnessSawActKind as Saw;
    let drew = crate::weapons::firearm_spent(g, g.gd[gd].weapon) > 0.0;
    let w = &g.gd[gd].witness;
    let (saw, body, called) = (w.saw_act.as_ref().map(|x| x.kind), w.saw_body.is_some(), w.called_to_body.is_some());
    let held = if b.held_since >= 0.0 { g.time - b.held_since } else { 0.0 };
    DialogueCtxFacts {
        floor_up: Some(b.level >= 1.0), lockdown: Some(b.level >= 2.0), episode: Some(g.alarm.episode),
        heard_small: Some(w.heard.small > 0.0 && w.heard.small_where.is_some()), heard_shots: Some(w.heard.shots > 0.0), heard_bang: Some(w.heard.kick + w.heard.bang > 0.0),
        body_found: Some(body), body_called: Some(called && !body), body_known: Some(body || called),
        missing_known: Some(b.missing.is_some()), blackout: Some(g.light_home.mains.active), held_long: Some(held >= HELD_LONG_SECS),
        rollcall_live: Some(!b.roll_idle()), dazzled: Some(w.dazzled > 0.0),
        saw_shot: Some(saw == Some(Saw::Shot)), saw_drop: Some(saw.is_some_and(|k| k != Saw::Shot && k != Saw::Grab)),
        alert_now: Some(g.gd[gd].state == crate::game::GuardState::Alert), drew_on_you: Some(drew), partner_known: Some(b.partner.is_some()),
        shift: Some(b.shift), office: Some(b.office),
    }
}

/// What he could give up: `live`, true of the world now (and only a man on the shift has any of it); `known`, he could plausibly know it.
fn intel_of(g: &Game, gd: GuardId, b: &About, t: crate::game::Temperament, k: crate::dialogue::Knowledge) -> Vec<IntelItem> {
    use IntelKind as I;
    let raised = heard_on_net(b.held_since, g.alarm.raised_at);
    let checks = b.checker == Some(gd);
    let item = |kind: IntelKind, live: bool, known: bool| IntelItem { kind, text: intel_text(kind, t, k), live: live && b.shift, known };
    let clearing = b.level >= 2.0 && g.clearing.as_ref().is_some_and(|c| c.stage != crate::squad::ClearStage::Done && c.a != gd && c.b != Some(gd));
    let mut all = vec![
        item(I::CheckOn, b.check.is_some_and(|c| c.who != gd), b.check.is_some_and(|c| checks || heard_on_net(b.held_since, c.t0))),
        item(I::Clearing, clearing, raised),
        item(I::RollcallLive, !b.roll_idle() && b.roll.caller.is_some_and(|c| c != gd), true),
        item(I::Blackout, g.light_home.mains.active, true),
        item(I::Door, b.keyed.is_some_and(|d| g.gd[gd].keys.has(g.dr[d].lock_id.clone())), true),
        item(I::Code, b.pad.as_ref().is_some_and(|p| !g.pl[g.player].codes.has(p.def.name.clone())), true),
        item(I::Rollcall, g.ai_enabled && b.partner.is_some(), true),
        item(I::LockdownPosts, b.level >= 2.0 && b.post_man.is_some(), raised || b.post_man == Some(gd)),
        item(I::Routes, b.partner.is_some(), true),
        item(I::Missing, b.missing.is_some() && b.level < 2.0, b.missing.is_some()),
        item(I::Drive, g.mission.pulled && g.mission.sweep_at < 0.0 && g.level.errand.told, true),
    ];
    if checks { all[0].text = check_on_self(); }
    if !b.office { all[10].text = String::from(drive_away_text(t)); }
    all
}

// ---------------------------------------------------------------- the picker (pure over a DialogueCtx + the man's Talk state)
/// The words for what was taken, off the office.
fn drive_away_text(t: crate::game::Temperament) -> &'static str {
    match t {
        crate::game::Temperament::Hard => "They know it's gone. Somebody's on his way to look.",
        _ => "They already know it's gone. Somebody's been sent to look.",
    }
}
/// What a voice puts in front of a given fact: nothing at all for a folder (""), None for a voice with no lead of its own.
fn lead_of_shape(shape: ShapeKey) -> Option<&'static str> {
    match shape {
        ShapeKey::Folder => Some(""),
        ShapeKey::Ramble => Some("—okay— "),
        ShapeKey::Fan => Some("Oh— and "),
        ShapeKey::Con => Some("Sir: "),
        _ => None,
    }
}
/// `!!facts[n]`
fn hold_fact(f: &DialogueCtxFacts, n: Need) -> bool {
    let v = match n {
        Need::Shift => f.shift, Need::RollcallLive => f.rollcall_live, Need::Blackout => f.blackout, Need::BodyFound => f.body_found, Need::FloorUp => f.floor_up, Need::Lockdown => f.lockdown, Need::Episode => f.episode,
        Need::HeardSmall => f.heard_small, Need::HeardShots => f.heard_shots, Need::HeardBang => f.heard_bang, Need::BodyCalled => f.body_called, Need::BodyKnown => f.body_known, Need::MissingKnown => f.missing_known,
        Need::HeldLong => f.held_long, Need::Dazzled => f.dazzled, Need::SawShot => f.saw_shot, Need::SawDrop => f.saw_drop, Need::AlertNow => f.alert_now, Need::DrewOnYou => f.drew_on_you, Need::PartnerKnown => f.partner_known, Need::Office => f.office,
    };
    v == Some(true)
}
/// `String(s)` of a stage: '1', '2', '3', 'x'
fn stage_str(s: StageKey) -> String {
    match s { StageKey::X => String::from("x"), StageKey::Num(n) => js::num_str(n) }
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
/// expand {tokens} from the table ({Name} capitalises); null if any comes back empty
pub(super) fn expand(text: &str, tokens: &JsMap<String, Option<String>>) -> Option<String> {
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
pub(super) fn stage_key_of(n: f64) -> crate::dialogue::StageKey {
    if n <= 1.0 { StageKey::Num(1.0) } else if n == 2.0 { StageKey::Num(2.0) } else if n == 3.0 { StageKey::Num(3.0) } else { StageKey::X }
}
/// Is stage `s` one this man GIVES the useful thing at?
pub(super) fn giving_stage(t: crate::game::Temperament, band: crate::dialogue::Band, s: crate::dialogue::StageKey, stage_n: f64, given_none: bool) -> bool {
    let gives = gives_at(t, band);
    gives.contains(&s) || (s == StageKey::X && given_none && !gives.is_empty() && stage_n == 4.0)
}
fn line_matches(l: &crate::dialogue::Line, k: crate::dialogue::Knowledge, band: crate::dialogue::Band, t: crate::game::Temperament, s: crate::dialogue::StageKey, use_t: bool, use_a: bool) -> bool {
    if l.s != s { return false; }
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
fn needs_hold(l: &crate::dialogue::Line, facts: &crate::dialogue::DialogueCtxFacts) -> bool {
    let only_ok = match l.only { None => true, Some(LineOnly::Shift) => hold_fact(facts, Need::Shift), Some(LineOnly::Office) => hold_fact(facts, Need::Office) };
    only_ok && l.needs.as_ref().map_or(true, |ns| ns.iter().all(|n| hold_fact(facts, *n)))
}
/// The pending useful things: true now, known to him, not yet given, in the order he would give them.
pub(super) fn pending_intel(g: &crate::game::Game, ctx: &crate::dialogue::DialogueCtx, talk: GameTalkId, sc: Option<&crate::dialogue::Script>) -> Vec<crate::dialogue::PendingIntelItem> {
    let mut out: Vec<PendingIntelItem> = Vec::new();
    let kinds = intel_kinds();
    let order: Vec<IntelKind> = match sc.and_then(|x| x.pin) {
        Some(pin) => { let mut o = vec![pin]; o.extend(kinds.iter().copied().filter(|k| *k != pin)); o }
        None => kinds.to_vec(),
    };
    let tokens = ctx_tokens_map(&ctx.tokens);
    for kind in order {
        let Some(it) = ctx.intel.iter().find(|i| i.kind == kind) else { continue; };
        if !it.live || !it.known || g.tab.game_talk[talk].given.holds(kind.as_str()) { continue; }
        let own = kind == IntelKind::CheckOn && it.text == check_on_self();
        let shaped = match sc {
            Some(x) if !own => shaped_intel_text(kind, &ShapedIntelTextSc { id: x.id.clone(), shape: x.shape }, ctx.t),
            _ => None,
        };
        let lead = if shaped.as_deref().map_or(false, |s| !s.is_empty()) { String::new() } else {
            let shape_lead = match sc { Some(x) if ctx.t != crate::game::Temperament::Hard => lead_of_shape(x.shape), _ => None };
            match shape_lead { Some(s) => String::from(s), None => intel_lead(ctx.t, ctx.k) }
        };
        let fact = run_on(&lead, shaped.as_deref().unwrap_or(it.text.as_str()));
        let Some(text) = expand(&alloc::format!("{}{}", lead, fact), &tokens) else { continue; };
        out.push(PendingIntelItem { kind, text });
    }
    out
}
/// The first pending useful thing, or null.
pub(super) fn intel_for(g: &crate::game::Game, ctx: &crate::dialogue::DialogueCtx, talk: GameTalkId, sc: Option<&crate::dialogue::Script>) -> Option<crate::dialogue::IntelFor> {
    pending_intel(g, ctx, talk, sc).into_iter().next().map(|x| IntelFor { kind: x.kind, text: x.text })
}
/// The rambler's stream swallows a giving stage, when exactly ONE thing is pending.
pub(super) fn swallowed(g: &crate::game::Game, sc: Option<&crate::dialogue::Script>, s: crate::dialogue::StageKey, ctx: &crate::dialogue::DialogueCtx, talk: GameTalkId) -> bool {
    match sc {
        Some(x) => x.shape == ShapeKey::Ramble && RAMBLE_SWALLOWS > 0.0 && s == StageKey::Num(2.0) && ctx.t == crate::game::Temperament::Nervy && pending_intel(g, ctx, talk, sc).len() == 1,
        None => false,
    }
}
/// `sayable`: a rejoinder as it would be said, or null
fn sayable(text: Option<&str>, tokens: &JsMap<String, Option<String>>) -> Option<String> {
    match text { Some(x) if !x.is_empty() && REJOINDERS => expand(x, tokens), _ => None }
}
/// `eligible`: the pool for this cell and stage: (index in the lines, the text, its weight)
fn eligible(g: &Game, lines: &[Line], ctx: &DialogueCtx, tokens: &JsMap<String, Option<String>>, talk: GameTalkId, s: StageKey, use_t: bool, use_a: bool, allow_said: bool) -> Vec<(usize, String, f64)> {
    let mut out: Vec<(usize, String, f64)> = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        if !line_matches(l, ctx.k, ctx.band, ctx.t, s, use_t, use_a) || !needs_hold(l, &ctx.facts) { continue; }
        if !allow_said && g.tab.game_talk[talk].said.has(l.id.clone()) { continue; }
        let said = said_text(&SaidTextL { text: l.text.clone(), away: l.away.clone(), bare: l.bare.clone() }, ctx);
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
/// One exchange: advance the man's stage, pick the line for (his cell × that stage), mark it said.
fn pick_line(g: &mut crate::game::Game, ctx: &mut crate::dialogue::DialogueCtx, talk: GameTalkId, rnd: &mut dyn FnMut(&mut crate::game::Game) -> f64) -> crate::dialogue::DialoguePick {
    let sc0 = script_for(g, ctx, talk);
    let stage_n = g.tab.game_talk[talk].stage + 1.0;
    let s = stage_key_of(stage_n);
    {
        let time = ctx.time;
        let row = &mut g.tab.game_talk[talk];
        row.stage = stage_n; row.presses += 1.0; row.last_t = time;
    }
    let sc = sc0.filter(|x| stage_n <= 4.0 && script_live(x, ctx));
    let beat = sc.as_ref().map(|x| beat_for(x, s, ctx));
    let tokens = ctx_tokens_map(&ctx.tokens);
    let base = DialoguePick { text: String::new(), id: String::new(), kind: None, stage_n, after: None };
    let stage_word = stage_str(s);
    // the giving stages: the INTEL generator first
    let given_none = g.tab.game_talk[talk].given.is_empty();
    if giving_stage(ctx.t, ctx.band, s, stage_n, given_none) && !swallowed(g, sc.as_ref(), s, ctx, talk) {
        if let Some(it) = intel_for(g, ctx, talk, sc.as_ref()) {
            let id = alloc::format!("intel:{}", it.kind.as_str());
            let row = &mut g.tab.game_talk[talk];
            row.given.add(String::from(it.kind.as_str()));
            row.said.add(id.clone());
            row.last = Some(crate::game::TalkLast { id: id.clone() });
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
            row.last = Some(crate::game::TalkLast { id: id.clone() });
            return DialoguePick { text: h.text, id, after: sayable(b.after.as_deref(), &tokens), ..base };
        }
    }
    // the pool for this cell and stage: exact, else any temperament, else any agitation too; never another man's knowledge
    let lines = crate::dialogue::lines();
    let chain: [(bool, bool, bool); 4] = if s == StageKey::X {
        [(true, true, false), (true, true, true), (false, true, true), (false, false, true)]
    } else {
        [(true, true, false), (false, true, false), (false, false, false), (false, false, true)]
    };
    let mut pool: Vec<(usize, String, f64)> = Vec::new();
    for (use_t, use_a, allow_said) in chain {
        pool = eligible(g, &lines, ctx, &tokens, talk, s, use_t, use_a, allow_said);
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
        g.tab.game_talk[talk].last = Some(crate::game::TalkLast { id: id.clone() });
        return DialoguePick { text: String::from("…"), id, ..base };
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
    // ctx.recent is a copy of the game's table, so the write goes to both
    if let Some(rec) = ctx.recent.as_mut() {
        rec.set(l.id.clone(), ctx.time);
        g.m.dialogue.recent_by_game.get_or_insert_with(JsMap::new).set(l.id.clone(), ctx.time);
    }
    g.tab.game_talk[talk].last = Some(crate::game::TalkLast { id: l.id.clone() });
    DialoguePick { text, id: l.id.clone(), ..base }
}
/// The whole exchange for a real man: gather → pick; null for a man who cannot talk.
pub fn interrogate(g: &mut crate::game::Game, gd: GuardId, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) -> Option<crate::dialogue::DialoguePick> {
    if crate::condition::down(g, g.gd[gd].ch) || !g.guards.contains(&gd) { return None; }
    let talk = g.gd[gd].talk;
    let time = g.time;
    let unheld = g.gd[gd].held.is_none();
    let row = &mut g.tab.game_talk[talk];
    if unheld && (row.held_since < 0.0 || (row.last_t >= 0.0 && time - row.last_t > HOLD_LAPSE_SECS)) { row.held_since = time; }
    let mut ctx = gather(g, gd);
    let pick = match rnd {
        Some(r) => pick_line(g, &mut ctx, talk, r),
        None => pick_line(g, &mut ctx, talk, &mut |g: &mut Game| crate::dice::draw_of(g, "dialogue::interrogate#1", gd)),
    };
    let player = g.player;
    let holds_him = g.pl[player].holding().map_or(false, |h| h.g == gd);
    if let Some(after) = pick.after.as_ref().filter(|a| !a.is_empty()) {
        // (the cap counts UTF-16 units: '…' and '—' are one each, not three bytes)
        if holds_him && after.encode_utf16().count() as f64 <= SAM_MAX_CHARS {
            let id = alloc::format!("{}+", pick.id);
            g.pl[player].holding_mut().unwrap().after = Some(crate::player::HoldAfter { text: after.clone() });
            let mem = sam_memory_of(g);
            mem.said.set(id.clone(), time);
            mem.last = Some(id);
        }
    }
    Some(pick)
}
/// The VERB on the held man's question row for the next exchange.
pub fn prompt_for(g: &mut crate::game::Game, gd: GuardId) -> crate::dialogue::PromptFor {
    let talk = g.gd[gd].talk;
    let next = stage_key_of(g.tab.game_talk[talk].stage + 1.0);
    if next == StageKey::Num(1.0) { return PromptFor { text: String::from("make him talk"), spent: false }; }
    let mut ctx = gather(g, gd);
    if peek_give(g, &mut ctx, talk).is_some() { return PromptFor { text: String::from("ask what he knows"), spent: false }; }
    if next == StageKey::Num(2.0) { PromptFor { text: String::from("lean on him"), spent: false } }
    else if next == StageKey::Num(3.0) { PromptFor { text: String::from("ask what he knows"), spent: false } }
    else { PromptFor { text: String::from("he's got nothing left"), spent: true } }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn table(rows: &[(&str, Option<&str>)]) -> JsMap<String, Option<String>> {
        let mut t: JsMap<String, Option<String>> = JsMap::default();
        for (k, v) in rows { t.set(String::from(*k), v.map(String::from)); }
        t
    }
    /// A FAILURE MEANS: a man says "the key's in ." because a fact he would name is not there: such a line is not said at all.
    #[test]
    fn a_line_with_a_fact_missing_is_not_said_and_a_capital_token_says_it_capital() {
        let t = table(&[("room", Some("the lobby")), ("door", None), ("code", Some(""))]);
        assert_eq!(expand("in {room}. {Room}!", &t).as_deref(), Some("in the lobby. The lobby!"));
        assert_eq!(expand("{door}", &t), None);
        assert_eq!(expand("{code}", &t), None);
        assert_eq!(expand("{nobody}", &t), None);
        assert_eq!(expand("— {x y} {} { …", &t).as_deref(), Some("— {x y} {} { …"));
    }
    /// A FAILURE MEANS: a man says "the the yard", "the block 2", "in 15 minutes" or a number with no sense of time in it.
    #[test]
    fn a_man_says_a_place_with_its_article_a_number_in_words_and_a_time_roughly() {
        assert_eq!(with_the("yard"), "the yard");
        assert_eq!(with_the("the yard"), "the yard");
        assert_eq!(with_the("block 2"), "block 2");
        assert_eq!(num_word(3.0), "three");
        assert_eq!(num_word(15.0), "fifteen");
        assert_eq!(num_word(16.0), "16");
        assert_eq!(eta_words(20.0), "any second now");
        assert_eq!(eta_words(60.0), "in a minute or so");
        assert_eq!(eta_words(300.0), "in five minutes");
        assert_eq!(eta_words(901.0), "before long");
    }
    /// A FAILURE MEANS: a line is picked for a fact the man does not have, or its token reads another's value.
    #[test]
    fn a_mans_facts_reach_the_picker_under_their_own_names_and_only_true_holds() {
        let mut t = DialogueCtxTokens::default();
        t.saw_where = Some(String::from("the yard"));
        t.pad_door = Some(String::from("the pad door"));
        let tab = ctx_tokens_map(&t);
        assert_eq!(expand("{SawWhere}, {padDoor}", &tab).as_deref(), Some("The yard, the pad door"));
        assert_eq!(expand("{clearRoom}", &tab), None);
        let mut f = DialogueCtxFacts::default();
        f.office = Some(true);
        f.saw_drop = Some(false);
        assert!(hold_fact(&f, Need::Office));
        assert!(!hold_fact(&f, Need::SawDrop));
        assert!(!hold_fact(&f, Need::Shift));
    }
    /// A FAILURE MEANS: the questioning does not run opener, press, ask, then dry.
    #[test]
    fn the_exchanges_are_three_stages_and_then_dry() {
        assert!(matches!(stage_key_of(0.0), StageKey::Num(x) if x == 1.0));
        assert!(matches!(stage_key_of(3.0), StageKey::Num(x) if x == 3.0));
        assert!(matches!(stage_key_of(4.0), StageKey::X));
    }
    /// A FAILURE MEANS: a man who walks a loop is far from its closing leg.
    #[test]
    fn route_distance_wraps_the_loop() {
        let pts = [[0.0, 0.0, 0.0], [4.0, 0.0, 0.0], [4.0, 0.0, 4.0]];
        assert_eq!(dist_to_route(&pts, [2.0, 0.0, -3.0]), 3.0);
        assert!((dist_to_route(&pts, [1.0, 0.0, 3.0]) - 2.0f64.sqrt()).abs() < 1e-9, "nearest the leg from the last point back to the first");
        assert_eq!(dist_to_route(&[], [0.0, 0.0, 0.0]), f64::INFINITY);
    }
    /// A FAILURE MEANS: a man you hold knows what the net said after you took his radio.
    #[test]
    fn a_held_man_hears_the_net_only_until_half_a_second_after_you_took_him() {
        assert!(heard_on_net(-1.0, 5.0));
        assert!(heard_on_net(10.0, 10.5));
        assert!(!heard_on_net(10.0, 10.6));
        assert!(!heard_on_net(-1.0, -1.0));
    }
}
