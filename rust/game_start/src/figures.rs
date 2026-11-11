//! WHAT PLAY IS AT A MOMENT: plain numbers read off the game by what they mean and never by how it keeps them, so a part of the state that only changes shape moves none of them. Who stands where, what he holds, what he
//! was told, the light he stands in; the doors; the panes, the lamps, the smoke; what is destroyed; the alarm, the mission and its tally; what is in the air and on the floor; the die. A replay's map of a run reads its men here too (`men`), and the gate's pins are a word of these.
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use game::game::Game;
use game::ids::*;
use game::view::MapKind;

/// a man on the floor, as anybody who reads a run reads him: the players first in their seats' order, then the men, then the civilians
pub struct Man { pub ch: CharId, pub team: String, pub kind: MapKind, pub down: bool, pub name: String, pub state: String, pub guard: Option<GuardId>, pub player: Option<(usize, PlayerId)> }

/// everybody on the floor, seen or not
pub fn men(g: &Game) -> Vec<Man> {
    let mut out = Vec::new();
    for (i, &s) in g.seats.iter().enumerate() { let p = game::cmd::seat_man(g, s); let q = &g.pl[p]; out.push(Man { ch: q.ch, team: String::from("player"), kind: MapKind::Player, down: g.cond[q.ch].down, name: game::seats::player_name(g, p), state: String::new(), guard: None, player: Some((i, p)) }); }
    for &gd in &g.guards { let m = &g.gd[gd]; out.push(Man { ch: m.ch, team: g.ch[m.ch].team.as_str().to_string(), kind: MapKind::Man, down: g.cond[m.ch].down, name: m.callsign.clone(), state: m.state.as_str().to_string(), guard: Some(gd), player: None }); }
    for &gd in &g.civilians { let m = &g.gd[gd]; out.push(Man { ch: m.ch, team: String::from("civilian"), kind: MapKind::Civ, down: g.cond[m.ch].down, name: m.callsign.clone(), state: String::new(), guard: Some(gd), player: None }); }
    out
}

fn bits(v: f64) -> String { format!("{:016x}", v.to_bits()) }
/// what is in a gun: chambered and in the magazine, the spare magazines, the rounds it has fired
fn rounds(g: &Game, gun: FirearmId) -> String { let f = &g.fa[gun]; format!("{} {} {} {}", f.chamber, f.mag.unwrap_or(-1.0), f.spare.iter().sum::<f64>(), f.shots_fired) }
fn order_of(g: &Game, gd: GuardId) -> String {
    for &s in &g.seats { if let Some(a) = game::orders::squad_orders_of(&game::seats::seat(g, s), gd) { return g.tab.task[a].order.clone(); } }
    String::new()
}

