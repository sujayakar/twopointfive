//! Men's boots on pieces at rest, and on the men down the pieces lie over: what is shoved, which way, and the slide it makes of it.
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js;
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::vec::Vec;

/// How high above a man down's floor a piece may lie and still be on him (m): a man lying is that high, and a piece on a pile over him is on him all the same.
const BODY_OVER: f64 = 0.3;
/// How far from a bone of a man down a piece must lie (m): a limb's reach, where the rule for a man on his feet keeps a stance's.
const BODY_RADIUS: f64 = 0.18;
/// A piece a boot may move: its box on the floor, what it weighs, and the shoves it has had.
struct Lying {
    c: DebrisChunkId,
    b: AnyBoxId,
    at: V3,
    h: V3,
    f: Footprint,
    mass: f64,
    /// Seconds since the last shove, how many in a row, and the side the last one sent it.
    since: f64,
    tries: f64,
    side: Option<f64>,
}

/// The men a piece is met by.
struct MetBy {
    /// It lies in one of them at least, the way out of them all, and whether a man down is one.
    inside: bool,
    out: [f64; 2],
    on_body: bool,
    /// The bones of the men down at its height.
    bones: Vec<V3>,
    /// In nobody: the walking man whose leading foot it is most in the way of.
    best: Option<FootBest>,
}

/// The best man a piece is in the way of: footGap's answer, the man, his speed.
#[derive(Clone, Copy)]
struct FootBest { hit: DebrisFootGap, m: FootMan, sp: f64 }

/// A shove: which way, how fast, and where on the piece.
struct Shove {
    dir: [f64; 2],
    speed: f64,
    lever: [f64; 2],
}

/// Men's boots on pieces at rest: shoved ahead and aside, or out of a man they lie in; the shoves made.
pub(in crate::breakage) fn debris_feet(g: &mut Game, men: &[FootMan], k: FootKnobs, list: Option<&[DebrisChunkId]>, book: Option<&mut FootBook>) -> f64 {
    let list: Vec<DebrisChunkId> = match list { Some(l) => l.to_vec(), None => g.destr.debris.list.clone() };
    if list.is_empty() || men.is_empty() { return 0.0; }
    g.destr.debris.foot_mu = k.slide;
    let mut bk = match &book { Some(b) => **b, None => FootBook { shoves: g.destr.debris.shoves, stats: g.destr.debris.feet_stats } };
    let mut n = 0.0;
    for c in list {
        if n >= k.per_frame { break; }
        if shoved(g, men, &k, c, &mut bk) { n += 1.0; }
    }
    match book {
        Some(b) => *b = bk,
        None => (g.destr.debris.shoves, g.destr.debris.feet_stats) = (bk.shoves, bk.stats),
    }
    n
}

/// One piece: a man's boot finds it, and it goes, or counts the try.
fn shoved(g: &mut Game, men: &[FootMan], k: &FootKnobs, c: DebrisChunkId, bk: &mut FootBook) -> bool {
    let Some(since) = ready(g, c, k) else { return false };
    if !by_anybody(g, c, men, k) { return false; }
    let p = Lying::of(g, c, since);
    let by = p.met_by(g, men, k);
    if !by.inside && by.best.is_none() { return false; }
    // (a man down stays where he is: tried as often as a boot tries, it lies there for good, and nobody is left waiting to hear it scrape)
    if by.on_body && p.tries >= k.give_up { return false; }
    bk.shoves += 1.0;
    let nth = bk.shoves;
    let tries = if p.since > 2.0 && !by.on_body { 1.0 } else { p.tries + 1.0 };
    let ch = &mut g.tab.debris_chunk[c];
    ch.foot_at = Some(ch.age);
    ch.foot_n = Some(tries);
    if tries == k.give_up { bk.stats.gave_up += 1.0; }
    let way = if by.on_body {
        p.off_body(g, k, &by)
    } else if by.inside {
        p.off_man(k, &by)
    } else {
        Some(p.kicked(g, k, by.best.unwrap(), nth))
    };
    // (a way off a man down is free as found)
    let Some(way) = way.and_then(|w| if by.on_body { Some(w) } else { p.along_wall(g, w) }) else { bk.stats.walled += 1.0; return false };
    p.goes(g, k, &way, nth);
    if !by.on_body { p.scuffs(g, k, &way, nth); }
    if by.inside { bk.stats.off_man += 1.0; } else { bk.stats.kicked += 1.0; }
    true
}

