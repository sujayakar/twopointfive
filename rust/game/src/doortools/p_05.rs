//! src/game/doortools.ts lines 1743 to 2146
#![allow(non_snake_case)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// a door order's row
macro_rules! dop {
    ($g:expr, $op:expr) => {
        $g.tab.doortools_door_op[$op]
    };
}
/// a door order's threshold (`op.thr!`)
macro_rules! thr {
    ($g:expr, $op:expr) => {
        $g.tab.doortools_door_op[$op].thr.as_mut().unwrap()
    };
}

// ------------------------------------------------------------ small reads

/// `input.pressed.delete(key)`
fn press_off(g: &mut Game, input: InputId, key: &str) {
    g.tab.input[input].pressed.delete(String::from(key));
}
fn pos_of(g: &mut Game, gd: GuardId) -> V3 {
    let c = g.gd[gd].ch;
    g.ch[c].pos
}
fn sam_pos(g: &mut Game) -> V3 {
    let c = g.pl[g.player].ch;
    g.ch[c].pos
}
fn is_dead(g: &mut Game, gd: GuardId) -> bool {
    g.gd[gd].state == crate::game::GuardState::Dead
}
fn callsigns(g: &mut Game, men: &[GuardId]) -> String {
    let names: Vec<String> = men.iter().map(|&m| g.gd[m].callsign.clone()).collect();
    names.join(", ")
}
fn alive(g: &mut Game, men: &[GuardId]) -> Vec<GuardId> {
    men.iter().copied().filter(|&m| g.gd[m].state != crate::game::GuardState::Dead).collect()
}
fn any_shared(a: &[GuardId], b: &[GuardId]) -> bool {
    a.iter().any(|m| b.contains(m))
}
/// `[...new Set(words)].join(' + ')`
fn unique_joined(words: Vec<String>) -> String {
    let mut seen: Vec<String> = Vec::new();
    for w in words {
        if !seen.contains(&w) {
            seen.push(w);
        }
    }
    seen.join(" + ")
}
fn say(g: &mut Game, gd: GuardId, text: &str) {
    crate::game::game_say(g, gd, text, None, None, None);
}
/// `const PRE`: the phases before the door goes (doortools.ts:621)
fn is_pre(ph: Phase) -> bool {
    matches!(ph, Phase::Stack | Phase::Ready | Phase::Place | Phase::Standoff | Phase::Armed)
}
/// `if (op.word) { op.word = null; op.worded = true; }`
fn word_off(g: &mut Game, op: DoorOpId) {
    if dop!(g, op).word.is_some() {
        dop!(g, op).word = None;
        dop!(g, op).worded = Some(true);
    }
}
/// `list.splice(list.indexOf(x), 1)`: a -1 from indexOf takes the LAST entry, as splice does
fn splice_out<T: PartialEq>(list: &mut Vec<T>, x: &T) {
    match list.iter().position(|y| y == x) {
        Some(i) => {
            list.remove(i);
        }
        None => {
            list.pop();
        }
    }
}
fn recipients_of(g: &mut Game, t: &crate::orders::OrderTarget) -> Vec<GuardId> {
    use crate::orders::{OrderTarget as O, SquadOrdersRecipientsT as R};
    let r = match t.clone() {
        O::Floor(x) => R::Floor(x),
        O::Door(x) => R::Door(x),
        O::None(x) => R::None(x),
        O::Foe(x) => R::Foe(x),
    };
    crate::orders::squad_orders_recipients(g, Some(&r), None)
}
/// `S.give('go', { kind: 'none', at: [p[0], 0, p[2]] }, undefined, { men })`
fn give_go(g: &mut Game, p: V3, men: &[GuardId]) -> Option<String> {
    let t = crate::orders::OrderTarget::None(crate::orders::OrderTargetNone { at: [p[0], 0.0, p[2]], man: None });
    let opts = crate::orders::GiveOpts { men: Some(men.to_vec()), ..Default::default() };
    crate::orders::squad_orders_give(g, "go", Some(&t), None, Some(&opts))
}
/// the row of the menu that is up with this hot key: -1 with none (or no menu)
fn menu_row_hot(g: &mut Game, key: &str) -> i32 {
    let Some(rows) = g.orders.menu.as_ref().map(|m| m.rows.clone()) else { return -1 };
    match rows.iter().position(|&d| g.tab.orders_order_def[d].hot_key.as_deref() == Some(key)) {
        Some(i) => i as i32,
        None => -1,
    }
}
fn trigger_free_keyed(g: &mut Game) {
    let set = g.door_tools.set.clone();
    let mut list = Vec::new();
    for c in set {
        if !door_tools_on_pending_word(g, c) && door_tools_key_is(g, c) {
            list.push(c);
        }
    }
    for c in list {
        door_tools_trigger(g, Some(c));
    }
}

// ------------------------------------------------------------ who a can would catch

