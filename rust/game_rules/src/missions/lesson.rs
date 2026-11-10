pub use ::game_types::lesson::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{format, string::String, vec::Vec};

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
fn bare_line(line: &str) -> String {
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
fn lower(name: &str) -> String {
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
fn token_line(line: &str, held: Option<bool>, bare: Option<bool>) -> String {
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

/// the first row of a front-trimmed log that came after `last`, the logs' rows given by serial; 0: `last` is gone, or there was none
fn after(log: &[u32], last: Option<u32>) -> usize {
    let mut k = log.len();
    while k > 0 && Some(log[k - 1]) != last { k -= 1; }
    k
}

fn inside(r: crate::scene_scenarios::Rect, p: V3) -> bool { p[0] > r.x0 && p[0] < r.x1 && p[2] > r.z0 && p[2] < r.z1 }

/// the player a lesson is taught to, one seat today
fn who(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> PlayerId { seat.player }

/// the door a teammate is on a plain 'stack up' on, by name; None: none
fn on_stack(g: &crate::game::Game, seat: &crate::seats::SeatRef, m: GuardId) -> Option<String> {
    let a = crate::orders::squad_orders_of(seat, m)?;
    if !crate::orders::rows::is(g, a, crate::orders::Is::Stacks) { return None; }
    let d = g.tab.orders_assignment[a].door?;
    Some(g.dr[d].def.name.clone())
}

/// a teammate on his feet
fn operative_up(g: &Game, x: GuardId) -> bool { g.team(x).ours() && !crate::condition::down(g, g.gd[x].ch) }

/// the one partner of a quiet pair: the first operative unit on his feet
fn partner_of(g: &crate::game::Game) -> Option<GuardId> { g.guards.iter().copied().find(|&x| operative_up(g, x)) }

/// is `v` within the record's below and above; a kind with neither field has nothing to refuse
fn band(v: f64, t: &crate::lesson::LessonTest) -> bool {
    let LessonTest::Meter { below, above, .. } = t else { return true };
    below.map_or(true, |b| v < b) && above.map_or(true, |a| v > a)
}

/// a body's place
fn man_pos(g: &Game, x: GuardId) -> V3 { g.ch[g.gd[x].ch].pos }
/// the player's place
fn own_pos(g: &Game, seat: &crate::seats::SeatRef) -> V3 { g.ch[g.pl[seat.player].ch].pos }
/// `onStack(g, m) === door`
fn stack_is(g: &Game, seat: &crate::seats::SeatRef, m: GuardId, door: &str) -> bool { on_stack(g, seat, m).as_deref() == Some(door) }
/// `!!g.orders.of(m)?.arrived`
fn arrived(g: &mut Game, m: GuardId) -> bool { crate::orders::squad_orders_of(&crate::seats::here(g), m).map_or(false, |a| g.tab.orders_assignment[a].arrived) }
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

/// one test asked: lesson.ts LESSON_TESTS, the `ok` of each kind (the `check` of each is check_of)
fn passes(g: &mut Game, t: &LessonTest, s: &mut StepSeen) -> bool {
    use LessonTest as T;
    match t {
        T::In { area } => inside(*area, own_pos(g, &crate::seats::here(g))),
        T::Near { at, r } => { let p = own_pos(g, &crate::seats::here(g)); m::hyp2(p[0] - at[0], p[2] - at[2]) < r.unwrap_or(1.2) }
        T::Flag { flag, value } => flag_of(g, *flag) == value.unwrap_or(true),
        T::Slot { slot, drawn } => { let p = who(g, &crate::seats::here(g)); g.pl[p].slot == *slot && drawn.map_or(true, |d| !g.pl[p].holstered == d) }
        T::Meter { which, .. } => { let p = who(g, &crate::seats::here(g)); let v = if *which == LessonMeter::Noise { g.pl[p].noise } else { g.shown[g.pl[p].ch].visibility }; band(v, t) }
        T::Stage { stage } => stage_index(g.mission.stage.as_str()) >= stage_index(stage.as_str()) && g.mission.stage != crate::game::MissionStage::Failed,
        T::Door { door, state } => ok_door(g, door, *state),
        T::Target { name, on } => ok_target(g, name, *on),
        T::Men { down } => ok_men(g, *down),
        T::Did { what, on, n } => {
            let key = match on { None => what.clone(), Some(on) => format!("{}@door:{}", what, on) };
            seen_n(s, key) >= n.unwrap_or(1.0)
        }
        T::Downed { how, n } => ok_downed(g, how.as_str(), *n),
        T::Body { area } => ok_body(g, &crate::seats::here(g), *area),
        T::Calm { route } => ok_calm(g, route.as_deref()),
        T::Loaded => ok_loaded(g, s),
        T::Reload => ok_reload(g),
        T::Launcher { on, fired } => ok_launcher(g, *on, *fired),
        T::Burst { n } => ok_burst(g, *n),
        T::Threw { kind, quick, area } => ok_threw(g, kind.map(LessonThrown::as_str), *quick, *area, s),
        T::OrderTo { who } => ok_order_to(g, &crate::seats::here(g), *who),
        T::StackOn { door, who, set } => ok_stack_on(g, door, *who, *set),
        T::PairsOn { doors } => ok_pairs_on(g, &crate::seats::here(g), *doors),
        T::Word { doors } => s.words.len() as f64 >= doors.unwrap_or(1.0),
        T::Taken { room } => ok_taken(g, room),
        T::Following => ok_following(g, &crate::seats::here(g)),
        T::Mate { area, at, r, by, away } => ok_mate(g, &crate::seats::here(g), *area, *at, *r, *by, *away),
        T::Pair { what, n } => ok_pair(g, what.as_str(), *n, s),
        T::Cue { name } => crate::rails::rails_cued(g, name),
        T::View { turned, zoomed } => ok_view(g, &crate::seats::here(g), *turned, *zoomed, s),
        T::Squad { area, n } => ok_squad(g, *area, *n),
        T::Stacks { doors, n, set } => ok_stacks(g, doors, *n, *set),
        T::Watching { at, r, n } => ok_watching(g, &crate::seats::here(g), *at, *r, *n),
        T::All { of } => of.iter().all(|u| passes(g, u, s)),
        T::Any { of } => of.iter().any(|u| passes(g, u, s)),
    }
}

fn ok_door(g: &Game, door: &str, state: LessonDoorState) -> bool {
    use LessonDoorState as S;
    use crate::doors::{door_is_closed, doors_by_name};
    let Some(d) = doors_by_name(g, door) else { return false };
    match state {
        S::Open => !door_is_closed(g, d),
        S::Shut => door_is_closed(g, d),
        S::Unlocked => !g.dr[d].leaf.locked || g.dr[d].leaf.lock_broken,
        S::Charged => crate::doortools::door_tools_charge_on(g, &crate::seats::here(g), d).is_some(),
        S::Broken => g.dr[d].leaf.lock_broken,
    }
}

fn ok_target(g: &Game, name: &str, on: bool) -> bool {
    let Some(x) = g.targets.iter().copied().find(|&y| g.tab.fixture[y].name == name) else { return false };
    (g.tab.fixture[x].on && !g.tab.fixture[x].broken && g.tab.fixture[x].factor > 0.05) == on
}

fn ok_men(g: &Game, down: Option<f64>) -> bool {
    let guards = g.guards.clone();
    let n = guards.iter().filter(|&&x| g.team(x).theirs() && crate::condition::down(g, g.gd[x].ch)).count();
    n as f64 >= down.unwrap_or(1.0)
}

fn ok_downed(g: &Game, how: &str, n: Option<f64>) -> bool {
    use crate::chronicle::ChronKind;
    let hits = g.chronicle.events.iter().filter(|e| if how == "knife" { e.kind == ChronKind::Kill && chron_text(e, "weapon") == Some("knife") } else { e.kind == ChronKind::Ko && chron_text(e, "method") == Some(how) }).count();
    hits as f64 >= n.unwrap_or(1.0)
}

fn ok_body(g: &Game, seat: &crate::seats::SeatRef, area: crate::scene_scenarios::Rect) -> bool {
    let (guards, drag) = (g.guards.clone(), g.pl[seat.player].dragging());
    guards.iter().any(|&x| {
        if !crate::condition::down(g, g.gd[x].ch) || drag == Some(x) { return false; }
        let c = g.gd[x].ch;
        inside(area, g.ch[c].bones.get(crate::character::Bone::Hips).copied().unwrap_or(g.ch[c].pos))
    })
}

fn ok_calm(g: &Game, route: Option<&str>) -> bool {
    let guards = g.guards.clone();
    guards.iter().all(|&x| {
        !g.team(x).theirs() || crate::condition::down(g, g.gd[x].ch)
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

fn ok_launcher(g: &Game, on: Option<crate::weapons::RifleAttachment>, fired: bool) -> bool {
    let p = who(g, &crate::seats::here(g));
    let r = g.pl[p].rifle;
    let up = crate::weapons::rifle_attachment(g, r).map_or("none", |a| a.as_str());
    on.map_or("none", |a| a.as_str()) == up && (!fired || (crate::game::player_rifle_up(g, p) && g.fa[r].ring_cd > 0.0))
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
    use crate::throwables::ItemKind;
    if k == Some("flash") || k == Some("frag") { return false; }
    let (me, items) = (g.ch[g.pl[g.player].ch].id, g.items.items.clone());
    items.iter().any(|&x| g.tab.item[x].kind == ItemKind::Smoke && g.tab.item[x].fired && g.tab.item[x].who == me && g.tab.item[x].age <= since && inside(a, g.tab.item[x].pos))
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
    g.staged.rooms.iter().find(|r| r.def.id == room).map_or(false, |r| r.cleared)
}

fn ok_following(g: &Game, seat: &crate::seats::SeatRef) -> bool {
    if !crate::orders::squad_orders_active(g, seat) { return false; }
    let men = squad_men(g, seat, crate::orders::SquadOrdersMenSel::All);
    !men.is_empty() && men.iter().all(|&m| crate::orders::squad_orders_of(seat, m).map_or(false, |a| crate::orders::rows::is(g, a, crate::orders::Is::Follows)))
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
        let Some(id) = crate::orders::squad_orders_of(seat, m) else { return false };
        let row = &g.tab.orders_assignment[id];
        crate::orders::rows::id_is(&row.order, crate::orders::Is::Covers) && row.watch.map_or(false, |w| m::hyp2(w[0] - a[0], w[2] - a[2]) < r)
    }).count();
    hits as f64 >= n.unwrap_or(1.0)
}

// ---- the checks: what is wrong with a record on this floor. A field's type is the skeleton's now, so a check of a type alone (`typeof t.on`) has nothing to ask

fn flag_of(g: &mut Game, flag: LessonFlag) -> bool {
    use LessonFlag as F;
    match flag {
        F::Crouch => flags::crouch(g),
        F::Sprint => flags::sprint(g, &crate::seats::here(g)),
        F::Wall => flags::wall(g),
        F::Peek => flags::peek(g, &crate::seats::here(g)),
        F::Lean => flags::lean(g),
        F::Nv => flags::nv(g),
        F::Light => flags::light(g),
        F::Holstered => flags::holstered(g, &crate::seats::here(g)),
        F::Gun => flags::gun(g),
        F::RingsOut => flags::rings_out(g, &crate::seats::here(g)),
        F::ShellsOut => flags::shells_out(g, &crate::seats::here(g)),
        F::Auto => flags::auto(g),
        F::Setup => flags::setup(g, &crate::seats::here(g)),
        F::SetupShut => flags::setup_shut(g, &crate::seats::here(g)),
        F::Dragging => flags::dragging(g),
        F::Dark => flags::dark(g),
        F::ChargeSet => flags::charge_set(&crate::seats::here(g)),
        F::ChargeClear => flags::charge_clear(g),
        F::WallBlown => flags::wall_blown(&crate::seats::here(g)),
        F::Holding => flags::holding(g, &crate::seats::here(g)),
        F::Picking => flags::picking(g, &crate::seats::here(g)),
        F::Coding => flags::coding(g, &crate::seats::here(g)),
        F::Menu => flags::menu(g),
        F::OwnRing => flags::own_ring(g, &crate::seats::here(g)),
        F::Silent => flags::silent(g),
    }
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
    pub fn dragging(g: &mut crate::game::Game) -> bool { g.pl[g.player].dragging().is_some() }
    /// the mains are off
    pub fn dark(g: &mut crate::game::Game) -> bool { g.light_home.mains.active }
    /// a wall charge is on a wall, waiting on the trigger
    pub fn charge_set(seat: &crate::seats::SeatRef) -> bool { !seat.wall_charge.set.is_empty() }
    /// …on the wall the step's mark is on (a step with none: any wall), with nobody of the squad inside its reach
    pub fn charge_clear(g: &mut crate::game::Game) -> bool {
        let c = g.sets[g.seat].wall_charge.set.first().copied();
        let at = crate::lesson::lesson_step(g).and_then(|step| step.at);
        let Some(c) = c else { return false };
        if !crate::wallcharge::wall_charge_too_close(g, Some(c)).is_empty() { return false; }
        let Some(at) = at else { return true };
        let wall = g.tab.wallcharge_set_wall_charge[c].wall.clone();
        let p = crate::wallcharge::wall_pt(g, &wall, 0.0, 0.0, None);
        (if wall.along_x { m::abs(p[2] - at[2]) } else { m::abs(p[0] - at[0]) }) < 0.4 && m::hyp2(p[0] - at[0], p[2] - at[2]) < 4.0
    }
    /// …and one has made its hole this run
    pub fn wall_blown(seat: &crate::seats::SeatRef) -> bool { seat.wall_charge.stats.holes > 0.0 }
    /// a man in his arm
    pub fn holding(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool { g.pl[seat.player].holding().is_some() }
    pub fn picking(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool { g.pl[seat.player].picking().is_some() }
    pub fn coding(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool { g.pl[seat.player].coding().is_some() }
    /// the order menu is up
    pub fn menu(g: &mut crate::game::Game) -> bool { g.sets[g.seat].orders.menu.is_some() }
    /// his own ring is up, with or without a key down
    pub fn own_ring(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool { let sel = &g.sets[seat.id].hand.sel; sel.open && sel.ring.is_some() }
    /// the squad's posture is SILENT
    pub fn silent(g: &mut crate::game::Game) -> bool { crate::silent::squad_silent(g, &crate::seats::here(g)) }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A FAILURE MEANS: a player is told to use a knife the game does not give him. Whole words only: a longer word that holds the phrase is left alone.
    #[test]
    fn a_line_that_names_the_knife_says_the_knockout() {
        assert_eq!(bare_line("Right click to knock him out, or left click to use the knife"), "Right-click to knock him out");
        assert_eq!(bare_line("Stay low, then left click to knife him"), "Stay low, then right-click to knock him out");
        assert_eq!(bare_line("Step in and left-click to kill him with the knife"), "Step in and right-click to knock him out");
        assert_eq!(bare_line("Hold left click to knife himself"), "Hold left click to knife himself");
        assert_eq!(bare_line("Hold Space, wheel to kick, let go"), "Hold Space, wheel to kick, let go");
    }

    /// A FAILURE MEANS: a line asks for a key that does not do it: a move's token is written out as the keys stand, keeping the capital its author gave it.
    #[test]
    fn a_token_is_written_out_as_the_keys_stand() {
        let (key, click) = ((Some(false), Some(true)), (Some(true), Some(true)));
        assert_eq!(token_line("Now {grab him}.", key.0, key.1), "Now press [Space] to grab him.");
        assert_eq!(token_line("{Grab him from behind}.", key.0, key.1), "Press [Space] behind him to grab him.");
        assert_eq!(token_line("{grab him}", click.0, click.1), "hold left click on him to grab him");
        assert_eq!(token_line("{Grab the near guard from behind}", click.0, click.1), "Hold left click on the near guard from behind to grab him");
        // with a knife to hand the blows say so; with none, each is the knockout
        assert_eq!(token_line("{knock him out}", Some(false), Some(false)), "right-click to knock him out, or left-click to use the knife");
        assert_eq!(token_line("{take him out}", Some(false), Some(false)), "left-click to knife him");
        assert_eq!(token_line("{take him out}", Some(false), Some(true)), "right-click to knock him out");
        assert_eq!(token_line("{knock o'neil out}", Some(false), Some(true)), "right-click to knock o'neil out");
        // what is no token is left as it was written
        for line in ["{open the door}", "{grab him", "{grab him from the left}", "{1x}", "no braces"] {
            assert_eq!(token_line(line, Some(false), Some(true)), line);
        }
    }

    /// The line the HUD shows: tokens out, no knife, and the hand ring's own keys for the whistle and the kick.
    #[test]
    fn the_line_on_the_page_names_the_rings_keys() {
        assert_eq!(line_on_page("V whistles the cook out", 5.0), "Space held, then 5, whistles the cook out");
        assert_eq!(line_on_page("xV whistles but V whistlesy", 5.0), "xV whistles but V whistlesy");
        assert_eq!(line_on_page("Hold Space, wheel to kick, let go", 5.0), "Hold Space, then 2 kicks it");
        assert_eq!(line_on_page("{Take him out} if he turns", 5.0), "Right-click to knock him out if he turns");
    }

    /// The level's check reads this: a token in a line that is no move's stands out as a brace on the player's screen, so it must be refused, and a real one let by.
    #[test]
    fn a_token_names_a_move_and_nothing_else() {
        for name in ["grab him", "Grab him", "grab him from behind", "Grab The Near Guard From Behind", "knock him out", "take him out", "Take yours out", "take o'neil out"] {
            assert!(lesson_token(name), "{}", name);
        }
        for name in ["open the door", "grab from behind", "grab him from the left", "grab", "grab ", "", "knock out", "'grab him", "grab  him", "grab\nhim", "\u{c9}a"] {
            assert!(!lesson_token(name), "{}", name);
        }
    }

    #[test]
    fn after_finds_the_row_past_last() {
        assert_eq!(after(&[4, 7, 9], Some(7)), 2);
        assert_eq!(after(&[4, 7, 9], Some(9)), 3);
        assert_eq!(after(&[4, 7, 9], Some(3)), 0);   // (trimmed off the front: every row is new)
        assert_eq!(after(&[4, 7, 9], None), 0);
        assert_eq!(after(&[], None), 0);
    }

    /// A meter's band is open at both ends; a kind with no band has nothing to refuse.
    #[test]
    fn a_band_is_open_at_its_ends() {
        let meter = |below, above| LessonTest::Meter { which: LessonMeter::Noise, below, above };
        assert!(band(0.5, &meter(Some(1.0), Some(0.0))) && !band(1.0, &meter(Some(1.0), None)) && !band(0.0, &meter(None, Some(0.0))));
        assert!(band(9.0, &LessonTest::Loaded));
    }

    /// A step that waits on a stage passes once the mission is at it or beyond: the stages run in order, and a failed run is on none of them.
    #[test]
    fn the_stages_run_in_order_and_failed_is_none_of_them() {
        let at: Vec<i32> = ["infiltrate", "drive", "exfil", "done"].iter().map(|s| stage_index(s)).collect();
        assert_eq!(at, [0, 1, 2, 3]);
        assert_eq!(stage_index("failed"), -1);
    }

    #[test]
    fn inside_is_open_at_its_edges() {
        let r = crate::scene_scenarios::Rect { x0: 0.0, x1: 1.0, z0: 0.0, z1: 1.0 };
        assert!(inside(r, [0.5, 9.0, 0.5]) && !inside(r, [0.0, 0.0, 0.5]) && !inside(r, [0.5, 0.0, 1.0]));
    }
}

/// A new lesson: nothing done, no run yet.
pub fn lesson_new() -> crate::lesson::Lesson {
    crate::lesson::Lesson::blank()
}

/// The lesson in force (None: not live).
pub fn lesson_data(g: &crate::game::Game) -> Option<&crate::scene_scenarios::LessonData> {
    if g.staged.up.is_none() { return None; }
    g.level.lesson().filter(|l| !l.steps.is_empty())
}

/// The step that is up for the mission that is running: a fresh run is on its first step.
fn lesson_at(g: &crate::game::Game) -> f64 {
    if run_is_mission(g) { g.lesson.i } else { 0.0 }
}

/// The index of the step that is up for the mission that is running.
pub fn lesson_index(g: &crate::game::Game) -> f64 {
    lesson_at(g)
}

/// The step that is up, if any.
pub fn lesson_step(g: &crate::game::Game) -> Option<crate::scene_scenarios::LessonStep> {
    let l = lesson_data(g)?;
    let at = lesson_at(g);
    l.steps.get(at as usize).cloned()
}

/// Once a frame, after updateMission: counts the frame's deeds, asks the step's test, moves on when it is done and settled.
pub fn lesson_tick(g: &mut crate::game::Game, dt: f64) {
    // (the list is shared, so it stands beside `g` with no copy: nothing writes a level's lesson)
    if lesson_data(g).is_none() { return; }
    let Some(l) = g.level.scenarios.as_ref().and_then(|s| s.lesson.clone()) else { return };
    if !run_is_mission(g) {
        g.lesson.run = Some(g.mission.serial);
        g.lesson.i = 0.0;
        g.lesson.seen = None;
    }
    if g.lesson.i >= l.steps.len() as f64 || g.mission.stage == crate::game::MissionStage::Failed || who_down(g, &crate::seats::here(g)) {
        return;
    }
    // The row is lent out of `g.lesson.seen` while `g` is passed along beside it; nothing it calls reads `seen` (private in the TypeScript, read by `settling` only).
    let mut s = match g.lesson.seen.take() {
        Some(s) => s,
        None => lesson_fresh(g, &crate::seats::here(g), Some(1e-6)),
    };
    if s.done_t.is_some() && s.done_i != Some(g.lesson.i) {
        s.done_t = None;
        s.done_i = None;
        s.idle = None;
        s.held = 0.0;
    }
    lesson_count(g, &crate::seats::here(g), &mut s);
    let i = g.lesson.i;
    let t = &l.steps[i as usize];
    if s.done_t.is_none() {
        if passes(g, &t.done, &mut s) {
            s.held += dt;
            let hold = t.hold.unwrap_or(0.0);
            if s.held > hold || !js::truthy(hold) {
                s.done_t = Some(g.time);
                s.done_i = Some(i);
                if g.time - s.t0 <= dt * 1.5 {
                    s.idle = Some(true);
                }
            }
        } else {
            s.held = 0.0;
        }
    }
    let (done_t, idle) = (s.done_t, s.idle.unwrap_or(false));
    g.lesson.seen = Some(s);
    if let Some(done_t) = done_t {
        let waits = (!idle && g.time - done_t < t.settle.unwrap_or(0.0)) || crate::rails::rails_settles(g, &l, i, done_t);
        if !waits {
            lesson_advance(g, &l);
        }
    }
}

/// The step is done: the next comes up, and a checkpoint is asked for unless the level is on rails.
fn lesson_advance(g: &mut crate::game::Game, l: &crate::scene_scenarios::LessonData) {
    g.lesson.i += 1.0;
    g.lesson.seen = Some(lesson_fresh(g, &crate::seats::here(g), None));
    let (i, len) = (g.lesson.i, l.steps.len() as f64);
    if crate::rails::rails_live(g) {
        return;
    }
    let label = if i < len { format!("training {}/{}", i as usize + 1, l.steps.len()) } else { String::from("training done") };
    crate::savestate::checkpoints_request(g, "lesson", &label);
}

/// What a step that has just come up has seen: nothing, counted from now.
fn lesson_fresh(g: &crate::game::Game, seat: &crate::seats::SeatRef, back: Option<f64>) -> crate::lesson::StepSeen {
    let back = back.unwrap_or(0.0);
    let ev = g.chronicle.events.len() as f64;
    let last_order = seat.orders.log.last().map(|r| r.serial);
    let last_pair = g.m.pairorders.pair_log.last().map(|r| r.serial);
    let cam = g.tab.cmd_seat[seat.id].cam;
    let (yaw, dist) = (g.tab.camera[cam].yaw, g.tab.camera[cam].distance);
    crate::lesson::StepSeen {
        t0: g.time - back,
        n: JsMap::new(),
        chron: ev,
        chron0: ev,
        held_at: -1.0,
        last_order,
        last_pair,
        words: JsSet::new(),
        heard: JsSet::new(),
        held: 0.0,
        dist0: dist,
        yaw_last: yaw,
        turned: 0.0,
        done_t: None,
        idle: None,
        done_i: None,
    }
}

/// This frame's deeds into the step's counts.
fn lesson_count(g: &crate::game::Game, seat: &crate::seats::SeatRef, s: &mut crate::lesson::StepSeen) {
    let cam = g.tab.cmd_seat[seat.id].cam;
    if js::truthy(g.rails.fresh) {
        let yaw = g.tab.camera[cam].yaw;
        s.yaw_last = yaw;
        s.dist0 = g.tab.camera[cam].distance;
        s.turned = 0.0;
    }
    {
        let y = g.tab.camera[cam].yaw;
        let mut d = y - s.yaw_last;
        d = m::atan2(m::sin(d), m::cos(d));
        s.turned += m::abs(d);
        s.yaw_last = y;
    }
    if let Some(verb) = seat.act.as_ref().and_then(|a| a.verb) {
        if !who_down(g, &crate::seats::here(g)) {
            add(s, format!("act:{}", verb.as_str()));
        }
    }
    let mut at = 0;
    while at < g.events.len() {
        let (who_did, time, kind, serial) = (g.events[at].who, g.events[at].time, g.events[at].kind, g.events[at].serial);
        if who_did == Some(crate::consts::PLAYER_ID) && !(time <= s.t0) && !s.heard.has(serial) {
            s.heard.add(serial);
            add(s, format!("event:{}", kind.as_str()));
        }
        at += 1;
    }
    {
        let len = g.chronicle.events.len() as f64;
        if len < s.chron {
            s.chron = len;
        }
        if len < s.chron0 {
            s.chron0 = len;
        }
        while s.chron < len {
            add(s, format!("chron:{}", g.chronicle.events[s.chron as usize].kind.as_str()));
            s.chron += 1.0;
        }
        if s.held_at < 0.0 {
            let p = who(g, &crate::seats::here(g));
            if g.sets[crate::seats::seat_of_man(g, p)].hand.throw_held {
                s.held_at = len;
            }
        }
    }
    {
        let serials: Vec<u32> = seat.orders.log.iter().map(|r| r.serial).collect();
        let mut k = after(&serials, s.last_order);
        while k < seat.orders.log.len() {
            add(s, format!("order:{}", seat.orders.log[k].order));
            add(s, format!("order:{}@{}", seat.orders.log[k].order, seat.orders.log[k].target));
            k += 1;
        }
        if let Some(r) = seat.orders.log.last() {
            s.last_order = Some(r.serial);
        }
    }
    for &id in &seat.door_tools.ops {
        let op = &g.tab.doortools_door_op[id];
        if !s.words.has(op.door) && op.beats.iter().any(|b| b.is == crate::doortools::Beat::Word && b.t > s.t0) {
            s.words.add(op.door);
        }
    }
}

/// The run is over, whichever scenario it was.
pub(super) fn lesson_over(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool {
    let stage = g.mission.stage;
    stage == crate::game::MissionStage::Failed
        || stage == crate::game::MissionStage::Done
        || who_down(g, seat)
        || g.staged.over
}

/// The step's deed is done and the moment is playing out: its own `settle`, or the beat before the next step's card.
pub fn lesson_settling(g: &crate::game::Game) -> bool {
    run_is_mission(g) && g.lesson.seen.as_ref().map_or(false, |s| s.done_t.is_some() && s.done_i == Some(g.lesson.i))
}

/// What a seat's HUD is told of the lesson that is live (None: none is).
pub fn lesson_view(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Option<crate::view::LessonView> {
    let l = lesson_data(g)?;
    let i = lesson_at(g);
    let (of, id, asks_view) = {
        let t = l.steps.get(i as usize);
        (l.steps.len(), t.map(|t| t.id.clone()), t.map_or(false, |t| matches!(t.done, LessonTest::View { .. })))
    };
    let frame = crate::rails::LessonAt { l, i: m::min(i, of as f64) };
    Some(crate::view::LessonView {
        step: i as u32,
        of: of as u32,
        finished: id.is_none(),
        id,
        over: lesson_over(g, seat),
        settling: lesson_settling(g),
        asks_view,
        rails: crate::rails::rails_view(g, seat, &frame),
    })
}

/// The objective while a step is up (None: the scenario's own).
pub fn lesson_objective(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Option<crate::lesson::LessonObjective> {
    let l = lesson_data(g)?;
    let i = lesson_at(g);
    if i >= l.steps.len() as f64 {
        return None;
    }
    if lesson_over(g, seat) {
        return None;
    }
    let t = &l.steps[i as usize];
    let mut man = None;
    if let Some(name) = filled(&t.at_man) {
        let mut k = 0;
        while k < g.guards.len() {
            let x = g.guards[k];
            let route = g.gd[x].route;
            if g.tab.route[route].name == name && !crate::condition::down(g, g.gd[x].ch) {
                man = Some(x);
                break;
            }
            k += 1;
        }
    }
    let line = line_of(t);
    let said = lesson_line(g, &line);
    // (each key by its code in brackets, as the big line's: the page has the player's keys, and words them)
    let text = format!("training {}/{} \u{25b8} {}", i as usize + 1, l.steps.len(), said);
    let pos = match man {
        Some(x) => {
            let p = g.ch[g.gd[x].ch].pos;
            Some([p[0], 1.0, p[2]])
        }
        None => t.at,
    };
    Some(crate::lesson::LessonObjective { text, pos })
}

/// A step's line as the keys stand.
pub fn line_of(t: &crate::scene_scenarios::LessonStep) -> String {
    let line = filled(&t.blow_line).or(filled(&t.ring_line)).or(filled(&t.setup_line)).unwrap_or(t.line.as_str());
    String::from(line)
}

/// The taught player is down (`who(g).down`).
fn who_down(g: &Game, seat: &crate::seats::SeatRef) -> bool {
    let p = who(g, seat);
    crate::condition::down(g, g.pl[p].ch)
}

/// One more of a deed, by its key.
fn add(s: &mut crate::lesson::StepSeen, k: String) {
    let n = s.n.get(k.clone()).copied().unwrap_or(0.0);
    s.n.set(k, n + 1.0);
}

/// An optional string as a truthy test takes it: an empty one is no line.
fn filled(o: &Option<String>) -> Option<&str> {
    o.as_deref().filter(|s| !s.is_empty())
}

/// `this.run === g.mission`: the same run, by the Mission's serial.
fn run_is_mission(g: &Game) -> bool {
    g.lesson.run == Some(g.mission.serial)
}
