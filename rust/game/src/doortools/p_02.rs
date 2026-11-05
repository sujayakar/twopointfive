//! src/game/doortools.ts lines 593 to 970
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- the seams: each waits on the skeleton

/// this.samOf   (doortools.ts:896)
fn seam_sam_of(g: &mut Game) -> Option<PlayerId> {
    todo!("needs: DoorTools.sam_of as Option<PlayerId> (DoorToolsSamOf is an empty struct)")
}
/// this.samOf = g.player   (doortools.ts:896)
fn seam_set_sam_of(g: &mut Game, p: PlayerId) {
    todo!("needs: DoorTools.sam_of as Option<PlayerId> (DoorToolsSamOf is an empty struct)")
}

/// `DoorTools.go_key` holds `null` (no such row) as this: `undefined` is its None, and the word's key is never '' on the page (`wordOnKey` settled)
const GO_NO_ROW: &str = "";

// ---------------------------------------------------------------- where a door is

/// doorPt's arithmetic: `th` the door's closedDir, `h` its hinge
fn frame_pt(th: f64, h: [f64; 2], a: f64, b: f64, side: f64, y: f64) -> V3 {
    let (ux, uz) = (m::cos(th), m::sin(th));
    [h[0] + ux * a - uz * side * b, y, h[1] + uz * a + ux * side * b]
}
/// alongOf's arithmetic
fn frame_along(th: f64, h: [f64; 2], p: V3) -> f64 {
    (p[0] - h[0]) * m::cos(th) + (p[2] - h[1]) * m::sin(th)
}
/// offOf's arithmetic
fn frame_off(th: f64, h: [f64; 2], p: V3, side: f64) -> f64 {
    (m::cos(th) * (p[2] - h[1]) - m::sin(th) * (p[0] - h[0])) * side
}
/// a point in a door's frame: `a` m along hinge → latch, `b` out from the frame line on face `side`   (doortools.ts doorPt)
pub fn door_pt(g: &mut crate::game::Game, d: DoorId, a: f64, b: f64, side: f64, y: Option<f64>) -> V3 {
    frame_pt(g.dr[d].def.closed_dir, g.dr[d].def.hinge, a, b, side, y.unwrap_or(0.0))
}
/// metres along the frame, hinge → latch   (doortools.ts alongOf)
pub fn along_of(g: &mut crate::game::Game, d: DoorId, p: V3) -> f64 {
    frame_along(g.dr[d].def.closed_dir, g.dr[d].def.hinge, p)
}
/// metres off the frame line on face `side` (negative: the other face)   (doortools.ts offOf)
pub fn off_of(g: &mut crate::game::Game, d: DoorId, p: V3, side: f64) -> f64 {
    frame_off(g.dr[d].def.closed_dir, g.dr[d].def.hinge, p, side)
}
/// unlatched and swung past OPEN_RAD   (doortools.ts isOpen)
pub fn is_open(g: &mut crate::game::Game, d: DoorId) -> bool {
    !crate::doors::door_latched(g, d) && m::abs(crate::doors::door_angle(g, d)) >= OPEN_RAD
}
/// the room these cells are, as SKY's key: the newest row of the rooms' table holding them, as a fresh flood is a fresh object (None: no row does)
fn room_of_cells(g: &Game, cells: &JsSet<crate::Num>) -> Option<RoomfightRoomId> {
    let mut found = None;
    for (id, room) in g.tab.roomfight_room.iter() {
        if room.cells.len() == cells.len() && room.cells.iter().eq(cells.iter()) { found = Some(id); }
    }
    found
}
/// is most of a room's floor under the sky (asked once per room): the walk of underSky over the room's cells
fn under_sky_cells(g: &mut Game, cells: &JsSet<crate::Num>) -> bool {
    let key = room_of_cells(g, cells);
    let size = cells.len() as f64;
    if let Some(hit) = key.and_then(|key| g.m.doortools.sky.get(key)) {
        if hit.n == size { return hit.open; }
    }
    let (w, cs) = (g.col.nav.w, g.col.nav.cell);
    let every = m::max(1.0, m::floor(size / 64.0));
    let (mut k, mut of, mut sky) = (0.0, 0.0, 0.0);
    for c in cells.iter() {
        let ci = c.0;
        let at = k;
        k += 1.0;
        if js::truthy(js::rem(at, every)) { continue; }
        let x = (js::rem(ci, w) + 0.5) * cs;
        let z = (js::to_i32(ci / w) as f64 + 0.5) * cs;
        of += 1.0;
        if !crate::scene_collision::solid_blocked(g, [x, 2.0, z], [x, 9.0, z], None) { sky += 1.0; }
    }
    let open = sky * 2.0 > of;
    if let Some(key) = key { g.m.doortools.sky.set(key, crate::doortools::DoortoolsModSKYValue { n: size, open }); }
    open
}
/// is most of a room's floor under the sky   (doortools.ts underSky)
pub fn under_sky(g: &mut crate::game::Game, room: &crate::doortools::UnderSkyRoom) -> bool {
    under_sky_cells(g, &room.cells)
}
/// open floor, not a room: a flood past `searchCells`, or floor under the sky   (doortools.ts openFloor)
pub fn open_floor(g: &mut crate::game::Game, room: &crate::doortools::OpenFloorRoom) -> bool {
    room.cells.len() as f64 > g.k.doortools.search_cells || under_sky_cells(g, &room.cells)
}

