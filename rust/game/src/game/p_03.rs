//! src/game/game.ts lines 588 to 962
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Stamp fix_t if the fix has moved since last looked at; called once a frame by update_guard. (game.ts Guard.trackFix)
pub fn guard_track_fix(g: &mut crate::game::Game, id: GuardId, now: f64) {
    let lk = match g.gd[id].last_known {
        Some(lk) => lk,
        None => { g.gd[id].fix_ref = None; return; }
    };
    let moved = match g.gd[id].fix_ref {
        None => true,
        Some(r) => m::hyp2(lk[0] - r[0], lk[2] - r[2]) > 0.5,
    };
    if moved { g.gd[id].fix_ref = Some(v3::copy(lk)); g.gd[id].fix_t = now; }
}

/// A man: his record, his body facing down the first leg, his keys, his gun. Field initialisers first (the look phase's draw comes BEFORE the body's own). (game.ts Guard.constructor)
pub fn guard_new(g: &mut crate::game::Game, men: crate::Opaque, ch: RigId, id_2: f64, route: RouteId, route_i: f64, flashlight: LightId, tint: V3, look: Option<&crate::game::GuardNewLook>) -> GuardId {
    let id = g.gd.push(Guard::blank());
    g.gd[id].witness = new_witness();
    g.gd[id].talk = new_talk(g);
    g.gd[id].look_phase = crate::math_dice::sim_random(g) * 10.0;

    let (p0, p1, weapon, keys) = {
        let r = &g.tab.route[route];
        (r.points[0], r.points[1 % r.points.len()], r.weapon, r.keys.clone())
    };
    let pos = v3::copy(p0);
    let desc = crate::character::CharacterDesc {
        id: id_2,
        tint,
        tint2: look.and_then(|l| l.tint2).unwrap_or([0.02, 0.02, 0.025]),
        pos,
        yaw: match look.and_then(|l| l.yaw) { Some(y) => y, None => v3::yaw_to(pos, p1) },
        men: Some(men),
    };
    let c = crate::character::character_new(g, ch, desc);
    g.gd[id].ch = c;
    g.ch[c].who = crate::character::Who::Guard(id);
    seam_same_man(g, id, c);

    g.gd[id].last_pos = v3::copy(pos);
    g.gd[id].route_i = if route_i < 0.0 { seam_route_i_negative(route_i) } else { route_i as usize };
    g.gd[id].route = route;
    g.gd[id].flashlight = flashlight;
    g.gd[id].callsign = match look.and_then(|l| l.name.clone()) {
        Some(name) => name,
        None => {
            let cs = callsigns();
            cs[js::rem(m::max(0.0, id_2 - 10.0), cs.len() as f64) as usize].clone()
        }
    };
    if let Some(team) = look.and_then(|l| l.team) { g.gd[id].team = team; }
    let team = g.gd[id].team;
    if team != crate::teams::Team::Guard && team != crate::teams::Team::Hostile { g.gd[id].foe = None; }
    let keys = match keys {
        Some(k) => k,
        None => vec![String::from(crate::doors::FLOOR_KEY)],
    };
    let mut held: JsSet<String> = JsSet::new();
    for k in keys { held.add(k); }
    g.gd[id].keys = held;
    // a row the table does not hold panics here, as the TypeScript's throws
    let row = g.m.weapons.weapons.get(weapon.unwrap_or(crate::weapons::WeaponId::Guard9mm)).unwrap().clone();
    let w = crate::weapons::firearm_new(g, &row);
    g.gd[id].weapon = w;
    g.ch[c].muzzle_ahead = row.muzzle_ahead;
    id
}

/// The snow's field and what the wind and the dust hold are handed to the GPU whole again next frame. Host's call (putback.ts). (game.ts Game.rehand)
pub fn game_rehand(g: &mut crate::game::Game) {
    g.snow_look_was = -1.0;
    crate::air_sim::air_resend(g);
    crate::air_dust::air_dust_resend(g);
}

/// PICTURE ONLY, not translated: the snow's field and prints follow their switches for the g-buffer. Empty, as nothing in a rule reads them and the game's constructor calls this unguarded (game.ts:1221). (game.ts Game.syncSnowLook)
pub fn game_sync_snow_look(g: &mut crate::game::Game) {}

