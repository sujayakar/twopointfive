//! src/game/orders.ts lines 774 to 1098
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

fn kind_of(t: &OrderTarget) -> TargetKind {
    match t {
        OrderTarget::Floor(_) => TargetKind::Floor,
        OrderTarget::Door(_) => TargetKind::Door,
        OrderTarget::None(_) => TargetKind::None,
        OrderTarget::Foe(_) => TargetKind::Foe,
    }
}
fn at_of(t: &OrderTarget) -> V3 {
    match t {
        OrderTarget::Floor(x) => x.at,
        OrderTarget::Door(x) => x.at,
        OrderTarget::None(x) => x.at,
        OrderTarget::Foe(x) => x.at,
    }
}
/// `recipients(t)` with its `who` left to the standing choice
fn recipients_of(g: &mut Game, t: &OrderTarget) -> Vec<GuardId> {
    squad_orders_recipients(g, Some(t), None)
}
fn on(b: Option<bool>) -> bool {
    b.unwrap_or(false)
}
/// `if (word)` of an optional string
fn word(s: &Option<String>) -> bool {
    s.as_ref().map_or(false, |x| !x.is_empty())
}
fn alive(g: &Game, gd: GuardId) -> bool {
    g.gd[gd].state != crate::game::GuardState::Dead
}
fn player_ch(g: &Game) -> CharId {
    g.pl[g.player].ch
}
fn press_delete(g: &mut Game, input: InputId, code: &str) -> bool {
    g.tab.input[input].pressed.delete(String::from(code))
}
fn press_has(g: &mut Game, input: InputId, code: &str) -> bool {
    g.tab.input[input].pressed.has(String::from(code))
}
/// `input.clicked &= ~bit`
fn unclick(g: &mut Game, input: InputId, bit: i32) {
    let c = g.tab.input[input].clicked;
    g.tab.input[input].clicked = (js::to_i32(c) & !bit) as f64;
}
/// a row of the list by a number, as `rows[i]`
fn row_at(rows: &[OrdersOrderDefId], i: f64) -> Option<OrdersOrderDefId> {
    if i >= 0.0 && i == m::floor(i) && (i as usize) < rows.len() { Some(rows[i as usize]) } else { None }
}
/// `v.input = input; v.cam = cam; v.canvas = canvas`
fn set_view(g: &mut Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View) {
    g.orders.view.input = Some(input);
    g.orders.view.cam = Some(cam);
    g.orders.view.canvas = Some(canvas);
}
/// the open menu's target as its men will work it (menuTarget, on the menu in hand)
fn menu_target_of(m: &Menu) -> OrderTarget {
    match (&m.target, m.men_side) {
        (OrderTarget::Door(x), Some(side)) => OrderTarget::Door(OrderTargetDoor { side, ..*x }),
        (t, _) => t.clone(),
    }
}
/// `label.replace(/ · (\d+ left|uses .+)$/, '')`
fn drop_stock_word(s: &str) -> &str {
    const CUT: &str = " · ";
    for (i, _) in s.match_indices(CUT) {
        let rest = &s[i + CUT.len()..];
        let digits = rest.bytes().take_while(|b| b.is_ascii_digit()).count();
        let left = digits > 0 && &rest[digits..] == " left";
        let uses = rest.strip_prefix("uses ").map_or(false, |u| !u.is_empty() && !u.contains(|c| matches!(c, '\n' | '\r' | '\u{2028}' | '\u{2029}')));
        if left || uses { return &s[..i]; }
    }
    s
}

