//! A man is staged with what his card shows: the desk and a start are laid the same record, and on every mission of the list whose cards can be changed the men stand
//! with the cards the desk says they go with.
use game::kit;
use game::view::{Pressed, StartView, TeamView};
use game_start::host::desk::Desk;
use game_start::host::kept;
use game_start::stand::{stand, StartOpts};
use game_start::{files, json};

fn text(b: &mut Vec<u8>, s: &str) { b.extend_from_slice(&(s.len() as u32).to_le_bytes()); b.extend_from_slice(s.as_bytes()); }
fn ask(n: u32, texts: &[&str], more: &[u32]) -> Vec<u8> {
    let mut b = n.to_le_bytes().to_vec();
    for t in texts { text(&mut b, t); }
    for m in more { b.extend_from_slice(&m.to_le_bytes()); }
    b
}
fn answer<T: 'static>(d: &mut Desk, b: &[u8]) -> T { *d.answer(b).expect("an ask the desk has").itself().downcast().expect("the answer it is asked for") }

/// what a machine keeps: every man's card moved off the issued one, colors too, the stealth card as well
const KEPT: &str = r#"{"v":2,"stealth":{"weapon":"pistol","suppressed":true,"sideCan":true,"light":false,"nvg":true,"bangs":1,"smoke":2,"frags":0,"shells":0,"rings":0,"cams":0,"charges":0,"walls":0,"wand":true,"big":"none","primary":"mk18","secondary":"g19","suit":"navy","trim":"red"},
"members":[{"weapon":"rifle","suppressed":false,"sideCan":true,"light":true,"nvg":false,"smoke":2,"bangs":0,"frags":1,"shells":0,"primary":"mk12","secondary":"hk45","suit":"navy","trim":"red","big":"hammer"},
{"weapon":"rifle","primary":"mp7","suppressed":true,"sideCan":true,"light":false,"nvg":true,"bangs":1,"charges":2,"frags":0,"suit":"olive","trim":"amber"},
{"weapon":"pistol","secondary":"g18","suppressed":true,"sideCan":false,"light":false,"nvg":true,"bangs":2,"charges":0,"frags":1,"big":"shotgun"},
{"weapon":"rifle","primary":"m1014","light":true,"nvg":false,"bangs":0,"charges":1,"frags":2,"suit":"khaki"},
{"weapon":"rifle","primary":"mk46","light":true,"nvg":true,"bangs":1,"charges":1,"frags":1,"trim":"white"}]}"#;

#[test]
fn a_man_is_staged_with_what_his_card_shows() {
    let held = kept::Kept { cards: Some(String::from(KEPT)), ..Default::default() };
    let mut d = Desk::new();
    d.machine(&format!("{{\"cards\":{}}}", kept::text(KEPT)), "").expect("a record");
    let list: StartView = answer(&mut d, &ask(11, &[""], &[]));
    let character = files::CharacterFiles::read();
    let (mut missions, mut men) = (0, 0);
    for row in &list.rows {
        // (a level that issues its own kit hands it out over the cards: the desk is told the block by the page, which reads the file)
        if files::level_text(&row.level).contains("\"issue\"") || row.as_issued { continue; }
        for size in 0..=4u32 {
            let _: Pressed = answer(&mut d, &ask(6, &[&row.id, "", ""], &[0]));
            let took: Pressed = answer(&mut d, &ask(9, &[], &[size]));
            if !took.ok || (size as f64 != row.squad && size != 3) { continue; }
            let team: TeamView = answer(&mut d, &ask(8, &[], &[]));
            let data = json::parse(&files::level_text(&row.level)).expect("a level's file");
            let s = stand(&data, character.assets(), &StartOpts { seed: 7, link: Some(team.link.clone()), kept: held.clone(), ..StartOpts::default() });
            let g = &s.game;
            let ops = g.m.kit.kit_staged.ops.clone();
            assert_eq!(ops.len(), size as usize, "{} with {size}", row.id);
            for (i, &gd) in ops.iter().enumerate() {
                let wears = kit::kit_of(g, gd).unwrap_or_else(|| panic!("{} with {size}: man {} has no card", row.id, i + 1));
                assert_eq!(kept::written(&wears), team.men[i + 1].worn, "{} with {size}: man {}", row.id, i + 1);
                men += 1;
            }
            let pl = g.player;
            assert_eq!((g.pl[pl].canisters, g.pl[pl].frags), (team.smoke, team.frags), "{} with {size}: what you carry", row.id);
            missions += 1;
        }
    }
    assert!(missions >= 10 && men >= 20, "{missions} missions, {men} men");
}
