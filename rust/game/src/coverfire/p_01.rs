//! src/game/coverfire.ts lines 1 to 391
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- local helpers

/// `a === b` of two Whos
fn same_who(a: Who, b: Who) -> bool {
    match (a, b) {
        (Who::Player, Who::Player) => true,
        (Who::Guard(x), Who::Guard(y)) => x == y,
        _ => false,
    }
}
/// `!who || K.who === who`
fn who_passes(who: Option<KnownWho>, k: Who) -> bool {
    match who {
        None => true,
        Some(KnownWho::Player) => same_who(k, Who::Player),
        Some(KnownWho::Guard(x)) => same_who(k, Who::Guard(x)),
    }
}
fn who_of_foe(f: crate::game::Foe) -> Who {
    match f {
        crate::game::Foe::Guard(x) => Who::Guard(x),
        crate::game::Foe::Player => Who::Player,
    }
}
fn known_who_of_foe(f: crate::game::Foe) -> KnownWho {
    match f {
        crate::game::Foe::Guard(x) => KnownWho::Guard(x),
        crate::game::Foe::Player => KnownWho::Player,
    }
}
/// what manOf makes for a man new to the books
fn new_man() -> Man {
    Man { know: Vec::new(), chew: None, gave: JsMap::new(), no: None, said_t: -100.0 }
}
/// `{ ...WALL_STATS }`: the engine's counts as this game's books open
fn wall_base(g: &Game) -> JsMap<String, f64> {
    let w = &g.stats.wallbreak;
    let rows = [
        ("rounds", w.rounds), ("ports", w.ports), ("holes", w.holes), ("coverCells", w.cover_cells), ("coverDead", w.cover_dead), ("blasts", w.blasts),
        ("refused", w.refused), ("falls", w.falls), ("floating", w.floating), ("chunks", w.chunks), ("chunksDemoted", w.chunks_demoted), ("navOpened", w.nav_opened),
    ];
    let mut out = JsMap::new();
    for (k, v) in rows { out.set(String::from(k), v as f64); }
    out
}
/// the row of the gun in his hands: `gd.weapon.def`. A copy, since what takes it also takes `g`
fn def_of(g: &Game, gd: GuardId) -> crate::weapons::WeaponDef {
    g.m.weapons.weapons.get(g.fa[g.gd[gd].weapon].def).unwrap().clone()
}
/// FIGHT_SKILL.operative
fn op_skill() -> crate::roomfight::FightSkill {
    *crate::roomfight::fight_skill().get(String::from("operative")).unwrap()
}
/// the room fight's own Op behind an OpLike, by its man: `ops` holds one a man (roomfight.ts:476)
fn op_row(g: &Game, gd: GuardId) -> Option<RoomfightOpId> {
    g.fight.ops.iter().copied().find(|&id| g.tab.roomfight_op[id].gd == gd)
}
/// the frame's sightings: `now(t)` of coverfireTick
fn seen_now(time: f64, dt: f64, t: f64) -> bool {
    time - t < dt * 0.5 + 1e-6
}
/// the team's line when one of them starts a chew or a burst (coverfire.ts:348,354)
fn say_line(g: &mut Game, gd: GuardId) {
    let (t, say_secs) = (g.time, g.k.coverfire.say_secs);
    if g.k.coverfire.line.is_empty() || g.gd[gd].team != crate::teams::Team::Operative { return; }
    let m_said = man_of(g, gd).said_t;
    let s_said = state_of(g).said_t;
    if !(t - m_said > say_secs && t - s_said > 4.0) { return; }
    man_of(g, gd).said_t = t;
    state_of(g).said_t = t;
    let line = g.k.coverfire.line.clone();
    crate::game::game_say(g, gd, &line, None, None, None);
}
/// hit: is this body in the lane (coverfire.ts:191)
fn lane_hit(g: &mut Game, ch: CharId, from: V3, dx: f64, dz: f64, l2: f64, t_max: f64, l: f64) -> bool {
    let h = match g.ch[ch].bones.get(crate::character::Bone::Hips) { Some(h) => *h, None => g.ch[ch].pos };
    let t = v3::clamp(((h[0] - from[0]) * dx + (h[2] - from[2]) * dz) / l2, 0.0, t_max);
    m::hyp2(from[0] + dx * t - h[0], from[2] + dz * t - h[2]) < g.k.coverfire.lane + g.k.coverfire.cone_k * t * l
}
/// writes through his chew's row, if he still has one (in the TypeScript the object lives on whether the row holds it or not)
fn set_chew(g: &mut Game, gd: GuardId, f: impl FnOnce(&mut Chew)) {
    let row = g.m.coverfire.states.as_mut().and_then(|s| s.men.get_mut(gd)).and_then(|m| m.chew.as_mut());
    if let Some(cw) = row { f(cw); }
}
/// is he up, for a reader that holds the state (coverfire.ts upOf)
fn up_now(g: &Game, who: Who) -> bool {
    match who {
        Who::Player => !g.pl[g.player].down,
        Who::Guard(x) => g.gd[x].state != crate::game::GuardState::Dead && g.ch[g.gd[x].ch].alive,
    }
}

// ---------------------------------------------------------------- the module's own

/// QA: why a known place was not worth rounds, counted (coverfire.ts no)
pub fn no(g: &mut crate::game::Game, why: &str) {
    let w = &mut g.m.coverfire.coverfire_why;
    let n = w.get(String::from(why)).copied().unwrap_or(0.0);
    w.set(String::from(why), n + 1.0);
}

