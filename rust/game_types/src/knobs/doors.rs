//! The numbers of doors.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// A DOOR OPENED INTO A MAN MOVES HIM AND STARTLES HIM; A DOOR KICKED INTO HIM THROWS HIM (the maintainer: 'opening a door into the enemy should majorly
///  confuse them' · 'kicking down a door when an enemy on the side should stun or knock them out' · having read the first cut, where a pushed door folded a
///  man for two seconds: 'opening a door into an enemy standing on the other side should just alert him, it's not like we're slamming these doors' ·
///  'confusion here being put on alert but being very confused and investigating what's going on'). A leaf DRIVEN OPEN — walked open by a man on its other
///  face, turned by a hand, pushed, shoved or flung by a door order, shouldered at a run, booted — wins against whoever stands in its swing:
///  he is carried clear ahead of it and along it toward its free end, and what it did to
///  him is handed out on his contact (`struck`) for the door-throw rule (`fittings/doorthrow.rs`) to make of it, by HOW the leaf was driven —
///    OPENED (walked, handed, pushed, shoved, flung)   he is moved aside and no more. A defender is startled for `startle` s and goes to see what is at
///                                                     the door (suspicious, the doorway his place to look): no fold, no blindness.
///    SLAMMED (a leaf in a kicked flight that is no     folded and blind for `push_secs`. The tier is the LEAF's (`since_bash`), never whoever last wrote
///      boot's: a shoulder at a run, a door ripped       the drive: the runner's own body pressing after his shoulder is the same slam.
///      or flung open at that pace)
///    BOOTED (a boot, the ram, a shell)                folded and blind for `kick_secs`, and on a knee if it met him before it had turned `floor_rad` from
///                                                     where it was booted (he stood right at the door)
///  A drive strikes a man ONCE; after that it goes on carrying him.
///  By behaviour, never by hit points: nothing here hurts anybody. The same for whoever it meets — a defender, Sam, a teammate, a civilian.
///  NOTHING AT ALL: a leaf eased (a hand slower than `hand_rate`, a crouched or silent man's body — `quiet` on the drive — or any leaf slower than `min_speed` where
///  it meets him), pulled (it passes the living on that side as ever), swinging shut, or one the man walked into himself; a leaf opened short of a boot by a
///  CALM man (MAN_CALM: staff not alert or searching, any civilian — he opens a door with his hand at a walk: the patrol's round, the cleaner's, move nobody
///  and raise no alarm of their own, and a man hidden behind a door stays hidden); a leaf a man of HIS OWN side opened any way short of a boot (the squad's
///  file through a door, a pair's shove into a room the other pair is already in, are not a brawl — a leaf one of them BOOTS meets him like anyone); a man
///  already folded (he is carried, whoever's leaf it is and however quietly it comes: a folded man holds no door shut) or just up from it (`again` s: no
///  second throw — an opened leaf moves and startles him like anyone); a man in somebody's arm. A man with no floor to be moved onto (his back
///  to a wall) is struck where he stands and the leaf rests on him.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct DoorthrowK {
    /// s folded and blind: a leaf shouldered open at a run into him · a booted one
    #[k(2.0)] pub push_secs: f64,
    #[k(4.0)] pub kick_secs: f64,
    /// a leaf merely OPENED into a defender: s he stands startled before he goes to look · the awareness he is left with (suspicious: the decoy's own)
    #[k(1.0)] pub startle: f64,
    #[k(0.5)] pub wary: f64,
    /// m/s a swinging leaf must still be doing where it meets him (slower, it comes to rest against him as ever)
    #[k(0.9)] pub min_speed: f64,
    /// rad/s a hand must turn a leaf at for it to be a push (the silent ease is 0.55; the hand's own push 2.6)
    #[k(1.2)] pub hand_rate: f64,
    /// rad a booted leaf may have turned when it meets him for him to have stood right at the door: he goes onto a knee for it
    #[k(0.6)] pub floor_rad: f64,
    /// m/s a leaf carries a thrown man ahead of it at most (a frame's turn beyond that and it rests on him this frame) · m/s he is moved along it toward its free end
    #[k(9.0)] pub rate: f64,
    #[k(1.5)] pub slide: f64,
    /// s after a fold has run out in which no leaf throws him again
    #[k(3.0)] pub again: f64,
    /// the awareness a man comes back with when the leaf was a foe's (the ring's own: alert)
    #[k(0.92)] pub aware: f64,
}
