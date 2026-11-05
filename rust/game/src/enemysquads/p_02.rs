//! src/game/enemysquads.ts lines 314 to 751
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// the two spellings of one union: `SquadEvent['what']` as event() takes it and as the row holds it
fn event_row_what(what: EventWhat) -> SquadEventWhat {
    match what {
        EventWhat::Down => SquadEventWhat::Down, EventWhat::Back => SquadEventWhat::Back, EventWhat::Contact => SquadEventWhat::Contact,
        EventWhat::Lost => SquadEventWhat::Lost, EventWhat::Quiet => SquadEventWhat::Quiet, EventWhat::Found => SquadEventWhat::Found,
        EventWhat::Said => SquadEventWhat::Said, EventWhat::Sweep => SquadEventWhat::Sweep, EventWhat::Flank => SquadEventWhat::Flank,
        EventWhat::Glimpse => SquadEventWhat::Glimpse, EventWhat::Barricade => SquadEventWhat::Barricade, EventWhat::Help => SquadEventWhat::Help,
        EventWhat::Reinforce => SquadEventWhat::Reinforce, EventWhat::InPosition => SquadEventWhat::InPosition, EventWhat::Met => SquadEventWhat::Met,
        EventWhat::Rally => SquadEventWhat::Rally, EventWhat::Stays => SquadEventWhat::Stays, EventWhat::Follows => SquadEventWhat::Follows,
        EventWhat::Check => SquadEventWhat::Check, EventWhat::Asks => SquadEventWhat::Asks, EventWhat::Looks => SquadEventWhat::Looks,
        EventWhat::NoSign => SquadEventWhat::NoSign, EventWhat::Calls => SquadEventWhat::Calls, EventWhat::CutOff => SquadEventWhat::CutOff,
        EventWhat::NetCheck => SquadEventWhat::NetCheck, EventWhat::NetQuery => SquadEventWhat::NetQuery, EventWhat::NetAsks => SquadEventWhat::NetAsks,
        EventWhat::NetLooks => SquadEventWhat::NetLooks, EventWhat::NetHelp => SquadEventWhat::NetHelp,
    }
}
/// the room's identity: rooms are copied here, the TypeScript compares the one object
fn same_room(a: &Option<crate::scene_scenarios::StructureRoom>, b: &Option<crate::scene_scenarios::StructureRoom>) -> bool {
    match (a, b) { (Some(x), Some(y)) => x.id == y.id, (None, None) => true, _ => false }
}
fn pos_of(g: &Game, gd: GuardId) -> V3 { let c = g.gd[gd].ch; g.ch[c].pos }
fn bounds_of(b: Grown) -> crate::scene_level::BreachPlanBounds { crate::scene_level::BreachPlanBounds { x0: b.x0, x1: b.x1, z0: b.z0, z1: b.z1 } }
fn rect_bounds(b: crate::scene_scenarios::Rect) -> crate::scene_level::BreachPlanBounds { crate::scene_level::BreachPlanBounds { x0: b.x0, x1: b.x1, z0: b.z0, z1: b.z1 } }
fn say_radio(g: &mut Game, gd: GuardId, text: &str) { crate::game::game_say(g, gd, text, Some(true), None, None); }
fn door_name(g: &Game, d: DoorId) -> String { g.dr[d].def.name.clone() }
fn nav_idx(g: &mut Game, ix: f64, iz: f64) -> f64 { crate::scene_collision::nav_grid_idx(g, ix, iz) }
fn nav_blocked(g: &mut Game, x: f64, z: f64) -> bool { crate::scene_collision::nav_grid_is_blocked(g, x, z) }
/// wayRound's and wayRound2's `cell`: no check of the grid's edges, as there
fn cell_of(g: &mut Game, p: V3) -> f64 { let c = g.col.nav.cell; nav_idx(g, m::floor(p[0] / c), m::floor(p[2] / c)) }
fn in_grid(g: &Game, ix: f64, iz: f64) -> bool { ix >= 0.0 && iz >= 0.0 && ix < g.col.nav.w && iz < g.col.nav.h }
fn room_has(g: &Game, t: RoomfightRoomId, i: f64) -> bool { g.tab.roomfight_room[t].cells.has(crate::Num(i)) }

/// roomfight.ts: the defenders' row of FIGHT_SKILL as a drilled garrison has it (squadSkill): drillReact and drillNerve are off on the page, on every map
pub fn squad_skill(g: &mut crate::game::Game, base: crate::roomfight::FightSkill) -> crate::roomfight::FightSkill { base }

/// (enemysquads.ts flat)
pub fn flat(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }
/// (enemysquads.ts pick)
pub fn pick(g: &mut crate::game::Game, r: [f64; 2]) -> f64 { r[0] + crate::math_dice::sim_random(g) * (r[1] - r[0]) }
/// a man who dropped it and ran is no man of his squad's any more (enemysquads.ts alive)
pub fn alive(g: &mut crate::game::Game, gd: GuardId) -> bool { g.gd[gd].state != crate::game::GuardState::Dead && !crate::runner::runner_fled(g, gd) }
/// (enemysquads.ts grown)
pub fn grown(b: crate::scene_scenarios::Rect, m: f64) -> crate::enemysquads::Grown { Grown { x0: b.x0 - m, x1: b.x1 + m, z0: b.z0 - m, z1: b.z1 + m } }
/// (enemysquads.ts roomOf)
pub fn room_of(g: &mut crate::game::Game, q: EnemysquadsSquadId, p: V3, m: Option<f64>) -> Option<crate::scene_scenarios::StructureRoom> {
    let m = m.unwrap_or(0.0);
    for r in g.tab.enemysquads_squad[q].rooms.iter() {
        let b = if js::truthy(m) { bounds_of(grown(r.bounds, m)) } else { rect_bounds(r.bounds) };
        if crate::squad::in_bounds(p, Some(b), None) { return Some(r.clone()); }
    }
    None
}
/// a line of the fight's log (enemysquads.ts note)
pub fn note(g: &mut crate::game::Game, gd: Option<GuardId>, what: &str) {
    let who = match gd { Some(x) => g.gd[x].callsign.clone(), None => String::from("-") };
    let t = g.time;
    let l = &mut g.fight.log;
    l.push(crate::roomfight::FightNote { t, who, what: format!("[squads] {}", what) });
    if l.len() > 600 { l.drain(0..200); }
}
/// every half metre along a path, its corners included; returns its length (enemysquads.ts along)
pub fn along(from: V3, path: &[V3], each: &mut dyn FnMut(V3, f64)) -> f64 {
    let (mut q, mut run0) = (from, 0.0);
    for &w in path {
        let l = flat(q, w);
        let n = m::max(1.0, m::ceil(l / 0.5));
        let mut i = 1.0;
        while i <= n {
            each([q[0] + (w[0] - q[0]) * i / n, 0.0, q[2] + (w[2] - q[2]) * i / n], run0 + l * i / n);
            i += 1.0;
        }
        run0 += l;
        q = w;
    }
    run0
}
/// (enemysquads.ts pathLen)
pub fn path_len(from: V3, path: &[V3]) -> f64 { along(from, path, &mut |_, _| {}) }

