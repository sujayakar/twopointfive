//! A START HANDED COMMANDS READS NO DIFFICULTY OFF ITS ADDRESS (`StartOpts.difficulty`, tests/hard.rs for the reading itself).
//! (A file of its own: this stand holds no `Play`, so it must not run beside the tests that do: the character's clips are one store of the process.)
use game::difficulty::{difficulty_now, DifficultyNow};
use game_start::files;
use game_start::handed::stand_handed;
use game_start::stand::StartOpts;

const LEVEL: &str = "depot.json";
const LINK: &str = "?levelFile=depot.json&roomclear&squad4";

fn pack(parts: &[&[u8]]) -> Vec<u8> { let mut b = (parts.len() as u32).to_le_bytes().to_vec(); for p in parts { b.extend((p.len() as u32).to_le_bytes()); b.extend(*p); } b }

/// A FAILURE MEANS: a host that handed commands has the address's difficulty laid over what they decided (a page's recipe, what it keeps). The control hands none: then the address is
/// all there is, and it is read.
#[test]
fn what_a_hosts_commands_wrote_stands() {
    let ch = files::CharacterFiles::read();
    let a = ch.assets();
    let level = files::level_text(LEVEL);
    let link = format!("{}&hard", LINK);
    let handed = |commands: &[u8]| pack(&[level.as_bytes(), link.as_bytes(), a.mannequin.text.as_bytes(), a.mannequin.bin.unwrap(), a.cast.map_or(&b""[..], |c| c.text.as_bytes()), a.cast_look.map_or(&b""[..], |c| c.text.as_bytes()), commands]);
    // (slot 1: this stand holds no `Play`, whose games stand in slot 0)
    let o = StartOpts { seed: 1, slot: 1, ..Default::default() };
    let s = stand_handed(&handed(b""), &o, &mut |_, _, _| {});
    assert_eq!(difficulty_now(&s.game), DifficultyNow::Hard, "no commands handed: the address's word stands");
    assert!(s.game.k.pressure.on && s.game.k.digin.on, "…with hard's rows");
    drop(s);
    let s = stand_handed(&handed(b"one"), &o, &mut |g, _, _| g.k.digin.on = true);
    assert!(s.game.k.digin.on && !s.game.k.pressure.on, "what the host's commands wrote stands, and the word is not laid over it");
}
