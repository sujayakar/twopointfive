//! src/game/darkfoe.ts lines 1 to 258
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A number's part above its floor.   (darkfoe.ts frac)
pub fn frac(x: f64) -> f64 {
    x - m::floor(x)
}

/// His id's share of 0..1: neighbours in the roster differ, no dice.   (darkfoe.ts idShare)
pub fn id_share(g: &mut crate::game::Game, gd: GuardId, salt: Option<f64>) -> f64 {
    let salt = salt.unwrap_or(0.0);
    let c = g.gd[gd].ch;
    frac((g.ch[c].id + salt) * 0.6180339887)
}

/// Does he carry a light of his own to switch on.   (darkfoe.ts darkFoeCarries)
pub fn dark_foe_carries(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].unit.is_some() && id_share(g, gd, None) < g.k.darkfoe.carry
}

/// Is he this rule's: a living enemy unit whose light the mission left off, in live play.   (darkfoe.ts darkFoeRuled)
pub fn dark_foe_ruled(g: &mut crate::game::Game, gd: GuardId) -> bool {
    match &g.gd[gd].unit {
        None => return false,
        Some(u) if u.light => return false,
        Some(_) => {}
    }
    if g.gd[gd].state == crate::game::GuardState::Dead { return false; }
    let c = g.gd[gd].ch;
    if !g.ch[c].alive { return false; }
    let team = g.gd[gd].team;
    crate::teams::is_hostile(g, team, crate::teams::Team::Player)
        && !g.touring
        && !crate::director::director_showing(g)
        && !g.quiet_utility
        && !crate::rescue::rescue_crew_hides(g, gd)
}

/// His lamp burns by this rule this frame: a read, nothing counted.   (darkfoe.ts darkFoeLit)
pub fn dark_foe_lit(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.m.darkfoe.state_of.get(gd).map_or(false, |s| s.lit) && dark_foe_carries(g, gd) && dark_foe_ruled(g, gd)
}

/// A light filter naming his own lamp: making it does nothing, and the stub returns nothing, so `dark_foe_blind` makes the closure itself.   (darkfoe.ts mine)
pub fn mine(g: &mut crate::game::Game, gd: GuardId) {}

/// A light filter that skips no light.   (darkfoe.ts placeOnly)
pub fn place_only() -> bool {
    true
}

/// The meter's reading of a point of floor: everything steady but the lights `skip` names.   (darkfoe.ts floorLight)
pub fn floor_light(g: &mut crate::game::Game, x: f64, z: f64, skip: &mut dyn FnMut(&mut crate::game::Game, f64) -> bool) -> f64 {
    let e = crate::guards::light_estimate_at(g, [x, 1.2, z], Some(skip), Some(true), None, None);
    crate::game::game_visibility_of(g, e)
}

/// Is the floor he is watching dark to him, his own lamp left out.   (darkfoe.ts darkFoeBlind)
pub fn dark_foe_blind(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let k = g.k.darkfoe;
    let c = g.gd[gd].ch;
    let (pos, aim_yaw, me) = (g.ch[c].pos, g.ch[c].aim_yaw, g.ch[c].id);
    let eye: V3 = [pos[0], 1.5, pos[2]];
    let mut skip = move |_: &mut Game, owner: f64| owner == me;   // (TS: mine(gd))
    let mut n: i32 = 0;
    let mut lit: i32 = 0;
    g.stats.darkfoe.asks += 1;
    for off in [0.0, -k.fan_rad, k.fan_rad] {
        let fx = m::sin(aim_yaw + off);
        let fz = m::cos(aim_yaw + off);
        let mut r = k.step_m;
        while r <= k.look_m + 1e-6 {
            let x = eye[0] + fx * r;
            let z = eye[2] + fz * r;
            if crate::scene_collision::segment_blocked(g, eye, [x, 1.5, z], None, None, None) { break; }
            n += 1;
            if floor_light(g, x, z, &mut skip) >= k.lit_at { lit += 1; }
            r += k.step_m;
        }
    }
    if n == 0 { return floor_light(g, pos[0], pos[2], &mut skip) < k.lit_at; }
    (lit as f64) / (n as f64) < k.see_share
}

