//! WHAT A LEVEL SAYS OF ITSELF, held in a played game: its own wall charges, its own cards, floor behind furniture, its own roofs, its own ways.
use game::game::Game;
use game_start::json::{self, Json};
use game_start::play::{Opts, Play};

/// a level's file changed for a test, written where a game can be stood from it
fn changed(name: &str, what: &str, how: &dyn Fn(&mut Json)) -> std::path::PathBuf {
    let mut level = json::parse(&game_start::files::level_text(name)).unwrap();
    how(&mut level);
    let path = std::env::temp_dir().join(format!("{}-{what}-{name}", std::process::id()));
    std::fs::write(&path, json::to_text(&level)).unwrap();
    path
}
fn member<'a>(j: &'a mut Json, k: &str) -> &'a mut Json { let Json::Obj(m, _) = j else { panic!("not an object: {k}") }; &mut m.iter_mut().find(|(n, _)| n == k).unwrap_or_else(|| panic!("no {k}")).1 }
fn carried(p: &Play) -> f64 { p.g.wall_charge.carried.unwrap_or(0.0) }

/// THE RULE (the clinic, whose one idea is that the walls are the way in): a room clear whose structure issues wall charges has that many at least in the bag, again after a restart with none
/// spent and after one with some; it raises a kit's count and never lowers one. A level that issues none keeps what the kit gave.
/// A FAILURE MEANS: a level built to be entered through its walls is played with no charge to open one.
#[test]
fn a_level_issues_its_own_wall_charges() {
    let n = json::parse(&game_start::files::level_text("mousehole.json")).unwrap().get("scenarios").get("roomClear").get("wallCharges").num().expect("the clinic issues wall charges");
    let mut p = Play::mission("mousehole.json", "?levelFile=mousehole.json&roomclear&squad4", &Opts::default());
    assert_eq!(carried(&p), n, "staged");
    game::game::game_restart_encounter(&mut p.g);
    assert_eq!(carried(&p), n, "restarted with none spent");
    p.g.wall_charge.carried = Some(1.0);
    game::game::game_restart_encounter(&mut p.g);
    assert_eq!(carried(&p), n, "restarted with some spent");
    game::wallcharge::wall_charge_raise_carried(&mut p.g, n + 4.0);
    game::issue::issue_stage(&mut p.g);
    assert_eq!(carried(&p), n + 4.0, "a larger count stands");
    drop(p);
    let mut p = Play::mission("block.json", "?levelFile=block.json&roomclear&squad4", &Opts::default());
    let was = p.g.wall_charge.carried;
    assert_eq!(game::issue::issue_stage(&mut p.g), None, "a level with no issue");
    assert_eq!(p.g.wall_charge.carried, was);
}

/// THE RULE: a level's own cards (`scenarios.issue`) are what its men wear, whatever kit this machine keeps, at every size, and again after a restart: the first teammate's written card on him, the plain
/// issue on the others. A level with no block wears what is kept.
/// A FAILURE MEANS: a mission built round a kit is played with another one, or a kept kit is lost to it.
#[test]
fn a_levels_own_cards_are_worn_over_the_kit_this_machine_keeps() {
    let pistol = changed("hall.json", "pistol", &|l| { let issue = member(member(l, "scenarios"), "issue"); *issue = json::obj(vec![("members", json::obj(vec![("1", json::obj(vec![("weapon", json::s("pistol")), ("charges", json::n(0.0)), ("frags", json::n(0.0))]))]))]); });
    let bare = changed("hall.json", "bare", &|l| { let Json::Obj(s, _) = member(l, "scenarios") else { panic!() }; s.retain(|(k, _)| k != "issue"); });
    // what each teammate's hands hold first, and the bags' counts, as the men stand
    let worn = |p: &Play| -> Vec<String> { p.mates().iter().map(|&m| format!("{:?}", p.g.fa[p.g.gd[m].weapon].def)).collect() };
    let at = |file: &std::path::Path, squad: &str| -> Vec<String> { let mut p = Play::mission(file.to_str().unwrap(), &format!("?roomclear&{squad}"), &Opts::default()); let a = worn(&p); game::game::game_restart_encounter(&mut p.g); assert_eq!(worn(&p), a, "{squad}: a restart stages the same cards"); a };
    let plain = at(&bare, "squad4");
    assert!(plain.iter().all(|w| *w == plain[0]), "the plain issue is one gun: {plain:?}");
    for (squad, n) in [("squad4", 4), ("squad=3", 3), ("squad=2", 2), ("squad=1", 1)] {
        let w = at(&pistol, squad);
        assert_eq!(w.len(), n, "{squad}");
        assert_ne!(w[0], plain[0], "{squad}: the named card is on the first man");
        assert!(w[1..].iter().all(|x| *x == plain[0]), "{squad}: and on nobody else: {w:?}");
    }
}

