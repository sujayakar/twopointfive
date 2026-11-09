//! THE ADDRESS'S WORDS AT A START (game_start host/words.rs), through the door the page's start goes by.
//! (A file of its own: this stand holds no `Play`, so it must not run beside the tests that do: the character's clips are one store of the process.)
use game_start::files;
use game_start::handed::stand_handed;
use game_start::stand::StartOpts;

fn pack(parts: &[&[u8]]) -> Vec<u8> { let mut b = (parts.len() as u32).to_le_bytes().to_vec(); for p in parts { b.extend((p.len() as u32).to_le_bytes()); b.extend(*p); } b }

/// THE RULE: what an address says of a number is on the game when it stands, and that holds for a word read off the list, for a number by name and for a pick; a number the tables don't hold is passed
/// over. A FAILURE MEANS: a word for a switch does nothing on the page, or a level laid the old number over it. The control is the same stand with no words.
#[test]
fn a_start_stands_with_what_its_address_says() {
    let ch = files::CharacterFiles::read();
    let a = ch.assets();
    let level = files::level_text("depot.json");
    let stand = |words: &str| {
        let link = format!("?levelFile=depot.json&roomclear&squad4{words}");
        let b = pack(&[level.as_bytes(), link.as_bytes(), a.mannequin.text.as_bytes(), a.mannequin.bin.unwrap(), a.cast.map_or(&b""[..], |c| c.text.as_bytes()), a.cast_look.map_or(&b""[..], |c| c.text.as_bytes()), b""]);
        stand_handed(&b, &StartOpts { seed: 1, slot: 1, ..Default::default() })
    };
    let plain = stand("");
    let read = |g: &game::game::Game, n: &str| game_start::host::knobs::get(g, n).unwrap();
    assert_eq!(read(&plain.game, "cmdring.twice"), 1.0);
    let (was_rate, was_lean) = (read(&plain.game, "sneak.seen_rate"), read(&plain.game, "fire.look.lean"));
    assert_ne!(was_lean, 0.07);
    drop(plain);
    let s = stand("&qtwice=0&nosquadthrow&firelean=0&seenrate=0.5&knob.pressure.memory_secs=123&knob.tune.wind=0.25&training=green&nerve=brittle");
    assert_eq!(read(&s.game, "cmdring.twice"), 0.0, "an off word");
    assert_eq!(read(&s.game, "squadthrow.on"), 0.0, "a word that's there");
    assert_eq!(read(&s.game, "fire.look.lean"), 0.07, "a word for the number from before its rule");
    assert_eq!(read(&s.game, "sneak.seen_rate"), 0.5, "a share");
    assert_ne!(was_rate, 0.5);
    assert_eq!(s.game.k.pressure.memory_secs, 123.0, "a number of the tables by its name");
    assert!(matches!(s.game.k.morale.training, game::morale::Training::Green), "a pick");
    assert!(matches!(s.game.k.morale.nerve, game::morale::Nerve::Brittle), "a pick");
}
