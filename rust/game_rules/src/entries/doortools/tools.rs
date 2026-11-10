use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- small private helpers

/// WEAPONS[w].pellets?.breach
fn weapon_breach(g: &Game, w: crate::weapons::WeaponId) -> Option<crate::weapons::PelletSpecBreach> { g.m.weapons.weapons.get(w).and_then(|d| d.pellets.as_ref()).and_then(|p| p.breach) }

pub(super) fn is_wedge(g: &Game, d: DoorId) -> bool { g.dr[d].lock_id.starts_with("wedge:") }
/// op.post.get(m)?.pos ?? m.char.pos
fn post_or_pos(g: &Game, op: DoorOpId, m: GuardId) -> V3 {
    match g.tab.doortools_door_op[op].post.get(m) { Some(p) => p.pos, None => g.pos_of(m) }
}
/// DOORTOOLS.blowKey.replace('Key', '')
pub(super) fn key_word(g: &Game) -> String { g.k.doortools.blow_key.replacen("Key", "", 1) }
fn by_sam(g: &Game, c: ChargeId) -> bool { g.tab.doortools_set_charge[c].by == crate::ownblow::SAM_BY }
/// this.ops.some(o => o.kind === 'charge' && o.door === d)
fn charge_order_on(g: &Game, seat: &crate::seats::SeatRef, d: DoorId) -> bool {
    seat.door_tools.ops.iter().any(|o| { let x = &g.tab.doortools_door_op[*o]; x.kind == DoorToolKind::Charge && x.door == d })
}
/// Is `p` where a charge on a door that is not `but` throws back: one set on it, or one ordered.
pub(in crate::entries) fn in_anothers_blast(g: &Game, seat: &crate::seats::SeatRef, but: DoorId, p: V3) -> bool {
    let reach = g.k.doortools.backblast_m + 0.2;
    let near = |d: DoorId| d != but && v3::dist_xz(p, g.dr[d].frame_centre) < reach;
    seat.door_tools.set.iter().any(|&c| near(g.tab.doortools_set_charge[c].door)) || g.doors.list.iter().any(|&d| near(d) && charge_order_on(g, seat, d))
}
/// op.word && op.kind !== 'bangClear'
fn on_word(g: &Game, op: DoorOpId) -> bool { let o = &g.tab.doortools_door_op[op]; o.word.is_some() && o.kind != DoorToolKind::BangClear }
/// Write through op.kick if it is still there (a TypeScript write to a kick some call has since dropped is lost: so here).
fn with_kick(g: &mut Game, op: DoorOpId, f: impl FnOnce(&mut DoorToolsDoneItemKick)) { if let Some(k) = g.tab.doortools_door_op[op].kick.as_mut() { f(k); } }
fn with_gun(g: &mut Game, op: DoorOpId, f: impl FnOnce(&mut DoorToolsDoneItemKickGun)) { with_kick(g, op, |k| if let Some(x) = k.gun.as_mut() { f(x); }); }

/// A man's mark under an order, as an assignment: the stack's usual shape.
fn stack_assign(p: &crate::orders::StackPost, watch: V3, name: &str) -> DoorToolsAssignA {
    DoorToolsAssignA {
        post: Some(p.pos), face: Some(p.face), watch: Some(watch), stance: Some(crate::character::Stance::Stack), upper: Some(crate::character::UpperMode::None),
        move_: Some(DoorToolsDoneItemLaneBackMove { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim }), stack_side: Some(p.stack_side),
        going: alloc::format!("stacking · {}", name).into(), there: crate::orders::Doing::new(crate::orders::At::Stacked, alloc::format!("stacked · {}", name)), ..Default::default()
    }
}

/// A mark of this stack stands in the lane of another open door with unknown floor beyond it: the rearmost man covers it first.
pub(super) fn door_tools_lane_cover(g: &mut crate::game::Game, op: DoorOpId) {
    let d = g.tab.doortools_door_op[op].door;
    if g.tab.doortools_door_op[op].men.len() < 2 { return; }
    let (men, lead, fc, width) = { let o = &g.tab.doortools_door_op[op]; (o.men.clone(), o.lead, g.dr[d].frame_centre, g.dr[d].def.width) };
    let mut cand: Vec<(GuardId, f64)> = Vec::new();
    for m in men.iter().copied() {
        let o = &g.tab.doortools_door_op[op];
        if m == lead || o.rear.has(m) { continue; }
        if let Some(p) = o.post.get(m) { cand.push((m, v3::dist_xz(p.pos, fc))); }
    }
    js::sort_by(&mut cand, |a, b| b.1 - a.1);
    let gd = match cand.first() { Some(x) => x.0, None => return };
    let a_id = match g.tab.doortools_door_op[op].mine.get(gd) { Some(a) => *a, None => return };
    let a_post = match g.tab.orders_assignment[a_id].post { Some(p) => p, None => return };
    let mut dd: Option<DoorId> = None;
    for m in men.iter().copied() {
        let p = g.tab.doortools_door_op[op].post.get(m).copied();
        if let Some(p) = p { dd = crate::doorway::doorway_lane(g, p.pos, Some(d)); if dd.is_some() { break; } }
    }
    let dd = match dd { Some(x) => x, None => return };
    let (mut sx, mut sz, mut n) = (0.0, 0.0, 0.0);
    for m in men.iter().copied() {
        if let Some(p) = g.tab.doortools_door_op[op].post.get(m) { sx += p.pos[0]; sz += p.pos[2]; n += 1.0; }
    }
    let c = g.dr[dd].frame_centre;
    let l = m::hyp2(sx / n - c[0], sz / n - c[2]);
    let (cover_m, cover_deep) = (g.k.doormanners.cover_m, g.k.doormanners.cover_deep);
    if l < cover_m + 0.5 { return; }
    {
        let s = crate::orders::side_of(g, dd, [sx / n, 0.0, sz / n]);
        let far = crate::cleared::cleared_rooms_room_by(g, dd, if s > 0.0 { -1.0 } else { 1.0 });
        if js::truthy(far) && crate::cleared::cleared_rooms_is_known(g, far) { return; }
    }
    let (ux, uz) = ((sx / n - c[0]) / l, (sz / n - c[2]) / l);
    let q: V3 = [c[0] + ux * cover_m, 0.0, c[2] + uz * cover_m];
    let w: V3 = [c[0] - ux * cover_deep, 1.05, c[2] - uz * cover_deep];
    if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) || crate::doorway::doorway_stand(g, q) || v3::dist_xz(q, fc) < width / 2.0 + 1.0
        || crate::scene_collision::low_blocked(g, q, c)
        || crate::scene_collision::low_blocked(g, q, a_post)
        || men.iter().any(|m| *m != gd && v3::dist_xz(post_or_pos(g, op, *m), q) < 0.9) { return; }
    let a = g.tab.orders_assignment[a_id].clone();
    let back = DoorToolsDoneItemLaneBack {
        post: a.post, face: a.face, watch: a.watch, stance: Some(a.stance), upper: Some(a.upper), move_: a.move_,
        stack_side: Some(a.stack_side), going: a.going.clone(), there: a.there.clone(), ..Default::default()
    };
    let words = alloc::format!("covering {}", crate::orders::door_words(g, dd));
    let now = g.time;
    door_tools_assign(g, op, gd, &DoorToolsAssignA {
        post: Some(q), face: Some(v3::yaw_to(q, w)), watch: Some(w), stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim),
        move_: Some(DoorToolsDoneItemLaneBackMove { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim }), stack_side: Some(a.stack_side),
        going: words.clone().into(), there: words.into(), search: Some(true), search_t: Some(now), ..Default::default()
    });
    let cover = *g.tab.doortools_door_op[op].mine.get(gd).unwrap();
    g.tab.doortools_door_op[op].lane = Some(DoorToolsDoneItemLane { gd, door: dd, cover, back, at: None });
    let what = alloc::format!("lane:{} covers {}", g.gd[gd].callsign.clone(), g.dr[dd].def.name);
    door_tools_beat(g, op, &what);
}

/// The lane's cover man comes up to his mark, unless the order has since given him another.
pub(super) fn door_tools_lane_in(g: &mut crate::game::Game, op: DoorOpId) {
    let l = g.tab.doortools_door_op[op].lane.take();
    let l = match l { Some(x) => x, None => return };
    let o = &g.tab.doortools_door_op[op];
    if crate::condition::down(g, g.gd[l.gd].ch) || !o.men.contains(&l.gd) || o.mine.get(l.gd).copied() != Some(l.cover) { return; }
    let b = l.back;
    door_tools_assign(g, op, l.gd, &DoorToolsAssignA {
        post: b.post, face: b.face, watch: b.watch, stance: b.stance, upper: b.upper, move_: b.move_, stack_side: b.stack_side,
        going: b.going, there: b.there, ..Default::default()
    });
    let what = alloc::format!("lane:{} comes up", g.gd[l.gd].callsign.clone());
    door_tools_beat(g, op, &what);
}

