use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

macro_rules! o { ($g:expr, $op:expr) => { $g.tab.doortools_door_op[$op] } }
macro_rules! thr { ($g:expr, $op:expr) => { $g.tab.doortools_door_op[$op].thr.as_mut().unwrap() } }

/// `detonate_stun` with nobody on its list: the guards' list is emptied around the call and put back as it was, so the stun's blast finds no man
/// to judge (who it hurts is decided by the caller, by which side of the door he is on). A man added to the emptied list during the call would be lost with it.
fn seam_detonate_stun_nobody(g: &mut Game, it: ItemId) {
    let all = core::mem::take(&mut g.guards);
    crate::combat::detonate_stun(g, it, Some(true), None);
    g.guards = all;
}

// ---- small reads
/// `gd.callsign ?? 'a man'`: a man with no callsign has an empty String in Rust
fn callsign_or(g: &Game, gd: GuardId) -> String { if g.gd[gd].callsign.is_empty() { String::from("a man") } else { g.gd[gd].callsign.clone() } }
fn names(g: &Game, men: &[GuardId]) -> String { men.iter().map(|m| g.gd[*m].callsign.clone()).collect::<Vec<String>>().join(", ") }
/// the `floor` closure of pieSet, toThreshold and thrFar's test: floor there and a clear step to it
fn floor_ok(g: &Game, from: V3, p: V3) -> bool { !crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) && !crate::scene_collision::low_blocked(g, from, p) }
/// the `mark` closure: his post of the order, or where he stands
fn mark_of(g: &Game, op: DoorOpId, gd: GuardId) -> V3 { match o!(g, op).post.get(gd) { Some(p) => p.pos, None => { let p = g.pos_of(gd); [p[0], 0.0, p[2]] } } }
/// the `hi` closure: is his mark on the latch half of the opening
fn hi_of(g: &Game, op: DoorOpId, gd: GuardId) -> bool { let d = o!(g, op).door; let w = g.dr[d].def.width; let k = mark_of(g, op, gd); super::along_of(g, d, k) > w / 2.0 }
/// The charge goes: the leaf, the blast, who it hurts by his side of the door, the order behind it.
pub(super) fn door_tools_blow_raw(g: &mut crate::game::Game, c_2: Option<ChargeId>) {
    let Some(c) = c_2 else { return };
    let (d, side) = { let r = &g.tab.doortools_set_charge[c]; (r.door, r.side) };
    let w = g.dr[d].def.width;
    let into = -side;
    let Some(at) = g.groups[g.group].tools.set.iter().position(|x| *x == c) else { return };
    g.groups[g.group].tools.set.remove(at);
    crate::holdfire::hold_fire_order(g, crate::orders::rows::charge::ORDER.id, Some(crate::holdfire::HoldFireOrderO { blown: Some(true), ..Default::default() }));
    let out = door_pt(g, d, w / 2.0, 0.5, side, Some(1.1));
    let inn = door_pt(g, d, w / 2.0, 0.45, into, Some(1.1));
    let who = g.seat_id(&crate::seats::here(g));
    let charge_k = g.k.doorblast.charge_k;
    if crate::doorblast::tear_off(g, d, out, who, Some(charge_k)) { /* gone */ }
    else if g.dr[d].def.steel != Some(true) {
        crate::doors::doors_kick_in(g, d, out, who, Some(true));
        if m::abs(g.dr[d].leaf.angle) < 1.4 {
            let pt = door_pt(g, d, w / 2.0, 0.6, side, None);
            let v = -crate::doors::doors_side(g, d, pt) * g.k.doortools.blow_vel;
            g.dr[d].leaf.vel = v;
        }
        let q = -g.k.doortools.hold_open;
        g.dr[d].leaf.quiet_t = q;
    }
    let charge_r = g.k.doorblast.charge_r;
    if charge_r > 0.0 {
        let at = door_pt(g, d, w / 2.0, 0.15, side, Some(1.1));
        crate::destruction::destruction_blast(g, at, Some(crate::wallbreak::BlastOpts { radius: Some(charge_r), who: Some(who), chaos: Some(false), ..Default::default() }));
    }
    // the blast itself is the stun charge's, with nobody on its list
    let spot = door_pt(g, d, w * 0.7, 0.2, into, Some(0.5));
    let it = crate::throwables::throwables_spawn(g, crate::throwables::ItemKind::Flash, spot, [0.0, 0.0, 0.0], Some(crate::throwables::ThrowablesSpawnOpts { fuse: Some(99.0), who: Some(who), ..Default::default() }));
    g.tab.item[it].fired = true;
    seam_detonate_stun_nobody(g, it);
    if let Some(ii) = g.items.items.iter().position(|x| *x == it) { g.items.items.remove(ii); }
    let (mut knocked, mut stunned, mut backblast) = (0.0_f64, 0.0_f64, 0.0_f64);
    let fc = g.dr[d].frame_centre;
    let mut sam_too = false;
    // (walls and every OTHER shut door stop it; this leaf is what is coming apart)
    let others = crate::ask::Over::but_door(g, d);
    for gd in g.guards.clone() {
        if crate::condition::down(g, g.gd[gd].ch) { continue; }
        let p = g.pos_of(gd);
        let far = super::off_of(g, d, p, side) < 0.0;
        let dist = v3::dist_xz(p, fc);
        let chest: V3 = [p[0], 1.1, p[2]];
        if dist > g.k.doortools.stun_m || crate::scene_collision::solid_blocked_over(g, &others, if far { inn } else { out }, chest, None) { continue; }
        let bb = g.k.doortools.backblast_secs;
        if !far {
            if dist < g.k.doortools.backblast_m {
                door_tools_dazzle(g, gd, bb, inn);
                if g.team(gd).ours() { backblast += 1.0; } else { stunned += 1.0; }
            }
            continue;
        }
        if g.team(gd).ours() && crate::orders::squad_orders_big(g, &crate::seats::here(g)) { door_tools_dazzle(g, gd, bb, inn); backblast += 1.0; continue; }
        if dist < g.k.doortools.knock_m {
            let dir = v3::normalize([p[0] - fc[0], 0.15, p[2] - fc[2]]);
            crate::combat::kill_guard(g, gd, dir, Some(false), Some(crate::game::DownKind::Struck), Some(crate::game::DownBy::Player), crate::game::With::Hand);
            knocked += 1.0;
        } else {
            let (s0, s1, stun_m) = (g.k.doortools.stun_secs[0], g.k.doortools.stun_secs[1], g.k.doortools.stun_m);
            door_tools_dazzle(g, gd, v3::lerp(s0, s1, v3::clamp(dist / stun_m, 0.0, 1.0)), inn);
            let (c, secs) = (g.gd[gd].ch, g.k.doortools.stagger_secs);
            crate::condition::fold(g, c, secs);
            let wp = g.gd[gd].weapon;
            g.fa[wp].burst = None;
            stunned += 1.0;
        }
    }
    // …and Sam, as one of ours: dazzled as a teammate there is, never felled (every player's man: eachSeat)
    crate::seats::each_seat(g, &mut |g: &mut Game| { if blow_seat(g, &others, d, side, fc, inn, out) { sam_too = true; } });
    crate::morale::morale_blast(g, inn, Some(who));
    g.groups[g.group].tools.stats.blown += 1.0;
    g.groups[g.group].tools.stats.backblast += backblast;
    let words = crate::orders::door_words(g, d);
    let mut text = format!("charge blown — {}", words);
    if js::truthy(knocked) { text.push_str(&format!(" · {} down", knocked)); }
    if js::truthy(stunned) { text.push_str(&format!(" · {} stunned", stunned)); }
    if js::truthy(backblast) { text.push_str(&format!(" · {} of yours too close", backblast)); }
    if sam_too { text.push_str(" · you were too close"); }
    let mut i = 0;
    while i < g.groups[g.group].tools.ops.len() {   // the squad that set it goes in behind it
        let e = g.groups[g.group].tools.ops[i];
        i += 1;
        let r = &o!(g, e);
        if r.kind == DoorToolKind::Charge && r.door == d && (r.phase == Phase::Standoff || r.phase == Phase::Armed) {
            let now = g.time;
            let r = &mut o!(g, e);
            r.open_t = now; r.bang_t = now; r.bang_at = Some(inn); r.phase = Phase::Wait; r.t0 = now;
            door_tools_beat(g, e, &format!("blow · {} down {} stunned", knocked, stunned));
        }
    }
}

