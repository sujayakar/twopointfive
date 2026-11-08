// THE SLOT IN HAND, PRESSED AGAIN, OPENS ITS SETUP (SLOTSETUP.on: on in the page, ?slotsetup=0; off here and on the frozen stand, TPS_SLOTSETUP=1).
//
// What it answers (the maintainer, 10-22): "repeating the current selection open up a menu for configuring the current one", "can allow us to pick between
// auto/semi (get rid of z)", "also lets us pick attachments better", and of the light: "for L can we move that to the pistol/gun's equipment menu?".
//
//   OPEN     the digit of the gun that is IN HIS HANDS (1 the pistol, 2 the long gun; Player.gunInHand: not holstered, not slung) opens that gun's
//            setup — and so does that digit pressed again while the gun is still coming to his hands (`draw`; the maintainer, 10-22, playing it:
//            "when on pistol, double tapping 2 quickly switches but doesn't open the menu"): 2 2 from the pistol is the rifle and its setup, and a
//            row's digit after them is the row. Only where HIS OWN PRESS of that digit drew the gun, and only while that draw lasts: a gun the
//            game put in his hand (the last can thrown, a grab let go) and a later lock on his hands (a tool going back on his pack, a throw's
//            swing) do not open it for one press. With `draw` off, not in the gun's first `settle` seconds in his hands nor while the rifle is
//            still coming round off its sling: there a double-tapped digit draws the gun and no more, as the first build had it.
//            A can has nothing to set: its digit is as it was.
//   ROWS     what the gun has that can be set in the field, each on a digit of its own that is the same on every gun:
//              1  the selector   (semi / auto: a gun with two positions)
//              2  the load       (what its launcher has on the right button: the next load round the tube that he has any of — one he is out of is
//                                 stepped over, and with nothing else to put up the row has no digit)
//              3  the light      (a kit that carries one)
//            The digit steps its row and the setup shuts (`stay`: it stays up for another). So the two presses are always the same two: 2 1 is the
//            rifle's selector, 2 2 its next load (the '2 again' the build had, one press on), 2 3 its light; 1 3 the pistol's light.
//   SHUT     by a pick; by the back key (backkey.ts); by the trigger, the right button or a reload (none of them is taken: the round goes — and a
//            round that goes later, off a click the kit's handling kept through the draw, shuts it as it leaves); by a digit
//            that is no row (the gun's own shuts it and no more, any other is the slot it always was); when what is in his hands changes or something
//            takes them; when the order menu or the Space list comes up (the digits are theirs). No clock runs on it (`pop`, below): it is a MODE he
//            is in until one of those — the maintainer, 10-22, after playing the first build's second and a half: "the timer is bad. we should just
//            have it switch a mode", "it should be something that pops up above the equipment menu rather than always taking up space". With `pop`
//            off it also shuts by itself `secs` after it opened or was last used (0: never), as that build did.
// His feet, his aim and his trigger are his own throughout: nothing is held, nothing is frozen, and no key is read here but the digits and the back key.
// THE RULE IS THE RUST GAME'S (rust/game_rules/src/slotsetup): this file keeps its table and how a line of words names its keys. What the HUD shows is what the game
// says (rust `setup_view`, game.view.kit.setup), drawn by ui/handbox.ts: the setup's rows stand above the kit column while it is up and at no other time; the box of
// the thing in his hands says what is set in plain words.
// THE TWO KEYS IT REPLACES (Z the selector, L the light) are the board's to drop, not the game's (host/keymap.ts `setFree`): the game goes on asking for
// 'KeyZ' and 'KeyL' and meaning what it meant, so a recording made before plays, and a player who wants either on a key puts its row on one in Options.
// State: three fields on the Player (`setup`; `setupHand`: which gun the hook last found in his hands, since when, and — where his own press drew
// it — the time that draw is over, `own`; `setupKey`: his last press of a gun's digit that was a draw), null on a fresh one.
// NOTHING IS UP AFTER A LOAD: a file's load builds the player again; a save kept in memory puts him back as he stood, the setup too, so
// savestate.ts drops it there (Checkpoints.loadMem).
// No dice, no clock but the game's.

export const SLOTSETUP = {
  on: false,
  /** the setup is a mode that pops up above the kit column: no clock runs on it (`secs` is not read) and its rows are drawn only while it is up
   *  (ui/handbox.ts). Off in the module and on the frozen stand with `on` (`TPS_SLOTSETUP=1` raises both); on in the page (`?setuppop=0`: the rows in
   *  the box and the clock, as the first build had them); a recording from before the key replays with it off */
  pop: false,
  /** with `pop` off: seconds the setup stays up with nothing pressed in it (0: until it is shut) */
  secs: 1.5,
  /** a pick leaves the setup up for another (false: a pick shuts it — the same two presses every time) */
  stay: false,
  /** the gun's digit pressed again while the gun is still coming to his hands opens its setup at once: a double tap off another slot is the draw
   *  and the setup (`settle` and the swap's own time are not waited for, where his own press drew it). Off in the module and on the frozen stand
   *  (`TPS_SETUPDRAW=1`, a word of its own: `TPS_SLOTSETUP=1` alone is the rule with the double tap as the draw); on in the page (`?setupdraw=0`:
   *  a double-tapped digit is the draw and no more); a recording from before the key replays with it off */
  draw: false,
  /** with `draw` off: seconds a gun is in his hands before its digit opens the setup (a double-tapped digit on the draw is the draw and no more) */
  settle: 0.35,
  /** THE BOX COUNTS THE ROUNDS IN THE GUN OVER WHAT ITS MAGAZINE HOLDS, WITH A BOX FOR EACH SPARE MAGAZINE BESIDE IT (ui/handbox.ts: 31/30 and two boxes,
   *  each as full as that magazine is; the maintainer of the box that said 31 / 60: "we should say 31/30 and then two spare magazines on the side"). A gun
   *  fed a shell at a time keeps its two numbers: what it carries is loose shells, and the second number is the count of them. The
   *  box's alone: nothing in the game reads it. Off here and on the frozen stand; on in the page (`?magboxes=0` = rounds in the gun over the rounds in all
   *  the spares, as it was); a recording from before the key replays with it off */
  mags: false,
};
export type SetupGun = 'pistol' | 'rifle';
const BREAK = (() => { try { return String((globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.SLOTSETUP_BREAK ?? '') === '1'; } catch { return false; } })();

/** the presses that put the next load on the launcher, as a line of words names them ('2 again' in the build before) */
export const loadKeys = (): string => SLOTSETUP.on && !BREAK ? 'press 2, then 2' : '2 again';   // ('press' and 'then' before a digit: host/keymap.ts names the key each is on)
/** where a kit's weapon light is worked, as a card's note says it ('on L' in the build before) */
export const lightKeys = (): string => SLOTSETUP.on && !BREAK ? 'its gun\'s key again, then 3' : 'on L';
