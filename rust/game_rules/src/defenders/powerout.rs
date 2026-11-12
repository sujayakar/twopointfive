//! The lights go out and somebody goes to put them back on: a room fight's defenders notice, say what they do on their net, and walk to the breaker.

pub use ::game_types::powerout::types;
pub use types::*;
use crate::v3::dist_xz;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

// ---------------------------------------------------------------- small private helpers

/// the state in its slot: the rule's passes run only with it there.
fn st(g: &mut Game) -> &mut State {
    g.m.powerout.states.as_mut().unwrap()
}
/// a man of the party, by his place in `st.men`
fn man(g: &mut Game, i: usize) -> &mut Man {
    &mut st(g).men[i]
}
/// `g.guards[i]` for an index kept in `seen` (−1 or off the end of the list: nobody)
fn guard_at(g: &Game, i: f64) -> Option<GuardId> {
    if i < 0.0 { None } else { g.guards.get(i as usize).copied() }
}
// ---------------------------------------------------------------- the rule's small questions

/// his id's share of 0‥1, no dice.
fn id_share(g: &crate::game::Game, gd: GuardId) -> f64 {
    v3::frac(g.char_id(gd) * 0.6180339887)
}

/// a living man who has not fled the level.
fn alive(g: &crate::game::Game, gd: GuardId) -> bool {
    !crate::condition::down(g, g.gd[gd].ch) && g.ch[g.gd[gd].ch].alive && !g.gd[gd].fled
}

/// alive, nobody's arm on him, his wits about him.
fn fit(g: &crate::game::Game, gd: GuardId) -> bool {
    alive(g, gd) && !crate::control::man::held(g, gd) && !crate::control::man::blind(g, gd) && !crate::control::man::staggered(g, gd)
}

pub use crate::roomfight::def_of;

/// the rule's men: the room fight's defenders on the other side from the player, in live play.
fn ruled(g: &crate::game::Game) -> Vec<RoomfightDefId> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < g.fight.defs.len() {
        let d = g.fight.defs[i];
        i += 1;
        let gd = g.tab.roomfight_def[d].gd;
        if g.team(gd).theirs() && g.gd[gd].unit.is_some() {
            out.push(d);
        }
    }
    out
}

/// a line of the timeline.
fn event(g: &mut crate::game::Game, what: crate::powerout::PowerOutEventWhat, gd: Option<GuardId>, why: &str) {
    let who: String = match gd {
        Some(id) => g.gd[id].callsign.clone(),
        None => "-".into(),
    };
    let t = g.time;
    let s = st(g);
    s.events.push(PowerOutEvent { t, what, who, why: why.into() });
    if s.events.len() > 120 {
        s.events.drain(0..40);
    }
}

/// a mains-fed fixture that was burning, in his plain view inside `seeM`.
fn saw_them_die(g: &crate::game::Game, gd: GuardId) -> bool {
    let c = g.pos_of(gd);
    let eye: V3 = [c[0], 1.5, c[2]];
    let mut i = 0;
    while i < g.targets.len() {
        let t = g.targets[i];
        i += 1;
        if !g.tab.fixture[t].mains
            || !g.tab.fixture[t].on
            || g.tab.fixture[t].broken
            || !(g.tab.fixture[t].factor > 0.3)
            || g.tab.fixture[t].kind == crate::fixtures::FixtureKind::Breaker
            || g.tab.fixture[t].kind == crate::fixtures::FixtureKind::Device
        {
            continue;
        }
        let tp = g.tab.fixture[t].pos;
        if dist_xz(tp, g.pos_of(gd)) > g.k.powerout.see_m {
            continue;
        }
        if !crate::scene_collision::segment_blocked(g, eye, [tp[0], m::max(1.2, tp[1] - 0.15), tp[2]], None, None, None) {
            return true;
        }
    }
    false
}

/// the meter's reading of a point of floor, by the place's own light.
fn floor_at(g: &mut Game, x: f64, z: f64) -> f64 {
    let e = crate::guards::light_estimate_at(g, [x, 1.2, z], Some(&mut |_g: &Game, _owner: f64| true), Some(true), None);
    crate::game::game_visibility_of(g, e)
}

/// the brightest floor he stands on, watches or has about him, by the place's own light.
fn floor_watched(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    let k = g.k.powerout;
    let c = g.pos_of(gd);
    let yaw = g.ch[g.gd[gd].ch].aim_yaw;
    let eye: V3 = [c[0], 1.5, c[2]];
    let mut best = floor_at(g, c[0], c[2]);
    for off in [0.0, -k.fan_rad, k.fan_rad] {
        let fx = m::sin(yaw + off);
        let fz = m::cos(yaw + off);
        let mut r = k.step_m;
        while r <= k.look_m + 1e-6 {
            let x = eye[0] + fx * r;
            let z = eye[2] + fz * r;
            if crate::scene_collision::segment_blocked(g, eye, [x, 1.5, z], None, None, None) {
                break;
            }
            let v = floor_at(g, x, z);
            best = m::max(best, v);
            r += k.step_m;
        }
    }
    // (… and the floor about him, whichever way he faces)
    let mut n = 0;
    while n < 8 {
        let a = (n as f64) * m::PI / 4.0;
        let fx = m::sin(a);
        let fz = m::cos(a);
        let mut r = k.step_m;
        while r <= k.about_m + 1e-6 {
            let x = eye[0] + fx * r;
            let z = eye[2] + fz * r;
            if crate::scene_collision::segment_blocked(g, eye, [x, 1.5, z], None, None, None) {
                break;
            }
            let v = floor_at(g, x, z);
            best = m::max(best, v);
            r += k.step_m;
        }
        n += 1;
    }
    best
}