// ---- what skeleton 5 still cannot reach ----
/// `b.owner`. Once the table of Box rows is there, the body is that row's field
fn seam_box_owner(g: &mut Game, b: BoxId) -> f64 { g.tab.boxes[b].owner }
/// `b.flags`. As seam_box_owner
fn seam_box_flags(g: &mut Game, b: BoxId) -> f64 { g.tab.boxes[b].flags }
// BRING-UP: THE DOOR ROWS' CLOSURES (doortools.ts:808-846 the constructor's rows, :864 thrRows on the page's `outsidePlain`, :908-938 sync's `go` and the big
// slot's row). The skeleton has their arms and no named function for their bodies, so the first door order given stopped here (room-clear-four tick 1009, 'bangClear').
// SKELETON-GAP: a public name in doortools for each Door* arm's body (label / valid / issue), as squadthrow has row_label / row_valid / row_issue; then these are calls.
fn tt_of(t: &OrderTarget) -> crate::doortools::DoorToolsThresholdThenT {
    use crate::doortools::DoorToolsThresholdThenT as T;
    match t { OrderTarget::Floor(x) => T::Floor(x.clone()), OrderTarget::Door(x) => T::Door(x.clone()), OrderTarget::None(x) => T::None(x.clone()), OrderTarget::Foe(x) => T::Foe(x.clone()) }
}
fn ct_of(t: &OrderTarget) -> crate::doortools::DoorToolsCanThresholdThenT {
    use crate::doortools::DoorToolsCanThresholdThenT as T;
    match t { OrderTarget::Floor(x) => T::Floor(x.clone()), OrderTarget::Door(x) => T::Door(x.clone()), OrderTarget::None(x) => T::None(x.clone()), OrderTarget::Foe(x) => T::Foe(x.clone()) }
}
/// `stackable(t, men)` as a row's `valid` answers it
fn door_stackable(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    match crate::doortools::door_tools_stackable(g, t, men) {
        crate::doortools::DoorToolsStackable::Text(w) => Err(Some(w)),
        crate::doortools::DoorToolsStackable::Bool(true) => Ok(()),
        crate::doortools::DoorToolsStackable::Bool(false) => Err(None),
    }
}
/// `canThresholdThen(what, t, men)` as a row's `valid` answers it
fn door_can_then(g: &mut Game, what: crate::doortools::DoorToolsCanThresholdThenWhat, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    match crate::doortools::door_tools_can_threshold_then(g, what, Some(&ct_of(t)), men) {
        crate::doortools::DoorToolsCanThresholdThen::Text(w) => Err(Some(w)),
        crate::doortools::DoorToolsCanThresholdThen::Bool(true) => Ok(()),
        crate::doortools::DoorToolsCanThresholdThen::Bool(false) => Err(None),
    }
}
/// `who === 'you' ? wg(…) : `${who} is behind that door.``
fn behind_words(g: &mut Game, who: String) -> String {
    if who == "you" { let (now, was) = (String::from("you're behind that door."), String::from("you are behind that door.")); crate::words::wg(g, &now, &was) } else { format!("{} is behind that door.", who) }
}
fn seam_arm_label(g: &mut Game, arm: &OrderLabel, t: &OrderTarget) -> String {
    use crate::doortools::DoorToolsEntryVerb as Verb;
    match arm {
        // doortools.ts:808 `held(t) ? 'go in and clear · no bang' : `${this.entryVerb(t)} and clear${known(t)}``
        OrderLabel::DoorOpenClear => {
            let men = recipients_of(g, t);
            if crate::doortools::door_tools_held_plain(g, t, &men) { return String::from("go in and clear · no bang"); }
            let verb = crate::doortools::door_tools_entry_verb(g, t);
            let known = match t { OrderTarget::Door(x) if crate::doortools::door_tools_entry_verb(g, t) != Verb::Open => format!(" · {}", crate::doortools::door_tools_shut_why(g, x.door)), _ => String::new() };
            format!("{} and clear{}", verb.as_str(), known)
        }
        // doortools.ts:812
        OrderLabel::DoorBangClear => {
            let men = recipients_of(g, t);
            if crate::doortools::door_tools_held_plain(g, t, &men) { return format!("bang, then go in · {} left", g.door_tools.bangs); }
            let first = if crate::doortools::door_tools_entry_verb(g, t) == Verb::Open { String::from("bang") } else { format!("{}, bang", crate::doortools::door_tools_entry_verb(g, t).as_str()) };
            format!("{} and clear · {} left", first, g.door_tools.bangs)
        }
        // doortools.ts:819 (OWNCHARGE.on is on on the page)
        OrderLabel::DoorCharge => {
            if let OrderTarget::Door(x) = t {
                if crate::doortools::door_tools_loose_on(g, t) { return String::from("stand off · in behind the charge"); }
                if crate::doortools::door_tools_charge_on(g, x.door).is_some() {
                    let c = crate::doortools::door_tools_charge_on(g, x.door).unwrap();
                    return format!("blow the charge{}", crate::doortools::door_tools_close_words(g, c));
                }
            }
            format!("breach with a charge · {} left", g.door_tools.charges)
        }
        // doortools.ts:838
        OrderLabel::DoorFragClear => format!("frag and clear · {} left", g.door_tools.frags),
        // doortools.ts:864 (thrRows on `outsidePlain`)
        OrderLabel::DoorThresholdHold => {
            let men = recipients_of(g, t);
            String::from(if crate::doortools::door_tools_held_plain(g, t, &men) { "clear from outside · holding it" } else { "clear from outside" })
        }
        // doortools.ts:908 / :916
        OrderLabel::DoorGoSmall => { let men = recipients_of(g, t); let w = crate::doortools::door_tools_stack_words(g, &men); if w.is_empty() { String::from("go") } else { format!("go · {}", w) } }
        OrderLabel::DoorGoBig => { let men = recipients_of(g, t); let w = crate::doortools::door_tools_waiting_words(g, &men); if w.is_empty() { String::from("go") } else { format!("go · {}", w) } }
        // doortools.ts:929
        OrderLabel::DoorBigClear => { let men = recipients_of(g, t); format!("{} and clear", crate::doortools::door_tools_big_word(g, t, &men).as_str()) }
        // doortools.ts:935
        OrderLabel::DoorThrWord => {
            if !squad_orders_big(g) {
                return String::from(if crate::silent::silent_no_cans(g) || crate::doortools::door_tools_word_quiet(g) { "go · step in · no bang" } else { "go · bang the blind pockets, step in" });
            }
            let men = recipients_of(g, t); let w = crate::doortools::door_tools_waiting_words(g, &men);
            if w.is_empty() { String::from("go") } else { format!("go · {}", w) }
        }
        // (the arms order_label takes itself, named so that a new row of the enum is a build error here and never a stop in play)
        OrderLabel::Text { .. } | OrderLabel::Cover | OrderLabel::WallRow | OrderLabel::ThrowRow { .. } | OrderLabel::ThrowOpen | OrderLabel::Shell40 | OrderLabel::Work | OrderLabel::Smoke | OrderLabel::PairGo => unreachable!("order_label answers this arm itself"),
    }
}
/// Err(None): false, Err(Some): the word for why not
fn seam_arm_valid(g: &mut Game, arm: &OrderValid, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    use crate::doortools::DoorToolsCanThresholdThenWhat as What;
    match arm {
        // doortools.ts:810
        OrderValid::DoorOpenClear => if crate::doortools::door_tools_held_plain(g, t, men) { door_can_then(g, What::Step, t, men) } else { door_stackable(g, t, men) },
        // doortools.ts:814-817
        OrderValid::DoorBangClear => {
            if crate::doortools::door_tools_held_plain(g, t, men) { return door_can_then(g, What::Bang, t, men); }
            if g.door_tools.bangs <= 0.0 { return Err(Some(String::from("no bangs left."))); }
            let who = match t { OrderTarget::Door(x) => crate::doortools::door_tools_can_behind(g, x.door, x.side, men), _ => None };
            if let Some(who) = who { return Err(Some(behind_words(g, who))); }
            door_stackable(g, t, men)
        }
        // doortools.ts:821-830
        OrderValid::DoorCharge => {
            let OrderTarget::Door(x) = t else { return Err(None) };
            let fits = |r: crate::doortools::DoorToolsStandoffFits| match r { crate::doortools::DoorToolsStandoffFits::Text(w) => Err(Some(w)), crate::doortools::DoorToolsStandoffFits::Bool(true) => Ok(()), crate::doortools::DoorToolsStandoffFits::Bool(false) => Err(None) };
            if crate::doortools::door_tools_loose_on(g, t) { return fits(crate::doortools::door_tools_standoff_fits(g, men, t, Some(true))); }
            if crate::doortools::door_tools_charge_on(g, x.door).is_some() { return Ok(()); }
            if on(g.dr[x.door].def.steel) { return Err(Some(String::from("steel. a charge won't cut it."))); }
            let open = crate::doortools::door_tools_known_open(g, x.door);
            if open { return Err(Some(String::from("it's open."))); }
            if g.door_tools.charges <= 0.0 { return Err(Some(String::from("no charges left."))); }
            let ops = g.door_tools.ops.clone();
            if ops.iter().any(|&o| { let op = &g.tab.doortools_door_op[o]; op.kind == crate::doortools::DoorToolKind::Charge && op.door == x.door && op.phase == crate::doortools::Phase::Place }) { return Err(Some(String::from("already on it."))); }
            fits(crate::doortools::door_tools_standoff_fits(g, men, t, None))
        }
        // doortools.ts:839-842 (FRAG.on is on on the page)
        OrderValid::DoorFragClear => {
            if g.door_tools.frags <= 0.0 { return Err(Some(String::from("no frags left."))); }
            let hostages = g.rescue.hostages.clone();
            if hostages.iter().any(|&h| g.gd[h].state != crate::game::GuardState::Dead) { return Err(Some(String::from("not with hostages on the floor."))); }
            let mate = match t { OrderTarget::Door(x) => crate::doortools::door_tools_frag_behind(g, x.door, x.side, men), _ => None };
            if let Some(mate) = mate { return Err(Some(behind_words(g, mate))); }
            door_stackable(g, t, men)
        }
        // doortools.ts:865
        OrderValid::DoorThresholdHold => {
            if crate::doortools::door_tools_held_plain(g, t, men) { return Err(Some(String::from("already holding it."))); }
            match crate::doortools::door_tools_can_threshold(g, t, men) {
                crate::doortools::DoorToolsCanThreshold::Text(w) => Err(Some(w)),
                crate::doortools::DoorToolsCanThreshold::Bool(true) => Ok(()),
                crate::doortools::DoorToolsCanThreshold::Bool(false) => Err(None),
            }
        }
        // doortools.ts:910 / :918
        OrderValid::DoorGoSmall => if !crate::doortools::door_tools_stack_waits(g, men).is_empty() || !crate::doortools::door_tools_frag_held(g, men).is_empty() { Ok(()) } else { Err(Some(String::from("nobody waiting."))) },
        OrderValid::DoorGoBig => if !crate::doortools::door_tools_waiting_any(g, men).is_empty() || !crate::doortools::door_tools_frag_held(g, men).is_empty() { Ok(()) } else { Err(Some(String::from("nobody waiting."))) },
        // doortools.ts:931-936 (SHOTGUN.on, SILENT.toolsLoud and DOORTOOLS.testDoor are on on the page: the silent refusal is not on its path)
        OrderValid::DoorBigClear => {
            let OrderTarget::Door(x) = t else { return Err(None) };
            if crate::doortools::is_open(g, x.door) { return Err(Some(String::from("it's open."))); }
            if on(g.dr[x.door].def.steel) { return Err(Some(String::from("steel. no way through."))); }
            let mut any = false;
            for &m in men { if crate::bigkit::carries_hammer(g, m) || (g.gd[m].state != crate::game::GuardState::Dead && crate::kit::kit_big(g, m) == crate::kit::BigKit::Shotgun) { any = true; break; } }
            if !any { return Err(Some(String::from("nobody here carries one."))); }
            if crate::doors::door_locked(g, x.door) && g.dr[x.door].lock_id.starts_with("wedge:") {
                let mut hammer = false;
                for &m in men { if crate::bigkit::carries_hammer(g, m) { hammer = true; break; } }
                if !hammer && crate::doortools::door_tools_known_shut(g, x.door, x.side) {
                    let (now, was) = (String::from("it's wedged. a shell won't open it."), String::from("wedged. no shell shifts it."));
                    return Err(Some(crate::words::wg(g, &now, &was)));
                }
            }
            door_stackable(g, t, men)
        }
        // doortools.ts:937 `this.heldOn(t, men).some(op => !op.word) || wg(…)`
        OrderValid::DoorThrWord => {
            let ops = crate::doortools::door_tools_held_on(g, t, men);
            if ops.iter().any(|&o| g.tab.doortools_door_op[o].word.is_none()) { return Ok(()); }
            let (now, was) = (String::from("nobody's holding that door."), String::from("nobody is holding that door."));
            Err(Some(crate::words::wg(g, &now, &was)))
        }
        // (the arms order_valid takes itself, named so that a new row of the enum is a build error here and never a stop in play)
        OrderValid::Move | OrderValid::Stack | OrderValid::WallRow | OrderValid::ShutDoor | OrderValid::ThrowRow { .. } | OrderValid::Wand | OrderValid::Shell40 | OrderValid::Work | OrderValid::Smoke | OrderValid::PairTake | OrderValid::PairWatch | OrderValid::PairGo => unreachable!("order_valid answers this arm itself"),
    }
}
/// Some: refused, in those words
fn seam_arm_issue(g: &mut Game, arm: &OrderIssue, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    use crate::doortools::{DoorToolKind as Kind, DoorToolsIssue, DoorToolsThenOnWord, DoorToolsThenOnWordWhat, DoorToolsThresholdThen, DoorToolsThresholdThenWhat};
    let on_word = o.map_or(false, |o| on(o.on_word));
    let issued = |r: DoorToolsIssue| match r { DoorToolsIssue::Text(w) => Some(w), DoorToolsIssue::Arm1(()) => None };
    // `this.heldPlain(t, men) ? (o?.onWord ? this.thenOnWord(what, men, t) : this.thresholdThen(what, men, t)) : this.issue(kind, men, t, !!o?.onWord)`   (doortools.ts:811, :818)
    fn entry(g: &mut Game, bang: bool, men: &[GuardId], t: &OrderTarget, on_word: bool) -> Option<String> {
        if crate::doortools::door_tools_held_plain(g, t, men) {
            if on_word {
                return match crate::doortools::door_tools_then_on_word(g, if bang { DoorToolsThenOnWordWhat::Bang } else { DoorToolsThenOnWordWhat::Step }, men, t) { DoorToolsThenOnWord::Text(w) => Some(w), DoorToolsThenOnWord::Arm1(()) => None };
            }
            return match crate::doortools::door_tools_threshold_then(g, if bang { DoorToolsThresholdThenWhat::Bang } else { DoorToolsThresholdThenWhat::Step }, men, Some(&tt_of(t))) { DoorToolsThresholdThen::Text(w) => Some(w), DoorToolsThresholdThen::Arm1(()) => None };
        }
        match crate::doortools::door_tools_issue(g, if bang { Kind::BangClear } else { Kind::OpenClear }, men, t, Some(on_word)) { DoorToolsIssue::Text(w) => Some(w), DoorToolsIssue::Arm1(()) => None }
    }
    match arm {
        OrderIssue::DoorOpenClear => entry(g, false, men, t, on_word),
        OrderIssue::DoorBangClear => entry(g, true, men, t, on_word),
        // doortools.ts:831
        OrderIssue::DoorCharge => {
            let OrderTarget::Door(x) = t else { return Some(String::from("no door")) };
            let started = |r: crate::doortools::DoorToolsStartCharge| match r { crate::doortools::DoorToolsStartCharge::Text(w) => Some(w), crate::doortools::DoorToolsStartCharge::Arm1(()) => None };
            if crate::doortools::door_tools_loose_on(g, t) { return started(crate::doortools::door_tools_start_charge(g, men, t, Some(true))); }
            if let Some(c) = crate::doortools::door_tools_charge_on(g, x.door) { crate::doortools::door_tools_trigger(g, Some(c)); return None; }
            started(crate::doortools::door_tools_start_charge(g, men, t, None))
        }
        // doortools.ts:843 `this.fragNext = true; try { return this.issue('bangClear', …) } finally { this.fragNext = false }`
        OrderIssue::DoorFragClear => { g.door_tools.frag_next = true; let r = crate::doortools::door_tools_issue(g, Kind::BangClear, men, t, Some(on_word)); g.door_tools.frag_next = false; issued(r) }
        // doortools.ts:865
        OrderIssue::DoorThresholdHold => match crate::doortools::door_tools_threshold_hold(g, men, t, Some(on_word)) { crate::doortools::DoorToolsThresholdHold::Text(w) => Some(w), crate::doortools::DoorToolsThresholdHold::Arm1(()) => None },
        // doortools.ts:910 `this.goMen(men, this.stackWaits(men)) ?? undefined`
        OrderIssue::DoorGoSmall => {
            let only: Vec<crate::doortools::DoorToolsGoMenOnlyItem> = crate::doortools::door_tools_stack_waits(g, men).into_iter().map(|x| crate::doortools::DoorToolsGoMenOnlyItem { stack: x.stack, men: x.men }).collect();
            crate::doortools::door_tools_go_men(g, men, Some(&only))
        }
        // doortools.ts:918 `{ this.goMen(men); }`
        OrderIssue::DoorGoBig => { crate::doortools::door_tools_go_men(g, men, None); None }
        // doortools.ts:937
        OrderIssue::DoorBigClear => { g.door_tools.big_next = true; let r = crate::doortools::door_tools_issue(g, Kind::OpenClear, men, t, Some(on_word)); g.door_tools.big_next = false; issued(r) }
        // doortools.ts:938 `(S.big ? this.goMen(men) : this.pairWord(men) ? null : 'nobody\'s waiting for you to say go.') ?? undefined`
        OrderIssue::DoorThrWord => if squad_orders_big(g) { crate::doortools::door_tools_go_men(g, men, None) } else if crate::doortools::door_tools_pair_word(g, men) { None } else { Some(String::from("nobody's waiting for you to say go.")) },
        // squad40.ts:208, squadsmoke.ts:135, pairorders.ts:211 / :227 / :240. These bodies are not in the skeleton's sig.rs: the paths resolve only in the joined crate
        OrderIssue::Shell40 => crate::squad40::p_01::row_issue(g, men, t),
        OrderIssue::Smoke => crate::squadsmoke::p_01::row_smoke_issue(g, men, t, o),
        OrderIssue::PairTake { mode } => crate::pairorders::p_01::take_issue(g, *mode, men, t),
        OrderIssue::PairWatch => crate::pairorders::p_01::watch_issue(g, men, t),
        OrderIssue::PairDark => crate::pairorders::p_01::dark_issue(g, men, t),
        // (the arms order_issue takes itself, named so that a new row of the enum is a build error here and never a stop in play)
        OrderIssue::Nothing | OrderIssue::Move | OrderIssue::Stack | OrderIssue::Cover | OrderIssue::FallIn | OrderIssue::Hold | OrderIssue::Silent | OrderIssue::Loud | OrderIssue::ShutDoor | OrderIssue::ThrowRow { .. } | OrderIssue::WallRow | OrderIssue::Wand | OrderIssue::Work | OrderIssue::PairGo => unreachable!("order_issue answers this arm itself"),
    }
}