/// A piece at rest, or only slid there by a boot, and not shoved a moment ago: the seconds since it was.
fn ready(g: &Game, c: DebrisChunkId, k: &FootKnobs) -> Option<f64> {
    let ch = &g.tab.debris_chunk[c];
    if !(ch.rest || is_on(ch.foot)) || ch.age <= 0.0 || ch.on.is_some() || ch.glass.is_some() { return None; }
    let since = ch.foot_at.map_or(m::INF, |at| ch.age - at);
    let wait = if ch.foot_n.unwrap_or(0.0) >= k.give_up { k.give_up_secs } else { k.cooldown };
    if since < wait { None } else { Some(since) }
}

/// No boot moves a heavy one; what lies on a man down goes whatever it weighs.
fn moves_it(mm: &FootMan, mass: f64, k: &FootKnobs) -> bool { mm.body || !(mass > k.heavy) }

/// What a box weighs.
fn box_mass(h: V3) -> f64 { h[0] * h[1] * h[2] * 8.0 * 600.0 }

/// Is anybody near enough for it to be worth asking.
fn by_anybody(g: &Game, c: DebrisChunkId, men: &[FootMan], k: &FootKnobs) -> bool {
    let row = &g.tab[g.tab.debris_chunk[c].box_];
    let (bh, bc) = (row.h, row.c);
    let mass = box_mass(bh);
    let far = bound_of(bh) + k.radius + k.reach + 0.05;
    men.iter().filter(|mm| moves_it(mm, mass, k)).any(|mm| {
        let d = far + m::hyp2(mm.vel[0], mm.vel[2]) * k.lead;
        m::abs(mm.pos[0] - bc[0]) < d && m::abs(mm.pos[2] - bc[2]) < d
    })
}

impl Lying {
    fn of(g: &Game, c: DebrisChunkId, since: f64) -> Lying {
        let b = g.tab.debris_chunk[c].box_;
        let f = footprint(g, b);
        let (ch, row) = (&g.tab.debris_chunk[c], &g.tab[b]);
        Lying { c, b, at: row.c, h: row.h, f, mass: box_mass(row.h), since, tries: ch.foot_n.unwrap_or(0.0), side: ch.foot_side }
    }

    /// The men it lies in, or, in nobody, the walking man it is most in the way of.
    fn met_by(&self, g: &Game, men: &[FootMan], k: &FootKnobs) -> MetBy {
        let (bottom, top) = (self.at[1] - self.f.hy, self.at[1] + self.f.hy);
        let mut by = MetBy { inside: false, out: [0.0, 0.0], on_body: false, bones: Vec::new(), best: None };
        for mm in men.iter().filter(|mm| moves_it(mm, self.mass, k)) {
            let fy = mm.pos[1];
            if bottom > fy + (if mm.body { BODY_OVER } else { k.floor }) || top < fy + 0.01 { continue; }
            if mm.body { by.bones.push(mm.pos); }
            let here = debris_foot_gap(g, self.b, mm.pos[0], mm.pos[2], &self.f, if mm.body { BODY_RADIUS } else { k.radius });
            if here.gap < -k.inside {
                by.out[0] -= here.dx * here.gap;
                by.out[1] -= here.dz * here.gap;
                by.inside = true;
                by.on_body |= mm.body;
                continue;
            }
            let sp = m::hyp2(mm.vel[0], mm.vel[2]);
            if by.inside || sp <= k.walk { continue; }
            let hit = debris_foot_gap(g, self.b, mm.pos[0] + mm.vel[0] * k.lead, mm.pos[2] + mm.vel[2] * k.lead, &self.f, k.radius);
            if hit.gap >= k.reach || (mm.vel[0] * hit.dx + mm.vel[2] * hit.dz) <= 0.2 * sp { continue; }
            if by.best.map_or(true, |x| hit.gap < x.hit.gap) { by.best = Some(FootBest { hit, m: *mm, sp }); }
        }
        by
    }

