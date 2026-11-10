//! A GAME PLAYED OVER THE NET, for any machine of one (a game server, a scripted client, a page): every machine starts the same game (`face_start` on the same
//! bytes, dressed), and steps it on the same records.
//!
//!   face_networked(h)                  once, as the game is stood: every seat is another machine's, so a record's view is worked out again from its pose on
//!                                      every machine alike
//!   face_record(h, seat, p, n, keys, kn)   A SEAT'S RECORD FOR THE NEXT STEP, as it travelled: `RECORD` numbers (`n` bytes; src/net/record.ts says them in the same
//!                                      order) and its keys' codes as text, those held and, after a line feed, those that went down. It is no hand: the host's
//!                                      own camera is not moved by it. Of a seat that has both, what was handed last is stepped.
//!   face_ahead(h, seat, p, n, keys, kn)    WHERE THE SEAT'S MAN WOULD BE after records the game has not had (game::walkahead), at face_out as `AHEAD` numbers and
//!                                      then his place after each record. `p`: the records' numbers, one after another; their keys' text the same way, a line
//!                                      feed between two. No record: where the game has him. Nothing a tick reads is written.
use game::cmd::{PlayerCmd, PlayerCmdView};

/// the numbers of a record: buttons clicked wheel, cursor x y, the view's size in CSS pixels and in its own, the look-at, yaw pitch distance, field of view, aspect, how
/// far it leads the aim
const RECORD: usize = 18;

/// a record with its eye and its matrices left to be worked out
fn record(v: &[f64], held: &str, down: &str) -> PlayerCmd {
    let codes = |line: &str| line.split(' ').filter(|k| !k.is_empty()).map(String::from).collect::<Vec<String>>();
    PlayerCmd {
        keys: codes(held), pressed: codes(down), buttons: v[0], clicked: v[1], wheel: v[2], mx: v[3], my: v[4], w: v[5], h: v[6], pw: v[7], ph: v[8],
        view: PlayerCmdView { vp: vec![f64::NAN; 16], ivp: vec![f64::NAN; 16], pos: [0.0; 3], forward: [0.0; 3], target: [v[9], v[10], v[11]], yaw: v[12], pitch: v[13], distance: v[14], fov: v[15], aspect: v[16], aim_lead: v[17] },
    }
}
/// the records of a run of them
/// # Safety
/// `p`, `n` and `keys`, `kn` name bytes the host wrote into memory from face_alloc.
unsafe fn records(p: *const f64, n: usize, keys: *const u8, kn: usize) -> Vec<PlayerCmd> {
    assert!(n % (RECORD * 8) == 0, "{} bytes of records, and a record is {} numbers", n, RECORD);
    if n == 0 { return Vec::new(); }
    let (v, text) = (std::slice::from_raw_parts(p, n / 8), super::text(keys, kn));
    let mut lines = text.split('\n');
    v.chunks(RECORD).map(|v| record(v, lines.next().unwrap_or(""), lines.next().unwrap_or(""))).collect()
}

#[no_mangle]
pub extern "C" fn face_networked(h: u32) {
    super::with(h, |x| {
        for s in x.g.seats.clone() { x.g.tab.cmd_seat[s].remote = true; }
        // (the lens a look ahead asks through is a row of the cameras' table, and a machine that never looks would not have it)
        game::game::game_aside_lens(&mut x.g);
    });
}
/// # Safety
/// `p`, `n` and `keys`, `kn` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_record(h: u32, seat: usize, p: *const f64, n: usize, keys: *const u8, kn: usize) {
    let mut r = records(p, n, keys, kn);
    assert!(r.len() == 1, "face_record: {} records, and a tick has one", r.len());
    // (asked inside and refused outside the call that holds the game: a seat it has not got loses the call and not the game)
    let known = super::with(h, |x| { let known = seat < x.g.seats.len(); if known { x.hand_over(seat, r.remove(0)); } known });
    assert!(known, "face_record: the game has no seat {}", seat);
}
/// # Safety
/// `p`, `n` and `keys`, `kn` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_ahead(h: u32, seat: usize, p: *const f64, n: usize, keys: *const u8, kn: usize) {
    let r = records(p, n, keys, kn);
    let a = super::with(h, |x| {
        if seat >= x.g.seats.len() { return None; }
        let s = x.g.seats[seat];
        let mut path = Vec::new();
        let a = game::walkahead::walk_ahead(&mut x.g, s, &r, 1.0 / 60.0, Some(&mut path));
        let g = &x.g;
        let here = game::seats::seat(g, s);
        let (pl, yes) = (&g.pl[here.player], |b: bool| if b { 1.0 } else { 0.0 });
        let c = &g.ch[pl.ch];
        let wall = pl.wall.as_ref().map(|w| w.n);
        let others = g.guards.iter().chain(g.civilians.iter()).map(|&gd| g.gd[gd].ch).chain(g.seats.iter().map(|&s| g.pl[game::seats::seat(g, s).player].ch));
        let near = others.filter(|&o| o != pl.ch).map(|o| { let p = g.ch[o].pos; game::v3::dist_xz(p, c.pos) }).fold(f64::INFINITY, f64::min);
        let mut o: Vec<f64> = vec![c.id, yes(game::condition::down(g, pl.ch)), c.pos[0], c.pos[1], c.pos[2], c.aim_yaw];
        o.extend([yes(wall.is_some()), wall.map_or(0.0, |n| n[0]), wall.map_or(0.0, |n| n[2]), near]);
        o.extend([yes(a.free), a.pos[0], a.pos[1], a.pos[2], a.aim_yaw, a.follow[0], a.follow[1], a.follow[2]]);
        let at = a.aim.unwrap_or([0.0; 3]);
        o.extend([yes(a.aim.is_some()), at[0], at[1], at[2]]);
        path.iter().for_each(|p| o.extend(p));
        Some(o.iter().flat_map(|f| f.to_le_bytes()).collect())
    });
    super::out(a.unwrap_or_else(|| panic!("face_ahead: the game has no seat {}", seat)));
}
