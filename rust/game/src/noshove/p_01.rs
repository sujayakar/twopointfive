//! src/game/noshove.ts lines 1 to 138
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Is this a man the rule has: one of the player's own under 'fall in', on his order's legs, not on a live grenade's. (noshove.ts noShoveMan)
pub fn no_shove_man(g: &mut Game, gd: GuardId) -> bool {
    if g.gd[gd].state == crate::game::GuardState::Dead || g.gd[gd].script.is_none() { return false; }
    match crate::orders::squad_orders_of(g, gd) {
        Some(a) => g.tab.orders_assignment[a].post.is_none() && !crate::frag::frag_running(g, gd),
        None => false,
    }
}

/// player.ts updatePlayer, first thing: nobody has moved him yet this frame; his own legs start here. `None` = the seat's player. (noshove.ts noShoveBegin)
pub fn no_shove_begin(g: &mut Game, pl: Option<&mut crate::noshove::Shoved>) {
    let now = g.time;
    match pl {
        Some(pl) => {
            let (px, pz) = (pl.char.pos[0], pl.char.pos[2]);
            pl.shoved = [0.0, 0.0]; pl.shoved_t = now; pl.shoved_from = [px, pz];
        }
        None => {
            let p = g.player;
            let c = g.pl[p].ch;
            let (px, pz) = (g.ch[c].pos[0], g.ch[c].pos[2]);
            let r = &mut g.pl[p];
            r.shoved = [0.0, 0.0]; r.shoved_t = now; r.shoved_from = [px, pz];
        }
    }
}

/// player.ts keepOffGuards: a man moved him by this much, after the walls had their say. (noshove.ts noShoveNote)
pub fn no_shove_note(g: &mut Game, dx: f64, dz: f64, pl: Option<&mut crate::noshove::Shoved>) {
    let now = g.time;
    match pl {
        Some(pl) => {
            let (px, pz) = (pl.char.pos[0], pl.char.pos[2]);
            note_on(&mut pl.shoved, &mut pl.shoved_t, &mut pl.shoved_from, px, pz, now, dx, dz);
        }
        None => {
            let p = g.player;
            let c = g.pl[p].ch;
            let (px, pz) = (g.ch[c].pos[0], g.ch[c].pos[2]);
            let r = &mut g.pl[p];
            note_on(&mut r.shoved, &mut r.shoved_t, &mut r.shoved_from, px, pz, now, dx, dz);
        }
    }
}

/// noShoveNote's body, over the three fields of whichever holder it was handed.
fn note_on(shoved: &mut [f64; 2], shoved_t: &mut f64, shoved_from: &mut [f64; 2], px: f64, pz: f64, now: f64, dx: f64, dz: f64) {
    if *shoved_t != now { *shoved = [0.0, 0.0]; *shoved_t = now; *shoved_from = [px - dx, pz - dz]; }
    shoved[0] += dx; shoved[1] += dz;
}

/// player.ts keepOffGuards, of a man the rule has at (mx, mz), `l` from the player: how far the player is put back, what his OWN legs closed on the man and no more. (noshove.ts noShoveBack)
pub fn no_shove_back(g: &mut Game, mx: f64, mz: f64, l: f64, pl: Option<&crate::noshove::Shoved>) -> f64 {
    let (shoved, shoved_t, shoved_from) = match pl {
        Some(pl) => (pl.shoved, pl.shoved_t, pl.shoved_from),
        None => { let r = &g.pl[g.player]; (r.shoved, r.shoved_t, r.shoved_from) }
    };
    if shoved_t != g.time { return 0.0; }
    back_from(shoved, shoved_from, mx, mz, l)
}

/// noShoveBack's sum: what his own legs closed on the man, measured from where he would stand had they not moved.
fn back_from(shoved: [f64; 2], shoved_from: [f64; 2], mx: f64, mz: f64, l: f64) -> f64 {
    let d0 = m::hyp2(shoved_from[0] + shoved[0] - mx, shoved_from[1] + shoved[1] - mz);
    m::max(0.0, m::min(crate::player::KEEP_OFF, d0) - l)
}

