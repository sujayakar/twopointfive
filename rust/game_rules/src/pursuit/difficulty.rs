pub use ::game_types::difficulty::types;
pub use types::*;
use crate::game::Game;
use crate::js;
use alloc::{string::String, vec::Vec};

// ---------------------------------------------------------------- what the part's own tables hold

/// The levels, lowest first; the enum sorts its words, so never walk by the enum.
const DIFFICULTIES_ROWS: [Difficulty; 2] = [Difficulty::Standard, Difficulty::Hard];

/// the `Cone && '.' + Cone` of difficultyWords, HARD.cone being settled on.
const CONE_TAIL: &str = ". Under the fog of war each teammate sees only the cone he's looking down, so the floor behind everyone's back is dark, and they call out what you can't see";

// ---------------------------------------------------------------- the two rows outside the code

/// HARD_AT[structure]?.stuns: a row with no stuns reads as no row.
fn hard_at_stuns(g: &Game, structure: &str) -> Option<f64> {
    g.k.hard_at.rows.of(structure).copied().flatten()
}

/// storeName with what it needs: `store_name`'s signature has no g, and the store is reached through it.
fn store_name_in(level: Difficulty) {
    if level != Difficulty::Standard { crate::savestate::save_store_set(); }
}

/// JSON.stringify of a Partial<Levers>, in the order of these rows; the words need no escape. None when no row is present (Object.keys(S).length === 0).
fn levers_json(rows: &[(&str, Option<&str>)]) -> Option<String> {
    let parts: Vec<String> = rows.iter().filter_map(|(k, v)| v.map(|v| alloc::format!("\"{}\":\"{}\"", k, v))).collect();
    if parts.is_empty() { None } else { Some(alloc::format!("{{{}}}", parts.join(","))) }
}

// ---------------------------------------------------------------- JSON.parse, as far as keptLevers looks at what it gives

// ---------------------------------------------------------------- JSON.stringify(a) === JSON.stringify(b), for what the part compares

trait JsonSame { fn json_same(&self, o: &Self) -> bool; }
impl JsonSame for bool { fn json_same(&self, o: &bool) -> bool { *self == *o } }
/// a number's JSON: -0 is 0; NaN and both infinities are null.
impl JsonSame for f64 {
    fn json_same(&self, o: &f64) -> bool { if !self.is_finite() || !o.is_finite() { !self.is_finite() && !o.is_finite() } else { *self == *o } }
}
impl JsonSame for [f64; 2] { fn json_same(&self, o: &[f64; 2]) -> bool { self[0].json_same(&o[0]) && self[1].json_same(&o[1]) } }

/// every row by name; the pattern has no `..`, so a new row in the table stops the build here instead of being skipped.
macro_rules! rows_same {
    ($a:ident, $b:ident, $T:path; $($f:ident),* $(,)?) => {{
        let $T { $($f: _),* } = *$a;
        true $(&& $a.$f.json_same(&$b.$f))*
    }};
}

/// `same` of two PRESSURE tables.
fn pressure_same(a: &crate::knobs::PressureK, b: &crate::knobs::PressureK) -> bool {
    rows_same!(a, b, crate::knobs::PressureK; on, stuns, rescue_crew, drilled, flank_max_m, flank_k, flank_slack_m, last_man_secs, loud_secs, out_secs, sweep)
}

/// `same` of two DIGIN tables.
fn digin_same(a: &crate::knobs::DiginK, b: &crate::knobs::DiginK) -> bool {
    rows_same!(a, b, crate::knobs::DiginK; on, reach_m, noticed_secs, gather_m, gather_shy_m, think, wedge_secs, wedges, caught, try_secs, pace, close_m, prop_m, haul_speed, scrape_level, scrape_every, shout_m, alt_m, better, fear, hold_secs, redig_secs, lane_secs)
}

// ---------------------------------------------------------------- the levers' words

fn set_as_tables(s: DifficultyModSET) -> TablesOfSet { TablesOfSet { ai: s.ai, kit: s.kit } }

/// SET[key] as its word.
fn set_word(s: &DifficultyModSET, key: LeverKey) -> Option<&'static str> {
    match key {
        LeverKey::Fog => s.fog.map(|r| r.as_str()),
        LeverKey::Ai => s.ai.map(|r| r.as_str()),
        LeverKey::Kit => s.kit.map(|r| r.as_str()),
        LeverKey::Told => s.told.map(|r| r.as_str()),
    }
}

/// `delete SET[key]`.
fn set_clear(s: &mut DifficultyModSET, key: LeverKey) {
    match key {
        LeverKey::Fog => s.fog = None,
        LeverKey::Ai => s.ai = None,
        LeverKey::Kit => s.kit = None,
        LeverKey::Told => s.told = None,
    }
}

