//! src/game/lesson.ts lines 1 to 298
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
/// the one byte `c` if it is at `at` (`c?`)
fn opt(b: &[u8], at: usize, c: u8) -> usize { if b.get(at) == Some(&c) { at + 1 } else { at } }
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
/// `[Pp]ress \[?Space\]?` at `i`, from a \b: the first letter and where it ends
fn press_space(b: &[u8], i: usize) -> Option<(u8, usize)> {
    if !boundary(b, i) { return None; }
    let p = *b.get(i)?;
    if p != b'P' && p != b'p' { return None; }
    let k = opt(b, lit(b, i + 1, "ress ")?, b'[');
    Some((p, opt(b, lit(b, k, "Space")?, b']')))
}
/// /\b([Pp])ress \[?Space\]? behind (him|the near guard) to grab him\b/g
fn grab_behind(b: &[u8], i: usize) -> Option<(usize, String)> {
    let (p, k) = press_space(b, i)?;
    let k = lit(b, k, " behind ")?;
    let (k, who) = match lit(b, k, "him") { Some(e) => (e, "him"), None => (lit(b, k, "the near guard")?, "the near guard") };
    let k = lit(b, k, " to grab him")?;
    if !boundary(b, k) { return None; }
    Some((k, format!("{}old left click on {} from behind to grab him", if p == b'P' { "H" } else { "h" }, who)))
}
/// /\b([Pp])ress \[?Space\]? to grab him\b/g
fn grab_key(b: &[u8], i: usize) -> Option<(usize, String)> {
    let (p, k) = press_space(b, i)?;
    let k = lit(b, k, " to grab him")?;
    if !boundary(b, k) { return None; }
    Some((k, format!("{}old left click on him to grab him", if p == b'P' { "H" } else { "h" })))
}

/// a line that names the grab's key, while the held click is the only way to a man (lesson.ts stalkLine)
pub fn stalk_line(line: &str) -> String { replace_all(&replace_all(line, grab_behind), grab_key) }

/// a line that names the knife, while there is none: the right click's knockout is the blow (lesson.ts bareLine)
pub fn bare_line(line: &str) -> String {
    let s = replace_words(line, "Right click to knock him out, or left click to use the knife", "Right-click to knock him out");
    let s = replace_words(&s, "left click to knife him", "right-click to knock him out");
    replace_words(&s, "left-click to kill him with the knife", "right-click to knock him out")
}

/// the four tokens, as a level writes them of 'him' (lesson.ts LESSON_TOKENS)
static LESSON_TOKENS: [&str; 4] = ["grab him", "grab him from behind", "knock him out", "take him out"];
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

/// the first letter lower-cased (lesson.ts lower)
pub fn lower(name: &str) -> String {
    let mut c = name.chars();
    match c.next() {
        // (charAt is one UTF-16 unit: half a surrogate pair lower-cases to itself)
        Some(f) if f.len_utf16() == 1 => { let mut s: String = f.to_lowercase().collect(); s.push_str(c.as_str()); s }
        _ => String::from(name),
    }
}

/// is this name (the words between the braces, either capital) one lessonLine writes out? (lesson.ts lessonToken)
pub fn lesson_token(name: &str) -> bool {
    let n = lower(name);
    // /^[A-Za-z]+(?: [A-Za-z']+)*$/
    let good = name.split(' ').enumerate().all(|(i, w)| !w.is_empty() && w.bytes().all(|c| c.is_ascii_alphabetic() || (i > 0 && c == b'\'')));
    if !good { return false; }
    // (…and a 'from' left in the man's words is a 'from behind' misspelt: /(^| )from( |$)/)
    match (0..4).find_map(|row| token_hit(row, &n)) { Some(who) => !who.split(' ').any(|w| w == "from"), None => false }
}

