use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---- the lines: what the regular expressions of lesson.ts match, written out (no regex in the game). All works on bytes, as every pattern here
// starts and ends on an ASCII character and a non-ASCII byte is no \w, as a non-ASCII unit is none in JavaScript.

/// \w
fn is_word(b: u8) -> bool { b.is_ascii_alphanumeric() || b == b'_' }
/// \b at `i`
fn boundary(b: &[u8], i: usize) -> bool { (i > 0 && is_word(b[i - 1])) != (i < b.len() && is_word(b[i])) }
/// the literal `s` at `at`: where it ends
fn lit(b: &[u8], at: usize, s: &str) -> Option<usize> { if b.get(at..)?.starts_with(s.as_bytes()) { Some(at + s.len()) } else { None } }
/// `String.replace` of a global pattern: `at` says whether one starts at this byte, where it ends and what it becomes
fn replace_all(line: &str, mut at: impl FnMut(&[u8], usize) -> Option<(usize, String)>) -> String {
    let b = line.as_bytes();
    let (mut out, mut last, mut i) = (String::new(), 0, 0);
    while i < b.len() {
        match at(b, i) {
            Some((end, s)) => { out.push_str(&line[last..i]); out.push_str(&s); last = end; i = end; }
            None => i += 1,
        }
    }
    out.push_str(&line[last..]);
    out
}
/// /\b<word>\b/g → with
fn replace_words(line: &str, word: &str, with: &str) -> String {
    replace_all(line, |b, i| { if !boundary(b, i) { return None; } let end = lit(b, i, word)?; if boundary(b, end) { Some((end, String::from(with))) } else { None } })
}
/// a line that names the knife, while there is none: the right click's knockout is the blow
pub(in crate::missions) fn bare_line(line: &str) -> String {
    let s = replace_words(line, "Right click to knock him out, or left click to use the knife", "Right-click to knock him out");
    let s = replace_words(&s, "left click to knife him", "right-click to knock him out");
    replace_words(&s, "left-click to kill him with the knife", "right-click to knock him out")
}

/// `s` between `head` and `tail`, at least a letter of it
fn between<'a>(s: &'a str, head: &str, tail: &str) -> Option<&'a str> { s.strip_prefix(head)?.strip_suffix(tail).filter(|w| !w.is_empty()) }
/// row `row` of lesson.ts TOKENS: its pattern on a name already of the letters lessonToken allows: the man's words
fn token_hit(row: usize, n: &str) -> Option<&str> {
    match row {
        0 => between(n, "grab ", " from behind"),
        1 => between(n, "grab ", ""),
        2 => between(n, "knock ", " out"),
        _ => between(n, "take ", " out"),
    }
}
/// …and what it says
fn token_say(row: usize, who: &str, held: bool, bare: bool) -> String {
    match row {
        0 => if held { format!("hold left click on {} from behind to grab him", who) } else { format!("press [Space] behind {} to grab him", who) },
        1 => if held { format!("hold left click on {} to grab him", who) } else { format!("press [Space] to grab {}", who) },
        2 => if bare { format!("right-click to knock {} out", who) } else { format!("right-click to knock {} out, or left-click to use the knife", who) },
        _ => if bare { format!("right-click to knock {} out", who) } else { format!("left-click to knife {}", who) },
    }
}

/// the first letter lower-cased
pub fn lower(name: &str) -> String {
    let mut c = name.chars();
    match c.next() {
        // (charAt is one UTF-16 unit: half a surrogate pair lower-cases to itself)
        Some(f) if f.len_utf16() == 1 => { let mut s: String = f.to_lowercase().collect(); s.push_str(c.as_str()); s }
        _ => String::from(name),
    }
}

/// is this name (the words between the braces, either capital) one lessonLine writes out?
pub fn lesson_token(name: &str) -> bool {
    let n = lower(name);
    // /^[A-Za-z]+(?: [A-Za-z']+)*$/
    let good = name.split(' ').enumerate().all(|(i, w)| !w.is_empty() && w.bytes().all(|c| c.is_ascii_alphabetic() || (i > 0 && c == b'\'')));
    if !good { return false; }
    // (…and a 'from' left in the man's words is a 'from behind' misspelt: /(^| )from( |$)/)
    match (0..4).find_map(|row| token_hit(row, &n)) { Some(who) => !who.split(' ').any(|w| w == "from"), None => false }
}

/// a line's tokens written out; on the page `held` is false and `bare` true when not given
pub(in crate::missions) fn token_line(line: &str, held: Option<bool>, bare: Option<bool>) -> String {
    // (held = stalkClickOnly() with NO game: STALK.on && STALK.only && !STALK.key, false on the page · bare = CLOSE.on && CLOSE.noKnife, true on it)
    let (held, bare) = (held.unwrap_or(false), bare.unwrap_or(true));
    if !line.contains('{') { return String::from(line); }
    // /\{([A-Za-z][A-Za-z' ]*)\}/g
    replace_all(line, |b, i| {
        if b[i] != b'{' || !b.get(i + 1)?.is_ascii_alphabetic() { return None; }
        let mut j = i + 2;
        while b.get(j).map_or(false, |&c| c.is_ascii_alphabetic() || c == b'\'' || c == b' ') { j += 1; }
        if b.get(j) != Some(&b'}') { return None; }
        let name = &line[i + 1..j];
        let n = lower(name);
        let mut said = String::from(&line[i..j + 1]);
        if lesson_token(name) {
            if let Some(row) = (0..4).find(|&row| token_hit(row, &n).is_some()) {
                said = token_say(row, token_hit(row, &n).unwrap_or(""), held, bare);
                if n != name { said[..1].make_ascii_uppercase(); }   // (the capital the author wrote, nothing guessed: all four begin on an ASCII letter)
            }
        }
        Some((j + 1, said))
    })
}

/// a step's line as the HUD's row says it now; the rails' frame reads it too, so it is no picture-only code here
pub fn lesson_line(g: &crate::game::Game, written: &str) -> String { line_on_page(written, g.k.cmdring.wedges) }
/// …with the ring's wedges read
fn line_on_page(written: &str, wedges: f64) -> String {
    // (stalkClickOnly() with no game is false on the page: no stalkLine · CLOSE.on && CLOSE.noKnife: bareLine · HANDRING.on: the whistle and the wheel)
    let s = bare_line(&token_line(written, None, None));
    let s = replace_words(&s, "V whistles", &format!("Space held, then {}, whistles", wedges));
    replace_words(&s, "Hold Space, wheel to kick, let go", WHEEL_RING)
}

