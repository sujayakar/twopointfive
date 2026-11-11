//! The numbers of slotsetup.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// What it answers (the maintainer): "repeating the current selection open up a menu for configuring the current one", "can allow us to pick between
/// auto/semi (get rid of z)", "also lets us pick attachments better", and of the light: "for L can we move that to the pistol/gun's equipment menu?".
///   OPEN     the digit of the gun that is IN HIS HANDS (1 the pistol, 2 the long gun; `player_gun_in_hand`: not holstered, not slung) opens that gun's
///            setup — and so does that digit pressed again while the gun is still coming to his hands (the maintainer, playing it:
///            "when on pistol, double tapping 2 quickly switches but doesn't open the menu"): 2 2 from the pistol is the rifle and its setup, and a
///            row's digit after them is the row. Only where HIS OWN PRESS of that digit drew the gun, and only while that draw lasts: a gun the
///            game put in his hand (the last can thrown, a grab let go) and a later lock on his hands (a tool going back on his pack, a throw's
///            swing) do not open it: while a gun he did not draw himself is still coming to his hands, and for its first `settle`
///            seconds in them, its digit does nothing.
///            A can has nothing to set: its digit is the slot it is.
///   ROWS     what the gun has that can be set in the field, each on a digit of its own that is the same on every gun:
///              1  the selector   (semi / auto: a gun with two positions)
///              2  the load       (what its launcher has on the right button: the next load round the tube that he has any of — one he is out of is
///                                 stepped over, and with nothing else to put up the row has no digit)
///              3  the light      (a kit that carries one)
///            The digit steps its row and the setup shuts. So the two presses are always the same two: 2 1 is the
///            rifle's selector, 2 2 its next load, 2 3 its light; 1 3 the pistol's light.
///   SHUT     by a pick; by the back key; by the trigger, the right button or a reload (none of them is taken: the round goes — and a
///            round that goes later, off a click the kit's handling kept through the draw, shuts it as it leaves); by a digit
///            that is no row (the gun's own shuts it and no more, any other is the slot it always was); when what is in his hands changes or something
///            takes them; when the order menu or the Space list comes up (the digits are theirs). No clock runs on it: it is a MODE he
///            is in until one of those — the maintainer: "the timer is bad. we should just
///            have it switch a mode", "it should be something that pops up above the equipment menu rather than always taking up space".
/// His feet, his aim and his trigger are his own throughout: nothing is held, nothing is frozen, and no key is read here but the digits and the back key.
/// THE RULE IS THE RUST GAME'S (rust/game_rules/src/outfit/slotsetup.rs): this file keeps its table and how a line of words names its keys. What the HUD shows is what the game
/// says (`setup_view`, game.view.kit.setup), drawn by ui/handbox.ts: the setup's rows stand above the kit column while it is up and at no other time; the box of
/// the thing in his hands says what is set in plain words.
/// THE TWO KEYS IT REPLACES (Z the selector, L the light) are the board's to drop, not the game's: the game goes on asking for
/// 'KeyZ' and 'KeyL' and meaning what it meant, so a recording plays, and a player who wants either on a key puts its row on one in Options.
/// State: `setup` and `setup_key` (his last press of a gun's digit that was a draw) on the seat's hand, `setup_hand` on the Player (which gun the hook last found in his hands, since when, and — where his own press drew
/// it — the time that draw is over, `own`), null on a fresh one.
/// NOTHING IS UP AFTER A LOAD: a file's load builds the player again; a save kept in memory puts him back as he stood, the setup too, so
/// `put_back` drops it there.
/// No dice, no clock but the game's.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct SlotsetupK {
    /// seconds a gun he did not draw himself is in his hands before its digit opens the setup
    #[k(0.35)] pub settle: f64,
}