    /// Off a man down in the one shove: the speed that slides it clear of him against the floor's drag.
    fn off_body(&self, g: &mut Game, k: &FootKnobs, by: &MetBy) -> Option<Shove> {
        let (wx, wz, d) = way_off(g, self.b, &self.f, &by.bones)?;
        Some(Shove { dir: [wx, wz], speed: m::min(k.v_max, m::sqrt(2.0 * k.slide * G * d)), lever: [0.0, 0.0] })
    }

    /// Out of the men it lies in.
    fn off_man(&self, k: &FootKnobs, by: &MetBy) -> Option<Shove> {
        let [ox, oz] = by.out;
        let ol = m::hyp2(ox, oz);
        if ol < 0.02 { return None; }
        Some(Shove { dir: [ox / ol, oz / ol], speed: k.off * (0.6 + 0.4 * self.heft()), lever: [0.0, 0.0] })
    }

    /// Ahead of him and aside: off his line to the side it already lies on (hashed when it lies dead ahead).
    fn kicked(&self, g: &mut Game, k: &FootKnobs, by: FootBest, nth: f64) -> Shove {
        let FootBest { hit, m: man, sp } = by;
        let (vx, vz) = (man.vel[0] / sp, man.vel[2] / sp);
        let toward = vx * hit.dx + vz * hit.dz;
        let cr = vx * (self.at[2] - man.pos[2]) - vz * (self.at[0] - man.pos[0]);
        let side = match self.side {
            Some(fs) if self.since <= 2.0 && js::truthy(fs) => fs,
            _ if m::abs(cr) > 0.03 => m::sign(cr),
            _ => if luck(nth, 4.0) < 0.5 { -1.0 } else { 1.0 },
        };
        g.tab.debris_chunk[self.c].foot_side = Some(side);
        let (dx, dz) = (hit.dx - vz * side * k.aside, hit.dz + vx * side * k.aside);
        let dn = m::hyp2(dx, dz);
        let speed = m::min(k.v_max, k.base + k.gain * sp * m::max(0.3, toward)) * self.heft() * (0.8 + 0.4 * luck(nth, 5.0));
        Shove { dir: [dx / dn, dz / dn], speed, lever: [hit.lx, hit.lz] }
    }

    fn heft(&self) -> f64 { 1.0 / (1.0 + self.mass / 4.0) }

    /// Not into a wall: along it, or (square on) not at all; the try still counts.
    fn along_wall(&self, g: &Game, s: Shove) -> Option<Shove> {
        let [dx, dz] = s.dir;
        let Footprint { d1x, d1z, e1, e2, hy } = self.f;
        let reach = m::abs(dx * d1x + dz * d1z) * e1 + m::abs(dx * -d1z + dz * d1x) * e2 + 0.1;
        let bc = self.at;
        let Some(wall) = crate::scene_collision::raycast(g, [bc[0], m::max(bc[1], bc[1] - hy + 0.03), bc[2]], [dx, 0.0, dz], reach) else { return Some(s) };
        let dn = dx * wall.n[0] + dz * wall.n[2];
        let (tx, tz) = (dx - wall.n[0] * dn, dz - wall.n[2] * dn);
        let tl = m::hyp2(tx, tz);
        if tl < 0.25 { return None; }
        Some(Shove { dir: [tx / tl, tz / tl], speed: s.speed * tl, lever: s.lever })
    }

    /// Off it goes. A boot on one end turns it about its middle; a touch of its own so two planks do not slide as one.
    fn goes(&self, g: &mut Game, k: &FootKnobs, s: &Shove, nth: f64) {
        let ([dx, dz], [lx, lz], Footprint { e1, e2, .. }) = (s.dir, s.lever, self.f);
        let ch = &mut g.tab.debris_chunk[self.c];
        if ch.rest { ch.rest = false; ch.still_t = 0.0; }
        ch.foot = Some(true);
        ch.v = [dx * s.speed, 0.0, dz * s.speed];
        let wy = m::max(-5.0, m::min(5.0, k.spin * (lz * dx - lx * dz) * s.speed / m::max(0.01, e1 * e1 + e2 * e2)));
        ch.w = [0.0, wy + (luck(nth, 3.0) - 0.5) * 0.6 * s.speed / m::max(0.08, bound_of(self.h)), 0.0];
    }