// ---- the tests

/// one test asked: an unknown kind, or a flag no row has (which throws in the TypeScript and is caught there), is never true
pub fn passes(g: &mut crate::game::Game, t: &crate::lesson::LessonTest, s: &mut crate::lesson::StepSeen) -> bool {
    test_ok(g, t, s).unwrap_or(false)
}

/// the first row of a front-trimmed log that came after `last`, the logs' rows given by serial; 0: `last` is gone, or there was none
pub fn after(log: &[u32], last: Option<u32>) -> usize {
    let mut k = log.len();
    while k > 0 && Some(log[k - 1]) != last { k -= 1; }
    k
}

pub fn inside(r: crate::scene_scenarios::Rect, p: V3) -> bool { p[0] > r.x0 && p[0] < r.x1 && p[2] > r.z0 && p[2] < r.z1 }

/// the player a lesson is taught to, one seat today
pub fn who(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> PlayerId { seat.player }

/// the door a teammate is on a plain 'stack up' on, by name; None: none
pub(in crate::missions) fn on_stack(g: &crate::game::Game, seat: &crate::seats::SeatRef, m: GuardId) -> Option<String> {
    let a = crate::orders::squad_orders_of(g, seat, m)?;
    if !crate::orders::rows::is(g, a, crate::orders::Is::Stacks) { return None; }
    let d = g.tab.orders_assignment[a].door?;
    Some(g.dr[d].def.name.clone())
}

/// a man no longer on his feet: dead, or out cold
pub fn downed(g: &crate::game::Game, x: GuardId) -> bool { g.gd[x].state == crate::game::GuardState::Dead }

/// a teammate on his feet
fn operative_up(g: &Game, x: GuardId) -> bool { g.gd[x].unit.is_some() && g.gd[x].team == crate::teams::Team::Operative && g.gd[x].state != crate::game::GuardState::Dead }

/// the one partner of a quiet pair: the first operative unit on his feet
pub fn partner_of(g: &crate::game::Game) -> Option<GuardId> { g.guards.iter().copied().find(|&x| operative_up(g, x)) }

/// is `v` within the record's below and above; a kind with neither field has nothing to refuse
pub fn band(v: f64, t: &crate::lesson::LessonTest) -> bool {
    let (LessonTest::Meter { below, above, .. } | LessonTest::Pace { below, above }) = t else { return true };
    below.map_or(true, |b| v < b) && above.map_or(true, |a| v > a)
}

/// a body's place
fn man_pos(g: &Game, x: GuardId) -> V3 { g.ch[g.gd[x].ch].pos }
/// the player's place
fn own_pos(g: &Game, seat: &crate::seats::SeatRef) -> V3 { g.ch[g.pl[seat.player].ch].pos }
/// `onStack(g, m) === door`
fn stack_is(g: &Game, seat: &crate::seats::SeatRef, m: GuardId, door: &str) -> bool { on_stack(g, seat, m).as_deref() == Some(door) }
/// `!!g.orders.of(m)?.arrived`
fn arrived(g: &mut Game, m: GuardId) -> bool { crate::orders::squad_orders_of(g, &crate::seats::here(g), m).map_or(false, |a| g.tab.orders_assignment[a].arrived) }
/// `g.orders.men(sel)`
fn squad_men(g: &Game, seat: &crate::seats::SeatRef, sel: crate::orders::SquadOrdersMenSel) -> Vec<GuardId> { crate::orders::squad_orders_men(g, seat, Some(sel), None) }
/// a text cell of a chronicle row (`e.data.<key>`)
fn chron_text<'a>(e: &'a crate::chronicle::ChronEvent, key: &str) -> Option<&'a str> {
    match e.data.get(String::from(key)) { Some(Some(crate::chronicle::ChronEventDataValue::Text(s))) => Some(s.as_str()), _ => None }
}
/// STAGES.indexOf
fn stage_index(s: &str) -> i32 { stages().iter().position(|w| w == s).map_or(-1, |i| i as i32) }
/// a count kept in the step's map, as a test reads it
fn seen_n(s: &StepSeen, key: String) -> f64 { s.n.get(key).copied().unwrap_or(0.0) }

/// lesson.ts LESSON_TESTS, the `ok` of each kind (the `check` of each is check_of). None: the row would have thrown, or no such kind
fn test_ok(g: &mut Game, t: &LessonTest, s: &mut StepSeen) -> Option<bool> {
    use LessonTest as T;
    Some(match t {
        T::Unknown { .. } => return None,
        T::In { area } => inside(*area, own_pos(g, &crate::seats::here(g))),
        T::Near { at, r } => { let p = own_pos(g, &crate::seats::here(g)); m::hyp2(p[0] - at[0], p[2] - at[2]) < r.unwrap_or(1.2) }
        T::Flag { flag, value } => flag_of(g, flag)? == value.unwrap_or(true),
        T::Slot { slot, drawn } => { let p = who(g, &crate::seats::here(g)); g.pl[p].slot == *slot && drawn.map_or(true, |d| !g.pl[p].holstered == d) }
        T::Pace { .. } => true,   // (TWOGEARS.on: the wheel is nobody's pace and the line goes by)
        T::Meter { which, .. } => { let p = who(g, &crate::seats::here(g)); let v = if which == "noise" { g.pl[p].noise } else { g.pl[p].visibility }; band(v, t) }
        T::Stage { stage } => stage_index(g.mission.stage.as_str()) >= stage_index(stage) && g.mission.stage != crate::game::MissionStage::Failed,
        T::Door { door, state } => ok_door(g, door, state),
        T::Target { name, on } => ok_target(g, name, *on),
        T::Men { down } => ok_men(g, *down),
        T::Did { what, on, n } => {
            let key = match on { None => what.clone(), Some(on) => format!("{}@door:{}", what, on) };
            seen_n(s, key) >= n.unwrap_or(1.0)
        }
        T::Downed { how, n } => ok_downed(g, how, *n),
        T::Body { area } => ok_body(g, &crate::seats::here(g), *area),
        T::Calm { route } => ok_calm(g, route.as_deref()),
        T::Loaded => ok_loaded(g, s),
        T::Reload => ok_reload(g),
        T::Launcher { on, fired } => ok_launcher(g, on, *fired),
        T::Burst { n } => ok_burst(g, *n),
        T::Threw { kind, quick, area } => ok_threw(g, kind.as_deref(), *quick, *area, s),
        T::OrderTo { who } => ok_order_to(g, &crate::seats::here(g), *who),
        T::StackOn { door, who, set } => ok_stack_on(g, door, *who, *set),
        T::PairsOn { doors } => ok_pairs_on(g, &crate::seats::here(g), *doors),
        T::Word { doors } => s.words.len() as f64 >= doors.unwrap_or(1.0),
        T::Taken { room } => ok_taken(g, room),
        T::Following => ok_following(g, &crate::seats::here(g)),
        T::Mate { area, at, r, by, away } => ok_mate(g, &crate::seats::here(g), *area, *at, *r, *by, *away),
        T::Pair { what, n } => ok_pair(g, what, *n, s),
        T::Cue { name } => crate::rails::rails_cued(g, name),
        T::View { turned, zoomed } => ok_view(g, &crate::seats::here(g), *turned, *zoomed, s),
        T::Squad { area, n } => ok_squad(g, *area, *n),
        T::Stacks { doors, n, set } => ok_stacks(g, doors, *n, *set),
        T::Watching { at, r, n } => ok_watching(g, &crate::seats::here(g), *at, *r, *n),
        T::All { of } => of.iter().all(|u| passes(g, u, s)),
        T::Any { of } => of.iter().any(|u| passes(g, u, s)),
    })
}