/// Has it reached him for good: alert or searching, or his ears counted gunfire, a kick or a bang.   (darkfoe.ts learns)
pub fn learns(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let man = &g.gd[gd];
    let h = &man.witness.heard;
    man.state == crate::game::GuardState::Alert
        || man.state == crate::game::GuardState::Search
        || h.shots + h.kick + h.bang > 0.0
        || man.witness.alerted_by.is_some()
}

/// Or for now: the net's word on the dark, or a burning lamp of a mate who knows in his view.   (darkfoe.ts hasWord)
pub fn has_word(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if crate::powerout::power_out_word(g, gd) { return true; }
    let c = g.gd[gd].ch;
    let (pos, aim_yaw) = (g.ch[c].pos, g.ch[c].aim_yaw);
    let team = g.gd[gd].team;
    let eye: V3 = [pos[0], 1.5, pos[2]];
    let cos = m::cos(g.k.guard_eye.half_deg * m::PI / 180.0);
    let fx = m::sin(aim_yaw);
    let fz = m::cos(aim_yaw);
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o == gd || g.gd[o].team != team || g.gd[o].state == crate::game::GuardState::Dead { continue; }
        if !g.m.darkfoe.state_of.get(o).map_or(false, |s| s.knows) || !dark_foe_lit(g, o) { continue; }
        let light = g.gd[o].flashlight;
        if !seam_light_enabled(g, light) { continue; }
        let oc = g.gd[o].ch;
        let op = g.ch[oc].pos;
        let dx = op[0] - pos[0];
        let dz = op[2] - pos[2];
        let d = m::hyp2(dx, dz);
        if d > g.k.darkfoe.lamp_m || d < 1e-3 || (dx * fx + dz * fz) / d < cos { continue; }
        if !crate::scene_collision::segment_blocked(g, eye, [op[0], 1.5, op[2]], None, None, None) { return true; }
    }
    false
}

/// The switch under his thumb, for the player's ears only: no event is filed.   (darkfoe.ts click)
pub fn click(g: &mut crate::game::Game, gd: GuardId, on: bool) {
    if on { g.stats.darkfoe.clicks_on += 1; } else { g.stats.darkfoe.clicks_off += 1; }
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    crate::audio_iface::game_audio_like_play(g, "click", Some(pos), Some(0.25), None);
}

/// Lit floor in plain view of a point within `edgeM`: eight ways, at half of it and at all of it.   (darkfoe.ts litInView)
pub fn lit_in_view(g: &mut crate::game::Game, p: V3) -> bool {
    let k = g.k.darkfoe;
    let eye: V3 = [p[0], 1.5, p[2]];
    for ring in 1..=2 {
        let r = k.edge_m * (ring as f64) / 2.0;
        let turn = if ring & 1 != 0 { 0.5 } else { 0.0 };
        for i in 0..8 {
            let a = ((i as f64) + turn) * m::PI / 4.0;
            let qx = p[0] + m::sin(a) * r;
            let qz = p[2] + m::cos(a) * r;
            if !crate::scene_collision::segment_blocked(g, eye, [qx, 1.5, qz], None, None, None) && floor_light(g, qx, qz, &mut |_: &mut Game, _: f64| place_only()) >= k.lit_at {
                return true;
            }
        }
    }
    false
}

/// Floor a man can see from: the place's own light on it, or lit floor in plain view; `edge`: only the second.   (darkfoe.ts darkFoeSees)
pub fn dark_foe_sees(g: &mut crate::game::Game, p: V3, edge: Option<bool>) -> bool {
    let edge = edge.unwrap_or(false);
    let lit = floor_light(g, p[0], p[2], &mut |_: &mut Game, _: f64| place_only()) >= g.k.darkfoe.lit_at;
    if edge { !lit && lit_in_view(g, p) } else { lit || lit_in_view(g, p) }
}

/// Is a point in his room; a man with no room is everywhere in it.   (darkfoe.ts inRoom)
pub fn in_room(g: &mut crate::game::Game, d: RoomfightDefId, p: V3) -> bool {
    let Some(room) = g.tab.roomfight_def[d].room else { return true };
    let cell = g.col.nav.cell;
    let key = crate::scene_collision::nav_grid_idx(g, m::floor(p[0] / cell), m::floor(p[2] / cell));
    g.tab.roomfight_room[room].cells.has(crate::Num(key))
}

