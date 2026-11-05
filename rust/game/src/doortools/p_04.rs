//! src/game/doortools.ts lines 1351 to 1743
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- seam: the one step skeleton 3 still cannot express; the next skeleton changes this and not the bodies

/// g.orders.ownMarkFirst: generic over P in the TypeScript; the stub's item has only `pos`, so a post cannot pass through it whole.
fn seam_own_mark_first(g: &mut Game, d: DoorId, side: f64, men: &[GuardId], ask: &mut dyn FnMut(&mut Game) -> Option<Vec<crate::orders::StackPost>>) -> Option<Vec<crate::orders::StackPost>> { todo!("needs: squad_orders_own_mark_first generic over the post type") }

// ---- small private helpers

/// WEAPONS[w].pellets?.breach
fn weapon_breach(g: &Game, w: crate::weapons::WeaponId) -> Option<crate::weapons::PelletSpecBreach> { g.m.weapons.weapons.get(w).and_then(|d| d.pellets.as_ref()).and_then(|p| p.breach) }

fn flat(a: V3, b: V3) -> f64 { v3::dist_xz(a, b) }
fn nav_blocked(g: &mut Game, x: f64, z: f64) -> bool { crate::scene_collision::nav_grid_is_blocked(g, x, z) }
fn pos_of(g: &Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }
fn callsign(g: &Game, gd: GuardId) -> String { g.gd[gd].callsign.clone() }
fn is_wedge(g: &Game, d: DoorId) -> bool { g.dr[d].lock_id.starts_with("wedge:") }
/// op.post.get(m)?.pos ?? m.char.pos
fn post_or_pos(g: &Game, op: DoorOpId, m: GuardId) -> V3 {
    match g.tab.doortools_door_op[op].post.get(m) { Some(p) => p.pos, None => pos_of(g, m) }
}
/// DOORTOOLS.blowKey.replace('Key', '')
fn key_word(g: &Game) -> String { g.k.doortools.blow_key.replacen("Key", "", 1) }
fn by_sam(g: &Game, c: ChargeId) -> bool { g.tab.doortools_set_charge[c].by == crate::ownblow::SAM_BY }
/// this.ops.some(o => o.kind === 'charge' && o.door === d)
fn charge_order_on(g: &Game, d: DoorId) -> bool {
    g.door_tools.ops.iter().any(|o| { let x = &g.tab.doortools_door_op[*o]; x.kind == DoorToolKind::Charge && x.door == d })
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
        going: alloc::format!("stacking · {}", name), there: alloc::format!("stacked · {}", name), ..Default::default()
    }
}

/// A mark of this stack stands in the lane of another open door with unknown floor beyond it: the rearmost man covers it first.   (doortools.ts laneCover)
pub fn door_tools_lane_cover(g: &mut crate::game::Game, op: DoorOpId) {
    let d = g.tab.doortools_door_op[op].door;
    if g.tab.doortools_door_op[op].men.len() < 2 { return; }
    let (men, lead, fc, width) = { let o = &g.tab.doortools_door_op[op]; (o.men.clone(), o.lead, g.dr[d].frame_centre, g.dr[d].def.width) };
    let mut cand: Vec<(GuardId, f64)> = Vec::new();
    for m in men.iter().copied() {
        let o = &g.tab.doortools_door_op[op];
        if m == lead || o.rear.has(m) { continue; }
        if let Some(p) = o.post.get(m) { cand.push((m, flat(p.pos, fc))); }
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
    if nav_blocked(g, q[0], q[2]) || crate::doorway::doorway_in_way(g, q) || flat(q, fc) < width / 2.0 + 1.0
        || crate::scene_collision::solid_blocked(g, [q[0], 0.45, q[2]], [c[0], 0.45, c[2]], None)
        || crate::scene_collision::solid_blocked(g, [q[0], 0.45, q[2]], [a_post[0], 0.45, a_post[2]], None)
        || men.iter().any(|m| *m != gd && flat(post_or_pos(g, op, *m), q) < 0.9) { return; }
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
        going: words.clone(), there: words, search: Some(true), search_t: Some(now), ..Default::default()
    });
    let cover = *g.tab.doortools_door_op[op].mine.get(gd).unwrap();
    g.tab.doortools_door_op[op].lane = Some(DoorToolsDoneItemLane { gd, door: dd, cover, back, at: None });
    g.stats.doorway.covered += 1;
    let what = alloc::format!("lane:{} covers {}", callsign(g, gd), g.dr[dd].def.name);
    door_tools_beat(g, op, &what);
}

/// The lane's cover man comes up to his mark, unless the order has since given him another.   (doortools.ts laneIn)
pub fn door_tools_lane_in(g: &mut crate::game::Game, op: DoorOpId) {
    let l = g.tab.doortools_door_op[op].lane.take();
    let l = match l { Some(x) => x, None => return };
    let o = &g.tab.doortools_door_op[op];
    if g.gd[l.gd].state == crate::game::GuardState::Dead || !o.men.contains(&l.gd) || o.mine.get(l.gd).copied() != Some(l.cover) { return; }
    let b = l.back;
    door_tools_assign(g, op, l.gd, &DoorToolsAssignA {
        post: b.post, face: b.face, watch: b.watch, stance: b.stance, upper: b.upper, move_: b.move_, stack_side: b.stack_side,
        going: b.going, there: b.there, ..Default::default()
    });
    let what = alloc::format!("lane:{} comes up", callsign(g, l.gd));
    door_tools_beat(g, op, &what);
}