fn ok_door(g: &Game, door: &str, state: &str) -> bool {
    use crate::doors::{door_is_closed, door_lock_broken, door_locked, doors_by_name};
    let Some(d) = doors_by_name(g, door) else { return false };
    match state {
        "open" => !door_is_closed(g, d),
        "shut" => door_is_closed(g, d),
        "unlocked" => !door_locked(g, d) || door_lock_broken(g, d),
        "charged" => crate::doortools::door_tools_charge_on(g, &crate::seats::here(g), d).is_some(),
        _ => door_lock_broken(g, d),
    }
}

fn ok_target(g: &Game, name: &str, on: bool) -> bool {
    use crate::sim_lightshome::{fixture_broken, fixture_factor, fixture_on};
    let Some(x) = g.targets.iter().copied().find(|&y| g.tab.sim_lightshome_fixture[y].name == name) else { return false };
    (fixture_on(g, x) && !fixture_broken(g, x) && fixture_factor(g, x) > 0.05) == on
}

fn ok_men(g: &Game, down: Option<f64>) -> bool {
    let guards = g.guards.clone();
    let n = guards.iter().filter(|&&x| crate::teams::hostile_to(g, x, crate::teams::HostileToO::Player) && downed(g, x)).count();
    n as f64 >= down.unwrap_or(1.0)
}

fn ok_downed(g: &Game, how: &str, n: Option<f64>) -> bool {
    use crate::chronicle::ChronKind;
    let hits = g.chronicle.events.iter().filter(|e| if how == "knife" { e.kind == ChronKind::Kill && chron_text(e, "weapon") == Some("knife") } else { e.kind == ChronKind::Ko && chron_text(e, "method") == Some(how) }).count();
    hits as f64 >= n.unwrap_or(1.0)
}

fn ok_body(g: &Game, seat: &crate::seats::SeatRef, area: crate::scene_scenarios::Rect) -> bool {
    let (guards, drag) = (g.guards.clone(), g.pl[seat.player].dragging);
    guards.iter().any(|&x| {
        if !downed(g, x) || drag == Some(x) { return false; }
        let c = g.gd[x].ch;
        inside(area, g.ch[c].bones.get(crate::character::Bone::Hips).copied().unwrap_or(g.ch[c].pos))
    })
}

fn ok_calm(g: &Game, route: Option<&str>) -> bool {
    let guards = g.guards.clone();
    guards.iter().all(|&x| {
        !crate::teams::hostile_to(g, x, crate::teams::HostileToO::Player) || downed(g, x)
            || route.map_or(false, |r| g.tab.route[g.gd[x].route].name != r)
            || g.gd[x].state == crate::game::GuardState::Patrol
    })
}

fn ok_loaded(g: &mut Game, s: &mut StepSeen) -> bool {
    let p = who(g, &crate::seats::here(g));
    let Some(w) = crate::game::player_gun_in_hand(g, p) else { return false };
    if crate::weapons::firearm_reloading(g, w) { s.n.set(String::from("loaded:began"), 1.0); return false; }
    if js::truthy(seen_n(s, String::from("loaded:began"))) || g.fa[w].spare.is_empty() { return true; }
    if g.fa[w].chamber != 1.0 { return false; }
    let (mag, most) = (g.fa[w].mag.unwrap_or(0.0), m::max_n(&g.fa[w].spare));
    mag >= m::min(most, crate::weapons::firearm_mag_room(g, w))
}

fn ok_reload(g: &Game) -> bool {
    let p = who(g, &crate::seats::here(g));
    let (pistol, rifle) = (g.pl[p].pistol, g.pl[p].rifle);
    (crate::game::player_pistol_up(g, p) && crate::weapons::firearm_reloading(g, pistol)) || (crate::game::player_rifle_up(g, p) && crate::weapons::firearm_reloading(g, rifle))
}

fn ok_launcher(g: &Game, on: &str, fired: bool) -> bool {
    let p = who(g, &crate::seats::here(g));
    let r = g.pl[p].rifle;
    let up = crate::weapons::rifle_attachment(g, r).map_or("none", |a| a.as_str());
    on == up && (!fired || (crate::game::player_rifle_up(g, p) && g.fa[r].ring_cd > 0.0))
}

fn ok_burst(g: &mut Game, n: Option<f64>) -> bool {
    let p = who(g, &crate::seats::here(g));
    let r = g.pl[p].rifle;
    crate::game::player_rifle_up(g, p) && crate::weapons::firearm_automatic(g, r) && g.fa[r].auto_idx >= n.unwrap_or(3.0)
}

