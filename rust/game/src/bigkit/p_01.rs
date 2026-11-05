//! src/game/bigkit.ts lines 1 to 338
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// the live tool of the seat's kick; only where nothing since the frame began can have taken the kick away
macro_rules! tool {
    ($g:ident, $p:ident) => {
        $g.pl[$p].kick.as_mut().unwrap().tool.as_mut().unwrap()
    };
}

/// a write to the tool after a call out: the TypeScript holds the object, and a kick that call took away is written to nobody
fn on_tool(g: &mut Game, pid: PlayerId, f: impl FnOnce(&mut crate::game::PlayerKickTool)) {
    if let Some(t) = g.pl[pid].kick.as_mut().and_then(|k| k.tool.as_mut()) {
        f(t);
    }
}

/// the row `WEAPONS.m870`; held only for the line that reads it
fn m870(g: &Game) -> &crate::weapons::WeaponDef {
    g.m.weapons.weapons.get(crate::weapons::WeaponId::M870).unwrap()
}

fn spots_tool(t: SamBigSpotTool) -> BigSpotsTool {
    match t {
        SamBigSpotTool::Shotgun => BigSpotsTool::Shotgun,
        SamBigSpotTool::Hammer => BigSpotsTool::Hammer,
    }
}
fn rebound_kind(k: crate::game::PlayerKickToolKind) -> crate::character::BigReboundKind {
    match k {
        crate::game::PlayerKickToolKind::Shotgun => crate::character::BigReboundKind::Shotgun,
        crate::game::PlayerKickToolKind::Hammer => crate::character::BigReboundKind::Hammer,
    }
}

/// `BIGKIT_BREAK=1`, the check's: never on in the game   (bigkit.ts bigBroken)
pub fn big_broken() -> bool {
    BREAK
}

/// The big slot's tool that opens this door from where you stand.   (bigkit.ts samBig)
pub fn sam_big(g: &mut crate::game::Game, d: DoorId, p: Option<V3>) -> Option<crate::bigkit::SamBig> {
    let pid = g.player;
    let kit = g.pl[pid].big_kit;
    if !g.k.bigslot.on
        || kit == crate::kit::BigKit::None
        || g.puppet.is_some()
        || g.touring
        || crate::doors::door_off(g, d)
        || g.dr[d].def.steel.unwrap_or(false)
        || !crate::doors::door_is_closed(g, d)
        || sam_big_tail(g)
    {
        return None;
    }
    if kit == crate::kit::BigKit::Shotgun && crate::doors::door_locked(g, d) && g.dr[d].lock_id.starts_with("wedge:") {
        return None;
    }
    match kit {
        crate::kit::BigKit::Shotgun => Some(crate::bigkit::SamBig::Shotgun),
        crate::kit::BigKit::Hammer => Some(crate::bigkit::SamBig::Hammer),
        crate::kit::BigKit::None => None,
    }
}

/// how near the straight line a→b a point stands   (bigkit.ts samBigSpot, its `off`)
fn off_line(p: V3, a: V3, b: V3) -> f64 {
    let ax = b[0] - a[0];
    let az = b[2] - a[2];
    let l2 = ax * ax + az * az;
    if l2 < 1e-4 {
        return v3::dist_xz(p, a);
    }
    let k = m::max(0.0, m::min(1.0, ((p[0] - a[0]) * ax + (p[2] - a[2]) * az) / l2));
    m::hyp2(p[0] - a[0] - ax * k, p[2] - a[2] - az * k)
}

/// a leg of his walk: nothing solid on it and no man by it   (bigkit.ts samBigSpot, its `leg`)
fn leg(g: &mut Game, men: &[V3], a: V3, b: V3) -> bool {
    !crate::scene_collision::solid_blocked(g, [a[0], 0.5, a[2]], [b[0], 0.5, b[2]], None) && !men.iter().any(|q| off_line(*q, a, b) < MOVE_LINE)
}

