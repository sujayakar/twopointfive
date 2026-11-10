use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};


/// Is the fixture lit, as it stands.
pub fn fog_of_war_lamp_lit(g: &crate::game::Game, t: FixtureId) -> bool {
    let on = g.tab.fixture[t].on;
    let broken = g.tab.fixture[t].broken;
    let until = g.tab.fixture[t].blind_until;
    on && !broken && !(until > 0.0 && g.time < until)
}

/// The fixture as the team last saw it: first met = as it stands; it comes up to date the frame it differs and the team has a look at it.
pub(super) fn fog_of_war_lamp_known(g: &mut crate::game::Game, t: FixtureId, lit: bool) -> bool {
    let rec = g.fog_of_war.lamp_recs.get(t).copied();
    let stale = match rec { None => true, Some(r) => r != lit && fog_of_war_lamp_seen(g, t) };
    if stale {
        g.fog_of_war.lamp_recs.set(t, lit);
        return lit;
    }
    rec.unwrap()
}

/// Is the floor at (x, z) in the lit part of the picture: some eye has it.
pub(super) fn fog_of_war_spot_seen(g: &mut crate::game::Game, x: f64, z: f64) -> bool {
    g.fog_of_war.thing_stats.tests += 1.0;
    fog_of_war_ground_look(g, x, z) == 2.0
}

/// `spot_seen`'s walk: 2 = an eye has the spot, 1 = in an eye's field and every such eye's line stopped, 0 = in no eye's field.
fn fog_of_war_ground_look(g: &crate::game::Game, x: f64, z: f64) -> f64 {
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

/// A one-frame box's spot, asked once per half-metre square a frame.
fn fog_of_war_cell_seen_at(g: &mut crate::game::Game, x: f64, z: f64) -> bool {
    let k = (m::floor(x * 2.0) + 4096.0) * 16384.0 + m::floor(z * 2.0) + 4096.0;
    if let Some(&v) = g.fog_of_war.cell_seen.get(crate::Num(k)) { return v; }
    let v = fog_of_war_spot_seen(g, x, z);
    g.fog_of_war.cell_seen.set(crate::Num(k), v);
    v
}

/// A thing is seen where its middle is, or a long one at either end of its long side.
fn fog_of_war_thing_seen(g: &mut crate::game::Game, x: f64, z: f64, ox: f64, oz: f64) -> bool {
    fog_of_war_spot_seen(g, x, z)
        || ((ox != 0.0 || oz != 0.0) && (fog_of_war_spot_seen(g, x + ox, z + oz) || fog_of_war_spot_seen(g, x - ox, z - oz)))
}

/// One box's long side against the best so far: None = no longer than it; else its length and its axis in plan, 15 cm in from the ends.
fn long_side(h: V3, yaw: f64, best: f64) -> Option<(f64, f64, f64)> {
    let along_x = h[0] >= h[2];
    let l = if along_x { h[0] } else { h[2] };
    if l <= best { return None; }
    let in_ = l - 0.15;
    let cs = m::cos(yaw) * in_;
    let sn = m::sin(yaw) * in_;
    Some(if along_x { (l, cs, -sn) } else { (l, sn, cs) })
}

/// The long side of boxes i‥j of a list into `span_x` / `span_z`: its biggest box's long axis in plan, 15 cm in from the ends; 0, 0 under 0.9 m long.
fn fog_of_war_span(g: &mut crate::game::Game, list: &[AnyBoxId], i: f64, j: f64) {
    let mut best = 0.45;
    g.fog_of_war.span_x = 0.0;
    g.fog_of_war.span_z = 0.0;
    let mut k = i as usize;
    while k < j as usize {
        let q = &g.tab[list[k].into()];
        let (h, yaw) = (q.h, q.yaw);
        k += 1;
        if let Some((l, x, z)) = long_side(h, yaw, best) {
            best = l;
            g.fog_of_war.span_x = x;
            g.fog_of_war.span_z = z;
        }
    }
}

/// `take` of `things`: the picture has boxes i‥j of the list as they stand.
fn thing_take(g: &mut Game, tk: f64, list: &[AnyBoxId], i: usize, j: usize, (x, z, ox, oz): (f64, f64, f64, f64), f: f64, moving: bool) {
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

/// Do boxes i‥j stand where the record's copies do?
fn thing_same(g: &Game, list: &[AnyBoxId], i: usize, j: usize, ghost: &[FrameBoxId]) -> bool {
    if ghost.len() != j - i { return false; }
    let mut k = i;
    while k < j {
        let (sc, sy, sr) = { let s = &g.tab[list[k].into()]; (s.c, s.yaw, s.rot) };
        let (qc, qy, qr) = { let q = &g.tab[ghost[k - i].into()]; (q.c, q.yaw, q.rot) };
        k += 1;
        if !(m::abs(sc[0] - qc[0]) + m::abs(sc[1] - qc[1]) + m::abs(sc[2] - qc[2]) < THING_EPS && m::abs(sy - qy) < THING_EPS) { return false; }
        if let (Some(a), Some(b)) = (sr, qr) {
            if !(m::abs(a[0] - b[0]) + m::abs(a[1] - b[1]) + m::abs(a[2] - b[2]) + m::abs(a[3] - b[3]) < THING_EPS) { return false; }
        }
    }
    true
}

/// The team's own rubble on its first way down, for `things`: the frame's keys into `our_chunks`, the ones at rest into `our_landed`.
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
            if g.tab.boxes[mate].mkey.is_some() {
                let mk = thing_key_of(g, mate.into());
                g.fog_of_war.our_chunks.add(crate::Num(mk));
            }
        }
    }
}

