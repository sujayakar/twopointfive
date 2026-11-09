#![allow(unused_imports, unused_variables)]
//! the verdict's tick. How a run went, worked out by the game itself and LATCHED: the first end wins and stays for that run.
//! It only reads the game outside `g.verdict`: it moves nobody and draws no die, so a play is the same play with or without it.

pub use ::game_types::verdict::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{format, string::String, vec::Vec};

/// The file a mission row names for the level this game stands on (None: the bundled office). Read back off the level's own name.
fn row_file(g: &crate::game::Game) -> Option<String> {
    let name = crate::scene_level_load::level_name_of(g)?;
    if name.is_empty() || name == "office" { return None; }
    Some(alloc::format!("{}.json", name))
}

/// The mission that is up is the stealth game's kind on a row that leans loud: nothing grades that run as a sneak. VERDICT.loudCard is settled ON on the page.
pub fn loud_row(g: &crate::game::Game) -> bool {
    g.staged.facts.unseen && crate::scenario_run::running_row(g, row_file(g).as_deref()).and_then(|e| e.lean) == Some(crate::ui_missions::Lean::Loud)
}

///  a man's walk is at most this many samples, one every half second until it is halved; a hole counts as walked through this near
const PATH_CAP: usize = 1500;
const PATH_SECS: f64 = 0.5;
const HOLE_M: f64 = 0.9;

/// A run a verdict can be given on: somebody plays it.
fn live(g: &Game) -> bool { !g.aside }
/// The try has started: the first order, stride or round has been (or the run was loaded from a checkpoint).
fn started(g: &Game) -> bool { g.verdict.t0.is_finite() }
/// Seconds since the try started.
fn secs(g: &Game) -> f64 { if started(g) { m::max(0.0, g.time - g.verdict.t0) } else { 0.0 } }
fn fired(g: &Game) -> f64 { g.mission.stats.shots + g.chronicle.counts.thrown }

/// The last order anybody gave, off the books' logs, by its row's serial (None: none yet).
fn last_order(g: &Game) -> Option<u32> {
    if crate::seats::one_book(g) { return crate::seats::here(g).orders.log.last().map(|e| e.serial); }
    let mut best: Option<(f64, u32)> = None;
    for &s in g.seats.iter() {
        if let Some(e) = crate::seats::seat(g, s).orders.log.last() { if best.map_or(true, |b| e.t > b.0) { best = Some((e.t, e.serial)); } }
    }
    best.map(|b| b.1)
}

/// Is the team in more than one pair.
pub fn team_paired(g: &Game, seat: &crate::seats::SeatRef) -> bool {
    if crate::seats::one_book(g) { return crate::orders::squad_orders_paired(g, seat); }
    let (mut red, mut blue) = (false, false);
    for &gd in g.guards.iter() {
        if g.gd[gd].team != crate::teams::Team::Operative { continue; }
        match g.gd[gd].unit.as_ref().and_then(|u| u.pair) { Some(crate::game::PairId::Red) => red = true, Some(crate::game::PairId::Blue) => blue = true, None => {} }
    }
    red && blue
}

/// Where the room clear stages its team's first man (None: no room clear is up, or its structure names none).
fn start_at(g: &Game) -> Option<V3> {
    if !crate::roomclear::room_clear_scenario_active(g) { return None; }
    let s = crate::roomclear::room_clear_scenario_structure(g)?;
    let way = g.staged.way;
    if way > 0.0 { if let Some(t) = s.starts.as_ref().and_then(|v| v.get(way as usize - 1)) { return Some(t.sam); } }
    s.team.as_ref().map(|t| t.sam)
}

