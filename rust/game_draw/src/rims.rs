//! WHOSE MEN ARE FOUND IN THE DARK: the shown player's own man and everybody on his team who is up, each with the light on him and the hue that says whose he is
//! (src/ui/findme.ts). Nobody while every player is down.
use alloc::vec::Vec;
use game::game::{Game, PairId};

section! {
    /// a man of the team: the first row is the shown player's own
    pub struct Rim {
        pub id: u32,
        /// 0 the first player's, 1 the red pair's, 2 the blue pair's, 3 the second player's
        pub hue: u32,
        /// he is on his feet
        pub up: u32,
        pub light: f32,
        pub at: [f32; 3],
    }
}

pub fn fill(g: &Game, out: &mut Vec<Rim>) {
    if game::seats::all_down(g) { return; }
    let shown = g.local_seat.unwrap_or(g.seats[0]);
    let mut put = |c: game::ids::CharId, hue: u32, up: bool, light: f64| out.push(Rim { id: g.ch[c].id as u32, hue, up: up as u32, light: light as f32, at: g.ch[c].pos.map(|x| x as f32) });
    let tone = |s: game::ids::SeatId| if g.seats.iter().position(|x| *x == s).unwrap_or(0) % 2 == 0 { 0 } else { 3 };
    for s in core::iter::once(shown).chain(g.seats.iter().copied().filter(|s| *s != shown)) {
        let p = &g.pl[game::cmd::seat_man(g, s)];
        if s == shown || !p.down { put(p.ch, tone(s), !p.down, p.visibility); }
    }
    for gd in game::seats::team_men(g, &game::seats::seat(g, shown)) {
        let m = &g.gd[gd];
        let red = m.unit.as_ref().is_some_and(|u| u.pair == Some(PairId::Red));
        put(m.ch, if red { 1 } else { 2 }, true, m.visibility);
    }
}