// ---- OrderDef's fields: the matches (the skeleton's INVENTED order_*: orders.ts calls these fields in this part's lines). An arm is the row's public name, or a few public calls written here; the rest wait in the seams above ----
fn plain_valid(w: Option<String>) -> Result<(), Option<String>> {
    match w { Some(w) => Err(Some(w)), None => Ok(()) }
}
/// `d.label(t)`
pub fn order_label(g: &mut crate::game::Game, d: OrdersOrderDefId, t: &crate::orders::OrderTarget) -> String {
    let arm = g.tab.orders_order_def[d].label.clone();
    match arm {
        OrderLabel::Text { text } => text,
        OrderLabel::Cover => row_cover_label(g, t),
        OrderLabel::WallRow => match crate::wallcharge::wall_charge_charge_on_target(g, t) {
            Some(c) => format!("blow the charge{}", crate::wallcharge::wall_charge_close_words(g, Some(c), None)),
            None => crate::wallcharge::wall_charge_row_words(g),
        },
        OrderLabel::ThrowRow { kind } => crate::squadthrow::row_label(g, kind, at_of(t)),
        OrderLabel::ThrowOpen => {
            let frag = crate::squadthrow::left(g, crate::squadthrow::SquadThrowKind::Frag);
            let stun = crate::squadthrow::left(g, crate::squadthrow::SquadThrowKind::Stun);
            format!("throw · frag {} · bang {}{}", frag, stun, crate::squad40::squad40_word(g))
        }
        OrderLabel::Shell40 => format!("40 mm there · {} left", crate::squad40::squad40_left(g)),
        OrderLabel::Work => match t {
            OrderTarget::Floor(OrderTargetFloor { thing: Some(th), .. }) => crate::workthings::work_things_words(g, th),
            _ => String::from("work it"),
        },
        OrderLabel::Smoke => format!("smoke there · {} left", crate::squadsmoke::squad_smoke_left(g)),
        OrderLabel::PairGo => {
            let waiting = crate::pairorders::pair_orders_waiting(g);
            if waiting.is_empty() { return String::from("mark"); }
            let mut parts: Vec<String> = Vec::with_capacity(waiting.len());
            for (gd, j) in &waiting {
                let foe = crate::pairorders::name_of(g, j.foe);
                parts.push(format!("{} takes {}{}", g.gd[*gd].callsign, foe, if j.watcher.is_some() { " anyway" } else { "" }));
            }
            format!("mark · {}", parts.join(" · "))
        }
        arm => seam_arm_label(g, &arm, t),
    }
}
/// `d.can(S, gd, t)`, the match over OrderCan: None he can, or the row has no `can`; Some the word he says (orders.ts:850, :868)
pub fn order_can(g: &mut crate::game::Game, d: OrdersOrderDefId, gd: GuardId, t: &crate::orders::OrderTarget) -> Option<String> {
    let Some(arm) = g.tab.orders_order_def[d].can else { return None };
    let at = at_of(t);
    match arm {
        OrderCan::Move => row_move_can(g, gd, t),
        OrderCan::Cover => row_cover_can(g, gd, t),
        OrderCan::WallRow => match crate::wallcharge::wall_charge_can_of(g, gd, t) {
            crate::wallcharge::WallChargeCanOf::Text(w) => Some(w),
            crate::wallcharge::WallChargeCanOf::Bool(_) => None,
        },
        OrderCan::Work => squad_orders_reach(g, gd, at),
        OrderCan::PairTake => {
            let OrderTarget::Foe(x) = t else {
                let (now, was) = (String::from("nobody there."), String::from("no man there."));
                return Some(crate::words::wg(g, &now, &was));
            };
            let to = crate::pairorders::pair_orders_behind(g, x.foe);
            let spot = squad_orders_free_spot(g, to).unwrap_or(to);
            if !crate::pairorders::pair_orders_back_open(g, x.foe, spot) || !crate::pairorders::pair_orders_rear(g, x.foe, spot, Some(true)) {
                let (now, was) = (String::from("no room behind him."), String::from("no room at his back."));
                return Some(crate::words::wg(g, &now, &was));
            }
            let stage = crate::pairorders::pair_orders_stage(g, x.foe);
            squad_orders_reach(g, gd, stage)
        }
        OrderCan::PairWatch => match t {
            OrderTarget::Foe(x) if crate::pairorders::pair_orders_sees(g, gd, x.foe) => None,
            _ => Some(String::from("no line from here.")),
        },
        OrderCan::PairDark => {
            let dark = crate::pairorders::pair_orders_dark_spot(g, at);
            let w = squad_orders_reach(g, gd, dark.unwrap_or(at));
            if w.is_some() { return w; }
            if dark.is_some() { None } else { Some(String::from("no dark spot there.")) }
        }
    }
}
/// `d.only(S, men, t)`, the match over OrderOnly. A row with no `only`: men, as a copy (orders.ts:865, :1006)
pub fn order_only(g: &mut crate::game::Game, d: OrdersOrderDefId, men: &[GuardId], t: &crate::orders::OrderTarget) -> Vec<GuardId> {
    let Some(arm) = g.tab.orders_order_def[d].only.clone() else { return men.to_vec() };
    let at = at_of(t);
    match arm {
        OrderOnly::DoorHeldOrAll => if crate::doortools::door_tools_held_plain(g, t, men) { crate::doortools::door_tools_held_men(g, men, t) } else { men.to_vec() },
        OrderOnly::DoorGoSmall => {
            let mut waits: Vec<GuardId> = Vec::new();
            for x in crate::doortools::door_tools_stack_waits(g, men) { waits.extend(x.men); }
            let held = crate::doortools::door_tools_frag_held(g, men);
            men.iter().copied().filter(|m| waits.contains(m) || held.iter().any(|&o| g.tab.doortools_door_op[o].men.contains(m))).collect()
        }
        OrderOnly::DoorGoBig => {
            let waits = crate::doortools::door_tools_waiting_men(g, men);
            if !waits.is_empty() { return waits; }
            let held = crate::doortools::door_tools_frag_held(g, men);
            men.iter().copied().filter(|m| held.iter().any(|&o| g.tab.doortools_door_op[o].men.contains(m))).collect()
        }
        // doortools.ts:936 `S.big ? this.waitingMen(men) : this.heldMen(men, t)`
        OrderOnly::DoorThrWord => if squad_orders_big(g) { crate::doortools::door_tools_waiting_men(g, men) } else { crate::doortools::door_tools_held_men(g, men, t) },
        OrderOnly::SilentAll => squad_orders_men(g, None, None),
        OrderOnly::ShutDoor => {
            let OrderTarget::Door(x) = t else { return men.to_vec() };
            if crate::doors::door_off(g, x.door) || !crate::shutdoor::stands_open(g, x.door) { return men.to_vec(); }
            match crate::shutdoor::plan(g, x.door, men) {
                crate::shutdoor::Plan::Text(_) => men.to_vec(),
                crate::shutdoor::Plan::Arm1(p) => vec![p.gd],
            }
        }
        OrderOnly::ThrowRow { kind } => crate::squadthrow::row_only(g, men, at, kind),
        OrderOnly::Wand => {
            let OrderTarget::Door(x) = t else { return Vec::new() };
            for side in [x.side, -x.side] {
                let mut there = OrderTarget::Door(OrderTargetDoor { side, ..*x });
                if let crate::wandorder::Pick::Arm1(p) = crate::wandorder::pick(g, men, &mut there) { return vec![p.gd]; }
            }
            Vec::new()
        }
        OrderOnly::Shell40 => match crate::squad40::plan(g, men, at, Some(crate::squad40::Asker::Only)) {
            crate::squad40::Plan::Text(_) => Vec::new(),
            crate::squad40::Plan::Arm1(a) => vec![a.a.gd],
        },
        OrderOnly::Work => crate::workthings::work_things_pick(g, men, t),
        OrderOnly::Smoke => match crate::squadsmoke::squad_smoke_thrower(g, men, at) { Some(th) => vec![th.gd], None => Vec::new() },
        OrderOnly::PairTake => {
            let OrderTarget::Foe(x) = t else { return Vec::new() };
            let to = crate::pairorders::pair_orders_stage(g, x.foe);
            let mut out: Vec<GuardId> = Vec::new();
            for &gd in men { if squad_orders_reach(g, gd, to).is_none() { out.push(gd); } }
            let there = g.ch[g.gd[x.foe].ch].pos;
            let rows: &Game = &*g;
            js::sort_by(&mut out, |&a, &b| v3::dist_xz(rows.ch[rows.gd[a].ch].pos, there) - v3::dist_xz(rows.ch[rows.gd[b].ch].pos, there));
            out.truncate(1);
            out
        }
        OrderOnly::PairGo => crate::pairorders::pair_orders_waiting(g).into_iter().map(|(gd, _)| gd).collect(),
    }
}
/// `d.valid(S, t, men)`, the match over OrderValid: Ok true, or the row has no `valid`; Err(None) false, whyNot's 'not here.'; Err(Some) the word (orders.ts:852)
pub fn order_valid(g: &mut crate::game::Game, d: OrdersOrderDefId, t: &crate::orders::OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    let Some(arm) = g.tab.orders_order_def[d].valid.clone() else { return Ok(()) };
    let at = at_of(t);
    match arm {
        OrderValid::Move => row_move_valid(g, t, men),
        OrderValid::Stack => row_stack_valid(g, t, men),
        OrderValid::WallRow => match crate::wallcharge::wall_charge_valid(g, t, men) {
            crate::wallcharge::WallChargeValid::Text(w) => Err(Some(w)),
            crate::wallcharge::WallChargeValid::Bool(true) => Ok(()),
            crate::wallcharge::WallChargeValid::Bool(false) => Err(None),
        },
        OrderValid::ShutDoor => {
            let OrderTarget::Door(x) = t else { return Err(None) };
            plain_valid(crate::shutdoor::why_not_shut(g, x.door, men))
        }
        OrderValid::ThrowRow { kind } => crate::squadthrow::row_valid(g, men, at, kind).map_err(Some),
        OrderValid::Wand => match crate::wandorder::pick(g, men, &mut t.clone()) {
            crate::wandorder::Pick::Text(w) => Err(Some(w)),
            crate::wandorder::Pick::Arm1(_) => Ok(()),
        },
        OrderValid::Shell40 => match crate::squad40::plan(g, men, at, Some(crate::squad40::Asker::Valid)) {
            crate::squad40::Plan::Text(w) => Err(Some(w)),
            crate::squad40::Plan::Arm1(a) => plain_valid(a.who),
        },
        OrderValid::Work => {
            let OrderTarget::Floor(OrderTargetFloor { thing: Some(th), .. }) = t else { return Err(Some(String::from("nothing to work there."))) };
            if th.stand.is_none() { return Err(Some(String::from("can't get a hand on it."))); }
            let verb = crate::workthings::work_things_verb_of(g, th);
            plain_valid(crate::workthings::work_things_moot(g, th, verb))
        }
        OrderValid::Smoke => {
            if crate::squadsmoke::squad_smoke_left(g) <= 0.0 { return Err(Some(String::from("no smoke left."))); }
            if crate::squadsmoke::squad_smoke_thrower(g, men, at).is_some() { Ok(()) } else { Err(Some(String::from("no throw from here."))) }
        }
        OrderValid::PairTake => {
            let OrderTarget::Foe(x) = t else { return Err(None) };
            if let Some(w) = crate::pairorders::pair_orders_gone(g, x.foe) { return Err(Some(w)); }
            if crate::pairorders::pair_orders_noticed(g, x.foe) {
                let (now, was) = (String::from("he's fighting. can't grab him."), String::from("he's fighting. no hands."));
                return Err(Some(crate::words::wg(g, &now, &was)));
            }
            if crate::pairorders::pair_orders_caught(g) {
                let (now, was) = (String::from("they're onto us. can't grab him."), String::from("they're onto us. no hands."));
                return Err(Some(crate::words::wg(g, &now, &was)));
            }
            Ok(())
        }
        OrderValid::PairWatch => {
            let OrderTarget::Foe(x) = t else { return Err(None) };
            plain_valid(crate::pairorders::pair_orders_gone(g, x.foe))
        }
        OrderValid::PairGo => {
            if !crate::pairorders::pair_orders_waiting(g).is_empty() { return Ok(()); }
            let (now, was) = (String::from("nobody's waiting on your mark."), String::from("nobody is waiting on your mark."));
            Err(Some(crate::words::wg(g, &now, &was)))
        }
        arm => seam_arm_valid(g, &arm, t, men),
    }
}
/// `d.issue(S, men, t, opts)`, the match over OrderIssue: Some is the refusal, the string the closure returned (orders.ts:877)
pub fn order_issue(g: &mut crate::game::Game, d: OrdersOrderDefId, men: &[GuardId], t: &crate::orders::OrderTarget, o: Option<&crate::orders::GiveOpts>) -> Option<String> {
    let arm = g.tab.orders_order_def[d].issue.clone();
    match arm {
        OrderIssue::Nothing => None,
        OrderIssue::Move => row_move_issue(g, men, t, o),
        OrderIssue::Stack => row_stack_issue(g, men, t, o),
        OrderIssue::Cover => row_cover_issue(g, men, t, o),
        OrderIssue::FallIn => row_fall_in_issue(g, men, t, o),
        OrderIssue::Hold => row_hold_issue(g, men, t, o),
        OrderIssue::Silent => { crate::silent::set_silent(g, true, Some(crate::silent::By::Order)); None }
        OrderIssue::Loud => { crate::silent::set_silent(g, false, Some(crate::silent::By::Order)); None }
        OrderIssue::ShutDoor => crate::shutdoor::order_issue(g, men, t),
        OrderIssue::ThrowRow { kind } => crate::squadthrow::row_issue(g, men, t, kind),
        OrderIssue::WallRow => match crate::wallcharge::wall_charge_issue(g, men, t) {
            crate::wallcharge::WallChargeIssue::Text(w) => Some(w),
            crate::wallcharge::WallChargeIssue::Arm1(()) => None,
        },
        OrderIssue::Wand => match crate::wandorder::issue(g, men, &mut t.clone()) {
            crate::wandorder::Issue::Text(w) => Some(w),
            crate::wandorder::Issue::Arm1(()) => None,
        },
        OrderIssue::Work => match crate::workthings::work_things_issue(g, men, t) {
            crate::workthings::WorkThingsIssue::Text(w) => Some(w),
            crate::workthings::WorkThingsIssue::Arm1(()) => None,
        },
        OrderIssue::PairGo => { crate::pairorders::pair_orders_mark(g, Some(crate::pairorders::PairOrdersMarkBy::Row)); None }
        arm => seam_arm_issue(g, &arm, men, t, o),
    }
}
/// `d.gone?.(S, t, men)`, the match over OrderGone. False with no `gone` (orders.ts:831)
pub fn order_gone(g: &mut crate::game::Game, d: OrdersOrderDefId, t: &crate::orders::OrderTarget, men: &[GuardId]) -> bool {
    let Some(arm) = g.tab.orders_order_def[d].gone else { return false };
    match arm {
        OrderGone::DoorCharge => match t {
            OrderTarget::Door(x) => on(g.dr[x.door].def.steel) && crate::doortools::door_tools_charge_on(g, x.door).is_none(),
            _ => false,
        },
        OrderGone::DoorBigClear => match t {
            OrderTarget::Door(x) => crate::doortools::is_open(g, x.door) || on(g.dr[x.door].def.steel),
            _ => false,
        },
        OrderGone::WallRow => g.wall_charge.brought == Some(0.0) && crate::wallcharge::wall_charge_charge_on_target(g, t).is_none(),
    }
}
/// `d.standsIn?.when(S, t, men)`, the match over OrderWhen. False with no stands_in (orders.ts:819)
pub fn order_when(g: &mut crate::game::Game, d: OrdersOrderDefId, t: &crate::orders::OrderTarget, men: &[GuardId]) -> bool {
    let Some(arm) = g.tab.orders_order_def[d].stands_in.as_ref().map(|s| s.when) else { return false };
    match arm {
        OrderWhen::ShutDoor => {
            let OrderTarget::Door(x) = t else { return false };
            crate::shutdoor::stands_open(g, x.door) && crate::doortools::door_tools_charge_on(g, x.door).is_none()
        }
        // doortools.ts:846 `if (DOORTOOLS.outsidePlain) return false; …`: outsidePlain is on on the page (settled), so the held page never stands in
        OrderWhen::DoorHeld => false,
        OrderWhen::Loud => crate::silent::squad_silent(g),
        OrderWhen::PairDark => crate::pairorders::pair_orders_up(g) && crate::silent::squad_silent(g),
    }
}
/// `this.people?.ray(ro, rd, max) ?? null`: the man of the other side a cursor ray lands on nearer than `max` (orders.ts:783; the closure is pairorders.ts:162)
pub fn squad_orders_people_ray(g: &mut crate::game::Game, ro: V3, rd: V3, max: f64) -> Option<GuardId> {
    if g.orders.people.is_none() || !crate::pairorders::pair_orders_up(g) { return None; }
    let (mut best, mut bt) = (None, max);
    let mut i = 0;
    while i < g.guards.len() {
        let h = g.guards[i];
        i += 1;
        if !crate::pairorders::pair_orders_is_foe(g, h) { continue; }
        let c = g.gd[h].ch;
        let t = crate::character::character_ray_hit(g, c, ro, rd, bt);
        if t >= 0.0 && t < bt { bt = t; best = Some(h); }
    }
    best
}