/// Where you work it from: the list's first place he can walk to that nobody stands on.   (bigkit.ts samBigSpot)
pub fn sam_big_spot(g: &mut crate::game::Game, d: DoorId, from: V3, tool: crate::bigkit::SamBigSpotTool) -> Option<crate::bigkit::SamBigSpot> {
    if crate::doors::door_off(g, d) {
        return None;
    }
    let side = big_side(g, d, &from);
    let men: Vec<V3> = g
        .guards
        .iter()
        .filter(|&&q| g.gd[q].state != crate::game::GuardState::Dead)
        .map(|&q| g.ch[g.gd[q].ch].pos)
        .collect();
    let n0 = crate::doortools::door_pt(g, d, 0.0, 0.0, side, None);
    let n1 = crate::doortools::door_pt(g, d, 0.0, 1.0, side, None);
    let nx = n1[0] - n0[0];
    let nz = n1[2] - n0[2];
    // (the places on HIS side of the opening before the ones across it)
    let width = g.dr[d].def.width;
    let mine = crate::doortools::along_of(g, d, from) < width / 2.0;
    let list = big_spots(g, d, side, spots_tool(tool));
    let mut order: Vec<BigSpot> = list.iter().copied().filter(|x| x.hinge == mine).collect();
    order.extend(list.iter().copied().filter(|x| x.hinge != mine));
    for s in order {
        if v3::dist_xz(s.pos, from) > g.k.bigslot.reach + MOVE_SLACK || men.iter().any(|q| v3::dist_xz(*q, s.pos) < MOVE_CLEAR) {
            continue;
        }
        if leg(g, &men, from, s.pos) {
            return Some(SamBigSpot { pos: s.pos, yaw: s.yaw, at: Some(s.at), via: None });
        }
        // a man of the file between him and it: round him on the side off the wall
        let mut near: Vec<V3> = men.iter().copied().filter(|q| off_line(*q, from, s.pos) < MOVE_LINE).collect();
        js::sort_by(&mut near, |a, b| v3::dist_xz(*a, from) - v3::dist_xz(*b, from));
        let Some(&man) = near.first() else { continue };
        if crate::scene_collision::solid_blocked(g, [from[0], 0.5, from[2]], [s.pos[0], 0.5, s.pos[2]], None) {
            continue;
        }
        let via: V3 = [man[0] + nx * MOVE_ROUND, 0.0, man[2] + nz * MOVE_ROUND];
        if !crate::scene_collision::nav_grid_is_blocked(g, via[0], via[2]) && leg(g, &men, from, via) && leg(g, &men, via, s.pos) {
            return Some(SamBigSpot { pos: s.pos, yaw: s.yaw, at: Some(s.at), via: Some(via) });
        }
    }
    None
}

/// a place with floor to stand on and a clear line to the leaf goes on the list   (bigkit.ts bigSpots, its `put`)
fn put(g: &mut Game, out: &mut Vec<BigSpot>, pos: V3, at: V3, hinge: bool) {
    if !crate::scene_collision::nav_grid_is_blocked(g, pos[0], pos[2]) && !crate::scene_collision::solid_blocked(g, [pos[0], 1.2, pos[2]], at, None) {
        out.push(BigSpot { pos, yaw: v3::yaw_to(pos, at), at, hinge });
    }
}

/// The places a man works a door's tool from, on face `side`, best first.   (bigkit.ts bigSpots)
pub fn big_spots(g: &mut crate::game::Game, d: DoorId, side: f64, tool: crate::bigkit::BigSpotsTool) -> Vec<crate::bigkit::BigSpot> {
    let w = g.dr[d].def.width;
    let lock = crate::shotgun::lock_point(g, d, side);
    let mut out: Vec<BigSpot> = Vec::new();
    let table = spots();
    let t = table.get(String::from(tool.as_str())).unwrap();
    for &[past, off] in &t.jamb {
        let pos = crate::doortools::door_pt(g, d, w + past, off, side, None);
        put(g, &mut out, pos, lock, false);
    }
    if tool == BigSpotsTool::Shotgun {
        for &[past, off] in &t.shoulder {
            let pos = crate::doortools::door_pt(g, d, w + past, off, side, None);
            put(g, &mut out, pos, lock, false);
        }
    }
    let mid = crate::doortools::door_pt(g, d, w / 2.0, 0.5, side, None);
    if !crate::doors::doors_pull_face(g, d, mid) {
        let at = if tool == BigSpotsTool::Hammer { crate::doortools::door_pt(g, d, HINGES_IN, PROUD, side, Some(lock[1])) } else { lock };
        for &[past, off] in &t.jamb {
            let pos = crate::doortools::door_pt(g, d, -past, off, side, None);
            put(g, &mut out, pos, at, true);
        }
    }
    if tool == BigSpotsTool::Hammer {
        for &[past, off] in &t.shoulder {
            let pos = crate::doortools::door_pt(g, d, w + past, off, side, None);
            put(g, &mut out, pos, lock, false);
        }
    }
    out
}

