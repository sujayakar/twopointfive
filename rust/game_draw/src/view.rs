//! THE SHOWN SEAT'S VIEW: what the page's camera follows, what the tick did to it, and the few numbers of his that the picture itself is drawn by.
use crate::Row;
use alloc::vec::Vec;
use game::game::Game;

section! {
    /// the seat this machine shows: one row
    pub struct View {
        /// the tick this is of: sixtieths of a second on the game's clock
        pub tick: u32,
        /// moves when the game is another than the one last drawn (a restart, a load, another seat): nothing is drawn or heard through it
        pub stood: u32,
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
        /// a show or a level's script holds the view here: where it looks, and from how far and which way (0: nobody does)
        pub pinned: u32,
        pub pin: [f32; 3],
        pub pin_yaw: f32,
        pub pin_pitch: f32,
        pub pin_distance: f32,
        /// the tick turned the host's own view: 1 its yaw · 2 its pitch · 4 its distance · 8 where it looks, to these
        pub turned: u32,
        pub yaw: f32,
        pub pitch: f32,
        pub distance: f32,
        pub target: [f32; 3],
        /// what a lesson does not let the hand do to the view: 1 turn it · 2 zoom it
        pub locks: u32,
        /// saves made of this game so far: moves when there is another
        pub saves: u32,
        /// what the game is set to, for a screen that sets it: hits do nothing to him, the men think, unlimited ammunition, it saves by itself, the wall hold's feel, walking into a wall takes
        /// it, the command key tapped twice calls the squad in
        pub god: u32,
        pub thinks: u32,
        pub ammo: u32,
        pub autosave: u32,
        pub wall_feel: u32,
        pub wall_press: u32,
        pub fall_in: u32,
        /// the run's end card is due: 1 it was lost, 2 won (0: not yet)
        pub ended: u32,
        /// this run, for whoever keeps what was won: 1 its try has begun · 2 it is over (what there is to keep is asked: face_run_view)
        pub run: u32,
        /// night vision's amount, and how hard he has just been hit
        pub night: f32,
        pub hit: f32,
        /// this game is the Team & kit picture: 1 staged, 2 nothing to stage (0: it is no picture)
        pub kit: u32,
    }
}

pub fn fill(g: &Game, stood: u32, desired: f64, out: &mut Vec<View>) {
    let f3 = |v: [f64; 3]| [v[0] as f32, v[1] as f32, v[2] as f32];
    let seat = g.local_seat.unwrap_or(g.seats[0]);
    let pl = game::seats::host_man(g);
    let (ch, cam) = (g.pl[pl].ch, &g.tab.camera[g.tab.cmd_seat[seat].cam]);
    let at = g.ch[ch].pos;
    let spectated = if g.spectate >= 0.0 { g.guards.get(g.spectate as usize).map(|&gd| g.ch[g.gd[gd].ch].pos) } else { None };
    let sky = if game::rails::rails_sky_on(g) { g.rails.sky.mid } else { None };
    let mut r = View::ZERO;
    if let Some(host) = g.host_cam {
        let c = &g.tab.camera[host];
        if let Some(p) = c.pin { (r.pinned, r.pin, r.pin_yaw, r.pin_pitch, r.pin_distance) = (1, f3(p.look), p.yaw as f32, p.pitch as f32, p.dist as f32); }
        // (the hand's own view, as the step was handed it: what the row says otherwise, a rule wrote)
        else if let Some(h) = g.tab.cmd_seat[seat].cmd.as_ref() {
            let v = &h.view;
            r.turned = (c.yaw != v.yaw) as u32 | ((c.pitch != v.pitch) as u32) << 1 | ((c.distance != v.distance || c.desired_distance != desired) as u32) << 2 | ((c.target != v.target) as u32) << 3;
            r.target = f3(c.target);
            (r.yaw, r.pitch, r.distance) = (c.yaw as f32, c.pitch as f32, c.desired_distance as f32);
        }
    }
    r.ended = ended(g);
    { let (begun, over) = game::verdict::run_on(g); r.run = begun as u32 | (over as u32) << 1; }
    r.saves = (g.saves.stats.autosaves + g.saves.stats.manual) as u32;
    (r.god, r.thinks, r.ammo, r.autosave) = (g.god_mode as u32, g.ai_enabled as u32, g.infinite_ammo as u32, g.saves.enabled as u32);
    (r.wall_feel, r.wall_press, r.fall_in) = (g.k.wallsnap.on as u32, g.k.wallsnap.press as u32, game::orders::squad_orders_twice_on(g) as u32);
    r.locks = game::rails::rails_locks(g, "camera") as u32 | (game::rails::rails_locks(g, "zoom") as u32) << 1;
    out.push(View {
        tick: crate::tick(g),
        stood,
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
        kit: g.kit_stage.as_ref().map_or(0, |k| if k.none { 2 } else { 1 }),
        ..r
    });
}

/// is this run over, and its card due: the verdict's a beat after the end, the stealth mission's as it is done or, down, a beat after
fn ended(g: &Game) -> u32 {
    use game::game::MissionStage;
    let (v, m) = (&g.verdict, &g.mission);
    if v.run == Some(m.serial) && v.card_at >= 0.0 { return if g.time < v.card_at { 0 } else { v.res.as_ref().map_or(0, |r| if r.won { 2 } else { 1 }) }; }
    if g.mission_in_tour { return 0; }
    match m.stage {
        MissionStage::Done => 2,
        MissionStage::Failed if m.down_card_at > 0.0 => 0,
        MissionStage::Failed => 1,
        _ => 0,
    }
}
