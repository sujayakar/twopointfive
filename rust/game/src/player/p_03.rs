//! src/game/player.ts lines 834 to 1281
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- a seam: the one step the skeleton still cannot give (notes.md, ## Skeleton)
/// g.audio.play(name, at, gain, { rate }): opts is a JsMap of Opaque, which cannot hold the rate, and an absent rate is a random one (audio.ts)
fn seam_audio_play(g: &mut Game, name: &str, at: V3, gain: f64, rate: f64) { todo!("needs: game_audio_like_play's opts to carry a rate") }

// ---- the body keepOffGuards keeps off: the player's own (None) or a look ahead's copy (Some)
fn legs_pos(g: &Game, m: &Option<&mut Legs>) -> V3 {
    match m { Some(l) => l.char.pos, None => g.ch[g.pl[g.player].ch].pos }
}
fn legs_set_xz(g: &mut Game, m: &mut Option<&mut Legs>, x: f64, z: f64) {
    match m {
        Some(l) => { l.char.pos[0] = x; l.char.pos[2] = z; }
        None => { let c = g.pl[g.player].ch; g.ch[c].pos[0] = x; g.ch[c].pos[2] = z; }
    }
}
fn legs_set_safe_xz(g: &mut Game, m: &mut Option<&mut Legs>, x: f64, z: f64) {
    let pl = g.player;
    let s = match m { Some(l) => l.safe.as_mut(), None => g.pl[pl].safe.as_mut() };
    if let Some(s) = s { s[0] = x; s[2] = z; }
}
fn legs_shoved(l: &Legs) -> crate::noshove::Shoved {
    crate::noshove::Shoved { char: crate::noshove::ShovedChar { pos: vec![l.char.pos[0], l.char.pos[1], l.char.pos[2]] }, shoved: l.shoved, shoved_t: l.shoved_t, shoved_from: l.shoved_from }
}
fn legs_no_shove_back(g: &mut Game, mx: f64, mz: f64, l: f64, m: &Option<&mut Legs>) -> f64 {
    match m {
        Some(legs) => { let s = legs_shoved(legs); crate::noshove::no_shove_back(g, mx, mz, l, Some(&s)) }
        None => crate::noshove::no_shove_back(g, mx, mz, l, None),
    }
}
fn legs_no_shove_note(g: &mut Game, dx: f64, dz: f64, m: &mut Option<&mut Legs>) {
    match m {
        Some(legs) => {
            let mut s = legs_shoved(legs);
            crate::noshove::no_shove_note(g, dx, dz, Some(&mut s));
            legs.shoved = s.shoved; legs.shoved_t = s.shoved_t; legs.shoved_from = s.shoved_from;
        }
        None => crate::noshove::no_shove_note(g, dx, dz, None),
    }
}

/// collideMan on either body: collide_man takes a shape of its own, so the two fields it reads and writes go in and come back
fn legs_collide(g: &mut Game, m: &mut Option<&mut Legs>, y1: f64) {
    let (pos, safe) = match m {
        Some(l) => (l.char.pos, l.safe),
        None => (g.ch[g.pl[g.player].ch].pos, g.pl[g.player].safe),
    };
    let mut body = CollideManPl { char: CollideManPlChar { pos }, safe };
    collide_man(g, &mut body, y1);
    match m {
        Some(l) => { l.char.pos = body.char.pos; l.safe = body.safe; }
        None => { let pl = g.player; let c = g.pl[pl].ch; g.ch[c].pos = body.char.pos; g.pl[pl].safe = body.safe; }
    }
}

/// The cone Sam's next rifle round is drawn with for how he stands. (player.ts rifleSpread)
pub fn rifle_spread(g: &mut crate::game::Game, speed: f64) -> f64 {
    let pl = g.player;
    let gun = g.pl[pl].rifle;
    let def = g.fa[gun].def;
    let h = g.m.weapons.weapons.get(def).and_then(|d| d.hands);
    let feet = if !g.touring { m::max(speed, g.pl[pl].aim.move_sm) } else { speed };
    let braced = crate::game::player_braced(g, pl);
    let (still, moving, move_at, crouch_k, brace_k) = (g.k.rifle_spread.still, g.k.rifle_spread.moving, g.k.rifle_spread.move_at, g.k.rifle_spread.crouch_k, g.k.rifle_spread.brace_k);
    let h_moving = match h { Some(r) => r.moving, None => 1.0 };
    let h_still = match h { Some(r) => r.still, None => 1.0 };
    let h_crouch = match h { Some(r) => r.crouch.unwrap_or(1.0), None => 1.0 };
    (if feet > move_at { moving * h_moving } else { still * h_still })
        * (if g.pl[pl].crouch { crouch_k * h_crouch } else { 1.0 })
        * (if braced { brace_k } else { 1.0 })
}

/// how far your centre can travel that way before your circle meets a static or a leaf at shin height (keepOffGuards `room`)
fn room(g: &mut Game, p: V3, sx: f64, sz: f64) -> f64 {
    let hit = crate::scene_collision::raycast(g, [p[0], 0.45, p[2]], [sx, 0.0, sz], 1.5);
    (match hit { Some(h) => h.t, None => 1.5 }) - BODY_R
}

/// Living guards are solid to you and never step aside: you are the one who yields, every shove collided again. (player.ts keepOffGuards)
pub fn keep_off_guards(g: &mut crate::game::Game, dt: f64, y1: f64, m: Option<&mut crate::player::Legs>) {
    let mut m = m;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if g.gd[gd].state == crate::game::GuardState::Dead || g.gd[gd].held.is_some() { continue; }
        let p = legs_pos(g, &m);
        let gc = g.gd[gd].ch;
        let (gp, gv) = (g.ch[gc].pos, g.ch[gc].vel);
        let (dx, dz) = (p[0] - gp[0], p[2] - gp[2]);
        let l = m::hyp2(dx, dz);
        if l >= KEEP_OFF { continue; }
        if crate::noshove::no_shove_man(g, gd) {
            let back = if l > 1e-4 { legs_no_shove_back(g, gp[0], gp[2], l, &m) } else { 0.0 };
            if back > 0.0 {
                let (bx, bz) = (p[0], p[2]);
                legs_set_xz(g, &mut m, p[0] + dx / l * back, p[2] + dz / l * back);
                legs_collide(g, &mut m, y1);
                let q = legs_pos(g, &m);
                legs_no_shove_note(g, q[0] - bx, q[2] - bz, &mut m);
            }
            continue;
        }
        let gs = m::hyp2(gv[0], gv[2]);
        let walking = gs > 0.2;
        let (mut nx, mut nz);
        if l > 1e-4 { nx = dx / l; nz = dz / l; } else if gs > 1e-3 { nx = gv[2] / gs; nz = -gv[0] / gs; } else { nx = 1.0; nz = 0.0; }
        let (mut cap, mut fx, mut fz, mut ahead) = (6.0, 0.0, 0.0, 0.0);
        if walking {
            let wp = if g.gd[gd].path_i < g.gd[gd].path.len() { Some(g.gd[gd].path[g.gd[gd].path_i]) } else { None };
            let wx = match wp { Some(w) => w[0] - gp[0], None => 0.0 };
            let wz = match wp { Some(w) => w[2] - gp[2], None => 0.0 };
            let wl = m::hyp2(wx, wz);
            if wl > 0.3 { fx = wx / wl; fz = wz / wl; } else { fx = gv[0] / gs; fz = gv[2] / gs; }
            ahead = nx * fx + nz * fz;
            if ahead > 0.64 {
                let (mut sx, mut sz) = (nx - ahead * fx, nz - ahead * fz);
                let sl = m::hyp2(sx, sz);
                let mut b = l * sl;
                if b >= 0.04 { sx /= sl; sz /= sl; } else { sx = fz; sz = -fx; b = 0.0; }
                let near = b + room(g, p, sx, sz);
                let far = room(g, p, -sx, -sz) - b;
                if near < KEEP_OFF && (far >= KEEP_OFF || far > near + 0.25) {
                    if far < 0.25 { continue; }
                    sx = -sx; sz = -sz;
                } else if near < 0.25 { continue; }
                nx = sx; nz = sz; cap = 4.0; ahead = 0.0;
            }
        }
        let push = m::min(KEEP_OFF - l, cap * dt);
        let (bx, bz) = (p[0], p[2]);
        let (mut qx, mut qz) = (p[0] + nx * push, p[2] + nz * push);
        if ahead > 0.0 {
            let his = ahead * m::min(push, gs * dt * ahead);
            qx -= fx * his; qz -= fz * his;
        }
        legs_set_xz(g, &mut m, qx, qz);
        legs_collide(g, &mut m, y1);
        let q = legs_pos(g, &m);
        if walking && (q[0] - bx) * nx + (q[2] - bz) * nz < 0.5 * push {
            legs_set_xz(g, &mut m, bx, bz);
            legs_set_safe_xz(g, &mut m, bx, bz);
        }
        let q = legs_pos(g, &m);
        legs_no_shove_note(g, q[0] - bx, q[2] - bz, &mut m);
    }
}