/// why a defender is not free to go (None = he is): calm, on a post the room fight gave him, and nobody else's.
pub fn power_out_kept(g: &mut crate::game::Game, d: RoomfightDefId) -> Option<String> {
    let gd = g.tab.roomfight_def[d].gd;
    if let Some(who) = crate::owner::kept(g, gd, crate::owner::by::BREAKER) { return Some(who.into()); }
    if !crate::roomfight::at_post(g, d) { return Some("on the move".into()); }
    if g.gd[gd].script.is_some() { return Some("on a script".into()); }
    if crate::enemysquads::roused(g, gd) { return Some("in a fight".into()); }
    if crate::mission::lies_in_wait(g, gd) || crate::mission::captor(g) == Some(gd) { return Some("the captor / lying in wait".into()); }
    if crate::gunroles::gun_keeps_post(g, gd) { return Some("his gun keeps its post".into()); }
    if crate::pressure::last_holds(g, d) { return Some("the last of them keeps his room".into()); }
    None
}

/// where the man at the lever stands.
fn lever_spot(g: &mut crate::game::Game) -> V3 {
    let e = crate::workthings::breaker_errand(g);
    let bp = g.level.breaker.pos;
    match e {
        Some(e) => e.goal,
        None => [bp[0] - 0.4, 0.0, bp[2] - 0.9],
    }
}

/// the way from a man to a spot as the walk grid stands, and its length; None = none that ends there.
fn way_to(g: &crate::game::Game, from: V3, to: V3) -> Option<crate::powerout::WayTo> {
    if dist_xz(from, to) < 0.5 {
        return Some(WayTo { path: vec![to], l: dist_xz(from, to) });
    }
    let path = crate::scene_collision::nav_grid_find_path(g, from, to, Some(20000.0), None)?;
    if dist_xz(*path.last()?, to) > 1.0 { return None; }
    Some(WayTo { l: crate::places::walk_len(from, &path), path })
}

/// the second man's place: free floor `coverM` short of the lever with a line to it, nearest the way they came; and what he watches from it.
fn cover_spot(g: &mut crate::game::Game, gd: GuardId, lever: V3, way: &[V3], from: V3) -> crate::powerout::CoverSpot {
    let [r0, r1] = g.k.powerout.cover_m;
    // the point of the way about six metres before its end: where anybody following them would come from
    let mut back = from;
    let mut left = 6.0;
    let mut i = way.len() as isize - 1;
    while i > 0 && left > 0.0 {
        let u = i as usize;
        left -= dist_xz(way[u], way[u - 1]);
        back = way[u - 1];
        i -= 1;
    }
    let mut best: Option<V3> = None;
    let mut bd = m::INF;
    let mut r = r0;
    while r <= r1 + 1e-6 {
        let mut k = 0;
        while k < 12 {
            let a = (k as f64) * m::PI / 6.0;
            k += 1;
            let p: V3 = [lever[0] + m::sin(a) * r, 0.0, lever[2] + m::cos(a) * r];
            if !crate::places::roomy(g, p) || crate::places::taken(g, gd, p, 0.9) { continue; }
            if crate::scene_collision::segment_blocked(g, [p[0], 1.2, p[2]], [lever[0], 1.2, lever[2]], None, None, None) {
                continue;
            }
            let d = dist_xz(p, back);
            if d < bd {
                bd = d;
                best = Some(p);
            }
        }
        r += 0.8;
    }
    CoverSpot { spot: best.unwrap_or(lever), watch: back }
}

/// out on the party's errand, at the pace of a man going to look: there, he stands there.
fn walk(g: &mut crate::game::Game, d: RoomfightDefId, to: V3) {
    let pace = g.k.tune.guard_investigate;
    crate::roomfight::send(g, d, crate::roomfight::Leg { stays: true, ..crate::roomfight::Leg::errand(to, pace) }, crate::owner::by::BREAKER);
}

// ---------------------------------------------------------------- the trip

/// a trip that found the mains up: the answer's clock starts.
pub fn power_out(g: &mut crate::game::Game) {
    decide_out(g);
    breaker_say(g);
}