/// A new run: everything this module holds of the last one goes.
fn fresh(g: &mut Game, tail: Option<u32>) {
    let men = crate::seats::men(g);
    let origins: Vec<V3> = men.iter().map(|&pl| g.ch[g.pl[pl].ch].pos).collect();
    let p0 = origins[0];
    let p = start_at(g).unwrap_or(p0);
    let (wall_serial, fired0, resumed) = (g.destr.walls.serial, fired(g), g.chronicle.counts.loads > 0.0);
    // a checkpoint load is a fresh mission carrying the old run's chronicle: its clock the mission's own
    let t0 = if resumed { g.time - g.mission.stats.elapsed } else { f64::NAN };
    let run = Some(g.mission.serial);
    let v = &mut g.verdict;
    v.run = run; v.res = None; v.card_at = -1.0; v.kept = None; v.cheat = false;
    v.found_at = -1.0; v.dark_at = -1.0; v.dark_when_found = false; v.through = false; v.holes = Vec::new();
    v.wall_serial = wall_serial;
    v.tracks.clear(); v.foes.clear(); v.fell.clear(); v.fell_at = Vec::new();
    v.origins = origins; v.origin = p; v.order0 = tail; v.fired0 = fired0;
    v.resumed = resumed; v.t0 = t0;
}

fn track(id: f64, name: String, tone: Option<crate::host_findme::OutlineHue>, human: bool, gd: Option<GuardId>) -> Track {
    Track { id, name, tone, human, gd, state: ManState::Up, down_at: -1.0, down_room: String::new(), down_by: String::new(), down_pos: None, drops: Vec::new(), path: Vec::new(), every: PATH_SECS, next: 0.0 }
}

/// The humans of the team, then its AI men: a track each, made the first tick he is seen.
fn roster(g: &mut Game) {
    use crate::host_findme::OutlineHue as Hue;
    for pl in crate::seats::men(g) {
        let id = g.ch[g.pl[pl].ch].id;
        if g.verdict.tracks.has(crate::Num(id)) { continue; }
        let seat = crate::seats::seat_of_man(g, pl);
        // (seats.ts SEAT_TONES: 'you', 'purple', by the seat's place)
        let tone = if (g.tab.cmd_seat[seat].index as usize) % 2 == 0 { Hue::You } else { Hue::Purple };
        let name = crate::seats::player_name(g, pl);
        g.verdict.tracks.set(crate::Num(id), track(id, name, Some(tone), true, None));
    }
    let paired = team_paired(g, &crate::seats::here(g));
    for gd in g.staged.team.clone() {
        let id = g.ch[g.gd[gd].ch].id;
        if g.verdict.tracks.has(crate::Num(id)) { continue; }
        let tone = if paired { match g.gd[gd].unit.as_ref().and_then(|u| u.pair) { Some(crate::game::PairId::Red) => Some(Hue::Red), Some(crate::game::PairId::Blue) => Some(Hue::Blue), None => None } } else { None };
        let name = g.gd[gd].callsign.clone();
        g.verdict.tracks.set(crate::Num(id), track(id, name, tone, false, Some(gd)));
    }
}

/// A man put down by hand with no name on it is the partner's where the team has ONE AI man; with more it is nobody's.
fn only_mate(g: &Game, h: GuardId) -> Option<crate::Num> {
    let x = &g.gd[h];
    if x.killer_id >= 0.0 || x.down_by != Some(crate::game::DownBy::Guard) || x.down_kind == Some(crate::game::DownKind::Shot) { return None; }
    let (mut one, mut n) = (None, 0);
    for (k, t) in g.verdict.tracks.iter() { if !t.human { one = Some(k); n += 1; } }
    if n == 1 { one } else { None }
}