/// The row from where a stack stands: within reach of the doorway, a tool that opens it, a place he can walk to.   (bigkit.ts samBigFar)
pub fn sam_big_far(g: &mut crate::game::Game, d: DoorId, p: V3) -> Option<crate::bigkit::SamBigFar> {
    let pid = g.player;
    if !g.k.bigslot.on || g.pl[pid].big_kit == crate::kit::BigKit::None {
        return None;
    }
    let door = crate::doors::door_pos(g, d);
    if m::min(v3::dist_xz(door, p), v3::dist_xz(g.dr[d].frame_centre, p)) >= g.k.bigslot.reach {
        return None;
    }
    let tool = sam_big(g, d, Some(p))?;
    let (asked, answer) = match tool {
        SamBig::Shotgun => (SamBigSpotTool::Shotgun, SamBigFar::Shotgun),
        SamBig::Hammer => (SamBigSpotTool::Hammer, SamBigFar::Hammer),
    };
    if sam_big_spot(g, d, p, asked).is_some() { Some(answer) } else { None }
}

/// One frame of his walk onto the place; returns the yaw his body wants.   (bigkit.ts samBigWalk)
pub fn sam_big_walk(g: &mut crate::game::Game, dt: f64) -> f64 {
    let pid = g.player;
    let c = g.pl[pid].ch;
    let pos = g.ch[c].pos;
    let k = g.pl[pid].kick.as_mut().unwrap();
    let (spot, yaw) = (k.spot, k.yaw);
    let mv = k.tool.as_mut().unwrap().moves.as_mut().unwrap();
    if mv.arrived {
        return yaw;
    }
    if let Some(via) = mv.via {
        if v3::dist_xz(via, pos) < 0.15 {
            mv.via = None; // (round the man: the place itself from here)
        }
    }
    let via = mv.via;
    let to = via.unwrap_or(spot);
    let dx = to[0] - pos[0];
    let dz = to[2] - pos[2];
    let l = m::hyp2(dx, dz);
    if via.is_none() && l < 0.06 {
        mv.arrived = true;
        g.stats.bigkit.sam_walks += 1;
        return yaw;
    }
    let step = m::min(l, MOVE_WALK * dt);
    g.ch[c].pos = [pos[0] + dx / l * step, pos[1], pos[2] + dz / l * step];
    if via.is_some() || l > 0.45 { m::atan2(dx, dz) } else { yaw }
}

/// What the animator is told of the beat this frame.   (bigkit.ts samBigPose)
pub fn sam_big_pose(g: &mut crate::game::Game) -> Option<crate::bigkit::SamBigPose> {
    let pid = g.player;
    let t = g.pl[pid].kick.as_ref()?.tool.as_ref()?;
    let mv = t.moves.as_ref()?;
    let (tool_kind, t_now, blows, end, at) = (t.kind, t.t, mv.blows.clone(), mv.end, mv.at);
    let (kind, equip) = match tool_kind {
        crate::game::PlayerKickToolKind::Shotgun => (SamBigPoseKind::Shotgun, big_equip(g, BigEquipKind::Shotgun)),
        crate::game::PlayerKickToolKind::Hammer => (SamBigPoseKind::Hammer, big_equip(g, BigEquipKind::Hammer)),
    };
    let windup = big_wind(g);
    let stow = big_stow(g);
    Some(SamBigPose { kind, t: t_now, equip, windup, stow, blows, end, at, snug: true })
}

/// The tool changes hands with the weight of a gun swap: on, on the page.   (bigkit.ts weighed)
pub fn weighed(g: &mut crate::game::Game) -> bool {
    true
}

/// … is on (kitanim.ts asks).   (bigkit.ts bigWeighed)
pub fn big_weighed(g: &mut crate::game::Game) -> bool {
    weighed(g)
}

/// Seconds from the hand leaving for the tool to the tool in his hands.   (bigkit.ts bigEquip)
pub fn big_equip(g: &mut crate::game::Game, kind: crate::bigkit::BigEquipKind) -> f64 {
    match kind {
        BigEquipKind::Hammer => g.k.bigslot.equip_w,
        BigEquipKind::Shotgun => g.k.bigslot.draw_w,
    }
}

/// Seconds to put it back.   (bigkit.ts bigStow)
pub fn big_stow(g: &mut crate::game::Game) -> f64 {
    g.k.bigslot.stow_w
}

/// After which a hammer's wind-up may begin (a shell: the same as bigEquip).   (bigkit.ts bigOutBy)
pub fn big_out_by(g: &mut crate::game::Game, kind: crate::bigkit::BigOutByKind) -> f64 {
    match kind {
        BigOutByKind::Hammer => {
            let equip = big_equip(g, BigEquipKind::Hammer);
            equip - m::min(g.k.bigslot.flow, equip * 0.5)
        }
        BigOutByKind::Shotgun => big_equip(g, BigEquipKind::Shotgun) - 0.0,
    }
}