/// the seat's man and the blast: true where it dazzled him   (the closure handed to eachSeat in blowRaw)
fn blow_seat(g: &mut Game, others: &crate::ask::Over, d: DoorId, side: f64, fc: V3, inn: V3, out: V3) -> bool {
    let pl = g.player;
    if crate::condition::down(g, g.pl[pl].ch) { return false; }
    let p = g.player_pos(pl);
    let far = super::off_of(g, d, p, side) < 0.0;
    let dist = v3::dist_xz(p, fc);
    let reach = if far { g.k.doortools.stun_m } else { g.k.doortools.backblast_m };
    if !(dist < reach) || crate::scene_collision::solid_blocked_over(g, others, if far { inn } else { out }, [p[0], 1.1, p[2]], None) { return false; }
    let bb = g.k.doortools.backblast_secs;
    crate::samstun::dazzle_player(g, bb, inn);
    true
}

/// flash-blind (`detonate_stun`'s own rule for a man with the charge in his open view)
fn door_tools_dazzle(g: &mut crate::game::Game, gd: GuardId, secs: f64, at: V3) {
    let c = g.gd[gd].ch;
    crate::condition::dazzled(g, gd, secs, at, Some(at), "doortools");
    let along = v3::sub(g.ch[c].pos, at);
    crate::character::anim_hit_from(g, c, Some(along));
}

/// the charge on the leaf: a dark slab by the lock with a lit tell (one-frame boxes)
pub(super) fn door_tools_draw_charge(g: &mut crate::game::Game, c_2: ChargeId) {
    let (d, side) = { let r = &g.tab.doortools_set_charge[c_2]; (r.door, r.side) };
    if super::is_open(g, d) { return; }
    let w = g.dr[d].def.width;
    let p = door_pt(g, d, w - 0.22, 0.045, side, Some(1.05));
    let n = door_pt(g, d, 0.0, 1.0, side, None);
    let e = door_pt(g, d, 0.0, 0.0, side, None);
    let yaw = v3::yaw_to(e, n);
    let fl = crate::scene_boxes::box_flag();
    let b = crate::scene_boxes::make_box_kept(g, &crate::scene_boxes::MakeBoxP { c: p, h: [0.07, 0.11, 0.02], yaw: Some(yaw), albedo: Some([0.09, 0.1, 0.08]), flags: Some(fl.dynamic), ..Default::default() });
    g.fx.push(crate::game::GameFxItem { box_: b, ttl: 1e-3 });
    let both = (js::to_i32(fl.dynamic) | js::to_i32(fl.no_shadow)) as f64;
    let b = crate::scene_boxes::make_box_kept(g, &crate::scene_boxes::MakeBoxP { c: [p[0] + (n[0] - e[0]) * 0.022, p[1] + 0.07, p[2] + (n[2] - e[2]) * 0.022], h: [0.012, 0.012, 0.004], yaw: Some(yaw), albedo: Some([0.9, 0.12, 0.08]), emissive: Some([2.2, 0.2, 0.1]), flags: Some(both), ..Default::default() });
    g.fx.push(crate::game::GameFxItem { box_: b, ttl: 1e-3 });
}

/// the next of this man's cuts that still shows HIM floor no muzzle has been across (-1: none)
pub(super) fn door_tools_pie_next(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, s: &crate::doortools::DoorToolsPieNextS) -> f64 {
    let look_m = g.k.doortools.look_m;
    let sr = o!(g, op).search.flatten();
    let eyes = &s.pie.as_ref().unwrap().eyes;
    let lit = g.tab.light[g.gd[gd].flashlight].enabled;
    let tube = crate::nvg::nvg_down(g, gd);
    let mut k = (s.i + 1.0) as usize;
    while k < s.pts.len() {
        let sr = match sr { Some(sr) => sr, None => return k as f64 };
        let e: V3 = [eyes[k][0], 1.55, eyes[k][2]];
        for i in 0..g.tab.doortools_room_search[sr].pts.len() {
            if g.tab.doortools_room_search[sr].looked[i] != 0 { continue; }
            let q = g.tab.doortools_room_search[sr].pts[i];
            let (dx, dz) = (q[0] - e[0], q[2] - e[2]);
            if dx * dx + dz * dz > look_m * look_m || crate::scene_collision::segment_blocked(g, e, q, None, None, None) { continue; }
            if lit || door_tools_thr_lit_cell(g, op, sr, i as f64)
                || (tube && (door_tools_thr_tube_cell(g, op, sr, i as f64) || crate::irlight::ir_shows_at(g, gd, q))) { return k as f64; }
        }
        k += 1;
    }
    -1.0
}

/// the hold is over with a man still mid-cut: he stands square where his hips are, on the middle of what his cut showed
pub(super) fn door_tools_pie_off(g: &mut crate::game::Game, op: DoorOpId) {
    let mut at = 0;
    while let Some(gd) = o!(g, op).slice.next_key(&mut at) {
        let s = o!(g, op).slice.get_mut(gd).unwrap();
        let p = match s.pie.as_mut() { Some(p) => p, None => continue };
        if p.fin || !p.begun { continue; }
        p.fin = true;
        let mid = { let l = &s.looks[s.i as usize]; l[l.len() >> 1] };
        let a = match crate::orders::squad_orders_of(&crate::seats::here(g), gd) { Some(a) => a, None => continue };
        if o!(g, op).mine.get(gd).copied() != Some(a) { continue; }
        let (search, going, there, bent) = { let r = &g.tab.task[a]; (r.search, r.going.clone(), r.there.clone(), r.lean.map_or(false, js::truthy) || r.crouch == Some(true)) };
        if !bent || crate::condition::down(g, g.gd[gd].ch) { continue; }
        let q = g.pos_of(gd);
        let light = super::steps::light_of(&crate::seats::here(g));
        door_tools_assign(g, op, gd, &DoorToolsAssignA { post: Some([q[0], 0.0, q[2]]), face: Some(v3::yaw_to(q, mid)), watch: Some(mid), stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim), light: Some(light), search, going, there, ..Default::default() });
    }
}

