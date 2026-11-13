//! THE MODULE A PAGE RUNS IS THIS GAME: `gate records` plays a few fights here and says, a line a tick, what each seat's hands did, and every second the word of each part of the game.
//! tools/draw/native-check.ts hands the checked-in module the same hands and asks for the same words.
use crate::rows::{hex, mission, stand, SEED};
use game::cmd::PlayerCmd;
use game_start::hands_off::{HandsOff, Plan};

/// the fights: a mission, how many players, seconds, and whether hands are on it (or nobody does anything)
const FIGHTS: [(&str, u32, usize, bool); 5] = [("room-clear-four", 2, 30, true), ("hostage-rescue", 1, 40, true), ("block-four", 1, 40, true), ("pass-four", 2, 30, true), ("bazaar-quiet", 1, 30, false)];

/// as it crosses to another machine (game_face `record`): no lens of his, his eye and his matrices are worked out there
fn crossed(c: &mut PlayerCmd) {
    c.view.vp = vec![f64::NAN; 16];
    c.view.ivp = vec![f64::NAN; 16];
    (c.view.pos, c.view.forward) = ([0.0; 3], [0.0; 3]);
}
/// a hand as the module is handed it (handed.rs `SeatInput`, in its order), then his keys held, and those that went down
fn row(c: &PlayerCmd) -> String {
    let v = &c.view;
    let n = [c.buttons, c.clicked, c.wheel, c.mx, c.my, c.w, c.h, c.pw, c.ph, v.target[0], v.target[1], v.target[2], v.yaw, v.pitch, v.distance, v.fov, v.aspect, v.aim_lead];
    format!("{}\t{}\t{}", n.map(|x| x.to_string()).join(" "), c.keys.join(" "), c.pressed.join(" "))
}

pub fn print() {
    for (id, seats, secs, hands) in FIGHTS {
        let e = mission(id);
        let mut p = stand(&e, SEED, seats);
        if let Some(why) = &p.refused { panic!("{id} is refused: {why}"); }
        // (as a machine that is sent its hands: game_face `networked`)
        for s in p.g.seats.clone() { p.g.tab.cmd_seat[s].remote = true; }
        game::game::game_aside_lens(&mut p.g);
        println!("fight\t{id}\t{seats}\t{SEED}");
        println!("words\t{}", hex(&p.words()));
        let mut players: Vec<HandsOff> = (0..seats as usize).map(|k| HandsOff::new(p.g.seats[k], Plan { seed: SEED + k as u32, cap: secs as f64 + 1.0, ..Default::default() })).collect();
        for i in 0..secs * 60 {
            if hands { for (k, him) in players.iter_mut().enumerate() { p.lens_of(him, k); } }
            let mut cmds = p.net_sample();
            if hands { for (him, c) in players.iter_mut().zip(&mut cmds) { p.hand_by(him, c); } }
            for (k, c) in cmds.iter_mut().enumerate() { crossed(c); println!("hand\t{k}\t{}", row(c)); }
            p.net_step(&cmds, 0);
            game::savestate::control::stood(&mut p.g);
            game::savestate::control::restart_turn(&mut p.g);
            p.lens_and_keys();
            println!("tick");
            if i % 60 == 59 { println!("words\t{}", hex(&p.words())); }
        }
    }
}