/// How long your own gun takes to be put away by hand before the hand goes for the tool.   (bigkit.ts samBigLeadOf)
pub fn sam_big_lead_of(g: &mut crate::game::Game) -> f64 {
    if g.touring {
        return 0.0;
    }
    let pid = g.player;
    let Some(Some(it)) = g.pl[pid].kit_was else { return 0.0 };
    if crate::kitanim::kit_in_hand(g).is_none() {
        return 0.0;
    }
    if it == crate::character::KitItem::Rifle && crate::kitanim::kit_sling_front(g) {
        g.k.kit.front.stow
    } else {
        *g.k.kit.stow.get(it).unwrap()
    }
}

/// That errand is the kit's to run this frame.   (bigkit.ts samBigCarry)
pub fn sam_big_carry(g: &mut crate::game::Game) -> bool {
    let pid = g.player;
    let c = g.pl[pid].ch;
    let Some(t) = g.pl[pid].kick.as_ref().and_then(|k| k.tool.as_ref()) else { return false };
    if t.moves.is_none() || !t.lead.map_or(false, |l| l > 0.0) {
        return false;
    }
    let now = t.t;
    now < 0.0 || (crate::character::anim_kit_busy(g, c) && now < CARRY_TAIL)
}

/// The hammer's wind-up (s).   (bigkit.ts bigWind)
pub fn big_wind(g: &mut crate::game::Game) -> f64 {
    g.k.bigslot.heft
}

/// From one blow to the next: never less than a follow-through and a wind-up.   (bigkit.ts bigSwing)
pub fn big_swing(g: &mut crate::game::Game) -> f64 {
    m::max(g.k.bigslot.swing, crate::character::big_rebound(crate::character::BigReboundKind::Hammer) + g.k.bigslot.heft)
}

/// The beat is over and his legs are his own, the tool still going back onto him.   (bigkit.ts samBigTail)
pub fn sam_big_tail(g: &mut crate::game::Game) -> bool {
    let pid = g.player;
    let c = g.pl[pid].ch;
    g.ch[c].anim.big_tool.is_some() && !g.pl[pid].kick.as_ref().map_or(false, |k| k.tool.is_some())
}

/// What is left of the tail (s).   (bigkit.ts samBigLeft)
pub fn sam_big_left(g: &mut crate::game::Game) -> f64 {
    let c = g.pl[g.player].ch;
    match &g.ch[c].anim.big_tool {
        Some(b) if b.end >= 0.0 => m::max(0.0, b.end + b.stow - b.t),
        _ => 0.0,
    }
}

/// The beat has ended: the tail begins, or a beat that never told its end gives him his hands at once.   (bigkit.ts samBigEnded)
pub fn sam_big_ended(g: &mut crate::game::Game) {
    let pid = g.player;
    let c = g.pl[pid].ch;
    let left = sam_big_left(g);
    if left > 0.0 {
        g.pl[pid].equip_t = m::max(g.pl[pid].equip_t, left);
    } else {
        g.ch[c].anim.big_tool = None;
    }
}

/// Dropped: his hands are his own, and nothing of the beat is left.   (bigkit.ts samBigDrop)
pub fn sam_big_drop(g: &mut crate::game::Game) {
    let c = g.pl[g.player].ch;
    g.ch[c].anim.big_tool = None;
}

/// One frame of the tail: over when the tool is back, or at once when anything else has his hands.   (bigkit.ts samBigTailTick)
pub fn sam_big_tail_tick(g: &mut crate::game::Game, dt: f64) {
    let pid = g.player;
    let c = g.pl[pid].ch;
    if g.ch[c].anim.big_tool.is_none() || g.pl[pid].kick.as_ref().map_or(false, |k| k.tool.is_some()) {
        return;
    }
    if let Some(b) = g.ch[c].anim.big_tool.as_mut() {
        b.t += dt;
    }
    let pl = &g.pl[pid];
    let taken = pl.down
        || pl.kick.is_some()
        || pl.holding.is_some()
        || pl.takedown.is_some()
        || pl.dragging.is_some()
        || pl.picking.is_some()
        || pl.wanding.is_some()
        || pl.coding.is_some()
        || pl.easing.is_some()
        || pl.door_move.is_some()
        || pl.wall.is_some()
        || pl.pending_throw.is_some()
        || pl.throw_held
        || crate::character::anim_in_throw(g, c)
        || crate::handwork::sam_work(g).is_some();
    let over = match &g.ch[c].anim.big_tool {
        Some(b) => b.end < 0.0 || b.t >= b.end + b.stow || taken,
        None => false,
    };
    if over {
        g.ch[c].anim.big_tool = None;
    }
}

