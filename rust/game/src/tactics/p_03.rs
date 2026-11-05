//! src/game/tactics.ts lines 875 to 1038
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

/// One frame of a man's gate.   (tactics.ts runGate)
pub fn run_gate(g: &mut crate::game::Game, gd: GuardId, g_2: &mut crate::tactics::Gate, dt: f64) -> bool {
    use crate::game::GuardState;
    let c = g.gd[gd].ch;
    let d = g_2.door;
    let now = g.time;
    let w = g.dr[d].def.width;
    let state = g.gd[gd].state;
    let hunting = state == GuardState::Alert || state == GuardState::Search || state == GuardState::Suspicious || (state == GuardState::Patrol && g.gd[gd].drawn);
    // (a release from in here still answers TRUE for the frame: followPath must not read the emptied path as 'arrived')
    if in_contact(g, gd) { release_gate(g, gd, g_2, "eyes on"); return true; }
    if !hunting { let why = format!("now {}", state.as_str()); release_gate(g, gd, g_2, &why); return true; }
    if now - g_2.t0 > GATE_CAP_SECS { release_gate(g, gd, g_2, "out of time"); return true; }
    if g.tab.tactics_danger_rec[g_2.rec].until <= now && (g_2.phase == GatePhase::Stack || g_2.phase == GatePhase::Wait || g_2.phase == GatePhase::Stacked) { release_gate(g, gd, g_2, "the doorway is plain again"); return true; }
    // the drill's lanes: asked each frame until he commits
    if g_2.phase == GatePhase::Stack || g_2.phase == GatePhase::Wait || g_2.phase == GatePhase::Stacked || g_2.phase == GatePhase::Open || g_2.phase == GatePhase::Slice {
        let lead_row = match (g_2.role, g_2.lead) { (GateRole::Second, Some(l)) => gate_read(g, l), _ => None };
        let mate = if g_2.role == GateRole::Lead { gate_mate(g, g_2, GateMateRole::Second, gd) } else { None };
        let going = match lead_row { Some(lg) => lg.phase == GatePhase::Toss || lg.phase == GatePhase::Bang || lg.phase == GatePhase::Cross || (lg.action == GateAction::Toss && lg.thrower == Some(gd)), None => false };
        let can = stuns_of(g, gd) > 0.0
            || (match &mate { Some(mm) => stuns_of(g, mm.gd) > 0.0, None => false })
            || (match (lead_row, g_2.lead) { (Some(_), Some(l)) => stuns_of(g, l) > 0.0, _ => false });
        let at = if g_2.phase == GatePhase::Slice && !g_2.slice_pts.is_empty() {
            g_2.slice_pts[m::min(g_2.slice_i, g_2.slice_pts.len() as f64 - 1.0) as usize]
        } else if g_2.phase == GatePhase::Open {
            match g_2.hs { Some(h) => h.pos, None => g_2.spot }
        } else { g_2.spot };
        if !going {
            let far = door_pt(g, d, w / 2.0, 1.2, -g_2.side, None);
            let centre = g.dr[d].frame_centre;
            if crate::lanes::lane_drill(g, gd, at, far, centre, can) { release_gate(g, gd, g_2, "a lane his side knows of is on it"); return true; }
        }
    }
    // (standing here on purpose: the branches' no-headway watchdogs are not to read it as a wedge)
    g.gd[gd].stall_ref = None; g.gd[gd].alert_stall_t = 0.0; g.gd[gd].chase_t = 0.0;
    g_2.t += dt;
    let inside = door_pt(g, d, w / 2.0, -1.0, g_2.side, Some(1.1));   // what a stacked man's muzzle rests on
    // a second whose lead has gone (dead, released, fighting) leads from where he is
    if g_2.role == GateRole::Second {
        let gone = match g_2.lead { None => true, Some(l) => !s(g).gates.has(l) || g.gd[l].state == GuardState::Dead };
        if gone { g_2.role = GateRole::Lead; g_2.lead = None; if g_2.phase == GatePhase::Stacked { g_2.phase = GatePhase::Wait; } }
    }
    match g_2.phase {
        GatePhase::Stack => {
            g_2.look = Some(inside); g_2.crouch = 0.0;
            let spot = g_2.spot;
            let face = if flat(g.ch[c].pos, spot) < 1.2 { Some(g_2.face) } else { None };
            let speed = g.tune.guard_investigate;
            let there = step_gate(g, gd, g_2, dt, spot, speed, face);
            if there || g_2.t > 5.0 {
                g_2.t = 0.0;
                if g_2.role == GateRole::Lead { gate_call(g, gd, g_2); g_2.phase = GatePhase::Wait; }
                else { g_2.phase = GatePhase::Stacked; }
            }
            true
        }
        GatePhase::Wait => {   // on the jamb, low, muzzle on the opening, until a second man is stacked behind him or doorWait is up
            stand_gate(g, gd, dt, Some(g_2.face)); g_2.look = Some(inside); g_2.crouch = 0.5;
            let mate = gate_mate(g, g_2, GateMateRole::Second, gd);
            let set = match &mate { Some(mm) => mm.g.phase == GatePhase::Stacked, None => false };
            if set || g_2.t >= g.tune.door_wait {
                g_2.thrower = if stuns_of(g, gd) > 0.0 { Some(gd) } else {
                    match &mate { Some(mm) if set && stuns_of(g, mm.gd) > 0.0 => Some(mm.gd), _ => None }
                };
                g_2.action = if g_2.thrower.is_some() { GateAction::Toss } else { GateAction::Slice };
                g_2.phase = GatePhase::Open; g_2.t = 0.0;
            }
            true
        }
        GatePhase::Open => {   // a shut leaf: his hand on the lever, shoved into the room and the closer held off; then the charge or the slice
            // (`G.hs !== null` needs no third state: a null is only ever written on the frame `opened` goes true, below, with no return between)
            if !g_2.opened && crate::doors::door_is_closed(g, d) {
                // (handleside.ts) the lever is worked from the handle's own place
                if g_2.hs.is_none() {
                    let pos = g.ch[c].pos;
                    let h = if crate::doors::doors_pull_face(g, d, pos) { crate::handleside::tool_spot(g, d, g_2.side, pos, None, None) } else { None };
                    g_2.hs = match h { Some(h) if !h.far => Some(GateHs { pos: h.pos, yaw: h.yaw }), _ => None };
                }
                if let Some(h) = g_2.hs {
                    let speed = g.tune.guard_investigate;
                    let stepped = step_gate(g, gd, g_2, dt, h.pos, speed, Some(h.yaw)) && crate::handleside::tool_step(g, gd, h.pos);
                    if !stepped && g_2.t < 3.0 { g_2.look = Some(inside); g_2.crouch = 0.4; return true; }
                    g_2.near_a = w; g_2.spot = h.pos; g_2.face = h.yaw; g_2.t = 0.0;
                }
                g_2.hs = None;
            }
            stand_gate(g, gd, dt, Some(g_2.face)); g_2.look = Some(inside); g_2.crouch = 0.4;
            if !g_2.opened {
                g_2.opened = true;
                if crate::doors::door_is_closed(g, d) {
                    let (pos, id, keys) = (g.ch[c].pos, g.ch[c].id, g.gd[gd].keys.clone());
                    crate::doors::doors_use(g, d, pos, id, Some(&keys));
                    let q = m::min(crate::doors::door_quiet_t(g, d), -6.0);
                    crate::doors::door_set_quiet_t(g, d, q);
                    g_2.t = 0.0;
                } else { g_2.t = 1.0; }
            }
            if g_2.t >= 0.8 {
                g_2.t = 0.0;
                let toss_by = match (g_2.action, g_2.thrower) { (GateAction::Toss, Some(t)) if g.gd[t].state != GuardState::Dead && !crate::doors::door_is_closed(g, d) => Some(t), _ => None };
                match toss_by { Some(t) => begin_toss(g, gd, g_2, t), None => begin_slice(g, gd, g_2) }
            }
            true
        }
        GatePhase::Stacked => {   // the second: tucked in behind the lead, muzzle over him on the opening; goes when he goes
            stand_gate(g, gd, dt, Some(g_2.face)); g_2.crouch = 0.3;
            let lead = match g_2.lead { Some(l) => gate_read(g, l), None => None };
            g_2.look = Some(match lead {
                Some(lr) if lr.phase == GatePhase::Toss || lr.phase == GatePhase::Bang => { let pos = g.ch[c].pos; away_look(g, d, g_2, pos) }
                _ => inside,
            });
            if let Some(lr) = lead {
                if lr.phase == GatePhase::Cross && lr.cross_at >= 0.0 && now >= lr.cross_at + 0.5 { begin_cross(g, gd, g_2); }
                else if lr.phase == GatePhase::Done { release_gate(g, gd, g_2, "the lead is through"); return true; }
            }
            true
        }
        GatePhase::Toss => {   // the charge has left the hand: heads down and away till it goes
            stand_gate(g, gd, dt, Some(g_2.face));
            let pos = g.ch[c].pos;
            g_2.look = Some(away_look(g, d, g_2, pos)); g_2.crouch = 0.7;
            let it = g_2.can.unwrap();
            let landed = crate::throwables::item_bounces(g, it) > 0.0 || crate::throwables::item_settled(g, it);
            if landed && g_2.land_t < 0.0 { g_2.land_t = now; }
            if (g_2.land_t >= 0.0 && now - g_2.land_t >= TOSS_FUSE) || now - g_2.toss_t > TOSS_AIR_CAP {
                crate::throwables::item_set_fired(g, it, true);
                crate::combat::detonate_stun(g, it, None, None);
                // (their own charge: nobody on their side shouts 'flashbang!' at it)
                let mut i = g.events.len();
                while i > 0 {
                    i -= 1;
                    let e = &mut g.events[i];
                    if e.kind == crate::director::EventKind::Bang && e.time == now { e.announced = Some(true); break; }
                }
                g.tab.tactics_danger_rec[g_2.rec].banged = true;
                g_2.bang_t = now; g_2.phase = GatePhase::Bang; g.stats.tactics.bangs += 1;
            }
            true
        }
        GatePhase::Bang => {
            stand_gate(g, gd, dt, Some(g_2.face)); g_2.look = Some(inside); g_2.crouch = 0.4;
            if now - g_2.bang_t >= GO_AFTER_BANG { crate::game::game_say(g, gd, "going in", None, None, None); begin_cross(g, gd, g_2); }
            true
        }
        GatePhase::Slice => {   // across the opening station by station, muzzle into the room, a quarter second on each
            g_2.crouch = 0.35;
            let i = m::min(g_2.slice_i, g_2.slice_pts.len() as f64 - 1.0) as usize;
            let (p, look) = (g_2.slice_pts[i], g_2.slice_look[i]);
            g_2.look = Some(look);
            let pos = g.ch[c].pos;
            let face_in = m::atan2(look[0] - pos[0], look[2] - pos[2]);
            let there = step_gate(g, gd, g_2, dt, p, SLICE_PACE, Some(face_in));
            if there { g_2.slice_t += dt; if g_2.slice_t >= SLICE_DWELL { g_2.slice_i += 1.0; g_2.slice_t = 0.0; } }
            if g_2.slice_i >= g_2.slice_pts.len() as f64 {
                if g_2.goal_near { release_gate(g, gd, g_2, "sliced — his mark is this side of it"); return true; }
                let line: &str = if gate_mate(g, g_2, GateMateRole::Second, gd).is_some() {
                    crate::words::wg(g, &"clear as far as I can see — crossing, on me", &"clear as far as I see — crossing, on me")
                } else { "crossing" };
                crate::game::game_say(g, gd, line, None, None, None);
                begin_cross(g, gd, g_2);
            }
            true
        }
        GatePhase::Cross => {   // fast through the funnel and off it to his side
            g_2.crouch = 0.0;
            if now < g_2.cross_at { stand_gate(g, gd, dt, Some(g_2.face)); g_2.look = Some(inside); return true; }
            let li = m::min(g_2.leg_i, g_2.legs.len() as f64 - 1.0);
            let leg = if li >= 0.0 { g_2.legs.get(li as usize).copied() } else { None };
            let Some(leg) = leg else {
                let pos = g.ch[c].pos;
                g_2.crossed = side_of(g, d, pos) != g_2.side;
                release_gate(g, gd, g_2, "no leg to run");
                return true;
            };
            g_2.look = Some(if g_2.leg_i + 1.0 < g_2.legs.len() as f64 {
                let next = g_2.legs[(g_2.leg_i + 1.0) as usize];
                [next[0], 1.1, next[2]]
            } else {
                let a = along_of(g, d, leg);
                door_pt(g, d, a + (if a > w / 2.0 { 1.5 } else { -1.5 }), -3.0, g_2.side, Some(1.0))
            });
            let speed = g.tune.guard_chase;
            let there = step_gate(g, gd, g_2, dt, leg, speed, None);
            let pos = g.ch[c].pos;
            if side_of(g, d, pos) != g_2.side { g_2.crossed = true; }
            if there || flat(g.ch[c].pos, leg) < 0.35 {
                g_2.leg_i += 1.0;
                if g_2.leg_i >= g_2.legs.len() as f64 { release_gate(g, gd, g_2, "in"); return true; }
            } else if g_2.t > 3.5 { release_gate(g, gd, g_2, "the entry leg went nowhere"); return true; }
            true
        }
        GatePhase::Done => { release_gate(g, gd, g_2, "done"); true }
    }
}