/// who knows the lights have gone, and when their word is due.
fn decide_out(g: &mut crate::game::Game) {
    let defs = ruled(g);
    if defs.is_empty() {
        return;
    }
    let mut first: Option<GuardId> = None;
    for &d in &defs {
        let gd = g.tab.roomfight_def[d].gd;
        if fit(g, gd) && saw_them_die(g, gd) {
            first = Some(gd);
            break;
        }
    }
    let prev = g.m.powerout.states.take();
    let out = prev.as_ref().map_or(false, |p| !p.men.is_empty()); // (a party of the last blackout still out: it is this one's answer too)
    let mut new_st = State {
        since: g.time,
        decide_at: m::INF,
        go_at: if out { prev.as_ref().unwrap().go_at } else { -1.0 },
        way_m: if out { prev.as_ref().unwrap().way_m } else { 0.0 },
        overdue_at: if out { prev.as_ref().unwrap().overdue_at } else { -1.0 },
        fell: if out { prev.as_ref().unwrap().fell } else { None },
        seen: Vec::new(),
        look_at: -1.0,
        decided: out,
        word: prev.as_ref().map_or(false, |p| p.word),
        parties: prev.as_ref().map_or(0.0, |p| p.parties),
        lost: prev.as_ref().map_or(false, |p| p.lost),
        men: Vec::new(),
        events: Vec::new(),
    };
    if let Some(p) = prev {
        new_st.men = p.men;
        new_st.events = p.events;
    }
    g.m.powerout.states = Some(new_st);
    if out {
        let mut i = 0;
        while i < st(g).men.len() {
            let gd = man(g, i).gd;
            let at = i;
            i += 1;
            let Some(d) = def_of(g, gd) else { continue };
            if man(g, at).phase != Phase::Back || !crate::roomfight::on_leg_of(g, d, crate::owner::by::BREAKER) {
                continue;
            }
            man(g, at).phase = Phase::Out;
            let goal = man(g, at).goal;
            walk(g, d, goal);
            event(g, PowerOutEventWhat::Go, Some(gd), "the lights go again on his way home: he turns back for the lever");
        }
        return;
    }
    if let Some(first) = first {
        return noticed(g, first, "the lights die in his view");
    }
    // (the lights are still burning this frame: what each man's floor reads now is read again in the dark)
    for &d in &defs {
        let gd = g.tab.roomfight_def[d].gd;
        if !fit(g, gd) {
            continue;
        }
        let lit = floor_watched(g, gd);
        if lit >= g.k.powerout.lit_at {
            let i = g.guards.iter().position(|&x| x == gd).map_or(-1.0, |p| p as f64);
            st(g).seen.push(StateSeenItem { i, lit });
        }
    }
    if !st(g).seen.is_empty() {
        let at = g.time + g.k.powerout.dark_after;
        st(g).look_at = at;
    } else {
        nobody(g);
    }
}

/// somebody knows: their word's clock, by his id.
fn noticed(g: &mut crate::game::Game, gd: GuardId, how: &str) {
    let k = g.k.powerout;
    let share = id_share(g, gd);
    let time = g.time;
    let s = st(g);
    s.seen = Vec::new();
    s.look_at = -1.0;
    s.decide_at = s.since + k.think[0] + (k.think[1] - k.think[0]) * share;
    let why = alloc::format!("{}: their word is due {} s on", how, js::to_fixed(s.decide_at - time, 1));
    event(g, PowerOutEventWhat::Noticed, Some(gd), &why);
}

/// nobody knows the power is out.
fn nobody(g: &mut crate::game::Game) {
    let s = st(g);
    s.seen = Vec::new();
    s.look_at = -1.0;
    s.decided = true;
    event(g, PowerOutEventWhat::Nobody, None, "no defender had a burning mains fixture in his view or lost the light on his floor: nobody knows the power is out");
}

/// Is the walk to the breaker this rule's to hand out, so the stock errand passes this man over?
pub fn power_out_rules(g: &crate::game::Game, gd: GuardId) -> bool {
    if g.gd[gd].unit.is_none() {
        return false;
    }
    let defs = ruled(g);
    defs.iter().any(|&d| g.tab.roomfight_def[d].gd == gd)
}

/// A man out for the lights burns his own while the mains are down.
pub fn power_out_lit(g: &crate::game::Game, gd: GuardId) -> bool {
    if !g.light_home.mains.active {
        return false;
    }
    let stepped = match g.m.powerout.states.as_ref().and_then(|s| s.men.iter().find(|x| x.gd == gd)) {
        Some(x) => x.phase != Phase::Wait,
        None => false,
    };
    stepped && crate::darkfoe::carries_dark_lamp(g, gd)
}

/// Their side holds this dark for the team's doing.
pub(super) fn power_out_word(g: &crate::game::Game, gd: GuardId) -> bool {
    if !g.light_home.mains.active || !g.m.powerout.states.as_ref().map_or(false, |s| s.word) {
        return false;
    }
    g.team(gd).theirs()
}

/// who the breaker's party has, on the one record: out for the lights, or waiting to be sent.
fn breaker_say(g: &mut crate::game::Game) {
    let rule = crate::owner::by::BREAKER;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        let t = g.time;
        if g.m.powerout.states.as_ref().map_or(false, |s| s.men.iter().any(|x| x.gd == gd)) {
            crate::owner::claim(g, gd, rule, t, None);
        } else {
            crate::owner::release(g, gd, rule, t);
        }
    }
}

// ---------------------------------------------------------------- their word

/// nobody goes: they say so.
fn stay(g: &mut Game, speaker: GuardId, line: &str, why: &str, word: bool) {
    let s = st(g);
    s.word = s.word || word;
    crate::game::game_say(g, speaker, line, Some(true), None);
    event(g, PowerOutEventWhat::Stay, Some(speaker), why);
}