/// is `p` in a stun can's reach through that door, the line asked without this door's own leaf (doortools.ts canReach)
pub fn door_tools_can_reach(g: &mut crate::game::Game, d: DoorId, side: f64, p: V3) -> bool {
    let fc = g.dr[d].frame_centre;
    if !(off_of(g, d, p, side) < 0.0 && v3::dist_xz(p, fc) < g.k.doortools.stun_m) {
        return false;
    }
    let w = g.dr[d].def.width;
    let inn = door_pt(g, d, w / 2.0, 0.45, -side, Some(1.1));
    // (the try/finally of the TypeScript: the list is put back, nothing in between can leave early)
    let (leaf, keep) = (g.dr[d].box_, g.col.dynamic_boxes.clone());
    g.col.dynamic_boxes.retain(|&b| b != leaf);
    let blocked = crate::scene_collision::solid_blocked(g, inn, [p[0], 1.1, p[2]], None);
    g.col.dynamic_boxes = keep;
    !blocked
}
/// Sam himself beyond the door, in a stun can's reach (doortools.ts samBehind)
pub fn door_tools_sam_behind(g: &mut crate::game::Game, d: DoorId, side: f64) -> bool {
    if g.pl[g.player].down {
        return false;
    }
    if !crate::orders::squad_orders_sams(g) {
        return false;
    }
    let p = sam_pos(g);
    door_tools_can_reach(g, d, side, p)
}
/// is `x` part of this entry, for the can (doortools.ts mateBehind `own`)
fn entry_own(g: &mut Game, op: DoorOpId, x: GuardId) -> bool {
    if dop!(g, op).men.contains(&x) {
        return true;
    }
    let (word, gate) = (dop!(g, op).word, dop!(g, op).gate);
    let ops = g.door_tools.ops.clone();
    for o in ops {
        if o != op
            && ((word.is_some() && dop!(g, o).word == word) || (gate.is_some() && dop!(g, o).gate == gate))
            && dop!(g, o).men.contains(&x)
        {
            let (door, side) = (dop!(g, o).door, dop!(g, o).side);
            let p = pos_of(g, x);
            if off_of(g, door, p, side) >= 0.0 {
                return true;
            }
        }
    }
    false
}
/// a living teammate in the can's reach who is no part of this entry (doortools.ts mateBehind)
pub fn door_tools_mate_behind(g: &mut crate::game::Game, op: DoorOpId) -> Option<GuardId> {
    let d = dop!(g, op).door;
    if !crate::orders::squad_orders_sams(g) {
        return None;
    }
    let side = dop!(g, op).side;
    for x in crate::orders::squad_orders_men(g, None, None) {
        if !entry_own(g, op, x) {
            let p = pos_of(g, x);
            if door_tools_can_reach(g, d, side, p) {
                return Some(x);
            }
        }
    }
    None
}
/// who of ours the squad's own can through that door would catch: a callsign, or "you" (doortools.ts canBehind)
pub fn door_tools_can_behind(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Option<String> {
    if let Some(mate) = door_tools_behind(g, d, side, men) {
        return Some(g.gd[mate].callsign.clone());
    }
    if door_tools_sam_behind(g, d, side) {
        Some(String::from("you"))
    } else {
        None
    }
}

// ------------------------------------------------------------ the word

/// what is waiting on Sam's word among these men (doortools.ts waiting4)
pub fn door_tools_waiting4(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<crate::doortools::DoorToolsWaiting4Item> {
    let mut out = Vec::new();
    let ops = g.door_tools.ops.clone();
    for op in ops {
        let o = &dop!(g, op);
        if any_shared(&o.men, men)
            && o.word.is_none()
            && (o.on_word || matches!(o.phase, Phase::Held | Phase::Armed | Phase::Standoff | Phase::Place))
        {
            out.push(DoorToolsWaiting4Item::DoorOp(op));
        }
    }
    for x in door_tools_stacks_of(g, men) {
        out.push(DoorToolsWaiting4Item::Arm1(DoorToolsWaiting4ItemArm1 { stack: x.stack, men: x.men }));
    }
    out
}
/// the men among `men` standing on a plain 'stack up', a door at a time (doortools.ts stacksOf)
pub fn door_tools_stacks_of(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<crate::doortools::DoorToolsStacksOfItem> {
    let mut by: JsMap<DoorId, [Vec<GuardId>; 2]> = JsMap::new();
    for &gd in men {
        let Some(a) = crate::orders::squad_orders_of(g, gd) else { continue };
        if g.gd[gd].state == crate::game::GuardState::Dead {
            continue;
        }
        let A = &g.tab.orders_assignment[a];
        let (Some(door), Some(post)) = (A.door, A.post) else { continue };
        if !(A.order == "stack" && A.there.starts_with("stacked")) {
            continue;
        }
        let sam = sam_pos(g);
        let i = if crate::orders::side_of(g, door, post) == crate::orders::side_of(g, door, sam) { 0 } else { 1 };
        by.get_or_insert_with(door, || [Vec::new(), Vec::new()])[i].push(gd);
    }
    let mut out = Vec::new();
    for (stack, l) in by.iter() {
        let [near, far] = l;
        out.push(DoorToolsStacksOfItem { stack, men: if !near.is_empty() { near.clone() } else { far.clone() } });
    }
    out
}
/// the small squad's word: its plain stacks, but for one on a door with a charge set on it (doortools.ts stackWaits)
pub fn door_tools_stack_waits(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<crate::doortools::DoorToolsStackWaitsItem> {
    if crate::orders::squad_orders_big(g) {
        return Vec::new();
    }
    let mut out = Vec::new();
    for x in door_tools_stacks_of(g, men) {
        if door_tools_charge_on(g, x.stack).is_none() {
            out.push(DoorToolsStackWaitsItem { stack: x.stack, men: x.men });
        }
    }
    out
}
/// (doortools.ts stackWords)
pub fn door_tools_stack_words(g: &mut crate::game::Game, men: &[GuardId]) -> String {
    let mut words = Vec::new();
    for x in door_tools_stack_waits(g, men) {
        words.push(crate::orders::squad_orders_who_words(g, Some(&x.men)));
    }
    unique_joined(words)
}
/// the men of a waiting thing (`x.men`)
fn item_men(g: &mut Game, x: &DoorToolsWaitingAnyItem) -> Vec<GuardId> {
    match x {
        DoorToolsWaitingAnyItem::DoorOp(op) => dop!(g, *op).men.clone(),
        DoorToolsWaitingAnyItem::Arm1(s) => s.men.clone(),
    }
}
/// the men among `men` with something waiting on the word (doortools.ts waitingMen)
pub fn door_tools_waiting_men(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<GuardId> {
    let mut on: JsSet<GuardId> = JsSet::new();
    for x in door_tools_waiting_any(g, men) {
        for m in item_men(g, &x) {
            on.add(m);
        }
    }
    men.iter().copied().filter(|&m| on.has(m)).collect()
}
/// the men the word is for: those the orders go to, or whoever stands waiting on it when none of those does (doortools.ts wordMen)
pub fn door_tools_word_men(g: &mut crate::game::Game) -> Vec<GuardId> {
    let men = crate::orders::squad_orders_recipients(g, None, None);
    let big = crate::orders::squad_orders_big(g);
    if if big { !door_tools_waiting_any(g, &men).is_empty() } else { !door_tools_stack_waits(g, &men).is_empty() } {
        return men;
    }
    let all = crate::orders::squad_orders_men(g, Some(crate::orders::SquadOrdersMenSel::All), None);
    let waiting = if crate::orders::squad_orders_big(g) {
        let list = door_tools_waiting_any(g, &all);
        let mut any = false;
        for x in list {
            let yes = match x {
                DoorToolsWaitingAnyItem::Arm1(_) => true,
                DoorToolsWaitingAnyItem::DoorOp(op) => {
                    let d = dop!(g, op).door;
                    door_tools_charge_on(g, d).is_none()
                }
            };
            if yes {
                any = true;
                break;
            }
        }
        any
    } else {
        !door_tools_stack_waits(g, &all).is_empty()
    };
    if waiting {
        all
    } else {
        men
    }
}
/// (doortools.ts waitingAny)
pub fn door_tools_waiting_any(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<crate::doortools::DoorToolsWaitingAnyItem> {
    let mut out = Vec::new();
    for x in door_tools_waiting4(g, men) {
        out.push(match x {
            DoorToolsWaiting4Item::DoorOp(op) => DoorToolsWaitingAnyItem::DoorOp(op),
            DoorToolsWaiting4Item::Arm1(s) => DoorToolsWaitingAnyItem::Arm1(DoorToolsWaitingAnyItemArm1 { stack: s.stack, men: s.men }),
        });
    }
    for op in door_tools_threshold_held(g, men, None) {
        if dop!(g, op).word.is_none() {
            out.push(DoorToolsWaitingAnyItem::DoorOp(op));
        }
    }
    out
}
/// what of these men's waits on THAT door, and who (doortools.ts waitsOn)
pub fn door_tools_waits_on(g: &mut crate::game::Game, men: &[GuardId], d: DoorId) -> Option<crate::doortools::DoorToolsWaitsOn> {
    let mut st: Option<Vec<GuardId>> = None;
    for x in door_tools_waiting_any(g, men) {
        match x {
            DoorToolsWaitingAnyItem::Arm1(s) => {
                if s.stack == d && st.is_none() {
                    st = Some(s.men);
                }
            }
            DoorToolsWaitingAnyItem::DoorOp(op) => {
                if dop!(g, op).door == d {
                    let of = dop!(g, op).men.clone();
                    return Some(DoorToolsWaitsOn { kind: DoorToolsWaitsOnKind::Word, men: alive(g, &of), set: true });
                }
            }
        }
    }
    let st = st?;
    let mut set = true;
    for &m in &st {
        let on = match crate::orders::squad_orders_of(g, m) {
            Some(a) => match g.tab.orders_assignment[a].post {
                Some(post) => {
                    let arrived = g.tab.orders_assignment[a].arrived;
                    arrived || v3::dist_xz(pos_of(g, m), post) < 0.6
                }
                None => false,
            },
            None => false,
        };
        if !on {
            set = false;
            break;
        }
    }
    Some(DoorToolsWaitsOn { kind: DoorToolsWaitsOnKind::Stack, men: st, set })
}
/// that set charge is on a word that has not gone yet (doortools.ts wordPending)
pub fn door_tools_word_pending(g: &mut crate::game::Game, c_2: ChargeId) -> bool {
    door_tools_on_pending_word(g, c_2)
}
/// (doortools.ts waitingWords)
pub fn door_tools_waiting_words(g: &mut crate::game::Game, men: &[GuardId]) -> String {
    let mut words = Vec::new();
    for x in door_tools_waiting_any(g, men) {
        let of = item_men(g, &x);
        let living = alive(g, &of);
        words.push(crate::orders::squad_orders_who_words(g, Some(&living)));
    }
    unique_joined(words)
}
/// THE WORD: everything waiting on it among `who` goes together. None when something went; else why not (doortools.ts go)
pub fn door_tools_go(g: &mut crate::game::Game, who: Option<crate::orders::Who>) -> Option<String> {
    use crate::orders::{SquadOrdersMenSel as Sel, Who};
    door_tools_sync(g);
    let sel = match who.unwrap_or(Who::All) {
        Who::Num(n) => Sel::Num(n),
        Who::All => Sel::All,
        Who::Red => Sel::Red,
        Who::Blue => Sel::Blue,
    };
    let men = crate::orders::squad_orders_men(g, Some(sel), None);
    if !crate::orders::squad_orders_big(g) {
        return if door_tools_word_on_hold(g, &men) { None } else { Some(String::from("nobody's waiting for you to say go.")) };
    }
    let r = door_tools_go_men(g, &men, None);
    if r.is_some() && door_tools_word_early(g, &men) {
        None
    } else {
        r
    }
}
/// every door held on the word is one the word steps into with no can (doortools.ts wordQuiet)
pub fn door_tools_word_quiet(g: &mut crate::game::Game) -> bool {
    let ops = g.door_tools.ops.clone();
    let held: Vec<DoorOpId> = ops.into_iter().filter(|&o| dop!(g, o).thr.is_some()).collect();
    if held.is_empty() {
        return false;
    }
    for o in held {
        if !door_tools_word_steps(g, o) {
            return false;
        }
    }
    true
}
/// the word steps a held door's men in, no can, unless the step is owed to cuts or to men who ran (doortools.ts wordStepIn)
pub fn door_tools_word_step_in(g: &mut crate::game::Game, op: DoorOpId) {
    let t = g.time;
    if !thr!(g, op).step_owed.unwrap_or(false) && door_tools_cuts_owe(g, op) {
        let T = thr!(g, op);
        T.step_owed = Some(true);
        T.owed_t = Some(t);
        T.cut_owed = Some(true);
        T.frag_t = None;
        word_off(g, op);
        door_tools_beat(g, op, "the word waits for the cuts");
        let lead = dop!(g, op).lead;
        say(g, lead, "checking the doorway first.");
        return;
    }
    if !thr!(g, op).step_owed.unwrap_or(false) && door_tools_thr_frag_wait(g, op) {
        let T = thr!(g, op);
        T.step_owed = Some(true);
        T.owed_t = Some(t);
        T.frag_t = None;
        word_off(g, op);
        return;
    }
    let T = thr!(g, op);
    if T.step_owed.unwrap_or(false) && t - T.owed_t.unwrap_or(-1e9) < g.k.doortools.word_tap {
        word_off(g, op);
        return;
    }
    door_tools_word_step_go(g, op);
}
/// (doortools.ts wordStepGo)
pub fn door_tools_word_step_go(g: &mut crate::game::Game, op: DoorOpId) {
    let wait = g.k.doortools.thr_hand_wait;
    let T = thr!(g, op);
    T.step_owed = Some(false);
    T.frag_t = None;
    T.cut_owed = Some(false);
    if T.frag_said.unwrap_or(false) {
        T.frag_said = Some(false);
        let fled: Vec<GuardId> = T.fled.as_ref().map_or(Vec::new(), |f| f.keys());
        if !fled.is_empty() {
            let spent = T.hand_done.unwrap_or(false) || T.hand_w.unwrap_or(0.0) >= wait;
            let names = callsigns(g, &fled);
            let text = format!("on without {} · {}", names, if spent { "the order's wait is spent" } else { "not waited for" });
            door_tools_beat(g, op, &text);
        }
    }
    door_tools_held_beat(g, op);
    door_tools_to_step(g, op);
}
/// the owed step in, once a tick of the hold: true while it is still waited on (doortools.ts wordOwed)
pub fn door_tools_word_owed(g: &mut crate::game::Game, op: DoorOpId) -> bool {
    let t = g.time;
    if thr!(g, op).cut_owed.unwrap_or(false) {
        if door_tools_cuts_owe(g, op) {
            return true;
        }
        let cap = g.k.doortools.word_cuts_cap;
        let T = thr!(g, op);
        T.cut_owed = Some(false);
        let text = if T.sliced {
            String::from("the cuts are done · in")
        } else if t - T.owed_t.unwrap_or(-1e9) >= cap {
            format!("the cuts not done in {} s · in", cap)
        } else {
            String::from("the cuts are not waited for · in")
        };
        door_tools_beat(g, op, &text);
    }
    if !door_tools_thr_frag_wait(g, op) {
        door_tools_word_step_go(g, op);
        return false;
    }
    let T = thr!(g, op);
    if let Some(ft) = T.frag_t {
        T.hand_w = Some(T.hand_w.unwrap_or(0.0) + m::min(t - ft, 0.1));
    }
    T.frag_t = Some(t);
    if !T.frag_said.unwrap_or(false) {
        T.frag_said = Some(true);
        let fled: Vec<GuardId> = T.fled.as_ref().unwrap().keys();
        let names = callsigns(g, &fled);
        door_tools_beat(g, op, &format!("the door waits: {} not back", names));
        let who = dop!(g, op).men.iter().copied().find(|m| !fled.contains(m)).unwrap_or(dop!(g, op).lead);
        say(g, who, "hold — back on the door.");
    }
    true
}
/// does the word at this held door step in with no can (doortools.ts wordSteps)
pub fn door_tools_word_steps(g: &mut crate::game::Game, op: DoorOpId) -> bool {
    let o = &dop!(g, op);
    o.thr.as_ref().unwrap().word_step.unwrap_or(o.thr_hold.unwrap_or(false))
}
/// the pair's word: a room it is working from the threshold has its hold cut short (doortools.ts wordOnHold)
pub fn door_tools_word_on_hold(g: &mut crate::game::Game, men: &[GuardId]) -> bool {
    let mut any = false;
    for op in door_tools_threshold_held(g, men, None) {
        if dop!(g, op).word.is_some() {
            continue;
        }
        door_tools_beat(g, op, "the word");
        if door_tools_word_steps(g, op) {
            door_tools_word_step_in(g, op);
        } else {
            thr!(g, op).asked = Some(true);
            door_tools_to_pockets(g, op);
        }
        any = true;
    }
    door_tools_word_early(g, men) || any
}
/// is a can of this order's that went in first still in the air or its burst running (doortools.ts wordEarly, wordKeyHeld)
fn can_first_on(g: &mut Game, op: DoorOpId) -> bool {
    matches!(dop!(g, op).thr.as_ref().and_then(|t| t.first), Some(ThresholdFirst::Can) | Some(ThresholdFirst::Burst))
}
/// the word while a can that went in first is still out: it is kept, and taken when they step out (doortools.ts wordEarly)
pub fn door_tools_word_early(g: &mut crate::game::Game, men: &[GuardId]) -> bool {
    let all = g.door_tools.ops.clone();
    let ops: Vec<DoorOpId> = all.into_iter().filter(|&op| can_first_on(g, op) && any_shared(&dop!(g, op).men, men)).collect();
    if ops.is_empty() {
        return false;
    }
    for op in ops {
        thr!(g, op).early = Some(true);
    }
    g.door_tools.word_key_t = g.time;
    true
}
/// would the word's key, pressed now with the menu that is up, be a held threshold's word (doortools.ts wordKeyHeld)
pub fn door_tools_word_key_held(g: &mut crate::game::Game) -> bool {
    if crate::orders::squad_orders_big(g) || g.pl[g.player].down {
        return false;
    }
    let men = crate::orders::squad_orders_recipients(g, None, None);
    for op in door_tools_threshold_held(g, &men, None) {
        if dop!(g, op).word.is_none() {
            return true;
        }
    }
    let ops = g.door_tools.ops.clone();
    ops.into_iter().any(|op| can_first_on(g, op) && any_shared(&dop!(g, op).men, &men))
}
/// the word as Sam gives it: a set charge goes with it, and the key is stamped (doortools.ts pairWord)
pub fn door_tools_pair_word(g: &mut crate::game::Game, men: &[GuardId]) -> bool {
    if !door_tools_word_on_hold(g, men) {
        return false;
    }
    g.door_tools.word_key_t = g.time;
    trigger_free_keyed(g);
    true
}
/// the word to these men (`only`: the small squad's, its plain stacks and nothing else) (doortools.ts goMen)
pub fn door_tools_go_men(g: &mut crate::game::Game, men: &[GuardId], only: Option<&[crate::doortools::DoorToolsGoMenOnlyItem]>) -> Option<String> {
    let held = door_tools_frag_held(g, men);
    if !held.is_empty() {
        for o in held {
            door_tools_beat(g, o, "the word again");
            door_tools_frag_on(g, o);
        }
        door_tools_word_tick(g);
        if go_list(g, men, only).is_empty() {
            return None;
        }
    }
    let list = go_list(g, men, only);
    if list.is_empty() {
        return Some(String::from("nobody's waiting for you to say go."));
    }
    crate::holdfire::hold_fire_order(g, "go", None);
    let w = g.tab.doortools_word.push(Word { t0: g.time, go1: f64::NAN, fled_t: None });
    g.door_tools.words.push(w);
    g.door_tools.stats.words += 1.0;
    for x in list {
        let op = match x {
            DoorToolsWaitingAnyItem::Arm1(s) => door_tools_from_stack(g, &s.men, s.stack),
            DoorToolsWaitingAnyItem::DoorOp(op) => Some(op),
        };
        let Some(op) = op else { continue };
        dop!(g, op).on_word = false;
        dop!(g, op).word = Some(w);
        if matches!(dop!(g, op).phase, Phase::Ready | Phase::Held | Phase::Armed | Phase::Thr) {
            dop!(g, op).hold = 1.0;
        }
        door_tools_beat(g, op, "the word");
        for gd in dop!(g, op).men.clone() {
            let Some(a) = dop!(g, op).mine.get(gd).copied() else { continue };
            if g.tab.orders_assignment[a].there.starts_with("ready") {
                let door = dop!(g, op).door;
                let words = crate::orders::door_words(g, door);
                g.tab.orders_assignment[a].there = format!("stacked · {}", words);
            }
        }
    }
    let ops = g.door_tools.ops.clone();
    let mut went: Vec<String> = Vec::new();
    for o in ops {
        if dop!(g, o).word == Some(w) {
            let door = dop!(g, o).door;
            went.push(format!("{} {}", dop!(g, o).kind.as_str(), g.dr[door].def.name));
        }
    }
    door_tools_note(g, &format!("go · {}", went.join(" + ")));
    door_tools_word_tick(g);
    None
}
/// `only ?? this.waitingAny(men)`
fn go_list(g: &mut Game, men: &[GuardId], only: Option<&[DoorToolsGoMenOnlyItem]>) -> Vec<DoorToolsWaitingAnyItem> {
    match only {
        Some(l) => l.iter().map(|x| DoorToolsWaitingAnyItem::Arm1(DoorToolsWaitingAnyItemArm1 { stack: x.stack, men: x.men.clone() })).collect(),
        None => door_tools_waiting_any(g, men),
    }
}
/// a pair standing on a plain 'stack up' becomes an open-and-clear from the very marks it stands on (doortools.ts fromStack)
pub fn door_tools_from_stack(g: &mut crate::game::Game, men: &[GuardId], d: DoorId) -> Option<DoorOpId> {
    let first = men.first().copied().and_then(|m| crate::orders::squad_orders_of(g, m))?;
    let first_post = g.tab.orders_assignment[first].post?;
    let side = crate::orders::side_of(g, d, first_post);
    let name = crate::orders::door_words(g, d);
    let w = g.dr[d].def.width;
    let mut posts: Vec<crate::orders::StackPost> = Vec::new();
    let mut on: Vec<bool> = Vec::new();
    for &gd in men {
        let a = crate::orders::squad_orders_of(g, gd).unwrap();
        let body = g.gd[gd].ch;
        let A = &g.tab.orders_assignment[a];
        posts.push(crate::orders::StackPost {
            pos: v3::copy(A.post.unwrap()),
            face: A.face.unwrap_or(g.ch[body].body_yaw),
            stack_side: A.stack_side,
            lead: A.stance == crate::character::Stance::Stack,
            group: None,
        });
    }
    for &gd in men {
        let a = crate::orders::squad_orders_of(g, gd).unwrap();
        on.push(g.tab.orders_assignment[a].arrived);
    }
    let op = door_tools_new_op(g, DoorToolKind::OpenClear, d, side, men, Phase::Stack);
    let mut lead: Option<GuardId> = None;
    let mut la = f64::NEG_INFINITY;
    let past = g.k.orders.jamb_past;
    let fc = g.dr[d].frame_centre;
    for (i, &gd) in men.iter().enumerate() {
        let P = posts[i];
        dop!(g, op).post.set(gd, P);
        let a = DoorToolsAssignA {
            post: Some(P.pos),
            face: Some(P.face),
            watch: Some([fc[0], 1.05, fc[2]]),
            stance: Some(if P.lead { crate::character::Stance::Stack } else { crate::character::Stance::HighReady }),
            upper: Some(crate::character::UpperMode::None),
            stack_side: Some(P.stack_side),
            going: format!("stacking · {}", name),
            there: format!("stacked · {}", name),
            ..Default::default()
        };
        door_tools_assign(g, op, gd, &a);
        if on[i] {
            let a = dop!(g, op).mine.get(gd).copied().unwrap();
            g.tab.orders_assignment[a].arrived = true;
            g.tab.orders_assignment[a].arr_t = g.time;
        }
        let along = along_of(g, d, P.pos);
        let score = (if P.lead { 10.0 } else { 0.0 }) - m::abs(along - (w + past));
        if score > la {
            la = score;
            lead = Some(gd);
        }
    }
    dop!(g, op).lead = lead.unwrap_or(men[0]);
    let names = callsigns(g, men);
    let lead_name = g.gd[dop!(g, op).lead].callsign.clone();
    door_tools_beat(g, op, &format!("from the stack · {} · lead {}", names, lead_name));
    Some(op)
}
/// the orders on this word
fn word_ops(g: &mut Game, w: WordId) -> Vec<DoorOpId> {
    let ops = g.door_tools.ops.clone();
    ops.into_iter().filter(|&o| dop!(g, o).word == Some(w)).collect()
}
/// once a frame (and on the word itself): the words' two moments (doortools.ts wordTick)
pub fn door_tools_word_tick(g: &mut crate::game::Game) {
    let t = g.time;
    for w in g.door_tools.words.clone() {
        let ops = word_ops(g, w);
        if ops.is_empty() {
            splice_out(&mut g.door_tools.words, &w);
            continue;
        }
        if g.tab.doortools_word[w].go1.is_nan() {
            if ops.iter().any(|&o| door_tools_frag_wait(g, o)) {
                let W = &mut g.tab.doortools_word[w];
                W.t0 += m::min(t - W.fled_t.unwrap_or(t), 0.1);
                W.fled_t = Some(t);
                continue;
            }
            g.tab.doortools_word[w].fled_t = None;
            if !ops.iter().all(|&o| dop!(g, o).hold >= 1.0) && t - g.tab.doortools_word[w].t0 < g.k.doortools.word_stack {
                continue;
            }
            g.tab.doortools_word[w].go1 = t;
            for op in ops {
                word_goes(g, op);
            }
        }
        let left = word_ops(g, w);
        let go1 = g.tab.doortools_word[w].go1;
        if !left.iter().all(|&o| dop!(g, o).hold == 2.0)
            && (t - go1 < g.k.doortools.word_in
                || left.iter().any(|&o| {
                    if !(dop!(g, o).frag.unwrap_or(false) && dop!(g, o).hold != 2.0) {
                        return false;
                    }
                    match dop!(g, o).can {
                        Some(c) => !crate::throwables::item_fired(g, c),
                        None => true,
                    }
                }))
        {
            continue;
        }
        splice_out(&mut g.door_tools.words, &w);
        for op in left {
            dop!(g, op).word = None;
            if dop!(g, op).hold != 2.0 {
                continue;
            }
            dop!(g, op).hold = 0.0;
            match dop!(g, op).phase {
                Phase::Ready => door_tools_open(g, op),
                Phase::Wait | Phase::Held => door_tools_enter(g, op),
                _ => {}
            }
        }
    }
}
/// the first moment of the word, one order's share of it (doortools.ts wordTick, `for (const op of ops)`)
fn word_goes(g: &mut Game, op: DoorOpId) {
    if dop!(g, op).hold < 1.0 {
        dop!(g, op).word = None;
        return;
    }
    door_tools_frag_on(g, op);
    let (kind, door, side) = (dop!(g, op).kind, dop!(g, op).door, dop!(g, op).side);
    if kind == DoorToolKind::Charge {
        match door_tools_charge_on(g, door) {
            Some(c) => {
                door_tools_blow(g, Some(c));
            }
            None => dop!(g, op).word = None,
        }
    } else if dop!(g, op).phase == Phase::Held {
        dop!(g, op).hold = 2.0;
    } else if kind == DoorToolKind::Threshold && dop!(g, op).thr.is_some() {
        dop!(g, op).hold = 0.0;
        if dop!(g, op).phase == Phase::Thr {
            if door_tools_word_steps(g, op) {
                door_tools_word_step_in(g, op);
            } else {
                thr!(g, op).asked = Some(true);
                door_tools_to_pockets(g, op);
            }
        }
    } else if kind == DoorToolKind::Threshold {
        dop!(g, op).hold = 0.0;
        dop!(g, op).word = None;
        dop!(g, op).worded = Some(true);
        door_tools_open(g, op);
    } else if kind == DoorToolKind::BangClear {
        dop!(g, op).hold = 0.0;
        door_tools_open(g, op);
    } else if !is_open(g, door) && word_opens_now(g, op, door, side) {
        dop!(g, op).hold = 0.0;
        door_tools_open(g, op);
    } else {
        dop!(g, op).hold = 2.0;
    }
}
/// a locked door, the big slot's tool, or a lead not at his handle: the door's man goes now
fn word_opens_now(g: &mut Game, op: DoorOpId, door: DoorId, side: f64) -> bool {
    if door_tools_locked_out(g, door, side) {
        return true;
    }
    if dop!(g, op).big.unwrap_or(false) && !dop!(g, op).no_gun.unwrap_or(false) {
        let men = dop!(g, op).men.clone();
        if door_tools_big_opens(g, door, &men) {
            return true;
        }
    }
    let lead = dop!(g, op).lead;
    let at = pos_of(g, lead);
    crate::handleside::tool_spot(g, door, side, at, Some(true), Some(true)).is_some()
}
/// on his door, ready: the leaf is opened, by the shove or the boot from the kick spot when it is locked (doortools.ts open)
pub fn door_tools_open(g: &mut crate::game::Game, op: DoorOpId) {
    let (d, lead, side) = (dop!(g, op).door, dop!(g, op).lead, dop!(g, op).side);
    let t = g.time;
    let name = crate::orders::door_words(g, d);
    if is_open(g, d) {
        door_tools_beat(g, op, "open already");
        let angle = crate::doors::door_angle(g, d);
        if m::abs(angle) < 1.2 {
            crate::doors::door_set_closing(g, d, false);
            let vel = crate::doors::door_vel(g, d);
            let v = js::or(m::sign(angle), 1.0) * m::max(m::abs(vel), g.k.doortools.shove);
            crate::doors::door_set_vel(g, d, v);
            crate::doors::door_set_quiet_t(g, d, -1.0);
            let body = g.gd[lead].ch;
            let who = g.ch[body].id;
            crate::doors::door_set_last_who(g, d, who);
        }
        door_tools_opened(g, op);
        return;
    }
    let men = dop!(g, op).men.clone();
    let big = dop!(g, op).big.unwrap_or(false);
    if door_tools_locked_out(g, d, side) {
        if !(big && door_tools_big_opens(g, d, &men)) && door_tools_halt_at(g, op) {
            return;
        }
        if g.dr[d].def.steel.unwrap_or(false) {
            say(g, lead, "it's steel. no way through.");
            door_tools_end(g, op, "steel and locked");
            return;
        }
        if crate::silent::silent_pick(g) && door_tools_pickable(g, d) {
            door_tools_to_pick(g, op);
            say(g, lead, "locked. picking.");
            return;
        }
        if door_tools_frame_held(g, d, side) && door_tools_big_opens(g, d, &men) && door_tools_to_gun(g, op) {
            return;
        }
        if door_tools_frame_held(g, d, side) && door_tools_frame_answer(g, op) {
            return;
        }
        let stuck: Option<&str> = if door_tools_pickable(g, d) {
            None
        } else if g.dr[d].lock_id.starts_with("wedge:") {
            Some("wedged from inside")
        } else {
            Some("keypad, no code")
        };
        let boot = if stuck.is_some() { crate::silent::silent_boot(g, d) } else { None };
        if boot.is_none() && crate::silent::silent_no_boot(g) {
            let text = match stuck {
                Some(s) => format!("{}. no quiet way in. go loud and we boot it.", s),
                None => String::from(crate::words::wg(g, &"locked. can't kick it while we're silent.", &"locked. no boot while we are silent.")),
            };
            say(g, lead, &text);
            door_tools_end(g, op, "locked · silent: no boot");
            return;
        }
        if boot.is_none() && door_tools_to_gun(g, op) {
            return;
        }
        let ks = door_tools_boot_spot(g, d, side);
        dop!(g, op).kick = Some(DoorToolsDoneItemKick { pos: ks.pos, yaw: ks.yaw, t: f64::NAN, ..Default::default() });
        dop!(g, op).phase = Phase::Kick;
        dop!(g, op).t0 = t;
        let a = DoorToolsAssignA {
            post: Some(ks.pos),
            face: Some(ks.yaw),
            watch: None,
            stance: Some(crate::character::Stance::None),
            upper: Some(crate::character::UpperMode::None),
            going: format!("on the door · {}", name),
            there: String::from("breaching"),
            ..Default::default()
        };
        door_tools_assign(g, op, lead, &a);
        match boot {
            None => say(g, lead, "locked. kicking it."),
            Some(b) => {
                door_tools_beat(g, op, "silent boot");
                if b == crate::silent::SilentBoot::First {
                    say(g, lead, &format!("{}. booting it.", stuck.unwrap_or("")));
                }
            }
        }
    } else {
        if big && door_tools_big_opens(g, d, &men) && door_tools_to_gun(g, op) {
            return;
        }
        if !door_tools_to_handle(g, op, false) {
            door_tools_shove(g, op);
            door_tools_opened(g, op);
        }
    }
}
/// the lead of an unlocked door sent to its handle; false: he is at it, or it cannot be walked to (doortools.ts toHandle)
pub fn door_tools_to_handle(g: &mut crate::game::Game, op: DoorOpId, gate: bool) -> bool {
    let (d, lead, side) = (dop!(g, op).door, dop!(g, op).lead, dop!(g, op).side);
    let at = pos_of(g, lead);
    let Some(hs) = crate::handleside::tool_spot(g, d, side, at, Some(false), Some(true)) else { return false };
    dop!(g, op).kick = Some(DoorToolsDoneItemKick { pos: hs.pos, yaw: hs.yaw, t: f64::NAN, hand: Some(true), gate: Some(gate), ..Default::default() });
    dop!(g, op).phase = Phase::Kick;
    dop!(g, op).t0 = g.time;
    let words = crate::orders::door_words(g, d);
    let a = DoorToolsAssignA {
        post: Some(hs.pos),
        face: Some(hs.yaw),
        watch: None,
        stance: Some(crate::character::Stance::None),
        upper: Some(crate::character::UpperMode::None),
        going: format!("on the door · {}", words),
        there: String::from("on the handle"),
        ..Default::default()
    };
    door_tools_assign(g, op, lead, &a);
    true
}
/// is a door beside this one on its signal still to be booted in (doortools.ts bootBeside)
pub fn door_tools_boot_beside(g: &mut crate::game::Game, op: DoorOpId) -> bool {
    let Some(gate) = dop!(g, op).gate else { return false };
    let kind = dop!(g, op).kind;
    for o in g.tab.doortools_door_gate[gate].ops.clone() {
        if o == op || !g.door_tools.ops.contains(&o) || !dop!(g, o).open_t.is_nan() {
            continue;
        }
        if kind == DoorToolKind::BangClear && dop!(g, o).kind == DoorToolKind::OpenClear {
            continue;
        }
        let phase = dop!(g, o).phase;
        if (phase == Phase::Kick && !dop!(g, o).kick.as_ref().and_then(|k| k.gate).unwrap_or(false))
            || (phase == Phase::Stack && door_tools_will_kick(g, o))
        {
            return true;
        }
    }
    false
}
/// the lead on his way to the handle of an unlocked door: there, or `kickCap` run out, he shoves it (doortools.ts handStep)
pub fn door_tools_hand_step(g: &mut crate::game::Game, op: DoorOpId, el: f64) {
    let (d, lead, side) = (dop!(g, op).door, dop!(g, op).lead, dop!(g, op).side);
    let word = dop!(g, op).word.is_some() && dop!(g, op).kind != DoorToolKind::BangClear;
    if is_open(g, d) {
        dop!(g, op).kick = None;
        door_tools_back_to_jamb(g, op, lead);
        if word {
            dop!(g, op).phase = Phase::Ready;
            dop!(g, op).hold = 2.0;
        } else {
            door_tools_opened(g, op);
        }
        return;
    }
    if door_tools_locked_out(g, d, side) {
        dop!(g, op).kick = None;
        door_tools_open(g, op);
        return;
    }
    let to = dop!(g, op).kick.as_ref().unwrap().pos;
    if !crate::handleside::tool_step(g, lead, to) && el < g.k.doortools.kick_cap {
        return;
    }
    let kick = dop!(g, op).kick.as_mut().unwrap();
    let gate = kick.gate.unwrap_or(false);
    if gate {
        kick.on_it = Some(true);
    }
    if word {
        dop!(g, op).hold = 2.0;
        return;
    }
    if gate && door_tools_boot_beside(g, op) {
        return;
    }
    door_tools_shove(g, op);
    dop!(g, op).kick = None;
    door_tools_back_to_jamb(g, op, lead);
    door_tools_opened(g, op);
}

// ------------------------------------------------------------ the stand-off's marks

/// stackroom.ts's say on a deal of marks (doortools.ts standoffPosts `fix`)
fn standoff_fix(g: &mut Game, d: DoorId, side: f64, posts: Option<&[crate::orders::StackPost]>, men: &[GuardId]) -> Option<Vec<crate::orders::StackPost>> {
    let mut taken = vec![sam_pos(g)];
    taken.extend(crate::orders::squad_orders_held_on(g, d, men, None));
    let min_m = g.k.doortools.backblast_m + 0.2;
    crate::stackroom::stack_room_fix(g, d, side, posts, &taken, Some(min_m))
}
/// picture.ts safestSets over whole deals: the indices of `whole`, safest first. Its stub is typed on `{pos}` alone: each deal it hands back is found again by its marks, the first not yet taken (safestSets keeps its own order among equals, and equal marks weigh equal)
fn safest_order(g: &mut Game, men: &[GuardId], whole: &[Vec<crate::orders::StackPost>]) -> Vec<usize> {
    use crate::picture::SafestSetsSetsItemItem as Q;
    let sets: Vec<Vec<Q>> = whole.iter().map(|F| F.iter().map(|P| Q { pos: P.pos }).collect()).collect();
    let ranked = crate::picture::safest_sets(g, men, &sets);
    let mut used = vec![false; whole.len()];
    let mut out = Vec::new();
    for R in &ranked {
        for (i, F) in whole.iter().enumerate() {
            if !used[i] && F.len() == R.len() && F.iter().zip(R.iter()).all(|(a, b)| a.pos == b.pos) {
                used[i] = true;
                out.push(i);
                break;
            }
        }
    }
    out
}
/// n marks in file along the wall, `standoff` metres past a jamb on face `side`, nearest the door first; None: no wall has the floor (doortools.ts standoffPosts)
pub fn door_tools_standoff_posts(g: &mut crate::game::Game, d: DoorId, side: f64, n: f64, men: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> {
    let men: &[GuardId] = men.unwrap_or(&[]);
    if !crate::picture::any_idea(g, men) {
        let was = door_tools_standoff_posts_as_were(g, d, side, n, Some(men), None);
        return standoff_fix(g, d, side, was.as_deref(), men);
    }
    let mut files: Vec<Vec<crate::orders::StackPost>> = Vec::new();
    let first = door_tools_standoff_posts_as_were(g, d, side, n, Some(men), Some(&mut files));
    let mut whole: Vec<Vec<crate::orders::StackPost>> = Vec::new();
    for F in &files {
        if let Some(fixed) = standoff_fix(g, d, side, Some(F), men) {
            whole.push(fixed);
        }
    }
    if whole.is_empty() {
        return standoff_fix(g, d, side, first.as_deref(), men);
    }
    let ranked = safest_order(g, men, &whole);
    let mut pick = 0usize;
    for i in ranked {
        let F = &whole[i];
        let ok = if men.len() as f64 == n {
            door_tools_standoff_walk(g, men, d, F).is_none()
        } else {
            let mut every = true;
            for P in F {
                let mut some = false;
                for &m in men {
                    if crate::orders::squad_orders_reach(g, m, P.pos).is_none() {
                        some = true;
                        break;
                    }
                }
                if !some {
                    every = false;
                    break;
                }
            }
            every
        };
        if ok {
            pick = i;
            break;
        }
    }
    if pick != 0 {
        g.stats.picture.moved += 1;
    }
    Some(whole.swap_remove(pick))
}
/// the side of the file a man stands on, from his facing (doortools.ts standoffPostsAsWere)
fn stack_side_of(ux: f64, uz: f64, side: f64, fx: f64, fz: f64) -> f64 {
    if (uz * side) * (-fz) + (-ux * side) * fx >= 0.0 {
        1.0
    } else {
        -1.0
    }
}
/// a file out past one jamb, as far as the floor goes (doortools.ts standoffPostsAsWere `fileOf`)
fn file_of(g: &mut Game, d: DoorId, side: f64, n: f64, off: f64, dir: f64, front: V3) -> Vec<crate::orders::StackPost> {
    let (w, th) = (g.dr[d].def.width, g.dr[d].def.closed_dir);
    let (ux, uz) = (m::cos(th), m::sin(th));
    let (jamb_out, gap) = (g.k.orders.jamb_out, g.k.orders.file_gap);
    let mut out = Vec::new();
    let mut k = 0.0;
    while k < n {
        let a = if dir > 0.0 { w + off + k * gap } else { -off - k * gap };
        let pos = door_pt(g, d, a, jamb_out, side, None);
        if crate::scene_collision::nav_grid_is_blocked(g, pos[0], pos[2]) || crate::scene_collision::solid_blocked(g, front, [pos[0], 0.45, pos[2]], None) {
            break;
        }
        let (fx, fz) = (-ux * dir, -uz * dir);
        out.push(crate::orders::StackPost {
            pos,
            face: m::atan2(fx, fz),
            stack_side: stack_side_of(ux, uz, side, fx, fz),
            lead: false,
            group: Some(if dir > 0.0 { 0.0 } else { 1.0 }),
        });
        k += 1.0;
    }
    out
}
/// the marks as the wall deals them: the first file with room (`files`: every such file is also handed back, in that order) (doortools.ts standoffPostsAsWere)
pub fn door_tools_standoff_posts_as_were(g: &mut crate::game::Game, d: DoorId, side: f64, n: f64, men: Option<&[GuardId]>, files: Option<&mut Vec<Vec<crate::orders::StackPost>>>) -> Option<Vec<crate::orders::StackPost>> {
    use crate::orders::StackPost;
    let mut files = files;
    let men: &[GuardId] = men.unwrap_or(&[]);
    let (w, th) = (g.dr[d].def.width, g.dr[d].def.closed_dir);
    let jamb_out = g.k.orders.jamb_out;
    let front = door_pt(g, d, w / 2.0, jamb_out, side, Some(0.45));
    let (ux, uz) = (m::cos(th), m::sin(th));
    let standoff = g.k.doortools.standoff.clone();
    for &off in &standoff {
        for dir in [1.0, -1.0] {
            let gap = g.k.orders.file_gap;
            let mut out: Vec<StackPost> = Vec::new();
            let mut k = 0.0;
            while k < n {
                let a = if dir > 0.0 { w + off + k * gap } else { -off - k * gap };
                let pos = door_pt(g, d, a, jamb_out, side, None);
                if crate::scene_collision::nav_grid_is_blocked(g, pos[0], pos[2]) || crate::scene_collision::solid_blocked(g, front, [pos[0], 0.45, pos[2]], None) {
                    break;
                }
                let (fx, fz) = (-ux * dir, -uz * dir);
                out.push(StackPost { pos, face: m::atan2(fx, fz), stack_side: stack_side_of(ux, uz, side, fx, fz), lead: k == 0.0, group: None });
                k += 1.0;
            }
            if out.len() as f64 == n {
                match files.as_deref_mut() {
                    None => return Some(out),
                    Some(fs) => fs.push(out),
                }
            }
        }
    }
    if let Some(fs) = files.as_deref() {
        if !fs.is_empty() {
            return Some(fs[0].clone());
        }
    }
    if n > 2.0 {
        for &off in &standoff {
            let A = file_of(g, d, side, n, off, 1.0, front);
            let B = file_of(g, d, side, n, off, -1.0, front);
            let mut na = m::min(A.len() as f64, m::ceil(n / 2.0));
            let nb = m::min(B.len() as f64, n - na);
            na = m::min(A.len() as f64, n - nb);
            let mut out: Vec<StackPost> = Vec::new();
            let mut k = 0usize;
            while (k as f64) < m::max(na, nb) {
                if (k as f64) < na {
                    out.push(A[k]);
                }
                if (k as f64) < nb {
                    out.push(B[k]);
                }
                k += 1;
            }
            if (out.len() as f64) < n {
                let taken: Vec<V3> = out.iter().map(|o| o.pos).collect();
                let more = crate::orders::squad_orders_rear_posts(g, d, side, n - out.len() as f64, &taken);
                out.extend(more);
            }
            if (out.len() as f64) < n {
                continue;
            }
            out[0].lead = true;
            return Some(out);
        }
    }
    let authored = g.orders.door_marks.iter().find(|M| M.door == d && M.side == side).map(|M| M.standoff.clone());
    let marks: Option<Vec<V3>> = match authored {
        Some(st) => {
            if st.len() as f64 >= n {
                Some(st[..n as usize].to_vec())
            } else {
                None
            }
        }
        None => {
            let mut taken = vec![sam_pos(g)];
            taken.extend(crate::orders::squad_orders_held_on(g, d, men, None));
            let min_m = g.k.doortools.backblast_m + 0.2;
            crate::orders::squad_orders_worked_pick(g, d, side, n, &taken, Some(min_m))
        }
    };
    if let Some(marks) = marks {
        let fc = g.dr[d].frame_centre;
        let mut out: Vec<StackPost> = Vec::new();
        for (k, &p) in marks.iter().enumerate() {
            let face = v3::yaw_to(p, fc);
            let (fx, fz) = (m::sin(face), m::cos(face));
            out.push(StackPost { pos: v3::copy(p), face, stack_side: stack_side_of(ux, uz, side, fx, fz), lead: k == 0, group: None });
        }
        return Some(out);
    }
    let sam = sam_pos(g);
    let held = crate::orders::squad_orders_held_on(g, d, men, None);
    let from = g.k.doortools.backblast_m + 0.4;
    let hall = crate::orders::squad_orders_hall_places(g, d, side, n, from, &mut |_g: &mut Game, q: V3| {
        v3::dist_xz(sam, q) < 0.7 || held.iter().any(|&h| v3::dist_xz(h, q) < 0.7)
    });
    if hall.len() as f64 == n {
        return Some(
            hall.iter()
                .enumerate()
                .map(|(k, P)| StackPost { pos: P.pos, face: P.face, stack_side: P.stack_side, lead: k == 0, group: None })
                .collect(),
        );
    }
    None
}

// ------------------------------------------------------------ once a frame

/// the frag row's key with the menu on the page that shows it: true when the press was the row's (doortools.ts menuKey)
pub fn door_tools_menu_key(g: &mut crate::game::Game, input: InputId) -> bool {
    let key = g.k.frag.order_key.clone();
    if g.orders.menu.is_none() || !crate::input::input_hit(g, input, &key) {
        return false;
    }
    let rows = g.orders.menu.as_ref().unwrap().rows.clone();
    let Some(i) = rows.iter().position(|&d| g.tab.orders_order_def[d].id == "fragClear") else { return false };
    press_off(g, input, &key);
    crate::orders::squad_orders_pick(g, Some(i as f64));
    true
}
/// the big slot's row on its own key with a door's menu up: on the first page the page is turned and the row given in one press (doortools.ts bigKey)
pub fn door_tools_big_key(g: &mut crate::game::Game, input: InputId) -> bool {
    let Some(M) = g.orders.menu.as_ref() else { return false };
    let is_door = matches!(M.target, crate::orders::OrderTarget::Door(_));
    let rows = M.rows.clone();
    if !g.door_tools.big_row.unwrap_or(false) || !is_door || !crate::input::input_hit(g, input, BIG_KEY) {
        return false;
    }
    let mut i = rows.iter().position(|&d| g.tab.orders_order_def[d].id == BIG_ORDER);
    if i.is_none() {
        let Some(o) = rows.iter().position(|&d| g.tab.orders_order_def[d].opens.as_deref() == Some(TOOLS_PAGE)) else { return false };
        crate::orders::squad_orders_pick(g, Some(o as f64));
        let now = g.orders.menu.as_ref().map(|M| M.rows.clone());
        i = now.and_then(|rows| rows.iter().position(|&d| g.tab.orders_order_def[d].id == BIG_ORDER));
        if i.is_none() {
            return false;
        }
    }
    press_off(g, input, BIG_KEY);
    crate::orders::squad_orders_pick(g, Some(i.unwrap() as f64));
    true
}
/// Sam's key: the word while anything of the men is waiting on it, else a set charge's trigger; never the drill's 'stack up' as well (doortools.ts input)
pub fn door_tools_input(g: &mut crate::game::Game, input: InputId) {
    use crate::nightbreach::{night_breach_drill_ops, night_breach_state, NightBreachState as NB};
    door_tools_sync(g);
    let key = g.k.doortools.blow_key.clone();
    if !crate::input::input_hit(g, input, &key) {
        return;
    }
    if g.pl[g.player].down {
        let set = g.door_tools.set.clone();
        if set.into_iter().any(|c| door_tools_key_is(g, c)) {
            press_off(g, input, &key);
        }
        return;
    }
    let four = crate::orders::squad_orders_big(g);
    let target = if four { g.orders.menu.as_ref().map(|M| M.target.clone()) } else { None };
    let row = if target.is_some() { menu_row_hot(g, &key) } else { -1 };
    let tap = g.k.doortools.word_tap;
    if !crate::orders::squad_orders_big(g) {
        let i = menu_row_hot(g, &key);
        if i >= 0 {
            press_off(g, input, &key);
            crate::orders::squad_orders_pick(g, Some(i as f64));
            return;
        }
        if g.time - g.door_tools.word_key_t < tap {
            press_off(g, input, &key);
            return;
        }
        let men = crate::orders::squad_orders_recipients(g, None, None);
        if door_tools_pair_word(g, &men) {
            press_off(g, input, &key);
            crate::orders::squad_orders_close(g);
            return;
        }
        let up = g.orders.menu.as_ref().map(|M| M.target.clone());
        let men = match up {
            Some(t) => recipients_of(g, &t),
            None => door_tools_word_men(g),
        };
        if !door_tools_stack_waits(g, &men).is_empty() || (g.time - g.door_tools.word_key_t >= tap && !door_tools_frag_held(g, &men).is_empty()) {
            press_off(g, input, &key);
            let p = sam_pos(g);
            crate::orders::squad_orders_close(g);
            if give_go(g, p, &men).is_none() {
                g.door_tools.word_key_t = g.time;
                trigger_free_keyed(g);
            }
            return;
        }
    }
    let men: Vec<GuardId> = if four {
        if row >= 0 {
            recipients_of(g, target.as_ref().unwrap())
        } else {
            door_tools_word_men(g)
        }
    } else {
        Vec::new()
    };
    let again = four && g.time - g.door_tools.word_key_t >= tap && !door_tools_frag_held(g, &men).is_empty();
    let waiting = !door_tools_waiting_any(g, &men).is_empty() || again;
    if waiting || row >= 0 {
        let st = night_breach_state(g);
        if !(waiting && (st == NB::Set || st == NB::Stacking) && any_shared(&night_breach_drill_ops(g), &men)) {
            press_off(g, input, &key);
        }
        if row >= 0 {
            crate::orders::squad_orders_pick(g, Some(row as f64));
        } else {
            let p = sam_pos(g);
            crate::orders::squad_orders_close(g);
            give_go(g, p, &men);
        }
        if waiting {
            g.door_tools.word_key_t = g.time;
        }
        return;
    }
    if four {
        let rec = crate::orders::squad_orders_recipients(g, None, None);
        if door_tools_word_early(g, &rec) {
            press_off(g, input, &key);
            crate::orders::squad_orders_close(g);
            return;
        }
    }
    let set = g.door_tools.set.clone();
    let mut mine = Vec::new();
    for c in set {
        if door_tools_key_is(g, c) {
            mine.push(c);
        }
    }
    if !mine.is_empty() {
        press_off(g, input, &key);
        let mut free = Vec::new();
        for c in mine {
            if !door_tools_on_pending_word(g, c) {
                free.push(c);
            }
        }
        if free.is_empty() {
            return;
        }
        let sam = sam_pos(g);
        let near = nearest_of(g, free, sam);
        door_tools_trigger(g, near);
        return;
    }
    if (four || !crate::orders::squad_orders_big(g))
        && night_breach_state(g) == NB::Staged
        && (g.time - g.door_tools.word_key_t < tap || {
            let drill = night_breach_drill_ops(g);
            let ops = g.door_tools.ops.clone();
            drill.iter().any(|&m| ops.iter().any(|&o| dop!(g, o).men.contains(&m)))
        })
    {
        press_off(g, input, &key);
    }
}
/// the charge of these nearest `p`: a stable sort by the distance to its door's frame, the first (doortools.ts input, nearest)
fn nearest_of(g: &mut Game, list: Vec<ChargeId>, p: V3) -> Option<ChargeId> {
    let mut by: Vec<(ChargeId, f64)> = list
        .into_iter()
        .map(|c| {
            let door = g.tab.doortools_set_charge[c].door;
            (c, v3::dist_xz(g.dr[door].frame_centre, p))
        })
        .collect();
    js::sort_by(&mut by, |a, b| a.1 - b.1);
    by.first().map(|x| x.0)
}
/// a set charge whose order is on a word that has not gone yet (doortools.ts onPendingWord)
pub fn door_tools_on_pending_word(g: &mut crate::game::Game, c_2: ChargeId) -> bool {
    let door = g.tab.doortools_set_charge[c_2].door;
    g.door_tools.ops.iter().any(|&o| {
        let O = &g.tab.doortools_door_op[o];
        O.kind == DoorToolKind::Charge && O.door == door && O.word.map_or(false, |w| g.tab.doortools_word[w].go1.is_nan())
    })
}
/// Sam's trigger as he pulls it: that charge and, on a linked door, every other set charge on them (doortools.ts trigger)
pub fn door_tools_trigger(g: &mut crate::game::Game, c_2: Option<ChargeId>) {
    let c = match c_2 {
        Some(c) => Some(c),
        None => door_tools_nearest(g, None),
    };
    let Some(c) = c else { return };
    let door = g.tab.doortools_set_charge[c].door;
    let also: Vec<ChargeId> = if g.door_tools.linked.contains(&door) {
        g.door_tools.set.iter().copied().filter(|&o| o != c && g.door_tools.linked.contains(&g.tab.doortools_set_charge[o].door)).collect()
    } else {
        Vec::new()
    };
    door_tools_blow(g, Some(c));
    for o in also {
        door_tools_blow(g, Some(o));
    }
}
/// (doortools.ts nearest)
pub fn door_tools_nearest(g: &mut crate::game::Game, of: Option<&mut dyn FnMut(&mut crate::game::Game, ChargeId) -> bool>) -> Option<ChargeId> {
    let mut of = of;
    let p = sam_pos(g);
    let set = g.door_tools.set.clone();
    let mut list = Vec::new();
    for c in set {
        let keep = match of.as_mut() {
            Some(f) => f(g, c),
            None => true,
        };
        if keep {
            list.push(c);
        }
    }
    nearest_of(g, list, p)
}
/// once a tick, before the orders pose the men: every door order's stage (doortools.ts tick)
pub fn door_tools_tick(g: &mut crate::game::Game) {
    door_tools_sync(g);
    door_tools_sling_tick(g);
    crate::bigkit::hammer_out_tick(g, &mut |g: &mut Game, gd: GuardId| {
        g.door_tools.ops.iter().any(|&o| {
            let O = &g.tab.doortools_door_op[o];
            O.phase == Phase::Kick
                && O.kick.as_ref().and_then(|k| k.gun.as_ref()).map_or(false, |gun| gun.by == gd && gun.hammer.unwrap_or(false))
        })
    });
    door_tools_sam_tick(g);
    if g.door_tools.roster.is_empty() {
        if !g.door_tools.set.is_empty() {
            door_tools_tick_set(g);
        }
        return;
    }
    if g.door_tools.seen.is_none() {
        let s = crate::floorseen::floor_seen_new(g);
        g.door_tools.seen = Some(s);
    }
    let mut roster = g.door_tools.roster.clone();
    let seen_m = g.k.doortools.lead_seen_m;
    crate::floorseen::floor_seen_tick(g, &mut roster, seen_m);
    door_tools_keep_sectors(g);
    door_tools_test_stacks(g);
    door_tools_walk_tried(g);
    for op in g.door_tools.watches.clone() {
        if !door_tools_other_doors(g, op, Some(true), None, None) {
            splice_out(&mut g.door_tools.watches, &op);
        }
    }
    door_tools_tick_set(g);
    for op in g.door_tools.ops.clone() {
        tick_op(g, op);
    }
    if !g.door_tools.words.is_empty() {
        door_tools_word_tick(g);
    }
    let t = g.time;
    for op in g.door_tools.ops.clone() {
        let O = &dop!(g, op);
        if let Some(gate) = O.gate {
            if O.phase == Phase::Stack && !O.ready_t.is_nan() && g.tab.doortools_door_gate[gate].ops.iter().any(|&o| o != op && dop!(g, o).open_t == t) {
                door_tools_step(g, op);
            }
        }
    }
}
/// one order's turn in the tick (doortools.ts tick, `for (const op of [...this.ops])`)
fn tick_op(g: &mut Game, op: DoorOpId) {
    let mut men = Vec::new();
    for gd in dop!(g, op).men.clone() {
        if is_dead(g, gd) {
            continue;
        }
        // (`S.of(gd) === op.mine.get(gd)`: null is never undefined, so a man with neither is off the order)
        let keep = match (crate::orders::squad_orders_of(g, gd), dop!(g, op).mine.get(gd).copied()) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        };
        if keep {
            men.push(gd);
        }
    }
    dop!(g, op).men = men.clone();
    let (door, side) = (dop!(g, op).door, dop!(g, op).side);
    let width = g.dr[door].def.width;
    for &gd in &men {
        if dop!(g, op).crossed.has(gd) {
            continue;
        }
        let p = pos_of(g, gd);
        if off_of(g, door, p, side) < -0.15 && along_of(g, door, p) > -0.6 && along_of(g, door, p) < width + 0.6 {
            let t = g.time;
            dop!(g, op).crossed.set(gd, t);
            let text = format!("cross:{}", g.gd[gd].callsign);
            door_tools_beat(g, op, &text);
        }
    }
    if men.is_empty() {
        door_tools_end(g, op, "nobody left on it");
        return;
    }
    let lead = dop!(g, op).lead;
    if !men.contains(&lead) {
        let L = dop!(g, op).post.get(lead).copied();
        let next = match L {
            Some(L) if men.len() > 1 => {
                let mut by: Vec<(GuardId, f64)> = Vec::new();
                for &a in &men {
                    let at = match dop!(g, op).post.get(a) {
                        Some(P) => P.pos,
                        None => pos_of(g, a),
                    };
                    by.push((a, v3::dist_xz(at, L.pos)));
                }
                js::sort_by(&mut by, |a, b| a.1 - b.1);
                by[0].0
            }
            _ => men[0],
        };
        dop!(g, op).lead = next;
        door_tools_new_lead(g, op);
    }
    let open_t = dop!(g, op).open_t;
    if !open_t.is_nan() && g.time - open_t < g.k.doortools.hold_open && crate::doors::door_quiet_t(g, door) > -1.0 && is_open(g, door) {
        crate::doors::door_set_quiet_t(g, door, -1.0);
    }
    door_tools_threat_note(g, op);
    if is_pre(dop!(g, op).phase) {
        door_tools_spare_rear(g, op);
    } else {
        for gd in dop!(g, op).men.clone() {
            let Some(a) = dop!(g, op).mine.get(gd).copied() else { continue };
            let O = &dop!(g, op);
            let spare = O.thr.is_some() && O.phase != Phase::Enter && O.rear.has(gd);
            let from = O.from.get(gd).copied();
            let A = &mut g.tab.orders_assignment[a];
            A.spare = Some(spare);
            if spare && A.from.is_none() {
                A.from = from;
            }
        }
    }
    door_tools_step(g, op);
    if g.door_tools.ops.contains(&op) && is_pre(dop!(g, op).phase) {
        door_tools_spare_rear(g, op);
    }
}
/// (doortools.ts arrived)
pub fn door_tools_arrived(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId) -> bool {
    match dop!(g, op).mine.get(gd) {
        Some(&a) => g.tab.orders_assignment[a].arrived,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn splice_out_takes_the_last_when_the_thing_is_gone() {
        let mut l = vec![1, 2, 3];
        splice_out(&mut l, &2);
        assert_eq!(l, vec![1, 3]);
        splice_out(&mut l, &9);
        assert_eq!(l, vec![1]);
    }
    #[test]
    fn words_joined_once_each_in_first_order() {
        let w = vec![String::from("red"), String::from("blue"), String::from("red")];
        assert_eq!(unique_joined(w), "red + blue");
        assert_eq!(unique_joined(Vec::new()), "");
    }
    #[test]
    fn pre_phases_are_the_five() {
        for p in [Phase::Stack, Phase::Ready, Phase::Place, Phase::Standoff, Phase::Armed] {
            assert!(is_pre(p));
        }
        for p in [Phase::Kick, Phase::Toss, Phase::Fuse, Phase::Wait, Phase::Enter, Phase::Slice, Phase::Held, Phase::Thr, Phase::Pocket] {
            assert!(!is_pre(p));
        }
    }
    #[test]
    fn stack_side_has_the_typescripts_sign() {
        // door along +x (th = 0: ux 1, uz 0), face +1: a man facing back along -x past the latch
        assert_eq!(stack_side_of(1.0, 0.0, 1.0, -1.0, 0.0), 1.0);
        assert_eq!(stack_side_of(1.0, 0.0, 1.0, 1.0, 0.0), -1.0);
        // exactly zero counts as +1 (`>= 0`)
        assert_eq!(stack_side_of(1.0, 0.0, 1.0, 0.0, 1.0), 1.0);
    }
}