/// PICTURE ONLY, not translated: this frame's footfalls pressed into the snow for the g-buffer. Empty, as nothing in a rule reads them. (game.ts Game.stepSnowTracks)
pub fn game_step_snow_tracks(g: &mut crate::game::Game, dt: f64) {}

/// Asks the renderer's irradiance at a point under `key`. (game.ts Game.lightQuery; the tools' swap of this field is not carried)
pub fn game_light_query(g: &mut crate::game::Game, key: &str, pos: V3, skip_owner: Option<f64>) {
    let skip_owner = skip_owner.unwrap_or(0.0);
    crate::sensors::sensors_ask(g, key, pos, Some(skip_owner));
}

/// The latest answer for `key`, or None while there is none. (game.ts Game.lightAt)
pub fn game_light_at(g: &mut crate::game::Game, key: &str) -> Option<f64> {
    crate::sensors::sensors_read(g, key)
}

/// Drops a stale answer for `key`. (game.ts Game.lightForget)
pub fn game_light_forget(g: &mut crate::game::Game, key: &str) {
    crate::sensors::sensors_forget(g, key);
}

/// Which side each door opens into, as the deals stand now. (game.ts Game.doorSwings)
pub fn game_door_swings(g: &mut crate::game::Game) -> Vec<crate::doorswing::SwingDeal> {
    let deals = g.door_deals.clone();
    crate::doorswing::swings_now(g, &deals)
}

/// A wall changed inside `rect` (None = a reset): noted for later under the blast's pace, else read at once. Returns how many lights were queued. (game.ts Game.reachChanged)
pub fn game_reach_changed(g: &mut crate::game::Game, rect: Option<[f64; 4]>) -> f64 {
    if g.reach_solid.is_none() { return 0.0; }
    if let Some(r) = rect {
        if g.m.destruction_switch.blast_pace.on && !g.m.render_reach.reach_pace.whole {
            if g.reach_held.is_empty() { g.reach_hold = g.m.destruction_switch.blast_pace.reach_hold; }
            g.reach_held.push(r);
            return 0.0;
        }
        let one = [r];
        return game_reach_read(g, Some(&one));
    }
    game_reach_read(g, None)
}

/// Builds the wall raster again and sets it against the last; queues the lights in reach of `rects` (None = a reset, the booted masks back). (game.ts Game.reachRead)
pub fn game_reach_read(g: &mut crate::game::Game, rects: Option<&[[f64; 4]]>) -> f64 {
    if g.reach_solid.is_none() { return 0.0; }
    if rects.is_none() { g.reach_held.clear(); }
    let statics = g.engine.world.statics.clone();
    let doors: Vec<crate::render_reach::ReachDoor> = g.level.doors.iter()
        .map(|d| crate::render_reach::ReachDoor { hinge: d.hinge, closed_dir: d.closed_dir, width: d.width })
        .collect();
    let solid = crate::render_reach::build_wall_raster(g, &statics, Some(&doors));
    let prev = g.reach_solid.clone().unwrap();
    let bx = match crate::render_reach::wall_raster_diff(g, &prev, &solid) {
        Some(b) => b,
        None => return 0.0,
    };
    g.reach_solid = Some(solid);
    let mut n = 0.0;
    let rects = match rects {
        Some(r) => r,
        None => {
            if g.reach_solid_boot.is_some() && g.reach_solid_boot == g.reach_solid { g.reach_solid = g.reach_solid_boot.clone(); }
            g.reach_queue.clear();
            for l in g.reach_boot.keys() {
                let mask = g.reach_boot.get(l).unwrap().clone();
                if seam_light_kind(g, l) == 2.0 { seam_light_set_reach_dir(g, l, mask); } else { seam_light_set_reach(g, l, mask); }
            }
            g.reach_boot.clear();
            return g.reach_lights.len() as f64;
        }
    };
    // `box` in the TypeScript is one array this call, and q.boxes.includes(box) asks by identity: "has this call's box gone on this light yet".
    let mut boxed: Vec<LightId> = Vec::new();
    for &r in rects {
        for l in g.reach_lights.clone() {
            let pos = seam_light_pos(g, l);
            let dx = m::max_n(&[r[0] - pos[0], 0.0, pos[0] - r[2]]);
            let dz = m::max_n(&[r[1] - pos[2], 0.0, pos[2] - r[3]]);
            if m::hyp2(dx, dz) > seam_light_range(g, l) + 1.0 { continue; }
            reach_queue_light(g, l, Some(r), bx, &mut n, &mut boxed);
        }
        for l in g.engine.lights.lights.clone() {
            if seam_light_kind(g, l) == 2.0 && seam_light_reach_dir(g, l).is_some() { reach_queue_light(g, l, Some(r), bx, &mut n, &mut boxed); }
        }
    }
    n
}