/// a level ray straight into the plane from the point (ox, y, oz), if it meets a face within `tol` of it, facing back out along n (faceAt `cast`)
fn face_cast(g: &mut Game, ox: f64, oz: f64, n: V3, y: f64, tol: f64) -> Option<crate::scene_collision::Raycast> {
    const OFF: f64 = 0.3;
    let dir: V3 = [-n[0], 0.0, -n[2]];
    let h = crate::scene_collision::raycast(g, [ox, y, oz], dir, OFF + tol + 0.02)?;
    if h.index < 1e6 && m::abs(h.t - OFF) <= tol && h.n[0] * n[0] + h.n[2] * n[2] > 0.98 { Some(h) } else { None }
}

/// The static face lying in the plane (n, d) at station u along t, if fit to lean on: that knee-height box's extent along t. (player.ts faceAt)
pub fn face_at(g: &mut crate::game::Game, n: V3, d: f64, t: V3, u: f64) -> Option<[f64; 2]> {
    const OFF: f64 = 0.3;
    let (ox, oz) = (t[0] * u + n[0] * (d + OFF), t[2] * u + n[2] * (d + OFF));
    let knee = face_cast(g, ox, oz, n, KNEE_Y, COPLANAR)?;
    if face_cast(g, ox, oz, n, CHEST_Y, UPPER_TOL).is_none() && face_cast(g, ox, oz, n, HIGH_Y, UPPER_TOL).is_none() { return None; }
    let b = &g.col.boxes[BoxId::from_ix(knee.index as usize)];
    let (c, h, yaw) = (b.c, b.h, b.yaw);
    let (cs, sn) = (m::cos(yaw), m::sin(yaw));
    let hw = m::abs(t[0] * cs - t[2] * sn) * h[0] + m::abs(t[0] * sn + t[2] * cs) * h[2];
    let uc = c[0] * t[0] + c[2] * t[2];
    Some([uc - hw, uc + hw])
}