/// `SET[key] = rung`, the word already held to be a rung of that lever (isRung).
fn set_put(s: &mut DifficultyModSET, key: LeverKey, word: &str) {
    match (key, word) {
        (LeverKey::Fog, "team") => s.fog = Some(crate::quietfog::QuietRung::Team),
        (LeverKey::Fog, "cones") => s.fog = Some(crate::quietfog::QuietRung::Cones),
        (LeverKey::Fog, "own") => s.fog = Some(crate::quietfog::QuietRung::Own),
        (LeverKey::Fog, "off") => s.fog = Some(crate::quietfog::QuietRung::Off),
        (LeverKey::Ai, "standard") => s.ai = Some(Difficulty::Standard),
        (LeverKey::Ai, "hard") => s.ai = Some(Difficulty::Hard),
        (LeverKey::Kit, "none") => s.kit = Some(KitRung::None),
        (LeverKey::Kit, "stuns") => s.kit = Some(KitRung::Stuns),
        (LeverKey::Told, "guided") => s.told = Some(crate::told::ToldRung::Guided),
        (LeverKey::Told, "standard") => s.told = Some(crate::told::ToldRung::Standard),
        (LeverKey::Told, "bare") => s.told = Some(crate::told::ToldRung::Bare),
        _ => {}
    }
}

/// /^(hard|difficulty)(=|$)/.
fn is_level_key(k: &str) -> bool {
    k == "hard" || k == "difficulty" || k.starts_with("hard=") || k.starts_with("difficulty=")
}

/// `const [path, query = ''] = href.split('?')` and `query.split('&').filter(k => k)`.
fn href_parts(href: &str) -> (&str, Vec<&str>) {
    let mut at = href.split('?');
    let path = at.next().unwrap_or("");
    let query = at.next().unwrap_or("");
    (path, query.split('&').filter(|k| !k.is_empty()).collect())
}

// ---------------------------------------------------------------- the part

/// the structure a level file's room clear stands on.
pub fn structure_of_level(file: Option<&str>) -> String {
    let file = file.unwrap_or("");
    level_structure().of(file).cloned().unwrap_or_default()
}

/// LEVERS with the fog's bottom rung and the fourth lever; TOLD.on and TOLD.fogOff are settled on.
pub fn levers_now() -> Vec<crate::difficulty::LeverDef> {
    let mut out = levers();
    for l in out.iter_mut() {
        if l.key == LeverKey::Fog {
            let r = fog_off_rung();
            l.rungs.insert(0, LeverDefRungsItem { id: r.id, label: r.label, words: r.words });
        }
    }
    out.push(told_lever());
    out
}

/// the fog lever stands on its bottom rung.
pub fn fog_off_now(g: &crate::game::Game) -> bool {
    g.m.difficulty.set.fog == Some(crate::quietfog::QuietRung::Off)
}

/// a lever is moved off the level by hand.
fn any_set(g: &crate::game::Game) -> bool {
    let s = &g.m.difficulty.set;
    s.fog.is_some() || s.ai.is_some() || s.kit.is_some() || s.told.is_some()
}

/// v is a word and a rung of lever k.
pub fn is_rung(k: crate::difficulty::LeverKey, v: Option<&str>) -> bool {
    match v {
        Some(w) => levers_now().iter().find(|l| l.key == k).map_or(false, |l| l.rungs.iter().any(|r| r.id == w)),
        None => false,
    }
}

/// where a level puts every lever, on the structure the game stands on.
fn preset_levers(g: &crate::game::Game, level: crate::difficulty::Difficulty, structure: Option<&str>) -> crate::difficulty::Levers {
    let here;
    let structure = match structure {
        Some(s) => s,
        None => { here = g.m.difficulty.here.clone(); here.as_str() }
    };
    let (fog, ai, kit) = if level == Difficulty::Hard {
        let stuns = match hard_at_stuns(g, structure) { Some(n) => n, None => g.k.hard.stuns };
        (crate::quietfog::QuietRung::Cones, Difficulty::Hard, if stuns > 0.0 { KitRung::Stuns } else { KitRung::None })
    } else {
        (crate::quietfog::QuietRung::Team, Difficulty::Standard, KitRung::None)
    };
    Levers { fog, ai, kit, told: Some(crate::told::told_of_level(level)) }
}

/// the game stands on a new structure: tables standing at hard are written again for it.
pub fn difficulty_here(g: &mut crate::game::Game, structure: &str) {
    if structure == g.m.difficulty.here { return; }
    let was = difficulty_now(g);
    let levers = any_set(g) && tables_stand(g) && fog_stands(g);
    g.m.difficulty.here = String::from(structure);
    if any_set(g) {
        if levers { write(g, None); }
    } else if was == DifficultyNow::Hard {
        set_difficulty(g, Difficulty::Hard, Some(false));
    }
}