/// reachRead's `queue` closure: `l` queued with `r`, or `r` and this call's changed box added to its entry. (game.ts Game.reachRead)
fn reach_queue_light(g: &mut Game, l: LightId, r: Option<[f64; 4]>, bx: [f64; 4], n: &mut f64, boxed: &mut Vec<LightId>) {
    let q = match g.reach_queue.iter_mut().find(|x| x.l == l) {
        Some(q) => q,
        None => {
            g.reach_queue.push(GameReachQueueItem { l, rects: r.map(|r| vec![r]), boxes: vec![bx], job: None, restarts: 0.0 });
            boxed.push(l);
            *n += 1.0;
            return;
        }
    };
    if q.job.as_ref().map_or(false, |j| j.st.is_some()) { q.job = None; q.restarts += 1.0; }
    match r {
        None => q.rects = None,
        Some(r) => {
            if let Some(rects) = &mut q.rects {
                rects.push(r);
                if !boxed.contains(&l) { q.boxes.push(bx); boxed.push(l); }
            }
        }
    }
}

/// reachTick's `boot` closure: what the level as authored gave this light, kept once. (game.ts Game.reachTick)
fn reach_boot_note(g: &mut Game, l: LightId) {
    if g.reach_boot.has(l) { return; }
    let mask = if seam_light_kind(g, l) == 2.0 { seam_light_reach_dir(g, l) } else { seam_light_reach(g, l) };
    if let Some(mask) = mask { g.reach_boot.set(l, mask); }
}

/// Re-derives queued reach masks in the queue's order, `steps` sight-line steps a frame and at most `n` lights finished. Returns the lights finished. (game.ts Game.reachTick)
pub fn game_reach_tick(g: &mut crate::game::Game, n: Option<f64>, steps: Option<f64>) -> f64 {
    let n = n.unwrap_or(8.0);
    let steps = steps.unwrap_or(g.m.render_reach.reach_pace.steps);
    if !g.reach_held.is_empty() {
        let hold = g.reach_hold;
        g.reach_hold = hold - 1.0;
        if hold <= 0.0 {
            let held = core::mem::take(&mut g.reach_held);
            game_reach_read(g, Some(&held));
            return 0.0;
        }
    }
    let mut done = 0.0;
    let (c0, k0) = (g.m.render_reach.reach_work.steps, g.m.render_reach.reach_work.cells);
    let whole = g.m.render_reach.reach_pace.whole;
    while done < n && !g.reach_queue.is_empty() && g.reach_solid.is_some() {
        let left = if whole { f64::INFINITY } else { steps - (g.m.render_reach.reach_work.steps - c0) };
        if left <= 0.0 || (whole && done > 0.0 && g.m.render_reach.reach_work.cells - k0 >= g.m.render_reach.reach_pace.whole_cells) { break; }
        let l = g.reach_queue[0].l;
        let kind = seam_light_kind(g, l);
        if kind == 2.0 {
            if !whole && g.m.render_reach.reach_work.steps != c0 { break; }
            reach_boot_note(g, l);
            let prev = seam_light_reach_dir(g, l);
            let only = match (g.reach_queue[0].rects.clone(), prev) {
                (Some(rects), Some(prev)) => Some(crate::render_reach::ReachOnly { prev, rects, max_dist: None, boxes: None }),
                _ => None,
            };
            game_build_dir_reach(g, l, only.as_ref());
            g.reach_queue.remove(0);
            done += 1.0;
            if whole { continue; }
            break;
        }
        let mut job = match g.reach_queue[0].job.take() {
            Some(job) => job,
            None => crate::render_reach::ReachJob { budget: left, own: None, st: None },
        };
        job.budget = if g.reach_queue[0].restarts >= 2.0 { f64::INFINITY } else { left };
        let pos = seam_light_pos(g, l);
        let dir = if kind == 3.0 || kind == 1.0 { seam_light_dir(g, l) } else { [0.0, 0.0, 0.0] };
        let radius = seam_light_radius(g, l);
        let prev = seam_light_reach(g, l);
        let only = match (g.reach_queue[0].rects.clone(), prev) {
            (Some(rects), Some(prev)) => {
                let max_dist = seam_light_range(g, l) + 1.5;
                Some(crate::render_reach::ReachOnly { prev, rects, max_dist: Some(max_dist), boxes: Some(g.reach_queue[0].boxes.clone()) })
            }
            _ => None,
        };
        let solid = g.reach_solid.clone().unwrap();
        let interior = g.reach_interior.clone();
        let mask = crate::render_reach::reach_mask_some(g, &solid, pos, dir, radius, interior.as_deref(), only.as_ref(), &mut job);
        g.reach_queue[0].job = Some(job);
        let mask = match mask {
            Some(mask) => mask,
            None => break,
        };
        reach_boot_note(g, l);
        seam_light_set_reach(g, l, mask);
        g.reach_queue.remove(0);
        done += 1.0;
    }
    done
}

