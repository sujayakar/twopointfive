use crate::v3::dist_xz;
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---- this part's reach into S(g)

/// what a man's gate needs of another man's row.
#[derive(Clone, Copy)]
struct GateRead { phase: GatePhase, action: GateAction, thrower: Option<GuardId>, cross_at: f64 }
/// S(g).gates.get(man), the fields read of it.
fn gate_read(g: &mut Game, man: GuardId) -> Option<GateRead> {
    let row = s(g).gates.get(man)?;
    Some(GateRead { phase: row.phase, action: row.action, thrower: row.thrower, cross_at: row.cross_at })
}

/// ONE FRAME OF A MAN'S GATE. He is held at it whatever it comes to this frame, its end included: followPath must not read the emptied path as 'arrived'.
pub(super) fn run_gate(g: &mut crate::game::Game, gd: GuardId, gate: &mut crate::tactics::Gate, dt: f64) -> bool {
    if gate_over(g, gd, gate) || lane_shut(g, gd, gate) { release_gate(g, gd, gate); return true; }
    // (standing here on purpose: the branches' no-headway watchdogs are not to read it as a wedge)
    g.gd[gd].stall_ref = None; g.gd[gd].alert_stall_t = 0.0; g.gd[gd].chase_t = 0.0;
    gate.t += dt;
    let w = g.dr[gate.door].def.width;
    let inside = door_pt(g, gate.door, w / 2.0, -1.0, gate.side, Some(1.1));   // what a stacked man's muzzle rests on
    leads_now(g, gate);
    match gate.phase {
        GatePhase::Stack => stack(g, gd, gate, dt, inside),
        GatePhase::Wait => wait(g, gd, gate, dt, inside),
        GatePhase::Open => open(g, gd, gate, dt, inside),
        GatePhase::Stacked => stacked(g, gd, gate, dt, inside),
        GatePhase::Toss => toss(g, gd, gate, dt),
        GatePhase::Bang => bang(g, gd, gate, dt, inside),
        GatePhase::Slice => slice(g, gd, gate, dt),
        GatePhase::Cross => cross(g, gd, gate, dt, inside),
        GatePhase::Done => release_gate(g, gd, gate),
    }
    true
}

/// Still on the near side with nothing begun.
fn before_the_door(gate: &Gate) -> bool { matches!(gate.phase, GatePhase::Stack | GatePhase::Wait | GatePhase::Stacked) }

/// He is in a fight, or no longer on the hunt; it has gone on too long; or the doorway is no danger any more and he has not begun.
fn gate_over(g: &Game, gd: GuardId, gate: &Gate) -> bool {
    use crate::game::GuardState as S;
    let state = g.gd[gd].state;
    let hunting = matches!(state, S::Alert | S::Search | S::Suspicious) || (state == S::Patrol && g.gd[gd].drawn);
    in_contact(g, gd) || !hunting || g.time - gate.t0 > GATE_CAP_SECS || (g.tab.tactics_danger_rec[gate.rec].until <= g.time && before_the_door(gate))
}

/// The drill's lanes, asked each frame until he commits: the squad's guns have that doorway, and a stun charge or two does not change it.
fn lane_shut(g: &mut Game, gd: GuardId, gate: &Gate) -> bool {
    if !before_the_door(gate) && !matches!(gate.phase, GatePhase::Open | GatePhase::Slice) { return false; }
    let lead = match (gate.role, gate.lead) { (GateRole::Second, Some(l)) => gate_read(g, l).map(|row| (l, row)), _ => None };
    let mate = if gate.role == GateRole::Lead { gate_mate(g, gate, crate::tactics::GateRole::Second, gd) } else { None };
    let going = lead.is_some_and(|(_, r)| matches!(r.phase, GatePhase::Toss | GatePhase::Bang | GatePhase::Cross) || (r.action == GateAction::Toss && r.thrower == Some(gd)));
    if going { return false; }
    let can = stuns_of(g, gd) > 0.0 || mate.as_ref().is_some_and(|mm| stuns_of(g, mm.gd) > 0.0) || lead.is_some_and(|(l, _)| stuns_of(g, l) > 0.0);
    let at = match (gate.phase, gate.hs) {
        (GatePhase::Slice, _) if !gate.slice_pts.is_empty() => gate.slice_pts[m::min(gate.slice_i, gate.slice_pts.len() as f64 - 1.0) as usize],
        (GatePhase::Open, Some(Some(h))) => h.pos,
        _ => gate.spot,
    };
    let w = g.dr[gate.door].def.width;
    let far = door_pt(g, gate.door, w / 2.0, 1.2, -gate.side, None);
    let centre = g.dr[gate.door].frame_centre;
    crate::lanes::lane_drill(g, gd, at, far, centre, can)
}

