pub use ::game_types::rails::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec::Vec};

/// A step's `after` line as its order stands: given (1), or given and waiting on his word (2). (rails.ts said)
pub fn said(a: &crate::scene_scenarios::LessonStepAfter, gave: f64) -> Option<String> {
    if gave == 2.0 { a.word.clone().or_else(|| a.line.clone()) } else { a.line.clone() }
}

/// A lookout as one of the fog's eyes. (rails.ts lookoutEye)
pub fn lookout_eye(id_2: f64, e: crate::rails::LookoutEyeE) -> crate::fogofwar::FogEye {
    crate::fogofwar::FogEye {
        id: id_2,
        pos: e.at,
        look: [m::sin(e.yaw), 0.0, m::cos(e.yaw)],
        at: e.at,
        fov: Some(e.fov.unwrap_or(120.0)),
        near: Some(0.0),
        wand: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene_scenarios::{LessonData, LessonStep, LessonStepAfter, RailsData, RailsDataWaits};

    #[test]
    fn said_takes_the_word_only_at_two() {
        let both = LessonStepAfter { line: Some(String::from("line")), on: None, word: Some(String::from("word")) };
        let no_word = LessonStepAfter { line: Some(String::from("line")), on: None, word: None };
        assert_eq!(said(&both, 1.0).as_deref(), Some("line"));
        assert_eq!(said(&both, 2.0).as_deref(), Some("word"));
        assert_eq!(said(&no_word, 2.0).as_deref(), Some("line"));
    }

    #[test]
    fn lookout_eye_defaults() {
        let e = lookout_eye(STEP_EYE_ID, LookoutEyeE { at: [1.0, 2.0, 3.0], yaw: 0.0, fov: None });
        assert_eq!(e.id, 198.0);
        assert_eq!(e.pos, [1.0, 2.0, 3.0]);
        assert_eq!(e.at, e.pos);
        assert_eq!(e.look, [0.0, 0.0, 1.0]);
        assert_eq!(e.fov, Some(120.0));
        assert_eq!(e.near, Some(0.0));
        assert_eq!(lookout_eye(STEP_EYE_ID, LookoutEyeE { at: [0.0; 3], yaw: 0.0, fov: Some(90.0) }).fov, Some(90.0));
    }

    #[test]
    fn a_card_size_reads_as_the_typescript_wrote_it() {
        assert_eq!(alloc::format!("{CARD_CHARS}"), "260");
    }

}

/// ACTS.move.keys: the walk keys
fn walk_keys() -> Vec<String> { crate::rails::acts().delete(String::from("move")).and_then(|r| r.keys).unwrap_or_default() }
/// rails.ts FREE: /^(Escape|Tab|Enter|NumpadEnter|Blur|Paused|F\d{1,2}|BracketLeft|BracketRight)$/
fn is_free(code: &str) -> bool {
    if matches!(code, "Escape" | "Tab" | "Enter" | "NumpadEnter" | "Blur" | "Paused" | "BracketLeft" | "BracketRight") { return true; }
    let b = code.as_bytes();
    (b.len() == 2 || b.len() == 3) && b[0] == b'F' && b[1..].iter().all(|c| c.is_ascii_digit())
}
/// rails.ts DIGIT: /^Digit[1-5]$/
fn is_digit(code: &str) -> bool {
    let b = code.as_bytes();
    b.len() == 6 && code.starts_with("Digit") && (b'1'..=b'5').contains(&b[5])
}
fn has(a: &JsSet<String>, k: &str) -> bool { a.has(String::from(k)) }
fn names(v: &Option<Vec<String>>, k: &str) -> bool { v.as_ref().map_or(false, |l| l.iter().any(|x| x == k)) }
fn step_at(l: &crate::scene_scenarios::LessonData, i: f64) -> Option<&crate::scene_scenarios::LessonStep> { l.steps.get(i as usize) }
fn word_of(s: &Option<String>) -> bool { s.as_ref().map_or(false, |x| !x.is_empty()) }
/// a man of the squad is under an order whose mark is within wayM of this place (rails.ts:312, 327)
fn squad_post_near(g: &Game, seat: &crate::seats::SeatRef, at: V3) -> bool {
    for gd in rails_squad(g) {
        if let Some(a) = crate::orders::squad_orders_of(g, seat, gd) {
            if let Some(p) = g.tab.orders_assignment[a].post {
                if m::hyp2(p[0] - at[0], p[2] - at[2]) < g.k.rails.way_m { return true; }
            }
        }
    }
    false
}
fn order_of(g: &Game, seat: &crate::seats::SeatRef, gd: GuardId) -> Option<String> {
    let a = crate::orders::squad_orders_of(g, seat, gd)?;
    Some(g.tab.orders_assignment[a].order.clone())
}

/// the last moment of HIS fight (rails.ts Rails.fight)
pub fn rails_fight(g: &mut crate::game::Game) -> f64 {
    use crate::director::EventKind as K;
    let mut n = 0;
    while n < g.events.len() {
        let (time, kind, loud, armed, who) = { let e = &g.events[n]; (e.time, e.kind, e.loud, e.weapon.is_some(), e.who) };
        n += 1;
        if !(time > g.rails.act_t) { continue; }
        let hit = kind == K::Kick || kind == K::Bang || (kind == K::Prop && loud)
            || ((kind == K::Shot || kind == K::GuardShot) && armed && (kind == K::Shot || crate::teams::team_of_id(g, who.unwrap_or(-1.0)) == crate::teams::Team::Operative));
        if hit { g.rails.act_t = time; }
    }
    for k in 0..g.guards.len() {
        let d = g.gd[g.guards[k]].died_at;
        if d >= 0.0 && d > g.rails.act_t && d <= g.time { g.rails.act_t = d; }
    }
    g.rails.act_t
}

/// (rails.ts Rails.constructor)
pub fn rails_new(g: &crate::game::Game) -> crate::rails::Rails { crate::rails::Rails::blank() }

/// the rails record in force (rails.ts Rails.data)
pub fn rails_data(g: &crate::game::Game) -> Option<&crate::scene_scenarios::RailsData> {
    crate::lesson::lesson_data(g).and_then(|l| l.rails.as_ref())
}
/// (rails.ts Rails.live)
pub fn rails_live(g: &crate::game::Game) -> bool { rails_data(g).is_some() }
/// this mission's ring is the flat tree's (rails.ts Rails.flatRing)
pub fn rails_flat_ring(g: &crate::game::Game) -> bool {
    rails_data(g).map_or(false, |d| d.flat_ring.unwrap_or(false))
}
/// the lesson under the frame and the step that is up (rails.ts Rails.lesson)
pub fn rails_lesson(g: &crate::game::Game) -> Option<crate::rails::RailsLesson> {
    let s = rails_at(g)?;
    Some(crate::rails::RailsLesson { l: s.l.clone(), i: s.i })
}
/// the same, borrowed from the level, for a question that holds the game shared
pub struct LessonAt<'a> { pub l: &'a crate::scene_scenarios::LessonData, pub i: f64 }
pub fn rails_at(g: &crate::game::Game) -> Option<LessonAt<'_>> {
    let l = crate::lesson::lesson_data(g)?;
    if l.rails.is_none() { return None; }
    Some(LessonAt { l, i: m::min(crate::lesson::lesson_index(g), l.steps.len() as f64) })
}
fn at_of(s: &crate::rails::RailsLesson) -> LessonAt<'_> { LessonAt { l: &s.l, i: s.i } }
/// the actions that work now, derived from the step that is up (rails.ts Rails.allowed)
pub fn rails_allowed(g: &crate::game::Game) -> Option<JsSet<String>> { rails_at(g).map(|s| allowed_of(&s)) }
fn allowed_of(s: &LessonAt) -> JsSet<String> {
    let mut set = JsSet::new();
    for a in &s.l.rails.as_ref().unwrap().allow { set.add(a.clone()); }
    let mut k = 0;
    while (k as f64) <= s.i && k < s.l.steps.len() {
        let t = &s.l.steps[k];
        if let Some(l) = &t.allow { for a in l { set.add(a.clone()); } }
        if let Some(l) = &t.deny { for a in l { set.delete(a.clone()); } }
        k += 1;
    }
    set
}
/// is this action locked right now (rails.ts Rails.locks)
pub fn rails_locks(g: &crate::game::Game, action: &str) -> bool {
    match rails_allowed(g) { Some(a) => !has(&a, action), None => false }
}
/// has a step taken this back: the last step that names it denies it (rails.ts Rails.shuts)
pub fn rails_shuts(g: &crate::game::Game, action: &str) -> bool { rails_at(g).map_or(false, |s| shuts_of(&s, action)) }
fn shuts_of(s: &LessonAt, action: &str) -> bool {
    let mut off = false;
    let mut k = 0;
    while (k as f64) <= s.i && k < s.l.steps.len() {
        let t = &s.l.steps[k];
        if names(&t.allow, action) { off = false; }
        if names(&t.deny, action) { off = true; }
        k += 1;
    }
    off
}
/// is this action's key kept off the picture (rails.ts Rails.hushes)
pub fn rails_hushes(g: &crate::game::Game, action: &str) -> bool { rails_locks(g, action) }
/// a mission that teaches the wall keeps a walk into one from taking it until that line (rails.ts Rails.locksWall)
pub fn rails_locks_wall(g: &crate::game::Game) -> bool {
    let a = rails_allowed(g);
    let s = rails_at(g);
    match (a, s) {
        (Some(a), Some(s)) => !has(&a, "wall") && s.l.steps.iter().any(|t| names(&t.allow, "wall")),
        _ => false,
    }
}
/// does the step whose deed is done stay up a little longer (rails.ts Rails.settles)
pub fn rails_settles(g: &mut crate::game::Game, l: &crate::scene_scenarios::LessonData, i: f64, done_t: f64) -> bool {
    let e = match step_at(l, i + 1.0).and_then(|t| t.explain.as_ref()) { Some(e) => e, None => return false };
    if !rails_live(g) || e.now.unwrap_or(false) { return false; }
    let t = g.time - done_t;
    t >= 0.0 && (t < g.k.rails.card_beat_secs || (e.quiet.unwrap_or(false) && t < g.k.rails.card_max && g.time - rails_fight(g) < g.k.rails.card_quiet))
}
/// the step teaches the hold: a tap gives nothing, and is noted for the page (rails.ts Rails.tapOff)
pub fn rails_tap_off(g: &mut crate::game::Game) -> bool {
    if !rails_teaches_hold(g) { return false; }
    g.rails.tap_t = g.time;
    g.stats.rails.dropped += 1;
    true
}
/// the step that is up teaches the hold (rails.ts Rails.teachesHold)
pub fn rails_teaches_hold(g: &crate::game::Game) -> bool {
    rails_at(g).map_or(false, |s| step_at(s.l, s.i).map_or(false, |t| word_of(&t.no_tap)))
}
/// a run begun (rails.ts Rails.clean)
pub fn rails_clean(g: &mut crate::game::Game) {
    let r = &mut g.rails;
    r.tap_t = -1e9; r.way_nv = -1.0; r.step_look = None; r.gave_step = -1.0; r.gave = 0.0; r.gave0 = -1e9; r.gave_t = r.gave0; r.act_t = -1e9;
}
/// an order went out: on a step with an `after` line, what the men are doing is the line for a while (rails.ts Rails.noteGiven)
pub fn rails_note_given(g: &mut crate::game::Game, o: &str) {
    let s = match rails_lesson(g) { Some(s) => s, None => return };
    let t = match step_at(&s.l, s.i) { Some(t) => t, None => return };
    let a = match &t.after { Some(a) => a, None => return };
    if g.rails.gave_step != s.i { g.rails.gave_step = s.i; g.rails.gave = 0.0; }
    if o == crate::orders::rows::go::ORDER.id {
        if word_of(&a.word) { g.rails.gave = 2.0; g.rails.gave0 = g.time; g.rails.gave_t = g.time; }
        return;
    }
    if !word_of(&a.line) || a.on.as_ref().map_or(false, |l| !l.iter().any(|x| x == o)) { return; }
    if let Some(at) = t.at {
        if g.k.rails.way_m > 0.0 && !squad_post_near(g, &crate::seats::here(g), at) { return; }
    }
    g.rails.gave = 1.0; g.rails.gave0 = g.time; g.rails.gave_t = g.time;
}
/// this man runs the leg he is on (rails.ts Rails.runs)
pub fn rails_runs(g: &crate::game::Game, seat: &crate::seats::SeatRef, gd: GuardId) -> bool {
    if !g.rails.running || g.gd[gd].unit.is_none() || g.gd[gd].team != crate::teams::Team::Operative || g.gd[gd].state == crate::game::GuardState::Dead { return false; }
    match order_of(g, seat, gd) { Some(o) => !crate::orders::rows::id_is(&o, crate::orders::Is::Follows) && !crate::orders::rows::id_is(&o, crate::orders::Is::Holds), None => false }
}
/// is a man of the squad under an order to the step's mark (rails.ts Rails.onWay)
pub fn rails_on_way(g: &crate::game::Game, seat: &crate::seats::SeatRef, s: Option<&LessonAt>) -> bool {
    let s = match s { Some(s) => s, None => return false };
    let t = match step_at(s.l, s.i) { Some(t) => t, None => return false };
    let at = match t.at { Some(a) if t.on_way.is_some() => a, _ => return false };
    if !(g.k.rails.way_m > 0.0) { return false; }
    squad_post_near(g, seat, at)
}
/// does Space offer a row of this verb here (rails.ts Rails.offers)
pub fn rails_offers(g: &crate::game::Game, verb: &str) -> bool {
    match rails_data(g).and_then(|d| d.rows.as_ref()) {
        None => true,
        Some(r) => r.iter().any(|x| x == verb) && !rails_locks(g, "use"),
    }
}