fn ok_threw(g: &mut Game, k: Option<&str>, quick: bool, area: Option<crate::scene_scenarios::Rect>, s: &mut StepSeen) -> bool {
    use crate::chronicle::ChronKind;
    let from = if quick { s.held_at } else { s.chron0 };
    if from < 0.0 { return false; }
    let key = area.map_or(String::new(), |a| format!("threw:{}:{},{},{},{}", k.unwrap_or("undefined"), a.x0, a.z0, a.x1, a.z1));
    if area.is_some() && js::truthy(seen_n(s, key.clone())) { return true; }   // (seen once is seen: a frag or a stun can is off the floor the frame after it goes off)
    let len = g.chronicle.events.len();
    for i in (m::min(from, len as f64) as usize)..len {
        let e = &g.chronicle.events[i];
        if e.kind != ChronKind::Thrown || !(k.is_none() || chron_text(e, "kind") == k) { continue; }
        let Some(a) = area else { return true };
        let since = g.mission.stats.elapsed - e.t + 0.5;   // (the chronicle's clock is the mission's own)
        if lying_can(g, a, k, since) || bang_in(g, a, k) { s.n.set(key, 1.0); return true; }
    }
    false
}
/// his own smoke can venting in `a`, on the floor no longer than `since`
fn lying_can(g: &mut Game, a: crate::scene_scenarios::Rect, k: Option<&str>, since: f64) -> bool {
    use crate::throwables::{item_age, item_fired, item_kind, item_pos, item_who, ItemKind};
    if k == Some("flash") || k == Some("frag") { return false; }
    let (me, items) = (g.ch[g.pl[g.player].ch].id, g.items.items.clone());
    items.iter().any(|&x| item_kind(g, x) == ItemKind::Smoke && item_fired(g, x) && item_who(g, x) == me && item_age(g, x) <= since && inside(a, item_pos(g, x)))
}
/// a bang of that kind in `a` this frame, not a charge's
fn bang_in(g: &Game, a: crate::scene_scenarios::Rect, k: Option<&str>) -> bool {
    if k == Some("smoke") { return false; }
    g.events.iter().any(|e| e.kind == crate::director::EventKind::Bang && !e.charge.unwrap_or(false) && (k.is_none() || (k == Some("frag")) == e.frag.unwrap_or(false)) && inside(a, e.pos))
}

fn ok_order_to(g: &Game, seat: &crate::seats::SeatRef, want: crate::orders::Who) -> bool {
    use crate::orders::Who;
    if !crate::orders::squad_orders_active(g, seat) { return false; }
    match (seat.orders.who, want) { (Who::All, Who::All) | (Who::Red, Who::Red) | (Who::Blue, Who::Blue) => true, (Who::Num(a), Who::Num(b)) => a == b, _ => false }
}

fn ok_stack_on(g: &mut Game, door: &str, sel: Option<crate::orders::SquadOrdersMenSel>, set: bool) -> bool {
    if !crate::orders::squad_orders_active(g, &crate::seats::here(g)) { return false; }
    let men = squad_men(g, &crate::seats::here(g), sel.unwrap_or(crate::orders::SquadOrdersMenSel::All));
    !men.is_empty() && men.iter().all(|&m| stack_is(g, &crate::seats::here(g), m, door) && (!set || arrived(g, m)))
}

fn ok_pairs_on(g: &Game, seat: &crate::seats::SeatRef, want: Option<f64>) -> bool {
    use crate::{game::PairId, orders::SquadOrdersMenSel as Sel};
    if !crate::orders::squad_orders_active(g, seat) { return false; }
    let mut doors: JsSet<String> = JsSet::new();
    for id in crate::orders::squad_orders_pairs(g, seat) {
        let men = squad_men(g, seat, match id { PairId::Red => Sel::Red, PairId::Blue => Sel::Blue });
        let d = match men.first() { Some(&m) => on_stack(g, seat, m), None => None };
        let Some(d) = d.filter(|w| !w.is_empty()) else { continue };
        if men.iter().all(|&m| stack_is(g, seat, m, &d)) { doors.add(d); }
    }
    doors.len() as f64 >= want.unwrap_or(2.0)
}

fn ok_taken(g: &Game, room: &str) -> bool {
    if !crate::roomclear::room_clear_scenario_active(g) { return false; }
    g.room_clear.rooms.iter().find(|r| r.def.id == room).map_or(false, |r| r.cleared)
}

fn ok_following(g: &Game, seat: &crate::seats::SeatRef) -> bool {
    if !crate::orders::squad_orders_active(g, seat) { return false; }
    let men = squad_men(g, seat, crate::orders::SquadOrdersMenSel::All);
    !men.is_empty() && men.iter().all(|&m| crate::orders::squad_orders_of(g, seat, m).map_or(false, |a| crate::orders::rows::is(g, a, crate::orders::Is::Follows)))
}

fn ok_mate(g: &Game, seat: &crate::seats::SeatRef, area: Option<crate::scene_scenarios::Rect>, at: Option<V3>, r: Option<f64>, by: Option<f64>, away: Option<f64>) -> bool {
    let Some(mate) = partner_of(g) else { return false };
    let (p, s) = (man_pos(g, mate), own_pos(g, seat));
    let d = m::hyp2(p[0] - s[0], p[2] - s[2]);
    area.map_or(true, |a| inside(a, p))
        && at.map_or(true, |a| m::hyp2(p[0] - a[0], p[2] - a[2]) < r.unwrap_or(1.5))
        && by.map_or(true, |by| d < by)
        && away.map_or(true, |away| d > away)
}

fn ok_pair(g: &Game, what: &str, n: Option<f64>, s: &StepSeen) -> bool {
    let serials: Vec<u32> = g.m.pairorders.pair_log.iter().map(|row| row.serial).collect();
    let from = after(&serials, s.last_pair);
    let hits = g.m.pairorders.pair_log.iter().skip(from).filter(|row| row.what.starts_with(what)).count();
    hits as f64 >= n.unwrap_or(1.0)
}

fn ok_view(g: &Game, seat: &crate::seats::SeatRef, turned: Option<f64>, zoomed: Option<f64>, s: &StepSeen) -> bool {
    let cam = g.tab.cmd_seat[seat.id].cam;
    let distance = g.tab.camera[cam].distance;
    turned.map_or(true, |d| s.turned * 180.0 / m::PI >= d)
        && zoomed.map_or(true, |z| m::abs(m::log(distance / s.dist0)) >= m::log(z))
}

fn ok_squad(g: &Game, area: crate::scene_scenarios::Rect, n: Option<f64>) -> bool {
    let men: Vec<GuardId> = g.guards.iter().copied().filter(|&x| operative_up(g, x)).collect();
    let inn = men.iter().filter(|&&x| inside(area, man_pos(g, x))).count();
    !men.is_empty() && inn as f64 >= m::min(men.len() as f64, n.unwrap_or(men.len() as f64))
}

