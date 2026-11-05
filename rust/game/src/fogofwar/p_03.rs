//! src/game/fogofwar.ts lines 796 to 1167
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A box's row. The index still says `g.none[id]`; `g.col.boxes` is the one `Tab<BoxId, Box>`, as scene_boxes and scene_collision read it: see the notes' Skeleton.
fn bx<'g>(g: &'g mut Game, b: BoxId) -> &'g mut crate::scene_boxes::Box {
    crate::scene_boxes::b_01::any_box_mut(g, b)   // TEMP-BRINGUP: a made box lives in the bring-up's arena (scene_boxes/b_01.rs)
}

/// Has the team a look at this fixture: an eye on the floor under it, a line to the fixture itself, or seen floor within two metres that it lights.   (fogofwar.ts FogOfWar.lampSeen)
pub fn fog_of_war_lamp_seen(g: &mut crate::game::Game, t: FixtureId) -> bool {
    let pos = g.tab.sim_lightshome_fixture[t].pos;
    let (x, y, z) = (pos[0], pos[1], pos[2]);
    if fog_of_war_spot_seen(g, x, z) { return true; }
    let mut n = 0;
    while n < g.fog_of_war.eyes.len() {
        let e = g.fog_of_war.eyes[n];
        n += 1;
        if !fog_of_war_in_field(g, e, pos) { continue; }
        let dx = x - e.at[0];
        let dy = y - e.at[1];
        let dz = z - e.at[2];
        let d = m::hyp3(dx, dy, dz);
        if d < 0.5 { return true; }
        let k = 1.0 - 0.3 / d;
        if !fog_of_war_sight_blocked(g, e.at, [e.at[0] + dx * k, e.at[1] + dy * k, e.at[2] + dz * k]) { return true; }
    }
    let from: V3 = [x, m::max(0.3, y - 0.3), z];
    let mut i = 0;
    while i < 8 {
        let a = i as f64 * m::PI / 4.0;
        let r = if i & 1 != 0 { 2.0 } else { 1.0 };
        let px = x + m::cos(a) * r;
        let pz = z + m::sin(a) * r;
        if !crate::scene_collision::segment_blocked(g, from, [px, 0.3, pz], None, None, None) && fog_of_war_spot_seen(g, px, pz) { return true; }
        i += 1;
    }
    false
}

/// Takes every class back off: the fog went off (update's off branch). The lights' own `fog` / `fogGhost` rows are the picture's and are not kept here.   (fogofwar.ts FogOfWar.fullClear)
pub fn fog_of_war_full_clear(g: &mut crate::game::Game) {
    if !g.fog_of_war.full_on { return; }
    g.engine.lights.fog_team.on = false;
    g.engine.lights.fog_team.owners.clear();
    g.engine.lights.fog_team.lights.clear();
    fog_of_war_faces_lift(g);
    g.fog_of_war.face_recs.clear();
    g.fog_of_war.full_on = false;
    g.fog_of_war.full_stats.hidden = 0.0;
    g.fog_of_war.full_stats.ghosts = 0.0;
    g.fog_of_war.full_stats.held = 0.0;
}

/// The picture's face of a lamp: its boxes drawn lit or dark through `fog_emit` / `fog_mark`, which the upload alone reads; `lit` None = as it stands.   (fogofwar.ts FogOfWar.lampFace)
pub fn fog_of_war_lamp_face(g: &mut crate::game::Game, t: FixtureId, lit: Option<bool>) {
    let rec = if lit == Some(true) { g.fog_of_war.face_recs.get(t).cloned() } else { None };
    let boxes = g.tab.sim_lightshome_fixture[t].boxes.clone();
    let base = g.tab.sim_lightshome_fixture[t].base_emissive;
    let flag = crate::scene_boxes::box_flag();
    let shows = js::to_i32(flag.area_lit) | js::to_i32(flag.no_shadow);
    let mut i = 0;
    while i < boxes.len() {
        let at = i;
        i += 1;
        let b = match g.engine.world.statics.get(boxes[at] as usize) { Some(&b) => b, None => continue };
        let q = bx(g, b);
        let on = match lit {
            Some(on) => on,
            None => {
                if q.fog_emit.is_some() { q.fog_emit = None; q.fog_mark = None; }
                continue;
            }
        };
        if js::to_i32(q.flags) & shows == 0 { continue; }
        let kept = rec.as_ref().and_then(|r| r.get(at)).copied();
        q.fog_emit = Some(if on {
            match kept { Some(k) if k[0] + k[1] + k[2] > 0.0 => k, _ => base }
        } else {
            dark()
        });
        q.fog_mark = Some(if on { 1.0 } else { 0.0 });
    }
}

/// A lamp the team knows lit: its face as it glows now is kept, the brightest of the last moments.   (fogofwar.ts FogOfWar.faceSeen)
pub fn fog_of_war_face_seen(g: &mut crate::game::Game, t: FixtureId) {
    let boxes = g.tab.sim_lightshome_fixture[t].boxes.clone();
    if boxes.is_empty() { return; }
    let n = boxes.len();
    g.fog_of_war.face_recs.get_or_insert_with(t, || vec![[0.0; 3]; n]);
    let mut i = 0;
    while i < n {
        let at = i;
        i += 1;
        let b = match g.engine.world.statics.get(boxes[at] as usize) { Some(&b) => b, None => continue };
        let e = bx(g, b).emissive;
        let k = &mut g.fog_of_war.face_recs.get_mut(t).unwrap()[at];
        k[0] = m::max(k[0] * FACE_HOLD, e[0]);
        k[1] = m::max(k[1] * FACE_HOLD, e[1]);
        k[2] = m::max(k[2] * FACE_HOLD, e[2]);
    }
}