/// JSON.stringify(a) === JSON.stringify(b), of what this file compares.
fn same(a: &crate::difficulty::SameOf, b: &crate::difficulty::SameOf) -> bool {
    match (a, b) {
        (SameOf::Levers { shown: a }, SameOf::Levers { shown: b }) => a.fog == b.fog && a.ai == b.ai && a.kit == b.kit && a.told == b.told,
        (SameOf::Pressure { table: a }, SameOf::Pressure { table: b }) => pressure_same(a, b),
        (SameOf::Digin { table: a }, SameOf::Digin { table: b }) => digin_same(a, b),
        _ => false,
    }
}

/// the two tables a level and the levers over it write.
fn tables_of(g: &crate::game::Game, level: crate::difficulty::Difficulty, set: Option<crate::difficulty::TablesOfSet>) -> crate::difficulty::TablesOf {
    let set = set.unwrap_or(TablesOfSet { ai: None, kit: None });
    let mut p = crate::pressure::pressure_defaults();
    let mut d = crate::digin::digin_defaults();
    if set.ai.unwrap_or(level) == Difficulty::Hard {
        let h = g.k.hard;
        p.on = true;
        p.rescue_crew = true;
        p.drilled = true;
        p.sweep = true;
        d.on = true;
        d.reach_m = h.reach_m;
    }
    if level == Difficulty::Hard {
        p.stuns = g.k.hard.stuns;
        if let Some(n) = hard_at_stuns(g, &g.m.difficulty.here) { p.stuns = n; }
    }
    if let Some(kit) = set.kit { p.stuns = if kit == KitRung::Stuns { LEVER_STUNS } else { 0.0 }; }
    TablesOf { p, d }
}

/// the level the two tables stand at now.
pub fn difficulty_now(g: &crate::game::Game) -> crate::difficulty::DifficultyNow {
    if any_set(g) {
        let s = levers_shown(g, None);
        for lv in DIFFICULTIES_ROWS {
            let l = preset_levers(g, lv, None);
            let shown = LeversShown { fog: Some(l.fog), ai: Some(l.ai), kit: Some(l.kit), told: l.told };
            if same(&SameOf::Levers { shown }, &SameOf::Levers { shown: s }) {
                return if lv == Difficulty::Hard { DifficultyNow::Hard } else { DifficultyNow::Standard };
            }
        }
        return DifficultyNow::Custom;
    }
    if !g.k.pressure.on && !g.k.digin.on {
        return if g.k.pressure.stuns > 0.0 { DifficultyNow::Custom } else { DifficultyNow::Standard };
    }
    let t = tables_of(g, Difficulty::Hard, None);
    if pressure_same(&t.p, &g.k.pressure) && digin_same(&t.d, &g.k.digin) { DifficultyNow::Hard } else { DifficultyNow::Custom }
}

/// the levers as they stand, each read off its table; None = no rung.
pub fn levers_shown(g: &crate::game::Game, structure: Option<&str>) -> crate::difficulty::LeversShown {
    let h = tables_of(g, Difficulty::Hard, None);
    if let Some(structure) = structure {
        if structure != g.m.difficulty.here && g.m.difficulty.set.kit.is_none() && tables_stand(g) {
            let mut s = levers_shown(g, None);
            if s.kit.is_some() {
                let base = g.m.difficulty.base;
                s.kit = Some(preset_levers(g, base, Some(structure)).kit);
            }
            return s;
        }
    }
    let fog = if fog_off_now(g) { Some(crate::quietfog::QuietRung::Off) }
        else if g.k.fog.cone { Some(if g.k.fog.own { crate::quietfog::QuietRung::Own } else { crate::quietfog::QuietRung::Cones }) }
        else if g.k.fog.own { None }
        else { Some(crate::quietfog::QuietRung::Team) };
    let ai = if !g.k.pressure.on && !g.k.digin.on { Some(Difficulty::Standard) }
        else {
            let (mut hp, mut p) = (h.p, g.k.pressure);
            hp.stuns = 0.0;
            p.stuns = 0.0;
            if pressure_same(&hp, &p) && digin_same(&h.d, &g.k.digin) { Some(Difficulty::Hard) } else { None }
        };
    let stuns = g.k.pressure.stuns;
    let kit = if stuns == 0.0 { Some(KitRung::None) } else if stuns == LEVER_STUNS { Some(KitRung::Stuns) } else { None };
    LeversShown { fog, ai, kit, told: Some(crate::told::told_now(g)) }
}

/// the two tables still stand where the level and the levers last wrote them.
fn tables_stand(g: &crate::game::Game) -> bool {
    let (base, set) = (g.m.difficulty.base, set_as_tables(g.m.difficulty.set));
    let t = tables_of(g, base, Some(set));
    pressure_same(&t.p, &g.k.pressure) && digin_same(&t.d, &g.k.digin)
}