/// Where the trouble is, as the room fight reads it for him.   (darkfoe.ts threatOf)
pub fn threat_of(g: &mut crate::game::Game, d: RoomfightDefId) -> V3 {
    let gd = g.tab.roomfight_def[d].gd;
    if g.time - g.gd[gd].sight_t < 6.0 { return g.gd[gd].sight_pos; }
    if let Some(src) = g.gd[gd].sup.src {
        if g.time - g.gd[gd].sup.src_t < 6.0 { return src; }
    }
    let row = &g.tab.roomfight_def[d];
    let mut bi: Option<usize> = None;
    let mut bv = 0.3;
    for (i, &v) in row.ev.iter().enumerate() {
        if v > bv {
            bv = v;
            bi = Some(i);
        }
    }
    match (bi, row.room) {
        (Some(i), Some(room)) => g.tab.roomfight_room[room].entries[i].at,
        _ => row.pt,
    }
}

/// His room is his fence: always, `roam` being off on the page.   (darkfoe.ts fenced)
pub fn fenced(g: &mut crate::game::Game, gd: GuardId) -> bool {
    true
}

/// A way of no more than `walkM` to a spot, never by a doorway of his room nor out of it.   (darkfoe.ts wayTo)
pub fn way_to(g: &mut crate::game::Game, d: RoomfightDefId, q: V3) -> bool {
    let gd = g.tab.roomfight_def[d].gd;
    let c = g.gd[gd].ch;
    let start = g.ch[c].pos;
    let path = match crate::scene_collision::nav_grid_find_path(g, start, q, Some(6000.0), None) {
        Some(path) if !path.is_empty() => path,
        _ => return false,
    };
    let room = g.tab.roomfight_def[d].room;
    let mut len = 0.0;
    let mut w0 = start;
    for w in path {
        len += v3::dist_xz(w0, w);
        w0 = w;
        if let Some(room) = room {
            if fenced(g, gd)
                && ((!in_room(g, d, w) && v3::dist_xz(w, q) > 0.8)
                    || g.tab.roomfight_room[room].entries.iter().any(|e| v3::dist_xz(w, e.at) < 0.9))
            {
                return false;
            }
        }
    }
    v3::dist_xz(w0, q) <= 0.8 && len <= g.k.darkfoe.walk_m
}

/// May he stand at `p`: room for his shoulders, his own room, off its doorways, no nearer the trouble, nobody there or sent there.   (darkfoe.ts darkFoeSpot's free)
fn free(g: &mut Game, d: RoomfightDefId, threat: V3, far: f64, p: V3) -> bool {
    let gd = g.tab.roomfight_def[d].gd;
    let (x, z) = (p[0], p[2]);
    let cs = g.col.nav.cell;
    if crate::scene_collision::nav_grid_is_blocked(g, x, z)
        || crate::scene_collision::nav_grid_is_blocked(g, x + cs, z)
        || crate::scene_collision::nav_grid_is_blocked(g, x - cs, z)
        || crate::scene_collision::nav_grid_is_blocked(g, x, z + cs)
        || crate::scene_collision::nav_grid_is_blocked(g, x, z - cs)
    {
        return false;
    }
    if fenced(g, gd) && !in_room(g, d, p) { return false; }
    if crate::lanes::lane_warns(g, gd, Some(p)) { return false; }
    if let Some(room) = g.tab.roomfight_def[d].room {
        if g.tab.roomfight_room[room].entries.iter().any(|e| v3::dist_xz(p, e.at) < 1.2) { return false; }
    }
    if !(v3::dist_xz(p, threat) >= far) { return false; }
    !g.guards.iter().any(|&o| {
        if o == gd || g.gd[o].state == crate::game::GuardState::Dead { return false; }
        let sent = g.m.darkfoe.state_of.get(o).and_then(|s| s.to);
        v3::dist_xz(g.ch[g.gd[o].ch].pos, p) < 0.9 || sent.map_or(false, |t| v3::dist_xz(t, p) < 0.9)
    })
}