/// THE RULE (the maintainer on the barracks: "weird that it doesn't let me use a wall charge here", two cots stood against the wall's far side): where the walk grid has no floor behind a place, floor that
/// only the piece this charge breaks first stands on counts; and it is a promise: blown, the wall is holed there and the plain walk grid goes through.
/// A FAILURE MEANS: a charge is refused at a wall a man can see through to, or is spent on a hole nobody can walk through.
#[test]
fn a_charge_taken_where_furniture_stands_behind_opens_a_way_through() {
    let stand = |level: &str| -> Play { let mut p = Play::mission(&format!("{level}.json"), &format!("?levelFile={level}.json&roomclear&squad4"), &Opts { seed: 3, ..Default::default() }); p.sam_unhurt(); p.g.ai_enabled = false; p.run(0.5); p };
    for level in ["barracks", "depot"] {
        let mut p = stand(level);
        let added = places(&mut p.g, &|g, w| game::wallcharge::wall_charge_far_floor(g, w, None).is_some() && game::wallcharge::wall_charge_far_floor(g, w, Some(false)).is_none());
        assert!(added.len() >= 3, "{level}: {} places the rule adds", added.len());
        drop(p);
        for w in added.iter().step_by((added.len() / 3).max(1)).take(3) {
            let mut p = stand(level);
            let spot = game::wallcharge::wall_charge_set_spot(&mut p.g, w).expect("a place to set it from");
            let t = { let b = &p.g.tab.boxes[w.box_]; 2.0 * b.h[0].min(b.h[2]) };
            let beyond = game::wallcharge::wall_pt(&p.g, w, 0.0, -(t + 1.0), None);
            let breach = if w.outer == Some(true) && game::wallbreak::wall_break_adopt(&mut p.g, w.index) { Some(w.index) } else { None };
            let hung = Some(game::wallbreak::BlastOptsHung { wall: w.index, r: p.g.k.wallcharge.hung_m });
            let pt = game::wallcharge::wall_pt(&p.g, w, 0.0, 0.02, Some(1.0));
            game::destruction::destruction_blast(&mut p.g, pt, Some(game::wallbreak::BlastOpts { who: Some(1.0), chaos: Some(false), breach, hung, ..Default::default() }));
            p.run(1.5);
            let to = game::scene_collision::nav_grid_nearest_free_point(&mut p.g, beyond[0], beyond[2]).filter(|f| (f[0] - beyond[0]).hypot(f[2] - beyond[2]) < 1.2);
            let to = to.unwrap_or_else(|| panic!("{level}: no floor behind the hole at {:.1}, {:.1}", beyond[0], beyond[2]));
            let way = game::scene_collision::nav_grid_find_path(&mut p.g, spot, to, None, None).unwrap_or_else(|| panic!("{level}: no way through the hole at {:.1}, {:.1}", beyond[0], beyond[2]));
            let mut len = 0.0;
            let mut a = spot;
            for b in way { len += (b[0] - a[0]).hypot(b[2] - a[2]); a = b; }
            assert!(len < 8.0, "{level}: {len:.1} m round to the other side at {:.1}, {:.1}", beyond[0], beyond[2]);
        }
    }
}
/// the level's places for a wall charge a rule picks, each once
fn places(g: &mut Game, pick: &dyn Fn(&mut Game, &game::wallcharge::WallSpot) -> bool) -> Vec<game::wallcharge::WallSpot> {
    let mut out: Vec<game::wallcharge::WallSpot> = vec![];
    for bi in 0..(g.destr.walls.authored as usize).min(g.col.boxes.len()) {
        let (c, h) = { let b = &g.tab.boxes[g.col.boxes[bi]]; (b.c, b.h) };
        if h[1] < 0.9 || h[0].min(h[2]) > 0.2 || h[0].max(h[2]) < 0.75 { continue; }
        let along_x = h[0] >= h[2];
        let l = if along_x { h[0] } else { h[2] };
        let mut u = -l + 0.1;
        while u <= l - 0.1 + 1e-6 {
            for side in [1.0, -1.0] {
                let at = if along_x { [c[0] + u, 0.0, c[2]] } else { [c[0], 0.0, c[2] + u] };
                let from = if along_x { [at[0], 0.0, c[2] + side * 2.0] } else { [c[0] + side * 2.0, 0.0, at[2]] };
                let Some(w) = game::wallcharge::wall_at(g, at, Some(from)).filter(|w| w.ok && w.index as usize == bi) else { continue };
                let w = game::wallcharge::wall_charge_settle(g, &w);
                if game::wallcharge::wall_charge_set_spot(g, &w).is_none() || game::wallcharge::wall_charge_open_at(g, &w) || !pick(g, &w) { continue; }
                if !out.iter().any(|x| x.index == w.index && x.side == w.side && (x.u - w.u).abs() < 5e-4) { out.push(w); }
            }
            u += 0.5;
        }
    }
    out
}

