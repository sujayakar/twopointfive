//! The kit picture's plan and its set, written out for the compare with the TypeScript (tools/blitz/kitplan-diff.ts).
//!
//!   kitplan <level file> <way>     one line: the way's plan, or the whole level `set_level` / `add_level` make for it where the way is on a set or adds a piece
//!   kitplan --ask <file>           a request a line (JSON, see `answer`), an answer a line: the process start is paid once
use game_start::json::{self, Json};
use game_start::kit_plan::{self as kp, PlanBlock, PlanLevel, PlanOpts, Way};
use game_start::kit_set::{self as ks, Query, QuietKind};
use std::collections::{BTreeSet, HashMap};

struct Level { data: Json, level: PlanLevel, block: PlanBlock, spawn: [f64; 2] }
fn read(path: &str) -> Level {
    let data = json::parse(&std::fs::read_to_string(path).expect("kitplan: the level file")).expect("kitplan: the level's text");
    let sp = data.get("mission").get("playerSpawn").nums();
    let (level, block) = (PlanLevel::of(&data), PlanBlock::of(data.get("scenarios").get("kitView")));
    Level { spawn: [sp[0], sp[2]], data, level, block }
}
fn v2(j: &Json) -> [f64; 2] { let a = j.nums(); [a[0], a[1]] }
fn slots(j: &Json) -> Vec<[f64; 3]> { j.items().iter().map(|p| p.v3().expect("kitplan: a place")).collect() }
fn usizes(x: &[usize]) -> Json { json::arr(x.iter().map(|&i| json::n(i as f64)).collect()) }
fn names(x: &[String]) -> Json { json::arr(x.iter().map(|s| json::s(s)).collect()) }
fn query(r: &Json) -> Query { Query::parse(r.get("link").str().expect("kitplan: a link")) }
/// the level a way is planned on, whole, as `set_level` / `add_level` have it: and whether it is the very level handed in
fn level_of(c: std::borrow::Cow<'_, Json>) -> Json {
    match c { std::borrow::Cow::Borrowed(_) => Json::Null, std::borrow::Cow::Owned(o) => o }
}