/// a line's tokens written out; on the page `held` is false and `bare` true when not given (lesson.ts tokenLine)
pub fn token_line(line: &str, held: Option<bool>, bare: Option<bool>) -> String {
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

/// a string's `.length` in JavaScript
fn len16(s: &str) -> usize { s.encode_utf16().count() }

/// what the check asks of a line with a brace in it: rails.ts railsFaults calls it, so the rails' part may (lesson.ts lessonTokenFaults)
pub fn lesson_token_faults(w: &str, line: &str, cap: f64, row: Option<&str>, always: Option<bool>) -> Vec<String> {
    let (row, always) = (row.unwrap_or("the objective row"), always.unwrap_or(false));
    if !line.contains('{') && !line.contains('}') { return Vec::new(); }
    let mut out: Vec<String> = Vec::new();
    // /\{([^{}]*)\}/g
    let rest = replace_all(line, |b, i| {
        if b[i] != b'{' { return None; }
        let mut j = i + 1;
        while j < b.len() && b[j] != b'{' && b[j] != b'}' { j += 1; }
        if b.get(j) != Some(&b'}') { return None; }
        let name = &line[i + 1..j];
        if !lesson_token(name) { out.push(format!("{} has {{{}}}, which is no move's token ({})", w, name, LESSON_TOKENS.join(", "))); }
        Some((j + 1, String::new()))
    });
    if rest.contains('{') || rest.contains('}') { out.push(format!("{} has a brace that closes nothing or is never closed", w)); }
    let long = [0, 1, 2, 3].iter().map(|k| len16(&token_line(line, Some(k & 1 != 0), Some(k & 2 != 0)))).max().unwrap_or(0);
    if out.is_empty() && (always || len16(line) as f64 <= cap) && long as f64 > cap {
        out.push(format!("{} is {} characters with its tokens written out: {} holds about {}", w, long, row, cap));
    }
    out
}

/// a step's line as the HUD's row says it now; the rails' frame reads it too, so it is no picture-only code here (lesson.ts lessonLine)
pub fn lesson_line(g: &mut crate::game::Game, written: &str) -> String { line_on_page(written, g.k.cmdring.wedges) }
/// …with the ring's wedges read
fn line_on_page(written: &str, wedges: f64) -> String {
    // (stalkClickOnly() with no game is false on the page: no stalkLine · CLOSE.on && CLOSE.noKnife: bareLine · HANDRING.on: the whistle and the wheel)
    let s = bare_line(&token_line(written, None, None));
    let s = replace_words(&s, "V whistles", &format!("Space held, then {}, whistles", wedges));
    replace_words(&s, "Hold Space, wheel to kick, let go", "Hold Space, then 2 kicks it")
}

// ---- the seams: what skeleton 4 still cannot say

/// lesson.ts:212, 233 S.n.set(...): a test writes the step's counts and `passes` has it as `&StepSeen`
fn seam_seen_set(s: &StepSeen, key: &str, v: f64) { todo!("needs: passes to take s: &mut StepSeen") }
/// lesson.ts:217 r.attachment: rifle_attachment() takes no rifle
fn seam_rifle_attachment(g: &mut Game, r: FirearmId) -> Option<crate::weapons::RifleAttachment> { todo!("needs: rifle_attachment to take g and the FirearmId") }

// ---- the tests

/// one test asked: an unknown kind, or a flag no row has (which throws in the TypeScript and is caught there), is never true (lesson.ts passes)
pub fn passes(g: &mut crate::game::Game, t: &crate::lesson::LessonTest, s: &crate::lesson::StepSeen) -> bool {
    test_ok(g, t, s).unwrap_or(false)
}

/// the first row of a front-trimmed log that came after `last`, the logs' rows given by serial; 0: `last` is gone, or there was none (lesson.ts after)
pub fn after(log: &[u32], last: Option<u32>) -> usize {
    let mut k = log.len();
    while k > 0 && Some(log[k - 1]) != last { k -= 1; }
    k
}

/// a type guard over a record's raw field: a typed LessonTest has no raw fields, and this signature can answer nothing (lesson.ts num; the checks use finite)
pub fn num(v: crate::Opaque) { todo!("needs: nothing calls it, and it can go: `(v: Opaque)` returning nothing can say no answer") }
/// …(lesson.ts str)
pub fn str(v: crate::Opaque) { todo!("needs: nothing calls it, and it can go: `(v: Opaque)` returning nothing can say no answer") }
/// …(lesson.ts rect; the checks use rect_ok)
pub fn rect(v: crate::Opaque) { todo!("needs: nothing calls it, and it can go: `(v: Opaque)` returning nothing can say no answer") }
/// …(lesson.ts vec; the checks use vec_ok)
pub fn vec(v: crate::Opaque) { todo!("needs: nothing calls it, and it can go: `(v: Opaque)` returning nothing can say no answer") }

/// (lesson.ts inside)
pub fn inside(r: crate::scene_scenarios::Rect, p: V3) -> bool { p[0] > r.x0 && p[0] < r.x1 && p[2] > r.z0 && p[2] < r.z1 }

/// the player a lesson is taught to, one seat today (lesson.ts who)
pub fn who(g: &mut crate::game::Game) -> PlayerId { g.player }

/// the door a teammate is on a plain 'stack up' on, by name; None: none (lesson.ts onStack)
pub fn on_stack(g: &mut crate::game::Game, m: GuardId) -> Option<String> {
    let a = crate::orders::squad_orders_of(g, m)?;
    if g.tab.orders_assignment[a].order != "stack" { return None; }
    let d = g.tab.orders_assignment[a].door?;
    Some(g.dr[d].def.name.clone())
}

/// a man no longer on his feet: dead, or out cold (lesson.ts downed)
pub fn downed(g: &mut crate::game::Game, x: GuardId) -> bool { g.gd[x].state == crate::game::GuardState::Dead }

/// a teammate on his feet (lesson.ts:127, 269)
fn operative_up(g: &Game, x: GuardId) -> bool { g.gd[x].unit.is_some() && g.gd[x].team == crate::teams::Team::Operative && g.gd[x].state != crate::game::GuardState::Dead }

/// the one partner of a quiet pair: the first operative unit on his feet (lesson.ts partnerOf)
pub fn partner_of(g: &mut crate::game::Game) -> Option<GuardId> { g.guards.iter().copied().find(|&x| operative_up(g, x)) }

/// is `v` within the record's below and above; a kind with neither field has nothing to refuse (lesson.ts band)
pub fn band(v: f64, t: &crate::lesson::LessonTest) -> bool {
    let LessonTest::Meter { below, above, .. } = t else { return true };
    below.map_or(true, |b| v < b) && above.map_or(true, |a| v > a)
}

/// a body's place
fn man_pos(g: &Game, x: GuardId) -> V3 { g.ch[g.gd[x].ch].pos }
/// the player's place
fn own_pos(g: &Game) -> V3 { g.ch[g.pl[g.player].ch].pos }
/// `onStack(g, m) === door`
fn stack_is(g: &mut Game, m: GuardId, door: &str) -> bool { on_stack(g, m).as_deref() == Some(door) }
/// `!!g.orders.of(m)?.arrived`
fn arrived(g: &mut Game, m: GuardId) -> bool { crate::orders::squad_orders_of(g, m).map_or(false, |a| g.tab.orders_assignment[a].arrived) }
/// `g.orders.men(sel)`
fn squad_men(g: &mut Game, sel: crate::orders::SquadOrdersMenSel) -> Vec<GuardId> { crate::orders::squad_orders_men(g, Some(sel), None) }
/// a text cell of a chronicle row (`e.data.<key>`)
fn chron_text<'a>(e: &'a crate::chronicle::ChronEvent, key: &str) -> Option<&'a str> {
    match e.data.get(String::from(key)) { Some(Some(crate::chronicle::ChronEventDataValue::Text(s))) => Some(s.as_str()), _ => None }
}
/// STAGES.indexOf
fn stage_index(s: &str) -> i32 { ["infiltrate", "drive", "exfil", "done"].iter().position(|w| *w == s).map_or(-1, |i| i as i32) }
/// a count kept in the step's map, as a test reads it
fn seen_n(s: &StepSeen, key: String) -> f64 { s.n.get(key).copied().unwrap_or(0.0) }

