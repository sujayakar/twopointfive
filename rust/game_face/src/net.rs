//! A GAME PLAYED OVER THE NET, for any machine of one (a game server, a scripted client, a page): every machine starts the same game (`face_start` on the same
//! bytes, dressed), and steps it on the same records.
use super::{hand::codes, Held};
use game::cmd::{PlayerCmd, PlayerCmdView};
use game::view::AheadView;

/// the numbers of a record: buttons clicked wheel, cursor x y, the view's size in CSS pixels and in its own, the look-at, yaw pitch distance, field of view, aspect, how
/// far it leads the aim
const RECORD: usize = 18;

/// a record with its eye and its matrices left to be worked out
fn one(v: &[f64], held: &str, down: &str) -> PlayerCmd {
    PlayerCmd {
        keys: codes(held), pressed: codes(down), buttons: v[0], clicked: v[1], wheel: v[2], mx: v[3], my: v[4], w: v[5], h: v[6], pw: v[7], ph: v[8],
        view: PlayerCmdView { vp: vec![f64::NAN; 16], ivp: vec![f64::NAN; 16], pos: [0.0; 3], forward: [0.0; 3], target: [v[9], v[10], v[11]], yaw: v[12], pitch: v[13], distance: v[14], fov: v[15], aspect: v[16], aim_lead: v[17] },
    }
}
/// the records of a run of them, or why they are none
fn records(v: &[f64], keys: &str) -> Result<Vec<PlayerCmd>, String> {
    if v.len() % RECORD != 0 { return Err(format!("{} numbers of records, and a record is {}", v.len(), RECORD)); }
    let mut lines = keys.split('\n');
    Ok(v.chunks(RECORD).map(|v| one(v, lines.next().unwrap_or(""), lines.next().unwrap_or(""))).collect())
}
fn seated(x: &Held, seat: u32) -> Result<usize, String> { if (seat as usize) < x.g.seats.len() { Ok(seat as usize) } else { Err(format!("the game has no seat {seat}")) } }

pub fn networked(x: &mut Held) {
    for s in x.g.seats.clone() { x.g.tab.cmd_seat[s].remote = true; }
    // (the lens a look ahead asks through is a row of the cameras' table, and a machine that never looks would not have it)
    game::game::game_aside_lens(&mut x.g);
}
pub fn record(x: &mut Held, seat: u32, numbers: Vec<f64>, keys: &str) -> Result<(), String> {
    let (seat, mut r) = (seated(x, seat)?, records(&numbers, keys)?);
    if r.len() != 1 { return Err(format!("{} records, and a tick has one", r.len())); }
    x.hand_over(seat, r.remove(0));
    Ok(())
}
pub fn ahead(x: &mut Held, seat: u32, numbers: Vec<f64>, keys: &str) -> Result<AheadView, String> {
    let r = records(&numbers, keys)?;
    let s = x.g.seats[seated(x, seat)?];
    let mut path = Vec::new();
    let a = game::walkahead::walk_ahead(&mut x.g, s, &r, 1.0 / 60.0, Some(&mut path));
    let g = &x.g;
    let pl = &g.pl[game::seats::seat(g, s).player];
    let c = &g.ch[pl.ch];
    let others = g.guards.iter().chain(g.civilians.iter()).map(|&gd| g.gd[gd].ch).chain(g.seats.iter().map(|&s| g.pl[game::seats::seat(g, s).player].ch));
    let near = others.filter(|&o| o != pl.ch).map(|o| game::v3::dist_xz(g.ch[o].pos, c.pos)).fold(f64::INFINITY, f64::min);
    Ok(AheadView {
        id: c.id, down: game::condition::down(g, pl.ch), at: c.pos, yaw: c.aim_yaw, wall: pl.wall.as_ref().map(|w| [w.n[0], w.n[2]]), near,
        free: a.free, pos: a.pos, aim_yaw: a.aim_yaw, follow: a.follow, aim: a.aim, path,
    })
}