/// the squads of this staging: each defender is the squad's whose room he was dealt into (enemysquads.ts squadsStage)
pub fn squads_stage(g: &mut crate::game::Game, s: Option<&crate::scene_scenarios::StructureData>, hostiles: &[GuardId], rooms: &[String]) {
    g.m.enemysquads.states = None;
    let Some(s) = s else { return };
    let Some(defs) = s.squads.as_ref().filter(|d| !d.is_empty()) else { return };
    let mut squads: Vec<EnemysquadsSquadId> = Vec::new();
    let mut of: JsMap<GuardId, EnemysquadsSquadId> = JsMap::new();
    for def in defs {
        if def.arrive.is_some() { continue; }   // (late squads: not built)
        let names: &[String] = def.rooms.as_deref().unwrap_or(&[]);
        let mine: Vec<_> = s.rooms.iter().filter(|r| names.contains(&r.id)).cloned().collect();
        let men: Vec<GuardId> = hostiles.iter().enumerate().filter(|(i, _)| rooms.get(*i).map_or(false, |r| names.contains(r))).map(|(_, m)| *m).collect();
        let mut barricade: JsSet<String> = JsSet::new();
        for b in crate::scene_scenarios::squad_barricade(def, &s.rooms) { barricade.add(b); }
        let q = g.tab.enemysquads_squad.push(Squad {
            def: def.clone(), name: def.name.clone().unwrap_or_else(|| def.id.clone()), rooms: mine, men: men.clone(), barricade,
            state: SquadState::Hold, state0: None, fix: None, news_t: -100.0, decide_at: 0.0, decided_at: -100.0, contact_t: -100.0, called_t: -100.0, called_room: None,
            fallen: JsSet::new(), ready: false, own_said: None, stayed: None, hit_room: None, rc: None, calls: None, call_die: None, hung: None, trouble: None, wary: None,
            ask_die: None, sent: Vec::new(), went: Vec::new(), due: Vec::new(), events: Vec::new(),
        });
        squads.push(q);
        for m in men { of.set(m, q); }
    }
    if !squads.is_empty() {
        let kept: Vec<_> = squads.iter().map(|q| g.tab.enemysquads_squad[*q].def.clone()).collect();
        let stay_ask = crate::stayask::stay_ask_here(g, &kept);
        g.m.enemysquads.states = Some(State { stay_ask: Some(stay_ask), s: s.clone(), squads, of, net: Vec::new(), nc: None, word: None });
    }
}
/// the staging is gone: the light meter's and the smoke's keys of its bodies go with it (enemysquads.ts squadsStop)
pub fn squads_stop(g: &mut crate::game::Game) {
    if let Some((squads, nc)) = g.m.enemysquads.states.as_ref().map(|st| (st.squads.clone(), st.nc.is_some())) {
        for &q in &squads {
            if g.tab.enemysquads_squad[q].rc.is_none() { continue; }
            let men = g.tab.enemysquads_squad[q].men.clone();
            for &a in &men {
                let ida = g.ch[g.gd[a].ch].id;
                crate::game::game_light_forget(g, &format!("bodyL{}", ida));
                for &b in &men { if b != a { let idb = g.ch[g.gd[b].ch].id; crate::game::game_light_forget(g, &format!("bodyS{}_{}", idb, ida)); } }
            }
        }
        if nc {
            for &q in &squads {
                let men = g.tab.enemysquads_squad[q].men.clone();
                for &a in &men {
                    let ida = g.ch[g.gd[a].ch].id;
                    crate::game::game_light_forget(g, &format!("bodyL{}", ida));
                    for &x in &squads {
                        if x == q { continue; }
                        let others = g.tab.enemysquads_squad[x].men.clone();
                        for &b in &others { let idb = g.ch[g.gd[b].ch].id; crate::game::game_light_forget(g, &format!("bodyS{}_{}", idb, ida)); }
                    }
                }
            }
        }
    }
    g.m.enemysquads.states = None;
    g.m.enemysquads.ctx = None;
}
/// squads are on the floor in this game right now (enemysquads.ts squadsLive)
pub fn squads_live(g: &mut crate::game::Game) -> bool { g.m.enemysquads.states.is_some() }
/// the squad of a man: its record as the level wrote it and its men in staged order (enemysquads.ts squadOf)
pub fn squad_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::enemysquads::SquadOf> {
    let q = squad_id_of(g, gd)?;
    let x = &g.tab.enemysquads_squad[q];
    Some(SquadOf { def: x.def.clone(), men: x.men.clone() })
}
/// STATES.get(g)?.of.get(gd)
fn squad_id_of(g: &Game, gd: GuardId) -> Option<EnemysquadsSquadId> { g.m.enemysquads.states.as_ref()?.of.get(gd).copied() }

/// he is a squad's man: whether he leaves his room is his squad's call (enemysquads.ts squadOwns)
pub fn squad_owns(g: &mut crate::game::Game, gd: GuardId) -> bool { squad_id_of(g, gd).is_some() }
/// his squad has heard and not said, or he is about to go: he does not start on his door yet (enemysquads.ts squadDefers)
pub fn squad_defers(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let Some(q) = squad_id_of(g, gd) else { return false };
    let x = &g.tab.enemysquads_squad[q];
    x.state == SquadState::Alerted || x.due.iter().any(|d| d.gd == gd)
        || x.sent.iter().any(|s| s.gd == gd && s.role == SentRole::Flank && s.phase != SentPhase::Loose)
        || x.rc.as_ref().map_or(false, |c| c.look.iter().any(|l| l.gd == gd))
}
/// may his squad wedge this door (enemysquads.ts squadMayWedge)
pub fn squad_may_wedge(g: &mut crate::game::Game, gd: GuardId, d: DoorId) -> bool {
    match squad_id_of(g, gd) { None => true, Some(q) => g.tab.enemysquads_squad[q].barricade.has(g.dr[d].def.name.clone()) }
}
/// his squad's men stay in its rooms (enemysquads.ts squadStaysIn)
pub fn squad_stays_in(g: &mut crate::game::Game, gd: GuardId) -> bool {
    squad_id_of(g, gd).map_or(false, |q| g.tab.enemysquads_squad[q].def.may_leave == Some(false))
}
/// where a man who gives ground goes, when his squad has a rally point he can reach in another of its rooms; None = the room fight's own answer (enemysquads.ts squadRally)
pub fn squad_rally(g: &mut crate::game::Game, gd: GuardId, threat: V3) -> Option<V3> {
    let q = squad_id_of(g, gd)?;
    let r = g.tab.enemysquads_squad[q].def.rally?;
    let pos = pos_of(g, gd);
    let here = room_of(g, q, pos, None);
    let there = room_of(g, q, r, None);
    let Some(there_room) = there.clone() else { return None };
    if same_room(&here, &there) || flat(r, threat) < 4.0 { return None; }
    let wide = bounds_of(grown(there_room.bounds, 1.0));
    if team_known(g).iter().any(|p| crate::squad::in_bounds(*p, Some(wide), None)) { return None; }
    let men = g.tab.enemysquads_squad[q].men.clone();
    for (dx, dz) in [(0.0, 0.0), (1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0), (1.0, 1.0), (-1.0, -1.0)] {
        let p: V3 = [r[0] + dx * 0.9, 0.0, r[2] + dz * 0.9];
        if nav_blocked(g, p[0], p[2]) || !crate::squad::in_bounds(p, Some(rect_bounds(there_room.bounds)), None) { continue; }
        let mut on_it = false;
        for &m in &men { if m != gd && alive(g, m) && flat(pos_of(g, m), p) < 0.8 { on_it = true; break; } }
        if on_it { continue; }
        let Some(path) = crate::scene_collision::nav_grid_find_path(g, pos, p, None, None).filter(|x| !x.is_empty()) else { continue };
        let mut toward = false;
        let l = along(pos, &path, &mut |q2, _| { if flat(q2, threat) < 2.0 { toward = true; } });
        if toward || l > 30.0 { continue; }
        let why = format!("{} gives ground to the rally point", g.gd[gd].callsign);
        event(g, q, EventWhat::Rally, &why, None);
        return Some(p);
    }
    None
}

/// (enemysquads.ts defOf)
pub fn def_of(g: &mut crate::game::Game, gd: GuardId) -> Option<RoomfightDefId> { g.fight.defs.iter().copied().find(|d| g.tab.roomfight_def[*d].gd == gd) }
/// his own wits about him: alive, not in Sam's arm, not blinded or reeling (enemysquads.ts fit)
pub fn fit(g: &mut crate::game::Game, m: GuardId) -> bool {
    alive(g, m) && g.gd[m].held.is_none() && g.time >= g.gd[m].dazzled_until && g.time >= g.gd[m].stagger_until && !crate::runner::runner_owns(g, m)
}
/// when this man was sent to look where the team was last put, or None. Words only (enemysquads.ts squadLookSince)
pub fn squad_look_since(g: &mut crate::game::Game, gd: GuardId) -> Option<f64> {
    let q = squad_id_of(g, gd)?;
    g.tab.enemysquads_squad[q].sent.iter().find(|x| x.gd == gd && x.role == SentRole::Look).map(|s| s.t0)
}
/// his squad has him out on a leg, or is about to send him (enemysquads.ts squadBusy)
pub fn squad_busy(g: &mut crate::game::Game, gd: GuardId) -> bool {
    squad_id_of(g, gd).map_or(false, |q| { let x = &g.tab.enemysquads_squad[q]; x.sent.iter().any(|s| s.gd == gd) || x.due.iter().any(|d| d.gd == gd) })
}
/// is this point in one of his squad's rooms; no squad = anywhere (enemysquads.ts squadHolds)
pub fn squad_holds(g: &mut crate::game::Game, gd: GuardId, p: V3) -> bool {
    match squad_id_of(g, gd) {
        None => true,
        Some(q) => g.tab.enemysquads_squad[q].rooms.iter().any(|r| p[0] >= r.bounds.x0 && p[0] <= r.bounds.x1 && p[2] >= r.bounds.z0 && p[2] <= r.bounds.z1),
    }
}
/// a squad's men at home and free to be told something (enemysquads.ts home)
pub fn home(g: &mut crate::game::Game, q: EnemysquadsSquadId) -> Vec<RoomfightDefId> {
    let mut out = Vec::new();
    let men = g.tab.enemysquads_squad[q].men.clone();
    for m in men {
        if !fit(g, m) || g.time - g.gd[m].last_seen_t < 2.0 || g.tab.enemysquads_squad[q].sent.iter().any(|s| s.gd == m) || crate::laststand::last_stand_man(g, m)
            || g.tab.enemysquads_squad[q].rc.as_ref().map_or(false, |c| c.look.iter().any(|l| l.gd == m)) { continue; }
        if crate::netquietfeed::net_feed_holds(g, m) { continue; }
        if g.gd[m].script.is_some() && crate::director::director_lesson_runs(g) { continue; }
        if let Some(d) = def_of(g, m) {
            let row = &g.tab.roomfight_def[d];
            if row.mode == crate::roomfight::DefMode::Hold && row.rally_at != f64::INFINITY { out.push(d); }
        }
    }
    out
}
/// what the net told a man who did not hear it himself (enemysquads.ts tell)
pub fn tell(g: &mut crate::game::Game, d: RoomfightDefId, at: V3) {
    let t = g.time;
    let row = &mut g.tab.roomfight_def[d];
    if row.noise_at.is_some() && row.noise_out && t - row.noise_t < 4.0 { return; }
    row.noise_at = Some(v3::copy(at)); row.noise_t = t; row.noise_out = true;
}
/// where the team is as far as the defenders know: these few seconds' sightings and loud things, never its true positions (enemysquads.ts teamKnown)
pub fn team_known(g: &mut crate::game::Game) -> Vec<V3> {
    let mut out = Vec::new();
    for d in g.fight.defs.clone() {
        let gd = g.tab.roomfight_def[d].gd;
        if !alive(g, gd) { continue; }
        if g.time - g.gd[gd].last_seen_t < 4.0 && g.gd[gd].last_known.is_some() && !g.tab.roomfight_def[d].fix_is_pt { out.push(g.gd[gd].last_known.unwrap()); }
        if let Some(n) = g.tab.roomfight_def[d].noise_at { if g.time - g.tab.roomfight_def[d].noise_t < 4.0 { out.push(n); } }
    }
    out
}