/// The row performed: the beat begins, the kick's verb with the tool in the boot's place.   (bigkit.ts samBigStart)
pub fn sam_big_start(g: &mut crate::game::Game, d: DoorId) {
    let tool = match sam_big(g, d, None) {
        Some(SamBig::Shotgun) => crate::player::StartKickTool::Shotgun,
        Some(SamBig::Hammer) => crate::player::StartKickTool::Hammer,
        None => return,
    };
    crate::player::start_kick(g, d, Some(false), Some(tool));
}

/// the beat ends in `tail` s; a blow or shell that has just landed is followed through, a blow still to come is not swung   (bigkit.ts samBigTick, its `over`)
fn over(g: &mut Game, pid: PlayerId, tail: f64, landed: bool) {
    on_tool(g, pid, |t| {
        let now = t.t;
        t.done = now + tail;
        let kind = t.kind;
        if let Some(mv) = t.moves.as_mut() {
            if mv.end < 0.0 {
                mv.end = now + if landed { m::min(tail, crate::character::big_rebound(rebound_kind(kind))) } else { 0.0 };
                mv.blows.retain(|b| *b <= now);
            }
        }
    });
}

/// the next blow or shell is due at `next`, and the animator is told   (bigkit.ts samBigTick: `T.next = …; M?.blows.push(T.next)`)
fn next_at(g: &mut Game, pid: PlayerId, next: f64) {
    on_tool(g, pid, |t| {
        t.next = next;
        if let Some(mv) = t.moves.as_mut() {
            mv.blows.push(next);
        }
    });
}