/// Breach with a charge: the nearest man goes to the door, the rest stand off along the wall; `by_hand`, the player's own charge stands on it.
pub fn door_tools_start_charge(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget, by_hand: Option<bool>) -> crate::doortools::DoorToolsStartCharge {
    let by_hand = by_hand.unwrap_or(false);
    let (d, side) = match t { crate::orders::OrderTarget::Door(x) => (x.door, x.side), _ => return DoorToolsStartCharge::Text(String::from("no door")) };
    door_tools_sync(g);
    let posts = match door_tools_standoff_posts(g, d, side, men.len() as f64, Some(men)) { Some(p) => p, None => return DoorToolsStartCharge::Text(String::from("no room to stand off from that door")) };
    let taken: Vec<V3> = posts.iter().map(|x| x.pos).collect();
    let back = match door_tools_step_backs(g, d, side, men, &taken, Some(true)) { Some(b) => b, None => return DoorToolsStartCharge::Text(String::from("no room to stand off from that door")) };
    door_tools_make_room(g, d, &back);
    let op = door_tools_new_op(g, DoorToolKind::Charge, d, side, men, if by_hand { Phase::Standoff } else { Phase::Place });
    let name = crate::orders::door_words(g, d);
    let fc = g.dr[d].frame_centre;
    let watch: V3 = [fc[0], 1.05, fc[2]];
    let deal = door_tools_standoff_deal(g, men, d, &posts);
    let lead = deal[0].0;
    g.tab.doortools_door_op[op].lead = lead;
    let rest: Vec<GuardId> = if by_hand { men.to_vec() } else { men.iter().copied().filter(|m| *m != lead).collect() };
    for (gd, p) in deal.iter() { g.tab.doortools_door_op[op].post.set(*gd, *p); }
    if !by_hand { door_tools_to_lock(g, op); }
    for gd in rest {
        let p = *g.tab.doortools_door_op[op].post.get(gd).unwrap();
        door_tools_assign(g, op, gd, &DoorToolsAssignA {
            post: Some(p.pos), face: Some(p.face), watch: Some(watch), stance: Some(crate::character::Stance::HighReady), upper: Some(crate::character::UpperMode::None),
            move_: Some(DoorToolsDoneItemLaneBackMove { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim }), stack_side: Some(p.stack_side),
            going: crate::orders::Doing::new(crate::orders::At::StandingOff, "standing off"), there: alloc::format!("stood off · {}", name).into(), ..Default::default()
        });
    }
    let names: Vec<String> = men.iter().map(|m| g.gd[*m].callsign.clone()).collect();
    let tail = if by_hand { String::from("stand off on a charge set by hand") } else { alloc::format!("{} sets it", g.gd[lead].callsign.clone()) };
    let what = alloc::format!("ordered · {} · {}", names.join(", "), tail);
    door_tools_beat(g, op, &what);
    door_tools_out_of_blast(g, op);
    DoorToolsStartCharge::Arm1(())
}

/// A charge has been ordered: the men standing off from another, where that leaves them at its door or in its blast, are dealt their marks again.
fn door_tools_out_of_blast(g: &mut Game, op: DoorOpId) {
    let d = g.tab.doortools_door_op[op].door;
    for o in g.sets[g.seat].door_tools.ops.clone() {
        let x = &g.tab.doortools_door_op[o];
        if o == op || x.kind != DoorToolKind::Charge || !matches!(x.phase, Phase::Place | Phase::Standoff | Phase::Armed) { continue; }
        let (door, side, lead, men, placing) = (x.door, x.side, x.lead, x.men.clone(), x.phase == Phase::Place);
        let reach = g.k.doortools.backblast_m + 0.2;
        let fc = g.dr[d].frame_centre;
        let at_it = men.iter().any(|&m| g.tab.doortools_door_op[o].post.get(m).map_or(false, |p| v3::dist_xz(p.pos, fc) < reach || crate::doorway::before_another_door(g, p.pos, door)));
        if !at_it { continue; }
        let Some(posts) = door_tools_standoff_posts(g, door, side, men.len() as f64, Some(&men)) else { continue };
        let rest: Vec<GuardId> = men.iter().copied().filter(|&m| m != lead).collect();
        let spots: Vec<V3> = posts[1..].iter().map(|p| p.pos).collect();
        let groups: Vec<f64> = posts[1..].iter().map(|p| p.group.unwrap_or(0.0)).collect();
        let mut deal = vec![(lead, posts[0])];
        for (m, at) in crate::orders::squad_orders_pair(g, &rest, &spots, Some(&groups), None) { deal.push((m, *posts.iter().find(|p| p.pos == at).unwrap())); }
        let name = crate::orders::door_words(g, door);
        for (m, p) in deal {
            g.tab.doortools_door_op[o].post.set(m, p);
            if m == lead && placing { continue; }   // (he is at the door: off it to the new place when it is set)
            let a = crate::doortools::standing_off(g, door, &p, &name);
            door_tools_assign(g, o, m, &a);
        }
        if !placing {
            let t = g.time;
            let x = &mut g.tab.doortools_door_op[o];
            x.phase = Phase::Standoff;
            x.t0 = t;
            if x.hold == 1.0 { x.hold = 0.0; }
        }
        door_tools_beat(g, o, "stood off again, out of the other charge's way");
    }
}

/// A charge the player set stands on this face with no squad order on the door: the row is 'stand off'.
pub fn door_tools_loose_on(g: &crate::game::Game, seat: &crate::seats::SeatRef, t: &crate::orders::OrderTarget) -> bool {
    let (door, side) = match t { crate::orders::OrderTarget::Door(x) => (x.door, x.side), _ => return false };
    let c = match door_tools_charge_on(g, seat, door) { Some(c) => c, None => return false };
    by_sam(g, c) && g.tab.doortools_set_charge[c].side == side && !charge_order_on(g, seat, door)
}

/// Can these men stand off from a charge on that face: true, or the first why-not.
pub fn door_tools_standoff_fits(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget, own: Option<bool>) -> crate::doortools::DoorToolsStandoffFits {
    let own = own.unwrap_or(false);
    let (d, side) = match t { crate::orders::OrderTarget::Door(x) => (x.door, x.side), _ => return DoorToolsStandoffFits::Text(String::from("no door")) };
    let posts = match door_tools_standoff_posts(g, d, side, men.len() as f64, Some(men)) { Some(p) => p, None => return DoorToolsStandoffFits::Text(String::from("no room to stand off.")) };
    if !own {
        if let Some(mate) = door_tools_behind(g, d, side, men) { return DoorToolsStandoffFits::Text(alloc::format!("{} is behind that door.", g.gd[mate].callsign.clone())); }
    }
    let taken: Vec<V3> = posts.iter().map(|x| x.pos).collect();
    if door_tools_step_backs(g, d, side, men, &taken, Some(true)).is_none() { return DoorToolsStandoffFits::Text(String::from("no room to stand off.")); }
    match door_tools_standoff_walk(g, men, d, &posts) { Some(w) => DoorToolsStandoffFits::Text(w), None => DoorToolsStandoffFits::Bool(true) }
}

/// Can these men walk to these marks, by the very dealing the issue makes: the first why-not, or None.
pub(super) fn door_tools_standoff_walk(g: &mut crate::game::Game, men: &[GuardId], d: DoorId, posts: &[crate::orders::StackPost]) -> Option<String> {
    for (gd, p) in door_tools_standoff_deal(g, men, d, posts) {
        if let Some(w) = crate::orders::squad_orders_reach(g, gd, p.pos) { return Some(w); }
    }
    None
}

/// One man's test for close_words: blow()'s own reach and line, the door's own leaf (which is coming apart) let through.
fn close_ask(g: &Game, others: &crate::ask::Over, d: DoorId, side: f64, out: V3, inn: V3, p: V3, who: String, sam: bool, near: &mut Vec<String>, far: &mut Vec<String>) {
    let beyond = off_of(g, d, p, side) < 0.0;
    let dist = v3::dist_xz(p, g.dr[d].frame_centre);
    let (stun_m, backblast_m) = (g.k.doortools.stun_m, g.k.doortools.backblast_m);
    if if sam { dist >= (if beyond { stun_m } else { backblast_m }) } else { dist > stun_m || (!beyond && dist >= backblast_m) } { return; }
    if !crate::scene_collision::solid_blocked_over(g, others, if beyond { inn } else { out }, [p[0], 1.1, p[2]], None) {
        if beyond { far.push(who); } else { near.push(who); }
    }
}

