//! WHAT THE JUDGE OF A LEVEL'S FILE REFUSES AND WHAT IT TAKES: a real file with one value of it said otherwise, and the place its one fault is told at (faults.rs, file.rs). A FAILURE MEANS: a file an
//! author gets wrong in this way goes into the tree with no word, or a file that is right can't.
use game_start::{faults, files, json::{self, Json}};

/// `file` with the value at `place` (members and items, by a dot) said as `v`
fn said(file: &str, place: &str, v: Json) -> Json {
    fn put(at: &mut Json, place: &str, v: Json) {
        let Some((head, rest)) = place.split_once('.') else {
            match at { Json::Obj(..) => at.set_member(place, v), Json::Arr(a) => a[place.parse::<usize>().unwrap()] = v, _ => panic!("nothing is at {place}") }
            return;
        };
        match at {
            Json::Obj(m, _) => put(&mut m.iter_mut().find(|(k, _)| k == head).unwrap_or_else(|| panic!("no member {head}")).1, rest, v),
            Json::Arr(a) => put(&mut a[head.parse::<usize>().unwrap()], rest, v),
            _ => panic!("nothing is under {head}"),
        }
    }
    let mut data = json::parse(&files::level_text(file)).unwrap();
    put(&mut data, place, v);
    data
}
fn told(data: &Json) -> Vec<String> { faults::of(data).iter().map(|f| f.split(": ").next().unwrap().to_string()).collect() }
/// the level files in their names' order, as trees
fn levels() -> Vec<(String, Json)> {
    let mut names: Vec<String> = std::fs::read_dir(files::root().join("levels")).unwrap().map(|e| e.unwrap().file_name().into_string().unwrap()).filter(|n| n.ends_with(".json")).collect();
    names.sort();
    names.into_iter().map(|n| { let d = json::parse(&files::level_text(&n)).unwrap(); (n, d) }).filter(|(_, d)| d.has("boxes")).collect()
}

#[test]
fn a_number_is_in_its_range() {
    for (v, bad) in [(0.0, false), (0.7, false), (1.0, false), (1.5, true), (-0.1, true)] {
        assert_eq!(told(&said("white-roof.json", "meta.kept", json::n(v))), if bad { vec!["meta.kept"] } else { vec![] }, "kept {v}");
    }
    assert_eq!(told(&said("white-roof.json", "meta.kept", json::s("1"))), ["meta.kept"]);
    let box_place = |i: usize, k: &str| { let d = json::parse(&files::level_text("office.json")).unwrap(); format!("boxes[{i}] '{}'.{k}", d.get("boxes").at(i).get("id").str().unwrap()) };
    assert_eq!(told(&said("office.json", "materials.lino.rough", json::n(1.5))), ["materials.lino.rough"]);
    assert_eq!(told(&said("office.json", "boxes.3.rough", json::n(-0.1))), [box_place(3, "rough")]);
    assert_eq!(told(&said("office.json", "materials.metal.metal", json::s("yes"))), ["materials.metal.metal"]);
    assert_eq!(told(&said("office.json", "boxes.4.metal", json::n(1.0))), [box_place(4, "metal")]);
}

/// A file from before a paint had a finish is a level.
#[test]
fn a_finish_may_be_left_out() {
    let mut d = json::parse(&files::level_text("office.json")).unwrap();
    let Json::Obj(top, _) = &mut d else { panic!("a file is a record") };
    for (k, v) in top.iter_mut() {
        let plain = |x: &Json| x.without("rough").without("metal");
        match v { Json::Arr(a) if k == "boxes" => a.iter_mut().for_each(|x| *x = plain(x)), Json::Obj(m, _) if k == "materials" => m.iter_mut().for_each(|(_, x)| *x = plain(x)), _ => {} }
    }
    assert_eq!(told(&d), [""; 0]);
}

/// A word of what a thing is made of, wherever a file may say one: the stone and the brick under a mud wall are words of the picture and no box's; glass and rubber, which a hit reads, a box may say.
#[test]
fn a_thing_is_made_of_a_word_there_is() {
    let shaped = levels().into_iter().find_map(|(n, d)| { let i = d.get("boxes").items().iter().position(|b| b.get("shape").get("with").at(0).has("material"))?; Some((n, format!("boxes.{i}.shape.with.0.stuff"))) });
    let set = levels().into_iter().find_map(|(n, d)| { let sets = d.get("scenarios").get("kitView").get("sets"); let k = sets.members().iter().find(|(_, s)| !s.get("boxes").items().is_empty())?.0.clone(); Some((n, format!("scenarios.kitView.sets.{k}.boxes.0.stuff"))) });
    let (shaped, set) = (shaped.expect("a level with a shape's piece that says its paint"), set.expect("a level with a set of the kit picture's"));
    let hall = |place: &str| (String::from("library-hall.json"), String::from(place));
    for (file, place) in [hall("boxes.0.stuff"), hall("props.0.parts.0.stuff"), hall("materials.wood.stuff"), shaped, set] {
        for (word, bad) in [("mixed", true), ("banana", true), ("mud brick", true), ("field stone", true), ("waits", false), ("veneer", false), ("fabric", false), ("steel", false), ("glass", false), ("rubber", false), ("paint", false), ("plain", false), ("slab", false), ("enamel", false), ("plaster", false)] {
            let t = told(&said(&file, &place, json::s(word)));
            assert_eq!(t.len(), usize::from(bad), "{file} {place} {word}: {t:?}");
            assert!(t.iter().all(|t| t.ends_with(".stuff")), "{file} {place} {word}: {t:?}");
        }
    }
    assert_eq!(told(&said("library-hall.json", "boxes.0.does", json::s("banana"))).len(), 1);
}

/// The kit picture's block, which no rule of the game reads: a word of the loader's own, and a member of the wrong kind.
#[test]
fn the_kit_pictures_block_is_held_to_its_form() {
    let (file, data) = levels().into_iter().find(|(_, d)| { let y = d.get("scenarios").get("kitView").get("ways").at(0); y.has("door") && !y.has("set") && !y.has("add") }).expect("a way on a level's own door");
    let way = "scenarios.kitView.ways.0";
    assert_eq!(told(&data), [""; 0]);
    assert_eq!(told(&said(&file, &format!("{way}.add"), json::obj(vec![("boxes", json::arr(vec![])), ("laid", json::b(true))]))), ["scenarios.kitView.ways[0].add.laid"]);
    assert_eq!(told(&said(&file, &format!("{way}.door"), json::n(7.0))), ["scenarios.kitView.ways[0].door"]);
    assert_eq!(told(&said(&file, &format!("{way}.dor"), json::s("a"))), ["scenarios.kitView.ways[0].dor"]);
}

/// What the game lets go of is held to its kind before it goes.
#[test]
fn a_member_the_game_lets_go_is_still_read() {
    for (file, place, at) in [("office.json", "boxes.0.floor", "floor"), ("tut-road.json", "volumes.0.kind", "kind"), ("office.json", "marks.0.yaw", "yaw"), ("office.json", "surfaces.zones.0.notes", "notes"), ("office.json", "scenarios.nightBreach.lights", "lights")] {
        let t = told(&said(file, place, json::b(true)));
        assert_eq!((t.len(), t[0].ends_with(at)), (1, true), "{file} {place}: {t:?}");
        let t = told(&said(file, &format!("{place}x"), json::b(true)));
        assert_eq!((t.len(), t[0].ends_with(&format!("{at}x"))), (1, true), "{file} {place}x: {t:?}");
    }
}