/// A second whose lead has gone (dead, released, fighting) leads from where he is.
fn leads_now(g: &mut Game, gate: &mut Gate) {
    if gate.role != GateRole::Second { return; }
    let there = gate.lead.is_some_and(|l| s(g).gates.has(l) && !crate::condition::down(g, g.gd[l].ch));
    if there { return; }
    gate.role = GateRole::Lead;
    gate.lead = None;
    if gate.phase == GatePhase::Stacked { gate.phase = GatePhase::Wait; }
}

/// To his mark at the jamb.
fn stack(g: &mut Game, gd: GuardId, gate: &mut Gate, dt: f64, inside: V3) {
    let c = g.gd[gd].ch;
    gate.look = Some(inside); gate.crouch = 0.0;
    let spot = gate.spot;
    let face = if dist_xz(g.ch[c].pos, spot) < 1.2 { Some(gate.face) } else { None };
    let speed = g.tune.guard_investigate;
    let there = step_gate(g, gd, gate, dt, spot, speed, face);
    if there || gate.t > 5.0 {
        gate.t = 0.0;
        if gate.role == GateRole::Lead { gate_call(g, gd, gate); gate.phase = GatePhase::Wait; }
        else { gate.phase = GatePhase::Stacked; }
    }
}

/// On the jamb, low, muzzle on the opening, until a second man is stacked behind him or `door_wait` is up.
fn wait(g: &mut Game, gd: GuardId, gate: &mut Gate, dt: f64, inside: V3) {
    stand_gate(g, gd, dt, Some(gate.face)); gate.look = Some(inside); gate.crouch = 0.5;
    let mate = gate_mate(g, gate, crate::tactics::GateRole::Second, gd);
    let set = match &mate { Some(mm) => mm.g.phase == GatePhase::Stacked, None => false };
    if set || gate.t >= g.tune.door_wait {
        gate.thrower = if stuns_of(g, gd) > 0.0 { Some(gd) } else {
            match &mate { Some(mm) if set && stuns_of(g, mm.gd) > 0.0 => Some(mm.gd), _ => None }
        };
        gate.action = if gate.thrower.is_some() { GateAction::Toss } else { GateAction::Slice };
        gate.phase = GatePhase::Open; gate.t = 0.0;
    }
}

/// A shut leaf: his hand on the lever, shoved into the room and the closer held off; then the charge or the slice.
fn open(g: &mut Game, gd: GuardId, gate: &mut Gate, dt: f64, inside: V3) {
    let d = gate.door;
    if !at_the_handle(g, gd, gate, dt, inside) { return; }
    stand_gate(g, gd, dt, Some(gate.face)); gate.look = Some(inside); gate.crouch = 0.4;
    if !gate.opened { works_the_lever(g, gd, gate); }
    if gate.t >= 0.8 {
        gate.t = 0.0;
        let toss_by = match (gate.action, gate.thrower) { (GateAction::Toss, Some(t)) if !crate::condition::down(g, g.gd[t].ch) && !crate::doors::door_is_closed(g, d) => Some(t), _ => None };
        match toss_by { Some(t) => begin_toss(g, gd, gate, t), None => begin_slice(g, gate) }
    }
}