/// One frame of the beat: true = over, he has his legs back.   (bigkit.ts samBigTick)
pub fn sam_big_tick(g: &mut crate::game::Game, dt: f64) -> bool {
    let pid = g.player;
    let c = g.pl[pid].ch;
    let d = g.pl[pid].kick.as_ref().unwrap().door;
    tool!(g, pid).t += dt;
    {
        let (now, lead) = (tool!(g, pid).t, tool!(g, pid).lead);
        // (the kit's errand is over before the wait it was given, or was never the kit's to run)
        if now < 0.0 && now > dt - lead.unwrap_or(0.0) && !crate::character::anim_kit_busy(g, c) {
            tool!(g, pid).t = 0.0;
        }
    }
    let now = tool!(g, pid).t;
    let done = tool!(g, pid).done;
    if done >= 0.0 {
        if now < done {
            return false;
        }
        // (broken from the face it opens toward: the same hands pull it open, once the tool is back on him)
        let gave = tool!(g, pid).gave.unwrap_or(false);
        let pos = g.ch[c].pos;
        let keys = g.pl[pid].keys.clone();
        let pull = gave && !crate::doors::door_off(g, d) && crate::doors::door_is_closed(g, d) && !crate::doors::doors_locked_out(g, d, pos, Some(&keys));
        if pull {
            let stow = big_stow(g);
            if let Some(end) = tool!(g, pid).moves.as_ref().map(|mv| mv.end) {
                if now < end + stow {
                    tool!(g, pid).done = end + stow;
                    return false;
                }
            }
            let id = g.ch[c].id;
            crate::doors::doors_use(g, d, pos, id, Some(&keys));
        }
        return true;
    }
    if crate::doors::door_off(g, d) || !crate::doors::door_is_closed(g, d) {
        if now < 0.0 {
            return true; // (his own gun still on its way to its place: nothing of the tool has begun)
        }
        over(g, pid, 0.15, false);
        return false;
    }
    let kind = tool!(g, pid).kind;
    let hammer = kind == crate::game::PlayerKickToolKind::Hammer;
    if tool!(g, pid).moves.is_some() && !tool!(g, pid).next.is_finite() {
        // (on his way to the place: the first blow is wound up, the first shell goes, once he stands on it with the tool in his hands)
        let arrived = tool!(g, pid).moves.as_ref().unwrap().arrived;
        if !arrived && now >= MOVE_CAP {
            let spot = g.pl[pid].kick.as_ref().unwrap().spot;
            if v3::dist_xz(g.ch[c].pos, spot) < 0.5 {
                tool!(g, pid).moves.as_mut().unwrap().arrived = true;
            } else {
                crate::game::game_msg(g, "no room to work it from here");
                over(g, pid, 0.0, false);
                return false;
            }
        }
        if !tool!(g, pid).moves.as_ref().unwrap().arrived {
            return false;
        }
        let out_by = big_out_by(g, if hammer { BigOutByKind::Hammer } else { BigOutByKind::Shotgun });
        let next = m::max(now, out_by) + if hammer { big_wind(g) } else { 0.0 };
        next_at(g, pid, next);
        return false;
    }
    if now < tool!(g, pid).next {
        return false;
    }
    tool!(g, pid).n += 1.0;
    let n = tool!(g, pid).n;
    if !hammer {
        g.stats.bigkit.sam_shells += 1;
        // (the shell's own rule takes a keep that is locked AND home; anything else shut is the latch's)
        let lock = crate::doors::door_locked(g, d) && crate::doors::door_latched(g, d);
        let side = tool!(g, pid).side;
        let at = crate::shotgun::lock_point(g, d, side);
        let opts = crate::combat::FireOpts { weapon: Some(m870(g).clone()), steady: Some(true), spread: Some(0.0), ..Default::default() };
        crate::combat::fire_weapon(g, c, at, true, Some(&opts));
        let gave = if lock {
            !crate::doors::door_locked(g, d)
        } else {
            let (pos, id) = (g.ch[c].pos, g.ch[c].id);
            crate::doors::doors_kick_in(g, d, pos, id, Some(true))
        };
        if gave && !lock {
            crate::game::game_msg(g, "you shot it open, and the whole floor heard that");
            crate::chronicle::chron_door(g, crate::chronicle::ChronDoorKind::DoorKicked, d);
        }
        if gave {
            g.stats.bigkit.sam_shot += 1;
            on_tool(g, pid, |t| t.gave = Some(true));
            over(g, pid, TAIL, true);
        } else if n >= g.k.bigslot.shells {
            crate::game::game_msg(g, if lock { "the lock held" } else { "it held" });
            over(g, pid, TAIL, true);
        } else {
            let next = now + m870(g).cd;
            next_at(g, pid, next);
        }
        return false;
    }
    g.stats.bigkit.sam_blows += 1;
    if n < tool!(g, pid).need {
        let id = g.ch[c].id;
        crate::doors::door_set_rattle_t(g, d, 0.6);
        crate::doors::door_set_last_who(g, d, id);
        let at = crate::doors::door_pos(g, d);
        crate::doors::doors_sound(g, d, crate::doors::DoorSound::Thud, at, id, 1.0);
        let next = now + big_swing(g);
        next_at(g, pid, next);
        return false;
    }
    let (pos, id) = (g.ch[c].pos, g.ch[c].id);
    let gave = crate::doors::doors_kick_in(g, d, pos, id, Some(true));
    let pos = g.ch[c].pos;
    crate::marks::kick_scuff(g, d, pos);
    if gave {
        g.stats.bigkit.sam_hammered += 1;
        on_tool(g, pid, |t| t.gave = Some(true));
        crate::game::game_msg(g, "you hammered it open, and the whole floor heard that");
        crate::chronicle::chron_door(g, crate::chronicle::ChronDoorKind::DoorKicked, d);
    }
    over(g, pid, TAIL, true);
    false
}

/// The blows the hammer needs on d from p.   (bigkit.ts hammerBlows)
pub fn hammer_blows(g: &mut crate::game::Game, d: DoorId, p: &[f64]) -> f64 {
    if crate::doors::door_locked(g, d) && crate::doors::doors_boot_holds(g, d, [p[0], p[1], p[2]]) {
        m::max(1.0, g.k.bigslot.swings)
    } else {
        1.0
    }
}

/// Which face of d he is on (the lock point's side).   (bigkit.ts bigSide)
pub fn big_side(g: &mut crate::game::Game, d: DoorId, p: &[f64]) -> f64 {
    if crate::doortools::off_of(g, d, [p[0], p[1], p[2]], 1.0) >= 0.0 { 1.0 } else { -1.0 }
}

/// The big slot's tool a man carries, for the drawer.   (bigkit.ts bigCarried)
pub fn big_carried(g: &mut crate::game::Game, c_2: crate::bigkit::BigCarriedC) -> Option<crate::bigkit::BigCarried> {
    if !g.k.bigslot.on {
        return None;
    }
    let kit = if let Some(pid) = crate::seats::man_by_id(g, c_2.id) {
        g.pl[pid].big_kit
    } else {
        let gd = g.guards.iter().copied().find(|&q| g.ch[g.gd[q].ch].id == c_2.id)?;
        crate::kit::kit_big(g, gd)
    };
    match kit {
        crate::kit::BigKit::Hammer => Some(BigCarried::Hammer),
        crate::kit::BigKit::Shotgun => Some(BigCarried::Shotgun),
        crate::kit::BigKit::None => None,
    }
}