/// Every kept face taken off.   (fogofwar.ts FogOfWar.facesLift)
pub fn fog_of_war_faces_lift(g: &mut crate::game::Game) {
    let mut i = 0;
    while i < g.targets.len() {
        let t = g.targets[i];
        i += 1;
        fog_of_war_lamp_face(g, t, None);
    }
}

/// The faces put back as the memory has them, without asking who sees what.   (fogofwar.ts FogOfWar.facesBack)
pub fn fog_of_war_faces_back(g: &mut crate::game::Game) {
    if !g.fog_of_war.full_on { return; }
    let down = crate::sim_lightshome::mains_active(g);
    let mut i = 0;
    while i < g.targets.len() {
        let t = g.targets[i];
        i += 1;
        let row = &g.tab.sim_lightshome_fixture[t];
        let (has_light, mains) = (row.light.is_some() || row.area_lights.is_some(), row.mains);
        if !has_light { continue; }
        let known = g.fog_of_war.lamp_recs.get(t).copied();
        let lit = fog_of_war_lamp_lit(g, t);
        if let Some(known) = known {
            if known != lit && (lit || !(mains && down)) { fog_of_war_lamp_face(g, t, Some(!lit)); }
        }
    }
}

/// NOT TRANSLATED: a checkpoint is a clone of Game (conventions 6).   (fogofwar.ts FogOfWar.snapshot)
pub fn fog_of_war_snapshot(g: &mut crate::game::Game) -> Option<crate::fogofwar::FogSnap> {
    todo!("not translated: a checkpoint is a clone of Game (conventions 6)")
}

/// NOT TRANSLATED: a checkpoint is a clone of Game (conventions 6).   (fogofwar.ts FogOfWar.restore)
pub fn fog_of_war_restore(g: &mut crate::game::Game, s: Option<&crate::fogofwar::FogSnap>) {
    todo!("not translated: a checkpoint is a clone of Game (conventions 6)")
}

/// Is the fixture lit, as it stands.   (fogofwar.ts FogOfWar.lampLit)
pub fn fog_of_war_lamp_lit(g: &mut crate::game::Game, t: FixtureId) -> bool {
    let on = crate::sim_lightshome::fixture_on(g, t);
    let broken = crate::sim_lightshome::fixture_broken(g, t);
    let until = crate::sim_lightshome::fixture_disabled_until(g, t);
    on && !broken && !(until > 0.0 && g.time < until)
}

/// The fixture as the team last saw it: first met = as it stands; it comes up to date the frame it differs and the team has a look at it.   (fogofwar.ts FogOfWar.lampKnown)
pub fn fog_of_war_lamp_known(g: &mut crate::game::Game, t: FixtureId, lit: bool) -> bool {
    let rec = g.fog_of_war.lamp_recs.get(t).copied();
    let stale = match rec { None => true, Some(r) => r != lit && fog_of_war_lamp_seen(g, t) };
    if stale {
        g.fog_of_war.lamp_recs.set(t, lit);
        return lit;
    }
    rec.unwrap()
}

/// THE TEAM FOR THE LIGHTS' CLASSES, which `ours_id` and `things` go by, and the rule's flag. The lamps' half is the picture's.   (fogofwar.ts FogOfWar.fullBlock)
pub fn fog_of_war_full_block(g: &mut crate::game::Game, f: &mut crate::render_fogofwar::FogFrame) -> bool {
    g.fog_of_war.full_stats.hidden = 0.0;
    g.fog_of_war.full_stats.ghosts = 0.0;
    g.fog_of_war.full_stats.held = 0.0;
    g.fog_of_war.full_on = true;
    f.full_flat = m::max(0.0, g.k.fog.flat * g.k.fog.full_fill);
    g.engine.lights.fog_team.on = true;
    g.engine.lights.fog_team.owners.clear();
    g.engine.lights.fog_team.lights.clear();
    let of = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
    for pl in crate::seats::men(g, &of) {
        let id = g.ch[g.pl[pl].ch].id;
        let light = g.pl[pl].light;
        g.engine.lights.fog_team.owners.add(crate::Num(id));
        g.engine.lights.fog_team.lights.add(light);
    }
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        if fog_of_war_on_team(g, gd) {
            let id = g.ch[g.gd[gd].ch].id;
            let light = g.gd[gd].flashlight;
            g.engine.lights.fog_team.owners.add(crate::Num(id));
            g.engine.lights.fog_team.lights.add(light);
        }
        i += 1;
    }
    true
}

/// NOT TRANSLATED: the picture's lamps; it reads an RtLight's rows, which have no home in the skeleton.   (fogofwar.ts FogOfWar.lampTake)
pub fn fog_of_war_lamp_take(g: &mut crate::game::Game, l: LightId, i: f64, far: bool) {
    todo!("not translated: picture only, and needs an RtLight's rows on a LightId")
}

/// NOT TRANSLATED: the lit fog's rows of the renderer's block, read by nothing in the game; it reads an RtLight's rows, which have no home in the skeleton.   (fogofwar.ts FogOfWar.litBlock)
pub fn fog_of_war_lit_block(g: &mut crate::game::Game, f: &mut crate::render_fogofwar::FogFrame) {
    todo!("not translated: picture only, and needs an RtLight's rows on a LightId")
}

