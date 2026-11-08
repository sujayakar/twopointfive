//! src/game/fogofwar.ts lines 806 to 1156
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A box's row. Every box read or written in this file goes through here, so this one body is all that changes when the table of Box rows is back.
fn bx<'g>(g: &'g mut Game, b: BoxId) -> &'g mut crate::scene_boxes::Box { &mut g.tab.boxes[b] }

/// The live GPU particle system or None, as a place to write `ours`: `gpu_fx` answers only whether there is one.   (fogofwar.ts `gpuFx(this.g)`, particleFx.ts:25)
fn gpu_particles(g: &mut Game) -> Option<&mut crate::render_particles::Particles> {
    if !crate::particle_fx::gpu_fx(g) { return None; }
    g.engine.particles.as_mut()
}






/// Is the fixture lit, as it stands.   (fogofwar.ts FogOfWar.lampLit)
pub fn fog_of_war_lamp_lit(g: &crate::game::Game, t: FixtureId) -> bool {
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




/// Is the floor at (x, z) in the lit part of the picture: some eye has it.   (fogofwar.ts FogOfWar.spotSeen)
#[doc(hidden)] pub fn fog_of_war_spot_seen__body(g: &mut crate::game::Game, x: f64, z: f64) -> bool {
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

/// One box's long side against the best so far: None = no longer than it; else its length and its axis in plan, 15 cm in from the ends.   (fogofwar.ts FogOfWar.span)
fn long_side(h: V3, yaw: f64, best: f64) -> Option<(f64, f64, f64)> {
    let along_x = h[0] >= h[2];
    let l = if along_x { h[0] } else { h[2] };
    if l <= best { return None; }
    let in_ = l - 0.15;
    let cs = m::cos(yaw) * in_;
    let sn = m::sin(yaw) * in_;
    Some(if along_x { (l, cs, -sn) } else { (l, sn, cs) })
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
        if let Some((l, x, z)) = long_side(h, yaw, best) {
            best = l;
            g.fog_of_war.span_x = x;
            g.fog_of_war.span_z = z;
        }
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
    // (fogofwar.ts:1036 writes fresh copies over the record's list and the copies before them are dropped: here each was a row of the box table, and a
    //  thing in sight is taken every frame, so the rows are let go as they are replaced. Before this a chair in view left five rows a frame for good.)
    let before = core::mem::take(&mut g.fog_of_war.thing_recs.get_mut(crate::Num(tk)).unwrap().ghost);
    for old in before { crate::scene_boxes::box_let_go(g, old); }
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
pub fn fog_of_war_seen(g: &crate::game::Game, gd: GuardId) -> bool {
    if !fog_of_war_active(g) || fog_of_war_on_team(g, gd) { return true; }
    g.fog_of_war.recs.get(gd).map_or(false, |r| r.seen)
}

/// Where the team last had him and how long ago; null = never.   (fogofwar.ts FogOfWar.sighting)
pub fn fog_of_war_sighting(g: &crate::game::Game, gd: GuardId) -> Option<crate::fogofwar::FogOfWarSighting> {
    let c = g.gd[gd].ch;
    if !fog_of_war_active(g) { return Some(FogOfWarSighting { at: g.ch[c].pos, age: 0.0 }); }
    let r = g.fog_of_war.recs.get(gd).copied();
    if r.map_or(false, |r| r.seen) || fog_of_war_fired_on(g, gd) { return Some(FogOfWarSighting { at: g.ch[c].pos, age: 0.0 }); }
    let r = r?;
    let at = r.at?;
    Some(FogOfWarSighting { at, age: g.time - r.t })
}

/// Is he in the picture: seen, seen a moment ago, fired on by the squad, or a body the team has seen.   (fogofwar.ts FogOfWar.drawn)
pub fn fog_of_war_drawn(g: &crate::game::Game, gd: GuardId) -> bool {
    if !fog_of_war_active(g) || fog_of_war_on_team(g, gd) { return true; }
    let r = match g.fog_of_war.recs.get(gd).copied() { Some(r) => r, None => return false };
    if r.seen || (g.gd[gd].state == crate::game::GuardState::Dead && r.body) { return true; }
    let since = g.time - r.t;
    (since >= 0.0 && since < g.k.fog.linger) || fog_of_war_fired_on(g, gd) || g.fog_of_war.shown.has(gd) || r.framed == Some(true)
}

/// Is the squad shooting at him: a teammate's aimed round went at him inside FOG.squadFireSecs.   (fogofwar.ts FogOfWar.firedOn)
pub fn fog_of_war_fired_on(g: &crate::game::Game, gd: GuardId) -> bool {
    match g.fog_of_war.fired_at.get(gd) {
        Some(&t) => g.fog_of_war.fire_clock - t < g.k.fog.squad_fire_secs,
        None => false,
    }
}

/// Left out of the picture this frame; false for everybody while the fog is off.   (fogofwar.ts FogOfWar.hidden)
#[doc(hidden)] pub fn fog_of_war_hidden__body(g: &crate::game::Game, gd: GuardId) -> bool { fog_of_war_hides(g, gd) }
/// does the team not see this man (asked of the game shared: no tool's tap is told)
pub fn fog_of_war_hides(g: &crate::game::Game, gd: GuardId) -> bool {
    g.fog_of_war.hidden_chars.len() > 0 && g.fog_of_war.hidden_chars.has(g.gd[gd].ch)
}

/// The rule is live: the fog on, and the full picture's block filled.   (fogofwar.ts FogOfWar.oursOn)
pub fn fog_of_war_ours_on(g: &crate::game::Game) -> bool {
    fog_of_war_active(g) && g.fog_of_war.live
}

/// Is what `who` does the team's own to see, off the list of the team `full_block` filled.   (fogofwar.ts FogOfWar.oursId)
pub fn fog_of_war_ours_id(g: &crate::game::Game, who: Option<f64>) -> bool {
    let w = match who { Some(w) if js::truthy(w) => w, _ => return false };
    fog_of_war_ours_on(g) && g.engine.lights.fog_team.owners.has(crate::Num(w))
}

/// A light `who` made without carrying it takes its doer's fog class.   (fogofwar.ts FogOfWar.lightBy)
pub fn fog_of_war_light_by(g: &mut crate::game::Game, l: LightId, who: Option<f64>) -> LightId {
    if let Some(w) = who {
        if js::truthy(w) { g.tab.light[l].fog_by = Some(w); }
    }
    l
}

/// What is filed from here to `done` is `who`'s doing. Returns what `done` puts back (−1: nothing).   (fogofwar.ts FogOfWar.begin)
pub fn fog_of_war_begin(g: &mut crate::game::Game, who: Option<f64>) -> f64 {
    if !who.map_or(false, js::truthy) { return -1.0; }
    let was = match gpu_particles(g) { Some(p) => p.ours, None => return -1.0 };
    let ours = fog_of_war_ours_id(g, who);
    if let Some(p) = gpu_particles(g) { p.ours = ours; }
    if was { 1.0 } else { 0.0 }
}

/// Puts back what `begin` returned.   (fogofwar.ts FogOfWar.done)
pub fn fog_of_war_done(g: &mut crate::game::Game, was: f64) {
    if was < 0.0 { return; }
    if let Some(p) = gpu_particles(g) { p.ours = was == 1.0; }
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
    if let Some(p) = gpu_particles(g) { p.ours = false; }
}

/// What a hidden man puts into the air is not filed: true = `c` is out of the picture.   (fogofwar.ts FogOfWar.airHides)
pub fn fog_of_war_air_hides(g: &crate::game::Game, c_2: CharId) -> bool {
    g.fog_of_war.live && g.fog_of_war.hidden_chars.len() > 0 && g.fog_of_war.hidden_chars.has(c_2)
}


#[doc(hidden)] pub fn fog_of_war_hidden(g: &mut crate::game::Game, gd: GuardId) -> bool {
    use crate::io::Io;
    if g.taps & (1u64 << 13) == 0 { return fog_of_war_hidden__body(g, gd); }
    let mut o = crate::io::W::new(); gd.w(&mut o);
    let r = fog_of_war_hidden__body(g, gd);
    let mut a = crate::io::W::new(); r.w(&mut a);
    crate::out_events::push_tap(g, 13, &o.b, &a.b);
    r
}


#[doc(hidden)] pub fn fog_of_war_spot_seen(g: &mut crate::game::Game, x: f64, z: f64) -> bool {
    use crate::io::Io;
    if g.taps & (1u64 << 14) == 0 { return fog_of_war_spot_seen__body(g, x, z); }
    let mut o = crate::io::W::new(); x.w(&mut o); z.w(&mut o);
    let r = fog_of_war_spot_seen__body(g, x, z);
    let mut a = crate::io::W::new(); r.w(&mut a);
    crate::out_events::push_tap(g, 14, &o.b, &a.b);
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game() -> alloc::boxed::Box<Game> { alloc::boxed::Box::new(Game::blank()) }

    /// The bits of span's arithmetic, printed by the TypeScript's own formula under EXACT.
    #[test]
    fn span_is_the_typescripts_bits() {
        let cases: [(&[(V3, f64)], u64, u64); 3] = [
            (&[([1.2, 0.5, 0.3], 0.7), ([0.2, 0.5, 2.05], -1.3)], 0xbffd4acb93ae7660, 0x3fe04390d6282ef4),
            (&[([0.4, 0.1, 0.45], 2.0)], 0, 0),
            (&[([0.9, 0.1, 0.2], 1.1)], 0x3fd5c5ca051588cd, 0xbfe56393f9220a6a),
        ];
        for (rows, x, z) in cases {
            let (mut best, mut sx, mut sz) = (0.45, 0.0f64, 0.0f64);
            for &(h, yaw) in rows {
                if let Some((l, a, b)) = long_side(h, yaw, best) { best = l; sx = a; sz = b; }
            }
            assert_eq!(sx.to_bits(), x);
            assert_eq!(sz.to_bits(), z);
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

    /// The men the fog leaves out are not on the list to draw; the rest keep the order they were given.
    #[test]
    fn the_hidden_are_left_off_the_list_to_draw() {
        let mut g = game();
        let all = [CharId(0), CharId(1), CharId(2), CharId(3)];
        g.fog_of_war.hidden_chars.add(CharId(0));
        g.fog_of_war.hidden_chars.add(CharId(2));
        assert_eq!(fog_of_war_filter(&mut g, &all), vec![CharId(1), CharId(3)]);
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