/// The picture's list of loose things, rewritten thing by thing: live where the team sees the floor it is on; out of sight, the copy where it was last seen at rest, or nothing. Hands back the list as it now stands.
pub fn fog_of_war_things(g: &mut crate::game::Game, list: &mut Vec<AnyBoxId>) -> Vec<AnyBoxId> {
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
        let (mkey, owner, c) = { let q = &g.tab[b]; (q.mkey, q.owner, q.c) };
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
        while j < n && g.tab[list[j].into()].mkey.is_some() && thing_key_of(g, list[j]) == tk { j += 1; }
        let mut x = 0.0;
        let mut z = 0.0;
        let mut k = i;
        while k < j {
            let c = g.tab[list[k].into()].c;
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
        out.extend(ghost.iter().map(|&b| AnyBoxId::Frame(b)));
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
        out.extend(r.ghost.iter().map(|&b| AnyBoxId::Frame(b)));
        g.fog_of_war.thing_stats.held += 1.0;
    }
    if js::truthy(g.fog_of_war.thing_stats.held) || js::truthy(g.fog_of_war.thing_stats.out) {
        list.clear();
        list.extend_from_slice(&out);
    }
    g.fog_of_war.thing_out = out;
    list.clone()
}

/// Does anybody on the team see him, as of this frame's update. True for the team, and for everybody while the fog is off.
pub fn fog_of_war_seen(g: &crate::game::Game, gd: GuardId) -> bool {
    if !fog_of_war_active(g) || g.team(gd).ours() { return true; }
    g.fog_of_war.recs.get(gd).map_or(false, |r| r.seen)
}

/// Where the team last had him and how long ago; null = never.
pub fn fog_of_war_sighting(g: &crate::game::Game, gd: GuardId) -> Option<crate::fogofwar::FogOfWarSighting> {
    let c = g.gd[gd].ch;
    if !fog_of_war_active(g) { return Some(FogOfWarSighting { at: g.ch[c].pos, age: 0.0 }); }
    let r = g.fog_of_war.recs.get(gd).copied();
    if r.map_or(false, |r| r.seen) || fog_of_war_fired_on(g, gd) { return Some(FogOfWarSighting { at: g.ch[c].pos, age: 0.0 }); }
    let r = r?;
    let at = r.at?;
    Some(FogOfWarSighting { at, age: g.time - r.t })
}

