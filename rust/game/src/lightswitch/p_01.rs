//! src/game/lightswitch.ts lines 1 to 317
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

type Ray = crate::scene_collision::Raycast;

// ---------------------------------------------------------------- seam: what the skeleton still cannot do (notes.md, Skeleton)
/// `S.sweeps(x, z, pad)`: the closure of `doors.swingOf`'s answer, which `DoorSwing` lost
fn seam_swing_sweeps(s: &crate::doors::DoorSwing, x: f64, z: f64, pad: f64) -> bool { crate::doors::p_02::swing_sweeps(s, x, z, pad) } // SKELETON-GAP: crate::doors::door_swing_sweeps(&DoorSwing, x, z, pad), the closure of swingOf's answer (doors.ts:559); the body is doors/p_02.rs swing_sweeps

// ---------------------------------------------------------------- finding them
/// Is the rule running: nothing under a show. (lightswitch.ts switchesLive)
pub fn switches_live(g: &mut crate::game::Game) -> bool {
    !g.touring && !crate::director::director_showing(g)
}

/// Is this fixture one a wall switch works: on the mains and a light of the room. (lightswitch.ts roomLight)
pub fn room_light(g: &mut crate::game::Game, t: FixtureId) -> bool {
    let f = &g.tab.sim_lightshome_fixture[t];
    if !f.mains || f.group == "exterior" || f.group == "emergency" { return false; }
    if f.kind == crate::sim_lightshome::FixtureKind::Analytic { return f.light.is_some(); }
    if f.kind != crate::sim_lightshome::FixtureKind::Emissive || f.boxes.is_empty() { return false; }
    let b = &g.level.boxes[f.boxes[0] as usize];
    b.h[1] <= b.h[0] && b.h[1] <= b.h[2] && b.c[1] > 1.5
}

/// The room a fixture hangs in: of the floor under it, or of the nearest floor it has a clear line down to. (lightswitch.ts roomOfLight)
pub fn room_of_light(g: &mut crate::game::Game, t: FixtureId) -> f64 {
    let pos = g.tab.sim_lightshome_fixture[t].pos;
    let r = crate::cleared::cleared_rooms_room_at(g, pos);
    if js::truthy(r) { return r; }
    let cs = g.col.nav.cell;
    let from: V3 = [pos[0], m::max(1.2, m::min(pos[1] - 0.25, 2.4)), pos[2]];
    let (mut best, mut bd) = (0.0, 2.6);
    for dz in -4..=4 {
        for dx in -4..=4 {
            let p: V3 = [(m::floor(pos[0] / cs) + dx as f64 + 0.5) * cs, 1.2, (m::floor(pos[2] / cs) + dz as f64 + 0.5) * cs];
            let q = crate::cleared::cleared_rooms_room_at(g, p);
            if !js::truthy(q) { continue; }
            let d = m::hyp2(p[0] - pos[0], p[2] - pos[2]);
            if d < bd && !crate::scene_collision::solid_blocked(g, from, p, None) { bd = d; best = q; }
        }
    }
    best
}