const SPOTS: [(f64, f64); 7] = [(2.4, 1.2), (2.4, -1.2), (3.0, 1.6), (3.0, -1.6), (1.2, 1.7), (1.2, -1.7), (2.6, 0.0)];
fn flank_limit(g: &Game, l0: f64) -> f64 { let k = &g.k.pressure; m::min(k.flank_max_m, m::max(k.flank_k * l0, l0 + k.flank_slack_m)) }
fn is_steel(g: &Game, d: DoorId) -> bool { g.dr[d].def.steel == Some(true) }
fn near_foe(known: &[V3], p: V3, r: f64) -> bool { known.iter().any(|f| flat(*f, p) < r) }
/// the first way by a door nobody has taken, else the first at all, inside the limit: `ways` already sorted
fn pick_way(ways: Vec<Way>, taken: &[DoorId], limit: f64) -> Option<Way> {
    if let Some(w) = ways.iter().find(|w| !taken.contains(&w.door.unwrap()) && w.l <= limit) { return Some(w.clone()); }
    ways.into_iter().find(|w| w.l <= limit)
}

/// a flanker's way to a fix: to a door of its room the team is not known to be at, or on open floor a way off the straight one (enemysquads.ts wayRound)
pub fn way_round(g: &mut crate::game::Game, d: RoomfightDefId, at: V3, known: &[V3], taken: &[DoorId], used: &[V3]) -> Option<crate::enemysquads::Way> {
    let gd = g.tab.roomfight_def[d].gd;
    let pos = pos_of(g, gd);
    let d_room = g.tab.roomfight_def[d].room;
    let cell = g.col.nav.cell;
    let t = crate::roomfight::room_fight_room_at(g, at);
    if let Some(t) = t.filter(|t| Some(*t) != d_room && g.tab.roomfight_room[*t].cells.len() <= 500 && g.tab.roomfight_room[*t].entries.len() > 1) {
        let mut cost: JsMap<crate::Num, f64> = JsMap::new();
        for x in g.fight.defs.clone() {
            let (xgd, xmode, xroom) = { let r = &g.tab.roomfight_def[x]; (r.gd, r.mode, r.room) };
            if !alive(g, xgd) || xmode != crate::roomfight::DefMode::Hold { continue; }
            let Some(xr) = xroom else { continue };
            if xr != t && Some(xr) != d_room && g.tab.roomfight_room[xr].cells.len() <= 500 { for i in g.tab.roomfight_room[xr].cells.keys() { cost.set(i, 5.0); } }
        }
        for f in known {
            let (fx, fz, r) = (m::floor(f[0] / cell), m::floor(f[2] / cell), m::ceil(4.0 / cell));
            let mut dx = -r;
            while dx <= r {
                let mut dz = -r;
                while dz <= r {
                    if dx * dx + dz * dz <= r * r && in_grid(g, fx + dx, fz + dz) { let i = nav_idx(g, fx + dx, fz + dz); cost.set(crate::Num(i), 12.0); }
                    dz += 1.0;
                }
                dx += 1.0;
            }
        }
        for i in g.tab.roomfight_room[t].cells.keys() { cost.set(i, 40.0); }
        let mut dear = |g: &mut Game, ix: f64, iz: f64| -> f64 { let i = nav_idx(g, ix, iz); cost.get(crate::Num(i)).copied().unwrap_or(1.0) };
        let mut ways: Vec<Way> = Vec::new();
        let mut l0 = f64::INFINITY;   // his shortest way to any door of it, the team's own included
        for e in g.tab.roomfight_room[t].entries.clone() {
            if is_steel(g, e.door) { continue; }
            let fr = crate::guards::frame_of(g, e.door);
            for (depth, lat) in SPOTS {
                let p: V3 = [e.at[0] + e.out[0] * depth + fr.ux * lat, 0.0, e.at[2] + e.out[1] * depth + fr.uz * lat];
                if nav_blocked(g, p[0], p[2]) { continue; }
                let c = cell_of(g, p);
                if room_has(g, t, c) || used.iter().any(|u| flat(*u, p) < 1.2) { continue; }
                let Some(path) = crate::scene_collision::nav_grid_find_path(g, pos, p, Some(6000.0), Some(&mut dear)).filter(|x| !x.is_empty()) else { continue };
                if flat(path[path.len() - 1], p) > 0.8 { continue; }
                let mut bad = false;
                let l = along(pos, &path, &mut |q, _| { let c = cell_of(g, q); if room_has(g, t, c) || near_foe(known, q, 3.0) { bad = true; } });
                l0 = m::min(l0, l);
                if !bad && !near_foe(known, p, 5.0) && !near_foe(known, [e.at[0] + e.out[0], 0.0, e.at[2] + e.out[1]], 4.0) { ways.push(Way { spot: p, door: Some(e.door), l, path: None, aim: None }); }
                break;
            }
        }
        js::sort_by(&mut ways, |a, b| a.l - b.l);
        return pick_way(ways, taken, flank_limit(g, l0));
    }
    // open floor: the straight way's middle is made dear, and what the search then finds is the way round, if it is one
    let p0 = crate::scene_collision::nav_grid_find_path(g, pos, at, Some(6000.0), None).filter(|x| !x.is_empty())?;
    let mut mid: JsSet<crate::Num> = JsSet::new();
    let r = 2.0;
    let l0 = path_len(pos, &p0);
    if l0 < 12.0 { return None; }
    along(pos, &p0, &mut |p, s| {
        if s < 5.0 || s > l0 - 5.0 { return; }
        let (x, z) = (m::floor(p[0] / cell), m::floor(p[2] / cell));
        let mut dx = -r;
        while dx <= r {
            let mut dz = -r;
            while dz <= r {
                if in_grid(g, x + dx, z + dz) { let i = nav_idx(g, x + dx, z + dz); mid.add(crate::Num(i)); }
                dz += 1.0;
            }
            dx += 1.0;
        }
    });
    let p1 = crate::scene_collision::nav_grid_find_path(g, pos, at, Some(8000.0), Some(&mut |g: &mut Game, ix: f64, iz: f64| -> f64 { let i = nav_idx(g, ix, iz); if mid.has(crate::Num(i)) { 8.0 } else { 1.0 } })).filter(|x| !x.is_empty())?;
    let l1 = path_len(pos, &p1);
    if !(l1 <= flank_limit(g, l0)) { return None; }
    let (mut on, mut all) = (0.0, 0.0);
    let mut spots: Vec<V3> = Vec::new();
    along(pos, &p1, &mut |p, s| {
        all += 1.0;
        let c = cell_of(g, p);
        if mid.has(crate::Num(c)) { on += 1.0; }
        if s <= l1 - 6.0 && !nav_blocked(g, p[0], p[2]) && !used.iter().any(|u| flat(*u, p) < 1.2) { spots.push(p); }
    });
    if spots.is_empty() || on / m::max(1.0, all) > 0.35 { return None; }
    Some(Way { spot: spots[spots.len() - 1], door: None, l: l1, path: None, aim: None })
}

/// teamKnown, each place with how well it is known (enemysquads.ts teamKnownHow)
pub fn team_known_how(g: &mut crate::game::Game) -> Vec<crate::enemysquads::Known> {
    let mut out = Vec::new();
    for d in g.fight.defs.clone() {
        let gd = g.tab.roomfight_def[d].gd;
        if !alive(g, gd) { continue; }
        if g.time - g.gd[gd].last_seen_t < 4.0 && g.gd[gd].last_known.is_some() && !g.tab.roomfight_def[d].fix_is_pt { out.push(Known { p: g.gd[gd].last_known.unwrap(), firm: true }); }
        if let Some(n) = g.tab.roomfight_def[d].noise_at {
            if g.time - g.tab.roomfight_def[d].noise_t < 4.0 {
                let c = pos_of(g, gd);
                let firm = !crate::scene_collision::solid_blocked(g, [c[0], 1.5, c[2]], [n[0], 1.0, n[2]], None);
                out.push(Known { p: n, firm });
            }
        }
    }
    out
}