/// every pane that has been touched, by its box's number in the level: gone or not, how many pieces of it are open, its sites, its damage, how many rounds have gone through it. A round can go through a pane no blow has
/// touched, so a pane is on the list by its holes too. A pane nothing has touched is whole, and has no figure.
fn glass(g: &Game) -> BTreeMap<u32, String> {
    let panes = &sim::state::state(g.sim).panes;
    let mut holes: BTreeMap<u32, u32> = BTreeMap::new();
    for h in 0..panes.nh as usize { let row = panes.hole_read(h); *holes.entry(row.bi).or_insert(0) += row.n; }
    let mut out = BTreeMap::new();
    for r in 0..panes.n as usize {
        let row = panes.row_read(r);
        let (bi, flags) = (row.w[0], row.w[sim::glass::GW_FLAGS]);
        out.insert(bi, format!("{} {} {} {} {}", flags & sim::glass::G_GONE != 0, row.n_open, flags >> 8, bits(row.damage), holes.get(&bi).copied().unwrap_or(0)));
    }
    for (bi, n) in holes { out.entry(bi).or_insert_with(|| format!("false 0 0 {} {}", bits(0.0), n)); }
    out
}
/// a lamp, by its number in the level: its switch, shot or not, out because the mains are, and what it burns at this tick
fn lamp(g: &Game, id: FixtureId) -> String {
    format!("{} {} {} {}", g.tab.fixture[id].on, g.tab.fixture[id].broken, g.tab.fixture[id].mains && g.light_home.mains.active, bits(g.tab.fixture[id].factor))
}
/// the smoke: how many emitters are letting it out, then each brick of the grid that holds any, by its place in plan, with all it holds. Sorted by place, because which slot a brick has is how the grid keeps it.
fn smoke(g: &Game) -> String {
    use sim::bricks::{CELLS, MAX_BRICKS, O_KEYS, O_RHO};
    let (mut said, mut standing) = (format!("{}", sim::bricks::emitting(g.sim)), Vec::new());
    if !sim::bricks::smoky(g.sim) { return said; }
    let st = &sim::state::state_read(g.sim).smoke.st;
    for b in 0..MAX_BRICKS {
        let (x, z) = (st[O_KEYS + b * 2], st[O_KEYS + b * 2 + 1]);
        if x < 1.0 || z < 1.0 { continue; }
        let amount: f64 = st[O_RHO + b * CELLS..O_RHO + (b + 1) * CELLS].iter().sum();
        if amount > 0.0 { standing.push((z as u32, x as u32, amount)); }
    }
    standing.sort_by_key(|&(z, x, _)| (z, x));
    for (z, x, amount) in standing { said += &format!(" {} {} {}", x, z, bits(amount)); }
    said
}
/// what a blast has taken: how many statics stand, then each that is hidden or cut, by its place in the level, then the wall skins that are out. A box hidden and a box put back are a list that moves back; a counter of writes would not.
fn destroyed(g: &Game) -> String {
    let w = &g.engine.world;
    let mut said = format!("{}", w.statics.len());
    let hidden = game::wallbreak::HIDE as u32;
    for (i, &b) in w.statics.iter().enumerate() {
        let x = &g.tab.boxes[b];
        let cut = x.cut.as_ref().map_or(0, |c| c.len());
        if (x.flags as i64 as u32) & hidden == hidden || cut > 0 { said += &format!(" {}/{}/{}", i, (x.flags as i64 as u32) & hidden == hidden, cut); }
    }
    let mut out: Vec<u64> = g.destr.walls.skins.out.iter().map(|n| n.0.to_bits()).collect();
    out.sort();
    for n in out { said += &format!(" {:x}", n); }
    said
}

/// EVERY FIGURE, named: `who what` and its value as text
pub fn read(g: &Game) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    out.push((String::from("draws"), format!("{}", g.dice.n)));
    for m in men(g) {
        let c = &g.ch[m.ch];
        let who = match (m.player, m.kind) { (Some((i, _)), _) => format!("seat {}", i), (_, MapKind::Civ) => format!("civilian {}", c.id), _ => format!("man {}", c.id) };
        out.push((format!("{} where", who), format!("{} {} {}", bits(c.pos[0]), bits(c.pos[1]), bits(c.pos[2]))));
        out.push((format!("{} facing", who), format!("{} {}", bits(c.body_yaw), bits(c.aim_yaw))));
        if let Some((_, p)) = m.player {
            let q = &g.pl[p];
            out.push((format!("{} up", who), format!("{} {}", g.cond[q.ch].down, q.hits_left)));
            out.push((format!("{} light", who), bits(g.shown[q.ch].visibility)));
            out.push((format!("{} holds", who), format!("{} {} {}", q.slot.as_str(), q.holstered, q.rifle_gone)));
            out.push((format!("{} rounds", who), format!("{} {}", rounds(g, q.pistol), rounds(g, q.rifle))));
        }
        if let Some(gd) = m.guard {
            let q = &g.gd[gd];
            out.push((format!("{} up", who), format!("{} {} {} {}", m.team, m.state, q.down_kind.map_or("", |k| k.as_str()), q.down_by.map_or("", |b| b.as_str()))));
            out.push((format!("{} holds", who), format!("{} {}", g.fa[q.weapon].def.as_str(), q.second.as_ref().map_or(String::new(), |s| format!("{} {}", s.id.as_str(), s.out)))));
            out.push((format!("{} rounds", who), format!("{} {}", rounds(g, q.weapon), q.second.as_ref().map_or(String::new(), |s| rounds(g, s.gun)))));
            out.push((format!("{} order", who), order_of(g, gd)));
        }
    }
    for &d in &g.doors.list {
        out.push((format!("door {}", g.dr[d].i), format!("{} {} {} {} {}", bits(g.dr[d].leaf.angle), g.dr[d].leaf.latched, g.dr[d].leaf.locked, g.dr[d].leaf.lock_broken, g.dr[d].leaf.off)));
    }
    for (bi, said) in glass(g) { out.push((format!("glass {}", bi), said)); }
    for id in g.tab.fixture.ids() { out.push((format!("lamp {}", id.0 as f64), lamp(g, id))); }
    out.push((String::from("smoke"), smoke(g)));
    out.push((String::from("destroyed"), destroyed(g)));
    out.push((String::from("alarm"), format!("{} {} {} {} {}", g.mission.alarm_on, g.alarm.episode, bits(g.alarm.raised_at), g.escalation, bits(g.escalation_t))));
    let t = &g.mission.stats;
    out.push((String::from("mission"), format!("{} {} {}", g.mission.stage.as_str(), g.room_clear.end.map_or("", |e| e.as_str()), g.verdict.res.as_ref().and_then(|r| r.kind).map_or("", |k| k.as_str()))));
    out.push((String::from("tally"), format!("{} {} {} {} {} {}", t.alerts, t.spotted, t.bodies, t.kills, t.knockouts, t.keys)));
    out.push((String::from("shots"), format!("{}", t.shots)));
    let mut things = String::new();
    for &id in &g.items.items { let it = &g.tab.item[id]; things += &format!("{} {} {} {} {} ", it.kind as u32, bits(it.pos[0]), bits(it.pos[1]), bits(it.pos[2]), it.settled as u32 | (it.fired as u32) << 1 | ((it.carried == Some(true)) as u32) << 2); }
    out.push((String::from("things"), format!("{}{} {}", things, g.shells.len(), g.rings.len())));
    out
}