/// what the gate's `ok` closed over (rails.ts:359)
struct GateOk<'a> { a: &'a JsSet<String>, keys: JsSet<String>, rows: Vec<crate::slotsetup::SetupRow>, menu_up: bool, hand_up: bool, hand_key: bool, enter: bool }
fn gate_ok(c: &GateOk, code: &str) -> bool {
    for r in &c.rows {
        if code.strip_prefix("Digit").map_or(false, |d| d == js::num_str(r.key)) {
            return match r.kind {
                crate::slotsetup::SetupKind::Load => has(c.a, "alt") || has(c.a, "load"),
                crate::slotsetup::SetupKind::Mode => has(c.a, "mode"),
                _ => has(c.a, "light"),
            };
        }
    }
    is_free(code) || has(&c.keys, code) || (c.menu_up && (is_digit(code) || code == "KeyV")) || ((c.hand_up || c.hand_key) && is_digit(code)) || (c.enter && (code == "ShiftLeft" || code == "ShiftRight"))
}

/// whatever the step has not unlocked is taken out of the tick (rails.ts Rails.gate)
pub fn rails_gate(g: &mut crate::game::Game, input: InputId) {
    let a = match rails_allowed(g) { Some(a) => a, None => return };
    let mut keys = JsSet::<String>::new();
    let mut buttons = 0i32;
    let mut wheel = false;
    let acts = crate::rails::acts();
    for x in a.iter() {
        let act = match acts.get(x) { Some(r) => r, None => continue };
        if let Some(l) = &act.keys { for k in l { keys.add(k.clone()); } }
        buttons |= js::to_i32(act.buttons.unwrap_or(0.0));
        wheel = wheel || act.wheel.unwrap_or(false);
    }
    let walk = walk_keys();
    let menu_up = g.orders.menu.is_some() && (has(&a, "menu") || has(&a, "ring") || has(&a, "ringPart"));
    let (shift, enter) = {
        let i = &g.tab.input[input];
        (i.keys.has(String::from("ShiftLeft")) || i.keys.has(String::from("ShiftRight")),
         i.keys.has(String::from("Enter")) || i.pressed.has(String::from("Enter")) || i.keys.has(String::from("NumpadEnter")) || i.pressed.has(String::from("NumpadEnter")))
    };
    let p = g.player;
    let hand_up = has(&a, "use") && g.pl[p].sel.open;
    if g.pl[p].holstered && has(&a, "use") && rails_offers(g, "grab") && !rails_shuts(g, "stalk") { buttons |= 1; }
    if menu_up || hand_up { buttons |= 1; }
    if hand_up { buttons |= 4; }
    let setup = if crate::slotsetup::slot_setup_open(g, &crate::seats::here(g)) { g.pl[p].setup.clone() } else { None };
    let rows = match &setup { Some(s) => crate::slotsetup::setup_rows(g, p, s.gun), None => Vec::new() };
    let hand_key = has(&a, "use") && !hand_up && g.puppet.is_none() && !g.pl[p].down && g.pl[p].takedown.is_none() && g.orders.cmd_ring.is_none() && g.orders.menu.is_none() && {
        let ik = String::from(crate::consts::INTERACT_KEY);
        let (down_t, f_held) = (g.pl[p].sel.down_t, g.pl[p].f_held);
        let i = &g.tab.input[input];
        i.keys.has(ik.clone()) && (down_t >= 0.0 || (i.pressed.has(ik) && !f_held))
    } && crate::interact::running_verb(g, &crate::seats::here(g)).is_empty() && (!rails_locks(g, "whistle") || crate::interact::choosable(&g.nearby));
    let c = GateOk { a: &a, keys, rows, menu_up, hand_up, hand_key, enter };
    let mut dropped = 0;
    let i = &mut g.tab.input[input];
    for k in i.pressed.keys() { if !gate_ok(&c, &k) { i.pressed.delete(k); dropped += 1; } }
    for k in i.keys.keys() { if !gate_ok(&c, &k) { i.keys.delete(k); } }
    // the command key held is the ring: its own action over `order`
    if has(&c.keys, "KeyB") {
        let kb = || String::from("KeyB");
        if shift && !has(&a, "goal") {
            let running = has(&a, "sprint") && walk.iter().any(|k| i.keys.has(k.clone()));
            if !running {
                if i.pressed.has(kb()) || i.keys.has(kb()) { i.keys.delete(String::from("ShiftLeft")); i.keys.delete(String::from("ShiftRight")); }
            }
        }
        // (`ringPart`: only with Shift down. Else the key is seen let go, so the press is a tap, which on a `noTap` step gives nothing and shows the note)
        if !has(&a, "ring") && !(has(&a, "ringPart") && shift) && !i.pressed.has(kb()) { i.keys.delete(kb()); }
    }
    let clicked = js::to_i32(i.clicked);
    if clicked & !buttons != 0 { dropped += 1; }
    i.buttons = (js::to_i32(i.buttons) & buttons) as f64;
    i.clicked = (clicked & buttons) as f64;
    if !wheel && !menu_up && !hand_up { i.wheel = 0.0; }
    let word = i.pressed.has(String::from("KeyF"));
    g.stats.rails.dropped += dropped;
    // (a word that sends nobody is still the step's word given)
    if word {
        let go = rails_lesson(g).map_or(false, |s| step_at(&s.l, s.i).and_then(|t| t.after.as_ref()).map_or(false, |w| word_of(&w.word) && !word_of(&w.line)));
        if go { rails_note_given(g, crate::orders::rows::go::ORDER.id); }
    }
}