// RoundCtx's closures (`fire`, `hot`, `hotCell`, `lane`): private fns over what they captured
fn ctx_line(g: &mut Game, a: V3, b: V3, y: f64) -> bool { !crate::scene_collision::shot_blocked(g, [a[0], y, a[2]], [b[0], y, b[2]], None, None) }
/// a head or a chest in the line
fn ctx_fire(g: &mut Game, a: V3, b: V3) -> bool { ctx_line(g, a, b, 1.5) || ctx_line(g, a, b, 1.0) }
/// the line with every leaf out of it
fn ctx_open(g: &mut Game, a: V3, b: V3) -> bool {
    let dyn_boxes = core::mem::take(&mut g.col.dynamic_boxes);
    let r = ctx_line(g, a, b, 1.5);
    g.col.dynamic_boxes = dyn_boxes;
    r
}
/// a man standing there is in front of one of the guns the side knows of
fn ctx_hot(g: &mut Game, known: &[Known], rear: &[V3], ajar: &[V3], p: V3) -> bool {
    let (fire_m, soft_m, rear_m) = (g.k.squads.fire_m, g.k.squads.soft_m, g.k.squads.rear_m);
    for k in known { let d = flat(k.p, p); if d < (if k.firm { fire_m } else { soft_m }) && (d < 0.6 || ctx_fire(g, k.p, p)) { return true; } }
    for r in rear { let d = flat(*r, p); if d < rear_m && (d < 0.6 || ctx_fire(g, *r, p)) { return true; } }
    for a in ajar { if flat(*a, p) < fire_m && ctx_fire(g, *a, p) { return true; } }
    false
}
/// hotCell's memo, on the record CTX holds: what round_ctx hands back is a copy, and the memo is shared by the men of one decision
fn hot_at_get(g: &Game, i: f64) -> Option<bool> {
    match g.m.enemysquads.ctx.as_ref()?.hot_cell { RoundHotCell::Memo { ref hot_at } => hot_at.get(crate::Num(i)).copied() }
}
fn hot_at_set(g: &mut Game, i: f64, h: bool) {
    if let Some(c) = g.m.enemysquads.ctx.as_mut() { let RoundHotCell::Memo { hot_at } = &mut c.hot_cell; hot_at.set(crate::Num(i), h); }
}
/// asked once a cell, as a search reaches it
fn ctx_hot_cell(g: &mut Game, known: &[Known], rear: &[V3], ajar: &[V3], ix: f64, iz: f64) -> bool {
    if !in_grid(g, ix, iz) { return false; }
    let i = nav_idx(g, ix, iz);
    if let Some(h) = hot_at_get(g, i) { return h; }
    let cell = g.col.nav.cell;
    let h = ctx_hot(g, known, rear, ajar, [(ix + 0.5) * cell, 0.0, (iz + 0.5) * cell]);
    hot_at_set(g, i, h);
    h
}
/// in a doorway's line whether its leaf is shut or not
fn ctx_lane(g: &mut Game, mouths: &[V3], p: V3) -> bool {
    let mouth_m = g.k.squads.mouth_m;
    for m0 in mouths { if flat(*m0, p) < mouth_m && ctx_open(g, *m0, p) { return true; } }
    false
}
fn ctx_dearer(g: &mut Game, cost: &mut JsMap<crate::Num, f64>, ix: f64, iz: f64, c: f64) {
    if !in_grid(g, ix, iz) { return; }
    let i = crate::Num(nav_idx(g, ix, iz));
    if cost.get(i).copied().unwrap_or(1.0) < c { cost.set(i, c); }
}

/// what a way round is judged against, built once a decision and shared by the men it sends (enemysquads.ts roundCtx)
pub fn round_ctx(g: &mut crate::game::Game, t: RoomfightRoomId, known: &[crate::enemysquads::Known], theirs: &[DoorId]) -> crate::enemysquads::RoundCtx {
    let names: Vec<String> = theirs.iter().map(|d| door_name(g, *d)).collect();
    let key = format!("{}#{}", names.join("|"), known.len());
    if let Some(c) = g.m.enemysquads.ctx.as_ref() { if c.t == g.time && c.t_2 == t && c.key == key { return c.clone(); } }
    let cell = g.col.nav.cell;
    let (fire_m, mouth_m) = (g.k.squads.fire_m, g.k.squads.mouth_m);
    let entries = g.tab.roomfight_room[t].entries.clone();
    // where the team's own men stand outside the door it was met at: on its axis and either side of it, as a stack does
    let mut rear: Vec<V3> = Vec::new();
    for e in &entries {
        if !theirs.contains(&e.door) { continue; }
        let fr = crate::guards::frame_of(g, e.door);
        let o1: V3 = [e.at[0] + e.out[0], 0.0, e.at[2] + e.out[1]];
        for (depth, lat) in [(1.0, 0.0), (2.4, 0.0), (2.4, 1.2), (2.4, -1.2)] {
            let r: V3 = [e.at[0] + e.out[0] * depth + fr.ux * lat, 0.0, e.at[2] + e.out[1] * depth + fr.uz * lat];
            if !nav_blocked(g, r[0], r[2]) && (depth == 1.0 || ctx_fire(g, o1, r)) { rear.push(r); }
        }
    }
    let mut mouths: Vec<V3> = Vec::new();
    for e in &entries {
        let mut found = None;
        for d in [3.0, 2.0, 1.2] {
            let p: V3 = [e.at[0] - e.out[0] * d, 0.0, e.at[2] - e.out[1] * d];
            let (ix, iz) = (m::floor(p[0] / cell), m::floor(p[2] / cell));
            let c = if in_grid(g, ix, iz) { nav_idx(g, ix, iz) } else { -1.0 };
            if room_has(g, t, c) { found = Some(p); break; }
        }
        mouths.push(found.unwrap_or([e.at[0] - e.out[0] * 0.8, 0.0, e.at[2] - e.out[1] * 0.8]));
    }
    // the doorways with a line out through them now
    let mut ajar: Vec<V3> = Vec::new();
    for (k, p) in mouths.iter().enumerate() { let e = &entries[k]; if ctx_fire(g, *p, [e.at[0] + e.out[0], 0.0, e.at[2] + e.out[1]]) { ajar.push(*p); } }
    let mut cost: JsMap<crate::Num, f64> = JsMap::new();
    let places: Vec<V3> = known.iter().map(|k| k.p).chain(rear.iter().copied()).collect();
    for f in places {
        let (fx, fz, r) = (m::floor(f[0] / cell), m::floor(f[2] / cell), m::ceil(4.0 / cell));
        let mut dx = -r;
        while dx <= r {
            let mut dz = -r;
            while dz <= r {
                if dx * dx + dz * dz <= r * r { ctx_dearer(g, &mut cost, fx + dx, fz + dz, 12.0); }
                dz += 1.0;
            }
            dx += 1.0;
        }
    }
    // the fan a man 3 m inside has through the opening, out to mouthM
    for (k, e) in entries.iter().enumerate() {
        let fr = crate::guards::frame_of(g, e.door);
        let m0 = mouths[k];
        let mut d = 0.5;
        while d <= mouth_m {
            let w = 0.75 + 0.2 * d;
            let mut l = -w;
            while l <= w {
                let q: V3 = [e.at[0] + e.out[0] * d + fr.ux * l, 0.0, e.at[2] + e.out[1] * d + fr.uz * l];
                if !nav_blocked(g, q[0], q[2]) && ctx_open(g, m0, q) { ctx_dearer(g, &mut cost, m::floor(q[0] / cell), m::floor(q[2] / cell), if flat(m0, q) < fire_m { 25.0 } else { 8.0 }); }
                l += cell;
            }
            d += cell;
        }
    }
    for i in g.tab.roomfight_room[t].cells.keys() { cost.set(i, 40.0); }
    // a room somebody of theirs holds: ×5 to everybody but its own men
    let mut held: JsMap<crate::Num, RoomfightRoomId> = JsMap::new();
    for x in g.fight.defs.clone() {
        let (xgd, xmode, xroom) = { let r = &g.tab.roomfight_def[x]; (r.gd, r.mode, r.room) };
        if !alive(g, xgd) || xmode != crate::roomfight::DefMode::Hold { continue; }
        let Some(xr) = xroom else { continue };
        if xr != t && g.tab.roomfight_room[xr].cells.len() <= 500 { for i in g.tab.roomfight_room[xr].cells.keys() { held.set(i, xr); } }
    }
    let ctx = RoundCtx { t: g.time, t_2: t, key, rear, mouths, hot: RoundHot::Places { known: known.to_vec(), ajar }, hot_cell: RoundHotCell::Memo { hot_at: JsMap::new() }, cost, held };
    g.m.enemysquads.ctx = Some(ctx.clone());
    ctx
}