/// Eyes and muzzle down and away from the opening: the foot of the wall behind him along it.   (tactics.ts awayLook)
pub fn away_look(g: &mut crate::game::Game, d: DoorId, g_2: &crate::tactics::Gate, pos: V3) -> V3 {
    let f = crate::guards::frame_of(g, d);
    let out = if g_2.near_a <= 0.0 { -1.0 } else { 1.0 };
    [pos[0] + f.ux * out * 1.4 + f.nx * g_2.side * 0.4, 0.15, pos[2] + f.uz * out * 1.4 + f.nz * g_2.side * 0.4]
}

/// Across the opening, or straight to the cross where there is no floor for it or the leaf is shut.   (tactics.ts beginSlice)
pub fn begin_slice(g: &mut crate::game::Game, gd: GuardId, g_2: &mut crate::tactics::Gate) {
    let st = slice_stations(g, g_2.door, g_2.side, g_2.near_a);
    let st = match st {
        Some(st) if !crate::doors::door_is_closed(g, g_2.door) => st,
        _ => {
            if g_2.goal_near { g_2.phase = GatePhase::Done; return; }
            begin_cross(g, gd, g_2);
            return;
        }
    };
    g_2.slice_pts = st.pts; g_2.slice_look = st.look; g_2.slice_i = 0.0; g_2.slice_t = 0.0; g_2.phase = GatePhase::Slice; g_2.t = 0.0; g_2.path = None;
    g.stats.tactics.slices += 1;
}