/// Where a man with no light goes: the nearest floor of his room he can see from, else the shoulder of a mate whose lamp burns.   (darkfoe.ts darkFoeSpot)
pub fn dark_foe_spot(g: &mut crate::game::Game, d: RoomfightDefId) -> Option<crate::darkfoe::DarkFoeSpot> {
    let k = g.k.darkfoe;
    let gd = g.tab.roomfight_def[d].gd;
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let threat = threat_of(g, d);
    let far = v3::dist_xz(pos, threat) - 0.5;
    for edge in [true, false] {
        let mut tries = 0.0;
        let mut r = 2.0;
        while r <= k.seek_m + 1e-6 && tries < WAY_TRIES {
            let n = m::max(8.0, m::round(r * 2.5));
            let turn = if js::truthy(js::rem(r, 3.0)) { 0.5 } else { 0.0 };
            let mut i = 0.0;
            while i < n && tries < WAY_TRIES {
                let a = (i + turn) * 2.0 * m::PI / n;
                let p: V3 = [pos[0] + m::sin(a) * r, 0.0, pos[2] + m::cos(a) * r];
                i += 1.0;
                if !free(g, d, threat, far, p) || !dark_foe_sees(g, p, Some(edge)) { continue; }
                tries += 1.0;
                if way_to(g, d, p) { return Some(crate::darkfoe::DarkFoeSpot { spot: p, mate: false }); }
            }
            r += 1.5;
        }
    }
    let team = g.gd[gd].team;
    let mut best: Option<V3> = None;
    let mut bd = f64::INFINITY;
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o == gd || g.gd[o].team != team || g.gd[o].state == crate::game::GuardState::Dead || !dark_foe_lit(g, o) { continue; }
        let light = g.gd[o].flashlight;
        if !seam_light_enabled(g, light) { continue; }
        let oc = g.gd[o].ch;
        let op = g.ch[oc].pos;
        let dist = v3::dist_xz(op, pos);
        if dist < 2.5 || dist > k.walk_m || dist >= bd { continue; }
        let f = crate::character::character_forward(g, oc);
        for side in [1.0, -1.0] {
            let p: V3 = [op[0] - f[0] * 0.9 + f[2] * 1.1 * side, 0.0, op[2] - f[2] * 0.9 - f[0] * 1.1 * side];
            if free(g, d, threat, far, p) && way_to(g, d, p) {
                best = Some(p);
                bd = dist;
                break;
            }
        }
    }
    best.map(|spot| crate::darkfoe::DarkFoeSpot { spot, mate: true })
}

/// The room fight's own 'move' leg: his script's legs, his own senses, planted where he gets to.   (darkfoe.ts walk)
pub fn walk(g: &mut crate::game::Game, d: RoomfightDefId, goal: V3) {
    let gd = g.tab.roomfight_def[d].gd;
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let row = &mut g.tab.roomfight_def[d];
    row.mode = crate::roomfight::DefMode::Move;
    row.move_kind = crate::roomfight::DefMoveKind::Fallback;
    row.came_by = None;
    row.move_t = 0.0;
    row.stall_at = v3::copy(pos);
    row.stall_t = 0.0;
    row.rally_at = f64::NAN;
    g.gd[gd].pinned = false;
    let speed = g.tune.guard_investigate * g.k.darkfoe.stumble_k;
    crate::squad::order(g, gd, crate::squad::OrderPatch {
        goal: Some(Some(v3::copy(goal))),
        speed: Some(Some(speed)),
        face: Some(None),
        strafe: Some(Some(false)),
        aim_at: Some(None),
        upper: Some(Some(crate::character::UpperMode::Aim)),
        stance: Some(Some(crate::character::Stance::None)),
        crouch: Some(Some(false)),
        ..Default::default()
    });
}

/// Why a blind man with no light may not leave his post now (None = he may).   (darkfoe.ts darkFoeKept)
pub fn dark_foe_kept(g: &mut crate::game::Game, gd: GuardId, d: Option<RoomfightDefId>) -> Option<String> {
    let d = match d {
        Some(d) => Some(d),
        None => g.fight.defs.iter().copied().find(|&row| g.tab.roomfight_def[row].gd == gd),
    };
    let Some(d) = d else { return Some(String::from("not a room's defender")) };
    if g.tab.roomfight_def[d].mode != crate::roomfight::DefMode::Hold { return Some(String::from("on the move")); }
    if g.tab.roomfight_def[d].rally_at == f64::INFINITY { return Some(String::from("the mission's man")); }
    if g.tab.roomfight_def[d].room.is_none() { return Some(String::from("no room of his own")); }
    if g.gd[gd].script.is_some() || g.gd[gd].held.is_some() || g.time < g.gd[gd].dazzled_until { return Some(String::from("not his own man")); }
    if g.gd[gd].state != crate::game::GuardState::Alert { return Some(String::from("not alert")); }
    if g.time - g.gd[gd].last_seen_t < g.k.darkfoe.quiet_secs
        || g.time - g.gd[gd].witness.heard.last_shot_t < g.k.darkfoe.calm_secs
        || g.gd[gd].sup.v > 0.05
    {
        return Some(String::from("in a fight"));
    }
    if crate::digin::dig_in_phase(g, gd) != "idle" { return Some(String::from("dug in")); }
    if crate::enemysquads::squad_defers(g, gd) { return Some(String::from("his squad's")); }
    if crate::lanes::lane_holds(g, gd, None) { return Some(String::from("holds a lane")); }
    // pressOwns may stop his rally clock as it answers: asked here it changes nothing.
    let rally = g.tab.roomfight_def[d].rally_at;
    let press = crate::pressure::press_owns(g, d);
    g.tab.roomfight_def[d].rally_at = rally;
    if press || crate::buddy::buddy_owns(g, d) { return Some(String::from("the press's")); }
    if crate::rescue::rescue_captor(g) == Some(gd) || crate::laststand::last_stand_man(g, gd) {
        return Some(String::from("the captor / last stand"));
    }
    None
}

