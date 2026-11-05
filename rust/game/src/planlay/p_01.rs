//! src/game/planlay.ts lines 1 to 187
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A deep copy of a plan. (planlay.ts copy)
pub fn copy(a: &[crate::planlay::LaidOrder]) -> Vec<crate::planlay::LaidOrder> {
    a.to_vec()
}

/// The key that sends the plan: the word's own. (planlay.ts layKey)
pub fn lay_key(g: &mut crate::game::Game) -> String {
    g.k.doortools.blow_key.clone()
}

/// Can that row be laid: an order, not a page, not a free row, not the word. (planlay.ts layable)
pub fn layable(g: &mut crate::game::Game, d: OrdersOrderDefId) -> bool {
    let def = &g.tab.orders_order_def[d];
    let opens = def.opens.as_deref().map_or(false, |s| !s.is_empty());
    !opens && !def.free.unwrap_or(false) && !g.m.planlay.word_rows.has(def.id.clone())
}

/// Is something laid and waiting on the key. (planlay.ts layPending)
pub fn lay_pending(g: &mut crate::game::Game) -> bool {
    for i in 0..g.orders.lay.laid.len() {
        let o = g.orders.lay.laid[i].clone();
        if !lay_men(g, &o).is_empty() {
            return true;
        }
    }
    false
}

/// A man who is down is off the plan; an order nobody is left on is gone. (planlay.ts layPrune)
pub fn lay_prune(g: &mut crate::game::Game) {
    let s = crate::orders::squad_orders_squad(g);
    if g.orders.lay.laid.is_empty() || s.is_empty() {
        return;
    }
    let mut up: JsSet<String> = JsSet::new();
    for m in &s {
        if g.gd[*m].state != crate::game::GuardState::Dead {
            up.add(g.gd[*m].callsign.clone());
        }
    }
    let mut n = 0i32;
    for o in g.orders.lay.laid.iter_mut() {
        let was = o.who.len() as i32;
        o.who.retain(|c| up.has(c.clone()));
        n += was - o.who.len() as i32;
    }
    if n != 0 {
        g.orders.lay.laid.retain(|o| !o.who.is_empty());
        g.stats.planlay.dropped += n;
    }
}

/// The start a plan is kept for: the scenario up, the cast, the mark Sam stands on. (planlay.ts startKey)
pub fn start_key(g: &mut crate::game::Game) -> String {
    let c = g.pl[g.player].ch;
    let p = g.ch[c].pos;
    let scenario = match crate::scenario_run::active_scenario(g) {
        Some(a) => a.as_str(),
        None => "none",
    };
    let cast: Vec<String> = crate::orders::squad_orders_squad(g).iter().map(|m| g.gd[*m].callsign.clone()).collect();
    format!("{}|{}|{},{}", scenario, cast.join(","), round_words(m::round(p[0] * 2.0)), round_words(m::round(p[2] * 2.0)))
}

/// The living men of the squad a laid order names. (planlay.ts layMen)
pub fn lay_men(g: &mut crate::game::Game, o: &crate::planlay::LaidOrder) -> Vec<GuardId> {
    let s = crate::orders::squad_orders_squad(g);
    let mut men = Vec::new();
    for c in &o.who {
        if let Some(m) = s.iter().copied().find(|m| g.gd[*m].callsign == *c && g.gd[*m].state != crate::game::GuardState::Dead) {
            men.push(m);
        }
    }
    men
}

/// Where a laid order is on the floor (None: fall in, hold). (planlay.ts layPoint)
pub fn lay_point(g: &mut crate::game::Game, o: &crate::planlay::LaidOrder) -> Option<V3> {
    if o.kind == crate::planlay::LaidOrderKind::Door {
        let d = door_at(g, o.door)?;
        let c = g.dr[d].frame_centre;
        return Some([c[0], 0.0, c[2]]);
    }
    if o.kind == crate::planlay::LaidOrderKind::Floor {
        Some([o.at[0], 0.0, o.at[2]])
    } else {
        None
    }
}