fn ok_stacks(g: &mut Game, doors: &[String], n: Option<f64>, set: bool) -> bool {
    if !crate::orders::squad_orders_active(g, &crate::seats::here(g)) { return false; }
    let men = squad_men(g, &crate::seats::here(g), crate::orders::SquadOrdersMenSel::All);
    doors.iter().all(|d| men.iter().filter(|&&m| stack_is(g, &crate::seats::here(g), m, d) && (!set || arrived(g, m))).count() as f64 >= n.unwrap_or(1.0))
}

fn ok_watching(g: &Game, seat: &crate::seats::SeatRef, a: V3, r: Option<f64>, n: Option<f64>) -> bool {
    if !crate::orders::squad_orders_active(g, seat) { return false; }
    let r = r.unwrap_or(3.0);
    let men = squad_men(g, seat, crate::orders::SquadOrdersMenSel::All);
    let hits = men.iter().filter(|&&m| {
        let Some(id) = crate::orders::squad_orders_of(g, seat, m) else { return false };
        let row = &g.tab.orders_assignment[id];
        crate::orders::rows::id_is(&row.order, crate::orders::Is::Covers) && row.watch.map_or(false, |w| m::hyp2(w[0] - a[0], w[2] - a[2]) < r)
    }).count();
    hits as f64 >= n.unwrap_or(1.0)
}

// ---- the checks: what is wrong with a record on this floor. A field's type is the skeleton's now, so a check of a type alone (`typeof t.on`) has nothing to ask

/// lesson.ts num of a number
fn finite(v: f64) -> bool { v.is_finite() }
/// a row's `below` and `above`
pub(in crate::missions) fn band_check(t: &crate::lesson::LessonTest) -> Option<String> {
    let (below, above) = match t { LessonTest::Pace { below, above } | LessonTest::Meter { below, above, .. } => (*below, *above), _ => (None, None) };
    let bad = |v: Option<f64>| v.map_or(false, |v| !finite(v));
    if (below.is_none() && above.is_none()) || bad(below) || bad(above) { Some(String::from("below and / or above: numbers")) } else { None }
}

/// a kind's word (`t.is`)
fn kind_word(t: &LessonTest) -> &'static str {
    use LessonTest as T;
    match t {
        T::Unknown { .. } => "", T::In { .. } => "in", T::Near { .. } => "near", T::Flag { .. } => "flag", T::Slot { .. } => "slot", T::Pace { .. } => "pace", T::Meter { .. } => "meter",
        T::Stage { .. } => "stage", T::Door { .. } => "door", T::Target { .. } => "target", T::Men { .. } => "men", T::Did { .. } => "did", T::Downed { .. } => "downed",
        T::Body { .. } => "body", T::Calm { .. } => "calm", T::Loaded => "loaded", T::Reload => "reload", T::Launcher { .. } => "launcher", T::Burst { .. } => "burst",
        T::Threw { .. } => "threw", T::OrderTo { .. } => "orderTo", T::StackOn { .. } => "stackOn", T::PairsOn { .. } => "pairsOn", T::Word { .. } => "word", T::Taken { .. } => "taken",
        T::Following => "following", T::Mate { .. } => "mate", T::Pair { .. } => "pair", T::Cue { .. } => "cue", T::View { .. } => "view", T::Squad { .. } => "squad",
        T::Stacks { .. } => "stacks", T::Watching { .. } => "watching", T::All { .. } => "all", T::Any { .. } => "any",
    }
}

/// lesson.ts FLAGS by the word a test names; None: no such flag
fn flag_of(g: &mut Game, name: &str) -> Option<bool> {
    Some(match name {
        "crouch" => flags::crouch(g),
        "sprint" => flags::sprint(g, &crate::seats::here(g)),
        "wall" => flags::wall(g),
        "peek" => flags::peek(g, &crate::seats::here(g)),
        "lean" => flags::lean(g),
        "nv" => flags::nv(g),
        "light" => flags::light(g),
        "holstered" => flags::holstered(g, &crate::seats::here(g)),
        "gun" => flags::gun(g),
        "ringsOut" => flags::rings_out(g, &crate::seats::here(g)),
        "shellsOut" => flags::shells_out(g, &crate::seats::here(g)),
        "auto" => flags::auto(g),
        "setup" => flags::setup(g, &crate::seats::here(g)),
        "setupShut" => flags::setup_shut(g, &crate::seats::here(g)),
        "dragging" => flags::dragging(g),
        "dark" => flags::dark(g),
        "chargeSet" => flags::charge_set(g, &crate::seats::here(g)),
        "chargeClear" => flags::charge_clear(g),
        "wallBlown" => flags::wall_blown(g, &crate::seats::here(g)),
        "holding" => flags::holding(g, &crate::seats::here(g)),
        "picking" => flags::picking(g, &crate::seats::here(g)),
        "coding" => flags::coding(g, &crate::seats::here(g)),
        "menu" => flags::menu(g),
        "ownRing" => flags::own_ring(g, &crate::seats::here(g)),
        "silent" => flags::silent(g),
        _ => return None,
    })
}

/// the flags a `flag` test can read off the player and the team's posture, each true while it holds
pub mod flags {
    use super::*;

