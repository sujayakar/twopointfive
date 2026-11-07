//! THE SHOWN SEAT'S VIEW: what the page's camera follows, what the tick did to it, and the few numbers of his that the picture itself is drawn by.
use alloc::vec::Vec;
use game::game::Game;

section! {
    /// the seat this machine shows: one row
    pub struct View {
        /// the tick this is of: sixtieths of a second on the game's clock
        pub tick: u32,
        /// the shown man's id
        pub man: u32,
        /// where he stands, and where the view follows (him, a man spectated, the squad from the sky)
        pub at: [f32; 3],
        pub follow: [f32; 3],
        /// where his aim meets the world
        pub aim: [f32; 3],
        /// the view is another man's: nothing leans on the cursor
        pub spectating: u32,
        /// the view is held off the follow point by this much (a peek round a wall's end), and kicked for this frame's picture by that much
        pub peek: [f32; 3],
        pub kick: [f32; 3],
        /// the tick cut the view to here: once each time the count moves
        pub cuts: u32,
        pub cut: [f32; 3],
        /// night vision's amount, and how hard he has just been hit
        pub night: f32,
        pub hit: f32,
    }
}

pub fn fill(g: &Game, out: &mut Vec<View>) {
    let f3 = |v: [f64; 3]| [v[0] as f32, v[1] as f32, v[2] as f32];
    let seat = g.local_seat.unwrap_or(g.seats[0]);
    let pl = game::seats::host_man(g);
    let (ch, cam) = (g.pl[pl].ch, &g.tab.camera[g.tab.cmd_seat[seat].cam]);
    let at = g.ch[ch].pos;
    let spectated = if g.spectate >= 0.0 { g.guards.get(g.spectate as usize).map(|&gd| g.ch[g.gd[gd].ch].pos) } else { None };
    let sky = if game::rails::rails_sky_on(g) { g.rails.sky.mid } else { None };
    out.push(View {
        tick: crate::tick(g),
        man: g.ch[ch].id as u32,
        at: f3(at),
        follow: f3(spectated.or(sky).unwrap_or(at)),
        aim: f3(g.aim_point),
        spectating: (g.spectate >= 0.0) as u32,
        peek: cam.peek_offset.map_or([0.0; 3], f3),
        kick: cam.kick.map_or([0.0; 3], f3),
        cuts: cam.desired_serial,
        cut: f3(cam.desired),
        night: g.pl[pl].nv_amount as f32,
        hit: g.pl[pl].hit_flash as f32,
    });
}
