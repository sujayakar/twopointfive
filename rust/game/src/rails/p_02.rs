//! src/game/rails.ts lines 213 to 594
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

use crate::scene_scenarios::{LessonData, LessonStep, LessonStepAfter, RailsData};

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
/// /\[[^\]]+\]/.test(s): a line that names a key
fn has_bracket(s: &str) -> bool {
    let b = s.as_bytes();
    for (p, c) in b.iter().enumerate() {
        if *c != b'[' { continue; }
        if let Some(q) = b[p + 1..].iter().position(|x| *x == b']') { if q > 0 { return true; } }
    }
    false
}
fn has(a: &JsSet<String>, k: &str) -> bool { a.has(String::from(k)) }
fn names(v: &Option<Vec<String>>, k: &str) -> bool { v.as_ref().map_or(false, |l| l.iter().any(|x| x == k)) }
fn step_at(l: &LessonData, i: f64) -> Option<&LessonStep> { l.steps.get(i as usize) }
fn word_of(s: &Option<String>) -> bool { s.as_ref().map_or(false, |x| !x.is_empty()) }
/// the distance of the camera the gate is handed: the first seat's, the game's own lens (game.ts:1728 `S[0].cam`)
fn first_cam_distance(g: &mut Game) -> f64 { let s = g.seats[0]; g.tab.cmd_seat[s].cam.distance }
/// `T.done?.is === 'cue'` (rails.ts:488, 499)
fn done_is_cue(t: &LessonStep) -> bool { matches!(t.done, crate::scene_scenarios::LessonTestKind::Cue { .. }) }
fn after_said(a: &LessonStepAfter, gave: f64) -> Option<String> { said(&SaidA { line: a.line.clone(), word: a.word.clone() }, gave) }
fn target_at(t: &crate::orders::OrderTarget) -> V3 {
    match t {
        crate::orders::OrderTarget::Floor(x) => x.at,
        crate::orders::OrderTarget::Door(x) => x.at,
        crate::orders::OrderTarget::None(x) => x.at,
        crate::orders::OrderTarget::Foe(x) => x.at,
    }
}
/// a man of the squad is under an order whose mark is within wayM of this place (rails.ts:312, 327)
fn squad_post_near(g: &mut Game, at: V3) -> bool {
    for gd in rails_squad(g) {
        if let Some(a) = crate::orders::squad_orders_of(g, gd) {
            if let Some(p) = g.tab.orders_assignment[a].post {
                if m::hyp2(p[0] - at[0], p[2] - at[2]) < g.k.rails.way_m { return true; }
            }
        }
    }
    false
}
fn order_of(g: &mut Game, gd: GuardId) -> Option<String> {
    let a = crate::orders::squad_orders_of(g, gd)?;
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
pub fn rails_new(g: &mut crate::game::Game) -> crate::rails::Rails { crate::rails::Rails::blank() }

/// the rails record in force (rails.ts Rails.data)
pub fn rails_data(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::RailsData> {
    if !g.k.rails.on { return None; }
    crate::lesson::lesson_data(g).and_then(|l| l.rails)
}
/// (rails.ts Rails.live)
pub fn rails_live(g: &mut crate::game::Game) -> bool { rails_data(g).is_some() }
/// (rails.ts Rails.skyOn)
pub fn rails_sky_on(g: &mut crate::game::Game) -> bool { rails_data(g).map_or(false, |d| d.sky.is_some()) }
/// this mission's ring is the flat tree's (rails.ts Rails.flatRing)
pub fn rails_flat_ring(g: &mut crate::game::Game) -> bool {
    g.k.rails.flat_ring && rails_data(g).map_or(false, |d| d.flat_ring.unwrap_or(false))
}
/// the lesson under the frame and the step that is up (rails.ts Rails.lesson)
pub fn rails_lesson(g: &mut crate::game::Game) -> Option<crate::rails::RailsLesson> {
    let l = crate::lesson::lesson_data(g)?;
    if l.rails.is_none() || !g.k.rails.on { return None; }
    let i = m::min(crate::lesson::lesson_index(g), l.steps.len() as f64);
    Some(crate::rails::RailsLesson { l, i })
}
/// the actions that work now, derived from the step that is up (rails.ts Rails.allowed)
pub fn rails_allowed(g: &mut crate::game::Game) -> Option<JsSet<String>> {
    let s = rails_lesson(g)?;
    let mut set = JsSet::new();
    for a in &s.l.rails.as_ref().unwrap().allow { set.add(a.clone()); }
    let mut k = 0;
    while (k as f64) <= s.i && k < s.l.steps.len() {
        let t = &s.l.steps[k];
        if let Some(l) = &t.allow { for a in l { set.add(a.clone()); } }
        if let Some(l) = &t.deny { for a in l { set.delete(a.clone()); } }
        k += 1;
    }
    Some(set)
}
/// is this action locked right now (rails.ts Rails.locks)
pub fn rails_locks(g: &mut crate::game::Game, action: &str) -> bool {
    match rails_allowed(g) { Some(a) => !has(&a, action), None => false }
}
/// has a step taken this back: the last step that names it denies it (rails.ts Rails.shuts)
pub fn rails_shuts(g: &mut crate::game::Game, action: &str) -> bool {
    let s = match rails_lesson(g) { Some(s) => s, None => return false };
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
pub fn rails_hushes(g: &mut crate::game::Game, action: &str) -> bool { rails_locks(g, action) }
/// a mission that teaches the wall keeps a walk into one from taking it until that line (rails.ts Rails.locksWall)
pub fn rails_locks_wall(g: &mut crate::game::Game) -> bool {
    let a = rails_allowed(g);
    let s = rails_lesson(g);
    match (a, s) {
        (Some(a), Some(s)) => !has(&a, "wall") && s.l.steps.iter().any(|t| names(&t.allow, "wall")),
        _ => false,
    }
}
/// the card of the step that is up (rails.ts Rails.card)
pub fn rails_card(g: &mut crate::game::Game) -> Option<crate::rails::RailsCard> {
    let s = rails_lesson(g)?;
    let e = step_at(&s.l, s.i)?.explain.as_ref()?;
    if crate::lesson::lesson_over(g) { return None; }
    let text = crate::lesson::lesson_line(g, &e.text);
    Some(crate::rails::RailsCard { title: e.title.clone(), text })
}
/// does the step whose deed is done stay up a little longer (rails.ts Rails.settles)
pub fn rails_settles(g: &mut crate::game::Game, l: &crate::scene_scenarios::LessonData, i: f64, done_t: f64) -> bool {
    let e = match step_at(l, i + 1.0).and_then(|t| t.explain.as_ref()) { Some(e) => e, None => return false };
    if !g.k.rails.card_beat || !rails_live(g) || e.now.unwrap_or(false) { return false; }
    let t = g.time - done_t;
    t >= 0.0 && (t < g.k.rails.card_beat_secs || (e.quiet.unwrap_or(false) && t < g.k.rails.card_max && g.time - rails_fight(g) < g.k.rails.card_quiet))
}
/// where the step's mark points (rails.ts Rails.arrow)
pub fn rails_arrow(g: &mut crate::game::Game) -> Option<V3> {
    let s = rails_lesson(g)?;
    let t = step_at(&s.l, s.i)?;
    let toward = t.toward?;
    t.at?;
    if crate::lesson::lesson_over(g) { return None; }
    Some(toward)
}
/// no teammate shows an aiming laser on this level (rails.ts Rails.noLasers)
pub fn rails_no_lasers(g: &mut crate::game::Game) -> bool { rails_data(g).map_or(false, |d| d.lasers == Some(false)) }
/// the step teaches the hold: a tap gives nothing, and is noted for the page (rails.ts Rails.tapOff)
pub fn rails_tap_off(g: &mut crate::game::Game) -> bool {
    if !rails_teaches_hold(g) { return false; }
    g.rails.tap_t = g.time;
    g.stats.rails.dropped += 1;
    true
}
/// the step that is up teaches the hold (rails.ts Rails.teachesHold)
pub fn rails_teaches_hold(g: &mut crate::game::Game) -> bool {
    rails_lesson(g).map_or(false, |s| step_at(&s.l, s.i).map_or(false, |t| word_of(&t.no_tap)))
}
/// a run begun or a checkpoint put back (rails.ts Rails.clean)
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
    if o == "go" {
        if word_of(&a.word) { g.rails.gave = 2.0; g.rails.gave0 = g.time; g.rails.gave_t = g.time; }
        return;
    }
    if !word_of(&a.line) || a.on.as_ref().map_or(false, |l| !l.iter().any(|x| x == o)) { return; }
    if let Some(at) = t.at {
        if g.k.rails.way_m > 0.0 && !squad_post_near(g, at) { return; }
    }
    g.rails.gave = 1.0; g.rails.gave0 = g.time; g.rails.gave_t = g.time;
}
/// this man runs the leg he is on (rails.ts Rails.runs)
pub fn rails_runs(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if !g.rails.running || g.gd[gd].unit.is_none() || g.gd[gd].team != crate::teams::Team::Operative || g.gd[gd].state == crate::game::GuardState::Dead { return false; }
    match order_of(g, gd) { Some(o) => o != "fallIn" && o != "hold", None => false }
}
/// is a man of the squad under an order to the step's mark (rails.ts Rails.onWay)
pub fn rails_on_way(g: &mut crate::game::Game, s: Option<&crate::rails::RailsOnWayS>) -> bool {
    let s = match s { Some(s) => s, None => return false };
    let t = match step_at(&s.l, s.i) { Some(t) => t, None => return false };
    let at = match t.at { Some(a) if t.on_way.is_some() => a, _ => return false };
    if !(g.k.rails.way_m > 0.0) { return false; }
    squad_post_near(g, at)
}
/// does Space offer a row of this verb here (rails.ts Rails.offers)
pub fn rails_offers(g: &mut crate::game::Game, verb: &str) -> bool {
    match rails_data(g).and_then(|d| d.rows) {
        None => true,
        Some(r) => r.iter().any(|x| x == verb) && !rails_locks(g, "use"),
    }
}

/// what the gate's `ok` closed over (rails.ts:359)
struct GateOk<'a> { a: &'a JsSet<String>, keys: JsSet<String>, rows: Vec<crate::slotsetup::SetupRow>, menu_up: bool, hand_up: bool, hand_key: bool, enter: bool }
fn gate_ok(c: &GateOk, code: &str) -> bool {
    for r in &c.rows {
        if code.strip_prefix("Digit").map_or(false, |d| d == format!("{}", r.key)) {
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
pub fn rails_gate(g: &mut crate::game::Game, input: InputId, dt: f64) {
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
    let menu_up = g.orders.menu.is_some() && (has(&a, "menu") || has(&a, "ring"));
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
    let setup = if crate::slotsetup::slot_setup_left(g) > 0.03 { g.pl[p].setup.clone() } else { None };
    let rows = match &setup { Some(s) => crate::slotsetup::setup_rows(g, p, s.gun), None => Vec::new() };
    let hand_key = has(&a, "use") && !hand_up && g.puppet.is_none() && !g.pl[p].down && g.pl[p].takedown.is_none() && g.orders.cmd_ring.is_none() && g.orders.menu.is_none() && {
        let ik = String::from(crate::consts::INTERACT_KEY);
        let (down_t, f_held) = (g.pl[p].sel.down_t, g.pl[p].f_held);
        let i = &g.tab.input[input];
        i.keys.has(ik.clone()) && (down_t >= 0.0 || (i.pressed.has(ik) && !f_held))
    } && crate::interact::running_verb(g).is_empty() && (!rails_locks(g, "whistle") || crate::interact::choosable(&g.nearby));
    let mut c = GateOk { a: &a, keys, rows, menu_up, hand_up, hand_key, enter };
    let mut dropped = 0;
    let shift_tap = g.k.rails.shift_tap;
    // the sky seat's walk keys are the view's, never his feet's: read here, then gone from the tick
    if rails_sky_on(g) {
        if has(&a, "pan") { rails_pan(g, input, dt); }
        rails_keep(g, input, dt);
        let i = &mut g.tab.input[input];
        for k in &walk { i.keys.delete(k.clone()); i.pressed.delete(k.clone()); }
        for k in ["KeyW", "KeyA", "KeyS", "KeyD"] { c.keys.delete(String::from(k)); }
    }
    let i = &mut g.tab.input[input];
    for k in i.pressed.keys() { if !gate_ok(&c, &k) { i.pressed.delete(k); dropped += 1; } }
    for k in i.keys.keys() { if !gate_ok(&c, &k) { i.keys.delete(k); } }
    // the command key held is the ring: its own action over `order`
    if has(&c.keys, "KeyB") {
        let kb = || String::from("KeyB");
        if shift && !has(&a, "goal") {
            let running = has(&a, "sprint") && walk.iter().any(|k| i.keys.has(k.clone()));
            if shift_tap && !running {
                if i.pressed.has(kb()) || i.keys.has(kb()) { i.keys.delete(String::from("ShiftLeft")); i.keys.delete(String::from("ShiftRight")); }
            } else if !shift_tap {
                if i.pressed.delete(kb()) { dropped += 1; }
                i.keys.delete(kb());
            }
        }
        if !has(&a, "ring") && !i.pressed.has(kb()) { i.keys.delete(kb()); }
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
        if go { rails_note_given(g, "go"); }
    }
}

/// the step's mark under the cursor (rails.ts Rails.markUnder)
pub fn rails_mark_under(g: &mut crate::game::Game, input: InputId, canvas: crate::rails::RailsMarkUnderCanvas) -> Option<V3> {
    let s = rails_lesson(g)?;
    let at = step_at(&s.l, s.i)?.at?;
    let (sx, sy, front) = crate::camera::follow_camera_project(g, at, canvas.client_width, canvas.client_height, None, None);
    if !front { return None; }
    let (mx, my) = (g.tab.input[input].mouse_x, g.tab.input[input].mouse_y);
    if g.k.rails.snap_px > 0.0 && m::hyp2(mx - sx, my - sy) <= g.k.rails.snap_px { Some(at) } else { None }
}
/// the menu's rows on a tutorial mission: only the orders the level names (rails.ts Rails.orderRows)
pub fn rails_order_rows(g: &mut crate::game::Game, rows: &[crate::rails::RailsOrderRowsRowsItem]) -> Vec<crate::rails::RailsOrderRowsRowsItem> {
    let l = if rails_live(g) { rails_data(g).and_then(|d| d.orders) } else { None };
    match l {
        Some(l) => rows.iter().filter(|d| l.iter().any(|x| *x == d.id)).cloned().collect(),
        None => rows.to_vec(),
    }
}
/// the sky seat's lookout (rails.ts Rails.eye)
pub fn rails_eye(g: &mut crate::game::Game) -> Option<crate::fogofwar::FogEye> {
    let e = rails_data(g)?.sky?.eye?;
    if g.rails.lookout.is_none() {
        g.rails.lookout = Some(lookout_eye(LOOKOUT_ID, crate::rails::LookoutEyeE { at: e.at, yaw: e.yaw, fov: e.fov }));
    }
    g.rails.lookout
}
/// (rails.ts Rails.eyeFirst)
pub fn rails_eye_first(g: &mut crate::game::Game) -> bool { g.k.rails.eye_first }
/// (rails.ts Rails.eyeView)
pub fn rails_eye_view(g: &mut crate::game::Game) -> bool { g.k.rails.eye_view }
/// the step's own lookout, for as long as the step is up (rails.ts Rails.stepEye)
pub fn rails_step_eye(g: &mut crate::game::Game) -> Option<crate::fogofwar::FogEye> {
    let s = rails_lesson(g)?;
    let e = step_at(&s.l, s.i)?.eye.as_ref()?;
    if g.rails.step_look.map_or(true, |l| l.i != s.i) {
        g.rails.step_look = Some(crate::rails::RailsStepLook { i: s.i, eye: lookout_eye(STEP_EYE_ID, crate::rails::LookoutEyeE { at: e.at, yaw: e.yaw, fov: e.fov }) });
    }
    g.rails.step_look.map(|l| l.eye)
}
/// between ticks, with the host's camera: the mission again opens on the first run's view (rails.ts Rails.hostView)
pub fn rails_host_view(g: &mut crate::game::Game) {
    if !rails_sky_on(g) || !g.rails.sky.parked { return; }
    match g.rails.sky.view {
        None => { g.rails.sky.view = g.director.cam.as_ref().map(|c| crate::rails::RailsSkyView { yaw: c.yaw, pitch: c.pitch, dist: c.desired_distance }); }
        Some(v) => if g.rails.sky.again {
            if let Some(c) = g.director.cam.as_mut() { c.yaw = v.yaw; c.pitch = v.pitch; c.desired_distance = v.dist; c.distance = v.dist; }
        },
    }
    g.rails.sky.again = false;
}

/// once a tick: the run's start, the step that came up answered, the spared men stood up (rails.ts Rails.tick)
pub fn rails_tick(g: &mut crate::game::Game, dt: f64) {
    let s = rails_lesson(g);
    let way = |s: &crate::rails::RailsLesson| crate::rails::RailsOnWayS { l: s.l.clone(), i: s.i };
    g.rails.running = g.k.rails.way_run
        && s.as_ref().map_or(false, |s| step_at(&s.l, s.i).and_then(|t| t.on_way.as_ref()).map_or(false, |w| w.run.unwrap_or(false)))
        && { let w = s.as_ref().map(way); rails_on_way(g, w.as_ref()) };
    let s = match s {
        Some(s) => s,
        None => { if g.rails.sky.parked { rails_unpark(g); } return; }
    };
    if g.rails.run != Some(g.mission.serial) {
        g.rails.run = Some(g.mission.serial);
        g.rails.answered = -1.0; g.rails.back = false; g.rails.spared.clear();
        if g.rails.sky.parked { rails_unpark(g); }
        g.rails.sky.again = true; g.rails.fresh = 3.0; g.rails.lookout = None;
        g.rails.sky.off = [0.0, 0.0, 0.0]; g.rails.sky.mid = None; g.rails.sky.mid_t = -1.0;
        g.rails.log.clear();
        g.rails.orders_seen = g.orders.log.len() as f64;
        g.rails.moved = false;
        rails_clean(g);
        rails_start_program(g, "start");
    } else if g.rails.fresh > 0.0 { g.rails.fresh -= 1.0; }
    let ans = crate::rails::RailsAnswerS { l: s.l.clone(), i: s.i };
    if g.rails.back {
        g.rails.back = false;
        rails_clean(g);
        let first = s.l.steps.iter().position(|t| names(&t.allow, "order")).map_or(-1.0, |k| k as f64);
        g.rails.moved = s.i > first;
        rails_start_program(g, "checkpoint");
        g.rails.answered = -1.0;
        rails_answer(g, &ans, true);
    }
    let rd = s.l.rails.as_ref().unwrap();
    if rails_sky_on(g) && !g.rails.sky.parked { rails_park(g, rd); }
    if g.rails.answered != s.i { rails_answer(g, &ans, false); }
    // (RAILS.cardBeat: the last moment of HIS fight is kept up to date wherever a card ahead waits for one to go quiet)
    if g.k.rails.card_beat && s.l.steps.iter().enumerate().any(|(k, t)| (k as f64) > s.i && t.explain.as_ref().map_or(false, |e| e.quiet.unwrap_or(false))) { rails_fight(g); }
    // every order the player gives is told to the level's program
    if (g.orders.log.len() as f64) < g.rails.orders_seen { g.rails.orders_seen = g.orders.log.len() as f64; }
    while g.rails.orders_seen < g.orders.log.len() as f64 {
        let o = g.orders.log[g.rails.orders_seen as usize].order.clone();
        if !(rails_sky_on(g) && o == "fallIn") {
            g.rails.moved = true;
            rails_note_given(g, &o);
            crate::director::director_signal(g, &format!("order:{}", o), None);
        }
        g.rails.orders_seen += 1.0;
    }
    // (while a man is still on his way, the step's `after` line stays)
    if js::truthy(g.rails.gave) && g.rails.gave_step == s.i {
        let mut walking = false;
        for gd in rails_squad(g) {
            if let Some(a) = crate::orders::squad_orders_of(g, gd) {
                let r = &g.tab.orders_assignment[a];
                if r.order != "hold" && r.order != "fallIn" && !r.arrived { walking = true; break; }
            }
        }
        if walking { g.rails.gave_t = g.time; }
    }
    // (the sky seat: there is nobody to fall in on. Such a man holds where he is)
    if rails_sky_on(g) {
        for gd in rails_squad(g) {
            if order_of(g, gd).as_deref() == Some("fallIn") {
                let low = rd.sky.as_ref().unwrap().low.unwrap_or(false) && !g.rails.moved;
                rails_hold_here(g, gd, Some(low));
            }
        }
    }
    rails_hold_waiting(g, &crate::rails::RailsHoldWaitingS { l: s.l.clone(), i: s.i });
    if step_at(&s.l, s.i).and_then(|t| t.on_way.as_ref()).map_or(false, |w| w.nv == Some(false)) && g.rails.way_nv != s.i && rails_on_way(g, Some(&way(&s))) {
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
                    let line = format!("{} is up again", g.gd[gd].callsign);
                    rails_note(g, &line);
                }
            }
        }
    }
}
/// the step that is up, answered once: its fog, its cue, its checkpoint (rails.ts Rails.answer)
pub fn rails_answer(g: &mut crate::game::Game, s: &crate::rails::RailsAnswerS, again: bool) {
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
        crate::director::director_signal(g, &format!("lesson:{}", cue), None);
        g.stats.rails.cues += 1;
        rails_note(g, &format!("cue '{}'{}", cue, if again { " (again)" } else { "" }));
    }
    if !again {
        if let Some(cp) = t.checkpoint.as_ref().filter(|c| !c.is_empty()) {
            crate::savestate::checkpoints_keep(g, cp);
            g.stats.rails.kept += 1;
            rails_note(g, &format!("checkpoint asked: {}", cp));
        }
    }
}
/// (rails.ts Rails.startProgram)
pub fn rails_start_program(g: &mut crate::game::Game, why: &str) {
    if !crate::director::director_programs(g).iter().any(|x| x == "tutorial") { return; }
    if g.director.run.as_ref().map_or(false, |r| !r.ended) { crate::director::director_stop(g, Some(false)); }
    let no = crate::director::director_start(g, "tutorial", None);
    g.stats.rails.starts += 1;
    let line = match no.filter(|n| !n.is_empty()) {
        Some(n) => format!("the level's program did not start ({}): {}", why, n),
        None => format!("the level's program started ({})", why),
    };
    rails_note(g, &line);
}
/// a checkpoint of this run was put back (rails.ts Rails.putBack)
pub fn rails_put_back(g: &mut crate::game::Game) {
    if !rails_live(g) { return; }
    g.rails.back = true;
    g.stats.rails.put_back += 1;
}
/// an authored checkpoint was taken with the program stopped for it (rails.ts Rails.resume)
pub fn rails_resume(g: &mut crate::game::Game) { if rails_live(g) { g.rails.back = true; } }
/// has the level's program said this beat has played out (rails.ts Rails.cued)
pub fn rails_cued(g: &mut crate::game::Game, name: &str) -> bool { crate::director::director_signalled(g, &format!("done:{}", name)) }
/// is the mission still on its steps (rails.ts Rails.holdsEnd)
pub fn rails_holds_end(g: &mut crate::game::Game) -> bool { rails_lesson(g).map_or(false, |s| s.i < s.l.steps.len() as f64) }
/// has its last step just gone by (rails.ts Rails.endsNow)
pub fn rails_ends_now(g: &mut crate::game::Game) -> bool { rails_lesson(g).map_or(false, |s| s.i >= s.l.steps.len() as f64) }