/// Who a charge on a door would catch if it went now, as words for the row: the seat's own man and his men, as things stand, of the game shared.
pub(super) fn door_tools_close_words_of(g: &crate::game::Game, seat: &crate::seats::SeatRef, c_2: ChargeId) -> String {
    let c = c_2;
    let (d, side) = (g.tab.doortools_set_charge[c].door, g.tab.doortools_set_charge[c].side);
    let width = g.dr[d].def.width;
    let into = -side;
    let out = door_pt(g, d, width / 2.0, 0.5, side, Some(1.1));
    let inn = door_pt(g, d, width / 2.0, 0.45, into, Some(1.1));
    let (mut near, mut far): (Vec<String>, Vec<String>) = (Vec::new(), Vec::new());
    let others = crate::ask::Over::but_door(g, d);
    if !crate::condition::down(g, g.pl[seat.player].ch) {
        let p = g.seat_pos(seat);
        close_ask(g, &others, d, side, out, inn, p, String::from("you"), true, &mut near, &mut far);
    }
    // (every man of the team's who is up, whoever's orders he takes: a charge asks nobody)
    for &gd in &g.guards {
        if !g.team(gd).ours() || crate::condition::down(g, g.gd[gd].ch) { continue; }
        let (p, who) = (g.pos_of(gd), g.gd[gd].callsign.clone());
        close_ask(g, &others, d, side, out, inn, p, who, false, &mut near, &mut far);
    }
    let mut w = String::new();
    if !near.is_empty() { w.push_str(&alloc::format!(" · too close: {}", near.join(", "))); }
    if !far.is_empty() { w.push_str(&alloc::format!(" · behind the door: {}", far.join(", "))); }
    w
}
/// …of the seat that stands here
pub fn door_tools_close_words(g: & crate::game::Game, c_2: ChargeId) -> String {
    door_tools_close_words_of(g, &crate::seats::here(g), c_2)
}

/// The face of this door the player could set a charge on by hand right now (None: none).
fn door_tools_sam_face(g: &mut crate::game::Game, d: DoorId) -> Option<f64> {
    let pl = g.player;
    if g.pl[pl].tow.is_some() || g.pl[pl].holding().is_some() || g.dr[d].leaf.off || g.dr[d].def.steel == Some(true)
        || is_open(g, d) || door_tools_charge_on(g, &crate::seats::here(g), d).is_some() || charge_order_on(g, &crate::seats::here(g), d) { return None; }
    door_tools_sync(g);
    if g.sets[g.seat].door_tools.charges <= 0.0 { return None; }
    let p = g.player_pos(pl);
    Some(if off_of(g, d, p, 1.0) >= 0.0 { 1.0 } else { -1.0 })
}

/// A door's rows: a set charge is said on the door's own line; else, in reach with free hands, 'set a charge'.
pub fn door_tools_sam_rows(g: &mut crate::game::Game, it: &mut crate::game::Interactable, d: DoorId, verbs: &mut Vec<crate::game::Interactable>, free: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>) -> crate::game::Interactable) {
    let c = door_tools_charge_on(g, &crate::seats::here(g), d);
    let key = key_word(g);
    let f = crate::ownblow::blow_key_word(false, &key);
    if let Some(c) = c {
        let close = door_tools_close_words(g, c);
        let is = door_tools_key_is(g, &crate::seats::here(g), c);
        let w = alloc::format!("{} · charge set{}", close, crate::ownblow::blow_tail(is, &key, None));
        it.line1.push_str(&w);
        for v in verbs.iter_mut() { v.line1.push_str(&w); }
        return;
    }
    if !free || door_tools_sam_face(g, d).is_none() { return; }
    let text = alloc::format!("set a charge · {} left", g.sets[g.seat].door_tools.charges);
    let r = row(g, it.clone(), crate::game::Verb::Charge, text, Some(alloc::format!("{}  blows it once set", f)));
    verbs.push(r);
}

/// Is this set charge the squad's key's to blow: one the squad set, or his own while the squad works its door.
pub fn door_tools_key_is(g: &crate::game::Game, seat: &crate::seats::SeatRef, c_2: ChargeId) -> bool {
    let c = c_2;
    if !by_sam(g, c) { return true; }
    let door = g.tab.doortools_set_charge[c].door;
    if seat.door_tools.ops.iter().any(|o| g.tab.doortools_door_op[*o].door == door) { return true; }
    let men = crate::orders::squad_orders_men(g, seat, None, None);
    door_tools_stacks_of(g, seat, &men).iter().any(|x| x.stack == door)
}

/// The nearest charge his own hands set that is on no word still to go.
pub fn door_tools_sam_set(g: &mut crate::game::Game) -> Option<ChargeId> {
    door_tools_nearest(g, Some(&mut |g: &mut Game, c: ChargeId| by_sam(g, c) && !door_tools_on_pending_word(g, &crate::seats::here(g), c)))
}

/// Where a charge is, in the row's words.
pub fn door_tools_sam_where(g: &crate::game::Game, c_2: ChargeId) -> String {
    let d = g.tab.doortools_set_charge[c_2].door;
    alloc::format!("the {}", crate::orders::door_words(g, d))
}