/// He carries the hammer in his big slot.   (bigkit.ts carriesHammer)
pub fn carries_hammer(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].state != crate::game::GuardState::Dead && crate::kit::kit_big(g, gd) == crate::kit::BigKit::Hammer
}

/// The shotgun comes up off its sling into a teammate's hands; true once it has been up long enough for the shell to go.   (bigkit.ts gunUp)
pub fn gun_up(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if !g.gd[gd].slung.as_ref().map_or(false, |s| s.out) {
        let own = g.gd[gd].weapon;
        let def = g.fa[own].def;
        if g.m.weapons.weapons.get(def).unwrap().pellets.as_ref().map_or(false, |p| p.breach.is_some()) {
            return true;
        }
        let (route, c) = (g.gd[gd].route, g.gd[gd].ch);
        let own_id = g.tab.route[route].weapon;
        let ahead = g.ch[c].muzzle_ahead;
        if g.gd[gd].slung.is_none() {
            let row = m870(g).clone();
            let gun = crate::weapons::firearm_new(g, &row);
            g.gd[gd].slung = Some(crate::game::GuardSlung { own, own_id, ahead, gun, out: false, t: 0.0 });
        }
        let gun = {
            let s = g.gd[gd].slung.as_mut().unwrap();
            s.own = own;
            s.own_id = own_id;
            s.ahead = ahead;
            s.gun
        };
        g.tab.route[route].weapon = Some(crate::weapons::WeaponId::M870);
        g.gd[gd].weapon = gun;
        g.gd[gd].reload_t = -1.0;
        g.ch[c].muzzle_ahead = m870(g).muzzle_ahead;
        let now = g.time;
        let s = g.gd[gd].slung.as_mut().unwrap();
        s.out = true;
        s.t = now;
        g.stats.bigkit.drawn += 1;
    }
    let up_at = g.gd[gd].slung.as_ref().unwrap().t;
    g.time - up_at >= big_equip(g, BigEquipKind::Shotgun)
}

/// … and is slung again: his own gun back in his hands, as he put it down.   (bigkit.ts gunDown)
pub fn gun_down(g: &mut crate::game::Game, gd: GuardId) {
    let Some(s) = g.gd[gd].slung.as_mut() else { return };
    if !s.out {
        return;
    }
    s.out = false;
    let (own, own_id, ahead) = (s.own, s.own_id, s.ahead);
    g.stats.bigkit.slung += 1;
    if g.gd[gd].state == crate::game::GuardState::Dead {
        return;
    }
    let (route, c) = (g.gd[gd].route, g.gd[gd].ch);
    g.tab.route[route].weapon = own_id;
    g.gd[gd].weapon = own;
    g.gd[gd].reload_t = -1.0;
    g.ch[c].muzzle_ahead = ahead;
}

/// One blow of a teammate's hammer on d; true = the door gave.   (bigkit.ts hammerBlow)
pub fn hammer_blow(g: &mut crate::game::Game, gd: GuardId, d: DoorId, last: bool) -> bool {
    let c = g.gd[gd].ch;
    g.stats.bigkit.mate_blows += 1;
    let (pos, id) = (g.ch[c].pos, g.ch[c].id);
    if !last {
        crate::doors::door_set_rattle_t(g, d, 0.6);
        crate::doors::door_set_last_who(g, d, id);
        let at = crate::doors::door_pos(g, d);
        crate::doors::doors_sound(g, d, crate::doors::DoorSound::Thud, at, id, 1.0);
        return false;
    }
    let gave = crate::doors::doors_kick_in(g, d, pos, id, Some(true));
    let pos = g.ch[c].pos;
    crate::marks::kick_scuff(g, d, pos);
    if gave {
        g.stats.bigkit.mate_hammered += 1;
    }
    gave
}

/// The hammer off a teammate's back; every frame he is its door's man. Returns a COPY: a caller that writes the record does so at `g.gd[gd].big_out`.   (bigkit.ts hammerOut)
pub fn hammer_out(g: &mut crate::game::Game, gd: GuardId, at: V3) -> crate::game::GuardBigOut {
    if g.gd[gd].big_out.as_ref().map_or(true, |h| h.end >= 0.0) {
        let t0 = g.time;
        g.gd[gd].big_out = Some(crate::game::GuardBigOut { t0, blows: Vec::new(), end: -1.0, at });
        g.stats.bigkit.hammers_out += 1;
    }
    g.gd[gd].big_out.clone().unwrap()
}