/// Breach with a charge: the nearest man goes to the door, the rest stand off along the wall; `by_hand`, the player's own charge stands on it.   (doortools.ts startCharge)
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
            going: String::from("standing off"), there: alloc::format!("stood off · {}", name), ..Default::default()
        });
    }
    let names: Vec<String> = men.iter().map(|m| callsign(g, *m)).collect();
    let tail = if by_hand { String::from("stand off on a charge set by hand") } else { alloc::format!("{} sets it", callsign(g, lead)) };
    let what = alloc::format!("ordered · {} · {}", names.join(", "), tail);
    door_tools_beat(g, op, &what);
    DoorToolsStartCharge::Arm1(())
}

/// A charge the player set stands on this face with no squad order on the door: the row is 'stand off'.   (doortools.ts looseOn)
pub fn door_tools_loose_on(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) -> bool {
    let (door, side) = match t { crate::orders::OrderTarget::Door(x) => (x.door, x.side), _ => return false };
    let c = match door_tools_charge_on(g, door) { Some(c) => c, None => return false };
    by_sam(g, c) && g.tab.doortools_set_charge[c].side == side && !charge_order_on(g, door)
}

/// Can these men stand off from a charge on that face: true, or the first why-not.   (doortools.ts standoffFits)
pub fn door_tools_standoff_fits(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget, own: Option<bool>) -> crate::doortools::DoorToolsStandoffFits {
    let own = own.unwrap_or(false);
    let (d, side) = match t { crate::orders::OrderTarget::Door(x) => (x.door, x.side), _ => return DoorToolsStandoffFits::Text(String::from("no door")) };
    let posts = match door_tools_standoff_posts(g, d, side, men.len() as f64, Some(men)) { Some(p) => p, None => return DoorToolsStandoffFits::Text(String::from("no room to stand off.")) };
    if !own {
        if let Some(mate) = door_tools_behind(g, d, side, men) { return DoorToolsStandoffFits::Text(alloc::format!("{} is behind that door.", callsign(g, mate))); }
    }
    let taken: Vec<V3> = posts.iter().map(|x| x.pos).collect();
    if door_tools_step_backs(g, d, side, men, &taken, Some(true)).is_none() { return DoorToolsStandoffFits::Text(String::from("no room to stand off.")); }
    match door_tools_standoff_walk(g, men, d, &posts) { Some(w) => DoorToolsStandoffFits::Text(w), None => DoorToolsStandoffFits::Bool(true) }
}

/// Can these men walk to these marks, by the very dealing the issue makes: the first why-not, or None.   (doortools.ts standoffWalk)
pub fn door_tools_standoff_walk(g: &mut crate::game::Game, men: &[GuardId], d: DoorId, posts: &[crate::orders::StackPost]) -> Option<String> {
    for (gd, p) in door_tools_standoff_deal(g, men, d, posts) {
        if let Some(w) = crate::orders::squad_orders_reach(g, gd, p.pos) { return Some(w); }
    }
    None
}

/// One man's test for close_words, asked inside without_leaf: blow()'s own reach and line.
fn close_ask(g: &mut Game, d: DoorId, side: f64, out: V3, inn: V3, p: V3, who: String, sam: bool, near: &mut Vec<String>, far: &mut Vec<String>) {
    let beyond = off_of(g, d, p, side) < 0.0;
    let dist = flat(p, g.dr[d].frame_centre);
    let (stun_m, backblast_m) = (g.k.doortools.stun_m, g.k.doortools.backblast_m);
    if if sam { dist >= (if beyond { stun_m } else { backblast_m }) } else { dist > stun_m || (!beyond && dist >= backblast_m) } { return; }
    if !crate::scene_collision::solid_blocked(g, if beyond { inn } else { out }, [p[0], 1.1, p[2]], None) {
        if beyond { far.push(who); } else { near.push(who); }
    }
}

/// Who a charge on a door would catch if it went now, as words for the row; kept a quarter second a charge.   (doortools.ts closeWords)
pub fn door_tools_close_words(g: &mut crate::game::Game, c_2: ChargeId) -> String {
    let c = c_2;
    if let Some(kept) = g.door_tools.close_at.get(c) { if m::abs(g.time - kept.t) < 0.25 { return kept.w.clone(); } }
    let (d, side) = (g.tab.doortools_set_charge[c].door, g.tab.doortools_set_charge[c].side);
    let width = g.dr[d].def.width;
    let into = -side;
    let out = door_pt(g, d, width / 2.0, 0.5, side, Some(1.1));
    let inn = door_pt(g, d, width / 2.0, 0.45, into, Some(1.1));
    let (mut near, mut far): (Vec<String>, Vec<String>) = (Vec::new(), Vec::new());
    crate::doorblast::without_leaf(g, d, &mut |g: &mut Game| {
        let pl = g.player;
        if !g.pl[pl].down && g.puppet.is_none() && !g.touring {
            let p = g.ch[g.pl[pl].ch].pos;
            close_ask(g, d, side, out, inn, p, String::from("you"), true, &mut near, &mut far);
        }
        for gd in g.guards.clone() {
            if g.gd[gd].unit.is_some() && g.gd[gd].team == crate::teams::Team::Operative && g.gd[gd].state != crate::game::GuardState::Dead {
                let (p, who) = (pos_of(g, gd), callsign(g, gd));
                close_ask(g, d, side, out, inn, p, who, false, &mut near, &mut far);
            }
        }
    });
    let mut w = String::new();
    if !near.is_empty() { w.push_str(&alloc::format!(" · too close: {}", near.join(", "))); }
    if !far.is_empty() { w.push_str(&alloc::format!(" · behind the door: {}", far.join(", "))); }
    let t = g.time;
    g.door_tools.close_at.set(c, DoorToolsCloseAtValue { t, w: w.clone() });
    w
}

