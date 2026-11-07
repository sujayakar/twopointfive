//! src/game/sprintdoor.ts lines 1 to 134
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Is the rule live for this man: not while he is down, nor under a show or the puppet's strings.   (sprintdoor.ts live)
pub fn live(g: &mut crate::game::Game, pl: crate::sprintdoor::LivePl) -> bool {
    !pl.down && g.puppet.is_none() && !g.touring
}

/// What his door contact carries this frame: the rule's numbers while he is at a sprint, nothing otherwise.   (sprintdoor.ts sprintDoorRun)
pub fn sprint_door_run(g: &mut crate::game::Game, pl: PlayerId) -> Option<crate::doors::DoorContactRun> {
    let down = g.pl[pl].down;
    if live(g, crate::sprintdoor::LivePl { down }) && g.pl[pl].sprinting {
        Some(crate::doors::DoorContactRun { bash: g.k.sprintdoor.bash_vel, rip: g.k.sprintdoor.rip_vel, rip_level: g.k.sprintdoor.rip_level })
    } else {
        None
    }
}

/// THE BOOT AT THE RUN: the leaf his sprint is about to meet from the face it opens toward, when a boot is the thing to do about it. None = no kick this frame.   (sprintdoor.ts sprintDoorKick)
pub fn sprint_door_kick(g: &mut crate::game::Game, pl: PlayerId) -> Option<DoorId> {
    let down = g.pl[pl].down;
    if !live(g, crate::sprintdoor::LivePl { down }) || !g.pl[pl].sprinting { return None; }
    {
        let p = &g.pl[pl];
        if p.kick.is_some() || p.picking.is_some() || p.coding.is_some() || p.takedown.is_some() || p.holding.is_some() { return None; }
    }
    let c = g.pl[pl].ch;
    let (vx, vz) = (g.ch[c].vel[0], g.ch[c].vel[2]);
    let mut i = 0;
    while i < g.doors.list.len() {
        let d = g.doors.list[i];
        i += 1;
        let pos = g.ch[c].pos;
        if crate::doors::door_locked(g, d) || !crate::doors::doors_boot_holds(g, d, pos) { continue; }
        let a = g.dr[d].def.closed_dir + crate::doors::door_angle(g, d);
        let (ux, uz) = (m::cos(a), m::sin(a));
        let px = g.ch[c].pos[0] - g.dr[d].def.hinge[0];
        let pz = g.ch[c].pos[2] - g.dr[d].def.hinge[1];
        let perp = ux * pz - uz * px;
        let s = if perp >= 0.0 { 1.0 } else { -1.0 };
        let off = m::abs(perp);
        let into = (uz * vx - ux * vz) * s; // his speed INTO the leaf (its normal on his side is (−u.z, u.x)·s)
        if into < g.k.sprintdoor.kick_into || off > KICK_OFF + into / KICK_EASE || off < KICK_OFF - KICK_NEAR { continue; }
        let meet = px * ux + pz * uz + (vx * ux + vz * uz) * (off / into); // where along the leaf his run lands
        if meet < 0.1 || meet > g.dr[d].def.width - 0.05 { continue; }
        let pos = g.ch[c].pos;
        let spot = crate::doors::doors_kick_spot(g, d, pos).pos;
        if crate::scene_collision::nav_grid_is_blocked(g, spot[0], spot[2]) { continue; }
        let pos = g.ch[c].pos;
        if crate::scene_collision::solid_blocked(g, [pos[0], 0.5, pos[2]], [spot[0], 0.5, spot[2]], None) { continue; }
        return Some(d);
    }
    None
}