/// A ray at the level's own boxes, the leaves aside. (lightswitch.ts wallRay)
pub fn wall_ray(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::Raycast> {
    let keep = core::mem::take(&mut g.col.dynamic_boxes);
    let h = crate::scene_collision::raycast(g, ro, rd, tmax);
    g.col.dynamic_boxes = keep;
    h
}

/// A WALL under the ray: not glass, not a leaf, standing to above a door's head. (lightswitch.ts plateOn's `solid`)
fn solid(g: &mut Game, h: &Option<Ray>) -> bool {
    let Some(h) = h else { return false };
    if h.index >= 1e6 { return false; }
    let b = &g.level.boxes[h.index as usize];
    if js::to_i32(b.flags) & js::to_i32(crate::scene_boxes::box_flag().glass) != 0 { return false; }
    b.c[1] + b.h[1] >= WALL_TOP
}

/// A plate on the wall behind `stand`, or none: plain solid wall under all of it, room for a man, this room's floor at his back, clear of every leaf. (lightswitch.ts plateOn)
pub fn plate_on(g: &mut crate::game::Game, stand: V3, n: V3, room: f64, d: DoorId) -> Option<crate::lightswitch::Plate> {
    let (height, stand_m) = (g.k.lightswitch.height, g.k.lightswitch.stand_m);
    let back: V3 = [-n[0], 0.0, -n[2]];
    let (ux, uz) = (n[2], -n[0]);
    let mut fit: V3 = [stand[0], 0.0, stand[2]];
    crate::scene_collision::collide_circle(g, &mut fit, 0.3, 0.2, 1.5, None);
    if m::hyp2(fit[0] - stand[0], fit[2] - stand[2]) > 0.02 { return None; }
    let mut floor: Option<V3> = None;
    let mut o = 0.0;
    while o <= 0.51 && floor.is_none() {
        let q: V3 = [stand[0] + n[0] * o, 0.0, stand[2] + n[2] * o];
        if !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2])
            && crate::cleared::cleared_rooms_room_at(g, q) == room
            && !crate::scene_collision::solid_blocked(g, [stand[0], 1.0, stand[2]], [q[0], 1.0, q[2]], None)
        {
            floor = Some(q);
        }
        o += 0.25;
    }
    let floor = floor?;
    let ro: V3 = [stand[0], height, stand[2]];
    let h = wall_ray(g, ro, back, stand_m + 0.45);
    if !solid(g, &h) { return None; }
    let h = h?;
    if h.t < 0.2 || v3::dot(h.n, n) < 0.95 { return None; }
    for (da, dy) in [(0.1, 0.0), (-0.1, 0.0), (0.0, 0.1), (0.0, -0.1)] {
        let hq = wall_ray(g, [ro[0] + ux * da, ro[1] + dy, ro[2] + uz * da], back, stand_m + 0.45);
        if !solid(g, &hq) { return None; }
        let hq = hq?;
        if m::abs(hq.t - h.t) > 0.01 { return None; }
    }
    let pos = v3::mad(ro, back, h.t);
    let face = v3::mad(pos, n, 0.06);
    let doors = g.doors.list.clone();
    for o in doors {
        if v3::dist_xz(g.dr[o].frame_centre, pos) > 3.0 { continue; }
        let s = crate::doors::doors_swing_of(g, o);
        if seam_swing_sweeps(&s, face[0], face[2], 0.08) || seam_swing_sweeps(&s, stand[0], stand[2], 0.3) { return None; }
    }
    if crate::scene_collision::solid_blocked(g, [stand[0], 1.3, stand[2]], v3::mad(pos, n, 0.05), None) { return None; }
    Some(Plate { pos, n, stand: [pos[0] + n[0] * stand_m, 0.0, pos[2] + n[2] * stand_m], floor, door: d, room })
}

/// The plate a door gives the room on its `side` face: past a jamb along the door's own wall, the latch side first. (lightswitch.ts plateAt)
pub fn plate_at(g: &mut crate::game::Game, d: DoorId, side: f64, room: f64) -> Option<crate::lightswitch::Plate> {
    let f = crate::guards::frame_of(g, d);
    let n: V3 = [f.nx * side, 0.0, f.nz * side];
    let (beside, stand_m) = (g.k.lightswitch.beside.clone(), g.k.lightswitch.stand_m);
    for latch in [true, false] {
        for &s in &beside {
            let along = if latch { f.w + s } else { -s };
            let p = plate_on(g, [f.hx + f.ux * along + n[0] * stand_m, 0.0, f.hz + f.uz * along + n[2] * stand_m], n, room, d);
            if p.is_some() { return p; }
        }
    }
    None
}