/// The charge going on under his hands: where on the leaf, and the face's normal.
pub fn door_tools_sam_job(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Option<crate::doortools::DoorToolsSamJob> {
    let pl = seat.player;
    let s = g.pl[pl].charging().copied()?;
    let d = s.door;
    let o = door_pt(g, d, 0.0, 0.0, s.side, None);
    let n1 = door_pt(g, d, 0.0, 1.0, s.side, None);
    let width = g.dr[d].def.width;
    let p = door_pt(g, d, width - 0.22, 0.025, s.side, Some(1.05));
    Some(DoorToolsSamJob { p, n: [n1[0] - o[0], 0.0, n1[2] - o[2]], at: s.at })
}

/// How far along it is, 0 to 1.
pub fn door_tools_sam_progress(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> f64 {
    let pl = seat.player;
    match g.pl[pl].charging() { Some(s) => v3::clamp((g.time - s.t0) / g.k.doortools.set_secs, 0.0, 1.0), None => 0.0 }
}

/// Each frame: the row performed starts it; his hands on it for setSecs set it; anything that takes him off it breaks it off.
pub(super) fn door_tools_sam_tick(g: &mut crate::game::Game) {
    let pl = g.player;
    let set_secs = g.k.doortools.set_secs;
    let (act_some, act_door, mine) = match &g.sets[g.seat].act {
        Some(a) => (true, a.door, a.kind == crate::game::InteractableKind::Door && a.verb == Some(crate::game::Verb::Charge) && a.door.is_some()),
        None => (false, None, false),
    };
    let s = match g.pl[pl].charging().copied() {
        Some(s) => s,
        None => {
            if !mine { return; }
            let d = act_door.unwrap();
            let side = match door_tools_sam_face(g, d) { Some(x) => x, None => return };
            let p = g.player_pos(pl);
            let t0 = g.time;
            crate::deed::begin(g, crate::game::Deed::Charge(crate::game::PlayerCharging { door: d, side, t0, at: [p[0], 0.0, p[2]] }));
            return;
        }
    };
    let d = s.door;
    let c = g.pl[pl].ch;
    let p = g.ch[c].pos;
    if crate::condition::down(g, g.pl[pl].ch) || g.pl[pl].sprinting
        || g.dr[d].leaf.off || is_open(g, d) || door_tools_charge_on(g, &crate::seats::here(g), d).is_some()
        || charge_order_on(g, &crate::seats::here(g), d)
        || off_of(g, d, p, s.side) <= 0.0 || v3::dist_xz(p, s.at) > SAM_SHIFT || (act_some && !(mine && act_door == Some(d))) {
        g.pl[pl].charging_over();
        return;
    }
    if crate::workstow::sam_stowing(g, &crate::seats::here(g)) {
        let t = g.time;
        if let Some(x) = g.pl[pl].charging_mut() { x.t0 = t; }
        return;
    }
    let j = door_tools_sam_job(g, &crate::seats::here(g)).unwrap();
    let hand = if crate::game::player_pistol_up(g, pl) { Some(0.0) } else { None };
    let el = g.time - s.t0;
    crate::handwork::charge_hands(g, c, j.p, j.n, el, set_secs, hand);
    if g.time - s.t0 < set_secs { return; }
    g.pl[pl].charging_over();
    if !door_tools_take(g, crate::doortools::DoorToolsCan::Charge) { return; }
    g.sets[g.seat].door_tools.sam_spent += 1.0;
    let row = SetCharge { door: d, side: s.side, by: String::from(crate::ownblow::SAM_BY) };
    let id = g.tab.doortools_set_charge.push(row);
    g.sets[g.seat].door_tools.set.push(id);
    let at = crate::doors::door_pos(g, d);
    crate::audio_iface::game_audio_like_play(g, "lockPick", Some(at), Some(0.6), None);
}

/// The charges that stand set: drawn; one whose door has come open or off its hinges is off it, back in the count.
pub(super) fn door_tools_tick_set(g: &mut crate::game::Game) {
    for c in g.sets[g.seat].door_tools.set.clone() {
        let d = g.tab.doortools_set_charge[c].door;
        let gone = g.dr[d].leaf.off;
        if gone || is_open(g, d) {
            if let Some(i) = g.sets[g.seat].door_tools.set.iter().position(|x| *x == c) { g.sets[g.seat].door_tools.set.remove(i); }
            door_tools_refund(g, crate::doortools::DoorToolsCan::Charge);
            if by_sam(g, c) { g.sets[g.seat].door_tools.sam_spent = m::max(0.0, g.sets[g.seat].door_tools.sam_spent - 1.0); }
            let setter = crate::orders::squad_orders_men(g, &crate::seats::here(g), None, None).into_iter().find(|&m| g.gd[m].callsign == g.tab.doortools_set_charge[c].by);
            crate::squadnet::team_says(g, setter, if gone { "that door's off its hinges. charge is off." } else { "that door came open. charge is off." });
            continue;
        }
        door_tools_draw_charge(g, c);
    }
}

/// Who stands off where for a charge: the man nearest the door on the first mark, the rest dealt the others; first entry = the setter.
fn door_tools_standoff_deal(g: &crate::game::Game, men: &[GuardId], d: DoorId, posts: &[crate::orders::StackPost]) -> Vec<(GuardId, crate::orders::StackPost)> {
    let fc = g.dr[d].frame_centre;
    let mut by: Vec<(GuardId, f64)> = men.iter().map(|m| (*m, v3::dist_xz(g.pos_of(*m), fc))).collect();
    js::sort_by(&mut by, |a, b| a.1 - b.1);
    let lead = by[0].0;
    let others: Vec<GuardId> = men.iter().copied().filter(|m| *m != lead).collect();
    let spots: Vec<V3> = posts[1..].iter().map(|x| x.pos).collect();
    let groups: Vec<f64> = posts[1..].iter().map(|x| x.group.unwrap_or(0.0)).collect();
    let dealt = crate::orders::squad_orders_pair(g, &others, &spots, Some(&groups), None);
    let mut out: Vec<(GuardId, crate::orders::StackPost)> = vec![(lead, posts[0])];
    for (gd, p) in dealt { out.push((gd, *posts.iter().find(|x| x.pos == p).unwrap())); }
    out
}

/// The man who sets the charge: to the lock.
fn door_tools_to_lock(g: &mut crate::game::Game, op: DoorOpId) {
    let (d, side, lead) = { let o = &g.tab.doortools_door_op[op]; (o.door, o.side, o.lead) };
    let width = g.dr[d].def.width;
    let from = door_pt(g, d, width / 2.0, 1.0, side, None);
    let ws = crate::doors::doors_work_spot(g, d, from, Some(0.5), None);
    let going = alloc::format!("charge · {}", crate::orders::door_words(g, d));
    door_tools_assign(g, op, lead, &DoorToolsAssignA {
        post: Some(ws.pos), face: Some(ws.yaw), watch: None, stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::None),
        going: going.into(), there: "setting the charge".into(), ..Default::default()
    });
}

/// A silent squad at a door locked against it: the lead to the keyway, to pick it.
pub(super) fn door_tools_to_pick(g: &mut crate::game::Game, op: DoorOpId) {
    let (d, side, lead) = { let o = &g.tab.doortools_door_op[op]; (o.door, o.side, o.lead) };
    let width = g.dr[d].def.width;
    let from = door_pt(g, d, width / 2.0, 1.0, side, None);
    let ws = crate::doors::doors_work_spot(g, d, from, None, None);
    let now = g.time;
    { let o = &mut g.tab.doortools_door_op[op];
      o.kick = Some(DoorToolsDoneItemKick { pos: ws.pos, yaw: ws.yaw, t: f64::NAN, pick: Some(true), ..Default::default() });
      o.phase = Phase::Kick; o.t0 = now; }
    let going = alloc::format!("on the lock · {}", crate::orders::door_words(g, d));
    door_tools_assign(g, op, lead, &DoorToolsAssignA {
        post: Some(ws.pos), face: Some(ws.yaw), watch: None, stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::None),
        going: going.into(), there: "picking the lock".into(), ..Default::default()
    });
}

/// A lock a pick opens: a cylinder, not a steel door's bolt, a keypad's, nor a wedge.
pub(super) fn door_tools_pickable(g: &crate::game::Game, d: DoorId) -> bool {
    g.dr[d].def.steel != Some(true) && crate::devices::devices_pad_of(g, d).is_none() && !is_wedge(g, d)
}

/// His hands off the lock.
pub(super) fn door_tools_off_lock(g: &mut crate::game::Game, op: DoorOpId) {
    let by = g.tab.doortools_door_op[op].kick.as_ref().and_then(|k| k.by);
    if let Some(by) = by {
        let c = g.gd[by].ch;
        g.ch[c].anim.lockpick = false;
        with_kick(g, op, |k| k.by = None);
    }
}

/// One frame of a lock being picked: the lead at the keyway with the player's own pick; it gives, he goes back to his jamb and the order goes on.
pub(super) fn door_tools_pick_step(g: &mut crate::game::Game, op: DoorOpId) {
    let kick_cap = g.k.doortools.kick_cap;
    let (d, side, lead) = { let o = &g.tab.doortools_door_op[op]; (o.door, o.side, o.lead) };
    let t = g.time;
    let c = g.gd[lead].ch;
    let k0 = g.tab.doortools_door_op[op].kick.unwrap();
    let dt = m::min(0.05, t - k0.at.unwrap_or(t));
    with_kick(g, op, |k| k.at = Some(t));
    if g.tab.doortools_door_op[op].kick.unwrap().done_t.is_none() && (is_open(g, d) || !door_tools_locked_out(g, d, side)) {
        door_tools_off_lock(g, op);
        door_tools_back_to_jamb(g, op, lead);
        with_kick(g, op, |k| k.done_t = Some(t));
    }
    let done_t = g.tab.doortools_door_op[op].kick.and_then(|k| k.done_t);
    if let Some(done_t) = done_t {
        if on_word(g, op) {
            let o = &mut g.tab.doortools_door_op[op];
            o.kick = None; o.phase = Phase::Ready; o.hold = 2.0;
            return;
        }
        if !is_open(g, d) && !door_tools_arrived(g, op, lead) && t - done_t < kick_cap { return; }
        g.tab.doortools_door_op[op].kick = None;
        door_tools_open(g, op);
        return;
    }
    if !(crate::silent::silent_pick(g) || door_tools_frame_held(g, d, side)) || !door_tools_pickable(g, d) {
        door_tools_off_lock(g, op);
        g.tab.doortools_door_op[op].kick = None;
        door_tools_beat(g, op, "pick off");
        door_tools_open(g, op);
        return;
    }
    let a = g.tab.doortools_door_op[op].mine.get(lead).copied();
    let busy = a.map_or(false, |a| g.tab.orders_assignment[a].yield_t >= 0.0) || crate::condition::is_blind(g, g.gd[lead].ch) || crate::condition::is_folded(g, g.gd[lead].ch)
        || (g.gd[lead].script.as_ref().map_or(false, |s| s.over.is_some()) && (crate::silent::silent_compromised(g, &crate::seats::here(g)) || !crate::silent::squad_silent(g, &crate::seats::here(g))));
    if !busy {
        with_kick(g, op, |k| k.off_t = None);
    } else {
        let off_t = g.tab.doortools_door_op[op].kick.and_then(|k| k.off_t).unwrap_or(t);
        with_kick(g, op, |k| k.off_t = Some(off_t));
        if t - off_t < kick_cap + 3.0 {
            door_tools_off_lock(g, op);
            g.tab.doortools_door_op[op].t0 = t;
            return;
        }
    }
    let k = g.tab.doortools_door_op[op].kick.unwrap();
    let p = g.ch[c].pos;
    let on = v3::dist_xz(p, k.pos) < 0.4 && m::abs(v3::wrap_angle(g.ch[c].body_yaw - k.yaw)) < 0.35;
    if !on {
        door_tools_off_lock(g, op);
        if t - g.tab.doortools_door_op[op].t0 < kick_cap { return; }
        if v3::dist_xz(p, k.pos) > 1.2 {
            crate::game::game_say(g, lead, "can't get on the door.", None, None);
            door_tools_end(g, op, crate::doortools::Ended::NoLock);
            return;
        }
    }
    if g.tab.doortools_door_op[op].kick.unwrap().by.is_none() { door_tools_beat(g, op, "picking"); }
    g.ch[c].anim.lockpick = true;
    with_kick(g, op, |k| k.by = Some(lead));
    let who = g.ch[c].id;
    if !crate::doors::doors_pick(g, d, who, dt) { return; }
    let _ = crate::silent::silent_pick(g);
    g.sets[g.seat].door_tools.stats.picks += 1.0;
    door_tools_beat(g, op, "picked");
    door_tools_off_lock(g, op);
    door_tools_back_to_jamb(g, op, lead);
    with_kick(g, op, |k| k.done_t = Some(t));
}

