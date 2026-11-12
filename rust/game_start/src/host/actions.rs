//! THE PANEL'S ACTIONS: what a person does to the game from the debugger that isn't a number, as one list. The page draws a button, a box or a choice from a row and knows none by name; a press is a
//! command (`Act`), so a tape carries it as it does a turned number.
use alloc::{format, string::String, vec, vec::Vec};
use game::game::{Game};
use game::m;
use game::teams::Team;
use game::view::{ActionKind, ActionRow};

/// how a row is drawn and what it hands its action
pub enum Takes {
    /// a button
    Press,
    /// a button a word
    Each(fn(&Game) -> Vec<String>),
    /// a pick of words: what they are, and which it stands at now
    Choice(fn(&Game) -> Vec<String>, fn(&Game) -> String),
    /// a number picked on a slider beside a button
    Count { lo: f64, hi: f64, step: f64, start: f64 },
    /// a pick of words that is a new page of this address with `word`=the pick: the game can't change it while it plays
    Reload { word: &'static str, picks: &'static [&'static str] },
}
pub struct Action { pub name: &'static str, pub label: &'static str, pub section: &'static str, pub title: &'static str, pub takes: Takes, pub run: fn(&mut Game, f64, &str) }
impl Action {
    /// its row of the panel, as the game stands
    pub fn told(&self, g: &Game) -> ActionRow {
        let r = ActionRow { name: self.name, label: self.label, section: self.section, title: self.title, kind: ActionKind::Press, words: Vec::new(), now: String::new(), lo: 0.0, hi: 0.0, step: 0.0, start: 0.0, word: "" };
        match self.takes {
            Takes::Press => r,
            Takes::Each(all) => ActionRow { kind: ActionKind::Each, words: all(g), ..r },
            Takes::Choice(all, now) => ActionRow { kind: ActionKind::Choice, words: all(g), now: now(g), ..r },
            Takes::Count { lo, hi, step, start } => ActionRow { kind: ActionKind::Count, lo, hi, step, start, ..r },
            Takes::Reload { word, picks } => ActionRow { kind: ActionKind::Reload, word, words: picks.iter().map(|w| String::from(*w)).collect(), ..r },
        }
    }
}

/// (a pick of "as it is" takes the word off the address)
const WEATHERS: &[&str] = &["as it is", "clear", "overcast", "rain", "downpour", "fog", "dust"];
/// (a row's name, and the word an address says it by)
const TRAININGS: &[(&str, &str, game::morale::Training)] = &[("none (an old recording)", "none", game::morale::Training::None), ("by difficulty", "difficulty", game::morale::Training::Difficulty), ("all green", "green", game::morale::Training::Green), ("leader and green", "mixed", game::morale::Training::Mixed), ("all trained", "trained", game::morale::Training::Trained)];
const NERVES: &[(&str, game::morale::Nerve)] = &[("steady", game::morale::Nerve::Steady), ("normal", game::morale::Nerve::Normal), ("brittle", game::morale::Nerve::Brittle)];

fn aim(g: &Game) -> [f64; 3] { g.sets[g.seat].aim_point }
fn every_group(g: &Game) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for &t in g.targets.iter() {
        let f = &g.tab.fixture[t];
        if f.group == "emergency" || matches!(f.kind, game::fixtures::FixtureKind::Breaker) || out.contains(&f.group) { continue; }
        out.push(f.group.clone());
    }
    out
}
fn every_man(g: &Game) -> Vec<game::ids::CharId> {
    let mut v: Vec<game::ids::CharId> = g.seats.iter().map(|&s| g.pl[game::seats::seat(g, s).player].ch).collect();
    v.extend(g.guards.iter().map(|&gd| g.gd[gd].ch));
    v
}
fn soak(g: &mut Game, c: game::ids::CharId) {
    let ch = &mut g.ch[c];
    ch.wet = 1.0; ch.beads = 1.0; ch.wet_rain = 0.8; ch.wet_line = f64::INFINITY;
    let (prints, now) = (g.k.wet.prints, g.time);
    if let Some(m) = g.m.wet.mem.get_mut(c) { m.prints_left = prints; m.last_out_wet_t = now; }
}

/// a floor spot for a man stood by the panel: where, what he covers, and on a room clear the room and mark he counts for
struct Spot { pos: [f64; 3], aim_at: [f64; 3], room: Option<String>, mark: f64 }
const SPACING_M: f64 = 1.0;
const PROP_CLEAR_M: f64 = 0.45;
const SAM_CLEAR_M: f64 = 8.0;
fn free(g: &Game, p: [f64; 3], taken: &[[f64; 3]]) -> bool {
    !game::scene_collision::nav_grid_is_blocked(g, p[0], p[2])
        && !g.props.props.iter().any(|&i| { let r = &g.tab.props_prop[i]; game::props::prop_gap(r.home, r.hx, r.hz, p) < PROP_CLEAR_M })
        && !taken.iter().any(|t| game::v3::dist_xz(*t, p) < SPACING_M)
}
fn shuffled<T>(mut a: Vec<T>, die: &mut game::dice::Die) -> Vec<T> {
    for i in (1..a.len()).rev() { let j = m::floor(die.roll() * (i + 1) as f64) as usize; a.swap(i, j); }
    a
}
/// where n more men can stand: a room clear's unused marks and then its free floor, a room in turn; on any other floor anywhere a man can walk to from the player, none on top of him
fn extra_spots(g: &mut Game, n: usize) -> Vec<Spot> {
    let mut die = game::dice::Die(0x57E55);
    let mut taken: Vec<[f64; 3]> = vec![g.player_pos(g.player)];
    taken.extend(g.guards.iter().filter(|&&x| !game::condition::down(g, g.gd[x].ch)).map(|&x| g.pos_of(x)));
    taken.extend(g.civilians.iter().filter(|&&x| !game::condition::down(g, g.gd[x].ch)).map(|&x| g.pos_of(x)));
    let mut out: Vec<Spot> = Vec::new();
    if !g.staged.rooms.is_empty() {
        let rooms: Vec<_> = g.staged.rooms.iter().map(|r| r.def.clone()).collect();
        let mut lists: Vec<Vec<Spot>> = Vec::new();
        for r in &rooms {
            let mut spots: Vec<Spot> = r.marks.iter().enumerate().filter(|(k, _)| !g.room_clear.placement.iter().any(|p| p.room == r.id && p.mark == *k as f64)).map(|(k, q)| Spot { pos: q.pos, aim_at: q.aim_at, room: Some(r.id.clone()), mark: k as f64 }).collect();
            let (b, mut floor) = (&r.bounds, Vec::new());
            let mut x = b.x0 + 0.7;
            while x <= b.x1 - 0.7 {
                let mut z = b.z0 + 0.7;
                while z <= b.z1 - 0.7 {
                    let pos = [m::round(x * 100.0) / 100.0, 0.0, m::round(z * 100.0) / 100.0];
                    let eye = [pos[0], 1.6, pos[2]];
                    let q = r.marks.iter().find(|q| !game::scene_collision::wall_between(g, eye, q.aim_at)).or(r.marks.first());
                    if let Some(q) = q { floor.push(Spot { pos, aim_at: q.aim_at, room: Some(r.id.clone()), mark: -1.0 }); }
                    z += 0.8;
                }
                x += 0.8;
            }
            spots.extend(shuffled(floor, &mut die));
            spots.reverse();   // (taken from the end: its first first)
            lists.push(spots);
        }
        let mut left = true;
        while left && out.len() < n {
            left = false;
            for l in lists.iter_mut() {
                if out.len() >= n { break; }
                while let Some(s) = l.pop() { if free(g, s.pos, &taken) { taken.push(s.pos); out.push(s); left = true; break; } }
            }
        }
        return out;
    }
    let (sam, cell, w, h) = (g.player_pos(g.player), g.col.nav.cell, g.col.nav.w, g.col.nav.h);
    let mut cells = Vec::new();
    let mut x = 0.0;
    while x < w { let mut z = 0.0; while z < h { let p = [(x + 0.5) * cell, 0.0, (z + 0.5) * cell]; if game::v3::dist_xz(p, sam) >= SAM_CLEAR_M { cells.push(p); } z += 2.0; } x += 2.0; }
    for pos in shuffled(cells, &mut die) {
        if out.len() >= n { break; }
        if !free(g, pos, &taken) || game::scene_collision::nav_grid_find_path(g, sam, pos, None, None).is_none() { continue; }
        taken.push(pos);
        out.push(Spot { pos, aim_at: [sam[0], 1.1, sam[2]], room: None, mark: -1.0 });
    }
    out
}
/// more defenders on the floor, held on their spots, and the mission's where it has them. A restart takes them away.
fn stand_extra(g: &mut Game, n: f64) {
    let n = n.max(0.0) as usize;
    let spots = extra_spots(g, n);
    let mut stood = 0;
    let serial = g.guards.iter().filter(|&&x| g.gd[x].callsign.starts_with("Extra ")).count();
    let budget = game::guards::MAX_SKINNED - g.civilians.len() as f64;
    for s in spots {
        let name = format!("Extra {}", serial + stood + 1);
        let o = game::guards::UnitOpts { team: Team::Hostile, pos: s.pos, yaw: Some(game::v3::yaw_to(s.pos, s.aim_at)), name: Some(name.clone()), light: Some(false), budget: Some(budget), ..game::guards::UnitOpts::default() };
        let gd = match game::guards::spawn_unit(g, &o) { Ok(gd) => gd, Err(_) => break };
        g.gd[gd].duty.holds = true; g.gd[gd].duty.pinned = true;
        game::mission::adopt(g, gd, &game::nightbreach::NightBreachMark { pos: s.pos, aim_at: s.aim_at, name: Some(name.clone()), drill: Some(false) });
        if let Some(room) = s.room { game::roomclear::room_clear_scenario_adopt(g, gd, &game::roomclear::Placement { room, mark: s.mark, pos: s.pos, aim_at: s.aim_at, name }); }
        stood += 1;
    }
}

pub static ACTIONS: &[Action] = &[
    Action { name: "mains", label: "trip or restore the mains", section: "Sandbox", title: "everything on the breaker goes dark for 30 s", takes: Takes::Press, run: |g, _, _| if g.light_home.mains.active { game::game::game_end_blackout(g) } else { game::game::game_set_blackout(g, 30.0, game::lighting::SetBlackoutCause::Debug) } },
    Action { name: "smoke", label: "smoke at the cursor", section: "Sandbox", title: "", takes: Takes::Press, run: |g, _, _| { let a = aim(g); game::sense::sense_tool(g, 2.0, &[a[0], a[1] + 0.12, a[2]]); } },
    Action { name: "clearsmoke", label: "clear smoke", section: "Sandbox", title: "", takes: Takes::Press, run: |g, _, _| { game::sense::sense_tool(g, 3.0, &[]); } },
    Action { name: "teleport", label: "teleport to the cursor", section: "Sandbox", title: "", takes: Takes::Press, run: |g, _, _| { let a = aim(g); game::game::game_teleport_player(g, a); } },
    Action { name: "spectate", label: "spectate next", section: "Sandbox", title: "", takes: Takes::Press, run: |g, _, _| { let next = g.spectate + 1.0; g.spectate = if next >= g.guards.len() as f64 { -1.0 } else { next }; } },
    Action { name: "extra", label: "stand extra enemies", section: "Sandbox", title: "ordinary defenders held on a spot; a restart takes them away", takes: Takes::Count { lo: 1.0, hi: 15.0, step: 1.0, start: 3.0 }, run: |g, v, _| stand_extra(g, v) },
    Action { name: "lightson", label: "all lights on", section: "Lights", title: "", takes: Takes::Press, run: |g, _, _| for w in every_group(g) { game::lighting::set_group(g, &w, true); } },
    Action { name: "lightsoff", label: "all lights off", section: "Lights", title: "", takes: Takes::Press, run: |g, _, _| for w in every_group(g) { game::lighting::set_group(g, &w, false); } },
    Action { name: "groupon", label: "a group on", section: "Lights", title: "", takes: Takes::Each(every_group), run: |g, _, w| game::lighting::set_group(g, w, true) },
    Action { name: "groupoff", label: "a group off", section: "Lights", title: "", takes: Takes::Each(every_group), run: |g, _, w| game::lighting::set_group(g, w, false) },
    Action { name: "repair", label: "repair the lights", section: "Lights", title: "", takes: Takes::Press, run: |g, _, _| game::game::game_repair_lights(g) },
    Action { name: "killmains", label: "kill the mains", section: "Lights", title: "", takes: Takes::Press, run: |g, _, _| game::game::game_set_blackout(g, f64::INFINITY, game::lighting::SetBlackoutCause::Debug) },
    Action { name: "weather", label: "weather kind", section: "Weather", title: "a new page of this address in that weather", takes: Takes::Reload { word: "weather", picks: WEATHERS }, run: |_, _, _| {} },
    Action { name: "strike", label: "lightning now", section: "Weather", title: "", takes: Takes::Press, run: |g, _, _| { game::weather::weather_strike(g, None); } },
    Action { name: "strikeclose", label: "lightning, close (0.4 km)", section: "Weather", title: "", takes: Takes::Press, run: |g, _, _| { game::weather::weather_strike(g, Some(game::weather::WeatherStrikeOpts { dist_km: Some(0.4), ..Default::default() })); } },
    Action { name: "strikefar", label: "lightning, far (3.5 km)", section: "Weather", title: "", takes: Takes::Press, run: |g, _, _| { game::weather::weather_strike(g, Some(game::weather::WeatherStrikeOpts { dist_km: Some(3.5), ..Default::default() })); } },
    Action { name: "soak", label: "soak the player", section: "Wet characters", title: "as if just in from a downpour", takes: Takes::Press, run: |g, _, _| { let c = g.pl[g.player].ch; soak(g, c); } },
    Action { name: "soakall", label: "soak everyone", section: "Wet characters", title: "", takes: Takes::Press, run: |g, _, _| for c in every_man(g) { soak(g, c); } },
    Action { name: "dryall", label: "dry everyone", section: "Wet characters", title: "", takes: Takes::Press, run: |g, _, _| for c in every_man(g) { let ch = &mut g.ch[c]; ch.wet = 0.0; ch.beads = 0.0; ch.wet_rain = 0.0; } },
    Action { name: "training", label: "training", section: "Enemy nerve", title: "who is green", takes: Takes::Choice(|_| TRAININGS.iter().map(|t| String::from(t.0)).collect(), |g| TRAININGS.iter().find(|t| t.2 == g.k.morale.training).map_or(String::new(), |t| String::from(t.0))), run: |g, _, w| if let Some(t) = TRAININGS.iter().find(|t| t.0 == w || t.1 == w) { g.k.morale.training = t.2; } },
    Action { name: "nerve", label: "nerve", section: "Enemy nerve", title: "how much a shock shakes them", takes: Takes::Choice(|_| NERVES.iter().map(|t| String::from(t.0)).collect(), |g| NERVES.iter().find(|t| t.1 == g.k.morale.nerve).map_or(String::new(), |t| String::from(t.0))), run: |g, _, w| if let Some(t) = NERVES.iter().find(|t| t.0 == w) { g.k.morale.nerve = t.1; } },
];

/// do the action of that name; a name no action has is nothing
pub fn act(g: &mut Game, name: &str, value: f64, word: &str) { if let Some(a) = ACTIONS.iter().find(|a| a.name == name) { (a.run)(g, value, word); } }