/// At the place the lever is worked from: the handle's own, or the stack's jamb where that serves. (`hs`: None = not asked yet, Some(None) = the jamb serves, Some(Some) = the handle's place)
fn at_the_handle(g: &mut Game, gd: GuardId, gate: &mut Gate, dt: f64, inside: V3) -> bool {
    let d = gate.door;
    if gate.opened || matches!(gate.hs, Some(None)) || !crate::doors::door_is_closed(g, d) { return true; }
    let h = match gate.hs {
        Some(h) => h,
        None => {
            let pos = g.pos_of(gd);
            let h = if crate::doors::doors_pull_face(g, d, pos) { crate::handleside::tool_spot(g, d, gate.side, pos, None) } else { None };
            let h = h.filter(|h| !h.far).map(|h| GateHs { pos: h.pos, yaw: h.yaw });
            gate.hs = Some(h);
            h
        }
    };
    if let Some(h) = h {
        let speed = g.tune.guard_investigate;
        let stepped = step_gate(g, gd, gate, dt, h.pos, speed, Some(h.yaw)) && crate::handleside::tool_step(g, gd, h.pos);
        if !stepped && gate.t < 3.0 { gate.look = Some(inside); gate.crouch = 0.4; return false; }
        gate.near_a = g.dr[d].def.width; gate.spot = h.pos; gate.face = h.yaw; gate.t = 0.0;
    }
    gate.hs = Some(None);
    true
}

/// The lever down and the leaf in, its closer held off a while; an open one costs no time.
fn works_the_lever(g: &mut Game, gd: GuardId, gate: &mut Gate) {
    let d = gate.door;
    gate.opened = true;
    if !crate::doors::door_is_closed(g, d) { gate.t = 1.0; return; }
    let c = g.gd[gd].ch;
    let (pos, id, keys) = (g.ch[c].pos, g.ch[c].id, g.gd[gd].keys.clone());
    crate::doors::doors_use(g, d, pos, id, Some(&keys));
    let q = m::min(g.dr[d].leaf.quiet_t, -6.0);
    g.dr[d].leaf.quiet_t = q;
    gate.t = 0.0;
}

/// The second: tucked in behind the lead, muzzle over him on the opening; goes when he goes.
fn stacked(g: &mut Game, gd: GuardId, gate: &mut Gate, dt: f64, inside: V3) {
    let (c, d, now) = (g.gd[gd].ch, gate.door, g.time);
    stand_gate(g, gd, dt, Some(gate.face)); gate.crouch = 0.3;
    let lead = match gate.lead { Some(l) => gate_read(g, l), None => None };
    gate.look = Some(match lead {
        Some(lr) if lr.phase == GatePhase::Toss || lr.phase == GatePhase::Bang => { let pos = g.ch[c].pos; away_look(g, d, gate, pos) }
        _ => inside,
    });
    if let Some(lr) = lead {
        if lr.phase == GatePhase::Cross && lr.cross_at >= 0.0 && now >= lr.cross_at + 0.5 { begin_cross(g, gate); }
        else if lr.phase == GatePhase::Done { return release_gate(g, gd, gate); }
    }
}

/// The charge has left the hand: heads down and away till it goes.
fn toss(g: &mut Game, gd: GuardId, gate: &mut Gate, dt: f64) {
    let (c, d, now) = (g.gd[gd].ch, gate.door, g.time);
    stand_gate(g, gd, dt, Some(gate.face));
    let pos = g.ch[c].pos;
    gate.look = Some(away_look(g, d, gate, pos)); gate.crouch = 0.7;
    let it = gate.can.unwrap();
    let landed = g.tab.item[it].bounces > 0.0 || g.tab.item[it].settled;
    if landed && gate.land_t < 0.0 { gate.land_t = now; }
    if (gate.land_t >= 0.0 && now - gate.land_t >= TOSS_FUSE) || now - gate.toss_t > TOSS_AIR_CAP {
        g.tab.item[it].fired = true;
        crate::combat::detonate_stun(g, it, None, None);
        // (their own charge: nobody on their side shouts 'flashbang!' at it)
        let mut i = g.events.len();
        while i > 0 {
            i -= 1;
            let e = &mut g.events[i];
            if e.kind == crate::director::EventKind::Bang && e.time == now { e.announced = Some(true); break; }
        }
        g.tab.tactics_danger_rec[gate.rec].banged = true;
        gate.bang_t = now; gate.phase = GatePhase::Bang; 
    }
}