/// How many lights wait, and how many wall changes are noted and unread. (game.ts Game.reachPending)
pub fn game_reach_pending(g: &mut crate::game::Game) -> f64 {
    (g.reach_queue.len() + g.reach_held.len()) as f64
}

fn seam_same_man(g: &mut Game, gd: GuardId, c: CharId) { todo!("needs: what sameMan becomes (game.ts:756): ManCols is empty and neither Guard nor Character holds one, so same_man(&mut ManCols, ManCols) has nothing to be given; Guard holds owner, rank, reason and since as plain fields") }
fn seam_route_i_negative(route_i: f64) -> usize { todo!("needs: Guard.route_i to hold -1 (game.ts:602, 757; needs.ts:146, opening.ts:81 test < 0): it is usize") }
fn seam_light_kind(g: &mut Game, l: LightId) -> f64 { g.tab.light[l].kind } // SKELETON-GAP was: RtLight.kind by LightId; skeleton 4 has the row (g.tab.light)
fn seam_light_pos(g: &mut Game, l: LightId) -> V3 { g.tab.light[l].pos } // SKELETON-GAP was: RtLight.pos by LightId; skeleton 4 has the row (g.tab.light)
fn seam_light_dir(g: &mut Game, l: LightId) -> V3 { g.tab.light[l].dir } // SKELETON-GAP was: RtLight.dir by LightId; skeleton 4 has the row (g.tab.light)
fn seam_light_range(g: &mut Game, l: LightId) -> f64 { g.tab.light[l].range } // SKELETON-GAP was: RtLight.range by LightId; skeleton 4 has the row (g.tab.light)
fn seam_light_radius(g: &mut Game, l: LightId) -> f64 { g.tab.light[l].radius } // SKELETON-GAP was: RtLight.radius by LightId; skeleton 4 has the row (g.tab.light)
fn seam_light_reach(g: &mut Game, l: LightId) -> Option<Vec<u32>> { g.tab.light[l].reach.clone() } // SKELETON-GAP was: RtLight.reach by LightId; skeleton 4 has the row (g.tab.light)
fn seam_light_reach_dir(g: &mut Game, l: LightId) -> Option<Vec<u32>> { g.tab.light[l].reach_dir.clone() } // SKELETON-GAP was: RtLight.reachDir by LightId; skeleton 4 has the row (g.tab.light)
fn seam_light_set_reach(g: &mut Game, l: LightId, mask: Vec<u32>) { g.tab.light[l].reach = Some(mask); } // SKELETON-GAP was: RtLight.reach by LightId; skeleton 4 has the row (g.tab.light)
fn seam_light_set_reach_dir(g: &mut Game, l: LightId, mask: Vec<u32>) { g.tab.light[l].reach_dir = Some(mask); } // SKELETON-GAP was: RtLight.reachDir by LightId; skeleton 4 has the row (g.tab.light)