    /// A scuff for the ear only (no hearing event: his steps already carry his noise).
    fn scuffs(&self, g: &mut Game, k: &FootKnobs, s: &Shove, nth: f64) {
        if !(k.scuff > 0.0) || !(g.time - g.destr.debris.scuff_at > 0.12) { return; }
        let Some(f) = g.tab.debris_chunk[self.c].sfx.first().cloned() else { return };
        g.destr.debris.scuff_at = g.time;
        let gain = m::min(1.0, k.scuff * f.gain * (0.6 + 0.6 * s.speed / m::max(0.1, k.v_max)));
        crate::audio_iface::game_audio_like_play(g, &f.name, Some(self.at), Some(gain), Some(&crate::audio_iface::PlayOpts::Rate { rate: f.rate * (1.15 + 0.2 * luck(nth, 7.0)) }));
    }
}

/// The shove's own luck: a hash of its number, not a die.
fn luck(nth: f64, j: f64) -> f64 { hash01(nth * 149.0 + j) }

/// A piece outside the pool, for the feet alone: a chunk record round its box, at rest.
pub(in crate::breakage) fn debris_loose(g: &mut crate::game::Game, box_: AnyBoxId, lift: Option<f64>) -> DebrisChunkId {
    let l = landing_of(ChunkKind::Chunk);
    g.tab.debris_chunk.push(Chunk {
        kind: ChunkKind::Chunk, box_, lift: Some(lift.unwrap_or(0.0)), v: [0.0, 0.0, 0.0], w: [0.0, 0.0, 0.0], rest: true, still_t: 0.0, age: 0.0,
        sounded: true, level: l.level, sfx: sfx_of(&l), serial: 0.0, who: crate::consts::NOBODY_ID, on: None, on_pose: 0.0,
        ..Chunk::default()
    })
}

/// A loose piece's frame: older by `dt`, and slid on if a boot set it going.
pub(in crate::breakage) fn debris_foot_step(g: &mut crate::game::Game, c_2: DebrisChunkId, dt: f64) {
    let c = c_2;
    g.tab.debris_chunk[c].age += dt;
    if g.tab.debris_chunk[c].rest { return; }
    if !is_on(g.tab.debris_chunk[c].foot) { g.tab.debris_chunk[c].rest = true; return; }
    let sub = 2;
    let h = dt / sub as f64;
    let mut s = 0;
    while s < sub && !g.tab.debris_chunk[c].rest { debris_slide(g, c, h); s += 1; }
}

/// The shortest way off the men down for a piece that lies on one, of sixteen, that it would slide free: its direction, and how far. A way is as long as it takes to come clear of every bone in the piece's
/// path that way, not only of those it lies on now: so a slide leaves less to slide, and the next is never back over him.
fn way_off(g: &mut Game, b: AnyBoxId, f: &Footprint, bones: &[V3]) -> Option<(f64, f64, f64)> {
    let bc = g.tab[b].c;
    let span = |x: f64, z: f64| m::abs(x * f.d1x + z * f.d1z) * f.e1 + m::abs(x * -f.d1z + z * f.d1x) * f.e2;
    let mut ways: Vec<(f64, f64, f64)> = (0..16).map(|i| {
        let (c, s) = (m::cos(i as f64 * m::PI / 8.0), m::sin(i as f64 * m::PI / 8.0));
        let (wx, wz) = (f.d1x * c - f.d1z * s, f.d1x * s + f.d1z * c);
        let (half, across) = (span(wx, wz) + BODY_RADIUS, span(-wz, wx) + BODY_RADIUS);
        let mut path: Vec<f64> = bones.iter().map(|b| [b[0] - bc[0], b[2] - bc[2]]).filter(|r| m::abs(r[1] * wx - r[0] * wz) < across).map(|r| r[0] * wx + r[1] * wz).collect();
        path.sort_by(|a, b| a.total_cmp(b));
        (wx, wz, path.into_iter().fold(0.0, |d, t| if m::abs(t - d) < half { t + half } else { d }))
    }).collect();
    ways.sort_by(|a, b| a.2.total_cmp(&b.2));
    ways.into_iter().find(|&(wx, wz, d)| slides_free(g, b, f, [wx, wz], d))
}