/// Where that wall gives none: the plain wall of the room nearest the door, inside `scanM` of it. (lightswitch.ts plateNear)
pub fn plate_near(g: &mut crate::game::Game, d: DoorId, room: f64) -> Option<crate::lightswitch::Plate> {
    let (scan_m, height, stand_m) = (g.k.lightswitch.scan_m, g.k.lightswitch.height, g.k.lightswitch.stand_m);
    let cs = g.col.nav.cell;
    let c = g.dr[d].frame_centre;
    let r = m::ceil(scan_m / cs) as i32;
    let mut cells: Vec<(V3, f64)> = Vec::new();
    for dz in -r..=r {
        for dx in -r..=r {
            let p: V3 = [(m::floor(c[0] / cs) + dx as f64 + 0.5) * cs, 0.0, (m::floor(c[2] / cs) + dz as f64 + 0.5) * cs];
            let dd = v3::dist_xz(p, c);
            if dd <= scan_m && crate::cleared::cleared_rooms_room_at(g, p) == room { cells.push((p, dd)); }
        }
    }
    js::sort_by(&mut cells, |a, b| js::or(js::or(a.1 - b.1, a.0[0] - b.0[0]), a.0[2] - b.0[2]));
    let ns: [V3; 4] = [[1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, -1.0]];
    for (p, _) in cells {
        for n in ns {
            let Some(h) = wall_ray(g, [p[0], height, p[2]], [-n[0], 0.0, -n[2]], 1.0) else { continue };
            let q = plate_on(g, [p[0] - n[0] * (h.t - stand_m), 0.0, p[2] - n[2] * (h.t - stand_m)], n, room, d);
            if q.is_some() { return q; }
        }
    }
    None
}

/// `fewest`'s rule on plain numbers. A plate is told from another by its place in `ps`, not by its value (TypeScript: `keep.includes(p)`, by identity).
fn fewest_of(ps: &[Plate], per_room: f64, apart_m: f64) -> Vec<Plate> {
    let mut keep: Vec<usize> = if ps.is_empty() { Vec::new() } else { vec![0] };
    while (keep.len() as f64) < per_room {
        let mut far: Option<usize> = None;
        let mut fd = apart_m;
        for (i, p) in ps.iter().enumerate() {
            if keep.contains(&i) { continue; }
            let ds: Vec<f64> = keep.iter().map(|&k| v3::dist_xz(p.pos, ps[k].pos)).collect();
            let dd = m::min_n(&ds);
            if dd >= fd { fd = dd; far = Some(i); }
        }
        let Some(i) = far else { break };
        keep.push(i);
    }
    keep.iter().map(|&i| ps[i]).collect()
}

/// Of a circuit's plates, the `perRoom` to keep: the first found, then each time the furthest from those kept, while far enough. (lightswitch.ts fewest)
pub fn fewest(g: &mut crate::game::Game, ps: &[crate::lightswitch::Plate]) -> Vec<crate::lightswitch::Plate> {
    fewest_of(ps, g.k.lightswitch.per_room, g.k.lightswitch.apart_m)
}