fn pie_in(g: &Game, d: DoorId, w: f64, sgn: f64, p: V3) -> f64 { let a = super::along_of(g, d, p); if sgn < 0.0 { a } else { w - a } }
fn pie_at(g: &Game, d: DoorId, w: f64, sgn: f64, side: f64, a: f64, b: f64) -> V3 { door_pt(g, d, if sgn < 0.0 { a } else { w - a }, b, side, None) }

/// a jamb man's stations: `first`, then the cuts of thrPieArc further in, each with floor and a clear step; his row of the slice is set
fn door_tools_pie_set(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, sgn: f64, first: &[V3], off: &[V3]) -> Vec<V3> {
    let (d, side) = (o!(g, op).door, o!(g, op).side);
    let w = g.dr[d].def.width;
    let pull = sgn < 0.0 && door_tools_leaf_ours(g, d, side);
    let lm = if !pull { g.k.doortools.thr_lean_m } else { 0.0 };
    let mut eyes: Vec<V3> = first.to_vec();
    let mut pts: Vec<V3> = Vec::new();
    for (k, e) in first.iter().copied().enumerate() {
        if k == 0 || !js::truthy(lm) { pts.push(e); continue; }
        let inn = pie_in(g, d, w, sgn, e) - lm;
        let b = super::off_of(g, d, e, side);
        let q = pie_at(g, d, w, sgn, side, inn, b);
        let from = pts[k - 1];
        pts.push(if floor_ok(g, from, q) { q } else { e });
    }
    if !pull {
        for row in g.k.doortools.thr_pie_arc.clone() {
            let (a, b0) = (row[0], row[1]);
            let b = b0 * g.k.doortools.slice_back;
            let prev = pts[pts.len() - 1];
            let last = eyes[eyes.len() - 1];
            if a <= pie_in(g, d, w, sgn, last) + 0.05 { continue; }
            let p = pie_at(g, d, w, sgn, side, a - lm, b);
            let apart = g.k.doortools.thr_pie_apart;
            if !floor_ok(g, prev, p) || off.iter().any(|q| v3::dist_xz(*q, p) < apart) { continue; }
            eyes.push(p);
            pts.push(p);
        }
    }
    let f1 = door_pt(g, d, w / 2.0, 1.0, -side, None);
    let f0 = door_pt(g, d, w / 2.0, 0.0, side, None);
    let fwd = v3::sub(f1, f0);
    let i1 = pie_at(g, d, w, sgn, side, 1.0, 0.0);
    let i0 = pie_at(g, d, w, sgn, side, 0.0, 0.0);
    let inw = v3::sub(i1, i0);
    let lean = if js::truthy(lm) { m::sign(inw[0] * -fwd[2] + inw[2] * fwd[0]) } else { 0.0 };
    let mut looks: Vec<Vec<V3>> = Vec::new();
    for (k, e) in eyes.iter().copied().enumerate() {
        let l = door_tools_slice_looks(g, op, e, sgn);
        looks.push(if k == 0 || l.len() < 3 { l } else { vec![l[2], l[1]] });
    }
    let now = g.time;
    let knee = vec![false; eyes.len()];
    o!(g, op).slice.set(gd, DoorToolsDoneItemSliceValue { pts: pts.clone(), looks, i: 0.0, j: 0.0, t: f64::NAN, pie: Some(DoorToolsDoneItemSliceValuePie { eyes, lean: 0.0, end: lean, end_on: false, knee, fin: false, begun: false, at: now, inw: v3::normalize(inw), gun: 0.0, gun_at: -100.0, on: false, to: None }) });
    pts
}

fn looks_through(g: &Game, d: DoorId, side: f64, p: V3, a: f64) -> V3 {
    let deep = g.k.doortools.slice_deep;
    let e = door_pt(g, d, a, 0.0, side, None);
    let (dx, dz) = (e[0] - p[0], e[2] - p[2]);
    let l = js::or(m::hyp2(dx, dz), 1.0);
    [e[0] + dx / l * deep, 1.1, e[2] + dz / l * deep]
}
/// what a station shows through the frame: past the far jamb, the middle, past the near jamb
fn door_tools_slice_looks(g: &crate::game::Game, op: DoorOpId, p: V3, sgn: f64) -> Vec<V3> {
    let (d, side) = (o!(g, op).door, o!(g, op).side);
    let w = g.dr[d].def.width;
    let into = -side;
    let edge = g.k.doortools.slice_edge;
    let near = if sgn < 0.0 { edge } else { w - edge };
    let far = w - near;
    let f = looks_through(g, d, side, p, far);
    let n = looks_through(g, d, side, p, near);
    let mid: V3 = [(f[0] + n[0]) / 2.0, 1.1, (f[2] + n[2]) / 2.0];
    if super::off_of(g, d, mid, side) < -0.3 { vec![f, mid, n] } else { let deep = g.k.doortools.slice_deep; vec![door_pt(g, d, w / 2.0, deep, into, Some(1.1))] }
}

/// this door's leaf comes round onto face `side`: a one-way door worked from the face it opens toward
fn door_tools_leaf_ours(g: &crate::game::Game, d: DoorId, side: f64) -> bool {
    if !crate::doors::doors_swing_of(g, d).one_way { return false; }
    let w = g.dr[d].def.width;
    let p = door_pt(g, d, w / 2.0, 0.6, side, None);
    crate::doors::doors_pull_face(g, d, p)
}

/// a jamb man's holding spot and where the walk to it is asked from
fn door_tools_thr_spot(g: &crate::game::Game, op: DoorOpId, sgn: f64, mark: V3) -> crate::doortools::DoorToolsThrSpot {
    let (d, side) = (o!(g, op).door, o!(g, op).side);
    let w = g.dr[d].def.width;
    let back = g.k.doortools.slice_back;
    let b = g.k.doortools.thr_pull_at[1] * back;
    if sgn < 0.0 && door_tools_leaf_ours(g, d, side) {
        let a0 = g.k.doortools.thr_pull_at[0];
        let q = door_pt(g, d, a0, b, side, None);
        let from = door_pt(g, d, w * 0.8, b, side, None);
        DoorToolsThrSpot { q, from }
    } else {
        let (s0, s1) = (g.k.doortools.thr_still[0], g.k.doortools.thr_still[1]);
        let q = door_pt(g, d, if sgn < 0.0 { -s0 } else { w + s0 }, s1 * back, side, None);
        DoorToolsThrSpot { q, from: mark }
    }
}

