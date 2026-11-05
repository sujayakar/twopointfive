//! src/game/doortools.ts lines 970 to 1351
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ------------------------------------------------------------ what the skeleton does not give yet

/// (doortools.ts:1139 `shy: op.door, ...a`) the shy an order's assignment ends with: `a.shy`, or the door when `a` does not name it. Three callers say `shy: null`
/// (doortools.ts:2343, 2933, 3062), which an Option cannot tell from not naming it
fn seam_assign_shy(a: &crate::doortools::DoorToolsAssignA, door: DoorId) -> Option<DoorId> { todo!("needs: DoorToolsAssignA.shy as Option<Option<DoorId>>") }

// ------------------------------------------------------------ small readers of this file

fn pos_of(g: &mut Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }
fn is_dead(g: &mut Game, gd: GuardId) -> bool { g.gd[gd].state == crate::game::GuardState::Dead }
fn is_steel(g: &mut Game, d: DoorId) -> bool { g.dr[d].def.steel.unwrap_or(false) }
fn is_wedge(g: &mut Game, d: DoorId) -> bool { g.dr[d].lock_id.starts_with("wedge:") }

/// `g.orders.of(m) === op.mine.get(m)`: `of` says null and `get` undefined for a man with neither, and those are not equal
fn still_mine(g: &mut Game, op: DoorOpId, m: GuardId) -> bool {
    let now = crate::orders::squad_orders_of(g, m);
    now.is_some() && now == g.tab.doortools_door_op[op].mine.get(m).copied()
}

/// (doortools.ts:1050) the verbs of a door's lock rows, where the player's own prompts tell him it is locked
fn is_lock_verb(v: Option<crate::game::Verb>) -> bool {
    matches!(v, Some(crate::game::Verb::Pick) | Some(crate::game::Verb::Kick) | Some(crate::game::Verb::Rattle) | Some(crate::game::Verb::Nocode) | Some(crate::game::Verb::Code))
}

/// (doortools.ts:1013) the bit of `tried` for this face, known shut or found free
fn know_mask(side: f64, free: bool) -> i32 { (if side > 0.0 { 1 } else { 2 }) << (if free { 2 } else { 0 }) }

/// (doortools.ts:1014) `tried` after this face is learnt: `free` none forgets the face
fn learned(was: f64, side: f64, free: Option<bool>) -> f64 {
    let b: i32 = if side > 0.0 { 1 } else { 2 };
    let kept = js::to_i32(was) & !(b | b << 2);
    match free {
        None => kept as f64,
        Some(f) => (kept | b << (if f { 2 } else { 0 })) as f64,
    }
}

/// an assignment of a plain stack, the shape every mark of this file gives (doortools.ts:1246 and on)
fn stacked_a(post: V3, face: f64, watch: V3, stance: crate::character::Stance, stack_side: f64, name: &str) -> crate::doortools::DoorToolsAssignA {
    crate::doortools::DoorToolsAssignA {
        post: Some(post),
        face: Some(face),
        watch: Some(watch),
        stance: Some(stance),
        upper: Some(crate::character::UpperMode::None),
        move_: Some(crate::doortools::DoorToolsDoneItemLaneBackMove { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim }),
        stack_side: Some(stack_side),
        going: format!("stacking · {}", name),
        there: format!("stacked · {}", name),
        ..Default::default()
    }
}

/// the `kind` of an order given by code, as a kind of order
fn order_kind(kind: crate::doortools::DoorToolsOrderKind) -> crate::doortools::DoorToolKind {
    match kind { crate::doortools::DoorToolsOrderKind::Outside => crate::doortools::DoorToolKind::Outside, crate::doortools::DoorToolsOrderKind::BangClear => crate::doortools::DoorToolKind::BangClear, crate::doortools::DoorToolsOrderKind::OpenClear => crate::doortools::DoorToolKind::OpenClear, crate::doortools::DoorToolsOrderKind::Threshold => crate::doortools::DoorToolKind::Threshold }
}

/// a `Who` as `men()` takes it
fn who_sel(who: crate::orders::Who) -> crate::orders::SquadOrdersMenSel {
    match who { crate::orders::Who::Num(n) => crate::orders::SquadOrdersMenSel::Num(n), crate::orders::Who::All => crate::orders::SquadOrdersMenSel::All, crate::orders::Who::Red => crate::orders::SquadOrdersMenSel::Red, crate::orders::Who::Blue => crate::orders::SquadOrdersMenSel::Blue }
}

/// `t` as canThresholdThen takes it
fn can_t(t: &crate::orders::OrderTarget) -> crate::doortools::DoorToolsCanThresholdThenT {
    match t { crate::orders::OrderTarget::Floor(x) => crate::doortools::DoorToolsCanThresholdThenT::Floor(x.clone()), crate::orders::OrderTarget::Door(x) => crate::doortools::DoorToolsCanThresholdThenT::Door(x.clone()), crate::orders::OrderTarget::None(x) => crate::doortools::DoorToolsCanThresholdThenT::None(x.clone()), crate::orders::OrderTarget::Foe(x) => crate::doortools::DoorToolsCanThresholdThenT::Foe(x.clone()) }
}

/// `t` as canThresholdThen takes it, from thresholdThen's own
fn can_t_then(t: &crate::doortools::DoorToolsThresholdThenT) -> crate::doortools::DoorToolsCanThresholdThenT {
    match t { crate::doortools::DoorToolsThresholdThenT::Floor(x) => crate::doortools::DoorToolsCanThresholdThenT::Floor(x.clone()), crate::doortools::DoorToolsThresholdThenT::Door(x) => crate::doortools::DoorToolsCanThresholdThenT::Door(x.clone()), crate::doortools::DoorToolsThresholdThenT::None(x) => crate::doortools::DoorToolsCanThresholdThenT::None(x.clone()), crate::doortools::DoorToolsThresholdThenT::Foe(x) => crate::doortools::DoorToolsCanThresholdThenT::Foe(x.clone()) }
}

/// the door of a target, none off a door
fn door_of_t(t: &crate::orders::OrderTarget) -> Option<DoorId> { match t { crate::orders::OrderTarget::Door(x) => Some(x.door), _ => None } }

/// (doortools.ts:1139) `{ order: op.kind, door: op.door, shy, ...a }`
fn merged_assign(kind: &str, door: DoorId, shy: Option<DoorId>, a: &crate::doortools::DoorToolsAssignA) -> crate::orders::SquadOrdersAssignA {
    crate::orders::SquadOrdersAssignA {
        post: a.post,
        going: a.going.clone(),
        order: match &a.order { Some(o) => o.clone(), None => String::from(kind) },
        face: a.face,
        there: a.there.clone(),
        t0: a.t0,
        follow: a.follow,
        watch: a.watch,
        door: a.door.or(Some(door)),
        strafe: a.strafe,
        stance: a.stance,
        upper: a.upper,
        stack_side: a.stack_side,
        move_: a.move_,
        set_line: a.set_line.clone(),
        group: a.group,
        set_said: a.set_said,
        goal: a.goal,
        arrived: a.arrived,
        held_set: a.held_set,
        held_fire: a.held_fire,
        arr_t: a.arr_t,
        moved_t: a.moved_t,
        light: a.light,
        low: a.low,
        yield_t: a.yield_t,
        face_now: a.face_now,
        search: a.search,
        search_t: a.search_t,
        via: a.via.clone(),
        anchor: a.anchor,
        mark: a.mark,
        mark_t: a.mark_t,
        had: a.had,
        back_n: a.back_n,
        shy,
        thru: a.thru,
        via_for: a.via_for,
        via_t: a.via_t,
        pace: a.pace,
        spare: a.spare,
        from: a.from,
        foe: a.foe,
        crouch: a.crouch,
        lean: a.lean,
        fit: a.fit.clone(),
        tries: a.tries.clone(),
    }
}

/// which dealt post a man's spot from `pair` is: the first not yet given with that place (the TypeScript finds the post by its array's identity)
fn post_at(posts: &[crate::orders::StackPost], used: &mut [bool], p: V3) -> crate::orders::StackPost {
    let i = posts.iter().enumerate().position(|(i, x)| !used[i] && x.pos == p).unwrap();
    used[i] = true;
    posts[i]
}

// ------------------------------------------------------------ the frame, the boot

/// locked against the team on the face the door opens toward: shut in its frame, which a boot only drives it onto (doortools.ts DoorTools.frameHeld)
pub fn door_tools_frame_held(g: &mut crate::game::Game, d: DoorId, side: f64) -> bool {
    if is_steel(g, d) || is_open(g, d) || !door_tools_locked_out(g, d, side) { return false; }
    let at = door_pt(g, d, g.dr[d].def.width / 2.0, 0.6, side, None);
    crate::doors::doors_boot_holds(g, d, at)
}

/// what a man says to an entry order there when no pick opens it (doortools.ts DoorTools.noBoot)
pub fn door_tools_no_boot(g: &mut crate::game::Game, d: DoorId) -> String {
    let s = if door_tools_charge_on(g, d).is_some() { "opens toward us. no boot. blow the charge." } else if g.door_tools.charges > 0.0 { "opens toward us. no boot. charge it." } else { "opens toward us. no boot, no charge." };
    String::from(s)
}