fn cone_of(rung: crate::quietfog::QuietRung) -> bool {
    rung == crate::quietfog::QuietRung::Cones || rung == crate::quietfog::QuietRung::Own
}

/// the level whose fog, AI and kit rungs the levers stand on as set.
fn level_of_three(g: &crate::game::Game) -> crate::difficulty::Difficulty {
    let (s, base) = (g.m.difficulty.set, g.m.difficulty.base);
    if s.fog.is_none() && s.ai.is_none() && s.kit.is_none() { return base; }
    let b = preset_levers(g, base, None);
    let mine = (s.fog.unwrap_or(b.fog), s.ai.unwrap_or(b.ai), s.kit.unwrap_or(b.kit));
    for lv in DIFFICULTIES_ROWS {
        let l = preset_levers(g, lv, None);
        if (l.fog, l.ai, l.kit) == mine { return lv; }
    }
    base
}

/// the fog's cone stands where the fog lever put it.
fn fog_stands(g: &crate::game::Game) -> bool {
    match g.m.difficulty.set.fog {
        None => true,
        Some(f) => g.k.fog.cone == g.m.difficulty.cone_visit.unwrap_or(cone_of(f)),
    }
}

/// every lever of the three with rows stands on a rung.
fn shown_whole(g: &crate::game::Game) -> bool {
    let s = levers_shown(g, None);
    s.fog.is_some() && s.ai.is_some() && s.kit.is_some()
}

/// levers are moved and every one stands on a rung, but on no level's.
pub fn levers_mixed(g: &crate::game::Game) -> bool {
    any_set(g) && shown_whole(g) && difficulty_now(g) == DifficultyNow::Custom
}

/// the tables written whole from the level and the levers over it, the fog's two rows with them.
fn write(g: &mut crate::game::Game, fog_only: Option<bool>) {
    if !fog_only.unwrap_or(false) {
        let (base, set) = (g.m.difficulty.base, set_as_tables(g.m.difficulty.set));
        let t = tables_of(g, base, Some(set));
        g.k.pressure = t.p;
        g.k.digin = t.d;
    }
    g.k.told.rung = match g.m.difficulty.set.told {
        Some(r) => r,
        None => { let lv = level_of_three(g); crate::told::told_of_level(lv) }
    };
    let (set_fog, base) = (g.m.difficulty.set.fog, g.m.difficulty.base);
    g.k.fog.cone = match g.m.difficulty.cone_visit {
        Some(v) => v,
        None => match set_fog { Some(f) => cone_of(f), None => base == Difficulty::Hard },
    };
    if let Some(f) = set_fog {
        if Some(f) != g.m.difficulty.own_for {
            if g.m.difficulty.own_was.is_none() { g.m.difficulty.own_was = Some(g.k.fog.own); }
            g.k.fog.own = f == crate::quietfog::QuietRung::Own;
            g.m.difficulty.own_for = Some(f);
        }
    } else if g.m.difficulty.own_for.is_some() {
        if let Some(was) = g.m.difficulty.own_was { g.k.fog.own = was; }
        g.m.difficulty.own_was = None;
        g.m.difficulty.own_for = None;
    }
}

/// the machine's store keeps the moved levers, with the kept rungs this visit hides unless `all`.
fn store_levers(g: &crate::game::Game) {
    let s = g.m.difficulty.set;
    let (fog, ai, kit, told) = (s.fog, s.ai, s.kit, s.told);
    if levers_json(&[("fog", fog.map(|r| r.as_str())), ("ai", ai.map(|r| r.as_str())), ("kit", kit.map(|r| r.as_str())), ("told", told.map(|r| r.as_str()))]).is_some() {
        crate::savestate::save_store_set();
    }
}

/// set the level: both tables written whole, every lever back on the level's own rung.
pub fn set_difficulty(g: &mut crate::game::Game, level: crate::difficulty::Difficulty, keep: Option<bool>) {
    let keep = keep.unwrap_or(true);
    g.m.difficulty.base = level;
    g.m.difficulty.set = DifficultyModSET { fog: None, ai: None, kit: None, told: None };
    if keep { g.m.difficulty.held = None; }
    if keep {
        crate::pressure::reset_pressure(g);
        crate::digin::reset_dig_in(g);
        store_levers(g);
        store_name_in(level);
    }
    write(g, None);
}

/// the level under the levers as the tables stand, read back off them if another road moved them.
fn adopt(g: &mut crate::game::Game) -> bool {
    if !fog_stands(g) { g.m.difficulty.set.fog = None; }
    if tables_stand(g) { return true; }
    let s = levers_shown(g, None);
    g.m.difficulty.set.ai = None;
    g.m.difficulty.set.kit = None;
    g.m.difficulty.base = s.ai.unwrap_or(Difficulty::Standard);
    if s.ai.is_some() {
        if let Some(kit) = s.kit {
            let base = g.m.difficulty.base;
            if kit != preset_levers(g, base, None).kit { g.m.difficulty.set.kit = Some(kit); }
        }
    }
    tables_stand(g)
}