/// the far jamb's man, the side of the opening he stands off, and his diagonal
fn door_tools_thr_far(g: &crate::game::Game, op: DoorOpId) -> Option<crate::doortools::DoorToolsThrFar> {
    let (d, lead) = (o!(g, op).door, o!(g, op).lead);
    let fc = g.dr[d].frame_centre;
    let hl = hi_of(g, op, lead);
    let mut row: Vec<(f64, GuardId)> = Vec::new();
    for m_ in o!(g, op).men.clone() {
        if m_ != lead && o!(g, op).post.get(m_).map_or(false, |p| p.lead) && hi_of(g, op, m_) != hl { row.push((v3::dist_xz(mark_of(g, op, m_), fc), m_)); }
    }
    js::sort_by(&mut row, |a, b| a.0 - b.0);
    let far = row.first()?.1;
    let p = mark_of(g, op, far);
    let fs = if hl { -1.0 } else { 1.0 };
    let DoorToolsThrSpot { q, from } = door_tools_thr_spot(g, op, fs, p);
    let here = if floor_ok(g, from, q) { q } else { p };
    Some(DoorToolsThrFar { far, fs, here })
}

/// is this a threshold order on a hostage room whose captor has given his last warning, and the order's own to time
pub(super) fn door_tools_last_call(g: &mut crate::game::Game, op: DoorOpId) -> bool {
    let r = &o!(g, op);
    if !(r.kind == DoorToolKind::Threshold && r.word.is_none() && r.gate.is_none() && r.thr_hold != Some(true)) { return false; }
    let d = r.door;
    crate::mission::last_warning(g, d)
}

fn stack_on(g: &Game, op: DoorOpId, m_: GuardId) -> bool { match o!(g, op).post.get(m_) { Some(p) => v3::dist_xz(g.pos_of(m_), p.pos) <= 0.6, None => false } }
fn stack_off2(g: &Game, op: DoorOpId, m_: GuardId) -> bool {
    let p = match o!(g, op).post.get(m_) { Some(p) => p.pos, None => return false };
    !door_tools_arrived(g, op, m_) && v3::dist_xz(g.pos_of(m_), p) > g.k.doortools.thr_last_m
}
/// the first free place: floor, a clear line from the lead's mark, nobody of the order or else within 0.7, the stack's room
fn stack_free(g: &mut Game, op: DoorOpId, from: V3, c: V3, gd: GuardId, others: &[V3]) -> bool {
    let (d, side) = (o!(g, op).door, o!(g, op).side);
    if crate::scene_collision::nav_grid_is_blocked(g, c[0], c[2]) || crate::scene_collision::low_blocked(g, from, c) { return false; }
    let taken = o!(g, op).men.iter().any(|m_| *m_ != gd && v3::dist_xz(o!(g, op).post.get(*m_).map_or(g.pos_of(*m_), |p| p.pos), c) < 0.7);
    if taken || others.iter().any(|p| v3::dist_xz(*p, c) < 0.7) { return false; }
    crate::stackroom::stack_room_keeps(g, d, side, c)
}
/// a last warning heard with a man on his jamb and another far short of his mark: the stack is made with who is there
pub(super) fn door_tools_last_stack(g: &mut crate::game::Game, op: DoorOpId) {
    use crate::character::{Stance, UpperMode};
    let (d, side) = (o!(g, op).door, o!(g, op).side);
    let w = g.dr[d].def.width;
    let name = crate::orders::door_words(g, d);
    let fc = g.dr[d].frame_centre;
    let mut lead = o!(g, op).lead;
    let mut handed = false;
    if !stack_on(g, op, lead) {
        if !stack_off2(g, op, lead) { return; }
        let mut row: Vec<(f64, GuardId)> = Vec::new();
        for m_ in o!(g, op).men.clone() {
            if m_ != lead && !crate::condition::down(g, g.gd[m_].ch) && o!(g, op).post.get(m_).map_or(false, |p| p.lead) && stack_on(g, op, m_) { row.push((v3::dist_xz(g.pos_of(m_), fc), m_)); }
        }
        js::sort_by(&mut row, |a, b| a.0 - b.0);
        lead = match row.first() { Some(r) => r.1, None => return };
        handed = true;
    }
    let mut late: Vec<GuardId> = Vec::new();
    for m_ in o!(g, op).men.clone() {
        if m_ != lead && !crate::condition::down(g, g.gd[m_].ch) && !door_tools_arrived(g, op, m_) && o!(g, op).post.has(m_) { late.push(m_); }
    }
    if late.is_empty() { return; }
    for m_ in late.iter().copied() { if !stack_off2(g, op, m_) { return; } }   // (a man a step from his mark is waited for as ever)
    o!(g, op).last_stack = Some(true);
    o!(g, op).lead = lead;
    let lp = *o!(g, op).post.get(lead).unwrap();
    let al = super::along_of(g, d, lp.pos);
    let dir = if al > w / 2.0 { 1.0 } else { -1.0 };
    let off = super::off_of(g, d, lp.pos, side);
    let watch: V3 = [fc[0], 1.05, fc[2]];
    let mut others: Vec<V3> = vec![crate::orders::squad_orders_sam_stood(g, &crate::seats::here(g))];
    for x in g.guards.clone() {
        let ours = if crate::orders::squad_orders_sams(&crate::seats::here(g)) { g.team(x).ours() } else { crate::orders::squad_orders_mine(g, &crate::seats::here(g), x) };
        if ours && !crate::condition::down(g, g.gd[x].ch) && !o!(g, op).men.contains(&x) { others.push(g.pos_of(x)); }
    }
    let file = g.k.doortools.bang_file;
    for gd in late.iter().copied() {
        let p = *o!(g, op).post.get(gd).unwrap();
        let a = super::along_of(g, d, g.pos_of(gd));
        if (super::along_of(g, d, p.pos) > w / 2.0) == (al > w / 2.0) { continue; }   // (his mark is on the lead's wall already)
        if if dir > 0.0 { a < -0.2 } else { a > w + 0.2 } { continue; }   // (past the opening already, on his own wall)
        let mut q: Option<V3> = None;
        let mut j = 1;
        while j <= 4 && q.is_none() {
            let c = door_pt(g, d, al + dir * j as f64 * file, off, side, None);
            if stack_free(g, op, lp.pos, c, gd, &others) { q = Some(c); }
            j += 1;
        }
        let q = match q { Some(q) => q, None => continue };
        let np = crate::orders::StackPost { pos: q, face: lp.face, stack_side: lp.stack_side, lead: false, ..p };
        o!(g, op).post.set(gd, np);
        door_tools_assign(g, op, gd, &DoorToolsAssignA { post: Some(q), face: Some(np.face), watch: Some(watch), stance: Some(Stance::HighReady), upper: Some(UpperMode::None), move_: Some(DoorToolsDoneItemLaneBackMove { stance: Stance::None, upper: UpperMode::Aim }), stack_side: Some(np.stack_side), going: format!("stacking · {}", name).into(), there: crate::orders::Doing::new(crate::orders::At::Stacked, format!("stacked · {}", name)), ..Default::default() });
    }
    let mut what = format!("last warning · not waiting for {}", names(g, &late));
    if handed { what.push_str(&format!(" · the door is {}'s", g.gd[lead].callsign)); }
    door_tools_beat(g, op, &what);
    crate::game::game_say(g, lead, "no time. going.", None, None);
}