// ---- the bodies ----

/// What the cursor is on, before the whole-squad rule (orders.ts SquadOrders.targetUnderOf)
pub fn squad_orders_target_under_of(g: &mut crate::game::Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View) -> crate::orders::OrderTarget {
    if g.puppet.as_ref().map_or(false, |p| p.aim.is_some()) { let a = g.aim_point; return squad_orders_target_at(g, a); }
    let mc = crate::rails::RailsMarkUnderCanvas { client_width: canvas.client_width, client_height: canvas.client_height };
    if let Some(mk) = crate::rails::rails_mark_under(g, input, cam, mc) { return squad_orders_target_at(g, mk); }
    let (mx, my) = (g.tab.input[input].mouse_x, g.tab.input[input].mouse_y);
    let (u, v) = (mx / m::max(1.0, canvas.client_width), my / m::max(1.0, canvas.client_height));
    let ray = crate::camera::follow_camera_ray(g, cam, u, v);
    let (ro, rd) = (ray.ro, ray.rd);
    let pc = player_ch(g);
    let own_id = g.ch[pc].id;
    let hit = crate::scene_boxes::box_world_raycast(g, ro, rd, Some(200.0), None, Some(own_id), None);
    let p: V3 = match &hit {
        Some(h) => v3::mad(ro, rd, h.t),
        None => match crate::camera::follow_camera_ground_point(g, cam, u, v, Some(0.0)) { Some(q) => q, None => g.aim_point },
    };
    let mut man: Option<GuardId> = None;
    let mut best = match &hit { Some(h) => h.t + 0.3, None => 200.0 };
    for gd in squad_orders_squad(g) {
        if !alive(g, gd) { continue; }
        let c = g.gd[gd].ch;
        let t = crate::character::character_ray_hit(g, c, ro, rd, best);
        if t >= 0.0 && t < best { best = t; man = Some(gd); }
    }
    let fo = squad_orders_people_ray(g, ro, rd, best);
    if let Some(foe) = fo {
        let q = g.ch[g.gd[foe].ch].pos;
        return OrderTarget::Foe(OrderTargetFoe { at: [q[0], 0.0, q[2]], foe });
    }
    if let Some(gd) = man {
        let q = g.ch[g.gd[gd].ch].pos;
        return OrderTarget::None(OrderTargetNone { at: [q[0], 0.0, q[2]], man: Some(gd) });
    }
    if let Some(h) = &hit {
        let b = g.engine.world.all.get(h.index as usize).copied();
        if let Some(wt) = crate::workthings::work_on(g, b) { return wt; }
    }
    let self_to = match &hit { Some(h) => h.t, None => 200.0 };
    if crate::character::character_ray_hit(g, pc, ro, rd, self_to) >= 0.0 {
        let q = g.ch[pc].pos;
        return OrderTarget::None(OrderTargetNone { at: [q[0], 0.0, q[2]], man: None });
    }
    let base = squad_orders_target_at(g, p);
    if matches!(base, OrderTarget::Door(_)) || crate::character::character_ray_hit(g, pc, ro, rd, best) >= 0.0 { return base; }
    let first = hit.as_ref().map(|h| SquadOrdersSceneHitFirst { t: h.t, index: h.index });
    let sc = squad_orders_scene_hit(g, ro, rd, first);
    let bx = if sc.index >= 0.0 { g.engine.world.all.get(sc.index as usize).copied() } else { None };
    let land = if sc.index >= 0.0 { v3::mad(ro, rd, sc.t) } else { p };
    let own = bx.and_then(|b| g.doors.list.iter().copied().find(|&d| g.dr[d].box_ == b));
    let leaf = match (own, bx) {
        (Some(d), _) => if crate::doors::door_off(g, d) { None } else { Some(d) },
        (None, Some(b)) => crate::fogofwar::fog_of_war_door_of_box(g, b),
        (None, None) => None,
    };
    if leaf.is_none() {
        if let OrderTarget::Floor(f) = &base { if f.thing.is_some() { return base; } }
    }
    let door = match leaf { Some(d) => Some(d), None => squad_orders_door_on_ray(g, ro, rd, Some(sc.t)) };
    if let Some(d) = door {
        let fc = g.dr[d].frame_centre;
        if Some(d) == leaf || v3::dist_xz(fc, land) <= g.k.orders.door_ray_reach_m {
            let q = g.ch[pc].pos;
            let side = crate::orders::side_of(g, d, q);
            return OrderTarget::Door(OrderTargetDoor { at: [fc[0], 0.0, fc[2]], door: d, side });
        }
    }
    base
}

/// What a cursor ray lands on of the scene, through glass and past the men (orders.ts SquadOrders.sceneHit)
pub fn squad_orders_scene_hit(g: &mut crate::game::Game, ro: V3, rd: V3, first: Option<crate::orders::SquadOrdersSceneHitFirst>) -> crate::orders::SquadOrdersSceneHit {
    let f = crate::scene_boxes::box_flag();
    let skip = (js::to_i32(f.no_shadow) | js::to_i32(f.no_primary) | js::to_i32(f.glass)) as f64;
    if let Some(h) = first {
        if let Some(b) = g.engine.world.all.get(h.index as usize).copied() {
            if !js::truthy(seam_box_owner(g, b)) && (js::to_i32(seam_box_flags(g, b)) & js::to_i32(f.glass)) == 0 {
                return SquadOrdersSceneHit { t: h.t, index: h.index };
            }
        }
    }
    let mut t0 = 0.0;
    let pc = player_ch(g);
    let mut owner = g.ch[pc].id;
    for k in 0..16 {
        let from = if k != 0 { v3::mad(ro, rd, t0) } else { ro };
        let Some(hit) = crate::scene_boxes::box_world_raycast(g, from, rd, Some(200.0), Some(skip), Some(owner), None) else { break };
        owner = match g.engine.world.all.get(hit.index as usize).copied() { Some(b) => seam_box_owner(g, b), None => 0.0 };
        t0 += hit.t;
        if !js::truthy(owner) { return SquadOrdersSceneHit { t: t0, index: hit.index }; }
    }
    SquadOrdersSceneHit { t: f64::INFINITY, index: -1.0 }
}