/// After the doors have been resolved: what the leaf he ran open or ripped round cost him.   (sprintdoor.ts sprintDoorAfter)
pub fn sprint_door_after(g: &mut crate::game::Game, pl: PlayerId, own: &crate::doors::DoorContact) {
    let c = g.pl[pl].ch;
    if own.run.is_none() { return; }
    let sp = m::hyp2(g.ch[c].vel[0], g.ch[c].vel[2]);
    if let (Some(d), true) = (own.bashed, g.time - g.pl[pl].door_bash_at >= AGAIN) {
        // the part of his run that went into the leaf, as it stood when he met it
        let a = g.dr[d].def.closed_dir + own.bashed_at.unwrap_or(0.0);
        let (nx, nz) = (-m::sin(a), m::cos(a));
        let sq = if sp > 1e-3 { m::min(1.0, m::abs(g.ch[c].vel[0] * nx + g.ch[c].vel[2] * nz) / sp) } else { 1.0 };
        let k = 1.0 - (1.0 - g.k.sprintdoor.keep) * sq;
        g.pl[pl].speed_sm *= k;
        g.ch[c].vel = [g.ch[c].vel[0] * k, 0.0, g.ch[c].vel[2] * k];
        // the shoulder nearer the latch leads: which way that turns him is the side of his run the hinge is on
        let hx = g.dr[d].def.hinge[0] - g.ch[c].pos[0];
        let hz = g.dr[d].def.hinge[1] - g.ch[c].pos[2];
        let cross = g.ch[c].vel[0] * hz - g.ch[c].vel[2] * hx;
        g.pl[pl].door_bash_at = g.time;
        g.pl[pl].door_bash_lean = if cross >= 0.0 { -1.0 } else { 1.0 };
        g.stats.sprintdoor.bashes += 1;
    }
    if own.ripped.is_some() {
        let cap = g.tune.player_sprint * g.k.sprintdoor.rip_pace;
        let now = m::hyp2(g.ch[c].vel[0], g.ch[c].vel[2]);
        if now > cap {
            let k = cap / now;
            g.ch[c].vel = [g.ch[c].vel[0] * k, 0.0, g.ch[c].vel[2] * k];
        }
        g.pl[pl].speed_sm = m::min(g.pl[pl].speed_sm, cap);
        g.pl[pl].door_rip_until = g.time + g.k.sprintdoor.rip_secs;
        g.stats.sprintdoor.rips += 1;
    }
}

/// The share of his top speed his run is held to this frame: `ripPace` while his hand is ripping a leaf round, else 1.   (sprintdoor.ts sprintDoorPace)
pub fn sprint_door_pace(g: &mut crate::game::Game, pl: crate::sprintdoor::DoorStamps, now: Option<f64>) -> f64 {
    let now = now.unwrap_or(g.time);
    if live(g, crate::sprintdoor::LivePl { down: pl.down }) && now < pl.door_rip_until { g.k.sprintdoor.rip_pace } else { 1.0 }
}

/// Is the turn of his shoulder into a leaf (or his head coming back off it) still running.   (sprintdoor.ts sprintDoorTurning)
pub fn sprint_door_turning(g: &mut crate::game::Game, pl: crate::sprintdoor::DoorStamps, now: Option<f64>) -> bool {
    let now = now.unwrap_or(g.time);
    g.k.sprintdoor.lean_secs > 0.0 && now - pl.door_bash_at < g.k.sprintdoor.lean_secs + SETTLE
}

/// Radians added to the yaw his body runs at: the shoulder turned into the leaf, easing back square. None = no turn running and none just ended.   (sprintdoor.ts sprintDoorLean)
pub fn sprint_door_lean(g: &mut crate::game::Game, pl: crate::sprintdoor::DoorStamps, now: Option<f64>) -> Option<f64> {
    let now = now.unwrap_or(g.time);
    if !sprint_door_turning(g, pl, Some(now)) { return None; }
    let t = (now - pl.door_bash_at) / g.k.sprintdoor.lean_secs;
    if t < 0.0 { return None; }
    Some(if t >= 1.0 { 0.0 } else { pl.door_bash_lean * g.k.sprintdoor.lean * (1.0 - t) * (1.0 - t) })
}