/// A laid order's target as the menu would hand it to give() now (None: its door or man is gone). (planlay.ts targetOf)
pub fn target_of(g: &mut crate::game::Game, o: &crate::planlay::LaidWhere) -> Option<crate::planlay::TargetOf> {
    use crate::orders::{OrderTargetDoor, OrderTargetFloor, OrderTargetNone};
    use crate::planlay::{LaidWhereFound, LaidWhereKind, TargetOf};
    let at: V3 = [o.at[0], 0.0, o.at[2]];
    if o.kind == LaidWhereKind::Door {
        let d = door_at(g, o.door)?;
        let c = g.dr[d].frame_centre;
        let ch = g.pl[g.player].ch;
        let p = g.ch[ch].pos;
        let side = crate::orders::side_of(g, d, p);
        return Some(TargetOf::Door(OrderTargetDoor { at: [c[0], 0.0, c[2]], door: d, side }));
    }
    if o.kind == LaidWhereKind::None {
        let man = match o.man.as_deref() {
            None | Some("") => return Some(TargetOf::None(OrderTargetNone { at, man: None })),
            Some(man) => man,
        };
        let s = crate::orders::squad_orders_squad(g);
        let m = s.iter().copied().find(|x| g.gd[*x].callsign == man && g.gd[*x].state != crate::game::GuardState::Dead)?;
        let p = g.ch[g.gd[m].ch].pos;
        return Some(TargetOf::None(OrderTargetNone { at: [p[0], 0.0, p[2]], man: Some(m) }));
    }
    // a wall's or a thing's own order is found again by the point ON it
    if let (Some(LaidWhereFound::Wall), Some(pick)) = (o.found, o.pick) {
        let w = crate::wallcharge::wall_at(g, pick, None)?;
        return Some(TargetOf::Floor(OrderTargetFloor { at, wall: Some(w), thing: None }));
    }
    if let (Some(LaidWhereFound::Thing), Some(pick)) = (o.found, o.pick) {
        let w = crate::workthings::work_near(g, pick)?;
        return match w.target {
            crate::orders::OrderTarget::Floor(f) if f.thing.is_some() => Some(TargetOf::Floor(f)),
            _ => None,
        };
    }
    Some(TargetOf::Floor(OrderTargetFloor { at, wall: None, thing: None }))
}

/// `target_of`, for the other modules. (planlay.ts layTarget)
pub fn lay_target(g: &mut crate::game::Game, o: &crate::planlay::LaidWhere) -> Option<crate::planlay::LayTarget> {
    use crate::planlay::{LayTarget, TargetOf};
    target_of(g, o).map(|t| match t {
        TargetOf::Floor(x) => LayTarget::Floor(x),
        TargetOf::Door(x) => LayTarget::Door(x),
        TargetOf::None(x) => LayTarget::None(x),
        TargetOf::Foe(x) => LayTarget::Foe(x),
    })
}

/// Those callsigns off every laid order; one with nobody left is gone. Returns how many orders lost a name. (planlay.ts strike)
pub fn strike(l: &mut crate::planlay::LayState, names: &JsSet<String>) -> f64 {
    let mut n = 0.0;
    for o in l.laid.iter_mut() {
        let was = o.who.len();
        o.who.retain(|c| !names.has(c.clone()));
        if o.who.len() != was {
            n += 1.0;
        }
    }
    l.laid.retain(|o| !o.who.is_empty());
    n
}