/// Is he in the picture: seen, seen a moment ago, fired on by the squad, or a body the team has seen.
pub fn fog_of_war_drawn(g: &crate::game::Game, gd: GuardId) -> bool {
    if !fog_of_war_active(g) || g.team(gd).ours() { return true; }
    let r = match g.fog_of_war.recs.get(gd).copied() { Some(r) => r, None => return false };
    if r.seen || (crate::condition::down(g, g.gd[gd].ch) && r.body) { return true; }
    let since = g.time - r.t;
    (since >= 0.0 && since < g.k.fog.linger) || fog_of_war_fired_on(g, gd) || r.framed == Some(true)
}

/// Is the squad shooting at him: a teammate's aimed round went at him inside FOG.squadFireSecs.
pub(super) fn fog_of_war_fired_on(g: &crate::game::Game, gd: GuardId) -> bool {
    match g.fog_of_war.fired_at.get(gd) {
        Some(&t) => g.fog_of_war.fire_clock - t < g.k.fog.squad_fire_secs,
        None => false,
    }
}

/// Left out of the picture this frame; false for everybody while the fog is off.
pub fn fog_of_war_hidden(g: &crate::game::Game, gd: GuardId) -> bool { fog_of_war_hides(g, gd) }
/// The last sightings: each living enemy out of the picture now, and in it recently enough: where he stood, and how long ago. Nobody's AI reads it.
pub fn fog_of_war_last_seen(g: &crate::game::Game) -> Vec<crate::view::SeenMark> {
    let told = match crate::told::told_now(g) { crate::told::ToldRung::Guided => m::max(0.0, g.k.told.guided), crate::told::ToldRung::Standard => m::max(0.0, g.k.told.standard), _ => 0.0 };
    let secs = if g.k.fog.last_seen > 0.0 { g.k.fog.last_seen } else { told };
    let wand = g.k.wand.mark_secs > 0.0;
    let mut out = Vec::new();
    if !fog_of_war_active(g) || !(secs > 0.0 || wand) { return out; }
    for &gd in g.guards.iter() {
        if crate::condition::down(g, g.gd[gd].ch) || !g.team(gd).theirs() || !fog_of_war_hides(g, gd) { continue; }
        let Some(r) = g.fog_of_war.recs.get(gd) else { continue };
        let Some(at) = r.at else { continue };
        let age = g.time - r.t;
        let life = if wand && r.wand_t.map_or(false, |w| m::abs(r.t - w) < 0.5) { m::max(secs, g.k.wand.mark_secs) } else { secs };
        if age >= 0.0 && age < life { out.push(crate::view::SeenMark { man: g.ch[g.gd[gd].ch].id, at, age, life }); }
    }
    out
}
/// does the team not see this man (asked of the game shared: no tool's tap is told)
pub fn fog_of_war_hides(g: &crate::game::Game, gd: GuardId) -> bool {
    g.fog_of_war.hidden_chars.len() > 0 && g.fog_of_war.hidden_chars.has(g.gd[gd].ch)
}

/// The rule is live: the fog on, and the full picture's block filled.
fn fog_of_war_ours_on(g: &crate::game::Game) -> bool {
    fog_of_war_active(g) && g.fog_of_war.live
}

/// Is what `who` does the team's own to see, off the list of the team `full_block` filled.
pub fn fog_of_war_ours_id(g: &crate::game::Game, who: Option<f64>) -> bool {
    let w = match who { Some(w) if js::truthy(w) => w, _ => return false };
    fog_of_war_ours_on(g) && g.engine.lights.fog_team.owners.has(crate::Num(w))
}

/// A light `who` made without carrying it takes its doer's fog class.
pub fn fog_of_war_light_by(g: &mut crate::game::Game, l: LightId, who: Option<f64>) -> LightId {
    if let Some(w) = who {
        if js::truthy(w) { g.tab.light[l].fog_by = Some(w); }
    }
    l
}

/// What is filed from here to `done` is `who`'s doing. Returns what `done` puts back (−1: nothing).
pub fn fog_of_war_begin(g: &mut crate::game::Game, who: Option<f64>) -> f64 {
    if !who.map_or(false, js::truthy) { return -1.0; }
    let was = g.fog_of_war.filing_ours;
    let ours = fog_of_war_ours_id(g, who);
    g.fog_of_war.filing_ours = ours;
    if was { 1.0 } else { 0.0 }
}