// ---------------------------------------------------------------- what the rows' functions share

/// `t` as a door: the door and the face
fn door_of(t: &crate::orders::OrderTarget) -> Option<(DoorId, f64)> {
    match t {
        crate::orders::OrderTarget::Door(x) => Some((x.door, x.side)),
        _ => None,
    }
}
/// S.recipients(t)
fn recipients_of(g: &mut Game, t: &crate::orders::OrderTarget) -> Vec<GuardId> {
    use crate::orders::{OrderTarget as T, SquadOrdersRecipientsT as R};
    let r = match t {
        T::Floor(x) => R::Floor(x.clone()),
        T::Door(x) => R::Door(x.clone()),
        T::None(x) => R::None(x.clone()),
        T::Foe(x) => R::Foe(x.clone()),
    };
    crate::orders::squad_orders_recipients(g, Some(&r), None)
}
/// this.thresholdThen(what, men, t)
fn then_now(g: &mut Game, what: crate::doortools::DoorToolsThresholdThenWhat, men: &[GuardId], t: &crate::orders::OrderTarget) -> Option<String> {
    use crate::doortools::DoorToolsThresholdThenT as R;
    use crate::orders::OrderTarget as T;
    let r = match t {
        T::Floor(x) => R::Floor(x.clone()),
        T::Door(x) => R::Door(x.clone()),
        T::None(x) => R::None(x.clone()),
        T::Foe(x) => R::Foe(x.clone()),
    };
    match door_tools_threshold_then(g, what, men, Some(&r)) {
        crate::doortools::DoorToolsThresholdThen::Text(s) => Some(s),
        crate::doortools::DoorToolsThresholdThen::Arm1(()) => None,
    }
}
/// this.thenOnWord(what, men, t)
fn then_word(g: &mut Game, what: crate::doortools::DoorToolsThenOnWordWhat, men: &[GuardId], t: &crate::orders::OrderTarget) -> Option<String> {
    match door_tools_then_on_word(g, what, men, t) {
        crate::doortools::DoorToolsThenOnWord::Text(s) => Some(s),
        crate::doortools::DoorToolsThenOnWord::Arm1(()) => None,
    }
}
/// this.canThresholdThen(what, t, men)
fn can_then(g: &mut Game, what: crate::doortools::DoorToolsCanThresholdThenWhat, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::doortools::DoorToolsStackable {
    use crate::doortools::{DoorToolsCanThresholdThen as C, DoorToolsCanThresholdThenT as R, DoorToolsStackable as V};
    use crate::orders::OrderTarget as T;
    let r = match t {
        T::Floor(x) => R::Floor(x.clone()),
        T::Door(x) => R::Door(x.clone()),
        T::None(x) => R::None(x.clone()),
        T::Foe(x) => R::Foe(x.clone()),
    };
    match door_tools_can_threshold_then(g, what, Some(&r), men) {
        C::Text(s) => V::Text(s),
        C::Bool(b) => V::Bool(b),
    }
}
/// this.standoffFits(men, t, own)
fn standoff_fits(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget, own: Option<bool>) -> crate::doortools::DoorToolsStackable {
    use crate::doortools::{DoorToolsStackable as V, DoorToolsStandoffFits as F};
    match door_tools_standoff_fits(g, men, t, own) {
        F::Text(s) => V::Text(s),
        F::Bool(b) => V::Bool(b),
    }
}
/// this.startCharge(men, t, byHand)
fn start_charge(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget, by_hand: Option<bool>) -> Option<String> {
    match door_tools_start_charge(g, men, t, by_hand) {
        crate::doortools::DoorToolsStartCharge::Text(s) => Some(s),
        crate::doortools::DoorToolsStartCharge::Arm1(()) => None,
    }
}
/// this.issue(kind, men, t, onWord)
fn issue_kind(g: &mut Game, kind: crate::doortools::DoorToolKind, men: &[GuardId], t: &crate::orders::OrderTarget, on_word: bool) -> Option<String> {
    match door_tools_issue(g, kind, men, t, Some(on_word)) {
        crate::doortools::DoorToolsIssue::Text(s) => Some(s),
        crate::doortools::DoorToolsIssue::Arm1(()) => None,
    }
}
/// !!o?.onWord
fn on_word_of(o: Option<&crate::orders::GiveOpts>) -> bool {
    o.and_then(|o| o.on_word).unwrap_or(false)
}
/// who is behind the door, in the words a man says it in; None: nobody
fn behind_words(g: &mut Game, who: Option<String>) -> Option<crate::doortools::DoorToolsStackable> {
    let who = who.filter(|w| !w.is_empty())?;
    let said = if who == "you" {
        crate::words::wg(g, &String::from("you're behind that door."), &String::from("you are behind that door."))
    } else {
        format!("{} is behind that door.", who)
    };
    Some(crate::doortools::DoorToolsStackable::Text(said))
}
/// men.filter(m => on.has(m) || ops.some(o => o.men.includes(m)))
fn men_in_ops(g: &mut Game, men: &[GuardId], on: &[GuardId], ops: &[DoorOpId]) -> Vec<GuardId> {
    let mut out = Vec::new();
    for &m in men {
        if on.contains(&m) || ops.iter().any(|&o| g.tab.doortools_door_op[o].men.contains(&m)) { out.push(m); }
    }
    out
}
/// a string list of a row
fn strs(xs: &[&str]) -> Vec<String> {
    xs.iter().map(|x| String::from(*x)).collect()
}
/// S.register(def, before): a row is a thing of the orders' table
fn register(g: &mut Game, def: crate::orders::OrderDef, before: Option<&str>) {
    let id = g.tab.orders_order_def.push(def);
    crate::orders::squad_orders_register(g, id, before);
}

// ---------------------------------------------------------------- the rows' functions: the arms of OrderLabel, OrderOnly, OrderValid, OrderIssue, OrderGone, called from orders' match: `Order<Field>::Door<Arm>` is `row_<arm>_<field>`, with the arguments the TypeScript calls the field with

/// held(t)   (doortools.ts:806)
fn row_held(g: &mut Game, t: &crate::orders::OrderTarget) -> bool {
    let men = recipients_of(g, t);
    door_tools_held_plain(g, t, &men)
}
/// known(t)   (doortools.ts:807)
fn row_known(g: &mut Game, t: &crate::orders::OrderTarget) -> String {
    if let Some((d, _)) = door_of(t) {
        if door_tools_entry_verb(g, t) != crate::doortools::DoorToolsEntryVerb::Open {
            return format!(" · {}", door_tools_shut_why(g, d));
        }
    }
    String::new()
}
/// the entry rows' `only`: while they hold it from outside, the men holding   (doortools.ts:809, 813)
pub(crate) fn row_held_or_all_only(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> Vec<GuardId> {
    if door_tools_held_plain(g, t, men) { door_tools_held_men(g, men, t) } else { men.to_vec() }
}
/// (doortools.ts:808)
pub(crate) fn row_open_clear_label(g: &mut Game, t: &crate::orders::OrderTarget) -> String {
    if row_held(g, t) { return String::from("go in and clear · no bang"); }
    let verb = door_tools_entry_verb(g, t);
    format!("{} and clear{}", verb.as_str(), row_known(g, t))
}
/// (doortools.ts:810)
pub(crate) fn row_open_clear_valid(g: &mut Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::doortools::DoorToolsStackable {
    if door_tools_held_plain(g, t, men) { can_then(g, crate::doortools::DoorToolsCanThresholdThenWhat::Step, t, men) } else { door_tools_stackable(g, t, men) }
}
/// (doortools.ts:811)
pub(crate) fn row_open_clear_issue(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget, o: Option<&crate::orders::GiveOpts>) -> Option<String> {
    if door_tools_held_plain(g, t, men) {
        if on_word_of(o) { then_word(g, crate::doortools::DoorToolsThenOnWordWhat::Step, men, t) } else { then_now(g, crate::doortools::DoorToolsThresholdThenWhat::Step, men, t) }
    } else {
        issue_kind(g, crate::doortools::DoorToolKind::OpenClear, men, t, on_word_of(o))
    }
}
/// (doortools.ts:812)
pub(crate) fn row_bang_clear_label(g: &mut Game, t: &crate::orders::OrderTarget) -> String {
    if row_held(g, t) { return format!("bang, then go in · {} left", g.door_tools.bangs); }
    let head = if door_tools_entry_verb(g, t) == crate::doortools::DoorToolsEntryVerb::Open {
        String::from("bang")
    } else {
        format!("{}, bang", door_tools_entry_verb(g, t).as_str())
    };
    format!("{} and clear · {} left", head, g.door_tools.bangs)
}
/// (doortools.ts:814)
pub(crate) fn row_bang_clear_valid(g: &mut Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::doortools::DoorToolsStackable {
    if door_tools_held_plain(g, t, men) { return can_then(g, crate::doortools::DoorToolsCanThresholdThenWhat::Bang, t, men); }
    if g.door_tools.bangs <= 0.0 { return crate::doortools::DoorToolsStackable::Text(String::from("no bangs left.")); }
    let who = match door_of(t) { Some((d, side)) => door_tools_can_behind(g, d, side, men), None => None };
    if let Some(said) = behind_words(g, who) { return said; }
    door_tools_stackable(g, t, men)
}
/// (doortools.ts:818)
pub(crate) fn row_bang_clear_issue(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget, o: Option<&crate::orders::GiveOpts>) -> Option<String> {
    if door_tools_held_plain(g, t, men) {
        if on_word_of(o) { then_word(g, crate::doortools::DoorToolsThenOnWordWhat::Bang, men, t) } else { then_now(g, crate::doortools::DoorToolsThresholdThenWhat::Bang, men, t) }
    } else {
        issue_kind(g, crate::doortools::DoorToolKind::BangClear, men, t, on_word_of(o))
    }
}
/// (doortools.ts:819)
pub(crate) fn row_charge_label(g: &mut Game, t: &crate::orders::OrderTarget) -> String {
    if door_of(t).is_some() && door_tools_loose_on(g, t) { return String::from("stand off · in behind the charge"); }
    if let Some((d, _)) = door_of(t) {
        if let Some(c) = door_tools_charge_on(g, d) {
            return format!("blow the charge{}", door_tools_close_words(g, c));
        }
    }
    format!("breach with a charge · {} left", g.door_tools.charges)
}
/// (doortools.ts:820)
pub(crate) fn row_charge_gone(g: &mut Game, t: &crate::orders::OrderTarget) -> bool {
    match door_of(t) {
        Some((d, _)) => g.dr[d].def.steel.unwrap_or(false) && door_tools_charge_on(g, d).is_none(),
        None => false,
    }
}
/// (doortools.ts:821)
pub(crate) fn row_charge_valid(g: &mut Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::doortools::DoorToolsStackable {
    use crate::doortools::DoorToolsStackable as V;
    let Some((d, _)) = door_of(t) else { return V::Bool(false) };
    if door_tools_loose_on(g, t) { return standoff_fits(g, men, t, Some(true)); }
    if door_tools_charge_on(g, d).is_some() { return V::Bool(true); }
    if g.dr[d].def.steel.unwrap_or(false) { return V::Text(String::from("steel. a charge won't cut it.")); }
    let open = if g.k.doortools.charge_answer { door_tools_known_open(g, d) } else { is_open(g, d) };
    if open { return V::Text(String::from("it's open.")); }
    if g.door_tools.charges <= 0.0 { return V::Text(String::from("no charges left.")); }
    let on_it = g.door_tools.ops.iter().any(|&o| {
        let op = &g.tab.doortools_door_op[o];
        op.kind == crate::doortools::DoorToolKind::Charge && op.door == d && op.phase == crate::doortools::Phase::Place
    });
    if on_it { return V::Text(String::from("already on it.")); }
    standoff_fits(g, men, t, None)
}
/// (doortools.ts:831)
pub(crate) fn row_charge_issue(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget, o: Option<&crate::orders::GiveOpts>) -> Option<String> {
    let Some((d, _)) = door_of(t) else { return Some(String::from("no door")) };
    if door_tools_loose_on(g, t) { return start_charge(g, men, t, Some(true)); }
    if let Some(c) = door_tools_charge_on(g, d) {
        door_tools_trigger(g, Some(c));
        return None;
    }
    start_charge(g, men, t, None)
}
/// (doortools.ts:837)
pub(crate) fn row_frag_clear_label(g: &mut Game, t: &crate::orders::OrderTarget) -> String {
    format!("frag and clear · {} left", g.door_tools.frags)
}
/// (doortools.ts:838)
pub(crate) fn row_frag_clear_valid(g: &mut Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::doortools::DoorToolsStackable {
    use crate::doortools::DoorToolsStackable as V;
    if g.door_tools.frags <= 0.0 { return V::Text(String::from("no frags left.")); }
    let hostage = g.rescue.hostages.iter().any(|&h| g.gd[h].state != crate::game::GuardState::Dead);
    if hostage { return V::Text(String::from("not with hostages on the floor.")); }
    let mate = match door_of(t) { Some((d, side)) => door_tools_frag_behind(g, d, side, men), None => None };
    if let Some(said) = behind_words(g, mate) { return said; }
    door_tools_stackable(g, t, men)
}
/// (doortools.ts:842)
pub(crate) fn row_frag_clear_issue(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget, o: Option<&crate::orders::GiveOpts>) -> Option<String> {
    g.door_tools.frag_next = true;
    let said = issue_kind(g, crate::doortools::DoorToolKind::BangClear, men, t, on_word_of(o));
    g.door_tools.frag_next = false;
    said
}
/// (doortools.ts:859)
pub(crate) fn row_threshold_hold_label(g: &mut Game, t: &crate::orders::OrderTarget) -> String {
    String::from(if row_held(g, t) { "clear from outside · holding it" } else { "clear from outside" })
}
/// (doortools.ts:860)
pub(crate) fn row_threshold_hold_valid(g: &mut Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::doortools::DoorToolsStackable {
    use crate::doortools::{DoorToolsCanThreshold as C, DoorToolsStackable as V};
    if door_tools_held_plain(g, t, men) { return V::Text(String::from("already holding it.")); }
    match door_tools_can_threshold(g, t, men) {
        C::Text(s) => V::Text(s),
        C::Bool(b) => V::Bool(b),
    }
}
/// (doortools.ts:860)
pub(crate) fn row_threshold_hold_issue(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget, o: Option<&crate::orders::GiveOpts>) -> Option<String> {
    match door_tools_threshold_hold(g, men, t, Some(on_word_of(o))) {
        crate::doortools::DoorToolsThresholdHold::Text(s) => Some(s),
        crate::doortools::DoorToolsThresholdHold::Arm1(()) => None,
    }
}
/// `w ? go · w : go`
fn go_words(w: String) -> String {
    if w.is_empty() { String::from("go") } else { format!("go · {}", w) }
}
/// (doortools.ts:902)
pub(crate) fn row_go_small_label(g: &mut Game, t: &crate::orders::OrderTarget) -> String {
    let men = recipients_of(g, t);
    go_words(door_tools_stack_words(g, &men))
}
/// (doortools.ts:903)
pub(crate) fn row_go_small_only(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> Vec<GuardId> {
    let mut on = JsSet::<GuardId>::new();
    for x in door_tools_stack_waits(g, men) {
        for m in x.men { on.add(m); }
    }
    let h = door_tools_frag_held(g, men);
    men_in_ops(g, men, &on.keys(), &h)
}
/// (doortools.ts:904)
pub(crate) fn row_go_small_valid(g: &mut Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::doortools::DoorToolsStackable {
    use crate::doortools::DoorToolsStackable as V;
    if !door_tools_stack_waits(g, men).is_empty() || !door_tools_frag_held(g, men).is_empty() { V::Bool(true) } else { V::Text(String::from("nobody waiting.")) }
}
/// (doortools.ts:904)
pub(crate) fn row_go_small_issue(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget, o: Option<&crate::orders::GiveOpts>) -> Option<String> {
    let only: Vec<crate::doortools::DoorToolsGoMenOnlyItem> = door_tools_stack_waits(g, men)
        .into_iter()
        .map(|x| crate::doortools::DoorToolsGoMenOnlyItem { stack: x.stack, men: x.men })
        .collect();
    door_tools_go_men(g, men, Some(&only))
}
/// (doortools.ts:909)
pub(crate) fn row_go_big_label(g: &mut Game, t: &crate::orders::OrderTarget) -> String {
    let men = recipients_of(g, t);
    go_words(door_tools_waiting_words(g, &men))
}
/// (doortools.ts:910)
pub(crate) fn row_go_big_only(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> Vec<GuardId> {
    let w = door_tools_waiting_men(g, men);
    if !w.is_empty() { return w; }
    let h = door_tools_frag_held(g, men);
    men_in_ops(g, men, &[], &h)
}
/// (doortools.ts:911)
pub(crate) fn row_go_big_valid(g: &mut Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::doortools::DoorToolsStackable {
    use crate::doortools::DoorToolsStackable as V;
    if !door_tools_waiting_any(g, men).is_empty() || !door_tools_frag_held(g, men).is_empty() { V::Bool(true) } else { V::Text(String::from("nobody waiting.")) }
}
/// (doortools.ts:911)
pub(crate) fn row_go_big_issue(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget, o: Option<&crate::orders::GiveOpts>) -> Option<String> {
    door_tools_go_men(g, men, None);
    None
}
/// (doortools.ts:922)
pub(crate) fn row_big_clear_label(g: &mut Game, t: &crate::orders::OrderTarget) -> String {
    let men = recipients_of(g, t);
    format!("{} and clear", door_tools_big_word(g, t, &men).as_str())
}
/// (doortools.ts:923)
pub(crate) fn row_big_clear_gone(g: &mut Game, t: &crate::orders::OrderTarget) -> bool {
    match door_of(t) {
        Some((d, _)) => is_open(g, d) || g.dr[d].def.steel.unwrap_or(false),
        None => false,
    }
}
/// (doortools.ts:924)
pub(crate) fn row_big_clear_valid(g: &mut Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::doortools::DoorToolsStackable {
    use crate::doortools::DoorToolsStackable as V;
    let Some((d, side)) = door_of(t) else { return V::Bool(false) };
    if is_open(g, d) { return V::Text(String::from("it's open.")); }
    if g.dr[d].def.steel.unwrap_or(false) { return V::Text(String::from("steel. no way through.")); }
    let mut carries = false;
    for &m in men {
        if crate::bigkit::carries_hammer(g, m) || (g.gd[m].state != crate::game::GuardState::Dead && crate::kit::kit_big(g, m) == crate::kit::BigKit::Shotgun) {
            carries = true;
            break;
        }
    }
    if !carries { return V::Text(String::from("nobody here carries one.")); }
    if crate::doors::door_locked(g, d) && g.dr[d].lock_id.starts_with("wedge:") {
        let mut hammer = false;
        for &m in men {
            if crate::bigkit::carries_hammer(g, m) { hammer = true; break; }
        }
        if !hammer && door_tools_known_shut(g, d, side) {
            return V::Text(crate::words::wg(g, &String::from("it's wedged. a shell won't open it."), &String::from("wedged. no shell shifts it.")));
        }
    }
    door_tools_stackable(g, t, men)
}
/// (doortools.ts:930)
pub(crate) fn row_big_clear_issue(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget, o: Option<&crate::orders::GiveOpts>) -> Option<String> {
    g.door_tools.big_next = true;
    let said = issue_kind(g, crate::doortools::DoorToolKind::OpenClear, men, t, on_word_of(o));
    g.door_tools.big_next = false;
    said
}

// ---------------------------------------------------------------- the class

/// the menu's switches as one number, to see one move: on the page all four are settled on   (doortools.ts DoorTools.thrSigNow)
pub fn door_tools_thr_sig_now(g: &mut crate::game::Game) -> f64 {
    (1 | 2 | 4 | 8) as f64
}

/// the door tools, and their rows on the menu   (doortools.ts DoorTools.constructor)
pub fn door_tools_new(g: &mut crate::game::Game) -> crate::doortools::DoorTools {
    use crate::orders::{OrderDef, OrderGone as Gone, OrderIssue as Issue, OrderLabel as L, OrderOnly as Only, OrderValid as Valid, TargetKind as K};
    use crate::doortools::TOOLS_PAGE;
    let mut me = crate::doortools::DoorTools::blank();
    me.knobs = g.k.doortools.clone();
    me.charges = g.k.doortools.charges;
    me.bangs = g.k.doortools.bangs;
    me.frags = g.k.frag.squad;
    register(g, OrderDef { id: String::from(TOOLS_PAGE), label: L::Text { text: String::from("door tools") }, targets: vec![K::Door], opens: Some(String::from(TOOLS_PAGE)), issue: Issue::Nothing, ..OrderDef::default() }, None);
    register(g, OrderDef { id: String::from("openClear"), page: Some(String::from(TOOLS_PAGE)), label: L::DoorOpenClear, targets: vec![K::Door], ack: Some(strs(&["on it.", "moving."])), on_word: Some(true),
        only: Some(Only::DoorHeldOrAll), valid: Some(Valid::DoorOpenClear), issue: Issue::DoorOpenClear, ..OrderDef::default() }, None);
    register(g, OrderDef { id: String::from("bangClear"), page: Some(String::from(TOOLS_PAGE)), label: L::DoorBangClear, targets: vec![K::Door], ack: Some(strs(&["bang and clear.", "copy, banging it."])), on_word: Some(true),
        only: Some(Only::DoorHeldOrAll), valid: Some(Valid::DoorBangClear), issue: Issue::DoorBangClear, ..OrderDef::default() }, None);
    register(g, OrderDef { id: String::from("charge"), page: Some(String::from(TOOLS_PAGE)), label: L::DoorCharge, targets: vec![K::Door], ack: Some(strs(&["copy.", "on it."])),
        gone: Some(Gone::DoorCharge), valid: Some(Valid::DoorCharge), issue: Issue::DoorCharge, ..OrderDef::default() }, None);
    let frag_key = g.k.frag.order_key.clone();
    register(g, OrderDef { id: String::from("fragClear"), page: Some(String::from(TOOLS_PAGE)), hot_key: Some(frag_key), page_only: Some(true), label: L::DoorFragClear, targets: vec![K::Door], ack: Some(strs(&["frag and clear.", "copy, fragging it."])), on_word: Some(true),
        valid: Some(Valid::DoorFragClear), issue: Issue::DoorFragClear, ..OrderDef::default() }, None);
    // thrRows writes `this`, and this is not on g yet: it stands there for the call
    core::mem::swap(&mut g.door_tools, &mut me);
    door_tools_thr_rows(g);
    core::mem::swap(&mut g.door_tools, &mut me);
    g.orders.notes.push(crate::orders::OrderNoteFn::DoorToolsHud);
    g.orders.notes.push(crate::orders::OrderNoteFn::DoorToolsWord);
    me
}

/// the row of the door held from outside, the one order of that page on the page's switches   (doortools.ts DoorTools.thrRows)
pub fn door_tools_thr_rows(g: &mut crate::game::Game) {
    use crate::orders::{OrderDef, OrderIssue as Issue, OrderLabel as L, OrderValid as Valid, TargetKind as K};
    g.door_tools.thr_sig = door_tools_thr_sig_now(g);
    for id in ["thresholdHold", "threshold", "outside", "thrStep", "thrBang", "thrLeave", crate::doortools::THR_PAGE] {
        crate::orders::squad_orders_unregister(g, id);
    }
    register(g, OrderDef { id: String::from("thresholdHold"), page: Some(String::from(crate::doortools::TOOLS_PAGE)), label: L::DoorThresholdHold, targets: vec![K::Door],
        ack: Some(strs(&["on the jambs. your call after.", "copy, holding it for you."])), ack_now: Some(strs(&["on the jambs. your call.", "copy, holding it for you."])), on_word: Some(true),
        valid: Some(Valid::DoorThresholdHold), issue: Issue::DoorThresholdHold, ..OrderDef::default() }, Some("fragClear"));
    g.door_tools.thr_key = None;
}

/// spend one (false: none left)   (doortools.ts DoorTools.take)
pub fn door_tools_take(g: &mut crate::game::Game, what: crate::doortools::DoorToolsTakeWhat) -> bool {
    use crate::doortools::DoorToolsTakeWhat as W;
    door_tools_sync(g);
    if what == W::Frag {
        if g.door_tools.frags <= 0.0 { return false; }
        g.door_tools.frags -= 1.0;
        let said = format!("frag spent · {} left", g.door_tools.frags);
        door_tools_note(g, &said);
        return true;
    }
    if what == W::Charge {
        if g.door_tools.charges <= 0.0 { return false; }
        g.door_tools.charges -= 1.0;
    } else {
        if g.door_tools.bangs <= 0.0 { return false; }
        g.door_tools.bangs -= 1.0;
    }
    let said = format!("{} spent · {} charges {} bangs left", what.as_str(), g.door_tools.charges, g.door_tools.bangs);
    door_tools_note(g, &said);
    true
}

/// one back in the bag   (doortools.ts DoorTools.refund)
pub fn door_tools_refund(g: &mut crate::game::Game, what: crate::doortools::DoorToolsRefundWhat) {
    use crate::doortools::DoorToolsRefundWhat as W;
    if what == W::Frag {
        g.door_tools.frags += 1.0;
        let said = format!("frag back · {} left", g.door_tools.frags);
        door_tools_note(g, &said);
        return;
    }
    if what == W::Charge { g.door_tools.charges += 1.0; } else { g.door_tools.bangs += 1.0; }
    let said = format!("{} back · {} charges {} bangs left", what.as_str(), g.door_tools.charges, g.door_tools.bangs);
    door_tools_note(g, &said);
}

/// a new cast is a fresh run; the rows that hang on the cast dealt again   (doortools.ts DoorTools.sync)
pub fn door_tools_sync(g: &mut crate::game::Game) {
    use crate::orders::{OrderDef, OrderGone as Gone, OrderIssue as Issue, OrderLabel as L, OrderOnly as Only, OrderValid as Valid, TargetKind as K, WORD_HEAD};
    use crate::doortools::{BIG_KEY, BIG_ORDER, TOOLS_PAGE};
    let (mut i, mut same, mut at) = (0usize, true, 0usize);
    while at < g.guards.len() {
        let gd = g.guards[at];
        at += 1;
        if crate::orders::squad_orders_mine(g, gd) {
            if g.door_tools.roster.get(i) != Some(&gd) { same = false; }
            i += 1;
        }
    }
    if !same || i != g.door_tools.roster.len() {
        g.door_tools.roster = crate::orders::squad_orders_squad(g);
        door_tools_reset(g);
        g.door_tools.go_key = None;
        g.door_tools.thr_key = None;
        g.door_tools.big_row = None;
    }
    // a man alone: a fresh Player is his new run
    let player = g.player;
    let sam_of = seam_sam_of(g);
    if sam_of != Some(player) {
        let first = sam_of.is_none();
        seam_set_sam_of(g, player);
        if !first && g.door_tools.roster.is_empty() {
            g.door_tools.set.clear();
            g.door_tools.charges += g.door_tools.sam_spent;
        }
        if !first { g.door_tools.sam_spent = 0.0; }
    }
    let n = g.door_tools.roster.len();
    let go_key: Option<String> = if n > 2 { Some(g.k.doortools.blow_key.clone()) } else { None };
    let small = n > 0 && n <= 2;
    let go_want: &str = go_key.as_deref().unwrap_or(GO_NO_ROW);
    if small && g.door_tools.go_key.as_deref() != Some("small") {
        g.door_tools.go_key = Some(String::from("small"));
        let blow = g.k.doortools.blow_key.clone();
        register(g, OrderDef { id: String::from("go"), head: Some(String::from(WORD_HEAD)), hot_key: Some(blow), label: L::DoorGoSmall, targets: vec![K::Floor, K::Door, K::None], listed: Some(Vec::new()), ack: Some(strs(&["going.", "moving."])),
            only: Some(Only::DoorGoSmall), valid: Some(Valid::DoorGoSmall), issue: Issue::DoorGoSmall, ..OrderDef::default() }, None);
    } else if !small && g.door_tools.go_key.as_deref() != Some(go_want) {
        g.door_tools.go_key = Some(String::from(go_want));
        if let Some(key) = &go_key {
            register(g, OrderDef { id: String::from("go"), head: Some(String::from(WORD_HEAD)), hot_key: Some(key.clone()).filter(|k| !k.is_empty()), label: L::DoorGoBig, targets: vec![K::Floor, K::Door, K::None], listed: Some(vec![K::Floor, K::None]), ack: Some(strs(&["going.", "moving."])),
                only: Some(Only::DoorGoBig), valid: Some(Valid::DoorGoBig), issue: Issue::DoorGoBig, ..OrderDef::default() }, None);
        } else {
            crate::orders::squad_orders_unregister(g, "go");
        }
    }
    if g.door_tools.thr_sig != door_tools_thr_sig_now(g) {
        door_tools_thr_rows(g);
        g.door_tools.big_row = None;
    }
    // the big slot's own row, while a man of the squad carries the hammer or the breaching shotgun
    let mut big = false;
    if g.k.bigslot.on {
        let mut k = 0;
        while k < g.door_tools.roster.len() {
            let m = g.door_tools.roster[k];
            k += 1;
            if crate::bigkit::carries_hammer(g, m) || (g.gd[m].state != crate::game::GuardState::Dead && crate::kit::kit_big(g, m) == crate::kit::BigKit::Shotgun) {
                big = true;
                break;
            }
        }
    }
    if Some(big) != g.door_tools.big_row {
        g.door_tools.big_row = Some(big);
        crate::orders::squad_orders_unregister(g, BIG_ORDER);
        if big {
            register(g, OrderDef { id: String::from(BIG_ORDER), page: Some(String::from(TOOLS_PAGE)), hot_key: Some(String::from(BIG_KEY)), label: L::DoorBigClear, targets: vec![K::Door], ack: Some(strs(&["on it.", "breaching."])), on_word: Some(true),
                gone: Some(Gone::DoorBigClear), valid: Some(Valid::DoorBigClear), issue: Issue::DoorBigClear, ..OrderDef::default() }, Some("fragClear"));
        }
    }
    // the key the word's row on a held threshold's page was dealt: that row and the page never come up on the page's switches (outsidePlain), so nothing is registered
    g.door_tools.thr_key = Some(match &go_key { None => g.k.doortools.blow_key.clone(), Some(k) => k.clone() });
}

/// a fresh run: the counts back, nothing set, nothing under way   (doortools.ts DoorTools.reset)
pub fn door_tools_reset(g: &mut crate::game::Game) {
    let mut i = 0;
    while i < g.door_tools.ops.len() {
        let op = g.door_tools.ops[i];
        i += 1;
        door_tools_defuse(g, op);
        door_tools_off_lock(g, op);
    }
    g.door_tools.ops.clear();
    g.door_tools.done.clear();
    g.door_tools.set.clear();
    g.door_tools.words.clear();
    g.door_tools.held.clear();
    g.door_tools.watches.clear();
    g.door_tools.tried.clear();
    g.door_tools.sam_spent = 0.0;
    let n = g.k.doortools.charges;
    g.door_tools.charges = crate::seats::seat_share(g, n);
    let n = g.k.doortools.bangs;
    g.door_tools.bangs = crate::seats::seat_share(g, n);
    let n = g.k.frag.squad;
    g.door_tools.frags = crate::seats::seat_share(g, n);
    g.door_tools.log.clear();
    g.door_tools.stats = crate::doortools::DoorToolsStats::zeroed();
    g.door_tools.seen = None;
    g.door_tools.led = crate::doortools::DoorToolsLed { entries: 0.0, by_seen: 0.0 };
}

/// nobody came with him: the count is his card's, nothing set, nothing spent   (doortools.ts DoorTools.alone)
pub fn door_tools_alone(g: &mut crate::game::Game, n: f64) {
    door_tools_sync(g);
    if !g.door_tools.roster.is_empty() { return; }
    g.door_tools.set.clear();
    g.door_tools.sam_spent = 0.0;
    g.door_tools.charges = m::max(0.0, m::floor(n));
}

/// a checkpoint's charge back on its door, as he set it   (doortools.ts DoorTools.setBack)
pub fn door_tools_set_back(g: &mut crate::game::Game, d: DoorId, side: f64) {
    if crate::doors::door_off(g, d) || door_tools_charge_on(g, d).is_some() { return; }
    let t = g.time;
    let c = g.tab.doortools_set_charge.push(crate::doortools::SetCharge { door: d, side, t, by: String::from(crate::ownblow::SAM_BY) });
    g.door_tools.set.push(c);
}

/// the charge set on this door, if any   (doortools.ts DoorTools.chargeOn)
pub fn door_tools_charge_on(g: &mut crate::game::Game, d: DoorId) -> Option<ChargeId> {
    g.door_tools.set.iter().copied().find(|&c| g.tab.doortools_set_charge[c].door == d)
}

/// can these men stack on that face of that door (the entry orders' menu test)   (doortools.ts DoorTools.stackable)
pub fn door_tools_stackable(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::doortools::DoorToolsStackable {
    use crate::doortools::DoorToolsStackable as V;
    let Some((d, side)) = door_of(t) else { return V::Bool(false) };
    let Some(deal) = door_tools_deal_stack(g, d, side, men) else {
        return V::Text(crate::orders::squad_orders_no_room_words(g, "no room on that door.", &mut |g, _| door_tools_deal_stack(g, d, side, men).is_some()));
    };
    let posts = deal.posts;
    let spots: Vec<V3> = posts.iter().map(|x| x.pos).collect();
    let groups: Vec<f64> = posts.iter().map(|x| x.group.unwrap_or(0.0)).collect();
    let front = g.dr[d].frame_centre;
    for (gd, p) in crate::orders::squad_orders_pair(g, men, &spots, Some(&groups), Some(front)) {
        if let Some(w) = crate::orders::squad_orders_reach(g, gd, p) {
            if !w.is_empty() { return V::Text(w); }
        }
    }
    if !door_tools_known_shut(g, d, side) { return V::Bool(true); }
    if g.dr[d].def.steel.unwrap_or(false) && !is_open(g, d) && door_tools_locked_out(g, d, side) {
        return V::Text(String::from("steel, and locked."));
    }
    if door_tools_frame_held(g, d, side) && !door_tools_pickable(g, d) && !door_tools_big_opens(g, d, men) {
        return V::Text(door_tools_no_boot(g, d));
    }
    V::Bool(true)
}

/// is the door locked against the team from that face   (doortools.ts DoorTools.lockedOut)
pub fn door_tools_locked_out(g: &mut crate::game::Game, d: DoorId, side: f64) -> bool {
    let w = g.dr[d].def.width;
    let from = door_pt(g, d, w / 2.0, 0.6, side, None);
    let ring = crate::orders::squad_orders_ring(g);
    crate::doors::doors_locked_out(g, d, from, Some(&ring))
}

/// locked against the team on the face it opens toward: a boot only drives it onto its frame   (doortools.ts DoorTools.frameHeld)
pub fn door_tools_frame_held(g: &mut crate::game::Game, d: DoorId, side: f64) -> bool {
    if g.dr[d].def.steel.unwrap_or(false) || is_open(g, d) || !door_tools_locked_out(g, d, side) { return false; }
    let w = g.dr[d].def.width;
    let from = door_pt(g, d, w / 2.0, 0.6, side, None);
    crate::doors::doors_boot_holds(g, d, from)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// bits from the TypeScript's own expressions under bun with EXACT.on
    #[test]
    fn a_door_s_frame_has_the_typescript_s_bits() {
        let (th, h) = (0.7, [3.25, -1.5]);
        let p = frame_pt(th, h, 1.1, 0.6, -1.0, 0.45);
        assert_eq!(p[0].to_bits(), BITS_PT[0]);
        assert_eq!(p[1].to_bits(), BITS_PT[1]);
        assert_eq!(p[2].to_bits(), BITS_PT[2]);
        let q = [4.5, 0.0, 2.25];
        assert_eq!(frame_along(th, h, q).to_bits(), BITS_ALONG);
        assert_eq!(frame_off(th, h, q, -1.0).to_bits(), BITS_OFF);
    }
    const BITS_PT: [u64; 3] = [0x4011e95359a80132, 0x3fdccccccccccccd, 0xbff40116c5495453];
    const BITS_ALONG: u64 = 0x400af9967c83995a;
    const BITS_OFF: u64 = 0xc00080ca6c8fbbb0;
}