/// a count, on this game's books and on the checks' (coverfire.ts bump)
pub fn bump(g: &mut crate::game::Game, k: crate::coverfire::BumpK) {
    state_of(g);
    let s = g.m.coverfire.states.as_mut().unwrap();
    let (a, b) = match k {
        BumpK::Chews => (&mut s.stats.chews, &mut g.stats.coverfire.chews),
        BumpK::Shots => (&mut s.stats.shots, &mut g.stats.coverfire.shots),
        BumpK::Landed => (&mut s.stats.landed, &mut g.stats.coverfire.landed),
        BumpK::Opened => (&mut s.stats.opened, &mut g.stats.coverfire.opened),
        BumpK::GaveUp => (&mut s.stats.gave_up, &mut g.stats.coverfire.gave_up),
        BumpK::HeldFriend => (&mut s.stats.held_friend, &mut g.stats.coverfire.held_friend),
        BumpK::HeldSilent => (&mut s.stats.held_silent, &mut g.stats.coverfire.held_silent),
        BumpK::Calls => (&mut s.stats.calls, &mut g.stats.coverfire.calls),
        BumpK::Walked => (&mut s.stats.walked, &mut g.stats.coverfire.walked),
        BumpK::OpChews => (&mut s.stats.op_chews, &mut g.stats.coverfire.op_chews),
        BumpK::DefChews => (&mut s.stats.def_chews, &mut g.stats.coverfire.def_chews),
        BumpK::Thru => (&mut s.stats.thru, &mut g.stats.coverfire.thru),
        BumpK::ThruShots => (&mut s.stats.thru_shots, &mut g.stats.coverfire.thru_shots),
    };
    *a += 1.0;
    *b += 1;
}

/// the engine's half: walls and cover give under rounds (coverfire.ts chewLive)
pub fn chew_live(g: &mut crate::game::Game) -> bool {
    crate::wallbreak::wall_break_on(g) && crate::destruction::chaos_of(g) > 0.0
}

/// is the rule running this frame (coverfire.ts live)
pub fn live(g: &mut crate::game::Game) -> bool {
    if !g.ai_enabled || g.touring { return false; }
    chew_live(g) || crate::wallpen::wall_men_live(g)
}

/// the state: a clock gone back forgets every man; none yet makes one (coverfire.ts stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::coverfire::State {
    let (time, serial) = (g.time, g.destr.walls.serial);
    if let Some(s) = g.m.coverfire.states.as_mut() {
        if time < s.last_t - 1e-6 {
            s.men.clear();
            s.by_id.clear();
            s.serial = serial;
            s.said_t = -100.0;
            s.prune_t = time;
            s.last_t = time;
        }
    } else {
        let base = wall_base(g);
        g.m.coverfire.states = Some(State {
            last_t: time,
            men: JsMap::new(),
            serial,
            base,
            stats: zero(),
            by_id: JsMap::new(),
            said_t: -100.0,
            prune_t: 0.0,
        });
    }
    g.m.coverfire.states.as_mut().unwrap()
}

/// a man's row, made if new (coverfire.ts manOf)
pub fn man_of<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::coverfire::Man {
    state_of(g).men.get_or_insert_with(gd, new_man)
}

/// distance over the floor (coverfire.ts flat)
pub fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// whose body (coverfire.ts charOf)
pub fn char_of(g: &mut crate::game::Game, who: crate::coverfire::Who) -> CharId {
    match who {
        Who::Player => g.pl[g.player].ch,
        Who::Guard(x) => g.gd[x].ch,
    }
}

/// is he up (coverfire.ts upOf)
pub fn up_of(g: &mut crate::game::Game, who: crate::coverfire::Who) -> bool {
    up_now(g, who)
}

/// his name (coverfire.ts nameOf)
pub fn name_of(g: &mut crate::game::Game, who: crate::coverfire::Who) -> String {
    match who {
        Who::Player => String::from("Player"),
        Who::Guard(x) => g.gd[x].callsign.clone(),
    }
}

/// QA: the run's line of what the rule did (coverfire.ts log)
pub fn log(g: &mut crate::game::Game, s: &str) {
    let line = alloc::format!("{:.2} {}", g.time, s);
    let l = &mut g.m.coverfire.coverfire_log;
    l.push(line);
    if l.len() > 400 { l.drain(0..200); }
}

/// this gun's round in the engine's units (coverfire.ts weightOf)
pub fn weight_of(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    let cls = g.m.weapons.weapons.get(g.fa[g.gd[gd].weapon].def).unwrap().cls;
    let k = if cls == crate::weapons::WeaponClass::Pistol { g.k.walls.pistol_k } else { 1.0 };
    k * crate::destruction::chaos_of(g) * 2.0
}

// ---------------------------------------------------------------- what he knows

/// a sighting of `who` copied into what `gd` knows (coverfire.ts see)
pub fn see(g: &mut crate::game::Game, gd: GuardId, who: crate::coverfire::Who, low: bool) {
    let c = char_of(g, who);
    let (pos, vel) = (g.ch[c].pos, g.ch[c].vel);
    let trunk = match g.ch[c].bones.get(crate::character::Bone::Spine2) {
        Some(t) => *t,
        None => match g.ch[c].bones.get(crate::character::Bone::Hips) { Some(t) => *t, None => pos },
    };
    let at: V3 = [trunk[0], pos[1] + (if low { 0.8 } else { 1.25 }), trunk[2]];
    let t = g.time;
    let m = man_of(g, gd);
    if let Some(k) = m.know.iter_mut().find(|k| same_who(k.who, who)) {
        k.at = at;
        k.y0 = pos[1];
        k.v = [vel[0], vel[2]];
        k.t = t;
        k.call = Some(false);
    } else {
        m.know.push(Know { who, at, y0: pos[1], v: [vel[0], vel[2]], t, call: None });
    }
}