/// lesson.ts LESSON_TESTS, the `ok` of each kind (the `check` of each is check_of). None: the row would have thrown, or no such kind
fn test_ok(g: &mut Game, t: &LessonTest, s: &StepSeen) -> Option<bool> {
    use LessonTest as T;
    Some(match t {
        T::Unknown => return None,
        T::In { area } => inside(*area, own_pos(g)),
        T::Near { at, r } => { let p = own_pos(g); m::hyp2(p[0] - at[0], p[2] - at[2]) < r.unwrap_or(1.2) }
        T::Flag { flag, value } => flag_of(g, flag)? == value.unwrap_or(true),
        T::Slot { slot, drawn } => { let p = who(g); g.pl[p].slot == *slot && drawn.map_or(true, |d| !g.pl[p].holstered == d) }
        T::Pace => true,   // (TWOGEARS.on: the wheel is nobody's pace and the line goes by)
        T::Meter { which, .. } => { let p = who(g); let v = if which == "noise" { g.pl[p].noise } else { g.pl[p].visibility }; band(v, t) }
        T::Stage { stage } => stage_index(g.mission.stage.as_str()) >= stage_index(stage) && g.mission.stage != crate::game::MissionStage::Failed,
        T::Door { door, state } => ok_door(g, door, state),
        T::Target { name, on } => ok_target(g, name, *on),
        T::Men { down } => ok_men(g, *down),
        T::Did { what, on, n } => {
            let key = match on { None => what.clone(), Some(on) => format!("{}@door:{}", what, on) };
            seen_n(s, key) >= n.unwrap_or(1.0)
        }
        T::Downed { how, n } => ok_downed(g, how, *n),
        T::Body { area } => ok_body(g, *area),
        T::Calm { route } => ok_calm(g, route.as_deref()),
        T::Loaded => ok_loaded(g, s),
        T::Reload => ok_reload(g),
        T::Launcher { on, fired } => ok_launcher(g, on, *fired),
        T::Burst { n } => ok_burst(g, *n),
        T::Threw { kind, quick, area } => ok_threw(g, kind.as_deref(), *quick, *area, s),
        T::OrderTo { who } => ok_order_to(g, *who),
        T::StackOn { door, who, set } => ok_stack_on(g, door, *who, *set),
        T::PairsOn { doors } => ok_pairs_on(g, *doors),
        T::Word { doors } => s.words.len() as f64 >= doors.unwrap_or(1.0),
        T::Taken { room } => ok_taken(g, room),
        T::Following => ok_following(g),
        T::Mate { area, at, r, by, away } => ok_mate(g, *area, *at, *r, *by, *away),
        T::Pair { what, n } => ok_pair(g, what, *n, s),
        T::Cue { name } => crate::rails::rails_cued(g, name),
        T::View { turned, zoomed, panned } => ok_view(g, *turned, *zoomed, *panned, s),
        T::Squad { area, n } => ok_squad(g, *area, *n),
        T::Stacks { doors, n, set } => ok_stacks(g, doors, *n, *set),
        T::Watching { at, r, n } => ok_watching(g, *at, *r, *n),
        T::All { of } => of.iter().all(|u| passes(g, u, s)),
        T::Any { of } => of.iter().any(|u| passes(g, u, s)),
    })
}

fn ok_door(g: &mut Game, door: &str, state: &str) -> bool {
    use crate::doors::{door_is_closed, door_lock_broken, door_locked, doors_by_name};
    let Some(d) = doors_by_name(g, door) else { return false };
    match state {
        "open" => !door_is_closed(g, d),
        "shut" => door_is_closed(g, d),
        "unlocked" => !door_locked(g, d) || door_lock_broken(g, d),
        "charged" => crate::doortools::door_tools_charge_on(g, d).is_some(),
        _ => door_lock_broken(g, d),
    }
}

fn ok_target(g: &mut Game, name: &str, on: bool) -> bool {
    use crate::sim_lightshome::{fixture_broken, fixture_factor, fixture_on};
    let Some(x) = g.targets.iter().copied().find(|&y| g.tab.sim_lightshome_fixture[y].name == name) else { return false };
    (fixture_on(g, x) && !fixture_broken(g, x) && fixture_factor(g, x) > 0.05) == on
}

fn ok_men(g: &mut Game, down: Option<f64>) -> bool {
    let guards = g.guards.clone();
    let n = guards.iter().filter(|&&x| crate::teams::hostile_to(g, x, crate::teams::HostileToO::Player) && downed(g, x)).count();
    n as f64 >= down.unwrap_or(1.0)
}

fn ok_downed(g: &mut Game, how: &str, n: Option<f64>) -> bool {
    use crate::chronicle::ChronKind;
    let hits = g.chronicle.events.iter().filter(|e| if how == "knife" { e.kind == ChronKind::Kill && chron_text(e, "weapon") == Some("knife") } else { e.kind == ChronKind::Ko && chron_text(e, "method") == Some(how) }).count();
    hits as f64 >= n.unwrap_or(1.0)
}

fn ok_body(g: &mut Game, area: crate::scene_scenarios::Rect) -> bool {
    let (guards, drag) = (g.guards.clone(), g.pl[g.player].dragging);
    guards.iter().any(|&x| {
        if !downed(g, x) || drag == Some(x) { return false; }
        let c = g.gd[x].ch;
        inside(area, g.ch[c].bones.get(crate::character::Bone::Hips).copied().unwrap_or(g.ch[c].pos))
    })
}

fn ok_calm(g: &mut Game, route: Option<&str>) -> bool {
    let guards = g.guards.clone();
    guards.iter().all(|&x| {
        !crate::teams::hostile_to(g, x, crate::teams::HostileToO::Player) || downed(g, x)
            || route.map_or(false, |r| g.tab.route[g.gd[x].route].name != r)
            || g.gd[x].state == crate::game::GuardState::Patrol
    })
}

fn ok_loaded(g: &mut Game, s: &StepSeen) -> bool {
    let p = who(g);
    let Some(w) = crate::game::player_gun_in_hand(g, p) else { return false };
    if crate::weapons::firearm_reloading(g, w) { seam_seen_set(s, "loaded:began", 1.0); return false; }
    if js::truthy(seen_n(s, String::from("loaded:began"))) || g.fa[w].spare.is_empty() { return true; }
    if g.fa[w].chamber != 1.0 { return false; }
    let (mag, most) = (g.fa[w].mag.unwrap_or(0.0), m::max_n(&g.fa[w].spare));
    mag >= m::min(most, crate::weapons::firearm_mag_room(g, w))
}

fn ok_reload(g: &mut Game) -> bool {
    let p = who(g);
    let (pistol, rifle) = (g.pl[p].pistol, g.pl[p].rifle);
    (crate::game::player_pistol_up(g, p) && crate::weapons::firearm_reloading(g, pistol)) || (crate::game::player_rifle_up(g, p) && crate::weapons::firearm_reloading(g, rifle))
}

fn ok_launcher(g: &mut Game, on: &str, fired: bool) -> bool {
    let p = who(g);
    let r = g.pl[p].rifle;
    let up = seam_rifle_attachment(g, r).map_or("none", |a| a.as_str());
    on == up && (!fired || (crate::game::player_rifle_up(g, p) && g.fa[r].ring_cd > 0.0))
}

fn ok_burst(g: &mut Game, n: Option<f64>) -> bool {
    let p = who(g);
    let r = g.pl[p].rifle;
    crate::game::player_rifle_up(g, p) && crate::weapons::firearm_automatic(g, r) && g.fa[r].auto_idx >= n.unwrap_or(3.0)
}

fn ok_threw(g: &mut Game, k: Option<&str>, quick: bool, area: Option<crate::scene_scenarios::Rect>, s: &StepSeen) -> bool {
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
        if lying_can(g, a, k, since) || bang_in(g, a, k) { seam_seen_set(s, &key, 1.0); return true; }
    }
    false
}
/// his own smoke can venting in `a`, on the floor no longer than `since` (lesson.ts:231)
fn lying_can(g: &mut Game, a: crate::scene_scenarios::Rect, k: Option<&str>, since: f64) -> bool {
    use crate::throwables::{item_age, item_fired, item_kind, item_pos, item_who, ItemKind};
    if k == Some("flash") || k == Some("frag") { return false; }
    let (me, items) = (g.ch[g.pl[g.player].ch].id, g.items.items.clone());
    items.iter().any(|&x| item_kind(g, x) == ItemKind::Smoke && item_fired(g, x) && item_who(g, x) == me && item_age(g, x) <= since && inside(a, item_pos(g, x)))
}
/// a bang of that kind in `a` this frame, not a charge's (lesson.ts:232)
fn bang_in(g: &mut Game, a: crate::scene_scenarios::Rect, k: Option<&str>) -> bool {
    if k == Some("smoke") { return false; }
    g.events.iter().any(|e| e.kind == crate::director::EventKind::Bang && !e.charge.unwrap_or(false) && (k.is_none() || (k == Some("frag")) == e.frag.unwrap_or(false)) && inside(a, e.pos))
}