/// orders.ts tick: the player has covered (dx, dz) since the last tick: the step his own legs took of it, less what the men took back. Always `Some` (its switch is settled on). (noshove.ts noShoveOwn)
pub fn no_shove_own(g: &mut Game, dx: f64, dz: f64) -> Option<[f64; 2]> {
    let p = g.player;
    if g.pl[p].shoved_t != g.time { return Some([dx, dz]); }
    let (sx, sz) = (g.pl[p].shoved[0], g.pl[p].shoved[1]);
    let sl = m::hyp2(sx, sz);
    if sl < 1e-9 { return Some([dx, dz]); }
    g.stats.noshove.not_walked += 1;
    let c = g.pl[p].ch;
    Some(own_step(g.ch[c].pos, sx, sz, sl, g.pl[p].shoved_from))
}

/// noShoveOwn's sum: the step of his own legs, less the part of the men's shove that runs against it, and no more than they gave.
fn own_step(pos: V3, sx: f64, sz: f64, sl: f64, shoved_from: [f64; 2]) -> [f64; 2] {
    let mx = pos[0] - sx - shoved_from[0];
    let mz = pos[2] - sz - shoved_from[1];
    let t = m::min(m::max(-(mx * sx + mz * sz) / sl, 0.0), sl);
    [mx + sx / sl * t, mz + sz / sl * t]
}