/// The menu's row `d`, picked with 'wait for my word' on: laid for `men` on `t`. None = laid, else why not. (planlay.ts layOrder)
pub fn lay_order(g: &mut crate::game::Game, d: OrdersOrderDefId, t: &crate::orders::OrderTarget, men: &[GuardId]) -> Option<String> {
    use crate::planlay::{LaidOrder, LaidOrderFound, LaidOrderKind};
    let kind = match t {
        crate::orders::OrderTarget::Foe(_) => return Some(String::from("not here.")),
        crate::orders::OrderTarget::Floor(_) => crate::orders::TargetKind::Floor,
        crate::orders::OrderTarget::Door(_) => crate::orders::TargetKind::Door,
        crate::orders::OrderTarget::None(_) => crate::orders::TargetKind::None,
    };
    if !g.tab.orders_order_def[d].targets.contains(&kind) {
        return Some(String::from("not here."));
    }
    let mut asked: Vec<GuardId> = Vec::new();
    for m in men.iter().copied() {
        if g.gd[m].state != crate::game::GuardState::Dead && crate::orders::squad_orders_mine(g, m) {
            asked.push(m);
        }
    }
    if asked.is_empty() {
        crate::game::game_msg(g, "nobody to take that order");
        return Some(String::from("nobody to take it"));
    }
    if g.tab.orders_order_def[d].only.is_some() {
        let o = seam_order_def_only(g, d, &asked, t);
        if !o.is_empty() {
            asked = o;
        }
    }
    let tm = crate::orders::squad_orders_for_men(g, t, &asked);
    let mut cant: Vec<(GuardId, String)> = Vec::new();
    // as give() has it: a man who cannot do his part says so and stays on the plan he was on
    let all: Vec<GuardId> = if g.tab.orders_order_def[d].can.is_some() {
        let mut able = Vec::new();
        for m in asked.iter().copied() {
            match seam_order_def_can(g, d, m, &tm) {
                None => able.push(m),
                Some(w) => cant.push((m, w)),
            }
        }
        able
    } else {
        asked.clone()
    };
    let no = if !all.is_empty() { crate::orders::squad_orders_why_not(g, d, &tm, Some(&all)) } else { Some(cant[0].1.clone()) };
    if let Some(no) = no.filter(|w| !w.is_empty()) {
        let who = if !all.is_empty() { all[0] } else { cant[0].0 };
        crate::game::game_say(g, who, &no, None, None, None);
        g.stats.planlay.refused += 1;
        return Some(no);
    }
    for (m, w) in &cant {
        crate::game::game_say(g, *m, w, None, None, None);
    }
    let asked = all;
    let mut names: JsSet<String> = JsSet::new();
    for m in &asked {
        names.add(g.gd[*m].callsign.clone());
    }
    if js::truthy(strike(&mut g.orders.lay, &names)) {
        g.stats.planlay.replaced += 1;
    }
    let id = g.tab.orders_order_def[d].id.clone();
    let at = match t {
        crate::orders::OrderTarget::Floor(x) => x.at,
        crate::orders::OrderTarget::Door(x) => x.at,
        crate::orders::OrderTarget::None(x) => x.at,
        crate::orders::OrderTarget::Foe(x) => x.at,
    };
    let mut o = LaidOrder {
        id: id.clone(),
        who: asked.iter().map(|m| g.gd[*m].callsign.clone()).collect(),
        kind: LaidOrderKind::None,
        at: [at[0], 0.0, at[2]],
        door: None,
        man: None,
        found: None,
        pick: None,
        t: g.time,
    };
    match t {
        crate::orders::OrderTarget::Door(x) => {
            o.kind = LaidOrderKind::Door;
            o.door = Some(g.doors.list.iter().position(|e| *e == x.door).map_or(-1.0, |i| i as f64));
        }
        crate::orders::OrderTarget::None(x) => {
            o.kind = LaidOrderKind::None;
            if let Some(m) = x.man {
                o.man = Some(g.gd[m].callsign.clone());
            }
        }
        crate::orders::OrderTarget::Floor(x) => {
            o.kind = LaidOrderKind::Floor;
            if let Some(thing) = &x.thing {
                o.found = Some(LaidOrderFound::Thing);
                o.pick = Some(thing.face);
            } else if let Some(wall) = &x.wall {
                // the floor's own rows on a spot under a wall are the spot's: no wall to find again
                if id != "move" && id != "cover" && id != "hold" && id != "fallIn" {
                    o.found = Some(LaidOrderFound::Wall);
                    o.pick = Some(crate::wallcharge::wall_pt(g, wall, 0.0, 0.0, None));
                }
            }
        }
        crate::orders::OrderTarget::Foe(_) => {}
    }
    g.orders.lay.laid.push(o);
    g.stats.planlay.laid += 1;
    let who = crate::orders::squad_orders_who_words(g, Some(&asked));
    let label = crate::orders::squad_orders_label_of(g, d, &tm);
    let text = format!("you: {} — wait · {}", who, without_count(&label));
    crate::game::game_msg(g, &text);
    None
}

/// One laid order as `laySend` works it out before anything is given.
struct Item {
    o: crate::planlay::LaidOrder,
    men: Vec<GuardId>,
    t: Option<crate::orders::OrderTarget>,
    d: Option<OrdersOrderDefId>,
    tool: bool,
}