/// the step's mark under the cursor (rails.ts Rails.markUnder)
pub fn rails_mark_under(g: &crate::game::Game, now: &crate::view::HostNow) -> Option<V3> {
    let s = rails_lesson(g)?;
    let at = step_at(&s.l, s.i)?.at?;
    let (sx, sy, front) = crate::camera::project(&now.view_proj, at, now.client[0], now.client[1], None, None);
    if !front { return None; }
    if g.k.rails.snap_px > 0.0 && m::hyp2(now.mouse[0] - sx, now.mouse[1] - sy) <= g.k.rails.snap_px { Some(at) } else { None }
}
/// the menu's rows on a tutorial mission: only the orders the level names (rails.ts Rails.orderRows)
pub fn rails_order_rows(g: &crate::game::Game, rows: &[OrdersOrderDefId]) -> Vec<OrdersOrderDefId> {
    let l = if rails_live(g) { rails_data(g).and_then(|d| d.orders.as_ref()) } else { None };
    match l {
        Some(l) => rows.iter().copied().filter(|d| l.iter().any(|x| *x == g.tab.orders_order_def[*d].id)).collect(),
        None => rows.to_vec(),
    }
}
/// the step's own lookout, for as long as the step is up (rails.ts Rails.stepEye)
pub fn rails_step_eye(g: &mut crate::game::Game) -> Option<crate::fogofwar::FogEye> {
    let s = rails_lesson(g)?;
    let e = step_at(&s.l, s.i)?.eye.as_ref()?;
    if g.rails.step_look.map_or(true, |l| l.i != s.i) {
        g.rails.step_look = Some(crate::rails::RailsStepLook { i: s.i, eye: lookout_eye(STEP_EYE_ID, crate::rails::LookoutEyeE { at: e.at, yaw: e.yaw, fov: e.fov }) });
    }
    g.rails.step_look.map(|l| l.eye)
}
/// once a tick: the run's start, the step that came up answered, the spared men stood up (rails.ts Rails.tick)
pub fn rails_tick(g: &mut crate::game::Game, dt: f64) {
    let s = rails_lesson(g);
    g.rails.running = s.as_ref().map_or(false, |s| step_at(&s.l, s.i).and_then(|t| t.on_way.as_ref()).map_or(false, |w| w.run.unwrap_or(false)))
        && rails_on_way(g, &crate::seats::here(g), s.as_ref().map(at_of).as_ref());
    let s = match s {
        Some(s) => s,
        None => return,
    };
    if g.rails.run != Some(g.mission.serial) {
        g.rails.run = Some(g.mission.serial);
        g.rails.answered = -1.0; g.rails.spared.clear();
        g.rails.fresh = 3.0;
        g.rails.log.clear();
        g.rails.orders_seen = g.orders.log.len() as f64;
        rails_clean(g);
        rails_start_program(g);
    } else if g.rails.fresh > 0.0 { g.rails.fresh -= 1.0; }
    if g.rails.answered != s.i { rails_answer(g, &s); }
    // (RAILS.cardBeat: the last moment of HIS fight is kept up to date wherever a card ahead waits for one to go quiet)
    if s.l.steps.iter().enumerate().any(|(k, t)| (k as f64) > s.i && t.explain.as_ref().map_or(false, |e| e.quiet.unwrap_or(false))) { rails_fight(g); }
    // every order the player gives is told to the level's program
    if (g.orders.log.len() as f64) < g.rails.orders_seen { g.rails.orders_seen = g.orders.log.len() as f64; }
    while g.rails.orders_seen < g.orders.log.len() as f64 {
        let o = g.orders.log[g.rails.orders_seen as usize].order.clone();
        rails_note_given(g, &o);
        crate::director::director_signal(g, &alloc::format!("order:{}", o), None);
        g.rails.orders_seen += 1.0;
    }
    // (while a man is still on his way, the step's `after` line stays)
    if js::truthy(g.rails.gave) && g.rails.gave_step == s.i {
        let mut walking = false;
        for gd in rails_squad(g) {
            if let Some(a) = crate::orders::squad_orders_of(g, &crate::seats::here(g), gd) {
                let r = &g.tab.orders_assignment[a];
                if !crate::orders::rows::id_is(&r.order, crate::orders::Is::Holds) && !crate::orders::rows::id_is(&r.order, crate::orders::Is::Follows) && !r.arrived { walking = true; break; }
            }
        }
        if walking { g.rails.gave_t = g.time; }
    }
    rails_hold_waiting(g, &s);
    if step_at(&s.l, s.i).and_then(|t| t.on_way.as_ref()).map_or(false, |w| w.nv == Some(false)) && g.rails.way_nv != s.i && rails_on_way(g, &crate::seats::here(g), Some(&at_of(&s))) {
        g.rails.way_nv = s.i;
        let p = g.player;
        if g.pl[p].nv { g.pl[p].nv = false; rails_note(g, "on the way out: night vision up"); }
    }
    let spared: Vec<(crate::Num, f64)> = g.rails.spared.iter().map(|(k, v)| (k, *v)).collect();
    for (id, t) in spared {
        if g.time >= t {
            g.rails.spared.delete(id);
            let gd = g.guards.iter().copied().find(|x| g.ch[g.gd[*x].ch].id == id.0);
            if let Some(gd) = gd {
                if g.gd[gd].state != crate::game::GuardState::Dead {
                    let line = alloc::format!("{} is up again", g.gd[gd].callsign);
                    rails_note(g, &line);
                }
            }
        }
    }
}
/// the step that is up, answered once: its fog, its cue, its checkpoint (rails.ts Rails.answer)
pub fn rails_answer(g: &mut crate::game::Game, s: &crate::rails::RailsLesson) {
    g.rails.answered = s.i;
    let mut fog = s.l.rails.as_ref().unwrap().fog;
    let mut k = 0;
    while (k as f64) <= s.i && k < s.l.steps.len() {
        if s.l.steps[k].fog.is_some() { fog = s.l.steps[k].fog; }
        k += 1;
    }
    if let Some(f) = fog {
        if g.fog_of_war.on != f {
            g.fog_of_war.on = f;
            rails_note(g, if f { "fog on" } else { "fog off" });
        }
    }
    let t = match step_at(&s.l, s.i) { Some(t) => t, None => return };
    if let Some(cue) = t.cue.as_ref().filter(|c| !c.is_empty()) {
        crate::director::director_signal(g, &alloc::format!("lesson:{}", cue), None);
        g.stats.rails.cues += 1;
        rails_note(g, &alloc::format!("cue '{}'", cue));
    }
    if let Some(cp) = t.checkpoint.as_ref().filter(|c| !c.is_empty()) {
        crate::savestate::checkpoints_keep(g, cp);
        g.stats.rails.kept += 1;
        rails_note(g, &alloc::format!("checkpoint asked: {}", cp));
    }
}
/// (rails.ts Rails.startProgram)
pub fn rails_start_program(g: &mut crate::game::Game) {
    if !crate::director::director_programs(g).iter().any(|x| x == "tutorial") { return; }
    if g.director.run.map_or(false, |r| !g.tab.director_run[r].ended) { crate::director::director_stop(g, Some(false)); }
    let no = crate::director::director_start(g, "tutorial", None);
    g.stats.rails.starts += 1;
    let line = match no.filter(|n| !n.is_empty()) {
        Some(n) => alloc::format!("the level's program did not start: {}", n),
        None => String::from("the level's program started"),
    };
    rails_note(g, &line);
}
/// has the level's program said this beat has played out (rails.ts Rails.cued)
pub fn rails_cued(g: &crate::game::Game, name: &str) -> bool { crate::director::director_signalled(g, &alloc::format!("done:{}", name)) }
/// is the mission still on its steps (rails.ts Rails.holdsEnd)
pub fn rails_holds_end(g: &crate::game::Game) -> bool { rails_at(g).map_or(false, |s| s.i < s.l.steps.len() as f64) }
/// has its last step just gone by (rails.ts Rails.endsNow)
pub fn rails_ends_now(g: &crate::game::Game) -> bool { rails_at(g).map_or(false, |s| s.i >= s.l.steps.len() as f64) }