/// The face of this door the player could set a charge on by hand right now (None: none).   (doortools.ts samFace)
pub fn door_tools_sam_face(g: &mut crate::game::Game, d: DoorId) -> Option<f64> {
    let pl = g.player;
    if g.puppet.is_some() || g.touring || g.pl[pl].dragging.is_some() || g.pl[pl].holding.is_some() || crate::doors::door_off(g, d) || g.dr[d].def.steel == Some(true)
        || is_open(g, d) || door_tools_charge_on(g, d).is_some() || charge_order_on(g, d) { return None; }
    door_tools_sync(g);
    if g.door_tools.charges <= 0.0 { return None; }
    let p = g.ch[g.pl[pl].ch].pos;
    Some(if off_of(g, d, p, 1.0) >= 0.0 { 1.0 } else { -1.0 })
}

/// A door's rows: a set charge is said on the door's own line; else, in reach with free hands, 'set a charge'.   (doortools.ts samRows)
pub fn door_tools_sam_rows(g: &mut crate::game::Game, it: &mut crate::game::Interactable, d: DoorId, verbs: &mut Vec<crate::game::Interactable>, free: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>) -> crate::game::Interactable) {
    let c = door_tools_charge_on(g, d);
    let key = key_word(g);
    let f = crate::ownblow::blow_key_word(false, &key);
    if let Some(c) = c {
        let close = door_tools_close_words(g, c);
        let is = door_tools_key_is(g, c);
        let w = alloc::format!("{} · charge set{}", close, crate::ownblow::blow_tail(is, &key, None));
        it.line1.push_str(&w);
        for v in verbs.iter_mut() { v.line1.push_str(&w); }
        return;
    }
    if !free || door_tools_sam_face(g, d).is_none() { return; }
    let text = alloc::format!("set a charge · {} left", g.door_tools.charges);
    let r = row(g, it.clone(), crate::game::Verb::Charge, text, Some(alloc::format!("{}  blows it once set", f)));
    verbs.push(r);
}

/// Is this set charge the squad's key's to blow: one the squad set, or his own while the squad works its door.   (doortools.ts keyIs)
pub fn door_tools_key_is(g: &mut crate::game::Game, c_2: ChargeId) -> bool {
    let c = c_2;
    if !by_sam(g, c) { return true; }
    let door = g.tab.doortools_set_charge[c].door;
    if g.door_tools.ops.iter().any(|o| g.tab.doortools_door_op[*o].door == door) { return true; }
    let men = crate::orders::squad_orders_men(g, None, None);
    door_tools_stacks_of(g, &men).iter().any(|x| x.stack == door)
}

/// The nearest charge his own hands set that is on no word still to go.   (doortools.ts samSet)
pub fn door_tools_sam_set(g: &mut crate::game::Game) -> Option<ChargeId> {
    door_tools_nearest(g, Some(&mut |g: &mut Game, c: ChargeId| by_sam(g, c) && !door_tools_on_pending_word(g, c)))
}

/// Where a charge is, in the row's words.   (doortools.ts samWhere)
pub fn door_tools_sam_where(g: &mut crate::game::Game, c_2: ChargeId) -> String {
    let d = g.tab.doortools_set_charge[c_2].door;
    alloc::format!("the {}", crate::orders::door_words(g, d))
}

/// The charge going on under his hands: where on the leaf, and the face's normal.   (doortools.ts samJob)
pub fn door_tools_sam_job(g: &mut crate::game::Game) -> Option<crate::doortools::DoorToolsSamJob> {
    let pl = g.player;
    let s = g.pl[pl].charging.clone()?;
    let d = s.door;
    let o = door_pt(g, d, 0.0, 0.0, s.side, None);
    let n1 = door_pt(g, d, 0.0, 1.0, s.side, None);
    let width = g.dr[d].def.width;
    let p = door_pt(g, d, width - 0.22, 0.025, s.side, Some(1.05));
    Some(DoorToolsSamJob { p, n: [n1[0] - o[0], 0.0, n1[2] - o[2]], at: s.at })
}

/// How far along it is, 0 to 1.   (doortools.ts samProgress)
pub fn door_tools_sam_progress(g: &mut crate::game::Game) -> f64 {
    let pl = g.player;
    match &g.pl[pl].charging { Some(s) => v3::clamp((g.time - s.t0) / g.k.doortools.set_secs, 0.0, 1.0), None => 0.0 }
}

/// Let go of it: nothing was spent (true: there was one).   (doortools.ts cancelSam)
pub fn door_tools_cancel_sam(g: &mut crate::game::Game) -> bool {
    let pl = g.player;
    if g.pl[pl].charging.is_none() { return false; }
    g.pl[pl].charging = None;
    crate::game::game_msg(g, "charge not set");
    true
}