/// THE RULE (the maintainer: "rain is busted on records office"): a level's storm is cut by its own roofs. A man on the open car park is soaked and one under the building's roof stays dry; an outside door
/// blows in where its level's roofs say it is outside; the office and the level in its shell keep the office's two; a quiet night stays dry and still; a picked weather takes over and hands back.
/// A FAILURE MEANS: it rains indoors, or not on a yard, or a storm's door hangs still.
#[test]
fn a_storm_is_cut_by_the_levels_own_roofs() {
    let o = Opts { dress: true, ..Default::default() };
    let mut p = Play::level("records.json", false, &o);
    p.sam_unhurt();
    p.g.ai_enabled = false;
    p.run(0.1);
    assert!(p.g.weather.own, "the records office has a storm of its own");
    let soak = |p: &mut Play, at: [f64; 2]| -> f64 { let c = p.g.pl[p.g.player].ch; p.g.ch[c].wet = 0.0; p.g.ch[c].beads = 0.0; p.sam_put(at); p.run(4.0); p.g.ch[c].wet };
    let r = p.g.weather.roof_rects[0];
    // the first open point of the car park inside the office's old rectangle, which once kept it dry
    let mut lot = None;
    'look: for i in 0..40 { for k in 0..28 { let (x, z) = (5.5 + i as f64, 4.5 + k as f64); if x > 35.0 || z > 23.0 { continue; } if !game::weather::weather_under_roof(&mut p.g, x, z) && !game::scene_collision::nav_grid_is_blocked(&p.g, x, z) && !game::scene_collision::solid_blocked(&mut p.g, [x, 0.3, z], [x, 9.0, z], None) { lot = Some([x, z]); break 'look; } } }
    let lot = lot.expect("an open point of the car park");
    let wet = soak(&mut p, lot);
    assert!(wet > 0.3, "on the car park at {lot:?}: {wet}");
    let dry = soak(&mut p, [(r[0] + r[2]) / 2.0, (r[1] + r[3]) / 2.0]);
    assert_eq!(dry, 0.0, "under the roof");
    drop(p);
    let mut p = Play::level("warehouse.json", false, &o);
    p.run(0.1);
    let gusts: Vec<String> = p.g.wind.all.iter().filter_map(|s| p.g.tab.wind_wind_source[*s].door).map(|d| p.g.dr[d].def.name.clone()).collect();
    assert!(gusts.iter().any(|n| n == "fire_exit"), "the warehouse's doors with a gust: {gusts:?}");
    drop(p);
    for level in ["office.json", "block.json", "bazaar.json"] { let mut p = Play::level(level, false, &o); p.run(0.1); assert!(!p.g.weather.own, "{level} keeps the office's roofs, or a still night"); if level == "bazaar.json" { assert_eq!(game::weather::weather_rain_rate(&p.g), 0.0); } }
}

