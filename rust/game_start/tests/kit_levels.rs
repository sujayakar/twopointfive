//! THE KIT PICTURE ON EVERY LEVEL THAT HAS ONE: every level file with a `scenarios.kitView` block, every way of it, each mission kind the mission list has for that level, stands from its link with `kitview`
//! in it as the page's frame stands it, says it has a picture (the view's word 1), has its lamps and pin in finite numbers, and holds for five seconds with nobody of the level roused. The list is read off its own
//! file (src/scene/scenarioList.ts), so a new row or a new way is asked here with nothing to write. What a level's AUTHOR runs is tools/qa/kitview-plan.ts: that one reads the plan, this one stands the game.
//! A look twin (`x-gwot.json`) has its plain file's rows; a level with a block and no row of the list has the kind its own file writes.
use game_start::json::{self, Json};
use game_start::play::{Opts, Play};
use std::collections::BTreeMap;
use std::path::PathBuf;

fn root() -> PathBuf { game_start::files::root() }

/// (kind, level file) of every row of the mission list; the office writes no file there
fn rows_of_the_list() -> Vec<(String, String)> {
    let text = std::fs::read_to_string(root().join("src/scene/scenarioList.ts")).expect("the mission list");
    let after = |c: &str, key: &str| c.find(key).map(|i| { let t = &c[i + key.len()..]; String::from(&t[..t.find('\'').expect("a closing quote")]) });
    let rows: Vec<(String, String)> = text.split("{ id: '").skip(1).filter_map(|c| {
        let kind = after(c, "kind: '")?;
        Some((kind, after(c, "level: '").unwrap_or_else(|| String::from("office.json"))))
    }).collect();
    assert!(rows.len() >= 20, "the mission list reads {} rows: its file has changed shape and this reader wants mending", rows.len());
    rows
}
/// what a mission of that kind says in its link, and how many teammates it brings by default
fn words_of(kind: &str) -> Option<&'static str> {
    match kind { "roomclear" => Some("roomclear&squad4"), "rescue" => Some("rescue&squad=2"), "nightbreach" => Some("nightbreach&squad=2"), "quiet" => Some("quiet&squad=1"), _ => None }
}
fn own_kinds(data: &Json) -> Vec<String> {
    let s = data.get("scenarios");
    let mut k: Vec<String> = [("roomClear", "roomclear"), ("rescue", "rescue"), ("nightBreach", "nightbreach"), ("quiet", "quiet")].iter().filter(|(key, _)| !s.get(key).is_null()).map(|(_, w)| String::from(*w)).collect();
    if k.is_empty() { k.push(String::new()); }
    k
}

fn staged(level: &str, words: &str) {
    let what = format!("{} ?{}", level, words);
    let link = format!("?levelFile={}&kitview&stillkit{}{}", level, if words.is_empty() { "" } else { "&" }, words);
    let mut p = Play::mission(level, &link, &Opts { seed: 3, ..Default::default() });
    let s = p.g.kit_stage.as_ref().unwrap_or_else(|| panic!("{}: no stage", what));
    assert!(!s.none, "{}: the view says nothing to stage (word 2), the page would show an empty column", what);
    for (name, l) in [("porch", s.porch), ("fill", s.fill)] {
        let l = l.unwrap_or_else(|| panic!("{}: the {} lamp is not hung", what, name));
        let r = &p.g.tab.light[l];
        assert!(r.pos.iter().all(|x| x.is_finite()) && r.intensity.is_finite() && r.range.is_finite(), "{}: the {} lamp: pos {:?}, intensity {}, range {}", what, name, r.pos, r.intensity, r.range);
    }
    let cam = p.g.host_cam.unwrap();
    let pin = p.g.tab.camera[cam].pin.unwrap_or_else(|| panic!("{}: no pin", what));
    assert!(pin.look.iter().all(|x| x.is_finite()) && pin.yaw.is_finite() && pin.pitch.is_finite() && pin.dist.is_finite() && pin.dist > 0.0, "{}: the pin: {:?}", what, pin);
    p.run(5.0);
    let s = p.g.kit_stage.as_ref().unwrap();
    assert!(!s.none, "{}: nothing staged five seconds on", what);
    assert_eq!((s.roused_men, s.roused_alarms), (0, 0), "{}: the park had to put men or alarms right", what);
    assert_eq!(p.g.escalation, 0.0, "{}: the floor was raised", what);
}

#[test]
fn every_way_of_every_block_stands_for_every_kind_the_list_has() {
    let mut listed: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (kind, level) in rows_of_the_list() { let e = listed.entry(level).or_default(); if !e.contains(&kind) { e.push(kind); } }
    let mut files: Vec<String> = std::fs::read_dir(root().join("levels")).expect("the levels").filter_map(|f| f.ok()?.file_name().into_string().ok()).filter(|f| f.ends_with(".json") && !f.contains(".lint.")).collect();
    files.sort();
    let (mut blocks, mut work) = (0, Vec::<(String, String)>::new());
    for f in &files {
        let data = json::parse(&game_start::files::level_text(f)).unwrap_or_else(|e| panic!("{}: {}", f, e));
        let ways = data.get("scenarios").get("kitView").get("ways").items().len();
        if ways == 0 { continue; }
        blocks += 1;
        let kinds = listed.get(f).or_else(|| listed.get(&f.replace("-gwot", ""))).cloned().unwrap_or_else(|| own_kinds(&data));
        for kind in &kinds {
            let base = words_of(kind).unwrap_or("");
            work.push((f.clone(), String::from(base)));
            // (only a room clear has ways of its own: every other kind stands on the level's one start, game_start::kit_set::way_of_link)
            if kind == "roomclear" { for way in 1..ways { work.push((f.clone(), format!("{}&way={}", base, way + 1))); } }
        }
        // (the solo night is the bundled office's, and no row of the list)
        if f == "office.json" { work.push((f.clone(), String::new())); }
    }
    assert!(blocks >= 30 && work.len() >= 40, "{} blocks, {} stagings: too few, the reader above has lost them", blocks, work.len());
    for (f, words) in &work { staged(f, words); }
}