/// a lever that writes nothing of the defenders' two tables.
fn own(key: crate::difficulty::LeverKey) -> bool {
    key == LeverKey::Fog || key == LeverKey::Told
}

/// move one lever; None = back to the level's own. Answers whether the level's name is kept anew with it.
pub fn set_lever(g: &mut crate::game::Game, key: crate::difficulty::LeverKey, rung: Option<&str>, keep: Option<bool>, structure: Option<&str>) -> bool {
    let keep = keep.unwrap_or(true);
    if rung.is_some() && !is_rung(key, rung) { return false; }
    if keep {
        if let Some(h) = g.m.difficulty.held {
            let s = &mut g.m.difficulty.set;
            if h.fog.is_some() && s.fog.is_none() { s.fog = h.fog; }
            if h.ai.is_some() && s.ai.is_none() { s.ai = h.ai; }
            if h.kit.is_some() && s.kit.is_none() { s.kit = h.kit; }
            if h.told.is_some() && s.told.is_none() { s.told = h.told; }
            write(g, Some(h.ai.is_none() && h.kit.is_none()));
            g.m.difficulty.held = None;
        }
    }
    let stood = adopt(g);
    let let_go = match (rung, structure) {
        (None, _) => true,
        (Some(_), None) => false,
        (Some(r), Some(s)) => {
            let at = if key == LeverKey::Told {
                let lv = level_of_three(g);
                crate::told::told_of_level(lv).as_str()
            } else {
                let base = g.m.difficulty.base;
                let l = preset_levers(g, base, Some(s));
                match key {
                    LeverKey::Fog => l.fog.as_str(),
                    LeverKey::Ai => l.ai.as_str(),
                    LeverKey::Kit => l.kit.as_str(),
                    LeverKey::Told => l.told.map_or("", |t| t.as_str()),
                }
            };
            r == at
        }
    };
    match rung {
        Some(r) if !let_go => set_put(&mut g.m.difficulty.set, key, r),
        _ => set_clear(&mut g.m.difficulty.set, key),
    }
    if keep {
        if !own(key) {
            crate::pressure::reset_pressure(g);
            crate::digin::reset_dig_in(g);
        }
        store_levers(g);
        if stood || !own(key) { let base = g.m.difficulty.base; store_name_in(base); }
    }
    write(g, Some(own(key)));
    keep && (stood || !own(key))
}

/// the fog lever's rung over the feel rows.
pub fn levers_fog(g: &mut crate::game::Game) {
    if g.m.difficulty.own_for.is_some() {
        if let Some(was) = g.m.difficulty.own_was { g.k.fog.own = was; }
    }
    if let Some(f) = g.m.difficulty.set.fog {
        g.m.difficulty.own_was = Some(g.k.fog.own);
        g.k.fog.own = f == crate::quietfog::QuietRung::Own;
        g.m.difficulty.own_for = Some(f);
    }
}

/// the cone for this visit, whatever the level; None = the level decides.
pub fn set_cone_visit(g: &mut crate::game::Game, v: Option<bool>) {
    g.m.difficulty.cone_visit = v;
    g.k.fog.cone = match v {
        Some(x) => x,
        None => match g.m.difficulty.set.fog {
            Some(f) => cone_of(f),
            None => if any_set(g) { g.m.difficulty.base == Difficulty::Hard } else { difficulty_now(g) == DifficultyNow::Hard },
        },
    };
}

/// the fog's cone as pinned for this visit.
pub fn cone_visit_now(g: &crate::game::Game) -> Option<bool> {
    g.m.difficulty.cone_visit
}

/// WHAT AN ADDRESS SAYS OF THE DIFFICULTY, and no more: None where it says nothing. `plain` (a Training lesson's address) reads no lever. Writes nothing.
pub fn difficulty_asked(q: &crate::query::Query, plain: bool) -> Option<crate::difficulty::DifficultyAsked> {
    let v = if q.has("hard") { Some(if q.get("hard") == Some("0") { "standard" } else { "hard" }) } else { q.get("difficulty") };
    let level = match v { Some("hard") => Some(Difficulty::Hard), Some("standard") => Some(Difficulty::Standard), _ => None };
    let mut levers = Vec::new();
    if !plain {
        for l in levers_now().iter() {
            if let Some(r) = q.get(&l.param) { if is_rung(l.key, Some(r)) { levers.push((l.key, String::from(r))); } }
        }
    }
    let digin = q.has("digin");
    let pressure = if q.has("pressure") { Some(q.get("pressure") != Some("0")) } else { None };
    let cone = if q.has("fogcone") { Some(q.get("fogcone") != Some("0")) } else { None };
    if level.is_none() && levers.is_empty() && !digin && pressure.is_none() && cone.is_none() { return None; }
    Some(DifficultyAsked { level, levers, digin, pressure, cone })
}