/// a mate's call he acts on becomes a place he knows, and stays a call (coverfire.ts heard)
pub fn heard(g: &mut crate::game::Game, gd: GuardId, k: crate::coverfire::Know) {
    let m = man_of(g, gd);
    if let Some(mine) = m.know.iter_mut().find(|x| same_who(x.who, k.who)) {
        if mine.t < k.t {
            mine.at = k.at;
            mine.y0 = k.y0;
            mine.v = [0.0, 0.0];
            mine.t = k.t;
            mine.call = Some(true);
        }
    } else {
        m.know.push(Know { v: [0.0, 0.0], call: Some(true), ..k });
    }
}

/// the places he may spend rounds on, freshest first: his own, then what a mate with eyes on is calling (coverfire.ts known)
pub fn known(g: &mut crate::game::Game, gd: GuardId, who: Option<crate::coverfire::KnownWho>) -> Vec<crate::coverfire::KnownItem> {
    state_of(g);
    let g: &Game = g;   // nothing below writes: the state and the men are read side by side
    let s = g.m.coverfire.states.as_ref().unwrap();
    let (stale_secs, call_secs, call_m) = (g.k.coverfire.stale_secs, g.k.coverfire.call_secs, g.k.coverfire.call_m);
    let time = g.time;
    let mut out: Vec<KnownItem> = Vec::new();
    if let Some(own) = s.men.get(gd) {
        for k in own.know.iter() {
            if who_passes(who, k.who) && time - k.t <= stale_secs && up_now(g, k.who) {
                out.push(KnownItem { k: *k, call: k.call.unwrap_or(false) });
            }
        }
    }
    let (team, my_pos) = (g.gd[gd].team, g.ch[g.gd[gd].ch].pos);
    for (mate, row) in s.men.iter() {
        if mate == gd || g.gd[mate].team != team || g.gd[mate].state == crate::game::GuardState::Dead { continue; }
        let mc = g.gd[mate].ch;
        if !g.ch[mc].alive || flat(g.ch[mc].pos, my_pos) > call_m { continue; }
        for k in row.know.iter() {
            if !k.call.unwrap_or(false)
                && who_passes(who, k.who)
                && time - k.t <= call_secs
                && up_now(g, k.who)
                && !out.iter().any(|o| same_who(o.k.who, k.who) && o.k.t >= k.t)
            {
                out.push(KnownItem { k: *k, call: true });
            }
        }
    }
    js::sort_by(&mut out, |a, b| b.k.t - a.k.t);
    out
}

// ---------------------------------------------------------------- what the line would hit

/// the engine's answer for a round from his gun at `to`: Clear · None = not his to chew · else the piece (coverfire.ts ask)
pub fn ask(g: &mut crate::game::Game, gd: GuardId, to: V3) -> Option<crate::coverfire::Ask2> {
    let c = g.gd[gd].ch;
    let from = crate::guards::shot_origin(g, c);
    let d = v3::sub(to, from);
    let l = v3::len(d);
    if l < g.k.coverfire.min_m || l > g.k.coverfire.max_m { return None; }
    let dir: V3 = [d[0] / l, d[1] / l, d[2] / l];
    let reach = l - 0.3;
    let st = match crate::scene_collision::raycast(g, from, dir, reach) { Some(st) => st, None => return Some(Ask2::Clear) };
    if crate::wallpen::wall_men_live(g) {
        let def = def_of(g, gd);
        let energy = crate::glasspen::round_energy(g, &def);
        let first = crate::wallpen::ThinLineFirst { t: st.t, index: st.index, n: st.n };
        let tl = crate::wallpen::thin_line(g, from, to, energy, Some(0.3), Some(first));
        if let Some(crate::wallpen::ThinLine2::ThinLine(tl)) = tl {
            if let Some(x) = tl.walls.first() {
                if x.t >= g.k.coverfire.near_m
                    && l - x.t <= g.k.wallpen.behind_m
                    && !(js::truthy(tl.panes) && crate::glasspen::shot_energy(g, c, Some(&def)) == Some(0.0))
                    && crate::props::prop_system_raycast(g, from, dir, reach).is_none()
                {
                    return Some(Ask2::Ask(Ask {
                        index: x.index,
                        id: -2.0 - x.index,
                        kind: String::from("thru"),
                        hit: x.at,
                        rounds: g.k.wallpen.thru_rounds,
                        layers: tl.walls.len() as f64,
                    }));
                }
            }
        }
    }
    if !chew_live(g) { return None; }
    if st.index >= 1e6 { return None; }
    let b = crate::wallbreak::wall_break_breakable_at(g, st.index)?;
    if st.t < g.k.coverfire.near_m { return None; }
    let kind = b.kind.as_str();
    let behind = g.k.coverfire.behind_m.get(String::from(kind)).copied().unwrap_or(3.0);
    if l - st.t > behind { return None; }
    if crate::props::prop_system_raycast(g, from, dir, st.t).is_some() { return None; }
    let w = weight_of(g, gd);
    if w <= 0.0 { return None; }
    let hit = v3::mad(from, dir, st.t);
    let mut left = crate::wallbreak::wall_break_work_left(g, st.index, hit, dir, Some(reach - st.t))?;
    let mut p = st.t + 0.04;
    let mut ids: JsSet<crate::Num> = JsSet::new();
    ids.add(crate::Num(b.id));
    loop {
        let h = match crate::scene_collision::raycast(g, v3::mad(from, dir, p), dir, reach - p) { Some(h) => h, None => break };
        let bh = if h.index >= 1e6 { None } else { crate::wallbreak::wall_break_breakable_at(g, h.index) };
        let bh = bh?;
        if !ids.has(crate::Num(bh.id)) {
            ids.add(crate::Num(bh.id));
            if ids.len() as f64 > g.k.coverfire.max_layers { return None; }
            let more = crate::wallbreak::wall_break_work_left(g, h.index, v3::mad(from, dir, p + h.t), dir, Some(reach - p - h.t))?;
            left += more;
        }
        p += h.t + 0.04;
    }
    let rounds = m::ceil(left / w - 1e-6);
    if rounds > g.k.coverfire.max_rounds { return None; }
    Some(Ask2::Ask(Ask {
        index: st.index,
        id: b.id,
        kind: String::from(kind),
        hit,
        rounds: m::max(1.0, rounds),
        layers: ids.len() as f64,
    }))
}