/// The whole run of wall through station u0, or only `look` either side of it. (player.ts traceFace)
pub fn trace_face(g: &mut crate::game::Game, n: V3, d: f64, t: V3, u0: f64, look: Option<f64>) -> Option<[f64; 2]> {
    let look = look.unwrap_or(m::INF);
    let mut seg: Option<[f64; 2]> = None;
    for du in [0.0, -0.21, 0.21, -BODY_R, BODY_R] {
        seg = face_at(g, n, d, t, u0 + du);
        if seg.is_some() { break; }
    }
    let [mut lo, mut hi] = seg?;
    let mut i = 0;
    while i < 32 && hi < u0 + look {
        let nx = match face_at(g, n, d, t, hi + 0.01) { Some(s) => Some(s), None => face_at(g, n, d, t, hi + WALL_BRIDGE) };
        let Some(nx) = nx else { break };
        if nx[1] <= hi + 1e-4 { break; }
        hi = nx[1];
        i += 1;
    }
    let mut i = 0;
    while i < 32 && lo > u0 - look {
        let nx = match face_at(g, n, d, t, lo - 0.01) { Some(s) => Some(s), None => face_at(g, n, d, t, lo - WALL_BRIDGE) };
        let Some(nx) = nx else { break };
        if nx[0] >= lo - 1e-4 { break; }
        lo = nx[0];
        i += 1;
    }
    Some([lo, hi])
}

/// Is the way on past an end of the face open (an outside corner, a doorway) or shut off? (player.ts endOpen)
pub fn end_open(g: &mut crate::game::Game, n: V3, d: f64, t: V3, u_end: f64, s: f64) -> bool {
    let (px, pz) = (t[0] * u_end + n[0] * (d + BODY_R + WALL_GAP), t[2] * u_end + n[2] * (d + BODY_R + WALL_GAP));
    let dir: V3 = [t[0] * s, 0.0, t[2] * s];
    for y in [KNEE_Y, 1.0] {
        if crate::scene_collision::raycast(g, [px, y, pz], dir, BODY_R + 0.2).is_some() { return false; }
    }
    true
}

/// Push the circle (q, r) out of the loose furniture as it stands. True if anything moved it. (player.ts clearOfProps)
pub fn clear_of_props(g: &mut crate::game::Game, q: &mut V3, r: f64) -> bool {
    let mut moved = false;
    for _it in 0..3 {
        let mut any = false;
        let mut i = 0;
        while i < g.props.props.len() {
            let pr = g.props.props[i];
            i += 1;
            let (px, pz) = (crate::props::prop_x(g, pr), crate::props::prop_z(g, pr));
            let (radius, hx, hz) = (g.tab.props_prop[pr].radius, g.tab.props_prop[pr].hx, g.tab.props_prop[pr].hz);
            let (dx, dz) = (q[0] - px, q[2] - pz);
            if dx * dx + dz * dz > (radius + r) * (radius + r) { continue; }
            let (cs, sn) = (crate::props::prop_cs(g, pr), crate::props::prop_sn(g, pr));
            let (lx, lz) = (cs * dx - sn * dz, sn * dx + cs * dz);
            let (qx, qz) = (v3::clamp(lx, -hx, hx), v3::clamp(lz, -hz, hz));
            let (ex, ez) = (lx - qx, lz - qz);
            let d2 = ex * ex + ez * ez;
            if d2 >= r * r { continue; }
            let (nx, nz, pen);
            if d2 > 1e-10 {
                let d = m::sqrt(d2);
                nx = ex / d; nz = ez / d; pen = r - d;
            } else {
                let (ax, az) = (hx - m::abs(lx), hz - m::abs(lz));
                if ax < az { nx = if lx < 0.0 { -1.0 } else { 1.0 }; nz = 0.0; pen = ax + r; }
                else { nx = 0.0; nz = if lz < 0.0 { -1.0 } else { 1.0 }; pen = az + r; }
            }
            q[0] += (cs * nx + sn * nz) * pen;
            q[2] += (-sn * nx + cs * nz) * pen;
            any = true; moved = true;
        }
        if !any { break; }
    }
    moved
}