/// Would `debris_slide` carry the piece `d` that way: no static, leaf or furniture before its middle, a floor under it all along, and no corner of it in a static it is clear of now.
fn slides_free(g: &mut Game, b: AnyBoxId, f: &Footprint, way: [f64; 2], d: f64) -> bool {
    let c0 = g.tab[b].c;
    let from = [c0[0], m::max(c0[1], c0[1] - f.hy + 0.05), c0[2]];
    let (ray, reach) = ([way[0], 0.0, way[1]], d + m::abs(way[0] * f.d1x + way[1] * f.d1z) * f.e1 + m::abs(way[1] * f.d1x - way[0] * f.d1z) * f.e2);
    if crate::scene_collision::raycast(g, from, ray, reach).is_some() || crate::props::prop_system_raycast(g, from, ray, reach).is_some() { return false; }
    let clear = !debris_foot_in(g, b);
    let n = m::max(1.0, m::ceil(d / 0.15));
    let mut free = true;
    let mut k = 1.0;
    while free && k <= n {
        let c = [c0[0] + way[0] * d * k / n, c0[1], c0[2] + way[1] * d * k / n];
        g.tab.edit(b).c = c;
        free = crate::scene_collision::raycast(g, [c[0], c[1] + 0.05, c[2]], [0.0, -1.0, 0.0], 0.05 + f.hy + 0.3).is_some() && !(clear && debris_foot_in(g, b));
        k += 1.0;
    }
    g.tab.edit(b).c = c0;
    free
}

/// A man's circle against a piece's footprint: the gap, the way out, the boot's lever.
fn debris_foot_gap(g: &Game, b: AnyBoxId, px: f64, pz: f64, f: &Footprint, radius: f64) -> DebrisFootGap {
    let Footprint { d1x, d1z, e1, e2, .. } = *f;
    let bc = g.tab[b].c;
    let (d2x, d2z) = (-d1z, d1x);
    let (rx, rz) = (px - bc[0], pz - bc[2]);
    let u = rx * d1x + rz * d1z;
    let v = rx * d2x + rz * d2z;
    let cu = m::max(-e1, m::min(e1, u));
    let cv = m::max(-e2, m::min(e2, v));
    if cu == u && cv == v {
        // his middle stands over it: out by the nearer side, away from him
        let pu = e1 - m::abs(u);
        let pv = e2 - m::abs(v);
        if pu < pv {
            let s = if u > 0.0 { -1.0 } else { 1.0 };
            return DebrisFootGap { gap: -(pu + radius), dx: d1x * s, dz: d1z * s, lx: 0.0, lz: 0.0 };
        }
        let s = if v > 0.0 { -1.0 } else { 1.0 };
        return DebrisFootGap { gap: -(pv + radius), dx: d2x * s, dz: d2z * s, lx: 0.0, lz: 0.0 };
    }
    let (ou, ov) = (u - cu, v - cv);
    let d = m::hyp2(ou, ov);
    DebrisFootGap { gap: d - radius, dx: -(ou * d1x + ov * d2x) / d, dz: -(ou * d1z + ov * d2z) / d, lx: cu * d1x + cv * d2x, lz: cu * d1z + cv * d2z }
}

/// Where a piece was, to be put back there.
struct Was { c: V3, rot: Option<Quat>, yaw: f64 }

impl Was {
    fn of(g: &Game, b: AnyBoxId) -> Was { let row = &g.tab[b]; Was { c: row.c, rot: row.rot, yaw: row.yaw } }
    fn put_back(&self, g: &mut Game, b: AnyBoxId) {
        let mut row = g.tab.edit(b);
        (row.c, row.rot, row.yaw) = (self.c, self.rot, self.yaw);
    }
}