/// Is the floor at (x, z) in the lit part of the picture: some eye has it.   (fogofwar.ts FogOfWar.spotSeen)
pub fn fog_of_war_spot_seen(g: &mut crate::game::Game, x: f64, z: f64) -> bool {
    g.fog_of_war.thing_stats.tests += 1.0;
    fog_of_war_ground_look(g, x, z) == 2.0
}

/// `spot_seen`'s walk: 2 = an eye has the spot, 1 = in an eye's field and every such eye's line stopped, 0 = in no eye's field.   (fogofwar.ts FogOfWar.groundLook)
pub fn fog_of_war_ground_look(g: &mut crate::game::Game, x: f64, z: f64) -> f64 {
    let mut r = 0.0;
    let mut i = 0;
    while i < g.fog_of_war.eyes.len() {
        let e = g.fog_of_war.eyes[i];
        i += 1;
        let dx = x - e.at[0];
        let dz = z - e.at[2];
        let d = m::hyp2(dx, dz);
        if d < 0.4 { return 2.0; }
        let y = e.at[1];
        if !fog_of_war_in_field(g, e, [x, y, z]) { continue; }
        r = 1.0;
        let k = 1.0 - 0.12 / d;
        if !fog_of_war_sight_blocked(g, e.at, [e.at[0] + dx * k, y, e.at[2] + dz * k]) { return 2.0; }
    }
    r
}

/// A one-frame box's spot, asked once per half-metre square a frame.   (fogofwar.ts FogOfWar.cellSeenAt)
pub fn fog_of_war_cell_seen_at(g: &mut crate::game::Game, x: f64, z: f64) -> bool {
    let k = (m::floor(x * 2.0) + 4096.0) * 16384.0 + m::floor(z * 2.0) + 4096.0;
    if let Some(&v) = g.fog_of_war.cell_seen.get(crate::Num(k)) { return v; }
    let v = fog_of_war_spot_seen(g, x, z);
    g.fog_of_war.cell_seen.set(crate::Num(k), v);
    v
}

/// A thing is seen where its middle is, or a long one at either end of its long side.   (fogofwar.ts FogOfWar.thingSeen)
pub fn fog_of_war_thing_seen(g: &mut crate::game::Game, x: f64, z: f64, ox: f64, oz: f64) -> bool {
    fog_of_war_spot_seen(g, x, z)
        || ((ox != 0.0 || oz != 0.0) && (fog_of_war_spot_seen(g, x + ox, z + oz) || fog_of_war_spot_seen(g, x - ox, z - oz)))
}

/// The long side of boxes i‥j of a list into `span_x` / `span_z`: its biggest box's long axis in plan, 15 cm in from the ends; 0, 0 under 0.9 m long.   (fogofwar.ts FogOfWar.span)
pub fn fog_of_war_span(g: &mut crate::game::Game, list: &[BoxId], i: f64, j: f64) {
    let mut best = 0.45;
    g.fog_of_war.span_x = 0.0;
    g.fog_of_war.span_z = 0.0;
    let mut k = i as usize;
    while k < j as usize {
        let q = bx(g, list[k]);
        let (h, yaw) = (q.h, q.yaw);
        k += 1;
        let along_x = h[0] >= h[2];
        let l = if along_x { h[0] } else { h[2] };
        if l <= best { continue; }
        best = l;
        let in_ = l - 0.15;
        let cs = m::cos(yaw) * in_;
        let sn = m::sin(yaw) * in_;
        g.fog_of_war.span_x = if along_x { cs } else { sn };
        g.fog_of_war.span_z = if along_x { -sn } else { cs };
    }
}

/// Does the team see this thing (its boxes as a list)?   (fogofwar.ts FogOfWar.seesThing)
pub fn fog_of_war_sees_thing(g: &mut crate::game::Game, boxes: &[BoxId]) -> bool {
    let mut x = 0.0;
    let mut z = 0.0;
    for &b in boxes {
        let c = bx(g, b).c;
        x += c[0];
        z += c[2];
    }
    x /= boxes.len() as f64;
    z /= boxes.len() as f64;
    fog_of_war_span(g, boxes, 0.0, boxes.len() as f64);
    let (ox, oz) = (g.fog_of_war.span_x, g.fog_of_war.span_z);
    fog_of_war_thing_seen(g, x, z, ox, oz)
}

/// `take` of `things`: the picture has boxes i‥j of the list as they stand.   (fogofwar.ts FogOfWar.things `take`)
fn thing_take(g: &mut Game, tk: f64, list: &[BoxId], i: usize, j: usize, (x, z, ox, oz): (f64, f64, f64, f64), f: f64, moving: bool) {
    if !g.fog_of_war.thing_recs.has(crate::Num(tk)) {
        g.fog_of_war.thing_recs.set(crate::Num(tk), ThingRec { ghost: Vec::new(), x, z, ox, oz, f, moving, held: false });
    }
    let mut ghost = Vec::with_capacity(j - i);
    let mut k = i;
    while k < j {
        ghost.push(ghost_box(g, list[k]));
        k += 1;
    }
    let r = g.fog_of_war.thing_recs.get_mut(crate::Num(tk)).unwrap();
    r.ghost = ghost;
    r.x = x;
    r.z = z;
    r.ox = ox;
    r.oz = oz;
    r.moving = moving;
    r.held = false;
}

