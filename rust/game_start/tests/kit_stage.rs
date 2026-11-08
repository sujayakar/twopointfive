//! THE KIT PICTURE'S STAGE: the mission's own level, stood from a link that carries `kitview`, is held as the picture of the team at its way in. Every case stands on what `stage` wrote: with it doing nothing the
//! men stand where the mission started them, nobody is parked and no pin is set.
use game::game::GuardState;
use game::kitstage::{self, KitStage};
use game::teams::Team;
use game_start::json::{self, Json};
use game_start::play::{Opts, Play};

/// (a level, the link's words, how many teammates come)
const CASES: &[(&str, &str, usize)] = &[
    ("records.json", "roomclear&squad4", 4),
    ("records.json", "roomclear&squad4&way=2", 4),
    ("warehouse.json", "roomclear&squad=2", 2),
    ("mousehole.json", "roomclear&squad4", 4),
    ("mousehole.json", "roomclear&squad4&way=2", 4),
    ("compound.json", "roomclear&squad=3", 3),
    ("compound.json", "roomclear&squad4&way=2", 4),
    ("consulate.json", "rescue&squad=2", 2),
    ("bazaar.json", "quiet&squad=1", 1),
    ("vault.json", "quiet", 0),
    ("office.json", "", 0),
    ("registry.json", "rescue&squad=1", 1),
    ("office.json", "nightbreach&squad=4", 4),
    ("killhouse.json", "roomclear&squad=0", 0),
    ("depot.json", "roomclear&squad=1&way=2", 1),
    ("records.json", "roomclear&squad=2&fog=team", 2),
];

fn stood(level: &str, words: &str) -> Play {
    let link = format!("?levelFile={}&kitview{}{}", level, if words.is_empty() { "" } else { "&" }, words);
    Play::mission(level, &link, &Opts { seed: 3, ..Default::default() })
}
fn st(p: &Play) -> &KitStage { p.g.kit_stage.as_ref().expect("a link with `kitview` has a stage") }
fn flat(a: [f64; 3], b: [f64; 2]) -> f64 { ((a[0] - b[0]).powi(2) + (a[2] - b[1]).powi(2)).sqrt() }

fn on_their_places(p: &Play, what: &str, tol: f64) {
    let s = st(p);
    let pl = p.g.player;
    let d = flat(p.g.ch[p.g.pl[pl].ch].pos, s.sam);
    assert!(d <= tol, "{}: Sam is {} m off his place", what, d);
    for (k, &x) in s.order.iter().enumerate() {
        let want = match s.mate { Some((m, _)) if k == 0 && s.who == 0 => m, _ => s.spots[k] };
        let d = flat(p.g.ch[p.g.gd[x].ch].pos, want);
        assert!(d <= tol, "{}: man {} of the file is {} m off his place", what, k, d);
    }
}
fn parked(p: &Play, what: &str) -> usize {
    let mut n = 0;
    for &x in p.g.guards.iter().chain(p.g.civilians.iter()) {
        let r = &p.g.gd[x];
        if r.team == Team::Operative { continue; }
        let c = p.g.ch[r.ch].pos;
        assert!(c[0] <= -39.0 && c[2] <= -39.0, "{}: a man of the level is at {:?}", what, c);
        assert!(r.state == GuardState::Patrol || r.state == GuardState::Dead, "{}: a man of the level is {:?}", what, r.state);
        n += 1;
    }
    n
}
fn lit(p: &Play, id: Option<game::ids::LightId>) -> Option<(bool, f64)> {
    let l = id?;
    if !p.g.engine.lights.lights.contains(&l) { return None; }
    Some((p.g.tab.light[l].enabled, p.g.tab.light[l].intensity))
}