/// … and put back: begun now or as the blow just landed is followed through; `hard`, on his back at once.   (bigkit.ts hammerIn)
pub fn hammer_in(g: &mut crate::game::Game, gd: GuardId, hard: Option<bool>) {
    let hard = hard.unwrap_or(false);
    let Some((t0, end)) = g.gd[gd].big_out.as_ref().map(|h| (h.t0, h.end)) else { return };
    if end < 0.0 {
        g.stats.bigkit.hammers_in += 1;
    }
    if hard {
        let c = g.gd[gd].ch;
        g.gd[gd].big_out = None;
        g.ch[c].anim.big_tool = None;
        return;
    }
    if end >= 0.0 {
        return;
    }
    let now = g.time - t0;
    let h = g.gd[gd].big_out.as_mut().unwrap();
    h.blows.retain(|b| *b <= now);
    h.end = match h.blows.last() {
        Some(&last) => m::max(now, last + crate::character::big_rebound(crate::character::BigReboundKind::Hammer)),
        None => now,
    };
}

/// A round has left this man's gun: the hammer is on his back.   (bigkit.ts bigFired)
pub fn big_fired(g: &mut crate::game::Game, c_2: CharId) {
    let gd = g.guards.iter().copied().find(|&x| g.gd[x].ch == c_2);
    match gd {
        Some(gd) if g.gd[gd].big_out.is_some() => hammer_in(g, gd, Some(true)),
        _ => g.ch[c_2].anim.big_tool = None,
    }
}

/// Every frame: the animator is told where each man's beat stands; a hammer whose door is no longer his goes back.   (bigkit.ts hammerOutTick)
pub fn hammer_out_tick(g: &mut crate::game::Game, mine: &mut dyn FnMut(&mut crate::game::Game, GuardId) -> bool) {
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        let Some(end) = g.gd[gd].big_out.as_ref().map(|h| h.end) else { continue };
        let c = g.gd[gd].ch;
        if g.gd[gd].state == crate::game::GuardState::Dead {
            g.gd[gd].big_out = None;
            g.ch[c].anim.big_tool = None;
            continue;
        }
        if end < 0.0 && !mine(g, gd) {
            hammer_in(g, gd, None);
        }
        let Some((t0, end)) = g.gd[gd].big_out.as_ref().map(|h| (h.t0, h.end)) else { continue };
        let t = g.time - t0;
        if end >= 0.0 && t >= end + big_stow(g) {
            g.gd[gd].big_out = None;
            g.ch[c].anim.big_tool = None;
            continue;
        }
        let equip = big_equip(g, BigEquipKind::Hammer);
        let windup = big_wind(g);
        let stow = big_stow(g);
        let (blows, at) = {
            let h = g.gd[gd].big_out.as_ref().unwrap();
            (h.blows.clone(), h.at)
        };
        g.ch[c].anim.big_tool = Some(crate::character::AnimBigTool {
            kind: crate::character::AnimBigToolKind::Hammer,
            t,
            equip,
            windup,
            stow,
            blows,
            end,
            at,
            snug: Some(true),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// the TypeScript's `off` (bigkit.ts:87), its doubles' bits under EXACT
    #[test]
    fn off_line_bits() {
        let cases: &[([f64; 3], [f64; 3], [f64; 3], u64)] = &[
            ([1.3, 0.0, 2.7], [0.1, 0.0, 0.2], [3.7, 0.0, 4.9], 0x3fe2294654bf8519),
            ([-2.25, 1.0, 0.125], [-5.5, 0.0, -1.75], [1.01, 0.0, 0.33], 0x3fe98047fe2c692b),
            ([10.0, 0.0, 10.0], [0.3, 0.0, 0.7], [0.3004, 0.0, 0.7003], 0x402ae0424bfb00f9),
            ([0.61, 0.0, -3.3], [0.0, 0.0, 0.0], [0.0, 0.0, -1.0], 0x4003094014f3944c),
            ([4.4, 0.0, 4.4], [1.0, 0.0, 1.0], [2.0, 0.0, 2.0], 0x400b27247aff1491),
            ([-7.77, 0.0, 3.14], [2.71, 0.0, -1.41], [-1.62, 0.0, 0.57], 0x401aa95b86f14118),
        ];
        for (p, a, b, bits) in cases {
            assert_eq!(off_line(*p, *a, *b).to_bits(), *bits);
        }
    }
}