/// The rows that fit a target's kind on one page of the menu (orders.ts SquadOrders.rowsFor)
pub fn squad_orders_rows_for(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, page: Option<&str>, men: Option<&[GuardId]>) -> Vec<OrdersOrderDefId> {
    let kind = kind_of(t);
    let listed: Vec<OrdersOrderDefId> = g.orders.defs.iter().copied().filter(|&d| {
        let r = &g.tab.orders_order_def[d];
        r.listed.as_ref().unwrap_or(&r.targets).contains(&kind) && r.page.as_deref() == page
    }).collect();
    let walled = crate::wallcharge::wall_rows(g, t, page, &listed);
    let rows = crate::workthings::work_rows(g, t, page, &walled);
    if !rows.iter().any(|&d| g.tab.orders_order_def[d].stands_in.is_some()) { return crate::rails::rails_order_rows(g, &rows); }
    let who = match men { Some(x) => x.to_vec(), None => recipients_of(g, t) };
    let mut ins: Vec<OrdersOrderDefId> = Vec::new();
    for &d in &rows {
        if g.tab.orders_order_def[d].stands_in.is_some() && order_when(g, d, t, &who) { ins.push(d); }
    }
    let mut out: JsSet<String> = JsSet::new();
    for &d in &ins {
        if let Some(s) = &g.tab.orders_order_def[d].stands_in { out.add(s.for_.clone()); }
    }
    let all: Vec<OrdersOrderDefId> = rows.iter().copied().filter(|&d| {
        let r = &g.tab.orders_order_def[d];
        if r.stands_in.is_some() { ins.contains(&d) } else { !out.has(r.id.clone()) }
    }).collect();
    crate::rails::rails_order_rows(g, &all)
}

/// A ring put up now is one of the flat tree (orders.ts SquadOrders.flatRing)
pub fn squad_orders_flat_ring(g: &mut crate::game::Game) -> bool {
    !crate::rails::rails_live(g) || crate::rails::rails_flat_ring(g)
}

fn flat_walk(g: &mut Game, t: &OrderTarget, men: Option<&[GuardId]>, who: &[GuardId], page: Option<&str>, out: &mut Vec<OrdersOrderDefId>, seen: &mut JsSet<String>) {
    let rows = squad_orders_rows_for(g, t, page, men);
    for &d in &rows {
        let r = &g.tab.orders_order_def[d];
        if word(&r.opens) || out.iter().any(|&x| g.tab.orders_order_def[x].id == r.id) { continue; }
        if order_gone(g, d, t, who) { continue; }
        out.push(d);
    }
    for &d in &rows {
        let opens = g.tab.orders_order_def[d].opens.clone();
        if let Some(p) = opens.filter(|p| !p.is_empty()) {
            if !seen.has(p.clone()) { seen.add(p.clone()); flat_walk(g, t, men, who, Some(&p), out, seen); }
        }
    }
}

/// Every order of the target for a flat ring (orders.ts SquadOrders.flatRows)
pub fn squad_orders_flat_rows(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: Option<&[GuardId]>) -> Vec<OrdersOrderDefId> {
    let mut out: Vec<OrdersOrderDefId> = Vec::new();
    let mut seen: JsSet<String> = JsSet::new();
    let who = match men { Some(x) => x.to_vec(), None => recipients_of(g, t) };
    flat_walk(g, t, men, &who, None, &mut out, &mut seen);
    let kind = kind_of(t);
    if kind != TargetKind::None {
        let place = OrderTarget::None(OrderTargetNone { at: at_of(t), man: None });
        for d in squad_orders_rows_for(g, &place, None, men) {
            let r = &g.tab.orders_order_def[d];
            if on(r.free) && !word(&r.opens) && !word(&r.hot_key) && r.targets.contains(&kind) && !out.iter().any(|&x| g.tab.orders_order_def[x].id == r.id) { out.push(d); }
        }
    }
    out
}

/// The rows of a menu that is up, read afresh (orders.ts SquadOrders.menuRows)
pub fn squad_orders_menu_rows(g: &mut crate::game::Game, m: &crate::orders::Menu, t: &crate::orders::OrderTarget) -> Vec<OrdersOrderDefId> {
    if on(m.flat) { squad_orders_flat_rows(g, t, Some(&m.men)) } else { squad_orders_rows_for(g, t, m.page.as_deref(), Some(&m.men)) }
}

/// A stand-in row put where the row it stands in for is (orders.ts SquadOrders.seat)
pub fn squad_orders_seat(g: &mut crate::game::Game, def: OrdersOrderDefId, stood: OrdersOrderDefId) {
    let (key, hot, page) = { let s = &g.tab.orders_order_def[stood]; (s.key, s.hot_key.clone(), s.page.clone()) };
    let r = &mut g.tab.orders_order_def[def];
    r.key = key; r.hot_key = hot; r.page = page;
    let at = g.orders.defs.iter().position(|&d| d == stood).map_or(0, |i| i + 1);
    g.orders.defs.insert(at, def);
}

/// Is the 'wait for my word' row offered under these rows (orders.ts SquadOrders.wordRowOf)
pub fn squad_orders_word_row_of(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, rows: &[OrdersOrderDefId]) -> bool {
    if matches!(t, OrderTarget::Foe(_)) { return false; }
    rows.iter().any(|&d| crate::planlay::layable(g, d))
}

/// The open menu's page that has the 'wait for my word' row (orders.ts SquadOrders.wordPage)
pub fn squad_orders_word_page(g: &mut crate::game::Game, m: &crate::orders::Menu) -> Option<String> {
    if m.page.is_some() { return None; }
    for &d in &m.rows {
        let opens = g.tab.orders_order_def[d].opens.clone();
        if let Some(p) = opens.filter(|p| !p.is_empty()) {
            let rows = squad_orders_rows_for(g, &m.target, Some(&p), Some(&m.men));
            if squad_orders_word_row_of(g, &m.target, &rows) { return Some(p); }
        }
    }
    None
}

/// The digit the open menu's 'wait for my word' row is on (orders.ts SquadOrders.waitDigit)
pub fn squad_orders_wait_digit(g: &mut crate::game::Game, m: &crate::orders::Menu) -> f64 {
    let k = crate::orders::digit_of(&g.k.orders.wait_key);
    if m.word_row && k <= crate::orders::ORDER_DIGITS && !m.rows.iter().any(|&d| g.tab.orders_order_def[d].key == Some(k)) { k } else { 0.0 }
}

/// Why these men cannot carry that order out on that target (orders.ts SquadOrders.whyNot)
pub fn squad_orders_why_not(g: &mut crate::game::Game, d: OrdersOrderDefId, t: &crate::orders::OrderTarget, men: Option<&[GuardId]>) -> Option<String> {
    let men: Vec<GuardId> = match men { Some(x) => x.to_vec(), None => recipients_of(g, t) };
    if !g.tab.orders_order_def[d].targets.contains(&kind_of(t)) { return Some(String::from("not here.")); }
    if men.is_empty() { return Some(String::from("nobody to take it.")); }
    let mut first: Option<String> = None;
    let able: Vec<GuardId> = if g.tab.orders_order_def[d].can.is_some() {
        let mut a = Vec::with_capacity(men.len());
        for &gd in &men {
            match order_can(g, d, gd, t) {
                None => a.push(gd),
                Some(w) => { if first.is_none() { first = Some(w); } }
            }
        }
        a
    } else { men };
    if able.is_empty() { return Some(first.unwrap_or_else(|| String::from("not here."))); }
    if g.tab.orders_order_def[d].valid.is_none() { return None; }
    match order_valid(g, d, t, &able) {
        Ok(()) => None,
        Err(Some(w)) => Some(w),
        Err(None) => Some(String::from("not here.")),
    }
}

/// A row's words (orders.ts SquadOrders.labelOf)
pub fn squad_orders_label_of(g: &mut crate::game::Game, d: OrdersOrderDefId, t: &crate::orders::OrderTarget) -> String {
    order_label(g, d, t)
}

/// Issue an order by id; None when given, else why not (orders.ts SquadOrders.give)
pub fn squad_orders_give(g: &mut crate::game::Game, id_2: &str, t: Option<&crate::orders::OrderTarget>, who: Option<crate::orders::Who>, opts: Option<&crate::orders::GiveOpts>) -> Option<String> {
    let t: OrderTarget = match t { Some(x) => x.clone(), None => { let a = g.aim_point; squad_orders_target_at(g, a) } };
    if !squad_orders_active(g) { return Some(String::from("no squad")); }
    let Some(def) = g.orders.defs.iter().copied().find(|&d| g.tab.orders_order_def[d].id == id_2) else { return Some(format!("no such order '{}'", id_2)); };
    if word(&g.tab.orders_order_def[def].opens) { return Some(format!("'{}' is a page of the menu, not an order", id_2)); }
    if !g.tab.orders_order_def[def].targets.contains(&kind_of(&t)) { return Some(format!("'{}' does not fit a {} target", id_2, kind_of(&t).as_str())); }
    let mut asked: Vec<GuardId> = match opts.and_then(|o| o.men.as_ref()) {
        Some(list) => {
            let mut a = Vec::with_capacity(list.len());
            for &gd in list { if alive(g, gd) && squad_orders_mine(g, gd) { a.push(gd); } }
            a
        }
        None => squad_orders_recipients(g, Some(&t), who),
    };
    if asked.is_empty() { crate::game::game_msg(g, "nobody to take that order"); return Some(String::from("nobody to take it")); }
    if g.tab.orders_order_def[def].only.is_some() {
        let o = order_only(g, def, &asked, &t);
        if !o.is_empty() { asked = o; }
    }
    g.orders.face_flip = false;
    let t = squad_orders_for_men(g, &t, &asked);
    let flipped = g.orders.face_flip;
    let mut cant: Vec<(GuardId, String)> = Vec::new();
    let all: Vec<GuardId> = if g.tab.orders_order_def[def].can.is_some() {
        let mut a = Vec::with_capacity(asked.len());
        for &gd in &asked {
            match order_can(g, def, gd, &t) {
                None => a.push(gd),
                Some(w) => cant.push((gd, w)),
            }
        }
        a
    } else { asked };
    let no = if !all.is_empty() { squad_orders_why_not(g, def, &t, Some(&all)) } else { Some(cant[0].1.clone()) };
    if let Some(no) = no.filter(|w| !w.is_empty()) {
        let by = if !all.is_empty() {
            let k = g.orders.acks;
            g.orders.acks += 1.0;
            all[js::rem(k, all.len() as f64) as usize]
        } else { cant[0].0 };
        crate::game::game_say(g, by, &no, None, None, None);
        return Some(no);
    }
    for (gd, w) in &cant { crate::game::game_say(g, *gd, w, None, None, None); }
    let mut why: Option<String> = None;
    let mut men: Vec<GuardId> = Vec::new();
    let free = on(g.tab.orders_order_def[def].free);
    for &gd in &all {
        let w = if !free && nightbreach_owns(g, gd) { nightbreach_release(g, gd) } else { None };
        match w { Some(w) if !w.is_empty() => why = Some(w), _ => men.push(gd) }
    }
    if men.is_empty() {
        let words = squad_orders_who_words(g, Some(&all));
        let line = format!("{}: {}", words, why.as_deref().unwrap_or("null"));
        crate::game::game_msg(g, &line);
        return why;
    }
    g.orders.group += 1.0;
    let was_silent = crate::holdfire::hold_fire_silent(g, id_2);
    if let Some(refused) = order_issue(g, def, &men, &t, opts) {
        let line = format!("{}: {}", squad_orders_who_words(g, Some(&men)), refused);
        crate::game::game_msg(g, &line);
        return Some(refused);
    }
    if men.len() > 1 { squad_orders_same_ways(g, &men); }
    if flipped { g.cleared.stats.face += 1.0; }
    if !free {
        let target = match &t {
            OrderTarget::Door(x) => format!("door:{}", g.dr[x.door].def.name),
            OrderTarget::Floor(x) => match &x.thing {
                Some(th) => format!("thing:{}", th.kind.as_str()),
                None => format!("floor:{},{}", js::to_fixed(x.at[0], 1), js::to_fixed(x.at[2], 1)),
            },
            OrderTarget::Foe(x) => format!("foe:{}", g.gd[x.foe].callsign),
            OrderTarget::None(_) => String::from("none"),
        };
        let serial = g.serial();
        let item = SquadOrdersLogItem { t: g.time, order: String::from(id_2), who: men.iter().map(|&gd| g.gd[gd].callsign.clone()).collect(), target, auto: None, laid: None, serial };
        g.orders.log.push(item);
    }
    if g.orders.log.len() > 200 { g.orders.log.drain(0..50); }
    if !free && g.orders.by_hand { crate::planlay::lay_given(g, &men); }
    let line = squad_orders_given_line(g, def, &t, &men);
    crate::game::game_msg(g, &line);
    let ack = g.tab.orders_order_def[def].ack.clone().filter(|a| !a.is_empty());
    if let Some(ack) = ack {
        let k = g.orders.acks;
        g.orders.acks += 1.0;
        let lines: Vec<String> = match opts.and_then(|o| o.ack.clone()).filter(|a| !a.is_empty()) {
            Some(own) => vec![own],
            None => {
                let now = g.tab.orders_order_def[def].ack_now.clone().unwrap_or_else(|| ack.clone());
                crate::words::wg(g, &now, &ack)
            }
        };
        let n = men.len() as f64;
        let by = men[js::rem(k, n) as usize];
        let said = lines[js::rem(m::floor(k / n), lines.len() as f64) as usize].clone();
        crate::game::game_say(g, by, &said, None, None, None);
    }
    if g.orders.posture.is_some() {
        let loud = crate::silent::loud_rows().iter().any(|r| r == id_2);
        crate::silent::silent_tool(g, loud);
    }
    let waits = opts.map_or(false, |o| on(o.on_word)) && squad_orders_big(g);
    crate::holdfire::hold_fire_order(g, id_2, Some(crate::holdfire::HoldFireOrderO { waits: Some(waits), silent: Some(was_silent), ..Default::default() }));
    None
}