/// Each frame: the row performed starts it; his hands on it for setSecs set it; anything that takes him off it breaks it off.   (doortools.ts samTick)
pub fn door_tools_sam_tick(g: &mut crate::game::Game) {
    let pl = g.player;
    let set_secs = g.k.doortools.set_secs;
    let (act_some, act_door, mine) = match &g.act {
        Some(a) => (true, a.door, a.kind == crate::game::InteractableKind::Door && a.verb == Some(crate::game::Verb::Charge) && a.door.is_some()),
        None => (false, None, false),
    };
    let s = match g.pl[pl].charging.clone() {
        Some(s) => s,
        None => {
            if !mine || crate::wallcharge::wall_charge_setting(g) { return; }
            let d = act_door.unwrap();
            let side = match door_tools_sam_face(g, d) { Some(x) => x, None => return };
            let p = g.ch[g.pl[pl].ch].pos;
            let t0 = g.time;
            g.pl[pl].charging = Some(crate::game::PlayerCharging { door: d, side, t0, at: [p[0], 0.0, p[2]] });
            return;
        }
    };
    let d = s.door;
    let c = g.pl[pl].ch;
    let p = g.ch[c].pos;
    if g.pl[pl].down || g.pl[pl].sprinting || g.pl[pl].dragging.is_some() || g.pl[pl].holding.is_some() || g.pl[pl].takedown.is_some() || g.pl[pl].kick.is_some() || g.puppet.is_some()
        || crate::doors::door_off(g, d) || is_open(g, d) || door_tools_charge_on(g, d).is_some()
        || charge_order_on(g, d)
        || off_of(g, d, p, s.side) <= 0.0 || flat(p, s.at) > SAM_SHIFT || (act_some && !(mine && act_door == Some(d))) {
        door_tools_cancel_sam(g);
        return;
    }
    if crate::workstow::sam_stowing(g) {
        let t = g.time;
        if let Some(x) = g.pl[pl].charging.as_mut() { x.t0 = t; }
        return;
    }
    let j = door_tools_sam_job(g).unwrap();
    let hand = if crate::game::player_pistol_up(g, pl) { Some(0.0) } else { None };
    let el = g.time - s.t0;
    crate::handwork::charge_hands(g, c, j.p, j.n, el, set_secs, hand);
    if g.time - s.t0 < set_secs { return; }
    g.pl[pl].charging = None;
    if !door_tools_take(g, DoorToolsTakeWhat::Charge) { crate::game::game_msg(g, "no charges left"); return; }
    g.door_tools.sam_spent += 1.0;
    let row = SetCharge { door: d, side: s.side, t: g.time, by: String::from(crate::ownblow::SAM_BY) };
    let id = g.tab.doortools_set_charge.push(row);
    g.door_tools.set.push(id);
    let at = crate::doors::door_pos(g, d);
    crate::audio_iface::game_audio_like_play(g, "lockPick", Some(at), Some(0.6), None);
    let key = key_word(g);
    let text = alloc::format!("charge set on the {}{}", crate::orders::door_words(g, d), crate::ownblow::blow_tail(false, &key, Some(". ")));
    crate::game::game_msg(g, &text);
    let what = alloc::format!("charge set by hand on {}", g.dr[d].def.name);
    door_tools_note(g, &what);
}

/// The charges that stand set: drawn; one whose door has come open or off its hinges is off it, back in the count.   (doortools.ts tickSet)
pub fn door_tools_tick_set(g: &mut crate::game::Game) {
    for c in g.door_tools.set.clone() {
        let d = g.tab.doortools_set_charge[c].door;
        let gone = crate::doors::door_off(g, d);
        if gone || is_open(g, d) {
            if let Some(i) = g.door_tools.set.iter().position(|x| *x == c) { g.door_tools.set.remove(i); }
            door_tools_refund(g, DoorToolsRefundWhat::Charge);
            if by_sam(g, c) { g.door_tools.sam_spent = m::max(0.0, g.door_tools.sam_spent - 1.0); }
            let words = crate::orders::door_words(g, d);
            let text = if gone { alloc::format!("the {} is off its hinges, so the charge came off with it", words) } else { alloc::format!("the {} came open, so the charge came off", words) };
            crate::game::game_msg(g, &text);
            let what = alloc::format!("charge off {}: the door {}", g.dr[d].def.name, if gone { "is gone" } else { "opened" });
            door_tools_note(g, &what);
            continue;
        }
        door_tools_draw_charge(g, c);
    }
}

/// Who stands off where for a charge: the man nearest the door on the first mark, the rest dealt the others; first entry = the setter.   (doortools.ts standoffDeal)
pub fn door_tools_standoff_deal(g: &mut crate::game::Game, men: &[GuardId], d: DoorId, posts: &[crate::orders::StackPost]) -> Vec<(GuardId, crate::orders::StackPost)> {
    let fc = g.dr[d].frame_centre;
    let mut by: Vec<(GuardId, f64)> = men.iter().map(|m| (*m, flat(pos_of(g, *m), fc))).collect();
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

/// The man who sets the charge: to the lock.   (doortools.ts toLock)
pub fn door_tools_to_lock(g: &mut crate::game::Game, op: DoorOpId) {
    let (d, side, lead) = { let o = &g.tab.doortools_door_op[op]; (o.door, o.side, o.lead) };
    let width = g.dr[d].def.width;
    let from = door_pt(g, d, width / 2.0, 1.0, side, None);
    let ws = crate::doors::doors_work_spot(g, d, from, Some(0.5), None);
    let going = alloc::format!("charge · {}", crate::orders::door_words(g, d));
    door_tools_assign(g, op, lead, &DoorToolsAssignA {
        post: Some(ws.pos), face: Some(ws.yaw), watch: None, stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::None),
        going, there: String::from("setting the charge"), ..Default::default()
    });
}

/// A silent squad at a door locked against it: the lead to the keyway, to pick it.   (doortools.ts toPick)
pub fn door_tools_to_pick(g: &mut crate::game::Game, op: DoorOpId) {
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
        going, there: String::from("picking the lock"), ..Default::default()
    });
}

/// A lock a pick opens: a cylinder, not a steel door's bolt, a keypad's, nor a wedge.   (doortools.ts pickable)
pub fn door_tools_pickable(g: &mut crate::game::Game, d: DoorId) -> bool {
    g.dr[d].def.steel != Some(true) && crate::devices::devices_pad_of(g, d).is_none() && !is_wedge(g, d)
}

/// His hands off the lock.   (doortools.ts offLock)
pub fn door_tools_off_lock(g: &mut crate::game::Game, op: DoorOpId) {
    let by = g.tab.doortools_door_op[op].kick.as_ref().and_then(|k| k.by);
    if let Some(by) = by {
        let c = g.gd[by].ch;
        g.ch[c].anim.lockpick = false;
        with_kick(g, op, |k| k.by = None);
    }
}

