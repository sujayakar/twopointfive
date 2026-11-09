//! A DRY GUN STARTS ITS OWN RELOAD, UNLIMITED AMMO COVERS WALL CHARGES. DEBRIS IS HEARD WHERE IT LANDS, A BOOT'S SHOVE FILES NOTHING.
mod world_ask;
use game::director::EventKind;
use game::ids::FirearmId;
use game_start::play::{dist, Button, Opts, Play};
use world_ask::{WallAsk, YardAsk};

fn alone(s: u32) -> Play {
    let mut p = Play::level("office.json", false, &Opts { seed: s, ..Default::default() });
    p.g.god_mode = true; p.g.player_invisible = true; p.g.ai_enabled = false;
    p.run(0.1);
    let men = p.g.guards.clone();
    for (k, &m) in men.iter().enumerate() { p.parked_far(m, k); }
    p.sam_put([20.0, 11.3]);
    p.point_at([27.5, 11.5]);
    p
}
fn kit(p: &Play, f: FirearmId) -> (f64, f64, Vec<f64>) { let a = &p.g.fa[f]; (a.chamber, a.mag.unwrap_or(0.0), a.spare.clone()) }
fn fired_dry(p: &mut Play, f: FirearmId) { for _ in 0..200 { if p.g.fa[f].chamber == 0.0 && p.g.fa[f].mag.unwrap_or(0.0) == 0.0 { return; } p.press(Button::Left); p.frame(); p.release(Button::Left); p.frame(); } panic!("the gun will not go dry: {:?}", kit(p, f)); }

/// THE RULE: the rifle fired dry with a magazine left: no R pressed, its reload starts a beat after the last round, the fullest spare goes in, a round is racked, and no line asks for R. CONTROL: a magazine only part spent is left alone, R is still the player's; and with nothing to seat there is no reload. The pistol the same.
/// A FAILURE MEANS: the player stands clicking an empty gun.
#[test]
fn a_dry_gun_starts_its_own_reload() {
    let mut p = alone(1);
    let (rifle, pistol) = (p.g.pl[p.g.player].rifle, p.g.pl[p.g.player].pistol);
    p.sam_takes_his_rifle();
    p.run(0.6);
    (p.g.fa[rifle].chamber, p.g.fa[rifle].mag, p.g.fa[rifle].spare) = (1.0, Some(2.0), vec![30.0, 25.0]);
    fired_dry(&mut p, rifle);
    p.until(0.5, |p| game::weapons::firearm_reloading(&p.g, rifle)).expect("the rifle went dry and no reload began");
    p.until(4.0, |p| !game::weapons::firearm_reloading(&p.g, rifle)).expect("the reload never ended");
    assert_eq!(kit(&p, rifle), (1.0, 29.0, vec![25.0]), "after the reload");

    (p.g.fa[rifle].chamber, p.g.fa[rifle].mag, p.g.fa[rifle].spare) = (1.0, Some(5.0), vec![30.0]);
    p.run(1.5);
    assert!(!game::weapons::firearm_reloading(&p.g, rifle), "a part-spent magazine was reloaded for him");
    (p.g.fa[rifle].chamber, p.g.fa[rifle].mag, p.g.fa[rifle].spare) = (1.0, Some(1.0), vec![]);
    fired_dry(&mut p, rifle);
    p.run(1.0);
    assert!(!game::weapons::firearm_reloading(&p.g, rifle), "nothing to seat and a reload began");

    p.key_down("Digit1"); p.frame(); p.key_up("Digit1");
    p.run(0.6);
    (p.g.fa[pistol].chamber, p.g.fa[pistol].mag, p.g.fa[pistol].spare) = (1.0, Some(1.0), vec![12.0]);
    fired_dry(&mut p, pistol);
    p.until(1.0, |p| game::weapons::firearm_reloading(&p.g, pistol)).expect("the pistol went dry and no reload began");
}