/// (thrDark, thrLight, thrPieOne: one light on the door, the one man cutting's)
fn thr_lit(g: &Game, gd: GuardId, lead: GuardId) -> bool {
    if crate::nvg::nvg_worn(g, gd) { return false; }
    let n = g.k.doortools.thr_light;
    n >= 2.0 || (n == 1.0 && gd == lead)
}
/// The `out` closure of toThreshold: the step onto the diagonals, from what it captured.
fn thr_run_out(g: &mut Game, op: DoorOpId, out: &ThresholdOut) {
    use crate::character::{Stance, UpperMode};
    let ThresholdOut::Step { lead, f, sl, sf, light, name } = out;
    let (lead, light) = (*lead, *light);
    let k_pace = g.k.doortools.slice_pace;
    let pace = if k_pace > 0.0 { Some(k_pace) } else { None };
    if o!(g, op).men.contains(&lead) {
        let search = thr_lit(g, lead, lead);
        let (p0, l0) = (sl.pts[0], sl.looks[0][0]);
        door_tools_assign(g, op, lead, &DoorToolsAssignA { post: Some(p0), face: Some(v3::yaw_to(p0, l0)), watch: Some(l0), stance: Some(Stance::None), upper: Some(UpperMode::Aim), light: Some(light), search: Some(search), pace, going: if sl.pie.is_some() { format!("slicing · {}", name).into() } else { format!("on the jamb · {}", name).into() }, there: if sl.pts.len() > 1 { format!("slicing · {}", name).into() } else { format!("holding the jamb · {}", name).into() }, ..Default::default() });
    }
    let fr = match f { Some(x) => *x, None => return };
    let far = fr.far;
    if !o!(g, op).men.contains(&far) { return; }
    let search = thr_lit(g, far, lead);
    if let Some(s) = sf {
        let (p0, l0) = (s.pts[0], s.looks[0][0]);
        door_tools_assign(g, op, far, &DoorToolsAssignA { post: Some(p0), face: Some(v3::yaw_to(p0, l0)), watch: Some(l0), stance: Some(Stance::None), upper: Some(UpperMode::Aim), light: Some(light), search: Some(search), pace, going: format!("slicing · {}", name).into(), there: if s.pts.len() > 1 { format!("slicing · {}", name).into() } else { format!("holding the jamb · {}", name).into() }, ..Default::default() });
    } else {
        let l = door_tools_slice_looks(g, op, fr.here, fr.fs);
        let mid = l[l.len() >> 1];
        door_tools_assign(g, op, far, &DoorToolsAssignA { post: Some(fr.here), face: Some(v3::yaw_to(fr.here, mid)), watch: Some(mid), stance: Some(Stance::None), upper: Some(UpperMode::Aim), light: Some(light), search: Some(search), pace, going: format!("on the jamb · {}", name).into(), there: format!("holding the jamb · {}", name).into(), ..Default::default() });
    }
}
/// The leaf is moving and nobody goes in: the two jamb men hold, the rest keep the squad's back
pub fn door_tools_to_threshold(g: &mut crate::game::Game, op: DoorOpId) {
    let (d, side, lead) = (o!(g, op).door, o!(g, op).side, o!(g, op).lead);
    let name = crate::orders::door_words(g, d);
    let t = g.time;
    let light = super::steps::light_of(&crate::seats::here(g));
    let fr = door_tools_thr_far(g, op);
    let far = fr.map(|f| f.far);
    let jamb: Vec<GuardId> = match far { Some(x) => vec![lead, x], None => vec![lead] };
    o!(g, op).rear.clear();
    for m_ in o!(g, op).men.clone() { if !jamb.contains(&m_) { o!(g, op).rear.add(m_); } }
    let hl = hi_of(g, op, lead);
    let sgn = if hl { 1.0 } else { -1.0 };
    let mut pts: Vec<V3> = vec![mark_of(g, op, lead)];
    { let DoorToolsThrSpot { q, from } = door_tools_thr_spot(g, op, sgn, pts[0]); if floor_ok(g, from, q) { pts[0] = q; } }
    let mut looks: Vec<Vec<V3>> = Vec::new();
    for q in pts.iter().copied() { let l = door_tools_slice_looks(g, op, q, sgn); looks.push(vec![l[l.len() >> 1]]); }
    o!(g, op).slice.set(lead, DoorToolsDoneItemSliceValue { pts, looks, i: 0.0, j: 0.0, t: f64::NAN, pie: None });
    // (thrPie, thrPieOne: the lead alone cuts the opening; the far man holds his diagonal)
    let lm = mark_of(g, op, lead);
    let DoorToolsThrSpot { q, from } = door_tools_thr_spot(g, op, sgn, lm);
    let first_eye = if floor_ok(g, from, q) { q } else { lm };
    let off: Vec<V3> = match fr { Some(f) => vec![f.here], None => Vec::new() };
    door_tools_pie_set(g, op, lead, sgn, &[first_eye], &off);
    let sl = o!(g, op).slice.get(lead).unwrap().clone();
    let sf = far.and_then(|x| o!(g, op).slice.get(x).cloned());
    let out = ThresholdOut::Step { lead, f: fr, sl, sf, light, name: name.clone() };
    let first = o!(g, op).thr_pre.is_none() && door_tools_can_first(g, op);
    if !first { thr_run_out(g, op, &out); }
    let n = m::max(2.0, o!(g, op).men.len() as f64);
    let pods = door_tools_shallow_pods(g, d, side, Some(n));
    let mut up: Vec<GuardId> = Vec::new();
    for x in g.guards.iter().copied() { if !crate::condition::down(g, g.gd[x].ch) && g.team(x) == crate::teams::Team::Hostile { up.push(x); } }
    let pod0 = pods.first().map(|p| p.pos);
    let hold = o!(g, op).thr_hold == Some(true);
    o!(g, op).thr = Some(Threshold { jamb, pods, quiet_t: t, ev_t: t, contact: JsSet::new(), up, down: 0.0, sliced: false, thrower: None, tossed: 0.0, next_t: 0.0, aim: None, from: None, banged: None, handed: false, los_t: 0.0, lit_c: None, tube_c: None, hold, told: false, force: false, hand_w: None, hand_t: None, hand_done: None, fled: None, frag_t: None, frag_said: None, step_owed: None, owed_t: None, cut_owed: None, first: if first { Some(ThresholdFirst::Can) } else { None }, out: if first { Some(out) } else { None }, wide: None, early: None, shut_t: None });
    if let Some(p) = pod0 { door_tools_search_of(g, op, Some(p)); }
    o!(g, op).phase = if first { Phase::Pocket } else { Phase::Thr };
    o!(g, op).t0 = t;
    if first {
        let kicked = o!(g, op).kick.map_or(false, |k| !k.t.is_nan());
        let after = if kicked { g.k.doortools.toss_after_kick } else { g.k.doortools.toss_after_open };
        thr!(g, op).next_t = t + after;
    }
    let mut what = format!("threshold · {} jamb {} slices · ", if hl { "latch" } else { "hinge" }, g.gd[lead].callsign);
    match far {
        Some(x) => { let hf = hi_of(g, op, x); what.push_str(&format!("{} jamb {} holds", if hf { "latch" } else { "hinge" }, g.gd[x].callsign)); }
        None => what.push_str(if o!(g, op).last_stack == Some(true) { "the other jamb not waited for" } else { "no floor at the other jamb" }),
    }
    if !o!(g, op).rear.is_empty() { let rear = o!(g, op).rear.keys(); what.push_str(&format!(" · rear {}", names(g, &rear))); }
    if hold { what.push_str(" · to hold"); }
    if first { what.push_str(" · a can first"); }
    door_tools_beat(g, op, &what);
    crate::game::game_say(g, lead, "working it from the door.", None, None);
}