/// One frame of a lock being picked: the lead at the keyway with the player's own pick; it gives, he goes back to his jamb and the order goes on.   (doortools.ts pickStep)
pub fn door_tools_pick_step(g: &mut crate::game::Game, op: DoorOpId) {
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
    let busy = a.map_or(false, |a| g.tab.orders_assignment[a].yield_t >= 0.0) || t < g.gd[lead].dazzled_until || t < g.gd[lead].stagger_until
        || (g.gd[lead].script.as_ref().map_or(false, |s| s.over.is_some()) && (crate::silent::silent_compromised(g) || !crate::silent::squad_silent(g)));
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
    let on = flat(p, k.pos) < 0.4 && m::abs(v3::wrap_angle(g.ch[c].body_yaw - k.yaw)) < 0.35;
    if !on {
        door_tools_off_lock(g, op);
        if t - g.tab.doortools_door_op[op].t0 < kick_cap { return; }
        if flat(p, k.pos) > 1.2 {
            crate::game::game_say(g, lead, "can't get on the door.", None, None, None);
            door_tools_end(g, op, "never made the lock");
            return;
        }
    }
    if g.tab.doortools_door_op[op].kick.unwrap().by.is_none() { door_tools_beat(g, op, "picking"); }
    g.ch[c].anim.lockpick = true;
    with_kick(g, op, |k| k.by = Some(lead));
    let who = g.ch[c].id;
    if !crate::doors::doors_pick(g, d, who, dt) { return; }
    if crate::silent::silent_pick(g) { g.stats.silent.picked += 1; }
    g.door_tools.stats.picks += 1.0;
    door_tools_beat(g, op, "picked");
    door_tools_off_lock(g, op);
    door_tools_back_to_jamb(g, op, lead);
    with_kick(g, op, |k| k.done_t = Some(t));
}

/// shell(m) of toGun: a breaching shotgun that can take this lock.
fn gun_shell(g: &mut Game, d: DoorId, m: GuardId) -> bool { !is_wedge(g, d) && crate::shotgun::carries_breacher(g, m) }
/// can(m) of toGun: that, or a hammer in his big slot.
fn gun_can(g: &mut Game, d: DoorId, m: GuardId) -> bool { gun_shell(g, d, m) || crate::bigkit::carries_hammer(g, m) }

/// A door locked against the team and a man of this order with a breaching shotgun or a hammer: he goes to the lock with it. False: the boot.   (doortools.ts toGun)
pub fn door_tools_to_gun(g: &mut crate::game::Game, op: DoorOpId) -> bool {
    let (d, side, lead, men) = { let o = &g.tab.doortools_door_op[op]; if o.no_gun == Some(true) { return false; } (o.door, o.side, o.lead, o.men.clone()) };
    let fc = g.dr[d].frame_centre;
    let mut held: Vec<(GuardId, f64)> = men.iter().copied().filter(|m| g.tab.doortools_door_op[op].post.has(*m)).map(|m| (m, flat(pos_of(g, m), fc))).collect();
    js::sort_by(&mut held, |a, b| a.1 - b.1);
    let mut by: Option<GuardId> = None;
    if gun_shell(g, d, lead) { by = Some(lead); }
    if by.is_none() { for (m, _) in held.iter() { if gun_shell(g, d, *m) { by = Some(*m); break; } } }
    if by.is_none() && gun_can(g, d, lead) { by = Some(lead); }
    if by.is_none() { for (m, _) in held.iter() { if gun_can(g, d, *m) { by = Some(*m); break; } } }
    let by = match by { Some(x) => x, None => return false };
    let hammer = !gun_shell(g, d, by);
    let mut avoid: Vec<V3> = men.iter().copied().filter(|m| *m != by).map(|m| post_or_pos(g, op, m)).collect();
    avoid.push(g.ch[g.pl[g.player].ch].pos);
    let mark = if hammer { None } else { Some(post_or_pos(g, op, by)) };
    let close = if g.k.bigslot.on && (hammer || crate::kit::kit_big(g, by) == crate::kit::BigKit::Shotgun) {
        Some(if hammer { crate::shotgun::BreachSpotClose::Hammer } else { crate::shotgun::BreachSpotClose::Shotgun })
    } else { None };
    let sp = match crate::shotgun::breach_spot(g, d, side, Some(&avoid), mark, close) { Some(s) => s, None => return false };
    let silent = crate::silent::squad_silent(g);
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
        going, there: String::from("breaching"), ..Default::default()
    });
    let locked_now = door_tools_locked_out(g, d, side);
    let line = if locked_now { if hammer { "locked. hammer." } else { "locked. shotgun." } } else if hammer { "hammer." } else { "shotgun." };
    crate::game::game_say(g, by, line, None, None, None);
    let what = alloc::format!("{}:{}{}", if hammer { "hammer" } else { "gun" }, callsign(g, by), if sp.beside { "" } else { " (in front)" });
    door_tools_beat(g, op, &what);
    true
}