/// The look his head is given while his body is `off` radians off his run: the same angle back the other way.   (sprintdoor.ts sprintDoorHead)
pub fn sprint_door_head(off: f64) -> f64 {
    let w = off - m::round(off / (2.0 * m::PI)) * 2.0 * m::PI;
    if m::abs(w) < 1e-6 { 0.0 } else { HEAD_SIGN * w }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;

    /// A blank Game is too big for a test thread's stack in a debug build once it can be built at all.
    fn big_stack(f: fn()) {
        std::thread::Builder::new().stack_size(1 << 30).spawn(f).unwrap().join().unwrap();
    }

    // Bits from the TypeScript's own functions under bun with EXACT.on.
    #[test]
    fn head_matches_the_typescript() {
        let rows: [(f64, u64); 9] = [
            (0.0, 0x0000000000000000),
            (1e-7, 0x0000000000000000),
            (0.45, 0xbfdccccccccccccd),
            (-0.45, 0x3fdccccccccccccd),
            (3.2, 0x4008aa5d0eeec096),
            (-3.2, 0xc008aa5d0eeec096),
            (7.1, 0xbfea23589111ca70),
            (-13.37, 0x3fe9b754f92dd250),
            (100.25, 0x3fd1fb54442d1800),
        ];
        for (off, bits) in rows { assert_eq!(sprint_door_head(off).to_bits(), bits, "off {off}"); }
    }

    #[test]
    fn lean_matches_the_typescript() {
        big_stack(|| {
        let mut g = Game::blank();
        g.k.sprintdoor = crate::knobs::page_sprintdoor();
        let rows: [(f64, f64, u64); 5] = [
            (10.1, -1.0, 0xbfc4bc6a7ef9db3e),
            (10.17, 1.0, 0x3fa797cc39ffd61a),
            (10.2499, 1.0, 0x3e7353cd652b4e5a),
            (10.3, -1.0, 0x0000000000000000),
            (10.45, 1.0, 0x0000000000000000),
        ];
        for (now, lean, bits) in rows {
            let st = DoorStamps { down: false, door_rip_until: -1.0, door_bash_at: 10.0, door_bash_lean: lean };
            assert_eq!(sprint_door_lean(&mut g, st, Some(now)).map(f64::to_bits), Some(bits), "now {now}");
        }
        let st = DoorStamps { down: false, door_rip_until: -1.0, door_bash_at: 10.0, door_bash_lean: 1.0 };
        assert_eq!(sprint_door_lean(&mut g, st, Some(10.46)), None); // past the turn and its settle
        assert_eq!(sprint_door_lean(&mut g, st, Some(9.9)), None); // before the stamp
        });
    }

    fn stage() -> (Game, PlayerId, CharId, DoorId) {
        let mut g = Game::blank();
        g.k.sprintdoor = crate::knobs::page_sprintdoor();
        g.time = 12.5;
        g.tune.player_sprint = 5.6;
        let mut ch = crate::character::Character::blank();
        ch.vel = [4.1, 0.0, 3.3];
        ch.pos = [0.4, 0.0, -1.9];
        let c = g.ch.push(ch);
        let mut p = crate::game::Player::blank();
        p.ch = c;
        p.speed_sm = 5.3;
        p.door_bash_at = -10.0;
        p.door_rip_until = -1.0;
        let pl = g.pl.push(p);
        let mut dr = crate::doors::Door::blank();
        dr.def.closed_dir = 0.7;
        dr.def.hinge = [1.25, -3.5];
        let d = g.dr.push(dr);
        (g, pl, c, d)
    }

    fn contact(d: DoorId, ripped: bool) -> crate::doors::DoorContact {
        let mut own = crate::doors::DoorContact::zeroed();
        own.run = Some(crate::doors::DoorContactRun::zeroed());
        own.bashed = Some(d);
        own.bashed_at = Some(0.3);
        own.ripped = if ripped { Some(d) } else { None };
        own
    }

    #[test]
    fn after_a_bash_matches_the_typescript() {
        big_stack(|| {
        let (mut g, pl, c, d) = stage();
        sprint_door_after(&mut g, pl, &contact(d, false));
        assert_eq!(g.ch[c].vel[0].to_bits(), 0x400e33e5a6dfdb01);
        assert_eq!(g.ch[c].vel[2].to_bits(), 0x40084f3bf6b42d26);
        assert_eq!(g.pl[pl].speed_sm.to_bits(), 0x401385721790afe5);
        assert_eq!(g.pl[pl].door_bash_lean, 1.0);
        assert_eq!(g.pl[pl].door_bash_at, 12.5);
        assert_eq!(g.stats.sprintdoor.bashes, 1);
        assert_eq!(g.stats.sprintdoor.rips, 0);
        });
    }

    #[test]
    fn after_a_bash_and_a_rip_matches_the_typescript() {
        big_stack(|| {
        let (mut g, pl, c, d) = stage();
        sprint_door_after(&mut g, pl, &contact(d, true));
        assert_eq!(g.ch[c].vel[0].to_bits(), 0x3ff4f098f7b29872);
        assert_eq!(g.ch[c].vel[2].to_bits(), 0x3ff0daa0956a48bf);
        assert_eq!(g.pl[pl].speed_sm.to_bits(), 0x3ffae147ae147ae1);
        assert_eq!(g.pl[pl].door_rip_until.to_bits(), 0x402999999999999a);
        assert_eq!(g.stats.sprintdoor.rips, 1);
        });
    }

    #[test]
    fn a_second_blow_within_again_costs_nothing() {
        big_stack(|| {
        let (mut g, pl, c, d) = stage();
        g.pl[pl].door_bash_at = 12.3;
        sprint_door_after(&mut g, pl, &contact(d, false));
        assert_eq!(g.ch[c].vel, [4.1, 0.0, 3.3]);
        assert_eq!(g.pl[pl].door_bash_at, 12.3);
        assert_eq!(g.stats.sprintdoor.bashes, 0);
        });
    }
}