fn ok_order_to(g: &mut Game, want: crate::orders::Who) -> bool {
    use crate::orders::Who;
    if !crate::orders::squad_orders_active(g) { return false; }
    match (g.orders.who, want) { (Who::All, Who::All) | (Who::Red, Who::Red) | (Who::Blue, Who::Blue) => true, (Who::Num(a), Who::Num(b)) => a == b, _ => false }
}

fn ok_stack_on(g: &mut Game, door: &str, sel: Option<crate::orders::SquadOrdersMenSel>, set: bool) -> bool {
    if !crate::orders::squad_orders_active(g) { return false; }
    let men = squad_men(g, sel.unwrap_or(crate::orders::SquadOrdersMenSel::All));
    !men.is_empty() && men.iter().all(|&m| stack_is(g, m, door) && (!set || arrived(g, m)))
}

fn ok_pairs_on(g: &mut Game, want: Option<f64>) -> bool {
    use crate::{game::PairId, orders::SquadOrdersMenSel as Sel};
    if !crate::orders::squad_orders_active(g) { return false; }
    let mut doors: JsSet<String> = JsSet::new();
    for id in crate::orders::squad_orders_pairs(g) {
        let men = squad_men(g, match id { PairId::Red => Sel::Red, PairId::Blue => Sel::Blue });
        let d = match men.first() { Some(&m) => on_stack(g, m), None => None };
        let Some(d) = d.filter(|w| !w.is_empty()) else { continue };
        if men.iter().all(|&m| stack_is(g, m, &d)) { doors.add(d); }
    }
    doors.len() as f64 >= want.unwrap_or(2.0)
}

fn ok_taken(g: &mut Game, room: &str) -> bool {
    if !crate::roomclear::room_clear_scenario_active(g) { return false; }
    g.room_clear.rooms.iter().find(|r| r.def.id == room).map_or(false, |r| r.cleared)
}

fn ok_following(g: &mut Game) -> bool {
    if !crate::orders::squad_orders_active(g) { return false; }
    let men = squad_men(g, crate::orders::SquadOrdersMenSel::All);
    !men.is_empty() && men.iter().all(|&m| crate::orders::squad_orders_of(g, m).map_or(false, |a| g.tab.orders_assignment[a].order == "fallIn"))
}

fn ok_mate(g: &mut Game, area: Option<crate::scene_scenarios::Rect>, at: Option<V3>, r: Option<f64>, by: Option<f64>, away: Option<f64>) -> bool {
    let Some(mate) = partner_of(g) else { return false };
    let (p, s) = (man_pos(g, mate), own_pos(g));
    let d = m::hyp2(p[0] - s[0], p[2] - s[2]);
    area.map_or(true, |a| inside(a, p))
        && at.map_or(true, |a| m::hyp2(p[0] - a[0], p[2] - a[2]) < r.unwrap_or(1.5))
        && by.map_or(true, |by| d < by)
        && away.map_or(true, |away| d > away)
}

fn ok_pair(g: &mut Game, what: &str, n: Option<f64>, s: &StepSeen) -> bool {
    let serials: Vec<u32> = g.m.pairorders.pair_log.iter().map(|row| row.serial).collect();
    let from = after(&serials, s.last_pair);
    let hits = g.m.pairorders.pair_log.iter().skip(from).filter(|row| row.what.starts_with(what)).count();
    hits as f64 >= n.unwrap_or(1.0)
}

fn ok_view(g: &mut Game, turned: Option<f64>, zoomed: Option<f64>, panned: Option<f64>, s: &StepSeen) -> bool {
    let cam = g.tab.cmd_seat[g.seat].cam;
    let distance = g.tab.camera[cam].distance;
    turned.map_or(true, |d| s.turned * 180.0 / m::PI >= d)
        && zoomed.map_or(true, |z| m::abs(m::log(distance / s.dist0)) >= m::log(z))
        && panned.map_or(true, |d| crate::rails::rails_panned(g) >= d)
}

fn ok_squad(g: &mut Game, area: crate::scene_scenarios::Rect, n: Option<f64>) -> bool {
    let men: Vec<GuardId> = g.guards.iter().copied().filter(|&x| operative_up(g, x)).collect();
    let inn = men.iter().filter(|&&x| inside(area, man_pos(g, x))).count();
    !men.is_empty() && inn as f64 >= m::min(men.len() as f64, n.unwrap_or(men.len() as f64))
}

fn ok_stacks(g: &mut Game, doors: &[String], n: Option<f64>, set: bool) -> bool {
    if !crate::orders::squad_orders_active(g) { return false; }
    let men = squad_men(g, crate::orders::SquadOrdersMenSel::All);
    doors.iter().all(|d| men.iter().filter(|&&m| stack_is(g, m, d) && (!set || arrived(g, m))).count() as f64 >= n.unwrap_or(1.0))
}

fn ok_watching(g: &mut Game, a: V3, r: Option<f64>, n: Option<f64>) -> bool {
    if !crate::orders::squad_orders_active(g) { return false; }
    let r = r.unwrap_or(3.0);
    let men = squad_men(g, crate::orders::SquadOrdersMenSel::All);
    let hits = men.iter().filter(|&&m| {
        let Some(id) = crate::orders::squad_orders_of(g, m) else { return false };
        let row = &g.tab.orders_assignment[id];
        row.order == "cover" && row.watch.map_or(false, |w| m::hyp2(w[0] - a[0], w[2] - a[2]) < r)
    }).count();
    hits as f64 >= n.unwrap_or(1.0)
}

// ---- the checks: what is wrong with a record on this floor. A field's type is the skeleton's now, so a check of a type alone (`typeof t.on`) has nothing to ask