/// Find the level's plates: each room with mains lights, each door of it, a plate a circuit. (lightswitch.ts build)
pub fn build(g: &mut crate::game::Game) -> Vec<crate::lightswitch::WallSwitch> {
    let mut lights: JsMap<crate::Num, Vec<FixtureId>> = JsMap::default();
    let targets = g.targets.clone();
    for t in targets {
        if !room_light(g, t) { continue; }
        let r = room_of_light(g, t);
        if js::truthy(r) { lights.get_or_insert_with(crate::Num(r), Vec::new).push(t); }
    }
    let mut found: JsMap<crate::Num, Vec<Plate>> = JsMap::default();
    let mut sides: Vec<(DoorId, f64)> = Vec::new();
    let doors = g.doors.list.clone();
    for &d in &doors {
        for side in [1.0, -1.0] {
            let r = crate::cleared::cleared_rooms_room_by(g, d, side);
            if !js::truthy(r) || !lights.has(crate::Num(r)) { continue; }
            sides.push((d, r));
            if let Some(p) = plate_at(g, d, side, r) { found.get_or_insert_with(crate::Num(r), Vec::new).push(p); }
        }
    }
    for &(d, r) in &sides {
        if !found.has(crate::Num(r)) {
            if let Some(p) = plate_near(g, d, r) { found.set(crate::Num(r), vec![p]); }
        }
    }
    let mut out: Vec<WallSwitch> = Vec::new();
    for r in found.keys() {
        // a room lit in several circuits has a switch for each: a plate works the circuit whose fixture hangs nearest it
        let ps = found.get(r).unwrap();
        let all = lights.get(r).unwrap();
        let mut by: JsMap<String, Vec<Plate>> = JsMap::default();
        for p in ps {
            let mut best = g.tab.sim_lightshome_fixture[all[0]].group.clone();
            let mut bd = m::INF;
            for &t in all {
                let dd = v3::dist_xz(g.tab.sim_lightshome_fixture[t].pos, p.pos);
                if dd < bd { bd = dd; best = g.tab.sim_lightshome_fixture[t].group.clone(); }
            }
            by.get_or_insert_with(best, Vec::new).push(*p);
        }
        for grp in by.keys() {
            let targets: Vec<FixtureId> = all.iter().copied().filter(|&t| g.tab.sim_lightshome_fixture[t].group == grp).collect();
            let gps = by.get(grp).unwrap();
            for p in fewest(g, gps) {
                let id = out.len() as f64;
                out.push(WallSwitch { id, pos: p.pos, n: p.n, stand: p.stand, floor: p.floor, door: p.door, room: p.room, targets: targets.clone() });
            }
        }
    }
    g.stats.lightswitch.plates += out.len() as i32;
    out
}

/// The level's plates whose wall stands, found on first asking. (lightswitch.ts wallSwitches)
pub fn wall_switches(g: &mut crate::game::Game) -> Vec<crate::lightswitch::WallSwitch> {
    if let Some(st) = &g.m.lightswitch.states { return st.list.clone(); }
    let all = build(g);
    let list = standing(g, &all);
    let serial = geom_serial(g);
    g.m.lightswitch.states = Some(St { all, list: list.clone(), hands: Vec::new(), serial });
    list
}

/// `all.filter(sw => wallStands(g, sw))`
fn standing(g: &mut Game, all: &[WallSwitch]) -> Vec<WallSwitch> {
    let mut list = Vec::new();
    for sw in all {
        if wall_stands(g, sw) { list.push(sw.clone()); }
    }
    list
}

/// The level's geometry serial, moved by every wall blown and by a load that puts one back. (lightswitch.ts geomSerial)
pub fn geom_serial(g: &mut crate::game::Game) -> f64 {
    g.engine.world.geom_serial
}

/// Is the wall a plate was hung on still behind it. (lightswitch.ts wallStands)
pub fn wall_stands(g: &mut crate::game::Game, sw: &crate::lightswitch::WallSwitch) -> bool {
    let (height, stand_m) = (g.k.lightswitch.height, g.k.lightswitch.stand_m);
    let h = wall_ray(g, [sw.stand[0], height, sw.stand[2]], [-sw.n[0], 0.0, -sw.n[2]], stand_m + 0.2);
    match h {
        Some(h) => h.index < 1e6 && m::abs(h.t - stand_m) < 0.03,
        None => false,
    }
}

/// A restart: the plates are found again on the level as dealt, and no fixture is a hand's. (lightswitch.ts lightSwitchReset)
pub fn light_switch_reset(g: &mut crate::game::Game) {
    g.m.lightswitch.states = None;
    let targets = g.targets.clone();
    for t in targets {
        if crate::sim_lightshome::fixture_hand_off(g, t).is_some() {
            let kind = crate::sim_lightshome::light_in().hand;
            crate::sim_lightshome::light_home_in(g, kind, Some(t), Some(m::NAN), None, None, None);
        }
    }
}