/// shell(m) of toGun: a breaching shotgun that can take this lock.
fn gun_shell(g: &Game, d: DoorId, m: GuardId) -> bool { !is_wedge(g, d) && crate::shotgun::carries_breacher(g, m) }
/// can(m) of toGun: that, or a hammer in his big slot.
fn gun_can(g: &Game, d: DoorId, m: GuardId) -> bool { gun_shell(g, d, m) || crate::bigkit::carries_hammer(g, m) }

/// A door locked against the team and a man of this order with a breaching shotgun or a hammer: he goes to the lock with it. False: the boot.
pub(super) fn door_tools_to_gun(g: &mut crate::game::Game, op: DoorOpId) -> bool {
    let (d, side, lead, men) = { let o = &g.tab.doortools_door_op[op]; if o.no_gun == Some(true) { return false; } (o.door, o.side, o.lead, o.men.clone()) };
    let fc = g.dr[d].frame_centre;
    let mut held: Vec<(GuardId, f64)> = men.iter().copied().filter(|m| g.tab.doortools_door_op[op].post.has(*m)).map(|m| (m, v3::dist_xz(g.pos_of(m), fc))).collect();
    js::sort_by(&mut held, |a, b| a.1 - b.1);
    let mut by: Option<GuardId> = None;
    if gun_shell(g, d, lead) { by = Some(lead); }
    if by.is_none() { for (m, _) in held.iter() { if gun_shell(g, d, *m) { by = Some(*m); break; } } }
    if by.is_none() && gun_can(g, d, lead) { by = Some(lead); }
    if by.is_none() { for (m, _) in held.iter() { if gun_can(g, d, *m) { by = Some(*m); break; } } }
    let by = match by { Some(x) => x, None => return false };
    let hammer = !gun_shell(g, d, by);
    let mut avoid: Vec<V3> = men.iter().copied().filter(|m| *m != by).map(|m| post_or_pos(g, op, m)).collect();
    avoid.push(g.player_pos(g.player));
    let mark = if hammer { None } else { Some(post_or_pos(g, op, by)) };
    let close = if hammer || crate::kit::kit_big(g, by) == crate::kit::BigKit::Shotgun {
        Some(if hammer { crate::kit::BigTool::Hammer } else { crate::kit::BigTool::Shotgun })
    } else { None };
    let sp = match crate::shotgun::breach_spot(g, d, side, Some(&avoid), mark, close) { Some(s) => s, None => return false };
    let silent = crate::silent::squad_silent(g, &crate::seats::here(g));
    let shut = if door_tools_locked_out(g, d, side) { None } else { Some(true) };
    let now = g.time;
    { let o = &mut g.tab.doortools_door_op[op];
      o.kick = Some(DoorToolsDoneItemKick {
          pos: sp.pos, yaw: sp.yaw, t: f64::NAN,
          gun: Some(DoorToolsDoneItemKickGun { by, at: sp.at, n: 0.0, next_t: 0.0, silent, own: Some(sp.own), hammer: if hammer { Some(true) } else { None }, shut }),
          ..Default::default()
      });
      o.phase = Phase::Kick; o.t0 = now; }
    let going = alloc::format!("on the lock · {}", crate::orders::door_words(g, d));
    door_tools_assign(g, op, by, &DoorToolsAssignA {
        post: Some(sp.pos), face: Some(sp.yaw), watch: Some(sp.at), stance: Some(crate::character::Stance::None), upper: Some(if hammer { crate::character::UpperMode::None } else { crate::character::UpperMode::Aim }),
        going: going.into(), there: "breaching".into(), ..Default::default()
    });
    let locked_now = door_tools_locked_out(g, d, side);
    let line = if locked_now { if hammer { "locked. hammer." } else { "locked. shotgun." } } else if hammer { "hammer." } else { "shotgun." };
    crate::game::game_say(g, by, line, None, None);
    let what = alloc::format!("{}:{}{}", if hammer { "hammer" } else { "gun" }, g.gd[by].callsign.clone(), if sp.beside { "" } else { " (in front)" });
    door_tools_beat(g, op, &what);
    true
}

/// The gun is off this door's lock for good: his mark again, and open()'s question with the gun ruled out.
fn door_tools_gun_off(g: &mut crate::game::Game, op: DoorOpId, why: &str) {
    let by = g.tab.doortools_door_op[op].kick.and_then(|k| k.gun).map(|x| x.by);
    { let o = &mut g.tab.doortools_door_op[op]; o.no_gun = Some(true); o.kick = None; }
    let what = alloc::format!("gun off: {}", why);
    door_tools_beat(g, op, &what);
    if let Some(by) = by {
        crate::bigkit::gun_down(g, by);
        let (lead, has) = { let o = &g.tab.doortools_door_op[op]; (o.lead, o.men.contains(&by)) };
        if by != lead && has { door_tools_back_to_jamb(g, op, by); }
    }
    door_tools_open(g, op);
}