/// squad.ts runGuardScript, after his step and its collisions: he stood at (x0, z0) before it. He goes round the player, stops short, squeezes past or steps back out. True: held short this frame (the caller stands his legs). (noshove.ts noShoveKeep)
pub fn no_shove_keep(g: &mut Game, gd: GuardId, x0: f64, z0: f64, dt: f64) -> bool {
    if !no_shove_man(g, gd) { return false; }
    let (ring, back) = (g.k.noshove.ring, g.k.noshove.back);
    let c = g.gd[gd].ch;
    let pc = g.pl[g.player].ch;
    let s = g.ch[pc].pos;   // (the TypeScript's live alias; only this man's body moves below, so a copy reads the same)
    let (x1, z1) = (g.ch[c].pos[0], g.ch[c].pos[2]);
    let l1 = m::hyp2(x1 - s[0], z1 - s[2]);
    if l1 >= ring { return false; }
    let (bx, bz) = (x0 - s[0], z0 - s[2]);
    let l0 = m::hyp2(bx, bz);
    if l1 >= l0 - 1e-6 {   // not closing on him: inside the player's own keep-off he steps back out
        if l1 >= crate::player::KEEP_OFF - 0.005 { return false; }
        let yaw = g.ch[c].body_yaw;
        let ux = if l1 > 1e-4 { (x1 - s[0]) / l1 } else { -m::sin(yaw) };
        let uz = if l1 > 1e-4 { (z1 - s[2]) / l1 } else { -m::cos(yaw) };
        let step = m::min(crate::player::KEEP_OFF - l1, back * dt);
        g.ch[c].pos[0] += ux * step;
        g.ch[c].pos[2] += uz * step;
        settle(g, c, s, l1, [x1, z1]);
        g.stats.noshove.back += 1;
        return false;
    }
    // closing on him inside the ring. A corner of his way under the player is not walked to: the one after it, where that is a straight walk
    while g.gd[gd].path_i < g.gd[gd].path.len() && under(g.gd[gd].path[g.gd[gd].path_i], s, ring) {
        let i = g.gd[gd].path_i;
        let nx = match g.gd[gd].path.get(i + 1) { Some(q) => Some(*q), None => script_goal(g, gd) };
        let Some(nx) = nx else { break };
        if crate::scene_collision::solid_blocked(g, [x0, 0.45, z0], [nx[0], 0.45, nx[2]], None) { break; }
        g.gd[gd].path_i += 1;
    }
    let goal = script_goal(g, gd);
    // `wp !== goal` of the TypeScript is "wp is a corner of his path": no path holds the goal object itself (a path's points are the nav's own)
    let corner = if g.gd[gd].path_i < g.gd[gd].path.len() { Some(g.gd[gd].path[g.gd[gd].path_i]) } else { None };
    let wp = if corner.is_some() { corner } else { goal };
    let (mx, mz) = (x1 - x0, z1 - z0);
    let ml = m::hyp2(mx, mz);
    let wl = match wp { Some(w) => m::hyp2(w[0] - x0, w[2] - z0), None => 0.0 };
    // the way he is going: the leg to his next corner (steady while he turns onto it), else this frame's step
    let (fx, fz) = match wp {
        Some(w) if wl > 0.3 => ((w[0] - x0) / wl, (w[2] - z0) / wl),
        _ => (mx / ml, mz / ml),
    };
    let rad = m::min(ring, l0);   // (never nearer than he stood)
    let ux = if l0 > 1e-4 { bx / l0 } else { -fx };
    let uz = if l0 > 1e-4 { bz / l0 } else { -fz };
    if let Some(q) = goal {
        if m::hyp2(q[0] - s[0], q[2] - s[2]) < ring { return short(g, gd, c, s, rad, ux, uz, x0, z0); }   // his place is where the player stands
    }
    if let Some(q) = corner {
        if under(q, s, ring) { return short(g, gd, c, s, rad, ux, uz, x0, z0); }   // a corner under the player that he cannot cut: he waits for it
    }
    // which side of the player he passes: the one his way already lies on (square on: his right), if his body has the room there; else the other
    let off = bx * fz - bz * fx;
    let mut side: f64 = if off <= -0.04 { -1.0 } else { 1.0 };
    if !room(g, c, s, fx, fz, ring, side) {
        side = if room(g, c, s, fx, fz, ring, -side) { -side } else { 0.0 };
    }
    if side == 0.0 { g.stats.noshove.through += 1; return false; }   // no room either way: he squeezes past, the player unmoved
    // round: this frame's stride, turned off his line toward that side by how squarely the player is in front of him, and never inside the ring
    let ahead = m::max(0.0, -(ux * fx + uz * fz));
    let dx = fx + fz * side * ahead;
    let dz = fz - fx * side * ahead;
    let dl = js::or(m::hyp2(dx, dz), 1.0);
    let mut qx = x0 + dx / dl * ml - s[0];
    let mut qz = z0 + dz / dl * ml - s[2];
    let ql = m::hyp2(qx, qz);
    if ql < rad {
        if ql > 1e-4 { qx *= rad / ql; qz *= rad / ql; } else { qx = ux * rad; qz = uz * rad; }
    }
    g.ch[c].pos[0] = s[0] + qx;
    g.ch[c].pos[2] = s[2] + qz;
    g.stats.noshove.round += 1;
    settle(g, c, s, rad, [x1, z1]);   // (the walls eat the turn: he squeezes past after all)
    false
}

/// `gd.script?.goal ?? null`
fn script_goal(g: &Game, gd: GuardId) -> Option<V3> {
    g.gd[gd].script.as_ref().and_then(|t| t.goal)
}

/// The closure `under`: is this point of his way under the player.
fn under(q: V3, s: V3, ring: f64) -> bool {
    m::hyp2(q[0] - s[0], q[2] - s[2]) < ring + 0.05
}

/// The closure `settle`: the walls have their say; left nearer the player than `min` by them, he is put at `back` (true).
fn settle(g: &mut Game, c: CharId, s: V3, min: f64, back: [f64; 2]) -> bool {
    let (mut p, r) = (g.ch[c].pos, g.ch[c].radius);
    crate::scene_collision::collide_circle(g, &mut p, r, 0.2, 1.5, None);
    g.ch[c].pos = p;
    if m::hyp2(p[0] - s[0], p[2] - s[2]) >= min - 1e-6 { return false; }
    g.ch[c].pos[0] = back[0];
    g.ch[c].pos[2] = back[1];
    true
}