/// nobody of his side, no neutral, no hostage in the lane or past it as far as a round would fly (coverfire.ts coverLane)
pub fn cover_lane(g: &mut crate::game::Game, gd: GuardId, from: V3, to: V3) -> bool {
    let dx = to[0] - from[0];
    let dz = to[2] - from[2];
    let l2 = dx * dx + dz * dz;
    if l2 < 0.01 { return false; }
    let l = m::sqrt(l2);
    let dir = v3::normalize(v3::sub(to, from));
    let past_m = g.k.coverfire.past_m;
    let on = if crate::wallpen::wall_men_live(g) { None } else { crate::scene_collision::raycast(g, to, dir, past_m) };
    let flight = if crate::wallpen::wall_men_live(g) {
        let def = def_of(g, gd);
        let energy = crate::glasspen::round_energy(g, &def);
        crate::wallpen::flight_len(g, to, dir, past_m, energy)
    } else {
        match on { Some(on) => on.t, None => past_m }
    };
    let t_max = 1.0 + flight / l;
    let team = g.gd[gd].team;
    if !crate::teams::is_hostile(g, team, crate::teams::Team::Player) && !g.pl[g.player].down {
        let pc = g.pl[g.player].ch;
        if lane_hit(g, pc, from, dx, dz, l2, t_max, l) { return false; }
    }
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o == gd || g.gd[o].state == crate::game::GuardState::Dead { continue; }
        let oc = g.gd[o].ch;
        if !g.ch[oc].alive { continue; }
        let oteam = g.gd[o].team;
        if !crate::teams::is_hostile(g, team, oteam) && lane_hit(g, oc, from, dx, dz, l2, t_max, l) { return false; }
    }
    for cv in crate::civilians::civilian_bodies(g) {
        let cc = g.gd[cv].ch;
        if lane_hit(g, cc, from, dx, dz, l2, t_max, l) { return false; }
    }
    let mut i = 0;
    while i < g.rescue.hostages.len() {
        let h = g.rescue.hostages[i];
        i += 1;
        let hc = g.gd[h].ch;
        if g.gd[h].state != crate::game::GuardState::Dead && g.ch[hc].alive && lane_hit(g, hc, from, dx, dz, l2, t_max, l) { return false; }
    }
    true
}

/// a free man's sum: is shooting through quicker than walking round (coverfire.ts worth)
pub fn worth(g: &mut crate::game::Game, gd: GuardId, to: V3, rounds: f64) -> bool {
    let pos = g.ch[g.gd[gd].ch].pos;
    let path = match crate::scene_collision::nav_grid_find_path(g, pos, to, Some(2000.0), None) { Some(p) => p, None => return true };
    let mut len = 0.0;
    let mut q = pos;
    for p in path.iter() {
        len += flat(q, *p);
        q = *p;
    }
    len += flat(q, to);
    rounds * g.k.coverfire.round_secs <= len / g.k.coverfire.walk_pace
}

/// rounds he will spend on it: a chew gets slack, a burst through is what it is (coverfire.ts budgetOf)
pub fn budget_of(g: &mut crate::game::Game, kind: &str, rounds: f64) -> f64 {
    if kind == "thru" { rounds } else { m::ceil(rounds * g.k.coverfire.slack_k) + g.k.coverfire.slack }
}

/// a man with no feet of his own to weigh (coverfire.ts stays)
pub fn stays(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].pinned || g.gd[gd].hold || g.gd[gd].script.is_some()
}

// ---------------------------------------------------------------- the choice