fn answer(r: &Json, levels: &mut HashMap<String, Level>) -> Json {
    let op = r.get("op").str().expect("kitplan: an op");
    if let Some(path) = r.get("level").str() { if !levels.contains_key(path) { let l = read(path); levels.insert(String::from(path), l); } }
    let l = r.get("level").str().map(|p| &levels[p]);
    let lv = || l.expect("kitplan: a level");
    // the way: written in the request, or the level's own by index
    let way = || -> Way {
        if !r.get("y").is_null() { Way::of(r.get("y")) } else { Way::of(lv().data.get("scenarios").get("kitView").get("ways").at(r.get("way").num().expect("kitplan: a way") as usize)) }
    };
    let ref_ = || if r.get("ref").is_null() { lv().spawn } else { v2(r.get("ref")) };
    match op {
        "plan" => {
            let also: Vec<String> = r.get("alsoHide").items().iter().map(|x| String::from(x.str().unwrap())).collect();
            let opts = PlanOpts { hidden_stand: r.get("hiddenStand").bool() == Some(true), also_hide: &also, ..PlanOpts::default() };
            let mut level = None;
            if !r.get("left").is_null() { let mut c = lv().level.clone(); c.left = r.get("left").items().iter().map(|x| String::from(x.str().unwrap())).collect(); level = Some(c); }
            kp::planned_json(&kp::plan_way(level.as_ref().unwrap_or(&lv().level), Some(&lv().block), &way(), ref_(), &opts))
        }
        "child" => {
            let y = way();
            let g = match kp::geometry(&lv().level, Some(&lv().block), &y, ref_()) { Some(g) => g, None => return Json::Null };
            let fl = kp::floor(&g.level.boxes, None);
            let free = |x: f64, z: f64| !fl.blocked(x, z);
            let mut sunk: Vec<Json> = Vec::new();
            let p = kp::plan_child(&g.level, &y, g.ref_, &free, &mut |h: &[usize]| sunk.push(usizes(h)));
            json::obj(vec![("plan", kp::planned_json(&p)), ("sunk", json::arr(sunk))])
        }
        "set_level" => level_of(ks::set_level(&lv().data, r.get("way").num().unwrap() as usize)),
        "add_level" => level_of(ks::add_level(&lv().data, r.get("way").num().unwrap() as usize)),
        "set_of" => match ks::set_of(&lv().data, r.get("way").num().unwrap() as usize) { Some(h) => json::s(h.name), None => Json::Null },
        "way_of_query" => json::n(ks::way_of_query(r.get("q").str(), &lv().data) as f64),
        "way_of_link" => json::n(ks::way_of_link(&query(r), &lv().data) as f64),
        "set_query" => json::arr(ks::set_query(&query(r)).0.iter().map(|(k, v)| json::arr(vec![json::s(k), json::s(v)])).collect()),
        "set_dress" => match ks::set_dress(&query(r)) { Some(d) => json::obj(vec![("suit", d.suit.map_or(Json::Null, json::s)), ("sling", json::n(d.sling))]), None => Json::Null },
        "quiet_kind" => match ks::quiet_kind(&query(r)) { Some(QuietKind::Quiet) => json::s("quiet"), Some(QuietKind::Solo) => json::s("solo"), None => Json::Null },
        "paint" => ks::set_paint(r.get("b"), r.get("materials")),
        "floor" => {
            let out: BTreeSet<usize> = r.get("out").items().iter().map(|x| x.num().unwrap() as usize).collect();
            let fl = kp::floor(&lv().level.boxes, if r.has("out") { Some(&out) } else { None });
            let mut grid = String::new();
            for z in -1..=56 { for x in -1..=80 { grid.push(if fl.blocked((x as f64 + 0.5) * 0.5, (z as f64 + 0.5) * 0.5) { '1' } else { '0' }); } }
            json::obj(vec![("solid", usizes(&fl.solid)), ("grid", json::s(&grid))])
        }
        "slots" => {
            let fl = kp::floor(&lv().level.boxes, None);
            let free = |x: f64, z: f64| !fl.blocked(x, z);
            let n = |k: &str| r.get(k).num().unwrap();
            let o = kp::slots(v2(r.get("a")), v2(r.get("t")), n("width"), v2(r.get("n")), n("side"), n("off"), n("samOff"), &free);
            json::obj(vec![("slots", json::arr(o.slots.iter().map(|p| json::nums(p)).collect())), ("stepped", json::n(o.stepped as f64)), ("steps", usizes(&o.steps))])
        }
        "act4" => {
            let fl = if r.get("useFree").bool() == Some(true) { Some(kp::floor(&lv().level.boxes, None)) } else { None };
            let free_own = |x: f64, z: f64| !fl.as_ref().unwrap().blocked(x, z);
            let free: Option<&dyn Fn(f64, f64) -> bool> = if fl.is_some() { Some(&free_own) } else { None };
            json::nums(&kp::act4(v2(r.get("mid")), v2(r.get("s")), v2(r.get("n")), r.get("width").num().unwrap(), &slots(r.get("slots")), free, r.get("clear").num()))
        }
        "lit_inside" => names(&kp::lit_inside(&lv().level.lights, v2(r.get("mid")), v2(r.get("n")), r.get("back").num().unwrap()).iter().map(|x| x.id.clone()).collect::<Vec<_>>()),
        "porch_lit" => json::n(kp::porch_lit(r.get("lit").num().unwrap(), r.get("day").num(), r.get("indoors").bool().unwrap(), r.get("lampsOut").bool().unwrap())),
        "day_out" => {
            let y = way();
            let g = match kp::geometry(&lv().level, Some(&lv().block), &y, ref_()) { Some(g) => g, None => return Json::Null };
            names(&kp::day_out(kp::own_lamps(Some(&lv().block), &y), &g.level.boxes, y.day_lights.as_deref(), r.get("indoors").bool().unwrap(), r.get("lampsOut").bool().unwrap()))
        }
        "covered" => json::b(kp::covered(&lv().level.boxes, r.get("x").num().unwrap(), r.get("z").num().unwrap(), r.get("y").num().unwrap())),
        _ => panic!("kitplan: no op '{}'", op),
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let mut levels: HashMap<String, Level> = HashMap::new();
    if a.get(1).map(|x| x.as_str()) == Some("--ask") {
        use std::io::Write;
        let text = std::fs::read_to_string(&a[2]).expect("kitplan: the requests");
        let out = std::io::stdout();
        let mut out = std::io::BufWriter::new(out.lock());
        for line in text.lines() {
            let r = json::parse(line).expect("kitplan: a request");
            writeln!(out, "{}", json::to_line(&answer(&r, &mut levels))).expect("kitplan: the answers");
        }
        return;
    }
    let (path, way) = (a.get(1).expect("kitplan <level file> <way>"), a.get(2).expect("kitplan <level file> <way>").parse::<usize>().expect("kitplan: a way's index"));
    let l = read(path);
    let made = match ks::set_level(&l.data, way) { std::borrow::Cow::Owned(o) => Some(o), _ => match ks::add_level(&l.data, way) { std::borrow::Cow::Owned(o) => Some(o), _ => None } };
    let y = Way::of(l.data.get("scenarios").get("kitView").get("ways").at(way));
    let plan = kp::plan_way(&l.level, Some(&l.block), &y, l.spawn, &PlanOpts::default());
    println!("{}", json::to_line(&json::obj(vec![("plan", kp::planned_json(&plan)), ("level", made.unwrap_or(Json::Null))])));
}
