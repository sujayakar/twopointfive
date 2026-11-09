//! The numbers of lived.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// LIVED-IN ROOMS (the levels goal; the maintainer, 10-10: "the office level is awesome and it has a lot of personality per room, we should makes sure all levels are
/// like that"): the furniture, clutter, floor pieces, loose props and lamps that make a room of another level ITS room, as the office's rooms are — a bunk room's
/// bunks, a radio room's bench, a tyre store's stacks. They ride in the level FILE under one name and the loader (scene/levelLoad.ts) leaves them out with the switch
/// off, so every other box, prop, light and fixture keeps its index and the level is the one it was before there were any:
///   a BOX whose editor group is 'lived' or starts 'lived.' ('lived.<room>')   ·   a PROP, LIGHT or SWITCHABLE whose id starts 'lived.'
/// ONE SWITCH. On at boot, before the
/// level loads (host/profile.ts pageSwitches: on unless the URL carries ?lived=0), and TPS_LIVED=1 in front of any bun tool raises it (read here, so a tool that
/// only parses a level file follows it too; tools/qa/headless.ts pins it with the other stand words). Furnished rooms are real things — cover, things to walk
/// round, things that break — so on is NOT the off game by digest; off is. levels stands holds off against git and the marks, doors and lanes with it on.
/// No imports here: scene/, game/ and the tools all read it.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct LivedK {
    #[k(true)] pub on: bool,
}