/// wayRound with goRound on: the net's door, no wall between a known place and a spot, every spot tried, a way walked mark by mark (enemysquads.ts wayRound2)
pub fn way_round2(g: &mut crate::game::Game, d: RoomfightDefId, at: V3, known: &[crate::enemysquads::Known], theirs: &[DoorId], taken: &[DoorId], used: &[V3], back: Option<f64>, says: Option<&mut crate::enemysquads::WayRound2Says>) -> Option<crate::enemysquads::Way> {
    let back = back.unwrap_or(0.0);
    let gd = g.tab.roomfight_def[d].gd;
    let pos = pos_of(g, gd);
    let d_room = g.tab.roomfight_def[d].room;
    let cell = g.col.nav.cell;
    let t = crate::roomfight::room_fight_room_at(g, at).filter(|t| Some(*t) != d_room && g.tab.roomfight_room[*t].cells.len() <= 500 && g.tab.roomfight_room[*t].entries.len() > 1);
    let Some(t) = t else {
        let places: Vec<V3> = known.iter().map(|k| k.p).collect();
        return way_round(g, d, at, &places, taken, used);   // (open floor: as it was)
    };
    let x = round_ctx(g, t, known, theirs);
    let RoundHot::Places { known: cap_known, ajar: cap_ajar } = x.hot.clone();
    let (rear, mouths) = (x.rear.clone(), x.mouths.clone());
    let mut ways: Vec<Way> = Vec::new();
    fn dear0(g: &mut Game, x: &RoundCtx, d_room: Option<RoomfightRoomId>, ix: f64, iz: f64) -> f64 {
        let i = crate::Num(nav_idx(g, ix, iz));
        let h = x.held.get(i).copied();
        m::max(x.cost.get(i).copied().unwrap_or(1.0), if h.is_some() && h != d_room { 5.0 } else { 1.0 })
    }
    // how far the room is by the plain way, guns or no guns, to the door that way comes in by
    let mut l0 = f64::INFINITY;
    {
        let p0 = crate::scene_collision::nav_grid_find_path(g, pos, at, Some(6000.0), Some(&mut |g: &mut Game, ix: f64, iz: f64| -> f64 { dear0(g, &x, d_room, ix, iz) }));
        if let Some(p0) = p0.filter(|p| !p.is_empty()) {
            let mut in_t = false;
            l0 = 0.0;
            along(pos, &p0, &mut |q, s| { if !in_t { let c = cell_of(g, q); if room_has(g, t, c) { in_t = true; } else { l0 = s; } } });
        }
    }
    let out_of = |e: &crate::roomfight::Entry| -> V3 { [e.at[0] + e.out[0], 0.0, e.at[2] + e.out[1]] };
    let mut entries = g.tab.roomfight_room[t].entries.clone();
    js::sort_by(&mut entries, |a, b| flat(pos, out_of(a)) - flat(pos, out_of(b)));   // (nearest first, and a way by a door his mate has not taken is enough)
    for e in &entries {
        if is_steel(g, e.door) { continue; }
        if ways.iter().any(|w| !taken.contains(&w.door.unwrap())) { break; }
        let fr = crate::guards::frame_of(g, e.door);
        let outside = out_of(e);
        // the team's door
        let mut team = theirs.contains(&e.door);
        if !team { for k in known { if k.firm && flat(k.p, outside) < 4.0 && ctx_fire(g, k.p, outside) { team = true; break; } } }
        if !team && theirs.is_empty() && known.iter().any(|k| flat(k.p, outside) < 4.0) { team = true; }
        if team { continue; }
        for (depth, lat) in SPOTS {
            let p: V3 = [e.at[0] + e.out[0] * depth + fr.ux * lat, 0.0, e.at[2] + e.out[1] * depth + fr.uz * lat];
            if nav_blocked(g, p[0], p[2]) { continue; }
            let c = cell_of(g, p);
            if room_has(g, t, c) || used.iter().any(|u| flat(*u, p) < 1.2) { continue; }
            if ctx_hot(g, &cap_known, &rear, &cap_ajar, p) { continue; }
            let path = crate::scene_collision::nav_grid_find_path(g, pos, p, Some(6000.0), Some(&mut |g: &mut Game, ix: f64, iz: f64| -> f64 {
                if ctx_hot_cell(g, &cap_known, &rear, &cap_ajar, ix, iz) { 200.0 } else { dear0(g, &x, d_room, ix, iz) }
            }));
            let Some(path) = path.filter(|x| !x.is_empty()) else { continue };
            if flat(path[path.len() - 1], p) > 0.8 { continue; }
            let mut bad = false;
            let mut pts: Vec<(V3, f64)> = Vec::new();
            let l = along(pos, &path, &mut |q, s| {
                pts.push((q, s));
                let c = cell_of(g, q);
                if room_has(g, t, c) || s > 1.0 && ctx_hot_cell(g, &cap_known, &rear, &cap_ajar, m::floor(q[0] / cell), m::floor(q[2] / cell)) { bad = true; }   // (his first metre is where he stands now)
            });
            if bad { continue; }
            // the room is lost and he is to HOLD its door: he stops short of a doorway's lane, with a line on the door inside `back` metres
            let (mut spot, mut ls) = (p, l);
            if back > 0.0 {
                let mut k: Option<usize> = None;
                for (i, x2) in pts.iter().enumerate() { if x2.1 > 1.0 && ctx_lane(g, &mouths, x2.0) { k = Some(i); break; } }
                if let Some(k) = k {
                    let mut j: Option<usize> = None;
                    let mut i = k;
                    while i > 0 && pts[k].1 - pts[i - 1].1 < 4.0 {
                        i -= 1;
                        if hold_ok(g, pts[i].0, pts[i].1, l, outside, back, used) { j = Some(i); break; }
                    }
                    if j.is_none() { for i in k..pts.len() { if hold_ok(g, pts[i].0, pts[i].1, l, outside, back, used) { j = Some(i); break; } } }
                    if let Some(j) = j { spot = pts[j].0; ls = pts[j].1; }
                }
            }
            let mut marks: Vec<V3> = pts.iter().enumerate().filter(|(i, x2)| x2.1 < ls - 1.5 && x2.1 > 1.5 && i % 5 == 4).map(|(_, x2)| x2.0).collect();
            marks.push(spot);
            ways.push(Way { spot, door: Some(e.door), l: ls, path: Some(marks), aim: Some(outside) });
            break;
        }
    }
    js::sort_by(&mut ways, |a, b| a.l - b.l);
    let limit = flank_limit(g, l0);
    if let Some(s) = says { s.limit = Some(limit); s.best = ways.first().map(|w| w.l); }
    pick_way(ways, taken, limit)
}
/// wayRound2's `ok`: a place short of the lane to hold the door from
fn hold_ok(g: &mut Game, q: V3, s: f64, l: f64, outside: V3, back: f64, used: &[V3]) -> bool {
    s >= 2.0 && l - s >= 3.0 && flat(q, outside) <= back && !nav_blocked(g, q[0], q[2]) && !used.iter().any(|u| flat(*u, q) < 1.2) && ctx_fire(g, q, outside)
}

/// a row of a squad's timeline, what the other squads heard of it, and the log (enemysquads.ts event)
pub fn event(g: &mut crate::game::Game, q: EnemysquadsSquadId, what: crate::enemysquads::EventWhat, why: &str, by: Option<GuardId>) {
    let n = home(g, q).len() as f64;
    let t = g.time;
    let row = SquadEvent { t, what: event_row_what(what), n, why: String::from(why) };
    let ev = &mut g.tab.enemysquads_squad[q].events;
    ev.push(row);
    if ev.len() > 240 { ev.drain(0..80); }
    if net_asks(g) {
        if matches!(what, EventWhat::Asks | EventWhat::Looks | EventWhat::CutOff | EventWhat::NetHelp) { g.tab.enemysquads_squad[q].trouble = Some(g.time); }
        else if matches!(what, EventWhat::NoSign | EventWhat::Found | EventWhat::Contact | EventWhat::Said) { g.tab.enemysquads_squad[q].trouble = None; }
    }
    let line = format!("{} {}: {}", g.tab.enemysquads_squad[q].name, what.as_str(), why);
    note(g, by, &line);
    if what == EventWhat::Found { let men = g.tab.enemysquads_squad[q].men.clone(); crate::netquietfeed::net_feed_body(g, by, &men); }
}
/// the men who go: nearest the fight first, a man left in every room that had one (enemysquads.ts goers)
pub fn goers(g: &mut crate::game::Game, h: &[RoomfightDefId], at: V3, k: f64) -> Vec<RoomfightDefId> {
    let mut by: Vec<(RoomfightDefId, f64)> = h.iter().map(|d| { let gd = g.tab.roomfight_def[*d].gd; (*d, flat(pos_of(g, gd), at)) }).collect();
    js::sort_by(&mut by, |a, b| a.1 - b.1);
    let mut left: JsMap<Option<RoomfightRoomId>, f64> = JsMap::new();
    for d in h { let r = g.tab.roomfight_def[*d].room; let n = left.get(r).copied().unwrap_or(0.0); left.set(r, n + 1.0); }
    let mut out = Vec::new();
    for (d, _) in by {
        if out.len() as f64 >= k { break; }
        let r = g.tab.roomfight_def[d].room;
        let n = left.get(r).copied().unwrap_or(0.0);
        if n > 1.0 { out.push(d); left.set(r, n - 1.0); }
    }
    out
}
static COUNT: [&str; 5] = ["nobody", "one", "two", "three", "four"];
static NUMBER: [&str; 17] = ["Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen"];
/// a mate's name in their mouths: 'Tango 8' is his number on their net; a man the level named keeps his name (enemysquads.ts mate)
pub fn mate(g: &mut crate::game::Game, m: GuardId) -> String {
    let cs = &g.gd[m].callsign;
    if let Some(n) = cs.strip_prefix("Tango ") {
        if !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()) {
            return match n.parse::<usize>().ok().and_then(|i| NUMBER.get(i)) { Some(w) => String::from(*w), None => String::from(n) };
        }
    }
    cs.clone()
}
/// (enemysquads.ts stayKey)
pub fn stay_key(g: &mut crate::game::Game, gd: GuardId, room: &crate::enemysquads::StayKeyRoom) -> String { format!("{}|{}", g.ch[g.gd[gd].ch].id, room.name) }
/// × a squad's beats on the net: a drilled garrison's `netK`, read off the map's own rows (enemysquads.ts netK)
pub fn net_k(g: &mut crate::game::Game) -> f64 { match squad_drill(g) { Some(k) if k.drill_net => k.net_k, _ => 1.0 } }
/// × the beat of a hit squad's own men in its other rooms (enemysquads.ts ownK)
pub fn own_k(g: &mut crate::game::Game) -> f64 { match squad_drill(g) { Some(k) if k.drill_net => k.reinforce_k, _ => 1.0 } }
/// how many rooms these men are in: `new Set(H.map(D => D.room)).size`
fn rooms_held(g: &Game, h: &[RoomfightDefId]) -> usize {
    let mut seen: JsSet<Option<RoomfightRoomId>> = JsSet::new();
    for d in h { seen.add(g.tab.roomfight_def[*d].room); }
    seen.len()
}
/// a squad that could send men out on a call now: decide()'s own first three tests (enemysquads.ts mayGo)
pub fn may_go(g: &mut crate::game::Game, q: EnemysquadsSquadId) -> bool {
    if g.tab.enemysquads_squad[q].def.may_leave == Some(false) { return false; }
    let sent: Vec<GuardId> = g.tab.enemysquads_squad[q].sent.iter().map(|s| s.gd).collect();
    for s in sent { if alive(g, s) { return false; } }
    let h = home(g, q);
    h.len() as f64 - rooms_held(g, &h) as f64 >= 2.0
}