#[test]
fn every_kind_of_way_stands_stays_and_is_lit() {
    let (mut defenders, mut lit_on_some) = (0, 0);
    for &(level, words, mates) in CASES {
        let what = format!("{} ?{}", level, words);
        let mut p = stood(level, words);
        assert!(!st(&p).none, "{}: nothing staged", what);
        assert_eq!(st(&p).order.len(), mates, "{}: the men", what);
        assert!(!p.g.fog_of_war.on && !game::fogofwar::fog_of_war_active(&p.g), "{}: the fog of war is on", what);
        on_their_places(&p, &what, 0.05);
        let plan = st(&p).plan.clone().unwrap();
        // the plan's places, but on the quiet picture's rest
        if !st(&p).quiet { for k in 0..mates { assert_eq!(st(&p).spots[k], [plan.slots[k + 1][0], plan.slots[k + 1][1]], "{}: man {}", what, k); } }
        defenders += parked(&p, &what);
        // the pin
        let cam = p.g.host_cam.unwrap();
        let pin = p.g.tab.camera[cam].pin.unwrap_or_else(|| panic!("{}: no pin", what));
        assert!(pin.look.iter().all(|x| x.is_finite()) && pin.yaw.is_finite() && pin.pitch.is_finite() && pin.dist > 0.0, "{}: {:?}", what, pin);
        // the two lamps, as the photocell has them
        assert!(lit(&p, st(&p).porch).is_some() && lit(&p, st(&p).fill).is_some(), "{}: a lamp is not hung", what);
        let o = &st(&p).over.porch;
        let table = if st(&p).quiet { kitstage::light::KIT_VIEW_QUIET.lamp.intensity } else { kitstage::light::KIT_VIEW_LIGHT.porch.intensity };
        let burns = kitstage::porch_lit(o.intensity.unwrap_or(table), o.day, plan.indoors, kitstage::hour_has_lamps_out(&p.g));
        assert_eq!(lit(&p, st(&p).porch).unwrap(), (burns > 0.0, burns), "{}: the porch lamp", what);
        lit_on_some += (burns > 0.0) as usize;
        // ten seconds of ticks: nobody moves, nobody of the level is roused, nothing the game writes each step moves the lamps
        p.run(10.0);
        on_their_places(&p, &format!("{} after ten seconds", what), 0.05);
        parked(&p, &what);
        assert_eq!((st(&p).roused_men, st(&p).roused_alarms), (0, 0), "{}: the park had to put men or alarms right", what);
        assert_eq!(p.g.escalation, 0.0, "{}: the floor was raised", what);
        assert!(p.g.tab.camera[cam].pin.is_some(), "{}: the pin went", what);
        assert!(lit(&p, st(&p).porch).is_some(), "{}: the lamp went", what);
    }
    assert!(defenders > 20, "the levels' men: {}", defenders);
    assert!(lit_on_some >= 8, "the porch lamp burns on {} of the ways", lit_on_some);
}

#[test]
fn by_day_the_porch_lamp_is_out_with_the_levels_own() {
    let p = stood("records.json", "roomclear&squad4&hour=midday");
    assert!(kitstage::hour_has_lamps_out(&p.g));
    assert_eq!(lit(&p, st(&p).porch), Some((false, 0.0)), "the lamp at noon");
    assert!(st(&p).fixture.is_none(), "its box glows only on a lit lamp");
    drop(p);
    let p = stood("records.json", "roomclear&squad4&hour=dusk");
    assert!(!kitstage::hour_has_lamps_out(&p.g));
    assert!(lit(&p, st(&p).porch).unwrap().0 && st(&p).fixture.is_some(), "the lamp at dusk");
}

#[test]
fn another_size_stages_that_many_and_the_mission_keeps_its_own() {
    let mut p = stood("records.json", "roomclear&squad4");
    for n in [2, 0, 3, 4, 1] {
        kitstage::cards(&mut p.g, n); p.frame();
        assert!(!st(&p).none, "{} men: nothing staged", n);
        assert_eq!(st(&p).order.len(), n as usize, "{} asked", n);
        on_their_places(&p, &format!("{} men", n), 0.05);
        p.run(1.0);
        on_their_places(&p, &format!("{} men a second on", n), 0.05);
    }
    drop(p);
    // (a quiet mission's men are its own: the cards are written, the size is not asked)
    let mut q = stood("bazaar.json", "quiet&squad=1");
    kitstage::cards(&mut q.g, 3); q.frame();
    assert_eq!(st(&q).order.len(), 1);
    assert!(st(&q).quiet);
}

#[test]
fn an_open_card_stands_its_man_first() {
    let mut p = stood("records.json", "roomclear&squad4");
    let team = game::scenario_run::team_of(&p.g);
    kitstage::open(&mut p.g, 2, "weapon"); p.frame();
    assert_eq!(st(&p).who, 2);
    let now = game::scenario_run::team_of(&p.g);
    assert_eq!(team.len(), now.len());
    assert_eq!(st(&p).order[0], now[1], "the open card's man is first at the door");
    on_their_places(&p, "card 2", 0.05);
    kitstage::open(&mut p.g, 0, ""); p.frame();
    assert_eq!(st(&p).order[0], game::scenario_run::team_of(&p.g)[0]);
}

#[test]
fn a_card_may_be_open_before_its_man_has_come() {
    let mut p = stood("records.json", "roomclear&squad4");
    kitstage::cards(&mut p.g, 3); p.frame();
    kitstage::open(&mut p.g, 4, "weapon"); p.frame();
    assert_eq!((st(&p).who, st(&p).order.len()), (0, 3), "three came: the player's card stands in");
    kitstage::cards(&mut p.g, 4); p.frame();
    assert_eq!(st(&p).who, 4);
    assert_eq!(st(&p).order[0], game::scenario_run::team_of(&p.g)[3], "he came, and his card is open");
    // (a message and a card in the same moment are one restaging, and the lamps hung are the same rows)
    kitstage::cards(&mut p.g, 3); p.frame();
    let was = p.g.mission.serial;
    kitstage::cards(&mut p.g, 3); p.frame();
    let (once, at) = (p.g.mission.serial - was, p.g.mission.serial);
    let lamps = (st(&p).porch, st(&p).fill);
    kitstage::cards(&mut p.g, 3);
    kitstage::open(&mut p.g, 1, "weapon"); p.frame();
    assert_eq!((st(&p).porch, st(&p).fill), lamps);
    assert_eq!(p.g.mission.serial - at, once, "staged again once");
}