/// THE RULE (the maintainer, 10-14: "unlimited ammo doesn't apply to wall charges"): with unlimited ammo on, wall charges set by his hands come out of no count, and one off the wall adds none. CONTROL: the switch off: one set is one spent. None carried is still none: the switch makes what he has bottomless, it issues nothing.
/// A FAILURE MEANS: the sandbox's row lies, or hands him charges he never had.
#[test]
fn unlimited_ammo_covers_wall_charges() {
    use game::wallcharge::{wall_charge_set_carried, wall_charge_take};
    let mut p = Play::mission("mousehole.json", "?levelFile=mousehole.json&roomclear&squad4", &Opts { seed: 1, ..Default::default() });
    p.g.ai_enabled = false; p.g.god_mode = true; p.g.tune.chaos = 0.5;
    p.run(0.1);
    wall_charge_set_carried(&mut p.g, Some(2.0));
    p.g.infinite_ammo = true;
    p.sam_sets_charge([12.7, 0.0, 7.0], [12.7, 6.0]).expect("Sam sets a charge");
    assert_eq!(p.g.wall_charge.carried, Some(2.0), "the count moved under the switch");
    p.g.infinite_ammo = false;
    p.sam_sets_charge([14.7, 0.0, 7.0], [14.7, 6.0]).expect("Sam sets a charge");
    assert_eq!(p.g.wall_charge.carried, Some(1.0), "with the switch off one set is not one spent");
    wall_charge_set_carried(&mut p.g, Some(0.0));
    p.g.infinite_ammo = true;
    assert_eq!(wall_charge_take(&mut p.g), -1.0, "the switch issued a charge he never had");
}

/// the prop noises filed from now on
fn props_heard(p: &mut Play, secs: f64) -> Vec<f64> {
    let mut seen = Vec::new();
    let (mut last, mut out) = (p.g.time, Vec::new());
    for _ in 0..(secs * 60.0) as usize {
        p.frame();
        for e in &p.g.events { if e.time >= last && e.kind == EventKind::Prop { let k = (e.time, e.level.unwrap_or(0.0).to_bits()); if !seen.contains(&k) { seen.push(k); out.push(e.level.unwrap_or(0.0)); } } }
        last = p.g.time;
    }
    out
}

/// THE RULE: a ceiling tile landing files one noise, and a calm man two metres off goes suspicious over it. CONTROL: the same man beside a stack of paper scattered and dust raised hears nothing: nothing landed.
/// A FAILURE MEANS: men answer to dust, or a falling ceiling is silent.
#[test]
fn debris_is_heard_where_it_lands() {
    for s in [3402u32, 3403] {
        let mut p = alone(s);
        p.g.ai_enabled = true; p.g.tune.chaos = 0.5;
        let man = p.g.guards[1];
        p.posted_calm(man, [21.6, 17.2], [21.6, 14.0]);
        p.run(0.5);
        let paper = p.g.destr.items.values().find(|d| d.kind == game::destruction::DKind::Paper && { let c = p.g.tab.boxes[d.box_].c; (c[0] - 21.6).hypot(c[2] - 17.2) < 6.0 }).cloned().expect("a stack of paper near him");
        game::destruction::destruction_convert_paper(&mut p.g, &paper, [0.0, 1.0, 0.0], 2.0, 0.5);
        for _ in 0..3 { game::destruction::destruction_dust_puff(&mut p.g, [21.6, 1.2, 15.5], [0.0, 0.0, 1.0], 1.0, game::scene_materials::Surf::Drywall); }
        assert!(props_heard(&mut p, 2.0).is_empty(), "seed {s}: paper and dust filed a noise");
        assert!(p.awareness(man) < 0.05, "seed {s}: paper and dust moved him: {}", p.awareness(man));
        assert!(game::destruction::destruction_tile_drop(&mut p.g, [20.2, 0.1, 15.4], 0.5, Some(true), None, None), "seed {s}: the tile drop was refused");
        let heard = props_heard(&mut p, 2.5);
        assert_eq!(heard.len(), 1, "seed {s}: noises from one tile: {heard:?}");
        assert!(p.awareness(man) > 0.2 || p.state(man) != game::game::GuardState::Patrol, "seed {s}: a tile landing two metres off left him at {}", p.awareness(man));
    }
}