/// The line the panel shows for an order given (orders.ts SquadOrders.givenLine)
pub fn squad_orders_given_line(g: &mut crate::game::Game, def: OrdersOrderDefId, t: &crate::orders::OrderTarget, men: &[GuardId]) -> String {
    let label = squad_orders_label_of(g, def, t);
    let said = drop_stock_word(&label).replacen("on your mark", "on my mark", 1);
    let who = squad_orders_who_words(g, Some(men));
    let mut door = String::new();
    if let OrderTarget::Door(x) = t {
        let w = crate::orders::door_words(g, x.door);
        if w != "door" { door = format!(" · {}", w); }
    }
    format!("you: {} — {}{}", who, said, door)
}

/// An order asked again of men who already stand on it (orders.ts SquadOrders.again)
pub fn squad_orders_again(g: &mut crate::game::Game, id_2: &str, t: &crate::orders::OrderTarget, men: &[GuardId], lines: &[String]) {
    let Some(def) = g.orders.defs.iter().copied().find(|&d| g.tab.orders_order_def[d].id == id_2) else { return };
    if men.is_empty() || lines.is_empty() { return; }
    let k = g.orders.acks;
    g.orders.acks += 1.0;
    let line = squad_orders_given_line(g, def, t, men);
    crate::game::game_msg(g, &line);
    let by = men[js::rem(k, men.len() as f64) as usize];
    let said = &lines[js::rem(k, lines.len() as f64) as usize];
    crate::game::game_say(g, by, said, None, None, None);
}

/// Hand a man his assignment (orders.ts SquadOrders.assign)
pub fn squad_orders_assign(g: &mut crate::game::Game, gd: GuardId, a: &crate::orders::SquadOrdersAssignA) {
    let p = g.ch[g.gd[gd].ch].pos;
    let t = g.time;
    let was = g.orders.asg.get(gd).copied();
    let fighting = g.gd[gd].script.is_none() && g.gd[gd].state == crate::game::GuardState::Alert;
    crate::control::set_pinned(g, gd, false, "orders");
    crate::control::set_hold(g, gd, false, "orders");
    g.orders.ours.delete(gd);
    let r = a.post.map(|post| squad_orders_route(g, gd, post, a.shy, a.thru, None));
    let via: Vec<V3> = r.as_ref().map_or(Vec::new(), |x| x.via.clone());
    match r.as_ref().and_then(|x| x.how) {
        Some(SquadOrdersRouteHow::Round) => g.cleared.stats.round += 1.0,
        Some(SquadOrdersRouteHow::Through) => g.cleared.stats.through += 1.0,
        Some(SquadOrdersRouteHow::Mouth) => g.cleared.stats.mouth += 1.0,
        None => {}
    }
    let light = match &g.gd[gd].script {
        Some(s) => s.light,
        None => was.and_then(|w| g.tab.orders_assignment[w].light),
    };
    let yield_t = match was { Some(w) if fighting => g.tab.orders_assignment[w].yield_t, _ => -1.0 };
    let goal = if !via.is_empty() { via[0] } else { match a.post { Some(q) => v3::copy(q), None => [p[0], 0.0, p[2]] } };
    let row = Assignment {
        order: a.order.clone(),
        t0: t,
        post: a.post,
        follow: a.follow.unwrap_or(0.0),
        face: a.face,
        watch: a.watch,
        door: a.door,
        strafe: a.strafe,
        stance: a.stance.unwrap_or(crate::character::Stance::LowReady),
        upper: a.upper.unwrap_or(crate::character::UpperMode::None),
        stack_side: a.stack_side.unwrap_or(1.0),
        move_: a.move_.clone(),
        going: a.going.clone(),
        there: a.there.clone(),
        set_line: a.set_line.clone(),
        group: g.orders.group,
        set_said: false,
        goal,
        arrived: false,
        held_set: a.held_set,
        held_fire: a.held_fire,
        arr_t: -1.0,
        moved_t: t,
        light: a.light.or(light),
        low: false,
        yield_t,
        face_now: None,
        search: a.search,
        search_t: a.search_t,
        via,
        anchor: [p[0], 0.0, p[2]],
        mark: [p[0], 0.0, p[2]],
        mark_t: t,
        had: a.had,
        back_n: a.back_n,
        shy: a.shy,
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
        new_leg: true,
    };
    let id = g.tab.orders_assignment.push(row);
    g.orders.asg.set(gd, id);
}

fn post_dist(g: &Game, gd: GuardId, centre: V3) -> f64 {
    let a = *g.orders.asg.get(gd).unwrap();
    v3::dist_xz(g.tab.orders_assignment[a].post.unwrap(), centre)
}

/// Rear security at a door: the last of the file have their eyes spare (orders.ts SquadOrders.spareRear)
pub fn squad_orders_spare_rear(g: &mut crate::game::Game, men: &[GuardId], door: DoorId, lead: Option<GuardId>, from: Option<&JsMap<GuardId, V3>>) -> Vec<GuardId> {
    let live: Vec<GuardId> = men.iter().copied().filter(|&gd| alive(g, gd) && g.orders.asg.get(gd).map_or(false, |&a| g.tab.orders_assignment[a].post.is_some())).collect();
    for &gd in &live {
        let a = *g.orders.asg.get(gd).unwrap();
        g.tab.orders_assignment[a].spare = Some(false);
    }
    if live.len() < 2 { return Vec::new(); }
    let centre = g.dr[door].frame_centre;
    let first = match lead.filter(|l| live.contains(l)) {
        Some(l) => l,
        None => {
            let mut best = live[0];
            for &b in &live[1..] { if post_dist(g, b, centre) < post_dist(g, best, centre) { best = b; } }
            best
        }
    };
    let mut rear: Vec<GuardId> = live.iter().copied().filter(|&gd| gd != first).collect();
    let place = |gd: GuardId| live.iter().position(|&x| x == gd).unwrap() as f64;
    {
        let g2: &Game = &*g;
        js::sort_by(&mut rear, |&a, &b| js::or(post_dist(g2, b, centre) - post_dist(g2, a, centre), place(b) - place(a)));
    }
    rear.truncate(if live.len() >= 4 { 2 } else { 1 });
    for &gd in &rear {
        let a = *g.orders.asg.get(gd).unwrap();
        let row = &mut g.tab.orders_assignment[a];
        row.spare = Some(true);
        if row.from.is_none() {
            row.from = Some(match from.and_then(|f| f.get(gd)) { Some(&q) => q, None => [row.anchor[0], 0.0, row.anchor[2]] });
        }
    }
    rear
}

/// A memory save put back: a ring and a press half made are the hand's, not the saved moment's (orders.ts SquadOrders.loaded)
pub fn squad_orders_loaded(g: &mut crate::game::Game) {
    if g.orders.cmd_ring.is_some() { g.orders.menu = None; g.orders.cmd_ring = None; }
    g.orders.cmd_press = None;
    g.orders.tap_was = None;
}

/// Esc: the menu steps back a page, or is dismissed (orders.ts SquadOrders.escape)
pub fn squad_orders_escape(g: &mut crate::game::Game) -> bool {
    if g.orders.menu.is_none() { return false; }
    if g.orders.cmd_ring.is_some() {
        g.orders.menu = None;
        g.orders.cmd_ring = None;
        g.stats.cmdring.cancelled += 1;
    } else if !squad_orders_page_back(g) {
        g.orders.menu = None;
    }
    g.orders.esc_used = true;
    true
}

/// Open the menu on a target (orders.ts SquadOrders.open)
pub fn squad_orders_open(g: &mut crate::game::Game, target: Option<&crate::orders::OrderTarget>, flat: Option<bool>, part: Option<&[GuardId]>) -> bool {
    let target: OrderTarget = match target { Some(x) => x.clone(), None => { let a = g.aim_point; squad_orders_target_at(g, a) } };
    let flat = flat.unwrap_or(false);
    if !squad_orders_active(g) || g.pl[g.player].down { return false; }
    let rows = if flat { squad_orders_flat_rows(g, &target, part) } else { squad_orders_rows_for(g, &target, None, part) };
    let word_row = squad_orders_word_row_of(g, &target, &rows);
    // (THE MENU'S OWN TARGET IS KNOWN BY IDENTITY: wallcharge.ts keptFor asks `M.target === t` before it keeps a walk for the open menu. A floor target
    //  carries a serial for that, and this is where the one a menu keeps is dealt; the rows above were asked before there was this menu, as in the
    //  TypeScript, so they are not kept. Without it no walk of an open menu was ever kept: the same answers, worked again at every re-read.)
    let mut target = target;
    if let OrderTarget::Floor(f) = &mut target { if f.serial == 0 { f.serial = g.serial(); } }
    g.orders.menu = Some(Menu {
        target,
        men: Vec::new(),
        rows,
        why: Vec::new(),
        sel: 0.0,
        wheel_acc: 0.0,
        read_t: -1.0,
        part: part.map(|x| x.to_vec()),
        split: None,
        page: None,
        word_row,
        on_word: false,
        men_side: None,
        side_key: String::new(),
        flat: if flat { Some(true) } else { None },
    });
    squad_orders_refresh(g);
    let laid = !g.orders.lay.laid.is_empty();
    if let Some(m) = g.orders.menu.as_mut() { if m.word_row && laid { m.on_word = true; } }
    g.orders.menu.is_some()
}