/// The legs through the doorway; a second goes half a second behind.   (tactics.ts beginCross)
pub fn begin_cross(g: &mut crate::game::Game, gd: GuardId, g_2: &mut crate::tactics::Gate) {
    let k = if g_2.role == GateRole::Lead { 0.0 } else { 1.0 };
    let plan = plan_for(g, g_2.door, g_2.side, g_2.near_a);
    g_2.legs = entry_legs(g, g_2.door, g_2.side, g_2.near_a, k, plan.as_ref());
    g_2.leg_i = 0.0; g_2.phase = GatePhase::Cross; g_2.t = 0.0; g_2.path = None;
    g_2.cross_at = g.time + (if g_2.role == GateRole::Lead { 0.0 } else { 0.5 });
}

/// The charge goes in round the near jamb, on a fuse only this gate's clock runs.   (tactics.ts beginToss)
pub fn begin_toss(g: &mut crate::game::Game, gd: GuardId, g_2: &mut crate::tactics::Gate, thrower: GuardId) {
    let d = g_2.door;
    if let Some(k) = g.m.tactics.kit.get_mut(thrower) { k.stuns = m::max(0.0, k.stuns - 1.0); }
    let ch = g.gd[thrower].ch;
    let dir = if g_2.near_a <= 0.0 { 1.0 } else { -1.0 };
    let w = g.dr[d].def.width;
    let from = door_pt(g, d, g_2.near_a + dir * 0.35, 0.3, g_2.side, Some(1.1));   // his arm round the near jamb: a lob from the stack mark meets the wall
    let land = door_pt(g, d, w / 2.0 + dir * 0.2, -TOSS_IN_M, g_2.side, Some(0.0));
    let sol = crate::throwables::throwables_solve(g, from, land, Some(9.0));
    let who = g.ch[ch].id;
    // (fuse 99: it goes off on OUR clock, TOSS_FUSE after it lands, not the item's)
    let it = crate::throwables::throwables_spawn(g, crate::throwables::ItemKind::Flash, from, sol.v0, Some(crate::throwables::ThrowablesSpawnOpts {
        fuse: Some(99.0), who: Some(who), heading: Some(v3::yaw_of(sol.v0)), ..Default::default()
    }));
    g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::Can);
    crate::audio_iface::game_audio_like_play(g, "throw", Some(from), Some(0.7), None);
    crate::game::game_say(g, thrower, "bang out", None, None, None);
    if thrower != gd {
        let row = s(g).gates.get(thrower).cloned();
        if let Some(row) = row {
            let pos = g.ch[ch].pos;
            let look = away_look(g, d, &row, pos);
            s(g).gates.get_mut(thrower).unwrap().look = Some(look);
        }
    }
    g_2.can = Some(it); g_2.toss_t = g.time; g_2.land_t = -1.0; g_2.phase = GatePhase::Toss; g_2.t = 0.0;
}