/// The gun is off this door's lock for good: his mark again, and open()'s question with the gun ruled out.   (doortools.ts gunOff)
pub fn door_tools_gun_off(g: &mut crate::game::Game, op: DoorOpId, why: &str) {
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

/// One frame of a lock being shot off: on his spot, square to the lock, a shell; the lock still holding after SHOTGUN.shells, the boot.   (doortools.ts gunStep)
pub fn door_tools_gun_step(g: &mut crate::game::Game, op: DoorOpId) {
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
    if !gun.silent && crate::silent::squad_silent(g) {
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
    let busy = a.map_or(false, |a| g.tab.orders_assignment[a].yield_t >= 0.0) || t < g.gd[by].dazzled_until || t < g.gd[by].stagger_until;
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
    let up = !kit || (flat(g.ch[c].pos, k.pos) < 1.5 && crate::kit::kit_big(g, by) == crate::kit::BigKit::Shotgun && crate::bigkit::gun_up(g, by));
    let w1 = g.fa[g.gd[by].weapon].def;
    let mut b = weapon_breach(g, w1);
    if b.is_none() && kit && crate::kit::kit_big(g, by) == crate::kit::BigKit::Shotgun { b = weapon_breach(g, crate::weapons::WeaponId::M870); }
    let b = match b { Some(x) => x, None => { door_tools_gun_off(g, op, "no gun in his hands"); return; } };
    let (p, muzzle, yaw) = (g.ch[c].pos, g.ch[c].muzzle, g.ch[c].body_yaw);
    let t0 = g.tab.doortools_door_op[op].t0;
    let own = gun.own == Some(true);
    let (own_slack, own_beat, aim_cap, shells) = (g.k.shotgun.own_slack, g.k.shotgun.own_beat, g.k.shotgun.aim_cap, g.k.shotgun.shells);
    let on = up && flat(p, k.pos) < (if own { own_slack } else { 0.4 }) && m::abs(v3::wrap_angle(yaw - v3::yaw_to(p, gun.at))) < 0.35 && v3::dist(muzzle, gun.at) <= b.reach_m
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
        let dur = g.m.weapons.weapons.get(g.fa[wid].def).unwrap().reload.dur;
        with_gun(g, op, |x| x.next_t = t + dur);
        crate::game::game_say(g, by, "reloading.", None, None, None);
        return;
    }
    let lock = crate::doors::door_locked(g, d) && crate::doors::door_latched(g, d);
    if !(kit && crate::bigkit::big_broken()) {
        crate::combat::fire_weapon(g, c, gun.at, false, Some(&crate::combat::FireOpts { steady: Some(true), spread: Some(0.0), ..Default::default() }));
        if !lock && gun.shut == Some(true) && !is_open(g, d) {
            let (from, who) = (g.ch[c].pos, g.ch[c].id);
            crate::doors::doors_kick_in(g, d, from, who, Some(true));
        }
    }
    let cd = g.m.weapons.weapons.get(g.fa[g.gd[by].weapon].def).unwrap().cd;
    with_gun(g, op, |x| { x.n += 1.0; x.next_t = t + cd; });
    g.tab.doortools_door_op[op].t0 = t;
    if !crate::doors::door_latched(g, d) {
        with_kick(g, op, |k| k.t = t);
        g.door_tools.stats.shot += 1.0;
        door_tools_beat(g, op, "shotgun");
        crate::bigkit::gun_down(g, by);
        door_tools_back_to_jamb(g, op, by);
        if !is_open(g, d) && m::abs(crate::doors::door_vel(g, d)) < 0.5 {
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

/// One frame of a lock being hammered: on his spot, one blow where a boot would do, BIGSLOT.swings where the frame holds one.   (doortools.ts hammerStep)
pub fn door_tools_hammer_step(g: &mut crate::game::Game, op: DoorOpId) {
    let kick_cap = g.k.doortools.kick_cap;
    let (d, side) = { let o = &g.tab.doortools_door_op[op]; (o.door, o.side) };
    let t = g.time;
    let k = g.tab.doortools_door_op[op].kick.unwrap();
    let gun = k.gun.unwrap();
    let by = gun.by;
    let c = g.gd[by].ch;
    let h = crate::bigkit::hammer_out(g, by, gun.at);
    let (p, yaw) = (g.ch[c].pos, g.ch[c].body_yaw);
    let on = flat(p, k.pos) < 0.4 && m::abs(v3::wrap_angle(yaw - v3::yaw_to(p, gun.at))) < 0.35;
    if !on {
        if t - g.tab.doortools_door_op[op].t0 >= kick_cap + g.k.shotgun.aim_cap { door_tools_gun_off(g, op, "never got onto the lock"); }
        return;
    }
    if gun.next_t == 0.0 {
        let by_t = h.t0 + crate::bigkit::big_out_by(g, crate::bigkit::BigOutByKind::Hammer);
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
    let held = if gun.shut == Some(true) { crate::doors::door_locked(g, d) && crate::doors::doors_boot_holds(g, d, p) } else { door_tools_frame_held(g, d, side) };
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
    if !is_open(g, d) && m::abs(crate::doors::door_vel(g, d)) < 0.5 {
        { let o = &mut g.tab.doortools_door_op[op]; o.kick = None; o.no_gun = Some(true); }
        door_tools_open(g, op);
    } else {
        door_tools_opened(g, op);
    }
}

/// A breaching shotgun out of a big slot is in nobody's hands without a lock to shoot: the net under every way out of the lock stage.   (doortools.ts slingTick)
pub fn door_tools_sling_tick(g: &mut crate::game::Game) {
    for gd in g.guards.clone() {
        if !g.gd[gd].slung.as_ref().map_or(false, |s| s.out) { continue; }
        let on_lock = g.door_tools.ops.iter().any(|o| {
            let x = &g.tab.doortools_door_op[*o];
            x.phase == Phase::Kick && x.kick.and_then(|k| k.gun).map_or(false, |u| u.by == gd && u.hammer != Some(true))
        });
        if !on_lock { crate::bigkit::gun_down(g, gd); }
    }
}

/// The man this op turned on is gone and the next has it: a job only he was at starts over with the new man walking to it.   (doortools.ts newLead)
pub fn door_tools_new_lead(g: &mut crate::game::Game, op: DoorOpId) {
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
        let what = alloc::format!("setter now {}", callsign(g, lead));
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
                going, there: String::from("picking the lock"), ..Default::default()
            });
        }
        let what = alloc::format!("picker now {}", callsign(g, lead));
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
            going, there: String::from("breaching"), ..Default::default()
        });
        let what = alloc::format!("kicker now {}", callsign(g, lead));
        door_tools_beat(g, op, &what);
    } else if phase == Phase::Stack && !g.tab.doortools_door_op[op].ready_t.is_nan() {
        let p = { let o = &g.tab.doortools_door_op[op]; o.post.iter().find(|(gd, q)| q.lead && !o.men.contains(gd)).map(|(_, q)| *q) };
        { let o = &mut g.tab.doortools_door_op[op]; o.ready_t = f64::NAN; o.t0 = t; }
        let p = match p { Some(p) => p, None => return };
        let name = crate::orders::door_words(g, d);
        g.tab.doortools_door_op[op].post.set(lead, p);
        let fc = g.dr[d].frame_centre;
        door_tools_assign(g, op, lead, &stack_assign(&p, [fc[0], 1.05, fc[2]], &name));
        let what = alloc::format!("lead now {}", callsign(g, lead));
        door_tools_beat(g, op, &what);
    }
}