/// open()'s answer at a door whose frame holds a boot; false: a silent squad with its picks switched off (doortools.ts DoorTools.frameAnswer)
pub fn door_tools_frame_answer(g: &mut crate::game::Game, op: DoorOpId) -> bool {
    let (d, lead) = (g.tab.doortools_door_op[op].door, g.tab.doortools_door_op[op].lead);
    if !door_tools_pickable(g, d) {
        g.door_tools.stats.no_boot += 1.0;
        let said = door_tools_no_boot(g, d);
        crate::game::game_say(g, lead, &said, None, None, None);
        door_tools_end(g, op, "opens toward us · no boot");
        return true;
    }
    if crate::silent::silent_no_boot(g) { return false; }
    door_tools_to_pick(g, op);
    door_tools_beat(g, op, "opens toward us: pick");
    crate::game::game_say(g, lead, "locked, and it opens toward us. picking it.", None, None, None);
    true
}

/// this order's door will take the boot, or where its frame holds a boot the pick: the lock a shared gate waits on (doortools.ts DoorTools.willKick)
pub fn door_tools_will_kick(g: &mut crate::game::Game, op: DoorOpId) -> bool {
    let (d, side) = (g.tab.doortools_door_op[op].door, g.tab.doortools_door_op[op].side);
    let men = g.tab.doortools_door_op[op].men.clone();
    if is_open(g, d) || is_steel(g, d) { return false; }
    if g.tab.doortools_door_op[op].big.unwrap_or(false) && !g.tab.doortools_door_op[op].no_gun.unwrap_or(false) && door_tools_big_opens(g, d, &men) { return true; }
    door_tools_locked_out(g, d, side) && (!door_tools_frame_held(g, d, side) || door_tools_pickable(g, d) || door_tools_big_opens(g, d, &men))
}

/// one of these men carries a big-slot tool that takes this door's lock from either face (doortools.ts DoorTools.bigOpens)
pub fn door_tools_big_opens(g: &mut crate::game::Game, d: DoorId, men: &[GuardId]) -> bool {
    if !g.k.bigslot.on || is_steel(g, d) { return false; }
    if crate::silent::squad_silent(g) && !crate::silent::urgent(g) { return false; }
    for &m in men {
        if crate::bigkit::carries_hammer(g, m) { return true; }
        if !is_dead(g, m) && crate::kit::kit_big(g, m) == crate::kit::BigKit::Shotgun && !is_wedge(g, d) { return true; }
    }
    false
}

