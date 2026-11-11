//! The numbers of teampace.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct TeampaceK {
    /// a teammate's walk, × the player's (tune.playerWalk)
    #[k(0.8)] pub k: f64,
    /// a follower off his place closes at up to this × the player's walk (at 1 a file strings out behind a walking player: nobody closes on a man at his own pace)
    #[k(1.3)] pub follow_k: f64,
    /// followers run while the player sprints; `behindM`: how far off his place a man starts to
    #[k(1.0)] pub behind_m: f64,
    /// a man who lost his man this long ago still moves at the fight's pace (s)
    #[k(2.0)] pub fight_secs: f64,
    /// … and a man a grenade took off his place goes back to it at the fight's pace, this long after he stopped running from it (s; 0 = at the walk)
    #[k(6.0)] pub back_secs: f64,
    /// a run to cover: the least leg worth running (m), how exposed the floor he stands on must be, how often it is asked (s)
    #[k(2.5)] pub dash_m: f64,
    #[k(0.5)] pub dash_seen: f64,
    #[k(0.25)] pub dash_every: f64,
    /// … and how lately a shot, a charge or a boot must have reached his ears (s)
    #[k(6.0)] pub dash_hot: f64,
    /// The maintainer, of a fight on the pass: "weird really fast squad movement / sprinting that didn't make sense"; asked where, "in a fight running to stack up on a door": three men crossing two to six metres at 4.7 m/s with their
    /// guns low, to stand at a door. A man sent to a spot on the floor still runs for cover, and so does one sent off a charge or to cover an opening. Off = the leg as it was
    #[k(true)] pub door_brisk: bool,
    /// The maintainer, of a pair sent to a wall that "walk[ed] very slowly to it": "maybe we do something where if you're walking with the squad but behind
    /// them, they slow down to let you catch up (and match your speed when you're ahead of them?)" — "and then we remove the static 0.8 multiplier". So a man
    /// walking to the mark an order gave him goes at the PLAYER'S walk; at `k` × it only while the player is within `paceNear` of him, `paceGap` or more
    /// further from that mark than he is, and walking after HIM (moving, within `paceCos` of the line to him) — `k` of the player's own pace then, so a
    /// creeping player gains on him too; and at the player's own pace, no more than the fight's (PACE.max: never a run; an order asks ORDERS.brisk at most),
    /// while the player is that much nearer the mark than he is and bound for it faster than a walk. A man falling in keeps teamFollow's paces; the silent
    /// squad's and the unseen squad's own caps still stand over it. The player's pace is the one his legs are asked for (char.vel): pressed against a wall he
    /// still counts as moving. Off = the walk as it was (teamWalkOf answers teamWalk)
    #[k(true)] pub pacing: bool,
    #[k(12.0)] pub pace_near: f64,
    #[k(1.0)] pub pace_gap: f64,
    #[k(0.5)] pub pace_cos: f64,
    /// An order that sends a man to a mark hands him its watch point (the door he is to stack on) from his first step, and its walking carriage — the gun up and leading,
    /// the orders' tick — would carry the gun on that point through whatever stands between: walking along a wall with the door beyond it, his muzzle would be 0.2 to 1 m off the plaster
    /// for a third to a half of the walk (and his weapon light, rightly, dark for it). So in the walking carriage and with no line from his gun
    /// to the order's watch (to short of it: a door's own leaf stands in its frame), the tick hands his gun a point straight ahead of his chest instead, and he walks his
    /// path facing it (no side-step with his chest to a point he cannot see); once he has the line the watch is his again and the gun comes round to it at the ordered gun's
    /// own rate. ONLY the order's own watch in the walking carriage (Assignment.move: a stack, a walk to a door, a cover mark, a lane to cover, a stand-off) is asked: a
    /// point the unasked cover dealt him (dealt on a line of sight), the spotted rule's, the fight's (GuardScript.over: his own eyes) and the legs of an entry or a slice
    /// — which have no walking carriage: their point is the corner the muzzle leads him round to — are not, nor is any drill. The door path hands the same kind
    /// of point, the way he goes, for a gun that would cross an open leaf. `walk_aim_off` / `walk_aim_on`: how long the line must have been shut before the watch is given up,
    /// and clear before it is his again (s).
    #[k(0.25)] pub walk_aim_off: f64,
    #[k(0.5)] pub walk_aim_on: f64,
    /// m/s from which a teammate is running: the gun goes low, nothing is fired, the boots are a sprinter's
    #[k(2.7)] pub run_at: f64,
    /// m/s² a man with leave to run picks pace up at (`pace.accel` is 2.6: a walker leaning into a walk)
    #[k(6.0)] pub accel: f64,
    /// a sprinting teammate's footfall (the player's own sprint is 1)
    #[k(1.0)] pub step: f64,
    /// two runs of one man this close together are one (s)
    #[k(1.0)] pub gap: f64,
}