/// The stack for this module's entries: orders.ts's marks, each `tuck` further along the wall where the floor allows.   (doortools.ts stackPosts)
pub fn door_tools_stack_posts(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> {
    let ignore: &[GuardId] = ignore.unwrap_or(&[]);
    seam_own_mark_first(g, d, side, men, &mut |g: &mut Game| door_tools_stack_tucked(g, d, side, men, ignore))
}

/// tuckedIn of stackTucked: the tuck does not take a man off his mark to put him on Sam, nor put a mark where a leaf swings.
fn tucked_in(g: &mut Game, own: &[V3], sam: V3, p: &crate::orders::StackPost, q: V3) -> bool {
    let mine = |x: V3| own.iter().any(|o| flat(*o, x) < crate::orders::OWN_EPS);
    (mine(p.pos) && flat(sam, q) < 0.7 && !mine(q))
        || (!p.lead && crate::doorway::doorway_in_way(g, q) && !crate::doorway::doorway_in_way(g, p.pos))
}

/// orders.ts's marks with the tuck applied: three or more tuck as files, the far man first.   (doortools.ts stackTucked)
pub fn door_tools_stack_tucked(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: &[GuardId]) -> Option<Vec<crate::orders::StackPost>> {
    let posts = crate::orders::squad_orders_stack_posts(g, d, side, men, Some(ignore))?;
    let sam = crate::orders::squad_orders_sam_stood(g);
    let own = crate::orders::squad_orders_own_marks(g, d, side, men);
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
            if !nav_blocked(g, q[0], q[2]) && !crate::scene_collision::solid_blocked(g, [p.pos[0], 0.45, p.pos[2]], [q[0], 0.45, q[2]], None)
                && !out.iter().enumerate().any(|(j, o)| j != i && flat(o.pos, q) < 0.7)
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
        let ok = !nav_blocked(g, q[0], q[2]) && !crate::scene_collision::solid_blocked(g, [p.pos[0], 0.45, p.pos[2]], [q[0], 0.45, q[2]], None)
            && !posts.iter().enumerate().any(|(j, o)| j != i && flat(o.pos, q) < 0.7)
            && !tucked_in(g, &own, sam, p, q) && crate::stackroom::stack_room_keeps(g, d, side, q);
        res.push(if ok { crate::orders::StackPost { pos: q, ..*p } } else { *p });
    }
    Some(res)
}