/// the tool the big slot's row names for these men on that door: toGun's own order (doortools.ts DoorTools.bigWord)
pub fn door_tools_big_word(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::doortools::DoorToolsBigWord {
    let wedged = match t {
        crate::orders::OrderTarget::Door(x) => crate::doors::door_locked(g, x.door) && is_wedge(g, x.door) && door_tools_known_shut(g, x.door, x.side),
        _ => false,
    };
    if !wedged {
        for &m in men {
            if !is_dead(g, m) && crate::kit::kit_big(g, m) == crate::kit::BigKit::Shotgun { return crate::doortools::DoorToolsBigWord::Shotgun; }
        }
    }
    crate::doortools::DoorToolsBigWord::Hammer
}

/// where the lead boots a door from: beside the latch jamb where the floor allows and no pane stands behind him, else square to the leaf (doortools.ts DoorTools.bootSpot)
pub fn door_tools_boot_spot(g: &mut crate::game::Game, d: DoorId, side: f64) -> crate::doortools::DoorToolsBootSpot {
    let from = door_pt(g, d, g.dr[d].def.width - 0.25, 1.0, side, None);
    let sq = crate::doors::doors_kick_spot(g, d, from);
    let square = crate::doortools::DoorToolsBootSpot { pos: sq.pos, yaw: sq.yaw };
    let s = crate::doors::doors_kick_spot_side(g, d, sq.pos);
    let beside = crate::doortools::DoorToolsBootSpot { pos: s.pos, yaw: s.yaw };
    if crate::scene_collision::nav_grid_is_blocked(g, s.pos[0], s.pos[2]) || crate::scene_collision::solid_blocked(g, [sq.pos[0], 0.5, sq.pos[2]], [s.pos[0], 0.5, s.pos[2]], None) { return square; }
    if g.k.glasspen.stack_eyes <= 0.0 || crate::scene_collision::nav_grid_is_blocked(g, sq.pos[0], sq.pos[2]) { return beside; }
    let n = crate::orders::squad_orders_glass_eyes(g, d, side, s.pos, Some(false));
    if n >= g.k.glasspen.stack_eyes && crate::orders::squad_orders_glass_eyes(g, d, side, sq.pos, Some(false)) <= g.k.glasspen.stack_least_k * n { square } else { beside }
}

// ------------------------------------------------------------ what the team knows of a door

/// the team knows this face of the door: shut, or (`free`) tried and found free (doortools.ts DoorTools.knows)
pub fn door_tools_knows(g: &mut crate::game::Game, d: DoorId, side: f64, free: bool) -> bool {
    let was = g.door_tools.tried.get(d).copied().unwrap_or(0.0);
    (js::to_i32(was) & know_mask(side, free)) != 0
}

/// write what the team knows of this face; `free` none forgets it (doortools.ts DoorTools.learn)
pub fn door_tools_learn(g: &mut crate::game::Game, d: DoorId, side: f64, free: Option<bool>) {
    let was = g.door_tools.tried.get(d).copied().unwrap_or(0.0);
    g.door_tools.tried.set(d, learned(was, side, free));
}

/// why a hand does not open it, in a man's words (doortools.ts DoorTools.shutWhy)
pub fn door_tools_shut_why(g: &mut crate::game::Game, d: DoorId) -> String {
    let s = if is_steel(g, d) { "steel, and locked" } else if door_tools_pickable(g, d) { "locked" } else if is_wedge(g, d) { "wedged from inside" } else { "keypad lock, no code" };
    String::from(s)
}

/// does the team know this door is locked against it on that face; a door since opened, unlocked or broken is forgotten (doortools.ts DoorTools.knownShut)
pub fn door_tools_known_shut(g: &mut crate::game::Game, d: DoorId, side: f64) -> bool {
    if !door_tools_knows(g, d, side, false) { return false; }
    if is_open(g, d) || crate::doors::door_off(g, d) || !door_tools_locked_out(g, d, side) { door_tools_learn(g, d, side, None); return false; }
    true
}

/// nobody of the team has tried that face of it (doortools.ts DoorTools.untried)
pub fn door_tools_untried(g: &mut crate::game::Game, d: DoorId, side: f64) -> bool {
    !door_tools_known_shut(g, d, side)
}

/// the handle tried by a man at the jamb; true: locked against the team (doortools.ts DoorTools.test)
pub fn door_tools_test(g: &mut crate::game::Game, d: DoorId, side: f64, gd: GuardId, say: bool) -> bool {
    if is_open(g, d) || crate::doors::door_off(g, d) { return false; }
    let out = door_tools_locked_out(g, d, side);
    if door_tools_knows(g, d, side, !out) { return out; }
    door_tools_learn(g, d, side, Some(!out));
    if out {
        let who = g.ch[g.gd[gd].ch].id;
        let level = if crate::silent::squad_silent(g) { 0.06 } else { 0.12 };
        crate::doors::doors_rattle(g, d, who, Some(level));
    }
    g.door_tools.stats.tested += 1.0;
    let why = if out { door_tools_shut_why(g, d) } else { String::from("not locked") };
    let line = format!("{} tried {}: {}", g.gd[gd].callsign, g.dr[d].def.name, why);
    door_tools_note(g, &line);
    if say {
        let said = if out { format!("{}.", door_tools_shut_why(g, d)) } else { String::from("it's not locked.") };
        crate::game::game_say(g, gd, &said, None, None, None);
    }
    out
}

/// a man of the stack on that door stood, at the orders' last tick, on his mark with a man in his sight whom hold fire holds (doortools.ts DoorTools.stackHeld)
pub fn door_tools_stack_held(g: &mut crate::game::Game, d: DoorId) -> bool {
    let roster = g.door_tools.roster.clone();
    for m in roster {
        if is_dead(g, m) { continue; }
        let Some(b) = crate::orders::squad_orders_of(g, m) else { continue };
        let b = &g.tab.orders_assignment[b];
        if b.order == "stack" && b.door == Some(d) && b.held_fire.unwrap_or(false) && b.arrived && b.yield_t < 0.0 { return true; }
    }
    false
}

/// once a frame: a man come to the jamb on a plain 'stack up' tries the door; a door whose lock rows the player stands at is known too (doortools.ts DoorTools.testStacks)
pub fn door_tools_test_stacks(g: &mut crate::game::Game) {
    let mut i = 0;
    while i < g.door_tools.roster.len() {
        let gd = g.door_tools.roster[i];
        i += 1;
        if is_dead(g, gd) { continue; }
        let Some(a) = crate::orders::squad_orders_of(g, gd) else { continue };
        let at = pos_of(g, gd);
        let row = &g.tab.orders_assignment[a];
        if row.order != "stack" || !row.arrived || row.stance != crate::character::Stance::Stack { continue; }
        let (Some(door), Some(post)) = (row.door, row.post) else { continue };
        if v3::dist_xz(at, post) > 0.6 { continue; }
        if crate::holdfire::hold_fire_on_man(g, gd) || door_tools_stack_held(g, door) { continue; }
        if g.tab.orders_assignment[a].held_set.unwrap_or(false) { continue; }
        let side = crate::orders::side_of(g, door, at);
        door_tools_test(g, door, side, gd, true);
    }
    let mut j = 0;
    while j < g.nearby.len() {
        let (kind_door, door, reach, verb) = { let it = &g.nearby[j]; (matches!(it.kind, crate::game::InteractableKind::Door), it.door, it.in_reach, it.verb) };
        j += 1;
        let Some(d) = door else { continue };
        if !kind_door || !reach || is_open(g, d) || !is_lock_verb(verb) { continue; }
        let at = g.ch[g.pl[g.player].ch].pos;
        let side = crate::orders::side_of(g, d, at);
        door_tools_learn(g, d, side, Some(false));
    }
}

/// the order halts at a door that turns out shut: he says what he found, the order is over, its men stay stacked. true = halted (doortools.ts DoorTools.haltAt)
pub fn door_tools_halt_at(g: &mut crate::game::Game, op: DoorOpId) -> bool {
    let (d, side, lead) = { let o = &g.tab.doortools_door_op[op]; (o.door, o.side, o.lead) };
    if g.tab.doortools_door_op[op].coded.unwrap_or(false) || is_open(g, d) { return false; }
    let knew = door_tools_known_shut(g, d, side);
    let out = door_tools_test(g, d, side, lead, false);
    if !out || knew || door_tools_last_call(g, op) { return false; }
    if crate::silent::squad_silent(g) || door_tools_frame_held(g, d, side) { return false; }
    let why = door_tools_shut_said(g, lead, d);
    door_tools_beat(g, op, &format!("tried the door · {} · halted", why));
    door_tools_end(g, op, "found shut");
    let men = g.tab.doortools_door_op[op].men.clone();
    let mut left = Vec::new();
    for &m in &men {
        if !is_dead(g, m) && g.tab.doortools_door_op[op].post.has(m) && still_mine(g, op, m) { left.push(m); }
    }
    door_tools_plain_stack(g, op, &left, Some(&format!(" · {}", why)));
    crate::orders::squad_orders_show(g, &men);
    door_tools_halt_back(g, d, side, &men);
    true
}

/// what the man who tried it says of a door found shut, and the count of it (doortools.ts DoorTools.shutSaid)
pub fn door_tools_shut_said(g: &mut crate::game::Game, lead: GuardId, d: DoorId) -> String {
    let why = door_tools_shut_why(g, d);
    g.door_tools.stats.halted += 1.0;
    crate::game::game_say(g, lead, &format!("{}. holding on it — your call.", why), None, None, None);
    why
}

/// (doortools.ts:1082 `on`) a walker stands on his mark
fn walker_on(g: &mut Game, m: GuardId) -> bool {
    let b = crate::orders::squad_orders_of(g, m).unwrap();
    let at = pos_of(g, m);
    let row = &g.tab.orders_assignment[b];
    match row.post { Some(p) => row.arrived && v3::dist_xz(at, p) <= 0.6, None => false }
}

/// (doortools.ts:1085 `marks`) each walker's mark on the door he came to
fn walk_marks(g: &mut Game, men: &[GuardId]) -> JsMap<GuardId, crate::doortools::DoorToolsStackLeftPostValue> {
    let mut out = JsMap::new();
    for &m in men {
        let b = crate::orders::squad_orders_of(g, m).unwrap();
        let p = g.tab.orders_assignment[b].tries.as_ref().unwrap().post;
        out.set(m, crate::doortools::DoorToolsStackLeftPostValue { lead: p.lead, pos: p.pos, face: p.face, stack_side: p.stack_side });
    }
    out
}

/// a walk that came to a door nobody had tried: the man on the jamb tries the handle; locked, the halt of a door order; else the walk is asked again (doortools.ts DoorTools.walkTried)
pub fn door_tools_walk_tried(g: &mut crate::game::Game) {
    let mut seen: JsSet<OrdersAssignmentId> = JsSet::new();
    let mut i = 0;
    while i < g.door_tools.roster.len() {
        let gd = g.door_tools.roster[i];
        i += 1;
        let Some(a) = crate::orders::squad_orders_of(g, gd) else { continue };
        let Some((d, side)) = g.tab.orders_assignment[a].tries.as_ref().map(|t| (t.door, t.side)) else { continue };
        if seen.has(a) || is_dead(g, gd) { continue; }
        let roster = g.door_tools.roster.clone();
        let mut men = Vec::new();
        for m in roster {
            let Some(b) = crate::orders::squad_orders_of(g, m) else { continue };
            let same = match g.tab.orders_assignment[b].tries.as_ref() { Some(u) => u.door == d && u.side == side, None => false };
            if same && !is_dead(g, m) { men.push(m); }
        }
        for &m in &men {
            let b = crate::orders::squad_orders_of(g, m).unwrap();
            seen.add(b);
        }
        let mut lead = None;
        for &m in &men {
            let b = crate::orders::squad_orders_of(g, m).unwrap();
            if g.tab.orders_assignment[b].tries.as_ref().unwrap().post.lead { lead = Some(m); break; }
        }
        if lead.is_none() {
            for &m in &men {
                if walker_on(g, m) { lead = Some(m); break; }
            }
        }
        if is_open(g, d) || crate::doors::door_off(g, d) { crate::orders::squad_orders_walk_on(g, &men); continue; }
        let locked = door_tools_locked_out(g, d, side);
        if locked && door_tools_known_shut(g, d, side) {
            crate::orders::squad_orders_new_group(g);
            let marks = walk_marks(g, &men);
            let note = format!(" · {}", door_tools_shut_why(g, d));
            door_tools_stack_left(g, d, &marks, &men, Some(&note));
            door_tools_halt_back(g, d, side, &men);
            continue;
        }
        let Some(lead) = lead else { continue };
        if !walker_on(g, lead) { continue; }
        if !locked {
            door_tools_test(g, d, side, lead, true);
            crate::orders::squad_orders_walk_on(g, &men);
            crate::orders::squad_orders_show(g, &men);
            continue;
        }
        door_tools_test(g, d, side, lead, false);
        crate::orders::squad_orders_new_group(g);
        let why = door_tools_shut_said(g, lead, d);
        let name = g.dr[d].def.name.clone();
        door_tools_note(g, &format!("move {}: tried the door · {} · halted", name, why));
        door_tools_note(g, &format!("move {}: end: found shut", name));
        let marks = walk_marks(g, &men);
        door_tools_stack_left(g, d, &marks, &men, Some(&format!(" · {}", why)));
        crate::orders::squad_orders_show(g, &men);
        door_tools_halt_back(g, d, side, &men);
    }
}

/// on a door the other pair is charging: the men just left stacked on it are asked to step back, clear of that order's marks (doortools.ts DoorTools.haltBack)
pub fn door_tools_halt_back(g: &mut crate::game::Game, d: DoorId, side: f64, halted: &[GuardId]) {
    let ch = g.door_tools.ops.iter().copied().find(|&o| {
        let r = &g.tab.doortools_door_op[o];
        r.kind == crate::doortools::DoorToolKind::Charge && r.door == d && r.side == side
    });
    if ch.is_none() {
        let any = g.door_tools.set.iter().any(|&c| { let r = &g.tab.doortools_set_charge[c]; r.door == d && r.side == side && r.by != crate::ownblow::SAM_BY });
        if !any { return; }
    }
    let mut rest = crate::orders::squad_orders_men(g, None, None);
    rest.retain(|m| !halted.contains(m));
    let hold = door_tools_holders(g, d, side, &rest);
    let mut taken: Vec<V3> = match ch { Some(c) => g.tab.doortools_door_op[c].post.values().map(|p| p.pos).collect(), None => Vec::new() };
    for m in crate::orders::squad_orders_men(g, None, None) {
        let a = crate::orders::squad_orders_of(g, m);
        let on_ch = match ch { Some(c) => g.tab.doortools_door_op[c].men.contains(&m), None => false };
        if hold.contains(&m) || on_ch { continue; }
        let Some(a) = a else { continue };
        let row = &g.tab.orders_assignment[a];
        if row.door == Some(d) {
            if let Some(p) = row.post { taken.push(p); }
        }
    }
    if let Some(back) = door_tools_step_backs(g, d, side, &rest, &taken, None) { door_tools_make_room(g, d, &back); }
}

/// an order over with its men left a plain stack on the marks they came to the door by (doortools.ts DoorTools.plainStack)
pub fn door_tools_plain_stack(g: &mut crate::game::Game, op: DoorOpId, left: &[GuardId], note: Option<&str>) {
    let d = g.tab.doortools_door_op[op].door;
    let mut post = JsMap::new();
    for (m, p) in g.tab.doortools_door_op[op].post.iter() {
        post.set(m, crate::doortools::DoorToolsStackLeftPostValue { lead: p.lead, pos: p.pos, face: p.face, stack_side: p.stack_side });
    }
    door_tools_stack_left(g, d, &post, left, note);
}

/// the same for men with a mark each on that door, whoever dealt it (doortools.ts DoorTools.stackLeft)
pub fn door_tools_stack_left(g: &mut crate::game::Game, d: DoorId, post: &JsMap<GuardId, crate::doortools::DoorToolsStackLeftPostValue>, left: &[GuardId], note: Option<&str>) {
    let note = note.unwrap_or("");
    let name = crate::orders::door_words(g, d);
    let c = g.dr[d].frame_centre;
    for &m in left {
        let p = post.get(m).unwrap();
        let a = crate::orders::SquadOrdersAssignA {
            order: String::from("stack"),
            post: Some(p.pos),
            face: Some(p.face),
            watch: Some([c[0], 1.05, c[2]]),
            door: Some(d),
            shy: Some(d),
            stance: Some(if p.lead { crate::character::Stance::Stack } else { crate::character::Stance::HighReady }),
            upper: Some(crate::character::UpperMode::None),
            move_: Some(crate::doortools::DoorToolsDoneItemLaneBackMove { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim }),
            stack_side: Some(p.stack_side),
            going: format!("stacking · {}", name),
            there: format!("stacked · {}{}", name, note),
            ..Default::default()
        };
        crate::orders::squad_orders_assign(g, m, &a);
    }
    crate::orders::squad_orders_spare_rear(g, left, d, None, None);
}

/// how an entry row on that door begins, by what the team knows of it (doortools.ts DoorTools.entryVerb)
pub fn door_tools_entry_verb(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) -> crate::doortools::DoorToolsEntryVerb {
    let crate::orders::OrderTarget::Door(x) = t else { return crate::doortools::DoorToolsEntryVerb::Open };
    if !door_tools_known_shut(g, x.door, x.side) || is_steel(g, x.door) { return crate::doortools::DoorToolsEntryVerb::Open; }
    let pick = door_tools_pickable(g, x.door);
    if pick && crate::silent::silent_pick(g) { return crate::doortools::DoorToolsEntryVerb::Pick; }
    if door_tools_frame_held(g, x.door, x.side) { if pick { crate::doortools::DoorToolsEntryVerb::Pick } else { crate::doortools::DoorToolsEntryVerb::Open } } else { crate::doortools::DoorToolsEntryVerb::Kick }
}

/// men these rows go to are holding that door from outside (doortools.ts DoorTools.heldPlain)
pub fn door_tools_held_plain(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> bool {
    if door_of_t(t).is_none() { return false; }
    let mut up = Vec::new();
    for &m in men {
        if !is_dead(g, m) { up.push(m); }
    }
    if up.is_empty() { return false; }
    !door_tools_held_on(g, t, &up).is_empty()
}

/// a line of the order log (doortools.ts DoorTools.note)
pub fn door_tools_note(g: &mut crate::game::Game, what: &str) {
    let t = g.time;
    g.door_tools.log.push(crate::doortools::DoorToolsLogItem { t, what: String::from(what) });
    if g.door_tools.log.len() > 300 { g.door_tools.log.drain(0..100); }
}

/// a beat of an order, and its line in the log (doortools.ts DoorTools.beat)
pub fn door_tools_beat(g: &mut crate::game::Game, op: DoorOpId, what: &str) {
    let t = g.time;
    g.tab.doortools_door_op[op].beats.push(crate::doortools::DoorToolsDoneItemBeatsItem { t, what: String::from(what) });
    let d = g.tab.doortools_door_op[op].door;
    let line = format!("{} {}: {}", g.tab.doortools_door_op[op].kind.as_str(), g.dr[d].def.name, what);
    door_tools_note(g, &line);
}

// ------------------------------------------------------------ orders in

/// hand a man this op's next assignment, and remember it as ours (doortools.ts DoorTools.assign)
pub fn door_tools_assign(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, a: &crate::doortools::DoorToolsAssignA) {
    let (kind, door) = (g.tab.doortools_door_op[op].kind, g.tab.doortools_door_op[op].door);
    let merged = merged_assign(kind.as_str(), door, seam_assign_shy(a, door), a);
    crate::orders::squad_orders_assign(g, gd, &merged);
    let now = crate::orders::squad_orders_of(g, gd).unwrap();
    g.tab.doortools_door_op[op].mine.set(gd, now);
}

/// a new order: the men are this order's now, and an order left with none is over (doortools.ts DoorTools.newOp)
pub fn door_tools_new_op(g: &mut crate::game::Game, kind: crate::doortools::DoorToolKind, d: DoorId, side: f64, men: &[GuardId], phase: crate::doortools::Phase) -> DoorOpId {
    for o in g.door_tools.ops.clone() {
        g.tab.doortools_door_op[o].men.retain(|m| !men.contains(m));
        if g.tab.doortools_door_op[o].men.is_empty() { door_tools_end(g, o, "re-ordered"); }
    }
    let mut from = JsMap::new();
    for &m in men {
        let p = pos_of(g, m);
        from.set(m, [p[0], 0.0, p[2]]);
    }
    let row = crate::doortools::DoorOp {
        kind,
        door: d,
        side,
        men: men.to_vec(),
        lead: men[0],
        rear: JsSet::new(),
        from,
        phase,
        t0: g.time,
        bang_held: false,
        mine: JsMap::new(),
        post: JsMap::new(),
        kick: None,
        open_t: f64::NAN,
        can: None,
        toss_t: f64::NAN,
        land_t: f64::NAN,
        bang_t: f64::NAN,
        bang_at: None,
        queue: Vec::new(),
        set_t: f64::NAN,
        pod: JsMap::new(),
        on_pod: JsMap::new(),
        slice: JsMap::new(),
        crossed: JsMap::new(),
        corner: JsMap::new(),
        search: None,
        beats: Vec::new(),
        on_word: false,
        word: None,
        hold: 0.0,
        ready_t: f64::NAN,
        gate: None,
        coded: None, fled: None, fled_w: None, fled_t: None, fled_said: None, fled_go: None, lane: None, frag: None, frag_way_t: None, leaf_a: None, leaf_t: None,
        no_gun: None, big: None, stow_off_t: None, first: None, threw: None, aim: None, pass: None, thr: None, thr_hold: None, thr_pre: None, worded: None,
        last_stack: None, last_hold: None, last_tossed: None, other: None, threat: None,
    };
    let op = g.tab.doortools_door_op.push(row);
    g.door_tools.ops.push(op);
    g.door_tools.stats.orders += 1.0;
    op
}

/// the menu's release: a reason when it cannot be done (doortools.ts DoorTools.issue)
pub fn door_tools_issue(g: &mut crate::game::Game, kind: crate::doortools::DoorToolKind, men: &[GuardId], t: &crate::orders::OrderTarget, on_word: Option<bool>) -> crate::doortools::DoorToolsIssue {
    match door_tools_start(g, kind, men, t, on_word, None) {
        crate::doortools::DoorToolsStart::Text(s) => crate::doortools::DoorToolsIssue::Text(s),
        crate::doortools::DoorToolsStart::DoorOp(_) => crate::doortools::DoorToolsIssue::Arm1(()),
    }
}

/// a fresh gate, for each door that is to go on the one signal; `go` false: they hold until somebody raises it (doortools.ts DoorTools.together)
pub fn door_tools_together(g: &mut crate::game::Game, go: Option<bool>) -> DoorGateId {
    let go = go.unwrap_or(true);
    g.tab.doortools_door_gate.push(crate::doortools::DoorGate { ops: Vec::new(), go, open_t: f64::NAN })
}

/// one of the entry orders given by code: to exactly these men, on that face of that door (doortools.ts DoorTools.order)
pub fn door_tools_order(g: &mut crate::game::Game, kind: crate::doortools::DoorToolsOrderKind, men: &[GuardId], door: DoorId, side: f64, gate: Option<DoorGateId>, hold: Option<bool>) -> crate::doortools::DoorToolsOrder {
    let hold = hold.unwrap_or(false);
    let mut up = Vec::new();
    for &gd in men {
        if !is_dead(g, gd) { up.push(gd); }
    }
    if up.is_empty() { return crate::doortools::DoorToolsOrder::Text(String::from("nobody to take it")); }
    let c = g.dr[door].frame_centre;
    let t = crate::orders::OrderTarget::Door(crate::orders::OrderTargetDoor { at: [c[0], 0.0, c[2]], door, side });
    match door_tools_stackable(g, &t, &up) {
        crate::doortools::DoorToolsStackable::Bool(true) => {}
        crate::doortools::DoorToolsStackable::Bool(false) => return crate::doortools::DoorToolsOrder::Text(String::from("no door")),
        crate::doortools::DoorToolsStackable::Text(s) => return crate::doortools::DoorToolsOrder::Text(if s.is_empty() { String::from("no door") } else { s }),
    }
    crate::silent::silent_tool(g, kind == crate::doortools::DoorToolsOrderKind::BangClear);
    let op = match door_tools_start(g, order_kind(kind), &up, &t, Some(false), Some(hold)) {
        crate::doortools::DoorToolsStart::Text(s) => return crate::doortools::DoorToolsOrder::Text(s),
        crate::doortools::DoorToolsStart::DoorOp(op) => op,
    };
    g.tab.doortools_door_op[op].coded = Some(true);
    if let Some(gt) = gate {
        g.tab.doortools_door_op[op].gate = Some(gt);
        g.tab.doortools_door_gate[gt].ops.push(op);
    }
    crate::doortools::DoorToolsOrder::DoorOp(op)
}

/// work the room from the threshold, as an order's `issue` (doortools.ts DoorTools.threshold)
pub fn door_tools_threshold(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget, on_word: Option<bool>, hold: Option<bool>) -> crate::doortools::DoorToolsThreshold {
    match door_tools_start(g, crate::doortools::DoorToolKind::Threshold, men, t, on_word, hold) {
        crate::doortools::DoorToolsStart::Text(s) => crate::doortools::DoorToolsThreshold::Text(s),
        crate::doortools::DoorToolsStart::DoorOp(_) => crate::doortools::DoorToolsThreshold::Arm1(()),
    }
}

/// work it and hold the jambs until told what next (doortools.ts DoorTools.thresholdHold)
pub fn door_tools_threshold_hold(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget, on_word: Option<bool>) -> crate::doortools::DoorToolsThresholdHold {
    match door_tools_threshold(g, men, t, on_word, Some(true)) {
        crate::doortools::DoorToolsThreshold::Text(s) => crate::doortools::DoorToolsThresholdHold::Text(s),
        crate::doortools::DoorToolsThreshold::Arm1(()) => crate::doortools::DoorToolsThresholdHold::Arm1(()),
    }
}

/// the same order's `valid` (doortools.ts DoorTools.canThreshold)
pub fn door_tools_can_threshold(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::doortools::DoorToolsCanThreshold {
    match door_tools_stackable(g, t, men) {
        crate::doortools::DoorToolsStackable::Text(s) => crate::doortools::DoorToolsCanThreshold::Text(s),
        crate::doortools::DoorToolsStackable::Bool(b) => crate::doortools::DoorToolsCanThreshold::Bool(b),
    }
}

/// the threshold by a door's name: the men `who` names, on the face they can walk to unless `side` says (doortools.ts DoorTools.thresholdOn)
pub fn door_tools_threshold_on(g: &mut crate::game::Game, door: &str, who: Option<crate::orders::Who>, side: Option<f64>, hold: Option<bool>) -> crate::doortools::DoorToolsThresholdOn {
    let who = who.unwrap_or(crate::orders::Who::All);
    let hold = hold.unwrap_or(false);
    door_tools_sync(g);
    let Some(d) = crate::doors::doors_by_name(g, door) else { return crate::doortools::DoorToolsThresholdOn::Text(format!("no door called {}", door)) };
    let men = crate::orders::squad_orders_men(g, Some(who_sel(who)), None);
    if men.is_empty() { return crate::doortools::DoorToolsThresholdOn::Text(String::from("nobody to take it")); }
    let at = g.ch[g.pl[g.player].ch].pos;
    let sam = crate::orders::side_of(g, d, at);
    let face = match side {
        Some(s) => s,
        None => if crate::orders::squad_orders_big(g) { crate::orders::squad_orders_side_for(g, d, &men, sam) } else { sam },
    };
    match door_tools_order(g, crate::doortools::DoorToolsOrderKind::Threshold, &men, d, face, None, Some(hold)) {
        crate::doortools::DoorToolsOrder::Text(s) => crate::doortools::DoorToolsThresholdOn::Text(s),
        crate::doortools::DoorToolsOrder::DoorOp(op) => crate::doortools::DoorToolsThresholdOn::DoorOp(op),
    }
}

/// the rooms being worked from the threshold whose men are still outside on the jambs, among these men (doortools.ts DoorTools.thresholdHeld)
pub fn door_tools_threshold_held(g: &mut crate::game::Game, men: &[GuardId], door: Option<DoorId>) -> Vec<DoorOpId> {
    let mut out = Vec::new();
    for &o in &g.door_tools.ops {
        let op = &g.tab.doortools_door_op[o];
        if op.thr.is_some() && op.phase == crate::doortools::Phase::Thr && door.map_or(true, |d| op.door == d) && op.men.iter().any(|m| men.contains(m)) { out.push(o); }
    }
    out
}

/// on the door a menu is up on (none off a door) (doortools.ts DoorTools.heldOn)
pub fn door_tools_held_on(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> Vec<DoorOpId> {
    match door_of_t(t) {
        Some(d) => door_tools_threshold_held(g, men, Some(d)),
        None => Vec::new(),
    }
}

/// the men among these who hold it: who the held page's rows go to (doortools.ts DoorTools.heldMen)
pub fn door_tools_held_men(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> Vec<GuardId> {
    let ops = door_tools_held_on(g, t, men);
    men.iter().copied().filter(|m| ops.iter().any(|&o| g.tab.doortools_door_op[o].men.contains(m))).collect()
}

/// what next, to men working a room from the threshold: step in, bang then step in, or off the door. A reason when there is nothing to tell (doortools.ts DoorTools.thresholdThen)
pub fn door_tools_threshold_then(g: &mut crate::game::Game, what: crate::doortools::DoorToolsThresholdThenWhat, men: &[GuardId], t: Option<&crate::doortools::DoorToolsThresholdThenT>) -> crate::doortools::DoorToolsThresholdThen {
    door_tools_sync(g);
    let ask = match what { crate::doortools::DoorToolsThresholdThenWhat::Bang => crate::doortools::DoorToolsCanThresholdThenWhat::Bang, crate::doortools::DoorToolsThresholdThenWhat::Step => crate::doortools::DoorToolsCanThresholdThenWhat::Step, crate::doortools::DoorToolsThresholdThenWhat::Leave => crate::doortools::DoorToolsCanThresholdThenWhat::Leave };
    let can = t.map(can_t_then);
    if let crate::doortools::DoorToolsCanThresholdThen::Text(s) = door_tools_can_threshold_then(g, ask, can.as_ref(), men) { return crate::doortools::DoorToolsThresholdThen::Text(s); }
    let door = match t { Some(crate::doortools::DoorToolsThresholdThenT::Door(x)) => Some(x.door), _ => None };
    let ops = door_tools_threshold_held(g, men, door);
    let w = if what != crate::doortools::DoorToolsThresholdThenWhat::Leave && ops.len() > 1 {
        let now = g.time;
        let id = g.tab.doortools_word.push(crate::doortools::Word { t0: now, go1: now, ..Default::default() });
        g.door_tools.words.push(id);
        Some(id)
    } else { None };
    for op in ops {
        let told = match what { crate::doortools::DoorToolsThresholdThenWhat::Step => "step in", crate::doortools::DoorToolsThresholdThenWhat::Bang => "bang and step in", crate::doortools::DoorToolsThresholdThenWhat::Leave => "leave it" };
        door_tools_beat(g, op, &format!("told · {}", told));
        if what != crate::doortools::DoorToolsThresholdThenWhat::Leave {
            g.tab.doortools_door_op[op].word = w;
            g.tab.doortools_door_op[op].hold = 0.0;
        }
        let thr = g.tab.doortools_door_op[op].thr.as_mut().unwrap();
        let owed = thr.step_owed.unwrap_or(false);
        thr.step_owed = Some(false);
        thr.cut_owed = Some(false);
        match what {
            crate::doortools::DoorToolsThresholdThenWhat::Bang => {
                thr.force = true;
                if thr.first.is_some() || thr.wide.unwrap_or(false) { thr.tossed = 0.0; }
                door_tools_to_pockets(g, op);
            }
            crate::doortools::DoorToolsThresholdThenWhat::Step => {
                if owed { door_tools_word_step_go(g, op); } else { door_tools_held_beat(g, op); door_tools_to_step(g, op); }
            }
            crate::doortools::DoorToolsThresholdThenWhat::Leave => {
                door_tools_held_beat(g, op);
                let lead = g.tab.doortools_door_op[op].lead;
                crate::game::game_say(g, lead, "coming off the door.", None, None, None);
                door_tools_end(g, op, "stood down");
                let them = g.tab.doortools_door_op[op].men.clone();
                let mut left = Vec::new();
                for m in them {
                    if !is_dead(g, m) && g.tab.doortools_door_op[op].post.has(m) && still_mine(g, op, m) { left.push(m); }
                }
                door_tools_plain_stack(g, op, &left, None);
            }
        }
    }
    crate::doortools::DoorToolsThresholdThen::Arm1(())
}

/// given with 'wait for my word' on: the jambs stay held and the word is then what the row said (doortools.ts DoorTools.thenOnWord)
pub fn door_tools_then_on_word(g: &mut crate::game::Game, what: crate::doortools::DoorToolsThenOnWordWhat, men: &[GuardId], t: &crate::orders::OrderTarget) -> crate::doortools::DoorToolsThenOnWord {
    let ask = match what { crate::doortools::DoorToolsThenOnWordWhat::Bang => crate::doortools::DoorToolsCanThresholdThenWhat::Bang, crate::doortools::DoorToolsThenOnWordWhat::Step => crate::doortools::DoorToolsCanThresholdThenWhat::Step };
    let can = can_t(t);
    if let crate::doortools::DoorToolsCanThresholdThen::Text(s) = door_tools_can_threshold_then(g, ask, Some(&can), men) { return crate::doortools::DoorToolsThenOnWord::Text(s); }
    for op in door_tools_threshold_held(g, men, door_of_t(t)) {
        let thr = g.tab.doortools_door_op[op].thr.as_mut().unwrap();
        thr.word_step = Some(what == crate::doortools::DoorToolsThenOnWordWhat::Step);
        if thr.step_owed.unwrap_or(false) {
            thr.step_owed = Some(false);
            thr.cut_owed = Some(false);
            thr.frag_t = None;
            thr.frag_said = Some(false);
        }
        let line = if what == crate::doortools::DoorToolsThenOnWordWhat::Step { "step in, no bang" } else { "bang, then step in" };
        door_tools_beat(g, op, &format!("on the word · {}", line));
    }
    crate::doortools::DoorToolsThenOnWord::Arm1(())
}

/// thresholdThen's `valid`: no cans, or one of ours behind the door: no bang (doortools.ts DoorTools.canThresholdThen)
pub fn door_tools_can_threshold_then(g: &mut crate::game::Game, what: crate::doortools::DoorToolsCanThresholdThenWhat, t: Option<&crate::doortools::DoorToolsCanThresholdThenT>, men: &[GuardId]) -> crate::doortools::DoorToolsCanThresholdThen {
    let door = match t { Some(crate::doortools::DoorToolsCanThresholdThenT::Door(x)) => Some(x.door), _ => None };
    let ops = door_tools_threshold_held(g, men, door);
    if ops.is_empty() {
        let s: &str = crate::words::wg(g, &"nobody's holding that door.", &"nobody is holding that door.");
        return crate::doortools::DoorToolsCanThresholdThen::Text(String::from(s));
    }
    if what != crate::doortools::DoorToolsCanThresholdThenWhat::Bang { return crate::doortools::DoorToolsCanThresholdThen::Bool(true); }
    if g.door_tools.bangs <= 0.0 { return crate::doortools::DoorToolsCanThresholdThen::Text(String::from("no bangs left.")); }
    for op in ops {
        let (d, side, them) = { let o = &g.tab.doortools_door_op[op]; (o.door, o.side, o.men.clone()) };
        let Some(who) = door_tools_can_behind(g, d, side, &them).filter(|s| !s.is_empty()) else { continue };
        if who == "you" {
            let s: &str = crate::words::wg(g, &"you're behind that door.", &"you are behind that door.");
            return crate::doortools::DoorToolsCanThresholdThen::Text(String::from(s));
        }
        return crate::doortools::DoorToolsCanThresholdThen::Text(format!("{} is behind that door.", who));
    }
    crate::doortools::DoorToolsCanThresholdThen::Bool(true)
}

/// the step in, from the jambs: at once, or on a word that has other doors to wait for, with them (doortools.ts DoorTools.toStep)
pub fn door_tools_to_step(g: &mut crate::game::Game, op: DoorOpId) {
    if g.tab.doortools_door_op[op].word.is_some() {
        g.tab.doortools_door_op[op].bang_t = -1e9;
        g.tab.doortools_door_op[op].phase = crate::doortools::Phase::Wait;
    } else {
        door_tools_enter(g, op);
    }
}

/// from the console or a probe: every held threshold of the men `who` names, or only the door named. None when it went (doortools.ts DoorTools.thresholdTell)
pub fn door_tools_threshold_tell(g: &mut crate::game::Game, what: crate::doortools::DoorToolsThresholdTellWhat, door: Option<&str>, who: Option<crate::orders::Who>) -> Option<String> {
    let who = who.unwrap_or(crate::orders::Who::All);
    let door = door.filter(|s| !s.is_empty());
    let d = match door { Some(n) => crate::doors::doors_by_name(g, n), None => None };
    if let (Some(n), None) = (door, d) { return Some(format!("no door called {}", n)); }
    let men = crate::orders::squad_orders_men(g, Some(who_sel(who)), None);
    let t = match d {
        Some(d) => {
            let c = g.dr[d].frame_centre;
            let at = g.ch[g.pl[g.player].ch].pos;
            let side = crate::orders::side_of(g, d, at);
            Some(crate::doortools::DoorToolsThresholdThenT::Door(crate::orders::OrderTargetDoor { at: [c[0], 0.0, c[2]], door: d, side }))
        }
        None => None,
    };
    let then = match what { crate::doortools::DoorToolsThresholdTellWhat::Bang => crate::doortools::DoorToolsThresholdThenWhat::Bang, crate::doortools::DoorToolsThresholdTellWhat::Step => crate::doortools::DoorToolsThresholdThenWhat::Step, crate::doortools::DoorToolsThresholdTellWhat::Leave => crate::doortools::DoorToolsThresholdThenWhat::Leave };
    match door_tools_threshold_then(g, then, &men, t.as_ref()) {
        crate::doortools::DoorToolsThresholdThen::Text(s) => Some(s),
        crate::doortools::DoorToolsThresholdThen::Arm1(()) => None,
    }
}

/// (doortools.ts:1260 `floor`) floor at `p`, nothing solid from `from`, and this room's
fn floor_at(g: &mut Game, d: DoorId, side: f64, from: V3, p: V3) -> bool {
    !crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) && !crate::scene_collision::solid_blocked(g, [from[0], 0.45, from[2]], [p[0], 0.45, p[2]], None) && crate::stackroom::stack_room_keeps(g, d, side, p)
}

/// open and clear / bang and clear / clear from outside / the threshold: into the stack first (doortools.ts DoorTools.start)
pub fn door_tools_start(g: &mut crate::game::Game, kind: crate::doortools::DoorToolKind, men: &[GuardId], t: &crate::orders::OrderTarget, on_word: Option<bool>, hold: Option<bool>) -> crate::doortools::DoorToolsStart {
    let on_word = on_word.unwrap_or(false);
    let hold = hold.unwrap_or(false);
    let crate::orders::OrderTarget::Door(target) = t else { return crate::doortools::DoorToolsStart::Text(String::from("no door")) };
    let (d, t_side) = (target.door, target.side);
    door_tools_sync(g);
    let Some(deal) = door_tools_deal_stack(g, d, t_side, men) else { return crate::doortools::DoorToolsStart::Text(crate::orders::squad_orders_no_room_words(g, "no room to stack on that door", &mut |g: &mut Game, _: &mut Game| door_tools_deal_stack(g, d, t_side, men).is_some())) };
    let posts = deal.posts;
    let frag = kind == crate::doortools::DoorToolKind::BangClear && g.door_tools.frag_next;
    if kind == crate::doortools::DoorToolKind::BangClear && !door_tools_take(g, if frag { crate::doortools::DoorToolsTakeWhat::Frag } else { crate::doortools::DoorToolsTakeWhat::Bang }) {
        return crate::doortools::DoorToolsStart::Text(String::from(if frag { "no frags left" } else { "no bangs left" }));
    }
    door_tools_make_room(g, d, &deal.back);
    let op = door_tools_new_op(g, kind, d, t_side, men, crate::doortools::Phase::Stack);
    g.tab.doortools_door_op[op].bang_held = kind == crate::doortools::DoorToolKind::BangClear;
    if frag { g.tab.doortools_door_op[op].frag = Some(true); }
    if g.door_tools.big_next && kind == crate::doortools::DoorToolKind::OpenClear { g.tab.doortools_door_op[op].big = Some(true); }
    g.tab.doortools_door_op[op].on_word = on_word && crate::orders::squad_orders_big(g);
    g.tab.doortools_door_op[op].thr_hold = Some(hold && kind == crate::doortools::DoorToolKind::Threshold);
    let name = crate::orders::door_words(g, d);
    let fc = g.dr[d].frame_centre;
    let watch: V3 = [fc[0], 1.05, fc[2]];
    let mut lead: Option<GuardId> = None;
    let (mut la, mut lx) = (f64::NEG_INFINITY, false);
    let oh = kind == crate::doortools::DoorToolKind::BangClear && crate::stackroom::stack_view_open(g, d);
    let spots: Vec<V3> = posts.iter().map(|x| x.pos).collect();
    let groups: Vec<f64> = posts.iter().map(|x| x.group.unwrap_or(0.0)).collect();
    let mut used = vec![false; posts.len()];
    for (gd, p) in crate::orders::squad_orders_pair(g, men, &spots, Some(&groups), Some(fc)) {
        let pp = post_at(&posts, &mut used, p);
        g.tab.doortools_door_op[op].post.set(gd, pp);
        let stance = if pp.lead { crate::character::Stance::Stack } else { crate::character::Stance::HighReady };
        door_tools_assign(g, op, gd, &stacked_a(pp.pos, pp.face, watch, stance, pp.stack_side, &name));
        let a = if pp.lead { along_of(g, d, pp.pos) } else { f64::NEG_INFINITY };
        let at = pos_of(g, gd);
        let x = oh && pp.lead && door_tools_crosses_open(g, d, t_side, at, pp.pos);
        if if lead.is_some() && pp.lead && x != lx { !x } else { a > la } { la = a; lead = Some(gd); lx = x; }
    }
    let the_lead = lead.unwrap_or(men[0]);
    g.tab.doortools_door_op[op].lead = the_lead;
    if oh { door_tools_lead_hand(g, op, &posts, watch); }
    let the_lead = g.tab.doortools_door_op[op].lead;
    let mut filed: JsSet<GuardId> = JsSet::new();
    let bang_file = g.k.doortools.bang_file;
    if kind == crate::doortools::DoorToolKind::Outside && men.len() > 2 {
        // three or more working a door from outside: the lead and one man behind him on his wall, the rest on down that wall, backs to the file
        let l = *g.tab.doortools_door_op[op].post.get(the_lead).unwrap();
        let al = along_of(g, d, l.pos);
        let w = g.dr[d].def.width;
        let dir: f64 = if al > w / 2.0 { 1.0 } else { -1.0 };
        let lead_pair = crate::orders::squad_orders_pair_of(g, the_lead);
        let mut rest: Vec<(GuardId, f64, f64)> = Vec::new();
        for &m in men {
            if m == the_lead { continue; }
            let pair = crate::orders::squad_orders_pair_of(g, m);
            let mate = if pair.is_some() && pair == lead_pair { 0.0 } else { 1.0 };
            let dist = v3::dist_xz(g.tab.doortools_door_op[op].post.get(m).unwrap().pos, l.pos);
            rest.push((m, mate, dist));
        }
        js::sort_by(&mut rest, |a, b| js::or(a.1 - b.1, a.2 - b.2));
        let mut k = 0.0;
        for (gd, _, _) in rest {
            k += 1.0;
            let rear = k >= 2.0;
            if rear { g.tab.doortools_door_op[op].rear.add(gd); }
            let pp = g.tab.doortools_door_op[op].post.get(gd).copied().unwrap_or_default();
            let off = off_of(g, d, l.pos, t_side);
            let mut q = door_pt(g, d, al + dir * k * bang_file, off, t_side, None);
            if !floor_at(g, d, t_side, l.pos, q) {
                if !rear { continue; }
                let far = door_pt(g, d, w - al, off, t_side, None);
                let mut found: Option<V3> = None;
                let mut j = 1.0;
                while j <= 4.0 && found.is_none() {
                    let c = door_pt(g, d, w - al - dir * j * bang_file, off, t_side, None);
                    if floor_at(g, d, t_side, far, c) && !men.iter().any(|&m| m != gd && v3::dist_xz(g.tab.doortools_door_op[op].post.get(m).unwrap().pos, c) < 0.7) { found = Some(c); }
                    j += 1.0;
                }
                let Some(c) = found else { continue };
                q = c;
            }
            let out = door_pt(g, d, w / 2.0 + (if k == 2.0 { dir } else { -dir }) * 4.0, 7.0, t_side, Some(1.05));
            let qq = crate::orders::StackPost { pos: q, face: if rear { v3::yaw_to(q, out) } else { l.face }, stack_side: l.stack_side, lead: false, ..pp };
            g.tab.doortools_door_op[op].post.set(gd, qq);
            filed.add(gd);
            let a = if rear {
                crate::doortools::DoorToolsAssignA {
                    post: Some(q),
                    face: Some(qq.face),
                    watch: Some(out),
                    stance: Some(crate::character::Stance::None),
                    upper: Some(crate::character::UpperMode::Aim),
                    going: format!("rear security · {}", name),
                    there: format!("rear security · {}", name),
                    ..Default::default()
                }
            } else {
                stacked_a(q, qq.face, watch, crate::character::Stance::HighReady, qq.stack_side, &name)
            };
            door_tools_assign(g, op, gd, &a);
        }
    }
    let split = kind == crate::doortools::DoorToolKind::OpenClear || kind == crate::doortools::DoorToolKind::Threshold;
    if !split && men.len() > 2 {
        // behind a bang (or a man the file above had no floor for): the men on the other jamb stand back a place along their own wall
        let w = g.dr[d].def.width;
        let l = *g.tab.doortools_door_op[op].post.get(the_lead).unwrap();
        let l_hi = along_of(g, d, l.pos) > w / 2.0;
        let mut far_first: Vec<(GuardId, f64)> = men.iter().map(|&m| (m, v3::dist_xz(g.tab.doortools_door_op[op].post.get(m).unwrap().pos, fc))).collect();
        js::sort_by(&mut far_first, |a, b| b.1 - a.1);
        for (gd, _) in far_first {
            let pp = *g.tab.doortools_door_op[op].post.get(gd).unwrap();
            if gd == the_lead || filed.has(gd) { continue; }
            let along = along_of(g, d, pp.pos);
            let hi = along > w / 2.0;
            if hi == l_hi { continue; }
            let off = off_of(g, d, pp.pos, t_side);
            let q = door_pt(g, d, along + (if hi { 1.0 } else { -1.0 }) * bang_file, off, t_side, None);
            if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2])
                || crate::scene_collision::solid_blocked(g, [pp.pos[0], 0.45, pp.pos[2]], [q[0], 0.45, q[2]], None)
                || men.iter().any(|&m| m != gd && v3::dist_xz(g.tab.doortools_door_op[op].post.get(m).unwrap().pos, q) < 0.7)
                || !crate::stackroom::stack_room_keeps(g, d, t_side, q) { continue; }
            let qq = crate::orders::StackPost { pos: q, lead: false, ..pp };
            g.tab.doortools_door_op[op].post.set(gd, qq);
            door_tools_assign(g, op, gd, &stacked_a(q, qq.face, watch, crate::character::Stance::HighReady, qq.stack_side, &name));
        }
    } else if !split {
        // behind a bang, or from outside, only the man who opens is at the frame: the rest file up behind him along his wall
        let l = *g.tab.doortools_door_op[op].post.get(the_lead).unwrap();
        let al = along_of(g, d, l.pos);
        let dir: f64 = if al > g.dr[d].def.width / 2.0 { 1.0 } else { -1.0 };
        let mut k = 0.0;
        for &gd in men {
            if gd == the_lead { continue; }
            k += 1.0;
            let pp = g.tab.doortools_door_op[op].post.get(gd).copied().unwrap_or_default();
            let off = off_of(g, d, l.pos, t_side);
            let q = door_pt(g, d, al + dir * k * bang_file, off, t_side, None);
            if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2])
                || crate::scene_collision::solid_blocked(g, [l.pos[0], 0.45, l.pos[2]], [q[0], 0.45, q[2]], None)
                || !crate::stackroom::stack_room_keeps(g, d, t_side, q) { continue; }
            let qq = crate::orders::StackPost { pos: q, face: l.face, stack_side: l.stack_side, lead: false, ..pp };
            g.tab.doortools_door_op[op].post.set(gd, qq);
            door_tools_assign(g, op, gd, &stacked_a(q, qq.face, watch, crate::character::Stance::HighReady, qq.stack_side, &name));
        }
    }
    if oh { door_tools_keep_hands(g, op, &posts, watch); }
    door_tools_lane_cover(g, op);
    let mut who = Vec::with_capacity(men.len());
    for &m in men { who.push(g.gd[m].callsign.as_str()); }
    let line = format!("ordered · {} · lead {}", who.join(", "), g.gd[the_lead].callsign);
    door_tools_beat(g, op, &line);
    crate::doortools::DoorToolsStart::DoorOp(op)
}