/// (orders.ts SquadOrders.close)
pub fn squad_orders_close(g: &mut crate::game::Game) {
    g.orders.menu = None;
}

/// The open menu turned to a page of it (orders.ts SquadOrders.turn)
pub fn squad_orders_turn(g: &mut crate::game::Game, page: Option<&str>, on_word: Option<bool>) {
    let on_word = on_word.unwrap_or(false);
    let Some(mut m) = g.orders.menu.clone() else { return };
    let from = m.page.clone();
    m.page = page.map(String::from);
    if let Some(up) = g.orders.menu.as_mut() { up.page = m.page.clone(); }
    let rows = squad_orders_rows_for(g, &m.target, page, Some(&m.men));
    let word_row = squad_orders_word_row_of(g, &m.target, &rows);
    let opener = from.as_ref().and_then(|f| g.orders.defs.iter().copied().find(|&d| g.tab.orders_order_def[d].opens.as_ref() == Some(f)));
    let mut sel: Option<usize> = rows.iter().position(|&d| { let o = &g.tab.orders_order_def[d].opens; word(o) && o.as_ref() == from.as_ref() });
    if sel.is_none() {
        if let Some(op) = opener {
            let (key, op_page) = (g.tab.orders_order_def[op].key, g.tab.orders_order_def[op].page.clone());
            if key.is_some() { sel = rows.iter().position(|&d| g.tab.orders_order_def[d].key == key && g.tab.orders_order_def[d].page == op_page); }
        }
    }
    if let Some(up) = g.orders.menu.as_mut() {
        up.rows = rows;
        up.word_row = word_row;
        up.on_word = up.on_word || (word_row && on_word);
        up.sel = sel.map_or(0.0, |i| i as f64);
        up.wheel_acc = 0.0;
    }
    if let Some(r) = g.orders.cmd_ring.as_mut() { r.lit = 0.0; r.armed = false; r.more = Some(false); }
    squad_orders_refresh(g);
}

/// A page of the open menu → the page it was opened from (orders.ts SquadOrders.pageBack)
pub fn squad_orders_page_back(g: &mut crate::game::Game) -> bool {
    let Some(page) = g.orders.menu.as_ref().and_then(|m| m.page.clone()) else { return false };
    let back = g.orders.defs.iter().copied().find(|&d| g.tab.orders_order_def[d].opens.as_ref() == Some(&page)).and_then(|d| g.tab.orders_order_def[d].page.clone());
    squad_orders_turn(g, back.as_deref(), None);
    true
}

/// Give the open menu's row `i` and take the menu down (orders.ts SquadOrders.pick)
pub fn squad_orders_pick(g: &mut crate::game::Game, i: Option<f64>) -> Option<String> {
    let i = match i { Some(x) => x, None => g.orders.menu.as_ref().map_or(-1.0, |m| m.sel) };
    let Some(m) = g.orders.menu.as_mut() else { return Some(String::from("no menu")); };
    if m.word_row && i == m.rows.len() as f64 {
        m.on_word = !m.on_word;
        return Some(String::from(if m.on_word { "on the word" } else { "at once" }));
    }
    let row = row_at(&m.rows, i);
    let page = row.and_then(|d| g.tab.orders_order_def[d].opens.clone()).filter(|p| !p.is_empty());
    if let Some(p) = page {
        squad_orders_turn(g, Some(&p), None);
        return Some(String::from(if g.orders.menu.is_some() { "opened" } else { "nobody to take it" }));
    }
    let there = squad_orders_reread(g);
    let m = g.orders.menu.take();
    if !there { return Some(String::from("nobody to take it")); }
    let Some(m) = m else { return Some(String::from("nobody to take it")); };
    if i < 0.0 || i >= m.rows.len() as f64 { return Some(String::from("cancelled")); }
    let d = m.rows[i as usize];
    if m.on_word && crate::planlay::layable(g, d) { return crate::planlay::lay_order(g, d, &m.target, &m.men); }
    let id = g.tab.orders_order_def[d].id.clone();
    let on_word = on(g.tab.orders_order_def[d].on_word) && m.on_word;
    if m.part.is_some() {
        let opts = GiveOpts { men: Some(m.men.clone()), on_word: if on_word { Some(true) } else { None }, ack: None };
        let target = m.target.clone();
        let mut give = |g: &mut Game| squad_orders_give(g, &id, Some(&target), None, Some(&opts));
        return squad_orders_part_give(g, &m, &mut give);
    }
    g.orders.by_hand = true;
    let opts = GiveOpts { ack: None, on_word: Some(true), men: None };
    let no = squad_orders_give(g, &id, Some(&m.target), None, if on_word { Some(&opts) } else { None });
    g.orders.by_hand = false;
    no
}

/// An order of the ring that adds a goal, given by hand to its men alone (orders.ts SquadOrders.partGive)
pub fn squad_orders_part_give(g: &mut crate::game::Game, m: &crate::orders::Menu, give: &mut dyn FnMut(&mut crate::game::Game) -> Option<String>) -> Option<String> {
    let n = g.orders.group;
    g.orders.by_hand = true;
    let no = give(g);
    g.orders.by_hand = false;
    if g.orders.group != n {
        g.stats.planmode.goals += 1;
        if on(m.split) { g.stats.planmode.split += 1; }
    }
    no
}

/// The open menu's target as its men will work it (orders.ts SquadOrders.menuTarget)
pub fn squad_orders_menu_target(g: &mut crate::game::Game) -> Option<crate::orders::OrderTarget> {
    g.orders.menu.as_ref().map(menu_target_of)
}

/// The open menu's cheap half, re-read against the floor as it now stands (orders.ts SquadOrders.reread)
pub fn squad_orders_reread(g: &mut crate::game::Game) -> bool {
    let Some(t) = g.orders.menu.as_ref().map(|m| m.target.clone()) else { return false };
    if let OrderTarget::None(x) = &t {
        if let Some(man) = x.man {
            if !alive(g, man) { g.orders.menu = None; return false; }
        }
    }
    let mut t = t;
    if let OrderTarget::Door(x) = &mut t {
        let q = g.ch[player_ch(g)].pos;
        x.side = crate::orders::side_of(g, x.door, q);
        if let Some(m) = g.orders.menu.as_mut() { m.target = OrderTarget::Door(*x); }
    }
    if let Some(part) = g.orders.menu.as_ref().and_then(|m| m.part.clone()) {
        let mut men = Vec::with_capacity(part.len());
        for gd in part { if alive(g, gd) && squad_orders_mine(g, gd) { men.push(gd); } }
        let none = men.is_empty();
        if let Some(m) = g.orders.menu.as_mut() { m.men = men; }
        if none { g.orders.menu = None; return false; }
        return true;
    }
    let men = recipients_of(g, &t);
    if let Some(m) = g.orders.menu.as_mut() { m.men = men; }
    true
}

/// … and the whole of it: each row's why-not as well (orders.ts SquadOrders.refresh)
pub fn squad_orders_refresh(g: &mut crate::game::Game) {
    if g.orders.menu.is_none() || !squad_orders_reread(g) { return; }
    let Some(m) = g.orders.menu.clone() else { return };
    let door_up = match &m.target { OrderTarget::Door(x) => if squad_orders_big(g) && !m.men.is_empty() { Some(*x) } else { None }, _ => None };
    if let Some(x) = door_up {
        let mut key = format!("{}|", x.side);
        for (n, &gd) in m.men.iter().enumerate() {
            let c = g.gd[gd].ch;
            let (id, q) = (g.ch[c].id, g.ch[c].pos);
            let side = crate::orders::side_of(g, x.door, q);
            if n > 0 { key.push(','); }
            key.push_str(&format!("{}{}", id, if side > 0.0 { '+' } else { '-' }));
        }
        if key != m.side_key {
            if let Some(up) = g.orders.menu.as_mut() { up.side_key = key; }
            let side = squad_orders_side_for(g, x.door, &m.men, x.side);
            if let Some(up) = g.orders.menu.as_mut() { up.men_side = Some(side); }
        }
    } else if let Some(up) = g.orders.menu.as_mut() {
        up.men_side = None;
        up.side_key = String::new();
    }
    let Some(m) = g.orders.menu.clone() else { return };
    let mt = menu_target_of(&m);
    let rows = squad_orders_menu_rows(g, &m, &mt);
    if rows != m.rows {
        let was = row_at(&m.rows, m.sel);
        if let Some(up) = g.orders.menu.as_mut() { up.rows = rows.clone(); }
        let word_row = squad_orders_word_row_of(g, &m.target, &rows);
        let i: Option<usize> = match was {
            None => None,
            Some(w) if on(m.flat) => rows.iter().position(|&d| d == w),
            Some(w) => {
                let wk = g.tab.orders_order_def[w].key;
                rows.iter().position(|&d| d == w || (g.tab.orders_order_def[d].key.is_some() && g.tab.orders_order_def[d].key == wk))
            }
        };
        let sel = match i { Some(x) => x as f64, None => m::min(m.sel, rows.len() as f64 + if word_row { 1.0 } else { 0.0 }) };
        if let Some(up) = g.orders.menu.as_mut() { up.word_row = word_row; up.sel = sel; }
    }
    let Some(m) = g.orders.menu.clone() else { return };
    let later: Option<Vec<f64>> = if on(m.flat) && !g.orders.cmd_ring.as_ref().map_or(false, |r| on(r.more)) {
        Some(squad_orders_seats(g, &m, None).over)
    } else { None };
    let mut why: Vec<Option<String>> = Vec::with_capacity(m.rows.len());
    for (i, &d) in m.rows.iter().enumerate() {
        if later.as_ref().map_or(false, |l| l.contains(&(i as f64))) { why.push(None); continue; }
        let o = if g.tab.orders_order_def[d].only.is_some() { order_only(g, d, &m.men, &mt) } else { m.men.clone() };
        let to: &[GuardId] = if o.is_empty() { &m.men } else { &o };
        why.push(squad_orders_why_not(g, d, &mt, Some(to)));
    }
    let now = g.time;
    if let Some(up) = g.orders.menu.as_mut() { up.why = why; up.read_t = now; }
}

/// Is the open menu's list stale now (orders.ts SquadOrders.due)
pub fn squad_orders_due(g: &mut crate::game::Game, m: &crate::orders::Menu) -> bool {
    if let Some(p) = &m.page {
        if g.orders.defs.iter().any(|&d| { let r = &g.tab.orders_order_def[d]; r.opens.as_ref() == Some(p) && r.stands_in.is_some() }) { return true; }
    }
    let t = menu_target_of(m);
    squad_orders_menu_rows(g, m, &t) != m.rows
}