/// Keep the gates honest once a frame (a man reset out of the list, a corpse); the phases run from followPath.   (tactics.ts tickGates)
pub fn tick_gates(g: &mut crate::game::Game) {
    let men = s(g).gates.keys();
    for man in men {
        if !g.guards.contains(&man) || g.gd[man].state == crate::game::GuardState::Dead || g.gd[man].held.is_some() {
            let row = s(g).gates.delete(man);
            if let Some(mut row) = row { loose_can(g, &mut row); }
        }
    }
}

/// A gated man's eyes and muzzle and how low he is; None for everybody else.   (tactics.ts tacticsLook)
pub fn tactics_look(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::tactics::TacticsLook> {
    if !g.tune.tactics { return None; }
    let row = g.m.tactics.states.as_ref()?.gates.get(gd)?;
    Some(TacticsLook { aim: row.look?, crouch: row.crouch })
}

/// Number.prototype.toFixed(1): as format's, except that an exact tie takes the larger magnitude (format's goes to even).
fn to_fixed_1(x: f64) -> String {
    if x.is_nan() { return String::from("NaN"); }
    if x.is_infinite() { return String::from(if x > 0.0 { "Infinity" } else { "-Infinity" }); }
    let (sign, a) = if x < 0.0 { ("-", -x) } else { ("", m::abs(x)) };   // (-0 prints as 0)
    let four = a * 4.0;
    if m::floor(four) == four && js::rem(four, 2.0) == 1.0 {   // an odd number of quarters: the only exact ties at one digit
        let tenths = a * 10.0 + 0.5;
        let whole = m::floor(tenths / 10.0);
        return format!("{}{}.{}", sign, whole, tenths - whole * 10.0);
    }
    format!("{}{:.1}", sign, a)
}

/// One line: the plans, the lone men, the dangerous doorways, the men on a gate. '' when nothing is up.   (tactics.ts tacticsSummary)
pub fn tactics_summary(g: &mut crate::game::Game) -> String {
    let Some(st) = g.m.tactics.states.as_ref() else { return String::from(if g.tune.tactics { "" } else { "tactics ▸ off" }); };
    let mut parts: Vec<String> = Vec::new();
    if !g.tune.tactics { parts.push(String::from("OFF")); }
    for &pid in st.plans.iter() {
        let p = &g.tab.tactics_plan[pid];
        let men: Vec<String> = p.members.iter().map(|&man| {
            let r = p.roles.get(man).map(|&rid| &g.tab.tactics_role[rid]);
            let kind = match r { Some(r) => r.kind.as_str(), None => "?" };
            let holding = match r { Some(r) if r.holding => " (holding)", _ => "" };
            let round = match r {
                Some(r) if r.kind == RoleKind::Flanker => format!(" {}° round", m::round(deg_apart(r.bearing, p.presser_bearing))),
                _ => String::new(),
            };
            format!("{} {}{}{}", g.gd[man].callsign, kind, holding, round)
        }).collect();
        parts.push(format!("plan #{} {} on ({}, {}): {}", p.id, p.phase.as_str(), to_fixed_1(p.target[0]), to_fixed_1(p.target[2]), men.join(", ")));
    }
    for (man, l) in st.lone.iter() {
        let what = if l.released { format!("on at a walk ({})", l.released_why) }
            else if l.hold_t > 0.0 { format!("holding {} s{}", to_fixed_1(l.hold_t), if l.called { ", called" } else { "" }) }
            else { String::from("to his corner") };
        parts.push(format!("{} lone: {}", g.gd[man].callsign, what));
    }
    for &rid in st.danger.iter() {
        let r = &g.tab.tactics_danger_rec[rid];
        if r.until > g.time {
            let victim = match &r.victim { Some(v) if !v.is_empty() => format!(" {}", v), _ => String::new() };
            parts.push(format!("{} dangerous for {} ({}{}) {} s{}{}", g.dr[r.door].def.name, r.team.as_str(), r.why.as_str(), victim, m::ceil(r.until - g.time),
                if r.banged { " banged" } else { "" }, if r.entered { " entered" } else { "" }));
        }
    }
    for (man, row) in st.gates.iter() {
        parts.push(format!("{} on the {} door: {} {}", g.gd[man].callsign, g.dr[row.door].def.name, row.role.as_str(), row.phase.as_str()));
    }
    if parts.is_empty() { String::new() } else { format!("tactics ▸ {}", parts.join(" · ")) }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// strings printed by the TypeScript's own toFixed(1) under bun
    #[test]
    fn to_fixed_1_as_the_language() {
        for (x, want) in [(0.25, "0.3"), (0.75, "0.8"), (1.25, "1.3"), (-0.25, "-0.3"), (2.75, "2.8"), (10.25, "10.3"), (0.35, "0.3"), (1.05, "1.1"), (0.0, "0.0"), (-0.0, "0.0"), (-0.04, "-0.0"), (3.0, "3.0"), (-12.34, "-12.3")] {
            assert_eq!(to_fixed_1(x), want, "{}", x);
        }
    }
}