    pub fn crouch(g: &mut crate::game::Game) -> bool { g.pl[g.player].crouch }
    pub fn sprint(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool { g.pl[seat.player].sprinting }
    /// back to a wall
    pub fn wall(g: &mut crate::game::Game) -> bool { g.pl[g.player].wall.is_some() }
    /// …and leaning out of its open end
    pub fn peek(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool { g.pl[seat.player].wall.as_ref().map_or(0.0, |w| w.peek) > 0.5 }
    /// out round a wall's open end, whichever hold
    pub fn lean(g: &mut crate::game::Game) -> bool {
        let Some(w) = g.pl[g.player].wall.as_ref() else { return false };
        w.lean.unwrap_or(0.0) > 0.5 || (w.kind != Some(crate::player::WallHoldKind::Stack) && w.peek > 0.5)
    }
    pub fn nv(g: &mut crate::game::Game) -> bool { g.pl[g.player].nv }
    /// the weapon light of the gun in hand
    pub fn light(g: &mut crate::game::Game) -> bool {
        let p = g.player;
        let (pistol, rifle) = (g.pl[p].pistol, g.pl[p].rifle);
        (crate::game::player_pistol_up(g, p) && g.fa[pistol].light_on) || (crate::game::player_rifle_up(g, p) && g.fa[rifle].light_on)
    }
    pub fn holstered(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool { g.pl[seat.player].holstered }
    /// a gun in the hands
    pub fn gun(g: &mut crate::game::Game) -> bool { let p = g.player; crate::game::player_gun_in_hand(g, p).is_some() }
    pub fn rings_out(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool { g.fa[g.pl[seat.player].rifle].rings <= 0.0 }
    pub fn shells_out(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool { g.fa[g.pl[seat.player].rifle].shells <= 0.0 }
    pub fn auto(g: &mut crate::game::Game) -> bool { g.fa[g.pl[g.player].rifle].mode != crate::weapons::FireMode::Semi }
    /// the setup of the gun in his hands is up; SLOTSETUP.on is settled true
    pub fn setup(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool { crate::slotsetup::slot_setup_open(g, seat) }
    pub fn setup_shut(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool { !crate::slotsetup::slot_setup_open(g, seat) }
    pub fn dragging(g: &mut crate::game::Game) -> bool { g.pl[g.player].dragging.is_some() }
    /// the mains are off
    pub fn dark(g: &mut crate::game::Game) -> bool { crate::sim_lightshome::mains_active(g) }
    /// a wall charge is on a wall, waiting on the trigger
    pub fn charge_set(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool { !seat.wall_charge.set.is_empty() }
    /// …on the wall the step's mark is on (a step with none: any wall), with nobody of the squad inside its reach
    pub fn charge_clear(g: &mut crate::game::Game) -> bool {
        let c = g.wall_charge.set.first().copied();
        let at = crate::lesson::lesson_step(g).and_then(|step| step.at);
        let Some(c) = c else { return false };
        if !crate::wallcharge::wall_charge_too_close(g, Some(c)).is_empty() { return false; }
        let Some(at) = at else { return true };
        let wall = g.tab.wallcharge_set_wall_charge[c].wall.clone();
        let p = crate::wallcharge::wall_pt(g, &wall, 0.0, 0.0, None);
        (if wall.along_x { m::abs(p[2] - at[2]) } else { m::abs(p[0] - at[0]) }) < 0.4 && m::hyp2(p[0] - at[0], p[2] - at[2]) < 4.0
    }
    /// …and one has made its hole this run
    pub fn wall_blown(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool { seat.wall_charge.stats.holes > 0.0 }
    /// a man in his arm
    pub fn holding(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool { g.pl[seat.player].holding.is_some() }
    pub fn picking(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool { g.pl[seat.player].picking.is_some() }
    pub fn coding(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool { g.pl[seat.player].coding.is_some() }
    /// the order menu is up
    pub fn menu(g: &mut crate::game::Game) -> bool { g.orders.menu.is_some() }
    /// his own ring is up, with or without a key down
    pub fn own_ring(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool { let sel = &g.pl[seat.player].sel; sel.open && sel.ring.is_some() }
    /// the squad's posture is SILENT
    pub fn silent(g: &mut crate::game::Game) -> bool { crate::silent::squad_silent(g, &crate::seats::here(g)) }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// the TypeScript's own answers
    static LOWER: &[(&str, &str)] = &[("Grab", "grab"), ("grab", "grab"), ("", ""), ("É1", "é1"), ("𐐀x", "𐐀x")];
    static BARE: &[(&str, &str)] = &[
        ("V whistles the cook out. Then hide at once, at the mark", "V whistles the cook out. Then hide at once, at the mark"),
        ("Hold Space, wheel to kick, let go", "Hold Space, wheel to kick, let go"),
        ("xV whistles and V whistlesy but V whistles.", "xV whistles and V whistlesy but V whistles."),
        ("Right click to knock him out, or left click to use the knife", "Right-click to knock him out"),
        ("Stay low so he doesn't hear you, then left click to knife him", "Stay low so he doesn't hear you, then right-click to knock him out"),
        ("When the guard comes through, step in and left-click to kill him with the knife", "When the guard comes through, step in and right-click to knock him out"),
        ("Press [Space] behind him to grab him, press Space behind the near guard to grab him.", "Press [Space] behind him to grab him, press Space behind the near guard to grab him."),
        ("press [Space to grab him now; Press Space] to grab him; Express Space to grab him; press Space to grab himself", "press [Space to grab him now; Press Space] to grab him; Express Space to grab him; press Space to grab himself"),
        ("When he lights up, creep in and {grab him from behind}. {Take him out} if he turns, or {knock him out}. {Grab him} now. {open the door}", "When he lights up, creep in and {grab him from behind}. {Take him out} if he turns, or {knock him out}. {Grab him} now. {open the door}"),
        ("{Grab the near guard from behind}, then {grab yours}. {grab from behind} {knock out} {grab o'neil} ▸ {grab  him} {Grab him from the left} {} {1x} {grab him", "{Grab the near guard from behind}, then {grab yours}. {grab from behind} {knock out} {grab o'neil} ▸ {grab  him} {Grab him from the left} {} {1x} {grab him"),
        ("training ▸ — {take the cook’s man out}", "training ▸ — {take the cook’s man out}"),
    ];
    static TOKEN: &[(&str, bool, bool, &str)] = &[
        ("V whistles the cook out. Then hide at once, at the mark", false, false, "V whistles the cook out. Then hide at once, at the mark"),
        ("V whistles the cook out. Then hide at once, at the mark", false, true, "V whistles the cook out. Then hide at once, at the mark"),
        ("V whistles the cook out. Then hide at once, at the mark", true, false, "V whistles the cook out. Then hide at once, at the mark"),
        ("V whistles the cook out. Then hide at once, at the mark", true, true, "V whistles the cook out. Then hide at once, at the mark"),
        ("Hold Space, wheel to kick, let go", false, false, "Hold Space, wheel to kick, let go"),
        ("Hold Space, wheel to kick, let go", false, true, "Hold Space, wheel to kick, let go"),
        ("Hold Space, wheel to kick, let go", true, false, "Hold Space, wheel to kick, let go"),
        ("Hold Space, wheel to kick, let go", true, true, "Hold Space, wheel to kick, let go"),
        ("xV whistles and V whistlesy but V whistles.", false, false, "xV whistles and V whistlesy but V whistles."),
        ("xV whistles and V whistlesy but V whistles.", false, true, "xV whistles and V whistlesy but V whistles."),
        ("xV whistles and V whistlesy but V whistles.", true, false, "xV whistles and V whistlesy but V whistles."),
        ("xV whistles and V whistlesy but V whistles.", true, true, "xV whistles and V whistlesy but V whistles."),
        ("Right click to knock him out, or left click to use the knife", false, false, "Right click to knock him out, or left click to use the knife"),
        ("Right click to knock him out, or left click to use the knife", false, true, "Right click to knock him out, or left click to use the knife"),
        ("Right click to knock him out, or left click to use the knife", true, false, "Right click to knock him out, or left click to use the knife"),
        ("Right click to knock him out, or left click to use the knife", true, true, "Right click to knock him out, or left click to use the knife"),
        ("Stay low so he doesn't hear you, then left click to knife him", false, false, "Stay low so he doesn't hear you, then left click to knife him"),
        ("Stay low so he doesn't hear you, then left click to knife him", false, true, "Stay low so he doesn't hear you, then left click to knife him"),
        ("Stay low so he doesn't hear you, then left click to knife him", true, false, "Stay low so he doesn't hear you, then left click to knife him"),
        ("Stay low so he doesn't hear you, then left click to knife him", true, true, "Stay low so he doesn't hear you, then left click to knife him"),
        ("When the guard comes through, step in and left-click to kill him with the knife", false, false, "When the guard comes through, step in and left-click to kill him with the knife"),
        ("When the guard comes through, step in and left-click to kill him with the knife", false, true, "When the guard comes through, step in and left-click to kill him with the knife"),
        ("When the guard comes through, step in and left-click to kill him with the knife", true, false, "When the guard comes through, step in and left-click to kill him with the knife"),
        ("When the guard comes through, step in and left-click to kill him with the knife", true, true, "When the guard comes through, step in and left-click to kill him with the knife"),
        ("Press [Space] behind him to grab him, press Space behind the near guard to grab him.", false, false, "Press [Space] behind him to grab him, press Space behind the near guard to grab him."),
        ("Press [Space] behind him to grab him, press Space behind the near guard to grab him.", false, true, "Press [Space] behind him to grab him, press Space behind the near guard to grab him."),
        ("Press [Space] behind him to grab him, press Space behind the near guard to grab him.", true, false, "Press [Space] behind him to grab him, press Space behind the near guard to grab him."),
        ("Press [Space] behind him to grab him, press Space behind the near guard to grab him.", true, true, "Press [Space] behind him to grab him, press Space behind the near guard to grab him."),
        ("press [Space to grab him now; Press Space] to grab him; Express Space to grab him; press Space to grab himself", false, false, "press [Space to grab him now; Press Space] to grab him; Express Space to grab him; press Space to grab himself"),
        ("press [Space to grab him now; Press Space] to grab him; Express Space to grab him; press Space to grab himself", false, true, "press [Space to grab him now; Press Space] to grab him; Express Space to grab him; press Space to grab himself"),
        ("press [Space to grab him now; Press Space] to grab him; Express Space to grab him; press Space to grab himself", true, false, "press [Space to grab him now; Press Space] to grab him; Express Space to grab him; press Space to grab himself"),
        ("press [Space to grab him now; Press Space] to grab him; Express Space to grab him; press Space to grab himself", true, true, "press [Space to grab him now; Press Space] to grab him; Express Space to grab him; press Space to grab himself"),
        ("When he lights up, creep in and {grab him from behind}. {Take him out} if he turns, or {knock him out}. {Grab him} now. {open the door}", false, false, "When he lights up, creep in and press [Space] behind him to grab him. Left-click to knife him if he turns, or right-click to knock him out, or left-click to use the knife. Press [Space] to grab him now. {open the door}"),
        ("When he lights up, creep in and {grab him from behind}. {Take him out} if he turns, or {knock him out}. {Grab him} now. {open the door}", false, true, "When he lights up, creep in and press [Space] behind him to grab him. Right-click to knock him out if he turns, or right-click to knock him out. Press [Space] to grab him now. {open the door}"),
        ("When he lights up, creep in and {grab him from behind}. {Take him out} if he turns, or {knock him out}. {Grab him} now. {open the door}", true, false, "When he lights up, creep in and hold left click on him from behind to grab him. Left-click to knife him if he turns, or right-click to knock him out, or left-click to use the knife. Hold left click on him to grab him now. {open the door}"),
        ("When he lights up, creep in and {grab him from behind}. {Take him out} if he turns, or {knock him out}. {Grab him} now. {open the door}", true, true, "When he lights up, creep in and hold left click on him from behind to grab him. Right-click to knock him out if he turns, or right-click to knock him out. Hold left click on him to grab him now. {open the door}"),
        ("{Grab the near guard from behind}, then {grab yours}. {grab from behind} {knock out} {grab o'neil} ▸ {grab  him} {Grab him from the left} {} {1x} {grab him", false, false, "Press [Space] behind the near guard to grab him, then press [Space] to grab yours. {grab from behind} {knock out} press [Space] to grab o'neil ▸ {grab  him} {Grab him from the left} {} {1x} {grab him"),
        ("{Grab the near guard from behind}, then {grab yours}. {grab from behind} {knock out} {grab o'neil} ▸ {grab  him} {Grab him from the left} {} {1x} {grab him", false, true, "Press [Space] behind the near guard to grab him, then press [Space] to grab yours. {grab from behind} {knock out} press [Space] to grab o'neil ▸ {grab  him} {Grab him from the left} {} {1x} {grab him"),
        ("{Grab the near guard from behind}, then {grab yours}. {grab from behind} {knock out} {grab o'neil} ▸ {grab  him} {Grab him from the left} {} {1x} {grab him", true, false, "Hold left click on the near guard from behind to grab him, then hold left click on yours to grab him. {grab from behind} {knock out} hold left click on o'neil to grab him ▸ {grab  him} {Grab him from the left} {} {1x} {grab him"),
        ("{Grab the near guard from behind}, then {grab yours}. {grab from behind} {knock out} {grab o'neil} ▸ {grab  him} {Grab him from the left} {} {1x} {grab him", true, true, "Hold left click on the near guard from behind to grab him, then hold left click on yours to grab him. {grab from behind} {knock out} hold left click on o'neil to grab him ▸ {grab  him} {Grab him from the left} {} {1x} {grab him"),
        ("training ▸ — {take the cook’s man out}", false, false, "training ▸ — {take the cook’s man out}"),
        ("training ▸ — {take the cook’s man out}", false, true, "training ▸ — {take the cook’s man out}"),
        ("training ▸ — {take the cook’s man out}", true, false, "training ▸ — {take the cook’s man out}"),
        ("training ▸ — {take the cook’s man out}", true, true, "training ▸ — {take the cook’s man out}"),
    ];
    static LINE: &[(&str, &str)] = &[
        ("V whistles the cook out. Then hide at once, at the mark", "Space held, then 5, whistles the cook out. Then hide at once, at the mark"),
        ("Hold Space, wheel to kick, let go", "Hold Space, then 2 kicks it"),
        ("xV whistles and V whistlesy but V whistles.", "xV whistles and V whistlesy but Space held, then 5, whistles."),
        ("Right click to knock him out, or left click to use the knife", "Right-click to knock him out"),
        ("Stay low so he doesn't hear you, then left click to knife him", "Stay low so he doesn't hear you, then right-click to knock him out"),
        ("When the guard comes through, step in and left-click to kill him with the knife", "When the guard comes through, step in and right-click to knock him out"),
        ("Press [Space] behind him to grab him, press Space behind the near guard to grab him.", "Press [Space] behind him to grab him, press Space behind the near guard to grab him."),
        ("press [Space to grab him now; Press Space] to grab him; Express Space to grab him; press Space to grab himself", "press [Space to grab him now; Press Space] to grab him; Express Space to grab him; press Space to grab himself"),
        ("When he lights up, creep in and {grab him from behind}. {Take him out} if he turns, or {knock him out}. {Grab him} now. {open the door}", "When he lights up, creep in and press [Space] behind him to grab him. Right-click to knock him out if he turns, or right-click to knock him out. Press [Space] to grab him now. {open the door}"),
        ("{Grab the near guard from behind}, then {grab yours}. {grab from behind} {knock out} {grab o'neil} ▸ {grab  him} {Grab him from the left} {} {1x} {grab him", "Press [Space] behind the near guard to grab him, then press [Space] to grab yours. {grab from behind} {knock out} press [Space] to grab o'neil ▸ {grab  him} {Grab him from the left} {} {1x} {grab him"),
        ("training ▸ — {take the cook’s man out}", "training ▸ — {take the cook’s man out}"),
    ];
    static NAMES: &[(&str, bool)] = &[("grab him", true), ("Grab him", true), ("grab him from behind", true), ("Grab The Near Guard From Behind", true), ("knock him out", true), ("take him out", true), ("Take yours out", true), ("open the door", false), ("grab from behind", false), ("grab him from the left", false), ("grab", false), ("grab ", false), ("", false), ("knock out", false), ("take o'neil out", true), ("'grab him", false), ("grab  him", false), ("hi", false), ("grab\nhim", false), ("Éa", false)];
    #[test]
    fn lower_as_the_typescript() { for (a, b) in LOWER { assert_eq!(lower(a), *b); } }
    #[test]
    fn bare_line_as_the_typescript() { for (a, b) in BARE { assert_eq!(bare_line(a), *b); } }
    #[test]
    fn token_line_as_the_typescript() { for (a, h, b, c) in TOKEN { assert_eq!(token_line(a, Some(*h), Some(*b)), *c); } }
    #[test]
    fn the_page_line_as_the_typescript() { for (a, b) in LINE { assert_eq!(line_on_page(a, 5.0), *b); } }
    #[test]
    fn names_as_the_typescript() { for (a, b) in NAMES { assert_eq!(lesson_token(a), *b, "{}", a); } }
    #[test]
    fn after_finds_the_row_past_last() {
        assert_eq!(after(&[4, 7, 9], Some(7)), 2);
        assert_eq!(after(&[4, 7, 9], Some(9)), 3);
        assert_eq!(after(&[4, 7, 9], Some(3)), 0);   // (trimmed off the front: every row is new)
        assert_eq!(after(&[4, 7, 9], None), 0);
        assert_eq!(after(&[], None), 0);
    }
    #[test]
    fn band_reads_the_meter_only() {
        let meter = |below, above| LessonTest::Meter { which: String::from("noise"), below, above };
        assert!(band(0.5, &meter(Some(1.0), Some(0.0))) && !band(1.0, &meter(Some(1.0), None)) && !band(0.0, &meter(None, Some(0.0))));
        assert!(band(9.0, &LessonTest::Loaded));
        assert!(band_check(&meter(None, None)).is_some() && band_check(&meter(Some(f64::NAN), None)).is_some() && band_check(&meter(Some(1.0), None)).is_none());
        let pace = |below, above| LessonTest::Pace { below, above };
        assert!(band(0.5, &pace(Some(1.0), None)) && !band(1.5, &pace(Some(1.0), None)));
        assert!(band_check(&pace(None, None)).is_some() && band_check(&pace(None, Some(0.5))).is_none());
        assert!(band_check(&LessonTest::Loaded).is_some());   // (t.below and t.above are undefined on any other kind)
    }
    #[test]
    fn the_skeletons_tables_say_what_the_typescript_says() {
        assert_eq!(flags().keys().join(", "), "crouch, sprint, wall, peek, lean, nv, light, holstered, gun, ringsOut, shellsOut, auto, setup, setupShut, dragging, dark, chargeSet, chargeClear, wallBlown, holding, picking, coding, menu, ownRing, silent");
        assert_eq!(hows().join(", "), "takedown, choke, whip, ring, knife");
        assert_eq!(pair_whats().join(", "), "mark, now, set, go, go:F, go:hands, down");
        let kinds = [LessonTest::Pace { below: None, above: None }, LessonTest::Loaded, LessonTest::Following, LessonTest::Cue { name: String::new() }, LessonTest::All { of: Vec::new() }];
        for k in &kinds { assert!(lesson_tests().has(String::from(kind_word(k)))); }
        assert_eq!(stage_index("infiltrate"), 0);
        assert_eq!(stage_index("done"), 3);
        assert_eq!(stage_index("failed"), -1);
    }
    #[test]
    fn inside_is_open_at_its_edges() {
        let r = crate::scene_scenarios::Rect { x0: 0.0, x1: 1.0, z0: 0.0, z1: 1.0 };
        assert!(inside(r, [0.5, 9.0, 0.5]) && !inside(r, [0.0, 0.0, 0.5]) && !inside(r, [0.5, 0.0, 1.0]));
    }
}