/// Object.keys(LESSON_TESTS)
static TEST_KINDS: [&str; 34] = ["in", "near", "flag", "slot", "pace", "meter", "stage", "door", "target", "men", "did", "downed", "body", "calm", "loaded", "reload", "launcher", "burst", "threw", "orderTo", "stackOn", "pairsOn", "word", "taken", "following", "mate", "pair", "cue", "view", "squad", "stacks", "watching", "all", "any"];
/// Object.keys(FLAGS)
static FLAG_WORDS: [&str; 25] = ["crouch", "sprint", "wall", "peek", "lean", "nv", "light", "holstered", "gun", "ringsOut", "shellsOut", "auto", "setup", "setupShut", "dragging", "dark", "chargeSet", "chargeClear", "wallBlown", "holding", "picking", "coding", "menu", "ownRing", "silent"];
/// lesson.ts HOWS
static HOWS_WORDS: [&str; 5] = ["takedown", "choke", "whip", "ring", "knife"];
/// lesson.ts PAIR_WHATS
static PAIR_WORDS: [&str; 7] = ["mark", "now", "set", "go", "go:F", "go:hands", "down"];

/// lesson.ts num of a number
fn finite(v: f64) -> bool { v.is_finite() }
/// lesson.ts rect
fn rect_ok(r: crate::scene_scenarios::Rect) -> bool { finite(r.x0) && finite(r.x1) && finite(r.z0) && finite(r.z1) && r.x0 < r.x1 && r.z0 < r.z1 }
/// lesson.ts vec
fn vec_ok(v: V3) -> bool { v.iter().all(|&c| finite(c)) }
/// `t.n === undefined || (num(t.n) && t.n >= least)`
fn count_ok(n: Option<f64>, least: f64) -> bool { n.map_or(true, |v| finite(v) && v >= least) }
/// …and `v > 0`
fn metres_ok(v: Option<f64>) -> bool { v.map_or(true, |v| finite(v) && v > 0.0) }
/// /[\w.-]+/ whole
fn is_id(s: &str) -> bool { !s.is_empty() && s.bytes().all(|c| is_word(c) || c == b'.' || c == b'-') }
/// /[\w.'-]+/ whole
fn is_name_word(s: &str) -> bool { !s.is_empty() && s.bytes().all(|c| is_word(c) || c == b'.' || c == b'\'' || c == b'-') }
/// the regular expression of a `did` row's `what` (lesson.ts:188). None: no match; Some(None): no target; Some(Some(x)): x, the target after the `@`
fn what_target(what: &str) -> Option<Option<&str>> {
    let (verb, rest) = what.split_once(':')?;
    if !["act", "event", "chron", "order"].contains(&verb) { return None; }
    if is_id(rest) { return Some(None); }
    if verb != "order" { return None; }
    let (id, target) = rest.split_once('@')?;
    if !is_id(id) { return None; }
    let good = match target.strip_prefix("foe:") {
        Some(name) => name.split(' ').all(is_name_word),
        None => matches!(target, "thing:breaker" | "thing:objective" | "thing:device"),
    };
    if good { Some(Some(target)) } else { None }
}

const NO_RECT: &str = "area: a rect { x0, x1, z0, z1 }";
const NO_FLOOR: &str = "area: no free floor inside it";