/// the one line on screen (rails.ts Rails.prompt)
pub fn rails_prompt(g: &mut crate::game::Game) -> Option<crate::rails::RailsPrompt> {
    let s = rails_lesson(g)?;
    let of = s.l.steps.len() as f64;
    if of == 0.0 { return None; }
    let up = s.i < of;
    if if up { crate::lesson::lesson_objective(g).is_none() } else { crate::lesson::lesson_over(g) } { return None; }
    let i = m::min(s.i, of - 1.0);
    let t = &s.l.steps[i as usize];
    // (the squad's ring is up ON THE STEP'S MARK: the line is what to press in it)
    let menu = if up { g.orders.menu.clone() } else { None };
    let on = menu.as_ref().map_or(false, |x| match t.at {
        None => true,
        Some(at) => { let c = target_at(&x.target); m::hyp2(c[0] - at[0], c[2] - at[2]) < MENU_M }
    });
    let ring: Option<String> = if on {
        let x = menu.as_ref().unwrap();
        if x.flat.unwrap_or(false) { t.flat_line.clone().or_else(|| t.menu_line.clone()) }
        else if x.page.is_none() { t.menu_line.clone() }
        else { t.page_line.clone().or_else(|| t.menu_line.clone()) }
    } else { None };
    let gave_here = g.rails.gave_step == s.i && js::truthy(g.rails.gave);
    // (RAILS.cardBeat: the step's deed is done and it settles before the next step's card: what its men are doing, or nothing)
    if up && g.k.rails.card_beat && step_at(&s.l, s.i + 1.0).map_or(false, |n| n.explain.is_some()) && crate::lesson::lesson_settling(g) {
        let w = if done_is_cue(t) { crate::lesson::step_line(g, t) }
            else if let (Some(a), true) = (&t.after, gave_here) { let l = after_said(a, g.rails.gave).unwrap_or_default(); crate::lesson::lesson_line(g, &l) }
            else { String::new() };
        return Some(crate::rails::RailsPrompt { line: if has_bracket(&w) { String::new() } else { w }, n: i + 1.0, of, note: None, watch: Some(true) });
    }
    let d = g.time - g.rails.tap_t;
    let note = match &t.no_tap {
        Some(n) if up && !n.is_empty() && menu.is_none() && d >= 0.0 && d < NOTE_SECS => Some(crate::lesson::lesson_line(g, n)),
        _ => None,
    };
    // (the order the step asks for has gone out: the line is what the men are doing; `afterSecs` on, the step asks again)
    let since = g.time - g.rails.gave_t;
    let aft = if up && gave_here && g.k.rails.after_secs > 0.0 && since >= 0.0 && since < g.k.rails.after_secs && g.time - g.rails.gave0 < g.k.rails.after_max {
        t.after.as_ref().and_then(|a| after_said(a, g.rails.gave))
    } else { None };
    let aft = aft.filter(|x| !x.is_empty());
    let ring = ring.filter(|x| !x.is_empty());
    // (…and over a FLAT ring it is the line whatever is open)
    let flat_aft = aft.is_some() && menu.as_ref().map_or(false, |x| x.flat.unwrap_or(false));
    let line = match (&ring, &aft) {
        (Some(r), _) if !flat_aft => crate::lesson::lesson_line(g, r),
        (_, Some(a)) => crate::lesson::lesson_line(g, a),
        _ => crate::lesson::step_line(g, t),
    };
    // (a line that asks for nothing is drawn as something to watch)
    let watch = (ring.is_none() || flat_aft) && (aft.is_some() || (up && done_is_cue(t))) && !has_bracket(&line);
    Some(crate::rails::RailsPrompt { line, n: i + 1.0, of, note, watch: if watch { Some(true) } else { None } })
}

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
    let line = format!("{} spared a down: up at {:.1}", g.gd[gd].callsign, t);
    rails_note(g, &line);
    true
}
/// is this man on his knee right now, spared (rails.ts Rails.isSpared)
pub fn rails_is_spared(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let id = crate::Num(g.ch[g.gd[gd].ch].id);
    g.rails.spared.get(id).map_or(false, |t| g.time < *t)
}
/// where your own men stand, for the big line to keep clear of (rails.ts Rails.lineMen)
pub fn rails_line_men(g: &mut crate::game::Game) -> Vec<crate::rails::RailsLineMenItem> {
    let mut out = Vec::new();
    if !g.k.rails.line_clear || !rails_live(g) { return out; }
    let item = |g: &Game, c: CharId| crate::rails::RailsLineMenItem { pos: g.ch[c].pos, bones: crate::rails::RailsLineMenItemBones { head: g.ch[c].bones.get(crate::character::Bone::Head).copied() } };
    for gd in rails_squad(g) { let c = g.gd[gd].ch; out.push(item(g, c)); }
    if !rails_sky_on(g) { let c = g.pl[g.player].ch; out.push(item(g, c)); }
    out
}
/// the squad's men still up (rails.ts Rails.squad)
pub fn rails_squad(g: &mut crate::game::Game) -> Vec<GuardId> {
    let mut out = Vec::new();
    for k in 0..g.guards.len() {
        let x = g.guards[k];
        let r = &g.gd[x];
        if r.unit.is_some() && r.team == crate::teams::Team::Operative && r.state != crate::game::GuardState::Dead { out.push(x); }
    }
    out
}
/// the sky seat: the player out of the picture, the squad holding where it stands (rails.ts Rails.park)
pub fn rails_park(g: &mut crate::game::Game, r: &crate::scene_scenarios::RailsData) {
    let sky = r.sky.as_ref().unwrap();
    g.rails.sky.parked = true;
    g.rails.sky.was = Some(crate::rails::RailsSkyWas { invisible: g.player_invisible });
    g.player_invisible = true;
    crate::game::game_teleport_player(g, sky.park);
    for gd in rails_squad(g) { rails_hold_here(g, gd, Some(sky.low.unwrap_or(false))); }
    rails_note(g, &format!("sky seat: the player parked at ({}, {})", sky.park[0], sky.park[2]));
}
/// he holds where he is (rails.ts Rails.holdHere)
pub fn rails_hold_here(g: &mut crate::game::Game, gd: GuardId, low: Option<bool>) {
    let low = low.unwrap_or(false);
    let c = g.gd[gd].ch;
    let (p, yaw) = (g.ch[c].pos, g.ch[c].body_yaw);
    let a = crate::orders::SquadOrdersAssignA {
        order: String::from("hold"),
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
pub fn rails_waiting(g: &mut crate::game::Game, s: &crate::rails::RailsWaitingS) -> bool {
    let r = s.l.rails.as_ref().unwrap();
    if r.waits.is_none() || rails_sky_on(g) || r.allow.iter().any(|x| x == "order") { return false; }
    let mut k = 0;
    while (k as f64) <= s.i && k < s.l.steps.len() {
        if names(&s.l.steps[k].allow, "order") { return false; }
        k += 1;
    }
    true
}
/// a waiting man on 'fall in' holds his place (rails.ts Rails.holdWaiting)
pub fn rails_hold_waiting(g: &mut crate::game::Game, s: &crate::rails::RailsHoldWaitingS) {
    if !rails_waiting(g, &crate::rails::RailsWaitingS { l: s.l.clone(), i: s.i }) { return; }
    let low = s.l.rails.as_ref().unwrap().waits.as_ref().unwrap().low.unwrap_or(false);
    for gd in rails_squad(g) {
        if order_of(g, gd).as_deref() == Some("fallIn") { rails_hold_here(g, gd, Some(low)); }
    }
}
/// (rails.ts Rails.unpark)
pub fn rails_unpark(g: &mut crate::game::Game) {
    g.rails.sky.parked = false;
    if let Some(w) = g.rails.sky.was { g.player_invisible = w.invisible; g.rails.sky.was = None; }
}
/// never past the walk keys' own reach (rails.ts:555, 568)
fn clamp_off(g: &mut Game) {
    let o = g.rails.sky.off;
    let d = m::hyp2(o[0], o[2]);
    let max = match rails_data(g).and_then(|x| x.sky).and_then(|x| x.reach) { Some(r) => r, None => g.k.rails.pan_reach };
    if d > max {
        g.rails.sky.off[0] *= max / d;
        g.rails.sky.off[2] *= max / d;
    }
}
/// the walk keys shift the view about the squad (rails.ts Rails.pan)
pub fn rails_pan(g: &mut crate::game::Game, input: InputId, dt: f64) {
    let (f, r) = {
        let i = &g.tab.input[input];
        let on = |k: &str| if i.keys.has(String::from(k)) { 1.0 } else { 0.0 };
        (on("KeyW") - on("KeyS"), on("KeyD") - on("KeyA"))
    };
    if f == 0.0 && r == 0.0 { return; }
    let b = crate::camera::follow_camera_planar_basis(g);
    let k = g.k.rails.pan_speed * (first_cam_distance(g) / 21.0) * dt / m::hyp2(f, r);
    g.rails.sky.off[0] += (b.fwd[0] * f + b.right[0] * r) * k;
    g.rails.sky.off[2] += (b.fwd[2] * f + b.right[2] * r) * k;
    clamp_off(g);
}
/// the mark stays in the picture (rails.ts Rails.keep)
pub fn rails_keep(g: &mut crate::game::Game, input: InputId, dt: f64) {
    if !js::truthy(g.k.rails.keep) { return; }
    { let i = &g.tab.input[input]; if walk_keys().iter().any(|k| i.keys.has(k.clone())) { return; } }
    let s = match rails_lesson(g) { Some(s) => s, None => return };
    let at = match step_at(&s.l, s.i).and_then(|t| t.at) { Some(a) => a, None => return };
    let v = match rails_look(g) { Some(v) => v, None => return };
    let dist = first_cam_distance(g);
    if rails_on_way(g, Some(&crate::rails::RailsOnWayS { l: s.l.clone(), i: s.i })) {
        let o = g.rails.sky.off;
        let d = m::hyp2(o[0], o[2]);
        if d < 1e-3 { return; }
        let k = m::max(0.0, 1.0 - g.k.rails.keep_speed * (dist / 21.0) * dt / d);
        g.rails.sky.off[0] *= k;
        g.rails.sky.off[2] *= k;
        return;
    }
    let b = crate::camera::follow_camera_planar_basis(g);
    let rx = g.k.rails.keep * dist;
    let rf = rx * g.k.rails.keep_deep;
    let (dx, dz) = (at[0] - v[0], at[2] - v[2]);
    let r = dx * b.right[0] + dz * b.right[2];
    let f = dx * b.fwd[0] + dz * b.fwd[2] - rx * g.k.rails.keep_up;
    let e = m::hyp2(r / rx, f / rf);
    if e <= 1.0 { return; }
    // (the shift that brings the mark to the oval's edge, along the line from the oval's middle to it)
    let sr = r * (1.0 - 1.0 / e);
    let sf = f * (1.0 - 1.0 / e);
    let k = m::min(1.0, g.k.rails.keep_speed * (dist / 21.0) * dt / m::hyp2(sr, sf));
    g.rails.sky.off[0] += (b.right[0] * sr + b.fwd[0] * sf) * k;
    g.rails.sky.off[2] += (b.right[2] * sr + b.fwd[2] * sf) * k;
    clamp_off(g);
}
/// how far the view has been shifted off the squad (rails.ts Rails.panned)
pub fn rails_panned(g: &mut crate::game::Game) -> f64 { m::hyp2(g.rails.sky.off[0], g.rails.sky.off[2]) }
/// where the view looks in the sky seat (rails.ts Rails.look)
pub fn rails_look(g: &mut crate::game::Game) -> Option<V3> {
    if !rails_sky_on(g) { return None; }
    let men = rails_squad(g);
    if !men.is_empty() {
        let n = men.len() as f64;
        let mut c: V3 = [0.0, 0.0, 0.0];
        for x in &men { let p = g.ch[g.gd[*x].ch].pos; c[0] += p[0] / n; c[2] += p[2] / n; }
        if g.rails.sky.mid.is_none() || g.rails.sky.mid_t != g.time {
            match g.rails.sky.mid {
                None => g.rails.sky.mid = Some(c),
                Some(mut q) => {
                    let dt = m::max(0.0, g.time - g.rails.sky.mid_t);
                    let a = 1.0 - m::exp(-dt * 2.5);
                    q[0] += (c[0] - q[0]) * a;
                    q[2] += (c[2] - q[2]) * a;
                    g.rails.sky.mid = Some(q);
                }
            }
            g.rails.sky.mid_t = g.time;
        }
    }
    let mid = match g.rails.sky.mid { Some(q) => q, None => g.ch[g.pl[g.player].ch].pos };
    let o = g.rails.sky.off;
    let w = crate::scene_world::world();
    Some([m::min(w.max[0], m::max(w.min[0], mid[0] + o[0])), 0.0, m::min(w.max[2], m::max(w.min[2], mid[2] + o[2]))])
}
/// the debug panel's line (rails.ts Rails.summary)
pub fn rails_summary(g: &mut crate::game::Game) -> String {
    let s = match rails_lesson(g) {
        Some(s) => s,
        None => {
            let carries = g.level.scenarios.as_ref().and_then(|x| x.lesson.as_ref()).map_or(false, |l| l.rails.is_some());
            return String::from(if carries { if g.k.rails.on { "off · this level carries the record (start its mission)" } else { "off · switched off" } } else { "off · this level carries none" });
        }
    };
    let a = rails_allowed(g).unwrap();
    let open: Vec<String> = a.iter().filter(|x| x != "stalk").collect();
    let open = if open.is_empty() { String::from("—") } else { open.join(" ") };
    let len = s.l.steps.len() as f64;
    let shut = if rails_shuts(g, "stalk") { " · stalk shut" } else { "" };
    let sky = if rails_sky_on(g) { " · sky seat" } else { "" };
    let spared = if g.rails.spared.len() > 0 { format!(" · spared {}", g.rails.spared.len()) } else { String::new() };
    format!("step {}/{} · unlocked: {}{}{}{}", m::min(s.i + 1.0, len), len, open, shut, sky, spared)
}
/// the frame's own log (rails.ts Rails.note)
pub fn rails_note(g: &mut crate::game::Game, s: &str) {
    let line = format!("{:.2} {}", g.time, s);
    g.rails.log.push(line);
    if g.rails.log.len() > 80 { g.rails.log.drain(0..40); }
}
/// combat.ts killGuard's question (rails.ts railsSpare)
pub fn rails_spare_2(g: &mut crate::game::Game, gd: GuardId) -> bool { rails_spare(g, gd) }
/// is this man down as far as an enemy's eye goes (rails.ts railsDown)
pub fn rails_down(g: &mut crate::game::Game, gd: GuardId) -> bool { g.k.rails.spare_down && rails_live(g) && rails_is_spared(g, gd) }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_line_names_a_key_only_with_something_in_its_brackets() {
        assert!(has_bracket("Press [Q]"));
        assert!(has_bracket("[] then [F]"));
        assert!(has_bracket("[[x]"));
        assert!(!has_bracket("[]"));
        assert!(!has_bracket("no key ] [ here"));
    }
    #[test]
    fn the_pages_own_keys_are_free_and_only_they() {
        assert!(is_free("Escape") && is_free("F1") && is_free("F12") && is_free("BracketLeft"));
        assert!(!is_free("F") && !is_free("F123") && !is_free("Fx") && !is_free("KeyF"));
        assert!(is_digit("Digit1") && is_digit("Digit5"));
        assert!(!is_digit("Digit0") && !is_digit("Digit6") && !is_digit("Digit12"));
    }
}