/// The closure `short`: held off, straight back out to where he may be; his legs stop (a man walking on the spot wakes followPath's stuck side-step). Always true.
fn short(g: &mut Game, gd: GuardId, c: CharId, s: V3, rad: f64, ux: f64, uz: f64, x0: f64, z0: f64) -> bool {
    let p = g.ch[c].pos;
    let l = m::hyp2(p[0] - s[0], p[2] - s[2]);
    let vx = if l > 1e-4 { (p[0] - s[0]) / l } else { ux };
    let vz = if l > 1e-4 { (p[2] - s[2]) / l } else { uz };
    g.ch[c].pos[0] = s[0] + vx * rad;
    g.ch[c].pos[2] = s[2] + vz * rad;
    settle(g, c, s, rad, [x0, z0]);
    g.stats.noshove.short += 1;
    g.gd[gd].speed = 0.0;
    g.ch[c].vel = [0.0, 0.0, 0.0];
    true
}

/// The closure `room`: has his body the room to pass the player on this side.
fn room(g: &mut Game, c: CharId, s: V3, fx: f64, fz: f64, ring: f64, side: f64) -> bool {
    let hit = crate::scene_collision::raycast(g, [s[0], 0.45, s[2]], [fz * side, 0.0, -fx * side], 1.5);
    let t = match hit { Some(h) => h.t, None => 1.5 };
    t >= ring + g.ch[c].radius - 0.05
}

/// One line for a tool or the debugger. (noshove.ts noShoveSummary)
pub fn no_shove_summary(g: &mut Game) -> String {
    let t = &g.stats.noshove;
    alloc::format!("no shove on · frames: round {}, short {}, squeezed past {}, stepped back {} · shoves kept out of the player's pace {}", t.round, t.short, t.through, t.back, t.not_walked)
}

#[cfg(test)]
mod tests {
    use super::*;

    // the numbers: the TypeScript's own, under EXACT, in bun
    #[test]
    fn back_is_the_typescripts_bits() {
        let b = back_from([0.11, -0.07], [1.05, -2.41], 1.61, -2.13, 0.31);
        assert_eq!(b.to_bits(), 0x3fceb851eb851eba);
    }

    #[test]
    fn note_starts_afresh_on_a_new_frame() {
        let (mut shoved, mut t, mut from) = ([0.11, -0.07], 4.0, [1.05, -2.41]);
        note_on(&mut shoved, &mut t, &mut from, 1.3, -2.7, 5.25, 0.013, -0.029);
        assert_eq!(shoved[0].to_bits(), 0x3f8a9fbe76c8b439);
        assert_eq!(shoved[1].to_bits(), 0xbf9db22d0e560419);
        assert_eq!(from[0].to_bits(), 0x3ff4978d4fdf3b65);
        assert_eq!(from[1].to_bits(), 0xc0055e353f7ced92);
        assert_eq!(t, 5.25);
    }

    #[test]
    fn own_takes_the_shove_out_of_his_step() {
        let pos = [1.3, 0.0, -2.7];
        let r = own_step(pos, 0.11, -0.07, m::hyp2(0.11, -0.07), [1.05, -2.41]);
        assert_eq!(r[0].to_bits(), 0x3fc1eb851eb851e8);
        assert_eq!(r[1].to_bits(), 0xbfcc28f5c28f5c30);
        let r = own_step(pos, 0.3, 0.2, m::hyp2(0.3, 0.2), [1.4, -2.2]);
        assert_eq!(r[0].to_bits(), 0xbfb9999999999994);
        assert_eq!(r[1].to_bits(), 0xbfe0000000000002);
    }
}
