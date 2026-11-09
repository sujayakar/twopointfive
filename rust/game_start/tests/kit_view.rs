//! THE KIT'S COLUMN (`view.kit`, rust/game_rules/src/outfit/kit `kit_view`): what a HUD is told of what a man carries, held to what the game says he carries and to the keys a player presses.
use game::game::SlotKind;
use game::view::{KitSlot, KitThing, KitView};
use game::weapons::WeaponId;
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [1, 2, 3];

fn kit(p: &Play) -> KitView { p.view().kit }
fn up(k: &KitView) -> Option<KitThing> { k.slots.iter().find(|s| s.up).map(|s| s.thing) }
fn place(k: &KitView, thing: KitThing) -> &KitSlot { k.slots.iter().find(|s| s.thing == thing).expect("the strip has a place for every thing") }
/// a key pressed and let go, and the hands given time
fn tap(p: &mut Play, code: &str) { p.key_down(code); p.frame(); p.key_up(code); p.run(0.8); }
/// a key pressed and let go, one frame
fn press(p: &mut Play, code: &str) { p.key_down(code); p.frame(); p.key_up(code); p.frame(); }

/// THE RULE: the strip has a place for each thing a key brings, in the keys' order, holding what the man holds, and one of them is up.
/// A FAILURE MEANS: the column says another thing than his hands hold, or another count than he has.
#[test]
fn the_strip_says_what_he_carries_and_what_is_up() {
    for seed in SEEDS {
        let mut p = Play::level("killhouse.json", true, &Opts { seed, ..Default::default() });
        p.frame();
        let k = kit(&p);
        let keys: Vec<(KitThing, &str)> = k.slots.iter().map(|s| (s.thing, s.key.as_str())).collect();
        assert_eq!(keys, [(KitThing::Hands, "Digit0"), (KitThing::Pistol, "Digit1"), (KitThing::Rifle, "Digit2"), (KitThing::Smoke, "Digit3"), (KitThing::Flash, "Digit4"), (KitThing::Frag, "Digit5")], "seed {}", seed);
        let pl = p.g.player;
        let (rf, gun) = (p.g.pl[pl].rifle, p.g.pl[pl].pistol);
        assert_eq!(place(&k, KitThing::Rifle).count, p.g.fa[rf].chamber + p.g.fa[rf].mag.unwrap(), "seed {}", seed);
        assert_eq!(place(&k, KitThing::Pistol).count, p.g.fa[gun].chamber + p.g.fa[gun].mag.unwrap(), "seed {}", seed);
        assert_eq!(place(&k, KitThing::Pistol).name, "Glock 19", "seed {}: a gun's name is in the kit's words", seed);
        assert_eq!((place(&k, KitThing::Smoke).count, place(&k, KitThing::Flash).count, place(&k, KitThing::Frag).count), (p.g.pl[pl].canisters, p.g.pl[pl].flashbangs, p.g.pl[pl].frags), "seed {}", seed);
        // what is up follows the keys, and only they
        for (code, thing) in [("Digit1", KitThing::Pistol), ("Digit3", KitThing::Smoke), ("Digit4", KitThing::Flash), ("Digit2", KitThing::Rifle)] {
            let was = kit(&p);
            p.run(0.3);
            assert_eq!(up(&kit(&p)), up(&was), "seed {}: nothing pressed, and something else came up", seed);
            tap(&mut p, code);
            let k = kit(&p);
            assert_eq!(up(&k), Some(thing), "seed {}: {} brought another thing", seed, code);
            assert_eq!(k.gun.is_some(), matches!(thing, KitThing::Pistol | KitThing::Rifle), "seed {}: {:?} up, and the box says {:?}", seed, thing, k.gun);
            assert_eq!(k.gun.is_some_and(|g| g.ocp.is_some()), thing == KitThing::Pistol, "seed {}: the OCP is the pistol's", seed);
        }
        tap(&mut p, "Digit0");
        let k = kit(&p);
        assert!(p.g.pl[pl].holstered && up(&k) == Some(KitThing::Hands) && k.gun.is_none(), "seed {}: everything put away, and {:?} is up", seed, up(&k));
        // his condition beside them: the goggles' key, the keys he took
        let k = kit(&p);
        assert!(k.night_vision == Some(false) && k.key_tag.is_none() && !k.down && k.hits_left == k.hits_max, "seed {}: {:?}", seed, k);
        tap(&mut p, "KeyN");
        assert_eq!(kit(&p).night_vision, Some(p.g.pl[pl].nv), "seed {}", seed);
        assert!(p.g.pl[pl].nv, "seed {}: N put no goggles on", seed);
        tap(&mut p, "KeyN");
        p.g.pl[pl].keys.add(String::from(game::doors::FLOOR_KEY));
        assert_eq!(kit(&p).key_tag.as_deref(), Some("KEY"), "seed {}: the floor's key has no name to say", seed);
        // the OCP's charge, as the pistol's box shows it; god mode
        tap(&mut p, "Digit1");
        p.g.pl[pl].ocp.charge = 0.4;
        let ocp = kit(&p).gun.and_then(|g| g.ocp).expect("the pistol is up");
        assert!(ocp.charge == 0.4 && !ocp.ready, "seed {}: {:?}", seed, ocp);
        p.g.pl[pl].ocp.charge = 1.0;
        assert!(kit(&p).gun.and_then(|g| g.ocp).is_some_and(|o| o.ready), "seed {}", seed);
        let was = p.g.god_mode;
        p.g.god_mode = !was;
        assert_eq!(kit(&p).god, !was, "seed {}", seed);
        p.g.god_mode = was;
        // a can with none left, and a kit with no rifle: their places stay
        p.g.pl[pl].canisters = 0.0;
        p.g.pl[pl].has_rifle = false;
        p.g.pl[pl].holstered = false;
        p.g.pl[pl].slot = SlotKind::Rifle;
        let k = kit(&p);
        let (none, rifle, flash) = (place(&k, KitThing::Smoke), place(&k, KitThing::Rifle), place(&k, KitThing::Flash));
        assert!(none.spent && none.name == "smoke" && !flash.spent, "seed {}: {:?} {:?}", seed, none, flash);
        assert!(rifle.absent && !rifle.up && k.gun.is_none() && up(&k).is_none(), "seed {}: no rifle in this kit, and the strip says {:?} {:?}", seed, rifle, k.gun);
    }
}

