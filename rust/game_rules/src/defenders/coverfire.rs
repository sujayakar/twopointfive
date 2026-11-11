pub use ::game_types::coverfire::types;
pub use types::*;
use crate::v3::dist_xz;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec::Vec};

// ---------------------------------------------------------------- local helpers

/// `!who || K.who === who`
fn who_passes(who: Option<CharId>, k: CharId) -> bool {
    match who {
        None => true,
        Some(w) => k == w,
    }
}
/// what manOf makes for a man new to the books
fn new_man() -> Man {
    Man { know: Vec::new(), chew: None, gave: JsMap::new(), no: None, said_t: -100.0 }
}
/// `gd.weapon.def`: which gun is in his hands
fn def_of(g: &Game, gd: GuardId) -> crate::weapons::WeaponId {
    g.fa[g.gd[gd].weapon].def
}
/// FIGHT_SKILL.operative
fn op_skill() -> crate::roomfight::FightSkill {
    *crate::roomfight::fight_skill().of("operative").unwrap()
}
/// the frame's sightings: `now(t)` of coverfireTick
fn seen_now(time: f64, dt: f64, t: f64) -> bool {
    time - t < dt * 0.5 + 1e-6
}
/// the team's line when one of them starts a chew or a burst
fn say_line(g: &mut Game, gd: GuardId) {
    let (t, say_secs) = (g.time, g.k.coverfire.say_secs);
    if g.k.coverfire.line.is_empty() || !g.team(gd).ours() { return; }
    let m_said = man_of(g, gd).said_t;
    let s_said = state_of(g).said_t;
    if !(t - m_said > say_secs && t - s_said > 4.0) { return; }
    man_of(g, gd).said_t = t;
    state_of(g).said_t = t;
    let line = g.k.coverfire.line.clone();
    crate::game::game_say(g, gd, &line, None, None);
}
/// hit: is this body in the lane
fn lane_hit(g: &Game, ch: CharId, from: V3, dx: f64, dz: f64, l2: f64, t_max: f64, l: f64) -> bool {
    let h = g.ch[ch].bone(crate::character::Bone::Hips).unwrap_or(g.ch[ch].pos);
    let t = v3::clamp(((h[0] - from[0]) * dx + (h[2] - from[2]) * dz) / l2, 0.0, t_max);
    m::hyp2(from[0] + dx * t - h[0], from[2] + dz * t - h[2]) < g.k.coverfire.lane + g.k.coverfire.cone_k * t * l
}
/// writes through his chew's row, if he still has one
fn set_chew(g: &mut Game, gd: GuardId, f: impl FnOnce(&mut Chew)) {
    let row = g.m.coverfire.states.as_mut().and_then(|s| s.men.get_mut(gd)).and_then(|m| m.chew.as_mut());
    if let Some(cw) = row { f(cw); }
}

// ---------------------------------------------------------------- the module's own

/// QA: why a known place was not worth rounds, counted
fn no(g: &mut crate::game::Game, why: &str) {
    let w = &mut g.m.coverfire.coverfire_why;
    let n = w.of(why).copied().unwrap_or(0.0);
    w.set(String::from(why), n + 1.0);
}

/// a count, on this game's books and on the checks'
fn bump(g: &mut crate::game::Game, k: crate::coverfire::BumpK) {
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

/// the state: a clock gone back forgets every man; none yet makes one
fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::coverfire::State {
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
        g.m.coverfire.states = Some(State {
            last_t: time,
            men: JsMap::new(),
            serial,
            stats: zero(),
            by_id: JsMap::new(),
            said_t: -100.0,
            prune_t: 0.0,
        });
    }
    g.m.coverfire.states.as_mut().unwrap()
}

/// a man's row, made if new
fn man_of<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::coverfire::Man {
    state_of(g).men.get_or_insert_with(gd, new_man)
}

/// his name
fn name_of(g: &crate::game::Game, who: CharId) -> String { crate::character::guard_of(g, who).map_or_else(|| String::from("Player"), |x| g.gd[x].callsign.clone()) }

/// QA: the run's line of what the rule did
fn log(g: &mut crate::game::Game, s: &str) {
    let line = alloc::format!("{} {}", js::to_fixed(g.time, 2), s);
    let l = &mut g.m.coverfire.coverfire_log;
    l.push(line);
    if l.len() > 400 { l.drain(0..200); }
}

/// this gun's round in the engine's units
fn weight_of(g: &crate::game::Game, gd: GuardId) -> f64 {
    let cls = g.weapon(g.fa[g.gd[gd].weapon].def).cls;
    let k = if cls == crate::weapons::WeaponClass::Pistol { g.k.walls.pistol_k } else { 1.0 };
    k * crate::destruction::chaos_of(g) * 2.0
}