/// One frame of a lock being shot off: on his spot, square to the lock, a shell; the lock still holding after SHOTGUN.shells, the boot.
pub(super) fn door_tools_gun_step(g: &mut crate::game::Game, op: DoorOpId) {
    let kick_cap = g.k.doortools.kick_cap;
    let (d, side, lead) = { let o = &g.tab.doortools_door_op[op]; (o.door, o.side, o.lead) };
    let t = g.time;
    let k = g.tab.doortools_door_op[op].kick.unwrap();
    let gun = k.gun.unwrap();
    let by = gun.by;
    let c = g.gd[by].ch;
    if !g.tab.doortools_door_op[op].men.contains(&by) {
        g.tab.doortools_door_op[op].kick = None;
        door_tools_beat(g, op, "gun off: its man is gone");
        door_tools_open(g, op);
        return;
    }
    if !gun.silent && crate::silent::squad_silent(g, &crate::seats::here(g)) {
        g.tab.doortools_door_op[op].kick = None;
        door_tools_beat(g, op, "gun off: the squad went silent");
        if by != lead { door_tools_back_to_jamb(g, op, by); }
        door_tools_open(g, op);
        return;
    }
    if is_open(g, d) || (gun.shut != Some(true) && !door_tools_locked_out(g, d, side)) {
        door_tools_back_to_jamb(g, op, by);
        if on_word(g, op) {
            let o = &mut g.tab.doortools_door_op[op];
            o.kick = None; o.phase = Phase::Ready; o.hold = 2.0;
            return;
        }
        g.tab.doortools_door_op[op].kick = None;
        door_tools_open(g, op);
        return;
    }
    let a = g.tab.doortools_door_op[op].mine.get(by).copied();
    let busy = a.map_or(false, |a| g.tab.orders_assignment[a].yield_t >= 0.0) || crate::condition::is_blind(g, g.gd[by].ch) || crate::condition::is_folded(g, g.gd[by].ch);
    if !busy {
        with_kick(g, op, |k| k.off_t = None);
    } else {
        let off_t = k.off_t.unwrap_or(t);
        with_kick(g, op, |k| k.off_t = Some(off_t));
        if t - off_t < kick_cap + 3.0 {
            crate::bigkit::gun_down(g, by);
            if g.gd[by].big_out.is_some() {
                crate::bigkit::hammer_in(g, by, Some(true));
                with_gun(g, op, |x| x.next_t = 0.0);
            }
            g.tab.doortools_door_op[op].t0 = t;
            return;
        }
    }
    if gun.hammer == Some(true) { door_tools_hammer_step(g, op); return; }
    let w0 = g.fa[g.gd[by].weapon].def;
    let kit = weapon_breach(g, w0).is_none() || g.gd[by].slung.as_ref().map_or(false, |s| s.out);
    let up = !kit || (v3::dist_xz(g.ch[c].pos, k.pos) < 1.5 && crate::kit::kit_big(g, by) == crate::kit::BigKit::Shotgun && crate::bigkit::gun_up(g, by));
    let w1 = g.fa[g.gd[by].weapon].def;
    let mut b = weapon_breach(g, w1);
    if b.is_none() && kit && crate::kit::kit_big(g, by) == crate::kit::BigKit::Shotgun { b = weapon_breach(g, crate::weapons::WeaponId::M870); }
    let b = match b { Some(x) => x, None => { door_tools_gun_off(g, op, "no gun in his hands"); return; } };
    let (p, muzzle, yaw) = (g.ch[c].pos, g.ch[c].muzzle, g.ch[c].body_yaw);
    let t0 = g.tab.doortools_door_op[op].t0;
    let own = gun.own == Some(true);
    let (own_slack, own_beat, aim_cap, shells) = (g.k.shotgun.own_slack, g.k.shotgun.own_beat, g.k.shotgun.aim_cap, g.k.shotgun.shells);
    let on = up && v3::dist_xz(p, k.pos) < (if own { own_slack } else { 0.4 }) && m::abs(v3::wrap_angle(yaw - v3::yaw_to(p, gun.at))) < 0.35 && v3::dist(muzzle, gun.at) <= b.reach_m
        && off_of(g, d, muzzle, side) > 0.05 && (!own || t - t0 >= own_beat);
    if !on {
        if t - t0 >= kick_cap + aim_cap { door_tools_gun_off(g, op, "never got onto the lock"); }
        return;
    }
    if on_word(g, op) { g.tab.doortools_door_op[op].hold = 2.0; return; }
    if t < gun.next_t { return; }
    let wid = g.gd[by].weapon;
    if !crate::weapons::firearm_fire(g, wid) {
        crate::weapons::firearm_rearm(g, wid);
        let dur = g.weapon(g.fa[wid].def).reload.dur;
        with_gun(g, op, |x| x.next_t = t + dur);
        crate::game::game_say(g, by, "reloading.", None, None);
        return;
    }
    let lock = g.dr[d].leaf.locked && g.dr[d].leaf.latched;
    crate::combat::fire_weapon(g, c, gun.at, Some(&crate::combat::FireOpts { steady: Some(true), spread: Some(0.0), ..Default::default() }));
    if !lock && gun.shut == Some(true) && !is_open(g, d) {
        let (from, who) = (g.ch[c].pos, g.ch[c].id);
        crate::doors::doors_kick_in(g, d, from, who, Some(true));
    }
    let cd = g.weapon(g.fa[g.gd[by].weapon].def).cd;
    with_gun(g, op, |x| { x.n += 1.0; x.next_t = t + cd; });
    g.tab.doortools_door_op[op].t0 = t;
    if !g.dr[d].leaf.latched {
        with_kick(g, op, |k| k.t = t);
        door_tools_beat(g, op, "shotgun");
        crate::bigkit::gun_down(g, by);
        door_tools_back_to_jamb(g, op, by);
        if !is_open(g, d) && m::abs(g.dr[d].leaf.vel) < 0.5 {
            { let o = &mut g.tab.doortools_door_op[op]; o.kick = None; o.no_gun = Some(true); }
            door_tools_open(g, op);
        } else {
            door_tools_opened(g, op);
        }
        return;
    }
    door_tools_beat(g, op, "shell: the lock held");
    if gun.n + 1.0 >= shells { door_tools_gun_off(g, op, "the lock held"); }
}

/// Tell the animator where a blow lands (hammerStep's `tell`): H is gd.bigOut, so it is written there.
fn hammer_tell(g: &mut Game, by: GuardId, t: f64, at: f64) {
    if let Some(h) = g.gd[by].big_out.as_mut() {
        let rel = t - h.t0;
        h.blows.retain(|b| *b <= rel);
        h.blows.push(at - h.t0);
    }
}

/// One frame of a lock being hammered: on his spot, one blow where a boot would do, BIGSLOT.swings where the frame holds one.
fn door_tools_hammer_step(g: &mut crate::game::Game, op: DoorOpId) {
    let kick_cap = g.k.doortools.kick_cap;
    let (d, side) = { let o = &g.tab.doortools_door_op[op]; (o.door, o.side) };
    let t = g.time;
    let k = g.tab.doortools_door_op[op].kick.unwrap();
    let gun = k.gun.unwrap();
    let by = gun.by;
    let c = g.gd[by].ch;
    let h_t0 = crate::bigkit::hammer_out(g, by, gun.at);
    let (p, yaw) = (g.ch[c].pos, g.ch[c].body_yaw);
    let on = v3::dist_xz(p, k.pos) < 0.4 && m::abs(v3::wrap_angle(yaw - v3::yaw_to(p, gun.at))) < 0.35;
    if !on {
        if t - g.tab.doortools_door_op[op].t0 >= kick_cap + g.k.shotgun.aim_cap { door_tools_gun_off(g, op, "never got onto the lock"); }
        return;
    }
    if gun.next_t == 0.0 {
        let by_t = h_t0 + crate::bigkit::big_out_by(g, crate::kit::BigTool::Hammer);
        let next = m::max(t, by_t) + crate::bigkit::big_wind(g);
        with_gun(g, op, |x| x.next_t = next);
        g.tab.doortools_door_op[op].t0 = t;
        hammer_tell(g, by, t, next);
        return;
    }
    if on_word(g, op) {
        g.tab.doortools_door_op[op].hold = 2.0;
        if t > gun.next_t - WORD_LEAD {
            let next = t + WORD_LEAD;
            with_gun(g, op, |x| x.next_t = next);
            hammer_tell(g, by, t, next);
        }
        return;
    }
    if t < gun.next_t { return; }
    let n = gun.n + 1.0;
    let next = t + crate::bigkit::big_swing(g);
    with_gun(g, op, |x| { x.n = n; x.next_t = next; });
    g.tab.doortools_door_op[op].t0 = t;
    let held = if gun.shut == Some(true) { g.dr[d].leaf.locked && crate::doors::doors_boot_holds(g, d, p) } else { door_tools_frame_held(g, d, side) };
    if n < (if held { m::max(1.0, g.k.bigslot.swings) } else { 1.0 }) {
        crate::bigkit::hammer_blow(g, by, d, false);
        hammer_tell(g, by, t, next);
        door_tools_beat(g, op, "hammer: a blow");
        return;
    }
    if !crate::bigkit::hammer_blow(g, by, d, true) { door_tools_gun_off(g, op, "the door held the hammer"); return; }
    with_kick(g, op, |k| k.t = t);
    door_tools_beat(g, op, "hammered");
    door_tools_back_to_jamb(g, op, by);
    if !is_open(g, d) && m::abs(g.dr[d].leaf.vel) < 0.5 {
        { let o = &mut g.tab.doortools_door_op[op]; o.kick = None; o.no_gun = Some(true); }
        door_tools_open(g, op);
    } else {
        door_tools_opened(g, op);
    }
}

/// A breaching shotgun out of a big slot is in nobody's hands without a lock to shoot: the net under every way out of the lock stage.
pub(super) fn door_tools_sling_tick(g: &mut crate::game::Game) {
    for gd in g.guards.clone() {
        if !g.gd[gd].slung.as_ref().map_or(false, |s| s.out) { continue; }
        // (only this book's own men are judged by this book's jobs: another seat's man on HIS seat's lock is not ours to put the gun away for)
        if !crate::orders::squad_orders_mine(g, &crate::seats::here(g), gd) { continue; }
        let on_lock = g.sets[g.seat].door_tools.ops.iter().any(|o| {
            let x = &g.tab.doortools_door_op[*o];
            x.phase == Phase::Kick && x.kick.and_then(|k| k.gun).map_or(false, |u| u.by == gd && u.hammer != Some(true))
        });
        if !on_lock { crate::bigkit::gun_down(g, gd); }
    }
}