/// THE RULE (the maintainer: "we should say 31/30 and then two spare magazines on the side"): the box counts the rounds in the gun over what its magazine holds, with each spare
/// magazine beside it, fullest first. A gun fed a shell at a time keeps the count of its loose shells.
/// A FAILURE MEANS: the box counts rounds the gun does not have, or a magazine that came out is lost from the row.
#[test]
fn the_box_counts_the_rounds_over_the_magazine_and_a_box_a_spare() {
    for seed in SEEDS {
        let mut p = Play::level("killhouse.json", true, &Opts { seed, ..Default::default() });
        p.sam_unhurt();
        p.sam_takes_his_rifle();
        let (pl, at) = (p.g.player, p.sam_at());
        let rf = p.g.pl[pl].rifle;
        let g0 = kit(&p).gun.expect("the rifle is up");
        let def = p.g.m.weapons.weapons.get(p.g.fa[rf].def).unwrap();
        let (cap, spares) = (def.mag_capacity, def.spare_mags as usize);
        assert!(spares > 0 && g0.capacity == cap && g0.ready == p.g.fa[rf].chamber + p.g.fa[rf].mag.unwrap() && g0.chamber && !g0.dry && !g0.reloading, "seed {}: {:?}", seed, g0);
        assert_eq!(g0.spares, vec![cap; spares], "seed {}", seed);
        assert_eq!(g0.loose, None, "seed {}", seed);
        // rounds fired come off the first number alone
        for _ in 0..4 { p.sam_fires_at([at[0] + 6.0, at[1]], 0.1); }
        let g1 = kit(&p).gun.unwrap();
        assert!(g1.ready < g0.ready && g1.ready == p.g.fa[rf].chamber + p.g.fa[rf].mag.unwrap(), "seed {}: {} fired and the box says {}", seed, g0.ready - p.g.fa[rf].chamber - p.g.fa[rf].mag.unwrap(), g1.ready);
        assert!(g1.capacity == cap && g1.spares == g0.spares, "seed {}: {:?}", seed, g1);
        // a reload: the fullest spare goes in, and the magazine that came out is a box as full as it was
        let was = p.g.fa[rf].mag.unwrap();
        press(&mut p, "KeyR");
        assert!(kit(&p).gun.unwrap().reloading, "seed {}: a reload began and the box does not say so", seed);
        p.until(10.0, |p| !kit(p).gun.unwrap().reloading).unwrap_or_else(|| panic!("seed {}: the reload never ended", seed));
        let g2 = kit(&p).gun.unwrap();
        assert_eq!(g2.spares.len(), spares, "seed {}", seed);
        assert!(g2.spares[0] == cap && g2.spares[spares - 1] == was && was < cap && g2.mag == cap, "seed {}: {} came out and the box says {:?}", seed, was, g2);
        // CONTROL on `dry`, the words of the launcher's load, and the light
        p.g.fa[rf].chamber = 0.0; p.g.fa[rf].mag = Some(0.0);
        p.g.fa[rf].rings = 0.0; p.g.fa[rf].att_idx = 0; p.g.fa[rf].light_on = true;
        let g3 = kit(&p).gun.unwrap();
        assert!(g3.dry && !g3.chamber && g3.ready == 0.0 && g3.light, "seed {}: {:?}", seed, g3);
        let load = g3.load.expect("the launcher has a load up");
        assert!(load.text == "SPONGE ×0" && load.out, "seed {}: {:?}", seed, load);
        assert!(matches!(g0.selector.as_deref(), Some("SEMI" | "AUTO")), "seed {}: a gun with two positions says which: {:?}", seed, g0.selector);
        // a magazine gun whose row never runs dry has no boxes and no loose shells
        let def = p.g.fa[rf].def;
        let mut row = p.g.m.weapons.weapons.get(def).unwrap().clone();
        let spare = row.spare_mags;
        row.spare_mags = f64::INFINITY;
        p.g.m.weapons.weapons.set(def, row.clone());
        let g5 = kit(&p).gun.unwrap();
        assert!(g5.spares.is_empty() && g5.loose.is_none(), "seed {}: {:?}", seed, g5);
        row.spare_mags = spare;
        p.g.m.weapons.weapons.set(def, row);
        // a gun fed a shell at a time
        p.g.fa[rf].def = WeaponId::M1014;
        p.g.fa[rf].spare = vec![5.0, 7.0];
        let g4 = kit(&p).gun.unwrap();
        assert!(g4.loose == Some(12.0) && g4.spares.is_empty() && g4.capacity == 7.0 && g4.selector.is_none(), "seed {}: {:?}", seed, g4);
    }
}