/// May he put a round through what stands between his gun and this known place, and where to; keeps the chew's own books. What it returns is a copy of the row `men[gd].chew`, which is M.chew in the TypeScript: writes go through the row (coverfire.ts choose)
pub fn choose(g: &mut crate::game::Game, gd: GuardId, k: crate::coverfire::Know, call: bool, seen: bool) -> Option<crate::coverfire::Chew> {
    let (linger, same_m, lead_m, lead_secs, low_y) = (g.k.coverfire.linger_secs, g.k.coverfire.same_m, g.k.coverfire.lead_m, g.k.coverfire.lead_secs, g.k.coverfire.low_y);
    let (thru_rest, give_secs, walk_ask_secs) = (g.k.wallpen.thru_rest_secs, g.k.coverfire.give_secs, g.k.coverfire.walk_ask_secs);
    let time = g.time;
    let row = man_of(g, gd);
    if row.chew.as_ref().map_or(false, |c| time - c.t > linger) {
        row.chew = None;
    }
    let held = row.chew.as_ref().map_or(false, |c| same_who(c.who, k.who) && (k.t <= c.kt + 1e-9 || flat(c.to, k.at) < same_m));
    let held_to = if held { row.chew.as_ref().map(|c| c.to) } else { None };
    let mut to = k.at;
    let a: Option<Ask2>;
    if let (Some(t), false) = (held_to, seen) {
        to = t;
        a = ask(g, gd, to);
    } else {
        let mut b = ask(g, gd, to);
        let sp = m::hyp2(k.v[0], k.v[1]);
        let age = time - k.t;
        if (matches!(b, None | Some(Ask2::Clear))) && !seen && !call && sp > 0.5 {
            let d = m::min(lead_m, sp * m::min(age, lead_secs));
            to = [k.at[0] + k.v[0] / sp * d, k.at[1], k.at[2] + k.v[1] / sp * d];
            b = ask(g, gd, to);
        }
        if (matches!(b, None | Some(Ask2::Clear))) && !seen && !call && age > 0.3 {
            to = [k.at[0], k.y0 + low_y, k.at[2]];
            b = ask(g, gd, to);
        }
        a = b;
    }
    let a = match a {
        Some(Ask2::Clear) => {
            if held { man_of(g, gd).chew = None; }
            return None;
        }
        None => return None,
        Some(Ask2::Ask(a)) => a,
    };
    let thru = a.kind == "thru";
    let budget = budget_of(g, &a.kind, a.rounds);
    let row = man_of(g, gd);
    let own_chew = held && !row.chew.as_ref().unwrap().call;
    if call && a.kind != "cover" && !own_chew {
        if held { row.chew = None; }
        return None;
    }
    let rest = row.gave.get(crate::Num(a.id)).copied().unwrap_or(-1.0);
    if rest > time && !(thru && (seen || k.t > rest - thru_rest)) {
        if held { row.chew = None; }
        return None;
    }
    let new_one = match row.chew.as_ref() { None => true, Some(c) => !held || c.id != a.id };
    if new_one {
        if !held {
            if let Some(no) = row.no.as_ref() {
                if no.id == a.id && no.until > time { return None; }
            }
            if !stays(g, gd) && !worth(g, gd, to, a.rounds) {
                man_of(g, gd).no = Some(ManNo { id: a.id, until: time + walk_ask_secs });
                bump(g, BumpK::Walked);
                return None;
            }
        }
        let row = man_of(g, gd);
        let (was_fired, was_lay, was_next, was_k) = match (held, row.chew.as_ref()) {
            (true, Some(c)) => (c.fired, c.lay, c.next, c.k),
            _ => (0.0, 0.0, 0.0, 0.0),
        };
        row.chew = Some(Chew {
            id: a.id,
            kind: a.kind.clone(),
            who: k.who,
            to: v3::copy(to),
            hit: a.hit,
            h0: a.hit,
            kt: k.t,
            n: 0.0,
            fired: was_fired,
            want: a.rounds,
            layers: a.layers,
            budget,
            fresh: false,
            lay: was_lay,
            t: time,
            next: was_next,
            k: was_k,
            call,
            seen,
        });
    }
    let row = man_of(g, gd);
    let cw = row.chew.as_mut().unwrap();
    cw.hit = a.hit;
    cw.t = time;
    if k.t > cw.kt { cw.kt = k.t; }
    if seen { cw.to = v3::copy(to); }
    if seen && v3::dist(a.hit, cw.h0) > 0.5 { cw.fresh = true; }
    if cw.fresh {
        cw.fresh = false;
        cw.n = 0.0;
        cw.h0 = a.hit;
        cw.want = a.rounds;
        cw.budget = budget;
    }
    if cw.n >= cw.budget && thru {
        if seen { return Some(cw.clone()); }
        row.gave.set(crate::Num(a.id), time + thru_rest);
        row.chew = None;
        return None;
    }
    if cw.n >= cw.budget {
        row.gave.set(crate::Num(a.id), time + give_secs);
        row.chew = None;
        bump(g, BumpK::GaveUp);
        return None;
    }
    Some(cw.clone())
}

// ---------------------------------------------------------------- the hooks

/// EYES ON, NO LINE: may the round go at `to` anyway, because what blocks it is breakable and worth the rounds (coverfire.ts coverSeen)
pub fn cover_seen(g: &mut crate::game::Game, gd: GuardId, who: Option<crate::coverfire::CoverSeenWho>, to: V3) -> bool {
    if !live(g) { return false; }
    let who = match who {
        None => return false,
        Some(CoverSeenWho::Guard(x)) => Who::Guard(x),
        Some(CoverSeenWho::Player) => Who::Player,
    };
    state_of(g);
    let c = char_of(g, who);
    let k = Know { who, at: to, y0: g.ch[c].pos[1], v: [0.0, 0.0], t: g.time, call: None };
    let cw = match choose(g, gd, k, false, true) {
        Some(cw) => cw,
        None => return false,
    };
    let muzzle = g.ch[g.gd[gd].ch].muzzle;
    if !cover_lane(g, gd, muzzle, cw.to) {
        bump(g, BumpK::HeldFriend);
        return false;
    }
    true
}