/// …AND IT LAID ON THE GAME'S TABLES, not stored: the level, the levers over it, the single-switch flags over whatever those wrote. (What a machine keeps
/// goes under it: game_start `host::lay`.)
pub fn difficulty_lay(g: &mut crate::game::Game, a: &crate::difficulty::DifficultyAsked) {
    if let Some(level) = a.level { set_difficulty(g, level, Some(false)); }
    for (key, rung) in a.levers.iter() { set_lever(g, *key, Some(rung.as_str()), Some(false), None); }
    if a.digin { g.k.digin.on = true; }
    if let Some(on) = a.pressure { g.k.pressure.on = on; }
    if a.cone.is_some() { set_cone_visit(g, a.cone); }
}

/// the rung one lever stands moved to by hand.
pub fn lever_set(g: &crate::game::Game, key: crate::difficulty::LeverKey) -> Option<&'static str> { set_word(&g.m.difficulty.set, key) }

/// the rung of one lever in a line ('' = it stands on none).
pub fn lever_words(key: crate::difficulty::LeverKey, rung: Option<&str>) -> String {
    let Some(rung) = rung else { return String::new() };
    levers_now().into_iter().find(|l| l.key == key).and_then(|l| l.rungs.into_iter().find(|r| r.id == rung)).map_or(String::new(), |r| r.words)
}

/// the text this machine keeps of the levers moved, in the order they were moved ('' = it keeps none).
pub fn levers_text(g: &crate::game::Game, order: &[crate::difficulty::LeverKey]) -> String {
    let rows: Vec<(&str, Option<&str>)> = order.iter().map(|k| (k.as_str(), lever_set(g, *k))).collect();
    levers_json(&rows).unwrap_or_default()
}

/// the text this machine keeps of the level ('' = standard, which keeps nothing).
pub fn level_text(g: &crate::game::Game) -> String {
    let base = g.m.difficulty.base;
    if base == Difficulty::Standard { String::new() } else { String::from(base.as_str()) }
}

/// how a row's Hard is worded: the row's own fact, or squads cut from the rooms of a room clear whose file names none.
pub fn squads_held(e: &crate::scene_scenarios::ScenarioEntry) -> crate::difficulty::DifficultyWordsSquads {
    use crate::difficulty::DifficultyWordsSquads as S;
    if e.squads == Some(true) { return e.level.clone().filter(|l| !l.is_empty()).map_or(S::Bool(true), S::Text); }
    S::Bool(e.kind == crate::scene_scenarios::MissionKind::Roomclear)
}

/// The same, by whoever knows whether this machine keeps the levers as they stand.
pub fn difficulty_href_kept(g: &crate::game::Game, href: &str, kept: bool) -> String {
    let now = difficulty_now(g);
    let (path, all) = href_parts(href);
    let keys: Vec<&str> = all.iter().copied().filter(|k| !is_level_key(k)).collect();
    if any_set(g) {
        if !shown_whole(g) { return String::from(href); }
        let mut q: Vec<String> = keys.iter().map(|k| String::from(*k)).collect();
        q.push(alloc::format!("difficulty={}", g.m.difficulty.base.as_str()));
        return levers_href_kept(g, &alloc::format!("{}?{}", path, q.join("&")), kept);
    }
    if now == DifficultyNow::Custom { return String::from(href); }
    if now == DifficultyNow::Standard && keys.len() == all.len() { return levers_href_kept(g, href, kept); }
    let mut q: Vec<String> = keys.iter().map(|k| String::from(*k)).collect();
    q.push(alloc::format!("difficulty={}", now.as_str()));
    levers_href_kept(g, &alloc::format!("{}?{}", path, q.join("&")), kept)
}

/// The same, by whoever knows whether this machine keeps the levers as they stand.
pub fn levers_href_kept(g: &crate::game::Game, href: &str, kept: bool) -> String {
    if any_set(g) && !shown_whole(g) { return String::from(href); }
    let (path, all) = href_parts(href);
    let now = levers_now();
    let keys: Vec<&str> = all.iter().copied().filter(|k| {
        let name = k.split('=').next().unwrap_or("");
        !now.iter().any(|l| name == l.param)
    }).collect();
    if !any_set(g) && keys.len() == all.len() { return String::from(href); }
    let mut q: Vec<String> = keys.iter().map(|k| String::from(*k)).collect();
    if any_set(g) && !kept {
        let s = g.m.difficulty.set;
        for l in now.iter() {
            if let Some(w) = set_word(&s, l.key) { q.push(alloc::format!("{}={}", l.param, w)); }
        }
    }
    let q = q.join("&");
    if q.is_empty() { String::from(path) } else { alloc::format!("{}?{}", path, q) }
}