/// their word: who goes, or why nobody does.
fn decide(g: &mut crate::game::Game, defs: &[RoomfightDefId]) {
    st(g).decided = true;
    let live: Vec<RoomfightDefId> = defs.iter().copied().filter(|&d| alive(g, g.tab.roomfight_def[d].gd)).collect();
    let Some(speaker) = who_speaks(g, &live) else { return };
    if takes_it_for_the_team(g, &live, speaker) {
        return;
    }
    let lever = lever_spot(g);
    let free: Vec<RoomfightDefId> = live.iter().copied().filter(|&d| power_out_kept(g, d).is_none()).collect();
    let own = man_of_the_breakers_room(g, &free, lever);
    let own_goes = own.is_some();
    let go = match own {
        Some(x) => vec![x],
        None => men_to_spare(g, &free, &live, lever),
    };
    if go.is_empty() {
        let why = if !free.is_empty() {
            alloc::format!("no way to the breaker inside {} m for a man who is free", js::num_str(g.k.powerout.max_m))
        } else {
            String::from("no man of theirs is free to go")
        };
        return stay(g, speaker, "power's out — nobody to spare. Stay on your doors!", &why, false);
    }
    name_the_party(g, &go, lever);
    say_who_goes(g, &go, own_goes);
}

/// A fit man who saw the lights die, else the first fit man.
fn who_speaks(g: &Game, live: &[RoomfightDefId]) -> Option<GuardId> {
    let men: Vec<GuardId> = live.iter().map(|&d| g.tab.roomfight_def[d].gd).collect();
    men.iter().copied().find(|&gd| fit(g, gd) && saw_them_die(g, gd)).or_else(|| men.iter().copied().find(|&gd| fit(g, gd)))
}

/// They take this dark for the team's doing, or have lost too many or sent too many for it, and say nobody goes.
fn takes_it_for_the_team(g: &mut Game, live: &[RoomfightDefId], speaker: GuardId) -> bool {
    let knows = live.iter().any(|&d| {
        let gd = g.tab.roomfight_def[d].gd;
        crate::enemysquads::roused(g, gd) || g.gd[gd].witness.heard.loud() > 0.0 || g.gd[gd].witness.alerted_by.is_some()
    });
    if knows {
        stay(g, speaker, "power's out — that's them! Stay on your doors!", "a man of theirs is roused or has heard the team: nobody walks out to mend a fuse", true);
        return true;
    }
    if crate::mission::captor(g).is_some() {
        stay(g, speaker, "power's out — that's them! Stay on your doors!", "men holding hostages take the dark for the assault (every one of them has learned of the team by it)", true);
        return true;
    }
    if st(g).lost {
        stay(g, speaker, "power's out again — nobody goes. Stay on your doors!", "the last men sent for the lights did not all come back", true);
        return true;
    }
    let parties = st(g).parties;
    if parties >= g.k.powerout.parties {
        let line = "power's out again — that's no accident. Stay on your doors!";
        let why = alloc::format!("the lights have gone {} times this fight", js::num_str(parties + 1.0));
        stay(g, speaker, line, &why, true);
        return true;
    }
    false
}

/// The men in order of how near they stand to `to`.
fn nearest_first(g: &Game, men: &mut Vec<RoomfightDefId>, to: V3) {
    js::sort_by(men, |&a, &b| dist_xz(g.pos_of(g.tab.roomfight_def[a].gd), to) - dist_xz(g.pos_of(g.tab.roomfight_def[b].gd), to));
}

/// The breaker in a defender's own room: its nearest man, alone, without leaving it.
fn man_of_the_breakers_room(g: &mut Game, free: &[RoomfightDefId], lever: V3) -> Option<(RoomfightDefId, WayTo)> {
    let r = crate::roomfight::room_fight_room_at(g, lever)?;
    let mut mine: Vec<RoomfightDefId> = free.iter().copied().filter(|&d| g.tab.roomfight_def[d].room == Some(r)).collect();
    nearest_first(g, &mut mine, lever);
    let own = mine.first().copied()?;
    let from = g.pos_of(g.tab.roomfight_def[own].gd);
    let way = way_to(g, from, lever)?;
    Some((own, way))
}

/// Of the nearest free men with a way there short enough, those whose rooms keep a man; failing that the one with the shortest way.
fn men_to_spare(g: &Game, free: &[RoomfightDefId], live: &[RoomfightDefId], lever: V3) -> Vec<(RoomfightDefId, WayTo)> {
    let mut near = free.to_vec();
    nearest_first(g, &mut near, lever);
    near.truncate(g.k.powerout.ask as usize); // (a path each is the dear part: only the nearest are asked)
    let mut ways: Vec<(RoomfightDefId, WayTo)> = Vec::new();
    for &d in &near {
        let from = g.pos_of(g.tab.roomfight_def[d].gd);
        if let Some(way) = way_to(g, from, lever) {
            if way.l <= g.k.powerout.max_m {
                ways.push((d, way));
            }
        }
    }
    js::sort_by(&mut ways, |a, b| a.1.l - b.1.l);
    let mut in_room: JsMap<Option<RoomfightRoomId>, f64> = JsMap::new();
    for &d in live {
        if crate::roomfight::at_post(g, d) {
            let room = g.tab.roomfight_def[d].room;
            let n = in_room.get(room).copied().unwrap_or(0.0);
            in_room.set(room, n + 1.0);
        }
    }
    let mut go: Vec<(RoomfightDefId, WayTo)> = Vec::new();
    for x in &ways {
        if (go.len() as f64) >= g.k.powerout.men {
            break;
        }
        let room = g.tab.roomfight_def[x.0].room;
        let n = in_room.get(room).copied().unwrap_or(0.0);
        if room.is_none() || n < 2.0 {
            continue;
        }
        in_room.set(room, n - 1.0);
        go.push(x.clone());
    }
    if go.is_empty() && !ways.is_empty() {
        go = vec![ways[0].clone()];
    }
    go
}