/// This tick read: who is where, who fell, who dropped whom, whether the team has been found, the mains, a hole walked through.
fn watch(g: &mut Game) {
    let now = secs(g);
    // the defenders: every man hostile to the player this run has met
    for i in 0..g.guards.len() {
        let h = g.guards[i];
        if !g.verdict.foes.has(h) && g.gd[h].team != crate::teams::Team::Operative && crate::teams::hostile_to(g, h, crate::teams::Team::Player) { g.verdict.foes.add(h); }
    }
    let mut hunting = false;
    for h in g.verdict.foes.keys() {
        let state = g.gd[h].state;
        if state == crate::game::GuardState::Dead {
            if !g.verdict.fell.has(h) {
                g.verdict.fell.add(h);
                let p = g.ch[g.gd[h].ch].pos;
                g.verdict.fell_at.push(p[0]); g.verdict.fell_at.push(p[2]);
                let killer = crate::Num(g.gd[h].killer_id);
                let by = if g.verdict.tracks.has(killer) { Some(killer) } else {
                    let sam = crate::Num(g.ch[g.pl[g.player].ch].id);
                    if g.gd[h].killer_id < 0.0 && g.gd[h].down_by == Some(crate::game::DownBy::Player) && g.verdict.tracks.has(sam) { Some(sam) } else { only_mate(g, h) }
                };
                if let Some(k) = by { if let Some(t) = g.verdict.tracks.get_mut(k) { t.drops.push(now); } }
            }
        } else if state == crate::game::GuardState::Alert || state == crate::game::GuardState::Search { hunting = true; }
    }
    if g.mission.stats.blackout && g.verdict.dark_at < 0.0 { g.verdict.dark_at = now; }
    if hunting && g.verdict.found_at < 0.0 { g.verdict.found_at = now; g.verdict.dark_when_found = g.verdict.dark_at >= 0.0 && g.light_home.mains.active; }
    // holes a blast opened in walls since the run began
    let serial = g.destr.walls.serial;
    if serial != g.verdict.wall_serial {
        for c in crate::wallbreak::wall_break_since(g, g.verdict.wall_serial).unwrap_or_default() {
            if c.kind == crate::wallbreak::LevelChangeKind::Reset { g.verdict.holes = Vec::new(); }
            else if c.kind == crate::wallbreak::LevelChangeKind::Hole && c.cause == crate::wallbreak::LevelChangeCause::Blast && c.walkable { g.verdict.holes.push(c.pos); }
        }
        g.verdict.wall_serial = serial;
    }
    // the men
    let men = crate::seats::men(g);
    for key in g.verdict.tracks.keys() {
        let (id, human, gd, up) = { let t = g.verdict.tracks.get(key).unwrap(); (t.id, t.human, t.gd, t.state == ManState::Up) };
        if !up { continue; }
        let pl = if human { men.iter().copied().find(|&x| g.ch[g.pl[x].ch].id == id) } else { None };
        let c = match (pl, gd) { (Some(pl), _) => g.pl[pl].ch, (None, Some(gd)) => g.gd[gd].ch, (None, None) => continue };
        let pos = g.ch[c].pos;
        let p = g.ch[c].bones.get(crate::character::Bone::Hips).copied().unwrap_or(pos);
        let down = match pl { Some(pl) => crate::condition::down(g, g.pl[pl].ch), None => crate::condition::down(g, g.gd[gd.unwrap()].ch) };
        if down {
            let state = match pl {
                Some(_) => ManState::Dead,
                None => { let x = &g.gd[gd.unwrap()]; if x.down_kind.is_some() && x.down_kind != Some(crate::game::DownKind::Shot) { ManState::Out } else { ManState::Dead } }
            };
            let room = crate::chronicle::room_of(g, p);
            let by = match pl {
                Some(_) => g.chronicle.counts.down_by.clone(),
                None => { let k = g.gd[gd.unwrap()].killer_id; g.guards.iter().copied().find(|&x| g.ch[g.gd[x].ch].id == k).map(|x| g.gd[x].callsign.clone()).unwrap_or_default() }
            };
            let t = g.verdict.tracks.get_mut(key).unwrap();
            t.state = state; t.down_at = now; t.down_pos = Some([p[0], p[2]]); t.down_room = room; t.down_by = by;
            t.path.push(p[0]); t.path.push(p[2]); t.path.push(now);
            continue;
        }
        let through = g.verdict.through || g.verdict.holes.iter().any(|h| ::sim::maths::hypot2(pos[0] - h[0], pos[2] - h[2]) < HOLE_M);
        let t = g.verdict.tracks.get_mut(key).unwrap();
        if now >= t.next || t.path.is_empty() {
            t.path.push(pos[0]); t.path.push(pos[2]); t.path.push(now); t.next = now + t.every;
            if t.path.len() >= PATH_CAP * 3 { let mut half = Vec::with_capacity(t.path.len() / 2 + 3); let mut i = 0; while i < t.path.len() { half.push(t.path[i]); half.push(t.path[i + 1]); half.push(t.path[i + 2]); i += 6; } t.path = half; t.every *= 2.0; }
        }
        g.verdict.through = through;
    }
}