/// A lump or a plank in a level's corridor, let fall where it lies.
fn litter(p: &mut Play, at: [f64; 3], h: [f64; 3], yaw: f64) -> game::ids::DebrisChunkId {
    let b = game::scene_boxes::make_box(&mut p.g, &game::scene_boxes::MakeBoxP { c: at, h, yaw: Some(yaw), ..game::scene_boxes::MakeBoxP::default() });
    game::debris::debris_spawn_raw(&mut p.g, game::debris::ChunkKind::Chunk, b, [0.0; 3], [0.0; 3], None, None).expect("a piece")
}
fn place_of(p: &Play, c: game::ids::DebrisChunkId) -> [f64; 3] { p.g.tab[p.g.tab.debris_chunk[c].box_].c }

/// THE RULE: Sam walks through pieces lying in his path: they are shoved aside and come to rest, and nothing is filed for it: no noise but his own steps, no landing counted, nothing told to the break table. CONTROL: a piece well off his path keeps its place to the bit.
/// A FAILURE MEANS: a walk through rubble alerts the floor, or rubble is a floor texture feet pass through.
#[test]
fn a_boots_shove_moves_the_pieces_and_files_nothing() {
    for s in [7107u32, 7108] {
        let mut p = Play::level("office.json", false, &Opts { seed: s, dress: true, ..Default::default() });
        p.g.god_mode = true; p.g.player_invisible = true; p.g.ai_enabled = false;
        p.run(0.1);
        let men = p.g.guards.clone();
        for (k, &m) in men.iter().enumerate() { p.parked_far(m, k); }
        p.sam_put([15.0, 11.3]);
        let path: Vec<_> = (0..4).map(|i| litter(&mut p, [16.5 + i as f64 * 0.6, 0.25, 11.5], if i % 2 == 0 { [0.09, 0.05, 0.07] } else { [0.3, 0.012, 0.06] }, i as f64 * 0.7)).collect();
        let aside = litter(&mut p, [13.0, 0.25, 11.5], [0.09, 0.05, 0.07], 0.4);
        p.run(2.0);
        let before: Vec<_> = path.iter().map(|&c| place_of(&p, c)).collect();
        let aside0 = place_of(&p, aside);
        let stats0 = (p.g.destr.debris.stats.spawned, p.g.destr.debris.stats.landed, p.g.destr.debris.stats.told);
        let mut noises = 0;
        let mut last = p.g.time;
        p.key_down("KeyD");
        for _ in 0..220 { p.frame(); noises += p.g.events.iter().filter(|e| e.time >= last && e.kind != EventKind::Step).count(); last = p.g.time; }
        p.key_up("KeyD");
        p.run(1.0);
        assert!(p.sam_at()[0] > 18.5, "seed {s}: he has not walked through: {:?}", p.sam_at());
        let moved = path.iter().zip(&before).filter(|(&c, b)| { let q = place_of(&p, c); dist([q[0], q[2]], [b[0], b[2]]) > 0.03 }).count();
        assert!(moved >= 2, "seed {s}: {moved} of 4 pieces shoved");
        assert_eq!(place_of(&p, aside), aside0, "seed {s}: the piece off his path moved");
        assert_eq!(noises, 0, "seed {s}: noises filed");
        assert_eq!((p.g.destr.debris.stats.spawned, p.g.destr.debris.stats.landed, p.g.destr.debris.stats.told), stats0, "seed {s}: spawned, landed, told");
    }
}