/// the role of the first of this squad's men out on this fight, if any: `X.sent.find(near)` (decide's `near`)
fn first_near(g: &mut Game, x: EnemysquadsSquadId, at: V3) -> Option<SentRole> {
    for i in 0..g.tab.enemysquads_squad[x].sent.len() {
        let (role, gd, fix_at) = { let s = &g.tab.enemysquads_squad[x].sent[i]; (s.role, s.gd, s.fix_at) };
        if role != SentRole::Reinforce && role != SentRole::Look && alive(g, gd) && flat(fix_at, at) < 12.0 { return Some(role); }
    }
    None
}
/// decide's `pair`: the men of a flank and each his way; none with no way round
fn pair(g: &mut Game, net: &[StateNetItem], h: &[RoomfightDefId], fix_at: V3, round_to: V3, spare: f64, ally: Option<EnemysquadsSquadId>, lost: bool) -> (Vec<RoomfightDefId>, Vec<Way>) {
    let k = m::min(g.k.squads.flank_men, spare);
    let go = goers(g, h, fix_at, k);
    let mut taken: Vec<DoorId> = Vec::new();
    let mut used: Vec<V3> = Vec::new();
    let how = team_known_how(g);
    // the team's door: what the squad in contact said on the net, these thirty seconds
    let mut named_set: JsSet<DoorId> = JsSet::new();
    if let Some(a) = ally { for c in net { if c.squad == a && g.time - c.t < 30.0 { if let Some(d) = c.door { named_set.add(d); } } } }
    let named = named_set.keys();
    let back = if lost { g.k.squads.hold_back_m } else { 0.0 };
    let mut ways: Vec<Option<Way>> = Vec::new();
    for d in &go {
        let w = way_round2(g, *d, round_to, &how, &named, &taken, &used, Some(back), None);
        if let Some(w) = &w { used.push(w.spot); if let Some(x) = w.door { taken.push(x); } }
        ways.push(w);
    }
    // (a man with no way round of his own goes his mate's: they end up side by side, not on one cell)
    match ways.iter().flatten().next().cloned() {
        Some(any) => (go, ways.into_iter().map(|w| w.unwrap_or_else(|| any.clone())).collect()),
        None => (Vec::new(), Vec::new()),
    }
}
/// decide's `send`
fn send(g: &mut Game, q: EnemysquadsSquadId, d: RoomfightDefId, role: SentRole, w: Option<&Way>, ally: Option<EnemysquadsSquadId>, fix_at: V3, lost: bool) {
    let (gd, station) = { let r = &g.tab.roomfight_def[d]; (r.gd, r.station) };
    let t0 = g.time;
    g.tab.enemysquads_squad[q].sent.push(Sent {
        gd, from: v3::copy(station), ally, role, phase: if w.is_some() { SentPhase::Run } else { SentPhase::Loose },
        spot: w.map(|x| x.spot), door: w.and_then(|x| x.door), fix_at: v3::copy(fix_at), tries: 0.0, step_t: -100.0, t0,
        hold: Some(role == SentRole::Flank && lost && w.map_or(false, |x| x.door.is_some())), way: w.and_then(|x| x.path.clone()), aim: w.and_then(|x| x.aim),
        there: None, set_t: None, gone_t: None, back: None,
    });
    match w {
        Some(x) => { let to = x.path.as_ref().and_then(|p| p.first().copied()).unwrap_or(x.spot); crate::pressure::run(g, d, to, x.door, Some(0.3)); }
        None => crate::pressure::loose(g, d, fix_at),
    }
}
fn squad_name(g: &Game, q: EnemysquadsSquadId) -> String { g.tab.enemysquads_squad[q].name.clone() }
fn count_word(n: usize) -> String { match COUNT.get(n) { Some(w) => String::from(*w), None => format!("{}", n) } }