// ---------------------------------------------------------------- the flip
/// Are the room's lights on at the switch: any fixture of it switched on. (lightswitch.ts switchLit)
pub fn switch_lit(g: &mut crate::game::Game, sw: &crate::lightswitch::WallSwitch) -> bool {
    sw.targets.iter().any(|&t| crate::sim_lightshome::fixture_on(g, t))
}

/// May Sam be planted at a plate for his hand's move: not from a wall hold. (lightswitch.ts plants)
pub fn plants(g: &mut crate::game::Game, by: CharId) -> bool {
    let pl = g.player;
    by == g.pl[pl].ch && g.puppet.is_none() && g.pl[pl].wall.is_none()
}

/// The time his turn to the plate and his step in to it take. (lightswitch.ts turnSecs)
pub fn turn_secs(g: &mut crate::game::Game, by: CharId, sw: &crate::lightswitch::WallSwitch) -> f64 {
    let (yaw, pos) = (g.ch[by].body_yaw, g.ch[by].pos);
    let (stand_m, step_pace) = (g.k.handwork.stand_m, g.k.handwork.step_pace);
    let face = m::atan2(-sw.n[0], -sw.n[2]);
    let d = m::atan2(m::sin(face - yaw), m::cos(face - yaw));
    let step = m::max(0.0, m::hyp2(sw.pos[0] + sw.n[0] * stand_m - pos[0], sw.pos[2] + sw.n[2] * stand_m - pos[2]) - 0.15);
    m::min(g.k.handring.turn_max, m::abs(d) / crate::handwork::WORK_TURN + step / step_pace)
}

/// What breaks the plant off: he is down, has a man or a body in hand, is folded by a blast, or has been moved off the plate. (lightswitch.ts plantBroken)
pub fn plant_broken(g: &mut crate::game::Game, h: &crate::lightswitch::Hand) -> bool {
    let pl = g.player;
    let p = &g.pl[pl];
    if p.down || p.holding.is_some() || p.takedown.is_some() || p.dragging.is_some() || p.wall.is_some() { return true; }
    let c = p.ch;
    crate::samstun::sam_staggered(g, None) || v3::dist_xz(h.sw.pos, g.ch[c].pos) > 1.6
}

/// A click or any row given lets go of Sam's plant at the plate. (lightswitch.ts switchLetGo)
pub fn switch_let_go(g: &mut crate::game::Game) {
    let c = g.pl[g.player].ch;
    if let Some(st) = &mut g.m.lightswitch.states {
        for h in &mut st.hands {
            if h.c == c { h.plant = false; }
        }
    }
}

/// The plate Sam is planted at, while he is. (lightswitch.ts switchHand)
pub fn switch_hand(g: &mut crate::game::Game) -> Option<crate::lightswitch::SwitchHand> {
    let c = g.pl[g.player].ch;
    let h = g.m.lightswitch.states.as_ref()?.hands.iter().find(|x| x.plant && x.c == c)?.clone();
    if g.time >= h.at && g.time - h.at < h.dur && !plant_broken(g, &h) { Some(SwitchHand { p: h.sw.pos, n: h.sw.n }) } else { None }
}

