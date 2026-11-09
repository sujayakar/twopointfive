//! The numbers of pairorders.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct PairK {
    /// THE SWITCH: the quiet orders for one partner (off: the game as it was)
    #[k(false)] pub on: bool,
    /// the biggest squad (men besides Sam) the rows are offered to: these are a PAIR's orders
    #[k(1.0)] pub max_squad: f64,
    /// a defender this near (m) the point under the cursor is the target (his body under the cursor always is)
    #[k(0.9)] pub pick_m: f64,
    /// on your mark: how far behind the man (m) the partner waits · his hands reach this far (m) and take this long (s) · the noise they make (a 'prop' event; 0: none)
    #[k(1.2)] pub stage_m: f64,
    #[k(1.05)] pub reach: f64,
    #[k(0.6)] pub secs: f64,
    #[k(0.0)] pub scuffle: f64,
    /// his pace working round to the man's back, and over the last paces once sent (m/s; the silent posture's own cap still applies) · on a knee on the way
    #[k(1.1)] pub pace: f64,
    #[k(1.6)] pub go_pace: f64,
    /// the way to that back is read this often (s); with every way across somebody's eyes he waits this long (s) before he says there is none
    #[k(0.3)] pub stalk_every: f64,
    #[k(20.0)] pub stalk_wait: f64,
    #[k(2.4)] pub ring_m: f64,
    /// HE DOES NOT TURN ABOUT ON HIS WAY (the maintainer, 10-22, the pair lesson: "partner seems to be very paranoid about always turning around to check the
    ///  other guy"). Two things turned him (the lesson's scripted run, sampled): the way to a man's back handed out with a corner he had already passed first —
    ///  the way he is kept on picks its nearest CORNER, and with that a pace behind him he turned right round to walk back to it and round again the next time
    ///  the way was read; and each time the way read watched he parked and at once squared to the man, sixty to a hundred degrees, and squared back to walk a
    ///  third of a second later. So: on the kept way a corner he is already past along its own leg is dropped, where the way on from the next is as plain and
    ///  as cheap a walk from where he stands (priced as any kept way is); and parked on a watched way he keeps the heading he has for `steadyFaceSecs` — only a wait that lasts turns him to watch
    ///  his man. The page: on (`?pairsteady=0`); false here and on the frozen harness stand (`TPS_PAIRSTEADY=1`; a FROZEN_HOLDS row). false = as it was
    #[k(1.2)] pub steady_face_secs: f64,
    #[k(1.2)] pub watch_secs: f64,
    #[k(0.25)] pub watch_aw: f64,
    /// 'take him now': he holds at this ring (m) for the man to glance away and back, and goes within `heelFresh` s of the glance ending — or after `heelWait` s of none
    #[k(2.9)] pub heel_ring: f64,
    #[k(1.2)] pub heel_fresh: f64,
    #[k(22.0)] pub heel_wait: f64,
    /// a job not done this long (s) after it was given (after the mark, for one that waited) is called off by the man himself
    #[k(75.0)] pub take_secs: f64,
    /// hold in the dark: floor is DARK at or under this on the light meter's 0‥1 scale (the place's steady light, direct only) · how far (m) from the point he
    ///  may look for it · none: refuse (off: go anyway, and say so) · on a knee there · the height (m) the light is read at
    #[k(0.3)] pub dark_vis: f64,
    #[k(1.5)] pub dark_m: f64,
    #[k(0.9)] pub dark_h: f64,
    /// … and once there he says so when the place has been lit this long (s), not more often than every `litEvery` s
    #[k(2.0)] pub lit_secs: f64,
    #[k(20.0)] pub lit_every: f64,
}