/// a squad's word: which of the three, by the situation, never a die (enemysquads.ts decide)
pub fn decide(g: &mut crate::game::Game, q: EnemysquadsSquadId) {
    // (nothing decide calls writes the list of squads or the net: a copy of each reads as the TypeScript's `st` does)
    let (st_squads, st_net) = { let st = g.m.enemysquads.states.as_ref().unwrap(); (st.squads.clone(), st.net.clone()) };
    let mut fix = g.tab.enemysquads_squad[q].fix.unwrap();
    let h = home(g, q);
    let n = h.len();
    let before = g.tab.enemysquads_squad[q].state0;
    let now = g.time;
    { let x = &mut g.tab.enemysquads_squad[q]; x.decided_at = now; x.state = SquadState::Hold; x.ready = false; }
    if n == 0 { return; }
    // WHO is being hit is the net's to say: a squad in contact now, else the squad whose room the noise was in; neither: the team is out on open floor
    let call_of = |x: EnemysquadsSquadId, fallback: V3| -> V3 { for c in st_net.iter().rev() { if c.squad == x { return c.at; } } fallback };
    let room: Option<crate::scene_scenarios::StructureRoom>;   // the room a pair going round is going to
    {
        let mut live: Vec<EnemysquadsSquadId> = st_squads.iter().copied().filter(|x| {
            let row = &g.tab.enemysquads_squad[*x];
            *x != q && row.state != SquadState::Down && (row.state == SquadState::Contact || g.time - row.contact_t < 6.0) && st_net.iter().any(|c| c.squad == *x)
        }).collect();
        if !live.is_empty() {
            let at0 = fix.at;
            js::sort_by(&mut live, |a, b| flat(call_of(*a, at0), at0) - flat(call_of(*b, at0), at0));
            fix.ally = Some(live[0]);
            fix.at = v3::copy(call_of(live[0], at0));
            room = g.tab.enemysquads_squad[live[0]].hit_room.clone();
        } else if fix.by == FixBy::Ears {
            let mut found = None;
            for &x in &st_squads { if x != q && room_of(g, x, fix.at, None).is_some() { found = Some(x); break; } }
            fix.ally = found;
            room = match fix.ally { Some(a) => room_of(g, a, fix.at, None), None => None };
        } else {
            room = fix.ally.and_then(|a| g.tab.enemysquads_squad[a].hit_room.clone());
        }
    }
    g.tab.enemysquads_squad[q].fix = Some(fix);   // (`fix` is Q.fix in the TypeScript: written through)
    let lead = g.tab.roomfight_def[h[0]].gd;
    let ally = fix.ally;
    let mut ally_up = 0usize;
    if let Some(a) = ally { for m0 in g.tab.enemysquads_squad[a].men.clone() { if alive(g, m0) { ally_up += 1; } } }
    // which OTHER squads already have men out on this fight, and as what
    let mut there: Vec<EnemysquadsSquadId> = Vec::new();
    let mut theirs: Option<SentRole> = None;
    for &x in &st_squads {
        if x == q { continue; }
        if let Some(role) = first_near(g, x, fix.at) { if there.is_empty() { theirs = Some(role); } there.push(x); }
    }
    let mut out = 0usize;
    for i in 0..g.tab.enemysquads_squad[q].sent.len() { let gd = g.tab.enemysquads_squad[q].sent[i].gd; if alive(g, gd) { out += 1; } }
    let rooms = rooms_held(g, &h);
    let spare = n as f64 - rooms as f64;   // (the men past one a room)
    let round_to: V3 = match &room { Some(r) => [(r.bounds.x0 + r.bounds.x1) / 2.0, 0.0, (r.bounds.z0 + r.bounds.z1) / 2.0], None => fix.at };
    // the room the fight is in is LOST, or as good as: its other door is held from outside, not gone in by
    let mut in_room = 0usize;
    if let (true, Some(a)) = (room.is_some(), ally) {
        for m0 in g.tab.enemysquads_squad[a].men.clone() {
            if alive(g, m0) { let p = pos_of(g, m0); if same_room(&room_of(g, a, p, Some(0.05)), &room) { in_room += 1; } }
        }
    }
    let lost = room.is_some() && ally.is_some() && in_room < 2;
    let q_name = squad_name(g, q);
    let ally_name = ally.map(|a| squad_name(g, a));
    let mut choice = SquadChoice::Barricade;
    let mut why: String;
    let mut go: Vec<RoomfightDefId> = Vec::new();
    let mut ways: Vec<Way> = Vec::new();
    let mut stood_to = false;
    if g.tab.enemysquads_squad[q].def.may_leave == Some(false) { why = String::from("its men stay in its rooms"); }
    else if out > 0 { why = String::from("its own men are still out"); }
    else if spare < 2.0 { why = format!("{} at home in {} room{}: nobody to spare", n, rooms, if rooms > 1 { "s" } else { "" }); }
    else if there.len() >= 2 { let names: Vec<String> = there.iter().map(|x| squad_name(g, *x)).collect(); why = format!("{} are already out on it", names.join(" and ")); }
    else if ally.is_some() && ally_up > 0 {
        // somebody to help: a pair goes round; no way round: straight to them; a squad already on its way: this one does the other thing
        let an = ally_name.clone().unwrap();
        let p = if theirs != Some(SentRole::Flank) { Some(pair(g, &st_net, &h, fix.at, round_to, spare, ally, lost)) } else { None };
        match p {
            Some((pg, pw)) if pg.len() >= 2 => {
                choice = SquadChoice::Flank; go = pg; ways = pw;
                why = if theirs.is_some() { format!("{} is going straight to {}: this pair goes round", squad_name(g, there[0]), an) }
                    else if lost { format!("{} {} {}: a pair goes round to hold its other door", an, if in_room > 0 { "has one man left in" } else { "has lost" }, room.as_ref().unwrap().name) }
                    else if ally_up < 2 { format!("{} is down to its last man: a pair goes round behind them", an) }
                    else { format!("{} has {} up to hold them: a pair goes round", an, ally_up) };
            }
            _ => {
                if theirs == Some(SentRole::Help) { why = format!("{} is already going to {}, and no way round", squad_name(g, there[0]), an); }
                else {
                    choice = SquadChoice::Help;
                    let k = m::min(g.k.squads.help_men, spare);
                    go = goers(g, &h, fix.at, k);
                    why = if theirs.is_some() { format!("{} is going round: this one goes straight to {}", squad_name(g, there[0]), an) }
                        else if ally_up < 2 { format!("{} is down to its last man, and no way round", an) }
                        else { format!("{} has {} up, and no way round", an, ally_up) };
                }
            }
        }
    } else if ally.is_some() {
        let an = ally_name.clone().unwrap();
        let p = if there.is_empty() && n >= 4 { Some(pair(g, &st_net, &h, fix.at, round_to, spare, ally, lost)) } else { None };
        match p {
            Some((pg, pw)) if pg.len() >= 2 => { choice = SquadChoice::Flank; go = pg; ways = pw; why = format!("{} is not answering: a pair goes round to where it was, the rest lock down", an); }
            _ => { why = format!("{} is not answering{}", an, if there.is_empty() { "" } else { " and men are already out" }); }
        }
    } else {
        // the team heard on open floor, nobody in contact: nobody walks out at a noise he cannot place; every squad gets on its doors with its answer ready
        let mut first: Option<EnemysquadsSquadId> = None;
        let mut nd = f64::INFINITY;
        for &x in &st_squads {
            let xh = if x == q { h.clone() } else { home(g, x) };
            if xh.is_empty() { continue; }
            let ds: Vec<f64> = xh.iter().map(|d| { let gd = g.tab.roomfight_def[*d].gd; flat(pos_of(g, gd), fix.at) }).collect();
            let d = m::min_n(&ds);
            if d < nd { nd = d; first = Some(x); }
        }
        g.tab.enemysquads_squad[q].ready = true;
        stood_to = first != Some(q);
        why = if first == Some(q) { String::from("the noise is nearest its own rooms: it is next") }
            else { format!("nobody has them in sight{}: on its doors until somebody calls it", match first { Some(x) => format!(" (nearest {})", squad_name(g, x)), None => String::new() }) };
    }
    if choice != SquadChoice::Barricade && go.len() < 2 { choice = SquadChoice::Barricade; go = Vec::new(); why.push_str(" (nobody free to go)"); }
    g.tab.enemysquads_squad[q].state = match choice { SquadChoice::Barricade => SquadState::Barricade, SquadChoice::Flank => SquadState::Flank, SquadChoice::Help => SquadState::Help };
    if choice == SquadChoice::Help {
        for d in &go { send(g, q, *d, SentRole::Help, None, ally, fix.at, lost); }
        let gd0 = g.tab.roomfight_def[go[0]].gd;
        let line = format!("{}: moving to {} — {} of us!", q_name, ally_name.as_deref().unwrap_or("them"), go.len());
        say_radio(g, gd0, &line);
    } else if choice == SquadChoice::Flank {
        for (i, d) in go.iter().enumerate() { send(g, q, *d, SentRole::Flank, Some(&ways[i]), ally, fix.at, lost); }
        let gd0 = g.tab.roomfight_def[go[0]].gd;
        let word = crate::words::wg(g, &"around", &"round");
        let line = format!("{}: {} going {} — {}!", q_name, count_word(go.len()), word, if lost { "cut them off" } else { "keep them there" });
        say_radio(g, gd0, &line);
        let mut by: JsSet<String> = JsSet::new();
        for w in &ways { by.add(match w.door { Some(d) => door_name(g, d), None => String::from("the long way") }); }
        why.push_str(&format!(" (by {})", by.keys().join(" and ")));
    }
    // whoever stays gets ready: the net's word is what digin.ts acts on
    for d in &h { if !go.contains(d) { tell(g, *d, fix.at); } }
    g.tab.enemysquads_squad[q].state0 = Some(choice);
    if choice == SquadChoice::Barricade && before == Some(SquadChoice::Barricade) { note(g, Some(lead), &format!("{} stays on its doors: {}", q_name, why)); return; }   // (said once)
    if choice == SquadChoice::Barricade {
        let line = if stood_to { format!("{}: heard it — on the doors, call it when you see them!", q_name) }
            else if ally.is_some() && ally_up == 0 { format!("{}: {}'s not answering — lock the doors!", q_name, ally_name.as_deref().unwrap()) }
            else { format!("{}: holding — lock the doors, let them come!", q_name) };
        say_radio(g, lead, &line);
    }
    let what = match choice { SquadChoice::Barricade => EventWhat::Barricade, SquadChoice::Flank => EventWhat::Flank, SquadChoice::Help => EventWhat::Help };
    event(g, q, what, &why, Some(lead));
}

/// the door of the room that was hit nearest the place the team was seen or heard, inside 4.5 m (enemysquads.ts doorAt)
pub fn door_at(g: &mut crate::game::Game, room: Option<&crate::scene_scenarios::StructureRoom>, at: V3) -> Option<DoorId> {
    let p = match room { Some(r) => [(r.bounds.x0 + r.bounds.x1) / 2.0, 0.0, (r.bounds.z0 + r.bounds.z1) / 2.0], None => at };
    let t = crate::roomfight::room_fight_room_at(g, p);
    let (mut best, mut bd) = (None, 4.5);
    if let Some(t) = t { for e in &g.tab.roomfight_room[t].entries { let d = flat(e.at, at); if d < bd { bd = d; best = Some(e.door); } } }
    best
}
/// a squad that has the team on it: said on the net by a man able to say it, and its men in its other rooms on their way after their own beat (enemysquads.ts contact)
pub fn contact(g: &mut crate::game::Game, q: EnemysquadsSquadId, at: V3, room: Option<&crate::scene_scenarios::StructureRoom>, by_drop: Option<bool>, by: Option<GuardId>, words: Option<&crate::enemysquads::ContactWords>) {
    let by_drop = by_drop.unwrap_or(false);
    let was = g.tab.enemysquads_squad[q].state;
    let fresh = was != SquadState::Contact;
    let now = g.time;
    { let x = &mut g.tab.enemysquads_squad[q]; x.state = SquadState::Contact; x.contact_t = now; x.ready = false; if let Some(r) = room { x.hit_room = Some(r.clone()); } }
    if net_asks(g) { if let Some(o) = g.tab.enemysquads_squad[q].rc.as_mut().and_then(|c| c.net_out.as_mut()) { o.clear(); } }
    // (`by`, callBeat: the man whose call it is ends it himself)
    let mut caller = None;
    if let Some(b) = by { if !crate::runner::runner_owns(g, b) { caller = Some(b); } }
    if caller.is_none() {
        for m0 in g.tab.enemysquads_squad[q].men.clone() { if fit(g, m0) && !g.tab.enemysquads_squad[q].sent.iter().any(|s| s.gd == m0) { caller = Some(m0); break; } }
    }
    let called_t = g.tab.enemysquads_squad[q].called_t;
    let room_new = room.is_some() && !same_room(&room.cloned(), &g.tab.enemysquads_squad[q].called_room);
    let q_name = squad_name(g, q);
    if caller.is_some() && (g.time - called_t > g.k.squads.recall_secs || fresh && g.time - called_t > 6.0 || room_new) {
        let caller = caller.unwrap();
        let door = if by_drop { None } else { door_at(g, room, at) };
        { let x = &mut g.tab.enemysquads_squad[q]; x.called_t = now; x.called_room = room.cloned(); }
        let net = &mut g.m.enemysquads.states.as_mut().unwrap().net;
        net.push(StateNetItem { t: now, squad: q, at: v3::copy(at), door });
        if net.len() > 40 { net.drain(0..20); }
        let line = match words { Some(w) => w.say.clone(), None => format!("{}: contact — {}!", q_name, room.map_or("here", |r| r.name.as_str())) };
        say_radio(g, caller, &line);
        let head = match words { Some(w) => w.why.clone(), None => String::from(room.map_or("its men", |r| r.name.as_str())) };
        let chose = match was {
            SquadState::Hold | SquadState::Contact => String::new(),
            SquadState::Alerted => String::from(" (before it had said what it would do)"),
            other => format!(" (it had chosen: {})", other.as_str()),
        };
        let met = match door { Some(d) => format!(" · met at {}", door_name(g, d)), None => String::new() };
        event(g, q, EventWhat::Contact, &format!("{}{}{}", head, chose, met), Some(caller));
    } else if caller.is_some() && !by_drop {   // (its call went out before anybody had seen by which door: the men who see it now say so to whoever asks next)
        let last = g.m.enemysquads.states.as_ref().unwrap().net.iter().rfind(|c| c.squad == q).map(|c| (c.door.is_none(), c.t));
        if let Some((no_door, t)) = last {
            if no_door && g.time - t < 30.0 {
                let door = door_at(g, room, at);
                let net = &mut g.m.enemysquads.states.as_mut().unwrap().net;
                if let Some(c) = net.iter_mut().rfind(|c| c.squad == q) { c.door = door; }
            }
        }
    } else if caller.is_none() && fresh { note(g, None, &format!("{} is in contact and has nobody able to call it", q_name)); }
    let Some(room) = room else { return };
    let reinforce = g.k.squads.reinforce;
    for d in home(g, q) {
        let gd = g.tab.roomfight_def[d].gd;
        let p = pos_of(g, gd);
        let his = room_of(g, q, p, None);   // (a man who gave ground out of it is the room fight's: not sent back in)
        if his.is_none() || same_room(&his, &Some(room.clone())) || g.tab.enemysquads_squad[q].due.iter().any(|x| x.gd == gd) { continue; }
        let key = stay_key(g, gd, &StayKeyRoom { name: room.name.clone() });
        if g.tab.enemysquads_squad[q].stayed.as_ref().map_or(false, |s| s.has(key.clone())) && squad_drill(g).map_or(false, |k| k.drill_own) { continue; }   // (drillOwn: he was told that room is gone)
        let t = g.time + pick(g, reinforce) * own_k(g);
        g.tab.enemysquads_squad[q].due.push(SquadDueItem { gd, at: v3::copy(at), t, room: Some(room.clone()) });
    }
}