/// Do boxes i‥j stand where the record's copies do?   (fogofwar.ts FogOfWar.things `same`)
fn thing_same(g: &mut Game, list: &[BoxId], i: usize, j: usize, ghost: &[BoxId]) -> bool {
    if ghost.len() != j - i { return false; }
    let mut k = i;
    while k < j {
        let (sc, sy, sr) = { let s = bx(g, list[k]); (s.c, s.yaw, s.rot) };
        let (qc, qy, qr) = { let q = bx(g, ghost[k - i]); (q.c, q.yaw, q.rot) };
        k += 1;
        if !(m::abs(sc[0] - qc[0]) + m::abs(sc[1] - qc[1]) + m::abs(sc[2] - qc[2]) < THING_EPS && m::abs(sy - qy) < THING_EPS) { return false; }
        if let (Some(a), Some(b)) = (sr, qr) {
            if !(m::abs(a[0] - b[0]) + m::abs(a[1] - b[1]) + m::abs(a[2] - b[2]) + m::abs(a[3] - b[3]) < THING_EPS) { return false; }
        }
    }
    true
}

/// The team's own rubble on its first way down, for `things`: the frame's keys into `our_chunks`, the ones at rest into `our_landed`.   (fogofwar.ts FogOfWar.things, FOG.ours)
fn things_ours(g: &mut Game, ours: bool, prime: bool) {
    if g.fog_of_war.our_chunks.len() > 0 { g.fog_of_war.our_chunks.clear(); }
    let list = if ours && !prime { g.destr.debris.list.clone() } else { Vec::new() };
    if list.is_empty() {
        if g.fog_of_war.our_landed.len() > 0 { g.fog_of_war.our_landed.clear(); }
        return;
    }
    for c in list {
        let row = &g.tab.debris_chunk[c];
        let (foot, who, the_box, rest, mate) = (row.foot == Some(true), row.who, row.box_, row.rest, row.mate);
        if foot || !g.engine.lights.fog_team.owners.has(crate::Num(who)) { continue; }
        let k = thing_key_of(g, the_box);
        if g.fog_of_war.our_landed.has(crate::Num(k)) { continue; }
        if rest {
            if let Some(r) = g.fog_of_war.thing_recs.get(crate::Num(k)) {
                if !r.moving {
                    g.fog_of_war.our_landed.add(crate::Num(k));
                    continue;
                }
            }
        }
        g.fog_of_war.our_chunks.add(crate::Num(k));
        if let Some(mate) = mate {
            if bx(g, mate).mkey.is_some() {
                let mk = thing_key_of(g, mate);
                g.fog_of_war.our_chunks.add(crate::Num(mk));
            }
        }
    }
}