/// Puts back what `begin` returned.
pub fn fog_of_war_done(g: &mut crate::game::Game, was: f64) {
    if was < 0.0 { return; }
    g.fog_of_war.filing_ours = was == 1.0;
}

/// `begin` / `done` round `fn_`.
pub fn fog_of_war_doing<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, who: Option<f64>, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    let was = fog_of_war_begin(g, who);
    let out = fn_(g);
    fog_of_war_done(g, was);
    out
}

/// The rest of this doing is not shown; the `done` that closes it puts the mark back.
pub fn fog_of_war_hush(g: &mut crate::game::Game) {
    g.fog_of_war.filing_ours = false;
}

/// What a hidden man puts into the air is not filed: true = `c` is out of the picture.
pub fn fog_of_war_air_hides(g: &crate::game::Game, c_2: CharId) -> bool {
    g.fog_of_war.live && g.fog_of_war.hidden_chars.len() > 0 && g.fog_of_war.hidden_chars.has(c_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game() -> alloc::boxed::Box<Game> { alloc::boxed::Box::new(Game::blank()) }

    fn spans(rows: &[(V3, f64)]) -> (f64, f64) {
        let mut g = game();
        let list: Vec<AnyBoxId> = rows.iter().map(|&(h, yaw)| {
            let b = g.tab.boxes.push(crate::scene_boxes::Box { h, yaw, ..crate::scene_boxes::Box::default() });
            AnyBoxId::from(b)
        }).collect();
        fog_of_war_span(&mut g, &list, 0.0, list.len() as f64);
        (g.fog_of_war.span_x, g.fog_of_war.span_z)
    }

    /// A long thing is seen from either end of its biggest box's long side, 15 cm in from the ends. One under 0.9 m long is seen by its middle alone.
    #[test]
    fn a_long_thing_is_seen_by_its_long_side() {
        assert_eq!(spans(&[([1.2, 0.5, 0.3], 0.0)]), (1.05, -0.0));
        assert_eq!(spans(&[([0.3, 0.5, 1.2], 0.0)]), (0.0, 1.05));
        assert_eq!(spans(&[([0.4, 0.1, 0.45], 2.0)]), (0.0, 0.0));
        let (x, z) = spans(&[([0.9, 0.1, 0.2], 1.1)]);
        assert!((m::hyp2(x, z) - 0.75).abs() < 1e-12, "turned with the box, and just as long");
        assert!(x > 0.0 && z < 0.0);
        // of two boxes, the longer's, in whichever order they come
        let (a, b) = (([1.2, 0.5, 0.3], 0.0), ([0.2, 0.5, 2.05], 0.0));
        assert_eq!(spans(&[a, b]), (0.0, 1.9));
        assert_eq!(spans(&[b, a]), (0.0, 1.9));
    }

    /// A half-metre square is asked once a frame: another point of the same square takes no test, and a neighbour in either direction is a square of its own.
    #[test]
    fn a_cell_is_asked_once() {
        let mut g = game();
        assert!(!fog_of_war_cell_seen_at(&mut g, -3.3, 7.9));
        assert!(!fog_of_war_cell_seen_at(&mut g, -3.3, 7.9));
        assert!(!fog_of_war_cell_seen_at(&mut g, -3.2, 7.6));
        assert_eq!(g.fog_of_war.thing_stats.tests, 1.0);
        assert!(!fog_of_war_cell_seen_at(&mut g, -3.6, 7.9));
        assert_eq!(g.fog_of_war.thing_stats.tests, 2.0);
        assert!(!fog_of_war_cell_seen_at(&mut g, -3.3, 8.1));
        assert_eq!(g.fog_of_war.thing_stats.tests, 3.0);
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
        let t = g.tab.fixture.push(crate::fixtures::Fixture::blank());
        assert!(fog_of_war_lamp_known(&mut g, t, true));
        assert!(fog_of_war_lamp_known(&mut g, t, true));
        assert_eq!(g.fog_of_war.lamp_recs.get(t).copied(), Some(true));
    }
}