/// does this door get a can before anybody shows in its opening
fn door_tools_can_first(g: &crate::game::Game, op: DoorOpId) -> bool {
    let r = &o!(g, op);
    if g.k.doortools.thr_first <= 0.0 || g.k.doortools.thr_bangs <= 0.0 || g.groups[g.group].tools.bangs <= 0.0 || r.word.is_some() || r.worded == Some(true) || r.gate.is_some() || r.thr_hold == Some(true) { return false; }
    if crate::silent::silent_no_cans(g, &crate::seats::here(g)) { return false; }
    let (d, side) = (o!(g, op).door, o!(g, op).side);
    let men = o!(g, op).men.clone();
    let mut hot = g.k.doortools.thr_first >= 2.0;
    if !hot { for m_ in men.iter().copied() { if !crate::condition::down(g, g.gd[m_].ch) && crate::roomfight::room_fight_hot_for(g, m_) { hot = true; break; } } }
    if !hot { return false; }
    // (one of ours beyond the door inside its reach with a line from the opening, and it stays in the kit)
    let w = g.dr[d].def.width;
    let inn = door_pt(g, d, w / 2.0, 0.45, -side, Some(1.1));
    let mut ours: Vec<V3> = Vec::new();
    for x in g.guards.clone() {
        let mine = if crate::orders::squad_orders_sams(&crate::seats::here(g)) { g.team(x).ours() } else { crate::orders::squad_orders_mine(g, &crate::seats::here(g), x) };
        if mine && !crate::condition::down(g, g.gd[x].ch) && !men.contains(&x) { ours.push(g.pos_of(x)); }
    }
    if !crate::condition::down(g, g.pl[g.player].ch) { ours.push(crate::orders::squad_orders_sam_stood(g, &crate::seats::here(g))); }
    let fc = g.dr[d].frame_centre;
    let stun_m = g.k.doortools.stun_m;
    for p in ours {
        if super::off_of(g, d, p, side) < 0.0 && v3::dist_xz(p, fc) < stun_m && !crate::scene_collision::solid_blocked(g, inn, [p[0], 1.1, p[2]], None) { return false; }
    }
    true
}

/// behind a bang: has the blind burst it drew run dry
pub(super) fn door_tools_burst_dry(g: &crate::game::Game, op: DoorOpId) -> bool {
    let t = g.time;
    let bang_t = o!(g, op).bang_t;
    let since = t - bang_t;
    let mut last = f64::NEG_INFINITY;
    for e in g.events.iter().rev() {
        if e.time < bang_t { break; }
        if e.kind == crate::director::EventKind::GuardShot {
            if let Some(who) = e.who {
                if !g.guards.iter().any(|x| g.team(*x).ours() && g.char_id(*x) == who) { last = e.time; break; }
            }
        }
    }
    (since >= g.k.doortools.after_bang && t - last >= g.k.doortools.quiet) || since >= g.k.doortools.entry_cap
}

/// behind that can (its burst run dry), or with no can thrown after all, the jamb men step out and the hold begins
pub(super) fn door_tools_thr_out(g: &mut crate::game::Game, op: DoorOpId, why: Option<&str>) {
    let why = why.unwrap_or("");
    let t = g.time;
    let banged = thr!(g, op).first == Some(ThresholdFirst::Burst);
    let wide = banged && thr!(g, op).wide == Some(true);
    door_tools_thr_men(g, op);
    thr!(g, op).first = if banged && !wide { Some(ThresholdFirst::Done) } else { None };
    if let Some(out) = thr!(g, op).out.take() { thr_run_out(g, op, &out); }
    if let Some(th) = thr!(g, op).thrower { if !thr!(g, op).jamb.contains(&th) { o!(g, op).rear.add(th); } }   // (the thrower has the squad's back again for the hold)
    let r = thr!(g, op);
    r.thrower = None; r.quiet_t = t; r.next_t = 0.0;
    o!(g, op).phase = Phase::Thr;
    o!(g, op).t0 = t;
    let what = if wide { String::from("out onto the diagonals · the can went off outside") } else if banged { String::from("out onto the diagonals behind the can") } else { format!("no can first · {}", why) };
    door_tools_beat(g, op, &what);
    if thr!(g, op).early == Some(true) {   // (the word, given while the can was in the air)
        let r = thr!(g, op);
        r.early = Some(false); 
        door_tools_beat_is(g, op, crate::doortools::Beat::Word, "the word");
        door_tools_to_pockets(g, op);
    }
}

/// the stage's men, kept true: who has the door when a jamb man is down or re-ordered
pub(super) fn door_tools_thr_men(g: &mut crate::game::Game, op: DoorOpId) {
    use crate::character::{Stance, UpperMode};
    let d = o!(g, op).door;
    let men = o!(g, op).men.clone();
    let lead = o!(g, op).lead;
    let r = thr!(g, op);
    r.jamb.retain(|m_| men.contains(m_));
    if let Some(th) = r.thrower { if !men.contains(&th) { r.thrower = None; } }
    if r.jamb.contains(&lead) {
        if r.jamb[0] != lead { let mut j = vec![lead]; j.extend(r.jamb.iter().copied().filter(|m_| *m_ != lead)); r.jamb = j; }
        return;
    }
    if !r.jamb.is_empty() { let first = r.jamb[0]; o!(g, op).lead = first; return; }
    let gd = lead;
    let p = g.pos_of(gd);
    let name = crate::orders::door_words(g, d);
    thr!(g, op).jamb = vec![gd];
    o!(g, op).rear.delete(gd);
    if o!(g, op).phase == Phase::Enter { return; }
    let fc = g.dr[d].frame_centre;
    let light = super::steps::light_of(&crate::seats::here(g));
    let search = !crate::nvg::nvg_worn(g, gd);
    door_tools_assign(g, op, gd, &DoorToolsAssignA { post: Some([p[0], 0.0, p[2]]), face: Some(v3::yaw_to(p, fc)), watch: Some([fc[0], 1.05, fc[2]]), stance: Some(Stance::None), upper: Some(UpperMode::Aim), light: Some(light), search: Some(search), going: format!("on the door · {}", name).into(), there: format!("holding the door · {}", name).into(), ..Default::default() });
    let what = format!("the door is {}'s", g.gd[gd].callsign);
    door_tools_beat(g, op, &what);
}