/// `!rect(area) ? … : freeIn(g, area) ? null : …`
fn check_area(g: &mut Game, area: crate::scene_scenarios::Rect) -> Option<String> {
    if !rect_ok(area) { return Some(String::from(NO_RECT)); }
    if free_in(g, area) { None } else { Some(String::from(NO_FLOOR)) }
}
/// lesson.ts:188-191
fn check_did(g: &mut Game, what: &str, on: Option<&str>) -> Option<String> {
    let Some(target) = what_target(what) else {
        return Some(String::from("what: 'act:<verb>', 'event:<kind>', 'chron:<kind>', 'order:<id>', 'order:<id>@foe:<callsign>' (an order given ON that man) or 'order:<id>@thing:breaker|objective|device' (a door is `on`)"));
    };
    if let Some(foe) = target.and_then(|x| x.strip_prefix("foe:")) {
        let foes = |g: &Game, x: GuardId| g.gd[x].team != crate::teams::Team::Operative;
        let up = g.guards.iter().any(|&x| foes(g, x) && g.gd[x].state != crate::game::GuardState::Dead);
        if up && !g.guards.iter().any(|&x| foes(g, x) && g.gd[x].callsign == foe) { return Some(format!("what: no man called '{}' on this floor", foe)); }
    }
    let on = on?;
    if target.is_some() { return Some(String::from("on: not with a target in `what`")); }
    if !what.starts_with("order:") { return Some(String::from("on: only with an order")); }
    if on.is_empty() || crate::doors::doors_by_name(g, on).is_none() { return Some(format!("on: no door '{}'", on)); }
    None
}
/// lesson.ts LESSON_TESTS, the `check` of each kind
fn check_of(g: &mut Game, t: &LessonTest) -> Option<String> {
    use crate::doors::doors_by_name;
    use LessonTest as T;
    let say = |s: &str| Some(String::from(s));
    match t {
        T::Unknown | T::Slot { .. } | T::Loaded | T::Reload | T::Following => None,   // (a slot is a SlotKind: all five are the check's)
        T::In { area } | T::Body { area } => check_area(g, *area),
        T::Near { at, r } => if !vec_ok(*at) { say("at: a Vec3") } else if r.map_or(false, |r| !finite(r)) { say("r: metres") } else { None },
        T::Flag { flag, .. } => if FLAG_WORDS.contains(&flag.as_str()) { None } else { Some(format!("flag: one of {}", FLAG_WORDS.join(", "))) },
        T::Pace => band_check(t),
        T::Meter { which, .. } => if which != "light" && which != "noise" { say("which: 'light' or 'noise'") } else { band_check(t) },
        T::Stage { stage } => if stage_index(stage) > 0 { None } else { say("stage: drive (the place reached), exfil (the thing taken) or done") },
        T::Door { door, state } => {
            if doors_by_name(g, door).is_none() { return Some(format!("door: no door named '{}'", door)); }
            if ["open", "shut", "unlocked", "broken", "charged"].contains(&state.as_str()) { None } else { say("state: open, shut, unlocked, broken or charged") }
        }
        T::Target { name, .. } => if g.targets.iter().any(|&y| g.tab.sim_lightshome_fixture[y].name == *name) { None } else { Some(format!("name: no light or device named '{}'", name)) },
        T::Men { down } => if count_ok(*down, 1.0) { None } else { say("down: how many, ≥ 1") },
        T::Did { what, on, .. } => check_did(g, what, on.as_deref()),
        T::Downed { how, n } => {
            if !HOWS_WORDS.contains(&how.as_str()) { return Some(format!("how: one of {}", HOWS_WORDS.join(", "))); }
            if count_ok(*n, 1.0) { None } else { say("n: how many, ≥ 1") }
        }
        T::Calm { route } => {
            let Some(route) = route else { return None };
            if g.level.routes.iter().any(|&r| g.tab.route[r].name == *route) { None } else { Some(format!("route: no patrol route named '{}'", route)) }
        }
        T::Launcher { on, fired } => {
            if !["ring", "he40", "none"].contains(&on.as_str()) { return say("on: 'ring', 'he40' or 'none'"); }
            if *fired && on == "none" { say("fired: nothing is fired with no attachment on") } else { None }
        }
        T::Burst { n } => if count_ok(*n, 2.0) { None } else { say("n: how many rounds in the one string, ≥ 2") },
        T::Threw { kind, quick, area } => {
            let k = kind.as_deref();
            if area.map_or(false, |a| !rect_ok(a)) { return Some(String::from(NO_RECT)); }
            if area.is_some() && (*quick || k == Some("he40")) { return say("area: a thrown can or frag (not the quick throw, not the 40 mm)"); }
            if k.map_or(false, |k| !["smoke", "flash", "frag", "he40"].contains(&k)) { return say("kind: 'smoke', 'flash', 'frag' or 'he40'"); }
            if *quick && (k == Some("frag") || k == Some("he40")) { say("quick: G throws only smoke and the stun can") } else { None }
        }
        T::OrderTo { who } => if matches!(who, crate::orders::Who::Num(_)) { say("who: 'all', 'red' or 'blue'") } else { None },
        T::StackOn { door, who, .. } => {
            use crate::orders::SquadOrdersMenSel as Sel;
            if doors_by_name(g, door).is_none() { return Some(format!("door: no door named '{}'", door)); }
            if matches!(who, Some(Sel::Num(_) | Sel::Guard(_))) { say("who: 'all', 'red' or 'blue'") } else { None }
        }
        T::PairsOn { doors } => if count_ok(*doors, 1.0) { None } else { say("doors: how many different doors, ≥ 1") },
        T::Word { doors } => if count_ok(*doors, 1.0) { None } else { say("doors: how many doors go on it, ≥ 1") },
        T::Taken { room } => {
            let has = g.level.scenarios.as_ref().and_then(|s| s.room_clear.as_ref()).map_or(false, |c| c.rooms.iter().any(|r| r.id == *room));
            if has { None } else { Some(format!("room: no room '{}' in the level's roomClear block", room)) }
        }
        T::Mate { area, at, r, by, away } => {
            if area.is_none() && at.is_none() && by.is_none() && away.is_none() { return say("one of area (a rect), at (a Vec3, + r), by (metres from the player), away (metres)"); }
            if let Some(a) = area { if let Some(why) = check_area(g, *a) { return Some(why); } }
            if at.map_or(false, |a| !vec_ok(a)) { return say("at: a Vec3"); }
            if metres_ok(*r) && metres_ok(*by) && metres_ok(*away) { None } else { say("r / by / away: metres, > 0") }
        }
        T::Pair { what, n } => {
            if !PAIR_WORDS.contains(&what.as_str()) { return Some(format!("what: one of {}", PAIR_WORDS.join(", "))); }
            if count_ok(*n, 1.0) { None } else { say("n: how many, ≥ 1") }
        }
        T::Cue { name } => if name.is_empty() { say("name: the beat the level's script ends with c.signal('done:<name>')") } else { None },
        T::View { turned, zoomed, panned } => {
            if turned.is_none() && zoomed.is_none() && panned.is_none() { return say("turned (degrees), zoomed (a ratio over 1) and / or panned (metres)"); }
            if turned.map_or(false, |v| !finite(v)) || panned.map_or(false, |v| !finite(v)) || zoomed.map_or(false, |v| !(finite(v) && v > 1.0)) { say("turned, panned: numbers · zoomed: a ratio over 1") } else { None }
        }
        T::Squad { area, n } => {
            if let Some(why) = check_area(g, *area) { return Some(why); }
            if count_ok(*n, 1.0) { None } else { say("n: how many, ≥ 1") }
        }
        T::Stacks { doors, n, .. } => {
            if doors.is_empty() { return say("doors: a list of door names"); }
            for d in doors { if doors_by_name(g, d).is_none() { return Some(format!("doors: no door named '{}'", d)); } }
            if count_ok(*n, 1.0) { None } else { say("n: how many men a door, ≥ 1") }
        }
        T::Watching { at, r, n } => {
            if !vec_ok(*at) { return say("at: a Vec3"); }
            if !metres_ok(*r) { return say("r: metres, > 0"); }
            if count_ok(*n, 1.0) { None } else { say("n: how many men, ≥ 1") }
        }
        T::All { .. } | T::Any { .. } => of_check(g, t),
    }
}

/// a row's `below` and `above`; `Pace` lost its two in the skeleton's enum, and on the page nothing asks them (lesson.ts bandCheck)
pub fn band_check(t: &crate::lesson::LessonTest) -> Option<String> {
    let (below, above) = match t { LessonTest::Pace => return None, LessonTest::Meter { below, above, .. } => (*below, *above), _ => (None, None) };
    let bad = |v: Option<f64>| v.map_or(false, |v| !finite(v));
    if (below.is_none() && above.is_none()) || bad(below) || bad(above) { Some(String::from("below and / or above: numbers")) } else { None }
}

/// (lesson.ts ofCheck)
pub fn of_check(g: &mut crate::game::Game, t: &crate::lesson::LessonTest) -> Option<String> {
    let of = match t { LessonTest::All { of } | LessonTest::Any { of } if !of.is_empty() => of, _ => return Some(String::from("of: a non-empty list of tests")) };
    for u in of { if let Some(why) = test_fault(g, u) { return Some(why); } }
    None
}

/// is there a free spot of floor in the rect (lesson.ts freeIn)
pub fn free_in(g: &mut crate::game::Game, r: crate::scene_scenarios::Rect) -> bool {
    let mut x = r.x0 + 0.2;
    while x < r.x1 {
        let mut z = r.z0 + 0.2;
        while z < r.z1 {
            if !crate::scene_collision::nav_grid_is_blocked(g, x, z) { return true; }
            z += 0.3;
        }
        x += 0.3;
    }
    false
}

/// a kind's word (`t.is`)
fn kind_word(t: &LessonTest) -> &'static str {
    use LessonTest as T;
    match t {
        T::Unknown => "", T::In { .. } => "in", T::Near { .. } => "near", T::Flag { .. } => "flag", T::Slot { .. } => "slot", T::Pace => "pace", T::Meter { .. } => "meter",
        T::Stage { .. } => "stage", T::Door { .. } => "door", T::Target { .. } => "target", T::Men { .. } => "men", T::Did { .. } => "did", T::Downed { .. } => "downed",
        T::Body { .. } => "body", T::Calm { .. } => "calm", T::Loaded => "loaded", T::Reload => "reload", T::Launcher { .. } => "launcher", T::Burst { .. } => "burst",
        T::Threw { .. } => "threw", T::OrderTo { .. } => "orderTo", T::StackOn { .. } => "stackOn", T::PairsOn { .. } => "pairsOn", T::Word { .. } => "word", T::Taken { .. } => "taken",
        T::Following => "following", T::Mate { .. } => "mate", T::Pair { .. } => "pair", T::Cue { .. } => "cue", T::View { .. } => "view", T::Squad { .. } => "squad",
        T::Stacks { .. } => "stacks", T::Watching { .. } => "watching", T::All { .. } => "all", T::Any { .. } => "any",
    }
}