/// NO EYES ON HIM: the fight AI's man lays on the place he knows and fires through what hides it; returns the point his gun is on (coverfire.ts coverBlind)
pub fn cover_blind(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> Option<V3> {
    if !live(g) { return None; }
    let c = g.gd[gd].ch;
    let foe = g.gd[gd].foe?;
    if crate::teams::foe_down(g, gd)
        || g.gd[gd].held.is_some()
        || g.gd[gd].script.is_some()
        || g.time < g.gd[gd].dazzled_until
        || g.time < g.gd[gd].stagger_until
    {
        return None;
    }
    state_of(g);
    let mut cw: Option<Chew> = None;
    for o in known(g, gd, Some(known_who_of_foe(foe))) {
        cw = choose(g, gd, o.k, o.call, false);
        if cw.is_some() {
            if o.call { heard(g, gd, o.k); }
            break;
        }
    }
    let cw = cw?;
    let to = cw.to;
    let p = g.k.aim.guard;
    let pos = g.ch[c].pos;
    let dist = flat(pos, to);
    if g.gd[gd].reload_t < 0.0 {
        let muzzle = g.ch[c].muzzle;
        if !cover_lane(g, gd, muzzle, to) {
            bump(g, BumpK::HeldFriend);
            return None;
        }
    }
    let facing_err = m::abs(v3::wrap_angle(m::atan2(to[0] - pos[0], to[2] - pos[2]) - g.ch[c].aim_yaw));
    let on = facing_err < 0.25 && !g.gd[gd].muzzle_down;
    let settle_k = g.gd[gd].skill.as_ref().map_or(1.0, |sk| sk.settle_k);
    let settle = m::max(p.settle_secs, 1e-3) * v3::clamp(dist / m::max(p.range_ref, 0.1), p.near_k, 1.0) * settle_k;
    let lay = if !g.tune.aim_model {
        if on { 1.0 } else { 0.0 }
    } else if on {
        m::min(1.0, cw.lay + dt / settle)
    } else {
        m::max(0.0, cw.lay - p.lose_rate * dt)
    };
    set_chew(g, gd, |row| row.lay = lay);
    g.gd[gd].fire_cd -= dt;
    if g.gd[gd].reload_t >= 0.0 {
        crate::guards::tick_reload_beat(g, gd, dt);
        return Some(to);
    }
    let ok = on && lay >= p.acquire_min - 1e-9;
    let fa = g.gd[gd].weapon;
    let def = g.fa[fa].def;
    if g.m.weapons.weapons.get(def).unwrap().burst.is_some() {
        let tr = g.gd[gd].aim.track;
        g.gd[gd].aim.track = lay;
        crate::guards::burst_trigger(g, gd, dt, to, ok, ok, None);
        g.gd[gd].aim.track = tr;
    } else if ok
        && g.gd[gd].fire_cd <= 0.0
        && !crate::guards::round_past_colleague(g, gd, to, None, None)
        && crate::weapons::firearm_fire(g, fa)
    {
        let mods = crate::suppression::guard_fire_mods(g, gd);
        let w = g.m.weapons.weapons.get(def).unwrap();
        let (r, cd, cd_jitter) = (w.ai_spread, w.cd, w.cd_jitter);
        g.gd[gd].fire_cd = (cd + crate::math_dice::sim_random(g) * cd_jitter) * mods.gap;
        let spread_k = g.gd[gd].skill.as_ref().map_or(1.0, |sk| sk.spread_k);
        let hands = (r.base + m::max(0.0, dist - r.range_from) * r.range_k) * spread_k;
        let opts = if g.tune.aim_model {
            crate::combat::FireOpts { steady: Some(true), spread: Some(hands * v3::lerp(p.unsettled_k, 1.0, v3::clamp(lay, 0.0, 1.0)) + mods.spread), ..Default::default() }
        } else {
            crate::combat::FireOpts { spread: Some(mods.spread), ..Default::default() }
        };
        crate::combat::fire_weapon(g, c, to, false, Some(&opts));
        if crate::weapons::firearm_rounds_ready(g, fa) == 0.0 {
            crate::guards::start_reload_beat(g, gd);
        }
    }
    Some(to)
}

/// NOBODY IN HIS SIGHT: the same choice with the room fight's own trigger; true = his gun is on a piece this frame. `o` is a struct apart from the room fight's Op: what is written goes to both, and O.aim, which `point` writes, is read off the Op (coverfire.ts coverOp)
pub fn cover_op(g: &mut crate::game::Game, o: &mut crate::coverfire::OpLike, dt: f64, speed: f64, point: &mut dyn FnMut(&mut crate::game::Game, V3) -> f64) -> bool {
    if !live(g) { return false; }
    let gd = o.gd;
    let c = g.gd[gd].ch;
    state_of(g);
    let mut cw: Option<Chew> = None;
    let mut who: Option<Who> = None;
    for item in known(g, gd, None) {
        if let Who::Guard(h) = item.k.who {
            if crate::silent::silent_holds(g, gd, h) || crate::pairorders::pair_holds(g, gd, h) || crate::spotted::spotted_holds(g, gd, h) {
                bump(g, BumpK::HeldSilent);
                continue;
            }
        }
        cw = choose(g, gd, item.k, item.call, false);
        if cw.is_some() {
            who = Some(item.k.who);
            if item.call { heard(g, gd, item.k); }
            break;
        }
    }
    let (cw, who) = match (cw, who) {
        (Some(cw), Some(who)) => (cw, who),
        _ => return false,
    };
    let to = cw.to;
    let p = g.k.aim.guard;
    let sk = op_skill();
    let pos = g.ch[c].pos;
    let dist = flat(pos, to);
    let muzzle = g.ch[c].muzzle;
    if !cover_lane(g, gd, muzzle, to) {
        bump(g, BumpK::HeldFriend);
        return false;
    }
    if let Who::Guard(h) = who {
        if crate::silent::silent_glass_holds(g, gd, h, c, to) {
            bump(g, BumpK::HeldSilent);
            return false;
        }
    }
    let brg = point(g, to);
    let op = op_row(g, gd);
    let op_aim = op.map_or(o.aim, |id| g.tab.roomfight_op[id].aim);
    let on = m::abs(v3::wrap_angle(brg - g.ch[c].aim_yaw)) < p.on_deg * v3::DEG + m::atan2(0.35, m::max(dist, 0.3))
        && v3::dist(op_aim, to) < 0.3
        && !g.gd[gd].muzzle_down;
    let settle = m::max(p.settle_secs, 1e-3) * v3::clamp(dist / m::max(p.range_ref, 0.1), p.near_k, 1.0) * sk.settle_k;
    let lay = if !g.tune.aim_model {
        if on { 1.0 } else { 0.0 }
    } else if on {
        m::min(1.0, cw.lay + dt / settle)
    } else {
        m::max(0.0, cw.lay - p.lose_rate * dt)
    };
    set_chew(g, gd, |row| row.lay = lay);
    if !on
        || lay < p.acquire_min - 1e-9
        || g.time < cw.next
        || g.time < o.next
        || g.time < o.reload_until
        || crate::gunslots::side_locked(g, gd)
    {
        return true;
    }
    let fa = g.gd[gd].weapon;
    if !crate::weapons::firearm_fire(g, fa) {
        o.reload_until = g.time + crate::gunreload::feed_begin(g, fa).dur;
        if let Some(id) = op { g.tab.roomfight_op[id].reload_until = o.reload_until; }
        crate::game::game_say(g, gd, "reloading.", None, None, None);
        return true;
    }
    let r = g.m.weapons.weapons.get(g.fa[fa].def).unwrap().ai_spread;
    let hands = (r.base + m::max(0.0, dist - r.range_from) * r.range_k) * sk.spread_k;
    let sup_v = g.gd[gd].sup.v;
    let sup_k = if g.tune.suppression && crate::condition::condition_sup(g, crate::condition::Man::Guard(gd), sup_v) >= g.k.suppression.flinch { 1.6 } else { 1.0 };
    let opts = crate::combat::FireOpts {
        steady: Some(true),
        spread: Some(hands * v3::lerp(p.unsettled_k, 1.0, v3::clamp(lay, 0.0, 1.0)) * sup_k + speed * p.shooter_move_k * 0.5),
        ..Default::default()
    };
    crate::combat::fire_weapon(g, c, to, false, Some(&opts));
    o.rounds += 1.0;
    let k_now = cw.k + 1.0;
    set_chew(g, gd, |row| row.k = k_now);
    g.fight.stats.op_rounds += 1.0;
    let str_ = crate::gunroles::gun_string(g, gd);
    let gap = if js::rem(k_now, js::or(str_, 2.0)) == 0.0 {
        sk.shift[0] + crate::math_dice::sim_random(g) * (sk.shift[1] - sk.shift[0])
    } else if js::truthy(str_) {
        crate::gunroles::gun_string_gap(g, gd)
    } else {
        sk.pair
    };
    let next = g.time + gap;
    o.next = next;
    set_chew(g, gd, |row| row.next = next);
    if let Some(id) = op {
        g.tab.roomfight_op[id].rounds = o.rounds;
        g.tab.roomfight_op[id].next = next;
    }
    true
}

// ---------------------------------------------------------------- once a frame

/// this frame's sightings copied into what each man knows · the change feed read · this frame's rounds counted on the chews (coverfire.ts coverfireTick)
pub fn coverfire_tick(g: &mut crate::game::Game, dt: f64) {
    if !live(g) { return; }
    let time = g.time;
    state_of(g).last_t = time;
    let was = state_of(g).serial;
    if g.destr.walls.serial != was {
        let rows = crate::wallbreak::wall_break_since(g, was);
        let serial = g.destr.walls.serial;
        state_of(g).serial = serial;
        let reset = match rows.as_ref() {
            None => true,
            Some(rows) => rows.iter().any(|r| r.kind == crate::wallbreak::LevelChangeKind::Reset),
        };
        if reset {
            let s = state_of(g);
            s.men.clear();
            s.by_id.clear();
        } else {
            for r in rows.unwrap().iter() {
                if r.kind == crate::wallbreak::LevelChangeKind::Door || r.id < 0.0 { continue; }
                let s = state_of(g);
                if let Some(rec) = s.by_id.get_mut(crate::Num(r.id)) { rec.gave += 1.0; }
                let mut mine = false;
                for (_, row) in s.men.iter_mut() {
                    if let Some(cw) = row.chew.as_mut() {
                        if cw.id == r.id {
                            cw.fresh = true;
                            mine = mine || time - cw.t < 1.0;
                        }
                    }
                    row.gave.delete(crate::Num(r.id));
                }
                if mine { bump(g, BumpK::Opened); }
            }
        }
    }
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if g.gd[gd].state == crate::game::GuardState::Dead || !g.ch[g.gd[gd].ch].alive { continue; }
        if g.gd[gd].state == crate::game::GuardState::Alert && seen_now(g.time, dt, g.gd[gd].last_seen_t) {
            if let Some(foe) = g.gd[gd].foe {
                if !crate::teams::foe_down(g, gd) {
                    let low = crate::teams::foe_crouched(g, gd);
                    see(g, gd, who_of_foe(foe), low);
                }
            }
        }
    }
    if g.fight.live {
        let notice = op_skill().notice;
        let mut i = 0;
        while i < g.fight.ops.len() {
            let id = g.fight.ops[i];
            i += 1;
            let (op, tgt) = (g.tab.roomfight_op[id].gd, g.tab.roomfight_op[id].tgt);
            if g.gd[op].state == crate::game::GuardState::Dead || g.gd[op].script.is_none() { continue; }
            let meters: Vec<(GuardId, f64, f64)> = g.tab.roomfight_op[id].meters.iter().map(|(h, v)| (h, v.m, v.seen_t)).collect();
            for (h, mv, seen_t) in meters {
                if g.gd[h].state != crate::game::GuardState::Dead && seen_now(g.time, dt, seen_t) && (tgt == Some(h) || mv >= notice) {
                    let low = g.ch[g.gd[h].ch].anim.crouch > 0.5;
                    see(g, op, Who::Guard(h), low);
                }
            }
        }
    }
    // by live index: a line said here may add an event, and the walk sees it, as for…of does
    let mut i = 0;
    while i < g.events.len() {
        let (kind, e_time, e_to, e_who) = (g.events[i].kind, g.events[i].time, g.events[i].to, g.events[i].who);
        i += 1;
        let e_to = match e_to {
            Some(t) if kind == crate::director::EventKind::GuardShot && e_time == g.time => t,
            _ => continue,
        };
        let mut at = 0;
        while let Some(gd) = state_of(g).men.next_key(&mut at) {
            let char_id = g.ch[g.gd[gd].ch].id;
            let (is_thru, fired, cw_call, cw_id, cw_hit) = match state_of(g).men.get(gd).and_then(|row| row.chew.as_ref()) {
                Some(cw) if Some(char_id) == e_who && cw.t == time => (cw.kind == "thru", cw.fired, cw.call, cw.id, cw.hit),
                _ => continue,
            };
            if is_thru {
                if !js::truthy(fired) {
                    bump(g, BumpK::Thru);
                    say_line(g, gd);
                }
                set_chew(g, gd, |row| { row.n += 1.0; row.fired += 1.0; });
                bump(g, BumpK::ThruShots);
                break;
            }
            if !js::truthy(fired) {
                bump(g, BumpK::Chews);
                if cw_call { bump(g, BumpK::Calls); }
                let side = if g.gd[gd].team == crate::teams::Team::Operative { BumpK::OpChews } else { BumpK::DefChews };
                bump(g, side);
                say_line(g, gd);
            }
            set_chew(g, gd, |row| { row.n += 1.0; row.fired += 1.0; });
            bump(g, BumpK::Shots);
            let gc = g.gd[gd].ch;
            let o = crate::guards::shot_origin(g, gc);
            let el = v3::dist(o, e_to);
            let eh = if el > 0.1 { crate::scene_collision::raycast(g, o, v3::scale(v3::sub(e_to, o), 1.0 / el), el + 0.15) } else { None };
            let landed = match eh {
                Some(eh) if m::abs(eh.t - el) < 0.2 => crate::wallbreak::wall_break_breakable_at(g, eh.index).map(|b| b.id) == Some(cw_id),
                _ => v3::dist(e_to, cw_hit) < 0.75 && el > 0.8,
            };
            if landed {
                bump(g, BumpK::Landed);
                let s = state_of(g);
                if let Some(rec) = s.by_id.get_mut(crate::Num(cw_id)) {
                    rec.landed += 1.0;
                } else {
                    s.by_id.set(crate::Num(cw_id), StateByIdValue { landed: 1.0, gave: 0.0 });
                }
            }
            break;
        }
    }
    if time - state_of(g).prune_t > 2.0 {
        state_of(g).prune_t = time;
        let keep = g.k.coverfire.stale_secs + 1.0;
        let mut at = 0;
        while let Some(gd) = state_of(g).men.next_key(&mut at) {
            if !g.guards.contains(&gd) {
                state_of(g).men.delete(gd);
                continue;
            }
            let row = man_of(g, gd);
            row.know.retain(|k| time - k.t <= keep);
            let mut at2 = 0;
            while let Some(id) = row.gave.next_key(&mut at2) {
                if row.gave.get(id).map_or(false, |until| *until <= time) {
                    row.gave.delete(id);
                }
            }
        }
    }
}