/// what a level is, in a sentence.
pub fn difficulty_words(g: &crate::game::Game, level: Option<crate::difficulty::DifficultyNow>, squads: Option<&crate::difficulty::DifficultyWordsSquads>) -> String {
    let level = match level {
        Some(l) => l,
        None => difficulty_now(g),
    };
    if level == crate::difficulty::DifficultyNow::Custom && levers_mixed(g) {
        return alloc::format!("your own mix: {}", levers_line(g, Some(true)));
    }
    // (squads: a word, or true = one not said which; '' and false = not one)
    let squad_word: Option<Option<&str>> = match squads {
        Some(DifficultyWordsSquads::Text(s)) if !s.is_empty() => Some(Some(s.as_str())),
        Some(DifficultyWordsSquads::Bool(true)) => Some(None),
        _ => None,
    };
    if let (Some(word), crate::difficulty::DifficultyNow::Hard) = (squad_word, level) {
        let w = word.and_then(|sq| squad_hard().into_iter().find(|x| x.file == sq || x.structure == sq).map(|x| x.words));
        let at = match word {
            Some(sq) => { let s = structure_of_level(Some(sq)); if s.is_empty() { String::from(sq) } else { s } }
            None => g.m.difficulty.here.clone(),
        };
        let kit = if hard_at_stuns(g, &at).map_or(false, js::truthy) { ". Here each defender carries a stun charge, and he'll throw it down a hallway he knows you're holding" } else { "" };
        return match w.as_deref() {
            Some("") => alloc::format!("hard: the squads here are the standard ones, because this floor is hard enough as it is{}{}", kit, CONE_TAIL),
            Some(w) => alloc::format!("hard: the squads are a drilled garrison. {}{}{}", w, kit, CONE_TAIL),
            None => alloc::format!("hard: the squads are a drilled garrison{}{}", kit, CONE_TAIL),
        };
    }
    match level {
        crate::difficulty::DifficultyNow::Standard => String::from("standard: the usual defenders"),
        crate::difficulty::DifficultyNow::Hard => {
            let here = g.m.difficulty.here.clone();
            let kit = if hard_at_stuns(g, &here).map_or(false, js::truthy) { ". Here each defender carries a stun charge, and he'll throw it through a door before he comes in" } else { "" };
            alloc::format!("hard: the defenders come at you together and around through a second door, a far room's defenders come as a group, and they stand and fight where they meet you in the open. If they hear you before your first entry, they'll get ready. The last defender in a room will slip out through another door before you reach his door. At the records office, the defenders who come to a lost room will watch its door from out of a flashbang's reach and let you come out to them. On a hostage rescue the captor calls your assault on the radio and his guards act on it. On the night breach the defenders are drilled, half as shaken by a flashbang, and they'll fire blind through the door they were watching{}{}", kit, CONE_TAIL)
        }
        crate::difficulty::DifficultyNow::Custom => String::from("custom: rows under the switch were moved by hand (the Enemy AI page of the debugger)"),
    }
}