/// which hand of the opening a point is on, on this face: +1 past the far jamb, -1 before the near one, 0 on neither (doortools.ts DoorTools.handOf)
pub fn door_tools_hand_of(g: &mut crate::game::Game, d: DoorId, side: f64, p: V3) -> f64 {
    if off_of(g, d, p, side) <= 0.0 || v3::dist_xz(p, g.dr[d].frame_centre) > g.k.stackroom.view_men { return 0.0; }
    let w = g.dr[d].def.width;
    let a = along_of(g, d, p);
    let h = if a >= w + 0.05 { 1.0 } else if a <= -0.05 { -1.0 } else { 0.0 };
    if h == 0.0 { return 0.0; }
    let j = door_pt(g, d, if h > 0.0 { w + 0.35 } else { -0.35 }, 0.45, side, None);
    if v3::dist_xz(p, j) > 0.5 && crate::scene_collision::solid_blocked(g, [p[0], 0.45, p[2]], [j[0], 0.45, j[2]], None) { 0.0 } else { h }
}

/// a charge order whose door came open under it: a man still on his way whose mark is on the other hand of the opening stops where he is (doortools.ts DoorTools.stayHands)
pub fn door_tools_stay_hands(g: &mut crate::game::Game, op: DoorOpId) {
    let (d, side) = (g.tab.doortools_door_op[op].door, g.tab.doortools_door_op[op].side);
    let name = crate::orders::door_words(g, d);
    let c = g.dr[d].frame_centre;
    for m in g.tab.doortools_door_op[op].men.clone() {
        let Some(a) = g.tab.doortools_door_op[op].mine.get(m).copied() else { continue };
        if is_dead(g, m) || crate::orders::squad_orders_of(g, m) != Some(a) { continue; }
        let Some(post) = g.tab.orders_assignment[a].post else { continue };
        if door_tools_arrived(g, op, m) { continue; }
        let p = pos_of(g, m);
        let at: V3 = [p[0], 0.0, p[2]];
        if !door_tools_crosses_open(g, d, side, at, post) { continue; }
        let assign = crate::orders::SquadOrdersAssignA {
            order: String::from("stack"),
            post: Some(at),
            face: Some(m::atan2(c[0] - at[0], c[2] - at[2])),
            watch: Some([c[0], 1.05, c[2]]),
            door: Some(d),
            shy: Some(d),
            stance: Some(crate::character::Stance::HighReady),
            upper: Some(crate::character::UpperMode::None),
            move_: Some(crate::doortools::DoorToolsDoneItemLaneBackMove { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim }),
            stack_side: Some(g.tab.orders_assignment[a].stack_side),
            going: format!("stacking · {}", name),
            there: format!("stacked · {}", name),
            ..Default::default()
        };
        crate::orders::squad_orders_assign(g, m, &assign);
        g.door_tools.stats.hand_kept += 1.0;
        let line = format!("hand kept · {}", g.gd[m].callsign);
        door_tools_beat(g, op, &line);
    }
}

