//! The numbers of wand.
use crate::bytes::ToBytes;
use alloc::string::String;
use game_derive::Knobs;

#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct WandK {
    /// s kneeling, hands on it, before the lens shows anything
    #[k(1.5)] pub feed_secs: f64,
    /// s without a round or a throw after it is pulled out (with kit handling on the gun's own draw by hand is the lock-out instead)
    #[k(0.6)] pub stow_secs: f64,
    /// the lens's wedge, degrees (under the cone fog of hard the wedge is the cone's own: the picture's soft edge is one number for every eye)
    #[k(90.0)] pub fov_deg: f64,
    /// degrees either side, at the wall the door is in, the wedge never turns into
    #[k(10.0)] pub blind_deg: f64,
    /// how fast the lens turns after the cursor, rad/s
    #[k(1.6)] pub pan_rate: f64,
    /// the lens off the floor, m
    #[k(0.1)] pub eye_y: f64,
    /// the lens past the leaf's middle plane, m (clear of the wall's far face: the levels' walls are 0.12 to 0.24 thick)
    #[k(0.22)] pub past_m: f64,
    /// s a man last seen through it keeps his '(?)' where he stood
    #[k(15.0)] pub mark_secs: f64,
    /// the leaf counts as shut inside this angle of home, rad (doors.ts's own 'home' for a boot is the same 0.05)
    #[k(0.05)] pub shut_rad: f64,
    /// s kept off his spot before he gives it up (a man standing where he must kneel, furniture in the way)
    #[k(2.5)] pub give_up_secs: f64,
    /// the order's key on a door's menu (X on the door, then this): no digit is free there, and nothing is ever on a digit above 5
    #[k(String::from("KeyV"))] pub order_key: String,
    /// seconds for a teammate's lens to sweep from one end of its arc to the other and back (nobody aims it for him)
    #[k(5.0)] pub sweep_secs: f64,
    /// s from the start of the look (a teammate: from on his mark at the door) to the lens through, with `quick`, where the squat at the door is not drawn
    ///  (a teammate, the stealth poses off): the kneel's own time. Where it IS drawn the wait is the squat's own `cableIn` (stealthpose.ts) — wandFeedSecs.
    ///  Never before he is down on his knee (`quickDown`), and for Sam never before both hands are on it, however long putting the gun away takes
    #[k(0.4)] pub quick_secs: f64,
    /// how far down the kneel must be (the animator's crouch, 0‥1) before a quick lens is through
    #[k(0.9)] pub quick_down: f64,
}
