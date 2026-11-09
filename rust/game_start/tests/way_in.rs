//! THE WAY IN YOU PICK IS THE WAY THE TEAM STANDS AT: every room clear of the game's own list, every way its sheet offers, pressed on the sheet and stood from the link the sheet then deploys by.
use game::game::Game;
use game::start_list::Sitting;
use game::view::StartRow;
use game_start::json::{self, Json};
use game_start::play::{Opts, Play};

const SEEDS: [u32; 2] = [1, 9];

fn place(j: &Json) -> [f64; 2] { let v = j.v3().expect("a start's place"); [v[0], v[2]] }
fn apart(a: [f64; 2], b: [f64; 2]) -> f64 { (a[0] - b[0]).hypot(a[1] - b[1]) }
fn level_of(row: &StartRow) -> Json { json::parse(&game_start::files::level_text(&row.level)).unwrap_or_else(|e| panic!("{}: {}", row.level, e)) }
/// the level file's starts: its own, then its others (a level with no `team` of its own, the office, has one way, the night breach's: none here)
fn starts_of(data: &Json) -> Vec<Json> {
    let r = data.get("scenarios").get("roomClear");
    if r.get("team").get("sam").v3().is_none() { return Vec::new(); }
    let mut all = vec![r.get("team").clone()];
    all.extend(r.get("starts").items().iter().cloned());
    all
}
fn room_clears(s: &mut Sitting, g: &mut Game) -> Vec<StartRow> {
    let rows: Vec<StartRow> = s.start(g, "").rows.into_iter().filter(|r| r.kind == "roomclear").collect();
    assert!(rows.len() >= 10, "{} room clears: the reader has lost them", rows.len());
    rows
}
/// the link the sheet deploys the whole team by, once this way is pressed
fn deploy_link(s: &mut Sitting, g: &mut Game, row: &StartRow, way: usize) -> String {
    if !row.ways.is_empty() { assert!(s.press(g, &row.id, "way", &(way + 1).to_string()).is_some(), "{}: the sheet refuses way {}", row.id, way + 1); }
    let link = s.deploy_link(g, &row.id, "", 4.0);
    assert_eq!(link.contains("way="), way > 0, "{} way {}: {}", row.id, way + 1, link);
    link
}
/// the seat and every teammate on this start
fn stands_on(p: &mut Play, start: &Json, what: &str) {
    let (sam, ops) = (place(start.get("sam")), start.get("ops").items());
    let seat = p.seat_at(0);
    assert!(apart(seat, sam) < 0.01, "{what}: the seat stands at {seat:?}, its start is {sam:?}");
    let team = p.team_at();
    assert!(!team.is_empty(), "{what}: nobody came");
    for (k, man) in team.iter().enumerate() {
        match ops.get(k) {
            Some(op) => assert!(apart(*man, place(op)) < 0.01, "{what}: man {k} stands at {man:?}, his place is {:?}", place(op)),
            None => assert!(apart(*man, sam) < 6.0, "{what}: man {k}, who has no place of his own there, stands at {man:?}, far from the start {sam:?}"),
        }
    }
}

#[test]
fn every_way_of_every_room_clear_stands_from_the_sheets_own_link() {
    let (mut g, mut s) = (Box::new(Game::blank()), Sitting::default());
    let mut more = 0;
    for row in room_clears(&mut s, &mut g) {
        let data = level_of(&row);
        let starts = starts_of(&data);
        assert_eq!(row.ways.len().max(1), starts.len().max(1), "{}: the sheet offers {} ways, the level has {}", row.id, row.ways.len(), starts.len());
        more += starts.len().saturating_sub(1);
        // (a way that stood on another's place would hold nothing here)
        for a in 0..starts.len() { for b in 0..a { assert!(apart(place(starts[a].get("sam")), place(starts[b].get("sam"))) > 1.0, "{}: ways {} and {} are one place", row.id, b + 1, a + 1); } }
        for (k, start) in starts.iter().enumerate() {
            let link = deploy_link(&mut s, &mut g, &row, k);
            for seed in SEEDS {
                let what = format!("{} way {} seed {seed} ({link})", row.id, k + 1);
                let mut p = Play::mission(&row.level, &link, &Opts { seed, ..Default::default() });
                assert_eq!(p.way_taken(), k, "{what}: the game says");
                stands_on(&mut p, start, &what);
                // the same again at a restart
                p.run(0.5);
                p.restaged();
                assert_eq!(p.way_taken(), k, "{what}, restarted: the game says");
                stands_on(&mut p, start, &format!("{what}, restarted"));
            }
        }
        // the sheet is left as it was found, for the next row's own link
        deploy_link(&mut s, &mut g, &row, 0);
    }
    assert!(more >= 5, "{more} other ways in all: the reader has lost them");
}

#[test]
fn past_the_last_is_the_last_and_a_word_that_is_no_number_is_the_levels_own() {
    let (mut g, mut s) = (Box::new(Game::blank()), Sitting::default());
    let row = room_clears(&mut s, &mut g).into_iter().find(|r| r.ways.len() > 1).expect("a room clear with two ways");
    let starts = starts_of(&level_of(&row));
    let own = deploy_link(&mut s, &mut g, &row, 0);
    let stand = |word: &str| { let mut p = Play::mission(&row.level, &format!("{own}{word}"), &Opts { seed: 1, ..Default::default() }); let w = p.way_taken(); (w, p.seat_at(0)) };
    let last = starts.len() - 1;
    for (word, way) in [("", 0), ("&way=1", 0), ("&way=0", 0), ("&way=any", 0), ("&way=two", 0), ("&way=", 0), ("&way=99", last), ("&way=99999999999999999999", last)] {
        let (w, at) = stand(word);
        assert_eq!(w, way, "'{word}'");
        assert!(apart(at, place(starts[way].get("sam"))) < 0.01, "'{word}' stands at {at:?}");
    }
}

#[test]
fn only_a_room_clear_has_ways() {
    // (the control: the word on a rescue and on a quiet mission stands on the level's one start)
    let (mut g, mut s) = (Box::new(Game::blank()), Sitting::default());
    let rows = s.start(&mut g, "").rows;
    for kind in ["rescue", "quiet"] {
        let row = rows.iter().find(|r| r.kind == kind).unwrap_or_else(|| panic!("no {kind} on the list"));
        let link = s.deploy_link(&mut g, &row.id, "", 1.0);
        let at = |words: &str| { let mut p = Play::mission(&row.level, &format!("{link}{words}"), &Opts { seed: 1, ..Default::default() }); (p.g.stood.way, p.seat_at(0)) };
        let (plain, asked) = (at(""), at("&way=2"));
        assert_eq!(asked.0, 0, "{}: a way is asked of a mission with one", row.id);
        assert_eq!(plain.1, asked.1, "{}: the word moved the start", row.id);
    }
}