/// a walk from where he stands to that place changes hands in front of the opening (doortools.ts DoorTools.crossesOpen)
pub fn door_tools_crosses_open(g: &mut crate::game::Game, d: DoorId, side: f64, from: V3, to: V3) -> bool {
    let h = door_tools_hand_of(g, d, side, from);
    h != 0.0 && (if along_of(g, d, to) > g.dr[d].def.width / 2.0 { 1.0 } else { -1.0 }) != h
}

/// (doortools.ts:1325, 1343 `others`) Sam and the teammates not on the order: nobody is sent to stand on them
fn others_of(g: &mut Game, op: DoorOpId) -> Vec<V3> {
    let mut out = vec![crate::orders::squad_orders_sam_stood(g)];
    for x in g.guards.clone() {
        let ours = if crate::orders::squad_orders_sams(g) { g.gd[x].team == crate::teams::Team::Operative } else { crate::orders::squad_orders_mine(g, x) };
        if ours && !is_dead(g, x) && !g.tab.doortools_door_op[op].men.contains(&x) { out.push(pos_of(g, x)); }
    }
    out
}

/// (doortools.ts:1326, 1344 `free`) clear of every man's place, floor, this room's
fn place_free(g: &mut Game, op: DoorOpId, gd: GuardId, others: &[V3], c: V3) -> bool {
    let (d, side) = (g.tab.doortools_door_op[op].door, g.tab.doortools_door_op[op].side);
    for m in g.tab.doortools_door_op[op].men.clone() {
        if m == gd { continue; }
        let at = match g.tab.doortools_door_op[op].post.get(m) { Some(p) => p.pos, None => pos_of(g, m) };
        if v3::dist_xz(at, c) < 0.7 { return false; }
    }
    if others.iter().any(|&p| v3::dist_xz(p, c) < 0.7) { return false; }
    !crate::scene_collision::nav_grid_is_blocked(g, c[0], c[2]) && crate::stackroom::stack_room_keeps(g, d, side, c)
}