/// the levers in a line; `all` false: only the ones moved by hand.
fn levers_line(g: &crate::game::Game, all: Option<bool>) -> String {
    let all = all.unwrap_or(false);
    let s = levers_shown(g, None);
    let set = g.m.difficulty.set;
    let now = levers_now();
    let mut parts: Vec<String> = Vec::new();
    for l in now.iter() {
        if !(all || set_word(&set, l.key).is_some()) { continue; }
        let word = match l.key {
            LeverKey::Fog => s.fog.map(|r| r.as_str()),
            LeverKey::Ai => s.ai.map(|r| r.as_str()),
            LeverKey::Kit => s.kit.map(|r| r.as_str()),
            LeverKey::Told => s.told.map(|r| r.as_str()),
        };
        let shown = l.rungs.iter().find(|r| Some(r.id.as_str()) == word).map_or("custom", |r| r.label.as_str());
        let mut head = l.label.chars();
        let first = head.next().map_or(String::new(), |c| c.to_lowercase().collect::<String>());
        parts.push(alloc::format!("{}{}: {}", first, head.as_str(), shown));
    }
    parts.join(" · ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rows_compare_as_their_json_does() {
        let a = crate::knobs::PressureK::page();
        let mut b = a;
        assert!(pressure_same(&a, &b));
        b.flank_k = -0.0;
        let mut c = a;
        c.flank_k = 0.0;
        assert!(pressure_same(&b, &c));   // -0 is 0
        c.flank_k = f64::NAN;
        b.flank_k = f64::INFINITY;
        assert!(pressure_same(&b, &c));   // both null
        b.flank_k = 1.0;
        assert!(!pressure_same(&b, &c));
        let mut d = a;
        d.last_man_secs[1] += 1.0;
        assert!(!pressure_same(&a, &d));
        let e = crate::knobs::DiginK::page();
        let mut f = e;
        assert!(digin_same(&e, &f));
        f.on = !f.on;
        assert!(!digin_same(&e, &f));
    }

    #[test]
    fn a_links_level_keys() {
        for k in ["hard", "hard=0", "difficulty", "difficulty=hard"] { assert!(is_level_key(k), "{}", k); }
        for k in ["hardly", "difficulty2", "x=hard", ""] { assert!(!is_level_key(k), "{}", k); }
        let (p, q) = href_parts("/a?b=1&&hard=0?dropped");
        assert_eq!(p, "/a");
        assert_eq!(q, ["b=1", "hard=0"]);
        let (p, q) = href_parts("/a");
        assert_eq!(p, "/a");
        assert!(q.is_empty());
    }

    #[test]
    fn same_is_false_across_arms_and_reads_each_arm() {
        let (p, d) = (crate::knobs::PressureK::page(), crate::knobs::DiginK::page());
        let pa = SameOf::Pressure { table: p };
        assert!(same(&pa, &SameOf::Pressure { table: p }));
        assert!(!same(&pa, &SameOf::Digin { table: d }));
        let mut q = p;
        q.on = !q.on;
        assert!(!same(&pa, &SameOf::Pressure { table: q }));
        assert!(same(&SameOf::Digin { table: d }, &SameOf::Digin { table: d }));
        let shown = LeversShown { fog: Some(crate::quietfog::QuietRung::Team), ai: Some(Difficulty::Standard), kit: Some(KitRung::None), told: Some(crate::told::ToldRung::Standard) };
        let mut other = shown;
        assert!(same(&SameOf::Levers { shown }, &SameOf::Levers { shown: other }));
        other.fog = None;   // (null is no rung, and is not 'team')
        assert!(!same(&SameOf::Levers { shown }, &SameOf::Levers { shown: other }));
    }

    #[test]
    fn levers_now_is_the_tables_with_the_bottom_rung_and_the_fourth() {
        let now = levers_now();
        assert_eq!(now.iter().map(|l| l.key.as_str()).collect::<Vec<_>>(), ["fog", "ai", "kit", "told"]);
        assert_eq!(now[0].rungs.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(), ["off", "team", "cones", "own"]);
        for l in now.iter() {
            for r in &l.rungs {
                assert!(is_rung(l.key, Some(r.id.as_str())));
                let mut s = DifficultyModSET::default();
                set_put(&mut s, l.key, &r.id);
                assert_eq!(set_word(&s, l.key), Some(r.id.as_str()));
            }
        }
        assert!(!is_rung(LeverKey::Kit, Some("cones")));
        assert!(!is_rung(LeverKey::Fog, None));
    }

    #[test]
    fn a_level_files_structure() {
        assert_eq!(structure_of_level(Some("block.json")), "the block");
        assert_eq!(structure_of_level(Some("office.json")), "");
        assert_eq!(structure_of_level(None), "");
    }

    #[test]
    fn hard_at_reads_its_rows() {
        let mut g = crate::game::Game::blank();
        assert_eq!(hard_at_stuns(&g, "the block"), Some(1.0));
        assert_eq!(hard_at_stuns(&g, ""), None);
        g.k.hard_at.rows.set(String::from("a floor"), None);   // (a row with no stuns reads as no row)
        assert_eq!(hard_at_stuns(&g, "a floor"), None);
    }

    #[test]
    fn what_the_store_is_given() {
        assert_eq!(levers_json(&[("fog", None), ("ai", None)]), None);
        assert_eq!(levers_json(&[("fog", Some("cones")), ("told", None), ("kit", Some("stuns"))]).as_deref(), Some("{\"fog\":\"cones\",\"kit\":\"stuns\"}"));
    }

    #[test]
    fn set_put_takes_every_rung_of_the_tables() {
        for l in levers().iter().chain([told_lever()].iter()) {
            for r in &l.rungs {
                let mut s = DifficultyModSET { fog: None, ai: None, kit: None, told: None };
                set_put(&mut s, l.key, &r.id);
                assert_eq!(set_word(&s, l.key), Some(r.id.as_str()));
                set_clear(&mut s, l.key);
                assert_eq!(set_word(&s, l.key), None);
            }
        }
        let mut s = DifficultyModSET { fog: None, ai: None, kit: None, told: None };
        set_put(&mut s, LeverKey::Fog, fog_off_rung().id.as_str());
        assert_eq!(s.fog, Some(crate::quietfog::QuietRung::Off));
    }
}