/// A man of the party, waiting to step off.
fn party_man(gd: GuardId, role: ManRole, goal: V3, from: V3, watch: Option<V3>) -> Man {
    Man { gd, role, phase: Phase::Wait, goal, from, watch, heard0: 0.0, there_t: -1.0 }
}

/// The first of them goes to the lever, a second to a place short of it covering him; the beat before they step off starts.
fn name_the_party(g: &mut Game, go: &[(RoomfightDefId, WayTo)], lever: V3) {
    let lead = &go[0];
    let lead_gd = g.tab.roomfight_def[lead.0].gd;
    let lead_from = crate::roomfight::station(g, lead.0);
    st(g).men.push(party_man(lead_gd, ManRole::Lever, lever, lead_from, None));
    if let Some(second) = go.get(1) {
        let from = g.pos_of(lead_gd);
        let gd = g.tab.roomfight_def[second.0].gd;
        let c = cover_spot(g, gd, lever, &lead.1.path, from);
        let station = crate::roomfight::station(g, second.0);
        st(g).men.push(party_man(gd, ManRole::Cover, c.spot, station, Some(c.watch)));
    }
    let go_at = g.time + g.k.powerout.beat;
    let s = st(g);
    s.parties += 1.0;
    s.go_at = go_at;
    s.way_m = lead.1.l;
}

/// The lead man says it on the net.
fn say_who_goes(g: &mut Game, go: &[(RoomfightDefId, WayTo)], own_goes: bool) {
    let lead = &go[0];
    let lead_gd = g.tab.roomfight_def[lead.0].gd;
    let two = go.len() > 1;
    let line = if two { "power's out — two moving to the breaker!" } else { "power's out — on my way to the breaker!" };
    crate::game::game_say(g, lead_gd, line, Some(true), None);
    let names: Vec<&str> = go.iter().map(|x| g.gd[g.tab.roomfight_def[x.0].gd].callsign.as_str()).collect();
    let why = alloc::format!(
        "{} for the breaker, {} m off by the way he walks{}; they step off in {} s",
        names.join(" and "),
        js::to_fixed(lead.1.l, 0),
        if own_goes { ", in his own room" } else if two { "" } else { ", alone" },
        js::num_str(g.k.powerout.beat)
    );
    event(g, PowerOutEventWhat::Said, Some(lead_gd), &why);
}

// ---------------------------------------------------------------- once a frame

/// once a frame, before the room fight's own tick.
pub fn power_out_tick(g: &mut crate::game::Game, dt: f64) {
    step_power_out(g, dt);
    breaker_say(g);
}

/// their word when it is due, and every man out a step on.
fn step_power_out(g: &mut crate::game::Game, _dt: f64) {
    if g.m.powerout.states.is_none() {
        return;
    }
    let defs = ruled(g);
    if defs.is_empty() {
        if !st(g).men.is_empty() {
            stand_down(g);
        }
        return;
    }
    let (decided, look_at) = (st(g).decided, st(g).look_at);
    if !decided && look_at >= 0.0 && g.time >= look_at {
        let mut hit: Option<GuardId> = None;
        let since = st(g).since;
        if g.light_home.mains.active && g.light_home.mains.since == since {
            let mut j = 0;
            while j < st(g).seen.len() {
                let s = st(g).seen[j];
                j += 1;
                let Some(gd) = guard_at(g, s.i) else { continue };
                if fit(g, gd) && floor_watched(g, gd) < s.lit * g.k.powerout.drop_k {
                    hit = Some(gd);
                    break;
                }
            }
        }
        match hit {
            Some(gd) => noticed(g, gd, "the floor he is watching goes dark"),
            None => nobody(g),
        }
    }
    let (decided, since, decide_at) = (st(g).decided, st(g).since, st(g).decide_at);
    if !decided && g.light_home.mains.active && g.light_home.mains.since == since && g.time >= decide_at {
        decide(g, &defs);
    }
    let mut fight = false;
    for &d in &defs {
        let gd = g.tab.roomfight_def[d].gd;
        if alive(g, gd) && crate::enemysquads::roused(g, gd) {
            fight = true;
            break;
        }
    }
    if st(g).men.is_empty() {
        missed(g, &defs, fight);
        return;
    }
    let go_at = st(g).go_at;
    if st(g).men.iter().any(|x| x.phase == Phase::Wait) && g.time >= go_at {
        step_off(g, fight);
    }
    // `lever` is the man's place in st.men, and it holds under the walk: st.men is only cut when the walk is over
    let lever = st(g).men.iter().position(|x| x.role == ManRole::Lever);
    let n = st(g).men.len();
    let mut keep = Vec::with_capacity(n);
    let mut i = 0;
    while i < n {
        keep.push(step_man(g, i, lever, &defs, fight));
        i += 1;
    }
    let mut at = 0;
    st(g).men.retain(|_| {
        let k = keep[at];
        at += 1;
        k
    });
}