/// THE RULE: the digit of the gun in his hands opens its setup: the column is told its rows and which places their digits take for now, and what the setup would take. A pick shuts it.
/// A FAILURE MEANS: the column shows a setup the game has shut (or none where it is open), or the page lets a digit of the setup go as a move of the cursor.
#[test]
fn the_setup_is_told_while_it_is_open_and_only_then() {
    for seed in SEEDS {
        let mut p = Play::level("killhouse.json", true, &Opts { seed, ..Default::default() });
        p.sam_takes_his_rifle();
        let (pl, rf) = (p.g.player, p.g.pl[p.g.player].rifle);
        if !p.g.fa[rf].attachments.contains(&game::weapons::RifleAttachment::He40) { p.g.fa[rf].attachments.push(game::weapons::RifleAttachment::He40); }
        p.g.fa[rf].shells = 2.0;
        p.g.fa[rf].rings = 3.0;
        p.run(0.8);   // (a gun just come to his hands does not open its setup)
        let k = kit(&p);
        assert!(k.setup.is_none() && !k.slots.iter().any(|s| s.taken), "seed {}: shut, and told {:?}", seed, k.setup);
        press(&mut p, "Digit2");
        let k = kit(&p);
        let s = k.setup.as_ref().unwrap_or_else(|| panic!("seed {}: its gun's digit again and no setup", seed));
        let keys: Vec<&str> = s.rows.iter().map(|r| r.key.as_str()).collect();
        assert_eq!(keys, ["Digit1", "Digit2", "Digit3"], "seed {}: {:?}", seed, s);
        assert_eq!(s.back, "TAB", "seed {}", seed);
        let on: Vec<&str> = s.rows[0].words.iter().filter(|w| w.on).map(|w| w.text.as_str()).collect();
        assert_eq!(on, [p.g.fa[rf].mode.as_str().to_uppercase().as_str()], "seed {}: {:?}", seed, s.rows[0]);
        let load: Vec<&str> = s.rows[1].words.iter().map(|w| w.text.as_str()).collect();
        assert_eq!(load, ["SPONGE ×3", "HE ×2"], "seed {}", seed);
        assert_eq!(s.rows[2].words.len(), 1, "seed {}: the light is one word: {:?}", seed, s.rows[2]);
        assert_eq!(s.rows[2].words[0].text, "LIGHT", "seed {}", seed);
        assert_eq!(s.rows[2].words[0].on, p.g.fa[rf].light_on, "seed {}", seed);
        for (t, taken) in [(KitThing::Pistol, true), (KitThing::Rifle, false), (KitThing::Smoke, true), (KitThing::Flash, false)] { assert_eq!(place(&k, t).taken, taken, "seed {}: {:?}", seed, t); }
        // a pick steps its row and shuts it
        let mode = p.g.fa[rf].mode;
        press(&mut p, "Digit1");
        let k = kit(&p);
        assert!(p.g.fa[rf].mode != mode && k.setup.is_none() && !k.slots.iter().any(|s| s.taken) && up(&k) == Some(KitThing::Rifle), "seed {}: {:?}", seed, k.setup);
        // CONTROL: a can has nothing to set, and its digit is the slot it always was
        tap(&mut p, "Digit3");
        let k = kit(&p);
        assert!(k.setup.is_none() && p.g.pl[pl].slot == SlotKind::Smoke, "seed {}: {:?}", seed, k.setup);
        // the pistol has its own
        tap(&mut p, "Digit1");
        press(&mut p, "Digit1");
        let k = kit(&p);
        assert_eq!(k.setup.expect("the pistol's digit again").rows.last().map(|r| r.key.clone()), Some(String::from("Digit3")), "seed {}", seed);
    }
}