struct WallCand { n: V3, d: f64, score: f64 }

/// What Q would take hold of from here: the nearest fit face in reach with room for the body. (player.ts findWall)
pub fn find_wall(g: &mut crate::game::Game, quick: Option<bool>, why: Option<&mut crate::player::FindWallWhy>, only: Option<V3>) -> Option<crate::player::FindWall> {
    let quick = quick.unwrap_or(false);
    let mut why = why;
    let pl = g.player;
    let c = g.pl[pl].ch;
    let p = g.ch[c].pos;
    let vel = g.ch[c].vel;
    let f = crate::character::character_forward(g, c);
    let sp = m::hyp2(vel[0], vel[2]);
    let far = g.k.wallsnap.on && sp > 0.5;
    let reach = if far { m::max(WALL_REACH, g.k.wallsnap.reach_moving) } else { WALL_REACH };
    let mut cands: Vec<WallCand> = Vec::new();
    for k in 0..16 {
        let a = (k as f64) * m::PI / 8.0;
        let dir: V3 = [m::sin(a), 0.0, m::cos(a)];
        let Some(hit) = crate::scene_collision::raycast(g, [p[0], KNEE_Y, p[2]], dir, BODY_R + reach) else { continue };
        if hit.index >= 1e6 || m::abs(hit.n[1]) > 0.2 { continue; }
        let n = v3::normalize([hit.n[0], 0.0, hit.n[2]]);
        if let Some(o) = only { if n[0] * o[0] + n[2] * o[2] < 0.99 { continue; } }
        if n[0] * dir[0] + n[2] * dir[2] > -0.35 { continue; }
        let d = (p[0] + dir[0] * hit.t) * n[0] + (p[2] + dir[2] * hit.t) * n[2];
        let dist = p[0] * n[0] + p[2] * n[2] - d;
        let heading = if far { -(vel[0] * n[0] + vel[2] * n[2]) / sp } else { 0.0 };
        let at = heading > g.k.wallsnap.heading_min;
        if dist < BODY_R - 0.06 || dist > BODY_R + (if at { reach } else { WALL_REACH }) { continue; }
        if let Some(dup) = cands.iter_mut().find(|c| c.n[0] * n[0] + c.n[2] * n[2] > 0.99 && m::abs(c.d - d) < 0.02) {
            if d < dup.d { dup.d = d; }
            continue;
        }
        let score = dist - 0.12 * m::max(0.0, -(f[0] * n[0] + f[2] * n[2])) - (if at { g.k.wallsnap.heading_bonus * heading } else { 0.0 });
        cands.push(WallCand { n, d, score });
    }
    js::sort_by(&mut cands, |a, b| a.score - b.score);
    let y1 = if g.pl[pl].crouch { 1.0 } else { 1.7 };
    for cand in &cands {
        let n = cand.n;
        let t: V3 = [-n[2], 0.0, n[0]];
        let u0 = p[0] * t[0] + p[2] * t[2];
        let Some([lo, hi]) = trace_face(g, n, cand.d, t, u0, Some(if quick { 1.0 } else { m::INF })) else { continue };
        if hi - lo < WALL_MIN_FACE { continue; }
        let u = v3::clamp(u0, lo + BODY_R, hi - BODY_R);
        if m::abs(u - u0) > 0.35 { continue; }
        let mut q: V3 = [t[0] * u + n[0] * (cand.d + BODY_R + WALL_GAP), 0.0, t[2] * u + n[2] * (cand.d + BODY_R + WALL_GAP)];
        crate::scene_collision::collide_circle(g, &mut q, BODY_R, 0.2, y1, Some(4.0));
        crate::scene_collision::clamp_to_world(g, &mut q, BODY_R);
        let by_props = clear_of_props(g, &mut q, PROP_BODY_R + 0.02);
        let off = q[0] * n[0] + q[2] * n[2] - (cand.d + BODY_R + WALL_GAP);
        let uq = q[0] * t[0] + q[2] * t[2];
        if m::abs(off) > 0.1 || uq < lo + BODY_R - 0.02 || uq > hi - BODY_R + 0.02 || m::abs(uq - u0) > 0.4 {
            if by_props { if let Some(w) = why.as_deref_mut() { w.blocked = true; } }
            continue;
        }
        if v3::dist_xz(p, q) > 0.05 && crate::scene_collision::solid_blocked(g, [p[0], KNEE_Y, p[2]], [q[0], KNEE_Y, q[2]], None) { continue; }
        return Some(FindWall { n, t, d: cand.d, lo, hi, u: v3::clamp(uq, lo + BODY_R, hi - BODY_R) });
    }
    None
}

/// Is he free to put his back to a wall, and would Q find one from where he stands? (player.ts canWallSnap)
pub fn can_wall_snap(g: &mut crate::game::Game) -> bool {
    let pl = g.player;
    let p = &g.pl[pl];
    if p.wall.is_some() || p.down || p.dragging.is_some() || p.takedown.is_some() || p.holding.is_some() || p.kick.is_some() || p.picking.is_some() || p.wanding.is_some() || p.sprinting { return false; }
    find_wall(g, Some(true), None, None).is_some()
}

