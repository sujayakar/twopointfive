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
    // (fresh copies go over the record's list, and each copy before them is a row of the box table. A thing in sight is
    //  taken every frame, so the rows are let go as they are replaced, or a chair in view leaves five rows a frame for good.)
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
    let f = &mut g.fog_of_war;
    f.thing_stats.held = 0.0;
    f.thing_stats.out = 0.0;
    f.thing_stats.tests = 0.0;
    if !fog_of_war_active(g) || g.fog_of_war.eyes.is_empty() {
        let f = &mut g.fog_of_war;
        f.thing_recs.clear();
        f.thing_since = -1.0;
        f.props_loaded = false;
        return list.clone();
    }
    let look = Look::of(g);
    things_ours(g, look.ours, look.prime);
    let mut out = core::mem::take(&mut g.fog_of_war.thing_out);
    out.clear();
    let (rig_from, rig_to) = (g.fog_of_war.rig_from as usize, g.fog_of_war.rig_to as usize);
    // (what men wear is theirs, and no thing of the list: it is asked for no key)
    let worn = |i: usize| i >= rig_from && i < rig_to;
    let keys: Vec<Option<f64>> = list.iter().enumerate().map(|(i, &b)| if worn(i) { None } else { g.tab[b].mkey.map(|k| key_of_mkey(g, k)) }).collect();
    let mut i = 0;
    while i < list.len() {
        if worn(i) { out.push(list[i]); i += 1; continue; }
        i = thing(g, look, list, &keys, i, &mut out);
    }
    things_gone(g, look, &mut out);
    let stats = &g.fog_of_war.thing_stats;
    if stats.held != 0.0 || stats.out != 0.0 { list.clone_from(&out); }
    g.fog_of_war.thing_out = out;
    list.clone()
}

/// How a pass over the things looks at them: its number, whether it is of the first few that take everything as seen (but the props of a save), and whether the team's own doing shows.
#[derive(Clone, Copy)]
struct Look { f: f64, prime: bool, props_loaded: bool, ours: bool }
impl Look {
    fn of(g: &mut Game) -> Look {
        let f = &mut g.fog_of_war;
        let n = f.frame_no;
        if f.thing_since < 0.0 || n < f.thing_since { f.thing_since = n; }
        let prime = n - f.thing_since < THING_PRIME;
        if !prime { f.props_loaded = false; }
        f.cell_seen.clear();
        Look { f: n, prime, props_loaded: f.props_loaded, ours: fog_of_war_ours_on(g) }
    }
    /// Whose turn of the few a thing out of sight is asked about in: on 0 it is looked for where it is, on 1 where it was last seen.
    fn turn(&self, key: f64) -> f64 { js::rem(self.f + key, THING_EVERY) }
}

/// One thing of the list, its boxes from `i` on: shown, shown as it was last seen, or left out. Returns where the next begins.
fn thing(g: &mut Game, look: Look, list: &[AnyBoxId], keys: &[Option<f64>], i: usize, out: &mut Vec<AnyBoxId>) -> usize {
    let Some(tk) = keys[i] else {
        let b = &g.tab[list[i]];
        let (owner, c) = (b.owner, b.c);
        if look.prime || (look.ours && owner != 0.0 && g.engine.lights.fog_team.owners.has(crate::Num(owner))) || fog_of_war_cell_seen_at(g, c[0], c[2]) { out.push(list[i]); }
        else { g.fog_of_war.thing_stats.out += 1.0; }
        return i + 1;
    };
    if tk < 0.0 { out.push(list[i]); return i + 1; }
    let j = i + keys[i..].iter().take_while(|&&k| k == Some(tk)).count();
    if thing_shows(g, look, list, i, j, tk) { out.extend_from_slice(&list[i..j]); return j; }
    match g.fog_of_war.thing_recs.get(crate::Num(tk)) {
        Some(r) => { out.extend(r.ghost.iter().map(|&b| AnyBoxId::Frame(b))); g.fog_of_war.thing_stats.held += 1.0; }
        None => g.fog_of_war.thing_stats.out += 1.0,
    }
    j
}