/// THE RULE: the load row's digit puts up the next load he has any of: one he is out of is stepped over, and with nothing else to put up the row is not told.
/// A FAILURE MEANS: the setup offers a load the launcher cannot step to, or the box names another load than the launcher has up.
#[test]
fn a_load_he_is_out_of_is_stepped_over() {
    use game::weapons::RifleAttachment as A;
    for seed in SEEDS {
        let mut p = Play::level("killhouse.json", true, &Opts { seed, ..Default::default() });
        p.sam_takes_his_rifle();
        let rf = p.g.pl[p.g.player].rifle;
        p.g.fa[rf].attachments = vec![A::Ring, A::He40, A::Cam];
        p.g.fa[rf].att_idx = 0;
        (p.g.fa[rf].rings, p.g.fa[rf].shells, p.g.fa[rf].cams) = (3.0, 0.0, 2.0);
        p.run(0.8);
        let rows = |p: &Play| kit(p).setup.map_or(Vec::new(), |s| s.rows.iter().map(|r| r.key.clone()).collect::<Vec<_>>());
        // ring up, no grenade, two cameras behind it
        press(&mut p, "Digit2");
        assert_eq!(rows(&p), ["Digit1", "Digit2", "Digit3"], "seed {}", seed);
        let load: Vec<(String, bool, bool)> = kit(&p).setup.unwrap().rows[1].words.iter().map(|w| (w.text.clone(), w.on, w.out)).collect();
        assert_eq!(load, [(String::from("SPONGE ×3"), true, false), (String::from("HE ×0"), false, true), (String::from("CAM ×2"), false, false)], "seed {}", seed);
        press(&mut p, "Digit2");
        assert_eq!(kit(&p).gun.unwrap().load.map(|w| w.text), Some(String::from("CAM ×2")), "seed {}: 2 2 stepped over the grenade he has none of", seed);
        // nothing else to put up: no row, and 2 2 only shuts it
        p.g.fa[rf].rings = 0.0;
        press(&mut p, "Digit2");
        assert_eq!(rows(&p), ["Digit1", "Digit3"], "seed {}", seed);
        press(&mut p, "Digit2");
        assert!(kit(&p).setup.is_none() && kit(&p).gun.unwrap().load.map(|w| w.text) == Some(String::from("CAM ×2")), "seed {}", seed);
    }
}