/// It has gone off: a beat, and in.
fn bang(g: &mut Game, gd: GuardId, gate: &mut Gate, dt: f64, inside: V3) {
    let now = g.time;
    stand_gate(g, gd, dt, Some(gate.face)); gate.look = Some(inside); gate.crouch = 0.4;
    if now - gate.bang_t >= GO_AFTER_BANG { crate::game::game_say(g, gd, "going in", None, None); begin_cross(g, gate); }
}

/// Across the opening station by station, muzzle into the room, a quarter second on each.
fn slice(g: &mut Game, gd: GuardId, gate: &mut Gate, dt: f64) {
    let c = g.gd[gd].ch;
    gate.crouch = 0.35;
    let i = m::min(gate.slice_i, gate.slice_pts.len() as f64 - 1.0) as usize;
    let (p, look) = (gate.slice_pts[i], gate.slice_look[i]);
    gate.look = Some(look);
    let pos = g.ch[c].pos;
    let face_in = v3::yaw_to(pos, look);
    let there = step_gate(g, gd, gate, dt, p, SLICE_PACE, Some(face_in));
    if there { gate.slice_t += dt; if gate.slice_t >= SLICE_DWELL { gate.slice_i += 1.0; gate.slice_t = 0.0; } }
    if gate.slice_i >= gate.slice_pts.len() as f64 {
        if gate.goal_near { return release_gate(g, gd, gate); }
        let line: &str = if gate_mate(g, gate, crate::tactics::GateRole::Second, gd).is_some() {
            "clear as far as I can see — crossing, on me"
        } else { "crossing" };
        crate::game::game_say(g, gd, line, None, None);
        begin_cross(g, gate);
    }
}

/// Fast through the funnel and off it to his side.
fn cross(g: &mut Game, gd: GuardId, gate: &mut Gate, dt: f64, inside: V3) {
    let (c, d, now) = (g.gd[gd].ch, gate.door, g.time);
    let w = g.dr[d].def.width;
    gate.crouch = 0.0;
    if now < gate.cross_at { stand_gate(g, gd, dt, Some(gate.face)); gate.look = Some(inside); return; }
    let li = m::min(gate.leg_i, gate.legs.len() as f64 - 1.0);
    let leg = if li >= 0.0 { gate.legs.get(li as usize).copied() } else { None };
    let Some(leg) = leg else {
        let pos = g.ch[c].pos;
        gate.crossed = crate::handleside::side_at(g, d, pos) != gate.side;
        release_gate(g, gd, gate);
        return;
    };
    gate.look = Some(if gate.leg_i + 1.0 < gate.legs.len() as f64 {
        let next = gate.legs[(gate.leg_i + 1.0) as usize];
        [next[0], 1.1, next[2]]
    } else {
        let a = along_of(g, d, leg);
        door_pt(g, d, a + (if a > w / 2.0 { 1.5 } else { -1.5 }), -3.0, gate.side, Some(1.0))
    });
    let speed = g.tune.guard_chase;
    let there = step_gate(g, gd, gate, dt, leg, speed, None);
    let pos = g.ch[c].pos;
    if crate::handleside::side_at(g, d, pos) != gate.side { gate.crossed = true; }
    if there || dist_xz(g.ch[c].pos, leg) < 0.35 {
        gate.leg_i += 1.0;
        if gate.leg_i >= gate.legs.len() as f64 { return release_gate(g, gd, gate); }
    } else if gate.t > 3.5 { return release_gate(g, gd, gate); }
}

/// Eyes and muzzle down and away from the opening: the foot of the wall behind him along it.
fn away_look(g: &crate::game::Game, d: DoorId, gate: &crate::tactics::Gate, pos: V3) -> V3 {
    let f = crate::guards::frame_of(g, d);
    let out = if gate.near_a <= 0.0 { -1.0 } else { 1.0 };
    [pos[0] + f.ux * out * 1.4 + f.nx * gate.side * 0.4, 0.15, pos[2] + f.uz * out * 1.4 + f.nz * gate.side * 0.4]
}