/// the door stands open: a man of the order whose place is on the other hand of the opening takes his own hand's wall (doortools.ts DoorTools.keepHands)
pub fn door_tools_keep_hands(g: &mut crate::game::Game, op: DoorOpId, posts: &[crate::orders::StackPost], watch: V3) {
    let (d, side) = (g.tab.doortools_door_op[op].door, g.tab.doortools_door_op[op].side);
    let w = g.dr[d].def.width;
    let name = crate::orders::door_words(g, d);
    let bang_file = g.k.doortools.bang_file;
    let fc = g.dr[d].frame_centre;
    for gd in g.tab.doortools_door_op[op].men.clone() {
        let Some(pp) = g.tab.doortools_door_op[op].post.get(gd).copied() else { continue };
        if gd == g.tab.doortools_door_op[op].lead || g.tab.doortools_door_op[op].rear.has(gd) { continue; }
        let at = pos_of(g, gd);
        if !door_tools_crosses_open(g, d, side, at, pp.pos) { continue; }
        let h = door_tools_hand_of(g, d, side, at);
        let mut on_hand: Vec<(crate::orders::StackPost, f64)> = Vec::new();
        for x in posts {
            if (if along_of(g, d, x.pos) > w / 2.0 { 1.0 } else { -1.0 }) == h { on_hand.push((*x, v3::dist_xz(x.pos, fc))); }
        }
        js::sort_by(&mut on_hand, |a, b| a.1 - b.1);
        let Some(&(jj, _)) = on_hand.first() else { continue };
        let aj = along_of(g, d, jj.pos);
        let oj = off_of(g, d, jj.pos, side);
        let others = others_of(g, op);
        let mut q: Option<V3> = None;
        let mut j = 1.0;
        while j <= 3.0 && q.is_none() {
            let c = door_pt(g, d, aj + h * j * bang_file, oj, side, None);
            if place_free(g, op, gd, &others, c) && !crate::scene_collision::solid_blocked(g, [jj.pos[0], 0.45, jj.pos[2]], [c[0], 0.45, c[2]], None) { q = Some(c); }
            j += 1.0;
        }
        if q.is_none() && place_free(g, op, gd, &others, jj.pos) { q = Some(jj.pos); }
        let Some(q) = q else { continue };
        let qq = crate::orders::StackPost { pos: q, lead: false, ..jj };
        g.tab.doortools_door_op[op].post.set(gd, qq);
        g.door_tools.stats.hand_kept += 1.0;
        door_tools_assign(g, op, gd, &stacked_a(q, qq.face, watch, crate::character::Stance::HighReady, qq.stack_side, &name));
        let line = format!("hand kept · {}", g.gd[gd].callsign);
        door_tools_beat(g, op, &line);
    }
}