/// The picture's list of loose things, rewritten thing by thing: live where the team sees the floor it is on; out of sight, the copy where it was last seen at rest, or nothing. Hands back the list as it now stands.   (fogofwar.ts FogOfWar.things)
pub fn fog_of_war_things(g: &mut crate::game::Game, list: &mut Vec<BoxId>) -> Vec<BoxId> {
    g.fog_of_war.thing_stats.held = 0.0;
    g.fog_of_war.thing_stats.out = 0.0;
    g.fog_of_war.thing_stats.tests = 0.0;
    if g.fog_of_war.viewer && fog_of_war_active(g) {
        let mut raw = core::mem::take(&mut g.fog_of_war.raw_list);
        raw.clear();
        let mut i = 0;
        if js::truthy(g.fog_of_war.door_n) {
            let from = g.fog_of_war.door_from as usize;
            while i < from { raw.push(list[i]); i += 1; }
            crate::doors::doors_boxes(g, &mut raw);
            i = (g.fog_of_war.door_from + g.fog_of_war.door_n) as usize;
        }
        while i < list.len() { raw.push(list[i]); i += 1; }
        g.fog_of_war.raw_list = raw;
        g.fog_of_war.raw_at = g.fog_of_war.frame_no;
    }
    if !fog_of_war_active(g) || g.fog_of_war.eyes.is_empty() {
        if g.fog_of_war.thing_recs.len() > 0 { g.fog_of_war.thing_recs.clear(); }
        g.fog_of_war.thing_since = -1.0;
        g.fog_of_war.props_loaded = false;
        return list.clone();
    }
    let f = g.fog_of_war.frame_no;
    let mut out = core::mem::take(&mut g.fog_of_war.thing_out);
    out.clear();
    if g.fog_of_war.thing_since < 0.0 || f < g.fog_of_war.thing_since { g.fog_of_war.thing_since = f; }
    let prime = f - g.fog_of_war.thing_since < THING_PRIME;
    if !prime { g.fog_of_war.props_loaded = false; }
    let loaded_p = g.fog_of_war.props_loaded;
    g.fog_of_war.cell_seen.clear();
    let ours = fog_of_war_ours_on(g);
    things_ours(g, ours, prime);
    let n = list.len();
    let (rig_from, rig_to) = (g.fog_of_war.rig_from, g.fog_of_war.rig_to);
    let mut i = 0;
    while i < n {
        let b = list[i];
        if i as f64 >= rig_from && (i as f64) < rig_to {
            out.push(b);
            i += 1;
            continue;
        }
        let (mkey, owner, c) = { let q = bx(g, b); (q.mkey, q.owner, q.c) };
        if mkey.is_none() {
            if prime || (ours && owner != 0.0 && g.engine.lights.fog_team.owners.has(crate::Num(owner))) || fog_of_war_cell_seen_at(g, c[0], c[2]) {
                out.push(b);
            } else {
                g.fog_of_war.thing_stats.out += 1.0;
            }
            i += 1;
            continue;
        }
        let tk = thing_key_of(g, b);
        if tk < 0.0 {
            out.push(b);
            i += 1;
            continue;
        }
        let mut j = i + 1;
        while j < n && bx(g, list[j]).mkey.is_some() && thing_key_of(g, list[j]) == tk { j += 1; }
        let mut x = 0.0;
        let mut z = 0.0;
        let mut k = i;
        while k < j {
            let c = bx(g, list[k]).c;
            x += c[0];
            z += c[2];
            k += 1;
        }
        x /= (j - i) as f64;
        z /= (j - i) as f64;
        fog_of_war_span(g, list, i as f64, j as f64);
        let (ox, oz) = (g.fog_of_war.span_x, g.fog_of_war.span_z);
        let where_ = (x, z, ox, oz);
        let key = crate::Num(tk);
        let mine = g.fog_of_war.our_chunks.len() > 0 && g.fog_of_war.our_chunks.has(key);
        let pr = prime && !(loaded_p && is_prop(tk));
        if !g.fog_of_war.thing_recs.has(key) {
            if pr || mine || fog_of_war_thing_seen(g, x, z, ox, oz) {
                thing_take(g, tk, list, i, j, where_, f, !pr);
                g.fog_of_war.thing_recs.get_mut(key).unwrap().f = f;
                out.extend_from_slice(&list[i..j]);
            } else {
                g.fog_of_war.thing_stats.out += 1.0;
            }
            i = j;
            continue;
        }
        g.fog_of_war.thing_recs.get_mut(key).unwrap().f = f;
        let ghost = g.fog_of_war.thing_recs.get(key).unwrap().ghost.clone();
        if thing_same(g, list, i, j, &ghost) {
            let r = g.fog_of_war.thing_recs.get_mut(key).unwrap();
            r.moving = false;
            r.held = false;
            out.extend_from_slice(&list[i..j]);
            i = j;
            continue;
        }
        let (held, moving, rx, rz, rox, roz) = { let r = g.fog_of_war.thing_recs.get(key).unwrap(); (r.held, r.moving, r.x, r.z, r.ox, r.oz) };
        if pr || mine || ((!held || js::rem(f + tk, THING_EVERY) == 0.0) && fog_of_war_thing_seen(g, x, z, ox, oz)) {
            thing_take(g, tk, list, i, j, where_, f, true);
            out.extend_from_slice(&list[i..j]);
            i = j;
            continue;
        }
        if !held {
            if moving || fog_of_war_thing_seen(g, rx, rz, rox, roz) {
                g.fog_of_war.thing_recs.delete(key);
                g.fog_of_war.thing_stats.out += 1.0;
                i = j;
                continue;
            }
            g.fog_of_war.thing_recs.get_mut(key).unwrap().held = true;
        } else if js::rem(f + tk, THING_EVERY) == 1.0 && fog_of_war_thing_seen(g, rx, rz, rox, roz) {
            g.fog_of_war.thing_recs.delete(key);
            g.fog_of_war.thing_stats.out += 1.0;
            i = j;
            continue;
        }
        out.extend_from_slice(&ghost);
        g.fog_of_war.thing_stats.held += 1.0;
        i = j;
    }
    // what has left the list altogether, out of sight: still in the picture where it was, until somebody looks there
    let mut at = 0;
    while let Some(key) = g.fog_of_war.thing_recs.next_key(&mut at) {
        let tk = key.0;
        let r = g.fog_of_war.thing_recs.get(key).unwrap();
        if r.f == f { continue; }
        let (moving, held, rx, rz, rox, roz) = (r.moving, r.held, r.x, r.z, r.ox, r.oz);
        if moving
            || g.fog_of_war.thing_recs.len() as f64 > THING_CAP
            || ((js::rem(f + tk, THING_EVERY) == 1.0 || !held) && fog_of_war_thing_seen(g, rx, rz, rox, roz))
        {
            g.fog_of_war.thing_recs.delete(key);
            continue;
        }
        let r = g.fog_of_war.thing_recs.get_mut(key).unwrap();
        r.held = true;
        out.extend_from_slice(&r.ghost);
        g.fog_of_war.thing_stats.held += 1.0;
    }
    if js::truthy(g.fog_of_war.thing_stats.held) || js::truthy(g.fog_of_war.thing_stats.out) {
        list.clear();
        list.extend_from_slice(&out);
    }
    g.fog_of_war.thing_out = out;
    list.clone()
}

/// A beam's point at `t`, looked at a hand short of its end.   (fogofwar.ts FogOfWar.beamRuns `at`, `see`)
fn beam_see(g: &mut Game, from: V3, dir: V3, l: f64, t: f64) -> bool {
    let p = v3::mad(from, dir, m::min(t, m::max(0.0, l - 0.05)));
    fog_of_war_eye_on(g, p)
}

/// The end of a seen stretch, by bisection between a seen and an unseen `t`.   (fogofwar.ts FogOfWar.beamRuns `edge`)
fn beam_edge(g: &mut Game, from: V3, dir: V3, l: f64, mut lit: f64, mut dark: f64) -> f64 {
    while m::abs(dark - lit) > BEAM_FINE {
        let mid = (lit + dark) / 2.0;
        if beam_see(g, from, dir, l, mid) { lit = mid; } else { dark = mid; }
    }
    lit
}