/// Across the opening, or straight to the cross where there is no floor for it or the leaf is shut.
fn begin_slice(g: &crate::game::Game, gate: &mut crate::tactics::Gate) {
    let st = slice_stations(g, gate.door, gate.side, gate.near_a);
    let st = match st {
        Some(st) if !crate::doors::door_is_closed(g, gate.door) => st,
        _ => {
            if gate.goal_near { gate.phase = GatePhase::Done; return; }
            begin_cross(g, gate);
            return;
        }
    };
    gate.slice_pts = st.pts; gate.slice_look = st.look; gate.slice_i = 0.0; gate.slice_t = 0.0; gate.phase = GatePhase::Slice; gate.t = 0.0; gate.path = None;
}

/// The legs through the doorway; a second goes half a second behind.
fn begin_cross(g: &crate::game::Game, gate: &mut crate::tactics::Gate) {
    let k = if gate.role == GateRole::Lead { 0.0 } else { 1.0 };
    let plan = plan_for(g, gate.door, gate.side, gate.near_a);
    gate.legs = entry_legs(g, gate.door, gate.side, gate.near_a, k, plan.as_ref());
    gate.leg_i = 0.0; gate.phase = GatePhase::Cross; gate.t = 0.0; gate.path = None;
    gate.cross_at = g.time + (if gate.role == GateRole::Lead { 0.0 } else { 0.5 });
}

/// The charge goes in round the near jamb, on a fuse only this gate's clock runs.
fn begin_toss(g: &mut crate::game::Game, gd: GuardId, gate: &mut crate::tactics::Gate, thrower: GuardId) {
    let d = gate.door;
    if let Some(k) = g.m.tactics.kit.get_mut(thrower) { k.stuns = m::max(0.0, k.stuns - 1.0); }
    let ch = g.gd[thrower].ch;
    let dir = if gate.near_a <= 0.0 { 1.0 } else { -1.0 };
    let w = g.dr[d].def.width;
    let from = door_pt(g, d, gate.near_a + dir * 0.35, 0.3, gate.side, Some(1.1));   // his arm round the near jamb: a lob from the stack mark meets the wall
    let land = door_pt(g, d, w / 2.0 + dir * 0.2, -TOSS_IN_M, gate.side, Some(0.0));
    let sol = crate::throwables::throwables_solve(g, from, land, Some(9.0));
    let who = g.ch[ch].id;
    // (fuse 99: it goes off on OUR clock, TOSS_FUSE after it lands, not the item's)
    let it = crate::throwables::throwables_spawn(g, crate::throwables::ItemKind::Flash, from, sol.v0, Some(crate::throwables::ThrowablesSpawnOpts {
        fuse: Some(99.0), who: Some(who), heading: Some(v3::yaw_of(sol.v0)), ..Default::default()
    }));
    g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::Can);
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Throw, Some(from), Some(0.7), None);
    crate::game::game_say(g, thrower, "bang out", None, None);
    if thrower != gd {
        let row = s(g).gates.get(thrower).cloned();
        if let Some(row) = row {
            let pos = g.ch[ch].pos;
            let look = away_look(g, d, &row, pos);
            s(g).gates.get_mut(thrower).unwrap().look = Some(look);
        }
    }
    gate.can = Some(it); gate.toss_t = g.time; gate.land_t = -1.0; gate.phase = GatePhase::Toss; gate.t = 0.0;
}

/// Keep the gates honest once a frame (a man reset out of the list, a corpse); the phases run from followPath.
pub(super) fn tick_gates(g: &mut crate::game::Game) {
    let men = s(g).gates.keys();
    for man in men {
        if !g.guards.contains(&man) || crate::condition::down(g, g.gd[man].ch) || g.gd[man].held.is_some() {
            let row = s(g).gates.delete(man);
            if let Some(mut row) = row { loose_can(g, &mut row); }
        }
    }
}

/// A gated man's eyes and muzzle and how low he is; None for everybody else.
pub fn tactics_look(g: &crate::game::Game, gd: GuardId) -> Option<crate::tactics::TacticsLook> {
    if !g.tune.tactics { return None; }
    let row = g.m.tactics.states.as_ref()?.gates.get(gd)?;
    Some(TacticsLook { aim: row.look?, crouch: row.crouch })
}