/// a man's row of his squad's `sent`: a row is named by its man
fn sent_row(g: &Game, q: EnemysquadsSquadId, gd: GuardId) -> Option<Sent> { g.tab.enemysquads_squad[q].sent.iter().find(|r| r.gd == gd).cloned() }
/// a man holding the other door of a room the team has taken hears its next entry go in somewhere else: he and his mate go after it (enemysquads.ts follow)
pub fn follow(g: &mut crate::game::Game, q: EnemysquadsSquadId, s: GuardId, d: RoomfightDefId) {
    let Some(mo) = moved_on(g, q, s, d, 8.0) else { return };
    let Some(row) = sent_row(g, q, s) else { return };
    let gd = s;
    let (at, pair) = (mo.at, mo.pair);
    for &x in &pair {
        let Some(xd) = def_of(g, x) else { continue };
        if g.tab.roomfight_def[xd].mode != crate::roomfight::DefMode::Hold { continue; }
        if let Some(r) = g.tab.enemysquads_squad[q].sent.iter_mut().find(|r| r.gd == x) { r.phase = SentPhase::Loose; r.hold = Some(false); }
        crate::pressure::loose(g, xd, at);
    }
    let line = format!("{}: they've moved on — after them!", squad_name(g, q));
    say_radio(g, gd, &line);
    let names: Vec<String> = pair.iter().map(|x| g.gd[*x].callsign.clone()).collect();
    let door = match row.door { Some(x) => door_name(g, x), None => String::from("the door") };
    let why = format!("{} leave{} {} for the noise {:.0} m off, {:.1} s after setting", names.join(" and "), if pair.len() > 1 { "" } else { "s" }, door, flat(at, pos_of(g, gd)), g.time - row.set_t.unwrap_or(row.t0));
    event(g, q, EventWhat::Follows, &why, Some(gd));
}
/// drillFollow's and drillBack's ears: a holding man hears the team go on somewhere else, and nobody of the pair has one of them in sight (enemysquads.ts movedOn)
pub fn moved_on(g: &mut crate::game::Game, q: EnemysquadsSquadId, s: GuardId, d: RoomfightDefId, reach: f64) -> Option<crate::enemysquads::MovedOn> {
    let gd = s;
    let row = sent_row(g, q, s)?;
    let at = g.tab.roomfight_def[d].noise_at?;
    if g.time - g.tab.roomfight_def[d].noise_t > 1.0 || g.time - row.set_t.unwrap_or(row.t0) < 4.0 || g.time - g.gd[gd].last_seen_t < 3.0 || !fit(g, gd) { return None; }
    let follow_m = match squad_drill(g) { Some(k) => k.follow_m, None => g.k.squads.follow_m };
    let from = row.aim.unwrap_or(pos_of(g, gd));
    if flat(at, from) < follow_m || lost_room_has(g, &row, at) { return None; }
    let mut pair: Vec<GuardId> = Vec::new();
    let pos = pos_of(g, gd);
    for x in g.tab.enemysquads_squad[q].sent.clone() {
        if x.role == SentRole::Flank && x.phase == SentPhase::Set && x.hold == Some(true) && alive(g, x.gd) && fit(g, x.gd) && (x.gd == s || flat(pos_of(g, x.gd), pos) < reach) { pair.push(x.gd); }
    }
    if pair.iter().any(|x| g.time - g.gd[*x].last_seen_t < 3.0) { return None; }   // (his mate has a man in sight: nobody leaves)
    Some(MovedOn { at, pair })
}

#[cfg(test)]
mod tests {
    use super::*;
    // the TypeScript's own bits: its `along` under EXACT.on, a leg, a leg of no length (one point, at its corner), a leg
    const BITS: [[u64; 3]; 22] = [
        [0x3fdd27d27d27d27d, 0xbfe4fa4fa4fa4fa6, 0x3fde22e7bdb3230c],
        [0x3fe38e38e38e38e4, 0xbfcb05b05b05b060, 0x3fee22e7bdb3230c],
        [0x3fe8888888888888, 0x3fcdddddddddddd8, 0x3ff69a2dce465a49],
        [0x3fed82d82d82d82e, 0x3fe5b05b05b05b04, 0x3ffe22e7bdb3230c],
        [0x3ff13e93e93e93e9, 0x3ff1f49f49f49f4a, 0x4002d5d0d68ff5e8],
        [0x3ff3bbbbbbbbbbbb, 0x3ff9111111111110, 0x40069a2dce465a49],
        [0x3ff638e38e38e38e, 0x400016c16c16c16c, 0x400a5e8ac5fcbeab],
        [0x3ff8b60b60b60b61, 0x4003a4fa4fa4fa4f, 0x400e22e7bdb3230c],
        [0x3ffb333333333333, 0x4007333333333333, 0x4010f3a25ab4c3b7],
        [0x3ffb333333333333, 0x4007333333333333, 0x4010f3a25ab4c3b7],
        [0x3ff4888888888888, 0x4005594237fa89e6, 0x4012dbb0ebf36322],
        [0x3febbbbbbbbbbbbb, 0x40037f513cc1e099, 0x4014c3bf7d32028e],
        [0x3fdccccccccccccc, 0x4001a5604189374c, 0x4016abce0e70a1f9],
        [0x3fa1111111111100, 0x3fff96de8ca11bfc, 0x401893dc9faf4165],
        [0xbfd888888888888c, 0x3ffbe2fc962fc962, 0x401a7beb30ede0d0],
        [0xbfe999999999999a, 0x3ff82f1a9fbe76c9, 0x401c63f9c22c803c],
        [0xbff3777777777777, 0x3ff47b38a94d242e, 0x401e4c08536b1fa7],
        [0xbffa222222222223, 0x3ff0c756b2dbd193, 0x40201a0b7254df89],
        [0xc000666666666666, 0x3fea26e978d4fdf0, 0x40210e12baf42f3f],
        [0xc003bbbbbbbbbbbc, 0x3fe2bf258bf258bc, 0x4022021a03937ef4],
        [0xc007111111111110, 0x3fd6aec33e1f6710, 0x4022f6214c32ceaa],
        [0xc00a666666666666, 0x3fbf7ced916872c0, 0x4023ea2894d21e60],
    ];
    const LEN: u64 = 0x4023ea2894d21e60;
    #[test]
    fn along_walks_as_the_typescript_does() {
        let path: [V3; 3] = [[1.7, 0.0, 2.9], [1.7, 0.0, 2.9], [-3.3, 1.0, 0.123]];
        let mut seen: Vec<[u64; 3]> = Vec::new();
        let l = along([0.3, 7.0, -1.1], &path, &mut |p, s| seen.push([p[0].to_bits(), p[2].to_bits(), s.to_bits()]));
        assert_eq!(seen.as_slice(), &BITS[..]);
        assert_eq!(l.to_bits(), LEN);
        assert_eq!(path_len([0.3, 7.0, -1.1], &path).to_bits(), LEN);
    }
    #[test]
    fn grown_widens_every_side() {
        let g0 = grown(crate::scene_scenarios::Rect { x0: 1.0, x1: 2.0, z0: 3.0, z1: 4.0 }, 0.5);
        assert_eq!((g0.x0, g0.x1, g0.z0, g0.z1), (0.5, 2.5, 2.5, 4.5));
    }
}