/// where a sole is drawn this moment (wallSnap `ft`)
fn foot_at(a: Option<V3>, toe: Option<V3>) -> Option<(V3, f64)> {
    match (a, toe) {
        (Some(a), Some(toe)) => Some((v3::copy(a), m::atan2(toe[0] - a[0], toe[2] - a[2]))),
        _ => None,
    }
}

/// Q: take hold of the wall findWall offers; moveOnWall eases him onto it. False = nothing in reach, or his hands or feet are busy. (player.ts wallSnap)
pub fn wall_snap(g: &mut crate::game::Game, why: Option<&mut crate::player::WallSnapWhy>, only: Option<V3>) -> bool {
    let pl = g.player;
    let c = g.pl[pl].ch;
    {
        let p = &g.pl[pl];
        if p.wall.is_some() || p.down || p.dragging.is_some() || p.takedown.is_some() || p.holding.is_some() || p.kick.is_some() || p.picking.is_some() || p.wanding.is_some() { return false; }
    }
    // one object in the TypeScript, two shapes in the skeleton: findWall's flag comes back to the caller's, whatever it found
    let mut fw = why.as_deref().map(|w| FindWallWhy { blocked: w.blocked });
    let found = find_wall(g, Some(false), fw.as_mut(), only);
    if let (Some(w), Some(fw)) = (why, fw) { w.blocked = fw.blocked; }
    let Some(f) = found else { return false };
    let (n, t) = (f.n, f.t);
    let vel = g.ch[c].vel;
    let body_yaw = g.ch[c].body_yaw;
    let vt = vel[0] * t[0] + vel[2] * t[2];
    let ft = m::sin(body_yaw) * t[0] + m::cos(body_yaw) * t[2];
    let rt = crate::camera::follow_camera_planar_basis(g).right;
    let side: f64 = if m::abs(vt) > 0.3 { if vt > 0.0 { 1.0 } else { -1.0 } }
        else if m::abs(ft) > 0.4 { if ft > 0.0 { 1.0 } else { -1.0 } }
        else if rt[0] * t[0] + rt[2] * t[2] >= 0.0 { 1.0 } else { -1.0 };
    let (u_lo, u_hi) = (f.lo + BODY_R, f.hi - BODY_R);
    let open_lo = end_open(g, n, f.d, t, u_lo, -1.0);
    let open_hi = end_open(g, n, f.d, t, u_hi, 1.0);
    g.pl[pl].wall = Some(WallHold {
        n, along: t, d: f.d, lo: f.lo, hi: f.hi, open_lo, open_hi,
        u: f.u, v: 0.0, side, off: 0.0, away_t: 0.0, settle: 0.0, from: v3::copy(g.ch[c].pos), block_dir: 0.0, block_t: 0.0, seated: false,
        drawn: !g.pl[pl].holstered, at_edge: 0.0, peek: 0.0, peek_dir: side, crouched: g.pl[pl].crouch,
        ..WallHold::zeroed()
    });
    let stack = g.k.wallsnap.on && g.k.wallsnap.stack && crate::game::player_rifle_up(g, pl);
    if g.k.wallsnap.on {
        let away_yaw = m::atan2(n[0], n[2]);
        let base = f.d + BODY_R + WALL_GAP;
        let mut yd = v3::wrap_angle((if stack { away_yaw - side * m::PI / 2.0 } else { away_yaw }) - body_yaw);
        if !stack && m::abs(yd) > m::PI - 0.3 { yd = side * m::abs(yd); }
        let pos = g.ch[c].pos;
        let take_t = crate::wallsnap::ws_take_secs(g, m::hyp2(t[0] * f.u + n[0] * base - pos[0], t[2] * f.u + n[2] * base - pos[2]), yd);
        if let Some(w) = g.pl[pl].wall.as_mut() {
            w.yaw0 = Some(body_yaw); w.yaw_d = Some(yd); w.hit_t = Some(0.0);
            w.take_t = Some(take_t);
        }
        {
            let b = &g.ch[c].bones;
            let bone = |k: crate::character::Bone| b.get(k).copied();
            let l = foot_at(bone(crate::character::Bone::FootL), bone(crate::character::Bone::ToeL));
            let r = foot_at(bone(crate::character::Bone::FootR), bone(crate::character::Bone::ToeR));
            let feet = match (l, r) {
                (Some(l), Some(r)) => Some(crate::character::AnimWallTakeFeet {
                    l: crate::character::AnimWallTakeFeetL { p: l.0, yaw: l.1 },
                    r: crate::character::AnimWallTakeFeetR { p: r.0, yaw: r.1 },
                }),
                _ => None,
            };
            g.ch[c].anim.wall_take_feet = feet;
        }
        if stack {
            if let Some(w) = g.pl[pl].wall.as_mut() {
                w.kind = Some(WallHoldKind::Stack); w.up = Some(0.0); w.lean = Some(0.0); w.lean_want = Some(false); w.lean_dir = Some(side);
                w.lean_dwell = Some(0.0); w.lean_out_t = Some(0.0); w.lean_off = Some(0.0);
            }
        }
        g.m.player.ws_pend.delete(pl);
    }
    g.pl[pl].sprinting = false;
    g.pl[pl].backpedal = false;
    g.ch[c].anim.reverse = false;
    g.pl[pl].speed_sm = 0.0;
    g.ch[c].vel = [0.0, 0.0, 0.0];
    if !stack { set_holstered(g, true, None); }
    true
}