// ---------------------------------------------------------------- what he knows

/// a sighting of `who` copied into what `gd` knows
fn see(g: &mut crate::game::Game, gd: GuardId, who: CharId, low: bool) {
    let c = who;
    let (pos, vel) = (g.ch[c].pos, g.ch[c].vel);
    let trunk = g.ch[c].bone(crate::character::Bone::Spine2).or(g.ch[c].bone(crate::character::Bone::Hips)).unwrap_or(pos);
    let at: V3 = [trunk[0], pos[1] + (if low { 0.8 } else { 1.25 }), trunk[2]];
    let t = g.time;
    let m = man_of(g, gd);
    if let Some(k) = m.know.iter_mut().find(|k| k.who == who) {
        k.at = at;
        k.y0 = pos[1];
        k.v = [vel[0], vel[2]];
        k.t = t;
        k.call = Some(false);
    } else {
        m.know.push(Know { who, at, y0: pos[1], v: [vel[0], vel[2]], t, call: None });
    }
}

/// a mate's call he acts on becomes a place he knows, and stays a call
fn heard(g: &mut crate::game::Game, gd: GuardId, k: crate::coverfire::Know) {
    let m = man_of(g, gd);
    if let Some(mine) = m.know.iter_mut().find(|x| x.who == k.who) {
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

/// the places he may spend rounds on, freshest first: his own, then what a mate with eyes on is calling
fn known(g: &mut crate::game::Game, gd: GuardId, who: Option<CharId>) -> Vec<crate::coverfire::KnownItem> {
    state_of(g);
    let g: &Game = g;   // nothing below writes: the state and the men are read side by side
    let s = g.m.coverfire.states.as_ref().unwrap();
    let (stale_secs, call_secs, call_m) = (g.k.coverfire.stale_secs, g.k.coverfire.call_secs, g.k.coverfire.call_m);
    let time = g.time;
    let mut out: Vec<KnownItem> = Vec::new();
    if let Some(own) = s.men.get(gd) {
        for k in own.know.iter() {
            if who_passes(who, k.who) && time - k.t <= stale_secs && crate::picture::up(g, k.who) {
                out.push(KnownItem { k: *k, call: k.call.unwrap_or(false) });
            }
        }
    }
    let (team, my_pos) = (g.team(gd), g.pos_of(gd));
    for (mate, row) in s.men.iter() {
        if mate == gd || g.team(mate) != team || crate::condition::down(g, g.gd[mate].ch) { continue; }
        let mc = g.gd[mate].ch;
        if !g.ch[mc].alive || dist_xz(g.ch[mc].pos, my_pos) > call_m { continue; }
        for k in row.know.iter() {
            if !k.call.unwrap_or(false)
                && who_passes(who, k.who)
                && time - k.t <= call_secs
                && crate::picture::up(g, k.who)
                && !out.iter().any(|o| o.k.who == k.who && o.k.t >= k.t)
            {
                out.push(KnownItem { k: *k, call: true });
            }
        }
    }
    js::sort_by(&mut out, |a, b| b.k.t - a.k.t);
    out
}

// ---------------------------------------------------------------- what the line would hit

/// the engine's answer for a round from his gun at `to`: Clear · None = not his to chew · else the piece
fn ask(g: &mut crate::game::Game, gd: GuardId, to: V3) -> Option<crate::coverfire::Ask2> {
    let c = g.gd[gd].ch;
    let from = crate::guards::shot_origin(g, c);
    let d = v3::sub(to, from);
    let l = v3::len(d);
    if l < g.k.coverfire.min_m || l > g.k.coverfire.max_m { no(g, "far"); return None; }
    let dir: V3 = [d[0] / l, d[1] / l, d[2] / l];
    let reach = l - 0.3;
    let st = match crate::scene_collision::raycast(g, from, dir, reach) {
        Some(st) => st,
        None => { no(g, "clear"); return Some(Ask2::Clear); }
    };
    let def = def_of(g, gd);
    let energy = crate::glasspen::round_energy(g, def);
    let first = crate::wallpen::ThinLineFirst { t: st.t, index: st.index, n: st.n };
    let tl = crate::wallpen::thin_line(g, from, to, energy, Some(0.3), Some(first));
    if let Some(crate::wallpen::ThinLine2::ThinLine(tl)) = tl {
        if let Some(x) = tl.walls.first() {
            if x.t >= g.k.coverfire.near_m
                && l - x.t <= g.k.wallpen.behind_m
                && !(js::truthy(tl.panes) && crate::glasspen::shot_energy(g, c, Some(def)) == Some(0.0))
                && crate::props::prop_system_raycast(g, from, dir, reach).is_none()
            {
                return Some(Ask2::Ask(Ask {
                    id: -2.0 - x.index,
                    kind: None,
                    hit: x.at,
                    rounds: g.k.wallpen.thru_rounds,
                    layers: tl.walls.len() as f64,
                }));
            }
        }
    }
    if !crate::wallcharge::wall_charge_on(g) { no(g, "solid"); return None; }
    if st.index >= 1e6 { no(g, "door"); return None; }
    let b = match crate::wallbreak::wall_break_breakable_at(g, st.index) {
        Some(b) => b,
        None => { no(g, "solid"); return None; }
    };
    if st.t < g.k.coverfire.near_m { no(g, "close"); return None; }
    let kind = b.kind;
    let behind = g.k.coverfire.behind_m.of(kind.as_str()).copied().unwrap_or(3.0);
    if l - st.t > behind { no(g, "beyond"); return None; }
    if crate::props::prop_system_raycast(g, from, dir, st.t).is_some() { no(g, "prop"); return None; }
    let w = weight_of(g, gd);
    if w <= 0.0 { return None; }
    let hit = v3::mad(from, dir, st.t);
    let mut left = match crate::wallbreak::wall_break_work_left(g, st.index, hit, dir, Some(reach - st.t)) {
        Some(left) => left,
        None => { no(g, "gone"); return None; }
    };
    let mut p = st.t + 0.04;
    let mut ids: JsSet<crate::Num> = JsSet::new();
    ids.add(crate::Num(b.id));
    loop {
        let h = match crate::scene_collision::raycast(g, v3::mad(from, dir, p), dir, reach - p) { Some(h) => h, None => break };
        let bh = if h.index >= 1e6 { None } else { crate::wallbreak::wall_break_breakable_at(g, h.index) };
        let bh = match bh {
            Some(bh) => bh,
            None => { no(g, "behind"); return None; }
        };
        if !ids.has(crate::Num(bh.id)) {
            ids.add(crate::Num(bh.id));
            if ids.len() as f64 > g.k.coverfire.max_layers { no(g, "deep"); return None; }
            let more = match crate::wallbreak::wall_break_work_left(g, h.index, v3::mad(from, dir, p + h.t), dir, Some(reach - p - h.t)) {
                Some(more) => more,
                None => { no(g, "gone"); return None; }
            };
            left += more;
        }
        p += h.t + 0.04;
    }
    let rounds = m::ceil(left / w - 1e-6);
    if rounds > g.k.coverfire.max_rounds { no(g, "rounds"); return None; }
    Some(Ask2::Ask(Ask {
        id: b.id,
        kind: Some(kind),
        hit,
        rounds: m::max(1.0, rounds),
        layers: ids.len() as f64,
    }))
}

/// nobody of his side, no neutral, no hostage in the lane or past it as far as a round would fly
pub fn cover_lane(g: &crate::game::Game, gd: GuardId, from: V3, to: V3) -> bool {
    let dx = to[0] - from[0];
    let dz = to[2] - from[2];
    let l2 = dx * dx + dz * dz;
    if l2 < 0.01 { return false; }
    let l = m::sqrt(l2);
    let dir = v3::normalize(v3::sub(to, from));
    let past_m = g.k.coverfire.past_m;
    let flight = { let def = def_of(g, gd);
        let energy = crate::glasspen::round_energy(g, def);
        crate::wallpen::flight_len(g, to, dir, past_m, energy) };
    let t_max = 1.0 + flight / l;
    let team = g.team(gd);
    if !team.theirs() {
        for pl in crate::seats::men(g) {
            if !crate::condition::down(g, g.pl[pl].ch) { let pc = g.pl[pl].ch; if lane_hit(g, pc, from, dx, dz, l2, t_max, l) { return false; } }
        }
    }
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o == gd || crate::condition::down(g, g.gd[o].ch) { continue; }
        let oc = g.gd[o].ch;
        if !g.ch[oc].alive { continue; }
        if !team.foe_of(g.team(o)) && lane_hit(g, oc, from, dx, dz, l2, t_max, l) { return false; }
    }
    for cv in crate::civilians::civilian_bodies(g) {
        let cc = g.gd[cv].ch;
        if lane_hit(g, cc, from, dx, dz, l2, t_max, l) { return false; }
    }
    let mut i = 0;
    while i < g.staged.captives.len() {
        let h = g.staged.captives[i];
        i += 1;
        let hc = g.gd[h].ch;
        if !crate::condition::down(g, g.gd[h].ch) && g.ch[hc].alive && lane_hit(g, hc, from, dx, dz, l2, t_max, l) { return false; }
    }
    true
}

/// a free man's sum: is shooting through quicker than walking round
fn worth(g: &crate::game::Game, gd: GuardId, to: V3, rounds: f64) -> bool {
    let pos = g.pos_of(gd);
    let path = match crate::scene_collision::nav_grid_find_path(g, pos, to, Some(2000.0), None) { Some(p) => p, None => return true };
    let len = crate::places::walk_len(pos, &path) + dist_xz(path.last().copied().unwrap_or(pos), to);
    rounds * g.k.coverfire.round_secs <= len / g.k.coverfire.walk_pace
}

/// rounds he will spend on it: a chew gets slack, a burst through is what it is
fn budget_of(g: &crate::game::Game, thru: bool, rounds: f64) -> f64 {
    if thru { rounds } else { m::ceil(rounds * g.k.coverfire.slack_k) + g.k.coverfire.slack }
}

/// a man with no feet of his own to weigh
fn stays(g: &crate::game::Game, gd: GuardId) -> bool {
    !g.gd[gd].duty.free() || g.gd[gd].script.is_some()
}

// ---------------------------------------------------------------- the choice

/// A known place as it is being weighed.
#[derive(Clone, Copy)]
struct Weighing {
    gd: GuardId,
    k: crate::coverfire::Know,
    call: bool,
    seen: bool,
    held: bool,
    time: f64,
}

/// May he put a round through what stands between his gun and this known place, and where to. Returns a copy of his row's chew: writes go through the row.
fn choose(g: &mut crate::game::Game, gd: GuardId, k: crate::coverfire::Know, call: bool, seen: bool) -> Option<crate::coverfire::Chew> {
    let time = g.time;
    leaves_a_chew_gone_cold(g, gd, time);
    let held_to = place_of_his_chew_on_this_man(g, gd, &k);
    let w = Weighing { gd, k, call, seen, held: held_to.is_some(), time };
    let (to, a) = ask_the_line(g, w, held_to);
    let a = match a {
        Some(Ask2::Clear) => {
            if w.held { the_line_has_come_clear(g, gd); }
            return None;
        }
        None => return None,
        Some(Ask2::Ask(a)) => a,
    };
    let budget = budget_of(g, a.kind.is_none(), a.rounds);
    if barred_from_it(g, w, &a) {
        return None;
    }
    let row = man_of(g, gd);
    let new_one = match row.chew.as_ref() { None => true, Some(c) => !w.held || c.id != a.id };
    if new_one && !opens_a_chew(g, w, &a, to, budget) {
        return None;
    }
    keep_the_chews_books(g, w, &a, to, budget)
}

/// A chew not fed for `linger` seconds is over.
fn leaves_a_chew_gone_cold(g: &mut Game, gd: GuardId, time: f64) {
    let linger = g.k.coverfire.linger_secs;
    let row = man_of(g, gd);
    if let Some((id, fired)) = row.chew.as_ref().filter(|c| time - c.t > linger).map(|c| (c.id, c.fired)) {
        row.chew = None;
        if js::truthy(fired) {
            let line = alloc::format!("{} off #{} after {}", g.gd[gd].callsign, js::num_str(id), js::num_str(fired));
            log(g, &line);
        }
    }
}

/// Where his chew's rounds are going, if it is on this man and this sighting is no news or no further off.
fn place_of_his_chew_on_this_man(g: &mut Game, gd: GuardId, k: &crate::coverfire::Know) -> Option<V3> {
    let same_m = g.k.coverfire.same_m;
    let row = man_of(g, gd);
    row.chew.as_ref().filter(|c| c.who == k.who && (k.t <= c.kt + 1e-9 || dist_xz(c.to, k.at) < same_m)).map(|c| c.to)
}

/// What the line answers, and where: on a chew its place; else the place he knows, a guess at where a man moving went on to, then low where he last stood.
fn ask_the_line(g: &mut Game, w: Weighing, held_to: Option<V3>) -> (V3, Option<Ask2>) {
    let Weighing { gd, k, call, seen, time, .. } = w;
    if let (Some(t), false) = (held_to, seen) {
        return (t, ask(g, gd, t));
    }
    let (lead_m, lead_secs, low_y) = (g.k.coverfire.lead_m, g.k.coverfire.lead_secs, g.k.coverfire.low_y);
    let mut to = k.at;
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
    (to, b)
}

/// Nothing stands between them any more: the chew is over.
fn the_line_has_come_clear(g: &mut Game, gd: GuardId) {
    let (id, fired) = man_of(g, gd).chew.as_ref().map_or((0.0, 0.0), |c| (c.id, c.fired));
    if js::truthy(fired) {
        let line = alloc::format!("{} off #{} after {}: the line is clear", g.gd[gd].callsign, js::num_str(id), js::num_str(fired));
        log(g, &line);
    }
    man_of(g, gd).chew = None;
}

/// A mate's call is worth rounds only on cover; a piece he gave up on waits, unless a burst through it has news. Barred, a chew he held on it ends.
fn barred_from_it(g: &mut Game, w: Weighing, a: &Ask) -> bool {
    let Weighing { gd, k, call, seen, held, time } = w;
    let thru_rest = g.k.wallpen.thru_rest_secs;
    let thru = a.kind.is_none();
    let row = man_of(g, gd);
    let own_chew = held && !row.chew.as_ref().unwrap().call;
    if call && a.kind != Some(crate::wallbreak::BKind::Cover) && !own_chew {
        if held { row.chew = None; }
        no(g, "call");
        return true;
    }
    let rest = row.gave.get(crate::Num(a.id)).copied().unwrap_or(-1.0);
    if rest > time && !(thru && (seen || k.t > rest - thru_rest)) {
        if held { row.chew = None; }
        return true;
    }
    false
}

/// A new piece to chew on: a free man first weighs it against walking round, and what he has fired on this man carries over. False = he walks.
fn opens_a_chew(g: &mut Game, w: Weighing, a: &Ask, to: V3, budget: f64) -> bool {
    let Weighing { gd, k, call, seen, held, time } = w;
    if !held {
        let walk_ask_secs = g.k.coverfire.walk_ask_secs;
        if let Some(no) = man_of(g, gd).no.as_ref() {
            if no.id == a.id && no.until > time { return false; }
        }
        if !stays(g, gd) && !worth(g, gd, to, a.rounds) {
            man_of(g, gd).no = Some(ManNo { id: a.id, until: time + walk_ask_secs });
            bump(g, BumpK::Walked);
            return false;
        }
    }
    let row = man_of(g, gd);
    let (was_fired, was_lay, was_next, was_k) = match (held, row.chew.as_ref()) {
        (true, Some(c)) => (c.fired, c.lay, c.next, c.k),
        _ => (0.0, 0.0, 0.0, 0.0),
    };
    row.chew = Some(Chew {
        id: a.id,
        kind: a.kind,
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
    true
}

/// The chew goes on: its count starts again when the piece has changed; rounds spent to no end and he leaves it for a while. None = he has left it.
fn keep_the_chews_books(g: &mut Game, w: Weighing, a: &Ask, to: V3, budget: f64) -> Option<Chew> {
    let Weighing { gd, k, seen, time, .. } = w;
    let thru = a.kind.is_none();
    let (thru_rest, give_secs) = (g.k.wallpen.thru_rest_secs, g.k.coverfire.give_secs);
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
    if !(cw.n >= cw.budget) || (thru && seen) {
        return Some(cw.clone());
    }
    let n = cw.n;
    row.gave.set(crate::Num(a.id), time + if thru { thru_rest } else { give_secs });
    row.chew = None;
    if !thru { bump(g, BumpK::GaveUp); }
    let line = if thru {
        alloc::format!("{} off #{}: {} rounds through it", g.gd[gd].callsign, js::num_str(a.id), js::num_str(n))
    } else {
        alloc::format!("{} leaves #{}: {} rounds in and it stands", g.gd[gd].callsign, js::num_str(a.id), js::num_str(n))
    };
    log(g, &line);
    None
}

// ---------------------------------------------------------------- the hooks

/// EYES ON, NO LINE: may the round go at `to` anyway, because what blocks it is breakable and worth the rounds
pub fn cover_seen(g: &mut crate::game::Game, gd: GuardId, who: Option<CharId>, to: V3) -> bool {
    if !g.ai_enabled { return false; }
    let who = match who {
        None => return false,
        Some(who) => who,
    };
    state_of(g);
    let c = who;
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

/// NO EYES ON HIM: the fight AI's man lays on the place he knows and fires through what hides it; returns the point his gun is on
pub fn cover_blind(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> Option<V3> {
    if !g.ai_enabled { return None; }
    let c = g.gd[gd].ch;
    let foe = g.gd[gd].foe?;
    if crate::teams::foe_down(g, gd)
        || g.gd[gd].held.is_some()
        || g.gd[gd].script.is_some()
        || crate::control::man::blind(g, gd)
        || crate::control::man::staggered(g, gd)
    {
        return None;
    }
    state_of(g);
    let mut cw: Option<Chew> = None;
    for o in known(g, gd, Some(foe)) {
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
    let dist = dist_xz(pos, to);
    if g.gd[gd].reload_t < 0.0 {
        let muzzle = g.ch[c].muzzle;
        if !cover_lane(g, gd, muzzle, to) {
            bump(g, BumpK::HeldFriend);
            return None;
        }
    }
    let facing_err = m::abs(v3::wrap_angle(v3::yaw_to(pos, to) - g.ch[c].aim_yaw));
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
    g.arms[g.gd[gd].ch].fire_cd -= dt;
    if crate::control::man::changing_magazine(g, gd) { return Some(to); }
    let ok = on && lay >= p.acquire_min - 1e-9;
    let fa = g.gd[gd].weapon;
    let def = g.fa[fa].def;
    if g.weapon(def).burst.is_some() {
        let tr = g.arms[g.gd[gd].ch].aim.track;
        g.arms[g.gd[gd].ch].aim.track = lay;
        crate::guards::burst_trigger(g, gd, dt, to, ok, ok, None);
        g.arms[g.gd[gd].ch].aim.track = tr;
    } else if ok
        && g.arms[g.gd[gd].ch].fire_cd <= 0.0
        && !crate::guards::round_past_colleague(g, gd, to, None, None)
        && crate::weapons::firearm_fire(g, fa)
    {
        let mods = crate::suppression::guard_fire_mods(g, gd);
        let w = g.weapon(def);
        let (r, cd, cd_jitter) = (w.ai_spread, w.cd, w.cd_jitter);
        g.arms[g.gd[gd].ch].fire_cd = (cd + crate::dice::draw_of(g, "coverfire::cover_blind#1", gd) * cd_jitter) * mods.gap;
        let spread_k = g.gd[gd].skill.as_ref().map_or(1.0, |sk| sk.spread_k);
        let hands = (r.base + m::max(0.0, dist - r.range_from) * r.range_k) * spread_k;
        let opts = if g.tune.aim_model {
            crate::combat::FireOpts { steady: Some(true), spread: Some(hands * v3::lerp(p.unsettled_k, 1.0, v3::clamp(lay, 0.0, 1.0)) + mods.spread), ..Default::default() }
        } else {
            crate::combat::FireOpts { spread: Some(mods.spread), ..Default::default() }
        };
        crate::combat::fire_weapon(g, c, to, Some(&opts));
        if crate::weapons::firearm_rounds_ready(g, fa) == 0.0 {
            crate::guards::start_reload_beat(g, gd);
        }
    }
    Some(to)
}

/// NOBODY IN HIS SIGHT: the same choice with the room fight's own trigger; true = his gun is on a piece this frame
pub(super) fn cover_op(g: &mut crate::game::Game, o: RoomfightOpId, dt: f64, speed: f64, point: &mut dyn FnMut(&mut crate::game::Game, V3) -> f64) -> bool {
    if !g.ai_enabled { return false; }
    let gd = g.tab.roomfight_op[o].gd;
    let c = g.gd[gd].ch;
    state_of(g);
    let mut cw: Option<Chew> = None;
    let mut who: Option<CharId> = None;
    for item in known(g, gd, None) {
        if let Some(h) = crate::character::guard_of(g, item.k.who) {
            if crate::duty::round_held(g, gd, h) {
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
    let dist = dist_xz(pos, to);
    let muzzle = g.ch[c].muzzle;
    if !cover_lane(g, gd, muzzle, to) {
        bump(g, BumpK::HeldFriend);
        return false;
    }
    if let Some(h) = crate::character::guard_of(g, who) {
        if crate::silent::silent_holds_glass(g, gd, h, c, to) {
            bump(g, BumpK::HeldSilent);
            return false;
        }
    }
    let brg = point(g, to);
    let op_aim = g.tab.roomfight_op[o].aim;
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
        || g.time < g.tab.roomfight_op[o].next
        || crate::control::man::changing_magazine(g, gd)
        || crate::gunslots::side_locked(g, gd)
    {
        return true;
    }
    let fa = g.gd[gd].weapon;
    if !crate::weapons::firearm_fire(g, fa) {
        crate::guards::begin_reload(g, gd);
        crate::game::game_say(g, gd, "reloading.", None, None);
        return true;
    }
    let r = g.weapon(g.fa[fa].def).ai_spread;
    let hands = (r.base + m::max(0.0, dist - r.range_from) * r.range_k) * sk.spread_k;
    let sup_v = g.cond[g.gd[gd].ch].sup.v;
    let sup_k = if g.tune.suppression && crate::condition::condition_sup(g, g.gd[gd].ch, sup_v) >= g.k.suppression.flinch { 1.6 } else { 1.0 };
    let opts = crate::combat::FireOpts {
        steady: Some(true),
        spread: Some(hands * v3::lerp(p.unsettled_k, 1.0, v3::clamp(lay, 0.0, 1.0)) * sup_k + speed * p.shooter_move_k * 0.5),
        ..Default::default()
    };
    crate::combat::fire_weapon(g, c, to, Some(&opts));
    g.tab.roomfight_op[o].rounds += 1.0;
    let k_now = cw.k + 1.0;
    set_chew(g, gd, |row| row.k = k_now);
    let str_ = crate::gunroles::gun_string(g, gd);
    let gap = if js::rem(k_now, js::or(str_, 2.0)) == 0.0 {
        sk.shift[0] + crate::dice::draw_op(g, "coverfire::cover_op#1", o) * (sk.shift[1] - sk.shift[0])
    } else if js::truthy(str_) {
        crate::gunroles::gun_string_gap(g, gd)
    } else {
        sk.pair
    };
    let next = g.time + gap;
    g.tab.roomfight_op[o].next = next;
    set_chew(g, gd, |row| row.next = next);
    true
}

// ---------------------------------------------------------------- once a frame

/// this frame's sightings copied into what each man knows · the change feed read · this frame's rounds counted on the chews
pub fn coverfire_tick(g: &mut crate::game::Game, dt: f64) {
    if !g.ai_enabled { return; }
    let time = g.time;
    state_of(g).last_t = time;
    what_gave_under_fire(g, time);
    the_fight_ais_men_see(g, dt);
    if g.fight.live { the_operatives_see(g, dt); }
    // by live index: a line said here may add an event, and the walk sees it, as for…of does
    let mut i = 0;
    while i < g.events.len() {
        let (kind, e_time, e_to, e_who) = (g.events[i].kind, g.events[i].time, g.events[i].to, g.events[i].who);
        i += 1;
        let e_to = match e_to {
            Some(t) if kind == crate::director::EventKind::GuardShot && e_time == g.time => t,
            _ => continue,
        };
        count_the_round_on_his_chew(g, e_to, e_who, time);
    }
    if time - state_of(g).prune_t > 2.0 { prune_the_books(g, time); }
}

/// The change feed: a piece that changed is news to the men on it and to a man who had given it up.
fn what_gave_under_fire(g: &mut Game, time: f64) {
    let was = state_of(g).serial;
    if g.destr.walls.serial == was { return; }
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
        return;
    }
    for r in rows.unwrap().iter() {
        if r.kind == crate::wallbreak::LevelChangeKind::Door || r.id < 0.0 { continue; }
        let s = state_of(g);
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
        if mine {
            bump(g, BumpK::Opened);
            let line = alloc::format!("#{} gave ({})", js::num_str(r.id), r.kind.as_str());
            log(g, &line);
        }
    }
}

/// An alert man with his foe in sight this frame knows where he is.
fn the_fight_ais_men_see(g: &mut Game, dt: f64) {
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if crate::condition::down(g, g.gd[gd].ch) || !g.ch[g.gd[gd].ch].alive { continue; }
        if g.gd[gd].state == crate::game::GuardState::Alert && seen_now(g.time, dt, g.gd[gd].last_seen_t) {
            if let Some(foe) = g.gd[gd].foe {
                if !crate::teams::foe_down(g, gd) {
                    let low = crate::teams::foe_crouched(g, gd);
                    see(g, gd, foe, low);
                }
            }
        }
    }
}

/// An operative with a man in sight this frame knows where he is, if that man is his target or he has noticed him.
fn the_operatives_see(g: &mut Game, dt: f64) {
    let notice = op_skill().notice;
    let mut i = 0;
    while i < g.fight.ops.len() {
        let id = g.fight.ops[i];
        i += 1;
        let (op, tgt) = (g.tab.roomfight_op[id].gd, g.tab.roomfight_op[id].tgt);
        if crate::condition::down(g, g.gd[op].ch) || g.gd[op].script.is_none() { continue; }
        let meters: Vec<(GuardId, f64, f64)> = g.tab.roomfight_op[id].meters.iter().map(|(h, v)| (h, v.m, v.seen_t)).collect();
        for (h, mv, seen_t) in meters {
            if !crate::condition::down(g, g.gd[h].ch) && seen_now(g.time, dt, seen_t) && (tgt == Some(h) || mv >= notice) {
                let low = g.ch[g.gd[h].ch].anim.crouch > 0.5;
                let c = g.gd[h].ch;
                see(g, op, c, low);
            }
        }
    }
}

/// A shot this frame, on the chew of the man who fired it, if he is on one this frame.
fn count_the_round_on_his_chew(g: &mut Game, e_to: V3, e_who: Option<f64>, time: f64) {
    let mut at = 0;
    while let Some(gd) = state_of(g).men.next_key(&mut at) {
        let char_id = g.char_id(gd);
        let cw = match state_of(g).men.get(gd).and_then(|row| row.chew.as_ref()) {
            Some(cw) if Some(char_id) == e_who && cw.t == time => cw.clone(),
            _ => continue,
        };
        if cw.kind.is_none() {
            return a_round_through(g, gd, &cw);
        }
        return a_round_into_the_piece(g, gd, &cw, e_to);
    }
}

/// A burst through a thin wall: said and logged at its first round.
fn a_round_through(g: &mut Game, gd: GuardId, cw: &Chew) {
    if !js::truthy(cw.fired) {
        bump(g, BumpK::Thru);
        let name = name_of(g, cw.who);
        let line = alloc::format!("{} through #{} for {}{}: {} rounds, {} deep", g.gd[gd].callsign, js::num_str(cw.id), name, if cw.seen { " (in sight)" } else { "" }, js::num_str(cw.want), js::num_str(cw.layers));
        log(g, &line);
        say_line(g, gd);
    }
    set_chew(g, gd, |row| { row.n += 1.0; row.fired += 1.0; });
    bump(g, BumpK::ThruShots);
}

/// A round on what he is chewing: said and logged at its first; counted as landed if it reached that piece.
fn a_round_into_the_piece(g: &mut Game, gd: GuardId, cw: &Chew, e_to: V3) {
    let (cw_call, cw_id, cw_hit) = (cw.call, cw.id, cw.hit);
    if !js::truthy(cw.fired) {
        bump(g, BumpK::Chews);
        if cw_call { bump(g, BumpK::Calls); }
        let side = if g.team(gd).ours() { BumpK::OpChews } else { BumpK::DefChews };
        bump(g, side);
        let name = name_of(g, cw.who);
        let line = alloc::format!("{} on {} #{} for {}{}: {} rounds wanted, {} deep", g.gd[gd].callsign, cw.kind.map_or("thru", |k| k.as_str()), js::num_str(cw_id), name, if cw_call { " (a call)" } else if cw.seen { " (in sight)" } else { "" }, js::num_str(cw.want), js::num_str(cw.layers));
        log(g, &line);
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
        if !s.by_id.has(crate::Num(cw_id)) {
            s.by_id.set(crate::Num(cw_id), StateByIdValue {  });
        }
    }
}

/// Every two seconds: men gone from the game, places gone stale, pieces given up long enough.
fn prune_the_books(g: &mut Game, time: f64) {
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

/// QA: what `gd` is on this frame
pub(super) fn coverfire_on(g: &crate::game::Game, gd: GuardId) -> Option<crate::coverfire::CoverfireOn> {
    let cw = g.m.coverfire.states.as_ref()?.men.get(gd)?.chew.as_ref()?;
    Some(CoverfireOn { kind: cw.kind, fired: cw.fired })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_man_and_a_call_kept_as_a_call() {
        let mut g = Game::blank();
        let (gd, foe) = (GuardId(0), CharId(0));
        assert_eq!(man_of(&mut g, gd).said_t, -100.0);
        let k = Know { who: foe, at: [1.0, 2.0, 3.0], y0: 0.5, v: [4.0, 5.0], t: 10.0, call: None };
        heard(&mut g, gd, k);
        let mine = man_of(&mut g, gd).know[0];
        assert_eq!((mine.v, mine.call, mine.t), ([0.0, 0.0], Some(true), 10.0));
        // an older call changes nothing; a newer one moves the place and stays a call
        heard(&mut g, gd, Know { at: [9.0, 9.0, 9.0], t: 9.0, ..k });
        assert_eq!(man_of(&mut g, gd).know[0].at, [1.0, 2.0, 3.0]);
        heard(&mut g, gd, Know { at: [7.0, 7.0, 7.0], t: 11.0, ..k });
        assert_eq!(man_of(&mut g, gd).know[0].at, [7.0, 7.0, 7.0]);
        assert_eq!(man_of(&mut g, gd).know.len(), 1);
    }

    #[test]
    fn a_clock_gone_back_forgets_every_man() {
        let mut g = Game::blank();
        g.time = 5.0;
        state_of(&mut g).last_t = 5.0;
        man_of(&mut g, GuardId(0)).said_t = 1.0;
        g.time = 4.0;
        let s = state_of(&mut g);
        assert_eq!((s.men.len(), s.last_t, s.prune_t, s.said_t), (0, 4.0, 4.0, -100.0));
    }

    #[test]
    fn the_books_of_why_and_the_log() {
        let mut g = Game::blank();
        no(&mut g, "far");
        no(&mut g, "far");
        assert_eq!(g.m.coverfire.coverfire_why.of("far").copied(), Some(2.0));
        // toFixed: a tie goes up, as format!'s does not
        g.time = 1.125;
        log(&mut g, "x");
        assert_eq!(g.m.coverfire.coverfire_log[0], "1.13 x");
        for _ in 0..400 { log(&mut g, "y"); }
        assert_eq!(g.m.coverfire.coverfire_log.len(), 201);
    }

    #[test]
    fn a_burst_through_gets_no_slack() {
        let mut g = Game::blank();
        (g.k.coverfire.slack_k, g.k.coverfire.slack) = (1.5, 2.0);
        assert_eq!(budget_of(&mut g, true, 3.0), 3.0);
        assert_eq!(budget_of(&mut g, false, 3.0), 7.0);
    }
}