/// THE RULE: a card that leaves him the light says so in the game from the start: its row is in the setup and L works it. A card with none takes the row away, L does nothing, and the box
/// never says a light is on. (On the page the box used to ask a TypeScript map.)
/// A FAILURE MEANS: the HUD offers a lamp the card did not give him, or the game lets him work one.
#[test]
fn a_card_without_the_light_has_no_lamp_to_work() {
    for seed in SEEDS {
        let mut p = Play::level("warehouse.json", false, &Opts { seed, ..Default::default() });
        p.sam_unhurt();
        p.sam_takes_his_rifle();
        p.run(0.8);
        let pl = p.g.player;
        // CONTROL: the issued card has one
        press(&mut p, "Digit2");
        assert!(kit(&p).setup.is_some_and(|s| s.rows.iter().any(|r| r.key == "Digit3")), "seed {}: the issued card's setup has no light", seed);
        press(&mut p, "Tab");
        press(&mut p, "KeyL");
        let rf = p.g.pl[pl].rifle;
        assert!(p.g.fa[rf].light_on && kit(&p).gun.unwrap().light, "seed {}: L worked no light on the issued card", seed);
        // the plain night wears the stealth card: the loadout screen up for this mission, its light taken off, the screen shut and the night stood again
        let hand = game::kit::hand_of(&p.g);
        assert!(hand.is_some(), "seed {}: the staging is wrong: this night is no stealth run", seed);
        game::kit::kit_mission(&mut p.g, hand);
        assert!(game::kit::kit_set(&mut p.g, 0.0, game::kit::KitField::Light, &game::kit::KitValue::Bool(false)), "seed {}: the card took no such row", seed);
        game::kit::kit_mission(&mut p.g, None);
        game::game::game_restart_encounter(&mut p.g);
        p.frame();
        p.sam_takes_his_rifle();
        p.run(0.8);
        let (pl, rf) = (p.g.player, p.g.pl[p.g.player].rifle);
        assert!(!game::kit::kit_you_light(&p.g, pl) && !p.g.fa[rf].light_on, "seed {}: the card gave up its light and the game still has it", seed);
        press(&mut p, "KeyL");
        assert!(!p.g.fa[rf].light_on && !kit(&p).gun.unwrap().light, "seed {}: L worked a lamp the card did not give him", seed);
        press(&mut p, "Digit2");
        let rows: Vec<String> = kit(&p).setup.map_or(Vec::new(), |s| s.rows.iter().map(|r| r.key.clone()).collect());
        assert!(!rows.iter().any(|r| r == "Digit3") && !rows.is_empty(), "seed {}: the setup offers the light: {:?}", seed, rows);
    }
}
/// THE SAME OF A SQUAD'S MISSION (the room clear), where the loud card is what he wears.
#[test]
fn a_card_without_the_light_has_none_in_a_room_clear() {
    let mut p = Play::level("kit-yard.json", false, &Opts::default());
    p.sam_unhurt();
    assert!(game::kit::kit_set(&mut p.g, 0.0, game::kit::KitField::Light, &game::kit::KitValue::Bool(false)), "the card took no such row");
    let spec = game::scene_scenario_list::ScenarioSpec { kind: game::scene_scenario_list::ScenarioSpecKind::Roomclear, squad: 4.0, level: None, seed: None };
    if let Some(why) = game::scenario_run::start_scenario(&mut p.g, &spec) { panic!("refused: {}", why); }
    p.frame();
    let pl = p.g.player;
    assert!(!game::kit::kit_you_light(&p.g, pl), "the card gave up its light and the mission still has it");
    p.sam_takes_his_rifle();
    p.run(0.8);
    let rf = p.g.pl[pl].rifle;
    press(&mut p, "KeyL");
    assert!(!p.g.fa[rf].light_on, "L worked a lamp the card did not give him");
    assert!(!kit(&p).gun.unwrap().light);
}

/// THE RULE: the big slot has a row while he carries one, and it lights while a door in reach offers it.
/// A FAILURE MEANS: the column says a tool he has not got, or does not say the door will take it.
#[test]
fn the_big_slot_is_told_with_the_door_that_offers_it() {
    for seed in SEEDS {
        let mut p = Play::level("killhouse.json", true, &Opts { seed, ..Default::default() });
        p.frame();
        let pl = p.g.player;
        let far = p.sam_at();
        p.g.pl[pl].big_kit = game::kit::BigKit::None;
        p.frame();
        assert!(kit(&p).big.is_none(), "seed {}: {:?}", seed, kit(&p).big);
        p.g.pl[pl].big_kit = game::kit::BigKit::Hammer;
        p.frame();
        let b = kit(&p).big.expect("a hammer is on him");
        assert!(b.name == "hammer" && b.place == "at a shut door", "seed {}: {:?}", seed, b);
        let door = shut_door_beside(&mut p);
        let b = kit(&p).big.unwrap();
        assert!(b.offered, "seed {}: beside the shut door at {:?}, and nothing is offered: {:?}", seed, door, p.g.nearby.iter().map(|r| r.verb).collect::<Vec<_>>());
        p.sam_put(far); p.run(0.3);
        assert!(!kit(&p).big.unwrap().offered, "seed {}: no door by him, and the row is lit", seed);
        // a tour shows no kit of its own, and a game with no big slot has no row
        p.g.touring = true;
        assert!(kit(&p).big.is_none(), "seed {}", seed);
        p.g.touring = false;
        p.g.k.bigslot.on = false;
        assert!(kit(&p).big.is_none(), "seed {}", seed);
    }
}
/// Sam stands beside the first shut door that offers his tool; where it stands
fn shut_door_beside(p: &mut Play) -> [f64; 2] {
    let doors = p.g.doors.list.clone();
    for d in doors {
        let c = p.g.dr[d].frame_centre;
        for (dx, dz) in [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)] {
            p.sam_put([c[0] + dx, c[2] + dz]);
            p.run(0.1);
            if p.g.nearby.iter().any(|r| matches!(r.verb, Some(game::game::Verb::Hammer | game::game::Verb::Breach))) { return [c[0], c[2]]; }
        }
    }
    panic!("no door on this level offers his tool");
}