/// The stretches of a beam the team can see, as [t0, t1] along `from + dir·t`.   (fogofwar.ts FogOfWar.beamRuns)
pub fn fog_of_war_beam_runs(g: &mut crate::game::Game, from: V3, dir: V3, l: f64) -> Vec<[f64; 2]> {
    if !fog_of_war_active(g) { return vec![[0.0, l]]; }
    let mut runs: Vec<[f64; 2]> = Vec::new();
    let n = m::max(1.0, m::ceil(l / BEAM_STEP));
    let mut open = -1.0;
    let mut prev = false;
    let mut prev_t = 0.0;
    let mut k = 0.0;
    while k <= n {
        let t = if k == n { l } else { k * BEAM_STEP };
        let s = beam_see(g, from, dir, l, t);
        if s && !prev { open = if k == 0.0 { 0.0 } else { beam_edge(g, from, dir, l, t, prev_t) }; }
        if !s && prev {
            let end = beam_edge(g, from, dir, l, prev_t, t);
            runs.push([open, end]);
            open = -1.0;
        }
        prev = s;
        prev_t = t;
        k += 1.0;
    }
    if prev { runs.push([open, l]); }
    runs
}

/// Does anybody on the team see him, as of this frame's update. True for the team, and for everybody while the fog is off.   (fogofwar.ts FogOfWar.seen)
pub fn fog_of_war_seen(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if !fog_of_war_active(g) || fog_of_war_on_team(g, gd) { return true; }
    g.fog_of_war.recs.get(gd).map_or(false, |r| r.seen)
}

/// Where the team last had him and how long ago; null = never.   (fogofwar.ts FogOfWar.sighting)
pub fn fog_of_war_sighting(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::fogofwar::FogOfWarSighting> {
    let c = g.gd[gd].ch;
    if !fog_of_war_active(g) { return Some(FogOfWarSighting { at: g.ch[c].pos, age: 0.0 }); }
    let r = g.fog_of_war.recs.get(gd).copied();
    if r.map_or(false, |r| r.seen) || fog_of_war_fired_on(g, gd) { return Some(FogOfWarSighting { at: g.ch[c].pos, age: 0.0 }); }
    let r = r?;
    let at = r.at?;
    Some(FogOfWarSighting { at, age: g.time - r.t })
}

/// Is he in the picture: seen, seen a moment ago, fired on by the squad, or a body the team has seen.   (fogofwar.ts FogOfWar.drawn)
pub fn fog_of_war_drawn(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if !fog_of_war_active(g) || fog_of_war_on_team(g, gd) { return true; }
    let r = match g.fog_of_war.recs.get(gd).copied() { Some(r) => r, None => return false };
    if r.seen || (g.gd[gd].state == crate::game::GuardState::Dead && r.body) { return true; }
    let since = g.time - r.t;
    (since >= 0.0 && since < g.k.fog.linger) || fog_of_war_fired_on(g, gd) || g.fog_of_war.shown.has(gd) || r.framed == Some(true)
}

/// Is the squad shooting at him: a teammate's aimed round went at him inside FOG.squadFireSecs.   (fogofwar.ts FogOfWar.firedOn)
pub fn fog_of_war_fired_on(g: &mut crate::game::Game, gd: GuardId) -> bool {
    match g.fog_of_war.fired_at.get(gd) {
        Some(&t) => g.fog_of_war.fire_clock - t < g.k.fog.squad_fire_secs,
        None => false,
    }
}

/// The last sightings the HUD marks: each living enemy out of the picture now who was in it that recently.   (fogofwar.ts FogOfWar.lastSeen)
pub fn fog_of_war_last_seen(g: &mut crate::game::Game) -> Vec<crate::fogofwar::FogOfWarLastSeenItem> {
    let secs = if g.k.fog.last_seen > 0.0 { g.k.fog.last_seen } else { crate::told::told_last_seen(g) };
    let mut out: Vec<FogOfWarLastSeenItem> = Vec::new();
    if !fog_of_war_active(g) || g.fog_of_war.bared || !(secs > 0.0 || g.k.wand.mark_secs > 0.0) { return out; }
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        let team = g.gd[gd].team;
        if g.gd[gd].state == crate::game::GuardState::Dead
            || !crate::teams::is_hostile(g, team, crate::teams::Team::Player)
            || !g.fog_of_war.hidden_chars.has(g.gd[gd].ch)
        {
            continue;
        }
        let r = match g.fog_of_war.recs.get(gd).copied() { Some(r) => r, None => continue };
        let at = match r.at { Some(at) => at, None => continue };
        let age = g.time - r.t;
        let wand_had = match r.wand_t { Some(w) => m::abs(r.t - w) < 0.5, None => false };
        let life = if wand_had { m::max(secs, g.k.wand.mark_secs) } else { secs };
        if age >= 0.0 && age < life { out.push(FogOfWarLastSeenItem { gd, at, age, life }); }
    }
    out
}

/// Left out of the picture this frame; false for everybody while the fog is off.   (fogofwar.ts FogOfWar.hidden)
pub fn fog_of_war_hidden(g: &mut crate::game::Game, gd: GuardId) -> bool {
    !g.fog_of_war.bared && g.fog_of_war.hidden_chars.len() > 0 && g.fog_of_war.hidden_chars.has(g.gd[gd].ch)
}