/// a teammate on a rails level with `respawn` is on a knee instead of down (rails.ts Rails.spare)
pub fn rails_spare(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let d = match rails_data(g) { Some(d) => d, None => return false };
    let respawn = match d.respawn { Some(r) => r, None => return false };
    if g.gd[gd].unit.is_none() || g.gd[gd].team != crate::teams::Team::Operative || g.gd[gd].state == crate::game::GuardState::Dead { return false; }
    let id = crate::Num(g.ch[g.gd[gd].ch].id);
    if let Some(until) = g.rails.spared.get(id) { if g.time < *until { return true; } }
    let t = g.time + if respawn > 0.0 { respawn } else { g.k.rails.spare_secs };
    g.rails.spared.set(id, t);
    g.gd[gd].stagger_until = m::max(g.gd[gd].stagger_until, t);
    g.gd[gd].door_down_until = m::max(g.gd[gd].door_down_until, t);
    g.stats.rails.spared += 1;
    let line = alloc::format!("{} spared a down: up at {}", g.gd[gd].callsign, js::to_fixed(t, 1));
    rails_note(g, &line);
    true
}
/// is this man on his knee right now, spared (rails.ts Rails.isSpared)
pub fn rails_is_spared(g: &crate::game::Game, gd: GuardId) -> bool {
    let id = crate::Num(g.ch[g.gd[gd].ch].id);
    g.rails.spared.get(id).map_or(false, |t| g.time < *t)
}
/// the squad's men still up (rails.ts Rails.squad)
pub fn rails_squad(g: &crate::game::Game) -> Vec<GuardId> {
    let mut out = Vec::new();
    for k in 0..g.guards.len() {
        let x = g.guards[k];
        let r = &g.gd[x];
        if r.unit.is_some() && r.team == crate::teams::Team::Operative && r.state != crate::game::GuardState::Dead { out.push(x); }
    }
    out
}
/// he holds where he is (rails.ts Rails.holdHere)
pub fn rails_hold_here(g: &mut crate::game::Game, gd: GuardId, low: Option<bool>) {
    let low = low.unwrap_or(false);
    let c = g.gd[gd].ch;
    let (p, yaw) = (g.ch[c].pos, g.ch[c].body_yaw);
    let a = crate::orders::SquadOrdersAssignA {
        order: String::from(crate::orders::rows::hold::ORDER.id),
        post: Some([p[0], 0.0, p[2]]),
        face: Some(yaw),
        stance: Some(crate::character::Stance::LowReady),
        upper: Some(crate::character::UpperMode::None),
        going: String::from("holding"),
        there: String::from("holding"),
        crouch: if low { Some(true) } else { None },
        ..Default::default()
    };
    crate::orders::squad_orders_assign(g, gd, &a);
}
/// is the squad still waiting where the mission stood it (rails.ts Rails.waiting)
pub fn rails_waiting(g: &crate::game::Game, s: &crate::rails::RailsLesson) -> bool {
    let r = s.l.rails.as_ref().unwrap();
    if r.waits.is_none() || r.allow.iter().any(|x| x == "order") { return false; }
    let mut k = 0;
    while (k as f64) <= s.i && k < s.l.steps.len() {
        if names(&s.l.steps[k].allow, "order") { return false; }
        k += 1;
    }
    true
}
/// a waiting man on 'fall in' holds his place (rails.ts Rails.holdWaiting)
pub fn rails_hold_waiting(g: &mut crate::game::Game, s: &crate::rails::RailsLesson) {
    if !rails_waiting(g, s) { return; }
    let low = s.l.rails.as_ref().unwrap().waits.as_ref().unwrap().low.unwrap_or(false);
    for gd in rails_squad(g) {
        if order_of(g, &crate::seats::here(g), gd).map_or(false, |o| crate::orders::rows::id_is(&o, crate::orders::Is::Follows)) { rails_hold_here(g, gd, Some(low)); }
    }
}
// ---------------------------------------------------------------- what the HUD is told (the view)
/// a line that names a key: /\[[^\]]+\]/
fn names_key(line: &str) -> bool {
    line.split('[').skip(1).any(|rest| rest.find(']').map_or(false, |end| end > 0))
}
/// the one line on screen (None: the run is over) (rails.ts Rails.prompt)
fn rails_prompt(g: &crate::game::Game, seat: &crate::seats::SeatRef, s: &LessonAt) -> Option<crate::view::RailPrompt> {
    use crate::lesson::{lesson_line, lesson_over, lesson_settling, line_of};
    let of = s.l.steps.len();
    if of == 0 || lesson_over(g, seat) { return None; }
    // (every step done and the mission not yet over: the last line stays up until the mission's own end)
    let up = (s.i as usize) < of;
    let i = m::min(s.i, of as f64 - 1.0) as usize;
    let t = &s.l.steps[i];
    let step_line = || lesson_line(g, &line_of(g, t));
    let cue = matches!(t.done, crate::lesson::LessonTest::Cue { .. });
    let said_now = |a: &crate::scene_scenarios::LessonStepAfter| said(a, g.rails.gave).filter(|l| !l.is_empty());
    let gave_here = g.rails.gave_step == s.i && js::truthy(g.rails.gave);
    let (n, of_n) = (i as u32 + 1, of as u32);
    // (the step's deed is done and it settles before the next step's card: what its men are doing, or nothing. Never a line that asks for a key)
    if up && s.l.steps.get(i + 1).map_or(false, |x| x.explain.is_some()) && lesson_settling(g) {
        let w = if cue { step_line() } else if gave_here { t.after.as_ref().and_then(said_now).map_or(String::new(), |l| lesson_line(g, &l)) } else { String::new() };
        return Some(crate::view::RailPrompt { line: if names_key(&w) { String::new() } else { w }, n, of: of_n, note: None, watch: true });
    }
    // (the squad's ring is up ON THE STEP'S MARK: the line is what to press in it)
    let menu = if up { seat.orders.menu.as_ref() } else { None };
    let flat = menu.map_or(false, |x| x.flat.unwrap_or(false));
    let on = menu.map_or(false, |x| t.at.map_or(true, |at| {
        let p = crate::orders::order_target_at(&x.target);
        m::hyp2(p[0] - at[0], p[2] - at[2]) < MENU_M
    }));
    let ring = if !on { None } else {
        let x = menu.unwrap();
        let l = if flat { t.flat_line.as_ref().or(t.menu_line.as_ref()) } else if x.page.is_none() { t.menu_line.as_ref() } else { t.page_line.as_ref().or(t.menu_line.as_ref()) };
        l.filter(|l| !l.is_empty())
    };
    let d = g.time - g.rails.tap_t;
    let note = if up && menu.is_none() && d >= 0.0 && d < NOTE_SECS { t.no_tap.as_ref().filter(|l| !l.is_empty()).map(|l| lesson_line(g, l)) } else { None };
    // (the order the step asks for has gone out: the line is what the men are doing. `afterSecs` on, the step asks again)
    let since = g.time - g.rails.gave_t;
    let aft = if up && gave_here && g.k.rails.after_secs > 0.0 && since >= 0.0 && since < g.k.rails.after_secs && g.time - g.rails.gave0 < g.k.rails.after_max { t.after.as_ref().and_then(said_now) } else { None };
    // (over a flat ring that line stands whatever is open: that ring's line names ONE digit, and with the order out the row on it may be the order's next half)
    let flat_aft = aft.is_some() && flat;
    let line = match (ring, &aft) {
        (Some(r), _) if !flat_aft => lesson_line(g, r),
        (_, Some(a)) => lesson_line(g, a),
        _ => step_line(),
    };
    // (a line that asks for nothing is drawn as something to watch, with no ring on its place. Never a step's own line that merely names no key)
    let watch = (ring.is_none() || flat_aft) && (aft.is_some() || (up && cue)) && !names_key(&line);
    Some(crate::view::RailPrompt { line, n, of: of_n, note, watch })
}
/// what a seat's HUD is told of the frame over this lesson
pub fn rails_view(g: &crate::game::Game, seat: &crate::seats::SeatRef, s: &LessonAt) -> Option<crate::view::RailsView> {
    let rd = s.l.rails.as_ref()?;
    let t = step_at(&s.l, s.i);
    let over = crate::lesson::lesson_over(g, seat);
    let mut line_men: Vec<f64> = rails_squad(g).into_iter().map(|x| g.ch[g.gd[x].ch].id).collect();
    line_men.push(g.ch[g.pl[seat.player].ch].id);
    let open = allowed_of(s);
    Some(crate::view::RailsView {
        prompt: rails_prompt(g, seat, s),
        card: t.filter(|_| !over).and_then(|t| t.explain.as_ref()).map(|e| crate::view::RailCard { title: e.title.clone(), text: crate::lesson::lesson_line(g, &e.text) }),
        arrow: t.filter(|t| !over && t.at.is_some()).and_then(|t| t.toward),
        line_men,
        rows: rd.rows.as_ref().map(|r| if has(&open, "use") { r.clone() } else { Vec::new() }),
        open: open.iter().collect(),
        orders: rd.orders.clone(),
        stalk_shut: shuts_of(s, "stalk"),
        teaches_hold: t.map_or(false, |t| word_of(&t.no_tap)),
        no_lasers: rd.lasers == Some(false),
    })
}
/// the frame's own log (rails.ts Rails.note)
pub fn rails_note(g: &mut crate::game::Game, s: &str) {
    let line = alloc::format!("{} {}", js::to_fixed(g.time, 2), s);
    g.rails.log.push(line);
    if g.rails.log.len() > 80 { g.rails.log.drain(0..40); }
}
/// combat.ts killGuard's question (rails.ts railsSpare)
pub fn rails_spare_2(g: &mut crate::game::Game, gd: GuardId) -> bool { rails_spare(g, gd) }
/// is this man down as far as an enemy's eye goes (rails.ts railsDown)
pub fn rails_down(g: &crate::game::Game, gd: GuardId) -> bool { rails_live(g) && rails_is_spared(g, gd) }

#[cfg(test)]
mod tests_2 {
    use super::*;
    #[test]
    fn the_pages_own_keys_are_free_and_only_they() {
        assert!(is_free("Escape") && is_free("F1") && is_free("F12") && is_free("BracketLeft"));
        assert!(!is_free("F") && !is_free("F123") && !is_free("Fx") && !is_free("KeyF"));
        assert!(is_digit("Digit1") && is_digit("Digit5"));
        assert!(!is_digit("Digit0") && !is_digit("Digit6") && !is_digit("Digit12"));
    }
}