/// What the cursor is on, for the HUD; kept in the asker's store while the cursor has not moved. Its rays and memos are asked aside (orders.ts SquadOrders.hover)
pub fn squad_orders_hover(g: &mut crate::game::Game, kept: Option<&mut crate::orders::HoverKept>) -> Option<crate::orders::OrderTarget> {
    let mut kept = kept;
    let (Some(input), Some(cam), Some(canvas)) = (g.orders.view.input, g.orders.view.cam, g.orders.view.canvas) else { return None };
    if !squad_orders_active(g) { return None; }
    let (x, y, t) = (g.tab.input[input].mouse_x, g.tab.input[input].mouse_y, g.time);
    if let Some(c) = kept.as_ref().and_then(|k| k.hov.as_ref()) {
        if c.x == x && c.y == y && t >= c.t && t - c.t < 0.1 { return Some(c.v.clone()); }
    }
    let mut work = crate::workthings::WorkThingsAskAsideOwn::zeroed();
    let mut col = crate::scene_collision::AsideOwn::zeroed();
    let mut world = crate::scene_boxes::BoxWorldRayAsideOwn::zeroed();
    let out: OrderTarget = crate::doors::doors_pos_aside(g, &mut |g: &mut Game| {
        crate::wallcharge::sky_aside(g, &mut |g: &mut Game| {
            crate::workthings::work_things_ask_aside(g, &mut work, &mut |g: &mut Game| {
                crate::scene_collision::aside(g, &mut col, &mut |g: &mut Game| {
                    crate::scene_boxes::box_world_ray_aside(g, &mut world, &mut |g: &mut Game| squad_orders_target_under(g, input, cam, canvas))
                })
            })
        })
    });
    if let Some(k) = kept.as_mut() { k.hov = Some(HoverKeptHov { x, y, t, v: out.clone() }); }
    Some(out)
}

/// The order the quick key gives on a target: its first row (orders.ts SquadOrders.defaultFor)
pub fn squad_orders_default_for(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) -> Option<OrdersOrderDefId> {
    squad_orders_rows_for(g, t, None, None).into_iter().find(|&d| !word(&g.tab.orders_order_def[d].opens))
}

/// What T would make of `who` with the cursor on `point` (orders.ts SquadOrders.whoAfterT)
pub fn squad_orders_who_after_t(g: &mut crate::game::Game, point: Option<&crate::orders::OrderTarget>, dir: Option<f64>) -> crate::orders::Who {
    let dir = dir.unwrap_or(1.0);
    let s = squad_orders_squad(g);
    let man = match point { Some(OrderTarget::None(x)) => x.man, _ => None };
    let k: Option<usize> = match man { Some(gd) if alive(g, gd) => s.iter().position(|&x| x == gd), _ => None };
    let menu_man = match g.orders.menu.as_ref().map(|m| &m.target) { Some(OrderTarget::None(x)) => x.man, _ => None };
    if let Some(k) = k {
        let same = matches!(g.orders.who, Who::Num(n) if n == k as f64);
        return if (same && menu_man.is_none()) || man == menu_man { Who::All } else { Who::Num(k as f64) };
    }
    if menu_man.is_some() { return Who::All; }
    let was = g.orders.who;
    let next = squad_orders_cycle_who(g, Some(dir));
    g.orders.who = was;
    next
}

/// Who the next order goes to is stepped, and a menu that is up is theirs from here on (orders.ts SquadOrders.stepWho)
pub fn squad_orders_step_who(g: &mut crate::game::Game, dir: f64, point: Option<&crate::orders::OrderTarget>) {
    g.orders.who = squad_orders_who_after_t(g, point, Some(dir));
    let Some(m) = g.orders.menu.as_mut() else { return };
    if let OrderTarget::None(x) = &m.target {
        if x.man.is_some() { m.target = OrderTarget::None(OrderTargetNone { at: x.at, man: None }); }
    }
    squad_orders_refresh(g);
}

/// After a tick: whoever asks between ticks is answered off the live input and camera (orders.ts SquadOrders.hostView)
pub fn squad_orders_host_view(g: &mut crate::game::Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View) {
    if g.orders.view.input.is_some() { set_view(g, input, cam, canvas); }
}

/// Once a frame right after the aim is known: T, the press that opens or closes the menu, and the open menu's keys, each taken out of the frame's input (orders.ts SquadOrders.input)
pub fn squad_orders_input(g: &mut crate::game::Game, _dt: f64, input: InputId, cam: CameraId, canvas: crate::host_hud::View) {
    squad_orders_sync(g);
    if g.orders.esc_used { g.orders.esc_used = false; press_delete(g, input, "Escape"); }
    if !squad_orders_active(g) || g.pl[g.player].down {
        g.orders.menu = None;
        g.orders.cmd_ring = None;
        g.orders.cmd_press = None;
        g.orders.tap_was = None;
        g.orders.view.input = None;
        g.orders.view.cam = None;
        g.orders.view.canvas = None;
        return;
    }
    if g.orders.tap_was.is_some() && (press_has(g, input, "Blur") || press_has(g, input, "Thawed")) { g.orders.tap_was = None; }
    set_view(g, input, cam, canvas);
    if !g.orders.lay.laid.is_empty() { crate::planlay::lay_prune(g); }
    if squad_orders_whole(g) {
        g.orders.who = Who::All;
    } else {
        let who_key = g.k.orders.who_key.clone();
        if crate::input::input_hit(g, input, &who_key) {
            let t = squad_orders_target_under(g, input, cam, canvas);
            squad_orders_step_who(g, 1.0, Some(&t));
        }
    }
    crate::planmode::goal_input(g, input, cam, canvas);
    if squad_orders_ring_input(g, input, cam, canvas) { return; }
    let k = g.k.orders.key.clone();
    let key = crate::input::input_hit(g, input, &k);
    let Some(m) = g.orders.menu.clone() else {
        let mut sent = false;
        if !g.orders.lay.laid.is_empty() {
            let lk = crate::planlay::lay_key(g);
            sent = !lk.is_empty() && press_delete(g, input, &lk);
        }
        if sent {
            crate::planlay::lay_send(g);
            return;
        }
        let quick = g.k.orders.quick_key.clone();
        if crate::input::input_hit(g, input, &quick) {
            let t = squad_orders_target_under(g, input, cam, canvas);
            squad_orders_tap_give(g, &t, input, None);
        } else if key && g.pl[g.player].sel.down_t < 0.0 {
            let t = squad_orders_target_under(g, input, cam, canvas);
            squad_orders_open(g, Some(&t), None, None);
        }
        return;
    };
    let esc = crate::backkey::back_take(g, input);
    let blur = press_has(g, input, "Blur");
    let space = press_delete(g, input, crate::consts::INTERACT_KEY);
    let rmb = crate::input::input_rmb_hit(g, input);
    if rmb { unclick(g, input, 4); }
    let lmb = crate::input::input_lmb_hit(g, input);
    if g.time - m.read_t >= g.k.orders.refresh_secs || squad_orders_due(g, &m) {
        squad_orders_refresh(g);
        if g.orders.menu.is_none() { return; }
    }
    let Some(m) = g.orders.menu.clone() else { return };
    let wait_d = squad_orders_wait_digit(g, &m);
    let word_page = squad_orders_word_page(g, &m);
    let (mut digit, mut wait) = (0.0, false);
    for d in 1..=9 {
        if press_delete(g, input, &format!("Digit{}", d)) {
            let d = d as f64;
            if d == wait_d { wait = true; } else if digit == 0.0 { digit = d; }
        }
    }
    let wait_key = g.k.orders.wait_key.clone();
    if !js::truthy(crate::orders::digit_of(&wait_key)) && press_delete(g, input, &wait_key) && m.word_row { wait = true; }
    let wait_alt = g.k.orders.wait_alt.clone();
    let alt_row = m.rows.iter().enumerate().any(|(i, &d)| {
        g.tab.orders_order_def[d].hot_key.as_deref() == Some(wait_alt.as_str()) && !m.why.get(i).map_or(false, word)
    }) || crate::doortools::door_tools_word_key_held(g);
    let alt = (m.word_row || word_page.is_some()) && !alt_row && !wait_alt.is_empty() && press_delete(g, input, &wait_alt);
    let mut d_sel = 0.0;
    if press_delete(g, input, "ArrowDown") { d_sel += 1.0; }
    if press_delete(g, input, "ArrowUp") { d_sel -= 1.0; }
    let mut wheel = 0.0;
    let turned = g.tab.input[input].wheel;
    if js::truthy(turned) {
        let mut acc = m::max(-2.0 * WHEEL_STEP, m::min(2.0 * WHEEL_STEP, m.wheel_acc + turned));
        g.tab.input[input].wheel = 0.0;
        if m::abs(acc) >= WHEEL_STEP {
            wheel = m::sign(acc);
            acc -= wheel * WHEEL_STEP;
        }
        if let Some(up) = g.orders.menu.as_mut() { up.wheel_acc = acc; }
    }
    if lmb && matches!(g.pl[g.player].holding.as_ref().map(|h| h.variant), Some(crate::character::HoldVariant::Arm)) { unclick(g, input, 1); }
    if blur || key || lmb { g.orders.menu = None; return; }
    if esc || rmb {
        if !squad_orders_page_back(g) { g.orders.menu = None; }
        return;
    }
    if alt && !m.word_row {
        if let Some(p) = &word_page { squad_orders_turn(g, Some(p), Some(true)); return; }
    }
    if wait || alt { squad_orders_pick(g, Some(m.rows.len() as f64)); }
    if digit != 0.0 {
        let found = m.rows.iter().position(|&r| g.tab.orders_order_def[r].key == Some(digit));
        if let Some(i) = found { squad_orders_pick(g, Some(i as f64)); return; }
    }
    d_sel += wheel;
    let Some(up) = g.orders.menu.as_mut() else { return };
    let n = up.rows.len() as f64 + if up.word_row { 1.0 } else { 0.0 };
    if js::truthy(d_sel) { up.sel = js::rem(js::rem(up.sel + d_sel, n + 1.0) + (n + 1.0), n + 1.0); }
    let sel = up.sel;
    if space { squad_orders_pick(g, Some(sel)); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_fixed_as_the_language_has_it() {
        // bun: x.toFixed(1) for each
        for (x, s) in [(0.25, "0.3"), (-0.25, "-0.3"), (0.75, "0.8"), (1.25, "1.3"), (0.35, "0.3"), (1.45, "1.4"), (-0.04, "-0.0"), (0.0, "0.0"), (-0.0, "0.0"), (2.5, "2.5"), (12.05, "12.1"), (-12.15, "-12.2"), (100.125, "100.1"), (1e-9, "0.0")] {
            assert_eq!(js::to_fixed(x, 1), s, "{}", x);
        }
    }

    #[test]
    fn the_stock_word_comes_off_as_the_regex_has_it() {
        // bun: s.replace(/ · (\d+ left|uses .+)$/, '').replace('on your mark', 'on my mark')
        for (s, out) in [
            ("frag · 2 left", "frag"),
            ("frag · 2 left · uses a can", "frag · 2 left"),
            ("go · uses a can", "go"),
            ("go · uses ", "go · uses "),
            ("a · b · 12 left", "a · b"),
            ("x · 2 leftover", "x · 2 leftover"),
            ("x · 2 left\n", "x · 2 left\n"),
            ("x · uses a\nb", "x · uses a\nb"),
            ("on your mark on your mark · 3 left", "on my mark on your mark"),
        ] {
            assert_eq!(drop_stock_word(s).replacen("on your mark", "on my mark", 1), out);
        }
    }

    #[test]
    fn a_door_menu_works_from_the_mens_face() {
        let d = OrderTargetDoor { at: [1.0, 0.0, 2.0], door: DoorId(3), side: 1.0 };
        let mut m = Menu::zeroed();
        m.target = OrderTarget::Door(d);
        m.men_side = Some(-1.0);
        match menu_target_of(&m) { OrderTarget::Door(x) => assert_eq!(x.side, -1.0), _ => panic!() }
        m.men_side = None;
        match menu_target_of(&m) { OrderTarget::Door(x) => assert_eq!(x.side, 1.0), _ => panic!() }
    }

    #[test]
    fn a_row_by_a_number() {
        let rows = [OrdersOrderDefId(0), OrdersOrderDefId(1)];
        assert!(row_at(&rows, -1.0).is_none());
        assert!(row_at(&rows, 2.0).is_none());
        assert!(row_at(&rows, 1.0) == Some(OrdersOrderDefId(1)));
    }
}