/// Work a switch: off, each burning fixture seen to go and marked as a hand's; on, what the hand put out or the room as authored. (lightswitch.ts flipSwitch)
pub fn flip_switch(g: &mut crate::game::Game, sw: &crate::lightswitch::WallSwitch, on: bool, by: CharId) {
    if !on {
        for &t in &sw.targets {
            if crate::sim_lightshome::fixture_on(g, t) && crate::sim_lightshome::fixture_factor(g, t) > 0.0 {
                crate::game::game_light_event(g, t, Some(crate::game::GameLightEventCause::Shot), Some(sw.floor));
                g.stats.lightswitch.events += 1;
            }
        }
    }
    let time = g.time;
    crate::sim_lightshome::light_home_flip(g, &sw.targets, on, time, true);
    crate::audio_iface::game_audio_like_play(g, "click", Some(sw.pos), Some(0.8), None);
    if !on { crate::audio_iface::game_audio_like_play(g, "lightOff", Some(sw.pos), Some(0.5), None); }
    if g.m.lightswitch.states.is_some() {
        g.m.lightswitch.states.as_mut().unwrap().hands.retain(|h| h.c != by);
        let plant = plants(g, by);
        let dur = g.k.lightswitch.hand_secs + (if plant { turn_secs(g, by, sw) } else { 0.0 });
        let hand = Hand { c: by, sw: sw.clone(), at: time, dir: if on { -1.0 } else { 1.0 }, dur, plant };
        g.m.lightswitch.states.as_mut().unwrap().hands.push(hand);
    }
}

// ---------------------------------------------------------------- Sam's row and its deed
/// (lightswitch.ts handsFree)
pub fn hands_free(g: &mut crate::game::Game, pl: PlayerId) -> bool {
    let p = &g.pl[pl];
    p.dragging.is_none() && !p.sprinting && p.takedown.is_none() && p.holding.is_none()
}

/// The plate nearest `pl`'s feet within `r` m that he has a clear line to from his chest. (lightswitch.ts switchNear)
pub fn switch_near(g: &mut crate::game::Game, pl: PlayerId, r: f64) -> Option<crate::lightswitch::WallSwitch> {
    if !switches_live(g) { return None; }
    let p = g.ch[g.pl[pl].ch].pos;
    let eye: V3 = [p[0], if g.pl[pl].crouch { 0.9 } else { 1.3 }, p[2]];
    let mut best: Option<WallSwitch> = None;
    let mut bd = r;
    for sw in wall_switches(g) {
        let d = v3::dist_xz(sw.pos, p);
        if d < bd && !crate::scene_collision::solid_blocked(g, eye, v3::mad(sw.pos, sw.n, 0.05), None) { bd = d; best = Some(sw); }
    }
    best
}

/// The NEAREST plate only: a marker from NEARBY_M, the row in reach with the hands free. (lightswitch.ts switchRows)
pub fn switch_rows(g: &mut crate::game::Game, pl: PlayerId, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::lightswitch::SwitchRowsRowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) {
    let Some(sw) = switch_near(g, pl, crate::player::NEARBY_M) else { return };
    let near = v3::dist_xz(sw.pos, g.ch[g.pl[pl].ch].pos) < g.k.lightswitch.reach;
    let reach = near && hands_free(g, pl);
    let lit = switch_lit(g, &sw);
    let line2 = if reach { "" } else if !near { "step closer" } else if g.pl[pl].sprinting { "stop sprinting" } else { "hands full" };
    let it = crate::game::Interactable {
        kind: crate::game::InteractableKind::Switch,
        pos: sw.pos,
        line1: String::from(if lit { "switch · on" } else { "switch · off" }),
        line2: String::from(line2),
        in_reach: reach,
        off: !lit,
        progress: None,
        target: None,
        door: None,
        guard: None,
        verb: None,
        running: None,
        close: None,
        past: None,
        framed: None,
    };
    let rows = if reach && !busy {
        let (verb, text) = if lit { (crate::game::Verb::LightsOff, "lights off") } else { (crate::game::Verb::LightsOn, "lights on") };
        vec![row(g, it.clone(), verb, String::from(text), None, None)]
    } else {
        Vec::new()
    };
    offer(g, it, rows);
}

/// Space on that row: the plate in reach is worked the way the row said. False = none in reach now, or it already stands that way. (lightswitch.ts switchAct)
pub fn switch_act(g: &mut crate::game::Game, pl: PlayerId, verb: crate::game::Verb) -> bool {
    let sw = if hands_free(g, pl) {
        let reach = g.k.lightswitch.reach;
        switch_near(g, pl, reach)
    } else {
        None
    };
    let Some(sw) = sw else { return false };
    let on = verb == crate::game::Verb::LightsOn;
    if switch_lit(g, &sw) == on { return false; }
    let c = g.pl[pl].ch;
    flip_switch(g, &sw, on, c);
    g.stats.lightswitch.flips += 1;
    true
}