/// Teammates outside this order stacked on this door, this face, under a plain 'stack up'.   (doortools.ts holders)
pub fn door_tools_holders(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Vec<GuardId> {
    if !crate::orders::squad_orders_big(g) { return Vec::new(); }
    let fc = g.dr[d].frame_centre;
    let jamb_out = g.orders.knobs.jamb_out;
    let mut out: Vec<GuardId> = Vec::new();
    for gd in crate::orders::squad_orders_men(g, None, None) {
        if men.contains(&gd) { continue; }
        let a = match crate::orders::squad_orders_of(g, gd) { Some(a) => a, None => continue };
        let (is_stack, door, post) = { let x = &g.tab.orders_assignment[a]; (x.order == "stack", x.door, x.post) };
        if !is_stack || door != Some(d) { continue; }
        let p = match post { Some(p) => p, None => continue };
        let r = flat(p, fc);
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

/// Where each holder steps back to, clear of `taken`: along his own wall first, then the other, then the open floor. `must`: None if one cannot and stands in the blast.   (doortools.ts stepBacks)
pub fn door_tools_step_backs(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], taken: &[V3], must: Option<bool>) -> Option<JsMap<GuardId, crate::orders::StackPost>> {
    let must = must.unwrap_or(false);
    let width = g.dr[d].def.width;
    let fc = g.dr[d].frame_centre;
    let (off, jamb_past) = (g.orders.knobs.jamb_out, g.orders.knobs.jamb_past);
    let room_back = g.k.doortools.room_back.clone();
    let backblast_m = g.k.doortools.backblast_m;
    let mut out: JsMap<GuardId, crate::orders::StackPost> = JsMap::new();
    let mut walls: Vec<f64> = Vec::new();
    let away = { let a = door_pt(g, d, width / 2.0, 0.0, side, None); let b = door_pt(g, d, width / 2.0, 1.0, side, None); v3::yaw_to(a, b) };
    let out2 = crate::stackroom::stack_view_reach(g, d) > 2.6;
    for gd in door_tools_holders(g, d, side, men) {
        let a_id = crate::orders::squad_orders_of(g, gd).unwrap();
        let from = g.tab.orders_assignment[a_id].post.unwrap();
        let stack_side = g.tab.orders_assignment[a_id].stack_side;
        let hi = along_of(g, d, from) > width / 2.0;
        let mut found: Option<crate::orders::StackPost> = None;
        for dir in if hi { [1.0, -1.0] } else { [-1.0, 1.0] } {
            let mut prev: V3 = if (dir > 0.0) == hi { from } else { door_pt(g, d, if dir > 0.0 { width + jamb_past } else { -jamb_past }, off, side, None) };
            for x in room_back.iter().copied() {
                let q = door_pt(g, d, if dir > 0.0 { width + x } else { -x }, off, side, None);
                if nav_blocked(g, q[0], q[2]) || crate::scene_collision::solid_blocked(g, [prev[0], 0.45, prev[2]], [q[0], 0.45, q[2]], None) || !crate::stackroom::stack_room_keeps(g, d, side, q) { break; }
                prev = q;
                if out2 && flat(q, fc) < flat(from, fc) - 0.1 { continue; }
                if taken_and_out(taken, &out).iter().any(|o| flat(*o, q) < 0.85) || flat(q, crate::orders::squad_orders_sam_stood(g)) < 0.7 { continue; }
                found = Some(crate::orders::StackPost { pos: q, face: if walls.contains(&dir) { away } else { v3::yaw_to(fc, q) }, stack_side, lead: false, group: None });
                walls.push(dir);
                break;
            }
            if found.is_some() { break; }
        }
        if found.is_none() {
            let rear = crate::orders::squad_orders_rear_posts(g, d, side, 6.0, &taken_and_out(taken, &out));
            for r in rear {
                if crate::stackroom::stack_room_has(g, d, side, r.pos) && !(out2 && flat(r.pos, fc) < flat(from, fc) - 0.1) {
                    found = Some(crate::orders::StackPost { pos: r.pos, face: v3::yaw_to(fc, r.pos), stack_side, lead: false, group: None });
                    break;
                }
            }
        }
        match found {
            Some(p) => { out.set(gd, p); }
            None => { if must && flat(from, fc) < backblast_m + 0.4 { return None; } }
        }
    }
    Some(out)
}

/// The stack for an order on a door other teammates hold: its marks as if they had stepped back, and where they step back to.   (doortools.ts dealStack)
pub fn door_tools_deal_stack(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Option<crate::doortools::DoorToolsDealStack> {
    let hold = door_tools_holders(g, d, side, men);
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

/// They step back: still on their 'stack up' on this door, from further along the wall, as the door's rear security.   (doortools.ts makeRoom)
pub fn door_tools_make_room(g: &mut crate::game::Game, d: DoorId, back: &JsMap<GuardId, crate::orders::StackPost>) {
    let name = crate::orders::door_words(g, d);
    for (gd, post) in back.iter() {
        let a = match crate::orders::squad_orders_of(g, gd) { Some(a) => a, None => continue };
        let grp = g.tab.orders_assignment[a].group;
        let p = post.pos;
        let hit = crate::scene_collision::raycast(g, [p[0], 1.5, p[2]], [m::sin(post.face), 0.0, m::cos(post.face)], 8.0);
        let r = m::max(1.5, (match hit { Some(h) => h.t, None => 8.0 }) - 0.5);
        crate::orders::squad_orders_assign(g, gd, &crate::orders::SquadOrdersAssignA {
            order: String::from("stack"), post: Some(p), face: Some(post.face),
            watch: Some([p[0] + m::sin(post.face) * r, 1.1, p[2] + m::cos(post.face) * r]),
            door: Some(d), shy: Some(d), stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim), light: Some(true), stack_side: Some(post.stack_side),
            going: alloc::format!("making room · {}", name), there: alloc::format!("rear security · {}", name), ..Default::default()
        });
        let a2 = crate::orders::squad_orders_of(g, gd).unwrap();
        g.tab.orders_assignment[a2].group = grp;
        g.door_tools.stats.stepped_back += 1.0;
    }
    if !back.is_empty() {
        let names: Vec<String> = back.keys().iter().map(|m| callsign(g, *m)).collect();
        let what = alloc::format!("made room on {}: {}", g.dr[d].def.name, names.join(", "));
        door_tools_note(g, &what);
    }
}

/// inIt of fragBehind: beyond the door, in a frag's reach, with a line to where it lands.
fn frag_in_it(g: &mut Game, d: DoorId, side: f64, land: V3, p: V3) -> bool {
    off_of(g, d, p, side) < 0.0 && flat(p, land) < g.k.frag.wound_m + 1.0
        && (crate::frag::exposed_to(g, land, p, [p[0], p[1] + 1.6, p[2]], None) > 0.0 || !crate::scene_collision::solid_blocked(g, [land[0], 0.9, land[2]], [p[0], 1.1, p[2]], None))
}

/// Anybody of the team on the far side of the door with a line to where a frag through it would land: his callsign, or 'you'.   (doortools.ts fragBehind)
pub fn door_tools_frag_behind(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Option<String> {
    let deep = g.k.frag.order_in;
    let at = door_tools_land_for(g, d, side, deep);
    let land: V3 = [at[0], 0.3, at[2]];
    for gd in g.guards.clone() {
        if g.gd[gd].team == crate::teams::Team::Operative && g.gd[gd].state != crate::game::GuardState::Dead && !men.contains(&gd) {
            let p = pos_of(g, gd);
            if frag_in_it(g, d, side, land, p) { return Some(callsign(g, gd)); }
        }
    }
    let pl = g.player;
    let p = g.ch[g.pl[pl].ch].pos;
    if !g.pl[pl].down && frag_in_it(g, d, side, land, p) { Some(String::from("you")) } else { None }
}

/// A teammate outside this order on the far side of the door, in the open reach of a charge on it.   (doortools.ts behind)
pub fn door_tools_behind(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Option<GuardId> {
    if !crate::orders::squad_orders_big(g) { return None; }
    let width = g.dr[d].def.width;
    let inn = door_pt(g, d, width / 2.0, 0.45, -side, Some(1.1));
    let (fc, stun_m) = (g.dr[d].frame_centre, g.k.doortools.stun_m);
    for gd in crate::orders::squad_orders_men(g, None, None) {
        let p = pos_of(g, gd);
        if !men.contains(&gd) && off_of(g, d, p, side) < 0.0 && flat(p, fc) < stun_m && !crate::scene_collision::solid_blocked(g, inn, [p[0], 1.1, p[2]], None) { return Some(gd); }
    }
    None
}