/// The man this op turned on is gone and the next has it: a job only he was at starts over with the new man walking to it.
pub(super) fn door_tools_new_lead(g: &mut crate::game::Game, op: DoorOpId) {
    let t = g.time;
    let (phase, kick, lead, d) = { let o = &g.tab.doortools_door_op[op]; (o.phase, o.kick, o.lead, o.door) };
    if phase == Phase::Place {
        { let o = &mut g.tab.doortools_door_op[op]; o.set_t = f64::NAN; o.t0 = t; }
        if !g.tab.doortools_door_op[op].post.has(lead) {
            let c = g.gd[lead].ch;
            let p = crate::orders::StackPost { pos: g.ch[c].pos, face: g.ch[c].body_yaw, stack_side: 1.0, lead: false, group: None };
            g.tab.doortools_door_op[op].post.set(lead, p);
        }
        door_tools_to_lock(g, op);
        let what = alloc::format!("setter now {}", g.gd[lead].callsign.clone());
        door_tools_beat(g, op, &what);
    } else if phase == Phase::Kick && kick.map_or(false, |k| k.pick == Some(true)) {
        door_tools_off_lock(g, op);
        g.tab.doortools_door_op[op].t0 = t;
        let k = g.tab.doortools_door_op[op].kick.unwrap();
        if k.done_t.is_some() {
            with_kick(g, op, |k| k.done_t = Some(t));
            door_tools_back_to_jamb(g, op, lead);
        } else {
            with_kick(g, op, |k| k.at = None);
            let going = alloc::format!("on the lock · {}", crate::orders::door_words(g, d));
            door_tools_assign(g, op, lead, &DoorToolsAssignA {
                post: Some(k.pos), face: Some(k.yaw), watch: None, stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::None),
                going: going.into(), there: "picking the lock".into(), ..Default::default()
            });
        }
        let what = alloc::format!("picker now {}", g.gd[lead].callsign.clone());
        door_tools_beat(g, op, &what);
    } else if phase == Phase::Kick && kick.map_or(false, |k| k.gun.is_some()) {
        let by = kick.unwrap().gun.unwrap().by;
        if !g.tab.doortools_door_op[op].men.contains(&by) {
            g.tab.doortools_door_op[op].kick = None;
            door_tools_open(g, op);
        }
    } else if phase == Phase::Kick && kick.is_some() {
        let k = kick.unwrap();
        with_kick(g, op, |k| k.t = f64::NAN);
        g.tab.doortools_door_op[op].t0 = t;
        let going = alloc::format!("on the door · {}", crate::orders::door_words(g, d));
        door_tools_assign(g, op, lead, &DoorToolsAssignA {
            post: Some(k.pos), face: Some(k.yaw), watch: None, stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::None),
            going: going.into(), there: "breaching".into(), ..Default::default()
        });
        let what = alloc::format!("kicker now {}", g.gd[lead].callsign.clone());
        door_tools_beat(g, op, &what);
    } else if phase == Phase::Stack && !g.tab.doortools_door_op[op].ready_t.is_nan() {
        let p = { let o = &g.tab.doortools_door_op[op]; o.post.iter().find(|(gd, q)| q.lead && !o.men.contains(gd)).map(|(_, q)| *q) };
        { let o = &mut g.tab.doortools_door_op[op]; o.ready_t = f64::NAN; o.t0 = t; }
        let p = match p { Some(p) => p, None => return };
        let name = crate::orders::door_words(g, d);
        g.tab.doortools_door_op[op].post.set(lead, p);
        let fc = g.dr[d].frame_centre;
        door_tools_assign(g, op, lead, &stack_assign(&p, [fc[0], 1.05, fc[2]], &name));
        let what = alloc::format!("lead now {}", g.gd[lead].callsign.clone());
        door_tools_beat(g, op, &what);
    }
}

/// The stack for this module's entries: orders.ts's marks, each `tuck` further along the wall where the floor allows.
pub fn door_tools_stack_posts(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> {
    let ignore: &[GuardId] = ignore.unwrap_or(&[]);
    crate::orders::squad_orders_own_mark_first(g, d, side, men, &mut |g: &mut Game| door_tools_stack_tucked(g, d, side, men, ignore))
}

/// tuckedIn of stackTucked: the tuck does not take a man off his mark to put him on Sam, nor put a mark where a leaf swings.
fn tucked_in(g: &Game, own: &[V3], sam: V3, p: &crate::orders::StackPost, q: V3) -> bool {
    let mine = |x: V3| own.iter().any(|o| v3::dist_xz(*o, x) < crate::orders::OWN_EPS);
    (mine(p.pos) && v3::dist_xz(sam, q) < 0.7 && !mine(q))
        || (!p.lead && crate::doorway::doorway_stand(g, q) && !crate::doorway::doorway_stand(g, p.pos))
}

/// orders.ts's marks with the tuck applied: three or more tuck as files, the far man first.
fn door_tools_stack_tucked(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: &[GuardId]) -> Option<Vec<crate::orders::StackPost>> {
    let posts = crate::orders::squad_orders_stack_posts(g, d, side, men, Some(ignore))?;
    let sam = crate::orders::squad_orders_sam_stood(g, &crate::seats::here(g));
    let own = crate::orders::squad_orders_own_marks(g, &crate::seats::here(g), d, side, men);
    let width = g.dr[d].def.width;
    let tuck = g.k.doortools.tuck;
    if posts.len() > 2 {
        let mut out = posts.clone();
        let mut order: Vec<(usize, f64)> = Vec::new();
        for (i, p) in posts.iter().enumerate() { order.push((i, m::abs(along_of(g, d, p.pos) - width / 2.0))); }
        js::sort_by(&mut order, |a, b| b.1 - a.1);
        for (i, _) in order {
            let p = posts[i];
            let a = along_of(g, d, p.pos);
            let dir = if a > width / 2.0 { 1.0 } else { -1.0 };
            let off = off_of(g, d, p.pos, side);
            let q = door_pt(g, d, a + dir * tuck, off, side, None);
            if !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) && !crate::scene_collision::low_blocked(g, p.pos, q)
                && !out.iter().enumerate().any(|(j, o)| j != i && v3::dist_xz(o.pos, q) < 0.7)
                && !tucked_in(g, &own, sam, &p, q) && crate::stackroom::stack_room_keeps(g, d, side, q) {
                out[i] = crate::orders::StackPost { pos: q, ..p };
            }
        }
        return Some(out);
    }
    let mut res: Vec<crate::orders::StackPost> = Vec::new();
    for (i, p) in posts.iter().enumerate() {
        let a = along_of(g, d, p.pos);
        let dir = if a > width / 2.0 { 1.0 } else { -1.0 };
        let off = off_of(g, d, p.pos, side);
        let q = door_pt(g, d, a + dir * tuck, off, side, None);
        let ok = !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) && !crate::scene_collision::low_blocked(g, p.pos, q)
            && !posts.iter().enumerate().any(|(j, o)| j != i && v3::dist_xz(o.pos, q) < 0.7)
            && !tucked_in(g, &own, sam, p, q) && crate::stackroom::stack_room_keeps(g, d, side, q);
        res.push(if ok { crate::orders::StackPost { pos: q, ..*p } } else { *p });
    }
    Some(res)
}

/// Teammates outside this order stacked on this door, this face, under a plain 'stack up'.
pub(super) fn door_tools_holders(g: &crate::game::Game, seat: &crate::seats::SeatRef, d: DoorId, side: f64, men: &[GuardId]) -> Vec<GuardId> {
    if !crate::orders::squad_orders_big(g, &crate::seats::here(g)) { return Vec::new(); }
    let fc = g.dr[d].frame_centre;
    let jamb_out = seat.orders.knobs.jamb_out;
    let mut out: Vec<GuardId> = Vec::new();
    for gd in crate::orders::squad_orders_men(g, &crate::seats::here(g), None, None) {
        if men.contains(&gd) { continue; }
        let a = match crate::orders::squad_orders_of(&crate::seats::here(g), gd) { Some(a) => a, None => continue };
        let (is_stack, door, post) = { let x = &g.tab.orders_assignment[a]; (crate::orders::rows::id_is(&x.order, crate::orders::Is::Stacks), x.door, x.post) };
        if !is_stack || door != Some(d) { continue; }
        let p = match post { Some(p) => p, None => continue };
        let r = v3::dist_xz(p, fc);
        let off = off_of(g, d, p, side);
        if off > 0.0 && (r < 2.6 || (r < crate::stackroom::stack_view_reach(g, d) && off < jamb_out + 0.5)) { out.push(gd); }
    }
    out
}

/// `taken` and the marks already handed out, for a man stepping back to keep clear of.
fn taken_and_out(taken: &[V3], out: &JsMap<GuardId, crate::orders::StackPost>) -> Vec<V3> {
    let mut v: Vec<V3> = taken.to_vec();
    for p in out.values() { v.push(p.pos); }
    v
}