/// QA: the run's own numbers (coverfire.ts coverfireReport)
pub fn coverfire_report(g: &mut crate::game::Game) -> JsMap<String, f64> {
    let on = if live(g) { 1.0 } else { 0.0 };
    let st = g.m.coverfire.states.as_ref().map_or(zero(), |s| s.stats);
    let (mut never, mut never_pieces, mut pieces) = (0.0, 0.0, 0.0);
    if let Some(s) = g.m.coverfire.states.as_ref() {
        for r in s.by_id.values() {
            pieces += 1.0;
            if !js::truthy(r.gave) {
                never += r.landed;
                never_pieces += 1.0;
            }
        }
    }
    let w = &g.stats.wallbreak;
    let walls = [("rounds", w.rounds), ("ports", w.ports), ("holes", w.holes), ("coverCells", w.cover_cells), ("coverDead", w.cover_dead)];
    let rows = [
        ("on", on), ("chews", st.chews), ("shots", st.shots), ("landed", st.landed), ("opened", st.opened), ("gaveUp", st.gave_up), ("heldFriend", st.held_friend),
        ("heldSilent", st.held_silent), ("calls", st.calls), ("walked", st.walked), ("opChews", st.op_chews), ("defChews", st.def_chews), ("thru", st.thru),
        ("thruShots", st.thru_shots), ("never", never), ("neverPieces", never_pieces), ("pieces", pieces),
    ];
    let mut out = JsMap::new();
    for (k, v) in rows { out.set(String::from(k), v); }
    for (k, v) in walls {
        let base = g.m.coverfire.states.as_ref().and_then(|s| s.base.get(String::from(k)).copied()).unwrap_or(0.0);
        out.set(alloc::format!("wall_{}", k), v as f64 - base);
    }
    out
}