/// the `inRoom` closure of thrWatch
fn in_room(g: &Game, room: Option<RoomfightRoomId>, p: V3) -> bool {
    let (cell, w) = (g.col.nav.cell, g.col.nav.w);
    let ci = m::floor(p[2] / cell) * w + m::floor(p[0] / cell);
    match room { Some(r) => g.tab.roomfight_room[r].cells.has(crate::Num(ci)), None => false }
}
/// what the hold listens for: a man at the frame in contact, a defender's round near the door, one of theirs dropping: each puts the quiet clock back
pub(super) fn door_tools_thr_watch(g: &mut crate::game::Game, op: DoorOpId) {
    let d = o!(g, op).door;
    let t = g.time;
    let fc = g.dr[d].frame_centre;
    for gd in thr!(g, op).jamb.clone() {
        if crate::roomfight::room_fight_contact(g, gd) || crate::roomfight::room_fight_engaged(g, gd) {
            thr!(g, op).quiet_t = t;
            if !thr!(g, op).contact.has(gd) {
                thr!(g, op).contact.add(gd);
                let what = format!("contact:{}", g.gd[gd].callsign);
                door_tools_beat(g, op, &what);
            }
        }
    }
    let room = o!(g, op).search.flatten().map(|s| g.tab.doortools_room_search[s].room);
    let ev_t = thr!(g, op).ev_t;
    let shot_m = g.k.doortools.thr_shot_m;
    let mut i = g.events.len();
    while i > 0 {   // (the men fire AFTER this tick in a frame: the last tick's own frame is read again)
        i -= 1;
        let (time, kind, who, pos) = { let e = &g.events[i]; (e.time, e.kind, e.who, e.pos) };
        if time < ev_t { break; }
        if kind == crate::director::EventKind::GuardShot {
            if let Some(who) = who {
                if (v3::dist_xz(pos, fc) < shot_m || in_room(g, room, pos)) && !g.guards.iter().any(|x| g.team(*x).ours() && g.char_id(*x) == who) { thr!(g, op).quiet_t = t; break; }
            }
        }
    }
    thr!(g, op).ev_t = t;
    let mut i = thr!(g, op).up.len();
    while i > 0 {
        i -= 1;
        let h = thr!(g, op).up[i];
        if !crate::condition::down(g, g.gd[h].ch) { continue; }
        thr!(g, op).up.remove(i);
        let p = g.pos_of(h);
        if !(v3::dist_xz(p, fc) < 3.0 || in_room(g, room, p)) { continue; }
        let r = thr!(g, op);
        r.quiet_t = t; r.down += 1.0;
        let what = format!("down from outside · {}", callsign_or(g, h));
        door_tools_beat(g, op, &what);
    }
    // A GRENADE ON THE STACK: who of the order ran from one and is not back yet (thrFragWait)
    let n0 = thr!(g, op).fled.get_or_insert_with(JsSet::new).len();
    let mut came = false;
    let men = o!(g, op).men.clone();
    for m_ in thr!(g, op).fled.as_ref().unwrap().keys() { if !men.contains(&m_) { thr!(g, op).fled.as_mut().unwrap().delete(m_); } }
    for m_ in men.iter().copied() {
        if crate::frag::frag_running(g, m_) { thr!(g, op).fled.as_mut().unwrap().add(m_); }
        else if thr!(g, op).fled.as_ref().unwrap().has(m_) && door_tools_thr_back(g, op, m_) { thr!(g, op).fled.as_mut().unwrap().delete(m_); came = true; }
    }
    let fled = thr!(g, op).fled.as_ref().unwrap().keys();
    if !fled.is_empty() && n0 == 0 { let what = format!("grenade on the stack · {} off the door", names(g, &fled)); door_tools_beat(g, op, &what); }
    if fled.is_empty() && n0 != 0 { door_tools_beat(g, op, if came { "the men are back" } else { "nobody left to wait for" }); }
    if o!(g, op).phase == Phase::Thr && door_tools_thr_frag_wait(g, op) { thr!(g, op).quiet_t = t; }   // (the hold does not count the door quiet meanwhile)
}

/// the cans this stage may spend on its door (outsidePlain: one a door)
pub(super) fn door_tools_thr_cans(g: &crate::game::Game, seat: &crate::seats::SeatRef, t: &crate::doortools::Threshold) -> f64 {
    if !t.force && crate::silent::silent_no_cans(g, seat) { return 0.0; }
    m::min(1.0, g.k.doortools.thr_bangs)
}

/// the hold is over: both jamb men stand in behind their jambs' edges, and the thrower comes round from the rear
pub(super) fn door_tools_to_pockets(g: &mut crate::game::Game, op: DoorOpId) {
    let t = g.time;
    door_tools_held_beat(g, op);
    let bang_t = o!(g, op).bang_t;
    let stale = g.k.doortools.thr_first_stale;
    if thr!(g, op).first == Some(ThresholdFirst::Done) && stale > 0.0 && t - bang_t >= stale && thr!(g, op).banged.is_some() {
        // (the hold is lent to thrCans, which takes it by reference and does not reach the order)
        let th = o!(g, op).thr.take().unwrap();
        let cans = door_tools_thr_cans(g, &crate::seats::here(g), &th);
        let tossed = th.tossed;
        o!(g, op).thr = Some(th);
        if tossed < cans && g.groups[g.group].tools.bangs > 0.0 {
            thr!(g, op).banged = None;
            door_tools_beat(g, op, &format!("the first can is {} s old · its floor is blind again", js::to_fixed(t - bang_t, 1)));
        }
    }
    let th = door_tools_thr_thrower(g, op);
    thr!(g, op).thrower = Some(th);
    o!(g, op).rear.delete(th);
    thr!(g, op).hand_t = None;
    door_tools_close_up(g, op, None);
    for m_ in thr!(g, op).jamb.clone() { if m_ != o!(g, op).lead { door_tools_close_up(g, op, Some(m_)); } }
    o!(g, op).phase = Phase::Pocket;
    o!(g, op).t0 = t;
    let turn = g.k.doortools.thr_turn;
    thr!(g, op).next_t = t + turn;
}

/// the hold's own count, as it ends
pub(super) fn door_tools_held_beat(g: &mut crate::game::Game, op: DoorOpId) {
    door_tools_thr_men(g, op);
    let held = g.time - o!(g, op).t0;
    let r = &o!(g, op);
    let th = r.thr.as_ref().unwrap();
    let mut what = format!("held {} s · {} in contact · {} down", js::to_fixed(held, 1), th.contact.len(), th.down);
    if let Some(sr) = r.search.flatten() { let sr = &g.tab.doortools_room_search[sr]; what.push_str(&format!(" · {} of {} cells unseen", sr.left, sr.pts.len())); }
    door_tools_beat(g, op, &what);
}