/// what is wrong with one test record on this floor; None: nothing (lesson.ts testFault)
pub fn test_fault(g: &mut crate::game::Game, t: &crate::lesson::LessonTest) -> Option<String> {
    if matches!(t, LessonTest::Unknown) { return Some(format!("no test kind ({})", TEST_KINDS.join(", "))); }
    check_of(g, t).map(|why| format!("{} — {}", kind_word(t), why))
}

/// lesson.ts FLAGS by the word a test names; None: no such flag
fn flag_of(g: &mut Game, name: &str) -> Option<bool> {
    Some(match name {
        "crouch" => flags::crouch(g),
        "sprint" => flags::sprint(g),
        "wall" => flags::wall(g),
        "peek" => flags::peek(g),
        "lean" => flags::lean(g),
        "nv" => flags::nv(g),
        "light" => flags::light(g),
        "holstered" => flags::holstered(g),
        "gun" => flags::gun(g),
        "ringsOut" => flags::rings_out(g),
        "shellsOut" => flags::shells_out(g),
        "auto" => flags::auto(g),
        "setup" => flags::setup(g),
        "setupShut" => flags::setup_shut(g),
        "dragging" => flags::dragging(g),
        "dark" => flags::dark(g),
        "chargeSet" => flags::charge_set(g),
        "chargeClear" => flags::charge_clear(g),
        "wallBlown" => flags::wall_blown(g),
        "holding" => flags::holding(g),
        "picking" => flags::picking(g),
        "coding" => flags::coding(g),
        "menu" => flags::menu(g),
        "ownRing" => flags::own_ring(g),
        "silent" => flags::silent(g),
        _ => return None,
    })
}

/// the flags a `flag` test can read off the player and the team's posture, each true while it holds (lesson.ts FLAGS)
pub mod flags {
    use super::*;