/// Each frame, after the room fight: who knows, who is blind, whose lamp moves, who walks.   (darkfoe.ts darkFoeTick)
pub fn dark_foe_tick(g: &mut crate::game::Game, dt: f64) {
    if dt <= 0.0 { return; }
    let k = g.k.darkfoe;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if !dark_foe_ruled(g, gd) {
            if let Some(s) = g.m.darkfoe.state_of.get_mut(gd) { s.lit = false; }
            continue;
        }
        if !g.m.darkfoe.state_of.has(gd) {
            let ask = id_share(g, gd, Some(7.0)) * k.ask_secs;
            g.m.darkfoe.state_of.set(gd, FoeState {
                knows: false,
                word: false,
                blind: false,
                ask,
                lit: false,
                against: 0.0,
                held: k.hold_secs,
                blind_for: 0.0,
                moves: 0.0,
                since: k.again_secs,
                to: None,
            });
        }
        let s = g.m.darkfoe.state_of.get_mut(gd).unwrap();
        s.held = m::min(s.held + dt, 60.0);
        s.since = m::min(s.since + dt, 600.0);
        s.ask += dt;
        if s.ask >= k.ask_secs {
            s.ask = 0.0;
            let knew = s.knows;
            if !knew {
                let knows = learns(g, gd);
                g.m.darkfoe.state_of.get_mut(gd).unwrap().knows = knows;
            }
            let knows = g.m.darkfoe.state_of.get(gd).unwrap().knows;
            let word = !knows && has_word(g, gd);
            g.m.darkfoe.state_of.get_mut(gd).unwrap().word = word;
            let blind = (knows || word) && dark_foe_blind(g, gd);
            g.m.darkfoe.state_of.get_mut(gd).unwrap().blind = blind;
        }
        if !dark_foe_carries(g, gd) {
            g.m.darkfoe.state_of.get_mut(gd).unwrap().lit = false;
        }
        if dark_foe_carries(g, gd) {
            let row = *g.m.darkfoe.state_of.get(gd).unwrap();
            if row.blind == row.lit {
                g.m.darkfoe.state_of.get_mut(gd).unwrap().against = 0.0;
            } else {
                let against = row.against + dt;
                g.m.darkfoe.state_of.get_mut(gd).unwrap().against = against;
                let need = if row.lit {
                    k.off_secs * (1.0 + id_share(g, gd, Some(9.0)) * k.off_jitter)
                } else {
                    k.on_secs + id_share(g, gd, Some(3.0)) * k.on_jitter
                };
                if against >= need && row.held >= k.hold_secs {
                    let lit = !row.lit;
                    let s = g.m.darkfoe.state_of.get_mut(gd).unwrap();
                    s.lit = lit;
                    s.against = 0.0;
                    s.held = 0.0;
                    click(g, gd, lit);
                }
            }
            continue;
        }
        if !g.m.darkfoe.state_of.get(gd).unwrap().knows { continue; }
        let s = g.m.darkfoe.state_of.get_mut(gd).unwrap();
        s.blind_for = if s.blind { s.blind_for + dt } else { 0.0 };
        let row = *s;
        if row.blind_for < k.blind_secs + id_share(g, gd, Some(5.0)) * 2.0 || row.moves >= k.moves || row.since < k.again_secs { continue; }
        let Some(d) = g.fight.defs.iter().copied().find(|&row| g.tab.roomfight_def[row].gd == gd) else { continue };
        if dark_foe_kept(g, gd, Some(d)).is_some() { continue; }
        let s = g.m.darkfoe.state_of.get_mut(gd).unwrap();
        s.since = 0.0;
        s.blind_for = 0.0;
        let c = g.gd[gd].ch;
        let pos = g.ch[c].pos;
        if dark_foe_sees(g, pos, None) { continue; }
        let Some(to) = dark_foe_spot(g, d) else {
            g.stats.darkfoe.no_spot += 1;
            continue;
        };
        let s = g.m.darkfoe.state_of.get_mut(gd).unwrap();
        s.moves += 1.0;
        s.to = Some(v3::copy(to.spot));
        g.stats.darkfoe.walks += 1;
        if to.mate { g.stats.darkfoe.to_mate += 1; }
        walk(g, d, to.spot);
    }
}