#[test]
fn a_frame_moves_the_pin_and_stays_in_range() {
    let mut p = stood("records.json", "roomclear&squad4");
    let cam = p.g.host_cam.unwrap();
    let was = p.g.tab.camera[cam].pin.unwrap();
    let ask = kitstage::frame_ask(-0.4, 0.0, 0.6, 0.5, 0.19, [-0.2, -0.9, 0.1, 0.85], [f64::NAN; 2]);
    kitstage::frame(&mut p.g, 1.6, ask);
    let now = p.g.tab.camera[cam].pin.unwrap();
    assert!(now.look != was.look || now.dist != was.dist, "the frame moved nothing");
    p.run(0.5);
    let then = p.g.tab.camera[cam].pin.unwrap();
    assert_eq!((then.look, then.dist), (now.look, now.dist), "the tick moved the pin");
}

#[test]
fn nothing_to_stage_says_so_and_never_stops() {
    // no block on the level; a mission the level does not have
    for (level, words) in [("kit-yard.json", "roomclear&squad4"), ("kit-yard.json", "rescue"), ("consulate.json", "roomclear&squad=2"), ("vault.json", "rescue")] {
        let what = format!("{} ?{}", level, words);
        let mut p = stood(level, words);
        assert!(st(&p).none, "{}: staged", what);
        p.run(0.5);
        assert!(st(&p).none, "{}: staged a tick on", what);
        kitstage::cards(&mut p.g, 2); p.frame();
        assert!(st(&p).none, "{}: staged by the cards", what);
    }
}

#[test]
fn a_game_with_no_kitview_has_no_stage() {
    let p = Play::mission("records.json", "?levelFile=records.json&roomclear&squad4", &Opts { seed: 3, ..Default::default() });
    assert!(p.g.kit_stage.is_none());
    let cam = p.g.host_cam.unwrap();
    assert!(p.g.tab.camera[cam].pin.is_none());
}

#[test]
fn a_way_with_pieces_lays_them_and_a_way_on_a_set_is_the_set() {
    // (no level writes `add` on a way today: one is written here)
    let mut d: Json = json::parse(&game_start::files::level_text("records.json")).unwrap();
    let k = d.get("scenarios").get("kitView").clone();
    let y0 = k.get("ways").at(0).clone();
    let plan_at = stood("records.json", "roomclear&squad4");
    let sam = st(&plan_at).plan.as_ref().unwrap().slots[0];
    drop(plan_at);
    let crate_at = format!("{{\"boxes\":[{{\"id\":\"kp_crate\",\"c\":[{},0.5,{}],\"h\":[0.4,0.5,0.4],\"albedo\":[1,1,1]}}]}}", sam[0] + 6.0, sam[1] + 6.0);
    let mut y = y0.clone();
    y.set_member("add", json::parse(&crate_at).unwrap());
    let mut block = k.clone();
    block.set_member("ways", json::arr(vec![y]));
    let mut scen = d.get("scenarios").clone();
    scen.set_member("kitView", block);
    d.set_member("scenarios", scen);
    let base = d.get("boxes").items().len();
    let dir = std::env::temp_dir().join("kit-stage-test");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("added.json");
    std::fs::write(&path, json::to_line(&d)).unwrap();
    let p = Play::mission(path.to_str().unwrap(), "?levelFile=added.json&kitview&roomclear&squad4", &Opts { seed: 3, ..Default::default() });
    assert!(!st(&p).none);
    assert!(p.g.engine.world.statics.len() > base, "the piece and the lamp's box are in the world");
    let found = p.g.col.boxes.iter().filter(|&&b| { let c = p.g.tab.boxes[b].c; (c[0] - (sam[0] + 6.0)).abs() < 1e-9 && (c[2] - (sam[1] + 6.0)).abs() < 1e-9 && p.g.tab.boxes[b].h[0] == 0.4 }).count();
    assert_eq!(found, 1, "the way's piece is laid in");
    drop(p);
    // a set: the world is the set's boxes, and the plan says which
    let on_set = stood("mousehole.json", "roomclear&squad4");
    let data = json::parse(&game_start::files::level_text("mousehole.json")).unwrap();
    let name = data.get("scenarios").get("kitView").get("ways").at(0).get("set").str().unwrap();
    assert_eq!(st(&on_set).plan.as_ref().unwrap().set.as_deref(), Some(name));
    let boxes = data.get("scenarios").get("kitView").get("sets").get(name).get("boxes").items().len();
    assert!(on_set.g.col.boxes.len() >= boxes && on_set.g.col.boxes.len() < data.get("boxes").items().len(), "{} boxes for a set of {}", on_set.g.col.boxes.len(), boxes);
}