/// The rule is live: the fog on, and the full picture's block filled.   (fogofwar.ts FogOfWar.oursOn)
pub fn fog_of_war_ours_on(g: &mut crate::game::Game) -> bool {
    fog_of_war_active(g) && g.fog_of_war.full_on
}

/// Is what `who` does the team's own to see, off the list of the team `full_block` filled.   (fogofwar.ts FogOfWar.oursId)
pub fn fog_of_war_ours_id(g: &mut crate::game::Game, who: Option<f64>) -> bool {
    let w = match who { Some(w) if js::truthy(w) => w, _ => return false };
    fog_of_war_ours_on(g) && g.engine.lights.fog_team.owners.has(crate::Num(w))
}

/// A light `who` made without carrying it takes its doer's fog class.   (fogofwar.ts FogOfWar.lightBy)
pub fn fog_of_war_light_by(g: &mut crate::game::Game, l: LightId, who: Option<f64>) -> LightId {
    if let Some(w) = who {
        if js::truthy(w) { seam_set_light_fog_by(g, l, w); }
    }
    l
}

/// What is filed from here to `done` is `who`'s doing. Returns what `done` puts back (−1: nothing).   (fogofwar.ts FogOfWar.begin)
pub fn fog_of_war_begin(g: &mut crate::game::Game, who: Option<f64>) -> f64 {
    if !who.map_or(false, js::truthy) { return -1.0; }
    let was = match crate::particle_fx::gpu_fx(g) { Some(p) => p.ours, None => return -1.0 };
    let ours = fog_of_war_ours_id(g, who);
    seam_set_gpu_fx_ours(g, ours);
    if was { 1.0 } else { 0.0 }
}

/// Puts back what `begin` returned.   (fogofwar.ts FogOfWar.done)
pub fn fog_of_war_done(g: &mut crate::game::Game, was: f64) {
    if was < 0.0 { return; }
    if crate::particle_fx::gpu_fx(g).is_some() { seam_set_gpu_fx_ours(g, was == 1.0); }
}

/// `begin` / `done` round `fn_`.   (fogofwar.ts FogOfWar.doing)
pub fn fog_of_war_doing<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, who: Option<f64>, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    let was = fog_of_war_begin(g, who);
    let out = fn_(g);
    fog_of_war_done(g, was);
    out
}

/// The rest of this doing is not shown; the `done` that closes it puts the mark back.   (fogofwar.ts FogOfWar.hush)
pub fn fog_of_war_hush(g: &mut crate::game::Game) {
    if crate::particle_fx::gpu_fx(g).is_some() { seam_set_gpu_fx_ours(g, false); }
}

/// What a hidden man puts into the air is not filed: true = `c` is out of the picture.   (fogofwar.ts FogOfWar.airHides)
pub fn fog_of_war_air_hides(g: &mut crate::game::Game, c_2: CharId) -> bool {
    g.fog_of_war.full_on && !g.fog_of_war.bared && g.fog_of_war.hidden_chars.len() > 0 && g.fog_of_war.hidden_chars.has(c_2)
}

/// The characters to draw, of the frame's list.   (fogofwar.ts FogOfWar.filter)
pub fn fog_of_war_filter(g: &mut crate::game::Game, all: &[CharId]) -> Vec<CharId> {
    all.iter().copied().filter(|&c| !g.fog_of_war.hidden_chars.has(c)).collect()
}

/// For a viewer's picture without the fog: everybody, the men the fog leaves out last.   (fogofwar.ts FogOfWar.everybody)
pub fn fog_of_war_everybody(g: &mut crate::game::Game, all: &[CharId]) -> Vec<CharId> {
    let mut out = fog_of_war_filter(g, all);
    out.extend(all.iter().copied().filter(|&c| g.fog_of_war.hidden_chars.has(c)));
    out
}

/// This frame's list of boxes as it stood before the fog cut it: no viewer, the fog standing aside, or nothing kept for this frame = None.   (fogofwar.ts FogOfWar.raw)
pub fn fog_of_war_raw(g: &mut crate::game::Game) -> Option<Vec<BoxId>> {
    if g.fog_of_war.viewer && fog_of_war_active(g) && g.fog_of_war.raw_at == g.fog_of_war.frame_no { Some(g.fog_of_war.raw_list.clone()) } else { None }
}

/// One line for the panel or the console.   (fogofwar.ts FogOfWar.summary)
pub fn fog_of_war_summary(g: &mut crate::game::Game) -> String {
    if !g.fog_of_war.on { return String::from("fog of war off"); }
    if !fog_of_war_active(g) { return String::from("fog of war on — standing aside for the show"); }
    let s = g.fog_of_war.stats;
    let mut line = alloc::format!("fog of war on · {} eye{} · {}/{} seen, {} hidden", s.eyes, if s.eyes == 1.0 { "" } else { "s" }, s.seen, s.tested, s.hidden);
    if js::truthy(s.smoke_hid) { line.push_str(&alloc::format!(" · smoke hid {}", s.smoke_hid)); }
    let deg = field_deg(g);
    if deg < 360.0 { line.push_str(&alloc::format!(" · {}° field", deg)); }
    line
}

/// `l.fogBy = who`: a light row's field, which the skeleton has no way to reach.   (fogofwar.ts:1136)
fn seam_set_light_fog_by(g: &mut Game, l: LightId, who: f64) {
    g.tab.light[l].fog_by = Some(who); // (skeleton 4 has the row)
}