/// the lead himself, whom a deal can leave no lead mark on his own hand: his place is then his own hand's jamb. None = the place as dealt (doortools.ts DoorTools.leadHand)
pub fn door_tools_lead_hand(g: &mut crate::game::Game, op: DoorOpId, posts: &[crate::orders::StackPost], watch: V3) {
    let (d, side, gd) = { let o = &g.tab.doortools_door_op[op]; (o.door, o.side, o.lead) };
    let w = g.dr[d].def.width;
    let name = crate::orders::door_words(g, d);
    let Some(pp) = g.tab.doortools_door_op[op].post.get(gd).copied() else { return };
    let at = pos_of(g, gd);
    if !pp.lead || !door_tools_crosses_open(g, d, side, at, pp.pos) { return; }
    let h = door_tools_hand_of(g, d, side, at);
    let others = others_of(g, op);
    let mut jj: Option<crate::orders::StackPost> = None;
    for x in posts {
        if !x.lead || x.pos == pp.pos { continue; }
        if (if along_of(g, d, x.pos) > w / 2.0 { 1.0 } else { -1.0 }) != h { continue; }
        if place_free(g, op, gd, &others, x.pos) && !crate::scene_collision::solid_blocked(g, [at[0], 0.45, at[2]], [x.pos[0], 0.45, x.pos[2]], None) { jj = Some(*x); break; }
    }
    let qq = match jj {
        Some(j) => j,
        None => {
            let along = along_of(g, d, pp.pos);
            let off = off_of(g, d, pp.pos, side);
            let q = door_pt(g, d, w - along, off, side, None);
            if (if along_of(g, d, q) > w / 2.0 { 1.0 } else { -1.0 }) != h { return; }
            if !place_free(g, op, gd, &others, q) || crate::scene_collision::solid_blocked(g, [at[0], 0.45, at[2]], [q[0], 0.45, q[2]], None) { return; }
            crate::orders::StackPost { pos: q, face: v3::wrap_angle(pp.face + core::f64::consts::PI), stack_side: -pp.stack_side, ..pp }
        }
    };
    g.tab.doortools_door_op[op].post.set(gd, qq);
    g.door_tools.stats.hand_kept += 1.0;
    door_tools_assign(g, op, gd, &stacked_a(qq.pos, qq.face, watch, crate::character::Stance::Stack, qq.stack_side, &name));
    let line = format!("hand kept · {}", g.gd[gd].callsign);
    door_tools_beat(g, op, &line);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_face_learnt_keeps_the_other_face() {
        // shut from +1, free from -1: bits 1 and 8
        let was = learned(learned(0.0, 1.0, Some(false)), -1.0, Some(true));
        assert_eq!(was, 9.0);
        assert_eq!(js::to_i32(was) & know_mask(1.0, false), 1);
        assert_eq!(js::to_i32(was) & know_mask(1.0, true), 0);
        assert_eq!(js::to_i32(was) & know_mask(-1.0, true), 8);
        // found free now on +1: its shut bit goes; forgetting -1 takes its free bit
        assert_eq!(learned(was, 1.0, Some(true)), 12.0);
        assert_eq!(learned(was, -1.0, None), 1.0);
    }

    #[test]
    fn assign_spread_the_row_wins_over_the_order() {
        let a = crate::doortools::DoorToolsAssignA { going: String::from("g"), there: String::from("t"), ..Default::default() };
        let m = merged_assign("openClear", DoorId(3), Some(DoorId(3)), &a);
        assert_eq!(m.order, "openClear");
        assert_eq!(m.door, Some(DoorId(3)));
        assert_eq!(m.shy, Some(DoorId(3)));
        let b = crate::doortools::DoorToolsAssignA { order: Some(String::from("stack")), door: Some(DoorId(7)), ..a };
        let n = merged_assign("openClear", DoorId(3), None, &b);
        assert_eq!(n.order, "stack");
        assert_eq!(n.door, Some(DoorId(7)));
        assert_eq!(n.shy, None);
    }

    #[test]
    fn two_men_on_one_place_get_the_two_posts_dealt_there() {
        let at = [1.0, 0.0, 2.0];
        let posts = [
            crate::orders::StackPost { pos: at, face: 1.0, ..Default::default() },
            crate::orders::StackPost { pos: [5.0, 0.0, 5.0], face: 2.0, ..Default::default() },
            crate::orders::StackPost { pos: at, face: 3.0, ..Default::default() },
        ];
        let mut used = [false; 3];
        assert_eq!(post_at(&posts, &mut used, at).face, 1.0);
        assert_eq!(post_at(&posts, &mut used, at).face, 3.0);
        assert_eq!(post_at(&posts, &mut used, [5.0, 0.0, 5.0]).face, 2.0);
    }
}