// ---------------------------------------------------------------- once a frame
/// Can this man put a room's lights back: a living defender on his own feet and his own business. (lightswitch.ts putsBack)
pub fn puts_back(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let time = g.time;
    let man = &g.gd[gd];
    let team = man.team;
    let own = (man.state == crate::game::GuardState::Patrol || man.state == crate::game::GuardState::Suspicious)
        && g.ch[man.ch].alive
        && man.held.is_none()
        && man.civ.is_none()
        && man.wound.is_none()
        && man.script.is_none()
        && man.body_duty.is_none()
        && time >= man.dazzled_until
        && time >= man.stagger_until;
    own && !crate::doze::doze_asleep(g, gd) && crate::teams::is_hostile(g, team, crate::teams::Team::Player)
}

/// Once a frame: a plate goes with its wall, a calm defender at the switch puts the room back, a hand stays at the plate a moment. (lightswitch.ts lightSwitchTick)
pub fn light_switch_tick(g: &mut crate::game::Game) {
    if !switches_live(g) { return; }
    let mut l = wall_switches(g);
    // a plate goes with its wall and is back with it: asked after only when the level's geometry has moved
    let serial = geom_serial(g);
    if g.m.lightswitch.states.as_ref().unwrap().serial != serial {
        let st = g.m.lightswitch.states.as_mut().unwrap();
        st.serial = serial;
        let n = st.list.len() as i32;
        let all = st.all.clone();
        l = standing(g, &all);
        g.stats.lightswitch.gone += (n - l.len() as i32).max(0);
        g.m.lightswitch.states.as_mut().unwrap().list = l.clone();
    }
    // a room the level had lit and a hand put out: a calm defender at its switch puts it back, a beat after the hand at the soonest
    if !crate::sim_lightshome::mains_active(g) && g.ai_enabled {
        for sw in &l {
            let mut since = -1.0;
            for &t in &sw.targets {
                if let Some(off) = crate::sim_lightshome::fixture_hand_off(g, t) {
                    if !crate::sim_lightshome::fixture_on(g, t) && crate::game::game_authored_on(g, t) { since = m::max(since, g.time - off); }
                }
            }
            if since < g.k.lightswitch.guard_secs { continue; }
            let reach = g.k.lightswitch.reach;
            let mut i = 0;
            while i < g.guards.len() {
                let gd = g.guards[i];
                i += 1;
                let c = g.gd[gd].ch;
                let p = g.ch[c].pos;
                if !puts_back(g, gd) || (v3::dist_xz(p, sw.pos) > reach && v3::dist_xz(p, sw.floor) > 0.6) { continue; }
                if crate::scene_collision::solid_blocked(g, [p[0], 1.3, p[2]], v3::mad(sw.pos, sw.n, 0.05), None) { continue; }
                flip_switch(g, sw, true, c);
                g.stats.lightswitch.guard_on += 1;
                crate::game::game_say(g, gd, "who turned these off?", Some(false), None, None);
                break;
            }
        }
    }
    // the hand at the plate for a moment after the flip: a plant broken off stays off
    let mut hands = core::mem::take(&mut g.m.lightswitch.states.as_mut().unwrap().hands);
    if hands.is_empty() { return; }
    let time = g.time;
    hands.retain(|h| time >= h.at && time - h.at < h.dur && g.ch[h.c].alive);
    let n = hands.len();
    g.m.lightswitch.states.as_mut().unwrap().hands = hands;
    for i in 0..n {
        let h = g.m.lightswitch.states.as_ref().unwrap().hands[i].clone();
        let mut plant = h.plant;
        if plant && plant_broken(g, &h) {
            g.m.lightswitch.states.as_mut().unwrap().hands[i].plant = false;
            plant = false;
        }
        let hand = if plant { crate::handwork::sam_hand(g) } else { None };
        let more = crate::handwork::WorkAtMore { keys: None, hand, dir: Some(h.dir) };
        let t = g.time - h.at;
        crate::handwork::work_at(g, h.c, crate::character::WorkKind::Lever, h.sw.pos, h.sw.n, t, h.dur, Some(&more));
    }
}