/// The way a won run was won: quiet, dark, walls, doors, in that order.
fn approach(g: &Game) -> Approach {
    let (s, n) = (&g.mission.stats, &g.chronicle.counts);
    // a loaded run lost this module's own latches: for the kinds that can be loaded the chronicle says the same, and for the stealth game the
    // mission's own books decide with it, loaded or not
    let stealth = !g.staged.facts.fight;
    let loaded = g.verdict.resumed && stealth;
    let found = g.verdict.found_at >= 0.0 || (stealth && (s.alerts > 0.0 || s.spotted > 0.0 || n.esc1 + n.esc2 > 0.0));
    if !found { return Approach::Quiet; }
    if g.verdict.dark_when_found || (loaded && g.verdict.found_at < 0.0 && s.blackout) { return Approach::Dark; }
    if g.verdict.through { Approach::Walls } else { Approach::Doors }
}

/// The run is over: its one verdict is made, and stays.
fn close(g: &mut Game, end: (bool, &'static str, String)) {
    let secs = secs(g);
    let men: Vec<VerdictMan> = g.verdict.tracks.values().map(|t| VerdictMan { id: t.id, name: t.name.clone(), tone: t.tone, human: t.human, state: t.state, down_at: t.down_at, down_room: t.down_room.clone(), down_by: t.down_by.clone(), down_pos: t.down_pos, drops: t.drops.clone(), path: t.path.clone() }).collect();
    let lost = men.iter().filter(|x| x.state != ManState::Up).count() as f64;
    let foes_down = g.verdict.foes.iter().filter(|&h| crate::condition::down(g, g.gd[h].ch)).count() as f64;
    let hostages = g.staged.saved.map(|alive| RunVerdictHostages { alive, of: g.staged.captives.len() as f64 });
    let (won, word, why) = end;
    let res = RunVerdict {
        kind: g.staged.up,
        won, word: String::from(word), why, secs,
        approach: if won { Some(approach(g)) } else { None },
        void: g.verdict.cheat || (g.verdict.resumed && g.mission.stats.sandbox),
        clean: lost == 0.0, of: men.len() as f64, men, lost,
        foes: RunVerdictFoes { down: foes_down, of: g.verdict.foes.keys().len() as f64, fell: g.verdict.fell_at.clone() },
        hostages, start: [g.verdict.origin[0], g.verdict.origin[2]],
    };
    // the stealth mission raises its own card (the exfil, a player down); every other end is this module's to announce
    let stage = g.mission.stage;
    let all_down = crate::seats::all_down(g);
    let own = stage == crate::game::MissionStage::Done || (stage == crate::game::MissionStage::Failed && g.mission.down_card_at > 0.0) || (stage != crate::game::MissionStage::Failed && all_down);
    g.verdict.card_at = if own { -1.0 } else { g.time + if won { g.k.verdict.win_secs } else { g.k.verdict.loss_secs } };
    g.verdict.res = Some(res);
}

/// THE VERDICT'S TICK, after the scenarios, and the mission have had this tick, so every end they wrote is in. The first
/// end of a run is latched in `g.verdict.res` and nothing after it changes it; a new mission is a new run.
pub fn verdict_tick(g: &mut crate::game::Game) {
    if !live(g) { return; }
    let tail = last_order(g);
    if g.verdict.run != Some(g.mission.serial) { fresh(g, tail); }
    if g.verdict.res.is_some() { return; }
    roster(g);
    if !started(g) {
        // any player off his own start, a round or a throw, or an order
        let men = crate::seats::men(g);
        let mut moved = false;
        for (i, &pl) in men.iter().enumerate() {
            let p = g.ch[g.pl[pl].ch].pos;
            let o = if i > 0 { g.verdict.origins.get(i).copied().unwrap_or(p) } else { g.verdict.origin };
            if ::sim::maths::hypot2(p[0] - o[0], p[2] - o[2]) > crate::par::PAR_START_M { moved = true; break; }
        }
        if moved || fired(g) != g.verdict.fired0 || tail != g.verdict.order0 { g.verdict.t0 = g.time; }
    }
    // (rails.ts: the seat with nobody on the floor parks him unseen: the mission's own staging, not a sandbox tick)
    if started(g) && (g.god_mode || !g.ai_enabled || g.infinite_ammo || g.player_invisible) { g.verdict.cheat = true; }
    watch(g);
    if let Some(end) = crate::mission::ended(g) { close(g, end); }
}

/// HOW THE MISSION ENDED, read off the latched record: None while it is being played (and for a run that is not this mission any more), else whether
/// it was won, the end's word and why. A read; while the answer is None it allocates nothing.
pub fn verdict_ended(g: &crate::game::Game) -> Option<(bool, &str, &str)> {
    if g.verdict.run != Some(g.mission.serial) { return None; }
    g.verdict.res.as_ref().map(|r| (r.won, r.word.as_str(), r.why.as_str()))
}

/// The latched record as lines, for a host with no picture and for holding the record to another's: the word, the why, then a line each of the
/// seconds, the way won, lost and of, the defenders down and of, the hostages alive and of, void, clean, and the men (`id state downAt downBy
/// drops walked`, where walked is the count of his path's numbers). None while the mission is being played.
pub fn verdict_lines(g: &crate::game::Game) -> Option<(bool, String)> {
    if g.verdict.run != Some(g.mission.serial) { return None; }
    let r = g.verdict.res.as_ref()?;
    let men: Vec<String> = r.men.iter().map(|x| format!("{} {} {} {} {} {}", x.id, x.state.as_str(), x.down_at, if x.down_by.is_empty() { "-" } else { x.down_by.as_str() }, x.drops.len(), x.path.len())).collect();
    Some((r.won, format!("{}\n{}\n{}\n{}\n{} {}\n{} {}\n{}\n{}\n{}\n{}", r.word, r.why, r.secs, r.approach.map_or("-", |a| a.as_str()), r.lost, r.of, r.foes.down, r.foes.of,
        r.hostages.as_ref().map_or(String::from("-"), |h| format!("{} {}", h.alive, h.of)), r.void, r.clean, men.join(";"))))
}

/// Has this run's try begun, and is it over: a run a machine may keep.
pub fn run_on(g: &crate::game::Game) -> (bool, bool) {
    if !live(g) || g.verdict.run != Some(g.mission.serial) { return (false, false); }
    (started(g), g.verdict.res.is_some())
}

/// What a machine keeps of this run. A read: asked as the try begins and as the run ends.
pub fn run_view(g: &crate::game::Game) -> crate::view::RunView {
    let (begun, over) = run_on(g);
    let file = row_file(g);
    let entry = crate::scenario_run::running_row(g, file.as_deref());
    let level = String::from(crate::difficulty::difficulty_now(g).as_str());
    let res = if over { g.verdict.res.as_ref() } else { None };
    let name = file.as_deref().unwrap_or("office");
    crate::view::RunView {
        begun,
        over,
        void: res.map_or(g.verdict.cheat, |r| r.void),
        lesson: entry.as_ref().map_or(false, |e| e.group == Some(crate::scene_scenarios::MissionGroup::Tutorial)),
        mission: match (&entry, &file) { (Some(e), _) => e.id.clone(), (None, Some(f)) => format!("night:{}", f), (None, None) => String::from(crate::scene_scenario_list::SOLO_ID) },
        won: res.map_or(false, |r| r.won),
        way: res.and_then(|r| r.approach),
        clean: res.map_or(false, |r| r.clean),
        secs: res.map_or(0.0, |r| r.secs),
        course: format!("{}|way{}|squad{}|{}", name, g.staged.way + 1.0, g.staged.team.len(), level),
        level,
        par: crate::par::par_result_now(g).map(|r| r.shown),
    }
}