/// Where each holder steps back to, clear of `taken`: along his own wall first, then the other, then the open floor. `must`: None if one cannot and stands in the blast.
pub(super) fn door_tools_step_backs(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], taken: &[V3], must: Option<bool>) -> Option<JsMap<GuardId, crate::orders::StackPost>> {
    let must = must.unwrap_or(false);
    let width = g.dr[d].def.width;
    let fc = g.dr[d].frame_centre;
    let (off, jamb_past) = (g.sets[g.seat].orders.knobs.jamb_out, g.sets[g.seat].orders.knobs.jamb_past);
    let room_back = g.k.doortools.room_back.clone();
    let backblast_m = g.k.doortools.backblast_m;
    let mut out: JsMap<GuardId, crate::orders::StackPost> = JsMap::new();
    let mut walls: Vec<f64> = Vec::new();
    let away = { let a = door_pt(g, d, width / 2.0, 0.0, side, None); let b = door_pt(g, d, width / 2.0, 1.0, side, None); v3::yaw_to(a, b) };
    let out2 = crate::stackroom::stack_view_reach(g, d) > 2.6;
    for gd in door_tools_holders(g, &crate::seats::here(g), d, side, men) {
        let a_id = crate::orders::squad_orders_of(&crate::seats::here(g), gd).unwrap();
        let from = g.tab.orders_assignment[a_id].post.unwrap();
        let stack_side = g.tab.orders_assignment[a_id].stack_side;
        let hi = along_of(g, d, from) > width / 2.0;
        let mut found: Option<crate::orders::StackPost> = None;
        for dir in if hi { [1.0, -1.0] } else { [-1.0, 1.0] } {
            let mut prev: V3 = if (dir > 0.0) == hi { from } else { door_pt(g, d, if dir > 0.0 { width + jamb_past } else { -jamb_past }, off, side, None) };
            for x in room_back.iter().copied() {
                let q = door_pt(g, d, if dir > 0.0 { width + x } else { -x }, off, side, None);
                if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) || crate::scene_collision::low_blocked(g, prev, q) || !crate::stackroom::stack_room_keeps(g, d, side, q) { break; }
                prev = q;
                if out2 && v3::dist_xz(q, fc) < v3::dist_xz(from, fc) - 0.1 { continue; }
                if taken_and_out(taken, &out).iter().any(|o| v3::dist_xz(*o, q) < 0.85) || v3::dist_xz(q, crate::orders::squad_orders_sam_stood(g, &crate::seats::here(g))) < 0.7 { continue; }
                found = Some(crate::orders::StackPost { pos: q, face: if walls.contains(&dir) { away } else { v3::yaw_to(fc, q) }, stack_side, lead: false, group: None });
                walls.push(dir);
                break;
            }
            if found.is_some() { break; }
        }
        if found.is_none() {
            let rear = crate::orders::squad_orders_rear_posts(g, d, side, 6.0, &taken_and_out(taken, &out));
            for r in rear {
                if crate::stackroom::stack_room_has(g, d, side, r.pos) && !(out2 && v3::dist_xz(r.pos, fc) < v3::dist_xz(from, fc) - 0.1) {
                    found = Some(crate::orders::StackPost { pos: r.pos, face: v3::yaw_to(fc, r.pos), stack_side, lead: false, group: None });
                    break;
                }
            }
        }
        match found {
            Some(p) => { out.set(gd, p); }
            None => { if must && v3::dist_xz(from, fc) < backblast_m + 0.4 { return None; } }
        }
    }
    Some(out)
}

/// The stack for an order on a door other teammates hold: its marks as if they had stepped back, and where they step back to.
pub(super) fn door_tools_deal_stack(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Option<crate::doortools::DoorToolsDealStack> {
    let hold = door_tools_holders(g, &crate::seats::here(g), d, side, men);
    if hold.is_empty() {
        let posts = door_tools_stack_posts(g, d, side, men, None)?;
        return Some(DoorToolsDealStack { posts, back: JsMap::new() });
    }
    let mut posts = match door_tools_stack_posts(g, d, side, men, Some(&hold)) {
        Some(p) => p,
        None => { let posts = door_tools_stack_posts(g, d, side, men, None)?; return Some(DoorToolsDealStack { posts, back: JsMap::new() }); }
    };
    let (width, bang_file) = (g.dr[d].def.width, g.k.doortools.bang_file);
    let mut taken: Vec<V3> = posts.iter().map(|p| p.pos).collect();
    for p in posts.iter() {
        let a = along_of(g, d, p.pos);
        let off = off_of(g, d, p.pos, side);
        taken.push(door_pt(g, d, if a > width / 2.0 { a + bang_file } else { a - bang_file }, off, side, None));
    }
    let back = door_tools_step_backs(g, d, side, men, &taken, None).unwrap();
    if back.len() < hold.len() {
        let keys = back.keys();
        posts = door_tools_stack_posts(g, d, side, men, Some(&keys))?;
    }
    Some(DoorToolsDealStack { posts, back })
}

/// They step back: still on their 'stack up' on this door, from further along the wall, as the door's rear security.
pub(super) fn door_tools_make_room(g: &mut crate::game::Game, d: DoorId, back: &JsMap<GuardId, crate::orders::StackPost>) {
    let name = crate::orders::door_words(g, d);
    for (gd, post) in back.iter() {
        let a = match crate::orders::squad_orders_of(&crate::seats::here(g), gd) { Some(a) => a, None => continue };
        let grp = g.tab.orders_assignment[a].group;
        let p = post.pos;
        let hit = crate::scene_collision::raycast(g, [p[0], 1.5, p[2]], [m::sin(post.face), 0.0, m::cos(post.face)], 8.0);
        let r = m::max(1.5, (match hit { Some(h) => h.t, None => 8.0 }) - 0.5);
        crate::orders::squad_orders_assign(g, gd, &crate::orders::SquadOrdersAssignA {
            order: String::from(crate::orders::rows::stack::ORDER.id), post: Some(p), face: Some(post.face),
            watch: Some([p[0] + m::sin(post.face) * r, 1.1, p[2] + m::cos(post.face) * r]),
            door: Some(d), shy: Some(d), stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim), light: Some(true), stack_side: Some(post.stack_side),
            going: alloc::format!("making room · {}", name).into(), there: alloc::format!("rear security · {}", name).into(), ..Default::default()
        });
        let a2 = crate::orders::squad_orders_of(&crate::seats::here(g), gd).unwrap();
        g.tab.orders_assignment[a2].group = grp;
    }
}

/// inIt of fragBehind: beyond the door, in a frag's reach, with a line to where it lands.
fn frag_in_it(g: &mut Game, d: DoorId, side: f64, land: V3, p: V3) -> bool {
    off_of(g, d, p, side) < 0.0 && v3::dist_xz(p, land) < g.k.frag.wound_m + 1.0
        && (crate::frag::exposed_to(g, land, p, [p[0], p[1] + 1.6, p[2]], None) > 0.0 || !crate::scene_collision::solid_blocked(g, [land[0], 0.9, land[2]], [p[0], 1.1, p[2]], None))
}

/// Anybody of the team on the far side of the door with a line to where a frag through it would land: his callsign, or 'you'.
pub fn door_tools_frag_behind(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Option<String> {
    let deep = g.k.frag.order_in;
    let at = door_tools_land_for(g, d, side, deep);
    let land: V3 = [at[0], 0.3, at[2]];
    for gd in g.guards.clone() {
        if g.team(gd).ours() && !crate::condition::down(g, g.gd[gd].ch) && !men.contains(&gd) {
            let p = g.pos_of(gd);
            if frag_in_it(g, d, side, land, p) { return Some(g.gd[gd].callsign.clone()); }
        }
    }
    let pl = g.player;
    let p = g.player_pos(pl);
    if !crate::condition::down(g, g.pl[pl].ch) && frag_in_it(g, d, side, land, p) { Some(String::from("you")) } else { None }
}

/// A teammate outside this order on the far side of the door, in the open reach of a charge on it.
pub(super) fn door_tools_behind(g: &crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Option<GuardId> {
    if !crate::orders::squad_orders_big(g, &crate::seats::here(g)) { return None; }
    let width = g.dr[d].def.width;
    let inn = door_pt(g, d, width / 2.0, 0.45, -side, Some(1.1));
    let (fc, stun_m) = (g.dr[d].frame_centre, g.k.doortools.stun_m);
    for gd in crate::orders::squad_orders_men(g, &crate::seats::here(g), None, None) {
        let p = g.pos_of(gd);
        if !men.contains(&gd) && off_of(g, d, p, side) < 0.0 && v3::dist_xz(p, fc) < stun_m && !crate::scene_collision::solid_blocked(g, inn, [p[0], 1.1, p[2]], None) { return Some(gd); }
    }
    None
}