/// every man out goes back to his post at a walk.
fn all_back(g: &mut Game) {
    let mut j = 0;
    while j < st(g).men.len() {
        let x = man(g, j).gd;
        if let Some(xd) = def_of(g, x) {
            if crate::roomfight::on_leg_of(g, xd, crate::owner::by::BREAKER) && man(g, j).phase != Phase::Wait {
                man(g, j).phase = Phase::Back;
                let from = man(g, j).from;
                walk(g, xd, from);
            }
        }
        j += 1;
    }
}

/// A man of the party out on his errand: fixed for his step.
#[derive(Clone, Copy)]
struct OnErrand {
    i: usize,
    gd: GuardId,
    d: RoomfightDefId,
    role: ManRole,
    from: V3,
    heard0: f64,
    lever: Option<usize>,
    at_lever: bool,
}

/// one man of the party a step on; false = he is off the list.
fn step_man(g: &mut Game, i: usize, lever: Option<usize>, defs: &[RoomfightDefId], fight: bool) -> bool {
    let gd = man(g, i).gd;
    let d = def_of(g, gd);
    if !alive(g, gd) {
        fell_while_out(g, i, gd);
        return false;
    }
    let Some(d) = d.filter(|_| g.gd[gd].held.is_none()) else {
        taken_while_out(g, i, gd);
        return false;
    };
    if man(g, i).phase == Phase::Wait {
        return true;
    }
    let role = man(g, i).role;
    let from = man(g, i).from;
    let heard0 = man(g, i).heard0;
    let at_lever = role == ManRole::Lever && man(g, i).phase == Phase::There;
    let o = OnErrand { i, gd, d, role, from, heard0, lever, at_lever };
    let how = crate::roomfight::leg_ended(g, d, crate::owner::by::BREAKER);
    if matches!(how, Some(crate::roomfight::LegEnd::Contact | crate::roomfight::LegEnd::Taken)) {
        // (met on the way: the room fight's man)
        event(g, PowerOutEventWhat::Abort, Some(gd), "the room fight has him where he stands");
        return false;
    }
    if sees_his_mate_down(g, o) {
        return false;
    }
    if (fight || crate::enemysquads::roused(g, gd) || g.gd[gd].witness.heard.loud() > heard0) && !(at_lever && g.light_home.mains.active && !g.light_home.mains.permanent) {
        runs_back_to_a_fight(g, o, defs);
        return false;
    }
    if man(g, i).phase != Phase::Back && !g.light_home.mains.active {
        mains_are_back_so_he_goes_home(g, o);
        return true;
    }
    let reached = how == Some(crate::roomfight::LegEnd::Arrived) || (man(g, i).phase != Phase::Back && role == ManRole::Lever && crate::lighting::breaker_reset_near(g, gd));
    if man(g, i).phase == Phase::There {
        stands_at_his_place(g, o);
        return true;
    }
    if !reached {
        return how.is_none() || finds_no_way(g, o);   // (on his way)
    }
    if man(g, i).phase == Phase::Back {
        let word = st(g).word;
        crate::roomfight::back_on(g, d, word, "powerout");
        let why = alloc::format!("back on his post {} s after the lights went", js::to_fixed(g.time - st(g).since, 1));
        event(g, PowerOutEventWhat::Home, Some(gd), &why);
        return false;
    }
    gets_to_his_place(g, o);
    true
}

/// Down while out: they have lost a man, and if he had stepped off, the place he lies is known.
fn fell_while_out(g: &mut Game, i: usize, gd: GuardId) {
    st(g).lost = true;
    if man(g, i).phase != Phase::Wait {
        let p = g.pos_of(gd);
        st(g).fell = Some(p);
    }
    event(g, PowerOutEventWhat::Lost, Some(gd), "down while he was out for the lights: nobody is sent again this fight");
}

/// In Sam's arm, or no defender of the fight's any more: whoever has him has him.
fn taken_while_out(g: &mut Game, i: usize, gd: GuardId) {
    st(g).lost = true;
    if g.gd[gd].script.is_some() && man(g, i).phase != Phase::Wait {
        let state = g.gd[gd].state;
        crate::squad::end_guard_script(g, gd, Some(state));
    }
    let why = if g.gd[gd].held.is_some() { "taken while he was out for the lights: nobody is sent again this fight" } else { "no man of the fight's any more" };
    event(g, PowerOutEventWhat::Lost, Some(gd), why);
}

/// His mate on the floor in his plain view, or he is set short of the lever, a line to it, and no mate at it: he says so and runs back to his post.
fn sees_his_mate_down(g: &mut Game, o: OnErrand) -> bool {
    let Some(fell) = st(g).fell else { return false };
    let p = g.pos_of(o.gd);
    if !((man(g, o.i).phase == Phase::There && o.lever.is_none())
        || (dist_xz(p, fell) <= g.k.powerout.see_m && !crate::scene_collision::segment_blocked(g, [p[0], 1.5, p[2]], [fell[0], 0.4, fell[2]], None, None, None)))
    {
        return false;
    }
    let s = st(g);
    s.fell = None;
    s.word = true;
    crate::game::game_say(g, o.gd, "he's down at the breaker — that's them! Stay on your doors!", Some(true), None);
    crate::roomfight::run(g, o.d, o.from, None, 0.0, crate::owner::by::BREAKER);
    event(g, PowerOutEventWhat::Abort, Some(o.gd), "sees the man he came with on the floor: says so, and back to his post at a run");
    true
}