/// Step off the wall, by any way: stance and neck come off, no carry, what he had in his hand is back in it. (player.ts releaseWall)
pub fn release_wall(g: &mut crate::game::Game) {
    let pl = g.player;
    let Some(w) = g.pl[pl].wall else { return };
    let c = g.pl[pl].ch;
    g.pl[pl].wall = None;
    g.ch[c].anim.stance = crate::character::Stance::None;
    g.ch[c].anim.look_yaw_extra = 0.0;
    g.ch[c].vel = [0.0, 0.0, 0.0];
    g.ch[c].anim.wall_take = -1.0;
    g.ch[c].anim.wall_hit = 0.0;
    g.ch[c].anim.stack_press = 0.0;
    g.ch[c].anim.stance_in = 0.0;
    if w.cam { if let Some(cam) = g.director.cam.as_mut() { cam.peek_offset = None; } }
    if w.drawn && !g.pl[pl].down { set_holstered(g, false, None); }
}

/// The player's own ways off the wall: releaseWall, and with the switch on his momentum kept. (player.ts wallStepOff)
pub fn wall_step_off(g: &mut crate::game::Game, mx: f64, mz: f64, ml: f64, how: crate::player::WallStepOffHow) {
    let pl = g.player;
    let w = g.pl[pl].wall;
    release_wall(g);
    let Some(w) = w else { return };
    if !g.k.wallsnap.on || g.pl[pl].down { return; }
    let c = g.pl[pl].ch;
    g.ch[c].anim.stance_out = if g.k.wallsnap.slow { g.k.wallsnap.slow_out } else { g.k.wallsnap.out_rate };
    wall_voice(g, w, false);
    if ml > 0.0 {
        if -(mx * w.n[0] + mz * w.n[2]) < 0.5 {
            let pace = if g.pl[pl].crouch { crouch_pace(g) } else { g.tune.player_walk };
            let walk = match &g.puppet { Some(p) => p.walk.unwrap_or(1.0), None => 1.0 };
            g.pl[pl].speed_sm = m::max(m::abs(w.v), 0.8 * pace * g.pl[pl].pace * walk);
        }
    } else if how == WallStepOffHow::Q {
        let push = g.k.wallsnap.off_push;
        g.ch[c].vel = [w.n[0] * push, 0.0, w.n[2] * push];
    }
}

/// The hold's own two sounds, the player's ears only: `on` = his back has just landed; else he has stepped or been pushed off. (player.ts wallVoice)
pub fn wall_voice(g: &mut crate::game::Game, w: crate::player::WallHold, on: bool) {
    if !g.k.wallsnap.on || !g.k.wallsnap.voice || w.take_t.is_none() { return; }
    if !on && !(w.hit_t.unwrap_or(0.0) > 0.0) { return; }
    let gain = if on { g.k.wallsnap.voice_on } else { g.k.wallsnap.voice_off };
    if gain > 0.0 {
        let at = g.ch[g.pl[g.player].ch].pos;
        let rate = if w.kind == Some(WallHoldKind::Stack) { 0.92 } else { 1.0 };
        seam_audio_play(g, if on { "wallOn" } else { "wallOff" }, at, gain, rate);
    }
}

/// Q pressed with no face in reach while he is on the move: remembered for WALLSNAP.buffer seconds. (player.ts wallSnapLater)
pub fn wall_snap_later(g: &mut crate::game::Game, ml: f64, why: crate::player::WallSnapLaterWhy) -> bool {
    let pl = g.player;
    if !g.k.wallsnap.on || ml <= 0.0 || g.k.wallsnap.buffer <= 0.0 || g.pl[pl].dragging.is_some() { return false; }
    let row = PlayerModWsPendValue { t: g.k.wallsnap.buffer, at: g.time, blocked: why.blocked };
    g.m.player.ws_pend.set(pl, row);
    true
}

/// The remembered Q, looked at each frame. (player.ts wallSnapPending)
pub fn wall_snap_pending(g: &mut crate::game::Game, dt: f64, want_sprint: bool) {
    let pl = g.player;
    let Some(at) = g.m.player.ws_pend.get(pl).map(|p| p.at) else { return };
    if g.pl[pl].wall.is_some() || want_sprint || g.time - at > dt * 1.5 { g.m.player.ws_pend.delete(pl); return; }
    let now = g.time;
    if let Some(p) = g.m.player.ws_pend.get_mut(pl) { p.at = now; }
    let mut why = WallSnapWhy { blocked: false };
    if wall_snap(g, Some(&mut why), None) { return; }
    let Some(p) = g.m.player.ws_pend.get_mut(pl) else { return };
    p.t -= dt;
    if p.t <= 0.0 {
        let blocked = p.blocked;
        g.m.player.ws_pend.delete(pl);
        game_msg_blocked(g, blocked || why.blocked);
    }
}
fn game_msg_blocked(g: &mut Game, blocked: bool) {
    crate::game::game_msg(g, if blocked { "can't get to the wall here" } else { "nothing to put your back to here" });
}