/// `P.ours = ours` on `gpuFx(g)`: its stub hands the Particles over by value, so a write is lost.   (fogofwar.ts:1140, 1141, 1146)
fn seam_set_gpu_fx_ours(g: &mut Game, ours: bool) {
    todo!("needs: gpuFx(g) as a place to write Particles.ours (HostApi has no particles row)")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game() -> alloc::boxed::Box<Game> { alloc::boxed::Box::new(Game::blank()) }

    fn add_box(g: &mut Game, h: V3, yaw: f64, flags: f64) -> BoxId {
        let mut b = crate::scene_boxes::Box::zeroed();
        b.h = h;
        b.yaw = yaw;
        b.flags = flags;
        g.col.boxes.push(b)
    }

    /// The bits of span's arithmetic, printed by the TypeScript's own formula under EXACT.
    #[test]
    fn span_is_the_typescripts_bits() {
        let mut g = game();
        let cases: [(&[(V3, f64)], u64, u64); 3] = [
            (&[([1.2, 0.5, 0.3], 0.7), ([0.2, 0.5, 2.05], -1.3)], 0xbffd4acb93ae7660, 0x3fe04390d6282ef4),
            (&[([0.4, 0.1, 0.45], 2.0)], 0, 0),
            (&[([0.9, 0.1, 0.2], 1.1)], 0x3fd5c5ca051588cd, 0xbfe56393f9220a6a),
        ];
        for (rows, x, z) in cases {
            let list: Vec<BoxId> = rows.iter().map(|&(h, yaw)| add_box(&mut g, h, yaw, 0.0)).collect();
            fog_of_war_span(&mut g, &list, 0.0, list.len() as f64);
            assert_eq!(g.fog_of_war.span_x.to_bits(), x);
            assert_eq!(g.fog_of_war.span_z.to_bits(), z);
        }
    }

    /// A half-metre square is asked once a frame: the second ask takes no test, and the key is the TypeScript's.
    #[test]
    fn a_cell_is_asked_once() {
        let mut g = game();
        assert!(!fog_of_war_cell_seen_at(&mut g, -3.3, 7.9));
        assert!(!fog_of_war_cell_seen_at(&mut g, -3.3, 7.9));
        assert_eq!(g.fog_of_war.thing_stats.tests, 1.0);
        assert!(g.fog_of_war.cell_seen.has(crate::Num(f64::from_bits(0x418ff28078000000))));
    }

    /// The men the fog leaves out come last, each list in the order it was given.
    #[test]
    fn everybody_puts_the_hidden_last() {
        let mut g = game();
        let all = [CharId(0), CharId(1), CharId(2), CharId(3)];
        g.fog_of_war.hidden_chars.add(CharId(0));
        g.fog_of_war.hidden_chars.add(CharId(2));
        assert_eq!(fog_of_war_filter(&mut g, &all), vec![CharId(1), CharId(3)]);
        assert_eq!(fog_of_war_everybody(&mut g, &all), vec![CharId(1), CharId(3), CharId(0), CharId(2)]);
    }

    /// A lit lamp's face: what was kept if it glowed, the level's own if not, dark when out; a box rays can read is left alone; None takes it off.
    #[test]
    fn a_face_is_kept_lit_or_dark() {
        let mut g = game();
        let area = crate::scene_boxes::box_flag().area_lit;
        let shown = add_box(&mut g, [1.0; 3], 0.0, area);
        let ray_met = add_box(&mut g, [1.0; 3], 0.0, 0.0);
        g.engine.world.statics = vec![shown, ray_met];
        g.col.boxes[shown].emissive = [2.0, 1.0, 0.5];
        let mut row = crate::sim_lightshome::Fixture::blank();
        row.boxes = vec![0.0, 1.0, 9.0];
        row.base_emissive = [7.0, 7.0, 7.0];
        let t = g.tab.sim_lightshome_fixture.push(row);

        fog_of_war_lamp_face(&mut g, t, Some(true));
        assert_eq!(g.col.boxes[shown].fog_emit, Some([7.0, 7.0, 7.0]));
        fog_of_war_face_seen(&mut g, t);
        fog_of_war_lamp_face(&mut g, t, Some(true));
        assert_eq!(g.col.boxes[shown].fog_emit, Some([2.0, 1.0, 0.5]));
        assert_eq!(g.col.boxes[shown].fog_mark, Some(1.0));
        assert_eq!(g.col.boxes[ray_met].fog_emit, None);
        fog_of_war_lamp_face(&mut g, t, Some(false));
        assert_eq!(g.col.boxes[shown].fog_emit, Some([0.0, 0.0, 0.0]));
        assert_eq!(g.col.boxes[shown].fog_mark, Some(0.0));
        fog_of_war_lamp_face(&mut g, t, None);
        assert_eq!(g.col.boxes[shown].fog_emit, None);
        assert_eq!(g.col.boxes[shown].fog_mark, None);
    }

    /// First met = as it stands, and no look is asked for while it agrees.
    #[test]
    fn a_lamp_is_first_known_as_it_stands() {
        let mut g = game();
        let t = g.tab.sim_lightshome_fixture.push(crate::sim_lightshome::Fixture::blank());
        assert!(fog_of_war_lamp_known(&mut g, t, true));
        assert!(fog_of_war_lamp_known(&mut g, t, true));
        assert_eq!(g.fog_of_war.lamp_recs.get(t).copied(), Some(true));
    }
}
