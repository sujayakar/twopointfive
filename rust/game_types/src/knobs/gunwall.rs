//! The numbers of gunwall.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// A RAISED WEAPON COMES DOWN AT A WALL. A shouldered rifle's muzzle stands 0.63 m ahead of the man who holds it and an aimed pistol's 0.64; a man can stand
///  with his centre 0.3 m off a wall — so every man who turned to cover a line with a wall, a jamb or a door leaf inside that reach was DRAWN with his gun in it
///  or out through its far face (39 play-throughs of the one build: 3.1 % of all armed man-frames; nine in ten of them the shouldered carriage). Rounds have a
///  rule for the pressed muzzle (combat.ts armThrough: the round dies in the panel) and so has the weapon light; this is the GUN's:
///  1. THE LOW READY. A gun that is up (the aim; the high port of a stack or the high ready) and whose own line meets a static or a leaf inside its reach is
///     brought to the low ready — through Guard.muzzleDown, the bit a colleague in the line already sets: the pose, every fire gate and
///     the visible laser read it, so nothing new has to learn about a lowered gun. Two lines are asked (walledNow): the gun AS DRAWN once it is up and settled
///     (exact at a jamb's edge), and otherwise the carriage he WANTS, worked out from his feet and his aim (a gun already down would report nothing in the way
///     if it were asked as drawn, and come straight back up) — with a lead for the speed he closes at and a band between down and up, so it comes down before
///     the wall and does not flutter at the edge of the reach. A man whose point of aim IS that surface (within `atM` of where the line meets it: the shotgun
///     on a lock — doortools.ts gunStep) keeps it up while the muzzle itself is short of it: that muzzle is there on purpose. One that would stand IN the
///     surface comes down like any other (a man walking at the door he watches).
///     A MAN IN A FIGHT (`hot`: the alert branch, a script's `over`) is treated more narrowly: his gun comes down only once it has stood IN a panel a round
///     would have died in anyway (armThrough's own test), and he then moves for a line (guards.ts `poked`; roomfight.ts walks on / steps aside).
///  2. THE TUCK. The low ready itself reaches 0.43‥0.51 m — still past a wall he is pressed to. CharacterAnimator.gunWall (0‥2, from how much room the low
///     carriage's line has) pulls the low carriages in toward character.ts WALL_TUCK: at 1 the muzzle is inside his own collision circle, and from 1 to 2 the
///     hand is drawn further back still (WALL_TUCK.over) for a torso twisted toward the wall.
///  Sam: the same test, the same low ready, and no round while it is down — but for the BREACHING SHOTGUN LAID ON A LOCK (`breachAim`: the raised
///  gun's own line meets a locked, latched leaf where a shell takes its keep): that gun stays up however close he stands, pressed to the leaf included, and the
///  shell goes — as a teammate's on the same lock always did. A breach shot at a lock always fires. Pinned men are left alone (they cannot step for a line).
///  Off (the table's own value, so a host that never says otherwise is unchanged): nothing asked, muzzleDown and gunWall never touched — the game as it was.
///  The page turns it on and so does the harness's shipped stand; its frozen stand holds it off
///  `downM` / `upM`: the room past the wanted muzzle under which the gun comes down / over which it may come back up, metres. `leadS`: seconds of his closing
///  speed added to both (the carriage takes about that long to change). `tuckM`: the room the tucked muzzle keeps; `tuckLeadS`: its lead.
///  `fightS`: a fight anywhere on the floor (a man gone alert, a scripted man with one in his sights) makes EVERY man a man in a fight for this long after it
///  was last seen — the calm man's margins bring a gun down a stride early, and a stride early at a jamb with defenders two rooms off is a shot not taken
///  (0 = only the man who has a target himself).
///  `mark`: the player's aim ring wears a bar while his gun cannot get a round out (game.view.aim: player::gun_shut; the overlay's, nothing in the game reads it — the
///  words a dead click files are in no list the page draws). Off here; the page turns it on (?nogunmark = off).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct GunwallK {
    #[k(0.03)] pub down_m: f64,
    #[k(0.12)] pub up_m: f64,
    #[k(0.2)] pub lead_s: f64,
    #[k(0.5)] pub at_m: f64,
    #[k(0.04)] pub tuck_m: f64,
    #[k(0.12)] pub tuck_lead_s: f64,
    #[k(5.0)] pub fight_s: f64,
}