/// A fight, or something loud in his ears: back to his post at a run, where the leg ends in the room fight's plant and he is its man.
fn runs_back_to_a_fight(g: &mut Game, o: OnErrand, defs: &[RoomfightDefId]) {
    crate::roomfight::run(g, o.d, o.from, None, 0.0, crate::owner::by::BREAKER);
    let why = if crate::enemysquads::roused(g, o.gd) {
        String::from("he is roused")
    } else if g.gd[o.gd].witness.heard.loud() > o.heard0 {
        String::from("a shot, a boot or a charge in his ears")
    } else {
        let mut who: String = "undefined".into();
        for &x in defs {
            let xg = g.tab.roomfight_def[x].gd;
            if alive(g, xg) && crate::enemysquads::roused(g, xg) {
                who = g.gd[xg].callsign.clone();
                break;
            }
        }
        alloc::format!("a fight ({} is roused)", who)
    };
    event(g, PowerOutEventWhat::Abort, Some(o.gd), &alloc::format!("{}: back to his post at a run", why));
}

/// The mains are back, by his mate's hand, the trip's own clock or anybody's: he walks home.
fn mains_are_back_so_he_goes_home(g: &mut Game, o: OnErrand) {
    man(g, o.i).phase = Phase::Back;
    walk(g, o.d, o.from);
    if o.role == ManRole::Lever || o.lever.is_none() {
        let why = alloc::format!("the mains are back {} s after they went: back to their posts", js::to_fixed(g.time - st(g).since, 1));
        event(g, PowerOutEventWhat::Restored, Some(o.gd), &why);
    }
}

/// Set at his place: the lever man works at it; the cover man watches the way they came, and goes home when the lever man does.
fn stands_at_his_place(g: &mut Game, o: OnErrand) {
    if o.role == ManRole::Lever {
        return works_the_lever(g, o);
    }
    if let Some(w) = man(g, o.i).watch {
        crate::squad::order(g, o.gd, crate::squad::OrderPatch { aim_at: Some(Some([w[0], 1.2, w[2]])), upper: Some(Some(crate::character::UpperMode::Aim)), ..Default::default() });
    }
    if o.role == ManRole::Cover && o.lever.map_or(true, |l| man(g, l).phase == Phase::Back) {
        man(g, o.i).phase = Phase::Back;
        walk(g, o.d, o.from);
    }
}

/// A lever that will not take his hand: he does not stand at it for ever. A breaker shot to scrap: he says so, once he has looked. Else he faces it.
fn works_the_lever(g: &mut Game, o: OnErrand) {
    let there_t = man(g, o.i).there_t;
    if g.light_home.mains.active && !g.light_home.mains.permanent && g.time - there_t > g.k.powerout.lever_secs {
        let why = alloc::format!("{} s at a lever he cannot get a hand to: back to their posts", js::num_str(g.k.powerout.lever_secs));
        event(g, PowerOutEventWhat::Abort, Some(o.gd), &why);
        return all_back(g);
    }
    let bp = g.level.breaker.pos;
    let p = g.pos_of(o.gd);
    let face = v3::yaw_to(p, bp);
    crate::squad::order(g, o.gd, crate::squad::OrderPatch { face: Some(Some(face)), aim_at: Some(None), upper: Some(Some(crate::character::UpperMode::None)), ..Default::default() });
    if g.light_home.mains.permanent && g.time - there_t > g.k.powerout.look_secs {
        st(g).word = true;
        crate::game::game_say(g, o.gd, "breaker's wrecked — that's them! Stay on your doors!", Some(true), None);
        event(g, PowerOutEventWhat::Wrecked, Some(o.gd), "the breaker is shot to scrap: no fault, and nothing to mend");
        all_back(g);
    }
}

/// His leg ended short: no way to the breaker sends him home; no way home and he holds where he is (false = off the list).
fn finds_no_way(g: &mut Game, o: OnErrand) -> bool {
    if man(g, o.i).phase == Phase::Back {
        crate::roomfight::stand_here(g, o.d);
        event(g, PowerOutEventWhat::Abort, Some(o.gd), "no way back to his post: he holds where he is");
        return false;
    }
    man(g, o.i).phase = Phase::Back;
    walk(g, o.d, o.from);
    event(g, PowerOutEventWhat::Abort, Some(o.gd), "no way to the breaker: back to his post");
    true
}

/// He has reached the lever, or his place short of it.
fn gets_to_his_place(g: &mut Game, o: OnErrand) {
    man(g, o.i).phase = Phase::There;
    let time = g.time;
    man(g, o.i).there_t = time;
    let why = if o.role == ManRole::Lever {
        alloc::format!("at the lever {} s after the lights went", js::to_fixed(g.time - st(g).since, 1))
    } else {
        String::from("set short of the lever, watching the way they came")
    };
    event(g, PowerOutEventWhat::There, Some(o.gd), &why);
}