// ---------------------------------------------------------------- drawing (into Game.dynBoxes, beside the keypads' plates)
/// The plate, its rocker (top proud with the lights on) and, in a room off at the switch, the small amber pilot: a decal's depth, nobody's collision, no shadow. (lightswitch.ts switchBoxes)
pub fn switch_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {
    let flag = crate::scene_boxes::box_flag();
    let deco = (js::to_i32(flag.dynamic) | js::to_i32(flag.no_shadow)) as f64;
    for sw in wall_switches(g) {
        let yaw = v3::yaw_of(sw.n);
        let lit = switch_lit(g, &sw);
        let key = crate::scene_boxes::mkey().device + SW_KEY + sw.id * 4.0;
        let a = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP { c: v3::mad(sw.pos, sw.n, 0.006), h: [0.042, 0.06, 0.006], yaw: Some(yaw), albedo: Some(plate()), flags: Some(deco), mkey: Some(key), ..Default::default() });
        out.push(a);
        let b = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP { c: v3::add(v3::mad(sw.pos, sw.n, 0.015), [0.0, if lit { 0.012 } else { -0.012 }, 0.0]), h: [0.016, 0.018, 0.005], yaw: Some(yaw), albedo: Some(rocker()), flags: Some(deco), mkey: Some(key + 1.0), ..Default::default() });
        out.push(b);
        let c = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP { c: v3::add(v3::mad(sw.pos, sw.n, 0.013), [0.0, -0.045, 0.0]), h: [0.007, 0.004, 0.002], yaw: Some(yaw), albedo: Some([0.05, 0.03, 0.01]), emissive: Some(if lit { [0.0, 0.0, 0.0] } else { glow() }), flags: Some(deco), mkey: Some(key + 2.0), ..Default::default() });
        out.push(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plate(x: f64) -> Plate { Plate { door: DoorId(0), n: [0.0; 3], pos: [x, 0.0, 0.0], floor: [0.0; 3], room: 1.0, stand: [0.0; 3] } }
    fn xs(ps: &[Plate]) -> Vec<f64> { ps.iter().map(|p| p.pos[0]).collect() }

    #[test]
    fn fewest_keeps_the_first_then_the_furthest() {
        let ps = [plate(0.0), plate(5.0), plate(10.0)];
        assert_eq!(xs(&fewest_of(&ps, 2.0, 4.0)), vec![0.0, 10.0]);
        assert_eq!(xs(&fewest_of(&ps, 3.0, 4.0)), vec![0.0, 10.0, 5.0]);
    }

    #[test]
    fn fewest_a_tie_goes_to_the_later_plate() {
        assert_eq!(xs(&fewest_of(&[plate(0.0), plate(5.0), plate(-5.0)], 2.0, 4.0)), vec![0.0, -5.0]);
    }

    #[test]
    fn fewest_stops_when_none_is_far_enough_and_on_nothing() {
        assert_eq!(xs(&fewest_of(&[plate(0.0), plate(3.0)], 2.0, 4.0)), vec![0.0]);
        assert!(fewest_of(&[], 2.0, 4.0).is_empty());
    }

    #[test]
    fn fewest_tells_equal_plates_apart_by_place() {
        // two plates at one spot: the TypeScript's `includes` is by identity, so the second is a candidate (and too near to keep)
        assert_eq!(xs(&fewest_of(&[plate(0.0), plate(0.0)], 2.0, 0.0)), vec![0.0, 0.0]);
    }
}