/// QA: hand `gd` the record a real sighting of `who` leaves, `age` seconds old, standing still (coverfire.ts coverfireSaw)
pub fn coverfire_saw(g: &mut crate::game::Game, gd: GuardId, who: crate::coverfire::Who, age: Option<f64>) {
    let age = age.unwrap_or(0.0);
    state_of(g);
    let c = char_of(g, who);
    let low = g.ch[c].anim.crouch > 0.5;
    see(g, gd, who, low);
    let t = g.time - age;
    if let Some(k) = man_of(g, gd).know.iter_mut().find(|k| same_who(k.who, who)) {
        k.t = t;
        k.v = [0.0, 0.0];
    }
}

/// QA: what `gd` is on this frame (coverfire.ts coverfireOn)
pub fn coverfire_on(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::coverfire::CoverfireOn> {
    let cw = g.m.coverfire.states.as_ref()?.men.get(gd)?.chew.as_ref()?;
    Some(CoverfireOn { id: cw.id, kind: cw.kind.clone(), fired: cw.fired, want: cw.want, to: v3::copy(cw.to) })
}

/// a restart or a checkpoint load: forget everything about this game (coverfire.ts coverfireReset)
pub fn coverfire_reset(g: &mut crate::game::Game) {
    g.m.coverfire.states = None;
}