/// QA / the panel: what the rule has of a man (None = never asked of him).   (darkfoe.ts darkFoeOf)
pub fn dark_foe_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::darkfoe::DarkFoeOf> {
    g.m.darkfoe.state_of.get(gd).map(|s| crate::darkfoe::DarkFoeOf {
        knows: s.knows,
        word: s.word,
        blind: s.blind,
        ask: s.ask,
        lit: s.lit,
        against: s.against,
        held: s.held,
        blind_for: s.blind_for,
        moves: s.moves,
        since: s.since,
        to: s.to,
    })
}

/// The debug panel's line: what the rule has of each man it rules.   (darkfoe.ts darkFoeSummary)
pub fn dark_foe_summary(g: &mut crate::game::Game) -> String {
    let mut men: Vec<String> = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if !dark_foe_ruled(g, gd) { continue; }
        let s = g.m.darkfoe.state_of.get(gd).copied();
        let has = dark_foe_carries(g, gd);
        let what = match s {
            Some(s) if s.knows || s.word => {
                if s.lit {
                    String::from("lamp ON")
                } else if !s.blind {
                    String::from("can see")
                } else if has {
                    String::from("blind, lamp coming")
                } else if js::truthy(s.moves) {
                    String::from("no light, walked")
                } else {
                    let c = g.gd[gd].ch;
                    let pos = g.ch[c].pos;
                    let why = if dark_foe_sees(g, pos, None) {
                        String::from(" by lit floor")
                    } else if dark_foe_kept(g, gd, None).is_some() {
                        alloc::format!(" ({})", dark_foe_kept(g, gd, None).unwrap_or_default())
                    } else {
                        String::new()
                    };
                    alloc::format!("no light, blind{}", why)
                }
            }
            _ => String::from("does not know"),
        };
        men.push(alloc::format!("{} {}", g.gd[gd].callsign, what));
    }
    let list = men.join(" \u{b7} ");
    alloc::format!("dark \u{25b8} {}", if list.is_empty() { "no defender with a dark light" } else { list.as_str() })
}

/// (darkfoe.ts:120, 174 o.flashlight.enabled) RtLight's `enabled` has no accessor over a LightId.
fn seam_light_enabled(g: &mut Game, l: LightId) -> bool {
    g.tab.light[l].enabled // (skeleton 4 has the row)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// (id + salt) * 0.6180339887 and its frac, printed by the TypeScript under EXACT.
    #[test]
    fn frac_has_the_typescripts_bits() {
        for (id, salt, whole, part) in [
            (12.0, 0.0, 0x401daa66d2bd94b0u64, 0x3fdaa66d2bd94b00u64),
            (17.0, 7.0, 0x402daa66d2bd94b0, 0x3feaa66d2bd94b00),
            (23.0, 9.0, 0x4033c6ef37290dcb, 0x3fe8dde6e521b960),
            (101.0, 3.0, 0x405011a25cd15b35, 0x3fd1a25cd15b3500),
            (3.0, 5.0, 0x4013c6ef37290dcb, 0x3fee3779b9486e58),
        ] {
            let x: f64 = (id + salt) * 0.6180339887;
            assert_eq!(x.to_bits(), whole);
            assert_eq!(frac(x).to_bits(), part);
        }
        assert_eq!(frac(-0.25).to_bits(), 0x3fe8000000000000);
    }
}