/// A boot's piece for `h` seconds: a slider on the floor it lay on.
pub(super) fn debris_slide(g: &mut Game, c: DebrisChunkId, h: f64) {
    let ch = &g.tab.debris_chunk[c];
    let (b, sp) = (ch.box_, m::hyp2(ch.v[0], ch.v[2]));
    if sp < SLEEP_V * 0.25 && m::abs(ch.w[1]) < 0.5 { return stops(g, c, b); }
    let was = Was::of(g, b);
    let in_wall = debris_foot_in(g, b);
    turns(g, c, b, h);
    let sup = support_along(g, b, UP);
    if sp > 1e-4 { slides_on(g, c, b, sp, sup, h); }
    let bc = g.tab[b].c;
    let Some(fl) = crate::scene_collision::raycast(g, [bc[0], was.c[1] + 0.05, bc[2]], [0.0, -1.0, 0.0], 0.05 + sup + 0.3) else {   // the floor ends here: it stops where it was
        was.put_back(g, b);
        let ch = &mut g.tab.debris_chunk[c];
        ch.v = [0.0, 0.0, 0.0];
        ch.w = [0.0, 0.0, 0.0];
        return;
    };
    // (it goes at the height it lay at, off a pile, and comes down to the floor by a fall's pace and not at a stroke)
    let floor = was.c[1] + 0.05 - fl.t + sup + 0.0015 + g.tab.debris_chunk[c].lift.unwrap_or(0.0);
    box_at_y(g, b, m::max(floor, m::min(was.c[1], bc[1]) - G * h * 0.25));
    if !in_wall && debris_foot_in(g, b) {
        was.put_back(g, b);
        let ch = &mut g.tab.debris_chunk[c];
        ch.w[1] = 0.0;
        ch.v[0] *= 0.5;
        ch.v[2] *= 0.5;
    }
    let mu = g.destr.debris.foot_mu;
    let ch = &mut g.tab.debris_chunk[c];
    let left = m::hyp2(ch.v[0], ch.v[2]);
    let keep = if left < 1e-4 { 0.0 } else { m::max(0.0, 1.0 - mu * G * h / left) };
    ch.v = [ch.v[0] * keep, 0.0, ch.v[2] * keep];
}

/// Stopped: asleep, laid flat, unless lying down would put a corner of it into a wall it is clear of; then tipped as it was, on what lies there.
fn stops(g: &mut Game, c: DebrisChunkId, b: AnyBoxId) {
    let clear = !debris_foot_in(g, b);
    let was = Was::of(g, b);
    debris_sleep(g, c, UP, None);
    if !clear || !debris_foot_in(g, b) { return; }
    was.put_back(g, b);
    if let Some(y) = debris_seat(g, c, false).0 { box_at_y(g, b, y); }
    let ch = &mut g.tab.debris_chunk[c];
    ch.rest = true;
    ch.foot = Some(false);
    ch.v = [0.0, 0.0, 0.0];
    ch.w = [0.0, 0.0, 0.0];
}

/// Laid flatter as it goes, and turned by what is left of its spin.
fn turns(g: &mut Game, c: DebrisChunkId, b: AnyBoxId, h: f64) {
    if face_tilt(g, b, UP) > 1e-3 { lay_flat(g, b, UP, 0.2); }
    let w = g.tab.debris_chunk[c].w[1];
    if !(m::abs(w) > 1e-4) { return; }
    let a = w * h;
    let r = rot_of(g, b);
    g.tab.edit(b).rot = Some(v3::quat::normalize(v3::quat::mul([0.0, m::sin(a / 2.0), 0.0, m::cos(a / 2.0)], r)));
    g.tab.debris_chunk[c].w[1] *= m::max(0.0, 1.0 - 5.0 * h);
}

/// On over the floor: up to what is in the way, then along it with what is left of its speed.
fn slides_on(g: &mut Game, c: DebrisChunkId, b: AnyBoxId, sp: f64, sup: f64, h: f64) {
    let v = g.tab.debris_chunk[c].v;
    let (dx, dz) = (v[0] / sp, v[2] / sp);
    let d = sp * h;
    let ahead = support_along(g, b, [dx, 0.0, dz]);
    let bc = g.tab[b].c;
    let hit = crate::props::level_ray(g, [bc[0], m::max(bc[1], bc[1] - sup + 0.05), bc[2]], [dx, 0.0, dz], d + ahead);
    let go = hit.map_or(d, |h| m::max(0.0, h.t - ahead));
    g.tab.edit(b).c = [bc[0] + dx * go, bc[1], bc[2] + dz * go];
    let Some(hit) = hit else { return };
    let ch = &mut g.tab.debris_chunk[c];
    let vn = ch.v[0] * hit.n[0] + ch.v[2] * hit.n[2];
    if vn < 0.0 {
        ch.v[0] = (ch.v[0] - hit.n[0] * vn * 1.1) * 0.7;
        ch.v[2] = (ch.v[2] - hit.n[2] * vn * 1.1) * 0.7;
    }
}