/// the beat is up: the party steps off as one, or does not.
fn step_off(g: &mut crate::game::Game, fight: bool) {
    let wait: Vec<usize> = (0..st(g).men.len()).filter(|&i| man(g, i).phase == Phase::Wait).collect();
    let all: Option<&str> = if !g.light_home.mains.active {
        Some("the mains are back")
    } else if fight {
        Some("a fight has started")
    } else {
        None
    };
    // `why` is a list beside `wait`: one reason for each man in it
    let mut why: Vec<Option<String>> = Vec::with_capacity(wait.len());
    for &i in &wait {
        let gd = man(g, i).gd;
        let d = def_of(g, gd);
        why.push(match (all, d) {
            (Some(a), _) => Some(String::from(a)),
            (None, None) => Some(String::from("no man of the fight's any more")),
            (None, Some(d)) => power_out_kept(g, d),
        });
    }
    let mut dropped = vec![false; st(g).men.len()];
    for (w, &i) in why.iter().zip(wait.iter()) {
        if let Some(w) = w {
            dropped[i] = true;
            let gd = man(g, i).gd;
            event(g, PowerOutEventWhat::Abort, Some(gd), &alloc::format!("does not step off: {}", w));
        }
    }
    let mut at = 0;
    st(g).men.retain(|_| {
        let d = dropped[at];
        at += 1;
        !d
    });
    // what is left at 'wait' now is exactly who goes, in the same order
    let goes: Vec<usize> = (0..st(g).men.len()).filter(|&i| man(g, i).phase == Phase::Wait).collect();
    if goes.is_empty() {
        if st(g).men.is_empty() {
            let s = st(g);
            s.parties = m::max(0.0, s.parties - 1.0);
        }
        return;
    }
    if !st(g).men.iter().any(|x| x.role == ManRole::Lever) {
        let first = goes[0];
        man(g, first).role = ManRole::Lever;
        let spot = lever_spot(g);
        man(g, first).goal = spot;
        man(g, first).watch = None;
    }
    for &i in &goes {
        let gd = man(g, i).gd;
        let d = def_of(g, gd).unwrap();
        let heard = g.gd[gd].witness.heard.loud();
        man(g, i).heard0 = heard;
        man(g, i).phase = Phase::Out;
        let goal = man(g, i).goal;
        walk(g, d, goal);
        let how = if man(g, i).role == ManRole::Lever { "walks to the lever" } else { "goes with him to cover it" };
        let why = alloc::format!("{}, {} m off, {} s after the lights went", how, js::to_fixed(dist_xz(g.pos_of(gd), goal), 0), js::to_fixed(g.time - st(g).since, 1));
        event(g, PowerOutEventWhat::Go, Some(gd), &why);
    }
    let at = g.time + st(g).way_m / m::max(0.5, g.k.tune.guard_investigate) + g.k.powerout.overdue;
    st(g).overdue_at = at;
}

/// nobody is out any more: a party that lost a man and sent no word is missed once it is `overdue`.
fn missed(g: &mut crate::game::Game, defs: &[RoomfightDefId], fight: bool) {
    if st(g).overdue_at < 0.0 {
        return;
    }
    if !st(g).lost || st(g).word || !g.light_home.mains.active {
        st(g).overdue_at = -1.0;
        return;
    }
    if g.time < st(g).overdue_at {
        return;
    }
    st(g).overdue_at = -1.0;
    st(g).word = true;
    let mut speaker: Option<GuardId> = None;
    for &d in defs {
        let gd = g.tab.roomfight_def[d].gd;
        if fit(g, gd) {
            speaker = Some(gd);
            break;
        }
    }
    let Some(speaker) = speaker else { return };
    if fight {
        return;
    }
    crate::game::game_say(g, speaker, "nothing from the breaker — that's them! Stay on your doors!", Some(true), None);
    event(g, PowerOutEventWhat::Overdue, Some(speaker), "the men sent for the lights have not been heard from and the mains are still down");
}

/// the rule let go of in the middle of a walk: every man out goes back to his post at a run.
fn stand_down(g: &mut crate::game::Game) {
    let mut i = 0;
    while i < st(g).men.len() {
        let gd = man(g, i).gd;
        if let Some(d) = def_of(g, gd) {
            if crate::roomfight::on_leg_of(g, d, crate::owner::by::BREAKER) && man(g, i).phase != Phase::Wait && alive(g, gd) && g.gd[gd].held.is_none() {
                let from = man(g, i).from;
                crate::roomfight::run(g, d, from, None, 0.0, crate::owner::by::BREAKER);
            }
        }
        i += 1;
    }
    let s = st(g);
    s.men = Vec::new();
    s.overdue_at = -1.0;
    s.fell = None;
}

#[cfg(test)]
mod tests {
    use super::*;
    // the pinned bits of dist_xz
    #[test]
    fn flat_keeps_its_bits() {
        assert_eq!(dist_xz([1.5, 0.0, -2.25], [0.1, 7.0, 3.3]).to_bits(), 0x4016e539e098fe40);
        assert_eq!(dist_xz([12.3, 1.0, 4.56], [-7.89, 0.0, 0.123]).to_bits(), 0x4034abfab1c8e1b8);
    }
    #[test]
    fn frac_keeps_its_bits() {
        assert_eq!(v3::frac(7.0 * 0.6180339887).to_bits(), 0x3fd4e115043ec120);
        assert_eq!(v3::frac(123.0 * 0.6180339887).to_bits(), 0x3f929df017341000);
        assert_eq!(v3::frac(-1.25).to_bits(), 0x3fe8000000000000);
    }
}