/// who throws: the man nearest the frame on neither jamb; with only the jamb men, the far one; alone, himself
pub(super) fn door_tools_thr_thrower(g: &mut crate::game::Game, op: DoorOpId) -> GuardId {
    door_tools_thr_pick(g, op, false, None).unwrap()
}

/// is this man BACK: at the door, or on his own mark of the order
pub(super) fn door_tools_thr_back(g: &crate::game::Game, op: DoorOpId, m: GuardId) -> bool {
    let a = o!(g, op).mine.get(m).copied();
    if door_tools_thr_at(g, op, m) { return true; }
    match a.and_then(|a| g.tab.task[a].post) { Some(p) => v3::dist_xz(g.pos_of(m), p) <= 0.6, None => false }
}

/// (`op.word && this.ops.some(o => o !== op && o.word === op.word)`)
fn word_shared(g: &Game, seat: &crate::seats::SeatRef, op: DoorOpId) -> bool {
    let w = o!(g, op).word;
    w.is_some() && seat.door_tools.ops.iter().any(|e| *e != op && o!(g, *e).word == w)
}
fn last_held(g: &Game, op: DoorOpId) -> bool { o!(g, op).last_stack == Some(true) || o!(g, op).last_hold == Some(true) }

/// Does the order go in its own time: no last warning has hurried it, nobody in there is about to be shot, and no other order goes on its word.
pub(super) fn door_tools_own_time(g: &mut Game, op: DoorOpId) -> bool {
    let d = o!(g, op).door;
    !last_held(g, op) && !crate::mission::last_warning(g, d) && !word_shared(g, &crate::seats::here(g), op)
}

/// is the word's step in waiting for the jamb men's cuts
pub(super) fn door_tools_cuts_owe(g: &mut crate::game::Game, op: DoorOpId) -> bool {
    let r = &o!(g, op);
    let th = r.thr.as_ref().unwrap();
    if th.sliced || th.told || r.phase != Phase::Thr { return false; }
    if !th.jamb.iter().any(|m_| r.slice.get(*m_).map_or(false, |s| s.pie.is_some())) { return false; }
    if th.cut_owed == Some(true) && g.time - th.owed_t.unwrap_or(-1e9) >= g.k.doortools.word_cuts_cap { return false; }
    for c in g.groups[g.group].tools.set.clone() {   // (a set charge goes on this same word: they go in behind its blast)
        if !door_tools_on_pending_word(g, &crate::seats::here(g), c) && door_tools_key_is(g, &crate::seats::here(g), c) { return false; }
    }
    door_tools_own_time(g, op)
}

/// is the stage waiting for men who ran from a grenade
pub(super) fn door_tools_thr_frag_wait(g: &mut crate::game::Game, op: DoorOpId) -> bool {
    let th = o!(g, op).thr.as_ref().unwrap();
    th.fled.as_ref().is_some_and(|f| !f.is_empty()) && th.hand_done != Some(true) && th.hand_w.unwrap_or(0.0) < g.k.doortools.thr_hand_wait && door_tools_own_time(g, op)
}

/// is this order waiting for men who ran from a grenade (Sam's own squad only)
pub(super) fn door_tools_frag_wait(g: &mut crate::game::Game, op: DoorOpId) -> bool {
    let r = &o!(g, op);
    if !(r.fled.as_ref().map_or(false, |f| !f.is_empty()) && r.fled_go != Some(true)) { return false; }
    if !crate::orders::squad_orders_sams(&crate::seats::here(g)) { return false; }
    if !(o!(g, op).fled_w.unwrap_or(0.0) < g.k.doortools.frag_hold_cap) { return false; }
    if !door_tools_frag_holds(g, op) || last_held(g, op) { return false; }
    let d = o!(g, op).door;
    !crate::mission::last_warning(g, d)
}

/// the orders on a word that has been given and is held for men who ran, where these men are on one of its doors
pub fn door_tools_frag_held(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<DoorOpId> {
    let mut held: Vec<WordId> = Vec::new();
    for w in g.groups[g.group].tools.words.clone() {
        if !g.tab.doortools_word[w].go1.is_nan() { continue; }
        if !g.groups[g.group].tools.ops.iter().any(|e| o!(g, *e).word == Some(w) && o!(g, *e).men.iter().any(|m_| men.contains(m_))) { continue; }
        let mut waits = false;
        for e in g.groups[g.group].tools.ops.clone() { if o!(g, e).word == Some(w) && door_tools_frag_wait(g, e) { waits = true; break; } }
        if waits { held.push(w); }
    }
    if held.is_empty() { return Vec::new(); }
    g.groups[g.group].tools.ops.iter().copied().filter(|e| o!(g, *e).word.map_or(false, |w| held.contains(&w))).collect()
}

/// the stages that hold, each while its door is still to be opened
pub(super) fn door_tools_frag_holds(g: &crate::game::Game, op: DoorOpId) -> bool {
    let r = &o!(g, op);
    let (d, k) = (r.door, r.kick);
    match r.phase {
        Phase::Stack => match r.gate { None => true, Some(gt) => g.tab.doortools_door_gate[gt].go },
        Phase::Ready => true,
        Phase::Standoff => door_tools_charge_on(g, &crate::seats::here(g), d).is_some(),
        Phase::Place => { let ran = r.fled.as_ref().map_or(false, |f| f.has(r.lead)); ran && !super::is_open(g, d) }
        Phase::Armed => r.word.is_some() && door_tools_charge_on(g, &crate::seats::here(g), d).is_some(),
        Phase::Kick => match k {
            Some(k) => k.t.is_nan() && k.by.is_none() && !k.gun.map_or(false, |gn| gn.n > 0.0 || gn.next_t != 0.0) && !super::is_open(g, d),
            None => false,
        },
        _ => false,
    }
}

/// back: on his own mark of the order (a man with no mark is back once he has stopped running)
pub(super) fn door_tools_frag_back(g: &crate::game::Game, op: DoorOpId, m: GuardId) -> bool {
    match o!(g, op).mine.get(m).copied().and_then(|a| g.tab.task[a].post) { None => true, Some(p) => v3::dist_xz(g.pos_of(m), p) <= 0.8 }
}

/// the wait that was said is over: said so once, where men are still out
pub(super) fn door_tools_frag_said(g: &mut crate::game::Game, op: DoorOpId) {
    if o!(g, op).fled_said != Some(true) { return; }
    o!(g, op).fled_said = Some(false);
    let fled = match o!(g, op).fled.as_ref() { Some(f) if !f.is_empty() => f.keys(), _ => return };
    let spent = o!(g, op).fled_w.unwrap_or(0.0) >= g.k.doortools.frag_hold_cap;
    let what = format!("on without {} · {}", names(g, &fled), if spent { "the order's wait is spent" } else { "not waited for" });
    door_tools_beat(g, op, &what);
}

/// the order's signal has gone: nothing of it waits from here
pub(super) fn door_tools_frag_on(g: &mut crate::game::Game, op: DoorOpId) {
    door_tools_frag_said(g, op);
    o!(g, op).fled_go = Some(true);
}
