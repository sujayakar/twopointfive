//! The kit picture's stage, written out for the compare with the TypeScript (tools/blitz/kitstage-diff.ts).
//!
//!   kitstage --ask <file>     a request a line (JSON: `level`, `link`, `seed`, `size`), an answer a line: what the staging left, as one object of plain numbers and names
use game::kitstage;
use game_start::json::{self, Json};
use game_start::{files, stand};

fn v(x: &[f64]) -> Json { json::nums(x) }

/// the men, the lights, the moon and sky, what is out of the picture and the last static, as the game has them now
fn staged(g: &game::game::Game) -> Json {
    let st = g.kit_stage.as_ref().expect("a stage");
    let pl = g.player;
    let man = |name: String, c: game::ids::CharId| { let r = &g.ch[c]; json::obj(vec![("name", json::s(&name)), ("x", json::n(r.pos[0])), ("z", json::n(r.pos[2])), ("yaw", json::n(r.body_yaw)), ("aim", json::n(r.aim_yaw))]) };
    let mut men = vec![man(String::from("Sam"), g.pl[pl].ch)];
    for &x in &st.order { men.push(man(g.gd[x].callsign.clone(), g.gd[x].ch)); }
    let lights: Vec<Json> = g.engine.lights.lights.iter().map(|&l| { let r = &g.tab.light[l]; json::obj(vec![("name", json::s(&r.name)), ("group", json::s(&r.group)), ("fixture", json::b(g.targets.iter().any(|&t| g.tab.sim_lightshome_fixture[t].light == Some(l)))), ("enabled", json::b(r.enabled)), ("intensity", json::n(r.intensity)), ("color", v(&r.color)), ("pos", v(&r.pos))]) }).collect();
    use game::sim_lightshome as home;
    let fixtures: Vec<Json> = g.targets.iter().map(|&t| { let r = &g.tab.sim_lightshome_fixture[t]; json::obj(vec![("name", json::s(&r.name)), ("on", json::b(home::fixture_on(g, t))), ("broken", json::b(home::fixture_broken(g, t))), ("base", json::n(home::fixture_base_intensity(g, t))), ("emissive", v(&r.base_emissive))]) }).collect();
    let moon = g.engine.lights.lights.iter().find(|&&l| g.tab.light[l].name == "moon" && g.tab.light[l].kind == 2.0).map_or(Json::Null, |&l| { let r = &g.tab.light[l]; json::obj(vec![("intensity", json::n(r.intensity)), ("color", v(&r.color))]) });
    let under: Vec<Json> = g.col.boxes.iter().enumerate().filter(|(_, &b)| g.tab.boxes[b].c[1] < -5.0).map(|(i, _)| json::n(i as f64)).collect();
    let props: Vec<Json> = g.props.all.iter().enumerate().filter(|(_, p)| !g.props.props.contains(p)).map(|(i, _)| json::n(i as f64)).collect();
    let statics = &g.engine.world.statics;
    let last = statics.last().map_or(Json::Null, |&b| { let r = &g.tab.boxes[b]; json::obj(vec![("c", v(&r.c)), ("h", v(&r.h)), ("yaw", json::n(r.yaw)), ("emissive", v(&r.emissive)), ("flags", json::n(r.flags))]) });
    let s = &g.engine.settings;
    json::obj(vec![("men", json::arr(men)), ("lights", json::arr(lights)), ("fixtures", json::arr(fixtures)), ("moon", moon), ("zenith", v(&s.sky_zenith)), ("horizon", v(&s.sky_horizon)), ("under", json::arr(under)), ("props", json::arr(props)), ("statics", json::n(statics.len() as f64)), ("last", last)])
}
fn pin(g: &game::game::Game) -> Json {
    match g.host_cam.and_then(|c| g.tab.camera[c].pin) {
        Some(p) => json::obj(vec![("look", v(&p.look)), ("yaw", json::n(p.yaw)), ("pitch", json::n(p.pitch)), ("dist", json::n(p.dist))]),
        None => Json::Null,
    }
}

fn answer(r: &Json, ch: &files::CharacterFiles) -> Json {
    let data = json::parse(&files::level_text(r.get("level").str().expect("kitstage: a level"))).expect("kitstage: the level's text");
    let link = String::from(r.get("link").str().expect("kitstage: a link"));
    let seed = r.get("seed").num().unwrap_or(3.0) as u32;
    let mut s = stand::stand(&data, ch.assets(), &stand::StartOpts { seed, link: Some(link), ..Default::default() });
    let g = &mut s.game;
    let none = g.kit_stage.as_ref().map_or(true, |k| k.none);
    let mut o = vec![("none", json::b(none))];
    if none { return json::obj(o); }
    o.push(("staged", staged(g)));
    kitstage::frame(g, 1.6, None);
    o.push(("pin0", pin(g)));
    kitstage::frame(g, 1.6, kitstage::frame_ask(-0.4, 0.0, 0.6, 0.5, 0.19, Some([-0.2, -0.9, 0.1, 0.85]), None));
    o.push(("pin1", pin(g)));
    if let Some(n) = r.get("size").num() {
        kitstage::cards(g, n as i32);
        let none = g.kit_stage.as_ref().map_or(true, |k| k.none);
        o.push(("restaged", if none { Json::Null } else { staged(g) }));
    }
    json::obj(o)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let path = a.get(2).expect("kitstage --ask <file>").clone();
    // (the constructor's frames are deep in a build with no optimisation: a thread with room)
    let h = std::thread::Builder::new().stack_size(1 << 30).spawn(move || {
        let ch = files::CharacterFiles::read();
        for line in std::fs::read_to_string(&path).expect("kitstage: the requests").lines() {
            let r = json::parse(line).expect("kitstage: a request");
            println!("{}", json::to_line(&answer(&r, &ch)));
        }
    }).unwrap();
    h.join().unwrap();
}