/// The word's key with a plan laid: every laid order given, in the order laid. Returns how many were given. (planlay.ts laySend)
pub fn lay_send(g: &mut crate::game::Game) -> f64 {
    if g.orders.lay.laid.is_empty() {
        return 0.0;
    }
    lay_prune(g);
    if g.orders.lay.laid.is_empty() {
        return 0.0;
    }
    if !g.orders.lay.started {
        g.orders.lay.kept = Some(copy(&g.orders.lay.laid));
        g.orders.lay.kept_for = g.orders.lay.try_key.clone();
    }
    let plan = core::mem::take(&mut g.orders.lay.laid);
    g.orders.lay.sending = true;
    let mut n = 0.0;
    // a door tool the four can wait on is given ON THE WORD, unless every one finds its men already on its door: then this press IS the word
    let mut items: Vec<Item> = Vec::with_capacity(plan.len());
    for o in plan {
        let men = lay_men(g, &o);
        let t = target_of(g, &where_of(&o)).map(order_target);
        let d = g.orders.defs.iter().copied().find(|x| g.tab.orders_order_def[*x].id == o.id);
        let on_word = d.map_or(false, |d| g.tab.orders_order_def[d].on_word.unwrap_or(false));
        let tool = on_word && crate::orders::squad_orders_big(g) && matches!(t, Some(crate::orders::OrderTarget::Door(_)));
        items.push(Item { o, men, t, d, tool });
    }
    let mut every_there = true;
    for x in &items {
        if x.tool && !x.men.is_empty() && !there(g, x) {
            every_there = false;
            break;
        }
    }
    let wait = !every_there;
    for x in &items {
        let t = match (&x.t, x.d) {
            (Some(t), Some(_)) if !x.men.is_empty() => t,
            _ => {
                g.stats.planlay.dropped += 1;
                continue;
            }
        };
        let tail = g.orders.log.last().map(|r| r.serial);
        let who = give_who(g.orders.who);
        let opts = crate::orders::GiveOpts { ack: None, on_word: Some(x.tool && wait), men: Some(x.men.clone()) };
        let no = crate::orders::squad_orders_give(g, &x.o.id, Some(t), Some(who), Some(&opts));
        if no.is_some() {
            g.stats.planlay.refused += 1;
            continue;
        }
        if let Some(row) = g.orders.log.last_mut() {
            if Some(row.serial) != tail {
                row.laid = Some(true);
            }
        }
        n += 1.0;
        g.stats.planlay.sent += 1;
    }
    g.orders.lay.sending = false;
    g.orders.lay.started = true;
    n
}

/// An order was given at once: its men are off the plan. (planlay.ts layGiven)
pub fn lay_given(g: &mut crate::game::Game, men: &[GuardId]) {
    if g.orders.lay.sending || g.orders.lay.laid.is_empty() {
        return;
    }
    let mut names: JsSet<String> = JsSet::new();
    for m in men {
        names.add(g.gd[*m].callsign.clone());
    }
    if js::truthy(strike(&mut g.orders.lay, &names)) {
        g.stats.planlay.struck += 1;
    }
}

/// A fresh cast: nothing is laid, but the plan kept from the last try of these men here is laid again. (planlay.ts layFresh)
pub fn lay_fresh(g: &mut crate::game::Game) {
    g.orders.lay.sending = false;
    let (restored, now) = (g.orders.lay.restored_t, g.time);
    if restored >= 0.0 && now >= restored && now - restored < 0.1 {
        return;
    }
    g.orders.lay.restored_t = -1.0;
    g.orders.lay.started = false;
    g.orders.lay.laid = Vec::new();
    if crate::orders::squad_orders_squad(g).is_empty() {
        return;
    }
    g.orders.lay.try_key = start_key(g);
    let Some(kept) = g.orders.lay.kept.as_ref() else { return };
    if g.orders.lay.kept_for != g.orders.lay.try_key {
        g.orders.lay.kept = None;
        g.orders.lay.kept_for = String::new();
        return;
    }
    let mut laid = copy(kept);
    for o in laid.iter_mut() {
        o.t = now;
    }
    g.orders.lay.laid = laid;
    g.stats.planlay.relaid += 1;
}

/// The plan thrown away, the kept one with it. (planlay.ts layClear)
pub fn lay_clear(g: &mut crate::game::Game) {
    let l = &mut g.orders.lay;
    l.laid = Vec::new();
    l.kept = None;
    l.kept_for = String::new();
    l.started = false;
    l.sending = false;
    l.restored_t = -1.0;
}

/// The HUD's line for the key while a plan waits. (planlay.ts layNote)
pub fn lay_note(g: &mut crate::game::Game, key: &str) -> Vec<crate::planlay::LayNoteItem> {
    if !lay_pending(g) {
        return Vec::new();
    }
    let mut men: JsSet<GuardId> = JsSet::new();
    for i in 0..g.orders.lay.laid.len() {
        let o = g.orders.lay.laid[i].clone();
        for m in lay_men(g, &o) {
            men.add(m);
        }
    }
    if men.is_empty() {
        return Vec::new();
    }
    let who = crate::orders::squad_orders_who_words(g, Some(&men.keys()));
    vec![crate::planlay::LayNoteItem { key: String::from(key), text: format!("go · {} on the plan", who) }]
}

