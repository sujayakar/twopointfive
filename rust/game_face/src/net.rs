//! A GAME PLAYED OVER THE NET, for any machine of one (a game server, a scripted client, a page): every machine starts the same game (`face_start` on the same
//! bytes, dressed), and steps it on the same records.
use super::{hand::cmd, Held};
use game::handed::SeatInput;
use game::view::AheadView;
use game_start::host::field::Laid;

fn seated(x: &Held, seat: u32) -> Result<usize, String> { if (seat as usize) < x.g.seats.len() { Ok(seat as usize) } else { Err(format!("the game has no seat {seat}")) } }

pub fn networked(x: &mut Held) {
    for s in x.g.seats.clone() { x.g.tab.cmd_seat[s].remote = true; }
    // (the lens a look ahead asks through is a row of the cameras' table, and a machine that never looks would not have it)
    game::game::game_aside_lens(&mut x.g);
}
pub fn record(x: &mut Held, seat: u32, Laid(input): Laid<SeatInput>, keys: &str) -> Result<(), String> {
    let seat = seated(x, seat)?;
    x.hand_over(seat, cmd(&input, None, keys));
    Ok(())
}
pub fn ahead(x: &mut Held, seat: u32, inputs: Vec<Laid<SeatInput>>, keys: Vec<&str>) -> Result<AheadView, String> {
    let r: Vec<_> = inputs.iter().zip(keys).map(|(Laid(i), k)| cmd(i, None, k)).collect();
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