    /// (lesson.ts FLAGS.crouch)
    pub fn crouch(g: &mut crate::game::Game) -> bool { g.pl[g.player].crouch }
    /// (lesson.ts FLAGS.sprint)
    pub fn sprint(g: &mut crate::game::Game) -> bool { g.pl[g.player].sprinting }
    /// back to a wall (lesson.ts FLAGS.wall)
    pub fn wall(g: &mut crate::game::Game) -> bool { g.pl[g.player].wall.is_some() }
    /// …and leaning out of its open end (lesson.ts FLAGS.peek)
    pub fn peek(g: &mut crate::game::Game) -> bool { g.pl[g.player].wall.as_ref().map_or(0.0, |w| w.peek) > 0.5 }
    /// out round a wall's open end, whichever hold (lesson.ts FLAGS.lean)
    pub fn lean(g: &mut crate::game::Game) -> bool {
        let Some(w) = g.pl[g.player].wall.as_ref() else { return false };
        w.lean.unwrap_or(0.0) > 0.5 || (w.kind != Some(crate::player::WallHoldKind::Stack) && w.peek > 0.5)
    }
    /// (lesson.ts FLAGS.nv)
    pub fn nv(g: &mut crate::game::Game) -> bool { g.pl[g.player].nv }
    /// the weapon light of the gun in hand (lesson.ts FLAGS.light)
    pub fn light(g: &mut crate::game::Game) -> bool {
        let p = g.player;
        let (pistol, rifle) = (g.pl[p].pistol, g.pl[p].rifle);
        (crate::game::player_pistol_up(g, p) && g.fa[pistol].light_on) || (crate::game::player_rifle_up(g, p) && g.fa[rifle].light_on)
    }
    /// (lesson.ts FLAGS.holstered)
    pub fn holstered(g: &mut crate::game::Game) -> bool { g.pl[g.player].holstered }
    /// a gun in the hands (lesson.ts FLAGS.gun)
    pub fn gun(g: &mut crate::game::Game) -> bool { let p = g.player; crate::game::player_gun_in_hand(g, p).is_some() }
    /// (lesson.ts FLAGS.ringsOut)
    pub fn rings_out(g: &mut crate::game::Game) -> bool { g.fa[g.pl[g.player].rifle].rings <= 0.0 }
    /// (lesson.ts FLAGS.shellsOut)
    pub fn shells_out(g: &mut crate::game::Game) -> bool { g.fa[g.pl[g.player].rifle].shells <= 0.0 }
    /// (lesson.ts FLAGS.auto)
    pub fn auto(g: &mut crate::game::Game) -> bool { g.fa[g.pl[g.player].rifle].mode != crate::weapons::FireMode::Semi }
    /// the setup of the gun in his hands is up; SLOTSETUP.on is settled true (lesson.ts FLAGS.setup)
    pub fn setup(g: &mut crate::game::Game) -> bool { crate::slotsetup::slot_setup_open(g) }
    /// (lesson.ts FLAGS.setupShut)
    pub fn setup_shut(g: &mut crate::game::Game) -> bool { !crate::slotsetup::slot_setup_open(g) }
    /// (lesson.ts FLAGS.dragging)
    pub fn dragging(g: &mut crate::game::Game) -> bool { g.pl[g.player].dragging.is_some() }
    /// the mains are off (lesson.ts FLAGS.dark)
    pub fn dark(g: &mut crate::game::Game) -> bool { crate::sim_lightshome::mains_active(g) }
    /// a wall charge is on a wall, waiting on the trigger (lesson.ts FLAGS.chargeSet)
    pub fn charge_set(g: &mut crate::game::Game) -> bool { !g.wall_charge.set.is_empty() }
    /// …on the wall the step's mark is on (a step with none: any wall), with nobody of the squad inside its reach (lesson.ts FLAGS.chargeClear)
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
    /// …and one has made its hole this run (lesson.ts FLAGS.wallBlown)
    pub fn wall_blown(g: &mut crate::game::Game) -> bool { g.wall_charge.stats.holes > 0.0 }
    /// a man in his arm (lesson.ts FLAGS.holding)
    pub fn holding(g: &mut crate::game::Game) -> bool { g.pl[g.player].holding.is_some() }
    /// (lesson.ts FLAGS.picking)
    pub fn picking(g: &mut crate::game::Game) -> bool { g.pl[g.player].picking.is_some() }
    /// (lesson.ts FLAGS.coding)
    pub fn coding(g: &mut crate::game::Game) -> bool { g.pl[g.player].coding.is_some() }
    /// the order menu is up (lesson.ts FLAGS.menu)
    pub fn menu(g: &mut crate::game::Game) -> bool { g.orders.menu.is_some() }
    /// his own ring is up, with or without a key down (lesson.ts FLAGS.ownRing)
    pub fn own_ring(g: &mut crate::game::Game) -> bool { let sel = &g.pl[g.player].sel; sel.open && sel.ring.is_some() }
    /// the squad's posture is SILENT (lesson.ts FLAGS.silent)
    pub fn silent(g: &mut crate::game::Game) -> bool { crate::silent::squad_silent(g) }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// the TypeScript's own answers (lesson.ts:35-78 run under bun, with the page's values)
    static LOWER: &[(&str, &str)] = &[("Grab", "grab"), ("grab", "grab"), ("", ""), ("É1", "é1"), ("𐐀x", "𐐀x")];
    static STALK: &[(&str, &str)] = &[
        ("V whistles the cook out. Then hide at once, at the mark", "V whistles the cook out. Then hide at once, at the mark"),
        ("Hold Space, wheel to kick, let go", "Hold Space, wheel to kick, let go"),
        ("xV whistles and V whistlesy but V whistles.", "xV whistles and V whistlesy but V whistles."),
        ("Right click to knock him out, or left click to use the knife", "Right click to knock him out, or left click to use the knife"),
        ("Stay low so he doesn't hear you, then left click to knife him", "Stay low so he doesn't hear you, then left click to knife him"),
        ("When the guard comes through, step in and left-click to kill him with the knife", "When the guard comes through, step in and left-click to kill him with the knife"),
        ("Press [Space] behind him to grab him, press Space behind the near guard to grab him.", "Hold left click on him from behind to grab him, hold left click on the near guard from behind to grab him."),
        ("press [Space to grab him now; Press Space] to grab him; Express Space to grab him; press Space to grab himself", "hold left click on him to grab him now; Hold left click on him to grab him; Express Space to grab him; press Space to grab himself"),
        ("When he lights up, creep in and {grab him from behind}. {Take him out} if he turns, or {knock him out}. {Grab him} now. {open the door}", "When he lights up, creep in and {grab him from behind}. {Take him out} if he turns, or {knock him out}. {Grab him} now. {open the door}"),
        ("{Grab the near guard from behind}, then {grab yours}. {grab from behind} {knock out} {grab o'neil} ▸ {grab  him} {Grab him from the left} {} {1x} {grab him", "{Grab the near guard from behind}, then {grab yours}. {grab from behind} {knock out} {grab o'neil} ▸ {grab  him} {Grab him from the left} {} {1x} {grab him"),
        ("training ▸ — {take the cook’s man out}", "training ▸ — {take the cook’s man out}"),
    ];
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
    static FAULTS: &[(&str, &str, f64, Option<&str>, Option<bool>, &[&str])] = &[
        ("t", "no braces", 96.0, None, None, &[]),
        ("t", "{grab him from behnd}", 96.0, None, None, &["t has {grab him from behnd}, which is no move's token (grab him, grab him from behind, knock him out, take him out)"]),
        ("t", "{knock-him-out} x", 96.0, None, None, &["t has {knock-him-out}, which is no move's token (grab him, grab him from behind, knock him out, take him out)"]),
        ("t", "{grab him", 96.0, None, None, &["t has a brace that closes nothing or is never closed"]),
        ("t", "grab him}", 96.0, None, None, &["t has a brace that closes nothing or is never closed"]),
        ("t", "{a{grab him}}", 96.0, None, None, &["t has a brace that closes nothing or is never closed"]),
        ("t", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx {grab him from behind}", 96.0, Some("a card"), None, &["t is 117 characters with its tokens written out: a card holds about 96"]),
        ("t", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx {grab him from behind}", 96.0, None, None, &[]),
        ("t", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx {grab him from behind}", 96.0, None, Some(true), &["t is 137 characters with its tokens written out: the objective row holds about 96"]),
        ("▸ w", "▸▸▸ {take him out} éééééééééééééééééééééééééééééé", 50.0, None, None, &["▸ w is 63 characters with its tokens written out: the objective row holds about 50"]),
    ];

    #[test]
    fn lower_as_the_typescript() { for (a, b) in LOWER { assert_eq!(lower(a), *b); } }
    #[test]
    fn stalk_line_as_the_typescript() { for (a, b) in STALK { assert_eq!(stalk_line(a), *b); } }
    #[test]
    fn bare_line_as_the_typescript() { for (a, b) in BARE { assert_eq!(bare_line(a), *b); } }
    #[test]
    fn token_line_as_the_typescript() { for (a, h, b, c) in TOKEN { assert_eq!(token_line(a, Some(*h), Some(*b)), *c); } }
    #[test]
    fn the_page_line_as_the_typescript() { for (a, b) in LINE { assert_eq!(line_on_page(a, 5.0), *b); } }
    #[test]
    fn names_as_the_typescript() { for (a, b) in NAMES { assert_eq!(lesson_token(a), *b, "{}", a); } }
    #[test]
    fn faults_as_the_typescript() { for (w, l, c, r, al, want) in FAULTS { assert_eq!(lesson_token_faults(w, l, *c, *r, *al), want.to_vec()); } }
    /// lesson.ts:188's regular expression run under bun: None no match, Some(None) no target, Some(Some(x)) the group
    static WHATS: &[(&str, Option<Option<&str>>)] = &[
        ("act:reload", Some(None)), ("event:bang", Some(None)), ("chron:ko", Some(None)), ("order:stack", Some(None)), ("order:stack.up-1", Some(None)),
        ("order:take@foe:Big Mike", Some(Some("foe:Big Mike"))), ("order:take@foe:o'neil j.", Some(Some("foe:o'neil j."))), ("order:take@foe:a  b", None), ("order:take@foe:", None), ("order:take@foe: a", None),
        ("order:take@thing:breaker", Some(Some("thing:breaker"))), ("order:take@thing:objective", Some(Some("thing:objective"))), ("order:take@thing:device", Some(Some("thing:device"))), ("order:take@thing:door", None),
        ("act:x@foe:y", None), ("act:", None), ("foo:x", None), ("", None), ("act:a b", None), ("order:a@b@foe:x", None), ("order:@foe:x", None), ("order:a@foe:x:y", None), ("act:é", None), ("order:x\n", None), ("act:a:b", None), ("order:a@thing:breaker\n", None),
    ];
    #[test]
    fn what_as_the_typescript() { for (w, want) in WHATS { assert_eq!(what_target(w), *want, "{}", w); } }
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
        assert!(band_check(&LessonTest::Pace).is_none());
    }
    #[test]
    fn inside_is_open_at_its_edges() {
        let r = crate::scene_scenarios::Rect { x0: 0.0, x1: 1.0, z0: 0.0, z1: 1.0 };
        assert!(inside(r, [0.5, 9.0, 0.5]) && !inside(r, [0.0, 0.0, 0.5]) && !inside(r, [0.5, 0.0, 1.0]));
    }
}