/// A SERIES OF FIGURES, one row a second: a word of all of them, and a hex digit of each by its name, which is what lets a moved word say which figure moved
#[derive(Default)]
pub struct Track { pub names: Vec<String>, pub words: Vec<u32>, rows: Vec<Vec<(usize, u8)>> }
fn hash(text: &str, mut h: u32) -> u32 { for b in text.bytes() { h = (h ^ b as u32).wrapping_mul(0x0100_0193); } h }
impl Track {
    pub fn second(&mut self, g: &Game) {
        let (mut all, mut row) = (0x811c_9dc5u32, Vec::new());
        for (name, value) in read(g) {
            all = hash(&value, hash(&name, all));
            let at = self.names.iter().position(|n| *n == name).unwrap_or_else(|| { self.names.push(name.clone()); self.names.len() - 1 });
            let v = hash(&value, 0x811c_9dc5);
            row.push((at, ((v ^ (v >> 4) ^ (v >> 8) ^ (v >> 12) ^ (v >> 16) ^ (v >> 20) ^ (v >> 24) ^ (v >> 28)) & 15) as u8));
        }
        self.words.push(all);
        self.rows.push(row);
    }
    /// a row a second: a hex digit a name, `-` where he was not there yet
    pub fn digits(&self) -> Vec<String> {
        self.rows.iter().map(|row| { let mut d = alloc::vec![b'-'; self.names.len()]; for (at, v) in row { d[*at] = b"0123456789abcdef"[*v as usize]; } String::from_utf8(d).unwrap() }).collect()
    }
}

/// what a reader reads of a moment: who is down, the rounds, the orders
pub fn line(g: &Game) -> String {
    let mut said = Vec::new();
    for (i, &s) in g.seats.iter().enumerate() { let q = &g.pl[game::cmd::seat_man(g, s)]; said.push(format!("seat {} {}", i, if g.cond[q.ch].down { "down" } else { "up" })); }
    let side = |ours: bool| g.guards.iter().filter(move |&&x| (g.team(x).ours()) == ours);
    let up = |ours: bool| side(ours).filter(|&&x| !game::condition::down(g, g.gd[x].ch) && g.gd[x].down_kind.is_none()).count();
    said.push(format!("{} of {} ours up", up(true), side(true).count()));
    said.push(format!("{} of {} others up", up(false), side(false).count()));
    said.push(format!("{} rounds", g.mission.stats.shots));
    let mut orders: Vec<String> = side(true).map(|&x| order_of(g, x)).filter(|o| !o.is_empty()).collect();
    orders.sort();
    if !orders.is_empty() { said.push(format!("told {}", orders.join(", "))); }
    said.join(" · ")
}