/// THE RULE ("wear, paths in the ground … can all tell a story for how people walk through the level"): the map a level's floors are worn by is worn along every way of it and at every threshold, and
/// bare where nobody walks.
/// A FAILURE MEANS: the ground tells another story than the level's, or none.
#[test]
fn the_traffic_map_is_worn_where_the_level_is_walked() {
    for level in ["pass.json", "compound.json"] {
        let p = Play::level(level, false, &Opts::default());
        let mut t = game_draw::traffic::Traffic::default();
        t.take(&p.g);
        let w = game::scene_world::world();
        let wide = (w.grid_w * w.grid_cell * 8.0) as usize;
        let worn = |x: f64, z: f64| -> u32 { let i = (z * 8.0) as usize * wide + (x * 8.0) as usize; (t.words[i / 2].w >> (i % 2 * 16)) & 255 };
        let ways = game_draw::traffic::ways(&p.g);
        assert!(ways.list.len() >= 10, "{level}: {} ways", ways.list.len());
        let mut near: Vec<[f64; 2]> = vec![];
        for x in &ways.list { for q in x.points.windows(2) { let n = ((q[1][0] - q[0][0]).hypot(q[1][2] - q[0][2]) / 0.25).ceil().max(1.0) as usize; for k in 0..=n { let s = k as f64 / n as f64; near.push([q[0][0] + (q[1][0] - q[0][0]) * s, q[0][2] + (q[1][2] - q[0][2]) * s]); } } }
        let inside: Vec<&[f64; 2]> = near.iter().filter(|q| q[0] > 0.5 && q[1] > 0.5 && q[0] < 39.5 && q[1] < 27.5).collect();
        let bare = inside.iter().filter(|q| worn(q[0], q[1]) < 20).count();
        assert!(bare == 0, "{level}: {bare} of {} points of its ways are not worn", inside.len());
        for d in &p.g.level.doors { let c = game_draw::traffic::door_mid(d); if c[0] > 0.5 && c[0] < 39.5 && c[2] > 0.5 && c[2] < 27.5 { assert!(worn(c[0], c[2]) > 120, "{level}: the threshold of {} is worn {}", d.name, worn(c[0], c[2])); } }
        let mut far = 0;
        for i in 1..39 { for k in 1..27 { let (x, z) = (i as f64 + 0.5, k as f64 + 0.5); if near.iter().all(|q| (q[0] - x).hypot(q[1] - z) > 2.0) && p.g.level.doors.iter().all(|d| { let c = game_draw::traffic::door_mid(d); (c[0] - x).hypot(c[2] - z) > 2.0 }) { far += 1; assert_eq!(worn(x, z), 0, "{level}: ({x}, {z}) is worn and nobody walks there"); } } }
        assert!(far > 20, "{level}: {far} points nobody walks by");
    }
}

/// THE RULE: a door's leaf is made of what its paint is. In a level that wears the look an outside leaf in the level's own accent is paint and an inside one in the palette's wood is wood,
/// whatever shade its own name gives it; one with old paint faded on it is plain; a level whose row gives its paint as numbers says nothing for that side unless its row says a word, and a row
/// with one word means every leaf. A steel door is left as it was made.
/// A FAILURE MEANS: a blown leaf's pieces and a round's chips are of another stuff than the door that was drawn.
#[test]
fn a_levels_leaves_are_made_of_their_paint() {
    // (outside, inside, steel) words met, and the shades of the outside leaves
    let leaves = |level: &str| {
        let p = Play::mission(level, "?roomclear&squad4", &Opts::default());
        let (mut out, mut inside, mut steel, mut shades) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        for &d in &p.g.doors.list {
            let (def, b) = (&p.g.dr[d].def, &p.g.tab.boxes[p.g.dr[d].box_]);
            let of = if def.steel == Some(true) { &mut steel } else if def.exterior == Some(true) { shades.push((b.albedo[2] * 1e4) as i64); &mut out } else { &mut inside };
            let word = b.stuff.map_or(String::from("plain"), |s| format!("{s:?}"));
            if !of.contains(&word) { of.push(word); }
        }
        out.sort(); inside.sort(); shades.sort(); shades.dedup();
        (out.join(" "), inside.join(" "), steel.join(" "), shades.len())
    };
    let is = |level: &str, out: &str, inside: &str, steel: &str| {
        let (o, i, s, shades) = leaves(level);
        assert_eq!((o.as_str(), i.as_str(), s.as_str()), (out, inside, steel), "{level}: outside, inside, steel");
        shades
    };
    let shades = is("barracks.json", "Paint", "Wood plain", "");   // no row: the accent its boxes are painted in, the palette's wood, old paint on some
    assert!(shades > 4, "each leaf a shade of its own: {shades}");
    is("office.json", "Paint", "Wood plain", "plain");
    is("block.json", "plain", "Wood plain", "plain");         // a row of numbers
    is("warehouse.json", "Paint", "Wood plain", "");          // a row that says paint for the outside
    is("killhouse.json", "plain", "Wood plain", "");          // a row that says wood for numbers inside
    let one = changed("killhouse.json", "enamel", &|l| *member(l, "name") = Json::Str("white-roof".into()));
    is(one.to_str().unwrap(), "Enamel", "Enamel", "");        // a row with one word: every leaf, the ones with old paint too
}

/// A level file's most (game::levelfit, what `faults.rs` holds it to) is what the game's lists have room for.
#[test]
fn a_level_may_have_what_the_game_has_room_for() {
    let c = game::levelfit::level_caps();
    for (name, most, room) in [("doors", c.doors, sim::doors_cap(0)), ("things", c.things, sim::props_cap(0)), ("parts", c.parts, sim::props_cap(3)), ("lamps", c.lamps, sim::lights::ROWS)] {
        assert_eq!(most, room as f64, "{name}");
    }
}