/// The plan in one line, for the debugger. (planlay.ts laySummary)
pub fn lay_summary(g: &mut crate::game::Game) -> String {
    use crate::planlay::LaidOrderKind;
    let mut rows: Vec<String> = Vec::new();
    for i in 0..g.orders.lay.laid.len() {
        let o = g.orders.lay.laid[i].clone();
        let place = match o.kind {
            LaidOrderKind::Door => {
                let name = match door_at(g, o.door) {
                    Some(d) => g.dr[d].def.name.clone(),
                    None => String::from("?"),
                };
                format!(" door {}", name)
            }
            LaidOrderKind::Floor => format!(" {},{}", to_fixed_1(o.at[0]), to_fixed_1(o.at[2])),
            LaidOrderKind::None => String::new(),
        };
        rows.push(format!("{}: {}{}", o.who.join("+"), o.id, place));
    }
    let head = if rows.is_empty() { String::from("nothing laid") } else { rows.join(" · ") };
    match &g.orders.lay.kept {
        Some(kept) => format!("{} · kept {}", head, kept.len()),
        None => head,
    }
}

/// `g.doors.list[o.door ?? -1]`: nothing where the place is off the list.
fn door_at(g: &mut Game, door: Option<f64>) -> Option<DoorId> {
    let n = door.unwrap_or(-1.0);
    if n < 0.0 || n != m::floor(n) {
        return None;
    }
    g.doors.list.get(n as usize).copied()
}

/// A laid order's place, alone (the TypeScript's `Pick` is the object itself).
fn where_of(o: &crate::planlay::LaidOrder) -> crate::planlay::LaidWhere {
    use crate::planlay::{LaidOrderFound, LaidOrderKind, LaidWhere, LaidWhereFound, LaidWhereKind};
    LaidWhere {
        kind: match o.kind {
            LaidOrderKind::None => LaidWhereKind::None,
            LaidOrderKind::Door => LaidWhereKind::Door,
            LaidOrderKind::Floor => LaidWhereKind::Floor,
        },
        at: o.at,
        pick: o.pick,
        door: o.door,
        man: o.man.clone(),
        found: o.found.map(|f| match f {
            LaidOrderFound::Wall => LaidWhereFound::Wall,
            LaidOrderFound::Thing => LaidWhereFound::Thing,
        }),
    }
}

/// The order book's own word for a target.
fn order_target(t: crate::planlay::TargetOf) -> crate::orders::OrderTarget {
    use crate::planlay::TargetOf;
    match t {
        TargetOf::Floor(x) => crate::orders::OrderTarget::Floor(x),
        TargetOf::Door(x) => crate::orders::OrderTarget::Door(x),
        TargetOf::None(x) => crate::orders::OrderTarget::None(x),
        TargetOf::Foe(x) => crate::orders::OrderTarget::Foe(x),
    }
}

/// `S.who` as give() takes it.
fn give_who(w: crate::orders::Who) -> crate::orders::SquadOrdersGiveWho {
    use crate::orders::{SquadOrdersGiveWho, Who};
    match w {
        Who::Num(n) => SquadOrdersGiveWho::Num(n),
        Who::All => SquadOrdersGiveWho::All,
        Who::Red => SquadOrdersGiveWho::Red,
        Who::Blue => SquadOrdersGiveWho::Blue,
    }
}

/// Every man of this door tool is already on its door, and the door's men are set. (planlay.ts laySend there)
fn there(g: &mut Game, x: &Item) -> bool {
    let Some(crate::orders::OrderTarget::Door(t)) = &x.t else { return false };
    let Some(w) = crate::doortools::door_tools_waits_on(g, &x.men, t.door) else { return false };
    w.set && x.men.iter().all(|m| w.men.contains(m))
}

/// `label.replace(/ · (\d+ left|uses .+)$/, '')`: the first place where the rest of the line is a count of what is left.
fn without_count(label: &str) -> &str {
    for (i, _) in label.char_indices() {
        if let Some(rest) = label[i..].strip_prefix(" · ") {
            let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
            let left = digits > 0 && &rest[digits..] == " left";
            let uses = rest.strip_prefix("uses ").map_or(false, |r| !r.is_empty() && !r.contains(['\n', '\r', '\u{2028}', '\u{2029}']));
            if left || uses {
                return &label[..i];
            }
        }
    }
    label
}

