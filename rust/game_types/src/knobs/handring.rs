//! The numbers of handring.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// HIS OWN ACTIONS ARE A RING ON ONE KEY (the maintainer, 10-22: "radial menu for our own actions, remove existing menu in top left and awkward scrolling
/// thing", "move whistle to our actions and remove v binding", "rework equipment selection and move our actions to E", "i think same would go for our
/// actions (1-5 with the ring open)", "we can make our actions and squad actions symmetric, where maybe we have some way of showing in the UI … what the
/// default action might be. or maybe that could be in the top left, which might be fine", "do we need a way to cancel all in progress stuff? tab could be
/// that for us").
/// THE RULE. The interaction key's TAP does the default action, as it always did. The key kept down a moment puts his rows up as a ring at the cursor
/// (interact.ts: the ring cmdring.ts CMDRING.hands drew as a prototype, now standing without the squad's switch — with this rule on, that prototype's
/// own switch and tick change nothing), and the ring is read by the press rule agreed with the squad ring's owner: let go with the cursor out on a wedge
/// and that wedge is done; let go in the middle and the ring STAYS with no key held. Standing, a digit 1 to 5 does its wedge at once, a left click does the lit one (and is not his trigger), the key again does the lit
/// one, a right click or the back key (Tab) puts it away with nothing done. The held list with its wheel and its cancel row is gone.
/// THE WHISTLE IS A WEDGE (`view.hand`, rust interact whistle_wedge): the LAST wedge, always the same one, wherever fewer rows than wedges are on offer — so on open
/// floor the ring is that one wedge. It is no row of the list, so it is never what a tap does (a noise is chosen, never fallen into) and nothing that
/// reads the rows meets it. Where the rows fill every wedge, and while a verb runs, there is no whistle: a step away or the verb's end gives it back. The key the whistle had is taken off it by the page's board (host/keymap.ts: its row has no key until the player gives
/// it one); the game still answers its old code, so a recording and every check that presses it are as they were.
/// THE CORNER SHOWS ONE LINE: what a tap would do now, and nothing where a tap does nothing. A running verb's own
/// row and the squad's block under it are as they were.
/// THE KEY: while this rule is on the whistle's row has no key, and while the empty-hands slot is on as
/// well the E key gives the interaction's code and the Space key gives the game nothing; the game goes on asking for the code it always asked for
/// On (`?handring=0`: the list on a held Space, the corner list, the
/// whistle on its key, as it was). Registered in knobs.ts; a recording from before the key replays with it off.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HandringK {
    /// the interaction key held is a ring that stays; the whistle is its last wedge; the corner shows the default action alone
    #[k(true)] pub on: bool,
    /// the key down this long (s) puts the ring up; let go sooner it is the tap. The squad ring's own number for a ring that stays
    #[k(0.3)] pub hold_secs: f64,
    /// s more than the hand's own time that a turn may add (a half turn at the plant's 10 rad/s is 0.31 s, the step in the rest)
    #[k(0.45)] pub turn_max: f64,
}
