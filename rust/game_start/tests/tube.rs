//! WHAT A GOGGLED MAN IS SHOWN (rust/game_rules/src/teammates/nvg.rs, irlight): by the place's own light through the tube, or by the team's infrared floods where that shows too little.
use game_start::play::{Opts, Play};

/// THE RULE: however much light a dark place has, a man in it is plain to a goggled teammate who looks at him: the tube shows him by the place's light, or the flood is on.
/// What this holds. A corridor outside a room lit by one bulb, where a man stands at the dark end: 0.004 W came to count as light enough for the flood to stay
/// out, and not enough for the tube to show him by (0.22 of the meter). The rescue's pair walked up to within two metres of him and neither saw him shoot.
/// A FAILURE MEANS: teammates stand blind in a half-dark place that a darker one would not blind them in.
#[test]
fn no_half_dark_place_blinds_a_goggled_man() {
    let mut p = Play::mission("test-rooms.json", "?levelFile=test-rooms.json&rescue", &Opts { seed: 1, ..Default::default() });
    p.sam_unhurt();
    let mate = p.g.guards.iter().copied().find(|&gd| p.g.team(gd).ours() && p.g.gd[gd].unit.is_some()).expect("the pair");
    let theirs: Vec<_> = p.g.guards.iter().copied().filter(|&gd| !p.g.team(gd).ours()).collect();
    let mut seen = Vec::new();
    for k in 0..8 {
        // (five metres short of the spot and looking along the corridor at it, with nobody who can do anything about him)
        let x = 21.6 + k as f64;
        for _ in 0..150 {
            for &o in &theirs { let until = p.g.time + 5.0; p.g.cond[p.g.gd[o].ch].blind_until = until; p.man_stands(o, [4.0, 4.0]); }
            p.man_stands(mate, [x - 5.0, 14.2]);
            let c = p.g.gd[mate].ch;
            (p.g.ch[c].aim_yaw, p.g.ch[c].body_yaw) = (core::f64::consts::FRAC_PI_2, core::f64::consts::FRAC_PI_2);
            p.frame();
        }
        if !game::nvg::nvg_down(&p.g, mate) { continue; }
        let at = [x, 1.2, 14.2];
        let (mut place, mut flood) = (0.0, 0.0);
        game::scene_collision::window(&mut p.g, &mut |g| { place = game::nvg::nvg_place_light(g, at); flood = game::irlight::ir_light_at(g, at, None); });
        seen.push((x, place, flood, game::nvg::tube_vis(&p.g, place + flood)));
    }
    assert!(seen.iter().any(|s| s.1 > 0.0028 && s.1 < 0.0104), "the scene: some of the corridor was to be half dark: {seen:.4?}");
    for (x, place, flood, tube) in &seen { assert!(*tube > 0.4, "at {x} the place has {place:.4} W and the flood {flood:.4}: a man there is {tube:.2} to a goggled mate five metres off ({seen:.4?})"); }
    println!("{seen:.4?}");
}