/// `${x}` of a whole number below 1e21: −0 is written "0".
fn round_words(x: f64) -> String {
    if x == 0.0 { String::from("0") } else { format!("{}", x) }
}

/// `x.toFixed(1)` below 1e21: on a tie the larger n, the sign apart, and −0 has none.
fn to_fixed_1(x: f64) -> String {
    if x.is_nan() {
        return String::from("NaN");
    }
    let (sign, a) = if x < 0.0 { ("-", -x) } else { ("", m::abs(x)) };
    let q = a * 4.0;
    if q == m::floor(q) && js::rem(q, 2.0) == 1.0 {
        // an exact tie (k/4, k odd), where format! would go to the even digit
        let n = m::ceil(a * 10.0);
        return format!("{}{}.{}", sign, m::floor(n / 10.0), js::rem(n, 10.0));
    }
    format!("{}{:.1}", sign, a)
}

/// `d.only(S, men, t)`: OrderOnly has its arms, but no function a part can call runs them. (planlay.ts:107)
fn seam_order_def_only(g: &mut Game, d: OrdersOrderDefId, men: &[GuardId], t: &crate::orders::OrderTarget) -> Vec<GuardId> {
    todo!("needs: a function of crate::orders that runs OrderDef.only's arm")
}

/// `d.can(S, m, t)`: None is `true`, Some is the man's word for why not; OrderCan has its arms, but no function a part can call runs them. (planlay.ts:110)
fn seam_order_def_can(g: &mut Game, d: OrdersOrderDefId, m: GuardId, t: &crate::orders::OrderTarget) -> Option<String> {
    todo!("needs: a function of crate::orders that runs OrderDef.can's arm")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planlay::{LaidOrder, LayState};

    fn order(who: &[&str]) -> LaidOrder {
        LaidOrder { who: who.iter().map(|c| String::from(*c)).collect(), ..LaidOrder::zeroed() }
    }

    #[test]
    fn strike_counts_orders_touched_and_drops_the_empty() {
        let mut l = LayState::zeroed();
        l.laid = vec![order(&["a", "b"]), order(&["c"]), order(&["d"])];
        let mut names: JsSet<String> = JsSet::new();
        names.add(String::from("a"));
        names.add(String::from("c"));
        assert_eq!(strike(&mut l, &names), 2.0);
        assert_eq!(l.laid.len(), 2);
        assert_eq!(l.laid[0].who, vec![String::from("b")]);
        assert_eq!(l.laid[1].who, vec![String::from("d")]);
    }

    #[test]
    fn the_label_loses_only_a_count_at_its_end() {
        // the outputs of the TypeScript's own regular expression, run under bun
        for (a, b) in [
            ("flash · 3 left", "flash"),
            ("flash · uses a can", "flash"),
            ("a · b · 12 left", "a · b"),
            ("a · uses x · 2 left", "a"),
            ("a · · 2 left", "a ·"),
            ("flash · left", "flash · left"),
            ("flash · 3 left now", "flash · 3 left now"),
            ("flash · uses ", "flash · uses "),
            ("flash · uses a\nb", "flash · uses a\nb"),
            ("plain", "plain"),
        ] {
            assert_eq!(without_count(a), b, "{a}");
        }
    }

    #[test]
    fn to_fixed_is_the_language_s() {
        // the outputs of the language's own toFixed(1), run under bun
        for (x, s) in [
            (0.25, "0.3"), (0.75, "0.8"), (1.25, "1.3"), (-0.25, "-0.3"), (-1.75, "-1.8"), (2.5, "2.5"), (0.05, "0.1"), (0.15, "0.1"),
            (-0.0, "0.0"), (0.0, "0.0"), (-0.01, "-0.0"), (12.34, "12.3"), (1234.25, "1234.3"), (9.96, "10.0"), (9.75, "9.8"),
        ] {
            assert_eq!(to_fixed_1(x), s, "{x}");
        }
        assert_eq!(to_fixed_1(f64::NAN), "NaN");
    }

    #[test]
    fn a_round_number_is_written_as_the_language_does() {
        assert_eq!(round_words(-0.0), "0");
        assert_eq!(round_words(0.0), "0");
        assert_eq!(round_words(3.0), "3");
        assert_eq!(round_words(-14.0), "-14");
    }
}