/// the face a ray from his knee along the stick meets, or None on open floor, a door's leaf, or a face the stick is more along than at. (player.ts pressFace)
pub fn press_face(g: &mut crate::game::Game, mx: f64, mz: f64) -> Option<crate::player::PressFace> {
    let c = g.pl[g.player].ch;
    let (p, v) = (g.ch[c].pos, g.ch[c].vel);
    let cos_min = m::cos(g.k.wallsnap.press_deg * v3::DEG);
    let near = crate::wallsnap::ws_press_near(g, m::hyp2(v[0], v[2]));
    let hit = crate::scene_collision::raycast(g, [p[0], KNEE_Y, p[2]], [mx, 0.0, mz], (BODY_R + near) / cos_min)?;
    if hit.index >= 1e6 || m::abs(hit.n[1]) > 0.2 { return None; }
    let n = v3::normalize([hit.n[0], 0.0, hit.n[2]]);
    let into = -(mx * n[0] + mz * n[2]);
    if into <= cos_min { return None; }
    let off = hit.t * into - BODY_R;
    if off > crate::wallsnap::ws_press_near(g, -(v[0] * n[0] + v[2] * n[2])) { None }
    else { Some(PressFace { n, at: [p[0] + mx * hit.t, KNEE_Y, p[2] + mz * hit.t], off }) }
}

/// the move keys that are down, as bits (W 1, A 2, S 4, D 8). (player.ts moveKeysDown)
pub fn move_keys_down(g: &mut crate::game::Game, input: InputId) -> f64 {
    let w = if crate::input::input_down(g, input, "KeyW") { 1 } else { 0 };
    let a = if crate::input::input_down(g, input, "KeyA") { 2 } else { 0 };
    let s = if crate::input::input_down(g, input, "KeyS") { 4 } else { 0 };
    let d = if crate::input::input_down(g, input, "KeyD") { 8 } else { 0 };
    (w | a | s | d) as f64
}

/// Is the run that put him on this wall still held, Shift never let go, the same move keys down? The first frame the keys differ ends it for this hold. (player.ts wallRanOn)
pub fn wall_ran_on(g: &mut crate::game::Game, pl: PlayerId, input: InputId) -> bool {
    let ran = match &g.pl[pl].wall { Some(w) => w.ran, None => None };
    let Some(ran) = ran else { return false };
    if !js::truthy(ran) { return false; }
    if (crate::input::input_down(g, input, "ShiftLeft") || crate::input::input_down(g, input, "ShiftRight")) && move_keys_down(g, input) == ran { return true; }
    if let Some(w) = g.pl[pl].wall.as_mut() { w.ran = None; }
    false
}