/// Is it shown as it is: nobody has to look at one lying where the team last saw it, and one that has moved is shown once it is seen. Out of
/// sight, the record of it stays until the team looks where it was, or it was last seen moving.
fn thing_shows(g: &mut Game, look: Look, list: &[AnyBoxId], i: usize, j: usize, tk: f64) -> bool {
    let key = crate::Num(tk);
    let at = thing_at(g, list, i, j);
    let (x, z, ox, oz) = at;
    let mine = g.fog_of_war.our_chunks.len() > 0 && g.fog_of_war.our_chunks.has(key);
    let prime = look.prime && !(look.props_loaded && is_prop(tk));
    let Some(r) = g.fog_of_war.thing_recs.get_mut(key) else {
        if !(prime || mine || fog_of_war_thing_seen(g, x, z, ox, oz)) { return false; }
        thing_take(g, tk, list, i, j, at, look.f, !prime);
        g.fog_of_war.thing_recs.get_mut(key).unwrap().f = look.f;
        return true;
    };
    r.f = look.f;
    let (held, moving, was) = (r.held, r.moving, (r.x, r.z, r.ox, r.oz));
    let ghost = core::mem::take(&mut r.ghost);
    let same = thing_same(g, list, i, j, &ghost);
    let r = g.fog_of_war.thing_recs.get_mut(key).unwrap();
    r.ghost = ghost;
    if same {
        r.moving = false;
        r.held = false;
        return true;
    }
    if prime || mine || ((!held || look.turn(tk) == 0.0) && fog_of_war_thing_seen(g, x, z, ox, oz)) {
        thing_take(g, tk, list, i, j, at, look.f, true);
        return true;
    }
    let gone = if held { look.turn(tk) == 1.0 && fog_of_war_thing_seen(g, was.0, was.1, was.2, was.3) } else { moving || fog_of_war_thing_seen(g, was.0, was.1, was.2, was.3) };
    if gone { g.fog_of_war.thing_recs.delete(key); } else { g.fog_of_war.thing_recs.get_mut(key).unwrap().held = true; }
    false
}
/// The middle of a thing's boxes on the floor, and what it spans.
fn thing_at(g: &mut Game, list: &[AnyBoxId], i: usize, j: usize) -> (f64, f64, f64, f64) {
    let (mut x, mut z) = (0.0, 0.0);
    for &b in &list[i..j] { let c = g.tab[b].c; x += c[0]; z += c[2]; }
    fog_of_war_span(g, list, i as f64, j as f64);
    (x / (j - i) as f64, z / (j - i) as f64, g.fog_of_war.span_x, g.fog_of_war.span_z)
}

/// What has left the list altogether, out of sight: still in the picture where it was, until somebody looks there.
fn things_gone(g: &mut Game, look: Look, out: &mut Vec<AnyBoxId>) {
    let mut at = 0;
    while let Some(key) = g.fog_of_war.thing_recs.next_key(&mut at) {
        let r = g.fog_of_war.thing_recs.get(key).unwrap();
        if r.f == look.f { continue; }
        let (moving, held, x, z, ox, oz) = (r.moving, r.held, r.x, r.z, r.ox, r.oz);
        if moving || g.fog_of_war.thing_recs.len() as f64 > THING_CAP || ((look.turn(key.0) == 1.0 || !held) && fog_of_war_thing_seen(g, x, z, ox, oz)) {
            g.fog_of_war.thing_recs.delete(key);
            continue;
        }
        let r = g.fog_of_war.thing_recs.get_mut(key).unwrap();
        r.held = true;
        out.extend(r.ghost.iter().map(|&b| AnyBoxId::Frame(b)));
        g.fog_of_war.thing_stats.held += 1.0;
    }
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
        if age >= 0.0 && age < life { out.push(crate::view::SeenMark { man: g.char_id(gd), at, age, life }); }
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