/// THE RULE: in a game of two each column is about its own man: the second player's strip, box and setup are told from HIS hands, whatever the first's are doing.
/// A FAILURE MEANS: a player's HUD shows the other man's kit.
#[test]
fn in_a_game_of_two_each_column_is_its_own_mans() {
    for seed in SEEDS {
        let mut p = Play::level("defuse.json", true, &Opts { seed, seats: 2, ..Default::default() });
        p.net_frame(0);
        let (a, b) = (p.g.seats[0], p.g.seats[1]);
        let tap_seat = |p: &mut Play, seat: game::ids::SeatId, code: &str, frames: usize| {
            let input = p.g.tab.cmd_seat[seat].input;
            p.g.tab.input[input].keys.add(String::from(code)); p.g.tab.input[input].pressed.add(String::from(code));
            p.net_frame(0);
            p.g.tab.input[input].keys.delete(String::from(code)); p.g.tab.input[input].pressed.delete(String::from(code));
            for _ in 0..frames { p.net_frame(0); }
        };
        // CONTROL: both as they were dealt, and one and the same
        let (ka, kb) = (p.view_of(0).kit, p.view_of(1).kit);
        assert_eq!(up(&ka), up(&kb), "seed {}", seed);
        // the second man's hands
        tap_seat(&mut p, b, "Digit3", 60);
        let (ka, kb) = (p.view_of(0).kit, p.view_of(1).kit);
        assert_eq!(up(&kb), Some(KitThing::Smoke), "seed {}: the second man took his smoke", seed);
        assert!(up(&ka) != Some(KitThing::Smoke) && ka.gun.is_some() && kb.gun.is_none(), "seed {}: the first man was told {:?}", seed, up(&ka));
        // his setup, and the first man's digits
        tap_seat(&mut p, b, "Digit1", 150);
        tap_seat(&mut p, b, "Digit1", 2);
        let (ka, kb) = (p.view_of(0).kit, p.view_of(1).kit);
        assert!(kb.setup.is_some() && ka.setup.is_none(), "seed {}: the second man opened his setup: {:?} {:?}", seed, ka.setup, kb.setup);
        // what he holds and what he took
        let (pa, pb) = (game::seats::seat(&p.g, a).player, game::seats::seat(&p.g, b).player);
        // the gun in HIS hands, his goggles
        tap_seat(&mut p, b, "Tab", 2);
        let (ga, gb) = (p.view_of(0).kit.gun.expect("the first man's gun is up"), p.view_of(1).kit.gun.expect("the second man's pistol is up"));
        assert!(gb.ocp.is_some() && ga.ocp.is_none() && gb.capacity != ga.capacity, "seed {}: the box of the second man's pistol: {:?} beside the first man's {:?}", seed, gb, ga);
        p.g.pl[pb].nv = true; p.g.pl[pa].nv = false;
        assert_eq!((p.view_of(0).kit.night_vision, p.view_of(1).kit.night_vision), (Some(false), Some(true)), "seed {}", seed);
        p.g.pl[pb].frags = 1.0; p.g.pl[pa].frags = 3.0;
        p.g.pl[pb].keys.add(String::from("vault"));
        let (ka, kb) = (p.view_of(0).kit, p.view_of(1).kit);
        assert_eq!((place(&ka, KitThing::Frag).count, place(&kb, KitThing::Frag).count), (3.0, 1.0), "seed {}", seed);
        assert_eq!((ka.key_tag.as_deref(), kb.key_tag.as_deref()), (None, Some("KEY · VAULT")), "seed {}", seed);
        p.seat_goes_down(1);
        let (ka, kb) = (p.view_of(0).kit, p.view_of(1).kit);
        assert!(kb.down && !ka.down, "seed {}: the second man went down", seed);
    }
}