/// WALLSNAP.press: the wall taken without the key, by a move key held into a face for pressSecs, with its latch. (player.ts wallPress)
pub fn wall_press(g: &mut crate::game::Game, dt: f64, input: InputId, mx: f64, mz: f64, ml: f64, want_sprint: bool) {
    let pl = g.player;
    let now = g.time;
    if !g.m.player.ws_press.has(pl) {
        g.m.player.ws_press.set(pl, PlayerModWsPressValue { t: 0.0, at: now, n: None, dx: 0.0, dz: 0.0, held: false, until: 0.0 });
    }
    let wall_n = g.pl[pl].wall.map(|w| [w.n[0], 0.0, w.n[2]]);
    let (press_again, press_turn) = (g.k.wallsnap.press_again, g.k.wallsnap.press_turn);
    {
        let p = g.m.player.ws_press.get_mut(pl).unwrap();
        if now - p.at > dt * 1.5 { p.t = 0.0; }
        p.at = now;
        if let Some(n) = wall_n { p.t = 0.0; p.n = Some(n); p.held = false; return; }
        if p.n.is_some() && !p.held { p.held = true; p.dx = mx; p.dz = mz; p.until = now + press_again; }
        if p.n.is_some() && (ml <= 0.0 || mx * p.dx + mz * p.dz < m::cos(press_turn * v3::DEG)) { p.n = None; }
    }
    let face = if ml > 0.0 && (!want_sprint || g.k.wallsnap.press_run) && g.puppet.is_none() && !g.touring { press_face(g, mx, mz) } else { None };
    let (latch_n, until) = { let p = g.m.player.ws_press.get(pl).unwrap(); (p.n, p.until) };
    let face = match face {
        Some(f) if !latch_n.map_or(false, |n| f.n[0] * n[0] + f.n[2] * n[2] > 0.9 || now < until) => f,
        _ => { g.m.player.ws_press.get_mut(pl).unwrap().t = 0.0; return; }
    };
    let holstered = g.pl[pl].holstered;
    let rifle_up = !holstered && crate::game::player_rifle_up(g, pl);
    let in_hand = if holstered { None } else if rifle_up { Some(g.pl[pl].rifle) } else { Some(g.pl[pl].pistol) };
    let gun_away = !holstered && !(g.k.wallsnap.stack && rifle_up);
    let busy = g.pl[pl].dragging.is_some() || g.pl[pl].haul.is_some() || g.pl[pl].door_move.is_some() || g.pl[pl].easing.is_some() || g.pl[pl].charging.is_some()
        || crate::game::player_choosing(g, pl) || g.pl[pl].sel.down_t >= 0.0 || g.pl[pl].throw_held || g.orders.menu.is_some()
        || crate::input::input_lmb(g, input) || crate::input::input_rmb(g, input)
        || in_hand.map_or(false, |gun| crate::weapons::firearm_reloading(g, gun))
        || g.pl[pl].equip_t > 0.0 || now - g.pl[pl].last_fire_t < g.k.wallsnap.press_after_shot;
    let alert_enemy = || g.guards.iter().any(|&h| g.gd[h].state == crate::game::GuardState::Alert && g.gd[h].team != crate::teams::Team::Operative && g.gd[h].team != crate::teams::Team::Civilian);
    if busy || (g.k.wallsnap.press_calm && gun_away && alert_enemy()) { g.m.player.ws_press.get_mut(pl).unwrap().t = 0.0; return; }
    let mut thing = false;
    let mut i = 0;
    while i < g.nearby.len() {
        if g.nearby[i].in_reach && g.m.player.press_things.has(String::from(g.nearby[i].kind.as_str())) {
            let it = g.nearby[i].clone();
            if thing_dist(g, &it, face.at) < g.k.wallsnap.press_thing_at { thing = true; break; }
        }
        i += 1;
    }
    let (press_touch, press_thing_secs, press_secs, press_flow) = (g.k.wallsnap.press_touch, g.k.wallsnap.press_thing, g.k.wallsnap.press_secs, g.k.wallsnap.press_flow);
    if thing && face.off > press_touch { g.m.player.ws_press.get_mut(pl).unwrap().t = 0.0; return; }
    {
        let p = g.m.player.ws_press.get_mut(pl).unwrap();
        p.t += dt;
        if p.t < (if thing { press_thing_secs } else { press_secs }) - 1e-6 || face.off > (if thing { press_touch } else { m::max(press_touch, press_flow) }) { return; }
        p.t = 0.0;
    }
    if !wall_snap(g, None, Some(face.n)) {
        let p = g.m.player.ws_press.get_mut(pl).unwrap();
        p.n = Some(face.n); p.held = true; p.dx = mx; p.dz = mz; p.until = now + press_again;
    } else if want_sprint {
        let keys = move_keys_down(g, input);
        if js::truthy(keys) { if let Some(w) = g.pl[pl].wall.as_mut() { w.ran = Some(keys); } }
    }
}

/// Does the rifle, drawn on this hold, stack on it? (player.ts wallStacks)
pub fn wall_stacks(g: &mut crate::game::Game, w: crate::player::WallHold) -> bool {
    g.k.wallsnap.on && g.k.wallsnap.stack && g.k.wallsnap.stack_draw && w.take_t.is_some()
}

/// The take's turn: his body comes round on the take's own clock. False = not taking. (player.ts wallTakeYaw)
pub fn wall_take_yaw(g: &mut crate::game::Game, pl: PlayerId) -> bool {
    let w = g.pl[pl].wall.unwrap();
    if w.take_t.is_none() || w.settle >= 1.0 { return false; }
    let e = crate::wallsnap::ws_ease(w.settle, g.k.wallsnap.yaw_pow);
    let c = g.pl[pl].ch;
    g.ch[c].body_yaw = v3::wrap_angle(w.yaw0.unwrap() + w.yaw_d.unwrap() * e);
    true
}

/// Where the can he is aiming will come to rest: the arc's first touch bounces and rolls on. Read only. (player.ts throwRest)
pub fn throw_rest(g: &mut crate::game::Game) -> Option<crate::player::ThrowRest> {
    let pl = g.player;
    let (from, v0) = match &g.pl[pl].throw_preview {
        Some(sol) if !sol.points.is_empty() => (sol.points[0], sol.v0),
        _ => return None,
    };
    if !(g.pl[pl].throw_held || (!g.pl[pl].holstered && crate::game::player_canister_sel(g, pl).is_some())) { return None; }
    let kind = g.pl[pl].held_kind;
    let rest = crate::frag::rest_of_throw(g, kind, from, v0);
    Some(ThrowRest { at: rest.at, path: rest.path })
}

/// Esc with the launcher's button down: the gesture is let go of, nothing sent. (player.ts launchCancel)
pub fn launch_cancel(g: &mut crate::game::Game) -> bool {
    let pl = g.player;
    if g.pl[pl].launch_held.is_none() { return false; }
    g.pl[pl].launch_held = None;
    g.pl[pl].throw_preview = None;
    true
}
